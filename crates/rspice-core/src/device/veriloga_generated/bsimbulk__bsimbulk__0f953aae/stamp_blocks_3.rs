#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_3(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let eq21_e1432: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, s.v[786]);
        let eq21_e1432_d_n0: f64 = (s.dn[786][0] * ddt_scale);
        let eq21_e1432_d_n1: f64 = (s.dn[786][1] * ddt_scale);
        let eq21_e1432_d_n2: f64 = (s.dn[786][2] * ddt_scale);
        let eq21_e1432_d_n3: f64 = (s.dn[786][3] * ddt_scale);
        let eq21_e1432_d_n4: f64 = (s.dn[786][4] * ddt_scale);
        let eq21_e1432_d_n5: f64 = (s.dn[786][5] * ddt_scale);
        let eq21_e1432_d_n6: f64 = (s.dn[786][6] * ddt_scale);
        let eq21_e1432_d_n7: f64 = (s.dn[786][7] * ddt_scale);
        let eq21_e1432_d_n8: f64 = (s.dn[786][8] * ddt_scale);
        let eq21_e1432_d_n9: f64 = (s.dn[786][9] * ddt_scale);
        let eq21_e1432_d_n10: f64 = (s.dn[786][10] * ddt_scale);
        let eq21_e1432_d_n11: f64 = (s.dn[786][11] * ddt_scale);
        let eq21_e1432_d_n12: f64 = (s.dn[786][12] * ddt_scale);
        let eq21_e1432_d_n13: f64 = (s.dn[786][13] * ddt_scale);
        let eq21_e1432_d_n14: f64 = (s.dn[786][14] * ddt_scale);
        let eq21_e1432_d_n15: f64 = (s.dn[786][15] * ddt_scale);
        let eq21_e1432_d_n16: f64 = (s.dn[786][16] * ddt_scale);
        let eq21_e1432_d_b0: f64 = (s.db[786][0] * ddt_scale);
        let eq21_e1432_d_b1: f64 = (s.db[786][1] * ddt_scale);
        let eq21_e1432_d_b2: f64 = (s.db[786][2] * ddt_scale);
        let eq21_e1432_d_b3: f64 = (s.db[786][3] * ddt_scale);
        let eq21_e1432_d_b4: f64 = (s.db[786][4] * ddt_scale);
        let eq21_e1432_d_b5: f64 = (s.db[786][5] * ddt_scale);
        let eq21_e1432_d_b6: f64 = (s.db[786][6] * ddt_scale);
        let eq21_e1432_d_b7: f64 = (s.db[786][7] * ddt_scale);
        let eq21_e1432_d_b8: f64 = (s.db[786][8] * ddt_scale);
        let eq21_e1432_d_b9: f64 = (s.db[786][9] * ddt_scale);
        let eq21_e1432_d_b10: f64 = (s.db[786][10] * ddt_scale);
        let eq21_e1432_d_b11: f64 = (s.db[786][11] * ddt_scale);
        let eq21_e1432_d_b12: f64 = (s.db[786][12] * ddt_scale);
        let eq21_e1432_d_b13: f64 = (s.db[786][13] * ddt_scale);
        let eq21_value: f64 = eq21_e1432;
        let eq21_node_derivatives: [f64; 17] = [eq21_e1432_d_n0, eq21_e1432_d_n1, eq21_e1432_d_n2, eq21_e1432_d_n3, eq21_e1432_d_n4, eq21_e1432_d_n5, eq21_e1432_d_n6, eq21_e1432_d_n7, eq21_e1432_d_n8, eq21_e1432_d_n9, eq21_e1432_d_n10, eq21_e1432_d_n11, eq21_e1432_d_n12, eq21_e1432_d_n13, eq21_e1432_d_n14, eq21_e1432_d_n15, eq21_e1432_d_n16];
        let eq21_branch_derivatives: [f64; 14] = [eq21_e1432_d_b0, eq21_e1432_d_b1, eq21_e1432_d_b2, eq21_e1432_d_b3, eq21_e1432_d_b4, eq21_e1432_d_b5, eq21_e1432_d_b6, eq21_e1432_d_b7, eq21_e1432_d_b8, eq21_e1432_d_b9, eq21_e1432_d_b10, eq21_e1432_d_b11, eq21_e1432_d_b12, eq21_e1432_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[11]),
            multiplicity * (eq21_value),
            nodes,
            &eq21_node_derivatives,
            branches,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let eq22_e1435: f64 = (-s.v[187]);
        let eq22_e1435_d_n0: f64 = (-s.dn[187][0]);
        let eq22_e1435_d_n1: f64 = (-s.dn[187][1]);
        let eq22_e1435_d_n2: f64 = (-s.dn[187][2]);
        let eq22_e1435_d_n3: f64 = (-s.dn[187][3]);
        let eq22_e1435_d_n4: f64 = (-s.dn[187][4]);
        let eq22_e1435_d_n5: f64 = (-s.dn[187][5]);
        let eq22_e1435_d_n6: f64 = (-s.dn[187][6]);
        let eq22_e1435_d_n7: f64 = (-s.dn[187][7]);
        let eq22_e1435_d_n8: f64 = (-s.dn[187][8]);
        let eq22_e1435_d_n9: f64 = (-s.dn[187][9]);
        let eq22_e1435_d_n10: f64 = (-s.dn[187][10]);
        let eq22_e1435_d_n11: f64 = (-s.dn[187][11]);
        let eq22_e1435_d_n12: f64 = (-s.dn[187][12]);
        let eq22_e1435_d_n13: f64 = (-s.dn[187][13]);
        let eq22_e1435_d_n14: f64 = (-s.dn[187][14]);
        let eq22_e1435_d_n15: f64 = (-s.dn[187][15]);
        let eq22_e1435_d_n16: f64 = (-s.dn[187][16]);
        let eq22_e1435_d_b0: f64 = (-s.db[187][0]);
        let eq22_e1435_d_b1: f64 = (-s.db[187][1]);
        let eq22_e1435_d_b2: f64 = (-s.db[187][2]);
        let eq22_e1435_d_b3: f64 = (-s.db[187][3]);
        let eq22_e1435_d_b4: f64 = (-s.db[187][4]);
        let eq22_e1435_d_b5: f64 = (-s.db[187][5]);
        let eq22_e1435_d_b6: f64 = (-s.db[187][6]);
        let eq22_e1435_d_b7: f64 = (-s.db[187][7]);
        let eq22_e1435_d_b8: f64 = (-s.db[187][8]);
        let eq22_e1435_d_b9: f64 = (-s.db[187][9]);
        let eq22_e1435_d_b10: f64 = (-s.db[187][10]);
        let eq22_e1435_d_b11: f64 = (-s.db[187][11]);
        let eq22_e1435_d_b12: f64 = (-s.db[187][12]);
        let eq22_e1435_d_b13: f64 = (-s.db[187][13]);
        let eq22_e1437: f64 = (eq22_e1435 * s.v[223]);
        let eq22_e1437_d_n0: f64 = ((eq22_e1435_d_n0 * s.v[223]) + (eq22_e1435 * s.dn[223][0]));
        let eq22_e1437_d_n1: f64 = ((eq22_e1435_d_n1 * s.v[223]) + (eq22_e1435 * s.dn[223][1]));
        let eq22_e1437_d_n2: f64 = ((eq22_e1435_d_n2 * s.v[223]) + (eq22_e1435 * s.dn[223][2]));
        let eq22_e1437_d_n3: f64 = ((eq22_e1435_d_n3 * s.v[223]) + (eq22_e1435 * s.dn[223][3]));
        let eq22_e1437_d_n4: f64 = ((eq22_e1435_d_n4 * s.v[223]) + (eq22_e1435 * s.dn[223][4]));
        let eq22_e1437_d_n5: f64 = ((eq22_e1435_d_n5 * s.v[223]) + (eq22_e1435 * s.dn[223][5]));
        let eq22_e1437_d_n6: f64 = ((eq22_e1435_d_n6 * s.v[223]) + (eq22_e1435 * s.dn[223][6]));
        let eq22_e1437_d_n7: f64 = ((eq22_e1435_d_n7 * s.v[223]) + (eq22_e1435 * s.dn[223][7]));
        let eq22_e1437_d_n8: f64 = ((eq22_e1435_d_n8 * s.v[223]) + (eq22_e1435 * s.dn[223][8]));
        let eq22_e1437_d_n9: f64 = ((eq22_e1435_d_n9 * s.v[223]) + (eq22_e1435 * s.dn[223][9]));
        let eq22_e1437_d_n10: f64 = ((eq22_e1435_d_n10 * s.v[223]) + (eq22_e1435 * s.dn[223][10]));
        let eq22_e1437_d_n11: f64 = ((eq22_e1435_d_n11 * s.v[223]) + (eq22_e1435 * s.dn[223][11]));
        let eq22_e1437_d_n12: f64 = ((eq22_e1435_d_n12 * s.v[223]) + (eq22_e1435 * s.dn[223][12]));
        let eq22_e1437_d_n13: f64 = ((eq22_e1435_d_n13 * s.v[223]) + (eq22_e1435 * s.dn[223][13]));
        let eq22_e1437_d_n14: f64 = ((eq22_e1435_d_n14 * s.v[223]) + (eq22_e1435 * s.dn[223][14]));
        let eq22_e1437_d_n15: f64 = ((eq22_e1435_d_n15 * s.v[223]) + (eq22_e1435 * s.dn[223][15]));
        let eq22_e1437_d_n16: f64 = ((eq22_e1435_d_n16 * s.v[223]) + (eq22_e1435 * s.dn[223][16]));
        let eq22_e1437_d_b0: f64 = ((eq22_e1435_d_b0 * s.v[223]) + (eq22_e1435 * s.db[223][0]));
        let eq22_e1437_d_b1: f64 = ((eq22_e1435_d_b1 * s.v[223]) + (eq22_e1435 * s.db[223][1]));
        let eq22_e1437_d_b2: f64 = ((eq22_e1435_d_b2 * s.v[223]) + (eq22_e1435 * s.db[223][2]));
        let eq22_e1437_d_b3: f64 = ((eq22_e1435_d_b3 * s.v[223]) + (eq22_e1435 * s.db[223][3]));
        let eq22_e1437_d_b4: f64 = ((eq22_e1435_d_b4 * s.v[223]) + (eq22_e1435 * s.db[223][4]));
        let eq22_e1437_d_b5: f64 = ((eq22_e1435_d_b5 * s.v[223]) + (eq22_e1435 * s.db[223][5]));
        let eq22_e1437_d_b6: f64 = ((eq22_e1435_d_b6 * s.v[223]) + (eq22_e1435 * s.db[223][6]));
        let eq22_e1437_d_b7: f64 = ((eq22_e1435_d_b7 * s.v[223]) + (eq22_e1435 * s.db[223][7]));
        let eq22_e1437_d_b8: f64 = ((eq22_e1435_d_b8 * s.v[223]) + (eq22_e1435 * s.db[223][8]));
        let eq22_e1437_d_b9: f64 = ((eq22_e1435_d_b9 * s.v[223]) + (eq22_e1435 * s.db[223][9]));
        let eq22_e1437_d_b10: f64 = ((eq22_e1435_d_b10 * s.v[223]) + (eq22_e1435 * s.db[223][10]));
        let eq22_e1437_d_b11: f64 = ((eq22_e1435_d_b11 * s.v[223]) + (eq22_e1435 * s.db[223][11]));
        let eq22_e1437_d_b12: f64 = ((eq22_e1435_d_b12 * s.v[223]) + (eq22_e1435 * s.db[223][12]));
        let eq22_e1437_d_b13: f64 = ((eq22_e1435_d_b13 * s.v[223]) + (eq22_e1435 * s.db[223][13]));
        let eq22_e1438: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, eq22_e1437);
        let eq22_e1438_d_n0: f64 = (eq22_e1437_d_n0 * ddt_scale);
        let eq22_e1438_d_n1: f64 = (eq22_e1437_d_n1 * ddt_scale);
        let eq22_e1438_d_n2: f64 = (eq22_e1437_d_n2 * ddt_scale);
        let eq22_e1438_d_n3: f64 = (eq22_e1437_d_n3 * ddt_scale);
        let eq22_e1438_d_n4: f64 = (eq22_e1437_d_n4 * ddt_scale);
        let eq22_e1438_d_n5: f64 = (eq22_e1437_d_n5 * ddt_scale);
        let eq22_e1438_d_n6: f64 = (eq22_e1437_d_n6 * ddt_scale);
        let eq22_e1438_d_n7: f64 = (eq22_e1437_d_n7 * ddt_scale);
        let eq22_e1438_d_n8: f64 = (eq22_e1437_d_n8 * ddt_scale);
        let eq22_e1438_d_n9: f64 = (eq22_e1437_d_n9 * ddt_scale);
        let eq22_e1438_d_n10: f64 = (eq22_e1437_d_n10 * ddt_scale);
        let eq22_e1438_d_n11: f64 = (eq22_e1437_d_n11 * ddt_scale);
        let eq22_e1438_d_n12: f64 = (eq22_e1437_d_n12 * ddt_scale);
        let eq22_e1438_d_n13: f64 = (eq22_e1437_d_n13 * ddt_scale);
        let eq22_e1438_d_n14: f64 = (eq22_e1437_d_n14 * ddt_scale);
        let eq22_e1438_d_n15: f64 = (eq22_e1437_d_n15 * ddt_scale);
        let eq22_e1438_d_n16: f64 = (eq22_e1437_d_n16 * ddt_scale);
        let eq22_e1438_d_b0: f64 = (eq22_e1437_d_b0 * ddt_scale);
        let eq22_e1438_d_b1: f64 = (eq22_e1437_d_b1 * ddt_scale);
        let eq22_e1438_d_b2: f64 = (eq22_e1437_d_b2 * ddt_scale);
        let eq22_e1438_d_b3: f64 = (eq22_e1437_d_b3 * ddt_scale);
        let eq22_e1438_d_b4: f64 = (eq22_e1437_d_b4 * ddt_scale);
        let eq22_e1438_d_b5: f64 = (eq22_e1437_d_b5 * ddt_scale);
        let eq22_e1438_d_b6: f64 = (eq22_e1437_d_b6 * ddt_scale);
        let eq22_e1438_d_b7: f64 = (eq22_e1437_d_b7 * ddt_scale);
        let eq22_e1438_d_b8: f64 = (eq22_e1437_d_b8 * ddt_scale);
        let eq22_e1438_d_b9: f64 = (eq22_e1437_d_b9 * ddt_scale);
        let eq22_e1438_d_b10: f64 = (eq22_e1437_d_b10 * ddt_scale);
        let eq22_e1438_d_b11: f64 = (eq22_e1437_d_b11 * ddt_scale);
        let eq22_e1438_d_b12: f64 = (eq22_e1437_d_b12 * ddt_scale);
        let eq22_e1438_d_b13: f64 = (eq22_e1437_d_b13 * ddt_scale);
        let eq22_e1439: f64 = (p.p29 * eq22_e1438);
        let eq22_e1439_d_n0: f64 = (p.p29 * eq22_e1438_d_n0);
        let eq22_e1439_d_n1: f64 = (p.p29 * eq22_e1438_d_n1);
        let eq22_e1439_d_n2: f64 = (p.p29 * eq22_e1438_d_n2);
        let eq22_e1439_d_n3: f64 = (p.p29 * eq22_e1438_d_n3);
        let eq22_e1439_d_n4: f64 = (p.p29 * eq22_e1438_d_n4);
        let eq22_e1439_d_n5: f64 = (p.p29 * eq22_e1438_d_n5);
        let eq22_e1439_d_n6: f64 = (p.p29 * eq22_e1438_d_n6);
        let eq22_e1439_d_n7: f64 = (p.p29 * eq22_e1438_d_n7);
        let eq22_e1439_d_n8: f64 = (p.p29 * eq22_e1438_d_n8);
        let eq22_e1439_d_n9: f64 = (p.p29 * eq22_e1438_d_n9);
        let eq22_e1439_d_n10: f64 = (p.p29 * eq22_e1438_d_n10);
        let eq22_e1439_d_n11: f64 = (p.p29 * eq22_e1438_d_n11);
        let eq22_e1439_d_n12: f64 = (p.p29 * eq22_e1438_d_n12);
        let eq22_e1439_d_n13: f64 = (p.p29 * eq22_e1438_d_n13);
        let eq22_e1439_d_n14: f64 = (p.p29 * eq22_e1438_d_n14);
        let eq22_e1439_d_n15: f64 = (p.p29 * eq22_e1438_d_n15);
        let eq22_e1439_d_n16: f64 = (p.p29 * eq22_e1438_d_n16);
        let eq22_e1439_d_b0: f64 = (p.p29 * eq22_e1438_d_b0);
        let eq22_e1439_d_b1: f64 = (p.p29 * eq22_e1438_d_b1);
        let eq22_e1439_d_b2: f64 = (p.p29 * eq22_e1438_d_b2);
        let eq22_e1439_d_b3: f64 = (p.p29 * eq22_e1438_d_b3);
        let eq22_e1439_d_b4: f64 = (p.p29 * eq22_e1438_d_b4);
        let eq22_e1439_d_b5: f64 = (p.p29 * eq22_e1438_d_b5);
        let eq22_e1439_d_b6: f64 = (p.p29 * eq22_e1438_d_b6);
        let eq22_e1439_d_b7: f64 = (p.p29 * eq22_e1438_d_b7);
        let eq22_e1439_d_b8: f64 = (p.p29 * eq22_e1438_d_b8);
        let eq22_e1439_d_b9: f64 = (p.p29 * eq22_e1438_d_b9);
        let eq22_e1439_d_b10: f64 = (p.p29 * eq22_e1438_d_b10);
        let eq22_e1439_d_b11: f64 = (p.p29 * eq22_e1438_d_b11);
        let eq22_e1439_d_b12: f64 = (p.p29 * eq22_e1438_d_b12);
        let eq22_e1439_d_b13: f64 = (p.p29 * eq22_e1438_d_b13);
        let eq22_value: f64 = eq22_e1439;
        let eq22_node_derivatives: [f64; 17] = [eq22_e1439_d_n0, eq22_e1439_d_n1, eq22_e1439_d_n2, eq22_e1439_d_n3, eq22_e1439_d_n4, eq22_e1439_d_n5, eq22_e1439_d_n6, eq22_e1439_d_n7, eq22_e1439_d_n8, eq22_e1439_d_n9, eq22_e1439_d_n10, eq22_e1439_d_n11, eq22_e1439_d_n12, eq22_e1439_d_n13, eq22_e1439_d_n14, eq22_e1439_d_n15, eq22_e1439_d_n16];
        let eq22_branch_derivatives: [f64; 14] = [eq22_e1439_d_b0, eq22_e1439_d_b1, eq22_e1439_d_b2, eq22_e1439_d_b3, eq22_e1439_d_b4, eq22_e1439_d_b5, eq22_e1439_d_b6, eq22_e1439_d_b7, eq22_e1439_d_b8, eq22_e1439_d_b9, eq22_e1439_d_b10, eq22_e1439_d_b11, eq22_e1439_d_b12, eq22_e1439_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            multiplicity * (eq22_value),
            nodes,
            &eq22_node_derivatives,
            branches,
            &eq22_branch_derivatives,
            multiplicity,
        );
        let eq23_e1442: f64 = (-s.v[187]);
        let eq23_e1442_d_n0: f64 = (-s.dn[187][0]);
        let eq23_e1442_d_n1: f64 = (-s.dn[187][1]);
        let eq23_e1442_d_n2: f64 = (-s.dn[187][2]);
        let eq23_e1442_d_n3: f64 = (-s.dn[187][3]);
        let eq23_e1442_d_n4: f64 = (-s.dn[187][4]);
        let eq23_e1442_d_n5: f64 = (-s.dn[187][5]);
        let eq23_e1442_d_n6: f64 = (-s.dn[187][6]);
        let eq23_e1442_d_n7: f64 = (-s.dn[187][7]);
        let eq23_e1442_d_n8: f64 = (-s.dn[187][8]);
        let eq23_e1442_d_n9: f64 = (-s.dn[187][9]);
        let eq23_e1442_d_n10: f64 = (-s.dn[187][10]);
        let eq23_e1442_d_n11: f64 = (-s.dn[187][11]);
        let eq23_e1442_d_n12: f64 = (-s.dn[187][12]);
        let eq23_e1442_d_n13: f64 = (-s.dn[187][13]);
        let eq23_e1442_d_n14: f64 = (-s.dn[187][14]);
        let eq23_e1442_d_n15: f64 = (-s.dn[187][15]);
        let eq23_e1442_d_n16: f64 = (-s.dn[187][16]);
        let eq23_e1442_d_b0: f64 = (-s.db[187][0]);
        let eq23_e1442_d_b1: f64 = (-s.db[187][1]);
        let eq23_e1442_d_b2: f64 = (-s.db[187][2]);
        let eq23_e1442_d_b3: f64 = (-s.db[187][3]);
        let eq23_e1442_d_b4: f64 = (-s.db[187][4]);
        let eq23_e1442_d_b5: f64 = (-s.db[187][5]);
        let eq23_e1442_d_b6: f64 = (-s.db[187][6]);
        let eq23_e1442_d_b7: f64 = (-s.db[187][7]);
        let eq23_e1442_d_b8: f64 = (-s.db[187][8]);
        let eq23_e1442_d_b9: f64 = (-s.db[187][9]);
        let eq23_e1442_d_b10: f64 = (-s.db[187][10]);
        let eq23_e1442_d_b11: f64 = (-s.db[187][11]);
        let eq23_e1442_d_b12: f64 = (-s.db[187][12]);
        let eq23_e1442_d_b13: f64 = (-s.db[187][13]);
        let eq23_e1444: f64 = (eq23_e1442 * s.v[224]);
        let eq23_e1444_d_n0: f64 = ((eq23_e1442_d_n0 * s.v[224]) + (eq23_e1442 * s.dn[224][0]));
        let eq23_e1444_d_n1: f64 = ((eq23_e1442_d_n1 * s.v[224]) + (eq23_e1442 * s.dn[224][1]));
        let eq23_e1444_d_n2: f64 = ((eq23_e1442_d_n2 * s.v[224]) + (eq23_e1442 * s.dn[224][2]));
        let eq23_e1444_d_n3: f64 = ((eq23_e1442_d_n3 * s.v[224]) + (eq23_e1442 * s.dn[224][3]));
        let eq23_e1444_d_n4: f64 = ((eq23_e1442_d_n4 * s.v[224]) + (eq23_e1442 * s.dn[224][4]));
        let eq23_e1444_d_n5: f64 = ((eq23_e1442_d_n5 * s.v[224]) + (eq23_e1442 * s.dn[224][5]));
        let eq23_e1444_d_n6: f64 = ((eq23_e1442_d_n6 * s.v[224]) + (eq23_e1442 * s.dn[224][6]));
        let eq23_e1444_d_n7: f64 = ((eq23_e1442_d_n7 * s.v[224]) + (eq23_e1442 * s.dn[224][7]));
        let eq23_e1444_d_n8: f64 = ((eq23_e1442_d_n8 * s.v[224]) + (eq23_e1442 * s.dn[224][8]));
        let eq23_e1444_d_n9: f64 = ((eq23_e1442_d_n9 * s.v[224]) + (eq23_e1442 * s.dn[224][9]));
        let eq23_e1444_d_n10: f64 = ((eq23_e1442_d_n10 * s.v[224]) + (eq23_e1442 * s.dn[224][10]));
        let eq23_e1444_d_n11: f64 = ((eq23_e1442_d_n11 * s.v[224]) + (eq23_e1442 * s.dn[224][11]));
        let eq23_e1444_d_n12: f64 = ((eq23_e1442_d_n12 * s.v[224]) + (eq23_e1442 * s.dn[224][12]));
        let eq23_e1444_d_n13: f64 = ((eq23_e1442_d_n13 * s.v[224]) + (eq23_e1442 * s.dn[224][13]));
        let eq23_e1444_d_n14: f64 = ((eq23_e1442_d_n14 * s.v[224]) + (eq23_e1442 * s.dn[224][14]));
        let eq23_e1444_d_n15: f64 = ((eq23_e1442_d_n15 * s.v[224]) + (eq23_e1442 * s.dn[224][15]));
        let eq23_e1444_d_n16: f64 = ((eq23_e1442_d_n16 * s.v[224]) + (eq23_e1442 * s.dn[224][16]));
        let eq23_e1444_d_b0: f64 = ((eq23_e1442_d_b0 * s.v[224]) + (eq23_e1442 * s.db[224][0]));
        let eq23_e1444_d_b1: f64 = ((eq23_e1442_d_b1 * s.v[224]) + (eq23_e1442 * s.db[224][1]));
        let eq23_e1444_d_b2: f64 = ((eq23_e1442_d_b2 * s.v[224]) + (eq23_e1442 * s.db[224][2]));
        let eq23_e1444_d_b3: f64 = ((eq23_e1442_d_b3 * s.v[224]) + (eq23_e1442 * s.db[224][3]));
        let eq23_e1444_d_b4: f64 = ((eq23_e1442_d_b4 * s.v[224]) + (eq23_e1442 * s.db[224][4]));
        let eq23_e1444_d_b5: f64 = ((eq23_e1442_d_b5 * s.v[224]) + (eq23_e1442 * s.db[224][5]));
        let eq23_e1444_d_b6: f64 = ((eq23_e1442_d_b6 * s.v[224]) + (eq23_e1442 * s.db[224][6]));
        let eq23_e1444_d_b7: f64 = ((eq23_e1442_d_b7 * s.v[224]) + (eq23_e1442 * s.db[224][7]));
        let eq23_e1444_d_b8: f64 = ((eq23_e1442_d_b8 * s.v[224]) + (eq23_e1442 * s.db[224][8]));
        let eq23_e1444_d_b9: f64 = ((eq23_e1442_d_b9 * s.v[224]) + (eq23_e1442 * s.db[224][9]));
        let eq23_e1444_d_b10: f64 = ((eq23_e1442_d_b10 * s.v[224]) + (eq23_e1442 * s.db[224][10]));
        let eq23_e1444_d_b11: f64 = ((eq23_e1442_d_b11 * s.v[224]) + (eq23_e1442 * s.db[224][11]));
        let eq23_e1444_d_b12: f64 = ((eq23_e1442_d_b12 * s.v[224]) + (eq23_e1442 * s.db[224][12]));
        let eq23_e1444_d_b13: f64 = ((eq23_e1442_d_b13 * s.v[224]) + (eq23_e1442 * s.db[224][13]));
        let eq23_e1445: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, eq23_e1444);
        let eq23_e1445_d_n0: f64 = (eq23_e1444_d_n0 * ddt_scale);
        let eq23_e1445_d_n1: f64 = (eq23_e1444_d_n1 * ddt_scale);
        let eq23_e1445_d_n2: f64 = (eq23_e1444_d_n2 * ddt_scale);
        let eq23_e1445_d_n3: f64 = (eq23_e1444_d_n3 * ddt_scale);
        let eq23_e1445_d_n4: f64 = (eq23_e1444_d_n4 * ddt_scale);
        let eq23_e1445_d_n5: f64 = (eq23_e1444_d_n5 * ddt_scale);
        let eq23_e1445_d_n6: f64 = (eq23_e1444_d_n6 * ddt_scale);
        let eq23_e1445_d_n7: f64 = (eq23_e1444_d_n7 * ddt_scale);
        let eq23_e1445_d_n8: f64 = (eq23_e1444_d_n8 * ddt_scale);
        let eq23_e1445_d_n9: f64 = (eq23_e1444_d_n9 * ddt_scale);
        let eq23_e1445_d_n10: f64 = (eq23_e1444_d_n10 * ddt_scale);
        let eq23_e1445_d_n11: f64 = (eq23_e1444_d_n11 * ddt_scale);
        let eq23_e1445_d_n12: f64 = (eq23_e1444_d_n12 * ddt_scale);
        let eq23_e1445_d_n13: f64 = (eq23_e1444_d_n13 * ddt_scale);
        let eq23_e1445_d_n14: f64 = (eq23_e1444_d_n14 * ddt_scale);
        let eq23_e1445_d_n15: f64 = (eq23_e1444_d_n15 * ddt_scale);
        let eq23_e1445_d_n16: f64 = (eq23_e1444_d_n16 * ddt_scale);
        let eq23_e1445_d_b0: f64 = (eq23_e1444_d_b0 * ddt_scale);
        let eq23_e1445_d_b1: f64 = (eq23_e1444_d_b1 * ddt_scale);
        let eq23_e1445_d_b2: f64 = (eq23_e1444_d_b2 * ddt_scale);
        let eq23_e1445_d_b3: f64 = (eq23_e1444_d_b3 * ddt_scale);
        let eq23_e1445_d_b4: f64 = (eq23_e1444_d_b4 * ddt_scale);
        let eq23_e1445_d_b5: f64 = (eq23_e1444_d_b5 * ddt_scale);
        let eq23_e1445_d_b6: f64 = (eq23_e1444_d_b6 * ddt_scale);
        let eq23_e1445_d_b7: f64 = (eq23_e1444_d_b7 * ddt_scale);
        let eq23_e1445_d_b8: f64 = (eq23_e1444_d_b8 * ddt_scale);
        let eq23_e1445_d_b9: f64 = (eq23_e1444_d_b9 * ddt_scale);
        let eq23_e1445_d_b10: f64 = (eq23_e1444_d_b10 * ddt_scale);
        let eq23_e1445_d_b11: f64 = (eq23_e1444_d_b11 * ddt_scale);
        let eq23_e1445_d_b12: f64 = (eq23_e1444_d_b12 * ddt_scale);
        let eq23_e1445_d_b13: f64 = (eq23_e1444_d_b13 * ddt_scale);
        let eq23_e1446: f64 = (p.p29 * eq23_e1445);
        let eq23_e1446_d_n0: f64 = (p.p29 * eq23_e1445_d_n0);
        let eq23_e1446_d_n1: f64 = (p.p29 * eq23_e1445_d_n1);
        let eq23_e1446_d_n2: f64 = (p.p29 * eq23_e1445_d_n2);
        let eq23_e1446_d_n3: f64 = (p.p29 * eq23_e1445_d_n3);
        let eq23_e1446_d_n4: f64 = (p.p29 * eq23_e1445_d_n4);
        let eq23_e1446_d_n5: f64 = (p.p29 * eq23_e1445_d_n5);
        let eq23_e1446_d_n6: f64 = (p.p29 * eq23_e1445_d_n6);
        let eq23_e1446_d_n7: f64 = (p.p29 * eq23_e1445_d_n7);
        let eq23_e1446_d_n8: f64 = (p.p29 * eq23_e1445_d_n8);
        let eq23_e1446_d_n9: f64 = (p.p29 * eq23_e1445_d_n9);
        let eq23_e1446_d_n10: f64 = (p.p29 * eq23_e1445_d_n10);
        let eq23_e1446_d_n11: f64 = (p.p29 * eq23_e1445_d_n11);
        let eq23_e1446_d_n12: f64 = (p.p29 * eq23_e1445_d_n12);
        let eq23_e1446_d_n13: f64 = (p.p29 * eq23_e1445_d_n13);
        let eq23_e1446_d_n14: f64 = (p.p29 * eq23_e1445_d_n14);
        let eq23_e1446_d_n15: f64 = (p.p29 * eq23_e1445_d_n15);
        let eq23_e1446_d_n16: f64 = (p.p29 * eq23_e1445_d_n16);
        let eq23_e1446_d_b0: f64 = (p.p29 * eq23_e1445_d_b0);
        let eq23_e1446_d_b1: f64 = (p.p29 * eq23_e1445_d_b1);
        let eq23_e1446_d_b2: f64 = (p.p29 * eq23_e1445_d_b2);
        let eq23_e1446_d_b3: f64 = (p.p29 * eq23_e1445_d_b3);
        let eq23_e1446_d_b4: f64 = (p.p29 * eq23_e1445_d_b4);
        let eq23_e1446_d_b5: f64 = (p.p29 * eq23_e1445_d_b5);
        let eq23_e1446_d_b6: f64 = (p.p29 * eq23_e1445_d_b6);
        let eq23_e1446_d_b7: f64 = (p.p29 * eq23_e1445_d_b7);
        let eq23_e1446_d_b8: f64 = (p.p29 * eq23_e1445_d_b8);
        let eq23_e1446_d_b9: f64 = (p.p29 * eq23_e1445_d_b9);
        let eq23_e1446_d_b10: f64 = (p.p29 * eq23_e1445_d_b10);
        let eq23_e1446_d_b11: f64 = (p.p29 * eq23_e1445_d_b11);
        let eq23_e1446_d_b12: f64 = (p.p29 * eq23_e1445_d_b12);
        let eq23_e1446_d_b13: f64 = (p.p29 * eq23_e1445_d_b13);
        let eq23_value: f64 = eq23_e1446;
        let eq23_node_derivatives: [f64; 17] = [eq23_e1446_d_n0, eq23_e1446_d_n1, eq23_e1446_d_n2, eq23_e1446_d_n3, eq23_e1446_d_n4, eq23_e1446_d_n5, eq23_e1446_d_n6, eq23_e1446_d_n7, eq23_e1446_d_n8, eq23_e1446_d_n9, eq23_e1446_d_n10, eq23_e1446_d_n11, eq23_e1446_d_n12, eq23_e1446_d_n13, eq23_e1446_d_n14, eq23_e1446_d_n15, eq23_e1446_d_n16];
        let eq23_branch_derivatives: [f64; 14] = [eq23_e1446_d_b0, eq23_e1446_d_b1, eq23_e1446_d_b2, eq23_e1446_d_b3, eq23_e1446_d_b4, eq23_e1446_d_b5, eq23_e1446_d_b6, eq23_e1446_d_b7, eq23_e1446_d_b8, eq23_e1446_d_b9, eq23_e1446_d_b10, eq23_e1446_d_b11, eq23_e1446_d_b12, eq23_e1446_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            multiplicity * (eq23_value),
            nodes,
            &eq23_node_derivatives,
            branches,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let eq24_e1449: f64 = (-s.v[187]);
        let eq24_e1449_d_n0: f64 = (-s.dn[187][0]);
        let eq24_e1449_d_n1: f64 = (-s.dn[187][1]);
        let eq24_e1449_d_n2: f64 = (-s.dn[187][2]);
        let eq24_e1449_d_n3: f64 = (-s.dn[187][3]);
        let eq24_e1449_d_n4: f64 = (-s.dn[187][4]);
        let eq24_e1449_d_n5: f64 = (-s.dn[187][5]);
        let eq24_e1449_d_n6: f64 = (-s.dn[187][6]);
        let eq24_e1449_d_n7: f64 = (-s.dn[187][7]);
        let eq24_e1449_d_n8: f64 = (-s.dn[187][8]);
        let eq24_e1449_d_n9: f64 = (-s.dn[187][9]);
        let eq24_e1449_d_n10: f64 = (-s.dn[187][10]);
        let eq24_e1449_d_n11: f64 = (-s.dn[187][11]);
        let eq24_e1449_d_n12: f64 = (-s.dn[187][12]);
        let eq24_e1449_d_n13: f64 = (-s.dn[187][13]);
        let eq24_e1449_d_n14: f64 = (-s.dn[187][14]);
        let eq24_e1449_d_n15: f64 = (-s.dn[187][15]);
        let eq24_e1449_d_n16: f64 = (-s.dn[187][16]);
        let eq24_e1449_d_b0: f64 = (-s.db[187][0]);
        let eq24_e1449_d_b1: f64 = (-s.db[187][1]);
        let eq24_e1449_d_b2: f64 = (-s.db[187][2]);
        let eq24_e1449_d_b3: f64 = (-s.db[187][3]);
        let eq24_e1449_d_b4: f64 = (-s.db[187][4]);
        let eq24_e1449_d_b5: f64 = (-s.db[187][5]);
        let eq24_e1449_d_b6: f64 = (-s.db[187][6]);
        let eq24_e1449_d_b7: f64 = (-s.db[187][7]);
        let eq24_e1449_d_b8: f64 = (-s.db[187][8]);
        let eq24_e1449_d_b9: f64 = (-s.db[187][9]);
        let eq24_e1449_d_b10: f64 = (-s.db[187][10]);
        let eq24_e1449_d_b11: f64 = (-s.db[187][11]);
        let eq24_e1449_d_b12: f64 = (-s.db[187][12]);
        let eq24_e1449_d_b13: f64 = (-s.db[187][13]);
        let eq24_e1451: f64 = (eq24_e1449 * s.v[221]);
        let eq24_e1451_d_n0: f64 = ((eq24_e1449_d_n0 * s.v[221]) + (eq24_e1449 * s.dn[221][0]));
        let eq24_e1451_d_n1: f64 = ((eq24_e1449_d_n1 * s.v[221]) + (eq24_e1449 * s.dn[221][1]));
        let eq24_e1451_d_n2: f64 = ((eq24_e1449_d_n2 * s.v[221]) + (eq24_e1449 * s.dn[221][2]));
        let eq24_e1451_d_n3: f64 = ((eq24_e1449_d_n3 * s.v[221]) + (eq24_e1449 * s.dn[221][3]));
        let eq24_e1451_d_n4: f64 = ((eq24_e1449_d_n4 * s.v[221]) + (eq24_e1449 * s.dn[221][4]));
        let eq24_e1451_d_n5: f64 = ((eq24_e1449_d_n5 * s.v[221]) + (eq24_e1449 * s.dn[221][5]));
        let eq24_e1451_d_n6: f64 = ((eq24_e1449_d_n6 * s.v[221]) + (eq24_e1449 * s.dn[221][6]));
        let eq24_e1451_d_n7: f64 = ((eq24_e1449_d_n7 * s.v[221]) + (eq24_e1449 * s.dn[221][7]));
        let eq24_e1451_d_n8: f64 = ((eq24_e1449_d_n8 * s.v[221]) + (eq24_e1449 * s.dn[221][8]));
        let eq24_e1451_d_n9: f64 = ((eq24_e1449_d_n9 * s.v[221]) + (eq24_e1449 * s.dn[221][9]));
        let eq24_e1451_d_n10: f64 = ((eq24_e1449_d_n10 * s.v[221]) + (eq24_e1449 * s.dn[221][10]));
        let eq24_e1451_d_n11: f64 = ((eq24_e1449_d_n11 * s.v[221]) + (eq24_e1449 * s.dn[221][11]));
        let eq24_e1451_d_n12: f64 = ((eq24_e1449_d_n12 * s.v[221]) + (eq24_e1449 * s.dn[221][12]));
        let eq24_e1451_d_n13: f64 = ((eq24_e1449_d_n13 * s.v[221]) + (eq24_e1449 * s.dn[221][13]));
        let eq24_e1451_d_n14: f64 = ((eq24_e1449_d_n14 * s.v[221]) + (eq24_e1449 * s.dn[221][14]));
        let eq24_e1451_d_n15: f64 = ((eq24_e1449_d_n15 * s.v[221]) + (eq24_e1449 * s.dn[221][15]));
        let eq24_e1451_d_n16: f64 = ((eq24_e1449_d_n16 * s.v[221]) + (eq24_e1449 * s.dn[221][16]));
        let eq24_e1451_d_b0: f64 = ((eq24_e1449_d_b0 * s.v[221]) + (eq24_e1449 * s.db[221][0]));
        let eq24_e1451_d_b1: f64 = ((eq24_e1449_d_b1 * s.v[221]) + (eq24_e1449 * s.db[221][1]));
        let eq24_e1451_d_b2: f64 = ((eq24_e1449_d_b2 * s.v[221]) + (eq24_e1449 * s.db[221][2]));
        let eq24_e1451_d_b3: f64 = ((eq24_e1449_d_b3 * s.v[221]) + (eq24_e1449 * s.db[221][3]));
        let eq24_e1451_d_b4: f64 = ((eq24_e1449_d_b4 * s.v[221]) + (eq24_e1449 * s.db[221][4]));
        let eq24_e1451_d_b5: f64 = ((eq24_e1449_d_b5 * s.v[221]) + (eq24_e1449 * s.db[221][5]));
        let eq24_e1451_d_b6: f64 = ((eq24_e1449_d_b6 * s.v[221]) + (eq24_e1449 * s.db[221][6]));
        let eq24_e1451_d_b7: f64 = ((eq24_e1449_d_b7 * s.v[221]) + (eq24_e1449 * s.db[221][7]));
        let eq24_e1451_d_b8: f64 = ((eq24_e1449_d_b8 * s.v[221]) + (eq24_e1449 * s.db[221][8]));
        let eq24_e1451_d_b9: f64 = ((eq24_e1449_d_b9 * s.v[221]) + (eq24_e1449 * s.db[221][9]));
        let eq24_e1451_d_b10: f64 = ((eq24_e1449_d_b10 * s.v[221]) + (eq24_e1449 * s.db[221][10]));
        let eq24_e1451_d_b11: f64 = ((eq24_e1449_d_b11 * s.v[221]) + (eq24_e1449 * s.db[221][11]));
        let eq24_e1451_d_b12: f64 = ((eq24_e1449_d_b12 * s.v[221]) + (eq24_e1449 * s.db[221][12]));
        let eq24_e1451_d_b13: f64 = ((eq24_e1449_d_b13 * s.v[221]) + (eq24_e1449 * s.db[221][13]));
        let eq24_e1452: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, eq24_e1451);
        let eq24_e1452_d_n0: f64 = (eq24_e1451_d_n0 * ddt_scale);
        let eq24_e1452_d_n1: f64 = (eq24_e1451_d_n1 * ddt_scale);
        let eq24_e1452_d_n2: f64 = (eq24_e1451_d_n2 * ddt_scale);
        let eq24_e1452_d_n3: f64 = (eq24_e1451_d_n3 * ddt_scale);
        let eq24_e1452_d_n4: f64 = (eq24_e1451_d_n4 * ddt_scale);
        let eq24_e1452_d_n5: f64 = (eq24_e1451_d_n5 * ddt_scale);
        let eq24_e1452_d_n6: f64 = (eq24_e1451_d_n6 * ddt_scale);
        let eq24_e1452_d_n7: f64 = (eq24_e1451_d_n7 * ddt_scale);
        let eq24_e1452_d_n8: f64 = (eq24_e1451_d_n8 * ddt_scale);
        let eq24_e1452_d_n9: f64 = (eq24_e1451_d_n9 * ddt_scale);
        let eq24_e1452_d_n10: f64 = (eq24_e1451_d_n10 * ddt_scale);
        let eq24_e1452_d_n11: f64 = (eq24_e1451_d_n11 * ddt_scale);
        let eq24_e1452_d_n12: f64 = (eq24_e1451_d_n12 * ddt_scale);
        let eq24_e1452_d_n13: f64 = (eq24_e1451_d_n13 * ddt_scale);
        let eq24_e1452_d_n14: f64 = (eq24_e1451_d_n14 * ddt_scale);
        let eq24_e1452_d_n15: f64 = (eq24_e1451_d_n15 * ddt_scale);
        let eq24_e1452_d_n16: f64 = (eq24_e1451_d_n16 * ddt_scale);
        let eq24_e1452_d_b0: f64 = (eq24_e1451_d_b0 * ddt_scale);
        let eq24_e1452_d_b1: f64 = (eq24_e1451_d_b1 * ddt_scale);
        let eq24_e1452_d_b2: f64 = (eq24_e1451_d_b2 * ddt_scale);
        let eq24_e1452_d_b3: f64 = (eq24_e1451_d_b3 * ddt_scale);
        let eq24_e1452_d_b4: f64 = (eq24_e1451_d_b4 * ddt_scale);
        let eq24_e1452_d_b5: f64 = (eq24_e1451_d_b5 * ddt_scale);
        let eq24_e1452_d_b6: f64 = (eq24_e1451_d_b6 * ddt_scale);
        let eq24_e1452_d_b7: f64 = (eq24_e1451_d_b7 * ddt_scale);
        let eq24_e1452_d_b8: f64 = (eq24_e1451_d_b8 * ddt_scale);
        let eq24_e1452_d_b9: f64 = (eq24_e1451_d_b9 * ddt_scale);
        let eq24_e1452_d_b10: f64 = (eq24_e1451_d_b10 * ddt_scale);
        let eq24_e1452_d_b11: f64 = (eq24_e1451_d_b11 * ddt_scale);
        let eq24_e1452_d_b12: f64 = (eq24_e1451_d_b12 * ddt_scale);
        let eq24_e1452_d_b13: f64 = (eq24_e1451_d_b13 * ddt_scale);
        let eq24_e1453: f64 = (p.p29 * eq24_e1452);
        let eq24_e1453_d_n0: f64 = (p.p29 * eq24_e1452_d_n0);
        let eq24_e1453_d_n1: f64 = (p.p29 * eq24_e1452_d_n1);
        let eq24_e1453_d_n2: f64 = (p.p29 * eq24_e1452_d_n2);
        let eq24_e1453_d_n3: f64 = (p.p29 * eq24_e1452_d_n3);
        let eq24_e1453_d_n4: f64 = (p.p29 * eq24_e1452_d_n4);
        let eq24_e1453_d_n5: f64 = (p.p29 * eq24_e1452_d_n5);
        let eq24_e1453_d_n6: f64 = (p.p29 * eq24_e1452_d_n6);
        let eq24_e1453_d_n7: f64 = (p.p29 * eq24_e1452_d_n7);
        let eq24_e1453_d_n8: f64 = (p.p29 * eq24_e1452_d_n8);
        let eq24_e1453_d_n9: f64 = (p.p29 * eq24_e1452_d_n9);
        let eq24_e1453_d_n10: f64 = (p.p29 * eq24_e1452_d_n10);
        let eq24_e1453_d_n11: f64 = (p.p29 * eq24_e1452_d_n11);
        let eq24_e1453_d_n12: f64 = (p.p29 * eq24_e1452_d_n12);
        let eq24_e1453_d_n13: f64 = (p.p29 * eq24_e1452_d_n13);
        let eq24_e1453_d_n14: f64 = (p.p29 * eq24_e1452_d_n14);
        let eq24_e1453_d_n15: f64 = (p.p29 * eq24_e1452_d_n15);
        let eq24_e1453_d_n16: f64 = (p.p29 * eq24_e1452_d_n16);
        let eq24_e1453_d_b0: f64 = (p.p29 * eq24_e1452_d_b0);
        let eq24_e1453_d_b1: f64 = (p.p29 * eq24_e1452_d_b1);
        let eq24_e1453_d_b2: f64 = (p.p29 * eq24_e1452_d_b2);
        let eq24_e1453_d_b3: f64 = (p.p29 * eq24_e1452_d_b3);
        let eq24_e1453_d_b4: f64 = (p.p29 * eq24_e1452_d_b4);
        let eq24_e1453_d_b5: f64 = (p.p29 * eq24_e1452_d_b5);
        let eq24_e1453_d_b6: f64 = (p.p29 * eq24_e1452_d_b6);
        let eq24_e1453_d_b7: f64 = (p.p29 * eq24_e1452_d_b7);
        let eq24_e1453_d_b8: f64 = (p.p29 * eq24_e1452_d_b8);
        let eq24_e1453_d_b9: f64 = (p.p29 * eq24_e1452_d_b9);
        let eq24_e1453_d_b10: f64 = (p.p29 * eq24_e1452_d_b10);
        let eq24_e1453_d_b11: f64 = (p.p29 * eq24_e1452_d_b11);
        let eq24_e1453_d_b12: f64 = (p.p29 * eq24_e1452_d_b12);
        let eq24_e1453_d_b13: f64 = (p.p29 * eq24_e1452_d_b13);
        let eq24_value: f64 = eq24_e1453;
        let eq24_node_derivatives: [f64; 17] = [eq24_e1453_d_n0, eq24_e1453_d_n1, eq24_e1453_d_n2, eq24_e1453_d_n3, eq24_e1453_d_n4, eq24_e1453_d_n5, eq24_e1453_d_n6, eq24_e1453_d_n7, eq24_e1453_d_n8, eq24_e1453_d_n9, eq24_e1453_d_n10, eq24_e1453_d_n11, eq24_e1453_d_n12, eq24_e1453_d_n13, eq24_e1453_d_n14, eq24_e1453_d_n15, eq24_e1453_d_n16];
        let eq24_branch_derivatives: [f64; 14] = [eq24_e1453_d_b0, eq24_e1453_d_b1, eq24_e1453_d_b2, eq24_e1453_d_b3, eq24_e1453_d_b4, eq24_e1453_d_b5, eq24_e1453_d_b6, eq24_e1453_d_b7, eq24_e1453_d_b8, eq24_e1453_d_b9, eq24_e1453_d_b10, eq24_e1453_d_b11, eq24_e1453_d_b12, eq24_e1453_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[11]),
            multiplicity * (eq24_value),
            nodes,
            &eq24_node_derivatives,
            branches,
            &eq24_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_4(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let eq25_e1456: f64 = (s.v[187] * p.p28);
        let eq25_e1456_d_n0: f64 = (s.dn[187][0] * p.p28);
        let eq25_e1456_d_n1: f64 = (s.dn[187][1] * p.p28);
        let eq25_e1456_d_n2: f64 = (s.dn[187][2] * p.p28);
        let eq25_e1456_d_n3: f64 = (s.dn[187][3] * p.p28);
        let eq25_e1456_d_n4: f64 = (s.dn[187][4] * p.p28);
        let eq25_e1456_d_n5: f64 = (s.dn[187][5] * p.p28);
        let eq25_e1456_d_n6: f64 = (s.dn[187][6] * p.p28);
        let eq25_e1456_d_n7: f64 = (s.dn[187][7] * p.p28);
        let eq25_e1456_d_n8: f64 = (s.dn[187][8] * p.p28);
        let eq25_e1456_d_n9: f64 = (s.dn[187][9] * p.p28);
        let eq25_e1456_d_n10: f64 = (s.dn[187][10] * p.p28);
        let eq25_e1456_d_n11: f64 = (s.dn[187][11] * p.p28);
        let eq25_e1456_d_n12: f64 = (s.dn[187][12] * p.p28);
        let eq25_e1456_d_n13: f64 = (s.dn[187][13] * p.p28);
        let eq25_e1456_d_n14: f64 = (s.dn[187][14] * p.p28);
        let eq25_e1456_d_n15: f64 = (s.dn[187][15] * p.p28);
        let eq25_e1456_d_n16: f64 = (s.dn[187][16] * p.p28);
        let eq25_e1456_d_b0: f64 = (s.db[187][0] * p.p28);
        let eq25_e1456_d_b1: f64 = (s.db[187][1] * p.p28);
        let eq25_e1456_d_b2: f64 = (s.db[187][2] * p.p28);
        let eq25_e1456_d_b3: f64 = (s.db[187][3] * p.p28);
        let eq25_e1456_d_b4: f64 = (s.db[187][4] * p.p28);
        let eq25_e1456_d_b5: f64 = (s.db[187][5] * p.p28);
        let eq25_e1456_d_b6: f64 = (s.db[187][6] * p.p28);
        let eq25_e1456_d_b7: f64 = (s.db[187][7] * p.p28);
        let eq25_e1456_d_b8: f64 = (s.db[187][8] * p.p28);
        let eq25_e1456_d_b9: f64 = (s.db[187][9] * p.p28);
        let eq25_e1456_d_b10: f64 = (s.db[187][10] * p.p28);
        let eq25_e1456_d_b11: f64 = (s.db[187][11] * p.p28);
        let eq25_e1456_d_b12: f64 = (s.db[187][12] * p.p28);
        let eq25_e1456_d_b13: f64 = (s.db[187][13] * p.p28);
        let eq25_e1458: f64 = (eq25_e1456 * s.v[57]);
        let eq25_e1458_d_n0: f64 = ((eq25_e1456_d_n0 * s.v[57]) + (eq25_e1456 * s.dn[57][0]));
        let eq25_e1458_d_n1: f64 = ((eq25_e1456_d_n1 * s.v[57]) + (eq25_e1456 * s.dn[57][1]));
        let eq25_e1458_d_n2: f64 = ((eq25_e1456_d_n2 * s.v[57]) + (eq25_e1456 * s.dn[57][2]));
        let eq25_e1458_d_n3: f64 = ((eq25_e1456_d_n3 * s.v[57]) + (eq25_e1456 * s.dn[57][3]));
        let eq25_e1458_d_n4: f64 = ((eq25_e1456_d_n4 * s.v[57]) + (eq25_e1456 * s.dn[57][4]));
        let eq25_e1458_d_n5: f64 = ((eq25_e1456_d_n5 * s.v[57]) + (eq25_e1456 * s.dn[57][5]));
        let eq25_e1458_d_n6: f64 = ((eq25_e1456_d_n6 * s.v[57]) + (eq25_e1456 * s.dn[57][6]));
        let eq25_e1458_d_n7: f64 = ((eq25_e1456_d_n7 * s.v[57]) + (eq25_e1456 * s.dn[57][7]));
        let eq25_e1458_d_n8: f64 = ((eq25_e1456_d_n8 * s.v[57]) + (eq25_e1456 * s.dn[57][8]));
        let eq25_e1458_d_n9: f64 = ((eq25_e1456_d_n9 * s.v[57]) + (eq25_e1456 * s.dn[57][9]));
        let eq25_e1458_d_n10: f64 = ((eq25_e1456_d_n10 * s.v[57]) + (eq25_e1456 * s.dn[57][10]));
        let eq25_e1458_d_n11: f64 = ((eq25_e1456_d_n11 * s.v[57]) + (eq25_e1456 * s.dn[57][11]));
        let eq25_e1458_d_n12: f64 = ((eq25_e1456_d_n12 * s.v[57]) + (eq25_e1456 * s.dn[57][12]));
        let eq25_e1458_d_n13: f64 = ((eq25_e1456_d_n13 * s.v[57]) + (eq25_e1456 * s.dn[57][13]));
        let eq25_e1458_d_n14: f64 = ((eq25_e1456_d_n14 * s.v[57]) + (eq25_e1456 * s.dn[57][14]));
        let eq25_e1458_d_n15: f64 = ((eq25_e1456_d_n15 * s.v[57]) + (eq25_e1456 * s.dn[57][15]));
        let eq25_e1458_d_n16: f64 = ((eq25_e1456_d_n16 * s.v[57]) + (eq25_e1456 * s.dn[57][16]));
        let eq25_e1458_d_b0: f64 = ((eq25_e1456_d_b0 * s.v[57]) + (eq25_e1456 * s.db[57][0]));
        let eq25_e1458_d_b1: f64 = ((eq25_e1456_d_b1 * s.v[57]) + (eq25_e1456 * s.db[57][1]));
        let eq25_e1458_d_b2: f64 = ((eq25_e1456_d_b2 * s.v[57]) + (eq25_e1456 * s.db[57][2]));
        let eq25_e1458_d_b3: f64 = ((eq25_e1456_d_b3 * s.v[57]) + (eq25_e1456 * s.db[57][3]));
        let eq25_e1458_d_b4: f64 = ((eq25_e1456_d_b4 * s.v[57]) + (eq25_e1456 * s.db[57][4]));
        let eq25_e1458_d_b5: f64 = ((eq25_e1456_d_b5 * s.v[57]) + (eq25_e1456 * s.db[57][5]));
        let eq25_e1458_d_b6: f64 = ((eq25_e1456_d_b6 * s.v[57]) + (eq25_e1456 * s.db[57][6]));
        let eq25_e1458_d_b7: f64 = ((eq25_e1456_d_b7 * s.v[57]) + (eq25_e1456 * s.db[57][7]));
        let eq25_e1458_d_b8: f64 = ((eq25_e1456_d_b8 * s.v[57]) + (eq25_e1456 * s.db[57][8]));
        let eq25_e1458_d_b9: f64 = ((eq25_e1456_d_b9 * s.v[57]) + (eq25_e1456 * s.db[57][9]));
        let eq25_e1458_d_b10: f64 = ((eq25_e1456_d_b10 * s.v[57]) + (eq25_e1456 * s.db[57][10]));
        let eq25_e1458_d_b11: f64 = ((eq25_e1456_d_b11 * s.v[57]) + (eq25_e1456 * s.db[57][11]));
        let eq25_e1458_d_b12: f64 = ((eq25_e1456_d_b12 * s.v[57]) + (eq25_e1456 * s.db[57][12]));
        let eq25_e1458_d_b13: f64 = ((eq25_e1456_d_b13 * s.v[57]) + (eq25_e1456 * s.db[57][13]));
        let eq25_e1460: f64 = (eq25_e1458 * s.v[188]);
        let eq25_e1460_d_n0: f64 = ((eq25_e1458_d_n0 * s.v[188]) + (eq25_e1458 * s.dn[188][0]));
        let eq25_e1460_d_n1: f64 = ((eq25_e1458_d_n1 * s.v[188]) + (eq25_e1458 * s.dn[188][1]));
        let eq25_e1460_d_n2: f64 = ((eq25_e1458_d_n2 * s.v[188]) + (eq25_e1458 * s.dn[188][2]));
        let eq25_e1460_d_n3: f64 = ((eq25_e1458_d_n3 * s.v[188]) + (eq25_e1458 * s.dn[188][3]));
        let eq25_e1460_d_n4: f64 = ((eq25_e1458_d_n4 * s.v[188]) + (eq25_e1458 * s.dn[188][4]));
        let eq25_e1460_d_n5: f64 = ((eq25_e1458_d_n5 * s.v[188]) + (eq25_e1458 * s.dn[188][5]));
        let eq25_e1460_d_n6: f64 = ((eq25_e1458_d_n6 * s.v[188]) + (eq25_e1458 * s.dn[188][6]));
        let eq25_e1460_d_n7: f64 = ((eq25_e1458_d_n7 * s.v[188]) + (eq25_e1458 * s.dn[188][7]));
        let eq25_e1460_d_n8: f64 = ((eq25_e1458_d_n8 * s.v[188]) + (eq25_e1458 * s.dn[188][8]));
        let eq25_e1460_d_n9: f64 = ((eq25_e1458_d_n9 * s.v[188]) + (eq25_e1458 * s.dn[188][9]));
        let eq25_e1460_d_n10: f64 = ((eq25_e1458_d_n10 * s.v[188]) + (eq25_e1458 * s.dn[188][10]));
        let eq25_e1460_d_n11: f64 = ((eq25_e1458_d_n11 * s.v[188]) + (eq25_e1458 * s.dn[188][11]));
        let eq25_e1460_d_n12: f64 = ((eq25_e1458_d_n12 * s.v[188]) + (eq25_e1458 * s.dn[188][12]));
        let eq25_e1460_d_n13: f64 = ((eq25_e1458_d_n13 * s.v[188]) + (eq25_e1458 * s.dn[188][13]));
        let eq25_e1460_d_n14: f64 = ((eq25_e1458_d_n14 * s.v[188]) + (eq25_e1458 * s.dn[188][14]));
        let eq25_e1460_d_n15: f64 = ((eq25_e1458_d_n15 * s.v[188]) + (eq25_e1458 * s.dn[188][15]));
        let eq25_e1460_d_n16: f64 = ((eq25_e1458_d_n16 * s.v[188]) + (eq25_e1458 * s.dn[188][16]));
        let eq25_e1460_d_b0: f64 = ((eq25_e1458_d_b0 * s.v[188]) + (eq25_e1458 * s.db[188][0]));
        let eq25_e1460_d_b1: f64 = ((eq25_e1458_d_b1 * s.v[188]) + (eq25_e1458 * s.db[188][1]));
        let eq25_e1460_d_b2: f64 = ((eq25_e1458_d_b2 * s.v[188]) + (eq25_e1458 * s.db[188][2]));
        let eq25_e1460_d_b3: f64 = ((eq25_e1458_d_b3 * s.v[188]) + (eq25_e1458 * s.db[188][3]));
        let eq25_e1460_d_b4: f64 = ((eq25_e1458_d_b4 * s.v[188]) + (eq25_e1458 * s.db[188][4]));
        let eq25_e1460_d_b5: f64 = ((eq25_e1458_d_b5 * s.v[188]) + (eq25_e1458 * s.db[188][5]));
        let eq25_e1460_d_b6: f64 = ((eq25_e1458_d_b6 * s.v[188]) + (eq25_e1458 * s.db[188][6]));
        let eq25_e1460_d_b7: f64 = ((eq25_e1458_d_b7 * s.v[188]) + (eq25_e1458 * s.db[188][7]));
        let eq25_e1460_d_b8: f64 = ((eq25_e1458_d_b8 * s.v[188]) + (eq25_e1458 * s.db[188][8]));
        let eq25_e1460_d_b9: f64 = ((eq25_e1458_d_b9 * s.v[188]) + (eq25_e1458 * s.db[188][9]));
        let eq25_e1460_d_b10: f64 = ((eq25_e1458_d_b10 * s.v[188]) + (eq25_e1458 * s.db[188][10]));
        let eq25_e1460_d_b11: f64 = ((eq25_e1458_d_b11 * s.v[188]) + (eq25_e1458 * s.db[188][11]));
        let eq25_e1460_d_b12: f64 = ((eq25_e1458_d_b12 * s.v[188]) + (eq25_e1458 * s.db[188][12]));
        let eq25_e1460_d_b13: f64 = ((eq25_e1458_d_b13 * s.v[188]) + (eq25_e1458 * s.db[188][13]));
        let eq25_value: f64 = eq25_e1460;
        let eq25_node_derivatives: [f64; 17] = [eq25_e1460_d_n0, eq25_e1460_d_n1, eq25_e1460_d_n2, eq25_e1460_d_n3, eq25_e1460_d_n4, eq25_e1460_d_n5, eq25_e1460_d_n6, eq25_e1460_d_n7, eq25_e1460_d_n8, eq25_e1460_d_n9, eq25_e1460_d_n10, eq25_e1460_d_n11, eq25_e1460_d_n12, eq25_e1460_d_n13, eq25_e1460_d_n14, eq25_e1460_d_n15, eq25_e1460_d_n16];
        let eq25_branch_derivatives: [f64; 14] = [eq25_e1460_d_b0, eq25_e1460_d_b1, eq25_e1460_d_b2, eq25_e1460_d_b3, eq25_e1460_d_b4, eq25_e1460_d_b5, eq25_e1460_d_b6, eq25_e1460_d_b7, eq25_e1460_d_b8, eq25_e1460_d_b9, eq25_e1460_d_b10, eq25_e1460_d_b11, eq25_e1460_d_b12, eq25_e1460_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            multiplicity * (eq25_value),
            nodes,
            &eq25_node_derivatives,
            branches,
            &eq25_branch_derivatives,
            multiplicity,
        );
        let (eq26_e1464, eq26_e1464_d_n0, eq26_e1464_d_n1, eq26_e1464_d_n2, eq26_e1464_d_n3, eq26_e1464_d_n4, eq26_e1464_d_n5, eq26_e1464_d_n6, eq26_e1464_d_n7, eq26_e1464_d_n8, eq26_e1464_d_n9, eq26_e1464_d_n10, eq26_e1464_d_n11, eq26_e1464_d_n12, eq26_e1464_d_n13, eq26_e1464_d_n14, eq26_e1464_d_n15, eq26_e1464_d_n16, eq26_e1464_d_b0, eq26_e1464_d_b1, eq26_e1464_d_b2, eq26_e1464_d_b3, eq26_e1464_d_b4, eq26_e1464_d_b5, eq26_e1464_d_b6, eq26_e1464_d_b7, eq26_e1464_d_b8, eq26_e1464_d_b9, eq26_e1464_d_b10, eq26_e1464_d_b11, eq26_e1464_d_b12, eq26_e1464_d_b13,) = {
    if s.b[1609] {
        (s.v[831], s.dn[831][0], s.dn[831][1], s.dn[831][2], s.dn[831][3], s.dn[831][4], s.dn[831][5], s.dn[831][6], s.dn[831][7], s.dn[831][8], s.dn[831][9], s.dn[831][10], s.dn[831][11], s.dn[831][12], s.dn[831][13], s.dn[831][14], s.dn[831][15], s.dn[831][16], s.db[831][0], s.db[831][1], s.db[831][2], s.db[831][3], s.db[831][4], s.db[831][5], s.db[831][6], s.db[831][7], s.db[831][8], s.db[831][9], s.db[831][10], s.db[831][11], s.db[831][12], s.db[831][13],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e1464;
        let eq26_node_derivatives: [f64; 17] = [eq26_e1464_d_n0, eq26_e1464_d_n1, eq26_e1464_d_n2, eq26_e1464_d_n3, eq26_e1464_d_n4, eq26_e1464_d_n5, eq26_e1464_d_n6, eq26_e1464_d_n7, eq26_e1464_d_n8, eq26_e1464_d_n9, eq26_e1464_d_n10, eq26_e1464_d_n11, eq26_e1464_d_n12, eq26_e1464_d_n13, eq26_e1464_d_n14, eq26_e1464_d_n15, eq26_e1464_d_n16];
        let eq26_branch_derivatives: [f64; 14] = [eq26_e1464_d_b0, eq26_e1464_d_b1, eq26_e1464_d_b2, eq26_e1464_d_b3, eq26_e1464_d_b4, eq26_e1464_d_b5, eq26_e1464_d_b6, eq26_e1464_d_b7, eq26_e1464_d_b8, eq26_e1464_d_b9, eq26_e1464_d_b10, eq26_e1464_d_b11, eq26_e1464_d_b12, eq26_e1464_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[11]),
            multiplicity * (eq26_value),
            nodes,
            &eq26_node_derivatives,
            branches,
            &eq26_branch_derivatives,
            multiplicity,
        );
        let (eq27_e1470, eq27_e1470_d_n0, eq27_e1470_d_n1, eq27_e1470_d_n2, eq27_e1470_d_n3, eq27_e1470_d_n4, eq27_e1470_d_n5, eq27_e1470_d_n6, eq27_e1470_d_n7, eq27_e1470_d_n8, eq27_e1470_d_n9, eq27_e1470_d_n10, eq27_e1470_d_n11, eq27_e1470_d_n12, eq27_e1470_d_n13, eq27_e1470_d_n14, eq27_e1470_d_n15, eq27_e1470_d_n16, eq27_e1470_d_b0, eq27_e1470_d_b1, eq27_e1470_d_b2, eq27_e1470_d_b3, eq27_e1470_d_b4, eq27_e1470_d_b5, eq27_e1470_d_b6, eq27_e1470_d_b7, eq27_e1470_d_b8, eq27_e1470_d_b9, eq27_e1470_d_b10, eq27_e1470_d_b11, eq27_e1470_d_b12, eq27_e1470_d_b13,) = {
    if s.b[1610] {
        let eq27_e1468: f64 = (s.v[827] + s.v[829]);
        let eq27_e1468_d_n0: f64 = (s.dn[827][0] + s.dn[829][0]);
        let eq27_e1468_d_n1: f64 = (s.dn[827][1] + s.dn[829][1]);
        let eq27_e1468_d_n2: f64 = (s.dn[827][2] + s.dn[829][2]);
        let eq27_e1468_d_n3: f64 = (s.dn[827][3] + s.dn[829][3]);
        let eq27_e1468_d_n4: f64 = (s.dn[827][4] + s.dn[829][4]);
        let eq27_e1468_d_n5: f64 = (s.dn[827][5] + s.dn[829][5]);
        let eq27_e1468_d_n6: f64 = (s.dn[827][6] + s.dn[829][6]);
        let eq27_e1468_d_n7: f64 = (s.dn[827][7] + s.dn[829][7]);
        let eq27_e1468_d_n8: f64 = (s.dn[827][8] + s.dn[829][8]);
        let eq27_e1468_d_n9: f64 = (s.dn[827][9] + s.dn[829][9]);
        let eq27_e1468_d_n10: f64 = (s.dn[827][10] + s.dn[829][10]);
        let eq27_e1468_d_n11: f64 = (s.dn[827][11] + s.dn[829][11]);
        let eq27_e1468_d_n12: f64 = (s.dn[827][12] + s.dn[829][12]);
        let eq27_e1468_d_n13: f64 = (s.dn[827][13] + s.dn[829][13]);
        let eq27_e1468_d_n14: f64 = (s.dn[827][14] + s.dn[829][14]);
        let eq27_e1468_d_n15: f64 = (s.dn[827][15] + s.dn[829][15]);
        let eq27_e1468_d_n16: f64 = (s.dn[827][16] + s.dn[829][16]);
        let eq27_e1468_d_b0: f64 = (s.db[827][0] + s.db[829][0]);
        let eq27_e1468_d_b1: f64 = (s.db[827][1] + s.db[829][1]);
        let eq27_e1468_d_b2: f64 = (s.db[827][2] + s.db[829][2]);
        let eq27_e1468_d_b3: f64 = (s.db[827][3] + s.db[829][3]);
        let eq27_e1468_d_b4: f64 = (s.db[827][4] + s.db[829][4]);
        let eq27_e1468_d_b5: f64 = (s.db[827][5] + s.db[829][5]);
        let eq27_e1468_d_b6: f64 = (s.db[827][6] + s.db[829][6]);
        let eq27_e1468_d_b7: f64 = (s.db[827][7] + s.db[829][7]);
        let eq27_e1468_d_b8: f64 = (s.db[827][8] + s.db[829][8]);
        let eq27_e1468_d_b9: f64 = (s.db[827][9] + s.db[829][9]);
        let eq27_e1468_d_b10: f64 = (s.db[827][10] + s.db[829][10]);
        let eq27_e1468_d_b11: f64 = (s.db[827][11] + s.db[829][11]);
        let eq27_e1468_d_b12: f64 = (s.db[827][12] + s.db[829][12]);
        let eq27_e1468_d_b13: f64 = (s.db[827][13] + s.db[829][13]);
        (eq27_e1468, eq27_e1468_d_n0, eq27_e1468_d_n1, eq27_e1468_d_n2, eq27_e1468_d_n3, eq27_e1468_d_n4, eq27_e1468_d_n5, eq27_e1468_d_n6, eq27_e1468_d_n7, eq27_e1468_d_n8, eq27_e1468_d_n9, eq27_e1468_d_n10, eq27_e1468_d_n11, eq27_e1468_d_n12, eq27_e1468_d_n13, eq27_e1468_d_n14, eq27_e1468_d_n15, eq27_e1468_d_n16, eq27_e1468_d_b0, eq27_e1468_d_b1, eq27_e1468_d_b2, eq27_e1468_d_b3, eq27_e1468_d_b4, eq27_e1468_d_b5, eq27_e1468_d_b6, eq27_e1468_d_b7, eq27_e1468_d_b8, eq27_e1468_d_b9, eq27_e1468_d_b10, eq27_e1468_d_b11, eq27_e1468_d_b12, eq27_e1468_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1470;
        let eq27_node_derivatives: [f64; 17] = [eq27_e1470_d_n0, eq27_e1470_d_n1, eq27_e1470_d_n2, eq27_e1470_d_n3, eq27_e1470_d_n4, eq27_e1470_d_n5, eq27_e1470_d_n6, eq27_e1470_d_n7, eq27_e1470_d_n8, eq27_e1470_d_n9, eq27_e1470_d_n10, eq27_e1470_d_n11, eq27_e1470_d_n12, eq27_e1470_d_n13, eq27_e1470_d_n14, eq27_e1470_d_n15, eq27_e1470_d_n16];
        let eq27_branch_derivatives: [f64; 14] = [eq27_e1470_d_b0, eq27_e1470_d_b1, eq27_e1470_d_b2, eq27_e1470_d_b3, eq27_e1470_d_b4, eq27_e1470_d_b5, eq27_e1470_d_b6, eq27_e1470_d_b7, eq27_e1470_d_b8, eq27_e1470_d_b9, eq27_e1470_d_b10, eq27_e1470_d_b11, eq27_e1470_d_b12, eq27_e1470_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            multiplicity * (eq27_value),
            nodes,
            &eq27_node_derivatives,
            branches,
            &eq27_branch_derivatives,
            multiplicity,
        );
        let (eq28_e1476, eq28_e1476_d_n0, eq28_e1476_d_n1, eq28_e1476_d_n2, eq28_e1476_d_n3, eq28_e1476_d_n4, eq28_e1476_d_n5, eq28_e1476_d_n6, eq28_e1476_d_n7, eq28_e1476_d_n8, eq28_e1476_d_n9, eq28_e1476_d_n10, eq28_e1476_d_n11, eq28_e1476_d_n12, eq28_e1476_d_n13, eq28_e1476_d_n14, eq28_e1476_d_n15, eq28_e1476_d_n16, eq28_e1476_d_b0, eq28_e1476_d_b1, eq28_e1476_d_b2, eq28_e1476_d_b3, eq28_e1476_d_b4, eq28_e1476_d_b5, eq28_e1476_d_b6, eq28_e1476_d_b7, eq28_e1476_d_b8, eq28_e1476_d_b9, eq28_e1476_d_b10, eq28_e1476_d_b11, eq28_e1476_d_b12, eq28_e1476_d_b13,) = {
    if s.b[1610] {
        let eq28_e1474: f64 = (s.v[828] + s.v[830]);
        let eq28_e1474_d_n0: f64 = (s.dn[828][0] + s.dn[830][0]);
        let eq28_e1474_d_n1: f64 = (s.dn[828][1] + s.dn[830][1]);
        let eq28_e1474_d_n2: f64 = (s.dn[828][2] + s.dn[830][2]);
        let eq28_e1474_d_n3: f64 = (s.dn[828][3] + s.dn[830][3]);
        let eq28_e1474_d_n4: f64 = (s.dn[828][4] + s.dn[830][4]);
        let eq28_e1474_d_n5: f64 = (s.dn[828][5] + s.dn[830][5]);
        let eq28_e1474_d_n6: f64 = (s.dn[828][6] + s.dn[830][6]);
        let eq28_e1474_d_n7: f64 = (s.dn[828][7] + s.dn[830][7]);
        let eq28_e1474_d_n8: f64 = (s.dn[828][8] + s.dn[830][8]);
        let eq28_e1474_d_n9: f64 = (s.dn[828][9] + s.dn[830][9]);
        let eq28_e1474_d_n10: f64 = (s.dn[828][10] + s.dn[830][10]);
        let eq28_e1474_d_n11: f64 = (s.dn[828][11] + s.dn[830][11]);
        let eq28_e1474_d_n12: f64 = (s.dn[828][12] + s.dn[830][12]);
        let eq28_e1474_d_n13: f64 = (s.dn[828][13] + s.dn[830][13]);
        let eq28_e1474_d_n14: f64 = (s.dn[828][14] + s.dn[830][14]);
        let eq28_e1474_d_n15: f64 = (s.dn[828][15] + s.dn[830][15]);
        let eq28_e1474_d_n16: f64 = (s.dn[828][16] + s.dn[830][16]);
        let eq28_e1474_d_b0: f64 = (s.db[828][0] + s.db[830][0]);
        let eq28_e1474_d_b1: f64 = (s.db[828][1] + s.db[830][1]);
        let eq28_e1474_d_b2: f64 = (s.db[828][2] + s.db[830][2]);
        let eq28_e1474_d_b3: f64 = (s.db[828][3] + s.db[830][3]);
        let eq28_e1474_d_b4: f64 = (s.db[828][4] + s.db[830][4]);
        let eq28_e1474_d_b5: f64 = (s.db[828][5] + s.db[830][5]);
        let eq28_e1474_d_b6: f64 = (s.db[828][6] + s.db[830][6]);
        let eq28_e1474_d_b7: f64 = (s.db[828][7] + s.db[830][7]);
        let eq28_e1474_d_b8: f64 = (s.db[828][8] + s.db[830][8]);
        let eq28_e1474_d_b9: f64 = (s.db[828][9] + s.db[830][9]);
        let eq28_e1474_d_b10: f64 = (s.db[828][10] + s.db[830][10]);
        let eq28_e1474_d_b11: f64 = (s.db[828][11] + s.db[830][11]);
        let eq28_e1474_d_b12: f64 = (s.db[828][12] + s.db[830][12]);
        let eq28_e1474_d_b13: f64 = (s.db[828][13] + s.db[830][13]);
        (eq28_e1474, eq28_e1474_d_n0, eq28_e1474_d_n1, eq28_e1474_d_n2, eq28_e1474_d_n3, eq28_e1474_d_n4, eq28_e1474_d_n5, eq28_e1474_d_n6, eq28_e1474_d_n7, eq28_e1474_d_n8, eq28_e1474_d_n9, eq28_e1474_d_n10, eq28_e1474_d_n11, eq28_e1474_d_n12, eq28_e1474_d_n13, eq28_e1474_d_n14, eq28_e1474_d_n15, eq28_e1474_d_n16, eq28_e1474_d_b0, eq28_e1474_d_b1, eq28_e1474_d_b2, eq28_e1474_d_b3, eq28_e1474_d_b4, eq28_e1474_d_b5, eq28_e1474_d_b6, eq28_e1474_d_b7, eq28_e1474_d_b8, eq28_e1474_d_b9, eq28_e1474_d_b10, eq28_e1474_d_b11, eq28_e1474_d_b12, eq28_e1474_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e1476;
        let eq28_node_derivatives: [f64; 17] = [eq28_e1476_d_n0, eq28_e1476_d_n1, eq28_e1476_d_n2, eq28_e1476_d_n3, eq28_e1476_d_n4, eq28_e1476_d_n5, eq28_e1476_d_n6, eq28_e1476_d_n7, eq28_e1476_d_n8, eq28_e1476_d_n9, eq28_e1476_d_n10, eq28_e1476_d_n11, eq28_e1476_d_n12, eq28_e1476_d_n13, eq28_e1476_d_n14, eq28_e1476_d_n15, eq28_e1476_d_n16];
        let eq28_branch_derivatives: [f64; 14] = [eq28_e1476_d_b0, eq28_e1476_d_b1, eq28_e1476_d_b2, eq28_e1476_d_b3, eq28_e1476_d_b4, eq28_e1476_d_b5, eq28_e1476_d_b6, eq28_e1476_d_b7, eq28_e1476_d_b8, eq28_e1476_d_b9, eq28_e1476_d_b10, eq28_e1476_d_b11, eq28_e1476_d_b12, eq28_e1476_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[5]),
            multiplicity * (eq28_value),
            nodes,
            &eq28_node_derivatives,
            branches,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let (eq29_e1482, eq29_e1482_d_n0, eq29_e1482_d_n1, eq29_e1482_d_n2, eq29_e1482_d_n3, eq29_e1482_d_n4, eq29_e1482_d_n5, eq29_e1482_d_n6, eq29_e1482_d_n7, eq29_e1482_d_n8, eq29_e1482_d_n9, eq29_e1482_d_n10, eq29_e1482_d_n11, eq29_e1482_d_n12, eq29_e1482_d_n13, eq29_e1482_d_n14, eq29_e1482_d_n15, eq29_e1482_d_n16, eq29_e1482_d_b0, eq29_e1482_d_b1, eq29_e1482_d_b2, eq29_e1482_d_b3, eq29_e1482_d_b4, eq29_e1482_d_b5, eq29_e1482_d_b6, eq29_e1482_d_b7, eq29_e1482_d_b8, eq29_e1482_d_b9, eq29_e1482_d_b10, eq29_e1482_d_b11, eq29_e1482_d_b12, eq29_e1482_d_b13,) = {
    if s.b[1611] {
        let eq29_e1480: f64 = (s.v[824] + s.v[825]);
        let eq29_e1480_d_n0: f64 = (s.dn[824][0] + s.dn[825][0]);
        let eq29_e1480_d_n1: f64 = (s.dn[824][1] + s.dn[825][1]);
        let eq29_e1480_d_n2: f64 = (s.dn[824][2] + s.dn[825][2]);
        let eq29_e1480_d_n3: f64 = (s.dn[824][3] + s.dn[825][3]);
        let eq29_e1480_d_n4: f64 = (s.dn[824][4] + s.dn[825][4]);
        let eq29_e1480_d_n5: f64 = (s.dn[824][5] + s.dn[825][5]);
        let eq29_e1480_d_n6: f64 = (s.dn[824][6] + s.dn[825][6]);
        let eq29_e1480_d_n7: f64 = (s.dn[824][7] + s.dn[825][7]);
        let eq29_e1480_d_n8: f64 = (s.dn[824][8] + s.dn[825][8]);
        let eq29_e1480_d_n9: f64 = (s.dn[824][9] + s.dn[825][9]);
        let eq29_e1480_d_n10: f64 = (s.dn[824][10] + s.dn[825][10]);
        let eq29_e1480_d_n11: f64 = (s.dn[824][11] + s.dn[825][11]);
        let eq29_e1480_d_n12: f64 = (s.dn[824][12] + s.dn[825][12]);
        let eq29_e1480_d_n13: f64 = (s.dn[824][13] + s.dn[825][13]);
        let eq29_e1480_d_n14: f64 = (s.dn[824][14] + s.dn[825][14]);
        let eq29_e1480_d_n15: f64 = (s.dn[824][15] + s.dn[825][15]);
        let eq29_e1480_d_n16: f64 = (s.dn[824][16] + s.dn[825][16]);
        let eq29_e1480_d_b0: f64 = (s.db[824][0] + s.db[825][0]);
        let eq29_e1480_d_b1: f64 = (s.db[824][1] + s.db[825][1]);
        let eq29_e1480_d_b2: f64 = (s.db[824][2] + s.db[825][2]);
        let eq29_e1480_d_b3: f64 = (s.db[824][3] + s.db[825][3]);
        let eq29_e1480_d_b4: f64 = (s.db[824][4] + s.db[825][4]);
        let eq29_e1480_d_b5: f64 = (s.db[824][5] + s.db[825][5]);
        let eq29_e1480_d_b6: f64 = (s.db[824][6] + s.db[825][6]);
        let eq29_e1480_d_b7: f64 = (s.db[824][7] + s.db[825][7]);
        let eq29_e1480_d_b8: f64 = (s.db[824][8] + s.db[825][8]);
        let eq29_e1480_d_b9: f64 = (s.db[824][9] + s.db[825][9]);
        let eq29_e1480_d_b10: f64 = (s.db[824][10] + s.db[825][10]);
        let eq29_e1480_d_b11: f64 = (s.db[824][11] + s.db[825][11]);
        let eq29_e1480_d_b12: f64 = (s.db[824][12] + s.db[825][12]);
        let eq29_e1480_d_b13: f64 = (s.db[824][13] + s.db[825][13]);
        (eq29_e1480, eq29_e1480_d_n0, eq29_e1480_d_n1, eq29_e1480_d_n2, eq29_e1480_d_n3, eq29_e1480_d_n4, eq29_e1480_d_n5, eq29_e1480_d_n6, eq29_e1480_d_n7, eq29_e1480_d_n8, eq29_e1480_d_n9, eq29_e1480_d_n10, eq29_e1480_d_n11, eq29_e1480_d_n12, eq29_e1480_d_n13, eq29_e1480_d_n14, eq29_e1480_d_n15, eq29_e1480_d_n16, eq29_e1480_d_b0, eq29_e1480_d_b1, eq29_e1480_d_b2, eq29_e1480_d_b3, eq29_e1480_d_b4, eq29_e1480_d_b5, eq29_e1480_d_b6, eq29_e1480_d_b7, eq29_e1480_d_b8, eq29_e1480_d_b9, eq29_e1480_d_b10, eq29_e1480_d_b11, eq29_e1480_d_b12, eq29_e1480_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e1482;
        let eq29_node_derivatives: [f64; 17] = [eq29_e1482_d_n0, eq29_e1482_d_n1, eq29_e1482_d_n2, eq29_e1482_d_n3, eq29_e1482_d_n4, eq29_e1482_d_n5, eq29_e1482_d_n6, eq29_e1482_d_n7, eq29_e1482_d_n8, eq29_e1482_d_n9, eq29_e1482_d_n10, eq29_e1482_d_n11, eq29_e1482_d_n12, eq29_e1482_d_n13, eq29_e1482_d_n14, eq29_e1482_d_n15, eq29_e1482_d_n16];
        let eq29_branch_derivatives: [f64; 14] = [eq29_e1482_d_b0, eq29_e1482_d_b1, eq29_e1482_d_b2, eq29_e1482_d_b3, eq29_e1482_d_b4, eq29_e1482_d_b5, eq29_e1482_d_b6, eq29_e1482_d_b7, eq29_e1482_d_b8, eq29_e1482_d_b9, eq29_e1482_d_b10, eq29_e1482_d_b11, eq29_e1482_d_b12, eq29_e1482_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[11]),
            multiplicity * (eq29_value),
            nodes,
            &eq29_node_derivatives,
            branches,
            &eq29_branch_derivatives,
            multiplicity,
        );
        let (eq30_e1490, eq30_e1490_d_n0, eq30_e1490_d_n1, eq30_e1490_d_n2, eq30_e1490_d_n3, eq30_e1490_d_n4, eq30_e1490_d_n5, eq30_e1490_d_n6, eq30_e1490_d_n7, eq30_e1490_d_n8, eq30_e1490_d_n9, eq30_e1490_d_n10, eq30_e1490_d_n11, eq30_e1490_d_n12, eq30_e1490_d_n13, eq30_e1490_d_n14, eq30_e1490_d_n15, eq30_e1490_d_n16, eq30_e1490_d_b0, eq30_e1490_d_b1, eq30_e1490_d_b2, eq30_e1490_d_b3, eq30_e1490_d_b4, eq30_e1490_d_b5, eq30_e1490_d_b6, eq30_e1490_d_b7, eq30_e1490_d_b8, eq30_e1490_d_b9, eq30_e1490_d_b10, eq30_e1490_d_b11, eq30_e1490_d_b12, eq30_e1490_d_b13,) = {
    if s.b[1611] {
        let eq30_e1486: f64 = (p.p28 * s.v[187]);
        let eq30_e1486_d_n0: f64 = (p.p28 * s.dn[187][0]);
        let eq30_e1486_d_n1: f64 = (p.p28 * s.dn[187][1]);
        let eq30_e1486_d_n2: f64 = (p.p28 * s.dn[187][2]);
        let eq30_e1486_d_n3: f64 = (p.p28 * s.dn[187][3]);
        let eq30_e1486_d_n4: f64 = (p.p28 * s.dn[187][4]);
        let eq30_e1486_d_n5: f64 = (p.p28 * s.dn[187][5]);
        let eq30_e1486_d_n6: f64 = (p.p28 * s.dn[187][6]);
        let eq30_e1486_d_n7: f64 = (p.p28 * s.dn[187][7]);
        let eq30_e1486_d_n8: f64 = (p.p28 * s.dn[187][8]);
        let eq30_e1486_d_n9: f64 = (p.p28 * s.dn[187][9]);
        let eq30_e1486_d_n10: f64 = (p.p28 * s.dn[187][10]);
        let eq30_e1486_d_n11: f64 = (p.p28 * s.dn[187][11]);
        let eq30_e1486_d_n12: f64 = (p.p28 * s.dn[187][12]);
        let eq30_e1486_d_n13: f64 = (p.p28 * s.dn[187][13]);
        let eq30_e1486_d_n14: f64 = (p.p28 * s.dn[187][14]);
        let eq30_e1486_d_n15: f64 = (p.p28 * s.dn[187][15]);
        let eq30_e1486_d_n16: f64 = (p.p28 * s.dn[187][16]);
        let eq30_e1486_d_b0: f64 = (p.p28 * s.db[187][0]);
        let eq30_e1486_d_b1: f64 = (p.p28 * s.db[187][1]);
        let eq30_e1486_d_b2: f64 = (p.p28 * s.db[187][2]);
        let eq30_e1486_d_b3: f64 = (p.p28 * s.db[187][3]);
        let eq30_e1486_d_b4: f64 = (p.p28 * s.db[187][4]);
        let eq30_e1486_d_b5: f64 = (p.p28 * s.db[187][5]);
        let eq30_e1486_d_b6: f64 = (p.p28 * s.db[187][6]);
        let eq30_e1486_d_b7: f64 = (p.p28 * s.db[187][7]);
        let eq30_e1486_d_b8: f64 = (p.p28 * s.db[187][8]);
        let eq30_e1486_d_b9: f64 = (p.p28 * s.db[187][9]);
        let eq30_e1486_d_b10: f64 = (p.p28 * s.db[187][10]);
        let eq30_e1486_d_b11: f64 = (p.p28 * s.db[187][11]);
        let eq30_e1486_d_b12: f64 = (p.p28 * s.db[187][12]);
        let eq30_e1486_d_b13: f64 = (p.p28 * s.db[187][13]);
        let eq30_e1488: f64 = (eq30_e1486 * s.v[780]);
        let eq30_e1488_d_n0: f64 = ((eq30_e1486_d_n0 * s.v[780]) + (eq30_e1486 * s.dn[780][0]));
        let eq30_e1488_d_n1: f64 = ((eq30_e1486_d_n1 * s.v[780]) + (eq30_e1486 * s.dn[780][1]));
        let eq30_e1488_d_n2: f64 = ((eq30_e1486_d_n2 * s.v[780]) + (eq30_e1486 * s.dn[780][2]));
        let eq30_e1488_d_n3: f64 = ((eq30_e1486_d_n3 * s.v[780]) + (eq30_e1486 * s.dn[780][3]));
        let eq30_e1488_d_n4: f64 = ((eq30_e1486_d_n4 * s.v[780]) + (eq30_e1486 * s.dn[780][4]));
        let eq30_e1488_d_n5: f64 = ((eq30_e1486_d_n5 * s.v[780]) + (eq30_e1486 * s.dn[780][5]));
        let eq30_e1488_d_n6: f64 = ((eq30_e1486_d_n6 * s.v[780]) + (eq30_e1486 * s.dn[780][6]));
        let eq30_e1488_d_n7: f64 = ((eq30_e1486_d_n7 * s.v[780]) + (eq30_e1486 * s.dn[780][7]));
        let eq30_e1488_d_n8: f64 = ((eq30_e1486_d_n8 * s.v[780]) + (eq30_e1486 * s.dn[780][8]));
        let eq30_e1488_d_n9: f64 = ((eq30_e1486_d_n9 * s.v[780]) + (eq30_e1486 * s.dn[780][9]));
        let eq30_e1488_d_n10: f64 = ((eq30_e1486_d_n10 * s.v[780]) + (eq30_e1486 * s.dn[780][10]));
        let eq30_e1488_d_n11: f64 = ((eq30_e1486_d_n11 * s.v[780]) + (eq30_e1486 * s.dn[780][11]));
        let eq30_e1488_d_n12: f64 = ((eq30_e1486_d_n12 * s.v[780]) + (eq30_e1486 * s.dn[780][12]));
        let eq30_e1488_d_n13: f64 = ((eq30_e1486_d_n13 * s.v[780]) + (eq30_e1486 * s.dn[780][13]));
        let eq30_e1488_d_n14: f64 = ((eq30_e1486_d_n14 * s.v[780]) + (eq30_e1486 * s.dn[780][14]));
        let eq30_e1488_d_n15: f64 = ((eq30_e1486_d_n15 * s.v[780]) + (eq30_e1486 * s.dn[780][15]));
        let eq30_e1488_d_n16: f64 = ((eq30_e1486_d_n16 * s.v[780]) + (eq30_e1486 * s.dn[780][16]));
        let eq30_e1488_d_b0: f64 = ((eq30_e1486_d_b0 * s.v[780]) + (eq30_e1486 * s.db[780][0]));
        let eq30_e1488_d_b1: f64 = ((eq30_e1486_d_b1 * s.v[780]) + (eq30_e1486 * s.db[780][1]));
        let eq30_e1488_d_b2: f64 = ((eq30_e1486_d_b2 * s.v[780]) + (eq30_e1486 * s.db[780][2]));
        let eq30_e1488_d_b3: f64 = ((eq30_e1486_d_b3 * s.v[780]) + (eq30_e1486 * s.db[780][3]));
        let eq30_e1488_d_b4: f64 = ((eq30_e1486_d_b4 * s.v[780]) + (eq30_e1486 * s.db[780][4]));
        let eq30_e1488_d_b5: f64 = ((eq30_e1486_d_b5 * s.v[780]) + (eq30_e1486 * s.db[780][5]));
        let eq30_e1488_d_b6: f64 = ((eq30_e1486_d_b6 * s.v[780]) + (eq30_e1486 * s.db[780][6]));
        let eq30_e1488_d_b7: f64 = ((eq30_e1486_d_b7 * s.v[780]) + (eq30_e1486 * s.db[780][7]));
        let eq30_e1488_d_b8: f64 = ((eq30_e1486_d_b8 * s.v[780]) + (eq30_e1486 * s.db[780][8]));
        let eq30_e1488_d_b9: f64 = ((eq30_e1486_d_b9 * s.v[780]) + (eq30_e1486 * s.db[780][9]));
        let eq30_e1488_d_b10: f64 = ((eq30_e1486_d_b10 * s.v[780]) + (eq30_e1486 * s.db[780][10]));
        let eq30_e1488_d_b11: f64 = ((eq30_e1486_d_b11 * s.v[780]) + (eq30_e1486 * s.db[780][11]));
        let eq30_e1488_d_b12: f64 = ((eq30_e1486_d_b12 * s.v[780]) + (eq30_e1486 * s.db[780][12]));
        let eq30_e1488_d_b13: f64 = ((eq30_e1486_d_b13 * s.v[780]) + (eq30_e1486 * s.db[780][13]));
        (eq30_e1488, eq30_e1488_d_n0, eq30_e1488_d_n1, eq30_e1488_d_n2, eq30_e1488_d_n3, eq30_e1488_d_n4, eq30_e1488_d_n5, eq30_e1488_d_n6, eq30_e1488_d_n7, eq30_e1488_d_n8, eq30_e1488_d_n9, eq30_e1488_d_n10, eq30_e1488_d_n11, eq30_e1488_d_n12, eq30_e1488_d_n13, eq30_e1488_d_n14, eq30_e1488_d_n15, eq30_e1488_d_n16, eq30_e1488_d_b0, eq30_e1488_d_b1, eq30_e1488_d_b2, eq30_e1488_d_b3, eq30_e1488_d_b4, eq30_e1488_d_b5, eq30_e1488_d_b6, eq30_e1488_d_b7, eq30_e1488_d_b8, eq30_e1488_d_b9, eq30_e1488_d_b10, eq30_e1488_d_b11, eq30_e1488_d_b12, eq30_e1488_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1490;
        let eq30_node_derivatives: [f64; 17] = [eq30_e1490_d_n0, eq30_e1490_d_n1, eq30_e1490_d_n2, eq30_e1490_d_n3, eq30_e1490_d_n4, eq30_e1490_d_n5, eq30_e1490_d_n6, eq30_e1490_d_n7, eq30_e1490_d_n8, eq30_e1490_d_n9, eq30_e1490_d_n10, eq30_e1490_d_n11, eq30_e1490_d_n12, eq30_e1490_d_n13, eq30_e1490_d_n14, eq30_e1490_d_n15, eq30_e1490_d_n16];
        let eq30_branch_derivatives: [f64; 14] = [eq30_e1490_d_b0, eq30_e1490_d_b1, eq30_e1490_d_b2, eq30_e1490_d_b3, eq30_e1490_d_b4, eq30_e1490_d_b5, eq30_e1490_d_b6, eq30_e1490_d_b7, eq30_e1490_d_b8, eq30_e1490_d_b9, eq30_e1490_d_b10, eq30_e1490_d_b11, eq30_e1490_d_b12, eq30_e1490_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[11]),
            multiplicity * (eq30_value),
            nodes,
            &eq30_node_derivatives,
            branches,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let (eq31_e1494, eq31_e1494_d_n0, eq31_e1494_d_n1, eq31_e1494_d_n2, eq31_e1494_d_n3, eq31_e1494_d_n4, eq31_e1494_d_n5, eq31_e1494_d_n6, eq31_e1494_d_n7, eq31_e1494_d_n8, eq31_e1494_d_n9, eq31_e1494_d_n10, eq31_e1494_d_n11, eq31_e1494_d_n12, eq31_e1494_d_n13, eq31_e1494_d_n14, eq31_e1494_d_n15, eq31_e1494_d_n16, eq31_e1494_d_b0, eq31_e1494_d_b1, eq31_e1494_d_b2, eq31_e1494_d_b3, eq31_e1494_d_b4, eq31_e1494_d_b5, eq31_e1494_d_b6, eq31_e1494_d_b7, eq31_e1494_d_b8, eq31_e1494_d_b9, eq31_e1494_d_b10, eq31_e1494_d_b11, eq31_e1494_d_b12, eq31_e1494_d_b13,) = {
    if s.b[1611] {
        (s.v[826], s.dn[826][0], s.dn[826][1], s.dn[826][2], s.dn[826][3], s.dn[826][4], s.dn[826][5], s.dn[826][6], s.dn[826][7], s.dn[826][8], s.dn[826][9], s.dn[826][10], s.dn[826][11], s.dn[826][12], s.dn[826][13], s.dn[826][14], s.dn[826][15], s.dn[826][16], s.db[826][0], s.db[826][1], s.db[826][2], s.db[826][3], s.db[826][4], s.db[826][5], s.db[826][6], s.db[826][7], s.db[826][8], s.db[826][9], s.db[826][10], s.db[826][11], s.db[826][12], s.db[826][13],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e1494;
        let eq31_node_derivatives: [f64; 17] = [eq31_e1494_d_n0, eq31_e1494_d_n1, eq31_e1494_d_n2, eq31_e1494_d_n3, eq31_e1494_d_n4, eq31_e1494_d_n5, eq31_e1494_d_n6, eq31_e1494_d_n7, eq31_e1494_d_n8, eq31_e1494_d_n9, eq31_e1494_d_n10, eq31_e1494_d_n11, eq31_e1494_d_n12, eq31_e1494_d_n13, eq31_e1494_d_n14, eq31_e1494_d_n15, eq31_e1494_d_n16];
        let eq31_branch_derivatives: [f64; 14] = [eq31_e1494_d_b0, eq31_e1494_d_b1, eq31_e1494_d_b2, eq31_e1494_d_b3, eq31_e1494_d_b4, eq31_e1494_d_b5, eq31_e1494_d_b6, eq31_e1494_d_b7, eq31_e1494_d_b8, eq31_e1494_d_b9, eq31_e1494_d_b10, eq31_e1494_d_b11, eq31_e1494_d_b12, eq31_e1494_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            multiplicity * (eq31_value),
            nodes,
            &eq31_node_derivatives,
            branches,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let (eq32_e1499, eq32_e1499_d_n0, eq32_e1499_d_n1, eq32_e1499_d_n2, eq32_e1499_d_n3, eq32_e1499_d_n4, eq32_e1499_d_n5, eq32_e1499_d_n6, eq32_e1499_d_n7, eq32_e1499_d_n8, eq32_e1499_d_n9, eq32_e1499_d_n10, eq32_e1499_d_n11, eq32_e1499_d_n12, eq32_e1499_d_n13, eq32_e1499_d_n14, eq32_e1499_d_n15, eq32_e1499_d_n16, eq32_e1499_d_b0, eq32_e1499_d_b1, eq32_e1499_d_b2, eq32_e1499_d_b3, eq32_e1499_d_b4, eq32_e1499_d_b5, eq32_e1499_d_b6, eq32_e1499_d_b7, eq32_e1499_d_b8, eq32_e1499_d_b9, eq32_e1499_d_b10, eq32_e1499_d_b11, eq32_e1499_d_b12, eq32_e1499_d_b13,) = {
    if (!s.b[1611]) {
        (s.v[825], s.dn[825][0], s.dn[825][1], s.dn[825][2], s.dn[825][3], s.dn[825][4], s.dn[825][5], s.dn[825][6], s.dn[825][7], s.dn[825][8], s.dn[825][9], s.dn[825][10], s.dn[825][11], s.dn[825][12], s.dn[825][13], s.dn[825][14], s.dn[825][15], s.dn[825][16], s.db[825][0], s.db[825][1], s.db[825][2], s.db[825][3], s.db[825][4], s.db[825][5], s.db[825][6], s.db[825][7], s.db[825][8], s.db[825][9], s.db[825][10], s.db[825][11], s.db[825][12], s.db[825][13],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e1499;
        let eq32_node_derivatives: [f64; 17] = [eq32_e1499_d_n0, eq32_e1499_d_n1, eq32_e1499_d_n2, eq32_e1499_d_n3, eq32_e1499_d_n4, eq32_e1499_d_n5, eq32_e1499_d_n6, eq32_e1499_d_n7, eq32_e1499_d_n8, eq32_e1499_d_n9, eq32_e1499_d_n10, eq32_e1499_d_n11, eq32_e1499_d_n12, eq32_e1499_d_n13, eq32_e1499_d_n14, eq32_e1499_d_n15, eq32_e1499_d_n16];
        let eq32_branch_derivatives: [f64; 14] = [eq32_e1499_d_b0, eq32_e1499_d_b1, eq32_e1499_d_b2, eq32_e1499_d_b3, eq32_e1499_d_b4, eq32_e1499_d_b5, eq32_e1499_d_b6, eq32_e1499_d_b7, eq32_e1499_d_b8, eq32_e1499_d_b9, eq32_e1499_d_b10, eq32_e1499_d_b11, eq32_e1499_d_b12, eq32_e1499_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[11]),
            multiplicity * (eq32_value),
            nodes,
            &eq32_node_derivatives,
            branches,
            &eq32_branch_derivatives,
            multiplicity,
        );
        let (eq33_e1506, eq33_e1506_d_n0, eq33_e1506_d_n1, eq33_e1506_d_n2, eq33_e1506_d_n3, eq33_e1506_d_n4, eq33_e1506_d_n5, eq33_e1506_d_n6, eq33_e1506_d_n7, eq33_e1506_d_n8, eq33_e1506_d_n9, eq33_e1506_d_n10, eq33_e1506_d_n11, eq33_e1506_d_n12, eq33_e1506_d_n13, eq33_e1506_d_n14, eq33_e1506_d_n15, eq33_e1506_d_n16, eq33_e1506_d_b0, eq33_e1506_d_b1, eq33_e1506_d_b2, eq33_e1506_d_b3, eq33_e1506_d_b4, eq33_e1506_d_b5, eq33_e1506_d_b6, eq33_e1506_d_b7, eq33_e1506_d_b8, eq33_e1506_d_b9, eq33_e1506_d_b10, eq33_e1506_d_b11, eq33_e1506_d_b12, eq33_e1506_d_b13,) = {
    if (!s.b[1611]) {
        let eq33_e1504: f64 = (s.v[824] + s.v[826]);
        let eq33_e1504_d_n0: f64 = (s.dn[824][0] + s.dn[826][0]);
        let eq33_e1504_d_n1: f64 = (s.dn[824][1] + s.dn[826][1]);
        let eq33_e1504_d_n2: f64 = (s.dn[824][2] + s.dn[826][2]);
        let eq33_e1504_d_n3: f64 = (s.dn[824][3] + s.dn[826][3]);
        let eq33_e1504_d_n4: f64 = (s.dn[824][4] + s.dn[826][4]);
        let eq33_e1504_d_n5: f64 = (s.dn[824][5] + s.dn[826][5]);
        let eq33_e1504_d_n6: f64 = (s.dn[824][6] + s.dn[826][6]);
        let eq33_e1504_d_n7: f64 = (s.dn[824][7] + s.dn[826][7]);
        let eq33_e1504_d_n8: f64 = (s.dn[824][8] + s.dn[826][8]);
        let eq33_e1504_d_n9: f64 = (s.dn[824][9] + s.dn[826][9]);
        let eq33_e1504_d_n10: f64 = (s.dn[824][10] + s.dn[826][10]);
        let eq33_e1504_d_n11: f64 = (s.dn[824][11] + s.dn[826][11]);
        let eq33_e1504_d_n12: f64 = (s.dn[824][12] + s.dn[826][12]);
        let eq33_e1504_d_n13: f64 = (s.dn[824][13] + s.dn[826][13]);
        let eq33_e1504_d_n14: f64 = (s.dn[824][14] + s.dn[826][14]);
        let eq33_e1504_d_n15: f64 = (s.dn[824][15] + s.dn[826][15]);
        let eq33_e1504_d_n16: f64 = (s.dn[824][16] + s.dn[826][16]);
        let eq33_e1504_d_b0: f64 = (s.db[824][0] + s.db[826][0]);
        let eq33_e1504_d_b1: f64 = (s.db[824][1] + s.db[826][1]);
        let eq33_e1504_d_b2: f64 = (s.db[824][2] + s.db[826][2]);
        let eq33_e1504_d_b3: f64 = (s.db[824][3] + s.db[826][3]);
        let eq33_e1504_d_b4: f64 = (s.db[824][4] + s.db[826][4]);
        let eq33_e1504_d_b5: f64 = (s.db[824][5] + s.db[826][5]);
        let eq33_e1504_d_b6: f64 = (s.db[824][6] + s.db[826][6]);
        let eq33_e1504_d_b7: f64 = (s.db[824][7] + s.db[826][7]);
        let eq33_e1504_d_b8: f64 = (s.db[824][8] + s.db[826][8]);
        let eq33_e1504_d_b9: f64 = (s.db[824][9] + s.db[826][9]);
        let eq33_e1504_d_b10: f64 = (s.db[824][10] + s.db[826][10]);
        let eq33_e1504_d_b11: f64 = (s.db[824][11] + s.db[826][11]);
        let eq33_e1504_d_b12: f64 = (s.db[824][12] + s.db[826][12]);
        let eq33_e1504_d_b13: f64 = (s.db[824][13] + s.db[826][13]);
        (eq33_e1504, eq33_e1504_d_n0, eq33_e1504_d_n1, eq33_e1504_d_n2, eq33_e1504_d_n3, eq33_e1504_d_n4, eq33_e1504_d_n5, eq33_e1504_d_n6, eq33_e1504_d_n7, eq33_e1504_d_n8, eq33_e1504_d_n9, eq33_e1504_d_n10, eq33_e1504_d_n11, eq33_e1504_d_n12, eq33_e1504_d_n13, eq33_e1504_d_n14, eq33_e1504_d_n15, eq33_e1504_d_n16, eq33_e1504_d_b0, eq33_e1504_d_b1, eq33_e1504_d_b2, eq33_e1504_d_b3, eq33_e1504_d_b4, eq33_e1504_d_b5, eq33_e1504_d_b6, eq33_e1504_d_b7, eq33_e1504_d_b8, eq33_e1504_d_b9, eq33_e1504_d_b10, eq33_e1504_d_b11, eq33_e1504_d_b12, eq33_e1504_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e1506;
        let eq33_node_derivatives: [f64; 17] = [eq33_e1506_d_n0, eq33_e1506_d_n1, eq33_e1506_d_n2, eq33_e1506_d_n3, eq33_e1506_d_n4, eq33_e1506_d_n5, eq33_e1506_d_n6, eq33_e1506_d_n7, eq33_e1506_d_n8, eq33_e1506_d_n9, eq33_e1506_d_n10, eq33_e1506_d_n11, eq33_e1506_d_n12, eq33_e1506_d_n13, eq33_e1506_d_n14, eq33_e1506_d_n15, eq33_e1506_d_n16];
        let eq33_branch_derivatives: [f64; 14] = [eq33_e1506_d_b0, eq33_e1506_d_b1, eq33_e1506_d_b2, eq33_e1506_d_b3, eq33_e1506_d_b4, eq33_e1506_d_b5, eq33_e1506_d_b6, eq33_e1506_d_b7, eq33_e1506_d_b8, eq33_e1506_d_b9, eq33_e1506_d_b10, eq33_e1506_d_b11, eq33_e1506_d_b12, eq33_e1506_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            multiplicity * (eq33_value),
            nodes,
            &eq33_node_derivatives,
            branches,
            &eq33_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_5(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq34_e1514, eq34_e1514_d_n0, eq34_e1514_d_n1, eq34_e1514_d_n2, eq34_e1514_d_n3, eq34_e1514_d_n4, eq34_e1514_d_n5, eq34_e1514_d_n6, eq34_e1514_d_n7, eq34_e1514_d_n8, eq34_e1514_d_n9, eq34_e1514_d_n10, eq34_e1514_d_n11, eq34_e1514_d_n12, eq34_e1514_d_n13, eq34_e1514_d_n14, eq34_e1514_d_n15, eq34_e1514_d_n16, eq34_e1514_d_b0, eq34_e1514_d_b1, eq34_e1514_d_b2, eq34_e1514_d_b3, eq34_e1514_d_b4, eq34_e1514_d_b5, eq34_e1514_d_b6, eq34_e1514_d_b7, eq34_e1514_d_b8, eq34_e1514_d_b9, eq34_e1514_d_b10, eq34_e1514_d_b11, eq34_e1514_d_b12, eq34_e1514_d_b13,) = {
    if s.b[1612] {
        let eq34_e1510: f64 = (p.p28 * (nv0 - nv6));
        let eq34_e1510_d_n0: f64 = p.p28;
        let eq34_e1510_d_n6: f64 = (-p.p28);
        let eq34_e1512: f64 = (eq34_e1510 * s.v[372]);
        let eq34_e1512_d_n0: f64 = ((eq34_e1510_d_n0 * s.v[372]) + (eq34_e1510 * s.dn[372][0]));
        let eq34_e1512_d_n1: f64 = (eq34_e1510 * s.dn[372][1]);
        let eq34_e1512_d_n2: f64 = (eq34_e1510 * s.dn[372][2]);
        let eq34_e1512_d_n3: f64 = (eq34_e1510 * s.dn[372][3]);
        let eq34_e1512_d_n4: f64 = (eq34_e1510 * s.dn[372][4]);
        let eq34_e1512_d_n5: f64 = (eq34_e1510 * s.dn[372][5]);
        let eq34_e1512_d_n6: f64 = ((eq34_e1510_d_n6 * s.v[372]) + (eq34_e1510 * s.dn[372][6]));
        let eq34_e1512_d_n7: f64 = (eq34_e1510 * s.dn[372][7]);
        let eq34_e1512_d_n8: f64 = (eq34_e1510 * s.dn[372][8]);
        let eq34_e1512_d_n9: f64 = (eq34_e1510 * s.dn[372][9]);
        let eq34_e1512_d_n10: f64 = (eq34_e1510 * s.dn[372][10]);
        let eq34_e1512_d_n11: f64 = (eq34_e1510 * s.dn[372][11]);
        let eq34_e1512_d_n12: f64 = (eq34_e1510 * s.dn[372][12]);
        let eq34_e1512_d_n13: f64 = (eq34_e1510 * s.dn[372][13]);
        let eq34_e1512_d_n14: f64 = (eq34_e1510 * s.dn[372][14]);
        let eq34_e1512_d_n15: f64 = (eq34_e1510 * s.dn[372][15]);
        let eq34_e1512_d_n16: f64 = (eq34_e1510 * s.dn[372][16]);
        let eq34_e1512_d_b0: f64 = (eq34_e1510 * s.db[372][0]);
        let eq34_e1512_d_b1: f64 = (eq34_e1510 * s.db[372][1]);
        let eq34_e1512_d_b2: f64 = (eq34_e1510 * s.db[372][2]);
        let eq34_e1512_d_b3: f64 = (eq34_e1510 * s.db[372][3]);
        let eq34_e1512_d_b4: f64 = (eq34_e1510 * s.db[372][4]);
        let eq34_e1512_d_b5: f64 = (eq34_e1510 * s.db[372][5]);
        let eq34_e1512_d_b6: f64 = (eq34_e1510 * s.db[372][6]);
        let eq34_e1512_d_b7: f64 = (eq34_e1510 * s.db[372][7]);
        let eq34_e1512_d_b8: f64 = (eq34_e1510 * s.db[372][8]);
        let eq34_e1512_d_b9: f64 = (eq34_e1510 * s.db[372][9]);
        let eq34_e1512_d_b10: f64 = (eq34_e1510 * s.db[372][10]);
        let eq34_e1512_d_b11: f64 = (eq34_e1510 * s.db[372][11]);
        let eq34_e1512_d_b12: f64 = (eq34_e1510 * s.db[372][12]);
        let eq34_e1512_d_b13: f64 = (eq34_e1510 * s.db[372][13]);
        (eq34_e1512, eq34_e1512_d_n0, eq34_e1512_d_n1, eq34_e1512_d_n2, eq34_e1512_d_n3, eq34_e1512_d_n4, eq34_e1512_d_n5, eq34_e1512_d_n6, eq34_e1512_d_n7, eq34_e1512_d_n8, eq34_e1512_d_n9, eq34_e1512_d_n10, eq34_e1512_d_n11, eq34_e1512_d_n12, eq34_e1512_d_n13, eq34_e1512_d_n14, eq34_e1512_d_n15, eq34_e1512_d_n16, eq34_e1512_d_b0, eq34_e1512_d_b1, eq34_e1512_d_b2, eq34_e1512_d_b3, eq34_e1512_d_b4, eq34_e1512_d_b5, eq34_e1512_d_b6, eq34_e1512_d_b7, eq34_e1512_d_b8, eq34_e1512_d_b9, eq34_e1512_d_b10, eq34_e1512_d_b11, eq34_e1512_d_b12, eq34_e1512_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e1514;
        let eq34_node_derivatives: [f64; 17] = [eq34_e1514_d_n0, eq34_e1514_d_n1, eq34_e1514_d_n2, eq34_e1514_d_n3, eq34_e1514_d_n4, eq34_e1514_d_n5, eq34_e1514_d_n6, eq34_e1514_d_n7, eq34_e1514_d_n8, eq34_e1514_d_n9, eq34_e1514_d_n10, eq34_e1514_d_n11, eq34_e1514_d_n12, eq34_e1514_d_n13, eq34_e1514_d_n14, eq34_e1514_d_n15, eq34_e1514_d_n16];
        let eq34_branch_derivatives: [f64; 14] = [eq34_e1514_d_b0, eq34_e1514_d_b1, eq34_e1514_d_b2, eq34_e1514_d_b3, eq34_e1514_d_b4, eq34_e1514_d_b5, eq34_e1514_d_b6, eq34_e1514_d_b7, eq34_e1514_d_b8, eq34_e1514_d_b9, eq34_e1514_d_b10, eq34_e1514_d_b11, eq34_e1514_d_b12, eq34_e1514_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[6]),
            multiplicity * (eq34_value),
            nodes,
            &eq34_node_derivatives,
            branches,
            &eq34_branch_derivatives,
            multiplicity,
        );
        let (eq35_e1524,) = {
    if s.b[1612] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e1524;
        stamper.stamp_current_const(
            Some(nodes[0]),
            Some(nodes[6]),
            multiplicity * (eq35_value),
        );
        let (eq36_e1534, eq36_e1534_d_n0, eq36_e1534_d_n1, eq36_e1534_d_n2, eq36_e1534_d_n3, eq36_e1534_d_n4, eq36_e1534_d_n5, eq36_e1534_d_n6, eq36_e1534_d_n7, eq36_e1534_d_n8, eq36_e1534_d_n9, eq36_e1534_d_n10, eq36_e1534_d_n11, eq36_e1534_d_n12, eq36_e1534_d_n13, eq36_e1534_d_n14, eq36_e1534_d_n15, eq36_e1534_d_n16, eq36_e1534_d_b0, eq36_e1534_d_b1, eq36_e1534_d_b2, eq36_e1534_d_b3, eq36_e1534_d_b4, eq36_e1534_d_b5, eq36_e1534_d_b6, eq36_e1534_d_b7, eq36_e1534_d_b8, eq36_e1534_d_b9, eq36_e1534_d_b10, eq36_e1534_d_b11, eq36_e1534_d_b12, eq36_e1534_d_b13,) = {
    if (s.b[1612] && s.b[1613]) {
        let eq36_e1530: f64 = (p.p28 * (nv6 - nv5));
        let eq36_e1530_d_n5: f64 = (-p.p28);
        let eq36_e1530_d_n6: f64 = p.p28;
        let eq36_e1532: f64 = (eq36_e1530 * s.v[374]);
        let eq36_e1532_d_n0: f64 = (eq36_e1530 * s.dn[374][0]);
        let eq36_e1532_d_n1: f64 = (eq36_e1530 * s.dn[374][1]);
        let eq36_e1532_d_n2: f64 = (eq36_e1530 * s.dn[374][2]);
        let eq36_e1532_d_n3: f64 = (eq36_e1530 * s.dn[374][3]);
        let eq36_e1532_d_n4: f64 = (eq36_e1530 * s.dn[374][4]);
        let eq36_e1532_d_n5: f64 = ((eq36_e1530_d_n5 * s.v[374]) + (eq36_e1530 * s.dn[374][5]));
        let eq36_e1532_d_n6: f64 = ((eq36_e1530_d_n6 * s.v[374]) + (eq36_e1530 * s.dn[374][6]));
        let eq36_e1532_d_n7: f64 = (eq36_e1530 * s.dn[374][7]);
        let eq36_e1532_d_n8: f64 = (eq36_e1530 * s.dn[374][8]);
        let eq36_e1532_d_n9: f64 = (eq36_e1530 * s.dn[374][9]);
        let eq36_e1532_d_n10: f64 = (eq36_e1530 * s.dn[374][10]);
        let eq36_e1532_d_n11: f64 = (eq36_e1530 * s.dn[374][11]);
        let eq36_e1532_d_n12: f64 = (eq36_e1530 * s.dn[374][12]);
        let eq36_e1532_d_n13: f64 = (eq36_e1530 * s.dn[374][13]);
        let eq36_e1532_d_n14: f64 = (eq36_e1530 * s.dn[374][14]);
        let eq36_e1532_d_n15: f64 = (eq36_e1530 * s.dn[374][15]);
        let eq36_e1532_d_n16: f64 = (eq36_e1530 * s.dn[374][16]);
        let eq36_e1532_d_b0: f64 = (eq36_e1530 * s.db[374][0]);
        let eq36_e1532_d_b1: f64 = (eq36_e1530 * s.db[374][1]);
        let eq36_e1532_d_b2: f64 = (eq36_e1530 * s.db[374][2]);
        let eq36_e1532_d_b3: f64 = (eq36_e1530 * s.db[374][3]);
        let eq36_e1532_d_b4: f64 = (eq36_e1530 * s.db[374][4]);
        let eq36_e1532_d_b5: f64 = (eq36_e1530 * s.db[374][5]);
        let eq36_e1532_d_b6: f64 = (eq36_e1530 * s.db[374][6]);
        let eq36_e1532_d_b7: f64 = (eq36_e1530 * s.db[374][7]);
        let eq36_e1532_d_b8: f64 = (eq36_e1530 * s.db[374][8]);
        let eq36_e1532_d_b9: f64 = (eq36_e1530 * s.db[374][9]);
        let eq36_e1532_d_b10: f64 = (eq36_e1530 * s.db[374][10]);
        let eq36_e1532_d_b11: f64 = (eq36_e1530 * s.db[374][11]);
        let eq36_e1532_d_b12: f64 = (eq36_e1530 * s.db[374][12]);
        let eq36_e1532_d_b13: f64 = (eq36_e1530 * s.db[374][13]);
        (eq36_e1532, eq36_e1532_d_n0, eq36_e1532_d_n1, eq36_e1532_d_n2, eq36_e1532_d_n3, eq36_e1532_d_n4, eq36_e1532_d_n5, eq36_e1532_d_n6, eq36_e1532_d_n7, eq36_e1532_d_n8, eq36_e1532_d_n9, eq36_e1532_d_n10, eq36_e1532_d_n11, eq36_e1532_d_n12, eq36_e1532_d_n13, eq36_e1532_d_n14, eq36_e1532_d_n15, eq36_e1532_d_n16, eq36_e1532_d_b0, eq36_e1532_d_b1, eq36_e1532_d_b2, eq36_e1532_d_b3, eq36_e1532_d_b4, eq36_e1532_d_b5, eq36_e1532_d_b6, eq36_e1532_d_b7, eq36_e1532_d_b8, eq36_e1532_d_b9, eq36_e1532_d_b10, eq36_e1532_d_b11, eq36_e1532_d_b12, eq36_e1532_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e1534;
        let eq36_node_derivatives: [f64; 17] = [eq36_e1534_d_n0, eq36_e1534_d_n1, eq36_e1534_d_n2, eq36_e1534_d_n3, eq36_e1534_d_n4, eq36_e1534_d_n5, eq36_e1534_d_n6, eq36_e1534_d_n7, eq36_e1534_d_n8, eq36_e1534_d_n9, eq36_e1534_d_n10, eq36_e1534_d_n11, eq36_e1534_d_n12, eq36_e1534_d_n13, eq36_e1534_d_n14, eq36_e1534_d_n15, eq36_e1534_d_n16];
        let eq36_branch_derivatives: [f64; 14] = [eq36_e1534_d_b0, eq36_e1534_d_b1, eq36_e1534_d_b2, eq36_e1534_d_b3, eq36_e1534_d_b4, eq36_e1534_d_b5, eq36_e1534_d_b6, eq36_e1534_d_b7, eq36_e1534_d_b8, eq36_e1534_d_b9, eq36_e1534_d_b10, eq36_e1534_d_b11, eq36_e1534_d_b12, eq36_e1534_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[5]),
            multiplicity * (eq36_value),
            nodes,
            &eq36_node_derivatives,
            branches,
            &eq36_branch_derivatives,
            multiplicity,
        );
        let (eq37_e1546,) = {
    if (s.b[1612] && s.b[1613]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq37_value: f64 = eq37_e1546;
        stamper.stamp_current_const(
            Some(nodes[6]),
            Some(nodes[5]),
            multiplicity * (eq37_value),
        );
        let (eq38_e1565,) = {
    if (s.b[1612] && s.b[1613]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq38_value: f64 = eq38_e1565;
        stamper.stamp_current_const(
            Some(nodes[6]),
            Some(nodes[5]),
            multiplicity * (eq38_value),
        );
        let (eq39_e1572,) = {
    if (s.b[1612] && (!s.b[1613])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq39_value: f64 = eq39_e1572;
        stamper.stamp_potential_const(
            branches[1],
            eq39_value,
        );
        let (eq40_e1577,) = {
    if (!s.b[1612]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq40_value: f64 = eq40_e1577;
        stamper.stamp_potential_const(
            branches[2],
            eq40_value,
        );
        let (eq41_e1582,) = {
    if (!s.b[1612]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq41_value: f64 = eq41_e1582;
        stamper.stamp_potential_const(
            branches[3],
            eq41_value,
        );
        let (eq42_e1590, eq42_e1590_d_n0, eq42_e1590_d_n1, eq42_e1590_d_n2, eq42_e1590_d_n3, eq42_e1590_d_n4, eq42_e1590_d_n5, eq42_e1590_d_n6, eq42_e1590_d_n7, eq42_e1590_d_n8, eq42_e1590_d_n9, eq42_e1590_d_n10, eq42_e1590_d_n11, eq42_e1590_d_n12, eq42_e1590_d_n13, eq42_e1590_d_n14, eq42_e1590_d_n15, eq42_e1590_d_n16, eq42_e1590_d_b0, eq42_e1590_d_b1, eq42_e1590_d_b2, eq42_e1590_d_b3, eq42_e1590_d_b4, eq42_e1590_d_b5, eq42_e1590_d_b6, eq42_e1590_d_b7, eq42_e1590_d_b8, eq42_e1590_d_b9, eq42_e1590_d_b10, eq42_e1590_d_b11, eq42_e1590_d_b12, eq42_e1590_d_b13,) = {
    if s.b[1614] {
        let eq42_e1586: f64 = (p.p28 * (nv2 - nv8));
        let eq42_e1586_d_n2: f64 = p.p28;
        let eq42_e1586_d_n8: f64 = (-p.p28);
        let eq42_e1588: f64 = (eq42_e1586 * s.v[371]);
        let eq42_e1588_d_n0: f64 = (eq42_e1586 * s.dn[371][0]);
        let eq42_e1588_d_n1: f64 = (eq42_e1586 * s.dn[371][1]);
        let eq42_e1588_d_n2: f64 = ((eq42_e1586_d_n2 * s.v[371]) + (eq42_e1586 * s.dn[371][2]));
        let eq42_e1588_d_n3: f64 = (eq42_e1586 * s.dn[371][3]);
        let eq42_e1588_d_n4: f64 = (eq42_e1586 * s.dn[371][4]);
        let eq42_e1588_d_n5: f64 = (eq42_e1586 * s.dn[371][5]);
        let eq42_e1588_d_n6: f64 = (eq42_e1586 * s.dn[371][6]);
        let eq42_e1588_d_n7: f64 = (eq42_e1586 * s.dn[371][7]);
        let eq42_e1588_d_n8: f64 = ((eq42_e1586_d_n8 * s.v[371]) + (eq42_e1586 * s.dn[371][8]));
        let eq42_e1588_d_n9: f64 = (eq42_e1586 * s.dn[371][9]);
        let eq42_e1588_d_n10: f64 = (eq42_e1586 * s.dn[371][10]);
        let eq42_e1588_d_n11: f64 = (eq42_e1586 * s.dn[371][11]);
        let eq42_e1588_d_n12: f64 = (eq42_e1586 * s.dn[371][12]);
        let eq42_e1588_d_n13: f64 = (eq42_e1586 * s.dn[371][13]);
        let eq42_e1588_d_n14: f64 = (eq42_e1586 * s.dn[371][14]);
        let eq42_e1588_d_n15: f64 = (eq42_e1586 * s.dn[371][15]);
        let eq42_e1588_d_n16: f64 = (eq42_e1586 * s.dn[371][16]);
        let eq42_e1588_d_b0: f64 = (eq42_e1586 * s.db[371][0]);
        let eq42_e1588_d_b1: f64 = (eq42_e1586 * s.db[371][1]);
        let eq42_e1588_d_b2: f64 = (eq42_e1586 * s.db[371][2]);
        let eq42_e1588_d_b3: f64 = (eq42_e1586 * s.db[371][3]);
        let eq42_e1588_d_b4: f64 = (eq42_e1586 * s.db[371][4]);
        let eq42_e1588_d_b5: f64 = (eq42_e1586 * s.db[371][5]);
        let eq42_e1588_d_b6: f64 = (eq42_e1586 * s.db[371][6]);
        let eq42_e1588_d_b7: f64 = (eq42_e1586 * s.db[371][7]);
        let eq42_e1588_d_b8: f64 = (eq42_e1586 * s.db[371][8]);
        let eq42_e1588_d_b9: f64 = (eq42_e1586 * s.db[371][9]);
        let eq42_e1588_d_b10: f64 = (eq42_e1586 * s.db[371][10]);
        let eq42_e1588_d_b11: f64 = (eq42_e1586 * s.db[371][11]);
        let eq42_e1588_d_b12: f64 = (eq42_e1586 * s.db[371][12]);
        let eq42_e1588_d_b13: f64 = (eq42_e1586 * s.db[371][13]);
        (eq42_e1588, eq42_e1588_d_n0, eq42_e1588_d_n1, eq42_e1588_d_n2, eq42_e1588_d_n3, eq42_e1588_d_n4, eq42_e1588_d_n5, eq42_e1588_d_n6, eq42_e1588_d_n7, eq42_e1588_d_n8, eq42_e1588_d_n9, eq42_e1588_d_n10, eq42_e1588_d_n11, eq42_e1588_d_n12, eq42_e1588_d_n13, eq42_e1588_d_n14, eq42_e1588_d_n15, eq42_e1588_d_n16, eq42_e1588_d_b0, eq42_e1588_d_b1, eq42_e1588_d_b2, eq42_e1588_d_b3, eq42_e1588_d_b4, eq42_e1588_d_b5, eq42_e1588_d_b6, eq42_e1588_d_b7, eq42_e1588_d_b8, eq42_e1588_d_b9, eq42_e1588_d_b10, eq42_e1588_d_b11, eq42_e1588_d_b12, eq42_e1588_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_value: f64 = eq42_e1590;
        let eq42_node_derivatives: [f64; 17] = [eq42_e1590_d_n0, eq42_e1590_d_n1, eq42_e1590_d_n2, eq42_e1590_d_n3, eq42_e1590_d_n4, eq42_e1590_d_n5, eq42_e1590_d_n6, eq42_e1590_d_n7, eq42_e1590_d_n8, eq42_e1590_d_n9, eq42_e1590_d_n10, eq42_e1590_d_n11, eq42_e1590_d_n12, eq42_e1590_d_n13, eq42_e1590_d_n14, eq42_e1590_d_n15, eq42_e1590_d_n16];
        let eq42_branch_derivatives: [f64; 14] = [eq42_e1590_d_b0, eq42_e1590_d_b1, eq42_e1590_d_b2, eq42_e1590_d_b3, eq42_e1590_d_b4, eq42_e1590_d_b5, eq42_e1590_d_b6, eq42_e1590_d_b7, eq42_e1590_d_b8, eq42_e1590_d_b9, eq42_e1590_d_b10, eq42_e1590_d_b11, eq42_e1590_d_b12, eq42_e1590_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            multiplicity * (eq42_value),
            nodes,
            &eq42_node_derivatives,
            branches,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let (eq43_e1600,) = {
    if s.b[1614] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq43_value: f64 = eq43_e1600;
        stamper.stamp_current_const(
            Some(nodes[2]),
            Some(nodes[8]),
            multiplicity * (eq43_value),
        );
        let (eq44_e1610, eq44_e1610_d_n0, eq44_e1610_d_n1, eq44_e1610_d_n2, eq44_e1610_d_n3, eq44_e1610_d_n4, eq44_e1610_d_n5, eq44_e1610_d_n6, eq44_e1610_d_n7, eq44_e1610_d_n8, eq44_e1610_d_n9, eq44_e1610_d_n10, eq44_e1610_d_n11, eq44_e1610_d_n12, eq44_e1610_d_n13, eq44_e1610_d_n14, eq44_e1610_d_n15, eq44_e1610_d_n16, eq44_e1610_d_b0, eq44_e1610_d_b1, eq44_e1610_d_b2, eq44_e1610_d_b3, eq44_e1610_d_b4, eq44_e1610_d_b5, eq44_e1610_d_b6, eq44_e1610_d_b7, eq44_e1610_d_b8, eq44_e1610_d_b9, eq44_e1610_d_b10, eq44_e1610_d_b11, eq44_e1610_d_b12, eq44_e1610_d_b13,) = {
    if (s.b[1614] && s.b[1615]) {
        let eq44_e1606: f64 = (p.p28 * (nv8 - nv7));
        let eq44_e1606_d_n7: f64 = (-p.p28);
        let eq44_e1606_d_n8: f64 = p.p28;
        let eq44_e1608: f64 = (eq44_e1606 * s.v[373]);
        let eq44_e1608_d_n0: f64 = (eq44_e1606 * s.dn[373][0]);
        let eq44_e1608_d_n1: f64 = (eq44_e1606 * s.dn[373][1]);
        let eq44_e1608_d_n2: f64 = (eq44_e1606 * s.dn[373][2]);
        let eq44_e1608_d_n3: f64 = (eq44_e1606 * s.dn[373][3]);
        let eq44_e1608_d_n4: f64 = (eq44_e1606 * s.dn[373][4]);
        let eq44_e1608_d_n5: f64 = (eq44_e1606 * s.dn[373][5]);
        let eq44_e1608_d_n6: f64 = (eq44_e1606 * s.dn[373][6]);
        let eq44_e1608_d_n7: f64 = ((eq44_e1606_d_n7 * s.v[373]) + (eq44_e1606 * s.dn[373][7]));
        let eq44_e1608_d_n8: f64 = ((eq44_e1606_d_n8 * s.v[373]) + (eq44_e1606 * s.dn[373][8]));
        let eq44_e1608_d_n9: f64 = (eq44_e1606 * s.dn[373][9]);
        let eq44_e1608_d_n10: f64 = (eq44_e1606 * s.dn[373][10]);
        let eq44_e1608_d_n11: f64 = (eq44_e1606 * s.dn[373][11]);
        let eq44_e1608_d_n12: f64 = (eq44_e1606 * s.dn[373][12]);
        let eq44_e1608_d_n13: f64 = (eq44_e1606 * s.dn[373][13]);
        let eq44_e1608_d_n14: f64 = (eq44_e1606 * s.dn[373][14]);
        let eq44_e1608_d_n15: f64 = (eq44_e1606 * s.dn[373][15]);
        let eq44_e1608_d_n16: f64 = (eq44_e1606 * s.dn[373][16]);
        let eq44_e1608_d_b0: f64 = (eq44_e1606 * s.db[373][0]);
        let eq44_e1608_d_b1: f64 = (eq44_e1606 * s.db[373][1]);
        let eq44_e1608_d_b2: f64 = (eq44_e1606 * s.db[373][2]);
        let eq44_e1608_d_b3: f64 = (eq44_e1606 * s.db[373][3]);
        let eq44_e1608_d_b4: f64 = (eq44_e1606 * s.db[373][4]);
        let eq44_e1608_d_b5: f64 = (eq44_e1606 * s.db[373][5]);
        let eq44_e1608_d_b6: f64 = (eq44_e1606 * s.db[373][6]);
        let eq44_e1608_d_b7: f64 = (eq44_e1606 * s.db[373][7]);
        let eq44_e1608_d_b8: f64 = (eq44_e1606 * s.db[373][8]);
        let eq44_e1608_d_b9: f64 = (eq44_e1606 * s.db[373][9]);
        let eq44_e1608_d_b10: f64 = (eq44_e1606 * s.db[373][10]);
        let eq44_e1608_d_b11: f64 = (eq44_e1606 * s.db[373][11]);
        let eq44_e1608_d_b12: f64 = (eq44_e1606 * s.db[373][12]);
        let eq44_e1608_d_b13: f64 = (eq44_e1606 * s.db[373][13]);
        (eq44_e1608, eq44_e1608_d_n0, eq44_e1608_d_n1, eq44_e1608_d_n2, eq44_e1608_d_n3, eq44_e1608_d_n4, eq44_e1608_d_n5, eq44_e1608_d_n6, eq44_e1608_d_n7, eq44_e1608_d_n8, eq44_e1608_d_n9, eq44_e1608_d_n10, eq44_e1608_d_n11, eq44_e1608_d_n12, eq44_e1608_d_n13, eq44_e1608_d_n14, eq44_e1608_d_n15, eq44_e1608_d_n16, eq44_e1608_d_b0, eq44_e1608_d_b1, eq44_e1608_d_b2, eq44_e1608_d_b3, eq44_e1608_d_b4, eq44_e1608_d_b5, eq44_e1608_d_b6, eq44_e1608_d_b7, eq44_e1608_d_b8, eq44_e1608_d_b9, eq44_e1608_d_b10, eq44_e1608_d_b11, eq44_e1608_d_b12, eq44_e1608_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e1610;
        let eq44_node_derivatives: [f64; 17] = [eq44_e1610_d_n0, eq44_e1610_d_n1, eq44_e1610_d_n2, eq44_e1610_d_n3, eq44_e1610_d_n4, eq44_e1610_d_n5, eq44_e1610_d_n6, eq44_e1610_d_n7, eq44_e1610_d_n8, eq44_e1610_d_n9, eq44_e1610_d_n10, eq44_e1610_d_n11, eq44_e1610_d_n12, eq44_e1610_d_n13, eq44_e1610_d_n14, eq44_e1610_d_n15, eq44_e1610_d_n16];
        let eq44_branch_derivatives: [f64; 14] = [eq44_e1610_d_b0, eq44_e1610_d_b1, eq44_e1610_d_b2, eq44_e1610_d_b3, eq44_e1610_d_b4, eq44_e1610_d_b5, eq44_e1610_d_b6, eq44_e1610_d_b7, eq44_e1610_d_b8, eq44_e1610_d_b9, eq44_e1610_d_b10, eq44_e1610_d_b11, eq44_e1610_d_b12, eq44_e1610_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            multiplicity * (eq44_value),
            nodes,
            &eq44_node_derivatives,
            branches,
            &eq44_branch_derivatives,
            multiplicity,
        );
        let (eq45_e1622,) = {
    if (s.b[1614] && s.b[1615]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq45_value: f64 = eq45_e1622;
        stamper.stamp_current_const(
            Some(nodes[8]),
            Some(nodes[7]),
            multiplicity * (eq45_value),
        );
        let (eq46_e1641,) = {
    if (s.b[1614] && s.b[1615]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq46_value: f64 = eq46_e1641;
        stamper.stamp_current_const(
            Some(nodes[8]),
            Some(nodes[7]),
            multiplicity * (eq46_value),
        );
        let (eq47_e1648,) = {
    if (s.b[1614] && (!s.b[1615])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq47_value: f64 = eq47_e1648;
        stamper.stamp_potential_const(
            branches[4],
            eq47_value,
        );
        let (eq48_e1653,) = {
    if (!s.b[1614]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq48_value: f64 = eq48_e1653;
        stamper.stamp_potential_const(
            branches[5],
            eq48_value,
        );
        let (eq49_e1658,) = {
    if (!s.b[1614]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e1658;
        stamper.stamp_potential_const(
            branches[6],
            eq49_value,
        );
        let (eq50_e1662,) = {
    if s.b[1616] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq50_value: f64 = eq50_e1662;
        stamper.stamp_potential_const(
            branches[7],
            eq50_value,
        );
        let (eq51_e1671, eq51_e1671_d_n0, eq51_e1671_d_n1, eq51_e1671_d_n2, eq51_e1671_d_n3, eq51_e1671_d_n4, eq51_e1671_d_n5, eq51_e1671_d_n6, eq51_e1671_d_n7, eq51_e1671_d_n8, eq51_e1671_d_n9, eq51_e1671_d_n10, eq51_e1671_d_n11, eq51_e1671_d_n12, eq51_e1671_d_n13, eq51_e1671_d_n14, eq51_e1671_d_n15, eq51_e1671_d_n16, eq51_e1671_d_b0, eq51_e1671_d_b1, eq51_e1671_d_b2, eq51_e1671_d_b3, eq51_e1671_d_b4, eq51_e1671_d_b5, eq51_e1671_d_b6, eq51_e1671_d_b7, eq51_e1671_d_b8, eq51_e1671_d_b9, eq51_e1671_d_b10, eq51_e1671_d_b11, eq51_e1671_d_b12, eq51_e1671_d_b13,) = {
    if (!s.b[1616]) {
        let eq51_e1667: f64 = (p.p28 * (nv1 - nv10));
        let eq51_e1667_d_n1: f64 = p.p28;
        let eq51_e1667_d_n10: f64 = (-p.p28);
        let eq51_e1669: f64 = (eq51_e1667 * s.v[1617]);
        let eq51_e1669_d_n0: f64 = (eq51_e1667 * s.dn[1617][0]);
        let eq51_e1669_d_n1: f64 = ((eq51_e1667_d_n1 * s.v[1617]) + (eq51_e1667 * s.dn[1617][1]));
        let eq51_e1669_d_n2: f64 = (eq51_e1667 * s.dn[1617][2]);
        let eq51_e1669_d_n3: f64 = (eq51_e1667 * s.dn[1617][3]);
        let eq51_e1669_d_n4: f64 = (eq51_e1667 * s.dn[1617][4]);
        let eq51_e1669_d_n5: f64 = (eq51_e1667 * s.dn[1617][5]);
        let eq51_e1669_d_n6: f64 = (eq51_e1667 * s.dn[1617][6]);
        let eq51_e1669_d_n7: f64 = (eq51_e1667 * s.dn[1617][7]);
        let eq51_e1669_d_n8: f64 = (eq51_e1667 * s.dn[1617][8]);
        let eq51_e1669_d_n9: f64 = (eq51_e1667 * s.dn[1617][9]);
        let eq51_e1669_d_n10: f64 = ((eq51_e1667_d_n10 * s.v[1617]) + (eq51_e1667 * s.dn[1617][10]));
        let eq51_e1669_d_n11: f64 = (eq51_e1667 * s.dn[1617][11]);
        let eq51_e1669_d_n12: f64 = (eq51_e1667 * s.dn[1617][12]);
        let eq51_e1669_d_n13: f64 = (eq51_e1667 * s.dn[1617][13]);
        let eq51_e1669_d_n14: f64 = (eq51_e1667 * s.dn[1617][14]);
        let eq51_e1669_d_n15: f64 = (eq51_e1667 * s.dn[1617][15]);
        let eq51_e1669_d_n16: f64 = (eq51_e1667 * s.dn[1617][16]);
        let eq51_e1669_d_b0: f64 = (eq51_e1667 * s.db[1617][0]);
        let eq51_e1669_d_b1: f64 = (eq51_e1667 * s.db[1617][1]);
        let eq51_e1669_d_b2: f64 = (eq51_e1667 * s.db[1617][2]);
        let eq51_e1669_d_b3: f64 = (eq51_e1667 * s.db[1617][3]);
        let eq51_e1669_d_b4: f64 = (eq51_e1667 * s.db[1617][4]);
        let eq51_e1669_d_b5: f64 = (eq51_e1667 * s.db[1617][5]);
        let eq51_e1669_d_b6: f64 = (eq51_e1667 * s.db[1617][6]);
        let eq51_e1669_d_b7: f64 = (eq51_e1667 * s.db[1617][7]);
        let eq51_e1669_d_b8: f64 = (eq51_e1667 * s.db[1617][8]);
        let eq51_e1669_d_b9: f64 = (eq51_e1667 * s.db[1617][9]);
        let eq51_e1669_d_b10: f64 = (eq51_e1667 * s.db[1617][10]);
        let eq51_e1669_d_b11: f64 = (eq51_e1667 * s.db[1617][11]);
        let eq51_e1669_d_b12: f64 = (eq51_e1667 * s.db[1617][12]);
        let eq51_e1669_d_b13: f64 = (eq51_e1667 * s.db[1617][13]);
        (eq51_e1669, eq51_e1669_d_n0, eq51_e1669_d_n1, eq51_e1669_d_n2, eq51_e1669_d_n3, eq51_e1669_d_n4, eq51_e1669_d_n5, eq51_e1669_d_n6, eq51_e1669_d_n7, eq51_e1669_d_n8, eq51_e1669_d_n9, eq51_e1669_d_n10, eq51_e1669_d_n11, eq51_e1669_d_n12, eq51_e1669_d_n13, eq51_e1669_d_n14, eq51_e1669_d_n15, eq51_e1669_d_n16, eq51_e1669_d_b0, eq51_e1669_d_b1, eq51_e1669_d_b2, eq51_e1669_d_b3, eq51_e1669_d_b4, eq51_e1669_d_b5, eq51_e1669_d_b6, eq51_e1669_d_b7, eq51_e1669_d_b8, eq51_e1669_d_b9, eq51_e1669_d_b10, eq51_e1669_d_b11, eq51_e1669_d_b12, eq51_e1669_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e1671;
        let eq51_node_derivatives: [f64; 17] = [eq51_e1671_d_n0, eq51_e1671_d_n1, eq51_e1671_d_n2, eq51_e1671_d_n3, eq51_e1671_d_n4, eq51_e1671_d_n5, eq51_e1671_d_n6, eq51_e1671_d_n7, eq51_e1671_d_n8, eq51_e1671_d_n9, eq51_e1671_d_n10, eq51_e1671_d_n11, eq51_e1671_d_n12, eq51_e1671_d_n13, eq51_e1671_d_n14, eq51_e1671_d_n15, eq51_e1671_d_n16];
        let eq51_branch_derivatives: [f64; 14] = [eq51_e1671_d_b0, eq51_e1671_d_b1, eq51_e1671_d_b2, eq51_e1671_d_b3, eq51_e1671_d_b4, eq51_e1671_d_b5, eq51_e1671_d_b6, eq51_e1671_d_b7, eq51_e1671_d_b8, eq51_e1671_d_b9, eq51_e1671_d_b10, eq51_e1671_d_b11, eq51_e1671_d_b12, eq51_e1671_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[10]),
            multiplicity * (eq51_value),
            nodes,
            &eq51_node_derivatives,
            branches,
            &eq51_branch_derivatives,
            multiplicity,
        );
        let (eq52_e1682,) = {
    if (!s.b[1616]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq52_value: f64 = eq52_e1682;
        stamper.stamp_current_const(
            Some(nodes[1]),
            Some(nodes[10]),
            multiplicity * (eq52_value),
        );
        let (eq53_e1690, eq53_e1690_d_n0, eq53_e1690_d_n1, eq53_e1690_d_n2, eq53_e1690_d_n3, eq53_e1690_d_n4, eq53_e1690_d_n5, eq53_e1690_d_n6, eq53_e1690_d_n7, eq53_e1690_d_n8, eq53_e1690_d_n9, eq53_e1690_d_n10, eq53_e1690_d_n11, eq53_e1690_d_n12, eq53_e1690_d_n13, eq53_e1690_d_n14, eq53_e1690_d_n15, eq53_e1690_d_n16, eq53_e1690_d_b0, eq53_e1690_d_b1, eq53_e1690_d_b2, eq53_e1690_d_b3, eq53_e1690_d_b4, eq53_e1690_d_b5, eq53_e1690_d_b6, eq53_e1690_d_b7, eq53_e1690_d_b8, eq53_e1690_d_b9, eq53_e1690_d_b10, eq53_e1690_d_b11, eq53_e1690_d_b12, eq53_e1690_d_b13,) = {
    if s.b[1620] {
        let eq53_e1686: f64 = ((nv10 - nv9) * p.p28);
        let eq53_e1686_d_n9: f64 = (-p.p28);
        let eq53_e1686_d_n10: f64 = p.p28;
        let eq53_e1688: f64 = (eq53_e1686 * s.v[254]);
        let eq53_e1688_d_n0: f64 = (eq53_e1686 * s.dn[254][0]);
        let eq53_e1688_d_n1: f64 = (eq53_e1686 * s.dn[254][1]);
        let eq53_e1688_d_n2: f64 = (eq53_e1686 * s.dn[254][2]);
        let eq53_e1688_d_n3: f64 = (eq53_e1686 * s.dn[254][3]);
        let eq53_e1688_d_n4: f64 = (eq53_e1686 * s.dn[254][4]);
        let eq53_e1688_d_n5: f64 = (eq53_e1686 * s.dn[254][5]);
        let eq53_e1688_d_n6: f64 = (eq53_e1686 * s.dn[254][6]);
        let eq53_e1688_d_n7: f64 = (eq53_e1686 * s.dn[254][7]);
        let eq53_e1688_d_n8: f64 = (eq53_e1686 * s.dn[254][8]);
        let eq53_e1688_d_n9: f64 = ((eq53_e1686_d_n9 * s.v[254]) + (eq53_e1686 * s.dn[254][9]));
        let eq53_e1688_d_n10: f64 = ((eq53_e1686_d_n10 * s.v[254]) + (eq53_e1686 * s.dn[254][10]));
        let eq53_e1688_d_n11: f64 = (eq53_e1686 * s.dn[254][11]);
        let eq53_e1688_d_n12: f64 = (eq53_e1686 * s.dn[254][12]);
        let eq53_e1688_d_n13: f64 = (eq53_e1686 * s.dn[254][13]);
        let eq53_e1688_d_n14: f64 = (eq53_e1686 * s.dn[254][14]);
        let eq53_e1688_d_n15: f64 = (eq53_e1686 * s.dn[254][15]);
        let eq53_e1688_d_n16: f64 = (eq53_e1686 * s.dn[254][16]);
        let eq53_e1688_d_b0: f64 = (eq53_e1686 * s.db[254][0]);
        let eq53_e1688_d_b1: f64 = (eq53_e1686 * s.db[254][1]);
        let eq53_e1688_d_b2: f64 = (eq53_e1686 * s.db[254][2]);
        let eq53_e1688_d_b3: f64 = (eq53_e1686 * s.db[254][3]);
        let eq53_e1688_d_b4: f64 = (eq53_e1686 * s.db[254][4]);
        let eq53_e1688_d_b5: f64 = (eq53_e1686 * s.db[254][5]);
        let eq53_e1688_d_b6: f64 = (eq53_e1686 * s.db[254][6]);
        let eq53_e1688_d_b7: f64 = (eq53_e1686 * s.db[254][7]);
        let eq53_e1688_d_b8: f64 = (eq53_e1686 * s.db[254][8]);
        let eq53_e1688_d_b9: f64 = (eq53_e1686 * s.db[254][9]);
        let eq53_e1688_d_b10: f64 = (eq53_e1686 * s.db[254][10]);
        let eq53_e1688_d_b11: f64 = (eq53_e1686 * s.db[254][11]);
        let eq53_e1688_d_b12: f64 = (eq53_e1686 * s.db[254][12]);
        let eq53_e1688_d_b13: f64 = (eq53_e1686 * s.db[254][13]);
        (eq53_e1688, eq53_e1688_d_n0, eq53_e1688_d_n1, eq53_e1688_d_n2, eq53_e1688_d_n3, eq53_e1688_d_n4, eq53_e1688_d_n5, eq53_e1688_d_n6, eq53_e1688_d_n7, eq53_e1688_d_n8, eq53_e1688_d_n9, eq53_e1688_d_n10, eq53_e1688_d_n11, eq53_e1688_d_n12, eq53_e1688_d_n13, eq53_e1688_d_n14, eq53_e1688_d_n15, eq53_e1688_d_n16, eq53_e1688_d_b0, eq53_e1688_d_b1, eq53_e1688_d_b2, eq53_e1688_d_b3, eq53_e1688_d_b4, eq53_e1688_d_b5, eq53_e1688_d_b6, eq53_e1688_d_b7, eq53_e1688_d_b8, eq53_e1688_d_b9, eq53_e1688_d_b10, eq53_e1688_d_b11, eq53_e1688_d_b12, eq53_e1688_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e1690;
        let eq53_node_derivatives: [f64; 17] = [eq53_e1690_d_n0, eq53_e1690_d_n1, eq53_e1690_d_n2, eq53_e1690_d_n3, eq53_e1690_d_n4, eq53_e1690_d_n5, eq53_e1690_d_n6, eq53_e1690_d_n7, eq53_e1690_d_n8, eq53_e1690_d_n9, eq53_e1690_d_n10, eq53_e1690_d_n11, eq53_e1690_d_n12, eq53_e1690_d_n13, eq53_e1690_d_n14, eq53_e1690_d_n15, eq53_e1690_d_n16];
        let eq53_branch_derivatives: [f64; 14] = [eq53_e1690_d_b0, eq53_e1690_d_b1, eq53_e1690_d_b2, eq53_e1690_d_b3, eq53_e1690_d_b4, eq53_e1690_d_b5, eq53_e1690_d_b6, eq53_e1690_d_b7, eq53_e1690_d_b8, eq53_e1690_d_b9, eq53_e1690_d_b10, eq53_e1690_d_b11, eq53_e1690_d_b12, eq53_e1690_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[9]),
            multiplicity * (eq53_value),
            nodes,
            &eq53_node_derivatives,
            branches,
            &eq53_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_6(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq54_e1695,) = {
    if (!s.b[1620]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq54_value: f64 = eq54_e1695;
        stamper.stamp_potential_const(
            branches[8],
            eq54_value,
        );
        let (eq55_e1708, eq55_e1708_d_n0, eq55_e1708_d_n1, eq55_e1708_d_n2, eq55_e1708_d_n3, eq55_e1708_d_n4, eq55_e1708_d_n5, eq55_e1708_d_n6, eq55_e1708_d_n7, eq55_e1708_d_n8, eq55_e1708_d_n9, eq55_e1708_d_n10, eq55_e1708_d_n11, eq55_e1708_d_n12, eq55_e1708_d_n13, eq55_e1708_d_n14, eq55_e1708_d_n15, eq55_e1708_d_n16, eq55_e1708_d_b0, eq55_e1708_d_b1, eq55_e1708_d_b2, eq55_e1708_d_b3, eq55_e1708_d_b4, eq55_e1708_d_b5, eq55_e1708_d_b6, eq55_e1708_d_b7, eq55_e1708_d_b8, eq55_e1708_d_b9, eq55_e1708_d_b10, eq55_e1708_d_b11, eq55_e1708_d_b12, eq55_e1708_d_b13,) = {
    if s.b[1621] {
        let eq55_e1699: f64 = (s.v[390] * s.v[747]);
        let eq55_e1699_d_n0: f64 = ((s.dn[390][0] * s.v[747]) + (s.v[390] * s.dn[747][0]));
        let eq55_e1699_d_n1: f64 = ((s.dn[390][1] * s.v[747]) + (s.v[390] * s.dn[747][1]));
        let eq55_e1699_d_n2: f64 = ((s.dn[390][2] * s.v[747]) + (s.v[390] * s.dn[747][2]));
        let eq55_e1699_d_n3: f64 = ((s.dn[390][3] * s.v[747]) + (s.v[390] * s.dn[747][3]));
        let eq55_e1699_d_n4: f64 = ((s.dn[390][4] * s.v[747]) + (s.v[390] * s.dn[747][4]));
        let eq55_e1699_d_n5: f64 = ((s.dn[390][5] * s.v[747]) + (s.v[390] * s.dn[747][5]));
        let eq55_e1699_d_n6: f64 = ((s.dn[390][6] * s.v[747]) + (s.v[390] * s.dn[747][6]));
        let eq55_e1699_d_n7: f64 = ((s.dn[390][7] * s.v[747]) + (s.v[390] * s.dn[747][7]));
        let eq55_e1699_d_n8: f64 = ((s.dn[390][8] * s.v[747]) + (s.v[390] * s.dn[747][8]));
        let eq55_e1699_d_n9: f64 = ((s.dn[390][9] * s.v[747]) + (s.v[390] * s.dn[747][9]));
        let eq55_e1699_d_n10: f64 = ((s.dn[390][10] * s.v[747]) + (s.v[390] * s.dn[747][10]));
        let eq55_e1699_d_n11: f64 = ((s.dn[390][11] * s.v[747]) + (s.v[390] * s.dn[747][11]));
        let eq55_e1699_d_n12: f64 = ((s.dn[390][12] * s.v[747]) + (s.v[390] * s.dn[747][12]));
        let eq55_e1699_d_n13: f64 = ((s.dn[390][13] * s.v[747]) + (s.v[390] * s.dn[747][13]));
        let eq55_e1699_d_n14: f64 = ((s.dn[390][14] * s.v[747]) + (s.v[390] * s.dn[747][14]));
        let eq55_e1699_d_n15: f64 = ((s.dn[390][15] * s.v[747]) + (s.v[390] * s.dn[747][15]));
        let eq55_e1699_d_n16: f64 = ((s.dn[390][16] * s.v[747]) + (s.v[390] * s.dn[747][16]));
        let eq55_e1699_d_b0: f64 = ((s.db[390][0] * s.v[747]) + (s.v[390] * s.db[747][0]));
        let eq55_e1699_d_b1: f64 = ((s.db[390][1] * s.v[747]) + (s.v[390] * s.db[747][1]));
        let eq55_e1699_d_b2: f64 = ((s.db[390][2] * s.v[747]) + (s.v[390] * s.db[747][2]));
        let eq55_e1699_d_b3: f64 = ((s.db[390][3] * s.v[747]) + (s.v[390] * s.db[747][3]));
        let eq55_e1699_d_b4: f64 = ((s.db[390][4] * s.v[747]) + (s.v[390] * s.db[747][4]));
        let eq55_e1699_d_b5: f64 = ((s.db[390][5] * s.v[747]) + (s.v[390] * s.db[747][5]));
        let eq55_e1699_d_b6: f64 = ((s.db[390][6] * s.v[747]) + (s.v[390] * s.db[747][6]));
        let eq55_e1699_d_b7: f64 = ((s.db[390][7] * s.v[747]) + (s.v[390] * s.db[747][7]));
        let eq55_e1699_d_b8: f64 = ((s.db[390][8] * s.v[747]) + (s.v[390] * s.db[747][8]));
        let eq55_e1699_d_b9: f64 = ((s.db[390][9] * s.v[747]) + (s.v[390] * s.db[747][9]));
        let eq55_e1699_d_b10: f64 = ((s.db[390][10] * s.v[747]) + (s.v[390] * s.db[747][10]));
        let eq55_e1699_d_b11: f64 = ((s.db[390][11] * s.v[747]) + (s.v[390] * s.db[747][11]));
        let eq55_e1699_d_b12: f64 = ((s.db[390][12] * s.v[747]) + (s.v[390] * s.db[747][12]));
        let eq55_e1699_d_b13: f64 = ((s.db[390][13] * s.v[747]) + (s.v[390] * s.db[747][13]));
        let eq55_e1702: f64 = (s.v[390] * s.v[748]);
        let eq55_e1702_d_n0: f64 = ((s.dn[390][0] * s.v[748]) + (s.v[390] * s.dn[748][0]));
        let eq55_e1702_d_n1: f64 = ((s.dn[390][1] * s.v[748]) + (s.v[390] * s.dn[748][1]));
        let eq55_e1702_d_n2: f64 = ((s.dn[390][2] * s.v[748]) + (s.v[390] * s.dn[748][2]));
        let eq55_e1702_d_n3: f64 = ((s.dn[390][3] * s.v[748]) + (s.v[390] * s.dn[748][3]));
        let eq55_e1702_d_n4: f64 = ((s.dn[390][4] * s.v[748]) + (s.v[390] * s.dn[748][4]));
        let eq55_e1702_d_n5: f64 = ((s.dn[390][5] * s.v[748]) + (s.v[390] * s.dn[748][5]));
        let eq55_e1702_d_n6: f64 = ((s.dn[390][6] * s.v[748]) + (s.v[390] * s.dn[748][6]));
        let eq55_e1702_d_n7: f64 = ((s.dn[390][7] * s.v[748]) + (s.v[390] * s.dn[748][7]));
        let eq55_e1702_d_n8: f64 = ((s.dn[390][8] * s.v[748]) + (s.v[390] * s.dn[748][8]));
        let eq55_e1702_d_n9: f64 = ((s.dn[390][9] * s.v[748]) + (s.v[390] * s.dn[748][9]));
        let eq55_e1702_d_n10: f64 = ((s.dn[390][10] * s.v[748]) + (s.v[390] * s.dn[748][10]));
        let eq55_e1702_d_n11: f64 = ((s.dn[390][11] * s.v[748]) + (s.v[390] * s.dn[748][11]));
        let eq55_e1702_d_n12: f64 = ((s.dn[390][12] * s.v[748]) + (s.v[390] * s.dn[748][12]));
        let eq55_e1702_d_n13: f64 = ((s.dn[390][13] * s.v[748]) + (s.v[390] * s.dn[748][13]));
        let eq55_e1702_d_n14: f64 = ((s.dn[390][14] * s.v[748]) + (s.v[390] * s.dn[748][14]));
        let eq55_e1702_d_n15: f64 = ((s.dn[390][15] * s.v[748]) + (s.v[390] * s.dn[748][15]));
        let eq55_e1702_d_n16: f64 = ((s.dn[390][16] * s.v[748]) + (s.v[390] * s.dn[748][16]));
        let eq55_e1702_d_b0: f64 = ((s.db[390][0] * s.v[748]) + (s.v[390] * s.db[748][0]));
        let eq55_e1702_d_b1: f64 = ((s.db[390][1] * s.v[748]) + (s.v[390] * s.db[748][1]));
        let eq55_e1702_d_b2: f64 = ((s.db[390][2] * s.v[748]) + (s.v[390] * s.db[748][2]));
        let eq55_e1702_d_b3: f64 = ((s.db[390][3] * s.v[748]) + (s.v[390] * s.db[748][3]));
        let eq55_e1702_d_b4: f64 = ((s.db[390][4] * s.v[748]) + (s.v[390] * s.db[748][4]));
        let eq55_e1702_d_b5: f64 = ((s.db[390][5] * s.v[748]) + (s.v[390] * s.db[748][5]));
        let eq55_e1702_d_b6: f64 = ((s.db[390][6] * s.v[748]) + (s.v[390] * s.db[748][6]));
        let eq55_e1702_d_b7: f64 = ((s.db[390][7] * s.v[748]) + (s.v[390] * s.db[748][7]));
        let eq55_e1702_d_b8: f64 = ((s.db[390][8] * s.v[748]) + (s.v[390] * s.db[748][8]));
        let eq55_e1702_d_b9: f64 = ((s.db[390][9] * s.v[748]) + (s.v[390] * s.db[748][9]));
        let eq55_e1702_d_b10: f64 = ((s.db[390][10] * s.v[748]) + (s.v[390] * s.db[748][10]));
        let eq55_e1702_d_b11: f64 = ((s.db[390][11] * s.v[748]) + (s.v[390] * s.db[748][11]));
        let eq55_e1702_d_b12: f64 = ((s.db[390][12] * s.v[748]) + (s.v[390] * s.db[748][12]));
        let eq55_e1702_d_b13: f64 = ((s.db[390][13] * s.v[748]) + (s.v[390] * s.db[748][13]));
        let eq55_e1703: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, eq55_e1702);
        let eq55_e1703_d_n0: f64 = (eq55_e1702_d_n0 * ddt_scale);
        let eq55_e1703_d_n1: f64 = (eq55_e1702_d_n1 * ddt_scale);
        let eq55_e1703_d_n2: f64 = (eq55_e1702_d_n2 * ddt_scale);
        let eq55_e1703_d_n3: f64 = (eq55_e1702_d_n3 * ddt_scale);
        let eq55_e1703_d_n4: f64 = (eq55_e1702_d_n4 * ddt_scale);
        let eq55_e1703_d_n5: f64 = (eq55_e1702_d_n5 * ddt_scale);
        let eq55_e1703_d_n6: f64 = (eq55_e1702_d_n6 * ddt_scale);
        let eq55_e1703_d_n7: f64 = (eq55_e1702_d_n7 * ddt_scale);
        let eq55_e1703_d_n8: f64 = (eq55_e1702_d_n8 * ddt_scale);
        let eq55_e1703_d_n9: f64 = (eq55_e1702_d_n9 * ddt_scale);
        let eq55_e1703_d_n10: f64 = (eq55_e1702_d_n10 * ddt_scale);
        let eq55_e1703_d_n11: f64 = (eq55_e1702_d_n11 * ddt_scale);
        let eq55_e1703_d_n12: f64 = (eq55_e1702_d_n12 * ddt_scale);
        let eq55_e1703_d_n13: f64 = (eq55_e1702_d_n13 * ddt_scale);
        let eq55_e1703_d_n14: f64 = (eq55_e1702_d_n14 * ddt_scale);
        let eq55_e1703_d_n15: f64 = (eq55_e1702_d_n15 * ddt_scale);
        let eq55_e1703_d_n16: f64 = (eq55_e1702_d_n16 * ddt_scale);
        let eq55_e1703_d_b0: f64 = (eq55_e1702_d_b0 * ddt_scale);
        let eq55_e1703_d_b1: f64 = (eq55_e1702_d_b1 * ddt_scale);
        let eq55_e1703_d_b2: f64 = (eq55_e1702_d_b2 * ddt_scale);
        let eq55_e1703_d_b3: f64 = (eq55_e1702_d_b3 * ddt_scale);
        let eq55_e1703_d_b4: f64 = (eq55_e1702_d_b4 * ddt_scale);
        let eq55_e1703_d_b5: f64 = (eq55_e1702_d_b5 * ddt_scale);
        let eq55_e1703_d_b6: f64 = (eq55_e1702_d_b6 * ddt_scale);
        let eq55_e1703_d_b7: f64 = (eq55_e1702_d_b7 * ddt_scale);
        let eq55_e1703_d_b8: f64 = (eq55_e1702_d_b8 * ddt_scale);
        let eq55_e1703_d_b9: f64 = (eq55_e1702_d_b9 * ddt_scale);
        let eq55_e1703_d_b10: f64 = (eq55_e1702_d_b10 * ddt_scale);
        let eq55_e1703_d_b11: f64 = (eq55_e1702_d_b11 * ddt_scale);
        let eq55_e1703_d_b12: f64 = (eq55_e1702_d_b12 * ddt_scale);
        let eq55_e1703_d_b13: f64 = (eq55_e1702_d_b13 * ddt_scale);
        let eq55_e1704: f64 = (eq55_e1699 + eq55_e1703);
        let eq55_e1704_d_n0: f64 = (eq55_e1699_d_n0 + eq55_e1703_d_n0);
        let eq55_e1704_d_n1: f64 = (eq55_e1699_d_n1 + eq55_e1703_d_n1);
        let eq55_e1704_d_n2: f64 = (eq55_e1699_d_n2 + eq55_e1703_d_n2);
        let eq55_e1704_d_n3: f64 = (eq55_e1699_d_n3 + eq55_e1703_d_n3);
        let eq55_e1704_d_n4: f64 = (eq55_e1699_d_n4 + eq55_e1703_d_n4);
        let eq55_e1704_d_n5: f64 = (eq55_e1699_d_n5 + eq55_e1703_d_n5);
        let eq55_e1704_d_n6: f64 = (eq55_e1699_d_n6 + eq55_e1703_d_n6);
        let eq55_e1704_d_n7: f64 = (eq55_e1699_d_n7 + eq55_e1703_d_n7);
        let eq55_e1704_d_n8: f64 = (eq55_e1699_d_n8 + eq55_e1703_d_n8);
        let eq55_e1704_d_n9: f64 = (eq55_e1699_d_n9 + eq55_e1703_d_n9);
        let eq55_e1704_d_n10: f64 = (eq55_e1699_d_n10 + eq55_e1703_d_n10);
        let eq55_e1704_d_n11: f64 = (eq55_e1699_d_n11 + eq55_e1703_d_n11);
        let eq55_e1704_d_n12: f64 = (eq55_e1699_d_n12 + eq55_e1703_d_n12);
        let eq55_e1704_d_n13: f64 = (eq55_e1699_d_n13 + eq55_e1703_d_n13);
        let eq55_e1704_d_n14: f64 = (eq55_e1699_d_n14 + eq55_e1703_d_n14);
        let eq55_e1704_d_n15: f64 = (eq55_e1699_d_n15 + eq55_e1703_d_n15);
        let eq55_e1704_d_n16: f64 = (eq55_e1699_d_n16 + eq55_e1703_d_n16);
        let eq55_e1704_d_b0: f64 = (eq55_e1699_d_b0 + eq55_e1703_d_b0);
        let eq55_e1704_d_b1: f64 = (eq55_e1699_d_b1 + eq55_e1703_d_b1);
        let eq55_e1704_d_b2: f64 = (eq55_e1699_d_b2 + eq55_e1703_d_b2);
        let eq55_e1704_d_b3: f64 = (eq55_e1699_d_b3 + eq55_e1703_d_b3);
        let eq55_e1704_d_b4: f64 = (eq55_e1699_d_b4 + eq55_e1703_d_b4);
        let eq55_e1704_d_b5: f64 = (eq55_e1699_d_b5 + eq55_e1703_d_b5);
        let eq55_e1704_d_b6: f64 = (eq55_e1699_d_b6 + eq55_e1703_d_b6);
        let eq55_e1704_d_b7: f64 = (eq55_e1699_d_b7 + eq55_e1703_d_b7);
        let eq55_e1704_d_b8: f64 = (eq55_e1699_d_b8 + eq55_e1703_d_b8);
        let eq55_e1704_d_b9: f64 = (eq55_e1699_d_b9 + eq55_e1703_d_b9);
        let eq55_e1704_d_b10: f64 = (eq55_e1699_d_b10 + eq55_e1703_d_b10);
        let eq55_e1704_d_b11: f64 = (eq55_e1699_d_b11 + eq55_e1703_d_b11);
        let eq55_e1704_d_b12: f64 = (eq55_e1699_d_b12 + eq55_e1703_d_b12);
        let eq55_e1704_d_b13: f64 = (eq55_e1699_d_b13 + eq55_e1703_d_b13);
        let eq55_e1706: f64 = (eq55_e1704 - s.v[749]);
        let eq55_e1706_d_n0: f64 = (eq55_e1704_d_n0 - s.dn[749][0]);
        let eq55_e1706_d_n1: f64 = (eq55_e1704_d_n1 - s.dn[749][1]);
        let eq55_e1706_d_n2: f64 = (eq55_e1704_d_n2 - s.dn[749][2]);
        let eq55_e1706_d_n3: f64 = (eq55_e1704_d_n3 - s.dn[749][3]);
        let eq55_e1706_d_n4: f64 = (eq55_e1704_d_n4 - s.dn[749][4]);
        let eq55_e1706_d_n5: f64 = (eq55_e1704_d_n5 - s.dn[749][5]);
        let eq55_e1706_d_n6: f64 = (eq55_e1704_d_n6 - s.dn[749][6]);
        let eq55_e1706_d_n7: f64 = (eq55_e1704_d_n7 - s.dn[749][7]);
        let eq55_e1706_d_n8: f64 = (eq55_e1704_d_n8 - s.dn[749][8]);
        let eq55_e1706_d_n9: f64 = (eq55_e1704_d_n9 - s.dn[749][9]);
        let eq55_e1706_d_n10: f64 = (eq55_e1704_d_n10 - s.dn[749][10]);
        let eq55_e1706_d_n11: f64 = (eq55_e1704_d_n11 - s.dn[749][11]);
        let eq55_e1706_d_n12: f64 = (eq55_e1704_d_n12 - s.dn[749][12]);
        let eq55_e1706_d_n13: f64 = (eq55_e1704_d_n13 - s.dn[749][13]);
        let eq55_e1706_d_n14: f64 = (eq55_e1704_d_n14 - s.dn[749][14]);
        let eq55_e1706_d_n15: f64 = (eq55_e1704_d_n15 - s.dn[749][15]);
        let eq55_e1706_d_n16: f64 = (eq55_e1704_d_n16 - s.dn[749][16]);
        let eq55_e1706_d_b0: f64 = (eq55_e1704_d_b0 - s.db[749][0]);
        let eq55_e1706_d_b1: f64 = (eq55_e1704_d_b1 - s.db[749][1]);
        let eq55_e1706_d_b2: f64 = (eq55_e1704_d_b2 - s.db[749][2]);
        let eq55_e1706_d_b3: f64 = (eq55_e1704_d_b3 - s.db[749][3]);
        let eq55_e1706_d_b4: f64 = (eq55_e1704_d_b4 - s.db[749][4]);
        let eq55_e1706_d_b5: f64 = (eq55_e1704_d_b5 - s.db[749][5]);
        let eq55_e1706_d_b6: f64 = (eq55_e1704_d_b6 - s.db[749][6]);
        let eq55_e1706_d_b7: f64 = (eq55_e1704_d_b7 - s.db[749][7]);
        let eq55_e1706_d_b8: f64 = (eq55_e1704_d_b8 - s.db[749][8]);
        let eq55_e1706_d_b9: f64 = (eq55_e1704_d_b9 - s.db[749][9]);
        let eq55_e1706_d_b10: f64 = (eq55_e1704_d_b10 - s.db[749][10]);
        let eq55_e1706_d_b11: f64 = (eq55_e1704_d_b11 - s.db[749][11]);
        let eq55_e1706_d_b12: f64 = (eq55_e1704_d_b12 - s.db[749][12]);
        let eq55_e1706_d_b13: f64 = (eq55_e1704_d_b13 - s.db[749][13]);
        (eq55_e1706, eq55_e1706_d_n0, eq55_e1706_d_n1, eq55_e1706_d_n2, eq55_e1706_d_n3, eq55_e1706_d_n4, eq55_e1706_d_n5, eq55_e1706_d_n6, eq55_e1706_d_n7, eq55_e1706_d_n8, eq55_e1706_d_n9, eq55_e1706_d_n10, eq55_e1706_d_n11, eq55_e1706_d_n12, eq55_e1706_d_n13, eq55_e1706_d_n14, eq55_e1706_d_n15, eq55_e1706_d_n16, eq55_e1706_d_b0, eq55_e1706_d_b1, eq55_e1706_d_b2, eq55_e1706_d_b3, eq55_e1706_d_b4, eq55_e1706_d_b5, eq55_e1706_d_b6, eq55_e1706_d_b7, eq55_e1706_d_b8, eq55_e1706_d_b9, eq55_e1706_d_b10, eq55_e1706_d_b11, eq55_e1706_d_b12, eq55_e1706_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e1708;
        let eq55_node_derivatives: [f64; 17] = [eq55_e1708_d_n0, eq55_e1708_d_n1, eq55_e1708_d_n2, eq55_e1708_d_n3, eq55_e1708_d_n4, eq55_e1708_d_n5, eq55_e1708_d_n6, eq55_e1708_d_n7, eq55_e1708_d_n8, eq55_e1708_d_n9, eq55_e1708_d_n10, eq55_e1708_d_n11, eq55_e1708_d_n12, eq55_e1708_d_n13, eq55_e1708_d_n14, eq55_e1708_d_n15, eq55_e1708_d_n16];
        let eq55_branch_derivatives: [f64; 14] = [eq55_e1708_d_b0, eq55_e1708_d_b1, eq55_e1708_d_b2, eq55_e1708_d_b3, eq55_e1708_d_b4, eq55_e1708_d_b5, eq55_e1708_d_b6, eq55_e1708_d_b7, eq55_e1708_d_b8, eq55_e1708_d_b9, eq55_e1708_d_b10, eq55_e1708_d_b11, eq55_e1708_d_b12, eq55_e1708_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            multiplicity * (eq55_value),
            nodes,
            &eq55_node_derivatives,
            branches,
            &eq55_branch_derivatives,
            multiplicity,
        );
        let (eq56_e1713,) = {
    if (!s.b[1621]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq56_value: f64 = eq56_e1713;
        stamper.stamp_potential_const(
            branches[9],
            eq56_value,
        );
        let (eq57_e1721, eq57_e1721_d_n0, eq57_e1721_d_n1, eq57_e1721_d_n2, eq57_e1721_d_n3, eq57_e1721_d_n4, eq57_e1721_d_n5, eq57_e1721_d_n6, eq57_e1721_d_n7, eq57_e1721_d_n8, eq57_e1721_d_n9, eq57_e1721_d_n10, eq57_e1721_d_n11, eq57_e1721_d_n12, eq57_e1721_d_n13, eq57_e1721_d_n14, eq57_e1721_d_n15, eq57_e1721_d_n16, eq57_e1721_d_b0, eq57_e1721_d_b1, eq57_e1721_d_b2, eq57_e1721_d_b3, eq57_e1721_d_b4, eq57_e1721_d_b5, eq57_e1721_d_b6, eq57_e1721_d_b7, eq57_e1721_d_b8, eq57_e1721_d_b9, eq57_e1721_d_b10, eq57_e1721_d_b11, eq57_e1721_d_b12, eq57_e1721_d_b13,) = {
    if s.b[1626] {
        let eq57_e1717: f64 = (p.p28 * (nv11 - nv12));
        let eq57_e1717_d_n11: f64 = p.p28;
        let eq57_e1717_d_n12: f64 = (-p.p28);
        let eq57_e1719: f64 = (eq57_e1717 * s.v[274]);
        let eq57_e1719_d_n0: f64 = (eq57_e1717 * s.dn[274][0]);
        let eq57_e1719_d_n1: f64 = (eq57_e1717 * s.dn[274][1]);
        let eq57_e1719_d_n2: f64 = (eq57_e1717 * s.dn[274][2]);
        let eq57_e1719_d_n3: f64 = (eq57_e1717 * s.dn[274][3]);
        let eq57_e1719_d_n4: f64 = (eq57_e1717 * s.dn[274][4]);
        let eq57_e1719_d_n5: f64 = (eq57_e1717 * s.dn[274][5]);
        let eq57_e1719_d_n6: f64 = (eq57_e1717 * s.dn[274][6]);
        let eq57_e1719_d_n7: f64 = (eq57_e1717 * s.dn[274][7]);
        let eq57_e1719_d_n8: f64 = (eq57_e1717 * s.dn[274][8]);
        let eq57_e1719_d_n9: f64 = (eq57_e1717 * s.dn[274][9]);
        let eq57_e1719_d_n10: f64 = (eq57_e1717 * s.dn[274][10]);
        let eq57_e1719_d_n11: f64 = ((eq57_e1717_d_n11 * s.v[274]) + (eq57_e1717 * s.dn[274][11]));
        let eq57_e1719_d_n12: f64 = ((eq57_e1717_d_n12 * s.v[274]) + (eq57_e1717 * s.dn[274][12]));
        let eq57_e1719_d_n13: f64 = (eq57_e1717 * s.dn[274][13]);
        let eq57_e1719_d_n14: f64 = (eq57_e1717 * s.dn[274][14]);
        let eq57_e1719_d_n15: f64 = (eq57_e1717 * s.dn[274][15]);
        let eq57_e1719_d_n16: f64 = (eq57_e1717 * s.dn[274][16]);
        let eq57_e1719_d_b0: f64 = (eq57_e1717 * s.db[274][0]);
        let eq57_e1719_d_b1: f64 = (eq57_e1717 * s.db[274][1]);
        let eq57_e1719_d_b2: f64 = (eq57_e1717 * s.db[274][2]);
        let eq57_e1719_d_b3: f64 = (eq57_e1717 * s.db[274][3]);
        let eq57_e1719_d_b4: f64 = (eq57_e1717 * s.db[274][4]);
        let eq57_e1719_d_b5: f64 = (eq57_e1717 * s.db[274][5]);
        let eq57_e1719_d_b6: f64 = (eq57_e1717 * s.db[274][6]);
        let eq57_e1719_d_b7: f64 = (eq57_e1717 * s.db[274][7]);
        let eq57_e1719_d_b8: f64 = (eq57_e1717 * s.db[274][8]);
        let eq57_e1719_d_b9: f64 = (eq57_e1717 * s.db[274][9]);
        let eq57_e1719_d_b10: f64 = (eq57_e1717 * s.db[274][10]);
        let eq57_e1719_d_b11: f64 = (eq57_e1717 * s.db[274][11]);
        let eq57_e1719_d_b12: f64 = (eq57_e1717 * s.db[274][12]);
        let eq57_e1719_d_b13: f64 = (eq57_e1717 * s.db[274][13]);
        (eq57_e1719, eq57_e1719_d_n0, eq57_e1719_d_n1, eq57_e1719_d_n2, eq57_e1719_d_n3, eq57_e1719_d_n4, eq57_e1719_d_n5, eq57_e1719_d_n6, eq57_e1719_d_n7, eq57_e1719_d_n8, eq57_e1719_d_n9, eq57_e1719_d_n10, eq57_e1719_d_n11, eq57_e1719_d_n12, eq57_e1719_d_n13, eq57_e1719_d_n14, eq57_e1719_d_n15, eq57_e1719_d_n16, eq57_e1719_d_b0, eq57_e1719_d_b1, eq57_e1719_d_b2, eq57_e1719_d_b3, eq57_e1719_d_b4, eq57_e1719_d_b5, eq57_e1719_d_b6, eq57_e1719_d_b7, eq57_e1719_d_b8, eq57_e1719_d_b9, eq57_e1719_d_b10, eq57_e1719_d_b11, eq57_e1719_d_b12, eq57_e1719_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e1721;
        let eq57_node_derivatives: [f64; 17] = [eq57_e1721_d_n0, eq57_e1721_d_n1, eq57_e1721_d_n2, eq57_e1721_d_n3, eq57_e1721_d_n4, eq57_e1721_d_n5, eq57_e1721_d_n6, eq57_e1721_d_n7, eq57_e1721_d_n8, eq57_e1721_d_n9, eq57_e1721_d_n10, eq57_e1721_d_n11, eq57_e1721_d_n12, eq57_e1721_d_n13, eq57_e1721_d_n14, eq57_e1721_d_n15, eq57_e1721_d_n16];
        let eq57_branch_derivatives: [f64; 14] = [eq57_e1721_d_b0, eq57_e1721_d_b1, eq57_e1721_d_b2, eq57_e1721_d_b3, eq57_e1721_d_b4, eq57_e1721_d_b5, eq57_e1721_d_b6, eq57_e1721_d_b7, eq57_e1721_d_b8, eq57_e1721_d_b9, eq57_e1721_d_b10, eq57_e1721_d_b11, eq57_e1721_d_b12, eq57_e1721_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[12]),
            multiplicity * (eq57_value),
            nodes,
            &eq57_node_derivatives,
            branches,
            &eq57_branch_derivatives,
            multiplicity,
        );
        let (eq58_e1729, eq58_e1729_d_n0, eq58_e1729_d_n1, eq58_e1729_d_n2, eq58_e1729_d_n3, eq58_e1729_d_n4, eq58_e1729_d_n5, eq58_e1729_d_n6, eq58_e1729_d_n7, eq58_e1729_d_n8, eq58_e1729_d_n9, eq58_e1729_d_n10, eq58_e1729_d_n11, eq58_e1729_d_n12, eq58_e1729_d_n13, eq58_e1729_d_n14, eq58_e1729_d_n15, eq58_e1729_d_n16, eq58_e1729_d_b0, eq58_e1729_d_b1, eq58_e1729_d_b2, eq58_e1729_d_b3, eq58_e1729_d_b4, eq58_e1729_d_b5, eq58_e1729_d_b6, eq58_e1729_d_b7, eq58_e1729_d_b8, eq58_e1729_d_b9, eq58_e1729_d_b10, eq58_e1729_d_b11, eq58_e1729_d_b12, eq58_e1729_d_b13,) = {
    if s.b[1626] {
        let eq58_e1725: f64 = (p.p28 * (nv3 - nv12));
        let eq58_e1725_d_n3: f64 = p.p28;
        let eq58_e1725_d_n12: f64 = (-p.p28);
        let eq58_e1727: f64 = (eq58_e1725 * s.v[271]);
        let eq58_e1727_d_n0: f64 = (eq58_e1725 * s.dn[271][0]);
        let eq58_e1727_d_n1: f64 = (eq58_e1725 * s.dn[271][1]);
        let eq58_e1727_d_n2: f64 = (eq58_e1725 * s.dn[271][2]);
        let eq58_e1727_d_n3: f64 = ((eq58_e1725_d_n3 * s.v[271]) + (eq58_e1725 * s.dn[271][3]));
        let eq58_e1727_d_n4: f64 = (eq58_e1725 * s.dn[271][4]);
        let eq58_e1727_d_n5: f64 = (eq58_e1725 * s.dn[271][5]);
        let eq58_e1727_d_n6: f64 = (eq58_e1725 * s.dn[271][6]);
        let eq58_e1727_d_n7: f64 = (eq58_e1725 * s.dn[271][7]);
        let eq58_e1727_d_n8: f64 = (eq58_e1725 * s.dn[271][8]);
        let eq58_e1727_d_n9: f64 = (eq58_e1725 * s.dn[271][9]);
        let eq58_e1727_d_n10: f64 = (eq58_e1725 * s.dn[271][10]);
        let eq58_e1727_d_n11: f64 = (eq58_e1725 * s.dn[271][11]);
        let eq58_e1727_d_n12: f64 = ((eq58_e1725_d_n12 * s.v[271]) + (eq58_e1725 * s.dn[271][12]));
        let eq58_e1727_d_n13: f64 = (eq58_e1725 * s.dn[271][13]);
        let eq58_e1727_d_n14: f64 = (eq58_e1725 * s.dn[271][14]);
        let eq58_e1727_d_n15: f64 = (eq58_e1725 * s.dn[271][15]);
        let eq58_e1727_d_n16: f64 = (eq58_e1725 * s.dn[271][16]);
        let eq58_e1727_d_b0: f64 = (eq58_e1725 * s.db[271][0]);
        let eq58_e1727_d_b1: f64 = (eq58_e1725 * s.db[271][1]);
        let eq58_e1727_d_b2: f64 = (eq58_e1725 * s.db[271][2]);
        let eq58_e1727_d_b3: f64 = (eq58_e1725 * s.db[271][3]);
        let eq58_e1727_d_b4: f64 = (eq58_e1725 * s.db[271][4]);
        let eq58_e1727_d_b5: f64 = (eq58_e1725 * s.db[271][5]);
        let eq58_e1727_d_b6: f64 = (eq58_e1725 * s.db[271][6]);
        let eq58_e1727_d_b7: f64 = (eq58_e1725 * s.db[271][7]);
        let eq58_e1727_d_b8: f64 = (eq58_e1725 * s.db[271][8]);
        let eq58_e1727_d_b9: f64 = (eq58_e1725 * s.db[271][9]);
        let eq58_e1727_d_b10: f64 = (eq58_e1725 * s.db[271][10]);
        let eq58_e1727_d_b11: f64 = (eq58_e1725 * s.db[271][11]);
        let eq58_e1727_d_b12: f64 = (eq58_e1725 * s.db[271][12]);
        let eq58_e1727_d_b13: f64 = (eq58_e1725 * s.db[271][13]);
        (eq58_e1727, eq58_e1727_d_n0, eq58_e1727_d_n1, eq58_e1727_d_n2, eq58_e1727_d_n3, eq58_e1727_d_n4, eq58_e1727_d_n5, eq58_e1727_d_n6, eq58_e1727_d_n7, eq58_e1727_d_n8, eq58_e1727_d_n9, eq58_e1727_d_n10, eq58_e1727_d_n11, eq58_e1727_d_n12, eq58_e1727_d_n13, eq58_e1727_d_n14, eq58_e1727_d_n15, eq58_e1727_d_n16, eq58_e1727_d_b0, eq58_e1727_d_b1, eq58_e1727_d_b2, eq58_e1727_d_b3, eq58_e1727_d_b4, eq58_e1727_d_b5, eq58_e1727_d_b6, eq58_e1727_d_b7, eq58_e1727_d_b8, eq58_e1727_d_b9, eq58_e1727_d_b10, eq58_e1727_d_b11, eq58_e1727_d_b12, eq58_e1727_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e1729;
        let eq58_node_derivatives: [f64; 17] = [eq58_e1729_d_n0, eq58_e1729_d_n1, eq58_e1729_d_n2, eq58_e1729_d_n3, eq58_e1729_d_n4, eq58_e1729_d_n5, eq58_e1729_d_n6, eq58_e1729_d_n7, eq58_e1729_d_n8, eq58_e1729_d_n9, eq58_e1729_d_n10, eq58_e1729_d_n11, eq58_e1729_d_n12, eq58_e1729_d_n13, eq58_e1729_d_n14, eq58_e1729_d_n15, eq58_e1729_d_n16];
        let eq58_branch_derivatives: [f64; 14] = [eq58_e1729_d_b0, eq58_e1729_d_b1, eq58_e1729_d_b2, eq58_e1729_d_b3, eq58_e1729_d_b4, eq58_e1729_d_b5, eq58_e1729_d_b6, eq58_e1729_d_b7, eq58_e1729_d_b8, eq58_e1729_d_b9, eq58_e1729_d_b10, eq58_e1729_d_b11, eq58_e1729_d_b12, eq58_e1729_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[12]),
            multiplicity * (eq58_value),
            nodes,
            &eq58_node_derivatives,
            branches,
            &eq58_branch_derivatives,
            multiplicity,
        );
        let (eq59_e1737, eq59_e1737_d_n0, eq59_e1737_d_n1, eq59_e1737_d_n2, eq59_e1737_d_n3, eq59_e1737_d_n4, eq59_e1737_d_n5, eq59_e1737_d_n6, eq59_e1737_d_n7, eq59_e1737_d_n8, eq59_e1737_d_n9, eq59_e1737_d_n10, eq59_e1737_d_n11, eq59_e1737_d_n12, eq59_e1737_d_n13, eq59_e1737_d_n14, eq59_e1737_d_n15, eq59_e1737_d_n16, eq59_e1737_d_b0, eq59_e1737_d_b1, eq59_e1737_d_b2, eq59_e1737_d_b3, eq59_e1737_d_b4, eq59_e1737_d_b5, eq59_e1737_d_b6, eq59_e1737_d_b7, eq59_e1737_d_b8, eq59_e1737_d_b9, eq59_e1737_d_b10, eq59_e1737_d_b11, eq59_e1737_d_b12, eq59_e1737_d_b13,) = {
    if s.b[1626] {
        let eq59_e1733: f64 = (p.p28 * (nv3 - nv11));
        let eq59_e1733_d_n3: f64 = p.p28;
        let eq59_e1733_d_n11: f64 = (-p.p28);
        let eq59_e1735: f64 = (eq59_e1733 * s.v[273]);
        let eq59_e1735_d_n0: f64 = (eq59_e1733 * s.dn[273][0]);
        let eq59_e1735_d_n1: f64 = (eq59_e1733 * s.dn[273][1]);
        let eq59_e1735_d_n2: f64 = (eq59_e1733 * s.dn[273][2]);
        let eq59_e1735_d_n3: f64 = ((eq59_e1733_d_n3 * s.v[273]) + (eq59_e1733 * s.dn[273][3]));
        let eq59_e1735_d_n4: f64 = (eq59_e1733 * s.dn[273][4]);
        let eq59_e1735_d_n5: f64 = (eq59_e1733 * s.dn[273][5]);
        let eq59_e1735_d_n6: f64 = (eq59_e1733 * s.dn[273][6]);
        let eq59_e1735_d_n7: f64 = (eq59_e1733 * s.dn[273][7]);
        let eq59_e1735_d_n8: f64 = (eq59_e1733 * s.dn[273][8]);
        let eq59_e1735_d_n9: f64 = (eq59_e1733 * s.dn[273][9]);
        let eq59_e1735_d_n10: f64 = (eq59_e1733 * s.dn[273][10]);
        let eq59_e1735_d_n11: f64 = ((eq59_e1733_d_n11 * s.v[273]) + (eq59_e1733 * s.dn[273][11]));
        let eq59_e1735_d_n12: f64 = (eq59_e1733 * s.dn[273][12]);
        let eq59_e1735_d_n13: f64 = (eq59_e1733 * s.dn[273][13]);
        let eq59_e1735_d_n14: f64 = (eq59_e1733 * s.dn[273][14]);
        let eq59_e1735_d_n15: f64 = (eq59_e1733 * s.dn[273][15]);
        let eq59_e1735_d_n16: f64 = (eq59_e1733 * s.dn[273][16]);
        let eq59_e1735_d_b0: f64 = (eq59_e1733 * s.db[273][0]);
        let eq59_e1735_d_b1: f64 = (eq59_e1733 * s.db[273][1]);
        let eq59_e1735_d_b2: f64 = (eq59_e1733 * s.db[273][2]);
        let eq59_e1735_d_b3: f64 = (eq59_e1733 * s.db[273][3]);
        let eq59_e1735_d_b4: f64 = (eq59_e1733 * s.db[273][4]);
        let eq59_e1735_d_b5: f64 = (eq59_e1733 * s.db[273][5]);
        let eq59_e1735_d_b6: f64 = (eq59_e1733 * s.db[273][6]);
        let eq59_e1735_d_b7: f64 = (eq59_e1733 * s.db[273][7]);
        let eq59_e1735_d_b8: f64 = (eq59_e1733 * s.db[273][8]);
        let eq59_e1735_d_b9: f64 = (eq59_e1733 * s.db[273][9]);
        let eq59_e1735_d_b10: f64 = (eq59_e1733 * s.db[273][10]);
        let eq59_e1735_d_b11: f64 = (eq59_e1733 * s.db[273][11]);
        let eq59_e1735_d_b12: f64 = (eq59_e1733 * s.db[273][12]);
        let eq59_e1735_d_b13: f64 = (eq59_e1733 * s.db[273][13]);
        (eq59_e1735, eq59_e1735_d_n0, eq59_e1735_d_n1, eq59_e1735_d_n2, eq59_e1735_d_n3, eq59_e1735_d_n4, eq59_e1735_d_n5, eq59_e1735_d_n6, eq59_e1735_d_n7, eq59_e1735_d_n8, eq59_e1735_d_n9, eq59_e1735_d_n10, eq59_e1735_d_n11, eq59_e1735_d_n12, eq59_e1735_d_n13, eq59_e1735_d_n14, eq59_e1735_d_n15, eq59_e1735_d_n16, eq59_e1735_d_b0, eq59_e1735_d_b1, eq59_e1735_d_b2, eq59_e1735_d_b3, eq59_e1735_d_b4, eq59_e1735_d_b5, eq59_e1735_d_b6, eq59_e1735_d_b7, eq59_e1735_d_b8, eq59_e1735_d_b9, eq59_e1735_d_b10, eq59_e1735_d_b11, eq59_e1735_d_b12, eq59_e1735_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e1737;
        let eq59_node_derivatives: [f64; 17] = [eq59_e1737_d_n0, eq59_e1737_d_n1, eq59_e1737_d_n2, eq59_e1737_d_n3, eq59_e1737_d_n4, eq59_e1737_d_n5, eq59_e1737_d_n6, eq59_e1737_d_n7, eq59_e1737_d_n8, eq59_e1737_d_n9, eq59_e1737_d_n10, eq59_e1737_d_n11, eq59_e1737_d_n12, eq59_e1737_d_n13, eq59_e1737_d_n14, eq59_e1737_d_n15, eq59_e1737_d_n16];
        let eq59_branch_derivatives: [f64; 14] = [eq59_e1737_d_b0, eq59_e1737_d_b1, eq59_e1737_d_b2, eq59_e1737_d_b3, eq59_e1737_d_b4, eq59_e1737_d_b5, eq59_e1737_d_b6, eq59_e1737_d_b7, eq59_e1737_d_b8, eq59_e1737_d_b9, eq59_e1737_d_b10, eq59_e1737_d_b11, eq59_e1737_d_b12, eq59_e1737_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[11]),
            multiplicity * (eq59_value),
            nodes,
            &eq59_node_derivatives,
            branches,
            &eq59_branch_derivatives,
            multiplicity,
        );
        let (eq60_e1745, eq60_e1745_d_n0, eq60_e1745_d_n1, eq60_e1745_d_n2, eq60_e1745_d_n3, eq60_e1745_d_n4, eq60_e1745_d_n5, eq60_e1745_d_n6, eq60_e1745_d_n7, eq60_e1745_d_n8, eq60_e1745_d_n9, eq60_e1745_d_n10, eq60_e1745_d_n11, eq60_e1745_d_n12, eq60_e1745_d_n13, eq60_e1745_d_n14, eq60_e1745_d_n15, eq60_e1745_d_n16, eq60_e1745_d_b0, eq60_e1745_d_b1, eq60_e1745_d_b2, eq60_e1745_d_b3, eq60_e1745_d_b4, eq60_e1745_d_b5, eq60_e1745_d_b6, eq60_e1745_d_b7, eq60_e1745_d_b8, eq60_e1745_d_b9, eq60_e1745_d_b10, eq60_e1745_d_b11, eq60_e1745_d_b12, eq60_e1745_d_b13,) = {
    if s.b[1626] {
        let eq60_e1741: f64 = (p.p28 * (nv3 - nv13));
        let eq60_e1741_d_n3: f64 = p.p28;
        let eq60_e1741_d_n13: f64 = (-p.p28);
        let eq60_e1743: f64 = (eq60_e1741 * s.v[272]);
        let eq60_e1743_d_n0: f64 = (eq60_e1741 * s.dn[272][0]);
        let eq60_e1743_d_n1: f64 = (eq60_e1741 * s.dn[272][1]);
        let eq60_e1743_d_n2: f64 = (eq60_e1741 * s.dn[272][2]);
        let eq60_e1743_d_n3: f64 = ((eq60_e1741_d_n3 * s.v[272]) + (eq60_e1741 * s.dn[272][3]));
        let eq60_e1743_d_n4: f64 = (eq60_e1741 * s.dn[272][4]);
        let eq60_e1743_d_n5: f64 = (eq60_e1741 * s.dn[272][5]);
        let eq60_e1743_d_n6: f64 = (eq60_e1741 * s.dn[272][6]);
        let eq60_e1743_d_n7: f64 = (eq60_e1741 * s.dn[272][7]);
        let eq60_e1743_d_n8: f64 = (eq60_e1741 * s.dn[272][8]);
        let eq60_e1743_d_n9: f64 = (eq60_e1741 * s.dn[272][9]);
        let eq60_e1743_d_n10: f64 = (eq60_e1741 * s.dn[272][10]);
        let eq60_e1743_d_n11: f64 = (eq60_e1741 * s.dn[272][11]);
        let eq60_e1743_d_n12: f64 = (eq60_e1741 * s.dn[272][12]);
        let eq60_e1743_d_n13: f64 = ((eq60_e1741_d_n13 * s.v[272]) + (eq60_e1741 * s.dn[272][13]));
        let eq60_e1743_d_n14: f64 = (eq60_e1741 * s.dn[272][14]);
        let eq60_e1743_d_n15: f64 = (eq60_e1741 * s.dn[272][15]);
        let eq60_e1743_d_n16: f64 = (eq60_e1741 * s.dn[272][16]);
        let eq60_e1743_d_b0: f64 = (eq60_e1741 * s.db[272][0]);
        let eq60_e1743_d_b1: f64 = (eq60_e1741 * s.db[272][1]);
        let eq60_e1743_d_b2: f64 = (eq60_e1741 * s.db[272][2]);
        let eq60_e1743_d_b3: f64 = (eq60_e1741 * s.db[272][3]);
        let eq60_e1743_d_b4: f64 = (eq60_e1741 * s.db[272][4]);
        let eq60_e1743_d_b5: f64 = (eq60_e1741 * s.db[272][5]);
        let eq60_e1743_d_b6: f64 = (eq60_e1741 * s.db[272][6]);
        let eq60_e1743_d_b7: f64 = (eq60_e1741 * s.db[272][7]);
        let eq60_e1743_d_b8: f64 = (eq60_e1741 * s.db[272][8]);
        let eq60_e1743_d_b9: f64 = (eq60_e1741 * s.db[272][9]);
        let eq60_e1743_d_b10: f64 = (eq60_e1741 * s.db[272][10]);
        let eq60_e1743_d_b11: f64 = (eq60_e1741 * s.db[272][11]);
        let eq60_e1743_d_b12: f64 = (eq60_e1741 * s.db[272][12]);
        let eq60_e1743_d_b13: f64 = (eq60_e1741 * s.db[272][13]);
        (eq60_e1743, eq60_e1743_d_n0, eq60_e1743_d_n1, eq60_e1743_d_n2, eq60_e1743_d_n3, eq60_e1743_d_n4, eq60_e1743_d_n5, eq60_e1743_d_n6, eq60_e1743_d_n7, eq60_e1743_d_n8, eq60_e1743_d_n9, eq60_e1743_d_n10, eq60_e1743_d_n11, eq60_e1743_d_n12, eq60_e1743_d_n13, eq60_e1743_d_n14, eq60_e1743_d_n15, eq60_e1743_d_n16, eq60_e1743_d_b0, eq60_e1743_d_b1, eq60_e1743_d_b2, eq60_e1743_d_b3, eq60_e1743_d_b4, eq60_e1743_d_b5, eq60_e1743_d_b6, eq60_e1743_d_b7, eq60_e1743_d_b8, eq60_e1743_d_b9, eq60_e1743_d_b10, eq60_e1743_d_b11, eq60_e1743_d_b12, eq60_e1743_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e1745;
        let eq60_node_derivatives: [f64; 17] = [eq60_e1745_d_n0, eq60_e1745_d_n1, eq60_e1745_d_n2, eq60_e1745_d_n3, eq60_e1745_d_n4, eq60_e1745_d_n5, eq60_e1745_d_n6, eq60_e1745_d_n7, eq60_e1745_d_n8, eq60_e1745_d_n9, eq60_e1745_d_n10, eq60_e1745_d_n11, eq60_e1745_d_n12, eq60_e1745_d_n13, eq60_e1745_d_n14, eq60_e1745_d_n15, eq60_e1745_d_n16];
        let eq60_branch_derivatives: [f64; 14] = [eq60_e1745_d_b0, eq60_e1745_d_b1, eq60_e1745_d_b2, eq60_e1745_d_b3, eq60_e1745_d_b4, eq60_e1745_d_b5, eq60_e1745_d_b6, eq60_e1745_d_b7, eq60_e1745_d_b8, eq60_e1745_d_b9, eq60_e1745_d_b10, eq60_e1745_d_b11, eq60_e1745_d_b12, eq60_e1745_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[13]),
            multiplicity * (eq60_value),
            nodes,
            &eq60_node_derivatives,
            branches,
            &eq60_branch_derivatives,
            multiplicity,
        );
        let (eq61_e1753, eq61_e1753_d_n0, eq61_e1753_d_n1, eq61_e1753_d_n2, eq61_e1753_d_n3, eq61_e1753_d_n4, eq61_e1753_d_n5, eq61_e1753_d_n6, eq61_e1753_d_n7, eq61_e1753_d_n8, eq61_e1753_d_n9, eq61_e1753_d_n10, eq61_e1753_d_n11, eq61_e1753_d_n12, eq61_e1753_d_n13, eq61_e1753_d_n14, eq61_e1753_d_n15, eq61_e1753_d_n16, eq61_e1753_d_b0, eq61_e1753_d_b1, eq61_e1753_d_b2, eq61_e1753_d_b3, eq61_e1753_d_b4, eq61_e1753_d_b5, eq61_e1753_d_b6, eq61_e1753_d_b7, eq61_e1753_d_b8, eq61_e1753_d_b9, eq61_e1753_d_b10, eq61_e1753_d_b11, eq61_e1753_d_b12, eq61_e1753_d_b13,) = {
    if s.b[1626] {
        let eq61_e1749: f64 = (p.p28 * (nv11 - nv13));
        let eq61_e1749_d_n11: f64 = p.p28;
        let eq61_e1749_d_n13: f64 = (-p.p28);
        let eq61_e1751: f64 = (eq61_e1749 * s.v[275]);
        let eq61_e1751_d_n0: f64 = (eq61_e1749 * s.dn[275][0]);
        let eq61_e1751_d_n1: f64 = (eq61_e1749 * s.dn[275][1]);
        let eq61_e1751_d_n2: f64 = (eq61_e1749 * s.dn[275][2]);
        let eq61_e1751_d_n3: f64 = (eq61_e1749 * s.dn[275][3]);
        let eq61_e1751_d_n4: f64 = (eq61_e1749 * s.dn[275][4]);
        let eq61_e1751_d_n5: f64 = (eq61_e1749 * s.dn[275][5]);
        let eq61_e1751_d_n6: f64 = (eq61_e1749 * s.dn[275][6]);
        let eq61_e1751_d_n7: f64 = (eq61_e1749 * s.dn[275][7]);
        let eq61_e1751_d_n8: f64 = (eq61_e1749 * s.dn[275][8]);
        let eq61_e1751_d_n9: f64 = (eq61_e1749 * s.dn[275][9]);
        let eq61_e1751_d_n10: f64 = (eq61_e1749 * s.dn[275][10]);
        let eq61_e1751_d_n11: f64 = ((eq61_e1749_d_n11 * s.v[275]) + (eq61_e1749 * s.dn[275][11]));
        let eq61_e1751_d_n12: f64 = (eq61_e1749 * s.dn[275][12]);
        let eq61_e1751_d_n13: f64 = ((eq61_e1749_d_n13 * s.v[275]) + (eq61_e1749 * s.dn[275][13]));
        let eq61_e1751_d_n14: f64 = (eq61_e1749 * s.dn[275][14]);
        let eq61_e1751_d_n15: f64 = (eq61_e1749 * s.dn[275][15]);
        let eq61_e1751_d_n16: f64 = (eq61_e1749 * s.dn[275][16]);
        let eq61_e1751_d_b0: f64 = (eq61_e1749 * s.db[275][0]);
        let eq61_e1751_d_b1: f64 = (eq61_e1749 * s.db[275][1]);
        let eq61_e1751_d_b2: f64 = (eq61_e1749 * s.db[275][2]);
        let eq61_e1751_d_b3: f64 = (eq61_e1749 * s.db[275][3]);
        let eq61_e1751_d_b4: f64 = (eq61_e1749 * s.db[275][4]);
        let eq61_e1751_d_b5: f64 = (eq61_e1749 * s.db[275][5]);
        let eq61_e1751_d_b6: f64 = (eq61_e1749 * s.db[275][6]);
        let eq61_e1751_d_b7: f64 = (eq61_e1749 * s.db[275][7]);
        let eq61_e1751_d_b8: f64 = (eq61_e1749 * s.db[275][8]);
        let eq61_e1751_d_b9: f64 = (eq61_e1749 * s.db[275][9]);
        let eq61_e1751_d_b10: f64 = (eq61_e1749 * s.db[275][10]);
        let eq61_e1751_d_b11: f64 = (eq61_e1749 * s.db[275][11]);
        let eq61_e1751_d_b12: f64 = (eq61_e1749 * s.db[275][12]);
        let eq61_e1751_d_b13: f64 = (eq61_e1749 * s.db[275][13]);
        (eq61_e1751, eq61_e1751_d_n0, eq61_e1751_d_n1, eq61_e1751_d_n2, eq61_e1751_d_n3, eq61_e1751_d_n4, eq61_e1751_d_n5, eq61_e1751_d_n6, eq61_e1751_d_n7, eq61_e1751_d_n8, eq61_e1751_d_n9, eq61_e1751_d_n10, eq61_e1751_d_n11, eq61_e1751_d_n12, eq61_e1751_d_n13, eq61_e1751_d_n14, eq61_e1751_d_n15, eq61_e1751_d_n16, eq61_e1751_d_b0, eq61_e1751_d_b1, eq61_e1751_d_b2, eq61_e1751_d_b3, eq61_e1751_d_b4, eq61_e1751_d_b5, eq61_e1751_d_b6, eq61_e1751_d_b7, eq61_e1751_d_b8, eq61_e1751_d_b9, eq61_e1751_d_b10, eq61_e1751_d_b11, eq61_e1751_d_b12, eq61_e1751_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_value: f64 = eq61_e1753;
        let eq61_node_derivatives: [f64; 17] = [eq61_e1753_d_n0, eq61_e1753_d_n1, eq61_e1753_d_n2, eq61_e1753_d_n3, eq61_e1753_d_n4, eq61_e1753_d_n5, eq61_e1753_d_n6, eq61_e1753_d_n7, eq61_e1753_d_n8, eq61_e1753_d_n9, eq61_e1753_d_n10, eq61_e1753_d_n11, eq61_e1753_d_n12, eq61_e1753_d_n13, eq61_e1753_d_n14, eq61_e1753_d_n15, eq61_e1753_d_n16];
        let eq61_branch_derivatives: [f64; 14] = [eq61_e1753_d_b0, eq61_e1753_d_b1, eq61_e1753_d_b2, eq61_e1753_d_b3, eq61_e1753_d_b4, eq61_e1753_d_b5, eq61_e1753_d_b6, eq61_e1753_d_b7, eq61_e1753_d_b8, eq61_e1753_d_b9, eq61_e1753_d_b10, eq61_e1753_d_b11, eq61_e1753_d_b12, eq61_e1753_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[13]),
            multiplicity * (eq61_value),
            nodes,
            &eq61_node_derivatives,
            branches,
            &eq61_branch_derivatives,
            multiplicity,
        );
        let (eq62_e1763,) = {
    if s.b[1626] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq62_value: f64 = eq62_e1763;
        stamper.stamp_current_const(
            Some(nodes[12]),
            Some(nodes[11]),
            multiplicity * (eq62_value),
        );
        let (eq63_e1773,) = {
    if s.b[1626] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq63_value: f64 = eq63_e1773;
        stamper.stamp_current_const(
            Some(nodes[12]),
            Some(nodes[3]),
            multiplicity * (eq63_value),
        );
    }

    pub(super) fn stamp_transient_equations_block_7(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq64_e1783,) = {
    if s.b[1626] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq64_value: f64 = eq64_e1783;
        stamper.stamp_current_const(
            Some(nodes[3]),
            Some(nodes[11]),
            multiplicity * (eq64_value),
        );
        let (eq65_e1793,) = {
    if s.b[1626] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq65_value: f64 = eq65_e1793;
        stamper.stamp_current_const(
            Some(nodes[13]),
            Some(nodes[11]),
            multiplicity * (eq65_value),
        );
        let (eq66_e1803,) = {
    if s.b[1626] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq66_value: f64 = eq66_e1803;
        stamper.stamp_current_const(
            Some(nodes[13]),
            Some(nodes[3]),
            multiplicity * (eq66_value),
        );
        let (eq67_e1808,) = {
    if (!s.b[1626]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq67_value: f64 = eq67_e1808;
        stamper.stamp_potential_const(
            branches[10],
            eq67_value,
        );
        let (eq68_e1813,) = {
    if (!s.b[1626]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq68_value: f64 = eq68_e1813;
        stamper.stamp_potential_const(
            branches[11],
            eq68_value,
        );
        let (eq69_e1818,) = {
    if (!s.b[1626]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq69_value: f64 = eq69_e1818;
        stamper.stamp_potential_const(
            branches[12],
            eq69_value,
        );
        let (eq70_e1832, eq70_e1832_d_n0, eq70_e1832_d_n1, eq70_e1832_d_n2, eq70_e1832_d_n3, eq70_e1832_d_n4, eq70_e1832_d_n5, eq70_e1832_d_n6, eq70_e1832_d_n7, eq70_e1832_d_n8, eq70_e1832_d_n9, eq70_e1832_d_n10, eq70_e1832_d_n11, eq70_e1832_d_n12, eq70_e1832_d_n13, eq70_e1832_d_n14, eq70_e1832_d_n15, eq70_e1832_d_n16, eq70_e1832_d_b0, eq70_e1832_d_b1, eq70_e1832_d_b2, eq70_e1832_d_b3, eq70_e1832_d_b4, eq70_e1832_d_b5, eq70_e1832_d_b6, eq70_e1832_d_b7, eq70_e1832_d_b8, eq70_e1832_d_b9, eq70_e1832_d_b10, eq70_e1832_d_b11, eq70_e1832_d_b12, eq70_e1832_d_b13,) = {
    if s.b[1627] {
        let eq70_e1822: f64 = (s.v[187] * p.p28);
        let eq70_e1822_d_n0: f64 = (s.dn[187][0] * p.p28);
        let eq70_e1822_d_n1: f64 = (s.dn[187][1] * p.p28);
        let eq70_e1822_d_n2: f64 = (s.dn[187][2] * p.p28);
        let eq70_e1822_d_n3: f64 = (s.dn[187][3] * p.p28);
        let eq70_e1822_d_n4: f64 = (s.dn[187][4] * p.p28);
        let eq70_e1822_d_n5: f64 = (s.dn[187][5] * p.p28);
        let eq70_e1822_d_n6: f64 = (s.dn[187][6] * p.p28);
        let eq70_e1822_d_n7: f64 = (s.dn[187][7] * p.p28);
        let eq70_e1822_d_n8: f64 = (s.dn[187][8] * p.p28);
        let eq70_e1822_d_n9: f64 = (s.dn[187][9] * p.p28);
        let eq70_e1822_d_n10: f64 = (s.dn[187][10] * p.p28);
        let eq70_e1822_d_n11: f64 = (s.dn[187][11] * p.p28);
        let eq70_e1822_d_n12: f64 = (s.dn[187][12] * p.p28);
        let eq70_e1822_d_n13: f64 = (s.dn[187][13] * p.p28);
        let eq70_e1822_d_n14: f64 = (s.dn[187][14] * p.p28);
        let eq70_e1822_d_n15: f64 = (s.dn[187][15] * p.p28);
        let eq70_e1822_d_n16: f64 = (s.dn[187][16] * p.p28);
        let eq70_e1822_d_b0: f64 = (s.db[187][0] * p.p28);
        let eq70_e1822_d_b1: f64 = (s.db[187][1] * p.p28);
        let eq70_e1822_d_b2: f64 = (s.db[187][2] * p.p28);
        let eq70_e1822_d_b3: f64 = (s.db[187][3] * p.p28);
        let eq70_e1822_d_b4: f64 = (s.db[187][4] * p.p28);
        let eq70_e1822_d_b5: f64 = (s.db[187][5] * p.p28);
        let eq70_e1822_d_b6: f64 = (s.db[187][6] * p.p28);
        let eq70_e1822_d_b7: f64 = (s.db[187][7] * p.p28);
        let eq70_e1822_d_b8: f64 = (s.db[187][8] * p.p28);
        let eq70_e1822_d_b9: f64 = (s.db[187][9] * p.p28);
        let eq70_e1822_d_b10: f64 = (s.db[187][10] * p.p28);
        let eq70_e1822_d_b11: f64 = (s.db[187][11] * p.p28);
        let eq70_e1822_d_b12: f64 = (s.db[187][12] * p.p28);
        let eq70_e1822_d_b13: f64 = (s.db[187][13] * p.p28);
        let eq70_e1824: f64 = (eq70_e1822 * s.v[303]);
        let eq70_e1824_d_n0: f64 = ((eq70_e1822_d_n0 * s.v[303]) + (eq70_e1822 * s.dn[303][0]));
        let eq70_e1824_d_n1: f64 = ((eq70_e1822_d_n1 * s.v[303]) + (eq70_e1822 * s.dn[303][1]));
        let eq70_e1824_d_n2: f64 = ((eq70_e1822_d_n2 * s.v[303]) + (eq70_e1822 * s.dn[303][2]));
        let eq70_e1824_d_n3: f64 = ((eq70_e1822_d_n3 * s.v[303]) + (eq70_e1822 * s.dn[303][3]));
        let eq70_e1824_d_n4: f64 = ((eq70_e1822_d_n4 * s.v[303]) + (eq70_e1822 * s.dn[303][4]));
        let eq70_e1824_d_n5: f64 = ((eq70_e1822_d_n5 * s.v[303]) + (eq70_e1822 * s.dn[303][5]));
        let eq70_e1824_d_n6: f64 = ((eq70_e1822_d_n6 * s.v[303]) + (eq70_e1822 * s.dn[303][6]));
        let eq70_e1824_d_n7: f64 = ((eq70_e1822_d_n7 * s.v[303]) + (eq70_e1822 * s.dn[303][7]));
        let eq70_e1824_d_n8: f64 = ((eq70_e1822_d_n8 * s.v[303]) + (eq70_e1822 * s.dn[303][8]));
        let eq70_e1824_d_n9: f64 = ((eq70_e1822_d_n9 * s.v[303]) + (eq70_e1822 * s.dn[303][9]));
        let eq70_e1824_d_n10: f64 = ((eq70_e1822_d_n10 * s.v[303]) + (eq70_e1822 * s.dn[303][10]));
        let eq70_e1824_d_n11: f64 = ((eq70_e1822_d_n11 * s.v[303]) + (eq70_e1822 * s.dn[303][11]));
        let eq70_e1824_d_n12: f64 = ((eq70_e1822_d_n12 * s.v[303]) + (eq70_e1822 * s.dn[303][12]));
        let eq70_e1824_d_n13: f64 = ((eq70_e1822_d_n13 * s.v[303]) + (eq70_e1822 * s.dn[303][13]));
        let eq70_e1824_d_n14: f64 = ((eq70_e1822_d_n14 * s.v[303]) + (eq70_e1822 * s.dn[303][14]));
        let eq70_e1824_d_n15: f64 = ((eq70_e1822_d_n15 * s.v[303]) + (eq70_e1822 * s.dn[303][15]));
        let eq70_e1824_d_n16: f64 = ((eq70_e1822_d_n16 * s.v[303]) + (eq70_e1822 * s.dn[303][16]));
        let eq70_e1824_d_b0: f64 = ((eq70_e1822_d_b0 * s.v[303]) + (eq70_e1822 * s.db[303][0]));
        let eq70_e1824_d_b1: f64 = ((eq70_e1822_d_b1 * s.v[303]) + (eq70_e1822 * s.db[303][1]));
        let eq70_e1824_d_b2: f64 = ((eq70_e1822_d_b2 * s.v[303]) + (eq70_e1822 * s.db[303][2]));
        let eq70_e1824_d_b3: f64 = ((eq70_e1822_d_b3 * s.v[303]) + (eq70_e1822 * s.db[303][3]));
        let eq70_e1824_d_b4: f64 = ((eq70_e1822_d_b4 * s.v[303]) + (eq70_e1822 * s.db[303][4]));
        let eq70_e1824_d_b5: f64 = ((eq70_e1822_d_b5 * s.v[303]) + (eq70_e1822 * s.db[303][5]));
        let eq70_e1824_d_b6: f64 = ((eq70_e1822_d_b6 * s.v[303]) + (eq70_e1822 * s.db[303][6]));
        let eq70_e1824_d_b7: f64 = ((eq70_e1822_d_b7 * s.v[303]) + (eq70_e1822 * s.db[303][7]));
        let eq70_e1824_d_b8: f64 = ((eq70_e1822_d_b8 * s.v[303]) + (eq70_e1822 * s.db[303][8]));
        let eq70_e1824_d_b9: f64 = ((eq70_e1822_d_b9 * s.v[303]) + (eq70_e1822 * s.db[303][9]));
        let eq70_e1824_d_b10: f64 = ((eq70_e1822_d_b10 * s.v[303]) + (eq70_e1822 * s.db[303][10]));
        let eq70_e1824_d_b11: f64 = ((eq70_e1822_d_b11 * s.v[303]) + (eq70_e1822 * s.db[303][11]));
        let eq70_e1824_d_b12: f64 = ((eq70_e1822_d_b12 * s.v[303]) + (eq70_e1822 * s.db[303][12]));
        let eq70_e1824_d_b13: f64 = ((eq70_e1822_d_b13 * s.v[303]) + (eq70_e1822 * s.db[303][13]));
        let eq70_e1827: f64 = ((nv12 - nv7) * p.p28);
        let eq70_e1827_d_n7: f64 = (-p.p28);
        let eq70_e1827_d_n12: f64 = p.p28;
        let eq70_e1829: f64 = (eq70_e1827 * s.v[781]);
        let eq70_e1829_d_n7: f64 = (eq70_e1827_d_n7 * s.v[781]);
        let eq70_e1829_d_n12: f64 = (eq70_e1827_d_n12 * s.v[781]);
        let eq70_e1830: f64 = (eq70_e1824 + eq70_e1829);
        let eq70_e1830_d_n7: f64 = (eq70_e1824_d_n7 + eq70_e1829_d_n7);
        let eq70_e1830_d_n12: f64 = (eq70_e1824_d_n12 + eq70_e1829_d_n12);
        (eq70_e1830, eq70_e1824_d_n0, eq70_e1824_d_n1, eq70_e1824_d_n2, eq70_e1824_d_n3, eq70_e1824_d_n4, eq70_e1824_d_n5, eq70_e1824_d_n6, eq70_e1830_d_n7, eq70_e1824_d_n8, eq70_e1824_d_n9, eq70_e1824_d_n10, eq70_e1824_d_n11, eq70_e1830_d_n12, eq70_e1824_d_n13, eq70_e1824_d_n14, eq70_e1824_d_n15, eq70_e1824_d_n16, eq70_e1824_d_b0, eq70_e1824_d_b1, eq70_e1824_d_b2, eq70_e1824_d_b3, eq70_e1824_d_b4, eq70_e1824_d_b5, eq70_e1824_d_b6, eq70_e1824_d_b7, eq70_e1824_d_b8, eq70_e1824_d_b9, eq70_e1824_d_b10, eq70_e1824_d_b11, eq70_e1824_d_b12, eq70_e1824_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq70_value: f64 = eq70_e1832;
        let eq70_node_derivatives: [f64; 17] = [eq70_e1832_d_n0, eq70_e1832_d_n1, eq70_e1832_d_n2, eq70_e1832_d_n3, eq70_e1832_d_n4, eq70_e1832_d_n5, eq70_e1832_d_n6, eq70_e1832_d_n7, eq70_e1832_d_n8, eq70_e1832_d_n9, eq70_e1832_d_n10, eq70_e1832_d_n11, eq70_e1832_d_n12, eq70_e1832_d_n13, eq70_e1832_d_n14, eq70_e1832_d_n15, eq70_e1832_d_n16];
        let eq70_branch_derivatives: [f64; 14] = [eq70_e1832_d_b0, eq70_e1832_d_b1, eq70_e1832_d_b2, eq70_e1832_d_b3, eq70_e1832_d_b4, eq70_e1832_d_b5, eq70_e1832_d_b6, eq70_e1832_d_b7, eq70_e1832_d_b8, eq70_e1832_d_b9, eq70_e1832_d_b10, eq70_e1832_d_b11, eq70_e1832_d_b12, eq70_e1832_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            multiplicity * (eq70_value),
            nodes,
            &eq70_node_derivatives,
            branches,
            &eq70_branch_derivatives,
            multiplicity,
        );
        let (eq71_e1841, eq71_e1841_d_n0, eq71_e1841_d_n1, eq71_e1841_d_n2, eq71_e1841_d_n3, eq71_e1841_d_n4, eq71_e1841_d_n5, eq71_e1841_d_n6, eq71_e1841_d_n7, eq71_e1841_d_n8, eq71_e1841_d_n9, eq71_e1841_d_n10, eq71_e1841_d_n11, eq71_e1841_d_n12, eq71_e1841_d_n13, eq71_e1841_d_n14, eq71_e1841_d_n15, eq71_e1841_d_n16, eq71_e1841_d_b0, eq71_e1841_d_b1, eq71_e1841_d_b2, eq71_e1841_d_b3, eq71_e1841_d_b4, eq71_e1841_d_b5, eq71_e1841_d_b6, eq71_e1841_d_b7, eq71_e1841_d_b8, eq71_e1841_d_b9, eq71_e1841_d_b10, eq71_e1841_d_b11, eq71_e1841_d_b12, eq71_e1841_d_b13,) = {
    if s.b[1627] {
        let eq71_e1837: f64 = (p.p29 * s.v[330]);
        let eq71_e1837_d_n0: f64 = (p.p29 * s.dn[330][0]);
        let eq71_e1837_d_n1: f64 = (p.p29 * s.dn[330][1]);
        let eq71_e1837_d_n2: f64 = (p.p29 * s.dn[330][2]);
        let eq71_e1837_d_n3: f64 = (p.p29 * s.dn[330][3]);
        let eq71_e1837_d_n4: f64 = (p.p29 * s.dn[330][4]);
        let eq71_e1837_d_n5: f64 = (p.p29 * s.dn[330][5]);
        let eq71_e1837_d_n6: f64 = (p.p29 * s.dn[330][6]);
        let eq71_e1837_d_n7: f64 = (p.p29 * s.dn[330][7]);
        let eq71_e1837_d_n8: f64 = (p.p29 * s.dn[330][8]);
        let eq71_e1837_d_n9: f64 = (p.p29 * s.dn[330][9]);
        let eq71_e1837_d_n10: f64 = (p.p29 * s.dn[330][10]);
        let eq71_e1837_d_n11: f64 = (p.p29 * s.dn[330][11]);
        let eq71_e1837_d_n12: f64 = (p.p29 * s.dn[330][12]);
        let eq71_e1837_d_n13: f64 = (p.p29 * s.dn[330][13]);
        let eq71_e1837_d_n14: f64 = (p.p29 * s.dn[330][14]);
        let eq71_e1837_d_n15: f64 = (p.p29 * s.dn[330][15]);
        let eq71_e1837_d_n16: f64 = (p.p29 * s.dn[330][16]);
        let eq71_e1837_d_b0: f64 = (p.p29 * s.db[330][0]);
        let eq71_e1837_d_b1: f64 = (p.p29 * s.db[330][1]);
        let eq71_e1837_d_b2: f64 = (p.p29 * s.db[330][2]);
        let eq71_e1837_d_b3: f64 = (p.p29 * s.db[330][3]);
        let eq71_e1837_d_b4: f64 = (p.p29 * s.db[330][4]);
        let eq71_e1837_d_b5: f64 = (p.p29 * s.db[330][5]);
        let eq71_e1837_d_b6: f64 = (p.p29 * s.db[330][6]);
        let eq71_e1837_d_b7: f64 = (p.p29 * s.db[330][7]);
        let eq71_e1837_d_b8: f64 = (p.p29 * s.db[330][8]);
        let eq71_e1837_d_b9: f64 = (p.p29 * s.db[330][9]);
        let eq71_e1837_d_b10: f64 = (p.p29 * s.db[330][10]);
        let eq71_e1837_d_b11: f64 = (p.p29 * s.db[330][11]);
        let eq71_e1837_d_b12: f64 = (p.p29 * s.db[330][12]);
        let eq71_e1837_d_b13: f64 = (p.p29 * s.db[330][13]);
        let eq71_e1838: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, eq71_e1837);
        let eq71_e1838_d_n0: f64 = (eq71_e1837_d_n0 * ddt_scale);
        let eq71_e1838_d_n1: f64 = (eq71_e1837_d_n1 * ddt_scale);
        let eq71_e1838_d_n2: f64 = (eq71_e1837_d_n2 * ddt_scale);
        let eq71_e1838_d_n3: f64 = (eq71_e1837_d_n3 * ddt_scale);
        let eq71_e1838_d_n4: f64 = (eq71_e1837_d_n4 * ddt_scale);
        let eq71_e1838_d_n5: f64 = (eq71_e1837_d_n5 * ddt_scale);
        let eq71_e1838_d_n6: f64 = (eq71_e1837_d_n6 * ddt_scale);
        let eq71_e1838_d_n7: f64 = (eq71_e1837_d_n7 * ddt_scale);
        let eq71_e1838_d_n8: f64 = (eq71_e1837_d_n8 * ddt_scale);
        let eq71_e1838_d_n9: f64 = (eq71_e1837_d_n9 * ddt_scale);
        let eq71_e1838_d_n10: f64 = (eq71_e1837_d_n10 * ddt_scale);
        let eq71_e1838_d_n11: f64 = (eq71_e1837_d_n11 * ddt_scale);
        let eq71_e1838_d_n12: f64 = (eq71_e1837_d_n12 * ddt_scale);
        let eq71_e1838_d_n13: f64 = (eq71_e1837_d_n13 * ddt_scale);
        let eq71_e1838_d_n14: f64 = (eq71_e1837_d_n14 * ddt_scale);
        let eq71_e1838_d_n15: f64 = (eq71_e1837_d_n15 * ddt_scale);
        let eq71_e1838_d_n16: f64 = (eq71_e1837_d_n16 * ddt_scale);
        let eq71_e1838_d_b0: f64 = (eq71_e1837_d_b0 * ddt_scale);
        let eq71_e1838_d_b1: f64 = (eq71_e1837_d_b1 * ddt_scale);
        let eq71_e1838_d_b2: f64 = (eq71_e1837_d_b2 * ddt_scale);
        let eq71_e1838_d_b3: f64 = (eq71_e1837_d_b3 * ddt_scale);
        let eq71_e1838_d_b4: f64 = (eq71_e1837_d_b4 * ddt_scale);
        let eq71_e1838_d_b5: f64 = (eq71_e1837_d_b5 * ddt_scale);
        let eq71_e1838_d_b6: f64 = (eq71_e1837_d_b6 * ddt_scale);
        let eq71_e1838_d_b7: f64 = (eq71_e1837_d_b7 * ddt_scale);
        let eq71_e1838_d_b8: f64 = (eq71_e1837_d_b8 * ddt_scale);
        let eq71_e1838_d_b9: f64 = (eq71_e1837_d_b9 * ddt_scale);
        let eq71_e1838_d_b10: f64 = (eq71_e1837_d_b10 * ddt_scale);
        let eq71_e1838_d_b11: f64 = (eq71_e1837_d_b11 * ddt_scale);
        let eq71_e1838_d_b12: f64 = (eq71_e1837_d_b12 * ddt_scale);
        let eq71_e1838_d_b13: f64 = (eq71_e1837_d_b13 * ddt_scale);
        let eq71_e1839: f64 = (s.v[187] * eq71_e1838);
        let eq71_e1839_d_n0: f64 = ((s.dn[187][0] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n0));
        let eq71_e1839_d_n1: f64 = ((s.dn[187][1] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n1));
        let eq71_e1839_d_n2: f64 = ((s.dn[187][2] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n2));
        let eq71_e1839_d_n3: f64 = ((s.dn[187][3] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n3));
        let eq71_e1839_d_n4: f64 = ((s.dn[187][4] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n4));
        let eq71_e1839_d_n5: f64 = ((s.dn[187][5] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n5));
        let eq71_e1839_d_n6: f64 = ((s.dn[187][6] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n6));
        let eq71_e1839_d_n7: f64 = ((s.dn[187][7] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n7));
        let eq71_e1839_d_n8: f64 = ((s.dn[187][8] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n8));
        let eq71_e1839_d_n9: f64 = ((s.dn[187][9] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n9));
        let eq71_e1839_d_n10: f64 = ((s.dn[187][10] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n10));
        let eq71_e1839_d_n11: f64 = ((s.dn[187][11] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n11));
        let eq71_e1839_d_n12: f64 = ((s.dn[187][12] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n12));
        let eq71_e1839_d_n13: f64 = ((s.dn[187][13] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n13));
        let eq71_e1839_d_n14: f64 = ((s.dn[187][14] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n14));
        let eq71_e1839_d_n15: f64 = ((s.dn[187][15] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n15));
        let eq71_e1839_d_n16: f64 = ((s.dn[187][16] * eq71_e1838) + (s.v[187] * eq71_e1838_d_n16));
        let eq71_e1839_d_b0: f64 = ((s.db[187][0] * eq71_e1838) + (s.v[187] * eq71_e1838_d_b0));
        let eq71_e1839_d_b1: f64 = ((s.db[187][1] * eq71_e1838) + (s.v[187] * eq71_e1838_d_b1));
        let eq71_e1839_d_b2: f64 = ((s.db[187][2] * eq71_e1838) + (s.v[187] * eq71_e1838_d_b2));
        let eq71_e1839_d_b3: f64 = ((s.db[187][3] * eq71_e1838) + (s.v[187] * eq71_e1838_d_b3));
        let eq71_e1839_d_b4: f64 = ((s.db[187][4] * eq71_e1838) + (s.v[187] * eq71_e1838_d_b4));
        let eq71_e1839_d_b5: f64 = ((s.db[187][5] * eq71_e1838) + (s.v[187] * eq71_e1838_d_b5));
        let eq71_e1839_d_b6: f64 = ((s.db[187][6] * eq71_e1838) + (s.v[187] * eq71_e1838_d_b6));
        let eq71_e1839_d_b7: f64 = ((s.db[187][7] * eq71_e1838) + (s.v[187] * eq71_e1838_d_b7));
        let eq71_e1839_d_b8: f64 = ((s.db[187][8] * eq71_e1838) + (s.v[187] * eq71_e1838_d_b8));
        let eq71_e1839_d_b9: f64 = ((s.db[187][9] * eq71_e1838) + (s.v[187] * eq71_e1838_d_b9));
        let eq71_e1839_d_b10: f64 = ((s.db[187][10] * eq71_e1838) + (s.v[187] * eq71_e1838_d_b10));
        let eq71_e1839_d_b11: f64 = ((s.db[187][11] * eq71_e1838) + (s.v[187] * eq71_e1838_d_b11));
        let eq71_e1839_d_b12: f64 = ((s.db[187][12] * eq71_e1838) + (s.v[187] * eq71_e1838_d_b12));
        let eq71_e1839_d_b13: f64 = ((s.db[187][13] * eq71_e1838) + (s.v[187] * eq71_e1838_d_b13));
        (eq71_e1839, eq71_e1839_d_n0, eq71_e1839_d_n1, eq71_e1839_d_n2, eq71_e1839_d_n3, eq71_e1839_d_n4, eq71_e1839_d_n5, eq71_e1839_d_n6, eq71_e1839_d_n7, eq71_e1839_d_n8, eq71_e1839_d_n9, eq71_e1839_d_n10, eq71_e1839_d_n11, eq71_e1839_d_n12, eq71_e1839_d_n13, eq71_e1839_d_n14, eq71_e1839_d_n15, eq71_e1839_d_n16, eq71_e1839_d_b0, eq71_e1839_d_b1, eq71_e1839_d_b2, eq71_e1839_d_b3, eq71_e1839_d_b4, eq71_e1839_d_b5, eq71_e1839_d_b6, eq71_e1839_d_b7, eq71_e1839_d_b8, eq71_e1839_d_b9, eq71_e1839_d_b10, eq71_e1839_d_b11, eq71_e1839_d_b12, eq71_e1839_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq71_value: f64 = eq71_e1841;
        let eq71_node_derivatives: [f64; 17] = [eq71_e1841_d_n0, eq71_e1841_d_n1, eq71_e1841_d_n2, eq71_e1841_d_n3, eq71_e1841_d_n4, eq71_e1841_d_n5, eq71_e1841_d_n6, eq71_e1841_d_n7, eq71_e1841_d_n8, eq71_e1841_d_n9, eq71_e1841_d_n10, eq71_e1841_d_n11, eq71_e1841_d_n12, eq71_e1841_d_n13, eq71_e1841_d_n14, eq71_e1841_d_n15, eq71_e1841_d_n16];
        let eq71_branch_derivatives: [f64; 14] = [eq71_e1841_d_b0, eq71_e1841_d_b1, eq71_e1841_d_b2, eq71_e1841_d_b3, eq71_e1841_d_b4, eq71_e1841_d_b5, eq71_e1841_d_b6, eq71_e1841_d_b7, eq71_e1841_d_b8, eq71_e1841_d_b9, eq71_e1841_d_b10, eq71_e1841_d_b11, eq71_e1841_d_b12, eq71_e1841_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            multiplicity * (eq71_value),
            nodes,
            &eq71_node_derivatives,
            branches,
            &eq71_branch_derivatives,
            multiplicity,
        );
        let (eq72_e1857, eq72_e1857_d_n0, eq72_e1857_d_n1, eq72_e1857_d_n2, eq72_e1857_d_n3, eq72_e1857_d_n4, eq72_e1857_d_n5, eq72_e1857_d_n6, eq72_e1857_d_n7, eq72_e1857_d_n8, eq72_e1857_d_n9, eq72_e1857_d_n10, eq72_e1857_d_n11, eq72_e1857_d_n12, eq72_e1857_d_n13, eq72_e1857_d_n14, eq72_e1857_d_n15, eq72_e1857_d_n16, eq72_e1857_d_b0, eq72_e1857_d_b1, eq72_e1857_d_b2, eq72_e1857_d_b3, eq72_e1857_d_b4, eq72_e1857_d_b5, eq72_e1857_d_b6, eq72_e1857_d_b7, eq72_e1857_d_b8, eq72_e1857_d_b9, eq72_e1857_d_b10, eq72_e1857_d_b11, eq72_e1857_d_b12, eq72_e1857_d_b13,) = {
    if (s.b[1627] && s.b[1628]) {
        let eq72_e1847: f64 = (s.v[187] * p.p28);
        let eq72_e1847_d_n0: f64 = (s.dn[187][0] * p.p28);
        let eq72_e1847_d_n1: f64 = (s.dn[187][1] * p.p28);
        let eq72_e1847_d_n2: f64 = (s.dn[187][2] * p.p28);
        let eq72_e1847_d_n3: f64 = (s.dn[187][3] * p.p28);
        let eq72_e1847_d_n4: f64 = (s.dn[187][4] * p.p28);
        let eq72_e1847_d_n5: f64 = (s.dn[187][5] * p.p28);
        let eq72_e1847_d_n6: f64 = (s.dn[187][6] * p.p28);
        let eq72_e1847_d_n7: f64 = (s.dn[187][7] * p.p28);
        let eq72_e1847_d_n8: f64 = (s.dn[187][8] * p.p28);
        let eq72_e1847_d_n9: f64 = (s.dn[187][9] * p.p28);
        let eq72_e1847_d_n10: f64 = (s.dn[187][10] * p.p28);
        let eq72_e1847_d_n11: f64 = (s.dn[187][11] * p.p28);
        let eq72_e1847_d_n12: f64 = (s.dn[187][12] * p.p28);
        let eq72_e1847_d_n13: f64 = (s.dn[187][13] * p.p28);
        let eq72_e1847_d_n14: f64 = (s.dn[187][14] * p.p28);
        let eq72_e1847_d_n15: f64 = (s.dn[187][15] * p.p28);
        let eq72_e1847_d_n16: f64 = (s.dn[187][16] * p.p28);
        let eq72_e1847_d_b0: f64 = (s.db[187][0] * p.p28);
        let eq72_e1847_d_b1: f64 = (s.db[187][1] * p.p28);
        let eq72_e1847_d_b2: f64 = (s.db[187][2] * p.p28);
        let eq72_e1847_d_b3: f64 = (s.db[187][3] * p.p28);
        let eq72_e1847_d_b4: f64 = (s.db[187][4] * p.p28);
        let eq72_e1847_d_b5: f64 = (s.db[187][5] * p.p28);
        let eq72_e1847_d_b6: f64 = (s.db[187][6] * p.p28);
        let eq72_e1847_d_b7: f64 = (s.db[187][7] * p.p28);
        let eq72_e1847_d_b8: f64 = (s.db[187][8] * p.p28);
        let eq72_e1847_d_b9: f64 = (s.db[187][9] * p.p28);
        let eq72_e1847_d_b10: f64 = (s.db[187][10] * p.p28);
        let eq72_e1847_d_b11: f64 = (s.db[187][11] * p.p28);
        let eq72_e1847_d_b12: f64 = (s.db[187][12] * p.p28);
        let eq72_e1847_d_b13: f64 = (s.db[187][13] * p.p28);
        let eq72_e1849: f64 = (eq72_e1847 * s.v[304]);
        let eq72_e1849_d_n0: f64 = ((eq72_e1847_d_n0 * s.v[304]) + (eq72_e1847 * s.dn[304][0]));
        let eq72_e1849_d_n1: f64 = ((eq72_e1847_d_n1 * s.v[304]) + (eq72_e1847 * s.dn[304][1]));
        let eq72_e1849_d_n2: f64 = ((eq72_e1847_d_n2 * s.v[304]) + (eq72_e1847 * s.dn[304][2]));
        let eq72_e1849_d_n3: f64 = ((eq72_e1847_d_n3 * s.v[304]) + (eq72_e1847 * s.dn[304][3]));
        let eq72_e1849_d_n4: f64 = ((eq72_e1847_d_n4 * s.v[304]) + (eq72_e1847 * s.dn[304][4]));
        let eq72_e1849_d_n5: f64 = ((eq72_e1847_d_n5 * s.v[304]) + (eq72_e1847 * s.dn[304][5]));
        let eq72_e1849_d_n6: f64 = ((eq72_e1847_d_n6 * s.v[304]) + (eq72_e1847 * s.dn[304][6]));
        let eq72_e1849_d_n7: f64 = ((eq72_e1847_d_n7 * s.v[304]) + (eq72_e1847 * s.dn[304][7]));
        let eq72_e1849_d_n8: f64 = ((eq72_e1847_d_n8 * s.v[304]) + (eq72_e1847 * s.dn[304][8]));
        let eq72_e1849_d_n9: f64 = ((eq72_e1847_d_n9 * s.v[304]) + (eq72_e1847 * s.dn[304][9]));
        let eq72_e1849_d_n10: f64 = ((eq72_e1847_d_n10 * s.v[304]) + (eq72_e1847 * s.dn[304][10]));
        let eq72_e1849_d_n11: f64 = ((eq72_e1847_d_n11 * s.v[304]) + (eq72_e1847 * s.dn[304][11]));
        let eq72_e1849_d_n12: f64 = ((eq72_e1847_d_n12 * s.v[304]) + (eq72_e1847 * s.dn[304][12]));
        let eq72_e1849_d_n13: f64 = ((eq72_e1847_d_n13 * s.v[304]) + (eq72_e1847 * s.dn[304][13]));
        let eq72_e1849_d_n14: f64 = ((eq72_e1847_d_n14 * s.v[304]) + (eq72_e1847 * s.dn[304][14]));
        let eq72_e1849_d_n15: f64 = ((eq72_e1847_d_n15 * s.v[304]) + (eq72_e1847 * s.dn[304][15]));
        let eq72_e1849_d_n16: f64 = ((eq72_e1847_d_n16 * s.v[304]) + (eq72_e1847 * s.dn[304][16]));
        let eq72_e1849_d_b0: f64 = ((eq72_e1847_d_b0 * s.v[304]) + (eq72_e1847 * s.db[304][0]));
        let eq72_e1849_d_b1: f64 = ((eq72_e1847_d_b1 * s.v[304]) + (eq72_e1847 * s.db[304][1]));
        let eq72_e1849_d_b2: f64 = ((eq72_e1847_d_b2 * s.v[304]) + (eq72_e1847 * s.db[304][2]));
        let eq72_e1849_d_b3: f64 = ((eq72_e1847_d_b3 * s.v[304]) + (eq72_e1847 * s.db[304][3]));
        let eq72_e1849_d_b4: f64 = ((eq72_e1847_d_b4 * s.v[304]) + (eq72_e1847 * s.db[304][4]));
        let eq72_e1849_d_b5: f64 = ((eq72_e1847_d_b5 * s.v[304]) + (eq72_e1847 * s.db[304][5]));
        let eq72_e1849_d_b6: f64 = ((eq72_e1847_d_b6 * s.v[304]) + (eq72_e1847 * s.db[304][6]));
        let eq72_e1849_d_b7: f64 = ((eq72_e1847_d_b7 * s.v[304]) + (eq72_e1847 * s.db[304][7]));
        let eq72_e1849_d_b8: f64 = ((eq72_e1847_d_b8 * s.v[304]) + (eq72_e1847 * s.db[304][8]));
        let eq72_e1849_d_b9: f64 = ((eq72_e1847_d_b9 * s.v[304]) + (eq72_e1847 * s.db[304][9]));
        let eq72_e1849_d_b10: f64 = ((eq72_e1847_d_b10 * s.v[304]) + (eq72_e1847 * s.db[304][10]));
        let eq72_e1849_d_b11: f64 = ((eq72_e1847_d_b11 * s.v[304]) + (eq72_e1847 * s.db[304][11]));
        let eq72_e1849_d_b12: f64 = ((eq72_e1847_d_b12 * s.v[304]) + (eq72_e1847 * s.db[304][12]));
        let eq72_e1849_d_b13: f64 = ((eq72_e1847_d_b13 * s.v[304]) + (eq72_e1847 * s.db[304][13]));
        let eq72_e1852: f64 = ((nv13 - nv5) * p.p28);
        let eq72_e1852_d_n5: f64 = (-p.p28);
        let eq72_e1852_d_n13: f64 = p.p28;
        let eq72_e1854: f64 = (eq72_e1852 * s.v[781]);
        let eq72_e1854_d_n5: f64 = (eq72_e1852_d_n5 * s.v[781]);
        let eq72_e1854_d_n13: f64 = (eq72_e1852_d_n13 * s.v[781]);
        let eq72_e1855: f64 = (eq72_e1849 + eq72_e1854);
        let eq72_e1855_d_n5: f64 = (eq72_e1849_d_n5 + eq72_e1854_d_n5);
        let eq72_e1855_d_n13: f64 = (eq72_e1849_d_n13 + eq72_e1854_d_n13);
        (eq72_e1855, eq72_e1849_d_n0, eq72_e1849_d_n1, eq72_e1849_d_n2, eq72_e1849_d_n3, eq72_e1849_d_n4, eq72_e1855_d_n5, eq72_e1849_d_n6, eq72_e1849_d_n7, eq72_e1849_d_n8, eq72_e1849_d_n9, eq72_e1849_d_n10, eq72_e1849_d_n11, eq72_e1849_d_n12, eq72_e1855_d_n13, eq72_e1849_d_n14, eq72_e1849_d_n15, eq72_e1849_d_n16, eq72_e1849_d_b0, eq72_e1849_d_b1, eq72_e1849_d_b2, eq72_e1849_d_b3, eq72_e1849_d_b4, eq72_e1849_d_b5, eq72_e1849_d_b6, eq72_e1849_d_b7, eq72_e1849_d_b8, eq72_e1849_d_b9, eq72_e1849_d_b10, eq72_e1849_d_b11, eq72_e1849_d_b12, eq72_e1849_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_value: f64 = eq72_e1857;
        let eq72_node_derivatives: [f64; 17] = [eq72_e1857_d_n0, eq72_e1857_d_n1, eq72_e1857_d_n2, eq72_e1857_d_n3, eq72_e1857_d_n4, eq72_e1857_d_n5, eq72_e1857_d_n6, eq72_e1857_d_n7, eq72_e1857_d_n8, eq72_e1857_d_n9, eq72_e1857_d_n10, eq72_e1857_d_n11, eq72_e1857_d_n12, eq72_e1857_d_n13, eq72_e1857_d_n14, eq72_e1857_d_n15, eq72_e1857_d_n16];
        let eq72_branch_derivatives: [f64; 14] = [eq72_e1857_d_b0, eq72_e1857_d_b1, eq72_e1857_d_b2, eq72_e1857_d_b3, eq72_e1857_d_b4, eq72_e1857_d_b5, eq72_e1857_d_b6, eq72_e1857_d_b7, eq72_e1857_d_b8, eq72_e1857_d_b9, eq72_e1857_d_b10, eq72_e1857_d_b11, eq72_e1857_d_b12, eq72_e1857_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            Some(nodes[5]),
            multiplicity * (eq72_value),
            nodes,
            &eq72_node_derivatives,
            branches,
            &eq72_branch_derivatives,
            multiplicity,
        );
        let (eq73_e1868, eq73_e1868_d_n0, eq73_e1868_d_n1, eq73_e1868_d_n2, eq73_e1868_d_n3, eq73_e1868_d_n4, eq73_e1868_d_n5, eq73_e1868_d_n6, eq73_e1868_d_n7, eq73_e1868_d_n8, eq73_e1868_d_n9, eq73_e1868_d_n10, eq73_e1868_d_n11, eq73_e1868_d_n12, eq73_e1868_d_n13, eq73_e1868_d_n14, eq73_e1868_d_n15, eq73_e1868_d_n16, eq73_e1868_d_b0, eq73_e1868_d_b1, eq73_e1868_d_b2, eq73_e1868_d_b3, eq73_e1868_d_b4, eq73_e1868_d_b5, eq73_e1868_d_b6, eq73_e1868_d_b7, eq73_e1868_d_b8, eq73_e1868_d_b9, eq73_e1868_d_b10, eq73_e1868_d_b11, eq73_e1868_d_b12, eq73_e1868_d_b13,) = {
    if (s.b[1627] && s.b[1628]) {
        let eq73_e1864: f64 = (p.p29 * s.v[334]);
        let eq73_e1864_d_n0: f64 = (p.p29 * s.dn[334][0]);
        let eq73_e1864_d_n1: f64 = (p.p29 * s.dn[334][1]);
        let eq73_e1864_d_n2: f64 = (p.p29 * s.dn[334][2]);
        let eq73_e1864_d_n3: f64 = (p.p29 * s.dn[334][3]);
        let eq73_e1864_d_n4: f64 = (p.p29 * s.dn[334][4]);
        let eq73_e1864_d_n5: f64 = (p.p29 * s.dn[334][5]);
        let eq73_e1864_d_n6: f64 = (p.p29 * s.dn[334][6]);
        let eq73_e1864_d_n7: f64 = (p.p29 * s.dn[334][7]);
        let eq73_e1864_d_n8: f64 = (p.p29 * s.dn[334][8]);
        let eq73_e1864_d_n9: f64 = (p.p29 * s.dn[334][9]);
        let eq73_e1864_d_n10: f64 = (p.p29 * s.dn[334][10]);
        let eq73_e1864_d_n11: f64 = (p.p29 * s.dn[334][11]);
        let eq73_e1864_d_n12: f64 = (p.p29 * s.dn[334][12]);
        let eq73_e1864_d_n13: f64 = (p.p29 * s.dn[334][13]);
        let eq73_e1864_d_n14: f64 = (p.p29 * s.dn[334][14]);
        let eq73_e1864_d_n15: f64 = (p.p29 * s.dn[334][15]);
        let eq73_e1864_d_n16: f64 = (p.p29 * s.dn[334][16]);
        let eq73_e1864_d_b0: f64 = (p.p29 * s.db[334][0]);
        let eq73_e1864_d_b1: f64 = (p.p29 * s.db[334][1]);
        let eq73_e1864_d_b2: f64 = (p.p29 * s.db[334][2]);
        let eq73_e1864_d_b3: f64 = (p.p29 * s.db[334][3]);
        let eq73_e1864_d_b4: f64 = (p.p29 * s.db[334][4]);
        let eq73_e1864_d_b5: f64 = (p.p29 * s.db[334][5]);
        let eq73_e1864_d_b6: f64 = (p.p29 * s.db[334][6]);
        let eq73_e1864_d_b7: f64 = (p.p29 * s.db[334][7]);
        let eq73_e1864_d_b8: f64 = (p.p29 * s.db[334][8]);
        let eq73_e1864_d_b9: f64 = (p.p29 * s.db[334][9]);
        let eq73_e1864_d_b10: f64 = (p.p29 * s.db[334][10]);
        let eq73_e1864_d_b11: f64 = (p.p29 * s.db[334][11]);
        let eq73_e1864_d_b12: f64 = (p.p29 * s.db[334][12]);
        let eq73_e1864_d_b13: f64 = (p.p29 * s.db[334][13]);
        let eq73_e1865: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 11, eq73_e1864);
        let eq73_e1865_d_n0: f64 = (eq73_e1864_d_n0 * ddt_scale);
        let eq73_e1865_d_n1: f64 = (eq73_e1864_d_n1 * ddt_scale);
        let eq73_e1865_d_n2: f64 = (eq73_e1864_d_n2 * ddt_scale);
        let eq73_e1865_d_n3: f64 = (eq73_e1864_d_n3 * ddt_scale);
        let eq73_e1865_d_n4: f64 = (eq73_e1864_d_n4 * ddt_scale);
        let eq73_e1865_d_n5: f64 = (eq73_e1864_d_n5 * ddt_scale);
        let eq73_e1865_d_n6: f64 = (eq73_e1864_d_n6 * ddt_scale);
        let eq73_e1865_d_n7: f64 = (eq73_e1864_d_n7 * ddt_scale);
        let eq73_e1865_d_n8: f64 = (eq73_e1864_d_n8 * ddt_scale);
        let eq73_e1865_d_n9: f64 = (eq73_e1864_d_n9 * ddt_scale);
        let eq73_e1865_d_n10: f64 = (eq73_e1864_d_n10 * ddt_scale);
        let eq73_e1865_d_n11: f64 = (eq73_e1864_d_n11 * ddt_scale);
        let eq73_e1865_d_n12: f64 = (eq73_e1864_d_n12 * ddt_scale);
        let eq73_e1865_d_n13: f64 = (eq73_e1864_d_n13 * ddt_scale);
        let eq73_e1865_d_n14: f64 = (eq73_e1864_d_n14 * ddt_scale);
        let eq73_e1865_d_n15: f64 = (eq73_e1864_d_n15 * ddt_scale);
        let eq73_e1865_d_n16: f64 = (eq73_e1864_d_n16 * ddt_scale);
        let eq73_e1865_d_b0: f64 = (eq73_e1864_d_b0 * ddt_scale);
        let eq73_e1865_d_b1: f64 = (eq73_e1864_d_b1 * ddt_scale);
        let eq73_e1865_d_b2: f64 = (eq73_e1864_d_b2 * ddt_scale);
        let eq73_e1865_d_b3: f64 = (eq73_e1864_d_b3 * ddt_scale);
        let eq73_e1865_d_b4: f64 = (eq73_e1864_d_b4 * ddt_scale);
        let eq73_e1865_d_b5: f64 = (eq73_e1864_d_b5 * ddt_scale);
        let eq73_e1865_d_b6: f64 = (eq73_e1864_d_b6 * ddt_scale);
        let eq73_e1865_d_b7: f64 = (eq73_e1864_d_b7 * ddt_scale);
        let eq73_e1865_d_b8: f64 = (eq73_e1864_d_b8 * ddt_scale);
        let eq73_e1865_d_b9: f64 = (eq73_e1864_d_b9 * ddt_scale);
        let eq73_e1865_d_b10: f64 = (eq73_e1864_d_b10 * ddt_scale);
        let eq73_e1865_d_b11: f64 = (eq73_e1864_d_b11 * ddt_scale);
        let eq73_e1865_d_b12: f64 = (eq73_e1864_d_b12 * ddt_scale);
        let eq73_e1865_d_b13: f64 = (eq73_e1864_d_b13 * ddt_scale);
        let eq73_e1866: f64 = (s.v[187] * eq73_e1865);
        let eq73_e1866_d_n0: f64 = ((s.dn[187][0] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n0));
        let eq73_e1866_d_n1: f64 = ((s.dn[187][1] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n1));
        let eq73_e1866_d_n2: f64 = ((s.dn[187][2] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n2));
        let eq73_e1866_d_n3: f64 = ((s.dn[187][3] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n3));
        let eq73_e1866_d_n4: f64 = ((s.dn[187][4] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n4));
        let eq73_e1866_d_n5: f64 = ((s.dn[187][5] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n5));
        let eq73_e1866_d_n6: f64 = ((s.dn[187][6] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n6));
        let eq73_e1866_d_n7: f64 = ((s.dn[187][7] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n7));
        let eq73_e1866_d_n8: f64 = ((s.dn[187][8] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n8));
        let eq73_e1866_d_n9: f64 = ((s.dn[187][9] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n9));
        let eq73_e1866_d_n10: f64 = ((s.dn[187][10] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n10));
        let eq73_e1866_d_n11: f64 = ((s.dn[187][11] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n11));
        let eq73_e1866_d_n12: f64 = ((s.dn[187][12] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n12));
        let eq73_e1866_d_n13: f64 = ((s.dn[187][13] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n13));
        let eq73_e1866_d_n14: f64 = ((s.dn[187][14] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n14));
        let eq73_e1866_d_n15: f64 = ((s.dn[187][15] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n15));
        let eq73_e1866_d_n16: f64 = ((s.dn[187][16] * eq73_e1865) + (s.v[187] * eq73_e1865_d_n16));
        let eq73_e1866_d_b0: f64 = ((s.db[187][0] * eq73_e1865) + (s.v[187] * eq73_e1865_d_b0));
        let eq73_e1866_d_b1: f64 = ((s.db[187][1] * eq73_e1865) + (s.v[187] * eq73_e1865_d_b1));
        let eq73_e1866_d_b2: f64 = ((s.db[187][2] * eq73_e1865) + (s.v[187] * eq73_e1865_d_b2));
        let eq73_e1866_d_b3: f64 = ((s.db[187][3] * eq73_e1865) + (s.v[187] * eq73_e1865_d_b3));
        let eq73_e1866_d_b4: f64 = ((s.db[187][4] * eq73_e1865) + (s.v[187] * eq73_e1865_d_b4));
        let eq73_e1866_d_b5: f64 = ((s.db[187][5] * eq73_e1865) + (s.v[187] * eq73_e1865_d_b5));
        let eq73_e1866_d_b6: f64 = ((s.db[187][6] * eq73_e1865) + (s.v[187] * eq73_e1865_d_b6));
        let eq73_e1866_d_b7: f64 = ((s.db[187][7] * eq73_e1865) + (s.v[187] * eq73_e1865_d_b7));
        let eq73_e1866_d_b8: f64 = ((s.db[187][8] * eq73_e1865) + (s.v[187] * eq73_e1865_d_b8));
        let eq73_e1866_d_b9: f64 = ((s.db[187][9] * eq73_e1865) + (s.v[187] * eq73_e1865_d_b9));
        let eq73_e1866_d_b10: f64 = ((s.db[187][10] * eq73_e1865) + (s.v[187] * eq73_e1865_d_b10));
        let eq73_e1866_d_b11: f64 = ((s.db[187][11] * eq73_e1865) + (s.v[187] * eq73_e1865_d_b11));
        let eq73_e1866_d_b12: f64 = ((s.db[187][12] * eq73_e1865) + (s.v[187] * eq73_e1865_d_b12));
        let eq73_e1866_d_b13: f64 = ((s.db[187][13] * eq73_e1865) + (s.v[187] * eq73_e1865_d_b13));
        (eq73_e1866, eq73_e1866_d_n0, eq73_e1866_d_n1, eq73_e1866_d_n2, eq73_e1866_d_n3, eq73_e1866_d_n4, eq73_e1866_d_n5, eq73_e1866_d_n6, eq73_e1866_d_n7, eq73_e1866_d_n8, eq73_e1866_d_n9, eq73_e1866_d_n10, eq73_e1866_d_n11, eq73_e1866_d_n12, eq73_e1866_d_n13, eq73_e1866_d_n14, eq73_e1866_d_n15, eq73_e1866_d_n16, eq73_e1866_d_b0, eq73_e1866_d_b1, eq73_e1866_d_b2, eq73_e1866_d_b3, eq73_e1866_d_b4, eq73_e1866_d_b5, eq73_e1866_d_b6, eq73_e1866_d_b7, eq73_e1866_d_b8, eq73_e1866_d_b9, eq73_e1866_d_b10, eq73_e1866_d_b11, eq73_e1866_d_b12, eq73_e1866_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_value: f64 = eq73_e1868;
        let eq73_node_derivatives: [f64; 17] = [eq73_e1868_d_n0, eq73_e1868_d_n1, eq73_e1868_d_n2, eq73_e1868_d_n3, eq73_e1868_d_n4, eq73_e1868_d_n5, eq73_e1868_d_n6, eq73_e1868_d_n7, eq73_e1868_d_n8, eq73_e1868_d_n9, eq73_e1868_d_n10, eq73_e1868_d_n11, eq73_e1868_d_n12, eq73_e1868_d_n13, eq73_e1868_d_n14, eq73_e1868_d_n15, eq73_e1868_d_n16];
        let eq73_branch_derivatives: [f64; 14] = [eq73_e1868_d_b0, eq73_e1868_d_b1, eq73_e1868_d_b2, eq73_e1868_d_b3, eq73_e1868_d_b4, eq73_e1868_d_b5, eq73_e1868_d_b6, eq73_e1868_d_b7, eq73_e1868_d_b8, eq73_e1868_d_b9, eq73_e1868_d_b10, eq73_e1868_d_b11, eq73_e1868_d_b12, eq73_e1868_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            Some(nodes[5]),
            multiplicity * (eq73_value),
            nodes,
            &eq73_node_derivatives,
            branches,
            &eq73_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_8(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq74_e1883, eq74_e1883_d_n0, eq74_e1883_d_n1, eq74_e1883_d_n2, eq74_e1883_d_n3, eq74_e1883_d_n4, eq74_e1883_d_n5, eq74_e1883_d_n6, eq74_e1883_d_n7, eq74_e1883_d_n8, eq74_e1883_d_n9, eq74_e1883_d_n10, eq74_e1883_d_n11, eq74_e1883_d_n12, eq74_e1883_d_n13, eq74_e1883_d_n14, eq74_e1883_d_n15, eq74_e1883_d_n16, eq74_e1883_d_b0, eq74_e1883_d_b1, eq74_e1883_d_b2, eq74_e1883_d_b3, eq74_e1883_d_b4, eq74_e1883_d_b5, eq74_e1883_d_b6, eq74_e1883_d_b7, eq74_e1883_d_b8, eq74_e1883_d_b9, eq74_e1883_d_b10, eq74_e1883_d_b11, eq74_e1883_d_b12, eq74_e1883_d_b13,) = {
    if (!s.b[1627]) {
        let eq74_e1873: f64 = (s.v[187] * p.p28);
        let eq74_e1873_d_n0: f64 = (s.dn[187][0] * p.p28);
        let eq74_e1873_d_n1: f64 = (s.dn[187][1] * p.p28);
        let eq74_e1873_d_n2: f64 = (s.dn[187][2] * p.p28);
        let eq74_e1873_d_n3: f64 = (s.dn[187][3] * p.p28);
        let eq74_e1873_d_n4: f64 = (s.dn[187][4] * p.p28);
        let eq74_e1873_d_n5: f64 = (s.dn[187][5] * p.p28);
        let eq74_e1873_d_n6: f64 = (s.dn[187][6] * p.p28);
        let eq74_e1873_d_n7: f64 = (s.dn[187][7] * p.p28);
        let eq74_e1873_d_n8: f64 = (s.dn[187][8] * p.p28);
        let eq74_e1873_d_n9: f64 = (s.dn[187][9] * p.p28);
        let eq74_e1873_d_n10: f64 = (s.dn[187][10] * p.p28);
        let eq74_e1873_d_n11: f64 = (s.dn[187][11] * p.p28);
        let eq74_e1873_d_n12: f64 = (s.dn[187][12] * p.p28);
        let eq74_e1873_d_n13: f64 = (s.dn[187][13] * p.p28);
        let eq74_e1873_d_n14: f64 = (s.dn[187][14] * p.p28);
        let eq74_e1873_d_n15: f64 = (s.dn[187][15] * p.p28);
        let eq74_e1873_d_n16: f64 = (s.dn[187][16] * p.p28);
        let eq74_e1873_d_b0: f64 = (s.db[187][0] * p.p28);
        let eq74_e1873_d_b1: f64 = (s.db[187][1] * p.p28);
        let eq74_e1873_d_b2: f64 = (s.db[187][2] * p.p28);
        let eq74_e1873_d_b3: f64 = (s.db[187][3] * p.p28);
        let eq74_e1873_d_b4: f64 = (s.db[187][4] * p.p28);
        let eq74_e1873_d_b5: f64 = (s.db[187][5] * p.p28);
        let eq74_e1873_d_b6: f64 = (s.db[187][6] * p.p28);
        let eq74_e1873_d_b7: f64 = (s.db[187][7] * p.p28);
        let eq74_e1873_d_b8: f64 = (s.db[187][8] * p.p28);
        let eq74_e1873_d_b9: f64 = (s.db[187][9] * p.p28);
        let eq74_e1873_d_b10: f64 = (s.db[187][10] * p.p28);
        let eq74_e1873_d_b11: f64 = (s.db[187][11] * p.p28);
        let eq74_e1873_d_b12: f64 = (s.db[187][12] * p.p28);
        let eq74_e1873_d_b13: f64 = (s.db[187][13] * p.p28);
        let eq74_e1875: f64 = (eq74_e1873 * s.v[303]);
        let eq74_e1875_d_n0: f64 = ((eq74_e1873_d_n0 * s.v[303]) + (eq74_e1873 * s.dn[303][0]));
        let eq74_e1875_d_n1: f64 = ((eq74_e1873_d_n1 * s.v[303]) + (eq74_e1873 * s.dn[303][1]));
        let eq74_e1875_d_n2: f64 = ((eq74_e1873_d_n2 * s.v[303]) + (eq74_e1873 * s.dn[303][2]));
        let eq74_e1875_d_n3: f64 = ((eq74_e1873_d_n3 * s.v[303]) + (eq74_e1873 * s.dn[303][3]));
        let eq74_e1875_d_n4: f64 = ((eq74_e1873_d_n4 * s.v[303]) + (eq74_e1873 * s.dn[303][4]));
        let eq74_e1875_d_n5: f64 = ((eq74_e1873_d_n5 * s.v[303]) + (eq74_e1873 * s.dn[303][5]));
        let eq74_e1875_d_n6: f64 = ((eq74_e1873_d_n6 * s.v[303]) + (eq74_e1873 * s.dn[303][6]));
        let eq74_e1875_d_n7: f64 = ((eq74_e1873_d_n7 * s.v[303]) + (eq74_e1873 * s.dn[303][7]));
        let eq74_e1875_d_n8: f64 = ((eq74_e1873_d_n8 * s.v[303]) + (eq74_e1873 * s.dn[303][8]));
        let eq74_e1875_d_n9: f64 = ((eq74_e1873_d_n9 * s.v[303]) + (eq74_e1873 * s.dn[303][9]));
        let eq74_e1875_d_n10: f64 = ((eq74_e1873_d_n10 * s.v[303]) + (eq74_e1873 * s.dn[303][10]));
        let eq74_e1875_d_n11: f64 = ((eq74_e1873_d_n11 * s.v[303]) + (eq74_e1873 * s.dn[303][11]));
        let eq74_e1875_d_n12: f64 = ((eq74_e1873_d_n12 * s.v[303]) + (eq74_e1873 * s.dn[303][12]));
        let eq74_e1875_d_n13: f64 = ((eq74_e1873_d_n13 * s.v[303]) + (eq74_e1873 * s.dn[303][13]));
        let eq74_e1875_d_n14: f64 = ((eq74_e1873_d_n14 * s.v[303]) + (eq74_e1873 * s.dn[303][14]));
        let eq74_e1875_d_n15: f64 = ((eq74_e1873_d_n15 * s.v[303]) + (eq74_e1873 * s.dn[303][15]));
        let eq74_e1875_d_n16: f64 = ((eq74_e1873_d_n16 * s.v[303]) + (eq74_e1873 * s.dn[303][16]));
        let eq74_e1875_d_b0: f64 = ((eq74_e1873_d_b0 * s.v[303]) + (eq74_e1873 * s.db[303][0]));
        let eq74_e1875_d_b1: f64 = ((eq74_e1873_d_b1 * s.v[303]) + (eq74_e1873 * s.db[303][1]));
        let eq74_e1875_d_b2: f64 = ((eq74_e1873_d_b2 * s.v[303]) + (eq74_e1873 * s.db[303][2]));
        let eq74_e1875_d_b3: f64 = ((eq74_e1873_d_b3 * s.v[303]) + (eq74_e1873 * s.db[303][3]));
        let eq74_e1875_d_b4: f64 = ((eq74_e1873_d_b4 * s.v[303]) + (eq74_e1873 * s.db[303][4]));
        let eq74_e1875_d_b5: f64 = ((eq74_e1873_d_b5 * s.v[303]) + (eq74_e1873 * s.db[303][5]));
        let eq74_e1875_d_b6: f64 = ((eq74_e1873_d_b6 * s.v[303]) + (eq74_e1873 * s.db[303][6]));
        let eq74_e1875_d_b7: f64 = ((eq74_e1873_d_b7 * s.v[303]) + (eq74_e1873 * s.db[303][7]));
        let eq74_e1875_d_b8: f64 = ((eq74_e1873_d_b8 * s.v[303]) + (eq74_e1873 * s.db[303][8]));
        let eq74_e1875_d_b9: f64 = ((eq74_e1873_d_b9 * s.v[303]) + (eq74_e1873 * s.db[303][9]));
        let eq74_e1875_d_b10: f64 = ((eq74_e1873_d_b10 * s.v[303]) + (eq74_e1873 * s.db[303][10]));
        let eq74_e1875_d_b11: f64 = ((eq74_e1873_d_b11 * s.v[303]) + (eq74_e1873 * s.db[303][11]));
        let eq74_e1875_d_b12: f64 = ((eq74_e1873_d_b12 * s.v[303]) + (eq74_e1873 * s.db[303][12]));
        let eq74_e1875_d_b13: f64 = ((eq74_e1873_d_b13 * s.v[303]) + (eq74_e1873 * s.db[303][13]));
        let eq74_e1878: f64 = ((nv11 - nv7) * p.p28);
        let eq74_e1878_d_n7: f64 = (-p.p28);
        let eq74_e1878_d_n11: f64 = p.p28;
        let eq74_e1880: f64 = (eq74_e1878 * s.v[781]);
        let eq74_e1880_d_n7: f64 = (eq74_e1878_d_n7 * s.v[781]);
        let eq74_e1880_d_n11: f64 = (eq74_e1878_d_n11 * s.v[781]);
        let eq74_e1881: f64 = (eq74_e1875 + eq74_e1880);
        let eq74_e1881_d_n7: f64 = (eq74_e1875_d_n7 + eq74_e1880_d_n7);
        let eq74_e1881_d_n11: f64 = (eq74_e1875_d_n11 + eq74_e1880_d_n11);
        (eq74_e1881, eq74_e1875_d_n0, eq74_e1875_d_n1, eq74_e1875_d_n2, eq74_e1875_d_n3, eq74_e1875_d_n4, eq74_e1875_d_n5, eq74_e1875_d_n6, eq74_e1881_d_n7, eq74_e1875_d_n8, eq74_e1875_d_n9, eq74_e1875_d_n10, eq74_e1881_d_n11, eq74_e1875_d_n12, eq74_e1875_d_n13, eq74_e1875_d_n14, eq74_e1875_d_n15, eq74_e1875_d_n16, eq74_e1875_d_b0, eq74_e1875_d_b1, eq74_e1875_d_b2, eq74_e1875_d_b3, eq74_e1875_d_b4, eq74_e1875_d_b5, eq74_e1875_d_b6, eq74_e1875_d_b7, eq74_e1875_d_b8, eq74_e1875_d_b9, eq74_e1875_d_b10, eq74_e1875_d_b11, eq74_e1875_d_b12, eq74_e1875_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_value: f64 = eq74_e1883;
        let eq74_node_derivatives: [f64; 17] = [eq74_e1883_d_n0, eq74_e1883_d_n1, eq74_e1883_d_n2, eq74_e1883_d_n3, eq74_e1883_d_n4, eq74_e1883_d_n5, eq74_e1883_d_n6, eq74_e1883_d_n7, eq74_e1883_d_n8, eq74_e1883_d_n9, eq74_e1883_d_n10, eq74_e1883_d_n11, eq74_e1883_d_n12, eq74_e1883_d_n13, eq74_e1883_d_n14, eq74_e1883_d_n15, eq74_e1883_d_n16];
        let eq74_branch_derivatives: [f64; 14] = [eq74_e1883_d_b0, eq74_e1883_d_b1, eq74_e1883_d_b2, eq74_e1883_d_b3, eq74_e1883_d_b4, eq74_e1883_d_b5, eq74_e1883_d_b6, eq74_e1883_d_b7, eq74_e1883_d_b8, eq74_e1883_d_b9, eq74_e1883_d_b10, eq74_e1883_d_b11, eq74_e1883_d_b12, eq74_e1883_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            multiplicity * (eq74_value),
            nodes,
            &eq74_node_derivatives,
            branches,
            &eq74_branch_derivatives,
            multiplicity,
        );
        let (eq75_e1898, eq75_e1898_d_n0, eq75_e1898_d_n1, eq75_e1898_d_n2, eq75_e1898_d_n3, eq75_e1898_d_n4, eq75_e1898_d_n5, eq75_e1898_d_n6, eq75_e1898_d_n7, eq75_e1898_d_n8, eq75_e1898_d_n9, eq75_e1898_d_n10, eq75_e1898_d_n11, eq75_e1898_d_n12, eq75_e1898_d_n13, eq75_e1898_d_n14, eq75_e1898_d_n15, eq75_e1898_d_n16, eq75_e1898_d_b0, eq75_e1898_d_b1, eq75_e1898_d_b2, eq75_e1898_d_b3, eq75_e1898_d_b4, eq75_e1898_d_b5, eq75_e1898_d_b6, eq75_e1898_d_b7, eq75_e1898_d_b8, eq75_e1898_d_b9, eq75_e1898_d_b10, eq75_e1898_d_b11, eq75_e1898_d_b12, eq75_e1898_d_b13,) = {
    if (!s.b[1627]) {
        let eq75_e1888: f64 = (s.v[187] * p.p28);
        let eq75_e1888_d_n0: f64 = (s.dn[187][0] * p.p28);
        let eq75_e1888_d_n1: f64 = (s.dn[187][1] * p.p28);
        let eq75_e1888_d_n2: f64 = (s.dn[187][2] * p.p28);
        let eq75_e1888_d_n3: f64 = (s.dn[187][3] * p.p28);
        let eq75_e1888_d_n4: f64 = (s.dn[187][4] * p.p28);
        let eq75_e1888_d_n5: f64 = (s.dn[187][5] * p.p28);
        let eq75_e1888_d_n6: f64 = (s.dn[187][6] * p.p28);
        let eq75_e1888_d_n7: f64 = (s.dn[187][7] * p.p28);
        let eq75_e1888_d_n8: f64 = (s.dn[187][8] * p.p28);
        let eq75_e1888_d_n9: f64 = (s.dn[187][9] * p.p28);
        let eq75_e1888_d_n10: f64 = (s.dn[187][10] * p.p28);
        let eq75_e1888_d_n11: f64 = (s.dn[187][11] * p.p28);
        let eq75_e1888_d_n12: f64 = (s.dn[187][12] * p.p28);
        let eq75_e1888_d_n13: f64 = (s.dn[187][13] * p.p28);
        let eq75_e1888_d_n14: f64 = (s.dn[187][14] * p.p28);
        let eq75_e1888_d_n15: f64 = (s.dn[187][15] * p.p28);
        let eq75_e1888_d_n16: f64 = (s.dn[187][16] * p.p28);
        let eq75_e1888_d_b0: f64 = (s.db[187][0] * p.p28);
        let eq75_e1888_d_b1: f64 = (s.db[187][1] * p.p28);
        let eq75_e1888_d_b2: f64 = (s.db[187][2] * p.p28);
        let eq75_e1888_d_b3: f64 = (s.db[187][3] * p.p28);
        let eq75_e1888_d_b4: f64 = (s.db[187][4] * p.p28);
        let eq75_e1888_d_b5: f64 = (s.db[187][5] * p.p28);
        let eq75_e1888_d_b6: f64 = (s.db[187][6] * p.p28);
        let eq75_e1888_d_b7: f64 = (s.db[187][7] * p.p28);
        let eq75_e1888_d_b8: f64 = (s.db[187][8] * p.p28);
        let eq75_e1888_d_b9: f64 = (s.db[187][9] * p.p28);
        let eq75_e1888_d_b10: f64 = (s.db[187][10] * p.p28);
        let eq75_e1888_d_b11: f64 = (s.db[187][11] * p.p28);
        let eq75_e1888_d_b12: f64 = (s.db[187][12] * p.p28);
        let eq75_e1888_d_b13: f64 = (s.db[187][13] * p.p28);
        let eq75_e1890: f64 = (eq75_e1888 * s.v[304]);
        let eq75_e1890_d_n0: f64 = ((eq75_e1888_d_n0 * s.v[304]) + (eq75_e1888 * s.dn[304][0]));
        let eq75_e1890_d_n1: f64 = ((eq75_e1888_d_n1 * s.v[304]) + (eq75_e1888 * s.dn[304][1]));
        let eq75_e1890_d_n2: f64 = ((eq75_e1888_d_n2 * s.v[304]) + (eq75_e1888 * s.dn[304][2]));
        let eq75_e1890_d_n3: f64 = ((eq75_e1888_d_n3 * s.v[304]) + (eq75_e1888 * s.dn[304][3]));
        let eq75_e1890_d_n4: f64 = ((eq75_e1888_d_n4 * s.v[304]) + (eq75_e1888 * s.dn[304][4]));
        let eq75_e1890_d_n5: f64 = ((eq75_e1888_d_n5 * s.v[304]) + (eq75_e1888 * s.dn[304][5]));
        let eq75_e1890_d_n6: f64 = ((eq75_e1888_d_n6 * s.v[304]) + (eq75_e1888 * s.dn[304][6]));
        let eq75_e1890_d_n7: f64 = ((eq75_e1888_d_n7 * s.v[304]) + (eq75_e1888 * s.dn[304][7]));
        let eq75_e1890_d_n8: f64 = ((eq75_e1888_d_n8 * s.v[304]) + (eq75_e1888 * s.dn[304][8]));
        let eq75_e1890_d_n9: f64 = ((eq75_e1888_d_n9 * s.v[304]) + (eq75_e1888 * s.dn[304][9]));
        let eq75_e1890_d_n10: f64 = ((eq75_e1888_d_n10 * s.v[304]) + (eq75_e1888 * s.dn[304][10]));
        let eq75_e1890_d_n11: f64 = ((eq75_e1888_d_n11 * s.v[304]) + (eq75_e1888 * s.dn[304][11]));
        let eq75_e1890_d_n12: f64 = ((eq75_e1888_d_n12 * s.v[304]) + (eq75_e1888 * s.dn[304][12]));
        let eq75_e1890_d_n13: f64 = ((eq75_e1888_d_n13 * s.v[304]) + (eq75_e1888 * s.dn[304][13]));
        let eq75_e1890_d_n14: f64 = ((eq75_e1888_d_n14 * s.v[304]) + (eq75_e1888 * s.dn[304][14]));
        let eq75_e1890_d_n15: f64 = ((eq75_e1888_d_n15 * s.v[304]) + (eq75_e1888 * s.dn[304][15]));
        let eq75_e1890_d_n16: f64 = ((eq75_e1888_d_n16 * s.v[304]) + (eq75_e1888 * s.dn[304][16]));
        let eq75_e1890_d_b0: f64 = ((eq75_e1888_d_b0 * s.v[304]) + (eq75_e1888 * s.db[304][0]));
        let eq75_e1890_d_b1: f64 = ((eq75_e1888_d_b1 * s.v[304]) + (eq75_e1888 * s.db[304][1]));
        let eq75_e1890_d_b2: f64 = ((eq75_e1888_d_b2 * s.v[304]) + (eq75_e1888 * s.db[304][2]));
        let eq75_e1890_d_b3: f64 = ((eq75_e1888_d_b3 * s.v[304]) + (eq75_e1888 * s.db[304][3]));
        let eq75_e1890_d_b4: f64 = ((eq75_e1888_d_b4 * s.v[304]) + (eq75_e1888 * s.db[304][4]));
        let eq75_e1890_d_b5: f64 = ((eq75_e1888_d_b5 * s.v[304]) + (eq75_e1888 * s.db[304][5]));
        let eq75_e1890_d_b6: f64 = ((eq75_e1888_d_b6 * s.v[304]) + (eq75_e1888 * s.db[304][6]));
        let eq75_e1890_d_b7: f64 = ((eq75_e1888_d_b7 * s.v[304]) + (eq75_e1888 * s.db[304][7]));
        let eq75_e1890_d_b8: f64 = ((eq75_e1888_d_b8 * s.v[304]) + (eq75_e1888 * s.db[304][8]));
        let eq75_e1890_d_b9: f64 = ((eq75_e1888_d_b9 * s.v[304]) + (eq75_e1888 * s.db[304][9]));
        let eq75_e1890_d_b10: f64 = ((eq75_e1888_d_b10 * s.v[304]) + (eq75_e1888 * s.db[304][10]));
        let eq75_e1890_d_b11: f64 = ((eq75_e1888_d_b11 * s.v[304]) + (eq75_e1888 * s.db[304][11]));
        let eq75_e1890_d_b12: f64 = ((eq75_e1888_d_b12 * s.v[304]) + (eq75_e1888 * s.db[304][12]));
        let eq75_e1890_d_b13: f64 = ((eq75_e1888_d_b13 * s.v[304]) + (eq75_e1888 * s.db[304][13]));
        let eq75_e1893: f64 = ((nv11 - nv5) * p.p28);
        let eq75_e1893_d_n5: f64 = (-p.p28);
        let eq75_e1893_d_n11: f64 = p.p28;
        let eq75_e1895: f64 = (eq75_e1893 * s.v[781]);
        let eq75_e1895_d_n5: f64 = (eq75_e1893_d_n5 * s.v[781]);
        let eq75_e1895_d_n11: f64 = (eq75_e1893_d_n11 * s.v[781]);
        let eq75_e1896: f64 = (eq75_e1890 + eq75_e1895);
        let eq75_e1896_d_n5: f64 = (eq75_e1890_d_n5 + eq75_e1895_d_n5);
        let eq75_e1896_d_n11: f64 = (eq75_e1890_d_n11 + eq75_e1895_d_n11);
        (eq75_e1896, eq75_e1890_d_n0, eq75_e1890_d_n1, eq75_e1890_d_n2, eq75_e1890_d_n3, eq75_e1890_d_n4, eq75_e1896_d_n5, eq75_e1890_d_n6, eq75_e1890_d_n7, eq75_e1890_d_n8, eq75_e1890_d_n9, eq75_e1890_d_n10, eq75_e1896_d_n11, eq75_e1890_d_n12, eq75_e1890_d_n13, eq75_e1890_d_n14, eq75_e1890_d_n15, eq75_e1890_d_n16, eq75_e1890_d_b0, eq75_e1890_d_b1, eq75_e1890_d_b2, eq75_e1890_d_b3, eq75_e1890_d_b4, eq75_e1890_d_b5, eq75_e1890_d_b6, eq75_e1890_d_b7, eq75_e1890_d_b8, eq75_e1890_d_b9, eq75_e1890_d_b10, eq75_e1890_d_b11, eq75_e1890_d_b12, eq75_e1890_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq75_value: f64 = eq75_e1898;
        let eq75_node_derivatives: [f64; 17] = [eq75_e1898_d_n0, eq75_e1898_d_n1, eq75_e1898_d_n2, eq75_e1898_d_n3, eq75_e1898_d_n4, eq75_e1898_d_n5, eq75_e1898_d_n6, eq75_e1898_d_n7, eq75_e1898_d_n8, eq75_e1898_d_n9, eq75_e1898_d_n10, eq75_e1898_d_n11, eq75_e1898_d_n12, eq75_e1898_d_n13, eq75_e1898_d_n14, eq75_e1898_d_n15, eq75_e1898_d_n16];
        let eq75_branch_derivatives: [f64; 14] = [eq75_e1898_d_b0, eq75_e1898_d_b1, eq75_e1898_d_b2, eq75_e1898_d_b3, eq75_e1898_d_b4, eq75_e1898_d_b5, eq75_e1898_d_b6, eq75_e1898_d_b7, eq75_e1898_d_b8, eq75_e1898_d_b9, eq75_e1898_d_b10, eq75_e1898_d_b11, eq75_e1898_d_b12, eq75_e1898_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[5]),
            multiplicity * (eq75_value),
            nodes,
            &eq75_node_derivatives,
            branches,
            &eq75_branch_derivatives,
            multiplicity,
        );
        let (eq76_e1908, eq76_e1908_d_n0, eq76_e1908_d_n1, eq76_e1908_d_n2, eq76_e1908_d_n3, eq76_e1908_d_n4, eq76_e1908_d_n5, eq76_e1908_d_n6, eq76_e1908_d_n7, eq76_e1908_d_n8, eq76_e1908_d_n9, eq76_e1908_d_n10, eq76_e1908_d_n11, eq76_e1908_d_n12, eq76_e1908_d_n13, eq76_e1908_d_n14, eq76_e1908_d_n15, eq76_e1908_d_n16, eq76_e1908_d_b0, eq76_e1908_d_b1, eq76_e1908_d_b2, eq76_e1908_d_b3, eq76_e1908_d_b4, eq76_e1908_d_b5, eq76_e1908_d_b6, eq76_e1908_d_b7, eq76_e1908_d_b8, eq76_e1908_d_b9, eq76_e1908_d_b10, eq76_e1908_d_b11, eq76_e1908_d_b12, eq76_e1908_d_b13,) = {
    if (!s.b[1627]) {
        let eq76_e1904: f64 = (p.p29 * s.v[330]);
        let eq76_e1904_d_n0: f64 = (p.p29 * s.dn[330][0]);
        let eq76_e1904_d_n1: f64 = (p.p29 * s.dn[330][1]);
        let eq76_e1904_d_n2: f64 = (p.p29 * s.dn[330][2]);
        let eq76_e1904_d_n3: f64 = (p.p29 * s.dn[330][3]);
        let eq76_e1904_d_n4: f64 = (p.p29 * s.dn[330][4]);
        let eq76_e1904_d_n5: f64 = (p.p29 * s.dn[330][5]);
        let eq76_e1904_d_n6: f64 = (p.p29 * s.dn[330][6]);
        let eq76_e1904_d_n7: f64 = (p.p29 * s.dn[330][7]);
        let eq76_e1904_d_n8: f64 = (p.p29 * s.dn[330][8]);
        let eq76_e1904_d_n9: f64 = (p.p29 * s.dn[330][9]);
        let eq76_e1904_d_n10: f64 = (p.p29 * s.dn[330][10]);
        let eq76_e1904_d_n11: f64 = (p.p29 * s.dn[330][11]);
        let eq76_e1904_d_n12: f64 = (p.p29 * s.dn[330][12]);
        let eq76_e1904_d_n13: f64 = (p.p29 * s.dn[330][13]);
        let eq76_e1904_d_n14: f64 = (p.p29 * s.dn[330][14]);
        let eq76_e1904_d_n15: f64 = (p.p29 * s.dn[330][15]);
        let eq76_e1904_d_n16: f64 = (p.p29 * s.dn[330][16]);
        let eq76_e1904_d_b0: f64 = (p.p29 * s.db[330][0]);
        let eq76_e1904_d_b1: f64 = (p.p29 * s.db[330][1]);
        let eq76_e1904_d_b2: f64 = (p.p29 * s.db[330][2]);
        let eq76_e1904_d_b3: f64 = (p.p29 * s.db[330][3]);
        let eq76_e1904_d_b4: f64 = (p.p29 * s.db[330][4]);
        let eq76_e1904_d_b5: f64 = (p.p29 * s.db[330][5]);
        let eq76_e1904_d_b6: f64 = (p.p29 * s.db[330][6]);
        let eq76_e1904_d_b7: f64 = (p.p29 * s.db[330][7]);
        let eq76_e1904_d_b8: f64 = (p.p29 * s.db[330][8]);
        let eq76_e1904_d_b9: f64 = (p.p29 * s.db[330][9]);
        let eq76_e1904_d_b10: f64 = (p.p29 * s.db[330][10]);
        let eq76_e1904_d_b11: f64 = (p.p29 * s.db[330][11]);
        let eq76_e1904_d_b12: f64 = (p.p29 * s.db[330][12]);
        let eq76_e1904_d_b13: f64 = (p.p29 * s.db[330][13]);
        let eq76_e1905: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 12, eq76_e1904);
        let eq76_e1905_d_n0: f64 = (eq76_e1904_d_n0 * ddt_scale);
        let eq76_e1905_d_n1: f64 = (eq76_e1904_d_n1 * ddt_scale);
        let eq76_e1905_d_n2: f64 = (eq76_e1904_d_n2 * ddt_scale);
        let eq76_e1905_d_n3: f64 = (eq76_e1904_d_n3 * ddt_scale);
        let eq76_e1905_d_n4: f64 = (eq76_e1904_d_n4 * ddt_scale);
        let eq76_e1905_d_n5: f64 = (eq76_e1904_d_n5 * ddt_scale);
        let eq76_e1905_d_n6: f64 = (eq76_e1904_d_n6 * ddt_scale);
        let eq76_e1905_d_n7: f64 = (eq76_e1904_d_n7 * ddt_scale);
        let eq76_e1905_d_n8: f64 = (eq76_e1904_d_n8 * ddt_scale);
        let eq76_e1905_d_n9: f64 = (eq76_e1904_d_n9 * ddt_scale);
        let eq76_e1905_d_n10: f64 = (eq76_e1904_d_n10 * ddt_scale);
        let eq76_e1905_d_n11: f64 = (eq76_e1904_d_n11 * ddt_scale);
        let eq76_e1905_d_n12: f64 = (eq76_e1904_d_n12 * ddt_scale);
        let eq76_e1905_d_n13: f64 = (eq76_e1904_d_n13 * ddt_scale);
        let eq76_e1905_d_n14: f64 = (eq76_e1904_d_n14 * ddt_scale);
        let eq76_e1905_d_n15: f64 = (eq76_e1904_d_n15 * ddt_scale);
        let eq76_e1905_d_n16: f64 = (eq76_e1904_d_n16 * ddt_scale);
        let eq76_e1905_d_b0: f64 = (eq76_e1904_d_b0 * ddt_scale);
        let eq76_e1905_d_b1: f64 = (eq76_e1904_d_b1 * ddt_scale);
        let eq76_e1905_d_b2: f64 = (eq76_e1904_d_b2 * ddt_scale);
        let eq76_e1905_d_b3: f64 = (eq76_e1904_d_b3 * ddt_scale);
        let eq76_e1905_d_b4: f64 = (eq76_e1904_d_b4 * ddt_scale);
        let eq76_e1905_d_b5: f64 = (eq76_e1904_d_b5 * ddt_scale);
        let eq76_e1905_d_b6: f64 = (eq76_e1904_d_b6 * ddt_scale);
        let eq76_e1905_d_b7: f64 = (eq76_e1904_d_b7 * ddt_scale);
        let eq76_e1905_d_b8: f64 = (eq76_e1904_d_b8 * ddt_scale);
        let eq76_e1905_d_b9: f64 = (eq76_e1904_d_b9 * ddt_scale);
        let eq76_e1905_d_b10: f64 = (eq76_e1904_d_b10 * ddt_scale);
        let eq76_e1905_d_b11: f64 = (eq76_e1904_d_b11 * ddt_scale);
        let eq76_e1905_d_b12: f64 = (eq76_e1904_d_b12 * ddt_scale);
        let eq76_e1905_d_b13: f64 = (eq76_e1904_d_b13 * ddt_scale);
        let eq76_e1906: f64 = (s.v[187] * eq76_e1905);
        let eq76_e1906_d_n0: f64 = ((s.dn[187][0] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n0));
        let eq76_e1906_d_n1: f64 = ((s.dn[187][1] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n1));
        let eq76_e1906_d_n2: f64 = ((s.dn[187][2] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n2));
        let eq76_e1906_d_n3: f64 = ((s.dn[187][3] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n3));
        let eq76_e1906_d_n4: f64 = ((s.dn[187][4] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n4));
        let eq76_e1906_d_n5: f64 = ((s.dn[187][5] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n5));
        let eq76_e1906_d_n6: f64 = ((s.dn[187][6] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n6));
        let eq76_e1906_d_n7: f64 = ((s.dn[187][7] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n7));
        let eq76_e1906_d_n8: f64 = ((s.dn[187][8] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n8));
        let eq76_e1906_d_n9: f64 = ((s.dn[187][9] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n9));
        let eq76_e1906_d_n10: f64 = ((s.dn[187][10] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n10));
        let eq76_e1906_d_n11: f64 = ((s.dn[187][11] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n11));
        let eq76_e1906_d_n12: f64 = ((s.dn[187][12] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n12));
        let eq76_e1906_d_n13: f64 = ((s.dn[187][13] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n13));
        let eq76_e1906_d_n14: f64 = ((s.dn[187][14] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n14));
        let eq76_e1906_d_n15: f64 = ((s.dn[187][15] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n15));
        let eq76_e1906_d_n16: f64 = ((s.dn[187][16] * eq76_e1905) + (s.v[187] * eq76_e1905_d_n16));
        let eq76_e1906_d_b0: f64 = ((s.db[187][0] * eq76_e1905) + (s.v[187] * eq76_e1905_d_b0));
        let eq76_e1906_d_b1: f64 = ((s.db[187][1] * eq76_e1905) + (s.v[187] * eq76_e1905_d_b1));
        let eq76_e1906_d_b2: f64 = ((s.db[187][2] * eq76_e1905) + (s.v[187] * eq76_e1905_d_b2));
        let eq76_e1906_d_b3: f64 = ((s.db[187][3] * eq76_e1905) + (s.v[187] * eq76_e1905_d_b3));
        let eq76_e1906_d_b4: f64 = ((s.db[187][4] * eq76_e1905) + (s.v[187] * eq76_e1905_d_b4));
        let eq76_e1906_d_b5: f64 = ((s.db[187][5] * eq76_e1905) + (s.v[187] * eq76_e1905_d_b5));
        let eq76_e1906_d_b6: f64 = ((s.db[187][6] * eq76_e1905) + (s.v[187] * eq76_e1905_d_b6));
        let eq76_e1906_d_b7: f64 = ((s.db[187][7] * eq76_e1905) + (s.v[187] * eq76_e1905_d_b7));
        let eq76_e1906_d_b8: f64 = ((s.db[187][8] * eq76_e1905) + (s.v[187] * eq76_e1905_d_b8));
        let eq76_e1906_d_b9: f64 = ((s.db[187][9] * eq76_e1905) + (s.v[187] * eq76_e1905_d_b9));
        let eq76_e1906_d_b10: f64 = ((s.db[187][10] * eq76_e1905) + (s.v[187] * eq76_e1905_d_b10));
        let eq76_e1906_d_b11: f64 = ((s.db[187][11] * eq76_e1905) + (s.v[187] * eq76_e1905_d_b11));
        let eq76_e1906_d_b12: f64 = ((s.db[187][12] * eq76_e1905) + (s.v[187] * eq76_e1905_d_b12));
        let eq76_e1906_d_b13: f64 = ((s.db[187][13] * eq76_e1905) + (s.v[187] * eq76_e1905_d_b13));
        (eq76_e1906, eq76_e1906_d_n0, eq76_e1906_d_n1, eq76_e1906_d_n2, eq76_e1906_d_n3, eq76_e1906_d_n4, eq76_e1906_d_n5, eq76_e1906_d_n6, eq76_e1906_d_n7, eq76_e1906_d_n8, eq76_e1906_d_n9, eq76_e1906_d_n10, eq76_e1906_d_n11, eq76_e1906_d_n12, eq76_e1906_d_n13, eq76_e1906_d_n14, eq76_e1906_d_n15, eq76_e1906_d_n16, eq76_e1906_d_b0, eq76_e1906_d_b1, eq76_e1906_d_b2, eq76_e1906_d_b3, eq76_e1906_d_b4, eq76_e1906_d_b5, eq76_e1906_d_b6, eq76_e1906_d_b7, eq76_e1906_d_b8, eq76_e1906_d_b9, eq76_e1906_d_b10, eq76_e1906_d_b11, eq76_e1906_d_b12, eq76_e1906_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_value: f64 = eq76_e1908;
        let eq76_node_derivatives: [f64; 17] = [eq76_e1908_d_n0, eq76_e1908_d_n1, eq76_e1908_d_n2, eq76_e1908_d_n3, eq76_e1908_d_n4, eq76_e1908_d_n5, eq76_e1908_d_n6, eq76_e1908_d_n7, eq76_e1908_d_n8, eq76_e1908_d_n9, eq76_e1908_d_n10, eq76_e1908_d_n11, eq76_e1908_d_n12, eq76_e1908_d_n13, eq76_e1908_d_n14, eq76_e1908_d_n15, eq76_e1908_d_n16];
        let eq76_branch_derivatives: [f64; 14] = [eq76_e1908_d_b0, eq76_e1908_d_b1, eq76_e1908_d_b2, eq76_e1908_d_b3, eq76_e1908_d_b4, eq76_e1908_d_b5, eq76_e1908_d_b6, eq76_e1908_d_b7, eq76_e1908_d_b8, eq76_e1908_d_b9, eq76_e1908_d_b10, eq76_e1908_d_b11, eq76_e1908_d_b12, eq76_e1908_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            multiplicity * (eq76_value),
            nodes,
            &eq76_node_derivatives,
            branches,
            &eq76_branch_derivatives,
            multiplicity,
        );
        let (eq77_e1918, eq77_e1918_d_n0, eq77_e1918_d_n1, eq77_e1918_d_n2, eq77_e1918_d_n3, eq77_e1918_d_n4, eq77_e1918_d_n5, eq77_e1918_d_n6, eq77_e1918_d_n7, eq77_e1918_d_n8, eq77_e1918_d_n9, eq77_e1918_d_n10, eq77_e1918_d_n11, eq77_e1918_d_n12, eq77_e1918_d_n13, eq77_e1918_d_n14, eq77_e1918_d_n15, eq77_e1918_d_n16, eq77_e1918_d_b0, eq77_e1918_d_b1, eq77_e1918_d_b2, eq77_e1918_d_b3, eq77_e1918_d_b4, eq77_e1918_d_b5, eq77_e1918_d_b6, eq77_e1918_d_b7, eq77_e1918_d_b8, eq77_e1918_d_b9, eq77_e1918_d_b10, eq77_e1918_d_b11, eq77_e1918_d_b12, eq77_e1918_d_b13,) = {
    if (!s.b[1627]) {
        let eq77_e1914: f64 = (p.p29 * s.v[334]);
        let eq77_e1914_d_n0: f64 = (p.p29 * s.dn[334][0]);
        let eq77_e1914_d_n1: f64 = (p.p29 * s.dn[334][1]);
        let eq77_e1914_d_n2: f64 = (p.p29 * s.dn[334][2]);
        let eq77_e1914_d_n3: f64 = (p.p29 * s.dn[334][3]);
        let eq77_e1914_d_n4: f64 = (p.p29 * s.dn[334][4]);
        let eq77_e1914_d_n5: f64 = (p.p29 * s.dn[334][5]);
        let eq77_e1914_d_n6: f64 = (p.p29 * s.dn[334][6]);
        let eq77_e1914_d_n7: f64 = (p.p29 * s.dn[334][7]);
        let eq77_e1914_d_n8: f64 = (p.p29 * s.dn[334][8]);
        let eq77_e1914_d_n9: f64 = (p.p29 * s.dn[334][9]);
        let eq77_e1914_d_n10: f64 = (p.p29 * s.dn[334][10]);
        let eq77_e1914_d_n11: f64 = (p.p29 * s.dn[334][11]);
        let eq77_e1914_d_n12: f64 = (p.p29 * s.dn[334][12]);
        let eq77_e1914_d_n13: f64 = (p.p29 * s.dn[334][13]);
        let eq77_e1914_d_n14: f64 = (p.p29 * s.dn[334][14]);
        let eq77_e1914_d_n15: f64 = (p.p29 * s.dn[334][15]);
        let eq77_e1914_d_n16: f64 = (p.p29 * s.dn[334][16]);
        let eq77_e1914_d_b0: f64 = (p.p29 * s.db[334][0]);
        let eq77_e1914_d_b1: f64 = (p.p29 * s.db[334][1]);
        let eq77_e1914_d_b2: f64 = (p.p29 * s.db[334][2]);
        let eq77_e1914_d_b3: f64 = (p.p29 * s.db[334][3]);
        let eq77_e1914_d_b4: f64 = (p.p29 * s.db[334][4]);
        let eq77_e1914_d_b5: f64 = (p.p29 * s.db[334][5]);
        let eq77_e1914_d_b6: f64 = (p.p29 * s.db[334][6]);
        let eq77_e1914_d_b7: f64 = (p.p29 * s.db[334][7]);
        let eq77_e1914_d_b8: f64 = (p.p29 * s.db[334][8]);
        let eq77_e1914_d_b9: f64 = (p.p29 * s.db[334][9]);
        let eq77_e1914_d_b10: f64 = (p.p29 * s.db[334][10]);
        let eq77_e1914_d_b11: f64 = (p.p29 * s.db[334][11]);
        let eq77_e1914_d_b12: f64 = (p.p29 * s.db[334][12]);
        let eq77_e1914_d_b13: f64 = (p.p29 * s.db[334][13]);
        let eq77_e1915: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 13, eq77_e1914);
        let eq77_e1915_d_n0: f64 = (eq77_e1914_d_n0 * ddt_scale);
        let eq77_e1915_d_n1: f64 = (eq77_e1914_d_n1 * ddt_scale);
        let eq77_e1915_d_n2: f64 = (eq77_e1914_d_n2 * ddt_scale);
        let eq77_e1915_d_n3: f64 = (eq77_e1914_d_n3 * ddt_scale);
        let eq77_e1915_d_n4: f64 = (eq77_e1914_d_n4 * ddt_scale);
        let eq77_e1915_d_n5: f64 = (eq77_e1914_d_n5 * ddt_scale);
        let eq77_e1915_d_n6: f64 = (eq77_e1914_d_n6 * ddt_scale);
        let eq77_e1915_d_n7: f64 = (eq77_e1914_d_n7 * ddt_scale);
        let eq77_e1915_d_n8: f64 = (eq77_e1914_d_n8 * ddt_scale);
        let eq77_e1915_d_n9: f64 = (eq77_e1914_d_n9 * ddt_scale);
        let eq77_e1915_d_n10: f64 = (eq77_e1914_d_n10 * ddt_scale);
        let eq77_e1915_d_n11: f64 = (eq77_e1914_d_n11 * ddt_scale);
        let eq77_e1915_d_n12: f64 = (eq77_e1914_d_n12 * ddt_scale);
        let eq77_e1915_d_n13: f64 = (eq77_e1914_d_n13 * ddt_scale);
        let eq77_e1915_d_n14: f64 = (eq77_e1914_d_n14 * ddt_scale);
        let eq77_e1915_d_n15: f64 = (eq77_e1914_d_n15 * ddt_scale);
        let eq77_e1915_d_n16: f64 = (eq77_e1914_d_n16 * ddt_scale);
        let eq77_e1915_d_b0: f64 = (eq77_e1914_d_b0 * ddt_scale);
        let eq77_e1915_d_b1: f64 = (eq77_e1914_d_b1 * ddt_scale);
        let eq77_e1915_d_b2: f64 = (eq77_e1914_d_b2 * ddt_scale);
        let eq77_e1915_d_b3: f64 = (eq77_e1914_d_b3 * ddt_scale);
        let eq77_e1915_d_b4: f64 = (eq77_e1914_d_b4 * ddt_scale);
        let eq77_e1915_d_b5: f64 = (eq77_e1914_d_b5 * ddt_scale);
        let eq77_e1915_d_b6: f64 = (eq77_e1914_d_b6 * ddt_scale);
        let eq77_e1915_d_b7: f64 = (eq77_e1914_d_b7 * ddt_scale);
        let eq77_e1915_d_b8: f64 = (eq77_e1914_d_b8 * ddt_scale);
        let eq77_e1915_d_b9: f64 = (eq77_e1914_d_b9 * ddt_scale);
        let eq77_e1915_d_b10: f64 = (eq77_e1914_d_b10 * ddt_scale);
        let eq77_e1915_d_b11: f64 = (eq77_e1914_d_b11 * ddt_scale);
        let eq77_e1915_d_b12: f64 = (eq77_e1914_d_b12 * ddt_scale);
        let eq77_e1915_d_b13: f64 = (eq77_e1914_d_b13 * ddt_scale);
        let eq77_e1916: f64 = (s.v[187] * eq77_e1915);
        let eq77_e1916_d_n0: f64 = ((s.dn[187][0] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n0));
        let eq77_e1916_d_n1: f64 = ((s.dn[187][1] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n1));
        let eq77_e1916_d_n2: f64 = ((s.dn[187][2] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n2));
        let eq77_e1916_d_n3: f64 = ((s.dn[187][3] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n3));
        let eq77_e1916_d_n4: f64 = ((s.dn[187][4] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n4));
        let eq77_e1916_d_n5: f64 = ((s.dn[187][5] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n5));
        let eq77_e1916_d_n6: f64 = ((s.dn[187][6] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n6));
        let eq77_e1916_d_n7: f64 = ((s.dn[187][7] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n7));
        let eq77_e1916_d_n8: f64 = ((s.dn[187][8] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n8));
        let eq77_e1916_d_n9: f64 = ((s.dn[187][9] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n9));
        let eq77_e1916_d_n10: f64 = ((s.dn[187][10] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n10));
        let eq77_e1916_d_n11: f64 = ((s.dn[187][11] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n11));
        let eq77_e1916_d_n12: f64 = ((s.dn[187][12] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n12));
        let eq77_e1916_d_n13: f64 = ((s.dn[187][13] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n13));
        let eq77_e1916_d_n14: f64 = ((s.dn[187][14] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n14));
        let eq77_e1916_d_n15: f64 = ((s.dn[187][15] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n15));
        let eq77_e1916_d_n16: f64 = ((s.dn[187][16] * eq77_e1915) + (s.v[187] * eq77_e1915_d_n16));
        let eq77_e1916_d_b0: f64 = ((s.db[187][0] * eq77_e1915) + (s.v[187] * eq77_e1915_d_b0));
        let eq77_e1916_d_b1: f64 = ((s.db[187][1] * eq77_e1915) + (s.v[187] * eq77_e1915_d_b1));
        let eq77_e1916_d_b2: f64 = ((s.db[187][2] * eq77_e1915) + (s.v[187] * eq77_e1915_d_b2));
        let eq77_e1916_d_b3: f64 = ((s.db[187][3] * eq77_e1915) + (s.v[187] * eq77_e1915_d_b3));
        let eq77_e1916_d_b4: f64 = ((s.db[187][4] * eq77_e1915) + (s.v[187] * eq77_e1915_d_b4));
        let eq77_e1916_d_b5: f64 = ((s.db[187][5] * eq77_e1915) + (s.v[187] * eq77_e1915_d_b5));
        let eq77_e1916_d_b6: f64 = ((s.db[187][6] * eq77_e1915) + (s.v[187] * eq77_e1915_d_b6));
        let eq77_e1916_d_b7: f64 = ((s.db[187][7] * eq77_e1915) + (s.v[187] * eq77_e1915_d_b7));
        let eq77_e1916_d_b8: f64 = ((s.db[187][8] * eq77_e1915) + (s.v[187] * eq77_e1915_d_b8));
        let eq77_e1916_d_b9: f64 = ((s.db[187][9] * eq77_e1915) + (s.v[187] * eq77_e1915_d_b9));
        let eq77_e1916_d_b10: f64 = ((s.db[187][10] * eq77_e1915) + (s.v[187] * eq77_e1915_d_b10));
        let eq77_e1916_d_b11: f64 = ((s.db[187][11] * eq77_e1915) + (s.v[187] * eq77_e1915_d_b11));
        let eq77_e1916_d_b12: f64 = ((s.db[187][12] * eq77_e1915) + (s.v[187] * eq77_e1915_d_b12));
        let eq77_e1916_d_b13: f64 = ((s.db[187][13] * eq77_e1915) + (s.v[187] * eq77_e1915_d_b13));
        (eq77_e1916, eq77_e1916_d_n0, eq77_e1916_d_n1, eq77_e1916_d_n2, eq77_e1916_d_n3, eq77_e1916_d_n4, eq77_e1916_d_n5, eq77_e1916_d_n6, eq77_e1916_d_n7, eq77_e1916_d_n8, eq77_e1916_d_n9, eq77_e1916_d_n10, eq77_e1916_d_n11, eq77_e1916_d_n12, eq77_e1916_d_n13, eq77_e1916_d_n14, eq77_e1916_d_n15, eq77_e1916_d_n16, eq77_e1916_d_b0, eq77_e1916_d_b1, eq77_e1916_d_b2, eq77_e1916_d_b3, eq77_e1916_d_b4, eq77_e1916_d_b5, eq77_e1916_d_b6, eq77_e1916_d_b7, eq77_e1916_d_b8, eq77_e1916_d_b9, eq77_e1916_d_b10, eq77_e1916_d_b11, eq77_e1916_d_b12, eq77_e1916_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_value: f64 = eq77_e1918;
        let eq77_node_derivatives: [f64; 17] = [eq77_e1918_d_n0, eq77_e1918_d_n1, eq77_e1918_d_n2, eq77_e1918_d_n3, eq77_e1918_d_n4, eq77_e1918_d_n5, eq77_e1918_d_n6, eq77_e1918_d_n7, eq77_e1918_d_n8, eq77_e1918_d_n9, eq77_e1918_d_n10, eq77_e1918_d_n11, eq77_e1918_d_n12, eq77_e1918_d_n13, eq77_e1918_d_n14, eq77_e1918_d_n15, eq77_e1918_d_n16];
        let eq77_branch_derivatives: [f64; 14] = [eq77_e1918_d_b0, eq77_e1918_d_b1, eq77_e1918_d_b2, eq77_e1918_d_b3, eq77_e1918_d_b4, eq77_e1918_d_b5, eq77_e1918_d_b6, eq77_e1918_d_b7, eq77_e1918_d_b8, eq77_e1918_d_b9, eq77_e1918_d_b10, eq77_e1918_d_b11, eq77_e1918_d_b12, eq77_e1918_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[5]),
            multiplicity * (eq77_value),
            nodes,
            &eq77_node_derivatives,
            branches,
            &eq77_branch_derivatives,
            multiplicity,
        );
        let (eq78_e1926, eq78_e1926_d_n0, eq78_e1926_d_n1, eq78_e1926_d_n2, eq78_e1926_d_n3, eq78_e1926_d_n4, eq78_e1926_d_n5, eq78_e1926_d_n6, eq78_e1926_d_n7, eq78_e1926_d_n8, eq78_e1926_d_n9, eq78_e1926_d_n10, eq78_e1926_d_n11, eq78_e1926_d_n12, eq78_e1926_d_n13, eq78_e1926_d_n14, eq78_e1926_d_n15, eq78_e1926_d_n16, eq78_e1926_d_b0, eq78_e1926_d_b1, eq78_e1926_d_b2, eq78_e1926_d_b3, eq78_e1926_d_b4, eq78_e1926_d_b5, eq78_e1926_d_b6, eq78_e1926_d_b7, eq78_e1926_d_b8, eq78_e1926_d_b9, eq78_e1926_d_b10, eq78_e1926_d_b11, eq78_e1926_d_b12, eq78_e1926_d_b13,) = {
    if s.b[1629] {
        let eq78_e1922: f64 = ((nv14 - nv0) * p.p28);
        let eq78_e1922_d_n0: f64 = (-p.p28);
        let eq78_e1922_d_n14: f64 = p.p28;
        let eq78_e1924: f64 = (eq78_e1922 * s.v[276]);
        let eq78_e1924_d_n0: f64 = ((eq78_e1922_d_n0 * s.v[276]) + (eq78_e1922 * s.dn[276][0]));
        let eq78_e1924_d_n1: f64 = (eq78_e1922 * s.dn[276][1]);
        let eq78_e1924_d_n2: f64 = (eq78_e1922 * s.dn[276][2]);
        let eq78_e1924_d_n3: f64 = (eq78_e1922 * s.dn[276][3]);
        let eq78_e1924_d_n4: f64 = (eq78_e1922 * s.dn[276][4]);
        let eq78_e1924_d_n5: f64 = (eq78_e1922 * s.dn[276][5]);
        let eq78_e1924_d_n6: f64 = (eq78_e1922 * s.dn[276][6]);
        let eq78_e1924_d_n7: f64 = (eq78_e1922 * s.dn[276][7]);
        let eq78_e1924_d_n8: f64 = (eq78_e1922 * s.dn[276][8]);
        let eq78_e1924_d_n9: f64 = (eq78_e1922 * s.dn[276][9]);
        let eq78_e1924_d_n10: f64 = (eq78_e1922 * s.dn[276][10]);
        let eq78_e1924_d_n11: f64 = (eq78_e1922 * s.dn[276][11]);
        let eq78_e1924_d_n12: f64 = (eq78_e1922 * s.dn[276][12]);
        let eq78_e1924_d_n13: f64 = (eq78_e1922 * s.dn[276][13]);
        let eq78_e1924_d_n14: f64 = ((eq78_e1922_d_n14 * s.v[276]) + (eq78_e1922 * s.dn[276][14]));
        let eq78_e1924_d_n15: f64 = (eq78_e1922 * s.dn[276][15]);
        let eq78_e1924_d_n16: f64 = (eq78_e1922 * s.dn[276][16]);
        let eq78_e1924_d_b0: f64 = (eq78_e1922 * s.db[276][0]);
        let eq78_e1924_d_b1: f64 = (eq78_e1922 * s.db[276][1]);
        let eq78_e1924_d_b2: f64 = (eq78_e1922 * s.db[276][2]);
        let eq78_e1924_d_b3: f64 = (eq78_e1922 * s.db[276][3]);
        let eq78_e1924_d_b4: f64 = (eq78_e1922 * s.db[276][4]);
        let eq78_e1924_d_b5: f64 = (eq78_e1922 * s.db[276][5]);
        let eq78_e1924_d_b6: f64 = (eq78_e1922 * s.db[276][6]);
        let eq78_e1924_d_b7: f64 = (eq78_e1922 * s.db[276][7]);
        let eq78_e1924_d_b8: f64 = (eq78_e1922 * s.db[276][8]);
        let eq78_e1924_d_b9: f64 = (eq78_e1922 * s.db[276][9]);
        let eq78_e1924_d_b10: f64 = (eq78_e1922 * s.db[276][10]);
        let eq78_e1924_d_b11: f64 = (eq78_e1922 * s.db[276][11]);
        let eq78_e1924_d_b12: f64 = (eq78_e1922 * s.db[276][12]);
        let eq78_e1924_d_b13: f64 = (eq78_e1922 * s.db[276][13]);
        (eq78_e1924, eq78_e1924_d_n0, eq78_e1924_d_n1, eq78_e1924_d_n2, eq78_e1924_d_n3, eq78_e1924_d_n4, eq78_e1924_d_n5, eq78_e1924_d_n6, eq78_e1924_d_n7, eq78_e1924_d_n8, eq78_e1924_d_n9, eq78_e1924_d_n10, eq78_e1924_d_n11, eq78_e1924_d_n12, eq78_e1924_d_n13, eq78_e1924_d_n14, eq78_e1924_d_n15, eq78_e1924_d_n16, eq78_e1924_d_b0, eq78_e1924_d_b1, eq78_e1924_d_b2, eq78_e1924_d_b3, eq78_e1924_d_b4, eq78_e1924_d_b5, eq78_e1924_d_b6, eq78_e1924_d_b7, eq78_e1924_d_b8, eq78_e1924_d_b9, eq78_e1924_d_b10, eq78_e1924_d_b11, eq78_e1924_d_b12, eq78_e1924_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq78_value: f64 = eq78_e1926;
        let eq78_node_derivatives: [f64; 17] = [eq78_e1926_d_n0, eq78_e1926_d_n1, eq78_e1926_d_n2, eq78_e1926_d_n3, eq78_e1926_d_n4, eq78_e1926_d_n5, eq78_e1926_d_n6, eq78_e1926_d_n7, eq78_e1926_d_n8, eq78_e1926_d_n9, eq78_e1926_d_n10, eq78_e1926_d_n11, eq78_e1926_d_n12, eq78_e1926_d_n13, eq78_e1926_d_n14, eq78_e1926_d_n15, eq78_e1926_d_n16];
        let eq78_branch_derivatives: [f64; 14] = [eq78_e1926_d_b0, eq78_e1926_d_b1, eq78_e1926_d_b2, eq78_e1926_d_b3, eq78_e1926_d_b4, eq78_e1926_d_b5, eq78_e1926_d_b6, eq78_e1926_d_b7, eq78_e1926_d_b8, eq78_e1926_d_b9, eq78_e1926_d_b10, eq78_e1926_d_b11, eq78_e1926_d_b12, eq78_e1926_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[14]),
            Some(nodes[0]),
            multiplicity * (eq78_value),
            nodes,
            &eq78_node_derivatives,
            branches,
            &eq78_branch_derivatives,
            multiplicity,
        );
        let (eq79_e1936,) = {
    if s.b[1629] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq79_value: f64 = eq79_e1936;
        stamper.stamp_current_const(
            Some(nodes[14]),
            Some(nodes[0]),
            multiplicity * (eq79_value),
        );
        let (eq80_e1941,) = {
    if (!s.b[1629]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq80_value: f64 = eq80_e1941;
        stamper.stamp_potential_const(
            branches[13],
            eq80_value,
        );
    }

    pub(super) fn stamp_transient_equations_block_9(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq81_e1959, eq81_e1959_d_n0, eq81_e1959_d_n1, eq81_e1959_d_n2, eq81_e1959_d_n3, eq81_e1959_d_n4, eq81_e1959_d_n5, eq81_e1959_d_n6, eq81_e1959_d_n7, eq81_e1959_d_n8, eq81_e1959_d_n9, eq81_e1959_d_n10, eq81_e1959_d_n11, eq81_e1959_d_n12, eq81_e1959_d_n13, eq81_e1959_d_n14, eq81_e1959_d_n15, eq81_e1959_d_n16, eq81_e1959_d_b0, eq81_e1959_d_b1, eq81_e1959_d_b2, eq81_e1959_d_b3, eq81_e1959_d_b4, eq81_e1959_d_b5, eq81_e1959_d_b6, eq81_e1959_d_b7, eq81_e1959_d_b8, eq81_e1959_d_b9, eq81_e1959_d_b10, eq81_e1959_d_b11, eq81_e1959_d_b12, eq81_e1959_d_b13,) = {
    if s.b[1630] {
        let eq81_e1945: f64 = (s.v[187] * p.p28);
        let eq81_e1945_d_n0: f64 = (s.dn[187][0] * p.p28);
        let eq81_e1945_d_n1: f64 = (s.dn[187][1] * p.p28);
        let eq81_e1945_d_n2: f64 = (s.dn[187][2] * p.p28);
        let eq81_e1945_d_n3: f64 = (s.dn[187][3] * p.p28);
        let eq81_e1945_d_n4: f64 = (s.dn[187][4] * p.p28);
        let eq81_e1945_d_n5: f64 = (s.dn[187][5] * p.p28);
        let eq81_e1945_d_n6: f64 = (s.dn[187][6] * p.p28);
        let eq81_e1945_d_n7: f64 = (s.dn[187][7] * p.p28);
        let eq81_e1945_d_n8: f64 = (s.dn[187][8] * p.p28);
        let eq81_e1945_d_n9: f64 = (s.dn[187][9] * p.p28);
        let eq81_e1945_d_n10: f64 = (s.dn[187][10] * p.p28);
        let eq81_e1945_d_n11: f64 = (s.dn[187][11] * p.p28);
        let eq81_e1945_d_n12: f64 = (s.dn[187][12] * p.p28);
        let eq81_e1945_d_n13: f64 = (s.dn[187][13] * p.p28);
        let eq81_e1945_d_n14: f64 = (s.dn[187][14] * p.p28);
        let eq81_e1945_d_n15: f64 = (s.dn[187][15] * p.p28);
        let eq81_e1945_d_n16: f64 = (s.dn[187][16] * p.p28);
        let eq81_e1945_d_b0: f64 = (s.db[187][0] * p.p28);
        let eq81_e1945_d_b1: f64 = (s.db[187][1] * p.p28);
        let eq81_e1945_d_b2: f64 = (s.db[187][2] * p.p28);
        let eq81_e1945_d_b3: f64 = (s.db[187][3] * p.p28);
        let eq81_e1945_d_b4: f64 = (s.db[187][4] * p.p28);
        let eq81_e1945_d_b5: f64 = (s.db[187][5] * p.p28);
        let eq81_e1945_d_b6: f64 = (s.db[187][6] * p.p28);
        let eq81_e1945_d_b7: f64 = (s.db[187][7] * p.p28);
        let eq81_e1945_d_b8: f64 = (s.db[187][8] * p.p28);
        let eq81_e1945_d_b9: f64 = (s.db[187][9] * p.p28);
        let eq81_e1945_d_b10: f64 = (s.db[187][10] * p.p28);
        let eq81_e1945_d_b11: f64 = (s.db[187][11] * p.p28);
        let eq81_e1945_d_b12: f64 = (s.db[187][12] * p.p28);
        let eq81_e1945_d_b13: f64 = (s.db[187][13] * p.p28);
        let eq81_e1947: f64 = (eq81_e1945 * s.v[304]);
        let eq81_e1947_d_n0: f64 = ((eq81_e1945_d_n0 * s.v[304]) + (eq81_e1945 * s.dn[304][0]));
        let eq81_e1947_d_n1: f64 = ((eq81_e1945_d_n1 * s.v[304]) + (eq81_e1945 * s.dn[304][1]));
        let eq81_e1947_d_n2: f64 = ((eq81_e1945_d_n2 * s.v[304]) + (eq81_e1945 * s.dn[304][2]));
        let eq81_e1947_d_n3: f64 = ((eq81_e1945_d_n3 * s.v[304]) + (eq81_e1945 * s.dn[304][3]));
        let eq81_e1947_d_n4: f64 = ((eq81_e1945_d_n4 * s.v[304]) + (eq81_e1945 * s.dn[304][4]));
        let eq81_e1947_d_n5: f64 = ((eq81_e1945_d_n5 * s.v[304]) + (eq81_e1945 * s.dn[304][5]));
        let eq81_e1947_d_n6: f64 = ((eq81_e1945_d_n6 * s.v[304]) + (eq81_e1945 * s.dn[304][6]));
        let eq81_e1947_d_n7: f64 = ((eq81_e1945_d_n7 * s.v[304]) + (eq81_e1945 * s.dn[304][7]));
        let eq81_e1947_d_n8: f64 = ((eq81_e1945_d_n8 * s.v[304]) + (eq81_e1945 * s.dn[304][8]));
        let eq81_e1947_d_n9: f64 = ((eq81_e1945_d_n9 * s.v[304]) + (eq81_e1945 * s.dn[304][9]));
        let eq81_e1947_d_n10: f64 = ((eq81_e1945_d_n10 * s.v[304]) + (eq81_e1945 * s.dn[304][10]));
        let eq81_e1947_d_n11: f64 = ((eq81_e1945_d_n11 * s.v[304]) + (eq81_e1945 * s.dn[304][11]));
        let eq81_e1947_d_n12: f64 = ((eq81_e1945_d_n12 * s.v[304]) + (eq81_e1945 * s.dn[304][12]));
        let eq81_e1947_d_n13: f64 = ((eq81_e1945_d_n13 * s.v[304]) + (eq81_e1945 * s.dn[304][13]));
        let eq81_e1947_d_n14: f64 = ((eq81_e1945_d_n14 * s.v[304]) + (eq81_e1945 * s.dn[304][14]));
        let eq81_e1947_d_n15: f64 = ((eq81_e1945_d_n15 * s.v[304]) + (eq81_e1945 * s.dn[304][15]));
        let eq81_e1947_d_n16: f64 = ((eq81_e1945_d_n16 * s.v[304]) + (eq81_e1945 * s.dn[304][16]));
        let eq81_e1947_d_b0: f64 = ((eq81_e1945_d_b0 * s.v[304]) + (eq81_e1945 * s.db[304][0]));
        let eq81_e1947_d_b1: f64 = ((eq81_e1945_d_b1 * s.v[304]) + (eq81_e1945 * s.db[304][1]));
        let eq81_e1947_d_b2: f64 = ((eq81_e1945_d_b2 * s.v[304]) + (eq81_e1945 * s.db[304][2]));
        let eq81_e1947_d_b3: f64 = ((eq81_e1945_d_b3 * s.v[304]) + (eq81_e1945 * s.db[304][3]));
        let eq81_e1947_d_b4: f64 = ((eq81_e1945_d_b4 * s.v[304]) + (eq81_e1945 * s.db[304][4]));
        let eq81_e1947_d_b5: f64 = ((eq81_e1945_d_b5 * s.v[304]) + (eq81_e1945 * s.db[304][5]));
        let eq81_e1947_d_b6: f64 = ((eq81_e1945_d_b6 * s.v[304]) + (eq81_e1945 * s.db[304][6]));
        let eq81_e1947_d_b7: f64 = ((eq81_e1945_d_b7 * s.v[304]) + (eq81_e1945 * s.db[304][7]));
        let eq81_e1947_d_b8: f64 = ((eq81_e1945_d_b8 * s.v[304]) + (eq81_e1945 * s.db[304][8]));
        let eq81_e1947_d_b9: f64 = ((eq81_e1945_d_b9 * s.v[304]) + (eq81_e1945 * s.db[304][9]));
        let eq81_e1947_d_b10: f64 = ((eq81_e1945_d_b10 * s.v[304]) + (eq81_e1945 * s.db[304][10]));
        let eq81_e1947_d_b11: f64 = ((eq81_e1945_d_b11 * s.v[304]) + (eq81_e1945 * s.db[304][11]));
        let eq81_e1947_d_b12: f64 = ((eq81_e1945_d_b12 * s.v[304]) + (eq81_e1945 * s.db[304][12]));
        let eq81_e1947_d_b13: f64 = ((eq81_e1945_d_b13 * s.v[304]) + (eq81_e1945 * s.db[304][13]));
        let eq81_e1950: f64 = (1.0 - p.p1128);
        let eq81_e1952: f64 = (eq81_e1950 * p.p28);
        let eq81_e1954: f64 = (eq81_e1952 * (nv13 - nv5));
        let eq81_e1954_d_n5: f64 = (-eq81_e1952);
        let eq81_e1956: f64 = (eq81_e1954 * s.v[781]);
        let eq81_e1956_d_n5: f64 = (eq81_e1954_d_n5 * s.v[781]);
        let eq81_e1956_d_n13: f64 = (eq81_e1952 * s.v[781]);
        let eq81_e1957: f64 = (eq81_e1947 + eq81_e1956);
        let eq81_e1957_d_n5: f64 = (eq81_e1947_d_n5 + eq81_e1956_d_n5);
        let eq81_e1957_d_n13: f64 = (eq81_e1947_d_n13 + eq81_e1956_d_n13);
        (eq81_e1957, eq81_e1947_d_n0, eq81_e1947_d_n1, eq81_e1947_d_n2, eq81_e1947_d_n3, eq81_e1947_d_n4, eq81_e1957_d_n5, eq81_e1947_d_n6, eq81_e1947_d_n7, eq81_e1947_d_n8, eq81_e1947_d_n9, eq81_e1947_d_n10, eq81_e1947_d_n11, eq81_e1947_d_n12, eq81_e1957_d_n13, eq81_e1947_d_n14, eq81_e1947_d_n15, eq81_e1947_d_n16, eq81_e1947_d_b0, eq81_e1947_d_b1, eq81_e1947_d_b2, eq81_e1947_d_b3, eq81_e1947_d_b4, eq81_e1947_d_b5, eq81_e1947_d_b6, eq81_e1947_d_b7, eq81_e1947_d_b8, eq81_e1947_d_b9, eq81_e1947_d_b10, eq81_e1947_d_b11, eq81_e1947_d_b12, eq81_e1947_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq81_value: f64 = eq81_e1959;
        let eq81_node_derivatives: [f64; 17] = [eq81_e1959_d_n0, eq81_e1959_d_n1, eq81_e1959_d_n2, eq81_e1959_d_n3, eq81_e1959_d_n4, eq81_e1959_d_n5, eq81_e1959_d_n6, eq81_e1959_d_n7, eq81_e1959_d_n8, eq81_e1959_d_n9, eq81_e1959_d_n10, eq81_e1959_d_n11, eq81_e1959_d_n12, eq81_e1959_d_n13, eq81_e1959_d_n14, eq81_e1959_d_n15, eq81_e1959_d_n16];
        let eq81_branch_derivatives: [f64; 14] = [eq81_e1959_d_b0, eq81_e1959_d_b1, eq81_e1959_d_b2, eq81_e1959_d_b3, eq81_e1959_d_b4, eq81_e1959_d_b5, eq81_e1959_d_b6, eq81_e1959_d_b7, eq81_e1959_d_b8, eq81_e1959_d_b9, eq81_e1959_d_b10, eq81_e1959_d_b11, eq81_e1959_d_b12, eq81_e1959_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            Some(nodes[5]),
            multiplicity * (eq81_value),
            nodes,
            &eq81_node_derivatives,
            branches,
            &eq81_branch_derivatives,
            multiplicity,
        );
        let (eq82_e1975, eq82_e1975_d_n0, eq82_e1975_d_n1, eq82_e1975_d_n2, eq82_e1975_d_n3, eq82_e1975_d_n4, eq82_e1975_d_n5, eq82_e1975_d_n6, eq82_e1975_d_n7, eq82_e1975_d_n8, eq82_e1975_d_n9, eq82_e1975_d_n10, eq82_e1975_d_n11, eq82_e1975_d_n12, eq82_e1975_d_n13, eq82_e1975_d_n14, eq82_e1975_d_n15, eq82_e1975_d_n16, eq82_e1975_d_b0, eq82_e1975_d_b1, eq82_e1975_d_b2, eq82_e1975_d_b3, eq82_e1975_d_b4, eq82_e1975_d_b5, eq82_e1975_d_b6, eq82_e1975_d_b7, eq82_e1975_d_b8, eq82_e1975_d_b9, eq82_e1975_d_b10, eq82_e1975_d_b11, eq82_e1975_d_b12, eq82_e1975_d_b13,) = {
    if s.b[1630] {
        let eq82_e1963: f64 = (s.v[187] * p.p28);
        let eq82_e1963_d_n0: f64 = (s.dn[187][0] * p.p28);
        let eq82_e1963_d_n1: f64 = (s.dn[187][1] * p.p28);
        let eq82_e1963_d_n2: f64 = (s.dn[187][2] * p.p28);
        let eq82_e1963_d_n3: f64 = (s.dn[187][3] * p.p28);
        let eq82_e1963_d_n4: f64 = (s.dn[187][4] * p.p28);
        let eq82_e1963_d_n5: f64 = (s.dn[187][5] * p.p28);
        let eq82_e1963_d_n6: f64 = (s.dn[187][6] * p.p28);
        let eq82_e1963_d_n7: f64 = (s.dn[187][7] * p.p28);
        let eq82_e1963_d_n8: f64 = (s.dn[187][8] * p.p28);
        let eq82_e1963_d_n9: f64 = (s.dn[187][9] * p.p28);
        let eq82_e1963_d_n10: f64 = (s.dn[187][10] * p.p28);
        let eq82_e1963_d_n11: f64 = (s.dn[187][11] * p.p28);
        let eq82_e1963_d_n12: f64 = (s.dn[187][12] * p.p28);
        let eq82_e1963_d_n13: f64 = (s.dn[187][13] * p.p28);
        let eq82_e1963_d_n14: f64 = (s.dn[187][14] * p.p28);
        let eq82_e1963_d_n15: f64 = (s.dn[187][15] * p.p28);
        let eq82_e1963_d_n16: f64 = (s.dn[187][16] * p.p28);
        let eq82_e1963_d_b0: f64 = (s.db[187][0] * p.p28);
        let eq82_e1963_d_b1: f64 = (s.db[187][1] * p.p28);
        let eq82_e1963_d_b2: f64 = (s.db[187][2] * p.p28);
        let eq82_e1963_d_b3: f64 = (s.db[187][3] * p.p28);
        let eq82_e1963_d_b4: f64 = (s.db[187][4] * p.p28);
        let eq82_e1963_d_b5: f64 = (s.db[187][5] * p.p28);
        let eq82_e1963_d_b6: f64 = (s.db[187][6] * p.p28);
        let eq82_e1963_d_b7: f64 = (s.db[187][7] * p.p28);
        let eq82_e1963_d_b8: f64 = (s.db[187][8] * p.p28);
        let eq82_e1963_d_b9: f64 = (s.db[187][9] * p.p28);
        let eq82_e1963_d_b10: f64 = (s.db[187][10] * p.p28);
        let eq82_e1963_d_b11: f64 = (s.db[187][11] * p.p28);
        let eq82_e1963_d_b12: f64 = (s.db[187][12] * p.p28);
        let eq82_e1963_d_b13: f64 = (s.db[187][13] * p.p28);
        let eq82_e1965: f64 = (eq82_e1963 * s.v[305]);
        let eq82_e1965_d_n0: f64 = ((eq82_e1963_d_n0 * s.v[305]) + (eq82_e1963 * s.dn[305][0]));
        let eq82_e1965_d_n1: f64 = ((eq82_e1963_d_n1 * s.v[305]) + (eq82_e1963 * s.dn[305][1]));
        let eq82_e1965_d_n2: f64 = ((eq82_e1963_d_n2 * s.v[305]) + (eq82_e1963 * s.dn[305][2]));
        let eq82_e1965_d_n3: f64 = ((eq82_e1963_d_n3 * s.v[305]) + (eq82_e1963 * s.dn[305][3]));
        let eq82_e1965_d_n4: f64 = ((eq82_e1963_d_n4 * s.v[305]) + (eq82_e1963 * s.dn[305][4]));
        let eq82_e1965_d_n5: f64 = ((eq82_e1963_d_n5 * s.v[305]) + (eq82_e1963 * s.dn[305][5]));
        let eq82_e1965_d_n6: f64 = ((eq82_e1963_d_n6 * s.v[305]) + (eq82_e1963 * s.dn[305][6]));
        let eq82_e1965_d_n7: f64 = ((eq82_e1963_d_n7 * s.v[305]) + (eq82_e1963 * s.dn[305][7]));
        let eq82_e1965_d_n8: f64 = ((eq82_e1963_d_n8 * s.v[305]) + (eq82_e1963 * s.dn[305][8]));
        let eq82_e1965_d_n9: f64 = ((eq82_e1963_d_n9 * s.v[305]) + (eq82_e1963 * s.dn[305][9]));
        let eq82_e1965_d_n10: f64 = ((eq82_e1963_d_n10 * s.v[305]) + (eq82_e1963 * s.dn[305][10]));
        let eq82_e1965_d_n11: f64 = ((eq82_e1963_d_n11 * s.v[305]) + (eq82_e1963 * s.dn[305][11]));
        let eq82_e1965_d_n12: f64 = ((eq82_e1963_d_n12 * s.v[305]) + (eq82_e1963 * s.dn[305][12]));
        let eq82_e1965_d_n13: f64 = ((eq82_e1963_d_n13 * s.v[305]) + (eq82_e1963 * s.dn[305][13]));
        let eq82_e1965_d_n14: f64 = ((eq82_e1963_d_n14 * s.v[305]) + (eq82_e1963 * s.dn[305][14]));
        let eq82_e1965_d_n15: f64 = ((eq82_e1963_d_n15 * s.v[305]) + (eq82_e1963 * s.dn[305][15]));
        let eq82_e1965_d_n16: f64 = ((eq82_e1963_d_n16 * s.v[305]) + (eq82_e1963 * s.dn[305][16]));
        let eq82_e1965_d_b0: f64 = ((eq82_e1963_d_b0 * s.v[305]) + (eq82_e1963 * s.db[305][0]));
        let eq82_e1965_d_b1: f64 = ((eq82_e1963_d_b1 * s.v[305]) + (eq82_e1963 * s.db[305][1]));
        let eq82_e1965_d_b2: f64 = ((eq82_e1963_d_b2 * s.v[305]) + (eq82_e1963 * s.db[305][2]));
        let eq82_e1965_d_b3: f64 = ((eq82_e1963_d_b3 * s.v[305]) + (eq82_e1963 * s.db[305][3]));
        let eq82_e1965_d_b4: f64 = ((eq82_e1963_d_b4 * s.v[305]) + (eq82_e1963 * s.db[305][4]));
        let eq82_e1965_d_b5: f64 = ((eq82_e1963_d_b5 * s.v[305]) + (eq82_e1963 * s.db[305][5]));
        let eq82_e1965_d_b6: f64 = ((eq82_e1963_d_b6 * s.v[305]) + (eq82_e1963 * s.db[305][6]));
        let eq82_e1965_d_b7: f64 = ((eq82_e1963_d_b7 * s.v[305]) + (eq82_e1963 * s.db[305][7]));
        let eq82_e1965_d_b8: f64 = ((eq82_e1963_d_b8 * s.v[305]) + (eq82_e1963 * s.db[305][8]));
        let eq82_e1965_d_b9: f64 = ((eq82_e1963_d_b9 * s.v[305]) + (eq82_e1963 * s.db[305][9]));
        let eq82_e1965_d_b10: f64 = ((eq82_e1963_d_b10 * s.v[305]) + (eq82_e1963 * s.db[305][10]));
        let eq82_e1965_d_b11: f64 = ((eq82_e1963_d_b11 * s.v[305]) + (eq82_e1963 * s.db[305][11]));
        let eq82_e1965_d_b12: f64 = ((eq82_e1963_d_b12 * s.v[305]) + (eq82_e1963 * s.db[305][12]));
        let eq82_e1965_d_b13: f64 = ((eq82_e1963_d_b13 * s.v[305]) + (eq82_e1963 * s.db[305][13]));
        let eq82_e1968: f64 = (p.p1128 * p.p28);
        let eq82_e1970: f64 = (eq82_e1968 * (nv13 - nv14));
        let eq82_e1970_d_n14: f64 = (-eq82_e1968);
        let eq82_e1972: f64 = (eq82_e1970 * s.v[781]);
        let eq82_e1972_d_n13: f64 = (eq82_e1968 * s.v[781]);
        let eq82_e1972_d_n14: f64 = (eq82_e1970_d_n14 * s.v[781]);
        let eq82_e1973: f64 = (eq82_e1965 + eq82_e1972);
        let eq82_e1973_d_n13: f64 = (eq82_e1965_d_n13 + eq82_e1972_d_n13);
        let eq82_e1973_d_n14: f64 = (eq82_e1965_d_n14 + eq82_e1972_d_n14);
        (eq82_e1973, eq82_e1965_d_n0, eq82_e1965_d_n1, eq82_e1965_d_n2, eq82_e1965_d_n3, eq82_e1965_d_n4, eq82_e1965_d_n5, eq82_e1965_d_n6, eq82_e1965_d_n7, eq82_e1965_d_n8, eq82_e1965_d_n9, eq82_e1965_d_n10, eq82_e1965_d_n11, eq82_e1965_d_n12, eq82_e1973_d_n13, eq82_e1973_d_n14, eq82_e1965_d_n15, eq82_e1965_d_n16, eq82_e1965_d_b0, eq82_e1965_d_b1, eq82_e1965_d_b2, eq82_e1965_d_b3, eq82_e1965_d_b4, eq82_e1965_d_b5, eq82_e1965_d_b6, eq82_e1965_d_b7, eq82_e1965_d_b8, eq82_e1965_d_b9, eq82_e1965_d_b10, eq82_e1965_d_b11, eq82_e1965_d_b12, eq82_e1965_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq82_value: f64 = eq82_e1975;
        let eq82_node_derivatives: [f64; 17] = [eq82_e1975_d_n0, eq82_e1975_d_n1, eq82_e1975_d_n2, eq82_e1975_d_n3, eq82_e1975_d_n4, eq82_e1975_d_n5, eq82_e1975_d_n6, eq82_e1975_d_n7, eq82_e1975_d_n8, eq82_e1975_d_n9, eq82_e1975_d_n10, eq82_e1975_d_n11, eq82_e1975_d_n12, eq82_e1975_d_n13, eq82_e1975_d_n14, eq82_e1975_d_n15, eq82_e1975_d_n16];
        let eq82_branch_derivatives: [f64; 14] = [eq82_e1975_d_b0, eq82_e1975_d_b1, eq82_e1975_d_b2, eq82_e1975_d_b3, eq82_e1975_d_b4, eq82_e1975_d_b5, eq82_e1975_d_b6, eq82_e1975_d_b7, eq82_e1975_d_b8, eq82_e1975_d_b9, eq82_e1975_d_b10, eq82_e1975_d_b11, eq82_e1975_d_b12, eq82_e1975_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            Some(nodes[14]),
            multiplicity * (eq82_value),
            nodes,
            &eq82_node_derivatives,
            branches,
            &eq82_branch_derivatives,
            multiplicity,
        );
        let (eq83_e1984, eq83_e1984_d_n0, eq83_e1984_d_n1, eq83_e1984_d_n2, eq83_e1984_d_n3, eq83_e1984_d_n4, eq83_e1984_d_n5, eq83_e1984_d_n6, eq83_e1984_d_n7, eq83_e1984_d_n8, eq83_e1984_d_n9, eq83_e1984_d_n10, eq83_e1984_d_n11, eq83_e1984_d_n12, eq83_e1984_d_n13, eq83_e1984_d_n14, eq83_e1984_d_n15, eq83_e1984_d_n16, eq83_e1984_d_b0, eq83_e1984_d_b1, eq83_e1984_d_b2, eq83_e1984_d_b3, eq83_e1984_d_b4, eq83_e1984_d_b5, eq83_e1984_d_b6, eq83_e1984_d_b7, eq83_e1984_d_b8, eq83_e1984_d_b9, eq83_e1984_d_b10, eq83_e1984_d_b11, eq83_e1984_d_b12, eq83_e1984_d_b13,) = {
    if s.b[1630] {
        let eq83_e1980: f64 = (p.p29 * s.v[334]);
        let eq83_e1980_d_n0: f64 = (p.p29 * s.dn[334][0]);
        let eq83_e1980_d_n1: f64 = (p.p29 * s.dn[334][1]);
        let eq83_e1980_d_n2: f64 = (p.p29 * s.dn[334][2]);
        let eq83_e1980_d_n3: f64 = (p.p29 * s.dn[334][3]);
        let eq83_e1980_d_n4: f64 = (p.p29 * s.dn[334][4]);
        let eq83_e1980_d_n5: f64 = (p.p29 * s.dn[334][5]);
        let eq83_e1980_d_n6: f64 = (p.p29 * s.dn[334][6]);
        let eq83_e1980_d_n7: f64 = (p.p29 * s.dn[334][7]);
        let eq83_e1980_d_n8: f64 = (p.p29 * s.dn[334][8]);
        let eq83_e1980_d_n9: f64 = (p.p29 * s.dn[334][9]);
        let eq83_e1980_d_n10: f64 = (p.p29 * s.dn[334][10]);
        let eq83_e1980_d_n11: f64 = (p.p29 * s.dn[334][11]);
        let eq83_e1980_d_n12: f64 = (p.p29 * s.dn[334][12]);
        let eq83_e1980_d_n13: f64 = (p.p29 * s.dn[334][13]);
        let eq83_e1980_d_n14: f64 = (p.p29 * s.dn[334][14]);
        let eq83_e1980_d_n15: f64 = (p.p29 * s.dn[334][15]);
        let eq83_e1980_d_n16: f64 = (p.p29 * s.dn[334][16]);
        let eq83_e1980_d_b0: f64 = (p.p29 * s.db[334][0]);
        let eq83_e1980_d_b1: f64 = (p.p29 * s.db[334][1]);
        let eq83_e1980_d_b2: f64 = (p.p29 * s.db[334][2]);
        let eq83_e1980_d_b3: f64 = (p.p29 * s.db[334][3]);
        let eq83_e1980_d_b4: f64 = (p.p29 * s.db[334][4]);
        let eq83_e1980_d_b5: f64 = (p.p29 * s.db[334][5]);
        let eq83_e1980_d_b6: f64 = (p.p29 * s.db[334][6]);
        let eq83_e1980_d_b7: f64 = (p.p29 * s.db[334][7]);
        let eq83_e1980_d_b8: f64 = (p.p29 * s.db[334][8]);
        let eq83_e1980_d_b9: f64 = (p.p29 * s.db[334][9]);
        let eq83_e1980_d_b10: f64 = (p.p29 * s.db[334][10]);
        let eq83_e1980_d_b11: f64 = (p.p29 * s.db[334][11]);
        let eq83_e1980_d_b12: f64 = (p.p29 * s.db[334][12]);
        let eq83_e1980_d_b13: f64 = (p.p29 * s.db[334][13]);
        let eq83_e1981: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 14, eq83_e1980);
        let eq83_e1981_d_n0: f64 = (eq83_e1980_d_n0 * ddt_scale);
        let eq83_e1981_d_n1: f64 = (eq83_e1980_d_n1 * ddt_scale);
        let eq83_e1981_d_n2: f64 = (eq83_e1980_d_n2 * ddt_scale);
        let eq83_e1981_d_n3: f64 = (eq83_e1980_d_n3 * ddt_scale);
        let eq83_e1981_d_n4: f64 = (eq83_e1980_d_n4 * ddt_scale);
        let eq83_e1981_d_n5: f64 = (eq83_e1980_d_n5 * ddt_scale);
        let eq83_e1981_d_n6: f64 = (eq83_e1980_d_n6 * ddt_scale);
        let eq83_e1981_d_n7: f64 = (eq83_e1980_d_n7 * ddt_scale);
        let eq83_e1981_d_n8: f64 = (eq83_e1980_d_n8 * ddt_scale);
        let eq83_e1981_d_n9: f64 = (eq83_e1980_d_n9 * ddt_scale);
        let eq83_e1981_d_n10: f64 = (eq83_e1980_d_n10 * ddt_scale);
        let eq83_e1981_d_n11: f64 = (eq83_e1980_d_n11 * ddt_scale);
        let eq83_e1981_d_n12: f64 = (eq83_e1980_d_n12 * ddt_scale);
        let eq83_e1981_d_n13: f64 = (eq83_e1980_d_n13 * ddt_scale);
        let eq83_e1981_d_n14: f64 = (eq83_e1980_d_n14 * ddt_scale);
        let eq83_e1981_d_n15: f64 = (eq83_e1980_d_n15 * ddt_scale);
        let eq83_e1981_d_n16: f64 = (eq83_e1980_d_n16 * ddt_scale);
        let eq83_e1981_d_b0: f64 = (eq83_e1980_d_b0 * ddt_scale);
        let eq83_e1981_d_b1: f64 = (eq83_e1980_d_b1 * ddt_scale);
        let eq83_e1981_d_b2: f64 = (eq83_e1980_d_b2 * ddt_scale);
        let eq83_e1981_d_b3: f64 = (eq83_e1980_d_b3 * ddt_scale);
        let eq83_e1981_d_b4: f64 = (eq83_e1980_d_b4 * ddt_scale);
        let eq83_e1981_d_b5: f64 = (eq83_e1980_d_b5 * ddt_scale);
        let eq83_e1981_d_b6: f64 = (eq83_e1980_d_b6 * ddt_scale);
        let eq83_e1981_d_b7: f64 = (eq83_e1980_d_b7 * ddt_scale);
        let eq83_e1981_d_b8: f64 = (eq83_e1980_d_b8 * ddt_scale);
        let eq83_e1981_d_b9: f64 = (eq83_e1980_d_b9 * ddt_scale);
        let eq83_e1981_d_b10: f64 = (eq83_e1980_d_b10 * ddt_scale);
        let eq83_e1981_d_b11: f64 = (eq83_e1980_d_b11 * ddt_scale);
        let eq83_e1981_d_b12: f64 = (eq83_e1980_d_b12 * ddt_scale);
        let eq83_e1981_d_b13: f64 = (eq83_e1980_d_b13 * ddt_scale);
        let eq83_e1982: f64 = (s.v[187] * eq83_e1981);
        let eq83_e1982_d_n0: f64 = ((s.dn[187][0] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n0));
        let eq83_e1982_d_n1: f64 = ((s.dn[187][1] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n1));
        let eq83_e1982_d_n2: f64 = ((s.dn[187][2] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n2));
        let eq83_e1982_d_n3: f64 = ((s.dn[187][3] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n3));
        let eq83_e1982_d_n4: f64 = ((s.dn[187][4] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n4));
        let eq83_e1982_d_n5: f64 = ((s.dn[187][5] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n5));
        let eq83_e1982_d_n6: f64 = ((s.dn[187][6] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n6));
        let eq83_e1982_d_n7: f64 = ((s.dn[187][7] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n7));
        let eq83_e1982_d_n8: f64 = ((s.dn[187][8] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n8));
        let eq83_e1982_d_n9: f64 = ((s.dn[187][9] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n9));
        let eq83_e1982_d_n10: f64 = ((s.dn[187][10] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n10));
        let eq83_e1982_d_n11: f64 = ((s.dn[187][11] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n11));
        let eq83_e1982_d_n12: f64 = ((s.dn[187][12] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n12));
        let eq83_e1982_d_n13: f64 = ((s.dn[187][13] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n13));
        let eq83_e1982_d_n14: f64 = ((s.dn[187][14] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n14));
        let eq83_e1982_d_n15: f64 = ((s.dn[187][15] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n15));
        let eq83_e1982_d_n16: f64 = ((s.dn[187][16] * eq83_e1981) + (s.v[187] * eq83_e1981_d_n16));
        let eq83_e1982_d_b0: f64 = ((s.db[187][0] * eq83_e1981) + (s.v[187] * eq83_e1981_d_b0));
        let eq83_e1982_d_b1: f64 = ((s.db[187][1] * eq83_e1981) + (s.v[187] * eq83_e1981_d_b1));
        let eq83_e1982_d_b2: f64 = ((s.db[187][2] * eq83_e1981) + (s.v[187] * eq83_e1981_d_b2));
        let eq83_e1982_d_b3: f64 = ((s.db[187][3] * eq83_e1981) + (s.v[187] * eq83_e1981_d_b3));
        let eq83_e1982_d_b4: f64 = ((s.db[187][4] * eq83_e1981) + (s.v[187] * eq83_e1981_d_b4));
        let eq83_e1982_d_b5: f64 = ((s.db[187][5] * eq83_e1981) + (s.v[187] * eq83_e1981_d_b5));
        let eq83_e1982_d_b6: f64 = ((s.db[187][6] * eq83_e1981) + (s.v[187] * eq83_e1981_d_b6));
        let eq83_e1982_d_b7: f64 = ((s.db[187][7] * eq83_e1981) + (s.v[187] * eq83_e1981_d_b7));
        let eq83_e1982_d_b8: f64 = ((s.db[187][8] * eq83_e1981) + (s.v[187] * eq83_e1981_d_b8));
        let eq83_e1982_d_b9: f64 = ((s.db[187][9] * eq83_e1981) + (s.v[187] * eq83_e1981_d_b9));
        let eq83_e1982_d_b10: f64 = ((s.db[187][10] * eq83_e1981) + (s.v[187] * eq83_e1981_d_b10));
        let eq83_e1982_d_b11: f64 = ((s.db[187][11] * eq83_e1981) + (s.v[187] * eq83_e1981_d_b11));
        let eq83_e1982_d_b12: f64 = ((s.db[187][12] * eq83_e1981) + (s.v[187] * eq83_e1981_d_b12));
        let eq83_e1982_d_b13: f64 = ((s.db[187][13] * eq83_e1981) + (s.v[187] * eq83_e1981_d_b13));
        (eq83_e1982, eq83_e1982_d_n0, eq83_e1982_d_n1, eq83_e1982_d_n2, eq83_e1982_d_n3, eq83_e1982_d_n4, eq83_e1982_d_n5, eq83_e1982_d_n6, eq83_e1982_d_n7, eq83_e1982_d_n8, eq83_e1982_d_n9, eq83_e1982_d_n10, eq83_e1982_d_n11, eq83_e1982_d_n12, eq83_e1982_d_n13, eq83_e1982_d_n14, eq83_e1982_d_n15, eq83_e1982_d_n16, eq83_e1982_d_b0, eq83_e1982_d_b1, eq83_e1982_d_b2, eq83_e1982_d_b3, eq83_e1982_d_b4, eq83_e1982_d_b5, eq83_e1982_d_b6, eq83_e1982_d_b7, eq83_e1982_d_b8, eq83_e1982_d_b9, eq83_e1982_d_b10, eq83_e1982_d_b11, eq83_e1982_d_b12, eq83_e1982_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq83_value: f64 = eq83_e1984;
        let eq83_node_derivatives: [f64; 17] = [eq83_e1984_d_n0, eq83_e1984_d_n1, eq83_e1984_d_n2, eq83_e1984_d_n3, eq83_e1984_d_n4, eq83_e1984_d_n5, eq83_e1984_d_n6, eq83_e1984_d_n7, eq83_e1984_d_n8, eq83_e1984_d_n9, eq83_e1984_d_n10, eq83_e1984_d_n11, eq83_e1984_d_n12, eq83_e1984_d_n13, eq83_e1984_d_n14, eq83_e1984_d_n15, eq83_e1984_d_n16];
        let eq83_branch_derivatives: [f64; 14] = [eq83_e1984_d_b0, eq83_e1984_d_b1, eq83_e1984_d_b2, eq83_e1984_d_b3, eq83_e1984_d_b4, eq83_e1984_d_b5, eq83_e1984_d_b6, eq83_e1984_d_b7, eq83_e1984_d_b8, eq83_e1984_d_b9, eq83_e1984_d_b10, eq83_e1984_d_b11, eq83_e1984_d_b12, eq83_e1984_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            Some(nodes[5]),
            multiplicity * (eq83_value),
            nodes,
            &eq83_node_derivatives,
            branches,
            &eq83_branch_derivatives,
            multiplicity,
        );
        let (eq84_e1993, eq84_e1993_d_n0, eq84_e1993_d_n1, eq84_e1993_d_n2, eq84_e1993_d_n3, eq84_e1993_d_n4, eq84_e1993_d_n5, eq84_e1993_d_n6, eq84_e1993_d_n7, eq84_e1993_d_n8, eq84_e1993_d_n9, eq84_e1993_d_n10, eq84_e1993_d_n11, eq84_e1993_d_n12, eq84_e1993_d_n13, eq84_e1993_d_n14, eq84_e1993_d_n15, eq84_e1993_d_n16, eq84_e1993_d_b0, eq84_e1993_d_b1, eq84_e1993_d_b2, eq84_e1993_d_b3, eq84_e1993_d_b4, eq84_e1993_d_b5, eq84_e1993_d_b6, eq84_e1993_d_b7, eq84_e1993_d_b8, eq84_e1993_d_b9, eq84_e1993_d_b10, eq84_e1993_d_b11, eq84_e1993_d_b12, eq84_e1993_d_b13,) = {
    if s.b[1630] {
        let eq84_e1989: f64 = (p.p29 * s.v[338]);
        let eq84_e1989_d_n0: f64 = (p.p29 * s.dn[338][0]);
        let eq84_e1989_d_n1: f64 = (p.p29 * s.dn[338][1]);
        let eq84_e1989_d_n2: f64 = (p.p29 * s.dn[338][2]);
        let eq84_e1989_d_n3: f64 = (p.p29 * s.dn[338][3]);
        let eq84_e1989_d_n4: f64 = (p.p29 * s.dn[338][4]);
        let eq84_e1989_d_n5: f64 = (p.p29 * s.dn[338][5]);
        let eq84_e1989_d_n6: f64 = (p.p29 * s.dn[338][6]);
        let eq84_e1989_d_n7: f64 = (p.p29 * s.dn[338][7]);
        let eq84_e1989_d_n8: f64 = (p.p29 * s.dn[338][8]);
        let eq84_e1989_d_n9: f64 = (p.p29 * s.dn[338][9]);
        let eq84_e1989_d_n10: f64 = (p.p29 * s.dn[338][10]);
        let eq84_e1989_d_n11: f64 = (p.p29 * s.dn[338][11]);
        let eq84_e1989_d_n12: f64 = (p.p29 * s.dn[338][12]);
        let eq84_e1989_d_n13: f64 = (p.p29 * s.dn[338][13]);
        let eq84_e1989_d_n14: f64 = (p.p29 * s.dn[338][14]);
        let eq84_e1989_d_n15: f64 = (p.p29 * s.dn[338][15]);
        let eq84_e1989_d_n16: f64 = (p.p29 * s.dn[338][16]);
        let eq84_e1989_d_b0: f64 = (p.p29 * s.db[338][0]);
        let eq84_e1989_d_b1: f64 = (p.p29 * s.db[338][1]);
        let eq84_e1989_d_b2: f64 = (p.p29 * s.db[338][2]);
        let eq84_e1989_d_b3: f64 = (p.p29 * s.db[338][3]);
        let eq84_e1989_d_b4: f64 = (p.p29 * s.db[338][4]);
        let eq84_e1989_d_b5: f64 = (p.p29 * s.db[338][5]);
        let eq84_e1989_d_b6: f64 = (p.p29 * s.db[338][6]);
        let eq84_e1989_d_b7: f64 = (p.p29 * s.db[338][7]);
        let eq84_e1989_d_b8: f64 = (p.p29 * s.db[338][8]);
        let eq84_e1989_d_b9: f64 = (p.p29 * s.db[338][9]);
        let eq84_e1989_d_b10: f64 = (p.p29 * s.db[338][10]);
        let eq84_e1989_d_b11: f64 = (p.p29 * s.db[338][11]);
        let eq84_e1989_d_b12: f64 = (p.p29 * s.db[338][12]);
        let eq84_e1989_d_b13: f64 = (p.p29 * s.db[338][13]);
        let eq84_e1990: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 15, eq84_e1989);
        let eq84_e1990_d_n0: f64 = (eq84_e1989_d_n0 * ddt_scale);
        let eq84_e1990_d_n1: f64 = (eq84_e1989_d_n1 * ddt_scale);
        let eq84_e1990_d_n2: f64 = (eq84_e1989_d_n2 * ddt_scale);
        let eq84_e1990_d_n3: f64 = (eq84_e1989_d_n3 * ddt_scale);
        let eq84_e1990_d_n4: f64 = (eq84_e1989_d_n4 * ddt_scale);
        let eq84_e1990_d_n5: f64 = (eq84_e1989_d_n5 * ddt_scale);
        let eq84_e1990_d_n6: f64 = (eq84_e1989_d_n6 * ddt_scale);
        let eq84_e1990_d_n7: f64 = (eq84_e1989_d_n7 * ddt_scale);
        let eq84_e1990_d_n8: f64 = (eq84_e1989_d_n8 * ddt_scale);
        let eq84_e1990_d_n9: f64 = (eq84_e1989_d_n9 * ddt_scale);
        let eq84_e1990_d_n10: f64 = (eq84_e1989_d_n10 * ddt_scale);
        let eq84_e1990_d_n11: f64 = (eq84_e1989_d_n11 * ddt_scale);
        let eq84_e1990_d_n12: f64 = (eq84_e1989_d_n12 * ddt_scale);
        let eq84_e1990_d_n13: f64 = (eq84_e1989_d_n13 * ddt_scale);
        let eq84_e1990_d_n14: f64 = (eq84_e1989_d_n14 * ddt_scale);
        let eq84_e1990_d_n15: f64 = (eq84_e1989_d_n15 * ddt_scale);
        let eq84_e1990_d_n16: f64 = (eq84_e1989_d_n16 * ddt_scale);
        let eq84_e1990_d_b0: f64 = (eq84_e1989_d_b0 * ddt_scale);
        let eq84_e1990_d_b1: f64 = (eq84_e1989_d_b1 * ddt_scale);
        let eq84_e1990_d_b2: f64 = (eq84_e1989_d_b2 * ddt_scale);
        let eq84_e1990_d_b3: f64 = (eq84_e1989_d_b3 * ddt_scale);
        let eq84_e1990_d_b4: f64 = (eq84_e1989_d_b4 * ddt_scale);
        let eq84_e1990_d_b5: f64 = (eq84_e1989_d_b5 * ddt_scale);
        let eq84_e1990_d_b6: f64 = (eq84_e1989_d_b6 * ddt_scale);
        let eq84_e1990_d_b7: f64 = (eq84_e1989_d_b7 * ddt_scale);
        let eq84_e1990_d_b8: f64 = (eq84_e1989_d_b8 * ddt_scale);
        let eq84_e1990_d_b9: f64 = (eq84_e1989_d_b9 * ddt_scale);
        let eq84_e1990_d_b10: f64 = (eq84_e1989_d_b10 * ddt_scale);
        let eq84_e1990_d_b11: f64 = (eq84_e1989_d_b11 * ddt_scale);
        let eq84_e1990_d_b12: f64 = (eq84_e1989_d_b12 * ddt_scale);
        let eq84_e1990_d_b13: f64 = (eq84_e1989_d_b13 * ddt_scale);
        let eq84_e1991: f64 = (s.v[187] * eq84_e1990);
        let eq84_e1991_d_n0: f64 = ((s.dn[187][0] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n0));
        let eq84_e1991_d_n1: f64 = ((s.dn[187][1] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n1));
        let eq84_e1991_d_n2: f64 = ((s.dn[187][2] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n2));
        let eq84_e1991_d_n3: f64 = ((s.dn[187][3] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n3));
        let eq84_e1991_d_n4: f64 = ((s.dn[187][4] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n4));
        let eq84_e1991_d_n5: f64 = ((s.dn[187][5] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n5));
        let eq84_e1991_d_n6: f64 = ((s.dn[187][6] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n6));
        let eq84_e1991_d_n7: f64 = ((s.dn[187][7] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n7));
        let eq84_e1991_d_n8: f64 = ((s.dn[187][8] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n8));
        let eq84_e1991_d_n9: f64 = ((s.dn[187][9] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n9));
        let eq84_e1991_d_n10: f64 = ((s.dn[187][10] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n10));
        let eq84_e1991_d_n11: f64 = ((s.dn[187][11] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n11));
        let eq84_e1991_d_n12: f64 = ((s.dn[187][12] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n12));
        let eq84_e1991_d_n13: f64 = ((s.dn[187][13] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n13));
        let eq84_e1991_d_n14: f64 = ((s.dn[187][14] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n14));
        let eq84_e1991_d_n15: f64 = ((s.dn[187][15] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n15));
        let eq84_e1991_d_n16: f64 = ((s.dn[187][16] * eq84_e1990) + (s.v[187] * eq84_e1990_d_n16));
        let eq84_e1991_d_b0: f64 = ((s.db[187][0] * eq84_e1990) + (s.v[187] * eq84_e1990_d_b0));
        let eq84_e1991_d_b1: f64 = ((s.db[187][1] * eq84_e1990) + (s.v[187] * eq84_e1990_d_b1));
        let eq84_e1991_d_b2: f64 = ((s.db[187][2] * eq84_e1990) + (s.v[187] * eq84_e1990_d_b2));
        let eq84_e1991_d_b3: f64 = ((s.db[187][3] * eq84_e1990) + (s.v[187] * eq84_e1990_d_b3));
        let eq84_e1991_d_b4: f64 = ((s.db[187][4] * eq84_e1990) + (s.v[187] * eq84_e1990_d_b4));
        let eq84_e1991_d_b5: f64 = ((s.db[187][5] * eq84_e1990) + (s.v[187] * eq84_e1990_d_b5));
        let eq84_e1991_d_b6: f64 = ((s.db[187][6] * eq84_e1990) + (s.v[187] * eq84_e1990_d_b6));
        let eq84_e1991_d_b7: f64 = ((s.db[187][7] * eq84_e1990) + (s.v[187] * eq84_e1990_d_b7));
        let eq84_e1991_d_b8: f64 = ((s.db[187][8] * eq84_e1990) + (s.v[187] * eq84_e1990_d_b8));
        let eq84_e1991_d_b9: f64 = ((s.db[187][9] * eq84_e1990) + (s.v[187] * eq84_e1990_d_b9));
        let eq84_e1991_d_b10: f64 = ((s.db[187][10] * eq84_e1990) + (s.v[187] * eq84_e1990_d_b10));
        let eq84_e1991_d_b11: f64 = ((s.db[187][11] * eq84_e1990) + (s.v[187] * eq84_e1990_d_b11));
        let eq84_e1991_d_b12: f64 = ((s.db[187][12] * eq84_e1990) + (s.v[187] * eq84_e1990_d_b12));
        let eq84_e1991_d_b13: f64 = ((s.db[187][13] * eq84_e1990) + (s.v[187] * eq84_e1990_d_b13));
        (eq84_e1991, eq84_e1991_d_n0, eq84_e1991_d_n1, eq84_e1991_d_n2, eq84_e1991_d_n3, eq84_e1991_d_n4, eq84_e1991_d_n5, eq84_e1991_d_n6, eq84_e1991_d_n7, eq84_e1991_d_n8, eq84_e1991_d_n9, eq84_e1991_d_n10, eq84_e1991_d_n11, eq84_e1991_d_n12, eq84_e1991_d_n13, eq84_e1991_d_n14, eq84_e1991_d_n15, eq84_e1991_d_n16, eq84_e1991_d_b0, eq84_e1991_d_b1, eq84_e1991_d_b2, eq84_e1991_d_b3, eq84_e1991_d_b4, eq84_e1991_d_b5, eq84_e1991_d_b6, eq84_e1991_d_b7, eq84_e1991_d_b8, eq84_e1991_d_b9, eq84_e1991_d_b10, eq84_e1991_d_b11, eq84_e1991_d_b12, eq84_e1991_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq84_value: f64 = eq84_e1993;
        let eq84_node_derivatives: [f64; 17] = [eq84_e1993_d_n0, eq84_e1993_d_n1, eq84_e1993_d_n2, eq84_e1993_d_n3, eq84_e1993_d_n4, eq84_e1993_d_n5, eq84_e1993_d_n6, eq84_e1993_d_n7, eq84_e1993_d_n8, eq84_e1993_d_n9, eq84_e1993_d_n10, eq84_e1993_d_n11, eq84_e1993_d_n12, eq84_e1993_d_n13, eq84_e1993_d_n14, eq84_e1993_d_n15, eq84_e1993_d_n16];
        let eq84_branch_derivatives: [f64; 14] = [eq84_e1993_d_b0, eq84_e1993_d_b1, eq84_e1993_d_b2, eq84_e1993_d_b3, eq84_e1993_d_b4, eq84_e1993_d_b5, eq84_e1993_d_b6, eq84_e1993_d_b7, eq84_e1993_d_b8, eq84_e1993_d_b9, eq84_e1993_d_b10, eq84_e1993_d_b11, eq84_e1993_d_b12, eq84_e1993_d_b13];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            Some(nodes[14]),
            multiplicity * (eq84_value),
            nodes,
            &eq84_node_derivatives,
            branches,
            &eq84_branch_derivatives,
            multiplicity,
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq8_e1290, eq8_e1290_d_n0, eq8_e1290_d_n1, eq8_e1290_d_n2, eq8_e1290_d_n3, eq8_e1290_d_n4, eq8_e1290_d_n5, eq8_e1290_d_n6, eq8_e1290_d_n7, eq8_e1290_d_n8, eq8_e1290_d_n9, eq8_e1290_d_n10, eq8_e1290_d_n11, eq8_e1290_d_n12, eq8_e1290_d_n13, eq8_e1290_d_n14, eq8_e1290_d_n15, eq8_e1290_d_n16, eq8_e1290_d_b0, eq8_e1290_d_b1, eq8_e1290_d_b2, eq8_e1290_d_b3, eq8_e1290_d_b4, eq8_e1290_d_b5, eq8_e1290_d_b6, eq8_e1290_d_b7, eq8_e1290_d_b8, eq8_e1290_d_b9, eq8_e1290_d_b10, eq8_e1290_d_b11, eq8_e1290_d_b12, eq8_e1290_d_b13, eq8_e1290_q, eq8_e1290_q_d_n0, eq8_e1290_q_d_n1, eq8_e1290_q_d_n2, eq8_e1290_q_d_n3, eq8_e1290_q_d_n4, eq8_e1290_q_d_n5, eq8_e1290_q_d_n6, eq8_e1290_q_d_n7, eq8_e1290_q_d_n8, eq8_e1290_q_d_n9, eq8_e1290_q_d_n10, eq8_e1290_q_d_n11, eq8_e1290_q_d_n12, eq8_e1290_q_d_n13, eq8_e1290_q_d_n14, eq8_e1290_q_d_n15, eq8_e1290_q_d_n16, eq8_e1290_q_d_b0, eq8_e1290_q_d_b1, eq8_e1290_q_d_b2, eq8_e1290_q_d_b3, eq8_e1290_q_d_b4, eq8_e1290_q_d_b5, eq8_e1290_q_d_b6, eq8_e1290_q_d_b7, eq8_e1290_q_d_b8, eq8_e1290_q_d_b9, eq8_e1290_q_d_b10, eq8_e1290_q_d_b11, eq8_e1290_q_d_b12, eq8_e1290_q_d_b13,) = {
    if (s.b[1556] && (!s.b[1555])) {
        let eq8_e1279: f64 = (s.v[378] * s.v[46]);
        let eq8_e1279_d_n0: f64 = (s.dn[378][0] * s.v[46]);
        let eq8_e1279_d_n1: f64 = (s.dn[378][1] * s.v[46]);
        let eq8_e1279_d_n2: f64 = (s.dn[378][2] * s.v[46]);
        let eq8_e1279_d_n3: f64 = (s.dn[378][3] * s.v[46]);
        let eq8_e1279_d_n4: f64 = (s.dn[378][4] * s.v[46]);
        let eq8_e1279_d_n5: f64 = (s.dn[378][5] * s.v[46]);
        let eq8_e1279_d_n6: f64 = (s.dn[378][6] * s.v[46]);
        let eq8_e1279_d_n7: f64 = (s.dn[378][7] * s.v[46]);
        let eq8_e1279_d_n8: f64 = (s.dn[378][8] * s.v[46]);
        let eq8_e1279_d_n9: f64 = (s.dn[378][9] * s.v[46]);
        let eq8_e1279_d_n10: f64 = (s.dn[378][10] * s.v[46]);
        let eq8_e1279_d_n11: f64 = (s.dn[378][11] * s.v[46]);
        let eq8_e1279_d_n12: f64 = (s.dn[378][12] * s.v[46]);
        let eq8_e1279_d_n13: f64 = (s.dn[378][13] * s.v[46]);
        let eq8_e1279_d_n14: f64 = (s.dn[378][14] * s.v[46]);
        let eq8_e1279_d_n15: f64 = (s.dn[378][15] * s.v[46]);
        let eq8_e1279_d_n16: f64 = (s.dn[378][16] * s.v[46]);
        let eq8_e1279_d_b0: f64 = (s.db[378][0] * s.v[46]);
        let eq8_e1279_d_b1: f64 = (s.db[378][1] * s.v[46]);
        let eq8_e1279_d_b2: f64 = (s.db[378][2] * s.v[46]);
        let eq8_e1279_d_b3: f64 = (s.db[378][3] * s.v[46]);
        let eq8_e1279_d_b4: f64 = (s.db[378][4] * s.v[46]);
        let eq8_e1279_d_b5: f64 = (s.db[378][5] * s.v[46]);
        let eq8_e1279_d_b6: f64 = (s.db[378][6] * s.v[46]);
        let eq8_e1279_d_b7: f64 = (s.db[378][7] * s.v[46]);
        let eq8_e1279_d_b8: f64 = (s.db[378][8] * s.v[46]);
        let eq8_e1279_d_b9: f64 = (s.db[378][9] * s.v[46]);
        let eq8_e1279_d_b10: f64 = (s.db[378][10] * s.v[46]);
        let eq8_e1279_d_b11: f64 = (s.db[378][11] * s.v[46]);
        let eq8_e1279_d_b12: f64 = (s.db[378][12] * s.v[46]);
        let eq8_e1279_d_b13: f64 = (s.db[378][13] * s.v[46]);
        let eq8_e1281: f64 = (eq8_e1279 * s.v[29]);
        let eq8_e1281_d_n0: f64 = (eq8_e1279_d_n0 * s.v[29]);
        let eq8_e1281_d_n1: f64 = (eq8_e1279_d_n1 * s.v[29]);
        let eq8_e1281_d_n2: f64 = (eq8_e1279_d_n2 * s.v[29]);
        let eq8_e1281_d_n3: f64 = (eq8_e1279_d_n3 * s.v[29]);
        let eq8_e1281_d_n4: f64 = (eq8_e1279_d_n4 * s.v[29]);
        let eq8_e1281_d_n5: f64 = (eq8_e1279_d_n5 * s.v[29]);
        let eq8_e1281_d_n6: f64 = (eq8_e1279_d_n6 * s.v[29]);
        let eq8_e1281_d_n7: f64 = (eq8_e1279_d_n7 * s.v[29]);
        let eq8_e1281_d_n8: f64 = (eq8_e1279_d_n8 * s.v[29]);
        let eq8_e1281_d_n9: f64 = (eq8_e1279_d_n9 * s.v[29]);
        let eq8_e1281_d_n10: f64 = (eq8_e1279_d_n10 * s.v[29]);
        let eq8_e1281_d_n11: f64 = (eq8_e1279_d_n11 * s.v[29]);
        let eq8_e1281_d_n12: f64 = (eq8_e1279_d_n12 * s.v[29]);
        let eq8_e1281_d_n13: f64 = (eq8_e1279_d_n13 * s.v[29]);
        let eq8_e1281_d_n14: f64 = (eq8_e1279_d_n14 * s.v[29]);
        let eq8_e1281_d_n15: f64 = (eq8_e1279_d_n15 * s.v[29]);
        let eq8_e1281_d_n16: f64 = (eq8_e1279_d_n16 * s.v[29]);
        let eq8_e1281_d_b0: f64 = (eq8_e1279_d_b0 * s.v[29]);
        let eq8_e1281_d_b1: f64 = (eq8_e1279_d_b1 * s.v[29]);
        let eq8_e1281_d_b2: f64 = (eq8_e1279_d_b2 * s.v[29]);
        let eq8_e1281_d_b3: f64 = (eq8_e1279_d_b3 * s.v[29]);
        let eq8_e1281_d_b4: f64 = (eq8_e1279_d_b4 * s.v[29]);
        let eq8_e1281_d_b5: f64 = (eq8_e1279_d_b5 * s.v[29]);
        let eq8_e1281_d_b6: f64 = (eq8_e1279_d_b6 * s.v[29]);
        let eq8_e1281_d_b7: f64 = (eq8_e1279_d_b7 * s.v[29]);
        let eq8_e1281_d_b8: f64 = (eq8_e1279_d_b8 * s.v[29]);
        let eq8_e1281_d_b9: f64 = (eq8_e1279_d_b9 * s.v[29]);
        let eq8_e1281_d_b10: f64 = (eq8_e1279_d_b10 * s.v[29]);
        let eq8_e1281_d_b11: f64 = (eq8_e1279_d_b11 * s.v[29]);
        let eq8_e1281_d_b12: f64 = (eq8_e1279_d_b12 * s.v[29]);
        let eq8_e1281_d_b13: f64 = (eq8_e1279_d_b13 * s.v[29]);
        let eq8_e1283: f64 = (eq8_e1281 * p.p2);
        let eq8_e1283_d_n0: f64 = (eq8_e1281_d_n0 * p.p2);
        let eq8_e1283_d_n1: f64 = (eq8_e1281_d_n1 * p.p2);
        let eq8_e1283_d_n2: f64 = (eq8_e1281_d_n2 * p.p2);
        let eq8_e1283_d_n3: f64 = (eq8_e1281_d_n3 * p.p2);
        let eq8_e1283_d_n4: f64 = (eq8_e1281_d_n4 * p.p2);
        let eq8_e1283_d_n5: f64 = (eq8_e1281_d_n5 * p.p2);
        let eq8_e1283_d_n6: f64 = (eq8_e1281_d_n6 * p.p2);
        let eq8_e1283_d_n7: f64 = (eq8_e1281_d_n7 * p.p2);
        let eq8_e1283_d_n8: f64 = (eq8_e1281_d_n8 * p.p2);
        let eq8_e1283_d_n9: f64 = (eq8_e1281_d_n9 * p.p2);
        let eq8_e1283_d_n10: f64 = (eq8_e1281_d_n10 * p.p2);
        let eq8_e1283_d_n11: f64 = (eq8_e1281_d_n11 * p.p2);
        let eq8_e1283_d_n12: f64 = (eq8_e1281_d_n12 * p.p2);
        let eq8_e1283_d_n13: f64 = (eq8_e1281_d_n13 * p.p2);
        let eq8_e1283_d_n14: f64 = (eq8_e1281_d_n14 * p.p2);
        let eq8_e1283_d_n15: f64 = (eq8_e1281_d_n15 * p.p2);
        let eq8_e1283_d_n16: f64 = (eq8_e1281_d_n16 * p.p2);
        let eq8_e1283_d_b0: f64 = (eq8_e1281_d_b0 * p.p2);
        let eq8_e1283_d_b1: f64 = (eq8_e1281_d_b1 * p.p2);
        let eq8_e1283_d_b2: f64 = (eq8_e1281_d_b2 * p.p2);
        let eq8_e1283_d_b3: f64 = (eq8_e1281_d_b3 * p.p2);
        let eq8_e1283_d_b4: f64 = (eq8_e1281_d_b4 * p.p2);
        let eq8_e1283_d_b5: f64 = (eq8_e1281_d_b5 * p.p2);
        let eq8_e1283_d_b6: f64 = (eq8_e1281_d_b6 * p.p2);
        let eq8_e1283_d_b7: f64 = (eq8_e1281_d_b7 * p.p2);
        let eq8_e1283_d_b8: f64 = (eq8_e1281_d_b8 * p.p2);
        let eq8_e1283_d_b9: f64 = (eq8_e1281_d_b9 * p.p2);
        let eq8_e1283_d_b10: f64 = (eq8_e1281_d_b10 * p.p2);
        let eq8_e1283_d_b11: f64 = (eq8_e1281_d_b11 * p.p2);
        let eq8_e1283_d_b12: f64 = (eq8_e1281_d_b12 * p.p2);
        let eq8_e1283_d_b13: f64 = (eq8_e1281_d_b13 * p.p2);
        let eq8_e1285: f64 = (eq8_e1283 * s.v[30]);
        let eq8_e1285_d_n0: f64 = (eq8_e1283_d_n0 * s.v[30]);
        let eq8_e1285_d_n1: f64 = (eq8_e1283_d_n1 * s.v[30]);
        let eq8_e1285_d_n2: f64 = (eq8_e1283_d_n2 * s.v[30]);
        let eq8_e1285_d_n3: f64 = (eq8_e1283_d_n3 * s.v[30]);
        let eq8_e1285_d_n4: f64 = (eq8_e1283_d_n4 * s.v[30]);
        let eq8_e1285_d_n5: f64 = (eq8_e1283_d_n5 * s.v[30]);
        let eq8_e1285_d_n6: f64 = (eq8_e1283_d_n6 * s.v[30]);
        let eq8_e1285_d_n7: f64 = (eq8_e1283_d_n7 * s.v[30]);
        let eq8_e1285_d_n8: f64 = (eq8_e1283_d_n8 * s.v[30]);
        let eq8_e1285_d_n9: f64 = (eq8_e1283_d_n9 * s.v[30]);
        let eq8_e1285_d_n10: f64 = (eq8_e1283_d_n10 * s.v[30]);
        let eq8_e1285_d_n11: f64 = (eq8_e1283_d_n11 * s.v[30]);
        let eq8_e1285_d_n12: f64 = (eq8_e1283_d_n12 * s.v[30]);
        let eq8_e1285_d_n13: f64 = (eq8_e1283_d_n13 * s.v[30]);
        let eq8_e1285_d_n14: f64 = (eq8_e1283_d_n14 * s.v[30]);
        let eq8_e1285_d_n15: f64 = (eq8_e1283_d_n15 * s.v[30]);
        let eq8_e1285_d_n16: f64 = (eq8_e1283_d_n16 * s.v[30]);
        let eq8_e1285_d_b0: f64 = (eq8_e1283_d_b0 * s.v[30]);
        let eq8_e1285_d_b1: f64 = (eq8_e1283_d_b1 * s.v[30]);
        let eq8_e1285_d_b2: f64 = (eq8_e1283_d_b2 * s.v[30]);
        let eq8_e1285_d_b3: f64 = (eq8_e1283_d_b3 * s.v[30]);
        let eq8_e1285_d_b4: f64 = (eq8_e1283_d_b4 * s.v[30]);
        let eq8_e1285_d_b5: f64 = (eq8_e1283_d_b5 * s.v[30]);
        let eq8_e1285_d_b6: f64 = (eq8_e1283_d_b6 * s.v[30]);
        let eq8_e1285_d_b7: f64 = (eq8_e1283_d_b7 * s.v[30]);
        let eq8_e1285_d_b8: f64 = (eq8_e1283_d_b8 * s.v[30]);
        let eq8_e1285_d_b9: f64 = (eq8_e1283_d_b9 * s.v[30]);
        let eq8_e1285_d_b10: f64 = (eq8_e1283_d_b10 * s.v[30]);
        let eq8_e1285_d_b11: f64 = (eq8_e1283_d_b11 * s.v[30]);
        let eq8_e1285_d_b12: f64 = (eq8_e1283_d_b12 * s.v[30]);
        let eq8_e1285_d_b13: f64 = (eq8_e1283_d_b13 * s.v[30]);
        let eq8_e1287: f64 = (eq8_e1285 * (nv15 - 0.0));
        let eq8_e1287_d_n0: f64 = (eq8_e1285_d_n0 * (nv15 - 0.0));
        let eq8_e1287_d_n1: f64 = (eq8_e1285_d_n1 * (nv15 - 0.0));
        let eq8_e1287_d_n2: f64 = (eq8_e1285_d_n2 * (nv15 - 0.0));
        let eq8_e1287_d_n3: f64 = (eq8_e1285_d_n3 * (nv15 - 0.0));
        let eq8_e1287_d_n4: f64 = (eq8_e1285_d_n4 * (nv15 - 0.0));
        let eq8_e1287_d_n5: f64 = (eq8_e1285_d_n5 * (nv15 - 0.0));
        let eq8_e1287_d_n6: f64 = (eq8_e1285_d_n6 * (nv15 - 0.0));
        let eq8_e1287_d_n7: f64 = (eq8_e1285_d_n7 * (nv15 - 0.0));
        let eq8_e1287_d_n8: f64 = (eq8_e1285_d_n8 * (nv15 - 0.0));
        let eq8_e1287_d_n9: f64 = (eq8_e1285_d_n9 * (nv15 - 0.0));
        let eq8_e1287_d_n10: f64 = (eq8_e1285_d_n10 * (nv15 - 0.0));
        let eq8_e1287_d_n11: f64 = (eq8_e1285_d_n11 * (nv15 - 0.0));
        let eq8_e1287_d_n12: f64 = (eq8_e1285_d_n12 * (nv15 - 0.0));
        let eq8_e1287_d_n13: f64 = (eq8_e1285_d_n13 * (nv15 - 0.0));
        let eq8_e1287_d_n14: f64 = (eq8_e1285_d_n14 * (nv15 - 0.0));
        let eq8_e1287_d_n15: f64 = ((eq8_e1285_d_n15 * (nv15 - 0.0)) + eq8_e1285);
        let eq8_e1287_d_n16: f64 = (eq8_e1285_d_n16 * (nv15 - 0.0));
        let eq8_e1287_d_b0: f64 = (eq8_e1285_d_b0 * (nv15 - 0.0));
        let eq8_e1287_d_b1: f64 = (eq8_e1285_d_b1 * (nv15 - 0.0));
        let eq8_e1287_d_b2: f64 = (eq8_e1285_d_b2 * (nv15 - 0.0));
        let eq8_e1287_d_b3: f64 = (eq8_e1285_d_b3 * (nv15 - 0.0));
        let eq8_e1287_d_b4: f64 = (eq8_e1285_d_b4 * (nv15 - 0.0));
        let eq8_e1287_d_b5: f64 = (eq8_e1285_d_b5 * (nv15 - 0.0));
        let eq8_e1287_d_b6: f64 = (eq8_e1285_d_b6 * (nv15 - 0.0));
        let eq8_e1287_d_b7: f64 = (eq8_e1285_d_b7 * (nv15 - 0.0));
        let eq8_e1287_d_b8: f64 = (eq8_e1285_d_b8 * (nv15 - 0.0));
        let eq8_e1287_d_b9: f64 = (eq8_e1285_d_b9 * (nv15 - 0.0));
        let eq8_e1287_d_b10: f64 = (eq8_e1285_d_b10 * (nv15 - 0.0));
        let eq8_e1287_d_b11: f64 = (eq8_e1285_d_b11 * (nv15 - 0.0));
        let eq8_e1287_d_b12: f64 = (eq8_e1285_d_b12 * (nv15 - 0.0));
        let eq8_e1287_d_b13: f64 = (eq8_e1285_d_b13 * (nv15 - 0.0));
        let eq8_e1288_q: f64 = eq8_e1287;
        (eq8_e1287, eq8_e1287_d_n0, eq8_e1287_d_n1, eq8_e1287_d_n2, eq8_e1287_d_n3, eq8_e1287_d_n4, eq8_e1287_d_n5, eq8_e1287_d_n6, eq8_e1287_d_n7, eq8_e1287_d_n8, eq8_e1287_d_n9, eq8_e1287_d_n10, eq8_e1287_d_n11, eq8_e1287_d_n12, eq8_e1287_d_n13, eq8_e1287_d_n14, eq8_e1287_d_n15, eq8_e1287_d_n16, eq8_e1287_d_b0, eq8_e1287_d_b1, eq8_e1287_d_b2, eq8_e1287_d_b3, eq8_e1287_d_b4, eq8_e1287_d_b5, eq8_e1287_d_b6, eq8_e1287_d_b7, eq8_e1287_d_b8, eq8_e1287_d_b9, eq8_e1287_d_b10, eq8_e1287_d_b11, eq8_e1287_d_b12, eq8_e1287_d_b13, eq8_e1288_q, eq8_e1287_d_n0, eq8_e1287_d_n1, eq8_e1287_d_n2, eq8_e1287_d_n3, eq8_e1287_d_n4, eq8_e1287_d_n5, eq8_e1287_d_n6, eq8_e1287_d_n7, eq8_e1287_d_n8, eq8_e1287_d_n9, eq8_e1287_d_n10, eq8_e1287_d_n11, eq8_e1287_d_n12, eq8_e1287_d_n13, eq8_e1287_d_n14, eq8_e1287_d_n15, eq8_e1287_d_n16, eq8_e1287_d_b0, eq8_e1287_d_b1, eq8_e1287_d_b2, eq8_e1287_d_b3, eq8_e1287_d_b4, eq8_e1287_d_b5, eq8_e1287_d_b6, eq8_e1287_d_b7, eq8_e1287_d_b8, eq8_e1287_d_b9, eq8_e1287_d_b10, eq8_e1287_d_b11, eq8_e1287_d_b12, eq8_e1287_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_reactive_node_derivatives: [f64; 17] = [eq8_e1290_q_d_n0, eq8_e1290_q_d_n1, eq8_e1290_q_d_n2, eq8_e1290_q_d_n3, eq8_e1290_q_d_n4, eq8_e1290_q_d_n5, eq8_e1290_q_d_n6, eq8_e1290_q_d_n7, eq8_e1290_q_d_n8, eq8_e1290_q_d_n9, eq8_e1290_q_d_n10, eq8_e1290_q_d_n11, eq8_e1290_q_d_n12, eq8_e1290_q_d_n13, eq8_e1290_q_d_n14, eq8_e1290_q_d_n15, eq8_e1290_q_d_n16];
        let eq8_reactive_branch_derivatives: [f64; 14] = [eq8_e1290_q_d_b0, eq8_e1290_q_d_b1, eq8_e1290_q_d_b2, eq8_e1290_q_d_b3, eq8_e1290_q_d_b4, eq8_e1290_q_d_b5, eq8_e1290_q_d_b6, eq8_e1290_q_d_b7, eq8_e1290_q_d_b8, eq8_e1290_q_d_b9, eq8_e1290_q_d_b10, eq8_e1290_q_d_b11, eq8_e1290_q_d_b12, eq8_e1290_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[15]),
            None,
            nodes,
            &eq8_reactive_node_derivatives,
            branches,
            &eq8_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq11_e1344, eq11_e1344_d_n0, eq11_e1344_d_n1, eq11_e1344_d_n2, eq11_e1344_d_n3, eq11_e1344_d_n4, eq11_e1344_d_n5, eq11_e1344_d_n6, eq11_e1344_d_n7, eq11_e1344_d_n8, eq11_e1344_d_n9, eq11_e1344_d_n10, eq11_e1344_d_n11, eq11_e1344_d_n12, eq11_e1344_d_n13, eq11_e1344_d_n14, eq11_e1344_d_n15, eq11_e1344_d_n16, eq11_e1344_d_b0, eq11_e1344_d_b1, eq11_e1344_d_b2, eq11_e1344_d_b3, eq11_e1344_d_b4, eq11_e1344_d_b5, eq11_e1344_d_b6, eq11_e1344_d_b7, eq11_e1344_d_b8, eq11_e1344_d_b9, eq11_e1344_d_b10, eq11_e1344_d_b11, eq11_e1344_d_b12, eq11_e1344_d_b13, eq11_e1344_q, eq11_e1344_q_d_n0, eq11_e1344_q_d_n1, eq11_e1344_q_d_n2, eq11_e1344_q_d_n3, eq11_e1344_q_d_n4, eq11_e1344_q_d_n5, eq11_e1344_q_d_n6, eq11_e1344_q_d_n7, eq11_e1344_q_d_n8, eq11_e1344_q_d_n9, eq11_e1344_q_d_n10, eq11_e1344_q_d_n11, eq11_e1344_q_d_n12, eq11_e1344_q_d_n13, eq11_e1344_q_d_n14, eq11_e1344_q_d_n15, eq11_e1344_q_d_n16, eq11_e1344_q_d_b0, eq11_e1344_q_d_b1, eq11_e1344_q_d_b2, eq11_e1344_q_d_b3, eq11_e1344_q_d_b4, eq11_e1344_q_d_b5, eq11_e1344_q_d_b6, eq11_e1344_q_d_b7, eq11_e1344_q_d_b8, eq11_e1344_q_d_b9, eq11_e1344_q_d_b10, eq11_e1344_q_d_b11, eq11_e1344_q_d_b12, eq11_e1344_q_d_b13,) = {
    if (s.b[1556] && (!s.b[1555])) {
        let eq11_e1327: f64 = (1.0 + s.v[57]);
        let eq11_e1329: f64 = (eq11_e1327 * s.v[378]);
        let eq11_e1329_d_n0: f64 = ((s.dn[57][0] * s.v[378]) + (eq11_e1327 * s.dn[378][0]));
        let eq11_e1329_d_n1: f64 = ((s.dn[57][1] * s.v[378]) + (eq11_e1327 * s.dn[378][1]));
        let eq11_e1329_d_n2: f64 = ((s.dn[57][2] * s.v[378]) + (eq11_e1327 * s.dn[378][2]));
        let eq11_e1329_d_n3: f64 = ((s.dn[57][3] * s.v[378]) + (eq11_e1327 * s.dn[378][3]));
        let eq11_e1329_d_n4: f64 = ((s.dn[57][4] * s.v[378]) + (eq11_e1327 * s.dn[378][4]));
        let eq11_e1329_d_n5: f64 = ((s.dn[57][5] * s.v[378]) + (eq11_e1327 * s.dn[378][5]));
        let eq11_e1329_d_n6: f64 = ((s.dn[57][6] * s.v[378]) + (eq11_e1327 * s.dn[378][6]));
        let eq11_e1329_d_n7: f64 = ((s.dn[57][7] * s.v[378]) + (eq11_e1327 * s.dn[378][7]));
        let eq11_e1329_d_n8: f64 = ((s.dn[57][8] * s.v[378]) + (eq11_e1327 * s.dn[378][8]));
        let eq11_e1329_d_n9: f64 = ((s.dn[57][9] * s.v[378]) + (eq11_e1327 * s.dn[378][9]));
        let eq11_e1329_d_n10: f64 = ((s.dn[57][10] * s.v[378]) + (eq11_e1327 * s.dn[378][10]));
        let eq11_e1329_d_n11: f64 = ((s.dn[57][11] * s.v[378]) + (eq11_e1327 * s.dn[378][11]));
        let eq11_e1329_d_n12: f64 = ((s.dn[57][12] * s.v[378]) + (eq11_e1327 * s.dn[378][12]));
        let eq11_e1329_d_n13: f64 = ((s.dn[57][13] * s.v[378]) + (eq11_e1327 * s.dn[378][13]));
        let eq11_e1329_d_n14: f64 = ((s.dn[57][14] * s.v[378]) + (eq11_e1327 * s.dn[378][14]));
        let eq11_e1329_d_n15: f64 = ((s.dn[57][15] * s.v[378]) + (eq11_e1327 * s.dn[378][15]));
        let eq11_e1329_d_n16: f64 = ((s.dn[57][16] * s.v[378]) + (eq11_e1327 * s.dn[378][16]));
        let eq11_e1329_d_b0: f64 = ((s.db[57][0] * s.v[378]) + (eq11_e1327 * s.db[378][0]));
        let eq11_e1329_d_b1: f64 = ((s.db[57][1] * s.v[378]) + (eq11_e1327 * s.db[378][1]));
        let eq11_e1329_d_b2: f64 = ((s.db[57][2] * s.v[378]) + (eq11_e1327 * s.db[378][2]));
        let eq11_e1329_d_b3: f64 = ((s.db[57][3] * s.v[378]) + (eq11_e1327 * s.db[378][3]));
        let eq11_e1329_d_b4: f64 = ((s.db[57][4] * s.v[378]) + (eq11_e1327 * s.db[378][4]));
        let eq11_e1329_d_b5: f64 = ((s.db[57][5] * s.v[378]) + (eq11_e1327 * s.db[378][5]));
        let eq11_e1329_d_b6: f64 = ((s.db[57][6] * s.v[378]) + (eq11_e1327 * s.db[378][6]));
        let eq11_e1329_d_b7: f64 = ((s.db[57][7] * s.v[378]) + (eq11_e1327 * s.db[378][7]));
        let eq11_e1329_d_b8: f64 = ((s.db[57][8] * s.v[378]) + (eq11_e1327 * s.db[378][8]));
        let eq11_e1329_d_b9: f64 = ((s.db[57][9] * s.v[378]) + (eq11_e1327 * s.db[378][9]));
        let eq11_e1329_d_b10: f64 = ((s.db[57][10] * s.v[378]) + (eq11_e1327 * s.db[378][10]));
        let eq11_e1329_d_b11: f64 = ((s.db[57][11] * s.v[378]) + (eq11_e1327 * s.db[378][11]));
        let eq11_e1329_d_b12: f64 = ((s.db[57][12] * s.v[378]) + (eq11_e1327 * s.db[378][12]));
        let eq11_e1329_d_b13: f64 = ((s.db[57][13] * s.v[378]) + (eq11_e1327 * s.db[378][13]));
        let eq11_e1331: f64 = (eq11_e1329 * s.v[46]);
        let eq11_e1331_d_n0: f64 = (eq11_e1329_d_n0 * s.v[46]);
        let eq11_e1331_d_n1: f64 = (eq11_e1329_d_n1 * s.v[46]);
        let eq11_e1331_d_n2: f64 = (eq11_e1329_d_n2 * s.v[46]);
        let eq11_e1331_d_n3: f64 = (eq11_e1329_d_n3 * s.v[46]);
        let eq11_e1331_d_n4: f64 = (eq11_e1329_d_n4 * s.v[46]);
        let eq11_e1331_d_n5: f64 = (eq11_e1329_d_n5 * s.v[46]);
        let eq11_e1331_d_n6: f64 = (eq11_e1329_d_n6 * s.v[46]);
        let eq11_e1331_d_n7: f64 = (eq11_e1329_d_n7 * s.v[46]);
        let eq11_e1331_d_n8: f64 = (eq11_e1329_d_n8 * s.v[46]);
        let eq11_e1331_d_n9: f64 = (eq11_e1329_d_n9 * s.v[46]);
        let eq11_e1331_d_n10: f64 = (eq11_e1329_d_n10 * s.v[46]);
        let eq11_e1331_d_n11: f64 = (eq11_e1329_d_n11 * s.v[46]);
        let eq11_e1331_d_n12: f64 = (eq11_e1329_d_n12 * s.v[46]);
        let eq11_e1331_d_n13: f64 = (eq11_e1329_d_n13 * s.v[46]);
        let eq11_e1331_d_n14: f64 = (eq11_e1329_d_n14 * s.v[46]);
        let eq11_e1331_d_n15: f64 = (eq11_e1329_d_n15 * s.v[46]);
        let eq11_e1331_d_n16: f64 = (eq11_e1329_d_n16 * s.v[46]);
        let eq11_e1331_d_b0: f64 = (eq11_e1329_d_b0 * s.v[46]);
        let eq11_e1331_d_b1: f64 = (eq11_e1329_d_b1 * s.v[46]);
        let eq11_e1331_d_b2: f64 = (eq11_e1329_d_b2 * s.v[46]);
        let eq11_e1331_d_b3: f64 = (eq11_e1329_d_b3 * s.v[46]);
        let eq11_e1331_d_b4: f64 = (eq11_e1329_d_b4 * s.v[46]);
        let eq11_e1331_d_b5: f64 = (eq11_e1329_d_b5 * s.v[46]);
        let eq11_e1331_d_b6: f64 = (eq11_e1329_d_b6 * s.v[46]);
        let eq11_e1331_d_b7: f64 = (eq11_e1329_d_b7 * s.v[46]);
        let eq11_e1331_d_b8: f64 = (eq11_e1329_d_b8 * s.v[46]);
        let eq11_e1331_d_b9: f64 = (eq11_e1329_d_b9 * s.v[46]);
        let eq11_e1331_d_b10: f64 = (eq11_e1329_d_b10 * s.v[46]);
        let eq11_e1331_d_b11: f64 = (eq11_e1329_d_b11 * s.v[46]);
        let eq11_e1331_d_b12: f64 = (eq11_e1329_d_b12 * s.v[46]);
        let eq11_e1331_d_b13: f64 = (eq11_e1329_d_b13 * s.v[46]);
        let eq11_e1333: f64 = (eq11_e1331 * s.v[29]);
        let eq11_e1333_d_n0: f64 = (eq11_e1331_d_n0 * s.v[29]);
        let eq11_e1333_d_n1: f64 = (eq11_e1331_d_n1 * s.v[29]);
        let eq11_e1333_d_n2: f64 = (eq11_e1331_d_n2 * s.v[29]);
        let eq11_e1333_d_n3: f64 = (eq11_e1331_d_n3 * s.v[29]);
        let eq11_e1333_d_n4: f64 = (eq11_e1331_d_n4 * s.v[29]);
        let eq11_e1333_d_n5: f64 = (eq11_e1331_d_n5 * s.v[29]);
        let eq11_e1333_d_n6: f64 = (eq11_e1331_d_n6 * s.v[29]);
        let eq11_e1333_d_n7: f64 = (eq11_e1331_d_n7 * s.v[29]);
        let eq11_e1333_d_n8: f64 = (eq11_e1331_d_n8 * s.v[29]);
        let eq11_e1333_d_n9: f64 = (eq11_e1331_d_n9 * s.v[29]);
        let eq11_e1333_d_n10: f64 = (eq11_e1331_d_n10 * s.v[29]);
        let eq11_e1333_d_n11: f64 = (eq11_e1331_d_n11 * s.v[29]);
        let eq11_e1333_d_n12: f64 = (eq11_e1331_d_n12 * s.v[29]);
        let eq11_e1333_d_n13: f64 = (eq11_e1331_d_n13 * s.v[29]);
        let eq11_e1333_d_n14: f64 = (eq11_e1331_d_n14 * s.v[29]);
        let eq11_e1333_d_n15: f64 = (eq11_e1331_d_n15 * s.v[29]);
        let eq11_e1333_d_n16: f64 = (eq11_e1331_d_n16 * s.v[29]);
        let eq11_e1333_d_b0: f64 = (eq11_e1331_d_b0 * s.v[29]);
        let eq11_e1333_d_b1: f64 = (eq11_e1331_d_b1 * s.v[29]);
        let eq11_e1333_d_b2: f64 = (eq11_e1331_d_b2 * s.v[29]);
        let eq11_e1333_d_b3: f64 = (eq11_e1331_d_b3 * s.v[29]);
        let eq11_e1333_d_b4: f64 = (eq11_e1331_d_b4 * s.v[29]);
        let eq11_e1333_d_b5: f64 = (eq11_e1331_d_b5 * s.v[29]);
        let eq11_e1333_d_b6: f64 = (eq11_e1331_d_b6 * s.v[29]);
        let eq11_e1333_d_b7: f64 = (eq11_e1331_d_b7 * s.v[29]);
        let eq11_e1333_d_b8: f64 = (eq11_e1331_d_b8 * s.v[29]);
        let eq11_e1333_d_b9: f64 = (eq11_e1331_d_b9 * s.v[29]);
        let eq11_e1333_d_b10: f64 = (eq11_e1331_d_b10 * s.v[29]);
        let eq11_e1333_d_b11: f64 = (eq11_e1331_d_b11 * s.v[29]);
        let eq11_e1333_d_b12: f64 = (eq11_e1331_d_b12 * s.v[29]);
        let eq11_e1333_d_b13: f64 = (eq11_e1331_d_b13 * s.v[29]);
        let eq11_e1335: f64 = (eq11_e1333 * p.p2);
        let eq11_e1335_d_n0: f64 = (eq11_e1333_d_n0 * p.p2);
        let eq11_e1335_d_n1: f64 = (eq11_e1333_d_n1 * p.p2);
        let eq11_e1335_d_n2: f64 = (eq11_e1333_d_n2 * p.p2);
        let eq11_e1335_d_n3: f64 = (eq11_e1333_d_n3 * p.p2);
        let eq11_e1335_d_n4: f64 = (eq11_e1333_d_n4 * p.p2);
        let eq11_e1335_d_n5: f64 = (eq11_e1333_d_n5 * p.p2);
        let eq11_e1335_d_n6: f64 = (eq11_e1333_d_n6 * p.p2);
        let eq11_e1335_d_n7: f64 = (eq11_e1333_d_n7 * p.p2);
        let eq11_e1335_d_n8: f64 = (eq11_e1333_d_n8 * p.p2);
        let eq11_e1335_d_n9: f64 = (eq11_e1333_d_n9 * p.p2);
        let eq11_e1335_d_n10: f64 = (eq11_e1333_d_n10 * p.p2);
        let eq11_e1335_d_n11: f64 = (eq11_e1333_d_n11 * p.p2);
        let eq11_e1335_d_n12: f64 = (eq11_e1333_d_n12 * p.p2);
        let eq11_e1335_d_n13: f64 = (eq11_e1333_d_n13 * p.p2);
        let eq11_e1335_d_n14: f64 = (eq11_e1333_d_n14 * p.p2);
        let eq11_e1335_d_n15: f64 = (eq11_e1333_d_n15 * p.p2);
        let eq11_e1335_d_n16: f64 = (eq11_e1333_d_n16 * p.p2);
        let eq11_e1335_d_b0: f64 = (eq11_e1333_d_b0 * p.p2);
        let eq11_e1335_d_b1: f64 = (eq11_e1333_d_b1 * p.p2);
        let eq11_e1335_d_b2: f64 = (eq11_e1333_d_b2 * p.p2);
        let eq11_e1335_d_b3: f64 = (eq11_e1333_d_b3 * p.p2);
        let eq11_e1335_d_b4: f64 = (eq11_e1333_d_b4 * p.p2);
        let eq11_e1335_d_b5: f64 = (eq11_e1333_d_b5 * p.p2);
        let eq11_e1335_d_b6: f64 = (eq11_e1333_d_b6 * p.p2);
        let eq11_e1335_d_b7: f64 = (eq11_e1333_d_b7 * p.p2);
        let eq11_e1335_d_b8: f64 = (eq11_e1333_d_b8 * p.p2);
        let eq11_e1335_d_b9: f64 = (eq11_e1333_d_b9 * p.p2);
        let eq11_e1335_d_b10: f64 = (eq11_e1333_d_b10 * p.p2);
        let eq11_e1335_d_b11: f64 = (eq11_e1333_d_b11 * p.p2);
        let eq11_e1335_d_b12: f64 = (eq11_e1333_d_b12 * p.p2);
        let eq11_e1335_d_b13: f64 = (eq11_e1333_d_b13 * p.p2);
        let eq11_e1337: f64 = (eq11_e1335 * s.v[30]);
        let eq11_e1337_d_n0: f64 = (eq11_e1335_d_n0 * s.v[30]);
        let eq11_e1337_d_n1: f64 = (eq11_e1335_d_n1 * s.v[30]);
        let eq11_e1337_d_n2: f64 = (eq11_e1335_d_n2 * s.v[30]);
        let eq11_e1337_d_n3: f64 = (eq11_e1335_d_n3 * s.v[30]);
        let eq11_e1337_d_n4: f64 = (eq11_e1335_d_n4 * s.v[30]);
        let eq11_e1337_d_n5: f64 = (eq11_e1335_d_n5 * s.v[30]);
        let eq11_e1337_d_n6: f64 = (eq11_e1335_d_n6 * s.v[30]);
        let eq11_e1337_d_n7: f64 = (eq11_e1335_d_n7 * s.v[30]);
        let eq11_e1337_d_n8: f64 = (eq11_e1335_d_n8 * s.v[30]);
        let eq11_e1337_d_n9: f64 = (eq11_e1335_d_n9 * s.v[30]);
        let eq11_e1337_d_n10: f64 = (eq11_e1335_d_n10 * s.v[30]);
        let eq11_e1337_d_n11: f64 = (eq11_e1335_d_n11 * s.v[30]);
        let eq11_e1337_d_n12: f64 = (eq11_e1335_d_n12 * s.v[30]);
        let eq11_e1337_d_n13: f64 = (eq11_e1335_d_n13 * s.v[30]);
        let eq11_e1337_d_n14: f64 = (eq11_e1335_d_n14 * s.v[30]);
        let eq11_e1337_d_n15: f64 = (eq11_e1335_d_n15 * s.v[30]);
        let eq11_e1337_d_n16: f64 = (eq11_e1335_d_n16 * s.v[30]);
        let eq11_e1337_d_b0: f64 = (eq11_e1335_d_b0 * s.v[30]);
        let eq11_e1337_d_b1: f64 = (eq11_e1335_d_b1 * s.v[30]);
        let eq11_e1337_d_b2: f64 = (eq11_e1335_d_b2 * s.v[30]);
        let eq11_e1337_d_b3: f64 = (eq11_e1335_d_b3 * s.v[30]);
        let eq11_e1337_d_b4: f64 = (eq11_e1335_d_b4 * s.v[30]);
        let eq11_e1337_d_b5: f64 = (eq11_e1335_d_b5 * s.v[30]);
        let eq11_e1337_d_b6: f64 = (eq11_e1335_d_b6 * s.v[30]);
        let eq11_e1337_d_b7: f64 = (eq11_e1335_d_b7 * s.v[30]);
        let eq11_e1337_d_b8: f64 = (eq11_e1335_d_b8 * s.v[30]);
        let eq11_e1337_d_b9: f64 = (eq11_e1335_d_b9 * s.v[30]);
        let eq11_e1337_d_b10: f64 = (eq11_e1335_d_b10 * s.v[30]);
        let eq11_e1337_d_b11: f64 = (eq11_e1335_d_b11 * s.v[30]);
        let eq11_e1337_d_b12: f64 = (eq11_e1335_d_b12 * s.v[30]);
        let eq11_e1337_d_b13: f64 = (eq11_e1335_d_b13 * s.v[30]);
        let eq11_e1339: f64 = (eq11_e1337 * (nv15 - 0.0));
        let eq11_e1339_d_n0: f64 = (eq11_e1337_d_n0 * (nv15 - 0.0));
        let eq11_e1339_d_n1: f64 = (eq11_e1337_d_n1 * (nv15 - 0.0));
        let eq11_e1339_d_n2: f64 = (eq11_e1337_d_n2 * (nv15 - 0.0));
        let eq11_e1339_d_n3: f64 = (eq11_e1337_d_n3 * (nv15 - 0.0));
        let eq11_e1339_d_n4: f64 = (eq11_e1337_d_n4 * (nv15 - 0.0));
        let eq11_e1339_d_n5: f64 = (eq11_e1337_d_n5 * (nv15 - 0.0));
        let eq11_e1339_d_n6: f64 = (eq11_e1337_d_n6 * (nv15 - 0.0));
        let eq11_e1339_d_n7: f64 = (eq11_e1337_d_n7 * (nv15 - 0.0));
        let eq11_e1339_d_n8: f64 = (eq11_e1337_d_n8 * (nv15 - 0.0));
        let eq11_e1339_d_n9: f64 = (eq11_e1337_d_n9 * (nv15 - 0.0));
        let eq11_e1339_d_n10: f64 = (eq11_e1337_d_n10 * (nv15 - 0.0));
        let eq11_e1339_d_n11: f64 = (eq11_e1337_d_n11 * (nv15 - 0.0));
        let eq11_e1339_d_n12: f64 = (eq11_e1337_d_n12 * (nv15 - 0.0));
        let eq11_e1339_d_n13: f64 = (eq11_e1337_d_n13 * (nv15 - 0.0));
        let eq11_e1339_d_n14: f64 = (eq11_e1337_d_n14 * (nv15 - 0.0));
        let eq11_e1339_d_n15: f64 = ((eq11_e1337_d_n15 * (nv15 - 0.0)) + eq11_e1337);
        let eq11_e1339_d_n16: f64 = (eq11_e1337_d_n16 * (nv15 - 0.0));
        let eq11_e1339_d_b0: f64 = (eq11_e1337_d_b0 * (nv15 - 0.0));
        let eq11_e1339_d_b1: f64 = (eq11_e1337_d_b1 * (nv15 - 0.0));
        let eq11_e1339_d_b2: f64 = (eq11_e1337_d_b2 * (nv15 - 0.0));
        let eq11_e1339_d_b3: f64 = (eq11_e1337_d_b3 * (nv15 - 0.0));
        let eq11_e1339_d_b4: f64 = (eq11_e1337_d_b4 * (nv15 - 0.0));
        let eq11_e1339_d_b5: f64 = (eq11_e1337_d_b5 * (nv15 - 0.0));
        let eq11_e1339_d_b6: f64 = (eq11_e1337_d_b6 * (nv15 - 0.0));
        let eq11_e1339_d_b7: f64 = (eq11_e1337_d_b7 * (nv15 - 0.0));
        let eq11_e1339_d_b8: f64 = (eq11_e1337_d_b8 * (nv15 - 0.0));
        let eq11_e1339_d_b9: f64 = (eq11_e1337_d_b9 * (nv15 - 0.0));
        let eq11_e1339_d_b10: f64 = (eq11_e1337_d_b10 * (nv15 - 0.0));
        let eq11_e1339_d_b11: f64 = (eq11_e1337_d_b11 * (nv15 - 0.0));
        let eq11_e1339_d_b12: f64 = (eq11_e1337_d_b12 * (nv15 - 0.0));
        let eq11_e1339_d_b13: f64 = (eq11_e1337_d_b13 * (nv15 - 0.0));
        let eq11_e1340: f64 = (0.5 * eq11_e1339);
        let eq11_e1340_d_n0: f64 = (0.5 * eq11_e1339_d_n0);
        let eq11_e1340_d_n1: f64 = (0.5 * eq11_e1339_d_n1);
        let eq11_e1340_d_n2: f64 = (0.5 * eq11_e1339_d_n2);
        let eq11_e1340_d_n3: f64 = (0.5 * eq11_e1339_d_n3);
        let eq11_e1340_d_n4: f64 = (0.5 * eq11_e1339_d_n4);
        let eq11_e1340_d_n5: f64 = (0.5 * eq11_e1339_d_n5);
        let eq11_e1340_d_n6: f64 = (0.5 * eq11_e1339_d_n6);
        let eq11_e1340_d_n7: f64 = (0.5 * eq11_e1339_d_n7);
        let eq11_e1340_d_n8: f64 = (0.5 * eq11_e1339_d_n8);
        let eq11_e1340_d_n9: f64 = (0.5 * eq11_e1339_d_n9);
        let eq11_e1340_d_n10: f64 = (0.5 * eq11_e1339_d_n10);
        let eq11_e1340_d_n11: f64 = (0.5 * eq11_e1339_d_n11);
        let eq11_e1340_d_n12: f64 = (0.5 * eq11_e1339_d_n12);
        let eq11_e1340_d_n13: f64 = (0.5 * eq11_e1339_d_n13);
        let eq11_e1340_d_n14: f64 = (0.5 * eq11_e1339_d_n14);
        let eq11_e1340_d_n15: f64 = (0.5 * eq11_e1339_d_n15);
        let eq11_e1340_d_n16: f64 = (0.5 * eq11_e1339_d_n16);
        let eq11_e1340_d_b0: f64 = (0.5 * eq11_e1339_d_b0);
        let eq11_e1340_d_b1: f64 = (0.5 * eq11_e1339_d_b1);
        let eq11_e1340_d_b2: f64 = (0.5 * eq11_e1339_d_b2);
        let eq11_e1340_d_b3: f64 = (0.5 * eq11_e1339_d_b3);
        let eq11_e1340_d_b4: f64 = (0.5 * eq11_e1339_d_b4);
        let eq11_e1340_d_b5: f64 = (0.5 * eq11_e1339_d_b5);
        let eq11_e1340_d_b6: f64 = (0.5 * eq11_e1339_d_b6);
        let eq11_e1340_d_b7: f64 = (0.5 * eq11_e1339_d_b7);
        let eq11_e1340_d_b8: f64 = (0.5 * eq11_e1339_d_b8);
        let eq11_e1340_d_b9: f64 = (0.5 * eq11_e1339_d_b9);
        let eq11_e1340_d_b10: f64 = (0.5 * eq11_e1339_d_b10);
        let eq11_e1340_d_b11: f64 = (0.5 * eq11_e1339_d_b11);
        let eq11_e1340_d_b12: f64 = (0.5 * eq11_e1339_d_b12);
        let eq11_e1340_d_b13: f64 = (0.5 * eq11_e1339_d_b13);
        let eq11_e1341_q: f64 = eq11_e1340;
        let eq11_e1342: f64 = (p.p29 * eq11_e1340);
        let eq11_e1342_d_n0: f64 = (p.p29 * eq11_e1340_d_n0);
        let eq11_e1342_d_n1: f64 = (p.p29 * eq11_e1340_d_n1);
        let eq11_e1342_d_n2: f64 = (p.p29 * eq11_e1340_d_n2);
        let eq11_e1342_d_n3: f64 = (p.p29 * eq11_e1340_d_n3);
        let eq11_e1342_d_n4: f64 = (p.p29 * eq11_e1340_d_n4);
        let eq11_e1342_d_n5: f64 = (p.p29 * eq11_e1340_d_n5);
        let eq11_e1342_d_n6: f64 = (p.p29 * eq11_e1340_d_n6);
        let eq11_e1342_d_n7: f64 = (p.p29 * eq11_e1340_d_n7);
        let eq11_e1342_d_n8: f64 = (p.p29 * eq11_e1340_d_n8);
        let eq11_e1342_d_n9: f64 = (p.p29 * eq11_e1340_d_n9);
        let eq11_e1342_d_n10: f64 = (p.p29 * eq11_e1340_d_n10);
        let eq11_e1342_d_n11: f64 = (p.p29 * eq11_e1340_d_n11);
        let eq11_e1342_d_n12: f64 = (p.p29 * eq11_e1340_d_n12);
        let eq11_e1342_d_n13: f64 = (p.p29 * eq11_e1340_d_n13);
        let eq11_e1342_d_n14: f64 = (p.p29 * eq11_e1340_d_n14);
        let eq11_e1342_d_n15: f64 = (p.p29 * eq11_e1340_d_n15);
        let eq11_e1342_d_n16: f64 = (p.p29 * eq11_e1340_d_n16);
        let eq11_e1342_d_b0: f64 = (p.p29 * eq11_e1340_d_b0);
        let eq11_e1342_d_b1: f64 = (p.p29 * eq11_e1340_d_b1);
        let eq11_e1342_d_b2: f64 = (p.p29 * eq11_e1340_d_b2);
        let eq11_e1342_d_b3: f64 = (p.p29 * eq11_e1340_d_b3);
        let eq11_e1342_d_b4: f64 = (p.p29 * eq11_e1340_d_b4);
        let eq11_e1342_d_b5: f64 = (p.p29 * eq11_e1340_d_b5);
        let eq11_e1342_d_b6: f64 = (p.p29 * eq11_e1340_d_b6);
        let eq11_e1342_d_b7: f64 = (p.p29 * eq11_e1340_d_b7);
        let eq11_e1342_d_b8: f64 = (p.p29 * eq11_e1340_d_b8);
        let eq11_e1342_d_b9: f64 = (p.p29 * eq11_e1340_d_b9);
        let eq11_e1342_d_b10: f64 = (p.p29 * eq11_e1340_d_b10);
        let eq11_e1342_d_b11: f64 = (p.p29 * eq11_e1340_d_b11);
        let eq11_e1342_d_b12: f64 = (p.p29 * eq11_e1340_d_b12);
        let eq11_e1342_d_b13: f64 = (p.p29 * eq11_e1340_d_b13);
        let eq11_e1342_q: f64 = (p.p29 * eq11_e1341_q);
        let eq11_e1342_q_d_n0: f64 = (p.p29 * eq11_e1340_d_n0);
        let eq11_e1342_q_d_n1: f64 = (p.p29 * eq11_e1340_d_n1);
        let eq11_e1342_q_d_n2: f64 = (p.p29 * eq11_e1340_d_n2);
        let eq11_e1342_q_d_n3: f64 = (p.p29 * eq11_e1340_d_n3);
        let eq11_e1342_q_d_n4: f64 = (p.p29 * eq11_e1340_d_n4);
        let eq11_e1342_q_d_n5: f64 = (p.p29 * eq11_e1340_d_n5);
        let eq11_e1342_q_d_n6: f64 = (p.p29 * eq11_e1340_d_n6);
        let eq11_e1342_q_d_n7: f64 = (p.p29 * eq11_e1340_d_n7);
        let eq11_e1342_q_d_n8: f64 = (p.p29 * eq11_e1340_d_n8);
        let eq11_e1342_q_d_n9: f64 = (p.p29 * eq11_e1340_d_n9);
        let eq11_e1342_q_d_n10: f64 = (p.p29 * eq11_e1340_d_n10);
        let eq11_e1342_q_d_n11: f64 = (p.p29 * eq11_e1340_d_n11);
        let eq11_e1342_q_d_n12: f64 = (p.p29 * eq11_e1340_d_n12);
        let eq11_e1342_q_d_n13: f64 = (p.p29 * eq11_e1340_d_n13);
        let eq11_e1342_q_d_n14: f64 = (p.p29 * eq11_e1340_d_n14);
        let eq11_e1342_q_d_n15: f64 = (p.p29 * eq11_e1340_d_n15);
        let eq11_e1342_q_d_n16: f64 = (p.p29 * eq11_e1340_d_n16);
        let eq11_e1342_q_d_b0: f64 = (p.p29 * eq11_e1340_d_b0);
        let eq11_e1342_q_d_b1: f64 = (p.p29 * eq11_e1340_d_b1);
        let eq11_e1342_q_d_b2: f64 = (p.p29 * eq11_e1340_d_b2);
        let eq11_e1342_q_d_b3: f64 = (p.p29 * eq11_e1340_d_b3);
        let eq11_e1342_q_d_b4: f64 = (p.p29 * eq11_e1340_d_b4);
        let eq11_e1342_q_d_b5: f64 = (p.p29 * eq11_e1340_d_b5);
        let eq11_e1342_q_d_b6: f64 = (p.p29 * eq11_e1340_d_b6);
        let eq11_e1342_q_d_b7: f64 = (p.p29 * eq11_e1340_d_b7);
        let eq11_e1342_q_d_b8: f64 = (p.p29 * eq11_e1340_d_b8);
        let eq11_e1342_q_d_b9: f64 = (p.p29 * eq11_e1340_d_b9);
        let eq11_e1342_q_d_b10: f64 = (p.p29 * eq11_e1340_d_b10);
        let eq11_e1342_q_d_b11: f64 = (p.p29 * eq11_e1340_d_b11);
        let eq11_e1342_q_d_b12: f64 = (p.p29 * eq11_e1340_d_b12);
        let eq11_e1342_q_d_b13: f64 = (p.p29 * eq11_e1340_d_b13);
        (eq11_e1342, eq11_e1342_d_n0, eq11_e1342_d_n1, eq11_e1342_d_n2, eq11_e1342_d_n3, eq11_e1342_d_n4, eq11_e1342_d_n5, eq11_e1342_d_n6, eq11_e1342_d_n7, eq11_e1342_d_n8, eq11_e1342_d_n9, eq11_e1342_d_n10, eq11_e1342_d_n11, eq11_e1342_d_n12, eq11_e1342_d_n13, eq11_e1342_d_n14, eq11_e1342_d_n15, eq11_e1342_d_n16, eq11_e1342_d_b0, eq11_e1342_d_b1, eq11_e1342_d_b2, eq11_e1342_d_b3, eq11_e1342_d_b4, eq11_e1342_d_b5, eq11_e1342_d_b6, eq11_e1342_d_b7, eq11_e1342_d_b8, eq11_e1342_d_b9, eq11_e1342_d_b10, eq11_e1342_d_b11, eq11_e1342_d_b12, eq11_e1342_d_b13, eq11_e1342_q, eq11_e1342_q_d_n0, eq11_e1342_q_d_n1, eq11_e1342_q_d_n2, eq11_e1342_q_d_n3, eq11_e1342_q_d_n4, eq11_e1342_q_d_n5, eq11_e1342_q_d_n6, eq11_e1342_q_d_n7, eq11_e1342_q_d_n8, eq11_e1342_q_d_n9, eq11_e1342_q_d_n10, eq11_e1342_q_d_n11, eq11_e1342_q_d_n12, eq11_e1342_q_d_n13, eq11_e1342_q_d_n14, eq11_e1342_q_d_n15, eq11_e1342_q_d_n16, eq11_e1342_q_d_b0, eq11_e1342_q_d_b1, eq11_e1342_q_d_b2, eq11_e1342_q_d_b3, eq11_e1342_q_d_b4, eq11_e1342_q_d_b5, eq11_e1342_q_d_b6, eq11_e1342_q_d_b7, eq11_e1342_q_d_b8, eq11_e1342_q_d_b9, eq11_e1342_q_d_b10, eq11_e1342_q_d_b11, eq11_e1342_q_d_b12, eq11_e1342_q_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_reactive_node_derivatives: [f64; 17] = [eq11_e1344_q_d_n0, eq11_e1344_q_d_n1, eq11_e1344_q_d_n2, eq11_e1344_q_d_n3, eq11_e1344_q_d_n4, eq11_e1344_q_d_n5, eq11_e1344_q_d_n6, eq11_e1344_q_d_n7, eq11_e1344_q_d_n8, eq11_e1344_q_d_n9, eq11_e1344_q_d_n10, eq11_e1344_q_d_n11, eq11_e1344_q_d_n12, eq11_e1344_q_d_n13, eq11_e1344_q_d_n14, eq11_e1344_q_d_n15, eq11_e1344_q_d_n16];
        let eq11_reactive_branch_derivatives: [f64; 14] = [eq11_e1344_q_d_b0, eq11_e1344_q_d_b1, eq11_e1344_q_d_b2, eq11_e1344_q_d_b3, eq11_e1344_q_d_b4, eq11_e1344_q_d_b5, eq11_e1344_q_d_b6, eq11_e1344_q_d_b7, eq11_e1344_q_d_b8, eq11_e1344_q_d_b9, eq11_e1344_q_d_b10, eq11_e1344_q_d_b11, eq11_e1344_q_d_b12, eq11_e1344_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq12_e1370, eq12_e1370_d_n0, eq12_e1370_d_n1, eq12_e1370_d_n2, eq12_e1370_d_n3, eq12_e1370_d_n4, eq12_e1370_d_n5, eq12_e1370_d_n6, eq12_e1370_d_n7, eq12_e1370_d_n8, eq12_e1370_d_n9, eq12_e1370_d_n10, eq12_e1370_d_n11, eq12_e1370_d_n12, eq12_e1370_d_n13, eq12_e1370_d_n14, eq12_e1370_d_n15, eq12_e1370_d_n16, eq12_e1370_d_b0, eq12_e1370_d_b1, eq12_e1370_d_b2, eq12_e1370_d_b3, eq12_e1370_d_b4, eq12_e1370_d_b5, eq12_e1370_d_b6, eq12_e1370_d_b7, eq12_e1370_d_b8, eq12_e1370_d_b9, eq12_e1370_d_b10, eq12_e1370_d_b11, eq12_e1370_d_b12, eq12_e1370_d_b13, eq12_e1370_q, eq12_e1370_q_d_n0, eq12_e1370_q_d_n1, eq12_e1370_q_d_n2, eq12_e1370_q_d_n3, eq12_e1370_q_d_n4, eq12_e1370_q_d_n5, eq12_e1370_q_d_n6, eq12_e1370_q_d_n7, eq12_e1370_q_d_n8, eq12_e1370_q_d_n9, eq12_e1370_q_d_n10, eq12_e1370_q_d_n11, eq12_e1370_q_d_n12, eq12_e1370_q_d_n13, eq12_e1370_q_d_n14, eq12_e1370_q_d_n15, eq12_e1370_q_d_n16, eq12_e1370_q_d_b0, eq12_e1370_q_d_b1, eq12_e1370_q_d_b2, eq12_e1370_q_d_b3, eq12_e1370_q_d_b4, eq12_e1370_q_d_b5, eq12_e1370_q_d_b6, eq12_e1370_q_d_b7, eq12_e1370_q_d_b8, eq12_e1370_q_d_b9, eq12_e1370_q_d_b10, eq12_e1370_q_d_b11, eq12_e1370_q_d_b12, eq12_e1370_q_d_b13,) = {
    if (s.b[1556] && (!s.b[1555])) {
        let eq12_e1353: f64 = (1.0 - s.v[57]);
        let eq12_e1353_d_n0: f64 = (-s.dn[57][0]);
        let eq12_e1353_d_n1: f64 = (-s.dn[57][1]);
        let eq12_e1353_d_n2: f64 = (-s.dn[57][2]);
        let eq12_e1353_d_n3: f64 = (-s.dn[57][3]);
        let eq12_e1353_d_n4: f64 = (-s.dn[57][4]);
        let eq12_e1353_d_n5: f64 = (-s.dn[57][5]);
        let eq12_e1353_d_n6: f64 = (-s.dn[57][6]);
        let eq12_e1353_d_n7: f64 = (-s.dn[57][7]);
        let eq12_e1353_d_n8: f64 = (-s.dn[57][8]);
        let eq12_e1353_d_n9: f64 = (-s.dn[57][9]);
        let eq12_e1353_d_n10: f64 = (-s.dn[57][10]);
        let eq12_e1353_d_n11: f64 = (-s.dn[57][11]);
        let eq12_e1353_d_n12: f64 = (-s.dn[57][12]);
        let eq12_e1353_d_n13: f64 = (-s.dn[57][13]);
        let eq12_e1353_d_n14: f64 = (-s.dn[57][14]);
        let eq12_e1353_d_n15: f64 = (-s.dn[57][15]);
        let eq12_e1353_d_n16: f64 = (-s.dn[57][16]);
        let eq12_e1353_d_b0: f64 = (-s.db[57][0]);
        let eq12_e1353_d_b1: f64 = (-s.db[57][1]);
        let eq12_e1353_d_b2: f64 = (-s.db[57][2]);
        let eq12_e1353_d_b3: f64 = (-s.db[57][3]);
        let eq12_e1353_d_b4: f64 = (-s.db[57][4]);
        let eq12_e1353_d_b5: f64 = (-s.db[57][5]);
        let eq12_e1353_d_b6: f64 = (-s.db[57][6]);
        let eq12_e1353_d_b7: f64 = (-s.db[57][7]);
        let eq12_e1353_d_b8: f64 = (-s.db[57][8]);
        let eq12_e1353_d_b9: f64 = (-s.db[57][9]);
        let eq12_e1353_d_b10: f64 = (-s.db[57][10]);
        let eq12_e1353_d_b11: f64 = (-s.db[57][11]);
        let eq12_e1353_d_b12: f64 = (-s.db[57][12]);
        let eq12_e1353_d_b13: f64 = (-s.db[57][13]);
        let eq12_e1355: f64 = (eq12_e1353 * s.v[378]);
        let eq12_e1355_d_n0: f64 = ((eq12_e1353_d_n0 * s.v[378]) + (eq12_e1353 * s.dn[378][0]));
        let eq12_e1355_d_n1: f64 = ((eq12_e1353_d_n1 * s.v[378]) + (eq12_e1353 * s.dn[378][1]));
        let eq12_e1355_d_n2: f64 = ((eq12_e1353_d_n2 * s.v[378]) + (eq12_e1353 * s.dn[378][2]));
        let eq12_e1355_d_n3: f64 = ((eq12_e1353_d_n3 * s.v[378]) + (eq12_e1353 * s.dn[378][3]));
        let eq12_e1355_d_n4: f64 = ((eq12_e1353_d_n4 * s.v[378]) + (eq12_e1353 * s.dn[378][4]));
        let eq12_e1355_d_n5: f64 = ((eq12_e1353_d_n5 * s.v[378]) + (eq12_e1353 * s.dn[378][5]));
        let eq12_e1355_d_n6: f64 = ((eq12_e1353_d_n6 * s.v[378]) + (eq12_e1353 * s.dn[378][6]));
        let eq12_e1355_d_n7: f64 = ((eq12_e1353_d_n7 * s.v[378]) + (eq12_e1353 * s.dn[378][7]));
        let eq12_e1355_d_n8: f64 = ((eq12_e1353_d_n8 * s.v[378]) + (eq12_e1353 * s.dn[378][8]));
        let eq12_e1355_d_n9: f64 = ((eq12_e1353_d_n9 * s.v[378]) + (eq12_e1353 * s.dn[378][9]));
        let eq12_e1355_d_n10: f64 = ((eq12_e1353_d_n10 * s.v[378]) + (eq12_e1353 * s.dn[378][10]));
        let eq12_e1355_d_n11: f64 = ((eq12_e1353_d_n11 * s.v[378]) + (eq12_e1353 * s.dn[378][11]));
        let eq12_e1355_d_n12: f64 = ((eq12_e1353_d_n12 * s.v[378]) + (eq12_e1353 * s.dn[378][12]));
        let eq12_e1355_d_n13: f64 = ((eq12_e1353_d_n13 * s.v[378]) + (eq12_e1353 * s.dn[378][13]));
        let eq12_e1355_d_n14: f64 = ((eq12_e1353_d_n14 * s.v[378]) + (eq12_e1353 * s.dn[378][14]));
        let eq12_e1355_d_n15: f64 = ((eq12_e1353_d_n15 * s.v[378]) + (eq12_e1353 * s.dn[378][15]));
        let eq12_e1355_d_n16: f64 = ((eq12_e1353_d_n16 * s.v[378]) + (eq12_e1353 * s.dn[378][16]));
        let eq12_e1355_d_b0: f64 = ((eq12_e1353_d_b0 * s.v[378]) + (eq12_e1353 * s.db[378][0]));
        let eq12_e1355_d_b1: f64 = ((eq12_e1353_d_b1 * s.v[378]) + (eq12_e1353 * s.db[378][1]));
        let eq12_e1355_d_b2: f64 = ((eq12_e1353_d_b2 * s.v[378]) + (eq12_e1353 * s.db[378][2]));
        let eq12_e1355_d_b3: f64 = ((eq12_e1353_d_b3 * s.v[378]) + (eq12_e1353 * s.db[378][3]));
        let eq12_e1355_d_b4: f64 = ((eq12_e1353_d_b4 * s.v[378]) + (eq12_e1353 * s.db[378][4]));
        let eq12_e1355_d_b5: f64 = ((eq12_e1353_d_b5 * s.v[378]) + (eq12_e1353 * s.db[378][5]));
        let eq12_e1355_d_b6: f64 = ((eq12_e1353_d_b6 * s.v[378]) + (eq12_e1353 * s.db[378][6]));
        let eq12_e1355_d_b7: f64 = ((eq12_e1353_d_b7 * s.v[378]) + (eq12_e1353 * s.db[378][7]));
        let eq12_e1355_d_b8: f64 = ((eq12_e1353_d_b8 * s.v[378]) + (eq12_e1353 * s.db[378][8]));
        let eq12_e1355_d_b9: f64 = ((eq12_e1353_d_b9 * s.v[378]) + (eq12_e1353 * s.db[378][9]));
        let eq12_e1355_d_b10: f64 = ((eq12_e1353_d_b10 * s.v[378]) + (eq12_e1353 * s.db[378][10]));
        let eq12_e1355_d_b11: f64 = ((eq12_e1353_d_b11 * s.v[378]) + (eq12_e1353 * s.db[378][11]));
        let eq12_e1355_d_b12: f64 = ((eq12_e1353_d_b12 * s.v[378]) + (eq12_e1353 * s.db[378][12]));
        let eq12_e1355_d_b13: f64 = ((eq12_e1353_d_b13 * s.v[378]) + (eq12_e1353 * s.db[378][13]));
        let eq12_e1357: f64 = (eq12_e1355 * s.v[46]);
        let eq12_e1357_d_n0: f64 = (eq12_e1355_d_n0 * s.v[46]);
        let eq12_e1357_d_n1: f64 = (eq12_e1355_d_n1 * s.v[46]);
        let eq12_e1357_d_n2: f64 = (eq12_e1355_d_n2 * s.v[46]);
        let eq12_e1357_d_n3: f64 = (eq12_e1355_d_n3 * s.v[46]);
        let eq12_e1357_d_n4: f64 = (eq12_e1355_d_n4 * s.v[46]);
        let eq12_e1357_d_n5: f64 = (eq12_e1355_d_n5 * s.v[46]);
        let eq12_e1357_d_n6: f64 = (eq12_e1355_d_n6 * s.v[46]);
        let eq12_e1357_d_n7: f64 = (eq12_e1355_d_n7 * s.v[46]);
        let eq12_e1357_d_n8: f64 = (eq12_e1355_d_n8 * s.v[46]);
        let eq12_e1357_d_n9: f64 = (eq12_e1355_d_n9 * s.v[46]);
        let eq12_e1357_d_n10: f64 = (eq12_e1355_d_n10 * s.v[46]);
        let eq12_e1357_d_n11: f64 = (eq12_e1355_d_n11 * s.v[46]);
        let eq12_e1357_d_n12: f64 = (eq12_e1355_d_n12 * s.v[46]);
        let eq12_e1357_d_n13: f64 = (eq12_e1355_d_n13 * s.v[46]);
        let eq12_e1357_d_n14: f64 = (eq12_e1355_d_n14 * s.v[46]);
        let eq12_e1357_d_n15: f64 = (eq12_e1355_d_n15 * s.v[46]);
        let eq12_e1357_d_n16: f64 = (eq12_e1355_d_n16 * s.v[46]);
        let eq12_e1357_d_b0: f64 = (eq12_e1355_d_b0 * s.v[46]);
        let eq12_e1357_d_b1: f64 = (eq12_e1355_d_b1 * s.v[46]);
        let eq12_e1357_d_b2: f64 = (eq12_e1355_d_b2 * s.v[46]);
        let eq12_e1357_d_b3: f64 = (eq12_e1355_d_b3 * s.v[46]);
        let eq12_e1357_d_b4: f64 = (eq12_e1355_d_b4 * s.v[46]);
        let eq12_e1357_d_b5: f64 = (eq12_e1355_d_b5 * s.v[46]);
        let eq12_e1357_d_b6: f64 = (eq12_e1355_d_b6 * s.v[46]);
        let eq12_e1357_d_b7: f64 = (eq12_e1355_d_b7 * s.v[46]);
        let eq12_e1357_d_b8: f64 = (eq12_e1355_d_b8 * s.v[46]);
        let eq12_e1357_d_b9: f64 = (eq12_e1355_d_b9 * s.v[46]);
        let eq12_e1357_d_b10: f64 = (eq12_e1355_d_b10 * s.v[46]);
        let eq12_e1357_d_b11: f64 = (eq12_e1355_d_b11 * s.v[46]);
        let eq12_e1357_d_b12: f64 = (eq12_e1355_d_b12 * s.v[46]);
        let eq12_e1357_d_b13: f64 = (eq12_e1355_d_b13 * s.v[46]);
        let eq12_e1359: f64 = (eq12_e1357 * s.v[29]);
        let eq12_e1359_d_n0: f64 = (eq12_e1357_d_n0 * s.v[29]);
        let eq12_e1359_d_n1: f64 = (eq12_e1357_d_n1 * s.v[29]);
        let eq12_e1359_d_n2: f64 = (eq12_e1357_d_n2 * s.v[29]);
        let eq12_e1359_d_n3: f64 = (eq12_e1357_d_n3 * s.v[29]);
        let eq12_e1359_d_n4: f64 = (eq12_e1357_d_n4 * s.v[29]);
        let eq12_e1359_d_n5: f64 = (eq12_e1357_d_n5 * s.v[29]);
        let eq12_e1359_d_n6: f64 = (eq12_e1357_d_n6 * s.v[29]);
        let eq12_e1359_d_n7: f64 = (eq12_e1357_d_n7 * s.v[29]);
        let eq12_e1359_d_n8: f64 = (eq12_e1357_d_n8 * s.v[29]);
        let eq12_e1359_d_n9: f64 = (eq12_e1357_d_n9 * s.v[29]);
        let eq12_e1359_d_n10: f64 = (eq12_e1357_d_n10 * s.v[29]);
        let eq12_e1359_d_n11: f64 = (eq12_e1357_d_n11 * s.v[29]);
        let eq12_e1359_d_n12: f64 = (eq12_e1357_d_n12 * s.v[29]);
        let eq12_e1359_d_n13: f64 = (eq12_e1357_d_n13 * s.v[29]);
        let eq12_e1359_d_n14: f64 = (eq12_e1357_d_n14 * s.v[29]);
        let eq12_e1359_d_n15: f64 = (eq12_e1357_d_n15 * s.v[29]);
        let eq12_e1359_d_n16: f64 = (eq12_e1357_d_n16 * s.v[29]);
        let eq12_e1359_d_b0: f64 = (eq12_e1357_d_b0 * s.v[29]);
        let eq12_e1359_d_b1: f64 = (eq12_e1357_d_b1 * s.v[29]);
        let eq12_e1359_d_b2: f64 = (eq12_e1357_d_b2 * s.v[29]);
        let eq12_e1359_d_b3: f64 = (eq12_e1357_d_b3 * s.v[29]);
        let eq12_e1359_d_b4: f64 = (eq12_e1357_d_b4 * s.v[29]);
        let eq12_e1359_d_b5: f64 = (eq12_e1357_d_b5 * s.v[29]);
        let eq12_e1359_d_b6: f64 = (eq12_e1357_d_b6 * s.v[29]);
        let eq12_e1359_d_b7: f64 = (eq12_e1357_d_b7 * s.v[29]);
        let eq12_e1359_d_b8: f64 = (eq12_e1357_d_b8 * s.v[29]);
        let eq12_e1359_d_b9: f64 = (eq12_e1357_d_b9 * s.v[29]);
        let eq12_e1359_d_b10: f64 = (eq12_e1357_d_b10 * s.v[29]);
        let eq12_e1359_d_b11: f64 = (eq12_e1357_d_b11 * s.v[29]);
        let eq12_e1359_d_b12: f64 = (eq12_e1357_d_b12 * s.v[29]);
        let eq12_e1359_d_b13: f64 = (eq12_e1357_d_b13 * s.v[29]);
        let eq12_e1361: f64 = (eq12_e1359 * p.p2);
        let eq12_e1361_d_n0: f64 = (eq12_e1359_d_n0 * p.p2);
        let eq12_e1361_d_n1: f64 = (eq12_e1359_d_n1 * p.p2);
        let eq12_e1361_d_n2: f64 = (eq12_e1359_d_n2 * p.p2);
        let eq12_e1361_d_n3: f64 = (eq12_e1359_d_n3 * p.p2);
        let eq12_e1361_d_n4: f64 = (eq12_e1359_d_n4 * p.p2);
        let eq12_e1361_d_n5: f64 = (eq12_e1359_d_n5 * p.p2);
        let eq12_e1361_d_n6: f64 = (eq12_e1359_d_n6 * p.p2);
        let eq12_e1361_d_n7: f64 = (eq12_e1359_d_n7 * p.p2);
        let eq12_e1361_d_n8: f64 = (eq12_e1359_d_n8 * p.p2);
        let eq12_e1361_d_n9: f64 = (eq12_e1359_d_n9 * p.p2);
        let eq12_e1361_d_n10: f64 = (eq12_e1359_d_n10 * p.p2);
        let eq12_e1361_d_n11: f64 = (eq12_e1359_d_n11 * p.p2);
        let eq12_e1361_d_n12: f64 = (eq12_e1359_d_n12 * p.p2);
        let eq12_e1361_d_n13: f64 = (eq12_e1359_d_n13 * p.p2);
        let eq12_e1361_d_n14: f64 = (eq12_e1359_d_n14 * p.p2);
        let eq12_e1361_d_n15: f64 = (eq12_e1359_d_n15 * p.p2);
        let eq12_e1361_d_n16: f64 = (eq12_e1359_d_n16 * p.p2);
        let eq12_e1361_d_b0: f64 = (eq12_e1359_d_b0 * p.p2);
        let eq12_e1361_d_b1: f64 = (eq12_e1359_d_b1 * p.p2);
        let eq12_e1361_d_b2: f64 = (eq12_e1359_d_b2 * p.p2);
        let eq12_e1361_d_b3: f64 = (eq12_e1359_d_b3 * p.p2);
        let eq12_e1361_d_b4: f64 = (eq12_e1359_d_b4 * p.p2);
        let eq12_e1361_d_b5: f64 = (eq12_e1359_d_b5 * p.p2);
        let eq12_e1361_d_b6: f64 = (eq12_e1359_d_b6 * p.p2);
        let eq12_e1361_d_b7: f64 = (eq12_e1359_d_b7 * p.p2);
        let eq12_e1361_d_b8: f64 = (eq12_e1359_d_b8 * p.p2);
        let eq12_e1361_d_b9: f64 = (eq12_e1359_d_b9 * p.p2);
        let eq12_e1361_d_b10: f64 = (eq12_e1359_d_b10 * p.p2);
        let eq12_e1361_d_b11: f64 = (eq12_e1359_d_b11 * p.p2);
        let eq12_e1361_d_b12: f64 = (eq12_e1359_d_b12 * p.p2);
        let eq12_e1361_d_b13: f64 = (eq12_e1359_d_b13 * p.p2);
        let eq12_e1363: f64 = (eq12_e1361 * s.v[30]);
        let eq12_e1363_d_n0: f64 = (eq12_e1361_d_n0 * s.v[30]);
        let eq12_e1363_d_n1: f64 = (eq12_e1361_d_n1 * s.v[30]);
        let eq12_e1363_d_n2: f64 = (eq12_e1361_d_n2 * s.v[30]);
        let eq12_e1363_d_n3: f64 = (eq12_e1361_d_n3 * s.v[30]);
        let eq12_e1363_d_n4: f64 = (eq12_e1361_d_n4 * s.v[30]);
        let eq12_e1363_d_n5: f64 = (eq12_e1361_d_n5 * s.v[30]);
        let eq12_e1363_d_n6: f64 = (eq12_e1361_d_n6 * s.v[30]);
        let eq12_e1363_d_n7: f64 = (eq12_e1361_d_n7 * s.v[30]);
        let eq12_e1363_d_n8: f64 = (eq12_e1361_d_n8 * s.v[30]);
        let eq12_e1363_d_n9: f64 = (eq12_e1361_d_n9 * s.v[30]);
        let eq12_e1363_d_n10: f64 = (eq12_e1361_d_n10 * s.v[30]);
        let eq12_e1363_d_n11: f64 = (eq12_e1361_d_n11 * s.v[30]);
        let eq12_e1363_d_n12: f64 = (eq12_e1361_d_n12 * s.v[30]);
        let eq12_e1363_d_n13: f64 = (eq12_e1361_d_n13 * s.v[30]);
        let eq12_e1363_d_n14: f64 = (eq12_e1361_d_n14 * s.v[30]);
        let eq12_e1363_d_n15: f64 = (eq12_e1361_d_n15 * s.v[30]);
        let eq12_e1363_d_n16: f64 = (eq12_e1361_d_n16 * s.v[30]);
        let eq12_e1363_d_b0: f64 = (eq12_e1361_d_b0 * s.v[30]);
        let eq12_e1363_d_b1: f64 = (eq12_e1361_d_b1 * s.v[30]);
        let eq12_e1363_d_b2: f64 = (eq12_e1361_d_b2 * s.v[30]);
        let eq12_e1363_d_b3: f64 = (eq12_e1361_d_b3 * s.v[30]);
        let eq12_e1363_d_b4: f64 = (eq12_e1361_d_b4 * s.v[30]);
        let eq12_e1363_d_b5: f64 = (eq12_e1361_d_b5 * s.v[30]);
        let eq12_e1363_d_b6: f64 = (eq12_e1361_d_b6 * s.v[30]);
        let eq12_e1363_d_b7: f64 = (eq12_e1361_d_b7 * s.v[30]);
        let eq12_e1363_d_b8: f64 = (eq12_e1361_d_b8 * s.v[30]);
        let eq12_e1363_d_b9: f64 = (eq12_e1361_d_b9 * s.v[30]);
        let eq12_e1363_d_b10: f64 = (eq12_e1361_d_b10 * s.v[30]);
        let eq12_e1363_d_b11: f64 = (eq12_e1361_d_b11 * s.v[30]);
        let eq12_e1363_d_b12: f64 = (eq12_e1361_d_b12 * s.v[30]);
        let eq12_e1363_d_b13: f64 = (eq12_e1361_d_b13 * s.v[30]);
        let eq12_e1365: f64 = (eq12_e1363 * (nv15 - 0.0));
        let eq12_e1365_d_n0: f64 = (eq12_e1363_d_n0 * (nv15 - 0.0));
        let eq12_e1365_d_n1: f64 = (eq12_e1363_d_n1 * (nv15 - 0.0));
        let eq12_e1365_d_n2: f64 = (eq12_e1363_d_n2 * (nv15 - 0.0));
        let eq12_e1365_d_n3: f64 = (eq12_e1363_d_n3 * (nv15 - 0.0));
        let eq12_e1365_d_n4: f64 = (eq12_e1363_d_n4 * (nv15 - 0.0));
        let eq12_e1365_d_n5: f64 = (eq12_e1363_d_n5 * (nv15 - 0.0));
        let eq12_e1365_d_n6: f64 = (eq12_e1363_d_n6 * (nv15 - 0.0));
        let eq12_e1365_d_n7: f64 = (eq12_e1363_d_n7 * (nv15 - 0.0));
        let eq12_e1365_d_n8: f64 = (eq12_e1363_d_n8 * (nv15 - 0.0));
        let eq12_e1365_d_n9: f64 = (eq12_e1363_d_n9 * (nv15 - 0.0));
        let eq12_e1365_d_n10: f64 = (eq12_e1363_d_n10 * (nv15 - 0.0));
        let eq12_e1365_d_n11: f64 = (eq12_e1363_d_n11 * (nv15 - 0.0));
        let eq12_e1365_d_n12: f64 = (eq12_e1363_d_n12 * (nv15 - 0.0));
        let eq12_e1365_d_n13: f64 = (eq12_e1363_d_n13 * (nv15 - 0.0));
        let eq12_e1365_d_n14: f64 = (eq12_e1363_d_n14 * (nv15 - 0.0));
        let eq12_e1365_d_n15: f64 = ((eq12_e1363_d_n15 * (nv15 - 0.0)) + eq12_e1363);
        let eq12_e1365_d_n16: f64 = (eq12_e1363_d_n16 * (nv15 - 0.0));
        let eq12_e1365_d_b0: f64 = (eq12_e1363_d_b0 * (nv15 - 0.0));
        let eq12_e1365_d_b1: f64 = (eq12_e1363_d_b1 * (nv15 - 0.0));
        let eq12_e1365_d_b2: f64 = (eq12_e1363_d_b2 * (nv15 - 0.0));
        let eq12_e1365_d_b3: f64 = (eq12_e1363_d_b3 * (nv15 - 0.0));
        let eq12_e1365_d_b4: f64 = (eq12_e1363_d_b4 * (nv15 - 0.0));
        let eq12_e1365_d_b5: f64 = (eq12_e1363_d_b5 * (nv15 - 0.0));
        let eq12_e1365_d_b6: f64 = (eq12_e1363_d_b6 * (nv15 - 0.0));
        let eq12_e1365_d_b7: f64 = (eq12_e1363_d_b7 * (nv15 - 0.0));
        let eq12_e1365_d_b8: f64 = (eq12_e1363_d_b8 * (nv15 - 0.0));
        let eq12_e1365_d_b9: f64 = (eq12_e1363_d_b9 * (nv15 - 0.0));
        let eq12_e1365_d_b10: f64 = (eq12_e1363_d_b10 * (nv15 - 0.0));
        let eq12_e1365_d_b11: f64 = (eq12_e1363_d_b11 * (nv15 - 0.0));
        let eq12_e1365_d_b12: f64 = (eq12_e1363_d_b12 * (nv15 - 0.0));
        let eq12_e1365_d_b13: f64 = (eq12_e1363_d_b13 * (nv15 - 0.0));
        let eq12_e1366: f64 = (0.5 * eq12_e1365);
        let eq12_e1366_d_n0: f64 = (0.5 * eq12_e1365_d_n0);
        let eq12_e1366_d_n1: f64 = (0.5 * eq12_e1365_d_n1);
        let eq12_e1366_d_n2: f64 = (0.5 * eq12_e1365_d_n2);
        let eq12_e1366_d_n3: f64 = (0.5 * eq12_e1365_d_n3);
        let eq12_e1366_d_n4: f64 = (0.5 * eq12_e1365_d_n4);
        let eq12_e1366_d_n5: f64 = (0.5 * eq12_e1365_d_n5);
        let eq12_e1366_d_n6: f64 = (0.5 * eq12_e1365_d_n6);
        let eq12_e1366_d_n7: f64 = (0.5 * eq12_e1365_d_n7);
        let eq12_e1366_d_n8: f64 = (0.5 * eq12_e1365_d_n8);
        let eq12_e1366_d_n9: f64 = (0.5 * eq12_e1365_d_n9);
        let eq12_e1366_d_n10: f64 = (0.5 * eq12_e1365_d_n10);
        let eq12_e1366_d_n11: f64 = (0.5 * eq12_e1365_d_n11);
        let eq12_e1366_d_n12: f64 = (0.5 * eq12_e1365_d_n12);
        let eq12_e1366_d_n13: f64 = (0.5 * eq12_e1365_d_n13);
        let eq12_e1366_d_n14: f64 = (0.5 * eq12_e1365_d_n14);
        let eq12_e1366_d_n15: f64 = (0.5 * eq12_e1365_d_n15);
        let eq12_e1366_d_n16: f64 = (0.5 * eq12_e1365_d_n16);
        let eq12_e1366_d_b0: f64 = (0.5 * eq12_e1365_d_b0);
        let eq12_e1366_d_b1: f64 = (0.5 * eq12_e1365_d_b1);
        let eq12_e1366_d_b2: f64 = (0.5 * eq12_e1365_d_b2);
        let eq12_e1366_d_b3: f64 = (0.5 * eq12_e1365_d_b3);
        let eq12_e1366_d_b4: f64 = (0.5 * eq12_e1365_d_b4);
        let eq12_e1366_d_b5: f64 = (0.5 * eq12_e1365_d_b5);
        let eq12_e1366_d_b6: f64 = (0.5 * eq12_e1365_d_b6);
        let eq12_e1366_d_b7: f64 = (0.5 * eq12_e1365_d_b7);
        let eq12_e1366_d_b8: f64 = (0.5 * eq12_e1365_d_b8);
        let eq12_e1366_d_b9: f64 = (0.5 * eq12_e1365_d_b9);
        let eq12_e1366_d_b10: f64 = (0.5 * eq12_e1365_d_b10);
        let eq12_e1366_d_b11: f64 = (0.5 * eq12_e1365_d_b11);
        let eq12_e1366_d_b12: f64 = (0.5 * eq12_e1365_d_b12);
        let eq12_e1366_d_b13: f64 = (0.5 * eq12_e1365_d_b13);
        let eq12_e1367_q: f64 = eq12_e1366;
        let eq12_e1368: f64 = (p.p29 * eq12_e1366);
        let eq12_e1368_d_n0: f64 = (p.p29 * eq12_e1366_d_n0);
        let eq12_e1368_d_n1: f64 = (p.p29 * eq12_e1366_d_n1);
        let eq12_e1368_d_n2: f64 = (p.p29 * eq12_e1366_d_n2);
        let eq12_e1368_d_n3: f64 = (p.p29 * eq12_e1366_d_n3);
        let eq12_e1368_d_n4: f64 = (p.p29 * eq12_e1366_d_n4);
        let eq12_e1368_d_n5: f64 = (p.p29 * eq12_e1366_d_n5);
        let eq12_e1368_d_n6: f64 = (p.p29 * eq12_e1366_d_n6);
        let eq12_e1368_d_n7: f64 = (p.p29 * eq12_e1366_d_n7);
        let eq12_e1368_d_n8: f64 = (p.p29 * eq12_e1366_d_n8);
        let eq12_e1368_d_n9: f64 = (p.p29 * eq12_e1366_d_n9);
        let eq12_e1368_d_n10: f64 = (p.p29 * eq12_e1366_d_n10);
        let eq12_e1368_d_n11: f64 = (p.p29 * eq12_e1366_d_n11);
        let eq12_e1368_d_n12: f64 = (p.p29 * eq12_e1366_d_n12);
        let eq12_e1368_d_n13: f64 = (p.p29 * eq12_e1366_d_n13);
        let eq12_e1368_d_n14: f64 = (p.p29 * eq12_e1366_d_n14);
        let eq12_e1368_d_n15: f64 = (p.p29 * eq12_e1366_d_n15);
        let eq12_e1368_d_n16: f64 = (p.p29 * eq12_e1366_d_n16);
        let eq12_e1368_d_b0: f64 = (p.p29 * eq12_e1366_d_b0);
        let eq12_e1368_d_b1: f64 = (p.p29 * eq12_e1366_d_b1);
        let eq12_e1368_d_b2: f64 = (p.p29 * eq12_e1366_d_b2);
        let eq12_e1368_d_b3: f64 = (p.p29 * eq12_e1366_d_b3);
        let eq12_e1368_d_b4: f64 = (p.p29 * eq12_e1366_d_b4);
        let eq12_e1368_d_b5: f64 = (p.p29 * eq12_e1366_d_b5);
        let eq12_e1368_d_b6: f64 = (p.p29 * eq12_e1366_d_b6);
        let eq12_e1368_d_b7: f64 = (p.p29 * eq12_e1366_d_b7);
        let eq12_e1368_d_b8: f64 = (p.p29 * eq12_e1366_d_b8);
        let eq12_e1368_d_b9: f64 = (p.p29 * eq12_e1366_d_b9);
        let eq12_e1368_d_b10: f64 = (p.p29 * eq12_e1366_d_b10);
        let eq12_e1368_d_b11: f64 = (p.p29 * eq12_e1366_d_b11);
        let eq12_e1368_d_b12: f64 = (p.p29 * eq12_e1366_d_b12);
        let eq12_e1368_d_b13: f64 = (p.p29 * eq12_e1366_d_b13);
        let eq12_e1368_q: f64 = (p.p29 * eq12_e1367_q);
        let eq12_e1368_q_d_n0: f64 = (p.p29 * eq12_e1366_d_n0);
        let eq12_e1368_q_d_n1: f64 = (p.p29 * eq12_e1366_d_n1);
        let eq12_e1368_q_d_n2: f64 = (p.p29 * eq12_e1366_d_n2);
        let eq12_e1368_q_d_n3: f64 = (p.p29 * eq12_e1366_d_n3);
        let eq12_e1368_q_d_n4: f64 = (p.p29 * eq12_e1366_d_n4);
        let eq12_e1368_q_d_n5: f64 = (p.p29 * eq12_e1366_d_n5);
        let eq12_e1368_q_d_n6: f64 = (p.p29 * eq12_e1366_d_n6);
        let eq12_e1368_q_d_n7: f64 = (p.p29 * eq12_e1366_d_n7);
        let eq12_e1368_q_d_n8: f64 = (p.p29 * eq12_e1366_d_n8);
        let eq12_e1368_q_d_n9: f64 = (p.p29 * eq12_e1366_d_n9);
        let eq12_e1368_q_d_n10: f64 = (p.p29 * eq12_e1366_d_n10);
        let eq12_e1368_q_d_n11: f64 = (p.p29 * eq12_e1366_d_n11);
        let eq12_e1368_q_d_n12: f64 = (p.p29 * eq12_e1366_d_n12);
        let eq12_e1368_q_d_n13: f64 = (p.p29 * eq12_e1366_d_n13);
        let eq12_e1368_q_d_n14: f64 = (p.p29 * eq12_e1366_d_n14);
        let eq12_e1368_q_d_n15: f64 = (p.p29 * eq12_e1366_d_n15);
        let eq12_e1368_q_d_n16: f64 = (p.p29 * eq12_e1366_d_n16);
        let eq12_e1368_q_d_b0: f64 = (p.p29 * eq12_e1366_d_b0);
        let eq12_e1368_q_d_b1: f64 = (p.p29 * eq12_e1366_d_b1);
        let eq12_e1368_q_d_b2: f64 = (p.p29 * eq12_e1366_d_b2);
        let eq12_e1368_q_d_b3: f64 = (p.p29 * eq12_e1366_d_b3);
        let eq12_e1368_q_d_b4: f64 = (p.p29 * eq12_e1366_d_b4);
        let eq12_e1368_q_d_b5: f64 = (p.p29 * eq12_e1366_d_b5);
        let eq12_e1368_q_d_b6: f64 = (p.p29 * eq12_e1366_d_b6);
        let eq12_e1368_q_d_b7: f64 = (p.p29 * eq12_e1366_d_b7);
        let eq12_e1368_q_d_b8: f64 = (p.p29 * eq12_e1366_d_b8);
        let eq12_e1368_q_d_b9: f64 = (p.p29 * eq12_e1366_d_b9);
        let eq12_e1368_q_d_b10: f64 = (p.p29 * eq12_e1366_d_b10);
        let eq12_e1368_q_d_b11: f64 = (p.p29 * eq12_e1366_d_b11);
        let eq12_e1368_q_d_b12: f64 = (p.p29 * eq12_e1366_d_b12);
        let eq12_e1368_q_d_b13: f64 = (p.p29 * eq12_e1366_d_b13);
        (eq12_e1368, eq12_e1368_d_n0, eq12_e1368_d_n1, eq12_e1368_d_n2, eq12_e1368_d_n3, eq12_e1368_d_n4, eq12_e1368_d_n5, eq12_e1368_d_n6, eq12_e1368_d_n7, eq12_e1368_d_n8, eq12_e1368_d_n9, eq12_e1368_d_n10, eq12_e1368_d_n11, eq12_e1368_d_n12, eq12_e1368_d_n13, eq12_e1368_d_n14, eq12_e1368_d_n15, eq12_e1368_d_n16, eq12_e1368_d_b0, eq12_e1368_d_b1, eq12_e1368_d_b2, eq12_e1368_d_b3, eq12_e1368_d_b4, eq12_e1368_d_b5, eq12_e1368_d_b6, eq12_e1368_d_b7, eq12_e1368_d_b8, eq12_e1368_d_b9, eq12_e1368_d_b10, eq12_e1368_d_b11, eq12_e1368_d_b12, eq12_e1368_d_b13, eq12_e1368_q, eq12_e1368_q_d_n0, eq12_e1368_q_d_n1, eq12_e1368_q_d_n2, eq12_e1368_q_d_n3, eq12_e1368_q_d_n4, eq12_e1368_q_d_n5, eq12_e1368_q_d_n6, eq12_e1368_q_d_n7, eq12_e1368_q_d_n8, eq12_e1368_q_d_n9, eq12_e1368_q_d_n10, eq12_e1368_q_d_n11, eq12_e1368_q_d_n12, eq12_e1368_q_d_n13, eq12_e1368_q_d_n14, eq12_e1368_q_d_n15, eq12_e1368_q_d_n16, eq12_e1368_q_d_b0, eq12_e1368_q_d_b1, eq12_e1368_q_d_b2, eq12_e1368_q_d_b3, eq12_e1368_q_d_b4, eq12_e1368_q_d_b5, eq12_e1368_q_d_b6, eq12_e1368_q_d_b7, eq12_e1368_q_d_b8, eq12_e1368_q_d_b9, eq12_e1368_q_d_b10, eq12_e1368_q_d_b11, eq12_e1368_q_d_b12, eq12_e1368_q_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq12_reactive_node_derivatives: [f64; 17] = [eq12_e1370_q_d_n0, eq12_e1370_q_d_n1, eq12_e1370_q_d_n2, eq12_e1370_q_d_n3, eq12_e1370_q_d_n4, eq12_e1370_q_d_n5, eq12_e1370_q_d_n6, eq12_e1370_q_d_n7, eq12_e1370_q_d_n8, eq12_e1370_q_d_n9, eq12_e1370_q_d_n10, eq12_e1370_q_d_n11, eq12_e1370_q_d_n12, eq12_e1370_q_d_n13, eq12_e1370_q_d_n14, eq12_e1370_q_d_n15, eq12_e1370_q_d_n16];
        let eq12_reactive_branch_derivatives: [f64; 14] = [eq12_e1370_q_d_b0, eq12_e1370_q_d_b1, eq12_e1370_q_d_b2, eq12_e1370_q_d_b3, eq12_e1370_q_d_b4, eq12_e1370_q_d_b5, eq12_e1370_q_d_b6, eq12_e1370_q_d_b7, eq12_e1370_q_d_b8, eq12_e1370_q_d_b9, eq12_e1370_q_d_b10, eq12_e1370_q_d_b11, eq12_e1370_q_d_b12, eq12_e1370_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[5]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e1428_q: f64 = s.v[787];
        let eq19_reactive_node_derivatives: [f64; 17] = [s.dn[787][0], s.dn[787][1], s.dn[787][2], s.dn[787][3], s.dn[787][4], s.dn[787][5], s.dn[787][6], s.dn[787][7], s.dn[787][8], s.dn[787][9], s.dn[787][10], s.dn[787][11], s.dn[787][12], s.dn[787][13], s.dn[787][14], s.dn[787][15], s.dn[787][16]];
        let eq19_reactive_branch_derivatives: [f64; 14] = [s.db[787][0], s.db[787][1], s.db[787][2], s.db[787][3], s.db[787][4], s.db[787][5], s.db[787][6], s.db[787][7], s.db[787][8], s.db[787][9], s.db[787][10], s.db[787][11], s.db[787][12], s.db[787][13]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[11]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let eq20_e1430_q: f64 = s.v[785];
        let eq20_reactive_node_derivatives: [f64; 17] = [s.dn[785][0], s.dn[785][1], s.dn[785][2], s.dn[785][3], s.dn[785][4], s.dn[785][5], s.dn[785][6], s.dn[785][7], s.dn[785][8], s.dn[785][9], s.dn[785][10], s.dn[785][11], s.dn[785][12], s.dn[785][13], s.dn[785][14], s.dn[785][15], s.dn[785][16]];
        let eq20_reactive_branch_derivatives: [f64; 14] = [s.db[785][0], s.db[785][1], s.db[785][2], s.db[785][3], s.db[785][4], s.db[785][5], s.db[785][6], s.db[785][7], s.db[785][8], s.db[785][9], s.db[785][10], s.db[785][11], s.db[785][12], s.db[785][13]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &eq20_reactive_branch_derivatives,
            multiplicity,
        );
        let eq21_e1432_q: f64 = s.v[786];
        let eq21_reactive_node_derivatives: [f64; 17] = [s.dn[786][0], s.dn[786][1], s.dn[786][2], s.dn[786][3], s.dn[786][4], s.dn[786][5], s.dn[786][6], s.dn[786][7], s.dn[786][8], s.dn[786][9], s.dn[786][10], s.dn[786][11], s.dn[786][12], s.dn[786][13], s.dn[786][14], s.dn[786][15], s.dn[786][16]];
        let eq21_reactive_branch_derivatives: [f64; 14] = [s.db[786][0], s.db[786][1], s.db[786][2], s.db[786][3], s.db[786][4], s.db[786][5], s.db[786][6], s.db[786][7], s.db[786][8], s.db[786][9], s.db[786][10], s.db[786][11], s.db[786][12], s.db[786][13]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[11]),
            nodes,
            &eq21_reactive_node_derivatives,
            branches,
            &eq21_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_2(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let eq22_e1435: f64 = (-s.v[187]);
        let eq22_e1435_d_n0: f64 = (-s.dn[187][0]);
        let eq22_e1435_d_n1: f64 = (-s.dn[187][1]);
        let eq22_e1435_d_n2: f64 = (-s.dn[187][2]);
        let eq22_e1435_d_n3: f64 = (-s.dn[187][3]);
        let eq22_e1435_d_n4: f64 = (-s.dn[187][4]);
        let eq22_e1435_d_n5: f64 = (-s.dn[187][5]);
        let eq22_e1435_d_n6: f64 = (-s.dn[187][6]);
        let eq22_e1435_d_n7: f64 = (-s.dn[187][7]);
        let eq22_e1435_d_n8: f64 = (-s.dn[187][8]);
        let eq22_e1435_d_n9: f64 = (-s.dn[187][9]);
        let eq22_e1435_d_n10: f64 = (-s.dn[187][10]);
        let eq22_e1435_d_n11: f64 = (-s.dn[187][11]);
        let eq22_e1435_d_n12: f64 = (-s.dn[187][12]);
        let eq22_e1435_d_n13: f64 = (-s.dn[187][13]);
        let eq22_e1435_d_n14: f64 = (-s.dn[187][14]);
        let eq22_e1435_d_n15: f64 = (-s.dn[187][15]);
        let eq22_e1435_d_n16: f64 = (-s.dn[187][16]);
        let eq22_e1435_d_b0: f64 = (-s.db[187][0]);
        let eq22_e1435_d_b1: f64 = (-s.db[187][1]);
        let eq22_e1435_d_b2: f64 = (-s.db[187][2]);
        let eq22_e1435_d_b3: f64 = (-s.db[187][3]);
        let eq22_e1435_d_b4: f64 = (-s.db[187][4]);
        let eq22_e1435_d_b5: f64 = (-s.db[187][5]);
        let eq22_e1435_d_b6: f64 = (-s.db[187][6]);
        let eq22_e1435_d_b7: f64 = (-s.db[187][7]);
        let eq22_e1435_d_b8: f64 = (-s.db[187][8]);
        let eq22_e1435_d_b9: f64 = (-s.db[187][9]);
        let eq22_e1435_d_b10: f64 = (-s.db[187][10]);
        let eq22_e1435_d_b11: f64 = (-s.db[187][11]);
        let eq22_e1435_d_b12: f64 = (-s.db[187][12]);
        let eq22_e1435_d_b13: f64 = (-s.db[187][13]);
        let eq22_e1437: f64 = (eq22_e1435 * s.v[223]);
        let eq22_e1437_d_n0: f64 = ((eq22_e1435_d_n0 * s.v[223]) + (eq22_e1435 * s.dn[223][0]));
        let eq22_e1437_d_n1: f64 = ((eq22_e1435_d_n1 * s.v[223]) + (eq22_e1435 * s.dn[223][1]));
        let eq22_e1437_d_n2: f64 = ((eq22_e1435_d_n2 * s.v[223]) + (eq22_e1435 * s.dn[223][2]));
        let eq22_e1437_d_n3: f64 = ((eq22_e1435_d_n3 * s.v[223]) + (eq22_e1435 * s.dn[223][3]));
        let eq22_e1437_d_n4: f64 = ((eq22_e1435_d_n4 * s.v[223]) + (eq22_e1435 * s.dn[223][4]));
        let eq22_e1437_d_n5: f64 = ((eq22_e1435_d_n5 * s.v[223]) + (eq22_e1435 * s.dn[223][5]));
        let eq22_e1437_d_n6: f64 = ((eq22_e1435_d_n6 * s.v[223]) + (eq22_e1435 * s.dn[223][6]));
        let eq22_e1437_d_n7: f64 = ((eq22_e1435_d_n7 * s.v[223]) + (eq22_e1435 * s.dn[223][7]));
        let eq22_e1437_d_n8: f64 = ((eq22_e1435_d_n8 * s.v[223]) + (eq22_e1435 * s.dn[223][8]));
        let eq22_e1437_d_n9: f64 = ((eq22_e1435_d_n9 * s.v[223]) + (eq22_e1435 * s.dn[223][9]));
        let eq22_e1437_d_n10: f64 = ((eq22_e1435_d_n10 * s.v[223]) + (eq22_e1435 * s.dn[223][10]));
        let eq22_e1437_d_n11: f64 = ((eq22_e1435_d_n11 * s.v[223]) + (eq22_e1435 * s.dn[223][11]));
        let eq22_e1437_d_n12: f64 = ((eq22_e1435_d_n12 * s.v[223]) + (eq22_e1435 * s.dn[223][12]));
        let eq22_e1437_d_n13: f64 = ((eq22_e1435_d_n13 * s.v[223]) + (eq22_e1435 * s.dn[223][13]));
        let eq22_e1437_d_n14: f64 = ((eq22_e1435_d_n14 * s.v[223]) + (eq22_e1435 * s.dn[223][14]));
        let eq22_e1437_d_n15: f64 = ((eq22_e1435_d_n15 * s.v[223]) + (eq22_e1435 * s.dn[223][15]));
        let eq22_e1437_d_n16: f64 = ((eq22_e1435_d_n16 * s.v[223]) + (eq22_e1435 * s.dn[223][16]));
        let eq22_e1437_d_b0: f64 = ((eq22_e1435_d_b0 * s.v[223]) + (eq22_e1435 * s.db[223][0]));
        let eq22_e1437_d_b1: f64 = ((eq22_e1435_d_b1 * s.v[223]) + (eq22_e1435 * s.db[223][1]));
        let eq22_e1437_d_b2: f64 = ((eq22_e1435_d_b2 * s.v[223]) + (eq22_e1435 * s.db[223][2]));
        let eq22_e1437_d_b3: f64 = ((eq22_e1435_d_b3 * s.v[223]) + (eq22_e1435 * s.db[223][3]));
        let eq22_e1437_d_b4: f64 = ((eq22_e1435_d_b4 * s.v[223]) + (eq22_e1435 * s.db[223][4]));
        let eq22_e1437_d_b5: f64 = ((eq22_e1435_d_b5 * s.v[223]) + (eq22_e1435 * s.db[223][5]));
        let eq22_e1437_d_b6: f64 = ((eq22_e1435_d_b6 * s.v[223]) + (eq22_e1435 * s.db[223][6]));
        let eq22_e1437_d_b7: f64 = ((eq22_e1435_d_b7 * s.v[223]) + (eq22_e1435 * s.db[223][7]));
        let eq22_e1437_d_b8: f64 = ((eq22_e1435_d_b8 * s.v[223]) + (eq22_e1435 * s.db[223][8]));
        let eq22_e1437_d_b9: f64 = ((eq22_e1435_d_b9 * s.v[223]) + (eq22_e1435 * s.db[223][9]));
        let eq22_e1437_d_b10: f64 = ((eq22_e1435_d_b10 * s.v[223]) + (eq22_e1435 * s.db[223][10]));
        let eq22_e1437_d_b11: f64 = ((eq22_e1435_d_b11 * s.v[223]) + (eq22_e1435 * s.db[223][11]));
        let eq22_e1437_d_b12: f64 = ((eq22_e1435_d_b12 * s.v[223]) + (eq22_e1435 * s.db[223][12]));
        let eq22_e1437_d_b13: f64 = ((eq22_e1435_d_b13 * s.v[223]) + (eq22_e1435 * s.db[223][13]));
        let eq22_e1438_q: f64 = eq22_e1437;
        let eq22_e1439: f64 = (p.p29 * eq22_e1437);
        let eq22_e1439_d_n0: f64 = (p.p29 * eq22_e1437_d_n0);
        let eq22_e1439_d_n1: f64 = (p.p29 * eq22_e1437_d_n1);
        let eq22_e1439_d_n2: f64 = (p.p29 * eq22_e1437_d_n2);
        let eq22_e1439_d_n3: f64 = (p.p29 * eq22_e1437_d_n3);
        let eq22_e1439_d_n4: f64 = (p.p29 * eq22_e1437_d_n4);
        let eq22_e1439_d_n5: f64 = (p.p29 * eq22_e1437_d_n5);
        let eq22_e1439_d_n6: f64 = (p.p29 * eq22_e1437_d_n6);
        let eq22_e1439_d_n7: f64 = (p.p29 * eq22_e1437_d_n7);
        let eq22_e1439_d_n8: f64 = (p.p29 * eq22_e1437_d_n8);
        let eq22_e1439_d_n9: f64 = (p.p29 * eq22_e1437_d_n9);
        let eq22_e1439_d_n10: f64 = (p.p29 * eq22_e1437_d_n10);
        let eq22_e1439_d_n11: f64 = (p.p29 * eq22_e1437_d_n11);
        let eq22_e1439_d_n12: f64 = (p.p29 * eq22_e1437_d_n12);
        let eq22_e1439_d_n13: f64 = (p.p29 * eq22_e1437_d_n13);
        let eq22_e1439_d_n14: f64 = (p.p29 * eq22_e1437_d_n14);
        let eq22_e1439_d_n15: f64 = (p.p29 * eq22_e1437_d_n15);
        let eq22_e1439_d_n16: f64 = (p.p29 * eq22_e1437_d_n16);
        let eq22_e1439_d_b0: f64 = (p.p29 * eq22_e1437_d_b0);
        let eq22_e1439_d_b1: f64 = (p.p29 * eq22_e1437_d_b1);
        let eq22_e1439_d_b2: f64 = (p.p29 * eq22_e1437_d_b2);
        let eq22_e1439_d_b3: f64 = (p.p29 * eq22_e1437_d_b3);
        let eq22_e1439_d_b4: f64 = (p.p29 * eq22_e1437_d_b4);
        let eq22_e1439_d_b5: f64 = (p.p29 * eq22_e1437_d_b5);
        let eq22_e1439_d_b6: f64 = (p.p29 * eq22_e1437_d_b6);
        let eq22_e1439_d_b7: f64 = (p.p29 * eq22_e1437_d_b7);
        let eq22_e1439_d_b8: f64 = (p.p29 * eq22_e1437_d_b8);
        let eq22_e1439_d_b9: f64 = (p.p29 * eq22_e1437_d_b9);
        let eq22_e1439_d_b10: f64 = (p.p29 * eq22_e1437_d_b10);
        let eq22_e1439_d_b11: f64 = (p.p29 * eq22_e1437_d_b11);
        let eq22_e1439_d_b12: f64 = (p.p29 * eq22_e1437_d_b12);
        let eq22_e1439_d_b13: f64 = (p.p29 * eq22_e1437_d_b13);
        let eq22_e1439_q: f64 = (p.p29 * eq22_e1438_q);
        let eq22_e1439_q_d_n0: f64 = (p.p29 * eq22_e1437_d_n0);
        let eq22_e1439_q_d_n1: f64 = (p.p29 * eq22_e1437_d_n1);
        let eq22_e1439_q_d_n2: f64 = (p.p29 * eq22_e1437_d_n2);
        let eq22_e1439_q_d_n3: f64 = (p.p29 * eq22_e1437_d_n3);
        let eq22_e1439_q_d_n4: f64 = (p.p29 * eq22_e1437_d_n4);
        let eq22_e1439_q_d_n5: f64 = (p.p29 * eq22_e1437_d_n5);
        let eq22_e1439_q_d_n6: f64 = (p.p29 * eq22_e1437_d_n6);
        let eq22_e1439_q_d_n7: f64 = (p.p29 * eq22_e1437_d_n7);
        let eq22_e1439_q_d_n8: f64 = (p.p29 * eq22_e1437_d_n8);
        let eq22_e1439_q_d_n9: f64 = (p.p29 * eq22_e1437_d_n9);
        let eq22_e1439_q_d_n10: f64 = (p.p29 * eq22_e1437_d_n10);
        let eq22_e1439_q_d_n11: f64 = (p.p29 * eq22_e1437_d_n11);
        let eq22_e1439_q_d_n12: f64 = (p.p29 * eq22_e1437_d_n12);
        let eq22_e1439_q_d_n13: f64 = (p.p29 * eq22_e1437_d_n13);
        let eq22_e1439_q_d_n14: f64 = (p.p29 * eq22_e1437_d_n14);
        let eq22_e1439_q_d_n15: f64 = (p.p29 * eq22_e1437_d_n15);
        let eq22_e1439_q_d_n16: f64 = (p.p29 * eq22_e1437_d_n16);
        let eq22_e1439_q_d_b0: f64 = (p.p29 * eq22_e1437_d_b0);
        let eq22_e1439_q_d_b1: f64 = (p.p29 * eq22_e1437_d_b1);
        let eq22_e1439_q_d_b2: f64 = (p.p29 * eq22_e1437_d_b2);
        let eq22_e1439_q_d_b3: f64 = (p.p29 * eq22_e1437_d_b3);
        let eq22_e1439_q_d_b4: f64 = (p.p29 * eq22_e1437_d_b4);
        let eq22_e1439_q_d_b5: f64 = (p.p29 * eq22_e1437_d_b5);
        let eq22_e1439_q_d_b6: f64 = (p.p29 * eq22_e1437_d_b6);
        let eq22_e1439_q_d_b7: f64 = (p.p29 * eq22_e1437_d_b7);
        let eq22_e1439_q_d_b8: f64 = (p.p29 * eq22_e1437_d_b8);
        let eq22_e1439_q_d_b9: f64 = (p.p29 * eq22_e1437_d_b9);
        let eq22_e1439_q_d_b10: f64 = (p.p29 * eq22_e1437_d_b10);
        let eq22_e1439_q_d_b11: f64 = (p.p29 * eq22_e1437_d_b11);
        let eq22_e1439_q_d_b12: f64 = (p.p29 * eq22_e1437_d_b12);
        let eq22_e1439_q_d_b13: f64 = (p.p29 * eq22_e1437_d_b13);
        let eq22_reactive_node_derivatives: [f64; 17] = [eq22_e1439_q_d_n0, eq22_e1439_q_d_n1, eq22_e1439_q_d_n2, eq22_e1439_q_d_n3, eq22_e1439_q_d_n4, eq22_e1439_q_d_n5, eq22_e1439_q_d_n6, eq22_e1439_q_d_n7, eq22_e1439_q_d_n8, eq22_e1439_q_d_n9, eq22_e1439_q_d_n10, eq22_e1439_q_d_n11, eq22_e1439_q_d_n12, eq22_e1439_q_d_n13, eq22_e1439_q_d_n14, eq22_e1439_q_d_n15, eq22_e1439_q_d_n16];
        let eq22_reactive_branch_derivatives: [f64; 14] = [eq22_e1439_q_d_b0, eq22_e1439_q_d_b1, eq22_e1439_q_d_b2, eq22_e1439_q_d_b3, eq22_e1439_q_d_b4, eq22_e1439_q_d_b5, eq22_e1439_q_d_b6, eq22_e1439_q_d_b7, eq22_e1439_q_d_b8, eq22_e1439_q_d_b9, eq22_e1439_q_d_b10, eq22_e1439_q_d_b11, eq22_e1439_q_d_b12, eq22_e1439_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            nodes,
            &eq22_reactive_node_derivatives,
            branches,
            &eq22_reactive_branch_derivatives,
            multiplicity,
        );
        let eq23_e1442: f64 = (-s.v[187]);
        let eq23_e1442_d_n0: f64 = (-s.dn[187][0]);
        let eq23_e1442_d_n1: f64 = (-s.dn[187][1]);
        let eq23_e1442_d_n2: f64 = (-s.dn[187][2]);
        let eq23_e1442_d_n3: f64 = (-s.dn[187][3]);
        let eq23_e1442_d_n4: f64 = (-s.dn[187][4]);
        let eq23_e1442_d_n5: f64 = (-s.dn[187][5]);
        let eq23_e1442_d_n6: f64 = (-s.dn[187][6]);
        let eq23_e1442_d_n7: f64 = (-s.dn[187][7]);
        let eq23_e1442_d_n8: f64 = (-s.dn[187][8]);
        let eq23_e1442_d_n9: f64 = (-s.dn[187][9]);
        let eq23_e1442_d_n10: f64 = (-s.dn[187][10]);
        let eq23_e1442_d_n11: f64 = (-s.dn[187][11]);
        let eq23_e1442_d_n12: f64 = (-s.dn[187][12]);
        let eq23_e1442_d_n13: f64 = (-s.dn[187][13]);
        let eq23_e1442_d_n14: f64 = (-s.dn[187][14]);
        let eq23_e1442_d_n15: f64 = (-s.dn[187][15]);
        let eq23_e1442_d_n16: f64 = (-s.dn[187][16]);
        let eq23_e1442_d_b0: f64 = (-s.db[187][0]);
        let eq23_e1442_d_b1: f64 = (-s.db[187][1]);
        let eq23_e1442_d_b2: f64 = (-s.db[187][2]);
        let eq23_e1442_d_b3: f64 = (-s.db[187][3]);
        let eq23_e1442_d_b4: f64 = (-s.db[187][4]);
        let eq23_e1442_d_b5: f64 = (-s.db[187][5]);
        let eq23_e1442_d_b6: f64 = (-s.db[187][6]);
        let eq23_e1442_d_b7: f64 = (-s.db[187][7]);
        let eq23_e1442_d_b8: f64 = (-s.db[187][8]);
        let eq23_e1442_d_b9: f64 = (-s.db[187][9]);
        let eq23_e1442_d_b10: f64 = (-s.db[187][10]);
        let eq23_e1442_d_b11: f64 = (-s.db[187][11]);
        let eq23_e1442_d_b12: f64 = (-s.db[187][12]);
        let eq23_e1442_d_b13: f64 = (-s.db[187][13]);
        let eq23_e1444: f64 = (eq23_e1442 * s.v[224]);
        let eq23_e1444_d_n0: f64 = ((eq23_e1442_d_n0 * s.v[224]) + (eq23_e1442 * s.dn[224][0]));
        let eq23_e1444_d_n1: f64 = ((eq23_e1442_d_n1 * s.v[224]) + (eq23_e1442 * s.dn[224][1]));
        let eq23_e1444_d_n2: f64 = ((eq23_e1442_d_n2 * s.v[224]) + (eq23_e1442 * s.dn[224][2]));
        let eq23_e1444_d_n3: f64 = ((eq23_e1442_d_n3 * s.v[224]) + (eq23_e1442 * s.dn[224][3]));
        let eq23_e1444_d_n4: f64 = ((eq23_e1442_d_n4 * s.v[224]) + (eq23_e1442 * s.dn[224][4]));
        let eq23_e1444_d_n5: f64 = ((eq23_e1442_d_n5 * s.v[224]) + (eq23_e1442 * s.dn[224][5]));
        let eq23_e1444_d_n6: f64 = ((eq23_e1442_d_n6 * s.v[224]) + (eq23_e1442 * s.dn[224][6]));
        let eq23_e1444_d_n7: f64 = ((eq23_e1442_d_n7 * s.v[224]) + (eq23_e1442 * s.dn[224][7]));
        let eq23_e1444_d_n8: f64 = ((eq23_e1442_d_n8 * s.v[224]) + (eq23_e1442 * s.dn[224][8]));
        let eq23_e1444_d_n9: f64 = ((eq23_e1442_d_n9 * s.v[224]) + (eq23_e1442 * s.dn[224][9]));
        let eq23_e1444_d_n10: f64 = ((eq23_e1442_d_n10 * s.v[224]) + (eq23_e1442 * s.dn[224][10]));
        let eq23_e1444_d_n11: f64 = ((eq23_e1442_d_n11 * s.v[224]) + (eq23_e1442 * s.dn[224][11]));
        let eq23_e1444_d_n12: f64 = ((eq23_e1442_d_n12 * s.v[224]) + (eq23_e1442 * s.dn[224][12]));
        let eq23_e1444_d_n13: f64 = ((eq23_e1442_d_n13 * s.v[224]) + (eq23_e1442 * s.dn[224][13]));
        let eq23_e1444_d_n14: f64 = ((eq23_e1442_d_n14 * s.v[224]) + (eq23_e1442 * s.dn[224][14]));
        let eq23_e1444_d_n15: f64 = ((eq23_e1442_d_n15 * s.v[224]) + (eq23_e1442 * s.dn[224][15]));
        let eq23_e1444_d_n16: f64 = ((eq23_e1442_d_n16 * s.v[224]) + (eq23_e1442 * s.dn[224][16]));
        let eq23_e1444_d_b0: f64 = ((eq23_e1442_d_b0 * s.v[224]) + (eq23_e1442 * s.db[224][0]));
        let eq23_e1444_d_b1: f64 = ((eq23_e1442_d_b1 * s.v[224]) + (eq23_e1442 * s.db[224][1]));
        let eq23_e1444_d_b2: f64 = ((eq23_e1442_d_b2 * s.v[224]) + (eq23_e1442 * s.db[224][2]));
        let eq23_e1444_d_b3: f64 = ((eq23_e1442_d_b3 * s.v[224]) + (eq23_e1442 * s.db[224][3]));
        let eq23_e1444_d_b4: f64 = ((eq23_e1442_d_b4 * s.v[224]) + (eq23_e1442 * s.db[224][4]));
        let eq23_e1444_d_b5: f64 = ((eq23_e1442_d_b5 * s.v[224]) + (eq23_e1442 * s.db[224][5]));
        let eq23_e1444_d_b6: f64 = ((eq23_e1442_d_b6 * s.v[224]) + (eq23_e1442 * s.db[224][6]));
        let eq23_e1444_d_b7: f64 = ((eq23_e1442_d_b7 * s.v[224]) + (eq23_e1442 * s.db[224][7]));
        let eq23_e1444_d_b8: f64 = ((eq23_e1442_d_b8 * s.v[224]) + (eq23_e1442 * s.db[224][8]));
        let eq23_e1444_d_b9: f64 = ((eq23_e1442_d_b9 * s.v[224]) + (eq23_e1442 * s.db[224][9]));
        let eq23_e1444_d_b10: f64 = ((eq23_e1442_d_b10 * s.v[224]) + (eq23_e1442 * s.db[224][10]));
        let eq23_e1444_d_b11: f64 = ((eq23_e1442_d_b11 * s.v[224]) + (eq23_e1442 * s.db[224][11]));
        let eq23_e1444_d_b12: f64 = ((eq23_e1442_d_b12 * s.v[224]) + (eq23_e1442 * s.db[224][12]));
        let eq23_e1444_d_b13: f64 = ((eq23_e1442_d_b13 * s.v[224]) + (eq23_e1442 * s.db[224][13]));
        let eq23_e1445_q: f64 = eq23_e1444;
        let eq23_e1446: f64 = (p.p29 * eq23_e1444);
        let eq23_e1446_d_n0: f64 = (p.p29 * eq23_e1444_d_n0);
        let eq23_e1446_d_n1: f64 = (p.p29 * eq23_e1444_d_n1);
        let eq23_e1446_d_n2: f64 = (p.p29 * eq23_e1444_d_n2);
        let eq23_e1446_d_n3: f64 = (p.p29 * eq23_e1444_d_n3);
        let eq23_e1446_d_n4: f64 = (p.p29 * eq23_e1444_d_n4);
        let eq23_e1446_d_n5: f64 = (p.p29 * eq23_e1444_d_n5);
        let eq23_e1446_d_n6: f64 = (p.p29 * eq23_e1444_d_n6);
        let eq23_e1446_d_n7: f64 = (p.p29 * eq23_e1444_d_n7);
        let eq23_e1446_d_n8: f64 = (p.p29 * eq23_e1444_d_n8);
        let eq23_e1446_d_n9: f64 = (p.p29 * eq23_e1444_d_n9);
        let eq23_e1446_d_n10: f64 = (p.p29 * eq23_e1444_d_n10);
        let eq23_e1446_d_n11: f64 = (p.p29 * eq23_e1444_d_n11);
        let eq23_e1446_d_n12: f64 = (p.p29 * eq23_e1444_d_n12);
        let eq23_e1446_d_n13: f64 = (p.p29 * eq23_e1444_d_n13);
        let eq23_e1446_d_n14: f64 = (p.p29 * eq23_e1444_d_n14);
        let eq23_e1446_d_n15: f64 = (p.p29 * eq23_e1444_d_n15);
        let eq23_e1446_d_n16: f64 = (p.p29 * eq23_e1444_d_n16);
        let eq23_e1446_d_b0: f64 = (p.p29 * eq23_e1444_d_b0);
        let eq23_e1446_d_b1: f64 = (p.p29 * eq23_e1444_d_b1);
        let eq23_e1446_d_b2: f64 = (p.p29 * eq23_e1444_d_b2);
        let eq23_e1446_d_b3: f64 = (p.p29 * eq23_e1444_d_b3);
        let eq23_e1446_d_b4: f64 = (p.p29 * eq23_e1444_d_b4);
        let eq23_e1446_d_b5: f64 = (p.p29 * eq23_e1444_d_b5);
        let eq23_e1446_d_b6: f64 = (p.p29 * eq23_e1444_d_b6);
        let eq23_e1446_d_b7: f64 = (p.p29 * eq23_e1444_d_b7);
        let eq23_e1446_d_b8: f64 = (p.p29 * eq23_e1444_d_b8);
        let eq23_e1446_d_b9: f64 = (p.p29 * eq23_e1444_d_b9);
        let eq23_e1446_d_b10: f64 = (p.p29 * eq23_e1444_d_b10);
        let eq23_e1446_d_b11: f64 = (p.p29 * eq23_e1444_d_b11);
        let eq23_e1446_d_b12: f64 = (p.p29 * eq23_e1444_d_b12);
        let eq23_e1446_d_b13: f64 = (p.p29 * eq23_e1444_d_b13);
        let eq23_e1446_q: f64 = (p.p29 * eq23_e1445_q);
        let eq23_e1446_q_d_n0: f64 = (p.p29 * eq23_e1444_d_n0);
        let eq23_e1446_q_d_n1: f64 = (p.p29 * eq23_e1444_d_n1);
        let eq23_e1446_q_d_n2: f64 = (p.p29 * eq23_e1444_d_n2);
        let eq23_e1446_q_d_n3: f64 = (p.p29 * eq23_e1444_d_n3);
        let eq23_e1446_q_d_n4: f64 = (p.p29 * eq23_e1444_d_n4);
        let eq23_e1446_q_d_n5: f64 = (p.p29 * eq23_e1444_d_n5);
        let eq23_e1446_q_d_n6: f64 = (p.p29 * eq23_e1444_d_n6);
        let eq23_e1446_q_d_n7: f64 = (p.p29 * eq23_e1444_d_n7);
        let eq23_e1446_q_d_n8: f64 = (p.p29 * eq23_e1444_d_n8);
        let eq23_e1446_q_d_n9: f64 = (p.p29 * eq23_e1444_d_n9);
        let eq23_e1446_q_d_n10: f64 = (p.p29 * eq23_e1444_d_n10);
        let eq23_e1446_q_d_n11: f64 = (p.p29 * eq23_e1444_d_n11);
        let eq23_e1446_q_d_n12: f64 = (p.p29 * eq23_e1444_d_n12);
        let eq23_e1446_q_d_n13: f64 = (p.p29 * eq23_e1444_d_n13);
        let eq23_e1446_q_d_n14: f64 = (p.p29 * eq23_e1444_d_n14);
        let eq23_e1446_q_d_n15: f64 = (p.p29 * eq23_e1444_d_n15);
        let eq23_e1446_q_d_n16: f64 = (p.p29 * eq23_e1444_d_n16);
        let eq23_e1446_q_d_b0: f64 = (p.p29 * eq23_e1444_d_b0);
        let eq23_e1446_q_d_b1: f64 = (p.p29 * eq23_e1444_d_b1);
        let eq23_e1446_q_d_b2: f64 = (p.p29 * eq23_e1444_d_b2);
        let eq23_e1446_q_d_b3: f64 = (p.p29 * eq23_e1444_d_b3);
        let eq23_e1446_q_d_b4: f64 = (p.p29 * eq23_e1444_d_b4);
        let eq23_e1446_q_d_b5: f64 = (p.p29 * eq23_e1444_d_b5);
        let eq23_e1446_q_d_b6: f64 = (p.p29 * eq23_e1444_d_b6);
        let eq23_e1446_q_d_b7: f64 = (p.p29 * eq23_e1444_d_b7);
        let eq23_e1446_q_d_b8: f64 = (p.p29 * eq23_e1444_d_b8);
        let eq23_e1446_q_d_b9: f64 = (p.p29 * eq23_e1444_d_b9);
        let eq23_e1446_q_d_b10: f64 = (p.p29 * eq23_e1444_d_b10);
        let eq23_e1446_q_d_b11: f64 = (p.p29 * eq23_e1444_d_b11);
        let eq23_e1446_q_d_b12: f64 = (p.p29 * eq23_e1444_d_b12);
        let eq23_e1446_q_d_b13: f64 = (p.p29 * eq23_e1444_d_b13);
        let eq23_reactive_node_derivatives: [f64; 17] = [eq23_e1446_q_d_n0, eq23_e1446_q_d_n1, eq23_e1446_q_d_n2, eq23_e1446_q_d_n3, eq23_e1446_q_d_n4, eq23_e1446_q_d_n5, eq23_e1446_q_d_n6, eq23_e1446_q_d_n7, eq23_e1446_q_d_n8, eq23_e1446_q_d_n9, eq23_e1446_q_d_n10, eq23_e1446_q_d_n11, eq23_e1446_q_d_n12, eq23_e1446_q_d_n13, eq23_e1446_q_d_n14, eq23_e1446_q_d_n15, eq23_e1446_q_d_n16];
        let eq23_reactive_branch_derivatives: [f64; 14] = [eq23_e1446_q_d_b0, eq23_e1446_q_d_b1, eq23_e1446_q_d_b2, eq23_e1446_q_d_b3, eq23_e1446_q_d_b4, eq23_e1446_q_d_b5, eq23_e1446_q_d_b6, eq23_e1446_q_d_b7, eq23_e1446_q_d_b8, eq23_e1446_q_d_b9, eq23_e1446_q_d_b10, eq23_e1446_q_d_b11, eq23_e1446_q_d_b12, eq23_e1446_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
        let eq24_e1449: f64 = (-s.v[187]);
        let eq24_e1449_d_n0: f64 = (-s.dn[187][0]);
        let eq24_e1449_d_n1: f64 = (-s.dn[187][1]);
        let eq24_e1449_d_n2: f64 = (-s.dn[187][2]);
        let eq24_e1449_d_n3: f64 = (-s.dn[187][3]);
        let eq24_e1449_d_n4: f64 = (-s.dn[187][4]);
        let eq24_e1449_d_n5: f64 = (-s.dn[187][5]);
        let eq24_e1449_d_n6: f64 = (-s.dn[187][6]);
        let eq24_e1449_d_n7: f64 = (-s.dn[187][7]);
        let eq24_e1449_d_n8: f64 = (-s.dn[187][8]);
        let eq24_e1449_d_n9: f64 = (-s.dn[187][9]);
        let eq24_e1449_d_n10: f64 = (-s.dn[187][10]);
        let eq24_e1449_d_n11: f64 = (-s.dn[187][11]);
        let eq24_e1449_d_n12: f64 = (-s.dn[187][12]);
        let eq24_e1449_d_n13: f64 = (-s.dn[187][13]);
        let eq24_e1449_d_n14: f64 = (-s.dn[187][14]);
        let eq24_e1449_d_n15: f64 = (-s.dn[187][15]);
        let eq24_e1449_d_n16: f64 = (-s.dn[187][16]);
        let eq24_e1449_d_b0: f64 = (-s.db[187][0]);
        let eq24_e1449_d_b1: f64 = (-s.db[187][1]);
        let eq24_e1449_d_b2: f64 = (-s.db[187][2]);
        let eq24_e1449_d_b3: f64 = (-s.db[187][3]);
        let eq24_e1449_d_b4: f64 = (-s.db[187][4]);
        let eq24_e1449_d_b5: f64 = (-s.db[187][5]);
        let eq24_e1449_d_b6: f64 = (-s.db[187][6]);
        let eq24_e1449_d_b7: f64 = (-s.db[187][7]);
        let eq24_e1449_d_b8: f64 = (-s.db[187][8]);
        let eq24_e1449_d_b9: f64 = (-s.db[187][9]);
        let eq24_e1449_d_b10: f64 = (-s.db[187][10]);
        let eq24_e1449_d_b11: f64 = (-s.db[187][11]);
        let eq24_e1449_d_b12: f64 = (-s.db[187][12]);
        let eq24_e1449_d_b13: f64 = (-s.db[187][13]);
        let eq24_e1451: f64 = (eq24_e1449 * s.v[221]);
        let eq24_e1451_d_n0: f64 = ((eq24_e1449_d_n0 * s.v[221]) + (eq24_e1449 * s.dn[221][0]));
        let eq24_e1451_d_n1: f64 = ((eq24_e1449_d_n1 * s.v[221]) + (eq24_e1449 * s.dn[221][1]));
        let eq24_e1451_d_n2: f64 = ((eq24_e1449_d_n2 * s.v[221]) + (eq24_e1449 * s.dn[221][2]));
        let eq24_e1451_d_n3: f64 = ((eq24_e1449_d_n3 * s.v[221]) + (eq24_e1449 * s.dn[221][3]));
        let eq24_e1451_d_n4: f64 = ((eq24_e1449_d_n4 * s.v[221]) + (eq24_e1449 * s.dn[221][4]));
        let eq24_e1451_d_n5: f64 = ((eq24_e1449_d_n5 * s.v[221]) + (eq24_e1449 * s.dn[221][5]));
        let eq24_e1451_d_n6: f64 = ((eq24_e1449_d_n6 * s.v[221]) + (eq24_e1449 * s.dn[221][6]));
        let eq24_e1451_d_n7: f64 = ((eq24_e1449_d_n7 * s.v[221]) + (eq24_e1449 * s.dn[221][7]));
        let eq24_e1451_d_n8: f64 = ((eq24_e1449_d_n8 * s.v[221]) + (eq24_e1449 * s.dn[221][8]));
        let eq24_e1451_d_n9: f64 = ((eq24_e1449_d_n9 * s.v[221]) + (eq24_e1449 * s.dn[221][9]));
        let eq24_e1451_d_n10: f64 = ((eq24_e1449_d_n10 * s.v[221]) + (eq24_e1449 * s.dn[221][10]));
        let eq24_e1451_d_n11: f64 = ((eq24_e1449_d_n11 * s.v[221]) + (eq24_e1449 * s.dn[221][11]));
        let eq24_e1451_d_n12: f64 = ((eq24_e1449_d_n12 * s.v[221]) + (eq24_e1449 * s.dn[221][12]));
        let eq24_e1451_d_n13: f64 = ((eq24_e1449_d_n13 * s.v[221]) + (eq24_e1449 * s.dn[221][13]));
        let eq24_e1451_d_n14: f64 = ((eq24_e1449_d_n14 * s.v[221]) + (eq24_e1449 * s.dn[221][14]));
        let eq24_e1451_d_n15: f64 = ((eq24_e1449_d_n15 * s.v[221]) + (eq24_e1449 * s.dn[221][15]));
        let eq24_e1451_d_n16: f64 = ((eq24_e1449_d_n16 * s.v[221]) + (eq24_e1449 * s.dn[221][16]));
        let eq24_e1451_d_b0: f64 = ((eq24_e1449_d_b0 * s.v[221]) + (eq24_e1449 * s.db[221][0]));
        let eq24_e1451_d_b1: f64 = ((eq24_e1449_d_b1 * s.v[221]) + (eq24_e1449 * s.db[221][1]));
        let eq24_e1451_d_b2: f64 = ((eq24_e1449_d_b2 * s.v[221]) + (eq24_e1449 * s.db[221][2]));
        let eq24_e1451_d_b3: f64 = ((eq24_e1449_d_b3 * s.v[221]) + (eq24_e1449 * s.db[221][3]));
        let eq24_e1451_d_b4: f64 = ((eq24_e1449_d_b4 * s.v[221]) + (eq24_e1449 * s.db[221][4]));
        let eq24_e1451_d_b5: f64 = ((eq24_e1449_d_b5 * s.v[221]) + (eq24_e1449 * s.db[221][5]));
        let eq24_e1451_d_b6: f64 = ((eq24_e1449_d_b6 * s.v[221]) + (eq24_e1449 * s.db[221][6]));
        let eq24_e1451_d_b7: f64 = ((eq24_e1449_d_b7 * s.v[221]) + (eq24_e1449 * s.db[221][7]));
        let eq24_e1451_d_b8: f64 = ((eq24_e1449_d_b8 * s.v[221]) + (eq24_e1449 * s.db[221][8]));
        let eq24_e1451_d_b9: f64 = ((eq24_e1449_d_b9 * s.v[221]) + (eq24_e1449 * s.db[221][9]));
        let eq24_e1451_d_b10: f64 = ((eq24_e1449_d_b10 * s.v[221]) + (eq24_e1449 * s.db[221][10]));
        let eq24_e1451_d_b11: f64 = ((eq24_e1449_d_b11 * s.v[221]) + (eq24_e1449 * s.db[221][11]));
        let eq24_e1451_d_b12: f64 = ((eq24_e1449_d_b12 * s.v[221]) + (eq24_e1449 * s.db[221][12]));
        let eq24_e1451_d_b13: f64 = ((eq24_e1449_d_b13 * s.v[221]) + (eq24_e1449 * s.db[221][13]));
        let eq24_e1452_q: f64 = eq24_e1451;
        let eq24_e1453: f64 = (p.p29 * eq24_e1451);
        let eq24_e1453_d_n0: f64 = (p.p29 * eq24_e1451_d_n0);
        let eq24_e1453_d_n1: f64 = (p.p29 * eq24_e1451_d_n1);
        let eq24_e1453_d_n2: f64 = (p.p29 * eq24_e1451_d_n2);
        let eq24_e1453_d_n3: f64 = (p.p29 * eq24_e1451_d_n3);
        let eq24_e1453_d_n4: f64 = (p.p29 * eq24_e1451_d_n4);
        let eq24_e1453_d_n5: f64 = (p.p29 * eq24_e1451_d_n5);
        let eq24_e1453_d_n6: f64 = (p.p29 * eq24_e1451_d_n6);
        let eq24_e1453_d_n7: f64 = (p.p29 * eq24_e1451_d_n7);
        let eq24_e1453_d_n8: f64 = (p.p29 * eq24_e1451_d_n8);
        let eq24_e1453_d_n9: f64 = (p.p29 * eq24_e1451_d_n9);
        let eq24_e1453_d_n10: f64 = (p.p29 * eq24_e1451_d_n10);
        let eq24_e1453_d_n11: f64 = (p.p29 * eq24_e1451_d_n11);
        let eq24_e1453_d_n12: f64 = (p.p29 * eq24_e1451_d_n12);
        let eq24_e1453_d_n13: f64 = (p.p29 * eq24_e1451_d_n13);
        let eq24_e1453_d_n14: f64 = (p.p29 * eq24_e1451_d_n14);
        let eq24_e1453_d_n15: f64 = (p.p29 * eq24_e1451_d_n15);
        let eq24_e1453_d_n16: f64 = (p.p29 * eq24_e1451_d_n16);
        let eq24_e1453_d_b0: f64 = (p.p29 * eq24_e1451_d_b0);
        let eq24_e1453_d_b1: f64 = (p.p29 * eq24_e1451_d_b1);
        let eq24_e1453_d_b2: f64 = (p.p29 * eq24_e1451_d_b2);
        let eq24_e1453_d_b3: f64 = (p.p29 * eq24_e1451_d_b3);
        let eq24_e1453_d_b4: f64 = (p.p29 * eq24_e1451_d_b4);
        let eq24_e1453_d_b5: f64 = (p.p29 * eq24_e1451_d_b5);
        let eq24_e1453_d_b6: f64 = (p.p29 * eq24_e1451_d_b6);
        let eq24_e1453_d_b7: f64 = (p.p29 * eq24_e1451_d_b7);
        let eq24_e1453_d_b8: f64 = (p.p29 * eq24_e1451_d_b8);
        let eq24_e1453_d_b9: f64 = (p.p29 * eq24_e1451_d_b9);
        let eq24_e1453_d_b10: f64 = (p.p29 * eq24_e1451_d_b10);
        let eq24_e1453_d_b11: f64 = (p.p29 * eq24_e1451_d_b11);
        let eq24_e1453_d_b12: f64 = (p.p29 * eq24_e1451_d_b12);
        let eq24_e1453_d_b13: f64 = (p.p29 * eq24_e1451_d_b13);
        let eq24_e1453_q: f64 = (p.p29 * eq24_e1452_q);
        let eq24_e1453_q_d_n0: f64 = (p.p29 * eq24_e1451_d_n0);
        let eq24_e1453_q_d_n1: f64 = (p.p29 * eq24_e1451_d_n1);
        let eq24_e1453_q_d_n2: f64 = (p.p29 * eq24_e1451_d_n2);
        let eq24_e1453_q_d_n3: f64 = (p.p29 * eq24_e1451_d_n3);
        let eq24_e1453_q_d_n4: f64 = (p.p29 * eq24_e1451_d_n4);
        let eq24_e1453_q_d_n5: f64 = (p.p29 * eq24_e1451_d_n5);
        let eq24_e1453_q_d_n6: f64 = (p.p29 * eq24_e1451_d_n6);
        let eq24_e1453_q_d_n7: f64 = (p.p29 * eq24_e1451_d_n7);
        let eq24_e1453_q_d_n8: f64 = (p.p29 * eq24_e1451_d_n8);
        let eq24_e1453_q_d_n9: f64 = (p.p29 * eq24_e1451_d_n9);
        let eq24_e1453_q_d_n10: f64 = (p.p29 * eq24_e1451_d_n10);
        let eq24_e1453_q_d_n11: f64 = (p.p29 * eq24_e1451_d_n11);
        let eq24_e1453_q_d_n12: f64 = (p.p29 * eq24_e1451_d_n12);
        let eq24_e1453_q_d_n13: f64 = (p.p29 * eq24_e1451_d_n13);
        let eq24_e1453_q_d_n14: f64 = (p.p29 * eq24_e1451_d_n14);
        let eq24_e1453_q_d_n15: f64 = (p.p29 * eq24_e1451_d_n15);
        let eq24_e1453_q_d_n16: f64 = (p.p29 * eq24_e1451_d_n16);
        let eq24_e1453_q_d_b0: f64 = (p.p29 * eq24_e1451_d_b0);
        let eq24_e1453_q_d_b1: f64 = (p.p29 * eq24_e1451_d_b1);
        let eq24_e1453_q_d_b2: f64 = (p.p29 * eq24_e1451_d_b2);
        let eq24_e1453_q_d_b3: f64 = (p.p29 * eq24_e1451_d_b3);
        let eq24_e1453_q_d_b4: f64 = (p.p29 * eq24_e1451_d_b4);
        let eq24_e1453_q_d_b5: f64 = (p.p29 * eq24_e1451_d_b5);
        let eq24_e1453_q_d_b6: f64 = (p.p29 * eq24_e1451_d_b6);
        let eq24_e1453_q_d_b7: f64 = (p.p29 * eq24_e1451_d_b7);
        let eq24_e1453_q_d_b8: f64 = (p.p29 * eq24_e1451_d_b8);
        let eq24_e1453_q_d_b9: f64 = (p.p29 * eq24_e1451_d_b9);
        let eq24_e1453_q_d_b10: f64 = (p.p29 * eq24_e1451_d_b10);
        let eq24_e1453_q_d_b11: f64 = (p.p29 * eq24_e1451_d_b11);
        let eq24_e1453_q_d_b12: f64 = (p.p29 * eq24_e1451_d_b12);
        let eq24_e1453_q_d_b13: f64 = (p.p29 * eq24_e1451_d_b13);
        let eq24_reactive_node_derivatives: [f64; 17] = [eq24_e1453_q_d_n0, eq24_e1453_q_d_n1, eq24_e1453_q_d_n2, eq24_e1453_q_d_n3, eq24_e1453_q_d_n4, eq24_e1453_q_d_n5, eq24_e1453_q_d_n6, eq24_e1453_q_d_n7, eq24_e1453_q_d_n8, eq24_e1453_q_d_n9, eq24_e1453_q_d_n10, eq24_e1453_q_d_n11, eq24_e1453_q_d_n12, eq24_e1453_q_d_n13, eq24_e1453_q_d_n14, eq24_e1453_q_d_n15, eq24_e1453_q_d_n16];
        let eq24_reactive_branch_derivatives: [f64; 14] = [eq24_e1453_q_d_b0, eq24_e1453_q_d_b1, eq24_e1453_q_d_b2, eq24_e1453_q_d_b3, eq24_e1453_q_d_b4, eq24_e1453_q_d_b5, eq24_e1453_q_d_b6, eq24_e1453_q_d_b7, eq24_e1453_q_d_b8, eq24_e1453_q_d_b9, eq24_e1453_q_d_b10, eq24_e1453_q_d_b11, eq24_e1453_q_d_b12, eq24_e1453_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[11]),
            nodes,
            &eq24_reactive_node_derivatives,
            branches,
            &eq24_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_3(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq55_e1708, eq55_e1708_d_n0, eq55_e1708_d_n1, eq55_e1708_d_n2, eq55_e1708_d_n3, eq55_e1708_d_n4, eq55_e1708_d_n5, eq55_e1708_d_n6, eq55_e1708_d_n7, eq55_e1708_d_n8, eq55_e1708_d_n9, eq55_e1708_d_n10, eq55_e1708_d_n11, eq55_e1708_d_n12, eq55_e1708_d_n13, eq55_e1708_d_n14, eq55_e1708_d_n15, eq55_e1708_d_n16, eq55_e1708_d_b0, eq55_e1708_d_b1, eq55_e1708_d_b2, eq55_e1708_d_b3, eq55_e1708_d_b4, eq55_e1708_d_b5, eq55_e1708_d_b6, eq55_e1708_d_b7, eq55_e1708_d_b8, eq55_e1708_d_b9, eq55_e1708_d_b10, eq55_e1708_d_b11, eq55_e1708_d_b12, eq55_e1708_d_b13, eq55_e1708_q, eq55_e1708_q_d_n0, eq55_e1708_q_d_n1, eq55_e1708_q_d_n2, eq55_e1708_q_d_n3, eq55_e1708_q_d_n4, eq55_e1708_q_d_n5, eq55_e1708_q_d_n6, eq55_e1708_q_d_n7, eq55_e1708_q_d_n8, eq55_e1708_q_d_n9, eq55_e1708_q_d_n10, eq55_e1708_q_d_n11, eq55_e1708_q_d_n12, eq55_e1708_q_d_n13, eq55_e1708_q_d_n14, eq55_e1708_q_d_n15, eq55_e1708_q_d_n16, eq55_e1708_q_d_b0, eq55_e1708_q_d_b1, eq55_e1708_q_d_b2, eq55_e1708_q_d_b3, eq55_e1708_q_d_b4, eq55_e1708_q_d_b5, eq55_e1708_q_d_b6, eq55_e1708_q_d_b7, eq55_e1708_q_d_b8, eq55_e1708_q_d_b9, eq55_e1708_q_d_b10, eq55_e1708_q_d_b11, eq55_e1708_q_d_b12, eq55_e1708_q_d_b13,) = {
    if s.b[1621] {
        let eq55_e1699: f64 = (s.v[390] * s.v[747]);
        let eq55_e1699_d_n0: f64 = ((s.dn[390][0] * s.v[747]) + (s.v[390] * s.dn[747][0]));
        let eq55_e1699_d_n1: f64 = ((s.dn[390][1] * s.v[747]) + (s.v[390] * s.dn[747][1]));
        let eq55_e1699_d_n2: f64 = ((s.dn[390][2] * s.v[747]) + (s.v[390] * s.dn[747][2]));
        let eq55_e1699_d_n3: f64 = ((s.dn[390][3] * s.v[747]) + (s.v[390] * s.dn[747][3]));
        let eq55_e1699_d_n4: f64 = ((s.dn[390][4] * s.v[747]) + (s.v[390] * s.dn[747][4]));
        let eq55_e1699_d_n5: f64 = ((s.dn[390][5] * s.v[747]) + (s.v[390] * s.dn[747][5]));
        let eq55_e1699_d_n6: f64 = ((s.dn[390][6] * s.v[747]) + (s.v[390] * s.dn[747][6]));
        let eq55_e1699_d_n7: f64 = ((s.dn[390][7] * s.v[747]) + (s.v[390] * s.dn[747][7]));
        let eq55_e1699_d_n8: f64 = ((s.dn[390][8] * s.v[747]) + (s.v[390] * s.dn[747][8]));
        let eq55_e1699_d_n9: f64 = ((s.dn[390][9] * s.v[747]) + (s.v[390] * s.dn[747][9]));
        let eq55_e1699_d_n10: f64 = ((s.dn[390][10] * s.v[747]) + (s.v[390] * s.dn[747][10]));
        let eq55_e1699_d_n11: f64 = ((s.dn[390][11] * s.v[747]) + (s.v[390] * s.dn[747][11]));
        let eq55_e1699_d_n12: f64 = ((s.dn[390][12] * s.v[747]) + (s.v[390] * s.dn[747][12]));
        let eq55_e1699_d_n13: f64 = ((s.dn[390][13] * s.v[747]) + (s.v[390] * s.dn[747][13]));
        let eq55_e1699_d_n14: f64 = ((s.dn[390][14] * s.v[747]) + (s.v[390] * s.dn[747][14]));
        let eq55_e1699_d_n15: f64 = ((s.dn[390][15] * s.v[747]) + (s.v[390] * s.dn[747][15]));
        let eq55_e1699_d_n16: f64 = ((s.dn[390][16] * s.v[747]) + (s.v[390] * s.dn[747][16]));
        let eq55_e1699_d_b0: f64 = ((s.db[390][0] * s.v[747]) + (s.v[390] * s.db[747][0]));
        let eq55_e1699_d_b1: f64 = ((s.db[390][1] * s.v[747]) + (s.v[390] * s.db[747][1]));
        let eq55_e1699_d_b2: f64 = ((s.db[390][2] * s.v[747]) + (s.v[390] * s.db[747][2]));
        let eq55_e1699_d_b3: f64 = ((s.db[390][3] * s.v[747]) + (s.v[390] * s.db[747][3]));
        let eq55_e1699_d_b4: f64 = ((s.db[390][4] * s.v[747]) + (s.v[390] * s.db[747][4]));
        let eq55_e1699_d_b5: f64 = ((s.db[390][5] * s.v[747]) + (s.v[390] * s.db[747][5]));
        let eq55_e1699_d_b6: f64 = ((s.db[390][6] * s.v[747]) + (s.v[390] * s.db[747][6]));
        let eq55_e1699_d_b7: f64 = ((s.db[390][7] * s.v[747]) + (s.v[390] * s.db[747][7]));
        let eq55_e1699_d_b8: f64 = ((s.db[390][8] * s.v[747]) + (s.v[390] * s.db[747][8]));
        let eq55_e1699_d_b9: f64 = ((s.db[390][9] * s.v[747]) + (s.v[390] * s.db[747][9]));
        let eq55_e1699_d_b10: f64 = ((s.db[390][10] * s.v[747]) + (s.v[390] * s.db[747][10]));
        let eq55_e1699_d_b11: f64 = ((s.db[390][11] * s.v[747]) + (s.v[390] * s.db[747][11]));
        let eq55_e1699_d_b12: f64 = ((s.db[390][12] * s.v[747]) + (s.v[390] * s.db[747][12]));
        let eq55_e1699_d_b13: f64 = ((s.db[390][13] * s.v[747]) + (s.v[390] * s.db[747][13]));
        let eq55_e1702: f64 = (s.v[390] * s.v[748]);
        let eq55_e1702_d_n0: f64 = ((s.dn[390][0] * s.v[748]) + (s.v[390] * s.dn[748][0]));
        let eq55_e1702_d_n1: f64 = ((s.dn[390][1] * s.v[748]) + (s.v[390] * s.dn[748][1]));
        let eq55_e1702_d_n2: f64 = ((s.dn[390][2] * s.v[748]) + (s.v[390] * s.dn[748][2]));
        let eq55_e1702_d_n3: f64 = ((s.dn[390][3] * s.v[748]) + (s.v[390] * s.dn[748][3]));
        let eq55_e1702_d_n4: f64 = ((s.dn[390][4] * s.v[748]) + (s.v[390] * s.dn[748][4]));
        let eq55_e1702_d_n5: f64 = ((s.dn[390][5] * s.v[748]) + (s.v[390] * s.dn[748][5]));
        let eq55_e1702_d_n6: f64 = ((s.dn[390][6] * s.v[748]) + (s.v[390] * s.dn[748][6]));
        let eq55_e1702_d_n7: f64 = ((s.dn[390][7] * s.v[748]) + (s.v[390] * s.dn[748][7]));
        let eq55_e1702_d_n8: f64 = ((s.dn[390][8] * s.v[748]) + (s.v[390] * s.dn[748][8]));
        let eq55_e1702_d_n9: f64 = ((s.dn[390][9] * s.v[748]) + (s.v[390] * s.dn[748][9]));
        let eq55_e1702_d_n10: f64 = ((s.dn[390][10] * s.v[748]) + (s.v[390] * s.dn[748][10]));
        let eq55_e1702_d_n11: f64 = ((s.dn[390][11] * s.v[748]) + (s.v[390] * s.dn[748][11]));
        let eq55_e1702_d_n12: f64 = ((s.dn[390][12] * s.v[748]) + (s.v[390] * s.dn[748][12]));
        let eq55_e1702_d_n13: f64 = ((s.dn[390][13] * s.v[748]) + (s.v[390] * s.dn[748][13]));
        let eq55_e1702_d_n14: f64 = ((s.dn[390][14] * s.v[748]) + (s.v[390] * s.dn[748][14]));
        let eq55_e1702_d_n15: f64 = ((s.dn[390][15] * s.v[748]) + (s.v[390] * s.dn[748][15]));
        let eq55_e1702_d_n16: f64 = ((s.dn[390][16] * s.v[748]) + (s.v[390] * s.dn[748][16]));
        let eq55_e1702_d_b0: f64 = ((s.db[390][0] * s.v[748]) + (s.v[390] * s.db[748][0]));
        let eq55_e1702_d_b1: f64 = ((s.db[390][1] * s.v[748]) + (s.v[390] * s.db[748][1]));
        let eq55_e1702_d_b2: f64 = ((s.db[390][2] * s.v[748]) + (s.v[390] * s.db[748][2]));
        let eq55_e1702_d_b3: f64 = ((s.db[390][3] * s.v[748]) + (s.v[390] * s.db[748][3]));
        let eq55_e1702_d_b4: f64 = ((s.db[390][4] * s.v[748]) + (s.v[390] * s.db[748][4]));
        let eq55_e1702_d_b5: f64 = ((s.db[390][5] * s.v[748]) + (s.v[390] * s.db[748][5]));
        let eq55_e1702_d_b6: f64 = ((s.db[390][6] * s.v[748]) + (s.v[390] * s.db[748][6]));
        let eq55_e1702_d_b7: f64 = ((s.db[390][7] * s.v[748]) + (s.v[390] * s.db[748][7]));
        let eq55_e1702_d_b8: f64 = ((s.db[390][8] * s.v[748]) + (s.v[390] * s.db[748][8]));
        let eq55_e1702_d_b9: f64 = ((s.db[390][9] * s.v[748]) + (s.v[390] * s.db[748][9]));
        let eq55_e1702_d_b10: f64 = ((s.db[390][10] * s.v[748]) + (s.v[390] * s.db[748][10]));
        let eq55_e1702_d_b11: f64 = ((s.db[390][11] * s.v[748]) + (s.v[390] * s.db[748][11]));
        let eq55_e1702_d_b12: f64 = ((s.db[390][12] * s.v[748]) + (s.v[390] * s.db[748][12]));
        let eq55_e1702_d_b13: f64 = ((s.db[390][13] * s.v[748]) + (s.v[390] * s.db[748][13]));
        let eq55_e1703_q: f64 = eq55_e1702;
        let eq55_e1704: f64 = (eq55_e1699 + eq55_e1702);
        let eq55_e1704_d_n0: f64 = (eq55_e1699_d_n0 + eq55_e1702_d_n0);
        let eq55_e1704_d_n1: f64 = (eq55_e1699_d_n1 + eq55_e1702_d_n1);
        let eq55_e1704_d_n2: f64 = (eq55_e1699_d_n2 + eq55_e1702_d_n2);
        let eq55_e1704_d_n3: f64 = (eq55_e1699_d_n3 + eq55_e1702_d_n3);
        let eq55_e1704_d_n4: f64 = (eq55_e1699_d_n4 + eq55_e1702_d_n4);
        let eq55_e1704_d_n5: f64 = (eq55_e1699_d_n5 + eq55_e1702_d_n5);
        let eq55_e1704_d_n6: f64 = (eq55_e1699_d_n6 + eq55_e1702_d_n6);
        let eq55_e1704_d_n7: f64 = (eq55_e1699_d_n7 + eq55_e1702_d_n7);
        let eq55_e1704_d_n8: f64 = (eq55_e1699_d_n8 + eq55_e1702_d_n8);
        let eq55_e1704_d_n9: f64 = (eq55_e1699_d_n9 + eq55_e1702_d_n9);
        let eq55_e1704_d_n10: f64 = (eq55_e1699_d_n10 + eq55_e1702_d_n10);
        let eq55_e1704_d_n11: f64 = (eq55_e1699_d_n11 + eq55_e1702_d_n11);
        let eq55_e1704_d_n12: f64 = (eq55_e1699_d_n12 + eq55_e1702_d_n12);
        let eq55_e1704_d_n13: f64 = (eq55_e1699_d_n13 + eq55_e1702_d_n13);
        let eq55_e1704_d_n14: f64 = (eq55_e1699_d_n14 + eq55_e1702_d_n14);
        let eq55_e1704_d_n15: f64 = (eq55_e1699_d_n15 + eq55_e1702_d_n15);
        let eq55_e1704_d_n16: f64 = (eq55_e1699_d_n16 + eq55_e1702_d_n16);
        let eq55_e1704_d_b0: f64 = (eq55_e1699_d_b0 + eq55_e1702_d_b0);
        let eq55_e1704_d_b1: f64 = (eq55_e1699_d_b1 + eq55_e1702_d_b1);
        let eq55_e1704_d_b2: f64 = (eq55_e1699_d_b2 + eq55_e1702_d_b2);
        let eq55_e1704_d_b3: f64 = (eq55_e1699_d_b3 + eq55_e1702_d_b3);
        let eq55_e1704_d_b4: f64 = (eq55_e1699_d_b4 + eq55_e1702_d_b4);
        let eq55_e1704_d_b5: f64 = (eq55_e1699_d_b5 + eq55_e1702_d_b5);
        let eq55_e1704_d_b6: f64 = (eq55_e1699_d_b6 + eq55_e1702_d_b6);
        let eq55_e1704_d_b7: f64 = (eq55_e1699_d_b7 + eq55_e1702_d_b7);
        let eq55_e1704_d_b8: f64 = (eq55_e1699_d_b8 + eq55_e1702_d_b8);
        let eq55_e1704_d_b9: f64 = (eq55_e1699_d_b9 + eq55_e1702_d_b9);
        let eq55_e1704_d_b10: f64 = (eq55_e1699_d_b10 + eq55_e1702_d_b10);
        let eq55_e1704_d_b11: f64 = (eq55_e1699_d_b11 + eq55_e1702_d_b11);
        let eq55_e1704_d_b12: f64 = (eq55_e1699_d_b12 + eq55_e1702_d_b12);
        let eq55_e1704_d_b13: f64 = (eq55_e1699_d_b13 + eq55_e1702_d_b13);
        let eq55_e1704_q: f64 = eq55_e1703_q;
        let eq55_e1706: f64 = (eq55_e1704 - s.v[749]);
        let eq55_e1706_d_n0: f64 = (eq55_e1704_d_n0 - s.dn[749][0]);
        let eq55_e1706_d_n1: f64 = (eq55_e1704_d_n1 - s.dn[749][1]);
        let eq55_e1706_d_n2: f64 = (eq55_e1704_d_n2 - s.dn[749][2]);
        let eq55_e1706_d_n3: f64 = (eq55_e1704_d_n3 - s.dn[749][3]);
        let eq55_e1706_d_n4: f64 = (eq55_e1704_d_n4 - s.dn[749][4]);
        let eq55_e1706_d_n5: f64 = (eq55_e1704_d_n5 - s.dn[749][5]);
        let eq55_e1706_d_n6: f64 = (eq55_e1704_d_n6 - s.dn[749][6]);
        let eq55_e1706_d_n7: f64 = (eq55_e1704_d_n7 - s.dn[749][7]);
        let eq55_e1706_d_n8: f64 = (eq55_e1704_d_n8 - s.dn[749][8]);
        let eq55_e1706_d_n9: f64 = (eq55_e1704_d_n9 - s.dn[749][9]);
        let eq55_e1706_d_n10: f64 = (eq55_e1704_d_n10 - s.dn[749][10]);
        let eq55_e1706_d_n11: f64 = (eq55_e1704_d_n11 - s.dn[749][11]);
        let eq55_e1706_d_n12: f64 = (eq55_e1704_d_n12 - s.dn[749][12]);
        let eq55_e1706_d_n13: f64 = (eq55_e1704_d_n13 - s.dn[749][13]);
        let eq55_e1706_d_n14: f64 = (eq55_e1704_d_n14 - s.dn[749][14]);
        let eq55_e1706_d_n15: f64 = (eq55_e1704_d_n15 - s.dn[749][15]);
        let eq55_e1706_d_n16: f64 = (eq55_e1704_d_n16 - s.dn[749][16]);
        let eq55_e1706_d_b0: f64 = (eq55_e1704_d_b0 - s.db[749][0]);
        let eq55_e1706_d_b1: f64 = (eq55_e1704_d_b1 - s.db[749][1]);
        let eq55_e1706_d_b2: f64 = (eq55_e1704_d_b2 - s.db[749][2]);
        let eq55_e1706_d_b3: f64 = (eq55_e1704_d_b3 - s.db[749][3]);
        let eq55_e1706_d_b4: f64 = (eq55_e1704_d_b4 - s.db[749][4]);
        let eq55_e1706_d_b5: f64 = (eq55_e1704_d_b5 - s.db[749][5]);
        let eq55_e1706_d_b6: f64 = (eq55_e1704_d_b6 - s.db[749][6]);
        let eq55_e1706_d_b7: f64 = (eq55_e1704_d_b7 - s.db[749][7]);
        let eq55_e1706_d_b8: f64 = (eq55_e1704_d_b8 - s.db[749][8]);
        let eq55_e1706_d_b9: f64 = (eq55_e1704_d_b9 - s.db[749][9]);
        let eq55_e1706_d_b10: f64 = (eq55_e1704_d_b10 - s.db[749][10]);
        let eq55_e1706_d_b11: f64 = (eq55_e1704_d_b11 - s.db[749][11]);
        let eq55_e1706_d_b12: f64 = (eq55_e1704_d_b12 - s.db[749][12]);
        let eq55_e1706_d_b13: f64 = (eq55_e1704_d_b13 - s.db[749][13]);
        let eq55_e1706_q: f64 = eq55_e1704_q;
        (eq55_e1706, eq55_e1706_d_n0, eq55_e1706_d_n1, eq55_e1706_d_n2, eq55_e1706_d_n3, eq55_e1706_d_n4, eq55_e1706_d_n5, eq55_e1706_d_n6, eq55_e1706_d_n7, eq55_e1706_d_n8, eq55_e1706_d_n9, eq55_e1706_d_n10, eq55_e1706_d_n11, eq55_e1706_d_n12, eq55_e1706_d_n13, eq55_e1706_d_n14, eq55_e1706_d_n15, eq55_e1706_d_n16, eq55_e1706_d_b0, eq55_e1706_d_b1, eq55_e1706_d_b2, eq55_e1706_d_b3, eq55_e1706_d_b4, eq55_e1706_d_b5, eq55_e1706_d_b6, eq55_e1706_d_b7, eq55_e1706_d_b8, eq55_e1706_d_b9, eq55_e1706_d_b10, eq55_e1706_d_b11, eq55_e1706_d_b12, eq55_e1706_d_b13, eq55_e1706_q, eq55_e1702_d_n0, eq55_e1702_d_n1, eq55_e1702_d_n2, eq55_e1702_d_n3, eq55_e1702_d_n4, eq55_e1702_d_n5, eq55_e1702_d_n6, eq55_e1702_d_n7, eq55_e1702_d_n8, eq55_e1702_d_n9, eq55_e1702_d_n10, eq55_e1702_d_n11, eq55_e1702_d_n12, eq55_e1702_d_n13, eq55_e1702_d_n14, eq55_e1702_d_n15, eq55_e1702_d_n16, eq55_e1702_d_b0, eq55_e1702_d_b1, eq55_e1702_d_b2, eq55_e1702_d_b3, eq55_e1702_d_b4, eq55_e1702_d_b5, eq55_e1702_d_b6, eq55_e1702_d_b7, eq55_e1702_d_b8, eq55_e1702_d_b9, eq55_e1702_d_b10, eq55_e1702_d_b11, eq55_e1702_d_b12, eq55_e1702_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_reactive_node_derivatives: [f64; 17] = [eq55_e1708_q_d_n0, eq55_e1708_q_d_n1, eq55_e1708_q_d_n2, eq55_e1708_q_d_n3, eq55_e1708_q_d_n4, eq55_e1708_q_d_n5, eq55_e1708_q_d_n6, eq55_e1708_q_d_n7, eq55_e1708_q_d_n8, eq55_e1708_q_d_n9, eq55_e1708_q_d_n10, eq55_e1708_q_d_n11, eq55_e1708_q_d_n12, eq55_e1708_q_d_n13, eq55_e1708_q_d_n14, eq55_e1708_q_d_n15, eq55_e1708_q_d_n16];
        let eq55_reactive_branch_derivatives: [f64; 14] = [eq55_e1708_q_d_b0, eq55_e1708_q_d_b1, eq55_e1708_q_d_b2, eq55_e1708_q_d_b3, eq55_e1708_q_d_b4, eq55_e1708_q_d_b5, eq55_e1708_q_d_b6, eq55_e1708_q_d_b7, eq55_e1708_q_d_b8, eq55_e1708_q_d_b9, eq55_e1708_q_d_b10, eq55_e1708_q_d_b11, eq55_e1708_q_d_b12, eq55_e1708_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq55_reactive_node_derivatives,
            branches,
            &eq55_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq71_e1841, eq71_e1841_d_n0, eq71_e1841_d_n1, eq71_e1841_d_n2, eq71_e1841_d_n3, eq71_e1841_d_n4, eq71_e1841_d_n5, eq71_e1841_d_n6, eq71_e1841_d_n7, eq71_e1841_d_n8, eq71_e1841_d_n9, eq71_e1841_d_n10, eq71_e1841_d_n11, eq71_e1841_d_n12, eq71_e1841_d_n13, eq71_e1841_d_n14, eq71_e1841_d_n15, eq71_e1841_d_n16, eq71_e1841_d_b0, eq71_e1841_d_b1, eq71_e1841_d_b2, eq71_e1841_d_b3, eq71_e1841_d_b4, eq71_e1841_d_b5, eq71_e1841_d_b6, eq71_e1841_d_b7, eq71_e1841_d_b8, eq71_e1841_d_b9, eq71_e1841_d_b10, eq71_e1841_d_b11, eq71_e1841_d_b12, eq71_e1841_d_b13, eq71_e1841_q, eq71_e1841_q_d_n0, eq71_e1841_q_d_n1, eq71_e1841_q_d_n2, eq71_e1841_q_d_n3, eq71_e1841_q_d_n4, eq71_e1841_q_d_n5, eq71_e1841_q_d_n6, eq71_e1841_q_d_n7, eq71_e1841_q_d_n8, eq71_e1841_q_d_n9, eq71_e1841_q_d_n10, eq71_e1841_q_d_n11, eq71_e1841_q_d_n12, eq71_e1841_q_d_n13, eq71_e1841_q_d_n14, eq71_e1841_q_d_n15, eq71_e1841_q_d_n16, eq71_e1841_q_d_b0, eq71_e1841_q_d_b1, eq71_e1841_q_d_b2, eq71_e1841_q_d_b3, eq71_e1841_q_d_b4, eq71_e1841_q_d_b5, eq71_e1841_q_d_b6, eq71_e1841_q_d_b7, eq71_e1841_q_d_b8, eq71_e1841_q_d_b9, eq71_e1841_q_d_b10, eq71_e1841_q_d_b11, eq71_e1841_q_d_b12, eq71_e1841_q_d_b13,) = {
    if s.b[1627] {
        let eq71_e1837: f64 = (p.p29 * s.v[330]);
        let eq71_e1837_d_n0: f64 = (p.p29 * s.dn[330][0]);
        let eq71_e1837_d_n1: f64 = (p.p29 * s.dn[330][1]);
        let eq71_e1837_d_n2: f64 = (p.p29 * s.dn[330][2]);
        let eq71_e1837_d_n3: f64 = (p.p29 * s.dn[330][3]);
        let eq71_e1837_d_n4: f64 = (p.p29 * s.dn[330][4]);
        let eq71_e1837_d_n5: f64 = (p.p29 * s.dn[330][5]);
        let eq71_e1837_d_n6: f64 = (p.p29 * s.dn[330][6]);
        let eq71_e1837_d_n7: f64 = (p.p29 * s.dn[330][7]);
        let eq71_e1837_d_n8: f64 = (p.p29 * s.dn[330][8]);
        let eq71_e1837_d_n9: f64 = (p.p29 * s.dn[330][9]);
        let eq71_e1837_d_n10: f64 = (p.p29 * s.dn[330][10]);
        let eq71_e1837_d_n11: f64 = (p.p29 * s.dn[330][11]);
        let eq71_e1837_d_n12: f64 = (p.p29 * s.dn[330][12]);
        let eq71_e1837_d_n13: f64 = (p.p29 * s.dn[330][13]);
        let eq71_e1837_d_n14: f64 = (p.p29 * s.dn[330][14]);
        let eq71_e1837_d_n15: f64 = (p.p29 * s.dn[330][15]);
        let eq71_e1837_d_n16: f64 = (p.p29 * s.dn[330][16]);
        let eq71_e1837_d_b0: f64 = (p.p29 * s.db[330][0]);
        let eq71_e1837_d_b1: f64 = (p.p29 * s.db[330][1]);
        let eq71_e1837_d_b2: f64 = (p.p29 * s.db[330][2]);
        let eq71_e1837_d_b3: f64 = (p.p29 * s.db[330][3]);
        let eq71_e1837_d_b4: f64 = (p.p29 * s.db[330][4]);
        let eq71_e1837_d_b5: f64 = (p.p29 * s.db[330][5]);
        let eq71_e1837_d_b6: f64 = (p.p29 * s.db[330][6]);
        let eq71_e1837_d_b7: f64 = (p.p29 * s.db[330][7]);
        let eq71_e1837_d_b8: f64 = (p.p29 * s.db[330][8]);
        let eq71_e1837_d_b9: f64 = (p.p29 * s.db[330][9]);
        let eq71_e1837_d_b10: f64 = (p.p29 * s.db[330][10]);
        let eq71_e1837_d_b11: f64 = (p.p29 * s.db[330][11]);
        let eq71_e1837_d_b12: f64 = (p.p29 * s.db[330][12]);
        let eq71_e1837_d_b13: f64 = (p.p29 * s.db[330][13]);
        let eq71_e1838_q: f64 = eq71_e1837;
        let eq71_e1839: f64 = (s.v[187] * eq71_e1837);
        let eq71_e1839_d_n0: f64 = ((s.dn[187][0] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n0));
        let eq71_e1839_d_n1: f64 = ((s.dn[187][1] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n1));
        let eq71_e1839_d_n2: f64 = ((s.dn[187][2] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n2));
        let eq71_e1839_d_n3: f64 = ((s.dn[187][3] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n3));
        let eq71_e1839_d_n4: f64 = ((s.dn[187][4] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n4));
        let eq71_e1839_d_n5: f64 = ((s.dn[187][5] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n5));
        let eq71_e1839_d_n6: f64 = ((s.dn[187][6] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n6));
        let eq71_e1839_d_n7: f64 = ((s.dn[187][7] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n7));
        let eq71_e1839_d_n8: f64 = ((s.dn[187][8] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n8));
        let eq71_e1839_d_n9: f64 = ((s.dn[187][9] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n9));
        let eq71_e1839_d_n10: f64 = ((s.dn[187][10] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n10));
        let eq71_e1839_d_n11: f64 = ((s.dn[187][11] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n11));
        let eq71_e1839_d_n12: f64 = ((s.dn[187][12] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n12));
        let eq71_e1839_d_n13: f64 = ((s.dn[187][13] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n13));
        let eq71_e1839_d_n14: f64 = ((s.dn[187][14] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n14));
        let eq71_e1839_d_n15: f64 = ((s.dn[187][15] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n15));
        let eq71_e1839_d_n16: f64 = ((s.dn[187][16] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n16));
        let eq71_e1839_d_b0: f64 = ((s.db[187][0] * eq71_e1837) + (s.v[187] * eq71_e1837_d_b0));
        let eq71_e1839_d_b1: f64 = ((s.db[187][1] * eq71_e1837) + (s.v[187] * eq71_e1837_d_b1));
        let eq71_e1839_d_b2: f64 = ((s.db[187][2] * eq71_e1837) + (s.v[187] * eq71_e1837_d_b2));
        let eq71_e1839_d_b3: f64 = ((s.db[187][3] * eq71_e1837) + (s.v[187] * eq71_e1837_d_b3));
        let eq71_e1839_d_b4: f64 = ((s.db[187][4] * eq71_e1837) + (s.v[187] * eq71_e1837_d_b4));
        let eq71_e1839_d_b5: f64 = ((s.db[187][5] * eq71_e1837) + (s.v[187] * eq71_e1837_d_b5));
        let eq71_e1839_d_b6: f64 = ((s.db[187][6] * eq71_e1837) + (s.v[187] * eq71_e1837_d_b6));
        let eq71_e1839_d_b7: f64 = ((s.db[187][7] * eq71_e1837) + (s.v[187] * eq71_e1837_d_b7));
        let eq71_e1839_d_b8: f64 = ((s.db[187][8] * eq71_e1837) + (s.v[187] * eq71_e1837_d_b8));
        let eq71_e1839_d_b9: f64 = ((s.db[187][9] * eq71_e1837) + (s.v[187] * eq71_e1837_d_b9));
        let eq71_e1839_d_b10: f64 = ((s.db[187][10] * eq71_e1837) + (s.v[187] * eq71_e1837_d_b10));
        let eq71_e1839_d_b11: f64 = ((s.db[187][11] * eq71_e1837) + (s.v[187] * eq71_e1837_d_b11));
        let eq71_e1839_d_b12: f64 = ((s.db[187][12] * eq71_e1837) + (s.v[187] * eq71_e1837_d_b12));
        let eq71_e1839_d_b13: f64 = ((s.db[187][13] * eq71_e1837) + (s.v[187] * eq71_e1837_d_b13));
        let eq71_e1839_q: f64 = (s.v[187] * eq71_e1838_q);
        let eq71_e1839_q_d_n0: f64 = ((s.dn[187][0] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n0));
        let eq71_e1839_q_d_n1: f64 = ((s.dn[187][1] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n1));
        let eq71_e1839_q_d_n2: f64 = ((s.dn[187][2] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n2));
        let eq71_e1839_q_d_n3: f64 = ((s.dn[187][3] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n3));
        let eq71_e1839_q_d_n4: f64 = ((s.dn[187][4] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n4));
        let eq71_e1839_q_d_n5: f64 = ((s.dn[187][5] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n5));
        let eq71_e1839_q_d_n6: f64 = ((s.dn[187][6] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n6));
        let eq71_e1839_q_d_n7: f64 = ((s.dn[187][7] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n7));
        let eq71_e1839_q_d_n8: f64 = ((s.dn[187][8] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n8));
        let eq71_e1839_q_d_n9: f64 = ((s.dn[187][9] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n9));
        let eq71_e1839_q_d_n10: f64 = ((s.dn[187][10] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n10));
        let eq71_e1839_q_d_n11: f64 = ((s.dn[187][11] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n11));
        let eq71_e1839_q_d_n12: f64 = ((s.dn[187][12] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n12));
        let eq71_e1839_q_d_n13: f64 = ((s.dn[187][13] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n13));
        let eq71_e1839_q_d_n14: f64 = ((s.dn[187][14] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n14));
        let eq71_e1839_q_d_n15: f64 = ((s.dn[187][15] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n15));
        let eq71_e1839_q_d_n16: f64 = ((s.dn[187][16] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n16));
        let eq71_e1839_q_d_b0: f64 = ((s.db[187][0] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_b0));
        let eq71_e1839_q_d_b1: f64 = ((s.db[187][1] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_b1));
        let eq71_e1839_q_d_b2: f64 = ((s.db[187][2] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_b2));
        let eq71_e1839_q_d_b3: f64 = ((s.db[187][3] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_b3));
        let eq71_e1839_q_d_b4: f64 = ((s.db[187][4] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_b4));
        let eq71_e1839_q_d_b5: f64 = ((s.db[187][5] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_b5));
        let eq71_e1839_q_d_b6: f64 = ((s.db[187][6] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_b6));
        let eq71_e1839_q_d_b7: f64 = ((s.db[187][7] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_b7));
        let eq71_e1839_q_d_b8: f64 = ((s.db[187][8] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_b8));
        let eq71_e1839_q_d_b9: f64 = ((s.db[187][9] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_b9));
        let eq71_e1839_q_d_b10: f64 = ((s.db[187][10] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_b10));
        let eq71_e1839_q_d_b11: f64 = ((s.db[187][11] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_b11));
        let eq71_e1839_q_d_b12: f64 = ((s.db[187][12] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_b12));
        let eq71_e1839_q_d_b13: f64 = ((s.db[187][13] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_b13));
        (eq71_e1839, eq71_e1839_d_n0, eq71_e1839_d_n1, eq71_e1839_d_n2, eq71_e1839_d_n3, eq71_e1839_d_n4, eq71_e1839_d_n5, eq71_e1839_d_n6, eq71_e1839_d_n7, eq71_e1839_d_n8, eq71_e1839_d_n9, eq71_e1839_d_n10, eq71_e1839_d_n11, eq71_e1839_d_n12, eq71_e1839_d_n13, eq71_e1839_d_n14, eq71_e1839_d_n15, eq71_e1839_d_n16, eq71_e1839_d_b0, eq71_e1839_d_b1, eq71_e1839_d_b2, eq71_e1839_d_b3, eq71_e1839_d_b4, eq71_e1839_d_b5, eq71_e1839_d_b6, eq71_e1839_d_b7, eq71_e1839_d_b8, eq71_e1839_d_b9, eq71_e1839_d_b10, eq71_e1839_d_b11, eq71_e1839_d_b12, eq71_e1839_d_b13, eq71_e1839_q, eq71_e1839_q_d_n0, eq71_e1839_q_d_n1, eq71_e1839_q_d_n2, eq71_e1839_q_d_n3, eq71_e1839_q_d_n4, eq71_e1839_q_d_n5, eq71_e1839_q_d_n6, eq71_e1839_q_d_n7, eq71_e1839_q_d_n8, eq71_e1839_q_d_n9, eq71_e1839_q_d_n10, eq71_e1839_q_d_n11, eq71_e1839_q_d_n12, eq71_e1839_q_d_n13, eq71_e1839_q_d_n14, eq71_e1839_q_d_n15, eq71_e1839_q_d_n16, eq71_e1839_q_d_b0, eq71_e1839_q_d_b1, eq71_e1839_q_d_b2, eq71_e1839_q_d_b3, eq71_e1839_q_d_b4, eq71_e1839_q_d_b5, eq71_e1839_q_d_b6, eq71_e1839_q_d_b7, eq71_e1839_q_d_b8, eq71_e1839_q_d_b9, eq71_e1839_q_d_b10, eq71_e1839_q_d_b11, eq71_e1839_q_d_b12, eq71_e1839_q_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq71_reactive_node_derivatives: [f64; 17] = [eq71_e1841_q_d_n0, eq71_e1841_q_d_n1, eq71_e1841_q_d_n2, eq71_e1841_q_d_n3, eq71_e1841_q_d_n4, eq71_e1841_q_d_n5, eq71_e1841_q_d_n6, eq71_e1841_q_d_n7, eq71_e1841_q_d_n8, eq71_e1841_q_d_n9, eq71_e1841_q_d_n10, eq71_e1841_q_d_n11, eq71_e1841_q_d_n12, eq71_e1841_q_d_n13, eq71_e1841_q_d_n14, eq71_e1841_q_d_n15, eq71_e1841_q_d_n16];
        let eq71_reactive_branch_derivatives: [f64; 14] = [eq71_e1841_q_d_b0, eq71_e1841_q_d_b1, eq71_e1841_q_d_b2, eq71_e1841_q_d_b3, eq71_e1841_q_d_b4, eq71_e1841_q_d_b5, eq71_e1841_q_d_b6, eq71_e1841_q_d_b7, eq71_e1841_q_d_b8, eq71_e1841_q_d_b9, eq71_e1841_q_d_b10, eq71_e1841_q_d_b11, eq71_e1841_q_d_b12, eq71_e1841_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            nodes,
            &eq71_reactive_node_derivatives,
            branches,
            &eq71_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq73_e1868, eq73_e1868_d_n0, eq73_e1868_d_n1, eq73_e1868_d_n2, eq73_e1868_d_n3, eq73_e1868_d_n4, eq73_e1868_d_n5, eq73_e1868_d_n6, eq73_e1868_d_n7, eq73_e1868_d_n8, eq73_e1868_d_n9, eq73_e1868_d_n10, eq73_e1868_d_n11, eq73_e1868_d_n12, eq73_e1868_d_n13, eq73_e1868_d_n14, eq73_e1868_d_n15, eq73_e1868_d_n16, eq73_e1868_d_b0, eq73_e1868_d_b1, eq73_e1868_d_b2, eq73_e1868_d_b3, eq73_e1868_d_b4, eq73_e1868_d_b5, eq73_e1868_d_b6, eq73_e1868_d_b7, eq73_e1868_d_b8, eq73_e1868_d_b9, eq73_e1868_d_b10, eq73_e1868_d_b11, eq73_e1868_d_b12, eq73_e1868_d_b13, eq73_e1868_q, eq73_e1868_q_d_n0, eq73_e1868_q_d_n1, eq73_e1868_q_d_n2, eq73_e1868_q_d_n3, eq73_e1868_q_d_n4, eq73_e1868_q_d_n5, eq73_e1868_q_d_n6, eq73_e1868_q_d_n7, eq73_e1868_q_d_n8, eq73_e1868_q_d_n9, eq73_e1868_q_d_n10, eq73_e1868_q_d_n11, eq73_e1868_q_d_n12, eq73_e1868_q_d_n13, eq73_e1868_q_d_n14, eq73_e1868_q_d_n15, eq73_e1868_q_d_n16, eq73_e1868_q_d_b0, eq73_e1868_q_d_b1, eq73_e1868_q_d_b2, eq73_e1868_q_d_b3, eq73_e1868_q_d_b4, eq73_e1868_q_d_b5, eq73_e1868_q_d_b6, eq73_e1868_q_d_b7, eq73_e1868_q_d_b8, eq73_e1868_q_d_b9, eq73_e1868_q_d_b10, eq73_e1868_q_d_b11, eq73_e1868_q_d_b12, eq73_e1868_q_d_b13,) = {
    if (s.b[1627] && s.b[1628]) {
        let eq73_e1864: f64 = (p.p29 * s.v[334]);
        let eq73_e1864_d_n0: f64 = (p.p29 * s.dn[334][0]);
        let eq73_e1864_d_n1: f64 = (p.p29 * s.dn[334][1]);
        let eq73_e1864_d_n2: f64 = (p.p29 * s.dn[334][2]);
        let eq73_e1864_d_n3: f64 = (p.p29 * s.dn[334][3]);
        let eq73_e1864_d_n4: f64 = (p.p29 * s.dn[334][4]);
        let eq73_e1864_d_n5: f64 = (p.p29 * s.dn[334][5]);
        let eq73_e1864_d_n6: f64 = (p.p29 * s.dn[334][6]);
        let eq73_e1864_d_n7: f64 = (p.p29 * s.dn[334][7]);
        let eq73_e1864_d_n8: f64 = (p.p29 * s.dn[334][8]);
        let eq73_e1864_d_n9: f64 = (p.p29 * s.dn[334][9]);
        let eq73_e1864_d_n10: f64 = (p.p29 * s.dn[334][10]);
        let eq73_e1864_d_n11: f64 = (p.p29 * s.dn[334][11]);
        let eq73_e1864_d_n12: f64 = (p.p29 * s.dn[334][12]);
        let eq73_e1864_d_n13: f64 = (p.p29 * s.dn[334][13]);
        let eq73_e1864_d_n14: f64 = (p.p29 * s.dn[334][14]);
        let eq73_e1864_d_n15: f64 = (p.p29 * s.dn[334][15]);
        let eq73_e1864_d_n16: f64 = (p.p29 * s.dn[334][16]);
        let eq73_e1864_d_b0: f64 = (p.p29 * s.db[334][0]);
        let eq73_e1864_d_b1: f64 = (p.p29 * s.db[334][1]);
        let eq73_e1864_d_b2: f64 = (p.p29 * s.db[334][2]);
        let eq73_e1864_d_b3: f64 = (p.p29 * s.db[334][3]);
        let eq73_e1864_d_b4: f64 = (p.p29 * s.db[334][4]);
        let eq73_e1864_d_b5: f64 = (p.p29 * s.db[334][5]);
        let eq73_e1864_d_b6: f64 = (p.p29 * s.db[334][6]);
        let eq73_e1864_d_b7: f64 = (p.p29 * s.db[334][7]);
        let eq73_e1864_d_b8: f64 = (p.p29 * s.db[334][8]);
        let eq73_e1864_d_b9: f64 = (p.p29 * s.db[334][9]);
        let eq73_e1864_d_b10: f64 = (p.p29 * s.db[334][10]);
        let eq73_e1864_d_b11: f64 = (p.p29 * s.db[334][11]);
        let eq73_e1864_d_b12: f64 = (p.p29 * s.db[334][12]);
        let eq73_e1864_d_b13: f64 = (p.p29 * s.db[334][13]);
        let eq73_e1865_q: f64 = eq73_e1864;
        let eq73_e1866: f64 = (s.v[187] * eq73_e1864);
        let eq73_e1866_d_n0: f64 = ((s.dn[187][0] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n0));
        let eq73_e1866_d_n1: f64 = ((s.dn[187][1] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n1));
        let eq73_e1866_d_n2: f64 = ((s.dn[187][2] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n2));
        let eq73_e1866_d_n3: f64 = ((s.dn[187][3] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n3));
        let eq73_e1866_d_n4: f64 = ((s.dn[187][4] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n4));
        let eq73_e1866_d_n5: f64 = ((s.dn[187][5] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n5));
        let eq73_e1866_d_n6: f64 = ((s.dn[187][6] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n6));
        let eq73_e1866_d_n7: f64 = ((s.dn[187][7] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n7));
        let eq73_e1866_d_n8: f64 = ((s.dn[187][8] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n8));
        let eq73_e1866_d_n9: f64 = ((s.dn[187][9] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n9));
        let eq73_e1866_d_n10: f64 = ((s.dn[187][10] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n10));
        let eq73_e1866_d_n11: f64 = ((s.dn[187][11] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n11));
        let eq73_e1866_d_n12: f64 = ((s.dn[187][12] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n12));
        let eq73_e1866_d_n13: f64 = ((s.dn[187][13] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n13));
        let eq73_e1866_d_n14: f64 = ((s.dn[187][14] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n14));
        let eq73_e1866_d_n15: f64 = ((s.dn[187][15] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n15));
        let eq73_e1866_d_n16: f64 = ((s.dn[187][16] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n16));
        let eq73_e1866_d_b0: f64 = ((s.db[187][0] * eq73_e1864) + (s.v[187] * eq73_e1864_d_b0));
        let eq73_e1866_d_b1: f64 = ((s.db[187][1] * eq73_e1864) + (s.v[187] * eq73_e1864_d_b1));
        let eq73_e1866_d_b2: f64 = ((s.db[187][2] * eq73_e1864) + (s.v[187] * eq73_e1864_d_b2));
        let eq73_e1866_d_b3: f64 = ((s.db[187][3] * eq73_e1864) + (s.v[187] * eq73_e1864_d_b3));
        let eq73_e1866_d_b4: f64 = ((s.db[187][4] * eq73_e1864) + (s.v[187] * eq73_e1864_d_b4));
        let eq73_e1866_d_b5: f64 = ((s.db[187][5] * eq73_e1864) + (s.v[187] * eq73_e1864_d_b5));
        let eq73_e1866_d_b6: f64 = ((s.db[187][6] * eq73_e1864) + (s.v[187] * eq73_e1864_d_b6));
        let eq73_e1866_d_b7: f64 = ((s.db[187][7] * eq73_e1864) + (s.v[187] * eq73_e1864_d_b7));
        let eq73_e1866_d_b8: f64 = ((s.db[187][8] * eq73_e1864) + (s.v[187] * eq73_e1864_d_b8));
        let eq73_e1866_d_b9: f64 = ((s.db[187][9] * eq73_e1864) + (s.v[187] * eq73_e1864_d_b9));
        let eq73_e1866_d_b10: f64 = ((s.db[187][10] * eq73_e1864) + (s.v[187] * eq73_e1864_d_b10));
        let eq73_e1866_d_b11: f64 = ((s.db[187][11] * eq73_e1864) + (s.v[187] * eq73_e1864_d_b11));
        let eq73_e1866_d_b12: f64 = ((s.db[187][12] * eq73_e1864) + (s.v[187] * eq73_e1864_d_b12));
        let eq73_e1866_d_b13: f64 = ((s.db[187][13] * eq73_e1864) + (s.v[187] * eq73_e1864_d_b13));
        let eq73_e1866_q: f64 = (s.v[187] * eq73_e1865_q);
        let eq73_e1866_q_d_n0: f64 = ((s.dn[187][0] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n0));
        let eq73_e1866_q_d_n1: f64 = ((s.dn[187][1] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n1));
        let eq73_e1866_q_d_n2: f64 = ((s.dn[187][2] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n2));
        let eq73_e1866_q_d_n3: f64 = ((s.dn[187][3] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n3));
        let eq73_e1866_q_d_n4: f64 = ((s.dn[187][4] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n4));
        let eq73_e1866_q_d_n5: f64 = ((s.dn[187][5] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n5));
        let eq73_e1866_q_d_n6: f64 = ((s.dn[187][6] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n6));
        let eq73_e1866_q_d_n7: f64 = ((s.dn[187][7] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n7));
        let eq73_e1866_q_d_n8: f64 = ((s.dn[187][8] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n8));
        let eq73_e1866_q_d_n9: f64 = ((s.dn[187][9] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n9));
        let eq73_e1866_q_d_n10: f64 = ((s.dn[187][10] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n10));
        let eq73_e1866_q_d_n11: f64 = ((s.dn[187][11] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n11));
        let eq73_e1866_q_d_n12: f64 = ((s.dn[187][12] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n12));
        let eq73_e1866_q_d_n13: f64 = ((s.dn[187][13] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n13));
        let eq73_e1866_q_d_n14: f64 = ((s.dn[187][14] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n14));
        let eq73_e1866_q_d_n15: f64 = ((s.dn[187][15] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n15));
        let eq73_e1866_q_d_n16: f64 = ((s.dn[187][16] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n16));
        let eq73_e1866_q_d_b0: f64 = ((s.db[187][0] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_b0));
        let eq73_e1866_q_d_b1: f64 = ((s.db[187][1] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_b1));
        let eq73_e1866_q_d_b2: f64 = ((s.db[187][2] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_b2));
        let eq73_e1866_q_d_b3: f64 = ((s.db[187][3] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_b3));
        let eq73_e1866_q_d_b4: f64 = ((s.db[187][4] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_b4));
        let eq73_e1866_q_d_b5: f64 = ((s.db[187][5] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_b5));
        let eq73_e1866_q_d_b6: f64 = ((s.db[187][6] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_b6));
        let eq73_e1866_q_d_b7: f64 = ((s.db[187][7] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_b7));
        let eq73_e1866_q_d_b8: f64 = ((s.db[187][8] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_b8));
        let eq73_e1866_q_d_b9: f64 = ((s.db[187][9] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_b9));
        let eq73_e1866_q_d_b10: f64 = ((s.db[187][10] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_b10));
        let eq73_e1866_q_d_b11: f64 = ((s.db[187][11] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_b11));
        let eq73_e1866_q_d_b12: f64 = ((s.db[187][12] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_b12));
        let eq73_e1866_q_d_b13: f64 = ((s.db[187][13] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_b13));
        (eq73_e1866, eq73_e1866_d_n0, eq73_e1866_d_n1, eq73_e1866_d_n2, eq73_e1866_d_n3, eq73_e1866_d_n4, eq73_e1866_d_n5, eq73_e1866_d_n6, eq73_e1866_d_n7, eq73_e1866_d_n8, eq73_e1866_d_n9, eq73_e1866_d_n10, eq73_e1866_d_n11, eq73_e1866_d_n12, eq73_e1866_d_n13, eq73_e1866_d_n14, eq73_e1866_d_n15, eq73_e1866_d_n16, eq73_e1866_d_b0, eq73_e1866_d_b1, eq73_e1866_d_b2, eq73_e1866_d_b3, eq73_e1866_d_b4, eq73_e1866_d_b5, eq73_e1866_d_b6, eq73_e1866_d_b7, eq73_e1866_d_b8, eq73_e1866_d_b9, eq73_e1866_d_b10, eq73_e1866_d_b11, eq73_e1866_d_b12, eq73_e1866_d_b13, eq73_e1866_q, eq73_e1866_q_d_n0, eq73_e1866_q_d_n1, eq73_e1866_q_d_n2, eq73_e1866_q_d_n3, eq73_e1866_q_d_n4, eq73_e1866_q_d_n5, eq73_e1866_q_d_n6, eq73_e1866_q_d_n7, eq73_e1866_q_d_n8, eq73_e1866_q_d_n9, eq73_e1866_q_d_n10, eq73_e1866_q_d_n11, eq73_e1866_q_d_n12, eq73_e1866_q_d_n13, eq73_e1866_q_d_n14, eq73_e1866_q_d_n15, eq73_e1866_q_d_n16, eq73_e1866_q_d_b0, eq73_e1866_q_d_b1, eq73_e1866_q_d_b2, eq73_e1866_q_d_b3, eq73_e1866_q_d_b4, eq73_e1866_q_d_b5, eq73_e1866_q_d_b6, eq73_e1866_q_d_b7, eq73_e1866_q_d_b8, eq73_e1866_q_d_b9, eq73_e1866_q_d_b10, eq73_e1866_q_d_b11, eq73_e1866_q_d_b12, eq73_e1866_q_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_reactive_node_derivatives: [f64; 17] = [eq73_e1868_q_d_n0, eq73_e1868_q_d_n1, eq73_e1868_q_d_n2, eq73_e1868_q_d_n3, eq73_e1868_q_d_n4, eq73_e1868_q_d_n5, eq73_e1868_q_d_n6, eq73_e1868_q_d_n7, eq73_e1868_q_d_n8, eq73_e1868_q_d_n9, eq73_e1868_q_d_n10, eq73_e1868_q_d_n11, eq73_e1868_q_d_n12, eq73_e1868_q_d_n13, eq73_e1868_q_d_n14, eq73_e1868_q_d_n15, eq73_e1868_q_d_n16];
        let eq73_reactive_branch_derivatives: [f64; 14] = [eq73_e1868_q_d_b0, eq73_e1868_q_d_b1, eq73_e1868_q_d_b2, eq73_e1868_q_d_b3, eq73_e1868_q_d_b4, eq73_e1868_q_d_b5, eq73_e1868_q_d_b6, eq73_e1868_q_d_b7, eq73_e1868_q_d_b8, eq73_e1868_q_d_b9, eq73_e1868_q_d_b10, eq73_e1868_q_d_b11, eq73_e1868_q_d_b12, eq73_e1868_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[5]),
            nodes,
            &eq73_reactive_node_derivatives,
            branches,
            &eq73_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq76_e1908, eq76_e1908_d_n0, eq76_e1908_d_n1, eq76_e1908_d_n2, eq76_e1908_d_n3, eq76_e1908_d_n4, eq76_e1908_d_n5, eq76_e1908_d_n6, eq76_e1908_d_n7, eq76_e1908_d_n8, eq76_e1908_d_n9, eq76_e1908_d_n10, eq76_e1908_d_n11, eq76_e1908_d_n12, eq76_e1908_d_n13, eq76_e1908_d_n14, eq76_e1908_d_n15, eq76_e1908_d_n16, eq76_e1908_d_b0, eq76_e1908_d_b1, eq76_e1908_d_b2, eq76_e1908_d_b3, eq76_e1908_d_b4, eq76_e1908_d_b5, eq76_e1908_d_b6, eq76_e1908_d_b7, eq76_e1908_d_b8, eq76_e1908_d_b9, eq76_e1908_d_b10, eq76_e1908_d_b11, eq76_e1908_d_b12, eq76_e1908_d_b13, eq76_e1908_q, eq76_e1908_q_d_n0, eq76_e1908_q_d_n1, eq76_e1908_q_d_n2, eq76_e1908_q_d_n3, eq76_e1908_q_d_n4, eq76_e1908_q_d_n5, eq76_e1908_q_d_n6, eq76_e1908_q_d_n7, eq76_e1908_q_d_n8, eq76_e1908_q_d_n9, eq76_e1908_q_d_n10, eq76_e1908_q_d_n11, eq76_e1908_q_d_n12, eq76_e1908_q_d_n13, eq76_e1908_q_d_n14, eq76_e1908_q_d_n15, eq76_e1908_q_d_n16, eq76_e1908_q_d_b0, eq76_e1908_q_d_b1, eq76_e1908_q_d_b2, eq76_e1908_q_d_b3, eq76_e1908_q_d_b4, eq76_e1908_q_d_b5, eq76_e1908_q_d_b6, eq76_e1908_q_d_b7, eq76_e1908_q_d_b8, eq76_e1908_q_d_b9, eq76_e1908_q_d_b10, eq76_e1908_q_d_b11, eq76_e1908_q_d_b12, eq76_e1908_q_d_b13,) = {
    if (!s.b[1627]) {
        let eq76_e1904: f64 = (p.p29 * s.v[330]);
        let eq76_e1904_d_n0: f64 = (p.p29 * s.dn[330][0]);
        let eq76_e1904_d_n1: f64 = (p.p29 * s.dn[330][1]);
        let eq76_e1904_d_n2: f64 = (p.p29 * s.dn[330][2]);
        let eq76_e1904_d_n3: f64 = (p.p29 * s.dn[330][3]);
        let eq76_e1904_d_n4: f64 = (p.p29 * s.dn[330][4]);
        let eq76_e1904_d_n5: f64 = (p.p29 * s.dn[330][5]);
        let eq76_e1904_d_n6: f64 = (p.p29 * s.dn[330][6]);
        let eq76_e1904_d_n7: f64 = (p.p29 * s.dn[330][7]);
        let eq76_e1904_d_n8: f64 = (p.p29 * s.dn[330][8]);
        let eq76_e1904_d_n9: f64 = (p.p29 * s.dn[330][9]);
        let eq76_e1904_d_n10: f64 = (p.p29 * s.dn[330][10]);
        let eq76_e1904_d_n11: f64 = (p.p29 * s.dn[330][11]);
        let eq76_e1904_d_n12: f64 = (p.p29 * s.dn[330][12]);
        let eq76_e1904_d_n13: f64 = (p.p29 * s.dn[330][13]);
        let eq76_e1904_d_n14: f64 = (p.p29 * s.dn[330][14]);
        let eq76_e1904_d_n15: f64 = (p.p29 * s.dn[330][15]);
        let eq76_e1904_d_n16: f64 = (p.p29 * s.dn[330][16]);
        let eq76_e1904_d_b0: f64 = (p.p29 * s.db[330][0]);
        let eq76_e1904_d_b1: f64 = (p.p29 * s.db[330][1]);
        let eq76_e1904_d_b2: f64 = (p.p29 * s.db[330][2]);
        let eq76_e1904_d_b3: f64 = (p.p29 * s.db[330][3]);
        let eq76_e1904_d_b4: f64 = (p.p29 * s.db[330][4]);
        let eq76_e1904_d_b5: f64 = (p.p29 * s.db[330][5]);
        let eq76_e1904_d_b6: f64 = (p.p29 * s.db[330][6]);
        let eq76_e1904_d_b7: f64 = (p.p29 * s.db[330][7]);
        let eq76_e1904_d_b8: f64 = (p.p29 * s.db[330][8]);
        let eq76_e1904_d_b9: f64 = (p.p29 * s.db[330][9]);
        let eq76_e1904_d_b10: f64 = (p.p29 * s.db[330][10]);
        let eq76_e1904_d_b11: f64 = (p.p29 * s.db[330][11]);
        let eq76_e1904_d_b12: f64 = (p.p29 * s.db[330][12]);
        let eq76_e1904_d_b13: f64 = (p.p29 * s.db[330][13]);
        let eq76_e1905_q: f64 = eq76_e1904;
        let eq76_e1906: f64 = (s.v[187] * eq76_e1904);
        let eq76_e1906_d_n0: f64 = ((s.dn[187][0] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n0));
        let eq76_e1906_d_n1: f64 = ((s.dn[187][1] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n1));
        let eq76_e1906_d_n2: f64 = ((s.dn[187][2] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n2));
        let eq76_e1906_d_n3: f64 = ((s.dn[187][3] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n3));
        let eq76_e1906_d_n4: f64 = ((s.dn[187][4] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n4));
        let eq76_e1906_d_n5: f64 = ((s.dn[187][5] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n5));
        let eq76_e1906_d_n6: f64 = ((s.dn[187][6] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n6));
        let eq76_e1906_d_n7: f64 = ((s.dn[187][7] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n7));
        let eq76_e1906_d_n8: f64 = ((s.dn[187][8] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n8));
        let eq76_e1906_d_n9: f64 = ((s.dn[187][9] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n9));
        let eq76_e1906_d_n10: f64 = ((s.dn[187][10] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n10));
        let eq76_e1906_d_n11: f64 = ((s.dn[187][11] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n11));
        let eq76_e1906_d_n12: f64 = ((s.dn[187][12] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n12));
        let eq76_e1906_d_n13: f64 = ((s.dn[187][13] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n13));
        let eq76_e1906_d_n14: f64 = ((s.dn[187][14] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n14));
        let eq76_e1906_d_n15: f64 = ((s.dn[187][15] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n15));
        let eq76_e1906_d_n16: f64 = ((s.dn[187][16] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n16));
        let eq76_e1906_d_b0: f64 = ((s.db[187][0] * eq76_e1904) + (s.v[187] * eq76_e1904_d_b0));
        let eq76_e1906_d_b1: f64 = ((s.db[187][1] * eq76_e1904) + (s.v[187] * eq76_e1904_d_b1));
        let eq76_e1906_d_b2: f64 = ((s.db[187][2] * eq76_e1904) + (s.v[187] * eq76_e1904_d_b2));
        let eq76_e1906_d_b3: f64 = ((s.db[187][3] * eq76_e1904) + (s.v[187] * eq76_e1904_d_b3));
        let eq76_e1906_d_b4: f64 = ((s.db[187][4] * eq76_e1904) + (s.v[187] * eq76_e1904_d_b4));
        let eq76_e1906_d_b5: f64 = ((s.db[187][5] * eq76_e1904) + (s.v[187] * eq76_e1904_d_b5));
        let eq76_e1906_d_b6: f64 = ((s.db[187][6] * eq76_e1904) + (s.v[187] * eq76_e1904_d_b6));
        let eq76_e1906_d_b7: f64 = ((s.db[187][7] * eq76_e1904) + (s.v[187] * eq76_e1904_d_b7));
        let eq76_e1906_d_b8: f64 = ((s.db[187][8] * eq76_e1904) + (s.v[187] * eq76_e1904_d_b8));
        let eq76_e1906_d_b9: f64 = ((s.db[187][9] * eq76_e1904) + (s.v[187] * eq76_e1904_d_b9));
        let eq76_e1906_d_b10: f64 = ((s.db[187][10] * eq76_e1904) + (s.v[187] * eq76_e1904_d_b10));
        let eq76_e1906_d_b11: f64 = ((s.db[187][11] * eq76_e1904) + (s.v[187] * eq76_e1904_d_b11));
        let eq76_e1906_d_b12: f64 = ((s.db[187][12] * eq76_e1904) + (s.v[187] * eq76_e1904_d_b12));
        let eq76_e1906_d_b13: f64 = ((s.db[187][13] * eq76_e1904) + (s.v[187] * eq76_e1904_d_b13));
        let eq76_e1906_q: f64 = (s.v[187] * eq76_e1905_q);
        let eq76_e1906_q_d_n0: f64 = ((s.dn[187][0] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n0));
        let eq76_e1906_q_d_n1: f64 = ((s.dn[187][1] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n1));
        let eq76_e1906_q_d_n2: f64 = ((s.dn[187][2] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n2));
        let eq76_e1906_q_d_n3: f64 = ((s.dn[187][3] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n3));
        let eq76_e1906_q_d_n4: f64 = ((s.dn[187][4] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n4));
        let eq76_e1906_q_d_n5: f64 = ((s.dn[187][5] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n5));
        let eq76_e1906_q_d_n6: f64 = ((s.dn[187][6] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n6));
        let eq76_e1906_q_d_n7: f64 = ((s.dn[187][7] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n7));
        let eq76_e1906_q_d_n8: f64 = ((s.dn[187][8] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n8));
        let eq76_e1906_q_d_n9: f64 = ((s.dn[187][9] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n9));
        let eq76_e1906_q_d_n10: f64 = ((s.dn[187][10] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n10));
        let eq76_e1906_q_d_n11: f64 = ((s.dn[187][11] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n11));
        let eq76_e1906_q_d_n12: f64 = ((s.dn[187][12] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n12));
        let eq76_e1906_q_d_n13: f64 = ((s.dn[187][13] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n13));
        let eq76_e1906_q_d_n14: f64 = ((s.dn[187][14] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n14));
        let eq76_e1906_q_d_n15: f64 = ((s.dn[187][15] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n15));
        let eq76_e1906_q_d_n16: f64 = ((s.dn[187][16] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n16));
        let eq76_e1906_q_d_b0: f64 = ((s.db[187][0] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_b0));
        let eq76_e1906_q_d_b1: f64 = ((s.db[187][1] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_b1));
        let eq76_e1906_q_d_b2: f64 = ((s.db[187][2] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_b2));
        let eq76_e1906_q_d_b3: f64 = ((s.db[187][3] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_b3));
        let eq76_e1906_q_d_b4: f64 = ((s.db[187][4] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_b4));
        let eq76_e1906_q_d_b5: f64 = ((s.db[187][5] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_b5));
        let eq76_e1906_q_d_b6: f64 = ((s.db[187][6] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_b6));
        let eq76_e1906_q_d_b7: f64 = ((s.db[187][7] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_b7));
        let eq76_e1906_q_d_b8: f64 = ((s.db[187][8] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_b8));
        let eq76_e1906_q_d_b9: f64 = ((s.db[187][9] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_b9));
        let eq76_e1906_q_d_b10: f64 = ((s.db[187][10] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_b10));
        let eq76_e1906_q_d_b11: f64 = ((s.db[187][11] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_b11));
        let eq76_e1906_q_d_b12: f64 = ((s.db[187][12] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_b12));
        let eq76_e1906_q_d_b13: f64 = ((s.db[187][13] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_b13));
        (eq76_e1906, eq76_e1906_d_n0, eq76_e1906_d_n1, eq76_e1906_d_n2, eq76_e1906_d_n3, eq76_e1906_d_n4, eq76_e1906_d_n5, eq76_e1906_d_n6, eq76_e1906_d_n7, eq76_e1906_d_n8, eq76_e1906_d_n9, eq76_e1906_d_n10, eq76_e1906_d_n11, eq76_e1906_d_n12, eq76_e1906_d_n13, eq76_e1906_d_n14, eq76_e1906_d_n15, eq76_e1906_d_n16, eq76_e1906_d_b0, eq76_e1906_d_b1, eq76_e1906_d_b2, eq76_e1906_d_b3, eq76_e1906_d_b4, eq76_e1906_d_b5, eq76_e1906_d_b6, eq76_e1906_d_b7, eq76_e1906_d_b8, eq76_e1906_d_b9, eq76_e1906_d_b10, eq76_e1906_d_b11, eq76_e1906_d_b12, eq76_e1906_d_b13, eq76_e1906_q, eq76_e1906_q_d_n0, eq76_e1906_q_d_n1, eq76_e1906_q_d_n2, eq76_e1906_q_d_n3, eq76_e1906_q_d_n4, eq76_e1906_q_d_n5, eq76_e1906_q_d_n6, eq76_e1906_q_d_n7, eq76_e1906_q_d_n8, eq76_e1906_q_d_n9, eq76_e1906_q_d_n10, eq76_e1906_q_d_n11, eq76_e1906_q_d_n12, eq76_e1906_q_d_n13, eq76_e1906_q_d_n14, eq76_e1906_q_d_n15, eq76_e1906_q_d_n16, eq76_e1906_q_d_b0, eq76_e1906_q_d_b1, eq76_e1906_q_d_b2, eq76_e1906_q_d_b3, eq76_e1906_q_d_b4, eq76_e1906_q_d_b5, eq76_e1906_q_d_b6, eq76_e1906_q_d_b7, eq76_e1906_q_d_b8, eq76_e1906_q_d_b9, eq76_e1906_q_d_b10, eq76_e1906_q_d_b11, eq76_e1906_q_d_b12, eq76_e1906_q_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_reactive_node_derivatives: [f64; 17] = [eq76_e1908_q_d_n0, eq76_e1908_q_d_n1, eq76_e1908_q_d_n2, eq76_e1908_q_d_n3, eq76_e1908_q_d_n4, eq76_e1908_q_d_n5, eq76_e1908_q_d_n6, eq76_e1908_q_d_n7, eq76_e1908_q_d_n8, eq76_e1908_q_d_n9, eq76_e1908_q_d_n10, eq76_e1908_q_d_n11, eq76_e1908_q_d_n12, eq76_e1908_q_d_n13, eq76_e1908_q_d_n14, eq76_e1908_q_d_n15, eq76_e1908_q_d_n16];
        let eq76_reactive_branch_derivatives: [f64; 14] = [eq76_e1908_q_d_b0, eq76_e1908_q_d_b1, eq76_e1908_q_d_b2, eq76_e1908_q_d_b3, eq76_e1908_q_d_b4, eq76_e1908_q_d_b5, eq76_e1908_q_d_b6, eq76_e1908_q_d_b7, eq76_e1908_q_d_b8, eq76_e1908_q_d_b9, eq76_e1908_q_d_b10, eq76_e1908_q_d_b11, eq76_e1908_q_d_b12, eq76_e1908_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq76_reactive_node_derivatives,
            branches,
            &eq76_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_4(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq77_e1918, eq77_e1918_d_n0, eq77_e1918_d_n1, eq77_e1918_d_n2, eq77_e1918_d_n3, eq77_e1918_d_n4, eq77_e1918_d_n5, eq77_e1918_d_n6, eq77_e1918_d_n7, eq77_e1918_d_n8, eq77_e1918_d_n9, eq77_e1918_d_n10, eq77_e1918_d_n11, eq77_e1918_d_n12, eq77_e1918_d_n13, eq77_e1918_d_n14, eq77_e1918_d_n15, eq77_e1918_d_n16, eq77_e1918_d_b0, eq77_e1918_d_b1, eq77_e1918_d_b2, eq77_e1918_d_b3, eq77_e1918_d_b4, eq77_e1918_d_b5, eq77_e1918_d_b6, eq77_e1918_d_b7, eq77_e1918_d_b8, eq77_e1918_d_b9, eq77_e1918_d_b10, eq77_e1918_d_b11, eq77_e1918_d_b12, eq77_e1918_d_b13, eq77_e1918_q, eq77_e1918_q_d_n0, eq77_e1918_q_d_n1, eq77_e1918_q_d_n2, eq77_e1918_q_d_n3, eq77_e1918_q_d_n4, eq77_e1918_q_d_n5, eq77_e1918_q_d_n6, eq77_e1918_q_d_n7, eq77_e1918_q_d_n8, eq77_e1918_q_d_n9, eq77_e1918_q_d_n10, eq77_e1918_q_d_n11, eq77_e1918_q_d_n12, eq77_e1918_q_d_n13, eq77_e1918_q_d_n14, eq77_e1918_q_d_n15, eq77_e1918_q_d_n16, eq77_e1918_q_d_b0, eq77_e1918_q_d_b1, eq77_e1918_q_d_b2, eq77_e1918_q_d_b3, eq77_e1918_q_d_b4, eq77_e1918_q_d_b5, eq77_e1918_q_d_b6, eq77_e1918_q_d_b7, eq77_e1918_q_d_b8, eq77_e1918_q_d_b9, eq77_e1918_q_d_b10, eq77_e1918_q_d_b11, eq77_e1918_q_d_b12, eq77_e1918_q_d_b13,) = {
    if (!s.b[1627]) {
        let eq77_e1914: f64 = (p.p29 * s.v[334]);
        let eq77_e1914_d_n0: f64 = (p.p29 * s.dn[334][0]);
        let eq77_e1914_d_n1: f64 = (p.p29 * s.dn[334][1]);
        let eq77_e1914_d_n2: f64 = (p.p29 * s.dn[334][2]);
        let eq77_e1914_d_n3: f64 = (p.p29 * s.dn[334][3]);
        let eq77_e1914_d_n4: f64 = (p.p29 * s.dn[334][4]);
        let eq77_e1914_d_n5: f64 = (p.p29 * s.dn[334][5]);
        let eq77_e1914_d_n6: f64 = (p.p29 * s.dn[334][6]);
        let eq77_e1914_d_n7: f64 = (p.p29 * s.dn[334][7]);
        let eq77_e1914_d_n8: f64 = (p.p29 * s.dn[334][8]);
        let eq77_e1914_d_n9: f64 = (p.p29 * s.dn[334][9]);
        let eq77_e1914_d_n10: f64 = (p.p29 * s.dn[334][10]);
        let eq77_e1914_d_n11: f64 = (p.p29 * s.dn[334][11]);
        let eq77_e1914_d_n12: f64 = (p.p29 * s.dn[334][12]);
        let eq77_e1914_d_n13: f64 = (p.p29 * s.dn[334][13]);
        let eq77_e1914_d_n14: f64 = (p.p29 * s.dn[334][14]);
        let eq77_e1914_d_n15: f64 = (p.p29 * s.dn[334][15]);
        let eq77_e1914_d_n16: f64 = (p.p29 * s.dn[334][16]);
        let eq77_e1914_d_b0: f64 = (p.p29 * s.db[334][0]);
        let eq77_e1914_d_b1: f64 = (p.p29 * s.db[334][1]);
        let eq77_e1914_d_b2: f64 = (p.p29 * s.db[334][2]);
        let eq77_e1914_d_b3: f64 = (p.p29 * s.db[334][3]);
        let eq77_e1914_d_b4: f64 = (p.p29 * s.db[334][4]);
        let eq77_e1914_d_b5: f64 = (p.p29 * s.db[334][5]);
        let eq77_e1914_d_b6: f64 = (p.p29 * s.db[334][6]);
        let eq77_e1914_d_b7: f64 = (p.p29 * s.db[334][7]);
        let eq77_e1914_d_b8: f64 = (p.p29 * s.db[334][8]);
        let eq77_e1914_d_b9: f64 = (p.p29 * s.db[334][9]);
        let eq77_e1914_d_b10: f64 = (p.p29 * s.db[334][10]);
        let eq77_e1914_d_b11: f64 = (p.p29 * s.db[334][11]);
        let eq77_e1914_d_b12: f64 = (p.p29 * s.db[334][12]);
        let eq77_e1914_d_b13: f64 = (p.p29 * s.db[334][13]);
        let eq77_e1915_q: f64 = eq77_e1914;
        let eq77_e1916: f64 = (s.v[187] * eq77_e1914);
        let eq77_e1916_d_n0: f64 = ((s.dn[187][0] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n0));
        let eq77_e1916_d_n1: f64 = ((s.dn[187][1] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n1));
        let eq77_e1916_d_n2: f64 = ((s.dn[187][2] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n2));
        let eq77_e1916_d_n3: f64 = ((s.dn[187][3] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n3));
        let eq77_e1916_d_n4: f64 = ((s.dn[187][4] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n4));
        let eq77_e1916_d_n5: f64 = ((s.dn[187][5] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n5));
        let eq77_e1916_d_n6: f64 = ((s.dn[187][6] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n6));
        let eq77_e1916_d_n7: f64 = ((s.dn[187][7] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n7));
        let eq77_e1916_d_n8: f64 = ((s.dn[187][8] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n8));
        let eq77_e1916_d_n9: f64 = ((s.dn[187][9] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n9));
        let eq77_e1916_d_n10: f64 = ((s.dn[187][10] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n10));
        let eq77_e1916_d_n11: f64 = ((s.dn[187][11] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n11));
        let eq77_e1916_d_n12: f64 = ((s.dn[187][12] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n12));
        let eq77_e1916_d_n13: f64 = ((s.dn[187][13] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n13));
        let eq77_e1916_d_n14: f64 = ((s.dn[187][14] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n14));
        let eq77_e1916_d_n15: f64 = ((s.dn[187][15] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n15));
        let eq77_e1916_d_n16: f64 = ((s.dn[187][16] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n16));
        let eq77_e1916_d_b0: f64 = ((s.db[187][0] * eq77_e1914) + (s.v[187] * eq77_e1914_d_b0));
        let eq77_e1916_d_b1: f64 = ((s.db[187][1] * eq77_e1914) + (s.v[187] * eq77_e1914_d_b1));
        let eq77_e1916_d_b2: f64 = ((s.db[187][2] * eq77_e1914) + (s.v[187] * eq77_e1914_d_b2));
        let eq77_e1916_d_b3: f64 = ((s.db[187][3] * eq77_e1914) + (s.v[187] * eq77_e1914_d_b3));
        let eq77_e1916_d_b4: f64 = ((s.db[187][4] * eq77_e1914) + (s.v[187] * eq77_e1914_d_b4));
        let eq77_e1916_d_b5: f64 = ((s.db[187][5] * eq77_e1914) + (s.v[187] * eq77_e1914_d_b5));
        let eq77_e1916_d_b6: f64 = ((s.db[187][6] * eq77_e1914) + (s.v[187] * eq77_e1914_d_b6));
        let eq77_e1916_d_b7: f64 = ((s.db[187][7] * eq77_e1914) + (s.v[187] * eq77_e1914_d_b7));
        let eq77_e1916_d_b8: f64 = ((s.db[187][8] * eq77_e1914) + (s.v[187] * eq77_e1914_d_b8));
        let eq77_e1916_d_b9: f64 = ((s.db[187][9] * eq77_e1914) + (s.v[187] * eq77_e1914_d_b9));
        let eq77_e1916_d_b10: f64 = ((s.db[187][10] * eq77_e1914) + (s.v[187] * eq77_e1914_d_b10));
        let eq77_e1916_d_b11: f64 = ((s.db[187][11] * eq77_e1914) + (s.v[187] * eq77_e1914_d_b11));
        let eq77_e1916_d_b12: f64 = ((s.db[187][12] * eq77_e1914) + (s.v[187] * eq77_e1914_d_b12));
        let eq77_e1916_d_b13: f64 = ((s.db[187][13] * eq77_e1914) + (s.v[187] * eq77_e1914_d_b13));
        let eq77_e1916_q: f64 = (s.v[187] * eq77_e1915_q);
        let eq77_e1916_q_d_n0: f64 = ((s.dn[187][0] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n0));
        let eq77_e1916_q_d_n1: f64 = ((s.dn[187][1] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n1));
        let eq77_e1916_q_d_n2: f64 = ((s.dn[187][2] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n2));
        let eq77_e1916_q_d_n3: f64 = ((s.dn[187][3] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n3));
        let eq77_e1916_q_d_n4: f64 = ((s.dn[187][4] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n4));
        let eq77_e1916_q_d_n5: f64 = ((s.dn[187][5] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n5));
        let eq77_e1916_q_d_n6: f64 = ((s.dn[187][6] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n6));
        let eq77_e1916_q_d_n7: f64 = ((s.dn[187][7] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n7));
        let eq77_e1916_q_d_n8: f64 = ((s.dn[187][8] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n8));
        let eq77_e1916_q_d_n9: f64 = ((s.dn[187][9] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n9));
        let eq77_e1916_q_d_n10: f64 = ((s.dn[187][10] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n10));
        let eq77_e1916_q_d_n11: f64 = ((s.dn[187][11] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n11));
        let eq77_e1916_q_d_n12: f64 = ((s.dn[187][12] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n12));
        let eq77_e1916_q_d_n13: f64 = ((s.dn[187][13] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n13));
        let eq77_e1916_q_d_n14: f64 = ((s.dn[187][14] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n14));
        let eq77_e1916_q_d_n15: f64 = ((s.dn[187][15] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n15));
        let eq77_e1916_q_d_n16: f64 = ((s.dn[187][16] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n16));
        let eq77_e1916_q_d_b0: f64 = ((s.db[187][0] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_b0));
        let eq77_e1916_q_d_b1: f64 = ((s.db[187][1] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_b1));
        let eq77_e1916_q_d_b2: f64 = ((s.db[187][2] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_b2));
        let eq77_e1916_q_d_b3: f64 = ((s.db[187][3] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_b3));
        let eq77_e1916_q_d_b4: f64 = ((s.db[187][4] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_b4));
        let eq77_e1916_q_d_b5: f64 = ((s.db[187][5] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_b5));
        let eq77_e1916_q_d_b6: f64 = ((s.db[187][6] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_b6));
        let eq77_e1916_q_d_b7: f64 = ((s.db[187][7] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_b7));
        let eq77_e1916_q_d_b8: f64 = ((s.db[187][8] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_b8));
        let eq77_e1916_q_d_b9: f64 = ((s.db[187][9] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_b9));
        let eq77_e1916_q_d_b10: f64 = ((s.db[187][10] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_b10));
        let eq77_e1916_q_d_b11: f64 = ((s.db[187][11] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_b11));
        let eq77_e1916_q_d_b12: f64 = ((s.db[187][12] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_b12));
        let eq77_e1916_q_d_b13: f64 = ((s.db[187][13] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_b13));
        (eq77_e1916, eq77_e1916_d_n0, eq77_e1916_d_n1, eq77_e1916_d_n2, eq77_e1916_d_n3, eq77_e1916_d_n4, eq77_e1916_d_n5, eq77_e1916_d_n6, eq77_e1916_d_n7, eq77_e1916_d_n8, eq77_e1916_d_n9, eq77_e1916_d_n10, eq77_e1916_d_n11, eq77_e1916_d_n12, eq77_e1916_d_n13, eq77_e1916_d_n14, eq77_e1916_d_n15, eq77_e1916_d_n16, eq77_e1916_d_b0, eq77_e1916_d_b1, eq77_e1916_d_b2, eq77_e1916_d_b3, eq77_e1916_d_b4, eq77_e1916_d_b5, eq77_e1916_d_b6, eq77_e1916_d_b7, eq77_e1916_d_b8, eq77_e1916_d_b9, eq77_e1916_d_b10, eq77_e1916_d_b11, eq77_e1916_d_b12, eq77_e1916_d_b13, eq77_e1916_q, eq77_e1916_q_d_n0, eq77_e1916_q_d_n1, eq77_e1916_q_d_n2, eq77_e1916_q_d_n3, eq77_e1916_q_d_n4, eq77_e1916_q_d_n5, eq77_e1916_q_d_n6, eq77_e1916_q_d_n7, eq77_e1916_q_d_n8, eq77_e1916_q_d_n9, eq77_e1916_q_d_n10, eq77_e1916_q_d_n11, eq77_e1916_q_d_n12, eq77_e1916_q_d_n13, eq77_e1916_q_d_n14, eq77_e1916_q_d_n15, eq77_e1916_q_d_n16, eq77_e1916_q_d_b0, eq77_e1916_q_d_b1, eq77_e1916_q_d_b2, eq77_e1916_q_d_b3, eq77_e1916_q_d_b4, eq77_e1916_q_d_b5, eq77_e1916_q_d_b6, eq77_e1916_q_d_b7, eq77_e1916_q_d_b8, eq77_e1916_q_d_b9, eq77_e1916_q_d_b10, eq77_e1916_q_d_b11, eq77_e1916_q_d_b12, eq77_e1916_q_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_reactive_node_derivatives: [f64; 17] = [eq77_e1918_q_d_n0, eq77_e1918_q_d_n1, eq77_e1918_q_d_n2, eq77_e1918_q_d_n3, eq77_e1918_q_d_n4, eq77_e1918_q_d_n5, eq77_e1918_q_d_n6, eq77_e1918_q_d_n7, eq77_e1918_q_d_n8, eq77_e1918_q_d_n9, eq77_e1918_q_d_n10, eq77_e1918_q_d_n11, eq77_e1918_q_d_n12, eq77_e1918_q_d_n13, eq77_e1918_q_d_n14, eq77_e1918_q_d_n15, eq77_e1918_q_d_n16];
        let eq77_reactive_branch_derivatives: [f64; 14] = [eq77_e1918_q_d_b0, eq77_e1918_q_d_b1, eq77_e1918_q_d_b2, eq77_e1918_q_d_b3, eq77_e1918_q_d_b4, eq77_e1918_q_d_b5, eq77_e1918_q_d_b6, eq77_e1918_q_d_b7, eq77_e1918_q_d_b8, eq77_e1918_q_d_b9, eq77_e1918_q_d_b10, eq77_e1918_q_d_b11, eq77_e1918_q_d_b12, eq77_e1918_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[5]),
            nodes,
            &eq77_reactive_node_derivatives,
            branches,
            &eq77_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq83_e1984, eq83_e1984_d_n0, eq83_e1984_d_n1, eq83_e1984_d_n2, eq83_e1984_d_n3, eq83_e1984_d_n4, eq83_e1984_d_n5, eq83_e1984_d_n6, eq83_e1984_d_n7, eq83_e1984_d_n8, eq83_e1984_d_n9, eq83_e1984_d_n10, eq83_e1984_d_n11, eq83_e1984_d_n12, eq83_e1984_d_n13, eq83_e1984_d_n14, eq83_e1984_d_n15, eq83_e1984_d_n16, eq83_e1984_d_b0, eq83_e1984_d_b1, eq83_e1984_d_b2, eq83_e1984_d_b3, eq83_e1984_d_b4, eq83_e1984_d_b5, eq83_e1984_d_b6, eq83_e1984_d_b7, eq83_e1984_d_b8, eq83_e1984_d_b9, eq83_e1984_d_b10, eq83_e1984_d_b11, eq83_e1984_d_b12, eq83_e1984_d_b13, eq83_e1984_q, eq83_e1984_q_d_n0, eq83_e1984_q_d_n1, eq83_e1984_q_d_n2, eq83_e1984_q_d_n3, eq83_e1984_q_d_n4, eq83_e1984_q_d_n5, eq83_e1984_q_d_n6, eq83_e1984_q_d_n7, eq83_e1984_q_d_n8, eq83_e1984_q_d_n9, eq83_e1984_q_d_n10, eq83_e1984_q_d_n11, eq83_e1984_q_d_n12, eq83_e1984_q_d_n13, eq83_e1984_q_d_n14, eq83_e1984_q_d_n15, eq83_e1984_q_d_n16, eq83_e1984_q_d_b0, eq83_e1984_q_d_b1, eq83_e1984_q_d_b2, eq83_e1984_q_d_b3, eq83_e1984_q_d_b4, eq83_e1984_q_d_b5, eq83_e1984_q_d_b6, eq83_e1984_q_d_b7, eq83_e1984_q_d_b8, eq83_e1984_q_d_b9, eq83_e1984_q_d_b10, eq83_e1984_q_d_b11, eq83_e1984_q_d_b12, eq83_e1984_q_d_b13,) = {
    if s.b[1630] {
        let eq83_e1980: f64 = (p.p29 * s.v[334]);
        let eq83_e1980_d_n0: f64 = (p.p29 * s.dn[334][0]);
        let eq83_e1980_d_n1: f64 = (p.p29 * s.dn[334][1]);
        let eq83_e1980_d_n2: f64 = (p.p29 * s.dn[334][2]);
        let eq83_e1980_d_n3: f64 = (p.p29 * s.dn[334][3]);
        let eq83_e1980_d_n4: f64 = (p.p29 * s.dn[334][4]);
        let eq83_e1980_d_n5: f64 = (p.p29 * s.dn[334][5]);
        let eq83_e1980_d_n6: f64 = (p.p29 * s.dn[334][6]);
        let eq83_e1980_d_n7: f64 = (p.p29 * s.dn[334][7]);
        let eq83_e1980_d_n8: f64 = (p.p29 * s.dn[334][8]);
        let eq83_e1980_d_n9: f64 = (p.p29 * s.dn[334][9]);
        let eq83_e1980_d_n10: f64 = (p.p29 * s.dn[334][10]);
        let eq83_e1980_d_n11: f64 = (p.p29 * s.dn[334][11]);
        let eq83_e1980_d_n12: f64 = (p.p29 * s.dn[334][12]);
        let eq83_e1980_d_n13: f64 = (p.p29 * s.dn[334][13]);
        let eq83_e1980_d_n14: f64 = (p.p29 * s.dn[334][14]);
        let eq83_e1980_d_n15: f64 = (p.p29 * s.dn[334][15]);
        let eq83_e1980_d_n16: f64 = (p.p29 * s.dn[334][16]);
        let eq83_e1980_d_b0: f64 = (p.p29 * s.db[334][0]);
        let eq83_e1980_d_b1: f64 = (p.p29 * s.db[334][1]);
        let eq83_e1980_d_b2: f64 = (p.p29 * s.db[334][2]);
        let eq83_e1980_d_b3: f64 = (p.p29 * s.db[334][3]);
        let eq83_e1980_d_b4: f64 = (p.p29 * s.db[334][4]);
        let eq83_e1980_d_b5: f64 = (p.p29 * s.db[334][5]);
        let eq83_e1980_d_b6: f64 = (p.p29 * s.db[334][6]);
        let eq83_e1980_d_b7: f64 = (p.p29 * s.db[334][7]);
        let eq83_e1980_d_b8: f64 = (p.p29 * s.db[334][8]);
        let eq83_e1980_d_b9: f64 = (p.p29 * s.db[334][9]);
        let eq83_e1980_d_b10: f64 = (p.p29 * s.db[334][10]);
        let eq83_e1980_d_b11: f64 = (p.p29 * s.db[334][11]);
        let eq83_e1980_d_b12: f64 = (p.p29 * s.db[334][12]);
        let eq83_e1980_d_b13: f64 = (p.p29 * s.db[334][13]);
        let eq83_e1981_q: f64 = eq83_e1980;
        let eq83_e1982: f64 = (s.v[187] * eq83_e1980);
        let eq83_e1982_d_n0: f64 = ((s.dn[187][0] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n0));
        let eq83_e1982_d_n1: f64 = ((s.dn[187][1] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n1));
        let eq83_e1982_d_n2: f64 = ((s.dn[187][2] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n2));
        let eq83_e1982_d_n3: f64 = ((s.dn[187][3] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n3));
        let eq83_e1982_d_n4: f64 = ((s.dn[187][4] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n4));
        let eq83_e1982_d_n5: f64 = ((s.dn[187][5] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n5));
        let eq83_e1982_d_n6: f64 = ((s.dn[187][6] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n6));
        let eq83_e1982_d_n7: f64 = ((s.dn[187][7] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n7));
        let eq83_e1982_d_n8: f64 = ((s.dn[187][8] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n8));
        let eq83_e1982_d_n9: f64 = ((s.dn[187][9] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n9));
        let eq83_e1982_d_n10: f64 = ((s.dn[187][10] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n10));
        let eq83_e1982_d_n11: f64 = ((s.dn[187][11] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n11));
        let eq83_e1982_d_n12: f64 = ((s.dn[187][12] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n12));
        let eq83_e1982_d_n13: f64 = ((s.dn[187][13] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n13));
        let eq83_e1982_d_n14: f64 = ((s.dn[187][14] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n14));
        let eq83_e1982_d_n15: f64 = ((s.dn[187][15] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n15));
        let eq83_e1982_d_n16: f64 = ((s.dn[187][16] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n16));
        let eq83_e1982_d_b0: f64 = ((s.db[187][0] * eq83_e1980) + (s.v[187] * eq83_e1980_d_b0));
        let eq83_e1982_d_b1: f64 = ((s.db[187][1] * eq83_e1980) + (s.v[187] * eq83_e1980_d_b1));
        let eq83_e1982_d_b2: f64 = ((s.db[187][2] * eq83_e1980) + (s.v[187] * eq83_e1980_d_b2));
        let eq83_e1982_d_b3: f64 = ((s.db[187][3] * eq83_e1980) + (s.v[187] * eq83_e1980_d_b3));
        let eq83_e1982_d_b4: f64 = ((s.db[187][4] * eq83_e1980) + (s.v[187] * eq83_e1980_d_b4));
        let eq83_e1982_d_b5: f64 = ((s.db[187][5] * eq83_e1980) + (s.v[187] * eq83_e1980_d_b5));
        let eq83_e1982_d_b6: f64 = ((s.db[187][6] * eq83_e1980) + (s.v[187] * eq83_e1980_d_b6));
        let eq83_e1982_d_b7: f64 = ((s.db[187][7] * eq83_e1980) + (s.v[187] * eq83_e1980_d_b7));
        let eq83_e1982_d_b8: f64 = ((s.db[187][8] * eq83_e1980) + (s.v[187] * eq83_e1980_d_b8));
        let eq83_e1982_d_b9: f64 = ((s.db[187][9] * eq83_e1980) + (s.v[187] * eq83_e1980_d_b9));
        let eq83_e1982_d_b10: f64 = ((s.db[187][10] * eq83_e1980) + (s.v[187] * eq83_e1980_d_b10));
        let eq83_e1982_d_b11: f64 = ((s.db[187][11] * eq83_e1980) + (s.v[187] * eq83_e1980_d_b11));
        let eq83_e1982_d_b12: f64 = ((s.db[187][12] * eq83_e1980) + (s.v[187] * eq83_e1980_d_b12));
        let eq83_e1982_d_b13: f64 = ((s.db[187][13] * eq83_e1980) + (s.v[187] * eq83_e1980_d_b13));
        let eq83_e1982_q: f64 = (s.v[187] * eq83_e1981_q);
        let eq83_e1982_q_d_n0: f64 = ((s.dn[187][0] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n0));
        let eq83_e1982_q_d_n1: f64 = ((s.dn[187][1] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n1));
        let eq83_e1982_q_d_n2: f64 = ((s.dn[187][2] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n2));
        let eq83_e1982_q_d_n3: f64 = ((s.dn[187][3] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n3));
        let eq83_e1982_q_d_n4: f64 = ((s.dn[187][4] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n4));
        let eq83_e1982_q_d_n5: f64 = ((s.dn[187][5] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n5));
        let eq83_e1982_q_d_n6: f64 = ((s.dn[187][6] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n6));
        let eq83_e1982_q_d_n7: f64 = ((s.dn[187][7] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n7));
        let eq83_e1982_q_d_n8: f64 = ((s.dn[187][8] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n8));
        let eq83_e1982_q_d_n9: f64 = ((s.dn[187][9] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n9));
        let eq83_e1982_q_d_n10: f64 = ((s.dn[187][10] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n10));
        let eq83_e1982_q_d_n11: f64 = ((s.dn[187][11] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n11));
        let eq83_e1982_q_d_n12: f64 = ((s.dn[187][12] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n12));
        let eq83_e1982_q_d_n13: f64 = ((s.dn[187][13] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n13));
        let eq83_e1982_q_d_n14: f64 = ((s.dn[187][14] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n14));
        let eq83_e1982_q_d_n15: f64 = ((s.dn[187][15] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n15));
        let eq83_e1982_q_d_n16: f64 = ((s.dn[187][16] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n16));
        let eq83_e1982_q_d_b0: f64 = ((s.db[187][0] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_b0));
        let eq83_e1982_q_d_b1: f64 = ((s.db[187][1] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_b1));
        let eq83_e1982_q_d_b2: f64 = ((s.db[187][2] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_b2));
        let eq83_e1982_q_d_b3: f64 = ((s.db[187][3] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_b3));
        let eq83_e1982_q_d_b4: f64 = ((s.db[187][4] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_b4));
        let eq83_e1982_q_d_b5: f64 = ((s.db[187][5] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_b5));
        let eq83_e1982_q_d_b6: f64 = ((s.db[187][6] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_b6));
        let eq83_e1982_q_d_b7: f64 = ((s.db[187][7] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_b7));
        let eq83_e1982_q_d_b8: f64 = ((s.db[187][8] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_b8));
        let eq83_e1982_q_d_b9: f64 = ((s.db[187][9] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_b9));
        let eq83_e1982_q_d_b10: f64 = ((s.db[187][10] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_b10));
        let eq83_e1982_q_d_b11: f64 = ((s.db[187][11] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_b11));
        let eq83_e1982_q_d_b12: f64 = ((s.db[187][12] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_b12));
        let eq83_e1982_q_d_b13: f64 = ((s.db[187][13] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_b13));
        (eq83_e1982, eq83_e1982_d_n0, eq83_e1982_d_n1, eq83_e1982_d_n2, eq83_e1982_d_n3, eq83_e1982_d_n4, eq83_e1982_d_n5, eq83_e1982_d_n6, eq83_e1982_d_n7, eq83_e1982_d_n8, eq83_e1982_d_n9, eq83_e1982_d_n10, eq83_e1982_d_n11, eq83_e1982_d_n12, eq83_e1982_d_n13, eq83_e1982_d_n14, eq83_e1982_d_n15, eq83_e1982_d_n16, eq83_e1982_d_b0, eq83_e1982_d_b1, eq83_e1982_d_b2, eq83_e1982_d_b3, eq83_e1982_d_b4, eq83_e1982_d_b5, eq83_e1982_d_b6, eq83_e1982_d_b7, eq83_e1982_d_b8, eq83_e1982_d_b9, eq83_e1982_d_b10, eq83_e1982_d_b11, eq83_e1982_d_b12, eq83_e1982_d_b13, eq83_e1982_q, eq83_e1982_q_d_n0, eq83_e1982_q_d_n1, eq83_e1982_q_d_n2, eq83_e1982_q_d_n3, eq83_e1982_q_d_n4, eq83_e1982_q_d_n5, eq83_e1982_q_d_n6, eq83_e1982_q_d_n7, eq83_e1982_q_d_n8, eq83_e1982_q_d_n9, eq83_e1982_q_d_n10, eq83_e1982_q_d_n11, eq83_e1982_q_d_n12, eq83_e1982_q_d_n13, eq83_e1982_q_d_n14, eq83_e1982_q_d_n15, eq83_e1982_q_d_n16, eq83_e1982_q_d_b0, eq83_e1982_q_d_b1, eq83_e1982_q_d_b2, eq83_e1982_q_d_b3, eq83_e1982_q_d_b4, eq83_e1982_q_d_b5, eq83_e1982_q_d_b6, eq83_e1982_q_d_b7, eq83_e1982_q_d_b8, eq83_e1982_q_d_b9, eq83_e1982_q_d_b10, eq83_e1982_q_d_b11, eq83_e1982_q_d_b12, eq83_e1982_q_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq83_reactive_node_derivatives: [f64; 17] = [eq83_e1984_q_d_n0, eq83_e1984_q_d_n1, eq83_e1984_q_d_n2, eq83_e1984_q_d_n3, eq83_e1984_q_d_n4, eq83_e1984_q_d_n5, eq83_e1984_q_d_n6, eq83_e1984_q_d_n7, eq83_e1984_q_d_n8, eq83_e1984_q_d_n9, eq83_e1984_q_d_n10, eq83_e1984_q_d_n11, eq83_e1984_q_d_n12, eq83_e1984_q_d_n13, eq83_e1984_q_d_n14, eq83_e1984_q_d_n15, eq83_e1984_q_d_n16];
        let eq83_reactive_branch_derivatives: [f64; 14] = [eq83_e1984_q_d_b0, eq83_e1984_q_d_b1, eq83_e1984_q_d_b2, eq83_e1984_q_d_b3, eq83_e1984_q_d_b4, eq83_e1984_q_d_b5, eq83_e1984_q_d_b6, eq83_e1984_q_d_b7, eq83_e1984_q_d_b8, eq83_e1984_q_d_b9, eq83_e1984_q_d_b10, eq83_e1984_q_d_b11, eq83_e1984_q_d_b12, eq83_e1984_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[5]),
            nodes,
            &eq83_reactive_node_derivatives,
            branches,
            &eq83_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq84_e1993, eq84_e1993_d_n0, eq84_e1993_d_n1, eq84_e1993_d_n2, eq84_e1993_d_n3, eq84_e1993_d_n4, eq84_e1993_d_n5, eq84_e1993_d_n6, eq84_e1993_d_n7, eq84_e1993_d_n8, eq84_e1993_d_n9, eq84_e1993_d_n10, eq84_e1993_d_n11, eq84_e1993_d_n12, eq84_e1993_d_n13, eq84_e1993_d_n14, eq84_e1993_d_n15, eq84_e1993_d_n16, eq84_e1993_d_b0, eq84_e1993_d_b1, eq84_e1993_d_b2, eq84_e1993_d_b3, eq84_e1993_d_b4, eq84_e1993_d_b5, eq84_e1993_d_b6, eq84_e1993_d_b7, eq84_e1993_d_b8, eq84_e1993_d_b9, eq84_e1993_d_b10, eq84_e1993_d_b11, eq84_e1993_d_b12, eq84_e1993_d_b13, eq84_e1993_q, eq84_e1993_q_d_n0, eq84_e1993_q_d_n1, eq84_e1993_q_d_n2, eq84_e1993_q_d_n3, eq84_e1993_q_d_n4, eq84_e1993_q_d_n5, eq84_e1993_q_d_n6, eq84_e1993_q_d_n7, eq84_e1993_q_d_n8, eq84_e1993_q_d_n9, eq84_e1993_q_d_n10, eq84_e1993_q_d_n11, eq84_e1993_q_d_n12, eq84_e1993_q_d_n13, eq84_e1993_q_d_n14, eq84_e1993_q_d_n15, eq84_e1993_q_d_n16, eq84_e1993_q_d_b0, eq84_e1993_q_d_b1, eq84_e1993_q_d_b2, eq84_e1993_q_d_b3, eq84_e1993_q_d_b4, eq84_e1993_q_d_b5, eq84_e1993_q_d_b6, eq84_e1993_q_d_b7, eq84_e1993_q_d_b8, eq84_e1993_q_d_b9, eq84_e1993_q_d_b10, eq84_e1993_q_d_b11, eq84_e1993_q_d_b12, eq84_e1993_q_d_b13,) = {
    if s.b[1630] {
        let eq84_e1989: f64 = (p.p29 * s.v[338]);
        let eq84_e1989_d_n0: f64 = (p.p29 * s.dn[338][0]);
        let eq84_e1989_d_n1: f64 = (p.p29 * s.dn[338][1]);
        let eq84_e1989_d_n2: f64 = (p.p29 * s.dn[338][2]);
        let eq84_e1989_d_n3: f64 = (p.p29 * s.dn[338][3]);
        let eq84_e1989_d_n4: f64 = (p.p29 * s.dn[338][4]);
        let eq84_e1989_d_n5: f64 = (p.p29 * s.dn[338][5]);
        let eq84_e1989_d_n6: f64 = (p.p29 * s.dn[338][6]);
        let eq84_e1989_d_n7: f64 = (p.p29 * s.dn[338][7]);
        let eq84_e1989_d_n8: f64 = (p.p29 * s.dn[338][8]);
        let eq84_e1989_d_n9: f64 = (p.p29 * s.dn[338][9]);
        let eq84_e1989_d_n10: f64 = (p.p29 * s.dn[338][10]);
        let eq84_e1989_d_n11: f64 = (p.p29 * s.dn[338][11]);
        let eq84_e1989_d_n12: f64 = (p.p29 * s.dn[338][12]);
        let eq84_e1989_d_n13: f64 = (p.p29 * s.dn[338][13]);
        let eq84_e1989_d_n14: f64 = (p.p29 * s.dn[338][14]);
        let eq84_e1989_d_n15: f64 = (p.p29 * s.dn[338][15]);
        let eq84_e1989_d_n16: f64 = (p.p29 * s.dn[338][16]);
        let eq84_e1989_d_b0: f64 = (p.p29 * s.db[338][0]);
        let eq84_e1989_d_b1: f64 = (p.p29 * s.db[338][1]);
        let eq84_e1989_d_b2: f64 = (p.p29 * s.db[338][2]);
        let eq84_e1989_d_b3: f64 = (p.p29 * s.db[338][3]);
        let eq84_e1989_d_b4: f64 = (p.p29 * s.db[338][4]);
        let eq84_e1989_d_b5: f64 = (p.p29 * s.db[338][5]);
        let eq84_e1989_d_b6: f64 = (p.p29 * s.db[338][6]);
        let eq84_e1989_d_b7: f64 = (p.p29 * s.db[338][7]);
        let eq84_e1989_d_b8: f64 = (p.p29 * s.db[338][8]);
        let eq84_e1989_d_b9: f64 = (p.p29 * s.db[338][9]);
        let eq84_e1989_d_b10: f64 = (p.p29 * s.db[338][10]);
        let eq84_e1989_d_b11: f64 = (p.p29 * s.db[338][11]);
        let eq84_e1989_d_b12: f64 = (p.p29 * s.db[338][12]);
        let eq84_e1989_d_b13: f64 = (p.p29 * s.db[338][13]);
        let eq84_e1990_q: f64 = eq84_e1989;
        let eq84_e1991: f64 = (s.v[187] * eq84_e1989);
        let eq84_e1991_d_n0: f64 = ((s.dn[187][0] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n0));
        let eq84_e1991_d_n1: f64 = ((s.dn[187][1] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n1));
        let eq84_e1991_d_n2: f64 = ((s.dn[187][2] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n2));
        let eq84_e1991_d_n3: f64 = ((s.dn[187][3] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n3));
        let eq84_e1991_d_n4: f64 = ((s.dn[187][4] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n4));
        let eq84_e1991_d_n5: f64 = ((s.dn[187][5] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n5));
        let eq84_e1991_d_n6: f64 = ((s.dn[187][6] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n6));
        let eq84_e1991_d_n7: f64 = ((s.dn[187][7] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n7));
        let eq84_e1991_d_n8: f64 = ((s.dn[187][8] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n8));
        let eq84_e1991_d_n9: f64 = ((s.dn[187][9] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n9));
        let eq84_e1991_d_n10: f64 = ((s.dn[187][10] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n10));
        let eq84_e1991_d_n11: f64 = ((s.dn[187][11] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n11));
        let eq84_e1991_d_n12: f64 = ((s.dn[187][12] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n12));
        let eq84_e1991_d_n13: f64 = ((s.dn[187][13] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n13));
        let eq84_e1991_d_n14: f64 = ((s.dn[187][14] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n14));
        let eq84_e1991_d_n15: f64 = ((s.dn[187][15] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n15));
        let eq84_e1991_d_n16: f64 = ((s.dn[187][16] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n16));
        let eq84_e1991_d_b0: f64 = ((s.db[187][0] * eq84_e1989) + (s.v[187] * eq84_e1989_d_b0));
        let eq84_e1991_d_b1: f64 = ((s.db[187][1] * eq84_e1989) + (s.v[187] * eq84_e1989_d_b1));
        let eq84_e1991_d_b2: f64 = ((s.db[187][2] * eq84_e1989) + (s.v[187] * eq84_e1989_d_b2));
        let eq84_e1991_d_b3: f64 = ((s.db[187][3] * eq84_e1989) + (s.v[187] * eq84_e1989_d_b3));
        let eq84_e1991_d_b4: f64 = ((s.db[187][4] * eq84_e1989) + (s.v[187] * eq84_e1989_d_b4));
        let eq84_e1991_d_b5: f64 = ((s.db[187][5] * eq84_e1989) + (s.v[187] * eq84_e1989_d_b5));
        let eq84_e1991_d_b6: f64 = ((s.db[187][6] * eq84_e1989) + (s.v[187] * eq84_e1989_d_b6));
        let eq84_e1991_d_b7: f64 = ((s.db[187][7] * eq84_e1989) + (s.v[187] * eq84_e1989_d_b7));
        let eq84_e1991_d_b8: f64 = ((s.db[187][8] * eq84_e1989) + (s.v[187] * eq84_e1989_d_b8));
        let eq84_e1991_d_b9: f64 = ((s.db[187][9] * eq84_e1989) + (s.v[187] * eq84_e1989_d_b9));
        let eq84_e1991_d_b10: f64 = ((s.db[187][10] * eq84_e1989) + (s.v[187] * eq84_e1989_d_b10));
        let eq84_e1991_d_b11: f64 = ((s.db[187][11] * eq84_e1989) + (s.v[187] * eq84_e1989_d_b11));
        let eq84_e1991_d_b12: f64 = ((s.db[187][12] * eq84_e1989) + (s.v[187] * eq84_e1989_d_b12));
        let eq84_e1991_d_b13: f64 = ((s.db[187][13] * eq84_e1989) + (s.v[187] * eq84_e1989_d_b13));
        let eq84_e1991_q: f64 = (s.v[187] * eq84_e1990_q);
        let eq84_e1991_q_d_n0: f64 = ((s.dn[187][0] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n0));
        let eq84_e1991_q_d_n1: f64 = ((s.dn[187][1] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n1));
        let eq84_e1991_q_d_n2: f64 = ((s.dn[187][2] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n2));
        let eq84_e1991_q_d_n3: f64 = ((s.dn[187][3] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n3));
        let eq84_e1991_q_d_n4: f64 = ((s.dn[187][4] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n4));
        let eq84_e1991_q_d_n5: f64 = ((s.dn[187][5] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n5));
        let eq84_e1991_q_d_n6: f64 = ((s.dn[187][6] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n6));
        let eq84_e1991_q_d_n7: f64 = ((s.dn[187][7] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n7));
        let eq84_e1991_q_d_n8: f64 = ((s.dn[187][8] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n8));
        let eq84_e1991_q_d_n9: f64 = ((s.dn[187][9] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n9));
        let eq84_e1991_q_d_n10: f64 = ((s.dn[187][10] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n10));
        let eq84_e1991_q_d_n11: f64 = ((s.dn[187][11] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n11));
        let eq84_e1991_q_d_n12: f64 = ((s.dn[187][12] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n12));
        let eq84_e1991_q_d_n13: f64 = ((s.dn[187][13] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n13));
        let eq84_e1991_q_d_n14: f64 = ((s.dn[187][14] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n14));
        let eq84_e1991_q_d_n15: f64 = ((s.dn[187][15] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n15));
        let eq84_e1991_q_d_n16: f64 = ((s.dn[187][16] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n16));
        let eq84_e1991_q_d_b0: f64 = ((s.db[187][0] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_b0));
        let eq84_e1991_q_d_b1: f64 = ((s.db[187][1] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_b1));
        let eq84_e1991_q_d_b2: f64 = ((s.db[187][2] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_b2));
        let eq84_e1991_q_d_b3: f64 = ((s.db[187][3] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_b3));
        let eq84_e1991_q_d_b4: f64 = ((s.db[187][4] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_b4));
        let eq84_e1991_q_d_b5: f64 = ((s.db[187][5] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_b5));
        let eq84_e1991_q_d_b6: f64 = ((s.db[187][6] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_b6));
        let eq84_e1991_q_d_b7: f64 = ((s.db[187][7] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_b7));
        let eq84_e1991_q_d_b8: f64 = ((s.db[187][8] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_b8));
        let eq84_e1991_q_d_b9: f64 = ((s.db[187][9] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_b9));
        let eq84_e1991_q_d_b10: f64 = ((s.db[187][10] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_b10));
        let eq84_e1991_q_d_b11: f64 = ((s.db[187][11] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_b11));
        let eq84_e1991_q_d_b12: f64 = ((s.db[187][12] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_b12));
        let eq84_e1991_q_d_b13: f64 = ((s.db[187][13] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_b13));
        (eq84_e1991, eq84_e1991_d_n0, eq84_e1991_d_n1, eq84_e1991_d_n2, eq84_e1991_d_n3, eq84_e1991_d_n4, eq84_e1991_d_n5, eq84_e1991_d_n6, eq84_e1991_d_n7, eq84_e1991_d_n8, eq84_e1991_d_n9, eq84_e1991_d_n10, eq84_e1991_d_n11, eq84_e1991_d_n12, eq84_e1991_d_n13, eq84_e1991_d_n14, eq84_e1991_d_n15, eq84_e1991_d_n16, eq84_e1991_d_b0, eq84_e1991_d_b1, eq84_e1991_d_b2, eq84_e1991_d_b3, eq84_e1991_d_b4, eq84_e1991_d_b5, eq84_e1991_d_b6, eq84_e1991_d_b7, eq84_e1991_d_b8, eq84_e1991_d_b9, eq84_e1991_d_b10, eq84_e1991_d_b11, eq84_e1991_d_b12, eq84_e1991_d_b13, eq84_e1991_q, eq84_e1991_q_d_n0, eq84_e1991_q_d_n1, eq84_e1991_q_d_n2, eq84_e1991_q_d_n3, eq84_e1991_q_d_n4, eq84_e1991_q_d_n5, eq84_e1991_q_d_n6, eq84_e1991_q_d_n7, eq84_e1991_q_d_n8, eq84_e1991_q_d_n9, eq84_e1991_q_d_n10, eq84_e1991_q_d_n11, eq84_e1991_q_d_n12, eq84_e1991_q_d_n13, eq84_e1991_q_d_n14, eq84_e1991_q_d_n15, eq84_e1991_q_d_n16, eq84_e1991_q_d_b0, eq84_e1991_q_d_b1, eq84_e1991_q_d_b2, eq84_e1991_q_d_b3, eq84_e1991_q_d_b4, eq84_e1991_q_d_b5, eq84_e1991_q_d_b6, eq84_e1991_q_d_b7, eq84_e1991_q_d_b8, eq84_e1991_q_d_b9, eq84_e1991_q_d_b10, eq84_e1991_q_d_b11, eq84_e1991_q_d_b12, eq84_e1991_q_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq84_reactive_node_derivatives: [f64; 17] = [eq84_e1993_q_d_n0, eq84_e1993_q_d_n1, eq84_e1993_q_d_n2, eq84_e1993_q_d_n3, eq84_e1993_q_d_n4, eq84_e1993_q_d_n5, eq84_e1993_q_d_n6, eq84_e1993_q_d_n7, eq84_e1993_q_d_n8, eq84_e1993_q_d_n9, eq84_e1993_q_d_n10, eq84_e1993_q_d_n11, eq84_e1993_q_d_n12, eq84_e1993_q_d_n13, eq84_e1993_q_d_n14, eq84_e1993_q_d_n15, eq84_e1993_q_d_n16];
        let eq84_reactive_branch_derivatives: [f64; 14] = [eq84_e1993_q_d_b0, eq84_e1993_q_d_b1, eq84_e1993_q_d_b2, eq84_e1993_q_d_b3, eq84_e1993_q_d_b4, eq84_e1993_q_d_b5, eq84_e1993_q_d_b6, eq84_e1993_q_d_b7, eq84_e1993_q_d_b8, eq84_e1993_q_d_b9, eq84_e1993_q_d_b10, eq84_e1993_q_d_b11, eq84_e1993_q_d_b12, eq84_e1993_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[14]),
            nodes,
            &eq84_reactive_node_derivatives,
            branches,
            &eq84_reactive_branch_derivatives,
            multiplicity,
        );
    }
}

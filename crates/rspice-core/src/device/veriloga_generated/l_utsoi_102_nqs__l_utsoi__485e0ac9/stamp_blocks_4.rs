#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

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
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let eq32_e683: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, s.v[362]);
        let eq32_e685: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, s.v[377]);
        let eq32_e686: f64 = (eq32_e683 + eq32_e685);
        let eq32_e686_d_n0: f64 = ((s.dn[362][0] * ddt_scale) + (s.dn[377][0] * ddt_scale));
        let eq32_e686_d_n1: f64 = ((s.dn[362][1] * ddt_scale) + (s.dn[377][1] * ddt_scale));
        let eq32_e686_d_n2: f64 = ((s.dn[362][2] * ddt_scale) + (s.dn[377][2] * ddt_scale));
        let eq32_e686_d_n3: f64 = ((s.dn[362][3] * ddt_scale) + (s.dn[377][3] * ddt_scale));
        let eq32_e686_d_n4: f64 = ((s.dn[362][4] * ddt_scale) + (s.dn[377][4] * ddt_scale));
        let eq32_e686_d_n5: f64 = ((s.dn[362][5] * ddt_scale) + (s.dn[377][5] * ddt_scale));
        let eq32_e686_d_n6: f64 = ((s.dn[362][6] * ddt_scale) + (s.dn[377][6] * ddt_scale));
        let eq32_e686_d_n7: f64 = ((s.dn[362][7] * ddt_scale) + (s.dn[377][7] * ddt_scale));
        let eq32_e686_d_n8: f64 = ((s.dn[362][8] * ddt_scale) + (s.dn[377][8] * ddt_scale));
        let eq32_e686_d_n9: f64 = ((s.dn[362][9] * ddt_scale) + (s.dn[377][9] * ddt_scale));
        let eq32_e686_d_n10: f64 = ((s.dn[362][10] * ddt_scale) + (s.dn[377][10] * ddt_scale));
        let eq32_e686_d_n11: f64 = ((s.dn[362][11] * ddt_scale) + (s.dn[377][11] * ddt_scale));
        let eq32_e686_d_n12: f64 = ((s.dn[362][12] * ddt_scale) + (s.dn[377][12] * ddt_scale));
        let eq32_e686_d_n13: f64 = ((s.dn[362][13] * ddt_scale) + (s.dn[377][13] * ddt_scale));
        let eq32_e686_d_b0: f64 = ((s.db[362][0] * ddt_scale) + (s.db[377][0] * ddt_scale));
        let eq32_e686_d_b1: f64 = ((s.db[362][1] * ddt_scale) + (s.db[377][1] * ddt_scale));
        let eq32_e686_d_b2: f64 = ((s.db[362][2] * ddt_scale) + (s.db[377][2] * ddt_scale));
        let eq32_e686_d_b3: f64 = ((s.db[362][3] * ddt_scale) + (s.db[377][3] * ddt_scale));
        let eq32_e688: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, s.v[381]);
        let eq32_e689: f64 = (eq32_e686 + eq32_e688);
        let eq32_e689_d_n0: f64 = (eq32_e686_d_n0 + (s.dn[381][0] * ddt_scale));
        let eq32_e689_d_n1: f64 = (eq32_e686_d_n1 + (s.dn[381][1] * ddt_scale));
        let eq32_e689_d_n2: f64 = (eq32_e686_d_n2 + (s.dn[381][2] * ddt_scale));
        let eq32_e689_d_n3: f64 = (eq32_e686_d_n3 + (s.dn[381][3] * ddt_scale));
        let eq32_e689_d_n4: f64 = (eq32_e686_d_n4 + (s.dn[381][4] * ddt_scale));
        let eq32_e689_d_n5: f64 = (eq32_e686_d_n5 + (s.dn[381][5] * ddt_scale));
        let eq32_e689_d_n6: f64 = (eq32_e686_d_n6 + (s.dn[381][6] * ddt_scale));
        let eq32_e689_d_n7: f64 = (eq32_e686_d_n7 + (s.dn[381][7] * ddt_scale));
        let eq32_e689_d_n8: f64 = (eq32_e686_d_n8 + (s.dn[381][8] * ddt_scale));
        let eq32_e689_d_n9: f64 = (eq32_e686_d_n9 + (s.dn[381][9] * ddt_scale));
        let eq32_e689_d_n10: f64 = (eq32_e686_d_n10 + (s.dn[381][10] * ddt_scale));
        let eq32_e689_d_n11: f64 = (eq32_e686_d_n11 + (s.dn[381][11] * ddt_scale));
        let eq32_e689_d_n12: f64 = (eq32_e686_d_n12 + (s.dn[381][12] * ddt_scale));
        let eq32_e689_d_n13: f64 = (eq32_e686_d_n13 + (s.dn[381][13] * ddt_scale));
        let eq32_e689_d_b0: f64 = (eq32_e686_d_b0 + (s.db[381][0] * ddt_scale));
        let eq32_e689_d_b1: f64 = (eq32_e686_d_b1 + (s.db[381][1] * ddt_scale));
        let eq32_e689_d_b2: f64 = (eq32_e686_d_b2 + (s.db[381][2] * ddt_scale));
        let eq32_e689_d_b3: f64 = (eq32_e686_d_b3 + (s.db[381][3] * ddt_scale));
        let eq32_e690: f64 = (p.p14 * eq32_e689);
        let eq32_e690_d_n0: f64 = (p.p14 * eq32_e689_d_n0);
        let eq32_e690_d_n1: f64 = (p.p14 * eq32_e689_d_n1);
        let eq32_e690_d_n2: f64 = (p.p14 * eq32_e689_d_n2);
        let eq32_e690_d_n3: f64 = (p.p14 * eq32_e689_d_n3);
        let eq32_e690_d_n4: f64 = (p.p14 * eq32_e689_d_n4);
        let eq32_e690_d_n5: f64 = (p.p14 * eq32_e689_d_n5);
        let eq32_e690_d_n6: f64 = (p.p14 * eq32_e689_d_n6);
        let eq32_e690_d_n7: f64 = (p.p14 * eq32_e689_d_n7);
        let eq32_e690_d_n8: f64 = (p.p14 * eq32_e689_d_n8);
        let eq32_e690_d_n9: f64 = (p.p14 * eq32_e689_d_n9);
        let eq32_e690_d_n10: f64 = (p.p14 * eq32_e689_d_n10);
        let eq32_e690_d_n11: f64 = (p.p14 * eq32_e689_d_n11);
        let eq32_e690_d_n12: f64 = (p.p14 * eq32_e689_d_n12);
        let eq32_e690_d_n13: f64 = (p.p14 * eq32_e689_d_n13);
        let eq32_e690_d_b0: f64 = (p.p14 * eq32_e689_d_b0);
        let eq32_e690_d_b1: f64 = (p.p14 * eq32_e689_d_b1);
        let eq32_e690_d_b2: f64 = (p.p14 * eq32_e689_d_b2);
        let eq32_e690_d_b3: f64 = (p.p14 * eq32_e689_d_b3);
        let eq32_value: f64 = eq32_e690;
        let eq32_node_derivatives: [f64; 14] = [eq32_e690_d_n0, eq32_e690_d_n1, eq32_e690_d_n2, eq32_e690_d_n3, eq32_e690_d_n4, eq32_e690_d_n5, eq32_e690_d_n6, eq32_e690_d_n7, eq32_e690_d_n8, eq32_e690_d_n9, eq32_e690_d_n10, eq32_e690_d_n11, eq32_e690_d_n12, eq32_e690_d_n13];
        let eq32_branch_derivatives: [f64; 4] = [eq32_e690_d_b0, eq32_e690_d_b1, eq32_e690_d_b2, eq32_e690_d_b3];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );
        let eq33_e693: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, s.v[371]);
        let eq33_e695: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, s.v[373]);
        let eq33_e696: f64 = (eq33_e693 + eq33_e695);
        let eq33_e696_d_n0: f64 = ((s.dn[371][0] * ddt_scale) + (s.dn[373][0] * ddt_scale));
        let eq33_e696_d_n1: f64 = ((s.dn[371][1] * ddt_scale) + (s.dn[373][1] * ddt_scale));
        let eq33_e696_d_n2: f64 = ((s.dn[371][2] * ddt_scale) + (s.dn[373][2] * ddt_scale));
        let eq33_e696_d_n3: f64 = ((s.dn[371][3] * ddt_scale) + (s.dn[373][3] * ddt_scale));
        let eq33_e696_d_n4: f64 = ((s.dn[371][4] * ddt_scale) + (s.dn[373][4] * ddt_scale));
        let eq33_e696_d_n5: f64 = ((s.dn[371][5] * ddt_scale) + (s.dn[373][5] * ddt_scale));
        let eq33_e696_d_n6: f64 = ((s.dn[371][6] * ddt_scale) + (s.dn[373][6] * ddt_scale));
        let eq33_e696_d_n7: f64 = ((s.dn[371][7] * ddt_scale) + (s.dn[373][7] * ddt_scale));
        let eq33_e696_d_n8: f64 = ((s.dn[371][8] * ddt_scale) + (s.dn[373][8] * ddt_scale));
        let eq33_e696_d_n9: f64 = ((s.dn[371][9] * ddt_scale) + (s.dn[373][9] * ddt_scale));
        let eq33_e696_d_n10: f64 = ((s.dn[371][10] * ddt_scale) + (s.dn[373][10] * ddt_scale));
        let eq33_e696_d_n11: f64 = ((s.dn[371][11] * ddt_scale) + (s.dn[373][11] * ddt_scale));
        let eq33_e696_d_n12: f64 = ((s.dn[371][12] * ddt_scale) + (s.dn[373][12] * ddt_scale));
        let eq33_e696_d_n13: f64 = ((s.dn[371][13] * ddt_scale) + (s.dn[373][13] * ddt_scale));
        let eq33_e696_d_b0: f64 = ((s.db[371][0] * ddt_scale) + (s.db[373][0] * ddt_scale));
        let eq33_e696_d_b1: f64 = ((s.db[371][1] * ddt_scale) + (s.db[373][1] * ddt_scale));
        let eq33_e696_d_b2: f64 = ((s.db[371][2] * ddt_scale) + (s.db[373][2] * ddt_scale));
        let eq33_e696_d_b3: f64 = ((s.db[371][3] * ddt_scale) + (s.db[373][3] * ddt_scale));
        let eq33_e698: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 11, s.v[380]);
        let eq33_e699: f64 = (eq33_e696 + eq33_e698);
        let eq33_e699_d_n0: f64 = (eq33_e696_d_n0 + (s.dn[380][0] * ddt_scale));
        let eq33_e699_d_n1: f64 = (eq33_e696_d_n1 + (s.dn[380][1] * ddt_scale));
        let eq33_e699_d_n2: f64 = (eq33_e696_d_n2 + (s.dn[380][2] * ddt_scale));
        let eq33_e699_d_n3: f64 = (eq33_e696_d_n3 + (s.dn[380][3] * ddt_scale));
        let eq33_e699_d_n4: f64 = (eq33_e696_d_n4 + (s.dn[380][4] * ddt_scale));
        let eq33_e699_d_n5: f64 = (eq33_e696_d_n5 + (s.dn[380][5] * ddt_scale));
        let eq33_e699_d_n6: f64 = (eq33_e696_d_n6 + (s.dn[380][6] * ddt_scale));
        let eq33_e699_d_n7: f64 = (eq33_e696_d_n7 + (s.dn[380][7] * ddt_scale));
        let eq33_e699_d_n8: f64 = (eq33_e696_d_n8 + (s.dn[380][8] * ddt_scale));
        let eq33_e699_d_n9: f64 = (eq33_e696_d_n9 + (s.dn[380][9] * ddt_scale));
        let eq33_e699_d_n10: f64 = (eq33_e696_d_n10 + (s.dn[380][10] * ddt_scale));
        let eq33_e699_d_n11: f64 = (eq33_e696_d_n11 + (s.dn[380][11] * ddt_scale));
        let eq33_e699_d_n12: f64 = (eq33_e696_d_n12 + (s.dn[380][12] * ddt_scale));
        let eq33_e699_d_n13: f64 = (eq33_e696_d_n13 + (s.dn[380][13] * ddt_scale));
        let eq33_e699_d_b0: f64 = (eq33_e696_d_b0 + (s.db[380][0] * ddt_scale));
        let eq33_e699_d_b1: f64 = (eq33_e696_d_b1 + (s.db[380][1] * ddt_scale));
        let eq33_e699_d_b2: f64 = (eq33_e696_d_b2 + (s.db[380][2] * ddt_scale));
        let eq33_e699_d_b3: f64 = (eq33_e696_d_b3 + (s.db[380][3] * ddt_scale));
        let eq33_e700: f64 = (p.p14 * eq33_e699);
        let eq33_e700_d_n0: f64 = (p.p14 * eq33_e699_d_n0);
        let eq33_e700_d_n1: f64 = (p.p14 * eq33_e699_d_n1);
        let eq33_e700_d_n2: f64 = (p.p14 * eq33_e699_d_n2);
        let eq33_e700_d_n3: f64 = (p.p14 * eq33_e699_d_n3);
        let eq33_e700_d_n4: f64 = (p.p14 * eq33_e699_d_n4);
        let eq33_e700_d_n5: f64 = (p.p14 * eq33_e699_d_n5);
        let eq33_e700_d_n6: f64 = (p.p14 * eq33_e699_d_n6);
        let eq33_e700_d_n7: f64 = (p.p14 * eq33_e699_d_n7);
        let eq33_e700_d_n8: f64 = (p.p14 * eq33_e699_d_n8);
        let eq33_e700_d_n9: f64 = (p.p14 * eq33_e699_d_n9);
        let eq33_e700_d_n10: f64 = (p.p14 * eq33_e699_d_n10);
        let eq33_e700_d_n11: f64 = (p.p14 * eq33_e699_d_n11);
        let eq33_e700_d_n12: f64 = (p.p14 * eq33_e699_d_n12);
        let eq33_e700_d_n13: f64 = (p.p14 * eq33_e699_d_n13);
        let eq33_e700_d_b0: f64 = (p.p14 * eq33_e699_d_b0);
        let eq33_e700_d_b1: f64 = (p.p14 * eq33_e699_d_b1);
        let eq33_e700_d_b2: f64 = (p.p14 * eq33_e699_d_b2);
        let eq33_e700_d_b3: f64 = (p.p14 * eq33_e699_d_b3);
        let eq33_value: f64 = eq33_e700;
        let eq33_node_derivatives: [f64; 14] = [eq33_e700_d_n0, eq33_e700_d_n1, eq33_e700_d_n2, eq33_e700_d_n3, eq33_e700_d_n4, eq33_e700_d_n5, eq33_e700_d_n6, eq33_e700_d_n7, eq33_e700_d_n8, eq33_e700_d_n9, eq33_e700_d_n10, eq33_e700_d_n11, eq33_e700_d_n12, eq33_e700_d_n13];
        let eq33_branch_derivatives: [f64; 4] = [eq33_e700_d_b0, eq33_e700_d_b1, eq33_e700_d_b2, eq33_e700_d_b3];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let eq34_e703: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 12, s.v[376]);
        let eq34_e705: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 13, s.v[382]);
        let eq34_e706: f64 = (eq34_e703 + eq34_e705);
        let eq34_e706_d_n0: f64 = ((s.dn[376][0] * ddt_scale) + (s.dn[382][0] * ddt_scale));
        let eq34_e706_d_n1: f64 = ((s.dn[376][1] * ddt_scale) + (s.dn[382][1] * ddt_scale));
        let eq34_e706_d_n2: f64 = ((s.dn[376][2] * ddt_scale) + (s.dn[382][2] * ddt_scale));
        let eq34_e706_d_n3: f64 = ((s.dn[376][3] * ddt_scale) + (s.dn[382][3] * ddt_scale));
        let eq34_e706_d_n4: f64 = ((s.dn[376][4] * ddt_scale) + (s.dn[382][4] * ddt_scale));
        let eq34_e706_d_n5: f64 = ((s.dn[376][5] * ddt_scale) + (s.dn[382][5] * ddt_scale));
        let eq34_e706_d_n6: f64 = ((s.dn[376][6] * ddt_scale) + (s.dn[382][6] * ddt_scale));
        let eq34_e706_d_n7: f64 = ((s.dn[376][7] * ddt_scale) + (s.dn[382][7] * ddt_scale));
        let eq34_e706_d_n8: f64 = ((s.dn[376][8] * ddt_scale) + (s.dn[382][8] * ddt_scale));
        let eq34_e706_d_n9: f64 = ((s.dn[376][9] * ddt_scale) + (s.dn[382][9] * ddt_scale));
        let eq34_e706_d_n10: f64 = ((s.dn[376][10] * ddt_scale) + (s.dn[382][10] * ddt_scale));
        let eq34_e706_d_n11: f64 = ((s.dn[376][11] * ddt_scale) + (s.dn[382][11] * ddt_scale));
        let eq34_e706_d_n12: f64 = ((s.dn[376][12] * ddt_scale) + (s.dn[382][12] * ddt_scale));
        let eq34_e706_d_n13: f64 = ((s.dn[376][13] * ddt_scale) + (s.dn[382][13] * ddt_scale));
        let eq34_e706_d_b0: f64 = ((s.db[376][0] * ddt_scale) + (s.db[382][0] * ddt_scale));
        let eq34_e706_d_b1: f64 = ((s.db[376][1] * ddt_scale) + (s.db[382][1] * ddt_scale));
        let eq34_e706_d_b2: f64 = ((s.db[376][2] * ddt_scale) + (s.db[382][2] * ddt_scale));
        let eq34_e706_d_b3: f64 = ((s.db[376][3] * ddt_scale) + (s.db[382][3] * ddt_scale));
        let eq34_e707: f64 = (p.p14 * eq34_e706);
        let eq34_e707_d_n0: f64 = (p.p14 * eq34_e706_d_n0);
        let eq34_e707_d_n1: f64 = (p.p14 * eq34_e706_d_n1);
        let eq34_e707_d_n2: f64 = (p.p14 * eq34_e706_d_n2);
        let eq34_e707_d_n3: f64 = (p.p14 * eq34_e706_d_n3);
        let eq34_e707_d_n4: f64 = (p.p14 * eq34_e706_d_n4);
        let eq34_e707_d_n5: f64 = (p.p14 * eq34_e706_d_n5);
        let eq34_e707_d_n6: f64 = (p.p14 * eq34_e706_d_n6);
        let eq34_e707_d_n7: f64 = (p.p14 * eq34_e706_d_n7);
        let eq34_e707_d_n8: f64 = (p.p14 * eq34_e706_d_n8);
        let eq34_e707_d_n9: f64 = (p.p14 * eq34_e706_d_n9);
        let eq34_e707_d_n10: f64 = (p.p14 * eq34_e706_d_n10);
        let eq34_e707_d_n11: f64 = (p.p14 * eq34_e706_d_n11);
        let eq34_e707_d_n12: f64 = (p.p14 * eq34_e706_d_n12);
        let eq34_e707_d_n13: f64 = (p.p14 * eq34_e706_d_n13);
        let eq34_e707_d_b0: f64 = (p.p14 * eq34_e706_d_b0);
        let eq34_e707_d_b1: f64 = (p.p14 * eq34_e706_d_b1);
        let eq34_e707_d_b2: f64 = (p.p14 * eq34_e706_d_b2);
        let eq34_e707_d_b3: f64 = (p.p14 * eq34_e706_d_b3);
        let eq34_value: f64 = eq34_e707;
        let eq34_node_derivatives: [f64; 14] = [eq34_e707_d_n0, eq34_e707_d_n1, eq34_e707_d_n2, eq34_e707_d_n3, eq34_e707_d_n4, eq34_e707_d_n5, eq34_e707_d_n6, eq34_e707_d_n7, eq34_e707_d_n8, eq34_e707_d_n9, eq34_e707_d_n10, eq34_e707_d_n11, eq34_e707_d_n12, eq34_e707_d_n13];
        let eq34_branch_derivatives: [f64; 4] = [eq34_e707_d_b0, eq34_e707_d_b1, eq34_e707_d_b2, eq34_e707_d_b3];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
            multiplicity,
        );
        let eq35_e710: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 14, s.v[374]);
        let eq35_e711: f64 = (p.p14 * eq35_e710);
        let eq35_e711_d_n0: f64 = (p.p14 * (s.dn[374][0] * ddt_scale));
        let eq35_e711_d_n1: f64 = (p.p14 * (s.dn[374][1] * ddt_scale));
        let eq35_e711_d_n2: f64 = (p.p14 * (s.dn[374][2] * ddt_scale));
        let eq35_e711_d_n3: f64 = (p.p14 * (s.dn[374][3] * ddt_scale));
        let eq35_e711_d_n4: f64 = (p.p14 * (s.dn[374][4] * ddt_scale));
        let eq35_e711_d_n5: f64 = (p.p14 * (s.dn[374][5] * ddt_scale));
        let eq35_e711_d_n6: f64 = (p.p14 * (s.dn[374][6] * ddt_scale));
        let eq35_e711_d_n7: f64 = (p.p14 * (s.dn[374][7] * ddt_scale));
        let eq35_e711_d_n8: f64 = (p.p14 * (s.dn[374][8] * ddt_scale));
        let eq35_e711_d_n9: f64 = (p.p14 * (s.dn[374][9] * ddt_scale));
        let eq35_e711_d_n10: f64 = (p.p14 * (s.dn[374][10] * ddt_scale));
        let eq35_e711_d_n11: f64 = (p.p14 * (s.dn[374][11] * ddt_scale));
        let eq35_e711_d_n12: f64 = (p.p14 * (s.dn[374][12] * ddt_scale));
        let eq35_e711_d_n13: f64 = (p.p14 * (s.dn[374][13] * ddt_scale));
        let eq35_e711_d_b0: f64 = (p.p14 * (s.db[374][0] * ddt_scale));
        let eq35_e711_d_b1: f64 = (p.p14 * (s.db[374][1] * ddt_scale));
        let eq35_e711_d_b2: f64 = (p.p14 * (s.db[374][2] * ddt_scale));
        let eq35_e711_d_b3: f64 = (p.p14 * (s.db[374][3] * ddt_scale));
        let eq35_value: f64 = eq35_e711;
        let eq35_node_derivatives: [f64; 14] = [eq35_e711_d_n0, eq35_e711_d_n1, eq35_e711_d_n2, eq35_e711_d_n3, eq35_e711_d_n4, eq35_e711_d_n5, eq35_e711_d_n6, eq35_e711_d_n7, eq35_e711_d_n8, eq35_e711_d_n9, eq35_e711_d_n10, eq35_e711_d_n11, eq35_e711_d_n12, eq35_e711_d_n13];
        let eq35_branch_derivatives: [f64; 4] = [eq35_e711_d_b0, eq35_e711_d_b1, eq35_e711_d_b2, eq35_e711_d_b3];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq35_value),
            &eq35_node_derivatives,
            &eq35_branch_derivatives,
            multiplicity,
        );
        let eq36_e714: f64 = (-s.v[1773]);
        let eq36_e716: f64 = (eq36_e714 * p.p32);
        let eq36_e716_d_n0: f64 = ((-s.dn[1773][0]) * p.p32);
        let eq36_e716_d_n1: f64 = ((-s.dn[1773][1]) * p.p32);
        let eq36_e716_d_n2: f64 = ((-s.dn[1773][2]) * p.p32);
        let eq36_e716_d_n3: f64 = ((-s.dn[1773][3]) * p.p32);
        let eq36_e716_d_n4: f64 = ((-s.dn[1773][4]) * p.p32);
        let eq36_e716_d_n5: f64 = ((-s.dn[1773][5]) * p.p32);
        let eq36_e716_d_n6: f64 = ((-s.dn[1773][6]) * p.p32);
        let eq36_e716_d_n7: f64 = ((-s.dn[1773][7]) * p.p32);
        let eq36_e716_d_n8: f64 = ((-s.dn[1773][8]) * p.p32);
        let eq36_e716_d_n9: f64 = ((-s.dn[1773][9]) * p.p32);
        let eq36_e716_d_n10: f64 = ((-s.dn[1773][10]) * p.p32);
        let eq36_e716_d_n11: f64 = ((-s.dn[1773][11]) * p.p32);
        let eq36_e716_d_n12: f64 = ((-s.dn[1773][12]) * p.p32);
        let eq36_e716_d_n13: f64 = ((-s.dn[1773][13]) * p.p32);
        let eq36_e716_d_b0: f64 = ((-s.db[1773][0]) * p.p32);
        let eq36_e716_d_b1: f64 = ((-s.db[1773][1]) * p.p32);
        let eq36_e716_d_b2: f64 = ((-s.db[1773][2]) * p.p32);
        let eq36_e716_d_b3: f64 = ((-s.db[1773][3]) * p.p32);
        let eq36_e718: f64 = (eq36_e716 * s.v[13]);
        let eq36_e718_d_n0: f64 = ((eq36_e716_d_n0 * s.v[13]) + (eq36_e716 * s.dn[13][0]));
        let eq36_e718_d_n1: f64 = ((eq36_e716_d_n1 * s.v[13]) + (eq36_e716 * s.dn[13][1]));
        let eq36_e718_d_n2: f64 = ((eq36_e716_d_n2 * s.v[13]) + (eq36_e716 * s.dn[13][2]));
        let eq36_e718_d_n3: f64 = ((eq36_e716_d_n3 * s.v[13]) + (eq36_e716 * s.dn[13][3]));
        let eq36_e718_d_n4: f64 = ((eq36_e716_d_n4 * s.v[13]) + (eq36_e716 * s.dn[13][4]));
        let eq36_e718_d_n5: f64 = ((eq36_e716_d_n5 * s.v[13]) + (eq36_e716 * s.dn[13][5]));
        let eq36_e718_d_n6: f64 = ((eq36_e716_d_n6 * s.v[13]) + (eq36_e716 * s.dn[13][6]));
        let eq36_e718_d_n7: f64 = ((eq36_e716_d_n7 * s.v[13]) + (eq36_e716 * s.dn[13][7]));
        let eq36_e718_d_n8: f64 = ((eq36_e716_d_n8 * s.v[13]) + (eq36_e716 * s.dn[13][8]));
        let eq36_e718_d_n9: f64 = ((eq36_e716_d_n9 * s.v[13]) + (eq36_e716 * s.dn[13][9]));
        let eq36_e718_d_n10: f64 = ((eq36_e716_d_n10 * s.v[13]) + (eq36_e716 * s.dn[13][10]));
        let eq36_e718_d_n11: f64 = ((eq36_e716_d_n11 * s.v[13]) + (eq36_e716 * s.dn[13][11]));
        let eq36_e718_d_n12: f64 = ((eq36_e716_d_n12 * s.v[13]) + (eq36_e716 * s.dn[13][12]));
        let eq36_e718_d_n13: f64 = ((eq36_e716_d_n13 * s.v[13]) + (eq36_e716 * s.dn[13][13]));
        let eq36_e718_d_b0: f64 = ((eq36_e716_d_b0 * s.v[13]) + (eq36_e716 * s.db[13][0]));
        let eq36_e718_d_b1: f64 = ((eq36_e716_d_b1 * s.v[13]) + (eq36_e716 * s.db[13][1]));
        let eq36_e718_d_b2: f64 = ((eq36_e716_d_b2 * s.v[13]) + (eq36_e716 * s.db[13][2]));
        let eq36_e718_d_b3: f64 = ((eq36_e716_d_b3 * s.v[13]) + (eq36_e716 * s.db[13][3]));
        let eq36_e722: f64 = (s.v[182]).sqrt();
        let eq36_e722_d_n0: f64 = (s.dn[182][0] / (2.0 * eq36_e722));
        let eq36_e722_d_n1: f64 = (s.dn[182][1] / (2.0 * eq36_e722));
        let eq36_e722_d_n2: f64 = (s.dn[182][2] / (2.0 * eq36_e722));
        let eq36_e722_d_n3: f64 = (s.dn[182][3] / (2.0 * eq36_e722));
        let eq36_e722_d_n4: f64 = (s.dn[182][4] / (2.0 * eq36_e722));
        let eq36_e722_d_n5: f64 = (s.dn[182][5] / (2.0 * eq36_e722));
        let eq36_e722_d_n6: f64 = (s.dn[182][6] / (2.0 * eq36_e722));
        let eq36_e722_d_n7: f64 = (s.dn[182][7] / (2.0 * eq36_e722));
        let eq36_e722_d_n8: f64 = (s.dn[182][8] / (2.0 * eq36_e722));
        let eq36_e722_d_n9: f64 = (s.dn[182][9] / (2.0 * eq36_e722));
        let eq36_e722_d_n10: f64 = (s.dn[182][10] / (2.0 * eq36_e722));
        let eq36_e722_d_n11: f64 = (s.dn[182][11] / (2.0 * eq36_e722));
        let eq36_e722_d_n12: f64 = (s.dn[182][12] / (2.0 * eq36_e722));
        let eq36_e722_d_n13: f64 = (s.dn[182][13] / (2.0 * eq36_e722));
        let eq36_e722_d_b0: f64 = (s.db[182][0] / (2.0 * eq36_e722));
        let eq36_e722_d_b1: f64 = (s.db[182][1] / (2.0 * eq36_e722));
        let eq36_e722_d_b2: f64 = (s.db[182][2] / (2.0 * eq36_e722));
        let eq36_e722_d_b3: f64 = (s.db[182][3] / (2.0 * eq36_e722));
        let eq36_e723: f64 = ((nv11 - nv13) / eq36_e722);
        let eq36_e723_d_n0: f64 = (-(((nv11 - nv13) * eq36_e722_d_n0) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n1: f64 = (-(((nv11 - nv13) * eq36_e722_d_n1) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n2: f64 = (-(((nv11 - nv13) * eq36_e722_d_n2) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n3: f64 = (-(((nv11 - nv13) * eq36_e722_d_n3) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n4: f64 = (-(((nv11 - nv13) * eq36_e722_d_n4) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n5: f64 = (-(((nv11 - nv13) * eq36_e722_d_n5) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n6: f64 = (-(((nv11 - nv13) * eq36_e722_d_n6) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n7: f64 = (-(((nv11 - nv13) * eq36_e722_d_n7) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n8: f64 = (-(((nv11 - nv13) * eq36_e722_d_n8) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n9: f64 = (-(((nv11 - nv13) * eq36_e722_d_n9) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n10: f64 = (-(((nv11 - nv13) * eq36_e722_d_n10) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n11: f64 = ((eq36_e722 - ((nv11 - nv13) * eq36_e722_d_n11)) / (eq36_e722 * eq36_e722));
        let eq36_e723_d_n12: f64 = (-(((nv11 - nv13) * eq36_e722_d_n12) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n13: f64 = (((-eq36_e722) - ((nv11 - nv13) * eq36_e722_d_n13)) / (eq36_e722 * eq36_e722));
        let eq36_e723_d_b0: f64 = (-(((nv11 - nv13) * eq36_e722_d_b0) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_b1: f64 = (-(((nv11 - nv13) * eq36_e722_d_b1) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_b2: f64 = (-(((nv11 - nv13) * eq36_e722_d_b2) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_b3: f64 = (-(((nv11 - nv13) * eq36_e722_d_b3) / (eq36_e722 * eq36_e722)));
        let eq36_e724: f64 = ((nv10 - nv13) + eq36_e723);
        let eq36_e724_d_n10: f64 = (1.0 + eq36_e723_d_n10);
        let eq36_e724_d_n13: f64 = (-1.0 + eq36_e723_d_n13);
        let eq36_e725: f64 = (eq36_e718 * eq36_e724);
        let eq36_e725_d_n0: f64 = ((eq36_e718_d_n0 * eq36_e724) + (eq36_e718 * eq36_e723_d_n0));
        let eq36_e725_d_n1: f64 = ((eq36_e718_d_n1 * eq36_e724) + (eq36_e718 * eq36_e723_d_n1));
        let eq36_e725_d_n2: f64 = ((eq36_e718_d_n2 * eq36_e724) + (eq36_e718 * eq36_e723_d_n2));
        let eq36_e725_d_n3: f64 = ((eq36_e718_d_n3 * eq36_e724) + (eq36_e718 * eq36_e723_d_n3));
        let eq36_e725_d_n4: f64 = ((eq36_e718_d_n4 * eq36_e724) + (eq36_e718 * eq36_e723_d_n4));
        let eq36_e725_d_n5: f64 = ((eq36_e718_d_n5 * eq36_e724) + (eq36_e718 * eq36_e723_d_n5));
        let eq36_e725_d_n6: f64 = ((eq36_e718_d_n6 * eq36_e724) + (eq36_e718 * eq36_e723_d_n6));
        let eq36_e725_d_n7: f64 = ((eq36_e718_d_n7 * eq36_e724) + (eq36_e718 * eq36_e723_d_n7));
        let eq36_e725_d_n8: f64 = ((eq36_e718_d_n8 * eq36_e724) + (eq36_e718 * eq36_e723_d_n8));
        let eq36_e725_d_n9: f64 = ((eq36_e718_d_n9 * eq36_e724) + (eq36_e718 * eq36_e723_d_n9));
        let eq36_e725_d_n10: f64 = ((eq36_e718_d_n10 * eq36_e724) + (eq36_e718 * eq36_e724_d_n10));
        let eq36_e725_d_n11: f64 = ((eq36_e718_d_n11 * eq36_e724) + (eq36_e718 * eq36_e723_d_n11));
        let eq36_e725_d_n12: f64 = ((eq36_e718_d_n12 * eq36_e724) + (eq36_e718 * eq36_e723_d_n12));
        let eq36_e725_d_n13: f64 = ((eq36_e718_d_n13 * eq36_e724) + (eq36_e718 * eq36_e724_d_n13));
        let eq36_e725_d_b0: f64 = ((eq36_e718_d_b0 * eq36_e724) + (eq36_e718 * eq36_e723_d_b0));
        let eq36_e725_d_b1: f64 = ((eq36_e718_d_b1 * eq36_e724) + (eq36_e718 * eq36_e723_d_b1));
        let eq36_e725_d_b2: f64 = ((eq36_e718_d_b2 * eq36_e724) + (eq36_e718 * eq36_e723_d_b2));
        let eq36_e725_d_b3: f64 = ((eq36_e718_d_b3 * eq36_e724) + (eq36_e718 * eq36_e723_d_b3));
        let eq36_e727: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 15, s.v[362]);
        let eq36_e728: f64 = (eq36_e725 - eq36_e727);
        let eq36_e728_d_n0: f64 = (eq36_e725_d_n0 - (s.dn[362][0] * ddt_scale));
        let eq36_e728_d_n1: f64 = (eq36_e725_d_n1 - (s.dn[362][1] * ddt_scale));
        let eq36_e728_d_n2: f64 = (eq36_e725_d_n2 - (s.dn[362][2] * ddt_scale));
        let eq36_e728_d_n3: f64 = (eq36_e725_d_n3 - (s.dn[362][3] * ddt_scale));
        let eq36_e728_d_n4: f64 = (eq36_e725_d_n4 - (s.dn[362][4] * ddt_scale));
        let eq36_e728_d_n5: f64 = (eq36_e725_d_n5 - (s.dn[362][5] * ddt_scale));
        let eq36_e728_d_n6: f64 = (eq36_e725_d_n6 - (s.dn[362][6] * ddt_scale));
        let eq36_e728_d_n7: f64 = (eq36_e725_d_n7 - (s.dn[362][7] * ddt_scale));
        let eq36_e728_d_n8: f64 = (eq36_e725_d_n8 - (s.dn[362][8] * ddt_scale));
        let eq36_e728_d_n9: f64 = (eq36_e725_d_n9 - (s.dn[362][9] * ddt_scale));
        let eq36_e728_d_n10: f64 = (eq36_e725_d_n10 - (s.dn[362][10] * ddt_scale));
        let eq36_e728_d_n11: f64 = (eq36_e725_d_n11 - (s.dn[362][11] * ddt_scale));
        let eq36_e728_d_n12: f64 = (eq36_e725_d_n12 - (s.dn[362][12] * ddt_scale));
        let eq36_e728_d_n13: f64 = (eq36_e725_d_n13 - (s.dn[362][13] * ddt_scale));
        let eq36_e728_d_b0: f64 = (eq36_e725_d_b0 - (s.db[362][0] * ddt_scale));
        let eq36_e728_d_b1: f64 = (eq36_e725_d_b1 - (s.db[362][1] * ddt_scale));
        let eq36_e728_d_b2: f64 = (eq36_e725_d_b2 - (s.db[362][2] * ddt_scale));
        let eq36_e728_d_b3: f64 = (eq36_e725_d_b3 - (s.db[362][3] * ddt_scale));
        let eq36_e730: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 16, s.v[370]);
        let eq36_e731: f64 = (eq36_e728 + eq36_e730);
        let eq36_e731_d_n0: f64 = (eq36_e728_d_n0 + (s.dn[370][0] * ddt_scale));
        let eq36_e731_d_n1: f64 = (eq36_e728_d_n1 + (s.dn[370][1] * ddt_scale));
        let eq36_e731_d_n2: f64 = (eq36_e728_d_n2 + (s.dn[370][2] * ddt_scale));
        let eq36_e731_d_n3: f64 = (eq36_e728_d_n3 + (s.dn[370][3] * ddt_scale));
        let eq36_e731_d_n4: f64 = (eq36_e728_d_n4 + (s.dn[370][4] * ddt_scale));
        let eq36_e731_d_n5: f64 = (eq36_e728_d_n5 + (s.dn[370][5] * ddt_scale));
        let eq36_e731_d_n6: f64 = (eq36_e728_d_n6 + (s.dn[370][6] * ddt_scale));
        let eq36_e731_d_n7: f64 = (eq36_e728_d_n7 + (s.dn[370][7] * ddt_scale));
        let eq36_e731_d_n8: f64 = (eq36_e728_d_n8 + (s.dn[370][8] * ddt_scale));
        let eq36_e731_d_n9: f64 = (eq36_e728_d_n9 + (s.dn[370][9] * ddt_scale));
        let eq36_e731_d_n10: f64 = (eq36_e728_d_n10 + (s.dn[370][10] * ddt_scale));
        let eq36_e731_d_n11: f64 = (eq36_e728_d_n11 + (s.dn[370][11] * ddt_scale));
        let eq36_e731_d_n12: f64 = (eq36_e728_d_n12 + (s.dn[370][12] * ddt_scale));
        let eq36_e731_d_n13: f64 = (eq36_e728_d_n13 + (s.dn[370][13] * ddt_scale));
        let eq36_e731_d_b0: f64 = (eq36_e728_d_b0 + (s.db[370][0] * ddt_scale));
        let eq36_e731_d_b1: f64 = (eq36_e728_d_b1 + (s.db[370][1] * ddt_scale));
        let eq36_e731_d_b2: f64 = (eq36_e728_d_b2 + (s.db[370][2] * ddt_scale));
        let eq36_e731_d_b3: f64 = (eq36_e728_d_b3 + (s.db[370][3] * ddt_scale));
        let eq36_e733: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 17, s.v[372]);
        let eq36_e734: f64 = (eq36_e731 + eq36_e733);
        let eq36_e734_d_n0: f64 = (eq36_e731_d_n0 + (s.dn[372][0] * ddt_scale));
        let eq36_e734_d_n1: f64 = (eq36_e731_d_n1 + (s.dn[372][1] * ddt_scale));
        let eq36_e734_d_n2: f64 = (eq36_e731_d_n2 + (s.dn[372][2] * ddt_scale));
        let eq36_e734_d_n3: f64 = (eq36_e731_d_n3 + (s.dn[372][3] * ddt_scale));
        let eq36_e734_d_n4: f64 = (eq36_e731_d_n4 + (s.dn[372][4] * ddt_scale));
        let eq36_e734_d_n5: f64 = (eq36_e731_d_n5 + (s.dn[372][5] * ddt_scale));
        let eq36_e734_d_n6: f64 = (eq36_e731_d_n6 + (s.dn[372][6] * ddt_scale));
        let eq36_e734_d_n7: f64 = (eq36_e731_d_n7 + (s.dn[372][7] * ddt_scale));
        let eq36_e734_d_n8: f64 = (eq36_e731_d_n8 + (s.dn[372][8] * ddt_scale));
        let eq36_e734_d_n9: f64 = (eq36_e731_d_n9 + (s.dn[372][9] * ddt_scale));
        let eq36_e734_d_n10: f64 = (eq36_e731_d_n10 + (s.dn[372][10] * ddt_scale));
        let eq36_e734_d_n11: f64 = (eq36_e731_d_n11 + (s.dn[372][11] * ddt_scale));
        let eq36_e734_d_n12: f64 = (eq36_e731_d_n12 + (s.dn[372][12] * ddt_scale));
        let eq36_e734_d_n13: f64 = (eq36_e731_d_n13 + (s.dn[372][13] * ddt_scale));
        let eq36_e734_d_b0: f64 = (eq36_e731_d_b0 + (s.db[372][0] * ddt_scale));
        let eq36_e734_d_b1: f64 = (eq36_e731_d_b1 + (s.db[372][1] * ddt_scale));
        let eq36_e734_d_b2: f64 = (eq36_e731_d_b2 + (s.db[372][2] * ddt_scale));
        let eq36_e734_d_b3: f64 = (eq36_e731_d_b3 + (s.db[372][3] * ddt_scale));
        let eq36_e736: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 18, s.v[379]);
        let eq36_e737: f64 = (eq36_e734 + eq36_e736);
        let eq36_e737_d_n0: f64 = (eq36_e734_d_n0 + (s.dn[379][0] * ddt_scale));
        let eq36_e737_d_n1: f64 = (eq36_e734_d_n1 + (s.dn[379][1] * ddt_scale));
        let eq36_e737_d_n2: f64 = (eq36_e734_d_n2 + (s.dn[379][2] * ddt_scale));
        let eq36_e737_d_n3: f64 = (eq36_e734_d_n3 + (s.dn[379][3] * ddt_scale));
        let eq36_e737_d_n4: f64 = (eq36_e734_d_n4 + (s.dn[379][4] * ddt_scale));
        let eq36_e737_d_n5: f64 = (eq36_e734_d_n5 + (s.dn[379][5] * ddt_scale));
        let eq36_e737_d_n6: f64 = (eq36_e734_d_n6 + (s.dn[379][6] * ddt_scale));
        let eq36_e737_d_n7: f64 = (eq36_e734_d_n7 + (s.dn[379][7] * ddt_scale));
        let eq36_e737_d_n8: f64 = (eq36_e734_d_n8 + (s.dn[379][8] * ddt_scale));
        let eq36_e737_d_n9: f64 = (eq36_e734_d_n9 + (s.dn[379][9] * ddt_scale));
        let eq36_e737_d_n10: f64 = (eq36_e734_d_n10 + (s.dn[379][10] * ddt_scale));
        let eq36_e737_d_n11: f64 = (eq36_e734_d_n11 + (s.dn[379][11] * ddt_scale));
        let eq36_e737_d_n12: f64 = (eq36_e734_d_n12 + (s.dn[379][12] * ddt_scale));
        let eq36_e737_d_n13: f64 = (eq36_e734_d_n13 + (s.dn[379][13] * ddt_scale));
        let eq36_e737_d_b0: f64 = (eq36_e734_d_b0 + (s.db[379][0] * ddt_scale));
        let eq36_e737_d_b1: f64 = (eq36_e734_d_b1 + (s.db[379][1] * ddt_scale));
        let eq36_e737_d_b2: f64 = (eq36_e734_d_b2 + (s.db[379][2] * ddt_scale));
        let eq36_e737_d_b3: f64 = (eq36_e734_d_b3 + (s.db[379][3] * ddt_scale));
        let eq36_e738: f64 = (p.p14 * eq36_e737);
        let eq36_e738_d_n0: f64 = (p.p14 * eq36_e737_d_n0);
        let eq36_e738_d_n1: f64 = (p.p14 * eq36_e737_d_n1);
        let eq36_e738_d_n2: f64 = (p.p14 * eq36_e737_d_n2);
        let eq36_e738_d_n3: f64 = (p.p14 * eq36_e737_d_n3);
        let eq36_e738_d_n4: f64 = (p.p14 * eq36_e737_d_n4);
        let eq36_e738_d_n5: f64 = (p.p14 * eq36_e737_d_n5);
        let eq36_e738_d_n6: f64 = (p.p14 * eq36_e737_d_n6);
        let eq36_e738_d_n7: f64 = (p.p14 * eq36_e737_d_n7);
        let eq36_e738_d_n8: f64 = (p.p14 * eq36_e737_d_n8);
        let eq36_e738_d_n9: f64 = (p.p14 * eq36_e737_d_n9);
        let eq36_e738_d_n10: f64 = (p.p14 * eq36_e737_d_n10);
        let eq36_e738_d_n11: f64 = (p.p14 * eq36_e737_d_n11);
        let eq36_e738_d_n12: f64 = (p.p14 * eq36_e737_d_n12);
        let eq36_e738_d_n13: f64 = (p.p14 * eq36_e737_d_n13);
        let eq36_e738_d_b0: f64 = (p.p14 * eq36_e737_d_b0);
        let eq36_e738_d_b1: f64 = (p.p14 * eq36_e737_d_b1);
        let eq36_e738_d_b2: f64 = (p.p14 * eq36_e737_d_b2);
        let eq36_e738_d_b3: f64 = (p.p14 * eq36_e737_d_b3);
        let eq36_value: f64 = eq36_e738;
        let eq36_node_derivatives: [f64; 14] = [eq36_e738_d_n0, eq36_e738_d_n1, eq36_e738_d_n2, eq36_e738_d_n3, eq36_e738_d_n4, eq36_e738_d_n5, eq36_e738_d_n6, eq36_e738_d_n7, eq36_e738_d_n8, eq36_e738_d_n9, eq36_e738_d_n10, eq36_e738_d_n11, eq36_e738_d_n12, eq36_e738_d_n13];
        let eq36_branch_derivatives: [f64; 4] = [eq36_e738_d_b0, eq36_e738_d_b1, eq36_e738_d_b2, eq36_e738_d_b3];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq36_value),
            &eq36_node_derivatives,
            &eq36_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let eq37_e741: f64 = (-s.v[1773]);
        let eq37_e743: f64 = (eq37_e741 * p.p31);
        let eq37_e743_d_n0: f64 = ((-s.dn[1773][0]) * p.p31);
        let eq37_e743_d_n1: f64 = ((-s.dn[1773][1]) * p.p31);
        let eq37_e743_d_n2: f64 = ((-s.dn[1773][2]) * p.p31);
        let eq37_e743_d_n3: f64 = ((-s.dn[1773][3]) * p.p31);
        let eq37_e743_d_n4: f64 = ((-s.dn[1773][4]) * p.p31);
        let eq37_e743_d_n5: f64 = ((-s.dn[1773][5]) * p.p31);
        let eq37_e743_d_n6: f64 = ((-s.dn[1773][6]) * p.p31);
        let eq37_e743_d_n7: f64 = ((-s.dn[1773][7]) * p.p31);
        let eq37_e743_d_n8: f64 = ((-s.dn[1773][8]) * p.p31);
        let eq37_e743_d_n9: f64 = ((-s.dn[1773][9]) * p.p31);
        let eq37_e743_d_n10: f64 = ((-s.dn[1773][10]) * p.p31);
        let eq37_e743_d_n11: f64 = ((-s.dn[1773][11]) * p.p31);
        let eq37_e743_d_n12: f64 = ((-s.dn[1773][12]) * p.p31);
        let eq37_e743_d_n13: f64 = ((-s.dn[1773][13]) * p.p31);
        let eq37_e743_d_b0: f64 = ((-s.db[1773][0]) * p.p31);
        let eq37_e743_d_b1: f64 = ((-s.db[1773][1]) * p.p31);
        let eq37_e743_d_b2: f64 = ((-s.db[1773][2]) * p.p31);
        let eq37_e743_d_b3: f64 = ((-s.db[1773][3]) * p.p31);
        let eq37_e745: f64 = (eq37_e743 * s.v[13]);
        let eq37_e745_d_n0: f64 = ((eq37_e743_d_n0 * s.v[13]) + (eq37_e743 * s.dn[13][0]));
        let eq37_e745_d_n1: f64 = ((eq37_e743_d_n1 * s.v[13]) + (eq37_e743 * s.dn[13][1]));
        let eq37_e745_d_n2: f64 = ((eq37_e743_d_n2 * s.v[13]) + (eq37_e743 * s.dn[13][2]));
        let eq37_e745_d_n3: f64 = ((eq37_e743_d_n3 * s.v[13]) + (eq37_e743 * s.dn[13][3]));
        let eq37_e745_d_n4: f64 = ((eq37_e743_d_n4 * s.v[13]) + (eq37_e743 * s.dn[13][4]));
        let eq37_e745_d_n5: f64 = ((eq37_e743_d_n5 * s.v[13]) + (eq37_e743 * s.dn[13][5]));
        let eq37_e745_d_n6: f64 = ((eq37_e743_d_n6 * s.v[13]) + (eq37_e743 * s.dn[13][6]));
        let eq37_e745_d_n7: f64 = ((eq37_e743_d_n7 * s.v[13]) + (eq37_e743 * s.dn[13][7]));
        let eq37_e745_d_n8: f64 = ((eq37_e743_d_n8 * s.v[13]) + (eq37_e743 * s.dn[13][8]));
        let eq37_e745_d_n9: f64 = ((eq37_e743_d_n9 * s.v[13]) + (eq37_e743 * s.dn[13][9]));
        let eq37_e745_d_n10: f64 = ((eq37_e743_d_n10 * s.v[13]) + (eq37_e743 * s.dn[13][10]));
        let eq37_e745_d_n11: f64 = ((eq37_e743_d_n11 * s.v[13]) + (eq37_e743 * s.dn[13][11]));
        let eq37_e745_d_n12: f64 = ((eq37_e743_d_n12 * s.v[13]) + (eq37_e743 * s.dn[13][12]));
        let eq37_e745_d_n13: f64 = ((eq37_e743_d_n13 * s.v[13]) + (eq37_e743 * s.dn[13][13]));
        let eq37_e745_d_b0: f64 = ((eq37_e743_d_b0 * s.v[13]) + (eq37_e743 * s.db[13][0]));
        let eq37_e745_d_b1: f64 = ((eq37_e743_d_b1 * s.v[13]) + (eq37_e743 * s.db[13][1]));
        let eq37_e745_d_b2: f64 = ((eq37_e743_d_b2 * s.v[13]) + (eq37_e743 * s.db[13][2]));
        let eq37_e745_d_b3: f64 = ((eq37_e743_d_b3 * s.v[13]) + (eq37_e743 * s.db[13][3]));
        let eq37_e747: f64 = (eq37_e745 * (nv12 - nv13));
        let eq37_e747_d_n0: f64 = (eq37_e745_d_n0 * (nv12 - nv13));
        let eq37_e747_d_n1: f64 = (eq37_e745_d_n1 * (nv12 - nv13));
        let eq37_e747_d_n2: f64 = (eq37_e745_d_n2 * (nv12 - nv13));
        let eq37_e747_d_n3: f64 = (eq37_e745_d_n3 * (nv12 - nv13));
        let eq37_e747_d_n4: f64 = (eq37_e745_d_n4 * (nv12 - nv13));
        let eq37_e747_d_n5: f64 = (eq37_e745_d_n5 * (nv12 - nv13));
        let eq37_e747_d_n6: f64 = (eq37_e745_d_n6 * (nv12 - nv13));
        let eq37_e747_d_n7: f64 = (eq37_e745_d_n7 * (nv12 - nv13));
        let eq37_e747_d_n8: f64 = (eq37_e745_d_n8 * (nv12 - nv13));
        let eq37_e747_d_n9: f64 = (eq37_e745_d_n9 * (nv12 - nv13));
        let eq37_e747_d_n10: f64 = (eq37_e745_d_n10 * (nv12 - nv13));
        let eq37_e747_d_n11: f64 = (eq37_e745_d_n11 * (nv12 - nv13));
        let eq37_e747_d_n12: f64 = ((eq37_e745_d_n12 * (nv12 - nv13)) + eq37_e745);
        let eq37_e747_d_n13: f64 = ((eq37_e745_d_n13 * (nv12 - nv13)) + (-eq37_e745));
        let eq37_e747_d_b0: f64 = (eq37_e745_d_b0 * (nv12 - nv13));
        let eq37_e747_d_b1: f64 = (eq37_e745_d_b1 * (nv12 - nv13));
        let eq37_e747_d_b2: f64 = (eq37_e745_d_b2 * (nv12 - nv13));
        let eq37_e747_d_b3: f64 = (eq37_e745_d_b3 * (nv12 - nv13));
        let eq37_e749: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 19, s.v[375]);
        let eq37_e750: f64 = (eq37_e747 + eq37_e749);
        let eq37_e750_d_n0: f64 = (eq37_e747_d_n0 + (s.dn[375][0] * ddt_scale));
        let eq37_e750_d_n1: f64 = (eq37_e747_d_n1 + (s.dn[375][1] * ddt_scale));
        let eq37_e750_d_n2: f64 = (eq37_e747_d_n2 + (s.dn[375][2] * ddt_scale));
        let eq37_e750_d_n3: f64 = (eq37_e747_d_n3 + (s.dn[375][3] * ddt_scale));
        let eq37_e750_d_n4: f64 = (eq37_e747_d_n4 + (s.dn[375][4] * ddt_scale));
        let eq37_e750_d_n5: f64 = (eq37_e747_d_n5 + (s.dn[375][5] * ddt_scale));
        let eq37_e750_d_n6: f64 = (eq37_e747_d_n6 + (s.dn[375][6] * ddt_scale));
        let eq37_e750_d_n7: f64 = (eq37_e747_d_n7 + (s.dn[375][7] * ddt_scale));
        let eq37_e750_d_n8: f64 = (eq37_e747_d_n8 + (s.dn[375][8] * ddt_scale));
        let eq37_e750_d_n9: f64 = (eq37_e747_d_n9 + (s.dn[375][9] * ddt_scale));
        let eq37_e750_d_n10: f64 = (eq37_e747_d_n10 + (s.dn[375][10] * ddt_scale));
        let eq37_e750_d_n11: f64 = (eq37_e747_d_n11 + (s.dn[375][11] * ddt_scale));
        let eq37_e750_d_n12: f64 = (eq37_e747_d_n12 + (s.dn[375][12] * ddt_scale));
        let eq37_e750_d_n13: f64 = (eq37_e747_d_n13 + (s.dn[375][13] * ddt_scale));
        let eq37_e750_d_b0: f64 = (eq37_e747_d_b0 + (s.db[375][0] * ddt_scale));
        let eq37_e750_d_b1: f64 = (eq37_e747_d_b1 + (s.db[375][1] * ddt_scale));
        let eq37_e750_d_b2: f64 = (eq37_e747_d_b2 + (s.db[375][2] * ddt_scale));
        let eq37_e750_d_b3: f64 = (eq37_e747_d_b3 + (s.db[375][3] * ddt_scale));
        let eq37_e751: f64 = (p.p14 * eq37_e750);
        let eq37_e751_d_n0: f64 = (p.p14 * eq37_e750_d_n0);
        let eq37_e751_d_n1: f64 = (p.p14 * eq37_e750_d_n1);
        let eq37_e751_d_n2: f64 = (p.p14 * eq37_e750_d_n2);
        let eq37_e751_d_n3: f64 = (p.p14 * eq37_e750_d_n3);
        let eq37_e751_d_n4: f64 = (p.p14 * eq37_e750_d_n4);
        let eq37_e751_d_n5: f64 = (p.p14 * eq37_e750_d_n5);
        let eq37_e751_d_n6: f64 = (p.p14 * eq37_e750_d_n6);
        let eq37_e751_d_n7: f64 = (p.p14 * eq37_e750_d_n7);
        let eq37_e751_d_n8: f64 = (p.p14 * eq37_e750_d_n8);
        let eq37_e751_d_n9: f64 = (p.p14 * eq37_e750_d_n9);
        let eq37_e751_d_n10: f64 = (p.p14 * eq37_e750_d_n10);
        let eq37_e751_d_n11: f64 = (p.p14 * eq37_e750_d_n11);
        let eq37_e751_d_n12: f64 = (p.p14 * eq37_e750_d_n12);
        let eq37_e751_d_n13: f64 = (p.p14 * eq37_e750_d_n13);
        let eq37_e751_d_b0: f64 = (p.p14 * eq37_e750_d_b0);
        let eq37_e751_d_b1: f64 = (p.p14 * eq37_e750_d_b1);
        let eq37_e751_d_b2: f64 = (p.p14 * eq37_e750_d_b2);
        let eq37_e751_d_b3: f64 = (p.p14 * eq37_e750_d_b3);
        let eq37_value: f64 = eq37_e751;
        let eq37_node_derivatives: [f64; 14] = [eq37_e751_d_n0, eq37_e751_d_n1, eq37_e751_d_n2, eq37_e751_d_n3, eq37_e751_d_n4, eq37_e751_d_n5, eq37_e751_d_n6, eq37_e751_d_n7, eq37_e751_d_n8, eq37_e751_d_n9, eq37_e751_d_n10, eq37_e751_d_n11, eq37_e751_d_n12, eq37_e751_d_n13];
        let eq37_branch_derivatives: [f64; 4] = [eq37_e751_d_b0, eq37_e751_d_b1, eq37_e751_d_b2, eq37_e751_d_b3];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq37_value),
            &eq37_node_derivatives,
            &eq37_branch_derivatives,
            multiplicity,
        );
        let eq38_e753: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 20, s.v[378]);
        let eq38_value: f64 = eq38_e753;
        let eq38_node_derivatives: [f64; 14] = [(s.dn[378][0] * ddt_scale), (s.dn[378][1] * ddt_scale), (s.dn[378][2] * ddt_scale), (s.dn[378][3] * ddt_scale), (s.dn[378][4] * ddt_scale), (s.dn[378][5] * ddt_scale), (s.dn[378][6] * ddt_scale), (s.dn[378][7] * ddt_scale), (s.dn[378][8] * ddt_scale), (s.dn[378][9] * ddt_scale), (s.dn[378][10] * ddt_scale), (s.dn[378][11] * ddt_scale), (s.dn[378][12] * ddt_scale), (s.dn[378][13] * ddt_scale)];
        let eq38_branch_derivatives: [f64; 4] = [(s.db[378][0] * ddt_scale), (s.db[378][1] * ddt_scale), (s.db[378][2] * ddt_scale), (s.db[378][3] * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq38_value),
            &eq38_node_derivatives,
            &eq38_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq40_e759: f64 = (s.v[1803] * (nv5 - 0.0));
        let eq40_e759_d_n0: f64 = (s.dn[1803][0] * (nv5 - 0.0));
        let eq40_e759_d_n1: f64 = (s.dn[1803][1] * (nv5 - 0.0));
        let eq40_e759_d_n2: f64 = (s.dn[1803][2] * (nv5 - 0.0));
        let eq40_e759_d_n3: f64 = (s.dn[1803][3] * (nv5 - 0.0));
        let eq40_e759_d_n4: f64 = (s.dn[1803][4] * (nv5 - 0.0));
        let eq40_e759_d_n5: f64 = ((s.dn[1803][5] * (nv5 - 0.0)) + s.v[1803]);
        let eq40_e759_d_n6: f64 = (s.dn[1803][6] * (nv5 - 0.0));
        let eq40_e759_d_n7: f64 = (s.dn[1803][7] * (nv5 - 0.0));
        let eq40_e759_d_n8: f64 = (s.dn[1803][8] * (nv5 - 0.0));
        let eq40_e759_d_n9: f64 = (s.dn[1803][9] * (nv5 - 0.0));
        let eq40_e759_d_n10: f64 = (s.dn[1803][10] * (nv5 - 0.0));
        let eq40_e759_d_n11: f64 = (s.dn[1803][11] * (nv5 - 0.0));
        let eq40_e759_d_n12: f64 = (s.dn[1803][12] * (nv5 - 0.0));
        let eq40_e759_d_n13: f64 = (s.dn[1803][13] * (nv5 - 0.0));
        let eq40_e759_d_b0: f64 = (s.db[1803][0] * (nv5 - 0.0));
        let eq40_e759_d_b1: f64 = (s.db[1803][1] * (nv5 - 0.0));
        let eq40_e759_d_b2: f64 = (s.db[1803][2] * (nv5 - 0.0));
        let eq40_e759_d_b3: f64 = (s.db[1803][3] * (nv5 - 0.0));
        let eq40_value: f64 = eq40_e759;
        let eq40_node_derivatives: [f64; 14] = [eq40_e759_d_n0, eq40_e759_d_n1, eq40_e759_d_n2, eq40_e759_d_n3, eq40_e759_d_n4, eq40_e759_d_n5, eq40_e759_d_n6, eq40_e759_d_n7, eq40_e759_d_n8, eq40_e759_d_n9, eq40_e759_d_n10, eq40_e759_d_n11, eq40_e759_d_n12, eq40_e759_d_n13];
        let eq40_branch_derivatives: [f64; 4] = [eq40_e759_d_b0, eq40_e759_d_b1, eq40_e759_d_b2, eq40_e759_d_b3];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq40_value),
            &eq40_node_derivatives,
            &eq40_branch_derivatives,
            multiplicity,
        );
        let eq41_e762: f64 = (s.v[1800] * (nv5 - 0.0));
        let eq41_e762_d_n0: f64 = (s.dn[1800][0] * (nv5 - 0.0));
        let eq41_e762_d_n1: f64 = (s.dn[1800][1] * (nv5 - 0.0));
        let eq41_e762_d_n2: f64 = (s.dn[1800][2] * (nv5 - 0.0));
        let eq41_e762_d_n3: f64 = (s.dn[1800][3] * (nv5 - 0.0));
        let eq41_e762_d_n4: f64 = (s.dn[1800][4] * (nv5 - 0.0));
        let eq41_e762_d_n5: f64 = ((s.dn[1800][5] * (nv5 - 0.0)) + s.v[1800]);
        let eq41_e762_d_n6: f64 = (s.dn[1800][6] * (nv5 - 0.0));
        let eq41_e762_d_n7: f64 = (s.dn[1800][7] * (nv5 - 0.0));
        let eq41_e762_d_n8: f64 = (s.dn[1800][8] * (nv5 - 0.0));
        let eq41_e762_d_n9: f64 = (s.dn[1800][9] * (nv5 - 0.0));
        let eq41_e762_d_n10: f64 = (s.dn[1800][10] * (nv5 - 0.0));
        let eq41_e762_d_n11: f64 = (s.dn[1800][11] * (nv5 - 0.0));
        let eq41_e762_d_n12: f64 = (s.dn[1800][12] * (nv5 - 0.0));
        let eq41_e762_d_n13: f64 = (s.dn[1800][13] * (nv5 - 0.0));
        let eq41_e762_d_b0: f64 = (s.db[1800][0] * (nv5 - 0.0));
        let eq41_e762_d_b1: f64 = (s.db[1800][1] * (nv5 - 0.0));
        let eq41_e762_d_b2: f64 = (s.db[1800][2] * (nv5 - 0.0));
        let eq41_e762_d_b3: f64 = (s.db[1800][3] * (nv5 - 0.0));
        let eq41_e763: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 21, eq41_e762);
        let eq41_value: f64 = eq41_e763;
        let eq41_node_derivatives: [f64; 14] = [(eq41_e762_d_n0 * ddt_scale), (eq41_e762_d_n1 * ddt_scale), (eq41_e762_d_n2 * ddt_scale), (eq41_e762_d_n3 * ddt_scale), (eq41_e762_d_n4 * ddt_scale), (eq41_e762_d_n5 * ddt_scale), (eq41_e762_d_n6 * ddt_scale), (eq41_e762_d_n7 * ddt_scale), (eq41_e762_d_n8 * ddt_scale), (eq41_e762_d_n9 * ddt_scale), (eq41_e762_d_n10 * ddt_scale), (eq41_e762_d_n11 * ddt_scale), (eq41_e762_d_n12 * ddt_scale), (eq41_e762_d_n13 * ddt_scale)];
        let eq41_branch_derivatives: [f64; 4] = [(eq41_e762_d_b0 * ddt_scale), (eq41_e762_d_b1 * ddt_scale), (eq41_e762_d_b2 * ddt_scale), (eq41_e762_d_b3 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq41_value),
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let eq42_e765: f64 = (-s.v[1801]);
        let eq42_e767: f64 = (eq42_e765 * (nv5 - 0.0));
        let eq42_e767_d_n0: f64 = ((-s.dn[1801][0]) * (nv5 - 0.0));
        let eq42_e767_d_n1: f64 = ((-s.dn[1801][1]) * (nv5 - 0.0));
        let eq42_e767_d_n2: f64 = ((-s.dn[1801][2]) * (nv5 - 0.0));
        let eq42_e767_d_n3: f64 = ((-s.dn[1801][3]) * (nv5 - 0.0));
        let eq42_e767_d_n4: f64 = ((-s.dn[1801][4]) * (nv5 - 0.0));
        let eq42_e767_d_n5: f64 = (((-s.dn[1801][5]) * (nv5 - 0.0)) + eq42_e765);
        let eq42_e767_d_n6: f64 = ((-s.dn[1801][6]) * (nv5 - 0.0));
        let eq42_e767_d_n7: f64 = ((-s.dn[1801][7]) * (nv5 - 0.0));
        let eq42_e767_d_n8: f64 = ((-s.dn[1801][8]) * (nv5 - 0.0));
        let eq42_e767_d_n9: f64 = ((-s.dn[1801][9]) * (nv5 - 0.0));
        let eq42_e767_d_n10: f64 = ((-s.dn[1801][10]) * (nv5 - 0.0));
        let eq42_e767_d_n11: f64 = ((-s.dn[1801][11]) * (nv5 - 0.0));
        let eq42_e767_d_n12: f64 = ((-s.dn[1801][12]) * (nv5 - 0.0));
        let eq42_e767_d_n13: f64 = ((-s.dn[1801][13]) * (nv5 - 0.0));
        let eq42_e767_d_b0: f64 = ((-s.db[1801][0]) * (nv5 - 0.0));
        let eq42_e767_d_b1: f64 = ((-s.db[1801][1]) * (nv5 - 0.0));
        let eq42_e767_d_b2: f64 = ((-s.db[1801][2]) * (nv5 - 0.0));
        let eq42_e767_d_b3: f64 = ((-s.db[1801][3]) * (nv5 - 0.0));
        let eq42_e768: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 22, eq42_e767);
        let eq42_value: f64 = eq42_e768;
        let eq42_node_derivatives: [f64; 14] = [(eq42_e767_d_n0 * ddt_scale), (eq42_e767_d_n1 * ddt_scale), (eq42_e767_d_n2 * ddt_scale), (eq42_e767_d_n3 * ddt_scale), (eq42_e767_d_n4 * ddt_scale), (eq42_e767_d_n5 * ddt_scale), (eq42_e767_d_n6 * ddt_scale), (eq42_e767_d_n7 * ddt_scale), (eq42_e767_d_n8 * ddt_scale), (eq42_e767_d_n9 * ddt_scale), (eq42_e767_d_n10 * ddt_scale), (eq42_e767_d_n11 * ddt_scale), (eq42_e767_d_n12 * ddt_scale), (eq42_e767_d_n13 * ddt_scale)];
        let eq42_branch_derivatives: [f64; 4] = [(eq42_e767_d_b0 * ddt_scale), (eq42_e767_d_b1 * ddt_scale), (eq42_e767_d_b2 * ddt_scale), (eq42_e767_d_b3 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq42_value),
            &eq42_node_derivatives,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let eq43_e770: f64 = (-s.v[1802]);
        let eq43_e772: f64 = (eq43_e770 * (nv5 - 0.0));
        let eq43_e772_d_n0: f64 = ((-s.dn[1802][0]) * (nv5 - 0.0));
        let eq43_e772_d_n1: f64 = ((-s.dn[1802][1]) * (nv5 - 0.0));
        let eq43_e772_d_n2: f64 = ((-s.dn[1802][2]) * (nv5 - 0.0));
        let eq43_e772_d_n3: f64 = ((-s.dn[1802][3]) * (nv5 - 0.0));
        let eq43_e772_d_n4: f64 = ((-s.dn[1802][4]) * (nv5 - 0.0));
        let eq43_e772_d_n5: f64 = (((-s.dn[1802][5]) * (nv5 - 0.0)) + eq43_e770);
        let eq43_e772_d_n6: f64 = ((-s.dn[1802][6]) * (nv5 - 0.0));
        let eq43_e772_d_n7: f64 = ((-s.dn[1802][7]) * (nv5 - 0.0));
        let eq43_e772_d_n8: f64 = ((-s.dn[1802][8]) * (nv5 - 0.0));
        let eq43_e772_d_n9: f64 = ((-s.dn[1802][9]) * (nv5 - 0.0));
        let eq43_e772_d_n10: f64 = ((-s.dn[1802][10]) * (nv5 - 0.0));
        let eq43_e772_d_n11: f64 = ((-s.dn[1802][11]) * (nv5 - 0.0));
        let eq43_e772_d_n12: f64 = ((-s.dn[1802][12]) * (nv5 - 0.0));
        let eq43_e772_d_n13: f64 = ((-s.dn[1802][13]) * (nv5 - 0.0));
        let eq43_e772_d_b0: f64 = ((-s.db[1802][0]) * (nv5 - 0.0));
        let eq43_e772_d_b1: f64 = ((-s.db[1802][1]) * (nv5 - 0.0));
        let eq43_e772_d_b2: f64 = ((-s.db[1802][2]) * (nv5 - 0.0));
        let eq43_e772_d_b3: f64 = ((-s.db[1802][3]) * (nv5 - 0.0));
        let eq43_e773: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 23, eq43_e772);
        let eq43_value: f64 = eq43_e773;
        let eq43_node_derivatives: [f64; 14] = [(eq43_e772_d_n0 * ddt_scale), (eq43_e772_d_n1 * ddt_scale), (eq43_e772_d_n2 * ddt_scale), (eq43_e772_d_n3 * ddt_scale), (eq43_e772_d_n4 * ddt_scale), (eq43_e772_d_n5 * ddt_scale), (eq43_e772_d_n6 * ddt_scale), (eq43_e772_d_n7 * ddt_scale), (eq43_e772_d_n8 * ddt_scale), (eq43_e772_d_n9 * ddt_scale), (eq43_e772_d_n10 * ddt_scale), (eq43_e772_d_n11 * ddt_scale), (eq43_e772_d_n12 * ddt_scale), (eq43_e772_d_n13 * ddt_scale)];
        let eq43_branch_derivatives: [f64; 4] = [(eq43_e772_d_b0 * ddt_scale), (eq43_e772_d_b1 * ddt_scale), (eq43_e772_d_b2 * ddt_scale), (eq43_e772_d_b3 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq43_value),
            &eq43_node_derivatives,
            &eq43_branch_derivatives,
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let __rspice_deriv_cse_0: f64 = (s.dn[1774][0] + s.dn[1775][0]);
        let __rspice_deriv_cse_1: f64 = (s.dn[1774][1] + s.dn[1775][1]);
        let __rspice_deriv_cse_2: f64 = (s.dn[1774][2] + s.dn[1775][2]);
        let __rspice_deriv_cse_3: f64 = (s.dn[1774][3] + s.dn[1775][3]);
        let __rspice_deriv_cse_4: f64 = (s.dn[1774][4] + s.dn[1775][4]);
        let __rspice_deriv_cse_5: f64 = (s.dn[1774][5] + s.dn[1775][5]);
        let __rspice_deriv_cse_6: f64 = (s.dn[1774][6] + s.dn[1775][6]);
        let __rspice_deriv_cse_7: f64 = (s.dn[1774][7] + s.dn[1775][7]);
        let __rspice_deriv_cse_8: f64 = (s.dn[1774][8] + s.dn[1775][8]);
        let __rspice_deriv_cse_9: f64 = (s.dn[1774][9] + s.dn[1775][9]);
        let __rspice_deriv_cse_10: f64 = (s.dn[1774][10] + s.dn[1775][10]);
        let __rspice_deriv_cse_11: f64 = (s.dn[1774][11] + s.dn[1775][11]);
        let __rspice_deriv_cse_12: f64 = (s.dn[1774][12] + s.dn[1775][12]);
        let __rspice_deriv_cse_13: f64 = (s.dn[1774][13] + s.dn[1775][13]);
        let __rspice_deriv_cse_14: f64 = (s.db[1774][0] + s.db[1775][0]);
        let __rspice_deriv_cse_15: f64 = (s.db[1774][1] + s.db[1775][1]);
        let __rspice_deriv_cse_16: f64 = (s.db[1774][2] + s.db[1775][2]);
        let __rspice_deriv_cse_17: f64 = (s.db[1774][3] + s.db[1775][3]);
        let eq23_e642: f64 = (s.v[1774] + s.v[1775]);
        let eq23_e643: f64 = (s.v[181] * eq23_e642);
        let eq23_e643_d_n0: f64 = ((s.dn[181][0] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_0));
        let eq23_e643_d_n1: f64 = ((s.dn[181][1] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_1));
        let eq23_e643_d_n2: f64 = ((s.dn[181][2] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_2));
        let eq23_e643_d_n3: f64 = ((s.dn[181][3] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_3));
        let eq23_e643_d_n4: f64 = ((s.dn[181][4] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_4));
        let eq23_e643_d_n5: f64 = ((s.dn[181][5] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_5));
        let eq23_e643_d_n6: f64 = ((s.dn[181][6] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_6));
        let eq23_e643_d_n7: f64 = ((s.dn[181][7] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_7));
        let eq23_e643_d_n8: f64 = ((s.dn[181][8] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_8));
        let eq23_e643_d_n9: f64 = ((s.dn[181][9] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_9));
        let eq23_e643_d_n10: f64 = ((s.dn[181][10] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_10));
        let eq23_e643_d_n11: f64 = ((s.dn[181][11] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_11));
        let eq23_e643_d_n12: f64 = ((s.dn[181][12] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_12));
        let eq23_e643_d_n13: f64 = ((s.dn[181][13] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_13));
        let eq23_e643_d_b0: f64 = ((s.db[181][0] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_14));
        let eq23_e643_d_b1: f64 = ((s.db[181][1] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_15));
        let eq23_e643_d_b2: f64 = ((s.db[181][2] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_16));
        let eq23_e643_d_b3: f64 = ((s.db[181][3] * eq23_e642) + (s.v[181] * __rspice_deriv_cse_17));
        let eq23_e644_q: f64 = eq23_e643;
        let eq23_reactive_node_derivatives: [f64; 14] = [eq23_e643_d_n0, eq23_e643_d_n1, eq23_e643_d_n2, eq23_e643_d_n3, eq23_e643_d_n4, eq23_e643_d_n5, eq23_e643_d_n6, eq23_e643_d_n7, eq23_e643_d_n8, eq23_e643_d_n9, eq23_e643_d_n10, eq23_e643_d_n11, eq23_e643_d_n12, eq23_e643_d_n13];
        let eq23_reactive_branch_derivatives: [f64; 4] = [eq23_e643_d_b0, eq23_e643_d_b1, eq23_e643_d_b2, eq23_e643_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[13]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
        let eq26_e653_q: f64 = s.v[1776];
        let eq26_reactive_node_derivatives: [f64; 14] = [s.dn[1776][0], s.dn[1776][1], s.dn[1776][2], s.dn[1776][3], s.dn[1776][4], s.dn[1776][5], s.dn[1776][6], s.dn[1776][7], s.dn[1776][8], s.dn[1776][9], s.dn[1776][10], s.dn[1776][11], s.dn[1776][12], s.dn[1776][13]];
        let eq26_reactive_branch_derivatives: [f64; 4] = [s.db[1776][0], s.db[1776][1], s.db[1776][2], s.db[1776][3]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[13]),
            nodes,
            &eq26_reactive_node_derivatives,
            branches,
            &eq26_reactive_branch_derivatives,
            multiplicity,
        );
        let eq29_e662: f64 = (s.v[182]).sqrt();
        let eq29_e662_d_n0: f64 = (s.dn[182][0] / (2.0 * eq29_e662));
        let eq29_e662_d_n1: f64 = (s.dn[182][1] / (2.0 * eq29_e662));
        let eq29_e662_d_n2: f64 = (s.dn[182][2] / (2.0 * eq29_e662));
        let eq29_e662_d_n3: f64 = (s.dn[182][3] / (2.0 * eq29_e662));
        let eq29_e662_d_n4: f64 = (s.dn[182][4] / (2.0 * eq29_e662));
        let eq29_e662_d_n5: f64 = (s.dn[182][5] / (2.0 * eq29_e662));
        let eq29_e662_d_n6: f64 = (s.dn[182][6] / (2.0 * eq29_e662));
        let eq29_e662_d_n7: f64 = (s.dn[182][7] / (2.0 * eq29_e662));
        let eq29_e662_d_n8: f64 = (s.dn[182][8] / (2.0 * eq29_e662));
        let eq29_e662_d_n9: f64 = (s.dn[182][9] / (2.0 * eq29_e662));
        let eq29_e662_d_n10: f64 = (s.dn[182][10] / (2.0 * eq29_e662));
        let eq29_e662_d_n11: f64 = (s.dn[182][11] / (2.0 * eq29_e662));
        let eq29_e662_d_n12: f64 = (s.dn[182][12] / (2.0 * eq29_e662));
        let eq29_e662_d_n13: f64 = (s.dn[182][13] / (2.0 * eq29_e662));
        let eq29_e662_d_b0: f64 = (s.db[182][0] / (2.0 * eq29_e662));
        let eq29_e662_d_b1: f64 = (s.db[182][1] / (2.0 * eq29_e662));
        let eq29_e662_d_b2: f64 = (s.db[182][2] / (2.0 * eq29_e662));
        let eq29_e662_d_b3: f64 = (s.db[182][3] / (2.0 * eq29_e662));
        let eq29_e665: f64 = (1.0 - s.v[181]);
        let eq29_e668: f64 = (s.v[1774] + s.v[1775]);
        let eq29_e669: f64 = (eq29_e665 * eq29_e668);
        let eq29_e669_d_n0: f64 = (((-s.dn[181][0]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_0));
        let eq29_e669_d_n1: f64 = (((-s.dn[181][1]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_1));
        let eq29_e669_d_n2: f64 = (((-s.dn[181][2]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_2));
        let eq29_e669_d_n3: f64 = (((-s.dn[181][3]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_3));
        let eq29_e669_d_n4: f64 = (((-s.dn[181][4]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_4));
        let eq29_e669_d_n5: f64 = (((-s.dn[181][5]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_5));
        let eq29_e669_d_n6: f64 = (((-s.dn[181][6]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_6));
        let eq29_e669_d_n7: f64 = (((-s.dn[181][7]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_7));
        let eq29_e669_d_n8: f64 = (((-s.dn[181][8]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_8));
        let eq29_e669_d_n9: f64 = (((-s.dn[181][9]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_9));
        let eq29_e669_d_n10: f64 = (((-s.dn[181][10]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_10));
        let eq29_e669_d_n11: f64 = (((-s.dn[181][11]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_11));
        let eq29_e669_d_n12: f64 = (((-s.dn[181][12]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_12));
        let eq29_e669_d_n13: f64 = (((-s.dn[181][13]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_13));
        let eq29_e669_d_b0: f64 = (((-s.db[181][0]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_14));
        let eq29_e669_d_b1: f64 = (((-s.db[181][1]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_15));
        let eq29_e669_d_b2: f64 = (((-s.db[181][2]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_16));
        let eq29_e669_d_b3: f64 = (((-s.db[181][3]) * eq29_e668) + (eq29_e665 * __rspice_deriv_cse_17));
        let eq29_e670_q: f64 = eq29_e669;
        let eq29_e671: f64 = (eq29_e662 * eq29_e669);
        let eq29_e671_d_n0: f64 = ((eq29_e662_d_n0 * eq29_e669) + (eq29_e662 * eq29_e669_d_n0));
        let eq29_e671_d_n1: f64 = ((eq29_e662_d_n1 * eq29_e669) + (eq29_e662 * eq29_e669_d_n1));
        let eq29_e671_d_n2: f64 = ((eq29_e662_d_n2 * eq29_e669) + (eq29_e662 * eq29_e669_d_n2));
        let eq29_e671_d_n3: f64 = ((eq29_e662_d_n3 * eq29_e669) + (eq29_e662 * eq29_e669_d_n3));
        let eq29_e671_d_n4: f64 = ((eq29_e662_d_n4 * eq29_e669) + (eq29_e662 * eq29_e669_d_n4));
        let eq29_e671_d_n5: f64 = ((eq29_e662_d_n5 * eq29_e669) + (eq29_e662 * eq29_e669_d_n5));
        let eq29_e671_d_n6: f64 = ((eq29_e662_d_n6 * eq29_e669) + (eq29_e662 * eq29_e669_d_n6));
        let eq29_e671_d_n7: f64 = ((eq29_e662_d_n7 * eq29_e669) + (eq29_e662 * eq29_e669_d_n7));
        let eq29_e671_d_n8: f64 = ((eq29_e662_d_n8 * eq29_e669) + (eq29_e662 * eq29_e669_d_n8));
        let eq29_e671_d_n9: f64 = ((eq29_e662_d_n9 * eq29_e669) + (eq29_e662 * eq29_e669_d_n9));
        let eq29_e671_d_n10: f64 = ((eq29_e662_d_n10 * eq29_e669) + (eq29_e662 * eq29_e669_d_n10));
        let eq29_e671_d_n11: f64 = ((eq29_e662_d_n11 * eq29_e669) + (eq29_e662 * eq29_e669_d_n11));
        let eq29_e671_d_n12: f64 = ((eq29_e662_d_n12 * eq29_e669) + (eq29_e662 * eq29_e669_d_n12));
        let eq29_e671_d_n13: f64 = ((eq29_e662_d_n13 * eq29_e669) + (eq29_e662 * eq29_e669_d_n13));
        let eq29_e671_d_b0: f64 = ((eq29_e662_d_b0 * eq29_e669) + (eq29_e662 * eq29_e669_d_b0));
        let eq29_e671_d_b1: f64 = ((eq29_e662_d_b1 * eq29_e669) + (eq29_e662 * eq29_e669_d_b1));
        let eq29_e671_d_b2: f64 = ((eq29_e662_d_b2 * eq29_e669) + (eq29_e662 * eq29_e669_d_b2));
        let eq29_e671_d_b3: f64 = ((eq29_e662_d_b3 * eq29_e669) + (eq29_e662 * eq29_e669_d_b3));
        let eq29_e671_q: f64 = (eq29_e662 * eq29_e670_q);
        let eq29_e671_q_d_n0: f64 = ((eq29_e662_d_n0 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n0));
        let eq29_e671_q_d_n1: f64 = ((eq29_e662_d_n1 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n1));
        let eq29_e671_q_d_n2: f64 = ((eq29_e662_d_n2 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n2));
        let eq29_e671_q_d_n3: f64 = ((eq29_e662_d_n3 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n3));
        let eq29_e671_q_d_n4: f64 = ((eq29_e662_d_n4 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n4));
        let eq29_e671_q_d_n5: f64 = ((eq29_e662_d_n5 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n5));
        let eq29_e671_q_d_n6: f64 = ((eq29_e662_d_n6 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n6));
        let eq29_e671_q_d_n7: f64 = ((eq29_e662_d_n7 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n7));
        let eq29_e671_q_d_n8: f64 = ((eq29_e662_d_n8 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n8));
        let eq29_e671_q_d_n9: f64 = ((eq29_e662_d_n9 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n9));
        let eq29_e671_q_d_n10: f64 = ((eq29_e662_d_n10 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n10));
        let eq29_e671_q_d_n11: f64 = ((eq29_e662_d_n11 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n11));
        let eq29_e671_q_d_n12: f64 = ((eq29_e662_d_n12 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n12));
        let eq29_e671_q_d_n13: f64 = ((eq29_e662_d_n13 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_n13));
        let eq29_e671_q_d_b0: f64 = ((eq29_e662_d_b0 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_b0));
        let eq29_e671_q_d_b1: f64 = ((eq29_e662_d_b1 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_b1));
        let eq29_e671_q_d_b2: f64 = ((eq29_e662_d_b2 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_b2));
        let eq29_e671_q_d_b3: f64 = ((eq29_e662_d_b3 * eq29_e670_q) + (eq29_e662 * eq29_e669_d_b3));
        let eq29_reactive_node_derivatives: [f64; 14] = [eq29_e671_q_d_n0, eq29_e671_q_d_n1, eq29_e671_q_d_n2, eq29_e671_q_d_n3, eq29_e671_q_d_n4, eq29_e671_q_d_n5, eq29_e671_q_d_n6, eq29_e671_q_d_n7, eq29_e671_q_d_n8, eq29_e671_q_d_n9, eq29_e671_q_d_n10, eq29_e671_q_d_n11, eq29_e671_q_d_n12, eq29_e671_q_d_n13];
        let eq29_reactive_branch_derivatives: [f64; 4] = [eq29_e671_q_d_b0, eq29_e671_q_d_b1, eq29_e671_q_d_b2, eq29_e671_q_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[13]),
            nodes,
            &eq29_reactive_node_derivatives,
            branches,
            &eq29_reactive_branch_derivatives,
            multiplicity,
        );
        let eq31_e678: f64 = (1e-9 * (nv11 - nv13));
        let eq31_e679_q: f64 = eq31_e678;
        let eq31_e680: f64 = (s.v[182] * eq31_e678);
        let eq31_e680_d_n0: f64 = (s.dn[182][0] * eq31_e678);
        let eq31_e680_d_n1: f64 = (s.dn[182][1] * eq31_e678);
        let eq31_e680_d_n2: f64 = (s.dn[182][2] * eq31_e678);
        let eq31_e680_d_n3: f64 = (s.dn[182][3] * eq31_e678);
        let eq31_e680_d_n4: f64 = (s.dn[182][4] * eq31_e678);
        let eq31_e680_d_n5: f64 = (s.dn[182][5] * eq31_e678);
        let eq31_e680_d_n6: f64 = (s.dn[182][6] * eq31_e678);
        let eq31_e680_d_n7: f64 = (s.dn[182][7] * eq31_e678);
        let eq31_e680_d_n8: f64 = (s.dn[182][8] * eq31_e678);
        let eq31_e680_d_n9: f64 = (s.dn[182][9] * eq31_e678);
        let eq31_e680_d_n10: f64 = (s.dn[182][10] * eq31_e678);
        let eq31_e680_d_n11: f64 = ((s.dn[182][11] * eq31_e678) + (s.v[182] * 1e-9));
        let eq31_e680_d_n12: f64 = (s.dn[182][12] * eq31_e678);
        let eq31_e680_d_n13: f64 = ((s.dn[182][13] * eq31_e678) + (s.v[182] * (-1e-9)));
        let eq31_e680_d_b0: f64 = (s.db[182][0] * eq31_e678);
        let eq31_e680_d_b1: f64 = (s.db[182][1] * eq31_e678);
        let eq31_e680_d_b2: f64 = (s.db[182][2] * eq31_e678);
        let eq31_e680_d_b3: f64 = (s.db[182][3] * eq31_e678);
        let eq31_e680_q: f64 = (s.v[182] * eq31_e679_q);
        let eq31_e680_q_d_n0: f64 = (s.dn[182][0] * eq31_e679_q);
        let eq31_e680_q_d_n1: f64 = (s.dn[182][1] * eq31_e679_q);
        let eq31_e680_q_d_n2: f64 = (s.dn[182][2] * eq31_e679_q);
        let eq31_e680_q_d_n3: f64 = (s.dn[182][3] * eq31_e679_q);
        let eq31_e680_q_d_n4: f64 = (s.dn[182][4] * eq31_e679_q);
        let eq31_e680_q_d_n5: f64 = (s.dn[182][5] * eq31_e679_q);
        let eq31_e680_q_d_n6: f64 = (s.dn[182][6] * eq31_e679_q);
        let eq31_e680_q_d_n7: f64 = (s.dn[182][7] * eq31_e679_q);
        let eq31_e680_q_d_n8: f64 = (s.dn[182][8] * eq31_e679_q);
        let eq31_e680_q_d_n9: f64 = (s.dn[182][9] * eq31_e679_q);
        let eq31_e680_q_d_n10: f64 = (s.dn[182][10] * eq31_e679_q);
        let eq31_e680_q_d_n11: f64 = ((s.dn[182][11] * eq31_e679_q) + (s.v[182] * 1e-9));
        let eq31_e680_q_d_n12: f64 = (s.dn[182][12] * eq31_e679_q);
        let eq31_e680_q_d_n13: f64 = ((s.dn[182][13] * eq31_e679_q) + (s.v[182] * (-1e-9)));
        let eq31_e680_q_d_b0: f64 = (s.db[182][0] * eq31_e679_q);
        let eq31_e680_q_d_b1: f64 = (s.db[182][1] * eq31_e679_q);
        let eq31_e680_q_d_b2: f64 = (s.db[182][2] * eq31_e679_q);
        let eq31_e680_q_d_b3: f64 = (s.db[182][3] * eq31_e679_q);
        let eq31_reactive_node_derivatives: [f64; 14] = [eq31_e680_q_d_n0, eq31_e680_q_d_n1, eq31_e680_q_d_n2, eq31_e680_q_d_n3, eq31_e680_q_d_n4, eq31_e680_q_d_n5, eq31_e680_q_d_n6, eq31_e680_q_d_n7, eq31_e680_q_d_n8, eq31_e680_q_d_n9, eq31_e680_q_d_n10, eq31_e680_q_d_n11, eq31_e680_q_d_n12, eq31_e680_q_d_n13];
        let eq31_reactive_branch_derivatives: [f64; 4] = [eq31_e680_q_d_b0, eq31_e680_q_d_b1, eq31_e680_q_d_b2, eq31_e680_q_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[13]),
            nodes,
            &eq31_reactive_node_derivatives,
            branches,
            &eq31_reactive_branch_derivatives,
            multiplicity,
        );
        let eq32_e683_q: f64 = s.v[362];
        let eq32_e685_q: f64 = s.v[377];
        let eq32_e686: f64 = (s.v[362] + s.v[377]);
        let eq32_e686_d_n0: f64 = (s.dn[362][0] + s.dn[377][0]);
        let eq32_e686_d_n1: f64 = (s.dn[362][1] + s.dn[377][1]);
        let eq32_e686_d_n2: f64 = (s.dn[362][2] + s.dn[377][2]);
        let eq32_e686_d_n3: f64 = (s.dn[362][3] + s.dn[377][3]);
        let eq32_e686_d_n4: f64 = (s.dn[362][4] + s.dn[377][4]);
        let eq32_e686_d_n5: f64 = (s.dn[362][5] + s.dn[377][5]);
        let eq32_e686_d_n6: f64 = (s.dn[362][6] + s.dn[377][6]);
        let eq32_e686_d_n7: f64 = (s.dn[362][7] + s.dn[377][7]);
        let eq32_e686_d_n8: f64 = (s.dn[362][8] + s.dn[377][8]);
        let eq32_e686_d_n9: f64 = (s.dn[362][9] + s.dn[377][9]);
        let eq32_e686_d_n10: f64 = (s.dn[362][10] + s.dn[377][10]);
        let eq32_e686_d_n11: f64 = (s.dn[362][11] + s.dn[377][11]);
        let eq32_e686_d_n12: f64 = (s.dn[362][12] + s.dn[377][12]);
        let eq32_e686_d_n13: f64 = (s.dn[362][13] + s.dn[377][13]);
        let eq32_e686_d_b0: f64 = (s.db[362][0] + s.db[377][0]);
        let eq32_e686_d_b1: f64 = (s.db[362][1] + s.db[377][1]);
        let eq32_e686_d_b2: f64 = (s.db[362][2] + s.db[377][2]);
        let eq32_e686_d_b3: f64 = (s.db[362][3] + s.db[377][3]);
        let eq32_e686_q: f64 = (eq32_e683_q + eq32_e685_q);
        let eq32_e688_q: f64 = s.v[381];
        let eq32_e689: f64 = (eq32_e686 + s.v[381]);
        let eq32_e689_d_n0: f64 = (eq32_e686_d_n0 + s.dn[381][0]);
        let eq32_e689_d_n1: f64 = (eq32_e686_d_n1 + s.dn[381][1]);
        let eq32_e689_d_n2: f64 = (eq32_e686_d_n2 + s.dn[381][2]);
        let eq32_e689_d_n3: f64 = (eq32_e686_d_n3 + s.dn[381][3]);
        let eq32_e689_d_n4: f64 = (eq32_e686_d_n4 + s.dn[381][4]);
        let eq32_e689_d_n5: f64 = (eq32_e686_d_n5 + s.dn[381][5]);
        let eq32_e689_d_n6: f64 = (eq32_e686_d_n6 + s.dn[381][6]);
        let eq32_e689_d_n7: f64 = (eq32_e686_d_n7 + s.dn[381][7]);
        let eq32_e689_d_n8: f64 = (eq32_e686_d_n8 + s.dn[381][8]);
        let eq32_e689_d_n9: f64 = (eq32_e686_d_n9 + s.dn[381][9]);
        let eq32_e689_d_n10: f64 = (eq32_e686_d_n10 + s.dn[381][10]);
        let eq32_e689_d_n11: f64 = (eq32_e686_d_n11 + s.dn[381][11]);
        let eq32_e689_d_n12: f64 = (eq32_e686_d_n12 + s.dn[381][12]);
        let eq32_e689_d_n13: f64 = (eq32_e686_d_n13 + s.dn[381][13]);
        let eq32_e689_d_b0: f64 = (eq32_e686_d_b0 + s.db[381][0]);
        let eq32_e689_d_b1: f64 = (eq32_e686_d_b1 + s.db[381][1]);
        let eq32_e689_d_b2: f64 = (eq32_e686_d_b2 + s.db[381][2]);
        let eq32_e689_d_b3: f64 = (eq32_e686_d_b3 + s.db[381][3]);
        let eq32_e689_q: f64 = (eq32_e686_q + eq32_e688_q);
        let eq32_e690: f64 = (p.p14 * eq32_e689);
        let eq32_e690_d_n0: f64 = (p.p14 * eq32_e689_d_n0);
        let eq32_e690_d_n1: f64 = (p.p14 * eq32_e689_d_n1);
        let eq32_e690_d_n2: f64 = (p.p14 * eq32_e689_d_n2);
        let eq32_e690_d_n3: f64 = (p.p14 * eq32_e689_d_n3);
        let eq32_e690_d_n4: f64 = (p.p14 * eq32_e689_d_n4);
        let eq32_e690_d_n5: f64 = (p.p14 * eq32_e689_d_n5);
        let eq32_e690_d_n6: f64 = (p.p14 * eq32_e689_d_n6);
        let eq32_e690_d_n7: f64 = (p.p14 * eq32_e689_d_n7);
        let eq32_e690_d_n8: f64 = (p.p14 * eq32_e689_d_n8);
        let eq32_e690_d_n9: f64 = (p.p14 * eq32_e689_d_n9);
        let eq32_e690_d_n10: f64 = (p.p14 * eq32_e689_d_n10);
        let eq32_e690_d_n11: f64 = (p.p14 * eq32_e689_d_n11);
        let eq32_e690_d_n12: f64 = (p.p14 * eq32_e689_d_n12);
        let eq32_e690_d_n13: f64 = (p.p14 * eq32_e689_d_n13);
        let eq32_e690_d_b0: f64 = (p.p14 * eq32_e689_d_b0);
        let eq32_e690_d_b1: f64 = (p.p14 * eq32_e689_d_b1);
        let eq32_e690_d_b2: f64 = (p.p14 * eq32_e689_d_b2);
        let eq32_e690_d_b3: f64 = (p.p14 * eq32_e689_d_b3);
        let eq32_e690_q: f64 = (p.p14 * eq32_e689_q);
        let eq32_reactive_node_derivatives: [f64; 14] = [eq32_e690_d_n0, eq32_e690_d_n1, eq32_e690_d_n2, eq32_e690_d_n3, eq32_e690_d_n4, eq32_e690_d_n5, eq32_e690_d_n6, eq32_e690_d_n7, eq32_e690_d_n8, eq32_e690_d_n9, eq32_e690_d_n10, eq32_e690_d_n11, eq32_e690_d_n12, eq32_e690_d_n13];
        let eq32_reactive_branch_derivatives: [f64; 4] = [eq32_e690_d_b0, eq32_e690_d_b1, eq32_e690_d_b2, eq32_e690_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq32_reactive_node_derivatives,
            branches,
            &eq32_reactive_branch_derivatives,
            multiplicity,
        );
        let eq33_e693_q: f64 = s.v[371];
        let eq33_e695_q: f64 = s.v[373];
        let eq33_e696: f64 = (s.v[371] + s.v[373]);
        let eq33_e696_d_n0: f64 = (s.dn[371][0] + s.dn[373][0]);
        let eq33_e696_d_n1: f64 = (s.dn[371][1] + s.dn[373][1]);
        let eq33_e696_d_n2: f64 = (s.dn[371][2] + s.dn[373][2]);
        let eq33_e696_d_n3: f64 = (s.dn[371][3] + s.dn[373][3]);
        let eq33_e696_d_n4: f64 = (s.dn[371][4] + s.dn[373][4]);
        let eq33_e696_d_n5: f64 = (s.dn[371][5] + s.dn[373][5]);
        let eq33_e696_d_n6: f64 = (s.dn[371][6] + s.dn[373][6]);
        let eq33_e696_d_n7: f64 = (s.dn[371][7] + s.dn[373][7]);
        let eq33_e696_d_n8: f64 = (s.dn[371][8] + s.dn[373][8]);
        let eq33_e696_d_n9: f64 = (s.dn[371][9] + s.dn[373][9]);
        let eq33_e696_d_n10: f64 = (s.dn[371][10] + s.dn[373][10]);
        let eq33_e696_d_n11: f64 = (s.dn[371][11] + s.dn[373][11]);
        let eq33_e696_d_n12: f64 = (s.dn[371][12] + s.dn[373][12]);
        let eq33_e696_d_n13: f64 = (s.dn[371][13] + s.dn[373][13]);
        let eq33_e696_d_b0: f64 = (s.db[371][0] + s.db[373][0]);
        let eq33_e696_d_b1: f64 = (s.db[371][1] + s.db[373][1]);
        let eq33_e696_d_b2: f64 = (s.db[371][2] + s.db[373][2]);
        let eq33_e696_d_b3: f64 = (s.db[371][3] + s.db[373][3]);
        let eq33_e696_q: f64 = (eq33_e693_q + eq33_e695_q);
        let eq33_e698_q: f64 = s.v[380];
        let eq33_e699: f64 = (eq33_e696 + s.v[380]);
        let eq33_e699_d_n0: f64 = (eq33_e696_d_n0 + s.dn[380][0]);
        let eq33_e699_d_n1: f64 = (eq33_e696_d_n1 + s.dn[380][1]);
        let eq33_e699_d_n2: f64 = (eq33_e696_d_n2 + s.dn[380][2]);
        let eq33_e699_d_n3: f64 = (eq33_e696_d_n3 + s.dn[380][3]);
        let eq33_e699_d_n4: f64 = (eq33_e696_d_n4 + s.dn[380][4]);
        let eq33_e699_d_n5: f64 = (eq33_e696_d_n5 + s.dn[380][5]);
        let eq33_e699_d_n6: f64 = (eq33_e696_d_n6 + s.dn[380][6]);
        let eq33_e699_d_n7: f64 = (eq33_e696_d_n7 + s.dn[380][7]);
        let eq33_e699_d_n8: f64 = (eq33_e696_d_n8 + s.dn[380][8]);
        let eq33_e699_d_n9: f64 = (eq33_e696_d_n9 + s.dn[380][9]);
        let eq33_e699_d_n10: f64 = (eq33_e696_d_n10 + s.dn[380][10]);
        let eq33_e699_d_n11: f64 = (eq33_e696_d_n11 + s.dn[380][11]);
        let eq33_e699_d_n12: f64 = (eq33_e696_d_n12 + s.dn[380][12]);
        let eq33_e699_d_n13: f64 = (eq33_e696_d_n13 + s.dn[380][13]);
        let eq33_e699_d_b0: f64 = (eq33_e696_d_b0 + s.db[380][0]);
        let eq33_e699_d_b1: f64 = (eq33_e696_d_b1 + s.db[380][1]);
        let eq33_e699_d_b2: f64 = (eq33_e696_d_b2 + s.db[380][2]);
        let eq33_e699_d_b3: f64 = (eq33_e696_d_b3 + s.db[380][3]);
        let eq33_e699_q: f64 = (eq33_e696_q + eq33_e698_q);
        let eq33_e700: f64 = (p.p14 * eq33_e699);
        let eq33_e700_d_n0: f64 = (p.p14 * eq33_e699_d_n0);
        let eq33_e700_d_n1: f64 = (p.p14 * eq33_e699_d_n1);
        let eq33_e700_d_n2: f64 = (p.p14 * eq33_e699_d_n2);
        let eq33_e700_d_n3: f64 = (p.p14 * eq33_e699_d_n3);
        let eq33_e700_d_n4: f64 = (p.p14 * eq33_e699_d_n4);
        let eq33_e700_d_n5: f64 = (p.p14 * eq33_e699_d_n5);
        let eq33_e700_d_n6: f64 = (p.p14 * eq33_e699_d_n6);
        let eq33_e700_d_n7: f64 = (p.p14 * eq33_e699_d_n7);
        let eq33_e700_d_n8: f64 = (p.p14 * eq33_e699_d_n8);
        let eq33_e700_d_n9: f64 = (p.p14 * eq33_e699_d_n9);
        let eq33_e700_d_n10: f64 = (p.p14 * eq33_e699_d_n10);
        let eq33_e700_d_n11: f64 = (p.p14 * eq33_e699_d_n11);
        let eq33_e700_d_n12: f64 = (p.p14 * eq33_e699_d_n12);
        let eq33_e700_d_n13: f64 = (p.p14 * eq33_e699_d_n13);
        let eq33_e700_d_b0: f64 = (p.p14 * eq33_e699_d_b0);
        let eq33_e700_d_b1: f64 = (p.p14 * eq33_e699_d_b1);
        let eq33_e700_d_b2: f64 = (p.p14 * eq33_e699_d_b2);
        let eq33_e700_d_b3: f64 = (p.p14 * eq33_e699_d_b3);
        let eq33_e700_q: f64 = (p.p14 * eq33_e699_q);
        let eq33_reactive_node_derivatives: [f64; 14] = [eq33_e700_d_n0, eq33_e700_d_n1, eq33_e700_d_n2, eq33_e700_d_n3, eq33_e700_d_n4, eq33_e700_d_n5, eq33_e700_d_n6, eq33_e700_d_n7, eq33_e700_d_n8, eq33_e700_d_n9, eq33_e700_d_n10, eq33_e700_d_n11, eq33_e700_d_n12, eq33_e700_d_n13];
        let eq33_reactive_branch_derivatives: [f64; 4] = [eq33_e700_d_b0, eq33_e700_d_b1, eq33_e700_d_b2, eq33_e700_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq33_reactive_node_derivatives,
            branches,
            &eq33_reactive_branch_derivatives,
            multiplicity,
        );
        let eq34_e703_q: f64 = s.v[376];
        let eq34_e705_q: f64 = s.v[382];
        let eq34_e706: f64 = (s.v[376] + s.v[382]);
        let eq34_e706_d_n0: f64 = (s.dn[376][0] + s.dn[382][0]);
        let eq34_e706_d_n1: f64 = (s.dn[376][1] + s.dn[382][1]);
        let eq34_e706_d_n2: f64 = (s.dn[376][2] + s.dn[382][2]);
        let eq34_e706_d_n3: f64 = (s.dn[376][3] + s.dn[382][3]);
        let eq34_e706_d_n4: f64 = (s.dn[376][4] + s.dn[382][4]);
        let eq34_e706_d_n5: f64 = (s.dn[376][5] + s.dn[382][5]);
        let eq34_e706_d_n6: f64 = (s.dn[376][6] + s.dn[382][6]);
        let eq34_e706_d_n7: f64 = (s.dn[376][7] + s.dn[382][7]);
        let eq34_e706_d_n8: f64 = (s.dn[376][8] + s.dn[382][8]);
        let eq34_e706_d_n9: f64 = (s.dn[376][9] + s.dn[382][9]);
        let eq34_e706_d_n10: f64 = (s.dn[376][10] + s.dn[382][10]);
        let eq34_e706_d_n11: f64 = (s.dn[376][11] + s.dn[382][11]);
        let eq34_e706_d_n12: f64 = (s.dn[376][12] + s.dn[382][12]);
        let eq34_e706_d_n13: f64 = (s.dn[376][13] + s.dn[382][13]);
        let eq34_e706_d_b0: f64 = (s.db[376][0] + s.db[382][0]);
        let eq34_e706_d_b1: f64 = (s.db[376][1] + s.db[382][1]);
        let eq34_e706_d_b2: f64 = (s.db[376][2] + s.db[382][2]);
        let eq34_e706_d_b3: f64 = (s.db[376][3] + s.db[382][3]);
        let eq34_e706_q: f64 = (eq34_e703_q + eq34_e705_q);
        let eq34_e707: f64 = (p.p14 * eq34_e706);
        let eq34_e707_d_n0: f64 = (p.p14 * eq34_e706_d_n0);
        let eq34_e707_d_n1: f64 = (p.p14 * eq34_e706_d_n1);
        let eq34_e707_d_n2: f64 = (p.p14 * eq34_e706_d_n2);
        let eq34_e707_d_n3: f64 = (p.p14 * eq34_e706_d_n3);
        let eq34_e707_d_n4: f64 = (p.p14 * eq34_e706_d_n4);
        let eq34_e707_d_n5: f64 = (p.p14 * eq34_e706_d_n5);
        let eq34_e707_d_n6: f64 = (p.p14 * eq34_e706_d_n6);
        let eq34_e707_d_n7: f64 = (p.p14 * eq34_e706_d_n7);
        let eq34_e707_d_n8: f64 = (p.p14 * eq34_e706_d_n8);
        let eq34_e707_d_n9: f64 = (p.p14 * eq34_e706_d_n9);
        let eq34_e707_d_n10: f64 = (p.p14 * eq34_e706_d_n10);
        let eq34_e707_d_n11: f64 = (p.p14 * eq34_e706_d_n11);
        let eq34_e707_d_n12: f64 = (p.p14 * eq34_e706_d_n12);
        let eq34_e707_d_n13: f64 = (p.p14 * eq34_e706_d_n13);
        let eq34_e707_d_b0: f64 = (p.p14 * eq34_e706_d_b0);
        let eq34_e707_d_b1: f64 = (p.p14 * eq34_e706_d_b1);
        let eq34_e707_d_b2: f64 = (p.p14 * eq34_e706_d_b2);
        let eq34_e707_d_b3: f64 = (p.p14 * eq34_e706_d_b3);
        let eq34_e707_q: f64 = (p.p14 * eq34_e706_q);
        let eq34_reactive_node_derivatives: [f64; 14] = [eq34_e707_d_n0, eq34_e707_d_n1, eq34_e707_d_n2, eq34_e707_d_n3, eq34_e707_d_n4, eq34_e707_d_n5, eq34_e707_d_n6, eq34_e707_d_n7, eq34_e707_d_n8, eq34_e707_d_n9, eq34_e707_d_n10, eq34_e707_d_n11, eq34_e707_d_n12, eq34_e707_d_n13];
        let eq34_reactive_branch_derivatives: [f64; 4] = [eq34_e707_d_b0, eq34_e707_d_b1, eq34_e707_d_b2, eq34_e707_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq34_reactive_node_derivatives,
            branches,
            &eq34_reactive_branch_derivatives,
            multiplicity,
        );
        let eq35_e710_q: f64 = s.v[374];
        let eq35_e711: f64 = (p.p14 * s.v[374]);
        let eq35_e711_d_n0: f64 = (p.p14 * s.dn[374][0]);
        let eq35_e711_d_n1: f64 = (p.p14 * s.dn[374][1]);
        let eq35_e711_d_n2: f64 = (p.p14 * s.dn[374][2]);
        let eq35_e711_d_n3: f64 = (p.p14 * s.dn[374][3]);
        let eq35_e711_d_n4: f64 = (p.p14 * s.dn[374][4]);
        let eq35_e711_d_n5: f64 = (p.p14 * s.dn[374][5]);
        let eq35_e711_d_n6: f64 = (p.p14 * s.dn[374][6]);
        let eq35_e711_d_n7: f64 = (p.p14 * s.dn[374][7]);
        let eq35_e711_d_n8: f64 = (p.p14 * s.dn[374][8]);
        let eq35_e711_d_n9: f64 = (p.p14 * s.dn[374][9]);
        let eq35_e711_d_n10: f64 = (p.p14 * s.dn[374][10]);
        let eq35_e711_d_n11: f64 = (p.p14 * s.dn[374][11]);
        let eq35_e711_d_n12: f64 = (p.p14 * s.dn[374][12]);
        let eq35_e711_d_n13: f64 = (p.p14 * s.dn[374][13]);
        let eq35_e711_d_b0: f64 = (p.p14 * s.db[374][0]);
        let eq35_e711_d_b1: f64 = (p.p14 * s.db[374][1]);
        let eq35_e711_d_b2: f64 = (p.p14 * s.db[374][2]);
        let eq35_e711_d_b3: f64 = (p.p14 * s.db[374][3]);
        let eq35_e711_q: f64 = (p.p14 * eq35_e710_q);
        let eq35_reactive_node_derivatives: [f64; 14] = [eq35_e711_d_n0, eq35_e711_d_n1, eq35_e711_d_n2, eq35_e711_d_n3, eq35_e711_d_n4, eq35_e711_d_n5, eq35_e711_d_n6, eq35_e711_d_n7, eq35_e711_d_n8, eq35_e711_d_n9, eq35_e711_d_n10, eq35_e711_d_n11, eq35_e711_d_n12, eq35_e711_d_n13];
        let eq35_reactive_branch_derivatives: [f64; 4] = [eq35_e711_d_b0, eq35_e711_d_b1, eq35_e711_d_b2, eq35_e711_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let eq36_e714: f64 = (-s.v[1773]);
        let eq36_e716: f64 = (eq36_e714 * p.p32);
        let eq36_e716_d_n0: f64 = ((-s.dn[1773][0]) * p.p32);
        let eq36_e716_d_n1: f64 = ((-s.dn[1773][1]) * p.p32);
        let eq36_e716_d_n2: f64 = ((-s.dn[1773][2]) * p.p32);
        let eq36_e716_d_n3: f64 = ((-s.dn[1773][3]) * p.p32);
        let eq36_e716_d_n4: f64 = ((-s.dn[1773][4]) * p.p32);
        let eq36_e716_d_n5: f64 = ((-s.dn[1773][5]) * p.p32);
        let eq36_e716_d_n6: f64 = ((-s.dn[1773][6]) * p.p32);
        let eq36_e716_d_n7: f64 = ((-s.dn[1773][7]) * p.p32);
        let eq36_e716_d_n8: f64 = ((-s.dn[1773][8]) * p.p32);
        let eq36_e716_d_n9: f64 = ((-s.dn[1773][9]) * p.p32);
        let eq36_e716_d_n10: f64 = ((-s.dn[1773][10]) * p.p32);
        let eq36_e716_d_n11: f64 = ((-s.dn[1773][11]) * p.p32);
        let eq36_e716_d_n12: f64 = ((-s.dn[1773][12]) * p.p32);
        let eq36_e716_d_n13: f64 = ((-s.dn[1773][13]) * p.p32);
        let eq36_e716_d_b0: f64 = ((-s.db[1773][0]) * p.p32);
        let eq36_e716_d_b1: f64 = ((-s.db[1773][1]) * p.p32);
        let eq36_e716_d_b2: f64 = ((-s.db[1773][2]) * p.p32);
        let eq36_e716_d_b3: f64 = ((-s.db[1773][3]) * p.p32);
        let eq36_e718: f64 = (eq36_e716 * s.v[13]);
        let eq36_e718_d_n0: f64 = ((eq36_e716_d_n0 * s.v[13]) + (eq36_e716 * s.dn[13][0]));
        let eq36_e718_d_n1: f64 = ((eq36_e716_d_n1 * s.v[13]) + (eq36_e716 * s.dn[13][1]));
        let eq36_e718_d_n2: f64 = ((eq36_e716_d_n2 * s.v[13]) + (eq36_e716 * s.dn[13][2]));
        let eq36_e718_d_n3: f64 = ((eq36_e716_d_n3 * s.v[13]) + (eq36_e716 * s.dn[13][3]));
        let eq36_e718_d_n4: f64 = ((eq36_e716_d_n4 * s.v[13]) + (eq36_e716 * s.dn[13][4]));
        let eq36_e718_d_n5: f64 = ((eq36_e716_d_n5 * s.v[13]) + (eq36_e716 * s.dn[13][5]));
        let eq36_e718_d_n6: f64 = ((eq36_e716_d_n6 * s.v[13]) + (eq36_e716 * s.dn[13][6]));
        let eq36_e718_d_n7: f64 = ((eq36_e716_d_n7 * s.v[13]) + (eq36_e716 * s.dn[13][7]));
        let eq36_e718_d_n8: f64 = ((eq36_e716_d_n8 * s.v[13]) + (eq36_e716 * s.dn[13][8]));
        let eq36_e718_d_n9: f64 = ((eq36_e716_d_n9 * s.v[13]) + (eq36_e716 * s.dn[13][9]));
        let eq36_e718_d_n10: f64 = ((eq36_e716_d_n10 * s.v[13]) + (eq36_e716 * s.dn[13][10]));
        let eq36_e718_d_n11: f64 = ((eq36_e716_d_n11 * s.v[13]) + (eq36_e716 * s.dn[13][11]));
        let eq36_e718_d_n12: f64 = ((eq36_e716_d_n12 * s.v[13]) + (eq36_e716 * s.dn[13][12]));
        let eq36_e718_d_n13: f64 = ((eq36_e716_d_n13 * s.v[13]) + (eq36_e716 * s.dn[13][13]));
        let eq36_e718_d_b0: f64 = ((eq36_e716_d_b0 * s.v[13]) + (eq36_e716 * s.db[13][0]));
        let eq36_e718_d_b1: f64 = ((eq36_e716_d_b1 * s.v[13]) + (eq36_e716 * s.db[13][1]));
        let eq36_e718_d_b2: f64 = ((eq36_e716_d_b2 * s.v[13]) + (eq36_e716 * s.db[13][2]));
        let eq36_e718_d_b3: f64 = ((eq36_e716_d_b3 * s.v[13]) + (eq36_e716 * s.db[13][3]));
        let eq36_e722: f64 = (s.v[182]).sqrt();
        let eq36_e722_d_n0: f64 = (s.dn[182][0] / (2.0 * eq36_e722));
        let eq36_e722_d_n1: f64 = (s.dn[182][1] / (2.0 * eq36_e722));
        let eq36_e722_d_n2: f64 = (s.dn[182][2] / (2.0 * eq36_e722));
        let eq36_e722_d_n3: f64 = (s.dn[182][3] / (2.0 * eq36_e722));
        let eq36_e722_d_n4: f64 = (s.dn[182][4] / (2.0 * eq36_e722));
        let eq36_e722_d_n5: f64 = (s.dn[182][5] / (2.0 * eq36_e722));
        let eq36_e722_d_n6: f64 = (s.dn[182][6] / (2.0 * eq36_e722));
        let eq36_e722_d_n7: f64 = (s.dn[182][7] / (2.0 * eq36_e722));
        let eq36_e722_d_n8: f64 = (s.dn[182][8] / (2.0 * eq36_e722));
        let eq36_e722_d_n9: f64 = (s.dn[182][9] / (2.0 * eq36_e722));
        let eq36_e722_d_n10: f64 = (s.dn[182][10] / (2.0 * eq36_e722));
        let eq36_e722_d_n11: f64 = (s.dn[182][11] / (2.0 * eq36_e722));
        let eq36_e722_d_n12: f64 = (s.dn[182][12] / (2.0 * eq36_e722));
        let eq36_e722_d_n13: f64 = (s.dn[182][13] / (2.0 * eq36_e722));
        let eq36_e722_d_b0: f64 = (s.db[182][0] / (2.0 * eq36_e722));
        let eq36_e722_d_b1: f64 = (s.db[182][1] / (2.0 * eq36_e722));
        let eq36_e722_d_b2: f64 = (s.db[182][2] / (2.0 * eq36_e722));
        let eq36_e722_d_b3: f64 = (s.db[182][3] / (2.0 * eq36_e722));
        let eq36_e723: f64 = ((nv11 - nv13) / eq36_e722);
        let eq36_e723_d_n0: f64 = (-(((nv11 - nv13) * eq36_e722_d_n0) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n1: f64 = (-(((nv11 - nv13) * eq36_e722_d_n1) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n2: f64 = (-(((nv11 - nv13) * eq36_e722_d_n2) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n3: f64 = (-(((nv11 - nv13) * eq36_e722_d_n3) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n4: f64 = (-(((nv11 - nv13) * eq36_e722_d_n4) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n5: f64 = (-(((nv11 - nv13) * eq36_e722_d_n5) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n6: f64 = (-(((nv11 - nv13) * eq36_e722_d_n6) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n7: f64 = (-(((nv11 - nv13) * eq36_e722_d_n7) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n8: f64 = (-(((nv11 - nv13) * eq36_e722_d_n8) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n9: f64 = (-(((nv11 - nv13) * eq36_e722_d_n9) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n10: f64 = (-(((nv11 - nv13) * eq36_e722_d_n10) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n11: f64 = ((eq36_e722 - ((nv11 - nv13) * eq36_e722_d_n11)) / (eq36_e722 * eq36_e722));
        let eq36_e723_d_n12: f64 = (-(((nv11 - nv13) * eq36_e722_d_n12) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_n13: f64 = (((-eq36_e722) - ((nv11 - nv13) * eq36_e722_d_n13)) / (eq36_e722 * eq36_e722));
        let eq36_e723_d_b0: f64 = (-(((nv11 - nv13) * eq36_e722_d_b0) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_b1: f64 = (-(((nv11 - nv13) * eq36_e722_d_b1) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_b2: f64 = (-(((nv11 - nv13) * eq36_e722_d_b2) / (eq36_e722 * eq36_e722)));
        let eq36_e723_d_b3: f64 = (-(((nv11 - nv13) * eq36_e722_d_b3) / (eq36_e722 * eq36_e722)));
        let eq36_e724: f64 = ((nv10 - nv13) + eq36_e723);
        let eq36_e724_d_n10: f64 = (1.0 + eq36_e723_d_n10);
        let eq36_e724_d_n13: f64 = (-1.0 + eq36_e723_d_n13);
        let eq36_e725: f64 = (eq36_e718 * eq36_e724);
        let eq36_e725_d_n0: f64 = ((eq36_e718_d_n0 * eq36_e724) + (eq36_e718 * eq36_e723_d_n0));
        let eq36_e725_d_n1: f64 = ((eq36_e718_d_n1 * eq36_e724) + (eq36_e718 * eq36_e723_d_n1));
        let eq36_e725_d_n2: f64 = ((eq36_e718_d_n2 * eq36_e724) + (eq36_e718 * eq36_e723_d_n2));
        let eq36_e725_d_n3: f64 = ((eq36_e718_d_n3 * eq36_e724) + (eq36_e718 * eq36_e723_d_n3));
        let eq36_e725_d_n4: f64 = ((eq36_e718_d_n4 * eq36_e724) + (eq36_e718 * eq36_e723_d_n4));
        let eq36_e725_d_n5: f64 = ((eq36_e718_d_n5 * eq36_e724) + (eq36_e718 * eq36_e723_d_n5));
        let eq36_e725_d_n6: f64 = ((eq36_e718_d_n6 * eq36_e724) + (eq36_e718 * eq36_e723_d_n6));
        let eq36_e725_d_n7: f64 = ((eq36_e718_d_n7 * eq36_e724) + (eq36_e718 * eq36_e723_d_n7));
        let eq36_e725_d_n8: f64 = ((eq36_e718_d_n8 * eq36_e724) + (eq36_e718 * eq36_e723_d_n8));
        let eq36_e725_d_n9: f64 = ((eq36_e718_d_n9 * eq36_e724) + (eq36_e718 * eq36_e723_d_n9));
        let eq36_e725_d_n10: f64 = ((eq36_e718_d_n10 * eq36_e724) + (eq36_e718 * eq36_e724_d_n10));
        let eq36_e725_d_n11: f64 = ((eq36_e718_d_n11 * eq36_e724) + (eq36_e718 * eq36_e723_d_n11));
        let eq36_e725_d_n12: f64 = ((eq36_e718_d_n12 * eq36_e724) + (eq36_e718 * eq36_e723_d_n12));
        let eq36_e725_d_n13: f64 = ((eq36_e718_d_n13 * eq36_e724) + (eq36_e718 * eq36_e724_d_n13));
        let eq36_e725_d_b0: f64 = ((eq36_e718_d_b0 * eq36_e724) + (eq36_e718 * eq36_e723_d_b0));
        let eq36_e725_d_b1: f64 = ((eq36_e718_d_b1 * eq36_e724) + (eq36_e718 * eq36_e723_d_b1));
        let eq36_e725_d_b2: f64 = ((eq36_e718_d_b2 * eq36_e724) + (eq36_e718 * eq36_e723_d_b2));
        let eq36_e725_d_b3: f64 = ((eq36_e718_d_b3 * eq36_e724) + (eq36_e718 * eq36_e723_d_b3));
        let eq36_e727_q: f64 = s.v[362];
        let eq36_e728: f64 = (eq36_e725 - s.v[362]);
        let eq36_e728_d_n0: f64 = (eq36_e725_d_n0 - s.dn[362][0]);
        let eq36_e728_d_n1: f64 = (eq36_e725_d_n1 - s.dn[362][1]);
        let eq36_e728_d_n2: f64 = (eq36_e725_d_n2 - s.dn[362][2]);
        let eq36_e728_d_n3: f64 = (eq36_e725_d_n3 - s.dn[362][3]);
        let eq36_e728_d_n4: f64 = (eq36_e725_d_n4 - s.dn[362][4]);
        let eq36_e728_d_n5: f64 = (eq36_e725_d_n5 - s.dn[362][5]);
        let eq36_e728_d_n6: f64 = (eq36_e725_d_n6 - s.dn[362][6]);
        let eq36_e728_d_n7: f64 = (eq36_e725_d_n7 - s.dn[362][7]);
        let eq36_e728_d_n8: f64 = (eq36_e725_d_n8 - s.dn[362][8]);
        let eq36_e728_d_n9: f64 = (eq36_e725_d_n9 - s.dn[362][9]);
        let eq36_e728_d_n10: f64 = (eq36_e725_d_n10 - s.dn[362][10]);
        let eq36_e728_d_n11: f64 = (eq36_e725_d_n11 - s.dn[362][11]);
        let eq36_e728_d_n12: f64 = (eq36_e725_d_n12 - s.dn[362][12]);
        let eq36_e728_d_n13: f64 = (eq36_e725_d_n13 - s.dn[362][13]);
        let eq36_e728_d_b0: f64 = (eq36_e725_d_b0 - s.db[362][0]);
        let eq36_e728_d_b1: f64 = (eq36_e725_d_b1 - s.db[362][1]);
        let eq36_e728_d_b2: f64 = (eq36_e725_d_b2 - s.db[362][2]);
        let eq36_e728_d_b3: f64 = (eq36_e725_d_b3 - s.db[362][3]);
        let eq36_e728_q: f64 = (-eq36_e727_q);
        let eq36_e730_q: f64 = s.v[370];
        let eq36_e731: f64 = (eq36_e728 + s.v[370]);
        let eq36_e731_d_n0: f64 = (eq36_e728_d_n0 + s.dn[370][0]);
        let eq36_e731_d_n1: f64 = (eq36_e728_d_n1 + s.dn[370][1]);
        let eq36_e731_d_n2: f64 = (eq36_e728_d_n2 + s.dn[370][2]);
        let eq36_e731_d_n3: f64 = (eq36_e728_d_n3 + s.dn[370][3]);
        let eq36_e731_d_n4: f64 = (eq36_e728_d_n4 + s.dn[370][4]);
        let eq36_e731_d_n5: f64 = (eq36_e728_d_n5 + s.dn[370][5]);
        let eq36_e731_d_n6: f64 = (eq36_e728_d_n6 + s.dn[370][6]);
        let eq36_e731_d_n7: f64 = (eq36_e728_d_n7 + s.dn[370][7]);
        let eq36_e731_d_n8: f64 = (eq36_e728_d_n8 + s.dn[370][8]);
        let eq36_e731_d_n9: f64 = (eq36_e728_d_n9 + s.dn[370][9]);
        let eq36_e731_d_n10: f64 = (eq36_e728_d_n10 + s.dn[370][10]);
        let eq36_e731_d_n11: f64 = (eq36_e728_d_n11 + s.dn[370][11]);
        let eq36_e731_d_n12: f64 = (eq36_e728_d_n12 + s.dn[370][12]);
        let eq36_e731_d_n13: f64 = (eq36_e728_d_n13 + s.dn[370][13]);
        let eq36_e731_d_b0: f64 = (eq36_e728_d_b0 + s.db[370][0]);
        let eq36_e731_d_b1: f64 = (eq36_e728_d_b1 + s.db[370][1]);
        let eq36_e731_d_b2: f64 = (eq36_e728_d_b2 + s.db[370][2]);
        let eq36_e731_d_b3: f64 = (eq36_e728_d_b3 + s.db[370][3]);
        let eq36_e731_q: f64 = (eq36_e728_q + eq36_e730_q);
        let eq36_e731_q_d_n0: f64 = ((-s.dn[362][0]) + s.dn[370][0]);
        let eq36_e731_q_d_n1: f64 = ((-s.dn[362][1]) + s.dn[370][1]);
        let eq36_e731_q_d_n2: f64 = ((-s.dn[362][2]) + s.dn[370][2]);
        let eq36_e731_q_d_n3: f64 = ((-s.dn[362][3]) + s.dn[370][3]);
        let eq36_e731_q_d_n4: f64 = ((-s.dn[362][4]) + s.dn[370][4]);
        let eq36_e731_q_d_n5: f64 = ((-s.dn[362][5]) + s.dn[370][5]);
        let eq36_e731_q_d_n6: f64 = ((-s.dn[362][6]) + s.dn[370][6]);
        let eq36_e731_q_d_n7: f64 = ((-s.dn[362][7]) + s.dn[370][7]);
        let eq36_e731_q_d_n8: f64 = ((-s.dn[362][8]) + s.dn[370][8]);
        let eq36_e731_q_d_n9: f64 = ((-s.dn[362][9]) + s.dn[370][9]);
        let eq36_e731_q_d_n10: f64 = ((-s.dn[362][10]) + s.dn[370][10]);
        let eq36_e731_q_d_n11: f64 = ((-s.dn[362][11]) + s.dn[370][11]);
        let eq36_e731_q_d_n12: f64 = ((-s.dn[362][12]) + s.dn[370][12]);
        let eq36_e731_q_d_n13: f64 = ((-s.dn[362][13]) + s.dn[370][13]);
        let eq36_e731_q_d_b0: f64 = ((-s.db[362][0]) + s.db[370][0]);
        let eq36_e731_q_d_b1: f64 = ((-s.db[362][1]) + s.db[370][1]);
        let eq36_e731_q_d_b2: f64 = ((-s.db[362][2]) + s.db[370][2]);
        let eq36_e731_q_d_b3: f64 = ((-s.db[362][3]) + s.db[370][3]);
        let eq36_e733_q: f64 = s.v[372];
        let eq36_e734: f64 = (eq36_e731 + s.v[372]);
        let eq36_e734_d_n0: f64 = (eq36_e731_d_n0 + s.dn[372][0]);
        let eq36_e734_d_n1: f64 = (eq36_e731_d_n1 + s.dn[372][1]);
        let eq36_e734_d_n2: f64 = (eq36_e731_d_n2 + s.dn[372][2]);
        let eq36_e734_d_n3: f64 = (eq36_e731_d_n3 + s.dn[372][3]);
        let eq36_e734_d_n4: f64 = (eq36_e731_d_n4 + s.dn[372][4]);
        let eq36_e734_d_n5: f64 = (eq36_e731_d_n5 + s.dn[372][5]);
        let eq36_e734_d_n6: f64 = (eq36_e731_d_n6 + s.dn[372][6]);
        let eq36_e734_d_n7: f64 = (eq36_e731_d_n7 + s.dn[372][7]);
        let eq36_e734_d_n8: f64 = (eq36_e731_d_n8 + s.dn[372][8]);
        let eq36_e734_d_n9: f64 = (eq36_e731_d_n9 + s.dn[372][9]);
        let eq36_e734_d_n10: f64 = (eq36_e731_d_n10 + s.dn[372][10]);
        let eq36_e734_d_n11: f64 = (eq36_e731_d_n11 + s.dn[372][11]);
        let eq36_e734_d_n12: f64 = (eq36_e731_d_n12 + s.dn[372][12]);
        let eq36_e734_d_n13: f64 = (eq36_e731_d_n13 + s.dn[372][13]);
        let eq36_e734_d_b0: f64 = (eq36_e731_d_b0 + s.db[372][0]);
        let eq36_e734_d_b1: f64 = (eq36_e731_d_b1 + s.db[372][1]);
        let eq36_e734_d_b2: f64 = (eq36_e731_d_b2 + s.db[372][2]);
        let eq36_e734_d_b3: f64 = (eq36_e731_d_b3 + s.db[372][3]);
        let eq36_e734_q: f64 = (eq36_e731_q + eq36_e733_q);
        let eq36_e734_q_d_n0: f64 = (eq36_e731_q_d_n0 + s.dn[372][0]);
        let eq36_e734_q_d_n1: f64 = (eq36_e731_q_d_n1 + s.dn[372][1]);
        let eq36_e734_q_d_n2: f64 = (eq36_e731_q_d_n2 + s.dn[372][2]);
        let eq36_e734_q_d_n3: f64 = (eq36_e731_q_d_n3 + s.dn[372][3]);
        let eq36_e734_q_d_n4: f64 = (eq36_e731_q_d_n4 + s.dn[372][4]);
        let eq36_e734_q_d_n5: f64 = (eq36_e731_q_d_n5 + s.dn[372][5]);
        let eq36_e734_q_d_n6: f64 = (eq36_e731_q_d_n6 + s.dn[372][6]);
        let eq36_e734_q_d_n7: f64 = (eq36_e731_q_d_n7 + s.dn[372][7]);
        let eq36_e734_q_d_n8: f64 = (eq36_e731_q_d_n8 + s.dn[372][8]);
        let eq36_e734_q_d_n9: f64 = (eq36_e731_q_d_n9 + s.dn[372][9]);
        let eq36_e734_q_d_n10: f64 = (eq36_e731_q_d_n10 + s.dn[372][10]);
        let eq36_e734_q_d_n11: f64 = (eq36_e731_q_d_n11 + s.dn[372][11]);
        let eq36_e734_q_d_n12: f64 = (eq36_e731_q_d_n12 + s.dn[372][12]);
        let eq36_e734_q_d_n13: f64 = (eq36_e731_q_d_n13 + s.dn[372][13]);
        let eq36_e734_q_d_b0: f64 = (eq36_e731_q_d_b0 + s.db[372][0]);
        let eq36_e734_q_d_b1: f64 = (eq36_e731_q_d_b1 + s.db[372][1]);
        let eq36_e734_q_d_b2: f64 = (eq36_e731_q_d_b2 + s.db[372][2]);
        let eq36_e734_q_d_b3: f64 = (eq36_e731_q_d_b3 + s.db[372][3]);
        let eq36_e736_q: f64 = s.v[379];
        let eq36_e737: f64 = (eq36_e734 + s.v[379]);
        let eq36_e737_d_n0: f64 = (eq36_e734_d_n0 + s.dn[379][0]);
        let eq36_e737_d_n1: f64 = (eq36_e734_d_n1 + s.dn[379][1]);
        let eq36_e737_d_n2: f64 = (eq36_e734_d_n2 + s.dn[379][2]);
        let eq36_e737_d_n3: f64 = (eq36_e734_d_n3 + s.dn[379][3]);
        let eq36_e737_d_n4: f64 = (eq36_e734_d_n4 + s.dn[379][4]);
        let eq36_e737_d_n5: f64 = (eq36_e734_d_n5 + s.dn[379][5]);
        let eq36_e737_d_n6: f64 = (eq36_e734_d_n6 + s.dn[379][6]);
        let eq36_e737_d_n7: f64 = (eq36_e734_d_n7 + s.dn[379][7]);
        let eq36_e737_d_n8: f64 = (eq36_e734_d_n8 + s.dn[379][8]);
        let eq36_e737_d_n9: f64 = (eq36_e734_d_n9 + s.dn[379][9]);
        let eq36_e737_d_n10: f64 = (eq36_e734_d_n10 + s.dn[379][10]);
        let eq36_e737_d_n11: f64 = (eq36_e734_d_n11 + s.dn[379][11]);
        let eq36_e737_d_n12: f64 = (eq36_e734_d_n12 + s.dn[379][12]);
        let eq36_e737_d_n13: f64 = (eq36_e734_d_n13 + s.dn[379][13]);
        let eq36_e737_d_b0: f64 = (eq36_e734_d_b0 + s.db[379][0]);
        let eq36_e737_d_b1: f64 = (eq36_e734_d_b1 + s.db[379][1]);
        let eq36_e737_d_b2: f64 = (eq36_e734_d_b2 + s.db[379][2]);
        let eq36_e737_d_b3: f64 = (eq36_e734_d_b3 + s.db[379][3]);
        let eq36_e737_q: f64 = (eq36_e734_q + eq36_e736_q);
        let eq36_e737_q_d_n0: f64 = (eq36_e734_q_d_n0 + s.dn[379][0]);
        let eq36_e737_q_d_n1: f64 = (eq36_e734_q_d_n1 + s.dn[379][1]);
        let eq36_e737_q_d_n2: f64 = (eq36_e734_q_d_n2 + s.dn[379][2]);
        let eq36_e737_q_d_n3: f64 = (eq36_e734_q_d_n3 + s.dn[379][3]);
        let eq36_e737_q_d_n4: f64 = (eq36_e734_q_d_n4 + s.dn[379][4]);
        let eq36_e737_q_d_n5: f64 = (eq36_e734_q_d_n5 + s.dn[379][5]);
        let eq36_e737_q_d_n6: f64 = (eq36_e734_q_d_n6 + s.dn[379][6]);
        let eq36_e737_q_d_n7: f64 = (eq36_e734_q_d_n7 + s.dn[379][7]);
        let eq36_e737_q_d_n8: f64 = (eq36_e734_q_d_n8 + s.dn[379][8]);
        let eq36_e737_q_d_n9: f64 = (eq36_e734_q_d_n9 + s.dn[379][9]);
        let eq36_e737_q_d_n10: f64 = (eq36_e734_q_d_n10 + s.dn[379][10]);
        let eq36_e737_q_d_n11: f64 = (eq36_e734_q_d_n11 + s.dn[379][11]);
        let eq36_e737_q_d_n12: f64 = (eq36_e734_q_d_n12 + s.dn[379][12]);
        let eq36_e737_q_d_n13: f64 = (eq36_e734_q_d_n13 + s.dn[379][13]);
        let eq36_e737_q_d_b0: f64 = (eq36_e734_q_d_b0 + s.db[379][0]);
        let eq36_e737_q_d_b1: f64 = (eq36_e734_q_d_b1 + s.db[379][1]);
        let eq36_e737_q_d_b2: f64 = (eq36_e734_q_d_b2 + s.db[379][2]);
        let eq36_e737_q_d_b3: f64 = (eq36_e734_q_d_b3 + s.db[379][3]);
        let eq36_e738: f64 = (p.p14 * eq36_e737);
        let eq36_e738_d_n0: f64 = (p.p14 * eq36_e737_d_n0);
        let eq36_e738_d_n1: f64 = (p.p14 * eq36_e737_d_n1);
        let eq36_e738_d_n2: f64 = (p.p14 * eq36_e737_d_n2);
        let eq36_e738_d_n3: f64 = (p.p14 * eq36_e737_d_n3);
        let eq36_e738_d_n4: f64 = (p.p14 * eq36_e737_d_n4);
        let eq36_e738_d_n5: f64 = (p.p14 * eq36_e737_d_n5);
        let eq36_e738_d_n6: f64 = (p.p14 * eq36_e737_d_n6);
        let eq36_e738_d_n7: f64 = (p.p14 * eq36_e737_d_n7);
        let eq36_e738_d_n8: f64 = (p.p14 * eq36_e737_d_n8);
        let eq36_e738_d_n9: f64 = (p.p14 * eq36_e737_d_n9);
        let eq36_e738_d_n10: f64 = (p.p14 * eq36_e737_d_n10);
        let eq36_e738_d_n11: f64 = (p.p14 * eq36_e737_d_n11);
        let eq36_e738_d_n12: f64 = (p.p14 * eq36_e737_d_n12);
        let eq36_e738_d_n13: f64 = (p.p14 * eq36_e737_d_n13);
        let eq36_e738_d_b0: f64 = (p.p14 * eq36_e737_d_b0);
        let eq36_e738_d_b1: f64 = (p.p14 * eq36_e737_d_b1);
        let eq36_e738_d_b2: f64 = (p.p14 * eq36_e737_d_b2);
        let eq36_e738_d_b3: f64 = (p.p14 * eq36_e737_d_b3);
        let eq36_e738_q: f64 = (p.p14 * eq36_e737_q);
        let eq36_e738_q_d_n0: f64 = (p.p14 * eq36_e737_q_d_n0);
        let eq36_e738_q_d_n1: f64 = (p.p14 * eq36_e737_q_d_n1);
        let eq36_e738_q_d_n2: f64 = (p.p14 * eq36_e737_q_d_n2);
        let eq36_e738_q_d_n3: f64 = (p.p14 * eq36_e737_q_d_n3);
        let eq36_e738_q_d_n4: f64 = (p.p14 * eq36_e737_q_d_n4);
        let eq36_e738_q_d_n5: f64 = (p.p14 * eq36_e737_q_d_n5);
        let eq36_e738_q_d_n6: f64 = (p.p14 * eq36_e737_q_d_n6);
        let eq36_e738_q_d_n7: f64 = (p.p14 * eq36_e737_q_d_n7);
        let eq36_e738_q_d_n8: f64 = (p.p14 * eq36_e737_q_d_n8);
        let eq36_e738_q_d_n9: f64 = (p.p14 * eq36_e737_q_d_n9);
        let eq36_e738_q_d_n10: f64 = (p.p14 * eq36_e737_q_d_n10);
        let eq36_e738_q_d_n11: f64 = (p.p14 * eq36_e737_q_d_n11);
        let eq36_e738_q_d_n12: f64 = (p.p14 * eq36_e737_q_d_n12);
        let eq36_e738_q_d_n13: f64 = (p.p14 * eq36_e737_q_d_n13);
        let eq36_e738_q_d_b0: f64 = (p.p14 * eq36_e737_q_d_b0);
        let eq36_e738_q_d_b1: f64 = (p.p14 * eq36_e737_q_d_b1);
        let eq36_e738_q_d_b2: f64 = (p.p14 * eq36_e737_q_d_b2);
        let eq36_e738_q_d_b3: f64 = (p.p14 * eq36_e737_q_d_b3);
        let eq36_reactive_node_derivatives: [f64; 14] = [eq36_e738_q_d_n0, eq36_e738_q_d_n1, eq36_e738_q_d_n2, eq36_e738_q_d_n3, eq36_e738_q_d_n4, eq36_e738_q_d_n5, eq36_e738_q_d_n6, eq36_e738_q_d_n7, eq36_e738_q_d_n8, eq36_e738_q_d_n9, eq36_e738_q_d_n10, eq36_e738_q_d_n11, eq36_e738_q_d_n12, eq36_e738_q_d_n13];
        let eq36_reactive_branch_derivatives: [f64; 4] = [eq36_e738_q_d_b0, eq36_e738_q_d_b1, eq36_e738_q_d_b2, eq36_e738_q_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            nodes,
            &eq36_reactive_node_derivatives,
            branches,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let eq37_e741: f64 = (-s.v[1773]);
        let eq37_e743: f64 = (eq37_e741 * p.p31);
        let eq37_e743_d_n0: f64 = ((-s.dn[1773][0]) * p.p31);
        let eq37_e743_d_n1: f64 = ((-s.dn[1773][1]) * p.p31);
        let eq37_e743_d_n2: f64 = ((-s.dn[1773][2]) * p.p31);
        let eq37_e743_d_n3: f64 = ((-s.dn[1773][3]) * p.p31);
        let eq37_e743_d_n4: f64 = ((-s.dn[1773][4]) * p.p31);
        let eq37_e743_d_n5: f64 = ((-s.dn[1773][5]) * p.p31);
        let eq37_e743_d_n6: f64 = ((-s.dn[1773][6]) * p.p31);
        let eq37_e743_d_n7: f64 = ((-s.dn[1773][7]) * p.p31);
        let eq37_e743_d_n8: f64 = ((-s.dn[1773][8]) * p.p31);
        let eq37_e743_d_n9: f64 = ((-s.dn[1773][9]) * p.p31);
        let eq37_e743_d_n10: f64 = ((-s.dn[1773][10]) * p.p31);
        let eq37_e743_d_n11: f64 = ((-s.dn[1773][11]) * p.p31);
        let eq37_e743_d_n12: f64 = ((-s.dn[1773][12]) * p.p31);
        let eq37_e743_d_n13: f64 = ((-s.dn[1773][13]) * p.p31);
        let eq37_e743_d_b0: f64 = ((-s.db[1773][0]) * p.p31);
        let eq37_e743_d_b1: f64 = ((-s.db[1773][1]) * p.p31);
        let eq37_e743_d_b2: f64 = ((-s.db[1773][2]) * p.p31);
        let eq37_e743_d_b3: f64 = ((-s.db[1773][3]) * p.p31);
        let eq37_e745: f64 = (eq37_e743 * s.v[13]);
        let eq37_e745_d_n0: f64 = ((eq37_e743_d_n0 * s.v[13]) + (eq37_e743 * s.dn[13][0]));
        let eq37_e745_d_n1: f64 = ((eq37_e743_d_n1 * s.v[13]) + (eq37_e743 * s.dn[13][1]));
        let eq37_e745_d_n2: f64 = ((eq37_e743_d_n2 * s.v[13]) + (eq37_e743 * s.dn[13][2]));
        let eq37_e745_d_n3: f64 = ((eq37_e743_d_n3 * s.v[13]) + (eq37_e743 * s.dn[13][3]));
        let eq37_e745_d_n4: f64 = ((eq37_e743_d_n4 * s.v[13]) + (eq37_e743 * s.dn[13][4]));
        let eq37_e745_d_n5: f64 = ((eq37_e743_d_n5 * s.v[13]) + (eq37_e743 * s.dn[13][5]));
        let eq37_e745_d_n6: f64 = ((eq37_e743_d_n6 * s.v[13]) + (eq37_e743 * s.dn[13][6]));
        let eq37_e745_d_n7: f64 = ((eq37_e743_d_n7 * s.v[13]) + (eq37_e743 * s.dn[13][7]));
        let eq37_e745_d_n8: f64 = ((eq37_e743_d_n8 * s.v[13]) + (eq37_e743 * s.dn[13][8]));
        let eq37_e745_d_n9: f64 = ((eq37_e743_d_n9 * s.v[13]) + (eq37_e743 * s.dn[13][9]));
        let eq37_e745_d_n10: f64 = ((eq37_e743_d_n10 * s.v[13]) + (eq37_e743 * s.dn[13][10]));
        let eq37_e745_d_n11: f64 = ((eq37_e743_d_n11 * s.v[13]) + (eq37_e743 * s.dn[13][11]));
        let eq37_e745_d_n12: f64 = ((eq37_e743_d_n12 * s.v[13]) + (eq37_e743 * s.dn[13][12]));
        let eq37_e745_d_n13: f64 = ((eq37_e743_d_n13 * s.v[13]) + (eq37_e743 * s.dn[13][13]));
        let eq37_e745_d_b0: f64 = ((eq37_e743_d_b0 * s.v[13]) + (eq37_e743 * s.db[13][0]));
        let eq37_e745_d_b1: f64 = ((eq37_e743_d_b1 * s.v[13]) + (eq37_e743 * s.db[13][1]));
        let eq37_e745_d_b2: f64 = ((eq37_e743_d_b2 * s.v[13]) + (eq37_e743 * s.db[13][2]));
        let eq37_e745_d_b3: f64 = ((eq37_e743_d_b3 * s.v[13]) + (eq37_e743 * s.db[13][3]));
        let eq37_e747: f64 = (eq37_e745 * (nv12 - nv13));
        let eq37_e747_d_n0: f64 = (eq37_e745_d_n0 * (nv12 - nv13));
        let eq37_e747_d_n1: f64 = (eq37_e745_d_n1 * (nv12 - nv13));
        let eq37_e747_d_n2: f64 = (eq37_e745_d_n2 * (nv12 - nv13));
        let eq37_e747_d_n3: f64 = (eq37_e745_d_n3 * (nv12 - nv13));
        let eq37_e747_d_n4: f64 = (eq37_e745_d_n4 * (nv12 - nv13));
        let eq37_e747_d_n5: f64 = (eq37_e745_d_n5 * (nv12 - nv13));
        let eq37_e747_d_n6: f64 = (eq37_e745_d_n6 * (nv12 - nv13));
        let eq37_e747_d_n7: f64 = (eq37_e745_d_n7 * (nv12 - nv13));
        let eq37_e747_d_n8: f64 = (eq37_e745_d_n8 * (nv12 - nv13));
        let eq37_e747_d_n9: f64 = (eq37_e745_d_n9 * (nv12 - nv13));
        let eq37_e747_d_n10: f64 = (eq37_e745_d_n10 * (nv12 - nv13));
        let eq37_e747_d_n11: f64 = (eq37_e745_d_n11 * (nv12 - nv13));
        let eq37_e747_d_n12: f64 = ((eq37_e745_d_n12 * (nv12 - nv13)) + eq37_e745);
        let eq37_e747_d_n13: f64 = ((eq37_e745_d_n13 * (nv12 - nv13)) + (-eq37_e745));
        let eq37_e747_d_b0: f64 = (eq37_e745_d_b0 * (nv12 - nv13));
        let eq37_e747_d_b1: f64 = (eq37_e745_d_b1 * (nv12 - nv13));
        let eq37_e747_d_b2: f64 = (eq37_e745_d_b2 * (nv12 - nv13));
        let eq37_e747_d_b3: f64 = (eq37_e745_d_b3 * (nv12 - nv13));
        let eq37_e749_q: f64 = s.v[375];
        let eq37_e750: f64 = (eq37_e747 + s.v[375]);
        let eq37_e750_d_n0: f64 = (eq37_e747_d_n0 + s.dn[375][0]);
        let eq37_e750_d_n1: f64 = (eq37_e747_d_n1 + s.dn[375][1]);
        let eq37_e750_d_n2: f64 = (eq37_e747_d_n2 + s.dn[375][2]);
        let eq37_e750_d_n3: f64 = (eq37_e747_d_n3 + s.dn[375][3]);
        let eq37_e750_d_n4: f64 = (eq37_e747_d_n4 + s.dn[375][4]);
        let eq37_e750_d_n5: f64 = (eq37_e747_d_n5 + s.dn[375][5]);
        let eq37_e750_d_n6: f64 = (eq37_e747_d_n6 + s.dn[375][6]);
        let eq37_e750_d_n7: f64 = (eq37_e747_d_n7 + s.dn[375][7]);
        let eq37_e750_d_n8: f64 = (eq37_e747_d_n8 + s.dn[375][8]);
        let eq37_e750_d_n9: f64 = (eq37_e747_d_n9 + s.dn[375][9]);
        let eq37_e750_d_n10: f64 = (eq37_e747_d_n10 + s.dn[375][10]);
        let eq37_e750_d_n11: f64 = (eq37_e747_d_n11 + s.dn[375][11]);
        let eq37_e750_d_n12: f64 = (eq37_e747_d_n12 + s.dn[375][12]);
        let eq37_e750_d_n13: f64 = (eq37_e747_d_n13 + s.dn[375][13]);
        let eq37_e750_d_b0: f64 = (eq37_e747_d_b0 + s.db[375][0]);
        let eq37_e750_d_b1: f64 = (eq37_e747_d_b1 + s.db[375][1]);
        let eq37_e750_d_b2: f64 = (eq37_e747_d_b2 + s.db[375][2]);
        let eq37_e750_d_b3: f64 = (eq37_e747_d_b3 + s.db[375][3]);
        let eq37_e750_q: f64 = eq37_e749_q;
        let eq37_e751: f64 = (p.p14 * eq37_e750);
        let eq37_e751_d_n0: f64 = (p.p14 * eq37_e750_d_n0);
        let eq37_e751_d_n1: f64 = (p.p14 * eq37_e750_d_n1);
        let eq37_e751_d_n2: f64 = (p.p14 * eq37_e750_d_n2);
        let eq37_e751_d_n3: f64 = (p.p14 * eq37_e750_d_n3);
        let eq37_e751_d_n4: f64 = (p.p14 * eq37_e750_d_n4);
        let eq37_e751_d_n5: f64 = (p.p14 * eq37_e750_d_n5);
        let eq37_e751_d_n6: f64 = (p.p14 * eq37_e750_d_n6);
        let eq37_e751_d_n7: f64 = (p.p14 * eq37_e750_d_n7);
        let eq37_e751_d_n8: f64 = (p.p14 * eq37_e750_d_n8);
        let eq37_e751_d_n9: f64 = (p.p14 * eq37_e750_d_n9);
        let eq37_e751_d_n10: f64 = (p.p14 * eq37_e750_d_n10);
        let eq37_e751_d_n11: f64 = (p.p14 * eq37_e750_d_n11);
        let eq37_e751_d_n12: f64 = (p.p14 * eq37_e750_d_n12);
        let eq37_e751_d_n13: f64 = (p.p14 * eq37_e750_d_n13);
        let eq37_e751_d_b0: f64 = (p.p14 * eq37_e750_d_b0);
        let eq37_e751_d_b1: f64 = (p.p14 * eq37_e750_d_b1);
        let eq37_e751_d_b2: f64 = (p.p14 * eq37_e750_d_b2);
        let eq37_e751_d_b3: f64 = (p.p14 * eq37_e750_d_b3);
        let eq37_e751_q: f64 = (p.p14 * eq37_e750_q);
        let eq37_e751_q_d_n0: f64 = (p.p14 * s.dn[375][0]);
        let eq37_e751_q_d_n1: f64 = (p.p14 * s.dn[375][1]);
        let eq37_e751_q_d_n2: f64 = (p.p14 * s.dn[375][2]);
        let eq37_e751_q_d_n3: f64 = (p.p14 * s.dn[375][3]);
        let eq37_e751_q_d_n4: f64 = (p.p14 * s.dn[375][4]);
        let eq37_e751_q_d_n5: f64 = (p.p14 * s.dn[375][5]);
        let eq37_e751_q_d_n6: f64 = (p.p14 * s.dn[375][6]);
        let eq37_e751_q_d_n7: f64 = (p.p14 * s.dn[375][7]);
        let eq37_e751_q_d_n8: f64 = (p.p14 * s.dn[375][8]);
        let eq37_e751_q_d_n9: f64 = (p.p14 * s.dn[375][9]);
        let eq37_e751_q_d_n10: f64 = (p.p14 * s.dn[375][10]);
        let eq37_e751_q_d_n11: f64 = (p.p14 * s.dn[375][11]);
        let eq37_e751_q_d_n12: f64 = (p.p14 * s.dn[375][12]);
        let eq37_e751_q_d_n13: f64 = (p.p14 * s.dn[375][13]);
        let eq37_e751_q_d_b0: f64 = (p.p14 * s.db[375][0]);
        let eq37_e751_q_d_b1: f64 = (p.p14 * s.db[375][1]);
        let eq37_e751_q_d_b2: f64 = (p.p14 * s.db[375][2]);
        let eq37_e751_q_d_b3: f64 = (p.p14 * s.db[375][3]);
        let eq37_reactive_node_derivatives: [f64; 14] = [eq37_e751_q_d_n0, eq37_e751_q_d_n1, eq37_e751_q_d_n2, eq37_e751_q_d_n3, eq37_e751_q_d_n4, eq37_e751_q_d_n5, eq37_e751_q_d_n6, eq37_e751_q_d_n7, eq37_e751_q_d_n8, eq37_e751_q_d_n9, eq37_e751_q_d_n10, eq37_e751_q_d_n11, eq37_e751_q_d_n12, eq37_e751_q_d_n13];
        let eq37_reactive_branch_derivatives: [f64; 4] = [eq37_e751_q_d_b0, eq37_e751_q_d_b1, eq37_e751_q_d_b2, eq37_e751_q_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            nodes,
            &eq37_reactive_node_derivatives,
            branches,
            &eq37_reactive_branch_derivatives,
            multiplicity,
        );
        let eq38_e753_q: f64 = s.v[378];
        let eq38_reactive_node_derivatives: [f64; 14] = [s.dn[378][0], s.dn[378][1], s.dn[378][2], s.dn[378][3], s.dn[378][4], s.dn[378][5], s.dn[378][6], s.dn[378][7], s.dn[378][8], s.dn[378][9], s.dn[378][10], s.dn[378][11], s.dn[378][12], s.dn[378][13]];
        let eq38_reactive_branch_derivatives: [f64; 4] = [s.db[378][0], s.db[378][1], s.db[378][2], s.db[378][3]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq38_reactive_node_derivatives,
            branches,
            &eq38_reactive_branch_derivatives,
            multiplicity,
        );
        let eq41_e762: f64 = (s.v[1800] * (nv5 - 0.0));
        let eq41_e762_d_n0: f64 = (s.dn[1800][0] * (nv5 - 0.0));
        let eq41_e762_d_n1: f64 = (s.dn[1800][1] * (nv5 - 0.0));
        let eq41_e762_d_n2: f64 = (s.dn[1800][2] * (nv5 - 0.0));
        let eq41_e762_d_n3: f64 = (s.dn[1800][3] * (nv5 - 0.0));
        let eq41_e762_d_n4: f64 = (s.dn[1800][4] * (nv5 - 0.0));
        let eq41_e762_d_n5: f64 = ((s.dn[1800][5] * (nv5 - 0.0)) + s.v[1800]);
        let eq41_e762_d_n6: f64 = (s.dn[1800][6] * (nv5 - 0.0));
        let eq41_e762_d_n7: f64 = (s.dn[1800][7] * (nv5 - 0.0));
        let eq41_e762_d_n8: f64 = (s.dn[1800][8] * (nv5 - 0.0));
        let eq41_e762_d_n9: f64 = (s.dn[1800][9] * (nv5 - 0.0));
        let eq41_e762_d_n10: f64 = (s.dn[1800][10] * (nv5 - 0.0));
        let eq41_e762_d_n11: f64 = (s.dn[1800][11] * (nv5 - 0.0));
        let eq41_e762_d_n12: f64 = (s.dn[1800][12] * (nv5 - 0.0));
        let eq41_e762_d_n13: f64 = (s.dn[1800][13] * (nv5 - 0.0));
        let eq41_e762_d_b0: f64 = (s.db[1800][0] * (nv5 - 0.0));
        let eq41_e762_d_b1: f64 = (s.db[1800][1] * (nv5 - 0.0));
        let eq41_e762_d_b2: f64 = (s.db[1800][2] * (nv5 - 0.0));
        let eq41_e762_d_b3: f64 = (s.db[1800][3] * (nv5 - 0.0));
        let eq41_e763_q: f64 = eq41_e762;
        let eq41_reactive_node_derivatives: [f64; 14] = [eq41_e762_d_n0, eq41_e762_d_n1, eq41_e762_d_n2, eq41_e762_d_n3, eq41_e762_d_n4, eq41_e762_d_n5, eq41_e762_d_n6, eq41_e762_d_n7, eq41_e762_d_n8, eq41_e762_d_n9, eq41_e762_d_n10, eq41_e762_d_n11, eq41_e762_d_n12, eq41_e762_d_n13];
        let eq41_reactive_branch_derivatives: [f64; 4] = [eq41_e762_d_b0, eq41_e762_d_b1, eq41_e762_d_b2, eq41_e762_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq42_e765: f64 = (-s.v[1801]);
        let eq42_e767: f64 = (eq42_e765 * (nv5 - 0.0));
        let eq42_e767_d_n0: f64 = ((-s.dn[1801][0]) * (nv5 - 0.0));
        let eq42_e767_d_n1: f64 = ((-s.dn[1801][1]) * (nv5 - 0.0));
        let eq42_e767_d_n2: f64 = ((-s.dn[1801][2]) * (nv5 - 0.0));
        let eq42_e767_d_n3: f64 = ((-s.dn[1801][3]) * (nv5 - 0.0));
        let eq42_e767_d_n4: f64 = ((-s.dn[1801][4]) * (nv5 - 0.0));
        let eq42_e767_d_n5: f64 = (((-s.dn[1801][5]) * (nv5 - 0.0)) + eq42_e765);
        let eq42_e767_d_n6: f64 = ((-s.dn[1801][6]) * (nv5 - 0.0));
        let eq42_e767_d_n7: f64 = ((-s.dn[1801][7]) * (nv5 - 0.0));
        let eq42_e767_d_n8: f64 = ((-s.dn[1801][8]) * (nv5 - 0.0));
        let eq42_e767_d_n9: f64 = ((-s.dn[1801][9]) * (nv5 - 0.0));
        let eq42_e767_d_n10: f64 = ((-s.dn[1801][10]) * (nv5 - 0.0));
        let eq42_e767_d_n11: f64 = ((-s.dn[1801][11]) * (nv5 - 0.0));
        let eq42_e767_d_n12: f64 = ((-s.dn[1801][12]) * (nv5 - 0.0));
        let eq42_e767_d_n13: f64 = ((-s.dn[1801][13]) * (nv5 - 0.0));
        let eq42_e767_d_b0: f64 = ((-s.db[1801][0]) * (nv5 - 0.0));
        let eq42_e767_d_b1: f64 = ((-s.db[1801][1]) * (nv5 - 0.0));
        let eq42_e767_d_b2: f64 = ((-s.db[1801][2]) * (nv5 - 0.0));
        let eq42_e767_d_b3: f64 = ((-s.db[1801][3]) * (nv5 - 0.0));
        let eq42_e768_q: f64 = eq42_e767;
        let eq42_reactive_node_derivatives: [f64; 14] = [eq42_e767_d_n0, eq42_e767_d_n1, eq42_e767_d_n2, eq42_e767_d_n3, eq42_e767_d_n4, eq42_e767_d_n5, eq42_e767_d_n6, eq42_e767_d_n7, eq42_e767_d_n8, eq42_e767_d_n9, eq42_e767_d_n10, eq42_e767_d_n11, eq42_e767_d_n12, eq42_e767_d_n13];
        let eq42_reactive_branch_derivatives: [f64; 4] = [eq42_e767_d_b0, eq42_e767_d_b1, eq42_e767_d_b2, eq42_e767_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq43_e770: f64 = (-s.v[1802]);
        let eq43_e772: f64 = (eq43_e770 * (nv5 - 0.0));
        let eq43_e772_d_n0: f64 = ((-s.dn[1802][0]) * (nv5 - 0.0));
        let eq43_e772_d_n1: f64 = ((-s.dn[1802][1]) * (nv5 - 0.0));
        let eq43_e772_d_n2: f64 = ((-s.dn[1802][2]) * (nv5 - 0.0));
        let eq43_e772_d_n3: f64 = ((-s.dn[1802][3]) * (nv5 - 0.0));
        let eq43_e772_d_n4: f64 = ((-s.dn[1802][4]) * (nv5 - 0.0));
        let eq43_e772_d_n5: f64 = (((-s.dn[1802][5]) * (nv5 - 0.0)) + eq43_e770);
        let eq43_e772_d_n6: f64 = ((-s.dn[1802][6]) * (nv5 - 0.0));
        let eq43_e772_d_n7: f64 = ((-s.dn[1802][7]) * (nv5 - 0.0));
        let eq43_e772_d_n8: f64 = ((-s.dn[1802][8]) * (nv5 - 0.0));
        let eq43_e772_d_n9: f64 = ((-s.dn[1802][9]) * (nv5 - 0.0));
        let eq43_e772_d_n10: f64 = ((-s.dn[1802][10]) * (nv5 - 0.0));
        let eq43_e772_d_n11: f64 = ((-s.dn[1802][11]) * (nv5 - 0.0));
        let eq43_e772_d_n12: f64 = ((-s.dn[1802][12]) * (nv5 - 0.0));
        let eq43_e772_d_n13: f64 = ((-s.dn[1802][13]) * (nv5 - 0.0));
        let eq43_e772_d_b0: f64 = ((-s.db[1802][0]) * (nv5 - 0.0));
        let eq43_e772_d_b1: f64 = ((-s.db[1802][1]) * (nv5 - 0.0));
        let eq43_e772_d_b2: f64 = ((-s.db[1802][2]) * (nv5 - 0.0));
        let eq43_e772_d_b3: f64 = ((-s.db[1802][3]) * (nv5 - 0.0));
        let eq43_e773_q: f64 = eq43_e772;
        let eq43_reactive_node_derivatives: [f64; 14] = [eq43_e772_d_n0, eq43_e772_d_n1, eq43_e772_d_n2, eq43_e772_d_n3, eq43_e772_d_n4, eq43_e772_d_n5, eq43_e772_d_n6, eq43_e772_d_n7, eq43_e772_d_n8, eq43_e772_d_n9, eq43_e772_d_n10, eq43_e772_d_n11, eq43_e772_d_n12, eq43_e772_d_n13];
        let eq43_reactive_branch_derivatives: [f64; 4] = [eq43_e772_d_b0, eq43_e772_d_b1, eq43_e772_d_b2, eq43_e772_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq43_reactive_node_derivatives,
            branches,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );
    }
}

#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq31_e687: f64 = (s.v[1793] * (nv5 - 0.0));
        let eq31_e687_d_n0: f64 = (s.dn[1793][0] * (nv5 - 0.0));
        let eq31_e687_d_n1: f64 = (s.dn[1793][1] * (nv5 - 0.0));
        let eq31_e687_d_n2: f64 = (s.dn[1793][2] * (nv5 - 0.0));
        let eq31_e687_d_n3: f64 = (s.dn[1793][3] * (nv5 - 0.0));
        let eq31_e687_d_n4: f64 = (s.dn[1793][4] * (nv5 - 0.0));
        let eq31_e687_d_n5: f64 = ((s.dn[1793][5] * (nv5 - 0.0)) + s.v[1793]);
        let eq31_e687_d_n6: f64 = (s.dn[1793][6] * (nv5 - 0.0));
        let eq31_e687_d_n7: f64 = (s.dn[1793][7] * (nv5 - 0.0));
        let eq31_e687_d_n8: f64 = (s.dn[1793][8] * (nv5 - 0.0));
        let eq31_e687_d_n9: f64 = (s.dn[1793][9] * (nv5 - 0.0));
        let eq31_e687_d_b0: f64 = (s.db[1793][0] * (nv5 - 0.0));
        let eq31_e687_d_b1: f64 = (s.db[1793][1] * (nv5 - 0.0));
        let eq31_e687_d_b2: f64 = (s.db[1793][2] * (nv5 - 0.0));
        let eq31_e687_d_b3: f64 = (s.db[1793][3] * (nv5 - 0.0));
        let eq31_value: f64 = eq31_e687;
        let eq31_node_derivatives: [f64; 10] = [eq31_e687_d_n0, eq31_e687_d_n1, eq31_e687_d_n2, eq31_e687_d_n3, eq31_e687_d_n4, eq31_e687_d_n5, eq31_e687_d_n6, eq31_e687_d_n7, eq31_e687_d_n8, eq31_e687_d_n9];
        let eq31_branch_derivatives: [f64; 4] = [eq31_e687_d_b0, eq31_e687_d_b1, eq31_e687_d_b2, eq31_e687_d_b3];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let eq32_e690: f64 = (s.v[1790] * (nv5 - 0.0));
        let eq32_e690_d_n0: f64 = (s.dn[1790][0] * (nv5 - 0.0));
        let eq32_e690_d_n1: f64 = (s.dn[1790][1] * (nv5 - 0.0));
        let eq32_e690_d_n2: f64 = (s.dn[1790][2] * (nv5 - 0.0));
        let eq32_e690_d_n3: f64 = (s.dn[1790][3] * (nv5 - 0.0));
        let eq32_e690_d_n4: f64 = (s.dn[1790][4] * (nv5 - 0.0));
        let eq32_e690_d_n5: f64 = ((s.dn[1790][5] * (nv5 - 0.0)) + s.v[1790]);
        let eq32_e690_d_n6: f64 = (s.dn[1790][6] * (nv5 - 0.0));
        let eq32_e690_d_n7: f64 = (s.dn[1790][7] * (nv5 - 0.0));
        let eq32_e690_d_n8: f64 = (s.dn[1790][8] * (nv5 - 0.0));
        let eq32_e690_d_n9: f64 = (s.dn[1790][9] * (nv5 - 0.0));
        let eq32_e690_d_b0: f64 = (s.db[1790][0] * (nv5 - 0.0));
        let eq32_e690_d_b1: f64 = (s.db[1790][1] * (nv5 - 0.0));
        let eq32_e690_d_b2: f64 = (s.db[1790][2] * (nv5 - 0.0));
        let eq32_e690_d_b3: f64 = (s.db[1790][3] * (nv5 - 0.0));
        let eq32_e691: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, eq32_e690);
        let eq32_value: f64 = eq32_e691;
        let eq32_node_derivatives: [f64; 10] = [(eq32_e690_d_n0 * ddt_scale), (eq32_e690_d_n1 * ddt_scale), (eq32_e690_d_n2 * ddt_scale), (eq32_e690_d_n3 * ddt_scale), (eq32_e690_d_n4 * ddt_scale), (eq32_e690_d_n5 * ddt_scale), (eq32_e690_d_n6 * ddt_scale), (eq32_e690_d_n7 * ddt_scale), (eq32_e690_d_n8 * ddt_scale), (eq32_e690_d_n9 * ddt_scale)];
        let eq32_branch_derivatives: [f64; 4] = [(eq32_e690_d_b0 * ddt_scale), (eq32_e690_d_b1 * ddt_scale), (eq32_e690_d_b2 * ddt_scale), (eq32_e690_d_b3 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );
        let eq33_e693: f64 = (-s.v[1791]);
        let eq33_e695: f64 = (eq33_e693 * (nv5 - 0.0));
        let eq33_e695_d_n0: f64 = ((-s.dn[1791][0]) * (nv5 - 0.0));
        let eq33_e695_d_n1: f64 = ((-s.dn[1791][1]) * (nv5 - 0.0));
        let eq33_e695_d_n2: f64 = ((-s.dn[1791][2]) * (nv5 - 0.0));
        let eq33_e695_d_n3: f64 = ((-s.dn[1791][3]) * (nv5 - 0.0));
        let eq33_e695_d_n4: f64 = ((-s.dn[1791][4]) * (nv5 - 0.0));
        let eq33_e695_d_n5: f64 = (((-s.dn[1791][5]) * (nv5 - 0.0)) + eq33_e693);
        let eq33_e695_d_n6: f64 = ((-s.dn[1791][6]) * (nv5 - 0.0));
        let eq33_e695_d_n7: f64 = ((-s.dn[1791][7]) * (nv5 - 0.0));
        let eq33_e695_d_n8: f64 = ((-s.dn[1791][8]) * (nv5 - 0.0));
        let eq33_e695_d_n9: f64 = ((-s.dn[1791][9]) * (nv5 - 0.0));
        let eq33_e695_d_b0: f64 = ((-s.db[1791][0]) * (nv5 - 0.0));
        let eq33_e695_d_b1: f64 = ((-s.db[1791][1]) * (nv5 - 0.0));
        let eq33_e695_d_b2: f64 = ((-s.db[1791][2]) * (nv5 - 0.0));
        let eq33_e695_d_b3: f64 = ((-s.db[1791][3]) * (nv5 - 0.0));
        let eq33_e696: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, eq33_e695);
        let eq33_value: f64 = eq33_e696;
        let eq33_node_derivatives: [f64; 10] = [(eq33_e695_d_n0 * ddt_scale), (eq33_e695_d_n1 * ddt_scale), (eq33_e695_d_n2 * ddt_scale), (eq33_e695_d_n3 * ddt_scale), (eq33_e695_d_n4 * ddt_scale), (eq33_e695_d_n5 * ddt_scale), (eq33_e695_d_n6 * ddt_scale), (eq33_e695_d_n7 * ddt_scale), (eq33_e695_d_n8 * ddt_scale), (eq33_e695_d_n9 * ddt_scale)];
        let eq33_branch_derivatives: [f64; 4] = [(eq33_e695_d_b0 * ddt_scale), (eq33_e695_d_b1 * ddt_scale), (eq33_e695_d_b2 * ddt_scale), (eq33_e695_d_b3 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let eq34_e698: f64 = (-s.v[1792]);
        let eq34_e700: f64 = (eq34_e698 * (nv5 - 0.0));
        let eq34_e700_d_n0: f64 = ((-s.dn[1792][0]) * (nv5 - 0.0));
        let eq34_e700_d_n1: f64 = ((-s.dn[1792][1]) * (nv5 - 0.0));
        let eq34_e700_d_n2: f64 = ((-s.dn[1792][2]) * (nv5 - 0.0));
        let eq34_e700_d_n3: f64 = ((-s.dn[1792][3]) * (nv5 - 0.0));
        let eq34_e700_d_n4: f64 = ((-s.dn[1792][4]) * (nv5 - 0.0));
        let eq34_e700_d_n5: f64 = (((-s.dn[1792][5]) * (nv5 - 0.0)) + eq34_e698);
        let eq34_e700_d_n6: f64 = ((-s.dn[1792][6]) * (nv5 - 0.0));
        let eq34_e700_d_n7: f64 = ((-s.dn[1792][7]) * (nv5 - 0.0));
        let eq34_e700_d_n8: f64 = ((-s.dn[1792][8]) * (nv5 - 0.0));
        let eq34_e700_d_n9: f64 = ((-s.dn[1792][9]) * (nv5 - 0.0));
        let eq34_e700_d_b0: f64 = ((-s.db[1792][0]) * (nv5 - 0.0));
        let eq34_e700_d_b1: f64 = ((-s.db[1792][1]) * (nv5 - 0.0));
        let eq34_e700_d_b2: f64 = ((-s.db[1792][2]) * (nv5 - 0.0));
        let eq34_e700_d_b3: f64 = ((-s.db[1792][3]) * (nv5 - 0.0));
        let eq34_e701: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq34_e700);
        let eq34_value: f64 = eq34_e701;
        let eq34_node_derivatives: [f64; 10] = [(eq34_e700_d_n0 * ddt_scale), (eq34_e700_d_n1 * ddt_scale), (eq34_e700_d_n2 * ddt_scale), (eq34_e700_d_n3 * ddt_scale), (eq34_e700_d_n4 * ddt_scale), (eq34_e700_d_n5 * ddt_scale), (eq34_e700_d_n6 * ddt_scale), (eq34_e700_d_n7 * ddt_scale), (eq34_e700_d_n8 * ddt_scale), (eq34_e700_d_n9 * ddt_scale)];
        let eq34_branch_derivatives: [f64; 4] = [(eq34_e700_d_b0 * ddt_scale), (eq34_e700_d_b1 * ddt_scale), (eq34_e700_d_b2 * ddt_scale), (eq34_e700_d_b3 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq23_e631_q: f64 = s.v[358];
        let eq23_e633_q: f64 = s.v[373];
        let eq23_e634: f64 = (s.v[358] + s.v[373]);
        let eq23_e634_d_n0: f64 = (s.dn[358][0] + s.dn[373][0]);
        let eq23_e634_d_n1: f64 = (s.dn[358][1] + s.dn[373][1]);
        let eq23_e634_d_n2: f64 = (s.dn[358][2] + s.dn[373][2]);
        let eq23_e634_d_n3: f64 = (s.dn[358][3] + s.dn[373][3]);
        let eq23_e634_d_n4: f64 = (s.dn[358][4] + s.dn[373][4]);
        let eq23_e634_d_n5: f64 = (s.dn[358][5] + s.dn[373][5]);
        let eq23_e634_d_n6: f64 = (s.dn[358][6] + s.dn[373][6]);
        let eq23_e634_d_n7: f64 = (s.dn[358][7] + s.dn[373][7]);
        let eq23_e634_d_n8: f64 = (s.dn[358][8] + s.dn[373][8]);
        let eq23_e634_d_n9: f64 = (s.dn[358][9] + s.dn[373][9]);
        let eq23_e634_d_b0: f64 = (s.db[358][0] + s.db[373][0]);
        let eq23_e634_d_b1: f64 = (s.db[358][1] + s.db[373][1]);
        let eq23_e634_d_b2: f64 = (s.db[358][2] + s.db[373][2]);
        let eq23_e634_d_b3: f64 = (s.db[358][3] + s.db[373][3]);
        let eq23_e634_q: f64 = (eq23_e631_q + eq23_e633_q);
        let eq23_e636_q: f64 = s.v[377];
        let eq23_e637: f64 = (eq23_e634 + s.v[377]);
        let eq23_e637_d_n0: f64 = (eq23_e634_d_n0 + s.dn[377][0]);
        let eq23_e637_d_n1: f64 = (eq23_e634_d_n1 + s.dn[377][1]);
        let eq23_e637_d_n2: f64 = (eq23_e634_d_n2 + s.dn[377][2]);
        let eq23_e637_d_n3: f64 = (eq23_e634_d_n3 + s.dn[377][3]);
        let eq23_e637_d_n4: f64 = (eq23_e634_d_n4 + s.dn[377][4]);
        let eq23_e637_d_n5: f64 = (eq23_e634_d_n5 + s.dn[377][5]);
        let eq23_e637_d_n6: f64 = (eq23_e634_d_n6 + s.dn[377][6]);
        let eq23_e637_d_n7: f64 = (eq23_e634_d_n7 + s.dn[377][7]);
        let eq23_e637_d_n8: f64 = (eq23_e634_d_n8 + s.dn[377][8]);
        let eq23_e637_d_n9: f64 = (eq23_e634_d_n9 + s.dn[377][9]);
        let eq23_e637_d_b0: f64 = (eq23_e634_d_b0 + s.db[377][0]);
        let eq23_e637_d_b1: f64 = (eq23_e634_d_b1 + s.db[377][1]);
        let eq23_e637_d_b2: f64 = (eq23_e634_d_b2 + s.db[377][2]);
        let eq23_e637_d_b3: f64 = (eq23_e634_d_b3 + s.db[377][3]);
        let eq23_e637_q: f64 = (eq23_e634_q + eq23_e636_q);
        let eq23_e638: f64 = (p.p14 * eq23_e637);
        let eq23_e638_d_n0: f64 = (p.p14 * eq23_e637_d_n0);
        let eq23_e638_d_n1: f64 = (p.p14 * eq23_e637_d_n1);
        let eq23_e638_d_n2: f64 = (p.p14 * eq23_e637_d_n2);
        let eq23_e638_d_n3: f64 = (p.p14 * eq23_e637_d_n3);
        let eq23_e638_d_n4: f64 = (p.p14 * eq23_e637_d_n4);
        let eq23_e638_d_n5: f64 = (p.p14 * eq23_e637_d_n5);
        let eq23_e638_d_n6: f64 = (p.p14 * eq23_e637_d_n6);
        let eq23_e638_d_n7: f64 = (p.p14 * eq23_e637_d_n7);
        let eq23_e638_d_n8: f64 = (p.p14 * eq23_e637_d_n8);
        let eq23_e638_d_n9: f64 = (p.p14 * eq23_e637_d_n9);
        let eq23_e638_d_b0: f64 = (p.p14 * eq23_e637_d_b0);
        let eq23_e638_d_b1: f64 = (p.p14 * eq23_e637_d_b1);
        let eq23_e638_d_b2: f64 = (p.p14 * eq23_e637_d_b2);
        let eq23_e638_d_b3: f64 = (p.p14 * eq23_e637_d_b3);
        let eq23_e638_q: f64 = (p.p14 * eq23_e637_q);
        let eq23_reactive_node_derivatives: [f64; 10] = [eq23_e638_d_n0, eq23_e638_d_n1, eq23_e638_d_n2, eq23_e638_d_n3, eq23_e638_d_n4, eq23_e638_d_n5, eq23_e638_d_n6, eq23_e638_d_n7, eq23_e638_d_n8, eq23_e638_d_n9];
        let eq23_reactive_branch_derivatives: [f64; 4] = [eq23_e638_d_b0, eq23_e638_d_b1, eq23_e638_d_b2, eq23_e638_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
        let eq24_e641_q: f64 = s.v[367];
        let eq24_e643_q: f64 = s.v[369];
        let eq24_e644: f64 = (s.v[367] + s.v[369]);
        let eq24_e644_d_n0: f64 = (s.dn[367][0] + s.dn[369][0]);
        let eq24_e644_d_n1: f64 = (s.dn[367][1] + s.dn[369][1]);
        let eq24_e644_d_n2: f64 = (s.dn[367][2] + s.dn[369][2]);
        let eq24_e644_d_n3: f64 = (s.dn[367][3] + s.dn[369][3]);
        let eq24_e644_d_n4: f64 = (s.dn[367][4] + s.dn[369][4]);
        let eq24_e644_d_n5: f64 = (s.dn[367][5] + s.dn[369][5]);
        let eq24_e644_d_n6: f64 = (s.dn[367][6] + s.dn[369][6]);
        let eq24_e644_d_n7: f64 = (s.dn[367][7] + s.dn[369][7]);
        let eq24_e644_d_n8: f64 = (s.dn[367][8] + s.dn[369][8]);
        let eq24_e644_d_n9: f64 = (s.dn[367][9] + s.dn[369][9]);
        let eq24_e644_d_b0: f64 = (s.db[367][0] + s.db[369][0]);
        let eq24_e644_d_b1: f64 = (s.db[367][1] + s.db[369][1]);
        let eq24_e644_d_b2: f64 = (s.db[367][2] + s.db[369][2]);
        let eq24_e644_d_b3: f64 = (s.db[367][3] + s.db[369][3]);
        let eq24_e644_q: f64 = (eq24_e641_q + eq24_e643_q);
        let eq24_e646_q: f64 = s.v[376];
        let eq24_e647: f64 = (eq24_e644 + s.v[376]);
        let eq24_e647_d_n0: f64 = (eq24_e644_d_n0 + s.dn[376][0]);
        let eq24_e647_d_n1: f64 = (eq24_e644_d_n1 + s.dn[376][1]);
        let eq24_e647_d_n2: f64 = (eq24_e644_d_n2 + s.dn[376][2]);
        let eq24_e647_d_n3: f64 = (eq24_e644_d_n3 + s.dn[376][3]);
        let eq24_e647_d_n4: f64 = (eq24_e644_d_n4 + s.dn[376][4]);
        let eq24_e647_d_n5: f64 = (eq24_e644_d_n5 + s.dn[376][5]);
        let eq24_e647_d_n6: f64 = (eq24_e644_d_n6 + s.dn[376][6]);
        let eq24_e647_d_n7: f64 = (eq24_e644_d_n7 + s.dn[376][7]);
        let eq24_e647_d_n8: f64 = (eq24_e644_d_n8 + s.dn[376][8]);
        let eq24_e647_d_n9: f64 = (eq24_e644_d_n9 + s.dn[376][9]);
        let eq24_e647_d_b0: f64 = (eq24_e644_d_b0 + s.db[376][0]);
        let eq24_e647_d_b1: f64 = (eq24_e644_d_b1 + s.db[376][1]);
        let eq24_e647_d_b2: f64 = (eq24_e644_d_b2 + s.db[376][2]);
        let eq24_e647_d_b3: f64 = (eq24_e644_d_b3 + s.db[376][3]);
        let eq24_e647_q: f64 = (eq24_e644_q + eq24_e646_q);
        let eq24_e648: f64 = (p.p14 * eq24_e647);
        let eq24_e648_d_n0: f64 = (p.p14 * eq24_e647_d_n0);
        let eq24_e648_d_n1: f64 = (p.p14 * eq24_e647_d_n1);
        let eq24_e648_d_n2: f64 = (p.p14 * eq24_e647_d_n2);
        let eq24_e648_d_n3: f64 = (p.p14 * eq24_e647_d_n3);
        let eq24_e648_d_n4: f64 = (p.p14 * eq24_e647_d_n4);
        let eq24_e648_d_n5: f64 = (p.p14 * eq24_e647_d_n5);
        let eq24_e648_d_n6: f64 = (p.p14 * eq24_e647_d_n6);
        let eq24_e648_d_n7: f64 = (p.p14 * eq24_e647_d_n7);
        let eq24_e648_d_n8: f64 = (p.p14 * eq24_e647_d_n8);
        let eq24_e648_d_n9: f64 = (p.p14 * eq24_e647_d_n9);
        let eq24_e648_d_b0: f64 = (p.p14 * eq24_e647_d_b0);
        let eq24_e648_d_b1: f64 = (p.p14 * eq24_e647_d_b1);
        let eq24_e648_d_b2: f64 = (p.p14 * eq24_e647_d_b2);
        let eq24_e648_d_b3: f64 = (p.p14 * eq24_e647_d_b3);
        let eq24_e648_q: f64 = (p.p14 * eq24_e647_q);
        let eq24_reactive_node_derivatives: [f64; 10] = [eq24_e648_d_n0, eq24_e648_d_n1, eq24_e648_d_n2, eq24_e648_d_n3, eq24_e648_d_n4, eq24_e648_d_n5, eq24_e648_d_n6, eq24_e648_d_n7, eq24_e648_d_n8, eq24_e648_d_n9];
        let eq24_reactive_branch_derivatives: [f64; 4] = [eq24_e648_d_b0, eq24_e648_d_b1, eq24_e648_d_b2, eq24_e648_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq24_reactive_node_derivatives,
            branches,
            &eq24_reactive_branch_derivatives,
            multiplicity,
        );
        let eq25_e651_q: f64 = s.v[372];
        let eq25_e653_q: f64 = s.v[378];
        let eq25_e654: f64 = (s.v[372] + s.v[378]);
        let eq25_e654_d_n0: f64 = (s.dn[372][0] + s.dn[378][0]);
        let eq25_e654_d_n1: f64 = (s.dn[372][1] + s.dn[378][1]);
        let eq25_e654_d_n2: f64 = (s.dn[372][2] + s.dn[378][2]);
        let eq25_e654_d_n3: f64 = (s.dn[372][3] + s.dn[378][3]);
        let eq25_e654_d_n4: f64 = (s.dn[372][4] + s.dn[378][4]);
        let eq25_e654_d_n5: f64 = (s.dn[372][5] + s.dn[378][5]);
        let eq25_e654_d_n6: f64 = (s.dn[372][6] + s.dn[378][6]);
        let eq25_e654_d_n7: f64 = (s.dn[372][7] + s.dn[378][7]);
        let eq25_e654_d_n8: f64 = (s.dn[372][8] + s.dn[378][8]);
        let eq25_e654_d_n9: f64 = (s.dn[372][9] + s.dn[378][9]);
        let eq25_e654_d_b0: f64 = (s.db[372][0] + s.db[378][0]);
        let eq25_e654_d_b1: f64 = (s.db[372][1] + s.db[378][1]);
        let eq25_e654_d_b2: f64 = (s.db[372][2] + s.db[378][2]);
        let eq25_e654_d_b3: f64 = (s.db[372][3] + s.db[378][3]);
        let eq25_e654_q: f64 = (eq25_e651_q + eq25_e653_q);
        let eq25_e655: f64 = (p.p14 * eq25_e654);
        let eq25_e655_d_n0: f64 = (p.p14 * eq25_e654_d_n0);
        let eq25_e655_d_n1: f64 = (p.p14 * eq25_e654_d_n1);
        let eq25_e655_d_n2: f64 = (p.p14 * eq25_e654_d_n2);
        let eq25_e655_d_n3: f64 = (p.p14 * eq25_e654_d_n3);
        let eq25_e655_d_n4: f64 = (p.p14 * eq25_e654_d_n4);
        let eq25_e655_d_n5: f64 = (p.p14 * eq25_e654_d_n5);
        let eq25_e655_d_n6: f64 = (p.p14 * eq25_e654_d_n6);
        let eq25_e655_d_n7: f64 = (p.p14 * eq25_e654_d_n7);
        let eq25_e655_d_n8: f64 = (p.p14 * eq25_e654_d_n8);
        let eq25_e655_d_n9: f64 = (p.p14 * eq25_e654_d_n9);
        let eq25_e655_d_b0: f64 = (p.p14 * eq25_e654_d_b0);
        let eq25_e655_d_b1: f64 = (p.p14 * eq25_e654_d_b1);
        let eq25_e655_d_b2: f64 = (p.p14 * eq25_e654_d_b2);
        let eq25_e655_d_b3: f64 = (p.p14 * eq25_e654_d_b3);
        let eq25_e655_q: f64 = (p.p14 * eq25_e654_q);
        let eq25_reactive_node_derivatives: [f64; 10] = [eq25_e655_d_n0, eq25_e655_d_n1, eq25_e655_d_n2, eq25_e655_d_n3, eq25_e655_d_n4, eq25_e655_d_n5, eq25_e655_d_n6, eq25_e655_d_n7, eq25_e655_d_n8, eq25_e655_d_n9];
        let eq25_reactive_branch_derivatives: [f64; 4] = [eq25_e655_d_b0, eq25_e655_d_b1, eq25_e655_d_b2, eq25_e655_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq25_reactive_node_derivatives,
            branches,
            &eq25_reactive_branch_derivatives,
            multiplicity,
        );
        let eq26_e658_q: f64 = s.v[370];
        let eq26_e659: f64 = (p.p14 * s.v[370]);
        let eq26_e659_q: f64 = (p.p14 * eq26_e658_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &s.dn[370],
            branches,
            &s.db[370],
            (multiplicity) * (p.p14),
        );
        let eq27_e662_q: f64 = s.v[357];
        let eq27_e664_q: f64 = s.v[366];
        let eq27_e665: f64 = (s.v[357] + s.v[366]);
        let eq27_e665_d_n0: f64 = (s.dn[357][0] + s.dn[366][0]);
        let eq27_e665_d_n1: f64 = (s.dn[357][1] + s.dn[366][1]);
        let eq27_e665_d_n2: f64 = (s.dn[357][2] + s.dn[366][2]);
        let eq27_e665_d_n3: f64 = (s.dn[357][3] + s.dn[366][3]);
        let eq27_e665_d_n4: f64 = (s.dn[357][4] + s.dn[366][4]);
        let eq27_e665_d_n5: f64 = (s.dn[357][5] + s.dn[366][5]);
        let eq27_e665_d_n6: f64 = (s.dn[357][6] + s.dn[366][6]);
        let eq27_e665_d_n7: f64 = (s.dn[357][7] + s.dn[366][7]);
        let eq27_e665_d_n8: f64 = (s.dn[357][8] + s.dn[366][8]);
        let eq27_e665_d_n9: f64 = (s.dn[357][9] + s.dn[366][9]);
        let eq27_e665_d_b0: f64 = (s.db[357][0] + s.db[366][0]);
        let eq27_e665_d_b1: f64 = (s.db[357][1] + s.db[366][1]);
        let eq27_e665_d_b2: f64 = (s.db[357][2] + s.db[366][2]);
        let eq27_e665_d_b3: f64 = (s.db[357][3] + s.db[366][3]);
        let eq27_e665_q: f64 = (eq27_e662_q + eq27_e664_q);
        let eq27_e667_q: f64 = s.v[368];
        let eq27_e668: f64 = (eq27_e665 + s.v[368]);
        let eq27_e668_d_n0: f64 = (eq27_e665_d_n0 + s.dn[368][0]);
        let eq27_e668_d_n1: f64 = (eq27_e665_d_n1 + s.dn[368][1]);
        let eq27_e668_d_n2: f64 = (eq27_e665_d_n2 + s.dn[368][2]);
        let eq27_e668_d_n3: f64 = (eq27_e665_d_n3 + s.dn[368][3]);
        let eq27_e668_d_n4: f64 = (eq27_e665_d_n4 + s.dn[368][4]);
        let eq27_e668_d_n5: f64 = (eq27_e665_d_n5 + s.dn[368][5]);
        let eq27_e668_d_n6: f64 = (eq27_e665_d_n6 + s.dn[368][6]);
        let eq27_e668_d_n7: f64 = (eq27_e665_d_n7 + s.dn[368][7]);
        let eq27_e668_d_n8: f64 = (eq27_e665_d_n8 + s.dn[368][8]);
        let eq27_e668_d_n9: f64 = (eq27_e665_d_n9 + s.dn[368][9]);
        let eq27_e668_d_b0: f64 = (eq27_e665_d_b0 + s.db[368][0]);
        let eq27_e668_d_b1: f64 = (eq27_e665_d_b1 + s.db[368][1]);
        let eq27_e668_d_b2: f64 = (eq27_e665_d_b2 + s.db[368][2]);
        let eq27_e668_d_b3: f64 = (eq27_e665_d_b3 + s.db[368][3]);
        let eq27_e668_q: f64 = (eq27_e665_q + eq27_e667_q);
        let eq27_e670_q: f64 = s.v[375];
        let eq27_e671: f64 = (eq27_e668 + s.v[375]);
        let eq27_e671_d_n0: f64 = (eq27_e668_d_n0 + s.dn[375][0]);
        let eq27_e671_d_n1: f64 = (eq27_e668_d_n1 + s.dn[375][1]);
        let eq27_e671_d_n2: f64 = (eq27_e668_d_n2 + s.dn[375][2]);
        let eq27_e671_d_n3: f64 = (eq27_e668_d_n3 + s.dn[375][3]);
        let eq27_e671_d_n4: f64 = (eq27_e668_d_n4 + s.dn[375][4]);
        let eq27_e671_d_n5: f64 = (eq27_e668_d_n5 + s.dn[375][5]);
        let eq27_e671_d_n6: f64 = (eq27_e668_d_n6 + s.dn[375][6]);
        let eq27_e671_d_n7: f64 = (eq27_e668_d_n7 + s.dn[375][7]);
        let eq27_e671_d_n8: f64 = (eq27_e668_d_n8 + s.dn[375][8]);
        let eq27_e671_d_n9: f64 = (eq27_e668_d_n9 + s.dn[375][9]);
        let eq27_e671_d_b0: f64 = (eq27_e668_d_b0 + s.db[375][0]);
        let eq27_e671_d_b1: f64 = (eq27_e668_d_b1 + s.db[375][1]);
        let eq27_e671_d_b2: f64 = (eq27_e668_d_b2 + s.db[375][2]);
        let eq27_e671_d_b3: f64 = (eq27_e668_d_b3 + s.db[375][3]);
        let eq27_e671_q: f64 = (eq27_e668_q + eq27_e670_q);
        let eq27_e672: f64 = (p.p14 * eq27_e671);
        let eq27_e672_d_n0: f64 = (p.p14 * eq27_e671_d_n0);
        let eq27_e672_d_n1: f64 = (p.p14 * eq27_e671_d_n1);
        let eq27_e672_d_n2: f64 = (p.p14 * eq27_e671_d_n2);
        let eq27_e672_d_n3: f64 = (p.p14 * eq27_e671_d_n3);
        let eq27_e672_d_n4: f64 = (p.p14 * eq27_e671_d_n4);
        let eq27_e672_d_n5: f64 = (p.p14 * eq27_e671_d_n5);
        let eq27_e672_d_n6: f64 = (p.p14 * eq27_e671_d_n6);
        let eq27_e672_d_n7: f64 = (p.p14 * eq27_e671_d_n7);
        let eq27_e672_d_n8: f64 = (p.p14 * eq27_e671_d_n8);
        let eq27_e672_d_n9: f64 = (p.p14 * eq27_e671_d_n9);
        let eq27_e672_d_b0: f64 = (p.p14 * eq27_e671_d_b0);
        let eq27_e672_d_b1: f64 = (p.p14 * eq27_e671_d_b1);
        let eq27_e672_d_b2: f64 = (p.p14 * eq27_e671_d_b2);
        let eq27_e672_d_b3: f64 = (p.p14 * eq27_e671_d_b3);
        let eq27_e672_q: f64 = (p.p14 * eq27_e671_q);
        let eq27_reactive_node_derivatives: [f64; 10] = [eq27_e672_d_n0, eq27_e672_d_n1, eq27_e672_d_n2, eq27_e672_d_n3, eq27_e672_d_n4, eq27_e672_d_n5, eq27_e672_d_n6, eq27_e672_d_n7, eq27_e672_d_n8, eq27_e672_d_n9];
        let eq27_reactive_branch_derivatives: [f64; 4] = [eq27_e672_d_b0, eq27_e672_d_b1, eq27_e672_d_b2, eq27_e672_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            nodes,
            &eq27_reactive_node_derivatives,
            branches,
            &eq27_reactive_branch_derivatives,
            multiplicity,
        );
        let eq28_e675_q: f64 = s.v[359];
        let eq28_e677_q: f64 = s.v[371];
        let eq28_e678: f64 = (s.v[359] + s.v[371]);
        let eq28_e678_d_n0: f64 = (s.dn[359][0] + s.dn[371][0]);
        let eq28_e678_d_n1: f64 = (s.dn[359][1] + s.dn[371][1]);
        let eq28_e678_d_n2: f64 = (s.dn[359][2] + s.dn[371][2]);
        let eq28_e678_d_n3: f64 = (s.dn[359][3] + s.dn[371][3]);
        let eq28_e678_d_n4: f64 = (s.dn[359][4] + s.dn[371][4]);
        let eq28_e678_d_n5: f64 = (s.dn[359][5] + s.dn[371][5]);
        let eq28_e678_d_n6: f64 = (s.dn[359][6] + s.dn[371][6]);
        let eq28_e678_d_n7: f64 = (s.dn[359][7] + s.dn[371][7]);
        let eq28_e678_d_n8: f64 = (s.dn[359][8] + s.dn[371][8]);
        let eq28_e678_d_n9: f64 = (s.dn[359][9] + s.dn[371][9]);
        let eq28_e678_d_b0: f64 = (s.db[359][0] + s.db[371][0]);
        let eq28_e678_d_b1: f64 = (s.db[359][1] + s.db[371][1]);
        let eq28_e678_d_b2: f64 = (s.db[359][2] + s.db[371][2]);
        let eq28_e678_d_b3: f64 = (s.db[359][3] + s.db[371][3]);
        let eq28_e678_q: f64 = (eq28_e675_q + eq28_e677_q);
        let eq28_e679: f64 = (p.p14 * eq28_e678);
        let eq28_e679_d_n0: f64 = (p.p14 * eq28_e678_d_n0);
        let eq28_e679_d_n1: f64 = (p.p14 * eq28_e678_d_n1);
        let eq28_e679_d_n2: f64 = (p.p14 * eq28_e678_d_n2);
        let eq28_e679_d_n3: f64 = (p.p14 * eq28_e678_d_n3);
        let eq28_e679_d_n4: f64 = (p.p14 * eq28_e678_d_n4);
        let eq28_e679_d_n5: f64 = (p.p14 * eq28_e678_d_n5);
        let eq28_e679_d_n6: f64 = (p.p14 * eq28_e678_d_n6);
        let eq28_e679_d_n7: f64 = (p.p14 * eq28_e678_d_n7);
        let eq28_e679_d_n8: f64 = (p.p14 * eq28_e678_d_n8);
        let eq28_e679_d_n9: f64 = (p.p14 * eq28_e678_d_n9);
        let eq28_e679_d_b0: f64 = (p.p14 * eq28_e678_d_b0);
        let eq28_e679_d_b1: f64 = (p.p14 * eq28_e678_d_b1);
        let eq28_e679_d_b2: f64 = (p.p14 * eq28_e678_d_b2);
        let eq28_e679_d_b3: f64 = (p.p14 * eq28_e678_d_b3);
        let eq28_e679_q: f64 = (p.p14 * eq28_e678_q);
        let eq28_reactive_node_derivatives: [f64; 10] = [eq28_e679_d_n0, eq28_e679_d_n1, eq28_e679_d_n2, eq28_e679_d_n3, eq28_e679_d_n4, eq28_e679_d_n5, eq28_e679_d_n6, eq28_e679_d_n7, eq28_e679_d_n8, eq28_e679_d_n9];
        let eq28_reactive_branch_derivatives: [f64; 4] = [eq28_e679_d_b0, eq28_e679_d_b1, eq28_e679_d_b2, eq28_e679_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
        let eq29_e681_q: f64 = s.v[374];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &s.dn[374],
            branches,
            &s.db[374],
            multiplicity,
        );
        let eq32_e690: f64 = (s.v[1790] * (nv5 - 0.0));
        let eq32_e690_d_n0: f64 = (s.dn[1790][0] * (nv5 - 0.0));
        let eq32_e690_d_n1: f64 = (s.dn[1790][1] * (nv5 - 0.0));
        let eq32_e690_d_n2: f64 = (s.dn[1790][2] * (nv5 - 0.0));
        let eq32_e690_d_n3: f64 = (s.dn[1790][3] * (nv5 - 0.0));
        let eq32_e690_d_n4: f64 = (s.dn[1790][4] * (nv5 - 0.0));
        let eq32_e690_d_n5: f64 = ((s.dn[1790][5] * (nv5 - 0.0)) + s.v[1790]);
        let eq32_e690_d_n6: f64 = (s.dn[1790][6] * (nv5 - 0.0));
        let eq32_e690_d_n7: f64 = (s.dn[1790][7] * (nv5 - 0.0));
        let eq32_e690_d_n8: f64 = (s.dn[1790][8] * (nv5 - 0.0));
        let eq32_e690_d_n9: f64 = (s.dn[1790][9] * (nv5 - 0.0));
        let eq32_e690_d_b0: f64 = (s.db[1790][0] * (nv5 - 0.0));
        let eq32_e690_d_b1: f64 = (s.db[1790][1] * (nv5 - 0.0));
        let eq32_e690_d_b2: f64 = (s.db[1790][2] * (nv5 - 0.0));
        let eq32_e690_d_b3: f64 = (s.db[1790][3] * (nv5 - 0.0));
        let eq32_e691_q: f64 = eq32_e690;
        let eq32_reactive_node_derivatives: [f64; 10] = [eq32_e690_d_n0, eq32_e690_d_n1, eq32_e690_d_n2, eq32_e690_d_n3, eq32_e690_d_n4, eq32_e690_d_n5, eq32_e690_d_n6, eq32_e690_d_n7, eq32_e690_d_n8, eq32_e690_d_n9];
        let eq32_reactive_branch_derivatives: [f64; 4] = [eq32_e690_d_b0, eq32_e690_d_b1, eq32_e690_d_b2, eq32_e690_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq32_reactive_node_derivatives,
            branches,
            &eq32_reactive_branch_derivatives,
            multiplicity,
        );
        let eq33_e693: f64 = (-s.v[1791]);
        let eq33_e695: f64 = (eq33_e693 * (nv5 - 0.0));
        let eq33_e695_d_n0: f64 = ((-s.dn[1791][0]) * (nv5 - 0.0));
        let eq33_e695_d_n1: f64 = ((-s.dn[1791][1]) * (nv5 - 0.0));
        let eq33_e695_d_n2: f64 = ((-s.dn[1791][2]) * (nv5 - 0.0));
        let eq33_e695_d_n3: f64 = ((-s.dn[1791][3]) * (nv5 - 0.0));
        let eq33_e695_d_n4: f64 = ((-s.dn[1791][4]) * (nv5 - 0.0));
        let eq33_e695_d_n5: f64 = (((-s.dn[1791][5]) * (nv5 - 0.0)) + eq33_e693);
        let eq33_e695_d_n6: f64 = ((-s.dn[1791][6]) * (nv5 - 0.0));
        let eq33_e695_d_n7: f64 = ((-s.dn[1791][7]) * (nv5 - 0.0));
        let eq33_e695_d_n8: f64 = ((-s.dn[1791][8]) * (nv5 - 0.0));
        let eq33_e695_d_n9: f64 = ((-s.dn[1791][9]) * (nv5 - 0.0));
        let eq33_e695_d_b0: f64 = ((-s.db[1791][0]) * (nv5 - 0.0));
        let eq33_e695_d_b1: f64 = ((-s.db[1791][1]) * (nv5 - 0.0));
        let eq33_e695_d_b2: f64 = ((-s.db[1791][2]) * (nv5 - 0.0));
        let eq33_e695_d_b3: f64 = ((-s.db[1791][3]) * (nv5 - 0.0));
        let eq33_e696_q: f64 = eq33_e695;
        let eq33_reactive_node_derivatives: [f64; 10] = [eq33_e695_d_n0, eq33_e695_d_n1, eq33_e695_d_n2, eq33_e695_d_n3, eq33_e695_d_n4, eq33_e695_d_n5, eq33_e695_d_n6, eq33_e695_d_n7, eq33_e695_d_n8, eq33_e695_d_n9];
        let eq33_reactive_branch_derivatives: [f64; 4] = [eq33_e695_d_b0, eq33_e695_d_b1, eq33_e695_d_b2, eq33_e695_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            nodes,
            &eq33_reactive_node_derivatives,
            branches,
            &eq33_reactive_branch_derivatives,
            multiplicity,
        );
        let eq34_e698: f64 = (-s.v[1792]);
        let eq34_e700: f64 = (eq34_e698 * (nv5 - 0.0));
        let eq34_e700_d_n0: f64 = ((-s.dn[1792][0]) * (nv5 - 0.0));
        let eq34_e700_d_n1: f64 = ((-s.dn[1792][1]) * (nv5 - 0.0));
        let eq34_e700_d_n2: f64 = ((-s.dn[1792][2]) * (nv5 - 0.0));
        let eq34_e700_d_n3: f64 = ((-s.dn[1792][3]) * (nv5 - 0.0));
        let eq34_e700_d_n4: f64 = ((-s.dn[1792][4]) * (nv5 - 0.0));
        let eq34_e700_d_n5: f64 = (((-s.dn[1792][5]) * (nv5 - 0.0)) + eq34_e698);
        let eq34_e700_d_n6: f64 = ((-s.dn[1792][6]) * (nv5 - 0.0));
        let eq34_e700_d_n7: f64 = ((-s.dn[1792][7]) * (nv5 - 0.0));
        let eq34_e700_d_n8: f64 = ((-s.dn[1792][8]) * (nv5 - 0.0));
        let eq34_e700_d_n9: f64 = ((-s.dn[1792][9]) * (nv5 - 0.0));
        let eq34_e700_d_b0: f64 = ((-s.db[1792][0]) * (nv5 - 0.0));
        let eq34_e700_d_b1: f64 = ((-s.db[1792][1]) * (nv5 - 0.0));
        let eq34_e700_d_b2: f64 = ((-s.db[1792][2]) * (nv5 - 0.0));
        let eq34_e700_d_b3: f64 = ((-s.db[1792][3]) * (nv5 - 0.0));
        let eq34_e701_q: f64 = eq34_e700;
        let eq34_reactive_node_derivatives: [f64; 10] = [eq34_e700_d_n0, eq34_e700_d_n1, eq34_e700_d_n2, eq34_e700_d_n3, eq34_e700_d_n4, eq34_e700_d_n5, eq34_e700_d_n6, eq34_e700_d_n7, eq34_e700_d_n8, eq34_e700_d_n9];
        let eq34_reactive_branch_derivatives: [f64; 4] = [eq34_e700_d_b0, eq34_e700_d_b1, eq34_e700_d_b2, eq34_e700_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq34_reactive_node_derivatives,
            branches,
            &eq34_reactive_branch_derivatives,
            multiplicity,
        );
    }
}

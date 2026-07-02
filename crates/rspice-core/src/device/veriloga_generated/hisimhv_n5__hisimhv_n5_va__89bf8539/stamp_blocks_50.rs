#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_1(
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let eq28_e1167: f64 = (locals.var_qg + locals.var_qg_nqs);
        let eq28_e1168: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq28_e1167);
        let eq28_e1169: f64 = (p.p87 * eq28_e1168);
        let eq28_e1169_d_n0: f64 = (p.p87 * (locals.var_qg_dn0 * ddt_scale));
        let eq28_e1169_d_n2: f64 = (p.p87 * (locals.var_qg_dn2 * ddt_scale));
        let eq28_e1169_d_n4: f64 = (p.p87 * (locals.var_qg_dn4 * ddt_scale));
        let eq28_e1169_d_n5: f64 = (p.p87 * (locals.var_qg_dn5 * ddt_scale));
        let eq28_e1169_d_n6: f64 = (p.p87 * (locals.var_qg_dn6 * ddt_scale));
        let eq28_e1169_d_n7: f64 = (p.p87 * (locals.var_qg_dn7 * ddt_scale));
        let eq28_e1169_d_n8: f64 = (p.p87 * (locals.var_qg_dn8 * ddt_scale));
        let eq28_e1169_d_n9: f64 = (p.p87 * (locals.var_qg_dn9 * ddt_scale));
        let eq28_e1169_d_n10: f64 = (p.p87 * (locals.var_qg_dn10 * ddt_scale));
        let eq28_e1169_d_n11: f64 = (p.p87 * (locals.var_qg_dn11 * ddt_scale));
        let eq28_e1169_d_n12: f64 = (p.p87 * (locals.var_qg_nqs_dn12 * ddt_scale));
        let eq28_e1169_d_n13: f64 = (p.p87 * (locals.var_qg_nqs_dn13 * ddt_scale));
        let eq28_e1169_d_n14: f64 = (p.p87 * (locals.var_qg_dn14 * ddt_scale));
        let eq28_value: f64 = eq28_e1169;
        let eq28_node_derivative_indices: [usize; 13] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq28_node_derivatives: [f64; 13] = [eq28_e1169_d_n0, eq28_e1169_d_n2, eq28_e1169_d_n4, eq28_e1169_d_n5, eq28_e1169_d_n6, eq28_e1169_d_n7, eq28_e1169_d_n8, eq28_e1169_d_n9, eq28_e1169_d_n10, eq28_e1169_d_n11, eq28_e1169_d_n12, eq28_e1169_d_n13, eq28_e1169_d_n14];
        let eq28_branch_derivative_indices: [usize; 0] = [];
        let eq28_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq28_value),
            &eq28_node_derivative_indices,
            &eq28_node_derivatives,
            &eq28_branch_derivative_indices,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let eq29_e1173: f64 = (locals.var_qd + locals.var_qd_nqs);
        let eq29_e1173_d_n0: f64 = (locals.var_qd_dn0 + locals.var_qd_nqs_dn0);
        let eq29_e1173_d_n2: f64 = (locals.var_qd_dn2 + locals.var_qd_nqs_dn2);
        let eq29_e1173_d_n4: f64 = (locals.var_qd_dn4 + locals.var_qd_nqs_dn4);
        let eq29_e1173_d_n5: f64 = (locals.var_qd_dn5 + locals.var_qd_nqs_dn5);
        let eq29_e1173_d_n6: f64 = (locals.var_qd_dn6 + locals.var_qd_nqs_dn6);
        let eq29_e1173_d_n7: f64 = (locals.var_qd_dn7 + locals.var_qd_nqs_dn7);
        let eq29_e1173_d_n8: f64 = (locals.var_qd_dn8 + locals.var_qd_nqs_dn8);
        let eq29_e1173_d_n9: f64 = (locals.var_qd_dn9 + locals.var_qd_nqs_dn9);
        let eq29_e1173_d_n10: f64 = (locals.var_qd_dn10 + locals.var_qd_nqs_dn10);
        let eq29_e1173_d_n11: f64 = (locals.var_qd_dn11 + locals.var_qd_nqs_dn11);
        let eq29_e1173_d_n14: f64 = (locals.var_qd_dn14 + locals.var_qd_nqs_dn14);
        let eq29_e1174: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq29_e1173);
        let eq29_e1175: f64 = (p.p87 * eq29_e1174);
        let eq29_e1175_d_n0: f64 = (p.p87 * (eq29_e1173_d_n0 * ddt_scale));
        let eq29_e1175_d_n2: f64 = (p.p87 * (eq29_e1173_d_n2 * ddt_scale));
        let eq29_e1175_d_n4: f64 = (p.p87 * (eq29_e1173_d_n4 * ddt_scale));
        let eq29_e1175_d_n5: f64 = (p.p87 * (eq29_e1173_d_n5 * ddt_scale));
        let eq29_e1175_d_n6: f64 = (p.p87 * (eq29_e1173_d_n6 * ddt_scale));
        let eq29_e1175_d_n7: f64 = (p.p87 * (eq29_e1173_d_n7 * ddt_scale));
        let eq29_e1175_d_n8: f64 = (p.p87 * (eq29_e1173_d_n8 * ddt_scale));
        let eq29_e1175_d_n9: f64 = (p.p87 * (eq29_e1173_d_n9 * ddt_scale));
        let eq29_e1175_d_n10: f64 = (p.p87 * (eq29_e1173_d_n10 * ddt_scale));
        let eq29_e1175_d_n11: f64 = (p.p87 * (eq29_e1173_d_n11 * ddt_scale));
        let eq29_e1175_d_n12: f64 = (p.p87 * (locals.var_qd_nqs_dn12 * ddt_scale));
        let eq29_e1175_d_n14: f64 = (p.p87 * (eq29_e1173_d_n14 * ddt_scale));
        let eq29_value: f64 = eq29_e1175;
        let eq29_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14];
        let eq29_node_derivatives: [f64; 12] = [eq29_e1175_d_n0, eq29_e1175_d_n2, eq29_e1175_d_n4, eq29_e1175_d_n5, eq29_e1175_d_n6, eq29_e1175_d_n7, eq29_e1175_d_n8, eq29_e1175_d_n9, eq29_e1175_d_n10, eq29_e1175_d_n11, eq29_e1175_d_n12, eq29_e1175_d_n14];
        let eq29_branch_derivative_indices: [usize; 0] = [];
        let eq29_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq29_value),
            &eq29_node_derivative_indices,
            &eq29_node_derivatives,
            &eq29_branch_derivative_indices,
            &eq29_branch_derivatives,
            multiplicity,
        );
        let eq30_e1180: f64 = (locals.var_qg_nqs + locals.var_qd_nqs);
        let eq30_e1180_d_n12: f64 = (locals.var_qg_nqs_dn12 + locals.var_qd_nqs_dn12);
        let eq30_e1182: f64 = (eq30_e1180 + locals.var_qs_nqs);
        let eq30_e1182_d_n0: f64 = (locals.var_qd_nqs_dn0 + locals.var_qs_nqs_dn0);
        let eq30_e1182_d_n2: f64 = (locals.var_qd_nqs_dn2 + locals.var_qs_nqs_dn2);
        let eq30_e1182_d_n4: f64 = (locals.var_qd_nqs_dn4 + locals.var_qs_nqs_dn4);
        let eq30_e1182_d_n5: f64 = (locals.var_qd_nqs_dn5 + locals.var_qs_nqs_dn5);
        let eq30_e1182_d_n6: f64 = (locals.var_qd_nqs_dn6 + locals.var_qs_nqs_dn6);
        let eq30_e1182_d_n7: f64 = (locals.var_qd_nqs_dn7 + locals.var_qs_nqs_dn7);
        let eq30_e1182_d_n8: f64 = (locals.var_qd_nqs_dn8 + locals.var_qs_nqs_dn8);
        let eq30_e1182_d_n9: f64 = (locals.var_qd_nqs_dn9 + locals.var_qs_nqs_dn9);
        let eq30_e1182_d_n10: f64 = (locals.var_qd_nqs_dn10 + locals.var_qs_nqs_dn10);
        let eq30_e1182_d_n11: f64 = (locals.var_qd_nqs_dn11 + locals.var_qs_nqs_dn11);
        let eq30_e1182_d_n12: f64 = (eq30_e1180_d_n12 + locals.var_qs_nqs_dn12);
        let eq30_e1182_d_n14: f64 = (locals.var_qd_nqs_dn14 + locals.var_qs_nqs_dn14);
        let eq30_e1183: f64 = (locals.var_qb - eq30_e1182);
        let eq30_e1183_d_n0: f64 = (locals.var_qb_dn0 - eq30_e1182_d_n0);
        let eq30_e1183_d_n2: f64 = (locals.var_qb_dn2 - eq30_e1182_d_n2);
        let eq30_e1183_d_n4: f64 = (locals.var_qb_dn4 - eq30_e1182_d_n4);
        let eq30_e1183_d_n5: f64 = (locals.var_qb_dn5 - eq30_e1182_d_n5);
        let eq30_e1183_d_n6: f64 = (locals.var_qb_dn6 - eq30_e1182_d_n6);
        let eq30_e1183_d_n7: f64 = (locals.var_qb_dn7 - eq30_e1182_d_n7);
        let eq30_e1183_d_n8: f64 = (locals.var_qb_dn8 - eq30_e1182_d_n8);
        let eq30_e1183_d_n9: f64 = (locals.var_qb_dn9 - eq30_e1182_d_n9);
        let eq30_e1183_d_n10: f64 = (locals.var_qb_dn10 - eq30_e1182_d_n10);
        let eq30_e1183_d_n11: f64 = (locals.var_qb_dn11 - eq30_e1182_d_n11);
        let eq30_e1183_d_n14: f64 = (locals.var_qb_dn14 - eq30_e1182_d_n14);
        let eq30_e1184: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq30_e1183);
        let eq30_e1185: f64 = (p.p87 * eq30_e1184);
        let eq30_e1185_d_n0: f64 = (p.p87 * (eq30_e1183_d_n0 * ddt_scale));
        let eq30_e1185_d_n2: f64 = (p.p87 * (eq30_e1183_d_n2 * ddt_scale));
        let eq30_e1185_d_n4: f64 = (p.p87 * (eq30_e1183_d_n4 * ddt_scale));
        let eq30_e1185_d_n5: f64 = (p.p87 * (eq30_e1183_d_n5 * ddt_scale));
        let eq30_e1185_d_n6: f64 = (p.p87 * (eq30_e1183_d_n6 * ddt_scale));
        let eq30_e1185_d_n7: f64 = (p.p87 * (eq30_e1183_d_n7 * ddt_scale));
        let eq30_e1185_d_n8: f64 = (p.p87 * (eq30_e1183_d_n8 * ddt_scale));
        let eq30_e1185_d_n9: f64 = (p.p87 * (eq30_e1183_d_n9 * ddt_scale));
        let eq30_e1185_d_n10: f64 = (p.p87 * (eq30_e1183_d_n10 * ddt_scale));
        let eq30_e1185_d_n11: f64 = (p.p87 * (eq30_e1183_d_n11 * ddt_scale));
        let eq30_e1185_d_n12: f64 = (p.p87 * ((-eq30_e1182_d_n12) * ddt_scale));
        let eq30_e1185_d_n13: f64 = (p.p87 * ((-locals.var_qg_nqs_dn13) * ddt_scale));
        let eq30_e1185_d_n14: f64 = (p.p87 * (eq30_e1183_d_n14 * ddt_scale));
        let eq30_value: f64 = eq30_e1185;
        let eq30_node_derivative_indices: [usize; 13] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq30_node_derivatives: [f64; 13] = [eq30_e1185_d_n0, eq30_e1185_d_n2, eq30_e1185_d_n4, eq30_e1185_d_n5, eq30_e1185_d_n6, eq30_e1185_d_n7, eq30_e1185_d_n8, eq30_e1185_d_n9, eq30_e1185_d_n10, eq30_e1185_d_n11, eq30_e1185_d_n12, eq30_e1185_d_n13, eq30_e1185_d_n14];
        let eq30_branch_derivative_indices: [usize; 0] = [];
        let eq30_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq30_value),
            &eq30_node_derivative_indices,
            &eq30_node_derivatives,
            &eq30_branch_derivative_indices,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let eq31_e1188: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, locals.var_qgext);
        let eq31_e1189: f64 = (p.p87 * eq31_e1188);
        let eq31_e1189_d_n0: f64 = (p.p87 * (locals.var_qgext_dn0 * ddt_scale));
        let eq31_e1189_d_n2: f64 = (p.p87 * (locals.var_qgext_dn2 * ddt_scale));
        let eq31_e1189_d_n4: f64 = (p.p87 * (locals.var_qgext_dn4 * ddt_scale));
        let eq31_e1189_d_n5: f64 = (p.p87 * (locals.var_qgext_dn5 * ddt_scale));
        let eq31_e1189_d_n6: f64 = (p.p87 * (locals.var_qgext_dn6 * ddt_scale));
        let eq31_e1189_d_n7: f64 = (p.p87 * (locals.var_qgext_dn7 * ddt_scale));
        let eq31_e1189_d_n8: f64 = (p.p87 * (locals.var_qgext_dn8 * ddt_scale));
        let eq31_e1189_d_n9: f64 = (p.p87 * (locals.var_qgext_dn9 * ddt_scale));
        let eq31_e1189_d_n10: f64 = (p.p87 * (locals.var_qgext_dn10 * ddt_scale));
        let eq31_e1189_d_n11: f64 = (p.p87 * (locals.var_qgext_dn11 * ddt_scale));
        let eq31_e1189_d_n14: f64 = (p.p87 * (locals.var_qgext_dn14 * ddt_scale));
        let eq31_value: f64 = eq31_e1189;
        let eq31_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq31_node_derivatives: [f64; 11] = [eq31_e1189_d_n0, eq31_e1189_d_n2, eq31_e1189_d_n4, eq31_e1189_d_n5, eq31_e1189_d_n6, eq31_e1189_d_n7, eq31_e1189_d_n8, eq31_e1189_d_n9, eq31_e1189_d_n10, eq31_e1189_d_n11, eq31_e1189_d_n14];
        let eq31_branch_derivative_indices: [usize; 0] = [];
        let eq31_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(2),
            multiplicity * (eq31_value),
            &eq31_node_derivative_indices,
            &eq31_node_derivatives,
            &eq31_branch_derivative_indices,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let eq32_e1192: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, locals.var_qdext);
        let eq32_e1193: f64 = (p.p87 * eq32_e1192);
        let eq32_e1193_d_n0: f64 = (p.p87 * (locals.var_qdext_dn0 * ddt_scale));
        let eq32_e1193_d_n2: f64 = (p.p87 * (locals.var_qdext_dn2 * ddt_scale));
        let eq32_e1193_d_n4: f64 = (p.p87 * (locals.var_qdext_dn4 * ddt_scale));
        let eq32_e1193_d_n5: f64 = (p.p87 * (locals.var_qdext_dn5 * ddt_scale));
        let eq32_e1193_d_n6: f64 = (p.p87 * (locals.var_qdext_dn6 * ddt_scale));
        let eq32_e1193_d_n7: f64 = (p.p87 * (locals.var_qdext_dn7 * ddt_scale));
        let eq32_e1193_d_n8: f64 = (p.p87 * (locals.var_qdext_dn8 * ddt_scale));
        let eq32_e1193_d_n9: f64 = (p.p87 * (locals.var_qdext_dn9 * ddt_scale));
        let eq32_e1193_d_n10: f64 = (p.p87 * (locals.var_qdext_dn10 * ddt_scale));
        let eq32_e1193_d_n11: f64 = (p.p87 * (locals.var_qdext_dn11 * ddt_scale));
        let eq32_e1193_d_n14: f64 = (p.p87 * (locals.var_qdext_dn14 * ddt_scale));
        let eq32_value: f64 = eq32_e1193;
        let eq32_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq32_node_derivatives: [f64; 11] = [eq32_e1193_d_n0, eq32_e1193_d_n2, eq32_e1193_d_n4, eq32_e1193_d_n5, eq32_e1193_d_n6, eq32_e1193_d_n7, eq32_e1193_d_n8, eq32_e1193_d_n9, eq32_e1193_d_n10, eq32_e1193_d_n11, eq32_e1193_d_n14];
        let eq32_branch_derivative_indices: [usize; 0] = [];
        let eq32_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq32_value),
            &eq32_node_derivative_indices,
            &eq32_node_derivatives,
            &eq32_branch_derivative_indices,
            &eq32_branch_derivatives,
            multiplicity,
        );
        let eq33_e1196: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, locals.var_qbext);
        let eq33_e1197: f64 = (p.p87 * eq33_e1196);
        let eq33_e1197_d_n0: f64 = (p.p87 * (locals.var_qbext_dn0 * ddt_scale));
        let eq33_e1197_d_n2: f64 = (p.p87 * (locals.var_qbext_dn2 * ddt_scale));
        let eq33_e1197_d_n4: f64 = (p.p87 * (locals.var_qbext_dn4 * ddt_scale));
        let eq33_e1197_d_n5: f64 = (p.p87 * (locals.var_qbext_dn5 * ddt_scale));
        let eq33_e1197_d_n6: f64 = (p.p87 * (locals.var_qbext_dn6 * ddt_scale));
        let eq33_e1197_d_n7: f64 = (p.p87 * (locals.var_qbext_dn7 * ddt_scale));
        let eq33_e1197_d_n8: f64 = (p.p87 * (locals.var_qbext_dn8 * ddt_scale));
        let eq33_e1197_d_n9: f64 = (p.p87 * (locals.var_qbext_dn9 * ddt_scale));
        let eq33_e1197_d_n10: f64 = (p.p87 * (locals.var_qbext_dn10 * ddt_scale));
        let eq33_e1197_d_n11: f64 = (p.p87 * (locals.var_qbext_dn11 * ddt_scale));
        let eq33_e1197_d_n14: f64 = (p.p87 * (locals.var_qbext_dn14 * ddt_scale));
        let eq33_value: f64 = eq33_e1197;
        let eq33_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq33_node_derivatives: [f64; 11] = [eq33_e1197_d_n0, eq33_e1197_d_n2, eq33_e1197_d_n4, eq33_e1197_d_n5, eq33_e1197_d_n6, eq33_e1197_d_n7, eq33_e1197_d_n8, eq33_e1197_d_n9, eq33_e1197_d_n10, eq33_e1197_d_n11, eq33_e1197_d_n14];
        let eq33_branch_derivative_indices: [usize; 0] = [];
        let eq33_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(2),
            multiplicity * (eq33_value),
            &eq33_node_derivative_indices,
            &eq33_node_derivatives,
            &eq33_branch_derivative_indices,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let eq34_e1199: f64 = (-p.p87);
        let eq34_e1201: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, locals.var_qfd);
        let eq34_e1202: f64 = (eq34_e1199 * eq34_e1201);
        let eq34_e1202_d_n0: f64 = (eq34_e1199 * (locals.var_qfd_dn0 * ddt_scale));
        let eq34_e1202_d_n2: f64 = (eq34_e1199 * (locals.var_qfd_dn2 * ddt_scale));
        let eq34_e1202_d_n7: f64 = (eq34_e1199 * (locals.var_qfd_dn7 * ddt_scale));
        let eq34_value: f64 = eq34_e1202;
        stamper.stamp_current_node3_local(
            Some(7),
            Some(0),
            multiplicity * (eq34_value),
            0,
            multiplicity * (eq34_e1202_d_n0),
            2,
            multiplicity * (eq34_e1202_d_n2),
            7,
            multiplicity * (eq34_e1202_d_n7),
        );
        let eq35_e1204: f64 = (-p.p87);
        let eq35_e1206: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, locals.var_qfs);
        let eq35_e1207: f64 = (eq35_e1204 * eq35_e1206);
        let eq35_e1207_d_n2: f64 = (eq35_e1204 * (locals.var_qfs_dn2 * ddt_scale));
        let eq35_e1207_d_n7: f64 = (eq35_e1204 * (locals.var_qfs_dn7 * ddt_scale));
        let eq35_value: f64 = eq35_e1207;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(2),
            multiplicity * (eq35_value),
            2,
            multiplicity * (eq35_e1207_d_n2),
            7,
            multiplicity * (eq35_e1207_d_n7),
        );
        let eq40_e1233: f64 = (locals.var_ci * (nv15 - 0.0));
        let eq40_e1233_d_n0: f64 = (locals.var_ci_dn0 * (nv15 - 0.0));
        let eq40_e1233_d_n2: f64 = (locals.var_ci_dn2 * (nv15 - 0.0));
        let eq40_e1233_d_n4: f64 = (locals.var_ci_dn4 * (nv15 - 0.0));
        let eq40_e1233_d_n5: f64 = (locals.var_ci_dn5 * (nv15 - 0.0));
        let eq40_e1233_d_n6: f64 = (locals.var_ci_dn6 * (nv15 - 0.0));
        let eq40_e1233_d_n7: f64 = (locals.var_ci_dn7 * (nv15 - 0.0));
        let eq40_e1233_d_n8: f64 = (locals.var_ci_dn8 * (nv15 - 0.0));
        let eq40_e1233_d_n9: f64 = (locals.var_ci_dn9 * (nv15 - 0.0));
        let eq40_e1233_d_n10: f64 = (locals.var_ci_dn10 * (nv15 - 0.0));
        let eq40_e1233_d_n11: f64 = (locals.var_ci_dn11 * (nv15 - 0.0));
        let eq40_e1233_d_n14: f64 = (locals.var_ci_dn14 * (nv15 - 0.0));
        let eq40_value: f64 = eq40_e1233;
        let eq40_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14, 15];
        let eq40_node_derivatives: [f64; 12] = [eq40_e1233_d_n0, eq40_e1233_d_n2, eq40_e1233_d_n4, eq40_e1233_d_n5, eq40_e1233_d_n6, eq40_e1233_d_n7, eq40_e1233_d_n8, eq40_e1233_d_n9, eq40_e1233_d_n10, eq40_e1233_d_n11, eq40_e1233_d_n14, locals.var_ci];
        let eq40_branch_derivative_indices: [usize; 0] = [];
        let eq40_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq40_value),
            &eq40_node_derivative_indices,
            &eq40_node_derivatives,
            &eq40_branch_derivative_indices,
            &eq40_branch_derivatives,
            multiplicity,
        );
        let eq41_e1236: f64 = ((nv15 - 0.0) * locals.var_sigrat_s);
        let eq41_e1236_d_n0: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn0);
        let eq41_e1236_d_n2: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn2);
        let eq41_e1236_d_n4: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn4);
        let eq41_e1236_d_n5: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn5);
        let eq41_e1236_d_n6: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn6);
        let eq41_e1236_d_n7: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn7);
        let eq41_e1236_d_n8: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn8);
        let eq41_e1236_d_n9: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn9);
        let eq41_e1236_d_n10: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn10);
        let eq41_e1236_d_n11: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn11);
        let eq41_e1236_d_n14: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn14);
        let eq41_e1237: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, eq41_e1236);
        let eq41_value: f64 = eq41_e1237;
        let eq41_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14, 15];
        let eq41_node_derivatives: [f64; 12] = [(eq41_e1236_d_n0 * ddt_scale), (eq41_e1236_d_n2 * ddt_scale), (eq41_e1236_d_n4 * ddt_scale), (eq41_e1236_d_n5 * ddt_scale), (eq41_e1236_d_n6 * ddt_scale), (eq41_e1236_d_n7 * ddt_scale), (eq41_e1236_d_n8 * ddt_scale), (eq41_e1236_d_n9 * ddt_scale), (eq41_e1236_d_n10 * ddt_scale), (eq41_e1236_d_n11 * ddt_scale), (eq41_e1236_d_n14 * ddt_scale), (locals.var_sigrat_s * ddt_scale)];
        let eq41_branch_derivative_indices: [usize; 0] = [];
        let eq41_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq41_value),
            &eq41_node_derivative_indices,
            &eq41_node_derivatives,
            &eq41_branch_derivative_indices,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let eq42_e1240: f64 = ((nv15 - 0.0) * locals.var_sigrat_d);
        let eq42_e1240_d_n0: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn0);
        let eq42_e1240_d_n2: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn2);
        let eq42_e1240_d_n4: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn4);
        let eq42_e1240_d_n5: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn5);
        let eq42_e1240_d_n6: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn6);
        let eq42_e1240_d_n7: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn7);
        let eq42_e1240_d_n8: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn8);
        let eq42_e1240_d_n9: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn9);
        let eq42_e1240_d_n10: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn10);
        let eq42_e1240_d_n11: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn11);
        let eq42_e1240_d_n14: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn14);
        let eq42_e1241: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, eq42_e1240);
        let eq42_value: f64 = eq42_e1241;
        let eq42_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14, 15];
        let eq42_node_derivatives: [f64; 12] = [(eq42_e1240_d_n0 * ddt_scale), (eq42_e1240_d_n2 * ddt_scale), (eq42_e1240_d_n4 * ddt_scale), (eq42_e1240_d_n5 * ddt_scale), (eq42_e1240_d_n6 * ddt_scale), (eq42_e1240_d_n7 * ddt_scale), (eq42_e1240_d_n8 * ddt_scale), (eq42_e1240_d_n9 * ddt_scale), (eq42_e1240_d_n10 * ddt_scale), (eq42_e1240_d_n11 * ddt_scale), (eq42_e1240_d_n14 * ddt_scale), (locals.var_sigrat_d * ddt_scale)];
        let eq42_branch_derivative_indices: [usize; 0] = [];
        let eq42_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq42_value),
            &eq42_node_derivative_indices,
            &eq42_node_derivatives,
            &eq42_branch_derivative_indices,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let (eq57_e1336, eq57_e1336_d_n0, eq57_e1336_d_n2, eq57_e1336_d_n4, eq57_e1336_d_n5, eq57_e1336_d_n6, eq57_e1336_d_n7, eq57_e1336_d_n8, eq57_e1336_d_n9, eq57_e1336_d_n10, eq57_e1336_d_n11, eq57_e1336_d_n14,) = {
    if (locals.var_guard2415 != 0.0) {
        let eq57_e1334: f64 = (-locals.var_itemp);
        (eq57_e1334, (-locals.var_itemp_dn0), (-locals.var_itemp_dn2), (-locals.var_itemp_dn4), (-locals.var_itemp_dn5), (-locals.var_itemp_dn6), (-locals.var_itemp_dn7), (-locals.var_itemp_dn8), (-locals.var_itemp_dn9), (-locals.var_itemp_dn10), (-locals.var_itemp_dn11), (-locals.var_itemp_dn14),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e1336;
        let eq57_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq57_node_derivatives: [f64; 11] = [eq57_e1336_d_n0, eq57_e1336_d_n2, eq57_e1336_d_n4, eq57_e1336_d_n5, eq57_e1336_d_n6, eq57_e1336_d_n7, eq57_e1336_d_n8, eq57_e1336_d_n9, eq57_e1336_d_n10, eq57_e1336_d_n11, eq57_e1336_d_n14];
        let eq57_branch_derivative_indices: [usize; 0] = [];
        let eq57_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            None,
            multiplicity * (eq57_value),
            &eq57_node_derivative_indices,
            &eq57_node_derivatives,
            &eq57_branch_derivative_indices,
            &eq57_branch_derivatives,
            multiplicity,
        );
        let (eq62_e1362, eq62_e1362_d_n12,) = {
    if (p.p28 != 0.0) {
        let eq62_e1359: f64 = (locals.var_cqi * (nv12 - 0.0));
        let eq62_e1360: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq62_e1359);
        (eq62_e1360, (locals.var_cqi * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e1362;
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (eq62_value),
            12,
            multiplicity * (eq62_e1362_d_n12),
        );
        let (eq63_e1369, eq63_e1369_d_n13,) = {
    if (p.p28 != 0.0) {
        let eq63_e1366: f64 = (locals.var_cqb * (nv13 - 0.0));
        let eq63_e1367: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, eq63_e1366);
        (eq63_e1367, (locals.var_cqb * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e1369;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq63_value),
            13,
            multiplicity * (eq63_e1369_d_n13),
        );
        let (eq67_e1388, eq67_e1388_d_n14,) = {
    if (p.p29 != 0.0) {
        let eq67_e1386: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 20, (nv14 - 0.0));
        (eq67_e1386, ddt_scale,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e1388;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (eq67_value),
            14,
            multiplicity * (eq67_e1388_d_n14),
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq1_e1022, eq1_e1022_d_n0, eq1_e1022_d_n2, eq1_e1022_d_n4, eq1_e1022_d_n5, eq1_e1022_d_n6, eq1_e1022_d_n7, eq1_e1022_d_n8, eq1_e1022_d_n9, eq1_e1022_d_n10, eq1_e1022_d_n11, eq1_e1022_d_n14, eq1_e1022_d_n16, eq1_e1022_q, eq1_e1022_q_d_n16,) = {
    if (locals.var_guard2311 != 0.0) {
        let eq1_e1019_q: f64 = locals.var_q_nqs_a;
        let eq1_e1020: f64 = (locals.var_inqs0_a + locals.var_q_nqs_a);
        let eq1_e1020_d_n16: f64 = (locals.var_inqs0_a_dn16 + locals.var_q_nqs_a_dn16);
        let eq1_e1020_q: f64 = eq1_e1019_q;
        (eq1_e1020, locals.var_inqs0_a_dn0, locals.var_inqs0_a_dn2, locals.var_inqs0_a_dn4, locals.var_inqs0_a_dn5, locals.var_inqs0_a_dn6, locals.var_inqs0_a_dn7, locals.var_inqs0_a_dn8, locals.var_inqs0_a_dn9, locals.var_inqs0_a_dn10, locals.var_inqs0_a_dn11, locals.var_inqs0_a_dn14, eq1_e1020_d_n16, eq1_e1020_q, locals.var_q_nqs_a_dn16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[16]),
            None,
            nodes[16],
            multiplicity * (eq1_e1022_q_d_n16),
        );
        let (eq2_e1029, eq2_e1029_d_n0, eq2_e1029_d_n2, eq2_e1029_d_n4, eq2_e1029_d_n5, eq2_e1029_d_n6, eq2_e1029_d_n7, eq2_e1029_d_n8, eq2_e1029_d_n9, eq2_e1029_d_n10, eq2_e1029_d_n11, eq2_e1029_d_n14, eq2_e1029_d_n17, eq2_e1029_q, eq2_e1029_q_d_n17,) = {
    if (locals.var_guard2311 != 0.0) {
        let eq2_e1026_q: f64 = locals.var_q_nqs_k;
        let eq2_e1027: f64 = (locals.var_inqs0_k + locals.var_q_nqs_k);
        let eq2_e1027_d_n17: f64 = (locals.var_inqs0_k_dn17 + locals.var_q_nqs_k_dn17);
        let eq2_e1027_q: f64 = eq2_e1026_q;
        (eq2_e1027, locals.var_inqs0_k_dn0, locals.var_inqs0_k_dn2, locals.var_inqs0_k_dn4, locals.var_inqs0_k_dn5, locals.var_inqs0_k_dn6, locals.var_inqs0_k_dn7, locals.var_inqs0_k_dn8, locals.var_inqs0_k_dn9, locals.var_inqs0_k_dn10, locals.var_inqs0_k_dn11, locals.var_inqs0_k_dn14, eq2_e1027_d_n17, eq2_e1027_q, locals.var_q_nqs_k_dn17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq2_e1029_q_d_n17),
        );
        let (eq5_e1046, eq5_e1046_d_n0, eq5_e1046_d_n2, eq5_e1046_d_n4, eq5_e1046_d_n5, eq5_e1046_d_n6, eq5_e1046_d_n7, eq5_e1046_d_n8, eq5_e1046_d_n9, eq5_e1046_d_n10, eq5_e1046_d_n11, eq5_e1046_d_n14, eq5_e1046_d_n18, eq5_e1046_q, eq5_e1046_q_d_n18,) = {
    if (locals.var_guard2312 != 0.0) {
        let eq5_e1043_q: f64 = locals.var_w_nqs_a;
        let eq5_e1044: f64 = (locals.var_iwnqs0_a + locals.var_w_nqs_a);
        let eq5_e1044_d_n18: f64 = (locals.var_iwnqs0_a_dn18 + locals.var_w_nqs_a_dn18);
        let eq5_e1044_q: f64 = eq5_e1043_q;
        (eq5_e1044, locals.var_iwnqs0_a_dn0, locals.var_iwnqs0_a_dn2, locals.var_iwnqs0_a_dn4, locals.var_iwnqs0_a_dn5, locals.var_iwnqs0_a_dn6, locals.var_iwnqs0_a_dn7, locals.var_iwnqs0_a_dn8, locals.var_iwnqs0_a_dn9, locals.var_iwnqs0_a_dn10, locals.var_iwnqs0_a_dn11, locals.var_iwnqs0_a_dn14, eq5_e1044_d_n18, eq5_e1044_q, locals.var_w_nqs_a_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[18]),
            None,
            nodes[18],
            multiplicity * (eq5_e1046_q_d_n18),
        );
        let eq15_e1092_q: f64 = locals.var_qbs;
        let eq15_e1093: f64 = (p.p87 * locals.var_qbs);
        let eq15_e1093_d_n0: f64 = (p.p87 * locals.var_qbs_dn0);
        let eq15_e1093_d_n2: f64 = (p.p87 * locals.var_qbs_dn2);
        let eq15_e1093_d_n4: f64 = (p.p87 * locals.var_qbs_dn4);
        let eq15_e1093_d_n5: f64 = (p.p87 * locals.var_qbs_dn5);
        let eq15_e1093_d_n6: f64 = (p.p87 * locals.var_qbs_dn6);
        let eq15_e1093_d_n7: f64 = (p.p87 * locals.var_qbs_dn7);
        let eq15_e1093_d_n8: f64 = (p.p87 * locals.var_qbs_dn8);
        let eq15_e1093_d_n9: f64 = (p.p87 * locals.var_qbs_dn9);
        let eq15_e1093_d_n10: f64 = (p.p87 * locals.var_qbs_dn10);
        let eq15_e1093_d_n11: f64 = (p.p87 * locals.var_qbs_dn11);
        let eq15_e1093_d_n14: f64 = (p.p87 * locals.var_qbs_dn14);
        let eq15_e1093_q: f64 = (p.p87 * eq15_e1092_q);
        let eq15_reactive_node_derivatives: [f64; 19] = [eq15_e1093_d_n0, 0.0, eq15_e1093_d_n2, 0.0, eq15_e1093_d_n4, eq15_e1093_d_n5, eq15_e1093_d_n6, eq15_e1093_d_n7, eq15_e1093_d_n8, eq15_e1093_d_n9, eq15_e1093_d_n10, eq15_e1093_d_n11, 0.0, 0.0, eq15_e1093_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq15_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[2]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq16_e1096_q: f64 = locals.var_qbd;
        let eq16_e1097: f64 = (p.p87 * locals.var_qbd);
        let eq16_e1097_d_n0: f64 = (p.p87 * locals.var_qbd_dn0);
        let eq16_e1097_d_n2: f64 = (p.p87 * locals.var_qbd_dn2);
        let eq16_e1097_d_n4: f64 = (p.p87 * locals.var_qbd_dn4);
        let eq16_e1097_d_n5: f64 = (p.p87 * locals.var_qbd_dn5);
        let eq16_e1097_d_n6: f64 = (p.p87 * locals.var_qbd_dn6);
        let eq16_e1097_d_n7: f64 = (p.p87 * locals.var_qbd_dn7);
        let eq16_e1097_d_n8: f64 = (p.p87 * locals.var_qbd_dn8);
        let eq16_e1097_d_n9: f64 = (p.p87 * locals.var_qbd_dn9);
        let eq16_e1097_d_n10: f64 = (p.p87 * locals.var_qbd_dn10);
        let eq16_e1097_d_n11: f64 = (p.p87 * locals.var_qbd_dn11);
        let eq16_e1097_d_n14: f64 = (p.p87 * locals.var_qbd_dn14);
        let eq16_e1097_d_n16: f64 = (p.p87 * locals.var_qbd_dn16);
        let eq16_e1097_d_n17: f64 = (p.p87 * locals.var_qbd_dn17);
        let eq16_e1097_d_n18: f64 = (p.p87 * locals.var_qbd_dn18);
        let eq16_e1097_q: f64 = (p.p87 * eq16_e1096_q);
        let eq16_reactive_node_derivatives: [f64; 19] = [eq16_e1097_d_n0, 0.0, eq16_e1097_d_n2, 0.0, eq16_e1097_d_n4, eq16_e1097_d_n5, eq16_e1097_d_n6, eq16_e1097_d_n7, eq16_e1097_d_n8, eq16_e1097_d_n9, eq16_e1097_d_n10, eq16_e1097_d_n11, 0.0, 0.0, eq16_e1097_d_n14, 0.0, eq16_e1097_d_n16, eq16_e1097_d_n17, eq16_e1097_d_n18];
        let eq16_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[0]),
            nodes,
            &eq16_reactive_node_derivatives,
            branches,
            &eq16_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq19_e1116, eq19_e1116_d_n0, eq19_e1116_d_n2, eq19_e1116_d_n4, eq19_e1116_d_n5, eq19_e1116_d_n6, eq19_e1116_d_n7, eq19_e1116_d_n8, eq19_e1116_d_n9, eq19_e1116_d_n10, eq19_e1116_d_n11, eq19_e1116_d_n14, eq19_e1116_q,) = {
    if (locals.var_guard2411 != 0.0) {
        let eq19_e1113_q: f64 = locals.var_qbsi;
        let eq19_e1114: f64 = (p.p87 * locals.var_qbsi);
        let eq19_e1114_d_n0: f64 = (p.p87 * locals.var_qbsi_dn0);
        let eq19_e1114_d_n2: f64 = (p.p87 * locals.var_qbsi_dn2);
        let eq19_e1114_d_n4: f64 = (p.p87 * locals.var_qbsi_dn4);
        let eq19_e1114_d_n5: f64 = (p.p87 * locals.var_qbsi_dn5);
        let eq19_e1114_d_n6: f64 = (p.p87 * locals.var_qbsi_dn6);
        let eq19_e1114_d_n7: f64 = (p.p87 * locals.var_qbsi_dn7);
        let eq19_e1114_d_n8: f64 = (p.p87 * locals.var_qbsi_dn8);
        let eq19_e1114_d_n9: f64 = (p.p87 * locals.var_qbsi_dn9);
        let eq19_e1114_d_n10: f64 = (p.p87 * locals.var_qbsi_dn10);
        let eq19_e1114_d_n11: f64 = (p.p87 * locals.var_qbsi_dn11);
        let eq19_e1114_d_n14: f64 = (p.p87 * locals.var_qbsi_dn14);
        let eq19_e1114_q: f64 = (p.p87 * eq19_e1113_q);
        (eq19_e1114, eq19_e1114_d_n0, eq19_e1114_d_n2, eq19_e1114_d_n4, eq19_e1114_d_n5, eq19_e1114_d_n6, eq19_e1114_d_n7, eq19_e1114_d_n8, eq19_e1114_d_n9, eq19_e1114_d_n10, eq19_e1114_d_n11, eq19_e1114_d_n14, eq19_e1114_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_reactive_node_derivatives: [f64; 19] = [eq19_e1116_d_n0, 0.0, eq19_e1116_d_n2, 0.0, eq19_e1116_d_n4, eq19_e1116_d_n5, eq19_e1116_d_n6, eq19_e1116_d_n7, eq19_e1116_d_n8, eq19_e1116_d_n9, eq19_e1116_d_n10, eq19_e1116_d_n11, 0.0, 0.0, eq19_e1116_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq19_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq20_e1123, eq20_e1123_d_n0, eq20_e1123_d_n2, eq20_e1123_d_n4, eq20_e1123_d_n5, eq20_e1123_d_n6, eq20_e1123_d_n7, eq20_e1123_d_n8, eq20_e1123_d_n9, eq20_e1123_d_n10, eq20_e1123_d_n11, eq20_e1123_d_n14, eq20_e1123_q,) = {
    if (locals.var_guard2411 != 0.0) {
        let eq20_e1120_q: f64 = locals.var_qbdi;
        let eq20_e1121: f64 = (p.p87 * locals.var_qbdi);
        let eq20_e1121_d_n0: f64 = (p.p87 * locals.var_qbdi_dn0);
        let eq20_e1121_d_n2: f64 = (p.p87 * locals.var_qbdi_dn2);
        let eq20_e1121_d_n4: f64 = (p.p87 * locals.var_qbdi_dn4);
        let eq20_e1121_d_n5: f64 = (p.p87 * locals.var_qbdi_dn5);
        let eq20_e1121_d_n6: f64 = (p.p87 * locals.var_qbdi_dn6);
        let eq20_e1121_d_n7: f64 = (p.p87 * locals.var_qbdi_dn7);
        let eq20_e1121_d_n8: f64 = (p.p87 * locals.var_qbdi_dn8);
        let eq20_e1121_d_n9: f64 = (p.p87 * locals.var_qbdi_dn9);
        let eq20_e1121_d_n10: f64 = (p.p87 * locals.var_qbdi_dn10);
        let eq20_e1121_d_n11: f64 = (p.p87 * locals.var_qbdi_dn11);
        let eq20_e1121_d_n14: f64 = (p.p87 * locals.var_qbdi_dn14);
        let eq20_e1121_q: f64 = (p.p87 * eq20_e1120_q);
        (eq20_e1121, eq20_e1121_d_n0, eq20_e1121_d_n2, eq20_e1121_d_n4, eq20_e1121_d_n5, eq20_e1121_d_n6, eq20_e1121_d_n7, eq20_e1121_d_n8, eq20_e1121_d_n9, eq20_e1121_d_n10, eq20_e1121_d_n11, eq20_e1121_d_n14, eq20_e1121_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_reactive_node_derivatives: [f64; 19] = [eq20_e1123_d_n0, 0.0, eq20_e1123_d_n2, 0.0, eq20_e1123_d_n4, eq20_e1123_d_n5, eq20_e1123_d_n6, eq20_e1123_d_n7, eq20_e1123_d_n8, eq20_e1123_d_n9, eq20_e1123_d_n10, eq20_e1123_d_n11, 0.0, 0.0, eq20_e1123_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq20_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &eq20_reactive_branch_derivatives,
            multiplicity,
        );
        let eq28_e1167: f64 = (locals.var_qg + locals.var_qg_nqs);
        let eq28_e1168_q: f64 = eq28_e1167;
        let eq28_e1169: f64 = (p.p87 * eq28_e1167);
        let eq28_e1169_d_n0: f64 = (p.p87 * locals.var_qg_dn0);
        let eq28_e1169_d_n2: f64 = (p.p87 * locals.var_qg_dn2);
        let eq28_e1169_d_n4: f64 = (p.p87 * locals.var_qg_dn4);
        let eq28_e1169_d_n5: f64 = (p.p87 * locals.var_qg_dn5);
        let eq28_e1169_d_n6: f64 = (p.p87 * locals.var_qg_dn6);
        let eq28_e1169_d_n7: f64 = (p.p87 * locals.var_qg_dn7);
        let eq28_e1169_d_n8: f64 = (p.p87 * locals.var_qg_dn8);
        let eq28_e1169_d_n9: f64 = (p.p87 * locals.var_qg_dn9);
        let eq28_e1169_d_n10: f64 = (p.p87 * locals.var_qg_dn10);
        let eq28_e1169_d_n11: f64 = (p.p87 * locals.var_qg_dn11);
        let eq28_e1169_d_n12: f64 = (p.p87 * locals.var_qg_nqs_dn12);
        let eq28_e1169_d_n13: f64 = (p.p87 * locals.var_qg_nqs_dn13);
        let eq28_e1169_d_n14: f64 = (p.p87 * locals.var_qg_dn14);
        let eq28_e1169_q: f64 = (p.p87 * eq28_e1168_q);
        let eq28_reactive_node_derivatives: [f64; 19] = [eq28_e1169_d_n0, 0.0, eq28_e1169_d_n2, 0.0, eq28_e1169_d_n4, eq28_e1169_d_n5, eq28_e1169_d_n6, eq28_e1169_d_n7, eq28_e1169_d_n8, eq28_e1169_d_n9, eq28_e1169_d_n10, eq28_e1169_d_n11, eq28_e1169_d_n12, eq28_e1169_d_n13, eq28_e1169_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq28_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[8]),
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
        let eq29_e1173: f64 = (locals.var_qd + locals.var_qd_nqs);
        let eq29_e1173_d_n0: f64 = (locals.var_qd_dn0 + locals.var_qd_nqs_dn0);
        let eq29_e1173_d_n2: f64 = (locals.var_qd_dn2 + locals.var_qd_nqs_dn2);
        let eq29_e1173_d_n4: f64 = (locals.var_qd_dn4 + locals.var_qd_nqs_dn4);
        let eq29_e1173_d_n5: f64 = (locals.var_qd_dn5 + locals.var_qd_nqs_dn5);
        let eq29_e1173_d_n6: f64 = (locals.var_qd_dn6 + locals.var_qd_nqs_dn6);
        let eq29_e1173_d_n7: f64 = (locals.var_qd_dn7 + locals.var_qd_nqs_dn7);
        let eq29_e1173_d_n8: f64 = (locals.var_qd_dn8 + locals.var_qd_nqs_dn8);
        let eq29_e1173_d_n9: f64 = (locals.var_qd_dn9 + locals.var_qd_nqs_dn9);
        let eq29_e1173_d_n10: f64 = (locals.var_qd_dn10 + locals.var_qd_nqs_dn10);
        let eq29_e1173_d_n11: f64 = (locals.var_qd_dn11 + locals.var_qd_nqs_dn11);
        let eq29_e1173_d_n14: f64 = (locals.var_qd_dn14 + locals.var_qd_nqs_dn14);
        let eq29_e1174_q: f64 = eq29_e1173;
        let eq29_e1175: f64 = (p.p87 * eq29_e1173);
        let eq29_e1175_d_n0: f64 = (p.p87 * eq29_e1173_d_n0);
        let eq29_e1175_d_n2: f64 = (p.p87 * eq29_e1173_d_n2);
        let eq29_e1175_d_n4: f64 = (p.p87 * eq29_e1173_d_n4);
        let eq29_e1175_d_n5: f64 = (p.p87 * eq29_e1173_d_n5);
        let eq29_e1175_d_n6: f64 = (p.p87 * eq29_e1173_d_n6);
        let eq29_e1175_d_n7: f64 = (p.p87 * eq29_e1173_d_n7);
        let eq29_e1175_d_n8: f64 = (p.p87 * eq29_e1173_d_n8);
        let eq29_e1175_d_n9: f64 = (p.p87 * eq29_e1173_d_n9);
        let eq29_e1175_d_n10: f64 = (p.p87 * eq29_e1173_d_n10);
        let eq29_e1175_d_n11: f64 = (p.p87 * eq29_e1173_d_n11);
        let eq29_e1175_d_n12: f64 = (p.p87 * locals.var_qd_nqs_dn12);
        let eq29_e1175_d_n14: f64 = (p.p87 * eq29_e1173_d_n14);
        let eq29_e1175_q: f64 = (p.p87 * eq29_e1174_q);
        let eq29_reactive_node_derivatives: [f64; 19] = [eq29_e1175_d_n0, 0.0, eq29_e1175_d_n2, 0.0, eq29_e1175_d_n4, eq29_e1175_d_n5, eq29_e1175_d_n6, eq29_e1175_d_n7, eq29_e1175_d_n8, eq29_e1175_d_n9, eq29_e1175_d_n10, eq29_e1175_d_n11, eq29_e1175_d_n12, 0.0, eq29_e1175_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq29_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            nodes,
            &eq29_reactive_node_derivatives,
            branches,
            &eq29_reactive_branch_derivatives,
            multiplicity,
        );
        let eq30_e1180: f64 = (locals.var_qg_nqs + locals.var_qd_nqs);
        let eq30_e1180_d_n12: f64 = (locals.var_qg_nqs_dn12 + locals.var_qd_nqs_dn12);
        let eq30_e1182: f64 = (eq30_e1180 + locals.var_qs_nqs);
        let eq30_e1182_d_n0: f64 = (locals.var_qd_nqs_dn0 + locals.var_qs_nqs_dn0);
        let eq30_e1182_d_n2: f64 = (locals.var_qd_nqs_dn2 + locals.var_qs_nqs_dn2);
        let eq30_e1182_d_n4: f64 = (locals.var_qd_nqs_dn4 + locals.var_qs_nqs_dn4);
        let eq30_e1182_d_n5: f64 = (locals.var_qd_nqs_dn5 + locals.var_qs_nqs_dn5);
        let eq30_e1182_d_n6: f64 = (locals.var_qd_nqs_dn6 + locals.var_qs_nqs_dn6);
        let eq30_e1182_d_n7: f64 = (locals.var_qd_nqs_dn7 + locals.var_qs_nqs_dn7);
        let eq30_e1182_d_n8: f64 = (locals.var_qd_nqs_dn8 + locals.var_qs_nqs_dn8);
        let eq30_e1182_d_n9: f64 = (locals.var_qd_nqs_dn9 + locals.var_qs_nqs_dn9);
        let eq30_e1182_d_n10: f64 = (locals.var_qd_nqs_dn10 + locals.var_qs_nqs_dn10);
        let eq30_e1182_d_n11: f64 = (locals.var_qd_nqs_dn11 + locals.var_qs_nqs_dn11);
        let eq30_e1182_d_n12: f64 = (eq30_e1180_d_n12 + locals.var_qs_nqs_dn12);
        let eq30_e1182_d_n14: f64 = (locals.var_qd_nqs_dn14 + locals.var_qs_nqs_dn14);
        let eq30_e1183: f64 = (locals.var_qb - eq30_e1182);
        let eq30_e1183_d_n0: f64 = (locals.var_qb_dn0 - eq30_e1182_d_n0);
        let eq30_e1183_d_n2: f64 = (locals.var_qb_dn2 - eq30_e1182_d_n2);
        let eq30_e1183_d_n4: f64 = (locals.var_qb_dn4 - eq30_e1182_d_n4);
        let eq30_e1183_d_n5: f64 = (locals.var_qb_dn5 - eq30_e1182_d_n5);
        let eq30_e1183_d_n6: f64 = (locals.var_qb_dn6 - eq30_e1182_d_n6);
        let eq30_e1183_d_n7: f64 = (locals.var_qb_dn7 - eq30_e1182_d_n7);
        let eq30_e1183_d_n8: f64 = (locals.var_qb_dn8 - eq30_e1182_d_n8);
        let eq30_e1183_d_n9: f64 = (locals.var_qb_dn9 - eq30_e1182_d_n9);
        let eq30_e1183_d_n10: f64 = (locals.var_qb_dn10 - eq30_e1182_d_n10);
        let eq30_e1183_d_n11: f64 = (locals.var_qb_dn11 - eq30_e1182_d_n11);
        let eq30_e1183_d_n14: f64 = (locals.var_qb_dn14 - eq30_e1182_d_n14);
        let eq30_e1184_q: f64 = eq30_e1183;
        let eq30_e1185: f64 = (p.p87 * eq30_e1183);
        let eq30_e1185_d_n0: f64 = (p.p87 * eq30_e1183_d_n0);
        let eq30_e1185_d_n2: f64 = (p.p87 * eq30_e1183_d_n2);
        let eq30_e1185_d_n4: f64 = (p.p87 * eq30_e1183_d_n4);
        let eq30_e1185_d_n5: f64 = (p.p87 * eq30_e1183_d_n5);
        let eq30_e1185_d_n6: f64 = (p.p87 * eq30_e1183_d_n6);
        let eq30_e1185_d_n7: f64 = (p.p87 * eq30_e1183_d_n7);
        let eq30_e1185_d_n8: f64 = (p.p87 * eq30_e1183_d_n8);
        let eq30_e1185_d_n9: f64 = (p.p87 * eq30_e1183_d_n9);
        let eq30_e1185_d_n10: f64 = (p.p87 * eq30_e1183_d_n10);
        let eq30_e1185_d_n11: f64 = (p.p87 * eq30_e1183_d_n11);
        let eq30_e1185_d_n12: f64 = (p.p87 * (-eq30_e1182_d_n12));
        let eq30_e1185_d_n13: f64 = (p.p87 * (-locals.var_qg_nqs_dn13));
        let eq30_e1185_d_n14: f64 = (p.p87 * eq30_e1183_d_n14);
        let eq30_e1185_q: f64 = (p.p87 * eq30_e1184_q);
        let eq30_reactive_node_derivatives: [f64; 19] = [eq30_e1185_d_n0, 0.0, eq30_e1185_d_n2, 0.0, eq30_e1185_d_n4, eq30_e1185_d_n5, eq30_e1185_d_n6, eq30_e1185_d_n7, eq30_e1185_d_n8, eq30_e1185_d_n9, eq30_e1185_d_n10, eq30_e1185_d_n11, eq30_e1185_d_n12, eq30_e1185_d_n13, eq30_e1185_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq30_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq30_reactive_node_derivatives,
            branches,
            &eq30_reactive_branch_derivatives,
            multiplicity,
        );
        let eq31_e1188_q: f64 = locals.var_qgext;
        let eq31_e1189: f64 = (p.p87 * locals.var_qgext);
        let eq31_e1189_d_n0: f64 = (p.p87 * locals.var_qgext_dn0);
        let eq31_e1189_d_n2: f64 = (p.p87 * locals.var_qgext_dn2);
        let eq31_e1189_d_n4: f64 = (p.p87 * locals.var_qgext_dn4);
        let eq31_e1189_d_n5: f64 = (p.p87 * locals.var_qgext_dn5);
        let eq31_e1189_d_n6: f64 = (p.p87 * locals.var_qgext_dn6);
        let eq31_e1189_d_n7: f64 = (p.p87 * locals.var_qgext_dn7);
        let eq31_e1189_d_n8: f64 = (p.p87 * locals.var_qgext_dn8);
        let eq31_e1189_d_n9: f64 = (p.p87 * locals.var_qgext_dn9);
        let eq31_e1189_d_n10: f64 = (p.p87 * locals.var_qgext_dn10);
        let eq31_e1189_d_n11: f64 = (p.p87 * locals.var_qgext_dn11);
        let eq31_e1189_d_n14: f64 = (p.p87 * locals.var_qgext_dn14);
        let eq31_e1189_q: f64 = (p.p87 * eq31_e1188_q);
        let eq31_reactive_node_derivatives: [f64; 19] = [eq31_e1189_d_n0, 0.0, eq31_e1189_d_n2, 0.0, eq31_e1189_d_n4, eq31_e1189_d_n5, eq31_e1189_d_n6, eq31_e1189_d_n7, eq31_e1189_d_n8, eq31_e1189_d_n9, eq31_e1189_d_n10, eq31_e1189_d_n11, 0.0, 0.0, eq31_e1189_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq31_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[2]),
            nodes,
            &eq31_reactive_node_derivatives,
            branches,
            &eq31_reactive_branch_derivatives,
            multiplicity,
        );
        let eq32_e1192_q: f64 = locals.var_qdext;
        let eq32_e1193: f64 = (p.p87 * locals.var_qdext);
        let eq32_e1193_d_n0: f64 = (p.p87 * locals.var_qdext_dn0);
        let eq32_e1193_d_n2: f64 = (p.p87 * locals.var_qdext_dn2);
        let eq32_e1193_d_n4: f64 = (p.p87 * locals.var_qdext_dn4);
        let eq32_e1193_d_n5: f64 = (p.p87 * locals.var_qdext_dn5);
        let eq32_e1193_d_n6: f64 = (p.p87 * locals.var_qdext_dn6);
        let eq32_e1193_d_n7: f64 = (p.p87 * locals.var_qdext_dn7);
        let eq32_e1193_d_n8: f64 = (p.p87 * locals.var_qdext_dn8);
        let eq32_e1193_d_n9: f64 = (p.p87 * locals.var_qdext_dn9);
        let eq32_e1193_d_n10: f64 = (p.p87 * locals.var_qdext_dn10);
        let eq32_e1193_d_n11: f64 = (p.p87 * locals.var_qdext_dn11);
        let eq32_e1193_d_n14: f64 = (p.p87 * locals.var_qdext_dn14);
        let eq32_e1193_q: f64 = (p.p87 * eq32_e1192_q);
        let eq32_reactive_node_derivatives: [f64; 19] = [eq32_e1193_d_n0, 0.0, eq32_e1193_d_n2, 0.0, eq32_e1193_d_n4, eq32_e1193_d_n5, eq32_e1193_d_n6, eq32_e1193_d_n7, eq32_e1193_d_n8, eq32_e1193_d_n9, eq32_e1193_d_n10, eq32_e1193_d_n11, 0.0, 0.0, eq32_e1193_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq32_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            nodes,
            &eq32_reactive_node_derivatives,
            branches,
            &eq32_reactive_branch_derivatives,
            multiplicity,
        );
        let eq33_e1196_q: f64 = locals.var_qbext;
        let eq33_e1197: f64 = (p.p87 * locals.var_qbext);
        let eq33_e1197_d_n0: f64 = (p.p87 * locals.var_qbext_dn0);
        let eq33_e1197_d_n2: f64 = (p.p87 * locals.var_qbext_dn2);
        let eq33_e1197_d_n4: f64 = (p.p87 * locals.var_qbext_dn4);
        let eq33_e1197_d_n5: f64 = (p.p87 * locals.var_qbext_dn5);
        let eq33_e1197_d_n6: f64 = (p.p87 * locals.var_qbext_dn6);
        let eq33_e1197_d_n7: f64 = (p.p87 * locals.var_qbext_dn7);
        let eq33_e1197_d_n8: f64 = (p.p87 * locals.var_qbext_dn8);
        let eq33_e1197_d_n9: f64 = (p.p87 * locals.var_qbext_dn9);
        let eq33_e1197_d_n10: f64 = (p.p87 * locals.var_qbext_dn10);
        let eq33_e1197_d_n11: f64 = (p.p87 * locals.var_qbext_dn11);
        let eq33_e1197_d_n14: f64 = (p.p87 * locals.var_qbext_dn14);
        let eq33_e1197_q: f64 = (p.p87 * eq33_e1196_q);
        let eq33_reactive_node_derivatives: [f64; 19] = [eq33_e1197_d_n0, 0.0, eq33_e1197_d_n2, 0.0, eq33_e1197_d_n4, eq33_e1197_d_n5, eq33_e1197_d_n6, eq33_e1197_d_n7, eq33_e1197_d_n8, eq33_e1197_d_n9, eq33_e1197_d_n10, eq33_e1197_d_n11, 0.0, 0.0, eq33_e1197_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq33_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[2]),
            nodes,
            &eq33_reactive_node_derivatives,
            branches,
            &eq33_reactive_branch_derivatives,
            multiplicity,
        );
        let eq34_e1199: f64 = (-p.p87);
        let eq34_e1201_q: f64 = locals.var_qfd;
        let eq34_e1202: f64 = (eq34_e1199 * locals.var_qfd);
        let eq34_e1202_d_n0: f64 = (eq34_e1199 * locals.var_qfd_dn0);
        let eq34_e1202_d_n2: f64 = (eq34_e1199 * locals.var_qfd_dn2);
        let eq34_e1202_d_n7: f64 = (eq34_e1199 * locals.var_qfd_dn7);
        let eq34_e1202_q: f64 = (eq34_e1199 * eq34_e1201_q);
        stamper.stamp_current_reactive_node3(
            Some(nodes[7]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (eq34_e1202_d_n0),
            nodes[2],
            multiplicity * (eq34_e1202_d_n2),
            nodes[7],
            multiplicity * (eq34_e1202_d_n7),
        );
        let eq35_e1204: f64 = (-p.p87);
        let eq35_e1206_q: f64 = locals.var_qfs;
        let eq35_e1207: f64 = (eq35_e1204 * locals.var_qfs);
        let eq35_e1207_d_n2: f64 = (eq35_e1204 * locals.var_qfs_dn2);
        let eq35_e1207_d_n7: f64 = (eq35_e1204 * locals.var_qfs_dn7);
        let eq35_e1207_q: f64 = (eq35_e1204 * eq35_e1206_q);
        stamper.stamp_current_reactive_node2(
            Some(nodes[7]),
            Some(nodes[2]),
            nodes[2],
            multiplicity * (eq35_e1207_d_n2),
            nodes[7],
            multiplicity * (eq35_e1207_d_n7),
        );
        let eq41_e1236: f64 = ((nv15 - 0.0) * locals.var_sigrat_s);
        let eq41_e1236_d_n0: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn0);
        let eq41_e1236_d_n2: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn2);
        let eq41_e1236_d_n4: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn4);
        let eq41_e1236_d_n5: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn5);
        let eq41_e1236_d_n6: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn6);
        let eq41_e1236_d_n7: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn7);
        let eq41_e1236_d_n8: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn8);
        let eq41_e1236_d_n9: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn9);
        let eq41_e1236_d_n10: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn10);
        let eq41_e1236_d_n11: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn11);
        let eq41_e1236_d_n14: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn14);
        let eq41_e1237_q: f64 = eq41_e1236;
        let eq41_reactive_node_derivatives: [f64; 19] = [eq41_e1236_d_n0, 0.0, eq41_e1236_d_n2, 0.0, eq41_e1236_d_n4, eq41_e1236_d_n5, eq41_e1236_d_n6, eq41_e1236_d_n7, eq41_e1236_d_n8, eq41_e1236_d_n9, eq41_e1236_d_n10, eq41_e1236_d_n11, 0.0, 0.0, eq41_e1236_d_n14, locals.var_sigrat_s, 0.0, 0.0, 0.0];
        let eq41_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[8]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq42_e1240: f64 = ((nv15 - 0.0) * locals.var_sigrat_d);
        let eq42_e1240_d_n0: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn0);
        let eq42_e1240_d_n2: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn2);
        let eq42_e1240_d_n4: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn4);
        let eq42_e1240_d_n5: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn5);
        let eq42_e1240_d_n6: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn6);
        let eq42_e1240_d_n7: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn7);
        let eq42_e1240_d_n8: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn8);
        let eq42_e1240_d_n9: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn9);
        let eq42_e1240_d_n10: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn10);
        let eq42_e1240_d_n11: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn11);
        let eq42_e1240_d_n14: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn14);
        let eq42_e1241_q: f64 = eq42_e1240;
        let eq42_reactive_node_derivatives: [f64; 19] = [eq42_e1240_d_n0, 0.0, eq42_e1240_d_n2, 0.0, eq42_e1240_d_n4, eq42_e1240_d_n5, eq42_e1240_d_n6, eq42_e1240_d_n7, eq42_e1240_d_n8, eq42_e1240_d_n9, eq42_e1240_d_n10, eq42_e1240_d_n11, 0.0, 0.0, eq42_e1240_d_n14, locals.var_sigrat_d, 0.0, 0.0, 0.0];
        let eq42_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq62_e1362, eq62_e1362_d_n12, eq62_e1362_q,) = {
    if (p.p28 != 0.0) {
        let eq62_e1359: f64 = (locals.var_cqi * (nv12 - 0.0));
        let eq62_e1360_q: f64 = eq62_e1359;
        (eq62_e1359, locals.var_cqi, eq62_e1360_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[12]),
            None,
            nodes[12],
            multiplicity * (eq62_e1362_d_n12),
        );
        let (eq63_e1369, eq63_e1369_d_n13, eq63_e1369_q,) = {
    if (p.p28 != 0.0) {
        let eq63_e1366: f64 = (locals.var_cqb * (nv13 - 0.0));
        let eq63_e1367_q: f64 = eq63_e1366;
        (eq63_e1366, locals.var_cqb, eq63_e1367_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq63_e1369_d_n13),
        );
        let (eq67_e1388, eq67_e1388_d_n14, eq67_e1388_q,) = {
    if (p.p29 != 0.0) {
        let eq67_e1386_q: f64 = (nv14 - 0.0);
        ((nv14 - 0.0), 1.0, eq67_e1386_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[14]),
            None,
            nodes[14],
            multiplicity * (eq67_e1388_d_n14),
        );
    }
}

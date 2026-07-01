#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq31_e687: f64 = (locals.var_gsig * (nv5 - 0.0));
        let eq31_e687_d_n4: f64 = (locals.var_gsig_dn4 * (nv5 - 0.0));
        let eq31_e687_d_n6: f64 = (locals.var_gsig_dn6 * (nv5 - 0.0));
        let eq31_e687_d_n7: f64 = (locals.var_gsig_dn7 * (nv5 - 0.0));
        let eq31_e687_d_n8: f64 = (locals.var_gsig_dn8 * (nv5 - 0.0));
        let eq31_e687_d_n9: f64 = (locals.var_gsig_dn9 * (nv5 - 0.0));
        let eq31_value: f64 = eq31_e687;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            None,
            multiplicity * (eq31_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq31_e687_d_n4), multiplicity * (locals.var_gsig), multiplicity * (eq31_e687_d_n6), multiplicity * (eq31_e687_d_n7), multiplicity * (eq31_e687_d_n8), multiplicity * (eq31_e687_d_n9)],
            [],
            [],
            1.0,
        );
        let eq32_e690: f64 = (locals.var_cgeff * (nv5 - 0.0));
        let eq32_e690_d_n4: f64 = (locals.var_cgeff_dn4 * (nv5 - 0.0));
        let eq32_e690_d_n6: f64 = (locals.var_cgeff_dn6 * (nv5 - 0.0));
        let eq32_e690_d_n7: f64 = (locals.var_cgeff_dn7 * (nv5 - 0.0));
        let eq32_e690_d_n8: f64 = (locals.var_cgeff_dn8 * (nv5 - 0.0));
        let eq32_e690_d_n9: f64 = (locals.var_cgeff_dn9 * (nv5 - 0.0));
        let eq32_e691: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, eq32_e690);
        let eq32_value: f64 = eq32_e691;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            None,
            multiplicity * (eq32_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq32_e690_d_n4 * ddt_scale)), multiplicity * ((locals.var_cgeff * ddt_scale)), multiplicity * ((eq32_e690_d_n6 * ddt_scale)), multiplicity * ((eq32_e690_d_n7 * ddt_scale)), multiplicity * ((eq32_e690_d_n8 * ddt_scale)), multiplicity * ((eq32_e690_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq33_e693: f64 = (-locals.var_csgeff);
        let eq33_e695: f64 = (eq33_e693 * (nv5 - 0.0));
        let eq33_e695_d_n4: f64 = ((-locals.var_csgeff_dn4) * (nv5 - 0.0));
        let eq33_e695_d_n6: f64 = ((-locals.var_csgeff_dn6) * (nv5 - 0.0));
        let eq33_e695_d_n7: f64 = ((-locals.var_csgeff_dn7) * (nv5 - 0.0));
        let eq33_e695_d_n8: f64 = ((-locals.var_csgeff_dn8) * (nv5 - 0.0));
        let eq33_e695_d_n9: f64 = ((-locals.var_csgeff_dn9) * (nv5 - 0.0));
        let eq33_e696: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, eq33_e695);
        let eq33_value: f64 = eq33_e696;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(6),
            multiplicity * (eq33_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq33_e695_d_n4 * ddt_scale)), multiplicity * ((eq33_e693 * ddt_scale)), multiplicity * ((eq33_e695_d_n6 * ddt_scale)), multiplicity * ((eq33_e695_d_n7 * ddt_scale)), multiplicity * ((eq33_e695_d_n8 * ddt_scale)), multiplicity * ((eq33_e695_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq34_e698: f64 = (-locals.var_cdgeff);
        let eq34_e700: f64 = (eq34_e698 * (nv5 - 0.0));
        let eq34_e700_d_n4: f64 = ((-locals.var_cdgeff_dn4) * (nv5 - 0.0));
        let eq34_e700_d_n6: f64 = ((-locals.var_cdgeff_dn6) * (nv5 - 0.0));
        let eq34_e700_d_n7: f64 = ((-locals.var_cdgeff_dn7) * (nv5 - 0.0));
        let eq34_e700_d_n8: f64 = ((-locals.var_cdgeff_dn8) * (nv5 - 0.0));
        let eq34_e700_d_n9: f64 = ((-locals.var_cdgeff_dn9) * (nv5 - 0.0));
        let eq34_e701: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq34_e700);
        let eq34_value: f64 = eq34_e701;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq34_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq34_e700_d_n4 * ddt_scale)), multiplicity * ((eq34_e698 * ddt_scale)), multiplicity * ((eq34_e700_d_n6 * ddt_scale)), multiplicity * ((eq34_e700_d_n7 * ddt_scale)), multiplicity * ((eq34_e700_d_n8 * ddt_scale)), multiplicity * ((eq34_e700_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq23_e631_q: f64 = locals.var_qb;
        let eq23_e633_q: f64 = locals.var_qssub;
        let eq23_e634: f64 = (locals.var_qb + locals.var_qssub);
        let eq23_e634_d_n6: f64 = (locals.var_qb_dn6 + locals.var_qssub_dn6);
        let eq23_e634_d_n8: f64 = (locals.var_qb_dn8 + locals.var_qssub_dn8);
        let eq23_e634_q: f64 = (eq23_e631_q + eq23_e633_q);
        let eq23_e636_q: f64 = locals.var_qbsif;
        let eq23_e637: f64 = (eq23_e634 + locals.var_qbsif);
        let eq23_e637_d_n4: f64 = (locals.var_qb_dn4 + locals.var_qbsif_dn4);
        let eq23_e637_d_n6: f64 = (eq23_e634_d_n6 + locals.var_qbsif_dn6);
        let eq23_e637_d_n7: f64 = (locals.var_qb_dn7 + locals.var_qbsif_dn7);
        let eq23_e637_d_n8: f64 = (eq23_e634_d_n8 + locals.var_qbsif_dn8);
        let eq23_e637_d_n9: f64 = (locals.var_qb_dn9 + locals.var_qbsif_dn9);
        let eq23_e637_q: f64 = (eq23_e634_q + eq23_e636_q);
        let eq23_e638: f64 = (p.p14 * eq23_e637);
        let eq23_e638_d_n4: f64 = (p.p14 * eq23_e637_d_n4);
        let eq23_e638_d_n6: f64 = (p.p14 * eq23_e637_d_n6);
        let eq23_e638_d_n7: f64 = (p.p14 * eq23_e637_d_n7);
        let eq23_e638_d_n8: f64 = (p.p14 * eq23_e637_d_n8);
        let eq23_e638_d_n9: f64 = (p.p14 * eq23_e637_d_n9);
        let eq23_e638_q: f64 = (p.p14 * eq23_e637_q);
        let eq23_reactive_node_derivatives: [f64; 10] = [0.0, 0.0, 0.0, 0.0, eq23_e638_d_n4, 0.0, eq23_e638_d_n6, eq23_e638_d_n7, eq23_e638_d_n8, eq23_e638_d_n9];
        let eq23_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
        let eq24_e641_q: f64 = locals.var_qgde;
        let eq24_e643_q: f64 = locals.var_qovd;
        let eq24_e644: f64 = (locals.var_qgde + locals.var_qovd);
        let eq24_e644_d_n4: f64 = (locals.var_qgde_dn4 + locals.var_qovd_dn4);
        let eq24_e644_d_n6: f64 = (locals.var_qgde_dn6 + locals.var_qovd_dn6);
        let eq24_e644_d_n7: f64 = (locals.var_qgde_dn7 + locals.var_qovd_dn7);
        let eq24_e644_d_n8: f64 = (locals.var_qgde_dn8 + locals.var_qovd_dn8);
        let eq24_e644_d_n9: f64 = (locals.var_qgde_dn9 + locals.var_qovd_dn9);
        let eq24_e644_q: f64 = (eq24_e641_q + eq24_e643_q);
        let eq24_e646_q: f64 = locals.var_qgdif;
        let eq24_e647: f64 = (eq24_e644 + locals.var_qgdif);
        let eq24_e647_d_n4: f64 = (eq24_e644_d_n4 + locals.var_qgdif_dn4);
        let eq24_e647_d_n6: f64 = (eq24_e644_d_n6 + locals.var_qgdif_dn6);
        let eq24_e647_d_n7: f64 = (eq24_e644_d_n7 + locals.var_qgdif_dn7);
        let eq24_e647_d_n8: f64 = (eq24_e644_d_n8 + locals.var_qgdif_dn8);
        let eq24_e647_d_n9: f64 = (eq24_e644_d_n9 + locals.var_qgdif_dn9);
        let eq24_e647_q: f64 = (eq24_e644_q + eq24_e646_q);
        let eq24_e648: f64 = (p.p14 * eq24_e647);
        let eq24_e648_d_n4: f64 = (p.p14 * eq24_e647_d_n4);
        let eq24_e648_d_n6: f64 = (p.p14 * eq24_e647_d_n6);
        let eq24_e648_d_n7: f64 = (p.p14 * eq24_e647_d_n7);
        let eq24_e648_d_n8: f64 = (p.p14 * eq24_e647_d_n8);
        let eq24_e648_d_n9: f64 = (p.p14 * eq24_e647_d_n9);
        let eq24_e648_q: f64 = (p.p14 * eq24_e647_q);
        let eq24_reactive_node_derivatives: [f64; 10] = [0.0, 0.0, 0.0, 0.0, eq24_e648_d_n4, 0.0, eq24_e648_d_n6, eq24_e648_d_n7, eq24_e648_d_n8, eq24_e648_d_n9];
        let eq24_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq24_reactive_node_derivatives,
            branches,
            &eq24_reactive_branch_derivatives,
            multiplicity,
        );
        let eq25_e651_q: f64 = locals.var_qdsub;
        let eq25_e653_q: f64 = locals.var_qbdif;
        let eq25_e654: f64 = (locals.var_qdsub + locals.var_qbdif);
        let eq25_e654_d_n6: f64 = (locals.var_qdsub_dn6 + locals.var_qbdif_dn6);
        let eq25_e654_d_n7: f64 = (locals.var_qdsub_dn7 + locals.var_qbdif_dn7);
        let eq25_e654_d_n8: f64 = (locals.var_qdsub_dn8 + locals.var_qbdif_dn8);
        let eq25_e654_q: f64 = (eq25_e651_q + eq25_e653_q);
        let eq25_e655: f64 = (p.p14 * eq25_e654);
        let eq25_e655_d_n4: f64 = (p.p14 * locals.var_qbdif_dn4);
        let eq25_e655_d_n6: f64 = (p.p14 * eq25_e654_d_n6);
        let eq25_e655_d_n7: f64 = (p.p14 * eq25_e654_d_n7);
        let eq25_e655_d_n8: f64 = (p.p14 * eq25_e654_d_n8);
        let eq25_e655_d_n9: f64 = (p.p14 * locals.var_qbdif_dn9);
        let eq25_e655_q: f64 = (p.p14 * eq25_e654_q);
        let eq25_reactive_node_derivatives: [f64; 10] = [0.0, 0.0, 0.0, 0.0, eq25_e655_d_n4, 0.0, eq25_e655_d_n6, eq25_e655_d_n7, eq25_e655_d_n8, eq25_e655_d_n9];
        let eq25_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq25_reactive_node_derivatives,
            branches,
            &eq25_reactive_branch_derivatives,
            multiplicity,
        );
        let eq26_e658_q: f64 = locals.var_qgbe;
        let eq26_e659: f64 = (p.p14 * locals.var_qgbe);
        let eq26_e659_d_n4: f64 = (p.p14 * locals.var_qgbe_dn4);
        let eq26_e659_d_n6: f64 = (p.p14 * locals.var_qgbe_dn6);
        let eq26_e659_d_n7: f64 = (p.p14 * locals.var_qgbe_dn7);
        let eq26_e659_d_n8: f64 = (p.p14 * locals.var_qgbe_dn8);
        let eq26_e659_d_n9: f64 = (p.p14 * locals.var_qgbe_dn9);
        let eq26_e659_q: f64 = (p.p14 * eq26_e658_q);
        let eq26_reactive_node_derivatives: [f64; 10] = [0.0, 0.0, 0.0, 0.0, eq26_e659_d_n4, 0.0, eq26_e659_d_n6, eq26_e659_d_n7, eq26_e659_d_n8, eq26_e659_d_n9];
        let eq26_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq26_reactive_node_derivatives,
            branches,
            &eq26_reactive_branch_derivatives,
            multiplicity,
        );
        let eq27_e662_q: f64 = locals.var_qg;
        let eq27_e664_q: f64 = locals.var_qgse;
        let eq27_e665: f64 = (locals.var_qg + locals.var_qgse);
        let eq27_e665_d_n4: f64 = (locals.var_qg_dn4 + locals.var_qgse_dn4);
        let eq27_e665_d_n6: f64 = (locals.var_qg_dn6 + locals.var_qgse_dn6);
        let eq27_e665_d_n7: f64 = (locals.var_qg_dn7 + locals.var_qgse_dn7);
        let eq27_e665_d_n8: f64 = (locals.var_qg_dn8 + locals.var_qgse_dn8);
        let eq27_e665_d_n9: f64 = (locals.var_qg_dn9 + locals.var_qgse_dn9);
        let eq27_e665_q: f64 = (eq27_e662_q + eq27_e664_q);
        let eq27_e667_q: f64 = locals.var_qovs;
        let eq27_e668: f64 = (eq27_e665 + locals.var_qovs);
        let eq27_e668_d_n4: f64 = (eq27_e665_d_n4 + locals.var_qovs_dn4);
        let eq27_e668_d_n6: f64 = (eq27_e665_d_n6 + locals.var_qovs_dn6);
        let eq27_e668_d_n7: f64 = (eq27_e665_d_n7 + locals.var_qovs_dn7);
        let eq27_e668_d_n8: f64 = (eq27_e665_d_n8 + locals.var_qovs_dn8);
        let eq27_e668_d_n9: f64 = (eq27_e665_d_n9 + locals.var_qovs_dn9);
        let eq27_e668_q: f64 = (eq27_e665_q + eq27_e667_q);
        let eq27_e670_q: f64 = locals.var_qgsif;
        let eq27_e671: f64 = (eq27_e668 + locals.var_qgsif);
        let eq27_e671_d_n4: f64 = (eq27_e668_d_n4 + locals.var_qgsif_dn4);
        let eq27_e671_d_n6: f64 = (eq27_e668_d_n6 + locals.var_qgsif_dn6);
        let eq27_e671_d_n7: f64 = (eq27_e668_d_n7 + locals.var_qgsif_dn7);
        let eq27_e671_d_n8: f64 = (eq27_e668_d_n8 + locals.var_qgsif_dn8);
        let eq27_e671_d_n9: f64 = (eq27_e668_d_n9 + locals.var_qgsif_dn9);
        let eq27_e671_q: f64 = (eq27_e668_q + eq27_e670_q);
        let eq27_e672: f64 = (p.p14 * eq27_e671);
        let eq27_e672_d_n4: f64 = (p.p14 * eq27_e671_d_n4);
        let eq27_e672_d_n6: f64 = (p.p14 * eq27_e671_d_n6);
        let eq27_e672_d_n7: f64 = (p.p14 * eq27_e671_d_n7);
        let eq27_e672_d_n8: f64 = (p.p14 * eq27_e671_d_n8);
        let eq27_e672_d_n9: f64 = (p.p14 * eq27_e671_d_n9);
        let eq27_e672_q: f64 = (p.p14 * eq27_e671_q);
        let eq27_reactive_node_derivatives: [f64; 10] = [0.0, 0.0, 0.0, 0.0, eq27_e672_d_n4, 0.0, eq27_e672_d_n6, eq27_e672_d_n7, eq27_e672_d_n8, eq27_e672_d_n9];
        let eq27_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            nodes,
            &eq27_reactive_node_derivatives,
            branches,
            &eq27_reactive_branch_derivatives,
            multiplicity,
        );
        let eq28_e675_q: f64 = locals.var_qd;
        let eq28_e677_q: f64 = locals.var_qdse;
        let eq28_e678: f64 = (locals.var_qd + locals.var_qdse);
        let eq28_e678_d_n6: f64 = (locals.var_qd_dn6 + locals.var_qdse_dn6);
        let eq28_e678_d_n7: f64 = (locals.var_qd_dn7 + locals.var_qdse_dn7);
        let eq28_e678_q: f64 = (eq28_e675_q + eq28_e677_q);
        let eq28_e679: f64 = (p.p14 * eq28_e678);
        let eq28_e679_d_n4: f64 = (p.p14 * locals.var_qd_dn4);
        let eq28_e679_d_n6: f64 = (p.p14 * eq28_e678_d_n6);
        let eq28_e679_d_n7: f64 = (p.p14 * eq28_e678_d_n7);
        let eq28_e679_d_n8: f64 = (p.p14 * locals.var_qd_dn8);
        let eq28_e679_d_n9: f64 = (p.p14 * locals.var_qd_dn9);
        let eq28_e679_q: f64 = (p.p14 * eq28_e678_q);
        let eq28_reactive_node_derivatives: [f64; 10] = [0.0, 0.0, 0.0, 0.0, eq28_e679_d_n4, 0.0, eq28_e679_d_n6, eq28_e679_d_n7, eq28_e679_d_n8, eq28_e679_d_n9];
        let eq28_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
        let eq32_e690: f64 = (locals.var_cgeff * (nv5 - 0.0));
        let eq32_e690_d_n4: f64 = (locals.var_cgeff_dn4 * (nv5 - 0.0));
        let eq32_e690_d_n6: f64 = (locals.var_cgeff_dn6 * (nv5 - 0.0));
        let eq32_e690_d_n7: f64 = (locals.var_cgeff_dn7 * (nv5 - 0.0));
        let eq32_e690_d_n8: f64 = (locals.var_cgeff_dn8 * (nv5 - 0.0));
        let eq32_e690_d_n9: f64 = (locals.var_cgeff_dn9 * (nv5 - 0.0));
        let eq32_e691_q: f64 = eq32_e690;
        let eq32_reactive_node_derivatives: [f64; 10] = [0.0, 0.0, 0.0, 0.0, eq32_e690_d_n4, locals.var_cgeff, eq32_e690_d_n6, eq32_e690_d_n7, eq32_e690_d_n8, eq32_e690_d_n9];
        let eq32_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq32_reactive_node_derivatives,
            branches,
            &eq32_reactive_branch_derivatives,
            multiplicity,
        );
        let eq33_e693: f64 = (-locals.var_csgeff);
        let eq33_e695: f64 = (eq33_e693 * (nv5 - 0.0));
        let eq33_e695_d_n4: f64 = ((-locals.var_csgeff_dn4) * (nv5 - 0.0));
        let eq33_e695_d_n6: f64 = ((-locals.var_csgeff_dn6) * (nv5 - 0.0));
        let eq33_e695_d_n7: f64 = ((-locals.var_csgeff_dn7) * (nv5 - 0.0));
        let eq33_e695_d_n8: f64 = ((-locals.var_csgeff_dn8) * (nv5 - 0.0));
        let eq33_e695_d_n9: f64 = ((-locals.var_csgeff_dn9) * (nv5 - 0.0));
        let eq33_e696_q: f64 = eq33_e695;
        let eq33_reactive_node_derivatives: [f64; 10] = [0.0, 0.0, 0.0, 0.0, eq33_e695_d_n4, eq33_e693, eq33_e695_d_n6, eq33_e695_d_n7, eq33_e695_d_n8, eq33_e695_d_n9];
        let eq33_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            nodes,
            &eq33_reactive_node_derivatives,
            branches,
            &eq33_reactive_branch_derivatives,
            multiplicity,
        );
        let eq34_e698: f64 = (-locals.var_cdgeff);
        let eq34_e700: f64 = (eq34_e698 * (nv5 - 0.0));
        let eq34_e700_d_n4: f64 = ((-locals.var_cdgeff_dn4) * (nv5 - 0.0));
        let eq34_e700_d_n6: f64 = ((-locals.var_cdgeff_dn6) * (nv5 - 0.0));
        let eq34_e700_d_n7: f64 = ((-locals.var_cdgeff_dn7) * (nv5 - 0.0));
        let eq34_e700_d_n8: f64 = ((-locals.var_cdgeff_dn8) * (nv5 - 0.0));
        let eq34_e700_d_n9: f64 = ((-locals.var_cdgeff_dn9) * (nv5 - 0.0));
        let eq34_e701_q: f64 = eq34_e700;
        let eq34_reactive_node_derivatives: [f64; 10] = [0.0, 0.0, 0.0, 0.0, eq34_e700_d_n4, eq34_e698, eq34_e700_d_n6, eq34_e700_d_n7, eq34_e700_d_n8, eq34_e700_d_n9];
        let eq34_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
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

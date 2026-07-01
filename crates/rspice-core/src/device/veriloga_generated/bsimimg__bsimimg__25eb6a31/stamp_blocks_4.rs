#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_cth: f64,
        var_devsign: f64,
        var_guard147: f64,
        var_qbgi: f64,
        var_qbgi_dn3: f64,
        var_qbgi_dn4: f64,
        var_qbgi_dn5: f64,
        var_qbgi_dn6: f64,
        var_qbgi_dn7: f64,
        var_qbgi_dn8: f64,
        var_qdbg: f64,
        var_qdbg_dn3: f64,
        var_qdbg_dn4: f64,
        var_qdbg_dn5: f64,
        var_qdbg_dn6: f64,
        var_qdbg_dn7: f64,
        var_qdbg_dn8: f64,
        var_qdi: f64,
        var_qdi_dn3: f64,
        var_qdi_dn4: f64,
        var_qdi_dn5: f64,
        var_qdi_dn6: f64,
        var_qdi_dn7: f64,
        var_qdi_dn8: f64,
        var_qfgd_parasitic: f64,
        var_qfgd_parasitic_dn3: f64,
        var_qfgd_parasitic_dn4: f64,
        var_qfgd_parasitic_dn5: f64,
        var_qfgd_parasitic_dn6: f64,
        var_qfgd_parasitic_dn7: f64,
        var_qfgd_parasitic_dn8: f64,
        var_qfgi: f64,
        var_qfgi_dn3: f64,
        var_qfgi_dn4: f64,
        var_qfgi_dn5: f64,
        var_qfgi_dn6: f64,
        var_qfgi_dn7: f64,
        var_qfgi_dn8: f64,
        var_qfgs_parasitic: f64,
        var_qfgs_parasitic_dn3: f64,
        var_qfgs_parasitic_dn4: f64,
        var_qfgs_parasitic_dn5: f64,
        var_qfgs_parasitic_dn6: f64,
        var_qfgs_parasitic_dn7: f64,
        var_qfgs_parasitic_dn8: f64,
        var_qsbg: f64,
        var_qsbg_dn3: f64,
        var_qsbg_dn4: f64,
        var_qsbg_dn5: f64,
        var_qsbg_dn6: f64,
        var_qsbg_dn7: f64,
        var_qsbg_dn8: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq12_e873_q: f64 = var_qdi;
        let eq12_e874: f64 = (var_devsign * var_qdi);
        let eq12_e874_d_n3: f64 = (var_devsign * var_qdi_dn3);
        let eq12_e874_d_n4: f64 = (var_devsign * var_qdi_dn4);
        let eq12_e874_d_n5: f64 = (var_devsign * var_qdi_dn5);
        let eq12_e874_d_n6: f64 = (var_devsign * var_qdi_dn6);
        let eq12_e874_d_n7: f64 = (var_devsign * var_qdi_dn7);
        let eq12_e874_d_n8: f64 = (var_devsign * var_qdi_dn8);
        let eq12_e874_q: f64 = (var_devsign * eq12_e873_q);
        let eq12_reactive_node_derivatives: [f64; 9] = [0.0, 0.0, 0.0, eq12_e874_d_n3, eq12_e874_d_n4, eq12_e874_d_n5, eq12_e874_d_n6, eq12_e874_d_n7, eq12_e874_d_n8];
        let eq12_reactive_branch_derivatives: [f64; 5] = [0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq13_e876_q: f64 = var_qfgi;
        let eq13_reactive_node_derivatives: [f64; 9] = [0.0, 0.0, 0.0, var_qfgi_dn3, var_qfgi_dn4, var_qfgi_dn5, var_qfgi_dn6, var_qfgi_dn7, var_qfgi_dn8];
        let eq13_reactive_branch_derivatives: [f64; 5] = [0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq13_reactive_node_derivatives,
            branches,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );
        let eq14_e879_q: f64 = var_qbgi;
        let eq14_e880: f64 = (var_devsign * var_qbgi);
        let eq14_e880_d_n3: f64 = (var_devsign * var_qbgi_dn3);
        let eq14_e880_d_n4: f64 = (var_devsign * var_qbgi_dn4);
        let eq14_e880_d_n5: f64 = (var_devsign * var_qbgi_dn5);
        let eq14_e880_d_n6: f64 = (var_devsign * var_qbgi_dn6);
        let eq14_e880_d_n7: f64 = (var_devsign * var_qbgi_dn7);
        let eq14_e880_d_n8: f64 = (var_devsign * var_qbgi_dn8);
        let eq14_e880_q: f64 = (var_devsign * eq14_e879_q);
        let eq14_reactive_node_derivatives: [f64; 9] = [0.0, 0.0, 0.0, eq14_e880_d_n3, eq14_e880_d_n4, eq14_e880_d_n5, eq14_e880_d_n6, eq14_e880_d_n7, eq14_e880_d_n8];
        let eq14_reactive_branch_derivatives: [f64; 5] = [0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[6]),
            nodes,
            &eq14_reactive_node_derivatives,
            branches,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
        let eq15_e882_q: f64 = var_qfgs_parasitic;
        let eq15_reactive_node_derivatives: [f64; 9] = [0.0, 0.0, 0.0, var_qfgs_parasitic_dn3, var_qfgs_parasitic_dn4, var_qfgs_parasitic_dn5, var_qfgs_parasitic_dn6, var_qfgs_parasitic_dn7, var_qfgs_parasitic_dn8];
        let eq15_reactive_branch_derivatives: [f64; 5] = [0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq16_e884_q: f64 = var_qfgd_parasitic;
        let eq16_reactive_node_derivatives: [f64; 9] = [0.0, 0.0, 0.0, var_qfgd_parasitic_dn3, var_qfgd_parasitic_dn4, var_qfgd_parasitic_dn5, var_qfgd_parasitic_dn6, var_qfgd_parasitic_dn7, var_qfgd_parasitic_dn8];
        let eq16_reactive_branch_derivatives: [f64; 5] = [0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq16_reactive_node_derivatives,
            branches,
            &eq16_reactive_branch_derivatives,
            multiplicity,
        );
        let eq17_e887_q: f64 = var_qsbg;
        let eq17_e888: f64 = (var_devsign * var_qsbg);
        let eq17_e888_d_n3: f64 = (var_devsign * var_qsbg_dn3);
        let eq17_e888_d_n4: f64 = (var_devsign * var_qsbg_dn4);
        let eq17_e888_d_n5: f64 = (var_devsign * var_qsbg_dn5);
        let eq17_e888_d_n6: f64 = (var_devsign * var_qsbg_dn6);
        let eq17_e888_d_n7: f64 = (var_devsign * var_qsbg_dn7);
        let eq17_e888_d_n8: f64 = (var_devsign * var_qsbg_dn8);
        let eq17_e888_q: f64 = (var_devsign * eq17_e887_q);
        let eq17_reactive_node_derivatives: [f64; 9] = [0.0, 0.0, 0.0, eq17_e888_d_n3, eq17_e888_d_n4, eq17_e888_d_n5, eq17_e888_d_n6, eq17_e888_d_n7, eq17_e888_d_n8];
        let eq17_reactive_branch_derivatives: [f64; 5] = [0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[3]),
            nodes,
            &eq17_reactive_node_derivatives,
            branches,
            &eq17_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e891_q: f64 = var_qdbg;
        let eq18_e892: f64 = (var_devsign * var_qdbg);
        let eq18_e892_d_n3: f64 = (var_devsign * var_qdbg_dn3);
        let eq18_e892_d_n4: f64 = (var_devsign * var_qdbg_dn4);
        let eq18_e892_d_n5: f64 = (var_devsign * var_qdbg_dn5);
        let eq18_e892_d_n6: f64 = (var_devsign * var_qdbg_dn6);
        let eq18_e892_d_n7: f64 = (var_devsign * var_qdbg_dn7);
        let eq18_e892_d_n8: f64 = (var_devsign * var_qdbg_dn8);
        let eq18_e892_q: f64 = (var_devsign * eq18_e891_q);
        let eq18_reactive_node_derivatives: [f64; 9] = [0.0, 0.0, 0.0, eq18_e892_d_n3, eq18_e892_d_n4, eq18_e892_d_n5, eq18_e892_d_n6, eq18_e892_d_n7, eq18_e892_d_n8];
        let eq18_reactive_branch_derivatives: [f64; 5] = [0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[3]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq41_e1106, eq41_e1106_d_n4, eq41_e1106_q,) = {
    if (var_guard147 != 0.0) {
        let eq41_e1103: f64 = ((nv4 - 0.0) * var_cth);
        let eq41_e1104_q: f64 = eq41_e1103;
        (eq41_e1103, var_cth, eq41_e1104_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (eq41_e1106_d_n4),
        );
    }
}

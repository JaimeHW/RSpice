#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_2(
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
        var_gem_n: f64,
        var_gem_n_dn0: f64,
        var_gem_n_dn1: f64,
        var_gem_n_dn10: f64,
        var_gem_n_dn3: f64,
        var_gem_n_dn4: f64,
        var_gem_n_dn5: f64,
        var_gem_n_dn6: f64,
        var_gem_n_dn7: f64,
        var_gem_n_dn8: f64,
        var_gem_n_dn9: f64,
        var_taun: f64,
        var_taun_dn0: f64,
        var_taun_dn1: f64,
        var_taun_dn10: f64,
        var_taun_dn3: f64,
        var_taun_dn4: f64,
        var_taun_dn5: f64,
        var_taun_dn6: f64,
        var_taun_dn7: f64,
        var_taun_dn8: f64,
        var_taun_dn9: f64,
    ) {
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq30_e367: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, (nv11 - 0.0));
        let eq30_e368: f64 = (var_taun * eq30_e367);
        let eq30_e368_d_n0: f64 = (var_taun_dn0 * eq30_e367);
        let eq30_e368_d_n1: f64 = (var_taun_dn1 * eq30_e367);
        let eq30_e368_d_n3: f64 = (var_taun_dn3 * eq30_e367);
        let eq30_e368_d_n4: f64 = (var_taun_dn4 * eq30_e367);
        let eq30_e368_d_n5: f64 = (var_taun_dn5 * eq30_e367);
        let eq30_e368_d_n6: f64 = (var_taun_dn6 * eq30_e367);
        let eq30_e368_d_n7: f64 = (var_taun_dn7 * eq30_e367);
        let eq30_e368_d_n8: f64 = (var_taun_dn8 * eq30_e367);
        let eq30_e368_d_n9: f64 = (var_taun_dn9 * eq30_e367);
        let eq30_e368_d_n10: f64 = (var_taun_dn10 * eq30_e367);
        let eq30_value: f64 = eq30_e368;
        let eq30_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq30_node_derivatives: [f64; 11] = [eq30_e368_d_n0, eq30_e368_d_n1, eq30_e368_d_n3, eq30_e368_d_n4, eq30_e368_d_n5, eq30_e368_d_n6, eq30_e368_d_n7, eq30_e368_d_n8, eq30_e368_d_n9, eq30_e368_d_n10, (var_taun * ddt_scale)];
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
        let eq31_e371: f64 = (var_gem_n * (nv11 - 0.0));
        let eq31_e371_d_n0: f64 = (var_gem_n_dn0 * (nv11 - 0.0));
        let eq31_e371_d_n1: f64 = (var_gem_n_dn1 * (nv11 - 0.0));
        let eq31_e371_d_n3: f64 = (var_gem_n_dn3 * (nv11 - 0.0));
        let eq31_e371_d_n4: f64 = (var_gem_n_dn4 * (nv11 - 0.0));
        let eq31_e371_d_n5: f64 = (var_gem_n_dn5 * (nv11 - 0.0));
        let eq31_e371_d_n6: f64 = (var_gem_n_dn6 * (nv11 - 0.0));
        let eq31_e371_d_n7: f64 = (var_gem_n_dn7 * (nv11 - 0.0));
        let eq31_e371_d_n8: f64 = (var_gem_n_dn8 * (nv11 - 0.0));
        let eq31_e371_d_n9: f64 = (var_gem_n_dn9 * (nv11 - 0.0));
        let eq31_e371_d_n10: f64 = (var_gem_n_dn10 * (nv11 - 0.0));
        let eq31_value: f64 = eq31_e371;
        let eq31_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq31_node_derivatives: [f64; 11] = [eq31_e371_d_n0, eq31_e371_d_n1, eq31_e371_d_n3, eq31_e371_d_n4, eq31_e371_d_n5, eq31_e371_d_n6, eq31_e371_d_n7, eq31_e371_d_n8, eq31_e371_d_n9, eq31_e371_d_n10, var_gem_n];
        let eq31_branch_derivative_indices: [usize; 0] = [];
        let eq31_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq31_value),
            &eq31_node_derivative_indices,
            &eq31_node_derivatives,
            &eq31_branch_derivative_indices,
            &eq31_branch_derivatives,
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
        var_i_cth_rdb0: f64,
        var_i_cth_rdb1: f64,
        var_i_cth_rdn0: f64,
        var_i_cth_rdn1: f64,
        var_i_cth_rdn10: f64,
        var_i_cth_rdn11: f64,
        var_i_cth_rdn2: f64,
        var_i_cth_rdn3: f64,
        var_i_cth_rdn4: f64,
        var_i_cth_rdn5: f64,
        var_i_cth_rdn6: f64,
        var_i_cth_rdn7: f64,
        var_i_cth_rdn8: f64,
        var_i_cth_rdn9: f64,
        var_i_cth_rv: f64,
        var_qb1b2: f64,
        var_qb1b2_db0: f64,
        var_qb1b2_db1: f64,
        var_qb1b2_dn0: f64,
        var_qb1b2_dn1: f64,
        var_qb1b2_dn10: f64,
        var_qb1b2_dn11: f64,
        var_qb1b2_dn2: f64,
        var_qb1b2_dn3: f64,
        var_qb1b2_dn4: f64,
        var_qb1b2_dn5: f64,
        var_qb1b2_dn6: f64,
        var_qb1b2_dn7: f64,
        var_qb1b2_dn8: f64,
        var_qb1b2_dn9: f64,
        var_qbc: f64,
        var_qbc_db0: f64,
        var_qbc_db1: f64,
        var_qbc_dn0: f64,
        var_qbc_dn1: f64,
        var_qbc_dn10: f64,
        var_qbc_dn11: f64,
        var_qbc_dn2: f64,
        var_qbc_dn3: f64,
        var_qbc_dn4: f64,
        var_qbc_dn5: f64,
        var_qbc_dn6: f64,
        var_qbc_dn7: f64,
        var_qbc_dn8: f64,
        var_qbc_dn9: f64,
        var_qbe: f64,
        var_qbe_db0: f64,
        var_qbe_db1: f64,
        var_qbe_dn0: f64,
        var_qbe_dn1: f64,
        var_qbe_dn10: f64,
        var_qbe_dn11: f64,
        var_qbe_dn2: f64,
        var_qbe_dn3: f64,
        var_qbe_dn4: f64,
        var_qbe_dn5: f64,
        var_qbe_dn6: f64,
        var_qbe_dn7: f64,
        var_qbe_dn8: f64,
        var_qbe_dn9: f64,
        var_qe: f64,
        var_qe_db0: f64,
        var_qe_db1: f64,
        var_qe_dn0: f64,
        var_qe_dn1: f64,
        var_qe_dn10: f64,
        var_qe_dn11: f64,
        var_qe_dn2: f64,
        var_qe_dn3: f64,
        var_qe_dn4: f64,
        var_qe_dn5: f64,
        var_qe_dn6: f64,
        var_qe_dn7: f64,
        var_qe_dn8: f64,
        var_qe_dn9: f64,
        var_qepi: f64,
        var_qepi_db0: f64,
        var_qepi_db1: f64,
        var_qepi_dn0: f64,
        var_qepi_dn1: f64,
        var_qepi_dn10: f64,
        var_qepi_dn11: f64,
        var_qepi_dn2: f64,
        var_qepi_dn3: f64,
        var_qepi_dn4: f64,
        var_qepi_dn5: f64,
        var_qepi_dn6: f64,
        var_qepi_dn7: f64,
        var_qepi_dn8: f64,
        var_qepi_dn9: f64,
        var_qex: f64,
        var_qex_db0: f64,
        var_qex_db1: f64,
        var_qex_dn0: f64,
        var_qex_dn1: f64,
        var_qex_dn10: f64,
        var_qex_dn11: f64,
        var_qex_dn2: f64,
        var_qex_dn3: f64,
        var_qex_dn4: f64,
        var_qex_dn5: f64,
        var_qex_dn6: f64,
        var_qex_dn7: f64,
        var_qex_dn8: f64,
        var_qex_dn9: f64,
        var_qtc: f64,
        var_qtc_db0: f64,
        var_qtc_db1: f64,
        var_qtc_dn0: f64,
        var_qtc_dn1: f64,
        var_qtc_dn10: f64,
        var_qtc_dn11: f64,
        var_qtc_dn2: f64,
        var_qtc_dn3: f64,
        var_qtc_dn4: f64,
        var_qtc_dn5: f64,
        var_qtc_dn6: f64,
        var_qtc_dn7: f64,
        var_qtc_dn8: f64,
        var_qtc_dn9: f64,
        var_qte: f64,
        var_qte_db0: f64,
        var_qte_db1: f64,
        var_qte_dn0: f64,
        var_qte_dn1: f64,
        var_qte_dn10: f64,
        var_qte_dn11: f64,
        var_qte_dn2: f64,
        var_qte_dn3: f64,
        var_qte_dn4: f64,
        var_qte_dn5: f64,
        var_qte_dn6: f64,
        var_qte_dn7: f64,
        var_qte_dn8: f64,
        var_qte_dn9: f64,
        var_qte_s: f64,
        var_qte_s_db0: f64,
        var_qte_s_db1: f64,
        var_qte_s_dn0: f64,
        var_qte_s_dn1: f64,
        var_qte_s_dn10: f64,
        var_qte_s_dn11: f64,
        var_qte_s_dn2: f64,
        var_qte_s_dn3: f64,
        var_qte_s_dn4: f64,
        var_qte_s_dn5: f64,
        var_qte_s_dn6: f64,
        var_qte_s_dn7: f64,
        var_qte_s_dn8: f64,
        var_qte_s_dn9: f64,
        var_qtex: f64,
        var_qtex_db0: f64,
        var_qtex_db1: f64,
        var_qtex_dn0: f64,
        var_qtex_dn1: f64,
        var_qtex_dn10: f64,
        var_qtex_dn11: f64,
        var_qtex_dn2: f64,
        var_qtex_dn3: f64,
        var_qtex_dn4: f64,
        var_qtex_dn5: f64,
        var_qtex_dn6: f64,
        var_qtex_dn7: f64,
        var_qtex_dn8: f64,
        var_qtex_dn9: f64,
        var_taun: f64,
        var_taun_db0: f64,
        var_taun_db1: f64,
        var_taun_dn0: f64,
        var_taun_dn1: f64,
        var_taun_dn10: f64,
        var_taun_dn11: f64,
        var_taun_dn2: f64,
        var_taun_dn3: f64,
        var_taun_dn4: f64,
        var_taun_dn5: f64,
        var_taun_dn6: f64,
        var_taun_dn7: f64,
        var_taun_dn8: f64,
        var_taun_dn9: f64,
        var_vbc: f64,
        var_vbc_db0: f64,
        var_vbc_db1: f64,
        var_vbc_dn0: f64,
        var_vbc_dn1: f64,
        var_vbc_dn10: f64,
        var_vbc_dn11: f64,
        var_vbc_dn2: f64,
        var_vbc_dn3: f64,
        var_vbc_dn4: f64,
        var_vbc_dn5: f64,
        var_vbc_dn6: f64,
        var_vbc_dn7: f64,
        var_vbc_dn8: f64,
        var_vbc_dn9: f64,
        var_vbe: f64,
        var_vbe_db0: f64,
        var_vbe_db1: f64,
        var_vbe_dn0: f64,
        var_vbe_dn1: f64,
        var_vbe_dn10: f64,
        var_vbe_dn11: f64,
        var_vbe_dn2: f64,
        var_vbe_dn3: f64,
        var_vbe_dn4: f64,
        var_vbe_dn5: f64,
        var_vbe_dn6: f64,
        var_vbe_dn7: f64,
        var_vbe_dn8: f64,
        var_vbe_dn9: f64,
        var_xqex: f64,
        var_xqex_db0: f64,
        var_xqex_db1: f64,
        var_xqex_dn0: f64,
        var_xqex_dn1: f64,
        var_xqex_dn10: f64,
        var_xqex_dn11: f64,
        var_xqex_dn2: f64,
        var_xqex_dn3: f64,
        var_xqex_dn4: f64,
        var_xqex_dn5: f64,
        var_xqex_dn6: f64,
        var_xqex_dn7: f64,
        var_xqex_dn8: f64,
        var_xqex_dn9: f64,
        var_xqtex: f64,
        var_xqtex_db0: f64,
        var_xqtex_db1: f64,
        var_xqtex_dn0: f64,
        var_xqtex_dn1: f64,
        var_xqtex_dn10: f64,
        var_xqtex_dn11: f64,
        var_xqtex_dn2: f64,
        var_xqtex_dn3: f64,
        var_xqtex_dn4: f64,
        var_xqtex_dn5: f64,
        var_xqtex_dn6: f64,
        var_xqtex_dn7: f64,
        var_xqtex_dn8: f64,
        var_xqtex_dn9: f64,
    ) {
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq11_e235_q: f64 = var_i_cth_rv;
        let eq11_reactive_node_derivatives: [f64; 12] = [var_i_cth_rdn0, var_i_cth_rdn1, var_i_cth_rdn2, var_i_cth_rdn3, var_i_cth_rdn4, var_i_cth_rdn5, var_i_cth_rdn6, var_i_cth_rdn7, var_i_cth_rdn8, var_i_cth_rdn9, var_i_cth_rdn10, var_i_cth_rdn11];
        let eq11_reactive_branch_derivatives: [f64; 2] = [var_i_cth_rdb0, var_i_cth_rdb1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            None,
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let eq13_e245: f64 = (var_qte + var_qbe);
        let eq13_e245_d_n0: f64 = (var_qte_dn0 + var_qbe_dn0);
        let eq13_e245_d_n1: f64 = (var_qte_dn1 + var_qbe_dn1);
        let eq13_e245_d_n2: f64 = (var_qte_dn2 + var_qbe_dn2);
        let eq13_e245_d_n3: f64 = (var_qte_dn3 + var_qbe_dn3);
        let eq13_e245_d_n4: f64 = (var_qte_dn4 + var_qbe_dn4);
        let eq13_e245_d_n5: f64 = (var_qte_dn5 + var_qbe_dn5);
        let eq13_e245_d_n6: f64 = (var_qte_dn6 + var_qbe_dn6);
        let eq13_e245_d_n7: f64 = (var_qte_dn7 + var_qbe_dn7);
        let eq13_e245_d_n8: f64 = (var_qte_dn8 + var_qbe_dn8);
        let eq13_e245_d_n9: f64 = (var_qte_dn9 + var_qbe_dn9);
        let eq13_e245_d_n10: f64 = (var_qte_dn10 + var_qbe_dn10);
        let eq13_e245_d_n11: f64 = (var_qte_dn11 + var_qbe_dn11);
        let eq13_e245_d_b0: f64 = (var_qte_db0 + var_qbe_db0);
        let eq13_e245_d_b1: f64 = (var_qte_db1 + var_qbe_db1);
        let eq13_e247: f64 = (eq13_e245 + var_qe);
        let eq13_e247_d_n0: f64 = (eq13_e245_d_n0 + var_qe_dn0);
        let eq13_e247_d_n1: f64 = (eq13_e245_d_n1 + var_qe_dn1);
        let eq13_e247_d_n2: f64 = (eq13_e245_d_n2 + var_qe_dn2);
        let eq13_e247_d_n3: f64 = (eq13_e245_d_n3 + var_qe_dn3);
        let eq13_e247_d_n4: f64 = (eq13_e245_d_n4 + var_qe_dn4);
        let eq13_e247_d_n5: f64 = (eq13_e245_d_n5 + var_qe_dn5);
        let eq13_e247_d_n6: f64 = (eq13_e245_d_n6 + var_qe_dn6);
        let eq13_e247_d_n7: f64 = (eq13_e245_d_n7 + var_qe_dn7);
        let eq13_e247_d_n8: f64 = (eq13_e245_d_n8 + var_qe_dn8);
        let eq13_e247_d_n9: f64 = (eq13_e245_d_n9 + var_qe_dn9);
        let eq13_e247_d_n10: f64 = (eq13_e245_d_n10 + var_qe_dn10);
        let eq13_e247_d_n11: f64 = (eq13_e245_d_n11 + var_qe_dn11);
        let eq13_e247_d_b0: f64 = (eq13_e245_d_b0 + var_qe_db0);
        let eq13_e247_d_b1: f64 = (eq13_e245_d_b1 + var_qe_db1);
        let eq13_e248: f64 = (p.p3 * eq13_e247);
        let eq13_e248_d_n0: f64 = (p.p3 * eq13_e247_d_n0);
        let eq13_e248_d_n1: f64 = (p.p3 * eq13_e247_d_n1);
        let eq13_e248_d_n2: f64 = (p.p3 * eq13_e247_d_n2);
        let eq13_e248_d_n3: f64 = (p.p3 * eq13_e247_d_n3);
        let eq13_e248_d_n4: f64 = (p.p3 * eq13_e247_d_n4);
        let eq13_e248_d_n5: f64 = (p.p3 * eq13_e247_d_n5);
        let eq13_e248_d_n6: f64 = (p.p3 * eq13_e247_d_n6);
        let eq13_e248_d_n7: f64 = (p.p3 * eq13_e247_d_n7);
        let eq13_e248_d_n8: f64 = (p.p3 * eq13_e247_d_n8);
        let eq13_e248_d_n9: f64 = (p.p3 * eq13_e247_d_n9);
        let eq13_e248_d_n10: f64 = (p.p3 * eq13_e247_d_n10);
        let eq13_e248_d_n11: f64 = (p.p3 * eq13_e247_d_n11);
        let eq13_e248_d_b0: f64 = (p.p3 * eq13_e247_d_b0);
        let eq13_e248_d_b1: f64 = (p.p3 * eq13_e247_d_b1);
        let eq13_e249_q: f64 = eq13_e248;
        let eq13_e251: f64 = (eq13_e248 * p.p1);
        let eq13_e251_d_n0: f64 = (eq13_e248_d_n0 * p.p1);
        let eq13_e251_d_n1: f64 = (eq13_e248_d_n1 * p.p1);
        let eq13_e251_d_n2: f64 = (eq13_e248_d_n2 * p.p1);
        let eq13_e251_d_n3: f64 = (eq13_e248_d_n3 * p.p1);
        let eq13_e251_d_n4: f64 = (eq13_e248_d_n4 * p.p1);
        let eq13_e251_d_n5: f64 = (eq13_e248_d_n5 * p.p1);
        let eq13_e251_d_n6: f64 = (eq13_e248_d_n6 * p.p1);
        let eq13_e251_d_n7: f64 = (eq13_e248_d_n7 * p.p1);
        let eq13_e251_d_n8: f64 = (eq13_e248_d_n8 * p.p1);
        let eq13_e251_d_n9: f64 = (eq13_e248_d_n9 * p.p1);
        let eq13_e251_d_n10: f64 = (eq13_e248_d_n10 * p.p1);
        let eq13_e251_d_n11: f64 = (eq13_e248_d_n11 * p.p1);
        let eq13_e251_d_b0: f64 = (eq13_e248_d_b0 * p.p1);
        let eq13_e251_d_b1: f64 = (eq13_e248_d_b1 * p.p1);
        let eq13_e251_q: f64 = (eq13_e249_q * p.p1);
        let eq13_reactive_node_derivatives: [f64; 12] = [eq13_e251_d_n0, eq13_e251_d_n1, eq13_e251_d_n2, eq13_e251_d_n3, eq13_e251_d_n4, eq13_e251_d_n5, eq13_e251_d_n6, eq13_e251_d_n7, eq13_e251_d_n8, eq13_e251_d_n9, eq13_e251_d_n10, eq13_e251_d_n11];
        let eq13_reactive_branch_derivatives: [f64; 2] = [eq13_e251_d_b0, eq13_e251_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes,
            &eq13_reactive_node_derivatives,
            branches,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );
        let eq14_e254: f64 = (p.p3 * var_qte_s);
        let eq14_e254_d_n0: f64 = (p.p3 * var_qte_s_dn0);
        let eq14_e254_d_n1: f64 = (p.p3 * var_qte_s_dn1);
        let eq14_e254_d_n2: f64 = (p.p3 * var_qte_s_dn2);
        let eq14_e254_d_n3: f64 = (p.p3 * var_qte_s_dn3);
        let eq14_e254_d_n4: f64 = (p.p3 * var_qte_s_dn4);
        let eq14_e254_d_n5: f64 = (p.p3 * var_qte_s_dn5);
        let eq14_e254_d_n6: f64 = (p.p3 * var_qte_s_dn6);
        let eq14_e254_d_n7: f64 = (p.p3 * var_qte_s_dn7);
        let eq14_e254_d_n8: f64 = (p.p3 * var_qte_s_dn8);
        let eq14_e254_d_n9: f64 = (p.p3 * var_qte_s_dn9);
        let eq14_e254_d_n10: f64 = (p.p3 * var_qte_s_dn10);
        let eq14_e254_d_n11: f64 = (p.p3 * var_qte_s_dn11);
        let eq14_e254_d_b0: f64 = (p.p3 * var_qte_s_db0);
        let eq14_e254_d_b1: f64 = (p.p3 * var_qte_s_db1);
        let eq14_e255_q: f64 = eq14_e254;
        let eq14_e257: f64 = (eq14_e254 * p.p1);
        let eq14_e257_d_n0: f64 = (eq14_e254_d_n0 * p.p1);
        let eq14_e257_d_n1: f64 = (eq14_e254_d_n1 * p.p1);
        let eq14_e257_d_n2: f64 = (eq14_e254_d_n2 * p.p1);
        let eq14_e257_d_n3: f64 = (eq14_e254_d_n3 * p.p1);
        let eq14_e257_d_n4: f64 = (eq14_e254_d_n4 * p.p1);
        let eq14_e257_d_n5: f64 = (eq14_e254_d_n5 * p.p1);
        let eq14_e257_d_n6: f64 = (eq14_e254_d_n6 * p.p1);
        let eq14_e257_d_n7: f64 = (eq14_e254_d_n7 * p.p1);
        let eq14_e257_d_n8: f64 = (eq14_e254_d_n8 * p.p1);
        let eq14_e257_d_n9: f64 = (eq14_e254_d_n9 * p.p1);
        let eq14_e257_d_n10: f64 = (eq14_e254_d_n10 * p.p1);
        let eq14_e257_d_n11: f64 = (eq14_e254_d_n11 * p.p1);
        let eq14_e257_d_b0: f64 = (eq14_e254_d_b0 * p.p1);
        let eq14_e257_d_b1: f64 = (eq14_e254_d_b1 * p.p1);
        let eq14_e257_q: f64 = (eq14_e255_q * p.p1);
        let eq14_reactive_node_derivatives: [f64; 12] = [eq14_e257_d_n0, eq14_e257_d_n1, eq14_e257_d_n2, eq14_e257_d_n3, eq14_e257_d_n4, eq14_e257_d_n5, eq14_e257_d_n6, eq14_e257_d_n7, eq14_e257_d_n8, eq14_e257_d_n9, eq14_e257_d_n10, eq14_e257_d_n11];
        let eq14_reactive_branch_derivatives: [f64; 2] = [eq14_e257_d_b0, eq14_e257_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes,
            &eq14_reactive_node_derivatives,
            branches,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
        let eq15_e261: f64 = (var_qtc + var_qbc);
        let eq15_e261_d_n0: f64 = (var_qtc_dn0 + var_qbc_dn0);
        let eq15_e261_d_n1: f64 = (var_qtc_dn1 + var_qbc_dn1);
        let eq15_e261_d_n2: f64 = (var_qtc_dn2 + var_qbc_dn2);
        let eq15_e261_d_n3: f64 = (var_qtc_dn3 + var_qbc_dn3);
        let eq15_e261_d_n4: f64 = (var_qtc_dn4 + var_qbc_dn4);
        let eq15_e261_d_n5: f64 = (var_qtc_dn5 + var_qbc_dn5);
        let eq15_e261_d_n6: f64 = (var_qtc_dn6 + var_qbc_dn6);
        let eq15_e261_d_n7: f64 = (var_qtc_dn7 + var_qbc_dn7);
        let eq15_e261_d_n8: f64 = (var_qtc_dn8 + var_qbc_dn8);
        let eq15_e261_d_n9: f64 = (var_qtc_dn9 + var_qbc_dn9);
        let eq15_e261_d_n10: f64 = (var_qtc_dn10 + var_qbc_dn10);
        let eq15_e261_d_n11: f64 = (var_qtc_dn11 + var_qbc_dn11);
        let eq15_e261_d_b0: f64 = (var_qtc_db0 + var_qbc_db0);
        let eq15_e261_d_b1: f64 = (var_qtc_db1 + var_qbc_db1);
        let eq15_e263: f64 = (eq15_e261 + var_qepi);
        let eq15_e263_d_n0: f64 = (eq15_e261_d_n0 + var_qepi_dn0);
        let eq15_e263_d_n1: f64 = (eq15_e261_d_n1 + var_qepi_dn1);
        let eq15_e263_d_n2: f64 = (eq15_e261_d_n2 + var_qepi_dn2);
        let eq15_e263_d_n3: f64 = (eq15_e261_d_n3 + var_qepi_dn3);
        let eq15_e263_d_n4: f64 = (eq15_e261_d_n4 + var_qepi_dn4);
        let eq15_e263_d_n5: f64 = (eq15_e261_d_n5 + var_qepi_dn5);
        let eq15_e263_d_n6: f64 = (eq15_e261_d_n6 + var_qepi_dn6);
        let eq15_e263_d_n7: f64 = (eq15_e261_d_n7 + var_qepi_dn7);
        let eq15_e263_d_n8: f64 = (eq15_e261_d_n8 + var_qepi_dn8);
        let eq15_e263_d_n9: f64 = (eq15_e261_d_n9 + var_qepi_dn9);
        let eq15_e263_d_n10: f64 = (eq15_e261_d_n10 + var_qepi_dn10);
        let eq15_e263_d_n11: f64 = (eq15_e261_d_n11 + var_qepi_dn11);
        let eq15_e263_d_b0: f64 = (eq15_e261_d_b0 + var_qepi_db0);
        let eq15_e263_d_b1: f64 = (eq15_e261_d_b1 + var_qepi_db1);
        let eq15_e264: f64 = (p.p3 * eq15_e263);
        let eq15_e264_d_n0: f64 = (p.p3 * eq15_e263_d_n0);
        let eq15_e264_d_n1: f64 = (p.p3 * eq15_e263_d_n1);
        let eq15_e264_d_n2: f64 = (p.p3 * eq15_e263_d_n2);
        let eq15_e264_d_n3: f64 = (p.p3 * eq15_e263_d_n3);
        let eq15_e264_d_n4: f64 = (p.p3 * eq15_e263_d_n4);
        let eq15_e264_d_n5: f64 = (p.p3 * eq15_e263_d_n5);
        let eq15_e264_d_n6: f64 = (p.p3 * eq15_e263_d_n6);
        let eq15_e264_d_n7: f64 = (p.p3 * eq15_e263_d_n7);
        let eq15_e264_d_n8: f64 = (p.p3 * eq15_e263_d_n8);
        let eq15_e264_d_n9: f64 = (p.p3 * eq15_e263_d_n9);
        let eq15_e264_d_n10: f64 = (p.p3 * eq15_e263_d_n10);
        let eq15_e264_d_n11: f64 = (p.p3 * eq15_e263_d_n11);
        let eq15_e264_d_b0: f64 = (p.p3 * eq15_e263_d_b0);
        let eq15_e264_d_b1: f64 = (p.p3 * eq15_e263_d_b1);
        let eq15_e265_q: f64 = eq15_e264;
        let eq15_e267: f64 = (eq15_e264 * p.p1);
        let eq15_e267_d_n0: f64 = (eq15_e264_d_n0 * p.p1);
        let eq15_e267_d_n1: f64 = (eq15_e264_d_n1 * p.p1);
        let eq15_e267_d_n2: f64 = (eq15_e264_d_n2 * p.p1);
        let eq15_e267_d_n3: f64 = (eq15_e264_d_n3 * p.p1);
        let eq15_e267_d_n4: f64 = (eq15_e264_d_n4 * p.p1);
        let eq15_e267_d_n5: f64 = (eq15_e264_d_n5 * p.p1);
        let eq15_e267_d_n6: f64 = (eq15_e264_d_n6 * p.p1);
        let eq15_e267_d_n7: f64 = (eq15_e264_d_n7 * p.p1);
        let eq15_e267_d_n8: f64 = (eq15_e264_d_n8 * p.p1);
        let eq15_e267_d_n9: f64 = (eq15_e264_d_n9 * p.p1);
        let eq15_e267_d_n10: f64 = (eq15_e264_d_n10 * p.p1);
        let eq15_e267_d_n11: f64 = (eq15_e264_d_n11 * p.p1);
        let eq15_e267_d_b0: f64 = (eq15_e264_d_b0 * p.p1);
        let eq15_e267_d_b1: f64 = (eq15_e264_d_b1 * p.p1);
        let eq15_e267_q: f64 = (eq15_e265_q * p.p1);
        let eq15_reactive_node_derivatives: [f64; 12] = [eq15_e267_d_n0, eq15_e267_d_n1, eq15_e267_d_n2, eq15_e267_d_n3, eq15_e267_d_n4, eq15_e267_d_n5, eq15_e267_d_n6, eq15_e267_d_n7, eq15_e267_d_n8, eq15_e267_d_n9, eq15_e267_d_n10, eq15_e267_d_n11];
        let eq15_reactive_branch_derivatives: [f64; 2] = [eq15_e267_d_b0, eq15_e267_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq16_e270: f64 = (p.p3 * var_qb1b2);
        let eq16_e270_d_n0: f64 = (p.p3 * var_qb1b2_dn0);
        let eq16_e270_d_n1: f64 = (p.p3 * var_qb1b2_dn1);
        let eq16_e270_d_n2: f64 = (p.p3 * var_qb1b2_dn2);
        let eq16_e270_d_n3: f64 = (p.p3 * var_qb1b2_dn3);
        let eq16_e270_d_n4: f64 = (p.p3 * var_qb1b2_dn4);
        let eq16_e270_d_n5: f64 = (p.p3 * var_qb1b2_dn5);
        let eq16_e270_d_n6: f64 = (p.p3 * var_qb1b2_dn6);
        let eq16_e270_d_n7: f64 = (p.p3 * var_qb1b2_dn7);
        let eq16_e270_d_n8: f64 = (p.p3 * var_qb1b2_dn8);
        let eq16_e270_d_n9: f64 = (p.p3 * var_qb1b2_dn9);
        let eq16_e270_d_n10: f64 = (p.p3 * var_qb1b2_dn10);
        let eq16_e270_d_n11: f64 = (p.p3 * var_qb1b2_dn11);
        let eq16_e270_d_b0: f64 = (p.p3 * var_qb1b2_db0);
        let eq16_e270_d_b1: f64 = (p.p3 * var_qb1b2_db1);
        let eq16_e271_q: f64 = eq16_e270;
        let eq16_e273: f64 = (eq16_e270 * p.p1);
        let eq16_e273_d_n0: f64 = (eq16_e270_d_n0 * p.p1);
        let eq16_e273_d_n1: f64 = (eq16_e270_d_n1 * p.p1);
        let eq16_e273_d_n2: f64 = (eq16_e270_d_n2 * p.p1);
        let eq16_e273_d_n3: f64 = (eq16_e270_d_n3 * p.p1);
        let eq16_e273_d_n4: f64 = (eq16_e270_d_n4 * p.p1);
        let eq16_e273_d_n5: f64 = (eq16_e270_d_n5 * p.p1);
        let eq16_e273_d_n6: f64 = (eq16_e270_d_n6 * p.p1);
        let eq16_e273_d_n7: f64 = (eq16_e270_d_n7 * p.p1);
        let eq16_e273_d_n8: f64 = (eq16_e270_d_n8 * p.p1);
        let eq16_e273_d_n9: f64 = (eq16_e270_d_n9 * p.p1);
        let eq16_e273_d_n10: f64 = (eq16_e270_d_n10 * p.p1);
        let eq16_e273_d_n11: f64 = (eq16_e270_d_n11 * p.p1);
        let eq16_e273_d_b0: f64 = (eq16_e270_d_b0 * p.p1);
        let eq16_e273_d_b1: f64 = (eq16_e270_d_b1 * p.p1);
        let eq16_e273_q: f64 = (eq16_e271_q * p.p1);
        let eq16_reactive_node_derivatives: [f64; 12] = [eq16_e273_d_n0, eq16_e273_d_n1, eq16_e273_d_n2, eq16_e273_d_n3, eq16_e273_d_n4, eq16_e273_d_n5, eq16_e273_d_n6, eq16_e273_d_n7, eq16_e273_d_n8, eq16_e273_d_n9, eq16_e273_d_n10, eq16_e273_d_n11];
        let eq16_reactive_branch_derivatives: [f64; 2] = [eq16_e273_d_b0, eq16_e273_d_b1];
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
        let eq17_e278: f64 = (eq17_e276 * var_vbe);
        let eq17_e278_d_n0: f64 = (eq17_e276 * var_vbe_dn0);
        let eq17_e278_d_n1: f64 = (eq17_e276 * var_vbe_dn1);
        let eq17_e278_d_n2: f64 = (eq17_e276 * var_vbe_dn2);
        let eq17_e278_d_n3: f64 = (eq17_e276 * var_vbe_dn3);
        let eq17_e278_d_n4: f64 = (eq17_e276 * var_vbe_dn4);
        let eq17_e278_d_n5: f64 = (eq17_e276 * var_vbe_dn5);
        let eq17_e278_d_n6: f64 = (eq17_e276 * var_vbe_dn6);
        let eq17_e278_d_n7: f64 = (eq17_e276 * var_vbe_dn7);
        let eq17_e278_d_n8: f64 = (eq17_e276 * var_vbe_dn8);
        let eq17_e278_d_n9: f64 = (eq17_e276 * var_vbe_dn9);
        let eq17_e278_d_n10: f64 = (eq17_e276 * var_vbe_dn10);
        let eq17_e278_d_n11: f64 = (eq17_e276 * var_vbe_dn11);
        let eq17_e278_d_b0: f64 = (eq17_e276 * var_vbe_db0);
        let eq17_e278_d_b1: f64 = (eq17_e276 * var_vbe_db1);
        let eq17_e279_q: f64 = eq17_e278;
        let eq17_e281: f64 = (eq17_e278 * p.p1);
        let eq17_e281_d_n0: f64 = (eq17_e278_d_n0 * p.p1);
        let eq17_e281_d_n1: f64 = (eq17_e278_d_n1 * p.p1);
        let eq17_e281_d_n2: f64 = (eq17_e278_d_n2 * p.p1);
        let eq17_e281_d_n3: f64 = (eq17_e278_d_n3 * p.p1);
        let eq17_e281_d_n4: f64 = (eq17_e278_d_n4 * p.p1);
        let eq17_e281_d_n5: f64 = (eq17_e278_d_n5 * p.p1);
        let eq17_e281_d_n6: f64 = (eq17_e278_d_n6 * p.p1);
        let eq17_e281_d_n7: f64 = (eq17_e278_d_n7 * p.p1);
        let eq17_e281_d_n8: f64 = (eq17_e278_d_n8 * p.p1);
        let eq17_e281_d_n9: f64 = (eq17_e278_d_n9 * p.p1);
        let eq17_e281_d_n10: f64 = (eq17_e278_d_n10 * p.p1);
        let eq17_e281_d_n11: f64 = (eq17_e278_d_n11 * p.p1);
        let eq17_e281_d_b0: f64 = (eq17_e278_d_b0 * p.p1);
        let eq17_e281_d_b1: f64 = (eq17_e278_d_b1 * p.p1);
        let eq17_e281_q: f64 = (eq17_e279_q * p.p1);
        let eq17_reactive_node_derivatives: [f64; 12] = [eq17_e281_d_n0, eq17_e281_d_n1, eq17_e281_d_n2, eq17_e281_d_n3, eq17_e281_d_n4, eq17_e281_d_n5, eq17_e281_d_n6, eq17_e281_d_n7, eq17_e281_d_n8, eq17_e281_d_n9, eq17_e281_d_n10, eq17_e281_d_n11];
        let eq17_reactive_branch_derivatives: [f64; 2] = [eq17_e281_d_b0, eq17_e281_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes,
            &eq17_reactive_node_derivatives,
            branches,
            &eq17_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e284: f64 = (p.p3 * p.p77);
        let eq18_e286: f64 = (eq18_e284 * var_vbc);
        let eq18_e286_d_n0: f64 = (eq18_e284 * var_vbc_dn0);
        let eq18_e286_d_n1: f64 = (eq18_e284 * var_vbc_dn1);
        let eq18_e286_d_n2: f64 = (eq18_e284 * var_vbc_dn2);
        let eq18_e286_d_n3: f64 = (eq18_e284 * var_vbc_dn3);
        let eq18_e286_d_n4: f64 = (eq18_e284 * var_vbc_dn4);
        let eq18_e286_d_n5: f64 = (eq18_e284 * var_vbc_dn5);
        let eq18_e286_d_n6: f64 = (eq18_e284 * var_vbc_dn6);
        let eq18_e286_d_n7: f64 = (eq18_e284 * var_vbc_dn7);
        let eq18_e286_d_n8: f64 = (eq18_e284 * var_vbc_dn8);
        let eq18_e286_d_n9: f64 = (eq18_e284 * var_vbc_dn9);
        let eq18_e286_d_n10: f64 = (eq18_e284 * var_vbc_dn10);
        let eq18_e286_d_n11: f64 = (eq18_e284 * var_vbc_dn11);
        let eq18_e286_d_b0: f64 = (eq18_e284 * var_vbc_db0);
        let eq18_e286_d_b1: f64 = (eq18_e284 * var_vbc_db1);
        let eq18_e287_q: f64 = eq18_e286;
        let eq18_e289: f64 = (eq18_e286 * p.p1);
        let eq18_e289_d_n0: f64 = (eq18_e286_d_n0 * p.p1);
        let eq18_e289_d_n1: f64 = (eq18_e286_d_n1 * p.p1);
        let eq18_e289_d_n2: f64 = (eq18_e286_d_n2 * p.p1);
        let eq18_e289_d_n3: f64 = (eq18_e286_d_n3 * p.p1);
        let eq18_e289_d_n4: f64 = (eq18_e286_d_n4 * p.p1);
        let eq18_e289_d_n5: f64 = (eq18_e286_d_n5 * p.p1);
        let eq18_e289_d_n6: f64 = (eq18_e286_d_n6 * p.p1);
        let eq18_e289_d_n7: f64 = (eq18_e286_d_n7 * p.p1);
        let eq18_e289_d_n8: f64 = (eq18_e286_d_n8 * p.p1);
        let eq18_e289_d_n9: f64 = (eq18_e286_d_n9 * p.p1);
        let eq18_e289_d_n10: f64 = (eq18_e286_d_n10 * p.p1);
        let eq18_e289_d_n11: f64 = (eq18_e286_d_n11 * p.p1);
        let eq18_e289_d_b0: f64 = (eq18_e286_d_b0 * p.p1);
        let eq18_e289_d_b1: f64 = (eq18_e286_d_b1 * p.p1);
        let eq18_e289_q: f64 = (eq18_e287_q * p.p1);
        let eq18_reactive_node_derivatives: [f64; 12] = [eq18_e289_d_n0, eq18_e289_d_n1, eq18_e289_d_n2, eq18_e289_d_n3, eq18_e289_d_n4, eq18_e289_d_n5, eq18_e289_d_n6, eq18_e289_d_n7, eq18_e289_d_n8, eq18_e289_d_n9, eq18_e289_d_n10, eq18_e289_d_n11];
        let eq18_reactive_branch_derivatives: [f64; 2] = [eq18_e289_d_b0, eq18_e289_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq21_e305: f64 = (var_xqtex + var_xqex);
        let eq21_e305_d_n0: f64 = (var_xqtex_dn0 + var_xqex_dn0);
        let eq21_e305_d_n1: f64 = (var_xqtex_dn1 + var_xqex_dn1);
        let eq21_e305_d_n2: f64 = (var_xqtex_dn2 + var_xqex_dn2);
        let eq21_e305_d_n3: f64 = (var_xqtex_dn3 + var_xqex_dn3);
        let eq21_e305_d_n4: f64 = (var_xqtex_dn4 + var_xqex_dn4);
        let eq21_e305_d_n5: f64 = (var_xqtex_dn5 + var_xqex_dn5);
        let eq21_e305_d_n6: f64 = (var_xqtex_dn6 + var_xqex_dn6);
        let eq21_e305_d_n7: f64 = (var_xqtex_dn7 + var_xqex_dn7);
        let eq21_e305_d_n8: f64 = (var_xqtex_dn8 + var_xqex_dn8);
        let eq21_e305_d_n9: f64 = (var_xqtex_dn9 + var_xqex_dn9);
        let eq21_e305_d_n10: f64 = (var_xqtex_dn10 + var_xqex_dn10);
        let eq21_e305_d_n11: f64 = (var_xqtex_dn11 + var_xqex_dn11);
        let eq21_e305_d_b0: f64 = (var_xqtex_db0 + var_xqex_db0);
        let eq21_e305_d_b1: f64 = (var_xqtex_db1 + var_xqex_db1);
        let eq21_e306: f64 = (p.p3 * eq21_e305);
        let eq21_e306_d_n0: f64 = (p.p3 * eq21_e305_d_n0);
        let eq21_e306_d_n1: f64 = (p.p3 * eq21_e305_d_n1);
        let eq21_e306_d_n2: f64 = (p.p3 * eq21_e305_d_n2);
        let eq21_e306_d_n3: f64 = (p.p3 * eq21_e305_d_n3);
        let eq21_e306_d_n4: f64 = (p.p3 * eq21_e305_d_n4);
        let eq21_e306_d_n5: f64 = (p.p3 * eq21_e305_d_n5);
        let eq21_e306_d_n6: f64 = (p.p3 * eq21_e305_d_n6);
        let eq21_e306_d_n7: f64 = (p.p3 * eq21_e305_d_n7);
        let eq21_e306_d_n8: f64 = (p.p3 * eq21_e305_d_n8);
        let eq21_e306_d_n9: f64 = (p.p3 * eq21_e305_d_n9);
        let eq21_e306_d_n10: f64 = (p.p3 * eq21_e305_d_n10);
        let eq21_e306_d_n11: f64 = (p.p3 * eq21_e305_d_n11);
        let eq21_e306_d_b0: f64 = (p.p3 * eq21_e305_d_b0);
        let eq21_e306_d_b1: f64 = (p.p3 * eq21_e305_d_b1);
        let eq21_e307_q: f64 = eq21_e306;
        let eq21_e309: f64 = (eq21_e306 * p.p1);
        let eq21_e309_d_n0: f64 = (eq21_e306_d_n0 * p.p1);
        let eq21_e309_d_n1: f64 = (eq21_e306_d_n1 * p.p1);
        let eq21_e309_d_n2: f64 = (eq21_e306_d_n2 * p.p1);
        let eq21_e309_d_n3: f64 = (eq21_e306_d_n3 * p.p1);
        let eq21_e309_d_n4: f64 = (eq21_e306_d_n4 * p.p1);
        let eq21_e309_d_n5: f64 = (eq21_e306_d_n5 * p.p1);
        let eq21_e309_d_n6: f64 = (eq21_e306_d_n6 * p.p1);
        let eq21_e309_d_n7: f64 = (eq21_e306_d_n7 * p.p1);
        let eq21_e309_d_n8: f64 = (eq21_e306_d_n8 * p.p1);
        let eq21_e309_d_n9: f64 = (eq21_e306_d_n9 * p.p1);
        let eq21_e309_d_n10: f64 = (eq21_e306_d_n10 * p.p1);
        let eq21_e309_d_n11: f64 = (eq21_e306_d_n11 * p.p1);
        let eq21_e309_d_b0: f64 = (eq21_e306_d_b0 * p.p1);
        let eq21_e309_d_b1: f64 = (eq21_e306_d_b1 * p.p1);
        let eq21_e309_q: f64 = (eq21_e307_q * p.p1);
        let eq21_reactive_node_derivatives: [f64; 12] = [eq21_e309_d_n0, eq21_e309_d_n1, eq21_e309_d_n2, eq21_e309_d_n3, eq21_e309_d_n4, eq21_e309_d_n5, eq21_e309_d_n6, eq21_e309_d_n7, eq21_e309_d_n8, eq21_e309_d_n9, eq21_e309_d_n10, eq21_e309_d_n11];
        let eq21_reactive_branch_derivatives: [f64; 2] = [eq21_e309_d_b0, eq21_e309_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[9]),
            nodes,
            &eq21_reactive_node_derivatives,
            branches,
            &eq21_reactive_branch_derivatives,
            multiplicity,
        );
        let eq23_e324: f64 = (var_qtex + var_qex);
        let eq23_e324_d_n0: f64 = (var_qtex_dn0 + var_qex_dn0);
        let eq23_e324_d_n1: f64 = (var_qtex_dn1 + var_qex_dn1);
        let eq23_e324_d_n2: f64 = (var_qtex_dn2 + var_qex_dn2);
        let eq23_e324_d_n3: f64 = (var_qtex_dn3 + var_qex_dn3);
        let eq23_e324_d_n4: f64 = (var_qtex_dn4 + var_qex_dn4);
        let eq23_e324_d_n5: f64 = (var_qtex_dn5 + var_qex_dn5);
        let eq23_e324_d_n6: f64 = (var_qtex_dn6 + var_qex_dn6);
        let eq23_e324_d_n7: f64 = (var_qtex_dn7 + var_qex_dn7);
        let eq23_e324_d_n8: f64 = (var_qtex_dn8 + var_qex_dn8);
        let eq23_e324_d_n9: f64 = (var_qtex_dn9 + var_qex_dn9);
        let eq23_e324_d_n10: f64 = (var_qtex_dn10 + var_qex_dn10);
        let eq23_e324_d_n11: f64 = (var_qtex_dn11 + var_qex_dn11);
        let eq23_e324_d_b0: f64 = (var_qtex_db0 + var_qex_db0);
        let eq23_e324_d_b1: f64 = (var_qtex_db1 + var_qex_db1);
        let eq23_e325: f64 = (p.p3 * eq23_e324);
        let eq23_e325_d_n0: f64 = (p.p3 * eq23_e324_d_n0);
        let eq23_e325_d_n1: f64 = (p.p3 * eq23_e324_d_n1);
        let eq23_e325_d_n2: f64 = (p.p3 * eq23_e324_d_n2);
        let eq23_e325_d_n3: f64 = (p.p3 * eq23_e324_d_n3);
        let eq23_e325_d_n4: f64 = (p.p3 * eq23_e324_d_n4);
        let eq23_e325_d_n5: f64 = (p.p3 * eq23_e324_d_n5);
        let eq23_e325_d_n6: f64 = (p.p3 * eq23_e324_d_n6);
        let eq23_e325_d_n7: f64 = (p.p3 * eq23_e324_d_n7);
        let eq23_e325_d_n8: f64 = (p.p3 * eq23_e324_d_n8);
        let eq23_e325_d_n9: f64 = (p.p3 * eq23_e324_d_n9);
        let eq23_e325_d_n10: f64 = (p.p3 * eq23_e324_d_n10);
        let eq23_e325_d_n11: f64 = (p.p3 * eq23_e324_d_n11);
        let eq23_e325_d_b0: f64 = (p.p3 * eq23_e324_d_b0);
        let eq23_e325_d_b1: f64 = (p.p3 * eq23_e324_d_b1);
        let eq23_e326_q: f64 = eq23_e325;
        let eq23_e328: f64 = (eq23_e325 * p.p1);
        let eq23_e328_d_n0: f64 = (eq23_e325_d_n0 * p.p1);
        let eq23_e328_d_n1: f64 = (eq23_e325_d_n1 * p.p1);
        let eq23_e328_d_n2: f64 = (eq23_e325_d_n2 * p.p1);
        let eq23_e328_d_n3: f64 = (eq23_e325_d_n3 * p.p1);
        let eq23_e328_d_n4: f64 = (eq23_e325_d_n4 * p.p1);
        let eq23_e328_d_n5: f64 = (eq23_e325_d_n5 * p.p1);
        let eq23_e328_d_n6: f64 = (eq23_e325_d_n6 * p.p1);
        let eq23_e328_d_n7: f64 = (eq23_e325_d_n7 * p.p1);
        let eq23_e328_d_n8: f64 = (eq23_e325_d_n8 * p.p1);
        let eq23_e328_d_n9: f64 = (eq23_e325_d_n9 * p.p1);
        let eq23_e328_d_n10: f64 = (eq23_e325_d_n10 * p.p1);
        let eq23_e328_d_n11: f64 = (eq23_e325_d_n11 * p.p1);
        let eq23_e328_d_b0: f64 = (eq23_e325_d_b0 * p.p1);
        let eq23_e328_d_b1: f64 = (eq23_e325_d_b1 * p.p1);
        let eq23_e328_q: f64 = (eq23_e326_q * p.p1);
        let eq23_reactive_node_derivatives: [f64; 12] = [eq23_e328_d_n0, eq23_e328_d_n1, eq23_e328_d_n2, eq23_e328_d_n3, eq23_e328_d_n4, eq23_e328_d_n5, eq23_e328_d_n6, eq23_e328_d_n7, eq23_e328_d_n8, eq23_e328_d_n9, eq23_e328_d_n10, eq23_e328_d_n11];
        let eq23_reactive_branch_derivatives: [f64; 2] = [eq23_e328_d_b0, eq23_e328_d_b1];
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
        let eq30_e368: f64 = (var_taun * (nv11 - 0.0));
        let eq30_e368_d_n0: f64 = (var_taun_dn0 * (nv11 - 0.0));
        let eq30_e368_d_n1: f64 = (var_taun_dn1 * (nv11 - 0.0));
        let eq30_e368_d_n2: f64 = (var_taun_dn2 * (nv11 - 0.0));
        let eq30_e368_d_n3: f64 = (var_taun_dn3 * (nv11 - 0.0));
        let eq30_e368_d_n4: f64 = (var_taun_dn4 * (nv11 - 0.0));
        let eq30_e368_d_n5: f64 = (var_taun_dn5 * (nv11 - 0.0));
        let eq30_e368_d_n6: f64 = (var_taun_dn6 * (nv11 - 0.0));
        let eq30_e368_d_n7: f64 = (var_taun_dn7 * (nv11 - 0.0));
        let eq30_e368_d_n8: f64 = (var_taun_dn8 * (nv11 - 0.0));
        let eq30_e368_d_n9: f64 = (var_taun_dn9 * (nv11 - 0.0));
        let eq30_e368_d_n10: f64 = (var_taun_dn10 * (nv11 - 0.0));
        let eq30_e368_d_n11: f64 = ((var_taun_dn11 * (nv11 - 0.0)) + var_taun);
        let eq30_e368_d_b0: f64 = (var_taun_db0 * (nv11 - 0.0));
        let eq30_e368_d_b1: f64 = (var_taun_db1 * (nv11 - 0.0));
        let eq30_e368_q: f64 = (var_taun * eq30_e367_q);
        let eq30_e368_q_d_n0: f64 = (var_taun_dn0 * eq30_e367_q);
        let eq30_e368_q_d_n1: f64 = (var_taun_dn1 * eq30_e367_q);
        let eq30_e368_q_d_n2: f64 = (var_taun_dn2 * eq30_e367_q);
        let eq30_e368_q_d_n3: f64 = (var_taun_dn3 * eq30_e367_q);
        let eq30_e368_q_d_n4: f64 = (var_taun_dn4 * eq30_e367_q);
        let eq30_e368_q_d_n5: f64 = (var_taun_dn5 * eq30_e367_q);
        let eq30_e368_q_d_n6: f64 = (var_taun_dn6 * eq30_e367_q);
        let eq30_e368_q_d_n7: f64 = (var_taun_dn7 * eq30_e367_q);
        let eq30_e368_q_d_n8: f64 = (var_taun_dn8 * eq30_e367_q);
        let eq30_e368_q_d_n9: f64 = (var_taun_dn9 * eq30_e367_q);
        let eq30_e368_q_d_n10: f64 = (var_taun_dn10 * eq30_e367_q);
        let eq30_e368_q_d_n11: f64 = ((var_taun_dn11 * eq30_e367_q) + var_taun);
        let eq30_e368_q_d_b0: f64 = (var_taun_db0 * eq30_e367_q);
        let eq30_e368_q_d_b1: f64 = (var_taun_db1 * eq30_e367_q);
        let eq30_reactive_node_derivatives: [f64; 12] = [eq30_e368_q_d_n0, eq30_e368_q_d_n1, eq30_e368_q_d_n2, eq30_e368_q_d_n3, eq30_e368_q_d_n4, eq30_e368_q_d_n5, eq30_e368_q_d_n6, eq30_e368_q_d_n7, eq30_e368_q_d_n8, eq30_e368_q_d_n9, eq30_e368_q_d_n10, eq30_e368_q_d_n11];
        let eq30_reactive_branch_derivatives: [f64; 2] = [eq30_e368_q_d_b0, eq30_e368_q_d_b1];
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

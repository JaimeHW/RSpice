#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        multiplicity: f64,
        var_gmin: f64,
        var_guard125: f64,
        var_ib1: f64,
        var_ib1_db0: f64,
        var_ib1_db1: f64,
        var_ib1_dn0: f64,
        var_ib1_dn1: f64,
        var_ib1_dn10: f64,
        var_ib1_dn11: f64,
        var_ib1_dn12: f64,
        var_ib1_dn2: f64,
        var_ib1_dn3: f64,
        var_ib1_dn4: f64,
        var_ib1_dn5: f64,
        var_ib1_dn6: f64,
        var_ib1_dn7: f64,
        var_ib1_dn8: f64,
        var_ib1_dn9: f64,
        var_ib1_s: f64,
        var_ib1_s_db0: f64,
        var_ib1_s_db1: f64,
        var_ib1_s_dn0: f64,
        var_ib1_s_dn1: f64,
        var_ib1_s_dn10: f64,
        var_ib1_s_dn11: f64,
        var_ib1_s_dn12: f64,
        var_ib1_s_dn2: f64,
        var_ib1_s_dn3: f64,
        var_ib1_s_dn4: f64,
        var_ib1_s_dn5: f64,
        var_ib1_s_dn6: f64,
        var_ib1_s_dn7: f64,
        var_ib1_s_dn8: f64,
        var_ib1_s_dn9: f64,
        var_ib2: f64,
        var_ib2_db0: f64,
        var_ib2_db1: f64,
        var_ib2_dn0: f64,
        var_ib2_dn1: f64,
        var_ib2_dn10: f64,
        var_ib2_dn11: f64,
        var_ib2_dn12: f64,
        var_ib2_dn2: f64,
        var_ib2_dn3: f64,
        var_ib2_dn4: f64,
        var_ib2_dn5: f64,
        var_ib2_dn6: f64,
        var_ib2_dn7: f64,
        var_ib2_dn8: f64,
        var_ib2_dn9: f64,
        var_ib2_s: f64,
        var_ib2_s_db0: f64,
        var_ib2_s_db1: f64,
        var_ib2_s_dn0: f64,
        var_ib2_s_dn1: f64,
        var_ib2_s_dn10: f64,
        var_ib2_s_dn11: f64,
        var_ib2_s_dn12: f64,
        var_ib2_s_dn2: f64,
        var_ib2_s_dn3: f64,
        var_ib2_s_dn4: f64,
        var_ib2_s_dn5: f64,
        var_ib2_s_dn6: f64,
        var_ib2_s_dn7: f64,
        var_ib2_s_dn8: f64,
        var_ib2_s_dn9: f64,
        var_ibrel: f64,
        var_ibrel_db0: f64,
        var_ibrel_db1: f64,
        var_ibrel_dn0: f64,
        var_ibrel_dn1: f64,
        var_ibrel_dn10: f64,
        var_ibrel_dn11: f64,
        var_ibrel_dn12: f64,
        var_ibrel_dn2: f64,
        var_ibrel_dn3: f64,
        var_ibrel_dn4: f64,
        var_ibrel_dn5: f64,
        var_ibrel_dn6: f64,
        var_ibrel_dn7: f64,
        var_ibrel_dn8: f64,
        var_ibrel_dn9: f64,
        var_ibtbt: f64,
        var_ibtbt_db0: f64,
        var_ibtbt_db1: f64,
        var_ibtbt_dn0: f64,
        var_ibtbt_dn1: f64,
        var_ibtbt_dn10: f64,
        var_ibtbt_dn11: f64,
        var_ibtbt_dn12: f64,
        var_ibtbt_dn2: f64,
        var_ibtbt_dn3: f64,
        var_ibtbt_dn4: f64,
        var_ibtbt_dn5: f64,
        var_ibtbt_dn6: f64,
        var_ibtbt_dn7: f64,
        var_ibtbt_dn8: f64,
        var_ibtbt_dn9: f64,
        var_ic1c2: f64,
        var_ic1c2_db0: f64,
        var_ic1c2_db1: f64,
        var_ic1c2_dn0: f64,
        var_ic1c2_dn1: f64,
        var_ic1c2_dn10: f64,
        var_ic1c2_dn11: f64,
        var_ic1c2_dn12: f64,
        var_ic1c2_dn2: f64,
        var_ic1c2_dn3: f64,
        var_ic1c2_dn4: f64,
        var_ic1c2_dn5: f64,
        var_ic1c2_dn6: f64,
        var_ic1c2_dn7: f64,
        var_ic1c2_dn8: f64,
        var_ic1c2_dn9: f64,
        var_in_: f64,
        var_in__db0: f64,
        var_in__db1: f64,
        var_in__dn0: f64,
        var_in__dn1: f64,
        var_in__dn10: f64,
        var_in__dn11: f64,
        var_in__dn12: f64,
        var_in__dn2: f64,
        var_in__dn3: f64,
        var_in__dn4: f64,
        var_in__dn5: f64,
        var_in__dn6: f64,
        var_in__dn7: f64,
        var_in__dn8: f64,
        var_in__dn9: f64,
        var_isub: f64,
        var_isub_db0: f64,
        var_isub_db1: f64,
        var_isub_dn0: f64,
        var_isub_dn1: f64,
        var_isub_dn10: f64,
        var_isub_dn11: f64,
        var_isub_dn12: f64,
        var_isub_dn2: f64,
        var_isub_dn3: f64,
        var_isub_dn4: f64,
        var_isub_dn5: f64,
        var_isub_dn6: f64,
        var_isub_dn7: f64,
        var_isub_dn8: f64,
        var_isub_dn9: f64,
        var_isub_int: f64,
        var_isub_int_db0: f64,
        var_isub_int_db1: f64,
        var_isub_int_dn0: f64,
        var_isub_int_dn1: f64,
        var_isub_int_dn10: f64,
        var_isub_int_dn11: f64,
        var_isub_int_dn12: f64,
        var_isub_int_dn2: f64,
        var_isub_int_dn3: f64,
        var_isub_int_dn4: f64,
        var_isub_int_dn5: f64,
        var_isub_int_dn6: f64,
        var_isub_int_dn7: f64,
        var_isub_int_dn8: f64,
        var_isub_int_dn9: f64,
        var_itat: f64,
        var_itat_db0: f64,
        var_itat_db1: f64,
        var_itat_dn0: f64,
        var_itat_dn1: f64,
        var_itat_dn10: f64,
        var_itat_dn11: f64,
        var_itat_dn12: f64,
        var_itat_dn2: f64,
        var_itat_dn3: f64,
        var_itat_dn4: f64,
        var_itat_dn5: f64,
        var_itat_dn6: f64,
        var_itat_dn7: f64,
        var_itat_dn8: f64,
        var_itat_dn9: f64,
        var_iztcb: f64,
        var_iztcb_db0: f64,
        var_iztcb_db1: f64,
        var_iztcb_dn0: f64,
        var_iztcb_dn1: f64,
        var_iztcb_dn10: f64,
        var_iztcb_dn11: f64,
        var_iztcb_dn12: f64,
        var_iztcb_dn2: f64,
        var_iztcb_dn3: f64,
        var_iztcb_dn4: f64,
        var_iztcb_dn5: f64,
        var_iztcb_dn6: f64,
        var_iztcb_dn7: f64,
        var_iztcb_dn8: f64,
        var_iztcb_dn9: f64,
        var_izteb: f64,
        var_izteb_db0: f64,
        var_izteb_db1: f64,
        var_izteb_dn0: f64,
        var_izteb_dn1: f64,
        var_izteb_dn10: f64,
        var_izteb_dn11: f64,
        var_izteb_dn12: f64,
        var_izteb_dn2: f64,
        var_izteb_dn3: f64,
        var_izteb_dn4: f64,
        var_izteb_dn5: f64,
        var_izteb_dn6: f64,
        var_izteb_dn7: f64,
        var_izteb_dn8: f64,
        var_izteb_dn9: f64,
        var_vb2e1: f64,
        var_vb2e1_db0: f64,
        var_vb2e1_db1: f64,
        var_vb2e1_dn0: f64,
        var_vb2e1_dn1: f64,
        var_vb2e1_dn10: f64,
        var_vb2e1_dn11: f64,
        var_vb2e1_dn12: f64,
        var_vb2e1_dn2: f64,
        var_vb2e1_dn3: f64,
        var_vb2e1_dn4: f64,
        var_vb2e1_dn5: f64,
        var_vb2e1_dn6: f64,
        var_vb2e1_dn7: f64,
        var_vb2e1_dn8: f64,
        var_vb2e1_dn9: f64,
    ) {
        let eq0_e167: f64 = (p.p3 * var_ic1c2);
        let eq0_e167_d_n0: f64 = (p.p3 * var_ic1c2_dn0);
        let eq0_e167_d_n1: f64 = (p.p3 * var_ic1c2_dn1);
        let eq0_e167_d_n2: f64 = (p.p3 * var_ic1c2_dn2);
        let eq0_e167_d_n3: f64 = (p.p3 * var_ic1c2_dn3);
        let eq0_e167_d_n4: f64 = (p.p3 * var_ic1c2_dn4);
        let eq0_e167_d_n5: f64 = (p.p3 * var_ic1c2_dn5);
        let eq0_e167_d_n6: f64 = (p.p3 * var_ic1c2_dn6);
        let eq0_e167_d_n7: f64 = (p.p3 * var_ic1c2_dn7);
        let eq0_e167_d_n8: f64 = (p.p3 * var_ic1c2_dn8);
        let eq0_e167_d_n9: f64 = (p.p3 * var_ic1c2_dn9);
        let eq0_e167_d_n10: f64 = (p.p3 * var_ic1c2_dn10);
        let eq0_e167_d_n11: f64 = (p.p3 * var_ic1c2_dn11);
        let eq0_e167_d_n12: f64 = (p.p3 * var_ic1c2_dn12);
        let eq0_e167_d_b0: f64 = (p.p3 * var_ic1c2_db0);
        let eq0_e167_d_b1: f64 = (p.p3 * var_ic1c2_db1);
        let eq0_e169: f64 = (eq0_e167 * p.p1);
        let eq0_e169_d_n0: f64 = (eq0_e167_d_n0 * p.p1);
        let eq0_e169_d_n1: f64 = (eq0_e167_d_n1 * p.p1);
        let eq0_e169_d_n2: f64 = (eq0_e167_d_n2 * p.p1);
        let eq0_e169_d_n3: f64 = (eq0_e167_d_n3 * p.p1);
        let eq0_e169_d_n4: f64 = (eq0_e167_d_n4 * p.p1);
        let eq0_e169_d_n5: f64 = (eq0_e167_d_n5 * p.p1);
        let eq0_e169_d_n6: f64 = (eq0_e167_d_n6 * p.p1);
        let eq0_e169_d_n7: f64 = (eq0_e167_d_n7 * p.p1);
        let eq0_e169_d_n8: f64 = (eq0_e167_d_n8 * p.p1);
        let eq0_e169_d_n9: f64 = (eq0_e167_d_n9 * p.p1);
        let eq0_e169_d_n10: f64 = (eq0_e167_d_n10 * p.p1);
        let eq0_e169_d_n11: f64 = (eq0_e167_d_n11 * p.p1);
        let eq0_e169_d_n12: f64 = (eq0_e167_d_n12 * p.p1);
        let eq0_e169_d_b0: f64 = (eq0_e167_d_b0 * p.p1);
        let eq0_e169_d_b1: f64 = (eq0_e167_d_b1 * p.p1);
        let eq0_value: f64 = eq0_e169;
        let eq0_node_derivatives: [f64; 13] = [eq0_e169_d_n0, eq0_e169_d_n1, eq0_e169_d_n2, eq0_e169_d_n3, eq0_e169_d_n4, eq0_e169_d_n5, eq0_e169_d_n6, eq0_e169_d_n7, eq0_e169_d_n8, eq0_e169_d_n9, eq0_e169_d_n10, eq0_e169_d_n11, eq0_e169_d_n12];
        let eq0_branch_derivatives: [f64; 2] = [eq0_e169_d_b0, eq0_e169_d_b1];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let eq1_e172: f64 = (p.p3 * var_in_);
        let eq1_e172_d_n0: f64 = (p.p3 * var_in__dn0);
        let eq1_e172_d_n1: f64 = (p.p3 * var_in__dn1);
        let eq1_e172_d_n2: f64 = (p.p3 * var_in__dn2);
        let eq1_e172_d_n3: f64 = (p.p3 * var_in__dn3);
        let eq1_e172_d_n4: f64 = (p.p3 * var_in__dn4);
        let eq1_e172_d_n5: f64 = (p.p3 * var_in__dn5);
        let eq1_e172_d_n6: f64 = (p.p3 * var_in__dn6);
        let eq1_e172_d_n7: f64 = (p.p3 * var_in__dn7);
        let eq1_e172_d_n8: f64 = (p.p3 * var_in__dn8);
        let eq1_e172_d_n9: f64 = (p.p3 * var_in__dn9);
        let eq1_e172_d_n10: f64 = (p.p3 * var_in__dn10);
        let eq1_e172_d_n11: f64 = (p.p3 * var_in__dn11);
        let eq1_e172_d_n12: f64 = (p.p3 * var_in__dn12);
        let eq1_e172_d_b0: f64 = (p.p3 * var_in__db0);
        let eq1_e172_d_b1: f64 = (p.p3 * var_in__db1);
        let eq1_e174: f64 = (eq1_e172 * p.p1);
        let eq1_e174_d_n0: f64 = (eq1_e172_d_n0 * p.p1);
        let eq1_e174_d_n1: f64 = (eq1_e172_d_n1 * p.p1);
        let eq1_e174_d_n2: f64 = (eq1_e172_d_n2 * p.p1);
        let eq1_e174_d_n3: f64 = (eq1_e172_d_n3 * p.p1);
        let eq1_e174_d_n4: f64 = (eq1_e172_d_n4 * p.p1);
        let eq1_e174_d_n5: f64 = (eq1_e172_d_n5 * p.p1);
        let eq1_e174_d_n6: f64 = (eq1_e172_d_n6 * p.p1);
        let eq1_e174_d_n7: f64 = (eq1_e172_d_n7 * p.p1);
        let eq1_e174_d_n8: f64 = (eq1_e172_d_n8 * p.p1);
        let eq1_e174_d_n9: f64 = (eq1_e172_d_n9 * p.p1);
        let eq1_e174_d_n10: f64 = (eq1_e172_d_n10 * p.p1);
        let eq1_e174_d_n11: f64 = (eq1_e172_d_n11 * p.p1);
        let eq1_e174_d_n12: f64 = (eq1_e172_d_n12 * p.p1);
        let eq1_e174_d_b0: f64 = (eq1_e172_d_b0 * p.p1);
        let eq1_e174_d_b1: f64 = (eq1_e172_d_b1 * p.p1);
        let eq1_value: f64 = eq1_e174;
        let eq1_node_derivatives: [f64; 13] = [eq1_e174_d_n0, eq1_e174_d_n1, eq1_e174_d_n2, eq1_e174_d_n3, eq1_e174_d_n4, eq1_e174_d_n5, eq1_e174_d_n6, eq1_e174_d_n7, eq1_e174_d_n8, eq1_e174_d_n9, eq1_e174_d_n10, eq1_e174_d_n11, eq1_e174_d_n12];
        let eq1_branch_derivatives: [f64; 2] = [eq1_e174_d_b0, eq1_e174_d_b1];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let eq2_e178: f64 = (var_ib1_s + var_ib2_s);
        let eq2_e178_d_n0: f64 = (var_ib1_s_dn0 + var_ib2_s_dn0);
        let eq2_e178_d_n1: f64 = (var_ib1_s_dn1 + var_ib2_s_dn1);
        let eq2_e178_d_n2: f64 = (var_ib1_s_dn2 + var_ib2_s_dn2);
        let eq2_e178_d_n3: f64 = (var_ib1_s_dn3 + var_ib2_s_dn3);
        let eq2_e178_d_n4: f64 = (var_ib1_s_dn4 + var_ib2_s_dn4);
        let eq2_e178_d_n5: f64 = (var_ib1_s_dn5 + var_ib2_s_dn5);
        let eq2_e178_d_n6: f64 = (var_ib1_s_dn6 + var_ib2_s_dn6);
        let eq2_e178_d_n7: f64 = (var_ib1_s_dn7 + var_ib2_s_dn7);
        let eq2_e178_d_n8: f64 = (var_ib1_s_dn8 + var_ib2_s_dn8);
        let eq2_e178_d_n9: f64 = (var_ib1_s_dn9 + var_ib2_s_dn9);
        let eq2_e178_d_n10: f64 = (var_ib1_s_dn10 + var_ib2_s_dn10);
        let eq2_e178_d_n11: f64 = (var_ib1_s_dn11 + var_ib2_s_dn11);
        let eq2_e178_d_n12: f64 = (var_ib1_s_dn12 + var_ib2_s_dn12);
        let eq2_e178_d_b0: f64 = (var_ib1_s_db0 + var_ib2_s_db0);
        let eq2_e178_d_b1: f64 = (var_ib1_s_db1 + var_ib2_s_db1);
        let eq2_e180: f64 = (eq2_e178 + var_ibrel);
        let eq2_e180_d_n0: f64 = (eq2_e178_d_n0 + var_ibrel_dn0);
        let eq2_e180_d_n1: f64 = (eq2_e178_d_n1 + var_ibrel_dn1);
        let eq2_e180_d_n2: f64 = (eq2_e178_d_n2 + var_ibrel_dn2);
        let eq2_e180_d_n3: f64 = (eq2_e178_d_n3 + var_ibrel_dn3);
        let eq2_e180_d_n4: f64 = (eq2_e178_d_n4 + var_ibrel_dn4);
        let eq2_e180_d_n5: f64 = (eq2_e178_d_n5 + var_ibrel_dn5);
        let eq2_e180_d_n6: f64 = (eq2_e178_d_n6 + var_ibrel_dn6);
        let eq2_e180_d_n7: f64 = (eq2_e178_d_n7 + var_ibrel_dn7);
        let eq2_e180_d_n8: f64 = (eq2_e178_d_n8 + var_ibrel_dn8);
        let eq2_e180_d_n9: f64 = (eq2_e178_d_n9 + var_ibrel_dn9);
        let eq2_e180_d_n10: f64 = (eq2_e178_d_n10 + var_ibrel_dn10);
        let eq2_e180_d_n11: f64 = (eq2_e178_d_n11 + var_ibrel_dn11);
        let eq2_e180_d_n12: f64 = (eq2_e178_d_n12 + var_ibrel_dn12);
        let eq2_e180_d_b0: f64 = (eq2_e178_d_b0 + var_ibrel_db0);
        let eq2_e180_d_b1: f64 = (eq2_e178_d_b1 + var_ibrel_db1);
        let eq2_e181: f64 = (p.p3 * eq2_e180);
        let eq2_e181_d_n0: f64 = (p.p3 * eq2_e180_d_n0);
        let eq2_e181_d_n1: f64 = (p.p3 * eq2_e180_d_n1);
        let eq2_e181_d_n2: f64 = (p.p3 * eq2_e180_d_n2);
        let eq2_e181_d_n3: f64 = (p.p3 * eq2_e180_d_n3);
        let eq2_e181_d_n4: f64 = (p.p3 * eq2_e180_d_n4);
        let eq2_e181_d_n5: f64 = (p.p3 * eq2_e180_d_n5);
        let eq2_e181_d_n6: f64 = (p.p3 * eq2_e180_d_n6);
        let eq2_e181_d_n7: f64 = (p.p3 * eq2_e180_d_n7);
        let eq2_e181_d_n8: f64 = (p.p3 * eq2_e180_d_n8);
        let eq2_e181_d_n9: f64 = (p.p3 * eq2_e180_d_n9);
        let eq2_e181_d_n10: f64 = (p.p3 * eq2_e180_d_n10);
        let eq2_e181_d_n11: f64 = (p.p3 * eq2_e180_d_n11);
        let eq2_e181_d_n12: f64 = (p.p3 * eq2_e180_d_n12);
        let eq2_e181_d_b0: f64 = (p.p3 * eq2_e180_d_b0);
        let eq2_e181_d_b1: f64 = (p.p3 * eq2_e180_d_b1);
        let eq2_e183: f64 = (eq2_e181 * p.p1);
        let eq2_e183_d_n0: f64 = (eq2_e181_d_n0 * p.p1);
        let eq2_e183_d_n1: f64 = (eq2_e181_d_n1 * p.p1);
        let eq2_e183_d_n2: f64 = (eq2_e181_d_n2 * p.p1);
        let eq2_e183_d_n3: f64 = (eq2_e181_d_n3 * p.p1);
        let eq2_e183_d_n4: f64 = (eq2_e181_d_n4 * p.p1);
        let eq2_e183_d_n5: f64 = (eq2_e181_d_n5 * p.p1);
        let eq2_e183_d_n6: f64 = (eq2_e181_d_n6 * p.p1);
        let eq2_e183_d_n7: f64 = (eq2_e181_d_n7 * p.p1);
        let eq2_e183_d_n8: f64 = (eq2_e181_d_n8 * p.p1);
        let eq2_e183_d_n9: f64 = (eq2_e181_d_n9 * p.p1);
        let eq2_e183_d_n10: f64 = (eq2_e181_d_n10 * p.p1);
        let eq2_e183_d_n11: f64 = (eq2_e181_d_n11 * p.p1);
        let eq2_e183_d_n12: f64 = (eq2_e181_d_n12 * p.p1);
        let eq2_e183_d_b0: f64 = (eq2_e181_d_b0 * p.p1);
        let eq2_e183_d_b1: f64 = (eq2_e181_d_b1 * p.p1);
        let eq2_value: f64 = eq2_e183;
        let eq2_node_derivatives: [f64; 13] = [eq2_e183_d_n0, eq2_e183_d_n1, eq2_e183_d_n2, eq2_e183_d_n3, eq2_e183_d_n4, eq2_e183_d_n5, eq2_e183_d_n6, eq2_e183_d_n7, eq2_e183_d_n8, eq2_e183_d_n9, eq2_e183_d_n10, eq2_e183_d_n11, eq2_e183_d_n12];
        let eq2_branch_derivatives: [f64; 2] = [eq2_e183_d_b0, eq2_e183_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let eq3_e187: f64 = (var_ib1 + var_ib2);
        let eq3_e187_d_n0: f64 = (var_ib1_dn0 + var_ib2_dn0);
        let eq3_e187_d_n1: f64 = (var_ib1_dn1 + var_ib2_dn1);
        let eq3_e187_d_n2: f64 = (var_ib1_dn2 + var_ib2_dn2);
        let eq3_e187_d_n3: f64 = (var_ib1_dn3 + var_ib2_dn3);
        let eq3_e187_d_n4: f64 = (var_ib1_dn4 + var_ib2_dn4);
        let eq3_e187_d_n5: f64 = (var_ib1_dn5 + var_ib2_dn5);
        let eq3_e187_d_n6: f64 = (var_ib1_dn6 + var_ib2_dn6);
        let eq3_e187_d_n7: f64 = (var_ib1_dn7 + var_ib2_dn7);
        let eq3_e187_d_n8: f64 = (var_ib1_dn8 + var_ib2_dn8);
        let eq3_e187_d_n9: f64 = (var_ib1_dn9 + var_ib2_dn9);
        let eq3_e187_d_n10: f64 = (var_ib1_dn10 + var_ib2_dn10);
        let eq3_e187_d_n11: f64 = (var_ib1_dn11 + var_ib2_dn11);
        let eq3_e187_d_n12: f64 = (var_ib1_dn12 + var_ib2_dn12);
        let eq3_e187_d_b0: f64 = (var_ib1_db0 + var_ib2_db0);
        let eq3_e187_d_b1: f64 = (var_ib1_db1 + var_ib2_db1);
        let eq3_e190: f64 = (var_gmin * var_vb2e1);
        let eq3_e190_d_n0: f64 = (var_gmin * var_vb2e1_dn0);
        let eq3_e190_d_n1: f64 = (var_gmin * var_vb2e1_dn1);
        let eq3_e190_d_n2: f64 = (var_gmin * var_vb2e1_dn2);
        let eq3_e190_d_n3: f64 = (var_gmin * var_vb2e1_dn3);
        let eq3_e190_d_n4: f64 = (var_gmin * var_vb2e1_dn4);
        let eq3_e190_d_n5: f64 = (var_gmin * var_vb2e1_dn5);
        let eq3_e190_d_n6: f64 = (var_gmin * var_vb2e1_dn6);
        let eq3_e190_d_n7: f64 = (var_gmin * var_vb2e1_dn7);
        let eq3_e190_d_n8: f64 = (var_gmin * var_vb2e1_dn8);
        let eq3_e190_d_n9: f64 = (var_gmin * var_vb2e1_dn9);
        let eq3_e190_d_n10: f64 = (var_gmin * var_vb2e1_dn10);
        let eq3_e190_d_n11: f64 = (var_gmin * var_vb2e1_dn11);
        let eq3_e190_d_n12: f64 = (var_gmin * var_vb2e1_dn12);
        let eq3_e190_d_b0: f64 = (var_gmin * var_vb2e1_db0);
        let eq3_e190_d_b1: f64 = (var_gmin * var_vb2e1_db1);
        let eq3_e191: f64 = (eq3_e187 + eq3_e190);
        let eq3_e191_d_n0: f64 = (eq3_e187_d_n0 + eq3_e190_d_n0);
        let eq3_e191_d_n1: f64 = (eq3_e187_d_n1 + eq3_e190_d_n1);
        let eq3_e191_d_n2: f64 = (eq3_e187_d_n2 + eq3_e190_d_n2);
        let eq3_e191_d_n3: f64 = (eq3_e187_d_n3 + eq3_e190_d_n3);
        let eq3_e191_d_n4: f64 = (eq3_e187_d_n4 + eq3_e190_d_n4);
        let eq3_e191_d_n5: f64 = (eq3_e187_d_n5 + eq3_e190_d_n5);
        let eq3_e191_d_n6: f64 = (eq3_e187_d_n6 + eq3_e190_d_n6);
        let eq3_e191_d_n7: f64 = (eq3_e187_d_n7 + eq3_e190_d_n7);
        let eq3_e191_d_n8: f64 = (eq3_e187_d_n8 + eq3_e190_d_n8);
        let eq3_e191_d_n9: f64 = (eq3_e187_d_n9 + eq3_e190_d_n9);
        let eq3_e191_d_n10: f64 = (eq3_e187_d_n10 + eq3_e190_d_n10);
        let eq3_e191_d_n11: f64 = (eq3_e187_d_n11 + eq3_e190_d_n11);
        let eq3_e191_d_n12: f64 = (eq3_e187_d_n12 + eq3_e190_d_n12);
        let eq3_e191_d_b0: f64 = (eq3_e187_d_b0 + eq3_e190_d_b0);
        let eq3_e191_d_b1: f64 = (eq3_e187_d_b1 + eq3_e190_d_b1);
        let eq3_e193: f64 = (eq3_e191 - var_izteb);
        let eq3_e193_d_n0: f64 = (eq3_e191_d_n0 - var_izteb_dn0);
        let eq3_e193_d_n1: f64 = (eq3_e191_d_n1 - var_izteb_dn1);
        let eq3_e193_d_n2: f64 = (eq3_e191_d_n2 - var_izteb_dn2);
        let eq3_e193_d_n3: f64 = (eq3_e191_d_n3 - var_izteb_dn3);
        let eq3_e193_d_n4: f64 = (eq3_e191_d_n4 - var_izteb_dn4);
        let eq3_e193_d_n5: f64 = (eq3_e191_d_n5 - var_izteb_dn5);
        let eq3_e193_d_n6: f64 = (eq3_e191_d_n6 - var_izteb_dn6);
        let eq3_e193_d_n7: f64 = (eq3_e191_d_n7 - var_izteb_dn7);
        let eq3_e193_d_n8: f64 = (eq3_e191_d_n8 - var_izteb_dn8);
        let eq3_e193_d_n9: f64 = (eq3_e191_d_n9 - var_izteb_dn9);
        let eq3_e193_d_n10: f64 = (eq3_e191_d_n10 - var_izteb_dn10);
        let eq3_e193_d_n11: f64 = (eq3_e191_d_n11 - var_izteb_dn11);
        let eq3_e193_d_n12: f64 = (eq3_e191_d_n12 - var_izteb_dn12);
        let eq3_e193_d_b0: f64 = (eq3_e191_d_b0 - var_izteb_db0);
        let eq3_e193_d_b1: f64 = (eq3_e191_d_b1 - var_izteb_db1);
        let eq3_e195: f64 = (eq3_e193 + var_ibtbt);
        let eq3_e195_d_n0: f64 = (eq3_e193_d_n0 + var_ibtbt_dn0);
        let eq3_e195_d_n1: f64 = (eq3_e193_d_n1 + var_ibtbt_dn1);
        let eq3_e195_d_n2: f64 = (eq3_e193_d_n2 + var_ibtbt_dn2);
        let eq3_e195_d_n3: f64 = (eq3_e193_d_n3 + var_ibtbt_dn3);
        let eq3_e195_d_n4: f64 = (eq3_e193_d_n4 + var_ibtbt_dn4);
        let eq3_e195_d_n5: f64 = (eq3_e193_d_n5 + var_ibtbt_dn5);
        let eq3_e195_d_n6: f64 = (eq3_e193_d_n6 + var_ibtbt_dn6);
        let eq3_e195_d_n7: f64 = (eq3_e193_d_n7 + var_ibtbt_dn7);
        let eq3_e195_d_n8: f64 = (eq3_e193_d_n8 + var_ibtbt_dn8);
        let eq3_e195_d_n9: f64 = (eq3_e193_d_n9 + var_ibtbt_dn9);
        let eq3_e195_d_n10: f64 = (eq3_e193_d_n10 + var_ibtbt_dn10);
        let eq3_e195_d_n11: f64 = (eq3_e193_d_n11 + var_ibtbt_dn11);
        let eq3_e195_d_n12: f64 = (eq3_e193_d_n12 + var_ibtbt_dn12);
        let eq3_e195_d_b0: f64 = (eq3_e193_d_b0 + var_ibtbt_db0);
        let eq3_e195_d_b1: f64 = (eq3_e193_d_b1 + var_ibtbt_db1);
        let eq3_e197: f64 = (eq3_e195 + var_itat);
        let eq3_e197_d_n0: f64 = (eq3_e195_d_n0 + var_itat_dn0);
        let eq3_e197_d_n1: f64 = (eq3_e195_d_n1 + var_itat_dn1);
        let eq3_e197_d_n2: f64 = (eq3_e195_d_n2 + var_itat_dn2);
        let eq3_e197_d_n3: f64 = (eq3_e195_d_n3 + var_itat_dn3);
        let eq3_e197_d_n4: f64 = (eq3_e195_d_n4 + var_itat_dn4);
        let eq3_e197_d_n5: f64 = (eq3_e195_d_n5 + var_itat_dn5);
        let eq3_e197_d_n6: f64 = (eq3_e195_d_n6 + var_itat_dn6);
        let eq3_e197_d_n7: f64 = (eq3_e195_d_n7 + var_itat_dn7);
        let eq3_e197_d_n8: f64 = (eq3_e195_d_n8 + var_itat_dn8);
        let eq3_e197_d_n9: f64 = (eq3_e195_d_n9 + var_itat_dn9);
        let eq3_e197_d_n10: f64 = (eq3_e195_d_n10 + var_itat_dn10);
        let eq3_e197_d_n11: f64 = (eq3_e195_d_n11 + var_itat_dn11);
        let eq3_e197_d_n12: f64 = (eq3_e195_d_n12 + var_itat_dn12);
        let eq3_e197_d_b0: f64 = (eq3_e195_d_b0 + var_itat_db0);
        let eq3_e197_d_b1: f64 = (eq3_e195_d_b1 + var_itat_db1);
        let eq3_e198: f64 = (p.p3 * eq3_e197);
        let eq3_e198_d_n0: f64 = (p.p3 * eq3_e197_d_n0);
        let eq3_e198_d_n1: f64 = (p.p3 * eq3_e197_d_n1);
        let eq3_e198_d_n2: f64 = (p.p3 * eq3_e197_d_n2);
        let eq3_e198_d_n3: f64 = (p.p3 * eq3_e197_d_n3);
        let eq3_e198_d_n4: f64 = (p.p3 * eq3_e197_d_n4);
        let eq3_e198_d_n5: f64 = (p.p3 * eq3_e197_d_n5);
        let eq3_e198_d_n6: f64 = (p.p3 * eq3_e197_d_n6);
        let eq3_e198_d_n7: f64 = (p.p3 * eq3_e197_d_n7);
        let eq3_e198_d_n8: f64 = (p.p3 * eq3_e197_d_n8);
        let eq3_e198_d_n9: f64 = (p.p3 * eq3_e197_d_n9);
        let eq3_e198_d_n10: f64 = (p.p3 * eq3_e197_d_n10);
        let eq3_e198_d_n11: f64 = (p.p3 * eq3_e197_d_n11);
        let eq3_e198_d_n12: f64 = (p.p3 * eq3_e197_d_n12);
        let eq3_e198_d_b0: f64 = (p.p3 * eq3_e197_d_b0);
        let eq3_e198_d_b1: f64 = (p.p3 * eq3_e197_d_b1);
        let eq3_e200: f64 = (eq3_e198 * p.p1);
        let eq3_e200_d_n0: f64 = (eq3_e198_d_n0 * p.p1);
        let eq3_e200_d_n1: f64 = (eq3_e198_d_n1 * p.p1);
        let eq3_e200_d_n2: f64 = (eq3_e198_d_n2 * p.p1);
        let eq3_e200_d_n3: f64 = (eq3_e198_d_n3 * p.p1);
        let eq3_e200_d_n4: f64 = (eq3_e198_d_n4 * p.p1);
        let eq3_e200_d_n5: f64 = (eq3_e198_d_n5 * p.p1);
        let eq3_e200_d_n6: f64 = (eq3_e198_d_n6 * p.p1);
        let eq3_e200_d_n7: f64 = (eq3_e198_d_n7 * p.p1);
        let eq3_e200_d_n8: f64 = (eq3_e198_d_n8 * p.p1);
        let eq3_e200_d_n9: f64 = (eq3_e198_d_n9 * p.p1);
        let eq3_e200_d_n10: f64 = (eq3_e198_d_n10 * p.p1);
        let eq3_e200_d_n11: f64 = (eq3_e198_d_n11 * p.p1);
        let eq3_e200_d_n12: f64 = (eq3_e198_d_n12 * p.p1);
        let eq3_e200_d_b0: f64 = (eq3_e198_d_b0 * p.p1);
        let eq3_e200_d_b1: f64 = (eq3_e198_d_b1 * p.p1);
        let eq3_value: f64 = eq3_e200;
        let eq3_node_derivatives: [f64; 13] = [eq3_e200_d_n0, eq3_e200_d_n1, eq3_e200_d_n2, eq3_e200_d_n3, eq3_e200_d_n4, eq3_e200_d_n5, eq3_e200_d_n6, eq3_e200_d_n7, eq3_e200_d_n8, eq3_e200_d_n9, eq3_e200_d_n10, eq3_e200_d_n11, eq3_e200_d_n12];
        let eq3_branch_derivatives: [f64; 2] = [eq3_e200_d_b0, eq3_e200_d_b1];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e209, eq4_e209_d_n0, eq4_e209_d_n1, eq4_e209_d_n2, eq4_e209_d_n3, eq4_e209_d_n4, eq4_e209_d_n5, eq4_e209_d_n6, eq4_e209_d_n7, eq4_e209_d_n8, eq4_e209_d_n9, eq4_e209_d_n10, eq4_e209_d_n11, eq4_e209_d_n12, eq4_e209_d_b0, eq4_e209_d_b1,) = {
    if (var_guard125 != 0.0) {
        let eq4_e204: f64 = (-var_iztcb);
        let eq4_e205: f64 = (p.p3 * eq4_e204);
        let eq4_e205_d_n0: f64 = (p.p3 * (-var_iztcb_dn0));
        let eq4_e205_d_n1: f64 = (p.p3 * (-var_iztcb_dn1));
        let eq4_e205_d_n2: f64 = (p.p3 * (-var_iztcb_dn2));
        let eq4_e205_d_n3: f64 = (p.p3 * (-var_iztcb_dn3));
        let eq4_e205_d_n4: f64 = (p.p3 * (-var_iztcb_dn4));
        let eq4_e205_d_n5: f64 = (p.p3 * (-var_iztcb_dn5));
        let eq4_e205_d_n6: f64 = (p.p3 * (-var_iztcb_dn6));
        let eq4_e205_d_n7: f64 = (p.p3 * (-var_iztcb_dn7));
        let eq4_e205_d_n8: f64 = (p.p3 * (-var_iztcb_dn8));
        let eq4_e205_d_n9: f64 = (p.p3 * (-var_iztcb_dn9));
        let eq4_e205_d_n10: f64 = (p.p3 * (-var_iztcb_dn10));
        let eq4_e205_d_n11: f64 = (p.p3 * (-var_iztcb_dn11));
        let eq4_e205_d_n12: f64 = (p.p3 * (-var_iztcb_dn12));
        let eq4_e205_d_b0: f64 = (p.p3 * (-var_iztcb_db0));
        let eq4_e205_d_b1: f64 = (p.p3 * (-var_iztcb_db1));
        let eq4_e207: f64 = (eq4_e205 * p.p1);
        let eq4_e207_d_n0: f64 = (eq4_e205_d_n0 * p.p1);
        let eq4_e207_d_n1: f64 = (eq4_e205_d_n1 * p.p1);
        let eq4_e207_d_n2: f64 = (eq4_e205_d_n2 * p.p1);
        let eq4_e207_d_n3: f64 = (eq4_e205_d_n3 * p.p1);
        let eq4_e207_d_n4: f64 = (eq4_e205_d_n4 * p.p1);
        let eq4_e207_d_n5: f64 = (eq4_e205_d_n5 * p.p1);
        let eq4_e207_d_n6: f64 = (eq4_e205_d_n6 * p.p1);
        let eq4_e207_d_n7: f64 = (eq4_e205_d_n7 * p.p1);
        let eq4_e207_d_n8: f64 = (eq4_e205_d_n8 * p.p1);
        let eq4_e207_d_n9: f64 = (eq4_e205_d_n9 * p.p1);
        let eq4_e207_d_n10: f64 = (eq4_e205_d_n10 * p.p1);
        let eq4_e207_d_n11: f64 = (eq4_e205_d_n11 * p.p1);
        let eq4_e207_d_n12: f64 = (eq4_e205_d_n12 * p.p1);
        let eq4_e207_d_b0: f64 = (eq4_e205_d_b0 * p.p1);
        let eq4_e207_d_b1: f64 = (eq4_e205_d_b1 * p.p1);
        (eq4_e207, eq4_e207_d_n0, eq4_e207_d_n1, eq4_e207_d_n2, eq4_e207_d_n3, eq4_e207_d_n4, eq4_e207_d_n5, eq4_e207_d_n6, eq4_e207_d_n7, eq4_e207_d_n8, eq4_e207_d_n9, eq4_e207_d_n10, eq4_e207_d_n11, eq4_e207_d_n12, eq4_e207_d_b0, eq4_e207_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e209;
        let eq4_node_derivatives: [f64; 13] = [eq4_e209_d_n0, eq4_e209_d_n1, eq4_e209_d_n2, eq4_e209_d_n3, eq4_e209_d_n4, eq4_e209_d_n5, eq4_e209_d_n6, eq4_e209_d_n7, eq4_e209_d_n8, eq4_e209_d_n9, eq4_e209_d_n10, eq4_e209_d_n11, eq4_e209_d_n12];
        let eq4_branch_derivatives: [f64; 2] = [eq4_e209_d_b0, eq4_e209_d_b1];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e219, eq5_e219_d_n0, eq5_e219_d_n1, eq5_e219_d_n2, eq5_e219_d_n3, eq5_e219_d_n4, eq5_e219_d_n5, eq5_e219_d_n6, eq5_e219_d_n7, eq5_e219_d_n8, eq5_e219_d_n9, eq5_e219_d_n10, eq5_e219_d_n11, eq5_e219_d_n12, eq5_e219_d_b0, eq5_e219_d_b1,) = {
    if (var_guard125 == 0.0) {
        let eq5_e214: f64 = (-var_iztcb);
        let eq5_e215: f64 = (p.p3 * eq5_e214);
        let eq5_e215_d_n0: f64 = (p.p3 * (-var_iztcb_dn0));
        let eq5_e215_d_n1: f64 = (p.p3 * (-var_iztcb_dn1));
        let eq5_e215_d_n2: f64 = (p.p3 * (-var_iztcb_dn2));
        let eq5_e215_d_n3: f64 = (p.p3 * (-var_iztcb_dn3));
        let eq5_e215_d_n4: f64 = (p.p3 * (-var_iztcb_dn4));
        let eq5_e215_d_n5: f64 = (p.p3 * (-var_iztcb_dn5));
        let eq5_e215_d_n6: f64 = (p.p3 * (-var_iztcb_dn6));
        let eq5_e215_d_n7: f64 = (p.p3 * (-var_iztcb_dn7));
        let eq5_e215_d_n8: f64 = (p.p3 * (-var_iztcb_dn8));
        let eq5_e215_d_n9: f64 = (p.p3 * (-var_iztcb_dn9));
        let eq5_e215_d_n10: f64 = (p.p3 * (-var_iztcb_dn10));
        let eq5_e215_d_n11: f64 = (p.p3 * (-var_iztcb_dn11));
        let eq5_e215_d_n12: f64 = (p.p3 * (-var_iztcb_dn12));
        let eq5_e215_d_b0: f64 = (p.p3 * (-var_iztcb_db0));
        let eq5_e215_d_b1: f64 = (p.p3 * (-var_iztcb_db1));
        let eq5_e217: f64 = (eq5_e215 * p.p1);
        let eq5_e217_d_n0: f64 = (eq5_e215_d_n0 * p.p1);
        let eq5_e217_d_n1: f64 = (eq5_e215_d_n1 * p.p1);
        let eq5_e217_d_n2: f64 = (eq5_e215_d_n2 * p.p1);
        let eq5_e217_d_n3: f64 = (eq5_e215_d_n3 * p.p1);
        let eq5_e217_d_n4: f64 = (eq5_e215_d_n4 * p.p1);
        let eq5_e217_d_n5: f64 = (eq5_e215_d_n5 * p.p1);
        let eq5_e217_d_n6: f64 = (eq5_e215_d_n6 * p.p1);
        let eq5_e217_d_n7: f64 = (eq5_e215_d_n7 * p.p1);
        let eq5_e217_d_n8: f64 = (eq5_e215_d_n8 * p.p1);
        let eq5_e217_d_n9: f64 = (eq5_e215_d_n9 * p.p1);
        let eq5_e217_d_n10: f64 = (eq5_e215_d_n10 * p.p1);
        let eq5_e217_d_n11: f64 = (eq5_e215_d_n11 * p.p1);
        let eq5_e217_d_n12: f64 = (eq5_e215_d_n12 * p.p1);
        let eq5_e217_d_b0: f64 = (eq5_e215_d_b0 * p.p1);
        let eq5_e217_d_b1: f64 = (eq5_e215_d_b1 * p.p1);
        (eq5_e217, eq5_e217_d_n0, eq5_e217_d_n1, eq5_e217_d_n2, eq5_e217_d_n3, eq5_e217_d_n4, eq5_e217_d_n5, eq5_e217_d_n6, eq5_e217_d_n7, eq5_e217_d_n8, eq5_e217_d_n9, eq5_e217_d_n10, eq5_e217_d_n11, eq5_e217_d_n12, eq5_e217_d_b0, eq5_e217_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e219;
        let eq5_node_derivatives: [f64; 13] = [eq5_e219_d_n0, eq5_e219_d_n1, eq5_e219_d_n2, eq5_e219_d_n3, eq5_e219_d_n4, eq5_e219_d_n5, eq5_e219_d_n6, eq5_e219_d_n7, eq5_e219_d_n8, eq5_e219_d_n9, eq5_e219_d_n10, eq5_e219_d_n11, eq5_e219_d_n12];
        let eq5_branch_derivatives: [f64; 2] = [eq5_e219_d_b0, eq5_e219_d_b1];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let eq6_e222: f64 = (p.p3 * var_isub);
        let eq6_e222_d_n0: f64 = (p.p3 * var_isub_dn0);
        let eq6_e222_d_n1: f64 = (p.p3 * var_isub_dn1);
        let eq6_e222_d_n2: f64 = (p.p3 * var_isub_dn2);
        let eq6_e222_d_n3: f64 = (p.p3 * var_isub_dn3);
        let eq6_e222_d_n4: f64 = (p.p3 * var_isub_dn4);
        let eq6_e222_d_n5: f64 = (p.p3 * var_isub_dn5);
        let eq6_e222_d_n6: f64 = (p.p3 * var_isub_dn6);
        let eq6_e222_d_n7: f64 = (p.p3 * var_isub_dn7);
        let eq6_e222_d_n8: f64 = (p.p3 * var_isub_dn8);
        let eq6_e222_d_n9: f64 = (p.p3 * var_isub_dn9);
        let eq6_e222_d_n10: f64 = (p.p3 * var_isub_dn10);
        let eq6_e222_d_n11: f64 = (p.p3 * var_isub_dn11);
        let eq6_e222_d_n12: f64 = (p.p3 * var_isub_dn12);
        let eq6_e222_d_b0: f64 = (p.p3 * var_isub_db0);
        let eq6_e222_d_b1: f64 = (p.p3 * var_isub_db1);
        let eq6_e224: f64 = (eq6_e222 * p.p1);
        let eq6_e224_d_n0: f64 = (eq6_e222_d_n0 * p.p1);
        let eq6_e224_d_n1: f64 = (eq6_e222_d_n1 * p.p1);
        let eq6_e224_d_n2: f64 = (eq6_e222_d_n2 * p.p1);
        let eq6_e224_d_n3: f64 = (eq6_e222_d_n3 * p.p1);
        let eq6_e224_d_n4: f64 = (eq6_e222_d_n4 * p.p1);
        let eq6_e224_d_n5: f64 = (eq6_e222_d_n5 * p.p1);
        let eq6_e224_d_n6: f64 = (eq6_e222_d_n6 * p.p1);
        let eq6_e224_d_n7: f64 = (eq6_e222_d_n7 * p.p1);
        let eq6_e224_d_n8: f64 = (eq6_e222_d_n8 * p.p1);
        let eq6_e224_d_n9: f64 = (eq6_e222_d_n9 * p.p1);
        let eq6_e224_d_n10: f64 = (eq6_e222_d_n10 * p.p1);
        let eq6_e224_d_n11: f64 = (eq6_e222_d_n11 * p.p1);
        let eq6_e224_d_n12: f64 = (eq6_e222_d_n12 * p.p1);
        let eq6_e224_d_b0: f64 = (eq6_e222_d_b0 * p.p1);
        let eq6_e224_d_b1: f64 = (eq6_e222_d_b1 * p.p1);
        let eq6_value: f64 = eq6_e224;
        let eq6_node_derivatives: [f64; 13] = [eq6_e224_d_n0, eq6_e224_d_n1, eq6_e224_d_n2, eq6_e224_d_n3, eq6_e224_d_n4, eq6_e224_d_n5, eq6_e224_d_n6, eq6_e224_d_n7, eq6_e224_d_n8, eq6_e224_d_n9, eq6_e224_d_n10, eq6_e224_d_n11, eq6_e224_d_n12];
        let eq6_branch_derivatives: [f64; 2] = [eq6_e224_d_b0, eq6_e224_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(3),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let eq7_e227: f64 = (p.p3 * var_isub_int);
        let eq7_e227_d_n0: f64 = (p.p3 * var_isub_int_dn0);
        let eq7_e227_d_n1: f64 = (p.p3 * var_isub_int_dn1);
        let eq7_e227_d_n2: f64 = (p.p3 * var_isub_int_dn2);
        let eq7_e227_d_n3: f64 = (p.p3 * var_isub_int_dn3);
        let eq7_e227_d_n4: f64 = (p.p3 * var_isub_int_dn4);
        let eq7_e227_d_n5: f64 = (p.p3 * var_isub_int_dn5);
        let eq7_e227_d_n6: f64 = (p.p3 * var_isub_int_dn6);
        let eq7_e227_d_n7: f64 = (p.p3 * var_isub_int_dn7);
        let eq7_e227_d_n8: f64 = (p.p3 * var_isub_int_dn8);
        let eq7_e227_d_n9: f64 = (p.p3 * var_isub_int_dn9);
        let eq7_e227_d_n10: f64 = (p.p3 * var_isub_int_dn10);
        let eq7_e227_d_n11: f64 = (p.p3 * var_isub_int_dn11);
        let eq7_e227_d_n12: f64 = (p.p3 * var_isub_int_dn12);
        let eq7_e227_d_b0: f64 = (p.p3 * var_isub_int_db0);
        let eq7_e227_d_b1: f64 = (p.p3 * var_isub_int_db1);
        let eq7_e229: f64 = (eq7_e227 * p.p1);
        let eq7_e229_d_n0: f64 = (eq7_e227_d_n0 * p.p1);
        let eq7_e229_d_n1: f64 = (eq7_e227_d_n1 * p.p1);
        let eq7_e229_d_n2: f64 = (eq7_e227_d_n2 * p.p1);
        let eq7_e229_d_n3: f64 = (eq7_e227_d_n3 * p.p1);
        let eq7_e229_d_n4: f64 = (eq7_e227_d_n4 * p.p1);
        let eq7_e229_d_n5: f64 = (eq7_e227_d_n5 * p.p1);
        let eq7_e229_d_n6: f64 = (eq7_e227_d_n6 * p.p1);
        let eq7_e229_d_n7: f64 = (eq7_e227_d_n7 * p.p1);
        let eq7_e229_d_n8: f64 = (eq7_e227_d_n8 * p.p1);
        let eq7_e229_d_n9: f64 = (eq7_e227_d_n9 * p.p1);
        let eq7_e229_d_n10: f64 = (eq7_e227_d_n10 * p.p1);
        let eq7_e229_d_n11: f64 = (eq7_e227_d_n11 * p.p1);
        let eq7_e229_d_n12: f64 = (eq7_e227_d_n12 * p.p1);
        let eq7_e229_d_b0: f64 = (eq7_e227_d_b0 * p.p1);
        let eq7_e229_d_b1: f64 = (eq7_e227_d_b1 * p.p1);
        let eq7_value: f64 = eq7_e229;
        let eq7_node_derivatives: [f64; 13] = [eq7_e229_d_n0, eq7_e229_d_n1, eq7_e229_d_n2, eq7_e229_d_n3, eq7_e229_d_n4, eq7_e229_d_n5, eq7_e229_d_n6, eq7_e229_d_n7, eq7_e229_d_n8, eq7_e229_d_n9, eq7_e229_d_n10, eq7_e229_d_n11, eq7_e229_d_n12];
        let eq7_branch_derivatives: [f64; 2] = [eq7_e229_d_b0, eq7_e229_d_b1];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(3),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
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
        var_i_cth: f64,
        var_i_cth_db0: f64,
        var_i_cth_db1: f64,
        var_i_cth_dn0: f64,
        var_i_cth_dn1: f64,
        var_i_cth_dn10: f64,
        var_i_cth_dn11: f64,
        var_i_cth_dn12: f64,
        var_i_cth_dn2: f64,
        var_i_cth_dn3: f64,
        var_i_cth_dn4: f64,
        var_i_cth_dn5: f64,
        var_i_cth_dn6: f64,
        var_i_cth_dn7: f64,
        var_i_cth_dn8: f64,
        var_i_cth_dn9: f64,
        var_iavl: f64,
        var_iavl_db0: f64,
        var_iavl_db1: f64,
        var_iavl_dn0: f64,
        var_iavl_dn1: f64,
        var_iavl_dn10: f64,
        var_iavl_dn11: f64,
        var_iavl_dn12: f64,
        var_iavl_dn2: f64,
        var_iavl_dn3: f64,
        var_iavl_dn4: f64,
        var_iavl_dn5: f64,
        var_iavl_dn6: f64,
        var_iavl_dn7: f64,
        var_iavl_dn8: f64,
        var_iavl_dn9: f64,
        var_ib1b2: f64,
        var_ib1b2_db0: f64,
        var_ib1b2_db1: f64,
        var_ib1b2_dn0: f64,
        var_ib1b2_dn1: f64,
        var_ib1b2_dn10: f64,
        var_ib1b2_dn11: f64,
        var_ib1b2_dn12: f64,
        var_ib1b2_dn2: f64,
        var_ib1b2_dn3: f64,
        var_ib1b2_dn4: f64,
        var_ib1b2_dn5: f64,
        var_ib1b2_dn6: f64,
        var_ib1b2_dn7: f64,
        var_ib1b2_dn8: f64,
        var_ib1b2_dn9: f64,
        var_isf: f64,
        var_isf_db0: f64,
        var_isf_db1: f64,
        var_isf_dn0: f64,
        var_isf_dn1: f64,
        var_isf_dn10: f64,
        var_isf_dn11: f64,
        var_isf_dn12: f64,
        var_isf_dn2: f64,
        var_isf_dn3: f64,
        var_isf_dn4: f64,
        var_isf_dn5: f64,
        var_isf_dn6: f64,
        var_isf_dn7: f64,
        var_isf_dn8: f64,
        var_isf_dn9: f64,
        var_p_rth: f64,
        var_p_rth_db0: f64,
        var_p_rth_db1: f64,
        var_p_rth_dn0: f64,
        var_p_rth_dn1: f64,
        var_p_rth_dn10: f64,
        var_p_rth_dn11: f64,
        var_p_rth_dn12: f64,
        var_p_rth_dn2: f64,
        var_p_rth_dn3: f64,
        var_p_rth_dn4: f64,
        var_p_rth_dn5: f64,
        var_p_rth_dn6: f64,
        var_p_rth_dn7: f64,
        var_p_rth_dn8: f64,
        var_p_rth_dn9: f64,
        var_power: f64,
        var_power_db0: f64,
        var_power_db1: f64,
        var_power_dn0: f64,
        var_power_dn1: f64,
        var_power_dn10: f64,
        var_power_dn11: f64,
        var_power_dn12: f64,
        var_power_dn2: f64,
        var_power_dn3: f64,
        var_power_dn4: f64,
        var_power_dn5: f64,
        var_power_dn6: f64,
        var_power_dn7: f64,
        var_power_dn8: f64,
        var_power_dn9: f64,
        var_qbe: f64,
        var_qbe_db0: f64,
        var_qbe_db1: f64,
        var_qbe_dn0: f64,
        var_qbe_dn1: f64,
        var_qbe_dn10: f64,
        var_qbe_dn11: f64,
        var_qbe_dn12: f64,
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
        var_qe_dn12: f64,
        var_qe_dn2: f64,
        var_qe_dn3: f64,
        var_qe_dn4: f64,
        var_qe_dn5: f64,
        var_qe_dn6: f64,
        var_qe_dn7: f64,
        var_qe_dn8: f64,
        var_qe_dn9: f64,
        var_qte: f64,
        var_qte_db0: f64,
        var_qte_db1: f64,
        var_qte_dn0: f64,
        var_qte_dn1: f64,
        var_qte_dn10: f64,
        var_qte_dn11: f64,
        var_qte_dn12: f64,
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
        var_qte_s_dn12: f64,
        var_qte_s_dn2: f64,
        var_qte_s_dn3: f64,
        var_qte_s_dn4: f64,
        var_qte_s_dn5: f64,
        var_qte_s_dn6: f64,
        var_qte_s_dn7: f64,
        var_qte_s_dn8: f64,
        var_qte_s_dn9: f64,
        var_rbc_t: f64,
        var_rbc_t_db0: f64,
        var_rbc_t_db1: f64,
        var_rbc_t_dn0: f64,
        var_rbc_t_dn1: f64,
        var_rbc_t_dn10: f64,
        var_rbc_t_dn11: f64,
        var_rbc_t_dn12: f64,
        var_rbc_t_dn2: f64,
        var_rbc_t_dn3: f64,
        var_rbc_t_dn4: f64,
        var_rbc_t_dn5: f64,
        var_rbc_t_dn6: f64,
        var_rbc_t_dn7: f64,
        var_rbc_t_dn8: f64,
        var_rbc_t_dn9: f64,
        var_re_t: f64,
        var_re_t_db0: f64,
        var_re_t_db1: f64,
        var_re_t_dn0: f64,
        var_re_t_dn1: f64,
        var_re_t_dn10: f64,
        var_re_t_dn11: f64,
        var_re_t_dn12: f64,
        var_re_t_dn2: f64,
        var_re_t_dn3: f64,
        var_re_t_dn4: f64,
        var_re_t_dn5: f64,
        var_re_t_dn6: f64,
        var_re_t_dn7: f64,
        var_re_t_dn8: f64,
        var_re_t_dn9: f64,
        var_vbb1: f64,
        var_vbb1_db0: f64,
        var_vbb1_db1: f64,
        var_vbb1_dn0: f64,
        var_vbb1_dn1: f64,
        var_vbb1_dn10: f64,
        var_vbb1_dn11: f64,
        var_vbb1_dn12: f64,
        var_vbb1_dn2: f64,
        var_vbb1_dn3: f64,
        var_vbb1_dn4: f64,
        var_vbb1_dn5: f64,
        var_vbb1_dn6: f64,
        var_vbb1_dn7: f64,
        var_vbb1_dn8: f64,
        var_vbb1_dn9: f64,
        var_vee1: f64,
        var_vee1_db0: f64,
        var_vee1_db1: f64,
        var_vee1_dn0: f64,
        var_vee1_dn1: f64,
        var_vee1_dn10: f64,
        var_vee1_dn11: f64,
        var_vee1_dn12: f64,
        var_vee1_dn2: f64,
        var_vee1_dn3: f64,
        var_vee1_dn4: f64,
        var_vee1_dn5: f64,
        var_vee1_dn6: f64,
        var_vee1_dn7: f64,
        var_vee1_dn8: f64,
        var_vee1_dn9: f64,
        var_xisub: f64,
        var_xisub_db0: f64,
        var_xisub_db1: f64,
        var_xisub_dn0: f64,
        var_xisub_dn1: f64,
        var_xisub_dn10: f64,
        var_xisub_dn11: f64,
        var_xisub_dn12: f64,
        var_xisub_dn2: f64,
        var_xisub_dn3: f64,
        var_xisub_dn4: f64,
        var_xisub_dn5: f64,
        var_xisub_dn6: f64,
        var_xisub_dn7: f64,
        var_xisub_dn8: f64,
        var_xisub_dn9: f64,
    ) {
        let eq8_e232: f64 = (p.p3 * var_xisub);
        let eq8_e232_d_n0: f64 = (p.p3 * var_xisub_dn0);
        let eq8_e232_d_n1: f64 = (p.p3 * var_xisub_dn1);
        let eq8_e232_d_n2: f64 = (p.p3 * var_xisub_dn2);
        let eq8_e232_d_n3: f64 = (p.p3 * var_xisub_dn3);
        let eq8_e232_d_n4: f64 = (p.p3 * var_xisub_dn4);
        let eq8_e232_d_n5: f64 = (p.p3 * var_xisub_dn5);
        let eq8_e232_d_n6: f64 = (p.p3 * var_xisub_dn6);
        let eq8_e232_d_n7: f64 = (p.p3 * var_xisub_dn7);
        let eq8_e232_d_n8: f64 = (p.p3 * var_xisub_dn8);
        let eq8_e232_d_n9: f64 = (p.p3 * var_xisub_dn9);
        let eq8_e232_d_n10: f64 = (p.p3 * var_xisub_dn10);
        let eq8_e232_d_n11: f64 = (p.p3 * var_xisub_dn11);
        let eq8_e232_d_n12: f64 = (p.p3 * var_xisub_dn12);
        let eq8_e232_d_b0: f64 = (p.p3 * var_xisub_db0);
        let eq8_e232_d_b1: f64 = (p.p3 * var_xisub_db1);
        let eq8_e234: f64 = (eq8_e232 * p.p1);
        let eq8_e234_d_n0: f64 = (eq8_e232_d_n0 * p.p1);
        let eq8_e234_d_n1: f64 = (eq8_e232_d_n1 * p.p1);
        let eq8_e234_d_n2: f64 = (eq8_e232_d_n2 * p.p1);
        let eq8_e234_d_n3: f64 = (eq8_e232_d_n3 * p.p1);
        let eq8_e234_d_n4: f64 = (eq8_e232_d_n4 * p.p1);
        let eq8_e234_d_n5: f64 = (eq8_e232_d_n5 * p.p1);
        let eq8_e234_d_n6: f64 = (eq8_e232_d_n6 * p.p1);
        let eq8_e234_d_n7: f64 = (eq8_e232_d_n7 * p.p1);
        let eq8_e234_d_n8: f64 = (eq8_e232_d_n8 * p.p1);
        let eq8_e234_d_n9: f64 = (eq8_e232_d_n9 * p.p1);
        let eq8_e234_d_n10: f64 = (eq8_e232_d_n10 * p.p1);
        let eq8_e234_d_n11: f64 = (eq8_e232_d_n11 * p.p1);
        let eq8_e234_d_n12: f64 = (eq8_e232_d_n12 * p.p1);
        let eq8_e234_d_b0: f64 = (eq8_e232_d_b0 * p.p1);
        let eq8_e234_d_b1: f64 = (eq8_e232_d_b1 * p.p1);
        let eq8_value: f64 = eq8_e234;
        let eq8_node_derivatives: [f64; 13] = [eq8_e234_d_n0, eq8_e234_d_n1, eq8_e234_d_n2, eq8_e234_d_n3, eq8_e234_d_n4, eq8_e234_d_n5, eq8_e234_d_n6, eq8_e234_d_n7, eq8_e234_d_n8, eq8_e234_d_n9, eq8_e234_d_n10, eq8_e234_d_n11, eq8_e234_d_n12];
        let eq8_branch_derivatives: [f64; 2] = [eq8_e234_d_b0, eq8_e234_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(3),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let eq9_e237: f64 = (p.p3 * var_isf);
        let eq9_e237_d_n0: f64 = (p.p3 * var_isf_dn0);
        let eq9_e237_d_n1: f64 = (p.p3 * var_isf_dn1);
        let eq9_e237_d_n2: f64 = (p.p3 * var_isf_dn2);
        let eq9_e237_d_n3: f64 = (p.p3 * var_isf_dn3);
        let eq9_e237_d_n4: f64 = (p.p3 * var_isf_dn4);
        let eq9_e237_d_n5: f64 = (p.p3 * var_isf_dn5);
        let eq9_e237_d_n6: f64 = (p.p3 * var_isf_dn6);
        let eq9_e237_d_n7: f64 = (p.p3 * var_isf_dn7);
        let eq9_e237_d_n8: f64 = (p.p3 * var_isf_dn8);
        let eq9_e237_d_n9: f64 = (p.p3 * var_isf_dn9);
        let eq9_e237_d_n10: f64 = (p.p3 * var_isf_dn10);
        let eq9_e237_d_n11: f64 = (p.p3 * var_isf_dn11);
        let eq9_e237_d_n12: f64 = (p.p3 * var_isf_dn12);
        let eq9_e237_d_b0: f64 = (p.p3 * var_isf_db0);
        let eq9_e237_d_b1: f64 = (p.p3 * var_isf_db1);
        let eq9_e239: f64 = (eq9_e237 * p.p1);
        let eq9_e239_d_n0: f64 = (eq9_e237_d_n0 * p.p1);
        let eq9_e239_d_n1: f64 = (eq9_e237_d_n1 * p.p1);
        let eq9_e239_d_n2: f64 = (eq9_e237_d_n2 * p.p1);
        let eq9_e239_d_n3: f64 = (eq9_e237_d_n3 * p.p1);
        let eq9_e239_d_n4: f64 = (eq9_e237_d_n4 * p.p1);
        let eq9_e239_d_n5: f64 = (eq9_e237_d_n5 * p.p1);
        let eq9_e239_d_n6: f64 = (eq9_e237_d_n6 * p.p1);
        let eq9_e239_d_n7: f64 = (eq9_e237_d_n7 * p.p1);
        let eq9_e239_d_n8: f64 = (eq9_e237_d_n8 * p.p1);
        let eq9_e239_d_n9: f64 = (eq9_e237_d_n9 * p.p1);
        let eq9_e239_d_n10: f64 = (eq9_e237_d_n10 * p.p1);
        let eq9_e239_d_n11: f64 = (eq9_e237_d_n11 * p.p1);
        let eq9_e239_d_n12: f64 = (eq9_e237_d_n12 * p.p1);
        let eq9_e239_d_b0: f64 = (eq9_e237_d_b0 * p.p1);
        let eq9_e239_d_b1: f64 = (eq9_e237_d_b1 * p.p1);
        let eq9_value: f64 = eq9_e239;
        let eq9_node_derivatives: [f64; 13] = [eq9_e239_d_n0, eq9_e239_d_n1, eq9_e239_d_n2, eq9_e239_d_n3, eq9_e239_d_n4, eq9_e239_d_n5, eq9_e239_d_n6, eq9_e239_d_n7, eq9_e239_d_n8, eq9_e239_d_n9, eq9_e239_d_n10, eq9_e239_d_n11, eq9_e239_d_n12];
        let eq9_branch_derivatives: [f64; 2] = [eq9_e239_d_b0, eq9_e239_d_b1];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(8),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let eq10_e242: f64 = (p.p3 * var_ib1b2);
        let eq10_e242_d_n0: f64 = (p.p3 * var_ib1b2_dn0);
        let eq10_e242_d_n1: f64 = (p.p3 * var_ib1b2_dn1);
        let eq10_e242_d_n2: f64 = (p.p3 * var_ib1b2_dn2);
        let eq10_e242_d_n3: f64 = (p.p3 * var_ib1b2_dn3);
        let eq10_e242_d_n4: f64 = (p.p3 * var_ib1b2_dn4);
        let eq10_e242_d_n5: f64 = (p.p3 * var_ib1b2_dn5);
        let eq10_e242_d_n6: f64 = (p.p3 * var_ib1b2_dn6);
        let eq10_e242_d_n7: f64 = (p.p3 * var_ib1b2_dn7);
        let eq10_e242_d_n8: f64 = (p.p3 * var_ib1b2_dn8);
        let eq10_e242_d_n9: f64 = (p.p3 * var_ib1b2_dn9);
        let eq10_e242_d_n10: f64 = (p.p3 * var_ib1b2_dn10);
        let eq10_e242_d_n11: f64 = (p.p3 * var_ib1b2_dn11);
        let eq10_e242_d_n12: f64 = (p.p3 * var_ib1b2_dn12);
        let eq10_e242_d_b0: f64 = (p.p3 * var_ib1b2_db0);
        let eq10_e242_d_b1: f64 = (p.p3 * var_ib1b2_db1);
        let eq10_e244: f64 = (eq10_e242 * p.p1);
        let eq10_e244_d_n0: f64 = (eq10_e242_d_n0 * p.p1);
        let eq10_e244_d_n1: f64 = (eq10_e242_d_n1 * p.p1);
        let eq10_e244_d_n2: f64 = (eq10_e242_d_n2 * p.p1);
        let eq10_e244_d_n3: f64 = (eq10_e242_d_n3 * p.p1);
        let eq10_e244_d_n4: f64 = (eq10_e242_d_n4 * p.p1);
        let eq10_e244_d_n5: f64 = (eq10_e242_d_n5 * p.p1);
        let eq10_e244_d_n6: f64 = (eq10_e242_d_n6 * p.p1);
        let eq10_e244_d_n7: f64 = (eq10_e242_d_n7 * p.p1);
        let eq10_e244_d_n8: f64 = (eq10_e242_d_n8 * p.p1);
        let eq10_e244_d_n9: f64 = (eq10_e242_d_n9 * p.p1);
        let eq10_e244_d_n10: f64 = (eq10_e242_d_n10 * p.p1);
        let eq10_e244_d_n11: f64 = (eq10_e242_d_n11 * p.p1);
        let eq10_e244_d_n12: f64 = (eq10_e242_d_n12 * p.p1);
        let eq10_e244_d_b0: f64 = (eq10_e242_d_b0 * p.p1);
        let eq10_e244_d_b1: f64 = (eq10_e242_d_b1 * p.p1);
        let eq10_value: f64 = eq10_e244;
        let eq10_node_derivatives: [f64; 13] = [eq10_e244_d_n0, eq10_e244_d_n1, eq10_e244_d_n2, eq10_e244_d_n3, eq10_e244_d_n4, eq10_e244_d_n5, eq10_e244_d_n6, eq10_e244_d_n7, eq10_e244_d_n8, eq10_e244_d_n9, eq10_e244_d_n10, eq10_e244_d_n11, eq10_e244_d_n12];
        let eq10_branch_derivatives: [f64; 2] = [eq10_e244_d_b0, eq10_e244_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e247: f64 = (-1.0);
        let eq11_e249: f64 = (eq11_e247 * var_iavl);
        let eq11_e249_d_n0: f64 = (eq11_e247 * var_iavl_dn0);
        let eq11_e249_d_n1: f64 = (eq11_e247 * var_iavl_dn1);
        let eq11_e249_d_n2: f64 = (eq11_e247 * var_iavl_dn2);
        let eq11_e249_d_n3: f64 = (eq11_e247 * var_iavl_dn3);
        let eq11_e249_d_n4: f64 = (eq11_e247 * var_iavl_dn4);
        let eq11_e249_d_n5: f64 = (eq11_e247 * var_iavl_dn5);
        let eq11_e249_d_n6: f64 = (eq11_e247 * var_iavl_dn6);
        let eq11_e249_d_n7: f64 = (eq11_e247 * var_iavl_dn7);
        let eq11_e249_d_n8: f64 = (eq11_e247 * var_iavl_dn8);
        let eq11_e249_d_n9: f64 = (eq11_e247 * var_iavl_dn9);
        let eq11_e249_d_n10: f64 = (eq11_e247 * var_iavl_dn10);
        let eq11_e249_d_n11: f64 = (eq11_e247 * var_iavl_dn11);
        let eq11_e249_d_n12: f64 = (eq11_e247 * var_iavl_dn12);
        let eq11_e249_d_b0: f64 = (eq11_e247 * var_iavl_db0);
        let eq11_e249_d_b1: f64 = (eq11_e247 * var_iavl_db1);
        let eq11_e250: f64 = (p.p3 * eq11_e249);
        let eq11_e250_d_n0: f64 = (p.p3 * eq11_e249_d_n0);
        let eq11_e250_d_n1: f64 = (p.p3 * eq11_e249_d_n1);
        let eq11_e250_d_n2: f64 = (p.p3 * eq11_e249_d_n2);
        let eq11_e250_d_n3: f64 = (p.p3 * eq11_e249_d_n3);
        let eq11_e250_d_n4: f64 = (p.p3 * eq11_e249_d_n4);
        let eq11_e250_d_n5: f64 = (p.p3 * eq11_e249_d_n5);
        let eq11_e250_d_n6: f64 = (p.p3 * eq11_e249_d_n6);
        let eq11_e250_d_n7: f64 = (p.p3 * eq11_e249_d_n7);
        let eq11_e250_d_n8: f64 = (p.p3 * eq11_e249_d_n8);
        let eq11_e250_d_n9: f64 = (p.p3 * eq11_e249_d_n9);
        let eq11_e250_d_n10: f64 = (p.p3 * eq11_e249_d_n10);
        let eq11_e250_d_n11: f64 = (p.p3 * eq11_e249_d_n11);
        let eq11_e250_d_n12: f64 = (p.p3 * eq11_e249_d_n12);
        let eq11_e250_d_b0: f64 = (p.p3 * eq11_e249_d_b0);
        let eq11_e250_d_b1: f64 = (p.p3 * eq11_e249_d_b1);
        let eq11_e252: f64 = (eq11_e250 * p.p1);
        let eq11_e252_d_n0: f64 = (eq11_e250_d_n0 * p.p1);
        let eq11_e252_d_n1: f64 = (eq11_e250_d_n1 * p.p1);
        let eq11_e252_d_n2: f64 = (eq11_e250_d_n2 * p.p1);
        let eq11_e252_d_n3: f64 = (eq11_e250_d_n3 * p.p1);
        let eq11_e252_d_n4: f64 = (eq11_e250_d_n4 * p.p1);
        let eq11_e252_d_n5: f64 = (eq11_e250_d_n5 * p.p1);
        let eq11_e252_d_n6: f64 = (eq11_e250_d_n6 * p.p1);
        let eq11_e252_d_n7: f64 = (eq11_e250_d_n7 * p.p1);
        let eq11_e252_d_n8: f64 = (eq11_e250_d_n8 * p.p1);
        let eq11_e252_d_n9: f64 = (eq11_e250_d_n9 * p.p1);
        let eq11_e252_d_n10: f64 = (eq11_e250_d_n10 * p.p1);
        let eq11_e252_d_n11: f64 = (eq11_e250_d_n11 * p.p1);
        let eq11_e252_d_n12: f64 = (eq11_e250_d_n12 * p.p1);
        let eq11_e252_d_b0: f64 = (eq11_e250_d_b0 * p.p1);
        let eq11_e252_d_b1: f64 = (eq11_e250_d_b1 * p.p1);
        let eq11_value: f64 = eq11_e252;
        let eq11_node_derivatives: [f64; 13] = [eq11_e252_d_n0, eq11_e252_d_n1, eq11_e252_d_n2, eq11_e252_d_n3, eq11_e252_d_n4, eq11_e252_d_n5, eq11_e252_d_n6, eq11_e252_d_n7, eq11_e252_d_n8, eq11_e252_d_n9, eq11_e252_d_n10, eq11_e252_d_n11, eq11_e252_d_n12];
        let eq11_branch_derivatives: [f64; 2] = [eq11_e252_d_b0, eq11_e252_d_b1];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e255: f64 = (p.p3 * var_vee1);
        let eq12_e255_d_n0: f64 = (p.p3 * var_vee1_dn0);
        let eq12_e255_d_n1: f64 = (p.p3 * var_vee1_dn1);
        let eq12_e255_d_n2: f64 = (p.p3 * var_vee1_dn2);
        let eq12_e255_d_n3: f64 = (p.p3 * var_vee1_dn3);
        let eq12_e255_d_n4: f64 = (p.p3 * var_vee1_dn4);
        let eq12_e255_d_n5: f64 = (p.p3 * var_vee1_dn5);
        let eq12_e255_d_n6: f64 = (p.p3 * var_vee1_dn6);
        let eq12_e255_d_n7: f64 = (p.p3 * var_vee1_dn7);
        let eq12_e255_d_n8: f64 = (p.p3 * var_vee1_dn8);
        let eq12_e255_d_n9: f64 = (p.p3 * var_vee1_dn9);
        let eq12_e255_d_n10: f64 = (p.p3 * var_vee1_dn10);
        let eq12_e255_d_n11: f64 = (p.p3 * var_vee1_dn11);
        let eq12_e255_d_n12: f64 = (p.p3 * var_vee1_dn12);
        let eq12_e255_d_b0: f64 = (p.p3 * var_vee1_db0);
        let eq12_e255_d_b1: f64 = (p.p3 * var_vee1_db1);
        let eq12_e257: f64 = (eq12_e255 / var_re_t);
        let __rspice_inv_cse_0: f64 = 1.0 / (var_re_t * var_re_t);
        let eq12_e257_d_n0: f64 = (((eq12_e255_d_n0 * var_re_t) - (eq12_e255 * var_re_t_dn0)) * __rspice_inv_cse_0);
        let eq12_e257_d_n1: f64 = (((eq12_e255_d_n1 * var_re_t) - (eq12_e255 * var_re_t_dn1)) * __rspice_inv_cse_0);
        let eq12_e257_d_n2: f64 = (((eq12_e255_d_n2 * var_re_t) - (eq12_e255 * var_re_t_dn2)) * __rspice_inv_cse_0);
        let eq12_e257_d_n3: f64 = (((eq12_e255_d_n3 * var_re_t) - (eq12_e255 * var_re_t_dn3)) * __rspice_inv_cse_0);
        let eq12_e257_d_n4: f64 = (((eq12_e255_d_n4 * var_re_t) - (eq12_e255 * var_re_t_dn4)) * __rspice_inv_cse_0);
        let eq12_e257_d_n5: f64 = (((eq12_e255_d_n5 * var_re_t) - (eq12_e255 * var_re_t_dn5)) * __rspice_inv_cse_0);
        let eq12_e257_d_n6: f64 = (((eq12_e255_d_n6 * var_re_t) - (eq12_e255 * var_re_t_dn6)) * __rspice_inv_cse_0);
        let eq12_e257_d_n7: f64 = (((eq12_e255_d_n7 * var_re_t) - (eq12_e255 * var_re_t_dn7)) * __rspice_inv_cse_0);
        let eq12_e257_d_n8: f64 = (((eq12_e255_d_n8 * var_re_t) - (eq12_e255 * var_re_t_dn8)) * __rspice_inv_cse_0);
        let eq12_e257_d_n9: f64 = (((eq12_e255_d_n9 * var_re_t) - (eq12_e255 * var_re_t_dn9)) * __rspice_inv_cse_0);
        let eq12_e257_d_n10: f64 = (((eq12_e255_d_n10 * var_re_t) - (eq12_e255 * var_re_t_dn10)) * __rspice_inv_cse_0);
        let eq12_e257_d_n11: f64 = (((eq12_e255_d_n11 * var_re_t) - (eq12_e255 * var_re_t_dn11)) * __rspice_inv_cse_0);
        let eq12_e257_d_n12: f64 = (((eq12_e255_d_n12 * var_re_t) - (eq12_e255 * var_re_t_dn12)) * __rspice_inv_cse_0);
        let eq12_e257_d_b0: f64 = (((eq12_e255_d_b0 * var_re_t) - (eq12_e255 * var_re_t_db0)) * __rspice_inv_cse_0);
        let eq12_e257_d_b1: f64 = (((eq12_e255_d_b1 * var_re_t) - (eq12_e255 * var_re_t_db1)) * __rspice_inv_cse_0);
        let eq12_e259: f64 = (eq12_e257 * p.p1);
        let eq12_e259_d_n0: f64 = (eq12_e257_d_n0 * p.p1);
        let eq12_e259_d_n1: f64 = (eq12_e257_d_n1 * p.p1);
        let eq12_e259_d_n2: f64 = (eq12_e257_d_n2 * p.p1);
        let eq12_e259_d_n3: f64 = (eq12_e257_d_n3 * p.p1);
        let eq12_e259_d_n4: f64 = (eq12_e257_d_n4 * p.p1);
        let eq12_e259_d_n5: f64 = (eq12_e257_d_n5 * p.p1);
        let eq12_e259_d_n6: f64 = (eq12_e257_d_n6 * p.p1);
        let eq12_e259_d_n7: f64 = (eq12_e257_d_n7 * p.p1);
        let eq12_e259_d_n8: f64 = (eq12_e257_d_n8 * p.p1);
        let eq12_e259_d_n9: f64 = (eq12_e257_d_n9 * p.p1);
        let eq12_e259_d_n10: f64 = (eq12_e257_d_n10 * p.p1);
        let eq12_e259_d_n11: f64 = (eq12_e257_d_n11 * p.p1);
        let eq12_e259_d_n12: f64 = (eq12_e257_d_n12 * p.p1);
        let eq12_e259_d_b0: f64 = (eq12_e257_d_b0 * p.p1);
        let eq12_e259_d_b1: f64 = (eq12_e257_d_b1 * p.p1);
        let eq12_value: f64 = eq12_e259;
        let eq12_node_derivatives: [f64; 13] = [eq12_e259_d_n0, eq12_e259_d_n1, eq12_e259_d_n2, eq12_e259_d_n3, eq12_e259_d_n4, eq12_e259_d_n5, eq12_e259_d_n6, eq12_e259_d_n7, eq12_e259_d_n8, eq12_e259_d_n9, eq12_e259_d_n10, eq12_e259_d_n11, eq12_e259_d_n12];
        let eq12_branch_derivatives: [f64; 2] = [eq12_e259_d_b0, eq12_e259_d_b1];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(5),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_e262: f64 = (p.p3 * var_vbb1);
        let eq13_e262_d_n0: f64 = (p.p3 * var_vbb1_dn0);
        let eq13_e262_d_n1: f64 = (p.p3 * var_vbb1_dn1);
        let eq13_e262_d_n2: f64 = (p.p3 * var_vbb1_dn2);
        let eq13_e262_d_n3: f64 = (p.p3 * var_vbb1_dn3);
        let eq13_e262_d_n4: f64 = (p.p3 * var_vbb1_dn4);
        let eq13_e262_d_n5: f64 = (p.p3 * var_vbb1_dn5);
        let eq13_e262_d_n6: f64 = (p.p3 * var_vbb1_dn6);
        let eq13_e262_d_n7: f64 = (p.p3 * var_vbb1_dn7);
        let eq13_e262_d_n8: f64 = (p.p3 * var_vbb1_dn8);
        let eq13_e262_d_n9: f64 = (p.p3 * var_vbb1_dn9);
        let eq13_e262_d_n10: f64 = (p.p3 * var_vbb1_dn10);
        let eq13_e262_d_n11: f64 = (p.p3 * var_vbb1_dn11);
        let eq13_e262_d_n12: f64 = (p.p3 * var_vbb1_dn12);
        let eq13_e262_d_b0: f64 = (p.p3 * var_vbb1_db0);
        let eq13_e262_d_b1: f64 = (p.p3 * var_vbb1_db1);
        let eq13_e264: f64 = (eq13_e262 / var_rbc_t);
        let __rspice_inv_cse_1: f64 = 1.0 / (var_rbc_t * var_rbc_t);
        let eq13_e264_d_n0: f64 = (((eq13_e262_d_n0 * var_rbc_t) - (eq13_e262 * var_rbc_t_dn0)) * __rspice_inv_cse_1);
        let eq13_e264_d_n1: f64 = (((eq13_e262_d_n1 * var_rbc_t) - (eq13_e262 * var_rbc_t_dn1)) * __rspice_inv_cse_1);
        let eq13_e264_d_n2: f64 = (((eq13_e262_d_n2 * var_rbc_t) - (eq13_e262 * var_rbc_t_dn2)) * __rspice_inv_cse_1);
        let eq13_e264_d_n3: f64 = (((eq13_e262_d_n3 * var_rbc_t) - (eq13_e262 * var_rbc_t_dn3)) * __rspice_inv_cse_1);
        let eq13_e264_d_n4: f64 = (((eq13_e262_d_n4 * var_rbc_t) - (eq13_e262 * var_rbc_t_dn4)) * __rspice_inv_cse_1);
        let eq13_e264_d_n5: f64 = (((eq13_e262_d_n5 * var_rbc_t) - (eq13_e262 * var_rbc_t_dn5)) * __rspice_inv_cse_1);
        let eq13_e264_d_n6: f64 = (((eq13_e262_d_n6 * var_rbc_t) - (eq13_e262 * var_rbc_t_dn6)) * __rspice_inv_cse_1);
        let eq13_e264_d_n7: f64 = (((eq13_e262_d_n7 * var_rbc_t) - (eq13_e262 * var_rbc_t_dn7)) * __rspice_inv_cse_1);
        let eq13_e264_d_n8: f64 = (((eq13_e262_d_n8 * var_rbc_t) - (eq13_e262 * var_rbc_t_dn8)) * __rspice_inv_cse_1);
        let eq13_e264_d_n9: f64 = (((eq13_e262_d_n9 * var_rbc_t) - (eq13_e262 * var_rbc_t_dn9)) * __rspice_inv_cse_1);
        let eq13_e264_d_n10: f64 = (((eq13_e262_d_n10 * var_rbc_t) - (eq13_e262 * var_rbc_t_dn10)) * __rspice_inv_cse_1);
        let eq13_e264_d_n11: f64 = (((eq13_e262_d_n11 * var_rbc_t) - (eq13_e262 * var_rbc_t_dn11)) * __rspice_inv_cse_1);
        let eq13_e264_d_n12: f64 = (((eq13_e262_d_n12 * var_rbc_t) - (eq13_e262 * var_rbc_t_dn12)) * __rspice_inv_cse_1);
        let eq13_e264_d_b0: f64 = (((eq13_e262_d_b0 * var_rbc_t) - (eq13_e262 * var_rbc_t_db0)) * __rspice_inv_cse_1);
        let eq13_e264_d_b1: f64 = (((eq13_e262_d_b1 * var_rbc_t) - (eq13_e262 * var_rbc_t_db1)) * __rspice_inv_cse_1);
        let eq13_e266: f64 = (eq13_e264 * p.p1);
        let eq13_e266_d_n0: f64 = (eq13_e264_d_n0 * p.p1);
        let eq13_e266_d_n1: f64 = (eq13_e264_d_n1 * p.p1);
        let eq13_e266_d_n2: f64 = (eq13_e264_d_n2 * p.p1);
        let eq13_e266_d_n3: f64 = (eq13_e264_d_n3 * p.p1);
        let eq13_e266_d_n4: f64 = (eq13_e264_d_n4 * p.p1);
        let eq13_e266_d_n5: f64 = (eq13_e264_d_n5 * p.p1);
        let eq13_e266_d_n6: f64 = (eq13_e264_d_n6 * p.p1);
        let eq13_e266_d_n7: f64 = (eq13_e264_d_n7 * p.p1);
        let eq13_e266_d_n8: f64 = (eq13_e264_d_n8 * p.p1);
        let eq13_e266_d_n9: f64 = (eq13_e264_d_n9 * p.p1);
        let eq13_e266_d_n10: f64 = (eq13_e264_d_n10 * p.p1);
        let eq13_e266_d_n11: f64 = (eq13_e264_d_n11 * p.p1);
        let eq13_e266_d_n12: f64 = (eq13_e264_d_n12 * p.p1);
        let eq13_e266_d_b0: f64 = (eq13_e264_d_b0 * p.p1);
        let eq13_e266_d_b1: f64 = (eq13_e264_d_b1 * p.p1);
        let eq13_value: f64 = eq13_e266;
        let eq13_node_derivatives: [f64; 13] = [eq13_e266_d_n0, eq13_e266_d_n1, eq13_e266_d_n2, eq13_e266_d_n3, eq13_e266_d_n4, eq13_e266_d_n5, eq13_e266_d_n6, eq13_e266_d_n7, eq13_e266_d_n8, eq13_e266_d_n9, eq13_e266_d_n10, eq13_e266_d_n11, eq13_e266_d_n12];
        let eq13_branch_derivatives: [f64; 2] = [eq13_e266_d_b0, eq13_e266_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(6),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_value: f64 = var_p_rth;
        let eq14_node_derivatives: [f64; 13] = [var_p_rth_dn0, var_p_rth_dn1, var_p_rth_dn2, var_p_rth_dn3, var_p_rth_dn4, var_p_rth_dn5, var_p_rth_dn6, var_p_rth_dn7, var_p_rth_dn8, var_p_rth_dn9, var_p_rth_dn10, var_p_rth_dn11, var_p_rth_dn12];
        let eq14_branch_derivatives: [f64; 2] = [var_p_rth_db0, var_p_rth_db1];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let eq15_value: f64 = var_i_cth;
        let eq15_node_derivatives: [f64; 13] = [var_i_cth_dn0, var_i_cth_dn1, var_i_cth_dn2, var_i_cth_dn3, var_i_cth_dn4, var_i_cth_dn5, var_i_cth_dn6, var_i_cth_dn7, var_i_cth_dn8, var_i_cth_dn9, var_i_cth_dn10, var_i_cth_dn11, var_i_cth_dn12];
        let eq15_branch_derivatives: [f64; 2] = [var_i_cth_db0, var_i_cth_db1];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let eq16_e270: f64 = (-1.0);
        let eq16_e272: f64 = (eq16_e270 * var_power);
        let eq16_e272_d_n0: f64 = (eq16_e270 * var_power_dn0);
        let eq16_e272_d_n1: f64 = (eq16_e270 * var_power_dn1);
        let eq16_e272_d_n2: f64 = (eq16_e270 * var_power_dn2);
        let eq16_e272_d_n3: f64 = (eq16_e270 * var_power_dn3);
        let eq16_e272_d_n4: f64 = (eq16_e270 * var_power_dn4);
        let eq16_e272_d_n5: f64 = (eq16_e270 * var_power_dn5);
        let eq16_e272_d_n6: f64 = (eq16_e270 * var_power_dn6);
        let eq16_e272_d_n7: f64 = (eq16_e270 * var_power_dn7);
        let eq16_e272_d_n8: f64 = (eq16_e270 * var_power_dn8);
        let eq16_e272_d_n9: f64 = (eq16_e270 * var_power_dn9);
        let eq16_e272_d_n10: f64 = (eq16_e270 * var_power_dn10);
        let eq16_e272_d_n11: f64 = (eq16_e270 * var_power_dn11);
        let eq16_e272_d_n12: f64 = (eq16_e270 * var_power_dn12);
        let eq16_e272_d_b0: f64 = (eq16_e270 * var_power_db0);
        let eq16_e272_d_b1: f64 = (eq16_e270 * var_power_db1);
        let eq16_e274: f64 = (eq16_e272 * p.p1);
        let eq16_e274_d_n0: f64 = (eq16_e272_d_n0 * p.p1);
        let eq16_e274_d_n1: f64 = (eq16_e272_d_n1 * p.p1);
        let eq16_e274_d_n2: f64 = (eq16_e272_d_n2 * p.p1);
        let eq16_e274_d_n3: f64 = (eq16_e272_d_n3 * p.p1);
        let eq16_e274_d_n4: f64 = (eq16_e272_d_n4 * p.p1);
        let eq16_e274_d_n5: f64 = (eq16_e272_d_n5 * p.p1);
        let eq16_e274_d_n6: f64 = (eq16_e272_d_n6 * p.p1);
        let eq16_e274_d_n7: f64 = (eq16_e272_d_n7 * p.p1);
        let eq16_e274_d_n8: f64 = (eq16_e272_d_n8 * p.p1);
        let eq16_e274_d_n9: f64 = (eq16_e272_d_n9 * p.p1);
        let eq16_e274_d_n10: f64 = (eq16_e272_d_n10 * p.p1);
        let eq16_e274_d_n11: f64 = (eq16_e272_d_n11 * p.p1);
        let eq16_e274_d_n12: f64 = (eq16_e272_d_n12 * p.p1);
        let eq16_e274_d_b0: f64 = (eq16_e272_d_b0 * p.p1);
        let eq16_e274_d_b1: f64 = (eq16_e272_d_b1 * p.p1);
        let eq16_value: f64 = eq16_e274;
        let eq16_node_derivatives: [f64; 13] = [eq16_e274_d_n0, eq16_e274_d_n1, eq16_e274_d_n2, eq16_e274_d_n3, eq16_e274_d_n4, eq16_e274_d_n5, eq16_e274_d_n6, eq16_e274_d_n7, eq16_e274_d_n8, eq16_e274_d_n9, eq16_e274_d_n10, eq16_e274_d_n11, eq16_e274_d_n12];
        let eq16_branch_derivatives: [f64; 2] = [eq16_e274_d_b0, eq16_e274_d_b1];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq16_value),
            &eq16_node_derivatives,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let eq17_e278: f64 = (var_qte + var_qbe);
        let eq17_e278_d_n0: f64 = (var_qte_dn0 + var_qbe_dn0);
        let eq17_e278_d_n1: f64 = (var_qte_dn1 + var_qbe_dn1);
        let eq17_e278_d_n2: f64 = (var_qte_dn2 + var_qbe_dn2);
        let eq17_e278_d_n3: f64 = (var_qte_dn3 + var_qbe_dn3);
        let eq17_e278_d_n4: f64 = (var_qte_dn4 + var_qbe_dn4);
        let eq17_e278_d_n5: f64 = (var_qte_dn5 + var_qbe_dn5);
        let eq17_e278_d_n6: f64 = (var_qte_dn6 + var_qbe_dn6);
        let eq17_e278_d_n7: f64 = (var_qte_dn7 + var_qbe_dn7);
        let eq17_e278_d_n8: f64 = (var_qte_dn8 + var_qbe_dn8);
        let eq17_e278_d_n9: f64 = (var_qte_dn9 + var_qbe_dn9);
        let eq17_e278_d_n10: f64 = (var_qte_dn10 + var_qbe_dn10);
        let eq17_e278_d_n11: f64 = (var_qte_dn11 + var_qbe_dn11);
        let eq17_e278_d_n12: f64 = (var_qte_dn12 + var_qbe_dn12);
        let eq17_e278_d_b0: f64 = (var_qte_db0 + var_qbe_db0);
        let eq17_e278_d_b1: f64 = (var_qte_db1 + var_qbe_db1);
        let eq17_e280: f64 = (eq17_e278 + var_qe);
        let eq17_e280_d_n0: f64 = (eq17_e278_d_n0 + var_qe_dn0);
        let eq17_e280_d_n1: f64 = (eq17_e278_d_n1 + var_qe_dn1);
        let eq17_e280_d_n2: f64 = (eq17_e278_d_n2 + var_qe_dn2);
        let eq17_e280_d_n3: f64 = (eq17_e278_d_n3 + var_qe_dn3);
        let eq17_e280_d_n4: f64 = (eq17_e278_d_n4 + var_qe_dn4);
        let eq17_e280_d_n5: f64 = (eq17_e278_d_n5 + var_qe_dn5);
        let eq17_e280_d_n6: f64 = (eq17_e278_d_n6 + var_qe_dn6);
        let eq17_e280_d_n7: f64 = (eq17_e278_d_n7 + var_qe_dn7);
        let eq17_e280_d_n8: f64 = (eq17_e278_d_n8 + var_qe_dn8);
        let eq17_e280_d_n9: f64 = (eq17_e278_d_n9 + var_qe_dn9);
        let eq17_e280_d_n10: f64 = (eq17_e278_d_n10 + var_qe_dn10);
        let eq17_e280_d_n11: f64 = (eq17_e278_d_n11 + var_qe_dn11);
        let eq17_e280_d_n12: f64 = (eq17_e278_d_n12 + var_qe_dn12);
        let eq17_e280_d_b0: f64 = (eq17_e278_d_b0 + var_qe_db0);
        let eq17_e280_d_b1: f64 = (eq17_e278_d_b1 + var_qe_db1);
        let eq17_e281: f64 = (p.p3 * eq17_e280);
        let eq17_e281_d_n0: f64 = (p.p3 * eq17_e280_d_n0);
        let eq17_e281_d_n1: f64 = (p.p3 * eq17_e280_d_n1);
        let eq17_e281_d_n2: f64 = (p.p3 * eq17_e280_d_n2);
        let eq17_e281_d_n3: f64 = (p.p3 * eq17_e280_d_n3);
        let eq17_e281_d_n4: f64 = (p.p3 * eq17_e280_d_n4);
        let eq17_e281_d_n5: f64 = (p.p3 * eq17_e280_d_n5);
        let eq17_e281_d_n6: f64 = (p.p3 * eq17_e280_d_n6);
        let eq17_e281_d_n7: f64 = (p.p3 * eq17_e280_d_n7);
        let eq17_e281_d_n8: f64 = (p.p3 * eq17_e280_d_n8);
        let eq17_e281_d_n9: f64 = (p.p3 * eq17_e280_d_n9);
        let eq17_e281_d_n10: f64 = (p.p3 * eq17_e280_d_n10);
        let eq17_e281_d_n11: f64 = (p.p3 * eq17_e280_d_n11);
        let eq17_e281_d_n12: f64 = (p.p3 * eq17_e280_d_n12);
        let eq17_e281_d_b0: f64 = (p.p3 * eq17_e280_d_b0);
        let eq17_e281_d_b1: f64 = (p.p3 * eq17_e280_d_b1);
        let eq17_e282: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq17_e281);
        let eq17_e284: f64 = (eq17_e282 * p.p1);
        let eq17_e284_d_n0: f64 = ((eq17_e281_d_n0 * ddt_scale) * p.p1);
        let eq17_e284_d_n1: f64 = ((eq17_e281_d_n1 * ddt_scale) * p.p1);
        let eq17_e284_d_n2: f64 = ((eq17_e281_d_n2 * ddt_scale) * p.p1);
        let eq17_e284_d_n3: f64 = ((eq17_e281_d_n3 * ddt_scale) * p.p1);
        let eq17_e284_d_n4: f64 = ((eq17_e281_d_n4 * ddt_scale) * p.p1);
        let eq17_e284_d_n5: f64 = ((eq17_e281_d_n5 * ddt_scale) * p.p1);
        let eq17_e284_d_n6: f64 = ((eq17_e281_d_n6 * ddt_scale) * p.p1);
        let eq17_e284_d_n7: f64 = ((eq17_e281_d_n7 * ddt_scale) * p.p1);
        let eq17_e284_d_n8: f64 = ((eq17_e281_d_n8 * ddt_scale) * p.p1);
        let eq17_e284_d_n9: f64 = ((eq17_e281_d_n9 * ddt_scale) * p.p1);
        let eq17_e284_d_n10: f64 = ((eq17_e281_d_n10 * ddt_scale) * p.p1);
        let eq17_e284_d_n11: f64 = ((eq17_e281_d_n11 * ddt_scale) * p.p1);
        let eq17_e284_d_n12: f64 = ((eq17_e281_d_n12 * ddt_scale) * p.p1);
        let eq17_e284_d_b0: f64 = ((eq17_e281_d_b0 * ddt_scale) * p.p1);
        let eq17_e284_d_b1: f64 = ((eq17_e281_d_b1 * ddt_scale) * p.p1);
        let eq17_value: f64 = eq17_e284;
        let eq17_node_derivatives: [f64; 13] = [eq17_e284_d_n0, eq17_e284_d_n1, eq17_e284_d_n2, eq17_e284_d_n3, eq17_e284_d_n4, eq17_e284_d_n5, eq17_e284_d_n6, eq17_e284_d_n7, eq17_e284_d_n8, eq17_e284_d_n9, eq17_e284_d_n10, eq17_e284_d_n11, eq17_e284_d_n12];
        let eq17_branch_derivatives: [f64; 2] = [eq17_e284_d_b0, eq17_e284_d_b1];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let eq18_e287: f64 = (p.p3 * var_qte_s);
        let eq18_e287_d_n0: f64 = (p.p3 * var_qte_s_dn0);
        let eq18_e287_d_n1: f64 = (p.p3 * var_qte_s_dn1);
        let eq18_e287_d_n2: f64 = (p.p3 * var_qte_s_dn2);
        let eq18_e287_d_n3: f64 = (p.p3 * var_qte_s_dn3);
        let eq18_e287_d_n4: f64 = (p.p3 * var_qte_s_dn4);
        let eq18_e287_d_n5: f64 = (p.p3 * var_qte_s_dn5);
        let eq18_e287_d_n6: f64 = (p.p3 * var_qte_s_dn6);
        let eq18_e287_d_n7: f64 = (p.p3 * var_qte_s_dn7);
        let eq18_e287_d_n8: f64 = (p.p3 * var_qte_s_dn8);
        let eq18_e287_d_n9: f64 = (p.p3 * var_qte_s_dn9);
        let eq18_e287_d_n10: f64 = (p.p3 * var_qte_s_dn10);
        let eq18_e287_d_n11: f64 = (p.p3 * var_qte_s_dn11);
        let eq18_e287_d_n12: f64 = (p.p3 * var_qte_s_dn12);
        let eq18_e287_d_b0: f64 = (p.p3 * var_qte_s_db0);
        let eq18_e287_d_b1: f64 = (p.p3 * var_qte_s_db1);
        let eq18_e288: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq18_e287);
        let eq18_e290: f64 = (eq18_e288 * p.p1);
        let eq18_e290_d_n0: f64 = ((eq18_e287_d_n0 * ddt_scale) * p.p1);
        let eq18_e290_d_n1: f64 = ((eq18_e287_d_n1 * ddt_scale) * p.p1);
        let eq18_e290_d_n2: f64 = ((eq18_e287_d_n2 * ddt_scale) * p.p1);
        let eq18_e290_d_n3: f64 = ((eq18_e287_d_n3 * ddt_scale) * p.p1);
        let eq18_e290_d_n4: f64 = ((eq18_e287_d_n4 * ddt_scale) * p.p1);
        let eq18_e290_d_n5: f64 = ((eq18_e287_d_n5 * ddt_scale) * p.p1);
        let eq18_e290_d_n6: f64 = ((eq18_e287_d_n6 * ddt_scale) * p.p1);
        let eq18_e290_d_n7: f64 = ((eq18_e287_d_n7 * ddt_scale) * p.p1);
        let eq18_e290_d_n8: f64 = ((eq18_e287_d_n8 * ddt_scale) * p.p1);
        let eq18_e290_d_n9: f64 = ((eq18_e287_d_n9 * ddt_scale) * p.p1);
        let eq18_e290_d_n10: f64 = ((eq18_e287_d_n10 * ddt_scale) * p.p1);
        let eq18_e290_d_n11: f64 = ((eq18_e287_d_n11 * ddt_scale) * p.p1);
        let eq18_e290_d_n12: f64 = ((eq18_e287_d_n12 * ddt_scale) * p.p1);
        let eq18_e290_d_b0: f64 = ((eq18_e287_d_b0 * ddt_scale) * p.p1);
        let eq18_e290_d_b1: f64 = ((eq18_e287_d_b1 * ddt_scale) * p.p1);
        let eq18_value: f64 = eq18_e290;
        let eq18_node_derivatives: [f64; 13] = [eq18_e290_d_n0, eq18_e290_d_n1, eq18_e290_d_n2, eq18_e290_d_n3, eq18_e290_d_n4, eq18_e290_d_n5, eq18_e290_d_n6, eq18_e290_d_n7, eq18_e290_d_n8, eq18_e290_d_n9, eq18_e290_d_n10, eq18_e290_d_n11, eq18_e290_d_n12];
        let eq18_branch_derivatives: [f64; 2] = [eq18_e290_d_b0, eq18_e290_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
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
        var_gcc_xx_t: f64,
        var_gcc_xx_t_db0: f64,
        var_gcc_xx_t_db1: f64,
        var_gcc_xx_t_dn0: f64,
        var_gcc_xx_t_dn1: f64,
        var_gcc_xx_t_dn10: f64,
        var_gcc_xx_t_dn11: f64,
        var_gcc_xx_t_dn12: f64,
        var_gcc_xx_t_dn2: f64,
        var_gcc_xx_t_dn3: f64,
        var_gcc_xx_t_dn4: f64,
        var_gcc_xx_t_dn5: f64,
        var_gcc_xx_t_dn6: f64,
        var_gcc_xx_t_dn7: f64,
        var_gcc_xx_t_dn8: f64,
        var_gcc_xx_t_dn9: f64,
        var_gmin: f64,
        var_ib3: f64,
        var_ib3_db0: f64,
        var_ib3_db1: f64,
        var_ib3_dn0: f64,
        var_ib3_dn1: f64,
        var_ib3_dn10: f64,
        var_ib3_dn11: f64,
        var_ib3_dn12: f64,
        var_ib3_dn2: f64,
        var_ib3_dn3: f64,
        var_ib3_dn4: f64,
        var_ib3_dn5: f64,
        var_ib3_dn6: f64,
        var_ib3_dn7: f64,
        var_ib3_dn8: f64,
        var_ib3_dn9: f64,
        var_iex: f64,
        var_iex_db0: f64,
        var_iex_db1: f64,
        var_iex_dn0: f64,
        var_iex_dn1: f64,
        var_iex_dn10: f64,
        var_iex_dn11: f64,
        var_iex_dn12: f64,
        var_iex_dn2: f64,
        var_iex_dn3: f64,
        var_iex_dn4: f64,
        var_iex_dn5: f64,
        var_iex_dn6: f64,
        var_iex_dn7: f64,
        var_iex_dn8: f64,
        var_iex_dn9: f64,
        var_qb1b2: f64,
        var_qb1b2_db0: f64,
        var_qb1b2_db1: f64,
        var_qb1b2_dn0: f64,
        var_qb1b2_dn1: f64,
        var_qb1b2_dn10: f64,
        var_qb1b2_dn11: f64,
        var_qb1b2_dn12: f64,
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
        var_qbc_dn12: f64,
        var_qbc_dn2: f64,
        var_qbc_dn3: f64,
        var_qbc_dn4: f64,
        var_qbc_dn5: f64,
        var_qbc_dn6: f64,
        var_qbc_dn7: f64,
        var_qbc_dn8: f64,
        var_qbc_dn9: f64,
        var_qepi: f64,
        var_qepi_db0: f64,
        var_qepi_db1: f64,
        var_qepi_dn0: f64,
        var_qepi_dn1: f64,
        var_qepi_dn10: f64,
        var_qepi_dn11: f64,
        var_qepi_dn12: f64,
        var_qepi_dn2: f64,
        var_qepi_dn3: f64,
        var_qepi_dn4: f64,
        var_qepi_dn5: f64,
        var_qepi_dn6: f64,
        var_qepi_dn7: f64,
        var_qepi_dn8: f64,
        var_qepi_dn9: f64,
        var_qtc: f64,
        var_qtc_db0: f64,
        var_qtc_db1: f64,
        var_qtc_dn0: f64,
        var_qtc_dn1: f64,
        var_qtc_dn10: f64,
        var_qtc_dn11: f64,
        var_qtc_dn12: f64,
        var_qtc_dn2: f64,
        var_qtc_dn3: f64,
        var_qtc_dn4: f64,
        var_qtc_dn5: f64,
        var_qtc_dn6: f64,
        var_qtc_dn7: f64,
        var_qtc_dn8: f64,
        var_qtc_dn9: f64,
        var_qts: f64,
        var_qts_db0: f64,
        var_qts_db1: f64,
        var_qts_dn0: f64,
        var_qts_dn1: f64,
        var_qts_dn10: f64,
        var_qts_dn11: f64,
        var_qts_dn12: f64,
        var_qts_dn2: f64,
        var_qts_dn3: f64,
        var_qts_dn4: f64,
        var_qts_dn5: f64,
        var_qts_dn6: f64,
        var_qts_dn7: f64,
        var_qts_dn8: f64,
        var_qts_dn9: f64,
        var_vb1c4: f64,
        var_vb1c4_db0: f64,
        var_vb1c4_db1: f64,
        var_vb1c4_dn0: f64,
        var_vb1c4_dn1: f64,
        var_vb1c4_dn10: f64,
        var_vb1c4_dn11: f64,
        var_vb1c4_dn12: f64,
        var_vb1c4_dn2: f64,
        var_vb1c4_dn3: f64,
        var_vb1c4_dn4: f64,
        var_vb1c4_dn5: f64,
        var_vb1c4_dn6: f64,
        var_vb1c4_dn7: f64,
        var_vb1c4_dn8: f64,
        var_vb1c4_dn9: f64,
        var_vbc: f64,
        var_vbc_db0: f64,
        var_vbc_db1: f64,
        var_vbc_dn0: f64,
        var_vbc_dn1: f64,
        var_vbc_dn10: f64,
        var_vbc_dn11: f64,
        var_vbc_dn12: f64,
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
        var_vbe_dn12: f64,
        var_vbe_dn2: f64,
        var_vbe_dn3: f64,
        var_vbe_dn4: f64,
        var_vbe_dn5: f64,
        var_vbe_dn6: f64,
        var_vbe_dn7: f64,
        var_vbe_dn8: f64,
        var_vbe_dn9: f64,
        var_vcc3: f64,
        var_vcc3_db0: f64,
        var_vcc3_db1: f64,
        var_vcc3_dn0: f64,
        var_vcc3_dn1: f64,
        var_vcc3_dn10: f64,
        var_vcc3_dn11: f64,
        var_vcc3_dn12: f64,
        var_vcc3_dn2: f64,
        var_vcc3_dn3: f64,
        var_vcc3_dn4: f64,
        var_vcc3_dn5: f64,
        var_vcc3_dn6: f64,
        var_vcc3_dn7: f64,
        var_vcc3_dn8: f64,
        var_vcc3_dn9: f64,
        var_xiex: f64,
        var_xiex_db0: f64,
        var_xiex_db1: f64,
        var_xiex_dn0: f64,
        var_xiex_dn1: f64,
        var_xiex_dn10: f64,
        var_xiex_dn11: f64,
        var_xiex_dn12: f64,
        var_xiex_dn2: f64,
        var_xiex_dn3: f64,
        var_xiex_dn4: f64,
        var_xiex_dn5: f64,
        var_xiex_dn6: f64,
        var_xiex_dn7: f64,
        var_xiex_dn8: f64,
        var_xiex_dn9: f64,
        var_xqex: f64,
        var_xqex_db0: f64,
        var_xqex_db1: f64,
        var_xqex_dn0: f64,
        var_xqex_dn1: f64,
        var_xqex_dn10: f64,
        var_xqex_dn11: f64,
        var_xqex_dn12: f64,
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
        var_xqtex_dn12: f64,
        var_xqtex_dn2: f64,
        var_xqtex_dn3: f64,
        var_xqtex_dn4: f64,
        var_xqtex_dn5: f64,
        var_xqtex_dn6: f64,
        var_xqtex_dn7: f64,
        var_xqtex_dn8: f64,
        var_xqtex_dn9: f64,
    ) {
        let eq19_e294: f64 = (var_qtc + var_qbc);
        let eq19_e294_d_n0: f64 = (var_qtc_dn0 + var_qbc_dn0);
        let eq19_e294_d_n1: f64 = (var_qtc_dn1 + var_qbc_dn1);
        let eq19_e294_d_n2: f64 = (var_qtc_dn2 + var_qbc_dn2);
        let eq19_e294_d_n3: f64 = (var_qtc_dn3 + var_qbc_dn3);
        let eq19_e294_d_n4: f64 = (var_qtc_dn4 + var_qbc_dn4);
        let eq19_e294_d_n5: f64 = (var_qtc_dn5 + var_qbc_dn5);
        let eq19_e294_d_n6: f64 = (var_qtc_dn6 + var_qbc_dn6);
        let eq19_e294_d_n7: f64 = (var_qtc_dn7 + var_qbc_dn7);
        let eq19_e294_d_n8: f64 = (var_qtc_dn8 + var_qbc_dn8);
        let eq19_e294_d_n9: f64 = (var_qtc_dn9 + var_qbc_dn9);
        let eq19_e294_d_n10: f64 = (var_qtc_dn10 + var_qbc_dn10);
        let eq19_e294_d_n11: f64 = (var_qtc_dn11 + var_qbc_dn11);
        let eq19_e294_d_n12: f64 = (var_qtc_dn12 + var_qbc_dn12);
        let eq19_e294_d_b0: f64 = (var_qtc_db0 + var_qbc_db0);
        let eq19_e294_d_b1: f64 = (var_qtc_db1 + var_qbc_db1);
        let eq19_e296: f64 = (eq19_e294 + var_qepi);
        let eq19_e296_d_n0: f64 = (eq19_e294_d_n0 + var_qepi_dn0);
        let eq19_e296_d_n1: f64 = (eq19_e294_d_n1 + var_qepi_dn1);
        let eq19_e296_d_n2: f64 = (eq19_e294_d_n2 + var_qepi_dn2);
        let eq19_e296_d_n3: f64 = (eq19_e294_d_n3 + var_qepi_dn3);
        let eq19_e296_d_n4: f64 = (eq19_e294_d_n4 + var_qepi_dn4);
        let eq19_e296_d_n5: f64 = (eq19_e294_d_n5 + var_qepi_dn5);
        let eq19_e296_d_n6: f64 = (eq19_e294_d_n6 + var_qepi_dn6);
        let eq19_e296_d_n7: f64 = (eq19_e294_d_n7 + var_qepi_dn7);
        let eq19_e296_d_n8: f64 = (eq19_e294_d_n8 + var_qepi_dn8);
        let eq19_e296_d_n9: f64 = (eq19_e294_d_n9 + var_qepi_dn9);
        let eq19_e296_d_n10: f64 = (eq19_e294_d_n10 + var_qepi_dn10);
        let eq19_e296_d_n11: f64 = (eq19_e294_d_n11 + var_qepi_dn11);
        let eq19_e296_d_n12: f64 = (eq19_e294_d_n12 + var_qepi_dn12);
        let eq19_e296_d_b0: f64 = (eq19_e294_d_b0 + var_qepi_db0);
        let eq19_e296_d_b1: f64 = (eq19_e294_d_b1 + var_qepi_db1);
        let eq19_e297: f64 = (p.p3 * eq19_e296);
        let eq19_e297_d_n0: f64 = (p.p3 * eq19_e296_d_n0);
        let eq19_e297_d_n1: f64 = (p.p3 * eq19_e296_d_n1);
        let eq19_e297_d_n2: f64 = (p.p3 * eq19_e296_d_n2);
        let eq19_e297_d_n3: f64 = (p.p3 * eq19_e296_d_n3);
        let eq19_e297_d_n4: f64 = (p.p3 * eq19_e296_d_n4);
        let eq19_e297_d_n5: f64 = (p.p3 * eq19_e296_d_n5);
        let eq19_e297_d_n6: f64 = (p.p3 * eq19_e296_d_n6);
        let eq19_e297_d_n7: f64 = (p.p3 * eq19_e296_d_n7);
        let eq19_e297_d_n8: f64 = (p.p3 * eq19_e296_d_n8);
        let eq19_e297_d_n9: f64 = (p.p3 * eq19_e296_d_n9);
        let eq19_e297_d_n10: f64 = (p.p3 * eq19_e296_d_n10);
        let eq19_e297_d_n11: f64 = (p.p3 * eq19_e296_d_n11);
        let eq19_e297_d_n12: f64 = (p.p3 * eq19_e296_d_n12);
        let eq19_e297_d_b0: f64 = (p.p3 * eq19_e296_d_b0);
        let eq19_e297_d_b1: f64 = (p.p3 * eq19_e296_d_b1);
        let eq19_e298: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq19_e297);
        let eq19_e300: f64 = (eq19_e298 * p.p1);
        let eq19_e300_d_n0: f64 = ((eq19_e297_d_n0 * ddt_scale) * p.p1);
        let eq19_e300_d_n1: f64 = ((eq19_e297_d_n1 * ddt_scale) * p.p1);
        let eq19_e300_d_n2: f64 = ((eq19_e297_d_n2 * ddt_scale) * p.p1);
        let eq19_e300_d_n3: f64 = ((eq19_e297_d_n3 * ddt_scale) * p.p1);
        let eq19_e300_d_n4: f64 = ((eq19_e297_d_n4 * ddt_scale) * p.p1);
        let eq19_e300_d_n5: f64 = ((eq19_e297_d_n5 * ddt_scale) * p.p1);
        let eq19_e300_d_n6: f64 = ((eq19_e297_d_n6 * ddt_scale) * p.p1);
        let eq19_e300_d_n7: f64 = ((eq19_e297_d_n7 * ddt_scale) * p.p1);
        let eq19_e300_d_n8: f64 = ((eq19_e297_d_n8 * ddt_scale) * p.p1);
        let eq19_e300_d_n9: f64 = ((eq19_e297_d_n9 * ddt_scale) * p.p1);
        let eq19_e300_d_n10: f64 = ((eq19_e297_d_n10 * ddt_scale) * p.p1);
        let eq19_e300_d_n11: f64 = ((eq19_e297_d_n11 * ddt_scale) * p.p1);
        let eq19_e300_d_n12: f64 = ((eq19_e297_d_n12 * ddt_scale) * p.p1);
        let eq19_e300_d_b0: f64 = ((eq19_e297_d_b0 * ddt_scale) * p.p1);
        let eq19_e300_d_b1: f64 = ((eq19_e297_d_b1 * ddt_scale) * p.p1);
        let eq19_value: f64 = eq19_e300;
        let eq19_node_derivatives: [f64; 13] = [eq19_e300_d_n0, eq19_e300_d_n1, eq19_e300_d_n2, eq19_e300_d_n3, eq19_e300_d_n4, eq19_e300_d_n5, eq19_e300_d_n6, eq19_e300_d_n7, eq19_e300_d_n8, eq19_e300_d_n9, eq19_e300_d_n10, eq19_e300_d_n11, eq19_e300_d_n12];
        let eq19_branch_derivatives: [f64; 2] = [eq19_e300_d_b0, eq19_e300_d_b1];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let eq20_e303: f64 = (p.p3 * var_qts);
        let eq20_e303_d_n0: f64 = (p.p3 * var_qts_dn0);
        let eq20_e303_d_n1: f64 = (p.p3 * var_qts_dn1);
        let eq20_e303_d_n2: f64 = (p.p3 * var_qts_dn2);
        let eq20_e303_d_n3: f64 = (p.p3 * var_qts_dn3);
        let eq20_e303_d_n4: f64 = (p.p3 * var_qts_dn4);
        let eq20_e303_d_n5: f64 = (p.p3 * var_qts_dn5);
        let eq20_e303_d_n6: f64 = (p.p3 * var_qts_dn6);
        let eq20_e303_d_n7: f64 = (p.p3 * var_qts_dn7);
        let eq20_e303_d_n8: f64 = (p.p3 * var_qts_dn8);
        let eq20_e303_d_n9: f64 = (p.p3 * var_qts_dn9);
        let eq20_e303_d_n10: f64 = (p.p3 * var_qts_dn10);
        let eq20_e303_d_n11: f64 = (p.p3 * var_qts_dn11);
        let eq20_e303_d_n12: f64 = (p.p3 * var_qts_dn12);
        let eq20_e303_d_b0: f64 = (p.p3 * var_qts_db0);
        let eq20_e303_d_b1: f64 = (p.p3 * var_qts_db1);
        let eq20_e304: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq20_e303);
        let eq20_e306: f64 = (eq20_e304 * p.p1);
        let eq20_e306_d_n0: f64 = ((eq20_e303_d_n0 * ddt_scale) * p.p1);
        let eq20_e306_d_n1: f64 = ((eq20_e303_d_n1 * ddt_scale) * p.p1);
        let eq20_e306_d_n2: f64 = ((eq20_e303_d_n2 * ddt_scale) * p.p1);
        let eq20_e306_d_n3: f64 = ((eq20_e303_d_n3 * ddt_scale) * p.p1);
        let eq20_e306_d_n4: f64 = ((eq20_e303_d_n4 * ddt_scale) * p.p1);
        let eq20_e306_d_n5: f64 = ((eq20_e303_d_n5 * ddt_scale) * p.p1);
        let eq20_e306_d_n6: f64 = ((eq20_e303_d_n6 * ddt_scale) * p.p1);
        let eq20_e306_d_n7: f64 = ((eq20_e303_d_n7 * ddt_scale) * p.p1);
        let eq20_e306_d_n8: f64 = ((eq20_e303_d_n8 * ddt_scale) * p.p1);
        let eq20_e306_d_n9: f64 = ((eq20_e303_d_n9 * ddt_scale) * p.p1);
        let eq20_e306_d_n10: f64 = ((eq20_e303_d_n10 * ddt_scale) * p.p1);
        let eq20_e306_d_n11: f64 = ((eq20_e303_d_n11 * ddt_scale) * p.p1);
        let eq20_e306_d_n12: f64 = ((eq20_e303_d_n12 * ddt_scale) * p.p1);
        let eq20_e306_d_b0: f64 = ((eq20_e303_d_b0 * ddt_scale) * p.p1);
        let eq20_e306_d_b1: f64 = ((eq20_e303_d_b1 * ddt_scale) * p.p1);
        let eq20_value: f64 = eq20_e306;
        let eq20_node_derivatives: [f64; 13] = [eq20_e306_d_n0, eq20_e306_d_n1, eq20_e306_d_n2, eq20_e306_d_n3, eq20_e306_d_n4, eq20_e306_d_n5, eq20_e306_d_n6, eq20_e306_d_n7, eq20_e306_d_n8, eq20_e306_d_n9, eq20_e306_d_n10, eq20_e306_d_n11, eq20_e306_d_n12];
        let eq20_branch_derivatives: [f64; 2] = [eq20_e306_d_b0, eq20_e306_d_b1];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(8),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let eq21_e309: f64 = (p.p3 * var_qb1b2);
        let eq21_e309_d_n0: f64 = (p.p3 * var_qb1b2_dn0);
        let eq21_e309_d_n1: f64 = (p.p3 * var_qb1b2_dn1);
        let eq21_e309_d_n2: f64 = (p.p3 * var_qb1b2_dn2);
        let eq21_e309_d_n3: f64 = (p.p3 * var_qb1b2_dn3);
        let eq21_e309_d_n4: f64 = (p.p3 * var_qb1b2_dn4);
        let eq21_e309_d_n5: f64 = (p.p3 * var_qb1b2_dn5);
        let eq21_e309_d_n6: f64 = (p.p3 * var_qb1b2_dn6);
        let eq21_e309_d_n7: f64 = (p.p3 * var_qb1b2_dn7);
        let eq21_e309_d_n8: f64 = (p.p3 * var_qb1b2_dn8);
        let eq21_e309_d_n9: f64 = (p.p3 * var_qb1b2_dn9);
        let eq21_e309_d_n10: f64 = (p.p3 * var_qb1b2_dn10);
        let eq21_e309_d_n11: f64 = (p.p3 * var_qb1b2_dn11);
        let eq21_e309_d_n12: f64 = (p.p3 * var_qb1b2_dn12);
        let eq21_e309_d_b0: f64 = (p.p3 * var_qb1b2_db0);
        let eq21_e309_d_b1: f64 = (p.p3 * var_qb1b2_db1);
        let eq21_e310: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq21_e309);
        let eq21_e312: f64 = (eq21_e310 * p.p1);
        let eq21_e312_d_n0: f64 = ((eq21_e309_d_n0 * ddt_scale) * p.p1);
        let eq21_e312_d_n1: f64 = ((eq21_e309_d_n1 * ddt_scale) * p.p1);
        let eq21_e312_d_n2: f64 = ((eq21_e309_d_n2 * ddt_scale) * p.p1);
        let eq21_e312_d_n3: f64 = ((eq21_e309_d_n3 * ddt_scale) * p.p1);
        let eq21_e312_d_n4: f64 = ((eq21_e309_d_n4 * ddt_scale) * p.p1);
        let eq21_e312_d_n5: f64 = ((eq21_e309_d_n5 * ddt_scale) * p.p1);
        let eq21_e312_d_n6: f64 = ((eq21_e309_d_n6 * ddt_scale) * p.p1);
        let eq21_e312_d_n7: f64 = ((eq21_e309_d_n7 * ddt_scale) * p.p1);
        let eq21_e312_d_n8: f64 = ((eq21_e309_d_n8 * ddt_scale) * p.p1);
        let eq21_e312_d_n9: f64 = ((eq21_e309_d_n9 * ddt_scale) * p.p1);
        let eq21_e312_d_n10: f64 = ((eq21_e309_d_n10 * ddt_scale) * p.p1);
        let eq21_e312_d_n11: f64 = ((eq21_e309_d_n11 * ddt_scale) * p.p1);
        let eq21_e312_d_n12: f64 = ((eq21_e309_d_n12 * ddt_scale) * p.p1);
        let eq21_e312_d_b0: f64 = ((eq21_e309_d_b0 * ddt_scale) * p.p1);
        let eq21_e312_d_b1: f64 = ((eq21_e309_d_b1 * ddt_scale) * p.p1);
        let eq21_value: f64 = eq21_e312;
        let eq21_node_derivatives: [f64; 13] = [eq21_e312_d_n0, eq21_e312_d_n1, eq21_e312_d_n2, eq21_e312_d_n3, eq21_e312_d_n4, eq21_e312_d_n5, eq21_e312_d_n6, eq21_e312_d_n7, eq21_e312_d_n8, eq21_e312_d_n9, eq21_e312_d_n10, eq21_e312_d_n11, eq21_e312_d_n12];
        let eq21_branch_derivatives: [f64; 2] = [eq21_e312_d_b0, eq21_e312_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let eq22_e315: f64 = (p.p3 * p.p69);
        let eq22_e317: f64 = (eq22_e315 * var_vbe);
        let eq22_e317_d_n0: f64 = (eq22_e315 * var_vbe_dn0);
        let eq22_e317_d_n1: f64 = (eq22_e315 * var_vbe_dn1);
        let eq22_e317_d_n2: f64 = (eq22_e315 * var_vbe_dn2);
        let eq22_e317_d_n3: f64 = (eq22_e315 * var_vbe_dn3);
        let eq22_e317_d_n4: f64 = (eq22_e315 * var_vbe_dn4);
        let eq22_e317_d_n5: f64 = (eq22_e315 * var_vbe_dn5);
        let eq22_e317_d_n6: f64 = (eq22_e315 * var_vbe_dn6);
        let eq22_e317_d_n7: f64 = (eq22_e315 * var_vbe_dn7);
        let eq22_e317_d_n8: f64 = (eq22_e315 * var_vbe_dn8);
        let eq22_e317_d_n9: f64 = (eq22_e315 * var_vbe_dn9);
        let eq22_e317_d_n10: f64 = (eq22_e315 * var_vbe_dn10);
        let eq22_e317_d_n11: f64 = (eq22_e315 * var_vbe_dn11);
        let eq22_e317_d_n12: f64 = (eq22_e315 * var_vbe_dn12);
        let eq22_e317_d_b0: f64 = (eq22_e315 * var_vbe_db0);
        let eq22_e317_d_b1: f64 = (eq22_e315 * var_vbe_db1);
        let eq22_e318: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq22_e317);
        let eq22_e320: f64 = (eq22_e318 * p.p1);
        let eq22_e320_d_n0: f64 = ((eq22_e317_d_n0 * ddt_scale) * p.p1);
        let eq22_e320_d_n1: f64 = ((eq22_e317_d_n1 * ddt_scale) * p.p1);
        let eq22_e320_d_n2: f64 = ((eq22_e317_d_n2 * ddt_scale) * p.p1);
        let eq22_e320_d_n3: f64 = ((eq22_e317_d_n3 * ddt_scale) * p.p1);
        let eq22_e320_d_n4: f64 = ((eq22_e317_d_n4 * ddt_scale) * p.p1);
        let eq22_e320_d_n5: f64 = ((eq22_e317_d_n5 * ddt_scale) * p.p1);
        let eq22_e320_d_n6: f64 = ((eq22_e317_d_n6 * ddt_scale) * p.p1);
        let eq22_e320_d_n7: f64 = ((eq22_e317_d_n7 * ddt_scale) * p.p1);
        let eq22_e320_d_n8: f64 = ((eq22_e317_d_n8 * ddt_scale) * p.p1);
        let eq22_e320_d_n9: f64 = ((eq22_e317_d_n9 * ddt_scale) * p.p1);
        let eq22_e320_d_n10: f64 = ((eq22_e317_d_n10 * ddt_scale) * p.p1);
        let eq22_e320_d_n11: f64 = ((eq22_e317_d_n11 * ddt_scale) * p.p1);
        let eq22_e320_d_n12: f64 = ((eq22_e317_d_n12 * ddt_scale) * p.p1);
        let eq22_e320_d_b0: f64 = ((eq22_e317_d_b0 * ddt_scale) * p.p1);
        let eq22_e320_d_b1: f64 = ((eq22_e317_d_b1 * ddt_scale) * p.p1);
        let eq22_value: f64 = eq22_e320;
        let eq22_node_derivatives: [f64; 13] = [eq22_e320_d_n0, eq22_e320_d_n1, eq22_e320_d_n2, eq22_e320_d_n3, eq22_e320_d_n4, eq22_e320_d_n5, eq22_e320_d_n6, eq22_e320_d_n7, eq22_e320_d_n8, eq22_e320_d_n9, eq22_e320_d_n10, eq22_e320_d_n11, eq22_e320_d_n12];
        let eq22_branch_derivatives: [f64; 2] = [eq22_e320_d_b0, eq22_e320_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(2),
            multiplicity * (eq22_value),
            &eq22_node_derivatives,
            &eq22_branch_derivatives,
            multiplicity,
        );
        let eq23_e323: f64 = (p.p3 * p.p78);
        let eq23_e325: f64 = (eq23_e323 * var_vbc);
        let eq23_e325_d_n0: f64 = (eq23_e323 * var_vbc_dn0);
        let eq23_e325_d_n1: f64 = (eq23_e323 * var_vbc_dn1);
        let eq23_e325_d_n2: f64 = (eq23_e323 * var_vbc_dn2);
        let eq23_e325_d_n3: f64 = (eq23_e323 * var_vbc_dn3);
        let eq23_e325_d_n4: f64 = (eq23_e323 * var_vbc_dn4);
        let eq23_e325_d_n5: f64 = (eq23_e323 * var_vbc_dn5);
        let eq23_e325_d_n6: f64 = (eq23_e323 * var_vbc_dn6);
        let eq23_e325_d_n7: f64 = (eq23_e323 * var_vbc_dn7);
        let eq23_e325_d_n8: f64 = (eq23_e323 * var_vbc_dn8);
        let eq23_e325_d_n9: f64 = (eq23_e323 * var_vbc_dn9);
        let eq23_e325_d_n10: f64 = (eq23_e323 * var_vbc_dn10);
        let eq23_e325_d_n11: f64 = (eq23_e323 * var_vbc_dn11);
        let eq23_e325_d_n12: f64 = (eq23_e323 * var_vbc_dn12);
        let eq23_e325_d_b0: f64 = (eq23_e323 * var_vbc_db0);
        let eq23_e325_d_b1: f64 = (eq23_e323 * var_vbc_db1);
        let eq23_e326: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq23_e325);
        let eq23_e328: f64 = (eq23_e326 * p.p1);
        let eq23_e328_d_n0: f64 = ((eq23_e325_d_n0 * ddt_scale) * p.p1);
        let eq23_e328_d_n1: f64 = ((eq23_e325_d_n1 * ddt_scale) * p.p1);
        let eq23_e328_d_n2: f64 = ((eq23_e325_d_n2 * ddt_scale) * p.p1);
        let eq23_e328_d_n3: f64 = ((eq23_e325_d_n3 * ddt_scale) * p.p1);
        let eq23_e328_d_n4: f64 = ((eq23_e325_d_n4 * ddt_scale) * p.p1);
        let eq23_e328_d_n5: f64 = ((eq23_e325_d_n5 * ddt_scale) * p.p1);
        let eq23_e328_d_n6: f64 = ((eq23_e325_d_n6 * ddt_scale) * p.p1);
        let eq23_e328_d_n7: f64 = ((eq23_e325_d_n7 * ddt_scale) * p.p1);
        let eq23_e328_d_n8: f64 = ((eq23_e325_d_n8 * ddt_scale) * p.p1);
        let eq23_e328_d_n9: f64 = ((eq23_e325_d_n9 * ddt_scale) * p.p1);
        let eq23_e328_d_n10: f64 = ((eq23_e325_d_n10 * ddt_scale) * p.p1);
        let eq23_e328_d_n11: f64 = ((eq23_e325_d_n11 * ddt_scale) * p.p1);
        let eq23_e328_d_n12: f64 = ((eq23_e325_d_n12 * ddt_scale) * p.p1);
        let eq23_e328_d_b0: f64 = ((eq23_e325_d_b0 * ddt_scale) * p.p1);
        let eq23_e328_d_b1: f64 = ((eq23_e325_d_b1 * ddt_scale) * p.p1);
        let eq23_value: f64 = eq23_e328;
        let eq23_node_derivatives: [f64; 13] = [eq23_e328_d_n0, eq23_e328_d_n1, eq23_e328_d_n2, eq23_e328_d_n3, eq23_e328_d_n4, eq23_e328_d_n5, eq23_e328_d_n6, eq23_e328_d_n7, eq23_e328_d_n8, eq23_e328_d_n9, eq23_e328_d_n10, eq23_e328_d_n11, eq23_e328_d_n12];
        let eq23_branch_derivatives: [f64; 2] = [eq23_e328_d_b0, eq23_e328_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(0),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let eq24_e331: f64 = (p.p3 * var_xiex);
        let eq24_e331_d_n0: f64 = (p.p3 * var_xiex_dn0);
        let eq24_e331_d_n1: f64 = (p.p3 * var_xiex_dn1);
        let eq24_e331_d_n2: f64 = (p.p3 * var_xiex_dn2);
        let eq24_e331_d_n3: f64 = (p.p3 * var_xiex_dn3);
        let eq24_e331_d_n4: f64 = (p.p3 * var_xiex_dn4);
        let eq24_e331_d_n5: f64 = (p.p3 * var_xiex_dn5);
        let eq24_e331_d_n6: f64 = (p.p3 * var_xiex_dn6);
        let eq24_e331_d_n7: f64 = (p.p3 * var_xiex_dn7);
        let eq24_e331_d_n8: f64 = (p.p3 * var_xiex_dn8);
        let eq24_e331_d_n9: f64 = (p.p3 * var_xiex_dn9);
        let eq24_e331_d_n10: f64 = (p.p3 * var_xiex_dn10);
        let eq24_e331_d_n11: f64 = (p.p3 * var_xiex_dn11);
        let eq24_e331_d_n12: f64 = (p.p3 * var_xiex_dn12);
        let eq24_e331_d_b0: f64 = (p.p3 * var_xiex_db0);
        let eq24_e331_d_b1: f64 = (p.p3 * var_xiex_db1);
        let eq24_e333: f64 = (eq24_e331 * p.p1);
        let eq24_e333_d_n0: f64 = (eq24_e331_d_n0 * p.p1);
        let eq24_e333_d_n1: f64 = (eq24_e331_d_n1 * p.p1);
        let eq24_e333_d_n2: f64 = (eq24_e331_d_n2 * p.p1);
        let eq24_e333_d_n3: f64 = (eq24_e331_d_n3 * p.p1);
        let eq24_e333_d_n4: f64 = (eq24_e331_d_n4 * p.p1);
        let eq24_e333_d_n5: f64 = (eq24_e331_d_n5 * p.p1);
        let eq24_e333_d_n6: f64 = (eq24_e331_d_n6 * p.p1);
        let eq24_e333_d_n7: f64 = (eq24_e331_d_n7 * p.p1);
        let eq24_e333_d_n8: f64 = (eq24_e331_d_n8 * p.p1);
        let eq24_e333_d_n9: f64 = (eq24_e331_d_n9 * p.p1);
        let eq24_e333_d_n10: f64 = (eq24_e331_d_n10 * p.p1);
        let eq24_e333_d_n11: f64 = (eq24_e331_d_n11 * p.p1);
        let eq24_e333_d_n12: f64 = (eq24_e331_d_n12 * p.p1);
        let eq24_e333_d_b0: f64 = (eq24_e331_d_b0 * p.p1);
        let eq24_e333_d_b1: f64 = (eq24_e331_d_b1 * p.p1);
        let eq24_value: f64 = eq24_e333;
        let eq24_node_derivatives: [f64; 13] = [eq24_e333_d_n0, eq24_e333_d_n1, eq24_e333_d_n2, eq24_e333_d_n3, eq24_e333_d_n4, eq24_e333_d_n5, eq24_e333_d_n6, eq24_e333_d_n7, eq24_e333_d_n8, eq24_e333_d_n9, eq24_e333_d_n10, eq24_e333_d_n11, eq24_e333_d_n12];
        let eq24_branch_derivatives: [f64; 2] = [eq24_e333_d_b0, eq24_e333_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(10),
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &eq24_branch_derivatives,
            multiplicity,
        );
        let eq25_e336: f64 = (p.p3 * var_vcc3);
        let eq25_e336_d_n0: f64 = (p.p3 * var_vcc3_dn0);
        let eq25_e336_d_n1: f64 = (p.p3 * var_vcc3_dn1);
        let eq25_e336_d_n2: f64 = (p.p3 * var_vcc3_dn2);
        let eq25_e336_d_n3: f64 = (p.p3 * var_vcc3_dn3);
        let eq25_e336_d_n4: f64 = (p.p3 * var_vcc3_dn4);
        let eq25_e336_d_n5: f64 = (p.p3 * var_vcc3_dn5);
        let eq25_e336_d_n6: f64 = (p.p3 * var_vcc3_dn6);
        let eq25_e336_d_n7: f64 = (p.p3 * var_vcc3_dn7);
        let eq25_e336_d_n8: f64 = (p.p3 * var_vcc3_dn8);
        let eq25_e336_d_n9: f64 = (p.p3 * var_vcc3_dn9);
        let eq25_e336_d_n10: f64 = (p.p3 * var_vcc3_dn10);
        let eq25_e336_d_n11: f64 = (p.p3 * var_vcc3_dn11);
        let eq25_e336_d_n12: f64 = (p.p3 * var_vcc3_dn12);
        let eq25_e336_d_b0: f64 = (p.p3 * var_vcc3_db0);
        let eq25_e336_d_b1: f64 = (p.p3 * var_vcc3_db1);
        let eq25_e338: f64 = (eq25_e336 * var_gcc_xx_t);
        let eq25_e338_d_n0: f64 = ((eq25_e336_d_n0 * var_gcc_xx_t) + (eq25_e336 * var_gcc_xx_t_dn0));
        let eq25_e338_d_n1: f64 = ((eq25_e336_d_n1 * var_gcc_xx_t) + (eq25_e336 * var_gcc_xx_t_dn1));
        let eq25_e338_d_n2: f64 = ((eq25_e336_d_n2 * var_gcc_xx_t) + (eq25_e336 * var_gcc_xx_t_dn2));
        let eq25_e338_d_n3: f64 = ((eq25_e336_d_n3 * var_gcc_xx_t) + (eq25_e336 * var_gcc_xx_t_dn3));
        let eq25_e338_d_n4: f64 = ((eq25_e336_d_n4 * var_gcc_xx_t) + (eq25_e336 * var_gcc_xx_t_dn4));
        let eq25_e338_d_n5: f64 = ((eq25_e336_d_n5 * var_gcc_xx_t) + (eq25_e336 * var_gcc_xx_t_dn5));
        let eq25_e338_d_n6: f64 = ((eq25_e336_d_n6 * var_gcc_xx_t) + (eq25_e336 * var_gcc_xx_t_dn6));
        let eq25_e338_d_n7: f64 = ((eq25_e336_d_n7 * var_gcc_xx_t) + (eq25_e336 * var_gcc_xx_t_dn7));
        let eq25_e338_d_n8: f64 = ((eq25_e336_d_n8 * var_gcc_xx_t) + (eq25_e336 * var_gcc_xx_t_dn8));
        let eq25_e338_d_n9: f64 = ((eq25_e336_d_n9 * var_gcc_xx_t) + (eq25_e336 * var_gcc_xx_t_dn9));
        let eq25_e338_d_n10: f64 = ((eq25_e336_d_n10 * var_gcc_xx_t) + (eq25_e336 * var_gcc_xx_t_dn10));
        let eq25_e338_d_n11: f64 = ((eq25_e336_d_n11 * var_gcc_xx_t) + (eq25_e336 * var_gcc_xx_t_dn11));
        let eq25_e338_d_n12: f64 = ((eq25_e336_d_n12 * var_gcc_xx_t) + (eq25_e336 * var_gcc_xx_t_dn12));
        let eq25_e338_d_b0: f64 = ((eq25_e336_d_b0 * var_gcc_xx_t) + (eq25_e336 * var_gcc_xx_t_db0));
        let eq25_e338_d_b1: f64 = ((eq25_e336_d_b1 * var_gcc_xx_t) + (eq25_e336 * var_gcc_xx_t_db1));
        let eq25_e340: f64 = (eq25_e338 * p.p1);
        let eq25_e340_d_n0: f64 = (eq25_e338_d_n0 * p.p1);
        let eq25_e340_d_n1: f64 = (eq25_e338_d_n1 * p.p1);
        let eq25_e340_d_n2: f64 = (eq25_e338_d_n2 * p.p1);
        let eq25_e340_d_n3: f64 = (eq25_e338_d_n3 * p.p1);
        let eq25_e340_d_n4: f64 = (eq25_e338_d_n4 * p.p1);
        let eq25_e340_d_n5: f64 = (eq25_e338_d_n5 * p.p1);
        let eq25_e340_d_n6: f64 = (eq25_e338_d_n6 * p.p1);
        let eq25_e340_d_n7: f64 = (eq25_e338_d_n7 * p.p1);
        let eq25_e340_d_n8: f64 = (eq25_e338_d_n8 * p.p1);
        let eq25_e340_d_n9: f64 = (eq25_e338_d_n9 * p.p1);
        let eq25_e340_d_n10: f64 = (eq25_e338_d_n10 * p.p1);
        let eq25_e340_d_n11: f64 = (eq25_e338_d_n11 * p.p1);
        let eq25_e340_d_n12: f64 = (eq25_e338_d_n12 * p.p1);
        let eq25_e340_d_b0: f64 = (eq25_e338_d_b0 * p.p1);
        let eq25_e340_d_b1: f64 = (eq25_e338_d_b1 * p.p1);
        let eq25_value: f64 = eq25_e340;
        let eq25_node_derivatives: [f64; 13] = [eq25_e340_d_n0, eq25_e340_d_n1, eq25_e340_d_n2, eq25_e340_d_n3, eq25_e340_d_n4, eq25_e340_d_n5, eq25_e340_d_n6, eq25_e340_d_n7, eq25_e340_d_n8, eq25_e340_d_n9, eq25_e340_d_n10, eq25_e340_d_n11, eq25_e340_d_n12];
        let eq25_branch_derivatives: [f64; 2] = [eq25_e340_d_b0, eq25_e340_d_b1];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(10),
            multiplicity * (eq25_value),
            &eq25_node_derivatives,
            &eq25_branch_derivatives,
            multiplicity,
        );
        let eq26_e344: f64 = (var_xqtex + var_xqex);
        let eq26_e344_d_n0: f64 = (var_xqtex_dn0 + var_xqex_dn0);
        let eq26_e344_d_n1: f64 = (var_xqtex_dn1 + var_xqex_dn1);
        let eq26_e344_d_n2: f64 = (var_xqtex_dn2 + var_xqex_dn2);
        let eq26_e344_d_n3: f64 = (var_xqtex_dn3 + var_xqex_dn3);
        let eq26_e344_d_n4: f64 = (var_xqtex_dn4 + var_xqex_dn4);
        let eq26_e344_d_n5: f64 = (var_xqtex_dn5 + var_xqex_dn5);
        let eq26_e344_d_n6: f64 = (var_xqtex_dn6 + var_xqex_dn6);
        let eq26_e344_d_n7: f64 = (var_xqtex_dn7 + var_xqex_dn7);
        let eq26_e344_d_n8: f64 = (var_xqtex_dn8 + var_xqex_dn8);
        let eq26_e344_d_n9: f64 = (var_xqtex_dn9 + var_xqex_dn9);
        let eq26_e344_d_n10: f64 = (var_xqtex_dn10 + var_xqex_dn10);
        let eq26_e344_d_n11: f64 = (var_xqtex_dn11 + var_xqex_dn11);
        let eq26_e344_d_n12: f64 = (var_xqtex_dn12 + var_xqex_dn12);
        let eq26_e344_d_b0: f64 = (var_xqtex_db0 + var_xqex_db0);
        let eq26_e344_d_b1: f64 = (var_xqtex_db1 + var_xqex_db1);
        let eq26_e345: f64 = (p.p3 * eq26_e344);
        let eq26_e345_d_n0: f64 = (p.p3 * eq26_e344_d_n0);
        let eq26_e345_d_n1: f64 = (p.p3 * eq26_e344_d_n1);
        let eq26_e345_d_n2: f64 = (p.p3 * eq26_e344_d_n2);
        let eq26_e345_d_n3: f64 = (p.p3 * eq26_e344_d_n3);
        let eq26_e345_d_n4: f64 = (p.p3 * eq26_e344_d_n4);
        let eq26_e345_d_n5: f64 = (p.p3 * eq26_e344_d_n5);
        let eq26_e345_d_n6: f64 = (p.p3 * eq26_e344_d_n6);
        let eq26_e345_d_n7: f64 = (p.p3 * eq26_e344_d_n7);
        let eq26_e345_d_n8: f64 = (p.p3 * eq26_e344_d_n8);
        let eq26_e345_d_n9: f64 = (p.p3 * eq26_e344_d_n9);
        let eq26_e345_d_n10: f64 = (p.p3 * eq26_e344_d_n10);
        let eq26_e345_d_n11: f64 = (p.p3 * eq26_e344_d_n11);
        let eq26_e345_d_n12: f64 = (p.p3 * eq26_e344_d_n12);
        let eq26_e345_d_b0: f64 = (p.p3 * eq26_e344_d_b0);
        let eq26_e345_d_b1: f64 = (p.p3 * eq26_e344_d_b1);
        let eq26_e346: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq26_e345);
        let eq26_e348: f64 = (eq26_e346 * p.p1);
        let eq26_e348_d_n0: f64 = ((eq26_e345_d_n0 * ddt_scale) * p.p1);
        let eq26_e348_d_n1: f64 = ((eq26_e345_d_n1 * ddt_scale) * p.p1);
        let eq26_e348_d_n2: f64 = ((eq26_e345_d_n2 * ddt_scale) * p.p1);
        let eq26_e348_d_n3: f64 = ((eq26_e345_d_n3 * ddt_scale) * p.p1);
        let eq26_e348_d_n4: f64 = ((eq26_e345_d_n4 * ddt_scale) * p.p1);
        let eq26_e348_d_n5: f64 = ((eq26_e345_d_n5 * ddt_scale) * p.p1);
        let eq26_e348_d_n6: f64 = ((eq26_e345_d_n6 * ddt_scale) * p.p1);
        let eq26_e348_d_n7: f64 = ((eq26_e345_d_n7 * ddt_scale) * p.p1);
        let eq26_e348_d_n8: f64 = ((eq26_e345_d_n8 * ddt_scale) * p.p1);
        let eq26_e348_d_n9: f64 = ((eq26_e345_d_n9 * ddt_scale) * p.p1);
        let eq26_e348_d_n10: f64 = ((eq26_e345_d_n10 * ddt_scale) * p.p1);
        let eq26_e348_d_n11: f64 = ((eq26_e345_d_n11 * ddt_scale) * p.p1);
        let eq26_e348_d_n12: f64 = ((eq26_e345_d_n12 * ddt_scale) * p.p1);
        let eq26_e348_d_b0: f64 = ((eq26_e345_d_b0 * ddt_scale) * p.p1);
        let eq26_e348_d_b1: f64 = ((eq26_e345_d_b1 * ddt_scale) * p.p1);
        let eq26_value: f64 = eq26_e348;
        let eq26_node_derivatives: [f64; 13] = [eq26_e348_d_n0, eq26_e348_d_n1, eq26_e348_d_n2, eq26_e348_d_n3, eq26_e348_d_n4, eq26_e348_d_n5, eq26_e348_d_n6, eq26_e348_d_n7, eq26_e348_d_n8, eq26_e348_d_n9, eq26_e348_d_n10, eq26_e348_d_n11, eq26_e348_d_n12];
        let eq26_branch_derivatives: [f64; 2] = [eq26_e348_d_b0, eq26_e348_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(10),
            multiplicity * (eq26_value),
            &eq26_node_derivatives,
            &eq26_branch_derivatives,
            multiplicity,
        );
        let eq27_e353: f64 = (var_gmin * var_vb1c4);
        let eq27_e353_d_n0: f64 = (var_gmin * var_vb1c4_dn0);
        let eq27_e353_d_n1: f64 = (var_gmin * var_vb1c4_dn1);
        let eq27_e353_d_n2: f64 = (var_gmin * var_vb1c4_dn2);
        let eq27_e353_d_n3: f64 = (var_gmin * var_vb1c4_dn3);
        let eq27_e353_d_n4: f64 = (var_gmin * var_vb1c4_dn4);
        let eq27_e353_d_n5: f64 = (var_gmin * var_vb1c4_dn5);
        let eq27_e353_d_n6: f64 = (var_gmin * var_vb1c4_dn6);
        let eq27_e353_d_n7: f64 = (var_gmin * var_vb1c4_dn7);
        let eq27_e353_d_n8: f64 = (var_gmin * var_vb1c4_dn8);
        let eq27_e353_d_n9: f64 = (var_gmin * var_vb1c4_dn9);
        let eq27_e353_d_n10: f64 = (var_gmin * var_vb1c4_dn10);
        let eq27_e353_d_n11: f64 = (var_gmin * var_vb1c4_dn11);
        let eq27_e353_d_n12: f64 = (var_gmin * var_vb1c4_dn12);
        let eq27_e353_d_b0: f64 = (var_gmin * var_vb1c4_db0);
        let eq27_e353_d_b1: f64 = (var_gmin * var_vb1c4_db1);
        let eq27_e354: f64 = (var_ib3 + eq27_e353);
        let eq27_e354_d_n0: f64 = (var_ib3_dn0 + eq27_e353_d_n0);
        let eq27_e354_d_n1: f64 = (var_ib3_dn1 + eq27_e353_d_n1);
        let eq27_e354_d_n2: f64 = (var_ib3_dn2 + eq27_e353_d_n2);
        let eq27_e354_d_n3: f64 = (var_ib3_dn3 + eq27_e353_d_n3);
        let eq27_e354_d_n4: f64 = (var_ib3_dn4 + eq27_e353_d_n4);
        let eq27_e354_d_n5: f64 = (var_ib3_dn5 + eq27_e353_d_n5);
        let eq27_e354_d_n6: f64 = (var_ib3_dn6 + eq27_e353_d_n6);
        let eq27_e354_d_n7: f64 = (var_ib3_dn7 + eq27_e353_d_n7);
        let eq27_e354_d_n8: f64 = (var_ib3_dn8 + eq27_e353_d_n8);
        let eq27_e354_d_n9: f64 = (var_ib3_dn9 + eq27_e353_d_n9);
        let eq27_e354_d_n10: f64 = (var_ib3_dn10 + eq27_e353_d_n10);
        let eq27_e354_d_n11: f64 = (var_ib3_dn11 + eq27_e353_d_n11);
        let eq27_e354_d_n12: f64 = (var_ib3_dn12 + eq27_e353_d_n12);
        let eq27_e354_d_b0: f64 = (var_ib3_db0 + eq27_e353_d_b0);
        let eq27_e354_d_b1: f64 = (var_ib3_db1 + eq27_e353_d_b1);
        let eq27_e356: f64 = (eq27_e354 + var_iex);
        let eq27_e356_d_n0: f64 = (eq27_e354_d_n0 + var_iex_dn0);
        let eq27_e356_d_n1: f64 = (eq27_e354_d_n1 + var_iex_dn1);
        let eq27_e356_d_n2: f64 = (eq27_e354_d_n2 + var_iex_dn2);
        let eq27_e356_d_n3: f64 = (eq27_e354_d_n3 + var_iex_dn3);
        let eq27_e356_d_n4: f64 = (eq27_e354_d_n4 + var_iex_dn4);
        let eq27_e356_d_n5: f64 = (eq27_e354_d_n5 + var_iex_dn5);
        let eq27_e356_d_n6: f64 = (eq27_e354_d_n6 + var_iex_dn6);
        let eq27_e356_d_n7: f64 = (eq27_e354_d_n7 + var_iex_dn7);
        let eq27_e356_d_n8: f64 = (eq27_e354_d_n8 + var_iex_dn8);
        let eq27_e356_d_n9: f64 = (eq27_e354_d_n9 + var_iex_dn9);
        let eq27_e356_d_n10: f64 = (eq27_e354_d_n10 + var_iex_dn10);
        let eq27_e356_d_n11: f64 = (eq27_e354_d_n11 + var_iex_dn11);
        let eq27_e356_d_n12: f64 = (eq27_e354_d_n12 + var_iex_dn12);
        let eq27_e356_d_b0: f64 = (eq27_e354_d_b0 + var_iex_db0);
        let eq27_e356_d_b1: f64 = (eq27_e354_d_b1 + var_iex_db1);
        let eq27_e357: f64 = (p.p3 * eq27_e356);
        let eq27_e357_d_n0: f64 = (p.p3 * eq27_e356_d_n0);
        let eq27_e357_d_n1: f64 = (p.p3 * eq27_e356_d_n1);
        let eq27_e357_d_n2: f64 = (p.p3 * eq27_e356_d_n2);
        let eq27_e357_d_n3: f64 = (p.p3 * eq27_e356_d_n3);
        let eq27_e357_d_n4: f64 = (p.p3 * eq27_e356_d_n4);
        let eq27_e357_d_n5: f64 = (p.p3 * eq27_e356_d_n5);
        let eq27_e357_d_n6: f64 = (p.p3 * eq27_e356_d_n6);
        let eq27_e357_d_n7: f64 = (p.p3 * eq27_e356_d_n7);
        let eq27_e357_d_n8: f64 = (p.p3 * eq27_e356_d_n8);
        let eq27_e357_d_n9: f64 = (p.p3 * eq27_e356_d_n9);
        let eq27_e357_d_n10: f64 = (p.p3 * eq27_e356_d_n10);
        let eq27_e357_d_n11: f64 = (p.p3 * eq27_e356_d_n11);
        let eq27_e357_d_n12: f64 = (p.p3 * eq27_e356_d_n12);
        let eq27_e357_d_b0: f64 = (p.p3 * eq27_e356_d_b0);
        let eq27_e357_d_b1: f64 = (p.p3 * eq27_e356_d_b1);
        let eq27_e359: f64 = (eq27_e357 * p.p1);
        let eq27_e359_d_n0: f64 = (eq27_e357_d_n0 * p.p1);
        let eq27_e359_d_n1: f64 = (eq27_e357_d_n1 * p.p1);
        let eq27_e359_d_n2: f64 = (eq27_e357_d_n2 * p.p1);
        let eq27_e359_d_n3: f64 = (eq27_e357_d_n3 * p.p1);
        let eq27_e359_d_n4: f64 = (eq27_e357_d_n4 * p.p1);
        let eq27_e359_d_n5: f64 = (eq27_e357_d_n5 * p.p1);
        let eq27_e359_d_n6: f64 = (eq27_e357_d_n6 * p.p1);
        let eq27_e359_d_n7: f64 = (eq27_e357_d_n7 * p.p1);
        let eq27_e359_d_n8: f64 = (eq27_e357_d_n8 * p.p1);
        let eq27_e359_d_n9: f64 = (eq27_e357_d_n9 * p.p1);
        let eq27_e359_d_n10: f64 = (eq27_e357_d_n10 * p.p1);
        let eq27_e359_d_n11: f64 = (eq27_e357_d_n11 * p.p1);
        let eq27_e359_d_n12: f64 = (eq27_e357_d_n12 * p.p1);
        let eq27_e359_d_b0: f64 = (eq27_e357_d_b0 * p.p1);
        let eq27_e359_d_b1: f64 = (eq27_e357_d_b1 * p.p1);
        let eq27_value: f64 = eq27_e359;
        let eq27_node_derivatives: [f64; 13] = [eq27_e359_d_n0, eq27_e359_d_n1, eq27_e359_d_n2, eq27_e359_d_n3, eq27_e359_d_n4, eq27_e359_d_n5, eq27_e359_d_n6, eq27_e359_d_n7, eq27_e359_d_n8, eq27_e359_d_n9, eq27_e359_d_n10, eq27_e359_d_n11, eq27_e359_d_n12];
        let eq27_branch_derivatives: [f64; 2] = [eq27_e359_d_b0, eq27_e359_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(11),
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
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
        var_gcc_ex_t: f64,
        var_gcc_ex_t_db0: f64,
        var_gcc_ex_t_db1: f64,
        var_gcc_ex_t_dn0: f64,
        var_gcc_ex_t_dn1: f64,
        var_gcc_ex_t_dn10: f64,
        var_gcc_ex_t_dn11: f64,
        var_gcc_ex_t_dn12: f64,
        var_gcc_ex_t_dn2: f64,
        var_gcc_ex_t_dn3: f64,
        var_gcc_ex_t_dn4: f64,
        var_gcc_ex_t_dn5: f64,
        var_gcc_ex_t_dn6: f64,
        var_gcc_ex_t_dn7: f64,
        var_gcc_ex_t_dn8: f64,
        var_gcc_ex_t_dn9: f64,
        var_gcc_in_t: f64,
        var_gcc_in_t_db0: f64,
        var_gcc_in_t_db1: f64,
        var_gcc_in_t_dn0: f64,
        var_gcc_in_t_dn1: f64,
        var_gcc_in_t_dn10: f64,
        var_gcc_in_t_dn11: f64,
        var_gcc_in_t_dn12: f64,
        var_gcc_in_t_dn2: f64,
        var_gcc_in_t_dn3: f64,
        var_gcc_in_t_dn4: f64,
        var_gcc_in_t_dn5: f64,
        var_gcc_in_t_dn6: f64,
        var_gcc_in_t_dn7: f64,
        var_gcc_in_t_dn8: f64,
        var_gcc_in_t_dn9: f64,
        var_gem_n: f64,
        var_gem_n_db0: f64,
        var_gem_n_db1: f64,
        var_gem_n_dn0: f64,
        var_gem_n_dn1: f64,
        var_gem_n_dn10: f64,
        var_gem_n_dn11: f64,
        var_gem_n_dn12: f64,
        var_gem_n_dn2: f64,
        var_gem_n_dn3: f64,
        var_gem_n_dn4: f64,
        var_gem_n_dn5: f64,
        var_gem_n_dn6: f64,
        var_gem_n_dn7: f64,
        var_gem_n_dn8: f64,
        var_gem_n_dn9: f64,
        var_guard129: f64,
        var_guard130: f64,
        var_qex: f64,
        var_qex_db0: f64,
        var_qex_db1: f64,
        var_qex_dn0: f64,
        var_qex_dn1: f64,
        var_qex_dn10: f64,
        var_qex_dn11: f64,
        var_qex_dn12: f64,
        var_qex_dn2: f64,
        var_qex_dn3: f64,
        var_qex_dn4: f64,
        var_qex_dn5: f64,
        var_qex_dn6: f64,
        var_qex_dn7: f64,
        var_qex_dn8: f64,
        var_qex_dn9: f64,
        var_qtex: f64,
        var_qtex_db0: f64,
        var_qtex_db1: f64,
        var_qtex_dn0: f64,
        var_qtex_dn1: f64,
        var_qtex_dn10: f64,
        var_qtex_dn11: f64,
        var_qtex_dn12: f64,
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
        var_taun_dn12: f64,
        var_taun_dn2: f64,
        var_taun_dn3: f64,
        var_taun_dn4: f64,
        var_taun_dn5: f64,
        var_taun_dn6: f64,
        var_taun_dn7: f64,
        var_taun_dn8: f64,
        var_taun_dn9: f64,
        var_vc3c4: f64,
        var_vc3c4_db0: f64,
        var_vc3c4_db1: f64,
        var_vc3c4_dn0: f64,
        var_vc3c4_dn1: f64,
        var_vc3c4_dn10: f64,
        var_vc3c4_dn11: f64,
        var_vc3c4_dn12: f64,
        var_vc3c4_dn2: f64,
        var_vc3c4_dn3: f64,
        var_vc3c4_dn4: f64,
        var_vc3c4_dn5: f64,
        var_vc3c4_dn6: f64,
        var_vc3c4_dn7: f64,
        var_vc3c4_dn8: f64,
        var_vc3c4_dn9: f64,
        var_vc4c1: f64,
        var_vc4c1_db0: f64,
        var_vc4c1_db1: f64,
        var_vc4c1_dn0: f64,
        var_vc4c1_dn1: f64,
        var_vc4c1_dn10: f64,
        var_vc4c1_dn11: f64,
        var_vc4c1_dn12: f64,
        var_vc4c1_dn2: f64,
        var_vc4c1_dn3: f64,
        var_vc4c1_dn4: f64,
        var_vc4c1_dn5: f64,
        var_vc4c1_dn6: f64,
        var_vc4c1_dn7: f64,
        var_vc4c1_dn8: f64,
        var_vc4c1_dn9: f64,
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let eq28_e363: f64 = (var_qtex + var_qex);
        let eq28_e363_d_n0: f64 = (var_qtex_dn0 + var_qex_dn0);
        let eq28_e363_d_n1: f64 = (var_qtex_dn1 + var_qex_dn1);
        let eq28_e363_d_n2: f64 = (var_qtex_dn2 + var_qex_dn2);
        let eq28_e363_d_n3: f64 = (var_qtex_dn3 + var_qex_dn3);
        let eq28_e363_d_n4: f64 = (var_qtex_dn4 + var_qex_dn4);
        let eq28_e363_d_n5: f64 = (var_qtex_dn5 + var_qex_dn5);
        let eq28_e363_d_n6: f64 = (var_qtex_dn6 + var_qex_dn6);
        let eq28_e363_d_n7: f64 = (var_qtex_dn7 + var_qex_dn7);
        let eq28_e363_d_n8: f64 = (var_qtex_dn8 + var_qex_dn8);
        let eq28_e363_d_n9: f64 = (var_qtex_dn9 + var_qex_dn9);
        let eq28_e363_d_n10: f64 = (var_qtex_dn10 + var_qex_dn10);
        let eq28_e363_d_n11: f64 = (var_qtex_dn11 + var_qex_dn11);
        let eq28_e363_d_n12: f64 = (var_qtex_dn12 + var_qex_dn12);
        let eq28_e363_d_b0: f64 = (var_qtex_db0 + var_qex_db0);
        let eq28_e363_d_b1: f64 = (var_qtex_db1 + var_qex_db1);
        let eq28_e364: f64 = (p.p3 * eq28_e363);
        let eq28_e364_d_n0: f64 = (p.p3 * eq28_e363_d_n0);
        let eq28_e364_d_n1: f64 = (p.p3 * eq28_e363_d_n1);
        let eq28_e364_d_n2: f64 = (p.p3 * eq28_e363_d_n2);
        let eq28_e364_d_n3: f64 = (p.p3 * eq28_e363_d_n3);
        let eq28_e364_d_n4: f64 = (p.p3 * eq28_e363_d_n4);
        let eq28_e364_d_n5: f64 = (p.p3 * eq28_e363_d_n5);
        let eq28_e364_d_n6: f64 = (p.p3 * eq28_e363_d_n6);
        let eq28_e364_d_n7: f64 = (p.p3 * eq28_e363_d_n7);
        let eq28_e364_d_n8: f64 = (p.p3 * eq28_e363_d_n8);
        let eq28_e364_d_n9: f64 = (p.p3 * eq28_e363_d_n9);
        let eq28_e364_d_n10: f64 = (p.p3 * eq28_e363_d_n10);
        let eq28_e364_d_n11: f64 = (p.p3 * eq28_e363_d_n11);
        let eq28_e364_d_n12: f64 = (p.p3 * eq28_e363_d_n12);
        let eq28_e364_d_b0: f64 = (p.p3 * eq28_e363_d_b0);
        let eq28_e364_d_b1: f64 = (p.p3 * eq28_e363_d_b1);
        let eq28_e365: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq28_e364);
        let eq28_e367: f64 = (eq28_e365 * p.p1);
        let eq28_e367_d_n0: f64 = ((eq28_e364_d_n0 * ddt_scale) * p.p1);
        let eq28_e367_d_n1: f64 = ((eq28_e364_d_n1 * ddt_scale) * p.p1);
        let eq28_e367_d_n2: f64 = ((eq28_e364_d_n2 * ddt_scale) * p.p1);
        let eq28_e367_d_n3: f64 = ((eq28_e364_d_n3 * ddt_scale) * p.p1);
        let eq28_e367_d_n4: f64 = ((eq28_e364_d_n4 * ddt_scale) * p.p1);
        let eq28_e367_d_n5: f64 = ((eq28_e364_d_n5 * ddt_scale) * p.p1);
        let eq28_e367_d_n6: f64 = ((eq28_e364_d_n6 * ddt_scale) * p.p1);
        let eq28_e367_d_n7: f64 = ((eq28_e364_d_n7 * ddt_scale) * p.p1);
        let eq28_e367_d_n8: f64 = ((eq28_e364_d_n8 * ddt_scale) * p.p1);
        let eq28_e367_d_n9: f64 = ((eq28_e364_d_n9 * ddt_scale) * p.p1);
        let eq28_e367_d_n10: f64 = ((eq28_e364_d_n10 * ddt_scale) * p.p1);
        let eq28_e367_d_n11: f64 = ((eq28_e364_d_n11 * ddt_scale) * p.p1);
        let eq28_e367_d_n12: f64 = ((eq28_e364_d_n12 * ddt_scale) * p.p1);
        let eq28_e367_d_b0: f64 = ((eq28_e364_d_b0 * ddt_scale) * p.p1);
        let eq28_e367_d_b1: f64 = ((eq28_e364_d_b1 * ddt_scale) * p.p1);
        let eq28_value: f64 = eq28_e367;
        let eq28_node_derivatives: [f64; 13] = [eq28_e367_d_n0, eq28_e367_d_n1, eq28_e367_d_n2, eq28_e367_d_n3, eq28_e367_d_n4, eq28_e367_d_n5, eq28_e367_d_n6, eq28_e367_d_n7, eq28_e367_d_n8, eq28_e367_d_n9, eq28_e367_d_n10, eq28_e367_d_n11, eq28_e367_d_n12];
        let eq28_branch_derivatives: [f64; 2] = [eq28_e367_d_b0, eq28_e367_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(11),
            multiplicity * (eq28_value),
            &eq28_node_derivatives,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let (eq29_e377, eq29_e377_d_n0, eq29_e377_d_n1, eq29_e377_d_n2, eq29_e377_d_n3, eq29_e377_d_n4, eq29_e377_d_n5, eq29_e377_d_n6, eq29_e377_d_n7, eq29_e377_d_n8, eq29_e377_d_n9, eq29_e377_d_n10, eq29_e377_d_n11, eq29_e377_d_n12, eq29_e377_d_b0, eq29_e377_d_b1,) = {
    if (var_guard129 != 0.0) {
        let eq29_e371: f64 = (p.p3 * var_vc3c4);
        let eq29_e371_d_n0: f64 = (p.p3 * var_vc3c4_dn0);
        let eq29_e371_d_n1: f64 = (p.p3 * var_vc3c4_dn1);
        let eq29_e371_d_n2: f64 = (p.p3 * var_vc3c4_dn2);
        let eq29_e371_d_n3: f64 = (p.p3 * var_vc3c4_dn3);
        let eq29_e371_d_n4: f64 = (p.p3 * var_vc3c4_dn4);
        let eq29_e371_d_n5: f64 = (p.p3 * var_vc3c4_dn5);
        let eq29_e371_d_n6: f64 = (p.p3 * var_vc3c4_dn6);
        let eq29_e371_d_n7: f64 = (p.p3 * var_vc3c4_dn7);
        let eq29_e371_d_n8: f64 = (p.p3 * var_vc3c4_dn8);
        let eq29_e371_d_n9: f64 = (p.p3 * var_vc3c4_dn9);
        let eq29_e371_d_n10: f64 = (p.p3 * var_vc3c4_dn10);
        let eq29_e371_d_n11: f64 = (p.p3 * var_vc3c4_dn11);
        let eq29_e371_d_n12: f64 = (p.p3 * var_vc3c4_dn12);
        let eq29_e371_d_b0: f64 = (p.p3 * var_vc3c4_db0);
        let eq29_e371_d_b1: f64 = (p.p3 * var_vc3c4_db1);
        let eq29_e373: f64 = (eq29_e371 * var_gcc_ex_t);
        let eq29_e373_d_n0: f64 = ((eq29_e371_d_n0 * var_gcc_ex_t) + (eq29_e371 * var_gcc_ex_t_dn0));
        let eq29_e373_d_n1: f64 = ((eq29_e371_d_n1 * var_gcc_ex_t) + (eq29_e371 * var_gcc_ex_t_dn1));
        let eq29_e373_d_n2: f64 = ((eq29_e371_d_n2 * var_gcc_ex_t) + (eq29_e371 * var_gcc_ex_t_dn2));
        let eq29_e373_d_n3: f64 = ((eq29_e371_d_n3 * var_gcc_ex_t) + (eq29_e371 * var_gcc_ex_t_dn3));
        let eq29_e373_d_n4: f64 = ((eq29_e371_d_n4 * var_gcc_ex_t) + (eq29_e371 * var_gcc_ex_t_dn4));
        let eq29_e373_d_n5: f64 = ((eq29_e371_d_n5 * var_gcc_ex_t) + (eq29_e371 * var_gcc_ex_t_dn5));
        let eq29_e373_d_n6: f64 = ((eq29_e371_d_n6 * var_gcc_ex_t) + (eq29_e371 * var_gcc_ex_t_dn6));
        let eq29_e373_d_n7: f64 = ((eq29_e371_d_n7 * var_gcc_ex_t) + (eq29_e371 * var_gcc_ex_t_dn7));
        let eq29_e373_d_n8: f64 = ((eq29_e371_d_n8 * var_gcc_ex_t) + (eq29_e371 * var_gcc_ex_t_dn8));
        let eq29_e373_d_n9: f64 = ((eq29_e371_d_n9 * var_gcc_ex_t) + (eq29_e371 * var_gcc_ex_t_dn9));
        let eq29_e373_d_n10: f64 = ((eq29_e371_d_n10 * var_gcc_ex_t) + (eq29_e371 * var_gcc_ex_t_dn10));
        let eq29_e373_d_n11: f64 = ((eq29_e371_d_n11 * var_gcc_ex_t) + (eq29_e371 * var_gcc_ex_t_dn11));
        let eq29_e373_d_n12: f64 = ((eq29_e371_d_n12 * var_gcc_ex_t) + (eq29_e371 * var_gcc_ex_t_dn12));
        let eq29_e373_d_b0: f64 = ((eq29_e371_d_b0 * var_gcc_ex_t) + (eq29_e371 * var_gcc_ex_t_db0));
        let eq29_e373_d_b1: f64 = ((eq29_e371_d_b1 * var_gcc_ex_t) + (eq29_e371 * var_gcc_ex_t_db1));
        let eq29_e375: f64 = (eq29_e373 * p.p1);
        let eq29_e375_d_n0: f64 = (eq29_e373_d_n0 * p.p1);
        let eq29_e375_d_n1: f64 = (eq29_e373_d_n1 * p.p1);
        let eq29_e375_d_n2: f64 = (eq29_e373_d_n2 * p.p1);
        let eq29_e375_d_n3: f64 = (eq29_e373_d_n3 * p.p1);
        let eq29_e375_d_n4: f64 = (eq29_e373_d_n4 * p.p1);
        let eq29_e375_d_n5: f64 = (eq29_e373_d_n5 * p.p1);
        let eq29_e375_d_n6: f64 = (eq29_e373_d_n6 * p.p1);
        let eq29_e375_d_n7: f64 = (eq29_e373_d_n7 * p.p1);
        let eq29_e375_d_n8: f64 = (eq29_e373_d_n8 * p.p1);
        let eq29_e375_d_n9: f64 = (eq29_e373_d_n9 * p.p1);
        let eq29_e375_d_n10: f64 = (eq29_e373_d_n10 * p.p1);
        let eq29_e375_d_n11: f64 = (eq29_e373_d_n11 * p.p1);
        let eq29_e375_d_n12: f64 = (eq29_e373_d_n12 * p.p1);
        let eq29_e375_d_b0: f64 = (eq29_e373_d_b0 * p.p1);
        let eq29_e375_d_b1: f64 = (eq29_e373_d_b1 * p.p1);
        (eq29_e375, eq29_e375_d_n0, eq29_e375_d_n1, eq29_e375_d_n2, eq29_e375_d_n3, eq29_e375_d_n4, eq29_e375_d_n5, eq29_e375_d_n6, eq29_e375_d_n7, eq29_e375_d_n8, eq29_e375_d_n9, eq29_e375_d_n10, eq29_e375_d_n11, eq29_e375_d_n12, eq29_e375_d_b0, eq29_e375_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e377;
        let eq29_node_derivatives: [f64; 13] = [eq29_e377_d_n0, eq29_e377_d_n1, eq29_e377_d_n2, eq29_e377_d_n3, eq29_e377_d_n4, eq29_e377_d_n5, eq29_e377_d_n6, eq29_e377_d_n7, eq29_e377_d_n8, eq29_e377_d_n9, eq29_e377_d_n10, eq29_e377_d_n11, eq29_e377_d_n12];
        let eq29_branch_derivatives: [f64; 2] = [eq29_e377_d_b0, eq29_e377_d_b1];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(11),
            multiplicity * (eq29_value),
            &eq29_node_derivatives,
            &eq29_branch_derivatives,
            multiplicity,
        );
        let (eq31_e392, eq31_e392_d_n0, eq31_e392_d_n1, eq31_e392_d_n2, eq31_e392_d_n3, eq31_e392_d_n4, eq31_e392_d_n5, eq31_e392_d_n6, eq31_e392_d_n7, eq31_e392_d_n8, eq31_e392_d_n9, eq31_e392_d_n10, eq31_e392_d_n11, eq31_e392_d_n12, eq31_e392_d_b0, eq31_e392_d_b1,) = {
    if (var_guard130 != 0.0) {
        let eq31_e386: f64 = (p.p3 * var_vc4c1);
        let eq31_e386_d_n0: f64 = (p.p3 * var_vc4c1_dn0);
        let eq31_e386_d_n1: f64 = (p.p3 * var_vc4c1_dn1);
        let eq31_e386_d_n2: f64 = (p.p3 * var_vc4c1_dn2);
        let eq31_e386_d_n3: f64 = (p.p3 * var_vc4c1_dn3);
        let eq31_e386_d_n4: f64 = (p.p3 * var_vc4c1_dn4);
        let eq31_e386_d_n5: f64 = (p.p3 * var_vc4c1_dn5);
        let eq31_e386_d_n6: f64 = (p.p3 * var_vc4c1_dn6);
        let eq31_e386_d_n7: f64 = (p.p3 * var_vc4c1_dn7);
        let eq31_e386_d_n8: f64 = (p.p3 * var_vc4c1_dn8);
        let eq31_e386_d_n9: f64 = (p.p3 * var_vc4c1_dn9);
        let eq31_e386_d_n10: f64 = (p.p3 * var_vc4c1_dn10);
        let eq31_e386_d_n11: f64 = (p.p3 * var_vc4c1_dn11);
        let eq31_e386_d_n12: f64 = (p.p3 * var_vc4c1_dn12);
        let eq31_e386_d_b0: f64 = (p.p3 * var_vc4c1_db0);
        let eq31_e386_d_b1: f64 = (p.p3 * var_vc4c1_db1);
        let eq31_e388: f64 = (eq31_e386 * var_gcc_in_t);
        let eq31_e388_d_n0: f64 = ((eq31_e386_d_n0 * var_gcc_in_t) + (eq31_e386 * var_gcc_in_t_dn0));
        let eq31_e388_d_n1: f64 = ((eq31_e386_d_n1 * var_gcc_in_t) + (eq31_e386 * var_gcc_in_t_dn1));
        let eq31_e388_d_n2: f64 = ((eq31_e386_d_n2 * var_gcc_in_t) + (eq31_e386 * var_gcc_in_t_dn2));
        let eq31_e388_d_n3: f64 = ((eq31_e386_d_n3 * var_gcc_in_t) + (eq31_e386 * var_gcc_in_t_dn3));
        let eq31_e388_d_n4: f64 = ((eq31_e386_d_n4 * var_gcc_in_t) + (eq31_e386 * var_gcc_in_t_dn4));
        let eq31_e388_d_n5: f64 = ((eq31_e386_d_n5 * var_gcc_in_t) + (eq31_e386 * var_gcc_in_t_dn5));
        let eq31_e388_d_n6: f64 = ((eq31_e386_d_n6 * var_gcc_in_t) + (eq31_e386 * var_gcc_in_t_dn6));
        let eq31_e388_d_n7: f64 = ((eq31_e386_d_n7 * var_gcc_in_t) + (eq31_e386 * var_gcc_in_t_dn7));
        let eq31_e388_d_n8: f64 = ((eq31_e386_d_n8 * var_gcc_in_t) + (eq31_e386 * var_gcc_in_t_dn8));
        let eq31_e388_d_n9: f64 = ((eq31_e386_d_n9 * var_gcc_in_t) + (eq31_e386 * var_gcc_in_t_dn9));
        let eq31_e388_d_n10: f64 = ((eq31_e386_d_n10 * var_gcc_in_t) + (eq31_e386 * var_gcc_in_t_dn10));
        let eq31_e388_d_n11: f64 = ((eq31_e386_d_n11 * var_gcc_in_t) + (eq31_e386 * var_gcc_in_t_dn11));
        let eq31_e388_d_n12: f64 = ((eq31_e386_d_n12 * var_gcc_in_t) + (eq31_e386 * var_gcc_in_t_dn12));
        let eq31_e388_d_b0: f64 = ((eq31_e386_d_b0 * var_gcc_in_t) + (eq31_e386 * var_gcc_in_t_db0));
        let eq31_e388_d_b1: f64 = ((eq31_e386_d_b1 * var_gcc_in_t) + (eq31_e386 * var_gcc_in_t_db1));
        let eq31_e390: f64 = (eq31_e388 * p.p1);
        let eq31_e390_d_n0: f64 = (eq31_e388_d_n0 * p.p1);
        let eq31_e390_d_n1: f64 = (eq31_e388_d_n1 * p.p1);
        let eq31_e390_d_n2: f64 = (eq31_e388_d_n2 * p.p1);
        let eq31_e390_d_n3: f64 = (eq31_e388_d_n3 * p.p1);
        let eq31_e390_d_n4: f64 = (eq31_e388_d_n4 * p.p1);
        let eq31_e390_d_n5: f64 = (eq31_e388_d_n5 * p.p1);
        let eq31_e390_d_n6: f64 = (eq31_e388_d_n6 * p.p1);
        let eq31_e390_d_n7: f64 = (eq31_e388_d_n7 * p.p1);
        let eq31_e390_d_n8: f64 = (eq31_e388_d_n8 * p.p1);
        let eq31_e390_d_n9: f64 = (eq31_e388_d_n9 * p.p1);
        let eq31_e390_d_n10: f64 = (eq31_e388_d_n10 * p.p1);
        let eq31_e390_d_n11: f64 = (eq31_e388_d_n11 * p.p1);
        let eq31_e390_d_n12: f64 = (eq31_e388_d_n12 * p.p1);
        let eq31_e390_d_b0: f64 = (eq31_e388_d_b0 * p.p1);
        let eq31_e390_d_b1: f64 = (eq31_e388_d_b1 * p.p1);
        (eq31_e390, eq31_e390_d_n0, eq31_e390_d_n1, eq31_e390_d_n2, eq31_e390_d_n3, eq31_e390_d_n4, eq31_e390_d_n5, eq31_e390_d_n6, eq31_e390_d_n7, eq31_e390_d_n8, eq31_e390_d_n9, eq31_e390_d_n10, eq31_e390_d_n11, eq31_e390_d_n12, eq31_e390_d_b0, eq31_e390_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e392;
        let eq31_node_derivatives: [f64; 13] = [eq31_e392_d_n0, eq31_e392_d_n1, eq31_e392_d_n2, eq31_e392_d_n3, eq31_e392_d_n4, eq31_e392_d_n5, eq31_e392_d_n6, eq31_e392_d_n7, eq31_e392_d_n8, eq31_e392_d_n9, eq31_e392_d_n10, eq31_e392_d_n11, eq31_e392_d_n12];
        let eq31_branch_derivatives: [f64; 2] = [eq31_e392_d_b0, eq31_e392_d_b1];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(8),
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let eq35_e406: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, (nv12 - 0.0));
        let eq35_e407: f64 = (var_taun * eq35_e406);
        let eq35_e407_d_n0: f64 = (var_taun_dn0 * eq35_e406);
        let eq35_e407_d_n1: f64 = (var_taun_dn1 * eq35_e406);
        let eq35_e407_d_n2: f64 = (var_taun_dn2 * eq35_e406);
        let eq35_e407_d_n3: f64 = (var_taun_dn3 * eq35_e406);
        let eq35_e407_d_n4: f64 = (var_taun_dn4 * eq35_e406);
        let eq35_e407_d_n5: f64 = (var_taun_dn5 * eq35_e406);
        let eq35_e407_d_n6: f64 = (var_taun_dn6 * eq35_e406);
        let eq35_e407_d_n7: f64 = (var_taun_dn7 * eq35_e406);
        let eq35_e407_d_n8: f64 = (var_taun_dn8 * eq35_e406);
        let eq35_e407_d_n9: f64 = (var_taun_dn9 * eq35_e406);
        let eq35_e407_d_n10: f64 = (var_taun_dn10 * eq35_e406);
        let eq35_e407_d_n11: f64 = (var_taun_dn11 * eq35_e406);
        let eq35_e407_d_n12: f64 = ((var_taun_dn12 * eq35_e406) + (var_taun * ddt_scale));
        let eq35_e407_d_b0: f64 = (var_taun_db0 * eq35_e406);
        let eq35_e407_d_b1: f64 = (var_taun_db1 * eq35_e406);
        let eq35_value: f64 = eq35_e407;
        let eq35_node_derivatives: [f64; 13] = [eq35_e407_d_n0, eq35_e407_d_n1, eq35_e407_d_n2, eq35_e407_d_n3, eq35_e407_d_n4, eq35_e407_d_n5, eq35_e407_d_n6, eq35_e407_d_n7, eq35_e407_d_n8, eq35_e407_d_n9, eq35_e407_d_n10, eq35_e407_d_n11, eq35_e407_d_n12];
        let eq35_branch_derivatives: [f64; 2] = [eq35_e407_d_b0, eq35_e407_d_b1];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq35_value),
            &eq35_node_derivatives,
            &eq35_branch_derivatives,
            multiplicity,
        );
        let eq36_e410: f64 = (var_gem_n * (nv12 - 0.0));
        let eq36_e410_d_n0: f64 = (var_gem_n_dn0 * (nv12 - 0.0));
        let eq36_e410_d_n1: f64 = (var_gem_n_dn1 * (nv12 - 0.0));
        let eq36_e410_d_n2: f64 = (var_gem_n_dn2 * (nv12 - 0.0));
        let eq36_e410_d_n3: f64 = (var_gem_n_dn3 * (nv12 - 0.0));
        let eq36_e410_d_n4: f64 = (var_gem_n_dn4 * (nv12 - 0.0));
        let eq36_e410_d_n5: f64 = (var_gem_n_dn5 * (nv12 - 0.0));
        let eq36_e410_d_n6: f64 = (var_gem_n_dn6 * (nv12 - 0.0));
        let eq36_e410_d_n7: f64 = (var_gem_n_dn7 * (nv12 - 0.0));
        let eq36_e410_d_n8: f64 = (var_gem_n_dn8 * (nv12 - 0.0));
        let eq36_e410_d_n9: f64 = (var_gem_n_dn9 * (nv12 - 0.0));
        let eq36_e410_d_n10: f64 = (var_gem_n_dn10 * (nv12 - 0.0));
        let eq36_e410_d_n11: f64 = (var_gem_n_dn11 * (nv12 - 0.0));
        let eq36_e410_d_n12: f64 = ((var_gem_n_dn12 * (nv12 - 0.0)) + var_gem_n);
        let eq36_e410_d_b0: f64 = (var_gem_n_db0 * (nv12 - 0.0));
        let eq36_e410_d_b1: f64 = (var_gem_n_db1 * (nv12 - 0.0));
        let eq36_value: f64 = eq36_e410;
        let eq36_node_derivatives: [f64; 13] = [eq36_e410_d_n0, eq36_e410_d_n1, eq36_e410_d_n2, eq36_e410_d_n3, eq36_e410_d_n4, eq36_e410_d_n5, eq36_e410_d_n6, eq36_e410_d_n7, eq36_e410_d_n8, eq36_e410_d_n9, eq36_e410_d_n10, eq36_e410_d_n11, eq36_e410_d_n12];
        let eq36_branch_derivatives: [f64; 2] = [eq36_e410_d_b0, eq36_e410_d_b1];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq36_value),
            &eq36_node_derivatives,
            &eq36_branch_derivatives,
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let eq15_e268_q: f64 = s.rv[220];
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (s.rdn[220][4]),
        );
        let eq17_e278: f64 = (s.v[221] + s.v[226]);
        let eq17_e278_d_n0: f64 = (s.dn[221][0] + s.dn[226][0]);
        let eq17_e278_d_n1: f64 = (s.dn[221][1] + s.dn[226][1]);
        let eq17_e278_d_n2: f64 = (s.dn[221][2] + s.dn[226][2]);
        let eq17_e278_d_n3: f64 = (s.dn[221][3] + s.dn[226][3]);
        let eq17_e278_d_n4: f64 = (s.dn[221][4] + s.dn[226][4]);
        let eq17_e278_d_n5: f64 = (s.dn[221][5] + s.dn[226][5]);
        let eq17_e278_d_n6: f64 = (s.dn[221][6] + s.dn[226][6]);
        let eq17_e278_d_n7: f64 = (s.dn[221][7] + s.dn[226][7]);
        let eq17_e278_d_n8: f64 = (s.dn[221][8] + s.dn[226][8]);
        let eq17_e278_d_n9: f64 = (s.dn[221][9] + s.dn[226][9]);
        let eq17_e278_d_n10: f64 = (s.dn[221][10] + s.dn[226][10]);
        let eq17_e278_d_n11: f64 = (s.dn[221][11] + s.dn[226][11]);
        let eq17_e278_d_n12: f64 = (s.dn[221][12] + s.dn[226][12]);
        let eq17_e278_d_b0: f64 = (s.db[221][0] + s.db[226][0]);
        let eq17_e278_d_b1: f64 = (s.db[221][1] + s.db[226][1]);
        let eq17_e280: f64 = (eq17_e278 + s.v[241]);
        let eq17_e280_d_n0: f64 = (eq17_e278_d_n0 + s.dn[241][0]);
        let eq17_e280_d_n1: f64 = (eq17_e278_d_n1 + s.dn[241][1]);
        let eq17_e280_d_n2: f64 = (eq17_e278_d_n2 + s.dn[241][2]);
        let eq17_e280_d_n3: f64 = (eq17_e278_d_n3 + s.dn[241][3]);
        let eq17_e280_d_n4: f64 = (eq17_e278_d_n4 + s.dn[241][4]);
        let eq17_e280_d_n5: f64 = (eq17_e278_d_n5 + s.dn[241][5]);
        let eq17_e280_d_n6: f64 = (eq17_e278_d_n6 + s.dn[241][6]);
        let eq17_e280_d_n7: f64 = (eq17_e278_d_n7 + s.dn[241][7]);
        let eq17_e280_d_n8: f64 = (eq17_e278_d_n8 + s.dn[241][8]);
        let eq17_e280_d_n9: f64 = (eq17_e278_d_n9 + s.dn[241][9]);
        let eq17_e280_d_n10: f64 = (eq17_e278_d_n10 + s.dn[241][10]);
        let eq17_e280_d_n11: f64 = (eq17_e278_d_n11 + s.dn[241][11]);
        let eq17_e280_d_n12: f64 = (eq17_e278_d_n12 + s.dn[241][12]);
        let eq17_e280_d_b0: f64 = (eq17_e278_d_b0 + s.db[241][0]);
        let eq17_e280_d_b1: f64 = (eq17_e278_d_b1 + s.db[241][1]);
        let eq17_e281: f64 = (p.p3 * eq17_e280);
        let eq17_e281_d_n0: f64 = (p.p3 * eq17_e280_d_n0);
        let eq17_e281_d_n1: f64 = (p.p3 * eq17_e280_d_n1);
        let eq17_e281_d_n2: f64 = (p.p3 * eq17_e280_d_n2);
        let eq17_e281_d_n3: f64 = (p.p3 * eq17_e280_d_n3);
        let eq17_e281_d_n4: f64 = (p.p3 * eq17_e280_d_n4);
        let eq17_e281_d_n5: f64 = (p.p3 * eq17_e280_d_n5);
        let eq17_e281_d_n6: f64 = (p.p3 * eq17_e280_d_n6);
        let eq17_e281_d_n7: f64 = (p.p3 * eq17_e280_d_n7);
        let eq17_e281_d_n8: f64 = (p.p3 * eq17_e280_d_n8);
        let eq17_e281_d_n9: f64 = (p.p3 * eq17_e280_d_n9);
        let eq17_e281_d_n10: f64 = (p.p3 * eq17_e280_d_n10);
        let eq17_e281_d_n11: f64 = (p.p3 * eq17_e280_d_n11);
        let eq17_e281_d_n12: f64 = (p.p3 * eq17_e280_d_n12);
        let eq17_e281_d_b0: f64 = (p.p3 * eq17_e280_d_b0);
        let eq17_e281_d_b1: f64 = (p.p3 * eq17_e280_d_b1);
        let eq17_e282_q: f64 = eq17_e281;
        let eq17_e284: f64 = (eq17_e281 * p.p1);
        let eq17_e284_d_n0: f64 = (eq17_e281_d_n0 * p.p1);
        let eq17_e284_d_n1: f64 = (eq17_e281_d_n1 * p.p1);
        let eq17_e284_d_n2: f64 = (eq17_e281_d_n2 * p.p1);
        let eq17_e284_d_n3: f64 = (eq17_e281_d_n3 * p.p1);
        let eq17_e284_d_n4: f64 = (eq17_e281_d_n4 * p.p1);
        let eq17_e284_d_n5: f64 = (eq17_e281_d_n5 * p.p1);
        let eq17_e284_d_n6: f64 = (eq17_e281_d_n6 * p.p1);
        let eq17_e284_d_n7: f64 = (eq17_e281_d_n7 * p.p1);
        let eq17_e284_d_n8: f64 = (eq17_e281_d_n8 * p.p1);
        let eq17_e284_d_n9: f64 = (eq17_e281_d_n9 * p.p1);
        let eq17_e284_d_n10: f64 = (eq17_e281_d_n10 * p.p1);
        let eq17_e284_d_n11: f64 = (eq17_e281_d_n11 * p.p1);
        let eq17_e284_d_n12: f64 = (eq17_e281_d_n12 * p.p1);
        let eq17_e284_d_b0: f64 = (eq17_e281_d_b0 * p.p1);
        let eq17_e284_d_b1: f64 = (eq17_e281_d_b1 * p.p1);
        let eq17_e284_q: f64 = (eq17_e282_q * p.p1);
        let eq17_reactive_node_derivatives: [f64; 13] = [eq17_e284_d_n0, eq17_e284_d_n1, eq17_e284_d_n2, eq17_e284_d_n3, eq17_e284_d_n4, eq17_e284_d_n5, eq17_e284_d_n6, eq17_e284_d_n7, eq17_e284_d_n8, eq17_e284_d_n9, eq17_e284_d_n10, eq17_e284_d_n11, eq17_e284_d_n12];
        let eq17_reactive_branch_derivatives: [f64; 2] = [eq17_e284_d_b0, eq17_e284_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq17_reactive_node_derivatives,
            branches,
            &eq17_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e287: f64 = (p.p3 * s.v[223]);
        let eq18_e288_q: f64 = eq18_e287;
        let eq18_e290: f64 = (eq18_e287 * p.p1);
        let eq18_e290_d_n0: f64 = ((p.p3 * s.dn[223][0]) * p.p1);
        let eq18_e290_d_n1: f64 = ((p.p3 * s.dn[223][1]) * p.p1);
        let eq18_e290_d_n2: f64 = ((p.p3 * s.dn[223][2]) * p.p1);
        let eq18_e290_d_n3: f64 = ((p.p3 * s.dn[223][3]) * p.p1);
        let eq18_e290_d_n4: f64 = ((p.p3 * s.dn[223][4]) * p.p1);
        let eq18_e290_d_n5: f64 = ((p.p3 * s.dn[223][5]) * p.p1);
        let eq18_e290_d_n6: f64 = ((p.p3 * s.dn[223][6]) * p.p1);
        let eq18_e290_d_n7: f64 = ((p.p3 * s.dn[223][7]) * p.p1);
        let eq18_e290_d_n8: f64 = ((p.p3 * s.dn[223][8]) * p.p1);
        let eq18_e290_d_n9: f64 = ((p.p3 * s.dn[223][9]) * p.p1);
        let eq18_e290_d_n10: f64 = ((p.p3 * s.dn[223][10]) * p.p1);
        let eq18_e290_d_n11: f64 = ((p.p3 * s.dn[223][11]) * p.p1);
        let eq18_e290_d_n12: f64 = ((p.p3 * s.dn[223][12]) * p.p1);
        let eq18_e290_d_b0: f64 = ((p.p3 * s.db[223][0]) * p.p1);
        let eq18_e290_d_b1: f64 = ((p.p3 * s.db[223][1]) * p.p1);
        let eq18_e290_q: f64 = (eq18_e288_q * p.p1);
        let eq18_reactive_node_derivatives: [f64; 13] = [eq18_e290_d_n0, eq18_e290_d_n1, eq18_e290_d_n2, eq18_e290_d_n3, eq18_e290_d_n4, eq18_e290_d_n5, eq18_e290_d_n6, eq18_e290_d_n7, eq18_e290_d_n8, eq18_e290_d_n9, eq18_e290_d_n10, eq18_e290_d_n11, eq18_e290_d_n12];
        let eq18_reactive_branch_derivatives: [f64; 2] = [eq18_e290_d_b0, eq18_e290_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[5]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e294: f64 = (s.v[224] + s.v[227]);
        let eq19_e294_d_n0: f64 = (s.dn[224][0] + s.dn[227][0]);
        let eq19_e294_d_n1: f64 = (s.dn[224][1] + s.dn[227][1]);
        let eq19_e294_d_n2: f64 = (s.dn[224][2] + s.dn[227][2]);
        let eq19_e294_d_n3: f64 = (s.dn[224][3] + s.dn[227][3]);
        let eq19_e294_d_n4: f64 = (s.dn[224][4] + s.dn[227][4]);
        let eq19_e294_d_n5: f64 = (s.dn[224][5] + s.dn[227][5]);
        let eq19_e294_d_n6: f64 = (s.dn[224][6] + s.dn[227][6]);
        let eq19_e294_d_n7: f64 = (s.dn[224][7] + s.dn[227][7]);
        let eq19_e294_d_n8: f64 = (s.dn[224][8] + s.dn[227][8]);
        let eq19_e294_d_n9: f64 = (s.dn[224][9] + s.dn[227][9]);
        let eq19_e294_d_n10: f64 = (s.dn[224][10] + s.dn[227][10]);
        let eq19_e294_d_n11: f64 = (s.dn[224][11] + s.dn[227][11]);
        let eq19_e294_d_n12: f64 = (s.dn[224][12] + s.dn[227][12]);
        let eq19_e294_d_b0: f64 = (s.db[224][0] + s.db[227][0]);
        let eq19_e294_d_b1: f64 = (s.db[224][1] + s.db[227][1]);
        let eq19_e296: f64 = (eq19_e294 + s.v[244]);
        let eq19_e296_d_n0: f64 = (eq19_e294_d_n0 + s.dn[244][0]);
        let eq19_e296_d_n1: f64 = (eq19_e294_d_n1 + s.dn[244][1]);
        let eq19_e296_d_n2: f64 = (eq19_e294_d_n2 + s.dn[244][2]);
        let eq19_e296_d_n3: f64 = (eq19_e294_d_n3 + s.dn[244][3]);
        let eq19_e296_d_n4: f64 = (eq19_e294_d_n4 + s.dn[244][4]);
        let eq19_e296_d_n5: f64 = (eq19_e294_d_n5 + s.dn[244][5]);
        let eq19_e296_d_n6: f64 = (eq19_e294_d_n6 + s.dn[244][6]);
        let eq19_e296_d_n7: f64 = (eq19_e294_d_n7 + s.dn[244][7]);
        let eq19_e296_d_n8: f64 = (eq19_e294_d_n8 + s.dn[244][8]);
        let eq19_e296_d_n9: f64 = (eq19_e294_d_n9 + s.dn[244][9]);
        let eq19_e296_d_n10: f64 = (eq19_e294_d_n10 + s.dn[244][10]);
        let eq19_e296_d_n11: f64 = (eq19_e294_d_n11 + s.dn[244][11]);
        let eq19_e296_d_n12: f64 = (eq19_e294_d_n12 + s.dn[244][12]);
        let eq19_e296_d_b0: f64 = (eq19_e294_d_b0 + s.db[244][0]);
        let eq19_e296_d_b1: f64 = (eq19_e294_d_b1 + s.db[244][1]);
        let eq19_e297: f64 = (p.p3 * eq19_e296);
        let eq19_e297_d_n0: f64 = (p.p3 * eq19_e296_d_n0);
        let eq19_e297_d_n1: f64 = (p.p3 * eq19_e296_d_n1);
        let eq19_e297_d_n2: f64 = (p.p3 * eq19_e296_d_n2);
        let eq19_e297_d_n3: f64 = (p.p3 * eq19_e296_d_n3);
        let eq19_e297_d_n4: f64 = (p.p3 * eq19_e296_d_n4);
        let eq19_e297_d_n5: f64 = (p.p3 * eq19_e296_d_n5);
        let eq19_e297_d_n6: f64 = (p.p3 * eq19_e296_d_n6);
        let eq19_e297_d_n7: f64 = (p.p3 * eq19_e296_d_n7);
        let eq19_e297_d_n8: f64 = (p.p3 * eq19_e296_d_n8);
        let eq19_e297_d_n9: f64 = (p.p3 * eq19_e296_d_n9);
        let eq19_e297_d_n10: f64 = (p.p3 * eq19_e296_d_n10);
        let eq19_e297_d_n11: f64 = (p.p3 * eq19_e296_d_n11);
        let eq19_e297_d_n12: f64 = (p.p3 * eq19_e296_d_n12);
        let eq19_e297_d_b0: f64 = (p.p3 * eq19_e296_d_b0);
        let eq19_e297_d_b1: f64 = (p.p3 * eq19_e296_d_b1);
        let eq19_e298_q: f64 = eq19_e297;
        let eq19_e300: f64 = (eq19_e297 * p.p1);
        let eq19_e300_d_n0: f64 = (eq19_e297_d_n0 * p.p1);
        let eq19_e300_d_n1: f64 = (eq19_e297_d_n1 * p.p1);
        let eq19_e300_d_n2: f64 = (eq19_e297_d_n2 * p.p1);
        let eq19_e300_d_n3: f64 = (eq19_e297_d_n3 * p.p1);
        let eq19_e300_d_n4: f64 = (eq19_e297_d_n4 * p.p1);
        let eq19_e300_d_n5: f64 = (eq19_e297_d_n5 * p.p1);
        let eq19_e300_d_n6: f64 = (eq19_e297_d_n6 * p.p1);
        let eq19_e300_d_n7: f64 = (eq19_e297_d_n7 * p.p1);
        let eq19_e300_d_n8: f64 = (eq19_e297_d_n8 * p.p1);
        let eq19_e300_d_n9: f64 = (eq19_e297_d_n9 * p.p1);
        let eq19_e300_d_n10: f64 = (eq19_e297_d_n10 * p.p1);
        let eq19_e300_d_n11: f64 = (eq19_e297_d_n11 * p.p1);
        let eq19_e300_d_n12: f64 = (eq19_e297_d_n12 * p.p1);
        let eq19_e300_d_b0: f64 = (eq19_e297_d_b0 * p.p1);
        let eq19_e300_d_b1: f64 = (eq19_e297_d_b1 * p.p1);
        let eq19_e300_q: f64 = (eq19_e298_q * p.p1);
        let eq19_reactive_node_derivatives: [f64; 13] = [eq19_e300_d_n0, eq19_e300_d_n1, eq19_e300_d_n2, eq19_e300_d_n3, eq19_e300_d_n4, eq19_e300_d_n5, eq19_e300_d_n6, eq19_e300_d_n7, eq19_e300_d_n8, eq19_e300_d_n9, eq19_e300_d_n10, eq19_e300_d_n11, eq19_e300_d_n12];
        let eq19_reactive_branch_derivatives: [f64; 2] = [eq19_e300_d_b0, eq19_e300_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let eq20_e303: f64 = (p.p3 * s.v[239]);
        let eq20_e304_q: f64 = eq20_e303;
        let eq20_e306: f64 = (eq20_e303 * p.p1);
        let eq20_e306_d_n0: f64 = ((p.p3 * s.dn[239][0]) * p.p1);
        let eq20_e306_d_n1: f64 = ((p.p3 * s.dn[239][1]) * p.p1);
        let eq20_e306_d_n2: f64 = ((p.p3 * s.dn[239][2]) * p.p1);
        let eq20_e306_d_n3: f64 = ((p.p3 * s.dn[239][3]) * p.p1);
        let eq20_e306_d_n4: f64 = ((p.p3 * s.dn[239][4]) * p.p1);
        let eq20_e306_d_n5: f64 = ((p.p3 * s.dn[239][5]) * p.p1);
        let eq20_e306_d_n6: f64 = ((p.p3 * s.dn[239][6]) * p.p1);
        let eq20_e306_d_n7: f64 = ((p.p3 * s.dn[239][7]) * p.p1);
        let eq20_e306_d_n8: f64 = ((p.p3 * s.dn[239][8]) * p.p1);
        let eq20_e306_d_n9: f64 = ((p.p3 * s.dn[239][9]) * p.p1);
        let eq20_e306_d_n10: f64 = ((p.p3 * s.dn[239][10]) * p.p1);
        let eq20_e306_d_n11: f64 = ((p.p3 * s.dn[239][11]) * p.p1);
        let eq20_e306_d_n12: f64 = ((p.p3 * s.dn[239][12]) * p.p1);
        let eq20_e306_d_b0: f64 = ((p.p3 * s.db[239][0]) * p.p1);
        let eq20_e306_d_b1: f64 = ((p.p3 * s.db[239][1]) * p.p1);
        let eq20_e306_q: f64 = (eq20_e304_q * p.p1);
        let eq20_reactive_node_derivatives: [f64; 13] = [eq20_e306_d_n0, eq20_e306_d_n1, eq20_e306_d_n2, eq20_e306_d_n3, eq20_e306_d_n4, eq20_e306_d_n5, eq20_e306_d_n6, eq20_e306_d_n7, eq20_e306_d_n8, eq20_e306_d_n9, eq20_e306_d_n10, eq20_e306_d_n11, eq20_e306_d_n12];
        let eq20_reactive_branch_derivatives: [f64; 2] = [eq20_e306_d_b0, eq20_e306_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &eq20_reactive_branch_derivatives,
            multiplicity,
        );
        let eq21_e309: f64 = (p.p3 * s.v[228]);
        let eq21_e310_q: f64 = eq21_e309;
        let eq21_e312: f64 = (eq21_e309 * p.p1);
        let eq21_e312_d_n0: f64 = ((p.p3 * s.dn[228][0]) * p.p1);
        let eq21_e312_d_n1: f64 = ((p.p3 * s.dn[228][1]) * p.p1);
        let eq21_e312_d_n2: f64 = ((p.p3 * s.dn[228][2]) * p.p1);
        let eq21_e312_d_n3: f64 = ((p.p3 * s.dn[228][3]) * p.p1);
        let eq21_e312_d_n4: f64 = ((p.p3 * s.dn[228][4]) * p.p1);
        let eq21_e312_d_n5: f64 = ((p.p3 * s.dn[228][5]) * p.p1);
        let eq21_e312_d_n6: f64 = ((p.p3 * s.dn[228][6]) * p.p1);
        let eq21_e312_d_n7: f64 = ((p.p3 * s.dn[228][7]) * p.p1);
        let eq21_e312_d_n8: f64 = ((p.p3 * s.dn[228][8]) * p.p1);
        let eq21_e312_d_n9: f64 = ((p.p3 * s.dn[228][9]) * p.p1);
        let eq21_e312_d_n10: f64 = ((p.p3 * s.dn[228][10]) * p.p1);
        let eq21_e312_d_n11: f64 = ((p.p3 * s.dn[228][11]) * p.p1);
        let eq21_e312_d_n12: f64 = ((p.p3 * s.dn[228][12]) * p.p1);
        let eq21_e312_d_b0: f64 = ((p.p3 * s.db[228][0]) * p.p1);
        let eq21_e312_d_b1: f64 = ((p.p3 * s.db[228][1]) * p.p1);
        let eq21_e312_q: f64 = (eq21_e310_q * p.p1);
        let eq21_reactive_node_derivatives: [f64; 13] = [eq21_e312_d_n0, eq21_e312_d_n1, eq21_e312_d_n2, eq21_e312_d_n3, eq21_e312_d_n4, eq21_e312_d_n5, eq21_e312_d_n6, eq21_e312_d_n7, eq21_e312_d_n8, eq21_e312_d_n9, eq21_e312_d_n10, eq21_e312_d_n11, eq21_e312_d_n12];
        let eq21_reactive_branch_derivatives: [f64; 2] = [eq21_e312_d_b0, eq21_e312_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq21_reactive_node_derivatives,
            branches,
            &eq21_reactive_branch_derivatives,
            multiplicity,
        );
        let eq22_e315: f64 = (p.p3 * p.p69);
        let eq22_e317: f64 = (eq22_e315 * s.v[269]);
        let eq22_e318_q: f64 = eq22_e317;
        let eq22_e320: f64 = (eq22_e317 * p.p1);
        let eq22_e320_d_n0: f64 = ((eq22_e315 * s.dn[269][0]) * p.p1);
        let eq22_e320_d_n1: f64 = ((eq22_e315 * s.dn[269][1]) * p.p1);
        let eq22_e320_d_n2: f64 = ((eq22_e315 * s.dn[269][2]) * p.p1);
        let eq22_e320_d_n3: f64 = ((eq22_e315 * s.dn[269][3]) * p.p1);
        let eq22_e320_d_n4: f64 = ((eq22_e315 * s.dn[269][4]) * p.p1);
        let eq22_e320_d_n5: f64 = ((eq22_e315 * s.dn[269][5]) * p.p1);
        let eq22_e320_d_n6: f64 = ((eq22_e315 * s.dn[269][6]) * p.p1);
        let eq22_e320_d_n7: f64 = ((eq22_e315 * s.dn[269][7]) * p.p1);
        let eq22_e320_d_n8: f64 = ((eq22_e315 * s.dn[269][8]) * p.p1);
        let eq22_e320_d_n9: f64 = ((eq22_e315 * s.dn[269][9]) * p.p1);
        let eq22_e320_d_n10: f64 = ((eq22_e315 * s.dn[269][10]) * p.p1);
        let eq22_e320_d_n11: f64 = ((eq22_e315 * s.dn[269][11]) * p.p1);
        let eq22_e320_d_n12: f64 = ((eq22_e315 * s.dn[269][12]) * p.p1);
        let eq22_e320_d_b0: f64 = ((eq22_e315 * s.db[269][0]) * p.p1);
        let eq22_e320_d_b1: f64 = ((eq22_e315 * s.db[269][1]) * p.p1);
        let eq22_e320_q: f64 = (eq22_e318_q * p.p1);
        let eq22_reactive_node_derivatives: [f64; 13] = [eq22_e320_d_n0, eq22_e320_d_n1, eq22_e320_d_n2, eq22_e320_d_n3, eq22_e320_d_n4, eq22_e320_d_n5, eq22_e320_d_n6, eq22_e320_d_n7, eq22_e320_d_n8, eq22_e320_d_n9, eq22_e320_d_n10, eq22_e320_d_n11, eq22_e320_d_n12];
        let eq22_reactive_branch_derivatives: [f64; 2] = [eq22_e320_d_b0, eq22_e320_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes,
            &eq22_reactive_node_derivatives,
            branches,
            &eq22_reactive_branch_derivatives,
            multiplicity,
        );
        let eq23_e323: f64 = (p.p3 * p.p78);
        let eq23_e325: f64 = (eq23_e323 * s.v[270]);
        let eq23_e326_q: f64 = eq23_e325;
        let eq23_e328: f64 = (eq23_e325 * p.p1);
        let eq23_e328_d_n0: f64 = ((eq23_e323 * s.dn[270][0]) * p.p1);
        let eq23_e328_d_n1: f64 = ((eq23_e323 * s.dn[270][1]) * p.p1);
        let eq23_e328_d_n2: f64 = ((eq23_e323 * s.dn[270][2]) * p.p1);
        let eq23_e328_d_n3: f64 = ((eq23_e323 * s.dn[270][3]) * p.p1);
        let eq23_e328_d_n4: f64 = ((eq23_e323 * s.dn[270][4]) * p.p1);
        let eq23_e328_d_n5: f64 = ((eq23_e323 * s.dn[270][5]) * p.p1);
        let eq23_e328_d_n6: f64 = ((eq23_e323 * s.dn[270][6]) * p.p1);
        let eq23_e328_d_n7: f64 = ((eq23_e323 * s.dn[270][7]) * p.p1);
        let eq23_e328_d_n8: f64 = ((eq23_e323 * s.dn[270][8]) * p.p1);
        let eq23_e328_d_n9: f64 = ((eq23_e323 * s.dn[270][9]) * p.p1);
        let eq23_e328_d_n10: f64 = ((eq23_e323 * s.dn[270][10]) * p.p1);
        let eq23_e328_d_n11: f64 = ((eq23_e323 * s.dn[270][11]) * p.p1);
        let eq23_e328_d_n12: f64 = ((eq23_e323 * s.dn[270][12]) * p.p1);
        let eq23_e328_d_b0: f64 = ((eq23_e323 * s.db[270][0]) * p.p1);
        let eq23_e328_d_b1: f64 = ((eq23_e323 * s.db[270][1]) * p.p1);
        let eq23_e328_q: f64 = (eq23_e326_q * p.p1);
        let eq23_reactive_node_derivatives: [f64; 13] = [eq23_e328_d_n0, eq23_e328_d_n1, eq23_e328_d_n2, eq23_e328_d_n3, eq23_e328_d_n4, eq23_e328_d_n5, eq23_e328_d_n6, eq23_e328_d_n7, eq23_e328_d_n8, eq23_e328_d_n9, eq23_e328_d_n10, eq23_e328_d_n11, eq23_e328_d_n12];
        let eq23_reactive_branch_derivatives: [f64; 2] = [eq23_e328_d_b0, eq23_e328_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
        let eq26_e344: f64 = (s.v[236] + s.v[248]);
        let eq26_e344_d_n0: f64 = (s.dn[236][0] + s.dn[248][0]);
        let eq26_e344_d_n1: f64 = (s.dn[236][1] + s.dn[248][1]);
        let eq26_e344_d_n2: f64 = (s.dn[236][2] + s.dn[248][2]);
        let eq26_e344_d_n3: f64 = (s.dn[236][3] + s.dn[248][3]);
        let eq26_e344_d_n4: f64 = (s.dn[236][4] + s.dn[248][4]);
        let eq26_e344_d_n5: f64 = (s.dn[236][5] + s.dn[248][5]);
        let eq26_e344_d_n6: f64 = (s.dn[236][6] + s.dn[248][6]);
        let eq26_e344_d_n7: f64 = (s.dn[236][7] + s.dn[248][7]);
        let eq26_e344_d_n8: f64 = (s.dn[236][8] + s.dn[248][8]);
        let eq26_e344_d_n9: f64 = (s.dn[236][9] + s.dn[248][9]);
        let eq26_e344_d_n10: f64 = (s.dn[236][10] + s.dn[248][10]);
        let eq26_e344_d_n11: f64 = (s.dn[236][11] + s.dn[248][11]);
        let eq26_e344_d_n12: f64 = (s.dn[236][12] + s.dn[248][12]);
        let eq26_e344_d_b0: f64 = (s.db[236][0] + s.db[248][0]);
        let eq26_e344_d_b1: f64 = (s.db[236][1] + s.db[248][1]);
        let eq26_e345: f64 = (p.p3 * eq26_e344);
        let eq26_e345_d_n0: f64 = (p.p3 * eq26_e344_d_n0);
        let eq26_e345_d_n1: f64 = (p.p3 * eq26_e344_d_n1);
        let eq26_e345_d_n2: f64 = (p.p3 * eq26_e344_d_n2);
        let eq26_e345_d_n3: f64 = (p.p3 * eq26_e344_d_n3);
        let eq26_e345_d_n4: f64 = (p.p3 * eq26_e344_d_n4);
        let eq26_e345_d_n5: f64 = (p.p3 * eq26_e344_d_n5);
        let eq26_e345_d_n6: f64 = (p.p3 * eq26_e344_d_n6);
        let eq26_e345_d_n7: f64 = (p.p3 * eq26_e344_d_n7);
        let eq26_e345_d_n8: f64 = (p.p3 * eq26_e344_d_n8);
        let eq26_e345_d_n9: f64 = (p.p3 * eq26_e344_d_n9);
        let eq26_e345_d_n10: f64 = (p.p3 * eq26_e344_d_n10);
        let eq26_e345_d_n11: f64 = (p.p3 * eq26_e344_d_n11);
        let eq26_e345_d_n12: f64 = (p.p3 * eq26_e344_d_n12);
        let eq26_e345_d_b0: f64 = (p.p3 * eq26_e344_d_b0);
        let eq26_e345_d_b1: f64 = (p.p3 * eq26_e344_d_b1);
        let eq26_e346_q: f64 = eq26_e345;
        let eq26_e348: f64 = (eq26_e345 * p.p1);
        let eq26_e348_d_n0: f64 = (eq26_e345_d_n0 * p.p1);
        let eq26_e348_d_n1: f64 = (eq26_e345_d_n1 * p.p1);
        let eq26_e348_d_n2: f64 = (eq26_e345_d_n2 * p.p1);
        let eq26_e348_d_n3: f64 = (eq26_e345_d_n3 * p.p1);
        let eq26_e348_d_n4: f64 = (eq26_e345_d_n4 * p.p1);
        let eq26_e348_d_n5: f64 = (eq26_e345_d_n5 * p.p1);
        let eq26_e348_d_n6: f64 = (eq26_e345_d_n6 * p.p1);
        let eq26_e348_d_n7: f64 = (eq26_e345_d_n7 * p.p1);
        let eq26_e348_d_n8: f64 = (eq26_e345_d_n8 * p.p1);
        let eq26_e348_d_n9: f64 = (eq26_e345_d_n9 * p.p1);
        let eq26_e348_d_n10: f64 = (eq26_e345_d_n10 * p.p1);
        let eq26_e348_d_n11: f64 = (eq26_e345_d_n11 * p.p1);
        let eq26_e348_d_n12: f64 = (eq26_e345_d_n12 * p.p1);
        let eq26_e348_d_b0: f64 = (eq26_e345_d_b0 * p.p1);
        let eq26_e348_d_b1: f64 = (eq26_e345_d_b1 * p.p1);
        let eq26_e348_q: f64 = (eq26_e346_q * p.p1);
        let eq26_reactive_node_derivatives: [f64; 13] = [eq26_e348_d_n0, eq26_e348_d_n1, eq26_e348_d_n2, eq26_e348_d_n3, eq26_e348_d_n4, eq26_e348_d_n5, eq26_e348_d_n6, eq26_e348_d_n7, eq26_e348_d_n8, eq26_e348_d_n9, eq26_e348_d_n10, eq26_e348_d_n11, eq26_e348_d_n12];
        let eq26_reactive_branch_derivatives: [f64; 2] = [eq26_e348_d_b0, eq26_e348_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[10]),
            nodes,
            &eq26_reactive_node_derivatives,
            branches,
            &eq26_reactive_branch_derivatives,
            multiplicity,
        );
        let eq28_e363: f64 = (s.v[233] + s.v[249]);
        let eq28_e363_d_n0: f64 = (s.dn[233][0] + s.dn[249][0]);
        let eq28_e363_d_n1: f64 = (s.dn[233][1] + s.dn[249][1]);
        let eq28_e363_d_n2: f64 = (s.dn[233][2] + s.dn[249][2]);
        let eq28_e363_d_n3: f64 = (s.dn[233][3] + s.dn[249][3]);
        let eq28_e363_d_n4: f64 = (s.dn[233][4] + s.dn[249][4]);
        let eq28_e363_d_n5: f64 = (s.dn[233][5] + s.dn[249][5]);
        let eq28_e363_d_n6: f64 = (s.dn[233][6] + s.dn[249][6]);
        let eq28_e363_d_n7: f64 = (s.dn[233][7] + s.dn[249][7]);
        let eq28_e363_d_n8: f64 = (s.dn[233][8] + s.dn[249][8]);
        let eq28_e363_d_n9: f64 = (s.dn[233][9] + s.dn[249][9]);
        let eq28_e363_d_n10: f64 = (s.dn[233][10] + s.dn[249][10]);
        let eq28_e363_d_n11: f64 = (s.dn[233][11] + s.dn[249][11]);
        let eq28_e363_d_n12: f64 = (s.dn[233][12] + s.dn[249][12]);
        let eq28_e363_d_b0: f64 = (s.db[233][0] + s.db[249][0]);
        let eq28_e363_d_b1: f64 = (s.db[233][1] + s.db[249][1]);
        let eq28_e364: f64 = (p.p3 * eq28_e363);
        let eq28_e364_d_n0: f64 = (p.p3 * eq28_e363_d_n0);
        let eq28_e364_d_n1: f64 = (p.p3 * eq28_e363_d_n1);
        let eq28_e364_d_n2: f64 = (p.p3 * eq28_e363_d_n2);
        let eq28_e364_d_n3: f64 = (p.p3 * eq28_e363_d_n3);
        let eq28_e364_d_n4: f64 = (p.p3 * eq28_e363_d_n4);
        let eq28_e364_d_n5: f64 = (p.p3 * eq28_e363_d_n5);
        let eq28_e364_d_n6: f64 = (p.p3 * eq28_e363_d_n6);
        let eq28_e364_d_n7: f64 = (p.p3 * eq28_e363_d_n7);
        let eq28_e364_d_n8: f64 = (p.p3 * eq28_e363_d_n8);
        let eq28_e364_d_n9: f64 = (p.p3 * eq28_e363_d_n9);
        let eq28_e364_d_n10: f64 = (p.p3 * eq28_e363_d_n10);
        let eq28_e364_d_n11: f64 = (p.p3 * eq28_e363_d_n11);
        let eq28_e364_d_n12: f64 = (p.p3 * eq28_e363_d_n12);
        let eq28_e364_d_b0: f64 = (p.p3 * eq28_e363_d_b0);
        let eq28_e364_d_b1: f64 = (p.p3 * eq28_e363_d_b1);
        let eq28_e365_q: f64 = eq28_e364;
        let eq28_e367: f64 = (eq28_e364 * p.p1);
        let eq28_e367_d_n0: f64 = (eq28_e364_d_n0 * p.p1);
        let eq28_e367_d_n1: f64 = (eq28_e364_d_n1 * p.p1);
        let eq28_e367_d_n2: f64 = (eq28_e364_d_n2 * p.p1);
        let eq28_e367_d_n3: f64 = (eq28_e364_d_n3 * p.p1);
        let eq28_e367_d_n4: f64 = (eq28_e364_d_n4 * p.p1);
        let eq28_e367_d_n5: f64 = (eq28_e364_d_n5 * p.p1);
        let eq28_e367_d_n6: f64 = (eq28_e364_d_n6 * p.p1);
        let eq28_e367_d_n7: f64 = (eq28_e364_d_n7 * p.p1);
        let eq28_e367_d_n8: f64 = (eq28_e364_d_n8 * p.p1);
        let eq28_e367_d_n9: f64 = (eq28_e364_d_n9 * p.p1);
        let eq28_e367_d_n10: f64 = (eq28_e364_d_n10 * p.p1);
        let eq28_e367_d_n11: f64 = (eq28_e364_d_n11 * p.p1);
        let eq28_e367_d_n12: f64 = (eq28_e364_d_n12 * p.p1);
        let eq28_e367_d_b0: f64 = (eq28_e364_d_b0 * p.p1);
        let eq28_e367_d_b1: f64 = (eq28_e364_d_b1 * p.p1);
        let eq28_e367_q: f64 = (eq28_e365_q * p.p1);
        let eq28_reactive_node_derivatives: [f64; 13] = [eq28_e367_d_n0, eq28_e367_d_n1, eq28_e367_d_n2, eq28_e367_d_n3, eq28_e367_d_n4, eq28_e367_d_n5, eq28_e367_d_n6, eq28_e367_d_n7, eq28_e367_d_n8, eq28_e367_d_n9, eq28_e367_d_n10, eq28_e367_d_n11, eq28_e367_d_n12];
        let eq28_reactive_branch_derivatives: [f64; 2] = [eq28_e367_d_b0, eq28_e367_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[11]),
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
        let eq35_e406_q: f64 = (nv12 - 0.0);
        let eq35_e407: f64 = (s.v[336] * (nv12 - 0.0));
        let eq35_e407_d_n0: f64 = (s.dn[336][0] * (nv12 - 0.0));
        let eq35_e407_d_n1: f64 = (s.dn[336][1] * (nv12 - 0.0));
        let eq35_e407_d_n2: f64 = (s.dn[336][2] * (nv12 - 0.0));
        let eq35_e407_d_n3: f64 = (s.dn[336][3] * (nv12 - 0.0));
        let eq35_e407_d_n4: f64 = (s.dn[336][4] * (nv12 - 0.0));
        let eq35_e407_d_n5: f64 = (s.dn[336][5] * (nv12 - 0.0));
        let eq35_e407_d_n6: f64 = (s.dn[336][6] * (nv12 - 0.0));
        let eq35_e407_d_n7: f64 = (s.dn[336][7] * (nv12 - 0.0));
        let eq35_e407_d_n8: f64 = (s.dn[336][8] * (nv12 - 0.0));
        let eq35_e407_d_n9: f64 = (s.dn[336][9] * (nv12 - 0.0));
        let eq35_e407_d_n10: f64 = (s.dn[336][10] * (nv12 - 0.0));
        let eq35_e407_d_n11: f64 = (s.dn[336][11] * (nv12 - 0.0));
        let eq35_e407_d_n12: f64 = ((s.dn[336][12] * (nv12 - 0.0)) + s.v[336]);
        let eq35_e407_d_b0: f64 = (s.db[336][0] * (nv12 - 0.0));
        let eq35_e407_d_b1: f64 = (s.db[336][1] * (nv12 - 0.0));
        let eq35_e407_q: f64 = (s.v[336] * eq35_e406_q);
        let eq35_e407_q_d_n12: f64 = ((s.dn[336][12] * eq35_e406_q) + s.v[336]);
        let eq35_reactive_node_derivatives: [f64; 13] = [(s.dn[336][0] * eq35_e406_q), (s.dn[336][1] * eq35_e406_q), (s.dn[336][2] * eq35_e406_q), (s.dn[336][3] * eq35_e406_q), (s.dn[336][4] * eq35_e406_q), (s.dn[336][5] * eq35_e406_q), (s.dn[336][6] * eq35_e406_q), (s.dn[336][7] * eq35_e406_q), (s.dn[336][8] * eq35_e406_q), (s.dn[336][9] * eq35_e406_q), (s.dn[336][10] * eq35_e406_q), (s.dn[336][11] * eq35_e406_q), eq35_e407_q_d_n12];
        let eq35_reactive_branch_derivatives: [f64; 2] = [(s.db[336][0] * eq35_e406_q), (s.db[336][1] * eq35_e406_q)];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
    }
}

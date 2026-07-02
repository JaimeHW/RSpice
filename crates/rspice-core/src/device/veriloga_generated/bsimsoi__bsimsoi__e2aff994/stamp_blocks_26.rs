#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

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
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq7_e1546, eq7_e1546_d_n3, eq7_e1546_d_n4, eq7_e1546_d_n5, eq7_e1546_d_n6, eq7_e1546_d_n7, eq7_e1546_d_n8, eq7_e1546_d_n9, eq7_e1546_d_n10, eq7_e1546_d_n11, eq7_e1546_d_n12,) = {
    if ((locals.var_guard492 != 0.0) && ((locals.var_guard666 != 0.0) && (locals.var_guard665 == 0.0))) {
        let eq7_e1535: f64 = (locals.var_mig * locals.var_cox);
        let eq7_e1535_d_n3: f64 = (locals.var_mig_dn3 * locals.var_cox);
        let eq7_e1535_d_n4: f64 = (locals.var_mig_dn4 * locals.var_cox);
        let eq7_e1535_d_n5: f64 = (locals.var_mig_dn5 * locals.var_cox);
        let eq7_e1535_d_n6: f64 = (locals.var_mig_dn6 * locals.var_cox);
        let eq7_e1535_d_n7: f64 = (locals.var_mig_dn7 * locals.var_cox);
        let eq7_e1535_d_n8: f64 = (locals.var_mig_dn8 * locals.var_cox);
        let eq7_e1535_d_n9: f64 = (locals.var_mig_dn9 * locals.var_cox);
        let eq7_e1535_d_n10: f64 = (locals.var_mig_dn10 * locals.var_cox);
        let eq7_e1535_d_n11: f64 = (locals.var_mig_dn11 * locals.var_cox);
        let eq7_e1537: f64 = (eq7_e1535 * locals.var_weff);
        let eq7_e1537_d_n3: f64 = (eq7_e1535_d_n3 * locals.var_weff);
        let eq7_e1537_d_n4: f64 = (eq7_e1535_d_n4 * locals.var_weff);
        let eq7_e1537_d_n5: f64 = (eq7_e1535_d_n5 * locals.var_weff);
        let eq7_e1537_d_n6: f64 = (eq7_e1535_d_n6 * locals.var_weff);
        let eq7_e1537_d_n7: f64 = (eq7_e1535_d_n7 * locals.var_weff);
        let eq7_e1537_d_n8: f64 = (eq7_e1535_d_n8 * locals.var_weff);
        let eq7_e1537_d_n9: f64 = (eq7_e1535_d_n9 * locals.var_weff);
        let eq7_e1537_d_n10: f64 = (eq7_e1535_d_n10 * locals.var_weff);
        let eq7_e1537_d_n11: f64 = (eq7_e1535_d_n11 * locals.var_weff);
        let eq7_e1539: f64 = (eq7_e1537 * p.p2);
        let eq7_e1539_d_n3: f64 = (eq7_e1537_d_n3 * p.p2);
        let eq7_e1539_d_n4: f64 = (eq7_e1537_d_n4 * p.p2);
        let eq7_e1539_d_n5: f64 = (eq7_e1537_d_n5 * p.p2);
        let eq7_e1539_d_n6: f64 = (eq7_e1537_d_n6 * p.p2);
        let eq7_e1539_d_n7: f64 = (eq7_e1537_d_n7 * p.p2);
        let eq7_e1539_d_n8: f64 = (eq7_e1537_d_n8 * p.p2);
        let eq7_e1539_d_n9: f64 = (eq7_e1537_d_n9 * p.p2);
        let eq7_e1539_d_n10: f64 = (eq7_e1537_d_n10 * p.p2);
        let eq7_e1539_d_n11: f64 = (eq7_e1537_d_n11 * p.p2);
        let eq7_e1541: f64 = (eq7_e1539 * locals.var_leff);
        let eq7_e1541_d_n3: f64 = (eq7_e1539_d_n3 * locals.var_leff);
        let eq7_e1541_d_n4: f64 = (eq7_e1539_d_n4 * locals.var_leff);
        let eq7_e1541_d_n5: f64 = (eq7_e1539_d_n5 * locals.var_leff);
        let eq7_e1541_d_n6: f64 = (eq7_e1539_d_n6 * locals.var_leff);
        let eq7_e1541_d_n7: f64 = (eq7_e1539_d_n7 * locals.var_leff);
        let eq7_e1541_d_n8: f64 = (eq7_e1539_d_n8 * locals.var_leff);
        let eq7_e1541_d_n9: f64 = (eq7_e1539_d_n9 * locals.var_leff);
        let eq7_e1541_d_n10: f64 = (eq7_e1539_d_n10 * locals.var_leff);
        let eq7_e1541_d_n11: f64 = (eq7_e1539_d_n11 * locals.var_leff);
        let eq7_e1543: f64 = (eq7_e1541 * (nv12 - 0.0));
        let eq7_e1543_d_n3: f64 = (eq7_e1541_d_n3 * (nv12 - 0.0));
        let eq7_e1543_d_n4: f64 = (eq7_e1541_d_n4 * (nv12 - 0.0));
        let eq7_e1543_d_n5: f64 = (eq7_e1541_d_n5 * (nv12 - 0.0));
        let eq7_e1543_d_n6: f64 = (eq7_e1541_d_n6 * (nv12 - 0.0));
        let eq7_e1543_d_n7: f64 = (eq7_e1541_d_n7 * (nv12 - 0.0));
        let eq7_e1543_d_n8: f64 = (eq7_e1541_d_n8 * (nv12 - 0.0));
        let eq7_e1543_d_n9: f64 = (eq7_e1541_d_n9 * (nv12 - 0.0));
        let eq7_e1543_d_n10: f64 = (eq7_e1541_d_n10 * (nv12 - 0.0));
        let eq7_e1543_d_n11: f64 = (eq7_e1541_d_n11 * (nv12 - 0.0));
        let eq7_e1544: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq7_e1543);
        (eq7_e1544, (eq7_e1543_d_n3 * ddt_scale), (eq7_e1543_d_n4 * ddt_scale), (eq7_e1543_d_n5 * ddt_scale), (eq7_e1543_d_n6 * ddt_scale), (eq7_e1543_d_n7 * ddt_scale), (eq7_e1543_d_n8 * ddt_scale), (eq7_e1543_d_n9 * ddt_scale), (eq7_e1543_d_n10 * ddt_scale), (eq7_e1543_d_n11 * ddt_scale), (eq7_e1541 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1546;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(12),
            None,
            multiplicity * (eq7_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq7_e1546_d_n3), multiplicity * (eq7_e1546_d_n4), multiplicity * (eq7_e1546_d_n5), multiplicity * (eq7_e1546_d_n6), multiplicity * (eq7_e1546_d_n7), multiplicity * (eq7_e1546_d_n8), multiplicity * (eq7_e1546_d_n9), multiplicity * (eq7_e1546_d_n10), multiplicity * (eq7_e1546_d_n11), multiplicity * (eq7_e1546_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq9_e1574, eq9_e1574_d_n3, eq9_e1574_d_n4, eq9_e1574_d_n5, eq9_e1574_d_n6, eq9_e1574_d_n7, eq9_e1574_d_n8, eq9_e1574_d_n9, eq9_e1574_d_n10, eq9_e1574_d_n11, eq9_e1574_d_n13,) = {
    if ((locals.var_guard492 != 0.0) && ((locals.var_guard666 != 0.0) && (locals.var_guard665 == 0.0))) {
        let eq9_e1572: f64 = (locals.var_sqid * (nv13 - 0.0));
        let eq9_e1572_d_n3: f64 = (locals.var_sqid_dn3 * (nv13 - 0.0));
        let eq9_e1572_d_n4: f64 = (locals.var_sqid_dn4 * (nv13 - 0.0));
        let eq9_e1572_d_n5: f64 = (locals.var_sqid_dn5 * (nv13 - 0.0));
        let eq9_e1572_d_n6: f64 = (locals.var_sqid_dn6 * (nv13 - 0.0));
        let eq9_e1572_d_n7: f64 = (locals.var_sqid_dn7 * (nv13 - 0.0));
        let eq9_e1572_d_n8: f64 = (locals.var_sqid_dn8 * (nv13 - 0.0));
        let eq9_e1572_d_n9: f64 = (locals.var_sqid_dn9 * (nv13 - 0.0));
        let eq9_e1572_d_n10: f64 = (locals.var_sqid_dn10 * (nv13 - 0.0));
        let eq9_e1572_d_n11: f64 = (locals.var_sqid_dn11 * (nv13 - 0.0));
        (eq9_e1572, eq9_e1572_d_n3, eq9_e1572_d_n4, eq9_e1572_d_n5, eq9_e1572_d_n6, eq9_e1572_d_n7, eq9_e1572_d_n8, eq9_e1572_d_n9, eq9_e1572_d_n10, eq9_e1572_d_n11, locals.var_sqid,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e1574;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq9_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 13],
            [multiplicity * (eq9_e1574_d_n3), multiplicity * (eq9_e1574_d_n4), multiplicity * (eq9_e1574_d_n5), multiplicity * (eq9_e1574_d_n6), multiplicity * (eq9_e1574_d_n7), multiplicity * (eq9_e1574_d_n8), multiplicity * (eq9_e1574_d_n9), multiplicity * (eq9_e1574_d_n10), multiplicity * (eq9_e1574_d_n11), multiplicity * (eq9_e1574_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq10_e1600, eq10_e1600_d_n3, eq10_e1600_d_n4, eq10_e1600_d_n5, eq10_e1600_d_n6, eq10_e1600_d_n7, eq10_e1600_d_n8, eq10_e1600_d_n9, eq10_e1600_d_n10, eq10_e1600_d_n11, eq10_e1600_d_n12,) = {
    if ((locals.var_guard492 != 0.0) && ((locals.var_guard666 != 0.0) && (locals.var_guard665 == 0.0))) {
        let eq10_e1584: f64 = (1.0 + locals.var_sigvds);
        let eq10_e1586: f64 = (eq10_e1584 * locals.var_mig);
        let eq10_e1586_d_n3: f64 = (eq10_e1584 * locals.var_mig_dn3);
        let eq10_e1586_d_n4: f64 = (eq10_e1584 * locals.var_mig_dn4);
        let eq10_e1586_d_n5: f64 = (eq10_e1584 * locals.var_mig_dn5);
        let eq10_e1586_d_n6: f64 = (eq10_e1584 * locals.var_mig_dn6);
        let eq10_e1586_d_n7: f64 = (eq10_e1584 * locals.var_mig_dn7);
        let eq10_e1586_d_n8: f64 = (eq10_e1584 * locals.var_mig_dn8);
        let eq10_e1586_d_n9: f64 = (eq10_e1584 * locals.var_mig_dn9);
        let eq10_e1586_d_n10: f64 = (eq10_e1584 * locals.var_mig_dn10);
        let eq10_e1586_d_n11: f64 = (eq10_e1584 * locals.var_mig_dn11);
        let eq10_e1588: f64 = (eq10_e1586 * locals.var_cox);
        let eq10_e1588_d_n3: f64 = (eq10_e1586_d_n3 * locals.var_cox);
        let eq10_e1588_d_n4: f64 = (eq10_e1586_d_n4 * locals.var_cox);
        let eq10_e1588_d_n5: f64 = (eq10_e1586_d_n5 * locals.var_cox);
        let eq10_e1588_d_n6: f64 = (eq10_e1586_d_n6 * locals.var_cox);
        let eq10_e1588_d_n7: f64 = (eq10_e1586_d_n7 * locals.var_cox);
        let eq10_e1588_d_n8: f64 = (eq10_e1586_d_n8 * locals.var_cox);
        let eq10_e1588_d_n9: f64 = (eq10_e1586_d_n9 * locals.var_cox);
        let eq10_e1588_d_n10: f64 = (eq10_e1586_d_n10 * locals.var_cox);
        let eq10_e1588_d_n11: f64 = (eq10_e1586_d_n11 * locals.var_cox);
        let eq10_e1590: f64 = (eq10_e1588 * locals.var_weff);
        let eq10_e1590_d_n3: f64 = (eq10_e1588_d_n3 * locals.var_weff);
        let eq10_e1590_d_n4: f64 = (eq10_e1588_d_n4 * locals.var_weff);
        let eq10_e1590_d_n5: f64 = (eq10_e1588_d_n5 * locals.var_weff);
        let eq10_e1590_d_n6: f64 = (eq10_e1588_d_n6 * locals.var_weff);
        let eq10_e1590_d_n7: f64 = (eq10_e1588_d_n7 * locals.var_weff);
        let eq10_e1590_d_n8: f64 = (eq10_e1588_d_n8 * locals.var_weff);
        let eq10_e1590_d_n9: f64 = (eq10_e1588_d_n9 * locals.var_weff);
        let eq10_e1590_d_n10: f64 = (eq10_e1588_d_n10 * locals.var_weff);
        let eq10_e1590_d_n11: f64 = (eq10_e1588_d_n11 * locals.var_weff);
        let eq10_e1592: f64 = (eq10_e1590 * p.p2);
        let eq10_e1592_d_n3: f64 = (eq10_e1590_d_n3 * p.p2);
        let eq10_e1592_d_n4: f64 = (eq10_e1590_d_n4 * p.p2);
        let eq10_e1592_d_n5: f64 = (eq10_e1590_d_n5 * p.p2);
        let eq10_e1592_d_n6: f64 = (eq10_e1590_d_n6 * p.p2);
        let eq10_e1592_d_n7: f64 = (eq10_e1590_d_n7 * p.p2);
        let eq10_e1592_d_n8: f64 = (eq10_e1590_d_n8 * p.p2);
        let eq10_e1592_d_n9: f64 = (eq10_e1590_d_n9 * p.p2);
        let eq10_e1592_d_n10: f64 = (eq10_e1590_d_n10 * p.p2);
        let eq10_e1592_d_n11: f64 = (eq10_e1590_d_n11 * p.p2);
        let eq10_e1594: f64 = (eq10_e1592 * locals.var_leff);
        let eq10_e1594_d_n3: f64 = (eq10_e1592_d_n3 * locals.var_leff);
        let eq10_e1594_d_n4: f64 = (eq10_e1592_d_n4 * locals.var_leff);
        let eq10_e1594_d_n5: f64 = (eq10_e1592_d_n5 * locals.var_leff);
        let eq10_e1594_d_n6: f64 = (eq10_e1592_d_n6 * locals.var_leff);
        let eq10_e1594_d_n7: f64 = (eq10_e1592_d_n7 * locals.var_leff);
        let eq10_e1594_d_n8: f64 = (eq10_e1592_d_n8 * locals.var_leff);
        let eq10_e1594_d_n9: f64 = (eq10_e1592_d_n9 * locals.var_leff);
        let eq10_e1594_d_n10: f64 = (eq10_e1592_d_n10 * locals.var_leff);
        let eq10_e1594_d_n11: f64 = (eq10_e1592_d_n11 * locals.var_leff);
        let eq10_e1596: f64 = (eq10_e1594 * (nv12 - 0.0));
        let eq10_e1596_d_n3: f64 = (eq10_e1594_d_n3 * (nv12 - 0.0));
        let eq10_e1596_d_n4: f64 = (eq10_e1594_d_n4 * (nv12 - 0.0));
        let eq10_e1596_d_n5: f64 = (eq10_e1594_d_n5 * (nv12 - 0.0));
        let eq10_e1596_d_n6: f64 = (eq10_e1594_d_n6 * (nv12 - 0.0));
        let eq10_e1596_d_n7: f64 = (eq10_e1594_d_n7 * (nv12 - 0.0));
        let eq10_e1596_d_n8: f64 = (eq10_e1594_d_n8 * (nv12 - 0.0));
        let eq10_e1596_d_n9: f64 = (eq10_e1594_d_n9 * (nv12 - 0.0));
        let eq10_e1596_d_n10: f64 = (eq10_e1594_d_n10 * (nv12 - 0.0));
        let eq10_e1596_d_n11: f64 = (eq10_e1594_d_n11 * (nv12 - 0.0));
        let eq10_e1597: f64 = (0.5 * eq10_e1596);
        let eq10_e1597_d_n3: f64 = (0.5 * eq10_e1596_d_n3);
        let eq10_e1597_d_n4: f64 = (0.5 * eq10_e1596_d_n4);
        let eq10_e1597_d_n5: f64 = (0.5 * eq10_e1596_d_n5);
        let eq10_e1597_d_n6: f64 = (0.5 * eq10_e1596_d_n6);
        let eq10_e1597_d_n7: f64 = (0.5 * eq10_e1596_d_n7);
        let eq10_e1597_d_n8: f64 = (0.5 * eq10_e1596_d_n8);
        let eq10_e1597_d_n9: f64 = (0.5 * eq10_e1596_d_n9);
        let eq10_e1597_d_n10: f64 = (0.5 * eq10_e1596_d_n10);
        let eq10_e1597_d_n11: f64 = (0.5 * eq10_e1596_d_n11);
        let eq10_e1597_d_n12: f64 = (0.5 * eq10_e1594);
        let eq10_e1598: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq10_e1597);
        (eq10_e1598, (eq10_e1597_d_n3 * ddt_scale), (eq10_e1597_d_n4 * ddt_scale), (eq10_e1597_d_n5 * ddt_scale), (eq10_e1597_d_n6 * ddt_scale), (eq10_e1597_d_n7 * ddt_scale), (eq10_e1597_d_n8 * ddt_scale), (eq10_e1597_d_n9 * ddt_scale), (eq10_e1597_d_n10 * ddt_scale), (eq10_e1597_d_n11 * ddt_scale), (eq10_e1597_d_n12 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e1600;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(7),
            multiplicity * (eq10_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq10_e1600_d_n3), multiplicity * (eq10_e1600_d_n4), multiplicity * (eq10_e1600_d_n5), multiplicity * (eq10_e1600_d_n6), multiplicity * (eq10_e1600_d_n7), multiplicity * (eq10_e1600_d_n8), multiplicity * (eq10_e1600_d_n9), multiplicity * (eq10_e1600_d_n10), multiplicity * (eq10_e1600_d_n11), multiplicity * (eq10_e1600_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq11_e1626, eq11_e1626_d_n3, eq11_e1626_d_n4, eq11_e1626_d_n5, eq11_e1626_d_n6, eq11_e1626_d_n7, eq11_e1626_d_n8, eq11_e1626_d_n9, eq11_e1626_d_n10, eq11_e1626_d_n11, eq11_e1626_d_n12,) = {
    if ((locals.var_guard492 != 0.0) && ((locals.var_guard666 != 0.0) && (locals.var_guard665 == 0.0))) {
        let eq11_e1610: f64 = (1.0 - locals.var_sigvds);
        let eq11_e1612: f64 = (eq11_e1610 * locals.var_mig);
        let eq11_e1612_d_n3: f64 = (eq11_e1610 * locals.var_mig_dn3);
        let eq11_e1612_d_n4: f64 = (eq11_e1610 * locals.var_mig_dn4);
        let eq11_e1612_d_n5: f64 = (eq11_e1610 * locals.var_mig_dn5);
        let eq11_e1612_d_n6: f64 = (eq11_e1610 * locals.var_mig_dn6);
        let eq11_e1612_d_n7: f64 = (eq11_e1610 * locals.var_mig_dn7);
        let eq11_e1612_d_n8: f64 = (eq11_e1610 * locals.var_mig_dn8);
        let eq11_e1612_d_n9: f64 = (eq11_e1610 * locals.var_mig_dn9);
        let eq11_e1612_d_n10: f64 = (eq11_e1610 * locals.var_mig_dn10);
        let eq11_e1612_d_n11: f64 = (eq11_e1610 * locals.var_mig_dn11);
        let eq11_e1614: f64 = (eq11_e1612 * locals.var_cox);
        let eq11_e1614_d_n3: f64 = (eq11_e1612_d_n3 * locals.var_cox);
        let eq11_e1614_d_n4: f64 = (eq11_e1612_d_n4 * locals.var_cox);
        let eq11_e1614_d_n5: f64 = (eq11_e1612_d_n5 * locals.var_cox);
        let eq11_e1614_d_n6: f64 = (eq11_e1612_d_n6 * locals.var_cox);
        let eq11_e1614_d_n7: f64 = (eq11_e1612_d_n7 * locals.var_cox);
        let eq11_e1614_d_n8: f64 = (eq11_e1612_d_n8 * locals.var_cox);
        let eq11_e1614_d_n9: f64 = (eq11_e1612_d_n9 * locals.var_cox);
        let eq11_e1614_d_n10: f64 = (eq11_e1612_d_n10 * locals.var_cox);
        let eq11_e1614_d_n11: f64 = (eq11_e1612_d_n11 * locals.var_cox);
        let eq11_e1616: f64 = (eq11_e1614 * locals.var_weff);
        let eq11_e1616_d_n3: f64 = (eq11_e1614_d_n3 * locals.var_weff);
        let eq11_e1616_d_n4: f64 = (eq11_e1614_d_n4 * locals.var_weff);
        let eq11_e1616_d_n5: f64 = (eq11_e1614_d_n5 * locals.var_weff);
        let eq11_e1616_d_n6: f64 = (eq11_e1614_d_n6 * locals.var_weff);
        let eq11_e1616_d_n7: f64 = (eq11_e1614_d_n7 * locals.var_weff);
        let eq11_e1616_d_n8: f64 = (eq11_e1614_d_n8 * locals.var_weff);
        let eq11_e1616_d_n9: f64 = (eq11_e1614_d_n9 * locals.var_weff);
        let eq11_e1616_d_n10: f64 = (eq11_e1614_d_n10 * locals.var_weff);
        let eq11_e1616_d_n11: f64 = (eq11_e1614_d_n11 * locals.var_weff);
        let eq11_e1618: f64 = (eq11_e1616 * p.p2);
        let eq11_e1618_d_n3: f64 = (eq11_e1616_d_n3 * p.p2);
        let eq11_e1618_d_n4: f64 = (eq11_e1616_d_n4 * p.p2);
        let eq11_e1618_d_n5: f64 = (eq11_e1616_d_n5 * p.p2);
        let eq11_e1618_d_n6: f64 = (eq11_e1616_d_n6 * p.p2);
        let eq11_e1618_d_n7: f64 = (eq11_e1616_d_n7 * p.p2);
        let eq11_e1618_d_n8: f64 = (eq11_e1616_d_n8 * p.p2);
        let eq11_e1618_d_n9: f64 = (eq11_e1616_d_n9 * p.p2);
        let eq11_e1618_d_n10: f64 = (eq11_e1616_d_n10 * p.p2);
        let eq11_e1618_d_n11: f64 = (eq11_e1616_d_n11 * p.p2);
        let eq11_e1620: f64 = (eq11_e1618 * locals.var_leff);
        let eq11_e1620_d_n3: f64 = (eq11_e1618_d_n3 * locals.var_leff);
        let eq11_e1620_d_n4: f64 = (eq11_e1618_d_n4 * locals.var_leff);
        let eq11_e1620_d_n5: f64 = (eq11_e1618_d_n5 * locals.var_leff);
        let eq11_e1620_d_n6: f64 = (eq11_e1618_d_n6 * locals.var_leff);
        let eq11_e1620_d_n7: f64 = (eq11_e1618_d_n7 * locals.var_leff);
        let eq11_e1620_d_n8: f64 = (eq11_e1618_d_n8 * locals.var_leff);
        let eq11_e1620_d_n9: f64 = (eq11_e1618_d_n9 * locals.var_leff);
        let eq11_e1620_d_n10: f64 = (eq11_e1618_d_n10 * locals.var_leff);
        let eq11_e1620_d_n11: f64 = (eq11_e1618_d_n11 * locals.var_leff);
        let eq11_e1622: f64 = (eq11_e1620 * (nv12 - 0.0));
        let eq11_e1622_d_n3: f64 = (eq11_e1620_d_n3 * (nv12 - 0.0));
        let eq11_e1622_d_n4: f64 = (eq11_e1620_d_n4 * (nv12 - 0.0));
        let eq11_e1622_d_n5: f64 = (eq11_e1620_d_n5 * (nv12 - 0.0));
        let eq11_e1622_d_n6: f64 = (eq11_e1620_d_n6 * (nv12 - 0.0));
        let eq11_e1622_d_n7: f64 = (eq11_e1620_d_n7 * (nv12 - 0.0));
        let eq11_e1622_d_n8: f64 = (eq11_e1620_d_n8 * (nv12 - 0.0));
        let eq11_e1622_d_n9: f64 = (eq11_e1620_d_n9 * (nv12 - 0.0));
        let eq11_e1622_d_n10: f64 = (eq11_e1620_d_n10 * (nv12 - 0.0));
        let eq11_e1622_d_n11: f64 = (eq11_e1620_d_n11 * (nv12 - 0.0));
        let eq11_e1623: f64 = (0.5 * eq11_e1622);
        let eq11_e1623_d_n3: f64 = (0.5 * eq11_e1622_d_n3);
        let eq11_e1623_d_n4: f64 = (0.5 * eq11_e1622_d_n4);
        let eq11_e1623_d_n5: f64 = (0.5 * eq11_e1622_d_n5);
        let eq11_e1623_d_n6: f64 = (0.5 * eq11_e1622_d_n6);
        let eq11_e1623_d_n7: f64 = (0.5 * eq11_e1622_d_n7);
        let eq11_e1623_d_n8: f64 = (0.5 * eq11_e1622_d_n8);
        let eq11_e1623_d_n9: f64 = (0.5 * eq11_e1622_d_n9);
        let eq11_e1623_d_n10: f64 = (0.5 * eq11_e1622_d_n10);
        let eq11_e1623_d_n11: f64 = (0.5 * eq11_e1622_d_n11);
        let eq11_e1623_d_n12: f64 = (0.5 * eq11_e1620);
        let eq11_e1624: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq11_e1623);
        (eq11_e1624, (eq11_e1623_d_n3 * ddt_scale), (eq11_e1623_d_n4 * ddt_scale), (eq11_e1623_d_n5 * ddt_scale), (eq11_e1623_d_n6 * ddt_scale), (eq11_e1623_d_n7 * ddt_scale), (eq11_e1623_d_n8 * ddt_scale), (eq11_e1623_d_n9 * ddt_scale), (eq11_e1623_d_n10 * ddt_scale), (eq11_e1623_d_n11 * ddt_scale), (eq11_e1623_d_n12 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e1626;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq11_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq11_e1626_d_n3), multiplicity * (eq11_e1626_d_n4), multiplicity * (eq11_e1626_d_n5), multiplicity * (eq11_e1626_d_n6), multiplicity * (eq11_e1626_d_n7), multiplicity * (eq11_e1626_d_n8), multiplicity * (eq11_e1626_d_n9), multiplicity * (eq11_e1626_d_n10), multiplicity * (eq11_e1626_d_n11), multiplicity * (eq11_e1626_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq24_e1784, eq24_e1784_d_n3, eq24_e1784_d_n4, eq24_e1784_d_n5, eq24_e1784_d_n6, eq24_e1784_d_n7, eq24_e1784_d_n8, eq24_e1784_d_n9, eq24_e1784_d_n10, eq24_e1784_d_n11, eq24_e1784_d_n12,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let eq24_e1773: f64 = (locals.var_mig * locals.var_cox);
        let eq24_e1773_d_n3: f64 = (locals.var_mig_dn3 * locals.var_cox);
        let eq24_e1773_d_n4: f64 = (locals.var_mig_dn4 * locals.var_cox);
        let eq24_e1773_d_n5: f64 = (locals.var_mig_dn5 * locals.var_cox);
        let eq24_e1773_d_n6: f64 = (locals.var_mig_dn6 * locals.var_cox);
        let eq24_e1773_d_n7: f64 = (locals.var_mig_dn7 * locals.var_cox);
        let eq24_e1773_d_n8: f64 = (locals.var_mig_dn8 * locals.var_cox);
        let eq24_e1773_d_n9: f64 = (locals.var_mig_dn9 * locals.var_cox);
        let eq24_e1773_d_n10: f64 = (locals.var_mig_dn10 * locals.var_cox);
        let eq24_e1773_d_n11: f64 = (locals.var_mig_dn11 * locals.var_cox);
        let eq24_e1775: f64 = (eq24_e1773 * locals.var_weff);
        let eq24_e1775_d_n3: f64 = (eq24_e1773_d_n3 * locals.var_weff);
        let eq24_e1775_d_n4: f64 = (eq24_e1773_d_n4 * locals.var_weff);
        let eq24_e1775_d_n5: f64 = (eq24_e1773_d_n5 * locals.var_weff);
        let eq24_e1775_d_n6: f64 = (eq24_e1773_d_n6 * locals.var_weff);
        let eq24_e1775_d_n7: f64 = (eq24_e1773_d_n7 * locals.var_weff);
        let eq24_e1775_d_n8: f64 = (eq24_e1773_d_n8 * locals.var_weff);
        let eq24_e1775_d_n9: f64 = (eq24_e1773_d_n9 * locals.var_weff);
        let eq24_e1775_d_n10: f64 = (eq24_e1773_d_n10 * locals.var_weff);
        let eq24_e1775_d_n11: f64 = (eq24_e1773_d_n11 * locals.var_weff);
        let eq24_e1777: f64 = (eq24_e1775 * p.p2);
        let eq24_e1777_d_n3: f64 = (eq24_e1775_d_n3 * p.p2);
        let eq24_e1777_d_n4: f64 = (eq24_e1775_d_n4 * p.p2);
        let eq24_e1777_d_n5: f64 = (eq24_e1775_d_n5 * p.p2);
        let eq24_e1777_d_n6: f64 = (eq24_e1775_d_n6 * p.p2);
        let eq24_e1777_d_n7: f64 = (eq24_e1775_d_n7 * p.p2);
        let eq24_e1777_d_n8: f64 = (eq24_e1775_d_n8 * p.p2);
        let eq24_e1777_d_n9: f64 = (eq24_e1775_d_n9 * p.p2);
        let eq24_e1777_d_n10: f64 = (eq24_e1775_d_n10 * p.p2);
        let eq24_e1777_d_n11: f64 = (eq24_e1775_d_n11 * p.p2);
        let eq24_e1779: f64 = (eq24_e1777 * locals.var_leff);
        let eq24_e1779_d_n3: f64 = (eq24_e1777_d_n3 * locals.var_leff);
        let eq24_e1779_d_n4: f64 = (eq24_e1777_d_n4 * locals.var_leff);
        let eq24_e1779_d_n5: f64 = (eq24_e1777_d_n5 * locals.var_leff);
        let eq24_e1779_d_n6: f64 = (eq24_e1777_d_n6 * locals.var_leff);
        let eq24_e1779_d_n7: f64 = (eq24_e1777_d_n7 * locals.var_leff);
        let eq24_e1779_d_n8: f64 = (eq24_e1777_d_n8 * locals.var_leff);
        let eq24_e1779_d_n9: f64 = (eq24_e1777_d_n9 * locals.var_leff);
        let eq24_e1779_d_n10: f64 = (eq24_e1777_d_n10 * locals.var_leff);
        let eq24_e1779_d_n11: f64 = (eq24_e1777_d_n11 * locals.var_leff);
        let eq24_e1781: f64 = (eq24_e1779 * (nv12 - 0.0));
        let eq24_e1781_d_n3: f64 = (eq24_e1779_d_n3 * (nv12 - 0.0));
        let eq24_e1781_d_n4: f64 = (eq24_e1779_d_n4 * (nv12 - 0.0));
        let eq24_e1781_d_n5: f64 = (eq24_e1779_d_n5 * (nv12 - 0.0));
        let eq24_e1781_d_n6: f64 = (eq24_e1779_d_n6 * (nv12 - 0.0));
        let eq24_e1781_d_n7: f64 = (eq24_e1779_d_n7 * (nv12 - 0.0));
        let eq24_e1781_d_n8: f64 = (eq24_e1779_d_n8 * (nv12 - 0.0));
        let eq24_e1781_d_n9: f64 = (eq24_e1779_d_n9 * (nv12 - 0.0));
        let eq24_e1781_d_n10: f64 = (eq24_e1779_d_n10 * (nv12 - 0.0));
        let eq24_e1781_d_n11: f64 = (eq24_e1779_d_n11 * (nv12 - 0.0));
        let eq24_e1782: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq24_e1781);
        (eq24_e1782, (eq24_e1781_d_n3 * ddt_scale), (eq24_e1781_d_n4 * ddt_scale), (eq24_e1781_d_n5 * ddt_scale), (eq24_e1781_d_n6 * ddt_scale), (eq24_e1781_d_n7 * ddt_scale), (eq24_e1781_d_n8 * ddt_scale), (eq24_e1781_d_n9 * ddt_scale), (eq24_e1781_d_n10 * ddt_scale), (eq24_e1781_d_n11 * ddt_scale), (eq24_e1779 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1784;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(12),
            None,
            multiplicity * (eq24_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq24_e1784_d_n3), multiplicity * (eq24_e1784_d_n4), multiplicity * (eq24_e1784_d_n5), multiplicity * (eq24_e1784_d_n6), multiplicity * (eq24_e1784_d_n7), multiplicity * (eq24_e1784_d_n8), multiplicity * (eq24_e1784_d_n9), multiplicity * (eq24_e1784_d_n10), multiplicity * (eq24_e1784_d_n11), multiplicity * (eq24_e1784_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq27_e1841, eq27_e1841_d_n3, eq27_e1841_d_n4, eq27_e1841_d_n5, eq27_e1841_d_n6, eq27_e1841_d_n7, eq27_e1841_d_n8, eq27_e1841_d_n9, eq27_e1841_d_n10, eq27_e1841_d_n11, eq27_e1841_d_n12,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let eq27_e1825: f64 = (1.0 + locals.var_sigvds);
        let eq27_e1827: f64 = (eq27_e1825 * locals.var_mig);
        let eq27_e1827_d_n3: f64 = (eq27_e1825 * locals.var_mig_dn3);
        let eq27_e1827_d_n4: f64 = (eq27_e1825 * locals.var_mig_dn4);
        let eq27_e1827_d_n5: f64 = (eq27_e1825 * locals.var_mig_dn5);
        let eq27_e1827_d_n6: f64 = (eq27_e1825 * locals.var_mig_dn6);
        let eq27_e1827_d_n7: f64 = (eq27_e1825 * locals.var_mig_dn7);
        let eq27_e1827_d_n8: f64 = (eq27_e1825 * locals.var_mig_dn8);
        let eq27_e1827_d_n9: f64 = (eq27_e1825 * locals.var_mig_dn9);
        let eq27_e1827_d_n10: f64 = (eq27_e1825 * locals.var_mig_dn10);
        let eq27_e1827_d_n11: f64 = (eq27_e1825 * locals.var_mig_dn11);
        let eq27_e1829: f64 = (eq27_e1827 * locals.var_cox);
        let eq27_e1829_d_n3: f64 = (eq27_e1827_d_n3 * locals.var_cox);
        let eq27_e1829_d_n4: f64 = (eq27_e1827_d_n4 * locals.var_cox);
        let eq27_e1829_d_n5: f64 = (eq27_e1827_d_n5 * locals.var_cox);
        let eq27_e1829_d_n6: f64 = (eq27_e1827_d_n6 * locals.var_cox);
        let eq27_e1829_d_n7: f64 = (eq27_e1827_d_n7 * locals.var_cox);
        let eq27_e1829_d_n8: f64 = (eq27_e1827_d_n8 * locals.var_cox);
        let eq27_e1829_d_n9: f64 = (eq27_e1827_d_n9 * locals.var_cox);
        let eq27_e1829_d_n10: f64 = (eq27_e1827_d_n10 * locals.var_cox);
        let eq27_e1829_d_n11: f64 = (eq27_e1827_d_n11 * locals.var_cox);
        let eq27_e1831: f64 = (eq27_e1829 * locals.var_weff);
        let eq27_e1831_d_n3: f64 = (eq27_e1829_d_n3 * locals.var_weff);
        let eq27_e1831_d_n4: f64 = (eq27_e1829_d_n4 * locals.var_weff);
        let eq27_e1831_d_n5: f64 = (eq27_e1829_d_n5 * locals.var_weff);
        let eq27_e1831_d_n6: f64 = (eq27_e1829_d_n6 * locals.var_weff);
        let eq27_e1831_d_n7: f64 = (eq27_e1829_d_n7 * locals.var_weff);
        let eq27_e1831_d_n8: f64 = (eq27_e1829_d_n8 * locals.var_weff);
        let eq27_e1831_d_n9: f64 = (eq27_e1829_d_n9 * locals.var_weff);
        let eq27_e1831_d_n10: f64 = (eq27_e1829_d_n10 * locals.var_weff);
        let eq27_e1831_d_n11: f64 = (eq27_e1829_d_n11 * locals.var_weff);
        let eq27_e1833: f64 = (eq27_e1831 * p.p2);
        let eq27_e1833_d_n3: f64 = (eq27_e1831_d_n3 * p.p2);
        let eq27_e1833_d_n4: f64 = (eq27_e1831_d_n4 * p.p2);
        let eq27_e1833_d_n5: f64 = (eq27_e1831_d_n5 * p.p2);
        let eq27_e1833_d_n6: f64 = (eq27_e1831_d_n6 * p.p2);
        let eq27_e1833_d_n7: f64 = (eq27_e1831_d_n7 * p.p2);
        let eq27_e1833_d_n8: f64 = (eq27_e1831_d_n8 * p.p2);
        let eq27_e1833_d_n9: f64 = (eq27_e1831_d_n9 * p.p2);
        let eq27_e1833_d_n10: f64 = (eq27_e1831_d_n10 * p.p2);
        let eq27_e1833_d_n11: f64 = (eq27_e1831_d_n11 * p.p2);
        let eq27_e1835: f64 = (eq27_e1833 * locals.var_leff);
        let eq27_e1835_d_n3: f64 = (eq27_e1833_d_n3 * locals.var_leff);
        let eq27_e1835_d_n4: f64 = (eq27_e1833_d_n4 * locals.var_leff);
        let eq27_e1835_d_n5: f64 = (eq27_e1833_d_n5 * locals.var_leff);
        let eq27_e1835_d_n6: f64 = (eq27_e1833_d_n6 * locals.var_leff);
        let eq27_e1835_d_n7: f64 = (eq27_e1833_d_n7 * locals.var_leff);
        let eq27_e1835_d_n8: f64 = (eq27_e1833_d_n8 * locals.var_leff);
        let eq27_e1835_d_n9: f64 = (eq27_e1833_d_n9 * locals.var_leff);
        let eq27_e1835_d_n10: f64 = (eq27_e1833_d_n10 * locals.var_leff);
        let eq27_e1835_d_n11: f64 = (eq27_e1833_d_n11 * locals.var_leff);
        let eq27_e1837: f64 = (eq27_e1835 * (nv12 - 0.0));
        let eq27_e1837_d_n3: f64 = (eq27_e1835_d_n3 * (nv12 - 0.0));
        let eq27_e1837_d_n4: f64 = (eq27_e1835_d_n4 * (nv12 - 0.0));
        let eq27_e1837_d_n5: f64 = (eq27_e1835_d_n5 * (nv12 - 0.0));
        let eq27_e1837_d_n6: f64 = (eq27_e1835_d_n6 * (nv12 - 0.0));
        let eq27_e1837_d_n7: f64 = (eq27_e1835_d_n7 * (nv12 - 0.0));
        let eq27_e1837_d_n8: f64 = (eq27_e1835_d_n8 * (nv12 - 0.0));
        let eq27_e1837_d_n9: f64 = (eq27_e1835_d_n9 * (nv12 - 0.0));
        let eq27_e1837_d_n10: f64 = (eq27_e1835_d_n10 * (nv12 - 0.0));
        let eq27_e1837_d_n11: f64 = (eq27_e1835_d_n11 * (nv12 - 0.0));
        let eq27_e1838: f64 = (0.5 * eq27_e1837);
        let eq27_e1838_d_n3: f64 = (0.5 * eq27_e1837_d_n3);
        let eq27_e1838_d_n4: f64 = (0.5 * eq27_e1837_d_n4);
        let eq27_e1838_d_n5: f64 = (0.5 * eq27_e1837_d_n5);
        let eq27_e1838_d_n6: f64 = (0.5 * eq27_e1837_d_n6);
        let eq27_e1838_d_n7: f64 = (0.5 * eq27_e1837_d_n7);
        let eq27_e1838_d_n8: f64 = (0.5 * eq27_e1837_d_n8);
        let eq27_e1838_d_n9: f64 = (0.5 * eq27_e1837_d_n9);
        let eq27_e1838_d_n10: f64 = (0.5 * eq27_e1837_d_n10);
        let eq27_e1838_d_n11: f64 = (0.5 * eq27_e1837_d_n11);
        let eq27_e1838_d_n12: f64 = (0.5 * eq27_e1835);
        let eq27_e1839: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq27_e1838);
        (eq27_e1839, (eq27_e1838_d_n3 * ddt_scale), (eq27_e1838_d_n4 * ddt_scale), (eq27_e1838_d_n5 * ddt_scale), (eq27_e1838_d_n6 * ddt_scale), (eq27_e1838_d_n7 * ddt_scale), (eq27_e1838_d_n8 * ddt_scale), (eq27_e1838_d_n9 * ddt_scale), (eq27_e1838_d_n10 * ddt_scale), (eq27_e1838_d_n11 * ddt_scale), (eq27_e1838_d_n12 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1841;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(7),
            multiplicity * (eq27_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq27_e1841_d_n3), multiplicity * (eq27_e1841_d_n4), multiplicity * (eq27_e1841_d_n5), multiplicity * (eq27_e1841_d_n6), multiplicity * (eq27_e1841_d_n7), multiplicity * (eq27_e1841_d_n8), multiplicity * (eq27_e1841_d_n9), multiplicity * (eq27_e1841_d_n10), multiplicity * (eq27_e1841_d_n11), multiplicity * (eq27_e1841_d_n12)],
            [],
            [],
            1.0,
        );
    }

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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq28_e1868, eq28_e1868_d_n3, eq28_e1868_d_n4, eq28_e1868_d_n5, eq28_e1868_d_n6, eq28_e1868_d_n7, eq28_e1868_d_n8, eq28_e1868_d_n9, eq28_e1868_d_n10, eq28_e1868_d_n11, eq28_e1868_d_n12,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let eq28_e1852: f64 = (1.0 - locals.var_sigvds);
        let eq28_e1854: f64 = (eq28_e1852 * locals.var_mig);
        let eq28_e1854_d_n3: f64 = (eq28_e1852 * locals.var_mig_dn3);
        let eq28_e1854_d_n4: f64 = (eq28_e1852 * locals.var_mig_dn4);
        let eq28_e1854_d_n5: f64 = (eq28_e1852 * locals.var_mig_dn5);
        let eq28_e1854_d_n6: f64 = (eq28_e1852 * locals.var_mig_dn6);
        let eq28_e1854_d_n7: f64 = (eq28_e1852 * locals.var_mig_dn7);
        let eq28_e1854_d_n8: f64 = (eq28_e1852 * locals.var_mig_dn8);
        let eq28_e1854_d_n9: f64 = (eq28_e1852 * locals.var_mig_dn9);
        let eq28_e1854_d_n10: f64 = (eq28_e1852 * locals.var_mig_dn10);
        let eq28_e1854_d_n11: f64 = (eq28_e1852 * locals.var_mig_dn11);
        let eq28_e1856: f64 = (eq28_e1854 * locals.var_cox);
        let eq28_e1856_d_n3: f64 = (eq28_e1854_d_n3 * locals.var_cox);
        let eq28_e1856_d_n4: f64 = (eq28_e1854_d_n4 * locals.var_cox);
        let eq28_e1856_d_n5: f64 = (eq28_e1854_d_n5 * locals.var_cox);
        let eq28_e1856_d_n6: f64 = (eq28_e1854_d_n6 * locals.var_cox);
        let eq28_e1856_d_n7: f64 = (eq28_e1854_d_n7 * locals.var_cox);
        let eq28_e1856_d_n8: f64 = (eq28_e1854_d_n8 * locals.var_cox);
        let eq28_e1856_d_n9: f64 = (eq28_e1854_d_n9 * locals.var_cox);
        let eq28_e1856_d_n10: f64 = (eq28_e1854_d_n10 * locals.var_cox);
        let eq28_e1856_d_n11: f64 = (eq28_e1854_d_n11 * locals.var_cox);
        let eq28_e1858: f64 = (eq28_e1856 * locals.var_weff);
        let eq28_e1858_d_n3: f64 = (eq28_e1856_d_n3 * locals.var_weff);
        let eq28_e1858_d_n4: f64 = (eq28_e1856_d_n4 * locals.var_weff);
        let eq28_e1858_d_n5: f64 = (eq28_e1856_d_n5 * locals.var_weff);
        let eq28_e1858_d_n6: f64 = (eq28_e1856_d_n6 * locals.var_weff);
        let eq28_e1858_d_n7: f64 = (eq28_e1856_d_n7 * locals.var_weff);
        let eq28_e1858_d_n8: f64 = (eq28_e1856_d_n8 * locals.var_weff);
        let eq28_e1858_d_n9: f64 = (eq28_e1856_d_n9 * locals.var_weff);
        let eq28_e1858_d_n10: f64 = (eq28_e1856_d_n10 * locals.var_weff);
        let eq28_e1858_d_n11: f64 = (eq28_e1856_d_n11 * locals.var_weff);
        let eq28_e1860: f64 = (eq28_e1858 * p.p2);
        let eq28_e1860_d_n3: f64 = (eq28_e1858_d_n3 * p.p2);
        let eq28_e1860_d_n4: f64 = (eq28_e1858_d_n4 * p.p2);
        let eq28_e1860_d_n5: f64 = (eq28_e1858_d_n5 * p.p2);
        let eq28_e1860_d_n6: f64 = (eq28_e1858_d_n6 * p.p2);
        let eq28_e1860_d_n7: f64 = (eq28_e1858_d_n7 * p.p2);
        let eq28_e1860_d_n8: f64 = (eq28_e1858_d_n8 * p.p2);
        let eq28_e1860_d_n9: f64 = (eq28_e1858_d_n9 * p.p2);
        let eq28_e1860_d_n10: f64 = (eq28_e1858_d_n10 * p.p2);
        let eq28_e1860_d_n11: f64 = (eq28_e1858_d_n11 * p.p2);
        let eq28_e1862: f64 = (eq28_e1860 * locals.var_leff);
        let eq28_e1862_d_n3: f64 = (eq28_e1860_d_n3 * locals.var_leff);
        let eq28_e1862_d_n4: f64 = (eq28_e1860_d_n4 * locals.var_leff);
        let eq28_e1862_d_n5: f64 = (eq28_e1860_d_n5 * locals.var_leff);
        let eq28_e1862_d_n6: f64 = (eq28_e1860_d_n6 * locals.var_leff);
        let eq28_e1862_d_n7: f64 = (eq28_e1860_d_n7 * locals.var_leff);
        let eq28_e1862_d_n8: f64 = (eq28_e1860_d_n8 * locals.var_leff);
        let eq28_e1862_d_n9: f64 = (eq28_e1860_d_n9 * locals.var_leff);
        let eq28_e1862_d_n10: f64 = (eq28_e1860_d_n10 * locals.var_leff);
        let eq28_e1862_d_n11: f64 = (eq28_e1860_d_n11 * locals.var_leff);
        let eq28_e1864: f64 = (eq28_e1862 * (nv12 - 0.0));
        let eq28_e1864_d_n3: f64 = (eq28_e1862_d_n3 * (nv12 - 0.0));
        let eq28_e1864_d_n4: f64 = (eq28_e1862_d_n4 * (nv12 - 0.0));
        let eq28_e1864_d_n5: f64 = (eq28_e1862_d_n5 * (nv12 - 0.0));
        let eq28_e1864_d_n6: f64 = (eq28_e1862_d_n6 * (nv12 - 0.0));
        let eq28_e1864_d_n7: f64 = (eq28_e1862_d_n7 * (nv12 - 0.0));
        let eq28_e1864_d_n8: f64 = (eq28_e1862_d_n8 * (nv12 - 0.0));
        let eq28_e1864_d_n9: f64 = (eq28_e1862_d_n9 * (nv12 - 0.0));
        let eq28_e1864_d_n10: f64 = (eq28_e1862_d_n10 * (nv12 - 0.0));
        let eq28_e1864_d_n11: f64 = (eq28_e1862_d_n11 * (nv12 - 0.0));
        let eq28_e1865: f64 = (0.5 * eq28_e1864);
        let eq28_e1865_d_n3: f64 = (0.5 * eq28_e1864_d_n3);
        let eq28_e1865_d_n4: f64 = (0.5 * eq28_e1864_d_n4);
        let eq28_e1865_d_n5: f64 = (0.5 * eq28_e1864_d_n5);
        let eq28_e1865_d_n6: f64 = (0.5 * eq28_e1864_d_n6);
        let eq28_e1865_d_n7: f64 = (0.5 * eq28_e1864_d_n7);
        let eq28_e1865_d_n8: f64 = (0.5 * eq28_e1864_d_n8);
        let eq28_e1865_d_n9: f64 = (0.5 * eq28_e1864_d_n9);
        let eq28_e1865_d_n10: f64 = (0.5 * eq28_e1864_d_n10);
        let eq28_e1865_d_n11: f64 = (0.5 * eq28_e1864_d_n11);
        let eq28_e1865_d_n12: f64 = (0.5 * eq28_e1862);
        let eq28_e1866: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq28_e1865);
        (eq28_e1866, (eq28_e1865_d_n3 * ddt_scale), (eq28_e1865_d_n4 * ddt_scale), (eq28_e1865_d_n5 * ddt_scale), (eq28_e1865_d_n6 * ddt_scale), (eq28_e1865_d_n7 * ddt_scale), (eq28_e1865_d_n8 * ddt_scale), (eq28_e1865_d_n9 * ddt_scale), (eq28_e1865_d_n10 * ddt_scale), (eq28_e1865_d_n11 * ddt_scale), (eq28_e1865_d_n12 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e1868;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq28_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq28_e1868_d_n3), multiplicity * (eq28_e1868_d_n4), multiplicity * (eq28_e1868_d_n5), multiplicity * (eq28_e1868_d_n6), multiplicity * (eq28_e1868_d_n7), multiplicity * (eq28_e1868_d_n8), multiplicity * (eq28_e1868_d_n9), multiplicity * (eq28_e1868_d_n10), multiplicity * (eq28_e1868_d_n11), multiplicity * (eq28_e1868_d_n12)],
            [],
            [],
            1.0,
        );
        let eq45_e1969: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, locals.var_qsub);
        let eq45_e1970: f64 = (locals.var_devsign * eq45_e1969);
        let eq45_e1970_d_n3: f64 = (locals.var_devsign * (locals.var_qsub_dn3 * ddt_scale));
        let eq45_e1970_d_n4: f64 = (locals.var_devsign * (locals.var_qsub_dn4 * ddt_scale));
        let eq45_e1970_d_n5: f64 = (locals.var_devsign * (locals.var_qsub_dn5 * ddt_scale));
        let eq45_e1970_d_n6: f64 = (locals.var_devsign * (locals.var_qsub_dn6 * ddt_scale));
        let eq45_e1970_d_n7: f64 = (locals.var_devsign * (locals.var_qsub_dn7 * ddt_scale));
        let eq45_e1970_d_n8: f64 = (locals.var_devsign * (locals.var_qsub_dn8 * ddt_scale));
        let eq45_e1970_d_n9: f64 = (locals.var_devsign * (locals.var_qsub_dn9 * ddt_scale));
        let eq45_e1970_d_n10: f64 = (locals.var_devsign * (locals.var_qsub_dn10 * ddt_scale));
        let eq45_e1970_d_n11: f64 = (locals.var_devsign * (locals.var_qsub_dn11 * ddt_scale));
        let eq45_value: f64 = eq45_e1970;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(3),
            Some(10),
            multiplicity * (eq45_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq45_e1970_d_n3), multiplicity * (eq45_e1970_d_n4), multiplicity * (eq45_e1970_d_n5), multiplicity * (eq45_e1970_d_n6), multiplicity * (eq45_e1970_d_n7), multiplicity * (eq45_e1970_d_n8), multiplicity * (eq45_e1970_d_n9), multiplicity * (eq45_e1970_d_n10), multiplicity * (eq45_e1970_d_n11)],
            [],
            [],
            1.0,
        );
        let eq46_e1972: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, locals.var_qde);
        let eq46_value: f64 = eq46_e1972;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(3),
            multiplicity * (eq46_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * ((locals.var_qde_dn3 * ddt_scale)), multiplicity * ((locals.var_qde_dn4 * ddt_scale)), multiplicity * ((locals.var_qde_dn5 * ddt_scale)), multiplicity * ((locals.var_qde_dn6 * ddt_scale)), multiplicity * ((locals.var_qde_dn7 * ddt_scale)), multiplicity * ((locals.var_qde_dn8 * ddt_scale)), multiplicity * ((locals.var_qde_dn9 * ddt_scale)), multiplicity * ((locals.var_qde_dn10 * ddt_scale)), multiplicity * ((locals.var_qde_dn11 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq47_e1974: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, locals.var_qse);
        let eq47_value: f64 = eq47_e1974;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(7),
            Some(3),
            multiplicity * (eq47_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * ((locals.var_qse_dn3 * ddt_scale)), multiplicity * ((locals.var_qse_dn4 * ddt_scale)), multiplicity * ((locals.var_qse_dn5 * ddt_scale)), multiplicity * ((locals.var_qse_dn6 * ddt_scale)), multiplicity * ((locals.var_qse_dn7 * ddt_scale)), multiplicity * ((locals.var_qse_dn8 * ddt_scale)), multiplicity * ((locals.var_qse_dn9 * ddt_scale)), multiplicity * ((locals.var_qse_dn10 * ddt_scale)), multiplicity * ((locals.var_qse_dn11 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let (eq59_e2043, eq59_e2043_d_n0, eq59_e2043_d_n3, eq59_e2043_d_n4, eq59_e2043_d_n5, eq59_e2043_d_n6, eq59_e2043_d_n7, eq59_e2043_d_n8, eq59_e2043_d_n9, eq59_e2043_d_n10, eq59_e2043_d_n11,) = {
    if (locals.var_guard888 != 0.0) {
        let eq59_e2041: f64 = ((nv0 - nv6) * locals.var_gdpr);
        let eq59_e2041_d_n3: f64 = ((nv0 - nv6) * locals.var_gdpr_dn3);
        let eq59_e2041_d_n4: f64 = ((nv0 - nv6) * locals.var_gdpr_dn4);
        let eq59_e2041_d_n5: f64 = ((nv0 - nv6) * locals.var_gdpr_dn5);
        let eq59_e2041_d_n6: f64 = ((-locals.var_gdpr) + ((nv0 - nv6) * locals.var_gdpr_dn6));
        let eq59_e2041_d_n7: f64 = ((nv0 - nv6) * locals.var_gdpr_dn7);
        let eq59_e2041_d_n8: f64 = ((nv0 - nv6) * locals.var_gdpr_dn8);
        let eq59_e2041_d_n9: f64 = ((nv0 - nv6) * locals.var_gdpr_dn9);
        let eq59_e2041_d_n10: f64 = ((nv0 - nv6) * locals.var_gdpr_dn10);
        let eq59_e2041_d_n11: f64 = ((nv0 - nv6) * locals.var_gdpr_dn11);
        (eq59_e2041, locals.var_gdpr, eq59_e2041_d_n3, eq59_e2041_d_n4, eq59_e2041_d_n5, eq59_e2041_d_n6, eq59_e2041_d_n7, eq59_e2041_d_n8, eq59_e2041_d_n9, eq59_e2041_d_n10, eq59_e2041_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e2043;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(0),
            Some(6),
            multiplicity * (eq59_value),
            [0, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq59_e2043_d_n0), multiplicity * (eq59_e2043_d_n3), multiplicity * (eq59_e2043_d_n4), multiplicity * (eq59_e2043_d_n5), multiplicity * (eq59_e2043_d_n6), multiplicity * (eq59_e2043_d_n7), multiplicity * (eq59_e2043_d_n8), multiplicity * (eq59_e2043_d_n9), multiplicity * (eq59_e2043_d_n10), multiplicity * (eq59_e2043_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq60_e2048,) = {
    if (locals.var_guard888 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq60_value: f64 = eq60_e2048;
        stamper.stamp_potential_const_local(
            3,
            eq60_value,
        );
        let (eq62_e2062, eq62_e2062_d_n2, eq62_e2062_d_n3, eq62_e2062_d_n4, eq62_e2062_d_n5, eq62_e2062_d_n6, eq62_e2062_d_n7, eq62_e2062_d_n8, eq62_e2062_d_n9, eq62_e2062_d_n10, eq62_e2062_d_n11,) = {
    if (locals.var_guard890 != 0.0) {
        let eq62_e2060: f64 = ((nv2 - nv7) * locals.var_gspr);
        let eq62_e2060_d_n3: f64 = ((nv2 - nv7) * locals.var_gspr_dn3);
        let eq62_e2060_d_n4: f64 = ((nv2 - nv7) * locals.var_gspr_dn4);
        let eq62_e2060_d_n5: f64 = ((nv2 - nv7) * locals.var_gspr_dn5);
        let eq62_e2060_d_n6: f64 = ((nv2 - nv7) * locals.var_gspr_dn6);
        let eq62_e2060_d_n7: f64 = ((-locals.var_gspr) + ((nv2 - nv7) * locals.var_gspr_dn7));
        let eq62_e2060_d_n8: f64 = ((nv2 - nv7) * locals.var_gspr_dn8);
        let eq62_e2060_d_n9: f64 = ((nv2 - nv7) * locals.var_gspr_dn9);
        let eq62_e2060_d_n10: f64 = ((nv2 - nv7) * locals.var_gspr_dn10);
        let eq62_e2060_d_n11: f64 = ((nv2 - nv7) * locals.var_gspr_dn11);
        (eq62_e2060, locals.var_gspr, eq62_e2060_d_n3, eq62_e2060_d_n4, eq62_e2060_d_n5, eq62_e2060_d_n6, eq62_e2060_d_n7, eq62_e2060_d_n8, eq62_e2060_d_n9, eq62_e2060_d_n10, eq62_e2060_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e2062;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(2),
            Some(7),
            multiplicity * (eq62_value),
            [2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq62_e2062_d_n2), multiplicity * (eq62_e2062_d_n3), multiplicity * (eq62_e2062_d_n4), multiplicity * (eq62_e2062_d_n5), multiplicity * (eq62_e2062_d_n6), multiplicity * (eq62_e2062_d_n7), multiplicity * (eq62_e2062_d_n8), multiplicity * (eq62_e2062_d_n9), multiplicity * (eq62_e2062_d_n10), multiplicity * (eq62_e2062_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq63_e2067,) = {
    if (locals.var_guard890 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq63_value: f64 = eq63_e2067;
        stamper.stamp_potential_const_local(
            4,
            eq63_value,
        );
        let (eq67_e2103, eq67_e2103_d_n0, eq67_e2103_d_n2, eq67_e2103_d_n3, eq67_e2103_d_n4, eq67_e2103_d_n5, eq67_e2103_d_n6, eq67_e2103_d_n7, eq67_e2103_d_n8, eq67_e2103_d_n9, eq67_e2103_d_n10, eq67_e2103_d_n11,) = {
    if (((locals.var_guard893 != 0.0) && (locals.var_guard896 != 0.0)) && (locals.var_guard897 != 0.0)) {
        let eq67_e2094: f64 = (locals.var_deltemp1 * locals.var_gth);
        let eq67_e2094_d_n4: f64 = (locals.var_deltemp1_dn4 * locals.var_gth);
        let eq67_e2094_d_n5: f64 = (locals.var_deltemp1_dn5 * locals.var_gth);
        let eq67_e2097: f64 = (locals.var_deltemp1 * locals.var_cth);
        let eq67_e2097_d_n4: f64 = (locals.var_deltemp1_dn4 * locals.var_cth);
        let eq67_e2097_d_n5: f64 = (locals.var_deltemp1_dn5 * locals.var_cth);
        let eq67_e2098: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq67_e2097);
        let eq67_e2099: f64 = (eq67_e2094 + eq67_e2098);
        let eq67_e2099_d_n4: f64 = (eq67_e2094_d_n4 + (eq67_e2097_d_n4 * ddt_scale));
        let eq67_e2099_d_n5: f64 = (eq67_e2094_d_n5 + (eq67_e2097_d_n5 * ddt_scale));
        let eq67_e2101: f64 = (eq67_e2099 - locals.var_pdiss);
        let eq67_e2101_d_n4: f64 = (eq67_e2099_d_n4 - locals.var_pdiss_dn4);
        let eq67_e2101_d_n5: f64 = (eq67_e2099_d_n5 - locals.var_pdiss_dn5);
        (eq67_e2101, (-locals.var_pdiss_dn0), (-locals.var_pdiss_dn2), (-locals.var_pdiss_dn3), eq67_e2101_d_n4, eq67_e2101_d_n5, (-locals.var_pdiss_dn6), (-locals.var_pdiss_dn7), (-locals.var_pdiss_dn8), (-locals.var_pdiss_dn9), (-locals.var_pdiss_dn10), (-locals.var_pdiss_dn11),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e2103;
        let eq67_node_derivative_indices: [usize; 11] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq67_node_derivatives: [f64; 11] = [eq67_e2103_d_n0, eq67_e2103_d_n2, eq67_e2103_d_n3, eq67_e2103_d_n4, eq67_e2103_d_n5, eq67_e2103_d_n6, eq67_e2103_d_n7, eq67_e2103_d_n8, eq67_e2103_d_n9, eq67_e2103_d_n10, eq67_e2103_d_n11];
        let eq67_branch_derivative_indices: [usize; 0] = [];
        let eq67_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq67_value),
            &eq67_node_derivative_indices,
            &eq67_node_derivatives,
            &eq67_branch_derivative_indices,
            &eq67_branch_derivatives,
            multiplicity,
        );
        let (eq68_e2121, eq68_e2121_d_n0, eq68_e2121_d_n2, eq68_e2121_d_n3, eq68_e2121_d_n4, eq68_e2121_d_n5, eq68_e2121_d_n6, eq68_e2121_d_n7, eq68_e2121_d_n8, eq68_e2121_d_n9, eq68_e2121_d_n10, eq68_e2121_d_n11,) = {
    if (((locals.var_guard893 != 0.0) && (locals.var_guard896 != 0.0)) && (locals.var_guard897 == 0.0)) {
        let eq68_e2112: f64 = (locals.var_deltemp1 * locals.var_gth);
        let eq68_e2112_d_n4: f64 = (locals.var_deltemp1_dn4 * locals.var_gth);
        let eq68_e2112_d_n5: f64 = (locals.var_deltemp1_dn5 * locals.var_gth);
        let eq68_e2115: f64 = (locals.var_deltemp1 * locals.var_cth);
        let eq68_e2115_d_n4: f64 = (locals.var_deltemp1_dn4 * locals.var_cth);
        let eq68_e2115_d_n5: f64 = (locals.var_deltemp1_dn5 * locals.var_cth);
        let eq68_e2116: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, eq68_e2115);
        let eq68_e2117: f64 = (eq68_e2112 + eq68_e2116);
        let eq68_e2117_d_n4: f64 = (eq68_e2112_d_n4 + (eq68_e2115_d_n4 * ddt_scale));
        let eq68_e2117_d_n5: f64 = (eq68_e2112_d_n5 + (eq68_e2115_d_n5 * ddt_scale));
        let eq68_e2119: f64 = (eq68_e2117 - locals.var_pdiss);
        let eq68_e2119_d_n4: f64 = (eq68_e2117_d_n4 - locals.var_pdiss_dn4);
        let eq68_e2119_d_n5: f64 = (eq68_e2117_d_n5 - locals.var_pdiss_dn5);
        (eq68_e2119, (-locals.var_pdiss_dn0), (-locals.var_pdiss_dn2), (-locals.var_pdiss_dn3), eq68_e2119_d_n4, eq68_e2119_d_n5, (-locals.var_pdiss_dn6), (-locals.var_pdiss_dn7), (-locals.var_pdiss_dn8), (-locals.var_pdiss_dn9), (-locals.var_pdiss_dn10), (-locals.var_pdiss_dn11),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq68_value: f64 = eq68_e2121;
        let eq68_node_derivative_indices: [usize; 11] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq68_node_derivatives: [f64; 11] = [eq68_e2121_d_n0, eq68_e2121_d_n2, eq68_e2121_d_n3, eq68_e2121_d_n4, eq68_e2121_d_n5, eq68_e2121_d_n6, eq68_e2121_d_n7, eq68_e2121_d_n8, eq68_e2121_d_n9, eq68_e2121_d_n10, eq68_e2121_d_n11];
        let eq68_branch_derivative_indices: [usize; 0] = [];
        let eq68_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            None,
            multiplicity * (eq68_value),
            &eq68_node_derivative_indices,
            &eq68_node_derivatives,
            &eq68_branch_derivative_indices,
            &eq68_branch_derivatives,
            multiplicity,
        );
        let (eq69_e2137, eq69_e2137_d_n0, eq69_e2137_d_n2, eq69_e2137_d_n3, eq69_e2137_d_n4, eq69_e2137_d_n5, eq69_e2137_d_n6, eq69_e2137_d_n7, eq69_e2137_d_n8, eq69_e2137_d_n9, eq69_e2137_d_n10, eq69_e2137_d_n11,) = {
    if ((locals.var_guard893 != 0.0) && (locals.var_guard896 == 0.0)) {
        let eq69_e2128: f64 = (locals.var_deltemp1 * locals.var_gth);
        let eq69_e2128_d_n4: f64 = (locals.var_deltemp1_dn4 * locals.var_gth);
        let eq69_e2128_d_n5: f64 = (locals.var_deltemp1_dn5 * locals.var_gth);
        let eq69_e2131: f64 = (locals.var_deltemp1 * locals.var_cth);
        let eq69_e2131_d_n4: f64 = (locals.var_deltemp1_dn4 * locals.var_cth);
        let eq69_e2131_d_n5: f64 = (locals.var_deltemp1_dn5 * locals.var_cth);
        let eq69_e2132: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 20, eq69_e2131);
        let eq69_e2133: f64 = (eq69_e2128 + eq69_e2132);
        let eq69_e2133_d_n4: f64 = (eq69_e2128_d_n4 + (eq69_e2131_d_n4 * ddt_scale));
        let eq69_e2133_d_n5: f64 = (eq69_e2128_d_n5 + (eq69_e2131_d_n5 * ddt_scale));
        let eq69_e2135: f64 = (eq69_e2133 - locals.var_pdiss);
        let eq69_e2135_d_n4: f64 = (eq69_e2133_d_n4 - locals.var_pdiss_dn4);
        let eq69_e2135_d_n5: f64 = (eq69_e2133_d_n5 - locals.var_pdiss_dn5);
        (eq69_e2135, (-locals.var_pdiss_dn0), (-locals.var_pdiss_dn2), (-locals.var_pdiss_dn3), eq69_e2135_d_n4, eq69_e2135_d_n5, (-locals.var_pdiss_dn6), (-locals.var_pdiss_dn7), (-locals.var_pdiss_dn8), (-locals.var_pdiss_dn9), (-locals.var_pdiss_dn10), (-locals.var_pdiss_dn11),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e2137;
        let eq69_node_derivative_indices: [usize; 11] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq69_node_derivatives: [f64; 11] = [eq69_e2137_d_n0, eq69_e2137_d_n2, eq69_e2137_d_n3, eq69_e2137_d_n4, eq69_e2137_d_n5, eq69_e2137_d_n6, eq69_e2137_d_n7, eq69_e2137_d_n8, eq69_e2137_d_n9, eq69_e2137_d_n10, eq69_e2137_d_n11];
        let eq69_branch_derivative_indices: [usize; 0] = [];
        let eq69_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            None,
            multiplicity * (eq69_value),
            &eq69_node_derivative_indices,
            &eq69_node_derivatives,
            &eq69_branch_derivative_indices,
            &eq69_branch_derivatives,
            multiplicity,
        );
        let eq80_e2212: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 21, locals.var_qbsj);
        let eq80_e2213: f64 = (locals.var_devsign * eq80_e2212);
        let eq80_e2213_d_n3: f64 = (locals.var_devsign * (locals.var_qbsj_dn3 * ddt_scale));
        let eq80_e2213_d_n4: f64 = (locals.var_devsign * (locals.var_qbsj_dn4 * ddt_scale));
        let eq80_e2213_d_n5: f64 = (locals.var_devsign * (locals.var_qbsj_dn5 * ddt_scale));
        let eq80_e2213_d_n6: f64 = (locals.var_devsign * (locals.var_qbsj_dn6 * ddt_scale));
        let eq80_e2213_d_n7: f64 = (locals.var_devsign * (locals.var_qbsj_dn7 * ddt_scale));
        let eq80_e2213_d_n8: f64 = (locals.var_devsign * (locals.var_qbsj_dn8 * ddt_scale));
        let eq80_e2213_d_n9: f64 = (locals.var_devsign * (locals.var_qbsj_dn9 * ddt_scale));
        let eq80_e2213_d_n10: f64 = (locals.var_devsign * (locals.var_qbsj_dn10 * ddt_scale));
        let eq80_e2213_d_n11: f64 = (locals.var_devsign * (locals.var_qbsj_dn11 * ddt_scale));
        let eq80_value: f64 = eq80_e2213;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(10),
            Some(7),
            multiplicity * (eq80_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq80_e2213_d_n3), multiplicity * (eq80_e2213_d_n4), multiplicity * (eq80_e2213_d_n5), multiplicity * (eq80_e2213_d_n6), multiplicity * (eq80_e2213_d_n7), multiplicity * (eq80_e2213_d_n8), multiplicity * (eq80_e2213_d_n9), multiplicity * (eq80_e2213_d_n10), multiplicity * (eq80_e2213_d_n11)],
            [],
            [],
            1.0,
        );
        let eq81_e2216: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 22, locals.var_qbdj);
        let eq81_e2217: f64 = (locals.var_devsign * eq81_e2216);
        let eq81_e2217_d_n3: f64 = (locals.var_devsign * (locals.var_qbdj_dn3 * ddt_scale));
        let eq81_e2217_d_n4: f64 = (locals.var_devsign * (locals.var_qbdj_dn4 * ddt_scale));
        let eq81_e2217_d_n5: f64 = (locals.var_devsign * (locals.var_qbdj_dn5 * ddt_scale));
        let eq81_e2217_d_n6: f64 = (locals.var_devsign * (locals.var_qbdj_dn6 * ddt_scale));
        let eq81_e2217_d_n7: f64 = (locals.var_devsign * (locals.var_qbdj_dn7 * ddt_scale));
        let eq81_e2217_d_n8: f64 = (locals.var_devsign * (locals.var_qbdj_dn8 * ddt_scale));
        let eq81_e2217_d_n9: f64 = (locals.var_devsign * (locals.var_qbdj_dn9 * ddt_scale));
        let eq81_e2217_d_n10: f64 = (locals.var_devsign * (locals.var_qbdj_dn10 * ddt_scale));
        let eq81_e2217_d_n11: f64 = (locals.var_devsign * (locals.var_qbdj_dn11 * ddt_scale));
        let eq81_value: f64 = eq81_e2217;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(10),
            Some(6),
            multiplicity * (eq81_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq81_e2217_d_n3), multiplicity * (eq81_e2217_d_n4), multiplicity * (eq81_e2217_d_n5), multiplicity * (eq81_e2217_d_n6), multiplicity * (eq81_e2217_d_n7), multiplicity * (eq81_e2217_d_n8), multiplicity * (eq81_e2217_d_n9), multiplicity * (eq81_e2217_d_n10), multiplicity * (eq81_e2217_d_n11)],
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq7_e1546, eq7_e1546_d_n3, eq7_e1546_d_n4, eq7_e1546_d_n5, eq7_e1546_d_n6, eq7_e1546_d_n7, eq7_e1546_d_n8, eq7_e1546_d_n9, eq7_e1546_d_n10, eq7_e1546_d_n11, eq7_e1546_d_n12, eq7_e1546_q,) = {
    if ((locals.var_guard492 != 0.0) && ((locals.var_guard666 != 0.0) && (locals.var_guard665 == 0.0))) {
        let eq7_e1535: f64 = (locals.var_mig * locals.var_cox);
        let eq7_e1535_d_n3: f64 = (locals.var_mig_dn3 * locals.var_cox);
        let eq7_e1535_d_n4: f64 = (locals.var_mig_dn4 * locals.var_cox);
        let eq7_e1535_d_n5: f64 = (locals.var_mig_dn5 * locals.var_cox);
        let eq7_e1535_d_n6: f64 = (locals.var_mig_dn6 * locals.var_cox);
        let eq7_e1535_d_n7: f64 = (locals.var_mig_dn7 * locals.var_cox);
        let eq7_e1535_d_n8: f64 = (locals.var_mig_dn8 * locals.var_cox);
        let eq7_e1535_d_n9: f64 = (locals.var_mig_dn9 * locals.var_cox);
        let eq7_e1535_d_n10: f64 = (locals.var_mig_dn10 * locals.var_cox);
        let eq7_e1535_d_n11: f64 = (locals.var_mig_dn11 * locals.var_cox);
        let eq7_e1537: f64 = (eq7_e1535 * locals.var_weff);
        let eq7_e1537_d_n3: f64 = (eq7_e1535_d_n3 * locals.var_weff);
        let eq7_e1537_d_n4: f64 = (eq7_e1535_d_n4 * locals.var_weff);
        let eq7_e1537_d_n5: f64 = (eq7_e1535_d_n5 * locals.var_weff);
        let eq7_e1537_d_n6: f64 = (eq7_e1535_d_n6 * locals.var_weff);
        let eq7_e1537_d_n7: f64 = (eq7_e1535_d_n7 * locals.var_weff);
        let eq7_e1537_d_n8: f64 = (eq7_e1535_d_n8 * locals.var_weff);
        let eq7_e1537_d_n9: f64 = (eq7_e1535_d_n9 * locals.var_weff);
        let eq7_e1537_d_n10: f64 = (eq7_e1535_d_n10 * locals.var_weff);
        let eq7_e1537_d_n11: f64 = (eq7_e1535_d_n11 * locals.var_weff);
        let eq7_e1539: f64 = (eq7_e1537 * p.p2);
        let eq7_e1539_d_n3: f64 = (eq7_e1537_d_n3 * p.p2);
        let eq7_e1539_d_n4: f64 = (eq7_e1537_d_n4 * p.p2);
        let eq7_e1539_d_n5: f64 = (eq7_e1537_d_n5 * p.p2);
        let eq7_e1539_d_n6: f64 = (eq7_e1537_d_n6 * p.p2);
        let eq7_e1539_d_n7: f64 = (eq7_e1537_d_n7 * p.p2);
        let eq7_e1539_d_n8: f64 = (eq7_e1537_d_n8 * p.p2);
        let eq7_e1539_d_n9: f64 = (eq7_e1537_d_n9 * p.p2);
        let eq7_e1539_d_n10: f64 = (eq7_e1537_d_n10 * p.p2);
        let eq7_e1539_d_n11: f64 = (eq7_e1537_d_n11 * p.p2);
        let eq7_e1541: f64 = (eq7_e1539 * locals.var_leff);
        let eq7_e1541_d_n3: f64 = (eq7_e1539_d_n3 * locals.var_leff);
        let eq7_e1541_d_n4: f64 = (eq7_e1539_d_n4 * locals.var_leff);
        let eq7_e1541_d_n5: f64 = (eq7_e1539_d_n5 * locals.var_leff);
        let eq7_e1541_d_n6: f64 = (eq7_e1539_d_n6 * locals.var_leff);
        let eq7_e1541_d_n7: f64 = (eq7_e1539_d_n7 * locals.var_leff);
        let eq7_e1541_d_n8: f64 = (eq7_e1539_d_n8 * locals.var_leff);
        let eq7_e1541_d_n9: f64 = (eq7_e1539_d_n9 * locals.var_leff);
        let eq7_e1541_d_n10: f64 = (eq7_e1539_d_n10 * locals.var_leff);
        let eq7_e1541_d_n11: f64 = (eq7_e1539_d_n11 * locals.var_leff);
        let eq7_e1543: f64 = (eq7_e1541 * (nv12 - 0.0));
        let eq7_e1543_d_n3: f64 = (eq7_e1541_d_n3 * (nv12 - 0.0));
        let eq7_e1543_d_n4: f64 = (eq7_e1541_d_n4 * (nv12 - 0.0));
        let eq7_e1543_d_n5: f64 = (eq7_e1541_d_n5 * (nv12 - 0.0));
        let eq7_e1543_d_n6: f64 = (eq7_e1541_d_n6 * (nv12 - 0.0));
        let eq7_e1543_d_n7: f64 = (eq7_e1541_d_n7 * (nv12 - 0.0));
        let eq7_e1543_d_n8: f64 = (eq7_e1541_d_n8 * (nv12 - 0.0));
        let eq7_e1543_d_n9: f64 = (eq7_e1541_d_n9 * (nv12 - 0.0));
        let eq7_e1543_d_n10: f64 = (eq7_e1541_d_n10 * (nv12 - 0.0));
        let eq7_e1543_d_n11: f64 = (eq7_e1541_d_n11 * (nv12 - 0.0));
        let eq7_e1544_q: f64 = eq7_e1543;
        (eq7_e1543, eq7_e1543_d_n3, eq7_e1543_d_n4, eq7_e1543_d_n5, eq7_e1543_d_n6, eq7_e1543_d_n7, eq7_e1543_d_n8, eq7_e1543_d_n9, eq7_e1543_d_n10, eq7_e1543_d_n11, eq7_e1541, eq7_e1544_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq7_e1546_d_n3, eq7_e1546_d_n4, eq7_e1546_d_n5, eq7_e1546_d_n6, eq7_e1546_d_n7, eq7_e1546_d_n8, eq7_e1546_d_n9, eq7_e1546_d_n10, eq7_e1546_d_n11, eq7_e1546_d_n12, 0.0];
        let eq7_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            None,
            nodes,
            &eq7_reactive_node_derivatives,
            branches,
            &eq7_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq10_e1600, eq10_e1600_d_n3, eq10_e1600_d_n4, eq10_e1600_d_n5, eq10_e1600_d_n6, eq10_e1600_d_n7, eq10_e1600_d_n8, eq10_e1600_d_n9, eq10_e1600_d_n10, eq10_e1600_d_n11, eq10_e1600_d_n12, eq10_e1600_q,) = {
    if ((locals.var_guard492 != 0.0) && ((locals.var_guard666 != 0.0) && (locals.var_guard665 == 0.0))) {
        let eq10_e1584: f64 = (1.0 + locals.var_sigvds);
        let eq10_e1586: f64 = (eq10_e1584 * locals.var_mig);
        let eq10_e1586_d_n3: f64 = (eq10_e1584 * locals.var_mig_dn3);
        let eq10_e1586_d_n4: f64 = (eq10_e1584 * locals.var_mig_dn4);
        let eq10_e1586_d_n5: f64 = (eq10_e1584 * locals.var_mig_dn5);
        let eq10_e1586_d_n6: f64 = (eq10_e1584 * locals.var_mig_dn6);
        let eq10_e1586_d_n7: f64 = (eq10_e1584 * locals.var_mig_dn7);
        let eq10_e1586_d_n8: f64 = (eq10_e1584 * locals.var_mig_dn8);
        let eq10_e1586_d_n9: f64 = (eq10_e1584 * locals.var_mig_dn9);
        let eq10_e1586_d_n10: f64 = (eq10_e1584 * locals.var_mig_dn10);
        let eq10_e1586_d_n11: f64 = (eq10_e1584 * locals.var_mig_dn11);
        let eq10_e1588: f64 = (eq10_e1586 * locals.var_cox);
        let eq10_e1588_d_n3: f64 = (eq10_e1586_d_n3 * locals.var_cox);
        let eq10_e1588_d_n4: f64 = (eq10_e1586_d_n4 * locals.var_cox);
        let eq10_e1588_d_n5: f64 = (eq10_e1586_d_n5 * locals.var_cox);
        let eq10_e1588_d_n6: f64 = (eq10_e1586_d_n6 * locals.var_cox);
        let eq10_e1588_d_n7: f64 = (eq10_e1586_d_n7 * locals.var_cox);
        let eq10_e1588_d_n8: f64 = (eq10_e1586_d_n8 * locals.var_cox);
        let eq10_e1588_d_n9: f64 = (eq10_e1586_d_n9 * locals.var_cox);
        let eq10_e1588_d_n10: f64 = (eq10_e1586_d_n10 * locals.var_cox);
        let eq10_e1588_d_n11: f64 = (eq10_e1586_d_n11 * locals.var_cox);
        let eq10_e1590: f64 = (eq10_e1588 * locals.var_weff);
        let eq10_e1590_d_n3: f64 = (eq10_e1588_d_n3 * locals.var_weff);
        let eq10_e1590_d_n4: f64 = (eq10_e1588_d_n4 * locals.var_weff);
        let eq10_e1590_d_n5: f64 = (eq10_e1588_d_n5 * locals.var_weff);
        let eq10_e1590_d_n6: f64 = (eq10_e1588_d_n6 * locals.var_weff);
        let eq10_e1590_d_n7: f64 = (eq10_e1588_d_n7 * locals.var_weff);
        let eq10_e1590_d_n8: f64 = (eq10_e1588_d_n8 * locals.var_weff);
        let eq10_e1590_d_n9: f64 = (eq10_e1588_d_n9 * locals.var_weff);
        let eq10_e1590_d_n10: f64 = (eq10_e1588_d_n10 * locals.var_weff);
        let eq10_e1590_d_n11: f64 = (eq10_e1588_d_n11 * locals.var_weff);
        let eq10_e1592: f64 = (eq10_e1590 * p.p2);
        let eq10_e1592_d_n3: f64 = (eq10_e1590_d_n3 * p.p2);
        let eq10_e1592_d_n4: f64 = (eq10_e1590_d_n4 * p.p2);
        let eq10_e1592_d_n5: f64 = (eq10_e1590_d_n5 * p.p2);
        let eq10_e1592_d_n6: f64 = (eq10_e1590_d_n6 * p.p2);
        let eq10_e1592_d_n7: f64 = (eq10_e1590_d_n7 * p.p2);
        let eq10_e1592_d_n8: f64 = (eq10_e1590_d_n8 * p.p2);
        let eq10_e1592_d_n9: f64 = (eq10_e1590_d_n9 * p.p2);
        let eq10_e1592_d_n10: f64 = (eq10_e1590_d_n10 * p.p2);
        let eq10_e1592_d_n11: f64 = (eq10_e1590_d_n11 * p.p2);
        let eq10_e1594: f64 = (eq10_e1592 * locals.var_leff);
        let eq10_e1594_d_n3: f64 = (eq10_e1592_d_n3 * locals.var_leff);
        let eq10_e1594_d_n4: f64 = (eq10_e1592_d_n4 * locals.var_leff);
        let eq10_e1594_d_n5: f64 = (eq10_e1592_d_n5 * locals.var_leff);
        let eq10_e1594_d_n6: f64 = (eq10_e1592_d_n6 * locals.var_leff);
        let eq10_e1594_d_n7: f64 = (eq10_e1592_d_n7 * locals.var_leff);
        let eq10_e1594_d_n8: f64 = (eq10_e1592_d_n8 * locals.var_leff);
        let eq10_e1594_d_n9: f64 = (eq10_e1592_d_n9 * locals.var_leff);
        let eq10_e1594_d_n10: f64 = (eq10_e1592_d_n10 * locals.var_leff);
        let eq10_e1594_d_n11: f64 = (eq10_e1592_d_n11 * locals.var_leff);
        let eq10_e1596: f64 = (eq10_e1594 * (nv12 - 0.0));
        let eq10_e1596_d_n3: f64 = (eq10_e1594_d_n3 * (nv12 - 0.0));
        let eq10_e1596_d_n4: f64 = (eq10_e1594_d_n4 * (nv12 - 0.0));
        let eq10_e1596_d_n5: f64 = (eq10_e1594_d_n5 * (nv12 - 0.0));
        let eq10_e1596_d_n6: f64 = (eq10_e1594_d_n6 * (nv12 - 0.0));
        let eq10_e1596_d_n7: f64 = (eq10_e1594_d_n7 * (nv12 - 0.0));
        let eq10_e1596_d_n8: f64 = (eq10_e1594_d_n8 * (nv12 - 0.0));
        let eq10_e1596_d_n9: f64 = (eq10_e1594_d_n9 * (nv12 - 0.0));
        let eq10_e1596_d_n10: f64 = (eq10_e1594_d_n10 * (nv12 - 0.0));
        let eq10_e1596_d_n11: f64 = (eq10_e1594_d_n11 * (nv12 - 0.0));
        let eq10_e1597: f64 = (0.5 * eq10_e1596);
        let eq10_e1597_d_n3: f64 = (0.5 * eq10_e1596_d_n3);
        let eq10_e1597_d_n4: f64 = (0.5 * eq10_e1596_d_n4);
        let eq10_e1597_d_n5: f64 = (0.5 * eq10_e1596_d_n5);
        let eq10_e1597_d_n6: f64 = (0.5 * eq10_e1596_d_n6);
        let eq10_e1597_d_n7: f64 = (0.5 * eq10_e1596_d_n7);
        let eq10_e1597_d_n8: f64 = (0.5 * eq10_e1596_d_n8);
        let eq10_e1597_d_n9: f64 = (0.5 * eq10_e1596_d_n9);
        let eq10_e1597_d_n10: f64 = (0.5 * eq10_e1596_d_n10);
        let eq10_e1597_d_n11: f64 = (0.5 * eq10_e1596_d_n11);
        let eq10_e1597_d_n12: f64 = (0.5 * eq10_e1594);
        let eq10_e1598_q: f64 = eq10_e1597;
        (eq10_e1597, eq10_e1597_d_n3, eq10_e1597_d_n4, eq10_e1597_d_n5, eq10_e1597_d_n6, eq10_e1597_d_n7, eq10_e1597_d_n8, eq10_e1597_d_n9, eq10_e1597_d_n10, eq10_e1597_d_n11, eq10_e1597_d_n12, eq10_e1598_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq10_e1600_d_n3, eq10_e1600_d_n4, eq10_e1600_d_n5, eq10_e1600_d_n6, eq10_e1600_d_n7, eq10_e1600_d_n8, eq10_e1600_d_n9, eq10_e1600_d_n10, eq10_e1600_d_n11, eq10_e1600_d_n12, 0.0];
        let eq10_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq11_e1626, eq11_e1626_d_n3, eq11_e1626_d_n4, eq11_e1626_d_n5, eq11_e1626_d_n6, eq11_e1626_d_n7, eq11_e1626_d_n8, eq11_e1626_d_n9, eq11_e1626_d_n10, eq11_e1626_d_n11, eq11_e1626_d_n12, eq11_e1626_q,) = {
    if ((locals.var_guard492 != 0.0) && ((locals.var_guard666 != 0.0) && (locals.var_guard665 == 0.0))) {
        let eq11_e1610: f64 = (1.0 - locals.var_sigvds);
        let eq11_e1612: f64 = (eq11_e1610 * locals.var_mig);
        let eq11_e1612_d_n3: f64 = (eq11_e1610 * locals.var_mig_dn3);
        let eq11_e1612_d_n4: f64 = (eq11_e1610 * locals.var_mig_dn4);
        let eq11_e1612_d_n5: f64 = (eq11_e1610 * locals.var_mig_dn5);
        let eq11_e1612_d_n6: f64 = (eq11_e1610 * locals.var_mig_dn6);
        let eq11_e1612_d_n7: f64 = (eq11_e1610 * locals.var_mig_dn7);
        let eq11_e1612_d_n8: f64 = (eq11_e1610 * locals.var_mig_dn8);
        let eq11_e1612_d_n9: f64 = (eq11_e1610 * locals.var_mig_dn9);
        let eq11_e1612_d_n10: f64 = (eq11_e1610 * locals.var_mig_dn10);
        let eq11_e1612_d_n11: f64 = (eq11_e1610 * locals.var_mig_dn11);
        let eq11_e1614: f64 = (eq11_e1612 * locals.var_cox);
        let eq11_e1614_d_n3: f64 = (eq11_e1612_d_n3 * locals.var_cox);
        let eq11_e1614_d_n4: f64 = (eq11_e1612_d_n4 * locals.var_cox);
        let eq11_e1614_d_n5: f64 = (eq11_e1612_d_n5 * locals.var_cox);
        let eq11_e1614_d_n6: f64 = (eq11_e1612_d_n6 * locals.var_cox);
        let eq11_e1614_d_n7: f64 = (eq11_e1612_d_n7 * locals.var_cox);
        let eq11_e1614_d_n8: f64 = (eq11_e1612_d_n8 * locals.var_cox);
        let eq11_e1614_d_n9: f64 = (eq11_e1612_d_n9 * locals.var_cox);
        let eq11_e1614_d_n10: f64 = (eq11_e1612_d_n10 * locals.var_cox);
        let eq11_e1614_d_n11: f64 = (eq11_e1612_d_n11 * locals.var_cox);
        let eq11_e1616: f64 = (eq11_e1614 * locals.var_weff);
        let eq11_e1616_d_n3: f64 = (eq11_e1614_d_n3 * locals.var_weff);
        let eq11_e1616_d_n4: f64 = (eq11_e1614_d_n4 * locals.var_weff);
        let eq11_e1616_d_n5: f64 = (eq11_e1614_d_n5 * locals.var_weff);
        let eq11_e1616_d_n6: f64 = (eq11_e1614_d_n6 * locals.var_weff);
        let eq11_e1616_d_n7: f64 = (eq11_e1614_d_n7 * locals.var_weff);
        let eq11_e1616_d_n8: f64 = (eq11_e1614_d_n8 * locals.var_weff);
        let eq11_e1616_d_n9: f64 = (eq11_e1614_d_n9 * locals.var_weff);
        let eq11_e1616_d_n10: f64 = (eq11_e1614_d_n10 * locals.var_weff);
        let eq11_e1616_d_n11: f64 = (eq11_e1614_d_n11 * locals.var_weff);
        let eq11_e1618: f64 = (eq11_e1616 * p.p2);
        let eq11_e1618_d_n3: f64 = (eq11_e1616_d_n3 * p.p2);
        let eq11_e1618_d_n4: f64 = (eq11_e1616_d_n4 * p.p2);
        let eq11_e1618_d_n5: f64 = (eq11_e1616_d_n5 * p.p2);
        let eq11_e1618_d_n6: f64 = (eq11_e1616_d_n6 * p.p2);
        let eq11_e1618_d_n7: f64 = (eq11_e1616_d_n7 * p.p2);
        let eq11_e1618_d_n8: f64 = (eq11_e1616_d_n8 * p.p2);
        let eq11_e1618_d_n9: f64 = (eq11_e1616_d_n9 * p.p2);
        let eq11_e1618_d_n10: f64 = (eq11_e1616_d_n10 * p.p2);
        let eq11_e1618_d_n11: f64 = (eq11_e1616_d_n11 * p.p2);
        let eq11_e1620: f64 = (eq11_e1618 * locals.var_leff);
        let eq11_e1620_d_n3: f64 = (eq11_e1618_d_n3 * locals.var_leff);
        let eq11_e1620_d_n4: f64 = (eq11_e1618_d_n4 * locals.var_leff);
        let eq11_e1620_d_n5: f64 = (eq11_e1618_d_n5 * locals.var_leff);
        let eq11_e1620_d_n6: f64 = (eq11_e1618_d_n6 * locals.var_leff);
        let eq11_e1620_d_n7: f64 = (eq11_e1618_d_n7 * locals.var_leff);
        let eq11_e1620_d_n8: f64 = (eq11_e1618_d_n8 * locals.var_leff);
        let eq11_e1620_d_n9: f64 = (eq11_e1618_d_n9 * locals.var_leff);
        let eq11_e1620_d_n10: f64 = (eq11_e1618_d_n10 * locals.var_leff);
        let eq11_e1620_d_n11: f64 = (eq11_e1618_d_n11 * locals.var_leff);
        let eq11_e1622: f64 = (eq11_e1620 * (nv12 - 0.0));
        let eq11_e1622_d_n3: f64 = (eq11_e1620_d_n3 * (nv12 - 0.0));
        let eq11_e1622_d_n4: f64 = (eq11_e1620_d_n4 * (nv12 - 0.0));
        let eq11_e1622_d_n5: f64 = (eq11_e1620_d_n5 * (nv12 - 0.0));
        let eq11_e1622_d_n6: f64 = (eq11_e1620_d_n6 * (nv12 - 0.0));
        let eq11_e1622_d_n7: f64 = (eq11_e1620_d_n7 * (nv12 - 0.0));
        let eq11_e1622_d_n8: f64 = (eq11_e1620_d_n8 * (nv12 - 0.0));
        let eq11_e1622_d_n9: f64 = (eq11_e1620_d_n9 * (nv12 - 0.0));
        let eq11_e1622_d_n10: f64 = (eq11_e1620_d_n10 * (nv12 - 0.0));
        let eq11_e1622_d_n11: f64 = (eq11_e1620_d_n11 * (nv12 - 0.0));
        let eq11_e1623: f64 = (0.5 * eq11_e1622);
        let eq11_e1623_d_n3: f64 = (0.5 * eq11_e1622_d_n3);
        let eq11_e1623_d_n4: f64 = (0.5 * eq11_e1622_d_n4);
        let eq11_e1623_d_n5: f64 = (0.5 * eq11_e1622_d_n5);
        let eq11_e1623_d_n6: f64 = (0.5 * eq11_e1622_d_n6);
        let eq11_e1623_d_n7: f64 = (0.5 * eq11_e1622_d_n7);
        let eq11_e1623_d_n8: f64 = (0.5 * eq11_e1622_d_n8);
        let eq11_e1623_d_n9: f64 = (0.5 * eq11_e1622_d_n9);
        let eq11_e1623_d_n10: f64 = (0.5 * eq11_e1622_d_n10);
        let eq11_e1623_d_n11: f64 = (0.5 * eq11_e1622_d_n11);
        let eq11_e1623_d_n12: f64 = (0.5 * eq11_e1620);
        let eq11_e1624_q: f64 = eq11_e1623;
        (eq11_e1623, eq11_e1623_d_n3, eq11_e1623_d_n4, eq11_e1623_d_n5, eq11_e1623_d_n6, eq11_e1623_d_n7, eq11_e1623_d_n8, eq11_e1623_d_n9, eq11_e1623_d_n10, eq11_e1623_d_n11, eq11_e1623_d_n12, eq11_e1624_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq11_e1626_d_n3, eq11_e1626_d_n4, eq11_e1626_d_n5, eq11_e1626_d_n6, eq11_e1626_d_n7, eq11_e1626_d_n8, eq11_e1626_d_n9, eq11_e1626_d_n10, eq11_e1626_d_n11, eq11_e1626_d_n12, 0.0];
        let eq11_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq24_e1784, eq24_e1784_d_n3, eq24_e1784_d_n4, eq24_e1784_d_n5, eq24_e1784_d_n6, eq24_e1784_d_n7, eq24_e1784_d_n8, eq24_e1784_d_n9, eq24_e1784_d_n10, eq24_e1784_d_n11, eq24_e1784_d_n12, eq24_e1784_q,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let eq24_e1773: f64 = (locals.var_mig * locals.var_cox);
        let eq24_e1773_d_n3: f64 = (locals.var_mig_dn3 * locals.var_cox);
        let eq24_e1773_d_n4: f64 = (locals.var_mig_dn4 * locals.var_cox);
        let eq24_e1773_d_n5: f64 = (locals.var_mig_dn5 * locals.var_cox);
        let eq24_e1773_d_n6: f64 = (locals.var_mig_dn6 * locals.var_cox);
        let eq24_e1773_d_n7: f64 = (locals.var_mig_dn7 * locals.var_cox);
        let eq24_e1773_d_n8: f64 = (locals.var_mig_dn8 * locals.var_cox);
        let eq24_e1773_d_n9: f64 = (locals.var_mig_dn9 * locals.var_cox);
        let eq24_e1773_d_n10: f64 = (locals.var_mig_dn10 * locals.var_cox);
        let eq24_e1773_d_n11: f64 = (locals.var_mig_dn11 * locals.var_cox);
        let eq24_e1775: f64 = (eq24_e1773 * locals.var_weff);
        let eq24_e1775_d_n3: f64 = (eq24_e1773_d_n3 * locals.var_weff);
        let eq24_e1775_d_n4: f64 = (eq24_e1773_d_n4 * locals.var_weff);
        let eq24_e1775_d_n5: f64 = (eq24_e1773_d_n5 * locals.var_weff);
        let eq24_e1775_d_n6: f64 = (eq24_e1773_d_n6 * locals.var_weff);
        let eq24_e1775_d_n7: f64 = (eq24_e1773_d_n7 * locals.var_weff);
        let eq24_e1775_d_n8: f64 = (eq24_e1773_d_n8 * locals.var_weff);
        let eq24_e1775_d_n9: f64 = (eq24_e1773_d_n9 * locals.var_weff);
        let eq24_e1775_d_n10: f64 = (eq24_e1773_d_n10 * locals.var_weff);
        let eq24_e1775_d_n11: f64 = (eq24_e1773_d_n11 * locals.var_weff);
        let eq24_e1777: f64 = (eq24_e1775 * p.p2);
        let eq24_e1777_d_n3: f64 = (eq24_e1775_d_n3 * p.p2);
        let eq24_e1777_d_n4: f64 = (eq24_e1775_d_n4 * p.p2);
        let eq24_e1777_d_n5: f64 = (eq24_e1775_d_n5 * p.p2);
        let eq24_e1777_d_n6: f64 = (eq24_e1775_d_n6 * p.p2);
        let eq24_e1777_d_n7: f64 = (eq24_e1775_d_n7 * p.p2);
        let eq24_e1777_d_n8: f64 = (eq24_e1775_d_n8 * p.p2);
        let eq24_e1777_d_n9: f64 = (eq24_e1775_d_n9 * p.p2);
        let eq24_e1777_d_n10: f64 = (eq24_e1775_d_n10 * p.p2);
        let eq24_e1777_d_n11: f64 = (eq24_e1775_d_n11 * p.p2);
        let eq24_e1779: f64 = (eq24_e1777 * locals.var_leff);
        let eq24_e1779_d_n3: f64 = (eq24_e1777_d_n3 * locals.var_leff);
        let eq24_e1779_d_n4: f64 = (eq24_e1777_d_n4 * locals.var_leff);
        let eq24_e1779_d_n5: f64 = (eq24_e1777_d_n5 * locals.var_leff);
        let eq24_e1779_d_n6: f64 = (eq24_e1777_d_n6 * locals.var_leff);
        let eq24_e1779_d_n7: f64 = (eq24_e1777_d_n7 * locals.var_leff);
        let eq24_e1779_d_n8: f64 = (eq24_e1777_d_n8 * locals.var_leff);
        let eq24_e1779_d_n9: f64 = (eq24_e1777_d_n9 * locals.var_leff);
        let eq24_e1779_d_n10: f64 = (eq24_e1777_d_n10 * locals.var_leff);
        let eq24_e1779_d_n11: f64 = (eq24_e1777_d_n11 * locals.var_leff);
        let eq24_e1781: f64 = (eq24_e1779 * (nv12 - 0.0));
        let eq24_e1781_d_n3: f64 = (eq24_e1779_d_n3 * (nv12 - 0.0));
        let eq24_e1781_d_n4: f64 = (eq24_e1779_d_n4 * (nv12 - 0.0));
        let eq24_e1781_d_n5: f64 = (eq24_e1779_d_n5 * (nv12 - 0.0));
        let eq24_e1781_d_n6: f64 = (eq24_e1779_d_n6 * (nv12 - 0.0));
        let eq24_e1781_d_n7: f64 = (eq24_e1779_d_n7 * (nv12 - 0.0));
        let eq24_e1781_d_n8: f64 = (eq24_e1779_d_n8 * (nv12 - 0.0));
        let eq24_e1781_d_n9: f64 = (eq24_e1779_d_n9 * (nv12 - 0.0));
        let eq24_e1781_d_n10: f64 = (eq24_e1779_d_n10 * (nv12 - 0.0));
        let eq24_e1781_d_n11: f64 = (eq24_e1779_d_n11 * (nv12 - 0.0));
        let eq24_e1782_q: f64 = eq24_e1781;
        (eq24_e1781, eq24_e1781_d_n3, eq24_e1781_d_n4, eq24_e1781_d_n5, eq24_e1781_d_n6, eq24_e1781_d_n7, eq24_e1781_d_n8, eq24_e1781_d_n9, eq24_e1781_d_n10, eq24_e1781_d_n11, eq24_e1779, eq24_e1782_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq24_e1784_d_n3, eq24_e1784_d_n4, eq24_e1784_d_n5, eq24_e1784_d_n6, eq24_e1784_d_n7, eq24_e1784_d_n8, eq24_e1784_d_n9, eq24_e1784_d_n10, eq24_e1784_d_n11, eq24_e1784_d_n12, 0.0];
        let eq24_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            None,
            nodes,
            &eq24_reactive_node_derivatives,
            branches,
            &eq24_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq27_e1841, eq27_e1841_d_n3, eq27_e1841_d_n4, eq27_e1841_d_n5, eq27_e1841_d_n6, eq27_e1841_d_n7, eq27_e1841_d_n8, eq27_e1841_d_n9, eq27_e1841_d_n10, eq27_e1841_d_n11, eq27_e1841_d_n12, eq27_e1841_q,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let eq27_e1825: f64 = (1.0 + locals.var_sigvds);
        let eq27_e1827: f64 = (eq27_e1825 * locals.var_mig);
        let eq27_e1827_d_n3: f64 = (eq27_e1825 * locals.var_mig_dn3);
        let eq27_e1827_d_n4: f64 = (eq27_e1825 * locals.var_mig_dn4);
        let eq27_e1827_d_n5: f64 = (eq27_e1825 * locals.var_mig_dn5);
        let eq27_e1827_d_n6: f64 = (eq27_e1825 * locals.var_mig_dn6);
        let eq27_e1827_d_n7: f64 = (eq27_e1825 * locals.var_mig_dn7);
        let eq27_e1827_d_n8: f64 = (eq27_e1825 * locals.var_mig_dn8);
        let eq27_e1827_d_n9: f64 = (eq27_e1825 * locals.var_mig_dn9);
        let eq27_e1827_d_n10: f64 = (eq27_e1825 * locals.var_mig_dn10);
        let eq27_e1827_d_n11: f64 = (eq27_e1825 * locals.var_mig_dn11);
        let eq27_e1829: f64 = (eq27_e1827 * locals.var_cox);
        let eq27_e1829_d_n3: f64 = (eq27_e1827_d_n3 * locals.var_cox);
        let eq27_e1829_d_n4: f64 = (eq27_e1827_d_n4 * locals.var_cox);
        let eq27_e1829_d_n5: f64 = (eq27_e1827_d_n5 * locals.var_cox);
        let eq27_e1829_d_n6: f64 = (eq27_e1827_d_n6 * locals.var_cox);
        let eq27_e1829_d_n7: f64 = (eq27_e1827_d_n7 * locals.var_cox);
        let eq27_e1829_d_n8: f64 = (eq27_e1827_d_n8 * locals.var_cox);
        let eq27_e1829_d_n9: f64 = (eq27_e1827_d_n9 * locals.var_cox);
        let eq27_e1829_d_n10: f64 = (eq27_e1827_d_n10 * locals.var_cox);
        let eq27_e1829_d_n11: f64 = (eq27_e1827_d_n11 * locals.var_cox);
        let eq27_e1831: f64 = (eq27_e1829 * locals.var_weff);
        let eq27_e1831_d_n3: f64 = (eq27_e1829_d_n3 * locals.var_weff);
        let eq27_e1831_d_n4: f64 = (eq27_e1829_d_n4 * locals.var_weff);
        let eq27_e1831_d_n5: f64 = (eq27_e1829_d_n5 * locals.var_weff);
        let eq27_e1831_d_n6: f64 = (eq27_e1829_d_n6 * locals.var_weff);
        let eq27_e1831_d_n7: f64 = (eq27_e1829_d_n7 * locals.var_weff);
        let eq27_e1831_d_n8: f64 = (eq27_e1829_d_n8 * locals.var_weff);
        let eq27_e1831_d_n9: f64 = (eq27_e1829_d_n9 * locals.var_weff);
        let eq27_e1831_d_n10: f64 = (eq27_e1829_d_n10 * locals.var_weff);
        let eq27_e1831_d_n11: f64 = (eq27_e1829_d_n11 * locals.var_weff);
        let eq27_e1833: f64 = (eq27_e1831 * p.p2);
        let eq27_e1833_d_n3: f64 = (eq27_e1831_d_n3 * p.p2);
        let eq27_e1833_d_n4: f64 = (eq27_e1831_d_n4 * p.p2);
        let eq27_e1833_d_n5: f64 = (eq27_e1831_d_n5 * p.p2);
        let eq27_e1833_d_n6: f64 = (eq27_e1831_d_n6 * p.p2);
        let eq27_e1833_d_n7: f64 = (eq27_e1831_d_n7 * p.p2);
        let eq27_e1833_d_n8: f64 = (eq27_e1831_d_n8 * p.p2);
        let eq27_e1833_d_n9: f64 = (eq27_e1831_d_n9 * p.p2);
        let eq27_e1833_d_n10: f64 = (eq27_e1831_d_n10 * p.p2);
        let eq27_e1833_d_n11: f64 = (eq27_e1831_d_n11 * p.p2);
        let eq27_e1835: f64 = (eq27_e1833 * locals.var_leff);
        let eq27_e1835_d_n3: f64 = (eq27_e1833_d_n3 * locals.var_leff);
        let eq27_e1835_d_n4: f64 = (eq27_e1833_d_n4 * locals.var_leff);
        let eq27_e1835_d_n5: f64 = (eq27_e1833_d_n5 * locals.var_leff);
        let eq27_e1835_d_n6: f64 = (eq27_e1833_d_n6 * locals.var_leff);
        let eq27_e1835_d_n7: f64 = (eq27_e1833_d_n7 * locals.var_leff);
        let eq27_e1835_d_n8: f64 = (eq27_e1833_d_n8 * locals.var_leff);
        let eq27_e1835_d_n9: f64 = (eq27_e1833_d_n9 * locals.var_leff);
        let eq27_e1835_d_n10: f64 = (eq27_e1833_d_n10 * locals.var_leff);
        let eq27_e1835_d_n11: f64 = (eq27_e1833_d_n11 * locals.var_leff);
        let eq27_e1837: f64 = (eq27_e1835 * (nv12 - 0.0));
        let eq27_e1837_d_n3: f64 = (eq27_e1835_d_n3 * (nv12 - 0.0));
        let eq27_e1837_d_n4: f64 = (eq27_e1835_d_n4 * (nv12 - 0.0));
        let eq27_e1837_d_n5: f64 = (eq27_e1835_d_n5 * (nv12 - 0.0));
        let eq27_e1837_d_n6: f64 = (eq27_e1835_d_n6 * (nv12 - 0.0));
        let eq27_e1837_d_n7: f64 = (eq27_e1835_d_n7 * (nv12 - 0.0));
        let eq27_e1837_d_n8: f64 = (eq27_e1835_d_n8 * (nv12 - 0.0));
        let eq27_e1837_d_n9: f64 = (eq27_e1835_d_n9 * (nv12 - 0.0));
        let eq27_e1837_d_n10: f64 = (eq27_e1835_d_n10 * (nv12 - 0.0));
        let eq27_e1837_d_n11: f64 = (eq27_e1835_d_n11 * (nv12 - 0.0));
        let eq27_e1838: f64 = (0.5 * eq27_e1837);
        let eq27_e1838_d_n3: f64 = (0.5 * eq27_e1837_d_n3);
        let eq27_e1838_d_n4: f64 = (0.5 * eq27_e1837_d_n4);
        let eq27_e1838_d_n5: f64 = (0.5 * eq27_e1837_d_n5);
        let eq27_e1838_d_n6: f64 = (0.5 * eq27_e1837_d_n6);
        let eq27_e1838_d_n7: f64 = (0.5 * eq27_e1837_d_n7);
        let eq27_e1838_d_n8: f64 = (0.5 * eq27_e1837_d_n8);
        let eq27_e1838_d_n9: f64 = (0.5 * eq27_e1837_d_n9);
        let eq27_e1838_d_n10: f64 = (0.5 * eq27_e1837_d_n10);
        let eq27_e1838_d_n11: f64 = (0.5 * eq27_e1837_d_n11);
        let eq27_e1838_d_n12: f64 = (0.5 * eq27_e1835);
        let eq27_e1839_q: f64 = eq27_e1838;
        (eq27_e1838, eq27_e1838_d_n3, eq27_e1838_d_n4, eq27_e1838_d_n5, eq27_e1838_d_n6, eq27_e1838_d_n7, eq27_e1838_d_n8, eq27_e1838_d_n9, eq27_e1838_d_n10, eq27_e1838_d_n11, eq27_e1838_d_n12, eq27_e1839_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq27_e1841_d_n3, eq27_e1841_d_n4, eq27_e1841_d_n5, eq27_e1841_d_n6, eq27_e1841_d_n7, eq27_e1841_d_n8, eq27_e1841_d_n9, eq27_e1841_d_n10, eq27_e1841_d_n11, eq27_e1841_d_n12, 0.0];
        let eq27_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq27_reactive_node_derivatives,
            branches,
            &eq27_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq28_e1868, eq28_e1868_d_n3, eq28_e1868_d_n4, eq28_e1868_d_n5, eq28_e1868_d_n6, eq28_e1868_d_n7, eq28_e1868_d_n8, eq28_e1868_d_n9, eq28_e1868_d_n10, eq28_e1868_d_n11, eq28_e1868_d_n12, eq28_e1868_q,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let eq28_e1852: f64 = (1.0 - locals.var_sigvds);
        let eq28_e1854: f64 = (eq28_e1852 * locals.var_mig);
        let eq28_e1854_d_n3: f64 = (eq28_e1852 * locals.var_mig_dn3);
        let eq28_e1854_d_n4: f64 = (eq28_e1852 * locals.var_mig_dn4);
        let eq28_e1854_d_n5: f64 = (eq28_e1852 * locals.var_mig_dn5);
        let eq28_e1854_d_n6: f64 = (eq28_e1852 * locals.var_mig_dn6);
        let eq28_e1854_d_n7: f64 = (eq28_e1852 * locals.var_mig_dn7);
        let eq28_e1854_d_n8: f64 = (eq28_e1852 * locals.var_mig_dn8);
        let eq28_e1854_d_n9: f64 = (eq28_e1852 * locals.var_mig_dn9);
        let eq28_e1854_d_n10: f64 = (eq28_e1852 * locals.var_mig_dn10);
        let eq28_e1854_d_n11: f64 = (eq28_e1852 * locals.var_mig_dn11);
        let eq28_e1856: f64 = (eq28_e1854 * locals.var_cox);
        let eq28_e1856_d_n3: f64 = (eq28_e1854_d_n3 * locals.var_cox);
        let eq28_e1856_d_n4: f64 = (eq28_e1854_d_n4 * locals.var_cox);
        let eq28_e1856_d_n5: f64 = (eq28_e1854_d_n5 * locals.var_cox);
        let eq28_e1856_d_n6: f64 = (eq28_e1854_d_n6 * locals.var_cox);
        let eq28_e1856_d_n7: f64 = (eq28_e1854_d_n7 * locals.var_cox);
        let eq28_e1856_d_n8: f64 = (eq28_e1854_d_n8 * locals.var_cox);
        let eq28_e1856_d_n9: f64 = (eq28_e1854_d_n9 * locals.var_cox);
        let eq28_e1856_d_n10: f64 = (eq28_e1854_d_n10 * locals.var_cox);
        let eq28_e1856_d_n11: f64 = (eq28_e1854_d_n11 * locals.var_cox);
        let eq28_e1858: f64 = (eq28_e1856 * locals.var_weff);
        let eq28_e1858_d_n3: f64 = (eq28_e1856_d_n3 * locals.var_weff);
        let eq28_e1858_d_n4: f64 = (eq28_e1856_d_n4 * locals.var_weff);
        let eq28_e1858_d_n5: f64 = (eq28_e1856_d_n5 * locals.var_weff);
        let eq28_e1858_d_n6: f64 = (eq28_e1856_d_n6 * locals.var_weff);
        let eq28_e1858_d_n7: f64 = (eq28_e1856_d_n7 * locals.var_weff);
        let eq28_e1858_d_n8: f64 = (eq28_e1856_d_n8 * locals.var_weff);
        let eq28_e1858_d_n9: f64 = (eq28_e1856_d_n9 * locals.var_weff);
        let eq28_e1858_d_n10: f64 = (eq28_e1856_d_n10 * locals.var_weff);
        let eq28_e1858_d_n11: f64 = (eq28_e1856_d_n11 * locals.var_weff);
        let eq28_e1860: f64 = (eq28_e1858 * p.p2);
        let eq28_e1860_d_n3: f64 = (eq28_e1858_d_n3 * p.p2);
        let eq28_e1860_d_n4: f64 = (eq28_e1858_d_n4 * p.p2);
        let eq28_e1860_d_n5: f64 = (eq28_e1858_d_n5 * p.p2);
        let eq28_e1860_d_n6: f64 = (eq28_e1858_d_n6 * p.p2);
        let eq28_e1860_d_n7: f64 = (eq28_e1858_d_n7 * p.p2);
        let eq28_e1860_d_n8: f64 = (eq28_e1858_d_n8 * p.p2);
        let eq28_e1860_d_n9: f64 = (eq28_e1858_d_n9 * p.p2);
        let eq28_e1860_d_n10: f64 = (eq28_e1858_d_n10 * p.p2);
        let eq28_e1860_d_n11: f64 = (eq28_e1858_d_n11 * p.p2);
        let eq28_e1862: f64 = (eq28_e1860 * locals.var_leff);
        let eq28_e1862_d_n3: f64 = (eq28_e1860_d_n3 * locals.var_leff);
        let eq28_e1862_d_n4: f64 = (eq28_e1860_d_n4 * locals.var_leff);
        let eq28_e1862_d_n5: f64 = (eq28_e1860_d_n5 * locals.var_leff);
        let eq28_e1862_d_n6: f64 = (eq28_e1860_d_n6 * locals.var_leff);
        let eq28_e1862_d_n7: f64 = (eq28_e1860_d_n7 * locals.var_leff);
        let eq28_e1862_d_n8: f64 = (eq28_e1860_d_n8 * locals.var_leff);
        let eq28_e1862_d_n9: f64 = (eq28_e1860_d_n9 * locals.var_leff);
        let eq28_e1862_d_n10: f64 = (eq28_e1860_d_n10 * locals.var_leff);
        let eq28_e1862_d_n11: f64 = (eq28_e1860_d_n11 * locals.var_leff);
        let eq28_e1864: f64 = (eq28_e1862 * (nv12 - 0.0));
        let eq28_e1864_d_n3: f64 = (eq28_e1862_d_n3 * (nv12 - 0.0));
        let eq28_e1864_d_n4: f64 = (eq28_e1862_d_n4 * (nv12 - 0.0));
        let eq28_e1864_d_n5: f64 = (eq28_e1862_d_n5 * (nv12 - 0.0));
        let eq28_e1864_d_n6: f64 = (eq28_e1862_d_n6 * (nv12 - 0.0));
        let eq28_e1864_d_n7: f64 = (eq28_e1862_d_n7 * (nv12 - 0.0));
        let eq28_e1864_d_n8: f64 = (eq28_e1862_d_n8 * (nv12 - 0.0));
        let eq28_e1864_d_n9: f64 = (eq28_e1862_d_n9 * (nv12 - 0.0));
        let eq28_e1864_d_n10: f64 = (eq28_e1862_d_n10 * (nv12 - 0.0));
        let eq28_e1864_d_n11: f64 = (eq28_e1862_d_n11 * (nv12 - 0.0));
        let eq28_e1865: f64 = (0.5 * eq28_e1864);
        let eq28_e1865_d_n3: f64 = (0.5 * eq28_e1864_d_n3);
        let eq28_e1865_d_n4: f64 = (0.5 * eq28_e1864_d_n4);
        let eq28_e1865_d_n5: f64 = (0.5 * eq28_e1864_d_n5);
        let eq28_e1865_d_n6: f64 = (0.5 * eq28_e1864_d_n6);
        let eq28_e1865_d_n7: f64 = (0.5 * eq28_e1864_d_n7);
        let eq28_e1865_d_n8: f64 = (0.5 * eq28_e1864_d_n8);
        let eq28_e1865_d_n9: f64 = (0.5 * eq28_e1864_d_n9);
        let eq28_e1865_d_n10: f64 = (0.5 * eq28_e1864_d_n10);
        let eq28_e1865_d_n11: f64 = (0.5 * eq28_e1864_d_n11);
        let eq28_e1865_d_n12: f64 = (0.5 * eq28_e1862);
        let eq28_e1866_q: f64 = eq28_e1865;
        (eq28_e1865, eq28_e1865_d_n3, eq28_e1865_d_n4, eq28_e1865_d_n5, eq28_e1865_d_n6, eq28_e1865_d_n7, eq28_e1865_d_n8, eq28_e1865_d_n9, eq28_e1865_d_n10, eq28_e1865_d_n11, eq28_e1865_d_n12, eq28_e1866_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq28_e1868_d_n3, eq28_e1868_d_n4, eq28_e1868_d_n5, eq28_e1868_d_n6, eq28_e1868_d_n7, eq28_e1868_d_n8, eq28_e1868_d_n9, eq28_e1868_d_n10, eq28_e1868_d_n11, eq28_e1868_d_n12, 0.0];
        let eq28_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        stamper: &mut GeneratedReactiveStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let eq45_e1969_q: f64 = locals.var_qsub;
        let eq45_e1970: f64 = (locals.var_devsign * locals.var_qsub);
        let eq45_e1970_d_n3: f64 = (locals.var_devsign * locals.var_qsub_dn3);
        let eq45_e1970_d_n4: f64 = (locals.var_devsign * locals.var_qsub_dn4);
        let eq45_e1970_d_n5: f64 = (locals.var_devsign * locals.var_qsub_dn5);
        let eq45_e1970_d_n6: f64 = (locals.var_devsign * locals.var_qsub_dn6);
        let eq45_e1970_d_n7: f64 = (locals.var_devsign * locals.var_qsub_dn7);
        let eq45_e1970_d_n8: f64 = (locals.var_devsign * locals.var_qsub_dn8);
        let eq45_e1970_d_n9: f64 = (locals.var_devsign * locals.var_qsub_dn9);
        let eq45_e1970_d_n10: f64 = (locals.var_devsign * locals.var_qsub_dn10);
        let eq45_e1970_d_n11: f64 = (locals.var_devsign * locals.var_qsub_dn11);
        let eq45_e1970_q: f64 = (locals.var_devsign * eq45_e1969_q);
        let eq45_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq45_e1970_d_n3, eq45_e1970_d_n4, eq45_e1970_d_n5, eq45_e1970_d_n6, eq45_e1970_d_n7, eq45_e1970_d_n8, eq45_e1970_d_n9, eq45_e1970_d_n10, eq45_e1970_d_n11, 0.0, 0.0];
        let eq45_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[10]),
            nodes,
            &eq45_reactive_node_derivatives,
            branches,
            &eq45_reactive_branch_derivatives,
            multiplicity,
        );
        let eq46_e1972_q: f64 = locals.var_qde;
        let eq46_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, locals.var_qde_dn3, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11, 0.0, 0.0];
        let eq46_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[3]),
            nodes,
            &eq46_reactive_node_derivatives,
            branches,
            &eq46_reactive_branch_derivatives,
            multiplicity,
        );
        let eq47_e1974_q: f64 = locals.var_qse;
        let eq47_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, locals.var_qse_dn3, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11, 0.0, 0.0];
        let eq47_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            nodes,
            &eq47_reactive_node_derivatives,
            branches,
            &eq47_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq67_e2103, eq67_e2103_d_n0, eq67_e2103_d_n2, eq67_e2103_d_n3, eq67_e2103_d_n4, eq67_e2103_d_n5, eq67_e2103_d_n6, eq67_e2103_d_n7, eq67_e2103_d_n8, eq67_e2103_d_n9, eq67_e2103_d_n10, eq67_e2103_d_n11, eq67_e2103_q, eq67_e2103_q_d_n4, eq67_e2103_q_d_n5,) = {
    if (((locals.var_guard893 != 0.0) && (locals.var_guard896 != 0.0)) && (locals.var_guard897 != 0.0)) {
        let eq67_e2094: f64 = (locals.var_deltemp1 * locals.var_gth);
        let eq67_e2094_d_n4: f64 = (locals.var_deltemp1_dn4 * locals.var_gth);
        let eq67_e2094_d_n5: f64 = (locals.var_deltemp1_dn5 * locals.var_gth);
        let eq67_e2097: f64 = (locals.var_deltemp1 * locals.var_cth);
        let eq67_e2097_d_n4: f64 = (locals.var_deltemp1_dn4 * locals.var_cth);
        let eq67_e2097_d_n5: f64 = (locals.var_deltemp1_dn5 * locals.var_cth);
        let eq67_e2098_q: f64 = eq67_e2097;
        let eq67_e2099: f64 = (eq67_e2094 + eq67_e2097);
        let eq67_e2099_d_n4: f64 = (eq67_e2094_d_n4 + eq67_e2097_d_n4);
        let eq67_e2099_d_n5: f64 = (eq67_e2094_d_n5 + eq67_e2097_d_n5);
        let eq67_e2099_q: f64 = eq67_e2098_q;
        let eq67_e2101: f64 = (eq67_e2099 - locals.var_pdiss);
        let eq67_e2101_d_n4: f64 = (eq67_e2099_d_n4 - locals.var_pdiss_dn4);
        let eq67_e2101_d_n5: f64 = (eq67_e2099_d_n5 - locals.var_pdiss_dn5);
        let eq67_e2101_q: f64 = eq67_e2099_q;
        (eq67_e2101, (-locals.var_pdiss_dn0), (-locals.var_pdiss_dn2), (-locals.var_pdiss_dn3), eq67_e2101_d_n4, eq67_e2101_d_n5, (-locals.var_pdiss_dn6), (-locals.var_pdiss_dn7), (-locals.var_pdiss_dn8), (-locals.var_pdiss_dn9), (-locals.var_pdiss_dn10), (-locals.var_pdiss_dn11), eq67_e2101_q, eq67_e2097_d_n4, eq67_e2097_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node2(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (eq67_e2103_q_d_n4),
            nodes[5],
            multiplicity * (eq67_e2103_q_d_n5),
        );
        let (eq68_e2121, eq68_e2121_d_n0, eq68_e2121_d_n2, eq68_e2121_d_n3, eq68_e2121_d_n4, eq68_e2121_d_n5, eq68_e2121_d_n6, eq68_e2121_d_n7, eq68_e2121_d_n8, eq68_e2121_d_n9, eq68_e2121_d_n10, eq68_e2121_d_n11, eq68_e2121_q, eq68_e2121_q_d_n4, eq68_e2121_q_d_n5,) = {
    if (((locals.var_guard893 != 0.0) && (locals.var_guard896 != 0.0)) && (locals.var_guard897 == 0.0)) {
        let eq68_e2112: f64 = (locals.var_deltemp1 * locals.var_gth);
        let eq68_e2112_d_n4: f64 = (locals.var_deltemp1_dn4 * locals.var_gth);
        let eq68_e2112_d_n5: f64 = (locals.var_deltemp1_dn5 * locals.var_gth);
        let eq68_e2115: f64 = (locals.var_deltemp1 * locals.var_cth);
        let eq68_e2115_d_n4: f64 = (locals.var_deltemp1_dn4 * locals.var_cth);
        let eq68_e2115_d_n5: f64 = (locals.var_deltemp1_dn5 * locals.var_cth);
        let eq68_e2116_q: f64 = eq68_e2115;
        let eq68_e2117: f64 = (eq68_e2112 + eq68_e2115);
        let eq68_e2117_d_n4: f64 = (eq68_e2112_d_n4 + eq68_e2115_d_n4);
        let eq68_e2117_d_n5: f64 = (eq68_e2112_d_n5 + eq68_e2115_d_n5);
        let eq68_e2117_q: f64 = eq68_e2116_q;
        let eq68_e2119: f64 = (eq68_e2117 - locals.var_pdiss);
        let eq68_e2119_d_n4: f64 = (eq68_e2117_d_n4 - locals.var_pdiss_dn4);
        let eq68_e2119_d_n5: f64 = (eq68_e2117_d_n5 - locals.var_pdiss_dn5);
        let eq68_e2119_q: f64 = eq68_e2117_q;
        (eq68_e2119, (-locals.var_pdiss_dn0), (-locals.var_pdiss_dn2), (-locals.var_pdiss_dn3), eq68_e2119_d_n4, eq68_e2119_d_n5, (-locals.var_pdiss_dn6), (-locals.var_pdiss_dn7), (-locals.var_pdiss_dn8), (-locals.var_pdiss_dn9), (-locals.var_pdiss_dn10), (-locals.var_pdiss_dn11), eq68_e2119_q, eq68_e2115_d_n4, eq68_e2115_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node2(
            Some(nodes[5]),
            None,
            nodes[4],
            multiplicity * (eq68_e2121_q_d_n4),
            nodes[5],
            multiplicity * (eq68_e2121_q_d_n5),
        );
        let (eq69_e2137, eq69_e2137_d_n0, eq69_e2137_d_n2, eq69_e2137_d_n3, eq69_e2137_d_n4, eq69_e2137_d_n5, eq69_e2137_d_n6, eq69_e2137_d_n7, eq69_e2137_d_n8, eq69_e2137_d_n9, eq69_e2137_d_n10, eq69_e2137_d_n11, eq69_e2137_q, eq69_e2137_q_d_n4, eq69_e2137_q_d_n5,) = {
    if ((locals.var_guard893 != 0.0) && (locals.var_guard896 == 0.0)) {
        let eq69_e2128: f64 = (locals.var_deltemp1 * locals.var_gth);
        let eq69_e2128_d_n4: f64 = (locals.var_deltemp1_dn4 * locals.var_gth);
        let eq69_e2128_d_n5: f64 = (locals.var_deltemp1_dn5 * locals.var_gth);
        let eq69_e2131: f64 = (locals.var_deltemp1 * locals.var_cth);
        let eq69_e2131_d_n4: f64 = (locals.var_deltemp1_dn4 * locals.var_cth);
        let eq69_e2131_d_n5: f64 = (locals.var_deltemp1_dn5 * locals.var_cth);
        let eq69_e2132_q: f64 = eq69_e2131;
        let eq69_e2133: f64 = (eq69_e2128 + eq69_e2131);
        let eq69_e2133_d_n4: f64 = (eq69_e2128_d_n4 + eq69_e2131_d_n4);
        let eq69_e2133_d_n5: f64 = (eq69_e2128_d_n5 + eq69_e2131_d_n5);
        let eq69_e2133_q: f64 = eq69_e2132_q;
        let eq69_e2135: f64 = (eq69_e2133 - locals.var_pdiss);
        let eq69_e2135_d_n4: f64 = (eq69_e2133_d_n4 - locals.var_pdiss_dn4);
        let eq69_e2135_d_n5: f64 = (eq69_e2133_d_n5 - locals.var_pdiss_dn5);
        let eq69_e2135_q: f64 = eq69_e2133_q;
        (eq69_e2135, (-locals.var_pdiss_dn0), (-locals.var_pdiss_dn2), (-locals.var_pdiss_dn3), eq69_e2135_d_n4, eq69_e2135_d_n5, (-locals.var_pdiss_dn6), (-locals.var_pdiss_dn7), (-locals.var_pdiss_dn8), (-locals.var_pdiss_dn9), (-locals.var_pdiss_dn10), (-locals.var_pdiss_dn11), eq69_e2135_q, eq69_e2131_d_n4, eq69_e2131_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node2(
            Some(nodes[5]),
            None,
            nodes[4],
            multiplicity * (eq69_e2137_q_d_n4),
            nodes[5],
            multiplicity * (eq69_e2137_q_d_n5),
        );
        let eq80_e2212_q: f64 = locals.var_qbsj;
        let eq80_e2213: f64 = (locals.var_devsign * locals.var_qbsj);
        let eq80_e2213_d_n3: f64 = (locals.var_devsign * locals.var_qbsj_dn3);
        let eq80_e2213_d_n4: f64 = (locals.var_devsign * locals.var_qbsj_dn4);
        let eq80_e2213_d_n5: f64 = (locals.var_devsign * locals.var_qbsj_dn5);
        let eq80_e2213_d_n6: f64 = (locals.var_devsign * locals.var_qbsj_dn6);
        let eq80_e2213_d_n7: f64 = (locals.var_devsign * locals.var_qbsj_dn7);
        let eq80_e2213_d_n8: f64 = (locals.var_devsign * locals.var_qbsj_dn8);
        let eq80_e2213_d_n9: f64 = (locals.var_devsign * locals.var_qbsj_dn9);
        let eq80_e2213_d_n10: f64 = (locals.var_devsign * locals.var_qbsj_dn10);
        let eq80_e2213_d_n11: f64 = (locals.var_devsign * locals.var_qbsj_dn11);
        let eq80_e2213_q: f64 = (locals.var_devsign * eq80_e2212_q);
        let eq80_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq80_e2213_d_n3, eq80_e2213_d_n4, eq80_e2213_d_n5, eq80_e2213_d_n6, eq80_e2213_d_n7, eq80_e2213_d_n8, eq80_e2213_d_n9, eq80_e2213_d_n10, eq80_e2213_d_n11, 0.0, 0.0];
        let eq80_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            nodes,
            &eq80_reactive_node_derivatives,
            branches,
            &eq80_reactive_branch_derivatives,
            multiplicity,
        );
        let eq81_e2216_q: f64 = locals.var_qbdj;
        let eq81_e2217: f64 = (locals.var_devsign * locals.var_qbdj);
        let eq81_e2217_d_n3: f64 = (locals.var_devsign * locals.var_qbdj_dn3);
        let eq81_e2217_d_n4: f64 = (locals.var_devsign * locals.var_qbdj_dn4);
        let eq81_e2217_d_n5: f64 = (locals.var_devsign * locals.var_qbdj_dn5);
        let eq81_e2217_d_n6: f64 = (locals.var_devsign * locals.var_qbdj_dn6);
        let eq81_e2217_d_n7: f64 = (locals.var_devsign * locals.var_qbdj_dn7);
        let eq81_e2217_d_n8: f64 = (locals.var_devsign * locals.var_qbdj_dn8);
        let eq81_e2217_d_n9: f64 = (locals.var_devsign * locals.var_qbdj_dn9);
        let eq81_e2217_d_n10: f64 = (locals.var_devsign * locals.var_qbdj_dn10);
        let eq81_e2217_d_n11: f64 = (locals.var_devsign * locals.var_qbdj_dn11);
        let eq81_e2217_q: f64 = (locals.var_devsign * eq81_e2216_q);
        let eq81_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq81_e2217_d_n3, eq81_e2217_d_n4, eq81_e2217_d_n5, eq81_e2217_d_n6, eq81_e2217_d_n7, eq81_e2217_d_n8, eq81_e2217_d_n9, eq81_e2217_d_n10, eq81_e2217_d_n11, 0.0, 0.0];
        let eq81_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[6]),
            nodes,
            &eq81_reactive_node_derivatives,
            branches,
            &eq81_reactive_branch_derivatives,
            multiplicity,
        );
    }
}

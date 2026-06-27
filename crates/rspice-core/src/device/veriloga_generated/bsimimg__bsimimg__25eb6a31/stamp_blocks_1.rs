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
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let eq12_e873: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, s.v[223]);
        let eq12_e873_d_n0: f64 = (s.dn[223][0] * ddt_scale);
        let eq12_e873_d_n1: f64 = (s.dn[223][1] * ddt_scale);
        let eq12_e873_d_n2: f64 = (s.dn[223][2] * ddt_scale);
        let eq12_e873_d_n3: f64 = (s.dn[223][3] * ddt_scale);
        let eq12_e873_d_n4: f64 = (s.dn[223][4] * ddt_scale);
        let eq12_e873_d_n5: f64 = (s.dn[223][5] * ddt_scale);
        let eq12_e873_d_n6: f64 = (s.dn[223][6] * ddt_scale);
        let eq12_e873_d_n7: f64 = (s.dn[223][7] * ddt_scale);
        let eq12_e873_d_n8: f64 = (s.dn[223][8] * ddt_scale);
        let eq12_e873_d_b0: f64 = (s.db[223][0] * ddt_scale);
        let eq12_e873_d_b1: f64 = (s.db[223][1] * ddt_scale);
        let eq12_e873_d_b2: f64 = (s.db[223][2] * ddt_scale);
        let eq12_e873_d_b3: f64 = (s.db[223][3] * ddt_scale);
        let eq12_e873_d_b4: f64 = (s.db[223][4] * ddt_scale);
        let eq12_e874: f64 = (s.v[212] * eq12_e873);
        let eq12_e874_d_n0: f64 = ((s.dn[212][0] * eq12_e873) + (s.v[212] * eq12_e873_d_n0));
        let eq12_e874_d_n1: f64 = ((s.dn[212][1] * eq12_e873) + (s.v[212] * eq12_e873_d_n1));
        let eq12_e874_d_n2: f64 = ((s.dn[212][2] * eq12_e873) + (s.v[212] * eq12_e873_d_n2));
        let eq12_e874_d_n3: f64 = ((s.dn[212][3] * eq12_e873) + (s.v[212] * eq12_e873_d_n3));
        let eq12_e874_d_n4: f64 = ((s.dn[212][4] * eq12_e873) + (s.v[212] * eq12_e873_d_n4));
        let eq12_e874_d_n5: f64 = ((s.dn[212][5] * eq12_e873) + (s.v[212] * eq12_e873_d_n5));
        let eq12_e874_d_n6: f64 = ((s.dn[212][6] * eq12_e873) + (s.v[212] * eq12_e873_d_n6));
        let eq12_e874_d_n7: f64 = ((s.dn[212][7] * eq12_e873) + (s.v[212] * eq12_e873_d_n7));
        let eq12_e874_d_n8: f64 = ((s.dn[212][8] * eq12_e873) + (s.v[212] * eq12_e873_d_n8));
        let eq12_e874_d_b0: f64 = ((s.db[212][0] * eq12_e873) + (s.v[212] * eq12_e873_d_b0));
        let eq12_e874_d_b1: f64 = ((s.db[212][1] * eq12_e873) + (s.v[212] * eq12_e873_d_b1));
        let eq12_e874_d_b2: f64 = ((s.db[212][2] * eq12_e873) + (s.v[212] * eq12_e873_d_b2));
        let eq12_e874_d_b3: f64 = ((s.db[212][3] * eq12_e873) + (s.v[212] * eq12_e873_d_b3));
        let eq12_e874_d_b4: f64 = ((s.db[212][4] * eq12_e873) + (s.v[212] * eq12_e873_d_b4));
        let eq12_value: f64 = eq12_e874;
        let eq12_node_derivatives: [f64; 9] = [eq12_e874_d_n0, eq12_e874_d_n1, eq12_e874_d_n2, eq12_e874_d_n3, eq12_e874_d_n4, eq12_e874_d_n5, eq12_e874_d_n6, eq12_e874_d_n7, eq12_e874_d_n8];
        let eq12_branch_derivatives: [f64; 5] = [eq12_e874_d_b0, eq12_e874_d_b1, eq12_e874_d_b2, eq12_e874_d_b3, eq12_e874_d_b4];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_e876: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, s.v[224]);
        let eq13_e876_d_n0: f64 = (s.dn[224][0] * ddt_scale);
        let eq13_e876_d_n1: f64 = (s.dn[224][1] * ddt_scale);
        let eq13_e876_d_n2: f64 = (s.dn[224][2] * ddt_scale);
        let eq13_e876_d_n3: f64 = (s.dn[224][3] * ddt_scale);
        let eq13_e876_d_n4: f64 = (s.dn[224][4] * ddt_scale);
        let eq13_e876_d_n5: f64 = (s.dn[224][5] * ddt_scale);
        let eq13_e876_d_n6: f64 = (s.dn[224][6] * ddt_scale);
        let eq13_e876_d_n7: f64 = (s.dn[224][7] * ddt_scale);
        let eq13_e876_d_n8: f64 = (s.dn[224][8] * ddt_scale);
        let eq13_e876_d_b0: f64 = (s.db[224][0] * ddt_scale);
        let eq13_e876_d_b1: f64 = (s.db[224][1] * ddt_scale);
        let eq13_e876_d_b2: f64 = (s.db[224][2] * ddt_scale);
        let eq13_e876_d_b3: f64 = (s.db[224][3] * ddt_scale);
        let eq13_e876_d_b4: f64 = (s.db[224][4] * ddt_scale);
        let eq13_value: f64 = eq13_e876;
        let eq13_node_derivatives: [f64; 9] = [eq13_e876_d_n0, eq13_e876_d_n1, eq13_e876_d_n2, eq13_e876_d_n3, eq13_e876_d_n4, eq13_e876_d_n5, eq13_e876_d_n6, eq13_e876_d_n7, eq13_e876_d_n8];
        let eq13_branch_derivatives: [f64; 5] = [eq13_e876_d_b0, eq13_e876_d_b1, eq13_e876_d_b2, eq13_e876_d_b3, eq13_e876_d_b4];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_e879: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, s.v[225]);
        let eq14_e879_d_n0: f64 = (s.dn[225][0] * ddt_scale);
        let eq14_e879_d_n1: f64 = (s.dn[225][1] * ddt_scale);
        let eq14_e879_d_n2: f64 = (s.dn[225][2] * ddt_scale);
        let eq14_e879_d_n3: f64 = (s.dn[225][3] * ddt_scale);
        let eq14_e879_d_n4: f64 = (s.dn[225][4] * ddt_scale);
        let eq14_e879_d_n5: f64 = (s.dn[225][5] * ddt_scale);
        let eq14_e879_d_n6: f64 = (s.dn[225][6] * ddt_scale);
        let eq14_e879_d_n7: f64 = (s.dn[225][7] * ddt_scale);
        let eq14_e879_d_n8: f64 = (s.dn[225][8] * ddt_scale);
        let eq14_e879_d_b0: f64 = (s.db[225][0] * ddt_scale);
        let eq14_e879_d_b1: f64 = (s.db[225][1] * ddt_scale);
        let eq14_e879_d_b2: f64 = (s.db[225][2] * ddt_scale);
        let eq14_e879_d_b3: f64 = (s.db[225][3] * ddt_scale);
        let eq14_e879_d_b4: f64 = (s.db[225][4] * ddt_scale);
        let eq14_e880: f64 = (s.v[212] * eq14_e879);
        let eq14_e880_d_n0: f64 = ((s.dn[212][0] * eq14_e879) + (s.v[212] * eq14_e879_d_n0));
        let eq14_e880_d_n1: f64 = ((s.dn[212][1] * eq14_e879) + (s.v[212] * eq14_e879_d_n1));
        let eq14_e880_d_n2: f64 = ((s.dn[212][2] * eq14_e879) + (s.v[212] * eq14_e879_d_n2));
        let eq14_e880_d_n3: f64 = ((s.dn[212][3] * eq14_e879) + (s.v[212] * eq14_e879_d_n3));
        let eq14_e880_d_n4: f64 = ((s.dn[212][4] * eq14_e879) + (s.v[212] * eq14_e879_d_n4));
        let eq14_e880_d_n5: f64 = ((s.dn[212][5] * eq14_e879) + (s.v[212] * eq14_e879_d_n5));
        let eq14_e880_d_n6: f64 = ((s.dn[212][6] * eq14_e879) + (s.v[212] * eq14_e879_d_n6));
        let eq14_e880_d_n7: f64 = ((s.dn[212][7] * eq14_e879) + (s.v[212] * eq14_e879_d_n7));
        let eq14_e880_d_n8: f64 = ((s.dn[212][8] * eq14_e879) + (s.v[212] * eq14_e879_d_n8));
        let eq14_e880_d_b0: f64 = ((s.db[212][0] * eq14_e879) + (s.v[212] * eq14_e879_d_b0));
        let eq14_e880_d_b1: f64 = ((s.db[212][1] * eq14_e879) + (s.v[212] * eq14_e879_d_b1));
        let eq14_e880_d_b2: f64 = ((s.db[212][2] * eq14_e879) + (s.v[212] * eq14_e879_d_b2));
        let eq14_e880_d_b3: f64 = ((s.db[212][3] * eq14_e879) + (s.v[212] * eq14_e879_d_b3));
        let eq14_e880_d_b4: f64 = ((s.db[212][4] * eq14_e879) + (s.v[212] * eq14_e879_d_b4));
        let eq14_value: f64 = eq14_e880;
        let eq14_node_derivatives: [f64; 9] = [eq14_e880_d_n0, eq14_e880_d_n1, eq14_e880_d_n2, eq14_e880_d_n3, eq14_e880_d_n4, eq14_e880_d_n5, eq14_e880_d_n6, eq14_e880_d_n7, eq14_e880_d_n8];
        let eq14_branch_derivatives: [f64; 5] = [eq14_e880_d_b0, eq14_e880_d_b1, eq14_e880_d_b2, eq14_e880_d_b3, eq14_e880_d_b4];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(6),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let eq15_e882: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, s.v[226]);
        let eq15_e882_d_n0: f64 = (s.dn[226][0] * ddt_scale);
        let eq15_e882_d_n1: f64 = (s.dn[226][1] * ddt_scale);
        let eq15_e882_d_n2: f64 = (s.dn[226][2] * ddt_scale);
        let eq15_e882_d_n3: f64 = (s.dn[226][3] * ddt_scale);
        let eq15_e882_d_n4: f64 = (s.dn[226][4] * ddt_scale);
        let eq15_e882_d_n5: f64 = (s.dn[226][5] * ddt_scale);
        let eq15_e882_d_n6: f64 = (s.dn[226][6] * ddt_scale);
        let eq15_e882_d_n7: f64 = (s.dn[226][7] * ddt_scale);
        let eq15_e882_d_n8: f64 = (s.dn[226][8] * ddt_scale);
        let eq15_e882_d_b0: f64 = (s.db[226][0] * ddt_scale);
        let eq15_e882_d_b1: f64 = (s.db[226][1] * ddt_scale);
        let eq15_e882_d_b2: f64 = (s.db[226][2] * ddt_scale);
        let eq15_e882_d_b3: f64 = (s.db[226][3] * ddt_scale);
        let eq15_e882_d_b4: f64 = (s.db[226][4] * ddt_scale);
        let eq15_value: f64 = eq15_e882;
        let eq15_node_derivatives: [f64; 9] = [eq15_e882_d_n0, eq15_e882_d_n1, eq15_e882_d_n2, eq15_e882_d_n3, eq15_e882_d_n4, eq15_e882_d_n5, eq15_e882_d_n6, eq15_e882_d_n7, eq15_e882_d_n8];
        let eq15_branch_derivatives: [f64; 5] = [eq15_e882_d_b0, eq15_e882_d_b1, eq15_e882_d_b2, eq15_e882_d_b3, eq15_e882_d_b4];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let eq16_e884: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, s.v[227]);
        let eq16_e884_d_n0: f64 = (s.dn[227][0] * ddt_scale);
        let eq16_e884_d_n1: f64 = (s.dn[227][1] * ddt_scale);
        let eq16_e884_d_n2: f64 = (s.dn[227][2] * ddt_scale);
        let eq16_e884_d_n3: f64 = (s.dn[227][3] * ddt_scale);
        let eq16_e884_d_n4: f64 = (s.dn[227][4] * ddt_scale);
        let eq16_e884_d_n5: f64 = (s.dn[227][5] * ddt_scale);
        let eq16_e884_d_n6: f64 = (s.dn[227][6] * ddt_scale);
        let eq16_e884_d_n7: f64 = (s.dn[227][7] * ddt_scale);
        let eq16_e884_d_n8: f64 = (s.dn[227][8] * ddt_scale);
        let eq16_e884_d_b0: f64 = (s.db[227][0] * ddt_scale);
        let eq16_e884_d_b1: f64 = (s.db[227][1] * ddt_scale);
        let eq16_e884_d_b2: f64 = (s.db[227][2] * ddt_scale);
        let eq16_e884_d_b3: f64 = (s.db[227][3] * ddt_scale);
        let eq16_e884_d_b4: f64 = (s.db[227][4] * ddt_scale);
        let eq16_value: f64 = eq16_e884;
        let eq16_node_derivatives: [f64; 9] = [eq16_e884_d_n0, eq16_e884_d_n1, eq16_e884_d_n2, eq16_e884_d_n3, eq16_e884_d_n4, eq16_e884_d_n5, eq16_e884_d_n6, eq16_e884_d_n7, eq16_e884_d_n8];
        let eq16_branch_derivatives: [f64; 5] = [eq16_e884_d_b0, eq16_e884_d_b1, eq16_e884_d_b2, eq16_e884_d_b3, eq16_e884_d_b4];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq16_value),
            &eq16_node_derivatives,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let eq17_e887: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, s.v[238]);
        let eq17_e887_d_n0: f64 = (s.dn[238][0] * ddt_scale);
        let eq17_e887_d_n1: f64 = (s.dn[238][1] * ddt_scale);
        let eq17_e887_d_n2: f64 = (s.dn[238][2] * ddt_scale);
        let eq17_e887_d_n3: f64 = (s.dn[238][3] * ddt_scale);
        let eq17_e887_d_n4: f64 = (s.dn[238][4] * ddt_scale);
        let eq17_e887_d_n5: f64 = (s.dn[238][5] * ddt_scale);
        let eq17_e887_d_n6: f64 = (s.dn[238][6] * ddt_scale);
        let eq17_e887_d_n7: f64 = (s.dn[238][7] * ddt_scale);
        let eq17_e887_d_n8: f64 = (s.dn[238][8] * ddt_scale);
        let eq17_e887_d_b0: f64 = (s.db[238][0] * ddt_scale);
        let eq17_e887_d_b1: f64 = (s.db[238][1] * ddt_scale);
        let eq17_e887_d_b2: f64 = (s.db[238][2] * ddt_scale);
        let eq17_e887_d_b3: f64 = (s.db[238][3] * ddt_scale);
        let eq17_e887_d_b4: f64 = (s.db[238][4] * ddt_scale);
        let eq17_e888: f64 = (s.v[212] * eq17_e887);
        let eq17_e888_d_n0: f64 = ((s.dn[212][0] * eq17_e887) + (s.v[212] * eq17_e887_d_n0));
        let eq17_e888_d_n1: f64 = ((s.dn[212][1] * eq17_e887) + (s.v[212] * eq17_e887_d_n1));
        let eq17_e888_d_n2: f64 = ((s.dn[212][2] * eq17_e887) + (s.v[212] * eq17_e887_d_n2));
        let eq17_e888_d_n3: f64 = ((s.dn[212][3] * eq17_e887) + (s.v[212] * eq17_e887_d_n3));
        let eq17_e888_d_n4: f64 = ((s.dn[212][4] * eq17_e887) + (s.v[212] * eq17_e887_d_n4));
        let eq17_e888_d_n5: f64 = ((s.dn[212][5] * eq17_e887) + (s.v[212] * eq17_e887_d_n5));
        let eq17_e888_d_n6: f64 = ((s.dn[212][6] * eq17_e887) + (s.v[212] * eq17_e887_d_n6));
        let eq17_e888_d_n7: f64 = ((s.dn[212][7] * eq17_e887) + (s.v[212] * eq17_e887_d_n7));
        let eq17_e888_d_n8: f64 = ((s.dn[212][8] * eq17_e887) + (s.v[212] * eq17_e887_d_n8));
        let eq17_e888_d_b0: f64 = ((s.db[212][0] * eq17_e887) + (s.v[212] * eq17_e887_d_b0));
        let eq17_e888_d_b1: f64 = ((s.db[212][1] * eq17_e887) + (s.v[212] * eq17_e887_d_b1));
        let eq17_e888_d_b2: f64 = ((s.db[212][2] * eq17_e887) + (s.v[212] * eq17_e887_d_b2));
        let eq17_e888_d_b3: f64 = ((s.db[212][3] * eq17_e887) + (s.v[212] * eq17_e887_d_b3));
        let eq17_e888_d_b4: f64 = ((s.db[212][4] * eq17_e887) + (s.v[212] * eq17_e887_d_b4));
        let eq17_value: f64 = eq17_e888;
        let eq17_node_derivatives: [f64; 9] = [eq17_e888_d_n0, eq17_e888_d_n1, eq17_e888_d_n2, eq17_e888_d_n3, eq17_e888_d_n4, eq17_e888_d_n5, eq17_e888_d_n6, eq17_e888_d_n7, eq17_e888_d_n8];
        let eq17_branch_derivatives: [f64; 5] = [eq17_e888_d_b0, eq17_e888_d_b1, eq17_e888_d_b2, eq17_e888_d_b3, eq17_e888_d_b4];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(3),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let eq18_e891: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, s.v[239]);
        let eq18_e891_d_n0: f64 = (s.dn[239][0] * ddt_scale);
        let eq18_e891_d_n1: f64 = (s.dn[239][1] * ddt_scale);
        let eq18_e891_d_n2: f64 = (s.dn[239][2] * ddt_scale);
        let eq18_e891_d_n3: f64 = (s.dn[239][3] * ddt_scale);
        let eq18_e891_d_n4: f64 = (s.dn[239][4] * ddt_scale);
        let eq18_e891_d_n5: f64 = (s.dn[239][5] * ddt_scale);
        let eq18_e891_d_n6: f64 = (s.dn[239][6] * ddt_scale);
        let eq18_e891_d_n7: f64 = (s.dn[239][7] * ddt_scale);
        let eq18_e891_d_n8: f64 = (s.dn[239][8] * ddt_scale);
        let eq18_e891_d_b0: f64 = (s.db[239][0] * ddt_scale);
        let eq18_e891_d_b1: f64 = (s.db[239][1] * ddt_scale);
        let eq18_e891_d_b2: f64 = (s.db[239][2] * ddt_scale);
        let eq18_e891_d_b3: f64 = (s.db[239][3] * ddt_scale);
        let eq18_e891_d_b4: f64 = (s.db[239][4] * ddt_scale);
        let eq18_e892: f64 = (s.v[212] * eq18_e891);
        let eq18_e892_d_n0: f64 = ((s.dn[212][0] * eq18_e891) + (s.v[212] * eq18_e891_d_n0));
        let eq18_e892_d_n1: f64 = ((s.dn[212][1] * eq18_e891) + (s.v[212] * eq18_e891_d_n1));
        let eq18_e892_d_n2: f64 = ((s.dn[212][2] * eq18_e891) + (s.v[212] * eq18_e891_d_n2));
        let eq18_e892_d_n3: f64 = ((s.dn[212][3] * eq18_e891) + (s.v[212] * eq18_e891_d_n3));
        let eq18_e892_d_n4: f64 = ((s.dn[212][4] * eq18_e891) + (s.v[212] * eq18_e891_d_n4));
        let eq18_e892_d_n5: f64 = ((s.dn[212][5] * eq18_e891) + (s.v[212] * eq18_e891_d_n5));
        let eq18_e892_d_n6: f64 = ((s.dn[212][6] * eq18_e891) + (s.v[212] * eq18_e891_d_n6));
        let eq18_e892_d_n7: f64 = ((s.dn[212][7] * eq18_e891) + (s.v[212] * eq18_e891_d_n7));
        let eq18_e892_d_n8: f64 = ((s.dn[212][8] * eq18_e891) + (s.v[212] * eq18_e891_d_n8));
        let eq18_e892_d_b0: f64 = ((s.db[212][0] * eq18_e891) + (s.v[212] * eq18_e891_d_b0));
        let eq18_e892_d_b1: f64 = ((s.db[212][1] * eq18_e891) + (s.v[212] * eq18_e891_d_b1));
        let eq18_e892_d_b2: f64 = ((s.db[212][2] * eq18_e891) + (s.v[212] * eq18_e891_d_b2));
        let eq18_e892_d_b3: f64 = ((s.db[212][3] * eq18_e891) + (s.v[212] * eq18_e891_d_b3));
        let eq18_e892_d_b4: f64 = ((s.db[212][4] * eq18_e891) + (s.v[212] * eq18_e891_d_b4));
        let eq18_value: f64 = eq18_e892;
        let eq18_node_derivatives: [f64; 9] = [eq18_e892_d_n0, eq18_e892_d_n1, eq18_e892_d_n2, eq18_e892_d_n3, eq18_e892_d_n4, eq18_e892_d_n5, eq18_e892_d_n6, eq18_e892_d_n7, eq18_e892_d_n8];
        let eq18_branch_derivatives: [f64; 5] = [eq18_e892_d_b0, eq18_e892_d_b1, eq18_e892_d_b2, eq18_e892_d_b3, eq18_e892_d_b4];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(3),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let (eq19_e896,) = {
    if s.b[663] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e896;
        stamper.stamp_potential_const_local(
            0,
            eq19_value,
        );
        let (eq20_e900,) = {
    if s.b[663] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e900;
        stamper.stamp_potential_const_local(
            1,
            eq20_value,
        );
        let (eq21_e907, eq21_e907_d_n0, eq21_e907_d_n1, eq21_e907_d_n2, eq21_e907_d_n3, eq21_e907_d_n4, eq21_e907_d_n5, eq21_e907_d_n6, eq21_e907_d_n7, eq21_e907_d_n8, eq21_e907_d_b0, eq21_e907_d_b1, eq21_e907_d_b2, eq21_e907_d_b3, eq21_e907_d_b4,) = {
    if (!s.b[663]) {
        let eq21_e905: f64 = ((nv0 - nv5) * s.v[149]);
        let eq21_e905_d_n0: f64 = (s.v[149] + ((nv0 - nv5) * s.dn[149][0]));
        let eq21_e905_d_n1: f64 = ((nv0 - nv5) * s.dn[149][1]);
        let eq21_e905_d_n2: f64 = ((nv0 - nv5) * s.dn[149][2]);
        let eq21_e905_d_n3: f64 = ((nv0 - nv5) * s.dn[149][3]);
        let eq21_e905_d_n4: f64 = ((nv0 - nv5) * s.dn[149][4]);
        let eq21_e905_d_n5: f64 = ((-s.v[149]) + ((nv0 - nv5) * s.dn[149][5]));
        let eq21_e905_d_n6: f64 = ((nv0 - nv5) * s.dn[149][6]);
        let eq21_e905_d_n7: f64 = ((nv0 - nv5) * s.dn[149][7]);
        let eq21_e905_d_n8: f64 = ((nv0 - nv5) * s.dn[149][8]);
        let eq21_e905_d_b0: f64 = ((nv0 - nv5) * s.db[149][0]);
        let eq21_e905_d_b1: f64 = ((nv0 - nv5) * s.db[149][1]);
        let eq21_e905_d_b2: f64 = ((nv0 - nv5) * s.db[149][2]);
        let eq21_e905_d_b3: f64 = ((nv0 - nv5) * s.db[149][3]);
        let eq21_e905_d_b4: f64 = ((nv0 - nv5) * s.db[149][4]);
        (eq21_e905, eq21_e905_d_n0, eq21_e905_d_n1, eq21_e905_d_n2, eq21_e905_d_n3, eq21_e905_d_n4, eq21_e905_d_n5, eq21_e905_d_n6, eq21_e905_d_n7, eq21_e905_d_n8, eq21_e905_d_b0, eq21_e905_d_b1, eq21_e905_d_b2, eq21_e905_d_b3, eq21_e905_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e907;
        let eq21_node_derivatives: [f64; 9] = [eq21_e907_d_n0, eq21_e907_d_n1, eq21_e907_d_n2, eq21_e907_d_n3, eq21_e907_d_n4, eq21_e907_d_n5, eq21_e907_d_n6, eq21_e907_d_n7, eq21_e907_d_n8];
        let eq21_branch_derivatives: [f64; 5] = [eq21_e907_d_b0, eq21_e907_d_b1, eq21_e907_d_b2, eq21_e907_d_b3, eq21_e907_d_b4];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(5),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let (eq22_e914, eq22_e914_d_n0, eq22_e914_d_n1, eq22_e914_d_n2, eq22_e914_d_n3, eq22_e914_d_n4, eq22_e914_d_n5, eq22_e914_d_n6, eq22_e914_d_n7, eq22_e914_d_n8, eq22_e914_d_b0, eq22_e914_d_b1, eq22_e914_d_b2, eq22_e914_d_b3, eq22_e914_d_b4,) = {
    if (!s.b[663]) {
        let eq22_e912: f64 = ((nv2 - nv6) * s.v[148]);
        let eq22_e912_d_n0: f64 = ((nv2 - nv6) * s.dn[148][0]);
        let eq22_e912_d_n1: f64 = ((nv2 - nv6) * s.dn[148][1]);
        let eq22_e912_d_n2: f64 = (s.v[148] + ((nv2 - nv6) * s.dn[148][2]));
        let eq22_e912_d_n3: f64 = ((nv2 - nv6) * s.dn[148][3]);
        let eq22_e912_d_n4: f64 = ((nv2 - nv6) * s.dn[148][4]);
        let eq22_e912_d_n5: f64 = ((nv2 - nv6) * s.dn[148][5]);
        let eq22_e912_d_n6: f64 = ((-s.v[148]) + ((nv2 - nv6) * s.dn[148][6]));
        let eq22_e912_d_n7: f64 = ((nv2 - nv6) * s.dn[148][7]);
        let eq22_e912_d_n8: f64 = ((nv2 - nv6) * s.dn[148][8]);
        let eq22_e912_d_b0: f64 = ((nv2 - nv6) * s.db[148][0]);
        let eq22_e912_d_b1: f64 = ((nv2 - nv6) * s.db[148][1]);
        let eq22_e912_d_b2: f64 = ((nv2 - nv6) * s.db[148][2]);
        let eq22_e912_d_b3: f64 = ((nv2 - nv6) * s.db[148][3]);
        let eq22_e912_d_b4: f64 = ((nv2 - nv6) * s.db[148][4]);
        (eq22_e912, eq22_e912_d_n0, eq22_e912_d_n1, eq22_e912_d_n2, eq22_e912_d_n3, eq22_e912_d_n4, eq22_e912_d_n5, eq22_e912_d_n6, eq22_e912_d_n7, eq22_e912_d_n8, eq22_e912_d_b0, eq22_e912_d_b1, eq22_e912_d_b2, eq22_e912_d_b3, eq22_e912_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq22_value: f64 = eq22_e914;
        let eq22_node_derivatives: [f64; 9] = [eq22_e914_d_n0, eq22_e914_d_n1, eq22_e914_d_n2, eq22_e914_d_n3, eq22_e914_d_n4, eq22_e914_d_n5, eq22_e914_d_n6, eq22_e914_d_n7, eq22_e914_d_n8];
        let eq22_branch_derivatives: [f64; 5] = [eq22_e914_d_b0, eq22_e914_d_b1, eq22_e914_d_b2, eq22_e914_d_b3, eq22_e914_d_b4];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(6),
            multiplicity * (eq22_value),
            &eq22_node_derivatives,
            &eq22_branch_derivatives,
            multiplicity,
        );
        let (eq23_e923,) = {
    if (!s.b[663]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e923;
        stamper.stamp_current_const_local(
            Some(0),
            Some(5),
            multiplicity * (eq23_value),
        );
        let (eq24_e932,) = {
    if (!s.b[663]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq24_value: f64 = eq24_e932;
        stamper.stamp_current_const_local(
            Some(2),
            Some(6),
            multiplicity * (eq24_value),
        );
        let (eq25_e938, eq25_e938_d_n0, eq25_e938_d_n1, eq25_e938_d_n2, eq25_e938_d_n3, eq25_e938_d_n4, eq25_e938_d_n5, eq25_e938_d_n6, eq25_e938_d_n7, eq25_e938_d_n8, eq25_e938_d_b0, eq25_e938_d_b1, eq25_e938_d_b2, eq25_e938_d_b3, eq25_e938_d_b4,) = {
    if s.b[664] {
        let eq25_e936: f64 = ((nv7 - nv8) * s.v[274]);
        let eq25_e936_d_n0: f64 = ((nv7 - nv8) * s.dn[274][0]);
        let eq25_e936_d_n1: f64 = ((nv7 - nv8) * s.dn[274][1]);
        let eq25_e936_d_n2: f64 = ((nv7 - nv8) * s.dn[274][2]);
        let eq25_e936_d_n3: f64 = ((nv7 - nv8) * s.dn[274][3]);
        let eq25_e936_d_n4: f64 = ((nv7 - nv8) * s.dn[274][4]);
        let eq25_e936_d_n5: f64 = ((nv7 - nv8) * s.dn[274][5]);
        let eq25_e936_d_n6: f64 = ((nv7 - nv8) * s.dn[274][6]);
        let eq25_e936_d_n7: f64 = (s.v[274] + ((nv7 - nv8) * s.dn[274][7]));
        let eq25_e936_d_n8: f64 = ((-s.v[274]) + ((nv7 - nv8) * s.dn[274][8]));
        let eq25_e936_d_b0: f64 = ((nv7 - nv8) * s.db[274][0]);
        let eq25_e936_d_b1: f64 = ((nv7 - nv8) * s.db[274][1]);
        let eq25_e936_d_b2: f64 = ((nv7 - nv8) * s.db[274][2]);
        let eq25_e936_d_b3: f64 = ((nv7 - nv8) * s.db[274][3]);
        let eq25_e936_d_b4: f64 = ((nv7 - nv8) * s.db[274][4]);
        (eq25_e936, eq25_e936_d_n0, eq25_e936_d_n1, eq25_e936_d_n2, eq25_e936_d_n3, eq25_e936_d_n4, eq25_e936_d_n5, eq25_e936_d_n6, eq25_e936_d_n7, eq25_e936_d_n8, eq25_e936_d_b0, eq25_e936_d_b1, eq25_e936_d_b2, eq25_e936_d_b3, eq25_e936_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e938;
        let eq25_node_derivatives: [f64; 9] = [eq25_e938_d_n0, eq25_e938_d_n1, eq25_e938_d_n2, eq25_e938_d_n3, eq25_e938_d_n4, eq25_e938_d_n5, eq25_e938_d_n6, eq25_e938_d_n7, eq25_e938_d_n8];
        let eq25_branch_derivatives: [f64; 5] = [eq25_e938_d_b0, eq25_e938_d_b1, eq25_e938_d_b2, eq25_e938_d_b3, eq25_e938_d_b4];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq25_value),
            &eq25_node_derivatives,
            &eq25_branch_derivatives,
            multiplicity,
        );
        let (eq26_e943,) = {
    if (!s.b[664]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e943;
        stamper.stamp_potential_const_local(
            2,
            eq26_value,
        );
        let (eq27_e947,) = {
    if s.b[665] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq27_value: f64 = eq27_e947;
        stamper.stamp_potential_const_local(
            3,
            eq27_value,
        );
        let (eq29_e963,) = {
    if (!s.b[665]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e963;
        stamper.stamp_current_const_local(
            Some(1),
            Some(7),
            multiplicity * (eq29_value),
        );
        let eq30_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (eq30_value),
        );
        let eq31_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (eq31_value),
        );
        let (eq32_e985,) = {
    if (s.b[668] && s.b[669]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e985;
        stamper.stamp_current_const_local(
            Some(8),
            Some(6),
            multiplicity * (eq32_value),
        );
        let (eq33_e1000,) = {
    if (s.b[668] && s.b[669]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq33_value: f64 = eq33_e1000;
        stamper.stamp_current_const_local(
            Some(8),
            Some(5),
            multiplicity * (eq33_value),
        );
        let (eq34_e1016,) = {
    if (s.b[668] && (!s.b[669])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq34_value: f64 = eq34_e1016;
        stamper.stamp_current_const_local(
            Some(8),
            Some(5),
            multiplicity * (eq34_value),
        );
        let (eq35_e1032,) = {
    if (s.b[668] && (!s.b[669])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e1032;
        stamper.stamp_current_const_local(
            Some(8),
            Some(6),
            multiplicity * (eq35_value),
        );
        let (eq36_e1043,) = {
    if s.b[670] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq36_value: f64 = eq36_e1043;
        stamper.stamp_current_const_local(
            Some(8),
            Some(6),
            multiplicity * (eq36_value),
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq37_e1054,) = {
    if s.b[670] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq37_value: f64 = eq37_e1054;
        stamper.stamp_current_const_local(
            Some(8),
            Some(5),
            multiplicity * (eq37_value),
        );
        let (eq38_e1079, eq38_e1079_d_n0, eq38_e1079_d_n1, eq38_e1079_d_n2, eq38_e1079_d_n3, eq38_e1079_d_n4, eq38_e1079_d_n5, eq38_e1079_d_n6, eq38_e1079_d_n7, eq38_e1079_d_n8, eq38_e1079_d_b0, eq38_e1079_d_b1, eq38_e1079_d_b2, eq38_e1079_d_b3, eq38_e1079_d_b4,) = {
    if (s.b[671] && s.b[672]) {
        let eq38_e1060: f64 = (s.v[212] * s.v[27]);
        let eq38_e1060_d_n0: f64 = ((s.dn[212][0] * s.v[27]) + (s.v[212] * s.dn[27][0]));
        let eq38_e1060_d_n1: f64 = ((s.dn[212][1] * s.v[27]) + (s.v[212] * s.dn[27][1]));
        let eq38_e1060_d_n2: f64 = ((s.dn[212][2] * s.v[27]) + (s.v[212] * s.dn[27][2]));
        let eq38_e1060_d_n3: f64 = ((s.dn[212][3] * s.v[27]) + (s.v[212] * s.dn[27][3]));
        let eq38_e1060_d_n4: f64 = ((s.dn[212][4] * s.v[27]) + (s.v[212] * s.dn[27][4]));
        let eq38_e1060_d_n5: f64 = ((s.dn[212][5] * s.v[27]) + (s.v[212] * s.dn[27][5]));
        let eq38_e1060_d_n6: f64 = ((s.dn[212][6] * s.v[27]) + (s.v[212] * s.dn[27][6]));
        let eq38_e1060_d_n7: f64 = ((s.dn[212][7] * s.v[27]) + (s.v[212] * s.dn[27][7]));
        let eq38_e1060_d_n8: f64 = ((s.dn[212][8] * s.v[27]) + (s.v[212] * s.dn[27][8]));
        let eq38_e1060_d_b0: f64 = ((s.db[212][0] * s.v[27]) + (s.v[212] * s.db[27][0]));
        let eq38_e1060_d_b1: f64 = ((s.db[212][1] * s.v[27]) + (s.v[212] * s.db[27][1]));
        let eq38_e1060_d_b2: f64 = ((s.db[212][2] * s.v[27]) + (s.v[212] * s.db[27][2]));
        let eq38_e1060_d_b3: f64 = ((s.db[212][3] * s.v[27]) + (s.v[212] * s.db[27][3]));
        let eq38_e1060_d_b4: f64 = ((s.db[212][4] * s.v[27]) + (s.v[212] * s.db[27][4]));
        let eq38_e1062: f64 = (eq38_e1060 * (nv5 - nv6));
        let eq38_e1062_d_n0: f64 = (eq38_e1060_d_n0 * (nv5 - nv6));
        let eq38_e1062_d_n1: f64 = (eq38_e1060_d_n1 * (nv5 - nv6));
        let eq38_e1062_d_n2: f64 = (eq38_e1060_d_n2 * (nv5 - nv6));
        let eq38_e1062_d_n3: f64 = (eq38_e1060_d_n3 * (nv5 - nv6));
        let eq38_e1062_d_n4: f64 = (eq38_e1060_d_n4 * (nv5 - nv6));
        let eq38_e1062_d_n5: f64 = ((eq38_e1060_d_n5 * (nv5 - nv6)) + eq38_e1060);
        let eq38_e1062_d_n6: f64 = ((eq38_e1060_d_n6 * (nv5 - nv6)) + (-eq38_e1060));
        let eq38_e1062_d_n7: f64 = (eq38_e1060_d_n7 * (nv5 - nv6));
        let eq38_e1062_d_n8: f64 = (eq38_e1060_d_n8 * (nv5 - nv6));
        let eq38_e1062_d_b0: f64 = (eq38_e1060_d_b0 * (nv5 - nv6));
        let eq38_e1062_d_b1: f64 = (eq38_e1060_d_b1 * (nv5 - nv6));
        let eq38_e1062_d_b2: f64 = (eq38_e1060_d_b2 * (nv5 - nv6));
        let eq38_e1062_d_b3: f64 = (eq38_e1060_d_b3 * (nv5 - nv6));
        let eq38_e1062_d_b4: f64 = (eq38_e1060_d_b4 * (nv5 - nv6));
        let eq38_e1064: f64 = (eq38_e1062 * s.v[214]);
        let eq38_e1064_d_n0: f64 = ((eq38_e1062_d_n0 * s.v[214]) + (eq38_e1062 * s.dn[214][0]));
        let eq38_e1064_d_n1: f64 = ((eq38_e1062_d_n1 * s.v[214]) + (eq38_e1062 * s.dn[214][1]));
        let eq38_e1064_d_n2: f64 = ((eq38_e1062_d_n2 * s.v[214]) + (eq38_e1062 * s.dn[214][2]));
        let eq38_e1064_d_n3: f64 = ((eq38_e1062_d_n3 * s.v[214]) + (eq38_e1062 * s.dn[214][3]));
        let eq38_e1064_d_n4: f64 = ((eq38_e1062_d_n4 * s.v[214]) + (eq38_e1062 * s.dn[214][4]));
        let eq38_e1064_d_n5: f64 = ((eq38_e1062_d_n5 * s.v[214]) + (eq38_e1062 * s.dn[214][5]));
        let eq38_e1064_d_n6: f64 = ((eq38_e1062_d_n6 * s.v[214]) + (eq38_e1062 * s.dn[214][6]));
        let eq38_e1064_d_n7: f64 = ((eq38_e1062_d_n7 * s.v[214]) + (eq38_e1062 * s.dn[214][7]));
        let eq38_e1064_d_n8: f64 = ((eq38_e1062_d_n8 * s.v[214]) + (eq38_e1062 * s.dn[214][8]));
        let eq38_e1064_d_b0: f64 = ((eq38_e1062_d_b0 * s.v[214]) + (eq38_e1062 * s.db[214][0]));
        let eq38_e1064_d_b1: f64 = ((eq38_e1062_d_b1 * s.v[214]) + (eq38_e1062 * s.db[214][1]));
        let eq38_e1064_d_b2: f64 = ((eq38_e1062_d_b2 * s.v[214]) + (eq38_e1062 * s.db[214][2]));
        let eq38_e1064_d_b3: f64 = ((eq38_e1062_d_b3 * s.v[214]) + (eq38_e1062 * s.db[214][3]));
        let eq38_e1064_d_b4: f64 = ((eq38_e1062_d_b4 * s.v[214]) + (eq38_e1062 * s.db[214][4]));
        let eq38_e1067: f64 = ((nv0 - nv5) * (nv0 - nv5));
        let eq38_e1067_d_n0: f64 = ((nv0 - nv5) + (nv0 - nv5));
        let eq38_e1067_d_n5: f64 = ((-(nv0 - nv5)) + (-(nv0 - nv5)));
        let eq38_e1069: f64 = (eq38_e1067 / s.v[146]);
        let eq38_e1069_d_n0: f64 = (((eq38_e1067_d_n0 * s.v[146]) - (eq38_e1067 * s.dn[146][0])) / (s.v[146] * s.v[146]));
        let eq38_e1069_d_n1: f64 = (-((eq38_e1067 * s.dn[146][1]) / (s.v[146] * s.v[146])));
        let eq38_e1069_d_n2: f64 = (-((eq38_e1067 * s.dn[146][2]) / (s.v[146] * s.v[146])));
        let eq38_e1069_d_n3: f64 = (-((eq38_e1067 * s.dn[146][3]) / (s.v[146] * s.v[146])));
        let eq38_e1069_d_n4: f64 = (-((eq38_e1067 * s.dn[146][4]) / (s.v[146] * s.v[146])));
        let eq38_e1069_d_n5: f64 = (((eq38_e1067_d_n5 * s.v[146]) - (eq38_e1067 * s.dn[146][5])) / (s.v[146] * s.v[146]));
        let eq38_e1069_d_n6: f64 = (-((eq38_e1067 * s.dn[146][6]) / (s.v[146] * s.v[146])));
        let eq38_e1069_d_n7: f64 = (-((eq38_e1067 * s.dn[146][7]) / (s.v[146] * s.v[146])));
        let eq38_e1069_d_n8: f64 = (-((eq38_e1067 * s.dn[146][8]) / (s.v[146] * s.v[146])));
        let eq38_e1069_d_b0: f64 = (-((eq38_e1067 * s.db[146][0]) / (s.v[146] * s.v[146])));
        let eq38_e1069_d_b1: f64 = (-((eq38_e1067 * s.db[146][1]) / (s.v[146] * s.v[146])));
        let eq38_e1069_d_b2: f64 = (-((eq38_e1067 * s.db[146][2]) / (s.v[146] * s.v[146])));
        let eq38_e1069_d_b3: f64 = (-((eq38_e1067 * s.db[146][3]) / (s.v[146] * s.v[146])));
        let eq38_e1069_d_b4: f64 = (-((eq38_e1067 * s.db[146][4]) / (s.v[146] * s.v[146])));
        let eq38_e1070: f64 = (eq38_e1064 + eq38_e1069);
        let eq38_e1070_d_n0: f64 = (eq38_e1064_d_n0 + eq38_e1069_d_n0);
        let eq38_e1070_d_n1: f64 = (eq38_e1064_d_n1 + eq38_e1069_d_n1);
        let eq38_e1070_d_n2: f64 = (eq38_e1064_d_n2 + eq38_e1069_d_n2);
        let eq38_e1070_d_n3: f64 = (eq38_e1064_d_n3 + eq38_e1069_d_n3);
        let eq38_e1070_d_n4: f64 = (eq38_e1064_d_n4 + eq38_e1069_d_n4);
        let eq38_e1070_d_n5: f64 = (eq38_e1064_d_n5 + eq38_e1069_d_n5);
        let eq38_e1070_d_n6: f64 = (eq38_e1064_d_n6 + eq38_e1069_d_n6);
        let eq38_e1070_d_n7: f64 = (eq38_e1064_d_n7 + eq38_e1069_d_n7);
        let eq38_e1070_d_n8: f64 = (eq38_e1064_d_n8 + eq38_e1069_d_n8);
        let eq38_e1070_d_b0: f64 = (eq38_e1064_d_b0 + eq38_e1069_d_b0);
        let eq38_e1070_d_b1: f64 = (eq38_e1064_d_b1 + eq38_e1069_d_b1);
        let eq38_e1070_d_b2: f64 = (eq38_e1064_d_b2 + eq38_e1069_d_b2);
        let eq38_e1070_d_b3: f64 = (eq38_e1064_d_b3 + eq38_e1069_d_b3);
        let eq38_e1070_d_b4: f64 = (eq38_e1064_d_b4 + eq38_e1069_d_b4);
        let eq38_e1073: f64 = ((nv2 - nv6) * (nv2 - nv6));
        let eq38_e1073_d_n2: f64 = ((nv2 - nv6) + (nv2 - nv6));
        let eq38_e1073_d_n6: f64 = ((-(nv2 - nv6)) + (-(nv2 - nv6)));
        let eq38_e1075: f64 = (eq38_e1073 / s.v[147]);
        let eq38_e1075_d_n0: f64 = (-((eq38_e1073 * s.dn[147][0]) / (s.v[147] * s.v[147])));
        let eq38_e1075_d_n1: f64 = (-((eq38_e1073 * s.dn[147][1]) / (s.v[147] * s.v[147])));
        let eq38_e1075_d_n2: f64 = (((eq38_e1073_d_n2 * s.v[147]) - (eq38_e1073 * s.dn[147][2])) / (s.v[147] * s.v[147]));
        let eq38_e1075_d_n3: f64 = (-((eq38_e1073 * s.dn[147][3]) / (s.v[147] * s.v[147])));
        let eq38_e1075_d_n4: f64 = (-((eq38_e1073 * s.dn[147][4]) / (s.v[147] * s.v[147])));
        let eq38_e1075_d_n5: f64 = (-((eq38_e1073 * s.dn[147][5]) / (s.v[147] * s.v[147])));
        let eq38_e1075_d_n6: f64 = (((eq38_e1073_d_n6 * s.v[147]) - (eq38_e1073 * s.dn[147][6])) / (s.v[147] * s.v[147]));
        let eq38_e1075_d_n7: f64 = (-((eq38_e1073 * s.dn[147][7]) / (s.v[147] * s.v[147])));
        let eq38_e1075_d_n8: f64 = (-((eq38_e1073 * s.dn[147][8]) / (s.v[147] * s.v[147])));
        let eq38_e1075_d_b0: f64 = (-((eq38_e1073 * s.db[147][0]) / (s.v[147] * s.v[147])));
        let eq38_e1075_d_b1: f64 = (-((eq38_e1073 * s.db[147][1]) / (s.v[147] * s.v[147])));
        let eq38_e1075_d_b2: f64 = (-((eq38_e1073 * s.db[147][2]) / (s.v[147] * s.v[147])));
        let eq38_e1075_d_b3: f64 = (-((eq38_e1073 * s.db[147][3]) / (s.v[147] * s.v[147])));
        let eq38_e1075_d_b4: f64 = (-((eq38_e1073 * s.db[147][4]) / (s.v[147] * s.v[147])));
        let eq38_e1076: f64 = (eq38_e1070 + eq38_e1075);
        let eq38_e1076_d_n0: f64 = (eq38_e1070_d_n0 + eq38_e1075_d_n0);
        let eq38_e1076_d_n1: f64 = (eq38_e1070_d_n1 + eq38_e1075_d_n1);
        let eq38_e1076_d_n2: f64 = (eq38_e1070_d_n2 + eq38_e1075_d_n2);
        let eq38_e1076_d_n3: f64 = (eq38_e1070_d_n3 + eq38_e1075_d_n3);
        let eq38_e1076_d_n4: f64 = (eq38_e1070_d_n4 + eq38_e1075_d_n4);
        let eq38_e1076_d_n5: f64 = (eq38_e1070_d_n5 + eq38_e1075_d_n5);
        let eq38_e1076_d_n6: f64 = (eq38_e1070_d_n6 + eq38_e1075_d_n6);
        let eq38_e1076_d_n7: f64 = (eq38_e1070_d_n7 + eq38_e1075_d_n7);
        let eq38_e1076_d_n8: f64 = (eq38_e1070_d_n8 + eq38_e1075_d_n8);
        let eq38_e1076_d_b0: f64 = (eq38_e1070_d_b0 + eq38_e1075_d_b0);
        let eq38_e1076_d_b1: f64 = (eq38_e1070_d_b1 + eq38_e1075_d_b1);
        let eq38_e1076_d_b2: f64 = (eq38_e1070_d_b2 + eq38_e1075_d_b2);
        let eq38_e1076_d_b3: f64 = (eq38_e1070_d_b3 + eq38_e1075_d_b3);
        let eq38_e1076_d_b4: f64 = (eq38_e1070_d_b4 + eq38_e1075_d_b4);
        let eq38_e1077: f64 = (-eq38_e1076);
        let eq38_e1077_d_n0: f64 = (-eq38_e1076_d_n0);
        let eq38_e1077_d_n1: f64 = (-eq38_e1076_d_n1);
        let eq38_e1077_d_n2: f64 = (-eq38_e1076_d_n2);
        let eq38_e1077_d_n3: f64 = (-eq38_e1076_d_n3);
        let eq38_e1077_d_n4: f64 = (-eq38_e1076_d_n4);
        let eq38_e1077_d_n5: f64 = (-eq38_e1076_d_n5);
        let eq38_e1077_d_n6: f64 = (-eq38_e1076_d_n6);
        let eq38_e1077_d_n7: f64 = (-eq38_e1076_d_n7);
        let eq38_e1077_d_n8: f64 = (-eq38_e1076_d_n8);
        let eq38_e1077_d_b0: f64 = (-eq38_e1076_d_b0);
        let eq38_e1077_d_b1: f64 = (-eq38_e1076_d_b1);
        let eq38_e1077_d_b2: f64 = (-eq38_e1076_d_b2);
        let eq38_e1077_d_b3: f64 = (-eq38_e1076_d_b3);
        let eq38_e1077_d_b4: f64 = (-eq38_e1076_d_b4);
        (eq38_e1077, eq38_e1077_d_n0, eq38_e1077_d_n1, eq38_e1077_d_n2, eq38_e1077_d_n3, eq38_e1077_d_n4, eq38_e1077_d_n5, eq38_e1077_d_n6, eq38_e1077_d_n7, eq38_e1077_d_n8, eq38_e1077_d_b0, eq38_e1077_d_b1, eq38_e1077_d_b2, eq38_e1077_d_b3, eq38_e1077_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e1079;
        let eq38_node_derivatives: [f64; 9] = [eq38_e1079_d_n0, eq38_e1079_d_n1, eq38_e1079_d_n2, eq38_e1079_d_n3, eq38_e1079_d_n4, eq38_e1079_d_n5, eq38_e1079_d_n6, eq38_e1079_d_n7, eq38_e1079_d_n8];
        let eq38_branch_derivatives: [f64; 5] = [eq38_e1079_d_b0, eq38_e1079_d_b1, eq38_e1079_d_b2, eq38_e1079_d_b3, eq38_e1079_d_b4];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq38_value),
            &eq38_node_derivatives,
            &eq38_branch_derivatives,
            multiplicity,
        );
        let (eq39_e1093, eq39_e1093_d_n0, eq39_e1093_d_n1, eq39_e1093_d_n2, eq39_e1093_d_n3, eq39_e1093_d_n4, eq39_e1093_d_n5, eq39_e1093_d_n6, eq39_e1093_d_n7, eq39_e1093_d_n8, eq39_e1093_d_b0, eq39_e1093_d_b1, eq39_e1093_d_b2, eq39_e1093_d_b3, eq39_e1093_d_b4,) = {
    if (s.b[671] && (!s.b[672])) {
        let eq39_e1086: f64 = (s.v[212] * s.v[27]);
        let eq39_e1086_d_n0: f64 = ((s.dn[212][0] * s.v[27]) + (s.v[212] * s.dn[27][0]));
        let eq39_e1086_d_n1: f64 = ((s.dn[212][1] * s.v[27]) + (s.v[212] * s.dn[27][1]));
        let eq39_e1086_d_n2: f64 = ((s.dn[212][2] * s.v[27]) + (s.v[212] * s.dn[27][2]));
        let eq39_e1086_d_n3: f64 = ((s.dn[212][3] * s.v[27]) + (s.v[212] * s.dn[27][3]));
        let eq39_e1086_d_n4: f64 = ((s.dn[212][4] * s.v[27]) + (s.v[212] * s.dn[27][4]));
        let eq39_e1086_d_n5: f64 = ((s.dn[212][5] * s.v[27]) + (s.v[212] * s.dn[27][5]));
        let eq39_e1086_d_n6: f64 = ((s.dn[212][6] * s.v[27]) + (s.v[212] * s.dn[27][6]));
        let eq39_e1086_d_n7: f64 = ((s.dn[212][7] * s.v[27]) + (s.v[212] * s.dn[27][7]));
        let eq39_e1086_d_n8: f64 = ((s.dn[212][8] * s.v[27]) + (s.v[212] * s.dn[27][8]));
        let eq39_e1086_d_b0: f64 = ((s.db[212][0] * s.v[27]) + (s.v[212] * s.db[27][0]));
        let eq39_e1086_d_b1: f64 = ((s.db[212][1] * s.v[27]) + (s.v[212] * s.db[27][1]));
        let eq39_e1086_d_b2: f64 = ((s.db[212][2] * s.v[27]) + (s.v[212] * s.db[27][2]));
        let eq39_e1086_d_b3: f64 = ((s.db[212][3] * s.v[27]) + (s.v[212] * s.db[27][3]));
        let eq39_e1086_d_b4: f64 = ((s.db[212][4] * s.v[27]) + (s.v[212] * s.db[27][4]));
        let eq39_e1088: f64 = (eq39_e1086 * (nv5 - nv6));
        let eq39_e1088_d_n0: f64 = (eq39_e1086_d_n0 * (nv5 - nv6));
        let eq39_e1088_d_n1: f64 = (eq39_e1086_d_n1 * (nv5 - nv6));
        let eq39_e1088_d_n2: f64 = (eq39_e1086_d_n2 * (nv5 - nv6));
        let eq39_e1088_d_n3: f64 = (eq39_e1086_d_n3 * (nv5 - nv6));
        let eq39_e1088_d_n4: f64 = (eq39_e1086_d_n4 * (nv5 - nv6));
        let eq39_e1088_d_n5: f64 = ((eq39_e1086_d_n5 * (nv5 - nv6)) + eq39_e1086);
        let eq39_e1088_d_n6: f64 = ((eq39_e1086_d_n6 * (nv5 - nv6)) + (-eq39_e1086));
        let eq39_e1088_d_n7: f64 = (eq39_e1086_d_n7 * (nv5 - nv6));
        let eq39_e1088_d_n8: f64 = (eq39_e1086_d_n8 * (nv5 - nv6));
        let eq39_e1088_d_b0: f64 = (eq39_e1086_d_b0 * (nv5 - nv6));
        let eq39_e1088_d_b1: f64 = (eq39_e1086_d_b1 * (nv5 - nv6));
        let eq39_e1088_d_b2: f64 = (eq39_e1086_d_b2 * (nv5 - nv6));
        let eq39_e1088_d_b3: f64 = (eq39_e1086_d_b3 * (nv5 - nv6));
        let eq39_e1088_d_b4: f64 = (eq39_e1086_d_b4 * (nv5 - nv6));
        let eq39_e1090: f64 = (eq39_e1088 * s.v[214]);
        let eq39_e1090_d_n0: f64 = ((eq39_e1088_d_n0 * s.v[214]) + (eq39_e1088 * s.dn[214][0]));
        let eq39_e1090_d_n1: f64 = ((eq39_e1088_d_n1 * s.v[214]) + (eq39_e1088 * s.dn[214][1]));
        let eq39_e1090_d_n2: f64 = ((eq39_e1088_d_n2 * s.v[214]) + (eq39_e1088 * s.dn[214][2]));
        let eq39_e1090_d_n3: f64 = ((eq39_e1088_d_n3 * s.v[214]) + (eq39_e1088 * s.dn[214][3]));
        let eq39_e1090_d_n4: f64 = ((eq39_e1088_d_n4 * s.v[214]) + (eq39_e1088 * s.dn[214][4]));
        let eq39_e1090_d_n5: f64 = ((eq39_e1088_d_n5 * s.v[214]) + (eq39_e1088 * s.dn[214][5]));
        let eq39_e1090_d_n6: f64 = ((eq39_e1088_d_n6 * s.v[214]) + (eq39_e1088 * s.dn[214][6]));
        let eq39_e1090_d_n7: f64 = ((eq39_e1088_d_n7 * s.v[214]) + (eq39_e1088 * s.dn[214][7]));
        let eq39_e1090_d_n8: f64 = ((eq39_e1088_d_n8 * s.v[214]) + (eq39_e1088 * s.dn[214][8]));
        let eq39_e1090_d_b0: f64 = ((eq39_e1088_d_b0 * s.v[214]) + (eq39_e1088 * s.db[214][0]));
        let eq39_e1090_d_b1: f64 = ((eq39_e1088_d_b1 * s.v[214]) + (eq39_e1088 * s.db[214][1]));
        let eq39_e1090_d_b2: f64 = ((eq39_e1088_d_b2 * s.v[214]) + (eq39_e1088 * s.db[214][2]));
        let eq39_e1090_d_b3: f64 = ((eq39_e1088_d_b3 * s.v[214]) + (eq39_e1088 * s.db[214][3]));
        let eq39_e1090_d_b4: f64 = ((eq39_e1088_d_b4 * s.v[214]) + (eq39_e1088 * s.db[214][4]));
        let eq39_e1091: f64 = (-eq39_e1090);
        let eq39_e1091_d_n0: f64 = (-eq39_e1090_d_n0);
        let eq39_e1091_d_n1: f64 = (-eq39_e1090_d_n1);
        let eq39_e1091_d_n2: f64 = (-eq39_e1090_d_n2);
        let eq39_e1091_d_n3: f64 = (-eq39_e1090_d_n3);
        let eq39_e1091_d_n4: f64 = (-eq39_e1090_d_n4);
        let eq39_e1091_d_n5: f64 = (-eq39_e1090_d_n5);
        let eq39_e1091_d_n6: f64 = (-eq39_e1090_d_n6);
        let eq39_e1091_d_n7: f64 = (-eq39_e1090_d_n7);
        let eq39_e1091_d_n8: f64 = (-eq39_e1090_d_n8);
        let eq39_e1091_d_b0: f64 = (-eq39_e1090_d_b0);
        let eq39_e1091_d_b1: f64 = (-eq39_e1090_d_b1);
        let eq39_e1091_d_b2: f64 = (-eq39_e1090_d_b2);
        let eq39_e1091_d_b3: f64 = (-eq39_e1090_d_b3);
        let eq39_e1091_d_b4: f64 = (-eq39_e1090_d_b4);
        (eq39_e1091, eq39_e1091_d_n0, eq39_e1091_d_n1, eq39_e1091_d_n2, eq39_e1091_d_n3, eq39_e1091_d_n4, eq39_e1091_d_n5, eq39_e1091_d_n6, eq39_e1091_d_n7, eq39_e1091_d_n8, eq39_e1091_d_b0, eq39_e1091_d_b1, eq39_e1091_d_b2, eq39_e1091_d_b3, eq39_e1091_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e1093;
        let eq39_node_derivatives: [f64; 9] = [eq39_e1093_d_n0, eq39_e1093_d_n1, eq39_e1093_d_n2, eq39_e1093_d_n3, eq39_e1093_d_n4, eq39_e1093_d_n5, eq39_e1093_d_n6, eq39_e1093_d_n7, eq39_e1093_d_n8];
        let eq39_branch_derivatives: [f64; 5] = [eq39_e1093_d_b0, eq39_e1093_d_b1, eq39_e1093_d_b2, eq39_e1093_d_b3, eq39_e1093_d_b4];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq39_value),
            &eq39_node_derivatives,
            &eq39_branch_derivatives,
            multiplicity,
        );
        let (eq40_e1099, eq40_e1099_d_n0, eq40_e1099_d_n1, eq40_e1099_d_n2, eq40_e1099_d_n3, eq40_e1099_d_n4, eq40_e1099_d_n5, eq40_e1099_d_n6, eq40_e1099_d_n7, eq40_e1099_d_n8, eq40_e1099_d_b0, eq40_e1099_d_b1, eq40_e1099_d_b2, eq40_e1099_d_b3, eq40_e1099_d_b4,) = {
    if s.b[671] {
        let eq40_e1097: f64 = ((nv4 - 0.0) * s.v[269]);
        let eq40_e1097_d_n0: f64 = ((nv4 - 0.0) * s.dn[269][0]);
        let eq40_e1097_d_n1: f64 = ((nv4 - 0.0) * s.dn[269][1]);
        let eq40_e1097_d_n2: f64 = ((nv4 - 0.0) * s.dn[269][2]);
        let eq40_e1097_d_n3: f64 = ((nv4 - 0.0) * s.dn[269][3]);
        let eq40_e1097_d_n4: f64 = (s.v[269] + ((nv4 - 0.0) * s.dn[269][4]));
        let eq40_e1097_d_n5: f64 = ((nv4 - 0.0) * s.dn[269][5]);
        let eq40_e1097_d_n6: f64 = ((nv4 - 0.0) * s.dn[269][6]);
        let eq40_e1097_d_n7: f64 = ((nv4 - 0.0) * s.dn[269][7]);
        let eq40_e1097_d_n8: f64 = ((nv4 - 0.0) * s.dn[269][8]);
        let eq40_e1097_d_b0: f64 = ((nv4 - 0.0) * s.db[269][0]);
        let eq40_e1097_d_b1: f64 = ((nv4 - 0.0) * s.db[269][1]);
        let eq40_e1097_d_b2: f64 = ((nv4 - 0.0) * s.db[269][2]);
        let eq40_e1097_d_b3: f64 = ((nv4 - 0.0) * s.db[269][3]);
        let eq40_e1097_d_b4: f64 = ((nv4 - 0.0) * s.db[269][4]);
        (eq40_e1097, eq40_e1097_d_n0, eq40_e1097_d_n1, eq40_e1097_d_n2, eq40_e1097_d_n3, eq40_e1097_d_n4, eq40_e1097_d_n5, eq40_e1097_d_n6, eq40_e1097_d_n7, eq40_e1097_d_n8, eq40_e1097_d_b0, eq40_e1097_d_b1, eq40_e1097_d_b2, eq40_e1097_d_b3, eq40_e1097_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_value: f64 = eq40_e1099;
        let eq40_node_derivatives: [f64; 9] = [eq40_e1099_d_n0, eq40_e1099_d_n1, eq40_e1099_d_n2, eq40_e1099_d_n3, eq40_e1099_d_n4, eq40_e1099_d_n5, eq40_e1099_d_n6, eq40_e1099_d_n7, eq40_e1099_d_n8];
        let eq40_branch_derivatives: [f64; 5] = [eq40_e1099_d_b0, eq40_e1099_d_b1, eq40_e1099_d_b2, eq40_e1099_d_b3, eq40_e1099_d_b4];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq40_value),
            &eq40_node_derivatives,
            &eq40_branch_derivatives,
            multiplicity,
        );
        let (eq41_e1106, eq41_e1106_d_n0, eq41_e1106_d_n1, eq41_e1106_d_n2, eq41_e1106_d_n3, eq41_e1106_d_n4, eq41_e1106_d_n5, eq41_e1106_d_n6, eq41_e1106_d_n7, eq41_e1106_d_n8, eq41_e1106_d_b0, eq41_e1106_d_b1, eq41_e1106_d_b2, eq41_e1106_d_b3, eq41_e1106_d_b4,) = {
    if s.b[671] {
        let eq41_e1103: f64 = ((nv4 - 0.0) * s.v[270]);
        let eq41_e1103_d_n0: f64 = ((nv4 - 0.0) * s.dn[270][0]);
        let eq41_e1103_d_n1: f64 = ((nv4 - 0.0) * s.dn[270][1]);
        let eq41_e1103_d_n2: f64 = ((nv4 - 0.0) * s.dn[270][2]);
        let eq41_e1103_d_n3: f64 = ((nv4 - 0.0) * s.dn[270][3]);
        let eq41_e1103_d_n4: f64 = (s.v[270] + ((nv4 - 0.0) * s.dn[270][4]));
        let eq41_e1103_d_n5: f64 = ((nv4 - 0.0) * s.dn[270][5]);
        let eq41_e1103_d_n6: f64 = ((nv4 - 0.0) * s.dn[270][6]);
        let eq41_e1103_d_n7: f64 = ((nv4 - 0.0) * s.dn[270][7]);
        let eq41_e1103_d_n8: f64 = ((nv4 - 0.0) * s.dn[270][8]);
        let eq41_e1103_d_b0: f64 = ((nv4 - 0.0) * s.db[270][0]);
        let eq41_e1103_d_b1: f64 = ((nv4 - 0.0) * s.db[270][1]);
        let eq41_e1103_d_b2: f64 = ((nv4 - 0.0) * s.db[270][2]);
        let eq41_e1103_d_b3: f64 = ((nv4 - 0.0) * s.db[270][3]);
        let eq41_e1103_d_b4: f64 = ((nv4 - 0.0) * s.db[270][4]);
        let eq41_e1104: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, eq41_e1103);
        let eq41_e1104_d_n0: f64 = (eq41_e1103_d_n0 * ddt_scale);
        let eq41_e1104_d_n1: f64 = (eq41_e1103_d_n1 * ddt_scale);
        let eq41_e1104_d_n2: f64 = (eq41_e1103_d_n2 * ddt_scale);
        let eq41_e1104_d_n3: f64 = (eq41_e1103_d_n3 * ddt_scale);
        let eq41_e1104_d_n4: f64 = (eq41_e1103_d_n4 * ddt_scale);
        let eq41_e1104_d_n5: f64 = (eq41_e1103_d_n5 * ddt_scale);
        let eq41_e1104_d_n6: f64 = (eq41_e1103_d_n6 * ddt_scale);
        let eq41_e1104_d_n7: f64 = (eq41_e1103_d_n7 * ddt_scale);
        let eq41_e1104_d_n8: f64 = (eq41_e1103_d_n8 * ddt_scale);
        let eq41_e1104_d_b0: f64 = (eq41_e1103_d_b0 * ddt_scale);
        let eq41_e1104_d_b1: f64 = (eq41_e1103_d_b1 * ddt_scale);
        let eq41_e1104_d_b2: f64 = (eq41_e1103_d_b2 * ddt_scale);
        let eq41_e1104_d_b3: f64 = (eq41_e1103_d_b3 * ddt_scale);
        let eq41_e1104_d_b4: f64 = (eq41_e1103_d_b4 * ddt_scale);
        (eq41_e1104, eq41_e1104_d_n0, eq41_e1104_d_n1, eq41_e1104_d_n2, eq41_e1104_d_n3, eq41_e1104_d_n4, eq41_e1104_d_n5, eq41_e1104_d_n6, eq41_e1104_d_n7, eq41_e1104_d_n8, eq41_e1104_d_b0, eq41_e1104_d_b1, eq41_e1104_d_b2, eq41_e1104_d_b3, eq41_e1104_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_value: f64 = eq41_e1106;
        let eq41_node_derivatives: [f64; 9] = [eq41_e1106_d_n0, eq41_e1106_d_n1, eq41_e1106_d_n2, eq41_e1106_d_n3, eq41_e1106_d_n4, eq41_e1106_d_n5, eq41_e1106_d_n6, eq41_e1106_d_n7, eq41_e1106_d_n8];
        let eq41_branch_derivatives: [f64; 5] = [eq41_e1106_d_b0, eq41_e1106_d_b1, eq41_e1106_d_b2, eq41_e1106_d_b3, eq41_e1106_d_b4];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq41_value),
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let (eq42_e1111,) = {
    if (!s.b[671]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq42_value: f64 = eq42_e1111;
        stamper.stamp_potential_const_local(
            4,
            eq42_value,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq12_e873_q: f64 = s.v[223];
        let eq12_e874: f64 = (s.v[212] * s.v[223]);
        let eq12_e874_d_n0: f64 = ((s.dn[212][0] * s.v[223]) + (s.v[212] * s.dn[223][0]));
        let eq12_e874_d_n1: f64 = ((s.dn[212][1] * s.v[223]) + (s.v[212] * s.dn[223][1]));
        let eq12_e874_d_n2: f64 = ((s.dn[212][2] * s.v[223]) + (s.v[212] * s.dn[223][2]));
        let eq12_e874_d_n3: f64 = ((s.dn[212][3] * s.v[223]) + (s.v[212] * s.dn[223][3]));
        let eq12_e874_d_n4: f64 = ((s.dn[212][4] * s.v[223]) + (s.v[212] * s.dn[223][4]));
        let eq12_e874_d_n5: f64 = ((s.dn[212][5] * s.v[223]) + (s.v[212] * s.dn[223][5]));
        let eq12_e874_d_n6: f64 = ((s.dn[212][6] * s.v[223]) + (s.v[212] * s.dn[223][6]));
        let eq12_e874_d_n7: f64 = ((s.dn[212][7] * s.v[223]) + (s.v[212] * s.dn[223][7]));
        let eq12_e874_d_n8: f64 = ((s.dn[212][8] * s.v[223]) + (s.v[212] * s.dn[223][8]));
        let eq12_e874_d_b0: f64 = ((s.db[212][0] * s.v[223]) + (s.v[212] * s.db[223][0]));
        let eq12_e874_d_b1: f64 = ((s.db[212][1] * s.v[223]) + (s.v[212] * s.db[223][1]));
        let eq12_e874_d_b2: f64 = ((s.db[212][2] * s.v[223]) + (s.v[212] * s.db[223][2]));
        let eq12_e874_d_b3: f64 = ((s.db[212][3] * s.v[223]) + (s.v[212] * s.db[223][3]));
        let eq12_e874_d_b4: f64 = ((s.db[212][4] * s.v[223]) + (s.v[212] * s.db[223][4]));
        let eq12_e874_q: f64 = (s.v[212] * eq12_e873_q);
        let eq12_e874_q_d_n0: f64 = ((s.dn[212][0] * eq12_e873_q) + (s.v[212] * s.dn[223][0]));
        let eq12_e874_q_d_n1: f64 = ((s.dn[212][1] * eq12_e873_q) + (s.v[212] * s.dn[223][1]));
        let eq12_e874_q_d_n2: f64 = ((s.dn[212][2] * eq12_e873_q) + (s.v[212] * s.dn[223][2]));
        let eq12_e874_q_d_n3: f64 = ((s.dn[212][3] * eq12_e873_q) + (s.v[212] * s.dn[223][3]));
        let eq12_e874_q_d_n4: f64 = ((s.dn[212][4] * eq12_e873_q) + (s.v[212] * s.dn[223][4]));
        let eq12_e874_q_d_n5: f64 = ((s.dn[212][5] * eq12_e873_q) + (s.v[212] * s.dn[223][5]));
        let eq12_e874_q_d_n6: f64 = ((s.dn[212][6] * eq12_e873_q) + (s.v[212] * s.dn[223][6]));
        let eq12_e874_q_d_n7: f64 = ((s.dn[212][7] * eq12_e873_q) + (s.v[212] * s.dn[223][7]));
        let eq12_e874_q_d_n8: f64 = ((s.dn[212][8] * eq12_e873_q) + (s.v[212] * s.dn[223][8]));
        let eq12_e874_q_d_b0: f64 = ((s.db[212][0] * eq12_e873_q) + (s.v[212] * s.db[223][0]));
        let eq12_e874_q_d_b1: f64 = ((s.db[212][1] * eq12_e873_q) + (s.v[212] * s.db[223][1]));
        let eq12_e874_q_d_b2: f64 = ((s.db[212][2] * eq12_e873_q) + (s.v[212] * s.db[223][2]));
        let eq12_e874_q_d_b3: f64 = ((s.db[212][3] * eq12_e873_q) + (s.v[212] * s.db[223][3]));
        let eq12_e874_q_d_b4: f64 = ((s.db[212][4] * eq12_e873_q) + (s.v[212] * s.db[223][4]));
        let eq12_reactive_node_derivatives: [f64; 9] = [eq12_e874_q_d_n0, eq12_e874_q_d_n1, eq12_e874_q_d_n2, eq12_e874_q_d_n3, eq12_e874_q_d_n4, eq12_e874_q_d_n5, eq12_e874_q_d_n6, eq12_e874_q_d_n7, eq12_e874_q_d_n8];
        let eq12_reactive_branch_derivatives: [f64; 5] = [eq12_e874_q_d_b0, eq12_e874_q_d_b1, eq12_e874_q_d_b2, eq12_e874_q_d_b3, eq12_e874_q_d_b4];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq13_e876_q: f64 = s.v[224];
        let eq13_reactive_node_derivatives: [f64; 9] = [s.dn[224][0], s.dn[224][1], s.dn[224][2], s.dn[224][3], s.dn[224][4], s.dn[224][5], s.dn[224][6], s.dn[224][7], s.dn[224][8]];
        let eq13_reactive_branch_derivatives: [f64; 5] = [s.db[224][0], s.db[224][1], s.db[224][2], s.db[224][3], s.db[224][4]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq13_reactive_node_derivatives,
            branches,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );
        let eq14_e879_q: f64 = s.v[225];
        let eq14_e880: f64 = (s.v[212] * s.v[225]);
        let eq14_e880_d_n0: f64 = ((s.dn[212][0] * s.v[225]) + (s.v[212] * s.dn[225][0]));
        let eq14_e880_d_n1: f64 = ((s.dn[212][1] * s.v[225]) + (s.v[212] * s.dn[225][1]));
        let eq14_e880_d_n2: f64 = ((s.dn[212][2] * s.v[225]) + (s.v[212] * s.dn[225][2]));
        let eq14_e880_d_n3: f64 = ((s.dn[212][3] * s.v[225]) + (s.v[212] * s.dn[225][3]));
        let eq14_e880_d_n4: f64 = ((s.dn[212][4] * s.v[225]) + (s.v[212] * s.dn[225][4]));
        let eq14_e880_d_n5: f64 = ((s.dn[212][5] * s.v[225]) + (s.v[212] * s.dn[225][5]));
        let eq14_e880_d_n6: f64 = ((s.dn[212][6] * s.v[225]) + (s.v[212] * s.dn[225][6]));
        let eq14_e880_d_n7: f64 = ((s.dn[212][7] * s.v[225]) + (s.v[212] * s.dn[225][7]));
        let eq14_e880_d_n8: f64 = ((s.dn[212][8] * s.v[225]) + (s.v[212] * s.dn[225][8]));
        let eq14_e880_d_b0: f64 = ((s.db[212][0] * s.v[225]) + (s.v[212] * s.db[225][0]));
        let eq14_e880_d_b1: f64 = ((s.db[212][1] * s.v[225]) + (s.v[212] * s.db[225][1]));
        let eq14_e880_d_b2: f64 = ((s.db[212][2] * s.v[225]) + (s.v[212] * s.db[225][2]));
        let eq14_e880_d_b3: f64 = ((s.db[212][3] * s.v[225]) + (s.v[212] * s.db[225][3]));
        let eq14_e880_d_b4: f64 = ((s.db[212][4] * s.v[225]) + (s.v[212] * s.db[225][4]));
        let eq14_e880_q: f64 = (s.v[212] * eq14_e879_q);
        let eq14_e880_q_d_n0: f64 = ((s.dn[212][0] * eq14_e879_q) + (s.v[212] * s.dn[225][0]));
        let eq14_e880_q_d_n1: f64 = ((s.dn[212][1] * eq14_e879_q) + (s.v[212] * s.dn[225][1]));
        let eq14_e880_q_d_n2: f64 = ((s.dn[212][2] * eq14_e879_q) + (s.v[212] * s.dn[225][2]));
        let eq14_e880_q_d_n3: f64 = ((s.dn[212][3] * eq14_e879_q) + (s.v[212] * s.dn[225][3]));
        let eq14_e880_q_d_n4: f64 = ((s.dn[212][4] * eq14_e879_q) + (s.v[212] * s.dn[225][4]));
        let eq14_e880_q_d_n5: f64 = ((s.dn[212][5] * eq14_e879_q) + (s.v[212] * s.dn[225][5]));
        let eq14_e880_q_d_n6: f64 = ((s.dn[212][6] * eq14_e879_q) + (s.v[212] * s.dn[225][6]));
        let eq14_e880_q_d_n7: f64 = ((s.dn[212][7] * eq14_e879_q) + (s.v[212] * s.dn[225][7]));
        let eq14_e880_q_d_n8: f64 = ((s.dn[212][8] * eq14_e879_q) + (s.v[212] * s.dn[225][8]));
        let eq14_e880_q_d_b0: f64 = ((s.db[212][0] * eq14_e879_q) + (s.v[212] * s.db[225][0]));
        let eq14_e880_q_d_b1: f64 = ((s.db[212][1] * eq14_e879_q) + (s.v[212] * s.db[225][1]));
        let eq14_e880_q_d_b2: f64 = ((s.db[212][2] * eq14_e879_q) + (s.v[212] * s.db[225][2]));
        let eq14_e880_q_d_b3: f64 = ((s.db[212][3] * eq14_e879_q) + (s.v[212] * s.db[225][3]));
        let eq14_e880_q_d_b4: f64 = ((s.db[212][4] * eq14_e879_q) + (s.v[212] * s.db[225][4]));
        let eq14_reactive_node_derivatives: [f64; 9] = [eq14_e880_q_d_n0, eq14_e880_q_d_n1, eq14_e880_q_d_n2, eq14_e880_q_d_n3, eq14_e880_q_d_n4, eq14_e880_q_d_n5, eq14_e880_q_d_n6, eq14_e880_q_d_n7, eq14_e880_q_d_n8];
        let eq14_reactive_branch_derivatives: [f64; 5] = [eq14_e880_q_d_b0, eq14_e880_q_d_b1, eq14_e880_q_d_b2, eq14_e880_q_d_b3, eq14_e880_q_d_b4];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[6]),
            nodes,
            &eq14_reactive_node_derivatives,
            branches,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
        let eq15_e882_q: f64 = s.v[226];
        let eq15_reactive_node_derivatives: [f64; 9] = [s.dn[226][0], s.dn[226][1], s.dn[226][2], s.dn[226][3], s.dn[226][4], s.dn[226][5], s.dn[226][6], s.dn[226][7], s.dn[226][8]];
        let eq15_reactive_branch_derivatives: [f64; 5] = [s.db[226][0], s.db[226][1], s.db[226][2], s.db[226][3], s.db[226][4]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq16_e884_q: f64 = s.v[227];
        let eq16_reactive_node_derivatives: [f64; 9] = [s.dn[227][0], s.dn[227][1], s.dn[227][2], s.dn[227][3], s.dn[227][4], s.dn[227][5], s.dn[227][6], s.dn[227][7], s.dn[227][8]];
        let eq16_reactive_branch_derivatives: [f64; 5] = [s.db[227][0], s.db[227][1], s.db[227][2], s.db[227][3], s.db[227][4]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq16_reactive_node_derivatives,
            branches,
            &eq16_reactive_branch_derivatives,
            multiplicity,
        );
        let eq17_e887_q: f64 = s.v[238];
        let eq17_e888: f64 = (s.v[212] * s.v[238]);
        let eq17_e888_d_n0: f64 = ((s.dn[212][0] * s.v[238]) + (s.v[212] * s.dn[238][0]));
        let eq17_e888_d_n1: f64 = ((s.dn[212][1] * s.v[238]) + (s.v[212] * s.dn[238][1]));
        let eq17_e888_d_n2: f64 = ((s.dn[212][2] * s.v[238]) + (s.v[212] * s.dn[238][2]));
        let eq17_e888_d_n3: f64 = ((s.dn[212][3] * s.v[238]) + (s.v[212] * s.dn[238][3]));
        let eq17_e888_d_n4: f64 = ((s.dn[212][4] * s.v[238]) + (s.v[212] * s.dn[238][4]));
        let eq17_e888_d_n5: f64 = ((s.dn[212][5] * s.v[238]) + (s.v[212] * s.dn[238][5]));
        let eq17_e888_d_n6: f64 = ((s.dn[212][6] * s.v[238]) + (s.v[212] * s.dn[238][6]));
        let eq17_e888_d_n7: f64 = ((s.dn[212][7] * s.v[238]) + (s.v[212] * s.dn[238][7]));
        let eq17_e888_d_n8: f64 = ((s.dn[212][8] * s.v[238]) + (s.v[212] * s.dn[238][8]));
        let eq17_e888_d_b0: f64 = ((s.db[212][0] * s.v[238]) + (s.v[212] * s.db[238][0]));
        let eq17_e888_d_b1: f64 = ((s.db[212][1] * s.v[238]) + (s.v[212] * s.db[238][1]));
        let eq17_e888_d_b2: f64 = ((s.db[212][2] * s.v[238]) + (s.v[212] * s.db[238][2]));
        let eq17_e888_d_b3: f64 = ((s.db[212][3] * s.v[238]) + (s.v[212] * s.db[238][3]));
        let eq17_e888_d_b4: f64 = ((s.db[212][4] * s.v[238]) + (s.v[212] * s.db[238][4]));
        let eq17_e888_q: f64 = (s.v[212] * eq17_e887_q);
        let eq17_e888_q_d_n0: f64 = ((s.dn[212][0] * eq17_e887_q) + (s.v[212] * s.dn[238][0]));
        let eq17_e888_q_d_n1: f64 = ((s.dn[212][1] * eq17_e887_q) + (s.v[212] * s.dn[238][1]));
        let eq17_e888_q_d_n2: f64 = ((s.dn[212][2] * eq17_e887_q) + (s.v[212] * s.dn[238][2]));
        let eq17_e888_q_d_n3: f64 = ((s.dn[212][3] * eq17_e887_q) + (s.v[212] * s.dn[238][3]));
        let eq17_e888_q_d_n4: f64 = ((s.dn[212][4] * eq17_e887_q) + (s.v[212] * s.dn[238][4]));
        let eq17_e888_q_d_n5: f64 = ((s.dn[212][5] * eq17_e887_q) + (s.v[212] * s.dn[238][5]));
        let eq17_e888_q_d_n6: f64 = ((s.dn[212][6] * eq17_e887_q) + (s.v[212] * s.dn[238][6]));
        let eq17_e888_q_d_n7: f64 = ((s.dn[212][7] * eq17_e887_q) + (s.v[212] * s.dn[238][7]));
        let eq17_e888_q_d_n8: f64 = ((s.dn[212][8] * eq17_e887_q) + (s.v[212] * s.dn[238][8]));
        let eq17_e888_q_d_b0: f64 = ((s.db[212][0] * eq17_e887_q) + (s.v[212] * s.db[238][0]));
        let eq17_e888_q_d_b1: f64 = ((s.db[212][1] * eq17_e887_q) + (s.v[212] * s.db[238][1]));
        let eq17_e888_q_d_b2: f64 = ((s.db[212][2] * eq17_e887_q) + (s.v[212] * s.db[238][2]));
        let eq17_e888_q_d_b3: f64 = ((s.db[212][3] * eq17_e887_q) + (s.v[212] * s.db[238][3]));
        let eq17_e888_q_d_b4: f64 = ((s.db[212][4] * eq17_e887_q) + (s.v[212] * s.db[238][4]));
        let eq17_reactive_node_derivatives: [f64; 9] = [eq17_e888_q_d_n0, eq17_e888_q_d_n1, eq17_e888_q_d_n2, eq17_e888_q_d_n3, eq17_e888_q_d_n4, eq17_e888_q_d_n5, eq17_e888_q_d_n6, eq17_e888_q_d_n7, eq17_e888_q_d_n8];
        let eq17_reactive_branch_derivatives: [f64; 5] = [eq17_e888_q_d_b0, eq17_e888_q_d_b1, eq17_e888_q_d_b2, eq17_e888_q_d_b3, eq17_e888_q_d_b4];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[3]),
            nodes,
            &eq17_reactive_node_derivatives,
            branches,
            &eq17_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e891_q: f64 = s.v[239];
        let eq18_e892: f64 = (s.v[212] * s.v[239]);
        let eq18_e892_d_n0: f64 = ((s.dn[212][0] * s.v[239]) + (s.v[212] * s.dn[239][0]));
        let eq18_e892_d_n1: f64 = ((s.dn[212][1] * s.v[239]) + (s.v[212] * s.dn[239][1]));
        let eq18_e892_d_n2: f64 = ((s.dn[212][2] * s.v[239]) + (s.v[212] * s.dn[239][2]));
        let eq18_e892_d_n3: f64 = ((s.dn[212][3] * s.v[239]) + (s.v[212] * s.dn[239][3]));
        let eq18_e892_d_n4: f64 = ((s.dn[212][4] * s.v[239]) + (s.v[212] * s.dn[239][4]));
        let eq18_e892_d_n5: f64 = ((s.dn[212][5] * s.v[239]) + (s.v[212] * s.dn[239][5]));
        let eq18_e892_d_n6: f64 = ((s.dn[212][6] * s.v[239]) + (s.v[212] * s.dn[239][6]));
        let eq18_e892_d_n7: f64 = ((s.dn[212][7] * s.v[239]) + (s.v[212] * s.dn[239][7]));
        let eq18_e892_d_n8: f64 = ((s.dn[212][8] * s.v[239]) + (s.v[212] * s.dn[239][8]));
        let eq18_e892_d_b0: f64 = ((s.db[212][0] * s.v[239]) + (s.v[212] * s.db[239][0]));
        let eq18_e892_d_b1: f64 = ((s.db[212][1] * s.v[239]) + (s.v[212] * s.db[239][1]));
        let eq18_e892_d_b2: f64 = ((s.db[212][2] * s.v[239]) + (s.v[212] * s.db[239][2]));
        let eq18_e892_d_b3: f64 = ((s.db[212][3] * s.v[239]) + (s.v[212] * s.db[239][3]));
        let eq18_e892_d_b4: f64 = ((s.db[212][4] * s.v[239]) + (s.v[212] * s.db[239][4]));
        let eq18_e892_q: f64 = (s.v[212] * eq18_e891_q);
        let eq18_e892_q_d_n0: f64 = ((s.dn[212][0] * eq18_e891_q) + (s.v[212] * s.dn[239][0]));
        let eq18_e892_q_d_n1: f64 = ((s.dn[212][1] * eq18_e891_q) + (s.v[212] * s.dn[239][1]));
        let eq18_e892_q_d_n2: f64 = ((s.dn[212][2] * eq18_e891_q) + (s.v[212] * s.dn[239][2]));
        let eq18_e892_q_d_n3: f64 = ((s.dn[212][3] * eq18_e891_q) + (s.v[212] * s.dn[239][3]));
        let eq18_e892_q_d_n4: f64 = ((s.dn[212][4] * eq18_e891_q) + (s.v[212] * s.dn[239][4]));
        let eq18_e892_q_d_n5: f64 = ((s.dn[212][5] * eq18_e891_q) + (s.v[212] * s.dn[239][5]));
        let eq18_e892_q_d_n6: f64 = ((s.dn[212][6] * eq18_e891_q) + (s.v[212] * s.dn[239][6]));
        let eq18_e892_q_d_n7: f64 = ((s.dn[212][7] * eq18_e891_q) + (s.v[212] * s.dn[239][7]));
        let eq18_e892_q_d_n8: f64 = ((s.dn[212][8] * eq18_e891_q) + (s.v[212] * s.dn[239][8]));
        let eq18_e892_q_d_b0: f64 = ((s.db[212][0] * eq18_e891_q) + (s.v[212] * s.db[239][0]));
        let eq18_e892_q_d_b1: f64 = ((s.db[212][1] * eq18_e891_q) + (s.v[212] * s.db[239][1]));
        let eq18_e892_q_d_b2: f64 = ((s.db[212][2] * eq18_e891_q) + (s.v[212] * s.db[239][2]));
        let eq18_e892_q_d_b3: f64 = ((s.db[212][3] * eq18_e891_q) + (s.v[212] * s.db[239][3]));
        let eq18_e892_q_d_b4: f64 = ((s.db[212][4] * eq18_e891_q) + (s.v[212] * s.db[239][4]));
        let eq18_reactive_node_derivatives: [f64; 9] = [eq18_e892_q_d_n0, eq18_e892_q_d_n1, eq18_e892_q_d_n2, eq18_e892_q_d_n3, eq18_e892_q_d_n4, eq18_e892_q_d_n5, eq18_e892_q_d_n6, eq18_e892_q_d_n7, eq18_e892_q_d_n8];
        let eq18_reactive_branch_derivatives: [f64; 5] = [eq18_e892_q_d_b0, eq18_e892_q_d_b1, eq18_e892_q_d_b2, eq18_e892_q_d_b3, eq18_e892_q_d_b4];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[3]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq41_e1106, eq41_e1106_d_n0, eq41_e1106_d_n1, eq41_e1106_d_n2, eq41_e1106_d_n3, eq41_e1106_d_n4, eq41_e1106_d_n5, eq41_e1106_d_n6, eq41_e1106_d_n7, eq41_e1106_d_n8, eq41_e1106_d_b0, eq41_e1106_d_b1, eq41_e1106_d_b2, eq41_e1106_d_b3, eq41_e1106_d_b4, eq41_e1106_q, eq41_e1106_q_d_n0, eq41_e1106_q_d_n1, eq41_e1106_q_d_n2, eq41_e1106_q_d_n3, eq41_e1106_q_d_n4, eq41_e1106_q_d_n5, eq41_e1106_q_d_n6, eq41_e1106_q_d_n7, eq41_e1106_q_d_n8, eq41_e1106_q_d_b0, eq41_e1106_q_d_b1, eq41_e1106_q_d_b2, eq41_e1106_q_d_b3, eq41_e1106_q_d_b4,) = {
    if s.b[671] {
        let eq41_e1103: f64 = ((nv4 - 0.0) * s.v[270]);
        let eq41_e1103_d_n0: f64 = ((nv4 - 0.0) * s.dn[270][0]);
        let eq41_e1103_d_n1: f64 = ((nv4 - 0.0) * s.dn[270][1]);
        let eq41_e1103_d_n2: f64 = ((nv4 - 0.0) * s.dn[270][2]);
        let eq41_e1103_d_n3: f64 = ((nv4 - 0.0) * s.dn[270][3]);
        let eq41_e1103_d_n4: f64 = (s.v[270] + ((nv4 - 0.0) * s.dn[270][4]));
        let eq41_e1103_d_n5: f64 = ((nv4 - 0.0) * s.dn[270][5]);
        let eq41_e1103_d_n6: f64 = ((nv4 - 0.0) * s.dn[270][6]);
        let eq41_e1103_d_n7: f64 = ((nv4 - 0.0) * s.dn[270][7]);
        let eq41_e1103_d_n8: f64 = ((nv4 - 0.0) * s.dn[270][8]);
        let eq41_e1103_d_b0: f64 = ((nv4 - 0.0) * s.db[270][0]);
        let eq41_e1103_d_b1: f64 = ((nv4 - 0.0) * s.db[270][1]);
        let eq41_e1103_d_b2: f64 = ((nv4 - 0.0) * s.db[270][2]);
        let eq41_e1103_d_b3: f64 = ((nv4 - 0.0) * s.db[270][3]);
        let eq41_e1103_d_b4: f64 = ((nv4 - 0.0) * s.db[270][4]);
        let eq41_e1104_q: f64 = eq41_e1103;
        (eq41_e1103, eq41_e1103_d_n0, eq41_e1103_d_n1, eq41_e1103_d_n2, eq41_e1103_d_n3, eq41_e1103_d_n4, eq41_e1103_d_n5, eq41_e1103_d_n6, eq41_e1103_d_n7, eq41_e1103_d_n8, eq41_e1103_d_b0, eq41_e1103_d_b1, eq41_e1103_d_b2, eq41_e1103_d_b3, eq41_e1103_d_b4, eq41_e1104_q, eq41_e1103_d_n0, eq41_e1103_d_n1, eq41_e1103_d_n2, eq41_e1103_d_n3, eq41_e1103_d_n4, eq41_e1103_d_n5, eq41_e1103_d_n6, eq41_e1103_d_n7, eq41_e1103_d_n8, eq41_e1103_d_b0, eq41_e1103_d_b1, eq41_e1103_d_b2, eq41_e1103_d_b3, eq41_e1103_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_reactive_node_derivatives: [f64; 9] = [eq41_e1106_q_d_n0, eq41_e1106_q_d_n1, eq41_e1106_q_d_n2, eq41_e1106_q_d_n3, eq41_e1106_q_d_n4, eq41_e1106_q_d_n5, eq41_e1106_q_d_n6, eq41_e1106_q_d_n7, eq41_e1106_q_d_n8];
        let eq41_reactive_branch_derivatives: [f64; 5] = [eq41_e1106_q_d_b0, eq41_e1106_q_d_b1, eq41_e1106_q_d_b2, eq41_e1106_q_d_b3, eq41_e1106_q_d_b4];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
    }
}

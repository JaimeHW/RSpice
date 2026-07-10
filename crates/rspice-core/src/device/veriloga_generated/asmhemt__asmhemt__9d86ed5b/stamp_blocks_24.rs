#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_78(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);let nv2 = ctx.node_voltage(nodes[2]);let nv4 = ctx.node_voltage(nodes[4]);let eq217_e2707: f64 = (p.p4 * p.p5);let eq217_e2709: f64 = (eq217_e2707 * p.p220);let eq217_e2711: f64 = (eq217_e2709 * (nv1 - nv2));let eq217_e2712_q: f64 = eq217_e2711;let eq217_e2713: f64 = (p.p7 * eq217_e2711);let eq217_e2713_d_n1: f64 = (p.p7 * eq217_e2709);let eq217_e2713_d_n2: f64 = (p.p7 * (-eq217_e2709));let eq217_e2713_q: f64 = (p.p7 * eq217_e2712_q);
        stamper.stamp_current_reactive_node2_local(
            Some(1),
            Some(2),
            1,
            multiplicity * (eq217_e2713_d_n1),
            2,
            multiplicity * (eq217_e2713_d_n2),
        );let eq218_e2716_q: f64 = s.v[196];let eq218_e2717: f64 = (p.p7 * s.v[196]);let eq218_e2717_q: f64 = (p.p7 * eq218_e2716_q);
        stamper.stamp_current_reactive_dense_local(
            Some(9),
            Some(2),
            &s.dn[196],
            &s.db[196],
            (multiplicity) * (p.p7),
        );let eq219_e2720_q: f64 = s.v[197];let eq219_e2721: f64 = (p.p7 * s.v[197]);let eq219_e2721_q: f64 = (p.p7 * eq219_e2720_q);
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(0),
            &s.dn[197],
            &s.db[197],
            (multiplicity) * (p.p7),
        );let eq220_e2724_q: f64 = s.v[194];let eq220_e2725: f64 = (p.p7 * s.v[194]);let eq220_e2725_q: f64 = (p.p7 * eq220_e2724_q);
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(0),
            &s.dn[194],
            &s.db[194],
            (multiplicity) * (p.p7),
        );
        let (eq223_e2771, eq223_e2771_d_n4, eq223_e2771_q,) = {
    if s.b[610] {
        let eq223_e2768: f64 = ((nv4 - 0.0) * p.p33);let eq223_e2769_q: f64 = eq223_e2768;
        (eq223_e2768, p.p33, eq223_e2769_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(4),
            None,
            4,
            multiplicity * (eq223_e2771_d_n4),
        );
    }
}

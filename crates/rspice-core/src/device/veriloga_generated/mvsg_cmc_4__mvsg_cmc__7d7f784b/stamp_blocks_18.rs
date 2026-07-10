#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);let nv7 = ctx.node_voltage(nodes[7]);let nv16 = ctx.node_voltage(nodes[16]);let nv17 = ctx.node_voltage(nodes[17]);let nv20 = ctx.node_voltage(nodes[20]);let nv21 = ctx.node_voltage(nodes[21]);
        let (eq8_e421, eq8_e421_d_n20, eq8_e421_d_n21, eq8_e421_q,) = {
    if (l.f1db2 != 0.0) {
        let eq8_e418: f64 = (p.p330 * (nv21 - nv20));let eq8_e419_q: f64 = eq8_e418;
        (eq8_e418, (-p.p330), p.p330, eq8_e419_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node2_local(
            Some(21),
            Some(20),
            20,
            multiplicity * (eq8_e421_d_n20),
            21,
            multiplicity * (eq8_e421_d_n21),
        );
        let (eq9_e428, eq9_e428_d_n20, eq9_e428_q,) = {
    if (l.f1db2 != 0.0) {
        let eq9_e425: f64 = (p.p332 * (nv20 - 0.0));let eq9_e426_q: f64 = eq9_e425;
        (eq9_e425, p.p332, eq9_e426_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(20),
            None,
            20,
            multiplicity * (eq9_e428_d_n20),
        );
        let (eq17_e564, eq17_e564_d_n4, eq17_e564_d_n23, eq17_e564_q, eq17_e564_q_d_n4,) = {
    if ((l.f1db2 == 0.0) && (l.f1dc8 != 0.0)) {
        let eq17_e543_q: f64 = l.f234a;let eq17_e544: f64 = (p.p341 * l.f234a);let eq17_e544_d_n23: f64 = (p.p341 * l.f234b);let eq17_e544_q: f64 = (p.p341 * eq17_e543_q);let eq17_e549: f64 = (l.f22ef - l.f22f4);let eq17_e550: f64 = (p.p342 * eq17_e549);let eq17_e550_d_n4: f64 = (p.p342 * l.f22f0);let eq17_e551: f64 = (1.0 + eq17_e550);let eq17_e555: f64 = (l.f22ef - l.f22f4);let eq17_e556: f64 = (p.p344 * eq17_e555);let eq17_e556_d_n4: f64 = (p.p344 * l.f22f0);let eq17_e559: f64 = (l.f22ef - l.f22f4);let eq17_e560: f64 = (eq17_e556 * eq17_e559);let eq17_e560_d_n4: f64 = ((eq17_e556_d_n4 * eq17_e559) + (eq17_e556 * l.f22f0));let eq17_e561: f64 = (eq17_e551 + eq17_e560);let eq17_e561_d_n4: f64 = (eq17_e550_d_n4 + eq17_e560_d_n4);let eq17_e562: f64 = (eq17_e544 * eq17_e561);let eq17_e562_d_n4: f64 = (eq17_e544 * eq17_e561_d_n4);let eq17_e562_d_n23: f64 = (eq17_e544_d_n23 * eq17_e561);let eq17_e562_q: f64 = (eq17_e544_q * eq17_e561);let eq17_e562_q_d_n4: f64 = (eq17_e544_q * eq17_e561_d_n4);
        (eq17_e562, eq17_e562_d_n4, eq17_e562_d_n23, eq17_e562_q, eq17_e562_q_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node2_local(
            Some(23),
            None,
            4,
            multiplicity * (eq17_e564_q_d_n4),
            23,
            multiplicity * (eq17_e564_d_n23),
        );
        let (eq22_e682, eq22_e682_d_n4, eq22_e682_d_n26, eq22_e682_q, eq22_e682_q_d_n4,) = {
    if ((l.f1db2 == 0.0) && (l.f1dc8 != 0.0)) {
        let eq22_e661_q: f64 = l.f237a;let eq22_e662: f64 = (p.p341 * l.f237a);let eq22_e662_d_n26: f64 = (p.p341 * l.f237b);let eq22_e662_q: f64 = (p.p341 * eq22_e661_q);let eq22_e667: f64 = (l.f22ef - l.f22f4);let eq22_e668: f64 = (p.p343 * eq22_e667);let eq22_e668_d_n4: f64 = (p.p343 * l.f22f0);let eq22_e669: f64 = (1.0 + eq22_e668);let eq22_e673: f64 = (l.f22ef - l.f22f4);let eq22_e674: f64 = (p.p345 * eq22_e673);let eq22_e674_d_n4: f64 = (p.p345 * l.f22f0);let eq22_e677: f64 = (l.f22ef - l.f22f4);let eq22_e678: f64 = (eq22_e674 * eq22_e677);let eq22_e678_d_n4: f64 = ((eq22_e674_d_n4 * eq22_e677) + (eq22_e674 * l.f22f0));let eq22_e679: f64 = (eq22_e669 + eq22_e678);let eq22_e679_d_n4: f64 = (eq22_e668_d_n4 + eq22_e678_d_n4);let eq22_e680: f64 = (eq22_e662 * eq22_e679);let eq22_e680_d_n4: f64 = (eq22_e662 * eq22_e679_d_n4);let eq22_e680_d_n26: f64 = (eq22_e662_d_n26 * eq22_e679);let eq22_e680_q: f64 = (eq22_e662_q * eq22_e679);let eq22_e680_q_d_n4: f64 = (eq22_e662_q * eq22_e679_d_n4);
        (eq22_e680, eq22_e680_d_n4, eq22_e680_d_n26, eq22_e680_q, eq22_e680_q_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node2_local(
            Some(26),
            None,
            4,
            multiplicity * (eq22_e682_q_d_n4),
            26,
            multiplicity * (eq22_e682_d_n26),
        );
        let (eq33_e769, eq33_e769_d_n2, eq33_e769_d_n4, eq33_e769_d_n7, eq33_e769_d_n16, eq33_e769_d_n17, eq33_e769_q,) = {
    if (l.f206a != 0.0) {
        let eq33_e762_q: f64 = l.f223f;let eq33_e765: f64 = (p.p355 * (nv7 - nv16));let eq33_e766_q: f64 = eq33_e765;let eq33_e767: f64 = (l.f223f + eq33_e765);let eq33_e767_d_n7: f64 = (l.f2244 + p.p355);let eq33_e767_d_n16: f64 = (l.f2240 + (-p.p355));let eq33_e767_q: f64 = (eq33_e762_q + eq33_e766_q);
        (eq33_e767, l.f2242, l.f2243, eq33_e767_d_n7, eq33_e767_d_n16, l.f2241, eq33_e767_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq33_e769_d_n2, 0.0, eq33_e769_d_n4, 0.0, 0.0, eq33_e769_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq33_e769_d_n16, eq33_e769_d_n17, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq33_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(16),
            &eq33_reactive_node_derivatives,
            &eq33_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq34_e779, eq34_e779_d_n2, eq34_e779_d_n4, eq34_e779_d_n7, eq34_e779_d_n16, eq34_e779_d_n17, eq34_e779_q,) = {
    if (l.f206a != 0.0) {
        let eq34_e772_q: f64 = l.f21fd;let eq34_e775: f64 = (p.p355 * (nv7 - nv17));let eq34_e776_q: f64 = eq34_e775;let eq34_e777: f64 = (l.f21fd + eq34_e775);let eq34_e777_d_n7: f64 = (l.f2202 + p.p355);let eq34_e777_d_n17: f64 = (l.f21ff + (-p.p355));let eq34_e777_q: f64 = (eq34_e772_q + eq34_e776_q);
        (eq34_e777, l.f2200, l.f2201, eq34_e777_d_n7, l.f21fe, eq34_e777_d_n17, eq34_e777_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq34_e779_d_n2, 0.0, eq34_e779_d_n4, 0.0, 0.0, eq34_e779_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq34_e779_d_n16, eq34_e779_d_n17, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq34_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(17),
            &eq34_reactive_node_derivatives,
            &eq34_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq35_e789, eq35_e789_d_n2, eq35_e789_d_n3, eq35_e789_d_n4, eq35_e789_d_n7, eq35_e789_d_n16, eq35_e789_d_n17, eq35_e789_q,) = {
    if (l.f206a != 0.0) {
        let eq35_e782_q: f64 = l.f21b6;let eq35_e785: f64 = (p.p355 * (nv2 - nv16));let eq35_e786_q: f64 = eq35_e785;let eq35_e787: f64 = (l.f21b6 + eq35_e785);let eq35_e787_d_n2: f64 = (l.f21b9 + p.p355);let eq35_e787_d_n16: f64 = (l.f21b7 + (-p.p355));let eq35_e787_q: f64 = (eq35_e782_q + eq35_e786_q);
        (eq35_e787, eq35_e787_d_n2, l.f21ba, l.f21bb, l.f21bc, eq35_e787_d_n16, l.f21b8, eq35_e787_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq35_e789_d_n2, eq35_e789_d_n3, eq35_e789_d_n4, 0.0, 0.0, eq35_e789_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq35_e789_d_n16, eq35_e789_d_n17, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq35_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(16),
            &eq35_reactive_node_derivatives,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);let nv3 = ctx.node_voltage(nodes[3]);let nv7 = ctx.node_voltage(nodes[7]);let nv9 = ctx.node_voltage(nodes[9]);let nv15 = ctx.node_voltage(nodes[15]);let nv16 = ctx.node_voltage(nodes[16]);let nv17 = ctx.node_voltage(nodes[17]);
        let (eq37_e803, eq37_e803_d_n2, eq37_e803_d_n3, eq37_e803_d_n4, eq37_e803_d_n7, eq37_e803_d_n9, eq37_e803_d_n16, eq37_e803_d_n17, eq37_e803_q,) = {
    if (l.f206a != 0.0) {
        let eq37_e796_q: f64 = l.f22bc;let eq37_e799: f64 = (p.p355 * (nv7 - nv9));let eq37_e800_q: f64 = eq37_e799;let eq37_e801: f64 = (l.f22bc + eq37_e799);let eq37_e801_d_n7: f64 = (l.f22c2 + p.p355);let eq37_e801_q: f64 = (eq37_e796_q + eq37_e800_q);
        (eq37_e801, l.f22bf, l.f22c0, l.f22c1, eq37_e801_d_n7, (-p.p355), l.f22bd, l.f22be, eq37_e801_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq37_e803_d_n2, eq37_e803_d_n3, eq37_e803_d_n4, 0.0, 0.0, eq37_e803_d_n7, 0.0, eq37_e803_d_n9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq37_e803_d_n16, eq37_e803_d_n17, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq37_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(9),
            &eq37_reactive_node_derivatives,
            &eq37_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq38_e814, eq38_e814_d_n2, eq38_e814_d_n4, eq38_e814_d_n7, eq38_e814_d_n16, eq38_e814_d_n17, eq38_e814_q,) = {
    if (l.f206a == 0.0) {
        let eq38_e807_q: f64 = l.f223f;let eq38_e810: f64 = (p.p355 * (nv2 - nv16));let eq38_e811_q: f64 = eq38_e810;let eq38_e812: f64 = (l.f223f + eq38_e810);let eq38_e812_d_n2: f64 = (l.f2242 + p.p355);let eq38_e812_d_n16: f64 = (l.f2240 + (-p.p355));let eq38_e812_q: f64 = (eq38_e807_q + eq38_e811_q);
        (eq38_e812, eq38_e812_d_n2, l.f2243, l.f2244, eq38_e812_d_n16, l.f2241, eq38_e812_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq38_e814_d_n2, 0.0, eq38_e814_d_n4, 0.0, 0.0, eq38_e814_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq38_e814_d_n16, eq38_e814_d_n17, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq38_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(16),
            &eq38_reactive_node_derivatives,
            &eq38_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq39_e825, eq39_e825_d_n2, eq39_e825_d_n4, eq39_e825_d_n7, eq39_e825_d_n16, eq39_e825_d_n17, eq39_e825_q,) = {
    if (l.f206a == 0.0) {
        let eq39_e818_q: f64 = l.f21fd;let eq39_e821: f64 = (p.p355 * (nv2 - nv17));let eq39_e822_q: f64 = eq39_e821;let eq39_e823: f64 = (l.f21fd + eq39_e821);let eq39_e823_d_n2: f64 = (l.f2200 + p.p355);let eq39_e823_d_n17: f64 = (l.f21ff + (-p.p355));let eq39_e823_q: f64 = (eq39_e818_q + eq39_e822_q);
        (eq39_e823, eq39_e823_d_n2, l.f2201, l.f2202, l.f21fe, eq39_e823_d_n17, eq39_e823_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq39_e825_d_n2, 0.0, eq39_e825_d_n4, 0.0, 0.0, eq39_e825_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq39_e825_d_n16, eq39_e825_d_n17, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq39_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(17),
            &eq39_reactive_node_derivatives,
            &eq39_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq40_e836, eq40_e836_d_n2, eq40_e836_d_n3, eq40_e836_d_n4, eq40_e836_d_n7, eq40_e836_d_n16, eq40_e836_d_n17, eq40_e836_q,) = {
    if (l.f206a == 0.0) {
        let eq40_e829_q: f64 = l.f21b6;let eq40_e832: f64 = (p.p355 * (nv7 - nv16));let eq40_e833_q: f64 = eq40_e832;let eq40_e834: f64 = (l.f21b6 + eq40_e832);let eq40_e834_d_n7: f64 = (l.f21bc + p.p355);let eq40_e834_d_n16: f64 = (l.f21b7 + (-p.p355));let eq40_e834_q: f64 = (eq40_e829_q + eq40_e833_q);
        (eq40_e834, l.f21b9, l.f21ba, l.f21bb, eq40_e834_d_n7, eq40_e834_d_n16, l.f21b8, eq40_e834_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq40_e836_d_n2, eq40_e836_d_n3, eq40_e836_d_n4, 0.0, 0.0, eq40_e836_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq40_e836_d_n16, eq40_e836_d_n17, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq40_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(16),
            &eq40_reactive_node_derivatives,
            &eq40_reactive_branch_derivatives,
            multiplicity,
        );let eq43_e848_q: f64 = l.f2176;let eq43_e851: f64 = (p.p355 * (nv3 - nv16));let eq43_e852_q: f64 = eq43_e851;let eq43_e853: f64 = (l.f2176 + eq43_e851);let eq43_e853_d_n3: f64 = (l.f217a + p.p355);let eq43_e853_d_n16: f64 = (l.f2177 + (-p.p355));let eq43_e853_q: f64 = (eq43_e848_q + eq43_e852_q);let eq43_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, l.f2179, eq43_e853_d_n3, l.f217b, 0.0, 0.0, l.f217c, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq43_e853_d_n16, l.f2178, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq43_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(3),
            Some(16),
            &eq43_reactive_node_derivatives,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq46_e876, eq46_e876_d_n2, eq46_e876_d_n4, eq46_e876_d_n7, eq46_e876_d_n15, eq46_e876_d_n16, eq46_e876_q,) = {
    if (l.f20b4 != 0.0) {
        let eq46_e869_q: f64 = l.f2238;let eq46_e872: f64 = (p.p355 * (nv7 - nv15));let eq46_e873_q: f64 = eq46_e872;let eq46_e874: f64 = (l.f2238 + eq46_e872);let eq46_e874_d_n7: f64 = (l.f223d + p.p355);let eq46_e874_d_n15: f64 = (l.f2239 + (-p.p355));let eq46_e874_q: f64 = (eq46_e869_q + eq46_e873_q);
        (eq46_e874, l.f223b, l.f223c, eq46_e874_d_n7, eq46_e874_d_n15, l.f223a, eq46_e874_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq46_e876_d_n2, 0.0, eq46_e876_d_n4, 0.0, 0.0, eq46_e876_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq46_e876_d_n15, eq46_e876_d_n16, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq46_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(15),
            &eq46_reactive_node_derivatives,
            &eq46_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq47_e886, eq47_e886_d_n2, eq47_e886_d_n4, eq47_e886_d_n7, eq47_e886_d_n15, eq47_e886_d_n16, eq47_e886_q,) = {
    if (l.f20b4 != 0.0) {
        let eq47_e879_q: f64 = l.f21f6;let eq47_e882: f64 = (p.p355 * (nv7 - nv16));let eq47_e883_q: f64 = eq47_e882;let eq47_e884: f64 = (l.f21f6 + eq47_e882);let eq47_e884_d_n7: f64 = (l.f21fb + p.p355);let eq47_e884_d_n16: f64 = (l.f21f8 + (-p.p355));let eq47_e884_q: f64 = (eq47_e879_q + eq47_e883_q);
        (eq47_e884, l.f21f9, l.f21fa, eq47_e884_d_n7, l.f21f7, eq47_e884_d_n16, eq47_e884_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq47_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq47_e886_d_n2, 0.0, eq47_e886_d_n4, 0.0, 0.0, eq47_e886_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq47_e886_d_n15, eq47_e886_d_n16, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq47_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(16),
            &eq47_reactive_node_derivatives,
            &eq47_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);let nv3 = ctx.node_voltage(nodes[3]);let nv7 = ctx.node_voltage(nodes[7]);let nv9 = ctx.node_voltage(nodes[9]);let nv14 = ctx.node_voltage(nodes[14]);let nv15 = ctx.node_voltage(nodes[15]);let nv16 = ctx.node_voltage(nodes[16]);
        let (eq48_e896, eq48_e896_d_n2, eq48_e896_d_n3, eq48_e896_d_n4, eq48_e896_d_n7, eq48_e896_d_n15, eq48_e896_d_n16, eq48_e896_q,) = {
    if (l.f20b4 != 0.0) {
        let eq48_e889_q: f64 = l.f21ae;let eq48_e892: f64 = (p.p355 * (nv2 - nv15));let eq48_e893_q: f64 = eq48_e892;let eq48_e894: f64 = (l.f21ae + eq48_e892);let eq48_e894_d_n2: f64 = (l.f21b1 + p.p355);let eq48_e894_d_n15: f64 = (l.f21af + (-p.p355));let eq48_e894_q: f64 = (eq48_e889_q + eq48_e893_q);
        (eq48_e894, eq48_e894_d_n2, l.f21b2, l.f21b3, l.f21b4, eq48_e894_d_n15, l.f21b0, eq48_e894_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq48_e896_d_n2, eq48_e896_d_n3, eq48_e896_d_n4, 0.0, 0.0, eq48_e896_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq48_e896_d_n15, eq48_e896_d_n16, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq48_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(15),
            &eq48_reactive_node_derivatives,
            &eq48_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq50_e910, eq50_e910_d_n2, eq50_e910_d_n3, eq50_e910_d_n4, eq50_e910_d_n7, eq50_e910_d_n9, eq50_e910_d_n15, eq50_e910_d_n16, eq50_e910_q,) = {
    if (l.f20b4 != 0.0) {
        let eq50_e903_q: f64 = l.f22b4;let eq50_e906: f64 = (p.p355 * (nv7 - nv9));let eq50_e907_q: f64 = eq50_e906;let eq50_e908: f64 = (l.f22b4 + eq50_e906);let eq50_e908_d_n7: f64 = (l.f22ba + p.p355);let eq50_e908_q: f64 = (eq50_e903_q + eq50_e907_q);
        (eq50_e908, l.f22b7, l.f22b8, l.f22b9, eq50_e908_d_n7, (-p.p355), l.f22b5, l.f22b6, eq50_e908_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq50_e910_d_n2, eq50_e910_d_n3, eq50_e910_d_n4, 0.0, 0.0, eq50_e910_d_n7, 0.0, eq50_e910_d_n9, 0.0, 0.0, 0.0, 0.0, 0.0, eq50_e910_d_n15, eq50_e910_d_n16, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq50_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(9),
            &eq50_reactive_node_derivatives,
            &eq50_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq51_e921, eq51_e921_d_n2, eq51_e921_d_n4, eq51_e921_d_n7, eq51_e921_d_n15, eq51_e921_d_n16, eq51_e921_q,) = {
    if (l.f20b4 == 0.0) {
        let eq51_e914_q: f64 = l.f2238;let eq51_e917: f64 = (p.p355 * (nv2 - nv15));let eq51_e918_q: f64 = eq51_e917;let eq51_e919: f64 = (l.f2238 + eq51_e917);let eq51_e919_d_n2: f64 = (l.f223b + p.p355);let eq51_e919_d_n15: f64 = (l.f2239 + (-p.p355));let eq51_e919_q: f64 = (eq51_e914_q + eq51_e918_q);
        (eq51_e919, eq51_e919_d_n2, l.f223c, l.f223d, eq51_e919_d_n15, l.f223a, eq51_e919_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq51_e921_d_n2, 0.0, eq51_e921_d_n4, 0.0, 0.0, eq51_e921_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq51_e921_d_n15, eq51_e921_d_n16, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq51_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(15),
            &eq51_reactive_node_derivatives,
            &eq51_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq52_e932, eq52_e932_d_n2, eq52_e932_d_n4, eq52_e932_d_n7, eq52_e932_d_n15, eq52_e932_d_n16, eq52_e932_q,) = {
    if (l.f20b4 == 0.0) {
        let eq52_e925_q: f64 = l.f21f6;let eq52_e928: f64 = (p.p355 * (nv2 - nv16));let eq52_e929_q: f64 = eq52_e928;let eq52_e930: f64 = (l.f21f6 + eq52_e928);let eq52_e930_d_n2: f64 = (l.f21f9 + p.p355);let eq52_e930_d_n16: f64 = (l.f21f8 + (-p.p355));let eq52_e930_q: f64 = (eq52_e925_q + eq52_e929_q);
        (eq52_e930, eq52_e930_d_n2, l.f21fa, l.f21fb, l.f21f7, eq52_e930_d_n16, eq52_e930_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq52_e932_d_n2, 0.0, eq52_e932_d_n4, 0.0, 0.0, eq52_e932_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq52_e932_d_n15, eq52_e932_d_n16, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq52_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(16),
            &eq52_reactive_node_derivatives,
            &eq52_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq53_e943, eq53_e943_d_n2, eq53_e943_d_n3, eq53_e943_d_n4, eq53_e943_d_n7, eq53_e943_d_n15, eq53_e943_d_n16, eq53_e943_q,) = {
    if (l.f20b4 == 0.0) {
        let eq53_e936_q: f64 = l.f21ae;let eq53_e939: f64 = (p.p355 * (nv7 - nv15));let eq53_e940_q: f64 = eq53_e939;let eq53_e941: f64 = (l.f21ae + eq53_e939);let eq53_e941_d_n7: f64 = (l.f21b4 + p.p355);let eq53_e941_d_n15: f64 = (l.f21af + (-p.p355));let eq53_e941_q: f64 = (eq53_e936_q + eq53_e940_q);
        (eq53_e941, l.f21b1, l.f21b2, l.f21b3, eq53_e941_d_n7, eq53_e941_d_n15, l.f21b0, eq53_e941_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq53_e943_d_n2, eq53_e943_d_n3, eq53_e943_d_n4, 0.0, 0.0, eq53_e943_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq53_e943_d_n15, eq53_e943_d_n16, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq53_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(15),
            &eq53_reactive_node_derivatives,
            &eq53_reactive_branch_derivatives,
            multiplicity,
        );let eq56_e955_q: f64 = l.f216e;let eq56_e958: f64 = (p.p355 * (nv3 - nv15));let eq56_e959_q: f64 = eq56_e958;let eq56_e960: f64 = (l.f216e + eq56_e958);let eq56_e960_d_n3: f64 = (l.f2172 + p.p355);let eq56_e960_d_n15: f64 = (l.f216f + (-p.p355));let eq56_e960_q: f64 = (eq56_e955_q + eq56_e959_q);let eq56_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, l.f2171, eq56_e960_d_n3, l.f2173, 0.0, 0.0, l.f2174, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq56_e960_d_n15, l.f2170, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq56_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(3),
            Some(15),
            &eq56_reactive_node_derivatives,
            &eq56_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq59_e983, eq59_e983_d_n2, eq59_e983_d_n4, eq59_e983_d_n7, eq59_e983_d_n14, eq59_e983_d_n15, eq59_e983_q,) = {
    if (l.f1dcb != 0.0) {
        let eq59_e976_q: f64 = l.f2231;let eq59_e979: f64 = (p.p355 * (nv7 - nv14));let eq59_e980_q: f64 = eq59_e979;let eq59_e981: f64 = (l.f2231 + eq59_e979);let eq59_e981_d_n7: f64 = (l.f2236 + p.p355);let eq59_e981_d_n14: f64 = (l.f2232 + (-p.p355));let eq59_e981_q: f64 = (eq59_e976_q + eq59_e980_q);
        (eq59_e981, l.f2234, l.f2235, eq59_e981_d_n7, eq59_e981_d_n14, l.f2233, eq59_e981_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq59_e983_d_n2, 0.0, eq59_e983_d_n4, 0.0, 0.0, eq59_e983_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq59_e983_d_n14, eq59_e983_d_n15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq59_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(14),
            &eq59_reactive_node_derivatives,
            &eq59_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);let nv3 = ctx.node_voltage(nodes[3]);let nv7 = ctx.node_voltage(nodes[7]);let nv9 = ctx.node_voltage(nodes[9]);let nv14 = ctx.node_voltage(nodes[14]);let nv15 = ctx.node_voltage(nodes[15]);
        let (eq60_e993, eq60_e993_d_n2, eq60_e993_d_n4, eq60_e993_d_n7, eq60_e993_d_n14, eq60_e993_d_n15, eq60_e993_q,) = {
    if (l.f1dcb != 0.0) {
        let eq60_e986_q: f64 = l.f21ef;let eq60_e989: f64 = (p.p355 * (nv7 - nv15));let eq60_e990_q: f64 = eq60_e989;let eq60_e991: f64 = (l.f21ef + eq60_e989);let eq60_e991_d_n7: f64 = (l.f21f4 + p.p355);let eq60_e991_d_n15: f64 = (l.f21f1 + (-p.p355));let eq60_e991_q: f64 = (eq60_e986_q + eq60_e990_q);
        (eq60_e991, l.f21f2, l.f21f3, eq60_e991_d_n7, l.f21f0, eq60_e991_d_n15, eq60_e991_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq60_e993_d_n2, 0.0, eq60_e993_d_n4, 0.0, 0.0, eq60_e993_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq60_e993_d_n14, eq60_e993_d_n15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq60_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(15),
            &eq60_reactive_node_derivatives,
            &eq60_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq61_e1003, eq61_e1003_d_n2, eq61_e1003_d_n3, eq61_e1003_d_n4, eq61_e1003_d_n7, eq61_e1003_d_n14, eq61_e1003_d_n15, eq61_e1003_q,) = {
    if (l.f1dcb != 0.0) {
        let eq61_e996_q: f64 = l.f21a6;let eq61_e999: f64 = (p.p355 * (nv2 - nv14));let eq61_e1000_q: f64 = eq61_e999;let eq61_e1001: f64 = (l.f21a6 + eq61_e999);let eq61_e1001_d_n2: f64 = (l.f21a9 + p.p355);let eq61_e1001_d_n14: f64 = (l.f21a7 + (-p.p355));let eq61_e1001_q: f64 = (eq61_e996_q + eq61_e1000_q);
        (eq61_e1001, eq61_e1001_d_n2, l.f21aa, l.f21ab, l.f21ac, eq61_e1001_d_n14, l.f21a8, eq61_e1001_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq61_e1003_d_n2, eq61_e1003_d_n3, eq61_e1003_d_n4, 0.0, 0.0, eq61_e1003_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq61_e1003_d_n14, eq61_e1003_d_n15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq61_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(14),
            &eq61_reactive_node_derivatives,
            &eq61_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq63_e1017, eq63_e1017_d_n2, eq63_e1017_d_n3, eq63_e1017_d_n4, eq63_e1017_d_n7, eq63_e1017_d_n9, eq63_e1017_d_n14, eq63_e1017_d_n15, eq63_e1017_q,) = {
    if (l.f1dcb != 0.0) {
        let eq63_e1010_q: f64 = l.f22ac;let eq63_e1013: f64 = (p.p355 * (nv7 - nv9));let eq63_e1014_q: f64 = eq63_e1013;let eq63_e1015: f64 = (l.f22ac + eq63_e1013);let eq63_e1015_d_n7: f64 = (l.f22b2 + p.p355);let eq63_e1015_q: f64 = (eq63_e1010_q + eq63_e1014_q);
        (eq63_e1015, l.f22af, l.f22b0, l.f22b1, eq63_e1015_d_n7, (-p.p355), l.f22ad, l.f22ae, eq63_e1015_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq63_e1017_d_n2, eq63_e1017_d_n3, eq63_e1017_d_n4, 0.0, 0.0, eq63_e1017_d_n7, 0.0, eq63_e1017_d_n9, 0.0, 0.0, 0.0, 0.0, eq63_e1017_d_n14, eq63_e1017_d_n15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq63_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(9),
            &eq63_reactive_node_derivatives,
            &eq63_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq64_e1028, eq64_e1028_d_n2, eq64_e1028_d_n4, eq64_e1028_d_n7, eq64_e1028_d_n14, eq64_e1028_d_n15, eq64_e1028_q,) = {
    if (l.f1dcb == 0.0) {
        let eq64_e1021_q: f64 = l.f2231;let eq64_e1024: f64 = (p.p355 * (nv2 - nv14));let eq64_e1025_q: f64 = eq64_e1024;let eq64_e1026: f64 = (l.f2231 + eq64_e1024);let eq64_e1026_d_n2: f64 = (l.f2234 + p.p355);let eq64_e1026_d_n14: f64 = (l.f2232 + (-p.p355));let eq64_e1026_q: f64 = (eq64_e1021_q + eq64_e1025_q);
        (eq64_e1026, eq64_e1026_d_n2, l.f2235, l.f2236, eq64_e1026_d_n14, l.f2233, eq64_e1026_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq64_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq64_e1028_d_n2, 0.0, eq64_e1028_d_n4, 0.0, 0.0, eq64_e1028_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq64_e1028_d_n14, eq64_e1028_d_n15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq64_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(14),
            &eq64_reactive_node_derivatives,
            &eq64_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq65_e1039, eq65_e1039_d_n2, eq65_e1039_d_n4, eq65_e1039_d_n7, eq65_e1039_d_n14, eq65_e1039_d_n15, eq65_e1039_q,) = {
    if (l.f1dcb == 0.0) {
        let eq65_e1032_q: f64 = l.f21ef;let eq65_e1035: f64 = (p.p355 * (nv2 - nv15));let eq65_e1036_q: f64 = eq65_e1035;let eq65_e1037: f64 = (l.f21ef + eq65_e1035);let eq65_e1037_d_n2: f64 = (l.f21f2 + p.p355);let eq65_e1037_d_n15: f64 = (l.f21f1 + (-p.p355));let eq65_e1037_q: f64 = (eq65_e1032_q + eq65_e1036_q);
        (eq65_e1037, eq65_e1037_d_n2, l.f21f3, l.f21f4, l.f21f0, eq65_e1037_d_n15, eq65_e1037_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq65_e1039_d_n2, 0.0, eq65_e1039_d_n4, 0.0, 0.0, eq65_e1039_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq65_e1039_d_n14, eq65_e1039_d_n15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq65_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(15),
            &eq65_reactive_node_derivatives,
            &eq65_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq66_e1050, eq66_e1050_d_n2, eq66_e1050_d_n3, eq66_e1050_d_n4, eq66_e1050_d_n7, eq66_e1050_d_n14, eq66_e1050_d_n15, eq66_e1050_q,) = {
    if (l.f1dcb == 0.0) {
        let eq66_e1043_q: f64 = l.f21a6;let eq66_e1046: f64 = (p.p355 * (nv7 - nv14));let eq66_e1047_q: f64 = eq66_e1046;let eq66_e1048: f64 = (l.f21a6 + eq66_e1046);let eq66_e1048_d_n7: f64 = (l.f21ac + p.p355);let eq66_e1048_d_n14: f64 = (l.f21a7 + (-p.p355));let eq66_e1048_q: f64 = (eq66_e1043_q + eq66_e1047_q);
        (eq66_e1048, l.f21a9, l.f21aa, l.f21ab, eq66_e1048_d_n7, eq66_e1048_d_n14, l.f21a8, eq66_e1048_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq66_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq66_e1050_d_n2, eq66_e1050_d_n3, eq66_e1050_d_n4, 0.0, 0.0, eq66_e1050_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq66_e1050_d_n14, eq66_e1050_d_n15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq66_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(14),
            &eq66_reactive_node_derivatives,
            &eq66_reactive_branch_derivatives,
            multiplicity,
        );let eq69_e1062_q: f64 = l.f2166;let eq69_e1065: f64 = (p.p355 * (nv3 - nv14));let eq69_e1066_q: f64 = eq69_e1065;let eq69_e1067: f64 = (l.f2166 + eq69_e1065);let eq69_e1067_d_n3: f64 = (l.f216a + p.p355);let eq69_e1067_d_n14: f64 = (l.f2167 + (-p.p355));let eq69_e1067_q: f64 = (eq69_e1062_q + eq69_e1066_q);let eq69_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, l.f2169, eq69_e1067_d_n3, l.f216b, 0.0, 0.0, l.f216c, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq69_e1067_d_n14, l.f2168, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq69_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(3),
            Some(14),
            &eq69_reactive_node_derivatives,
            &eq69_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);let nv5 = ctx.node_voltage(nodes[5]);let nv7 = ctx.node_voltage(nodes[7]);let nv9 = ctx.node_voltage(nodes[9]);let nv14 = ctx.node_voltage(nodes[14]);
        let (eq72_e1090, eq72_e1090_d_n2, eq72_e1090_d_n4, eq72_e1090_d_n5, eq72_e1090_d_n7, eq72_e1090_d_n14, eq72_e1090_q,) = {
    if (l.f1e15 != 0.0) {
        let eq72_e1083_q: f64 = l.f222a;let eq72_e1086: f64 = (p.p355 * (nv7 - nv5));let eq72_e1087_q: f64 = eq72_e1086;let eq72_e1088: f64 = (l.f222a + eq72_e1086);let eq72_e1088_d_n5: f64 = (l.f222e + (-p.p355));let eq72_e1088_d_n7: f64 = (l.f222f + p.p355);let eq72_e1088_q: f64 = (eq72_e1083_q + eq72_e1087_q);
        (eq72_e1088, l.f222c, l.f222d, eq72_e1088_d_n5, eq72_e1088_d_n7, l.f222b, eq72_e1088_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq72_e1090_d_n2, 0.0, eq72_e1090_d_n4, eq72_e1090_d_n5, 0.0, eq72_e1090_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq72_e1090_d_n14, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq72_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(5),
            &eq72_reactive_node_derivatives,
            &eq72_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq73_e1100, eq73_e1100_d_n2, eq73_e1100_d_n4, eq73_e1100_d_n5, eq73_e1100_d_n7, eq73_e1100_d_n14, eq73_e1100_q,) = {
    if (l.f1e15 != 0.0) {
        let eq73_e1093_q: f64 = l.f21e8;let eq73_e1096: f64 = (p.p355 * (nv7 - nv14));let eq73_e1097_q: f64 = eq73_e1096;let eq73_e1098: f64 = (l.f21e8 + eq73_e1096);let eq73_e1098_d_n7: f64 = (l.f21ed + p.p355);let eq73_e1098_d_n14: f64 = (l.f21e9 + (-p.p355));let eq73_e1098_q: f64 = (eq73_e1093_q + eq73_e1097_q);
        (eq73_e1098, l.f21ea, l.f21eb, l.f21ec, eq73_e1098_d_n7, eq73_e1098_d_n14, eq73_e1098_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq73_e1100_d_n2, 0.0, eq73_e1100_d_n4, eq73_e1100_d_n5, 0.0, eq73_e1100_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq73_e1100_d_n14, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq73_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(14),
            &eq73_reactive_node_derivatives,
            &eq73_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq74_e1110, eq74_e1110_d_n2, eq74_e1110_d_n3, eq74_e1110_d_n4, eq74_e1110_d_n5, eq74_e1110_d_n7, eq74_e1110_d_n14, eq74_e1110_q,) = {
    if (l.f1e15 != 0.0) {
        let eq74_e1103_q: f64 = l.f219e;let eq74_e1106: f64 = (p.p355 * (nv2 - nv5));let eq74_e1107_q: f64 = eq74_e1106;let eq74_e1108: f64 = (l.f219e + eq74_e1106);let eq74_e1108_d_n2: f64 = (l.f21a0 + p.p355);let eq74_e1108_d_n5: f64 = (l.f21a3 + (-p.p355));let eq74_e1108_q: f64 = (eq74_e1103_q + eq74_e1107_q);
        (eq74_e1108, eq74_e1108_d_n2, l.f21a1, l.f21a2, eq74_e1108_d_n5, l.f21a4, l.f219f, eq74_e1108_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq74_e1110_d_n2, eq74_e1110_d_n3, eq74_e1110_d_n4, eq74_e1110_d_n5, 0.0, eq74_e1110_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq74_e1110_d_n14, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq74_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(5),
            &eq74_reactive_node_derivatives,
            &eq74_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq76_e1124, eq76_e1124_d_n2, eq76_e1124_d_n3, eq76_e1124_d_n4, eq76_e1124_d_n5, eq76_e1124_d_n7, eq76_e1124_d_n9, eq76_e1124_d_n14, eq76_e1124_q,) = {
    if (l.f1e15 != 0.0) {
        let eq76_e1117_q: f64 = l.f22a4;let eq76_e1120: f64 = (p.p355 * (nv7 - nv9));let eq76_e1121_q: f64 = eq76_e1120;let eq76_e1122: f64 = (l.f22a4 + eq76_e1120);let eq76_e1122_d_n7: f64 = (l.f22aa + p.p355);let eq76_e1122_q: f64 = (eq76_e1117_q + eq76_e1121_q);
        (eq76_e1122, l.f22a6, l.f22a7, l.f22a8, l.f22a9, eq76_e1122_d_n7, (-p.p355), l.f22a5, eq76_e1122_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq76_e1124_d_n2, eq76_e1124_d_n3, eq76_e1124_d_n4, eq76_e1124_d_n5, 0.0, eq76_e1124_d_n7, 0.0, eq76_e1124_d_n9, 0.0, 0.0, 0.0, 0.0, eq76_e1124_d_n14, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq76_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(9),
            &eq76_reactive_node_derivatives,
            &eq76_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq77_e1135, eq77_e1135_d_n2, eq77_e1135_d_n4, eq77_e1135_d_n5, eq77_e1135_d_n7, eq77_e1135_d_n14, eq77_e1135_q,) = {
    if (l.f1e15 == 0.0) {
        let eq77_e1128_q: f64 = l.f222a;let eq77_e1131: f64 = (p.p355 * (nv2 - nv5));let eq77_e1132_q: f64 = eq77_e1131;let eq77_e1133: f64 = (l.f222a + eq77_e1131);let eq77_e1133_d_n2: f64 = (l.f222c + p.p355);let eq77_e1133_d_n5: f64 = (l.f222e + (-p.p355));let eq77_e1133_q: f64 = (eq77_e1128_q + eq77_e1132_q);
        (eq77_e1133, eq77_e1133_d_n2, l.f222d, eq77_e1133_d_n5, l.f222f, l.f222b, eq77_e1133_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq77_e1135_d_n2, 0.0, eq77_e1135_d_n4, eq77_e1135_d_n5, 0.0, eq77_e1135_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq77_e1135_d_n14, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq77_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(5),
            &eq77_reactive_node_derivatives,
            &eq77_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq78_e1146, eq78_e1146_d_n2, eq78_e1146_d_n4, eq78_e1146_d_n5, eq78_e1146_d_n7, eq78_e1146_d_n14, eq78_e1146_q,) = {
    if (l.f1e15 == 0.0) {
        let eq78_e1139_q: f64 = l.f21e8;let eq78_e1142: f64 = (p.p355 * (nv2 - nv14));let eq78_e1143_q: f64 = eq78_e1142;let eq78_e1144: f64 = (l.f21e8 + eq78_e1142);let eq78_e1144_d_n2: f64 = (l.f21ea + p.p355);let eq78_e1144_d_n14: f64 = (l.f21e9 + (-p.p355));let eq78_e1144_q: f64 = (eq78_e1139_q + eq78_e1143_q);
        (eq78_e1144, eq78_e1144_d_n2, l.f21eb, l.f21ec, l.f21ed, eq78_e1144_d_n14, eq78_e1144_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq78_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq78_e1146_d_n2, 0.0, eq78_e1146_d_n4, eq78_e1146_d_n5, 0.0, eq78_e1146_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq78_e1146_d_n14, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq78_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(14),
            &eq78_reactive_node_derivatives,
            &eq78_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq79_e1157, eq79_e1157_d_n2, eq79_e1157_d_n3, eq79_e1157_d_n4, eq79_e1157_d_n5, eq79_e1157_d_n7, eq79_e1157_d_n14, eq79_e1157_q,) = {
    if (l.f1e15 == 0.0) {
        let eq79_e1150_q: f64 = l.f219e;let eq79_e1153: f64 = (p.p355 * (nv7 - nv5));let eq79_e1154_q: f64 = eq79_e1153;let eq79_e1155: f64 = (l.f219e + eq79_e1153);let eq79_e1155_d_n5: f64 = (l.f21a3 + (-p.p355));let eq79_e1155_d_n7: f64 = (l.f21a4 + p.p355);let eq79_e1155_q: f64 = (eq79_e1150_q + eq79_e1154_q);
        (eq79_e1155, l.f21a0, l.f21a1, l.f21a2, eq79_e1155_d_n5, eq79_e1155_d_n7, l.f219f, eq79_e1155_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq79_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq79_e1157_d_n2, eq79_e1157_d_n3, eq79_e1157_d_n4, eq79_e1157_d_n5, 0.0, eq79_e1157_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq79_e1157_d_n14, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq79_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(5),
            &eq79_reactive_node_derivatives,
            &eq79_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_5(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);let nv3 = ctx.node_voltage(nodes[3]);let nv5 = ctx.node_voltage(nodes[5]);let nv7 = ctx.node_voltage(nodes[7]);let nv9 = ctx.node_voltage(nodes[9]);let nv10 = ctx.node_voltage(nodes[10]);let eq82_e1169_q: f64 = l.f215e;let eq82_e1172: f64 = (p.p355 * (nv3 - nv5));let eq82_e1173_q: f64 = eq82_e1172;let eq82_e1174: f64 = (l.f215e + eq82_e1172);let eq82_e1174_d_n3: f64 = (l.f2161 + p.p355);let eq82_e1174_d_n5: f64 = (l.f2163 + (-p.p355));let eq82_e1174_q: f64 = (eq82_e1169_q + eq82_e1173_q);let eq82_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, l.f2160, eq82_e1174_d_n3, l.f2162, eq82_e1174_d_n5, 0.0, l.f2164, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, l.f215f, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq82_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(3),
            Some(5),
            &eq82_reactive_node_derivatives,
            &eq82_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq85_e1197, eq85_e1197_d_n2, eq85_e1197_d_n4, eq85_e1197_d_n7, eq85_e1197_d_n9, eq85_e1197_d_n10, eq85_e1197_q,) = {
    if (l.f1e64 != 0.0) {
        let eq85_e1190_q: f64 = l.f2246;let eq85_e1193: f64 = (p.p355 * (nv7 - nv10));let eq85_e1194_q: f64 = eq85_e1193;let eq85_e1195: f64 = (l.f2246 + eq85_e1193);let eq85_e1195_d_n7: f64 = (l.f224a + p.p355);let eq85_e1195_d_n10: f64 = (l.f2247 + (-p.p355));let eq85_e1195_q: f64 = (eq85_e1190_q + eq85_e1194_q);
        (eq85_e1195, l.f2248, l.f2249, eq85_e1195_d_n7, l.f224b, eq85_e1195_d_n10, eq85_e1195_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq85_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq85_e1197_d_n2, 0.0, eq85_e1197_d_n4, 0.0, 0.0, eq85_e1197_d_n7, 0.0, eq85_e1197_d_n9, eq85_e1197_d_n10, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq85_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(10),
            &eq85_reactive_node_derivatives,
            &eq85_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq86_e1207, eq86_e1207_d_n2, eq86_e1207_d_n4, eq86_e1207_d_n7, eq86_e1207_d_n9, eq86_e1207_d_n10, eq86_e1207_q,) = {
    if (l.f1e64 != 0.0) {
        let eq86_e1200_q: f64 = l.f2204;let eq86_e1203: f64 = (p.p355 * (nv7 - nv9));let eq86_e1204_q: f64 = eq86_e1203;let eq86_e1205: f64 = (l.f2204 + eq86_e1203);let eq86_e1205_d_n7: f64 = (l.f2208 + p.p355);let eq86_e1205_d_n9: f64 = (l.f2209 + (-p.p355));let eq86_e1205_q: f64 = (eq86_e1200_q + eq86_e1204_q);
        (eq86_e1205, l.f2206, l.f2207, eq86_e1205_d_n7, eq86_e1205_d_n9, l.f2205, eq86_e1205_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq86_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq86_e1207_d_n2, 0.0, eq86_e1207_d_n4, 0.0, 0.0, eq86_e1207_d_n7, 0.0, eq86_e1207_d_n9, eq86_e1207_d_n10, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq86_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(9),
            &eq86_reactive_node_derivatives,
            &eq86_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq87_e1217, eq87_e1217_d_n2, eq87_e1217_d_n3, eq87_e1217_d_n4, eq87_e1217_d_n7, eq87_e1217_d_n9, eq87_e1217_d_n10, eq87_e1217_q,) = {
    if (l.f1e64 != 0.0) {
        let eq87_e1210_q: f64 = l.f21be;let eq87_e1213: f64 = (p.p355 * (nv2 - nv10));let eq87_e1214_q: f64 = eq87_e1213;let eq87_e1215: f64 = (l.f21be + eq87_e1213);let eq87_e1215_d_n2: f64 = (l.f21c0 + p.p355);let eq87_e1215_d_n10: f64 = (l.f21bf + (-p.p355));let eq87_e1215_q: f64 = (eq87_e1210_q + eq87_e1214_q);
        (eq87_e1215, eq87_e1215_d_n2, l.f21c1, l.f21c2, l.f21c3, l.f21c4, eq87_e1215_d_n10, eq87_e1215_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq87_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq87_e1217_d_n2, eq87_e1217_d_n3, eq87_e1217_d_n4, 0.0, 0.0, eq87_e1217_d_n7, 0.0, eq87_e1217_d_n9, eq87_e1217_d_n10, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq87_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(10),
            &eq87_reactive_node_derivatives,
            &eq87_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq89_e1231, eq89_e1231_d_n2, eq89_e1231_d_n3, eq89_e1231_d_n4, eq89_e1231_d_n7, eq89_e1231_d_n9, eq89_e1231_d_n10, eq89_e1231_q,) = {
    if (l.f1e64 != 0.0) {
        let eq89_e1224_q: f64 = l.f22c4;let eq89_e1227: f64 = (p.p355 * (nv7 - nv9));let eq89_e1228_q: f64 = eq89_e1227;let eq89_e1229: f64 = (l.f22c4 + eq89_e1227);let eq89_e1229_d_n7: f64 = (l.f22c9 + p.p355);let eq89_e1229_d_n9: f64 = (l.f22ca + (-p.p355));let eq89_e1229_q: f64 = (eq89_e1224_q + eq89_e1228_q);
        (eq89_e1229, l.f22c6, l.f22c7, l.f22c8, eq89_e1229_d_n7, eq89_e1229_d_n9, l.f22c5, eq89_e1229_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq89_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq89_e1231_d_n2, eq89_e1231_d_n3, eq89_e1231_d_n4, 0.0, 0.0, eq89_e1231_d_n7, 0.0, eq89_e1231_d_n9, eq89_e1231_d_n10, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq89_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(9),
            &eq89_reactive_node_derivatives,
            &eq89_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq90_e1242, eq90_e1242_d_n2, eq90_e1242_d_n4, eq90_e1242_d_n7, eq90_e1242_d_n9, eq90_e1242_d_n10, eq90_e1242_q,) = {
    if (l.f1e64 == 0.0) {
        let eq90_e1235_q: f64 = l.f2246;let eq90_e1238: f64 = (p.p355 * (nv2 - nv10));let eq90_e1239_q: f64 = eq90_e1238;let eq90_e1240: f64 = (l.f2246 + eq90_e1238);let eq90_e1240_d_n2: f64 = (l.f2248 + p.p355);let eq90_e1240_d_n10: f64 = (l.f2247 + (-p.p355));let eq90_e1240_q: f64 = (eq90_e1235_q + eq90_e1239_q);
        (eq90_e1240, eq90_e1240_d_n2, l.f2249, l.f224a, l.f224b, eq90_e1240_d_n10, eq90_e1240_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq90_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq90_e1242_d_n2, 0.0, eq90_e1242_d_n4, 0.0, 0.0, eq90_e1242_d_n7, 0.0, eq90_e1242_d_n9, eq90_e1242_d_n10, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq90_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(10),
            &eq90_reactive_node_derivatives,
            &eq90_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq91_e1253, eq91_e1253_d_n2, eq91_e1253_d_n4, eq91_e1253_d_n7, eq91_e1253_d_n9, eq91_e1253_d_n10, eq91_e1253_q,) = {
    if (l.f1e64 == 0.0) {
        let eq91_e1246_q: f64 = l.f2204;let eq91_e1249: f64 = (p.p355 * (nv2 - nv9));let eq91_e1250_q: f64 = eq91_e1249;let eq91_e1251: f64 = (l.f2204 + eq91_e1249);let eq91_e1251_d_n2: f64 = (l.f2206 + p.p355);let eq91_e1251_d_n9: f64 = (l.f2209 + (-p.p355));let eq91_e1251_q: f64 = (eq91_e1246_q + eq91_e1250_q);
        (eq91_e1251, eq91_e1251_d_n2, l.f2207, l.f2208, eq91_e1251_d_n9, l.f2205, eq91_e1251_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq91_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq91_e1253_d_n2, 0.0, eq91_e1253_d_n4, 0.0, 0.0, eq91_e1253_d_n7, 0.0, eq91_e1253_d_n9, eq91_e1253_d_n10, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq91_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(9),
            &eq91_reactive_node_derivatives,
            &eq91_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_6(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);let nv3 = ctx.node_voltage(nodes[3]);let nv7 = ctx.node_voltage(nodes[7]);let nv9 = ctx.node_voltage(nodes[9]);let nv10 = ctx.node_voltage(nodes[10]);let nv11 = ctx.node_voltage(nodes[11]);
        let (eq92_e1264, eq92_e1264_d_n2, eq92_e1264_d_n3, eq92_e1264_d_n4, eq92_e1264_d_n7, eq92_e1264_d_n9, eq92_e1264_d_n10, eq92_e1264_q,) = {
    if (l.f1e64 == 0.0) {
        let eq92_e1257_q: f64 = l.f21be;let eq92_e1260: f64 = (p.p355 * (nv7 - nv10));let eq92_e1261_q: f64 = eq92_e1260;let eq92_e1262: f64 = (l.f21be + eq92_e1260);let eq92_e1262_d_n7: f64 = (l.f21c3 + p.p355);let eq92_e1262_d_n10: f64 = (l.f21bf + (-p.p355));let eq92_e1262_q: f64 = (eq92_e1257_q + eq92_e1261_q);
        (eq92_e1262, l.f21c0, l.f21c1, l.f21c2, eq92_e1262_d_n7, l.f21c4, eq92_e1262_d_n10, eq92_e1262_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq92_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq92_e1264_d_n2, eq92_e1264_d_n3, eq92_e1264_d_n4, 0.0, 0.0, eq92_e1264_d_n7, 0.0, eq92_e1264_d_n9, eq92_e1264_d_n10, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq92_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(10),
            &eq92_reactive_node_derivatives,
            &eq92_reactive_branch_derivatives,
            multiplicity,
        );let eq95_e1276_q: f64 = l.f217e;let eq95_e1279: f64 = (p.p355 * (nv3 - nv10));let eq95_e1280_q: f64 = eq95_e1279;let eq95_e1281: f64 = (l.f217e + eq95_e1279);let eq95_e1281_d_n3: f64 = (l.f2181 + p.p355);let eq95_e1281_d_n10: f64 = (l.f217f + (-p.p355));let eq95_e1281_q: f64 = (eq95_e1276_q + eq95_e1280_q);let eq95_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, l.f2180, eq95_e1281_d_n3, l.f2182, 0.0, 0.0, l.f2183, 0.0, l.f2184, eq95_e1281_d_n10, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq95_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(3),
            Some(10),
            &eq95_reactive_node_derivatives,
            &eq95_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq98_e1304, eq98_e1304_d_n2, eq98_e1304_d_n4, eq98_e1304_d_n7, eq98_e1304_d_n10, eq98_e1304_d_n11, eq98_e1304_q,) = {
    if (l.f1eb0 != 0.0) {
        let eq98_e1297_q: f64 = l.f224d;let eq98_e1300: f64 = (p.p355 * (nv7 - nv11));let eq98_e1301_q: f64 = eq98_e1300;let eq98_e1302: f64 = (l.f224d + eq98_e1300);let eq98_e1302_d_n7: f64 = (l.f2252 + p.p355);let eq98_e1302_d_n11: f64 = (l.f224f + (-p.p355));let eq98_e1302_q: f64 = (eq98_e1297_q + eq98_e1301_q);
        (eq98_e1302, l.f2250, l.f2251, eq98_e1302_d_n7, l.f224e, eq98_e1302_d_n11, eq98_e1302_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq98_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq98_e1304_d_n2, 0.0, eq98_e1304_d_n4, 0.0, 0.0, eq98_e1304_d_n7, 0.0, 0.0, eq98_e1304_d_n10, eq98_e1304_d_n11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq98_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(11),
            &eq98_reactive_node_derivatives,
            &eq98_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq99_e1314, eq99_e1314_d_n2, eq99_e1314_d_n4, eq99_e1314_d_n7, eq99_e1314_d_n10, eq99_e1314_d_n11, eq99_e1314_q,) = {
    if (l.f1eb0 != 0.0) {
        let eq99_e1307_q: f64 = l.f220b;let eq99_e1310: f64 = (p.p355 * (nv7 - nv10));let eq99_e1311_q: f64 = eq99_e1310;let eq99_e1312: f64 = (l.f220b + eq99_e1310);let eq99_e1312_d_n7: f64 = (l.f2210 + p.p355);let eq99_e1312_d_n10: f64 = (l.f220c + (-p.p355));let eq99_e1312_q: f64 = (eq99_e1307_q + eq99_e1311_q);
        (eq99_e1312, l.f220e, l.f220f, eq99_e1312_d_n7, eq99_e1312_d_n10, l.f220d, eq99_e1312_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq99_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq99_e1314_d_n2, 0.0, eq99_e1314_d_n4, 0.0, 0.0, eq99_e1314_d_n7, 0.0, 0.0, eq99_e1314_d_n10, eq99_e1314_d_n11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq99_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(10),
            &eq99_reactive_node_derivatives,
            &eq99_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq100_e1324, eq100_e1324_d_n2, eq100_e1324_d_n3, eq100_e1324_d_n4, eq100_e1324_d_n7, eq100_e1324_d_n10, eq100_e1324_d_n11, eq100_e1324_q,) = {
    if (l.f1eb0 != 0.0) {
        let eq100_e1317_q: f64 = l.f21c6;let eq100_e1320: f64 = (p.p355 * (nv2 - nv11));let eq100_e1321_q: f64 = eq100_e1320;let eq100_e1322: f64 = (l.f21c6 + eq100_e1320);let eq100_e1322_d_n2: f64 = (l.f21c9 + p.p355);let eq100_e1322_d_n11: f64 = (l.f21c8 + (-p.p355));let eq100_e1322_q: f64 = (eq100_e1317_q + eq100_e1321_q);
        (eq100_e1322, eq100_e1322_d_n2, l.f21ca, l.f21cb, l.f21cc, l.f21c7, eq100_e1322_d_n11, eq100_e1322_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq100_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq100_e1324_d_n2, eq100_e1324_d_n3, eq100_e1324_d_n4, 0.0, 0.0, eq100_e1324_d_n7, 0.0, 0.0, eq100_e1324_d_n10, eq100_e1324_d_n11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq100_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(11),
            &eq100_reactive_node_derivatives,
            &eq100_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq102_e1338, eq102_e1338_d_n2, eq102_e1338_d_n3, eq102_e1338_d_n4, eq102_e1338_d_n7, eq102_e1338_d_n9, eq102_e1338_d_n10, eq102_e1338_d_n11, eq102_e1338_q,) = {
    if (l.f1eb0 != 0.0) {
        let eq102_e1331_q: f64 = l.f22cc;let eq102_e1334: f64 = (p.p355 * (nv7 - nv9));let eq102_e1335_q: f64 = eq102_e1334;let eq102_e1336: f64 = (l.f22cc + eq102_e1334);let eq102_e1336_d_n7: f64 = (l.f22d2 + p.p355);let eq102_e1336_q: f64 = (eq102_e1331_q + eq102_e1335_q);
        (eq102_e1336, l.f22cf, l.f22d0, l.f22d1, eq102_e1336_d_n7, (-p.p355), l.f22cd, l.f22ce, eq102_e1336_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq102_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq102_e1338_d_n2, eq102_e1338_d_n3, eq102_e1338_d_n4, 0.0, 0.0, eq102_e1338_d_n7, 0.0, eq102_e1338_d_n9, eq102_e1338_d_n10, eq102_e1338_d_n11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq102_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(9),
            &eq102_reactive_node_derivatives,
            &eq102_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq103_e1349, eq103_e1349_d_n2, eq103_e1349_d_n4, eq103_e1349_d_n7, eq103_e1349_d_n10, eq103_e1349_d_n11, eq103_e1349_q,) = {
    if (l.f1eb0 == 0.0) {
        let eq103_e1342_q: f64 = l.f224d;let eq103_e1345: f64 = (p.p355 * (nv2 - nv11));let eq103_e1346_q: f64 = eq103_e1345;let eq103_e1347: f64 = (l.f224d + eq103_e1345);let eq103_e1347_d_n2: f64 = (l.f2250 + p.p355);let eq103_e1347_d_n11: f64 = (l.f224f + (-p.p355));let eq103_e1347_q: f64 = (eq103_e1342_q + eq103_e1346_q);
        (eq103_e1347, eq103_e1347_d_n2, l.f2251, l.f2252, l.f224e, eq103_e1347_d_n11, eq103_e1347_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq103_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq103_e1349_d_n2, 0.0, eq103_e1349_d_n4, 0.0, 0.0, eq103_e1349_d_n7, 0.0, 0.0, eq103_e1349_d_n10, eq103_e1349_d_n11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq103_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(11),
            &eq103_reactive_node_derivatives,
            &eq103_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_7(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);let nv3 = ctx.node_voltage(nodes[3]);let nv7 = ctx.node_voltage(nodes[7]);let nv9 = ctx.node_voltage(nodes[9]);let nv10 = ctx.node_voltage(nodes[10]);let nv11 = ctx.node_voltage(nodes[11]);let nv12 = ctx.node_voltage(nodes[12]);
        let (eq104_e1360, eq104_e1360_d_n2, eq104_e1360_d_n4, eq104_e1360_d_n7, eq104_e1360_d_n10, eq104_e1360_d_n11, eq104_e1360_q,) = {
    if (l.f1eb0 == 0.0) {
        let eq104_e1353_q: f64 = l.f220b;let eq104_e1356: f64 = (p.p355 * (nv2 - nv10));let eq104_e1357_q: f64 = eq104_e1356;let eq104_e1358: f64 = (l.f220b + eq104_e1356);let eq104_e1358_d_n2: f64 = (l.f220e + p.p355);let eq104_e1358_d_n10: f64 = (l.f220c + (-p.p355));let eq104_e1358_q: f64 = (eq104_e1353_q + eq104_e1357_q);
        (eq104_e1358, eq104_e1358_d_n2, l.f220f, l.f2210, eq104_e1358_d_n10, l.f220d, eq104_e1358_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq104_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq104_e1360_d_n2, 0.0, eq104_e1360_d_n4, 0.0, 0.0, eq104_e1360_d_n7, 0.0, 0.0, eq104_e1360_d_n10, eq104_e1360_d_n11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq104_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(10),
            &eq104_reactive_node_derivatives,
            &eq104_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq105_e1371, eq105_e1371_d_n2, eq105_e1371_d_n3, eq105_e1371_d_n4, eq105_e1371_d_n7, eq105_e1371_d_n10, eq105_e1371_d_n11, eq105_e1371_q,) = {
    if (l.f1eb0 == 0.0) {
        let eq105_e1364_q: f64 = l.f21c6;let eq105_e1367: f64 = (p.p355 * (nv7 - nv11));let eq105_e1368_q: f64 = eq105_e1367;let eq105_e1369: f64 = (l.f21c6 + eq105_e1367);let eq105_e1369_d_n7: f64 = (l.f21cc + p.p355);let eq105_e1369_d_n11: f64 = (l.f21c8 + (-p.p355));let eq105_e1369_q: f64 = (eq105_e1364_q + eq105_e1368_q);
        (eq105_e1369, l.f21c9, l.f21ca, l.f21cb, eq105_e1369_d_n7, l.f21c7, eq105_e1369_d_n11, eq105_e1369_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq105_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq105_e1371_d_n2, eq105_e1371_d_n3, eq105_e1371_d_n4, 0.0, 0.0, eq105_e1371_d_n7, 0.0, 0.0, eq105_e1371_d_n10, eq105_e1371_d_n11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq105_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(11),
            &eq105_reactive_node_derivatives,
            &eq105_reactive_branch_derivatives,
            multiplicity,
        );let eq108_e1383_q: f64 = l.f2186;let eq108_e1386: f64 = (p.p355 * (nv3 - nv11));let eq108_e1387_q: f64 = eq108_e1386;let eq108_e1388: f64 = (l.f2186 + eq108_e1386);let eq108_e1388_d_n3: f64 = (l.f218a + p.p355);let eq108_e1388_d_n11: f64 = (l.f2188 + (-p.p355));let eq108_e1388_q: f64 = (eq108_e1383_q + eq108_e1387_q);let eq108_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, l.f2189, eq108_e1388_d_n3, l.f218b, 0.0, 0.0, l.f218c, 0.0, 0.0, l.f2187, eq108_e1388_d_n11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq108_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(3),
            Some(11),
            &eq108_reactive_node_derivatives,
            &eq108_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq111_e1411, eq111_e1411_d_n2, eq111_e1411_d_n4, eq111_e1411_d_n7, eq111_e1411_d_n11, eq111_e1411_d_n12, eq111_e1411_q,) = {
    if (l.f1efc != 0.0) {
        let eq111_e1404_q: f64 = l.f2254;let eq111_e1407: f64 = (p.p355 * (nv7 - nv12));let eq111_e1408_q: f64 = eq111_e1407;let eq111_e1409: f64 = (l.f2254 + eq111_e1407);let eq111_e1409_d_n7: f64 = (l.f2259 + p.p355);let eq111_e1409_d_n12: f64 = (l.f2256 + (-p.p355));let eq111_e1409_q: f64 = (eq111_e1404_q + eq111_e1408_q);
        (eq111_e1409, l.f2257, l.f2258, eq111_e1409_d_n7, l.f2255, eq111_e1409_d_n12, eq111_e1409_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq111_e1411_d_n2, 0.0, eq111_e1411_d_n4, 0.0, 0.0, eq111_e1411_d_n7, 0.0, 0.0, 0.0, eq111_e1411_d_n11, eq111_e1411_d_n12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq111_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(12),
            &eq111_reactive_node_derivatives,
            &eq111_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq112_e1421, eq112_e1421_d_n2, eq112_e1421_d_n4, eq112_e1421_d_n7, eq112_e1421_d_n11, eq112_e1421_d_n12, eq112_e1421_q,) = {
    if (l.f1efc != 0.0) {
        let eq112_e1414_q: f64 = l.f2212;let eq112_e1417: f64 = (p.p355 * (nv7 - nv11));let eq112_e1418_q: f64 = eq112_e1417;let eq112_e1419: f64 = (l.f2212 + eq112_e1417);let eq112_e1419_d_n7: f64 = (l.f2217 + p.p355);let eq112_e1419_d_n11: f64 = (l.f2213 + (-p.p355));let eq112_e1419_q: f64 = (eq112_e1414_q + eq112_e1418_q);
        (eq112_e1419, l.f2215, l.f2216, eq112_e1419_d_n7, eq112_e1419_d_n11, l.f2214, eq112_e1419_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq112_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq112_e1421_d_n2, 0.0, eq112_e1421_d_n4, 0.0, 0.0, eq112_e1421_d_n7, 0.0, 0.0, 0.0, eq112_e1421_d_n11, eq112_e1421_d_n12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq112_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(11),
            &eq112_reactive_node_derivatives,
            &eq112_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq113_e1431, eq113_e1431_d_n2, eq113_e1431_d_n3, eq113_e1431_d_n4, eq113_e1431_d_n7, eq113_e1431_d_n11, eq113_e1431_d_n12, eq113_e1431_q,) = {
    if (l.f1efc != 0.0) {
        let eq113_e1424_q: f64 = l.f21ce;let eq113_e1427: f64 = (p.p355 * (nv2 - nv12));let eq113_e1428_q: f64 = eq113_e1427;let eq113_e1429: f64 = (l.f21ce + eq113_e1427);let eq113_e1429_d_n2: f64 = (l.f21d1 + p.p355);let eq113_e1429_d_n12: f64 = (l.f21d0 + (-p.p355));let eq113_e1429_q: f64 = (eq113_e1424_q + eq113_e1428_q);
        (eq113_e1429, eq113_e1429_d_n2, l.f21d2, l.f21d3, l.f21d4, l.f21cf, eq113_e1429_d_n12, eq113_e1429_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq113_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq113_e1431_d_n2, eq113_e1431_d_n3, eq113_e1431_d_n4, 0.0, 0.0, eq113_e1431_d_n7, 0.0, 0.0, 0.0, eq113_e1431_d_n11, eq113_e1431_d_n12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq113_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(12),
            &eq113_reactive_node_derivatives,
            &eq113_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq115_e1445, eq115_e1445_d_n2, eq115_e1445_d_n3, eq115_e1445_d_n4, eq115_e1445_d_n7, eq115_e1445_d_n9, eq115_e1445_d_n11, eq115_e1445_d_n12, eq115_e1445_q,) = {
    if (l.f1efc != 0.0) {
        let eq115_e1438_q: f64 = l.f22d4;let eq115_e1441: f64 = (p.p355 * (nv7 - nv9));let eq115_e1442_q: f64 = eq115_e1441;let eq115_e1443: f64 = (l.f22d4 + eq115_e1441);let eq115_e1443_d_n7: f64 = (l.f22da + p.p355);let eq115_e1443_q: f64 = (eq115_e1438_q + eq115_e1442_q);
        (eq115_e1443, l.f22d7, l.f22d8, l.f22d9, eq115_e1443_d_n7, (-p.p355), l.f22d5, l.f22d6, eq115_e1443_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq115_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq115_e1445_d_n2, eq115_e1445_d_n3, eq115_e1445_d_n4, 0.0, 0.0, eq115_e1445_d_n7, 0.0, eq115_e1445_d_n9, 0.0, eq115_e1445_d_n11, eq115_e1445_d_n12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq115_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(9),
            &eq115_reactive_node_derivatives,
            &eq115_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_8(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);let nv3 = ctx.node_voltage(nodes[3]);let nv7 = ctx.node_voltage(nodes[7]);let nv11 = ctx.node_voltage(nodes[11]);let nv12 = ctx.node_voltage(nodes[12]);let nv13 = ctx.node_voltage(nodes[13]);
        let (eq116_e1456, eq116_e1456_d_n2, eq116_e1456_d_n4, eq116_e1456_d_n7, eq116_e1456_d_n11, eq116_e1456_d_n12, eq116_e1456_q,) = {
    if (l.f1efc == 0.0) {
        let eq116_e1449_q: f64 = l.f2254;let eq116_e1452: f64 = (p.p355 * (nv2 - nv12));let eq116_e1453_q: f64 = eq116_e1452;let eq116_e1454: f64 = (l.f2254 + eq116_e1452);let eq116_e1454_d_n2: f64 = (l.f2257 + p.p355);let eq116_e1454_d_n12: f64 = (l.f2256 + (-p.p355));let eq116_e1454_q: f64 = (eq116_e1449_q + eq116_e1453_q);
        (eq116_e1454, eq116_e1454_d_n2, l.f2258, l.f2259, l.f2255, eq116_e1454_d_n12, eq116_e1454_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq116_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq116_e1456_d_n2, 0.0, eq116_e1456_d_n4, 0.0, 0.0, eq116_e1456_d_n7, 0.0, 0.0, 0.0, eq116_e1456_d_n11, eq116_e1456_d_n12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq116_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(12),
            &eq116_reactive_node_derivatives,
            &eq116_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq117_e1467, eq117_e1467_d_n2, eq117_e1467_d_n4, eq117_e1467_d_n7, eq117_e1467_d_n11, eq117_e1467_d_n12, eq117_e1467_q,) = {
    if (l.f1efc == 0.0) {
        let eq117_e1460_q: f64 = l.f2212;let eq117_e1463: f64 = (p.p355 * (nv2 - nv11));let eq117_e1464_q: f64 = eq117_e1463;let eq117_e1465: f64 = (l.f2212 + eq117_e1463);let eq117_e1465_d_n2: f64 = (l.f2215 + p.p355);let eq117_e1465_d_n11: f64 = (l.f2213 + (-p.p355));let eq117_e1465_q: f64 = (eq117_e1460_q + eq117_e1464_q);
        (eq117_e1465, eq117_e1465_d_n2, l.f2216, l.f2217, eq117_e1465_d_n11, l.f2214, eq117_e1465_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq117_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq117_e1467_d_n2, 0.0, eq117_e1467_d_n4, 0.0, 0.0, eq117_e1467_d_n7, 0.0, 0.0, 0.0, eq117_e1467_d_n11, eq117_e1467_d_n12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq117_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(11),
            &eq117_reactive_node_derivatives,
            &eq117_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq118_e1478, eq118_e1478_d_n2, eq118_e1478_d_n3, eq118_e1478_d_n4, eq118_e1478_d_n7, eq118_e1478_d_n11, eq118_e1478_d_n12, eq118_e1478_q,) = {
    if (l.f1efc == 0.0) {
        let eq118_e1471_q: f64 = l.f21ce;let eq118_e1474: f64 = (p.p355 * (nv7 - nv12));let eq118_e1475_q: f64 = eq118_e1474;let eq118_e1476: f64 = (l.f21ce + eq118_e1474);let eq118_e1476_d_n7: f64 = (l.f21d4 + p.p355);let eq118_e1476_d_n12: f64 = (l.f21d0 + (-p.p355));let eq118_e1476_q: f64 = (eq118_e1471_q + eq118_e1475_q);
        (eq118_e1476, l.f21d1, l.f21d2, l.f21d3, eq118_e1476_d_n7, l.f21cf, eq118_e1476_d_n12, eq118_e1476_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq118_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq118_e1478_d_n2, eq118_e1478_d_n3, eq118_e1478_d_n4, 0.0, 0.0, eq118_e1478_d_n7, 0.0, 0.0, 0.0, eq118_e1478_d_n11, eq118_e1478_d_n12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq118_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(12),
            &eq118_reactive_node_derivatives,
            &eq118_reactive_branch_derivatives,
            multiplicity,
        );let eq121_e1490_q: f64 = l.f218e;let eq121_e1493: f64 = (p.p355 * (nv3 - nv12));let eq121_e1494_q: f64 = eq121_e1493;let eq121_e1495: f64 = (l.f218e + eq121_e1493);let eq121_e1495_d_n3: f64 = (l.f2192 + p.p355);let eq121_e1495_d_n12: f64 = (l.f2190 + (-p.p355));let eq121_e1495_q: f64 = (eq121_e1490_q + eq121_e1494_q);let eq121_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, l.f2191, eq121_e1495_d_n3, l.f2193, 0.0, 0.0, l.f2194, 0.0, 0.0, 0.0, l.f218f, eq121_e1495_d_n12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq121_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(3),
            Some(12),
            &eq121_reactive_node_derivatives,
            &eq121_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq124_e1518, eq124_e1518_d_n2, eq124_e1518_d_n4, eq124_e1518_d_n7, eq124_e1518_d_n12, eq124_e1518_d_n13, eq124_e1518_q,) = {
    if (l.f1f4c != 0.0) {
        let eq124_e1511_q: f64 = l.f225b;let eq124_e1514: f64 = (p.p355 * (nv7 - nv13));let eq124_e1515_q: f64 = eq124_e1514;let eq124_e1516: f64 = (l.f225b + eq124_e1514);let eq124_e1516_d_n7: f64 = (l.f2260 + p.p355);let eq124_e1516_d_n13: f64 = (l.f225d + (-p.p355));let eq124_e1516_q: f64 = (eq124_e1511_q + eq124_e1515_q);
        (eq124_e1516, l.f225e, l.f225f, eq124_e1516_d_n7, l.f225c, eq124_e1516_d_n13, eq124_e1516_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq124_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq124_e1518_d_n2, 0.0, eq124_e1518_d_n4, 0.0, 0.0, eq124_e1518_d_n7, 0.0, 0.0, 0.0, 0.0, eq124_e1518_d_n12, eq124_e1518_d_n13, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq124_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(13),
            &eq124_reactive_node_derivatives,
            &eq124_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq125_e1528, eq125_e1528_d_n2, eq125_e1528_d_n4, eq125_e1528_d_n7, eq125_e1528_d_n12, eq125_e1528_d_n13, eq125_e1528_q,) = {
    if (l.f1f4c != 0.0) {
        let eq125_e1521_q: f64 = l.f2219;let eq125_e1524: f64 = (p.p355 * (nv7 - nv12));let eq125_e1525_q: f64 = eq125_e1524;let eq125_e1526: f64 = (l.f2219 + eq125_e1524);let eq125_e1526_d_n7: f64 = (l.f221e + p.p355);let eq125_e1526_d_n12: f64 = (l.f221a + (-p.p355));let eq125_e1526_q: f64 = (eq125_e1521_q + eq125_e1525_q);
        (eq125_e1526, l.f221c, l.f221d, eq125_e1526_d_n7, eq125_e1526_d_n12, l.f221b, eq125_e1526_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq125_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq125_e1528_d_n2, 0.0, eq125_e1528_d_n4, 0.0, 0.0, eq125_e1528_d_n7, 0.0, 0.0, 0.0, 0.0, eq125_e1528_d_n12, eq125_e1528_d_n13, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq125_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(12),
            &eq125_reactive_node_derivatives,
            &eq125_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq126_e1538, eq126_e1538_d_n2, eq126_e1538_d_n3, eq126_e1538_d_n4, eq126_e1538_d_n7, eq126_e1538_d_n12, eq126_e1538_d_n13, eq126_e1538_q,) = {
    if (l.f1f4c != 0.0) {
        let eq126_e1531_q: f64 = l.f21d6;let eq126_e1534: f64 = (p.p355 * (nv2 - nv13));let eq126_e1535_q: f64 = eq126_e1534;let eq126_e1536: f64 = (l.f21d6 + eq126_e1534);let eq126_e1536_d_n2: f64 = (l.f21d9 + p.p355);let eq126_e1536_d_n13: f64 = (l.f21d8 + (-p.p355));let eq126_e1536_q: f64 = (eq126_e1531_q + eq126_e1535_q);
        (eq126_e1536, eq126_e1536_d_n2, l.f21da, l.f21db, l.f21dc, l.f21d7, eq126_e1536_d_n13, eq126_e1536_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq126_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq126_e1538_d_n2, eq126_e1538_d_n3, eq126_e1538_d_n4, 0.0, 0.0, eq126_e1538_d_n7, 0.0, 0.0, 0.0, 0.0, eq126_e1538_d_n12, eq126_e1538_d_n13, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq126_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(13),
            &eq126_reactive_node_derivatives,
            &eq126_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_9(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);let nv3 = ctx.node_voltage(nodes[3]);let nv7 = ctx.node_voltage(nodes[7]);let nv8 = ctx.node_voltage(nodes[8]);let nv9 = ctx.node_voltage(nodes[9]);let nv12 = ctx.node_voltage(nodes[12]);let nv13 = ctx.node_voltage(nodes[13]);let nv28 = ctx.node_voltage(nodes[28]);let nv29 = ctx.node_voltage(nodes[29]);
        let (eq128_e1552, eq128_e1552_d_n2, eq128_e1552_d_n3, eq128_e1552_d_n4, eq128_e1552_d_n7, eq128_e1552_d_n9, eq128_e1552_d_n12, eq128_e1552_d_n13, eq128_e1552_q,) = {
    if (l.f1f4c != 0.0) {
        let eq128_e1545_q: f64 = l.f22dc;let eq128_e1548: f64 = (p.p355 * (nv7 - nv9));let eq128_e1549_q: f64 = eq128_e1548;let eq128_e1550: f64 = (l.f22dc + eq128_e1548);let eq128_e1550_d_n7: f64 = (l.f22e2 + p.p355);let eq128_e1550_q: f64 = (eq128_e1545_q + eq128_e1549_q);
        (eq128_e1550, l.f22df, l.f22e0, l.f22e1, eq128_e1550_d_n7, (-p.p355), l.f22dd, l.f22de, eq128_e1550_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq128_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq128_e1552_d_n2, eq128_e1552_d_n3, eq128_e1552_d_n4, 0.0, 0.0, eq128_e1552_d_n7, 0.0, eq128_e1552_d_n9, 0.0, 0.0, eq128_e1552_d_n12, eq128_e1552_d_n13, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq128_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(9),
            &eq128_reactive_node_derivatives,
            &eq128_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq129_e1563, eq129_e1563_d_n2, eq129_e1563_d_n4, eq129_e1563_d_n7, eq129_e1563_d_n12, eq129_e1563_d_n13, eq129_e1563_q,) = {
    if (l.f1f4c == 0.0) {
        let eq129_e1556_q: f64 = l.f225b;let eq129_e1559: f64 = (p.p355 * (nv2 - nv13));let eq129_e1560_q: f64 = eq129_e1559;let eq129_e1561: f64 = (l.f225b + eq129_e1559);let eq129_e1561_d_n2: f64 = (l.f225e + p.p355);let eq129_e1561_d_n13: f64 = (l.f225d + (-p.p355));let eq129_e1561_q: f64 = (eq129_e1556_q + eq129_e1560_q);
        (eq129_e1561, eq129_e1561_d_n2, l.f225f, l.f2260, l.f225c, eq129_e1561_d_n13, eq129_e1561_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq129_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq129_e1563_d_n2, 0.0, eq129_e1563_d_n4, 0.0, 0.0, eq129_e1563_d_n7, 0.0, 0.0, 0.0, 0.0, eq129_e1563_d_n12, eq129_e1563_d_n13, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq129_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(13),
            &eq129_reactive_node_derivatives,
            &eq129_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq130_e1574, eq130_e1574_d_n2, eq130_e1574_d_n4, eq130_e1574_d_n7, eq130_e1574_d_n12, eq130_e1574_d_n13, eq130_e1574_q,) = {
    if (l.f1f4c == 0.0) {
        let eq130_e1567_q: f64 = l.f2219;let eq130_e1570: f64 = (p.p355 * (nv2 - nv12));let eq130_e1571_q: f64 = eq130_e1570;let eq130_e1572: f64 = (l.f2219 + eq130_e1570);let eq130_e1572_d_n2: f64 = (l.f221c + p.p355);let eq130_e1572_d_n12: f64 = (l.f221a + (-p.p355));let eq130_e1572_q: f64 = (eq130_e1567_q + eq130_e1571_q);
        (eq130_e1572, eq130_e1572_d_n2, l.f221d, l.f221e, eq130_e1572_d_n12, l.f221b, eq130_e1572_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq130_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq130_e1574_d_n2, 0.0, eq130_e1574_d_n4, 0.0, 0.0, eq130_e1574_d_n7, 0.0, 0.0, 0.0, 0.0, eq130_e1574_d_n12, eq130_e1574_d_n13, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq130_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(2),
            Some(12),
            &eq130_reactive_node_derivatives,
            &eq130_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq131_e1585, eq131_e1585_d_n2, eq131_e1585_d_n3, eq131_e1585_d_n4, eq131_e1585_d_n7, eq131_e1585_d_n12, eq131_e1585_d_n13, eq131_e1585_q,) = {
    if (l.f1f4c == 0.0) {
        let eq131_e1578_q: f64 = l.f21d6;let eq131_e1581: f64 = (p.p355 * (nv7 - nv13));let eq131_e1582_q: f64 = eq131_e1581;let eq131_e1583: f64 = (l.f21d6 + eq131_e1581);let eq131_e1583_d_n7: f64 = (l.f21dc + p.p355);let eq131_e1583_d_n13: f64 = (l.f21d8 + (-p.p355));let eq131_e1583_q: f64 = (eq131_e1578_q + eq131_e1582_q);
        (eq131_e1583, l.f21d9, l.f21da, l.f21db, eq131_e1583_d_n7, l.f21d7, eq131_e1583_d_n13, eq131_e1583_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq131_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq131_e1585_d_n2, eq131_e1585_d_n3, eq131_e1585_d_n4, 0.0, 0.0, eq131_e1585_d_n7, 0.0, 0.0, 0.0, 0.0, eq131_e1585_d_n12, eq131_e1585_d_n13, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq131_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(13),
            &eq131_reactive_node_derivatives,
            &eq131_reactive_branch_derivatives,
            multiplicity,
        );let eq134_e1597_q: f64 = l.f2196;let eq134_e1600: f64 = (p.p355 * (nv3 - nv13));let eq134_e1601_q: f64 = eq134_e1600;let eq134_e1602: f64 = (l.f2196 + eq134_e1600);let eq134_e1602_d_n3: f64 = (l.f219a + p.p355);let eq134_e1602_d_n13: f64 = (l.f2198 + (-p.p355));let eq134_e1602_q: f64 = (eq134_e1597_q + eq134_e1601_q);let eq134_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, l.f2199, eq134_e1602_d_n3, l.f219b, 0.0, 0.0, l.f219c, 0.0, 0.0, 0.0, 0.0, l.f2197, eq134_e1602_d_n13, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];let eq134_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(3),
            Some(13),
            &eq134_reactive_node_derivatives,
            &eq134_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq142_e1656, eq142_e1656_d_n4, eq142_e1656_d_n5, eq142_e1656_d_n8, eq142_e1656_d_n9, eq142_e1656_d_n22, eq142_e1656_d_n23, eq142_e1656_d_n25, eq142_e1656_d_n26, eq142_e1656_d_n28, eq142_e1656_d_n29, eq142_e1656_q,) = {
    if (l.f1fd2 == 0.0) {
        let eq142_e1649: f64 = (l.f20bc - (nv29 - 0.0));let eq142_e1652: f64 = (p.p323 * (nv28 - 0.0));let eq142_e1653_q: f64 = eq142_e1652;let eq142_e1654: f64 = (eq142_e1649 - eq142_e1652);let eq142_e1654_q: f64 = (-eq142_e1653_q);
        (eq142_e1654, l.f20c1, l.f20c2, l.f20c3, l.f20c4, l.f20bd, l.f20be, l.f20bf, l.f20c0, (-p.p323), (-1.0), eq142_e1654_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(28),
            None,
            28,
            multiplicity * (eq142_e1656_d_n28),
        );
        let (eq143_e1670, eq143_e1670_d_n28, eq143_e1670_d_n29, eq143_e1670_q, eq143_e1670_q_d_n29,) = {
    if (l.f1fd2 == 0.0) {
        let eq143_e1661: f64 = ((nv28 - 0.0) - (nv29 - 0.0));let eq143_e1664: f64 = (p.p323 / 3.0);let eq143_e1666: f64 = (eq143_e1664 * (nv29 - 0.0));let eq143_e1667_q: f64 = eq143_e1666;let eq143_e1668: f64 = (eq143_e1661 - eq143_e1666);let eq143_e1668_d_n29: f64 = ((-1.0) - eq143_e1664);let eq143_e1668_q: f64 = (-eq143_e1667_q);
        (eq143_e1668, 1.0, eq143_e1668_d_n29, eq143_e1668_q, (-eq143_e1664),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(29),
            None,
            29,
            multiplicity * (eq143_e1670_q_d_n29),
        );let eq145_e1681_q: f64 = l.f2220;let eq145_e1684: f64 = (p.p355 * (nv8 - nv9));let eq145_e1685_q: f64 = eq145_e1684;let eq145_e1686: f64 = (l.f2220 + eq145_e1684);let eq145_e1686_d_n8: f64 = (l.f2227 + p.p355);let eq145_e1686_d_n9: f64 = (l.f2228 + (-p.p355));let eq145_e1686_q: f64 = (eq145_e1681_q + eq145_e1685_q);let eq145_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, 0.0, 0.0, l.f2225, l.f2226, 0.0, 0.0, eq145_e1686_d_n8, eq145_e1686_d_n9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, l.f2221, l.f2222, 0.0, l.f2223, l.f2224, 0.0, 0.0, 0.0];let eq145_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(8),
            Some(9),
            &eq145_reactive_node_derivatives,
            &eq145_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_10(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);let nv5 = ctx.node_voltage(nodes[5]);let nv8 = ctx.node_voltage(nodes[8]);let eq146_e1688_q: f64 = l.f21de;let eq146_e1691: f64 = (p.p355 * (nv8 - nv5));let eq146_e1692_q: f64 = eq146_e1691;let eq146_e1693: f64 = (l.f21de + eq146_e1691);let eq146_e1693_d_n5: f64 = (l.f21e4 + (-p.p355));let eq146_e1693_d_n8: f64 = (l.f21e5 + p.p355);let eq146_e1693_q: f64 = (eq146_e1688_q + eq146_e1692_q);let eq146_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, 0.0, 0.0, l.f21e3, eq146_e1693_d_n5, 0.0, 0.0, eq146_e1693_d_n8, l.f21e6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, l.f21df, l.f21e0, 0.0, l.f21e1, l.f21e2, 0.0, 0.0, 0.0];let eq146_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(8),
            Some(5),
            &eq146_reactive_node_derivatives,
            &eq146_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq157_e1796, eq157_e1796_d_n7, eq157_e1796_d_n8, eq157_e1796_q,) = {
    if (l.f2002 != 0.0) {
        let eq157_e1794_q: f64 = l.f2280;
        (l.f2280, l.f22a1, l.f22a2, eq157_e1794_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node2_local(
            Some(8),
            Some(7),
            7,
            multiplicity * (eq157_e1796_d_n7),
            8,
            multiplicity * (eq157_e1796_d_n8),
        );let eq172_e1881_q: f64 = l.f2276;
        stamper.stamp_current_reactive_node3_local(
            Some(6),
            Some(2),
            2,
            multiplicity * (l.f2277),
            4,
            multiplicity * (l.f2278),
            6,
            multiplicity * (l.f2279),
        );let eq173_e1883_q: f64 = l.f2262;
        stamper.stamp_current_reactive_node3_local(
            Some(6),
            Some(0),
            0,
            multiplicity * (l.f2263),
            4,
            multiplicity * (l.f2264),
            6,
            multiplicity * (l.f2265),
        );let eq174_e1885_q: f64 = l.f2267;
        stamper.stamp_current_reactive_node3_local(
            Some(2),
            Some(0),
            0,
            multiplicity * (l.f2268),
            2,
            multiplicity * (l.f2269),
            4,
            multiplicity * (l.f226a),
        );let eq175_e1887_q: f64 = l.f227b;
        stamper.stamp_current_reactive_node3_local(
            Some(3),
            Some(2),
            2,
            multiplicity * (l.f227c),
            3,
            multiplicity * (l.f227d),
            4,
            multiplicity * (l.f227e),
        );let eq176_e1889_q: f64 = l.f226c;
        stamper.stamp_current_reactive_node3_local(
            Some(3),
            Some(0),
            0,
            multiplicity * (l.f226d),
            3,
            multiplicity * (l.f226e),
            4,
            multiplicity * (l.f226f),
        );let eq177_e1891_q: f64 = l.f2271;
        stamper.stamp_current_reactive_node3_local(
            Some(6),
            Some(3),
            3,
            multiplicity * (l.f2272),
            4,
            multiplicity * (l.f2273),
            6,
            multiplicity * (l.f2274),
        );
        let (eq194_e2167, eq194_e2167_d_n4, eq194_e2167_q,) = {
    if (l.f205b != 0.0) {
        let eq194_e2164: f64 = (p.p321 * (nv4 - 0.0));let eq194_e2165_q: f64 = eq194_e2164;
        (eq194_e2164, p.p321, eq194_e2165_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(4),
            None,
            4,
            multiplicity * (eq194_e2167_d_n4),
        );
    }
}

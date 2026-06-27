#![allow(dead_code, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

use crate::device::veriloga_generated::support::{AdValue as GenericAdValue, ReactiveScratch as GenericReactiveScratch, Scratch as GenericScratch};

type A = GenericAdValue<{ Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;
type Scratch = GenericScratch<{ Instance::VARIABLE_COUNT }, { Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;
type ReactiveScratch = GenericReactiveScratch<{ Instance::VARIABLE_COUNT }, { Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;

const LIMEXP_MAX: f64 = 5.54062238439351e34;
#[path = "stamp_blocks_0.rs"]
mod stamp_blocks_0;
#[path = "stamp_blocks_1.rs"]
mod stamp_blocks_1;
#[path = "stamp_blocks_2.rs"]
mod stamp_blocks_2;

const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

#[inline]
fn eval_ddt<const STATE_COUNT: usize>(
    current: &mut [f64; STATE_COUNT],
    previous: &mut [f64; STATE_COUNT],
    initialized: &mut [bool; STATE_COUNT],
    ddt_active: bool,
    ddt_scale: f64,
    slot: usize,
    value: f64,
) -> f64 {
    debug_assert!(slot < STATE_COUNT, "generated ddt state slot out of range");
    let previous_value = if initialized[slot] { previous[slot] } else { value };
    current[slot] = value;
    if ddt_active {
        (value - previous_value) * ddt_scale
    } else {
        previous[slot] = value;
        initialized[slot] = true;
        0.0
    }
}

#[inline]
fn ddt_jacobian(timestep: f64, derivative: f64) -> f64 {
    if timestep.abs() > Instance::DDT_EPSILON {
        derivative / timestep
    } else {
        0.0
    }
}

#[inline]
fn eval_idt<const STATE_COUNT: usize>(
    current: &mut [f64; STATE_COUNT],
    previous: &mut [f64; STATE_COUNT],
    initialized: &mut [bool; STATE_COUNT],
    ddt_active: bool,
    idt_scale: f64,
    slot: usize,
    value: f64,
    ic: f64,
) -> f64 {
    debug_assert!(slot < STATE_COUNT, "generated idt state slot out of range");
    let previous_value = if initialized[slot] { previous[slot] } else { ic };
    let current_value = if ddt_active {
        previous_value + value * idt_scale
    } else {
        ic
    };
    current[slot] = current_value;
    if !ddt_active {
        previous[slot] = current_value;
        initialized[slot] = true;
    }
    current_value
}

#[inline]
fn idt_jacobian(timestep: f64, derivative: f64) -> f64 {
    if timestep.abs() > Instance::DDT_EPSILON {
        derivative * timestep
    } else {
        0.0
    }
}

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let ctx_temp = ctx.temperature();
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_active = timestep.abs() > Instance::DDT_EPSILON;
        let ddt_scale = if ddt_active { 1.0 / timestep } else { 0.0 };
        let v0: f64 = ctx_temp;
        let v2: f64 = (v0 + self.scalar_v1);
        let v28: f64 = 0.0;
        let v57: f64 = 1.60219e-19;
        let v67: f64 = 2.0;
        let v68: f64 = 1.0;
        let v70: f64 = (v2 / self.scalar_v25);
        let v151: f64 = 0.5;
        let v152: f64 = (v70 - v68);
        let v153: f64 = 1000000.0;
        let v159: f64 = (if self.scalar_v157 { self.scalar_v158 } else { v152 });
        let v160: f64 = 3.021e22;
        let v161: f64 = (v160 * v159);
        let v162: f64 = (v161 * v159);
        let v163: f64 = (if self.scalar_v157 { v162 } else { self.scalar_v123 });
        let v174: bool = (v163 > self.scalar_v173);
        let v175: bool = (self.scalar_v165 && v174);
        let v176: f64 = (if v175 { self.scalar_v173 } else { v163 });
        let v183: bool = (v176 > self.scalar_v182);
        let v184: bool = (self.scalar_v177 && v183);
        let v185: f64 = (if v184 { self.scalar_v182 } else { v176 });
        let v190: f64 = (v57 * v185);
        let v193: f64 = (v190 * self.scalar_v192);
        let v194: f64 = (v193 * v153);
        let v195: f64 = (v194 * self.scalar_v27);
        let v196: f64 = (if (self.scalar_v14 != 0.0) { v195 } else { v28 });
        let v197: f64 = (v194 * self.scalar_v26);
        let v198: f64 = (if self.scalar_v61 { v197 } else { v196 });
        let v199: f64 = 0.8;
        let v200: f64 = (v151 * v198);
        let v201: f64 = (v200 / self.scalar_v189);
        let v202: f64 = (v199 - v201);
        let v203: f64 = (v202 + self.scalar_v132);
        let v206: bool = (v203 > self.scalar_v150);
        let v207: bool = (self.scalar_v205 && v206);
        let v208: f64 = (if v207 { v67 } else { self.scalar_v9 });
        let v209: bool = (v203 < self.scalar_v141);
        let v210: bool = (!v206);
        let v211: bool = (self.scalar_v205 && v210);
        let v212: bool = (v211 && v209);
        let v213: f64 = (if v212 { v28 } else { v208 });
        let v214: bool = (!v209);
        let v215: bool = (v211 && v214);
        let v216: f64 = (if v215 { v68 } else { v213 });
        let v257: f64 = nv5;
        let v258: f64 = nv11;
        let v259: f64 = nv12;
        let v260: f64 = nv10;
        let v261: bool = (v216 == v67);
        let v263: f64 = (if v261 { v28 } else { v28 });
        let v273: f64 = nv1;
        let v274: f64 = (v273 - v260);
        let v275: f64 = (v274 * self.scalar_v232);
        let v276: f64 = (if self.scalar_v272 { v275 } else { v28 });
        let v278: f64 = (v257 - v259);
        let v279: f64 = (v278 * self.scalar_v250);
        let v280: f64 = (if (self.scalar_v13 != 0.0) { v279 } else { v28 });
        let v281: f64 = (v257 - v258);
        let v282: f64 = (v281 * self.scalar_v251);
        let v283: f64 = (if (self.scalar_v13 != 0.0) { v282 } else { v28 });

        stamper.stamp_potential_branch_local(
            Some(0),
            Some(7),
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            self.scalar_v270,
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            Some(8),
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            self.scalar_v270,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(4),
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            v28,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(10),
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            self.scalar_v271,
        );
        let d276_dn1: f64 = self.scalar_v287;
        let d276_dn10: f64 = self.scalar_v288;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(10),
            multiplicity * (v276),
            1,
            multiplicity * (d276_dn1),
            10,
            multiplicity * (d276_dn10),
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(9),
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            self.scalar_v277,
        );
        let d280_dn5: f64 = self.scalar_v290;
        let d280_dn12: f64 = self.scalar_v291;
        stamper.stamp_current_node2_local(
            Some(5),
            Some(12),
            multiplicity * (v280),
            5,
            multiplicity * (d280_dn5),
            12,
            multiplicity * (d280_dn12),
        );
        let d283_dn5: f64 = self.scalar_v293;
        let d283_dn11: f64 = self.scalar_v294;
        stamper.stamp_current_node2_local(
            Some(5),
            Some(11),
            multiplicity * (v283),
            5,
            multiplicity * (d283_dn5),
            11,
            multiplicity * (d283_dn11),
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(12),
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            self.scalar_v284,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(11),
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            self.scalar_v284,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(8),
            7,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            7,
            v263,
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            None,
            8,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            8,
            self.scalar_v285,
        );
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Scratch::new_box()).as_mut(),
        };

        Self::stamp_transient_block_0(ctx, s, p);
        Self::stamp_transient_block_1(s, p, param_given);
        Self::stamp_transient_block_2(s, p);
        Self::stamp_transient_block_3(s, param_given);
        Self::stamp_transient_block_4(s, p, param_given);
        Self::stamp_transient_block_5(s);
        Self::stamp_transient_block_6(ctx, s, p, nodes);
        Self::stamp_transient_block_7(s, p);
        Self::stamp_transient_block_8(ctx, s, p, nodes, param_given);
        Self::stamp_transient_block_9(s, p);
        Self::stamp_transient_block_10(s);
        Self::stamp_transient_block_11(s, p);
        Self::stamp_transient_block_12(s);
        Self::stamp_transient_block_13(s);
        Self::stamp_transient_block_14(s);
        Self::stamp_transient_block_15(s);
        Self::stamp_transient_block_16(s);
        Self::stamp_transient_block_17(s);
        Self::stamp_transient_block_18(s, p);
        Self::stamp_transient_block_19(s);
        Self::stamp_transient_block_20(s, p);

        Self::stamp_transient_equations_block_0(ctx, stamper, s, nodes, multiplicity);
        Self::stamp_transient_equations_block_1(stamper, s, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_2(ctx, stamper, s, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_3(stamper, s, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let s = match &mut self.reactive_scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(ReactiveScratch::new_box()).as_mut(),
        };

        Self::stamp_reactive_block_0(ctx, s, p);
        Self::stamp_reactive_block_1(s, p, param_given);
        Self::stamp_reactive_block_2(s, p);
        Self::stamp_reactive_block_3(s, param_given);
        Self::stamp_reactive_block_4(s, p, param_given);
        Self::stamp_reactive_block_5(s);
        Self::stamp_reactive_block_6(ctx, s, p, nodes);
        Self::stamp_reactive_block_7(s, p);
        Self::stamp_reactive_block_8(ctx, s, p, nodes, param_given);
        Self::stamp_reactive_block_9(s);
        Self::stamp_reactive_block_10(s);
        Self::stamp_reactive_block_11(s, p);
        Self::stamp_reactive_block_12(s);
        Self::stamp_reactive_block_13(s);
        Self::stamp_reactive_block_14(s);
        Self::stamp_reactive_block_15(s);
        Self::stamp_reactive_block_16(s);
        Self::stamp_reactive_block_17(s, p);
        Self::stamp_reactive_block_18(s);
        Self::stamp_reactive_block_19(s, p);

        Self::stamp_reactive_equations_block_0(ctx, stamper, s, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_1(ctx, stamper, s, nodes, branches, multiplicity);
    }
}

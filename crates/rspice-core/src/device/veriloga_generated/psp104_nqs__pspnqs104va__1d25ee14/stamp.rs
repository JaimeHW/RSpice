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
#[path = "stamp_blocks_3.rs"]
mod stamp_blocks_3;
#[path = "stamp_blocks_4.rs"]
mod stamp_blocks_4;
#[path = "stamp_blocks_5.rs"]
mod stamp_blocks_5;
#[path = "stamp_blocks_6.rs"]
mod stamp_blocks_6;
#[path = "stamp_blocks_7.rs"]
mod stamp_blocks_7;

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
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let bi7 = ctx.branch_current(branches[7]);
        let bi9 = ctx.branch_current(branches[9]);
        let bi11 = ctx.branch_current(branches[11]);
        let bi13 = ctx.branch_current(branches[13]);
        let bi15 = ctx.branch_current(branches[15]);
        let bi17 = ctx.branch_current(branches[17]);
        let bi19 = ctx.branch_current(branches[19]);
        let bi21 = ctx.branch_current(branches[21]);
        let bi23 = ctx.branch_current(branches[23]);
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let idt_state_current = self.idt_state_current.as_mut();
        let idt_state_previous = self.idt_state_previous.as_mut();
        let idt_state_initialized = self.idt_state_initialized.as_mut();
        let ddt_active = timestep.abs() > Instance::DDT_EPSILON;
        let ddt_scale = if ddt_active { 1.0 / timestep } else { 0.0 };
        let idt_scale = if ddt_active { timestep } else { 0.0 };
        let v5: f64 = nv6;
        let v6: f64 = nv7;
        let v7: f64 = nv8;
        let v8: f64 = (v5 - v7);
        let v11: f64 = (v6 - v7);
        let v12: f64 = (self.scalar_v10 * v11);
        let v13: f64 = (self.scalar_v10 * v8);

        let d12_dn7: f64 = self.scalar_v10;
        let d12_dn8: f64 = self.scalar_v41;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(8),
            multiplicity * (v12),
            7,
            multiplicity * (d12_dn7),
            8,
            multiplicity * (d12_dn8),
        );
        let d13_dn6: f64 = self.scalar_v10;
        let d13_dn8: f64 = self.scalar_v41;
        stamper.stamp_current_node2_local(
            Some(6),
            Some(8),
            multiplicity * (v13),
            6,
            multiplicity * (d13_dn6),
            8,
            multiplicity * (d13_dn8),
        );
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Scratch::new_box()).as_mut(),
        };

        Self::stamp_transient_block_0(s, p);
        Self::stamp_transient_block_1(ctx, s, p);
        Self::stamp_transient_block_2(s, p, param_given);
        Self::stamp_transient_block_3(s, p, param_given);
        Self::stamp_transient_block_4(s, p, param_given);
        Self::stamp_transient_block_5(s, p, param_given);
        Self::stamp_transient_block_6(s, p);
        Self::stamp_transient_block_7(s, p);
        Self::stamp_transient_block_8(s, p);
        Self::stamp_transient_block_9(s, p);
        Self::stamp_transient_block_10(s, p);
        Self::stamp_transient_block_11(s, p);
        Self::stamp_transient_block_12(s, p);
        Self::stamp_transient_block_13(s, p);
        Self::stamp_transient_block_14(s, p);
        Self::stamp_transient_block_15(s, p);
        Self::stamp_transient_block_16(s, p);
        Self::stamp_transient_block_17(s, p);
        Self::stamp_transient_block_18(s, p);
        Self::stamp_transient_block_19(s, p);
        Self::stamp_transient_block_20(s, p);
        Self::stamp_transient_block_21(s, p);
        Self::stamp_transient_block_22(s, p);
        Self::stamp_transient_block_23(s, p);
        Self::stamp_transient_block_24(s, p);
        Self::stamp_transient_block_25(s, p);
        Self::stamp_transient_block_26(s, p);
        Self::stamp_transient_block_27(s, p);
        Self::stamp_transient_block_28(ctx, s, p, nodes);
        Self::stamp_transient_block_29(s);
        Self::stamp_transient_block_30(s);
        Self::stamp_transient_block_31(s, p);
        Self::stamp_transient_block_32(s, p);
        Self::stamp_transient_block_33(s, p);
        Self::stamp_transient_block_34(s, p);
        Self::stamp_transient_block_35(s, p);
        Self::stamp_transient_block_36(s);
        Self::stamp_transient_block_37(s, p);
        Self::stamp_transient_block_38(s, p);
        Self::stamp_transient_block_39(s, p);
        Self::stamp_transient_block_40(s, p);
        Self::stamp_transient_block_41(s, p);
        Self::stamp_transient_block_42(s, p);
        Self::stamp_transient_block_43(s, p);
        Self::stamp_transient_block_44(s);
        Self::stamp_transient_block_45(s);
        Self::stamp_transient_block_46(ctx, s, nodes);
        Self::stamp_transient_block_47(s);
        Self::stamp_transient_block_48(s);
        Self::stamp_transient_block_49(s);
        Self::stamp_transient_block_50(s);
        Self::stamp_transient_block_51(s);
        Self::stamp_transient_block_52(s);
        Self::stamp_transient_block_53(s);
        Self::stamp_transient_block_54(s);
        Self::stamp_transient_block_55(s);
        Self::stamp_transient_block_56(s);
        Self::stamp_transient_block_57(s);
        Self::stamp_transient_block_58(s);
        Self::stamp_transient_block_59(s, p);

        stamper.stamp_potential_branch_local(
            Some(1),
            Some(5),
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            Some(6),
            1,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(7),
            2,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            3,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(9),
            4,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            Some(9),
            5,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(9),
            6,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(12),
            None,
            7,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(12),
            None,
            8,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            None,
            9,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            None,
            10,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(14),
            None,
            11,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(14),
            None,
            12,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(15),
            None,
            13,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(15),
            None,
            14,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(16),
            None,
            15,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(16),
            None,
            16,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(17),
            None,
            17,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(17),
            None,
            18,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(18),
            None,
            19,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(18),
            None,
            20,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(19),
            None,
            21,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(19),
            None,
            22,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(20),
            None,
            23,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(20),
            None,
            24,
            multiplicity,
        );

        Self::stamp_transient_equations_block_0(stamper, s, p, multiplicity);
        Self::stamp_transient_equations_block_1(stamper, s, p, multiplicity);
        Self::stamp_transient_equations_block_2(stamper, s, p, multiplicity);
        Self::stamp_transient_equations_block_3(stamper, s, p, multiplicity);
        Self::stamp_transient_equations_block_4(stamper, s, p, multiplicity);
        Self::stamp_transient_equations_block_5(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_6(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_7(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_8(ctx, stamper, s, p, nodes, multiplicity);
        let eq38_e1270: f64 = (s.v[4] * bi7);
        let eq38_e1270_d_b7: f64 = s.v[4];
        let eq38_e1272: f64 = (eq38_e1270 * s.v[2]);
        let eq38_e1272_d_b7: f64 = (eq38_e1270_d_b7 * s.v[2]);
        let eq38_value: f64 = eq38_e1272;
        stamper.stamp_potential_branch1_local(
            7,
            eq38_value,
            7,
            eq38_e1272_d_b7,
        );
        Self::stamp_transient_equations_block_9(stamper, s, ddt_active, idt_scale, idt_state_current, idt_state_previous, idt_state_initialized);
        let eq40_e1283: f64 = (s.v[4] * bi9);
        let eq40_e1283_d_b9: f64 = s.v[4];
        let eq40_e1285: f64 = (eq40_e1283 * s.v[2]);
        let eq40_e1285_d_b9: f64 = (eq40_e1283_d_b9 * s.v[2]);
        let eq40_value: f64 = eq40_e1285;
        stamper.stamp_potential_branch1_local(
            9,
            eq40_value,
            9,
            eq40_e1285_d_b9,
        );
        Self::stamp_transient_equations_block_10(stamper, s, ddt_active, idt_scale, idt_state_current, idt_state_previous, idt_state_initialized);
        let eq42_e1296: f64 = (s.v[4] * bi11);
        let eq42_e1296_d_b11: f64 = s.v[4];
        let eq42_e1298: f64 = (eq42_e1296 * s.v[2]);
        let eq42_e1298_d_b11: f64 = (eq42_e1296_d_b11 * s.v[2]);
        let eq42_value: f64 = eq42_e1298;
        stamper.stamp_potential_branch1_local(
            11,
            eq42_value,
            11,
            eq42_e1298_d_b11,
        );
        Self::stamp_transient_equations_block_11(stamper, s, ddt_active, idt_scale, idt_state_current, idt_state_previous, idt_state_initialized);
        let eq44_e1309: f64 = (s.v[4] * bi13);
        let eq44_e1309_d_b13: f64 = s.v[4];
        let eq44_e1311: f64 = (eq44_e1309 * s.v[2]);
        let eq44_e1311_d_b13: f64 = (eq44_e1309_d_b13 * s.v[2]);
        let eq44_value: f64 = eq44_e1311;
        stamper.stamp_potential_branch1_local(
            13,
            eq44_value,
            13,
            eq44_e1311_d_b13,
        );
        Self::stamp_transient_equations_block_12(stamper, s, ddt_active, idt_scale, idt_state_current, idt_state_previous, idt_state_initialized);
        let eq46_e1322: f64 = (s.v[4] * bi15);
        let eq46_e1322_d_b15: f64 = s.v[4];
        let eq46_e1324: f64 = (eq46_e1322 * s.v[2]);
        let eq46_e1324_d_b15: f64 = (eq46_e1322_d_b15 * s.v[2]);
        let eq46_value: f64 = eq46_e1324;
        stamper.stamp_potential_branch1_local(
            15,
            eq46_value,
            15,
            eq46_e1324_d_b15,
        );
        Self::stamp_transient_equations_block_13(stamper, s, ddt_active, idt_scale, idt_state_current, idt_state_previous, idt_state_initialized);
        let eq48_e1335: f64 = (s.v[4] * bi17);
        let eq48_e1335_d_b17: f64 = s.v[4];
        let eq48_e1337: f64 = (eq48_e1335 * s.v[2]);
        let eq48_e1337_d_b17: f64 = (eq48_e1335_d_b17 * s.v[2]);
        let eq48_value: f64 = eq48_e1337;
        stamper.stamp_potential_branch1_local(
            17,
            eq48_value,
            17,
            eq48_e1337_d_b17,
        );
        Self::stamp_transient_equations_block_14(stamper, s, ddt_active, idt_scale, idt_state_current, idt_state_previous, idt_state_initialized);
        let eq50_e1348: f64 = (s.v[4] * bi19);
        let eq50_e1348_d_b19: f64 = s.v[4];
        let eq50_e1350: f64 = (eq50_e1348 * s.v[2]);
        let eq50_e1350_d_b19: f64 = (eq50_e1348_d_b19 * s.v[2]);
        let eq50_value: f64 = eq50_e1350;
        stamper.stamp_potential_branch1_local(
            19,
            eq50_value,
            19,
            eq50_e1350_d_b19,
        );
        Self::stamp_transient_equations_block_15(stamper, s, ddt_active, idt_scale, idt_state_current, idt_state_previous, idt_state_initialized);
        let eq52_e1361: f64 = (s.v[4] * bi21);
        let eq52_e1361_d_b21: f64 = s.v[4];
        let eq52_e1363: f64 = (eq52_e1361 * s.v[2]);
        let eq52_e1363_d_b21: f64 = (eq52_e1361_d_b21 * s.v[2]);
        let eq52_value: f64 = eq52_e1363;
        stamper.stamp_potential_branch1_local(
            21,
            eq52_value,
            21,
            eq52_e1363_d_b21,
        );
        Self::stamp_transient_equations_block_16(stamper, s, ddt_active, idt_scale, idt_state_current, idt_state_previous, idt_state_initialized);
        let eq54_e1374: f64 = (s.v[4] * bi23);
        let eq54_e1374_d_b23: f64 = s.v[4];
        let eq54_e1376: f64 = (eq54_e1374 * s.v[2]);
        let eq54_e1376_d_b23: f64 = (eq54_e1374_d_b23 * s.v[2]);
        let eq54_value: f64 = eq54_e1376;
        stamper.stamp_potential_branch1_local(
            23,
            eq54_value,
            23,
            eq54_e1376_d_b23,
        );
        Self::stamp_transient_equations_block_17(stamper, s, p, multiplicity, ddt_active, ddt_scale, idt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized, idt_state_current, idt_state_previous, idt_state_initialized);
        Self::stamp_transient_equations_block_18(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_19(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_20(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_21(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        let eq64_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(4),
            None,
            multiplicity * (eq64_value),
        );
        Self::stamp_transient_equations_block_22(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_23(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        let eq69_e1488: f64 = (s.v[19] * p.p32);
        let eq69_e1488_d_n0: f64 = (s.dn[19][0] * p.p32);
        let eq69_e1488_d_n1: f64 = (s.dn[19][1] * p.p32);
        let eq69_e1488_d_n2: f64 = (s.dn[19][2] * p.p32);
        let eq69_e1488_d_n3: f64 = (s.dn[19][3] * p.p32);
        let eq69_e1488_d_n4: f64 = (s.dn[19][4] * p.p32);
        let eq69_e1488_d_n5: f64 = (s.dn[19][5] * p.p32);
        let eq69_e1488_d_n6: f64 = (s.dn[19][6] * p.p32);
        let eq69_e1488_d_n7: f64 = (s.dn[19][7] * p.p32);
        let eq69_e1488_d_n8: f64 = (s.dn[19][8] * p.p32);
        let eq69_e1488_d_n9: f64 = (s.dn[19][9] * p.p32);
        let eq69_e1488_d_n10: f64 = (s.dn[19][10] * p.p32);
        let eq69_e1488_d_n11: f64 = (s.dn[19][11] * p.p32);
        let eq69_e1488_d_n12: f64 = (s.dn[19][12] * p.p32);
        let eq69_e1488_d_n13: f64 = (s.dn[19][13] * p.p32);
        let eq69_e1488_d_n14: f64 = (s.dn[19][14] * p.p32);
        let eq69_e1488_d_n15: f64 = (s.dn[19][15] * p.p32);
        let eq69_e1488_d_n16: f64 = (s.dn[19][16] * p.p32);
        let eq69_e1488_d_n17: f64 = (s.dn[19][17] * p.p32);
        let eq69_e1488_d_n18: f64 = (s.dn[19][18] * p.p32);
        let eq69_e1488_d_n19: f64 = (s.dn[19][19] * p.p32);
        let eq69_e1488_d_n20: f64 = (s.dn[19][20] * p.p32);
        let eq69_e1488_d_b0: f64 = (s.db[19][0] * p.p32);
        let eq69_e1488_d_b1: f64 = (s.db[19][1] * p.p32);
        let eq69_e1488_d_b2: f64 = (s.db[19][2] * p.p32);
        let eq69_e1488_d_b3: f64 = (s.db[19][3] * p.p32);
        let eq69_e1488_d_b4: f64 = (s.db[19][4] * p.p32);
        let eq69_e1488_d_b5: f64 = (s.db[19][5] * p.p32);
        let eq69_e1488_d_b6: f64 = (s.db[19][6] * p.p32);
        let eq69_e1488_d_b7: f64 = (s.db[19][7] * p.p32);
        let eq69_e1488_d_b8: f64 = (s.db[19][8] * p.p32);
        let eq69_e1488_d_b9: f64 = (s.db[19][9] * p.p32);
        let eq69_e1488_d_b10: f64 = (s.db[19][10] * p.p32);
        let eq69_e1488_d_b11: f64 = (s.db[19][11] * p.p32);
        let eq69_e1488_d_b12: f64 = (s.db[19][12] * p.p32);
        let eq69_e1488_d_b13: f64 = (s.db[19][13] * p.p32);
        let eq69_e1488_d_b14: f64 = (s.db[19][14] * p.p32);
        let eq69_e1488_d_b15: f64 = (s.db[19][15] * p.p32);
        let eq69_e1488_d_b16: f64 = (s.db[19][16] * p.p32);
        let eq69_e1488_d_b17: f64 = (s.db[19][17] * p.p32);
        let eq69_e1488_d_b18: f64 = (s.db[19][18] * p.p32);
        let eq69_e1488_d_b19: f64 = (s.db[19][19] * p.p32);
        let eq69_e1488_d_b20: f64 = (s.db[19][20] * p.p32);
        let eq69_e1488_d_b21: f64 = (s.db[19][21] * p.p32);
        let eq69_e1488_d_b22: f64 = (s.db[19][22] * p.p32);
        let eq69_e1488_d_b23: f64 = (s.db[19][23] * p.p32);
        let eq69_e1488_d_b24: f64 = (s.db[19][24] * p.p32);
        let eq69_e1489: f64 = (eq69_e1488).sqrt();
        let eq69_e1489_d_n0: f64 = (eq69_e1488_d_n0 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n1: f64 = (eq69_e1488_d_n1 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n2: f64 = (eq69_e1488_d_n2 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n3: f64 = (eq69_e1488_d_n3 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n4: f64 = (eq69_e1488_d_n4 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n5: f64 = (eq69_e1488_d_n5 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n6: f64 = (eq69_e1488_d_n6 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n7: f64 = (eq69_e1488_d_n7 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n8: f64 = (eq69_e1488_d_n8 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n9: f64 = (eq69_e1488_d_n9 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n10: f64 = (eq69_e1488_d_n10 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n11: f64 = (eq69_e1488_d_n11 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n12: f64 = (eq69_e1488_d_n12 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n13: f64 = (eq69_e1488_d_n13 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n14: f64 = (eq69_e1488_d_n14 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n15: f64 = (eq69_e1488_d_n15 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n16: f64 = (eq69_e1488_d_n16 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n17: f64 = (eq69_e1488_d_n17 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n18: f64 = (eq69_e1488_d_n18 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n19: f64 = (eq69_e1488_d_n19 / (2.0 * eq69_e1489));
        let eq69_e1489_d_n20: f64 = (eq69_e1488_d_n20 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b0: f64 = (eq69_e1488_d_b0 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b1: f64 = (eq69_e1488_d_b1 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b2: f64 = (eq69_e1488_d_b2 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b3: f64 = (eq69_e1488_d_b3 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b4: f64 = (eq69_e1488_d_b4 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b5: f64 = (eq69_e1488_d_b5 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b6: f64 = (eq69_e1488_d_b6 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b7: f64 = (eq69_e1488_d_b7 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b8: f64 = (eq69_e1488_d_b8 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b9: f64 = (eq69_e1488_d_b9 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b10: f64 = (eq69_e1488_d_b10 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b11: f64 = (eq69_e1488_d_b11 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b12: f64 = (eq69_e1488_d_b12 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b13: f64 = (eq69_e1488_d_b13 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b14: f64 = (eq69_e1488_d_b14 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b15: f64 = (eq69_e1488_d_b15 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b16: f64 = (eq69_e1488_d_b16 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b17: f64 = (eq69_e1488_d_b17 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b18: f64 = (eq69_e1488_d_b18 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b19: f64 = (eq69_e1488_d_b19 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b20: f64 = (eq69_e1488_d_b20 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b21: f64 = (eq69_e1488_d_b21 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b22: f64 = (eq69_e1488_d_b22 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b23: f64 = (eq69_e1488_d_b23 / (2.0 * eq69_e1489));
        let eq69_e1489_d_b24: f64 = (eq69_e1488_d_b24 / (2.0 * eq69_e1489));
        let eq69_e1490: f64 = (s.v[831] * eq69_e1489);
        let eq69_e1490_d_n0: f64 = ((s.dn[831][0] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n0));
        let eq69_e1490_d_n1: f64 = ((s.dn[831][1] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n1));
        let eq69_e1490_d_n2: f64 = ((s.dn[831][2] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n2));
        let eq69_e1490_d_n3: f64 = ((s.dn[831][3] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n3));
        let eq69_e1490_d_n4: f64 = ((s.dn[831][4] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n4));
        let eq69_e1490_d_n5: f64 = ((s.dn[831][5] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n5));
        let eq69_e1490_d_n6: f64 = ((s.dn[831][6] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n6));
        let eq69_e1490_d_n7: f64 = ((s.dn[831][7] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n7));
        let eq69_e1490_d_n8: f64 = ((s.dn[831][8] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n8));
        let eq69_e1490_d_n9: f64 = ((s.dn[831][9] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n9));
        let eq69_e1490_d_n10: f64 = ((s.dn[831][10] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n10));
        let eq69_e1490_d_n11: f64 = ((s.dn[831][11] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n11));
        let eq69_e1490_d_n12: f64 = ((s.dn[831][12] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n12));
        let eq69_e1490_d_n13: f64 = ((s.dn[831][13] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n13));
        let eq69_e1490_d_n14: f64 = ((s.dn[831][14] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n14));
        let eq69_e1490_d_n15: f64 = ((s.dn[831][15] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n15));
        let eq69_e1490_d_n16: f64 = ((s.dn[831][16] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n16));
        let eq69_e1490_d_n17: f64 = ((s.dn[831][17] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n17));
        let eq69_e1490_d_n18: f64 = ((s.dn[831][18] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n18));
        let eq69_e1490_d_n19: f64 = ((s.dn[831][19] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n19));
        let eq69_e1490_d_n20: f64 = ((s.dn[831][20] * eq69_e1489) + (s.v[831] * eq69_e1489_d_n20));
        let eq69_e1490_d_b0: f64 = ((s.db[831][0] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b0));
        let eq69_e1490_d_b1: f64 = ((s.db[831][1] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b1));
        let eq69_e1490_d_b2: f64 = ((s.db[831][2] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b2));
        let eq69_e1490_d_b3: f64 = ((s.db[831][3] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b3));
        let eq69_e1490_d_b4: f64 = ((s.db[831][4] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b4));
        let eq69_e1490_d_b5: f64 = ((s.db[831][5] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b5));
        let eq69_e1490_d_b6: f64 = ((s.db[831][6] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b6));
        let eq69_e1490_d_b7: f64 = ((s.db[831][7] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b7));
        let eq69_e1490_d_b8: f64 = ((s.db[831][8] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b8));
        let eq69_e1490_d_b9: f64 = ((s.db[831][9] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b9));
        let eq69_e1490_d_b10: f64 = ((s.db[831][10] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b10));
        let eq69_e1490_d_b11: f64 = ((s.db[831][11] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b11));
        let eq69_e1490_d_b12: f64 = ((s.db[831][12] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b12));
        let eq69_e1490_d_b13: f64 = ((s.db[831][13] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b13));
        let eq69_e1490_d_b14: f64 = ((s.db[831][14] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b14));
        let eq69_e1490_d_b15: f64 = ((s.db[831][15] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b15));
        let eq69_e1490_d_b16: f64 = ((s.db[831][16] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b16));
        let eq69_e1490_d_b17: f64 = ((s.db[831][17] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b17));
        let eq69_e1490_d_b18: f64 = ((s.db[831][18] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b18));
        let eq69_e1490_d_b19: f64 = ((s.db[831][19] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b19));
        let eq69_e1490_d_b20: f64 = ((s.db[831][20] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b20));
        let eq69_e1490_d_b21: f64 = ((s.db[831][21] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b21));
        let eq69_e1490_d_b22: f64 = ((s.db[831][22] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b22));
        let eq69_e1490_d_b23: f64 = ((s.db[831][23] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b23));
        let eq69_e1490_d_b24: f64 = ((s.db[831][24] * eq69_e1489) + (s.v[831] * eq69_e1489_d_b24));
        let eq69_e1492: f64 = (eq69_e1490 * s.v[861]);
        let eq69_e1492_d_n0: f64 = ((eq69_e1490_d_n0 * s.v[861]) + (eq69_e1490 * s.dn[861][0]));
        let eq69_e1492_d_n1: f64 = ((eq69_e1490_d_n1 * s.v[861]) + (eq69_e1490 * s.dn[861][1]));
        let eq69_e1492_d_n2: f64 = ((eq69_e1490_d_n2 * s.v[861]) + (eq69_e1490 * s.dn[861][2]));
        let eq69_e1492_d_n3: f64 = ((eq69_e1490_d_n3 * s.v[861]) + (eq69_e1490 * s.dn[861][3]));
        let eq69_e1492_d_n4: f64 = ((eq69_e1490_d_n4 * s.v[861]) + (eq69_e1490 * s.dn[861][4]));
        let eq69_e1492_d_n5: f64 = ((eq69_e1490_d_n5 * s.v[861]) + (eq69_e1490 * s.dn[861][5]));
        let eq69_e1492_d_n6: f64 = ((eq69_e1490_d_n6 * s.v[861]) + (eq69_e1490 * s.dn[861][6]));
        let eq69_e1492_d_n7: f64 = ((eq69_e1490_d_n7 * s.v[861]) + (eq69_e1490 * s.dn[861][7]));
        let eq69_e1492_d_n8: f64 = ((eq69_e1490_d_n8 * s.v[861]) + (eq69_e1490 * s.dn[861][8]));
        let eq69_e1492_d_n9: f64 = ((eq69_e1490_d_n9 * s.v[861]) + (eq69_e1490 * s.dn[861][9]));
        let eq69_e1492_d_n10: f64 = ((eq69_e1490_d_n10 * s.v[861]) + (eq69_e1490 * s.dn[861][10]));
        let eq69_e1492_d_n11: f64 = ((eq69_e1490_d_n11 * s.v[861]) + (eq69_e1490 * s.dn[861][11]));
        let eq69_e1492_d_n12: f64 = ((eq69_e1490_d_n12 * s.v[861]) + (eq69_e1490 * s.dn[861][12]));
        let eq69_e1492_d_n13: f64 = ((eq69_e1490_d_n13 * s.v[861]) + (eq69_e1490 * s.dn[861][13]));
        let eq69_e1492_d_n14: f64 = ((eq69_e1490_d_n14 * s.v[861]) + (eq69_e1490 * s.dn[861][14]));
        let eq69_e1492_d_n15: f64 = ((eq69_e1490_d_n15 * s.v[861]) + (eq69_e1490 * s.dn[861][15]));
        let eq69_e1492_d_n16: f64 = ((eq69_e1490_d_n16 * s.v[861]) + (eq69_e1490 * s.dn[861][16]));
        let eq69_e1492_d_n17: f64 = ((eq69_e1490_d_n17 * s.v[861]) + (eq69_e1490 * s.dn[861][17]));
        let eq69_e1492_d_n18: f64 = ((eq69_e1490_d_n18 * s.v[861]) + (eq69_e1490 * s.dn[861][18]));
        let eq69_e1492_d_n19: f64 = ((eq69_e1490_d_n19 * s.v[861]) + (eq69_e1490 * s.dn[861][19]));
        let eq69_e1492_d_n20: f64 = ((eq69_e1490_d_n20 * s.v[861]) + (eq69_e1490 * s.dn[861][20]));
        let eq69_e1492_d_b0: f64 = ((eq69_e1490_d_b0 * s.v[861]) + (eq69_e1490 * s.db[861][0]));
        let eq69_e1492_d_b1: f64 = ((eq69_e1490_d_b1 * s.v[861]) + (eq69_e1490 * s.db[861][1]));
        let eq69_e1492_d_b2: f64 = ((eq69_e1490_d_b2 * s.v[861]) + (eq69_e1490 * s.db[861][2]));
        let eq69_e1492_d_b3: f64 = ((eq69_e1490_d_b3 * s.v[861]) + (eq69_e1490 * s.db[861][3]));
        let eq69_e1492_d_b4: f64 = ((eq69_e1490_d_b4 * s.v[861]) + (eq69_e1490 * s.db[861][4]));
        let eq69_e1492_d_b5: f64 = ((eq69_e1490_d_b5 * s.v[861]) + (eq69_e1490 * s.db[861][5]));
        let eq69_e1492_d_b6: f64 = ((eq69_e1490_d_b6 * s.v[861]) + (eq69_e1490 * s.db[861][6]));
        let eq69_e1492_d_b7: f64 = ((eq69_e1490_d_b7 * s.v[861]) + (eq69_e1490 * s.db[861][7]));
        let eq69_e1492_d_b8: f64 = ((eq69_e1490_d_b8 * s.v[861]) + (eq69_e1490 * s.db[861][8]));
        let eq69_e1492_d_b9: f64 = ((eq69_e1490_d_b9 * s.v[861]) + (eq69_e1490 * s.db[861][9]));
        let eq69_e1492_d_b10: f64 = ((eq69_e1490_d_b10 * s.v[861]) + (eq69_e1490 * s.db[861][10]));
        let eq69_e1492_d_b11: f64 = ((eq69_e1490_d_b11 * s.v[861]) + (eq69_e1490 * s.db[861][11]));
        let eq69_e1492_d_b12: f64 = ((eq69_e1490_d_b12 * s.v[861]) + (eq69_e1490 * s.db[861][12]));
        let eq69_e1492_d_b13: f64 = ((eq69_e1490_d_b13 * s.v[861]) + (eq69_e1490 * s.db[861][13]));
        let eq69_e1492_d_b14: f64 = ((eq69_e1490_d_b14 * s.v[861]) + (eq69_e1490 * s.db[861][14]));
        let eq69_e1492_d_b15: f64 = ((eq69_e1490_d_b15 * s.v[861]) + (eq69_e1490 * s.db[861][15]));
        let eq69_e1492_d_b16: f64 = ((eq69_e1490_d_b16 * s.v[861]) + (eq69_e1490 * s.db[861][16]));
        let eq69_e1492_d_b17: f64 = ((eq69_e1490_d_b17 * s.v[861]) + (eq69_e1490 * s.db[861][17]));
        let eq69_e1492_d_b18: f64 = ((eq69_e1490_d_b18 * s.v[861]) + (eq69_e1490 * s.db[861][18]));
        let eq69_e1492_d_b19: f64 = ((eq69_e1490_d_b19 * s.v[861]) + (eq69_e1490 * s.db[861][19]));
        let eq69_e1492_d_b20: f64 = ((eq69_e1490_d_b20 * s.v[861]) + (eq69_e1490 * s.db[861][20]));
        let eq69_e1492_d_b21: f64 = ((eq69_e1490_d_b21 * s.v[861]) + (eq69_e1490 * s.db[861][21]));
        let eq69_e1492_d_b22: f64 = ((eq69_e1490_d_b22 * s.v[861]) + (eq69_e1490 * s.db[861][22]));
        let eq69_e1492_d_b23: f64 = ((eq69_e1490_d_b23 * s.v[861]) + (eq69_e1490 * s.db[861][23]));
        let eq69_e1492_d_b24: f64 = ((eq69_e1490_d_b24 * s.v[861]) + (eq69_e1490 * s.db[861][24]));
        let eq69_e1494: f64 = (eq69_e1492 * eq64_value);
        let eq69_e1494_d_n0: f64 = (eq69_e1492_d_n0 * eq64_value);
        let eq69_e1494_d_n1: f64 = (eq69_e1492_d_n1 * eq64_value);
        let eq69_e1494_d_n2: f64 = (eq69_e1492_d_n2 * eq64_value);
        let eq69_e1494_d_n3: f64 = (eq69_e1492_d_n3 * eq64_value);
        let eq69_e1494_d_n4: f64 = (eq69_e1492_d_n4 * eq64_value);
        let eq69_e1494_d_n5: f64 = (eq69_e1492_d_n5 * eq64_value);
        let eq69_e1494_d_n6: f64 = (eq69_e1492_d_n6 * eq64_value);
        let eq69_e1494_d_n7: f64 = (eq69_e1492_d_n7 * eq64_value);
        let eq69_e1494_d_n8: f64 = (eq69_e1492_d_n8 * eq64_value);
        let eq69_e1494_d_n9: f64 = (eq69_e1492_d_n9 * eq64_value);
        let eq69_e1494_d_n10: f64 = (eq69_e1492_d_n10 * eq64_value);
        let eq69_e1494_d_n11: f64 = (eq69_e1492_d_n11 * eq64_value);
        let eq69_e1494_d_n12: f64 = (eq69_e1492_d_n12 * eq64_value);
        let eq69_e1494_d_n13: f64 = (eq69_e1492_d_n13 * eq64_value);
        let eq69_e1494_d_n14: f64 = (eq69_e1492_d_n14 * eq64_value);
        let eq69_e1494_d_n15: f64 = (eq69_e1492_d_n15 * eq64_value);
        let eq69_e1494_d_n16: f64 = (eq69_e1492_d_n16 * eq64_value);
        let eq69_e1494_d_n17: f64 = (eq69_e1492_d_n17 * eq64_value);
        let eq69_e1494_d_n18: f64 = (eq69_e1492_d_n18 * eq64_value);
        let eq69_e1494_d_n19: f64 = (eq69_e1492_d_n19 * eq64_value);
        let eq69_e1494_d_n20: f64 = (eq69_e1492_d_n20 * eq64_value);
        let eq69_e1494_d_b0: f64 = (eq69_e1492_d_b0 * eq64_value);
        let eq69_e1494_d_b1: f64 = (eq69_e1492_d_b1 * eq64_value);
        let eq69_e1494_d_b2: f64 = (eq69_e1492_d_b2 * eq64_value);
        let eq69_e1494_d_b3: f64 = (eq69_e1492_d_b3 * eq64_value);
        let eq69_e1494_d_b4: f64 = (eq69_e1492_d_b4 * eq64_value);
        let eq69_e1494_d_b5: f64 = (eq69_e1492_d_b5 * eq64_value);
        let eq69_e1494_d_b6: f64 = (eq69_e1492_d_b6 * eq64_value);
        let eq69_e1494_d_b7: f64 = (eq69_e1492_d_b7 * eq64_value);
        let eq69_e1494_d_b8: f64 = (eq69_e1492_d_b8 * eq64_value);
        let eq69_e1494_d_b9: f64 = (eq69_e1492_d_b9 * eq64_value);
        let eq69_e1494_d_b10: f64 = (eq69_e1492_d_b10 * eq64_value);
        let eq69_e1494_d_b11: f64 = (eq69_e1492_d_b11 * eq64_value);
        let eq69_e1494_d_b12: f64 = (eq69_e1492_d_b12 * eq64_value);
        let eq69_e1494_d_b13: f64 = (eq69_e1492_d_b13 * eq64_value);
        let eq69_e1494_d_b14: f64 = (eq69_e1492_d_b14 * eq64_value);
        let eq69_e1494_d_b15: f64 = (eq69_e1492_d_b15 * eq64_value);
        let eq69_e1494_d_b16: f64 = (eq69_e1492_d_b16 * eq64_value);
        let eq69_e1494_d_b17: f64 = (eq69_e1492_d_b17 * eq64_value);
        let eq69_e1494_d_b18: f64 = (eq69_e1492_d_b18 * eq64_value);
        let eq69_e1494_d_b19: f64 = (eq69_e1492_d_b19 * eq64_value);
        let eq69_e1494_d_b20: f64 = (eq69_e1492_d_b20 * eq64_value);
        let eq69_e1494_d_b21: f64 = (eq69_e1492_d_b21 * eq64_value);
        let eq69_e1494_d_b22: f64 = (eq69_e1492_d_b22 * eq64_value);
        let eq69_e1494_d_b23: f64 = (eq69_e1492_d_b23 * eq64_value);
        let eq69_e1494_d_b24: f64 = (eq69_e1492_d_b24 * eq64_value);
        let eq69_value: f64 = eq69_e1494;
        let eq69_node_derivatives: [f64; 21] = [eq69_e1494_d_n0, eq69_e1494_d_n1, eq69_e1494_d_n2, eq69_e1494_d_n3, eq69_e1494_d_n4, eq69_e1494_d_n5, eq69_e1494_d_n6, eq69_e1494_d_n7, eq69_e1494_d_n8, eq69_e1494_d_n9, eq69_e1494_d_n10, eq69_e1494_d_n11, eq69_e1494_d_n12, eq69_e1494_d_n13, eq69_e1494_d_n14, eq69_e1494_d_n15, eq69_e1494_d_n16, eq69_e1494_d_n17, eq69_e1494_d_n18, eq69_e1494_d_n19, eq69_e1494_d_n20];
        let eq69_branch_derivatives: [f64; 25] = [eq69_e1494_d_b0, eq69_e1494_d_b1, eq69_e1494_d_b2, eq69_e1494_d_b3, eq69_e1494_d_b4, eq69_e1494_d_b5, eq69_e1494_d_b6, eq69_e1494_d_b7, eq69_e1494_d_b8, eq69_e1494_d_b9, eq69_e1494_d_b10, eq69_e1494_d_b11, eq69_e1494_d_b12, eq69_e1494_d_b13, eq69_e1494_d_b14, eq69_e1494_d_b15, eq69_e1494_d_b16, eq69_e1494_d_b17, eq69_e1494_d_b18, eq69_e1494_d_b19, eq69_e1494_d_b20, eq69_e1494_d_b21, eq69_e1494_d_b22, eq69_e1494_d_b23, eq69_e1494_d_b24];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq69_value),
            &eq69_node_derivatives,
            &eq69_branch_derivatives,
            multiplicity,
        );
        Self::stamp_transient_equations_block_24(stamper, multiplicity);
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

        Self::stamp_reactive_block_0(s, p);
        Self::stamp_reactive_block_1(ctx, s, p);
        Self::stamp_reactive_block_2(s, p, param_given);
        Self::stamp_reactive_block_3(s, p, param_given);
        Self::stamp_reactive_block_4(s, p, param_given);
        Self::stamp_reactive_block_5(s, p, param_given);
        Self::stamp_reactive_block_6(s, p);
        Self::stamp_reactive_block_7(s, p);
        Self::stamp_reactive_block_8(s, p);
        Self::stamp_reactive_block_9(ctx, s, p, nodes);
        Self::stamp_reactive_block_10(s);
        Self::stamp_reactive_block_11(s);
        Self::stamp_reactive_block_12(s, p);
        Self::stamp_reactive_block_13(s, p);
        Self::stamp_reactive_block_14(s, p);
        Self::stamp_reactive_block_15(s);
        Self::stamp_reactive_block_16(s, p);
        Self::stamp_reactive_block_17(s);
        Self::stamp_reactive_block_18(s, p);
        Self::stamp_reactive_block_19(s, p);
        Self::stamp_reactive_block_20(s, p);
        Self::stamp_reactive_block_21(s, p);
        Self::stamp_reactive_block_22(s, p);
        Self::stamp_reactive_block_23(s);
        Self::stamp_reactive_block_24(ctx, s, nodes);
        Self::stamp_reactive_block_25(s);
        Self::stamp_reactive_block_26(s);
        Self::stamp_reactive_block_27(s);
        Self::stamp_reactive_block_28(s);
        Self::stamp_reactive_block_29(s);
        Self::stamp_reactive_block_30(s);
        Self::stamp_reactive_block_31(s);
        Self::stamp_reactive_block_32(s);
        Self::stamp_reactive_block_33(s);
        Self::stamp_reactive_block_34(s);
        Self::stamp_reactive_block_35(s);
        Self::stamp_reactive_block_36(s);
        Self::stamp_reactive_block_37(s, p);

        Self::stamp_reactive_equations_block_0(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_1(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_2(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_3(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_4(ctx, stamper, s, p, nodes, branches, multiplicity);
    }
}

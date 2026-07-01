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
#[path = "stamp_blocks_8.rs"]
mod stamp_blocks_8;
#[path = "stamp_blocks_9.rs"]
mod stamp_blocks_9;
#[path = "stamp_blocks_10.rs"]
mod stamp_blocks_10;
#[path = "stamp_blocks_11.rs"]
mod stamp_blocks_11;
#[path = "stamp_blocks_12.rs"]
mod stamp_blocks_12;
#[path = "stamp_blocks_13.rs"]
mod stamp_blocks_13;

const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

#[inline]
fn eval_ddt<const STATE_COUNT: usize>(
    current: &mut [f64; STATE_COUNT],
    previous: &mut [f64; STATE_COUNT],
    older: &mut [f64; STATE_COUNT],
    initialized: &mut [bool; STATE_COUNT],
    derivative_current: &mut [f64; STATE_COUNT],
    derivative_previous: &mut [f64; STATE_COUNT],
    ddt_active: bool,
    ddt_scale: f64,
    ddt_previous_value_scale: f64,
    ddt_older_value_scale: f64,
    ddt_previous_derivative_scale: f64,
    slot: usize,
    value: f64,
) -> f64 {
    debug_assert!(slot < STATE_COUNT, "generated ddt state slot out of range");
    let previous_value = if initialized[slot] { previous[slot] } else { value };
    let older_value = if initialized[slot] { older[slot] } else { value };
    current[slot] = value;
    if ddt_active {
        let result = value * ddt_scale
            - previous_value * ddt_previous_value_scale
            - older_value * ddt_older_value_scale
            - derivative_previous[slot] * ddt_previous_derivative_scale;
        derivative_current[slot] = result;
        result
    } else {
        current[slot] = value;
        previous[slot] = value;
        older[slot] = value;
        derivative_current[slot] = 0.0;
        derivative_previous[slot] = 0.0;
        initialized[slot] = true;
        0.0
    }
}

#[inline]
fn ddt_jacobian(ddt_active: bool, ddt_scale: f64, derivative: f64) -> f64 {
    if ddt_active {
        derivative * ddt_scale
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
        let scalar_temperature_static_temperature = (ctx).temperature();
        let scalar_temperature_static_thermal_voltage = (ctx).thermal_voltage();
        self.ensure_temperature_static(scalar_temperature_static_temperature, scalar_temperature_static_thermal_voltage);
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_older = self.ddt_state_older.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_derivative_current = self.ddt_derivative_current.as_mut();
        let ddt_derivative_previous = self.ddt_derivative_previous.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_scale = self.ddt_coefficients.derivative_scale;
        let ddt_previous_value_scale = self.ddt_coefficients.previous_value_scale;
        let ddt_older_value_scale = self.ddt_coefficients.older_value_scale;
        let ddt_previous_derivative_scale = self.ddt_coefficients.previous_derivative_scale;
        let v0: f64 = 1.0;
        let v1: f64 = 0.0;
        let v35: f64 = 10000.0;
        let v329: f64 = nv6;
        let v330: f64 = nv8;
        let v331: f64 = nv10;
        let v332: f64 = nv9;
        let v337: f64 = nv4;
        let v385: f64 = nv14;
        let v386: f64 = nv1;
        let v387: f64 = (v386 - v329);
        let v388: f64 = (self.scalar_v251 * v387);
        let v389: f64 = (if self.scalar_v375 { v388 } else { v1 });
        let v392: f64 = (v331 - v330);
        let v393: f64 = (self.scalar_v284 * v392);
        let v394: f64 = (if (self.scalar_v252 != 0.0) { v393 } else { v1 });
        let v395: f64 = (v332 - v330);
        let v396: f64 = (self.scalar_v285 * v395);
        let v397: f64 = (if (self.scalar_v252 != 0.0) { v396 } else { v1 });
        let v398: f64 = nv3;
        let v399: f64 = (v398 - v330);
        let v400: f64 = (self.scalar_v283 * v399);
        let v401: f64 = (if (self.scalar_v252 != 0.0) { v400 } else { v1 });
        let v404: f64 = (v337 * self.scalar_v373);
        let v405: f64 = (if self.scalar_v334 { v404 } else { v1 });
        let v406: f64 = (v35 * v337);
        let v407: f64 = (if self.scalar_v372 { v406 } else { v1 });
        let v408: f64 = (v337 * self.scalar_v376);

        stamper.stamp_potential_branch_local(
            Some(15),
            None,
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            self.scalar_v378,
        );
        stamper.stamp_potential_branch_local(
            Some(16),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            self.scalar_v378,
        );
        stamper.stamp_potential_branch_local(
            Some(17),
            None,
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            self.scalar_v380,
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(5),
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            self.scalar_v382,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(2),
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            self.scalar_v384,
        );
        let d385_dn14: f64 = v0;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (v385),
            14,
            multiplicity * (d385_dn14),
        );
        let d389_dn1: f64 = self.scalar_v412;
        let d389_dn6: f64 = self.scalar_v413;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(6),
            multiplicity * (v389),
            1,
            multiplicity * (d389_dn1),
            6,
            multiplicity * (d389_dn6),
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(6),
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            self.scalar_v391,
        );
        let d394_dn8: f64 = self.scalar_v415;
        let d394_dn10: f64 = self.scalar_v416;
        stamper.stamp_current_node2_local(
            Some(10),
            Some(8),
            multiplicity * (v394),
            8,
            multiplicity * (d394_dn8),
            10,
            multiplicity * (d394_dn10),
        );
        let d397_dn8: f64 = self.scalar_v418;
        let d397_dn9: f64 = self.scalar_v419;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(8),
            multiplicity * (v397),
            8,
            multiplicity * (d397_dn8),
            9,
            multiplicity * (d397_dn9),
        );
        let d401_dn3: f64 = self.scalar_v421;
        let d401_dn8: f64 = self.scalar_v422;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(8),
            multiplicity * (v401),
            3,
            multiplicity * (d401_dn3),
            8,
            multiplicity * (d401_dn8),
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(8),
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            self.scalar_v403,
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(8),
            7,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            7,
            self.scalar_v403,
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(8),
            8,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            8,
            self.scalar_v403,
        );
        let d405_dn4: f64 = self.scalar_v423;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (v405),
            4,
            multiplicity * (d405_dn4),
        );
        let d407_dn4: f64 = self.scalar_v424;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (v407),
            4,
            multiplicity * (d407_dn4),
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            None,
            9,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            9,
            self.scalar_v409,
        );
        stamper.stamp_potential_branch_local(
            Some(12),
            None,
            10,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            10,
            self.scalar_v409,
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            None,
            11,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            11,
            self.scalar_v410,
        );
        let d408_dn4: f64 = self.scalar_v376;
        let v408_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, v408);
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (v408_ddt),
            4,
            multiplicity * (((d408_dn4) * ddt_scale)),
        );
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Scratch::new_box()).as_mut(),
        };

        let mut var_c_eox: f64 = 0.0;
        let mut var_tox0: f64 = 0.0;
        let mut var_cox0: f64 = 0.0;
        let mut var_cgdoe: f64 = 0.0;
        let mut var_cgsoe: f64 = 0.0;
        let mut var_qgso: f64 = 0.0;
        let mut var_qgso_dn0: f64 = 0.0;
        let mut var_qgso_dn1: f64 = 0.0;
        let mut var_qgso_dn2: f64 = 0.0;
        let mut var_qgso_dn3: f64 = 0.0;
        let mut var_qgso_dn4: f64 = 0.0;
        let mut var_qgso_dn5: f64 = 0.0;
        let mut var_qgso_dn6: f64 = 0.0;
        let mut var_qgso_dn7: f64 = 0.0;
        let mut var_qgso_dn8: f64 = 0.0;
        let mut var_qgso_dn9: f64 = 0.0;
        let mut var_qgso_dn10: f64 = 0.0;
        let mut var_qgso_dn11: f64 = 0.0;
        let mut var_qgso_dn12: f64 = 0.0;
        let mut var_qgso_dn13: f64 = 0.0;
        let mut var_qgso_dn14: f64 = 0.0;
        let mut var_qgso_dn15: f64 = 0.0;
        let mut var_qgso_dn16: f64 = 0.0;
        let mut var_qgso_dn17: f64 = 0.0;
        let mut var_qgso_db0: f64 = 0.0;
        let mut var_qgso_db1: f64 = 0.0;
        let mut var_qgso_db2: f64 = 0.0;
        let mut var_qgso_db3: f64 = 0.0;
        let mut var_qgso_db4: f64 = 0.0;
        let mut var_qgso_db5: f64 = 0.0;
        let mut var_qgso_db6: f64 = 0.0;
        let mut var_qgso_db7: f64 = 0.0;
        let mut var_qgso_db8: f64 = 0.0;
        let mut var_qgso_db9: f64 = 0.0;
        let mut var_qgso_db10: f64 = 0.0;
        let mut var_qgso_db11: f64 = 0.0;
        let mut var_qgdo: f64 = 0.0;
        let mut var_qgdo_dn0: f64 = 0.0;
        let mut var_qgdo_dn1: f64 = 0.0;
        let mut var_qgdo_dn2: f64 = 0.0;
        let mut var_qgdo_dn3: f64 = 0.0;
        let mut var_qgdo_dn4: f64 = 0.0;
        let mut var_qgdo_dn5: f64 = 0.0;
        let mut var_qgdo_dn6: f64 = 0.0;
        let mut var_qgdo_dn7: f64 = 0.0;
        let mut var_qgdo_dn8: f64 = 0.0;
        let mut var_qgdo_dn9: f64 = 0.0;
        let mut var_qgdo_dn10: f64 = 0.0;
        let mut var_qgdo_dn11: f64 = 0.0;
        let mut var_qgdo_dn12: f64 = 0.0;
        let mut var_qgdo_dn13: f64 = 0.0;
        let mut var_qgdo_dn14: f64 = 0.0;
        let mut var_qgdo_dn15: f64 = 0.0;
        let mut var_qgdo_dn16: f64 = 0.0;
        let mut var_qgdo_dn17: f64 = 0.0;
        let mut var_qgdo_db0: f64 = 0.0;
        let mut var_qgdo_db1: f64 = 0.0;
        let mut var_qgdo_db2: f64 = 0.0;
        let mut var_qgdo_db3: f64 = 0.0;
        let mut var_qgdo_db4: f64 = 0.0;
        let mut var_qgdo_db5: f64 = 0.0;
        let mut var_qgdo_db6: f64 = 0.0;
        let mut var_qgdo_db7: f64 = 0.0;
        let mut var_qgdo_db8: f64 = 0.0;
        let mut var_qgdo_db9: f64 = 0.0;
        let mut var_qgdo_db10: f64 = 0.0;
        let mut var_qgdo_db11: f64 = 0.0;
        let mut var_qfd: f64 = 0.0;
        let mut var_qfd_dn0: f64 = 0.0;
        let mut var_qfd_dn1: f64 = 0.0;
        let mut var_qfd_dn2: f64 = 0.0;
        let mut var_qfd_dn3: f64 = 0.0;
        let mut var_qfd_dn4: f64 = 0.0;
        let mut var_qfd_dn5: f64 = 0.0;
        let mut var_qfd_dn6: f64 = 0.0;
        let mut var_qfd_dn7: f64 = 0.0;
        let mut var_qfd_dn8: f64 = 0.0;
        let mut var_qfd_dn9: f64 = 0.0;
        let mut var_qfd_dn10: f64 = 0.0;
        let mut var_qfd_dn11: f64 = 0.0;
        let mut var_qfd_dn12: f64 = 0.0;
        let mut var_qfd_dn13: f64 = 0.0;
        let mut var_qfd_dn14: f64 = 0.0;
        let mut var_qfd_dn15: f64 = 0.0;
        let mut var_qfd_dn16: f64 = 0.0;
        let mut var_qfd_dn17: f64 = 0.0;
        let mut var_qfd_db0: f64 = 0.0;
        let mut var_qfd_db1: f64 = 0.0;
        let mut var_qfd_db2: f64 = 0.0;
        let mut var_qfd_db3: f64 = 0.0;
        let mut var_qfd_db4: f64 = 0.0;
        let mut var_qfd_db5: f64 = 0.0;
        let mut var_qfd_db6: f64 = 0.0;
        let mut var_qfd_db7: f64 = 0.0;
        let mut var_qfd_db8: f64 = 0.0;
        let mut var_qfd_db9: f64 = 0.0;
        let mut var_qfd_db10: f64 = 0.0;
        let mut var_qfd_db11: f64 = 0.0;
        let mut var_cfd: f64 = 0.0;
        let mut var_qfs: f64 = 0.0;
        let mut var_qfs_dn0: f64 = 0.0;
        let mut var_qfs_dn1: f64 = 0.0;
        let mut var_qfs_dn2: f64 = 0.0;
        let mut var_qfs_dn3: f64 = 0.0;
        let mut var_qfs_dn4: f64 = 0.0;
        let mut var_qfs_dn5: f64 = 0.0;
        let mut var_qfs_dn6: f64 = 0.0;
        let mut var_qfs_dn7: f64 = 0.0;
        let mut var_qfs_dn8: f64 = 0.0;
        let mut var_qfs_dn9: f64 = 0.0;
        let mut var_qfs_dn10: f64 = 0.0;
        let mut var_qfs_dn11: f64 = 0.0;
        let mut var_qfs_dn12: f64 = 0.0;
        let mut var_qfs_dn13: f64 = 0.0;
        let mut var_qfs_dn14: f64 = 0.0;
        let mut var_qfs_dn15: f64 = 0.0;
        let mut var_qfs_dn16: f64 = 0.0;
        let mut var_qfs_dn17: f64 = 0.0;
        let mut var_qfs_db0: f64 = 0.0;
        let mut var_qfs_db1: f64 = 0.0;
        let mut var_qfs_db2: f64 = 0.0;
        let mut var_qfs_db3: f64 = 0.0;
        let mut var_qfs_db4: f64 = 0.0;
        let mut var_qfs_db5: f64 = 0.0;
        let mut var_qfs_db6: f64 = 0.0;
        let mut var_qfs_db7: f64 = 0.0;
        let mut var_qfs_db8: f64 = 0.0;
        let mut var_qfs_db9: f64 = 0.0;
        let mut var_qfs_db10: f64 = 0.0;
        let mut var_qfs_db11: f64 = 0.0;
        let mut var_cfs: f64 = 0.0;
        let mut var_mfactor: f64 = 0.0;
        let mut var_coxb0: f64 = 0.0;
        let mut var_uc_nover: f64 = 0.0;
        let mut var_uc_novers: f64 = 0.0;
        let mut var_uc_cgso: f64 = 0.0;
        let mut var_uc_cgdo: f64 = 0.0;
        let mut var_uc_toxb: f64 = 0.0;
        let mut var_mks_wl: f64 = 0.0;
        let mut var_lg: f64 = 0.0;
        let mut var_dwcv: f64 = 0.0;
        let mut var_wg: f64 = 0.0;
        let mut var_lgate: f64 = 0.0;
        let mut var_wgate: f64 = 0.0;
        let mut var_cecox: f64 = 0.0;
        let mut var_weff_cv: f64 = 0.0;
        let mut var_weffcv_nf: f64 = 0.0;
        let mut var_cfrng: f64 = 0.0;
        let mut var_qdp: f64 = 0.0;
        let mut var_qdp_dn0: f64 = 0.0;
        let mut var_qdp_dn1: f64 = 0.0;
        let mut var_qdp_dn2: f64 = 0.0;
        let mut var_qdp_dn3: f64 = 0.0;
        let mut var_qdp_dn4: f64 = 0.0;
        let mut var_qdp_dn5: f64 = 0.0;
        let mut var_qdp_dn6: f64 = 0.0;
        let mut var_qdp_dn7: f64 = 0.0;
        let mut var_qdp_dn8: f64 = 0.0;
        let mut var_qdp_dn9: f64 = 0.0;
        let mut var_qdp_dn10: f64 = 0.0;
        let mut var_qdp_dn11: f64 = 0.0;
        let mut var_qdp_dn12: f64 = 0.0;
        let mut var_qdp_dn13: f64 = 0.0;
        let mut var_qdp_dn14: f64 = 0.0;
        let mut var_qdp_dn15: f64 = 0.0;
        let mut var_qdp_dn16: f64 = 0.0;
        let mut var_qdp_dn17: f64 = 0.0;
        let mut var_qdp_db0: f64 = 0.0;
        let mut var_qdp_db1: f64 = 0.0;
        let mut var_qdp_db2: f64 = 0.0;
        let mut var_qdp_db3: f64 = 0.0;
        let mut var_qdp_db4: f64 = 0.0;
        let mut var_qdp_db5: f64 = 0.0;
        let mut var_qdp_db6: f64 = 0.0;
        let mut var_qdp_db7: f64 = 0.0;
        let mut var_qdp_db8: f64 = 0.0;
        let mut var_qdp_db9: f64 = 0.0;
        let mut var_qdp_db10: f64 = 0.0;
        let mut var_qdp_db11: f64 = 0.0;
        let mut var_qsp: f64 = 0.0;
        let mut var_qsp_dn0: f64 = 0.0;
        let mut var_qsp_dn1: f64 = 0.0;
        let mut var_qsp_dn2: f64 = 0.0;
        let mut var_qsp_dn3: f64 = 0.0;
        let mut var_qsp_dn4: f64 = 0.0;
        let mut var_qsp_dn5: f64 = 0.0;
        let mut var_qsp_dn6: f64 = 0.0;
        let mut var_qsp_dn7: f64 = 0.0;
        let mut var_qsp_dn8: f64 = 0.0;
        let mut var_qsp_dn9: f64 = 0.0;
        let mut var_qsp_dn10: f64 = 0.0;
        let mut var_qsp_dn11: f64 = 0.0;
        let mut var_qsp_dn12: f64 = 0.0;
        let mut var_qsp_dn13: f64 = 0.0;
        let mut var_qsp_dn14: f64 = 0.0;
        let mut var_qsp_dn15: f64 = 0.0;
        let mut var_qsp_dn16: f64 = 0.0;
        let mut var_qsp_dn17: f64 = 0.0;
        let mut var_qsp_db0: f64 = 0.0;
        let mut var_qsp_db1: f64 = 0.0;
        let mut var_qsp_db2: f64 = 0.0;
        let mut var_qsp_db3: f64 = 0.0;
        let mut var_qsp_db4: f64 = 0.0;
        let mut var_qsp_db5: f64 = 0.0;
        let mut var_qsp_db6: f64 = 0.0;
        let mut var_qsp_db7: f64 = 0.0;
        let mut var_qsp_db8: f64 = 0.0;
        let mut var_qsp_db9: f64 = 0.0;
        let mut var_qsp_db10: f64 = 0.0;
        let mut var_qsp_db11: f64 = 0.0;
        let mut var_vdsei: f64 = 0.0;
        let mut var_vdsei_dn0: f64 = 0.0;
        let mut var_vdsei_dn1: f64 = 0.0;
        let mut var_vdsei_dn2: f64 = 0.0;
        let mut var_vdsei_dn3: f64 = 0.0;
        let mut var_vdsei_dn4: f64 = 0.0;
        let mut var_vdsei_dn5: f64 = 0.0;
        let mut var_vdsei_dn6: f64 = 0.0;
        let mut var_vdsei_dn7: f64 = 0.0;
        let mut var_vdsei_dn8: f64 = 0.0;
        let mut var_vdsei_dn9: f64 = 0.0;
        let mut var_vdsei_dn10: f64 = 0.0;
        let mut var_vdsei_dn11: f64 = 0.0;
        let mut var_vdsei_dn12: f64 = 0.0;
        let mut var_vdsei_dn13: f64 = 0.0;
        let mut var_vdsei_dn14: f64 = 0.0;
        let mut var_vdsei_dn15: f64 = 0.0;
        let mut var_vdsei_dn16: f64 = 0.0;
        let mut var_vdsei_dn17: f64 = 0.0;
        let mut var_vdsei_db0: f64 = 0.0;
        let mut var_vdsei_db1: f64 = 0.0;
        let mut var_vdsei_db2: f64 = 0.0;
        let mut var_vdsei_db3: f64 = 0.0;
        let mut var_vdsei_db4: f64 = 0.0;
        let mut var_vdsei_db5: f64 = 0.0;
        let mut var_vdsei_db6: f64 = 0.0;
        let mut var_vdsei_db7: f64 = 0.0;
        let mut var_vdsei_db8: f64 = 0.0;
        let mut var_vdsei_db9: f64 = 0.0;
        let mut var_vdsei_db10: f64 = 0.0;
        let mut var_vdsei_db11: f64 = 0.0;
        let mut var_vgsei: f64 = 0.0;
        let mut var_vgsei_dn0: f64 = 0.0;
        let mut var_vgsei_dn1: f64 = 0.0;
        let mut var_vgsei_dn2: f64 = 0.0;
        let mut var_vgsei_dn3: f64 = 0.0;
        let mut var_vgsei_dn4: f64 = 0.0;
        let mut var_vgsei_dn5: f64 = 0.0;
        let mut var_vgsei_dn6: f64 = 0.0;
        let mut var_vgsei_dn7: f64 = 0.0;
        let mut var_vgsei_dn8: f64 = 0.0;
        let mut var_vgsei_dn9: f64 = 0.0;
        let mut var_vgsei_dn10: f64 = 0.0;
        let mut var_vgsei_dn11: f64 = 0.0;
        let mut var_vgsei_dn12: f64 = 0.0;
        let mut var_vgsei_dn13: f64 = 0.0;
        let mut var_vgsei_dn14: f64 = 0.0;
        let mut var_vgsei_dn15: f64 = 0.0;
        let mut var_vgsei_dn16: f64 = 0.0;
        let mut var_vgsei_dn17: f64 = 0.0;
        let mut var_vgsei_db0: f64 = 0.0;
        let mut var_vgsei_db1: f64 = 0.0;
        let mut var_vgsei_db2: f64 = 0.0;
        let mut var_vgsei_db3: f64 = 0.0;
        let mut var_vgsei_db4: f64 = 0.0;
        let mut var_vgsei_db5: f64 = 0.0;
        let mut var_vgsei_db6: f64 = 0.0;
        let mut var_vgsei_db7: f64 = 0.0;
        let mut var_vgsei_db8: f64 = 0.0;
        let mut var_vgsei_db9: f64 = 0.0;
        let mut var_vgsei_db10: f64 = 0.0;
        let mut var_vgsei_db11: f64 = 0.0;
        let mut var_cgso_given: f64 = 0.0;
        let mut var_cgdo_given: f64 = 0.0;
        let mut var_lbin: f64 = 0.0;
        let mut var_wbin: f64 = 0.0;
        let mut var_lwbin: f64 = 0.0;
        let mut var_cqi: f64 = 0.0;
        let mut var_cqb: f64 = 0.0;
        let mut var_guard168: f64 = 0.0;
        let mut var_guard169: f64 = 0.0;
        let mut var_flg_coovlps: f64 = 0.0;
        let mut var_flg_coovlp: f64 = 0.0;
        let mut var_guard1627: f64 = 0.0;
        let mut var_guard1628: f64 = 0.0;
        let mut var_guard1629: f64 = 0.0;
        let mut var_guard1631: f64 = 0.0;
        let mut var_guard1633: f64 = 0.0;
        let mut var_guard1748: f64 = 0.0;
        let mut var_guard1749: f64 = 0.0;
        let mut var_guard1750: f64 = 0.0;
        let mut var_guard1752: f64 = 0.0;
        let mut var_guard1754: f64 = 0.0;
        let mut var_guard1869: f64 = 0.0;
        let mut var_guard1870: f64 = 0.0;
        let mut var_guard1871: f64 = 0.0;
        let mut var_guard1873: f64 = 0.0;
        let mut var_guard1875: f64 = 0.0;
        let mut var_guard1990: f64 = 0.0;
        let mut var_guard1991: f64 = 0.0;
        let mut var_guard1992: f64 = 0.0;
        let mut var_guard1994: f64 = 0.0;
        let mut var_guard1996: f64 = 0.0;
        let mut var_guard2215: f64 = 0.0;
        let mut var_guard2217: f64 = 0.0;
        let mut var_guard2219: f64 = 0.0;
        let mut var_guard2220: f64 = 0.0;
        let mut var_guard2331: f64 = 0.0;

        Self::stamp_transient_block_0(s, param_given, &mut var_c_eox, &mut var_cgdo_given, &mut var_cgdoe, &mut var_cgso_given, &mut var_cgsoe, &mut var_cox0, &mut var_coxb0, &mut var_qgso, &mut var_qgso_db0, &mut var_qgso_db1, &mut var_qgso_db10, &mut var_qgso_db11, &mut var_qgso_db2, &mut var_qgso_db3, &mut var_qgso_db4, &mut var_qgso_db5, &mut var_qgso_db6, &mut var_qgso_db7, &mut var_qgso_db8, &mut var_qgso_db9, &mut var_qgso_dn0, &mut var_qgso_dn1, &mut var_qgso_dn10, &mut var_qgso_dn11, &mut var_qgso_dn12, &mut var_qgso_dn13, &mut var_qgso_dn14, &mut var_qgso_dn15, &mut var_qgso_dn16, &mut var_qgso_dn17, &mut var_qgso_dn2, &mut var_qgso_dn3, &mut var_qgso_dn4, &mut var_qgso_dn5, &mut var_qgso_dn6, &mut var_qgso_dn7, &mut var_qgso_dn8, &mut var_qgso_dn9, &mut var_tox0);
        Self::stamp_transient_block_1(s, p, &mut var_cfd, &mut var_cfs, &mut var_dwcv, &mut var_lg, &mut var_lgate, &mut var_qfd, &mut var_qfd_db0, &mut var_qfd_db1, &mut var_qfd_db10, &mut var_qfd_db11, &mut var_qfd_db2, &mut var_qfd_db3, &mut var_qfd_db4, &mut var_qfd_db5, &mut var_qfd_db6, &mut var_qfd_db7, &mut var_qfd_db8, &mut var_qfd_db9, &mut var_qfd_dn0, &mut var_qfd_dn1, &mut var_qfd_dn10, &mut var_qfd_dn11, &mut var_qfd_dn12, &mut var_qfd_dn13, &mut var_qfd_dn14, &mut var_qfd_dn15, &mut var_qfd_dn16, &mut var_qfd_dn17, &mut var_qfd_dn2, &mut var_qfd_dn3, &mut var_qfd_dn4, &mut var_qfd_dn5, &mut var_qfd_dn6, &mut var_qfd_dn7, &mut var_qfd_dn8, &mut var_qfd_dn9, &mut var_qfs, &mut var_qfs_db0, &mut var_qfs_db1, &mut var_qfs_db10, &mut var_qfs_db11, &mut var_qfs_db2, &mut var_qfs_db3, &mut var_qfs_db4, &mut var_qfs_db5, &mut var_qfs_db6, &mut var_qfs_db7, &mut var_qfs_db8, &mut var_qfs_db9, &mut var_qfs_dn0, &mut var_qfs_dn1, &mut var_qfs_dn10, &mut var_qfs_dn11, &mut var_qfs_dn12, &mut var_qfs_dn13, &mut var_qfs_dn14, &mut var_qfs_dn15, &mut var_qfs_dn16, &mut var_qfs_dn17, &mut var_qfs_dn2, &mut var_qfs_dn3, &mut var_qfs_dn4, &mut var_qfs_dn5, &mut var_qfs_dn6, &mut var_qfs_dn7, &mut var_qfs_dn8, &mut var_qfs_dn9, &mut var_qgdo, &mut var_qgdo_db0, &mut var_qgdo_db1, &mut var_qgdo_db10, &mut var_qgdo_db11, &mut var_qgdo_db2, &mut var_qgdo_db3, &mut var_qgdo_db4, &mut var_qgdo_db5, &mut var_qgdo_db6, &mut var_qgdo_db7, &mut var_qgdo_db8, &mut var_qgdo_db9, &mut var_qgdo_dn0, &mut var_qgdo_dn1, &mut var_qgdo_dn10, &mut var_qgdo_dn11, &mut var_qgdo_dn12, &mut var_qgdo_dn13, &mut var_qgdo_dn14, &mut var_qgdo_dn15, &mut var_qgdo_dn16, &mut var_qgdo_dn17, &mut var_qgdo_dn2, &mut var_qgdo_dn3, &mut var_qgdo_dn4, &mut var_qgdo_dn5, &mut var_qgdo_dn6, &mut var_qgdo_dn7, &mut var_qgdo_dn8, &mut var_qgdo_dn9, &mut var_vdsei, &mut var_vdsei_db0, &mut var_vdsei_db1, &mut var_vdsei_db10, &mut var_vdsei_db11, &mut var_vdsei_db2, &mut var_vdsei_db3, &mut var_vdsei_db4, &mut var_vdsei_db5, &mut var_vdsei_db6, &mut var_vdsei_db7, &mut var_vdsei_db8, &mut var_vdsei_db9, &mut var_vdsei_dn0, &mut var_vdsei_dn1, &mut var_vdsei_dn10, &mut var_vdsei_dn11, &mut var_vdsei_dn12, &mut var_vdsei_dn13, &mut var_vdsei_dn14, &mut var_vdsei_dn15, &mut var_vdsei_dn16, &mut var_vdsei_dn17, &mut var_vdsei_dn2, &mut var_vdsei_dn3, &mut var_vdsei_dn4, &mut var_vdsei_dn5, &mut var_vdsei_dn6, &mut var_vdsei_dn7, &mut var_vdsei_dn8, &mut var_vdsei_dn9, &mut var_vgsei, &mut var_vgsei_db0, &mut var_vgsei_db1, &mut var_vgsei_db10, &mut var_vgsei_db11, &mut var_vgsei_db2, &mut var_vgsei_db3, &mut var_vgsei_db4, &mut var_vgsei_db5, &mut var_vgsei_db6, &mut var_vgsei_db7, &mut var_vgsei_db8, &mut var_vgsei_db9, &mut var_vgsei_dn0, &mut var_vgsei_dn1, &mut var_vgsei_dn10, &mut var_vgsei_dn11, &mut var_vgsei_dn12, &mut var_vgsei_dn13, &mut var_vgsei_dn14, &mut var_vgsei_dn15, &mut var_vgsei_dn16, &mut var_vgsei_dn17, &mut var_vgsei_dn2, &mut var_vgsei_dn3, &mut var_vgsei_dn4, &mut var_vgsei_dn5, &mut var_vgsei_dn6, &mut var_vgsei_dn7, &mut var_vgsei_dn8, &mut var_vgsei_dn9, &mut var_wg, &mut var_wgate);
        Self::stamp_transient_block_2(s, p, &mut var_guard168, &mut var_guard169, &mut var_mfactor, &mut var_mks_wl, &mut var_uc_toxb);
        Self::stamp_transient_block_3(s, p, &mut var_lbin, &mut var_lg, &mut var_lgate, &mut var_lwbin, &mut var_uc_cgdo, &mut var_uc_cgso, &mut var_uc_nover, &mut var_uc_novers, &mut var_wbin, &mut var_wg, &mut var_wgate);
        Self::stamp_transient_block_4(s, p, var_lg, var_lgate, var_mks_wl, var_wg, var_wgate, &mut var_cecox, &mut var_dwcv, &mut var_uc_nover, &mut var_uc_novers, &mut var_weff_cv, &mut var_weffcv_nf);
        Self::stamp_transient_block_5(ctx, s, p, var_lg, var_lgate, var_mfactor, var_uc_nover, var_weffcv_nf, var_wg, &mut var_cfrng);
        Self::stamp_transient_block_6(s, p, var_uc_nover, var_uc_novers);
        Self::stamp_transient_block_7(s, p);
        Self::stamp_transient_block_8(ctx, s, p, nodes, &mut var_vdsei, &mut var_vdsei_db0, &mut var_vdsei_db1, &mut var_vdsei_db10, &mut var_vdsei_db11, &mut var_vdsei_db2, &mut var_vdsei_db3, &mut var_vdsei_db4, &mut var_vdsei_db5, &mut var_vdsei_db6, &mut var_vdsei_db7, &mut var_vdsei_db8, &mut var_vdsei_db9, &mut var_vdsei_dn0, &mut var_vdsei_dn1, &mut var_vdsei_dn10, &mut var_vdsei_dn11, &mut var_vdsei_dn12, &mut var_vdsei_dn13, &mut var_vdsei_dn14, &mut var_vdsei_dn15, &mut var_vdsei_dn16, &mut var_vdsei_dn17, &mut var_vdsei_dn2, &mut var_vdsei_dn3, &mut var_vdsei_dn4, &mut var_vdsei_dn5, &mut var_vdsei_dn6, &mut var_vdsei_dn7, &mut var_vdsei_dn8, &mut var_vdsei_dn9, &mut var_vgsei, &mut var_vgsei_db0, &mut var_vgsei_db1, &mut var_vgsei_db10, &mut var_vgsei_db11, &mut var_vgsei_db2, &mut var_vgsei_db3, &mut var_vgsei_db4, &mut var_vgsei_db5, &mut var_vgsei_db6, &mut var_vgsei_db7, &mut var_vgsei_db8, &mut var_vgsei_db9, &mut var_vgsei_dn0, &mut var_vgsei_dn1, &mut var_vgsei_dn10, &mut var_vgsei_dn11, &mut var_vgsei_dn12, &mut var_vgsei_dn13, &mut var_vgsei_dn14, &mut var_vgsei_dn15, &mut var_vgsei_dn16, &mut var_vgsei_dn17, &mut var_vgsei_dn2, &mut var_vgsei_dn3, &mut var_vgsei_dn4, &mut var_vgsei_dn5, &mut var_vgsei_dn6, &mut var_vgsei_dn7, &mut var_vgsei_dn8, &mut var_vgsei_dn9);
        Self::stamp_transient_block_9(s, p, var_uc_nover, var_uc_novers);
        Self::stamp_transient_block_10(ctx, s, p, param_given, var_cecox, var_uc_toxb, &mut var_c_eox, &mut var_cox0, &mut var_coxb0, &mut var_tox0);
        Self::stamp_transient_block_11(s, p, param_given, var_uc_nover, var_vdsei, var_vdsei_db0, var_vdsei_db1, var_vdsei_db10, var_vdsei_db11, var_vdsei_db2, var_vdsei_db3, var_vdsei_db4, var_vdsei_db5, var_vdsei_db6, var_vdsei_db7, var_vdsei_db8, var_vdsei_db9, var_vdsei_dn0, var_vdsei_dn1, var_vdsei_dn10, var_vdsei_dn11, var_vdsei_dn12, var_vdsei_dn13, var_vdsei_dn14, var_vdsei_dn15, var_vdsei_dn16, var_vdsei_dn17, var_vdsei_dn2, var_vdsei_dn3, var_vdsei_dn4, var_vdsei_dn5, var_vdsei_dn6, var_vdsei_dn7, var_vdsei_dn8, var_vdsei_dn9, var_vgsei, var_vgsei_db0, var_vgsei_db1, var_vgsei_db10, var_vgsei_db11, var_vgsei_db2, var_vgsei_db3, var_vgsei_db4, var_vgsei_db5, var_vgsei_db6, var_vgsei_db7, var_vgsei_db8, var_vgsei_db9, var_vgsei_dn0, var_vgsei_dn1, var_vgsei_dn10, var_vgsei_dn11, var_vgsei_dn12, var_vgsei_dn13, var_vgsei_dn14, var_vgsei_dn15, var_vgsei_dn16, var_vgsei_dn17, var_vgsei_dn2, var_vgsei_dn3, var_vgsei_dn4, var_vgsei_dn5, var_vgsei_dn6, var_vgsei_dn7, var_vgsei_dn8, var_vgsei_dn9);
        Self::stamp_transient_block_12(s, p, var_cox0, var_tox0, var_uc_nover);
        Self::stamp_transient_block_13(s, p, var_c_eox, var_lgate, var_tox0, var_wg);
        Self::stamp_transient_block_14(s, p);
        Self::stamp_transient_block_15(s);
        Self::stamp_transient_block_16(s);
        Self::stamp_transient_block_17(s);
        Self::stamp_transient_block_18(s);
        Self::stamp_transient_block_19(s);
        Self::stamp_transient_block_20(s);
        Self::stamp_transient_block_21(s);
        Self::stamp_transient_block_22(s);
        Self::stamp_transient_block_23(s);
        Self::stamp_transient_block_24(s);
        Self::stamp_transient_block_25(s);
        Self::stamp_transient_block_26(s);
        Self::stamp_transient_block_27(s);
        Self::stamp_transient_block_28(s);
        Self::stamp_transient_block_29(s);
        Self::stamp_transient_block_30(s, p);
        Self::stamp_transient_block_31(s, p);
        Self::stamp_transient_block_32(s, p);
        Self::stamp_transient_block_33(s, p);
        Self::stamp_transient_block_34(s, p);
        Self::stamp_transient_block_35(s);
        Self::stamp_transient_block_36(s);
        Self::stamp_transient_block_37(s, p);
        Self::stamp_transient_block_38(s, p);
        Self::stamp_transient_block_39(s, p);
        Self::stamp_transient_block_40(s, p);
        Self::stamp_transient_block_41(s, p);
        Self::stamp_transient_block_42(s, p);
        Self::stamp_transient_block_43(s, p, param_given);
        Self::stamp_transient_block_44(s, p);
        Self::stamp_transient_block_45(s, p);
        Self::stamp_transient_block_46(s, p);
        Self::stamp_transient_block_47(s, p);
        Self::stamp_transient_block_48(s, p);
        Self::stamp_transient_block_49(s, p);
        Self::stamp_transient_block_50(s, p);
        Self::stamp_transient_block_51(s, p);
        Self::stamp_transient_block_52(s, p);
        Self::stamp_transient_block_53(s, p);
        Self::stamp_transient_block_54(s);
        Self::stamp_transient_block_55(s, p);
        Self::stamp_transient_block_56(s, p);
        Self::stamp_transient_block_57(s, p);
        Self::stamp_transient_block_58(s, p, param_given);
        Self::stamp_transient_block_59(s, p);
        Self::stamp_transient_block_60(s);
        Self::stamp_transient_block_61(s);
        Self::stamp_transient_block_62(s);
        Self::stamp_transient_block_63(s, p);
        Self::stamp_transient_block_64(s, p);
        Self::stamp_transient_block_65(s, p);
        Self::stamp_transient_block_66(s, p);
        Self::stamp_transient_block_67(s, p, var_weffcv_nf);
        Self::stamp_transient_block_68(s, p, var_cox0, var_lgate);
        Self::stamp_transient_block_69(s, p, var_cox0, var_mfactor, var_tox0);
        Self::stamp_transient_block_70(s, p, var_cox0, var_coxb0, var_lgate, var_tox0, var_uc_nover, var_uc_novers, var_vgsei, var_vgsei_db0, var_vgsei_db1, var_vgsei_db10, var_vgsei_db11, var_vgsei_db2, var_vgsei_db3, var_vgsei_db4, var_vgsei_db5, var_vgsei_db6, var_vgsei_db7, var_vgsei_db8, var_vgsei_db9, var_vgsei_dn0, var_vgsei_dn1, var_vgsei_dn10, var_vgsei_dn11, var_vgsei_dn12, var_vgsei_dn13, var_vgsei_dn14, var_vgsei_dn15, var_vgsei_dn16, var_vgsei_dn17, var_vgsei_dn2, var_vgsei_dn3, var_vgsei_dn4, var_vgsei_dn5, var_vgsei_dn6, var_vgsei_dn7, var_vgsei_dn8, var_vgsei_dn9, &mut var_flg_coovlp, &mut var_flg_coovlps, &mut var_guard1627, &mut var_guard1628, &mut var_guard1629, &mut var_guard1631, &mut var_guard1633);
        Self::stamp_transient_block_71(s, p, param_given, var_guard1627, var_guard1628, var_guard1629, var_vdsei, var_vdsei_db0, var_vdsei_db1, var_vdsei_db10, var_vdsei_db11, var_vdsei_db2, var_vdsei_db3, var_vdsei_db4, var_vdsei_db5, var_vdsei_db6, var_vdsei_db7, var_vdsei_db8, var_vdsei_db9, var_vdsei_dn0, var_vdsei_dn1, var_vdsei_dn10, var_vdsei_dn11, var_vdsei_dn12, var_vdsei_dn13, var_vdsei_dn14, var_vdsei_dn15, var_vdsei_dn16, var_vdsei_dn17, var_vdsei_dn2, var_vdsei_dn3, var_vdsei_dn4, var_vdsei_dn5, var_vdsei_dn6, var_vdsei_dn7, var_vdsei_dn8, var_vdsei_dn9, var_vgsei, var_vgsei_db0, var_vgsei_db1, var_vgsei_db10, var_vgsei_db11, var_vgsei_db2, var_vgsei_db3, var_vgsei_db4, var_vgsei_db5, var_vgsei_db6, var_vgsei_db7, var_vgsei_db8, var_vgsei_db9, var_vgsei_dn0, var_vgsei_dn1, var_vgsei_dn10, var_vgsei_dn11, var_vgsei_dn12, var_vgsei_dn13, var_vgsei_dn14, var_vgsei_dn15, var_vgsei_dn16, var_vgsei_dn17, var_vgsei_dn2, var_vgsei_dn3, var_vgsei_dn4, var_vgsei_dn5, var_vgsei_dn6, var_vgsei_dn7, var_vgsei_dn8, var_vgsei_dn9);
        Self::stamp_transient_block_72(s, p);
        Self::stamp_transient_block_73(s, p);
        Self::stamp_transient_block_74(s, p);
        Self::stamp_transient_block_75(s, p, var_weffcv_nf);
        Self::stamp_transient_block_76(s, p, param_given, var_cox0, var_coxb0, var_uc_nover, var_uc_novers, var_vdsei, var_vdsei_db0, var_vdsei_db1, var_vdsei_db10, var_vdsei_db11, var_vdsei_db2, var_vdsei_db3, var_vdsei_db4, var_vdsei_db5, var_vdsei_db6, var_vdsei_db7, var_vdsei_db8, var_vdsei_db9, var_vdsei_dn0, var_vdsei_dn1, var_vdsei_dn10, var_vdsei_dn11, var_vdsei_dn12, var_vdsei_dn13, var_vdsei_dn14, var_vdsei_dn15, var_vdsei_dn16, var_vdsei_dn17, var_vdsei_dn2, var_vdsei_dn3, var_vdsei_dn4, var_vdsei_dn5, var_vdsei_dn6, var_vdsei_dn7, var_vdsei_dn8, var_vdsei_dn9, var_vgsei, var_vgsei_db0, var_vgsei_db1, var_vgsei_db10, var_vgsei_db11, var_vgsei_db2, var_vgsei_db3, var_vgsei_db4, var_vgsei_db5, var_vgsei_db6, var_vgsei_db7, var_vgsei_db8, var_vgsei_db9, var_vgsei_dn0, var_vgsei_dn1, var_vgsei_dn10, var_vgsei_dn11, var_vgsei_dn12, var_vgsei_dn13, var_vgsei_dn14, var_vgsei_dn15, var_vgsei_dn16, var_vgsei_dn17, var_vgsei_dn2, var_vgsei_dn3, var_vgsei_dn4, var_vgsei_dn5, var_vgsei_dn6, var_vgsei_dn7, var_vgsei_dn8, var_vgsei_dn9, var_weffcv_nf, &mut var_flg_coovlp, &mut var_flg_coovlps, &mut var_guard1748, &mut var_guard1749, &mut var_guard1750, &mut var_guard1752, &mut var_guard1754);
        Self::stamp_transient_block_77(s, p);
        Self::stamp_transient_block_78(s, p);
        Self::stamp_transient_block_79(s, p);
        Self::stamp_transient_block_80(s, p);
        Self::stamp_transient_block_81(s, p, var_cox0, var_uc_nover, var_uc_novers, var_vgsei, var_vgsei_db0, var_vgsei_db1, var_vgsei_db10, var_vgsei_db11, var_vgsei_db2, var_vgsei_db3, var_vgsei_db4, var_vgsei_db5, var_vgsei_db6, var_vgsei_db7, var_vgsei_db8, var_vgsei_db9, var_vgsei_dn0, var_vgsei_dn1, var_vgsei_dn10, var_vgsei_dn11, var_vgsei_dn12, var_vgsei_dn13, var_vgsei_dn14, var_vgsei_dn15, var_vgsei_dn16, var_vgsei_dn17, var_vgsei_dn2, var_vgsei_dn3, var_vgsei_dn4, var_vgsei_dn5, var_vgsei_dn6, var_vgsei_dn7, var_vgsei_dn8, var_vgsei_dn9, var_weffcv_nf, &mut var_flg_coovlps, &mut var_guard1869, &mut var_guard1870, &mut var_guard1871, &mut var_guard1873, &mut var_guard1875);
        Self::stamp_transient_block_82(s, p, param_given, var_coxb0, var_guard1869, var_guard1870, var_guard1871, var_guard1875, var_uc_nover, var_vdsei, var_vdsei_db0, var_vdsei_db1, var_vdsei_db10, var_vdsei_db11, var_vdsei_db2, var_vdsei_db3, var_vdsei_db4, var_vdsei_db5, var_vdsei_db6, var_vdsei_db7, var_vdsei_db8, var_vdsei_db9, var_vdsei_dn0, var_vdsei_dn1, var_vdsei_dn10, var_vdsei_dn11, var_vdsei_dn12, var_vdsei_dn13, var_vdsei_dn14, var_vdsei_dn15, var_vdsei_dn16, var_vdsei_dn17, var_vdsei_dn2, var_vdsei_dn3, var_vdsei_dn4, var_vdsei_dn5, var_vdsei_dn6, var_vdsei_dn7, var_vdsei_dn8, var_vdsei_dn9, var_vgsei, var_vgsei_db0, var_vgsei_db1, var_vgsei_db10, var_vgsei_db11, var_vgsei_db2, var_vgsei_db3, var_vgsei_db4, var_vgsei_db5, var_vgsei_db6, var_vgsei_db7, var_vgsei_db8, var_vgsei_db9, var_vgsei_dn0, var_vgsei_dn1, var_vgsei_dn10, var_vgsei_dn11, var_vgsei_dn12, var_vgsei_dn13, var_vgsei_dn14, var_vgsei_dn15, var_vgsei_dn16, var_vgsei_dn17, var_vgsei_dn2, var_vgsei_dn3, var_vgsei_dn4, var_vgsei_dn5, var_vgsei_dn6, var_vgsei_dn7, var_vgsei_dn8, var_vgsei_dn9, &mut var_flg_coovlp);
        Self::stamp_transient_block_83(s, p);
        Self::stamp_transient_block_84(s, p);
        Self::stamp_transient_block_85(s, p);
        Self::stamp_transient_block_86(s, p);
        Self::stamp_transient_block_87(s, p, var_cox0, var_uc_nover, var_uc_novers, var_vgsei, var_vgsei_db0, var_vgsei_db1, var_vgsei_db10, var_vgsei_db11, var_vgsei_db2, var_vgsei_db3, var_vgsei_db4, var_vgsei_db5, var_vgsei_db6, var_vgsei_db7, var_vgsei_db8, var_vgsei_db9, var_vgsei_dn0, var_vgsei_dn1, var_vgsei_dn10, var_vgsei_dn11, var_vgsei_dn12, var_vgsei_dn13, var_vgsei_dn14, var_vgsei_dn15, var_vgsei_dn16, var_vgsei_dn17, var_vgsei_dn2, var_vgsei_dn3, var_vgsei_dn4, var_vgsei_dn5, var_vgsei_dn6, var_vgsei_dn7, var_vgsei_dn8, var_vgsei_dn9, var_weffcv_nf, &mut var_flg_coovlps, &mut var_guard1990, &mut var_guard1991, &mut var_guard1992, &mut var_guard1994, &mut var_guard1996);
        Self::stamp_transient_block_88(s, p, param_given, var_coxb0, var_guard1990, var_guard1991, var_guard1992, var_guard1996, var_uc_nover, var_vdsei, var_vdsei_db0, var_vdsei_db1, var_vdsei_db10, var_vdsei_db11, var_vdsei_db2, var_vdsei_db3, var_vdsei_db4, var_vdsei_db5, var_vdsei_db6, var_vdsei_db7, var_vdsei_db8, var_vdsei_db9, var_vdsei_dn0, var_vdsei_dn1, var_vdsei_dn10, var_vdsei_dn11, var_vdsei_dn12, var_vdsei_dn13, var_vdsei_dn14, var_vdsei_dn15, var_vdsei_dn16, var_vdsei_dn17, var_vdsei_dn2, var_vdsei_dn3, var_vdsei_dn4, var_vdsei_dn5, var_vdsei_dn6, var_vdsei_dn7, var_vdsei_dn8, var_vdsei_dn9, var_vgsei, var_vgsei_db0, var_vgsei_db1, var_vgsei_db10, var_vgsei_db11, var_vgsei_db2, var_vgsei_db3, var_vgsei_db4, var_vgsei_db5, var_vgsei_db6, var_vgsei_db7, var_vgsei_db8, var_vgsei_db9, var_vgsei_dn0, var_vgsei_dn1, var_vgsei_dn10, var_vgsei_dn11, var_vgsei_dn12, var_vgsei_dn13, var_vgsei_dn14, var_vgsei_dn15, var_vgsei_dn16, var_vgsei_dn17, var_vgsei_dn2, var_vgsei_dn3, var_vgsei_dn4, var_vgsei_dn5, var_vgsei_dn6, var_vgsei_dn7, var_vgsei_dn8, var_vgsei_dn9, &mut var_flg_coovlp);
        Self::stamp_transient_block_89(s, p);
        Self::stamp_transient_block_90(s, p);
        Self::stamp_transient_block_91(s, p);
        Self::stamp_transient_block_92(s, p);
        Self::stamp_transient_block_93(s, p, var_coxb0, var_uc_nover, var_weffcv_nf);
        Self::stamp_transient_block_94(s, p, param_given);
        Self::stamp_transient_block_95(s, p);
        Self::stamp_transient_block_96(s, p);
        Self::stamp_transient_block_97(s, p);
        Self::stamp_transient_block_98(s, p, var_cgso_given, var_cox0, var_coxb0, var_uc_cgso, var_uc_nover, var_uc_novers, var_weffcv_nf, &mut var_cgsoe, &mut var_flg_coovlp, &mut var_flg_coovlps, &mut var_guard2215, &mut var_guard2217, &mut var_guard2219);
        Self::stamp_transient_block_99(s, p, var_cfrng, var_cgdo_given, var_cgso_given, var_cox0, var_coxb0, var_flg_coovlp, var_guard2219, var_lgate, var_uc_cgdo, var_vdsei, var_vdsei_db0, var_vdsei_db1, var_vdsei_db10, var_vdsei_db11, var_vdsei_db2, var_vdsei_db3, var_vdsei_db4, var_vdsei_db5, var_vdsei_db6, var_vdsei_db7, var_vdsei_db8, var_vdsei_db9, var_vdsei_dn0, var_vdsei_dn1, var_vdsei_dn10, var_vdsei_dn11, var_vdsei_dn12, var_vdsei_dn13, var_vdsei_dn14, var_vdsei_dn15, var_vdsei_dn16, var_vdsei_dn17, var_vdsei_dn2, var_vdsei_dn3, var_vdsei_dn4, var_vdsei_dn5, var_vdsei_dn6, var_vdsei_dn7, var_vdsei_dn8, var_vdsei_dn9, var_vgsei, var_vgsei_db0, var_vgsei_db1, var_vgsei_db10, var_vgsei_db11, var_vgsei_db2, var_vgsei_db3, var_vgsei_db4, var_vgsei_db5, var_vgsei_db6, var_vgsei_db7, var_vgsei_db8, var_vgsei_db9, var_vgsei_dn0, var_vgsei_dn1, var_vgsei_dn10, var_vgsei_dn11, var_vgsei_dn12, var_vgsei_dn13, var_vgsei_dn14, var_vgsei_dn15, var_vgsei_dn16, var_vgsei_dn17, var_vgsei_dn2, var_vgsei_dn3, var_vgsei_dn4, var_vgsei_dn5, var_vgsei_dn6, var_vgsei_dn7, var_vgsei_dn8, var_vgsei_dn9, var_weffcv_nf, &mut var_cfd, &mut var_cfs, &mut var_cgdoe, &mut var_cgsoe, &mut var_guard2220, &mut var_qfd, &mut var_qfd_db0, &mut var_qfd_db1, &mut var_qfd_db10, &mut var_qfd_db11, &mut var_qfd_db2, &mut var_qfd_db3, &mut var_qfd_db4, &mut var_qfd_db5, &mut var_qfd_db6, &mut var_qfd_db7, &mut var_qfd_db8, &mut var_qfd_db9, &mut var_qfd_dn0, &mut var_qfd_dn1, &mut var_qfd_dn10, &mut var_qfd_dn11, &mut var_qfd_dn12, &mut var_qfd_dn13, &mut var_qfd_dn14, &mut var_qfd_dn15, &mut var_qfd_dn16, &mut var_qfd_dn17, &mut var_qfd_dn2, &mut var_qfd_dn3, &mut var_qfd_dn4, &mut var_qfd_dn5, &mut var_qfd_dn6, &mut var_qfd_dn7, &mut var_qfd_dn8, &mut var_qfd_dn9, &mut var_qfs, &mut var_qfs_db0, &mut var_qfs_db1, &mut var_qfs_db10, &mut var_qfs_db11, &mut var_qfs_db2, &mut var_qfs_db3, &mut var_qfs_db4, &mut var_qfs_db5, &mut var_qfs_db6, &mut var_qfs_db7, &mut var_qfs_db8, &mut var_qfs_db9, &mut var_qfs_dn0, &mut var_qfs_dn1, &mut var_qfs_dn10, &mut var_qfs_dn11, &mut var_qfs_dn12, &mut var_qfs_dn13, &mut var_qfs_dn14, &mut var_qfs_dn15, &mut var_qfs_dn16, &mut var_qfs_dn17, &mut var_qfs_dn2, &mut var_qfs_dn3, &mut var_qfs_dn4, &mut var_qfs_dn5, &mut var_qfs_dn6, &mut var_qfs_dn7, &mut var_qfs_dn8, &mut var_qfs_dn9, &mut var_qgdo, &mut var_qgdo_db0, &mut var_qgdo_db1, &mut var_qgdo_db10, &mut var_qgdo_db11, &mut var_qgdo_db2, &mut var_qgdo_db3, &mut var_qgdo_db4, &mut var_qgdo_db5, &mut var_qgdo_db6, &mut var_qgdo_db7, &mut var_qgdo_db8, &mut var_qgdo_db9, &mut var_qgdo_dn0, &mut var_qgdo_dn1, &mut var_qgdo_dn10, &mut var_qgdo_dn11, &mut var_qgdo_dn12, &mut var_qgdo_dn13, &mut var_qgdo_dn14, &mut var_qgdo_dn15, &mut var_qgdo_dn16, &mut var_qgdo_dn17, &mut var_qgdo_dn2, &mut var_qgdo_dn3, &mut var_qgdo_dn4, &mut var_qgdo_dn5, &mut var_qgdo_dn6, &mut var_qgdo_dn7, &mut var_qgdo_dn8, &mut var_qgdo_dn9, &mut var_qgso, &mut var_qgso_db0, &mut var_qgso_db1, &mut var_qgso_db10, &mut var_qgso_db11, &mut var_qgso_db2, &mut var_qgso_db3, &mut var_qgso_db4, &mut var_qgso_db5, &mut var_qgso_db6, &mut var_qgso_db7, &mut var_qgso_db8, &mut var_qgso_db9, &mut var_qgso_dn0, &mut var_qgso_dn1, &mut var_qgso_dn10, &mut var_qgso_dn11, &mut var_qgso_dn12, &mut var_qgso_dn13, &mut var_qgso_dn14, &mut var_qgso_dn15, &mut var_qgso_dn16, &mut var_qgso_dn17, &mut var_qgso_dn2, &mut var_qgso_dn3, &mut var_qgso_dn4, &mut var_qgso_dn5, &mut var_qgso_dn6, &mut var_qgso_dn7, &mut var_qgso_dn8, &mut var_qgso_dn9);
        Self::stamp_transient_block_100(ctx, s, p);
        Self::stamp_transient_block_101(s, p);
        Self::stamp_transient_block_102(ctx, s, p, nodes, var_cox0, var_mfactor);
        Self::stamp_transient_block_103(s, p, var_lgate, var_weffcv_nf);
        Self::stamp_transient_block_104(ctx, s, p, nodes, var_cox0, var_mfactor, var_qfd, var_qfd_db0, var_qfd_db1, var_qfd_db10, var_qfd_db11, var_qfd_db2, var_qfd_db3, var_qfd_db4, var_qfd_db5, var_qfd_db6, var_qfd_db7, var_qfd_db8, var_qfd_db9, var_qfd_dn0, var_qfd_dn1, var_qfd_dn10, var_qfd_dn11, var_qfd_dn12, var_qfd_dn13, var_qfd_dn14, var_qfd_dn15, var_qfd_dn16, var_qfd_dn17, var_qfd_dn2, var_qfd_dn3, var_qfd_dn4, var_qfd_dn5, var_qfd_dn6, var_qfd_dn7, var_qfd_dn8, var_qfd_dn9, var_qfs, var_qfs_db0, var_qfs_db1, var_qfs_db10, var_qfs_db11, var_qfs_db2, var_qfs_db3, var_qfs_db4, var_qfs_db5, var_qfs_db6, var_qfs_db7, var_qfs_db8, var_qfs_db9, var_qfs_dn0, var_qfs_dn1, var_qfs_dn10, var_qfs_dn11, var_qfs_dn12, var_qfs_dn13, var_qfs_dn14, var_qfs_dn15, var_qfs_dn16, var_qfs_dn17, var_qfs_dn2, var_qfs_dn3, var_qfs_dn4, var_qfs_dn5, var_qfs_dn6, var_qfs_dn7, var_qfs_dn8, var_qfs_dn9, var_qgdo, var_qgdo_db0, var_qgdo_db1, var_qgdo_db10, var_qgdo_db11, var_qgdo_db2, var_qgdo_db3, var_qgdo_db4, var_qgdo_db5, var_qgdo_db6, var_qgdo_db7, var_qgdo_db8, var_qgdo_db9, var_qgdo_dn0, var_qgdo_dn1, var_qgdo_dn10, var_qgdo_dn11, var_qgdo_dn12, var_qgdo_dn13, var_qgdo_dn14, var_qgdo_dn15, var_qgdo_dn16, var_qgdo_dn17, var_qgdo_dn2, var_qgdo_dn3, var_qgdo_dn4, var_qgdo_dn5, var_qgdo_dn6, var_qgdo_dn7, var_qgdo_dn8, var_qgdo_dn9, var_qgso, var_qgso_db0, var_qgso_db1, var_qgso_db10, var_qgso_db11, var_qgso_db2, var_qgso_db3, var_qgso_db4, var_qgso_db5, var_qgso_db6, var_qgso_db7, var_qgso_db8, var_qgso_db9, var_qgso_dn0, var_qgso_dn1, var_qgso_dn10, var_qgso_dn11, var_qgso_dn12, var_qgso_dn13, var_qgso_dn14, var_qgso_dn15, var_qgso_dn16, var_qgso_dn17, var_qgso_dn2, var_qgso_dn3, var_qgso_dn4, var_qgso_dn5, var_qgso_dn6, var_qgso_dn7, var_qgso_dn8, var_qgso_dn9, var_uc_novers, var_weffcv_nf, &mut var_guard2331, &mut var_qdp, &mut var_qdp_db0, &mut var_qdp_db1, &mut var_qdp_db10, &mut var_qdp_db11, &mut var_qdp_db2, &mut var_qdp_db3, &mut var_qdp_db4, &mut var_qdp_db5, &mut var_qdp_db6, &mut var_qdp_db7, &mut var_qdp_db8, &mut var_qdp_db9, &mut var_qdp_dn0, &mut var_qdp_dn1, &mut var_qdp_dn10, &mut var_qdp_dn11, &mut var_qdp_dn12, &mut var_qdp_dn13, &mut var_qdp_dn14, &mut var_qdp_dn15, &mut var_qdp_dn16, &mut var_qdp_dn17, &mut var_qdp_dn2, &mut var_qdp_dn3, &mut var_qdp_dn4, &mut var_qdp_dn5, &mut var_qdp_dn6, &mut var_qdp_dn7, &mut var_qdp_dn8, &mut var_qdp_dn9, &mut var_qsp, &mut var_qsp_db0, &mut var_qsp_db1, &mut var_qsp_db10, &mut var_qsp_db11, &mut var_qsp_db2, &mut var_qsp_db3, &mut var_qsp_db4, &mut var_qsp_db5, &mut var_qsp_db6, &mut var_qsp_db7, &mut var_qsp_db8, &mut var_qsp_db9, &mut var_qsp_dn0, &mut var_qsp_dn1, &mut var_qsp_dn10, &mut var_qsp_dn11, &mut var_qsp_dn12, &mut var_qsp_dn13, &mut var_qsp_dn14, &mut var_qsp_dn15, &mut var_qsp_dn16, &mut var_qsp_dn17, &mut var_qsp_dn2, &mut var_qsp_dn3, &mut var_qsp_dn4, &mut var_qsp_dn5, &mut var_qsp_dn6, &mut var_qsp_dn7, &mut var_qsp_dn8, &mut var_qsp_dn9);
        Self::stamp_transient_block_105(ctx, s, p, var_mfactor, var_uc_nover);
        Self::stamp_transient_block_106(s, p);
        Self::stamp_transient_block_107(s, p, var_mfactor);
        Self::stamp_transient_block_108(s, p, var_qdp, var_qdp_db0, var_qdp_db1, var_qdp_db10, var_qdp_db11, var_qdp_db2, var_qdp_db3, var_qdp_db4, var_qdp_db5, var_qdp_db6, var_qdp_db7, var_qdp_db8, var_qdp_db9, var_qdp_dn0, var_qdp_dn1, var_qdp_dn10, var_qdp_dn11, var_qdp_dn12, var_qdp_dn13, var_qdp_dn14, var_qdp_dn15, var_qdp_dn16, var_qdp_dn17, var_qdp_dn2, var_qdp_dn3, var_qdp_dn4, var_qdp_dn5, var_qdp_dn6, var_qdp_dn7, var_qdp_dn8, var_qdp_dn9, var_qsp, var_qsp_db0, var_qsp_db1, var_qsp_db10, var_qsp_db11, var_qsp_db2, var_qsp_db3, var_qsp_db4, var_qsp_db5, var_qsp_db6, var_qsp_db7, var_qsp_db8, var_qsp_db9, var_qsp_dn0, var_qsp_dn1, var_qsp_dn10, var_qsp_dn11, var_qsp_dn12, var_qsp_dn13, var_qsp_dn14, var_qsp_dn15, var_qsp_dn16, var_qsp_dn17, var_qsp_dn2, var_qsp_dn3, var_qsp_dn4, var_qsp_dn5, var_qsp_dn6, var_qsp_dn7, var_qsp_dn8, var_qsp_dn9, var_vdsei, var_vdsei_db0, var_vdsei_db1, var_vdsei_db10, var_vdsei_db11, var_vdsei_db2, var_vdsei_db3, var_vdsei_db4, var_vdsei_db5, var_vdsei_db6, var_vdsei_db7, var_vdsei_db8, var_vdsei_db9, var_vdsei_dn0, var_vdsei_dn1, var_vdsei_dn10, var_vdsei_dn11, var_vdsei_dn12, var_vdsei_dn13, var_vdsei_dn14, var_vdsei_dn15, var_vdsei_dn16, var_vdsei_dn17, var_vdsei_dn2, var_vdsei_dn3, var_vdsei_dn4, var_vdsei_dn5, var_vdsei_dn6, var_vdsei_dn7, var_vdsei_dn8, var_vdsei_dn9, &mut var_cqb, &mut var_cqi, &mut var_qfd, &mut var_qfd_db0, &mut var_qfd_db1, &mut var_qfd_db10, &mut var_qfd_db11, &mut var_qfd_db2, &mut var_qfd_db3, &mut var_qfd_db4, &mut var_qfd_db5, &mut var_qfd_db6, &mut var_qfd_db7, &mut var_qfd_db8, &mut var_qfd_db9, &mut var_qfd_dn0, &mut var_qfd_dn1, &mut var_qfd_dn10, &mut var_qfd_dn11, &mut var_qfd_dn12, &mut var_qfd_dn13, &mut var_qfd_dn14, &mut var_qfd_dn15, &mut var_qfd_dn16, &mut var_qfd_dn17, &mut var_qfd_dn2, &mut var_qfd_dn3, &mut var_qfd_dn4, &mut var_qfd_dn5, &mut var_qfd_dn6, &mut var_qfd_dn7, &mut var_qfd_dn8, &mut var_qfd_dn9, &mut var_qfs, &mut var_qfs_db0, &mut var_qfs_db1, &mut var_qfs_db10, &mut var_qfs_db11, &mut var_qfs_db2, &mut var_qfs_db3, &mut var_qfs_db4, &mut var_qfs_db5, &mut var_qfs_db6, &mut var_qfs_db7, &mut var_qfs_db8, &mut var_qfs_db9, &mut var_qfs_dn0, &mut var_qfs_dn1, &mut var_qfs_dn10, &mut var_qfs_dn11, &mut var_qfs_dn12, &mut var_qfs_dn13, &mut var_qfs_dn14, &mut var_qfs_dn15, &mut var_qfs_dn16, &mut var_qfs_dn17, &mut var_qfs_dn2, &mut var_qfs_dn3, &mut var_qfs_dn4, &mut var_qfs_dn5, &mut var_qfs_dn6, &mut var_qfs_dn7, &mut var_qfs_dn8, &mut var_qfs_dn9);

        Self::stamp_transient_equations_block_0(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_1(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_2(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, var_qfd, var_qfd_db0, var_qfd_db1, var_qfd_db10, var_qfd_db11, var_qfd_db2, var_qfd_db3, var_qfd_db4, var_qfd_db5, var_qfd_db6, var_qfd_db7, var_qfd_db8, var_qfd_db9, var_qfd_dn0, var_qfd_dn1, var_qfd_dn10, var_qfd_dn11, var_qfd_dn12, var_qfd_dn13, var_qfd_dn14, var_qfd_dn15, var_qfd_dn16, var_qfd_dn17, var_qfd_dn2, var_qfd_dn3, var_qfd_dn4, var_qfd_dn5, var_qfd_dn6, var_qfd_dn7, var_qfd_dn8, var_qfd_dn9);
        Self::stamp_transient_equations_block_3(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, var_cqb, var_cqi, var_qfs, var_qfs_db0, var_qfs_db1, var_qfs_db10, var_qfs_db11, var_qfs_db2, var_qfs_db3, var_qfs_db4, var_qfs_db5, var_qfs_db6, var_qfs_db7, var_qfs_db8, var_qfs_db9, var_qfs_dn0, var_qfs_dn1, var_qfs_dn10, var_qfs_dn11, var_qfs_dn12, var_qfs_dn13, var_qfs_dn14, var_qfs_dn15, var_qfs_dn16, var_qfs_dn17, var_qfs_dn2, var_qfs_dn3, var_qfs_dn4, var_qfs_dn5, var_qfs_dn6, var_qfs_dn7, var_qfs_dn8, var_qfs_dn9);
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let scalar_temperature_static_temperature = (ctx).temperature();
        let scalar_temperature_static_thermal_voltage = (ctx).thermal_voltage();
        self.ensure_temperature_static(scalar_temperature_static_temperature, scalar_temperature_static_thermal_voltage);
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let nv4 = ctx.node_voltage(nodes[4]);
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let v337: f64 = nv4;
        let v408: f64 = (v337 * self.scalar_v376);

        let d408_dn4: f64 = self.scalar_v376;
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (d408_dn4),
        );
        let s = match &mut self.reactive_scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(ReactiveScratch::new_box()).as_mut(),
        };

        Self::stamp_reactive_block_0(s, param_given);
        Self::stamp_reactive_block_1(s, p);
        Self::stamp_reactive_block_2(s, p);
        Self::stamp_reactive_block_3(s, p);
        Self::stamp_reactive_block_4(ctx, s, p);
        Self::stamp_reactive_block_5(s, p);
        Self::stamp_reactive_block_6(s, p);
        Self::stamp_reactive_block_7(ctx, s, p, nodes);
        Self::stamp_reactive_block_8(s, p);
        Self::stamp_reactive_block_9(ctx, s, p);
        Self::stamp_reactive_block_10(s, p, param_given);
        Self::stamp_reactive_block_11(s, p);
        Self::stamp_reactive_block_12(s, p);
        Self::stamp_reactive_block_13(s, p);
        Self::stamp_reactive_block_14(s);
        Self::stamp_reactive_block_15(s);
        Self::stamp_reactive_block_16(s);
        Self::stamp_reactive_block_17(s);
        Self::stamp_reactive_block_18(s);
        Self::stamp_reactive_block_19(s);
        Self::stamp_reactive_block_20(s);
        Self::stamp_reactive_block_21(s);
        Self::stamp_reactive_block_22(s);
        Self::stamp_reactive_block_23(s);
        Self::stamp_reactive_block_24(s);
        Self::stamp_reactive_block_25(s);
        Self::stamp_reactive_block_26(s);
        Self::stamp_reactive_block_27(s, p);
        Self::stamp_reactive_block_28(s, p);
        Self::stamp_reactive_block_29(s, p);
        Self::stamp_reactive_block_30(s);
        Self::stamp_reactive_block_31(s, p);
        Self::stamp_reactive_block_32(s);
        Self::stamp_reactive_block_33(s);
        Self::stamp_reactive_block_34(s, p);
        Self::stamp_reactive_block_35(s, p);
        Self::stamp_reactive_block_36(s, p);
        Self::stamp_reactive_block_37(s, p);
        Self::stamp_reactive_block_38(s, p);
        Self::stamp_reactive_block_39(s, p, param_given);
        Self::stamp_reactive_block_40(s, p);
        Self::stamp_reactive_block_41(s, p);
        Self::stamp_reactive_block_42(s, p);
        Self::stamp_reactive_block_43(s, p);
        Self::stamp_reactive_block_44(s, p);
        Self::stamp_reactive_block_45(s, p);
        Self::stamp_reactive_block_46(s, p);
        Self::stamp_reactive_block_47(s, p);
        Self::stamp_reactive_block_48(s, p);
        Self::stamp_reactive_block_49(s, p);
        Self::stamp_reactive_block_50(s, p);
        Self::stamp_reactive_block_51(s, p);
        Self::stamp_reactive_block_52(s, p);
        Self::stamp_reactive_block_53(s, p, param_given);
        Self::stamp_reactive_block_54(s);
        Self::stamp_reactive_block_55(s);
        Self::stamp_reactive_block_56(s);
        Self::stamp_reactive_block_57(s, p);
        Self::stamp_reactive_block_58(s, p);
        Self::stamp_reactive_block_59(s, p);
        Self::stamp_reactive_block_60(s, p);
        Self::stamp_reactive_block_61(s, p);
        Self::stamp_reactive_block_62(s, p);
        Self::stamp_reactive_block_63(s, p);
        Self::stamp_reactive_block_64(s, p, param_given);
        Self::stamp_reactive_block_65(s, p);
        Self::stamp_reactive_block_66(s, p);
        Self::stamp_reactive_block_67(s, p);
        Self::stamp_reactive_block_68(s, p);
        Self::stamp_reactive_block_69(s, p, param_given);
        Self::stamp_reactive_block_70(s, p);
        Self::stamp_reactive_block_71(s, p);
        Self::stamp_reactive_block_72(s, p);
        Self::stamp_reactive_block_73(s, p);
        Self::stamp_reactive_block_74(s, p, param_given);
        Self::stamp_reactive_block_75(s, p);
        Self::stamp_reactive_block_76(s, p);
        Self::stamp_reactive_block_77(s, p);
        Self::stamp_reactive_block_78(s, p);
        Self::stamp_reactive_block_79(s, p, param_given);
        Self::stamp_reactive_block_80(s, p);
        Self::stamp_reactive_block_81(s, p);
        Self::stamp_reactive_block_82(s, p);
        Self::stamp_reactive_block_83(s, p);
        Self::stamp_reactive_block_84(s, p, param_given);
        Self::stamp_reactive_block_85(s, p);
        Self::stamp_reactive_block_86(s, p);
        Self::stamp_reactive_block_87(s, p);
        Self::stamp_reactive_block_88(s, p);
        Self::stamp_reactive_block_89(s, p);
        Self::stamp_reactive_block_90(ctx, s, p);
        Self::stamp_reactive_block_91(ctx, s, p, nodes);
        Self::stamp_reactive_block_92(ctx, s, p, nodes);
        Self::stamp_reactive_block_93(ctx, s, p, nodes);
        Self::stamp_reactive_block_94(ctx, s, p);
        Self::stamp_reactive_block_95(s, p);
        Self::stamp_reactive_block_96(s, p);

        Self::stamp_reactive_equations_block_0(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_1(ctx, stamper, s, p, nodes, branches, multiplicity);
    }
}

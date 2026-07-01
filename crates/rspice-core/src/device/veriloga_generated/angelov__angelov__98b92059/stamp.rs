#![allow(dead_code, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
#[path = "stamp_blocks_0.rs"]
mod stamp_blocks_0;

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

#[derive(Default)]
pub(crate) struct StampLocals {
    pub(crate) var_cgd: f64,
    pub(crate) var_cgd0_t: f64,
    pub(crate) var_cgd0_t_dn11: f64,
    pub(crate) var_cgd0_t_rv: f64,
    pub(crate) var_cgd_dn11: f64,
    pub(crate) var_cgd_dn3: f64,
    pub(crate) var_cgd_dn5: f64,
    pub(crate) var_cgd_dn7: f64,
    pub(crate) var_cgd_dn8: f64,
    pub(crate) var_cgd_rv: f64,
    pub(crate) var_cgs: f64,
    pub(crate) var_cgs0_t: f64,
    pub(crate) var_cgs0_t_dn11: f64,
    pub(crate) var_cgs0_t_rv: f64,
    pub(crate) var_cgs_dn11: f64,
    pub(crate) var_cgs_dn3: f64,
    pub(crate) var_cgs_dn5: f64,
    pub(crate) var_cgs_dn7: f64,
    pub(crate) var_cgs_dn8: f64,
    pub(crate) var_cgs_rv: f64,
    pub(crate) var_ci: f64,
    pub(crate) var_ci_dn11: f64,
    pub(crate) var_ci_rv: f64,
    pub(crate) var_cosh0: f64,
    pub(crate) var_cosh0_dn11: f64,
    pub(crate) var_cosh0_dn3: f64,
    pub(crate) var_cosh0_dn5: f64,
    pub(crate) var_cosh0_rv: f64,
    pub(crate) var_cosh1: f64,
    pub(crate) var_cosh1_dn11: f64,
    pub(crate) var_cosh1_dn3: f64,
    pub(crate) var_cosh1_dn5: f64,
    pub(crate) var_cosh1_dn7: f64,
    pub(crate) var_cosh1_dn8: f64,
    pub(crate) var_cosh1_rv: f64,
    pub(crate) var_delta_t: f64,
    pub(crate) var_delta_t_dn11: f64,
    pub(crate) var_delta_t_rv: f64,
    pub(crate) var_guard1: f64,
    pub(crate) var_guard10: f64,
    pub(crate) var_guard11: f64,
    pub(crate) var_guard13: f64,
    pub(crate) var_guard13_rv: f64,
    pub(crate) var_guard14: f64,
    pub(crate) var_guard14_rv: f64,
    pub(crate) var_guard15: f64,
    pub(crate) var_guard15_rv: f64,
    pub(crate) var_guard16: f64,
    pub(crate) var_guard16_rv: f64,
    pub(crate) var_guard1_rv: f64,
    pub(crate) var_guard2: f64,
    pub(crate) var_guard21: f64,
    pub(crate) var_guard21_rv: f64,
    pub(crate) var_guard22: f64,
    pub(crate) var_guard22_rv: f64,
    pub(crate) var_guard23: f64,
    pub(crate) var_guard24: f64,
    pub(crate) var_guard24_rv: f64,
    pub(crate) var_guard25: f64,
    pub(crate) var_guard25_rv: f64,
    pub(crate) var_guard26: f64,
    pub(crate) var_guard26_rv: f64,
    pub(crate) var_guard27: f64,
    pub(crate) var_guard27_rv: f64,
    pub(crate) var_guard2_rv: f64,
    pub(crate) var_guard3: f64,
    pub(crate) var_guard3_rv: f64,
    pub(crate) var_guard4: f64,
    pub(crate) var_guard43: f64,
    pub(crate) var_guard43_rv: f64,
    pub(crate) var_guard5: f64,
    pub(crate) var_guard6: f64,
    pub(crate) var_guard7: f64,
    pub(crate) var_guard8: f64,
    pub(crate) var_guard9: f64,
    pub(crate) var_k: f64,
    pub(crate) var_k_dn11: f64,
    pub(crate) var_k_rv: f64,
    pub(crate) var_lc1: f64,
    pub(crate) var_lc10: f64,
    pub(crate) var_lc10_dn11: f64,
    pub(crate) var_lc10_dn3: f64,
    pub(crate) var_lc10_dn5: f64,
    pub(crate) var_lc10_rv: f64,
    pub(crate) var_lc1_dn11: f64,
    pub(crate) var_lc1_dn3: f64,
    pub(crate) var_lc1_dn5: f64,
    pub(crate) var_lc1_dn7: f64,
    pub(crate) var_lc1_dn8: f64,
    pub(crate) var_lc1_rv: f64,
    pub(crate) var_lc4: f64,
    pub(crate) var_lc40: f64,
    pub(crate) var_lc40_dn11: f64,
    pub(crate) var_lc40_dn3: f64,
    pub(crate) var_lc40_dn5: f64,
    pub(crate) var_lc40_rv: f64,
    pub(crate) var_lc4_dn11: f64,
    pub(crate) var_lc4_dn3: f64,
    pub(crate) var_lc4_dn5: f64,
    pub(crate) var_lc4_dn7: f64,
    pub(crate) var_lc4_dn8: f64,
    pub(crate) var_lc4_rv: f64,
    pub(crate) var_p10_t: f64,
    pub(crate) var_p10_t_dn11: f64,
    pub(crate) var_p10_t_rv: f64,
    pub(crate) var_p1_t: f64,
    pub(crate) var_p1_t_dn11: f64,
    pub(crate) var_p1m: f64,
    pub(crate) var_p1m_dn11: f64,
    pub(crate) var_p1m_dn3: f64,
    pub(crate) var_p1m_dn4: f64,
    pub(crate) var_p1m_dn5: f64,
    pub(crate) var_p1m_dn8: f64,
    pub(crate) var_p40_t: f64,
    pub(crate) var_p40_t_dn11: f64,
    pub(crate) var_p40_t_rv: f64,
    pub(crate) var_pg_param: f64,
    pub(crate) var_pg_param_dn11: f64,
    pub(crate) var_psi: f64,
    pub(crate) var_psi_1: f64,
    pub(crate) var_psi_1_dn11: f64,
    pub(crate) var_psi_1_dn3: f64,
    pub(crate) var_psi_1_dn5: f64,
    pub(crate) var_psi_1_dn8: f64,
    pub(crate) var_psi_1_rv: f64,
    pub(crate) var_psi_2: f64,
    pub(crate) var_psi_2_dn3: f64,
    pub(crate) var_psi_2_dn5: f64,
    pub(crate) var_psi_2_rv: f64,
    pub(crate) var_psi_3: f64,
    pub(crate) var_psi_3_dn3: f64,
    pub(crate) var_psi_3_dn5: f64,
    pub(crate) var_psi_3_rv: f64,
    pub(crate) var_psi_4: f64,
    pub(crate) var_psi_4_dn11: f64,
    pub(crate) var_psi_4_dn3: f64,
    pub(crate) var_psi_4_dn5: f64,
    pub(crate) var_psi_4_dn7: f64,
    pub(crate) var_psi_4_rv: f64,
    pub(crate) var_psi_dn11: f64,
    pub(crate) var_psi_dn3: f64,
    pub(crate) var_psi_dn4: f64,
    pub(crate) var_psi_dn5: f64,
    pub(crate) var_psi_dn8: f64,
    pub(crate) var_qgd: f64,
    pub(crate) var_qgd0: f64,
    pub(crate) var_qgd0_dn11: f64,
    pub(crate) var_qgd0_dn3: f64,
    pub(crate) var_qgd0_dn5: f64,
    pub(crate) var_qgd0_rv: f64,
    pub(crate) var_qgd_dn11: f64,
    pub(crate) var_qgd_dn3: f64,
    pub(crate) var_qgd_dn5: f64,
    pub(crate) var_qgd_dn7: f64,
    pub(crate) var_qgd_dn8: f64,
    pub(crate) var_qgd_rv: f64,
    pub(crate) var_qgs: f64,
    pub(crate) var_qgs0: f64,
    pub(crate) var_qgs0_dn11: f64,
    pub(crate) var_qgs0_dn3: f64,
    pub(crate) var_qgs0_dn5: f64,
    pub(crate) var_qgs0_rv: f64,
    pub(crate) var_qgs_dn11: f64,
    pub(crate) var_qgs_dn3: f64,
    pub(crate) var_qgs_dn5: f64,
    pub(crate) var_qgs_dn7: f64,
    pub(crate) var_qgs_dn8: f64,
    pub(crate) var_qgs_rv: f64,
    pub(crate) var_rd1: f64,
    pub(crate) var_rd1_dn11: f64,
    pub(crate) var_rd1_dn3: f64,
    pub(crate) var_rd1_dn4: f64,
    pub(crate) var_rd1_dn5: f64,
    pub(crate) var_rd1_dn8: f64,
    pub(crate) var_rd1_t: f64,
    pub(crate) var_rd1_t_dn11: f64,
    pub(crate) var_rd1_t_dn3: f64,
    pub(crate) var_rd1_t_dn4: f64,
    pub(crate) var_rd1_t_dn5: f64,
    pub(crate) var_rd1_t_dn8: f64,
    pub(crate) var_rs1: f64,
    pub(crate) var_rs1_dn11: f64,
    pub(crate) var_rs1_dn3: f64,
    pub(crate) var_rs1_dn4: f64,
    pub(crate) var_rs1_dn5: f64,
    pub(crate) var_rs1_dn8: f64,
    pub(crate) var_rs_t: f64,
    pub(crate) var_rs_t_dn11: f64,
    pub(crate) var_rs_t_dn3: f64,
    pub(crate) var_rs_t_dn4: f64,
    pub(crate) var_rs_t_dn5: f64,
    pub(crate) var_rs_t_dn8: f64,
    pub(crate) var_t: f64,
    pub(crate) var_t0: f64,
    pub(crate) var_t0_dn11: f64,
    pub(crate) var_t0_dn3: f64,
    pub(crate) var_t0_dn4: f64,
    pub(crate) var_t0_dn5: f64,
    pub(crate) var_t0_dn8: f64,
    pub(crate) var_t1: f64,
    pub(crate) var_t1_dn11: f64,
    pub(crate) var_t1_dn3: f64,
    pub(crate) var_t1_dn4: f64,
    pub(crate) var_t1_dn5: f64,
    pub(crate) var_t1_dn8: f64,
    pub(crate) var_t2: f64,
    pub(crate) var_t2_dn11: f64,
    pub(crate) var_t2_dn3: f64,
    pub(crate) var_t2_dn4: f64,
    pub(crate) var_t2_dn5: f64,
    pub(crate) var_t2_dn8: f64,
    pub(crate) var_t_dn11: f64,
    pub(crate) var_t_nom: f64,
    pub(crate) var_t_nom_rv: f64,
    pub(crate) var_t_rv: f64,
    pub(crate) var_tanh1: f64,
    pub(crate) var_tanh1_dn11: f64,
    pub(crate) var_tanh1_dn3: f64,
    pub(crate) var_tanh1_dn5: f64,
    pub(crate) var_tanh1_dn8: f64,
    pub(crate) var_tanh1_rv: f64,
    pub(crate) var_tanh2: f64,
    pub(crate) var_tanh2_dn3: f64,
    pub(crate) var_tanh2_dn5: f64,
    pub(crate) var_tanh2_rv: f64,
    pub(crate) var_tanh3: f64,
    pub(crate) var_tanh3_dn3: f64,
    pub(crate) var_tanh3_dn5: f64,
    pub(crate) var_tanh3_rv: f64,
    pub(crate) var_tanh4: f64,
    pub(crate) var_tanh4_dn11: f64,
    pub(crate) var_tanh4_dn3: f64,
    pub(crate) var_tanh4_dn5: f64,
    pub(crate) var_tanh4_dn7: f64,
    pub(crate) var_tanh4_rv: f64,
    pub(crate) var_tanh_psi: f64,
    pub(crate) var_tanh_psi1: f64,
    pub(crate) var_tanh_psi1_dn11: f64,
    pub(crate) var_tanh_psi1_dn3: f64,
    pub(crate) var_tanh_psi1_dn4: f64,
    pub(crate) var_tanh_psi1_dn5: f64,
    pub(crate) var_tanh_psi1_dn8: f64,
    pub(crate) var_tanh_psi_dn11: f64,
    pub(crate) var_tanh_psi_dn3: f64,
    pub(crate) var_tanh_psi_dn4: f64,
    pub(crate) var_tanh_psi_dn5: f64,
    pub(crate) var_tanh_psi_dn8: f64,
    pub(crate) var_vdg: f64,
    pub(crate) var_vdg_dn3: f64,
    pub(crate) var_vdg_dn4: f64,
    pub(crate) var_vds: f64,
    pub(crate) var_vds_dn3: f64,
    pub(crate) var_vds_dn5: f64,
    pub(crate) var_vds_rv: f64,
    pub(crate) var_vgd: f64,
    pub(crate) var_vgd_dn3: f64,
    pub(crate) var_vgd_dn4: f64,
    pub(crate) var_vgdc: f64,
    pub(crate) var_vgdc_dn3: f64,
    pub(crate) var_vgdc_dn7: f64,
    pub(crate) var_vgdc_rv: f64,
    pub(crate) var_vgs: f64,
    pub(crate) var_vgs_dn5: f64,
    pub(crate) var_vgs_dn8: f64,
    pub(crate) var_vgs_rv: f64,
    pub(crate) var_vgsc: f64,
    pub(crate) var_vgsc_dn5: f64,
    pub(crate) var_vgsc_dn8: f64,
    pub(crate) var_vgsc_rv: f64,
    pub(crate) var_vjg_t: f64,
    pub(crate) var_vjg_t_dn11: f64,
    pub(crate) var_vpkm: f64,
    pub(crate) var_vpkm_dn11: f64,
    pub(crate) var_vpkm_dn3: f64,
    pub(crate) var_vpkm_dn4: f64,
    pub(crate) var_vpkm_dn5: f64,
    pub(crate) var_vpks_t: f64,
    pub(crate) var_vpks_t_dn11: f64,
    pub(crate) var_vth: f64,
    pub(crate) var_vth_dn11: f64,
    pub(crate) var_vtr_t: f64,
    pub(crate) var_vtr_t_dn11: f64,
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
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
        let v0: f64 = nv8;
        let v1: f64 = nv5;
        let v2: f64 = (v0 - v1);
        let v3: f64 = nv4;
        let v4: f64 = nv3;
        let v5: f64 = (v3 - v4);
        let v6: f64 = (-v5);
        let v7: f64 = (v4 - v1);
        let v8: f64 = nv7;
        let v9: f64 = (v8 - v4);
        let v10: f64 = nv13;
        let v11: f64 = 0.0;
        let v30: f64 = nv11;
        let v31: f64 = ((v30) as f64).abs();
        let v32: f64 = (self.scalar_v21 + v31);
        let v33: f64 = (if (self.scalar_v29 != 0.0) { v32 } else { self.scalar_v21 });
        let v34: f64 = 8.617333262145179e-5;
        let v35: f64 = (v33 * v34);
        let v36: f64 = (v33 - self.scalar_v28);
        let v37: f64 = ((v36) as f64).abs();
        let v38: bool = (v37 > v11);
        let v41: bool = (v38 || self.scalar_v40);
        let v42: f64 = 1.0;
        let v45: f64 = (v37 * self.scalar_v44);
        let v46: f64 = (v42 + v45);
        let v47: f64 = (self.scalar_v43 * v46);
        let v48: f64 = (if v41 { v47 } else { v11 });
        let v51: f64 = (v37 * self.scalar_v50);
        let v52: f64 = (v42 + v51);
        let v53: f64 = (self.scalar_v49 * v52);
        let v54: f64 = (if v41 { v53 } else { v11 });
        let v57: f64 = (v37 * self.scalar_v56);
        let v58: f64 = (v42 + v57);
        let v59: f64 = (self.scalar_v55 * v58);
        let v60: f64 = (if v41 { v59 } else { v11 });
        let v63: f64 = (v37 * self.scalar_v62);
        let v64: f64 = (v42 + v63);
        let v65: f64 = (self.scalar_v61 * v64);
        let v66: f64 = (if v41 { v65 } else { v11 });
        let v75: f64 = (v37 * self.scalar_v74);
        let v76: f64 = (v42 + v75);
        let v77: f64 = (self.scalar_v73 * v76);
        let v78: f64 = (if v41 { v77 } else { v11 });
        let v81: f64 = (v37 * self.scalar_v80);
        let v82: f64 = (v42 + v81);
        let v83: f64 = (self.scalar_v79 * v82);
        let v84: f64 = (if v41 { v83 } else { v11 });
        let v87: f64 = (v37 * self.scalar_v86);
        let v88: f64 = (self.scalar_v85 + v87);
        let v89: f64 = (if v41 { v88 } else { v11 });
        let v104: f64 = (v37 * self.scalar_v103);
        let v105: f64 = (self.scalar_v102 + v104);
        let v106: f64 = (if v41 { v105 } else { v11 });
        let v109: f64 = (v37 * self.scalar_v108);
        let v110: f64 = (self.scalar_v107 + v109);
        let v111: f64 = (if v41 { v110 } else { v11 });
        let v112: bool = (!v41);
        let v113: f64 = (if v112 { self.scalar_v43 } else { v48 });
        let v114: f64 = (if v112 { self.scalar_v49 } else { v54 });
        let v115: f64 = (if v112 { self.scalar_v55 } else { v60 });
        let v116: f64 = (if v112 { self.scalar_v61 } else { v66 });
        let v118: f64 = (if v112 { self.scalar_v73 } else { v78 });
        let v119: f64 = (if v112 { self.scalar_v79 } else { v84 });
        let v120: f64 = (if v112 { self.scalar_v85 } else { v89 });
        let v123: f64 = (if v112 { self.scalar_v102 } else { v106 });
        let v124: f64 = (if v112 { self.scalar_v107 } else { v111 });
        let v129: f64 = 0.5;
        let v132: f64 = (self.scalar_v131 / v35);
        let v133: f64 = (if self.scalar_v128 { v132 } else { v11 });
        let v136: f64 = (if self.scalar_v134 { self.scalar_v135 } else { v133 });
        let v138: f64 = (v7 * self.scalar_v137);
        let v139: f64 = ((v138) as f64).cosh();
        let v141: f64 = (v139 * v139);
        let v142: f64 = (self.scalar_v140 / v141);
        let v143: f64 = (v42 + v142);
        let v144: f64 = (v114 * v143);
        let v146: f64 = (v120 - self.scalar_v145);
        let v148: f64 = (v7 * self.scalar_v147);
        let v149: f64 = ((v148) as f64).tanh();
        let v150: f64 = (self.scalar_v145 * v149);
        let v151: f64 = (v146 + v150);
        let v153: f64 = (v6 - self.scalar_v107);
        let v154: f64 = (self.scalar_v152 * v153);
        let v155: f64 = (v6 - v124);
        let v156: f64 = (v154 * v155);
        let v157: f64 = (v151 - v156);
        let v158: f64 = (v2 - v157);
        let v159: f64 = (v158 * v158);
        let v160: f64 = (v144 * v158);
        let v162: f64 = (v159 * self.scalar_v161);
        let v163: f64 = (v160 + v162);
        let v165: f64 = (v158 * self.scalar_v164);
        let v166: f64 = (v159 * v165);
        let v167: f64 = (v163 + v166);
        let v168: f64 = ((v167) as f64).tanh();
        let v169: f64 = (v42 + v168);
        let v170: f64 = { let limexp_arg = v167; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v171: f64 = (-v167);
        let v172: f64 = { let limexp_arg = v171; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v173: f64 = (v170 - v172);
        let v174: f64 = (v129 * v173);
        let v175: f64 = ((v174) as f64).tanh();
        let v176: f64 = (v42 + v175);
        let v178: f64 = (self.scalar_v147 * v169);
        let v179: f64 = (self.scalar_v177 + v178);
        let v180: f64 = (v7 * v179);
        let v181: f64 = ((v180) as f64).tanh();
        let v189: f64 = (v113 * v169);
        let v190: f64 = (v181 * v189);
        let v192: f64 = (v7 * self.scalar_v191);
        let v193: f64 = (v42 + v192);
        let v194: f64 = { let limexp_arg = v155; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v195: f64 = (v115 * v194);
        let v196: f64 = (v193 + v195);
        let v197: f64 = (v190 * v196);
        let v198: f64 = (if self.scalar_v183 { v197 } else { v11 });
        let v201: f64 = (v5 - v157);
        let v202: f64 = (if self.scalar_v200 { v201 } else { v139 });
        let v203: f64 = (v202 * v202);
        let v204: f64 = (if self.scalar_v200 { v203 } else { v158 });
        let v205: f64 = (v202 * v204);
        let v206: f64 = (if self.scalar_v200 { v205 } else { v159 });
        let v207: f64 = (v144 * v202);
        let v208: f64 = (self.scalar_v161 * v204);
        let v209: f64 = (v207 + v208);
        let v210: f64 = (self.scalar_v164 * v206);
        let v211: f64 = (v209 + v210);
        let v212: f64 = (if self.scalar_v200 { v211 } else { v11 });
        let v213: f64 = ((v212) as f64).tanh();
        let v214: f64 = (v42 + v213);
        let v215: f64 = (if self.scalar_v200 { v214 } else { v11 });
        let v216: f64 = (self.scalar_v147 * v215);
        let v217: f64 = (self.scalar_v177 + v216);
        let v218: f64 = (if self.scalar_v200 { v217 } else { v11 });
        let v220: f64 = (v169 * self.scalar_v219);
        let v221: f64 = (self.scalar_v191 + v220);
        let v222: f64 = (if self.scalar_v200 { v221 } else { v11 });
        let v223: f64 = (v42 + v181);
        let v224: f64 = (v189 * v223);
        let v225: f64 = (v7 * v222);
        let v226: f64 = (v42 + v225);
        let v227: f64 = (v7 - v124);
        let v228: f64 = { let limexp_arg = v227; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v229: f64 = (v115 * v228);
        let v230: f64 = (v226 + v229);
        let v231: f64 = (v224 * v230);
        let v232: f64 = (if self.scalar_v200 { v231 } else { v11 });
        let v233: f64 = (v215 * self.scalar_v219);
        let v234: f64 = (self.scalar_v191 + v233);
        let v235: f64 = (if self.scalar_v200 { v234 } else { v11 });
        let v236: f64 = (v7 * v218);
        let v237: f64 = ((v236) as f64).tanh();
        let v238: f64 = (if self.scalar_v200 { v237 } else { v11 });
        let v239: f64 = (v113 * v215);
        let v240: f64 = (v42 - v238);
        let v241: f64 = (v239 * v240);
        let v242: f64 = (v7 * v235);
        let v243: f64 = (v42 - v242);
        let v244: f64 = (v241 * v243);
        let v245: f64 = (if self.scalar_v200 { v244 } else { v11 });
        let v246: f64 = (v232 - v245);
        let v247: f64 = (v129 * v246);
        let v248: f64 = (if self.scalar_v200 { v247 } else { v198 });
        let v252: f64 = (if self.scalar_v251 { v158 } else { v202 });
        let v253: f64 = (v252 * v252);
        let v254: f64 = (if self.scalar_v251 { v253 } else { v204 });
        let v255: f64 = (self.scalar_v161 * v254);
        let v256: f64 = (v252 + v255);
        let v257: f64 = (self.scalar_v164 * v254);
        let v258: f64 = (v252 * v257);
        let v259: f64 = (v256 + v258);
        let v260: f64 = (v144 * v259);
        let v261: f64 = (if self.scalar_v251 { v260 } else { v167 });
        let v262: f64 = { let limexp_arg = v261; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v263: f64 = (-v261);
        let v264: f64 = { let limexp_arg = v263; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v265: f64 = (v262 - v264);
        let v266: f64 = (v129 * v265);
        let v267: f64 = ((v266) as f64).tanh();
        let v268: f64 = (v42 + v267);
        let v269: f64 = (if self.scalar_v251 { v268 } else { v176 });
        let v270: f64 = (self.scalar_v147 * v269);
        let v271: f64 = (self.scalar_v177 + v270);
        let v272: f64 = (if self.scalar_v251 { v271 } else { v11 });
        let v273: f64 = (v7 * v272);
        let v274: f64 = ((v273) as f64).tanh();
        let v275: f64 = (if self.scalar_v251 { v274 } else { v11 });
        let v276: f64 = (self.scalar_v219 * v269);
        let v277: f64 = (self.scalar_v191 + v276);
        let v278: f64 = (if self.scalar_v251 { v277 } else { v222 });
        let v279: f64 = (v113 * v269);
        let v280: f64 = (v275 * v279);
        let v281: f64 = (v7 * v278);
        let v282: f64 = (v42 + v281);
        let v283: f64 = (v195 + v282);
        let v284: f64 = (v280 * v283);
        let v285: f64 = (if self.scalar_v251 { v284 } else { v248 });
        let v289: f64 = (if self.scalar_v288 { v158 } else { v252 });
        let v290: f64 = (v289 * v289);
        let v291: f64 = (if self.scalar_v288 { v290 } else { v254 });
        let v292: f64 = (self.scalar_v161 * v291);
        let v293: f64 = (v289 + v292);
        let v294: f64 = (self.scalar_v164 * v291);
        let v295: f64 = (v289 * v294);
        let v296: f64 = (v293 + v295);
        let v297: f64 = (v144 * v296);
        let v298: f64 = (if self.scalar_v288 { v297 } else { v261 });
        let v299: f64 = (if self.scalar_v288 { v201 } else { v206 });
        let v300: f64 = (v299 * v299);
        let v301: f64 = (if self.scalar_v288 { v300 } else { v11 });
        let v302: f64 = (self.scalar_v161 * v301);
        let v303: f64 = (v299 + v302);
        let v304: f64 = (self.scalar_v164 * v299);
        let v305: f64 = (v301 * v304);
        let v306: f64 = (v303 + v305);
        let v307: f64 = (v144 * v306);
        let v308: f64 = (if self.scalar_v288 { v307 } else { v212 });
        let v309: f64 = { let limexp_arg = v298; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v310: f64 = (-v298);
        let v311: f64 = { let limexp_arg = v310; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v312: f64 = (v309 - v311);
        let v313: f64 = (v129 * v312);
        let v314: f64 = ((v313) as f64).tanh();
        let v315: f64 = (v42 + v314);
        let v316: f64 = (if self.scalar_v288 { v315 } else { v269 });
        let v317: f64 = { let limexp_arg = v308; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v318: f64 = (-v308);
        let v319: f64 = { let limexp_arg = v318; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v320: f64 = (v317 - v319);
        let v321: f64 = (v129 * v320);
        let v322: f64 = ((v321) as f64).tanh();
        let v323: f64 = (v42 + v322);
        let v324: f64 = (if self.scalar_v288 { v323 } else { v11 });
        let v325: f64 = (self.scalar_v147 * v316);
        let v326: f64 = (self.scalar_v177 + v325);
        let v327: f64 = (if self.scalar_v288 { v326 } else { v272 });
        let v328: f64 = (self.scalar_v147 * v324);
        let v329: f64 = (self.scalar_v177 + v328);
        let v330: f64 = (if self.scalar_v288 { v329 } else { v11 });
        let v331: f64 = (v7 * v327);
        let v332: f64 = ((v331) as f64).tanh();
        let v333: f64 = (if self.scalar_v288 { v332 } else { v275 });
        let v334: f64 = (v7 * v330);
        let v335: f64 = ((v334) as f64).tanh();
        let v336: f64 = (if self.scalar_v288 { v335 } else { v11 });
        let v337: f64 = (self.scalar_v219 * v324);
        let v338: f64 = (self.scalar_v191 + v337);
        let v339: f64 = (if self.scalar_v288 { v338 } else { v11 });
        let v340: f64 = (self.scalar_v219 * v316);
        let v341: f64 = (self.scalar_v191 + v340);
        let v342: f64 = (if self.scalar_v288 { v341 } else { v11 });
        let v343: f64 = (v113 * v316);
        let v344: f64 = (v42 + v333);
        let v345: f64 = (v343 * v344);
        let v346: f64 = (v7 * v342);
        let v347: f64 = (v42 + v346);
        let v348: f64 = (v229 + v347);
        let v349: f64 = (v345 * v348);
        let v350: f64 = (if self.scalar_v288 { v349 } else { v232 });
        let v351: f64 = (v113 * v324);
        let v352: f64 = (v42 - v336);
        let v353: f64 = (v351 * v352);
        let v354: f64 = (v7 * v339);
        let v355: f64 = (v42 - v354);
        let v356: f64 = (v353 * v355);
        let v357: f64 = (if self.scalar_v288 { v356 } else { v245 });
        let v358: f64 = (v350 - v357);
        let v359: f64 = (v129 * v358);
        let v360: f64 = (if self.scalar_v288 { v359 } else { v285 });
        let v362: f64 = (v42 + v169);
        let v363: f64 = (v118 / v362);
        let v364: f64 = (self.scalar_v361 + v363);
        let v365: f64 = (if self.scalar_v249 { v364 } else { v11 });
        let v369: f64 = (v42 + v316);
        let v370: f64 = (v118 / v369);
        let v371: f64 = (self.scalar_v361 + v370);
        let v372: f64 = (if self.scalar_v250 { v371 } else { v365 });
        let v375: f64 = -1.0;
        let v376: f64 = (-v123);
        let v377: f64 = ((v376) as f64).tanh();
        let v378: f64 = (v136 * v377);
        let v379: f64 = { let limexp_arg = v378; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v380: f64 = (if self.scalar_v374 { v379 } else { v289 });
        let v381: f64 = (v2 - v123);
        let v382: f64 = (if self.scalar_v374 { v381 } else { v11 });
        let v383: f64 = (v9 - v123);
        let v384: f64 = (if self.scalar_v374 { v383 } else { v11 });
        let v386: f64 = (-v136);
        let v387: f64 = (v123 * v386);
        let v388: f64 = { let limexp_arg = v387; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v389: f64 = (if self.scalar_v385 { v388 } else { v380 });
        let v392: f64 = ((v381) as f64).tanh();
        let v393: f64 = (if self.scalar_v391 { v392 } else { v382 });
        let v394: f64 = ((v383) as f64).tanh();
        let v395: f64 = (if self.scalar_v391 { v394 } else { v384 });
        let v398: f64 = (if self.scalar_v397 { v381 } else { v393 });
        let v399: f64 = (if self.scalar_v397 { v383 } else { v395 });
        let v401: f64 = (v136 * v398);
        let v402: f64 = { let limexp_arg = v401; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v403: f64 = (v402 - v389);
        let v404: f64 = (self.scalar_v400 * v403);
        let v405: f64 = (v136 * v399);
        let v406: f64 = { let limexp_arg = v405; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v407: f64 = (v406 - v389);
        let v408: f64 = (self.scalar_v400 * v407);
        let v529: f64 = 5.5226012e-23;
        let v530: f64 = (v33 * v529);
        let v534: f64 = (v530 * self.scalar_v533);
        let v535: f64 = (v116 * v534);
        let v538: f64 = (v535 * self.scalar_v537);
        let v539: f64 = (if self.scalar_v528 { v538 } else { v11 });
        let v540: f64 = (v539 * v539);
        let v541: f64 = (v42 - v540);
        let v542: f64 = ((v541) as f64).sqrt();
        let v543: f64 = (if self.scalar_v528 { v542 } else { v11 });
        let v544: f64 = (-v539);
        let v545: f64 = 3.141592653589793;
        let v546: f64 = (v544 * v545);
        let v547: f64 = (if self.scalar_v528 { v546 } else { v11 });
        let v553: f64 = (-v360);
        let v555: f64 = nv12;
        let v556: f64 = (self.scalar_v554 * v555);
        let v569: f64 = nv1;
        let v570: f64 = (v569 - v4);
        let v571: f64 = (self.scalar_v568 * v570);
        let v573: f64 = (v7 * self.scalar_v572);
        let v574: f64 = nv10;
        let v575: f64 = (v4 - v574);
        let v576: f64 = (v119 * v575);
        let v577: f64 = (v574 - v1);
        let v578: f64 = (v577 / v372);
        let v579: f64 = (if self.scalar_v505 { v578 } else { v11 });
        let v583: f64 = nv9;
        let v584: f64 = (v583 - v0);
        let v585: f64 = (self.scalar_v582 * v584);
        let v586: f64 = (v583 - v1);
        let v587: f64 = (v586 / self.scalar_v506);
        let v588: f64 = (if self.scalar_v507 { v587 } else { v11 });
        let v591: f64 = (v3 - v8);
        let v592: f64 = (v591 / self.scalar_v508);
        let v593: f64 = (if self.scalar_v509 { v592 } else { v11 });
        let v598: f64 = (v3 - v0);
        let v599: f64 = (v598 / self.scalar_v510);
        let v600: f64 = (if self.scalar_v511 { v599 } else { v11 });
        let v622: f64 = nv14;
        let v623: f64 = (if self.scalar_v528 { v622 } else { v11 });
        let v624: f64 = nv15;
        let v625: f64 = (if self.scalar_v528 { v624 } else { v11 });
        let v626: f64 = (v547 * v622);
        let v627: f64 = (v543 * v624);
        let v628: f64 = (v626 + v627);
        let v629: f64 = (if self.scalar_v528 { v628 } else { v11 });
        let v643: f64 = (-v10);
        let v644: f64 = (v7 * v643);
        let v645: f64 = (v2 * v404);
        let v646: f64 = (v644 + v645);
        let v647: f64 = ((v646) as f64).abs();
        let v648: f64 = (-v647);
        let v649: f64 = (if self.scalar_v552 { v648 } else { v11 });
        let v650: f64 = (v30 / self.scalar_v39);
        let v651: f64 = (if self.scalar_v552 { v650 } else { v11 });
        let v653: f64 = 1e-12;
        let v654: f64 = (v30 * v653);
        let v655: f64 = (if self.scalar_v652 { v654 } else { v11 });
        let v657: f64 = ((v138) as f64).sinh();
        let v658: f64 = (self.scalar_v137 * v657);
        let v659: f64 = (self.scalar_v656 * v657);
        let v660: f64 = (v139 * v658);
        let v661: f64 = (v660 + v660);
        let v662: f64 = (v139 * v659);
        let v663: f64 = (v662 + v662);
        let v664: f64 = (self.scalar_v140 * v661);
        let v665: f64 = (-v664);
        let v666: f64 = (v141 * v141);
        let v667: f64 = (v665 / v666);
        let v668: f64 = (self.scalar_v140 * v663);
        let v669: f64 = (-v668);
        let v670: f64 = (v669 / v666);
        let v671: f64 = (v114 * v667);
        let v672: f64 = (v114 * v670);
        let v674: f64 = (v149 * v149);
        let v675: f64 = (v42 - v674);
        let v676: f64 = (self.scalar_v147 * v675);
        let v677: f64 = (self.scalar_v673 * v675);
        let v678: f64 = (self.scalar_v145 * v676);
        let v679: f64 = (self.scalar_v145 * v677);
        let v681: f64 = (self.scalar_v152 * v155);
        let v682: f64 = (v154 + v681);
        let v683: f64 = (v155 * self.scalar_v680);
        let v684: f64 = (-v154);
        let v685: f64 = (v683 + v684);
        let v686: f64 = (v678 - v682);
        let v687: f64 = (-v685);
        let v688: f64 = (-v686);
        let v689: f64 = (v375 - v679);
        let v690: f64 = (v158 * v688);
        let v691: f64 = (v690 + v690);
        let v692: f64 = (v158 * v685);
        let v693: f64 = (v692 + v692);
        let v694: f64 = (v158 * v689);
        let v695: f64 = (v694 + v694);
        let v696: f64 = (v158 + v158);
        let v697: f64 = (v158 * v671);
        let v698: f64 = (v144 * v688);
        let v699: f64 = (v697 + v698);
        let v700: f64 = (v144 * v685);
        let v701: f64 = (v158 * v672);
        let v702: f64 = (v144 * v689);
        let v703: f64 = (v701 + v702);
        let v704: f64 = (self.scalar_v161 * v691);
        let v705: f64 = (self.scalar_v161 * v693);
        let v706: f64 = (self.scalar_v161 * v695);
        let v707: f64 = (self.scalar_v161 * v696);
        let v708: f64 = (v699 + v704);
        let v709: f64 = (v700 + v705);
        let v710: f64 = (v703 + v706);
        let v711: f64 = (v144 + v707);
        let v712: f64 = (self.scalar_v164 * v688);
        let v713: f64 = (self.scalar_v164 * v685);
        let v714: f64 = (self.scalar_v164 * v689);
        let v715: f64 = (v165 * v691);
        let v716: f64 = (v159 * v712);
        let v717: f64 = (v715 + v716);
        let v718: f64 = (v165 * v693);
        let v719: f64 = (v159 * v713);
        let v720: f64 = (v718 + v719);
        let v721: f64 = (v165 * v695);
        let v722: f64 = (v159 * v714);
        let v723: f64 = (v721 + v722);
        let v724: f64 = (v165 * v696);
        let v725: f64 = (v159 * self.scalar_v164);
        let v726: f64 = (v724 + v725);
        let v727: f64 = (v708 + v717);
        let v728: f64 = (v709 + v720);
        let v729: f64 = (v710 + v723);
        let v730: f64 = (v711 + v726);
        let v731: f64 = (v168 * v168);
        let v732: f64 = (v42 - v731);
        let v733: f64 = (v727 * v732);
        let v734: f64 = (v728 * v732);
        let v735: f64 = (v729 * v732);
        let v736: f64 = (v730 * v732);
        let v737: f64 = { let limexp_arg = v167; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v738: f64 = (v727 * v737);
        let v739: f64 = (v728 * v737);
        let v740: f64 = (v729 * v737);
        let v741: f64 = (v730 * v737);
        let v742: f64 = (-v727);
        let v743: f64 = (-v728);
        let v744: f64 = (-v729);
        let v745: f64 = (-v730);
        let v746: f64 = { let limexp_arg = v171; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v747: f64 = (v742 * v746);
        let v748: f64 = (v743 * v746);
        let v749: f64 = (v744 * v746);
        let v750: f64 = (v745 * v746);
        let v751: f64 = (v738 - v747);
        let v752: f64 = (v739 - v748);
        let v753: f64 = (v740 - v749);
        let v754: f64 = (v741 - v750);
        let v755: f64 = (v129 * v751);
        let v756: f64 = (v129 * v752);
        let v757: f64 = (v129 * v753);
        let v758: f64 = (v129 * v754);
        let v759: f64 = (v175 * v175);
        let v760: f64 = (v42 - v759);
        let v761: f64 = (v755 * v760);
        let v762: f64 = (v756 * v760);
        let v763: f64 = (v757 * v760);
        let v764: f64 = (v758 * v760);
        let v765: f64 = (self.scalar_v147 * v733);
        let v766: f64 = (self.scalar_v147 * v734);
        let v767: f64 = (self.scalar_v147 * v735);
        let v768: f64 = (self.scalar_v147 * v736);
        let v769: f64 = (v7 * v765);
        let v770: f64 = (v179 + v769);
        let v771: f64 = (v7 * v766);
        let v772: f64 = (-v179);
        let v773: f64 = (v7 * v767);
        let v774: f64 = (v772 + v773);
        let v775: f64 = (v7 * v768);
        let v776: f64 = (v181 * v181);
        let v777: f64 = (v42 - v776);
        let v778: f64 = (v770 * v777);
        let v779: f64 = (v771 * v777);
        let v780: f64 = (v774 * v777);
        let v781: f64 = (v775 * v777);
        let v782: f64 = (v113 * v733);
        let v783: f64 = (v113 * v734);
        let v784: f64 = (v113 * v735);
        let v785: f64 = (v113 * v736);
        let v786: f64 = (v189 * v778);
        let v787: f64 = (v181 * v782);
        let v788: f64 = (v786 + v787);
        let v789: f64 = (v189 * v779);
        let v790: f64 = (v181 * v783);
        let v791: f64 = (v789 + v790);
        let v792: f64 = (v189 * v780);
        let v793: f64 = (v181 * v784);
        let v794: f64 = (v792 + v793);
        let v795: f64 = (v189 * v781);
        let v796: f64 = (v181 * v785);
        let v797: f64 = (v795 + v796);
        let v799: f64 = { let limexp_arg = v155; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v800: f64 = (-v799);
        let v801: f64 = (v115 * v799);
        let v802: f64 = (v115 * v800);
        let v803: f64 = (self.scalar_v191 + v801);
        let v804: f64 = (v196 * v788);
        let v805: f64 = (v190 * v803);
        let v806: f64 = (v804 + v805);
        let v807: f64 = (v196 * v791);
        let v808: f64 = (v190 * v802);
        let v809: f64 = (v807 + v808);
        let v810: f64 = (v196 * v794);
        let v811: f64 = (v190 * self.scalar_v798);
        let v812: f64 = (v810 + v811);
        let v813: f64 = (v196 * v797);
        let v814: f64 = (if self.scalar_v183 { v806 } else { v11 });
        let v815: f64 = (if self.scalar_v183 { v809 } else { v11 });
        let v816: f64 = (if self.scalar_v183 { v812 } else { v11 });
        let v817: f64 = (if self.scalar_v183 { v813 } else { v11 });
        let v818: f64 = (v375 - v686);
        let v819: f64 = (v42 - v687);
        let v820: f64 = (-v679);
        let v821: f64 = (if self.scalar_v200 { v818 } else { v658 });
        let v822: f64 = (if self.scalar_v200 { v819 } else { v11 });
        let v823: f64 = (if self.scalar_v200 { v820 } else { v659 });
        let v824: f64 = (v202 * v821);
        let v825: f64 = (v824 + v824);
        let v826: f64 = (v202 * v822);
        let v827: f64 = (v826 + v826);
        let v828: f64 = (v202 * v823);
        let v829: f64 = (v828 + v828);
        let v830: f64 = (if self.scalar_v200 { v825 } else { v688 });
        let v831: f64 = (if self.scalar_v200 { v827 } else { v685 });
        let v832: f64 = (if self.scalar_v200 { v829 } else { v689 });
        let v834: f64 = (v204 * v821);
        let v835: f64 = (v202 * v830);
        let v836: f64 = (v834 + v835);
        let v837: f64 = (v204 * v822);
        let v838: f64 = (v202 * v831);
        let v839: f64 = (v837 + v838);
        let v840: f64 = (v204 * v823);
        let v841: f64 = (v202 * v832);
        let v842: f64 = (v840 + v841);
        let v843: f64 = (v202 * self.scalar_v833);
        let v844: f64 = (if self.scalar_v200 { v836 } else { v691 });
        let v845: f64 = (if self.scalar_v200 { v839 } else { v693 });
        let v846: f64 = (if self.scalar_v200 { v842 } else { v695 });
        let v847: f64 = (if self.scalar_v200 { v843 } else { v696 });
        let v848: f64 = (v202 * v671);
        let v849: f64 = (v144 * v821);
        let v850: f64 = (v848 + v849);
        let v851: f64 = (v144 * v822);
        let v852: f64 = (v202 * v672);
        let v853: f64 = (v144 * v823);
        let v854: f64 = (v852 + v853);
        let v855: f64 = (self.scalar_v161 * v830);
        let v856: f64 = (self.scalar_v161 * v831);
        let v857: f64 = (self.scalar_v161 * v832);
        let v859: f64 = (v850 + v855);
        let v860: f64 = (v851 + v856);
        let v861: f64 = (v854 + v857);
        let v862: f64 = (self.scalar_v164 * v844);
        let v863: f64 = (self.scalar_v164 * v845);
        let v864: f64 = (self.scalar_v164 * v846);
        let v865: f64 = (self.scalar_v164 * v847);
        let v866: f64 = (v859 + v862);
        let v867: f64 = (v860 + v863);
        let v868: f64 = (v861 + v864);
        let v869: f64 = (self.scalar_v858 + v865);
        let v870: f64 = (if self.scalar_v200 { v866 } else { v11 });
        let v871: f64 = (if self.scalar_v200 { v867 } else { v11 });
        let v872: f64 = (if self.scalar_v200 { v868 } else { v11 });
        let v873: f64 = (if self.scalar_v200 { v869 } else { v11 });
        let v874: f64 = (v213 * v213);
        let v875: f64 = (v42 - v874);
        let v876: f64 = (v870 * v875);
        let v877: f64 = (v871 * v875);
        let v878: f64 = (v872 * v875);
        let v879: f64 = (v873 * v875);
        let v880: f64 = (if self.scalar_v200 { v876 } else { v11 });
        let v881: f64 = (if self.scalar_v200 { v877 } else { v11 });
        let v882: f64 = (if self.scalar_v200 { v878 } else { v11 });
        let v883: f64 = (if self.scalar_v200 { v879 } else { v11 });
        let v884: f64 = (self.scalar_v147 * v880);
        let v885: f64 = (self.scalar_v147 * v881);
        let v886: f64 = (self.scalar_v147 * v882);
        let v887: f64 = (self.scalar_v147 * v883);
        let v888: f64 = (if self.scalar_v200 { v884 } else { v11 });
        let v889: f64 = (if self.scalar_v200 { v885 } else { v11 });
        let v890: f64 = (if self.scalar_v200 { v886 } else { v11 });
        let v891: f64 = (if self.scalar_v200 { v887 } else { v11 });
        let v892: f64 = (self.scalar_v219 * v733);
        let v893: f64 = (self.scalar_v219 * v734);
        let v894: f64 = (self.scalar_v219 * v735);
        let v895: f64 = (self.scalar_v219 * v736);
        let v896: f64 = (if self.scalar_v200 { v892 } else { v11 });
        let v897: f64 = (if self.scalar_v200 { v893 } else { v11 });
        let v898: f64 = (if self.scalar_v200 { v894 } else { v11 });
        let v899: f64 = (if self.scalar_v200 { v895 } else { v11 });
        let v900: f64 = (v223 * v782);
        let v901: f64 = (v786 + v900);
        let v902: f64 = (v223 * v783);
        let v903: f64 = (v789 + v902);
        let v904: f64 = (v223 * v784);
        let v905: f64 = (v792 + v904);
        let v906: f64 = (v223 * v785);
        let v907: f64 = (v795 + v906);
        let v908: f64 = (v7 * v896);
        let v909: f64 = (v222 + v908);
        let v910: f64 = (v7 * v897);
        let v911: f64 = (-v222);
        let v912: f64 = (v7 * v898);
        let v913: f64 = (v911 + v912);
        let v914: f64 = (v7 * v899);
        let v915: f64 = { let limexp_arg = v227; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v916: f64 = (-v915);
        let v917: f64 = (v115 * v915);
        let v918: f64 = (v115 * v916);
        let v919: f64 = (v909 + v917);
        let v920: f64 = (v913 + v918);
        let v921: f64 = (v230 * v901);
        let v922: f64 = (v224 * v919);
        let v923: f64 = (v921 + v922);
        let v924: f64 = (v230 * v903);
        let v925: f64 = (v224 * v910);
        let v926: f64 = (v924 + v925);
        let v927: f64 = (v230 * v905);
        let v928: f64 = (v224 * v920);
        let v929: f64 = (v927 + v928);
        let v930: f64 = (v230 * v907);
        let v931: f64 = (v224 * v914);
        let v932: f64 = (v930 + v931);
        let v933: f64 = (if self.scalar_v200 { v923 } else { v11 });
        let v934: f64 = (if self.scalar_v200 { v926 } else { v11 });
        let v935: f64 = (if self.scalar_v200 { v929 } else { v11 });
        let v936: f64 = (if self.scalar_v200 { v932 } else { v11 });
        let v937: f64 = (self.scalar_v219 * v880);
        let v938: f64 = (self.scalar_v219 * v881);
        let v939: f64 = (self.scalar_v219 * v882);
        let v940: f64 = (self.scalar_v219 * v883);
        let v941: f64 = (if self.scalar_v200 { v937 } else { v11 });
        let v942: f64 = (if self.scalar_v200 { v938 } else { v11 });
        let v943: f64 = (if self.scalar_v200 { v939 } else { v11 });
        let v944: f64 = (if self.scalar_v200 { v940 } else { v11 });
        let v945: f64 = (v7 * v888);
        let v946: f64 = (v218 + v945);
        let v947: f64 = (v7 * v889);
        let v948: f64 = (-v218);
        let v949: f64 = (v7 * v890);
        let v950: f64 = (v948 + v949);
        let v951: f64 = (v7 * v891);
        let v952: f64 = (v237 * v237);
        let v953: f64 = (v42 - v952);
        let v954: f64 = (v946 * v953);
        let v955: f64 = (v947 * v953);
        let v956: f64 = (v950 * v953);
        let v957: f64 = (v951 * v953);
        let v958: f64 = (if self.scalar_v200 { v954 } else { v11 });
        let v959: f64 = (if self.scalar_v200 { v955 } else { v11 });
        let v960: f64 = (if self.scalar_v200 { v956 } else { v11 });
        let v961: f64 = (if self.scalar_v200 { v957 } else { v11 });
        let v962: f64 = (v113 * v880);
        let v963: f64 = (v113 * v881);
        let v964: f64 = (v113 * v882);
        let v965: f64 = (v113 * v883);
        let v966: f64 = (-v958);
        let v967: f64 = (-v959);
        let v968: f64 = (-v960);
        let v969: f64 = (-v961);
        let v970: f64 = (v240 * v962);
        let v971: f64 = (v239 * v966);
        let v972: f64 = (v970 + v971);
        let v973: f64 = (v240 * v963);
        let v974: f64 = (v239 * v967);
        let v975: f64 = (v973 + v974);
        let v976: f64 = (v240 * v964);
        let v977: f64 = (v239 * v968);
        let v978: f64 = (v976 + v977);
        let v979: f64 = (v240 * v965);
        let v980: f64 = (v239 * v969);
        let v981: f64 = (v979 + v980);
        let v982: f64 = (v7 * v941);
        let v983: f64 = (v235 + v982);
        let v984: f64 = (v7 * v942);
        let v985: f64 = (-v235);
        let v986: f64 = (v7 * v943);
        let v987: f64 = (v985 + v986);
        let v988: f64 = (v7 * v944);
        let v989: f64 = (-v983);
        let v990: f64 = (-v984);
        let v991: f64 = (-v987);
        let v992: f64 = (-v988);
        let v993: f64 = (v243 * v972);
        let v994: f64 = (v241 * v989);
        let v995: f64 = (v993 + v994);
        let v996: f64 = (v243 * v975);
        let v997: f64 = (v241 * v990);
        let v998: f64 = (v996 + v997);
        let v999: f64 = (v243 * v978);
        let v1000: f64 = (v241 * v991);
        let v1001: f64 = (v999 + v1000);
        let v1002: f64 = (v243 * v981);
        let v1003: f64 = (v241 * v992);
        let v1004: f64 = (v1002 + v1003);
        let v1005: f64 = (if self.scalar_v200 { v995 } else { v11 });
        let v1006: f64 = (if self.scalar_v200 { v998 } else { v11 });
        let v1007: f64 = (if self.scalar_v200 { v1001 } else { v11 });
        let v1008: f64 = (if self.scalar_v200 { v1004 } else { v11 });
        let v1009: f64 = (v933 - v1005);
        let v1010: f64 = (v934 - v1006);
        let v1011: f64 = (v935 - v1007);
        let v1012: f64 = (v936 - v1008);
        let v1013: f64 = (v129 * v1009);
        let v1014: f64 = (v129 * v1010);
        let v1015: f64 = (v129 * v1011);
        let v1016: f64 = (v129 * v1012);
        let v1017: f64 = (if self.scalar_v200 { v1013 } else { v814 });
        let v1018: f64 = (if self.scalar_v200 { v1014 } else { v815 });
        let v1019: f64 = (if self.scalar_v200 { v1015 } else { v816 });
        let v1020: f64 = (if self.scalar_v200 { v1016 } else { v817 });
        let v1021: f64 = (if self.scalar_v251 { v688 } else { v821 });
        let v1022: f64 = (if self.scalar_v251 { v685 } else { v822 });
        let v1023: f64 = (if self.scalar_v251 { v689 } else { v823 });
        let v1025: f64 = (v252 * v1021);
        let v1026: f64 = (v1025 + v1025);
        let v1027: f64 = (v252 * v1022);
        let v1028: f64 = (v1027 + v1027);
        let v1029: f64 = (v252 * v1023);
        let v1030: f64 = (v1029 + v1029);
        let v1031: f64 = (v252 * self.scalar_v1024);
        let v1032: f64 = (v1031 + v1031);
        let v1033: f64 = (if self.scalar_v251 { v1026 } else { v830 });
        let v1034: f64 = (if self.scalar_v251 { v1028 } else { v831 });
        let v1035: f64 = (if self.scalar_v251 { v1030 } else { v832 });
        let v1036: f64 = (if self.scalar_v251 { v1032 } else { self.scalar_v833 });
        let v1037: f64 = (self.scalar_v161 * v1033);
        let v1038: f64 = (self.scalar_v161 * v1034);
        let v1039: f64 = (self.scalar_v161 * v1035);
        let v1040: f64 = (self.scalar_v161 * v1036);
        let v1041: f64 = (v1021 + v1037);
        let v1042: f64 = (v1022 + v1038);
        let v1043: f64 = (v1023 + v1039);
        let v1044: f64 = (self.scalar_v1024 + v1040);
        let v1045: f64 = (self.scalar_v164 * v1033);
        let v1046: f64 = (self.scalar_v164 * v1034);
        let v1047: f64 = (self.scalar_v164 * v1035);
        let v1048: f64 = (self.scalar_v164 * v1036);
        let v1049: f64 = (v257 * v1021);
        let v1050: f64 = (v252 * v1045);
        let v1051: f64 = (v1049 + v1050);
        let v1052: f64 = (v257 * v1022);
        let v1053: f64 = (v252 * v1046);
        let v1054: f64 = (v1052 + v1053);
        let v1055: f64 = (v257 * v1023);
        let v1056: f64 = (v252 * v1047);
        let v1057: f64 = (v1055 + v1056);
        let v1058: f64 = (v257 * self.scalar_v1024);
        let v1059: f64 = (v252 * v1048);
        let v1060: f64 = (v1058 + v1059);
        let v1061: f64 = (v1041 + v1051);
        let v1062: f64 = (v1042 + v1054);
        let v1063: f64 = (v1043 + v1057);
        let v1064: f64 = (v1044 + v1060);
        let v1065: f64 = (v259 * v671);
        let v1066: f64 = (v144 * v1061);
        let v1067: f64 = (v1065 + v1066);
        let v1068: f64 = (v144 * v1062);
        let v1069: f64 = (v259 * v672);
        let v1070: f64 = (v144 * v1063);
        let v1071: f64 = (v1069 + v1070);
        let v1072: f64 = (v144 * v1064);
        let v1073: f64 = (if self.scalar_v251 { v1067 } else { v727 });
        let v1074: f64 = (if self.scalar_v251 { v1068 } else { v728 });
        let v1075: f64 = (if self.scalar_v251 { v1071 } else { v729 });
        let v1076: f64 = (if self.scalar_v251 { v1072 } else { v730 });
        let v1077: f64 = { let limexp_arg = v261; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1078: f64 = (v1073 * v1077);
        let v1079: f64 = (v1074 * v1077);
        let v1080: f64 = (v1075 * v1077);
        let v1081: f64 = (v1076 * v1077);
        let v1082: f64 = (-v1073);
        let v1083: f64 = (-v1074);
        let v1084: f64 = (-v1075);
        let v1085: f64 = (-v1076);
        let v1086: f64 = { let limexp_arg = v263; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1087: f64 = (v1082 * v1086);
        let v1088: f64 = (v1083 * v1086);
        let v1089: f64 = (v1084 * v1086);
        let v1090: f64 = (v1085 * v1086);
        let v1091: f64 = (v1078 - v1087);
        let v1092: f64 = (v1079 - v1088);
        let v1093: f64 = (v1080 - v1089);
        let v1094: f64 = (v1081 - v1090);
        let v1095: f64 = (v129 * v1091);
        let v1096: f64 = (v129 * v1092);
        let v1097: f64 = (v129 * v1093);
        let v1098: f64 = (v129 * v1094);
        let v1099: f64 = (v267 * v267);
        let v1100: f64 = (v42 - v1099);
        let v1101: f64 = (v1095 * v1100);
        let v1102: f64 = (v1096 * v1100);
        let v1103: f64 = (v1097 * v1100);
        let v1104: f64 = (v1098 * v1100);
        let v1105: f64 = (if self.scalar_v251 { v1101 } else { v761 });
        let v1106: f64 = (if self.scalar_v251 { v1102 } else { v762 });
        let v1107: f64 = (if self.scalar_v251 { v1103 } else { v763 });
        let v1108: f64 = (if self.scalar_v251 { v1104 } else { v764 });
        let v1109: f64 = (self.scalar_v147 * v1105);
        let v1110: f64 = (self.scalar_v147 * v1106);
        let v1111: f64 = (self.scalar_v147 * v1107);
        let v1112: f64 = (self.scalar_v147 * v1108);
        let v1113: f64 = (if self.scalar_v251 { v1109 } else { v11 });
        let v1114: f64 = (if self.scalar_v251 { v1110 } else { v11 });
        let v1115: f64 = (if self.scalar_v251 { v1111 } else { v11 });
        let v1116: f64 = (if self.scalar_v251 { v1112 } else { v11 });
        let v1117: f64 = (v7 * v1113);
        let v1118: f64 = (v272 + v1117);
        let v1119: f64 = (v7 * v1114);
        let v1120: f64 = (-v272);
        let v1121: f64 = (v7 * v1115);
        let v1122: f64 = (v1120 + v1121);
        let v1123: f64 = (v7 * v1116);
        let v1124: f64 = (v274 * v274);
        let v1125: f64 = (v42 - v1124);
        let v1126: f64 = (v1118 * v1125);
        let v1127: f64 = (v1119 * v1125);
        let v1128: f64 = (v1122 * v1125);
        let v1129: f64 = (v1123 * v1125);
        let v1130: f64 = (if self.scalar_v251 { v1126 } else { v11 });
        let v1131: f64 = (if self.scalar_v251 { v1127 } else { v11 });
        let v1132: f64 = (if self.scalar_v251 { v1128 } else { v11 });
        let v1133: f64 = (if self.scalar_v251 { v1129 } else { v11 });
        let v1134: f64 = (self.scalar_v219 * v1105);
        let v1135: f64 = (self.scalar_v219 * v1106);
        let v1136: f64 = (self.scalar_v219 * v1107);
        let v1137: f64 = (self.scalar_v219 * v1108);
        let v1138: f64 = (if self.scalar_v251 { v1134 } else { v896 });
        let v1139: f64 = (if self.scalar_v251 { v1135 } else { v897 });
        let v1140: f64 = (if self.scalar_v251 { v1136 } else { v898 });
        let v1141: f64 = (if self.scalar_v251 { v1137 } else { v899 });
        let v1142: f64 = (v113 * v1105);
        let v1143: f64 = (v113 * v1106);
        let v1144: f64 = (v113 * v1107);
        let v1145: f64 = (v113 * v1108);
        let v1146: f64 = (v279 * v1130);
        let v1147: f64 = (v275 * v1142);
        let v1148: f64 = (v1146 + v1147);
        let v1149: f64 = (v279 * v1131);
        let v1150: f64 = (v275 * v1143);
        let v1151: f64 = (v1149 + v1150);
        let v1152: f64 = (v279 * v1132);
        let v1153: f64 = (v275 * v1144);
        let v1154: f64 = (v1152 + v1153);
        let v1155: f64 = (v279 * v1133);
        let v1156: f64 = (v275 * v1145);
        let v1157: f64 = (v1155 + v1156);
        let v1158: f64 = (v7 * v1138);
        let v1159: f64 = (v278 + v1158);
        let v1160: f64 = (v7 * v1139);
        let v1161: f64 = (-v278);
        let v1162: f64 = (v7 * v1140);
        let v1163: f64 = (v1161 + v1162);
        let v1164: f64 = (v7 * v1141);
        let v1165: f64 = (v801 + v1159);
        let v1166: f64 = (v802 + v1160);
        let v1167: f64 = (v283 * v1148);
        let v1168: f64 = (v280 * v1165);
        let v1169: f64 = (v1167 + v1168);
        let v1170: f64 = (v283 * v1151);
        let v1171: f64 = (v280 * v1166);
        let v1172: f64 = (v1170 + v1171);
        let v1173: f64 = (v283 * v1154);
        let v1174: f64 = (v280 * v1163);
        let v1175: f64 = (v1173 + v1174);
        let v1176: f64 = (v283 * v1157);
        let v1177: f64 = (v280 * v1164);
        let v1178: f64 = (v1176 + v1177);
        let v1179: f64 = (if self.scalar_v251 { v1169 } else { v1017 });
        let v1180: f64 = (if self.scalar_v251 { v1172 } else { v1018 });
        let v1181: f64 = (if self.scalar_v251 { v1175 } else { v1019 });
        let v1182: f64 = (if self.scalar_v251 { v1178 } else { v1020 });
        let v1183: f64 = (if self.scalar_v288 { v688 } else { v1021 });
        let v1184: f64 = (if self.scalar_v288 { v685 } else { v1022 });
        let v1185: f64 = (if self.scalar_v288 { v689 } else { v1023 });
        let v1187: f64 = (v289 * v1183);
        let v1188: f64 = (v1187 + v1187);
        let v1189: f64 = (v289 * v1184);
        let v1190: f64 = (v1189 + v1189);
        let v1191: f64 = (v289 * v1185);
        let v1192: f64 = (v1191 + v1191);
        let v1193: f64 = (v289 * self.scalar_v1186);
        let v1194: f64 = (v1193 + v1193);
        let v1195: f64 = (if self.scalar_v288 { v1188 } else { v1033 });
        let v1196: f64 = (if self.scalar_v288 { v1190 } else { v1034 });
        let v1197: f64 = (if self.scalar_v288 { v1192 } else { v1035 });
        let v1198: f64 = (if self.scalar_v288 { v1194 } else { v1036 });
        let v1199: f64 = (self.scalar_v161 * v1195);
        let v1200: f64 = (self.scalar_v161 * v1196);
        let v1201: f64 = (self.scalar_v161 * v1197);
        let v1202: f64 = (self.scalar_v161 * v1198);
        let v1203: f64 = (v1183 + v1199);
        let v1204: f64 = (v1184 + v1200);
        let v1205: f64 = (v1185 + v1201);
        let v1206: f64 = (self.scalar_v1186 + v1202);
        let v1207: f64 = (self.scalar_v164 * v1195);
        let v1208: f64 = (self.scalar_v164 * v1196);
        let v1209: f64 = (self.scalar_v164 * v1197);
        let v1210: f64 = (self.scalar_v164 * v1198);
        let v1211: f64 = (v294 * v1183);
        let v1212: f64 = (v289 * v1207);
        let v1213: f64 = (v1211 + v1212);
        let v1214: f64 = (v294 * v1184);
        let v1215: f64 = (v289 * v1208);
        let v1216: f64 = (v1214 + v1215);
        let v1217: f64 = (v294 * v1185);
        let v1218: f64 = (v289 * v1209);
        let v1219: f64 = (v1217 + v1218);
        let v1220: f64 = (v294 * self.scalar_v1186);
        let v1221: f64 = (v289 * v1210);
        let v1222: f64 = (v1220 + v1221);
        let v1223: f64 = (v1203 + v1213);
        let v1224: f64 = (v1204 + v1216);
        let v1225: f64 = (v1205 + v1219);
        let v1226: f64 = (v1206 + v1222);
        let v1227: f64 = (v296 * v671);
        let v1228: f64 = (v144 * v1223);
        let v1229: f64 = (v1227 + v1228);
        let v1230: f64 = (v144 * v1224);
        let v1231: f64 = (v296 * v672);
        let v1232: f64 = (v144 * v1225);
        let v1233: f64 = (v1231 + v1232);
        let v1234: f64 = (v144 * v1226);
        let v1235: f64 = (if self.scalar_v288 { v1229 } else { v1073 });
        let v1236: f64 = (if self.scalar_v288 { v1230 } else { v1074 });
        let v1237: f64 = (if self.scalar_v288 { v1233 } else { v1075 });
        let v1238: f64 = (if self.scalar_v288 { v1234 } else { v1076 });
        let v1239: f64 = (if self.scalar_v288 { v818 } else { v844 });
        let v1240: f64 = (if self.scalar_v288 { v819 } else { v845 });
        let v1241: f64 = (if self.scalar_v288 { v820 } else { v846 });
        let v1242: f64 = (if self.scalar_v288 { v11 } else { v847 });
        let v1243: f64 = (v299 * v1239);
        let v1244: f64 = (v1243 + v1243);
        let v1245: f64 = (v299 * v1240);
        let v1246: f64 = (v1245 + v1245);
        let v1247: f64 = (v299 * v1241);
        let v1248: f64 = (v1247 + v1247);
        let v1249: f64 = (v299 * v1242);
        let v1250: f64 = (v1249 + v1249);
        let v1251: f64 = (if self.scalar_v288 { v1244 } else { v11 });
        let v1252: f64 = (if self.scalar_v288 { v1246 } else { v11 });
        let v1253: f64 = (if self.scalar_v288 { v1248 } else { v11 });
        let v1254: f64 = (if self.scalar_v288 { v1250 } else { v11 });
        let v1255: f64 = (self.scalar_v161 * v1251);
        let v1256: f64 = (self.scalar_v161 * v1252);
        let v1257: f64 = (self.scalar_v161 * v1253);
        let v1258: f64 = (self.scalar_v161 * v1254);
        let v1259: f64 = (v1239 + v1255);
        let v1260: f64 = (v1240 + v1256);
        let v1261: f64 = (v1241 + v1257);
        let v1262: f64 = (v1242 + v1258);
        let v1263: f64 = (self.scalar_v164 * v1239);
        let v1264: f64 = (self.scalar_v164 * v1240);
        let v1265: f64 = (self.scalar_v164 * v1241);
        let v1266: f64 = (self.scalar_v164 * v1242);
        let v1267: f64 = (v304 * v1251);
        let v1268: f64 = (v301 * v1263);
        let v1269: f64 = (v1267 + v1268);
        let v1270: f64 = (v304 * v1252);
        let v1271: f64 = (v301 * v1264);
        let v1272: f64 = (v1270 + v1271);
        let v1273: f64 = (v304 * v1253);
        let v1274: f64 = (v301 * v1265);
        let v1275: f64 = (v1273 + v1274);
        let v1276: f64 = (v304 * v1254);
        let v1277: f64 = (v301 * v1266);
        let v1278: f64 = (v1276 + v1277);
        let v1279: f64 = (v1259 + v1269);
        let v1280: f64 = (v1260 + v1272);
        let v1281: f64 = (v1261 + v1275);
        let v1282: f64 = (v1262 + v1278);
        let v1283: f64 = (v306 * v671);
        let v1284: f64 = (v144 * v1279);
        let v1285: f64 = (v1283 + v1284);
        let v1286: f64 = (v144 * v1280);
        let v1287: f64 = (v306 * v672);
        let v1288: f64 = (v144 * v1281);
        let v1289: f64 = (v1287 + v1288);
        let v1290: f64 = (v144 * v1282);
        let v1291: f64 = (if self.scalar_v288 { v1285 } else { v870 });
        let v1292: f64 = (if self.scalar_v288 { v1286 } else { v871 });
        let v1293: f64 = (if self.scalar_v288 { v1289 } else { v872 });
        let v1294: f64 = (if self.scalar_v288 { v1290 } else { v873 });
        let v1295: f64 = { let limexp_arg = v298; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1296: f64 = (v1235 * v1295);
        let v1297: f64 = (v1236 * v1295);
        let v1298: f64 = (v1237 * v1295);
        let v1299: f64 = (v1238 * v1295);
        let v1300: f64 = (-v1235);
        let v1301: f64 = (-v1236);
        let v1302: f64 = (-v1237);
        let v1303: f64 = (-v1238);
        let v1304: f64 = { let limexp_arg = v310; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1305: f64 = (v1300 * v1304);
        let v1306: f64 = (v1301 * v1304);
        let v1307: f64 = (v1302 * v1304);
        let v1308: f64 = (v1303 * v1304);
        let v1309: f64 = (v1296 - v1305);
        let v1310: f64 = (v1297 - v1306);
        let v1311: f64 = (v1298 - v1307);
        let v1312: f64 = (v1299 - v1308);
        let v1313: f64 = (v129 * v1309);
        let v1314: f64 = (v129 * v1310);
        let v1315: f64 = (v129 * v1311);
        let v1316: f64 = (v129 * v1312);
        let v1317: f64 = (v314 * v314);
        let v1318: f64 = (v42 - v1317);
        let v1319: f64 = (v1313 * v1318);
        let v1320: f64 = (v1314 * v1318);
        let v1321: f64 = (v1315 * v1318);
        let v1322: f64 = (v1316 * v1318);
        let v1323: f64 = (if self.scalar_v288 { v1319 } else { v1105 });
        let v1324: f64 = (if self.scalar_v288 { v1320 } else { v1106 });
        let v1325: f64 = (if self.scalar_v288 { v1321 } else { v1107 });
        let v1326: f64 = (if self.scalar_v288 { v1322 } else { v1108 });
        let v1327: f64 = { let limexp_arg = v308; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1328: f64 = (v1291 * v1327);
        let v1329: f64 = (v1292 * v1327);
        let v1330: f64 = (v1293 * v1327);
        let v1331: f64 = (v1294 * v1327);
        let v1332: f64 = (-v1291);
        let v1333: f64 = (-v1292);
        let v1334: f64 = (-v1293);
        let v1335: f64 = (-v1294);
        let v1336: f64 = { let limexp_arg = v318; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1337: f64 = (v1332 * v1336);
        let v1338: f64 = (v1333 * v1336);
        let v1339: f64 = (v1334 * v1336);
        let v1340: f64 = (v1335 * v1336);
        let v1341: f64 = (v1328 - v1337);
        let v1342: f64 = (v1329 - v1338);
        let v1343: f64 = (v1330 - v1339);
        let v1344: f64 = (v1331 - v1340);
        let v1345: f64 = (v129 * v1341);
        let v1346: f64 = (v129 * v1342);
        let v1347: f64 = (v129 * v1343);
        let v1348: f64 = (v129 * v1344);
        let v1349: f64 = (v322 * v322);
        let v1350: f64 = (v42 - v1349);
        let v1351: f64 = (v1345 * v1350);
        let v1352: f64 = (v1346 * v1350);
        let v1353: f64 = (v1347 * v1350);
        let v1354: f64 = (v1348 * v1350);
        let v1355: f64 = (if self.scalar_v288 { v1351 } else { v11 });
        let v1356: f64 = (if self.scalar_v288 { v1352 } else { v11 });
        let v1357: f64 = (if self.scalar_v288 { v1353 } else { v11 });
        let v1358: f64 = (if self.scalar_v288 { v1354 } else { v11 });
        let v1359: f64 = (self.scalar_v147 * v1323);
        let v1360: f64 = (self.scalar_v147 * v1324);
        let v1361: f64 = (self.scalar_v147 * v1325);
        let v1362: f64 = (self.scalar_v147 * v1326);
        let v1363: f64 = (if self.scalar_v288 { v1359 } else { v1113 });
        let v1364: f64 = (if self.scalar_v288 { v1360 } else { v1114 });
        let v1365: f64 = (if self.scalar_v288 { v1361 } else { v1115 });
        let v1366: f64 = (if self.scalar_v288 { v1362 } else { v1116 });
        let v1367: f64 = (self.scalar_v147 * v1355);
        let v1368: f64 = (self.scalar_v147 * v1356);
        let v1369: f64 = (self.scalar_v147 * v1357);
        let v1370: f64 = (self.scalar_v147 * v1358);
        let v1371: f64 = (if self.scalar_v288 { v1367 } else { v11 });
        let v1372: f64 = (if self.scalar_v288 { v1368 } else { v11 });
        let v1373: f64 = (if self.scalar_v288 { v1369 } else { v11 });
        let v1374: f64 = (if self.scalar_v288 { v1370 } else { v11 });
        let v1375: f64 = (v7 * v1363);
        let v1376: f64 = (v327 + v1375);
        let v1377: f64 = (v7 * v1364);
        let v1378: f64 = (-v327);
        let v1379: f64 = (v7 * v1365);
        let v1380: f64 = (v1378 + v1379);
        let v1381: f64 = (v7 * v1366);
        let v1382: f64 = (v332 * v332);
        let v1383: f64 = (v42 - v1382);
        let v1384: f64 = (v1376 * v1383);
        let v1385: f64 = (v1377 * v1383);
        let v1386: f64 = (v1380 * v1383);
        let v1387: f64 = (v1381 * v1383);
        let v1388: f64 = (if self.scalar_v288 { v1384 } else { v1130 });
        let v1389: f64 = (if self.scalar_v288 { v1385 } else { v1131 });
        let v1390: f64 = (if self.scalar_v288 { v1386 } else { v1132 });
        let v1391: f64 = (if self.scalar_v288 { v1387 } else { v1133 });
        let v1392: f64 = (v7 * v1371);
        let v1393: f64 = (v330 + v1392);
        let v1394: f64 = (v7 * v1372);
        let v1395: f64 = (-v330);
        let v1396: f64 = (v7 * v1373);
        let v1397: f64 = (v1395 + v1396);
        let v1398: f64 = (v7 * v1374);
        let v1399: f64 = (v335 * v335);
        let v1400: f64 = (v42 - v1399);
        let v1401: f64 = (v1393 * v1400);
        let v1402: f64 = (v1394 * v1400);
        let v1403: f64 = (v1397 * v1400);
        let v1404: f64 = (v1398 * v1400);
        let v1405: f64 = (if self.scalar_v288 { v1401 } else { v11 });
        let v1406: f64 = (if self.scalar_v288 { v1402 } else { v11 });
        let v1407: f64 = (if self.scalar_v288 { v1403 } else { v11 });
        let v1408: f64 = (if self.scalar_v288 { v1404 } else { v11 });
        let v1409: f64 = (self.scalar_v219 * v1355);
        let v1410: f64 = (self.scalar_v219 * v1356);
        let v1411: f64 = (self.scalar_v219 * v1357);
        let v1412: f64 = (self.scalar_v219 * v1358);
        let v1413: f64 = (if self.scalar_v288 { v1409 } else { v11 });
        let v1414: f64 = (if self.scalar_v288 { v1410 } else { v11 });
        let v1415: f64 = (if self.scalar_v288 { v1411 } else { v11 });
        let v1416: f64 = (if self.scalar_v288 { v1412 } else { v11 });
        let v1417: f64 = (self.scalar_v219 * v1323);
        let v1418: f64 = (self.scalar_v219 * v1324);
        let v1419: f64 = (self.scalar_v219 * v1325);
        let v1420: f64 = (self.scalar_v219 * v1326);
        let v1421: f64 = (if self.scalar_v288 { v1417 } else { v11 });
        let v1422: f64 = (if self.scalar_v288 { v1418 } else { v11 });
        let v1423: f64 = (if self.scalar_v288 { v1419 } else { v11 });
        let v1424: f64 = (if self.scalar_v288 { v1420 } else { v11 });
        let v1425: f64 = (v113 * v1323);
        let v1426: f64 = (v113 * v1324);
        let v1427: f64 = (v113 * v1325);
        let v1428: f64 = (v113 * v1326);
        let v1429: f64 = (v344 * v1425);
        let v1430: f64 = (v343 * v1388);
        let v1431: f64 = (v1429 + v1430);
        let v1432: f64 = (v344 * v1426);
        let v1433: f64 = (v343 * v1389);
        let v1434: f64 = (v1432 + v1433);
        let v1435: f64 = (v344 * v1427);
        let v1436: f64 = (v343 * v1390);
        let v1437: f64 = (v1435 + v1436);
        let v1438: f64 = (v344 * v1428);
        let v1439: f64 = (v343 * v1391);
        let v1440: f64 = (v1438 + v1439);
        let v1441: f64 = (v7 * v1421);
        let v1442: f64 = (v342 + v1441);
        let v1443: f64 = (v7 * v1422);
        let v1444: f64 = (-v342);
        let v1445: f64 = (v7 * v1423);
        let v1446: f64 = (v1444 + v1445);
        let v1447: f64 = (v7 * v1424);
        let v1448: f64 = (v917 + v1442);
        let v1449: f64 = (v918 + v1446);
        let v1450: f64 = (v348 * v1431);
        let v1451: f64 = (v345 * v1448);
        let v1452: f64 = (v1450 + v1451);
        let v1453: f64 = (v348 * v1434);
        let v1454: f64 = (v345 * v1443);
        let v1455: f64 = (v1453 + v1454);
        let v1456: f64 = (v348 * v1437);
        let v1457: f64 = (v345 * v1449);
        let v1458: f64 = (v1456 + v1457);
        let v1459: f64 = (v348 * v1440);
        let v1460: f64 = (v345 * v1447);
        let v1461: f64 = (v1459 + v1460);
        let v1462: f64 = (if self.scalar_v288 { v1452 } else { v933 });
        let v1463: f64 = (if self.scalar_v288 { v1455 } else { v934 });
        let v1464: f64 = (if self.scalar_v288 { v1458 } else { v935 });
        let v1465: f64 = (if self.scalar_v288 { v1461 } else { v936 });
        let v1466: f64 = (v113 * v1355);
        let v1467: f64 = (v113 * v1356);
        let v1468: f64 = (v113 * v1357);
        let v1469: f64 = (v113 * v1358);
        let v1470: f64 = (-v1405);
        let v1471: f64 = (-v1406);
        let v1472: f64 = (-v1407);
        let v1473: f64 = (-v1408);
        let v1474: f64 = (v352 * v1466);
        let v1475: f64 = (v351 * v1470);
        let v1476: f64 = (v1474 + v1475);
        let v1477: f64 = (v352 * v1467);
        let v1478: f64 = (v351 * v1471);
        let v1479: f64 = (v1477 + v1478);
        let v1480: f64 = (v352 * v1468);
        let v1481: f64 = (v351 * v1472);
        let v1482: f64 = (v1480 + v1481);
        let v1483: f64 = (v352 * v1469);
        let v1484: f64 = (v351 * v1473);
        let v1485: f64 = (v1483 + v1484);
        let v1486: f64 = (v7 * v1413);
        let v1487: f64 = (v339 + v1486);
        let v1488: f64 = (v7 * v1414);
        let v1489: f64 = (-v339);
        let v1490: f64 = (v7 * v1415);
        let v1491: f64 = (v1489 + v1490);
        let v1492: f64 = (v7 * v1416);
        let v1493: f64 = (-v1487);
        let v1494: f64 = (-v1488);
        let v1495: f64 = (-v1491);
        let v1496: f64 = (-v1492);
        let v1497: f64 = (v355 * v1476);
        let v1498: f64 = (v353 * v1493);
        let v1499: f64 = (v1497 + v1498);
        let v1500: f64 = (v355 * v1479);
        let v1501: f64 = (v353 * v1494);
        let v1502: f64 = (v1500 + v1501);
        let v1503: f64 = (v355 * v1482);
        let v1504: f64 = (v353 * v1495);
        let v1505: f64 = (v1503 + v1504);
        let v1506: f64 = (v355 * v1485);
        let v1507: f64 = (v353 * v1496);
        let v1508: f64 = (v1506 + v1507);
        let v1509: f64 = (if self.scalar_v288 { v1499 } else { v1005 });
        let v1510: f64 = (if self.scalar_v288 { v1502 } else { v1006 });
        let v1511: f64 = (if self.scalar_v288 { v1505 } else { v1007 });
        let v1512: f64 = (if self.scalar_v288 { v1508 } else { v1008 });
        let v1513: f64 = (v1462 - v1509);
        let v1514: f64 = (v1463 - v1510);
        let v1515: f64 = (v1464 - v1511);
        let v1516: f64 = (v1465 - v1512);
        let v1517: f64 = (v129 * v1513);
        let v1518: f64 = (v129 * v1514);
        let v1519: f64 = (v129 * v1515);
        let v1520: f64 = (v129 * v1516);
        let v1521: f64 = (if self.scalar_v288 { v1517 } else { v1179 });
        let v1522: f64 = (if self.scalar_v288 { v1518 } else { v1180 });
        let v1523: f64 = (if self.scalar_v288 { v1519 } else { v1181 });
        let v1524: f64 = (if self.scalar_v288 { v1520 } else { v1182 });
        let v1525: f64 = (v118 * v733);
        let v1526: f64 = (-v1525);
        let v1527: f64 = (v362 * v362);
        let v1528: f64 = (v1526 / v1527);
        let v1529: f64 = (v118 * v734);
        let v1530: f64 = (-v1529);
        let v1531: f64 = (v1530 / v1527);
        let v1532: f64 = (v118 * v735);
        let v1533: f64 = (-v1532);
        let v1534: f64 = (v1533 / v1527);
        let v1535: f64 = (v118 * v736);
        let v1536: f64 = (-v1535);
        let v1537: f64 = (v1536 / v1527);
        let v1538: f64 = (if self.scalar_v249 { v1528 } else { v11 });
        let v1539: f64 = (if self.scalar_v249 { v1531 } else { v11 });
        let v1540: f64 = (if self.scalar_v249 { v1534 } else { v11 });
        let v1541: f64 = (if self.scalar_v249 { v1537 } else { v11 });
        let v1542: f64 = (v118 * v1323);
        let v1543: f64 = (-v1542);
        let v1544: f64 = (v369 * v369);
        let v1545: f64 = (v1543 / v1544);
        let v1546: f64 = (v118 * v1324);
        let v1547: f64 = (-v1546);
        let v1548: f64 = (v1547 / v1544);
        let v1549: f64 = (v118 * v1325);
        let v1550: f64 = (-v1549);
        let v1551: f64 = (v1550 / v1544);
        let v1552: f64 = (v118 * v1326);
        let v1553: f64 = (-v1552);
        let v1554: f64 = (v1553 / v1544);
        let v1555: f64 = (if self.scalar_v250 { v1545 } else { v1538 });
        let v1556: f64 = (if self.scalar_v250 { v1548 } else { v1539 });
        let v1557: f64 = (if self.scalar_v250 { v1551 } else { v1540 });
        let v1558: f64 = (if self.scalar_v250 { v1554 } else { v1541 });
        let v1559: f64 = (if self.scalar_v374 { v11 } else { v1183 });
        let v1560: f64 = (if self.scalar_v374 { v11 } else { v1184 });
        let v1561: f64 = (if self.scalar_v374 { v11 } else { v1185 });
        let v1565: f64 = (if self.scalar_v385 { v11 } else { v1559 });
        let v1566: f64 = (if self.scalar_v385 { v11 } else { v1560 });
        let v1567: f64 = (if self.scalar_v385 { v11 } else { v1561 });
        let v1569: f64 = (v392 * v392);
        let v1570: f64 = (v42 - v1569);
        let v1571: f64 = (-v1570);
        let v1572: f64 = (if self.scalar_v391 { v1571 } else { self.scalar_v1563 });
        let v1573: f64 = (if self.scalar_v391 { v1570 } else { self.scalar_v1564 });
        let v1574: f64 = (v394 * v394);
        let v1575: f64 = (v42 - v1574);
        let v1576: f64 = (-v1575);
        let v1577: f64 = (if self.scalar_v391 { v1576 } else { self.scalar_v1563 });
        let v1578: f64 = (if self.scalar_v391 { v1575 } else { self.scalar_v1564 });
        let v1579: f64 = (if self.scalar_v397 { v375 } else { v1572 });
        let v1580: f64 = (if self.scalar_v397 { v42 } else { v1573 });
        let v1581: f64 = (if self.scalar_v397 { v375 } else { v1577 });
        let v1582: f64 = (if self.scalar_v397 { v42 } else { v1578 });
        let v1583: f64 = (v136 * v1579);
        let v1584: f64 = (v136 * v1580);
        let v1585: f64 = { let limexp_arg = v401; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1586: f64 = (v1583 * v1585);
        let v1587: f64 = (v1584 * v1585);
        let v1588: f64 = (-v1565);
        let v1589: f64 = (-v1566);
        let v1590: f64 = (v1586 - v1567);
        let v1591: f64 = (v1587 - self.scalar_v1568);
        let v1592: f64 = (self.scalar_v400 * v1588);
        let v1593: f64 = (self.scalar_v400 * v1589);
        let v1594: f64 = (self.scalar_v400 * v1590);
        let v1595: f64 = (self.scalar_v400 * v1591);
        let v1596: f64 = (v136 * v1581);
        let v1597: f64 = (v136 * v1582);
        let v1598: f64 = { let limexp_arg = v405; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1599: f64 = (v1596 * v1598);
        let v1600: f64 = (v1597 * v1598);
        let v1601: f64 = (v1599 - v1565);
        let v1602: f64 = (-v1567);
        let v1604: f64 = (self.scalar_v400 * v1601);
        let v1605: f64 = (self.scalar_v400 * v1602);
        let v1606: f64 = (self.scalar_v400 * v1600);
        let v1779: f64 = (-v1521);
        let v1780: f64 = (-v1522);
        let v1781: f64 = (-v1523);
        let v1782: f64 = (-v1524);
        let v1824: f64 = (-v119);
        let v1825: f64 = (v577 * v1555);
        let v1826: f64 = (-v1825);
        let v1827: f64 = (v372 * v372);
        let v1828: f64 = (v1826 / v1827);
        let v1829: f64 = (v577 * v1556);
        let v1830: f64 = (-v1829);
        let v1831: f64 = (v1830 / v1827);
        let v1832: f64 = (-v372);
        let v1833: f64 = (v577 * v1557);
        let v1834: f64 = (v1832 - v1833);
        let v1835: f64 = (v1834 / v1827);
        let v1836: f64 = (v577 * v1558);
        let v1837: f64 = (-v1836);
        let v1838: f64 = (v1837 / v1827);
        let v1839: f64 = (v42 / v372);
        let v1840: f64 = (if self.scalar_v505 { v1828 } else { v11 });
        let v1841: f64 = (if self.scalar_v505 { v1831 } else { v11 });
        let v1842: f64 = (if self.scalar_v505 { v1835 } else { v11 });
        let v1843: f64 = (if self.scalar_v505 { v1838 } else { v11 });
        let v1844: f64 = (if self.scalar_v505 { v1839 } else { v11 });
        let v1859: f64 = (if self.scalar_v528 { v547 } else { v11 });
        let v1860: f64 = (if self.scalar_v528 { v543 } else { v11 });

        let d553_dn3: f64 = v1779;
        let d553_dn4: f64 = v1780;
        let d553_dn5: f64 = v1781;
        let d553_dn8: f64 = v1782;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(12),
            None,
            multiplicity * (v553),
            [3, 4, 5, 8],
            [d553_dn3, d553_dn4, d553_dn5, d553_dn8],
            [],
            [],
            multiplicity,
        );
        let d10_dn13: f64 = v42;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (v10),
            13,
            multiplicity * (d10_dn13),
        );
        let d10_dn13: f64 = v42;
        stamper.stamp_current_node1_local(
            Some(3),
            Some(5),
            multiplicity * (v10),
            13,
            multiplicity * (d10_dn13),
        );
        let d404_dn3: f64 = v1592;
        let d404_dn4: f64 = v1593;
        let d404_dn5: f64 = v1594;
        let d404_dn8: f64 = v1595;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(5),
            multiplicity * (v404),
            [3, 4, 5, 8],
            [d404_dn3, d404_dn4, d404_dn5, d404_dn8],
            [],
            [],
            multiplicity,
        );
        let d408_dn3: f64 = v1604;
        let d408_dn4: f64 = v1593;
        let d408_dn5: f64 = v1605;
        let d408_dn7: f64 = v1606;
        let d408_dn8: f64 = self.scalar_v1607;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(3),
            multiplicity * (v408),
            [3, 4, 5, 7, 8],
            [d408_dn3, d408_dn4, d408_dn5, d408_dn7, d408_dn8],
            [],
            [],
            multiplicity,
        );
        let d579_dn3: f64 = v1840;
        let d579_dn4: f64 = v1841;
        let d579_dn5: f64 = v1842;
        let d579_dn8: f64 = v1843;
        let d579_dn10: f64 = v1844;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            Some(5),
            multiplicity * (v579),
            [3, 4, 5, 8, 10],
            [d579_dn3, d579_dn4, d579_dn5, d579_dn8, d579_dn10],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(5),
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            self.scalar_v581,
        );
        let d588_dn5: f64 = self.scalar_v1848;
        let d588_dn9: f64 = self.scalar_v1849;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(5),
            multiplicity * (v588),
            5,
            multiplicity * (d588_dn5),
            9,
            multiplicity * (d588_dn9),
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(5),
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            self.scalar_v590,
        );
        let d593_dn4: f64 = self.scalar_v1852;
        let d593_dn7: f64 = self.scalar_v1853;
        stamper.stamp_current_node2_local(
            Some(4),
            Some(7),
            multiplicity * (v593),
            4,
            multiplicity * (d593_dn4),
            7,
            multiplicity * (d593_dn7),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(7),
            multiplicity * (self.scalar_v595),
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(7),
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            self.scalar_v597,
        );
        let d600_dn4: f64 = self.scalar_v1856;
        let d600_dn8: f64 = self.scalar_v1857;
        stamper.stamp_current_node2_local(
            Some(4),
            Some(8),
            multiplicity * (v600),
            4,
            multiplicity * (d600_dn4),
            8,
            multiplicity * (d600_dn8),
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            self.scalar_v602,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            7,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            7,
            self.scalar_v604,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            9,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            9,
            self.scalar_v608,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            11,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            11,
            self.scalar_v610,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            12,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            12,
            self.scalar_v612,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            16,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            16,
            self.scalar_v614,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            18,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            18,
            self.scalar_v618,
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (self.scalar_v620),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (self.scalar_v620),
        );
        stamper.stamp_current_const_local(
            Some(14),
            None,
            multiplicity * (self.scalar_v621),
        );
        let d623_dn14: f64 = self.scalar_v1858;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (v623),
            14,
            multiplicity * (d623_dn14),
        );
        stamper.stamp_current_const_local(
            Some(15),
            None,
            multiplicity * (self.scalar_v621),
        );
        let d625_dn15: f64 = self.scalar_v1858;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (v625),
            15,
            multiplicity * (d625_dn15),
        );
        let d623_dn14: f64 = self.scalar_v1858;
        stamper.stamp_current_node1_local(
            Some(4),
            Some(5),
            multiplicity * (v623),
            14,
            multiplicity * (d623_dn14),
        );
        let d629_dn14: f64 = v1859;
        let d629_dn15: f64 = v1860;
        stamper.stamp_current_node2_local(
            Some(4),
            Some(3),
            multiplicity * (v629),
            14,
            multiplicity * (d629_dn14),
            15,
            multiplicity * (d629_dn15),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (self.scalar_v621),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(5),
            multiplicity * (self.scalar_v621),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (self.scalar_v621),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (self.scalar_v635),
        );
        let d622_dn14: f64 = v42;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (v622),
            14,
            multiplicity * (d622_dn14),
        );
        let d624_dn15: f64 = v42;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (v624),
            15,
            multiplicity * (d624_dn15),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(5),
            multiplicity * (self.scalar_v636),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(3),
            multiplicity * (self.scalar_v636),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(5),
            multiplicity * (self.scalar_v638),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(3),
            multiplicity * (self.scalar_v638),
        );
        stamper.stamp_current_const_local(
            Some(11),
            None,
            multiplicity * (v649),
        );
        let d651_dn11: f64 = self.scalar_v1866;
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (v651),
            11,
            multiplicity * (d651_dn11),
        );
        let d655_dn11: f64 = self.scalar_v1867;
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (v655),
            11,
            multiplicity * (d655_dn11),
        );
        let d556_dn12: f64 = self.scalar_v554;
        let v556_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, v556);
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (v556_ddt),
            12,
            multiplicity * (((d556_dn12) * ddt_scale)),
        );
        let d571_dn1: f64 = self.scalar_v568;
        let d571_dn3: f64 = self.scalar_v1822;
        let v571_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, v571);
        stamper.stamp_current_node2_local(
            Some(4),
            Some(3),
            multiplicity * (v571_ddt),
            1,
            multiplicity * (((d571_dn1) * ddt_scale)),
            3,
            multiplicity * (((d571_dn3) * ddt_scale)),
        );
        let d573_dn3: f64 = self.scalar_v572;
        let d573_dn5: f64 = self.scalar_v1823;
        let v573_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, v573);
        stamper.stamp_current_node2_local(
            Some(3),
            Some(5),
            multiplicity * (v573_ddt),
            3,
            multiplicity * (((d573_dn3) * ddt_scale)),
            5,
            multiplicity * (((d573_dn5) * ddt_scale)),
        );
        let d576_dn3: f64 = v119;
        let d576_dn10: f64 = v1824;
        let v576_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, v576);
        stamper.stamp_current_node2_local(
            Some(3),
            Some(10),
            multiplicity * (v576_ddt),
            3,
            multiplicity * (((d576_dn3) * ddt_scale)),
            10,
            multiplicity * (((d576_dn10) * ddt_scale)),
        );
        let d585_dn8: f64 = self.scalar_v1845;
        let d585_dn9: f64 = self.scalar_v582;
        let v585_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, v585);
        stamper.stamp_current_node2_local(
            Some(9),
            Some(8),
            multiplicity * (v585_ddt),
            8,
            multiplicity * (((d585_dn8) * ddt_scale)),
            9,
            multiplicity * (((d585_dn9) * ddt_scale)),
        );
        let mut locals = StampLocals::default();

        Self::stamp_transient_block_0(ctx, p, nodes, param_given, &mut locals);
        Self::stamp_transient_block_1(p, &mut locals);
        Self::stamp_transient_block_2(p, &mut locals);

        stamper.stamp_potential_branch_local(
            Some(12),
            Some(13),
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            5,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            6,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            8,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            10,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            Some(2),
            13,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            14,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            15,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            17,
            multiplicity,
        );

        Self::stamp_transient_equations_block_0(ctx, stamper, p, nodes, branches, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, &mut locals);
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let scalar_temperature_static_temperature = (ctx).temperature();
        let scalar_temperature_static_thermal_voltage = (ctx).thermal_voltage();
        self.ensure_temperature_static(scalar_temperature_static_temperature, scalar_temperature_static_thermal_voltage);
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let v0: f64 = nv8;
        let v1: f64 = nv5;
        let v4: f64 = nv3;
        let v7: f64 = (v4 - v1);
        let v11: f64 = 0.0;
        let v30: f64 = nv11;
        let v31: f64 = ((v30) as f64).abs();
        let v32: f64 = (self.scalar_v21 + v31);
        let v33: f64 = (if (self.scalar_v29 != 0.0) { v32 } else { self.scalar_v21 });
        let v36: f64 = (v33 - self.scalar_v28);
        let v37: f64 = ((v36) as f64).abs();
        let v38: bool = (v37 > v11);
        let v41: bool = (v38 || self.scalar_v40);
        let v42: f64 = 1.0;
        let v81: f64 = (v37 * self.scalar_v80);
        let v82: f64 = (v42 + v81);
        let v83: f64 = (self.scalar_v79 * v82);
        let v84: f64 = (if v41 { v83 } else { v11 });
        let v112: bool = (!v41);
        let v119: f64 = (if v112 { self.scalar_v79 } else { v84 });
        let v555: f64 = nv12;
        let v556: f64 = (self.scalar_v554 * v555);
        let v569: f64 = nv1;
        let v570: f64 = (v569 - v4);
        let v571: f64 = (self.scalar_v568 * v570);
        let v573: f64 = (v7 * self.scalar_v572);
        let v574: f64 = nv10;
        let v575: f64 = (v4 - v574);
        let v576: f64 = (v119 * v575);
        let v583: f64 = nv9;
        let v584: f64 = (v583 - v0);
        let v585: f64 = (self.scalar_v582 * v584);
        let v1824: f64 = (-v119);

        let d556_dn12: f64 = self.scalar_v554;
        stamper.stamp_current_reactive_node1(
            Some(nodes[12]),
            None,
            nodes[12],
            multiplicity * (d556_dn12),
        );
        let d571_dn1: f64 = self.scalar_v568;
        let d571_dn3: f64 = self.scalar_v1822;
        stamper.stamp_current_reactive_node2(
            Some(nodes[4]),
            Some(nodes[3]),
            nodes[1],
            multiplicity * (d571_dn1),
            nodes[3],
            multiplicity * (d571_dn3),
        );
        let d573_dn3: f64 = self.scalar_v572;
        let d573_dn5: f64 = self.scalar_v1823;
        stamper.stamp_current_reactive_node2(
            Some(nodes[3]),
            Some(nodes[5]),
            nodes[3],
            multiplicity * (d573_dn3),
            nodes[5],
            multiplicity * (d573_dn5),
        );
        let d576_dn3: f64 = v119;
        let d576_dn10: f64 = v1824;
        stamper.stamp_current_reactive_node2(
            Some(nodes[3]),
            Some(nodes[10]),
            nodes[3],
            multiplicity * (d576_dn3),
            nodes[10],
            multiplicity * (d576_dn10),
        );
        let d585_dn8: f64 = self.scalar_v1845;
        let d585_dn9: f64 = self.scalar_v582;
        stamper.stamp_current_reactive_node2(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes[8],
            multiplicity * (d585_dn8),
            nodes[9],
            multiplicity * (d585_dn9),
        );
        let mut locals = StampLocals::default();

        Self::stamp_reactive_block_0(ctx, p, nodes, param_given, &mut locals);
        Self::stamp_reactive_block_1(p, &mut locals);

        Self::stamp_reactive_equations_block_0(ctx, stamper, p, nodes, branches, multiplicity, &mut locals);
    }
}

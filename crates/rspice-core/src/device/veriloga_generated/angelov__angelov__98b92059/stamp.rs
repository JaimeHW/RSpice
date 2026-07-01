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
        let v69: f64 = (v37 * self.scalar_v68);
        let v70: f64 = (v42 + v69);
        let v71: f64 = (self.scalar_v67 * v70);
        let v72: f64 = (if v41 { v71 } else { v11 });
        let v75: f64 = (v37 * self.scalar_v74);
        let v76: f64 = (v42 + v75);
        let v77: f64 = (self.scalar_v73 * v76);
        let v78: f64 = (if v41 { v77 } else { v11 });
        let v81: f64 = (v37 * self.scalar_v80);
        let v82: f64 = (self.scalar_v79 + v81);
        let v83: f64 = (if v41 { v82 } else { v11 });
        let v86: f64 = (v37 * self.scalar_v85);
        let v87: f64 = (self.scalar_v84 + v86);
        let v88: f64 = (if v41 { v87 } else { v11 });
        let v91: f64 = (v37 * self.scalar_v90);
        let v92: f64 = (self.scalar_v89 + v91);
        let v93: f64 = (if v41 { v92 } else { v11 });
        let v94: bool = (!v41);
        let v95: f64 = (if v94 { self.scalar_v43 } else { v48 });
        let v96: f64 = (if v94 { self.scalar_v49 } else { v54 });
        let v97: f64 = (if v94 { self.scalar_v55 } else { v60 });
        let v98: f64 = (if v94 { self.scalar_v61 } else { v66 });
        let v99: f64 = (if v94 { self.scalar_v67 } else { v72 });
        let v100: f64 = (if v94 { self.scalar_v73 } else { v78 });
        let v101: f64 = (if v94 { self.scalar_v79 } else { v83 });
        let v102: f64 = (if v94 { self.scalar_v84 } else { v88 });
        let v103: f64 = (if v94 { self.scalar_v89 } else { v93 });
        let v108: f64 = 0.5;
        let v111: f64 = (self.scalar_v110 / v35);
        let v112: f64 = (if self.scalar_v107 { v111 } else { v11 });
        let v115: f64 = (if self.scalar_v113 { self.scalar_v114 } else { v112 });
        let v117: f64 = (v7 * self.scalar_v116);
        let v118: f64 = ((v117) as f64).cosh();
        let v120: f64 = (v118 * v118);
        let v121: f64 = (self.scalar_v119 / v120);
        let v122: f64 = (v42 + v121);
        let v123: f64 = (v96 * v122);
        let v125: f64 = (v101 - self.scalar_v124);
        let v127: f64 = (v7 * self.scalar_v126);
        let v128: f64 = ((v127) as f64).tanh();
        let v129: f64 = (self.scalar_v124 * v128);
        let v130: f64 = (v125 + v129);
        let v132: f64 = (v6 - self.scalar_v89);
        let v133: f64 = (self.scalar_v131 * v132);
        let v134: f64 = (v6 - v103);
        let v135: f64 = (v133 * v134);
        let v136: f64 = (v130 - v135);
        let v137: f64 = (v2 - v136);
        let v138: f64 = (v137 * v137);
        let v139: f64 = (v123 * v137);
        let v141: f64 = (v138 * self.scalar_v140);
        let v142: f64 = (v139 + v141);
        let v144: f64 = (v137 * self.scalar_v143);
        let v145: f64 = (v138 * v144);
        let v146: f64 = (v142 + v145);
        let v147: f64 = ((v146) as f64).tanh();
        let v148: f64 = (v42 + v147);
        let v149: f64 = { let limexp_arg = v146; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v150: f64 = (-v146);
        let v151: f64 = { let limexp_arg = v150; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v152: f64 = (v149 - v151);
        let v153: f64 = (v108 * v152);
        let v154: f64 = ((v153) as f64).tanh();
        let v155: f64 = (v42 + v154);
        let v157: f64 = (self.scalar_v126 * v148);
        let v158: f64 = (self.scalar_v156 + v157);
        let v159: f64 = (v7 * v158);
        let v160: f64 = ((v159) as f64).tanh();
        let v168: f64 = (v95 * v148);
        let v169: f64 = (v160 * v168);
        let v171: f64 = (v7 * self.scalar_v170);
        let v172: f64 = (v42 + v171);
        let v173: f64 = { let limexp_arg = v134; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v174: f64 = (v97 * v173);
        let v175: f64 = (v172 + v174);
        let v176: f64 = (v169 * v175);
        let v177: f64 = (if self.scalar_v162 { v176 } else { v11 });
        let v180: f64 = (v5 - v136);
        let v181: f64 = (if self.scalar_v179 { v180 } else { v118 });
        let v182: f64 = (v181 * v181);
        let v183: f64 = (if self.scalar_v179 { v182 } else { v137 });
        let v184: f64 = (v181 * v183);
        let v185: f64 = (if self.scalar_v179 { v184 } else { v138 });
        let v186: f64 = (v123 * v181);
        let v187: f64 = (self.scalar_v140 * v183);
        let v188: f64 = (v186 + v187);
        let v189: f64 = (self.scalar_v143 * v185);
        let v190: f64 = (v188 + v189);
        let v191: f64 = (if self.scalar_v179 { v190 } else { v11 });
        let v192: f64 = ((v191) as f64).tanh();
        let v193: f64 = (v42 + v192);
        let v194: f64 = (if self.scalar_v179 { v193 } else { v11 });
        let v195: f64 = (self.scalar_v126 * v194);
        let v196: f64 = (self.scalar_v156 + v195);
        let v197: f64 = (if self.scalar_v179 { v196 } else { v11 });
        let v199: f64 = (v148 * self.scalar_v198);
        let v200: f64 = (self.scalar_v170 + v199);
        let v201: f64 = (if self.scalar_v179 { v200 } else { v11 });
        let v202: f64 = (v42 + v160);
        let v203: f64 = (v168 * v202);
        let v204: f64 = (v7 * v201);
        let v205: f64 = (v42 + v204);
        let v206: f64 = (v7 - v103);
        let v207: f64 = { let limexp_arg = v206; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v208: f64 = (v97 * v207);
        let v209: f64 = (v205 + v208);
        let v210: f64 = (v203 * v209);
        let v211: f64 = (if self.scalar_v179 { v210 } else { v11 });
        let v212: f64 = (v194 * self.scalar_v198);
        let v213: f64 = (self.scalar_v170 + v212);
        let v214: f64 = (if self.scalar_v179 { v213 } else { v11 });
        let v215: f64 = (v7 * v197);
        let v216: f64 = ((v215) as f64).tanh();
        let v217: f64 = (if self.scalar_v179 { v216 } else { v11 });
        let v218: f64 = (v95 * v194);
        let v219: f64 = (v42 - v217);
        let v220: f64 = (v218 * v219);
        let v221: f64 = (v7 * v214);
        let v222: f64 = (v42 - v221);
        let v223: f64 = (v220 * v222);
        let v224: f64 = (if self.scalar_v179 { v223 } else { v11 });
        let v225: f64 = (v211 - v224);
        let v226: f64 = (v108 * v225);
        let v227: f64 = (if self.scalar_v179 { v226 } else { v177 });
        let v231: f64 = (if self.scalar_v230 { v137 } else { v181 });
        let v232: f64 = (v231 * v231);
        let v233: f64 = (if self.scalar_v230 { v232 } else { v183 });
        let v234: f64 = (self.scalar_v140 * v233);
        let v235: f64 = (v231 + v234);
        let v236: f64 = (self.scalar_v143 * v233);
        let v237: f64 = (v231 * v236);
        let v238: f64 = (v235 + v237);
        let v239: f64 = (v123 * v238);
        let v240: f64 = (if self.scalar_v230 { v239 } else { v146 });
        let v241: f64 = { let limexp_arg = v240; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v242: f64 = (-v240);
        let v243: f64 = { let limexp_arg = v242; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v244: f64 = (v241 - v243);
        let v245: f64 = (v108 * v244);
        let v246: f64 = ((v245) as f64).tanh();
        let v247: f64 = (v42 + v246);
        let v248: f64 = (if self.scalar_v230 { v247 } else { v155 });
        let v249: f64 = (self.scalar_v126 * v248);
        let v250: f64 = (self.scalar_v156 + v249);
        let v251: f64 = (if self.scalar_v230 { v250 } else { v11 });
        let v252: f64 = (v7 * v251);
        let v253: f64 = ((v252) as f64).tanh();
        let v254: f64 = (if self.scalar_v230 { v253 } else { v11 });
        let v255: f64 = (self.scalar_v198 * v248);
        let v256: f64 = (self.scalar_v170 + v255);
        let v257: f64 = (if self.scalar_v230 { v256 } else { v201 });
        let v258: f64 = (v95 * v248);
        let v259: f64 = (v254 * v258);
        let v260: f64 = (v7 * v257);
        let v261: f64 = (v42 + v260);
        let v262: f64 = (v174 + v261);
        let v263: f64 = (v259 * v262);
        let v264: f64 = (if self.scalar_v230 { v263 } else { v227 });
        let v268: f64 = (if self.scalar_v267 { v137 } else { v231 });
        let v269: f64 = (v268 * v268);
        let v270: f64 = (if self.scalar_v267 { v269 } else { v233 });
        let v271: f64 = (self.scalar_v140 * v270);
        let v272: f64 = (v268 + v271);
        let v273: f64 = (self.scalar_v143 * v270);
        let v274: f64 = (v268 * v273);
        let v275: f64 = (v272 + v274);
        let v276: f64 = (v123 * v275);
        let v277: f64 = (if self.scalar_v267 { v276 } else { v240 });
        let v278: f64 = (if self.scalar_v267 { v180 } else { v185 });
        let v279: f64 = (v278 * v278);
        let v280: f64 = (if self.scalar_v267 { v279 } else { v11 });
        let v281: f64 = (self.scalar_v140 * v280);
        let v282: f64 = (v278 + v281);
        let v283: f64 = (self.scalar_v143 * v278);
        let v284: f64 = (v280 * v283);
        let v285: f64 = (v282 + v284);
        let v286: f64 = (v123 * v285);
        let v287: f64 = (if self.scalar_v267 { v286 } else { v191 });
        let v288: f64 = { let limexp_arg = v277; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v289: f64 = (-v277);
        let v290: f64 = { let limexp_arg = v289; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v291: f64 = (v288 - v290);
        let v292: f64 = (v108 * v291);
        let v293: f64 = ((v292) as f64).tanh();
        let v294: f64 = (v42 + v293);
        let v295: f64 = (if self.scalar_v267 { v294 } else { v248 });
        let v296: f64 = { let limexp_arg = v287; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v297: f64 = (-v287);
        let v298: f64 = { let limexp_arg = v297; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v299: f64 = (v296 - v298);
        let v300: f64 = (v108 * v299);
        let v301: f64 = ((v300) as f64).tanh();
        let v302: f64 = (v42 + v301);
        let v303: f64 = (if self.scalar_v267 { v302 } else { v11 });
        let v304: f64 = (self.scalar_v126 * v295);
        let v305: f64 = (self.scalar_v156 + v304);
        let v306: f64 = (if self.scalar_v267 { v305 } else { v251 });
        let v307: f64 = (self.scalar_v126 * v303);
        let v308: f64 = (self.scalar_v156 + v307);
        let v309: f64 = (if self.scalar_v267 { v308 } else { v11 });
        let v310: f64 = (v7 * v306);
        let v311: f64 = ((v310) as f64).tanh();
        let v312: f64 = (if self.scalar_v267 { v311 } else { v254 });
        let v313: f64 = (v7 * v309);
        let v314: f64 = ((v313) as f64).tanh();
        let v315: f64 = (if self.scalar_v267 { v314 } else { v11 });
        let v316: f64 = (self.scalar_v198 * v303);
        let v317: f64 = (self.scalar_v170 + v316);
        let v318: f64 = (if self.scalar_v267 { v317 } else { v11 });
        let v319: f64 = (self.scalar_v198 * v295);
        let v320: f64 = (self.scalar_v170 + v319);
        let v321: f64 = (if self.scalar_v267 { v320 } else { v11 });
        let v322: f64 = (v95 * v295);
        let v323: f64 = (v42 + v312);
        let v324: f64 = (v322 * v323);
        let v325: f64 = (v7 * v321);
        let v326: f64 = (v42 + v325);
        let v327: f64 = (v208 + v326);
        let v328: f64 = (v324 * v327);
        let v329: f64 = (if self.scalar_v267 { v328 } else { v211 });
        let v330: f64 = (v95 * v303);
        let v331: f64 = (v42 - v315);
        let v332: f64 = (v330 * v331);
        let v333: f64 = (v7 * v318);
        let v334: f64 = (v42 - v333);
        let v335: f64 = (v332 * v334);
        let v336: f64 = (if self.scalar_v267 { v335 } else { v224 });
        let v337: f64 = (v329 - v336);
        let v338: f64 = (v108 * v337);
        let v339: f64 = (if self.scalar_v267 { v338 } else { v264 });
        let v341: f64 = (v42 + v148);
        let v342: f64 = (v99 / v341);
        let v343: f64 = (self.scalar_v340 + v342);
        let v344: f64 = (if self.scalar_v228 { v343 } else { v11 });
        let v348: f64 = (v42 + v295);
        let v349: f64 = (v99 / v348);
        let v350: f64 = (self.scalar_v340 + v349);
        let v351: f64 = (if self.scalar_v229 { v350 } else { v344 });
        let v354: f64 = -1.0;
        let v355: f64 = (-v102);
        let v356: f64 = ((v355) as f64).tanh();
        let v357: f64 = (v115 * v356);
        let v358: f64 = { let limexp_arg = v357; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v359: f64 = (if self.scalar_v353 { v358 } else { v268 });
        let v360: f64 = (v2 - v102);
        let v361: f64 = (if self.scalar_v353 { v360 } else { v11 });
        let v362: f64 = (v9 - v102);
        let v363: f64 = (if self.scalar_v353 { v362 } else { v11 });
        let v365: f64 = (-v115);
        let v366: f64 = (v102 * v365);
        let v367: f64 = { let limexp_arg = v366; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v368: f64 = (if self.scalar_v364 { v367 } else { v359 });
        let v371: f64 = ((v360) as f64).tanh();
        let v372: f64 = (if self.scalar_v370 { v371 } else { v361 });
        let v373: f64 = ((v362) as f64).tanh();
        let v374: f64 = (if self.scalar_v370 { v373 } else { v363 });
        let v377: f64 = (if self.scalar_v376 { v360 } else { v372 });
        let v378: f64 = (if self.scalar_v376 { v362 } else { v374 });
        let v380: f64 = (v115 * v377);
        let v381: f64 = { let limexp_arg = v380; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v382: f64 = (v381 - v368);
        let v383: f64 = (self.scalar_v379 * v382);
        let v384: f64 = (v115 * v378);
        let v385: f64 = { let limexp_arg = v384; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v386: f64 = (v385 - v368);
        let v387: f64 = (self.scalar_v379 * v386);
        let v412: f64 = 5.5226012e-23;
        let v413: f64 = (v33 * v412);
        let v417: f64 = (v413 * self.scalar_v416);
        let v418: f64 = (v98 * v417);
        let v421: f64 = (v418 * self.scalar_v420);
        let v422: f64 = (if self.scalar_v411 { v421 } else { v11 });
        let v423: f64 = (v422 * v422);
        let v424: f64 = (v42 - v423);
        let v425: f64 = ((v424) as f64).sqrt();
        let v426: f64 = (if self.scalar_v411 { v425 } else { v11 });
        let v427: f64 = (-v422);
        let v428: f64 = 3.141592653589793;
        let v429: f64 = (v427 * v428);
        let v430: f64 = (if self.scalar_v411 { v429 } else { v11 });
        let v434: f64 = (-v339);
        let v436: f64 = nv12;
        let v437: f64 = (self.scalar_v435 * v436);
        let v439: f64 = nv1;
        let v440: f64 = (v439 - v4);
        let v441: f64 = (self.scalar_v438 * v440);
        let v443: f64 = (v7 * self.scalar_v442);
        let v444: f64 = nv10;
        let v445: f64 = (v4 - v444);
        let v446: f64 = (v100 * v445);
        let v447: f64 = (v444 - v1);
        let v448: f64 = (v447 / v351);
        let v449: f64 = (if self.scalar_v388 { v448 } else { v11 });
        let v453: f64 = nv9;
        let v454: f64 = (v453 - v0);
        let v455: f64 = (self.scalar_v452 * v454);
        let v456: f64 = (v453 - v1);
        let v457: f64 = (v456 / self.scalar_v389);
        let v458: f64 = (if self.scalar_v390 { v457 } else { v11 });
        let v461: f64 = (v3 - v8);
        let v462: f64 = (v461 / self.scalar_v391);
        let v463: f64 = (if self.scalar_v392 { v462 } else { v11 });
        let v468: f64 = (v3 - v0);
        let v469: f64 = (v468 / self.scalar_v393);
        let v470: f64 = (if self.scalar_v394 { v469 } else { v11 });
        let v492: f64 = nv14;
        let v493: f64 = (if self.scalar_v411 { v492 } else { v11 });
        let v494: f64 = nv15;
        let v495: f64 = (if self.scalar_v411 { v494 } else { v11 });
        let v496: f64 = (v430 * v492);
        let v497: f64 = (v426 * v494);
        let v498: f64 = (v496 + v497);
        let v499: f64 = (if self.scalar_v411 { v498 } else { v11 });
        let v505: f64 = (-v10);
        let v506: f64 = (v7 * v505);
        let v507: f64 = (v2 * v383);
        let v508: f64 = (v506 + v507);
        let v509: f64 = ((v508) as f64).abs();
        let v510: f64 = (-v509);
        let v511: f64 = (if self.scalar_v433 { v510 } else { v11 });
        let v512: f64 = (v30 / self.scalar_v39);
        let v513: f64 = (if self.scalar_v433 { v512 } else { v11 });
        let v515: f64 = 1e-12;
        let v516: f64 = (v30 * v515);
        let v517: f64 = (if self.scalar_v514 { v516 } else { v11 });
        let v519: f64 = ((v117) as f64).sinh();
        let v520: f64 = (self.scalar_v116 * v519);
        let v521: f64 = (self.scalar_v518 * v519);
        let v522: f64 = (v118 * v520);
        let v523: f64 = (v522 + v522);
        let v524: f64 = (v118 * v521);
        let v525: f64 = (v524 + v524);
        let v526: f64 = (self.scalar_v119 * v523);
        let v527: f64 = (-v526);
        let v528: f64 = (v120 * v120);
        let v529: f64 = (v527 / v528);
        let v530: f64 = (self.scalar_v119 * v525);
        let v531: f64 = (-v530);
        let v532: f64 = (v531 / v528);
        let v533: f64 = (v96 * v529);
        let v534: f64 = (v96 * v532);
        let v536: f64 = (v128 * v128);
        let v537: f64 = (v42 - v536);
        let v538: f64 = (self.scalar_v126 * v537);
        let v539: f64 = (self.scalar_v535 * v537);
        let v540: f64 = (self.scalar_v124 * v538);
        let v541: f64 = (self.scalar_v124 * v539);
        let v543: f64 = (self.scalar_v131 * v134);
        let v544: f64 = (v133 + v543);
        let v545: f64 = (v134 * self.scalar_v542);
        let v546: f64 = (-v133);
        let v547: f64 = (v545 + v546);
        let v548: f64 = (v540 - v544);
        let v549: f64 = (-v547);
        let v550: f64 = (-v548);
        let v551: f64 = (v354 - v541);
        let v552: f64 = (v137 * v550);
        let v553: f64 = (v552 + v552);
        let v554: f64 = (v137 * v547);
        let v555: f64 = (v554 + v554);
        let v556: f64 = (v137 * v551);
        let v557: f64 = (v556 + v556);
        let v558: f64 = (v137 + v137);
        let v559: f64 = (v137 * v533);
        let v560: f64 = (v123 * v550);
        let v561: f64 = (v559 + v560);
        let v562: f64 = (v123 * v547);
        let v563: f64 = (v137 * v534);
        let v564: f64 = (v123 * v551);
        let v565: f64 = (v563 + v564);
        let v566: f64 = (self.scalar_v140 * v553);
        let v567: f64 = (self.scalar_v140 * v555);
        let v568: f64 = (self.scalar_v140 * v557);
        let v569: f64 = (self.scalar_v140 * v558);
        let v570: f64 = (v561 + v566);
        let v571: f64 = (v562 + v567);
        let v572: f64 = (v565 + v568);
        let v573: f64 = (v123 + v569);
        let v574: f64 = (self.scalar_v143 * v550);
        let v575: f64 = (self.scalar_v143 * v547);
        let v576: f64 = (self.scalar_v143 * v551);
        let v577: f64 = (v144 * v553);
        let v578: f64 = (v138 * v574);
        let v579: f64 = (v577 + v578);
        let v580: f64 = (v144 * v555);
        let v581: f64 = (v138 * v575);
        let v582: f64 = (v580 + v581);
        let v583: f64 = (v144 * v557);
        let v584: f64 = (v138 * v576);
        let v585: f64 = (v583 + v584);
        let v586: f64 = (v144 * v558);
        let v587: f64 = (v138 * self.scalar_v143);
        let v588: f64 = (v586 + v587);
        let v589: f64 = (v570 + v579);
        let v590: f64 = (v571 + v582);
        let v591: f64 = (v572 + v585);
        let v592: f64 = (v573 + v588);
        let v593: f64 = (v147 * v147);
        let v594: f64 = (v42 - v593);
        let v595: f64 = (v589 * v594);
        let v596: f64 = (v590 * v594);
        let v597: f64 = (v591 * v594);
        let v598: f64 = (v592 * v594);
        let v599: f64 = { let limexp_arg = v146; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v600: f64 = (v589 * v599);
        let v601: f64 = (v590 * v599);
        let v602: f64 = (v591 * v599);
        let v603: f64 = (v592 * v599);
        let v604: f64 = (-v589);
        let v605: f64 = (-v590);
        let v606: f64 = (-v591);
        let v607: f64 = (-v592);
        let v608: f64 = { let limexp_arg = v150; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v609: f64 = (v604 * v608);
        let v610: f64 = (v605 * v608);
        let v611: f64 = (v606 * v608);
        let v612: f64 = (v607 * v608);
        let v613: f64 = (v600 - v609);
        let v614: f64 = (v601 - v610);
        let v615: f64 = (v602 - v611);
        let v616: f64 = (v603 - v612);
        let v617: f64 = (v108 * v613);
        let v618: f64 = (v108 * v614);
        let v619: f64 = (v108 * v615);
        let v620: f64 = (v108 * v616);
        let v621: f64 = (v154 * v154);
        let v622: f64 = (v42 - v621);
        let v623: f64 = (v617 * v622);
        let v624: f64 = (v618 * v622);
        let v625: f64 = (v619 * v622);
        let v626: f64 = (v620 * v622);
        let v627: f64 = (self.scalar_v126 * v595);
        let v628: f64 = (self.scalar_v126 * v596);
        let v629: f64 = (self.scalar_v126 * v597);
        let v630: f64 = (self.scalar_v126 * v598);
        let v631: f64 = (v7 * v627);
        let v632: f64 = (v158 + v631);
        let v633: f64 = (v7 * v628);
        let v634: f64 = (-v158);
        let v635: f64 = (v7 * v629);
        let v636: f64 = (v634 + v635);
        let v637: f64 = (v7 * v630);
        let v638: f64 = (v160 * v160);
        let v639: f64 = (v42 - v638);
        let v640: f64 = (v632 * v639);
        let v641: f64 = (v633 * v639);
        let v642: f64 = (v636 * v639);
        let v643: f64 = (v637 * v639);
        let v644: f64 = (v95 * v595);
        let v645: f64 = (v95 * v596);
        let v646: f64 = (v95 * v597);
        let v647: f64 = (v95 * v598);
        let v648: f64 = (v168 * v640);
        let v649: f64 = (v160 * v644);
        let v650: f64 = (v648 + v649);
        let v651: f64 = (v168 * v641);
        let v652: f64 = (v160 * v645);
        let v653: f64 = (v651 + v652);
        let v654: f64 = (v168 * v642);
        let v655: f64 = (v160 * v646);
        let v656: f64 = (v654 + v655);
        let v657: f64 = (v168 * v643);
        let v658: f64 = (v160 * v647);
        let v659: f64 = (v657 + v658);
        let v661: f64 = { let limexp_arg = v134; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v662: f64 = (-v661);
        let v663: f64 = (v97 * v661);
        let v664: f64 = (v97 * v662);
        let v665: f64 = (self.scalar_v170 + v663);
        let v666: f64 = (v175 * v650);
        let v667: f64 = (v169 * v665);
        let v668: f64 = (v666 + v667);
        let v669: f64 = (v175 * v653);
        let v670: f64 = (v169 * v664);
        let v671: f64 = (v669 + v670);
        let v672: f64 = (v175 * v656);
        let v673: f64 = (v169 * self.scalar_v660);
        let v674: f64 = (v672 + v673);
        let v675: f64 = (v175 * v659);
        let v676: f64 = (if self.scalar_v162 { v668 } else { v11 });
        let v677: f64 = (if self.scalar_v162 { v671 } else { v11 });
        let v678: f64 = (if self.scalar_v162 { v674 } else { v11 });
        let v679: f64 = (if self.scalar_v162 { v675 } else { v11 });
        let v680: f64 = (v354 - v548);
        let v681: f64 = (v42 - v549);
        let v682: f64 = (-v541);
        let v683: f64 = (if self.scalar_v179 { v680 } else { v520 });
        let v684: f64 = (if self.scalar_v179 { v681 } else { v11 });
        let v685: f64 = (if self.scalar_v179 { v682 } else { v521 });
        let v686: f64 = (v181 * v683);
        let v687: f64 = (v686 + v686);
        let v688: f64 = (v181 * v684);
        let v689: f64 = (v688 + v688);
        let v690: f64 = (v181 * v685);
        let v691: f64 = (v690 + v690);
        let v692: f64 = (if self.scalar_v179 { v687 } else { v550 });
        let v693: f64 = (if self.scalar_v179 { v689 } else { v547 });
        let v694: f64 = (if self.scalar_v179 { v691 } else { v551 });
        let v696: f64 = (v183 * v683);
        let v697: f64 = (v181 * v692);
        let v698: f64 = (v696 + v697);
        let v699: f64 = (v183 * v684);
        let v700: f64 = (v181 * v693);
        let v701: f64 = (v699 + v700);
        let v702: f64 = (v183 * v685);
        let v703: f64 = (v181 * v694);
        let v704: f64 = (v702 + v703);
        let v705: f64 = (v181 * self.scalar_v695);
        let v706: f64 = (if self.scalar_v179 { v698 } else { v553 });
        let v707: f64 = (if self.scalar_v179 { v701 } else { v555 });
        let v708: f64 = (if self.scalar_v179 { v704 } else { v557 });
        let v709: f64 = (if self.scalar_v179 { v705 } else { v558 });
        let v710: f64 = (v181 * v533);
        let v711: f64 = (v123 * v683);
        let v712: f64 = (v710 + v711);
        let v713: f64 = (v123 * v684);
        let v714: f64 = (v181 * v534);
        let v715: f64 = (v123 * v685);
        let v716: f64 = (v714 + v715);
        let v717: f64 = (self.scalar_v140 * v692);
        let v718: f64 = (self.scalar_v140 * v693);
        let v719: f64 = (self.scalar_v140 * v694);
        let v721: f64 = (v712 + v717);
        let v722: f64 = (v713 + v718);
        let v723: f64 = (v716 + v719);
        let v724: f64 = (self.scalar_v143 * v706);
        let v725: f64 = (self.scalar_v143 * v707);
        let v726: f64 = (self.scalar_v143 * v708);
        let v727: f64 = (self.scalar_v143 * v709);
        let v728: f64 = (v721 + v724);
        let v729: f64 = (v722 + v725);
        let v730: f64 = (v723 + v726);
        let v731: f64 = (self.scalar_v720 + v727);
        let v732: f64 = (if self.scalar_v179 { v728 } else { v11 });
        let v733: f64 = (if self.scalar_v179 { v729 } else { v11 });
        let v734: f64 = (if self.scalar_v179 { v730 } else { v11 });
        let v735: f64 = (if self.scalar_v179 { v731 } else { v11 });
        let v736: f64 = (v192 * v192);
        let v737: f64 = (v42 - v736);
        let v738: f64 = (v732 * v737);
        let v739: f64 = (v733 * v737);
        let v740: f64 = (v734 * v737);
        let v741: f64 = (v735 * v737);
        let v742: f64 = (if self.scalar_v179 { v738 } else { v11 });
        let v743: f64 = (if self.scalar_v179 { v739 } else { v11 });
        let v744: f64 = (if self.scalar_v179 { v740 } else { v11 });
        let v745: f64 = (if self.scalar_v179 { v741 } else { v11 });
        let v746: f64 = (self.scalar_v126 * v742);
        let v747: f64 = (self.scalar_v126 * v743);
        let v748: f64 = (self.scalar_v126 * v744);
        let v749: f64 = (self.scalar_v126 * v745);
        let v750: f64 = (if self.scalar_v179 { v746 } else { v11 });
        let v751: f64 = (if self.scalar_v179 { v747 } else { v11 });
        let v752: f64 = (if self.scalar_v179 { v748 } else { v11 });
        let v753: f64 = (if self.scalar_v179 { v749 } else { v11 });
        let v754: f64 = (self.scalar_v198 * v595);
        let v755: f64 = (self.scalar_v198 * v596);
        let v756: f64 = (self.scalar_v198 * v597);
        let v757: f64 = (self.scalar_v198 * v598);
        let v758: f64 = (if self.scalar_v179 { v754 } else { v11 });
        let v759: f64 = (if self.scalar_v179 { v755 } else { v11 });
        let v760: f64 = (if self.scalar_v179 { v756 } else { v11 });
        let v761: f64 = (if self.scalar_v179 { v757 } else { v11 });
        let v762: f64 = (v202 * v644);
        let v763: f64 = (v648 + v762);
        let v764: f64 = (v202 * v645);
        let v765: f64 = (v651 + v764);
        let v766: f64 = (v202 * v646);
        let v767: f64 = (v654 + v766);
        let v768: f64 = (v202 * v647);
        let v769: f64 = (v657 + v768);
        let v770: f64 = (v7 * v758);
        let v771: f64 = (v201 + v770);
        let v772: f64 = (v7 * v759);
        let v773: f64 = (-v201);
        let v774: f64 = (v7 * v760);
        let v775: f64 = (v773 + v774);
        let v776: f64 = (v7 * v761);
        let v777: f64 = { let limexp_arg = v206; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v778: f64 = (-v777);
        let v779: f64 = (v97 * v777);
        let v780: f64 = (v97 * v778);
        let v781: f64 = (v771 + v779);
        let v782: f64 = (v775 + v780);
        let v783: f64 = (v209 * v763);
        let v784: f64 = (v203 * v781);
        let v785: f64 = (v783 + v784);
        let v786: f64 = (v209 * v765);
        let v787: f64 = (v203 * v772);
        let v788: f64 = (v786 + v787);
        let v789: f64 = (v209 * v767);
        let v790: f64 = (v203 * v782);
        let v791: f64 = (v789 + v790);
        let v792: f64 = (v209 * v769);
        let v793: f64 = (v203 * v776);
        let v794: f64 = (v792 + v793);
        let v795: f64 = (if self.scalar_v179 { v785 } else { v11 });
        let v796: f64 = (if self.scalar_v179 { v788 } else { v11 });
        let v797: f64 = (if self.scalar_v179 { v791 } else { v11 });
        let v798: f64 = (if self.scalar_v179 { v794 } else { v11 });
        let v799: f64 = (self.scalar_v198 * v742);
        let v800: f64 = (self.scalar_v198 * v743);
        let v801: f64 = (self.scalar_v198 * v744);
        let v802: f64 = (self.scalar_v198 * v745);
        let v803: f64 = (if self.scalar_v179 { v799 } else { v11 });
        let v804: f64 = (if self.scalar_v179 { v800 } else { v11 });
        let v805: f64 = (if self.scalar_v179 { v801 } else { v11 });
        let v806: f64 = (if self.scalar_v179 { v802 } else { v11 });
        let v807: f64 = (v7 * v750);
        let v808: f64 = (v197 + v807);
        let v809: f64 = (v7 * v751);
        let v810: f64 = (-v197);
        let v811: f64 = (v7 * v752);
        let v812: f64 = (v810 + v811);
        let v813: f64 = (v7 * v753);
        let v814: f64 = (v216 * v216);
        let v815: f64 = (v42 - v814);
        let v816: f64 = (v808 * v815);
        let v817: f64 = (v809 * v815);
        let v818: f64 = (v812 * v815);
        let v819: f64 = (v813 * v815);
        let v820: f64 = (if self.scalar_v179 { v816 } else { v11 });
        let v821: f64 = (if self.scalar_v179 { v817 } else { v11 });
        let v822: f64 = (if self.scalar_v179 { v818 } else { v11 });
        let v823: f64 = (if self.scalar_v179 { v819 } else { v11 });
        let v824: f64 = (v95 * v742);
        let v825: f64 = (v95 * v743);
        let v826: f64 = (v95 * v744);
        let v827: f64 = (v95 * v745);
        let v828: f64 = (-v820);
        let v829: f64 = (-v821);
        let v830: f64 = (-v822);
        let v831: f64 = (-v823);
        let v832: f64 = (v219 * v824);
        let v833: f64 = (v218 * v828);
        let v834: f64 = (v832 + v833);
        let v835: f64 = (v219 * v825);
        let v836: f64 = (v218 * v829);
        let v837: f64 = (v835 + v836);
        let v838: f64 = (v219 * v826);
        let v839: f64 = (v218 * v830);
        let v840: f64 = (v838 + v839);
        let v841: f64 = (v219 * v827);
        let v842: f64 = (v218 * v831);
        let v843: f64 = (v841 + v842);
        let v844: f64 = (v7 * v803);
        let v845: f64 = (v214 + v844);
        let v846: f64 = (v7 * v804);
        let v847: f64 = (-v214);
        let v848: f64 = (v7 * v805);
        let v849: f64 = (v847 + v848);
        let v850: f64 = (v7 * v806);
        let v851: f64 = (-v845);
        let v852: f64 = (-v846);
        let v853: f64 = (-v849);
        let v854: f64 = (-v850);
        let v855: f64 = (v222 * v834);
        let v856: f64 = (v220 * v851);
        let v857: f64 = (v855 + v856);
        let v858: f64 = (v222 * v837);
        let v859: f64 = (v220 * v852);
        let v860: f64 = (v858 + v859);
        let v861: f64 = (v222 * v840);
        let v862: f64 = (v220 * v853);
        let v863: f64 = (v861 + v862);
        let v864: f64 = (v222 * v843);
        let v865: f64 = (v220 * v854);
        let v866: f64 = (v864 + v865);
        let v867: f64 = (if self.scalar_v179 { v857 } else { v11 });
        let v868: f64 = (if self.scalar_v179 { v860 } else { v11 });
        let v869: f64 = (if self.scalar_v179 { v863 } else { v11 });
        let v870: f64 = (if self.scalar_v179 { v866 } else { v11 });
        let v871: f64 = (v795 - v867);
        let v872: f64 = (v796 - v868);
        let v873: f64 = (v797 - v869);
        let v874: f64 = (v798 - v870);
        let v875: f64 = (v108 * v871);
        let v876: f64 = (v108 * v872);
        let v877: f64 = (v108 * v873);
        let v878: f64 = (v108 * v874);
        let v879: f64 = (if self.scalar_v179 { v875 } else { v676 });
        let v880: f64 = (if self.scalar_v179 { v876 } else { v677 });
        let v881: f64 = (if self.scalar_v179 { v877 } else { v678 });
        let v882: f64 = (if self.scalar_v179 { v878 } else { v679 });
        let v883: f64 = (if self.scalar_v230 { v550 } else { v683 });
        let v884: f64 = (if self.scalar_v230 { v547 } else { v684 });
        let v885: f64 = (if self.scalar_v230 { v551 } else { v685 });
        let v887: f64 = (v231 * v883);
        let v888: f64 = (v887 + v887);
        let v889: f64 = (v231 * v884);
        let v890: f64 = (v889 + v889);
        let v891: f64 = (v231 * v885);
        let v892: f64 = (v891 + v891);
        let v893: f64 = (v231 * self.scalar_v886);
        let v894: f64 = (v893 + v893);
        let v895: f64 = (if self.scalar_v230 { v888 } else { v692 });
        let v896: f64 = (if self.scalar_v230 { v890 } else { v693 });
        let v897: f64 = (if self.scalar_v230 { v892 } else { v694 });
        let v898: f64 = (if self.scalar_v230 { v894 } else { self.scalar_v695 });
        let v899: f64 = (self.scalar_v140 * v895);
        let v900: f64 = (self.scalar_v140 * v896);
        let v901: f64 = (self.scalar_v140 * v897);
        let v902: f64 = (self.scalar_v140 * v898);
        let v903: f64 = (v883 + v899);
        let v904: f64 = (v884 + v900);
        let v905: f64 = (v885 + v901);
        let v906: f64 = (self.scalar_v886 + v902);
        let v907: f64 = (self.scalar_v143 * v895);
        let v908: f64 = (self.scalar_v143 * v896);
        let v909: f64 = (self.scalar_v143 * v897);
        let v910: f64 = (self.scalar_v143 * v898);
        let v911: f64 = (v236 * v883);
        let v912: f64 = (v231 * v907);
        let v913: f64 = (v911 + v912);
        let v914: f64 = (v236 * v884);
        let v915: f64 = (v231 * v908);
        let v916: f64 = (v914 + v915);
        let v917: f64 = (v236 * v885);
        let v918: f64 = (v231 * v909);
        let v919: f64 = (v917 + v918);
        let v920: f64 = (v236 * self.scalar_v886);
        let v921: f64 = (v231 * v910);
        let v922: f64 = (v920 + v921);
        let v923: f64 = (v903 + v913);
        let v924: f64 = (v904 + v916);
        let v925: f64 = (v905 + v919);
        let v926: f64 = (v906 + v922);
        let v927: f64 = (v238 * v533);
        let v928: f64 = (v123 * v923);
        let v929: f64 = (v927 + v928);
        let v930: f64 = (v123 * v924);
        let v931: f64 = (v238 * v534);
        let v932: f64 = (v123 * v925);
        let v933: f64 = (v931 + v932);
        let v934: f64 = (v123 * v926);
        let v935: f64 = (if self.scalar_v230 { v929 } else { v589 });
        let v936: f64 = (if self.scalar_v230 { v930 } else { v590 });
        let v937: f64 = (if self.scalar_v230 { v933 } else { v591 });
        let v938: f64 = (if self.scalar_v230 { v934 } else { v592 });
        let v939: f64 = { let limexp_arg = v240; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v940: f64 = (v935 * v939);
        let v941: f64 = (v936 * v939);
        let v942: f64 = (v937 * v939);
        let v943: f64 = (v938 * v939);
        let v944: f64 = (-v935);
        let v945: f64 = (-v936);
        let v946: f64 = (-v937);
        let v947: f64 = (-v938);
        let v948: f64 = { let limexp_arg = v242; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v949: f64 = (v944 * v948);
        let v950: f64 = (v945 * v948);
        let v951: f64 = (v946 * v948);
        let v952: f64 = (v947 * v948);
        let v953: f64 = (v940 - v949);
        let v954: f64 = (v941 - v950);
        let v955: f64 = (v942 - v951);
        let v956: f64 = (v943 - v952);
        let v957: f64 = (v108 * v953);
        let v958: f64 = (v108 * v954);
        let v959: f64 = (v108 * v955);
        let v960: f64 = (v108 * v956);
        let v961: f64 = (v246 * v246);
        let v962: f64 = (v42 - v961);
        let v963: f64 = (v957 * v962);
        let v964: f64 = (v958 * v962);
        let v965: f64 = (v959 * v962);
        let v966: f64 = (v960 * v962);
        let v967: f64 = (if self.scalar_v230 { v963 } else { v623 });
        let v968: f64 = (if self.scalar_v230 { v964 } else { v624 });
        let v969: f64 = (if self.scalar_v230 { v965 } else { v625 });
        let v970: f64 = (if self.scalar_v230 { v966 } else { v626 });
        let v971: f64 = (self.scalar_v126 * v967);
        let v972: f64 = (self.scalar_v126 * v968);
        let v973: f64 = (self.scalar_v126 * v969);
        let v974: f64 = (self.scalar_v126 * v970);
        let v975: f64 = (if self.scalar_v230 { v971 } else { v11 });
        let v976: f64 = (if self.scalar_v230 { v972 } else { v11 });
        let v977: f64 = (if self.scalar_v230 { v973 } else { v11 });
        let v978: f64 = (if self.scalar_v230 { v974 } else { v11 });
        let v979: f64 = (v7 * v975);
        let v980: f64 = (v251 + v979);
        let v981: f64 = (v7 * v976);
        let v982: f64 = (-v251);
        let v983: f64 = (v7 * v977);
        let v984: f64 = (v982 + v983);
        let v985: f64 = (v7 * v978);
        let v986: f64 = (v253 * v253);
        let v987: f64 = (v42 - v986);
        let v988: f64 = (v980 * v987);
        let v989: f64 = (v981 * v987);
        let v990: f64 = (v984 * v987);
        let v991: f64 = (v985 * v987);
        let v992: f64 = (if self.scalar_v230 { v988 } else { v11 });
        let v993: f64 = (if self.scalar_v230 { v989 } else { v11 });
        let v994: f64 = (if self.scalar_v230 { v990 } else { v11 });
        let v995: f64 = (if self.scalar_v230 { v991 } else { v11 });
        let v996: f64 = (self.scalar_v198 * v967);
        let v997: f64 = (self.scalar_v198 * v968);
        let v998: f64 = (self.scalar_v198 * v969);
        let v999: f64 = (self.scalar_v198 * v970);
        let v1000: f64 = (if self.scalar_v230 { v996 } else { v758 });
        let v1001: f64 = (if self.scalar_v230 { v997 } else { v759 });
        let v1002: f64 = (if self.scalar_v230 { v998 } else { v760 });
        let v1003: f64 = (if self.scalar_v230 { v999 } else { v761 });
        let v1004: f64 = (v95 * v967);
        let v1005: f64 = (v95 * v968);
        let v1006: f64 = (v95 * v969);
        let v1007: f64 = (v95 * v970);
        let v1008: f64 = (v258 * v992);
        let v1009: f64 = (v254 * v1004);
        let v1010: f64 = (v1008 + v1009);
        let v1011: f64 = (v258 * v993);
        let v1012: f64 = (v254 * v1005);
        let v1013: f64 = (v1011 + v1012);
        let v1014: f64 = (v258 * v994);
        let v1015: f64 = (v254 * v1006);
        let v1016: f64 = (v1014 + v1015);
        let v1017: f64 = (v258 * v995);
        let v1018: f64 = (v254 * v1007);
        let v1019: f64 = (v1017 + v1018);
        let v1020: f64 = (v7 * v1000);
        let v1021: f64 = (v257 + v1020);
        let v1022: f64 = (v7 * v1001);
        let v1023: f64 = (-v257);
        let v1024: f64 = (v7 * v1002);
        let v1025: f64 = (v1023 + v1024);
        let v1026: f64 = (v7 * v1003);
        let v1027: f64 = (v663 + v1021);
        let v1028: f64 = (v664 + v1022);
        let v1029: f64 = (v262 * v1010);
        let v1030: f64 = (v259 * v1027);
        let v1031: f64 = (v1029 + v1030);
        let v1032: f64 = (v262 * v1013);
        let v1033: f64 = (v259 * v1028);
        let v1034: f64 = (v1032 + v1033);
        let v1035: f64 = (v262 * v1016);
        let v1036: f64 = (v259 * v1025);
        let v1037: f64 = (v1035 + v1036);
        let v1038: f64 = (v262 * v1019);
        let v1039: f64 = (v259 * v1026);
        let v1040: f64 = (v1038 + v1039);
        let v1041: f64 = (if self.scalar_v230 { v1031 } else { v879 });
        let v1042: f64 = (if self.scalar_v230 { v1034 } else { v880 });
        let v1043: f64 = (if self.scalar_v230 { v1037 } else { v881 });
        let v1044: f64 = (if self.scalar_v230 { v1040 } else { v882 });
        let v1045: f64 = (if self.scalar_v267 { v550 } else { v883 });
        let v1046: f64 = (if self.scalar_v267 { v547 } else { v884 });
        let v1047: f64 = (if self.scalar_v267 { v551 } else { v885 });
        let v1049: f64 = (v268 * v1045);
        let v1050: f64 = (v1049 + v1049);
        let v1051: f64 = (v268 * v1046);
        let v1052: f64 = (v1051 + v1051);
        let v1053: f64 = (v268 * v1047);
        let v1054: f64 = (v1053 + v1053);
        let v1055: f64 = (v268 * self.scalar_v1048);
        let v1056: f64 = (v1055 + v1055);
        let v1057: f64 = (if self.scalar_v267 { v1050 } else { v895 });
        let v1058: f64 = (if self.scalar_v267 { v1052 } else { v896 });
        let v1059: f64 = (if self.scalar_v267 { v1054 } else { v897 });
        let v1060: f64 = (if self.scalar_v267 { v1056 } else { v898 });
        let v1061: f64 = (self.scalar_v140 * v1057);
        let v1062: f64 = (self.scalar_v140 * v1058);
        let v1063: f64 = (self.scalar_v140 * v1059);
        let v1064: f64 = (self.scalar_v140 * v1060);
        let v1065: f64 = (v1045 + v1061);
        let v1066: f64 = (v1046 + v1062);
        let v1067: f64 = (v1047 + v1063);
        let v1068: f64 = (self.scalar_v1048 + v1064);
        let v1069: f64 = (self.scalar_v143 * v1057);
        let v1070: f64 = (self.scalar_v143 * v1058);
        let v1071: f64 = (self.scalar_v143 * v1059);
        let v1072: f64 = (self.scalar_v143 * v1060);
        let v1073: f64 = (v273 * v1045);
        let v1074: f64 = (v268 * v1069);
        let v1075: f64 = (v1073 + v1074);
        let v1076: f64 = (v273 * v1046);
        let v1077: f64 = (v268 * v1070);
        let v1078: f64 = (v1076 + v1077);
        let v1079: f64 = (v273 * v1047);
        let v1080: f64 = (v268 * v1071);
        let v1081: f64 = (v1079 + v1080);
        let v1082: f64 = (v273 * self.scalar_v1048);
        let v1083: f64 = (v268 * v1072);
        let v1084: f64 = (v1082 + v1083);
        let v1085: f64 = (v1065 + v1075);
        let v1086: f64 = (v1066 + v1078);
        let v1087: f64 = (v1067 + v1081);
        let v1088: f64 = (v1068 + v1084);
        let v1089: f64 = (v275 * v533);
        let v1090: f64 = (v123 * v1085);
        let v1091: f64 = (v1089 + v1090);
        let v1092: f64 = (v123 * v1086);
        let v1093: f64 = (v275 * v534);
        let v1094: f64 = (v123 * v1087);
        let v1095: f64 = (v1093 + v1094);
        let v1096: f64 = (v123 * v1088);
        let v1097: f64 = (if self.scalar_v267 { v1091 } else { v935 });
        let v1098: f64 = (if self.scalar_v267 { v1092 } else { v936 });
        let v1099: f64 = (if self.scalar_v267 { v1095 } else { v937 });
        let v1100: f64 = (if self.scalar_v267 { v1096 } else { v938 });
        let v1101: f64 = (if self.scalar_v267 { v680 } else { v706 });
        let v1102: f64 = (if self.scalar_v267 { v681 } else { v707 });
        let v1103: f64 = (if self.scalar_v267 { v682 } else { v708 });
        let v1104: f64 = (if self.scalar_v267 { v11 } else { v709 });
        let v1105: f64 = (v278 * v1101);
        let v1106: f64 = (v1105 + v1105);
        let v1107: f64 = (v278 * v1102);
        let v1108: f64 = (v1107 + v1107);
        let v1109: f64 = (v278 * v1103);
        let v1110: f64 = (v1109 + v1109);
        let v1111: f64 = (v278 * v1104);
        let v1112: f64 = (v1111 + v1111);
        let v1113: f64 = (if self.scalar_v267 { v1106 } else { v11 });
        let v1114: f64 = (if self.scalar_v267 { v1108 } else { v11 });
        let v1115: f64 = (if self.scalar_v267 { v1110 } else { v11 });
        let v1116: f64 = (if self.scalar_v267 { v1112 } else { v11 });
        let v1117: f64 = (self.scalar_v140 * v1113);
        let v1118: f64 = (self.scalar_v140 * v1114);
        let v1119: f64 = (self.scalar_v140 * v1115);
        let v1120: f64 = (self.scalar_v140 * v1116);
        let v1121: f64 = (v1101 + v1117);
        let v1122: f64 = (v1102 + v1118);
        let v1123: f64 = (v1103 + v1119);
        let v1124: f64 = (v1104 + v1120);
        let v1125: f64 = (self.scalar_v143 * v1101);
        let v1126: f64 = (self.scalar_v143 * v1102);
        let v1127: f64 = (self.scalar_v143 * v1103);
        let v1128: f64 = (self.scalar_v143 * v1104);
        let v1129: f64 = (v283 * v1113);
        let v1130: f64 = (v280 * v1125);
        let v1131: f64 = (v1129 + v1130);
        let v1132: f64 = (v283 * v1114);
        let v1133: f64 = (v280 * v1126);
        let v1134: f64 = (v1132 + v1133);
        let v1135: f64 = (v283 * v1115);
        let v1136: f64 = (v280 * v1127);
        let v1137: f64 = (v1135 + v1136);
        let v1138: f64 = (v283 * v1116);
        let v1139: f64 = (v280 * v1128);
        let v1140: f64 = (v1138 + v1139);
        let v1141: f64 = (v1121 + v1131);
        let v1142: f64 = (v1122 + v1134);
        let v1143: f64 = (v1123 + v1137);
        let v1144: f64 = (v1124 + v1140);
        let v1145: f64 = (v285 * v533);
        let v1146: f64 = (v123 * v1141);
        let v1147: f64 = (v1145 + v1146);
        let v1148: f64 = (v123 * v1142);
        let v1149: f64 = (v285 * v534);
        let v1150: f64 = (v123 * v1143);
        let v1151: f64 = (v1149 + v1150);
        let v1152: f64 = (v123 * v1144);
        let v1153: f64 = (if self.scalar_v267 { v1147 } else { v732 });
        let v1154: f64 = (if self.scalar_v267 { v1148 } else { v733 });
        let v1155: f64 = (if self.scalar_v267 { v1151 } else { v734 });
        let v1156: f64 = (if self.scalar_v267 { v1152 } else { v735 });
        let v1157: f64 = { let limexp_arg = v277; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1158: f64 = (v1097 * v1157);
        let v1159: f64 = (v1098 * v1157);
        let v1160: f64 = (v1099 * v1157);
        let v1161: f64 = (v1100 * v1157);
        let v1162: f64 = (-v1097);
        let v1163: f64 = (-v1098);
        let v1164: f64 = (-v1099);
        let v1165: f64 = (-v1100);
        let v1166: f64 = { let limexp_arg = v289; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1167: f64 = (v1162 * v1166);
        let v1168: f64 = (v1163 * v1166);
        let v1169: f64 = (v1164 * v1166);
        let v1170: f64 = (v1165 * v1166);
        let v1171: f64 = (v1158 - v1167);
        let v1172: f64 = (v1159 - v1168);
        let v1173: f64 = (v1160 - v1169);
        let v1174: f64 = (v1161 - v1170);
        let v1175: f64 = (v108 * v1171);
        let v1176: f64 = (v108 * v1172);
        let v1177: f64 = (v108 * v1173);
        let v1178: f64 = (v108 * v1174);
        let v1179: f64 = (v293 * v293);
        let v1180: f64 = (v42 - v1179);
        let v1181: f64 = (v1175 * v1180);
        let v1182: f64 = (v1176 * v1180);
        let v1183: f64 = (v1177 * v1180);
        let v1184: f64 = (v1178 * v1180);
        let v1185: f64 = (if self.scalar_v267 { v1181 } else { v967 });
        let v1186: f64 = (if self.scalar_v267 { v1182 } else { v968 });
        let v1187: f64 = (if self.scalar_v267 { v1183 } else { v969 });
        let v1188: f64 = (if self.scalar_v267 { v1184 } else { v970 });
        let v1189: f64 = { let limexp_arg = v287; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1190: f64 = (v1153 * v1189);
        let v1191: f64 = (v1154 * v1189);
        let v1192: f64 = (v1155 * v1189);
        let v1193: f64 = (v1156 * v1189);
        let v1194: f64 = (-v1153);
        let v1195: f64 = (-v1154);
        let v1196: f64 = (-v1155);
        let v1197: f64 = (-v1156);
        let v1198: f64 = { let limexp_arg = v297; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1199: f64 = (v1194 * v1198);
        let v1200: f64 = (v1195 * v1198);
        let v1201: f64 = (v1196 * v1198);
        let v1202: f64 = (v1197 * v1198);
        let v1203: f64 = (v1190 - v1199);
        let v1204: f64 = (v1191 - v1200);
        let v1205: f64 = (v1192 - v1201);
        let v1206: f64 = (v1193 - v1202);
        let v1207: f64 = (v108 * v1203);
        let v1208: f64 = (v108 * v1204);
        let v1209: f64 = (v108 * v1205);
        let v1210: f64 = (v108 * v1206);
        let v1211: f64 = (v301 * v301);
        let v1212: f64 = (v42 - v1211);
        let v1213: f64 = (v1207 * v1212);
        let v1214: f64 = (v1208 * v1212);
        let v1215: f64 = (v1209 * v1212);
        let v1216: f64 = (v1210 * v1212);
        let v1217: f64 = (if self.scalar_v267 { v1213 } else { v11 });
        let v1218: f64 = (if self.scalar_v267 { v1214 } else { v11 });
        let v1219: f64 = (if self.scalar_v267 { v1215 } else { v11 });
        let v1220: f64 = (if self.scalar_v267 { v1216 } else { v11 });
        let v1221: f64 = (self.scalar_v126 * v1185);
        let v1222: f64 = (self.scalar_v126 * v1186);
        let v1223: f64 = (self.scalar_v126 * v1187);
        let v1224: f64 = (self.scalar_v126 * v1188);
        let v1225: f64 = (if self.scalar_v267 { v1221 } else { v975 });
        let v1226: f64 = (if self.scalar_v267 { v1222 } else { v976 });
        let v1227: f64 = (if self.scalar_v267 { v1223 } else { v977 });
        let v1228: f64 = (if self.scalar_v267 { v1224 } else { v978 });
        let v1229: f64 = (self.scalar_v126 * v1217);
        let v1230: f64 = (self.scalar_v126 * v1218);
        let v1231: f64 = (self.scalar_v126 * v1219);
        let v1232: f64 = (self.scalar_v126 * v1220);
        let v1233: f64 = (if self.scalar_v267 { v1229 } else { v11 });
        let v1234: f64 = (if self.scalar_v267 { v1230 } else { v11 });
        let v1235: f64 = (if self.scalar_v267 { v1231 } else { v11 });
        let v1236: f64 = (if self.scalar_v267 { v1232 } else { v11 });
        let v1237: f64 = (v7 * v1225);
        let v1238: f64 = (v306 + v1237);
        let v1239: f64 = (v7 * v1226);
        let v1240: f64 = (-v306);
        let v1241: f64 = (v7 * v1227);
        let v1242: f64 = (v1240 + v1241);
        let v1243: f64 = (v7 * v1228);
        let v1244: f64 = (v311 * v311);
        let v1245: f64 = (v42 - v1244);
        let v1246: f64 = (v1238 * v1245);
        let v1247: f64 = (v1239 * v1245);
        let v1248: f64 = (v1242 * v1245);
        let v1249: f64 = (v1243 * v1245);
        let v1250: f64 = (if self.scalar_v267 { v1246 } else { v992 });
        let v1251: f64 = (if self.scalar_v267 { v1247 } else { v993 });
        let v1252: f64 = (if self.scalar_v267 { v1248 } else { v994 });
        let v1253: f64 = (if self.scalar_v267 { v1249 } else { v995 });
        let v1254: f64 = (v7 * v1233);
        let v1255: f64 = (v309 + v1254);
        let v1256: f64 = (v7 * v1234);
        let v1257: f64 = (-v309);
        let v1258: f64 = (v7 * v1235);
        let v1259: f64 = (v1257 + v1258);
        let v1260: f64 = (v7 * v1236);
        let v1261: f64 = (v314 * v314);
        let v1262: f64 = (v42 - v1261);
        let v1263: f64 = (v1255 * v1262);
        let v1264: f64 = (v1256 * v1262);
        let v1265: f64 = (v1259 * v1262);
        let v1266: f64 = (v1260 * v1262);
        let v1267: f64 = (if self.scalar_v267 { v1263 } else { v11 });
        let v1268: f64 = (if self.scalar_v267 { v1264 } else { v11 });
        let v1269: f64 = (if self.scalar_v267 { v1265 } else { v11 });
        let v1270: f64 = (if self.scalar_v267 { v1266 } else { v11 });
        let v1271: f64 = (self.scalar_v198 * v1217);
        let v1272: f64 = (self.scalar_v198 * v1218);
        let v1273: f64 = (self.scalar_v198 * v1219);
        let v1274: f64 = (self.scalar_v198 * v1220);
        let v1275: f64 = (if self.scalar_v267 { v1271 } else { v11 });
        let v1276: f64 = (if self.scalar_v267 { v1272 } else { v11 });
        let v1277: f64 = (if self.scalar_v267 { v1273 } else { v11 });
        let v1278: f64 = (if self.scalar_v267 { v1274 } else { v11 });
        let v1279: f64 = (self.scalar_v198 * v1185);
        let v1280: f64 = (self.scalar_v198 * v1186);
        let v1281: f64 = (self.scalar_v198 * v1187);
        let v1282: f64 = (self.scalar_v198 * v1188);
        let v1283: f64 = (if self.scalar_v267 { v1279 } else { v11 });
        let v1284: f64 = (if self.scalar_v267 { v1280 } else { v11 });
        let v1285: f64 = (if self.scalar_v267 { v1281 } else { v11 });
        let v1286: f64 = (if self.scalar_v267 { v1282 } else { v11 });
        let v1287: f64 = (v95 * v1185);
        let v1288: f64 = (v95 * v1186);
        let v1289: f64 = (v95 * v1187);
        let v1290: f64 = (v95 * v1188);
        let v1291: f64 = (v323 * v1287);
        let v1292: f64 = (v322 * v1250);
        let v1293: f64 = (v1291 + v1292);
        let v1294: f64 = (v323 * v1288);
        let v1295: f64 = (v322 * v1251);
        let v1296: f64 = (v1294 + v1295);
        let v1297: f64 = (v323 * v1289);
        let v1298: f64 = (v322 * v1252);
        let v1299: f64 = (v1297 + v1298);
        let v1300: f64 = (v323 * v1290);
        let v1301: f64 = (v322 * v1253);
        let v1302: f64 = (v1300 + v1301);
        let v1303: f64 = (v7 * v1283);
        let v1304: f64 = (v321 + v1303);
        let v1305: f64 = (v7 * v1284);
        let v1306: f64 = (-v321);
        let v1307: f64 = (v7 * v1285);
        let v1308: f64 = (v1306 + v1307);
        let v1309: f64 = (v7 * v1286);
        let v1310: f64 = (v779 + v1304);
        let v1311: f64 = (v780 + v1308);
        let v1312: f64 = (v327 * v1293);
        let v1313: f64 = (v324 * v1310);
        let v1314: f64 = (v1312 + v1313);
        let v1315: f64 = (v327 * v1296);
        let v1316: f64 = (v324 * v1305);
        let v1317: f64 = (v1315 + v1316);
        let v1318: f64 = (v327 * v1299);
        let v1319: f64 = (v324 * v1311);
        let v1320: f64 = (v1318 + v1319);
        let v1321: f64 = (v327 * v1302);
        let v1322: f64 = (v324 * v1309);
        let v1323: f64 = (v1321 + v1322);
        let v1324: f64 = (if self.scalar_v267 { v1314 } else { v795 });
        let v1325: f64 = (if self.scalar_v267 { v1317 } else { v796 });
        let v1326: f64 = (if self.scalar_v267 { v1320 } else { v797 });
        let v1327: f64 = (if self.scalar_v267 { v1323 } else { v798 });
        let v1328: f64 = (v95 * v1217);
        let v1329: f64 = (v95 * v1218);
        let v1330: f64 = (v95 * v1219);
        let v1331: f64 = (v95 * v1220);
        let v1332: f64 = (-v1267);
        let v1333: f64 = (-v1268);
        let v1334: f64 = (-v1269);
        let v1335: f64 = (-v1270);
        let v1336: f64 = (v331 * v1328);
        let v1337: f64 = (v330 * v1332);
        let v1338: f64 = (v1336 + v1337);
        let v1339: f64 = (v331 * v1329);
        let v1340: f64 = (v330 * v1333);
        let v1341: f64 = (v1339 + v1340);
        let v1342: f64 = (v331 * v1330);
        let v1343: f64 = (v330 * v1334);
        let v1344: f64 = (v1342 + v1343);
        let v1345: f64 = (v331 * v1331);
        let v1346: f64 = (v330 * v1335);
        let v1347: f64 = (v1345 + v1346);
        let v1348: f64 = (v7 * v1275);
        let v1349: f64 = (v318 + v1348);
        let v1350: f64 = (v7 * v1276);
        let v1351: f64 = (-v318);
        let v1352: f64 = (v7 * v1277);
        let v1353: f64 = (v1351 + v1352);
        let v1354: f64 = (v7 * v1278);
        let v1355: f64 = (-v1349);
        let v1356: f64 = (-v1350);
        let v1357: f64 = (-v1353);
        let v1358: f64 = (-v1354);
        let v1359: f64 = (v334 * v1338);
        let v1360: f64 = (v332 * v1355);
        let v1361: f64 = (v1359 + v1360);
        let v1362: f64 = (v334 * v1341);
        let v1363: f64 = (v332 * v1356);
        let v1364: f64 = (v1362 + v1363);
        let v1365: f64 = (v334 * v1344);
        let v1366: f64 = (v332 * v1357);
        let v1367: f64 = (v1365 + v1366);
        let v1368: f64 = (v334 * v1347);
        let v1369: f64 = (v332 * v1358);
        let v1370: f64 = (v1368 + v1369);
        let v1371: f64 = (if self.scalar_v267 { v1361 } else { v867 });
        let v1372: f64 = (if self.scalar_v267 { v1364 } else { v868 });
        let v1373: f64 = (if self.scalar_v267 { v1367 } else { v869 });
        let v1374: f64 = (if self.scalar_v267 { v1370 } else { v870 });
        let v1375: f64 = (v1324 - v1371);
        let v1376: f64 = (v1325 - v1372);
        let v1377: f64 = (v1326 - v1373);
        let v1378: f64 = (v1327 - v1374);
        let v1379: f64 = (v108 * v1375);
        let v1380: f64 = (v108 * v1376);
        let v1381: f64 = (v108 * v1377);
        let v1382: f64 = (v108 * v1378);
        let v1383: f64 = (if self.scalar_v267 { v1379 } else { v1041 });
        let v1384: f64 = (if self.scalar_v267 { v1380 } else { v1042 });
        let v1385: f64 = (if self.scalar_v267 { v1381 } else { v1043 });
        let v1386: f64 = (if self.scalar_v267 { v1382 } else { v1044 });
        let v1387: f64 = (v99 * v595);
        let v1388: f64 = (-v1387);
        let v1389: f64 = (v341 * v341);
        let v1390: f64 = (v1388 / v1389);
        let v1391: f64 = (v99 * v596);
        let v1392: f64 = (-v1391);
        let v1393: f64 = (v1392 / v1389);
        let v1394: f64 = (v99 * v597);
        let v1395: f64 = (-v1394);
        let v1396: f64 = (v1395 / v1389);
        let v1397: f64 = (v99 * v598);
        let v1398: f64 = (-v1397);
        let v1399: f64 = (v1398 / v1389);
        let v1400: f64 = (if self.scalar_v228 { v1390 } else { v11 });
        let v1401: f64 = (if self.scalar_v228 { v1393 } else { v11 });
        let v1402: f64 = (if self.scalar_v228 { v1396 } else { v11 });
        let v1403: f64 = (if self.scalar_v228 { v1399 } else { v11 });
        let v1404: f64 = (v99 * v1185);
        let v1405: f64 = (-v1404);
        let v1406: f64 = (v348 * v348);
        let v1407: f64 = (v1405 / v1406);
        let v1408: f64 = (v99 * v1186);
        let v1409: f64 = (-v1408);
        let v1410: f64 = (v1409 / v1406);
        let v1411: f64 = (v99 * v1187);
        let v1412: f64 = (-v1411);
        let v1413: f64 = (v1412 / v1406);
        let v1414: f64 = (v99 * v1188);
        let v1415: f64 = (-v1414);
        let v1416: f64 = (v1415 / v1406);
        let v1417: f64 = (if self.scalar_v229 { v1407 } else { v1400 });
        let v1418: f64 = (if self.scalar_v229 { v1410 } else { v1401 });
        let v1419: f64 = (if self.scalar_v229 { v1413 } else { v1402 });
        let v1420: f64 = (if self.scalar_v229 { v1416 } else { v1403 });
        let v1421: f64 = (if self.scalar_v353 { v11 } else { v1045 });
        let v1422: f64 = (if self.scalar_v353 { v11 } else { v1046 });
        let v1423: f64 = (if self.scalar_v353 { v11 } else { v1047 });
        let v1427: f64 = (if self.scalar_v364 { v11 } else { v1421 });
        let v1428: f64 = (if self.scalar_v364 { v11 } else { v1422 });
        let v1429: f64 = (if self.scalar_v364 { v11 } else { v1423 });
        let v1431: f64 = (v371 * v371);
        let v1432: f64 = (v42 - v1431);
        let v1433: f64 = (-v1432);
        let v1434: f64 = (if self.scalar_v370 { v1433 } else { self.scalar_v1425 });
        let v1435: f64 = (if self.scalar_v370 { v1432 } else { self.scalar_v1426 });
        let v1436: f64 = (v373 * v373);
        let v1437: f64 = (v42 - v1436);
        let v1438: f64 = (-v1437);
        let v1439: f64 = (if self.scalar_v370 { v1438 } else { self.scalar_v1425 });
        let v1440: f64 = (if self.scalar_v370 { v1437 } else { self.scalar_v1426 });
        let v1441: f64 = (if self.scalar_v376 { v354 } else { v1434 });
        let v1442: f64 = (if self.scalar_v376 { v42 } else { v1435 });
        let v1443: f64 = (if self.scalar_v376 { v354 } else { v1439 });
        let v1444: f64 = (if self.scalar_v376 { v42 } else { v1440 });
        let v1445: f64 = (v115 * v1441);
        let v1446: f64 = (v115 * v1442);
        let v1447: f64 = { let limexp_arg = v380; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1448: f64 = (v1445 * v1447);
        let v1449: f64 = (v1446 * v1447);
        let v1450: f64 = (-v1427);
        let v1451: f64 = (-v1428);
        let v1452: f64 = (v1448 - v1429);
        let v1453: f64 = (v1449 - self.scalar_v1430);
        let v1454: f64 = (self.scalar_v379 * v1450);
        let v1455: f64 = (self.scalar_v379 * v1451);
        let v1456: f64 = (self.scalar_v379 * v1452);
        let v1457: f64 = (self.scalar_v379 * v1453);
        let v1458: f64 = (v115 * v1443);
        let v1459: f64 = (v115 * v1444);
        let v1460: f64 = { let limexp_arg = v384; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1461: f64 = (v1458 * v1460);
        let v1462: f64 = (v1459 * v1460);
        let v1463: f64 = (v1461 - v1427);
        let v1464: f64 = (-v1429);
        let v1466: f64 = (self.scalar_v379 * v1463);
        let v1467: f64 = (self.scalar_v379 * v1464);
        let v1468: f64 = (self.scalar_v379 * v1462);
        let v1470: f64 = (-v1383);
        let v1471: f64 = (-v1384);
        let v1472: f64 = (-v1385);
        let v1473: f64 = (-v1386);
        let v1476: f64 = (-v100);
        let v1477: f64 = (v447 * v1417);
        let v1478: f64 = (-v1477);
        let v1479: f64 = (v351 * v351);
        let v1480: f64 = (v1478 / v1479);
        let v1481: f64 = (v447 * v1418);
        let v1482: f64 = (-v1481);
        let v1483: f64 = (v1482 / v1479);
        let v1484: f64 = (-v351);
        let v1485: f64 = (v447 * v1419);
        let v1486: f64 = (v1484 - v1485);
        let v1487: f64 = (v1486 / v1479);
        let v1488: f64 = (v447 * v1420);
        let v1489: f64 = (-v1488);
        let v1490: f64 = (v1489 / v1479);
        let v1491: f64 = (v42 / v351);
        let v1492: f64 = (if self.scalar_v388 { v1480 } else { v11 });
        let v1493: f64 = (if self.scalar_v388 { v1483 } else { v11 });
        let v1494: f64 = (if self.scalar_v388 { v1487 } else { v11 });
        let v1495: f64 = (if self.scalar_v388 { v1490 } else { v11 });
        let v1496: f64 = (if self.scalar_v388 { v1491 } else { v11 });
        let v1511: f64 = (if self.scalar_v411 { v430 } else { v11 });
        let v1512: f64 = (if self.scalar_v411 { v426 } else { v11 });

        let d434_dn3: f64 = v1470;
        let d434_dn4: f64 = v1471;
        let d434_dn5: f64 = v1472;
        let d434_dn8: f64 = v1473;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(12),
            None,
            multiplicity * (v434),
            [3, 4, 5, 8],
            [d434_dn3, d434_dn4, d434_dn5, d434_dn8],
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
        let d383_dn3: f64 = v1454;
        let d383_dn4: f64 = v1455;
        let d383_dn5: f64 = v1456;
        let d383_dn8: f64 = v1457;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(5),
            multiplicity * (v383),
            [3, 4, 5, 8],
            [d383_dn3, d383_dn4, d383_dn5, d383_dn8],
            [],
            [],
            multiplicity,
        );
        let d387_dn3: f64 = v1466;
        let d387_dn4: f64 = v1455;
        let d387_dn5: f64 = v1467;
        let d387_dn7: f64 = v1468;
        let d387_dn8: f64 = self.scalar_v1469;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(3),
            multiplicity * (v387),
            [3, 4, 5, 7, 8],
            [d387_dn3, d387_dn4, d387_dn5, d387_dn7, d387_dn8],
            [],
            [],
            multiplicity,
        );
        let d449_dn3: f64 = v1492;
        let d449_dn4: f64 = v1493;
        let d449_dn5: f64 = v1494;
        let d449_dn8: f64 = v1495;
        let d449_dn10: f64 = v1496;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            Some(5),
            multiplicity * (v449),
            [3, 4, 5, 8, 10],
            [d449_dn3, d449_dn4, d449_dn5, d449_dn8, d449_dn10],
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
            self.scalar_v451,
        );
        let d458_dn5: f64 = self.scalar_v1500;
        let d458_dn9: f64 = self.scalar_v1501;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(5),
            multiplicity * (v458),
            5,
            multiplicity * (d458_dn5),
            9,
            multiplicity * (d458_dn9),
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(5),
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            self.scalar_v460,
        );
        let d463_dn4: f64 = self.scalar_v1504;
        let d463_dn7: f64 = self.scalar_v1505;
        stamper.stamp_current_node2_local(
            Some(4),
            Some(7),
            multiplicity * (v463),
            4,
            multiplicity * (d463_dn4),
            7,
            multiplicity * (d463_dn7),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(7),
            multiplicity * (self.scalar_v465),
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(7),
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            self.scalar_v467,
        );
        let d470_dn4: f64 = self.scalar_v1508;
        let d470_dn8: f64 = self.scalar_v1509;
        stamper.stamp_current_node2_local(
            Some(4),
            Some(8),
            multiplicity * (v470),
            4,
            multiplicity * (d470_dn4),
            8,
            multiplicity * (d470_dn8),
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            self.scalar_v472,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            7,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            7,
            self.scalar_v474,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            9,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            9,
            self.scalar_v478,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            11,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            11,
            self.scalar_v480,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            12,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            12,
            self.scalar_v482,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            16,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            16,
            self.scalar_v484,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            18,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            18,
            self.scalar_v488,
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (self.scalar_v490),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (self.scalar_v490),
        );
        stamper.stamp_current_const_local(
            Some(14),
            None,
            multiplicity * (self.scalar_v491),
        );
        let d493_dn14: f64 = self.scalar_v1510;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (v493),
            14,
            multiplicity * (d493_dn14),
        );
        stamper.stamp_current_const_local(
            Some(15),
            None,
            multiplicity * (self.scalar_v491),
        );
        let d495_dn15: f64 = self.scalar_v1510;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (v495),
            15,
            multiplicity * (d495_dn15),
        );
        let d493_dn14: f64 = self.scalar_v1510;
        stamper.stamp_current_node1_local(
            Some(4),
            Some(5),
            multiplicity * (v493),
            14,
            multiplicity * (d493_dn14),
        );
        let d499_dn14: f64 = v1511;
        let d499_dn15: f64 = v1512;
        stamper.stamp_current_node2_local(
            Some(4),
            Some(3),
            multiplicity * (v499),
            14,
            multiplicity * (d499_dn14),
            15,
            multiplicity * (d499_dn15),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (self.scalar_v491),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(5),
            multiplicity * (self.scalar_v491),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (self.scalar_v491),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (self.scalar_v501),
        );
        let d492_dn14: f64 = v42;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (v492),
            14,
            multiplicity * (d492_dn14),
        );
        let d494_dn15: f64 = v42;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (v494),
            15,
            multiplicity * (d494_dn15),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(5),
            multiplicity * (self.scalar_v502),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(3),
            multiplicity * (self.scalar_v502),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(5),
            multiplicity * (self.scalar_v504),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(3),
            multiplicity * (self.scalar_v504),
        );
        stamper.stamp_current_const_local(
            Some(11),
            None,
            multiplicity * (v511),
        );
        let d513_dn11: f64 = self.scalar_v1514;
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (v513),
            11,
            multiplicity * (d513_dn11),
        );
        let d517_dn11: f64 = self.scalar_v1515;
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (v517),
            11,
            multiplicity * (d517_dn11),
        );
        let d437_dn12: f64 = self.scalar_v435;
        let v437_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, v437);
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (v437_ddt),
            12,
            multiplicity * (((d437_dn12) * ddt_scale)),
        );
        let d441_dn1: f64 = self.scalar_v438;
        let d441_dn3: f64 = self.scalar_v1474;
        let v441_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, v441);
        stamper.stamp_current_node2_local(
            Some(4),
            Some(3),
            multiplicity * (v441_ddt),
            1,
            multiplicity * (((d441_dn1) * ddt_scale)),
            3,
            multiplicity * (((d441_dn3) * ddt_scale)),
        );
        let d443_dn3: f64 = self.scalar_v442;
        let d443_dn5: f64 = self.scalar_v1475;
        let v443_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, v443);
        stamper.stamp_current_node2_local(
            Some(3),
            Some(5),
            multiplicity * (v443_ddt),
            3,
            multiplicity * (((d443_dn3) * ddt_scale)),
            5,
            multiplicity * (((d443_dn5) * ddt_scale)),
        );
        let d446_dn3: f64 = v100;
        let d446_dn10: f64 = v1476;
        let v446_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, v446);
        stamper.stamp_current_node2_local(
            Some(3),
            Some(10),
            multiplicity * (v446_ddt),
            3,
            multiplicity * (((d446_dn3) * ddt_scale)),
            10,
            multiplicity * (((d446_dn10) * ddt_scale)),
        );
        let d455_dn8: f64 = self.scalar_v1497;
        let d455_dn9: f64 = self.scalar_v452;
        let v455_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, v455);
        stamper.stamp_current_node2_local(
            Some(9),
            Some(8),
            multiplicity * (v455_ddt),
            8,
            multiplicity * (((d455_dn8) * ddt_scale)),
            9,
            multiplicity * (((d455_dn9) * ddt_scale)),
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
        let v75: f64 = (v37 * self.scalar_v74);
        let v76: f64 = (v42 + v75);
        let v77: f64 = (self.scalar_v73 * v76);
        let v78: f64 = (if v41 { v77 } else { v11 });
        let v94: bool = (!v41);
        let v100: f64 = (if v94 { self.scalar_v73 } else { v78 });
        let v436: f64 = nv12;
        let v437: f64 = (self.scalar_v435 * v436);
        let v439: f64 = nv1;
        let v440: f64 = (v439 - v4);
        let v441: f64 = (self.scalar_v438 * v440);
        let v443: f64 = (v7 * self.scalar_v442);
        let v444: f64 = nv10;
        let v445: f64 = (v4 - v444);
        let v446: f64 = (v100 * v445);
        let v453: f64 = nv9;
        let v454: f64 = (v453 - v0);
        let v455: f64 = (self.scalar_v452 * v454);
        let v1476: f64 = (-v100);

        let d437_dn12: f64 = self.scalar_v435;
        stamper.stamp_current_reactive_node1(
            Some(nodes[12]),
            None,
            nodes[12],
            multiplicity * (d437_dn12),
        );
        let d441_dn1: f64 = self.scalar_v438;
        let d441_dn3: f64 = self.scalar_v1474;
        stamper.stamp_current_reactive_node2(
            Some(nodes[4]),
            Some(nodes[3]),
            nodes[1],
            multiplicity * (d441_dn1),
            nodes[3],
            multiplicity * (d441_dn3),
        );
        let d443_dn3: f64 = self.scalar_v442;
        let d443_dn5: f64 = self.scalar_v1475;
        stamper.stamp_current_reactive_node2(
            Some(nodes[3]),
            Some(nodes[5]),
            nodes[3],
            multiplicity * (d443_dn3),
            nodes[5],
            multiplicity * (d443_dn5),
        );
        let d446_dn3: f64 = v100;
        let d446_dn10: f64 = v1476;
        stamper.stamp_current_reactive_node2(
            Some(nodes[3]),
            Some(nodes[10]),
            nodes[3],
            multiplicity * (d446_dn3),
            nodes[10],
            multiplicity * (d446_dn10),
        );
        let d455_dn8: f64 = self.scalar_v1497;
        let d455_dn9: f64 = self.scalar_v452;
        stamper.stamp_current_reactive_node2(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes[8],
            multiplicity * (d455_dn8),
            nodes[9],
            multiplicity * (d455_dn9),
        );
        let mut locals = StampLocals::default();

        Self::stamp_reactive_block_0(ctx, p, nodes, param_given, &mut locals);
        Self::stamp_reactive_block_1(p, &mut locals);

        Self::stamp_reactive_equations_block_0(ctx, stamper, p, nodes, branches, multiplicity, &mut locals);
    }
}

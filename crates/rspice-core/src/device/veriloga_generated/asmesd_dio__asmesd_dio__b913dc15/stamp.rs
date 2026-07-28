#![allow(dead_code, non_snake_case, unused_imports, unused_mut, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

#[inline(always)]
fn rspice_limexp(x: f64) -> f64 {
    if x < 80.0 { x.exp() } else { (80.0f64).exp() * (x - 80.0 + 1.0) }
}

#[inline(always)]
fn rspice_limited_exp(x: f64) -> f64 {
    if x > 80.0 {
        5.54062238439351e34 * (x - 80.0 + 1.0)
    } else if x < -80.0 {
        1.804851387e-35
    } else {
        x.exp()
    }
}

#[inline(always)]
fn rspice_limited_exp_derivative(x: f64) -> f64 {
    if x > 80.0 {
        5.54062238439351e34
    } else if x < -80.0 {
        0.0
    } else {
        x.exp()
    }
}

/// A packed derivative: one partial per unknown the value can reach.
///
/// A newtype rather than a bare `[f64; N]` so the elementwise rules emit as
/// `a + b` and `a * s` instead of named calls. That is not cosmetic — these
/// operations are most of a large model's generated source, and an operator is
/// a dozen characters shorter than a call at every one of them.
#[derive(Clone, Copy)]
struct Lanes<const N: usize>([f64; N]);

impl<const N: usize> core::ops::Add for Lanes<N> {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        let mut out = self.0;
        let mut i = 0;
        while i < N {
            out[i] = self.0[i] + rhs.0[i];
            i += 1;
        }
        Self(out)
    }
}

impl<const N: usize> core::ops::Sub for Lanes<N> {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        let mut out = self.0;
        let mut i = 0;
        while i < N {
            out[i] = self.0[i] - rhs.0[i];
            i += 1;
        }
        Self(out)
    }
}

impl<const N: usize> core::ops::Mul<f64> for Lanes<N> {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f64) -> Self {
        let mut out = self.0;
        let mut i = 0;
        while i < N {
            out[i] = self.0[i] * rhs;
            i += 1;
        }
        Self(out)
    }
}

impl<const N: usize> core::ops::Div<f64> for Lanes<N> {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f64) -> Self {
        let mut out = self.0;
        let mut i = 0;
        while i < N {
            out[i] = self.0[i] / rhs;
            i += 1;
        }
        Self(out)
    }
}

impl<const N: usize> core::ops::Index<usize> for Lanes<N> {
    type Output = f64;
    #[inline(always)]
    fn index(&self, index: usize) -> &f64 {
        &self.0[index]
    }
}

#[inline]
fn rspice_eval_ddt<const STATE_COUNT: usize>(
    current: &mut [f64; STATE_COUNT],
    previous: &mut [f64; STATE_COUNT],
    older: &mut [f64; STATE_COUNT],
    initialized: &mut [bool; STATE_COUNT],
    derivative_current: &mut [f64; STATE_COUNT],
    derivative_previous: &mut [f64; STATE_COUNT],
    active: bool,
    scale: f64,
    previous_value_scale: f64,
    older_value_scale: f64,
    previous_derivative_scale: f64,
    slot: usize,
    value: f64,
) -> f64 {
    debug_assert!(slot < STATE_COUNT, "generated ddt state slot out of range");
    let previous_value = if initialized[slot] { previous[slot] } else { value };
    let older_value = if initialized[slot] { older[slot] } else { value };
    current[slot] = value;
    if active {
        let result = value * scale
            - previous_value * previous_value_scale
            - older_value * older_value_scale
            - derivative_previous[slot] * previous_derivative_scale;
        derivative_current[slot] = result;
        result
    } else {
        previous[slot] = value;
        older[slot] = value;
        derivative_current[slot] = 0.0;
        derivative_previous[slot] = 0.0;
        initialized[slot] = true;
        0.0
    }
}

impl Instance {
    fn canonical_instance_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        if self.canonical_instance_valid {
            return;
        }
        let produced: [f64; 31] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let v0 = parameters[43];
                let v1 = parameters[42];
                let v3 = parameters[25];
                let v4 = 2.7315e2f64;
                let v6 = 3.0015e2f64;
                let v9 = 4e-4f64;
                let v11 = parameters[49];
                let v12 = 1e0f64;
                let v14 = parameters[51];
                let v16 = 1e0f64;
                let v21 = parameters[31];
                let v23 = parameters[39];
                let v27 = parameters[32];
                let v29 = parameters[44];
                let v31 = 0e0f64;
                let v32 = 0e0f64;
                let v34 = parameters[30];
                let v36 = parameters[33];
                let v39 = 0e0f64;
                let v40 = 2e0f64;
                let v43 = parameters[35];
                let v50 = parameters[13];
                let v52 = parameters[12];
                let v55 = parameters[15];
                let v57 = parameters[14];
                let v61 = parameters[46];
                let v64 = -1e0f64;
                let v69 = 0e0f64;
                let v70 = 0e0f64;
                let v71 = 0e0f64;
                let v75 = 0e0f64;
                let v81 = 0e0f64;
                let v82 = 0e0f64;
                let v85 = parameters[28];
                let v87 = parameters[27];
                let v91 = 0e0f64;
                let mut out30: f64 = 0.0;
                let mut out45: f64 = 0.0;
                let mut out65: f64 = 0.0;
                let v2 = v0 * v1;
                let v5 = v3 + v4;
                let v7 = v5 / v6;
                let v10 = v9 * (v5 - v6);
                let v13 = v11 - v12;
                let v15 = v14 - v12;
                let v17 = v16 / v11;
                let v18 = v17 - v12;
                let v19 = v16 / v14;
                let v20 = v19 - v12;
                let v22 = if v21 == v16 { 1.0 } else { 0.0 };
                let v24 = v23 - v12;
                let v25 = v16 / v23;
                let v26 = v25 - v12;
                let v28 = if v27 == v16 { 1.0 } else { 0.0 };
                let v33: f64;
                if v28 != 0.0 {
                    let v30 = v29 - v12;
                    out30 = v30;
                    v33 = v31;
                } else {
                    v33 = v32;
                }
                let v37 = if v36 > v31 { 1.0 } else { 0.0 };
                let v38 = if (if v34 == v16 { 1.0 } else { 0.0 }) != 0.0 && v37 != 0.0 { 1.0 } else { 0.0 };
                let v46: f64;
                let v47: f64;
                let v48: f64;
                let v49: f64;
                if v38 != 0.0 {
                    v46 = v39;
                    v47 = v31;
                    v48 = v31;
                    v49 = v31;
                } else {
                    let v45 = if (if (if v34 == v40 { 1.0 } else { 0.0 }) != 0.0 && v37 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v43 > v31 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    out45 = v45;
                    let v66: f64;
                    let v67: f64;
                    let v68: f64;
                    if v45 != 0.0 {
                        v66 = v31;
                        v67 = v31;
                        v68 = v31;
                    } else {
                        let v65 = if v34 == v64 { 1.0 } else { 0.0 };
                        out65 = v65;
                        let v72: f64;
                        let v73: f64;
                        let v74: f64;
                        if v65 != 0.0 {
                            v72 = v69;
                            v73 = v31;
                            v74 = v31;
                        } else {
                            v72 = v31;
                            v73 = v70;
                            v74 = v71;
                        }
                        v66 = v72;
                        v67 = v73;
                        v68 = v74;
                    }
                    v46 = v31;
                    v47 = v66;
                    v48 = v67;
                    v49 = v68;
                }
                let v54 = (v52 + (v21 * v50)) / v2;
                let v59 = (v57 + (v21 * v55)) / v2;
                let v63 = if (if v54 > v31 { 1.0 } else { 0.0 }) != 0.0 && (if v54 >= v61 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v76: f64;
                let v77: f64;
                if v63 != 0.0 {
                    v76 = v81;
                    v77 = v31;
                } else {
                    v76 = v31;
                    v77 = v75;
                }
                let v80 = if (if v59 > v31 { 1.0 } else { 0.0 }) != 0.0 && (if v59 >= v61 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v83: f64;
                let v84: f64;
                if v80 != 0.0 {
                    v83 = v91;
                    v84 = v31;
                } else {
                    v83 = v31;
                    v84 = v82;
                }
                let v90 = if (if (if v85 > v31 { 1.0 } else { 0.0 }) != 0.0 && (if v87 > v31 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) > v31 { 1.0 } else { 0.0 };
            [v2, v5, v7, v10, v17, v19, v22, v25, v28, v38, out45, out65, v63, v80, v90, v33, v46, v47, v48, v49, v76, v77, v83, v84, v13, v15, v18, v20, v24, v26, out30]
        };
        self.canonical_staged[6] = produced[0];
        self.canonical_staged[0] = produced[1];
        self.canonical_staged[1] = produced[2];
        self.canonical_staged[2] = produced[3];
        self.canonical_staged[3] = produced[4];
        self.canonical_staged[4] = produced[5];
        self.canonical_staged[14] = produced[6];
        self.canonical_staged[5] = produced[7];
        self.canonical_staged[15] = produced[8];
        self.canonical_staged[16] = produced[9];
        self.canonical_staged[17] = produced[10];
        self.canonical_staged[19] = produced[11];
        self.canonical_staged[18] = produced[12];
        self.canonical_staged[20] = produced[13];
        self.canonical_staged[21] = produced[14];
        self.canonical_staged[22] = produced[15];
        self.canonical_staged[23] = produced[16];
        self.canonical_staged[24] = produced[17];
        self.canonical_staged[25] = produced[18];
        self.canonical_staged[26] = produced[19];
        self.canonical_staged[27] = produced[20];
        self.canonical_staged[28] = produced[21];
        self.canonical_staged[29] = produced[22];
        self.canonical_staged[30] = produced[23];
        self.canonical_staged[7] = produced[24];
        self.canonical_staged[8] = produced[25];
        self.canonical_staged[9] = produced[26];
        self.canonical_staged[10] = produced[27];
        self.canonical_staged[11] = produced[28];
        self.canonical_staged[12] = produced[29];
        self.canonical_staged[13] = produced[30];
        self.canonical_instance_valid = true;
    }

    fn canonical_temperature_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        let temperature = ctx.temperature();
        let thermal_voltage = ctx.thermal_voltage();
        if self.canonical_temperature_valid
            && self.canonical_temperature == temperature
            && self.canonical_thermal_voltage == thermal_voltage
        {
            return;
        }
        let produced: [f64; 1] = {
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
            [0.0]
        };
        self.canonical_temperature = temperature;
        self.canonical_thermal_voltage = thermal_voltage;
        self.canonical_temperature_valid = true;
    }

    fn canonical_timestep_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        let produced: [f64; 1] = {
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
            [0.0]
        };
    }

    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        self.canonical_instance_stage(ctx);
        self.canonical_temperature_stage(ctx);
        self.canonical_timestep_stage(ctx);
        let parameters = &self.params.values;
        let multiplicity = self.multiplicity;
        let temperature = ctx.temperature();
        let staged = &*self.canonical_staged;
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 1504 => 0usize, 1615 => 1usize, 1641 => 2usize, 1648 => 3usize, 1767 => 4usize, 1773 => 5usize, _ => usize::MAX };
            rspice_eval_ddt(
                &mut ddt_state.ddt_current,
                &mut ddt_state.ddt_previous,
                &mut ddt_state.ddt_older,
                &mut ddt_state.ddt_initialized,
                &mut ddt_state.ddt_derivative_current,
                &mut ddt_state.ddt_derivative_previous,
                ddt_active,
                ddt_coefficients.derivative_scale,
                ddt_coefficients.previous_value_scale,
                ddt_coefficients.older_value_scale,
                ddt_coefficients.previous_derivative_scale,
                slot,
                value,
            )
        };
            let v0 = temperature;
            let v1 = node_potentials[2];
            let v3 = parameters[45];
            let v5 = 1.7314999999999998e2f64;
            let v7 = 1.7314999999999998e2f64;
            let v9 = 1.3e3f64;
            let v11 = 1.3e3f64;
            let v12 = Lanes([0e0f64; 1]);
            let v13 = 1.7314999999999998e2f64;
            let v17 = Lanes([1e0f64; 1]);
            let v18 = 1.7314999999999998e2f64;
            let v21 = parameters[26];
            let v23 = 8.6170869e-5f64;
            let v26 = staged[0];
            let v30 = 1e0f64;
            let v33 = parameters[22];
            let v36 = 1e0f64;
            let v38 = parameters[21];
            let v47 = parameters[23];
            let v52 = parameters[0];
            let v57 = parameters[2];
            let v60 = parameters[7];
            let v64 = parameters[47];
            let v67 = parameters[6];
            let v71 = parameters[5];
            let v74 = parameters[10];
            let v78 = parameters[9];
            let v81 = 3.0015e2f64;
            let v84 = 7.02e-4f64;
            let v91 = 1.108e3f64;
            let v97 = 1.16e0f64;
            let v99 = -1e0f64;
            let v105 = 1.3806226e-23f64;
            let v112 = 1.3454442398941469e20f64;
            let v121 = 1.5e0f64;
            let v124 = 1.6021918e-19f64;
            let v133 = parameters[17];
            let v136 = staged[1];
            let v145 = staged[2];
            let v148 = parameters[18];
            let v152 = parameters[16];
            let v170 = 4e-4f64;
            let v182 = node_potentials[3];
            let v183 = node_potentials[4];
            let v185 = Lanes([1e0f64; 1]);
            let v187 = Lanes([1e0f64; 1]);
            let v190 = parameters[29];
            let v193 = node_potentials[0];
            let v195 = Lanes([1e0f64; 1]);
            let v201 = node_potentials[1];
            let v203 = Lanes([1e0f64; 1]);
            let v209 = 0e0f64;
            let v211 = parameters[1];
            let v226 = parameters[11];
            let v240 = 8e1f64;
            let v242 = Lanes([0e0f64; 3]);
            let v258 = 3.7e1f64;
            let v260 = -3.7e1f64;
            let v275 = -3.7e1f64;
            let v294 = 0e0f64;
            let v296 = 2e0f64;
            let v311 = parameters[8];
            let v331 = parameters[4];
            let v334 = 1e-3f64;
            let v338 = -1e0f64;
            let v343 = parameters[3];
            let v362 = parameters[48];
            let v370 = parameters[49];
            let v372 = staged[7];
            let v377 = parameters[50];
            let v385 = parameters[51];
            let v387 = staged[8];
            let v392 = parameters[37];
            let v397 = parameters[12];
            let v400 = staged[3];
            let v402 = staged[9];
            let v412 = parameters[38];
            let v417 = parameters[14];
            let v420 = staged[4];
            let v422 = staged[10];
            let v432 = staged[14];
            let v451 = parameters[13];
            let v453 = parameters[15];
            let v463 = parameters[40];
            let v471 = parameters[39];
            let v473 = staged[11];
            let v478 = staged[5];
            let v480 = staged[12];
            let v485 = parameters[41];
            let v489 = parameters[19];
            let v498 = staged[15];
            let v507 = node_potentials[6];
            let v509 = Lanes([1e0f64; 1]);
            let v510 = ddt_scale();
            let v523 = parameters[20];
            let v526 = parameters[44];
            let v528 = staged[13];
            let v540 = Lanes([0e0f64; 5]);
            let v541 = Lanes([0e0f64; 1]);
            let v542 = Lanes([0e0f64; 3]);
            let v553 = parameters[24];
            let v561 = -1e0f64;
            let v575 = 5e-1f64;
            let v627 = staged[16];
            let v639 = -1e0f64;
            let v642 = parameters[33];
            let v645 = parameters[34];
            let v650 = Lanes([0e0f64; 2]);
            let v651 = Lanes([0e0f64; 1]);
            let v652 = staged[17];
            let v680 = staged[18];
            let v692 = -1e0f64;
            let v695 = node_potentials[5];
            let v698 = Lanes([1e0f64; 1]);
            let v707 = parameters[35];
            let v710 = parameters[36];
            let v715 = staged[19];
            let v743 = -1e0f64;
            let v748 = staged[6];
            let v751 = parameters[46];
            let v753 = Lanes([0e0f64; 4]);
            let v756 = staged[20];
            let v768 = Lanes([0e0f64; 3]);
            let v858 = 0e0f64;
            let v859 = 0e0f64;
            let v4 = (v0 + v1) + v3;
            let v6 = if v4 > v5 { 1.0 } else { 0.0 };
            let v8: f64;
            if v6 != 0.0 {
                v8 = v4;
            } else {
                v8 = v7;
            }
            let v10 = if v9 < v8 { 1.0 } else { 0.0 };
            let v15: f64;
            let v16: Lanes<1>;
            if v10 != 0.0 {
                v15 = v11;
                v16 = v12;
            } else {
                let v14 = if v4 > v13 { 1.0 } else { 0.0 };
                let v19: f64;
                let v20: Lanes<1>;
                if v14 != 0.0 {
                    v19 = v4;
                    v20 = v17;
                } else {
                    v19 = v18;
                    v20 = v12;
                }
                v15 = v19;
                v16 = v20;
            }
            let v22 = if v15 > v21 { 1.0 } else { 0.0 };
            let v24 = v23 * v15;
            let v25 = v16 * v23;
            let v27 = v15 / v26;
            let v28 = v16 / v26;
            let v29 = v27.ln();
            let v32 = v28 * (v30 / v27);
            let v37 = v27 - v36;
            let v41 = (v38 * v37) / v24;
            let v50 = ((v33 * v29) + v41).exp();
            let v53 = v52 * v50;
            let v54 = (((v32 * v33) + (((v28 * v38) - (v25 * v41)) / v24)) * v50) * v52;
            let v55 = (v47 * v29).exp();
            let v58 = v57 * v55;
            let v59 = ((v32 * v47) * v55) * v57;
            let v65 = v64 * (v36 + (v60 * v37));
            let v66 = (v28 * v60) * v64;
            let v72 = v71 * (v36 + (v67 * v37));
            let v73 = (v28 * v67) * v71;
            let v79 = v78 * (v36 + (v74 * v37));
            let v80 = (v28 * v74) * v78;
            let v82 = v15 / v81;
            let v83 = v16 / v81;
            let v85 = v84 * v15;
            let v92 = v91 + v15;
            let v93 = (v85 * v15) / v92;
            let v106 = v105 * (v15 + v15);
            let v108 = (-(v97 - v93)) / v106;
            let v116 = -(v24 + v24);
            let v127 = (v121 * (v82.ln())) + (v124 * (v108 + v112));
            let v129 = v116 * v127;
            let v132 = (((v25 + v25) * v99) * v127) + ((((v83 * (v30 / v82)) * v121) + ((((((((((v16 * v84) * v15) + (v16 * v85)) - (v16 * v93)) / v92) * v99) * v99) - (((v16 + v16) * v105) * v108)) / v106) * v124)) * v116);
            let v137 = (v133 - v129) / v136;
            let v138 = (v132 * v99) / v136;
            let v141 = (v133 - v137) / v137;
            let v151 = v36 + (v148 * (v145 - v141));
            let v153 = v152 / v151;
            let v161 = (v82 * v137) + v129;
            let v162 = ((v83 * v137) + (v138 * v82)) + v132;
            let v165 = (v161 - v137) / v137;
            let v177 = v36 + (v148 * ((v170 * (v15 - v81)) - v165));
            let v178 = v153 * v177;
            let v181 = (((((((((v138 * v99) - (v138 * v141)) / v137) * v99) * v148) * v153) * v99) / v151) * v177) + ((((v16 * v170) - (((v162 - v138) - (v138 * v165)) / v137)) * v148) * v153);
            let v184 = v182 - v183;
            let v189 = (Lanes([v185[0], 0.0])) - (Lanes([0.0, v187[0]]));
            let v191 = v190 * v184;
            let v192 = v189 * v190;
            let v194 = v193 - v182;
            let v198 = (Lanes([v195[0], 0.0])) - (Lanes([0.0, v185[0]]));
            let v199 = v190 * v194;
            let v200 = v198 * v190;
            let v202 = v201 - v183;
            let v206 = (Lanes([v203[0], 0.0])) - (Lanes([0.0, v187[0]]));
            let v207 = v190 * v202;
            let v208 = v206 * v190;
            let v210 = if v53 > v209 { 1.0 } else { 0.0 };
            let v243: f64;
            let v244: Lanes<3>;
            if v210 != 0.0 {
                let v212 = v211 * v24;
                let v214 = v191 / v212;
                let v215 = (v25 * v211) * v214;
                let v219 = ((Lanes([0.0, v192[0], v192[1]])) - (Lanes([v215[0], 0.0, 0.0]))) / v212;
                let v221 = v192 * v99;
                let v227 = v226 * v24;
                let v228 = v25 * v226;
                let v229 = ((-v191) - v72) / v227;
                let v230 = v228 * v229;
                let v233 = (((Lanes([0.0, v221[0], v221[1]])) - (Lanes([v73[0], 0.0, 0.0]))) - (Lanes([v230[0], 0.0, 0.0]))) / v227;
                let v236 = (-v72) / v227;
                let v239 = ((v73 * v99) - (v228 * v236)) / v227;
                let v241 = if v214 > v240 { 1.0 } else { 0.0 };
                let v248: f64;
                let v249: f64;
                let v250: Lanes<3>;
                let v251: Lanes<3>;
                if v241 != 0.0 {
                    let v247 = v36 + (v214 - v240);
                    v248 = v247;
                    v249 = v240;
                    v250 = v219;
                    v251 = v242;
                } else {
                    v248 = v36;
                    v249 = v214;
                    v250 = v242;
                    v251 = v219;
                }
                let v252 = v249.exp();
                let v254 = v248 * v252;
                let v257 = (v250 * v252) + ((v251 * v252) * v248);
                let v259 = if v229 >= v258 { 1.0 } else { 0.0 };
                let v262: f64;
                let v263: Lanes<3>;
                if v259 != 0.0 {
                    v262 = v229;
                    v263 = v233;
                } else {
                    let v261 = if v229 <= v260 { 1.0 } else { 0.0 };
                    let v273: f64;
                    let v274: Lanes<3>;
                    if v261 != 0.0 {
                        let v265 = v229.exp();
                        let v266 = v233 * v265;
                        v273 = v265;
                        v274 = v266;
                    } else {
                        let v267 = v229.exp();
                        let v269 = v267 + v36;
                        let v270 = v269.ln();
                        let v272 = (v233 * v267) * (v30 / v269);
                        v273 = v270;
                        v274 = v272;
                    }
                    v262 = v273;
                    v263 = v274;
                }
                let v264 = if v236 >= v258 { 1.0 } else { 0.0 };
                let v277: f64;
                let v278: Lanes<1>;
                if v264 != 0.0 {
                    v277 = v236;
                    v278 = v239;
                } else {
                    let v276 = if v236 <= v275 { 1.0 } else { 0.0 };
                    let v329: f64;
                    let v330: Lanes<1>;
                    if v276 != 0.0 {
                        let v321 = v236.exp();
                        let v322 = v239 * v321;
                        v329 = v321;
                        v330 = v322;
                    } else {
                        let v323 = v236.exp();
                        let v325 = v323 + v36;
                        let v326 = v325.ln();
                        let v328 = (v239 * v323) * (v30 / v325);
                        v329 = v326;
                        v330 = v328;
                    }
                    v277 = v329;
                    v278 = v330;
                }
                let v279 = v262 - v277;
                let v282 = v254 - v36;
                let v284 = v54 * v282;
                let v289 = v66 * v279;
                let v293 = v191.abs();
                let v300 = v293.powf(v79);
                let v304 = (v192 * ((v296 * (if v191 >= v294 { 1.0 } else { 0.0 })) - v30)) * (v79 * (v293.powf((v79 - v30))));
                let v307 = v80 * (v300 * (v293.ln()));
                let v314 = v36 + (v311 * v300);
                let v315 = (v65 * v279) / v314;
                let v319 = (v53 * v282) - v315;
                let v320 = ((Lanes([v284[0], 0.0, 0.0])) + (v257 * v53)) - ((((Lanes([v289[0], 0.0, 0.0])) + ((v263 - (Lanes([v278[0], 0.0, 0.0]))) * v65)) - ((((Lanes([0.0, v304[0], v304[1]])) + (Lanes([v307[0], 0.0, 0.0]))) * v311) * v315)) / v314);
                v243 = v319;
                v244 = v320;
            } else {
                v243 = v209;
                v244 = v242;
            }
            let v245 = if v58 > v209 { 1.0 } else { 0.0 };
            let v358: f64;
            let v359: Lanes<3>;
            if v245 != 0.0 {
                let v332 = v331 - v191;
                let v335 = if v332 >= v334 { v332 } else { v334 };
                let v342 = (v192 * v338) * v331;
                let v344 = v343 * v24;
                let v346 = v344 * v335;
                let v347 = (v25 * v343) * v335;
                let v348 = ((v192 * v99) * (if v332 >= v334 { 1.0 } else { 0.0 })) * v344;
                let v352 = ((v338 * v191) * v331) / v346;
                let v356 = ((Lanes([0.0, v342[0], v342[1]])) - (((Lanes([v347[0], 0.0, 0.0])) + (Lanes([0.0, v348[0], v348[1]]))) * v352)) / v346;
                let v357 = if v352 > v240 { 1.0 } else { 0.0 };
                let v435: f64;
                let v436: f64;
                let v437: Lanes<3>;
                let v438: Lanes<3>;
                if v357 != 0.0 {
                    let v434 = v36 + (v352 - v240);
                    v435 = v434;
                    v436 = v240;
                    v437 = v356;
                    v438 = v242;
                } else {
                    v435 = v36;
                    v436 = v352;
                    v437 = v242;
                    v438 = v356;
                }
                let v439 = v436.exp();
                let v445 = (v435 * v439) - v36;
                let v446 = v58 * v445;
                let v447 = v59 * v445;
                let v450 = (Lanes([v447[0], 0.0, 0.0])) + (((v437 * v439) + ((v438 * v439) * v435)) * v58);
                v358 = v446;
                v359 = v450;
            } else {
                v358 = v209;
                v359 = v242;
            }
            let v360 = v243 - v358;
            let v361 = v244 - v359;
            let v363 = v199 / v362;
            let v365 = v363.abs();
            let v376 = v36 + (v365.powf(v370));
            let v378 = v207 / v377;
            let v380 = v378.abs();
            let v391 = v36 + (v380.powf(v385));
            let v395 = (v29 * v392).exp();
            let v398 = v397 * v395;
            let v401 = v376.powf(v400);
            let v406 = v398 * v401;
            let v407 = (((v32 * v392) * v395) * v397) * v401;
            let v408 = ((((v200 / v362) * ((v296 * (if v363 >= v294 { 1.0 } else { 0.0 })) - v30)) * (v370 * (v365.powf(v372)))) * (v400 * (v376.powf(v402)))) * v398;
            let v411 = (Lanes([0.0, v407[0], 0.0])) + (Lanes([v408[0], 0.0, v408[1]]));
            let v415 = (v29 * v412).exp();
            let v418 = v417 * v415;
            let v421 = v391.powf(v420);
            let v426 = v418 * v421;
            let v427 = (((v32 * v412) * v415) * v417) * v421;
            let v428 = ((((v208 / v377) * ((v296 * (if v378 >= v294 { 1.0 } else { 0.0 })) - v30)) * (v385 * (v380.powf(v387)))) * (v420 * (v391.powf(v422)))) * v418;
            let v431 = (Lanes([0.0, v427[0], 0.0])) + (Lanes([v428[0], 0.0, v428[1]]));
            let v455: f64;
            let v456: f64;
            let v457: Lanes<3>;
            let v458: Lanes<3>;
            if v432 != 0.0 {
                let v452 = v406 + v451;
                let v454 = v426 + v453;
                v455 = v452;
                v456 = v454;
                v457 = v411;
                v458 = v431;
            } else {
                v455 = v406;
                v456 = v426;
                v457 = v411;
                v458 = v431;
            }
            let v459 = v193 - v201;
            let v462 = (Lanes([v195[0], 0.0])) - (Lanes([0.0, v203[0]]));
            let v464 = v459 / v463;
            let v466 = v464.abs();
            let v477 = v36 + (v466.powf(v471));
            let v490 = v489 * (v36 + (v485 * ((v477.powf(v478)) - v36)));
            let v491 = (((((v462 / v463) * ((v296 * (if v464 >= v294 { 1.0 } else { 0.0 })) - v30)) * (v471 * (v466.powf(v473)))) * (v478 * (v477.powf(v480)))) * v485) * v489;
            let v492 = v490 * v243;
            let v493 = v491 * v243;
            let v494 = v244 * v490;
            let v497 = (Lanes([v493[0], v493[1], 0.0, 0.0, 0.0])) + (Lanes([0.0, 0.0, v494[0], v494[1], v494[2]]));
            let v543: f64;
            let v544: f64;
            let v545: f64;
            let v546: f64;
            let v547: Lanes<4>;
            let v548: Lanes<5>;
            let v549: Lanes<1>;
            let v550: Lanes<3>;
            if v498 != 0.0 {
                let v499 = -v243;
                let v501 = v499 * v490;
                let v502 = (v244 * v99) * v490;
                let v503 = v491 * v499;
                let v506 = (Lanes([0.0, 0.0, v502[0], v502[1], v502[2]])) + (Lanes([v503[0], v503[1], 0.0, 0.0, 0.0]));
                let v508 = ddt(1504, v507);
                let v512 = v490 * v508;
                let v513 = v491 * v508;
                let v514 = (v509 * v510) * v490;
                let v517 = (Lanes([v513[0], v513[1], 0.0])) + (Lanes([0.0, 0.0, v514[0]]));
                let v524 = (v507.abs()) / v523;
                let v532 = v36 + (v524.powf(v526));
                let v533 = v455 / v532;
                let v534 = (((v509 * ((v296 * (if v507 >= v294 { 1.0 } else { 0.0 })) - v30)) / v523) * (v526 * (v524.powf(v528)))) * v533;
                let v538 = ((Lanes([v457[0], v457[1], v457[2], 0.0])) - (Lanes([0.0, 0.0, 0.0, v534[0]]))) / v532;
                v543 = v533;
                v544 = v501;
                v545 = v507;
                v546 = v512;
                v547 = v538;
                v548 = v506;
                v549 = v509;
                v550 = v517;
            } else {
                let v539 = Lanes([v457[0], v457[1], v457[2], 0.0]);
                v543 = v455;
                v544 = v209;
                v545 = v209;
                v546 = v209;
                v547 = v539;
                v548 = v540;
                v549 = v541;
                v550 = v542;
            }
            let v555 = (v162 * v99) * v553;
            let v556 = v191 + ((-v161) * v553);
            let v557 = Lanes([0.0, v192[0], v192[1]]);
            let v559 = v557 + (Lanes([v555[0], 0.0, 0.0]));
            let v560 = if v556 > v209 { 1.0 } else { 0.0 };
            let v616: f64;
            let v617: f64;
            let v618: Lanes<3>;
            let v619: Lanes<3>;
            if v560 != 0.0 {
                let v563 = v36 - v553;
                let v566 = ((v561 - v148) * (v563.ln())).exp();
                let v569 = v36 - ((v566 * v563) * v563);
                let v572 = v36 - v148;
                let v573 = (v161 * v569) / v572;
                let v574 = (v162 * v569) / v572;
                let v576 = v575 * v148;
                let v579 = (v576 * v556) / v161;
                let v580 = v162 * v579;
                let v584 = v563 + v579;
                let v589 = (v556 * v584) * v566;
                let v590 = ((v559 * v584) + ((((v559 * v576) - (Lanes([v580[0], 0.0, 0.0]))) / v161) * v556)) * v566;
                let v591 = Lanes([v574[0], 0.0, 0.0]);
                v616 = v573;
                v617 = v589;
                v618 = v591;
                v619 = v590;
            } else {
                let v592 = v36 - v148;
                let v593 = v191 / v161;
                let v594 = v162 * v593;
                let v598 = v36 - v593;
                let v605 = (v592 * (v598.ln())).exp();
                let v607 = v36 - v605;
                let v610 = v162 * v607;
                let v614 = (v161 * v607) / v592;
                let v615 = ((Lanes([v610[0], 0.0, 0.0])) + ((((((((v557 - (Lanes([v594[0], 0.0, 0.0]))) / v161) * v99) * (v30 / v598)) * v592) * v605) * v99) * v161)) / v592;
                v616 = v614;
                v617 = v209;
                v618 = v615;
                v619 = v242;
            }
            let v620 = v616 + v617;
            let v622 = v178 * v620;
            let v623 = v181 * v620;
            let v626 = (Lanes([v623[0], 0.0, 0.0])) + ((v618 + v619) * v178);
            let v653: f64;
            let v654: f64;
            let v655: f64;
            let v656: f64;
            let v657: f64;
            let v658: f64;
            let v659: f64;
            let v660: f64;
            let v661: f64;
            let v662: f64;
            let v663: f64;
            let v664: f64;
            let v665: Lanes<5>;
            let v666: Lanes<1>;
            let v667: Lanes<1>;
            let v668: Lanes<5>;
            let v669: Lanes<2>;
            let v670: Lanes<1>;
            let v671: Lanes<1>;
            let v672: Lanes<1>;
            let v673: Lanes<5>;
            let v674: Lanes<1>;
            let v675: Lanes<1>;
            let v676: Lanes<1>;
            if v627 != 0.0 {
                let v628 = v360 * v459;
                let v629 = v361 * v459;
                let v630 = v462 * v360;
                let v640 = v639 * (v628.abs());
                let v641 = (((Lanes([0.0, 0.0, v629[0], v629[1], v629[2]])) + (Lanes([v630[0], v630[1], 0.0, 0.0, 0.0]))) * ((v296 * (if v628 >= v294 { 1.0 } else { 0.0 })) - v30)) * v639;
                let v643 = v1 / v642;
                let v644 = v17 / v642;
                let v646 = v1 * v645;
                let v647 = v17 * v645;
                let v648 = ddt(1615, v646);
                let v649 = v647 * v510;
                v653 = v640;
                v654 = v643;
                v655 = v648;
                v656 = v209;
                v657 = v209;
                v658 = v209;
                v659 = v209;
                v660 = v209;
                v661 = v209;
                v662 = v646;
                v663 = v209;
                v664 = v209;
                v665 = v641;
                v666 = v644;
                v667 = v649;
                v668 = v540;
                v669 = v650;
                v670 = v12;
                v671 = v651;
                v672 = v651;
                v673 = v540;
                v674 = v647;
                v675 = v12;
                v676 = v651;
            } else {
                let v716: f64;
                let v717: f64;
                let v718: f64;
                let v719: f64;
                let v720: f64;
                let v721: f64;
                let v722: f64;
                let v723: f64;
                let v724: Lanes<5>;
                let v725: Lanes<2>;
                let v726: Lanes<1>;
                let v727: Lanes<1>;
                let v728: Lanes<1>;
                let v729: Lanes<5>;
                let v730: Lanes<1>;
                let v731: Lanes<1>;
                if v652 != 0.0 {
                    let v681 = v360 * v459;
                    let v682 = v361 * v459;
                    let v683 = v462 * v360;
                    let v693 = v692 * (v681.abs());
                    let v694 = (((Lanes([0.0, 0.0, v682[0], v682[1], v682[2]])) + (Lanes([v683[0], v683[1], 0.0, 0.0, 0.0]))) * ((v296 * (if v681 >= v294 { 1.0 } else { 0.0 })) - v30)) * v692;
                    let v701 = (v1 - v695) / v642;
                    let v702 = ((Lanes([v17[0], 0.0])) - (Lanes([0.0, v698[0]]))) / v642;
                    let v703 = v645 * v1;
                    let v704 = v17 * v645;
                    let v705 = ddt(1641, v703);
                    let v706 = v704 * v510;
                    let v708 = v695 / v707;
                    let v709 = v698 / v707;
                    let v711 = v710 * v695;
                    let v712 = v698 * v710;
                    let v713 = ddt(1648, v711);
                    let v714 = v712 * v510;
                    v716 = v693;
                    v717 = v701;
                    v718 = v705;
                    v719 = v708;
                    v720 = v713;
                    v721 = v209;
                    v722 = v703;
                    v723 = v711;
                    v724 = v694;
                    v725 = v702;
                    v726 = v706;
                    v727 = v709;
                    v728 = v714;
                    v729 = v540;
                    v730 = v704;
                    v731 = v712;
                } else {
                    let v746: f64;
                    let v747: Lanes<5>;
                    if v715 != 0.0 {
                        let v732 = v360 * v459;
                        let v733 = v361 * v459;
                        let v734 = v462 * v360;
                        let v744 = v743 * (v732.abs());
                        let v745 = (((Lanes([0.0, 0.0, v733[0], v733[1], v733[2]])) + (Lanes([v734[0], v734[1], 0.0, 0.0, 0.0]))) * ((v296 * (if v732 >= v294 { 1.0 } else { 0.0 })) - v30)) * v743;
                        v746 = v744;
                        v747 = v745;
                    } else {
                        v746 = v209;
                        v747 = v540;
                    }
                    v716 = v209;
                    v717 = v209;
                    v718 = v209;
                    v719 = v209;
                    v720 = v209;
                    v721 = v746;
                    v722 = v209;
                    v723 = v209;
                    v724 = v540;
                    v725 = v650;
                    v726 = v12;
                    v727 = v651;
                    v728 = v651;
                    v729 = v747;
                    v730 = v12;
                    v731 = v651;
                }
                v653 = v209;
                v654 = v209;
                v655 = v209;
                v656 = v716;
                v657 = v717;
                v658 = v718;
                v659 = v719;
                v660 = v720;
                v661 = v721;
                v662 = v209;
                v663 = v722;
                v664 = v723;
                v665 = v540;
                v666 = v12;
                v667 = v12;
                v668 = v724;
                v669 = v725;
                v670 = v726;
                v671 = v727;
                v672 = v728;
                v673 = v729;
                v674 = v12;
                v675 = v730;
                v676 = v731;
            }
            let v677 = ctx.simparam_or("gmin", v209);
            let v678 = v677 * v184;
            let v679 = v189 * v677;
            let v754: f64;
            let v755: Lanes<4>;
            if v680 != 0.0 {
                let v749 = v543 / v748;
                let v750 = v547 / v748;
                let v752 = if v749 > v751 { 1.0 } else { 0.0 };
                let v757: f64;
                let v758: Lanes<4>;
                if v752 != 0.0 {
                    v757 = v749;
                    v758 = v750;
                } else {
                    v757 = v751;
                    v758 = v753;
                }
                let v759 = v194 / v757;
                let v763 = ((Lanes([v198[0], 0.0, v198[1], 0.0])) - (v758 * v759)) / v757;
                let v764 = if v749 >= v751 { 1.0 } else { 0.0 };
                v754 = v759;
                v755 = v763;
            } else {
                v754 = v209;
                v755 = v753;
            }
            let v769: f64;
            let v770: Lanes<3>;
            if v756 != 0.0 {
                let v765 = v456 / v748;
                let v766 = v458 / v748;
                let v767 = if v765 > v751 { 1.0 } else { 0.0 };
                let v787: f64;
                let v788: Lanes<3>;
                if v767 != 0.0 {
                    v787 = v765;
                    v788 = v766;
                } else {
                    v787 = v751;
                    v788 = v768;
                }
                let v789 = v202 / v787;
                let v793 = ((Lanes([v206[0], 0.0, v206[1]])) - (v788 * v789)) / v787;
                let v794 = if v765 >= v751 { 1.0 } else { 0.0 };
                v769 = v789;
                v770 = v793;
            } else {
                v769 = v209;
                v770 = v768;
            }
            let v771 = v190 * v360;
            let v773 = v771 * v748;
            let v774 = (v361 * v190) * v748;
            let v777 = (v190 * v622) * v748;
            let v778 = (v626 * v190) * v748;
            let v779 = ddt(1767, v777);
            let v780 = v778 * v510;
            let v783 = (v190 * v492) * v748;
            let v784 = (v497 * v190) * v748;
            let v785 = ddt(1773, v783);
            let v786 = v784 * v510;
            let v795 = if v771 >= v209 { 1.0 } else { 0.0 };
            let v796 = v548[0];
            let v797 = v548[1];
            let v798 = v548[2];
            let v799 = v548[3];
            let v800 = v548[4];
            let v801 = v549[0];
            let v802 = v550[0];
            let v803 = v550[1];
            let v804 = v550[2];
            let v805 = v665[0];
            let v806 = v665[1];
            let v807 = v665[2];
            let v808 = v665[3];
            let v809 = v665[4];
            let v810 = v666[0];
            let v811 = v667[0];
            let v812 = v668[0];
            let v813 = v668[1];
            let v814 = v668[2];
            let v815 = v668[3];
            let v816 = v668[4];
            let v817 = v669[0];
            let v818 = v669[1];
            let v819 = v670[0];
            let v820 = v671[0];
            let v821 = v672[0];
            let v822 = v673[0];
            let v823 = v673[1];
            let v824 = v673[2];
            let v825 = v673[3];
            let v826 = v673[4];
            let v827 = v679[0];
            let v828 = v679[1];
            let v829 = v755[0];
            let v830 = v755[1];
            let v831 = v755[2];
            let v832 = v755[3];
            let v833 = v770[0];
            let v834 = v770[1];
            let v835 = v770[2];
            let v836 = v774[0];
            let v837 = v774[1];
            let v838 = v774[2];
            let v839 = v780[0];
            let v840 = v780[1];
            let v841 = v780[2];
            let v842 = v786[0];
            let v843 = v786[1];
            let v844 = v786[2];
            let v845 = v786[3];
            let v846 = v786[4];
            let v847 = v674[0];
            let v848 = v675[0];
            let v849 = v676[0];
            let v850 = v778[0];
            let v851 = v778[1];
            let v852 = v778[2];
            let v853 = v784[0];
            let v854 = v784[1];
            let v855 = v784[2];
            let v856 = v784[3];
            let v857 = v784[4];
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            None,
            multiplicity * (v544),
            [0, 1, 2, 3, 4],
            [v796, v797, v798, v799, v800],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(6),
            None,
            multiplicity * (v545),
            [6],
            [v801],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(6),
            None,
            multiplicity * (v546),
            [0, 1, 6],
            [v802, v803, v804],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(6), None, 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            staged[22],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            None,
            multiplicity * (v653),
            [0, 1, 2, 3, 4],
            [v805, v806, v807, v808, v809],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(2),
            None,
            multiplicity * (v654),
            [2],
            [v810],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(2),
            None,
            multiplicity * (v655),
            [2],
            [v811],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(5), None, 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            staged[23],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            None,
            multiplicity * (v656),
            [0, 1, 2, 3, 4],
            [v812, v813, v814, v815, v816],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(2),
            Some(5),
            multiplicity * (v657),
            [2, 5],
            [v817, v818],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(2),
            None,
            multiplicity * (v658),
            [2],
            [v819],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(5),
            None,
            multiplicity * (v659),
            [5],
            [v820],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(5),
            None,
            multiplicity * (v660),
            [5],
            [v821],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            None,
            multiplicity * (v661),
            [0, 1, 2, 3, 4],
            [v822, v823, v824, v825, v826],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(5), None, 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            staged[24],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(2), None, 3, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            3,
            staged[25],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(5), None, 4, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            4,
            staged[26],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(3),
            Some(4),
            multiplicity * (v678),
            [3, 4],
            [v827, v828],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(0),
            Some(3),
            multiplicity * (v754),
            [0, 2, 3, 6],
            [v829, v830, v831, v832],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(3),
            multiplicity * (staged[27]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(0), Some(3), 5, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            5,
            staged[28],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(4),
            multiplicity * (v769),
            [1, 2, 4],
            [v833, v834, v835],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(4),
            multiplicity * (staged[29]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(1), Some(4), 6, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            6,
            staged[30],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(3),
            Some(4),
            multiplicity * (v773),
            [2, 3, 4],
            [v836, v837, v838],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(3),
            Some(4),
            multiplicity * (v779),
            [2, 3, 4],
            [v839, v840, v841],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(3),
            Some(4),
            multiplicity * (v785),
            [0, 1, 2, 3, 4],
            [v842, v843, v844, v845, v846],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(3),
            Some(4),
            multiplicity * (v858),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(3),
            Some(4),
            multiplicity * (v859),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = v544;
        self.canonical_reactive[1] = v545;
        self.canonical_reactive[2] = v546;
        self.canonical_reactive[3] = staged[22];
        self.canonical_reactive[4] = v653;
        self.canonical_reactive[5] = v654;
        self.canonical_reactive[6] = v662;
        self.canonical_reactive[7] = v847;
        self.canonical_reactive[8] = staged[23];
        self.canonical_reactive[9] = v656;
        self.canonical_reactive[10] = v657;
        self.canonical_reactive[11] = v663;
        self.canonical_reactive[12] = v848;
        self.canonical_reactive[13] = v659;
        self.canonical_reactive[14] = v664;
        self.canonical_reactive[15] = v849;
        self.canonical_reactive[16] = v661;
        self.canonical_reactive[17] = staged[24];
        self.canonical_reactive[18] = staged[25];
        self.canonical_reactive[19] = staged[26];
        self.canonical_reactive[20] = v678;
        self.canonical_reactive[21] = v754;
        self.canonical_reactive[22] = staged[27];
        self.canonical_reactive[23] = staged[28];
        self.canonical_reactive[24] = v769;
        self.canonical_reactive[25] = staged[29];
        self.canonical_reactive[26] = staged[30];
        self.canonical_reactive[27] = v773;
        self.canonical_reactive[28] = v777;
        self.canonical_reactive[29] = v850;
        self.canonical_reactive[30] = v851;
        self.canonical_reactive[31] = v852;
        self.canonical_reactive[32] = v783;
        self.canonical_reactive[33] = v853;
        self.canonical_reactive[34] = v854;
        self.canonical_reactive[35] = v855;
        self.canonical_reactive[36] = v856;
        self.canonical_reactive[37] = v857;
        self.canonical_reactive[38] = v858;
        self.canonical_reactive[39] = v859;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(2),
            None,
            &[2],
            &[cached[7]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(2),
            None,
            &[2],
            &[cached[12]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            None,
            &[5],
            &[cached[15]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            Some(4),
            &[2, 3, 4],
            &[cached[29], cached[30], cached[31]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            Some(4),
            &[0, 1, 2, 3, 4],
            &[cached[33], cached[34], cached[35], cached[36], cached[37]],
            &[],
            &[],
            multiplicity,
        );
    }

}

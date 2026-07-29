#![allow(dead_code, non_snake_case, unused_imports, unused_mut, unused_parens, unused_variables)]

use super::state::{CanonicalModelValues, Instance, PARAMETER_MODEL_FLAGS};
use rspice_veriloga_runtime::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock, Weak};

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


static CANONICAL_MODEL_CACHE: OnceLock<Mutex<HashMap<Box<[u64]>, Weak<CanonicalModelValues>>>> = OnceLock::new();

fn canonical_model_cache() -> &'static Mutex<HashMap<Box<[u64]>, Weak<CanonicalModelValues>>> {
    CANONICAL_MODEL_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn canonical_model_cache_lookup(key: &[u64]) -> Option<Arc<CanonicalModelValues>> {
    let mut cache = canonical_model_cache()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let found = cache.get(key).and_then(Weak::upgrade);
    if found.is_none() {
        cache.remove(key);
    }
    found
}

fn canonical_model_cache_intern(
    key: Box<[u64]>,
    candidate: Arc<CanonicalModelValues>,
) -> Arc<CanonicalModelValues> {
    let mut cache = canonical_model_cache()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(existing) = cache.get(key.as_ref()).and_then(Weak::upgrade) {
        return existing;
    }
    cache.retain(|_, values| values.strong_count() > 0);
    cache.insert(key, Arc::downgrade(&candidate));
    candidate
}

impl Instance {
    fn canonical_model_key(&self) -> Box<[u64]> {
        let mut key = Vec::with_capacity(98);
        for index in 0..Self::PARAMETER_COUNT {
            if PARAMETER_MODEL_FLAGS[index] {
                key.push(self.params.values[index].to_bits());
                key.push(u64::from(self.param_given[index]));
            }
        }
        key.into_boxed_slice()
    }

    fn canonical_install_model_values(&mut self, values: Arc<CanonicalModelValues>) {
        self.canonical_staged[0] = values[0];
        self.canonical_staged[1] = values[1];
        self.canonical_staged[2] = values[2];
        self.canonical_staged[3] = values[3];
        self.canonical_staged[4] = values[4];
        self.canonical_staged[16] = values[5];
        self.canonical_staged[5] = values[6];
        self.canonical_staged[17] = values[7];
        self.canonical_staged[18] = values[8];
        self.canonical_staged[19] = values[9];
        self.canonical_staged[21] = values[10];
        self.canonical_staged[6] = values[11];
        self.canonical_staged[7] = values[12];
        self.canonical_staged[23] = values[13];
        self.canonical_staged[24] = values[14];
        self.canonical_staged[25] = values[15];
        self.canonical_staged[26] = values[16];
        self.canonical_staged[27] = values[17];
        self.canonical_staged[28] = values[18];
        self.canonical_staged[9] = values[19];
        self.canonical_staged[10] = values[20];
        self.canonical_staged[11] = values[21];
        self.canonical_staged[12] = values[22];
        self.canonical_staged[13] = values[23];
        self.canonical_staged[14] = values[24];
        self.canonical_staged[15] = values[25];
        self.canonical_model_values = Some(values);
    }

    fn canonical_model_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        if self.canonical_model_values.is_some() {
            return;
        }
        let key = self.canonical_model_key();
        if let Some(values) = canonical_model_cache_lookup(key.as_ref()) {
            self.canonical_install_model_values(values);
            return;
        }
        let produced: CanonicalModelValues = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let v0 = parameters[25];
                let v1 = 2.7315e2f64;
                let v3 = 3.0015e2f64;
                let v6 = 4e-4f64;
                let v8 = parameters[49];
                let v9 = 1e0f64;
                let v11 = parameters[51];
                let v13 = 1e0f64;
                let v18 = parameters[31];
                let v20 = parameters[39];
                let v24 = parameters[32];
                let v26 = parameters[44];
                let v28 = 0e0f64;
                let v29 = 0e0f64;
                let v31 = parameters[30];
                let v33 = parameters[33];
                let v36 = 0e0f64;
                let v37 = 2e0f64;
                let v40 = parameters[35];
                let v47 = parameters[13];
                let v49 = parameters[12];
                let v51 = parameters[15];
                let v53 = parameters[14];
                let v55 = -1e0f64;
                let v60 = 0e0f64;
                let v61 = 0e0f64;
                let v62 = 0e0f64;
                let v66 = parameters[28];
                let v68 = parameters[27];
                let mut out27: f64 = 0.0;
                let mut out42: f64 = 0.0;
                let mut out56: f64 = 0.0;
                let v2 = v0 + v1;
                let v4 = v2 / v3;
                let v7 = v6 * (v2 - v3);
                let v10 = v8 - v9;
                let v12 = v11 - v9;
                let v14 = v13 / v8;
                let v15 = v14 - v9;
                let v16 = v13 / v11;
                let v17 = v16 - v9;
                let v19 = if v18 == v13 { 1.0 } else { 0.0 };
                let v21 = v20 - v9;
                let v22 = v13 / v20;
                let v23 = v22 - v9;
                let v25 = if v24 == v13 { 1.0 } else { 0.0 };
                let v30: f64;
                if v25 != 0.0 {
                    let v27 = v26 - v9;
                    out27 = v27;
                    v30 = v28;
                } else {
                    v30 = v29;
                }
                let v34 = if v33 > v28 { 1.0 } else { 0.0 };
                let v35 = if (if v31 == v13 { 1.0 } else { 0.0 }) != 0.0 && v34 != 0.0 { 1.0 } else { 0.0 };
                let v43: f64;
                let v44: f64;
                let v45: f64;
                let v46: f64;
                if v35 != 0.0 {
                    v43 = v36;
                    v44 = v28;
                    v45 = v28;
                    v46 = v28;
                } else {
                    let v42 = if (if (if v31 == v37 { 1.0 } else { 0.0 }) != 0.0 && v34 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v40 > v28 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    out42 = v42;
                    let v57: f64;
                    let v58: f64;
                    let v59: f64;
                    if v42 != 0.0 {
                        v57 = v28;
                        v58 = v28;
                        v59 = v28;
                    } else {
                        let v56 = if v31 == v55 { 1.0 } else { 0.0 };
                        out56 = v56;
                        let v63: f64;
                        let v64: f64;
                        let v65: f64;
                        if v56 != 0.0 {
                            v63 = v60;
                            v64 = v28;
                            v65 = v28;
                        } else {
                            v63 = v28;
                            v64 = v61;
                            v65 = v62;
                        }
                        v57 = v63;
                        v58 = v64;
                        v59 = v65;
                    }
                    v43 = v28;
                    v44 = v57;
                    v45 = v58;
                    v46 = v59;
                }
                let v50 = v49 + (v18 * v47);
                let v54 = v53 + (v18 * v51);
                let v71 = if (if (if v66 > v28 { 1.0 } else { 0.0 }) != 0.0 && (if v68 > v28 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) > v28 { 1.0 } else { 0.0 };
            [v2, v4, v7, v14, v16, v19, v22, v25, v35, out42, out56, v50, v54, v71, v30, v43, v44, v45, v46, v10, v12, v15, v17, v21, v23, out27]
        };
        let values = canonical_model_cache_intern(key, Arc::new(produced));
        self.canonical_install_model_values(values);
    }

    fn canonical_instance_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        if self.canonical_instance_valid {
            return;
        }
        let produced: [f64; 7] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let v0 = parameters[43];
                let v1 = parameters[42];
                let v3 = staged[6];
                let v5 = staged[7];
                let v7 = 0e0f64;
                let v9 = parameters[46];
                let v12 = 0e0f64;
                let v18 = 0e0f64;
                let v19 = 0e0f64;
                let v22 = 0e0f64;
                let v2 = v0 * v1;
                let v4 = v3 / v2;
                let v6 = v5 / v2;
                let v11 = if (if v4 > v7 { 1.0 } else { 0.0 }) != 0.0 && (if v4 >= v9 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v13: f64;
                let v14: f64;
                if v11 != 0.0 {
                    v13 = v18;
                    v14 = v7;
                } else {
                    v13 = v7;
                    v14 = v12;
                }
                let v17 = if (if v6 > v7 { 1.0 } else { 0.0 }) != 0.0 && (if v6 >= v9 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v20: f64;
                let v21: f64;
                if v17 != 0.0 {
                    v20 = v22;
                    v21 = v7;
                } else {
                    v20 = v7;
                    v21 = v19;
                }
            [v2, v11, v17, v13, v14, v20, v21]
        };
        self.canonical_staged[8] = produced[0];
        self.canonical_staged[20] = produced[1];
        self.canonical_staged[22] = produced[2];
        self.canonical_staged[29] = produced[3];
        self.canonical_staged[30] = produced[4];
        self.canonical_staged[31] = produced[5];
        self.canonical_staged[32] = produced[6];
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
        self.canonical_model_stage(ctx);
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
            let v12 = 0e0f64;
            let v13 = 1.7314999999999998e2f64;
            let v17 = 1e0f64;
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
            let v185 = 1e0f64;
            let v187 = 1e0f64;
            let v190 = parameters[29];
            let v193 = node_potentials[0];
            let v195 = 1e0f64;
            let v201 = node_potentials[1];
            let v203 = 1e0f64;
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
            let v372 = staged[9];
            let v377 = parameters[50];
            let v385 = parameters[51];
            let v387 = staged[10];
            let v392 = parameters[37];
            let v397 = parameters[12];
            let v400 = staged[3];
            let v402 = staged[11];
            let v412 = parameters[38];
            let v417 = parameters[14];
            let v420 = staged[4];
            let v422 = staged[12];
            let v432 = staged[16];
            let v451 = parameters[13];
            let v453 = parameters[15];
            let v463 = parameters[40];
            let v471 = parameters[39];
            let v473 = staged[13];
            let v478 = staged[5];
            let v480 = staged[14];
            let v485 = parameters[41];
            let v489 = parameters[19];
            let v498 = staged[17];
            let v507 = node_potentials[6];
            let v509 = 1e0f64;
            let v510 = ddt_scale();
            let v529 = parameters[20];
            let v532 = parameters[44];
            let v534 = staged[15];
            let v546 = Lanes([0e0f64; 5]);
            let v547 = 0e0f64;
            let v548 = Lanes([0e0f64; 3]);
            let v561 = parameters[24];
            let v569 = -1e0f64;
            let v583 = 5e-1f64;
            let v635 = staged[18];
            let v647 = -1e0f64;
            let v650 = parameters[33];
            let v653 = parameters[34];
            let v658 = Lanes([0e0f64; 2]);
            let v659 = 0e0f64;
            let v660 = staged[19];
            let v688 = staged[20];
            let v700 = -1e0f64;
            let v703 = node_potentials[5];
            let v706 = 1e0f64;
            let v715 = parameters[35];
            let v718 = parameters[36];
            let v723 = staged[21];
            let v751 = -1e0f64;
            let v756 = staged[8];
            let v759 = parameters[46];
            let v761 = Lanes([0e0f64; 4]);
            let v764 = staged[22];
            let v776 = Lanes([0e0f64; 3]);
            let v869 = 0e0f64;
            let v870 = 0e0f64;
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
            let v16: f64;
            if v10 != 0.0 {
                v15 = v11;
                v16 = v12;
            } else {
                let v14 = if v4 > v13 { 1.0 } else { 0.0 };
                let v19: f64;
                let v20: f64;
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
            let v189 = (Lanes([v185, 0.0])) - (Lanes([0.0, v187]));
            let v191 = v190 * v184;
            let v192 = v189 * v190;
            let v194 = v193 - v182;
            let v198 = (Lanes([v195, 0.0])) - (Lanes([0.0, v185]));
            let v199 = v190 * v194;
            let v200 = v198 * v190;
            let v202 = v201 - v183;
            let v206 = (Lanes([v203, 0.0])) - (Lanes([0.0, v187]));
            let v207 = v190 * v202;
            let v208 = v206 * v190;
            let v210 = if v53 > v209 { 1.0 } else { 0.0 };
            let v243: f64;
            let v244: Lanes<3>;
            if v210 != 0.0 {
                let v212 = v211 * v24;
                let v214 = v191 / v212;
                let v219 = ((Lanes([0.0, v192[0], v192[1]])) - (Lanes([((v25 * v211) * v214), 0.0, 0.0]))) / v212;
                let v221 = v192 * v99;
                let v227 = v226 * v24;
                let v228 = v25 * v226;
                let v229 = ((-v191) - v72) / v227;
                let v233 = (((Lanes([0.0, v221[0], v221[1]])) - (Lanes([v73, 0.0, 0.0]))) - (Lanes([(v228 * v229), 0.0, 0.0]))) / v227;
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
                let v278: f64;
                if v264 != 0.0 {
                    v277 = v236;
                    v278 = v239;
                } else {
                    let v276 = if v236 <= v275 { 1.0 } else { 0.0 };
                    let v329: f64;
                    let v330: f64;
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
                let v293 = v191.abs();
                let v300 = v293.powf(v79);
                let v304 = (v192 * ((v296 * (if v191 >= v294 { 1.0 } else { 0.0 })) - v30)) * (v79 * (v293.powf((v79 - v30))));
                let v314 = v36 + (v311 * v300);
                let v315 = (v65 * v279) / v314;
                let v319 = (v53 * v282) - v315;
                let v320 = ((Lanes([(v54 * v282), 0.0, 0.0])) + (v257 * v53)) - ((((Lanes([(v66 * v279), 0.0, 0.0])) + ((v263 - (Lanes([v278, 0.0, 0.0]))) * v65)) - ((((Lanes([0.0, v304[0], v304[1]])) + (Lanes([(v80 * (v300 * (v293.ln()))), 0.0, 0.0]))) * v311) * v315)) / v314);
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
                let v348 = ((v192 * v99) * (if v332 >= v334 { 1.0 } else { 0.0 })) * v344;
                let v352 = ((v338 * v191) * v331) / v346;
                let v356 = ((Lanes([0.0, v342[0], v342[1]])) - (((Lanes([((v25 * v343) * v335), 0.0, 0.0])) + (Lanes([0.0, v348[0], v348[1]]))) * v352)) / v346;
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
                let v450 = (Lanes([(v59 * v445), 0.0, 0.0])) + (((v437 * v439) + ((v438 * v439) * v435)) * v58);
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
            let v408 = ((((v200 / v362) * ((v296 * (if v363 >= v294 { 1.0 } else { 0.0 })) - v30)) * (v370 * (v365.powf(v372)))) * (v400 * (v376.powf(v402)))) * v398;
            let v411 = (Lanes([0.0, ((((v32 * v392) * v395) * v397) * v401), 0.0])) + (Lanes([v408[0], 0.0, v408[1]]));
            let v415 = (v29 * v412).exp();
            let v418 = v417 * v415;
            let v421 = v391.powf(v420);
            let v426 = v418 * v421;
            let v428 = ((((v208 / v377) * ((v296 * (if v378 >= v294 { 1.0 } else { 0.0 })) - v30)) * (v385 * (v380.powf(v387)))) * (v420 * (v391.powf(v422)))) * v418;
            let v431 = (Lanes([0.0, ((((v32 * v412) * v415) * v417) * v421), 0.0])) + (Lanes([v428[0], 0.0, v428[1]]));
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
            let v462 = (Lanes([v195, 0.0])) - (Lanes([0.0, v203]));
            let v464 = v459 / v463;
            let v466 = v464.abs();
            let v477 = v36 + (v466.powf(v471));
            let v490 = v489 * (v36 + (v485 * ((v477.powf(v478)) - v36)));
            let v491 = (((((v462 / v463) * ((v296 * (if v464 >= v294 { 1.0 } else { 0.0 })) - v30)) * (v471 * (v466.powf(v473)))) * (v478 * (v477.powf(v480)))) * v485) * v489;
            let v492 = v490 * v243;
            let v493 = v491 * v243;
            let v494 = v244 * v490;
            let v497 = (Lanes([v493[0], v493[1], 0.0, 0.0, 0.0])) + (Lanes([0.0, 0.0, v494[0], v494[1], v494[2]]));
            let v549: f64;
            let v550: f64;
            let v551: f64;
            let v552: f64;
            let v553: f64;
            let v554: Lanes<4>;
            let v555: Lanes<5>;
            let v556: f64;
            let v557: Lanes<3>;
            let v558: Lanes<3>;
            if v498 != 0.0 {
                let v499 = -v243;
                let v501 = v499 * v490;
                let v502 = (v244 * v99) * v490;
                let v503 = v491 * v499;
                let v506 = (Lanes([0.0, 0.0, v502[0], v502[1], v502[2]])) + (Lanes([v503[0], v503[1], 0.0, 0.0, 0.0]));
                let v508 = ddt(1504, v507);
                let v512 = v490 * v508;
                let v513 = v491 * v508;
                let v517 = (Lanes([v513[0], v513[1], 0.0])) + (Lanes([0.0, 0.0, ((v509 * v510) * v490)]));
                let v518 = v490 * v507;
                let v519 = v491 * v507;
                let v523 = (Lanes([v519[0], v519[1], 0.0])) + (Lanes([0.0, 0.0, (v509 * v490)]));
                let v530 = (v507.abs()) / v529;
                let v538 = v36 + (v530.powf(v532));
                let v539 = v455 / v538;
                let v544 = ((Lanes([v457[0], v457[1], v457[2], 0.0])) - (Lanes([0.0, 0.0, 0.0, ((((v509 * ((v296 * (if v507 >= v294 { 1.0 } else { 0.0 })) - v30)) / v529) * (v532 * (v530.powf(v534)))) * v539)]))) / v538;
                v549 = v539;
                v550 = v501;
                v551 = v507;
                v552 = v512;
                v553 = v518;
                v554 = v544;
                v555 = v506;
                v556 = v509;
                v557 = v517;
                v558 = v523;
            } else {
                let v545 = Lanes([v457[0], v457[1], v457[2], 0.0]);
                v549 = v455;
                v550 = v209;
                v551 = v209;
                v552 = v209;
                v553 = v209;
                v554 = v545;
                v555 = v546;
                v556 = v547;
                v557 = v548;
                v558 = v548;
            }
            let v564 = v191 + ((-v161) * v561);
            let v565 = Lanes([0.0, v192[0], v192[1]]);
            let v567 = v565 + (Lanes([((v162 * v99) * v561), 0.0, 0.0]));
            let v568 = if v564 > v209 { 1.0 } else { 0.0 };
            let v624: f64;
            let v625: f64;
            let v626: Lanes<3>;
            let v627: Lanes<3>;
            if v568 != 0.0 {
                let v571 = v36 - v561;
                let v574 = ((v569 - v148) * (v571.ln())).exp();
                let v577 = v36 - ((v574 * v571) * v571);
                let v580 = v36 - v148;
                let v581 = (v161 * v577) / v580;
                let v584 = v583 * v148;
                let v587 = (v584 * v564) / v161;
                let v592 = v571 + v587;
                let v597 = (v564 * v592) * v574;
                let v598 = ((v567 * v592) + ((((v567 * v584) - (Lanes([(v162 * v587), 0.0, 0.0]))) / v161) * v564)) * v574;
                let v599 = Lanes([((v162 * v577) / v580), 0.0, 0.0]);
                v624 = v581;
                v625 = v597;
                v626 = v599;
                v627 = v598;
            } else {
                let v600 = v36 - v148;
                let v601 = v191 / v161;
                let v606 = v36 - v601;
                let v613 = (v600 * (v606.ln())).exp();
                let v615 = v36 - v613;
                let v622 = (v161 * v615) / v600;
                let v623 = ((Lanes([(v162 * v615), 0.0, 0.0])) + ((((((((v565 - (Lanes([(v162 * v601), 0.0, 0.0]))) / v161) * v99) * (v30 / v606)) * v600) * v613) * v99) * v161)) / v600;
                v624 = v622;
                v625 = v209;
                v626 = v623;
                v627 = v242;
            }
            let v628 = v624 + v625;
            let v630 = v178 * v628;
            let v634 = (Lanes([(v181 * v628), 0.0, 0.0])) + ((v626 + v627) * v178);
            let v661: f64;
            let v662: f64;
            let v663: f64;
            let v664: f64;
            let v665: f64;
            let v666: f64;
            let v667: f64;
            let v668: f64;
            let v669: f64;
            let v670: f64;
            let v671: f64;
            let v672: f64;
            let v673: Lanes<5>;
            let v674: f64;
            let v675: f64;
            let v676: Lanes<5>;
            let v677: Lanes<2>;
            let v678: f64;
            let v679: f64;
            let v680: f64;
            let v681: Lanes<5>;
            let v682: f64;
            let v683: f64;
            let v684: f64;
            if v635 != 0.0 {
                let v636 = v360 * v459;
                let v637 = v361 * v459;
                let v638 = v462 * v360;
                let v648 = v647 * (v636.abs());
                let v649 = (((Lanes([0.0, 0.0, v637[0], v637[1], v637[2]])) + (Lanes([v638[0], v638[1], 0.0, 0.0, 0.0]))) * ((v296 * (if v636 >= v294 { 1.0 } else { 0.0 })) - v30)) * v647;
                let v651 = v1 / v650;
                let v652 = v17 / v650;
                let v654 = v1 * v653;
                let v655 = v17 * v653;
                let v656 = ddt(1615, v654);
                let v657 = v655 * v510;
                v661 = v648;
                v662 = v651;
                v663 = v656;
                v664 = v209;
                v665 = v209;
                v666 = v209;
                v667 = v209;
                v668 = v209;
                v669 = v209;
                v670 = v654;
                v671 = v209;
                v672 = v209;
                v673 = v649;
                v674 = v652;
                v675 = v657;
                v676 = v546;
                v677 = v658;
                v678 = v12;
                v679 = v659;
                v680 = v659;
                v681 = v546;
                v682 = v655;
                v683 = v12;
                v684 = v659;
            } else {
                let v724: f64;
                let v725: f64;
                let v726: f64;
                let v727: f64;
                let v728: f64;
                let v729: f64;
                let v730: f64;
                let v731: f64;
                let v732: Lanes<5>;
                let v733: Lanes<2>;
                let v734: f64;
                let v735: f64;
                let v736: f64;
                let v737: Lanes<5>;
                let v738: f64;
                let v739: f64;
                if v660 != 0.0 {
                    let v689 = v360 * v459;
                    let v690 = v361 * v459;
                    let v691 = v462 * v360;
                    let v701 = v700 * (v689.abs());
                    let v702 = (((Lanes([0.0, 0.0, v690[0], v690[1], v690[2]])) + (Lanes([v691[0], v691[1], 0.0, 0.0, 0.0]))) * ((v296 * (if v689 >= v294 { 1.0 } else { 0.0 })) - v30)) * v700;
                    let v709 = (v1 - v703) / v650;
                    let v710 = ((Lanes([v17, 0.0])) - (Lanes([0.0, v706]))) / v650;
                    let v711 = v653 * v1;
                    let v712 = v17 * v653;
                    let v713 = ddt(1641, v711);
                    let v714 = v712 * v510;
                    let v716 = v703 / v715;
                    let v717 = v706 / v715;
                    let v719 = v718 * v703;
                    let v720 = v706 * v718;
                    let v721 = ddt(1648, v719);
                    let v722 = v720 * v510;
                    v724 = v701;
                    v725 = v709;
                    v726 = v713;
                    v727 = v716;
                    v728 = v721;
                    v729 = v209;
                    v730 = v711;
                    v731 = v719;
                    v732 = v702;
                    v733 = v710;
                    v734 = v714;
                    v735 = v717;
                    v736 = v722;
                    v737 = v546;
                    v738 = v712;
                    v739 = v720;
                } else {
                    let v754: f64;
                    let v755: Lanes<5>;
                    if v723 != 0.0 {
                        let v740 = v360 * v459;
                        let v741 = v361 * v459;
                        let v742 = v462 * v360;
                        let v752 = v751 * (v740.abs());
                        let v753 = (((Lanes([0.0, 0.0, v741[0], v741[1], v741[2]])) + (Lanes([v742[0], v742[1], 0.0, 0.0, 0.0]))) * ((v296 * (if v740 >= v294 { 1.0 } else { 0.0 })) - v30)) * v751;
                        v754 = v752;
                        v755 = v753;
                    } else {
                        v754 = v209;
                        v755 = v546;
                    }
                    v724 = v209;
                    v725 = v209;
                    v726 = v209;
                    v727 = v209;
                    v728 = v209;
                    v729 = v754;
                    v730 = v209;
                    v731 = v209;
                    v732 = v546;
                    v733 = v658;
                    v734 = v12;
                    v735 = v659;
                    v736 = v659;
                    v737 = v755;
                    v738 = v12;
                    v739 = v659;
                }
                v661 = v209;
                v662 = v209;
                v663 = v209;
                v664 = v724;
                v665 = v725;
                v666 = v726;
                v667 = v727;
                v668 = v728;
                v669 = v729;
                v670 = v209;
                v671 = v730;
                v672 = v731;
                v673 = v546;
                v674 = v12;
                v675 = v12;
                v676 = v732;
                v677 = v733;
                v678 = v734;
                v679 = v735;
                v680 = v736;
                v681 = v737;
                v682 = v12;
                v683 = v738;
                v684 = v739;
            }
            let v685 = ctx.simparam_or("gmin", v209);
            let v686 = v685 * v184;
            let v687 = v189 * v685;
            let v762: f64;
            let v763: Lanes<4>;
            if v688 != 0.0 {
                let v757 = v549 / v756;
                let v758 = v554 / v756;
                let v760 = if v757 > v759 { 1.0 } else { 0.0 };
                let v765: f64;
                let v766: Lanes<4>;
                if v760 != 0.0 {
                    v765 = v757;
                    v766 = v758;
                } else {
                    v765 = v759;
                    v766 = v761;
                }
                let v767 = v194 / v765;
                let v771 = ((Lanes([v198[0], 0.0, v198[1], 0.0])) - (v766 * v767)) / v765;
                let v772 = if v757 >= v759 { 1.0 } else { 0.0 };
                v762 = v767;
                v763 = v771;
            } else {
                v762 = v209;
                v763 = v761;
            }
            let v777: f64;
            let v778: Lanes<3>;
            if v764 != 0.0 {
                let v773 = v456 / v756;
                let v774 = v458 / v756;
                let v775 = if v773 > v759 { 1.0 } else { 0.0 };
                let v795: f64;
                let v796: Lanes<3>;
                if v775 != 0.0 {
                    v795 = v773;
                    v796 = v774;
                } else {
                    v795 = v759;
                    v796 = v776;
                }
                let v797 = v202 / v795;
                let v801 = ((Lanes([v206[0], 0.0, v206[1]])) - (v796 * v797)) / v795;
                let v802 = if v773 >= v759 { 1.0 } else { 0.0 };
                v777 = v797;
                v778 = v801;
            } else {
                v777 = v209;
                v778 = v776;
            }
            let v779 = v190 * v360;
            let v781 = v779 * v756;
            let v782 = (v361 * v190) * v756;
            let v785 = (v190 * v630) * v756;
            let v786 = (v634 * v190) * v756;
            let v787 = ddt(1767, v785);
            let v788 = v786 * v510;
            let v791 = (v190 * v492) * v756;
            let v792 = (v497 * v190) * v756;
            let v793 = ddt(1773, v791);
            let v794 = v792 * v510;
            let v803 = if v779 >= v209 { 1.0 } else { 0.0 };
            let v804 = v555[0];
            let v805 = v555[1];
            let v806 = v555[2];
            let v807 = v555[3];
            let v808 = v555[4];
            let v809 = v556;
            let v810 = v557[0];
            let v811 = v557[1];
            let v812 = v557[2];
            let v813 = v673[0];
            let v814 = v673[1];
            let v815 = v673[2];
            let v816 = v673[3];
            let v817 = v673[4];
            let v818 = v674;
            let v819 = v675;
            let v820 = v676[0];
            let v821 = v676[1];
            let v822 = v676[2];
            let v823 = v676[3];
            let v824 = v676[4];
            let v825 = v677[0];
            let v826 = v677[1];
            let v827 = v678;
            let v828 = v679;
            let v829 = v680;
            let v830 = v681[0];
            let v831 = v681[1];
            let v832 = v681[2];
            let v833 = v681[3];
            let v834 = v681[4];
            let v835 = v687[0];
            let v836 = v687[1];
            let v837 = v763[0];
            let v838 = v763[1];
            let v839 = v763[2];
            let v840 = v763[3];
            let v841 = v778[0];
            let v842 = v778[1];
            let v843 = v778[2];
            let v844 = v782[0];
            let v845 = v782[1];
            let v846 = v782[2];
            let v847 = v788[0];
            let v848 = v788[1];
            let v849 = v788[2];
            let v850 = v794[0];
            let v851 = v794[1];
            let v852 = v794[2];
            let v853 = v794[3];
            let v854 = v794[4];
            let v855 = v558[0];
            let v856 = v558[1];
            let v857 = v558[2];
            let v858 = v682;
            let v859 = v683;
            let v860 = v684;
            let v861 = v786[0];
            let v862 = v786[1];
            let v863 = v786[2];
            let v864 = v792[0];
            let v865 = v792[1];
            let v866 = v792[2];
            let v867 = v792[3];
            let v868 = v792[4];
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            None,
            multiplicity * (v550),
            [0, 1, 2, 3, 4],
            [v804, v805, v806, v807, v808],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(6),
            None,
            multiplicity * (v551),
            [6],
            [v809],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(6),
            None,
            multiplicity * (v552),
            [0, 1, 6],
            [v810, v811, v812],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(6), None, 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            staged[24],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            None,
            multiplicity * (v661),
            [0, 1, 2, 3, 4],
            [v813, v814, v815, v816, v817],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(2),
            None,
            multiplicity * (v662),
            [2],
            [v818],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(2),
            None,
            multiplicity * (v663),
            [2],
            [v819],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(5), None, 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            staged[25],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            None,
            multiplicity * (v664),
            [0, 1, 2, 3, 4],
            [v820, v821, v822, v823, v824],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(2),
            Some(5),
            multiplicity * (v665),
            [2, 5],
            [v825, v826],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(2),
            None,
            multiplicity * (v666),
            [2],
            [v827],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(5),
            None,
            multiplicity * (v667),
            [5],
            [v828],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(5),
            None,
            multiplicity * (v668),
            [5],
            [v829],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            None,
            multiplicity * (v669),
            [0, 1, 2, 3, 4],
            [v830, v831, v832, v833, v834],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(5), None, 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            staged[26],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(2), None, 3, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            3,
            staged[27],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(5), None, 4, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            4,
            staged[28],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(3),
            Some(4),
            multiplicity * (v686),
            [3, 4],
            [v835, v836],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(0),
            Some(3),
            multiplicity * (v762),
            [0, 2, 3, 6],
            [v837, v838, v839, v840],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(3),
            multiplicity * (staged[29]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(0), Some(3), 5, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            5,
            staged[30],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(4),
            multiplicity * (v777),
            [1, 2, 4],
            [v841, v842, v843],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(4),
            multiplicity * (staged[31]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(1), Some(4), 6, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            6,
            staged[32],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(3),
            Some(4),
            multiplicity * (v781),
            [2, 3, 4],
            [v844, v845, v846],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(3),
            Some(4),
            multiplicity * (v787),
            [2, 3, 4],
            [v847, v848, v849],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(3),
            Some(4),
            multiplicity * (v793),
            [0, 1, 2, 3, 4],
            [v850, v851, v852, v853, v854],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(3),
            Some(4),
            multiplicity * (v869),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(3),
            Some(4),
            multiplicity * (v870),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = v550;
        self.canonical_reactive[1] = v551;
        self.canonical_reactive[2] = v553;
        self.canonical_reactive[3] = v855;
        self.canonical_reactive[4] = v856;
        self.canonical_reactive[5] = v857;
        self.canonical_reactive[6] = staged[24];
        self.canonical_reactive[7] = v661;
        self.canonical_reactive[8] = v662;
        self.canonical_reactive[9] = v670;
        self.canonical_reactive[10] = v858;
        self.canonical_reactive[11] = staged[25];
        self.canonical_reactive[12] = v664;
        self.canonical_reactive[13] = v665;
        self.canonical_reactive[14] = v671;
        self.canonical_reactive[15] = v859;
        self.canonical_reactive[16] = v667;
        self.canonical_reactive[17] = v672;
        self.canonical_reactive[18] = v860;
        self.canonical_reactive[19] = v669;
        self.canonical_reactive[20] = staged[26];
        self.canonical_reactive[21] = staged[27];
        self.canonical_reactive[22] = staged[28];
        self.canonical_reactive[23] = v686;
        self.canonical_reactive[24] = v762;
        self.canonical_reactive[25] = staged[29];
        self.canonical_reactive[26] = staged[30];
        self.canonical_reactive[27] = v777;
        self.canonical_reactive[28] = staged[31];
        self.canonical_reactive[29] = staged[32];
        self.canonical_reactive[30] = v781;
        self.canonical_reactive[31] = v785;
        self.canonical_reactive[32] = v861;
        self.canonical_reactive[33] = v862;
        self.canonical_reactive[34] = v863;
        self.canonical_reactive[35] = v791;
        self.canonical_reactive[36] = v864;
        self.canonical_reactive[37] = v865;
        self.canonical_reactive[38] = v866;
        self.canonical_reactive[39] = v867;
        self.canonical_reactive[40] = v868;
        self.canonical_reactive[41] = v869;
        self.canonical_reactive[42] = v870;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            None,
            &[0, 1, 6],
            &[cached[3], cached[4], cached[5]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(2),
            None,
            &[2],
            &[cached[10]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(2),
            None,
            &[2],
            &[cached[15]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            None,
            &[5],
            &[cached[18]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            Some(4),
            &[2, 3, 4],
            &[cached[32], cached[33], cached[34]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            Some(4),
            &[0, 1, 2, 3, 4],
            &[cached[36], cached[37], cached[38], cached[39], cached[40]],
            &[],
            &[],
            multiplicity,
        );
    }

}

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
        let mut key = Vec::with_capacity(72);
        for index in 0..Self::PARAMETER_COUNT {
            if PARAMETER_MODEL_FLAGS[index] {
                key.push(self.params.values[index].to_bits());
                key.push(u64::from(self.param_given[index]));
            }
        }
        key.into_boxed_slice()
    }

    fn canonical_install_model_values(&mut self, values: Arc<CanonicalModelValues>) {
        self.canonical_staged[5] = values[0];
        self.canonical_staged[6] = values[1];
        self.canonical_staged[1] = values[2];
        self.canonical_staged[0] = values[3];
        self.canonical_staged[3] = values[4];
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
            let parameter_given = &*self.param_given;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let v0 = parameters[14];
                let v1 = 1.002e3f64;
                let v3 = if parameter_given[10] { 1.0 } else { 0.0 };
                let v4 = 1e-2f64;
                let v5 = parameters[10];
                let v7 = 1e0f64;
                let v9 = 2.7315e2f64;
                let v10 = parameters[15];
                let v12 = parameters[34];
                let v14 = parameters[28];
                let v15 = 0e0f64;
                let v17 = parameters[26];
                let mut out8: f64 = 0.0;
                let v2 = if v0 != v1 { 1.0 } else { 0.0 };
                if v3 != 0.0 {
                    let v8 = v7 - (v4 * v5);
                    out8 = v8;
                } else {
                }
                let v11 = v9 + v10;
                let v13 = v12 + v7;
                let v19 = if (if v14 > v15 { 1.0 } else { 0.0 }) != 0.0 || (if v17 > v15 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            [v2, out8, v11, v13, v19]
        };
        let values = canonical_model_cache_intern(key, Arc::new(produced));
        self.canonical_install_model_values(values);
    }

    fn canonical_instance_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        if self.canonical_instance_valid {
            return;
        }
        let produced: [f64; 10] = {
            let parameters = &self.params.values;
            let parameter_given = &*self.param_given;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let v0 = parameters[3];
                let v1 = parameters[4];
                let v3 = parameters[22];
                let v6 = if parameter_given[1] { 1.0 } else { 0.0 };
                let v7 = if parameter_given[2] { 1.0 } else { 0.0 };
                let v9 = if parameter_given[0] { 1.0 } else { 0.0 };
                let v12 = 5e-1f64;
                let v14 = 0e0f64;
                let v16 = parameters[2];
                let v18 = parameters[1];
                let v24 = parameters[0];
                let mut out4: f64 = 0.0;
                let mut out20: f64 = 0.0;
                let mut out22: f64 = 0.0;
                let mut out23: f64 = 0.0;
                let mut out25: f64 = 0.0;
                let mut out26: f64 = 0.0;
                let mut out27: f64 = 0.0;
                let v2 = if v0 != 0.0 && v1 != 0.0 { 1.0 } else { 0.0 };
                let v5: f64;
                if v2 != 0.0 {
                    v5 = v3;
                } else {
                    let v4 = if v0 != 0.0 || v1 != 0.0 { 1.0 } else { 0.0 };
                    out4 = v4;
                    let v15: f64;
                    if v4 != 0.0 {
                        let v13 = v3 * v12;
                        v15 = v13;
                    } else {
                        v15 = v14;
                    }
                    v5 = v15;
                }
                let v11 = if (if v6 != 0.0 && v7 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v9 == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v11 != 0.0 {
                    let v20 = if (if v16 == v14 { 1.0 } else { 0.0 }) != 0.0 || (if v18 == v14 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    out20 = v20;
                } else {
                    let v22 = if v7 != 0.0 && (if v6 == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    out22 = v22;
                    if v22 != 0.0 {
                        let v23 = if v16 == v14 { 1.0 } else { 0.0 };
                        out23 = v23;
                        if v23 != 0.0 {
                        } else {
                            let v26 = if v24 == v14 { 1.0 } else { 0.0 };
                            out26 = v26;
                        }
                    } else {
                        let v25 = if v24 == v14 { 1.0 } else { 0.0 };
                        out25 = v25;
                        if v25 != 0.0 {
                        } else {
                            let v27 = if v18 == v14 { 1.0 } else { 0.0 };
                            out27 = v27;
                        }
                    }
                }
            [v2, out4, v11, out20, v5, out22, out23, out26, out25, out27]
        };
        self.canonical_staged[12] = produced[0];
        self.canonical_staged[13] = produced[1];
        self.canonical_staged[14] = produced[2];
        self.canonical_staged[15] = produced[3];
        self.canonical_staged[2] = produced[4];
        self.canonical_staged[16] = produced[5];
        self.canonical_staged[17] = produced[6];
        self.canonical_staged[19] = produced[7];
        self.canonical_staged[18] = produced[8];
        self.canonical_staged[20] = produced[9];
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
        let produced: [f64; 6] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let temperature = ctx.temperature();
            let staged = &*self.canonical_staged;
                let v0 = temperature;
                let v1 = parameters[5];
                let v3 = 2.7315e2f64;
                let v5 = parameters[11];
                let v7 = parameters[12];
                let v9 = staged[0];
                let v11 = parameters[34];
                let v13 = 1e0f64;
                let v17 = parameters[35];
                let v22 = staged[1];
                let v24 = parameters[42];
                let v27 = parameters[29];
                let v29 = 0e0f64;
                let mut out19: f64 = 0.0;
                let v4 = (v0 + v1) - v3;
                let v6 = if v4 < v5 { 1.0 } else { 0.0 };
                let v8 = if v4 > v7 { 1.0 } else { 0.0 };
                let v10 = if v4 < v9 { 1.0 } else { 0.0 };
                let v20: f64;
                if v10 != 0.0 {
                    let v16 = v11 + (((v4 - v11) - v13).exp());
                    v20 = v16;
                } else {
                    let v19 = if v4 > (v17 - v13) { 1.0 } else { 0.0 };
                    out19 = v19;
                    let v35: f64;
                    if v19 != 0.0 {
                        let v34 = v17 - (((v17 - v4) - v13).exp());
                        v35 = v34;
                    } else {
                        v35 = v4;
                    }
                    v20 = v35;
                }
                let v23 = (v20 + v3) - v22;
                let v30 = if ((v13 + (v23 * v24)) * v27) < v29 { 1.0 } else { 0.0 };
            [v6, v8, v10, out19, v23, v30]
        };
        self.canonical_staged[7] = produced[0];
        self.canonical_staged[8] = produced[1];
        self.canonical_staged[9] = produced[2];
        self.canonical_staged[10] = produced[3];
        self.canonical_staged[4] = produced[4];
        self.canonical_staged[11] = produced[5];
        self.canonical_temperature = temperature;
        self.canonical_thermal_voltage = thermal_voltage;
        self.canonical_temperature_valid = true;
    }

    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        self.canonical_model_stage(ctx);
        self.canonical_instance_stage(ctx);
        self.canonical_temperature_stage(ctx);
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let multiplicity = self.multiplicity;
        let staged = &*self.canonical_staged;
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1])];
            let v0 = if parameter_given[9] { 1.0 } else { 0.0 };
            let v1 = parameters[9];
            let v2 = 1e0f64;
            let v5 = if parameter_given[10] { 1.0 } else { 0.0 };
            let v6 = staged[6];
            let v7 = 0e0f64;
            let v9 = 1e-2f64;
            let v13 = if parameter_given[13] { 1.0 } else { 0.0 };
            let v14 = parameters[13];
            let v15 = 1e-3f64;
            let v19 = 1e6f64;
            let v21 = staged[12];
            let v22 = staged[14];
            let v23 = staged[15];
            let v24 = staged[16];
            let v31 = parameters[17];
            let v33 = parameters[0];
            let v35 = parameters[21];
            let v37 = 1e99f64;
            let v38 = parameters[1];
            let v40 = staged[2];
            let v50 = parameters[16];
            let v51 = parameters[2];
            let v63 = staged[17];
            let v64 = staged[18];
            let v73 = staged[19];
            let v105 = staged[20];
            let v135 = parameters[18];
            let v137 = parameters[19];
            let v139 = parameters[20];
            let v141 = parameters[24];
            let v142 = parameters[23];
            let v149 = staged[3];
            let v152 = parameters[36];
            let v153 = parameters[37];
            let v157 = parameters[38];
            let v160 = parameters[39];
            let v163 = parameters[3];
            let v164 = parameters[4];
            let v168 = 5e-1f64;
            let v177 = parameters[40];
            let v180 = parameters[41];
            let v185 = multiplicity;
            let v188 = staged[4];
            let v193 = 1.1e-1f64;
            let v196 = 1e1f64;
            let v200 = 1e-1f64;
            let v205 = node_potentials[0];
            let v206 = node_potentials[1];
            let v208 = 1e0f64;
            let v210 = 1e0f64;
            let v216 = parameters[27];
            let v224 = 2e0f64;
            let v226 = 1e0f64;
            let v230 = 0e0f64;
            let v235 = parameters[25];
            let v246 = 3.333333333333333e-1f64;
            let v248 = -6.666666666666667e-1f64;
            let v252 = parameters[28];
            let v254 = parameters[26];
            let v263 = Lanes([0e0f64; 2]);
            let v274 = parameters[33];
            let v276 = parameters[6];
            let v280 = parameters[32];
            let v292 = 0e0f64;
            let v293 = 0e0f64;
            let v4: f64;
            if v0 != 0.0 {
                v4 = v1;
            } else {
                let v3 = ctx.simparam_or("scale", v2);
                v4 = v3;
            }
            let v12: f64;
            if v5 != 0.0 {
                v12 = v6;
            } else {
                let v11 = v2 - (v9 * (ctx.simparam_or("shrink", v7)));
                v12 = v11;
            }
            let v17: f64;
            if v13 != 0.0 {
                v17 = v14;
            } else {
                let v16 = ctx.simparam_or("rthresh", v15);
                v17 = v16;
            }
            let v20 = (v12 * v4) * v19;
            let v25: f64;
            let v26: f64;
            let v27: f64;
            let v28: f64;
            let v29: f64;
            let v30: f64;
            if v22 != 0.0 {
                let v43: f64;
                let v44: f64;
                let v45: f64;
                let v46: f64;
                let v47: f64;
                let v48: f64;
                if v23 != 0.0 {
                    let v34 = v33 * v20;
                    let v36 = v34 + v35;
                    v43 = v7;
                    v44 = v34;
                    v45 = v7;
                    v46 = v7;
                    v47 = v36;
                    v48 = v37;
                } else {
                    let v39 = v38 * v20;
                    let v41 = v39 + v40;
                    let v42 = if v41 < v7 { 1.0 } else { 0.0 };
                    let v49 = if v41 > v7 { 1.0 } else { 0.0 };
                    let v58: f64;
                    let v59: f64;
                    let v60: f64;
                    let v61: f64;
                    if v49 != 0.0 {
                        let v53 = (v50 / v51) * v41;
                        let v54 = v53 - v35;
                        let v55 = if v54 <= v7 { 1.0 } else { 0.0 };
                        let v62 = v2 / v51;
                        v58 = v54;
                        v59 = v51;
                        v60 = v53;
                        v61 = v62;
                    } else {
                        let v56 = v33 * v20;
                        let v57 = v56 + v35;
                        v58 = v56;
                        v59 = v7;
                        v60 = v57;
                        v61 = v37;
                    }
                    v43 = v39;
                    v44 = v58;
                    v45 = v41;
                    v46 = v59;
                    v47 = v60;
                    v48 = v61;
                }
                v25 = v43;
                v26 = v44;
                v27 = v45;
                v28 = v46;
                v29 = v47;
                v30 = v48;
            } else {
                let v65: f64;
                let v66: f64;
                let v67: f64;
                let v68: f64;
                let v69: f64;
                let v70: f64;
                if v24 != 0.0 {
                    let v74: f64;
                    let v75: f64;
                    let v76: f64;
                    let v77: f64;
                    let v78: f64;
                    let v79: f64;
                    if v63 != 0.0 {
                        let v71 = v33 * v20;
                        let v72 = v71 + v35;
                        v74 = v7;
                        v75 = v71;
                        v76 = v7;
                        v77 = v7;
                        v78 = v72;
                        v79 = v37;
                    } else {
                        let v85: f64;
                        let v86: f64;
                        let v87: f64;
                        let v88: f64;
                        let v89: f64;
                        let v90: f64;
                        if v73 != 0.0 {
                            let v80 = v38 * v20;
                            let v81 = v80 + v40;
                            v85 = v80;
                            v86 = v7;
                            v87 = v81;
                            v88 = v37;
                            v89 = v7;
                            v90 = v7;
                        } else {
                            let v82 = v33 * v20;
                            let v83 = v82 + v35;
                            let v84 = if v83 < v7 { 1.0 } else { 0.0 };
                            let v91 = if v83 > v7 { 1.0 } else { 0.0 };
                            let v98: f64;
                            let v99: f64;
                            let v100: f64;
                            let v101: f64;
                            if v91 != 0.0 {
                                let v93 = (v51 / v50) * v83;
                                let v94 = v93 - v40;
                                let v95 = if v94 <= v7 { 1.0 } else { 0.0 };
                                let v102 = v2 / v51;
                                v98 = v94;
                                v99 = v93;
                                v100 = v51;
                                v101 = v102;
                            } else {
                                let v96 = v38 * v20;
                                let v97 = v96 + v40;
                                v98 = v96;
                                v99 = v97;
                                v100 = v37;
                                v101 = v7;
                            }
                            v85 = v98;
                            v86 = v82;
                            v87 = v99;
                            v88 = v100;
                            v89 = v83;
                            v90 = v101;
                        }
                        v74 = v85;
                        v75 = v86;
                        v76 = v87;
                        v77 = v88;
                        v78 = v89;
                        v79 = v90;
                    }
                    v65 = v74;
                    v66 = v75;
                    v67 = v76;
                    v68 = v77;
                    v69 = v78;
                    v70 = v79;
                } else {
                    let v106: f64;
                    let v107: f64;
                    let v108: f64;
                    let v109: f64;
                    let v110: f64;
                    let v111: f64;
                    if v64 != 0.0 {
                        let v103 = v38 * v20;
                        let v104 = v103 + v40;
                        v106 = v103;
                        v107 = v7;
                        v108 = v104;
                        v109 = v37;
                        v110 = v7;
                        v111 = v7;
                    } else {
                        let v117: f64;
                        let v118: f64;
                        let v119: f64;
                        let v120: f64;
                        let v121: f64;
                        let v122: f64;
                        if v105 != 0.0 {
                            let v112 = v33 * v20;
                            let v113 = v112 + v35;
                            v117 = v7;
                            v118 = v112;
                            v119 = v7;
                            v120 = v7;
                            v121 = v113;
                            v122 = v37;
                        } else {
                            let v114 = v33 * v20;
                            let v115 = v114 + v35;
                            let v116 = if v115 < v7 { 1.0 } else { 0.0 };
                            let v123 = v38 * v20;
                            let v124 = v123 + v40;
                            let v125 = if v115 > v7 { 1.0 } else { 0.0 };
                            let v127: f64;
                            let v128: f64;
                            if v125 != 0.0 {
                                let v126 = if v124 < v7 { 1.0 } else { 0.0 };
                                let v129 = if v124 > v7 { 1.0 } else { 0.0 };
                                let v133: f64;
                                let v134: f64;
                                if v129 != 0.0 {
                                    let v131 = v50 * (v124 / v115);
                                    let v132 = v2 / v131;
                                    v133 = v131;
                                    v134 = v132;
                                } else {
                                    v133 = v7;
                                    v134 = v37;
                                }
                                v127 = v133;
                                v128 = v134;
                            } else {
                                v127 = v37;
                                v128 = v7;
                            }
                            v117 = v123;
                            v118 = v114;
                            v119 = v124;
                            v120 = v127;
                            v121 = v115;
                            v122 = v128;
                        }
                        v106 = v117;
                        v107 = v118;
                        v108 = v119;
                        v109 = v120;
                        v110 = v121;
                        v111 = v122;
                    }
                    v65 = v106;
                    v66 = v107;
                    v67 = v108;
                    v68 = v109;
                    v69 = v110;
                    v70 = v111;
                }
                v25 = v65;
                v26 = v66;
                v27 = v67;
                v28 = v68;
                v29 = v69;
                v30 = v70;
            }
            let v32 = if v25 < v31 { 1.0 } else { 0.0 };
            let v136 = if v25 > v135 { 1.0 } else { 0.0 };
            let v138 = if v26 < v137 { 1.0 } else { 0.0 };
            let v140 = if v26 > v139 { 1.0 } else { 0.0 };
            let v145: f64;
            if v141 != 0.0 {
                let v143 = v27 + v142;
                v145 = v143;
            } else {
                let v144 = v25 + v142;
                v145 = v144;
            }
            let v147 = if v28 > v7 { 1.0 } else { 0.0 };
            let v150 = if (if (if v145 <= v7 { 1.0 } else { 0.0 }) != 0.0 && v147 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v149 != 0.0 { 1.0 } else { 0.0 };
            let v151 = if v27 > v7 { 1.0 } else { 0.0 };
            let v154: f64;
            let v155: f64;
            if v151 != 0.0 {
                let v166: f64;
                let v167: f64;
                if v21 != 0.0 {
                    let v159 = v152 + (v157 / v27);
                    let v162 = v153 + (v160 / v27);
                    v166 = v159;
                    v167 = v162;
                } else {
                    let v165 = if v163 != 0.0 || v164 != 0.0 { 1.0 } else { 0.0 };
                    let v175: f64;
                    let v176: f64;
                    if v165 != 0.0 {
                        let v171 = v152 + ((v168 * v157) / v27);
                        let v174 = v153 + ((v168 * v160) / v27);
                        v175 = v171;
                        v176 = v174;
                    } else {
                        v175 = v152;
                        v176 = v153;
                    }
                    v166 = v175;
                    v167 = v176;
                }
                v154 = v166;
                v155 = v167;
            } else {
                v154 = v152;
                v155 = v153;
            }
            let v156 = if v29 > v7 { 1.0 } else { 0.0 };
            let v183: f64;
            let v184: f64;
            if v156 != 0.0 {
                let v179 = v154 + (v177 / v29);
                let v182 = v155 + (v180 / v29);
                v183 = v179;
                v184 = v182;
            } else {
                v183 = v154;
                v184 = v155;
            }
            let v187 = if v28 > (v17 / v185) { 1.0 } else { 0.0 };
            let v192 = v2 + (v188 * (v183 + (v188 * v184)));
            let v194 = if v192 < v193 { 1.0 } else { 0.0 };
            let v203: f64;
            if v194 != 0.0 {
                let v202 = v9 + (v200 * (((v196 * (v192 - v9)) - v2).exp()));
                v203 = v202;
            } else {
                v203 = v192;
            }
            let v204 = v28 * v203;
            let v207 = v205 - v206;
            let v212 = (Lanes([v208, 0.0])) - (Lanes([0.0, v210]));
            let v213 = if v147 != 0.0 && v149 != 0.0 { 1.0 } else { 0.0 };
            let v264: f64;
            let v265: Lanes<2>;
            if v213 != 0.0 {
                let v214 = v207 / v145;
                let v215 = v212 / v145;
                let v217 = v216 * v214;
                let v220 = (v215 * v216) * v217;
                let v223 = (v2 + (v217 * v217)).sqrt();
                let v236 = v235 * (v214.abs());
                let v237 = (v215 * ((v224 * (if v214 >= v230 { 1.0 } else { 0.0 })) - v226)) * v235;
                let v238 = v236 * v236;
                let v239 = v237 * v236;
                let v245 = v2 + (v238 * v236);
                let v261 = (((v2 - v252) - v254) + (v252 * v223)) + (v254 * (v245.powf(v246)));
                let v262 = (((v220 + v220) * (v226 / (v224 * v223))) * v252) + (((((v239 + v239) * v236) + (v237 * v238)) * (v246 * (v245.powf(v248)))) * v254);
                v264 = v261;
                v265 = v262;
            } else {
                v264 = v2;
                v265 = v263;
            }
            let v266 = v204 * v264;
            let v268 = v207 / v266;
            let v271 = (v212 - ((v265 * v204) * v268)) / v266;
            if v156 != 0.0 {
                let v275 = if ((v268 / v29).abs()) > v274 { 1.0 } else { 0.0 };
            } else {
            }
            let v278 = if v30 > v7 { 1.0 } else { 0.0 };
            let v279 = if (if v276 != 0.0 && v147 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v278 != 0.0 { 1.0 } else { 0.0 };
            if v279 != 0.0 {
                let v282 = if (if v280 != 0.0 && v151 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v156 != 0.0 { 1.0 } else { 0.0 };
                if v282 != 0.0 {
                } else {
                    let v286 = if (if v25 > v7 { 1.0 } else { 0.0 }) != 0.0 && (if v26 > v7 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                }
                let v287 = if v268 < v7 { 1.0 } else { 0.0 };
            } else {
            }
            let v283 = if v147 != 0.0 && v278 != 0.0 { 1.0 } else { 0.0 };
            if v283 != 0.0 {
                let v289 = if (v271[0]) != v7 { 1.0 } else { 0.0 };
            } else {
            }
            let v290 = v271[0];
            let v291 = v271[1];
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(0),
            Some(1),
            multiplicity * (v268),
            [0, 1],
            [v290, v291],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(1),
            multiplicity * (v292),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(1),
            multiplicity * (v293),
            [],
            [],
            [],
            [],
            multiplicity,
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
    }

}

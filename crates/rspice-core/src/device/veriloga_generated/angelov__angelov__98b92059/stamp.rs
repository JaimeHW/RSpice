#![allow(dead_code, non_snake_case, unused_imports, unused_mut, unused_parens, unused_variables)]

use super::state::{CanonicalModelValues, Instance, PARAMETER_MODEL_FLAGS};
use crate::device::veriloga_generated::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};
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
        let mut key = Vec::with_capacity(164);
        for index in 0..Self::PARAMETER_COUNT {
            if PARAMETER_MODEL_FLAGS[index] {
                key.push(self.params.values[index].to_bits());
                key.push(u64::from(self.param_given[index]));
            }
        }
        key.into_boxed_slice()
    }

    fn canonical_install_model_values(&mut self, values: Arc<CanonicalModelValues>) {
        self.canonical_staged[1] = values[0];
        self.canonical_staged[2] = values[1];
        self.canonical_staged[9] = values[2];
        self.canonical_staged[3] = values[3];
        self.canonical_staged[10] = values[4];
        self.canonical_staged[11] = values[5];
        self.canonical_staged[13] = values[6];
        self.canonical_staged[14] = values[7];
        self.canonical_staged[12] = values[8];
        self.canonical_staged[15] = values[9];
        self.canonical_staged[16] = values[10];
        self.canonical_staged[17] = values[11];
        self.canonical_staged[18] = values[12];
        self.canonical_staged[4] = values[13];
        self.canonical_staged[20] = values[14];
        self.canonical_staged[5] = values[15];
        self.canonical_staged[6] = values[16];
        self.canonical_staged[19] = values[17];
        self.canonical_staged[21] = values[18];
        self.canonical_staged[22] = values[19];
        self.canonical_staged[23] = values[20];
        self.canonical_staged[24] = values[21];
        self.canonical_staged[25] = values[22];
        self.canonical_staged[26] = values[23];
        self.canonical_staged[27] = values[24];
        self.canonical_staged[28] = values[25];
        self.canonical_staged[29] = values[26];
        self.canonical_staged[30] = values[27];
        self.canonical_staged[31] = values[28];
        self.canonical_staged[32] = values[29];
        self.canonical_staged[36] = values[30];
        self.canonical_staged[37] = values[31];
        self.canonical_staged[39] = values[32];
        self.canonical_staged[40] = values[33];
        self.canonical_staged[42] = values[34];
        self.canonical_staged[44] = values[35];
        self.canonical_staged[46] = values[36];
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
                let v0 = if parameter_given[85] { 1.0 } else { 0.0 };
                let v1 = parameters[85];
                let v2 = 2.7315e2f64;
                let v4 = 3.0015e2f64;
                let v6 = parameters[57];
                let v7 = 0e0f64;
                let v9 = if parameter_given[39] { 1.0 } else { 0.0 };
                let v11 = if parameter_given[40] { 1.0 } else { 0.0 };
                let v13 = 5e-1f64;
                let v14 = parameters[40];
                let v16 = parameters[4];
                let v18 = 1e0f64;
                let v22 = 2e0f64;
                let v24 = 3e0f64;
                let v26 = parameters[5];
                let v29 = parameters[6];
                let v32 = parameters[51];
                let v35 = parameters[37];
                let v39 = parameters[53];
                let v41 = 0e0f64;
                let v43 = parameters[55];
                let v45 = 0e0f64;
                let v47 = parameters[47];
                let v49 = 0e0f64;
                let v51 = parameters[45];
                let v53 = 0e0f64;
                let v55 = parameters[42];
                let v57 = parameters[50];
                let v60 = parameters[46];
                let v62 = 0e0f64;
                let v64 = 0e0f64;
                let v66 = parameters[43];
                let v68 = parameters[44];
                let v71 = parameters[48];
                let v74 = parameters[7];
                let v76 = 0e0f64;
                let mut out15: f64 = 0.0;
                let mut out19: f64 = 0.0;
                let mut out23: f64 = 0.0;
                let mut out25: f64 = 0.0;
                let mut out28: f64 = 0.0;
                let mut out31: f64 = 0.0;
                let mut out36: f64 = 0.0;
                let mut out37: f64 = 0.0;
                let mut out38: f64 = 0.0;
                let mut out58: f64 = 0.0;
                let mut out72: f64 = 0.0;
                let mut out78: f64 = 0.0;
                let mut out79: f64 = 0.0;
                let v5: f64;
                if v0 != 0.0 {
                    let v3 = v1 + v2;
                    v5 = v3;
                } else {
                    v5 = v4;
                }
                let v8 = if v6 > v7 { 1.0 } else { 0.0 };
                let v12 = if (if v9 == 0.0 { 1.0 } else { 0.0 }) != 0.0 && v11 != 0.0 { 1.0 } else { 0.0 };
                if v12 != 0.0 {
                    let v15 = v13 / v14;
                    out15 = v15;
                } else {
                }
                let v17 = if v16 == v7 { 1.0 } else { 0.0 };
                if v17 != 0.0 {
                } else {
                    let v19 = if v16 == v18 { 1.0 } else { 0.0 };
                    out19 = v19;
                    if v19 != 0.0 {
                    } else {
                        let v23 = if v16 == v22 { 1.0 } else { 0.0 };
                        out23 = v23;
                        if v23 != 0.0 {
                        } else {
                            let v25 = if v16 == v24 { 1.0 } else { 0.0 };
                            out25 = v25;
                        }
                    }
                }
                let v21 = if v17 != 0.0 || (if v16 == v18 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v27 = if v26 == v7 { 1.0 } else { 0.0 };
                if v27 != 0.0 {
                } else {
                    let v28 = if v26 == v18 { 1.0 } else { 0.0 };
                    out28 = v28;
                }
                let v30 = if v29 == v7 { 1.0 } else { 0.0 };
                if v30 != 0.0 {
                } else {
                    let v31 = if v29 == v18 { 1.0 } else { 0.0 };
                    out31 = v31;
                    if v31 != 0.0 {
                        let v36 = v22 * v35;
                        out36 = v36;
                    } else {
                        let v37 = if v29 == v22 { 1.0 } else { 0.0 };
                        out37 = v37;
                        if v37 != 0.0 {
                            let v38 = v22 * v35;
                            out38 = v38;
                        } else {
                        }
                    }
                }
                let v33 = v32 / v24;
                let v34 = if v29 == v22 { 1.0 } else { 0.0 };
                let v40 = if v39 > v7 { 1.0 } else { 0.0 };
                let v42: f64;
                if v40 != 0.0 {
                    v42 = v7;
                } else {
                    v42 = v41;
                }
                let v44 = if v43 > v7 { 1.0 } else { 0.0 };
                let v46: f64;
                if v44 != 0.0 {
                    v46 = v7;
                } else {
                    v46 = v45;
                }
                let v48 = if v47 > v7 { 1.0 } else { 0.0 };
                let v50: f64;
                if v48 != 0.0 {
                    v50 = v7;
                } else {
                    v50 = v49;
                }
                let v52 = if v51 > v7 { 1.0 } else { 0.0 };
                let v54: f64;
                if v52 != 0.0 {
                    v54 = v7;
                } else {
                    v54 = v53;
                }
                let v56 = if v55 > v7 { 1.0 } else { 0.0 };
                let v59: f64;
                if v56 != 0.0 {
                    v59 = v7;
                } else {
                    let v58 = if v57 > v7 { 1.0 } else { 0.0 };
                    out58 = v58;
                    let v63: f64;
                    if v58 != 0.0 {
                        v63 = v7;
                    } else {
                        v63 = v62;
                    }
                    v59 = v63;
                }
                let v61 = if v60 > v7 { 1.0 } else { 0.0 };
                let v65: f64;
                if v61 != 0.0 {
                    v65 = v7;
                } else {
                    v65 = v64;
                }
                let v70 = if (if v66 > v7 { 1.0 } else { 0.0 }) != 0.0 || (if v68 > v7 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v73: f64;
                if v70 != 0.0 {
                    v73 = v7;
                } else {
                    let v72 = if v71 > v7 { 1.0 } else { 0.0 };
                    out72 = v72;
                    let v77: f64;
                    if v72 != 0.0 {
                        v77 = v7;
                    } else {
                        v77 = v76;
                    }
                    v73 = v77;
                }
                let v75 = if v74 == v7 { 1.0 } else { 0.0 };
                if v75 != 0.0 {
                } else {
                    let v78 = if v74 == v18 { 1.0 } else { 0.0 };
                    out78 = v78;
                    if v78 != 0.0 {
                    } else {
                        let v79 = if v74 == v22 { 1.0 } else { 0.0 };
                        out79 = v79;
                    }
                }
            [v5, v8, v12, out15, v17, out19, out23, out25, v21, v27, out28, v30, out31, out36, out37, out38, v33, v34, v40, v44, v48, v52, v56, out58, v61, v70, out72, v75, out78, out79, v42, v46, v50, v54, v59, v65, v73]
        };
        let values = canonical_model_cache_intern(key, Arc::new(produced));
        self.canonical_install_model_values(values);
    }

    fn canonical_instance_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        if self.canonical_instance_valid {
            return;
        }
        let produced: [f64; 21] = {
            let parameters = &self.params.values;
            let parameter_given = &*self.param_given;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let v0 = if parameter_given[3] { 1.0 } else { 0.0 };
                let v1 = parameters[3];
                let v2 = 2.7315e2f64;
                let v4 = parameters[1];
                let v5 = staged[23];
                let v6 = parameters[0];
                let v7 = 0e0f64;
                let v9 = 0e0f64;
                let v11 = staged[25];
                let v13 = staged[27];
                let v14 = 0e0f64;
                let v17 = staged[28];
                let v18 = 0e0f64;
                let v21 = staged[30];
                let v22 = 0e0f64;
                let v24 = staged[31];
                let v33 = 0e0f64;
                let v34 = 0e0f64;
                let v49 = parameters[72];
                let v50 = parameters[71];
                let v53 = parameters[75];
                let v55 = 0e0f64;
                let v57 = 0e0f64;
                let v58 = 0e0f64;
                let v59 = 0e0f64;
                let v60 = 0e0f64;
                let v61 = 0e0f64;
                let v67 = parameters[57];
                let v69 = 0e0f64;
                let v70 = 0e0f64;
                let v73 = 0e0f64;
                let v74 = 0e0f64;
                let mut out3: f64 = 0.0;
                let mut out52: f64 = 0.0;
                let mut out54: f64 = 0.0;
                let mut out62: f64 = 0.0;
                if v0 != 0.0 {
                    let v3 = v1 + v2;
                    out3 = v3;
                } else {
                }
                let v8: f64;
                if v5 != 0.0 {
                    let v10: f64;
                    if v6 != 0.0 {
                        v10 = v9;
                    } else {
                        v10 = v7;
                    }
                    v8 = v10;
                } else {
                    v8 = v7;
                }
                let v12: f64;
                if v11 != 0.0 {
                    let v15: f64;
                    if v6 != 0.0 {
                        v15 = v14;
                    } else {
                        v15 = v7;
                    }
                    v12 = v15;
                } else {
                    v12 = v7;
                }
                let v16: f64;
                if v13 != 0.0 {
                    let v19: f64;
                    if v6 != 0.0 {
                        v19 = v18;
                    } else {
                        v19 = v7;
                    }
                    v16 = v19;
                } else {
                    v16 = v7;
                }
                let v20: f64;
                if v17 != 0.0 {
                    let v23: f64;
                    if v6 != 0.0 {
                        v23 = v22;
                    } else {
                        v23 = v7;
                    }
                    v20 = v23;
                } else {
                    v20 = v7;
                }
                let v25: f64;
                let v26: f64;
                let v27: f64;
                let v28: f64;
                let v29: f64;
                let v30: f64;
                let v31: f64;
                let v32: f64;
                if v21 != 0.0 {
                    let v35: f64;
                    let v36: f64;
                    if v6 != 0.0 {
                        v35 = v33;
                        v36 = v34;
                    } else {
                        v35 = v7;
                        v36 = v7;
                    }
                    v25 = v35;
                    v26 = v36;
                    v27 = v7;
                    v28 = v7;
                    v29 = v7;
                    v30 = v7;
                    v31 = v7;
                    v32 = v7;
                } else {
                    let v37: f64;
                    let v38: f64;
                    let v39: f64;
                    let v40: f64;
                    let v41: f64;
                    let v42: f64;
                    if v24 != 0.0 {
                        let v43: f64;
                        let v44: f64;
                        let v45: f64;
                        let v46: f64;
                        let v47: f64;
                        let v48: f64;
                        if v6 != 0.0 {
                            let v52 = (v49 * v50).sqrt();
                            out52 = v52;
                            let v54 = if v53 > v7 { 1.0 } else { 0.0 };
                            out54 = v54;
                            let v56: f64;
                            if v54 != 0.0 {
                                v56 = v55;
                            } else {
                                v56 = v7;
                            }
                            v43 = v57;
                            v44 = v58;
                            v45 = v59;
                            v46 = v60;
                            v47 = v61;
                            v48 = v56;
                        } else {
                            v43 = v7;
                            v44 = v7;
                            v45 = v7;
                            v46 = v7;
                            v47 = v7;
                            v48 = v7;
                        }
                        v37 = v43;
                        v38 = v44;
                        v39 = v45;
                        v40 = v46;
                        v41 = v47;
                        v42 = v48;
                    } else {
                        v37 = v7;
                        v38 = v7;
                        v39 = v7;
                        v40 = v7;
                        v41 = v7;
                        v42 = v7;
                    }
                    v25 = v7;
                    v26 = v7;
                    v27 = v37;
                    v28 = v38;
                    v29 = v39;
                    v30 = v40;
                    v31 = v41;
                    v32 = v42;
                }
                let v63: f64;
                let v64: f64;
                let v65: f64;
                let v66: f64;
                if v6 != 0.0 {
                    let v62 = if v53 > v7 { 1.0 } else { 0.0 };
                    out62 = v62;
                    let v71: f64;
                    let v72: f64;
                    if v62 != 0.0 {
                        v71 = v69;
                        v72 = v70;
                    } else {
                        v71 = v7;
                        v72 = v7;
                    }
                    v63 = v73;
                    v64 = v74;
                    v65 = v71;
                    v66 = v72;
                } else {
                    v63 = v7;
                    v64 = v7;
                    v65 = v7;
                    v66 = v7;
                }
                let v68 = if v4 != 0.0 && v67 != 0.0 { 1.0 } else { 0.0 };
            [out3, out52, out54, out62, v68, v8, v12, v16, v20, v25, v26, v27, v28, v29, v30, v31, v32, v63, v64, v65, v66]
        };
        self.canonical_staged[8] = produced[0];
        self.canonical_staged[7] = produced[1];
        self.canonical_staged[33] = produced[2];
        self.canonical_staged[34] = produced[3];
        self.canonical_staged[35] = produced[4];
        self.canonical_staged[38] = produced[5];
        self.canonical_staged[41] = produced[6];
        self.canonical_staged[43] = produced[7];
        self.canonical_staged[45] = produced[8];
        self.canonical_staged[47] = produced[9];
        self.canonical_staged[48] = produced[10];
        self.canonical_staged[49] = produced[11];
        self.canonical_staged[50] = produced[12];
        self.canonical_staged[51] = produced[13];
        self.canonical_staged[52] = produced[14];
        self.canonical_staged[53] = produced[15];
        self.canonical_staged[54] = produced[16];
        self.canonical_staged[55] = produced[17];
        self.canonical_staged[56] = produced[18];
        self.canonical_staged[57] = produced[19];
        self.canonical_staged[58] = produced[20];
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
            let parameters = &self.params.values;
            let parameter_given = &*self.param_given;
            let multiplicity = self.multiplicity;
            let temperature = ctx.temperature();
            let staged = &*self.canonical_staged;
                let v0 = if parameter_given[3] { 1.0 } else { 0.0 };
                let v1 = staged[8];
                let v2 = temperature;
                let v3 = parameters[2];
                let v5: f64;
                if v0 != 0.0 {
                    v5 = v1;
                } else {
                    let v4 = v2 + v3;
                    v5 = v4;
                }
            [v5]
        };
        self.canonical_staged[0] = produced[0];
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
        let staged = &*self.canonical_staged;
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13]), ctx.node_voltage(self.nodes[14]), ctx.node_voltage(self.nodes[15])];
        let branch_unknown_flows = [ctx.branch_current(self.branches[0]), ctx.branch_current(self.branches[1]), ctx.branch_current(self.branches[2]), ctx.branch_current(self.branches[3]), ctx.branch_current(self.branches[4]), ctx.branch_current(self.branches[5]), ctx.branch_current(self.branches[6]), ctx.branch_current(self.branches[7]), ctx.branch_current(self.branches[8]), ctx.branch_current(self.branches[9]), ctx.branch_current(self.branches[10]), ctx.branch_current(self.branches[11]), ctx.branch_current(self.branches[12]), ctx.branch_current(self.branches[13]), ctx.branch_current(self.branches[14]), ctx.branch_current(self.branches[15]), ctx.branch_current(self.branches[16]), ctx.branch_current(self.branches[17]), ctx.branch_current(self.branches[18])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 3101 => 0usize, 3108 => 1usize, 3116 => 2usize, 3118 => 3usize, 3122 => 4usize, 3126 => 5usize, 3130 => 6usize, 3134 => 7usize, 3138 => 8usize, 3149 => 9usize, 3190 => 10usize, 3207 => 11usize, 3229 => 12usize, 3243 => 13usize, 3260 => 14usize, 3399 => 15usize, 3482 => 16usize, _ => usize::MAX };
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
            let v0 = node_potentials[8];
            let v1 = node_potentials[5];
            let v3 = 1e0f64;
            let v5 = 1e0f64;
            let v8 = node_potentials[4];
            let v9 = node_potentials[3];
            let v11 = 1e0f64;
            let v13 = 1e0f64;
            let v17 = -1e0f64;
            let v23 = node_potentials[7];
            let v25 = 1e0f64;
            let v29 = parameters[1];
            let v30 = node_potentials[11];
            let v32 = 0e0f64;
            let v34 = 2e0f64;
            let v36 = 1e0f64;
            let v38 = 1e0f64;
            let v40 = staged[0];
            let v42 = 0e0f64;
            let v45 = 8.617333262e-5f64;
            let v48 = staged[1];
            let v55 = 0e0f64;
            let v57 = staged[2];
            let v59 = parameters[59];
            let v62 = 1e0f64;
            let v64 = parameters[8];
            let v67 = parameters[60];
            let v71 = parameters[11];
            let v74 = parameters[63];
            let v78 = parameters[20];
            let v81 = parameters[61];
            let v85 = parameters[25];
            let v88 = parameters[62];
            let v92 = parameters[28];
            let v95 = parameters[64];
            let v99 = parameters[53];
            let v102 = parameters[65];
            let v106 = parameters[54];
            let v109 = parameters[68];
            let v112 = parameters[9];
            let v114 = parameters[30];
            let v118 = parameters[29];
            let v120 = parameters[36];
            let v124 = parameters[35];
            let v126 = parameters[69];
            let v129 = parameters[41];
            let v131 = parameters[70];
            let v134 = parameters[21];
            let v160 = staged[9];
            let v161 = staged[3];
            let v166 = parameters[39];
            let v169 = parameters[19];
            let v178 = parameters[18];
            let v190 = parameters[10];
            let v192 = parameters[15];
            let v206 = parameters[22];
            let v234 = parameters[12];
            let v239 = parameters[13];
            let v261 = 5e-1f64;
            let v271 = parameters[14];
            let v282 = staged[10];
            let v292 = parameters[16];
            let v312 = staged[11];
            let v317 = staged[12];
            let v349 = parameters[17];
            let v426 = staged[13];
            let v505 = staged[14];
            let v669 = Lanes([0e0f64; 5]);
            let v680 = parameters[52];
            let v682 = parameters[44];
            let v685 = parameters[43];
            let v687 = parameters[46];
            let v707 = parameters[66];
            let v725 = staged[15];
            let v726 = -1e0f64;
            let v755 = staged[16];
            let v772 = parameters[38];
            let v793 = parameters[37];
            let v805 = parameters[32];
            let v808 = parameters[31];
            let v815 = parameters[34];
            let v818 = parameters[33];
            let v842 = staged[17];
            let v871 = parameters[26];
            let v872 = parameters[24];
            let v873 = Lanes([0e0f64; 4]);
            let v874 = Lanes([0e0f64; 4]);
            let v875 = staged[18];
            let v886 = parameters[51];
            let v887 = node_potentials[12];
            let v889 = 1e0f64;
            let v892 = ddt_scale();
            let v894 = staged[6];
            let v895 = branch_unknown_flows[0];
            let v897 = 1e0f64;
            let v901 = staged[19];
            let v918 = staged[4];
            let v926 = staged[20];
            let v966 = staged[5];
            let v1068 = node_potentials[1];
            let v1070 = 1e0f64;
            let v1074 = parameters[27];
            let v1079 = parameters[23];
            let v1084 = node_potentials[10];
            let v1087 = 1e0f64;
            let v1098 = staged[21];
            let v1109 = Lanes([0e0f64; 6]);
            let v1112 = node_potentials[9];
            let v1114 = 1e0f64;
            let v1118 = parameters[56];
            let v1123 = staged[22];
            let v1128 = parameters[55];
            let v1131 = Lanes([0e0f64; 2]);
            let v1134 = staged[23];
            let v1139 = parameters[47];
            let v1142 = parameters[0];
            let v1143 = Lanes([0e0f64; 2]);
            let v1146 = staged[24];
            let v1151 = parameters[45];
            let v1154 = Lanes([0e0f64; 2]);
            let v1157 = staged[25];
            let v1158 = branch_unknown_flows[5];
            let v1159 = parameters[42];
            let v1161 = 1e0f64;
            let v1163 = parameters[50];
            let v1168 = staged[26];
            let v1179 = staged[27];
            let v1180 = 0e0f64;
            let v1189 = branch_unknown_flows[10];
            let v1191 = 1e0f64;
            let v1197 = Lanes([0e0f64; 6]);
            let v1200 = parameters[49];
            let v1201 = branch_unknown_flows[13];
            let v1203 = 1e0f64;
            let v1207 = staged[28];
            let v1208 = branch_unknown_flows[14];
            let v1210 = 1e0f64;
            let v1216 = parameters[48];
            let v1221 = staged[29];
            let v1232 = staged[30];
            let v1233 = 0e0f64;
            let v1242 = Lanes([0e0f64; 6]);
            let v1243 = staged[31];
            let v1256 = 0e0f64;
            let v1257 = 0e0f64;
            let v1258 = Lanes([0e0f64; 3]);
            let v1259 = Lanes([0e0f64; 2]);
            let v1286 = 5.5226012e-23f64;
            let v1289 = parameters[73];
            let v1296 = staged[7];
            let v1310 = 3.141592653589793e0f64;
            let v1315 = node_potentials[14];
            let v1318 = 1e0f64;
            let v1323 = node_potentials[15];
            let v1326 = 1e0f64;
            let v1345 = staged[35];
            let v1346 = parameters[58];
            let v1351 = node_potentials[13];
            let v1353 = 1e0f64;
            let v1375 = -1e0f64;
            let v1378 = parameters[57];
            let v1381 = 1e-12f64;
            let v1384 = Lanes([0e0f64; 5]);
            let v2 = v0 - v1;
            let v7 = (Lanes([0.0, v3])) - (Lanes([v5, 0.0]));
            let v10 = v8 - v9;
            let v15 = (Lanes([0.0, v11])) - (Lanes([v13, 0.0]));
            let v16 = -v10;
            let v18 = v15 * v17;
            let v19 = v9 - v1;
            let v22 = (Lanes([v13, 0.0])) - (Lanes([0.0, v5]));
            let v24 = v23 - v9;
            let v28 = (Lanes([0.0, v25])) - (Lanes([v13, 0.0]));
            let v43: f64;
            let v44: f64;
            if v29 != 0.0 {
                let v39 = v38 * ((v34 * (if v30 >= v32 { 1.0 } else { 0.0 })) - v36);
                let v41 = v40 + (v30.abs());
                v43 = v41;
                v44 = v39;
            } else {
                v43 = v40;
                v44 = v42;
            }
            let v46 = v43 * v45;
            let v47 = v44 * v45;
            let v49 = v43 - v48;
            let v50 = v49.abs();
            let v54 = v44 * ((v34 * (if v49 >= v32 { 1.0 } else { 0.0 })) - v36);
            let v58 = if (if v50 > v55 { 1.0 } else { 0.0 }) != 0.0 || v57 != 0.0 { 1.0 } else { 0.0 };
            let v136: f64;
            let v137: f64;
            let v138: f64;
            let v139: f64;
            let v140: f64;
            let v141: f64;
            let v142: f64;
            let v143: f64;
            let v144: f64;
            let v145: f64;
            let v146: f64;
            let v147: f64;
            let v148: f64;
            let v149: f64;
            let v150: f64;
            let v151: f64;
            let v152: f64;
            let v153: f64;
            let v154: f64;
            let v155: f64;
            let v156: f64;
            let v157: f64;
            let v158: f64;
            let v159: f64;
            if v58 != 0.0 {
                let v65 = v64 * (v62 + (v59 * v50));
                let v66 = (v54 * v59) * v64;
                let v72 = v71 * (v62 + (v67 * v50));
                let v73 = (v54 * v67) * v71;
                let v79 = v78 * (v62 + (v74 * v50));
                let v80 = (v54 * v74) * v78;
                let v86 = v85 * (v62 + (v81 * v50));
                let v87 = (v54 * v81) * v85;
                let v93 = v92 * (v62 + (v88 * v50));
                let v94 = (v54 * v88) * v92;
                let v100 = v99 * (v62 + (v95 * v50));
                let v101 = (v54 * v95) * v99;
                let v107 = v106 * (v62 + (v102 * v50));
                let v108 = (v54 * v102) * v106;
                let v111 = v54 * v109;
                let v113 = v112 + (v109 * v50);
                let v115 = v114 * v109;
                let v117 = v54 * v115;
                let v119 = v118 + (v115 * v50);
                let v121 = v120 * v109;
                let v123 = v54 * v121;
                let v125 = v124 + (v121 * v50);
                let v128 = v54 * v126;
                let v130 = v129 + (v126 * v50);
                let v133 = v54 * v131;
                let v135 = v134 + (v131 * v50);
                v136 = v72;
                v137 = v113;
                v138 = v135;
                v139 = v65;
                v140 = v79;
                v141 = v100;
                v142 = v130;
                v143 = v119;
                v144 = v125;
                v145 = v86;
                v146 = v93;
                v147 = v107;
                v148 = v73;
                v149 = v111;
                v150 = v133;
                v151 = v66;
                v152 = v80;
                v153 = v101;
                v154 = v128;
                v155 = v117;
                v156 = v123;
                v157 = v87;
                v158 = v94;
                v159 = v108;
            } else {
                v136 = v71;
                v137 = v112;
                v138 = v134;
                v139 = v64;
                v140 = v78;
                v141 = v99;
                v142 = v129;
                v143 = v118;
                v144 = v124;
                v145 = v85;
                v146 = v92;
                v147 = v106;
                v148 = v42;
                v149 = v42;
                v150 = v42;
                v151 = v42;
                v152 = v42;
                v153 = v42;
                v154 = v42;
                v155 = v42;
                v156 = v42;
                v157 = v42;
                v158 = v42;
                v159 = v42;
            }
            let v167: f64;
            let v168: f64;
            if v160 != 0.0 {
                let v162 = v161 / v46;
                let v165 = ((v47 * v162) * v17) / v46;
                v167 = v162;
                v168 = v165;
            } else {
                v167 = v166;
                v168 = v42;
            }
            let v170 = v169 * v19;
            let v172 = v170.cosh();
            let v175 = v172 * v172;
            let v176 = ((v22 * v169) * (v170.sinh())) * v172;
            let v179 = v178 / v175;
            let v183 = v62 + v179;
            let v184 = v136 * v183;
            let v186 = ((((v176 + v176) * v179) * v17) / v175) * v136;
            let v189 = (Lanes([0.0, 0.0, (v148 * v183)])) + (Lanes([v186[0], v186[1], 0.0]));
            let v195 = (v192 * v19).tanh();
            let v200 = ((v22 * v192) * (v36 - (v195 * v195))) * v190;
            let v204 = (Lanes([0.0, 0.0, v149])) + (Lanes([v200[0], v200[1], 0.0]));
            let v207 = v206 * (v16 - v134);
            let v209 = v16 - v138;
            let v212 = (Lanes([v18[0], v18[1], 0.0])) - (Lanes([0.0, 0.0, v150]));
            let v214 = (v18 * v206) * v209;
            let v217 = (Lanes([v214[0], v214[1], 0.0])) + (v212 * v207);
            let v218 = ((v137 - v190) + (v190 * v195)) - (v207 * v209);
            let v221 = (Lanes([v204[0], 0.0, v204[1], v204[2]])) - (Lanes([v217[0], v217[1], 0.0, v217[2]]));
            let v222 = v2 - v218;
            let v225 = (Lanes([0.0, 0.0, v7[0], v7[1], 0.0])) - (Lanes([v221[0], v221[1], v221[2], 0.0, v221[3]]));
            let v226 = v222 * v222;
            let v227 = v225 * v222;
            let v228 = v227 + v227;
            let v230 = v189 * v222;
            let v235 = v234 * v226;
            let v236 = v228 * v234;
            let v240 = v239 * v222;
            let v246 = ((v184 * v222) + v235) + (v240 * v226);
            let v247 = (((Lanes([v230[0], 0.0, v230[1], 0.0, v230[2]])) + (v225 * v184)) + v236) + (((v225 * v239) * v226) + (v228 * v240));
            let v248 = v246.tanh();
            let v251 = v247 * (v36 - (v248 * v248));
            let v252 = v62 + v248;
            let v253 = rspice_limexp(v246);
            let v257 = rspice_limexp((-v246));
            let v264 = (v261 * (v253 - v257)).tanh();
            let v267 = (((v247 * v253) - ((v247 * v17) * v257)) * v261) * (v36 - (v264 * v264));
            let v268 = v62 + v264;
            let v272 = v271 + (v192 * v252);
            let v275 = v22 * v272;
            let v278 = (v272 * v19).tanh();
            let v281 = (((v251 * v192) * v19) + (Lanes([v275[0], 0.0, v275[1], 0.0, 0.0]))) * (v36 - (v278 * v278));
            let v313: f64;
            let v314: f64;
            let v315: Lanes<5>;
            let v316: Lanes<5>;
            if v282 != 0.0 {
                let v283 = v139 * v252;
                let v288 = v283 * v278;
                let v294 = v22 * v292;
                let v296 = rspice_limexp(v209);
                let v302 = (Lanes([0.0, 0.0, (v152 * v296)])) + ((v212 * v296) * v140);
                let v303 = (v62 + (v292 * v19)) + (v140 * v296);
                let v307 = v288 * v303;
                let v309 = ((Lanes([v294[0], 0.0, v294[1], 0.0])) + (Lanes([v302[0], v302[1], 0.0, v302[2]]))) * v288;
                let v311 = (((((Lanes([0.0, 0.0, 0.0, 0.0, (v151 * v252)])) + (v251 * v139)) * v278) + (v281 * v283)) * v303) + (Lanes([v309[0], v309[1], v309[2], 0.0, v309[3]]));
                v313 = v268;
                v314 = v307;
                v315 = v267;
                v316 = v311;
            } else {
                let v427: f64;
                let v428: f64;
                let v429: Lanes<5>;
                let v430: Lanes<5>;
                if v312 != 0.0 {
                    let v318 = v10 - v218;
                    let v320 = (Lanes([v15[0], v15[1], 0.0, 0.0])) - v221;
                    let v321 = v318 * v318;
                    let v322 = v320 * v318;
                    let v323 = v322 + v322;
                    let v329 = v189 * v318;
                    let v341 = (((v184 * v318) + (v234 * v321)) + (v239 * (v321 * v318))).tanh();
                    let v344 = ((((Lanes([v329[0], 0.0, v329[1], v329[2]])) + (v320 * v184)) + (v323 * v234)) + (((v323 * v318) + (v320 * v321)) * v239)) * (v36 - (v341 * v341));
                    let v345 = v62 + v341;
                    let v348 = v271 + (v192 * v345);
                    let v352 = v292 + (v349 * v252);
                    let v353 = v139 * v252;
                    let v358 = v62 + v278;
                    let v359 = v353 * v358;
                    let v365 = v22 * v352;
                    let v373 = rspice_limexp((v19 - v138));
                    let v379 = (Lanes([0.0, 0.0, (v152 * v373)])) + ((((Lanes([v22[0], v22[1], 0.0])) - (Lanes([0.0, 0.0, v150]))) * v373) * v140);
                    let v380 = (v62 + (v352 * v19)) + (v140 * v373);
                    let v389 = v292 + (v349 * v345);
                    let v392 = v22 * v348;
                    let v395 = (v348 * v19).tanh();
                    let v399 = v139 * v345;
                    let v404 = v62 - v395;
                    let v406 = v399 * v404;
                    let v412 = v22 * v389;
                    let v415 = v62 - (v389 * v19);
                    let v420 = (((((Lanes([0.0, 0.0, 0.0, (v151 * v345)])) + (v344 * v139)) * v404) + ((((((v344 * v192) * v19) + (Lanes([v392[0], 0.0, v392[1], 0.0]))) * (v36 - (v395 * v395))) * v17) * v399)) * v415) + (((((v344 * v349) * v19) + (Lanes([v412[0], 0.0, v412[1], 0.0]))) * v17) * v406);
                    let v424 = v261 * ((v359 * v380) - (v406 * v415));
                    let v425 = (((((((Lanes([0.0, 0.0, 0.0, 0.0, (v151 * v252)])) + (v251 * v139)) * v358) + (v281 * v353)) * v380) + (((((v251 * v349) * v19) + (Lanes([v365[0], 0.0, v365[1], 0.0, 0.0]))) + (Lanes([v379[0], 0.0, v379[1], 0.0, v379[2]]))) * v359)) - (Lanes([v420[0], v420[1], v420[2], 0.0, v420[3]]))) * v261;
                    v427 = v268;
                    v428 = v424;
                    v429 = v267;
                    v430 = v425;
                } else {
                    let v506: f64;
                    let v507: f64;
                    let v508: Lanes<5>;
                    let v509: Lanes<5>;
                    if v426 != 0.0 {
                        let v433 = v239 * v226;
                        let v439 = (v222 + v235) + (v433 * v222);
                        let v441 = v184 * v439;
                        let v442 = v189 * v439;
                        let v445 = (Lanes([v442[0], 0.0, v442[1], 0.0, v442[2]])) + (((v225 + v236) + (((v228 * v239) * v222) + (v225 * v433))) * v184);
                        let v446 = rspice_limexp(v441);
                        let v450 = rspice_limexp((-v441));
                        let v456 = (v261 * (v446 - v450)).tanh();
                        let v459 = (((v445 * v446) - ((v445 * v17) * v450)) * v261) * (v36 - (v456 * v456));
                        let v460 = v62 + v456;
                        let v463 = v271 + (v192 * v460);
                        let v466 = v22 * v463;
                        let v469 = (v463 * v19).tanh();
                        let v475 = v292 + (v349 * v460);
                        let v476 = v139 * v460;
                        let v481 = v476 * v469;
                        let v487 = v22 * v475;
                        let v491 = rspice_limexp(v209);
                        let v497 = (Lanes([0.0, 0.0, (v152 * v491)])) + ((v212 * v491) * v140);
                        let v498 = (v62 + (v475 * v19)) + (v140 * v491);
                        let v501 = v481 * v498;
                        let v504 = (((((Lanes([0.0, 0.0, 0.0, 0.0, (v151 * v460)])) + (v459 * v139)) * v469) + (((((v459 * v192) * v19) + (Lanes([v466[0], 0.0, v466[1], 0.0, 0.0]))) * (v36 - (v469 * v469))) * v476)) * v498) + (((((v459 * v349) * v19) + (Lanes([v487[0], 0.0, v487[1], 0.0, 0.0]))) + (Lanes([v497[0], v497[1], 0.0, 0.0, v497[2]]))) * v481);
                        v506 = v460;
                        v507 = v501;
                        v508 = v459;
                        v509 = v504;
                    } else {
                        let v670: f64;
                        let v671: f64;
                        let v672: Lanes<5>;
                        let v673: Lanes<5>;
                        if v505 != 0.0 {
                            let v512 = v239 * v226;
                            let v518 = (v222 + v235) + (v512 * v222);
                            let v520 = v184 * v518;
                            let v521 = v189 * v518;
                            let v524 = (Lanes([v521[0], 0.0, v521[1], 0.0, v521[2]])) + (((v225 + v236) + (((v228 * v239) * v222) + (v225 * v512))) * v184);
                            let v525 = v10 - v218;
                            let v527 = (Lanes([v15[0], v15[1], 0.0, 0.0])) - v221;
                            let v528 = v525 * v525;
                            let v529 = v527 * v525;
                            let v530 = v529 + v529;
                            let v535 = v239 * v525;
                            let v541 = (v525 + (v234 * v528)) + (v535 * v528);
                            let v543 = v184 * v541;
                            let v544 = v189 * v541;
                            let v547 = (Lanes([v544[0], 0.0, v544[1], v544[2]])) + (((v527 + (v530 * v234)) + (((v527 * v239) * v528) + (v530 * v535))) * v184);
                            let v548 = rspice_limexp(v520);
                            let v552 = rspice_limexp((-v520));
                            let v558 = (v261 * (v548 - v552)).tanh();
                            let v561 = (((v524 * v548) - ((v524 * v17) * v552)) * v261) * (v36 - (v558 * v558));
                            let v562 = v62 + v558;
                            let v563 = rspice_limexp(v543);
                            let v567 = rspice_limexp((-v543));
                            let v573 = (v261 * (v563 - v567)).tanh();
                            let v576 = (((v547 * v563) - ((v547 * v17) * v567)) * v261) * (v36 - (v573 * v573));
                            let v577 = v62 + v573;
                            let v580 = v271 + (v192 * v562);
                            let v583 = v271 + (v192 * v577);
                            let v586 = v22 * v580;
                            let v589 = (v580 * v19).tanh();
                            let v595 = v22 * v583;
                            let v598 = (v583 * v19).tanh();
                            let v604 = v292 + (v349 * v577);
                            let v607 = v292 + (v349 * v562);
                            let v608 = v139 * v562;
                            let v613 = v62 + v589;
                            let v614 = v608 * v613;
                            let v620 = v22 * v607;
                            let v628 = rspice_limexp((v19 - v138));
                            let v634 = (Lanes([0.0, 0.0, (v152 * v628)])) + ((((Lanes([v22[0], v22[1], 0.0])) - (Lanes([0.0, 0.0, v150]))) * v628) * v140);
                            let v635 = (v62 + (v607 * v19)) + (v140 * v628);
                            let v642 = v139 * v577;
                            let v647 = v62 - v598;
                            let v649 = v642 * v647;
                            let v655 = v22 * v604;
                            let v658 = v62 - (v604 * v19);
                            let v663 = (((((Lanes([0.0, 0.0, 0.0, (v151 * v577)])) + (v576 * v139)) * v647) + ((((((v576 * v192) * v19) + (Lanes([v595[0], 0.0, v595[1], 0.0]))) * (v36 - (v598 * v598))) * v17) * v642)) * v658) + (((((v576 * v349) * v19) + (Lanes([v655[0], 0.0, v655[1], 0.0]))) * v17) * v649);
                            let v667 = v261 * ((v614 * v635) - (v649 * v658));
                            let v668 = (((((((Lanes([0.0, 0.0, 0.0, 0.0, (v151 * v562)])) + (v561 * v139)) * v613) + (((((v561 * v192) * v19) + (Lanes([v586[0], 0.0, v586[1], 0.0, 0.0]))) * (v36 - (v589 * v589))) * v608)) * v635) + (((((v561 * v349) * v19) + (Lanes([v620[0], 0.0, v620[1], 0.0, 0.0]))) + (Lanes([v634[0], 0.0, v634[1], 0.0, v634[2]]))) * v614)) - (Lanes([v663[0], v663[1], v663[2], 0.0, v663[3]]))) * v261;
                            v670 = v562;
                            v671 = v667;
                            v672 = v561;
                            v673 = v668;
                        } else {
                            v670 = v268;
                            v671 = v55;
                            v672 = v267;
                            v673 = v669;
                        }
                        v506 = v670;
                        v507 = v671;
                        v508 = v672;
                        v509 = v673;
                    }
                    v427 = v506;
                    v428 = v507;
                    v429 = v508;
                    v430 = v509;
                }
                v313 = v427;
                v314 = v428;
                v315 = v429;
                v316 = v430;
            }
            let v700: f64;
            let v701: f64;
            let v702: f64;
            let v703: Lanes<5>;
            let v704: Lanes<5>;
            let v705: Lanes<5>;
            if v317 != 0.0 {
                let v674 = v62 + v252;
                let v675 = v141 / v674;
                let v679 = ((Lanes([0.0, 0.0, 0.0, 0.0, v153])) - (v251 * v675)) / v674;
                let v681 = v680 + v675;
                let v683 = v682 * v252;
                let v684 = v251 * v682;
                let v686 = v685 + v683;
                let v688 = v687 + v683;
                v700 = v688;
                v701 = v686;
                v702 = v681;
                v703 = v684;
                v704 = v684;
                v705 = v679;
            } else {
                let v689 = v62 + v313;
                let v690 = v141 / v689;
                let v694 = ((Lanes([0.0, 0.0, 0.0, 0.0, v153])) - (v315 * v690)) / v689;
                let v695 = v680 + v690;
                let v696 = v682 * v313;
                let v697 = v315 * v682;
                let v698 = v685 + v696;
                let v699 = v687 + v696;
                v700 = v699;
                v701 = v698;
                v702 = v695;
                v703 = v697;
                v704 = v697;
                v705 = v694;
            }
            let v706 = if v50 != 0.0 || v57 != 0.0 { 1.0 } else { 0.0 };
            let v721: f64;
            let v722: f64;
            let v723: Lanes<5>;
            let v724: Lanes<5>;
            if v706 != 0.0 {
                let v709 = v54 * v707;
                let v710 = v62 + (v707 * v50);
                let v711 = v700 * v710;
                let v715 = (v703 * v710) + (Lanes([0.0, 0.0, 0.0, 0.0, (v709 * v700)]));
                let v716 = v701 * v710;
                let v720 = (v704 * v710) + (Lanes([0.0, 0.0, 0.0, 0.0, (v709 * v701)]));
                v721 = v711;
                v722 = v716;
                v723 = v715;
                v724 = v720;
            } else {
                v721 = v700;
                v722 = v701;
                v723 = v703;
                v724 = v704;
            }
            let v756: f64;
            let v757: f64;
            let v758: f64;
            let v759: Lanes<3>;
            let v760: f64;
            let v761: Lanes<3>;
            if v725 != 0.0 {
                let v729 = (v726 * v142).tanh();
                let v737 = rspice_limexp((v167 * v729));
                let v738 = ((v168 * v729) + (((v154 * v726) * (v36 - (v729 * v729))) * v167)) * v737;
                let v739 = v2 - v142;
                let v742 = (Lanes([v7[0], v7[1], 0.0])) - (Lanes([0.0, 0.0, v154]));
                let v743 = v24 - v142;
                let v746 = (Lanes([v28[0], v28[1], 0.0])) - (Lanes([0.0, 0.0, v154]));
                v756 = v739;
                v757 = v737;
                v758 = v743;
                v759 = v742;
                v760 = v738;
                v761 = v746;
            } else {
                let v747 = -v167;
                let v753 = rspice_limexp((v747 * v142));
                let v754 = (((v168 * v17) * v142) + (v154 * v747)) * v753;
                let v867: f64;
                let v868: f64;
                let v869: Lanes<3>;
                let v870: Lanes<3>;
                if v755 != 0.0 {
                    let v847 = (v2 - v142).tanh();
                    let v850 = ((Lanes([v7[0], v7[1], 0.0])) - (Lanes([0.0, 0.0, v154]))) * (v36 - (v847 * v847));
                    let v855 = (v24 - v142).tanh();
                    let v858 = ((Lanes([v28[0], v28[1], 0.0])) - (Lanes([0.0, 0.0, v154]))) * (v36 - (v855 * v855));
                    v867 = v847;
                    v868 = v855;
                    v869 = v850;
                    v870 = v858;
                } else {
                    let v859 = v2 - v142;
                    let v862 = (Lanes([v7[0], v7[1], 0.0])) - (Lanes([0.0, 0.0, v154]));
                    let v863 = v24 - v142;
                    let v866 = (Lanes([v28[0], v28[1], 0.0])) - (Lanes([0.0, 0.0, v154]));
                    v867 = v859;
                    v868 = v863;
                    v869 = v862;
                    v870 = v866;
                }
                v756 = v867;
                v757 = v753;
                v758 = v868;
                v759 = v869;
                v760 = v754;
                v761 = v870;
            }
            let v767 = rspice_limexp((v167 * v756));
            let v773 = v772 * (v767 - v757);
            let v774 = ((((Lanes([0.0, 0.0, (v168 * v756)])) + (v759 * v167)) * v767) - (Lanes([0.0, 0.0, v760]))) * v772;
            let v780 = rspice_limexp((v167 * v758));
            let v785 = v772 * (v780 - v757);
            let v786 = ((((Lanes([0.0, 0.0, (v168 * v758)])) + (v761 * v167)) * v780) - (Lanes([0.0, 0.0, v760]))) * v772;
            let v788 = v7 * v114;
            let v792 = (Lanes([0.0, 0.0, v155])) + (Lanes([v788[0], v788[1], 0.0]));
            let v794 = v793 * v19;
            let v795 = v22 * v793;
            let v796 = (v143 + (v114 * v2)) + v794;
            let v799 = (Lanes([0.0, v792[0], v792[1], v792[2]])) + (Lanes([v795[0], v795[1], 0.0, 0.0]));
            let v800 = v796.tanh();
            let v803 = v799 * (v36 - (v800 * v800));
            let v804 = v62 + v800;
            let v810 = (v808 + (v805 * v19)).tanh();
            let v813 = (v22 * v805) * (v36 - (v810 * v810));
            let v814 = v62 + v810;
            let v821 = (v818 - (v815 * v19)).tanh();
            let v824 = ((v22 * v815) * v17) * (v36 - (v821 * v821));
            let v826 = (v62 + v821) - v793;
            let v828 = v28 * v120;
            let v832 = (Lanes([0.0, 0.0, v156])) + (Lanes([v828[0], v828[1], 0.0]));
            let v833 = (v144 + (v120 * v24)) - v794;
            let v836 = (Lanes([v832[0], 0.0, v832[1], v832[2]])) - (Lanes([v795[0], v795[1], 0.0, 0.0]));
            let v837 = v833.tanh();
            let v840 = v836 * (v36 - (v837 * v837));
            let v841 = v62 + v837;
            let v876: f64;
            let v877: f64;
            let v878: f64;
            let v879: f64;
            let v880: Lanes<4>;
            let v881: Lanes<4>;
            let v882: Lanes<4>;
            let v883: Lanes<4>;
            if v842 != 0.0 {
                v876 = v55;
                v877 = v55;
                v878 = v871;
                v879 = v872;
                v880 = v873;
                v881 = v874;
                v882 = v873;
                v883 = v874;
            } else {
                let v927: f64;
                let v928: f64;
                let v929: f64;
                let v930: f64;
                let v931: Lanes<4>;
                let v932: Lanes<4>;
                let v933: Lanes<4>;
                let v934: Lanes<4>;
                if v875 != 0.0 {
                    let v902 = v145 * v804;
                    let v909 = v813 * v902;
                    let v911 = (((Lanes([0.0, 0.0, 0.0, (v157 * v804)])) + (v803 * v145)) * v814) + (Lanes([v909[0], v909[1], 0.0, 0.0]));
                    let v912 = v872 + (v902 * v814);
                    let v914 = v824 * v841;
                    let v919 = (v826 * v841) + v918;
                    let v924 = (Lanes([0.0, 0.0, 0.0, (v158 * v919)])) + (((Lanes([v914[0], v914[1], 0.0, 0.0])) + (v840 * v826)) * v146);
                    let v925 = v871 + (v146 * v919);
                    v927 = v55;
                    v928 = v55;
                    v929 = v925;
                    v930 = v912;
                    v931 = v873;
                    v932 = v874;
                    v933 = v924;
                    v934 = v911;
                } else {
                    let v1028: f64;
                    let v1029: f64;
                    let v1030: f64;
                    let v1031: f64;
                    let v1032: Lanes<4>;
                    let v1033: Lanes<4>;
                    if v926 != 0.0 {
                        let v935 = v814 - v793;
                        let v936 = v143 + v794;
                        let v938 = Lanes([v795[0], v795[1], 0.0]);
                        let v939 = (Lanes([0.0, 0.0, v155])) + v938;
                        let v940 = v936.cosh();
                        let v946 = v796.cosh();
                        let v953 = v939 + ((v939 * (v936.sinh())) * (v36 / v940));
                        let v956 = (v796 + (v946.ln())) - (v936 + (v940.ln()));
                        let v961 = v813 * v956;
                        let v968 = v7 * v966;
                        let v969 = ((v956 * v935) / v114) + (v966 * v2);
                        let v978 = v7 * v872;
                        let v979 = (v145 * v969) + (v872 * v2);
                        let v981 = ((Lanes([0.0, 0.0, 0.0, (v157 * v969)])) + (((((((v799 + ((v799 * (v796.sinh())) * (v36 / v946))) - (Lanes([v953[0], v953[1], 0.0, v953[2]]))) * v935) + (Lanes([v961[0], v961[1], 0.0, 0.0]))) / v114) + (Lanes([0.0, v968[0], v968[1], 0.0]))) * v145)) + (Lanes([0.0, v978[0], v978[1], 0.0]));
                        let v982 = v144 - v794;
                        let v984 = (Lanes([0.0, 0.0, v156])) - v938;
                        let v985 = v982.cosh();
                        let v991 = v833.cosh();
                        let v998 = v984 + ((v984 * (v982.sinh())) * (v36 / v985));
                        let v1001 = (v833 + (v991.ln())) - (v982 + (v985.ln()));
                        let v1006 = v824 * v1001;
                        let v1012 = v28 * v966;
                        let v1013 = ((v1001 * v826) / v120) + (v966 * v24);
                        let v1022 = v28 * v871;
                        let v1023 = (v146 * v1013) + (v871 * v24);
                        let v1025 = ((Lanes([0.0, 0.0, 0.0, (v158 * v1013)])) + (((((((v836 + ((v836 * (v833.sinh())) * (v36 / v991))) - (Lanes([v998[0], v998[1], 0.0, v998[2]]))) * v826) + (Lanes([v1006[0], v1006[1], 0.0, 0.0]))) / v120) + (Lanes([v1012[0], 0.0, v1012[1], 0.0]))) * v146)) + (Lanes([v1022[0], 0.0, v1022[1], 0.0]));
                        let v1026 = v981[2];
                        let v1027 = v1025[2];
                        v1028 = v1023;
                        v1029 = v979;
                        v1030 = v1027;
                        v1031 = v1026;
                        v1032 = v1025;
                        v1033 = v981;
                    } else {
                        v1028 = v55;
                        v1029 = v55;
                        v1030 = v55;
                        v1031 = v55;
                        v1032 = v873;
                        v1033 = v874;
                    }
                    v927 = v1028;
                    v928 = v1029;
                    v929 = v1030;
                    v930 = v1031;
                    v931 = v1032;
                    v932 = v1033;
                    v933 = v873;
                    v934 = v874;
                }
                v876 = v927;
                v877 = v928;
                v878 = v929;
                v879 = v930;
                v880 = v931;
                v881 = v932;
                v882 = v933;
                v883 = v934;
            }
            let v884 = -v314;
            let v885 = v316 * v17;
            let v888 = v886 * v887;
            let v890 = v889 * v886;
            let v891 = ddt(3101, v888);
            let v893 = v890 * v892;
            let v896 = v894 * v895;
            let v898 = v897 * v894;
            let v899 = ddt(3108, v896);
            let v900 = v898 * v892;
            let v1052: f64;
            let v1053: f64;
            let v1054: f64;
            let v1055: f64;
            let v1056: f64;
            let v1057: f64;
            let v1058: f64;
            let v1059: f64;
            let v1060: Lanes<4>;
            let v1061: Lanes<4>;
            let v1062: Lanes<4>;
            let v1063: Lanes<4>;
            let v1064: Lanes<4>;
            let v1065: Lanes<4>;
            let v1066: Lanes<4>;
            let v1067: Lanes<4>;
            if v901 != 0.0 {
                let v1034 = ddt(3116, v876);
                let v1035 = v880 * v892;
                let v1036 = ddt(3118, v877);
                let v1037 = v881 * v892;
                v1052 = v1034;
                v1053 = v1036;
                v1054 = v55;
                v1055 = v55;
                v1056 = v876;
                v1057 = v877;
                v1058 = v55;
                v1059 = v55;
                v1060 = v1035;
                v1061 = v1037;
                v1062 = v873;
                v1063 = v874;
                v1064 = v880;
                v1065 = v881;
                v1066 = v873;
                v1067 = v874;
            } else {
                let v1038 = v878 * v24;
                let v1040 = v28 * v878;
                let v1042 = (v882 * v24) + (Lanes([v1040[0], 0.0, v1040[1], 0.0]));
                let v1043 = ddt(3122, v1038);
                let v1044 = v1042 * v892;
                let v1045 = v879 * v2;
                let v1047 = v7 * v879;
                let v1049 = (v883 * v2) + (Lanes([0.0, v1047[0], v1047[1], 0.0]));
                let v1050 = ddt(3126, v1045);
                let v1051 = v1049 * v892;
                v1052 = v55;
                v1053 = v55;
                v1054 = v1043;
                v1055 = v1050;
                v1056 = v55;
                v1057 = v55;
                v1058 = v1038;
                v1059 = v1045;
                v1060 = v873;
                v1061 = v874;
                v1062 = v1044;
                v1063 = v1051;
                v1064 = v873;
                v1065 = v874;
                v1066 = v1042;
                v1067 = v1049;
            }
            let v1075 = v1074 * (v1068 - v9);
            let v1076 = ((Lanes([v1070, 0.0])) - (Lanes([0.0, v13]))) * v1074;
            let v1077 = ddt(3130, v1075);
            let v1078 = v1076 * v892;
            let v1080 = v1079 * v19;
            let v1081 = v22 * v1079;
            let v1082 = ddt(3134, v1080);
            let v1083 = v1081 * v892;
            let v1085 = v9 - v1084;
            let v1090 = v147 * v1085;
            let v1092 = ((Lanes([v13, 0.0])) - (Lanes([0.0, v1087]))) * v147;
            let v1095 = (Lanes([0.0, 0.0, (v159 * v1085)])) + (Lanes([v1092[0], v1092[1], 0.0]));
            let v1096 = ddt(3138, v1090);
            let v1097 = v1095 * v892;
            let v1110: f64;
            let v1111: Lanes<6>;
            if v1098 != 0.0 {
                let v1102 = (Lanes([0.0, v1087])) - (Lanes([v5, 0.0]));
                let v1103 = (v1084 - v1) / v702;
                let v1104 = v705 * v1103;
                let v1108 = ((Lanes([0.0, 0.0, v1102[0], 0.0, v1102[1], 0.0])) - (Lanes([v1104[0], v1104[1], v1104[2], v1104[3], 0.0, v1104[4]]))) / v702;
                v1110 = v1103;
                v1111 = v1108;
            } else {
                v1110 = v55;
                v1111 = v1109;
            }
            let v1119 = v1118 * (v1112 - v0);
            let v1120 = ((Lanes([0.0, v1114])) - (Lanes([v3, 0.0]))) * v1118;
            let v1121 = ddt(3149, v1119);
            let v1122 = v1120 * v892;
            let v1132: f64;
            let v1133: Lanes<2>;
            if v1123 != 0.0 {
                let v1129 = (v1112 - v1) / v1128;
                let v1130 = ((Lanes([0.0, v1114])) - (Lanes([v5, 0.0]))) / v1128;
                v1132 = v1129;
                v1133 = v1130;
            } else {
                v1132 = v55;
                v1133 = v1131;
            }
            let v1144: f64;
            let v1145: Lanes<2>;
            if v1134 != 0.0 {
                let v1140 = (v8 - v23) / v1139;
                let v1141 = ((Lanes([v11, 0.0])) - (Lanes([0.0, v25]))) / v1139;
                v1144 = v1140;
                v1145 = v1141;
            } else {
                v1144 = v55;
                v1145 = v1143;
            }
            let v1155: f64;
            let v1156: Lanes<2>;
            if v1146 != 0.0 {
                let v1152 = (v8 - v0) / v1151;
                let v1153 = ((Lanes([v11, 0.0])) - (Lanes([0.0, v3]))) / v1151;
                v1155 = v1152;
                v1156 = v1153;
            } else {
                v1155 = v55;
                v1156 = v1154;
            }
            let v1169: f64;
            let v1170: f64;
            let v1171: f64;
            let v1172: f64;
            let v1173: f64;
            let v1174: f64;
            let v1175: f64;
            let v1176: f64;
            let v1177: f64;
            let v1178: f64;
            if v1157 != 0.0 {
                let v1160 = v1158 * v1159;
                let v1162 = v1161 * v1159;
                let v1164 = v1163 * v1158;
                let v1165 = v1161 * v1163;
                let v1166 = ddt(3190, v1164);
                let v1167 = v1165 * v892;
                v1169 = v1160;
                v1170 = v1166;
                v1171 = v55;
                v1172 = v1164;
                v1173 = v55;
                v1174 = v1162;
                v1175 = v1167;
                v1176 = v1180;
                v1177 = v1165;
                v1178 = v1180;
            } else {
                let v1185: f64;
                let v1186: f64;
                let v1187: f64;
                let v1188: f64;
                if v1168 != 0.0 {
                    let v1181 = v1163 * v1158;
                    let v1182 = v1161 * v1163;
                    let v1183 = ddt(3207, v1181);
                    let v1184 = v1182 * v892;
                    v1185 = v1183;
                    v1186 = v1181;
                    v1187 = v1184;
                    v1188 = v1182;
                } else {
                    v1185 = v55;
                    v1186 = v55;
                    v1187 = v1180;
                    v1188 = v1180;
                }
                v1169 = v55;
                v1170 = v55;
                v1171 = v1185;
                v1172 = v55;
                v1173 = v1186;
                v1174 = v1180;
                v1175 = v1180;
                v1176 = v1187;
                v1177 = v1180;
                v1178 = v1188;
            }
            let v1198: f64;
            let v1199: Lanes<6>;
            if v1179 != 0.0 {
                let v1190 = v1189 * v721;
                let v1193 = v723 * v1189;
                let v1196 = (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, (v1191 * v721)])) + (Lanes([v1193[0], v1193[1], v1193[2], v1193[3], v1193[4], 0.0]));
                v1198 = v1190;
                v1199 = v1196;
            } else {
                v1198 = v55;
                v1199 = v1197;
            }
            let v1202 = v1200 * v1201;
            let v1204 = v1203 * v1200;
            let v1205 = ddt(3229, v1202);
            let v1206 = v1204 * v892;
            let v1222: f64;
            let v1223: f64;
            let v1224: f64;
            let v1225: f64;
            let v1226: f64;
            let v1227: Lanes<6>;
            let v1228: f64;
            let v1229: f64;
            let v1230: f64;
            let v1231: f64;
            if v1207 != 0.0 {
                let v1209 = v1208 * v722;
                let v1212 = v724 * v1208;
                let v1215 = (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, (v1210 * v722)])) + (Lanes([v1212[0], v1212[1], v1212[2], v1212[3], v1212[4], 0.0]));
                let v1217 = v1216 * v1208;
                let v1218 = v1210 * v1216;
                let v1219 = ddt(3243, v1217);
                let v1220 = v1218 * v892;
                v1222 = v1209;
                v1223 = v1219;
                v1224 = v55;
                v1225 = v1217;
                v1226 = v55;
                v1227 = v1215;
                v1228 = v1220;
                v1229 = v1233;
                v1230 = v1218;
                v1231 = v1233;
            } else {
                let v1238: f64;
                let v1239: f64;
                let v1240: f64;
                let v1241: f64;
                if v1221 != 0.0 {
                    let v1234 = v1216 * v1208;
                    let v1235 = v1210 * v1216;
                    let v1236 = ddt(3260, v1234);
                    let v1237 = v1235 * v892;
                    v1238 = v1236;
                    v1239 = v1234;
                    v1240 = v1237;
                    v1241 = v1235;
                } else {
                    v1238 = v55;
                    v1239 = v55;
                    v1240 = v1233;
                    v1241 = v1233;
                }
                v1222 = v55;
                v1223 = v55;
                v1224 = v1238;
                v1225 = v55;
                v1226 = v1239;
                v1227 = v1242;
                v1228 = v1233;
                v1229 = v1240;
                v1230 = v1233;
                v1231 = v1241;
            }
            let v1244: f64;
            let v1245: f64;
            let v1246: f64;
            let v1247: f64;
            let v1248: f64;
            let v1249: f64;
            let v1250: f64;
            let v1251: f64;
            let v1252: f64;
            let v1253: Lanes<3>;
            let v1254: Lanes<2>;
            let v1255: Lanes<2>;
            if v1232 != 0.0 {
                v1244 = v55;
                v1245 = v55;
                v1246 = v55;
                v1247 = v55;
                v1248 = v55;
                v1249 = v55;
                v1250 = v1256;
                v1251 = v1257;
                v1252 = v1256;
                v1253 = v1258;
                v1254 = v1259;
                v1255 = v1259;
            } else {
                let v1260: f64;
                let v1261: f64;
                let v1262: f64;
                let v1263: f64;
                let v1264: f64;
                let v1265: f64;
                let v1266: f64;
                let v1267: f64;
                let v1268: f64;
                let v1269: Lanes<3>;
                let v1270: Lanes<2>;
                let v1271: Lanes<2>;
                if v1243 != 0.0 {
                    let v1274: f64;
                    let v1275: f64;
                    let v1276: f64;
                    let v1277: f64;
                    let v1278: f64;
                    let v1279: f64;
                    let v1280: f64;
                    let v1281: f64;
                    let v1282: f64;
                    let v1283: Lanes<3>;
                    let v1284: Lanes<2>;
                    let v1285: Lanes<2>;
                    if v1142 != 0.0 {
                        let v1273 = if (v316[1]) > v55 { 1.0 } else { 0.0 };
                        let v1290 = (v1286 * v43) * v1289;
                        let v1297 = (v1290 * v145) * v1296;
                        let v1298 = ((((v44 * v1286) * v1289) * v145) + (v157 * v1290)) * v1296;
                        let v1300 = v1298 * v1297;
                        let v1304 = (v62 - (v1297 * v1297)).sqrt();
                        let v1311 = (-v1297) * v1310;
                        let v1322 = (Lanes([(((v1298 * v17) * v1310) * v1315), 0.0])) + (Lanes([0.0, (v1318 * v1311)]));
                        let v1330 = (Lanes([((((v1300 + v1300) * v17) * (v36 / (v34 * v1304))) * v1323), 0.0])) + (Lanes([0.0, (v1326 * v1304)]));
                        let v1331 = (v1311 * v1315) + (v1304 * v1323);
                        let v1334 = (Lanes([v1322[0], v1322[1], 0.0])) + (Lanes([v1330[0], 0.0, v1330[1]]));
                        let v1335 = -(v1297 * v1310);
                        let v1337 = v1335 * v1315;
                        let v1342 = (Lanes([(((v1298 * v1310) * v17) * v1315), 0.0])) + (Lanes([0.0, (v1318 * v1335)]));
                        let v1343 = ddt(3399, v1337);
                        let v1344 = v1342 * v892;
                        v1274 = v1315;
                        v1275 = v1323;
                        v1276 = v1315;
                        v1277 = v1331;
                        v1278 = v1343;
                        v1279 = v1337;
                        v1280 = v1318;
                        v1281 = v1326;
                        v1282 = v1318;
                        v1283 = v1334;
                        v1284 = v1344;
                        v1285 = v1342;
                    } else {
                        v1274 = v55;
                        v1275 = v55;
                        v1276 = v55;
                        v1277 = v55;
                        v1278 = v55;
                        v1279 = v55;
                        v1280 = v1256;
                        v1281 = v1257;
                        v1282 = v1256;
                        v1283 = v1258;
                        v1284 = v1259;
                        v1285 = v1259;
                    }
                    v1260 = v1274;
                    v1261 = v1275;
                    v1262 = v1276;
                    v1263 = v1277;
                    v1264 = v1278;
                    v1265 = v1279;
                    v1266 = v1280;
                    v1267 = v1281;
                    v1268 = v1282;
                    v1269 = v1283;
                    v1270 = v1284;
                    v1271 = v1285;
                } else {
                    v1260 = v55;
                    v1261 = v55;
                    v1262 = v55;
                    v1263 = v55;
                    v1264 = v55;
                    v1265 = v55;
                    v1266 = v1256;
                    v1267 = v1257;
                    v1268 = v1256;
                    v1269 = v1258;
                    v1270 = v1259;
                    v1271 = v1259;
                }
                v1244 = v1260;
                v1245 = v1261;
                v1246 = v1262;
                v1247 = v1263;
                v1248 = v1264;
                v1249 = v1265;
                v1250 = v1266;
                v1251 = v1267;
                v1252 = v1268;
                v1253 = v1269;
                v1254 = v1270;
                v1255 = v1271;
            }
            let v1385: f64;
            let v1386: f64;
            let v1387: f64;
            let v1388: f64;
            let v1389: f64;
            let v1390: f64;
            let v1391: Lanes<5>;
            let v1392: f64;
            let v1393: f64;
            let v1394: f64;
            if v1345 != 0.0 {
                let v1347 = v1346 * v30;
                let v1348 = v38 * v1346;
                let v1349 = ddt(3482, v1347);
                let v1350 = v1348 * v892;
                let v1352 = -v1351;
                let v1357 = v22 * v1352;
                let v1360 = (Lanes([0.0, 0.0, ((v1353 * v17) * v19)])) + (Lanes([v1357[0], v1357[1], 0.0]));
                let v1363 = v7 * v773;
                let v1365 = (v774 * v2) + (Lanes([v1363[0], v1363[1], 0.0]));
                let v1366 = (v1352 * v19) + (v773 * v2);
                let v1376 = v1375 * (v1366.abs());
                let v1377 = (((Lanes([v1360[0], v1360[1], 0.0, 0.0, v1360[2]])) + (Lanes([0.0, v1365[0], v1365[1], v1365[2], 0.0]))) * ((v34 * (if v1366 >= v32 { 1.0 } else { 0.0 })) - v36)) * v1375;
                let v1379 = v30 / v1378;
                let v1380 = v38 / v1378;
                v1385 = v1349;
                v1386 = v1376;
                v1387 = v1379;
                v1388 = v55;
                v1389 = v1347;
                v1390 = v1350;
                v1391 = v1377;
                v1392 = v1380;
                v1393 = v42;
                v1394 = v1348;
            } else {
                let v1382 = v30 * v1381;
                let v1383 = v38 * v1381;
                v1385 = v55;
                v1386 = v55;
                v1387 = v55;
                v1388 = v1382;
                v1389 = v55;
                v1390 = v42;
                v1391 = v1384;
                v1392 = v42;
                v1393 = v1383;
                v1394 = v42;
            }
            let v1395 = v885[0];
            let v1396 = v885[1];
            let v1397 = v885[2];
            let v1398 = v885[3];
            let v1399 = v885[4];
            let v1400 = v893;
            let v1401 = v1353;
            let v1402 = v900;
            let v1403 = v774[0];
            let v1404 = v774[1];
            let v1405 = v774[2];
            let v1406 = v786[0];
            let v1407 = v786[1];
            let v1408 = v786[2];
            let v1409 = v1060[0];
            let v1410 = v1060[1];
            let v1411 = v1060[2];
            let v1412 = v1060[3];
            let v1413 = v1061[0];
            let v1414 = v1061[1];
            let v1415 = v1061[2];
            let v1416 = v1061[3];
            let v1417 = v1062[0];
            let v1418 = v1062[1];
            let v1419 = v1062[2];
            let v1420 = v1062[3];
            let v1421 = v1063[0];
            let v1422 = v1063[1];
            let v1423 = v1063[2];
            let v1424 = v1063[3];
            let v1425 = v1078[0];
            let v1426 = v1078[1];
            let v1427 = v1083[0];
            let v1428 = v1083[1];
            let v1429 = v1097[0];
            let v1430 = v1097[1];
            let v1431 = v1097[2];
            let v1432 = v1111[0];
            let v1433 = v1111[1];
            let v1434 = v1111[2];
            let v1435 = v1111[3];
            let v1436 = v1111[4];
            let v1437 = v1111[5];
            let v1438 = v1122[0];
            let v1439 = v1122[1];
            let v1440 = v1133[0];
            let v1441 = v1133[1];
            let v1442 = v1145[0];
            let v1443 = v1145[1];
            let v1444 = v1156[0];
            let v1445 = v1156[1];
            let v1446 = v1174;
            let v1447 = v1175;
            let v1448 = v1176;
            let v1449 = v1199[0];
            let v1450 = v1199[1];
            let v1451 = v1199[2];
            let v1452 = v1199[3];
            let v1453 = v1199[4];
            let v1454 = v1199[5];
            let v1455 = v1206;
            let v1456 = v1227[0];
            let v1457 = v1227[1];
            let v1458 = v1227[2];
            let v1459 = v1227[3];
            let v1460 = v1227[4];
            let v1461 = v1227[5];
            let v1462 = v1228;
            let v1463 = v1229;
            let v1464 = v1250;
            let v1465 = v1251;
            let v1466 = v1252;
            let v1467 = v1253[0];
            let v1468 = v1253[1];
            let v1469 = v1253[2];
            let v1470 = v1254[0];
            let v1471 = v1254[1];
            let v1472 = v1318;
            let v1473 = v1326;
            let v1474 = v1390;
            let v1475 = v1391[0];
            let v1476 = v1391[1];
            let v1477 = v1391[2];
            let v1478 = v1391[3];
            let v1479 = v1391[4];
            let v1480 = v1392;
            let v1481 = v1393;
            let v1482 = v890;
            let v1483 = v898;
            let v1484 = v1064[0];
            let v1485 = v1064[1];
            let v1486 = v1064[2];
            let v1487 = v1064[3];
            let v1488 = v1065[0];
            let v1489 = v1065[1];
            let v1490 = v1065[2];
            let v1491 = v1065[3];
            let v1492 = v1066[0];
            let v1493 = v1066[1];
            let v1494 = v1066[2];
            let v1495 = v1066[3];
            let v1496 = v1067[0];
            let v1497 = v1067[1];
            let v1498 = v1067[2];
            let v1499 = v1067[3];
            let v1500 = v1076[0];
            let v1501 = v1076[1];
            let v1502 = v1081[0];
            let v1503 = v1081[1];
            let v1504 = v1095[0];
            let v1505 = v1095[1];
            let v1506 = v1095[2];
            let v1507 = v1120[0];
            let v1508 = v1120[1];
            let v1509 = v1177;
            let v1510 = v1178;
            let v1511 = v1204;
            let v1512 = v1230;
            let v1513 = v1231;
            let v1514 = v1255[0];
            let v1515 = v1255[1];
            let v1516 = v1394;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(12),
            None,
            multiplicity * (v884),
            [3, 4, 5, 8, 11],
            [v1395, v1396, v1397, v1398, v1399],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(12),
            None,
            multiplicity * (v891),
            [12],
            [v1400],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(13),
            None,
            multiplicity * (v1351),
            [13],
            [v1401],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(12), Some(13), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 1>(
            0,
            v899,
            [],
            [],
            [0],
            [v1402],
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            Some(5),
            multiplicity * (v1351),
            [13],
            [v1401],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(8),
            Some(5),
            multiplicity * (v773),
            [5, 8, 11],
            [v1403, v1404, v1405],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(3),
            multiplicity * (v785),
            [3, 7, 11],
            [v1406, v1407, v1408],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(3),
            multiplicity * (v1052),
            [3, 5, 7, 11],
            [v1409, v1410, v1411, v1412],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(5),
            multiplicity * (v1053),
            [3, 5, 8, 11],
            [v1413, v1414, v1415, v1416],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(3),
            multiplicity * (v1054),
            [3, 5, 7, 11],
            [v1417, v1418, v1419, v1420],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(5),
            multiplicity * (v1055),
            [3, 5, 8, 11],
            [v1421, v1422, v1423, v1424],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(4),
            Some(3),
            multiplicity * (v1077),
            [1, 3],
            [v1425, v1426],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(3),
            Some(5),
            multiplicity * (v1082),
            [3, 5],
            [v1427, v1428],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(3),
            Some(10),
            multiplicity * (v1096),
            [3, 10, 11],
            [v1429, v1430, v1431],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            Some(5),
            multiplicity * (v1110),
            [3, 4, 5, 8, 10, 11],
            [v1432, v1433, v1434, v1435, v1436, v1437],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(10), Some(5), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            staged[36],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(9),
            Some(8),
            multiplicity * (v1121),
            [8, 9],
            [v1438, v1439],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(9),
            Some(5),
            multiplicity * (v1132),
            [5, 9],
            [v1440, v1441],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(9), Some(5), 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            staged[37],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(4),
            Some(7),
            multiplicity * (v1144),
            [4, 7],
            [v1442, v1443],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(7),
            multiplicity * (staged[38]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(4), Some(7), 3, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            3,
            staged[39],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(4),
            Some(8),
            multiplicity * (v1155),
            [4, 8],
            [v1444, v1445],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(4), Some(8), 4, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            4,
            staged[40],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(1), Some(4), 5, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 1>(
            5,
            v1169,
            [],
            [],
            [5],
            [v1446],
        );
        stamper.stamp_potential_branch_local(Some(1), Some(4), 6, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 1>(
            6,
            v1170,
            [],
            [],
            [5],
            [v1447],
        );
        stamper.stamp_potential_branch_local(Some(1), Some(4), 7, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            7,
            staged[41],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(1), Some(4), 8, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 1>(
            8,
            v1171,
            [],
            [],
            [5],
            [v1448],
        );
        stamper.stamp_potential_branch_local(Some(1), Some(4), 9, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            9,
            staged[42],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(5), Some(6), 10, multiplicity);
        stamper.stamp_potential_sparse_local::<5, 1>(
            10,
            v1198,
            [3, 4, 5, 8, 11],
            [v1449, v1450, v1451, v1452, v1453],
            [10],
            [v1454],
        );
        stamper.stamp_potential_branch_local(Some(5), Some(6), 11, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            11,
            staged[43],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(5), Some(6), 12, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            12,
            staged[44],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(6), Some(2), 13, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 1>(
            13,
            v1205,
            [],
            [],
            [13],
            [v1455],
        );
        stamper.stamp_potential_branch_local(Some(3), Some(0), 14, multiplicity);
        stamper.stamp_potential_sparse_local::<5, 1>(
            14,
            v1222,
            [3, 4, 5, 8, 11],
            [v1456, v1457, v1458, v1459, v1460],
            [14],
            [v1461],
        );
        stamper.stamp_potential_branch_local(Some(3), Some(0), 15, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 1>(
            15,
            v1223,
            [],
            [],
            [14],
            [v1462],
        );
        stamper.stamp_potential_branch_local(Some(3), Some(0), 16, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            16,
            staged[45],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(3), Some(0), 17, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 1>(
            17,
            v1224,
            [],
            [],
            [14],
            [v1463],
        );
        stamper.stamp_potential_branch_local(Some(3), Some(0), 18, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            18,
            staged[46],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(3),
            Some(5),
            multiplicity * (staged[47]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(3),
            Some(5),
            multiplicity * (staged[48]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(14),
            None,
            multiplicity * (staged[49]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(14),
            None,
            multiplicity * (v1244),
            [14],
            [v1464],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(15),
            None,
            multiplicity * (staged[50]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(15),
            None,
            multiplicity * (v1245),
            [15],
            [v1465],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            Some(5),
            multiplicity * (v1246),
            [14],
            [v1466],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(4),
            Some(3),
            multiplicity * (v1247),
            [11, 14, 15],
            [v1467, v1468, v1469],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(4),
            Some(3),
            multiplicity * (v1248),
            [11, 14],
            [v1470, v1471],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(3),
            Some(5),
            multiplicity * (staged[51]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(5),
            multiplicity * (staged[52]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(3),
            Some(5),
            multiplicity * (staged[53]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(3),
            Some(5),
            multiplicity * (staged[54]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(14),
            None,
            multiplicity * (v1315),
            [14],
            [v1472],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(15),
            None,
            multiplicity * (v1323),
            [15],
            [v1473],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(5),
            multiplicity * (staged[55]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(3),
            multiplicity * (staged[56]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(5),
            multiplicity * (staged[57]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(3),
            multiplicity * (staged[58]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(11),
            None,
            multiplicity * (v1385),
            [11],
            [v1474],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(11),
            None,
            multiplicity * (v1386),
            [3, 5, 8, 11, 13],
            [v1475, v1476, v1477, v1478, v1479],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(11),
            None,
            multiplicity * (v1387),
            [11],
            [v1480],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(11),
            None,
            multiplicity * (v1388),
            [11],
            [v1481],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = v884;
        self.canonical_reactive[1] = v888;
        self.canonical_reactive[2] = v1482;
        self.canonical_reactive[3] = v1351;
        self.canonical_reactive[4] = v896;
        self.canonical_reactive[5] = v1483;
        self.canonical_reactive[6] = v1351;
        self.canonical_reactive[7] = v773;
        self.canonical_reactive[8] = v785;
        self.canonical_reactive[9] = v1056;
        self.canonical_reactive[10] = v1484;
        self.canonical_reactive[11] = v1485;
        self.canonical_reactive[12] = v1486;
        self.canonical_reactive[13] = v1487;
        self.canonical_reactive[14] = v1057;
        self.canonical_reactive[15] = v1488;
        self.canonical_reactive[16] = v1489;
        self.canonical_reactive[17] = v1490;
        self.canonical_reactive[18] = v1491;
        self.canonical_reactive[19] = v1058;
        self.canonical_reactive[20] = v1492;
        self.canonical_reactive[21] = v1493;
        self.canonical_reactive[22] = v1494;
        self.canonical_reactive[23] = v1495;
        self.canonical_reactive[24] = v1059;
        self.canonical_reactive[25] = v1496;
        self.canonical_reactive[26] = v1497;
        self.canonical_reactive[27] = v1498;
        self.canonical_reactive[28] = v1499;
        self.canonical_reactive[29] = v1075;
        self.canonical_reactive[30] = v1500;
        self.canonical_reactive[31] = v1501;
        self.canonical_reactive[32] = v1080;
        self.canonical_reactive[33] = v1502;
        self.canonical_reactive[34] = v1503;
        self.canonical_reactive[35] = v1090;
        self.canonical_reactive[36] = v1504;
        self.canonical_reactive[37] = v1505;
        self.canonical_reactive[38] = v1506;
        self.canonical_reactive[39] = v1110;
        self.canonical_reactive[40] = staged[36];
        self.canonical_reactive[41] = v1119;
        self.canonical_reactive[42] = v1507;
        self.canonical_reactive[43] = v1508;
        self.canonical_reactive[44] = v1132;
        self.canonical_reactive[45] = staged[37];
        self.canonical_reactive[46] = v1144;
        self.canonical_reactive[47] = staged[38];
        self.canonical_reactive[48] = staged[39];
        self.canonical_reactive[49] = v1155;
        self.canonical_reactive[50] = staged[40];
        self.canonical_reactive[51] = v1169;
        self.canonical_reactive[52] = v1172;
        self.canonical_reactive[53] = v1509;
        self.canonical_reactive[54] = staged[41];
        self.canonical_reactive[55] = v1173;
        self.canonical_reactive[56] = v1510;
        self.canonical_reactive[57] = staged[42];
        self.canonical_reactive[58] = v1198;
        self.canonical_reactive[59] = staged[43];
        self.canonical_reactive[60] = staged[44];
        self.canonical_reactive[61] = v1202;
        self.canonical_reactive[62] = v1511;
        self.canonical_reactive[63] = v1222;
        self.canonical_reactive[64] = v1225;
        self.canonical_reactive[65] = v1512;
        self.canonical_reactive[66] = staged[45];
        self.canonical_reactive[67] = v1226;
        self.canonical_reactive[68] = v1513;
        self.canonical_reactive[69] = staged[46];
        self.canonical_reactive[70] = staged[47];
        self.canonical_reactive[71] = staged[48];
        self.canonical_reactive[72] = staged[49];
        self.canonical_reactive[73] = v1244;
        self.canonical_reactive[74] = staged[50];
        self.canonical_reactive[75] = v1245;
        self.canonical_reactive[76] = v1246;
        self.canonical_reactive[77] = v1247;
        self.canonical_reactive[78] = v1249;
        self.canonical_reactive[79] = v1514;
        self.canonical_reactive[80] = v1515;
        self.canonical_reactive[81] = staged[51];
        self.canonical_reactive[82] = staged[52];
        self.canonical_reactive[83] = staged[53];
        self.canonical_reactive[84] = staged[54];
        self.canonical_reactive[85] = v1315;
        self.canonical_reactive[86] = v1323;
        self.canonical_reactive[87] = staged[55];
        self.canonical_reactive[88] = staged[56];
        self.canonical_reactive[89] = staged[57];
        self.canonical_reactive[90] = staged[58];
        self.canonical_reactive[91] = v1389;
        self.canonical_reactive[92] = v1516;
        self.canonical_reactive[93] = v1386;
        self.canonical_reactive[94] = v1387;
        self.canonical_reactive[95] = v1388;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(12),
            None,
            &[12],
            &[cached[2]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_potential_reactive_indexed_dense_local(
            0,
            &[],
            &[],
            &[0],
            &[cached[5]],
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(3),
            &[3, 5, 7, 11],
            &[cached[10], cached[11], cached[12], cached[13]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(5),
            &[3, 5, 8, 11],
            &[cached[15], cached[16], cached[17], cached[18]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(3),
            &[3, 5, 7, 11],
            &[cached[20], cached[21], cached[22], cached[23]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(5),
            &[3, 5, 8, 11],
            &[cached[25], cached[26], cached[27], cached[28]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            Some(3),
            &[1, 3],
            &[cached[30], cached[31]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            Some(5),
            &[3, 5],
            &[cached[33], cached[34]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            Some(10),
            &[3, 10, 11],
            &[cached[36], cached[37], cached[38]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(9),
            Some(8),
            &[8, 9],
            &[cached[42], cached[43]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_potential_reactive_indexed_dense_local(
            6,
            &[],
            &[],
            &[5],
            &[cached[53]],
        );
        stamper.stamp_potential_reactive_indexed_dense_local(
            8,
            &[],
            &[],
            &[5],
            &[cached[56]],
        );
        stamper.stamp_potential_reactive_indexed_dense_local(
            13,
            &[],
            &[],
            &[13],
            &[cached[62]],
        );
        stamper.stamp_potential_reactive_indexed_dense_local(
            15,
            &[],
            &[],
            &[14],
            &[cached[65]],
        );
        stamper.stamp_potential_reactive_indexed_dense_local(
            17,
            &[],
            &[],
            &[14],
            &[cached[68]],
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            Some(3),
            &[11, 14],
            &[cached[79], cached[80]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(11),
            None,
            &[11],
            &[cached[92]],
            &[],
            &[],
            multiplicity,
        );
    }

}

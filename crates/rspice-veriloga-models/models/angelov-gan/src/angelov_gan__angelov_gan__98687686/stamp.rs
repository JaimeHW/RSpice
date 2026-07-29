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
        let mut key = Vec::with_capacity(194);
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
        self.canonical_staged[17] = values[2];
        self.canonical_staged[3] = values[3];
        self.canonical_staged[18] = values[4];
        self.canonical_staged[19] = values[5];
        self.canonical_staged[21] = values[6];
        self.canonical_staged[22] = values[7];
        self.canonical_staged[23] = values[8];
        self.canonical_staged[20] = values[9];
        self.canonical_staged[24] = values[10];
        self.canonical_staged[25] = values[11];
        self.canonical_staged[4] = values[12];
        self.canonical_staged[5] = values[13];
        self.canonical_staged[6] = values[14];
        self.canonical_staged[26] = values[15];
        self.canonical_staged[27] = values[16];
        self.canonical_staged[7] = values[17];
        self.canonical_staged[29] = values[18];
        self.canonical_staged[8] = values[19];
        self.canonical_staged[30] = values[20];
        self.canonical_staged[9] = values[21];
        self.canonical_staged[10] = values[22];
        self.canonical_staged[31] = values[23];
        self.canonical_staged[11] = values[24];
        self.canonical_staged[12] = values[25];
        self.canonical_staged[13] = values[26];
        self.canonical_staged[14] = values[27];
        self.canonical_staged[28] = values[28];
        self.canonical_staged[32] = values[29];
        self.canonical_staged[33] = values[30];
        self.canonical_staged[34] = values[31];
        self.canonical_staged[35] = values[32];
        self.canonical_staged[36] = values[33];
        self.canonical_staged[37] = values[34];
        self.canonical_staged[38] = values[35];
        self.canonical_staged[39] = values[36];
        self.canonical_staged[40] = values[37];
        self.canonical_staged[41] = values[38];
        self.canonical_staged[42] = values[39];
        self.canonical_staged[46] = values[40];
        self.canonical_staged[47] = values[41];
        self.canonical_staged[48] = values[42];
        self.canonical_staged[49] = values[43];
        self.canonical_staged[51] = values[44];
        self.canonical_staged[53] = values[45];
        self.canonical_staged[55] = values[46];
        self.canonical_staged[57] = values[47];
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
                let v0 = if parameter_given[100] { 1.0 } else { 0.0 };
                let v1 = parameters[100];
                let v2 = 2.7315e2f64;
                let v4 = 3.0015e2f64;
                let v6 = parameters[66];
                let v7 = 0e0f64;
                let v9 = if parameter_given[43] { 1.0 } else { 0.0 };
                let v11 = if parameter_given[44] { 1.0 } else { 0.0 };
                let v13 = 5e-1f64;
                let v14 = parameters[44];
                let v16 = parameters[4];
                let v18 = 1e0f64;
                let v22 = 4e0f64;
                let v25 = 2e0f64;
                let v27 = 3e0f64;
                let v30 = parameters[5];
                let v32 = parameters[85];
                let v34 = parameters[83];
                let v37 = parameters[84];
                let v43 = 1e-3f64;
                let v44 = parameters[82];
                let v46 = parameters[6];
                let v49 = parameters[56];
                let v54 = parameters[38];
                let v62 = parameters[39];
                let v63 = parameters[40];
                let v65 = parameters[41];
                let v67 = -5e-1f64;
                let v72 = parameters[58];
                let v74 = 0e0f64;
                let v76 = parameters[63];
                let v78 = parameters[62];
                let v81 = 0e0f64;
                let v83 = parameters[60];
                let v85 = 0e0f64;
                let v87 = parameters[51];
                let v89 = 0e0f64;
                let v91 = parameters[49];
                let v93 = 0e0f64;
                let v95 = parameters[46];
                let v97 = 0e0f64;
                let v99 = parameters[50];
                let v101 = 0e0f64;
                let v103 = parameters[47];
                let v105 = parameters[48];
                let v108 = 0e0f64;
                let v110 = parameters[7];
                let mut out15: f64 = 0.0;
                let mut out19: f64 = 0.0;
                let mut out26: f64 = 0.0;
                let mut out28: f64 = 0.0;
                let mut out29: f64 = 0.0;
                let mut out40: f64 = 0.0;
                let mut out48: f64 = 0.0;
                let mut out55: f64 = 0.0;
                let mut out56: f64 = 0.0;
                let mut out57: f64 = 0.0;
                let mut out58: f64 = 0.0;
                let mut out59: f64 = 0.0;
                let mut out60: f64 = 0.0;
                let mut out61: f64 = 0.0;
                let mut out69: f64 = 0.0;
                let mut out70: f64 = 0.0;
                let mut out71: f64 = 0.0;
                let mut out112: f64 = 0.0;
                let mut out113: f64 = 0.0;
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
                        let v26 = if v16 == v25 { 1.0 } else { 0.0 };
                        out26 = v26;
                        if v26 != 0.0 {
                        } else {
                            let v28 = if v16 == v27 { 1.0 } else { 0.0 };
                            out28 = v28;
                            if v28 != 0.0 {
                            } else {
                                let v29 = if v16 == v22 { 1.0 } else { 0.0 };
                                out29 = v29;
                            }
                        }
                    }
                }
                let v24 = if (if v17 != 0.0 || (if v16 == v18 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v16 == v22 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v31 = if v30 == v7 { 1.0 } else { 0.0 };
                let v41: f64;
                let v42: f64;
                if v31 != 0.0 {
                    v41 = v7;
                    v42 = v7;
                } else {
                    let v33 = -v32;
                    let v36 = rspice_limexp((v33 * v34));
                    let v39 = rspice_limexp((v33 * v37));
                    let v40 = if v30 == v18 { 1.0 } else { 0.0 };
                    out40 = v40;
                    v41 = v36;
                    v42 = v39;
                }
                let v45 = v43 * v44;
                let v47 = if v46 == v7 { 1.0 } else { 0.0 };
                if v47 != 0.0 {
                } else {
                    let v48 = if v46 == v18 { 1.0 } else { 0.0 };
                    out48 = v48;
                    if v48 != 0.0 {
                        let v55 = v25 * v54;
                        out55 = v55;
                    } else {
                        let v56 = if v46 == v25 { 1.0 } else { 0.0 };
                        out56 = v56;
                        if v56 != 0.0 {
                            let v57 = v25 * v54;
                            out57 = v57;
                        } else {
                            let v58 = if v46 == v27 { 1.0 } else { 0.0 };
                            out58 = v58;
                            if v58 != 0.0 {
                                let v59 = v18 - v54;
                                out59 = v59;
                                let v60 = v25 * v54;
                                out60 = v60;
                            } else {
                                let v61 = if v46 == v22 { 1.0 } else { 0.0 };
                                out61 = v61;
                                if v61 != 0.0 {
                                    let v69 = (v62 * v63) * ((v65 + v18).powf(v67));
                                    out69 = v69;
                                    let v70 = v18 - v54;
                                    out70 = v70;
                                    let v71 = v25 * v54;
                                    out71 = v71;
                                } else {
                                }
                            }
                        }
                    }
                }
                let v50 = v49 / v27;
                let v53 = if (if v46 == v25 { 1.0 } else { 0.0 }) != 0.0 || (if v46 == v22 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v73 = if v72 > v7 { 1.0 } else { 0.0 };
                let v75: f64;
                if v73 != 0.0 {
                    v75 = v7;
                } else {
                    v75 = v74;
                }
                let v80 = if (if v76 > v7 { 1.0 } else { 0.0 }) != 0.0 || (if v78 > v7 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v82: f64;
                if v80 != 0.0 {
                    v82 = v7;
                } else {
                    v82 = v81;
                }
                let v84 = if v83 > v7 { 1.0 } else { 0.0 };
                let v86: f64;
                if v84 != 0.0 {
                    v86 = v7;
                } else {
                    v86 = v85;
                }
                let v88 = if v87 > v7 { 1.0 } else { 0.0 };
                let v90: f64;
                if v88 != 0.0 {
                    v90 = v7;
                } else {
                    v90 = v89;
                }
                let v92 = if v91 > v7 { 1.0 } else { 0.0 };
                let v94: f64;
                if v92 != 0.0 {
                    v94 = v7;
                } else {
                    v94 = v93;
                }
                let v96 = if v95 > v7 { 1.0 } else { 0.0 };
                let v98: f64;
                if v96 != 0.0 {
                    v98 = v7;
                } else {
                    v98 = v97;
                }
                let v100 = if v99 > v7 { 1.0 } else { 0.0 };
                let v102: f64;
                if v100 != 0.0 {
                    v102 = v7;
                } else {
                    v102 = v101;
                }
                let v107 = if (if v103 > v7 { 1.0 } else { 0.0 }) != 0.0 || (if v105 > v7 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v109: f64;
                if v107 != 0.0 {
                    v109 = v7;
                } else {
                    v109 = v108;
                }
                let v111 = if v110 == v7 { 1.0 } else { 0.0 };
                if v111 != 0.0 {
                } else {
                    let v112 = if v110 == v18 { 1.0 } else { 0.0 };
                    out112 = v112;
                    if v112 != 0.0 {
                    } else {
                        let v113 = if v110 == v25 { 1.0 } else { 0.0 };
                        out113 = v113;
                    }
                }
            [v5, v8, v12, out15, v17, out19, out26, out28, out29, v24, v31, out40, v41, v45, v42, v47, out48, out55, out56, out57, out58, out59, out60, out61, out69, out70, out71, v50, v53, v73, v80, v84, v88, v92, v96, v100, v107, v111, out112, out113, v75, v82, v86, v90, v94, v98, v102, v109]
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
                let v5 = parameters[0];
                let v6 = 0e0f64;
                let v7 = 0e0f64;
                let v9 = staged[37];
                let v11 = staged[38];
                let v12 = 0e0f64;
                let v15 = staged[39];
                let v16 = 0e0f64;
                let v19 = staged[40];
                let v20 = 0e0f64;
                let v22 = staged[41];
                let v31 = 0e0f64;
                let v32 = 0e0f64;
                let v47 = parameters[87];
                let v48 = parameters[86];
                let v51 = parameters[90];
                let v53 = 0e0f64;
                let v55 = 0e0f64;
                let v56 = 0e0f64;
                let v57 = 0e0f64;
                let v58 = 0e0f64;
                let v59 = 0e0f64;
                let v65 = 1e0f64;
                let v67 = 0e0f64;
                let v68 = 0e0f64;
                let v71 = 0e0f64;
                let v72 = 0e0f64;
                let mut out3: f64 = 0.0;
                let mut out50: f64 = 0.0;
                let mut out52: f64 = 0.0;
                let mut out60: f64 = 0.0;
                if v0 != 0.0 {
                    let v3 = v1 + v2;
                    out3 = v3;
                } else {
                }
                let v8: f64;
                if v5 != 0.0 {
                    v8 = v6;
                } else {
                    v8 = v7;
                }
                let v10: f64;
                if v9 != 0.0 {
                    let v13: f64;
                    if v5 != 0.0 {
                        v13 = v12;
                    } else {
                        v13 = v7;
                    }
                    v10 = v13;
                } else {
                    v10 = v7;
                }
                let v14: f64;
                if v11 != 0.0 {
                    let v17: f64;
                    if v5 != 0.0 {
                        v17 = v16;
                    } else {
                        v17 = v7;
                    }
                    v14 = v17;
                } else {
                    v14 = v7;
                }
                let v18: f64;
                if v15 != 0.0 {
                    let v21: f64;
                    if v5 != 0.0 {
                        v21 = v20;
                    } else {
                        v21 = v7;
                    }
                    v18 = v21;
                } else {
                    v18 = v7;
                }
                let v23: f64;
                let v24: f64;
                let v25: f64;
                let v26: f64;
                let v27: f64;
                let v28: f64;
                let v29: f64;
                let v30: f64;
                if v19 != 0.0 {
                    let v33: f64;
                    let v34: f64;
                    if v5 != 0.0 {
                        v33 = v31;
                        v34 = v32;
                    } else {
                        v33 = v7;
                        v34 = v7;
                    }
                    v23 = v33;
                    v24 = v34;
                    v25 = v7;
                    v26 = v7;
                    v27 = v7;
                    v28 = v7;
                    v29 = v7;
                    v30 = v7;
                } else {
                    let v35: f64;
                    let v36: f64;
                    let v37: f64;
                    let v38: f64;
                    let v39: f64;
                    let v40: f64;
                    if v22 != 0.0 {
                        let v41: f64;
                        let v42: f64;
                        let v43: f64;
                        let v44: f64;
                        let v45: f64;
                        let v46: f64;
                        if v5 != 0.0 {
                            let v50 = (v47 * v48).sqrt();
                            out50 = v50;
                            let v52 = if v51 > v7 { 1.0 } else { 0.0 };
                            out52 = v52;
                            let v54: f64;
                            if v52 != 0.0 {
                                v54 = v53;
                            } else {
                                v54 = v7;
                            }
                            v41 = v55;
                            v42 = v56;
                            v43 = v57;
                            v44 = v58;
                            v45 = v59;
                            v46 = v54;
                        } else {
                            v41 = v7;
                            v42 = v7;
                            v43 = v7;
                            v44 = v7;
                            v45 = v7;
                            v46 = v7;
                        }
                        v35 = v41;
                        v36 = v42;
                        v37 = v43;
                        v38 = v44;
                        v39 = v45;
                        v40 = v46;
                    } else {
                        v35 = v7;
                        v36 = v7;
                        v37 = v7;
                        v38 = v7;
                        v39 = v7;
                        v40 = v7;
                    }
                    v23 = v7;
                    v24 = v7;
                    v25 = v35;
                    v26 = v36;
                    v27 = v37;
                    v28 = v38;
                    v29 = v39;
                    v30 = v40;
                }
                let v61: f64;
                let v62: f64;
                let v63: f64;
                let v64: f64;
                if v5 != 0.0 {
                    let v60 = if v51 > v7 { 1.0 } else { 0.0 };
                    out60 = v60;
                    let v69: f64;
                    let v70: f64;
                    if v60 != 0.0 {
                        v69 = v67;
                        v70 = v68;
                    } else {
                        v69 = v7;
                        v70 = v7;
                    }
                    v61 = v71;
                    v62 = v72;
                    v63 = v69;
                    v64 = v70;
                } else {
                    v61 = v7;
                    v62 = v7;
                    v63 = v7;
                    v64 = v7;
                }
                let v66 = if v4 == v65 { 1.0 } else { 0.0 };
            [out3, out50, out52, out60, v66, v8, v10, v14, v18, v23, v24, v25, v26, v27, v28, v29, v30, v61, v62, v63, v64]
        };
        self.canonical_staged[16] = produced[0];
        self.canonical_staged[15] = produced[1];
        self.canonical_staged[43] = produced[2];
        self.canonical_staged[44] = produced[3];
        self.canonical_staged[45] = produced[4];
        self.canonical_staged[50] = produced[5];
        self.canonical_staged[52] = produced[6];
        self.canonical_staged[54] = produced[7];
        self.canonical_staged[56] = produced[8];
        self.canonical_staged[58] = produced[9];
        self.canonical_staged[59] = produced[10];
        self.canonical_staged[60] = produced[11];
        self.canonical_staged[61] = produced[12];
        self.canonical_staged[62] = produced[13];
        self.canonical_staged[63] = produced[14];
        self.canonical_staged[64] = produced[15];
        self.canonical_staged[65] = produced[16];
        self.canonical_staged[66] = produced[17];
        self.canonical_staged[67] = produced[18];
        self.canonical_staged[68] = produced[19];
        self.canonical_staged[69] = produced[20];
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
                let v1 = staged[16];
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
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13]), ctx.node_voltage(self.nodes[14]), ctx.node_voltage(self.nodes[15]), ctx.node_voltage(self.nodes[16]), ctx.node_voltage(self.nodes[17]), ctx.node_voltage(self.nodes[18])];
        let branch_unknown_flows = [ctx.branch_current(self.branches[0]), ctx.branch_current(self.branches[1]), ctx.branch_current(self.branches[2]), ctx.branch_current(self.branches[3]), ctx.branch_current(self.branches[4]), ctx.branch_current(self.branches[5]), ctx.branch_current(self.branches[6]), ctx.branch_current(self.branches[7]), ctx.branch_current(self.branches[8]), ctx.branch_current(self.branches[9]), ctx.branch_current(self.branches[10]), ctx.branch_current(self.branches[11]), ctx.branch_current(self.branches[12]), ctx.branch_current(self.branches[13]), ctx.branch_current(self.branches[14]), ctx.branch_current(self.branches[15]), ctx.branch_current(self.branches[16]), ctx.branch_current(self.branches[17]), ctx.branch_current(self.branches[18])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 4327 => 0usize, 4334 => 1usize, 4346 => 2usize, 4348 => 3usize, 4352 => 4usize, 4356 => 5usize, 4360 => 6usize, 4364 => 7usize, 4368 => 8usize, 4375 => 9usize, 4398 => 10usize, 4403 => 11usize, 4455 => 12usize, 4476 => 13usize, 4501 => 14usize, 4654 => 15usize, 4752 => 16usize, _ => usize::MAX };
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
            let v0 = node_potentials[12];
            let v1 = node_potentials[8];
            let v3 = 1e0f64;
            let v5 = 1e0f64;
            let v8 = node_potentials[10];
            let v9 = node_potentials[5];
            let v11 = 1e0f64;
            let v13 = 1e0f64;
            let v17 = -1e0f64;
            let v23 = node_potentials[11];
            let v25 = 1e0f64;
            let v29 = node_potentials[4];
            let v31 = 1e0f64;
            let v35 = parameters[1];
            let v36 = node_potentials[3];
            let v38 = 0e0f64;
            let v40 = 2e0f64;
            let v42 = 1e0f64;
            let v44 = 1e0f64;
            let v46 = staged[0];
            let v48 = 0e0f64;
            let v51 = 8.617333262e-5f64;
            let v54 = staged[1];
            let v61 = 0e0f64;
            let v63 = staged[2];
            let v70 = parameters[68];
            let v73 = 1e0f64;
            let v75 = parameters[8];
            let v78 = parameters[80];
            let v82 = parameters[20];
            let v85 = parameters[72];
            let v89 = parameters[26];
            let v92 = parameters[73];
            let v96 = parameters[29];
            let v99 = parameters[74];
            let v103 = parameters[58];
            let v106 = parameters[75];
            let v110 = parameters[59];
            let v113 = parameters[78];
            let v116 = parameters[9];
            let v118 = parameters[71];
            let v122 = parameters[30];
            let v125 = parameters[36];
            let v128 = parameters[79];
            let v131 = parameters[45];
            let v133 = parameters[81];
            let v136 = parameters[21];
            let v138 = parameters[4];
            let v140 = 4e0f64;
            let v143 = parameters[6];
            let v146 = parameters[62];
            let v147 = parameters[63];
            let v174 = staged[17];
            let v193 = staged[3];
            let v198 = parameters[43];
            let v201 = parameters[19];
            let v207 = parameters[64];
            let v213 = 1e-12f64;
            let v215 = parameters[18];
            let v221 = parameters[11];
            let v229 = parameters[69];
            let v239 = parameters[70];
            let v243 = parameters[13];
            let v246 = parameters[10];
            let v248 = parameters[15];
            let v269 = parameters[22];
            let v300 = parameters[12];
            let v329 = 5e-1f64;
            let v339 = parameters[14];
            let v350 = staged[18];
            let v360 = parameters[16];
            let v380 = staged[19];
            let v385 = staged[20];
            let v420 = parameters[17];
            let v444 = parameters[23];
            let v500 = staged[21];
            let v584 = staged[22];
            let v756 = staged[23];
            let v790 = parameters[65];
            let v831 = Lanes([0e0f64; 6]);
            let v840 = parameters[57];
            let v842 = parameters[48];
            let v845 = parameters[47];
            let v847 = parameters[50];
            let v866 = parameters[76];
            let v880 = parameters[77];
            let v884 = parameters[66];
            let v887 = staged[24];
            let v888 = -1e0f64;
            let v907 = parameters[83];
            let v913 = parameters[84];
            let v923 = staged[25];
            let v934 = parameters[85];
            let v939 = staged[4];
            let v948 = staged[5];
            let v957 = parameters[42];
            let v964 = staged[6];
            let v983 = parameters[31];
            let v990 = parameters[38];
            let v1002 = parameters[33];
            let v1005 = parameters[32];
            let v1012 = parameters[35];
            let v1015 = parameters[34];
            let v1024 = parameters[37];
            let v1040 = staged[26];
            let v1073 = parameters[27];
            let v1074 = parameters[25];
            let v1075 = Lanes([0e0f64; 4]);
            let v1076 = Lanes([0e0f64; 4]);
            let v1077 = staged[27];
            let v1088 = parameters[56];
            let v1089 = node_potentials[15];
            let v1091 = 1e0f64;
            let v1094 = ddt_scale();
            let v1096 = staged[14];
            let v1097 = branch_unknown_flows[0];
            let v1099 = 1e0f64;
            let v1103 = staged[28];
            let v1120 = staged[7];
            let v1128 = staged[29];
            let v1168 = staged[8];
            let v1230 = staged[30];
            let v1239 = parameters[40];
            let v1246 = parameters[41];
            let v1248 = -1.5e0f64;
            let v1250 = -2.5e0f64;
            let v1254 = 0e0f64;
            let v1277 = staged[9];
            let v1296 = parameters[39];
            let v1318 = staged[10];
            let v1326 = staged[31];
            let v1356 = -1e0f64;
            let v1359 = 2e0f64;
            let v1363 = -5e-1f64;
            let v1365 = -1.5e0f64;
            let v1383 = staged[11];
            let v1385 = staged[12];
            let v1394 = staged[13];
            let v1497 = node_potentials[7];
            let v1499 = 1e0f64;
            let v1503 = parameters[28];
            let v1508 = parameters[24];
            let v1513 = node_potentials[6];
            let v1515 = 1e0f64;
            let v1529 = parameters[55];
            let v1530 = branch_unknown_flows[1];
            let v1532 = 1e0f64;
            let v1536 = staged[32];
            let v1546 = Lanes([0e0f64; 7]);
            let v1547 = 0e0f64;
            let v1552 = staged[33];
            let v1571 = Lanes([0e0f64; 3]);
            let v1572 = Lanes([0e0f64; 3]);
            let v1579 = node_potentials[14];
            let v1582 = 1e0f64;
            let v1585 = parameters[61];
            let v1590 = staged[34];
            let v1595 = parameters[60];
            let v1598 = Lanes([0e0f64; 2]);
            let v1601 = staged[35];
            let v1602 = node_potentials[13];
            let v1604 = 1e0f64;
            let v1608 = parameters[51];
            let v1611 = Lanes([0e0f64; 2]);
            let v1614 = parameters[0];
            let v1615 = staged[36];
            let v1620 = parameters[49];
            let v1623 = Lanes([0e0f64; 2]);
            let v1626 = staged[37];
            let v1627 = branch_unknown_flows[7];
            let v1628 = parameters[46];
            let v1630 = 1e0f64;
            let v1632 = 0e0f64;
            let v1635 = parameters[54];
            let v1636 = branch_unknown_flows[10];
            let v1638 = 1e0f64;
            let v1642 = staged[38];
            let v1643 = branch_unknown_flows[11];
            let v1645 = 1e0f64;
            let v1651 = Lanes([0e0f64; 7]);
            let v1654 = parameters[53];
            let v1655 = branch_unknown_flows[14];
            let v1657 = 1e0f64;
            let v1661 = staged[39];
            let v1662 = branch_unknown_flows[15];
            let v1664 = 1e0f64;
            let v1670 = Lanes([0e0f64; 7]);
            let v1673 = parameters[52];
            let v1674 = branch_unknown_flows[18];
            let v1676 = 1e0f64;
            let v1680 = node_potentials[2];
            let v1683 = 1e0f64;
            let v1692 = staged[40];
            let v1693 = staged[41];
            let v1706 = 0e0f64;
            let v1707 = 0e0f64;
            let v1708 = Lanes([0e0f64; 3]);
            let v1709 = Lanes([0e0f64; 2]);
            let v1735 = 5.5226012e-23f64;
            let v1738 = parameters[88];
            let v1745 = staged[15];
            let v1759 = 3.141592653589793e0f64;
            let v1764 = node_potentials[17];
            let v1767 = 1e0f64;
            let v1772 = node_potentials[18];
            let v1775 = 1e0f64;
            let v1794 = staged[45];
            let v1819 = -1e0f64;
            let v1826 = parameters[67];
            let v1833 = Lanes([0e0f64; 7]);
            let v1851 = 1e0f64;
            let v1983 = node_potentials[16];
            let v1984 = 1e-15f64;
            let v1985 = 1e-12f64;
            let v2 = v0 - v1;
            let v7 = (Lanes([0.0, v3])) - (Lanes([v5, 0.0]));
            let v10 = v8 - v9;
            let v15 = (Lanes([0.0, v11])) - (Lanes([v13, 0.0]));
            let v16 = -v10;
            let v18 = v15 * v17;
            let v19 = v9 - v1;
            let v22 = (Lanes([v13, 0.0])) - (Lanes([0.0, v5]));
            let v24 = v23 - v1;
            let v28 = (Lanes([0.0, v25])) - (Lanes([v5, 0.0]));
            let v30 = v29 - v1;
            let v34 = (Lanes([v31, 0.0])) - (Lanes([0.0, v5]));
            let v49: f64;
            let v50: f64;
            if v35 != 0.0 {
                let v45 = v44 * ((v40 * (if v36 >= v38 { 1.0 } else { 0.0 })) - v42);
                let v47 = v46 + (v36.abs());
                v49 = v47;
                v50 = v45;
            } else {
                v49 = v46;
                v50 = v48;
            }
            let v52 = v49 * v51;
            let v53 = v50 * v51;
            let v55 = v49 - v54;
            let v56 = v55.abs();
            let v60 = v50 * ((v40 * (if v55 >= v38 { 1.0 } else { 0.0 })) - v42);
            let v64 = if (if v56 > v61 { 1.0 } else { 0.0 }) != 0.0 || v63 != 0.0 { 1.0 } else { 0.0 };
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
            let v160: f64;
            let v161: f64;
            let v162: f64;
            let v163: f64;
            let v164: f64;
            let v165: f64;
            let v166: f64;
            let v167: f64;
            let v168: f64;
            let v169: f64;
            let v170: f64;
            let v171: f64;
            let v172: f64;
            let v173: f64;
            if v64 != 0.0 {
                let v65 = v56.abs();
                let v69 = v60 * ((v40 * (if v56 >= v38 { 1.0 } else { 0.0 })) - v42);
                let v76 = v75 * (v73 + (v70 * v65));
                let v77 = (v69 * v70) * v75;
                let v83 = v82 * (v73 + (v78 * v65));
                let v84 = (v69 * v78) * v82;
                let v90 = v89 * (v73 + (v85 * v65));
                let v91 = (v69 * v85) * v89;
                let v97 = v96 * (v73 + (v92 * v65));
                let v98 = (v69 * v92) * v96;
                let v104 = v103 * (v73 + (v99 * v65));
                let v105 = (v69 * v99) * v103;
                let v108 = v69 * v106;
                let v109 = v73 + (v106 * v65);
                let v111 = v110 * v109;
                let v112 = v108 * v110;
                let v115 = v60 * v113;
                let v117 = v116 + (v113 * v56);
                let v120 = v60 * v118;
                let v121 = v73 + (v118 * v56);
                let v123 = v122 * v121;
                let v124 = v120 * v122;
                let v126 = v125 * v121;
                let v127 = v120 * v125;
                let v130 = v60 * v128;
                let v132 = v131 + (v128 * v56);
                let v135 = v60 * v133;
                let v137 = v136 + (v133 * v56);
                let v145 = if (if (if v138 == v73 { 1.0 } else { 0.0 }) != 0.0 || (if v138 == v140 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v143 == v140 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v189: f64;
                let v190: f64;
                let v191: f64;
                let v192: f64;
                if v145 != 0.0 {
                    let v176 = v60 * v56;
                    let v179 = (v176 + v176) * v106;
                    let v180 = v73 + (v106 * (v56 * v56));
                    let v181 = v146 * v180;
                    let v182 = v179 * v146;
                    let v183 = v147 * v180;
                    let v184 = v179 * v147;
                    v189 = v181;
                    v190 = v183;
                    v191 = v182;
                    v192 = v184;
                } else {
                    let v185 = v146 * v109;
                    let v186 = v108 * v146;
                    let v187 = v147 * v109;
                    let v188 = v108 * v147;
                    v189 = v185;
                    v190 = v187;
                    v191 = v186;
                    v192 = v188;
                }
                v148 = v117;
                v149 = v137;
                v150 = v76;
                v151 = v83;
                v152 = v104;
                v153 = v132;
                v154 = v123;
                v155 = v126;
                v156 = v90;
                v157 = v97;
                v158 = v111;
                v159 = v189;
                v160 = v190;
                v161 = v115;
                v162 = v135;
                v163 = v77;
                v164 = v84;
                v165 = v105;
                v166 = v130;
                v167 = v124;
                v168 = v127;
                v169 = v91;
                v170 = v98;
                v171 = v112;
                v172 = v191;
                v173 = v192;
            } else {
                v148 = v116;
                v149 = v136;
                v150 = v75;
                v151 = v82;
                v152 = v103;
                v153 = v131;
                v154 = v122;
                v155 = v125;
                v156 = v89;
                v157 = v96;
                v158 = v110;
                v159 = v146;
                v160 = v147;
                v161 = v48;
                v162 = v48;
                v163 = v48;
                v164 = v48;
                v165 = v48;
                v166 = v48;
                v167 = v48;
                v168 = v48;
                v169 = v48;
                v170 = v48;
                v171 = v48;
                v172 = v48;
                v173 = v48;
            }
            let v199: f64;
            let v200: f64;
            if v174 != 0.0 {
                let v194 = v193 / v52;
                let v197 = ((v53 * v194) * v17) / v52;
                v199 = v194;
                v200 = v197;
            } else {
                v199 = v198;
                v200 = v48;
            }
            let v202 = v201 * v19;
            let v204 = v202.cosh();
            let v209 = v34 * v207;
            let v211 = ((v22 * v201) * (v202.sinh())) * v204;
            let v214 = v213 + (v204 * v204);
            let v216 = v215 / v214;
            let v222 = v221 * (v73 + v216);
            let v224 = v56.abs();
            let v228 = v60 * ((v40 * (if v56 >= v38 { 1.0 } else { 0.0 })) - v42);
            let v232 = v73 + (v229 * v224);
            let v233 = v222 * v232;
            let v234 = (((((v211 + v211) * v216) * v17) / v214) * v221) * v232;
            let v238 = (Lanes([0.0, v234[0], v234[1]])) + (Lanes([((v228 * v229) * v222), 0.0, 0.0]));
            let v244 = v243 * (v73 + (v239 * v224));
            let v245 = (v228 * v239) * v243;
            let v251 = (v248 * v19).tanh();
            let v256 = ((v22 * v248) * (v42 - (v251 * v251))) * v246;
            let v260 = (Lanes([v161, 0.0, 0.0])) + (Lanes([0.0, v256[0], v256[1]]));
            let v264 = (Lanes([v260[0], 0.0, v260[1], v260[2]])) - (Lanes([0.0, v209[0], 0.0, v209[1]]));
            let v265 = v16 - v149;
            let v268 = (Lanes([0.0, v18[0], v18[1]])) - (Lanes([v162, 0.0, 0.0]));
            let v270 = v269 * v265;
            let v275 = ((v268 * v269) * v265) + (v268 * v270);
            let v276 = (((v148 - v246) + (v246 * v251)) - (v207 * v30)) - (v270 * v265);
            let v282 = v73 + (v113 * v224);
            let v283 = v276 * v282;
            let v287 = (((Lanes([v264[0], v264[1], v264[2], v264[3], 0.0])) - (Lanes([v275[0], 0.0, v275[1], 0.0, v275[2]]))) * v282) + (Lanes([((v228 * v113) * v276), 0.0, 0.0, 0.0, 0.0]));
            let v288 = v2 - v283;
            let v291 = (Lanes([0.0, 0.0, 0.0, v7[0], 0.0, v7[1]])) - (Lanes([v287[0], v287[1], v287[2], v287[3], v287[4], 0.0]));
            let v292 = v288 * v288;
            let v293 = v291 * v288;
            let v294 = v293 + v293;
            let v296 = v238 * v288;
            let v301 = v300 * v292;
            let v302 = v294 * v300;
            let v305 = v244 * v288;
            let v314 = ((v233 * v288) + v301) + (v305 * v292);
            let v315 = (((Lanes([v296[0], 0.0, v296[1], v296[2], 0.0, 0.0])) + (v291 * v233)) + v302) + ((((Lanes([(v245 * v288), 0.0, 0.0, 0.0, 0.0, 0.0])) + (v291 * v244)) * v292) + (v294 * v305));
            let v316 = v314.tanh();
            let v319 = v315 * (v42 - (v316 * v316));
            let v320 = v73 + v316;
            let v321 = rspice_limexp(v314);
            let v325 = rspice_limexp((-v314));
            let v332 = (v329 * (v321 - v325)).tanh();
            let v335 = (((v315 * v321) - ((v315 * v17) * v325)) * v329) * (v42 - (v332 * v332));
            let v336 = v73 + v332;
            let v340 = v339 + (v248 * v320);
            let v343 = v22 * v340;
            let v346 = (v340 * v19).tanh();
            let v349 = (((v319 * v248) * v19) + (Lanes([0.0, 0.0, v343[0], v343[1], 0.0, 0.0]))) * (v42 - (v346 * v346));
            let v381: f64;
            let v382: f64;
            let v383: Lanes<6>;
            let v384: Lanes<6>;
            if v350 != 0.0 {
                let v351 = v150 * v320;
                let v356 = v351 * v346;
                let v362 = v22 * v360;
                let v364 = rspice_limexp(v265);
                let v370 = (Lanes([(v164 * v364), 0.0, 0.0])) + ((v268 * v364) * v151);
                let v371 = (v73 + (v360 * v19)) + (v151 * v364);
                let v375 = v356 * v371;
                let v377 = ((Lanes([0.0, v362[0], v362[1], 0.0])) + (Lanes([v370[0], v370[1], 0.0, v370[2]]))) * v356;
                let v379 = (((((Lanes([(v163 * v320), 0.0, 0.0, 0.0, 0.0, 0.0])) + (v319 * v150)) * v346) + (v349 * v351)) * v371) + (Lanes([v377[0], 0.0, v377[1], v377[2], v377[3], 0.0]));
                v381 = v336;
                v382 = v375;
                v383 = v335;
                v384 = v379;
            } else {
                let v501: f64;
                let v502: f64;
                let v503: Lanes<6>;
                let v504: Lanes<6>;
                if v380 != 0.0 {
                    let v386 = v10 - v283;
                    let v388 = (Lanes([0.0, 0.0, v15[0], 0.0, v15[1]])) - v287;
                    let v389 = v386 * v386;
                    let v390 = v388 * v386;
                    let v391 = v390 + v390;
                    let v392 = v389 * v386;
                    let v397 = v238 * v386;
                    let v412 = (((v233 * v386) + (v300 * v389)) + (v244 * v392)).tanh();
                    let v415 = ((((Lanes([v397[0], 0.0, v397[1], v397[2], 0.0])) + (v388 * v233)) + (v391 * v300)) + ((Lanes([(v245 * v392), 0.0, 0.0, 0.0, 0.0])) + (((v391 * v386) + (v388 * v389)) * v244))) * (v42 - (v412 * v412));
                    let v416 = v73 + v412;
                    let v419 = v339 + (v248 * v416);
                    let v423 = v360 + (v420 * v320);
                    let v424 = v150 * v320;
                    let v429 = v73 + v346;
                    let v430 = v424 * v429;
                    let v436 = v22 * v423;
                    let v447 = rspice_limexp((v444 * (v19 - v149)));
                    let v453 = (Lanes([(v164 * v447), 0.0, 0.0])) + (((((Lanes([0.0, v22[0], v22[1]])) - (Lanes([v162, 0.0, 0.0]))) * v444) * v447) * v151);
                    let v454 = (v73 + (v423 * v19)) + (v151 * v447);
                    let v463 = v360 + (v420 * v416);
                    let v466 = v22 * v419;
                    let v469 = (v419 * v19).tanh();
                    let v473 = v150 * v416;
                    let v478 = v73 - v469;
                    let v480 = v473 * v478;
                    let v486 = v22 * v463;
                    let v489 = v73 - (v463 * v19);
                    let v494 = (((((Lanes([(v163 * v416), 0.0, 0.0, 0.0, 0.0])) + (v415 * v150)) * v478) + ((((((v415 * v248) * v19) + (Lanes([0.0, 0.0, v466[0], v466[1], 0.0]))) * (v42 - (v469 * v469))) * v17) * v473)) * v489) + (((((v415 * v420) * v19) + (Lanes([0.0, 0.0, v486[0], v486[1], 0.0]))) * v17) * v480);
                    let v498 = v329 * ((v430 * v454) - (v480 * v489));
                    let v499 = (((((((Lanes([(v163 * v320), 0.0, 0.0, 0.0, 0.0, 0.0])) + (v319 * v150)) * v429) + (v349 * v424)) * v454) + (((((v319 * v420) * v19) + (Lanes([0.0, 0.0, v436[0], v436[1], 0.0, 0.0]))) + (Lanes([v453[0], 0.0, v453[1], v453[2], 0.0, 0.0]))) * v430)) - (Lanes([v494[0], v494[1], v494[2], v494[3], v494[4], 0.0]))) * v329;
                    v501 = v336;
                    v502 = v498;
                    v503 = v335;
                    v504 = v499;
                } else {
                    let v585: f64;
                    let v586: f64;
                    let v587: Lanes<6>;
                    let v588: Lanes<6>;
                    if v500 != 0.0 {
                        let v507 = v244 * v292;
                        let v516 = (v288 + v301) + (v507 * v288);
                        let v518 = v233 * v516;
                        let v519 = v238 * v516;
                        let v522 = (Lanes([v519[0], 0.0, v519[1], v519[2], 0.0, 0.0])) + (((v291 + v302) + ((((Lanes([(v245 * v292), 0.0, 0.0, 0.0, 0.0, 0.0])) + (v294 * v244)) * v288) + (v291 * v507))) * v233);
                        let v523 = rspice_limexp(v518);
                        let v527 = rspice_limexp((-v518));
                        let v533 = (v329 * (v523 - v527)).tanh();
                        let v536 = (((v522 * v523) - ((v522 * v17) * v527)) * v329) * (v42 - (v533 * v533));
                        let v537 = v73 + v533;
                        let v540 = v339 + (v248 * v537);
                        let v543 = v22 * v540;
                        let v546 = (v540 * v19).tanh();
                        let v552 = v360 + (v420 * v537);
                        let v553 = v150 * v537;
                        let v558 = v553 * v546;
                        let v564 = v22 * v552;
                        let v570 = rspice_limexp((v444 * v265));
                        let v576 = (Lanes([(v164 * v570), 0.0, 0.0])) + (((v268 * v444) * v570) * v151);
                        let v577 = (v73 + (v552 * v19)) + (v151 * v570);
                        let v580 = v558 * v577;
                        let v583 = (((((Lanes([(v163 * v537), 0.0, 0.0, 0.0, 0.0, 0.0])) + (v536 * v150)) * v546) + (((((v536 * v248) * v19) + (Lanes([0.0, 0.0, v543[0], v543[1], 0.0, 0.0]))) * (v42 - (v546 * v546))) * v553)) * v577) + (((((v536 * v420) * v19) + (Lanes([0.0, 0.0, v564[0], v564[1], 0.0, 0.0]))) + (Lanes([v576[0], 0.0, v576[1], 0.0, v576[2], 0.0]))) * v558);
                        v585 = v537;
                        v586 = v580;
                        v587 = v536;
                        v588 = v583;
                    } else {
                        let v757: f64;
                        let v758: f64;
                        let v759: Lanes<6>;
                        let v760: Lanes<6>;
                        if v584 != 0.0 {
                            let v591 = v244 * v292;
                            let v600 = (v288 + v301) + (v591 * v288);
                            let v602 = v233 * v600;
                            let v603 = v238 * v600;
                            let v606 = (Lanes([v603[0], 0.0, v603[1], v603[2], 0.0, 0.0])) + (((v291 + v302) + ((((Lanes([(v245 * v292), 0.0, 0.0, 0.0, 0.0, 0.0])) + (v294 * v244)) * v288) + (v291 * v591))) * v233);
                            let v607 = v10 - v283;
                            let v609 = (Lanes([0.0, 0.0, v15[0], 0.0, v15[1]])) - v287;
                            let v610 = v607 * v607;
                            let v611 = v609 * v607;
                            let v612 = v611 + v611;
                            let v617 = v244 * v607;
                            let v626 = (v607 + (v300 * v610)) + (v617 * v610);
                            let v628 = v233 * v626;
                            let v629 = v238 * v626;
                            let v632 = (Lanes([v629[0], 0.0, v629[1], v629[2], 0.0])) + (((v609 + (v612 * v300)) + ((((Lanes([(v245 * v607), 0.0, 0.0, 0.0, 0.0])) + (v609 * v244)) * v610) + (v612 * v617))) * v233);
                            let v633 = rspice_limexp(v602);
                            let v637 = rspice_limexp((-v602));
                            let v643 = (v329 * (v633 - v637)).tanh();
                            let v646 = (((v606 * v633) - ((v606 * v17) * v637)) * v329) * (v42 - (v643 * v643));
                            let v647 = v73 + v643;
                            let v648 = rspice_limexp(v628);
                            let v652 = rspice_limexp((-v628));
                            let v658 = (v329 * (v648 - v652)).tanh();
                            let v661 = (((v632 * v648) - ((v632 * v17) * v652)) * v329) * (v42 - (v658 * v658));
                            let v662 = v73 + v658;
                            let v665 = v339 + (v248 * v647);
                            let v668 = v339 + (v248 * v662);
                            let v671 = v22 * v665;
                            let v674 = (v665 * v19).tanh();
                            let v680 = v22 * v668;
                            let v683 = (v668 * v19).tanh();
                            let v689 = v360 + (v420 * v662);
                            let v692 = v360 + (v420 * v647);
                            let v693 = v150 * v647;
                            let v698 = v73 + v674;
                            let v699 = v693 * v698;
                            let v705 = v22 * v692;
                            let v715 = rspice_limexp((v444 * (v19 - v149)));
                            let v721 = (Lanes([(v164 * v715), 0.0, 0.0])) + (((((Lanes([0.0, v22[0], v22[1]])) - (Lanes([v162, 0.0, 0.0]))) * v444) * v715) * v151);
                            let v722 = (v73 + (v692 * v19)) + (v151 * v715);
                            let v729 = v150 * v662;
                            let v734 = v73 - v683;
                            let v736 = v729 * v734;
                            let v742 = v22 * v689;
                            let v745 = v73 - (v689 * v19);
                            let v750 = (((((Lanes([(v163 * v662), 0.0, 0.0, 0.0, 0.0])) + (v661 * v150)) * v734) + ((((((v661 * v248) * v19) + (Lanes([0.0, 0.0, v680[0], v680[1], 0.0]))) * (v42 - (v683 * v683))) * v17) * v729)) * v745) + (((((v661 * v420) * v19) + (Lanes([0.0, 0.0, v742[0], v742[1], 0.0]))) * v17) * v736);
                            let v754 = v329 * ((v699 * v722) - (v736 * v745));
                            let v755 = (((((((Lanes([(v163 * v647), 0.0, 0.0, 0.0, 0.0, 0.0])) + (v646 * v150)) * v698) + (((((v646 * v248) * v19) + (Lanes([0.0, 0.0, v671[0], v671[1], 0.0, 0.0]))) * (v42 - (v674 * v674))) * v693)) * v722) + (((((v646 * v420) * v19) + (Lanes([0.0, 0.0, v705[0], v705[1], 0.0, 0.0]))) + (Lanes([v721[0], 0.0, v721[1], v721[2], 0.0, 0.0]))) * v699)) - (Lanes([v750[0], v750[1], v750[2], v750[3], v750[4], 0.0]))) * v329;
                            v757 = v647;
                            v758 = v754;
                            v759 = v646;
                            v760 = v755;
                        } else {
                            let v832: f64;
                            let v833: Lanes<6>;
                            if v756 != 0.0 {
                                let v763 = v360 + (v420 * v320);
                                let v765 = v335 * v248;
                                let v766 = v339 + (v248 * v336);
                                let v769 = v22 * v766;
                                let v772 = (v766 * v19).tanh();
                                let v778 = v34 * v766;
                                let v781 = (v766 * v30).tanh();
                                let v785 = v150 * v320;
                                let v793 = v772 + (v790 * v781);
                                let v795 = v785 * v793;
                                let v800 = v34 * v790;
                                let v801 = v19 + (v790 * v30);
                                let v807 = ((Lanes([0.0, v22[0], v22[1]])) + (Lanes([v800[0], 0.0, v800[1]]))) * v763;
                                let v817 = rspice_limexp((v444 * (v19 - v149)));
                                let v823 = (Lanes([(v164 * v817), 0.0, 0.0])) + (((((Lanes([0.0, v22[0], v22[1]])) - (Lanes([v162, 0.0, 0.0]))) * v444) * v817) * v151);
                                let v824 = (v73 + (v763 * v801)) + (v151 * v817);
                                let v827 = v795 * v824;
                                let v830 = (((((Lanes([(v163 * v320), 0.0, 0.0, 0.0, 0.0, 0.0])) + (v319 * v150)) * v793) + (((((v765 * v19) + (Lanes([0.0, 0.0, v769[0], v769[1], 0.0, 0.0]))) * (v42 - (v772 * v772))) + ((((v765 * v30) + (Lanes([0.0, v778[0], 0.0, v778[1], 0.0, 0.0]))) * (v42 - (v781 * v781))) * v790)) * v785)) * v824) + (((((v319 * v420) * v801) + (Lanes([0.0, v807[0], v807[1], v807[2], 0.0, 0.0]))) + (Lanes([v823[0], 0.0, v823[1], v823[2], 0.0, 0.0]))) * v795);
                                v832 = v827;
                                v833 = v830;
                            } else {
                                v832 = v61;
                                v833 = v831;
                            }
                            v757 = v336;
                            v758 = v832;
                            v759 = v335;
                            v760 = v833;
                        }
                        v585 = v757;
                        v586 = v758;
                        v587 = v759;
                        v588 = v760;
                    }
                    v501 = v585;
                    v502 = v586;
                    v503 = v587;
                    v504 = v588;
                }
                v381 = v501;
                v382 = v502;
                v383 = v503;
                v384 = v504;
            }
            let v860: f64;
            let v861: f64;
            let v862: f64;
            let v863: Lanes<6>;
            let v864: Lanes<6>;
            let v865: Lanes<6>;
            if v385 != 0.0 {
                let v834 = v73 + v320;
                let v835 = v152 / v834;
                let v839 = ((Lanes([v165, 0.0, 0.0, 0.0, 0.0, 0.0])) - (v319 * v835)) / v834;
                let v841 = v840 + v835;
                let v843 = v842 * v320;
                let v844 = v319 * v842;
                let v846 = v845 + v843;
                let v848 = v847 + v843;
                v860 = v848;
                v861 = v846;
                v862 = v841;
                v863 = v844;
                v864 = v844;
                v865 = v839;
            } else {
                let v849 = v73 + v381;
                let v850 = v152 / v849;
                let v854 = ((Lanes([v165, 0.0, 0.0, 0.0, 0.0, 0.0])) - (v383 * v850)) / v849;
                let v855 = v840 + v850;
                let v856 = v842 * v381;
                let v857 = v383 * v842;
                let v858 = v845 + v856;
                let v859 = v847 + v856;
                v860 = v859;
                v861 = v858;
                v862 = v855;
                v863 = v857;
                v864 = v857;
                v865 = v854;
            }
            let v868 = v228 * v866;
            let v869 = v73 + (v866 * v224);
            let v870 = v860 * v869;
            let v874 = (v863 * v869) + (Lanes([(v868 * v860), 0.0, 0.0, 0.0, 0.0, 0.0]));
            let v875 = v861 * v869;
            let v879 = (v864 * v869) + (Lanes([(v868 * v861), 0.0, 0.0, 0.0, 0.0, 0.0]));
            let v885 = v884 * (v73 + (v880 * v224));
            let v886 = (v228 * v880) * v884;
            let v924: f64;
            let v925: f64;
            let v926: f64;
            let v927: f64;
            let v928: f64;
            let v929: Lanes<2>;
            let v930: Lanes<3>;
            let v931: f64;
            let v932: Lanes<2>;
            let v933: Lanes<3>;
            if v887 != 0.0 {
                let v891 = (v888 * v153).tanh();
                let v899 = rspice_limexp((v199 * v891));
                let v900 = ((v200 * v891) + (((v166 * v888) * (v42 - (v891 * v891))) * v199)) * v899;
                let v901 = v24 - v153;
                let v904 = (Lanes([0.0, v28[0], v28[1]])) - (Lanes([v166, 0.0, 0.0]));
                let v906 = v28 * v17;
                let v908 = (-v24) - v907;
                let v909 = v10 - v153;
                let v912 = (Lanes([0.0, v15[0], v15[1]])) - (Lanes([v166, 0.0, 0.0]));
                let v914 = v16 - v913;
                v924 = v908;
                v925 = v901;
                v926 = v899;
                v927 = v914;
                v928 = v909;
                v929 = v906;
                v930 = v904;
                v931 = v900;
                v932 = v18;
                v933 = v912;
            } else {
                let v915 = -v199;
                let v921 = rspice_limexp((v915 * v153));
                let v922 = (((v200 * v17) * v153) + (v166 * v915)) * v921;
                let v1065: f64;
                let v1066: f64;
                let v1067: Lanes<3>;
                let v1068: Lanes<3>;
                if v923 != 0.0 {
                    let v1045 = (v24 - v153).tanh();
                    let v1048 = ((Lanes([0.0, v28[0], v28[1]])) - (Lanes([v166, 0.0, 0.0]))) * (v42 - (v1045 * v1045));
                    let v1053 = (v10 - v153).tanh();
                    let v1056 = ((Lanes([0.0, v15[0], v15[1]])) - (Lanes([v166, 0.0, 0.0]))) * (v42 - (v1053 * v1053));
                    v1065 = v1045;
                    v1066 = v1053;
                    v1067 = v1048;
                    v1068 = v1056;
                } else {
                    let v1057 = v24 - v153;
                    let v1060 = (Lanes([0.0, v28[0], v28[1]])) - (Lanes([v166, 0.0, 0.0]));
                    let v1061 = v10 - v153;
                    let v1064 = (Lanes([0.0, v15[0], v15[1]])) - (Lanes([v166, 0.0, 0.0]));
                    v1065 = v1057;
                    v1066 = v1061;
                    v1067 = v1060;
                    v1068 = v1064;
                }
                let v1070 = v28 * v17;
                let v1071 = (-v24) - v907;
                let v1072 = v16 - v913;
                v924 = v1071;
                v925 = v1065;
                v926 = v921;
                v927 = v1072;
                v928 = v1066;
                v929 = v1070;
                v930 = v1067;
                v931 = v922;
                v932 = v18;
                v933 = v1068;
            }
            let v937 = rspice_limexp((v934 * v924));
            let v946 = rspice_limexp((v199 * v925));
            let v950 = ((v929 * v934) * v937) * v948;
            let v958 = v957 * ((v946 - (v948 * (v937 - v939))) - v926);
            let v959 = (((((Lanes([(v200 * v925), 0.0, 0.0])) + (v930 * v199)) * v946) - (Lanes([0.0, v950[0], v950[1]]))) - (Lanes([v931, 0.0, 0.0]))) * v957;
            let v962 = rspice_limexp((v934 * v927));
            let v971 = rspice_limexp((v199 * v928));
            let v974 = ((v932 * v934) * v962) * v948;
            let v981 = v957 * ((v971 - (v948 * (v962 - v964))) - v926);
            let v982 = (((((Lanes([(v200 * v928), 0.0, 0.0])) + (v933 * v199)) * v971) - (Lanes([0.0, v974[0], v974[1]]))) - (Lanes([v931, 0.0, 0.0]))) * v957;
            let v985 = v28 * v983;
            let v989 = (Lanes([v167, 0.0, 0.0])) + (Lanes([0.0, v985[0], v985[1]]));
            let v991 = v990 * v19;
            let v992 = v22 * v990;
            let v993 = (v154 + (v983 * v24)) + v991;
            let v996 = (Lanes([v989[0], 0.0, v989[1], v989[2]])) + (Lanes([0.0, v992[0], v992[1], 0.0]));
            let v997 = v993.tanh();
            let v1000 = v996 * (v42 - (v997 * v997));
            let v1001 = v73 + v997;
            let v1007 = (v1005 + (v1002 * v19)).tanh();
            let v1010 = (v22 * v1002) * (v42 - (v1007 * v1007));
            let v1011 = v73 + v1007;
            let v1018 = (v1015 - (v1012 * v19)).tanh();
            let v1021 = ((v22 * v1012) * v17) * (v42 - (v1018 * v1018));
            let v1023 = (v73 + v1018) - v990;
            let v1026 = v15 * v1024;
            let v1030 = (Lanes([v168, 0.0, 0.0])) + (Lanes([0.0, v1026[0], v1026[1]]));
            let v1031 = (v155 + (v1024 * v10)) - v991;
            let v1034 = (Lanes([v1030[0], v1030[1], 0.0, v1030[2]])) - (Lanes([0.0, v992[0], v992[1], 0.0]));
            let v1035 = v1031.tanh();
            let v1038 = v1034 * (v42 - (v1035 * v1035));
            let v1039 = v73 + v1035;
            let v1078: f64;
            let v1079: f64;
            let v1080: f64;
            let v1081: f64;
            let v1082: Lanes<4>;
            let v1083: Lanes<4>;
            let v1084: Lanes<4>;
            let v1085: Lanes<4>;
            if v1040 != 0.0 {
                v1078 = v61;
                v1079 = v61;
                v1080 = v1073;
                v1081 = v1074;
                v1082 = v1075;
                v1083 = v1076;
                v1084 = v1075;
                v1085 = v1076;
            } else {
                let v1129: f64;
                let v1130: f64;
                let v1131: f64;
                let v1132: f64;
                let v1133: Lanes<4>;
                let v1134: Lanes<4>;
                let v1135: Lanes<4>;
                let v1136: Lanes<4>;
                if v1077 != 0.0 {
                    let v1104 = v156 * v1001;
                    let v1111 = v1010 * v1104;
                    let v1113 = (((Lanes([(v169 * v1001), 0.0, 0.0, 0.0])) + (v1000 * v156)) * v1011) + (Lanes([0.0, v1111[0], v1111[1], 0.0]));
                    let v1114 = v1074 + (v1104 * v1011);
                    let v1116 = v1021 * v1039;
                    let v1121 = (v1023 * v1039) + v1120;
                    let v1126 = (Lanes([(v170 * v1121), 0.0, 0.0, 0.0])) + (((Lanes([0.0, v1116[0], v1116[1], 0.0])) + (v1038 * v1023)) * v157);
                    let v1127 = v1073 + (v157 * v1121);
                    v1129 = v61;
                    v1130 = v61;
                    v1131 = v1127;
                    v1132 = v1114;
                    v1133 = v1075;
                    v1134 = v1076;
                    v1135 = v1126;
                    v1136 = v1113;
                } else {
                    let v1231: f64;
                    let v1232: f64;
                    let v1233: f64;
                    let v1234: f64;
                    let v1235: Lanes<4>;
                    let v1236: Lanes<4>;
                    let v1237: Lanes<4>;
                    let v1238: Lanes<4>;
                    if v1128 != 0.0 {
                        let v1137 = v1011 - v990;
                        let v1138 = v154 + v991;
                        let v1140 = Lanes([0.0, v992[0], v992[1]]);
                        let v1141 = (Lanes([v167, 0.0, 0.0])) + v1140;
                        let v1142 = v1138.cosh();
                        let v1148 = v993.cosh();
                        let v1155 = v1141 + ((v1141 * (v1138.sinh())) * (v42 / v1142));
                        let v1158 = (v993 + (v1148.ln())) - (v1138 + (v1142.ln()));
                        let v1163 = v1010 * v1158;
                        let v1170 = v28 * v1168;
                        let v1171 = ((v1158 * v1137) / v983) + (v1168 * v24);
                        let v1180 = v28 * v1074;
                        let v1181 = (v156 * v1171) + (v1074 * v24);
                        let v1183 = ((Lanes([(v169 * v1171), 0.0, 0.0, 0.0])) + (((((((v996 + ((v996 * (v993.sinh())) * (v42 / v1148))) - (Lanes([v1155[0], v1155[1], v1155[2], 0.0]))) * v1137) + (Lanes([0.0, v1163[0], v1163[1], 0.0]))) / v983) + (Lanes([0.0, 0.0, v1170[0], v1170[1]]))) * v156)) + (Lanes([0.0, 0.0, v1180[0], v1180[1]]));
                        let v1184 = v155 - v991;
                        let v1186 = (Lanes([v168, 0.0, 0.0])) - v1140;
                        let v1187 = v1184.cosh();
                        let v1193 = v1031.cosh();
                        let v1200 = v1186 + ((v1186 * (v1184.sinh())) * (v42 / v1187));
                        let v1203 = (v1031 + (v1193.ln())) - (v1184 + (v1187.ln()));
                        let v1208 = v1021 * v1203;
                        let v1214 = v15 * v1168;
                        let v1215 = ((v1203 * v1023) / v1024) + (v1168 * v10);
                        let v1224 = v15 * v1073;
                        let v1225 = (v157 * v1215) + (v1073 * v10);
                        let v1227 = ((Lanes([(v170 * v1215), 0.0, 0.0, 0.0])) + (((((((v1034 + ((v1034 * (v1031.sinh())) * (v42 / v1193))) - (Lanes([v1200[0], v1200[1], v1200[2], 0.0]))) * v1023) + (Lanes([0.0, v1208[0], v1208[1], 0.0]))) / v1024) + (Lanes([0.0, v1214[0], 0.0, v1214[1]]))) * v157)) + (Lanes([0.0, v1224[0], 0.0, v1224[1]]));
                        let v1228 = v1183[3];
                        let v1229 = v1227[3];
                        v1231 = v1225;
                        v1232 = v1181;
                        v1233 = v1229;
                        v1234 = v1228;
                        v1235 = v1227;
                        v1236 = v1183;
                        v1237 = v1075;
                        v1238 = v1076;
                    } else {
                        let v1327: f64;
                        let v1328: f64;
                        let v1329: f64;
                        let v1330: f64;
                        let v1331: Lanes<4>;
                        let v1332: Lanes<4>;
                        let v1333: Lanes<4>;
                        let v1334: Lanes<4>;
                        if v1230 != 0.0 {
                            let v1242 = (v24 / v1239) - v73;
                            let v1243 = v1242 * v1242;
                            let v1244 = (v28 / v1239) * v1242;
                            let v1245 = v1244 + v1244;
                            let v1247 = v1246 + v1243;
                            let v1249 = v1247.powf(v1248);
                            let v1257 = v1246 + (v1254 * v1243);
                            let v1267 = ((Lanes([0.0, v28[0], v28[1]])) + (Lanes([v992[0], v992[1], 0.0]))) * v983;
                            let v1272 = (v154 + (v983 * (v24 + v991))).tanh();
                            let v1278 = v1277 + v1018;
                            let v1280 = v22 * v1277;
                            let v1286 = ((Lanes([v15[0], 0.0, v15[1]])) + (Lanes([v1280[0], v1280[1], 0.0]))) * v1024;
                            let v1291 = (v155 + (v1024 * (v10 + (v19 * v1277)))).tanh();
                            let v1295 = v73 + v1291;
                            let v1298 = (((v1245 * (v1248 * (v1247.powf(v1250)))) * v1257) + ((v1245 * v1254) * v1249)) * v1296;
                            let v1299 = (v73 + v1272) + (v1296 * (v1249 * v1257));
                            let v1302 = v156 * v1299;
                            let v1309 = v1010 * v1302;
                            let v1311 = (((Lanes([(v169 * v1299), 0.0, 0.0, 0.0])) + (((((Lanes([v167, 0.0, 0.0, 0.0])) + (Lanes([0.0, v1267[0], v1267[1], v1267[2]]))) * (v42 - (v1272 * v1272))) + (Lanes([0.0, 0.0, v1298[0], v1298[1]]))) * v156)) * v1011) + (Lanes([0.0, v1309[0], v1309[1], 0.0]));
                            let v1312 = (v1302 * v1011) + v1074;
                            let v1314 = v1021 * v1295;
                            let v1319 = (v1278 * v1295) + v1318;
                            let v1324 = (Lanes([(v170 * v1319), 0.0, 0.0, 0.0])) + (((Lanes([0.0, v1314[0], v1314[1], 0.0])) + ((((Lanes([v168, 0.0, 0.0, 0.0])) + (Lanes([0.0, v1286[0], v1286[1], v1286[2]]))) * (v42 - (v1291 * v1291))) * v1278)) * v157);
                            let v1325 = (v157 * v1319) + v1073;
                            v1327 = v61;
                            v1328 = v61;
                            v1329 = v1325;
                            v1330 = v1312;
                            v1331 = v1075;
                            v1332 = v1076;
                            v1333 = v1324;
                            v1334 = v1311;
                        } else {
                            let v1457: f64;
                            let v1458: f64;
                            let v1459: f64;
                            let v1460: f64;
                            let v1461: Lanes<4>;
                            let v1462: Lanes<4>;
                            if v1326 != 0.0 {
                                let v1335 = v154 + v991;
                                let v1337 = Lanes([0.0, v992[0], v992[1]]);
                                let v1338 = (Lanes([v167, 0.0, 0.0])) + v1337;
                                let v1339 = v1335.cosh();
                                let v1345 = v993.cosh();
                                let v1352 = v1296 * (v1239 + v24);
                                let v1357 = v1356 + (v24 / v1239);
                                let v1362 = v1246 + (v1357 * v1357);
                                let v1364 = v1362.powf(v1363);
                                let v1372 = ((v28 * v1296) * v1364) + ((((v28 / v1239) * (v1359 * v1357)) * (v1363 * (v1362.powf(v1365)))) * v1352);
                                let v1374 = v1338 + ((v1338 * (v1335.sinh())) * (v42 / v1339));
                                let v1384 = (((v993 + (v1345.ln())) - (v1335 + (v1339.ln()))) + (v1352 * v1364)) - v1383;
                                let v1386 = v1385 + v1007;
                                let v1389 = v1010 * v1384;
                                let v1396 = v28 * v1394;
                                let v1397 = ((v1384 * v1386) / v983) + (v1394 * v24);
                                let v1406 = v28 * v1074;
                                let v1407 = (v156 * v1397) + (v1074 * v24);
                                let v1409 = ((Lanes([(v169 * v1397), 0.0, 0.0, 0.0])) + ((((((((v996 + ((v996 * (v993.sinh())) * (v42 / v1345))) - (Lanes([v1374[0], v1374[1], v1374[2], 0.0]))) + (Lanes([0.0, 0.0, v1372[0], v1372[1]]))) * v1386) + (Lanes([0.0, v1389[0], v1389[1], 0.0]))) / v983) + (Lanes([0.0, 0.0, v1396[0], v1396[1]]))) * v156)) + (Lanes([0.0, 0.0, v1406[0], v1406[1]]));
                                let v1410 = v155 - v991;
                                let v1412 = (Lanes([v168, 0.0, 0.0])) - v1337;
                                let v1413 = v1410.cosh();
                                let v1419 = v1031.cosh();
                                let v1426 = v1412 + ((v1412 * (v1410.sinh())) * (v42 / v1413));
                                let v1429 = (v1031 + (v1419.ln())) - (v1410 + (v1413.ln()));
                                let v1432 = v1385 + v1018;
                                let v1435 = v1021 * v1429;
                                let v1441 = v15 * v1394;
                                let v1442 = ((v1429 * v1432) / v1024) + (v1394 * v10);
                                let v1451 = v15 * v1073;
                                let v1452 = (v157 * v1442) + (v1073 * v10);
                                let v1454 = ((Lanes([(v170 * v1442), 0.0, 0.0, 0.0])) + (((((((v1034 + ((v1034 * (v1031.sinh())) * (v42 / v1419))) - (Lanes([v1426[0], v1426[1], v1426[2], 0.0]))) * v1432) + (Lanes([0.0, v1435[0], v1435[1], 0.0]))) / v1024) + (Lanes([0.0, v1441[0], 0.0, v1441[1]]))) * v157)) + (Lanes([0.0, v1451[0], 0.0, v1451[1]]));
                                let v1455 = v1409[3];
                                let v1456 = v1454[3];
                                v1457 = v1452;
                                v1458 = v1407;
                                v1459 = v1456;
                                v1460 = v1455;
                                v1461 = v1454;
                                v1462 = v1409;
                            } else {
                                v1457 = v61;
                                v1458 = v61;
                                v1459 = v61;
                                v1460 = v61;
                                v1461 = v1075;
                                v1462 = v1076;
                            }
                            v1327 = v1457;
                            v1328 = v1458;
                            v1329 = v1459;
                            v1330 = v1460;
                            v1331 = v1461;
                            v1332 = v1462;
                            v1333 = v1075;
                            v1334 = v1076;
                        }
                        v1231 = v1327;
                        v1232 = v1328;
                        v1233 = v1329;
                        v1234 = v1330;
                        v1235 = v1331;
                        v1236 = v1332;
                        v1237 = v1333;
                        v1238 = v1334;
                    }
                    v1129 = v1231;
                    v1130 = v1232;
                    v1131 = v1233;
                    v1132 = v1234;
                    v1133 = v1235;
                    v1134 = v1236;
                    v1135 = v1237;
                    v1136 = v1238;
                }
                v1078 = v1129;
                v1079 = v1130;
                v1080 = v1131;
                v1081 = v1132;
                v1082 = v1133;
                v1083 = v1134;
                v1084 = v1135;
                v1085 = v1136;
            }
            let v1086 = -v382;
            let v1087 = v384 * v17;
            let v1090 = v1088 * v1089;
            let v1092 = v1091 * v1088;
            let v1093 = ddt(4327, v1090);
            let v1095 = v1092 * v1094;
            let v1098 = v1096 * v1097;
            let v1100 = v1099 * v1096;
            let v1101 = ddt(4334, v1098);
            let v1102 = v1100 * v1094;
            let v1481: f64;
            let v1482: f64;
            let v1483: f64;
            let v1484: f64;
            let v1485: f64;
            let v1486: f64;
            let v1487: f64;
            let v1488: f64;
            let v1489: Lanes<4>;
            let v1490: Lanes<4>;
            let v1491: Lanes<4>;
            let v1492: Lanes<4>;
            let v1493: Lanes<4>;
            let v1494: Lanes<4>;
            let v1495: Lanes<4>;
            let v1496: Lanes<4>;
            if v1103 != 0.0 {
                let v1463 = ddt(4346, v1078);
                let v1464 = v1082 * v1094;
                let v1465 = ddt(4348, v1079);
                let v1466 = v1083 * v1094;
                v1481 = v1463;
                v1482 = v1465;
                v1483 = v61;
                v1484 = v61;
                v1485 = v1078;
                v1486 = v1079;
                v1487 = v61;
                v1488 = v61;
                v1489 = v1464;
                v1490 = v1466;
                v1491 = v1075;
                v1492 = v1076;
                v1493 = v1082;
                v1494 = v1083;
                v1495 = v1075;
                v1496 = v1076;
            } else {
                let v1467 = v1080 * v10;
                let v1469 = v15 * v1080;
                let v1471 = (v1084 * v10) + (Lanes([0.0, v1469[0], 0.0, v1469[1]]));
                let v1472 = ddt(4352, v1467);
                let v1473 = v1471 * v1094;
                let v1474 = v1081 * v24;
                let v1476 = v28 * v1081;
                let v1478 = (v1085 * v24) + (Lanes([0.0, 0.0, v1476[0], v1476[1]]));
                let v1479 = ddt(4356, v1474);
                let v1480 = v1478 * v1094;
                v1481 = v61;
                v1482 = v61;
                v1483 = v1472;
                v1484 = v1479;
                v1485 = v61;
                v1486 = v61;
                v1487 = v1467;
                v1488 = v1474;
                v1489 = v1075;
                v1490 = v1076;
                v1491 = v1473;
                v1492 = v1480;
                v1493 = v1075;
                v1494 = v1076;
                v1495 = v1471;
                v1496 = v1478;
            }
            let v1504 = v1503 * (v1497 - v9);
            let v1505 = ((Lanes([0.0, v1499])) - (Lanes([v13, 0.0]))) * v1503;
            let v1506 = ddt(4360, v1504);
            let v1507 = v1505 * v1094;
            let v1509 = v1508 * v19;
            let v1510 = v22 * v1508;
            let v1511 = ddt(4364, v1509);
            let v1512 = v1510 * v1094;
            let v1514 = v1513 - v29;
            let v1518 = (Lanes([0.0, v1515])) - (Lanes([v31, 0.0]));
            let v1519 = v158 * v1514;
            let v1521 = v1518 * v158;
            let v1524 = (Lanes([(v171 * v1514), 0.0, 0.0])) + (Lanes([0.0, v1521[0], v1521[1]]));
            let v1525 = ddt(4368, v1519);
            let v1526 = v1524 * v1094;
            let v1527 = v213 * v1514;
            let v1528 = v1518 * v213;
            let v1531 = v1529 * v1530;
            let v1533 = v1532 * v1529;
            let v1534 = ddt(4375, v1531);
            let v1535 = v1533 * v1094;
            let v1548: f64;
            let v1549: f64;
            let v1550: Lanes<7>;
            let v1551: f64;
            if v1536 != 0.0 {
                let v1539 = v865 * v1530;
                let v1543 = (v1530 * v862) + v1534;
                let v1545 = ((Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (v1532 * v862)])) + (Lanes([v1539[0], v1539[1], v1539[2], v1539[3], v1539[4], v1539[5], 0.0]))) + (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, v1535]));
                v1548 = v1543;
                v1549 = v1531;
                v1550 = v1545;
                v1551 = v1533;
            } else {
                v1548 = v61;
                v1549 = v61;
                v1550 = v1546;
                v1551 = v1547;
            }
            let v1573: f64;
            let v1574: f64;
            let v1575: f64;
            let v1576: Lanes<3>;
            let v1577: Lanes<3>;
            let v1578: Lanes<3>;
            if v1552 != 0.0 {
                let v1556 = (Lanes([v25, 0.0])) - (Lanes([0.0, v3]));
                let v1557 = (v23 - v0) / v159;
                let v1562 = ((Lanes([0.0, v1556[0], v1556[1]])) - (Lanes([(v172 * v1557), 0.0, 0.0]))) / v159;
                let v1563 = v160 * v2;
                let v1565 = v7 * v160;
                let v1568 = (Lanes([(v173 * v2), 0.0, 0.0])) + (Lanes([0.0, v1565[0], v1565[1]]));
                let v1569 = ddt(4398, v1563);
                let v1570 = v1568 * v1094;
                v1573 = v1557;
                v1574 = v1569;
                v1575 = v1563;
                v1576 = v1562;
                v1577 = v1570;
                v1578 = v1568;
            } else {
                v1573 = v61;
                v1574 = v61;
                v1575 = v61;
                v1576 = v1571;
                v1577 = v1572;
                v1578 = v1572;
            }
            let v1586 = v1585 * (v23 - v1579);
            let v1587 = ((Lanes([v25, 0.0])) - (Lanes([0.0, v1582]))) * v1585;
            let v1588 = ddt(4403, v1586);
            let v1589 = v1587 * v1094;
            let v1599: f64;
            let v1600: Lanes<2>;
            if v1590 != 0.0 {
                let v1596 = (v1579 - v1) / v1595;
                let v1597 = ((Lanes([0.0, v1582])) - (Lanes([v5, 0.0]))) / v1595;
                v1599 = v1596;
                v1600 = v1597;
            } else {
                v1599 = v61;
                v1600 = v1598;
            }
            let v1612: f64;
            let v1613: Lanes<2>;
            if v1601 != 0.0 {
                let v1609 = (v1602 - v8) / v1608;
                let v1610 = ((Lanes([0.0, v1604])) - (Lanes([v11, 0.0]))) / v1608;
                v1612 = v1609;
                v1613 = v1610;
            } else {
                v1612 = v61;
                v1613 = v1611;
            }
            let v1624: f64;
            let v1625: Lanes<2>;
            if v1615 != 0.0 {
                let v1621 = (v1602 - v23) / v1620;
                let v1622 = ((Lanes([0.0, v1604])) - (Lanes([v25, 0.0]))) / v1620;
                v1624 = v1621;
                v1625 = v1622;
            } else {
                v1624 = v61;
                v1625 = v1623;
            }
            let v1633: f64;
            let v1634: f64;
            if v1626 != 0.0 {
                let v1629 = v1627 * v1628;
                let v1631 = v1630 * v1628;
                v1633 = v1629;
                v1634 = v1631;
            } else {
                v1633 = v61;
                v1634 = v1632;
            }
            let v1637 = v1635 * v1636;
            let v1639 = v1638 * v1635;
            let v1640 = ddt(4455, v1637);
            let v1641 = v1639 * v1094;
            let v1652: f64;
            let v1653: Lanes<7>;
            if v1642 != 0.0 {
                let v1644 = v1643 * v870;
                let v1647 = v874 * v1643;
                let v1650 = (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (v1645 * v870)])) + (Lanes([v1647[0], v1647[1], v1647[2], v1647[3], v1647[4], v1647[5], 0.0]));
                v1652 = v1644;
                v1653 = v1650;
            } else {
                v1652 = v61;
                v1653 = v1651;
            }
            let v1656 = v1654 * v1655;
            let v1658 = v1657 * v1654;
            let v1659 = ddt(4476, v1656);
            let v1660 = v1658 * v1094;
            let v1671: f64;
            let v1672: Lanes<7>;
            if v1661 != 0.0 {
                let v1663 = v1662 * v875;
                let v1666 = v879 * v1662;
                let v1669 = (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (v1664 * v875)])) + (Lanes([v1666[0], v1666[1], v1666[2], v1666[3], v1666[4], v1666[5], 0.0]));
                v1671 = v1663;
                v1672 = v1669;
            } else {
                v1671 = v61;
                v1672 = v1670;
            }
            let v1675 = v1673 * v1674;
            let v1677 = v1676 * v1673;
            let v1678 = ddt(4501, v1675);
            let v1679 = v1677 * v1094;
            let v1686 = (v0 - v1680) * v213;
            let v1687 = ((Lanes([0.0, v3])) - (Lanes([v1683, 0.0]))) * v213;
            let v1688 = v384[5];
            let v1691 = v1688 / (v73 + (v1688 * v847));
            let v1694: f64;
            let v1695: f64;
            let v1696: f64;
            let v1697: f64;
            let v1698: f64;
            let v1699: f64;
            let v1700: f64;
            let v1701: f64;
            let v1702: f64;
            let v1703: Lanes<3>;
            let v1704: Lanes<2>;
            let v1705: Lanes<2>;
            if v1692 != 0.0 {
                v1694 = v61;
                v1695 = v61;
                v1696 = v61;
                v1697 = v61;
                v1698 = v61;
                v1699 = v61;
                v1700 = v1706;
                v1701 = v1707;
                v1702 = v1706;
                v1703 = v1708;
                v1704 = v1709;
                v1705 = v1709;
            } else {
                let v1710: f64;
                let v1711: f64;
                let v1712: f64;
                let v1713: f64;
                let v1714: f64;
                let v1715: f64;
                let v1716: f64;
                let v1717: f64;
                let v1718: f64;
                let v1719: Lanes<3>;
                let v1720: Lanes<2>;
                let v1721: Lanes<2>;
                if v1693 != 0.0 {
                    let v1723: f64;
                    let v1724: f64;
                    let v1725: f64;
                    let v1726: f64;
                    let v1727: f64;
                    let v1728: f64;
                    let v1729: f64;
                    let v1730: f64;
                    let v1731: f64;
                    let v1732: Lanes<3>;
                    let v1733: Lanes<2>;
                    let v1734: Lanes<2>;
                    if v1614 != 0.0 {
                        let v1722 = if v1691 > v61 { 1.0 } else { 0.0 };
                        let v1739 = (v1735 * v49) * v1738;
                        let v1746 = (v1739 * v156) * v1745;
                        let v1747 = ((((v50 * v1735) * v1738) * v156) + (v169 * v1739)) * v1745;
                        let v1749 = v1747 * v1746;
                        let v1753 = (v73 - (v1746 * v1746)).sqrt();
                        let v1760 = (-v1746) * v1759;
                        let v1771 = (Lanes([(((v1747 * v17) * v1759) * v1764), 0.0])) + (Lanes([0.0, (v1767 * v1760)]));
                        let v1779 = (Lanes([((((v1749 + v1749) * v17) * (v42 / (v40 * v1753))) * v1772), 0.0])) + (Lanes([0.0, (v1775 * v1753)]));
                        let v1780 = (v1760 * v1764) + (v1753 * v1772);
                        let v1783 = (Lanes([v1771[0], v1771[1], 0.0])) + (Lanes([v1779[0], 0.0, v1779[1]]));
                        let v1784 = -(v1746 * v1759);
                        let v1786 = v1784 * v1764;
                        let v1791 = (Lanes([(((v1747 * v1759) * v17) * v1764), 0.0])) + (Lanes([0.0, (v1767 * v1784)]));
                        let v1792 = ddt(4654, v1786);
                        let v1793 = v1791 * v1094;
                        v1723 = v1764;
                        v1724 = v1772;
                        v1725 = v1764;
                        v1726 = v1780;
                        v1727 = v1792;
                        v1728 = v1786;
                        v1729 = v1767;
                        v1730 = v1775;
                        v1731 = v1767;
                        v1732 = v1783;
                        v1733 = v1793;
                        v1734 = v1791;
                    } else {
                        v1723 = v61;
                        v1724 = v61;
                        v1725 = v61;
                        v1726 = v61;
                        v1727 = v61;
                        v1728 = v61;
                        v1729 = v1706;
                        v1730 = v1707;
                        v1731 = v1706;
                        v1732 = v1708;
                        v1733 = v1709;
                        v1734 = v1709;
                    }
                    v1710 = v1723;
                    v1711 = v1724;
                    v1712 = v1725;
                    v1713 = v1726;
                    v1714 = v1727;
                    v1715 = v1728;
                    v1716 = v1729;
                    v1717 = v1730;
                    v1718 = v1731;
                    v1719 = v1732;
                    v1720 = v1733;
                    v1721 = v1734;
                } else {
                    v1710 = v61;
                    v1711 = v61;
                    v1712 = v61;
                    v1713 = v61;
                    v1714 = v61;
                    v1715 = v61;
                    v1716 = v1706;
                    v1717 = v1707;
                    v1718 = v1706;
                    v1719 = v1708;
                    v1720 = v1709;
                    v1721 = v1709;
                }
                v1694 = v1710;
                v1695 = v1711;
                v1696 = v1712;
                v1697 = v1713;
                v1698 = v1714;
                v1699 = v1715;
                v1700 = v1716;
                v1701 = v1717;
                v1702 = v1718;
                v1703 = v1719;
                v1704 = v1720;
                v1705 = v1721;
            }
            let v1834: f64;
            let v1835: f64;
            let v1836: f64;
            let v1837: f64;
            let v1838: f64;
            let v1839: Lanes<7>;
            let v1840: f64;
            let v1841: f64;
            let v1842: f64;
            let v1843: f64;
            if v1794 != 0.0 {
                let v1795 = v382 * v19;
                let v1797 = v22 * v382;
                let v1804 = ((v384 * v19) + (Lanes([0.0, 0.0, v1797[0], v1797[1], 0.0, 0.0]))) * ((v40 * (if v1795 >= v38 { 1.0 } else { 0.0 })) - v42);
                let v1805 = v958 * v24;
                let v1807 = v28 * v958;
                let v1814 = ((v959 * v24) + (Lanes([0.0, v1807[0], v1807[1]]))) * ((v40 * (if v1805 >= v38 { 1.0 } else { 0.0 })) - v42);
                let v1820 = v1819 * ((v1795.abs()) + (v1805.abs()));
                let v1821 = ((Lanes([v1804[0], v1804[1], v1804[2], v1804[3], v1804[4], 0.0, v1804[5]])) + (Lanes([v1814[0], 0.0, 0.0, v1814[1], 0.0, v1814[2], 0.0]))) * v1819;
                let v1822 = v36 / v885;
                let v1825 = (v44 - (v886 * v1822)) / v885;
                let v1827 = v1826 * v36;
                let v1828 = v44 * v1826;
                let v1829 = ddt(4752, v1827);
                let v1830 = v1828 * v1094;
                v1834 = v1820;
                v1835 = v1822;
                v1836 = v1829;
                v1837 = v61;
                v1838 = v1827;
                v1839 = v1821;
                v1840 = v1825;
                v1841 = v1830;
                v1842 = v48;
                v1843 = v1828;
            } else {
                let v1831 = v36 * v213;
                let v1832 = v44 * v213;
                v1834 = v61;
                v1835 = v61;
                v1836 = v61;
                v1837 = v1831;
                v1838 = v61;
                v1839 = v1833;
                v1840 = v48;
                v1841 = v48;
                v1842 = v1832;
                v1843 = v48;
            }
            let v1844 = v1087[0];
            let v1845 = v1087[1];
            let v1846 = v1087[2];
            let v1847 = v1087[3];
            let v1848 = v1087[4];
            let v1849 = v1087[5];
            let v1850 = v1095;
            let v1852 = v1851;
            let v1853 = v1102;
            let v1854 = v959[0];
            let v1855 = v959[1];
            let v1856 = v959[2];
            let v1857 = v982[0];
            let v1858 = v982[1];
            let v1859 = v982[2];
            let v1860 = v1489[0];
            let v1861 = v1489[1];
            let v1862 = v1489[2];
            let v1863 = v1489[3];
            let v1864 = v1490[0];
            let v1865 = v1490[1];
            let v1866 = v1490[2];
            let v1867 = v1490[3];
            let v1868 = v1491[0];
            let v1869 = v1491[1];
            let v1870 = v1491[2];
            let v1871 = v1491[3];
            let v1872 = v1492[0];
            let v1873 = v1492[1];
            let v1874 = v1492[2];
            let v1875 = v1492[3];
            let v1876 = v1507[0];
            let v1877 = v1507[1];
            let v1878 = v1512[0];
            let v1879 = v1512[1];
            let v1880 = v1526[0];
            let v1881 = v1526[1];
            let v1882 = v1526[2];
            let v1883 = v1528[0];
            let v1884 = v1528[1];
            let v1885 = v1550[0];
            let v1886 = v1550[1];
            let v1887 = v1550[2];
            let v1888 = v1550[3];
            let v1889 = v1550[4];
            let v1890 = v1550[5];
            let v1891 = v1550[6];
            let v1892 = v1576[0];
            let v1893 = v1576[1];
            let v1894 = v1576[2];
            let v1895 = v1577[0];
            let v1896 = v1577[1];
            let v1897 = v1577[2];
            let v1898 = v1589[0];
            let v1899 = v1589[1];
            let v1900 = v1600[0];
            let v1901 = v1600[1];
            let v1902 = v1613[0];
            let v1903 = v1613[1];
            let v1904 = v1625[0];
            let v1905 = v1625[1];
            let v1906 = v1634;
            let v1907 = v1641;
            let v1908 = v1653[0];
            let v1909 = v1653[1];
            let v1910 = v1653[2];
            let v1911 = v1653[3];
            let v1912 = v1653[4];
            let v1913 = v1653[5];
            let v1914 = v1653[6];
            let v1915 = v1660;
            let v1916 = v1672[0];
            let v1917 = v1672[1];
            let v1918 = v1672[2];
            let v1919 = v1672[3];
            let v1920 = v1672[4];
            let v1921 = v1672[5];
            let v1922 = v1672[6];
            let v1923 = v1679;
            let v1924 = v1687[0];
            let v1925 = v1687[1];
            let v1926 = v1700;
            let v1927 = v1701;
            let v1928 = v1702;
            let v1929 = v1703[0];
            let v1930 = v1703[1];
            let v1931 = v1703[2];
            let v1932 = v1704[0];
            let v1933 = v1704[1];
            let v1934 = v1767;
            let v1935 = v1775;
            let v1936 = v1839[0];
            let v1937 = v1839[1];
            let v1938 = v1839[2];
            let v1939 = v1839[3];
            let v1940 = v1839[4];
            let v1941 = v1839[5];
            let v1942 = v1839[6];
            let v1943 = v1840;
            let v1944 = v1841;
            let v1945 = v1842;
            let v1946 = v1092;
            let v1947 = v1100;
            let v1948 = v1493[0];
            let v1949 = v1493[1];
            let v1950 = v1493[2];
            let v1951 = v1493[3];
            let v1952 = v1494[0];
            let v1953 = v1494[1];
            let v1954 = v1494[2];
            let v1955 = v1494[3];
            let v1956 = v1495[0];
            let v1957 = v1495[1];
            let v1958 = v1495[2];
            let v1959 = v1495[3];
            let v1960 = v1496[0];
            let v1961 = v1496[1];
            let v1962 = v1496[2];
            let v1963 = v1496[3];
            let v1964 = v1505[0];
            let v1965 = v1505[1];
            let v1966 = v1510[0];
            let v1967 = v1510[1];
            let v1968 = v1524[0];
            let v1969 = v1524[1];
            let v1970 = v1524[2];
            let v1971 = v1551;
            let v1972 = v1578[0];
            let v1973 = v1578[1];
            let v1974 = v1578[2];
            let v1975 = v1587[0];
            let v1976 = v1587[1];
            let v1977 = v1639;
            let v1978 = v1658;
            let v1979 = v1677;
            let v1980 = v1705[0];
            let v1981 = v1705[1];
            let v1982 = v1843;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(15),
            None,
            multiplicity * (v1086),
            [3, 4, 5, 8, 10, 12],
            [v1844, v1845, v1846, v1847, v1848, v1849],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(15),
            None,
            multiplicity * (v1093),
            [15],
            [v1850],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(16),
            None,
            multiplicity * (v1983),
            [16],
            [v1852],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(15), Some(16), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 1>(
            0,
            v1101,
            [],
            [],
            [0],
            [v1853],
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(5),
            Some(8),
            multiplicity * (v1983),
            [16],
            [v1852],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(11),
            Some(8),
            multiplicity * (v958),
            [3, 8, 11],
            [v1854, v1855, v1856],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(10),
            Some(5),
            multiplicity * (v981),
            [3, 5, 10],
            [v1857, v1858, v1859],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(10),
            Some(5),
            multiplicity * (v1481),
            [3, 5, 8, 10],
            [v1860, v1861, v1862, v1863],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(11),
            Some(8),
            multiplicity * (v1482),
            [3, 5, 8, 11],
            [v1864, v1865, v1866, v1867],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(10),
            Some(5),
            multiplicity * (v1483),
            [3, 5, 8, 10],
            [v1868, v1869, v1870, v1871],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(11),
            Some(8),
            multiplicity * (v1484),
            [3, 5, 8, 11],
            [v1872, v1873, v1874, v1875],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(7),
            Some(5),
            multiplicity * (v1506),
            [5, 7],
            [v1876, v1877],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(5),
            Some(8),
            multiplicity * (v1511),
            [5, 8],
            [v1878, v1879],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(6),
            Some(4),
            multiplicity * (v1525),
            [3, 4, 6],
            [v1880, v1881, v1882],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(6),
            Some(4),
            multiplicity * (v1527),
            [4, 6],
            [v1883, v1884],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(4), Some(8), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<6, 1>(
            1,
            v1548,
            [3, 4, 5, 8, 10, 12],
            [v1885, v1886, v1887, v1888, v1889, v1890],
            [1],
            [v1891],
        );
        stamper.stamp_potential_branch_local(Some(4), Some(8), 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            staged[46],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(11),
            Some(12),
            multiplicity * (v1573),
            [3, 11, 12],
            [v1892, v1893, v1894],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(12),
            Some(8),
            multiplicity * (v1574),
            [3, 8, 12],
            [v1895, v1896, v1897],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(11), Some(8), 3, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            3,
            staged[47],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(11),
            Some(14),
            multiplicity * (v1588),
            [11, 14],
            [v1898, v1899],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(14),
            Some(8),
            multiplicity * (v1599),
            [8, 14],
            [v1900, v1901],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(14), Some(8), 4, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            4,
            staged[48],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(13),
            Some(10),
            multiplicity * (v1612),
            [10, 13],
            [v1902, v1903],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(13), Some(10), 5, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            5,
            staged[49],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(13),
            Some(10),
            multiplicity * (staged[50]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(13),
            Some(11),
            multiplicity * (v1624),
            [11, 13],
            [v1904, v1905],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(13), Some(11), 6, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            6,
            staged[51],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(7), Some(13), 7, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 1>(
            7,
            v1633,
            [],
            [],
            [7],
            [v1906],
        );
        stamper.stamp_potential_branch_local(Some(7), Some(13), 8, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            8,
            staged[52],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(7), Some(13), 9, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            9,
            staged[53],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(1), Some(7), 10, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 1>(
            10,
            v1640,
            [],
            [],
            [10],
            [v1907],
        );
        stamper.stamp_potential_branch_local(Some(8), Some(9), 11, multiplicity);
        stamper.stamp_potential_sparse_local::<6, 1>(
            11,
            v1652,
            [3, 4, 5, 8, 10, 12],
            [v1908, v1909, v1910, v1911, v1912, v1913],
            [11],
            [v1914],
        );
        stamper.stamp_potential_branch_local(Some(8), Some(9), 12, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            12,
            staged[54],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(8), Some(9), 13, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            13,
            staged[55],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(9), Some(2), 14, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 1>(
            14,
            v1659,
            [],
            [],
            [14],
            [v1915],
        );
        stamper.stamp_potential_branch_local(Some(5), Some(6), 15, multiplicity);
        stamper.stamp_potential_sparse_local::<6, 1>(
            15,
            v1671,
            [3, 4, 5, 8, 10, 12],
            [v1916, v1917, v1918, v1919, v1920, v1921],
            [15],
            [v1922],
        );
        stamper.stamp_potential_branch_local(Some(5), Some(6), 16, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            16,
            staged[56],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(5), Some(6), 17, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            17,
            staged[57],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(6), Some(0), 18, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 1>(
            18,
            v1678,
            [],
            [],
            [18],
            [v1923],
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(2),
            multiplicity * (v1984),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(14),
            Some(2),
            multiplicity * (v1985),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(12),
            Some(2),
            multiplicity * (v1686),
            [2, 12],
            [v1924, v1925],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(8),
            multiplicity * (staged[58]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(8),
            multiplicity * (staged[59]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(17),
            None,
            multiplicity * (staged[60]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(17),
            None,
            multiplicity * (v1694),
            [17],
            [v1926],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(18),
            None,
            multiplicity * (staged[61]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(18),
            None,
            multiplicity * (v1695),
            [18],
            [v1927],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(7),
            Some(8),
            multiplicity * (v1696),
            [17],
            [v1928],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(5),
            multiplicity * (v1697),
            [3, 17, 18],
            [v1929, v1930, v1931],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(7),
            Some(5),
            multiplicity * (v1698),
            [3, 17],
            [v1932, v1933],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(8),
            multiplicity * (staged[62]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(8),
            multiplicity * (staged[63]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(8),
            multiplicity * (staged[64]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(8),
            multiplicity * (staged[65]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(17),
            None,
            multiplicity * (v1764),
            [17],
            [v1934],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(18),
            None,
            multiplicity * (v1772),
            [18],
            [v1935],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(11),
            Some(8),
            multiplicity * (staged[66]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(10),
            Some(5),
            multiplicity * (staged[67]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(11),
            Some(8),
            multiplicity * (staged[68]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(10),
            Some(5),
            multiplicity * (staged[69]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(3),
            None,
            multiplicity * (v1834),
            [3, 4, 5, 8, 10, 11, 12],
            [v1936, v1937, v1938, v1939, v1940, v1941, v1942],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (v1835),
            [3],
            [v1943],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (v1836),
            [3],
            [v1944],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (v1837),
            [3],
            [v1945],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = v1086;
        self.canonical_reactive[1] = v1090;
        self.canonical_reactive[2] = v1946;
        self.canonical_reactive[3] = v1983;
        self.canonical_reactive[4] = v1098;
        self.canonical_reactive[5] = v1947;
        self.canonical_reactive[6] = v1983;
        self.canonical_reactive[7] = v958;
        self.canonical_reactive[8] = v981;
        self.canonical_reactive[9] = v1485;
        self.canonical_reactive[10] = v1948;
        self.canonical_reactive[11] = v1949;
        self.canonical_reactive[12] = v1950;
        self.canonical_reactive[13] = v1951;
        self.canonical_reactive[14] = v1486;
        self.canonical_reactive[15] = v1952;
        self.canonical_reactive[16] = v1953;
        self.canonical_reactive[17] = v1954;
        self.canonical_reactive[18] = v1955;
        self.canonical_reactive[19] = v1487;
        self.canonical_reactive[20] = v1956;
        self.canonical_reactive[21] = v1957;
        self.canonical_reactive[22] = v1958;
        self.canonical_reactive[23] = v1959;
        self.canonical_reactive[24] = v1488;
        self.canonical_reactive[25] = v1960;
        self.canonical_reactive[26] = v1961;
        self.canonical_reactive[27] = v1962;
        self.canonical_reactive[28] = v1963;
        self.canonical_reactive[29] = v1504;
        self.canonical_reactive[30] = v1964;
        self.canonical_reactive[31] = v1965;
        self.canonical_reactive[32] = v1509;
        self.canonical_reactive[33] = v1966;
        self.canonical_reactive[34] = v1967;
        self.canonical_reactive[35] = v1519;
        self.canonical_reactive[36] = v1968;
        self.canonical_reactive[37] = v1969;
        self.canonical_reactive[38] = v1970;
        self.canonical_reactive[39] = v1527;
        self.canonical_reactive[40] = v1549;
        self.canonical_reactive[41] = v1971;
        self.canonical_reactive[42] = staged[46];
        self.canonical_reactive[43] = v1573;
        self.canonical_reactive[44] = v1575;
        self.canonical_reactive[45] = v1972;
        self.canonical_reactive[46] = v1973;
        self.canonical_reactive[47] = v1974;
        self.canonical_reactive[48] = staged[47];
        self.canonical_reactive[49] = v1586;
        self.canonical_reactive[50] = v1975;
        self.canonical_reactive[51] = v1976;
        self.canonical_reactive[52] = v1599;
        self.canonical_reactive[53] = staged[48];
        self.canonical_reactive[54] = v1612;
        self.canonical_reactive[55] = staged[49];
        self.canonical_reactive[56] = staged[50];
        self.canonical_reactive[57] = v1624;
        self.canonical_reactive[58] = staged[51];
        self.canonical_reactive[59] = v1633;
        self.canonical_reactive[60] = staged[52];
        self.canonical_reactive[61] = staged[53];
        self.canonical_reactive[62] = v1637;
        self.canonical_reactive[63] = v1977;
        self.canonical_reactive[64] = v1652;
        self.canonical_reactive[65] = staged[54];
        self.canonical_reactive[66] = staged[55];
        self.canonical_reactive[67] = v1656;
        self.canonical_reactive[68] = v1978;
        self.canonical_reactive[69] = v1671;
        self.canonical_reactive[70] = staged[56];
        self.canonical_reactive[71] = staged[57];
        self.canonical_reactive[72] = v1675;
        self.canonical_reactive[73] = v1979;
        self.canonical_reactive[74] = v1984;
        self.canonical_reactive[75] = v1985;
        self.canonical_reactive[76] = v1686;
        self.canonical_reactive[77] = staged[58];
        self.canonical_reactive[78] = staged[59];
        self.canonical_reactive[79] = staged[60];
        self.canonical_reactive[80] = v1694;
        self.canonical_reactive[81] = staged[61];
        self.canonical_reactive[82] = v1695;
        self.canonical_reactive[83] = v1696;
        self.canonical_reactive[84] = v1697;
        self.canonical_reactive[85] = v1699;
        self.canonical_reactive[86] = v1980;
        self.canonical_reactive[87] = v1981;
        self.canonical_reactive[88] = staged[62];
        self.canonical_reactive[89] = staged[63];
        self.canonical_reactive[90] = staged[64];
        self.canonical_reactive[91] = staged[65];
        self.canonical_reactive[92] = v1764;
        self.canonical_reactive[93] = v1772;
        self.canonical_reactive[94] = staged[66];
        self.canonical_reactive[95] = staged[67];
        self.canonical_reactive[96] = staged[68];
        self.canonical_reactive[97] = staged[69];
        self.canonical_reactive[98] = v1834;
        self.canonical_reactive[99] = v1835;
        self.canonical_reactive[100] = v1838;
        self.canonical_reactive[101] = v1982;
        self.canonical_reactive[102] = v1837;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(15),
            None,
            &[15],
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
            Some(10),
            Some(5),
            &[3, 5, 8, 10],
            &[cached[10], cached[11], cached[12], cached[13]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(11),
            Some(8),
            &[3, 5, 8, 11],
            &[cached[15], cached[16], cached[17], cached[18]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(10),
            Some(5),
            &[3, 5, 8, 10],
            &[cached[20], cached[21], cached[22], cached[23]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(11),
            Some(8),
            &[3, 5, 8, 11],
            &[cached[25], cached[26], cached[27], cached[28]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(5),
            &[5, 7],
            &[cached[30], cached[31]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(8),
            &[5, 8],
            &[cached[33], cached[34]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(4),
            &[3, 4, 6],
            &[cached[36], cached[37], cached[38]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_potential_reactive_indexed_dense_local(
            1,
            &[],
            &[],
            &[1],
            &[cached[41]],
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(12),
            Some(8),
            &[3, 8, 12],
            &[cached[45], cached[46], cached[47]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(11),
            Some(14),
            &[11, 14],
            &[cached[50], cached[51]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_potential_reactive_indexed_dense_local(
            10,
            &[],
            &[],
            &[10],
            &[cached[63]],
        );
        stamper.stamp_potential_reactive_indexed_dense_local(
            14,
            &[],
            &[],
            &[14],
            &[cached[68]],
        );
        stamper.stamp_potential_reactive_indexed_dense_local(
            18,
            &[],
            &[],
            &[18],
            &[cached[73]],
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(5),
            &[3, 17],
            &[cached[86], cached[87]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            None,
            &[3],
            &[cached[101]],
            &[],
            &[],
            multiplicity,
        );
    }

}

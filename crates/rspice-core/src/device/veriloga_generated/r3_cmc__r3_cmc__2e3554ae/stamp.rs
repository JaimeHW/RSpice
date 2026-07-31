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
        let mut key = Vec::with_capacity(222);
        for index in 0..Self::PARAMETER_COUNT {
            if PARAMETER_MODEL_FLAGS[index] {
                key.push(self.params.values[index].to_bits());
                key.push(u64::from(self.param_given[index]));
            }
        }
        key.into_boxed_slice()
    }

    fn canonical_install_model_values(&mut self, values: Arc<CanonicalModelValues>) {
        self.canonical_staged[65] = values[0];
        self.canonical_staged[66] = values[1];
        self.canonical_staged[67] = values[2];
        self.canonical_staged[2] = values[3];
        self.canonical_staged[1] = values[4];
        self.canonical_staged[0] = values[5];
        self.canonical_staged[24] = values[6];
        self.canonical_staged[26] = values[7];
        self.canonical_staged[44] = values[8];
        self.canonical_staged[5] = values[9];
        self.canonical_staged[85] = values[10];
        self.canonical_staged[6] = values[11];
        self.canonical_staged[86] = values[12];
        self.canonical_staged[87] = values[13];
        self.canonical_staged[8] = values[14];
        self.canonical_staged[10] = values[15];
        self.canonical_staged[12] = values[16];
        self.canonical_staged[13] = values[17];
        self.canonical_staged[14] = values[18];
        self.canonical_staged[15] = values[19];
        self.canonical_staged[16] = values[20];
        self.canonical_staged[92] = values[21];
        self.canonical_staged[22] = values[22];
        self.canonical_staged[93] = values[23];
        self.canonical_staged[23] = values[24];
        self.canonical_staged[94] = values[25];
        self.canonical_staged[28] = values[26];
        self.canonical_staged[29] = values[27];
        self.canonical_staged[95] = values[28];
        self.canonical_staged[30] = values[29];
        self.canonical_staged[31] = values[30];
        self.canonical_staged[96] = values[31];
        self.canonical_staged[32] = values[32];
        self.canonical_staged[33] = values[33];
        self.canonical_staged[61] = values[34];
        self.canonical_staged[62] = values[35];
        self.canonical_staged[63] = values[36];
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
                let v0 = 1.003e3f64;
                let v1 = parameters[20];
                let v3 = 1e0f64;
                let v4 = parameters[17];
                let v6 = parameters[18];
                let v8 = 1e-2f64;
                let v9 = parameters[23];
                let v12 = parameters[22];
                let v14 = 1e6f64;
                let v17 = 2.7315e2f64;
                let v18 = parameters[28];
                let v20 = parameters[35];
                let v22 = 0e0f64;
                let v24 = 0e0f64;
                let v26 = parameters[63];
                let v28 = 2e0f64;
                let v29 = parameters[64];
                let v31 = 0e0f64;
                let v36 = parameters[46];
                let v38 = parameters[66];
                let v40 = parameters[71];
                let v43 = parameters[72];
                let v46 = parameters[21];
                let v48 = parameters[92];
                let v49 = 1e0f64;
                let v51 = parameters[69];
                let v53 = parameters[90];
                let v55 = parameters[76];
                let v59 = 5e-1f64;
                let v60 = parameters[73];
                let v62 = -5e-1f64;
                let v64 = parameters[74];
                let v66 = parameters[79];
                let v68 = parameters[80];
                let v70 = -5e-1f64;
                let v72 = parameters[81];
                let v74 = parameters[83];
                let v76 = parameters[60];
                let v78 = parameters[27];
                let v79 = parameters[84];
                let mut out30: f64 = 0.0;
                let mut out32: f64 = 0.0;
                let mut out33: f64 = 0.0;
                let mut out34: f64 = 0.0;
                let mut out35: f64 = 0.0;
                let mut out37: f64 = 0.0;
                let mut out54: f64 = 0.0;
                let mut out57: f64 = 0.0;
                let mut out61: f64 = 0.0;
                let mut out63: f64 = 0.0;
                let mut out65: f64 = 0.0;
                let mut out69: f64 = 0.0;
                let mut out71: f64 = 0.0;
                let mut out73: f64 = 0.0;
                let mut out80: f64 = 0.0;
                let v2 = if v0 != v1 { 1.0 } else { 0.0 };
                let v5 = if v3 != v4 { 1.0 } else { 0.0 };
                let v7 = if v3 < v6 { 1.0 } else { 0.0 };
                let v15 = ((v3 - (v8 * v9)) * v12) * v14;
                let v16 = v15 * v15;
                let v19 = v17 + v18;
                let v21 = v20 + v3;
                let v23 = v22 * v16;
                let v25 = v24 * v16;
                let v27 = if v26 > v3 { 1.0 } else { 0.0 };
                if v27 != 0.0 {
                    let v30 = v28 * v29;
                    out30 = v30;
                } else {
                    let v32 = if v26 > v31 { 1.0 } else { 0.0 };
                    out32 = v32;
                    if v32 != 0.0 {
                        let v33 = v28 * v29;
                        out33 = v33;
                    } else {
                    }
                }
                if v27 != 0.0 {
                    let v34 = if v26 > v28 { 1.0 } else { 0.0 };
                    out34 = v34;
                } else {
                    let v35 = if v26 > v31 { 1.0 } else { 0.0 };
                    out35 = v35;
                    if v35 != 0.0 {
                        let v37 = v28 * v36;
                        out37 = v37;
                    } else {
                    }
                }
                let v39 = if v38 > v31 { 1.0 } else { 0.0 };
                let v41 = v40 * v23;
                let v42 = v40 * v25;
                let v44 = v43 * v23;
                let v45 = v43 * v25;
                let v47 = -v46;
                let v50 = v48 - v49;
                let v52 = if v51 > v31 { 1.0 } else { 0.0 };
                if v52 != 0.0 {
                    let v54 = -v53;
                    out54 = v54;
                } else {
                }
                let v56 = if v55 > v31 { 1.0 } else { 0.0 };
                if v56 != 0.0 {
                    let v57 = -v53;
                    out57 = v57;
                } else {
                }
                let v58 = if v43 > v31 { 1.0 } else { 0.0 };
                if v58 != 0.0 {
                    let v61 = v59 * v60;
                    out61 = v61;
                    let v63 = v62 * v60;
                    out63 = v63;
                    let v65 = v64 - v49;
                    out65 = v65;
                } else {
                }
                let v67 = if v66 > v31 { 1.0 } else { 0.0 };
                if v67 != 0.0 {
                    let v69 = v59 * v68;
                    out69 = v69;
                    let v71 = v70 * v68;
                    out71 = v71;
                    let v73 = v72 - v49;
                    out73 = v73;
                } else {
                }
                let v75 = if v74 > v31 { 1.0 } else { 0.0 };
                if v75 != 0.0 {
                    let v80 = v78 / v79;
                    out80 = v80;
                } else {
                }
                let v77 = if v76 > v31 { 1.0 } else { 0.0 };
            [v2, v5, v7, v15, v19, v21, v23, v25, v27, out30, out32, out33, out34, out35, out37, v39, v41, v42, v44, v45, v47, v52, out54, v56, out57, v58, out61, out63, v67, out69, out71, v75, out80, v77, v50, out65, out73]
        };
        let values = canonical_model_cache_intern(key, Arc::new(produced));
        self.canonical_install_model_values(values);
    }

    fn canonical_instance_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        if self.canonical_instance_valid {
            return;
        }
        let produced: [f64; 43] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let v0 = 1e0f64;
                let v1 = parameters[0];
                let v2 = staged[2];
                let v4 = parameters[1];
                let v6 = parameters[31];
                let v8 = parameters[32];
                let v10 = parameters[29];
                let v12 = parameters[30];
                let v14 = parameters[2];
                let v16 = parameters[4];
                let v18 = parameters[7];
                let v21 = 2e0f64;
                let v23 = parameters[5];
                let v24 = 0e0f64;
                let v26 = parameters[8];
                let v31 = 5e-1f64;
                let v33 = parameters[44];
                let v35 = parameters[43];
                let v38 = parameters[38];
                let v40 = parameters[39];
                let v44 = parameters[41];
                let v48 = parameters[42];
                let v51 = parameters[40];
                let v57 = parameters[127];
                let v60 = parameters[16];
                let v61 = parameters[119];
                let v62 = parameters[122];
                let v65 = parameters[11];
                let v66 = parameters[125];
                let v68 = multiplicity;
                let v73 = parameters[120];
                let v74 = parameters[123];
                let v77 = parameters[12];
                let v78 = parameters[126];
                let v84 = parameters[118];
                let v85 = parameters[121];
                let v87 = parameters[10];
                let v88 = parameters[124];
                let v94 = 1e-2f64;
                let v149 = parameters[45];
                let v152 = parameters[53];
                let v155 = parameters[56];
                let v158 = parameters[58];
                let v161 = parameters[55];
                let v164 = parameters[54];
                let v166 = parameters[57];
                let v170 = parameters[59];
                let v175 = parameters[15];
                let v176 = parameters[50];
                let v178 = parameters[51];
                let v181 = parameters[52];
                let v185 = parameters[49];
                let v188 = staged[44];
                let v189 = parameters[48];
                let v192 = parameters[47];
                let v194 = staged[86];
                let v195 = staged[87];
                let v196 = parameters[37];
                let v201 = 4e0f64;
                let v207 = staged[10];
                let v209 = parameters[67];
                let v211 = parameters[66];
                let v220 = parameters[111];
                let v222 = parameters[110];
                let v224 = parameters[112];
                let v228 = parameters[113];
                let v231 = parameters[115];
                let v233 = parameters[114];
                let v235 = parameters[116];
                let v238 = parameters[117];
                let v242 = parameters[97];
                let v244 = parameters[93];
                let v246 = parameters[99];
                let v248 = parameters[95];
                let v253 = parameters[98];
                let v255 = parameters[94];
                let v257 = parameters[100];
                let v259 = parameters[96];
                let v264 = parameters[78];
                let v266 = staged[12];
                let v269 = staged[13];
                let v271 = parameters[79];
                let v273 = staged[14];
                let v276 = staged[15];
                let v279 = staged[33];
                let v281 = parameters[65];
                let v288 = parameters[26];
                let v292 = parameters[13];
                let v299 = 0e0f64;
                let v300 = 0e0f64;
                let v301 = 0e0f64;
                let v302 = 0e0f64;
                let mut out101: f64 = 0.0;
                let mut out120: f64 = 0.0;
                let mut out135: f64 = 0.0;
                let mut out200: f64 = 0.0;
                let mut out206: f64 = 0.0;
                let mut out230: f64 = 0.0;
                let mut out283: f64 = 0.0;
                let mut out284: f64 = 0.0;
                let mut out297: f64 = 0.0;
                let mut out298: f64 = 0.0;
                let v3 = v1 * v2;
                let v5 = v4 * v2;
                let v7 = if v3 < v6 { 1.0 } else { 0.0 };
                let v9 = if v3 > v8 { 1.0 } else { 0.0 };
                let v11 = if v5 < v10 { 1.0 } else { 0.0 };
                let v13 = if v5 > v12 { 1.0 } else { 0.0 };
                let v17 = v16 * v2;
                let v19 = v18 * v2;
                let v20 = v5 * v3;
                let v25 = if v23 > v24 { 1.0 } else { 0.0 };
                let v27 = if v26 > v24 { 1.0 } else { 0.0 };
                let v28 = v25 + v27;
                let v30 = (v21 * v5) + (v28 * v3);
                let v32 = v31 * v28;
                let v55 = (((v3 + v38) + (v40 / v3)) + (v48 * (v0 - (((-v3) / v44).exp())))) / (v0 - ((v51 * (v14 * v2)) / v20));
                let v56 = v5 + (v32 * (v35 + (v33 / v3)));
                let v58: f64;
                let v59: f64;
                if v57 != 0.0 {
                    v58 = v56;
                    v59 = v55;
                } else {
                    v58 = v5;
                    v59 = v3;
                }
                let v102: f64;
                let v103: f64;
                let v104: f64;
                if v60 != 0.0 {
                    let v69 = v68 * v58;
                    let v72 = (v55 + (v61 * v62)) + ((v65 * v66) / (v69.sqrt()));
                    let v83 = (v56 + (v73 * v74)) + ((v77 * v78) / ((v68 * v59).sqrt()));
                    let v96 = (v94 * ((v84 * v85) + ((v87 * v88) / ((v69 * v59).sqrt())))).exp();
                    v102 = v72;
                    v103 = v83;
                    v104 = v96;
                } else {
                    let v101 = if (if v61 != v24 { 1.0 } else { 0.0 }) != 0.0 && (if (if v66 > v24 { 1.0 } else { 0.0 }) != 0.0 || (if v62 > v24 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    out101 = v101;
                    let v115: f64;
                    if v101 != 0.0 {
                        let v108 = v66 / ((v68 * v58).sqrt());
                        let v114 = v55 + (v61 * (((v62 * v62) + (v108 * v108)).sqrt()));
                        v115 = v114;
                    } else {
                        v115 = v55;
                    }
                    let v120 = if (if v73 != v24 { 1.0 } else { 0.0 }) != 0.0 && (if (if v78 > v24 { 1.0 } else { 0.0 }) != 0.0 || (if v74 > v24 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    out120 = v120;
                    let v130: f64;
                    if v120 != 0.0 {
                        let v123 = v78 / ((v68 * v59).sqrt());
                        let v129 = v56 + (v73 * (((v74 * v74) + (v123 * v123)).sqrt()));
                        v130 = v129;
                    } else {
                        v130 = v56;
                    }
                    let v135 = if (if v84 != v24 { 1.0 } else { 0.0 }) != 0.0 && (if (if v88 > v24 { 1.0 } else { 0.0 }) != 0.0 || (if v85 > v24 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    out135 = v135;
                    let v147: f64;
                    if v135 != 0.0 {
                        let v139 = v88 / (((v68 * v58) * v59).sqrt());
                        let v146 = ((v94 * v84) * (((v85 * v85) + (v139 * v139)).sqrt())).exp();
                        v147 = v146;
                    } else {
                        v147 = v0;
                    }
                    v102 = v115;
                    v103 = v130;
                    v104 = v147;
                }
                let v105 = if v102 <= v24 { 1.0 } else { 0.0 };
                let v148 = if v103 <= v24 { 1.0 } else { 0.0 };
                let v150 = v103 + v149;
                let v151 = if v150 <= v24 { 1.0 } else { 0.0 };
                let v153: f64;
                let v154: f64;
                if v152 != 0.0 {
                    v153 = v102;
                    v154 = v103;
                } else {
                    v153 = v3;
                    v154 = v5;
                }
                let v157 = v0 / (v153.powf(v155));
                let v160 = v0 / (v154.powf(v158));
                let v174 = ((v164 * (v0 + (v161 * v157))) * (v0 + (v166 * v160))) * (v0 + ((v170 * v157) * v160));
                let v187: f64;
                if v175 != 0.0 {
                    v187 = v24;
                } else {
                    let v186 = v185 + ((((v176 * v154) + (v178 * v153)) + v181) / (v154 * v153));
                    v187 = v186;
                }
                let v193 = v192 / (v0 + (v189 / v103));
                if v188 != 0.0 {
                    if v194 != 0.0 {
                        let v200 = -v193;
                        out200 = v200;
                    } else {
                    }
                } else {
                    let v206: f64;
                    if v195 != 0.0 {
                        let v203 = (v201 * v193) * v193;
                        v206 = v203;
                    } else {
                        let v205 = (v201 * v193) * v193;
                        v206 = v205;
                    }
                    out206 = v206;
                }
                let v199 = (v196 * v104) * (v103 / v102);
                let v208 = if v207 != 0.0 && v25 != 0.0 { 1.0 } else { 0.0 };
                let v214: f64;
                if v208 != 0.0 {
                    let v213 = (v211 + (v209 / v3)) / v23;
                    v214 = v213;
                } else {
                    v214 = v24;
                }
                let v215 = if v207 != 0.0 && v27 != 0.0 { 1.0 } else { 0.0 };
                let v219: f64;
                if v215 != 0.0 {
                    let v218 = (v211 + (v209 / v3)) / v26;
                    v219 = v218;
                } else {
                    v219 = v24;
                }
                let v241: f64;
                if v175 != 0.0 {
                    v241 = v24;
                } else {
                    let v227 = v23 + v26;
                    let v230 = ((v222 + (v220 * v30)) + (v224 * v20)) + (v228 * v227);
                    out230 = v230;
                    let v240 = ((v233 + (v231 * v30)) + (v235 * v20)) + (v238 * v227);
                    v241 = v240;
                }
                let v252 = (v244 + (v242 / v102)) + ((v32 * (v248 + (v246 / v102))) / v103);
                let v263 = (v255 + (v253 / v102)) + ((v32 * (v259 + (v257 / v102))) / v103);
                let v267 = v266 + (v264 * v17);
                let v270 = v269 + (v264 * v19);
                let v274 = v273 + (v271 * v17);
                let v277 = v276 + (v271 * v19);
                let v278 = if v175 == 0.0 { 1.0 } else { 0.0 };
                let v280 = if v279 != 0.0 && v278 != 0.0 { 1.0 } else { 0.0 };
                if v280 != 0.0 {
                    let v283 = (v201 * v281) * v281;
                    out283 = v283;
                    let v284 = v21 * v281;
                    out284 = v284;
                } else {
                }
                let v285 = if v274 > v24 { 1.0 } else { 0.0 };
                let v286 = if v277 > v24 { 1.0 } else { 0.0 };
                let v289 = if (v214 / v68) <= v288 { 1.0 } else { 0.0 };
                let v291 = if (v219 / v68) <= v288 { 1.0 } else { 0.0 };
                let v293: f64;
                let v294: f64;
                let v295: f64;
                let v296: f64;
                if v292 != 0.0 {
                    let v297 = if v214 > v24 { 1.0 } else { 0.0 };
                    out297 = v297;
                    let v298 = if v219 > v24 { 1.0 } else { 0.0 };
                    out298 = v298;
                    v293 = v299;
                    v294 = v300;
                    v295 = v301;
                    v296 = v302;
                } else {
                    v293 = v24;
                    v294 = v24;
                    v295 = v24;
                    v296 = v24;
                }
            [v7, v9, v11, v13, v17, v19, out101, out120, out135, v102, v105, v148, v150, v151, v174, v187, out200, v199, v208, v215, out230, v252, v263, v267, v270, v278, v280, out283, out284, out206, v285, v286, v241, v214, v289, v219, v291, out297, out298, v293, v294, v295, v296]
        };
        self.canonical_staged[72] = produced[0];
        self.canonical_staged[73] = produced[1];
        self.canonical_staged[74] = produced[2];
        self.canonical_staged[75] = produced[3];
        self.canonical_staged[25] = produced[4];
        self.canonical_staged[27] = produced[5];
        self.canonical_staged[76] = produced[6];
        self.canonical_staged[78] = produced[7];
        self.canonical_staged[79] = produced[8];
        self.canonical_staged[54] = produced[9];
        self.canonical_staged[77] = produced[10];
        self.canonical_staged[80] = produced[11];
        self.canonical_staged[36] = produced[12];
        self.canonical_staged[81] = produced[13];
        self.canonical_staged[3] = produced[14];
        self.canonical_staged[4] = produced[15];
        self.canonical_staged[7] = produced[16];
        self.canonical_staged[9] = produced[17];
        self.canonical_staged[90] = produced[18];
        self.canonical_staged[91] = produced[19];
        self.canonical_staged[11] = produced[20];
        self.canonical_staged[19] = produced[21];
        self.canonical_staged[18] = produced[22];
        self.canonical_staged[55] = produced[23];
        self.canonical_staged[56] = produced[24];
        self.canonical_staged[48] = produced[25];
        self.canonical_staged[97] = produced[26];
        self.canonical_staged[34] = produced[27];
        self.canonical_staged[35] = produced[28];
        self.canonical_staged[89] = produced[29];
        self.canonical_staged[104] = produced[30];
        self.canonical_staged[105] = produced[31];
        self.canonical_staged[57] = produced[32];
        self.canonical_staged[58] = produced[33];
        self.canonical_staged[106] = produced[34];
        self.canonical_staged[59] = produced[35];
        self.canonical_staged[107] = produced[36];
        self.canonical_staged[109] = produced[37];
        self.canonical_staged[110] = produced[38];
        self.canonical_staged[111] = produced[39];
        self.canonical_staged[112] = produced[40];
        self.canonical_staged[113] = produced[41];
        self.canonical_staged[114] = produced[42];
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
        let produced: [f64; 35] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let temperature = ctx.temperature();
            let staged = &*self.canonical_staged;
                let v0 = 1e0f64;
                let v1 = temperature;
                let v2 = parameters[9];
                let v4 = 2.7315e2f64;
                let v6 = parameters[24];
                let v8 = parameters[25];
                let v10 = staged[0];
                let v12 = parameters[35];
                let v17 = parameters[36];
                let v22 = 1.3806505e-23f64;
                let v24 = 1.60217653e-19f64;
                let v26 = staged[1];
                let v34 = parameters[104];
                let v36 = parameters[103];
                let v40 = staged[3];
                let v42 = 1e-1f64;
                let v46 = 1e4f64;
                let v49 = parameters[15];
                let v50 = staged[4];
                let v52 = 0e0f64;
                let v57 = 5e-1f64;
                let v61 = staged[44];
                let v64 = staged[5];
                let v67 = 1.666666666666667e-1f64;
                let v70 = staged[85];
                let v73 = staged[6];
                let v78 = parameters[46];
                let v80 = staged[86];
                let v81 = staged[87];
                let v86 = staged[9];
                let v88 = 1e-99f64;
                let v90 = 5.5e-1f64;
                let v92 = staged[7];
                let v97 = 1.1e0f64;
                let v100 = staged[8];
                let v104 = staged[89];
                let v105 = parameters[109];
                let v107 = staged[11];
                let v110 = parameters[63];
                let v112 = -4e-1f64;
                let v114 = 1e-9f64;
                let v117 = 2e0f64;
                let v120 = parameters[14];
                let v122 = staged[48];
                let v142 = 1e0f64;
                let mut out19: f64 = 0.0;
                let mut out53: f64 = 0.0;
                let mut out111: f64 = 0.0;
                let mut out113: f64 = 0.0;
                let mut out118: f64 = 0.0;
                let mut out124: f64 = 0.0;
                let mut out130: f64 = 0.0;
                let mut out132: f64 = 0.0;
                let mut out133: f64 = 0.0;
                let mut out135: f64 = 0.0;
                let mut out141: f64 = 0.0;
                let mut out143: f64 = 0.0;
                let mut out144: f64 = 0.0;
                let v3 = v1 + v2;
                let v5 = v3 - v4;
                let v7 = if v5 < v6 { 1.0 } else { 0.0 };
                let v9 = if v5 > v8 { 1.0 } else { 0.0 };
                let v11 = if v5 < v10 { 1.0 } else { 0.0 };
                let v20: f64;
                if v11 != 0.0 {
                    let v16 = v12 + (((v5 - v12) - v0).exp());
                    v20 = v16;
                } else {
                    let v19 = if v5 > (v17 - v0) { 1.0 } else { 0.0 };
                    out19 = v19;
                    let v33: f64;
                    if v19 != 0.0 {
                        let v32 = v17 - (((v17 - v5) - v0).exp());
                        v33 = v32;
                    } else {
                        v33 = v5;
                    }
                    v20 = v33;
                }
                let v21 = v20 + v4;
                let v25 = (v22 * v21) / v24;
                let v27 = v21 / v26;
                let v28 = v21 - v26;
                let v41 = v40 * (v0 + (v28 * (v36 + (v28 * v34))));
                let v43 = if v41 > v42 { 1.0 } else { 0.0 };
                let v44: f64;
                if v43 != 0.0 {
                    v44 = v41;
                } else {
                    v44 = v42;
                }
                let v45 = v44.sqrt();
                let v48 = v45 / (v44 + v46);
                let v51 = if v50 < v48 { 1.0 } else { 0.0 };
                let v55: f64;
                let v56: f64;
                if v51 != 0.0 {
                    let v53 = if v50 > v52 { 1.0 } else { 0.0 };
                    out53 = v53;
                    let v62: f64;
                    if v53 != 0.0 {
                        v62 = v50;
                    } else {
                        v62 = v52;
                    }
                    let v63 = v48 * v48;
                    v55 = v63;
                    v56 = v62;
                } else {
                    let v54 = v50 * v50;
                    v55 = v54;
                    v56 = v50;
                }
                let v59 = v44 * v57;
                let v60 = (v57 / v55) - v59;
                let v71: f64;
                let v72: f64;
                if v61 != 0.0 {
                    let v66 = v60 - (v64 / v55);
                    let v69 = (v67 / v55) - v59;
                    v71 = v66;
                    v72 = v69;
                } else {
                    let v77: f64;
                    if v70 != 0.0 {
                        let v76 = v60 - ((v73 / v55).sqrt());
                        v77 = v76;
                    } else {
                        v77 = v60;
                    }
                    v71 = v77;
                    v72 = v52;
                }
                let v82: f64;
                let v83: f64;
                if v61 != 0.0 {
                    let v79 = v78 * v25;
                    let v99: f64;
                    if v80 != 0.0 {
                        let v96 = (v90 * v25) * (v0 + ((v92 / v25).exp()));
                        v99 = v96;
                    } else {
                        let v98 = v97 * v25;
                        v99 = v98;
                    }
                    v82 = v79;
                    v83 = v99;
                } else {
                    let v103: f64;
                    if v81 != 0.0 {
                        let v101 = v100 * v25;
                        v103 = v101;
                    } else {
                        let v102 = v78 * v25;
                        v103 = v102;
                    }
                    v82 = v103;
                    v83 = v104;
                }
                let v85 = v0 - (v56 * v45);
                let v87 = v86 * v85;
                let v89 = if v87 <= v88 { 1.0 } else { 0.0 };
                let v109: f64;
                if v49 != 0.0 {
                    v109 = v52;
                } else {
                    let v108 = v107 * (v27.powf(v105));
                    v109 = v108;
                }
                if v110 != 0.0 {
                    let v111 = v87 * v85;
                    out111 = v111;
                } else {
                }
                if v110 != 0.0 {
                } else {
                    let v113 = v112 * v44;
                    out113 = v113;
                }
                let v116 = if v61 != 0.0 && (if v56 > v114 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v116 != 0.0 {
                    let v118 = if v110 > v117 { 1.0 } else { 0.0 };
                    out118 = v118;
                } else {
                }
                let v119 = if v109 > v52 { 1.0 } else { 0.0 };
                let v123 = if (if v119 != 0.0 && v120 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v122 != 0.0 { 1.0 } else { 0.0 };
                if v123 != 0.0 {
                    let v124 = if v105 == v52 { 1.0 } else { 0.0 };
                    out124 = v124;
                    if v124 != 0.0 {
                    } else {
                        let v131: f64;
                        if v11 != 0.0 {
                            let v128 = v12 + (((v5 - v12) - v0).exp());
                            v131 = v128;
                        } else {
                            let v130 = if v5 > (v17 - v0) { 1.0 } else { 0.0 };
                            out130 = v130;
                            let v140: f64;
                            if v130 != 0.0 {
                                let v139 = v17 - (((v17 - v5) - v0).exp());
                                v140 = v139;
                            } else {
                                v140 = v5;
                            }
                            v131 = v140;
                        }
                        let v132 = v131 + v4;
                        out132 = v132;
                        let v133 = v105 + v0;
                        out133 = v133;
                        let v135 = if (v133.abs()) > v42 { 1.0 } else { 0.0 };
                        out135 = v135;
                        if v135 != 0.0 {
                            let v141 = v109 * v132;
                            out141 = v141;
                            let v143 = v133 - v142;
                            out143 = v143;
                        } else {
                            let v144 = v57 * v105;
                            out144 = v144;
                        }
                    }
                } else {
                }
                let v146: f64;
                if v119 != 0.0 {
                    let v145 = v0 / v109;
                    v146 = v145;
                } else {
                    v146 = v52;
                }
            [v3, v7, v9, v11, out19, v25, v43, v44, v51, out53, v55, v60, v56, v87, v89, out111, v71, v82, out113, v72, v116, v83, out118, v109, v119, v123, out124, out130, out132, out133, out135, out141, out144, v146, out143]
        };
        self.canonical_staged[17] = produced[0];
        self.canonical_staged[68] = produced[1];
        self.canonical_staged[69] = produced[2];
        self.canonical_staged[70] = produced[3];
        self.canonical_staged[71] = produced[4];
        self.canonical_staged[45] = produced[5];
        self.canonical_staged[82] = produced[6];
        self.canonical_staged[39] = produced[7];
        self.canonical_staged[83] = produced[8];
        self.canonical_staged[84] = produced[9];
        self.canonical_staged[41] = produced[10];
        self.canonical_staged[43] = produced[11];
        self.canonical_staged[46] = produced[12];
        self.canonical_staged[21] = produced[13];
        self.canonical_staged[88] = produced[14];
        self.canonical_staged[20] = produced[15];
        self.canonical_staged[37] = produced[16];
        self.canonical_staged[38] = produced[17];
        self.canonical_staged[40] = produced[18];
        self.canonical_staged[42] = produced[19];
        self.canonical_staged[98] = produced[20];
        self.canonical_staged[47] = produced[21];
        self.canonical_staged[99] = produced[22];
        self.canonical_staged[49] = produced[23];
        self.canonical_staged[108] = produced[24];
        self.canonical_staged[100] = produced[25];
        self.canonical_staged[101] = produced[26];
        self.canonical_staged[102] = produced[27];
        self.canonical_staged[50] = produced[28];
        self.canonical_staged[51] = produced[29];
        self.canonical_staged[103] = produced[30];
        self.canonical_staged[52] = produced[31];
        self.canonical_staged[53] = produced[32];
        self.canonical_staged[60] = produced[33];
        self.canonical_staged[64] = produced[34];
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
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5])];
        let branch_unknown_flows = [ctx.branch_current(self.branches[0]), ctx.branch_current(self.branches[1])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 8156 => 0usize, 8158 => 1usize, 8160 => 2usize, _ => usize::MAX };
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
            let v0 = 0e0f64;
            let v2 = 1e0f64;
            let v3 = node_potentials[5];
            let v4 = node_potentials[4];
            let v6 = 1e0f64;
            let v8 = 1e0f64;
            let v11 = staged[16];
            let v14 = node_potentials[1];
            let v16 = 1e0f64;
            let v28 = staged[17];
            let v29 = node_potentials[3];
            let v31 = 2.7315e2f64;
            let v33 = staged[0];
            let v35 = parameters[35];
            let v39 = 1e0f64;
            let v42 = parameters[36];
            let v48 = 1.3806505e-23f64;
            let v51 = 1.60217653e-19f64;
            let v54 = staged[1];
            let v58 = staged[18];
            let v61 = staged[19];
            let v68 = 1.1e-1f64;
            let v71 = -1e0f64;
            let v80 = 1e-2f64;
            let v82 = 1e1f64;
            let v88 = 1e-1f64;
            let v94 = parameters[63];
            let v95 = staged[20];
            let v102 = staged[21];
            let v111 = parameters[102];
            let v114 = parameters[101];
            let v121 = 1.1e-1f64;
            let v134 = parameters[92];
            let v136 = staged[61];
            let v140 = staged[92];
            let v143 = staged[22];
            let v151 = 1e0f64;
            let v154 = parameters[91];
            let v159 = parameters[70];
            let v164 = parameters[69];
            let v169 = parameters[27];
            let v182 = 0e0f64;
            let v187 = staged[93];
            let v190 = staged[23];
            let v204 = parameters[77];
            let v209 = parameters[76];
            let v230 = staged[24];
            let v233 = staged[25];
            let v237 = staged[26];
            let v240 = staged[27];
            let v244 = staged[94];
            let v249 = 2e0f64;
            let v252 = staged[28];
            let v261 = staged[29];
            let v283 = 3e0f64;
            let v296 = parameters[90];
            let v311 = 4e0f64;
            let v316 = 2e0f64;
            let v321 = 5e-1f64;
            let v333 = parameters[73];
            let v338 = parameters[74];
            let v340 = staged[62];
            let v344 = parameters[72];
            let v351 = staged[95];
            let v358 = staged[30];
            let v367 = staged[31];
            let v434 = parameters[80];
            let v439 = parameters[81];
            let v441 = staged[63];
            let v445 = parameters[79];
            let v452 = parameters[108];
            let v455 = parameters[86];
            let v458 = staged[96];
            let v459 = parameters[106];
            let v462 = parameters[105];
            let v469 = parameters[83];
            let v473 = parameters[85];
            let v480 = staged[97];
            let v483 = parameters[107];
            let v501 = staged[32];
            let v510 = parameters[62];
            let v511 = 1e3f64;
            let v522 = staged[36];
            let v525 = 1e5f64;
            let v527 = parameters[61];
            let v534 = parameters[60];
            let v548 = staged[34];
            let v561 = staged[35];
            let v566 = parameters[65];
            let v605 = -1e0f64;
            let v614 = staged[37];
            let v618 = staged[38];
            let v649 = staged[40];
            let v655 = staged[39];
            let v660 = -4e-1f64;
            let v669 = -4e-1f64;
            let v672 = -4e-1f64;
            let v674 = Lanes([0e0f64; 3]);
            let v677 = staged[41];
            let v689 = -1e0f64;
            let v697 = 2.25e0f64;
            let v701 = 1.5e0f64;
            let v771 = 3.333333333333333e-1f64;
            let v784 = 9e0f64;
            let v796 = 2.7e1f64;
            let v799 = 2.5e-1f64;
            let v813 = staged[42];
            let v817 = staged[98];
            let v818 = -5e-1f64;
            let v829 = -5e-1f64;
            let v844 = 1e-6f64;
            let v847 = -6.666666666666667e-1f64;
            let v851 = -1e-6f64;
            let v859 = -6.666666666666667e-1f64;
            let v865 = 1e4f64;
            let v871 = -6.666666666666667e-1f64;
            let v875 = -1e-6f64;
            let v896 = 7.5e-1f64;
            let v935 = -6.666666666666667e-1f64;
            let v949 = -2.5e-1f64;
            let v964 = 1e-4f64;
            let v974 = -2.5e-1f64;
            let v986 = staged[43];
            let v1031 = 4.5e0f64;
            let v1038 = staged[45];
            let v1047 = staged[46];
            let v1063 = staged[47];
            let v1091 = parameters[64];
            let v1217 = parameters[47];
            let v1268 = staged[99];
            let v1359 = Lanes([0e0f64; 4]);
            let v1440 = Lanes([0e0f64; 3]);
            let v1593 = parameters[84];
            let v1607 = Lanes([0e0f64; 3]);
            let v1631 = node_potentials[0];
            let v1633 = 1e0f64;
            let v1637 = branch_unknown_flows[0];
            let v1639 = 1e0f64;
            let v1649 = node_potentials[2];
            let v1651 = 1e0f64;
            let v1655 = branch_unknown_flows[1];
            let v1657 = 1e0f64;
            let v1667 = staged[100];
            let v1831 = staged[101];
            let v1832 = 1e6f64;
            let v1835 = Lanes([0e0f64; 8]);
            let v1846 = staged[54];
            let v1849 = parameters[33];
            let v1851 = staged[49];
            let v1856 = staged[103];
            let v1857 = staged[50];
            let v1861 = staged[51];
            let v1863 = staged[64];
            let v1868 = staged[52];
            let v1875 = staged[53];
            let v1891 = parameters[34];
            let v1898 = staged[104];
            let v1901 = staged[105];
            let v1907 = 4e-2f64;
            let v1926 = parameters[68];
            let v1929 = parameters[75];
            let v1951 = -5e-1f64;
            let v2081 = parameters[82];
            let v2114 = -5e-1f64;
            let v2242 = staged[55];
            let v2248 = staged[56];
            let v2258 = staged[57];
            let v2261 = staged[106];
            let v2308 = -5e-1f64;
            let v2470 = -5e-1f64;
            let v2596 = staged[58];
            let v2605 = Lanes([0e0f64; 3]);
            let v2614 = Lanes([0e0f64; 2]);
            let v2619 = staged[107];
            let v2620 = staged[59];
            let v2629 = Lanes([0e0f64; 3]);
            let v2638 = Lanes([0e0f64; 2]);
            let v2644 = ddt_scale();
            let v2650 = parameters[13];
            let v2654 = 0e0f64;
            let v2656 = 0e0f64;
            let v2660 = staged[60];
            let v2667 = 1e-99f64;
            let v1 = ctx.simparam_or("gmin", v0);
            let v12 = v11 * (v3 - v4);
            let v13 = ((Lanes([0.0, v6])) - (Lanes([v8, 0.0]))) * v11;
            let v20 = v11 * (v14 - v4);
            let v21 = ((Lanes([v16, 0.0])) - (Lanes([0.0, v8]))) * v11;
            let v26 = v11 * (v14 - v3);
            let v27 = ((Lanes([v16, 0.0])) - (Lanes([0.0, v6]))) * v11;
            let v32 = (v28 + v29) - v31;
            let v34 = if v32 < v33 { 1.0 } else { 0.0 };
            let v45: f64;
            let v46: f64;
            if v34 != 0.0 {
                let v38 = ((v32 - v35) - v2).exp();
                let v40 = v39 * v38;
                let v41 = v35 + v38;
                v45 = v41;
                v46 = v40;
            } else {
                let v44 = if v32 > (v42 - v2) { 1.0 } else { 0.0 };
                let v78: f64;
                let v79: f64;
                if v44 != 0.0 {
                    let v74 = ((v42 - v32) - v2).exp();
                    let v76 = v42 - v74;
                    let v77 = ((v39 * v71) * v74) * v71;
                    v78 = v76;
                    v79 = v77;
                } else {
                    v78 = v32;
                    v79 = v39;
                }
                v45 = v78;
                v46 = v79;
            }
            let v47 = v45 + v31;
            let v52 = (v48 * v47) / v51;
            let v53 = (v46 * v48) / v51;
            let v55 = v47 / v54;
            let v56 = v46 / v54;
            let v57 = v47 - v54;
            let v62 = v61 + (v57 * v58);
            let v66 = (v46 * v62) + ((v46 * v58) * v57);
            let v67 = v2 + (v57 * v62);
            let v69 = if v67 < v68 { 1.0 } else { 0.0 };
            let v92: f64;
            let v93: f64;
            if v69 != 0.0 {
                let v86 = ((v82 * (v67 - v80)) - v2).exp();
                let v90 = ((v66 * v82) * v86) * v88;
                let v91 = v80 + (v88 * v86);
                v92 = v91;
                v93 = v90;
            } else {
                v92 = v67;
                v93 = v66;
            }
            let v109: f64;
            let v110: f64;
            if v94 != 0.0 {
                let v96 = v95 * v92;
                let v98 = v2 / v96;
                let v101 = (((v93 * v95) * v98) * v71) / v96;
                v109 = v98;
                v110 = v101;
            } else {
                let v103 = v102 * v92;
                let v105 = v2 / v103;
                let v108 = (((v93 * v102) * v105) * v71) / v103;
                v109 = v105;
                v110 = v108;
            }
            let v115 = v114 + (v57 * v111);
            let v119 = (v46 * v115) + ((v46 * v111) * v57);
            let v120 = v2 + (v57 * v115);
            let v122 = if v120 < v121 { 1.0 } else { 0.0 };
            let v132: f64;
            let v133: f64;
            if v122 != 0.0 {
                let v127 = ((v82 * (v120 - v80)) - v2).exp();
                let v130 = ((v119 * v82) * v127) * v88;
                let v131 = v80 + (v88 * v127);
                v132 = v131;
                v133 = v130;
            } else {
                v132 = v120;
                v133 = v119;
            }
            let v135 = v55.powf(v134);
            let v139 = v56 * (v134 * (v55.powf(v136)));
            let v183: f64;
            let v184: f64;
            let v185: f64;
            let v186: f64;
            if v140 != 0.0 {
                let v146 = (v143 * (v2 - v55)) / v52;
                let v162 = ((v146 + (v154 * (v55.ln()))) / v159).exp();
                let v165 = v164 * v162;
                let v166 = (((((((v56 * v71) * v143) - (v53 * v146)) / v52) + ((v56 * (v151 / v55)) * v154)) / v159) * v162) * v164;
                let v167 = v159 * v52;
                let v170 = v169 / v165;
                let v174 = v2 + v170;
                let v175 = v174.ln();
                let v178 = v167 * v175;
                let v181 = ((v53 * v159) * v175) + (((((v166 * v170) * v71) / v165) * (v151 / v174)) * v167);
                v183 = v165;
                v184 = v178;
                v185 = v166;
                v186 = v181;
            } else {
                v183 = v0;
                v184 = v0;
                v185 = v182;
                v186 = v182;
            }
            let v226: f64;
            let v227: f64;
            let v228: f64;
            let v229: f64;
            if v187 != 0.0 {
                let v193 = (v190 * (v2 - v55)) / v52;
                let v207 = ((v193 + (v154 * (v55.ln()))) / v204).exp();
                let v210 = v209 * v207;
                let v211 = (((((((v56 * v71) * v190) - (v53 * v193)) / v52) + ((v56 * (v151 / v55)) * v154)) / v204) * v207) * v209;
                let v212 = v204 * v52;
                let v214 = v169 / v210;
                let v218 = v2 + v214;
                let v219 = v218.ln();
                let v222 = v212 * v219;
                let v225 = ((v53 * v204) * v219) + (((((v211 * v214) * v71) / v210) * (v151 / v218)) * v212);
                v226 = v210;
                v227 = v222;
                v228 = v211;
                v229 = v225;
            } else {
                v226 = v0;
                v227 = v0;
                v228 = v182;
                v229 = v182;
            }
            let v231 = v230 * v183;
            let v232 = v185 * v230;
            let v234 = v233 * v226;
            let v235 = v228 * v233;
            let v236 = v231 + v234;
            let v238 = v237 * v183;
            let v239 = v185 * v237;
            let v241 = v240 * v226;
            let v242 = v228 * v240;
            let v243 = v238 + v241;
            let v347: f64;
            let v348: f64;
            let v349: f64;
            let v350: f64;
            if v244 != 0.0 {
                let v245 = v52 / v55;
                let v250 = v249 * v245;
                let v255 = (v252 * v55) / v52;
                let v259 = v255.exp();
                let v264 = (v261 * v55) / v52;
                let v268 = v264.exp();
                let v270 = v259 - v268;
                let v272 = v270.ln();
                let v275 = v250 * v272;
                let v284 = v283 * v52;
                let v286 = v55.ln();
                let v299 = ((v275 * v55) - (v284 * v286)) - (v296 * (v55 - v2));
                let v300 = ((((((((v53 - (v56 * v245)) / v55) * v249) * v272) + (((((((v56 * v252) - (v53 * v255)) / v52) * v259) - ((((v56 * v261) - (v53 * v264)) / v52) * v268)) * (v151 / v270)) * v250)) * v55) + (v56 * v275)) - (((v53 * v283) * v286) + ((v56 * (v151 / v55)) * v284))) - (v56 * v296);
                let v301 = v249 * v52;
                let v305 = (-v299) / v52;
                let v309 = v305.exp();
                let v315 = (v2 + (v311 * v309)).sqrt();
                let v322 = v321 * (v2 + v315);
                let v324 = v322.ln();
                let v331 = v299 + (v301 * v324);
                let v332 = v300 + (((v53 * v249) * v324) + (((((((((v300 * v71) - (v53 * v305)) / v52) * v309) * v311) * (v151 / (v316 * v315))) * v321) * (v151 / v322)) * v301));
                let v334 = v333 / v331;
                let v345 = v344 * (v334.powf(v338));
                let v346 = ((((v332 * v334) * v71) / v331) * (v338 * (v334.powf(v340)))) * v344;
                v347 = v345;
                v348 = v331;
                v349 = v346;
                v350 = v332;
            } else {
                v347 = v0;
                v348 = v333;
                v349 = v182;
                v350 = v182;
            }
            let v448: f64;
            let v449: f64;
            let v450: f64;
            let v451: f64;
            if v351 != 0.0 {
                let v352 = v52 / v55;
                let v356 = v249 * v352;
                let v361 = (v358 * v55) / v52;
                let v365 = v361.exp();
                let v370 = (v367 * v55) / v52;
                let v374 = v370.exp();
                let v376 = v365 - v374;
                let v378 = v376.ln();
                let v381 = v356 * v378;
                let v389 = v283 * v52;
                let v391 = v55.ln();
                let v403 = ((v381 * v55) - (v389 * v391)) - (v296 * (v55 - v2));
                let v404 = ((((((((v53 - (v56 * v352)) / v55) * v249) * v378) + (((((((v56 * v358) - (v53 * v361)) / v52) * v365) - ((((v56 * v367) - (v53 * v370)) / v52) * v374)) * (v151 / v376)) * v356)) * v55) + (v56 * v381)) - (((v53 * v283) * v391) + ((v56 * (v151 / v55)) * v389))) - (v56 * v296);
                let v405 = v249 * v52;
                let v409 = (-v403) / v52;
                let v413 = v409.exp();
                let v418 = (v2 + (v311 * v413)).sqrt();
                let v423 = v321 * (v2 + v418);
                let v425 = v423.ln();
                let v432 = v403 + (v405 * v425);
                let v433 = v404 + (((v53 * v249) * v425) + (((((((((v404 * v71) - (v53 * v409)) / v52) * v413) * v311) * (v151 / (v316 * v418))) * v321) * (v151 / v423)) * v405));
                let v435 = v434 / v432;
                let v446 = v445 * (v435.powf(v439));
                let v447 = ((((v433 * v435) * v71) / v432) * (v439 * (v435.powf(v441)))) * v445;
                v448 = v446;
                v449 = v432;
                v450 = v447;
                v451 = v433;
            } else {
                v448 = v0;
                v449 = v434;
                v450 = v182;
                v451 = v182;
            }
            let v457 = if ((v2 + (v57 * v452)) * v455) > v0 { 1.0 } else { 0.0 };
            let v474: f64;
            let v475: f64;
            let v476: f64;
            let v477: f64;
            let v478: f64;
            let v479: f64;
            if v458 != 0.0 {
                let v463 = v462 + (v57 * v459);
                let v470 = v469 * (v2 + (v57 * v463));
                let v471 = ((v46 * v463) + ((v46 * v459) * v57)) * v469;
                let v472 = if v470 > v0 { 1.0 } else { 0.0 };
                let v481: f64;
                let v482: f64;
                if v472 != 0.0 {
                    v481 = v470;
                    v482 = v471;
                } else {
                    v481 = v0;
                    v482 = v182;
                }
                let v487 = v473 * (v2 + (v483 * v57));
                let v488 = (v46 * v483) * v473;
                let v489 = v487 * v52;
                let v492 = (v488 * v52) + (v53 * v487);
                let v495 = (-v481) / v489;
                let v499 = v495.exp();
                let v502 = v499 + v501;
                let v503 = v502.ln();
                let v506 = v489 * v503;
                let v509 = (v492 * v503) + ((((((v482 * v71) - (v492 * v495)) / v489) * v499) * (v151 / v502)) * v489);
                v474 = v481;
                v475 = v487;
                v476 = v506;
                v477 = v482;
                v478 = v488;
                v479 = v509;
            } else {
                v474 = v469;
                v475 = v473;
                v476 = v2;
                v477 = v182;
                v478 = v182;
                v479 = v182;
            }
            let v512: f64;
            let v513: f64;
            let v514: f64;
            let v515: f64;
            let v516: f64;
            let v517: f64;
            let v518: f64;
            let v519: f64;
            let v520: f64;
            let v521: f64;
            if v480 != 0.0 {
                let v541: f64;
                let v542: f64;
                let v543: f64;
                let v544: f64;
                if v510 != 0.0 {
                    let v528 = v527 * v135;
                    let v530 = v528 * v92;
                    let v533 = ((v139 * v527) * v92) + (v93 * v528);
                    let v535 = v534 * v135;
                    let v537 = v535 * v92;
                    let v540 = ((v139 * v534) * v92) + (v93 * v535);
                    v541 = v530;
                    v542 = v537;
                    v543 = v533;
                    v544 = v540;
                } else {
                    v541 = v527;
                    v542 = v534;
                    v543 = v182;
                    v544 = v182;
                }
                let v546 = v543 * v541;
                let v549 = v548 * v542;
                let v557 = ((v541 * v541) + (v549 * v542)).sqrt();
                let v564 = v557 - (v561 * v542);
                let v565 = (((v546 + v546) + (((v544 * v548) * v542) + (v544 * v549))) * (v151 / (v316 * v557))) - (v544 * v561);
                let v569 = (v566 * v564) / v542;
                let v572 = ((v565 * v566) - (v544 * v569)) / v542;
                let v574 = v565 * v564;
                let v576 = v542 * v542;
                let v577 = v544 * v542;
                let v579 = (v564 * v564) / v576;
                let v587 = (v579 + (v311 * v569)).sqrt();
                let v590 = ((((v574 + v574) - ((v577 + v577) * v579)) / v576) + (v572 * v311)) * (v151 / (v316 * v587));
                let v591 = v542 - v541;
                let v592 = v544 - v543;
                let v593 = v2 / v542;
                let v596 = ((v544 * v593) * v71) / v542;
                v512 = v591;
                v513 = v593;
                v514 = v564;
                v515 = v569;
                v516 = v587;
                v517 = v592;
                v518 = v596;
                v519 = v565;
                v520 = v572;
                v521 = v590;
            } else {
                v512 = v511;
                v513 = v0;
                v514 = v0;
                v515 = v0;
                v516 = v0;
                v517 = v182;
                v518 = v182;
                v519 = v182;
                v520 = v182;
                v521 = v182;
            }
            let v523 = v522 * v512;
            let v524 = v517 * v522;
            let v526 = if v523 > v525 { 1.0 } else { 0.0 };
            let v597: f64;
            let v598: f64;
            if v526 != 0.0 {
                v597 = v525;
                v598 = v182;
            } else {
                v597 = v523;
                v598 = v524;
            }
            let v599 = if v12 < v0 { 1.0 } else { 0.0 };
            let v609: f64;
            let v610: f64;
            let v611: f64;
            let v612: Lanes<3>;
            let v613: Lanes<2>;
            if v599 != 0.0 {
                let v600 = -v26;
                let v601 = v27 * v71;
                let v602 = -v12;
                let v603 = v13 * v71;
                let v604 = Lanes([v601[0], 0.0, v601[1]]);
                v609 = v600;
                v610 = v602;
                v611 = v605;
                v612 = v604;
                v613 = v603;
            } else {
                let v606 = -v20;
                let v607 = v21 * v71;
                let v608 = Lanes([v607[0], v607[1], 0.0]);
                v609 = v606;
                v610 = v12;
                v611 = v2;
                v612 = v608;
                v613 = v13;
            }
            let v615 = if v609 > v614 { 1.0 } else { 0.0 };
            let v644: f64;
            let v645: Lanes<3>;
            if v615 != 0.0 {
                let v621 = ((v614 - v609) / v618).exp();
                let v623 = v2 + v621;
                let v629 = v614 - (v618 * (v623.ln()));
                let v630 = (((((v612 * v71) / v618) * v621) * (v151 / v623)) * v618) * v71;
                v644 = v629;
                v645 = v630;
            } else {
                let v634 = ((v609 - v614) / v618).exp();
                let v636 = v2 + v634;
                let v642 = v609 - (v618 * (v636.ln()));
                let v643 = v612 - ((((v612 / v618) * v634) * (v151 / v636)) * v618);
                v644 = v642;
                v645 = v643;
            }
            let v651: f64;
            let v652: Lanes<3>;
            if v94 != 0.0 {
                let v646 = v614 - v644;
                let v647 = v645 * v71;
                let v648 = if v610 < v646 { 1.0 } else { 0.0 };
                let v658: f64;
                if v648 != 0.0 {
                    v658 = v610;
                } else {
                    v658 = v646;
                }
                let v662 = if v644 < (v660 * (v655 + v658)) { 1.0 } else { 0.0 };
                let v663: f64;
                let v664: Lanes<3>;
                if v662 != 0.0 {
                    let v666: f64;
                    let v667: Lanes<3>;
                    if v648 != 0.0 {
                        let v665 = Lanes([0.0, v613[0], v613[1]]);
                        v666 = v610;
                        v667 = v665;
                    } else {
                        v666 = v646;
                        v667 = v647;
                    }
                    let v670 = v669 * (v655 + v666);
                    let v671 = v667 * v669;
                    v663 = v670;
                    v664 = v671;
                } else {
                    v663 = v644;
                    v664 = v645;
                }
                v651 = v663;
                v652 = v664;
            } else {
                let v650 = if v644 < v649 { 1.0 } else { 0.0 };
                let v675: f64;
                let v676: Lanes<3>;
                if v650 != 0.0 {
                    let v673 = v672 * v655;
                    v675 = v673;
                    v676 = v674;
                } else {
                    v675 = v644;
                    v676 = v645;
                }
                v651 = v675;
                v652 = v676;
            }
            let v654 = v652 * v249;
            let v656 = v655 + (v249 * v651);
            let v657 = if v513 > v0 { 1.0 } else { 0.0 };
            let v815: f64;
            let v816: Lanes<4>;
            if v657 != 0.0 {
                let v678 = v677 * v656;
                let v684 = (v678 * v656) - v656;
                let v686 = v283 * v677;
                let v690 = v689 + (v686 * v656);
                let v691 = v656 / v597;
                let v699 = v677 * (v697 + v691);
                let v703 = (v701 * v677) / v597;
                let v707 = v311 * v597;
                let v713 = (v707 * v597) / v677;
                let v714 = (((v598 * v311) * v597) + (v598 * v707)) / v677;
                let v715 = v684 * v713;
                let v716 = ((((v654 * v677) * v656) + (v654 * v678)) - v654) * v713;
                let v720 = (Lanes([v716[0], 0.0, v716[1], v716[2]])) + (Lanes([0.0, (v714 * v684), 0.0, 0.0]));
                let v721 = v690 * v713;
                let v722 = (v654 * v686) * v713;
                let v726 = (Lanes([v722[0], 0.0, v722[1], v722[2]])) + (Lanes([0.0, (v714 * v690), 0.0, 0.0]));
                let v727 = v699 * v713;
                let v731 = (((((Lanes([v654[0], 0.0, v654[1], v654[2]])) - (Lanes([0.0, (v598 * v691), 0.0, 0.0]))) / v597) * v677) * v713) + (Lanes([0.0, (v714 * v699), 0.0, 0.0]));
                let v732 = v703 * v713;
                let v735 = ((((v598 * v703) * v71) / v597) * v713) + (v714 * v703);
                let v736 = v732 * v732;
                let v737 = v735 * v732;
                let v738 = v737 + v737;
                let v739 = -v727;
                let v740 = v731 * v71;
                let v748 = (v732 * v721) - (v311 * v715);
                let v749 = ((Lanes([0.0, (v735 * v721), 0.0, 0.0])) + (v726 * v732)) - (v720 * v311);
                let v750 = v311 * v727;
                let v757 = v726 * v721;
                let v769 = v740 * v739;
                let v774 = v748 - ((v739 * v739) * v771);
                let v775 = v749 - ((v769 + v769) * v771);
                let v778 = v748 + (v249 * v774);
                let v787 = (((v750 * v715) - (v721 * v721)) - (v715 * v736)) - ((v739 * v778) / v784);
                let v788 = (((((v731 * v311) * v715) + (v720 * v750)) - (v757 + v757)) - ((v720 * v736) + (Lanes([0.0, (v738 * v715), 0.0, 0.0])))) - (((v740 * v778) + ((v749 + (v775 * v249)) * v739)) / v784);
                let v789 = v774 * v774;
                let v790 = v775 * v774;
                let v797 = (v789 * v774) / v796;
                let v798 = (((v790 + v790) * v774) + (v775 * v789)) / v796;
                let v800 = v799 * v787;
                let v808 = ((v800 * v787) + v797).sqrt();
                let v811 = ((((v788 * v799) * v787) + (v788 * v800)) + v798) * (v151 / (v316 * v808));
                let v812 = if v787 < v0 { 1.0 } else { 0.0 };
                let v840: f64;
                let v841: f64;
                let v842: Lanes<4>;
                let v843: Lanes<4>;
                if v812 != 0.0 {
                    let v821 = (v818 * v787) + v808;
                    let v822 = (v788 * v818) + v811;
                    let v825 = (-v797) / v821;
                    let v828 = ((v798 * v71) - (v822 * v825)) / v821;
                    v840 = v821;
                    v841 = v825;
                    v842 = v822;
                    v843 = v828;
                } else {
                    let v832 = (v829 * v787) - v808;
                    let v833 = (v788 * v829) - v811;
                    let v836 = (-v797) / v832;
                    let v839 = ((v798 * v71) - (v833 * v836)) / v832;
                    v840 = v836;
                    v841 = v832;
                    v842 = v839;
                    v843 = v833;
                }
                let v845 = if v840 > v844 { 1.0 } else { 0.0 };
                let v853: f64;
                let v854: Lanes<4>;
                if v845 != 0.0 {
                    let v846 = v840.powf(v771);
                    let v850 = v842 * (v771 * (v840.powf(v847)));
                    v853 = v846;
                    v854 = v850;
                } else {
                    let v852 = if v840 < v851 { 1.0 } else { 0.0 };
                    let v868: f64;
                    let v869: Lanes<4>;
                    if v852 != 0.0 {
                        let v856 = -v840;
                        let v863 = -(v856.powf(v771));
                        let v864 = ((v842 * v71) * (v771 * (v856.powf(v859)))) * v71;
                        v868 = v863;
                        v869 = v864;
                    } else {
                        let v866 = v865 * v840;
                        let v867 = v842 * v865;
                        v868 = v866;
                        v869 = v867;
                    }
                    v853 = v868;
                    v854 = v869;
                }
                let v855 = if v841 > v844 { 1.0 } else { 0.0 };
                let v877: f64;
                let v878: Lanes<4>;
                if v855 != 0.0 {
                    let v870 = v841.powf(v771);
                    let v874 = v843 * (v771 * (v841.powf(v871)));
                    v877 = v870;
                    v878 = v874;
                } else {
                    let v876 = if v841 < v875 { 1.0 } else { 0.0 };
                    let v943: f64;
                    let v944: Lanes<4>;
                    if v876 != 0.0 {
                        let v932 = -v841;
                        let v939 = -(v932.powf(v771));
                        let v940 = ((v843 * v71) * (v771 * (v932.powf(v935)))) * v71;
                        v943 = v939;
                        v944 = v940;
                    } else {
                        let v941 = v865 * v841;
                        let v942 = v843 * v865;
                        v943 = v941;
                        v944 = v942;
                    }
                    v877 = v943;
                    v878 = v944;
                }
                let v885 = v799 * v736;
                let v886 = v738 * v799;
                let v892 = ((v885 - v727) + ((v853 + v877) - (v739 * v771))).sqrt();
                let v895 = (((Lanes([0.0, v886, 0.0, 0.0])) - v731) + ((v854 + v878) - (v740 * v771))) * (v151 / (v316 * v892));
                let v900 = v895 * v892;
                let v907 = ((v896 * v736) - (v892 * v892)) - (v249 * v727);
                let v908 = ((Lanes([0.0, (v738 * v896), 0.0, 0.0])) - (v900 + v900)) - (v731 * v249);
                let v925 = (((v732 * v727) - (v249 * v721)) - (v885 * v732)) / v892;
                let v928 = (((((Lanes([0.0, (v735 * v727), 0.0, 0.0])) + (v731 * v732)) - (v726 * v249)) - (Lanes([0.0, ((v886 * v732) + (v735 * v885)), 0.0, 0.0]))) - (v895 * v925)) / v892;
                let v929 = v907 + v925;
                let v930 = v908 + v928;
                let v931 = if v929 > v0 { 1.0 } else { 0.0 };
                let v984: f64;
                let v985: Lanes<4>;
                if v931 != 0.0 {
                    let v945 = v929.sqrt();
                    let v956 = (v949 * v732) + (v321 * (v945 + v892));
                    let v958 = (Lanes([0.0, (v735 * v949), 0.0, 0.0])) + (((v930 * (v151 / (v316 * v945))) + v895) * v321);
                    v984 = v956;
                    v985 = v958;
                } else {
                    let v959 = v907 - v925;
                    let v962 = (v908 - v928) * v959;
                    let v966 = ((v959 * v959) + v964).sqrt();
                    let v970 = v966.sqrt();
                    let v981 = (v974 * v732) + (v321 * (v970 - v892));
                    let v983 = (Lanes([0.0, (v735 * v974), 0.0, 0.0])) + (((((v962 + v962) * (v151 / (v316 * v966))) * (v151 / (v316 * v970))) - v895) * v321);
                    v984 = v981;
                    v985 = v983;
                }
                v815 = v984;
                v816 = v985;
            } else {
                let v814 = if v651 > v813 { 1.0 } else { 0.0 };
                let v1035: f64;
                let v1036: Lanes<3>;
                if v814 != 0.0 {
                    let v987 = v986 - v651;
                    let v988 = v652 * v71;
                    let v989 = v677 * v987;
                    let v990 = v988 * v677;
                    let v995 = v249 * (v2 - (v249 * v989));
                    let v1009 = (v2 - (v701 * v989)).sqrt();
                    let v1013 = (v2 - (v283 * v989)) + v1009;
                    let v1015 = (v995 * v987) / v1013;
                    let v1018 = ((((((v990 * v249) * v71) * v249) * v987) + (v988 * v995)) - ((((v990 * v283) * v71) + (((v990 * v701) * v71) * (v151 / (v316 * v1009)))) * v1015)) / v1013;
                    v1035 = v1015;
                    v1036 = v1018;
                } else {
                    let v1019 = v283 * v677;
                    let v1020 = v1019 * v656;
                    let v1021 = v654 * v1019;
                    let v1025 = (v2 + v1020).sqrt();
                    let v1032 = v1031 * v677;
                    let v1033 = ((v2 - v1020) + v1025) / v1032;
                    let v1034 = ((v1021 * v71) + (v1021 * (v151 / (v316 * v1025)))) / v1032;
                    v1035 = v1033;
                    v1036 = v1034;
                }
                let v1037 = Lanes([v1036[0], 0.0, v1036[1], v1036[2]]);
                v815 = v1035;
                v816 = v1037;
            }
            let v1085: f64;
            let v1086: f64;
            let v1087: f64;
            let v1088: Lanes<4>;
            let v1089: Lanes<4>;
            let v1090: Lanes<4>;
            if v817 != 0.0 {
                let v1039 = v815 + v1038;
                let v1040 = v656 + v815;
                let v1042 = (Lanes([v654[0], 0.0, v654[1], v654[2]])) + v816;
                let v1043 = v1040.sqrt();
                let v1048 = v1047 * v1043;
                let v1049 = (v1042 * (v151 / (v316 * v1043))) * v1047;
                let v1204: f64;
                let v1205: Lanes<4>;
                if v657 != 0.0 {
                    let v1093 = v1039 / v522;
                    let v1094 = v816 / v522;
                    let v1096 = Lanes([0.0, v519, 0.0, 0.0]);
                    let v1098 = v321 * (v1093 - v514);
                    let v1100 = v1098 * v513;
                    let v1104 = (((v1094 - v1096) * v321) * v513) + (Lanes([0.0, (v518 * v1098), 0.0, 0.0]));
                    let v1107 = v321 * (v1093 + v514);
                    let v1109 = v1107 * v513;
                    let v1113 = (((v1094 + v1096) * v321) * v513) + (Lanes([0.0, (v518 * v1107), 0.0, 0.0]));
                    let v1115 = v1104 * v1100;
                    let v1118 = Lanes([0.0, v520, 0.0, 0.0]);
                    let v1120 = ((v1100 * v1100) + v515).sqrt();
                    let v1123 = ((v1115 + v1115) + v1118) * (v151 / (v316 * v1120));
                    let v1125 = v1113 * v1109;
                    let v1129 = ((v1109 * v1109) + v515).sqrt();
                    let v1132 = ((v1125 + v1125) + v1118) * (v151 / (v316 * v1129));
                    let v1138 = v1100 / v1120;
                    let v1142 = v1109 / v1129;
                    let v1148 = v321 * (v1138 + v1142);
                    let v1155 = (v1148 * v513) / v522;
                    let v1157 = v249 * v1048;
                    let v1159 = v2 - v1048;
                    let v1161 = v1157 * v1159;
                    let v1169 = v2 + ((v1120 + v1129) - v516);
                    let v1170 = (v1155 * v1039) / v1169;
                    let v1174 = v2 - v1170;
                    let v1180 = (v1161 * v1174) / v1039;
                    let v1184 = v1180.sqrt();
                    let v1187 = (((((((v1049 * v249) * v1159) + ((v1049 * v71) * v1157)) * v1174) + (((((((((((((v1104 - (v1123 * v1138)) / v1120) + ((v1113 - (v1132 * v1142)) / v1129)) * v321) * v513) + (Lanes([0.0, (v518 * v1148), 0.0, 0.0]))) / v522) * v1039) + (v816 * v1155)) - (((v1123 + v1132) - (Lanes([0.0, v521, 0.0, 0.0]))) * v1170)) / v1169) * v71) * v1161)) - (v816 * v1180)) / v1039) * (v151 / (v316 * v1184));
                    v1204 = v1184;
                    v1205 = v1187;
                } else {
                    let v1188 = v249 * v1048;
                    let v1190 = v2 - v1048;
                    let v1196 = (v1188 * v1190) / v1039;
                    let v1200 = v1196.sqrt();
                    let v1203 = (((((v1049 * v249) * v1190) + ((v1049 * v71) * v1188)) - (v816 * v1196)) / v1039) * (v151 / (v316 * v1200));
                    v1204 = v1200;
                    v1205 = v1203;
                }
                let v1208 = v1204 * v1204;
                let v1209 = v1205 * v1204;
                let v1211 = (v677 * v1040) / v1208;
                let v1215 = v1211 - v1039;
                let v1216 = (((v1042 * v677) - ((v1209 + v1209) * v1211)) / v1208) - v816;
                let v1220 = v1217 + v1039;
                let v1221 = (v1217 * v815) / v1220;
                let v1224 = ((v816 * v1217) - (v816 * v1221)) / v1220;
                let v1225 = v1063 + v1221;
                let v1226 = v311 * v1225;
                let v1228 = v1226 * v1225;
                let v1231 = ((v1224 * v311) * v1225) + (v1224 * v1226);
                let v1232 = v249 * v610;
                let v1234 = v1232 * v1039;
                let v1235 = (v613 * v249) * v1039;
                let v1238 = (Lanes([0.0, 0.0, v1235[0], v1235[1]])) + (v816 * v1232);
                let v1239 = v610 - v1039;
                let v1240 = Lanes([0.0, 0.0, v613[0], v613[1]]);
                let v1242 = v1239 * v1239;
                let v1243 = (v1240 - v816) * v1239;
                let v1244 = v1243 + v1243;
                let v1247 = (v1242 + v1228).sqrt();
                let v1251 = v610 + v1039;
                let v1253 = v1251 * v1251;
                let v1254 = (v1240 + v816) * v1251;
                let v1255 = v1254 + v1254;
                let v1258 = (v1253 + v1228).sqrt();
                let v1262 = v1247 + v1258;
                let v1264 = v1234 / v1262;
                let v1267 = (v1238 - ((((v1244 + v1231) * (v151 / (v316 * v1247))) + ((v1255 + v1231) * (v151 / (v316 * v1258)))) * v1264)) / v1262;
                let v1300: f64;
                let v1301: Lanes<4>;
                if v1268 != 0.0 {
                    let v1271 = (v1217 * v1264) / v1220;
                    let v1274 = ((v1267 * v1217) - (v816 * v1271)) / v1220;
                    let v1275 = v1063 + v1271;
                    let v1276 = v311 * v1275;
                    let v1278 = v1276 * v1275;
                    let v1281 = ((v1274 * v311) * v1275) + (v1274 * v1276);
                    let v1284 = (v1242 + v1278).sqrt();
                    let v1290 = (v1253 + v1278).sqrt();
                    let v1294 = v1284 + v1290;
                    let v1296 = v1234 / v1294;
                    let v1299 = (v1238 - ((((v1244 + v1281) * (v151 / (v316 * v1284))) + ((v1255 + v1281) * (v151 / (v316 * v1290)))) * v1296)) / v1294;
                    v1300 = v1296;
                    v1301 = v1299;
                } else {
                    v1300 = v1264;
                    v1301 = v1267;
                }
                let v1304 = (v1215 + v1300).sqrt();
                let v1312 = v2 - (v1204 * v1304);
                let v1313 = ((v1205 * v1304) + (((v1216 + v1301) * (v151 / (v316 * v1304))) * v1204)) * v71;
                let v1360: f64;
                let v1361: Lanes<4>;
                if v657 != 0.0 {
                    let v1314 = v1300 / v522;
                    let v1315 = v1301 / v522;
                    let v1317 = Lanes([0.0, v519, 0.0, 0.0]);
                    let v1319 = v321 * (v1314 - v514);
                    let v1321 = v1319 * v513;
                    let v1328 = v321 * (v1314 + v514);
                    let v1330 = v1328 * v513;
                    let v1336 = ((((v1315 - v1317) * v321) * v513) + (Lanes([0.0, (v518 * v1319), 0.0, 0.0]))) * v1321;
                    let v1339 = Lanes([0.0, v520, 0.0, 0.0]);
                    let v1341 = ((v1321 * v1321) + v515).sqrt();
                    let v1346 = ((((v1315 + v1317) * v321) * v513) + (Lanes([0.0, (v518 * v1328), 0.0, 0.0]))) * v1330;
                    let v1350 = ((v1330 * v1330) + v515).sqrt();
                    let v1356 = (v1341 + v1350) - v516;
                    let v1358 = ((((v1336 + v1336) + v1339) * (v151 / (v316 * v1341))) + (((v1346 + v1346) + v1339) * (v151 / (v316 * v1350)))) - (Lanes([0.0, v521, 0.0, 0.0]));
                    v1360 = v1356;
                    v1361 = v1358;
                } else {
                    v1360 = v0;
                    v1361 = v1359;
                }
                v1085 = v1312;
                v1086 = v1360;
                v1087 = v1300;
                v1088 = v1313;
                v1089 = v1361;
                v1090 = v1301;
            } else {
                let v1050 = v249 * v610;
                let v1053 = (v613 * v249) * v815;
                let v1057 = v610 - v815;
                let v1058 = Lanes([0.0, 0.0, v613[0], v613[1]]);
                let v1061 = (v1058 - v816) * v1057;
                let v1065 = ((v1057 * v1057) + v1063).sqrt();
                let v1069 = v610 + v815;
                let v1072 = (v1058 + v816) * v1069;
                let v1075 = ((v1069 * v1069) + v1063).sqrt();
                let v1079 = v1065 + v1075;
                let v1081 = (v1050 * v815) / v1079;
                let v1084 = (((Lanes([0.0, 0.0, v1053[0], v1053[1]])) + (v816 * v1050)) - ((((v1061 + v1061) * (v151 / (v316 * v1065))) + ((v1072 + v1072) * (v151 / (v316 * v1075)))) * v1081)) / v1079;
                let v1407: f64;
                let v1408: Lanes<4>;
                if v657 != 0.0 {
                    let v1362 = v1081 / v522;
                    let v1363 = v1084 / v522;
                    let v1365 = Lanes([0.0, v519, 0.0, 0.0]);
                    let v1367 = v321 * (v1362 - v514);
                    let v1369 = v1367 * v513;
                    let v1376 = v321 * (v1362 + v514);
                    let v1378 = v1376 * v513;
                    let v1384 = ((((v1363 - v1365) * v321) * v513) + (Lanes([0.0, (v518 * v1367), 0.0, 0.0]))) * v1369;
                    let v1387 = Lanes([0.0, v520, 0.0, 0.0]);
                    let v1389 = ((v1369 * v1369) + v515).sqrt();
                    let v1394 = ((((v1363 + v1365) * v321) * v513) + (Lanes([0.0, (v518 * v1376), 0.0, 0.0]))) * v1378;
                    let v1398 = ((v1378 * v1378) + v515).sqrt();
                    let v1404 = (v1389 + v1398) - v516;
                    let v1406 = ((((v1384 + v1384) + v1387) * (v151 / (v316 * v1389))) + (((v1394 + v1394) + v1387) * (v151 / (v316 * v1398)))) - (Lanes([0.0, v521, 0.0, 0.0]));
                    v1407 = v1404;
                    v1408 = v1406;
                } else {
                    v1407 = v0;
                    v1408 = v1359;
                }
                let v1412 = (v656 + v1081).sqrt();
                let v1418 = v2 - (v1047 * v1412);
                let v1419 = ((((Lanes([v654[0], 0.0, v654[1], v654[2]])) + v1084) * (v151 / (v316 * v1412))) * v1047) * v71;
                v1085 = v1418;
                v1086 = v1407;
                v1087 = v1081;
                v1088 = v1419;
                v1089 = v1408;
                v1090 = v1084;
            }
            let v1092 = if v1085 < v1091 { 1.0 } else { 0.0 };
            let v1420: f64;
            let v1421: Lanes<4>;
            if v1092 != 0.0 {
                v1420 = v1091;
                v1421 = v1359;
            } else {
                v1420 = v1085;
                v1421 = v1088;
            }
            let v1427 = v2 + v1086;
            let v1428 = (v109 * v1420) / v1427;
            let v1432 = v611 * v1428;
            let v1434 = v1432 * v1087;
            let v1437 = ((((((Lanes([0.0, (v110 * v1420), 0.0, 0.0])) + (v1421 * v109)) - (v1089 * v1428)) / v1427) * v611) * v1087) + (v1090 * v1432);
            let v1438 = if v236 > v0 { 1.0 } else { 0.0 };
            let v1441: f64;
            let v1442: Lanes<3>;
            if v1438 != 0.0 {
                let v1439 = if v231 > v0 { 1.0 } else { 0.0 };
                let v1451: f64;
                let v1452: Lanes<3>;
                if v1439 != 0.0 {
                    let v1444 = v159 * v52;
                    let v1446 = v2 / v1444;
                    let v1449 = (((v53 * v159) * v1446) * v71) / v1444;
                    let v1450 = if v20 < v184 { 1.0 } else { 0.0 };
                    let v1483: f64;
                    let v1484: Lanes<3>;
                    if v1450 != 0.0 {
                        let v1455 = v21 * v1446;
                        let v1460 = (v20 * v1446).exp();
                        let v1461 = ((Lanes([v1455[0], 0.0, v1455[1]])) + (Lanes([0.0, (v1449 * v20), 0.0]))) * v1460;
                        v1483 = v1460;
                        v1484 = v1461;
                    } else {
                        let v1466 = (v184 * v1446).exp();
                        let v1468 = v20 - v184;
                        let v1477 = v2 + (v1468 * v1446);
                        let v1478 = v1466 * v1477;
                        let v1482 = (Lanes([0.0, ((((v186 * v1446) + (v1449 * v184)) * v1466) * v1477), 0.0])) + (((((Lanes([v21[0], 0.0, v21[1]])) - (Lanes([0.0, v186, 0.0]))) * v1446) + (Lanes([0.0, (v1449 * v1468), 0.0]))) * v1466);
                        v1483 = v1478;
                        v1484 = v1482;
                    }
                    let v1485 = v1483 - v2;
                    let v1486 = v231 * v1485;
                    let v1490 = (Lanes([0.0, (v232 * v1485), 0.0])) + (v1484 * v231);
                    v1451 = v1486;
                    v1452 = v1490;
                } else {
                    v1451 = v0;
                    v1452 = v1440;
                }
                let v1453 = if v234 > v0 { 1.0 } else { 0.0 };
                let v1498: f64;
                let v1499: Lanes<3>;
                if v1453 != 0.0 {
                    let v1491 = v204 * v52;
                    let v1493 = v2 / v1491;
                    let v1496 = (((v53 * v204) * v1493) * v71) / v1491;
                    let v1497 = if v20 < v227 { 1.0 } else { 0.0 };
                    let v1532: f64;
                    let v1533: Lanes<3>;
                    if v1497 != 0.0 {
                        let v1504 = v21 * v1493;
                        let v1509 = (v20 * v1493).exp();
                        let v1510 = ((Lanes([v1504[0], 0.0, v1504[1]])) + (Lanes([0.0, (v1496 * v20), 0.0]))) * v1509;
                        v1532 = v1509;
                        v1533 = v1510;
                    } else {
                        let v1515 = (v227 * v1493).exp();
                        let v1517 = v20 - v227;
                        let v1526 = v2 + (v1517 * v1493);
                        let v1527 = v1515 * v1526;
                        let v1531 = (Lanes([0.0, ((((v229 * v1493) + (v1496 * v227)) * v1515) * v1526), 0.0])) + (((((Lanes([v21[0], 0.0, v21[1]])) - (Lanes([0.0, v229, 0.0]))) * v1493) + (Lanes([0.0, (v1496 * v1517), 0.0]))) * v1515);
                        v1532 = v1527;
                        v1533 = v1531;
                    }
                    let v1534 = v1532 - v2;
                    let v1535 = v234 * v1534;
                    let v1539 = (Lanes([0.0, (v235 * v1534), 0.0])) + (v1533 * v234);
                    v1498 = v1535;
                    v1499 = v1539;
                } else {
                    v1498 = v0;
                    v1499 = v1440;
                }
                let v1500 = v1451 + v1498;
                let v1501 = v1452 + v1499;
                let v1502 = if v474 > v0 { 1.0 } else { 0.0 };
                let v1555: f64;
                let v1556: Lanes<3>;
                if v1502 != 0.0 {
                    let v1540 = -v474;
                    let v1541 = v477 * v71;
                    let v1542 = v1540 - v20;
                    let v1545 = (Lanes([0.0, v1541, 0.0])) - (Lanes([v21[0], 0.0, v21[1]]));
                    let v1546 = v475 * v52;
                    let v1550 = v2 / v1546;
                    let v1553 = ((((v478 * v52) + (v53 * v475)) * v1550) * v71) / v1546;
                    let v1554 = if v1542 < v476 { 1.0 } else { 0.0 };
                    let v1591: f64;
                    let v1592: Lanes<3>;
                    if v1554 != 0.0 {
                        let v1569 = (v1542 * v1550).exp();
                        let v1570 = ((v1545 * v1550) + (Lanes([0.0, (v1553 * v1542), 0.0]))) * v1569;
                        v1591 = v1569;
                        v1592 = v1570;
                    } else {
                        let v1575 = (v476 * v1550).exp();
                        let v1577 = v1542 - v476;
                        let v1585 = v2 + (v1577 * v1550);
                        let v1586 = v1575 * v1585;
                        let v1590 = (Lanes([0.0, ((((v479 * v1550) + (v1553 * v476)) * v1575) * v1585), 0.0])) + ((((v1545 - (Lanes([0.0, v479, 0.0]))) * v1550) + (Lanes([0.0, (v1553 * v1577), 0.0]))) * v1575);
                        v1591 = v1586;
                        v1592 = v1590;
                    }
                    let v1594 = -v1593;
                    let v1599 = (v1540 * v1550).exp();
                    let v1604 = v1594 * (v1591 - v1599);
                    let v1605 = (v1592 - (Lanes([0.0, (((v1541 * v1550) + (v1553 * v1540)) * v1599), 0.0]))) * v1594;
                    v1555 = v1604;
                    v1556 = v1605;
                } else {
                    v1555 = v0;
                    v1556 = v1440;
                }
                let v1560 = v21 * v1;
                let v1561 = (v1500 + v1555) + (v1 * v20);
                let v1563 = (v1501 + v1556) + (Lanes([v1560[0], 0.0, v1560[1]]));
                v1441 = v1561;
                v1442 = v1563;
            } else {
                v1441 = v0;
                v1442 = v1440;
            }
            let v1443 = if v243 > v0 { 1.0 } else { 0.0 };
            let v1608: f64;
            let v1609: Lanes<3>;
            if v1443 != 0.0 {
                let v1606 = if v238 > v0 { 1.0 } else { 0.0 };
                let v1675: f64;
                let v1676: Lanes<3>;
                if v1606 != 0.0 {
                    let v1668 = v159 * v52;
                    let v1670 = v2 / v1668;
                    let v1673 = (((v53 * v159) * v1670) * v71) / v1668;
                    let v1674 = if v26 < v184 { 1.0 } else { 0.0 };
                    let v1707: f64;
                    let v1708: Lanes<3>;
                    if v1674 != 0.0 {
                        let v1679 = v27 * v1670;
                        let v1684 = (v26 * v1670).exp();
                        let v1685 = ((Lanes([v1679[0], 0.0, v1679[1]])) + (Lanes([0.0, (v1673 * v26), 0.0]))) * v1684;
                        v1707 = v1684;
                        v1708 = v1685;
                    } else {
                        let v1690 = (v184 * v1670).exp();
                        let v1692 = v26 - v184;
                        let v1701 = v2 + (v1692 * v1670);
                        let v1702 = v1690 * v1701;
                        let v1706 = (Lanes([0.0, ((((v186 * v1670) + (v1673 * v184)) * v1690) * v1701), 0.0])) + (((((Lanes([v27[0], 0.0, v27[1]])) - (Lanes([0.0, v186, 0.0]))) * v1670) + (Lanes([0.0, (v1673 * v1692), 0.0]))) * v1690);
                        v1707 = v1702;
                        v1708 = v1706;
                    }
                    let v1709 = v1707 - v2;
                    let v1710 = v238 * v1709;
                    let v1714 = (Lanes([0.0, (v239 * v1709), 0.0])) + (v1708 * v238);
                    v1675 = v1710;
                    v1676 = v1714;
                } else {
                    v1675 = v0;
                    v1676 = v1607;
                }
                let v1677 = if v241 > v0 { 1.0 } else { 0.0 };
                let v1722: f64;
                let v1723: Lanes<3>;
                if v1677 != 0.0 {
                    let v1715 = v204 * v52;
                    let v1717 = v2 / v1715;
                    let v1720 = (((v53 * v204) * v1717) * v71) / v1715;
                    let v1721 = if v26 < v227 { 1.0 } else { 0.0 };
                    let v1756: f64;
                    let v1757: Lanes<3>;
                    if v1721 != 0.0 {
                        let v1728 = v27 * v1717;
                        let v1733 = (v26 * v1717).exp();
                        let v1734 = ((Lanes([v1728[0], 0.0, v1728[1]])) + (Lanes([0.0, (v1720 * v26), 0.0]))) * v1733;
                        v1756 = v1733;
                        v1757 = v1734;
                    } else {
                        let v1739 = (v227 * v1717).exp();
                        let v1741 = v26 - v227;
                        let v1750 = v2 + (v1741 * v1717);
                        let v1751 = v1739 * v1750;
                        let v1755 = (Lanes([0.0, ((((v229 * v1717) + (v1720 * v227)) * v1739) * v1750), 0.0])) + (((((Lanes([v27[0], 0.0, v27[1]])) - (Lanes([0.0, v229, 0.0]))) * v1717) + (Lanes([0.0, (v1720 * v1741), 0.0]))) * v1739);
                        v1756 = v1751;
                        v1757 = v1755;
                    }
                    let v1758 = v1756 - v2;
                    let v1759 = v241 * v1758;
                    let v1763 = (Lanes([0.0, (v242 * v1758), 0.0])) + (v1757 * v241);
                    v1722 = v1759;
                    v1723 = v1763;
                } else {
                    v1722 = v0;
                    v1723 = v1607;
                }
                let v1724 = v1675 + v1722;
                let v1725 = v1676 + v1723;
                let v1726 = if v474 > v0 { 1.0 } else { 0.0 };
                let v1779: f64;
                let v1780: Lanes<3>;
                if v1726 != 0.0 {
                    let v1764 = -v474;
                    let v1765 = v477 * v71;
                    let v1766 = v1764 - v26;
                    let v1769 = (Lanes([0.0, v1765, 0.0])) - (Lanes([v27[0], 0.0, v27[1]]));
                    let v1770 = v475 * v52;
                    let v1774 = v2 / v1770;
                    let v1777 = ((((v478 * v52) + (v53 * v475)) * v1774) * v71) / v1770;
                    let v1778 = if v1766 < v476 { 1.0 } else { 0.0 };
                    let v1815: f64;
                    let v1816: Lanes<3>;
                    if v1778 != 0.0 {
                        let v1793 = (v1766 * v1774).exp();
                        let v1794 = ((v1769 * v1774) + (Lanes([0.0, (v1777 * v1766), 0.0]))) * v1793;
                        v1815 = v1793;
                        v1816 = v1794;
                    } else {
                        let v1799 = (v476 * v1774).exp();
                        let v1801 = v1766 - v476;
                        let v1809 = v2 + (v1801 * v1774);
                        let v1810 = v1799 * v1809;
                        let v1814 = (Lanes([0.0, ((((v479 * v1774) + (v1777 * v476)) * v1799) * v1809), 0.0])) + ((((v1769 - (Lanes([0.0, v479, 0.0]))) * v1774) + (Lanes([0.0, (v1777 * v1801), 0.0]))) * v1799);
                        v1815 = v1810;
                        v1816 = v1814;
                    }
                    let v1817 = -v1593;
                    let v1822 = (v1764 * v1774).exp();
                    let v1827 = v1817 * (v1815 - v1822);
                    let v1828 = (v1816 - (Lanes([0.0, (((v1765 * v1774) + (v1777 * v1764)) * v1822), 0.0]))) * v1817;
                    v1779 = v1827;
                    v1780 = v1828;
                } else {
                    v1779 = v0;
                    v1780 = v1607;
                }
                let v1784 = v27 * v1;
                let v1785 = (v1724 + v1779) + (v1 * v26);
                let v1787 = (v1725 + v1780) + (Lanes([v1784[0], 0.0, v1784[1]]));
                v1608 = v1785;
                v1609 = v1787;
            } else {
                v1608 = v0;
                v1609 = v1607;
            }
            let v1612 = v13 * v1434;
            let v1617 = v21 * v1441;
            let v1619 = (v1442 * v20) + (Lanes([v1617[0], 0.0, v1617[1]]));
            let v1625 = v27 * v1608;
            let v1627 = (v1609 * v26) + (Lanes([v1625[0], 0.0, v1625[1]]));
            let v1630 = (((v1437 * v12) + (Lanes([0.0, 0.0, v1612[0], v1612[1]]))) + (Lanes([v1619[0], v1619[1], v1619[2], 0.0]))) + (Lanes([v1627[0], v1627[1], 0.0, v1627[2]]));
            let v1632 = v1631 - v4;
            let v1636 = (Lanes([v1633, 0.0])) - (Lanes([0.0, v8]));
            let v1641 = v1636 * v1637;
            let v1644 = (Lanes([0.0, 0.0, (v1639 * v1632)])) + (Lanes([v1641[0], v1641[1], 0.0]));
            let v1648 = (Lanes([0.0, v1630[0], v1630[1], v1630[2], v1630[3], 0.0])) + (Lanes([v1644[0], 0.0, 0.0, v1644[1], 0.0, v1644[2]]));
            let v1650 = v1649 - v3;
            let v1654 = (Lanes([v1651, 0.0])) - (Lanes([0.0, v6]));
            let v1659 = v1654 * v1655;
            let v1662 = (Lanes([0.0, 0.0, (v1657 * v1650)])) + (Lanes([v1659[0], v1659[1], 0.0]));
            let v1663 = ((((v1434 * v12) + (v1441 * v20)) + (v1608 * v26)) + (v1637 * v1632)) + (v1655 * v1650);
            let v1666 = (Lanes([v1648[0], v1648[1], 0.0, v1648[2], v1648[3], v1648[4], v1648[5], 0.0])) + (Lanes([0.0, 0.0, v1662[0], 0.0, 0.0, v1662[1], 0.0, v1662[2]]));
            let v1836: f64;
            let v1837: f64;
            let v1838: f64;
            let v1839: Lanes<8>;
            if v1667 != 0.0 {
                let v1829 = -v1663;
                let v1830 = v1666 * v71;
                let v1854: f64;
                let v1855: f64;
                if v1831 != 0.0 {
                    let v1852 = v1851 * v29;
                    let v1853 = v39 * v1851;
                    v1854 = v1852;
                    v1855 = v1853;
                } else {
                    let v1885: f64;
                    let v1886: f64;
                    if v1856 != 0.0 {
                        let v1860 = v2 + (v29 / v1857);
                        let v1871 = (v1868 * ((v1860.powf(v1861)) - v2)) / v1861;
                        let v1872 = (((v39 / v1857) * (v1861 * (v1860.powf(v1863)))) * v1868) / v1861;
                        v1885 = v1871;
                        v1886 = v1872;
                    } else {
                        let v1873 = v1851 * v29;
                        let v1880 = v2 + ((v1875 * v29) / v1857);
                        let v1881 = v1873 * v1880;
                        let v1884 = ((v39 * v1851) * v1880) + (((v39 * v1875) / v1857) * v1873);
                        v1885 = v1881;
                        v1886 = v1884;
                    }
                    v1854 = v1885;
                    v1855 = v1886;
                }
                v1836 = v1854;
                v1837 = v1829;
                v1838 = v1855;
                v1839 = v1830;
            } else {
                let v1833 = v1832 * v29;
                let v1834 = v39 * v1832;
                v1836 = v1833;
                v1837 = v0;
                v1838 = v1834;
                v1839 = v1835;
            }
            let v1840 = v11 * v1434;
            let v1841 = v1437 * v11;
            let v1842 = v11 * v1441;
            let v1843 = v1442 * v11;
            let v1844 = v11 * v1608;
            let v1845 = v1609 * v11;
            let v1850 = if ((v1840 / v1846).abs()) > v1849 { 1.0 } else { 0.0 };
            let v1889 = if ((v1842 / v1846).abs()) > v1849 { 1.0 } else { 0.0 };
            let v1892 = if (v20.abs()) > v1891 { 1.0 } else { 0.0 };
            let v1895 = if ((v1844 / v1846).abs()) > v1849 { 1.0 } else { 0.0 };
            let v1897 = if (v26.abs()) > v1891 { 1.0 } else { 0.0 };
            let v1899: f64;
            let v1900: Lanes<3>;
            if v1898 != 0.0 {
                let v1917: f64;
                let v1918: Lanes<2>;
                if v94 != 0.0 {
                    let v1903 = v20 + v986;
                    let v1905 = v21 * v1903;
                    let v1909 = ((v1903 * v1903) + v1907).sqrt();
                    let v1915 = v321 * ((v20 - v986) + v1909);
                    let v1916 = (v21 + ((v1905 + v1905) * (v151 / (v316 * v1909)))) * v321;
                    v1917 = v1915;
                    v1918 = v1916;
                } else {
                    v1917 = v20;
                    v1918 = v21;
                }
                let v1919 = v230 * v347;
                let v1920 = v349 * v230;
                let v1921 = v233 * v448;
                let v1922 = v450 * v233;
                let v1923 = if v1919 > v0 { 1.0 } else { 0.0 };
                let v1931: f64;
                let v1932: Lanes<3>;
                if v1923 != 0.0 {
                    let v1924 = -v348;
                    let v1925 = v350 * v71;
                    let v1927 = v1924 * v1926;
                    let v1928 = v1925 * v1926;
                    let v1930 = if v1929 <= v0 { 1.0 } else { 0.0 };
                    let v2019: f64;
                    let v2020: Lanes<3>;
                    if v1930 != 0.0 {
                        let v1934 = v1917 + v1927;
                        let v1935 = Lanes([v1918[0], 0.0, v1918[1]]);
                        let v1937 = v1935 + (Lanes([0.0, v1928, 0.0]));
                        let v1938 = if v1934 > v0 { 1.0 } else { 0.0 };
                        let v2071: f64;
                        let v2072: f64;
                        let v2073: Lanes<3>;
                        let v2074: Lanes<3>;
                        if v1938 != 0.0 {
                            let v2021 = v2 - v1926;
                            let v2023 = v2021.powf((-v338));
                            let v2025 = v2 - (v2023 * v2021);
                            let v2028 = v2 - v338;
                            let v2029 = (v348 * v2025) / v2028;
                            let v2031 = v321 * v338;
                            let v2034 = v348 * v2021;
                            let v2036 = (v2031 * v1934) / v2034;
                            let v2041 = v2 + v2036;
                            let v2046 = (v1934 * v2041) * v2023;
                            let v2047 = ((v1937 * v2041) + ((((v1937 * v2031) - (Lanes([0.0, ((v350 * v2021) * v2036), 0.0]))) / v2034) * v1934)) * v2023;
                            let v2048 = Lanes([0.0, ((v350 * v2025) / v2028), 0.0]);
                            v2071 = v2029;
                            v2072 = v2046;
                            v2073 = v2048;
                            v2074 = v2047;
                        } else {
                            let v2049 = v1917 / v348;
                            let v2054 = v2 - v2049;
                            let v2056 = v2 - v338;
                            let v2062 = v2 - (v2054.powf(v2056));
                            let v2069 = (v348 * v2062) / v2056;
                            let v2070 = ((Lanes([0.0, (v350 * v2062), 0.0])) + ((((((v1935 - (Lanes([0.0, (v350 * v2049), 0.0]))) / v348) * v71) * (v2056 * (v2054.powf((v2056 - v151))))) * v71) * v348)) / v2056;
                            v2071 = v2069;
                            v2072 = v0;
                            v2073 = v2070;
                            v2074 = v1440;
                        }
                        let v2075 = v2071 + v2072;
                        let v2076 = v2073 + v2074;
                        v2019 = v2075;
                        v2020 = v2076;
                    } else {
                        let v1940 = v1928 * v1927;
                        let v1943 = (v311 * v1929) * v1929;
                        let v1945 = ((v1927 * v1927) + v1943).sqrt();
                        let v1954 = v1917 + v1927;
                        let v1955 = Lanes([v1918[0], 0.0, v1918[1]]);
                        let v1956 = Lanes([0.0, v1928, 0.0]);
                        let v1957 = v1955 + v1956;
                        let v1959 = v1957 * v1954;
                        let v1962 = ((v1954 * v1954) + v1943).sqrt();
                        let v1970 = (v321 * (v1954 - v1962)) - v1927;
                        let v1971 = ((v1957 - ((v1959 + v1959) * (v151 / (v316 * v1962)))) * v321) - v1956;
                        let v1972 = v1970 / v348;
                        let v1977 = v2 - v1972;
                        let v1979 = v2 - v338;
                        let v1980 = v1977.powf(v1979);
                        let v1992 = v2 - v1926;
                        let v1994 = v1992.powf((-v338));
                        let v1997 = (v1917 - v1970) + (v1951 * (v1927 + v1945));
                        let v1999 = (v1955 - v1971) + (Lanes([0.0, ((v1928 + ((v1940 + v1940) * (v151 / (v316 * v1945)))) * v1951), 0.0]));
                        let v2000 = v1994 * v1997;
                        let v2002 = v321 * v338;
                        let v2005 = v348 * v1992;
                        let v2007 = (v2002 * v1997) / v2005;
                        let v2012 = v2 + v2007;
                        let v2017 = ((v1924 * v1980) / v1979) + (v2000 * v2012);
                        let v2018 = (((Lanes([0.0, (v1925 * v1980), 0.0])) + (((((v1971 - (Lanes([0.0, (v350 * v1972), 0.0]))) / v348) * v71) * (v1979 * (v1977.powf((v1979 - v151))))) * v1924)) / v1979) + (((v1999 * v1994) * v2012) + ((((v1999 * v2002) - (Lanes([0.0, ((v350 * v1992) * v2007), 0.0]))) / v2005) * v2000));
                        v2019 = v2017;
                        v2020 = v2018;
                    }
                    v1931 = v2019;
                    v1932 = v2020;
                } else {
                    v1931 = v0;
                    v1932 = v1440;
                }
                let v1933 = if v1921 > v0 { 1.0 } else { 0.0 };
                let v2083: f64;
                let v2084: Lanes<3>;
                if v1933 != 0.0 {
                    let v2077 = -v449;
                    let v2078 = v451 * v71;
                    let v2079 = v2077 * v1926;
                    let v2080 = v2078 * v1926;
                    let v2082 = if v2081 <= v0 { 1.0 } else { 0.0 };
                    let v2182: f64;
                    let v2183: Lanes<3>;
                    if v2082 != 0.0 {
                        let v2097 = v1917 + v2079;
                        let v2098 = Lanes([v1918[0], 0.0, v1918[1]]);
                        let v2100 = v2098 + (Lanes([0.0, v2080, 0.0]));
                        let v2101 = if v2097 > v0 { 1.0 } else { 0.0 };
                        let v2234: f64;
                        let v2235: f64;
                        let v2236: Lanes<3>;
                        let v2237: Lanes<3>;
                        if v2101 != 0.0 {
                            let v2184 = v2 - v1926;
                            let v2186 = v2184.powf((-v439));
                            let v2188 = v2 - (v2186 * v2184);
                            let v2191 = v2 - v439;
                            let v2192 = (v449 * v2188) / v2191;
                            let v2194 = v321 * v439;
                            let v2197 = v449 * v2184;
                            let v2199 = (v2194 * v2097) / v2197;
                            let v2204 = v2 + v2199;
                            let v2209 = (v2097 * v2204) * v2186;
                            let v2210 = ((v2100 * v2204) + ((((v2100 * v2194) - (Lanes([0.0, ((v451 * v2184) * v2199), 0.0]))) / v2197) * v2097)) * v2186;
                            let v2211 = Lanes([0.0, ((v451 * v2188) / v2191), 0.0]);
                            v2234 = v2192;
                            v2235 = v2209;
                            v2236 = v2211;
                            v2237 = v2210;
                        } else {
                            let v2212 = v1917 / v449;
                            let v2217 = v2 - v2212;
                            let v2219 = v2 - v439;
                            let v2225 = v2 - (v2217.powf(v2219));
                            let v2232 = (v449 * v2225) / v2219;
                            let v2233 = ((Lanes([0.0, (v451 * v2225), 0.0])) + ((((((v2098 - (Lanes([0.0, (v451 * v2212), 0.0]))) / v449) * v71) * (v2219 * (v2217.powf((v2219 - v151))))) * v71) * v449)) / v2219;
                            v2234 = v2232;
                            v2235 = v0;
                            v2236 = v2233;
                            v2237 = v1440;
                        }
                        let v2238 = v2234 + v2235;
                        let v2239 = v2236 + v2237;
                        v2182 = v2238;
                        v2183 = v2239;
                    } else {
                        let v2103 = v2080 * v2079;
                        let v2106 = (v311 * v2081) * v2081;
                        let v2108 = ((v2079 * v2079) + v2106).sqrt();
                        let v2117 = v1917 + v2079;
                        let v2118 = Lanes([v1918[0], 0.0, v1918[1]]);
                        let v2119 = Lanes([0.0, v2080, 0.0]);
                        let v2120 = v2118 + v2119;
                        let v2122 = v2120 * v2117;
                        let v2125 = ((v2117 * v2117) + v2106).sqrt();
                        let v2133 = (v321 * (v2117 - v2125)) - v2079;
                        let v2134 = ((v2120 - ((v2122 + v2122) * (v151 / (v316 * v2125)))) * v321) - v2119;
                        let v2135 = v2133 / v449;
                        let v2140 = v2 - v2135;
                        let v2142 = v2 - v439;
                        let v2143 = v2140.powf(v2142);
                        let v2155 = v2 - v1926;
                        let v2157 = v2155.powf((-v439));
                        let v2160 = (v1917 - v2133) + (v2114 * (v2079 + v2108));
                        let v2162 = (v2118 - v2134) + (Lanes([0.0, ((v2080 + ((v2103 + v2103) * (v151 / (v316 * v2108)))) * v2114), 0.0]));
                        let v2163 = v2157 * v2160;
                        let v2165 = v321 * v439;
                        let v2168 = v449 * v2155;
                        let v2170 = (v2165 * v2160) / v2168;
                        let v2175 = v2 + v2170;
                        let v2180 = ((v2077 * v2143) / v2142) + (v2163 * v2175);
                        let v2181 = (((Lanes([0.0, (v2078 * v2143), 0.0])) + (((((v2134 - (Lanes([0.0, (v451 * v2135), 0.0]))) / v449) * v71) * (v2142 * (v2140.powf((v2142 - v151))))) * v2077)) / v2142) + (((v2162 * v2157) * v2175) + ((((v2162 * v2165) - (Lanes([0.0, ((v451 * v2155) * v2170), 0.0]))) / v2168) * v2163));
                        v2182 = v2180;
                        v2183 = v2181;
                    }
                    v2083 = v2182;
                    v2084 = v2183;
                } else {
                    v2083 = v0;
                    v2084 = v1440;
                }
                let v2095 = (v1919 * v1931) + (v1921 * v2083);
                let v2096 = ((Lanes([0.0, (v1920 * v1931), 0.0])) + (v1932 * v1919)) + ((Lanes([0.0, (v1922 * v2083), 0.0])) + (v2084 * v1921));
                v1899 = v2095;
                v1900 = v2096;
            } else {
                v1899 = v0;
                v1900 = v1440;
            }
            let v2240: f64;
            let v2241: Lanes<3>;
            if v1901 != 0.0 {
                let v2276: f64;
                let v2277: Lanes<2>;
                if v94 != 0.0 {
                    let v2263 = v26 + v986;
                    let v2265 = v27 * v2263;
                    let v2268 = ((v2263 * v2263) + v1907).sqrt();
                    let v2274 = v321 * ((v26 - v986) + v2268);
                    let v2275 = (v27 + ((v2265 + v2265) * (v151 / (v316 * v2268)))) * v321;
                    v2276 = v2274;
                    v2277 = v2275;
                } else {
                    v2276 = v26;
                    v2277 = v27;
                }
                let v2278 = v237 * v347;
                let v2279 = v349 * v237;
                let v2280 = v240 * v448;
                let v2281 = v450 * v240;
                let v2282 = if v2278 > v0 { 1.0 } else { 0.0 };
                let v2288: f64;
                let v2289: Lanes<3>;
                if v2282 != 0.0 {
                    let v2283 = -v348;
                    let v2284 = v350 * v71;
                    let v2285 = v2283 * v1926;
                    let v2286 = v2284 * v1926;
                    let v2287 = if v1929 <= v0 { 1.0 } else { 0.0 };
                    let v2376: f64;
                    let v2377: Lanes<3>;
                    if v2287 != 0.0 {
                        let v2291 = v2276 + v2285;
                        let v2292 = Lanes([v2277[0], 0.0, v2277[1]]);
                        let v2294 = v2292 + (Lanes([0.0, v2286, 0.0]));
                        let v2295 = if v2291 > v0 { 1.0 } else { 0.0 };
                        let v2428: f64;
                        let v2429: f64;
                        let v2430: Lanes<3>;
                        let v2431: Lanes<3>;
                        if v2295 != 0.0 {
                            let v2378 = v2 - v1926;
                            let v2380 = v2378.powf((-v338));
                            let v2382 = v2 - (v2380 * v2378);
                            let v2385 = v2 - v338;
                            let v2386 = (v348 * v2382) / v2385;
                            let v2388 = v321 * v338;
                            let v2391 = v348 * v2378;
                            let v2393 = (v2388 * v2291) / v2391;
                            let v2398 = v2 + v2393;
                            let v2403 = (v2291 * v2398) * v2380;
                            let v2404 = ((v2294 * v2398) + ((((v2294 * v2388) - (Lanes([0.0, ((v350 * v2378) * v2393), 0.0]))) / v2391) * v2291)) * v2380;
                            let v2405 = Lanes([0.0, ((v350 * v2382) / v2385), 0.0]);
                            v2428 = v2386;
                            v2429 = v2403;
                            v2430 = v2405;
                            v2431 = v2404;
                        } else {
                            let v2406 = v2276 / v348;
                            let v2411 = v2 - v2406;
                            let v2413 = v2 - v338;
                            let v2419 = v2 - (v2411.powf(v2413));
                            let v2426 = (v348 * v2419) / v2413;
                            let v2427 = ((Lanes([0.0, (v350 * v2419), 0.0])) + ((((((v2292 - (Lanes([0.0, (v350 * v2406), 0.0]))) / v348) * v71) * (v2413 * (v2411.powf((v2413 - v151))))) * v71) * v348)) / v2413;
                            v2428 = v2426;
                            v2429 = v0;
                            v2430 = v2427;
                            v2431 = v1607;
                        }
                        let v2432 = v2428 + v2429;
                        let v2433 = v2430 + v2431;
                        v2376 = v2432;
                        v2377 = v2433;
                    } else {
                        let v2297 = v2286 * v2285;
                        let v2300 = (v311 * v1929) * v1929;
                        let v2302 = ((v2285 * v2285) + v2300).sqrt();
                        let v2311 = v2276 + v2285;
                        let v2312 = Lanes([v2277[0], 0.0, v2277[1]]);
                        let v2313 = Lanes([0.0, v2286, 0.0]);
                        let v2314 = v2312 + v2313;
                        let v2316 = v2314 * v2311;
                        let v2319 = ((v2311 * v2311) + v2300).sqrt();
                        let v2327 = (v321 * (v2311 - v2319)) - v2285;
                        let v2328 = ((v2314 - ((v2316 + v2316) * (v151 / (v316 * v2319)))) * v321) - v2313;
                        let v2329 = v2327 / v348;
                        let v2334 = v2 - v2329;
                        let v2336 = v2 - v338;
                        let v2337 = v2334.powf(v2336);
                        let v2349 = v2 - v1926;
                        let v2351 = v2349.powf((-v338));
                        let v2354 = (v2276 - v2327) + (v2308 * (v2285 + v2302));
                        let v2356 = (v2312 - v2328) + (Lanes([0.0, ((v2286 + ((v2297 + v2297) * (v151 / (v316 * v2302)))) * v2308), 0.0]));
                        let v2357 = v2351 * v2354;
                        let v2359 = v321 * v338;
                        let v2362 = v348 * v2349;
                        let v2364 = (v2359 * v2354) / v2362;
                        let v2369 = v2 + v2364;
                        let v2374 = ((v2283 * v2337) / v2336) + (v2357 * v2369);
                        let v2375 = (((Lanes([0.0, (v2284 * v2337), 0.0])) + (((((v2328 - (Lanes([0.0, (v350 * v2329), 0.0]))) / v348) * v71) * (v2336 * (v2334.powf((v2336 - v151))))) * v2283)) / v2336) + (((v2356 * v2351) * v2369) + ((((v2356 * v2359) - (Lanes([0.0, ((v350 * v2349) * v2364), 0.0]))) / v2362) * v2357));
                        v2376 = v2374;
                        v2377 = v2375;
                    }
                    v2288 = v2376;
                    v2289 = v2377;
                } else {
                    v2288 = v0;
                    v2289 = v1607;
                }
                let v2290 = if v2280 > v0 { 1.0 } else { 0.0 };
                let v2439: f64;
                let v2440: Lanes<3>;
                if v2290 != 0.0 {
                    let v2434 = -v449;
                    let v2435 = v451 * v71;
                    let v2436 = v2434 * v1926;
                    let v2437 = v2435 * v1926;
                    let v2438 = if v2081 <= v0 { 1.0 } else { 0.0 };
                    let v2538: f64;
                    let v2539: Lanes<3>;
                    if v2438 != 0.0 {
                        let v2453 = v2276 + v2436;
                        let v2454 = Lanes([v2277[0], 0.0, v2277[1]]);
                        let v2456 = v2454 + (Lanes([0.0, v2437, 0.0]));
                        let v2457 = if v2453 > v0 { 1.0 } else { 0.0 };
                        let v2590: f64;
                        let v2591: f64;
                        let v2592: Lanes<3>;
                        let v2593: Lanes<3>;
                        if v2457 != 0.0 {
                            let v2540 = v2 - v1926;
                            let v2542 = v2540.powf((-v439));
                            let v2544 = v2 - (v2542 * v2540);
                            let v2547 = v2 - v439;
                            let v2548 = (v449 * v2544) / v2547;
                            let v2550 = v321 * v439;
                            let v2553 = v449 * v2540;
                            let v2555 = (v2550 * v2453) / v2553;
                            let v2560 = v2 + v2555;
                            let v2565 = (v2453 * v2560) * v2542;
                            let v2566 = ((v2456 * v2560) + ((((v2456 * v2550) - (Lanes([0.0, ((v451 * v2540) * v2555), 0.0]))) / v2553) * v2453)) * v2542;
                            let v2567 = Lanes([0.0, ((v451 * v2544) / v2547), 0.0]);
                            v2590 = v2548;
                            v2591 = v2565;
                            v2592 = v2567;
                            v2593 = v2566;
                        } else {
                            let v2568 = v2276 / v449;
                            let v2573 = v2 - v2568;
                            let v2575 = v2 - v439;
                            let v2581 = v2 - (v2573.powf(v2575));
                            let v2588 = (v449 * v2581) / v2575;
                            let v2589 = ((Lanes([0.0, (v451 * v2581), 0.0])) + ((((((v2454 - (Lanes([0.0, (v451 * v2568), 0.0]))) / v449) * v71) * (v2575 * (v2573.powf((v2575 - v151))))) * v71) * v449)) / v2575;
                            v2590 = v2588;
                            v2591 = v0;
                            v2592 = v2589;
                            v2593 = v1607;
                        }
                        let v2594 = v2590 + v2591;
                        let v2595 = v2592 + v2593;
                        v2538 = v2594;
                        v2539 = v2595;
                    } else {
                        let v2459 = v2437 * v2436;
                        let v2462 = (v311 * v2081) * v2081;
                        let v2464 = ((v2436 * v2436) + v2462).sqrt();
                        let v2473 = v2276 + v2436;
                        let v2474 = Lanes([v2277[0], 0.0, v2277[1]]);
                        let v2475 = Lanes([0.0, v2437, 0.0]);
                        let v2476 = v2474 + v2475;
                        let v2478 = v2476 * v2473;
                        let v2481 = ((v2473 * v2473) + v2462).sqrt();
                        let v2489 = (v321 * (v2473 - v2481)) - v2436;
                        let v2490 = ((v2476 - ((v2478 + v2478) * (v151 / (v316 * v2481)))) * v321) - v2475;
                        let v2491 = v2489 / v449;
                        let v2496 = v2 - v2491;
                        let v2498 = v2 - v439;
                        let v2499 = v2496.powf(v2498);
                        let v2511 = v2 - v1926;
                        let v2513 = v2511.powf((-v439));
                        let v2516 = (v2276 - v2489) + (v2470 * (v2436 + v2464));
                        let v2518 = (v2474 - v2490) + (Lanes([0.0, ((v2437 + ((v2459 + v2459) * (v151 / (v316 * v2464)))) * v2470), 0.0]));
                        let v2519 = v2513 * v2516;
                        let v2521 = v321 * v439;
                        let v2524 = v449 * v2511;
                        let v2526 = (v2521 * v2516) / v2524;
                        let v2531 = v2 + v2526;
                        let v2536 = ((v2434 * v2499) / v2498) + (v2519 * v2531);
                        let v2537 = (((Lanes([0.0, (v2435 * v2499), 0.0])) + (((((v2490 - (Lanes([0.0, (v451 * v2491), 0.0]))) / v449) * v71) * (v2498 * (v2496.powf((v2498 - v151))))) * v2434)) / v2498) + (((v2518 * v2513) * v2531) + ((((v2518 * v2521) - (Lanes([0.0, ((v451 * v2511) * v2526), 0.0]))) / v2524) * v2519));
                        v2538 = v2536;
                        v2539 = v2537;
                    }
                    v2439 = v2538;
                    v2440 = v2539;
                } else {
                    v2439 = v0;
                    v2440 = v1607;
                }
                let v2451 = (v2278 * v2288) + (v2280 * v2439);
                let v2452 = ((Lanes([0.0, (v2279 * v2288), 0.0])) + (v2289 * v2278)) + ((Lanes([0.0, (v2281 * v2439), 0.0])) + (v2440 * v2280));
                v2240 = v2451;
                v2241 = v2452;
            } else {
                v2240 = v0;
                v2241 = v1607;
            }
            let v2244 = v21 * v2242;
            let v2250 = v27 * v2248;
            let v2254 = v11 * (v1899 + (v2242 * v20));
            let v2255 = (v1900 + (Lanes([v2244[0], 0.0, v2244[1]]))) * v11;
            let v2256 = v11 * (v2240 + (v2248 * v26));
            let v2257 = (v2241 + (Lanes([v2250[0], 0.0, v2250[1]]))) * v11;
            let v2259 = v29 * v2258;
            let v2260 = v39 * v2258;
            let v2615: f64;
            let v2616: f64;
            let v2617: Lanes<2>;
            let v2618: Lanes<3>;
            if v2261 != 0.0 {
                let v2597 = v1637 * v2596;
                let v2599 = v2597 * v132;
                let v2604 = (Lanes([0.0, ((v1639 * v2596) * v132)])) + (Lanes([(v133 * v2597), 0.0]));
                v2615 = v2599;
                v2616 = v0;
                v2617 = v2604;
                v2618 = v2605;
            } else {
                let v2606 = v2596 * v132;
                let v2608 = v1632 / v2606;
                let v2613 = ((Lanes([v1636[0], 0.0, v1636[1]])) - (Lanes([0.0, ((v133 * v2596) * v2608), 0.0]))) / v2606;
                v2615 = v0;
                v2616 = v2608;
                v2617 = v2614;
                v2618 = v2613;
            }
            let v2639: f64;
            let v2640: f64;
            let v2641: Lanes<2>;
            let v2642: Lanes<3>;
            if v2619 != 0.0 {
                let v2621 = v1655 * v2620;
                let v2623 = v2621 * v132;
                let v2628 = (Lanes([0.0, ((v1657 * v2620) * v132)])) + (Lanes([(v133 * v2621), 0.0]));
                v2639 = v2623;
                v2640 = v0;
                v2641 = v2628;
                v2642 = v2629;
            } else {
                let v2630 = v2620 * v132;
                let v2632 = v1650 / v2630;
                let v2637 = ((Lanes([v1654[0], 0.0, v1654[1]])) - (Lanes([0.0, ((v133 * v2620) * v2632), 0.0]))) / v2630;
                v2639 = v0;
                v2640 = v2632;
                v2641 = v2638;
                v2642 = v2637;
            }
            let v2643 = ddt(8156, v2254);
            let v2645 = v2255 * v2644;
            let v2646 = ddt(8158, v2256);
            let v2647 = v2257 * v2644;
            let v2648 = ddt(8160, v2259);
            let v2649 = v2260 * v2644;
            let v2651: f64;
            let v2652: f64;
            if v2650 != 0.0 {
                let v2653 = if v1840 < v0 { 1.0 } else { 0.0 };
                let v2655: f64;
                if v1438 != 0.0 {
                    v2655 = v2654;
                } else {
                    v2655 = v0;
                }
                let v2657: f64;
                if v1443 != 0.0 {
                    v2657 = v2656;
                } else {
                    v2657 = v0;
                }
                v2651 = v2655;
                v2652 = v2657;
            } else {
                v2651 = v0;
                v2652 = v0;
            }
            let v2658 = v1841[3];
            let v2659 = v1841[1];
            let v2668 = if ((v2658 + ((v2659 * v2660) * (v1840 + (v12 * v2658)))).abs()) > v2667 { 1.0 } else { 0.0 };
            let v2669 = v1841[0];
            let v2670 = v1841[2];
            let v2671 = v1843[0];
            let v2672 = v1843[1];
            let v2673 = v1843[2];
            let v2674 = v1845[0];
            let v2675 = v1845[1];
            let v2676 = v1845[2];
            let v2677 = v1838;
            let v2678 = v1839[0];
            let v2679 = v1839[1];
            let v2680 = v1839[2];
            let v2681 = v1839[3];
            let v2682 = v1839[4];
            let v2683 = v1839[5];
            let v2684 = v1839[6];
            let v2685 = v1839[7];
            let v2686 = v2617[0];
            let v2687 = v2617[1];
            let v2688 = v2618[0];
            let v2689 = v2618[1];
            let v2690 = v2618[2];
            let v2691 = v2641[0];
            let v2692 = v2641[1];
            let v2693 = v2642[0];
            let v2694 = v2642[1];
            let v2695 = v2642[2];
            let v2696 = v2645[0];
            let v2697 = v2645[1];
            let v2698 = v2645[2];
            let v2699 = v2647[0];
            let v2700 = v2647[1];
            let v2701 = v2647[2];
            let v2702 = v2649;
            let v2703 = v2255[0];
            let v2704 = v2255[1];
            let v2705 = v2255[2];
            let v2706 = v2257[0];
            let v2707 = v2257[1];
            let v2708 = v2257[2];
            let v2709 = v2260;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (v1840),
            [1, 3, 4, 5],
            [v2669, v2659, v2670, v2658],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(4),
            multiplicity * (v1842),
            [1, 3, 4],
            [v2671, v2672, v2673],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(5),
            multiplicity * (v1844),
            [1, 3, 5],
            [v2674, v2675, v2676],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (v1836),
            [3],
            [v2677],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 2>(
            Some(3),
            None,
            multiplicity * (v1837),
            [0, 1, 2, 3, 4, 5],
            [v2678, v2679, v2680, v2681, v2682, v2683],
            [0, 1],
            [v2684, v2685],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(0), Some(4), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<1, 1>(
            0,
            v2615,
            [3],
            [v2686],
            [0],
            [v2687],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(0),
            Some(4),
            multiplicity * (v2616),
            [0, 3, 4],
            [v2688, v2689, v2690],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(2), Some(5), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<1, 1>(
            1,
            v2639,
            [3],
            [v2691],
            [1],
            [v2692],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(2),
            Some(5),
            multiplicity * (v2640),
            [2, 3, 5],
            [v2693, v2694, v2695],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(4),
            multiplicity * (v2643),
            [1, 3, 4],
            [v2696, v2697, v2698],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(5),
            multiplicity * (v2646),
            [1, 3, 5],
            [v2699, v2700, v2701],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (v2648),
            [3],
            [v2702],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(4),
            multiplicity * (staged[111]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(4),
            multiplicity * (staged[112]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(4),
            multiplicity * (staged[113]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(2),
            Some(5),
            multiplicity * (staged[114]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(4),
            multiplicity * (v2651),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(5),
            multiplicity * (v2652),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = v1840;
        self.canonical_reactive[1] = v1842;
        self.canonical_reactive[2] = v1844;
        self.canonical_reactive[3] = v1836;
        self.canonical_reactive[4] = v1837;
        self.canonical_reactive[5] = v2615;
        self.canonical_reactive[6] = v2616;
        self.canonical_reactive[7] = v2639;
        self.canonical_reactive[8] = v2640;
        self.canonical_reactive[9] = v2254;
        self.canonical_reactive[10] = v2703;
        self.canonical_reactive[11] = v2704;
        self.canonical_reactive[12] = v2705;
        self.canonical_reactive[13] = v2256;
        self.canonical_reactive[14] = v2706;
        self.canonical_reactive[15] = v2707;
        self.canonical_reactive[16] = v2708;
        self.canonical_reactive[17] = v2259;
        self.canonical_reactive[18] = v2709;
        self.canonical_reactive[19] = staged[111];
        self.canonical_reactive[20] = staged[112];
        self.canonical_reactive[21] = staged[113];
        self.canonical_reactive[22] = staged[114];
        self.canonical_reactive[23] = v2651;
        self.canonical_reactive[24] = v2652;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(4),
            &[1, 3, 4],
            &[cached[10], cached[11], cached[12]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(5),
            &[1, 3, 5],
            &[cached[14], cached[15], cached[16]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            None,
            &[3],
            &[cached[18]],
            &[],
            &[],
            multiplicity,
        );
    }

}

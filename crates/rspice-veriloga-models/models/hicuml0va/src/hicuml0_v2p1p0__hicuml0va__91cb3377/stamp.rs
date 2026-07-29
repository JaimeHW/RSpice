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
        self.canonical_staged[0] = values[0];
        self.canonical_staged[2] = values[1];
        self.canonical_staged[8] = values[2];
        self.canonical_staged[16] = values[3];
        self.canonical_staged[3] = values[4];
        self.canonical_staged[9] = values[5];
        self.canonical_staged[17] = values[6];
        self.canonical_staged[11] = values[7];
        self.canonical_staged[12] = values[8];
        self.canonical_staged[23] = values[9];
        self.canonical_staged[1] = values[10];
        self.canonical_staged[4] = values[11];
        self.canonical_staged[5] = values[12];
        self.canonical_staged[6] = values[13];
        self.canonical_staged[7] = values[14];
        self.canonical_staged[10] = values[15];
        self.canonical_staged[26] = values[16];
        self.canonical_staged[27] = values[17];
        self.canonical_staged[13] = values[18];
        self.canonical_staged[28] = values[19];
        self.canonical_staged[14] = values[20];
        self.canonical_staged[15] = values[21];
        self.canonical_staged[29] = values[22];
        self.canonical_staged[30] = values[23];
        self.canonical_staged[70] = values[24];
        self.canonical_staged[71] = values[25];
        self.canonical_staged[19] = values[26];
        self.canonical_staged[72] = values[27];
        self.canonical_staged[73] = values[28];
        self.canonical_staged[74] = values[29];
        self.canonical_staged[75] = values[30];
        self.canonical_staged[20] = values[31];
        self.canonical_staged[21] = values[32];
        self.canonical_staged[76] = values[33];
        self.canonical_staged[77] = values[34];
        self.canonical_staged[78] = values[35];
        self.canonical_staged[79] = values[36];
        self.canonical_staged[80] = values[37];
        self.canonical_staged[81] = values[38];
        self.canonical_staged[82] = values[39];
        self.canonical_staged[83] = values[40];
        self.canonical_staged[84] = values[41];
        self.canonical_staged[85] = values[42];
        self.canonical_staged[86] = values[43];
        self.canonical_staged[87] = values[44];
        self.canonical_staged[88] = values[45];
        self.canonical_staged[22] = values[46];
        self.canonical_staged[93] = values[47];
        self.canonical_staged[89] = values[48];
        self.canonical_staged[90] = values[49];
        self.canonical_staged[91] = values[50];
        self.canonical_staged[92] = values[51];
        self.canonical_staged[94] = values[52];
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
                let v0 = parameters[108];
                let v1 = 2.7315e2f64;
                let v3 = 1.3806226e-23f64;
                let v5 = 1.602176462e-19f64;
                let v7 = parameters[88];
                let v9 = parameters[76];
                let v10 = parameters[77];
                let v12 = 5e-1f64;
                let v14 = parameters[78];
                let v17 = parameters[79];
                let v20 = parameters[80];
                let v23 = 3e0f64;
                let v25 = 1e0f64;
                let v27 = parameters[87];
                let v29 = 1.5e0f64;
                let v31 = parameters[82];
                let v32 = parameters[81];
                let v36 = parameters[21];
                let v37 = 0e0f64;
                let v39 = parameters[41];
                let v43 = parameters[35];
                let v46 = 2e0f64;
                let v54 = parameters[38];
                let v65 = parameters[42];
                let v75 = parameters[65];
                let v77 = parameters[96];
                let v81 = parameters[46];
                let v90 = parameters[51];
                let v99 = parameters[0];
                let v100 = 2e2f64;
                let v102 = parameters[103];
                let v104 = parameters[104];
                let v105 = parameters[111];
                let v108 = parameters[44];
                let v109 = 1e2f64;
                let v111 = parameters[36];
                let v113 = parameters[7];
                let v115 = parameters[39];
                let v116 = parameters[10];
                let v118 = parameters[13];
                let v120 = parameters[2];
                let v122 = parameters[9];
                let v123 = 1e6f64;
                let v125 = parameters[12];
                let v128 = parameters[60];
                let v132 = parameters[58];
                let v134 = parameters[15];
                let v136 = parameters[17];
                let v138 = parameters[19];
                let v140 = parameters[30];
                let v142 = parameters[32];
                let v144 = parameters[53];
                let v148 = parameters[73];
                let v150 = parameters[54];
                let v153 = parameters[28];
                let v155 = 0e0f64;
                let v157 = parameters[29];
                let v159 = 0e0f64;
                let v161 = parameters[23];
                let v163 = parameters[26];
                let v166 = 0e0f64;
                let v169 = parameters[107];
                let v174 = 0e0f64;
                let v176 = 0e0f64;
                let v178 = 0e0f64;
                let v180 = 0e0f64;
                let mut out127: f64 = 0.0;
                let v2 = v0 + v1;
                let v6 = (v3 * v2) / v5;
                let v8 = v7 * v2;
                let v13 = v12 * (v9 + v10);
                let v16 = v12 * (v9 + v14);
                let v19 = v12 * (v17 + v14);
                let v24 = v23 - ((v5 * v20) / v3);
                let v28 = (v24 + v25) - v27;
                let v30 = v24 - v29;
                let v34 = (v31 - v32) - v12;
                let v35 = v9 - v10;
                let v41 = if (if v36 > v37 { 1.0 } else { 0.0 }) != 0.0 && (if v39 > v37 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v42: f64;
                if v41 != 0.0 {
                    v42 = v25;
                } else {
                    v42 = v37;
                }
                let v45 = (v12 * v43) / v6;
                let v47 = v46 * v6;
                let v53 = v47 * (((v45.exp()) - ((-v45).exp())).ln());
                let v56 = (v12 * v54) / v6;
                let v62 = v47 * (((v56.exp()) - ((-v56).exp())).ln());
                let v63 = v12 * v24;
                let v64 = v12 * v13;
                let v67 = (v12 * v65) / v6;
                let v73 = v47 * (((v67.exp()) - ((-v67).exp())).ln());
                let v74 = v27 - v8;
                let v76 = if v75 > v37 { 1.0 } else { 0.0 };
                let v78 = if v77 == v25 { 1.0 } else { 0.0 };
                let v79 = v27 - v25;
                let v80 = if v42 == v25 { 1.0 } else { 0.0 };
                let v83 = (v12 * v81) / v6;
                let v89 = v47 * (((v83.exp()) - ((-v83).exp())).ln());
                let v92 = (v12 * v90) / v6;
                let v98 = v47 * (((v92.exp()) - ((-v92).exp())).ln());
                let v101 = if v99 <= v100 { 1.0 } else { 0.0 };
                let v106 = if v104 >= v105 { 1.0 } else { 0.0 };
                let v107 = if (if v102 != v37 { 1.0 } else { 0.0 }) != 0.0 && v106 != 0.0 { 1.0 } else { 0.0 };
                let v110 = if v108 < v109 { 1.0 } else { 0.0 };
                let v112: f64;
                if v101 != 0.0 {
                    v112 = v115;
                } else {
                    v112 = v111;
                }
                let v114 = if v113 == v37 { 1.0 } else { 0.0 };
                let v117 = if v116 == v25 { 1.0 } else { 0.0 };
                let v119 = if v118 != v37 { 1.0 } else { 0.0 };
                let v121 = if v120 == v37 { 1.0 } else { 0.0 };
                if v121 != 0.0 {
                } else {
                    let v127 = if (if v122 == v123 { 1.0 } else { 0.0 }) != 0.0 && (if v125 == v123 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    out127 = v127;
                }
                let v131 = v25 + ((v25 + v128).sqrt());
                let v133 = v132 + v25;
                let v135 = if v134 > v37 { 1.0 } else { 0.0 };
                let v137 = if v136 > v37 { 1.0 } else { 0.0 };
                let v139 = if v138 > v37 { 1.0 } else { 0.0 };
                let v141 = if v140 > v37 { 1.0 } else { 0.0 };
                let v143 = if v142 > v37 { 1.0 } else { 0.0 };
                let v145 = if v144 < v109 { 1.0 } else { 0.0 };
                let v147 = if (if v102 == v25 { 1.0 } else { 0.0 }) != 0.0 && v106 != 0.0 { 1.0 } else { 0.0 };
                let v152 = if (if v148 != v37 { 1.0 } else { 0.0 }) != 0.0 && (if v150 != v37 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v154 = if v153 >= v105 { 1.0 } else { 0.0 };
                let v156: f64;
                if v154 != 0.0 {
                    v156 = v37;
                } else {
                    v156 = v155;
                }
                let v158 = if v157 >= v105 { 1.0 } else { 0.0 };
                let v160: f64;
                if v158 != 0.0 {
                    v160 = v37;
                } else {
                    v160 = v159;
                }
                let v165 = if (if v161 >= v105 { 1.0 } else { 0.0 }) != 0.0 || (if v163 >= v105 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v167: f64;
                if v165 != 0.0 {
                    v167 = v37;
                } else {
                    v167 = v166;
                }
                let v168 = if v102 == v37 { 1.0 } else { 0.0 };
                let v171 = if v168 != 0.0 || (if v169 == v37 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v173 = if v168 != 0.0 || (if v104 < v105 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v175: f64;
                if v173 != 0.0 {
                    v175 = v174;
                } else {
                    v175 = v37;
                }
                let v177: f64;
                if v165 != 0.0 {
                    v177 = v176;
                } else {
                    v177 = v37;
                }
                let v179: f64;
                if v158 != 0.0 {
                    v179 = v178;
                } else {
                    v179 = v37;
                }
                let v181: f64;
                if v154 != 0.0 {
                    v181 = v180;
                } else {
                    v181 = v37;
                }
            [v2, v13, v16, v19, v24, v28, v30, v34, v35, v41, v53, v62, v63, v64, v73, v74, v76, v78, v79, v80, v89, v98, v101, v107, v110, v114, v112, v117, v119, v121, out127, v131, v133, v135, v137, v139, v141, v143, v145, v147, v152, v154, v158, v165, v171, v173, v177, v179, v156, v160, v167, v175, v181]
        };
        let values = canonical_model_cache_intern(key, Arc::new(produced));
        self.canonical_install_model_values(values);
    }

    fn canonical_instance_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        if self.canonical_instance_valid {
            return;
        }
        let produced: [f64; 1] = {
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
            [0.0]
        };
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
        let produced: [f64; 42] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let temperature = ctx.temperature();
            let staged = &*self.canonical_staged;
                let v0 = temperature;
                let v1 = parameters[109];
                let v3 = 1.7314999999999998e2f64;
                let v5 = 1.7314999999999998e2f64;
                let v6 = 6e2f64;
                let v9 = 1.3806226e-23f64;
                let v11 = 1.602176462e-19f64;
                let v13 = 1e0f64;
                let v15 = staged[0];
                let v21 = staged[1];
                let v24 = staged[2];
                let v27 = staged[3];
                let v31 = 2e0f64;
                let v36 = 4e0f64;
                let v41 = 5e-1f64;
                let v46 = parameters[35];
                let v49 = parameters[36];
                let v52 = parameters[34];
                let v54 = parameters[37];
                let v57 = staged[4];
                let v72 = parameters[38];
                let v75 = parameters[39];
                let v79 = parameters[40];
                let v82 = parameters[82];
                let v84 = parameters[77];
                let v88 = parameters[15];
                let v90 = staged[5];
                let v92 = staged[6];
                let v96 = parameters[17];
                let v98 = staged[7];
                let v100 = staged[8];
                let v115 = parameters[42];
                let v118 = parameters[43];
                let v121 = parameters[41];
                let v123 = staged[9];
                let v125 = parameters[78];
                let v129 = parameters[19];
                let v131 = parameters[81];
                let v133 = parameters[76];
                let v137 = parameters[1];
                let v139 = parameters[95];
                let v141 = parameters[83];
                let v145 = parameters[9];
                let v147 = staged[10];
                let v150 = parameters[62];
                let v152 = parameters[87];
                let v155 = parameters[61];
                let v158 = parameters[89];
                let v161 = parameters[64];
                let v163 = staged[26];
                let v164 = 6e2f64;
                let v166 = parameters[90];
                let v169 = parameters[65];
                let v173 = parameters[85];
                let v176 = parameters[86];
                let v180 = parameters[54];
                let v182 = staged[27];
                let v183 = staged[11];
                let v185 = staged[12];
                let v189 = parameters[57];
                let v192 = staged[13];
                let v195 = parameters[59];
                let v197 = staged[28];
                let v198 = parameters[99];
                let v201 = parameters[21];
                let v203 = parameters[100];
                let v206 = parameters[22];
                let v210 = parameters[91];
                let v213 = parameters[23];
                let v215 = staged[14];
                let v230 = parameters[46];
                let v233 = parameters[47];
                let v236 = parameters[45];
                let v238 = staged[15];
                let v240 = staged[16];
                let v255 = parameters[51];
                let v258 = parameters[52];
                let v261 = parameters[50];
                let v263 = staged[17];
                let v265 = parameters[79];
                let v269 = parameters[32];
                let v273 = parameters[30];
                let v275 = parameters[97];
                let v278 = parameters[7];
                let v281 = parameters[84];
                let v287 = parameters[6];
                let v289 = staged[29];
                let v290 = parameters[102];
                let v292 = parameters[101];
                let v296 = parameters[98];
                let v300 = parameters[12];
                let v302 = parameters[13];
                let v307 = parameters[93];
                let v310 = parameters[29];
                let v312 = parameters[92];
                let v315 = parameters[26];
                let v317 = parameters[94];
                let v320 = parameters[28];
                let v322 = parameters[105];
                let v325 = parameters[104];
                let v327 = parameters[106];
                let mut out7: f64 = 0.0;
                let v2 = v0 + v1;
                let v4 = if v2 < v3 { 1.0 } else { 0.0 };
                let v8: f64;
                if v4 != 0.0 {
                    v8 = v5;
                } else {
                    let v7 = if v2 > v6 { 1.0 } else { 0.0 };
                    out7 = v7;
                    let v165: f64;
                    if v7 != 0.0 {
                        v165 = v164;
                    } else {
                        v165 = v2;
                    }
                    v8 = v165;
                }
                let v12 = (v9 * v8) / v11;
                let v14 = v13 / v12;
                let v16 = v8 - v15;
                let v17 = v8 / v15;
                let v18 = v17.ln();
                let v20 = v14 * (v17 - v13);
                let v23 = v13 - v17;
                let v25 = v24 * v23;
                let v29 = (v27 * v12) * v18;
                let v30 = ((v21 * v17) + v25) - v29;
                let v32 = v31 * v12;
                let v45 = v30 + (v32 * ((v41 * (v13 + ((v13 + (v36 * (((-v30) * v14).exp()))).sqrt()))).ln()));
                let v53 = v52 * ((v49 * ((v46 / v45).ln())).exp());
                let v56 = (v54 * v45) / v46;
                let v60 = ((v57 * v17) + v25) - v29;
                let v71 = v60 + (v32 * ((v41 * (v13 + ((v13 + (v36 * (((-v60) * v14).exp()))).sqrt()))).ln()));
                let v78 = v52 * ((v75 * ((v72 / v71).ln())).exp());
                let v81 = (v79 * v71) / v72;
                let v89 = v88 * (((v82 * v18) + (v84 * v20)).exp());
                let v97 = v96 * (((v90 * v18) + (v92 * v20)).exp());
                let v101 = v100 * v23;
                let v103 = ((v98 * v17) + v101) - v29;
                let v114 = v103 + (v32 * ((v41 * (v13 + ((v13 + (v36 * (((-v103) * v14).exp()))).sqrt()))).ln()));
                let v122 = v121 * ((v118 * ((v115 / v114).ln())).exp());
                let v126 = v125 * v20;
                let v130 = v129 * (((v123 * v18) + v126).exp());
                let v138 = v137 * (((v131 * v18) + (v133 * v20)).exp());
                let v146 = v145 * (((v139 * v18) - (v141 * v20)).exp());
                let v151 = v150 * ((v147 * v18).exp());
                let v157 = v13 / (v155 * ((v152 * v18).exp()));
                let v162 = v161 * (v13 + (v158 * v16));
                let v171: f64;
                let v172: f64;
                if v163 != 0.0 {
                    let v170 = v169 * (v13 - (v166 * v16));
                    v171 = v170;
                    v172 = v161;
                } else {
                    v171 = v169;
                    v172 = v162;
                }
                let v181 = v180 * ((v13 + (v173 * v16)) + ((v176 * v16) * v16));
                let v191: f64;
                if v182 != 0.0 {
                    let v190 = v189 * (((v183 * v18) - (v185 * v20)).exp());
                    v191 = v190;
                } else {
                    v191 = v189;
                }
                let v196 = v195 * ((v192 * v18).exp());
                let v208: f64;
                let v209: f64;
                if v197 != 0.0 {
                    let v202 = v201 * ((v198 * v16).exp());
                    let v207 = v206 * ((v203 * v16).exp());
                    v208 = v207;
                    v209 = v202;
                } else {
                    v208 = v206;
                    v209 = v201;
                }
                let v214 = v213 * ((v210 * v18).exp());
                let v218 = ((v215 * v17) + v101) - v29;
                let v229 = v218 + (v32 * ((v41 * (v13 + ((v13 + (v36 * (((-v218) * v14).exp()))).sqrt()))).ln()));
                let v237 = v236 * ((v233 * ((v230 / v229).ln())).exp());
                let v243 = ((v238 * v17) + (v240 * v23)) - v29;
                let v254 = v243 + (v32 * ((v41 * (v13 + ((v13 + (v36 * (((-v243) * v14).exp()))).sqrt()))).ln()));
                let v262 = v261 * ((v258 * ((v255 / v254).ln())).exp());
                let v264 = v263 * v18;
                let v270 = v269 * ((v264 + (v265 * v20)).exp());
                let v274 = v273 * ((v264 + v126).exp());
                let v279 = v278 * ((v275 * v18).exp());
                let v288 = v287 / (((v141 * v14) * (((v281 * v18).exp()) - v13)).exp());
                let v299: f64;
                if v289 != 0.0 {
                    let v295 = v13 + (v16 * (v292 + (v290 * v16)));
                    v299 = v295;
                } else {
                    let v298 = (v296 * v18).exp();
                    v299 = v298;
                }
                let v301 = v300 * v299;
                let v306 = (v302 * v299) * ((v185 * v20).exp());
                let v311 = v310 * ((v307 * v18).exp());
                let v316 = v315 * ((v312 * v18).exp());
                let v321 = v320 * ((v317 * v18).exp());
                let v330 = (v325 * ((v322 * v18).exp())) * (v13 + (v327 * v16));
            [v2, v4, out7, v12, v14, v45, v53, v56, v71, v78, v81, v89, v97, v114, v122, v130, v138, v146, v151, v157, v181, v196, v214, v229, v237, v254, v262, v270, v274, v279, v288, v301, v306, v311, v316, v321, v330, v171, v172, v191, v208, v209]
        };
        self.canonical_staged[18] = produced[0];
        self.canonical_staged[24] = produced[1];
        self.canonical_staged[25] = produced[2];
        self.canonical_staged[35] = produced[3];
        self.canonical_staged[34] = produced[4];
        self.canonical_staged[42] = produced[5];
        self.canonical_staged[41] = produced[6];
        self.canonical_staged[43] = produced[7];
        self.canonical_staged[45] = produced[8];
        self.canonical_staged[44] = produced[9];
        self.canonical_staged[46] = produced[10];
        self.canonical_staged[56] = produced[11];
        self.canonical_staged[57] = produced[12];
        self.canonical_staged[33] = produced[13];
        self.canonical_staged[32] = produced[14];
        self.canonical_staged[58] = produced[15];
        self.canonical_staged[51] = produced[16];
        self.canonical_staged[50] = produced[17];
        self.canonical_staged[39] = produced[18];
        self.canonical_staged[40] = produced[19];
        self.canonical_staged[49] = produced[20];
        self.canonical_staged[54] = produced[21];
        self.canonical_staged[61] = produced[22];
        self.canonical_staged[36] = produced[23];
        self.canonical_staged[31] = produced[24];
        self.canonical_staged[66] = produced[25];
        self.canonical_staged[65] = produced[26];
        self.canonical_staged[64] = produced[27];
        self.canonical_staged[63] = produced[28];
        self.canonical_staged[47] = produced[29];
        self.canonical_staged[48] = produced[30];
        self.canonical_staged[53] = produced[31];
        self.canonical_staged[52] = produced[32];
        self.canonical_staged[68] = produced[33];
        self.canonical_staged[62] = produced[34];
        self.canonical_staged[67] = produced[35];
        self.canonical_staged[69] = produced[36];
        self.canonical_staged[37] = produced[37];
        self.canonical_staged[38] = produced[38];
        self.canonical_staged[55] = produced[39];
        self.canonical_staged[59] = produced[40];
        self.canonical_staged[60] = produced[41];
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
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9])];
        let branch_unknown_flows = [ctx.branch_current(self.branches[0]), ctx.branch_current(self.branches[1]), ctx.branch_current(self.branches[2]), ctx.branch_current(self.branches[3])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 12300 => 0usize, 12302 => 1usize, 12304 => 2usize, 12306 => 3usize, 12334 => 4usize, 12337 => 5usize, 12350 => 6usize, 12367 => 7usize, 12370 => 8usize, _ => usize::MAX };
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
            let v0 = node_potentials[1];
            let v1 = node_potentials[5];
            let v3 = 1e0f64;
            let v5 = 1e0f64;
            let v8 = parameters[110];
            let v11 = node_potentials[6];
            let v13 = 1e0f64;
            let v19 = node_potentials[7];
            let v22 = 1e0f64;
            let v31 = node_potentials[3];
            let v33 = 1e0f64;
            let v39 = node_potentials[2];
            let v42 = 1e0f64;
            let v51 = node_potentials[0];
            let v54 = 1e0f64;
            let v61 = staged[26];
            let v62 = staged[27];
            let v63 = staged[28];
            let v64 = staged[29];
            let v65 = staged[30];
            let v66 = staged[18];
            let v67 = node_potentials[4];
            let v69 = 1.7314999999999998e2f64;
            let v71 = staged[31];
            let v72 = staged[32];
            let v73 = staged[33];
            let v74 = staged[34];
            let v75 = staged[35];
            let v76 = staged[36];
            let v77 = staged[37];
            let v78 = staged[38];
            let v79 = staged[39];
            let v80 = staged[40];
            let v81 = staged[41];
            let v82 = staged[42];
            let v83 = staged[43];
            let v84 = staged[44];
            let v85 = staged[45];
            let v86 = staged[46];
            let v87 = staged[47];
            let v88 = staged[48];
            let v89 = staged[49];
            let v90 = staged[50];
            let v91 = staged[51];
            let v92 = staged[52];
            let v93 = staged[53];
            let v94 = staged[54];
            let v95 = staged[55];
            let v96 = staged[56];
            let v97 = staged[57];
            let v98 = staged[58];
            let v99 = staged[59];
            let v100 = staged[60];
            let v101 = staged[61];
            let v102 = staged[62];
            let v103 = staged[63];
            let v104 = staged[64];
            let v105 = staged[65];
            let v106 = staged[66];
            let v107 = staged[67];
            let v108 = staged[68];
            let v109 = staged[69];
            let v110 = 0e0f64;
            let v189 = 1e-30f64;
            let v191 = 1.7314999999999998e2f64;
            let v192 = 6e2f64;
            let v196 = 1.3806226e-23f64;
            let v199 = 1.602176462e-19f64;
            let v202 = 1e0f64;
            let v205 = -1e0f64;
            let v208 = staged[0];
            let v213 = 1e0f64;
            let v221 = staged[1];
            let v226 = staged[2];
            let v231 = staged[3];
            let v240 = 2e0f64;
            let v251 = 4e0f64;
            let v256 = 2e0f64;
            let v261 = 5e-1f64;
            let v273 = parameters[35];
            let v281 = parameters[36];
            let v286 = parameters[34];
            let v289 = parameters[37];
            let v294 = staged[4];
            let v328 = parameters[38];
            let v336 = parameters[39];
            let v343 = parameters[40];
            let v348 = parameters[82];
            let v351 = parameters[77];
            let v358 = parameters[15];
            let v361 = staged[5];
            let v364 = staged[6];
            let v371 = parameters[17];
            let v374 = staged[7];
            let v377 = staged[8];
            let v411 = parameters[42];
            let v419 = parameters[43];
            let v424 = parameters[41];
            let v427 = staged[9];
            let v430 = parameters[78];
            let v437 = parameters[19];
            let v440 = parameters[81];
            let v443 = parameters[76];
            let v450 = parameters[1];
            let v453 = parameters[95];
            let v456 = parameters[83];
            let v463 = parameters[9];
            let v466 = staged[10];
            let v471 = parameters[62];
            let v474 = parameters[87];
            let v479 = parameters[61];
            let v486 = parameters[89];
            let v490 = parameters[64];
            let v493 = 6e2f64;
            let v494 = 1e0f64;
            let v497 = parameters[90];
            let v502 = parameters[65];
            let v509 = parameters[85];
            let v513 = parameters[86];
            let v522 = parameters[54];
            let v525 = staged[11];
            let v528 = staged[12];
            let v535 = parameters[57];
            let v540 = staged[13];
            let v545 = parameters[59];
            let v548 = parameters[99];
            let v553 = parameters[21];
            let v556 = parameters[100];
            let v561 = parameters[22];
            let v568 = parameters[91];
            let v573 = parameters[23];
            let v576 = staged[14];
            let v610 = parameters[46];
            let v618 = parameters[47];
            let v623 = parameters[45];
            let v626 = staged[15];
            let v629 = staged[16];
            let v663 = parameters[51];
            let v671 = parameters[52];
            let v676 = parameters[50];
            let v679 = staged[17];
            let v682 = parameters[79];
            let v689 = parameters[32];
            let v696 = parameters[30];
            let v699 = parameters[97];
            let v704 = parameters[7];
            let v709 = parameters[84];
            let v721 = parameters[6];
            let v726 = parameters[102];
            let v729 = parameters[101];
            let v736 = parameters[98];
            let v743 = parameters[12];
            let v746 = parameters[13];
            let v757 = parameters[93];
            let v762 = parameters[29];
            let v765 = parameters[92];
            let v770 = parameters[26];
            let v773 = parameters[94];
            let v778 = parameters[28];
            let v781 = parameters[105];
            let v786 = parameters[104];
            let v789 = parameters[106];
            let v797 = parameters[49];
            let v803 = parameters[44];
            let v804 = 1e2f64;
            let v808 = parameters[48];
            let v816 = staged[70];
            let v817 = 0e0f64;
            let v822 = Lanes([0e0f64; 3]);
            let v826 = -8.754687373538999e-1f64;
            let v832 = 2.4e0f64;
            let v860 = 8e1f64;
            let v862 = Lanes([0e0f64; 3]);
            let v880 = 1e-1f64;
            let v1006 = -8.754687373538999e-1f64;
            let v1024 = 1.921812e0f64;
            let v1088 = -8.754687373538999e-1f64;
            let v1264 = -8.754687373538999e-1f64;
            let v1342 = -8.754687373538999e-1f64;
            let v1518 = -8.754687373538999e-1f64;
            let v1599 = -8.754687373538999e-1f64;
            let v1775 = -8.754687373538999e-1f64;
            let v1846 = -8.754687373538999e-1f64;
            let v1969 = parameters[67];
            let v1989 = parameters[63];
            let v1995 = parameters[66];
            let v2108 = Lanes([0e0f64; 3]);
            let v2121 = staged[71];
            let v2210 = parameters[8];
            let v2251 = staged[19];
            let v2264 = 1e-3f64;
            let v2278 = parameters[5];
            let v2285 = 2e1f64;
            let v2302 = 2.5e-2f64;
            let v2306 = parameters[55];
            let v2317 = parameters[56];
            let v2322 = staged[72];
            let v2350 = parameters[3];
            let v2377 = parameters[4];
            let v2404 = staged[73];
            let v2411 = parameters[11];
            let v2438 = 6.666e-1f64;
            let v2502 = 1e-8f64;
            let v2504 = parameters[14];
            let v2533 = 1e-2f64;
            let v2541 = 2.004987562112089e0f64;
            let v2544 = Lanes([0e0f64; 4]);
            let v2547 = staged[74];
            let v2548 = -2e0f64;
            let v2551 = staged[75];
            let v2554 = 1e-20f64;
            let v2702 = 3.333333333333333e-1f64;
            let v2713 = 2.7e1f64;
            let v2729 = 2.5e-1f64;
            let v2744 = 1e-10f64;
            let v2746 = 3e0f64;
            let v2775 = -2.7e1f64;
            let v2858 = 1.5707963267948966e0f64;
            let v2876 = 1.5707963267948966e0f64;
            let v2880 = -4e0f64;
            let v2934 = parameters[60];
            let v2942 = staged[20];
            let v2965 = parameters[58];
            let v2979 = staged[21];
            let v2986 = parameters[68];
            let v2989 = staged[76];
            let v2990 = parameters[16];
            let v3001 = staged[77];
            let v3020 = parameters[18];
            let v3033 = staged[78];
            let v3052 = parameters[20];
            let v3092 = -8.754687373538999e-1f64;
            let v3265 = -8.754687373538999e-1f64;
            let v3410 = parameters[24];
            let v3414 = parameters[25];
            let v3455 = staged[79];
            let v3456 = parameters[27];
            let v3468 = 1e-6f64;
            let v3494 = parameters[31];
            let v3522 = Lanes([0e0f64; 4]);
            let v3525 = staged[80];
            let v3526 = parameters[33];
            let v3536 = Lanes([0e0f64; 3]);
            let v3539 = staged[81];
            let v3562 = staged[82];
            let v3564 = parameters[53];
            let v3567 = -8.754687373538999e-1f64;
            let v3743 = -8.754687373538999e-1f64;
            let v3831 = staged[83];
            let v3832 = node_potentials[8];
            let v3834 = 1e0f64;
            let v3838 = parameters[71];
            let v3843 = node_potentials[9];
            let v3845 = 1e0f64;
            let v3849 = parameters[72];
            let v3858 = 0e0f64;
            let v3859 = 0e0f64;
            let v3874 = parameters[70];
            let v3877 = parameters[69];
            let v3920 = ddt_scale();
            let v3928 = staged[84];
            let v3935 = Lanes([0e0f64; 3]);
            let v3938 = staged[85];
            let v3945 = Lanes([0e0f64; 3]);
            let v3948 = staged[86];
            let v3955 = Lanes([0e0f64; 5]);
            let v3962 = staged[87];
            let v3963 = parameters[107];
            let v3972 = staged[88];
            let v3994 = staged[22];
            let v3996 = branch_unknown_flows[2];
            let v4113 = 0e0f64;
            let v4114 = 0e0f64;
            let v4115 = 0e0f64;
            let v9 = v8 * (v0 - v1);
            let v10 = ((Lanes([v3, 0.0])) - (Lanes([0.0, v5]))) * v8;
            let v12 = v11 - v1;
            let v16 = (Lanes([0.0, v13])) - (Lanes([v5, 0.0]));
            let v17 = v8 * v12;
            let v18 = v16 * v8;
            let v20 = v11 - v19;
            let v24 = (Lanes([v13, 0.0])) - (Lanes([0.0, v22]));
            let v25 = v8 * v20;
            let v26 = v24 * v8;
            let v27 = v25 - v17;
            let v30 = (Lanes([0.0, v26[0], v26[1]])) - (Lanes([v18[0], v18[1], 0.0]));
            let v37 = v8 * (v31 - v1);
            let v38 = ((Lanes([v33, 0.0])) - (Lanes([0.0, v5]))) * v8;
            let v45 = v8 * (v0 - v39);
            let v46 = ((Lanes([v3, 0.0])) - (Lanes([0.0, v42]))) * v8;
            let v47 = v19 - v39;
            let v50 = (Lanes([0.0, v22])) - (Lanes([v42, 0.0]));
            let v52 = v1 - v51;
            let v56 = (Lanes([0.0, v5])) - (Lanes([v54, 0.0]));
            let v57 = v0 - v11;
            let v60 = (Lanes([v3, 0.0])) - (Lanes([0.0, v13]));
            let v111: f64;
            let v112: f64;
            let v113: f64;
            let v114: f64;
            let v115: f64;
            let v116: f64;
            let v117: f64;
            let v118: f64;
            let v119: f64;
            let v120: f64;
            let v121: f64;
            let v122: f64;
            let v123: f64;
            let v124: f64;
            let v125: f64;
            let v126: f64;
            let v127: f64;
            let v128: f64;
            let v129: f64;
            let v130: f64;
            let v131: f64;
            let v132: f64;
            let v133: f64;
            let v134: f64;
            let v135: f64;
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
            let v174: f64;
            let v175: f64;
            let v176: f64;
            let v177: f64;
            let v178: f64;
            let v179: f64;
            let v180: f64;
            let v181: f64;
            let v182: f64;
            let v183: f64;
            let v184: f64;
            let v185: f64;
            let v186: f64;
            let v187: f64;
            let v188: f64;
            if v65 != 0.0 {
                let v68 = v66 + v67;
                let v70 = if v68 < v69 { 1.0 } else { 0.0 };
                let v194: f64;
                let v195: f64;
                if v70 != 0.0 {
                    v194 = v191;
                    v195 = v110;
                } else {
                    let v193 = if v68 > v192 { 1.0 } else { 0.0 };
                    let v495: f64;
                    let v496: f64;
                    if v193 != 0.0 {
                        v495 = v493;
                        v496 = v110;
                    } else {
                        v495 = v68;
                        v496 = v494;
                    }
                    v194 = v495;
                    v195 = v496;
                }
                let v200 = (v196 * v194) / v199;
                let v201 = (v195 * v196) / v199;
                let v203 = v202 / v200;
                let v207 = ((v201 * v203) * v205) / v200;
                let v209 = v194 - v208;
                let v210 = v194 / v208;
                let v211 = v195 / v208;
                let v212 = v210.ln();
                let v215 = v211 * (v213 / v210);
                let v216 = v210 - v202;
                let v217 = v203 * v216;
                let v220 = (v207 * v216) + (v211 * v203);
                let v224 = v202 - v210;
                let v225 = v211 * v205;
                let v227 = v226 * v224;
                let v228 = v225 * v226;
                let v232 = v231 * v200;
                let v234 = v232 * v212;
                let v237 = ((v201 * v231) * v212) + (v215 * v232);
                let v238 = ((v221 * v210) + v227) - v234;
                let v239 = ((v211 * v221) + v228) - v237;
                let v241 = v240 * v200;
                let v242 = v201 * v240;
                let v243 = -v238;
                let v249 = (v243 * v203).exp();
                let v255 = (v202 + (v251 * v249)).sqrt();
                let v262 = v261 * (v202 + v255);
                let v264 = v262.ln();
                let v271 = v238 + (v241 * v264);
                let v272 = v239 + ((v242 * v264) + (((((((((v239 * v205) * v203) + (v207 * v243)) * v249) * v251) * (v213 / (v256 * v255))) * v261) * (v213 / v262)) * v241));
                let v274 = v273 / v271;
                let v284 = (v281 * (v274.ln())).exp();
                let v287 = v286 * v284;
                let v288 = ((((((v272 * v274) * v205) / v271) * (v213 / v274)) * v281) * v284) * v286;
                let v292 = (v289 * v271) / v273;
                let v293 = (v272 * v289) / v273;
                let v299 = ((v294 * v210) + v227) - v234;
                let v300 = ((v211 * v294) + v228) - v237;
                let v301 = -v299;
                let v307 = (v301 * v203).exp();
                let v312 = (v202 + (v251 * v307)).sqrt();
                let v317 = v261 * (v202 + v312);
                let v319 = v317.ln();
                let v326 = v299 + (v241 * v319);
                let v327 = v300 + ((v242 * v319) + (((((((((v300 * v205) * v203) + (v207 * v301)) * v307) * v251) * (v213 / (v256 * v312))) * v261) * (v213 / v317)) * v241));
                let v329 = v328 / v326;
                let v339 = (v336 * (v329.ln())).exp();
                let v341 = v286 * v339;
                let v342 = ((((((v327 * v329) * v205) / v326) * (v213 / v329)) * v336) * v339) * v286;
                let v346 = (v343 * v326) / v328;
                let v347 = (v327 * v343) / v328;
                let v356 = ((v348 * v212) + (v351 * v217)).exp();
                let v359 = v358 * v356;
                let v360 = (((v215 * v348) + (v220 * v351)) * v356) * v358;
                let v369 = ((v361 * v212) + (v364 * v217)).exp();
                let v372 = v371 * v369;
                let v373 = (((v215 * v361) + (v220 * v364)) * v369) * v371;
                let v378 = v377 * v224;
                let v379 = v225 * v377;
                let v382 = ((v374 * v210) + v378) - v234;
                let v383 = ((v211 * v374) + v379) - v237;
                let v384 = -v382;
                let v390 = (v384 * v203).exp();
                let v395 = (v202 + (v251 * v390)).sqrt();
                let v400 = v261 * (v202 + v395);
                let v402 = v400.ln();
                let v409 = v382 + (v241 * v402);
                let v410 = v383 + ((v242 * v402) + (((((((((v383 * v205) * v203) + (v207 * v384)) * v390) * v251) * (v213 / (v256 * v395))) * v261) * (v213 / v400)) * v241));
                let v412 = v411 / v409;
                let v422 = (v419 * (v412.ln())).exp();
                let v425 = v424 * v422;
                let v426 = ((((((v410 * v412) * v205) / v409) * (v213 / v412)) * v419) * v422) * v424;
                let v431 = v430 * v217;
                let v432 = v220 * v430;
                let v435 = ((v427 * v212) + v431).exp();
                let v438 = v437 * v435;
                let v439 = (((v215 * v427) + v432) * v435) * v437;
                let v448 = ((v440 * v212) + (v443 * v217)).exp();
                let v451 = v450 * v448;
                let v452 = (((v215 * v440) + (v220 * v443)) * v448) * v450;
                let v461 = ((v453 * v212) - (v456 * v217)).exp();
                let v464 = v463 * v461;
                let v465 = (((v215 * v453) - (v220 * v456)) * v461) * v463;
                let v469 = (v466 * v212).exp();
                let v472 = v471 * v469;
                let v473 = ((v215 * v466) * v469) * v471;
                let v477 = (v474 * v212).exp();
                let v480 = v479 * v477;
                let v482 = v202 / v480;
                let v485 = (((((v215 * v474) * v477) * v479) * v482) * v205) / v480;
                let v491 = v490 * (v202 + (v486 * v209));
                let v492 = (v195 * v486) * v490;
                let v505: f64;
                let v506: f64;
                let v507: f64;
                let v508: f64;
                if v61 != 0.0 {
                    let v503 = v502 * (v202 - (v497 * v209));
                    let v504 = ((v195 * v497) * v205) * v502;
                    v505 = v503;
                    v506 = v490;
                    v507 = v504;
                    v508 = v110;
                } else {
                    v505 = v502;
                    v506 = v491;
                    v507 = v110;
                    v508 = v492;
                }
                let v514 = v513 * v209;
                let v523 = v522 * ((v202 + (v509 * v209)) + (v514 * v209));
                let v524 = ((v195 * v509) + (((v195 * v513) * v209) + (v195 * v514))) * v522;
                let v538: f64;
                let v539: f64;
                if v62 != 0.0 {
                    let v533 = ((v525 * v212) - (v528 * v217)).exp();
                    let v536 = v535 * v533;
                    let v537 = (((v215 * v525) - (v220 * v528)) * v533) * v535;
                    v538 = v536;
                    v539 = v537;
                } else {
                    v538 = v535;
                    v539 = v110;
                }
                let v543 = (v540 * v212).exp();
                let v546 = v545 * v543;
                let v547 = ((v215 * v540) * v543) * v545;
                let v564: f64;
                let v565: f64;
                let v566: f64;
                let v567: f64;
                if v63 != 0.0 {
                    let v551 = (v548 * v209).exp();
                    let v554 = v553 * v551;
                    let v555 = ((v195 * v548) * v551) * v553;
                    let v559 = (v556 * v209).exp();
                    let v562 = v561 * v559;
                    let v563 = ((v195 * v556) * v559) * v561;
                    v564 = v562;
                    v565 = v554;
                    v566 = v563;
                    v567 = v555;
                } else {
                    v564 = v561;
                    v565 = v553;
                    v566 = v110;
                    v567 = v110;
                }
                let v571 = (v568 * v212).exp();
                let v574 = v573 * v571;
                let v575 = ((v215 * v568) * v571) * v573;
                let v581 = ((v576 * v210) + v378) - v234;
                let v582 = ((v211 * v576) + v379) - v237;
                let v583 = -v581;
                let v589 = (v583 * v203).exp();
                let v594 = (v202 + (v251 * v589)).sqrt();
                let v599 = v261 * (v202 + v594);
                let v601 = v599.ln();
                let v608 = v581 + (v241 * v601);
                let v609 = v582 + ((v242 * v601) + (((((((((v582 * v205) * v203) + (v207 * v583)) * v589) * v251) * (v213 / (v256 * v594))) * v261) * (v213 / v599)) * v241));
                let v611 = v610 / v608;
                let v621 = (v618 * (v611.ln())).exp();
                let v624 = v623 * v621;
                let v625 = ((((((v609 * v611) * v205) / v608) * (v213 / v611)) * v618) * v621) * v623;
                let v634 = ((v626 * v210) + (v629 * v224)) - v234;
                let v635 = ((v211 * v626) + (v225 * v629)) - v237;
                let v636 = -v634;
                let v642 = (v636 * v203).exp();
                let v647 = (v202 + (v251 * v642)).sqrt();
                let v652 = v261 * (v202 + v647);
                let v654 = v652.ln();
                let v661 = v634 + (v241 * v654);
                let v662 = v635 + ((v242 * v654) + (((((((((v635 * v205) * v203) + (v207 * v636)) * v642) * v251) * (v213 / (v256 * v647))) * v261) * (v213 / v652)) * v241));
                let v664 = v663 / v661;
                let v674 = (v671 * (v664.ln())).exp();
                let v677 = v676 * v674;
                let v678 = ((((((v662 * v664) * v205) / v661) * (v213 / v664)) * v671) * v674) * v676;
                let v680 = v679 * v212;
                let v681 = v215 * v679;
                let v687 = (v680 + (v682 * v217)).exp();
                let v690 = v689 * v687;
                let v691 = ((v681 + (v220 * v682)) * v687) * v689;
                let v694 = (v680 + v431).exp();
                let v697 = v696 * v694;
                let v698 = ((v681 + v432) * v694) * v696;
                let v702 = (v699 * v212).exp();
                let v705 = v704 * v702;
                let v706 = ((v215 * v699) * v702) * v704;
                let v707 = v456 * v203;
                let v712 = (v709 * v212).exp();
                let v714 = v712 - v202;
                let v719 = (v707 * v714).exp();
                let v722 = v721 / v719;
                let v725 = ((((((v207 * v456) * v714) + (((v215 * v709) * v712) * v707)) * v719) * v722) * v205) / v719;
                let v741: f64;
                let v742: f64;
                if v64 != 0.0 {
                    let v730 = v729 + (v726 * v209);
                    let v734 = (v195 * v730) + ((v195 * v726) * v209);
                    let v735 = v202 + (v209 * v730);
                    v741 = v735;
                    v742 = v734;
                } else {
                    let v739 = (v736 * v212).exp();
                    let v740 = (v215 * v736) * v739;
                    v741 = v739;
                    v742 = v740;
                }
                let v744 = v743 * v741;
                let v745 = v742 * v743;
                let v747 = v746 * v741;
                let v751 = (v528 * v217).exp();
                let v753 = v747 * v751;
                let v756 = ((v742 * v746) * v751) + (((v220 * v528) * v751) * v747);
                let v760 = (v757 * v212).exp();
                let v763 = v762 * v760;
                let v764 = ((v215 * v757) * v760) * v762;
                let v768 = (v765 * v212).exp();
                let v771 = v770 * v768;
                let v772 = ((v215 * v765) * v768) * v770;
                let v776 = (v773 * v212).exp();
                let v779 = v778 * v776;
                let v780 = ((v215 * v773) * v776) * v778;
                let v784 = (v781 * v212).exp();
                let v787 = v786 * v784;
                let v792 = v202 + (v789 * v209);
                let v793 = v787 * v792;
                let v796 = ((((v215 * v781) * v784) * v786) * v792) + ((v195 * v789) * v787);
                v111 = v624;
                v112 = v425;
                v113 = v409;
                v114 = v203;
                v115 = v200;
                v116 = v608;
                v117 = v505;
                v118 = v506;
                v119 = v472;
                v120 = v482;
                v121 = v287;
                v122 = v271;
                v123 = v292;
                v124 = v341;
                v125 = v326;
                v126 = v346;
                v127 = v705;
                v128 = v722;
                v129 = v523;
                v130 = v464;
                v131 = v451;
                v132 = v753;
                v133 = v744;
                v134 = v546;
                v135 = v538;
                v136 = v359;
                v137 = v372;
                v138 = v438;
                v139 = v564;
                v140 = v565;
                v141 = v574;
                v142 = v771;
                v143 = v697;
                v144 = v690;
                v145 = v677;
                v146 = v661;
                v147 = v779;
                v148 = v763;
                v149 = v793;
                v150 = v625;
                v151 = v426;
                v152 = v410;
                v153 = v207;
                v154 = v201;
                v155 = v609;
                v156 = v507;
                v157 = v508;
                v158 = v473;
                v159 = v485;
                v160 = v288;
                v161 = v272;
                v162 = v293;
                v163 = v342;
                v164 = v327;
                v165 = v347;
                v166 = v706;
                v167 = v725;
                v168 = v524;
                v169 = v465;
                v170 = v452;
                v171 = v756;
                v172 = v745;
                v173 = v547;
                v174 = v539;
                v175 = v360;
                v176 = v373;
                v177 = v439;
                v178 = v566;
                v179 = v567;
                v180 = v575;
                v181 = v772;
                v182 = v698;
                v183 = v691;
                v184 = v678;
                v185 = v662;
                v186 = v780;
                v187 = v764;
                v188 = v796;
            } else {
                v111 = v71;
                v112 = v72;
                v113 = v73;
                v114 = v74;
                v115 = v75;
                v116 = v76;
                v117 = v77;
                v118 = v78;
                v119 = v79;
                v120 = v80;
                v121 = v81;
                v122 = v82;
                v123 = v83;
                v124 = v84;
                v125 = v85;
                v126 = v86;
                v127 = v87;
                v128 = v88;
                v129 = v89;
                v130 = v90;
                v131 = v91;
                v132 = v92;
                v133 = v93;
                v134 = v94;
                v135 = v95;
                v136 = v96;
                v137 = v97;
                v138 = v98;
                v139 = v99;
                v140 = v100;
                v141 = v101;
                v142 = v102;
                v143 = v103;
                v144 = v104;
                v145 = v105;
                v146 = v106;
                v147 = v107;
                v148 = v108;
                v149 = v109;
                v150 = v110;
                v151 = v110;
                v152 = v110;
                v153 = v110;
                v154 = v110;
                v155 = v110;
                v156 = v110;
                v157 = v110;
                v158 = v110;
                v159 = v110;
                v160 = v110;
                v161 = v110;
                v162 = v110;
                v163 = v110;
                v164 = v110;
                v165 = v110;
                v166 = v110;
                v167 = v110;
                v168 = v110;
                v169 = v110;
                v170 = v110;
                v171 = v110;
                v172 = v110;
                v173 = v110;
                v174 = v110;
                v175 = v110;
                v176 = v110;
                v177 = v110;
                v178 = v110;
                v179 = v110;
                v180 = v110;
                v181 = v110;
                v182 = v110;
                v183 = v110;
                v184 = v110;
                v185 = v110;
                v186 = v110;
                v187 = v110;
                v188 = v110;
            }
            let v190 = if v111 <= v189 { 1.0 } else { 0.0 };
            let v810: f64;
            let v811: f64;
            let v812: f64;
            let v813: f64;
            let v814: Lanes<3>;
            let v815: Lanes<3>;
            if v190 != 0.0 {
                let v798 = v112 * v797;
                let v799 = v151 * v797;
                let v800 = v202 - v797;
                let v801 = v112 * v800;
                let v802 = v151 * v800;
                let v805 = if v803 < v804 { 1.0 } else { 0.0 };
                let v820: f64;
                let v821: Lanes<3>;
                if v805 != 0.0 {
                    let v818 = if v801 > v817 { 1.0 } else { 0.0 };
                    let v863: f64;
                    let v864: Lanes<3>;
                    if v818 != 0.0 {
                        let v823 = v419 / v251;
                        let v824 = v803 - v113;
                        let v825 = v152 * v205;
                        let v829 = v202 - ((v826 / v419).exp());
                        let v830 = v113 * v829;
                        let v831 = v152 * v829;
                        let v833 = v832 * v801;
                        let v834 = v802 * v832;
                        let v835 = v823 - v419;
                        let v836 = v803 / v113;
                        let v845 = (v835 * (v836.ln())).exp();
                        let v847 = v801 * v845;
                        let v850 = (v802 * v845) + (((((((v152 * v836) * v205) / v113) * (v213 / v836)) * v835) * v845) * v801);
                        let v851 = v830 - v9;
                        let v852 = Lanes([0.0, v831, 0.0]);
                        let v853 = Lanes([v10[0], 0.0, v10[1]]);
                        let v855 = v851 * v114;
                        let v859 = ((v852 - v853) * v114) + (Lanes([0.0, (v153 * v851), 0.0]));
                        let v861 = if v855 < v860 { 1.0 } else { 0.0 };
                        let v878: f64;
                        let v879: Lanes<3>;
                        if v861 != 0.0 {
                            let v865 = v855.exp();
                            let v867 = v202 + v865;
                            let v868 = v867.ln();
                            let v876 = v830 - (v115 * v868);
                            let v877 = v852 - ((Lanes([0.0, (v154 * v868), 0.0])) + (((v859 * v865) * (v213 / v867)) * v115));
                            v878 = v876;
                            v879 = v877;
                        } else {
                            v878 = v9;
                            v879 = v853;
                        }
                        let v885 = (v880 * v824) + (v251 * v115);
                        let v886 = (v825 * v880) + (v154 * v251);
                        let v890 = (v824 + v878) / v885;
                        let v894 = (((Lanes([0.0, v825, 0.0])) + v879) - (Lanes([0.0, (v886 * v890), 0.0]))) / v885;
                        let v895 = if v890 < v860 { 1.0 } else { 0.0 };
                        let v925: f64;
                        let v926: Lanes<3>;
                        if v895 != 0.0 {
                            let v896 = v890.exp();
                            let v898 = v202 + v896;
                            let v908 = (-(v824 + v830)) / v885;
                            let v912 = v908.exp();
                            let v914 = (v898.ln()) - v912;
                            let v922 = (-v824) + (v885 * v914);
                            let v924 = (Lanes([0.0, (v825 * v205), 0.0])) + ((Lanes([0.0, (v886 * v914), 0.0])) + ((((v894 * v896) * (v213 / v898)) - (Lanes([0.0, (((((v825 + v831) * v205) - (v886 * v908)) / v885) * v912), 0.0]))) * v885));
                            v925 = v922;
                            v926 = v924;
                        } else {
                            v925 = v878;
                            v926 = v879;
                        }
                        let v927 = v9 - v878;
                        let v929 = v878 / v113;
                        let v934 = v202 - v929;
                        let v939 = v925 / v113;
                        let v944 = v202 - v939;
                        let v946 = v944.ln();
                        let v948 = (((v926 - (Lanes([0.0, (v152 * v939), 0.0]))) / v113) * v205) * (v213 / v944);
                        let v949 = v202 - v419;
                        let v950 = v202 - v823;
                        let v953 = (v946 * v949).exp();
                        let v955 = v202 - v953;
                        let v966 = ((v934.ln()) * v950).exp();
                        let v968 = v202 - v966;
                        let v979 = (v946 * v950).exp();
                        let v981 = v202 - v979;
                        let v992 = (((v801 * v955) / v949) + ((v847 * v968) / v950)) - ((v847 * v981) / v950);
                        let v1004 = (v992 * v113) + (v833 * v927);
                        let v1005 = (((((((Lanes([0.0, (v802 * v955), 0.0])) + ((((v948 * v949) * v953) * v205) * v801)) / v949) + (((Lanes([0.0, (v850 * v968), 0.0])) + ((((((((v879 - (Lanes([0.0, (v152 * v929), 0.0]))) / v113) * v205) * (v213 / v934)) * v950) * v966) * v205) * v847)) / v950)) - (((Lanes([0.0, (v850 * v981), 0.0])) + ((((v948 * v950) * v979) * v205) * v847)) / v950)) * v113) + (Lanes([0.0, (v152 * v992), 0.0]))) + ((Lanes([0.0, (v834 * v927), 0.0])) + ((v853 - v879) * v833));
                        v863 = v1004;
                        v864 = v1005;
                    } else {
                        v863 = v817;
                        v864 = v862;
                    }
                    v820 = v863;
                    v821 = v864;
                } else {
                    let v819 = if v801 > v817 { 1.0 } else { 0.0 };
                    let v1076: f64;
                    let v1077: Lanes<3>;
                    if v819 != 0.0 {
                        let v1009 = v202 - ((v1006 / v419).exp());
                        let v1010 = v113 * v1009;
                        let v1012 = v1010 - v9;
                        let v1013 = Lanes([0.0, (v152 * v1009), 0.0]);
                        let v1014 = Lanes([v10[0], 0.0, v10[1]]);
                        let v1016 = v1012 * v114;
                        let v1020 = ((v1013 - v1014) * v114) + (Lanes([0.0, (v153 * v1012), 0.0]));
                        let v1022 = v1020 * v1016;
                        let v1026 = ((v1016 * v1016) + v1024).sqrt();
                        let v1032 = (v1016 + v1026) * v261;
                        let v1039 = v1010 - (v115 * v1032);
                        let v1040 = v1013 - ((Lanes([0.0, (v154 * v1032), 0.0])) + (((v1020 + ((v1022 + v1022) * (v213 / (v256 * v1026)))) * v261) * v115));
                        let v1041 = v1039 / v113;
                        let v1046 = v202 - v1041;
                        let v1051 = v202 - v419;
                        let v1054 = ((v1046.ln()) * v1051).exp();
                        let v1056 = v202 - v1054;
                        let v1069 = ((v113 * v1056) / v1051) + (v832 * (v9 - v1039));
                        let v1071 = v801 * v1069;
                        let v1075 = (Lanes([0.0, (v802 * v1069), 0.0])) + (((((Lanes([0.0, (v152 * v1056), 0.0])) + ((((((((v1040 - (Lanes([0.0, (v152 * v1041), 0.0]))) / v113) * v205) * (v213 / v1046)) * v1051) * v1054) * v205) * v113)) / v1051) + ((v1014 - v1040) * v832)) * v801);
                        v1076 = v1071;
                        v1077 = v1075;
                    } else {
                        v1076 = v817;
                        v1077 = v862;
                    }
                    v820 = v1076;
                    v821 = v1077;
                }
                v810 = v798;
                v811 = v817;
                v812 = v820;
                v813 = v799;
                v814 = v822;
                v815 = v821;
            } else {
                let v806 = v111 * v797;
                let v807 = v150 * v797;
                let v809 = if v808 < v804 { 1.0 } else { 0.0 };
                let v1080: f64;
                let v1081: Lanes<3>;
                if v809 != 0.0 {
                    let v1078 = if v806 > v817 { 1.0 } else { 0.0 };
                    let v1122: f64;
                    let v1123: Lanes<3>;
                    if v1078 != 0.0 {
                        let v1085 = v618 / v251;
                        let v1086 = v808 - v116;
                        let v1087 = v155 * v205;
                        let v1091 = v202 - ((v1088 / v618).exp());
                        let v1092 = v116 * v1091;
                        let v1093 = v155 * v1091;
                        let v1094 = v832 * v806;
                        let v1095 = v807 * v832;
                        let v1096 = v1085 - v618;
                        let v1097 = v808 / v116;
                        let v1106 = (v1096 * (v1097.ln())).exp();
                        let v1108 = v806 * v1106;
                        let v1111 = (v807 * v1106) + (((((((v155 * v1097) * v205) / v116) * (v213 / v1097)) * v1096) * v1106) * v806);
                        let v1112 = v1092 - v17;
                        let v1113 = Lanes([v1093, 0.0, 0.0]);
                        let v1114 = Lanes([0.0, v18[0], v18[1]]);
                        let v1116 = v1112 * v114;
                        let v1120 = ((v1113 - v1114) * v114) + (Lanes([(v153 * v1112), 0.0, 0.0]));
                        let v1121 = if v1116 < v860 { 1.0 } else { 0.0 };
                        let v1137: f64;
                        let v1138: Lanes<3>;
                        if v1121 != 0.0 {
                            let v1124 = v1116.exp();
                            let v1126 = v202 + v1124;
                            let v1127 = v1126.ln();
                            let v1135 = v1092 - (v115 * v1127);
                            let v1136 = v1113 - ((Lanes([(v154 * v1127), 0.0, 0.0])) + (((v1120 * v1124) * (v213 / v1126)) * v115));
                            v1137 = v1135;
                            v1138 = v1136;
                        } else {
                            v1137 = v17;
                            v1138 = v1114;
                        }
                        let v1143 = (v880 * v1086) + (v251 * v115);
                        let v1144 = (v1087 * v880) + (v154 * v251);
                        let v1148 = (v1086 + v1137) / v1143;
                        let v1152 = (((Lanes([v1087, 0.0, 0.0])) + v1138) - (Lanes([(v1144 * v1148), 0.0, 0.0]))) / v1143;
                        let v1153 = if v1148 < v860 { 1.0 } else { 0.0 };
                        let v1183: f64;
                        let v1184: Lanes<3>;
                        if v1153 != 0.0 {
                            let v1154 = v1148.exp();
                            let v1156 = v202 + v1154;
                            let v1166 = (-(v1086 + v1092)) / v1143;
                            let v1170 = v1166.exp();
                            let v1172 = (v1156.ln()) - v1170;
                            let v1180 = (-v1086) + (v1143 * v1172);
                            let v1182 = (Lanes([(v1087 * v205), 0.0, 0.0])) + ((Lanes([(v1144 * v1172), 0.0, 0.0])) + ((((v1152 * v1154) * (v213 / v1156)) - (Lanes([(((((v1087 + v1093) * v205) - (v1144 * v1166)) / v1143) * v1170), 0.0, 0.0]))) * v1143));
                            v1183 = v1180;
                            v1184 = v1182;
                        } else {
                            v1183 = v1137;
                            v1184 = v1138;
                        }
                        let v1185 = v17 - v1137;
                        let v1187 = v1137 / v116;
                        let v1192 = v202 - v1187;
                        let v1197 = v1183 / v116;
                        let v1202 = v202 - v1197;
                        let v1204 = v1202.ln();
                        let v1206 = (((v1184 - (Lanes([(v155 * v1197), 0.0, 0.0]))) / v116) * v205) * (v213 / v1202);
                        let v1207 = v202 - v618;
                        let v1208 = v202 - v1085;
                        let v1211 = (v1204 * v1207).exp();
                        let v1213 = v202 - v1211;
                        let v1224 = ((v1192.ln()) * v1208).exp();
                        let v1226 = v202 - v1224;
                        let v1237 = (v1204 * v1208).exp();
                        let v1239 = v202 - v1237;
                        let v1250 = (((v806 * v1213) / v1207) + ((v1108 * v1226) / v1208)) - ((v1108 * v1239) / v1208);
                        let v1262 = (v1250 * v116) + (v1094 * v1185);
                        let v1263 = (((((((Lanes([(v807 * v1213), 0.0, 0.0])) + ((((v1206 * v1207) * v1211) * v205) * v806)) / v1207) + (((Lanes([(v1111 * v1226), 0.0, 0.0])) + ((((((((v1138 - (Lanes([(v155 * v1187), 0.0, 0.0]))) / v116) * v205) * (v213 / v1192)) * v1208) * v1224) * v205) * v1108)) / v1208)) - (((Lanes([(v1111 * v1239), 0.0, 0.0])) + ((((v1206 * v1208) * v1237) * v205) * v1108)) / v1208)) * v116) + (Lanes([(v155 * v1250), 0.0, 0.0]))) + ((Lanes([(v1095 * v1185), 0.0, 0.0])) + ((v1114 - v1138) * v1094));
                        v1122 = v1262;
                        v1123 = v1263;
                    } else {
                        v1122 = v817;
                        v1123 = v822;
                    }
                    v1080 = v1122;
                    v1081 = v1123;
                } else {
                    let v1079 = if v806 > v817 { 1.0 } else { 0.0 };
                    let v1333: f64;
                    let v1334: Lanes<3>;
                    if v1079 != 0.0 {
                        let v1267 = v202 - ((v1264 / v618).exp());
                        let v1268 = v116 * v1267;
                        let v1270 = v1268 - v17;
                        let v1271 = Lanes([(v155 * v1267), 0.0, 0.0]);
                        let v1272 = Lanes([0.0, v18[0], v18[1]]);
                        let v1274 = v1270 * v114;
                        let v1278 = ((v1271 - v1272) * v114) + (Lanes([(v153 * v1270), 0.0, 0.0]));
                        let v1280 = v1278 * v1274;
                        let v1283 = ((v1274 * v1274) + v1024).sqrt();
                        let v1289 = (v1274 + v1283) * v261;
                        let v1296 = v1268 - (v115 * v1289);
                        let v1297 = v1271 - ((Lanes([(v154 * v1289), 0.0, 0.0])) + (((v1278 + ((v1280 + v1280) * (v213 / (v256 * v1283)))) * v261) * v115));
                        let v1298 = v1296 / v116;
                        let v1303 = v202 - v1298;
                        let v1308 = v202 - v618;
                        let v1311 = ((v1303.ln()) * v1308).exp();
                        let v1313 = v202 - v1311;
                        let v1326 = ((v116 * v1313) / v1308) + (v832 * (v17 - v1296));
                        let v1328 = v806 * v1326;
                        let v1332 = (Lanes([(v807 * v1326), 0.0, 0.0])) + (((((Lanes([(v155 * v1313), 0.0, 0.0])) + ((((((((v1297 - (Lanes([(v155 * v1298), 0.0, 0.0]))) / v116) * v205) * (v213 / v1303)) * v1308) * v1311) * v205) * v116)) / v1308) + ((v1272 - v1297) * v832)) * v806);
                        v1333 = v1328;
                        v1334 = v1332;
                    } else {
                        v1333 = v817;
                        v1334 = v822;
                    }
                    v1080 = v1333;
                    v1081 = v1334;
                }
                let v1082 = v202 - v797;
                let v1083 = v111 * v1082;
                let v1084 = v150 * v1082;
                let v1337: f64;
                let v1338: Lanes<3>;
                if v809 != 0.0 {
                    let v1335 = if v1083 > v817 { 1.0 } else { 0.0 };
                    let v1376: f64;
                    let v1377: Lanes<3>;
                    if v1335 != 0.0 {
                        let v1339 = v618 / v251;
                        let v1340 = v808 - v116;
                        let v1341 = v155 * v205;
                        let v1345 = v202 - ((v1342 / v618).exp());
                        let v1346 = v116 * v1345;
                        let v1347 = v155 * v1345;
                        let v1348 = v832 * v1083;
                        let v1349 = v1084 * v832;
                        let v1350 = v1339 - v618;
                        let v1351 = v808 / v116;
                        let v1360 = (v1350 * (v1351.ln())).exp();
                        let v1362 = v1083 * v1360;
                        let v1365 = (v1084 * v1360) + (((((((v155 * v1351) * v205) / v116) * (v213 / v1351)) * v1350) * v1360) * v1083);
                        let v1366 = v1346 - v9;
                        let v1367 = Lanes([0.0, v1347, 0.0]);
                        let v1368 = Lanes([v10[0], 0.0, v10[1]]);
                        let v1370 = v1366 * v114;
                        let v1374 = ((v1367 - v1368) * v114) + (Lanes([0.0, (v153 * v1366), 0.0]));
                        let v1375 = if v1370 < v860 { 1.0 } else { 0.0 };
                        let v1391: f64;
                        let v1392: Lanes<3>;
                        if v1375 != 0.0 {
                            let v1378 = v1370.exp();
                            let v1380 = v202 + v1378;
                            let v1381 = v1380.ln();
                            let v1389 = v1346 - (v115 * v1381);
                            let v1390 = v1367 - ((Lanes([0.0, (v154 * v1381), 0.0])) + (((v1374 * v1378) * (v213 / v1380)) * v115));
                            v1391 = v1389;
                            v1392 = v1390;
                        } else {
                            v1391 = v9;
                            v1392 = v1368;
                        }
                        let v1397 = (v880 * v1340) + (v251 * v115);
                        let v1398 = (v1341 * v880) + (v154 * v251);
                        let v1402 = (v1340 + v1391) / v1397;
                        let v1406 = (((Lanes([0.0, v1341, 0.0])) + v1392) - (Lanes([0.0, (v1398 * v1402), 0.0]))) / v1397;
                        let v1407 = if v1402 < v860 { 1.0 } else { 0.0 };
                        let v1437: f64;
                        let v1438: Lanes<3>;
                        if v1407 != 0.0 {
                            let v1408 = v1402.exp();
                            let v1410 = v202 + v1408;
                            let v1420 = (-(v1340 + v1346)) / v1397;
                            let v1424 = v1420.exp();
                            let v1426 = (v1410.ln()) - v1424;
                            let v1434 = (-v1340) + (v1397 * v1426);
                            let v1436 = (Lanes([0.0, (v1341 * v205), 0.0])) + ((Lanes([0.0, (v1398 * v1426), 0.0])) + ((((v1406 * v1408) * (v213 / v1410)) - (Lanes([0.0, (((((v1341 + v1347) * v205) - (v1398 * v1420)) / v1397) * v1424), 0.0]))) * v1397));
                            v1437 = v1434;
                            v1438 = v1436;
                        } else {
                            v1437 = v1391;
                            v1438 = v1392;
                        }
                        let v1439 = v9 - v1391;
                        let v1441 = v1391 / v116;
                        let v1446 = v202 - v1441;
                        let v1451 = v1437 / v116;
                        let v1456 = v202 - v1451;
                        let v1458 = v1456.ln();
                        let v1460 = (((v1438 - (Lanes([0.0, (v155 * v1451), 0.0]))) / v116) * v205) * (v213 / v1456);
                        let v1461 = v202 - v618;
                        let v1462 = v202 - v1339;
                        let v1465 = (v1458 * v1461).exp();
                        let v1467 = v202 - v1465;
                        let v1478 = ((v1446.ln()) * v1462).exp();
                        let v1480 = v202 - v1478;
                        let v1491 = (v1458 * v1462).exp();
                        let v1493 = v202 - v1491;
                        let v1504 = (((v1083 * v1467) / v1461) + ((v1362 * v1480) / v1462)) - ((v1362 * v1493) / v1462);
                        let v1516 = (v1504 * v116) + (v1348 * v1439);
                        let v1517 = (((((((Lanes([0.0, (v1084 * v1467), 0.0])) + ((((v1460 * v1461) * v1465) * v205) * v1083)) / v1461) + (((Lanes([0.0, (v1365 * v1480), 0.0])) + ((((((((v1392 - (Lanes([0.0, (v155 * v1441), 0.0]))) / v116) * v205) * (v213 / v1446)) * v1462) * v1478) * v205) * v1362)) / v1462)) - (((Lanes([0.0, (v1365 * v1493), 0.0])) + ((((v1460 * v1462) * v1491) * v205) * v1362)) / v1462)) * v116) + (Lanes([0.0, (v155 * v1504), 0.0]))) + ((Lanes([0.0, (v1349 * v1439), 0.0])) + ((v1368 - v1392) * v1348));
                        v1376 = v1516;
                        v1377 = v1517;
                    } else {
                        v1376 = v817;
                        v1377 = v862;
                    }
                    v1337 = v1376;
                    v1338 = v1377;
                } else {
                    let v1336 = if v1083 > v817 { 1.0 } else { 0.0 };
                    let v1587: f64;
                    let v1588: Lanes<3>;
                    if v1336 != 0.0 {
                        let v1521 = v202 - ((v1518 / v618).exp());
                        let v1522 = v116 * v1521;
                        let v1524 = v1522 - v9;
                        let v1525 = Lanes([0.0, (v155 * v1521), 0.0]);
                        let v1526 = Lanes([v10[0], 0.0, v10[1]]);
                        let v1528 = v1524 * v114;
                        let v1532 = ((v1525 - v1526) * v114) + (Lanes([0.0, (v153 * v1524), 0.0]));
                        let v1534 = v1532 * v1528;
                        let v1537 = ((v1528 * v1528) + v1024).sqrt();
                        let v1543 = (v1528 + v1537) * v261;
                        let v1550 = v1522 - (v115 * v1543);
                        let v1551 = v1525 - ((Lanes([0.0, (v154 * v1543), 0.0])) + (((v1532 + ((v1534 + v1534) * (v213 / (v256 * v1537)))) * v261) * v115));
                        let v1552 = v1550 / v116;
                        let v1557 = v202 - v1552;
                        let v1562 = v202 - v618;
                        let v1565 = ((v1557.ln()) * v1562).exp();
                        let v1567 = v202 - v1565;
                        let v1580 = ((v116 * v1567) / v1562) + (v832 * (v9 - v1550));
                        let v1582 = v1083 * v1580;
                        let v1586 = (Lanes([0.0, (v1084 * v1580), 0.0])) + (((((Lanes([0.0, (v155 * v1567), 0.0])) + ((((((((v1551 - (Lanes([0.0, (v155 * v1552), 0.0]))) / v116) * v205) * (v213 / v1557)) * v1562) * v1565) * v205) * v116)) / v1562) + ((v1526 - v1551) * v832)) * v1083);
                        v1587 = v1582;
                        v1588 = v1586;
                    } else {
                        v1587 = v817;
                        v1588 = v862;
                    }
                    v1337 = v1587;
                    v1338 = v1588;
                }
                v810 = v112;
                v811 = v1080;
                v812 = v1337;
                v813 = v151;
                v814 = v1081;
                v815 = v1338;
            }
            let v1591: f64;
            let v1592: Lanes<3>;
            if v816 != 0.0 {
                let v1589 = if v810 > v817 { 1.0 } else { 0.0 };
                let v1633: f64;
                let v1634: Lanes<3>;
                if v1589 != 0.0 {
                    let v1596 = v419 / v251;
                    let v1597 = v803 - v113;
                    let v1598 = v152 * v205;
                    let v1602 = v202 - ((v1599 / v419).exp());
                    let v1603 = v113 * v1602;
                    let v1604 = v152 * v1602;
                    let v1605 = v832 * v810;
                    let v1606 = v813 * v832;
                    let v1607 = v1596 - v419;
                    let v1608 = v803 / v113;
                    let v1617 = (v1607 * (v1608.ln())).exp();
                    let v1619 = v810 * v1617;
                    let v1622 = (v813 * v1617) + (((((((v152 * v1608) * v205) / v113) * (v213 / v1608)) * v1607) * v1617) * v810);
                    let v1623 = v1603 - v17;
                    let v1624 = Lanes([v1604, 0.0, 0.0]);
                    let v1625 = Lanes([0.0, v18[0], v18[1]]);
                    let v1627 = v1623 * v114;
                    let v1631 = ((v1624 - v1625) * v114) + (Lanes([(v153 * v1623), 0.0, 0.0]));
                    let v1632 = if v1627 < v860 { 1.0 } else { 0.0 };
                    let v1648: f64;
                    let v1649: Lanes<3>;
                    if v1632 != 0.0 {
                        let v1635 = v1627.exp();
                        let v1637 = v202 + v1635;
                        let v1638 = v1637.ln();
                        let v1646 = v1603 - (v115 * v1638);
                        let v1647 = v1624 - ((Lanes([(v154 * v1638), 0.0, 0.0])) + (((v1631 * v1635) * (v213 / v1637)) * v115));
                        v1648 = v1646;
                        v1649 = v1647;
                    } else {
                        v1648 = v17;
                        v1649 = v1625;
                    }
                    let v1654 = (v880 * v1597) + (v251 * v115);
                    let v1655 = (v1598 * v880) + (v154 * v251);
                    let v1659 = (v1597 + v1648) / v1654;
                    let v1663 = (((Lanes([v1598, 0.0, 0.0])) + v1649) - (Lanes([(v1655 * v1659), 0.0, 0.0]))) / v1654;
                    let v1664 = if v1659 < v860 { 1.0 } else { 0.0 };
                    let v1694: f64;
                    let v1695: Lanes<3>;
                    if v1664 != 0.0 {
                        let v1665 = v1659.exp();
                        let v1667 = v202 + v1665;
                        let v1677 = (-(v1597 + v1603)) / v1654;
                        let v1681 = v1677.exp();
                        let v1683 = (v1667.ln()) - v1681;
                        let v1691 = (-v1597) + (v1654 * v1683);
                        let v1693 = (Lanes([(v1598 * v205), 0.0, 0.0])) + ((Lanes([(v1655 * v1683), 0.0, 0.0])) + ((((v1663 * v1665) * (v213 / v1667)) - (Lanes([(((((v1598 + v1604) * v205) - (v1655 * v1677)) / v1654) * v1681), 0.0, 0.0]))) * v1654));
                        v1694 = v1691;
                        v1695 = v1693;
                    } else {
                        v1694 = v1648;
                        v1695 = v1649;
                    }
                    let v1696 = v17 - v1648;
                    let v1698 = v1648 / v113;
                    let v1703 = v202 - v1698;
                    let v1708 = v1694 / v113;
                    let v1713 = v202 - v1708;
                    let v1715 = v1713.ln();
                    let v1717 = (((v1695 - (Lanes([(v152 * v1708), 0.0, 0.0]))) / v113) * v205) * (v213 / v1713);
                    let v1718 = v202 - v419;
                    let v1719 = v202 - v1596;
                    let v1722 = (v1715 * v1718).exp();
                    let v1724 = v202 - v1722;
                    let v1735 = ((v1703.ln()) * v1719).exp();
                    let v1737 = v202 - v1735;
                    let v1748 = (v1715 * v1719).exp();
                    let v1750 = v202 - v1748;
                    let v1761 = (((v810 * v1724) / v1718) + ((v1619 * v1737) / v1719)) - ((v1619 * v1750) / v1719);
                    let v1773 = (v1761 * v113) + (v1605 * v1696);
                    let v1774 = (((((((Lanes([(v813 * v1724), 0.0, 0.0])) + ((((v1717 * v1718) * v1722) * v205) * v810)) / v1718) + (((Lanes([(v1622 * v1737), 0.0, 0.0])) + ((((((((v1649 - (Lanes([(v152 * v1698), 0.0, 0.0]))) / v113) * v205) * (v213 / v1703)) * v1719) * v1735) * v205) * v1619)) / v1719)) - (((Lanes([(v1622 * v1750), 0.0, 0.0])) + ((((v1717 * v1719) * v1748) * v205) * v1619)) / v1719)) * v113) + (Lanes([(v152 * v1761), 0.0, 0.0]))) + ((Lanes([(v1606 * v1696), 0.0, 0.0])) + ((v1625 - v1649) * v1605));
                    v1633 = v1773;
                    v1634 = v1774;
                } else {
                    v1633 = v817;
                    v1634 = v822;
                }
                v1591 = v1633;
                v1592 = v1634;
            } else {
                let v1590 = if v810 > v817 { 1.0 } else { 0.0 };
                let v1844: f64;
                let v1845: Lanes<3>;
                if v1590 != 0.0 {
                    let v1778 = v202 - ((v1775 / v419).exp());
                    let v1779 = v113 * v1778;
                    let v1781 = v1779 - v17;
                    let v1782 = Lanes([(v152 * v1778), 0.0, 0.0]);
                    let v1783 = Lanes([0.0, v18[0], v18[1]]);
                    let v1785 = v1781 * v114;
                    let v1789 = ((v1782 - v1783) * v114) + (Lanes([(v153 * v1781), 0.0, 0.0]));
                    let v1791 = v1789 * v1785;
                    let v1794 = ((v1785 * v1785) + v1024).sqrt();
                    let v1800 = (v1785 + v1794) * v261;
                    let v1807 = v1779 - (v115 * v1800);
                    let v1808 = v1782 - ((Lanes([(v154 * v1800), 0.0, 0.0])) + (((v1789 + ((v1791 + v1791) * (v213 / (v256 * v1794)))) * v261) * v115));
                    let v1809 = v1807 / v113;
                    let v1814 = v202 - v1809;
                    let v1819 = v202 - v419;
                    let v1822 = ((v1814.ln()) * v1819).exp();
                    let v1824 = v202 - v1822;
                    let v1837 = ((v113 * v1824) / v1819) + (v832 * (v17 - v1807));
                    let v1839 = v810 * v1837;
                    let v1843 = (Lanes([(v813 * v1837), 0.0, 0.0])) + (((((Lanes([(v152 * v1824), 0.0, 0.0])) + ((((((((v1808 - (Lanes([(v152 * v1809), 0.0, 0.0]))) / v113) * v205) * (v213 / v1814)) * v1819) * v1822) * v205) * v113)) / v1819) + ((v1783 - v1808) * v832)) * v810);
                    v1844 = v1839;
                    v1845 = v1843;
                } else {
                    v1844 = v817;
                    v1845 = v822;
                }
                v1591 = v1844;
                v1592 = v1845;
            }
            let v1593 = v1591 + v811;
            let v1594 = v1592 + v814;
            let v1595 = if v810 > v817 { 1.0 } else { 0.0 };
            let v1919: f64;
            let v1920: Lanes<3>;
            if v1595 != 0.0 {
                let v1849 = v202 - ((v1846 / v419).exp());
                let v1850 = v113 * v1849;
                let v1852 = v1850 - v17;
                let v1853 = Lanes([(v152 * v1849), 0.0, 0.0]);
                let v1856 = v1852 * v114;
                let v1860 = ((v1853 - (Lanes([0.0, v18[0], v18[1]]))) * v114) + (Lanes([(v153 * v1852), 0.0, 0.0]));
                let v1862 = v1860 * v1856;
                let v1865 = ((v1856 * v1856) + v1024).sqrt();
                let v1868 = (v1862 + v1862) * (v213 / (v256 * v1865));
                let v1871 = (v1856 + v1865) * v261;
                let v1872 = (v1860 + v1868) * v261;
                let v1880 = v1871 / v1865;
                let v1883 = (v1872 - (v1868 * v1880)) / v1865;
                let v1884 = -v419;
                let v1885 = (v1850 - (v115 * v1871)) / v113;
                let v1890 = v202 - v1885;
                let v1897 = (v1884 * (v1890.ln())).exp();
                let v1899 = v810 * v1897;
                let v1908 = v832 * v810;
                let v1910 = v202 - v1880;
                let v1917 = (v1899 * v1880) + (v1908 * v1910);
                let v1918 = ((((Lanes([(v813 * v1897), 0.0, 0.0])) + ((((((((v1853 - ((Lanes([(v154 * v1871), 0.0, 0.0])) + (v1872 * v115))) - (Lanes([(v152 * v1885), 0.0, 0.0]))) / v113) * v205) * (v213 / v1890)) * v1884) * v1897) * v810)) * v1880) + (v1883 * v1899)) + ((Lanes([((v813 * v832) * v1910), 0.0, 0.0])) + ((v1883 * v205) * v1908));
                v1919 = v1917;
                v1920 = v1918;
            } else {
                v1919 = v817;
                v1920 = v822;
            }
            let v1930: f64;
            let v1931: Lanes<4>;
            if v61 != 0.0 {
                let v1921 = v117 - v17;
                let v1924 = (Lanes([v156, 0.0, 0.0])) - (Lanes([0.0, v18[0], v18[1]]));
                let v1925 = Lanes([v1924[0], v1924[1], v1924[2], 0.0]);
                v1930 = v1921;
                v1931 = v1925;
            } else {
                let v1926 = v27 - v118;
                let v1929 = (Lanes([0.0, v30[0], v30[1], v30[2]])) - (Lanes([v157, 0.0, 0.0, 0.0]));
                v1930 = v1926;
                v1931 = v1929;
            }
            let v1936 = (v1931 * v114) + (Lanes([(v153 * v1930), 0.0, 0.0, 0.0]));
            let v1937 = (v1930 * v114) - v202;
            let v1939 = v1936 * v1937;
            let v1942 = ((v1937 * v1937) + v1024).sqrt();
            let v1950 = v202 + ((v1937 + v1942) / v240);
            let v1951 = v1950 * v115;
            let v1955 = (((v1936 + ((v1939 + v1939) * (v213 / (v256 * v1942)))) / v240) * v115) + (Lanes([(v154 * v1950), 0.0, 0.0, 0.0]));
            let v1956 = v1951 / v119;
            let v1972 = (v1969 * (v1956.ln())).exp();
            let v1974 = v202 + v1972;
            let v1980 = ((v1974.ln()) / v1969).exp();
            let v1982 = (v1951 * v120) / v1980;
            let v1990 = (v1951 - v119) / v1989;
            let v1991 = (v1955 - (Lanes([v158, 0.0, 0.0, 0.0]))) / v1989;
            let v1993 = v1991 * v1990;
            let v1997 = ((v1990 * v1990) + v1995).sqrt();
            let v2005 = v202 + (v261 * (v1990 + v1997));
            let v2006 = v1982 * v2005;
            let v2009 = (((((v1955 * v120) + (Lanes([(v159 * v1951), 0.0, 0.0, 0.0]))) - (((((((((v1955 - (Lanes([(v158 * v1956), 0.0, 0.0, 0.0]))) / v119) * (v213 / v1956)) * v1969) * v1972) * (v213 / v1974)) / v1969) * v1980) * v1982)) / v1980) * v2005) + (((v1991 + ((v1993 + v1993) * (v213 / (v256 * v1997)))) * v261) * v1982);
            let v2011 = if (if v1919 > v817 { 1.0 } else { 0.0 }) != 0.0 && v1595 != 0.0 { 1.0 } else { 0.0 };
            let v2022: f64;
            let v2023: f64;
            let v2024: Lanes<3>;
            let v2025: Lanes<3>;
            if v2011 != 0.0 {
                let v2012 = v810 / v1919;
                let v2016 = ((Lanes([v813, 0.0, 0.0])) - (v1920 * v2012)) / v1919;
                let v2017 = v1591 / v810;
                let v2021 = (v1592 - (Lanes([(v813 * v2017), 0.0, 0.0]))) / v810;
                v2022 = v2017;
                v2023 = v2012;
                v2024 = v2021;
                v2025 = v2016;
            } else {
                v2022 = v817;
                v2023 = v202;
                v2024 = v822;
                v2025 = v822;
            }
            let v2026 = if v121 > v817 { 1.0 } else { 0.0 };
            let v2109: f64;
            let v2110: Lanes<3>;
            if v2026 != 0.0 {
                let v2034 = ((-(v123.ln())) / v281).exp();
                let v2036 = v202 - v2034;
                let v2038 = v122 * v2036;
                let v2042 = v2038 - v25;
                let v2043 = Lanes([((v161 * v2036) + ((((((v162 * (v213 / v123)) * v205) / v281) * v2034) * v205) * v122)), 0.0, 0.0]);
                let v2044 = Lanes([0.0, v26[0], v26[1]]);
                let v2046 = v2042 * v114;
                let v2050 = ((v2043 - v2044) * v114) + (Lanes([(v153 * v2042), 0.0, 0.0]));
                let v2052 = v2050 * v2046;
                let v2055 = ((v2046 * v2046) + v1024).sqrt();
                let v2061 = (v2046 + v2055) * v261;
                let v2068 = v2038 - (v115 * v2061);
                let v2069 = v2043 - ((Lanes([(v154 * v2061), 0.0, 0.0])) + (((v2050 + ((v2052 + v2052) * (v213 / (v256 * v2055)))) * v261) * v115));
                let v2070 = v2068 / v122;
                let v2075 = v202 - v2070;
                let v2080 = v202 - v281;
                let v2083 = ((v2075.ln()) * v2080).exp();
                let v2085 = v202 - v2083;
                let v2094 = v25 - v2068;
                let v2101 = ((v122 * v2085) / v2080) + (v123 * v2094);
                let v2103 = v121 * v2101;
                let v2107 = (Lanes([(v160 * v2101), 0.0, 0.0])) + (((((Lanes([(v161 * v2085), 0.0, 0.0])) + ((((((((v2069 - (Lanes([(v161 * v2070), 0.0, 0.0]))) / v122) * v205) * (v213 / v2075)) * v2080) * v2083) * v205) * v122)) / v2080) + ((Lanes([(v162 * v2094), 0.0, 0.0])) + ((v2044 - v2069) * v123))) * v121);
                v2109 = v2103;
                v2110 = v2107;
            } else {
                v2109 = v817;
                v2110 = v2108;
            }
            let v2111 = v2109 / v121;
            let v2115 = (v2110 - (Lanes([(v160 * v2111), 0.0, 0.0]))) / v121;
            let v2117: f64;
            let v2118: f64;
            let v2119: f64;
            let v2120: Lanes<3>;
            if v64 != 0.0 {
                let v2116 = if v124 > v817 { 1.0 } else { 0.0 };
                let v2203: f64;
                let v2204: Lanes<3>;
                if v2116 != 0.0 {
                    let v2129 = ((-(v126.ln())) / v336).exp();
                    let v2131 = v202 - v2129;
                    let v2133 = v125 * v2131;
                    let v2137 = v2133 - v25;
                    let v2138 = Lanes([((v164 * v2131) + ((((((v165 * (v213 / v126)) * v205) / v336) * v2129) * v205) * v125)), 0.0, 0.0]);
                    let v2139 = Lanes([0.0, v26[0], v26[1]]);
                    let v2141 = v2137 * v114;
                    let v2145 = ((v2138 - v2139) * v114) + (Lanes([(v153 * v2137), 0.0, 0.0]));
                    let v2147 = v2145 * v2141;
                    let v2150 = ((v2141 * v2141) + v1024).sqrt();
                    let v2156 = (v2141 + v2150) * v261;
                    let v2163 = v2133 - (v115 * v2156);
                    let v2164 = v2138 - ((Lanes([(v154 * v2156), 0.0, 0.0])) + (((v2145 + ((v2147 + v2147) * (v213 / (v256 * v2150)))) * v261) * v115));
                    let v2165 = v2163 / v125;
                    let v2170 = v202 - v2165;
                    let v2175 = v202 - v336;
                    let v2178 = ((v2170.ln()) * v2175).exp();
                    let v2180 = v202 - v2178;
                    let v2189 = v25 - v2163;
                    let v2196 = ((v125 * v2180) / v2175) + (v126 * v2189);
                    let v2198 = v124 * v2196;
                    let v2202 = (Lanes([(v163 * v2196), 0.0, 0.0])) + (((((Lanes([(v164 * v2180), 0.0, 0.0])) + ((((((((v2164 - (Lanes([(v164 * v2165), 0.0, 0.0]))) / v125) * v205) * (v213 / v2170)) * v2175) * v2178) * v205) * v125)) / v2175) + ((Lanes([(v165 * v2189), 0.0, 0.0])) + ((v2139 - v2164) * v126))) * v124);
                    v2203 = v2198;
                    v2204 = v2202;
                } else {
                    v2203 = v817;
                    v2204 = v2108;
                }
                let v2205 = v2203 / v124;
                let v2209 = (v2204 - (Lanes([(v163 * v2205), 0.0, 0.0]))) / v124;
                v2117 = v125;
                v2118 = v2205;
                v2119 = v164;
                v2120 = v2209;
            } else {
                v2117 = v122;
                v2118 = v2111;
                v2119 = v161;
                v2120 = v2115;
            }
            let v2266: f64;
            let v2267: Lanes<3>;
            if v2121 != 0.0 {
                v2266 = v202;
                v2267 = v2108;
            } else {
                let v2211 = v2210 * v115;
                let v2212 = v154 * v2210;
                let v2214 = Lanes([v2119, 0.0, 0.0]);
                let v2217 = (v2117 - v25) / v2211;
                let v2221 = ((v2214 - (Lanes([0.0, v26[0], v26[1]]))) - (Lanes([(v2212 * v2217), 0.0, 0.0]))) / v2211;
                let v2223 = v2221 * v2217;
                let v2226 = ((v2217 * v2217) + v1024).sqrt();
                let v2230 = v2217 + v2226;
                let v2241 = (v2117 - ((v2211 * v2230) * v261)) / v2117;
                let v2246 = v202 - v2241;
                let v2254 = (v2251 * (v2246.ln())).exp();
                let v2256 = v202 - v2254;
                let v2258 = v127 * v2256;
                let v2262 = (Lanes([(v166 * v2256), 0.0, 0.0])) + (((((((((v2214 - (((Lanes([(v2212 * v2230), 0.0, 0.0])) + ((v2221 + ((v2223 + v2223) * (v213 / (v256 * v2226)))) * v2211)) * v261)) - (Lanes([(v2119 * v2241), 0.0, 0.0]))) / v2117) * v205) * (v213 / v2246)) * v2251) * v2254) * v205) * v127);
                let v2265 = if (v2258.abs()) >= v2264 { 1.0 } else { 0.0 };
                let v2333: f64;
                let v2334: Lanes<3>;
                if v2265 != 0.0 {
                    let v2323 = v2258.exp();
                    let v2326 = (v2323 - v202) / v2258;
                    let v2329 = ((v2262 * v2323) - (v2262 * v2326)) / v2258;
                    v2333 = v2326;
                    v2334 = v2329;
                } else {
                    let v2331 = v2262 * v261;
                    let v2332 = v202 + (v2258 * v261);
                    v2333 = v2332;
                    v2334 = v2331;
                }
                v2266 = v2333;
                v2267 = v2334;
            }
            let v2272 = (v2266 * v2118) / v128;
            let v2276 = (((v2267 * v2118) + (v2120 * v2266)) - (Lanes([(v167 * v2272), 0.0, 0.0]))) / v128;
            let v2280 = v2024 / v2278;
            let v2287 = ((Lanes([v2276[0], 0.0, v2276[1], v2276[2]])) + (Lanes([v2280[0], v2280[1], v2280[2], 0.0]))) * v2285;
            let v2288 = (v2285 * ((v202 + v2272) + (v2022 / v2278))) - v202;
            let v2290 = v2287 * v2288;
            let v2293 = ((v2288 * v2288) + v1024).sqrt();
            let v2303 = v2302 * (v202 + ((v2288 + v2293) / v240));
            let v2304 = ((v2287 + ((v2290 + v2290) * (v213 / (v256 * v2293)))) / v240) * v2302;
            let v2312 = v202 / v2023;
            let v2320 = (v129 + (v2306 * (v2023 - v202))) + (v2317 * (v2312 - v202));
            let v2321 = ((Lanes([v168, 0.0, 0.0])) + (v2025 * v2306)) + ((((v2025 * v2312) * v205) / v2023) * v2317);
            let v2348: f64;
            let v2349: Lanes<3>;
            if v2322 != 0.0 {
                let v2335 = v2320 / v129;
                let v2341 = v202 + (v2335 - v202);
                let v2342 = v130 / v2341;
                let v2346 = ((Lanes([v169, 0.0, 0.0])) - (((v2321 - (Lanes([(v168 * v2335), 0.0, 0.0]))) / v129) * v2342)) / v2341;
                v2348 = v2342;
                v2349 = v2346;
            } else {
                let v2347 = Lanes([v169, 0.0, 0.0]);
                v2348 = v130;
                v2349 = v2347;
            }
            let v2351 = v2350 * v115;
            let v2353 = v25 / v2351;
            let v2355 = Lanes([0.0, v26[0], v26[1]]);
            let v2358 = (v2355 - (Lanes([((v154 * v2350) * v2353), 0.0, 0.0]))) / v2351;
            let v2359 = if v2353 > v860 { 1.0 } else { 0.0 };
            let v2362: f64;
            let v2363: f64;
            let v2364: Lanes<3>;
            let v2365: Lanes<3>;
            if v2359 != 0.0 {
                let v2361 = v202 + (v2353 - v860);
                v2362 = v2361;
                v2363 = v860;
                v2364 = v2358;
                v2365 = v2108;
            } else {
                v2362 = v202;
                v2363 = v2353;
                v2364 = v2108;
                v2365 = v2358;
            }
            let v2366 = rspice_limexp(v2363);
            let v2368 = v2362 * v2366;
            let v2372 = v131 * v2368;
            let v2376 = (Lanes([(v170 * v2368), 0.0, 0.0])) + (((v2364 * v2366) + ((v2365 * v2366) * v2362)) * v131);
            let v2378 = v2377 * v115;
            let v2380 = v17 / v2378;
            let v2382 = Lanes([0.0, v18[0], v18[1]]);
            let v2385 = (v2382 - (Lanes([((v154 * v2377) * v2380), 0.0, 0.0]))) / v2378;
            let v2386 = if v2380 > v860 { 1.0 } else { 0.0 };
            let v2389: f64;
            let v2390: f64;
            let v2391: Lanes<3>;
            let v2392: Lanes<3>;
            if v2386 != 0.0 {
                let v2388 = v202 + (v2380 - v860);
                v2389 = v2388;
                v2390 = v860;
                v2391 = v2385;
                v2392 = v822;
            } else {
                v2389 = v202;
                v2390 = v2380;
                v2391 = v822;
                v2392 = v2385;
            }
            let v2393 = rspice_limexp(v2390);
            let v2395 = v2389 * v2393;
            let v2399 = v131 * v2395;
            let v2403 = (Lanes([(v170 * v2395), 0.0, 0.0])) + (((v2391 * v2393) + ((v2392 * v2393) * v2389)) * v131);
            let v2477: f64;
            let v2478: f64;
            let v2479: Lanes<4>;
            let v2480: Lanes<4>;
            if v2404 != 0.0 {
                let v2405 = v2372 / v2348;
                let v2406 = v2349 * v2405;
                let v2407 = Lanes([v2376[0], 0.0, v2376[1], v2376[2]]);
                let v2413 = v2403 / v2411;
                let v2414 = v2405 + (v2399 / v2411);
                let v2416 = ((v2407 - (Lanes([v2406[0], v2406[1], v2406[2], 0.0]))) / v2348) + (Lanes([v2413[0], v2413[1], v2413[2], 0.0]));
                let v2417 = v2372 / v2006;
                let v2421 = v2372 * v2417;
                let v2422 = v2376 * v2417;
                let v2426 = v132 / v133;
                let v2430 = v2421 * v2426;
                let v2434 = (((Lanes([v2422[0], 0.0, v2422[1], v2422[2]])) + (((v2407 - (v2009 * v2417)) / v2006) * v2372)) * v2426) + (Lanes([(((v171 - (v172 * v2426)) / v133) * v2421), 0.0, 0.0, 0.0]));
                let v2436 = v213 / v2430;
                let v2441 = (v2438 * (v2430.ln())).exp();
                let v2443 = v2414 + v2441;
                let v2444 = v2416 + (((v2434 * v2436) * v2438) * v2441);
                let v2445 = v2372 / v133;
                let v2449 = (v2376 - (Lanes([(v172 * v2445), 0.0, 0.0]))) / v133;
                let v2456 = (v2414 + v2445) + v2441;
                let v2457 = (v2416 + (Lanes([v2449[0], 0.0, v2449[1], v2449[2]]))) + (((v2434 * v2436) * v2438) * v2441);
                v2477 = v2443;
                v2478 = v2456;
                v2479 = v2444;
                v2480 = v2457;
            } else {
                let v2458 = v2372 / v2348;
                let v2459 = v2349 * v2458;
                let v2465 = v2403 / v2411;
                let v2466 = v2458 + (v2399 / v2411);
                let v2468 = (((Lanes([v2376[0], 0.0, v2376[1], v2376[2]])) - (Lanes([v2459[0], v2459[1], v2459[2], 0.0]))) / v2348) + (Lanes([v2465[0], v2465[1], v2465[2], 0.0]));
                let v2469 = v2372 / v133;
                let v2473 = (v2376 - (Lanes([(v172 * v2469), 0.0, 0.0]))) / v133;
                let v2474 = v2466 + v2469;
                let v2476 = v2468 + (Lanes([v2473[0], 0.0, v2473[1], v2473[2]]));
                v2477 = v2466;
                v2478 = v2474;
                v2479 = v2468;
                v2480 = v2476;
            }
            let v2481 = v2303 * v2303;
            let v2482 = v2304 * v2303;
            let v2483 = v2482 + v2482;
            let v2486 = (v2481 + v2477).sqrt();
            let v2490 = v2303 + v2486;
            let v2491 = v2304 + ((v2483 + v2479) * (v213 / (v256 * v2486)));
            let v2494 = (v2481 + v2478).sqrt();
            let v2498 = v2303 + v2494;
            let v2499 = v2304 + ((v2483 + v2480) * (v213 / (v256 * v2494)));
            let v2503 = if ((v2478 - v2477).abs()) > v2502 { 1.0 } else { 0.0 };
            let v2545: f64;
            let v2546: Lanes<4>;
            if v2503 != 0.0 {
                let v2505 = v202 + v2504;
                let v2508 = (v2006 / v2505) / v2372;
                let v2509 = v2376 * v2508;
                let v2512 = ((v2009 / v2505) - (Lanes([v2509[0], 0.0, v2509[1], v2509[2]]))) / v2372;
                let v2519 = v2498 - v2490;
                let v2525 = v202 + (v2508 * v2519);
                let v2526 = (v202 - (v2508 * v2490)) / v2525;
                let v2529 = ((((v2512 * v2490) + (v2491 * v2508)) * v205) - (((v2512 * v2519) + ((v2499 - v2491) * v2508)) * v2526)) / v2525;
                let v2531 = v2529 * v2526;
                let v2535 = ((v2526 * v2526) + v2533).sqrt();
                let v2542 = (v2535 + v2526) / v2541;
                let v2543 = (((v2531 + v2531) * (v213 / (v256 * v2535))) + v2529) / v2541;
                v2545 = v2542;
                v2546 = v2543;
            } else {
                v2545 = v817;
                v2546 = v2544;
            }
            let v2552: f64;
            let v2553: Lanes<4>;
            if v2547 != 0.0 {
                let v2637: f64;
                let v2638: Lanes<4>;
                if v2404 != 0.0 {
                    let v2556 = v2372 / v2348;
                    let v2557 = v2349 * v2556;
                    let v2558 = Lanes([v2376[0], 0.0, v2376[1], v2376[2]]);
                    let v2563 = v2403 / v2411;
                    let v2567 = v2372 / v133;
                    let v2572 = v2567 * v2545;
                    let v2573 = ((v2376 - (Lanes([(v172 * v2567), 0.0, 0.0]))) / v133) * v2545;
                    let v2583 = v2372 / v2006;
                    let v2587 = v2372 * v2583;
                    let v2588 = v2376 * v2583;
                    let v2592 = v132 / v133;
                    let v2596 = v2587 * v2592;
                    let v2606 = (v2438 * (v2596.ln())).exp();
                    let v2608 = ((v2556 + (v2399 / v2411)) + (v2572 * v2545)) + v2606;
                    let v2609 = ((((v2558 - (Lanes([v2557[0], v2557[1], v2557[2], 0.0]))) / v2348) + (Lanes([v2563[0], v2563[1], v2563[2], 0.0]))) + ((((Lanes([v2573[0], 0.0, v2573[1], v2573[2]])) + (v2546 * v2567)) * v2545) + (v2546 * v2572))) + (((((((Lanes([v2588[0], 0.0, v2588[1], v2588[2]])) + (((v2558 - (v2009 * v2583)) / v2006) * v2372)) * v2592) + (Lanes([(((v171 - (v172 * v2592)) / v133) * v2587), 0.0, 0.0, 0.0]))) * (v213 / v2596)) * v2438) * v2606);
                    v2637 = v2608;
                    v2638 = v2609;
                } else {
                    let v2610 = v2372 / v2348;
                    let v2611 = v2349 * v2610;
                    let v2617 = v2403 / v2411;
                    let v2621 = v2372 / v133;
                    let v2626 = v2621 * v2545;
                    let v2627 = ((v2376 - (Lanes([(v172 * v2621), 0.0, 0.0]))) / v133) * v2545;
                    let v2635 = (v2610 + (v2399 / v2411)) + (v2626 * v2545);
                    let v2636 = ((((Lanes([v2376[0], 0.0, v2376[1], v2376[2]])) - (Lanes([v2611[0], v2611[1], v2611[2], 0.0]))) / v2348) + (Lanes([v2617[0], v2617[1], v2617[2], 0.0]))) + ((((Lanes([v2627[0], 0.0, v2627[1], v2627[2]])) + (v2546 * v2621)) * v2545) + (v2546 * v2626));
                    v2637 = v2635;
                    v2638 = v2636;
                }
                let v2641 = (v2481 + v2637).sqrt();
                let v2645 = v2303 + v2641;
                let v2646 = v2304 + ((v2483 + v2638) * (v213 / (v256 * v2641)));
                v2552 = v2645;
                v2553 = v2646;
            } else {
                let v2549 = v2548 * v2303;
                let v2550 = v2304 * v2548;
                let v2676: f64;
                let v2677: Lanes<4>;
                if v2551 != 0.0 {
                    v2676 = v817;
                    v2677 = v2544;
                } else {
                    let v2647 = v2372 / v2348;
                    let v2648 = v2349 * v2647;
                    let v2654 = v2403 / v2411;
                    let v2658 = v2372 / v133;
                    let v2663 = v2658 * v2545;
                    let v2664 = ((v2376 - (Lanes([(v172 * v2658), 0.0, 0.0]))) / v133) * v2545;
                    let v2674 = -((v2647 + (v2399 / v2411)) + (v2663 * v2545));
                    let v2675 = (((((Lanes([v2376[0], 0.0, v2376[1], v2376[2]])) - (Lanes([v2648[0], v2648[1], v2648[2], 0.0]))) / v2348) + (Lanes([v2654[0], v2654[1], v2654[2], 0.0]))) + ((((Lanes([v2664[0], 0.0, v2664[1], v2664[2]])) + (v2546 * v2658)) * v2545) + (v2546 * v2663))) * v205;
                    v2676 = v2674;
                    v2677 = v2675;
                }
                let v2678 = -v2372;
                let v2683 = ((v2376 * v205) * v2372) + (v2376 * v2678);
                let v2684 = (v2678 * v2372) / v2006;
                let v2694 = (v2684 * v132) / v133;
                let v2699 = v2549 * v2549;
                let v2700 = v2550 * v2549;
                let v2701 = v2700 + v2700;
                let v2705 = v2676 - (v2699 * v2702);
                let v2706 = v2677 - (v2701 * v2702);
                let v2707 = v240 * v2549;
                let v2724 = (((v2707 * v2699) / v2713) - ((v2549 * v2676) * v2702)) + v2694;
                let v2725 = (((((v2550 * v240) * v2699) + (v2701 * v2707)) / v2713) - (((v2550 * v2676) + (v2677 * v2549)) * v2702)) + (((((((Lanes([v2683[0], 0.0, v2683[1], v2683[2]])) - (v2009 * v2684)) / v2006) * v132) + (Lanes([(v171 * v2684), 0.0, 0.0, 0.0]))) - (Lanes([(v172 * v2694), 0.0, 0.0, 0.0]))) / v133);
                let v2727 = v2725 * v2724;
                let v2732 = v2705 * v2705;
                let v2733 = v2706 * v2705;
                let v2735 = v2732 * v2705;
                let v2738 = ((v2733 + v2733) * v2705) + (v2706 * v2732);
                let v2741 = ((v2724 * v2724) * v2729) + (v2735 / v2713);
                let v2742 = ((v2727 + v2727) * v2729) + (v2738 / v2713);
                let v2745 = if (v2741.abs()) < v2744 { 1.0 } else { 0.0 };
                let v2758: f64;
                let v2759: Lanes<4>;
                if v2745 != 0.0 {
                    let v2749 = (v2746 * v2724) / v2705;
                    let v2755 = v2749 - (v2549 * v2702);
                    let v2756 = (((v2725 * v2746) - (v2706 * v2749)) / v2705) - (v2550 * v2702);
                    v2758 = v2755;
                    v2759 = v2756;
                } else {
                    let v2757 = if v2741 > v817 { 1.0 } else { 0.0 };
                    let v2792: f64;
                    let v2793: Lanes<4>;
                    if v2757 != 0.0 {
                        let v2762 = (-v2724) * v261;
                        let v2763 = (v2725 * v205) * v261;
                        let v2764 = v2741.sqrt();
                        let v2767 = v2742 * (v213 / (v256 * v2764));
                        let v2768 = v2762 + v2764;
                        let v2769 = v2763 + v2767;
                        let v2770 = if v2768 > v817 { 1.0 } else { 0.0 };
                        let v2812: f64;
                        let v2813: Lanes<4>;
                        if v2770 != 0.0 {
                            let v2799 = (v2702 * (v2768.ln())).exp();
                            let v2800 = ((v2769 * (v213 / v2768)) * v2702) * v2799;
                            v2812 = v2799;
                            v2813 = v2800;
                        } else {
                            let v2801 = -v2768;
                            let v2808 = (v2702 * (v2801.ln())).exp();
                            let v2810 = -v2808;
                            let v2811 = ((((v2769 * v205) * (v213 / v2801)) * v2702) * v2808) * v205;
                            v2812 = v2810;
                            v2813 = v2811;
                        }
                        let v2814 = v2762 - v2764;
                        let v2815 = v2763 - v2767;
                        let v2816 = if v2814 > v817 { 1.0 } else { 0.0 };
                        let v2835: f64;
                        let v2836: Lanes<4>;
                        if v2816 != 0.0 {
                            let v2822 = (v2702 * (v2814.ln())).exp();
                            let v2823 = ((v2815 * (v213 / v2814)) * v2702) * v2822;
                            v2835 = v2822;
                            v2836 = v2823;
                        } else {
                            let v2824 = -v2814;
                            let v2831 = (v2702 * (v2824.ln())).exp();
                            let v2833 = -v2831;
                            let v2834 = ((((v2815 * v205) * (v213 / v2824)) * v2702) * v2831) * v205;
                            v2835 = v2833;
                            v2836 = v2834;
                        }
                        let v2841 = (v2812 + v2835) - (v2549 * v2702);
                        let v2842 = (v2813 + v2836) - (v2550 * v2702);
                        v2792 = v2841;
                        v2793 = v2842;
                    } else {
                        let v2773 = (-v2724) * v261;
                        let v2776 = v2775 / v2735;
                        let v2780 = v2776.sqrt();
                        let v2784 = v2773 * v2780;
                        let v2788 = v2784 * v2784;
                        let v2789 = ((((v2725 * v205) * v261) * v2780) + (((((v2738 * v2776) * v205) / v2735) * (v213 / (v256 * v2780))) * v2773)) * v2784;
                        let v2790 = v2789 + v2789;
                        let v2791 = if v2784 >= v817 { 1.0 } else { 0.0 };
                        let v2878: f64;
                        let v2879: Lanes<4>;
                        if v2791 != 0.0 {
                            let v2843 = v202 - v2788;
                            let v2845 = v2788 / v2843;
                            let v2849 = v2845.sqrt();
                            let v2859 = v2858 - (v2849.atan());
                            let v2860 = ((((v2790 - ((v2790 * v205) * v2845)) / v2843) * (v213 / (v256 * v2849))) * (v213 / (v213 + (v2849 * v2849)))) * v205;
                            v2878 = v2859;
                            v2879 = v2860;
                        } else {
                            let v2861 = v202 - v2788;
                            let v2863 = v2788 / v2861;
                            let v2867 = v2863.sqrt();
                            let v2875 = (((v2790 - ((v2790 * v205) * v2863)) / v2861) * (v213 / (v256 * v2867))) * (v213 / (v213 + (v2867 * v2867)));
                            let v2877 = v2876 + (v2867.atan());
                            v2878 = v2877;
                            v2879 = v2875;
                        }
                        let v2885 = ((v2880 * v2705) * v2702).sqrt();
                        let v2889 = v2702 * v2878;
                        let v2891 = v2889.cos();
                        let v2901 = (v2885 * v2891) - (v2549 * v2702);
                        let v2902 = (((((v2706 * v2880) * v2702) * (v213 / (v256 * v2885))) * v2891) + (((v2879 * v2702) * (v205 * (v2889.sin()))) * v2885)) - (v2550 * v2702);
                        v2792 = v2901;
                        v2793 = v2902;
                    }
                    v2758 = v2792;
                    v2759 = v2793;
                }
                v2552 = v2758;
                v2553 = v2759;
            }
            let v2555 = if v2552 < v2554 { 1.0 } else { 0.0 };
            let v2903: f64;
            let v2904: Lanes<4>;
            if v2555 != 0.0 {
                v2903 = v2554;
                v2904 = v2544;
            } else {
                v2903 = v2552;
                v2904 = v2553;
            }
            let v2905 = v2372 / v2903;
            let v2909 = ((Lanes([v2376[0], 0.0, v2376[1], v2376[2]])) - (v2904 * v2905)) / v2903;
            let v2910 = v2399 / v2903;
            let v2914 = ((Lanes([v2403[0], v2403[1], v2403[2], 0.0])) - (v2904 * v2910)) / v2903;
            let v2915 = if v2905 < v2554 { 1.0 } else { 0.0 };
            let v2916: f64;
            let v2917: Lanes<4>;
            if v2915 != 0.0 {
                v2916 = v2554;
                v2917 = v2544;
            } else {
                v2916 = v2905;
                v2917 = v2909;
            }
            let v2918 = v2916 - v2910;
            let v2919 = v2917 - v2914;
            let v2921 = v2321 * v2916;
            let v2925 = v2006 / v2916;
            let v2929 = v202 - v2925;
            let v2930 = ((v2009 - (v2917 * v2925)) / v2916) * v205;
            let v2932 = v2930 * v2929;
            let v2936 = ((v2929 * v2929) + v2934).sqrt();
            let v2943 = (v2929 + v2936) / v2942;
            let v2944 = (v2930 + ((v2932 + v2932) * (v213 / (v256 * v2936)))) / v2942;
            let v2945 = v134 * v2943;
            let v2950 = v2945 * v2943;
            let v2958 = v2916 / v2006;
            let v2968 = (v2965 * (v2958.ln())).exp();
            let v2970 = v135 * v2968;
            let v2984 = ((v2320 * v2916) + ((v2970 * v2916) / v2979)) + (v2950 * v2916);
            let v2985 = (((Lanes([v2921[0], v2921[1], v2921[2], 0.0])) + (v2917 * v2320)) + (((((Lanes([(v174 * v2968), 0.0, 0.0, 0.0])) + ((((((v2917 - (v2009 * v2958)) / v2006) * (v213 / v2958)) * v2965) * v2968) * v135)) * v2916) + (v2917 * v2970)) / v2979)) + ((((((Lanes([(v173 * v2943), 0.0, 0.0, 0.0])) + (v2944 * v134)) * v2943) + (v2944 * v2945)) * v2916) + (v2917 * v2950));
            let v2987 = v2986 * v2910;
            let v2988 = v2914 * v2986;
            let v2999: f64;
            let v3000: Lanes<3>;
            if v2989 != 0.0 {
                let v2991 = v2990 * v115;
                let v2993 = v25 / v2991;
                let v2997 = (v2355 - (Lanes([((v154 * v2990) * v2993), 0.0, 0.0]))) / v2991;
                let v2998 = if v2993 > v860 { 1.0 } else { 0.0 };
                let v3004: f64;
                let v3005: f64;
                let v3006: Lanes<3>;
                let v3007: Lanes<3>;
                if v2998 != 0.0 {
                    let v3003 = v202 + (v2993 - v860);
                    v3004 = v3003;
                    v3005 = v860;
                    v3006 = v2997;
                    v3007 = v2108;
                } else {
                    v3004 = v202;
                    v3005 = v2993;
                    v3006 = v2108;
                    v3007 = v2997;
                }
                let v3008 = rspice_limexp(v3005);
                let v3014 = (v3004 * v3008) - v202;
                let v3015 = v136 * v3014;
                let v3019 = (Lanes([(v175 * v3014), 0.0, 0.0])) + (((v3006 * v3008) + ((v3007 * v3008) * v3004)) * v136);
                v2999 = v3015;
                v3000 = v3019;
            } else {
                v2999 = v817;
                v3000 = v2108;
            }
            let v3029: f64;
            let v3030: Lanes<3>;
            if v3001 != 0.0 {
                let v3021 = v3020 * v115;
                let v3023 = v25 / v3021;
                let v3027 = (v2355 - (Lanes([((v154 * v3020) * v3023), 0.0, 0.0]))) / v3021;
                let v3028 = if v3023 > v860 { 1.0 } else { 0.0 };
                let v3036: f64;
                let v3037: f64;
                let v3038: Lanes<3>;
                let v3039: Lanes<3>;
                if v3028 != 0.0 {
                    let v3035 = v202 + (v3023 - v860);
                    v3036 = v3035;
                    v3037 = v860;
                    v3038 = v3027;
                    v3039 = v2108;
                } else {
                    v3036 = v202;
                    v3037 = v3023;
                    v3038 = v2108;
                    v3039 = v3027;
                }
                let v3040 = rspice_limexp(v3037);
                let v3046 = (v3036 * v3040) - v202;
                let v3047 = v137 * v3046;
                let v3051 = (Lanes([(v176 * v3046), 0.0, 0.0])) + (((v3038 * v3040) + ((v3039 * v3040) * v3036)) * v137);
                v3029 = v3047;
                v3030 = v3051;
            } else {
                v3029 = v817;
                v3030 = v2108;
            }
            let v3031 = v2999 + v3029;
            let v3032 = v3000 + v3030;
            let v3061: f64;
            let v3062: Lanes<3>;
            if v3033 != 0.0 {
                let v3053 = v3052 * v115;
                let v3055 = v17 / v3053;
                let v3059 = (v2382 - (Lanes([((v154 * v3052) * v3055), 0.0, 0.0]))) / v3053;
                let v3060 = if v3055 > v860 { 1.0 } else { 0.0 };
                let v3069: f64;
                let v3070: f64;
                let v3071: Lanes<3>;
                let v3072: Lanes<3>;
                if v3060 != 0.0 {
                    let v3068 = v202 + (v3055 - v860);
                    v3069 = v3068;
                    v3070 = v860;
                    v3071 = v3059;
                    v3072 = v822;
                } else {
                    v3069 = v202;
                    v3070 = v3055;
                    v3071 = v822;
                    v3072 = v3059;
                }
                let v3073 = rspice_limexp(v3070);
                let v3079 = (v3069 * v3073) - v202;
                let v3080 = v138 * v3079;
                let v3084 = (Lanes([(v177 * v3079), 0.0, 0.0])) + (((v3071 * v3073) + ((v3072 * v3073) * v3069)) * v138);
                v3061 = v3080;
                v3062 = v3084;
            } else {
                v3061 = v817;
                v3062 = v822;
            }
            let v3063 = v3031 + v3061;
            let v3065 = Lanes([v3062[0], v3062[1], v3062[2], 0.0]);
            let v3066 = (Lanes([v3032[0], 0.0, v3032[1], v3032[2]])) + v3065;
            let v3087: f64;
            let v3088: Lanes<3>;
            if v816 != 0.0 {
                let v3085 = if v112 > v817 { 1.0 } else { 0.0 };
                let v3125: f64;
                let v3126: Lanes<3>;
                if v3085 != 0.0 {
                    let v3089 = v419 / v251;
                    let v3090 = v803 - v113;
                    let v3091 = v152 * v205;
                    let v3095 = v202 - ((v3092 / v419).exp());
                    let v3096 = v113 * v3095;
                    let v3097 = v152 * v3095;
                    let v3098 = v832 * v112;
                    let v3099 = v151 * v832;
                    let v3100 = v3089 - v419;
                    let v3101 = v803 / v113;
                    let v3110 = (v3100 * (v3101.ln())).exp();
                    let v3112 = v112 * v3110;
                    let v3115 = (v151 * v3110) + (((((((v152 * v3101) * v205) / v113) * (v213 / v3101)) * v3100) * v3110) * v112);
                    let v3116 = v3096 - v17;
                    let v3117 = Lanes([v3097, 0.0, 0.0]);
                    let v3119 = v3116 * v114;
                    let v3123 = ((v3117 - v2382) * v114) + (Lanes([(v153 * v3116), 0.0, 0.0]));
                    let v3124 = if v3119 < v860 { 1.0 } else { 0.0 };
                    let v3144: f64;
                    let v3145: f64;
                    let v3146: Lanes<3>;
                    let v3147: Lanes<3>;
                    if v3124 != 0.0 {
                        let v3127 = v3119.exp();
                        let v3128 = v3123 * v3127;
                        let v3129 = v202 + v3127;
                        let v3130 = v3127 / v3129;
                        let v3133 = (v3128 - (v3128 * v3130)) / v3129;
                        let v3134 = v3129.ln();
                        let v3142 = v3096 - (v115 * v3134);
                        let v3143 = v3117 - ((Lanes([(v154 * v3134), 0.0, 0.0])) + ((v3128 * (v213 / v3129)) * v115));
                        v3144 = v3142;
                        v3145 = v3130;
                        v3146 = v3143;
                        v3147 = v3133;
                    } else {
                        v3144 = v17;
                        v3145 = v202;
                        v3146 = v2382;
                        v3147 = v822;
                    }
                    let v3152 = (v880 * v3090) + (v251 * v115);
                    let v3153 = (v3091 * v880) + (v154 * v251);
                    let v3157 = (v3090 + v3144) / v3152;
                    let v3161 = (((Lanes([v3091, 0.0, 0.0])) + v3146) - (Lanes([(v3153 * v3157), 0.0, 0.0]))) / v3152;
                    let v3162 = if v3157 < v860 { 1.0 } else { 0.0 };
                    let v3196: f64;
                    let v3197: f64;
                    let v3198: Lanes<3>;
                    let v3199: Lanes<3>;
                    if v3162 != 0.0 {
                        let v3163 = v3157.exp();
                        let v3164 = v3161 * v3163;
                        let v3165 = v202 + v3163;
                        let v3166 = v3163 / v3165;
                        let v3169 = (v3164 - (v3164 * v3166)) / v3165;
                        let v3179 = (-(v3090 + v3096)) / v3152;
                        let v3183 = v3179.exp();
                        let v3185 = (v3165.ln()) - v3183;
                        let v3193 = (-v3090) + (v3152 * v3185);
                        let v3195 = (Lanes([(v3091 * v205), 0.0, 0.0])) + ((Lanes([(v3153 * v3185), 0.0, 0.0])) + (((v3164 * (v213 / v3165)) - (Lanes([(((((v3091 + v3097) * v205) - (v3153 * v3179)) / v3152) * v3183), 0.0, 0.0]))) * v3152));
                        v3196 = v3193;
                        v3197 = v3166;
                        v3198 = v3195;
                        v3199 = v3169;
                    } else {
                        v3196 = v3144;
                        v3197 = v202;
                        v3198 = v3146;
                        v3199 = v822;
                    }
                    let v3200 = v3144 / v113;
                    let v3205 = v202 - v3200;
                    let v3210 = v3196 / v113;
                    let v3215 = v202 - v3210;
                    let v3220 = -v419;
                    let v3223 = ((v3215.ln()) * v3220).exp();
                    let v3225 = v112 * v3223;
                    let v3230 = v3225 * v3145;
                    let v3238 = -v3089;
                    let v3241 = ((v3205.ln()) * v3238).exp();
                    let v3243 = v3112 * v3241;
                    let v3248 = v202 - v3197;
                    let v3254 = v202 - v3145;
                    let v3263 = ((v3230 * v3197) + (v3243 * v3248)) + (v3098 * v3254);
                    let v3264 = (((((((Lanes([(v151 * v3223), 0.0, 0.0])) + (((((((v3198 - (Lanes([(v152 * v3210), 0.0, 0.0]))) / v113) * v205) * (v213 / v3215)) * v3220) * v3223) * v112)) * v3145) + (v3147 * v3225)) * v3197) + (v3199 * v3230)) + ((((Lanes([(v3115 * v3241), 0.0, 0.0])) + (((((((v3146 - (Lanes([(v152 * v3200), 0.0, 0.0]))) / v113) * v205) * (v213 / v3205)) * v3238) * v3241) * v3112)) * v3248) + ((v3199 * v205) * v3243))) + ((Lanes([(v3099 * v3254), 0.0, 0.0])) + ((v3147 * v205) * v3098));
                    v3125 = v3263;
                    v3126 = v3264;
                } else {
                    v3125 = v817;
                    v3126 = v822;
                }
                v3087 = v3125;
                v3088 = v3126;
            } else {
                let v3086 = if v112 > v817 { 1.0 } else { 0.0 };
                let v3332: f64;
                let v3333: Lanes<3>;
                if v3086 != 0.0 {
                    let v3268 = v202 - ((v3265 / v419).exp());
                    let v3269 = v113 * v3268;
                    let v3271 = v3269 - v17;
                    let v3272 = Lanes([(v152 * v3268), 0.0, 0.0]);
                    let v3274 = v3271 * v114;
                    let v3278 = ((v3272 - v2382) * v114) + (Lanes([(v153 * v3271), 0.0, 0.0]));
                    let v3280 = v3278 * v3274;
                    let v3283 = ((v3274 * v3274) + v1024).sqrt();
                    let v3286 = (v3280 + v3280) * (v213 / (v256 * v3283));
                    let v3289 = (v3274 + v3283) * v261;
                    let v3290 = (v3278 + v3286) * v261;
                    let v3298 = v3289 / v3283;
                    let v3301 = (v3290 - (v3286 * v3298)) / v3283;
                    let v3302 = (v3269 - (v115 * v3289)) / v113;
                    let v3307 = v202 - v3302;
                    let v3312 = -v419;
                    let v3315 = (v3312 * (v3307.ln())).exp();
                    let v3325 = (v3315 * v3298) + (v832 * (v202 - v3298));
                    let v3327 = v112 * v3325;
                    let v3331 = (Lanes([(v151 * v3325), 0.0, 0.0])) + (((((((((((v3272 - ((Lanes([(v154 * v3289), 0.0, 0.0])) + (v3290 * v115))) - (Lanes([(v152 * v3302), 0.0, 0.0]))) / v113) * v205) * (v213 / v3307)) * v3312) * v3315) * v3298) + (v3301 * v3315)) + ((v3301 * v205) * v832)) * v112);
                    v3332 = v3327;
                    v3333 = v3331;
                } else {
                    v3332 = v817;
                    v3333 = v822;
                }
                v3087 = v3332;
                v3088 = v3333;
            }
            let v3338: f64;
            let v3339: Lanes<4>;
            if v63 != 0.0 {
                let v3334 = v113 - v17;
                let v3336 = (Lanes([v152, 0.0, 0.0])) - v2382;
                let v3337 = if v3334 > v817 { 1.0 } else { 0.0 };
                let v3351: f64;
                let v3352: Lanes<4>;
                if v3337 != 0.0 {
                    let v3341 = v139 / v3087;
                    let v3345 = ((Lanes([v178, 0.0, 0.0])) - (v3088 * v3341)) / v3087;
                    let v3346 = v139 / v112;
                    let v3349 = (v178 - (v151 * v3346)) / v112;
                    let v3350 = if v3334 > v3346 { 1.0 } else { 0.0 };
                    let v3403: f64;
                    let v3404: Lanes<3>;
                    if v3350 != 0.0 {
                        let v3355 = (-v3341) / v3346;
                        let v3360 = v3355.exp();
                        let v3362 = v140 * v3360;
                        let v3367 = v3341 / v3346;
                        let v3372 = v202 + v3367;
                        let v3373 = v3334 - v3346;
                        let v3374 = Lanes([v3349, 0.0, 0.0]);
                        let v3380 = v3346 + (v3372 * v3373);
                        let v3382 = v3362 * v3380;
                        let v3385 = (((Lanes([(v179 * v3360), 0.0, 0.0])) + (((((v3345 * v205) - (Lanes([(v3349 * v3355), 0.0, 0.0]))) / v3346) * v3360) * v140)) * v3380) + ((v3374 + ((((v3345 - (Lanes([(v3349 * v3367), 0.0, 0.0]))) / v3346) * v3373) + ((v3336 - v3374) * v3372))) * v3362);
                        v3403 = v3382;
                        v3404 = v3385;
                    } else {
                        let v3386 = v140 * v3334;
                        let v3393 = (-v3341) / v3334;
                        let v3397 = v3393.exp();
                        let v3399 = v3386 * v3397;
                        let v3402 = (((Lanes([(v179 * v3334), 0.0, 0.0])) + (v3336 * v140)) * v3397) + (((((v3345 * v205) - (v3336 * v3393)) / v3334) * v3397) * v3386);
                        v3403 = v3399;
                        v3404 = v3402;
                    }
                    let v3405 = v2916 * v3403;
                    let v3407 = v3404 * v2916;
                    let v3409 = (v2917 * v3403) + (Lanes([v3407[0], v3407[1], v3407[2], 0.0]));
                    v3351 = v3405;
                    v3352 = v3409;
                } else {
                    v3351 = v817;
                    v3352 = v2544;
                }
                v3338 = v3351;
                v3339 = v3352;
            } else {
                v3338 = v817;
                v3339 = v2544;
            }
            let v3340 = if v141 > v817 { 1.0 } else { 0.0 };
            let v3450: f64;
            let v3451: Lanes<4>;
            if v3340 != 0.0 {
                let v3412 = v2115 / v3410;
                let v3416 = v2024 / v3414;
                let v3421 = v2916 / v2348;
                let v3422 = v2349 * v3421;
                let v3430 = (((v202 + (v2111 / v3410)) + (v2022 / v3414)) + v3421) + (v2910 / v2411);
                let v3431 = (((Lanes([v3412[0], 0.0, v3412[1], v3412[2]])) + (Lanes([v3416[0], v3416[1], v3416[2], 0.0]))) + ((v2917 - (Lanes([v3422[0], v3422[1], v3422[2], 0.0]))) / v2348)) + (v2914 / v2411);
                let v3433 = v3431 * v3430;
                let v3436 = ((v3430 * v3430) + v2533).sqrt();
                let v3442 = v261 * (v3430 + v3436);
                let v3444 = v141 / v3442;
                let v3448 = ((Lanes([v180, 0.0, 0.0, 0.0])) - (((v3431 + ((v3433 + v3433) * (v213 / (v256 * v3436)))) * v261) * v3444)) / v3442;
                let v3449 = if v3063 > v817 { 1.0 } else { 0.0 };
                let v3470: f64;
                let v3471: Lanes<4>;
                if v3449 != 0.0 {
                    let v3457 = v3456 * v3444;
                    let v3459 = v3457 * v3063;
                    let v3463 = v3459 * v114;
                    let v3467 = ((((v3448 * v3456) * v3063) + (v3066 * v3457)) * v114) + (Lanes([(v153 * v3459), 0.0, 0.0, 0.0]));
                    let v3469 = if v3463 < v3468 { 1.0 } else { 0.0 };
                    let v3492: f64;
                    let v3493: Lanes<4>;
                    if v3469 != 0.0 {
                        let v3474 = v202 - (v261 * v3463);
                        let v3476 = v3444 * v3474;
                        let v3479 = (v3448 * v3474) + (((v3467 * v261) * v205) * v3444);
                        v3492 = v3476;
                        v3493 = v3479;
                    } else {
                        let v3480 = v3463 + v202;
                        let v3481 = v3480.ln();
                        let v3488 = (v3444 * v3481) / v3463;
                        let v3491 = (((v3448 * v3481) + ((v3467 * (v213 / v3480)) * v3444)) - (v3467 * v3488)) / v3463;
                        v3492 = v3488;
                        v3493 = v3491;
                    }
                    v3470 = v3492;
                    v3471 = v3493;
                } else {
                    v3470 = v3444;
                    v3471 = v3448;
                }
                v3450 = v3470;
                v3451 = v3471;
            } else {
                v3450 = v817;
                v3451 = v2544;
            }
            let v3452 = v3450 + v142;
            let v3454 = v3451 + (Lanes([v181, 0.0, 0.0, 0.0]));
            let v3523: f64;
            let v3524: Lanes<4>;
            if v3455 != 0.0 {
                let v3495 = v3494 * v115;
                let v3496 = v154 * v3494;
                let v3497 = v9 / v3495;
                let v3503 = rspice_limexp(v3497);
                let v3504 = (((Lanes([v10[0], 0.0, v10[1]])) - (Lanes([0.0, (v3496 * v3497), 0.0]))) / v3495) * v3503;
                let v3505 = v37 / v3495;
                let v3511 = rspice_limexp(v3505);
                let v3512 = (((Lanes([v38[0], 0.0, v38[1]])) - (Lanes([0.0, (v3496 * v3505), 0.0]))) / v3495) * v3511;
                let v3513 = v3503 - v3511;
                let v3517 = v143 * v3513;
                let v3521 = (Lanes([0.0, 0.0, (v182 * v3513), 0.0])) + (((Lanes([v3504[0], 0.0, v3504[1], v3504[2]])) - (Lanes([0.0, v3512[0], v3512[1], v3512[2]]))) * v143);
                v3523 = v3517;
                v3524 = v3521;
            } else {
                v3523 = v817;
                v3524 = v3522;
            }
            let v3537: f64;
            let v3538: Lanes<3>;
            if v3525 != 0.0 {
                let v3527 = v3526 * v115;
                let v3529 = v37 / v3527;
                let v3534 = ((Lanes([v38[0], 0.0, v38[1]])) - (Lanes([0.0, ((v154 * v3526) * v3529), 0.0]))) / v3527;
                let v3535 = if v3529 > v860 { 1.0 } else { 0.0 };
                let v3542: f64;
                let v3543: f64;
                let v3544: Lanes<3>;
                let v3545: Lanes<3>;
                if v3535 != 0.0 {
                    let v3541 = v202 + (v3529 - v860);
                    v3542 = v3541;
                    v3543 = v860;
                    v3544 = v3534;
                    v3545 = v3536;
                } else {
                    v3542 = v202;
                    v3543 = v3529;
                    v3544 = v3536;
                    v3545 = v3534;
                }
                let v3546 = rspice_limexp(v3543);
                let v3552 = (v3542 * v3546) - v202;
                let v3553 = v144 * v3552;
                let v3557 = (Lanes([0.0, (v183 * v3552), 0.0])) + (((v3544 * v3546) + ((v3545 * v3546) * v3542)) * v144);
                v3537 = v3553;
                v3538 = v3557;
            } else {
                v3537 = v817;
                v3538 = v3536;
            }
            let v3560: f64;
            let v3561: Lanes<3>;
            if v3539 != 0.0 {
                let v3558 = if v145 > v817 { 1.0 } else { 0.0 };
                let v3601: f64;
                let v3602: Lanes<3>;
                if v3558 != 0.0 {
                    let v3563 = v671 / v251;
                    let v3565 = v3564 - v146;
                    let v3566 = v185 * v205;
                    let v3570 = v202 - ((v3567 / v671).exp());
                    let v3571 = v146 * v3570;
                    let v3572 = v185 * v3570;
                    let v3573 = v832 * v145;
                    let v3574 = v184 * v832;
                    let v3575 = v3563 - v671;
                    let v3576 = v3564 / v146;
                    let v3585 = (v3575 * (v3576.ln())).exp();
                    let v3587 = v145 * v3585;
                    let v3590 = (v184 * v3585) + (((((((v185 * v3576) * v205) / v146) * (v213 / v3576)) * v3575) * v3585) * v145);
                    let v3591 = v3571 - v37;
                    let v3592 = Lanes([0.0, v3572, 0.0]);
                    let v3593 = Lanes([v38[0], 0.0, v38[1]]);
                    let v3595 = v3591 * v114;
                    let v3599 = ((v3592 - v3593) * v114) + (Lanes([0.0, (v153 * v3591), 0.0]));
                    let v3600 = if v3595 < v860 { 1.0 } else { 0.0 };
                    let v3616: f64;
                    let v3617: Lanes<3>;
                    if v3600 != 0.0 {
                        let v3603 = v3595.exp();
                        let v3605 = v202 + v3603;
                        let v3606 = v3605.ln();
                        let v3614 = v3571 - (v115 * v3606);
                        let v3615 = v3592 - ((Lanes([0.0, (v154 * v3606), 0.0])) + (((v3599 * v3603) * (v213 / v3605)) * v115));
                        v3616 = v3614;
                        v3617 = v3615;
                    } else {
                        v3616 = v37;
                        v3617 = v3593;
                    }
                    let v3622 = (v880 * v3565) + (v251 * v115);
                    let v3623 = (v3566 * v880) + (v154 * v251);
                    let v3627 = (v3565 + v3616) / v3622;
                    let v3631 = (((Lanes([0.0, v3566, 0.0])) + v3617) - (Lanes([0.0, (v3623 * v3627), 0.0]))) / v3622;
                    let v3632 = if v3627 < v860 { 1.0 } else { 0.0 };
                    let v3662: f64;
                    let v3663: Lanes<3>;
                    if v3632 != 0.0 {
                        let v3633 = v3627.exp();
                        let v3635 = v202 + v3633;
                        let v3645 = (-(v3565 + v3571)) / v3622;
                        let v3649 = v3645.exp();
                        let v3651 = (v3635.ln()) - v3649;
                        let v3659 = (-v3565) + (v3622 * v3651);
                        let v3661 = (Lanes([0.0, (v3566 * v205), 0.0])) + ((Lanes([0.0, (v3623 * v3651), 0.0])) + ((((v3631 * v3633) * (v213 / v3635)) - (Lanes([0.0, (((((v3566 + v3572) * v205) - (v3623 * v3645)) / v3622) * v3649), 0.0]))) * v3622));
                        v3662 = v3659;
                        v3663 = v3661;
                    } else {
                        v3662 = v3616;
                        v3663 = v3617;
                    }
                    let v3664 = v37 - v3616;
                    let v3666 = v3616 / v146;
                    let v3671 = v202 - v3666;
                    let v3676 = v3662 / v146;
                    let v3681 = v202 - v3676;
                    let v3683 = v3681.ln();
                    let v3685 = (((v3663 - (Lanes([0.0, (v185 * v3676), 0.0]))) / v146) * v205) * (v213 / v3681);
                    let v3686 = v202 - v671;
                    let v3687 = v202 - v3563;
                    let v3690 = (v3683 * v3686).exp();
                    let v3692 = v202 - v3690;
                    let v3703 = ((v3671.ln()) * v3687).exp();
                    let v3705 = v202 - v3703;
                    let v3716 = (v3683 * v3687).exp();
                    let v3718 = v202 - v3716;
                    let v3729 = (((v145 * v3692) / v3686) + ((v3587 * v3705) / v3687)) - ((v3587 * v3718) / v3687);
                    let v3741 = (v3729 * v146) + (v3573 * v3664);
                    let v3742 = (((((((Lanes([0.0, (v184 * v3692), 0.0])) + ((((v3685 * v3686) * v3690) * v205) * v145)) / v3686) + (((Lanes([0.0, (v3590 * v3705), 0.0])) + ((((((((v3617 - (Lanes([0.0, (v185 * v3666), 0.0]))) / v146) * v205) * (v213 / v3671)) * v3687) * v3703) * v205) * v3587)) / v3687)) - (((Lanes([0.0, (v3590 * v3718), 0.0])) + ((((v3685 * v3687) * v3716) * v205) * v3587)) / v3687)) * v146) + (Lanes([0.0, (v185 * v3729), 0.0]))) + ((Lanes([0.0, (v3574 * v3664), 0.0])) + ((v3593 - v3617) * v3573));
                    v3601 = v3741;
                    v3602 = v3742;
                } else {
                    v3601 = v817;
                    v3602 = v3536;
                }
                v3560 = v3601;
                v3561 = v3602;
            } else {
                let v3559 = if v145 > v817 { 1.0 } else { 0.0 };
                let v3812: f64;
                let v3813: Lanes<3>;
                if v3559 != 0.0 {
                    let v3746 = v202 - ((v3743 / v671).exp());
                    let v3747 = v146 * v3746;
                    let v3749 = v3747 - v37;
                    let v3750 = Lanes([0.0, (v185 * v3746), 0.0]);
                    let v3751 = Lanes([v38[0], 0.0, v38[1]]);
                    let v3753 = v3749 * v114;
                    let v3757 = ((v3750 - v3751) * v114) + (Lanes([0.0, (v153 * v3749), 0.0]));
                    let v3759 = v3757 * v3753;
                    let v3762 = ((v3753 * v3753) + v1024).sqrt();
                    let v3768 = (v3753 + v3762) * v261;
                    let v3775 = v3747 - (v115 * v3768);
                    let v3776 = v3750 - ((Lanes([0.0, (v154 * v3768), 0.0])) + (((v3757 + ((v3759 + v3759) * (v213 / (v256 * v3762)))) * v261) * v115));
                    let v3777 = v3775 / v146;
                    let v3782 = v202 - v3777;
                    let v3787 = v202 - v671;
                    let v3790 = ((v3782.ln()) * v3787).exp();
                    let v3792 = v202 - v3790;
                    let v3805 = ((v146 * v3792) / v3787) + (v832 * (v37 - v3775));
                    let v3807 = v145 * v3805;
                    let v3811 = (Lanes([0.0, (v184 * v3805), 0.0])) + (((((Lanes([0.0, (v185 * v3792), 0.0])) + ((((((((v3776 - (Lanes([0.0, (v185 * v3777), 0.0]))) / v146) * v205) * (v213 / v3782)) * v3787) * v3790) * v205) * v146)) / v3787) + ((v3751 - v3776) * v832)) * v145);
                    v3812 = v3807;
                    v3813 = v3811;
                } else {
                    v3812 = v817;
                    v3813 = v3536;
                }
                v3560 = v3812;
                v3561 = v3813;
            }
            let v3829: f64;
            let v3830: Lanes<4>;
            if v3562 != 0.0 {
                let v3815 = v30 * v2918;
                let v3819 = v113 - v17;
                let v3823 = ((Lanes([v152, 0.0, 0.0])) - v2382) * v3338;
                let v3827 = (v27 * v2918) + (v3819 * v3338);
                let v3828 = ((Lanes([0.0, v3815[0], v3815[1], v3815[2]])) + (v2919 * v27)) + ((Lanes([v3823[0], v3823[1], v3823[2], 0.0])) + (v3339 * v3819));
                v3829 = v3827;
                v3830 = v3828;
            } else {
                v3829 = v817;
                v3830 = v2544;
            }
            let v3860: f64;
            let v3861: f64;
            let v3862: f64;
            let v3863: f64;
            let v3864: f64;
            let v3865: f64;
            let v3866: Lanes<5>;
            let v3867: Lanes<5>;
            let v3868: Lanes<5>;
            let v3869: f64;
            let v3870: Lanes<5>;
            let v3871: f64;
            if v3831 != 0.0 {
                let v3833 = v3832 - v2984;
                let v3835 = Lanes([0.0, 0.0, 0.0, 0.0, v3834]);
                let v3837 = v3835 - (Lanes([v2985[0], v2985[1], v2985[2], v2985[3], 0.0]));
                let v3841 = (v3838 * v3832) * v522;
                let v3842 = (v3834 * v3838) * v522;
                let v3844 = v3843 - v2916;
                let v3846 = Lanes([0.0, 0.0, 0.0, 0.0, v3845]);
                let v3848 = v3846 - (Lanes([v2917[0], v2917[1], v2917[2], v2917[3], 0.0]));
                let v3852 = (v3849 * v3843) * v522;
                let v3853 = (v3845 * v3849) * v522;
                v3860 = v3832;
                v3861 = v3843;
                v3862 = v3833;
                v3863 = v3841;
                v3864 = v3844;
                v3865 = v3852;
                v3866 = v3835;
                v3867 = v3846;
                v3868 = v3837;
                v3869 = v3842;
                v3870 = v3848;
                v3871 = v3853;
            } else {
                let v3854 = Lanes([v2985[0], v2985[1], v2985[2], v2985[3], 0.0]);
                let v3855 = Lanes([v2917[0], v2917[1], v2917[2], v2917[3], 0.0]);
                let v3856 = Lanes([0.0, 0.0, 0.0, 0.0, v3834]);
                let v3857 = Lanes([0.0, 0.0, 0.0, 0.0, v3845]);
                v3860 = v2984;
                v3861 = v2916;
                v3862 = v3832;
                v3863 = v817;
                v3864 = v3843;
                v3865 = v817;
                v3866 = v3854;
                v3867 = v3855;
                v3868 = v3856;
                v3869 = v3858;
                v3870 = v3857;
                v3871 = v3859;
            }
            let v3886 = v8 * v3537;
            let v3887 = v3538 * v8;
            let v3888 = v8 * v3560;
            let v3889 = v3561 * v8;
            let v3890 = v8 * v812;
            let v3891 = v815 * v8;
            let v3892 = v8 * (v3874 * v9);
            let v3893 = (v10 * v3874) * v8;
            let v3894 = v8 * (v3877 * v45);
            let v3895 = (v46 * v3877) * v8;
            let v3896 = v8 * (v3061 - v3338);
            let v3897 = (v3065 - v3339) * v8;
            let v3898 = v8 * (v1593 + v2987);
            let v3899 = ((Lanes([v1594[0], v1594[1], v1594[2], 0.0])) + v2988) * v8;
            let v3900 = v8 * v3031;
            let v3901 = v3032 * v8;
            let v3902 = v8 * (v2109 + v3860);
            let v3903 = ((Lanes([v2110[0], 0.0, v2110[1], v2110[2], 0.0])) + v3866) * v8;
            let v3904 = v2919 * v8;
            let v3908 = v8 * (v3861 - v2910);
            let v3909 = (v3867 - (Lanes([v2914[0], v2914[1], v2914[2], v2914[3], 0.0]))) * v8;
            let v3910 = v3339 * v8;
            let v3911 = ctx.simparam_or("gmin", v817);
            let v3912 = v3911 * v20;
            let v3913 = v24 * v3911;
            let v3914 = ctx.simparam_or("gmin", v817);
            let v3915 = v3914 * v12;
            let v3916 = v16 * v3914;
            let v3917 = v8 * v3523;
            let v3918 = v3524 * v8;
            let v3919 = ddt(12300, v3888);
            let v3921 = v3889 * v3920;
            let v3922 = ddt(12302, v3890);
            let v3923 = v3891 * v3920;
            let v3924 = ddt(12304, v3892);
            let v3925 = v3893 * v3920;
            let v3926 = ddt(12306, v3894);
            let v3927 = v3895 * v3920;
            let v3936: f64;
            let v3937: Lanes<3>;
            if v3928 != 0.0 {
                let v3929 = v47 / v147;
                let v3934 = ((Lanes([v50[0], 0.0, v50[1]])) - (Lanes([0.0, (v186 * v3929), 0.0]))) / v147;
                v3936 = v3929;
                v3937 = v3934;
            } else {
                v3936 = v817;
                v3937 = v3935;
            }
            let v3946: f64;
            let v3947: Lanes<3>;
            if v3938 != 0.0 {
                let v3939 = v52 / v148;
                let v3944 = ((Lanes([v56[0], 0.0, v56[1]])) - (Lanes([0.0, (v187 * v3939), 0.0]))) / v148;
                v3946 = v3939;
                v3947 = v3944;
            } else {
                v3946 = v817;
                v3947 = v3945;
            }
            let v3956: f64;
            let v3957: Lanes<5>;
            if v3948 != 0.0 {
                let v3949 = v57 / v3452;
                let v3950 = v3454 * v3949;
                let v3954 = ((Lanes([v60[0], 0.0, 0.0, v60[1], 0.0])) - (Lanes([0.0, v3950[0], v3950[1], v3950[2], v3950[3]]))) / v3452;
                v3956 = v3949;
                v3957 = v3954;
            } else {
                v3956 = v817;
                v3957 = v3955;
            }
            let v3958 = ddt(12334, v3898);
            let v3959 = v3899 * v3920;
            let v3960 = ddt(12337, v3902);
            let v3961 = v3903 * v3920;
            let v3968: f64;
            let v3969: f64;
            let v3970: f64;
            let v3971: f64;
            if v3962 != 0.0 {
                v3968 = v817;
                v3969 = v817;
                v3970 = v110;
                v3971 = v110;
            } else {
                let v3964 = v3963 * v67;
                let v3965 = v494 * v3963;
                let v3966 = ddt(12350, v3964);
                let v3967 = v3965 * v3920;
                v3968 = v3966;
                v3969 = v3964;
                v3970 = v3967;
                v3971 = v3965;
            }
            let v3980: f64;
            let v3981: f64;
            let v3982: f64;
            let v3983: Lanes<4>;
            let v3984: f64;
            let v3985: f64;
            if v3972 != 0.0 {
                v3980 = v817;
                v3981 = v817;
                v3982 = v817;
                v3983 = v2544;
                v3984 = v110;
                v3985 = v110;
            } else {
                let v3973 = v67 / v149;
                let v3977 = v3973 - v3829;
                let v3979 = (Lanes([((v494 - (v188 * v3973)) / v149), 0.0, 0.0, 0.0])) - v3830;
                v3980 = v3977;
                v3981 = v3968;
                v3982 = v3969;
                v3983 = v3979;
                v3984 = v3970;
                v3985 = v3971;
            }
            let v3986 = ddt(12367, v3863);
            let v3987 = v3869 * v3920;
            let v3988 = ddt(12370, v3865);
            let v3989 = v3871 * v3920;
            let v3998 = if ((((((v3917 + v3922) + v3924) + v3926) + v3956) + v3994) + v3996) != v817 { 1.0 } else { 0.0 };
            let v3999 = v3901[2];
            let v4003 = (-v3999) - (-(v3910[3]));
            let v4006 = if (v4003.abs()) > (ctx.simparam_or("gmin", v817)) { 1.0 } else { 0.0 };
            if v4006 != 0.0 {
            } else {
                let v4007 = if v4003 >= v817 { 1.0 } else { 0.0 };
            }
            let v4012 = (-(v3062[1])) - (-(v3910[1]));
            let v4015 = if (v4012.abs()) > (ctx.simparam_or("gmin", v817)) { 1.0 } else { 0.0 };
            if v4015 != 0.0 {
            } else {
                let v4016 = if v4012 >= v817 { 1.0 } else { 0.0 };
            }
            let v4017 = v3904[1];
            let v4020 = if (v4017.abs()) > (ctx.simparam_or("gmin", v817)) { 1.0 } else { 0.0 };
            if v4020 != 0.0 {
            } else {
                let v4021 = if v4017 >= v817 { 1.0 } else { 0.0 };
            }
            let v4022 = v3913[0];
            let v4023 = v3913[1];
            let v4024 = v3916[0];
            let v4025 = v3916[1];
            let v4026 = v3918[0];
            let v4027 = v3918[1];
            let v4028 = v3918[2];
            let v4029 = v3918[3];
            let v4030 = v3887[0];
            let v4031 = v3887[1];
            let v4032 = v3887[2];
            let v4033 = v3921[0];
            let v4034 = v3921[1];
            let v4035 = v3921[2];
            let v4036 = v3923[0];
            let v4037 = v3923[1];
            let v4038 = v3923[2];
            let v4039 = v3925[0];
            let v4040 = v3925[1];
            let v4041 = v3927[0];
            let v4042 = v3927[1];
            let v4043 = v3937[0];
            let v4044 = v3937[1];
            let v4045 = v3937[2];
            let v4046 = v3947[0];
            let v4047 = v3947[1];
            let v4048 = v3947[2];
            let v4049 = v3957[0];
            let v4050 = v3957[1];
            let v4051 = v3957[2];
            let v4052 = v3957[3];
            let v4053 = v3957[4];
            let v4054 = v3897[0];
            let v4055 = v3897[1];
            let v4056 = v3897[2];
            let v4057 = v3897[3];
            let v4058 = v3959[0];
            let v4059 = v3959[1];
            let v4060 = v3959[2];
            let v4061 = v3959[3];
            let v4062 = v3901[0];
            let v4063 = v3901[1];
            let v4064 = v3961[0];
            let v4065 = v3961[1];
            let v4066 = v3961[2];
            let v4067 = v3961[3];
            let v4068 = v3961[4];
            let v4069 = v3909[0];
            let v4070 = v3909[1];
            let v4071 = v3909[2];
            let v4072 = v3909[3];
            let v4073 = v3909[4];
            let v4074 = v3983[0];
            let v4075 = v3983[1];
            let v4076 = v3983[2];
            let v4077 = v3983[3];
            let v4078 = v3984;
            let v4079 = v3868[0];
            let v4080 = v3868[1];
            let v4081 = v3868[2];
            let v4082 = v3868[3];
            let v4083 = v3868[4];
            let v4084 = v3987;
            let v4085 = v3870[0];
            let v4086 = v3870[1];
            let v4087 = v3870[2];
            let v4088 = v3870[3];
            let v4089 = v3870[4];
            let v4090 = v3989;
            let v4091 = v3889[0];
            let v4092 = v3889[1];
            let v4093 = v3889[2];
            let v4094 = v3891[0];
            let v4095 = v3891[1];
            let v4096 = v3891[2];
            let v4097 = v3893[0];
            let v4098 = v3893[1];
            let v4099 = v3895[0];
            let v4100 = v3895[1];
            let v4101 = v3899[0];
            let v4102 = v3899[1];
            let v4103 = v3899[2];
            let v4104 = v3899[3];
            let v4105 = v3903[0];
            let v4106 = v3903[1];
            let v4107 = v3903[2];
            let v4108 = v3903[3];
            let v4109 = v3903[4];
            let v4110 = v3985;
            let v4111 = v3869;
            let v4112 = v3871;
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(6),
            Some(7),
            multiplicity * (v3912),
            [6, 7],
            [v4022, v4023],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(6),
            Some(5),
            multiplicity * (v3915),
            [5, 6],
            [v4024, v4025],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(3),
            multiplicity * (v3917),
            [1, 3, 4, 5],
            [v4026, v4027, v4028, v4029],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(3),
            Some(5),
            multiplicity * (v3886),
            [3, 4, 5],
            [v4030, v4031, v4032],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(3),
            Some(5),
            multiplicity * (v3919),
            [3, 4, 5],
            [v4033, v4034, v4035],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(5),
            multiplicity * (v3922),
            [1, 4, 5],
            [v4036, v4037, v4038],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(5),
            multiplicity * (v3924),
            [1, 5],
            [v4039, v4040],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(2),
            multiplicity * (v3926),
            [1, 2],
            [v4041, v4042],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(2),
            multiplicity * (v3936),
            [2, 4, 7],
            [v4043, v4044, v4045],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(7), Some(2), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            staged[89],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(0),
            multiplicity * (v3946),
            [0, 4, 5],
            [v4046, v4047, v4048],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(5), Some(0), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            staged[90],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(1),
            Some(6),
            multiplicity * (v3956),
            [1, 4, 5, 6, 7],
            [v4049, v4050, v4051, v4052, v4053],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(1), Some(6), 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            staged[91],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(5),
            multiplicity * (v3896),
            [4, 5, 6, 7],
            [v4054, v4055, v4056, v4057],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(5),
            multiplicity * (v3958),
            [4, 5, 6, 7],
            [v4058, v4059, v4060, v4061],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(6),
            Some(7),
            multiplicity * (v3900),
            [4, 6, 7],
            [v4062, v4063, v3999],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (v3960),
            [4, 5, 6, 7, 8],
            [v4064, v4065, v4066, v4067, v4068],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(7),
            multiplicity * (v3908),
            [4, 5, 6, 7, 9],
            [v4069, v4070, v4071, v4072, v4073],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(4), None, 3, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            3,
            staged[92],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            None,
            multiplicity * (v3980),
            [4, 5, 6, 7],
            [v4074, v4075, v4076, v4077],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (v3981),
            [4],
            [v4078],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            None,
            multiplicity * (v3862),
            [4, 5, 6, 7, 8],
            [v4079, v4080, v4081, v4082, v4083],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(8),
            None,
            multiplicity * (v3986),
            [8],
            [v4084],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            None,
            multiplicity * (v3864),
            [4, 5, 6, 7, 9],
            [v4085, v4086, v4087, v4088, v4089],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(9),
            None,
            multiplicity * (v3988),
            [9],
            [v4090],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(6),
            multiplicity * (staged[22]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(0),
            multiplicity * (staged[93]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(2),
            multiplicity * (staged[94]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(7),
            multiplicity * (v4113),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(7),
            multiplicity * (v4114),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(7),
            multiplicity * (v4115),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = v3912;
        self.canonical_reactive[1] = v3915;
        self.canonical_reactive[2] = v3917;
        self.canonical_reactive[3] = v3886;
        self.canonical_reactive[4] = v3888;
        self.canonical_reactive[5] = v4091;
        self.canonical_reactive[6] = v4092;
        self.canonical_reactive[7] = v4093;
        self.canonical_reactive[8] = v3890;
        self.canonical_reactive[9] = v4094;
        self.canonical_reactive[10] = v4095;
        self.canonical_reactive[11] = v4096;
        self.canonical_reactive[12] = v3892;
        self.canonical_reactive[13] = v4097;
        self.canonical_reactive[14] = v4098;
        self.canonical_reactive[15] = v3894;
        self.canonical_reactive[16] = v4099;
        self.canonical_reactive[17] = v4100;
        self.canonical_reactive[18] = v3936;
        self.canonical_reactive[19] = staged[89];
        self.canonical_reactive[20] = v3946;
        self.canonical_reactive[21] = staged[90];
        self.canonical_reactive[22] = v3956;
        self.canonical_reactive[23] = staged[91];
        self.canonical_reactive[24] = v3896;
        self.canonical_reactive[25] = v3898;
        self.canonical_reactive[26] = v4101;
        self.canonical_reactive[27] = v4102;
        self.canonical_reactive[28] = v4103;
        self.canonical_reactive[29] = v4104;
        self.canonical_reactive[30] = v3900;
        self.canonical_reactive[31] = v3902;
        self.canonical_reactive[32] = v4105;
        self.canonical_reactive[33] = v4106;
        self.canonical_reactive[34] = v4107;
        self.canonical_reactive[35] = v4108;
        self.canonical_reactive[36] = v4109;
        self.canonical_reactive[37] = v3908;
        self.canonical_reactive[38] = staged[92];
        self.canonical_reactive[39] = v3980;
        self.canonical_reactive[40] = v3982;
        self.canonical_reactive[41] = v4110;
        self.canonical_reactive[42] = v3862;
        self.canonical_reactive[43] = v3863;
        self.canonical_reactive[44] = v4111;
        self.canonical_reactive[45] = v3864;
        self.canonical_reactive[46] = v3865;
        self.canonical_reactive[47] = v4112;
        self.canonical_reactive[48] = staged[22];
        self.canonical_reactive[49] = staged[93];
        self.canonical_reactive[50] = staged[94];
        self.canonical_reactive[51] = v4113;
        self.canonical_reactive[52] = v4114;
        self.canonical_reactive[53] = v4115;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            Some(5),
            &[3, 4, 5],
            &[cached[5], cached[6], cached[7]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(5),
            &[1, 4, 5],
            &[cached[9], cached[10], cached[11]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(5),
            &[1, 5],
            &[cached[13], cached[14]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(2),
            &[1, 2],
            &[cached[16], cached[17]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(5),
            &[4, 5, 6, 7],
            &[cached[26], cached[27], cached[28], cached[29]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(7),
            &[4, 5, 6, 7, 8],
            &[cached[32], cached[33], cached[34], cached[35], cached[36]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            None,
            &[4],
            &[cached[41]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            None,
            &[8],
            &[cached[44]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(9),
            None,
            &[9],
            &[cached[47]],
            &[],
            &[],
            multiplicity,
        );
    }

}

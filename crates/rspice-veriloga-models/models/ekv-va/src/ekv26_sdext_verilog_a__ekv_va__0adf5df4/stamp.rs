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
        let mut key = Vec::with_capacity(150);
        for index in 0..Self::PARAMETER_COUNT {
            if PARAMETER_MODEL_FLAGS[index] {
                key.push(self.params.values[index].to_bits());
                key.push(u64::from(self.param_given[index]));
            }
        }
        key.into_boxed_slice()
    }

    fn canonical_install_model_values(&mut self, values: Arc<CanonicalModelValues>) {
        self.canonical_staged[2] = values[0];
        self.canonical_staged[25] = values[1];
        self.canonical_staged[40] = values[2];
        self.canonical_staged[33] = values[3];
        self.canonical_staged[69] = values[4];
        self.canonical_staged[32] = values[5];
        self.canonical_staged[70] = values[6];
        self.canonical_staged[71] = values[7];
        self.canonical_staged[72] = values[8];
        self.canonical_staged[0] = values[9];
        self.canonical_staged[1] = values[10];
        self.canonical_staged[3] = values[11];
        self.canonical_staged[7] = values[12];
        self.canonical_staged[73] = values[13];
        self.canonical_staged[4] = values[14];
        self.canonical_staged[74] = values[15];
        self.canonical_staged[5] = values[16];
        self.canonical_staged[75] = values[17];
        self.canonical_staged[6] = values[18];
        self.canonical_staged[76] = values[19];
        self.canonical_staged[8] = values[20];
        self.canonical_staged[77] = values[21];
        self.canonical_staged[10] = values[22];
        self.canonical_staged[19] = values[23];
        self.canonical_staged[18] = values[24];
        self.canonical_staged[15] = values[25];
        self.canonical_staged[16] = values[26];
        self.canonical_staged[27] = values[27];
        self.canonical_staged[29] = values[28];
        self.canonical_staged[78] = values[29];
        self.canonical_staged[36] = values[30];
        self.canonical_staged[38] = values[31];
        self.canonical_staged[39] = values[32];
        self.canonical_staged[44] = values[33];
        self.canonical_staged[79] = values[34];
        self.canonical_staged[80] = values[35];
        self.canonical_staged[81] = values[36];
        self.canonical_staged[82] = values[37];
        self.canonical_staged[45] = values[38];
        self.canonical_staged[46] = values[39];
        self.canonical_staged[47] = values[40];
        self.canonical_staged[50] = values[41];
        self.canonical_staged[58] = values[42];
        self.canonical_staged[59] = values[43];
        self.canonical_staged[83] = values[44];
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
                let v0 = 1.0359399871014713e-10f64;
                let v1 = parameters[13];
                let v3 = parameters[14];
                let v6 = parameters[25];
                let v8 = 3e0f64;
                let v10 = parameters[28];
                let v12 = parameters[29];
                let v14 = parameters[35];
                let v16 = parameters[22];
                let v19 = parameters[30];
                let v22 = parameters[0];
                let v23 = 0e0f64;
                let v25 = 5e-1f64;
                let v26 = 3.333333333333e-1f64;
                let v28 = parameters[3];
                let v29 = 1e21f64;
                let v31 = 2.7315e2f64;
                let v33 = parameters[4];
                let v34 = 1e21f64;
                let v36 = 2.9815e2f64;
                let v39 = 7.02e-4f64;
                let v42 = 1.108e3f64;
                let v45 = 1.16e0f64;
                let v47 = parameters[5];
                let v48 = parameters[26];
                let v50 = parameters[6];
                let v51 = parameters[27];
                let v55 = 1e0f64;
                let v57 = parameters[38];
                let v58 = 1e-6f64;
                let v61 = parameters[39];
                let v70 = parameters[40];
                let v74 = parameters[17];
                let v78 = parameters[31];
                let v79 = parameters[8];
                let v82 = 1e-1f64;
                let v84 = 2.8e-1f64;
                let v87 = 1.936e-3f64;
                let v97 = parameters[7];
                let v102 = 2.5e-1f64;
                let v108 = -5e-1f64;
                let v113 = parameters[36];
                let v114 = parameters[37];
                let v119 = parameters[1];
                let v120 = 0e0f64;
                let v122 = parameters[9];
                let v126 = 2e0f64;
                let v130 = parameters[11];
                let v133 = 4e0f64;
                let v137 = parameters[10];
                let v143 = parameters[12];
                let v149 = 8.617333262e-5f64;
                let mut out32: f64 = 0.0;
                let mut out59: f64 = 0.0;
                let mut out60: f64 = 0.0;
                let mut out64: f64 = 0.0;
                let mut out66: f64 = 0.0;
                let mut out69: f64 = 0.0;
                let v2 = v0 / v1;
                let v5 = (v2 * v3).sqrt();
                let v7 = v5 * v6;
                let v11 = (v8 * v2) * v10;
                let v13 = v2 * v12;
                let v15 = v14 + v14;
                let v18 = v1 / (v0 * v16);
                let v21 = (v19 + v19) / v1;
                let v24 = if v22 > v23 { 1.0 } else { 0.0 };
                let v27: f64;
                if v24 != 0.0 {
                    v27 = v25;
                } else {
                    v27 = v26;
                }
                let v30 = if v28 == v29 { 1.0 } else { 0.0 };
                if v30 != 0.0 {
                } else {
                    let v32 = v28 + v31;
                    out32 = v32;
                }
                let v35 = if v33 == v34 { 1.0 } else { 0.0 };
                let v38: f64;
                if v35 != 0.0 {
                    v38 = v36;
                } else {
                    let v37 = v33 + v31;
                    v38 = v37;
                }
                let v46 = v45 - (((v39 * v38) * v38) / (v38 + v42));
                let v49 = v47 + v48;
                let v52 = v50 + v51;
                let v53 = v52 * v49;
                let v56 = v55 / (v53.sqrt());
                if v24 != 0.0 {
                    let v59 = if v57 != v58 { 1.0 } else { 0.0 };
                    out59 = v59;
                    if v59 != 0.0 {
                        let v64 = v56 * (v57 - v58);
                        out64 = v64;
                    } else {
                    }
                } else {
                    let v60 = if v57 != v58 { 1.0 } else { 0.0 };
                    out60 = v60;
                    if v60 != 0.0 {
                        let v66 = v56 * (v58 - v57);
                        out66 = v66;
                    } else {
                    }
                }
                let v62 = if v61 != v58 { 1.0 } else { 0.0 };
                if v62 != 0.0 {
                    let v69 = v55 + ((v61 - v58) * v56);
                    out69 = v69;
                } else {
                }
                let v71 = if v70 != v58 { 1.0 } else { 0.0 };
                let v76: f64;
                if v71 != 0.0 {
                    let v75 = v74 + ((v70 - v58) * v56);
                    v76 = v75;
                } else {
                    v76 = v74;
                }
                let v77 = if v21 == v23 { 1.0 } else { 0.0 };
                let v96: f64;
                if v77 != 0.0 {
                    v96 = v23;
                } else {
                    let v85 = v84 * ((v49 / (v78 * v79)) - v82);
                    let v93 = v55 / (v55 + (v25 * (v85 + (((v85 * v85) + v87).sqrt()))));
                    let v95 = (v21 * v93) * v93;
                    v96 = v95;
                }
                let v99 = (v11 * v97) / v52;
                let v101 = (v13 * v79) / v49;
                let v104 = (v102 * v76) * v76;
                let v105 = v25 * v76;
                let v106 = v82 * v49;
                let v107 = v106 * v106;
                let v109 = v108 * v76;
                let v110 = if v16 == v23 { 1.0 } else { 0.0 };
                let v111 = -v101;
                let v112 = -v76;
                let v117 = (v113 * v114) / (v52 - v51);
                let v118 = v53 * v1;
                let v121: f64;
                if v119 != 0.0 {
                    v121 = v120;
                } else {
                    v121 = v23;
                }
                let v124 = if v114 > v23 { 1.0 } else { 0.0 };
                let v125 = if (if v122 == v23 { 1.0 } else { 0.0 }) != 0.0 && v124 != 0.0 { 1.0 } else { 0.0 };
                let v129: f64;
                if v125 != 0.0 {
                    let v128 = (v126 * v114) * v52;
                    v129 = v128;
                } else {
                    v129 = v122;
                }
                let v132 = if (if v130 == v23 { 1.0 } else { 0.0 }) != 0.0 && v124 != 0.0 { 1.0 } else { 0.0 };
                let v136: f64;
                if v132 != 0.0 {
                    let v135 = (v133 * v114) + v52;
                    v136 = v135;
                } else {
                    v136 = v130;
                }
                let v139 = if (if v137 == v23 { 1.0 } else { 0.0 }) != 0.0 && v124 != 0.0 { 1.0 } else { 0.0 };
                let v142: f64;
                if v139 != 0.0 {
                    let v141 = (v126 * v114) * v52;
                    v142 = v141;
                } else {
                    v142 = v137;
                }
                let v145 = if (if v143 == v23 { 1.0 } else { 0.0 }) != 0.0 && v124 != 0.0 { 1.0 } else { 0.0 };
                let v148: f64;
                if v145 != 0.0 {
                    let v147 = (v133 * v114) + v52;
                    v148 = v147;
                } else {
                    v148 = v143;
                }
                let v151 = v46 / (v38 * v149);
                let v152 = -v52;
            [v5, v7, v15, v18, v24, v27, v30, out32, v35, v38, v46, v49, v52, out59, out64, out60, out66, v62, out69, v71, v76, v77, v96, v99, v101, v104, v105, v107, v109, v110, v111, v112, v117, v118, v125, v132, v139, v145, v151, v142, v148, v152, v129, v136, v121]
        };
        let values = canonical_model_cache_intern(key, Arc::new(produced));
        self.canonical_install_model_values(values);
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
        let produced: [f64; 39] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let temperature = ctx.temperature();
            let staged = &*self.canonical_staged;
                let v0 = staged[69];
                let v1 = staged[70];
                let v2 = temperature;
                let v3 = parameters[2];
                let v5 = staged[71];
                let v7 = 8.617333262e-5f64;
                let v9 = 1e-1f64;
                let v11 = 1e0f64;
                let v17 = 1.6e1f64;
                let v19 = 7.02e-4f64;
                let v22 = 1.108e3f64;
                let v25 = 1.16e0f64;
                let v27 = staged[0];
                let v30 = parameters[16];
                let v32 = parameters[15];
                let v34 = parameters[20];
                let v36 = parameters[19];
                let v38 = parameters[24];
                let v40 = parameters[23];
                let v42 = parameters[34];
                let v45 = parameters[33];
                let v47 = parameters[18];
                let v49 = 3e0f64;
                let v54 = staged[1];
                let v58 = 2e-1f64;
                let v64 = 5e-1f64;
                let v69 = staged[2];
                let v72 = parameters[32];
                let v74 = staged[3];
                let v79 = 6e-1f64;
                let v82 = staged[73];
                let v83 = staged[74];
                let v85 = staged[75];
                let v86 = staged[4];
                let v89 = staged[5];
                let v93 = staged[6];
                let v96 = staged[7];
                let v98 = staged[8];
                let v100 = 2e0f64;
                let v103 = staged[78];
                let v105 = parameters[25];
                let v107 = staged[33];
                let v111 = 0e0f64;
                let v114 = staged[45];
                let v116 = parameters[65];
                let v119 = parameters[43];
                let v122 = parameters[44];
                let v124 = parameters[45];
                let v126 = parameters[46];
                let v128 = parameters[69];
                let v130 = parameters[50];
                let v132 = parameters[70];
                let v134 = parameters[51];
                let v136 = parameters[71];
                let v138 = parameters[52];
                let v140 = parameters[66];
                let v143 = parameters[53];
                let v145 = parameters[67];
                let v148 = parameters[54];
                let v150 = parameters[68];
                let v153 = parameters[55];
                let v156 = parameters[72];
                let v159 = parameters[59];
                let v161 = parameters[73];
                let v164 = parameters[60];
                let v166 = parameters[74];
                let v169 = parameters[61];
                let v171 = staged[46];
                let v173 = staged[47];
                let v179 = staged[50];
                let v184 = staged[58];
                let v186 = staged[59];
                let mut out110: f64 = 0.0;
                let v6: f64;
                if v1 != 0.0 {
                    let v4 = v2 + v3;
                    v6 = v4;
                } else {
                    v6 = v5;
                }
                let v8 = v6 * v7;
                let v10 = v9 * v8;
                let v12 = v11 / v8;
                let v13 = v8 + v8;
                let v14 = v13 + v13;
                let v15 = v8 * v8;
                let v16 = v15 + v15;
                let v18 = v17 * v15;
                let v26 = v25 - (((v19 * v6) * v6) / (v6 + v22));
                let v28 = v6 - v27;
                let v29 = v6 / v27;
                let v33 = v32 - (v30 * v28);
                let v37 = v36 * (v29.powf(v34));
                let v41 = v40 * (v29.powf(v38));
                let v46 = v45 * (v11 + (v42 * v28));
                let v51 = v29.ln();
                let v59 = ((((v47 * v29) - ((v49 * v8) * v51)) - (v54 * v29)) + v26) - v58;
                let v66 = (v64 * (v59 + (((v59 * v59) + v15).sqrt()))) + v58;
                let v67 = v66.sqrt();
                let v68 = v11 / v41;
                let v70 = v69 * v41;
                let v71 = v69 * v46;
                let v73 = v72 / v46;
                let v75 = v41 * v74;
                let v81 = v8 * ((((v64 * v75) * v12).ln()) - v79);
                let v84: f64;
                if v0 != 0.0 {
                    let v88: f64;
                    if v82 != 0.0 {
                        let v87 = v86 + v33;
                        v88 = v87;
                    } else {
                        v88 = v33;
                    }
                    v84 = v88;
                } else {
                    let v92: f64;
                    if v83 != 0.0 {
                        let v90 = v89 - v33;
                        v92 = v90;
                    } else {
                        let v91 = -v33;
                        v92 = v91;
                    }
                    v84 = v92;
                }
                let v95: f64;
                if v85 != 0.0 {
                    let v94 = v37 * v93;
                    v95 = v94;
                } else {
                    v95 = v37;
                }
                let v97 = v96 * v95;
                let v99 = v98 * v67;
                let v101 = v100 * v18;
                let v102 = v8 / v75;
                if v103 != 0.0 {
                } else {
                    let v110 = v97 * (v11 + (v107 * v99));
                    out110 = v110;
                }
                let v106 = (v14 + v14) * v105;
                let v112 = if v73 > v111 { 1.0 } else { 0.0 };
                let v121 = (((v114 - (v26 / v8)) + (v116 * v51)) / v119).exp();
                let v123 = v122 * v121;
                let v125 = v124 * v121;
                let v127 = v126 * v121;
                let v131 = v130 - (v128 * v28);
                let v135 = v134 - (v132 * v28);
                let v139 = v138 - (v136 * v28);
                let v144 = v143 * (v11 + (v140 * v28));
                let v149 = v148 * (v11 + (v145 * v28));
                let v154 = v153 * (v11 + (v150 * v28));
                let v155 = v29 - v11;
                let v160 = v159 * (v11 + (v155 * v156));
                let v165 = v164 * (v11 + (v155 * v161));
                let v170 = v169 * (v11 + (v155 * v166));
                let v172 = v123 * v171;
                let v174 = v125 * v173;
                let v176 = v127 * v96;
                let v177 = (v172 + v174) + v176;
                let v178 = v8 * v119;
                let v180 = v179 * v127;
                let v181 = v8 * v170;
                let v182 = v8 * v165;
                let v183 = v8 * v160;
                let v185 = v123 * v184;
                let v187 = v125 * v186;
                let v189 = (v185 + v187) + v176;
            [v8, v10, v12, v14, v16, v18, v29, v66, v68, v70, v71, v73, v75, v81, v97, v99, v84, v101, v102, out110, v106, v112, v131, v135, v139, v144, v149, v154, v172, v174, v177, v178, v180, v181, v182, v183, v185, v187, v189]
        };
        self.canonical_staged[28] = produced[0];
        self.canonical_staged[17] = produced[1];
        self.canonical_staged[20] = produced[2];
        self.canonical_staged[35] = produced[3];
        self.canonical_staged[30] = produced[4];
        self.canonical_staged[14] = produced[5];
        self.canonical_staged[48] = produced[6];
        self.canonical_staged[11] = produced[7];
        self.canonical_staged[26] = produced[8];
        self.canonical_staged[24] = produced[9];
        self.canonical_staged[42] = produced[10];
        self.canonical_staged[43] = produced[11];
        self.canonical_staged[22] = produced[12];
        self.canonical_staged[23] = produced[13];
        self.canonical_staged[31] = produced[14];
        self.canonical_staged[12] = produced[15];
        self.canonical_staged[9] = produced[16];
        self.canonical_staged[13] = produced[17];
        self.canonical_staged[21] = produced[18];
        self.canonical_staged[34] = produced[19];
        self.canonical_staged[37] = produced[20];
        self.canonical_staged[41] = produced[21];
        self.canonical_staged[64] = produced[22];
        self.canonical_staged[66] = produced[23];
        self.canonical_staged[68] = produced[24];
        self.canonical_staged[63] = produced[25];
        self.canonical_staged[65] = produced[26];
        self.canonical_staged[67] = produced[27];
        self.canonical_staged[56] = produced[28];
        self.canonical_staged[54] = produced[29];
        self.canonical_staged[57] = produced[30];
        self.canonical_staged[49] = produced[31];
        self.canonical_staged[52] = produced[32];
        self.canonical_staged[51] = produced[33];
        self.canonical_staged[53] = produced[34];
        self.canonical_staged[55] = produced[35];
        self.canonical_staged[61] = produced[36];
        self.canonical_staged[60] = produced[37];
        self.canonical_staged[62] = produced[38];
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
        self.canonical_temperature_stage(ctx);
        self.canonical_timestep_stage(ctx);
        let parameters = &self.params.values;
        let multiplicity = self.multiplicity;
        let staged = &*self.canonical_staged;
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 4749 => 0usize, 4751 => 1usize, 4775 => 2usize, 5312 => 3usize, 5403 => 4usize, _ => usize::MAX };
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
            let v1 = node_potentials[3];
            let v3 = 1e0f64;
            let v5 = 1e0f64;
            let v8 = parameters[0];
            let v11 = node_potentials[2];
            let v13 = 1e0f64;
            let v19 = node_potentials[0];
            let v21 = 1e0f64;
            let v30 = 0e0f64;
            let v32 = -1e0f64;
            let v33 = 1e0f64;
            let v39 = staged[9];
            let v41 = staged[10];
            let v43 = staged[11];
            let v45 = staged[12];
            let v50 = staged[13];
            let v53 = 2e0f64;
            let v55 = 1e0f64;
            let v60 = 5e-1f64;
            let v67 = staged[14];
            let v98 = staged[15];
            let v105 = staged[16];
            let v107 = staged[8];
            let v113 = staged[17];
            let v121 = staged[18];
            let v125 = -1e0f64;
            let v127 = staged[19];
            let v146 = 2.5e-1f64;
            let v173 = staged[20];
            let v176 = -3.5e-1f64;
            let v178 = 1.3e0f64;
            let v180 = 1.6e0f64;
            let v187 = 2e0f64;
            let v213 = -1.5e1f64;
            let v226 = staged[21];
            let v235 = staged[22];
            let v246 = parameters[25];
            let v249 = 1.5625e-2f64;
            let v279 = 7.5e-1f64;
            let v294 = staged[23];
            let v326 = -3.5e-1f64;
            let v332 = 1.55e0f64;
            let v355 = -2.3e1f64;
            let v370 = 1e-64f64;
            let v406 = -1.5e1f64;
            let v417 = staged[24];
            let v424 = staged[25];
            let v427 = staged[3];
            let v432 = staged[26];
            let v440 = staged[27];
            let v455 = -3.5e-1f64;
            let v483 = -2.3e1f64;
            let v533 = -1.5e1f64;
            let v558 = 1e-6f64;
            let v578 = staged[28];
            let v589 = 1.33333332e0f64;
            let v601 = staged[29];
            let v610 = staged[78];
            let v637 = -2.3e1f64;
            let v658 = staged[30];
            let v668 = parameters[21];
            let v676 = staged[31];
            let v681 = 0e0f64;
            let v682 = Lanes([0e0f64; 4]);
            let v683 = staged[32];
            let v699 = staged[35];
            let v730 = staged[36];
            let v778 = 4e0f64;
            let v809 = staged[37];
            let v862 = -5e-1f64;
            let v877 = 1.5e0f64;
            let v953 = -5e-1f64;
            let v984 = -5e-1f64;
            let v1008 = -5e-1f64;
            let v1029 = 6.6666666e-1f64;
            let v1134 = staged[33];
            let v1148 = staged[34];
            let v1221 = staged[38];
            let v1264 = staged[39];
            let v1280 = staged[40];
            let v1287 = staged[41];
            let v1293 = staged[42];
            let v1297 = -3.5e1f64;
            let v1325 = staged[44];
            let v1330 = 3e0f64;
            let v1333 = 6e0f64;
            let v1354 = 2.66666666e-1f64;
            let v1401 = -5e-1f64;
            let v1435 = ddt_scale();
            let v1440 = -3.5e1f64;
            let v1445 = staged[43];
            let v1504 = staged[48];
            let v1507 = staged[49];
            let v1510 = -4e1f64;
            let v1512 = -4e1f64;
            let v1513 = Lanes([0e0f64; 2]);
            let v1516 = parameters[58];
            let v1520 = 7e1f64;
            let v1526 = parameters[57];
            let v1534 = staged[51];
            let v1537 = parameters[64];
            let v1541 = 1e-3f64;
            let v1552 = staged[52];
            let v1555 = staged[53];
            let v1558 = parameters[63];
            let v1572 = staged[54];
            let v1577 = staged[55];
            let v1580 = parameters[62];
            let v1594 = staged[56];
            let v1603 = staged[57];
            let v1610 = parameters[56];
            let v1619 = parameters[7];
            let v1628 = -4e1f64;
            let v1630 = -4e1f64;
            let v1631 = Lanes([0e0f64; 2]);
            let v1681 = staged[60];
            let v1701 = staged[61];
            let v1710 = staged[62];
            let v1728 = staged[63];
            let v1729 = staged[46];
            let v1731 = parameters[47];
            let v1733 = staged[64];
            let v1746 = staged[65];
            let v1747 = staged[47];
            let v1749 = parameters[48];
            let v1751 = staged[66];
            let v1764 = staged[67];
            let v1765 = staged[7];
            let v1767 = parameters[49];
            let v1769 = staged[68];
            let v1834 = staged[58];
            let v1849 = staged[59];
            let v9 = v8 * (v0 - v1);
            let v10 = ((Lanes([v3, 0.0])) - (Lanes([0.0, v5]))) * v8;
            let v17 = v8 * (v11 - v1);
            let v18 = ((Lanes([v13, 0.0])) - (Lanes([0.0, v5]))) * v8;
            let v25 = v8 * (v19 - v1);
            let v26 = ((Lanes([v21, 0.0])) - (Lanes([0.0, v5]))) * v8;
            let v28 = Lanes([v26[0], 0.0, v26[1]]);
            let v29 = Lanes([0.0, v18[0], v18[1]]);
            let v31 = if (v25 - v17) < v30 { 1.0 } else { 0.0 };
            let v34: f64;
            let v35: f64;
            let v36: f64;
            let v37: Lanes<3>;
            let v38: Lanes<3>;
            if v31 != 0.0 {
                v34 = v25;
                v35 = v17;
                v36 = v32;
                v37 = v28;
                v38 = v29;
            } else {
                v34 = v17;
                v35 = v25;
                v36 = v33;
                v37 = v29;
                v38 = v28;
            }
            let v46 = (((v9 - v39) - v41) + v43) + v45;
            let v48 = v10 * v46;
            let v52 = ((v46 * v46) + v50).sqrt();
            let v61 = v60 * (v46 + v52);
            let v62 = (v10 + ((v48 + v48) * (v55 / (v53 * v52)))) * v60;
            let v63 = v43 + v34;
            let v65 = v37 * v63;
            let v69 = ((v63 * v63) + v67).sqrt();
            let v72 = (v65 + v65) * (v55 / (v53 * v69));
            let v77 = (v60 * (v63 + v69)).sqrt();
            let v80 = ((v37 + v72) * v60) * (v55 / (v53 * v77));
            let v81 = v43 + v35;
            let v83 = v38 * v81;
            let v86 = ((v81 * v81) + v67).sqrt();
            let v89 = (v83 + v83) * (v55 / (v53 * v86));
            let v94 = (v60 * (v81 + v86)).sqrt();
            let v97 = ((v38 + v89) * v60) * (v55 / (v53 * v94));
            let v100 = (v61 + v98).sqrt();
            let v104 = v61 - v43;
            let v115 = (((v104 - (v107 * (v100 - v105))) + v43) + v113).sqrt();
            let v126 = ((v80 + v97) * v121) * v125;
            let v129 = ((v62 - ((v62 * (v55 / (v53 * v100))) * v107)) * (v55 / (v53 * v115))) * v127;
            let v130 = (v107 - (v121 * (v77 + v94))) + (v127 * v115);
            let v133 = (Lanes([v126[0], 0.0, v126[1], v126[2]])) + (Lanes([0.0, v129[0], 0.0, v129[1]]));
            let v135 = v133 * v130;
            let v138 = ((v130 * v130) + v113).sqrt();
            let v141 = (v135 + v135) * (v55 / (v53 * v138));
            let v144 = v60 * (v130 + v138);
            let v145 = (v133 + v141) * v60;
            let v147 = v146 * v144;
            let v154 = Lanes([0.0, v62[0], 0.0, v62[1]]);
            let v156 = (v61 + (v147 * v144)).sqrt();
            let v159 = (v154 + (((v145 * v146) * v144) + (v145 * v147))) * (v55 / (v53 * v156));
            let v162 = v156 - (v60 * v144);
            let v168 = v104 - (v144 * v162);
            let v169 = v154 - ((v145 * v162) + ((v159 - (v145 * v60)) * v144));
            let v171 = Lanes([v37[0], 0.0, v37[1], v37[2]]);
            let v174 = (v168 - v34) * v173;
            let v175 = (v169 - v171) * v173;
            let v177 = if v174 > v176 { 1.0 } else { 0.0 };
            let v215: f64;
            let v216: Lanes<4>;
            if v177 != 0.0 {
                let v181 = v174 + v180;
                let v185 = (v178 + v174) - (v181.ln());
                let v188 = v187 / v185;
                let v191 = (((v175 - (v175 * (v55 / v181))) * v188) * v125) / v185;
                let v193 = v33 + v174;
                let v197 = v193 + (v188.ln());
                let v199 = (v187 + v188) / v197;
                let v202 = (v191 - ((v175 + (v191 * (v55 / v188))) * v199)) / v197;
                let v208 = v187 + v199;
                let v209 = (v193 + (v199.ln())) / v208;
                let v212 = ((v175 + (v202 * (v55 / v199))) - (v202 * v209)) / v208;
                v215 = v209;
                v216 = v212;
            } else {
                let v214 = if v174 > v213 { 1.0 } else { 0.0 };
                let v357: f64;
                let v358: Lanes<4>;
                if v214 != 0.0 {
                    let v330 = (-v174).exp();
                    let v331 = (v175 * v125) * v330;
                    let v333 = v332 + v330;
                    let v335 = v33 + v174;
                    let v339 = v335 + (v333.ln());
                    let v341 = (v187 + v333) / v339;
                    let v344 = (v331 - ((v175 + (v331 * (v55 / v333))) * v341)) / v339;
                    let v350 = v187 + v341;
                    let v351 = (v335 + (v341.ln())) / v350;
                    let v354 = ((v175 + (v344 * (v55 / v341))) - (v344 * v351)) / v350;
                    v357 = v351;
                    v358 = v354;
                } else {
                    let v356 = if v174 > v355 { 1.0 } else { 0.0 };
                    let v372: f64;
                    let v373: Lanes<4>;
                    if v356 != 0.0 {
                        let v361 = (-v174).exp();
                        let v363 = v187 + v361;
                        let v364 = v33 / v363;
                        let v367 = ((((v175 * v125) * v361) * v364) * v125) / v363;
                        v372 = v364;
                        v373 = v367;
                    } else {
                        let v368 = v174.exp();
                        let v369 = v175 * v368;
                        let v371 = v368 + v370;
                        v372 = v371;
                        v373 = v369;
                    }
                    v357 = v372;
                    v358 = v373;
                }
                v215 = v357;
                v216 = v358;
            }
            let v217 = v33 + v215;
            let v218 = v215 * v217;
            let v221 = (v216 * v217) + (v216 * v215);
            let v222 = v218.sqrt();
            let v225 = v221 * (v55 / (v53 * v222));
            let v230 = (v146 + (v222 * v226)).sqrt();
            let v233 = (v225 * v226) * (v55 / (v53 * v230));
            let v236 = v235 * (v230 - v60);
            let v237 = v233 * v235;
            let v238 = v35 - v34;
            let v239 = v38 - v37;
            let v240 = v60 * v238;
            let v241 = v239 * v60;
            let v251 = v67 * ((v246 * (v222 - (v236 * v173))) + v249);
            let v252 = ((v225 - (v237 * v173)) * v246) * v67;
            let v254 = v237 * v236;
            let v258 = ((v236 * v236) + v251).sqrt();
            let v261 = ((v254 + v254) + v252) * (v55 / (v53 * v258));
            let v262 = v240 - v236;
            let v263 = Lanes([v241[0], 0.0, v241[1], v241[2]]);
            let v264 = v263 - v237;
            let v266 = v264 * v262;
            let v270 = ((v262 * v262) + v251).sqrt();
            let v273 = ((v266 + v266) + v252) * (v55 / (v53 * v270));
            let v274 = v258 - v270;
            let v275 = v261 - v273;
            let v287 = (v146 + ((v222 - (v279 * (v218.ln()))) * v226)).sqrt();
            let v290 = ((v225 - ((v221 * (v55 / v218)) * v279)) * v226) * (v55 / (v53 * v287));
            let v293 = v290 * v235;
            let v295 = (v235 * (v287 - v60)) + v294;
            let v296 = v240 - v295;
            let v297 = v263 - v293;
            let v299 = v293 * v295;
            let v303 = ((v295 * v295) + v251).sqrt();
            let v306 = ((v299 + v299) + v252) * (v55 / (v53 * v303));
            let v308 = v297 * v296;
            let v312 = ((v296 * v296) + v251).sqrt();
            let v315 = ((v308 + v308) + v252) * (v55 / (v53 * v312));
            let v324 = ((((v168 - v240) - v34) - v303) + v312) * v173;
            let v325 = ((((v169 - v263) - v171) - v306) + v315) * v173;
            let v327 = if v324 > v326 { 1.0 } else { 0.0 };
            let v408: f64;
            let v409: Lanes<4>;
            if v327 != 0.0 {
                let v375 = v324 + v180;
                let v379 = (v178 + v324) - (v375.ln());
                let v381 = v187 / v379;
                let v384 = (((v325 - (v325 * (v55 / v375))) * v381) * v125) / v379;
                let v386 = v33 + v324;
                let v390 = v386 + (v381.ln());
                let v392 = (v187 + v381) / v390;
                let v395 = (v384 - ((v325 + (v384 * (v55 / v381))) * v392)) / v390;
                let v401 = v187 + v392;
                let v402 = (v386 + (v392.ln())) / v401;
                let v405 = ((v325 + (v395 * (v55 / v392))) - (v395 * v402)) / v401;
                v408 = v402;
                v409 = v405;
            } else {
                let v407 = if v324 > v406 { 1.0 } else { 0.0 };
                let v485: f64;
                let v486: Lanes<4>;
                if v407 != 0.0 {
                    let v459 = (-v324).exp();
                    let v460 = (v325 * v125) * v459;
                    let v461 = v332 + v459;
                    let v463 = v33 + v324;
                    let v467 = v463 + (v461.ln());
                    let v469 = (v187 + v461) / v467;
                    let v472 = (v460 - ((v325 + (v460 * (v55 / v461))) * v469)) / v467;
                    let v478 = v187 + v469;
                    let v479 = (v463 + (v469.ln())) / v478;
                    let v482 = ((v325 + (v472 * (v55 / v469))) - (v472 * v479)) / v478;
                    v485 = v479;
                    v486 = v482;
                } else {
                    let v484 = if v324 > v483 { 1.0 } else { 0.0 };
                    let v499: f64;
                    let v500: Lanes<4>;
                    if v484 != 0.0 {
                        let v489 = (-v324).exp();
                        let v491 = v187 + v489;
                        let v492 = v33 / v491;
                        let v495 = ((((v325 * v125) * v489) * v492) * v125) / v491;
                        v499 = v492;
                        v500 = v495;
                    } else {
                        let v496 = v324.exp();
                        let v497 = v325 * v496;
                        let v498 = v496 + v370;
                        v499 = v498;
                        v500 = v497;
                    }
                    v485 = v499;
                    v486 = v500;
                }
                v408 = v485;
                v409 = v486;
            }
            let v410 = v33 + v408;
            let v411 = v408 * v410;
            let v414 = (v409 * v410) + (v409 * v408);
            let v416 = v263 - v275;
            let v420 = v33 + ((v240 - v274) / v417);
            let v435 = (v427 - (v424 * (v420.ln()))) + ((v240 + v274) * v432);
            let v436 = ((((v416 / v417) * (v55 / v420)) * v424) * v125) + ((v263 + v275) * v432);
            let v438 = v436 * v435;
            let v442 = ((v435 * v435) + v440).sqrt();
            let v445 = (v438 + v438) * (v55 / (v53 * v442));
            let v448 = v60 * (v435 + v442);
            let v449 = (v436 + v445) * v60;
            let v453 = (v168 - v35) * v173;
            let v454 = (v169 - (Lanes([v38[0], 0.0, v38[1], v38[2]]))) * v173;
            let v456 = if v453 > v455 { 1.0 } else { 0.0 };
            let v535: f64;
            let v536: Lanes<4>;
            if v456 != 0.0 {
                let v502 = v453 + v180;
                let v506 = (v178 + v453) - (v502.ln());
                let v508 = v187 / v506;
                let v511 = (((v454 - (v454 * (v55 / v502))) * v508) * v125) / v506;
                let v513 = v33 + v453;
                let v517 = v513 + (v508.ln());
                let v519 = (v187 + v508) / v517;
                let v522 = (v511 - ((v454 + (v511 * (v55 / v508))) * v519)) / v517;
                let v528 = v187 + v519;
                let v529 = (v513 + (v519.ln())) / v528;
                let v532 = ((v454 + (v522 * (v55 / v519))) - (v522 * v529)) / v528;
                v535 = v529;
                v536 = v532;
            } else {
                let v534 = if v453 > v533 { 1.0 } else { 0.0 };
                let v639: f64;
                let v640: Lanes<4>;
                if v534 != 0.0 {
                    let v613 = (-v453).exp();
                    let v614 = (v454 * v125) * v613;
                    let v615 = v332 + v613;
                    let v617 = v33 + v453;
                    let v621 = v617 + (v615.ln());
                    let v623 = (v187 + v615) / v621;
                    let v626 = (v614 - ((v454 + (v614 * (v55 / v615))) * v623)) / v621;
                    let v632 = v187 + v623;
                    let v633 = (v617 + (v623.ln())) / v632;
                    let v636 = ((v454 + (v626 * (v55 / v623))) - (v626 * v633)) / v632;
                    v639 = v633;
                    v640 = v636;
                } else {
                    let v638 = if v453 > v637 { 1.0 } else { 0.0 };
                    let v653: f64;
                    let v654: Lanes<4>;
                    if v638 != 0.0 {
                        let v643 = (-v453).exp();
                        let v645 = v187 + v643;
                        let v646 = v33 / v645;
                        let v649 = ((((v454 * v125) * v643) * v646) * v125) / v645;
                        v653 = v646;
                        v654 = v649;
                    } else {
                        let v650 = v453.exp();
                        let v651 = v454 * v650;
                        let v652 = v650 + v370;
                        v653 = v652;
                        v654 = v651;
                    }
                    v639 = v653;
                    v640 = v654;
                }
                v535 = v639;
                v536 = v640;
            }
            let v537 = v33 + v535;
            let v541 = (v536 * v537) + (v536 * v535);
            let v542 = v146 + v218;
            let v543 = v146 + (v535 * v537);
            let v544 = v542.sqrt();
            let v547 = v221 * (v55 / (v53 * v544));
            let v548 = v543.sqrt();
            let v551 = v541 * (v55 / (v53 * v548));
            let v552 = v544 + v548;
            let v553 = v547 + v551;
            let v554 = v552 * v552;
            let v555 = v553 * v552;
            let v556 = v555 + v555;
            let v557 = v168 + v43;
            let v559 = v557 + v558;
            let v560 = v559.sqrt();
            let v564 = v187 * v560;
            let v565 = (v169 * (v55 / (v53 * v560))) * v187;
            let v566 = v107 / v564;
            let v569 = ((v565 * v566) * v125) / v564;
            let v570 = v564 + v107;
            let v571 = v107 / v570;
            let v574 = ((v565 * v571) * v125) / v570;
            let v575 = v33 + v566;
            let v577 = v569 * v125;
            let v579 = (-v575) * v578;
            let v580 = v577 * v578;
            let v592 = (v589 * ((v543 + (v548 * v544)) + v542)) / v552;
            let v596 = v592 - v33;
            let v597 = v579 * v596;
            let v600 = (v580 * v596) + ((((((v541 + ((v551 * v544) + (v547 * v548))) + v221) * v589) - (v553 * v592)) / v552) * v579);
            let v608 = (v601 * v564) - (v571 * v597);
            let v609 = (v565 * v601) - ((v574 * v597) + (v600 * v571));
            let v689: f64;
            let v690: f64;
            let v691: f64;
            let v692: f64;
            let v693: f64;
            let v694: Lanes<4>;
            let v695: Lanes<4>;
            let v696: Lanes<4>;
            let v697: Lanes<4>;
            let v698: Lanes<4>;
            if v610 != 0.0 {
                let v656 = v169 * v168;
                let v660 = ((v168 * v168) + v658).sqrt();
                let v663 = (v656 + v656) * (v55 / (v53 * v660));
                let v666 = v60 * (v168 + v660);
                let v667 = (v169 + v663) * v60;
                let v670 = v667 * v668;
                let v671 = v33 + (v668 * v666);
                let v672 = v448 * v671;
                let v677 = v676 / v672;
                let v680 = ((((v449 * v671) + (v670 * v448)) * v677) * v125) / v672;
                v689 = v677;
                v690 = v666;
                v691 = v671;
                v692 = v660;
                v693 = v681;
                v694 = v680;
                v695 = v667;
                v696 = v670;
                v697 = v663;
                v698 = v682;
            } else {
                let v686 = v608 + (v683 * v597);
                let v687 = v609 + (v600 * v683);
                let v688 = if v686 > v30 { 1.0 } else { 0.0 };
                let v1142: f64;
                let v1143: Lanes<4>;
                if v688 != 0.0 {
                    let v1136 = v687 * v1134;
                    let v1137 = v33 + (v1134 * v686);
                    v1142 = v1137;
                    v1143 = v1136;
                } else {
                    let v1140 = v33 - (v1134 * v686);
                    let v1141 = (v687 * v1134) * v125;
                    v1142 = v1140;
                    v1143 = v1141;
                }
                let v1144 = v448 * v1142;
                let v1149 = v1148 / v1144;
                let v1152 = ((((v449 * v1142) + (v1143 * v448)) * v1149) * v125) / v1144;
                v689 = v1149;
                v690 = v30;
                v691 = v30;
                v692 = v30;
                v693 = v1142;
                v694 = v1152;
                v695 = v682;
                v696 = v682;
                v697 = v682;
                v698 = v1143;
            }
            let v700 = v557 + v699;
            let v701 = v700.sqrt();
            let v704 = v169 * (v55 / (v53 * v701));
            let v705 = v187 * v701;
            let v707 = v107 / v705;
            let v710 = (((v704 * v187) * v707) * v125) / v705;
            let v711 = v33 + v707;
            let v712 = v218 - v411;
            let v713 = v221 - v414;
            let v714 = v658 * v711;
            let v716 = v714 * v689;
            let v719 = ((v710 * v658) * v689) + (v694 * v714);
            let v720 = v716 * v712;
            let v723 = (v719 * v712) + (v713 * v716);
            let v724 = v138 + v138;
            let v726 = v144 / v724;
            let v731 = v730 * v726;
            let v732 = ((v145 - ((v141 + v141) * v726)) / v724) * v730;
            let v735 = v97 * v731;
            let v738 = (v731 * v94) / v86;
            let v739 = v89 * v738;
            let v745 = v80 * v731;
            let v748 = (v731 * v77) / v69;
            let v749 = v72 * v748;
            let v753 = v557 / v156;
            let v757 = -v753;
            let v758 = ((v169 - (v159 * v753)) / v156) * v125;
            let v759 = v757 * v738;
            let v762 = (v758 * v738) + (((((v732 * v94) + (Lanes([v735[0], 0.0, v735[1], v735[2]]))) - (Lanes([v739[0], 0.0, v739[1], v739[2]]))) / v86) * v757);
            let v763 = v757 * v748;
            let v766 = (v758 * v748) + (((((v732 * v77) + (Lanes([v745[0], 0.0, v745[1], v745[2]]))) - (Lanes([v749[0], 0.0, v749[1], v749[2]]))) / v69) * v757);
            let v767 = v215 * v173;
            let v768 = v216 * v173;
            let v769 = v767 * v759;
            let v772 = (v768 * v759) + (v762 * v767);
            let v773 = v763 - v33;
            let v774 = v767 * v773;
            let v777 = (v768 * v773) + (v766 * v767);
            let v779 = v778 * v230;
            let v781 = v779 * v222;
            let v785 = v578 / v781;
            let v788 = (((((v233 * v778) * v222) + (v225 * v779)) * v785) * v125) / v781;
            let v789 = v785 * v769;
            let v792 = (v788 * v769) + (v772 * v785);
            let v793 = v785 * v774;
            let v796 = (v788 * v774) + (v777 * v785);
            let v797 = v222 + v222;
            let v799 = v578 / v797;
            let v802 = (((v225 + v225) * v799) * v125) / v797;
            let v810 = v809 * ((v769 * v799) - v789);
            let v811 = (((v772 * v799) + (v802 * v769)) - v792) * v809;
            let v818 = v809 * ((v774 * v799) - v793);
            let v819 = (((v777 * v799) + (v802 * v774)) - v796) * v809;
            let v820 = v33 / v258;
            let v823 = ((v261 * v820) * v125) / v258;
            let v824 = v33 / v270;
            let v827 = ((v273 * v824) * v125) / v270;
            let v832 = (v236 * v789) + v810;
            let v838 = v60 - v789;
            let v844 = (v262 * v838) + v810;
            let v850 = (v832 * v820) - (v844 * v824);
            let v851 = (((((v237 * v789) + (v792 * v236)) + v811) * v820) + (v823 * v832)) - (((((v264 * v838) + ((v792 * v125) * v262)) + v811) * v824) + (v827 * v844));
            let v856 = (v236 * v793) + v818;
            let v863 = v862 - v793;
            let v869 = (v262 * v863) + v818;
            let v875 = (v856 * v820) - (v869 * v824);
            let v876 = (((((v237 * v793) + (v796 * v236)) + v819) * v820) + (v823 * v856)) - (((((v264 * v863) + ((v796 * v125) * v262)) + v819) * v824) + (v827 * v869));
            let v881 = v778 * v287;
            let v883 = v881 * v218;
            let v887 = (v578 * (v222 - v877)) / v883;
            let v890 = ((v225 * v578) - ((((v290 * v778) * v218) + (v221 * v881)) * v887)) / v883;
            let v891 = v887 * v769;
            let v894 = (v890 * v769) + (v772 * v887);
            let v895 = v887 * v774;
            let v898 = (v890 * v774) + (v777 * v887);
            let v899 = v408 * v173;
            let v900 = v409 * v173;
            let v901 = v33 / v303;
            let v904 = ((v306 * v901) * v125) / v303;
            let v905 = v33 / v312;
            let v908 = ((v315 * v905) * v125) / v312;
            let v914 = (v295 * v891) + v810;
            let v922 = v60 - v891;
            let v928 = (v296 * v922) + v810;
            let v934 = ((v759 - v60) - (v914 * v901)) + (v928 * v905);
            let v936 = v899 * v934;
            let v939 = (v900 * v934) + (((v762 - (((((v293 * v891) + (v894 * v295)) + v811) * v901) + (v904 * v914))) + (((((v297 * v922) + ((v894 * v125) * v296)) + v811) * v905) + (v908 * v928))) * v899);
            let v945 = (v295 * v895) + v818;
            let v954 = v953 - v895;
            let v960 = (v296 * v954) + v818;
            let v966 = ((v763 - v60) - (v945 * v901)) + (v960 * v905);
            let v968 = v899 * v966;
            let v971 = (v900 * v966) + (((v766 - (((((v293 * v895) + (v898 * v295)) + v819) * v901) + (v904 * v945))) + (((((v297 * v954) + ((v898 * v125) * v296)) + v819) * v905) + (v908 * v960))) * v899);
            let v973 = (v417 + v240) - v274;
            let v974 = v424 / v973;
            let v977 = ((v416 * v974) * v125) / v973;
            let v978 = v60 - v850;
            let v985 = v984 - v875;
            let v991 = v33 / v442;
            let v994 = ((v445 * v991) * v125) / v442;
            let v1000 = (-(v974 * v978)) + ((v60 + v850) * v432);
            let v1002 = v991 * v1000;
            let v1005 = (v994 * v1000) + (((((v977 * v978) + ((v851 * v125) * v974)) * v125) + (v851 * v432)) * v991);
            let v1012 = (-(v974 * v985)) + ((v1008 + v875) * v432);
            let v1014 = v991 * v1012;
            let v1017 = (v994 * v1012) + (((((v977 * v985) + ((v876 * v125) * v974)) * v125) + (v876 * v432)) * v991);
            let v1018 = v535 * v173;
            let v1019 = v536 * v173;
            let v1020 = v759 - v33;
            let v1021 = v1018 * v1020;
            let v1025 = v1018 * v763;
            let v1032 = (v579 * v1029) / v554;
            let v1035 = ((v580 * v1029) - (v556 * v1032)) / v554;
            let v1038 = v544 + (v187 * v548);
            let v1040 = v1032 * v1038;
            let v1043 = (v1035 * v1038) + ((v547 + (v551 * v187)) * v1032);
            let v1046 = v548 + (v187 * v544);
            let v1048 = v1032 * v1046;
            let v1051 = (v1035 * v1046) + ((v551 + (v547 * v187)) * v1032);
            let v1052 = -v566;
            let v1058 = (v187 + v566) + v566;
            let v1060 = v1058 * v559;
            let v1064 = (v1052 * v597) / v1060;
            let v1067 = (((v577 * v597) + (v600 * v1052)) - ((((v569 + v569) * v559) + (v169 * v1058)) * v1064)) / v1060;
            let v1082 = ((v1064 * v759) + (v1040 * v769)) + (v1048 * v1021);
            let v1083 = (((v1067 * v759) + (v762 * v1064)) + ((v1043 * v769) + (v772 * v1040))) + ((v1051 * v1021) + (((v1019 * v1020) + (v762 * v1018)) * v1048));
            let v1098 = ((v1064 * v763) + (v1040 * v774)) + (v1048 * v1025);
            let v1099 = (((v1067 * v763) + (v766 * v1064)) + ((v1043 * v774) + (v777 * v1040))) + ((v1051 * v1025) + (((v1019 * v763) + (v766 * v1018)) * v1048));
            let v1100 = v187 * v575;
            let v1102 = v1100 * v559;
            let v1106 = v597 / v1102;
            let v1110 = v575 - v1106;
            let v1111 = v569 - ((v600 - ((((v569 * v187) * v559) + (v169 * v1100)) * v1106)) / v1102);
            let v1112 = -v571;
            let v1113 = v574 * v125;
            let v1118 = (v1110 * v759) + v1082;
            let v1120 = v1112 * v1118;
            let v1123 = (v1113 * v1118) + ((((v1111 * v759) + (v762 * v1110)) + v1083) * v1112);
            let v1128 = (v1110 * v763) + v1098;
            let v1130 = v1112 * v1128;
            let v1133 = (v1113 * v1128) + ((((v1111 * v763) + (v766 * v1110)) + v1099) * v1112);
            let v1207: f64;
            let v1208: f64;
            let v1209: Lanes<4>;
            let v1210: Lanes<4>;
            if v610 != 0.0 {
                let v1155 = v691 * v692;
                let v1159 = (v668 * v690) / v1155;
                let v1162 = ((v695 * v668) - (((v696 * v692) + (v697 * v691)) * v1159)) / v1155;
                let v1173 = (-v1002) - (v1159 * v759);
                let v1174 = (v1005 * v125) - ((v1162 * v759) + (v762 * v1159));
                let v1177 = (-v1014) - (v1159 * v763);
                let v1178 = (v1017 * v125) - ((v1162 * v763) + (v766 * v1159));
                v1207 = v1173;
                v1208 = v1177;
                v1209 = v1174;
                v1210 = v1178;
            } else {
                let v1179 = v1134 / v693;
                let v1182 = ((v698 * v1179) * v125) / v693;
                let v1187 = v1120 + (v683 * v1082);
                let v1193 = (-v1002) + (v1179 * v1187);
                let v1194 = (v1005 * v125) + ((v1182 * v1187) + ((v1123 + (v1083 * v683)) * v1179));
                let v1199 = v1130 + (v683 * v1098);
                let v1205 = (-v1014) + (v1179 * v1199);
                let v1206 = (v1017 * v125) + ((v1182 * v1199) + ((v1133 + (v1099 * v683)) * v1179));
                v1207 = v1193;
                v1208 = v1205;
                v1209 = v1194;
                v1210 = v1206;
            }
            let v1211 = v778 * v711;
            let v1213 = v1211 * v701;
            let v1217 = v1213 * v700;
            let v1222 = v1221 / v1217;
            let v1225 = (((((((v710 * v778) * v701) + (v704 * v1211)) * v700) + (v169 * v1213)) * v1222) * v125) / v1217;
            let v1234 = (v1222 * v759) + v1207;
            let v1242 = ((v1234 * v712) + v769) - v936;
            let v1248 = -v716;
            let v1250 = (v1222 * v763) + v1208;
            let v1258 = ((v1250 * v712) + v774) - v968;
            let v1270 = (v33 + ((v1248 * v1258) * v1264)) + ((v716 * v1242) * v1264);
            let v1272 = v33 / v1270;
            let v1276 = v720 * v1272;
            let v1279 = (v723 * v1272) + (((((((((v719 * v125) * v1258) + ((((((((v1225 * v763) + (v766 * v1222)) + v1210) * v712) + (v713 * v1250)) + v777) - v971) * v1248)) * v1264) + (((v719 * v1242) + ((((((((v1225 * v759) + (v762 * v1222)) + v1209) * v712) + (v713 * v1234)) + v772) - v939) * v716)) * v1264)) * v1272) * v125) / v1270) * v720);
            let v1283 = v238 - (v1280 * v236);
            let v1285 = (Lanes([v239[0], 0.0, v239[1], v239[2]])) - (v237 * v1280);
            let v1288 = if (if v1283 > v30 { 1.0 } else { 0.0 }) != 0.0 && v1287 != 0.0 { 1.0 } else { 0.0 };
            let v1299: f64;
            let v1300: Lanes<4>;
            if v1288 != 0.0 {
                let v1289 = v33 / v1283;
                let v1294 = -v1293;
                let v1295 = v1294 * v1289;
                let v1296 = (((v1285 * v1289) * v125) / v1283) * v1294;
                let v1298 = if v1295 < v1297 { 1.0 } else { 0.0 };
                let v1441: f64;
                let v1442: Lanes<4>;
                if v1298 != 0.0 {
                    v1441 = v1440;
                    v1442 = v682;
                } else {
                    v1441 = v1295;
                    v1442 = v1296;
                }
                let v1443 = v1441.exp();
                let v1446 = v1445 * v1283;
                let v1448 = v1446 * v1443;
                let v1452 = v1448 * v1276;
                let v1455 = ((((v1285 * v1445) * v1443) + ((v1442 * v1443) * v1446)) * v1276) + (v1279 * v1448);
                v1299 = v1452;
                v1300 = v1455;
            } else {
                v1299 = v30;
                v1300 = v682;
            }
            let v1301 = v544 * v542;
            let v1304 = (v547 * v542) + (v221 * v544);
            let v1305 = v548 * v543;
            let v1308 = (v551 * v543) + (v541 * v548);
            let v1312 = (v43 + (v60 * v168)).sqrt();
            let v1315 = (v169 * v60) * (v55 / (v53 * v1312));
            let v1316 = v1312 + v1312;
            let v1317 = v1315 + v1315;
            let v1318 = v144 / v1316;
            let v1328 = -(((v33 + v1318) * v578) * v1325);
            let v1329 = ((((v145 - (v1317 * v1318)) / v1316) * v578) * v1325) * v125;
            let v1334 = v1333 * v543;
            let v1342 = v778 * v548;
            let v1357 = (v1354 * ((((v1330 * v1305) + (v1334 * v544)) + (v1342 * v542)) + (v187 * v1301))) / v554;
            let v1361 = v1357 - v60;
            let v1362 = v1328 * v1361;
            let v1365 = (v1329 * v1361) + ((((((((v1308 * v1330) + (((v541 * v1333) * v544) + (v547 * v1334))) + (((v551 * v778) * v542) + (v221 * v1342))) + (v1304 * v187)) * v1354) - (v556 * v1357)) / v554) * v1328);
            let v1368 = v1333 * v542;
            let v1376 = v778 * v544;
            let v1390 = (v1354 * ((((v1330 * v1301) + (v1368 * v548)) + (v1376 * v543)) + (v187 * v1305))) / v554;
            let v1394 = v1390 - v60;
            let v1395 = v1328 * v1394;
            let v1398 = (v1329 * v1394) + ((((((((v1304 * v1330) + (((v221 * v1333) * v548) + (v551 * v1368))) + (((v547 * v778) * v543) + (v541 * v1376))) + (v1308 * v187)) * v1354) - (v556 * v1390)) / v554) * v1328);
            let v1399 = v1395 + v1362;
            let v1400 = v1398 + v1365;
            let v1402 = v1401 * v144;
            let v1419 = v144 + v1316;
            let v1421 = (v1399 * v144) / v1419;
            let v1429 = (-v1399) - ((v1325 * (((v1402 * v564) + v61) - v46)) - v1421);
            let v1430 = (v1400 * v125) - (((((((v145 * v1401) * v564) + (v565 * v1402)) + v154) - (Lanes([0.0, v10[0], 0.0, v10[1]]))) * v1325) - ((((v1400 * v144) + (v145 * v1399)) - ((v145 + v1317) * v1421)) / v1419));
            let v1431 = v8 * v36;
            let v1432 = v1431 * v1276;
            let v1433 = v1279 * v1431;
            let v1434 = ddt(4749, v1362);
            let v1436 = v1365 * v1435;
            let v1437 = ddt(4751, v1395);
            let v1438 = v1398 * v1435;
            let v1439 = if v36 == v33 { 1.0 } else { 0.0 };
            let v1476: f64;
            let v1477: f64;
            let v1478: f64;
            let v1479: f64;
            let v1480: f64;
            let v1481: f64;
            let v1482: f64;
            let v1483: f64;
            let v1484: f64;
            let v1485: f64;
            let v1486: Lanes<4>;
            let v1487: Lanes<4>;
            let v1488: Lanes<4>;
            let v1489: Lanes<4>;
            let v1490: Lanes<4>;
            let v1491: Lanes<4>;
            let v1492: Lanes<4>;
            let v1493: Lanes<4>;
            let v1494: Lanes<4>;
            let v1495: Lanes<4>;
            if v1439 != 0.0 {
                let v1456 = v8 * v1434;
                let v1457 = v1436 * v8;
                let v1458 = v8 * v1362;
                let v1459 = v1365 * v8;
                let v1460 = v8 * v1437;
                let v1461 = v1438 * v8;
                let v1462 = v8 * v1395;
                let v1463 = v1398 * v8;
                let v1464 = v8 * v1299;
                let v1465 = v1300 * v8;
                v1476 = v1456;
                v1477 = v1460;
                v1478 = v1464;
                v1479 = v30;
                v1480 = v30;
                v1481 = v30;
                v1482 = v1458;
                v1483 = v1462;
                v1484 = v30;
                v1485 = v30;
                v1486 = v1457;
                v1487 = v1461;
                v1488 = v1465;
                v1489 = v682;
                v1490 = v682;
                v1491 = v682;
                v1492 = v1459;
                v1493 = v1463;
                v1494 = v682;
                v1495 = v682;
            } else {
                let v1466 = v8 * v1434;
                let v1467 = v1436 * v8;
                let v1468 = v8 * v1362;
                let v1469 = v1365 * v8;
                let v1470 = v8 * v1437;
                let v1471 = v1438 * v8;
                let v1472 = v8 * v1395;
                let v1473 = v1398 * v8;
                let v1474 = v8 * v1299;
                let v1475 = v1300 * v8;
                v1476 = v30;
                v1477 = v30;
                v1478 = v30;
                v1479 = v1466;
                v1480 = v1470;
                v1481 = v1474;
                v1482 = v30;
                v1483 = v30;
                v1484 = v1468;
                v1485 = v1472;
                v1486 = v682;
                v1487 = v682;
                v1488 = v682;
                v1489 = v1467;
                v1490 = v1471;
                v1491 = v1475;
                v1492 = v682;
                v1493 = v682;
                v1494 = v1469;
                v1495 = v1473;
            }
            let v1498 = v8 * (ddt(4775, v1429));
            let v1499 = (v1430 * v1435) * v8;
            let v1500 = v8 * v1429;
            let v1501 = v1430 * v8;
            let v1502 = -v25;
            let v1508 = (v1502 * v1504) / v1507;
            let v1509 = ((v26 * v125) * v1504) / v1507;
            let v1511 = if v1508 < v1510 { 1.0 } else { 0.0 };
            let v1514: f64;
            let v1515: Lanes<2>;
            if v1511 != 0.0 {
                v1514 = v1512;
                v1515 = v1513;
            } else {
                v1514 = v1508;
                v1515 = v1509;
            }
            let v1519 = ((v1502 + v1516) * v1504) / v1507;
            let v1521 = if v1519 > v1520 { 1.0 } else { 0.0 };
            let v1530: f64;
            let v1531: Lanes<2>;
            if v1521 != 0.0 {
                v1530 = v33;
                v1531 = v1513;
            } else {
                let v1524 = (-v1519).exp();
                let v1528 = ((v1509 * v125) * v1524) * v1526;
                let v1529 = v33 + (v1526 * v1524);
                v1530 = v1529;
                v1531 = v1528;
            }
            let v1532 = v25 * v1504;
            let v1533 = v26 * v1504;
            let v1540 = v1537 + v25;
            let v1542 = if v1540 >= v1541 { v1540 } else { v1541 };
            let v1545 = ((v1532 / v1534) * v1537) / v1542;
            let v1549 = v1545.exp();
            let v1561 = v1558 + v25;
            let v1562 = if v1561 >= v1541 { v1561 } else { v1541 };
            let v1565 = ((v1532 / v1555) * v1558) / v1562;
            let v1569 = v1565.exp();
            let v1583 = v1580 + v25;
            let v1584 = if v1583 >= v1541 { v1583 } else { v1541 };
            let v1587 = ((v1532 / v1577) * v1580) / v1584;
            let v1591 = v1587.exp();
            let v1599 = v1514.exp();
            let v1604 = v1603 * (v33 - v1599);
            let v1620 = ((((v1604 * v1530) + (v25 * v1610)) + (((v1552 * (v1549 - v33)) - (v1572 * (v1569 - v33))) - (v1594 * (v1591 - v33)))) * v8) * v1619;
            let v1621 = ((((((((v1515 * v1599) * v125) * v1603) * v1530) + (v1531 * v1604)) + (v26 * v1610)) + ((((((((v1533 / v1534) * v1537) - ((v26 * (if v1540 >= v1541 { 1.0 } else { 0.0 })) * v1545)) / v1542) * v1549) * v1552) - ((((((v1533 / v1555) * v1558) - ((v26 * (if v1561 >= v1541 { 1.0 } else { 0.0 })) * v1565)) / v1562) * v1569) * v1572)) - ((((((v1533 / v1577) * v1580) - ((v26 * (if v1583 >= v1541 { 1.0 } else { 0.0 })) * v1587)) / v1584) * v1591) * v1594))) * v8) * v1619;
            let v1622 = -v17;
            let v1626 = (v1622 * v1504) / v1507;
            let v1627 = ((v18 * v125) * v1504) / v1507;
            let v1629 = if v1626 < v1628 { 1.0 } else { 0.0 };
            let v1632: f64;
            let v1633: Lanes<2>;
            if v1629 != 0.0 {
                v1632 = v1630;
                v1633 = v1631;
            } else {
                v1632 = v1626;
                v1633 = v1627;
            }
            let v1636 = ((v1622 + v1516) * v1504) / v1507;
            let v1637 = if v1636 > v1520 { 1.0 } else { 0.0 };
            let v1645: f64;
            let v1646: Lanes<2>;
            if v1637 != 0.0 {
                v1645 = v33;
                v1646 = v1631;
            } else {
                let v1640 = (-v1636).exp();
                let v1643 = ((v1627 * v125) * v1640) * v1526;
                let v1644 = v33 + (v1526 * v1640);
                v1645 = v1644;
                v1646 = v1643;
            }
            let v1647 = v17 * v1504;
            let v1648 = v18 * v1504;
            let v1653 = v1537 + v17;
            let v1654 = if v1653 >= v1541 { v1653 } else { v1541 };
            let v1657 = ((v1647 / v1534) * v1537) / v1654;
            let v1661 = v1657.exp();
            let v1670 = v1558 + v17;
            let v1671 = if v1670 >= v1541 { v1670 } else { v1541 };
            let v1674 = ((v1647 / v1555) * v1558) / v1671;
            let v1678 = v1674.exp();
            let v1690 = v1580 + v17;
            let v1691 = if v1690 >= v1541 { v1690 } else { v1541 };
            let v1694 = ((v1647 / v1577) * v1580) / v1691;
            let v1698 = v1694.exp();
            let v1706 = v1632.exp();
            let v1711 = v1710 * (v33 - v1706);
            let v1725 = ((((v1711 * v1645) + (v17 * v1610)) + (((v1552 * (v1661 - v33)) - (v1681 * (v1678 - v33))) - (v1701 * (v1698 - v33)))) * v8) * v1619;
            let v1726 = ((((((((v1633 * v1706) * v125) * v1710) * v1645) + (v1646 * v1711)) + (v18 * v1610)) + ((((((((v1648 / v1534) * v1537) - ((v18 * (if v1653 >= v1541 { 1.0 } else { 0.0 })) * v1657)) / v1654) * v1661) * v1552) - ((((((v1648 / v1555) * v1558) - ((v18 * (if v1670 >= v1541 { 1.0 } else { 0.0 })) * v1674)) / v1671) * v1678) * v1681)) - ((((((v1648 / v1577) * v1580) - ((v18 * (if v1690 >= v1541 { 1.0 } else { 0.0 })) * v1694)) / v1691) * v1698) * v1701))) * v8) * v1619;
            let v1727 = if v25 > v30 { 1.0 } else { 0.0 };
            let v1809: f64;
            let v1810: f64;
            let v1811: f64;
            let v1812: Lanes<2>;
            let v1813: Lanes<2>;
            let v1814: Lanes<2>;
            if v1727 != 0.0 {
                let v1730 = v1728 * v1729;
                let v1732 = -v1731;
                let v1736 = v33 + (v25 / v1733);
                let v1742 = (v1732 * (v1736.ln())).exp();
                let v1744 = v1730 * v1742;
                let v1745 = ((((v26 / v1733) * (v55 / v1736)) * v1732) * v1742) * v1730;
                let v1748 = v1746 * v1747;
                let v1750 = -v1749;
                let v1754 = v33 + (v25 / v1751);
                let v1760 = (v1750 * (v1754.ln())).exp();
                let v1762 = v1748 * v1760;
                let v1763 = ((((v26 / v1751) * (v55 / v1754)) * v1750) * v1760) * v1748;
                let v1766 = v1764 * v1765;
                let v1768 = -v1767;
                let v1772 = v33 + (v25 / v1769);
                let v1778 = (v1768 * (v1772.ln())).exp();
                let v1780 = v1766 * v1778;
                let v1781 = ((((v26 / v1769) * (v55 / v1772)) * v1768) * v1778) * v1766;
                v1809 = v1744;
                v1810 = v1762;
                v1811 = v1780;
                v1812 = v1745;
                v1813 = v1763;
                v1814 = v1781;
            } else {
                let v1782 = v1728 * v1729;
                let v1789 = v1782 * (v33 - ((v1731 * v25) / v1733));
                let v1790 = (((v26 * v1731) / v1733) * v125) * v1782;
                let v1791 = v1746 * v1747;
                let v1798 = v1791 * (v33 - ((v1749 * v25) / v1751));
                let v1799 = (((v26 * v1749) / v1751) * v125) * v1791;
                let v1800 = v1764 * v1765;
                let v1807 = v1800 * (v33 - ((v1767 * v25) / v1769));
                let v1808 = (((v26 * v1767) / v1769) * v125) * v1800;
                v1809 = v1789;
                v1810 = v1798;
                v1811 = v1807;
                v1812 = v1790;
                v1813 = v1799;
                v1814 = v1808;
            }
            let v1817 = (v1809 + v1810) + v1811;
            let v1819 = v1817 * v25;
            let v1822 = (((v1812 + v1813) + v1814) * v25) + (v26 * v1817);
            let v1829 = ((ddt(5312, v1819)) * v8) * v1619;
            let v1830 = ((v1822 * v1435) * v8) * v1619;
            let v1831 = (v1819 * v8) * v1619;
            let v1832 = (v1822 * v8) * v1619;
            let v1833 = if v17 > v30 { 1.0 } else { 0.0 };
            let v1905: f64;
            let v1906: f64;
            let v1907: f64;
            let v1908: Lanes<2>;
            let v1909: Lanes<2>;
            let v1910: Lanes<2>;
            if v1833 != 0.0 {
                let v1835 = v1728 * v1834;
                let v1836 = -v1731;
                let v1839 = v33 + (v17 / v1733);
                let v1845 = (v1836 * (v1839.ln())).exp();
                let v1847 = v1835 * v1845;
                let v1848 = ((((v18 / v1733) * (v55 / v1839)) * v1836) * v1845) * v1835;
                let v1850 = v1746 * v1849;
                let v1851 = -v1749;
                let v1854 = v33 + (v17 / v1751);
                let v1860 = (v1851 * (v1854.ln())).exp();
                let v1862 = v1850 * v1860;
                let v1863 = ((((v18 / v1751) * (v55 / v1854)) * v1851) * v1860) * v1850;
                let v1864 = v1764 * v1765;
                let v1865 = -v1767;
                let v1868 = v33 + (v17 / v1769);
                let v1874 = (v1865 * (v1868.ln())).exp();
                let v1876 = v1864 * v1874;
                let v1877 = ((((v18 / v1769) * (v55 / v1868)) * v1865) * v1874) * v1864;
                v1905 = v1847;
                v1906 = v1862;
                v1907 = v1876;
                v1908 = v1848;
                v1909 = v1863;
                v1910 = v1877;
            } else {
                let v1878 = v1728 * v1834;
                let v1885 = v1878 * (v33 - ((v1731 * v17) / v1733));
                let v1886 = (((v18 * v1731) / v1733) * v125) * v1878;
                let v1887 = v1746 * v1849;
                let v1894 = v1887 * (v33 - ((v1749 * v17) / v1751));
                let v1895 = (((v18 * v1749) / v1751) * v125) * v1887;
                let v1896 = v1764 * v1765;
                let v1903 = v1896 * (v33 - ((v1767 * v17) / v1769));
                let v1904 = (((v18 * v1767) / v1769) * v125) * v1896;
                v1905 = v1885;
                v1906 = v1894;
                v1907 = v1903;
                v1908 = v1886;
                v1909 = v1895;
                v1910 = v1904;
            }
            let v1913 = (v1905 + v1906) + v1907;
            let v1915 = v1913 * v17;
            let v1918 = (((v1908 + v1909) + v1910) * v17) + (v18 * v1913);
            let v1925 = ((ddt(5403, v1915)) * v8) * v1619;
            let v1926 = ((v1918 * v1435) * v8) * v1619;
            let v1927 = (v1915 * v8) * v1619;
            let v1928 = (v1918 * v8) * v1619;
            let v1929 = v1433[0];
            let v1930 = v1433[1];
            let v1931 = v1433[2];
            let v1932 = v1433[3];
            let v1933 = v1486[0];
            let v1934 = v1486[1];
            let v1935 = v1486[2];
            let v1936 = v1486[3];
            let v1937 = v1487[0];
            let v1938 = v1487[1];
            let v1939 = v1487[2];
            let v1940 = v1487[3];
            let v1941 = v1488[0];
            let v1942 = v1488[1];
            let v1943 = v1488[2];
            let v1944 = v1488[3];
            let v1945 = v1489[0];
            let v1946 = v1489[1];
            let v1947 = v1489[2];
            let v1948 = v1489[3];
            let v1949 = v1490[0];
            let v1950 = v1490[1];
            let v1951 = v1490[2];
            let v1952 = v1490[3];
            let v1953 = v1491[0];
            let v1954 = v1491[1];
            let v1955 = v1491[2];
            let v1956 = v1491[3];
            let v1957 = v1499[0];
            let v1958 = v1499[1];
            let v1959 = v1499[2];
            let v1960 = v1499[3];
            let v1961 = v1621[0];
            let v1962 = v1621[1];
            let v1963 = v1726[0];
            let v1964 = v1726[1];
            let v1965 = v1830[0];
            let v1966 = v1830[1];
            let v1967 = v1926[0];
            let v1968 = v1926[1];
            let v1969 = v1492[0];
            let v1970 = v1492[1];
            let v1971 = v1492[2];
            let v1972 = v1492[3];
            let v1973 = v1493[0];
            let v1974 = v1493[1];
            let v1975 = v1493[2];
            let v1976 = v1493[3];
            let v1977 = v1494[0];
            let v1978 = v1494[1];
            let v1979 = v1494[2];
            let v1980 = v1494[3];
            let v1981 = v1495[0];
            let v1982 = v1495[1];
            let v1983 = v1495[2];
            let v1984 = v1495[3];
            let v1985 = v1501[0];
            let v1986 = v1501[1];
            let v1987 = v1501[2];
            let v1988 = v1501[3];
            let v1989 = v1832[0];
            let v1990 = v1832[1];
            let v1991 = v1928[0];
            let v1992 = v1928[1];
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(0),
            Some(2),
            multiplicity * (v1432),
            [0, 1, 2, 3],
            [v1929, v1930, v1931, v1932],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(0),
            Some(3),
            multiplicity * (v1476),
            [0, 1, 2, 3],
            [v1933, v1934, v1935, v1936],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(2),
            Some(3),
            multiplicity * (v1477),
            [0, 1, 2, 3],
            [v1937, v1938, v1939, v1940],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(0),
            Some(3),
            multiplicity * (v1478),
            [0, 1, 2, 3],
            [v1941, v1942, v1943, v1944],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(2),
            Some(3),
            multiplicity * (v1479),
            [0, 1, 2, 3],
            [v1945, v1946, v1947, v1948],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(0),
            Some(3),
            multiplicity * (v1480),
            [0, 1, 2, 3],
            [v1949, v1950, v1951, v1952],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(2),
            Some(3),
            multiplicity * (v1481),
            [0, 1, 2, 3],
            [v1953, v1954, v1955, v1956],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(3),
            multiplicity * (v1498),
            [0, 1, 2, 3],
            [v1957, v1958, v1959, v1960],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(2),
            multiplicity * (staged[83]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(0),
            Some(3),
            multiplicity * (v1620),
            [0, 3],
            [v1961, v1962],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(2),
            Some(3),
            multiplicity * (v1725),
            [2, 3],
            [v1963, v1964],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(0),
            Some(3),
            multiplicity * (v1829),
            [0, 3],
            [v1965, v1966],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(2),
            Some(3),
            multiplicity * (v1925),
            [2, 3],
            [v1967, v1968],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = v1432;
        self.canonical_reactive[1] = v1482;
        self.canonical_reactive[2] = v1969;
        self.canonical_reactive[3] = v1970;
        self.canonical_reactive[4] = v1971;
        self.canonical_reactive[5] = v1972;
        self.canonical_reactive[6] = v1483;
        self.canonical_reactive[7] = v1973;
        self.canonical_reactive[8] = v1974;
        self.canonical_reactive[9] = v1975;
        self.canonical_reactive[10] = v1976;
        self.canonical_reactive[11] = v1478;
        self.canonical_reactive[12] = v1484;
        self.canonical_reactive[13] = v1977;
        self.canonical_reactive[14] = v1978;
        self.canonical_reactive[15] = v1979;
        self.canonical_reactive[16] = v1980;
        self.canonical_reactive[17] = v1485;
        self.canonical_reactive[18] = v1981;
        self.canonical_reactive[19] = v1982;
        self.canonical_reactive[20] = v1983;
        self.canonical_reactive[21] = v1984;
        self.canonical_reactive[22] = v1481;
        self.canonical_reactive[23] = v1500;
        self.canonical_reactive[24] = v1985;
        self.canonical_reactive[25] = v1986;
        self.canonical_reactive[26] = v1987;
        self.canonical_reactive[27] = v1988;
        self.canonical_reactive[28] = staged[83];
        self.canonical_reactive[29] = v1620;
        self.canonical_reactive[30] = v1725;
        self.canonical_reactive[31] = v1831;
        self.canonical_reactive[32] = v1989;
        self.canonical_reactive[33] = v1990;
        self.canonical_reactive[34] = v1927;
        self.canonical_reactive[35] = v1991;
        self.canonical_reactive[36] = v1992;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(0),
            Some(3),
            &[0, 1, 2, 3],
            &[cached[2], cached[3], cached[4], cached[5]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(2),
            Some(3),
            &[0, 1, 2, 3],
            &[cached[7], cached[8], cached[9], cached[10]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(2),
            Some(3),
            &[0, 1, 2, 3],
            &[cached[13], cached[14], cached[15], cached[16]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(0),
            Some(3),
            &[0, 1, 2, 3],
            &[cached[18], cached[19], cached[20], cached[21]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(3),
            &[0, 1, 2, 3],
            &[cached[24], cached[25], cached[26], cached[27]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(0),
            Some(3),
            &[0, 3],
            &[cached[32], cached[33]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(2),
            Some(3),
            &[2, 3],
            &[cached[35], cached[36]],
            &[],
            &[],
            multiplicity,
        );
    }

}

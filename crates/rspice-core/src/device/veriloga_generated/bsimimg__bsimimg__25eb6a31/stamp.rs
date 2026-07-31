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
        let mut key = Vec::with_capacity(1496);
        for index in 0..Self::PARAMETER_COUNT {
            if PARAMETER_MODEL_FLAGS[index] {
                key.push(self.params.values[index].to_bits());
                key.push(u64::from(self.param_given[index]));
            }
        }
        key.into_boxed_slice()
    }

    fn canonical_install_model_values(&mut self, values: Arc<CanonicalModelValues>) {
        self.canonical_staged[267] = values[0];
        self.canonical_staged[268] = values[1];
        self.canonical_staged[269] = values[2];
        self.canonical_staged[107] = values[3];
        self.canonical_staged[270] = values[4];
        self.canonical_staged[0] = values[5];
        self.canonical_staged[1] = values[6];
        self.canonical_staged[2] = values[7];
        self.canonical_staged[3] = values[8];
        self.canonical_staged[253] = values[9];
        self.canonical_staged[283] = values[10];
        self.canonical_staged[103] = values[11];
        self.canonical_staged[338] = values[12];
        self.canonical_staged[7] = values[13];
        self.canonical_staged[122] = values[14];
        self.canonical_staged[184] = values[15];
        self.canonical_staged[284] = values[16];
        self.canonical_staged[285] = values[17];
        self.canonical_staged[4] = values[18];
        self.canonical_staged[286] = values[19];
        self.canonical_staged[5] = values[20];
        self.canonical_staged[287] = values[21];
        self.canonical_staged[6] = values[22];
        self.canonical_staged[296] = values[23];
        self.canonical_staged[298] = values[24];
        self.canonical_staged[300] = values[25];
        self.canonical_staged[64] = values[26];
        self.canonical_staged[8] = values[27];
        self.canonical_staged[307] = values[28];
        self.canonical_staged[233] = values[29];
        self.canonical_staged[125] = values[30];
        self.canonical_staged[242] = values[31];
        self.canonical_staged[62] = values[32];
        self.canonical_staged[127] = values[33];
        self.canonical_staged[309] = values[34];
        self.canonical_staged[312] = values[35];
        self.canonical_staged[315] = values[36];
        self.canonical_staged[313] = values[37];
        self.canonical_staged[314] = values[38];
        self.canonical_staged[10] = values[39];
        self.canonical_staged[9] = values[40];
        self.canonical_staged[11] = values[41];
        self.canonical_staged[12] = values[42];
        self.canonical_staged[13] = values[43];
        self.canonical_staged[14] = values[44];
        self.canonical_staged[326] = values[45];
        self.canonical_staged[322] = values[46];
        self.canonical_staged[323] = values[47];
        self.canonical_staged[17] = values[48];
        self.canonical_staged[19] = values[49];
        self.canonical_staged[22] = values[50];
        self.canonical_staged[68] = values[51];
        self.canonical_staged[63] = values[52];
        self.canonical_staged[67] = values[53];
        self.canonical_staged[325] = values[54];
        self.canonical_staged[157] = values[55];
        self.canonical_staged[83] = values[56];
        self.canonical_staged[82] = values[57];
        self.canonical_staged[86] = values[58];
        self.canonical_staged[87] = values[59];
        self.canonical_staged[101] = values[60];
        self.canonical_staged[104] = values[61];
        self.canonical_staged[112] = values[62];
        self.canonical_staged[109] = values[63];
        self.canonical_staged[108] = values[64];
        self.canonical_staged[111] = values[65];
        self.canonical_staged[110] = values[66];
        self.canonical_staged[113] = values[67];
        self.canonical_staged[114] = values[68];
        self.canonical_staged[115] = values[69];
        self.canonical_staged[116] = values[70];
        self.canonical_staged[117] = values[71];
        self.canonical_staged[118] = values[72];
        self.canonical_staged[119] = values[73];
        self.canonical_staged[120] = values[74];
        self.canonical_staged[121] = values[75];
        self.canonical_staged[128] = values[76];
        self.canonical_staged[131] = values[77];
        self.canonical_staged[327] = values[78];
        self.canonical_staged[141] = values[79];
        self.canonical_staged[150] = values[80];
        self.canonical_staged[151] = values[81];
        self.canonical_staged[152] = values[82];
        self.canonical_staged[153] = values[83];
        self.canonical_staged[154] = values[84];
        self.canonical_staged[328] = values[85];
        self.canonical_staged[155] = values[86];
        self.canonical_staged[156] = values[87];
        self.canonical_staged[329] = values[88];
        self.canonical_staged[160] = values[89];
        self.canonical_staged[161] = values[90];
        self.canonical_staged[162] = values[91];
        self.canonical_staged[165] = values[92];
        self.canonical_staged[173] = values[93];
        self.canonical_staged[176] = values[94];
        self.canonical_staged[334] = values[95];
        self.canonical_staged[182] = values[96];
        self.canonical_staged[189] = values[97];
        self.canonical_staged[190] = values[98];
        self.canonical_staged[192] = values[99];
        self.canonical_staged[339] = values[100];
        self.canonical_staged[205] = values[101];
        self.canonical_staged[211] = values[102];
        self.canonical_staged[340] = values[103];
        self.canonical_staged[217] = values[104];
        self.canonical_staged[341] = values[105];
        self.canonical_staged[342] = values[106];
        self.canonical_staged[343] = values[107];
        self.canonical_staged[344] = values[108];
        self.canonical_staged[246] = values[109];
        self.canonical_staged[249] = values[110];
        self.canonical_staged[346] = values[111];
        self.canonical_staged[347] = values[112];
        self.canonical_staged[348] = values[113];
        self.canonical_staged[349] = values[114];
        self.canonical_staged[350] = values[115];
        self.canonical_staged[351] = values[116];
        self.canonical_staged[352] = values[117];
        self.canonical_staged[354] = values[118];
        self.canonical_staged[355] = values[119];
        self.canonical_staged[356] = values[120];
        self.canonical_staged[357] = values[121];
        self.canonical_staged[358] = values[122];
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
                let v0 = 1.0f64;
                let v1 = parameters[18];
                let v2 = 0e0f64;
                let v4 = parameters[310];
                let v7 = parameters[12];
                let v8 = 1e0f64;
                let v10 = -1e0f64;
                let v12 = parameters[13];
                let v14 = -1e0f64;
                let v16 = parameters[59];
                let v17 = 8.85418e-12f64;
                let v19 = parameters[21];
                let v21 = parameters[29];
                let v23 = parameters[30];
                let v25 = parameters[35];
                let v27 = parameters[36];
                let v29 = parameters[20];
                let v31 = parameters[317];
                let v34 = 3.4531302e-11f64;
                let v35 = parameters[45];
                let v37 = 3.4531302e-11f64;
                let v38 = parameters[47];
                let v40 = 3.4531302e-11f64;
                let v41 = parameters[46];
                let v43 = parameters[49];
                let v45 = 3.9e0f64;
                let v47 = if parameter_given[47] { 1.0 } else { 0.0 };
                let v49 = parameters[60];
                let v52 = parameters[48];
                let v55 = parameters[138];
                let v58 = parameters[188];
                let v61 = parameters[14];
                let v63 = parameters[115];
                let v65 = parameters[190];
                let v68 = parameters[194];
                let v71 = parameters[198];
                let v79 = 1e-38f64;
                let v82 = parameters[267];
                let v85 = 3.333333333333333e-1f64;
                let v86 = 5e-1f64;
                let v89 = 1e-8f64;
                let v103 = parameters[297];
                let v109 = 3.0015e2f64;
                let v110 = 2.7315e2f64;
                let v113 = 4.97232e-7f64;
                let v114 = 3.42537e-7f64;
                let v116 = 7.45669e11f64;
                let v117 = 1.16645e12f64;
                let v119 = parameters[99];
                let v121 = parameters[239];
                let v125 = parameters[298];
                let v127 = parameters[55];
                let v128 = 5.1728331239999994e-2f64;
                let v130 = parameters[52];
                let v132 = if parameter_given[58] { 1.0 } else { 0.0 };
                let v135 = -1e0f64;
                let v142 = 3.75e-1f64;
                let v149 = -1e0f64;
                let v151 = 1.60219e-19f64;
                let v154 = 2e0f64;
                let v182 = -2e0f64;
                let v185 = -2e0f64;
                let v187 = -2e0f64;
                let v189 = -2e0f64;
                let v191 = -2e0f64;
                let v193 = 1e-2f64;
                let v195 = 2.5e-1f64;
                let v196 = parameters[154];
                let v200 = -2e0f64;
                let v202 = -2e0f64;
                let v204 = -2e0f64;
                let v206 = -2e0f64;
                let v208 = -2e0f64;
                let v210 = parameters[162];
                let v214 = parameters[189];
                let v217 = parameters[109];
                let v220 = parameters[134];
                let v225 = parameters[265];
                let v227 = parameters[266];
                let v229 = parameters[17];
                let v231 = -9.82222e11f64;
                let v233 = parameters[16];
                let v235 = -7.45669e11f64;
                let v239 = parameters[15];
                let v241 = parameters[288];
                let v243 = parameters[289];
                let v246 = parameters[290];
                let v249 = parameters[287];
                let v251 = parameters[22];
                let v253 = parameters[292];
                let v258 = 0e0f64;
                let v259 = 0e0f64;
                let v260 = 0e0f64;
                let v261 = 0e0f64;
                let v266 = parameters[19];
                let v268 = 0e0f64;
                let v269 = 0e0f64;
                let v272 = 0e0f64;
                let v273 = 0e0f64;
                let v277 = 0e0f64;
                let mut out6: f64 = 0.0;
                let mut out57: f64 = 0.0;
                let mut out60: f64 = 0.0;
                let mut out98: f64 = 0.0;
                let mut out99: f64 = 0.0;
                let mut out106: f64 = 0.0;
                let mut out136: f64 = 0.0;
                let mut out137: f64 = 0.0;
                let mut out138: f64 = 0.0;
                let mut out199: f64 = 0.0;
                let mut out212: f64 = 0.0;
                let mut out213: f64 = 0.0;
                let mut out223: f64 = 0.0;
                let mut out232: f64 = 0.0;
                let mut out236: f64 = 0.0;
                let mut out238: f64 = 0.0;
                let mut out250: f64 = 0.0;
                let mut out252: f64 = 0.0;
                let mut out255: f64 = 0.0;
                let mut out256: f64 = 0.0;
                let mut out276: f64 = 0.0;
                if v0 != 0.0 {
                    let v6 = if (if v1 == v2 { 1.0 } else { 0.0 }) != 0.0 || (if v4 == v2 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    out6 = v6;
                } else {
                }
                let v9 = if v7 == v8 { 1.0 } else { 0.0 };
                let v11: f64;
                if v9 != 0.0 {
                    v11 = v8;
                } else {
                    v11 = v10;
                }
                let v13 = if v12 == v8 { 1.0 } else { 0.0 };
                let v15: f64;
                if v13 != 0.0 {
                    v15 = v8;
                } else {
                    v15 = v14;
                }
                let v18 = v16 * v17;
                let v20 = if v19 == v2 { 1.0 } else { 0.0 };
                let v22 = -v21;
                let v24 = -v23;
                let v26 = -v25;
                let v28 = -v27;
                let v30 = if v29 == v8 { 1.0 } else { 0.0 };
                let v33 = if v30 != 0.0 && (if v31 != v2 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v36 = v34 / v35;
                let v39 = v37 / v38;
                let v42 = v40 / v41;
                let v44 = v18 / v43;
                let v46 = v16 / v45;
                let v48 = if v47 == 0.0 { 1.0 } else { 0.0 };
                let v54: f64;
                if v48 != 0.0 {
                    let v53 = ((v35 * v49) / v45) - v52;
                    v54 = v53;
                } else {
                    v54 = v38;
                }
                let v56 = if v55 > v2 { 1.0 } else { 0.0 };
                if v56 != 0.0 {
                    let v57 = -v55;
                    out57 = v57;
                } else {
                }
                let v59 = if v58 > v2 { 1.0 } else { 0.0 };
                if v59 != 0.0 {
                    let v60 = -v58;
                    out60 = v60;
                } else {
                }
                let v62 = if v61 == v8 { 1.0 } else { 0.0 };
                let v64 = -v63;
                let v66 = if v65 < v2 { 1.0 } else { 0.0 };
                let v67: f64;
                if v66 != 0.0 {
                    v67 = v2;
                } else {
                    v67 = v65;
                }
                let v69 = if v68 < v2 { 1.0 } else { 0.0 };
                let v70: f64;
                if v69 != 0.0 {
                    v70 = v2;
                } else {
                    v70 = v68;
                }
                let v72 = if v71 < v2 { 1.0 } else { 0.0 };
                let v73: f64;
                if v72 != 0.0 {
                    v73 = v2;
                } else {
                    v73 = v71;
                }
                let v76 = v43 + (v46 * (v35 + v41));
                let v83 = v82 * ((if (v8 + (v43 / v41)) >= v79 { (v8 + (v43 / v41)) } else { v79 }).ln());
                let v84 = if v7 != v8 { 1.0 } else { 0.0 };
                let v87: f64;
                if v84 != 0.0 {
                    v87 = v85;
                } else {
                    v87 = v86;
                }
                let v88 = v46 * v35;
                let v90 = v89 / v88;
                let v92 = (v88 * v43).sqrt();
                let v93 = v46 * v41;
                let v94 = v89 / v93;
                let v97 = if (if v1 != v2 { 1.0 } else { 0.0 }) != 0.0 && (if v4 > v2 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v100: f64;
                let v101: f64;
                let v102: f64;
                if v62 != 0.0 {
                    let v98 = if v70 <= v2 { 1.0 } else { 0.0 };
                    out98 = v98;
                    let v105: f64;
                    if v98 != 0.0 {
                        v105 = v2;
                    } else {
                        v105 = v70;
                    }
                    let v106 = if v73 <= v2 { 1.0 } else { 0.0 };
                    out106 = v106;
                    let v107: f64;
                    if v106 != 0.0 {
                        v107 = v2;
                    } else {
                        v107 = v73;
                    }
                    v100 = v67;
                    v101 = v105;
                    v102 = v107;
                } else {
                    let v99 = if v67 <= v2 { 1.0 } else { 0.0 };
                    out99 = v99;
                    let v108: f64;
                    if v99 != 0.0 {
                        v108 = v2;
                    } else {
                        v108 = v67;
                    }
                    v100 = v108;
                    v101 = v70;
                    v102 = v73;
                }
                let v104 = if v103 <= v2 { 1.0 } else { 0.0 };
                let v112: f64;
                if v104 != 0.0 {
                    v112 = v109;
                } else {
                    let v111 = v103 + v110;
                    v112 = v111;
                }
                let v115: f64;
                if v9 != 0.0 {
                    v115 = v113;
                } else {
                    v115 = v114;
                }
                let v118: f64;
                if v9 != 0.0 {
                    v118 = v116;
                } else {
                    v118 = v117;
                }
                let v120 = v119 * v119;
                let v124 = (if (v121 / v119) >= v79 { (v121 / v119) } else { v79 }).ln();
                let v126 = v125 + v110;
                let v129 = v127 / v128;
                let v131 = if v130 != v2 { 1.0 } else { 0.0 };
                let v134 = if v131 != 0.0 && (if v132 == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v134 != 0.0 {
                    let v136 = if v12 == v135 { 1.0 } else { 0.0 };
                    out136 = v136;
                    if v136 != 0.0 {
                        let v137 = v86 * v127;
                        out137 = v137;
                    } else {
                        let v138 = v86 * v127;
                        out138 = v138;
                    }
                } else {
                }
                let v146 = (v43 * (v88 + (v142 * v43))).sqrt();
                let v147 = v88 + v43;
                let v148 = (((v46 * v43) * v35).sqrt()) - v146;
                let v150 = if v12 == v149 { 1.0 } else { 0.0 };
                let v155 = v154 * v42;
                let v157 = ((v151 * v130) * v18) / (v155 * v42);
                let v160 = v42 + v44;
                let v162 = ((-v42) * v44) / (v160 * v36);
                let v163 = v11 * v15;
                let v166 = v36 + ((v44 * v42) / v160);
                let v170 = v8 - ((v86 * v43) / (v43 + v93));
                let v171 = v36 / v44;
                let v172 = v42 / v44;
                let v173 = v171 * v171;
                let v177 = v171 / (((v172 * v171) + v172) + v171);
                let v178 = v8 + v172;
                let v179 = v8 + v171;
                let v180 = v172 * v172;
                let v181 = v154 * v173;
                let v183 = v182 * v171;
                let v184 = -v171;
                let v186 = v185 * v171;
                let v188 = v187 * v171;
                let v190 = v189 * v171;
                let v192 = v191 * v171;
                let v194 = v193 / v36;
                let v198 = (v195 * v196) * v196;
                if v62 != 0.0 {
                } else {
                    let v199 = if v61 == v2 { 1.0 } else { 0.0 };
                    out199 = v199;
                }
                let v201 = v200 * v171;
                let v203 = v202 * v171;
                let v205 = v204 * v171;
                let v207 = v206 * v171;
                let v209 = v208 * v171;
                let v211 = if v210 != v2 { 1.0 } else { 0.0 };
                if v211 != 0.0 {
                    let v212 = v154 * v36;
                    out212 = v212;
                } else {
                    let v213 = v154 * v36;
                    out213 = v213;
                }
                let v215 = if v214 != v2 { 1.0 } else { 0.0 };
                let v216 = v154 * v36;
                let v219 = v8 + (v217.sqrt());
                let v222 = (v195 * v220) * v220;
                if v62 != 0.0 {
                } else {
                    let v223 = if v61 == v154 { 1.0 } else { 0.0 };
                    out223 = v223;
                }
                let v224 = v35 / v41;
                let v226 = v86 * v225;
                let v228 = v86 * v227;
                let v230 = if v229 != v2 { 1.0 } else { 0.0 };
                if v230 != 0.0 {
                    let v232 = v231 * v119;
                    out232 = v232;
                    let v236 = v235 * v119;
                    out236 = v236;
                } else {
                }
                let v234 = if v233 != v2 { 1.0 } else { 0.0 };
                if v234 != 0.0 {
                    let v238 = (-v118) * v119;
                    out238 = v238;
                } else {
                }
                let v240 = if v239 != v2 { 1.0 } else { 0.0 };
                let v248 = if (if (if v241 > v2 { 1.0 } else { 0.0 }) != 0.0 || (if v243 > v2 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v246 > v2 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v248 != 0.0 {
                    let v250 = if v249 <= v2 { 1.0 } else { 0.0 };
                    out250 = v250;
                    let v252 = if v251 == v8 { 1.0 } else { 0.0 };
                    out252 = v252;
                    if v252 != 0.0 {
                        let v255 = (v195 * v253) * v253;
                        out255 = v255;
                    } else {
                    }
                    let v256 = v86 * v246;
                    out256 = v256;
                } else {
                }
                let v257 = if v61 == v154 { 1.0 } else { 0.0 };
                let v262: f64;
                let v263: f64;
                let v264: f64;
                let v265: f64;
                if v257 != 0.0 {
                    v262 = v258;
                    v263 = v259;
                    v264 = v2;
                    v265 = v2;
                } else {
                    v262 = v2;
                    v263 = v2;
                    v264 = v260;
                    v265 = v261;
                }
                let v267 = if v266 == v2 { 1.0 } else { 0.0 };
                let v270: f64;
                let v271: f64;
                if v267 != 0.0 {
                    v270 = v268;
                    v271 = v2;
                } else {
                    v270 = v2;
                    v271 = v269;
                }
                let v274: f64;
                let v275: f64;
                if v230 != 0.0 {
                    v274 = v272;
                    v275 = v273;
                } else {
                    v274 = v2;
                    v275 = v2;
                }
                let v278: f64;
                if v97 != 0.0 {
                    let v276 = if v61 != v154 { 1.0 } else { 0.0 };
                    out276 = v276;
                    v278 = v2;
                } else {
                    v278 = v277;
                }
            [out6, v9, v13, v18, v20, v22, v24, v26, v28, v30, v33, v36, v39, v42, v44, v46, v48, v56, out57, v59, out60, v62, v64, v66, v69, v72, v76, v83, v84, v88, v90, v92, v93, v94, v97, out98, out106, out99, v104, v120, v124, v115, v126, v112, v129, v131, v134, out136, out137, out138, v11, v146, v147, v148, v150, v155, v157, v15, v162, v163, v166, v170, v171, v172, v173, v177, v178, v179, v180, v181, v183, v184, v186, v188, v190, v192, v194, v198, out199, v100, v201, v203, v205, v207, v209, v211, out212, out213, v215, v216, v87, v219, v222, v101, v102, out223, v54, v224, v226, v228, v230, out232, out236, v234, out238, v240, v248, out250, out252, out255, out256, v257, v267, out276, v262, v263, v264, v265, v270, v271, v274, v275, v278]
        };
        let values = canonical_model_cache_intern(key, Arc::new(produced));
        self.canonical_install_model_values(values);
    }

    fn canonical_instance_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        if self.canonical_instance_valid {
            return;
        }
        let produced: [f64; 235] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let v0 = staged[270];
                let v1 = parameters[1];
                let v2 = parameters[2];
                let v5 = parameters[0];
                let v6 = parameters[23];
                let v8 = parameters[24];
                let v10 = staged[0];
                let v12 = staged[1];
                let v15 = parameters[26];
                let v17 = parameters[25];
                let v19 = parameters[27];
                let v22 = parameters[28];
                let v25 = staged[2];
                let v27 = staged[3];
                let v30 = parameters[32];
                let v32 = parameters[31];
                let v34 = parameters[33];
                let v37 = parameters[34];
                let v40 = 2e0f64;
                let v43 = 0e0f64;
                let v45 = 1e-9f64;
                let v51 = parameters[38];
                let v53 = parameters[37];
                let v55 = parameters[39];
                let v58 = parameters[40];
                let v61 = parameters[42];
                let v63 = parameters[41];
                let v65 = parameters[43];
                let v68 = parameters[44];
                let v79 = 1e-6f64;
                let v83 = parameters[319];
                let v85 = parameters[191];
                let v87 = parameters[320];
                let v90 = parameters[321];
                let v93 = parameters[325];
                let v95 = parameters[199];
                let v97 = parameters[326];
                let v100 = parameters[327];
                let v103 = parameters[322];
                let v105 = parameters[195];
                let v107 = parameters[323];
                let v110 = parameters[324];
                let v113 = parameters[328];
                let v115 = parameters[202];
                let v117 = parameters[329];
                let v120 = parameters[330];
                let v123 = parameters[331];
                let v125 = parameters[203];
                let v127 = parameters[332];
                let v130 = parameters[333];
                let v133 = parameters[334];
                let v135 = parameters[204];
                let v137 = parameters[335];
                let v140 = parameters[336];
                let v143 = parameters[337];
                let v145 = parameters[57];
                let v147 = parameters[338];
                let v150 = parameters[339];
                let v153 = parameters[340];
                let v155 = parameters[58];
                let v157 = parameters[341];
                let v160 = parameters[342];
                let v163 = parameters[343];
                let v165 = parameters[51];
                let v167 = parameters[344];
                let v170 = parameters[345];
                let v173 = parameters[346];
                let v175 = parameters[50];
                let v177 = parameters[347];
                let v180 = parameters[348];
                let v183 = parameters[349];
                let v185 = parameters[63];
                let v187 = parameters[350];
                let v190 = parameters[351];
                let v193 = parameters[352];
                let v195 = parameters[64];
                let v197 = parameters[353];
                let v200 = parameters[354];
                let v203 = parameters[355];
                let v205 = parameters[65];
                let v207 = parameters[356];
                let v210 = parameters[357];
                let v213 = parameters[358];
                let v215 = parameters[68];
                let v217 = parameters[359];
                let v220 = parameters[360];
                let v223 = parameters[361];
                let v225 = parameters[276];
                let v227 = parameters[362];
                let v230 = parameters[363];
                let v233 = parameters[751];
                let v235 = parameters[291];
                let v237 = parameters[752];
                let v240 = parameters[753];
                let v243 = parameters[757];
                let v245 = parameters[294];
                let v247 = parameters[758];
                let v250 = parameters[759];
                let v253 = parameters[754];
                let v255 = parameters[293];
                let v257 = parameters[755];
                let v260 = parameters[756];
                let v264 = 1e0f64;
                let v267 = parameters[364];
                let v269 = parameters[277];
                let v271 = parameters[365];
                let v274 = parameters[366];
                let v277 = parameters[367];
                let v279 = parameters[278];
                let v281 = parameters[368];
                let v284 = parameters[369];
                let v287 = parameters[370];
                let v289 = parameters[275];
                let v291 = parameters[371];
                let v294 = parameters[372];
                let v297 = parameters[373];
                let v299 = parameters[272];
                let v301 = parameters[374];
                let v304 = parameters[375];
                let v307 = parameters[376];
                let v309 = parameters[273];
                let v311 = parameters[377];
                let v314 = parameters[378];
                let v317 = parameters[379];
                let v319 = parameters[274];
                let v321 = parameters[380];
                let v324 = parameters[381];
                let v327 = parameters[382];
                let v329 = parameters[283];
                let v331 = parameters[383];
                let v334 = parameters[384];
                let v341 = parameters[385];
                let v343 = parameters[284];
                let v345 = parameters[386];
                let v348 = parameters[387];
                let v351 = parameters[388];
                let v353 = parameters[285];
                let v355 = parameters[389];
                let v358 = parameters[390];
                let v361 = parameters[391];
                let v363 = parameters[282];
                let v365 = parameters[392];
                let v368 = parameters[393];
                let v371 = parameters[394];
                let v373 = parameters[279];
                let v375 = parameters[395];
                let v378 = parameters[396];
                let v381 = parameters[397];
                let v383 = parameters[280];
                let v385 = parameters[398];
                let v388 = parameters[399];
                let v391 = parameters[400];
                let v393 = parameters[281];
                let v395 = parameters[401];
                let v398 = parameters[402];
                let v401 = parameters[403];
                let v403 = parameters[71];
                let v405 = parameters[404];
                let v408 = parameters[405];
                let v411 = parameters[406];
                let v413 = parameters[72];
                let v415 = parameters[407];
                let v418 = parameters[408];
                let v421 = parameters[409];
                let v423 = parameters[73];
                let v425 = parameters[410];
                let v428 = parameters[411];
                let v431 = parameters[412];
                let v433 = parameters[74];
                let v435 = parameters[413];
                let v438 = parameters[414];
                let v441 = parameters[415];
                let v443 = parameters[75];
                let v445 = parameters[416];
                let v448 = parameters[417];
                let v451 = parameters[418];
                let v453 = parameters[84];
                let v455 = parameters[419];
                let v458 = parameters[420];
                let v461 = parameters[421];
                let v463 = parameters[76];
                let v465 = parameters[422];
                let v468 = parameters[423];
                let v471 = parameters[430];
                let v473 = parameters[87];
                let v475 = parameters[431];
                let v478 = parameters[432];
                let v481 = parameters[433];
                let v483 = parameters[88];
                let v485 = parameters[434];
                let v488 = parameters[435];
                let v491 = parameters[436];
                let v493 = parameters[61];
                let v495 = parameters[437];
                let v498 = parameters[438];
                let v501 = parameters[439];
                let v503 = parameters[62];
                let v505 = parameters[440];
                let v508 = parameters[441];
                let v511 = parameters[424];
                let v513 = parameters[85];
                let v515 = parameters[425];
                let v518 = parameters[426];
                let v521 = parameters[427];
                let v523 = parameters[86];
                let v525 = parameters[428];
                let v528 = parameters[429];
                let v531 = parameters[460];
                let v533 = parameters[113];
                let v535 = parameters[461];
                let v538 = parameters[462];
                let v541 = parameters[442];
                let v543 = parameters[89];
                let v545 = parameters[443];
                let v548 = parameters[444];
                let v551 = parameters[445];
                let v553 = parameters[90];
                let v555 = parameters[446];
                let v558 = parameters[447];
                let v561 = parameters[448];
                let v563 = parameters[91];
                let v565 = parameters[449];
                let v568 = parameters[450];
                let v571 = parameters[451];
                let v573 = parameters[92];
                let v575 = parameters[452];
                let v578 = parameters[453];
                let v581 = parameters[454];
                let v583 = parameters[93];
                let v585 = parameters[455];
                let v588 = parameters[456];
                let v591 = parameters[457];
                let v593 = parameters[94];
                let v595 = parameters[458];
                let v598 = parameters[459];
                let v601 = parameters[463];
                let v603 = parameters[116];
                let v605 = parameters[464];
                let v608 = parameters[465];
                let v611 = parameters[466];
                let v613 = parameters[123];
                let v615 = parameters[467];
                let v618 = parameters[468];
                let v621 = parameters[469];
                let v623 = parameters[124];
                let v625 = parameters[470];
                let v628 = parameters[471];
                let v631 = parameters[472];
                let v633 = parameters[122];
                let v635 = parameters[473];
                let v638 = parameters[474];
                let v641 = parameters[475];
                let v643 = parameters[135];
                let v645 = parameters[476];
                let v648 = parameters[477];
                let v651 = parameters[478];
                let v653 = parameters[139];
                let v655 = parameters[479];
                let v658 = parameters[480];
                let v661 = parameters[481];
                let v663 = parameters[145];
                let v665 = parameters[482];
                let v668 = parameters[483];
                let v671 = parameters[484];
                let v673 = parameters[148];
                let v675 = parameters[485];
                let v678 = parameters[486];
                let v681 = parameters[487];
                let v683 = parameters[155];
                let v685 = parameters[488];
                let v688 = parameters[489];
                let v691 = parameters[490];
                let v693 = parameters[142];
                let v695 = parameters[491];
                let v698 = parameters[492];
                let v701 = parameters[493];
                let v703 = parameters[163];
                let v705 = parameters[494];
                let v708 = parameters[495];
                let v711 = parameters[496];
                let v713 = parameters[157];
                let v715 = parameters[497];
                let v718 = parameters[498];
                let v721 = parameters[499];
                let v723 = parameters[156];
                let v725 = parameters[500];
                let v728 = parameters[501];
                let v731 = parameters[502];
                let v733 = parameters[158];
                let v735 = parameters[503];
                let v738 = parameters[504];
                let v741 = parameters[505];
                let v743 = parameters[160];
                let v745 = parameters[506];
                let v748 = parameters[507];
                let v751 = parameters[508];
                let v753 = parameters[161];
                let v755 = parameters[509];
                let v758 = parameters[510];
                let v761 = parameters[511];
                let v763 = parameters[136];
                let v765 = parameters[512];
                let v768 = parameters[513];
                let v771 = parameters[514];
                let v773 = parameters[166];
                let v775 = parameters[515];
                let v778 = parameters[516];
                let v781 = parameters[517];
                let v783 = parameters[167];
                let v785 = parameters[518];
                let v788 = parameters[519];
                let v791 = parameters[520];
                let v793 = parameters[173];
                let v795 = parameters[521];
                let v798 = parameters[522];
                let v801 = parameters[523];
                let v803 = parameters[176];
                let v805 = parameters[524];
                let v808 = parameters[525];
                let v811 = parameters[526];
                let v813 = parameters[182];
                let v815 = parameters[527];
                let v818 = parameters[528];
                let v821 = parameters[529];
                let v823 = parameters[170];
                let v825 = parameters[530];
                let v828 = parameters[531];
                let v831 = parameters[532];
                let v833 = parameters[183];
                let v835 = parameters[533];
                let v838 = parameters[534];
                let v841 = parameters[535];
                let v843 = parameters[186];
                let v845 = parameters[536];
                let v848 = parameters[537];
                let v851 = parameters[538];
                let v853 = parameters[119];
                let v855 = parameters[539];
                let v858 = parameters[540];
                let v861 = parameters[541];
                let v863 = parameters[130];
                let v865 = parameters[542];
                let v868 = parameters[543];
                let v871 = parameters[544];
                let v873 = parameters[205];
                let v875 = parameters[545];
                let v878 = parameters[546];
                let v881 = parameters[547];
                let v883 = parameters[305];
                let v885 = parameters[548];
                let v888 = parameters[549];
                let v891 = parameters[550];
                let v893 = parameters[306];
                let v895 = parameters[551];
                let v898 = parameters[552];
                let v901 = parameters[553];
                let v903 = parameters[307];
                let v905 = parameters[554];
                let v908 = parameters[555];
                let v911 = parameters[556];
                let v913 = parameters[308];
                let v915 = parameters[557];
                let v918 = parameters[558];
                let v921 = parameters[559];
                let v923 = parameters[210];
                let v925 = parameters[560];
                let v928 = parameters[561];
                let v931 = parameters[562];
                let v933 = parameters[214];
                let v935 = parameters[563];
                let v938 = parameters[564];
                let v941 = parameters[565];
                let v943 = parameters[208];
                let v945 = parameters[566];
                let v948 = parameters[567];
                let v951 = parameters[568];
                let v953 = parameters[206];
                let v955 = parameters[569];
                let v958 = parameters[570];
                let v961 = parameters[571];
                let v963 = parameters[207];
                let v965 = parameters[572];
                let v968 = parameters[573];
                let v971 = parameters[574];
                let v973 = parameters[209];
                let v975 = parameters[575];
                let v978 = parameters[576];
                let v981 = parameters[577];
                let v983 = parameters[256];
                let v985 = parameters[578];
                let v988 = parameters[579];
                let v991 = parameters[580];
                let v993 = parameters[257];
                let v995 = parameters[581];
                let v998 = parameters[582];
                let v1001 = parameters[583];
                let v1003 = parameters[258];
                let v1005 = parameters[584];
                let v1008 = parameters[585];
                let v1011 = parameters[706];
                let v1013 = parameters[217];
                let v1015 = parameters[707];
                let v1018 = parameters[708];
                let v1021 = parameters[709];
                let v1023 = parameters[218];
                let v1025 = parameters[710];
                let v1028 = parameters[711];
                let v1031 = parameters[712];
                let v1033 = parameters[219];
                let v1035 = parameters[713];
                let v1038 = parameters[714];
                let v1041 = parameters[715];
                let v1043 = parameters[220];
                let v1045 = parameters[716];
                let v1048 = parameters[717];
                let v1051 = parameters[718];
                let v1053 = parameters[221];
                let v1055 = parameters[719];
                let v1058 = parameters[720];
                let v1061 = parameters[721];
                let v1063 = parameters[222];
                let v1065 = parameters[722];
                let v1068 = parameters[723];
                let v1071 = parameters[724];
                let v1073 = parameters[223];
                let v1075 = parameters[725];
                let v1078 = parameters[726];
                let v1081 = parameters[727];
                let v1083 = parameters[224];
                let v1085 = parameters[728];
                let v1088 = parameters[729];
                let v1091 = parameters[730];
                let v1093 = parameters[225];
                let v1095 = parameters[731];
                let v1098 = parameters[732];
                let v1101 = parameters[586];
                let v1103 = parameters[226];
                let v1105 = parameters[587];
                let v1108 = parameters[588];
                let v1111 = parameters[589];
                let v1113 = parameters[227];
                let v1115 = parameters[590];
                let v1118 = parameters[591];
                let v1121 = parameters[592];
                let v1123 = parameters[228];
                let v1125 = parameters[593];
                let v1128 = parameters[594];
                let v1131 = parameters[595];
                let v1133 = parameters[230];
                let v1135 = parameters[596];
                let v1138 = parameters[597];
                let v1141 = parameters[598];
                let v1143 = parameters[229];
                let v1145 = parameters[599];
                let v1148 = parameters[600];
                let v1151 = parameters[610];
                let v1153 = parameters[247];
                let v1155 = parameters[611];
                let v1158 = parameters[612];
                let v1161 = parameters[619];
                let v1163 = parameters[250];
                let v1165 = parameters[620];
                let v1168 = parameters[621];
                let v1171 = parameters[622];
                let v1173 = parameters[251];
                let v1175 = parameters[623];
                let v1178 = parameters[624];
                let v1181 = parameters[625];
                let v1183 = parameters[252];
                let v1185 = parameters[626];
                let v1188 = parameters[627];
                let v1191 = parameters[628];
                let v1193 = parameters[253];
                let v1195 = parameters[629];
                let v1198 = parameters[630];
                let v1201 = parameters[601];
                let v1203 = parameters[244];
                let v1205 = parameters[602];
                let v1208 = parameters[603];
                let v1211 = parameters[604];
                let v1213 = parameters[245];
                let v1215 = parameters[605];
                let v1218 = parameters[606];
                let v1221 = parameters[607];
                let v1223 = parameters[246];
                let v1225 = parameters[608];
                let v1228 = parameters[609];
                let v1231 = parameters[613];
                let v1233 = parameters[248];
                let v1235 = parameters[614];
                let v1238 = parameters[615];
                let v1241 = parameters[631];
                let v1243 = parameters[254];
                let v1245 = parameters[632];
                let v1248 = parameters[633];
                let v1251 = parameters[616];
                let v1253 = parameters[249];
                let v1255 = parameters[617];
                let v1258 = parameters[618];
                let v1261 = parameters[634];
                let v1263 = parameters[255];
                let v1265 = parameters[635];
                let v1268 = parameters[636];
                let v1271 = parameters[637];
                let v1273 = parameters[231];
                let v1275 = parameters[638];
                let v1278 = parameters[639];
                let v1281 = parameters[643];
                let v1283 = parameters[232];
                let v1285 = parameters[644];
                let v1288 = parameters[645];
                let v1291 = parameters[649];
                let v1293 = parameters[233];
                let v1295 = parameters[650];
                let v1298 = parameters[651];
                let v1301 = parameters[655];
                let v1303 = parameters[242];
                let v1305 = parameters[656];
                let v1308 = parameters[657];
                let v1311 = parameters[640];
                let v1313 = parameters[236];
                let v1315 = parameters[641];
                let v1318 = parameters[642];
                let v1321 = parameters[646];
                let v1323 = parameters[237];
                let v1325 = parameters[647];
                let v1328 = parameters[648];
                let v1331 = parameters[652];
                let v1333 = parameters[238];
                let v1335 = parameters[653];
                let v1338 = parameters[654];
                let v1341 = parameters[658];
                let v1343 = parameters[243];
                let v1345 = parameters[659];
                let v1348 = parameters[660];
                let v1351 = parameters[661];
                let v1353 = parameters[240];
                let v1355 = parameters[662];
                let v1358 = parameters[663];
                let v1361 = parameters[664];
                let v1363 = parameters[241];
                let v1365 = parameters[665];
                let v1368 = parameters[666];
                let v1371 = parameters[667];
                let v1373 = parameters[259];
                let v1375 = parameters[668];
                let v1378 = parameters[669];
                let v1381 = parameters[670];
                let v1383 = parameters[260];
                let v1385 = parameters[671];
                let v1388 = parameters[672];
                let v1391 = parameters[673];
                let v1393 = parameters[261];
                let v1395 = parameters[674];
                let v1398 = parameters[675];
                let v1401 = parameters[676];
                let v1403 = parameters[262];
                let v1405 = parameters[677];
                let v1408 = parameters[678];
                let v1411 = parameters[679];
                let v1413 = parameters[100];
                let v1415 = parameters[680];
                let v1418 = parameters[681];
                let v1421 = parameters[682];
                let v1423 = parameters[129];
                let v1425 = parameters[683];
                let v1428 = parameters[684];
                let v1431 = parameters[685];
                let v1433 = parameters[103];
                let v1435 = parameters[686];
                let v1438 = parameters[687];
                let v1441 = parameters[688];
                let v1443 = parameters[106];
                let v1445 = parameters[689];
                let v1448 = parameters[690];
                let v1451 = parameters[691];
                let v1453 = parameters[110];
                let v1455 = parameters[692];
                let v1458 = parameters[693];
                let v1461 = parameters[694];
                let v1463 = parameters[111];
                let v1465 = parameters[695];
                let v1468 = parameters[696];
                let v1471 = parameters[697];
                let v1473 = parameters[112];
                let v1475 = parameters[698];
                let v1478 = parameters[699];
                let v1481 = parameters[700];
                let v1483 = parameters[137];
                let v1485 = parameters[701];
                let v1488 = parameters[702];
                let v1491 = parameters[703];
                let v1493 = parameters[187];
                let v1495 = parameters[704];
                let v1498 = parameters[705];
                let v1501 = parameters[739];
                let v1503 = parameters[95];
                let v1505 = parameters[740];
                let v1508 = parameters[741];
                let v1511 = parameters[742];
                let v1513 = parameters[96];
                let v1515 = parameters[743];
                let v1518 = parameters[744];
                let v1521 = parameters[745];
                let v1523 = parameters[97];
                let v1525 = parameters[746];
                let v1528 = parameters[747];
                let v1531 = parameters[748];
                let v1533 = parameters[98];
                let v1535 = parameters[749];
                let v1538 = parameters[750];
                let v1541 = staged[283];
                let v1543 = parameters[733];
                let v1545 = parameters[317];
                let v1547 = parameters[734];
                let v1550 = parameters[735];
                let v1553 = parameters[736];
                let v1555 = parameters[318];
                let v1557 = parameters[737];
                let v1560 = parameters[738];
                let v1565 = staged[285];
                let v1566 = staged[4];
                let v1575 = parameters[141];
                let v1578 = parameters[140];
                let v1581 = parameters[147];
                let v1584 = parameters[146];
                let v1587 = parameters[153];
                let v1590 = parameters[152];
                let v1592 = parameters[151];
                let v1594 = parameters[150];
                let v1597 = parameters[149];
                let v1600 = parameters[144];
                let v1603 = parameters[143];
                let v1606 = parameters[165];
                let v1609 = parameters[164];
                let v1612 = staged[286];
                let v1613 = staged[5];
                let v1621 = parameters[169];
                let v1624 = parameters[168];
                let v1627 = parameters[175];
                let v1630 = parameters[174];
                let v1633 = parameters[181];
                let v1636 = parameters[180];
                let v1638 = parameters[179];
                let v1640 = parameters[178];
                let v1643 = parameters[177];
                let v1646 = parameters[172];
                let v1649 = parameters[171];
                let v1652 = parameters[185];
                let v1655 = parameters[184];
                let v1658 = staged[287];
                let v1659 = parameters[197];
                let v1662 = parameters[196];
                let v1665 = parameters[201];
                let v1668 = parameters[200];
                let v1671 = parameters[193];
                let v1674 = parameters[192];
                let v1680 = parameters[212];
                let v1683 = parameters[211];
                let v1686 = 1e6f64;
                let v1688 = staged[6];
                let v1690 = parameters[114];
                let v1693 = parameters[118];
                let v1696 = parameters[117];
                let v1699 = parameters[126];
                let v1702 = parameters[125];
                let v1705 = parameters[128];
                let v1708 = parameters[127];
                let v1711 = parameters[102];
                let v1714 = parameters[101];
                let v1717 = parameters[133];
                let v1720 = parameters[132];
                let v1723 = parameters[105];
                let v1726 = parameters[104];
                let v1729 = parameters[108];
                let v1732 = parameters[107];
                let v1735 = parameters[80];
                let v1738 = parameters[79];
                let v1740 = parameters[77];
                let v1742 = parameters[82];
                let v1745 = parameters[81];
                let v1747 = parameters[78];
                let v1750 = 3e-2f64;
                let v1782 = staged[7];
                let v1783 = parameters[3];
                let v1785 = parameters[4];
                let v1787 = parameters[5];
                let v1790 = staged[8];
                let v1793 = parameters[6];
                let v1798 = 1e-20f64;
                let v1801 = 5e-1f64;
                let v1804 = staged[307];
                let v1805 = 3.333333333333333e-1f64;
                let v1807 = 3.333333333333333e-1f64;
                let v1816 = parameters[296];
                let v1819 = staged[309];
                let v1821 = parameters[312];
                let v1823 = parameters[310];
                let v1825 = parameters[311];
                let v1829 = parameters[215];
                let v1830 = parameters[7];
                let v1832 = parameters[216];
                let v1833 = parameters[8];
                let v1835 = 1e-3f64;
                let v1849 = parameters[99];
                let v1852 = staged[9];
                let v1855 = staged[10];
                let v1857 = parameters[239];
                let v1859 = 1e-38f64;
                let v1865 = staged[11];
                let v1868 = 3e0f64;
                let v1870 = parameters[315];
                let v1872 = parameters[313];
                let v1874 = parameters[316];
                let v1877 = parameters[314];
                let v1883 = parameters[19];
                let v1886 = 1e3f64;
                let v1888 = staged[322];
                let v1889 = staged[323];
                let v1890 = 1e0f64;
                let v1894 = parameters[120];
                let v1898 = staged[17];
                let v1900 = staged[19];
                let v1902 = parameters[131];
                let v1907 = 4e0f64;
                let v1910 = parameters[302];
                let v1912 = parameters[301];
                let v1918 = staged[325];
                let v1929 = 1e-2f64;
                let v1931 = staged[87];
                let v1938 = staged[101];
                let v1940 = 1.60219e-19f64;
                let v1942 = parameters[49];
                let v1944 = staged[103];
                let v1946 = staged[104];
                let v1948 = parameters[304];
                let v1950 = parameters[303];
                let v1953 = staged[327];
                let v1955 = staged[141];
                let v1961 = parameters[213];
                let v1965 = staged[334];
                let v1972 = staged[182];
                let v1973 = 3.9e0f64;
                let v1975 = parameters[60];
                let v1982 = staged[22];
                let v1984 = parameters[263];
                let v1986 = parameters[264];
                let v1996 = staged[339];
                let v1998 = 3.75956e-7f64;
                let v2001 = staged[340];
                let v2002 = 4.97232e-7f64;
                let v2008 = staged[86];
                let v2010 = staged[217];
                let v2012 = staged[341];
                let v2015 = staged[342];
                let v2021 = 1e10f64;
                let v2029 = staged[253];
                let v2032 = 0e0f64;
                let mut out46: f64 = 0.0;
                let mut out50: f64 = 0.0;
                let mut out74: f64 = 0.0;
                let mut out78: f64 = 0.0;
                let mut out265: f64 = 0.0;
                let mut out339: f64 = 0.0;
                let mut out1843: f64 = 0.0;
                let mut out1845: f64 = 0.0;
                let mut out1847: f64 = 0.0;
                let mut out1884: f64 = 0.0;
                let mut out1899: f64 = 0.0;
                let mut out1901: f64 = 0.0;
                let mut out1919: f64 = 0.0;
                let mut out1920: f64 = 0.0;
                let mut out1925: f64 = 0.0;
                let mut out1926: f64 = 0.0;
                let mut out1956: f64 = 0.0;
                let mut out1962: f64 = 0.0;
                let mut out1964: f64 = 0.0;
                let mut out1968: f64 = 0.0;
                let mut out1969: f64 = 0.0;
                let mut out1970: f64 = 0.0;
                let mut out1971: f64 = 0.0;
                let mut out1976: f64 = 0.0;
                let mut out2000: f64 = 0.0;
                let mut out2004: f64 = 0.0;
                let mut out2007: f64 = 0.0;
                let mut out2009: f64 = 0.0;
                let mut out2011: f64 = 0.0;
                let mut out2013: f64 = 0.0;
                let mut out2014: f64 = 0.0;
                let mut out2016: f64 = 0.0;
                let mut out2019: f64 = 0.0;
                let mut out2024: f64 = 0.0;
                let mut out2027: f64 = 0.0;
                let mut out2031: f64 = 0.0;
                let v4: f64;
                if v0 != 0.0 {
                    let v3 = v1 / v2;
                    v4 = v3;
                } else {
                    v4 = v1;
                }
                let v7 = v5 + v6;
                let v9 = v4 + v8;
                let v11 = v7.powf(v10);
                let v13 = v9.powf(v12);
                let v14 = v11 * v13;
                let v26 = v7.powf(v25);
                let v28 = v9.powf(v27);
                let v29 = v26 * v28;
                let v39 = ((v32 + (v30 * v26)) + (v34 * v28)) + (v37 * v29);
                let v42 = v7 - (v40 * (((v17 + (v15 * v11)) + (v19 * v13)) + (v22 * v14)));
                let v44 = if v42 <= v43 { 1.0 } else { 0.0 };
                if v44 != 0.0 {
                } else {
                    let v46 = if v42 <= v45 { 1.0 } else { 0.0 };
                    out46 = v46;
                }
                let v48 = v9 - (v40 * v39);
                let v49 = if v48 <= v43 { 1.0 } else { 0.0 };
                if v49 != 0.0 {
                } else {
                    let v50 = if v48 <= v45 { 1.0 } else { 0.0 };
                    out50 = v50;
                }
                let v70 = ((v63 + (v61 * v26)) + (v65 * v28)) + (v68 * v29);
                let v72 = v7 - (v40 * (((v53 + (v51 * v11)) + (v55 * v13)) + (v58 * v14)));
                let v73 = if v72 <= v43 { 1.0 } else { 0.0 };
                if v73 != 0.0 {
                } else {
                    let v74 = if v72 <= v45 { 1.0 } else { 0.0 };
                    out74 = v74;
                }
                let v76 = v9 - (v40 * v70);
                let v77 = if v76 <= v43 { 1.0 } else { 0.0 };
                if v77 != 0.0 {
                } else {
                    let v78 = if v76 <= v45 { 1.0 } else { 0.0 };
                    out78 = v78;
                }
                let v80 = v79 / v42;
                let v81 = v79 / v48;
                let v82 = v80 * v81;
                let v92 = ((v85 + (v83 * v80)) + (v87 * v81)) + (v90 * v82);
                let v102 = ((v95 + (v93 * v80)) + (v97 * v81)) + (v100 * v82);
                let v112 = ((v105 + (v103 * v80)) + (v107 * v81)) + (v110 * v82);
                let v122 = ((v115 + (v113 * v80)) + (v117 * v81)) + (v120 * v82);
                let v132 = ((v125 + (v123 * v80)) + (v127 * v81)) + (v130 * v82);
                let v142 = ((v135 + (v133 * v80)) + (v137 * v81)) + (v140 * v82);
                let v152 = ((v145 + (v143 * v80)) + (v147 * v81)) + (v150 * v82);
                let v162 = ((v155 + (v153 * v80)) + (v157 * v81)) + (v160 * v82);
                let v172 = ((v165 + (v163 * v80)) + (v167 * v81)) + (v170 * v82);
                let v182 = ((v175 + (v173 * v80)) + (v177 * v81)) + (v180 * v82);
                let v192 = ((v185 + (v183 * v80)) + (v187 * v81)) + (v190 * v82);
                let v202 = ((v195 + (v193 * v80)) + (v197 * v81)) + (v200 * v82);
                let v212 = ((v205 + (v203 * v80)) + (v207 * v81)) + (v210 * v82);
                let v222 = ((v215 + (v213 * v80)) + (v217 * v81)) + (v220 * v82);
                let v232 = ((v225 + (v223 * v80)) + (v227 * v81)) + (v230 * v82);
                let v242 = ((v235 + (v233 * v80)) + (v237 * v81)) + (v240 * v82);
                let v252 = ((v245 + (v243 * v80)) + (v247 * v81)) + (v250 * v82);
                let v262 = ((v255 + (v253 * v80)) + (v257 * v81)) + (v260 * v82);
                let v263 = if v232 < v43 { 1.0 } else { 0.0 };
                let v266: f64;
                if v263 != 0.0 {
                    v266 = v43;
                } else {
                    let v265 = if v232 > v264 { 1.0 } else { 0.0 };
                    out265 = v265;
                    let v338: f64;
                    if v265 != 0.0 {
                        v338 = v264;
                    } else {
                        v338 = v232;
                    }
                    v266 = v338;
                }
                let v276 = ((v269 + (v267 * v80)) + (v271 * v81)) + (v274 * v82);
                let v286 = ((v279 + (v277 * v80)) + (v281 * v81)) + (v284 * v82);
                let v296 = ((v289 + (v287 * v80)) + (v291 * v81)) + (v294 * v82);
                let v306 = ((v299 + (v297 * v80)) + (v301 * v81)) + (v304 * v82);
                let v316 = ((v309 + (v307 * v80)) + (v311 * v81)) + (v314 * v82);
                let v326 = ((v319 + (v317 * v80)) + (v321 * v81)) + (v324 * v82);
                let v336 = ((v329 + (v327 * v80)) + (v331 * v81)) + (v334 * v82);
                let v337 = if v336 < v43 { 1.0 } else { 0.0 };
                let v340: f64;
                if v337 != 0.0 {
                    v340 = v43;
                } else {
                    let v339 = if v336 > v264 { 1.0 } else { 0.0 };
                    out339 = v339;
                    let v1542: f64;
                    if v339 != 0.0 {
                        v1542 = v264;
                    } else {
                        v1542 = v336;
                    }
                    v340 = v1542;
                }
                let v350 = ((v343 + (v341 * v80)) + (v345 * v81)) + (v348 * v82);
                let v360 = ((v353 + (v351 * v80)) + (v355 * v81)) + (v358 * v82);
                let v370 = ((v363 + (v361 * v80)) + (v365 * v81)) + (v368 * v82);
                let v380 = ((v373 + (v371 * v80)) + (v375 * v81)) + (v378 * v82);
                let v390 = ((v383 + (v381 * v80)) + (v385 * v81)) + (v388 * v82);
                let v400 = ((v393 + (v391 * v80)) + (v395 * v81)) + (v398 * v82);
                let v410 = ((v403 + (v401 * v80)) + (v405 * v81)) + (v408 * v82);
                let v420 = ((v413 + (v411 * v80)) + (v415 * v81)) + (v418 * v82);
                let v430 = ((v423 + (v421 * v80)) + (v425 * v81)) + (v428 * v82);
                let v440 = ((v433 + (v431 * v80)) + (v435 * v81)) + (v438 * v82);
                let v450 = ((v443 + (v441 * v80)) + (v445 * v81)) + (v448 * v82);
                let v460 = ((v453 + (v451 * v80)) + (v455 * v81)) + (v458 * v82);
                let v470 = ((v463 + (v461 * v80)) + (v465 * v81)) + (v468 * v82);
                let v480 = ((v473 + (v471 * v80)) + (v475 * v81)) + (v478 * v82);
                let v490 = ((v483 + (v481 * v80)) + (v485 * v81)) + (v488 * v82);
                let v500 = ((v493 + (v491 * v80)) + (v495 * v81)) + (v498 * v82);
                let v510 = ((v503 + (v501 * v80)) + (v505 * v81)) + (v508 * v82);
                let v520 = ((v513 + (v511 * v80)) + (v515 * v81)) + (v518 * v82);
                let v530 = ((v523 + (v521 * v80)) + (v525 * v81)) + (v528 * v82);
                let v540 = ((v533 + (v531 * v80)) + (v535 * v81)) + (v538 * v82);
                let v550 = ((v543 + (v541 * v80)) + (v545 * v81)) + (v548 * v82);
                let v560 = ((v553 + (v551 * v80)) + (v555 * v81)) + (v558 * v82);
                let v570 = ((v563 + (v561 * v80)) + (v565 * v81)) + (v568 * v82);
                let v580 = ((v573 + (v571 * v80)) + (v575 * v81)) + (v578 * v82);
                let v590 = ((v583 + (v581 * v80)) + (v585 * v81)) + (v588 * v82);
                let v600 = ((v593 + (v591 * v80)) + (v595 * v81)) + (v598 * v82);
                let v610 = ((v603 + (v601 * v80)) + (v605 * v81)) + (v608 * v82);
                let v620 = ((v613 + (v611 * v80)) + (v615 * v81)) + (v618 * v82);
                let v630 = ((v623 + (v621 * v80)) + (v625 * v81)) + (v628 * v82);
                let v640 = ((v633 + (v631 * v80)) + (v635 * v81)) + (v638 * v82);
                let v650 = ((v643 + (v641 * v80)) + (v645 * v81)) + (v648 * v82);
                let v660 = ((v653 + (v651 * v80)) + (v655 * v81)) + (v658 * v82);
                let v670 = ((v663 + (v661 * v80)) + (v665 * v81)) + (v668 * v82);
                let v680 = ((v673 + (v671 * v80)) + (v675 * v81)) + (v678 * v82);
                let v690 = ((v683 + (v681 * v80)) + (v685 * v81)) + (v688 * v82);
                let v700 = ((v693 + (v691 * v80)) + (v695 * v81)) + (v698 * v82);
                let v710 = ((v703 + (v701 * v80)) + (v705 * v81)) + (v708 * v82);
                let v720 = ((v713 + (v711 * v80)) + (v715 * v81)) + (v718 * v82);
                let v730 = ((v723 + (v721 * v80)) + (v725 * v81)) + (v728 * v82);
                let v740 = ((v733 + (v731 * v80)) + (v735 * v81)) + (v738 * v82);
                let v750 = ((v743 + (v741 * v80)) + (v745 * v81)) + (v748 * v82);
                let v760 = ((v753 + (v751 * v80)) + (v755 * v81)) + (v758 * v82);
                let v770 = ((v763 + (v761 * v80)) + (v765 * v81)) + (v768 * v82);
                let v780 = ((v773 + (v771 * v80)) + (v775 * v81)) + (v778 * v82);
                let v790 = ((v783 + (v781 * v80)) + (v785 * v81)) + (v788 * v82);
                let v800 = ((v793 + (v791 * v80)) + (v795 * v81)) + (v798 * v82);
                let v810 = ((v803 + (v801 * v80)) + (v805 * v81)) + (v808 * v82);
                let v820 = ((v813 + (v811 * v80)) + (v815 * v81)) + (v818 * v82);
                let v830 = ((v823 + (v821 * v80)) + (v825 * v81)) + (v828 * v82);
                let v840 = ((v833 + (v831 * v80)) + (v835 * v81)) + (v838 * v82);
                let v850 = ((v843 + (v841 * v80)) + (v845 * v81)) + (v848 * v82);
                let v860 = ((v853 + (v851 * v80)) + (v855 * v81)) + (v858 * v82);
                let v870 = ((v863 + (v861 * v80)) + (v865 * v81)) + (v868 * v82);
                let v880 = ((v873 + (v871 * v80)) + (v875 * v81)) + (v878 * v82);
                let v890 = ((v883 + (v881 * v80)) + (v885 * v81)) + (v888 * v82);
                let v900 = ((v893 + (v891 * v80)) + (v895 * v81)) + (v898 * v82);
                let v910 = ((v903 + (v901 * v80)) + (v905 * v81)) + (v908 * v82);
                let v920 = ((v913 + (v911 * v80)) + (v915 * v81)) + (v918 * v82);
                let v930 = ((v923 + (v921 * v80)) + (v925 * v81)) + (v928 * v82);
                let v940 = ((v933 + (v931 * v80)) + (v935 * v81)) + (v938 * v82);
                let v950 = ((v943 + (v941 * v80)) + (v945 * v81)) + (v948 * v82);
                let v960 = ((v953 + (v951 * v80)) + (v955 * v81)) + (v958 * v82);
                let v970 = ((v963 + (v961 * v80)) + (v965 * v81)) + (v968 * v82);
                let v980 = ((v973 + (v971 * v80)) + (v975 * v81)) + (v978 * v82);
                let v990 = ((v983 + (v981 * v80)) + (v985 * v81)) + (v988 * v82);
                let v1000 = ((v993 + (v991 * v80)) + (v995 * v81)) + (v998 * v82);
                let v1010 = ((v1003 + (v1001 * v80)) + (v1005 * v81)) + (v1008 * v82);
                let v1020 = ((v1013 + (v80 * v1011)) + (v81 * v1015)) + (v82 * v1018);
                let v1030 = ((v1023 + (v80 * v1021)) + (v81 * v1025)) + (v82 * v1028);
                let v1040 = ((v1033 + (v80 * v1031)) + (v81 * v1035)) + (v82 * v1038);
                let v1050 = ((v1043 + (v80 * v1041)) + (v81 * v1045)) + (v82 * v1048);
                let v1060 = ((v1053 + (v80 * v1051)) + (v81 * v1055)) + (v82 * v1058);
                let v1070 = ((v1063 + (v80 * v1061)) + (v81 * v1065)) + (v82 * v1068);
                let v1080 = ((v1073 + (v80 * v1071)) + (v81 * v1075)) + (v82 * v1078);
                let v1090 = ((v1083 + (v80 * v1081)) + (v81 * v1085)) + (v82 * v1088);
                let v1100 = ((v1093 + (v80 * v1091)) + (v81 * v1095)) + (v82 * v1098);
                let v1110 = ((v1103 + (v1101 * v80)) + (v1105 * v81)) + (v1108 * v82);
                let v1120 = ((v1113 + (v1111 * v80)) + (v1115 * v81)) + (v1118 * v82);
                let v1130 = ((v1123 + (v1121 * v80)) + (v1125 * v81)) + (v1128 * v82);
                let v1140 = ((v1133 + (v1131 * v80)) + (v1135 * v81)) + (v1138 * v82);
                let v1150 = ((v1143 + (v1141 * v80)) + (v1145 * v81)) + (v1148 * v82);
                let v1160 = ((v1153 + (v1151 * v80)) + (v1155 * v81)) + (v1158 * v82);
                let v1170 = ((v1163 + (v1161 * v80)) + (v1165 * v81)) + (v1168 * v82);
                let v1180 = ((v1173 + (v1171 * v80)) + (v1175 * v81)) + (v1178 * v82);
                let v1190 = ((v1183 + (v1181 * v80)) + (v1185 * v81)) + (v1188 * v82);
                let v1200 = ((v1193 + (v1191 * v80)) + (v1195 * v81)) + (v1198 * v82);
                let v1210 = ((v1203 + (v1201 * v80)) + (v1205 * v81)) + (v1208 * v82);
                let v1220 = ((v1213 + (v1211 * v80)) + (v1215 * v81)) + (v1218 * v82);
                let v1230 = ((v1223 + (v1221 * v80)) + (v1225 * v81)) + (v1228 * v82);
                let v1240 = ((v1233 + (v1231 * v80)) + (v1235 * v81)) + (v1238 * v82);
                let v1250 = ((v1243 + (v1241 * v80)) + (v1245 * v81)) + (v1248 * v82);
                let v1260 = ((v1253 + (v1251 * v80)) + (v1255 * v81)) + (v1258 * v82);
                let v1270 = ((v1263 + (v1261 * v80)) + (v1265 * v81)) + (v1268 * v82);
                let v1280 = ((v1273 + (v1271 * v80)) + (v1275 * v81)) + (v1278 * v82);
                let v1290 = ((v1283 + (v1281 * v80)) + (v1285 * v81)) + (v1288 * v82);
                let v1300 = ((v1293 + (v1291 * v80)) + (v1295 * v81)) + (v1298 * v82);
                let v1310 = ((v1303 + (v1301 * v80)) + (v1305 * v81)) + (v1308 * v82);
                let v1320 = ((v1313 + (v1311 * v80)) + (v1315 * v81)) + (v1318 * v82);
                let v1330 = ((v1323 + (v1321 * v80)) + (v1325 * v81)) + (v1328 * v82);
                let v1340 = ((v1333 + (v1331 * v80)) + (v1335 * v81)) + (v1338 * v82);
                let v1350 = ((v1343 + (v1341 * v80)) + (v1345 * v81)) + (v1348 * v82);
                let v1360 = ((v1353 + (v1351 * v80)) + (v1355 * v81)) + (v1358 * v82);
                let v1370 = ((v1363 + (v1361 * v80)) + (v1365 * v81)) + (v1368 * v82);
                let v1380 = ((v1373 + (v1371 * v80)) + (v1375 * v81)) + (v1378 * v82);
                let v1390 = ((v1383 + (v1381 * v80)) + (v1385 * v81)) + (v1388 * v82);
                let v1400 = ((v1393 + (v1391 * v80)) + (v1395 * v81)) + (v1398 * v82);
                let v1410 = ((v1403 + (v1401 * v80)) + (v1405 * v81)) + (v1408 * v82);
                let v1420 = ((v1413 + (v1411 * v80)) + (v1415 * v81)) + (v1418 * v82);
                let v1430 = ((v1423 + (v1421 * v80)) + (v1425 * v81)) + (v1428 * v82);
                let v1440 = ((v1433 + (v1431 * v80)) + (v1435 * v81)) + (v1438 * v82);
                let v1450 = ((v1443 + (v1441 * v80)) + (v1445 * v81)) + (v1448 * v82);
                let v1460 = ((v1453 + (v1451 * v80)) + (v1455 * v81)) + (v1458 * v82);
                let v1470 = ((v1463 + (v1461 * v80)) + (v1465 * v81)) + (v1468 * v82);
                let v1480 = ((v1473 + (v1471 * v80)) + (v1475 * v81)) + (v1478 * v82);
                let v1490 = ((v1483 + (v1481 * v80)) + (v1485 * v81)) + (v1488 * v82);
                let v1500 = ((v1493 + (v1491 * v80)) + (v1495 * v81)) + (v1498 * v82);
                let v1510 = ((v1503 + (v1501 * v80)) + (v1505 * v81)) + (v1508 * v82);
                let v1520 = ((v1513 + (v1511 * v80)) + (v1515 * v81)) + (v1518 * v82);
                let v1530 = ((v1523 + (v1521 * v80)) + (v1525 * v81)) + (v1528 * v82);
                let v1540 = ((v1533 + (v1531 * v80)) + (v1535 * v81)) + (v1538 * v82);
                let v1563: f64;
                let v1564: f64;
                if v1541 != 0.0 {
                    let v1552 = ((v1545 + (v1543 * v80)) + (v1547 * v81)) + (v1550 * v82);
                    let v1562 = ((v1555 + (v1553 * v80)) + (v1557 * v81)) + (v1560 * v82);
                    v1563 = v1552;
                    v1564 = v1562;
                } else {
                    v1563 = v43;
                    v1564 = v43;
                }
                let v1573: f64;
                if v1565 != 0.0 {
                    let v1570 = v650 * (v264 - (v1490 * (v42.powf(v1566))));
                    v1573 = v1570;
                } else {
                    let v1572 = v650 * (v264 - v1490);
                    v1573 = v1572;
                }
                let v1574 = -v42;
                let v1580 = v660 + (v1578 * (rspice_limited_exp((v1574 / v1575))));
                let v1586 = v670 + (v1584 * (rspice_limited_exp((v1574 / v1581))));
                let v1593 = v1592 + (v1590 * (rspice_limited_exp((v1574 / v1587))));
                let v1599 = v680 + (v1597 * (rspice_limited_exp((v1574 / v1594))));
                let v1605 = v700 + (v1603 * (rspice_limited_exp((v1574 / v1600))));
                let v1611 = v710 + (v1609 * (rspice_limited_exp((v1574 / v1606))));
                let v1620: f64;
                if v1612 != 0.0 {
                    let v1617 = v780 * (v264 - (v1500 * (v42.powf(v1613))));
                    v1620 = v1617;
                } else {
                    let v1619 = v780 * (v264 - v1500);
                    v1620 = v1619;
                }
                let v1626 = v790 + (v1624 * (rspice_limited_exp((v1574 / v1621))));
                let v1632 = v800 + (v1630 * (rspice_limited_exp((v1574 / v1627))));
                let v1639 = v1638 + (v1636 * (rspice_limited_exp((v1574 / v1633))));
                let v1645 = v810 + (v1643 * (rspice_limited_exp((v1574 / v1640))));
                let v1651 = v830 + (v1649 * (rspice_limited_exp((v1574 / v1646))));
                let v1657 = v840 + (v1655 * (rspice_limited_exp((v1574 / v1652))));
                let v1677: f64;
                let v1678: f64;
                let v1679: f64;
                if v1658 != 0.0 {
                    let v1664 = v112 + (v1662 * (rspice_limited_exp((v1574 / v1659))));
                    let v1670 = v102 + (v1668 * (rspice_limited_exp((v1574 / v1665))));
                    v1677 = v92;
                    v1678 = v1664;
                    v1679 = v1670;
                } else {
                    let v1676 = v92 + (v1674 * (rspice_limited_exp((v1574 / v1671))));
                    v1677 = v1676;
                    v1678 = v112;
                    v1679 = v102;
                }
                let v1685 = v930 + (v1683 * (rspice_limited_exp((v1574 / v1680))));
                let v1692 = v540 + (v1690 * ((v42 * v1686).powf(v1688)));
                let v1698 = v610 + (v1696 * (rspice_limited_exp((v1574 / v1693))));
                let v1704 = v620 + (v1702 * (rspice_limited_exp((v1574 / v1699))));
                let v1710 = v630 + (v1708 * (rspice_limited_exp((v1574 / v1705))));
                let v1716 = v1420 + (v1714 * (rspice_limited_exp((v1574 / v1711))));
                let v1722 = v1430 + (v1720 * (rspice_limited_exp((v1574 / v1717))));
                let v1728 = v1440 + (v1726 * (rspice_limited_exp((v1574 / v1723))));
                let v1734 = v1450 + (v1732 * (rspice_limited_exp((v1574 / v1729))));
                let v1741 = v1740 + (v1738 * (rspice_limited_exp((v1574 / v1735))));
                let v1748 = v1747 + (v1745 * (rspice_limited_exp((v1574 / v1742))));
                let v1749 = if v1573 < v43 { 1.0 } else { 0.0 };
                let v1751: f64;
                if v1749 != 0.0 {
                    v1751 = v1750;
                } else {
                    v1751 = v1573;
                }
                let v1752 = if v1580 < v43 { 1.0 } else { 0.0 };
                let v1753: f64;
                if v1752 != 0.0 {
                    v1753 = v43;
                } else {
                    v1753 = v1580;
                }
                let v1754 = if v1605 < v43 { 1.0 } else { 0.0 };
                let v1755: f64;
                if v1754 != 0.0 {
                    v1755 = v43;
                } else {
                    v1755 = v1605;
                }
                let v1756 = if v1599 < v43 { 1.0 } else { 0.0 };
                let v1757: f64;
                if v1756 != 0.0 {
                    v1757 = v43;
                } else {
                    v1757 = v1599;
                }
                let v1758 = if v690 < v43 { 1.0 } else { 0.0 };
                let v1759: f64;
                if v1758 != 0.0 {
                    v1759 = v43;
                } else {
                    v1759 = v690;
                }
                let v1760 = if v1722 < v43 { 1.0 } else { 0.0 };
                let v1761: f64;
                if v1760 != 0.0 {
                    v1761 = v43;
                } else {
                    v1761 = v1722;
                }
                let v1762 = if v420 <= v43 { 1.0 } else { 0.0 };
                let v1763 = if v470 <= v43 { 1.0 } else { 0.0 };
                let v1764 = if v1677 < v43 { 1.0 } else { 0.0 };
                let v1765: f64;
                if v1764 != 0.0 {
                    v1765 = v43;
                } else {
                    v1765 = v1677;
                }
                let v1766 = if v1678 < v43 { 1.0 } else { 0.0 };
                let v1767: f64;
                if v1766 != 0.0 {
                    v1767 = v43;
                } else {
                    v1767 = v1678;
                }
                let v1768 = if v1679 < v43 { 1.0 } else { 0.0 };
                let v1769: f64;
                if v1768 != 0.0 {
                    v1769 = v43;
                } else {
                    v1769 = v1679;
                }
                let v1770 = if v122 < v43 { 1.0 } else { 0.0 };
                let v1771: f64;
                if v1770 != 0.0 {
                    v1771 = v43;
                } else {
                    v1771 = v122;
                }
                let v1772 = if v960 < v43 { 1.0 } else { 0.0 };
                let v1773 = if v970 < v43 { 1.0 } else { 0.0 };
                let v1774 = if v950 <= v43 { 1.0 } else { 0.0 };
                let v1775 = if v1692 < v40 { 1.0 } else { 0.0 };
                let v1776: f64;
                if v1775 != 0.0 {
                    v1776 = v40;
                } else {
                    v1776 = v1692;
                }
                let v1780 = ((v264 + (v530 / v42)).sqrt()) - v264;
                let v1781 = v264 / v1776;
                let v1799 = if ((v1782 * v1783) + (v1790 * (if (v1787 - v1) >= v43 { (v1787 - v1) } else { v43 }))) >= v1798 { ((v1782 * v1783) + (v1790 * (if (v1787 - v1) >= v43 { (v1787 - v1) } else { v43 }))) } else { v1798 };
                let v1800 = if ((v1782 * v1785) + (v1790 * (if (v1793 - v1) >= v43 { (v1793 - v1) } else { v43 }))) >= v1798 { ((v1782 * v1785) + (v1790 * (if (v1793 - v1) >= v43 { (v1793 - v1) } else { v43 }))) } else { v1798 };
                let v1802 = v1801 * v770;
                let v1803 = v1801 * v850;
                let v1809: f64;
                let v1810: f64;
                if v1804 != 0.0 {
                    let v1806 = v1805 * v770;
                    let v1808 = v1807 * v850;
                    v1809 = v1806;
                    v1810 = v1808;
                } else {
                    v1809 = v1802;
                    v1810 = v1803;
                }
                let v1814 = v264 / (((v48 * v1686).powf(v142)) * v2);
                let v1817 = if v1816 >= (v42 / v40) { 1.0 } else { 0.0 };
                let v1818: f64;
                if v1817 != 0.0 {
                    v1818 = v43;
                } else {
                    v1818 = v1816;
                }
                let v1827: f64;
                let v1828: f64;
                if v1819 != 0.0 {
                    let v1822 = v1821 + (v48 * v2);
                    let v1824 = v1822 / v1823;
                    let v1826 = v1825 * v1822;
                    v1827 = v1824;
                    v1828 = v1826;
                } else {
                    v1827 = v264;
                    v1828 = v43;
                }
                let v1831 = v1829 * v1830;
                let v1834 = v1832 * v1833;
                let v1836 = if v1831 <= v1835 { 1.0 } else { 0.0 };
                let v1837: f64;
                if v1836 != 0.0 {
                    v1837 = v1835;
                } else {
                    v1837 = v1831;
                }
                let v1838 = if v1834 <= v1835 { 1.0 } else { 0.0 };
                let v1839: f64;
                if v1838 != 0.0 {
                    v1839 = v1835;
                } else {
                    v1839 = v1834;
                }
                let v1840: f64;
                let v1841: f64;
                let v1842: f64;
                if v1658 != 0.0 {
                    let v1843 = if v1767 <= v43 { 1.0 } else { 0.0 };
                    out1843 = v1843;
                    let v1844: f64;
                    if v1843 != 0.0 {
                        v1844 = v43;
                    } else {
                        v1844 = v1767;
                    }
                    let v1845 = if v1769 <= v43 { 1.0 } else { 0.0 };
                    out1845 = v1845;
                    let v1846: f64;
                    if v1845 != 0.0 {
                        v1846 = v43;
                    } else {
                        v1846 = v1769;
                    }
                    v1840 = v1765;
                    v1841 = v1844;
                    v1842 = v1846;
                } else {
                    let v1847 = if v1765 <= v43 { 1.0 } else { 0.0 };
                    out1847 = v1847;
                    let v1848: f64;
                    if v1847 != 0.0 {
                        v1848 = v43;
                    } else {
                        v1848 = v1765;
                    }
                    v1840 = v1848;
                    v1841 = v1767;
                    v1842 = v1769;
                }
                let v1850 = v1849 * v1370;
                let v1856 = (rspice_limited_exp((v1360 * v1852))) / v1855;
                let v1867 = (v48 * v1865) * ((rspice_limited_exp((v1360 * ((if (v1857 / v1850) >= v1859 { (v1857 / v1850) } else { v1859 }).ln())))) / (v1850 * v1850));
                let v1880 = (v1874 * (v1872 + ((v48 / v1868) / v1870))) / ((v1870 * v2) * (v7 - v1877));
                let v1881 = if v1880 > v1835 { 1.0 } else { 0.0 };
                let v1885: f64;
                if v1881 != 0.0 {
                    let v1882 = v264 / v1880;
                    v1885 = v1882;
                } else {
                    let v1884 = if v1883 != v43 { 1.0 } else { 0.0 };
                    out1884 = v1884;
                    v1885 = v1886;
                }
                let v1887 = v172 * v182;
                if v1888 != 0.0 {
                    if v1889 != 0.0 {
                        let v1899 = v162 - v1898;
                        out1899 = v1899;
                    } else {
                        let v1901 = v162 + v1900;
                        out1901 = v1901;
                    }
                } else {
                }
                let v1891 = v730 - v1890;
                let v1892 = v750 - v1890;
                let v1893 = v760 - v1890;
                let v1897 = v860 * (v264 + (v80 * v1894));
                let v1905 = v870 * (v264 + (v80 * v1902));
                let v1906 = -v570;
                let v1909 = (v1907 * v1906) * v79;
                let v1913 = v1912 + (v1910 / v42);
                let v1914 = v890 - v1890;
                let v1915 = v420 * v42;
                let v1916 = v470 * v42;
                let v1917 = v950 * v42;
                let v1921: f64;
                let v1922: f64;
                let v1923: f64;
                let v1924: f64;
                if v1918 != 0.0 {
                    let v1919 = v296 * v42;
                    out1919 = v1919;
                    let v1925 = v1801 * v316;
                    out1925 = v1925;
                    v1921 = v326;
                    v1922 = v276;
                    v1923 = v286;
                    v1924 = v266;
                } else {
                    let v1920 = v370 * v42;
                    out1920 = v1920;
                    let v1926 = v1801 * v390;
                    out1926 = v1926;
                    v1921 = v400;
                    v1922 = v350;
                    v1923 = v360;
                    v1924 = v340;
                }
                let v1927 = -v1923;
                let v1930 = (v1907 * v1927) * v1929;
                let v1932 = v1931 * v1924;
                let v1933 = -v410;
                let v1934 = v1748 - v1890;
                let v1937 = (-v480) / (v42 + v490);
                let v1939 = v1938 + v192;
                let v1945 = ((v1940 * v182) * v1942) / v1944;
                let v1947 = v1945 * v1946;
                let v1951 = v1950 + (v1948 / v42);
                let v1952 = v820 - v1890;
                if v1658 != 0.0 {
                } else {
                    if v1953 != 0.0 {
                    } else {
                        let v1956 = (v1837 + v1839) + v1955;
                        out1956 = v1956;
                    }
                }
                let v1957 = v1781 - v1890;
                let v1958 = v1755 - v1890;
                let v1959 = if v980 > v43 { 1.0 } else { 0.0 };
                let v1960 = if v1685 > v43 { 1.0 } else { 0.0 };
                if v1960 != 0.0 {
                    let v1962 = if v1961 < v43 { 1.0 } else { 0.0 };
                    out1962 = v1962;
                    if v1962 != 0.0 {
                        let v1964 = v264 / v1685;
                        out1964 = v1964;
                    } else {
                    }
                } else {
                }
                let v1963 = if v940 > v43 { 1.0 } else { 0.0 };
                if v1658 != 0.0 {
                } else {
                    let v1969: f64;
                    let v1970: f64;
                    if v1965 != 0.0 {
                        let v1968 = (v1837 + v1839) + v1955;
                        out1968 = v1968;
                        v1969 = v43;
                        v1970 = v43;
                    } else {
                        v1969 = v1839;
                        v1970 = v1837;
                    }
                    out1969 = v1969;
                    out1970 = v1970;
                }
                let v1966 = if v1510 > v43 { 1.0 } else { 0.0 };
                if v1966 != 0.0 {
                    let v1971 = v1520 * v1945;
                    out1971 = v1971;
                    let v1976 = (v1972 * v1973) / v1975;
                    out1976 = v1976;
                } else {
                }
                let v1977 = v76 * v72;
                let v1979 = (v76 * v1380) * v1944;
                let v1981 = (v76 * v1390) * v1944;
                let v1983 = v1982 * v76;
                let v1985 = v1983 * v1984;
                let v1987 = v1983 * v1986;
                let v1988 = v76 * v1400;
                let v1989 = v76 * v1410;
                let v1990 = v1982 * v1799;
                let v1991 = v1982 * v1800;
                let v1994 = (v990 + (v1000 * v42)) / v42;
                let v1995 = if v1994 <= v43 { 1.0 } else { 0.0 };
                if v1996 != 0.0 {
                    let v1997 = v48 * v42;
                    let v2000 = (v1997 * v1998) * v1856;
                    out2000 = v2000;
                    let v2004 = (v1997 * v2002) * v1856;
                    out2004 = v2004;
                } else {
                }
                if v2001 != 0.0 {
                    let v2007 = ((v48 * v42) * v1865) * v1856;
                    out2007 = v2007;
                    let v2009 = v1310 * v2008;
                    out2009 = v2009;
                    let v2011 = v2010 * v1370;
                    out2011 = v2011;
                    let v2013 = v1350 * v2008;
                    out2013 = v2013;
                } else {
                }
                if v2012 != 0.0 {
                    let v2014 = if v1210 <= v43 { 1.0 } else { 0.0 };
                    out2014 = v2014;
                    let v2016 = if v1170 <= v43 { 1.0 } else { 0.0 };
                    out2016 = v2016;
                } else {
                }
                if v2015 != 0.0 {
                    let v2018 = v42 - (v40 * v1818);
                    let v2019 = v2018 * v2018;
                    out2019 = v2019;
                    let v2024 = ((v2021 * v2019) * v48) * v2;
                    out2024 = v2024;
                    let v2027 = ((v48 * v2) * v2018) * v2021;
                    out2027 = v2027;
                } else {
                }
                let v2020 = v1982 * v2;
                let v2030 = if v2029 != 0.0 && (if v1563 != v43 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v2030 != 0.0 {
                    let v2031 = v2 * v1563;
                    out2031 = v2031;
                } else {
                }
                let v2033: f64;
                if v2030 != 0.0 {
                    v2033 = v43;
                } else {
                    v2033 = v2032;
                }
            [v42, v44, out46, v48, v49, out50, v72, v73, out74, v77, out78, v132, v152, v162, v172, v182, v192, v202, v212, v222, v242, v252, v262, v263, out265, v306, v337, out339, v380, v430, v440, v450, v460, v500, v510, v520, v550, v560, v570, v580, v590, v600, v640, v720, v730, v740, v750, v760, v820, v880, v890, v900, v910, v920, v940, v960, v970, v980, v1010, v1020, v1030, v1040, v1050, v1060, v1070, v1080, v1090, v1100, v1110, v1120, v1130, v1140, v1150, v1160, v1170, v1180, v1190, v1200, v1210, v1220, v1230, v1240, v1250, v1260, v1270, v1280, v1290, v1300, v1320, v1330, v1340, v1460, v1470, v1480, v1510, v1530, v1540, v1586, v1593, v1611, v1626, v1632, v1639, v1645, v1651, v1657, v1685, v1698, v1704, v1710, v1716, v1728, v1734, v1741, v1748, v1749, v1752, v1754, v1756, v1758, v1760, v1762, v1763, v1764, v1766, v1768, v1770, v1772, v1773, v1774, v1775, v1780, v1776, v1781, v1814, v1817, v1836, v1838, out1843, out1845, out1847, v1867, v1881, out1884, v1887, out1899, out1901, v1751, v1753, v1757, v1759, v1897, v1905, v1761, v1906, v1909, v1913, v1915, v1916, v1917, out1919, out1925, out1920, out1926, v1921, v1922, v1927, v1930, v1932, v1933, v1937, v1939, v1945, v1947, v1951, v1809, v1810, v1755, v1620, v1771, v1840, v1837, v1839, out1956, v1959, v1960, out1962, out1964, v1963, v1841, v1842, out1968, v1966, out1971, out1976, v1977, v1979, v1981, v1985, v1987, v1988, v1989, v1990, v1991, v1994, v1995, out2000, out2004, out2007, out2009, out2011, out2013, out2014, out2016, out2019, out2024, out2027, v2020, v2030, out2031, v1564, out1969, out1970, v1885, v1827, v1828, v2033, v1891, v1892, v1893, v1914, v1934, v1952, v1957, v1958]
        };
        self.canonical_staged[144] = produced[0];
        self.canonical_staged[271] = produced[1];
        self.canonical_staged[272] = produced[2];
        self.canonical_staged[148] = produced[3];
        self.canonical_staged[273] = produced[4];
        self.canonical_staged[274] = produced[5];
        self.canonical_staged[166] = produced[6];
        self.canonical_staged[275] = produced[7];
        self.canonical_staged[276] = produced[8];
        self.canonical_staged[277] = produced[9];
        self.canonical_staged[278] = produced[10];
        self.canonical_staged[171] = produced[11];
        self.canonical_staged[21] = produced[12];
        self.canonical_staged[324] = produced[13];
        self.canonical_staged[23] = produced[14];
        self.canonical_staged[16] = produced[15];
        self.canonical_staged[248] = produced[16];
        self.canonical_staged[100] = produced[17];
        self.canonical_staged[98] = produced[18];
        self.canonical_staged[99] = produced[19];
        self.canonical_staged[245] = produced[20];
        self.canonical_staged[243] = produced[21];
        self.canonical_staged[244] = produced[22];
        self.canonical_staged[279] = produced[23];
        self.canonical_staged[280] = produced[24];
        self.canonical_staged[76] = produced[25];
        self.canonical_staged[281] = produced[26];
        self.canonical_staged[282] = produced[27];
        self.canonical_staged[79] = produced[28];
        self.canonical_staged[89] = produced[29];
        self.canonical_staged[39] = produced[30];
        self.canonical_staged[94] = produced[31];
        self.canonical_staged[93] = produced[32];
        self.canonical_staged[66] = produced[33];
        self.canonical_staged[65] = produced[34];
        self.canonical_staged[90] = produced[35];
        self.canonical_staged[44] = produced[36];
        self.canonical_staged[43] = produced[37];
        self.canonical_staged[48] = produced[38];
        self.canonical_staged[45] = produced[39];
        self.canonical_staged[50] = produced[40];
        self.canonical_staged[49] = produced[41];
        self.canonical_staged[51] = produced[42];
        self.canonical_staged[26] = produced[43];
        self.canonical_staged[24] = produced[44];
        self.canonical_staged[28] = produced[45];
        self.canonical_staged[30] = produced[46];
        self.canonical_staged[32] = produced[47];
        self.canonical_staged[132] = produced[48];
        self.canonical_staged[34] = produced[49];
        self.canonical_staged[54] = produced[50];
        self.canonical_staged[56] = produced[51];
        self.canonical_staged[58] = produced[52];
        self.canonical_staged[60] = produced[53];
        self.canonical_staged[170] = produced[54];
        self.canonical_staged[72] = produced[55];
        self.canonical_staged[73] = produced[56];
        self.canonical_staged[167] = produced[57];
        self.canonical_staged[55] = produced[58];
        self.canonical_staged[203] = produced[59];
        self.canonical_staged[202] = produced[60];
        self.canonical_staged[204] = produced[61];
        self.canonical_staged[200] = produced[62];
        self.canonical_staged[201] = produced[63];
        self.canonical_staged[209] = produced[64];
        self.canonical_staged[208] = produced[65];
        self.canonical_staged[210] = produced[66];
        self.canonical_staged[207] = produced[67];
        self.canonical_staged[215] = produced[68];
        self.canonical_staged[214] = produced[69];
        self.canonical_staged[216] = produced[70];
        self.canonical_staged[213] = produced[71];
        self.canonical_staged[219] = produced[72];
        self.canonical_staged[234] = produced[73];
        self.canonical_staged[241] = produced[74];
        self.canonical_staged[59] = produced[75];
        self.canonical_staged[237] = produced[76];
        self.canonical_staged[240] = produced[77];
        self.canonical_staged[235] = produced[78];
        self.canonical_staged[57] = produced[79];
        self.canonical_staged[230] = produced[80];
        self.canonical_staged[231] = produced[81];
        self.canonical_staged[238] = produced[82];
        self.canonical_staged[232] = produced[83];
        self.canonical_staged[239] = produced[84];
        self.canonical_staged[222] = produced[85];
        self.canonical_staged[221] = produced[86];
        self.canonical_staged[223] = produced[87];
        self.canonical_staged[227] = produced[88];
        self.canonical_staged[226] = produced[89];
        self.canonical_staged[228] = produced[90];
        self.canonical_staged[147] = produced[91];
        self.canonical_staged[146] = produced[92];
        self.canonical_staged[145] = produced[93];
        self.canonical_staged[183] = produced[94];
        self.canonical_staged[180] = produced[95];
        self.canonical_staged[181] = produced[96];
        self.canonical_staged[27] = produced[97];
        self.canonical_staged[158] = produced[98];
        self.canonical_staged[129] = produced[99];
        self.canonical_staged[134] = produced[100];
        self.canonical_staged[133] = produced[101];
        self.canonical_staged[159] = produced[102];
        self.canonical_staged[137] = produced[103];
        self.canonical_staged[136] = produced[104];
        self.canonical_staged[135] = produced[105];
        self.canonical_staged[169] = produced[106];
        self.canonical_staged[52] = produced[107];
        self.canonical_staged[163] = produced[108];
        self.canonical_staged[164] = produced[109];
        self.canonical_staged[36] = produced[110];
        self.canonical_staged[37] = produced[111];
        self.canonical_staged[38] = produced[112];
        self.canonical_staged[95] = produced[113];
        self.canonical_staged[96] = produced[114];
        self.canonical_staged[288] = produced[115];
        self.canonical_staged[289] = produced[116];
        self.canonical_staged[290] = produced[117];
        self.canonical_staged[291] = produced[118];
        self.canonical_staged[292] = produced[119];
        self.canonical_staged[293] = produced[120];
        self.canonical_staged[294] = produced[121];
        self.canonical_staged[295] = produced[122];
        self.canonical_staged[297] = produced[123];
        self.canonical_staged[299] = produced[124];
        self.canonical_staged[301] = produced[125];
        self.canonical_staged[302] = produced[126];
        self.canonical_staged[303] = produced[127];
        self.canonical_staged[304] = produced[128];
        self.canonical_staged[305] = produced[129];
        self.canonical_staged[306] = produced[130];
        self.canonical_staged[91] = produced[131];
        self.canonical_staged[42] = produced[132];
        self.canonical_staged[149] = produced[133];
        self.canonical_staged[142] = produced[134];
        self.canonical_staged[308] = produced[135];
        self.canonical_staged[310] = produced[136];
        self.canonical_staged[311] = produced[137];
        self.canonical_staged[316] = produced[138];
        self.canonical_staged[317] = produced[139];
        self.canonical_staged[318] = produced[140];
        self.canonical_staged[61] = produced[141];
        self.canonical_staged[319] = produced[142];
        self.canonical_staged[320] = produced[143];
        self.canonical_staged[15] = produced[144];
        self.canonical_staged[18] = produced[145];
        self.canonical_staged[20] = produced[146];
        self.canonical_staged[25] = produced[147];
        self.canonical_staged[29] = produced[148];
        self.canonical_staged[31] = produced[149];
        self.canonical_staged[33] = produced[150];
        self.canonical_staged[35] = produced[151];
        self.canonical_staged[40] = produced[152];
        self.canonical_staged[41] = produced[153];
        self.canonical_staged[46] = produced[154];
        self.canonical_staged[47] = produced[155];
        self.canonical_staged[53] = produced[156];
        self.canonical_staged[69] = produced[157];
        self.canonical_staged[70] = produced[158];
        self.canonical_staged[71] = produced[159];
        self.canonical_staged[74] = produced[160];
        self.canonical_staged[75] = produced[161];
        self.canonical_staged[77] = produced[162];
        self.canonical_staged[78] = produced[163];
        self.canonical_staged[80] = produced[164];
        self.canonical_staged[81] = produced[165];
        self.canonical_staged[84] = produced[166];
        self.canonical_staged[85] = produced[167];
        self.canonical_staged[88] = produced[168];
        self.canonical_staged[92] = produced[169];
        self.canonical_staged[97] = produced[170];
        self.canonical_staged[102] = produced[171];
        self.canonical_staged[124] = produced[172];
        self.canonical_staged[106] = produced[173];
        self.canonical_staged[105] = produced[174];
        self.canonical_staged[123] = produced[175];
        self.canonical_staged[126] = produced[176];
        self.canonical_staged[130] = produced[177];
        self.canonical_staged[138] = produced[178];
        self.canonical_staged[139] = produced[179];
        self.canonical_staged[140] = produced[180];
        self.canonical_staged[174] = produced[181];
        self.canonical_staged[177] = produced[182];
        self.canonical_staged[143] = produced[183];
        self.canonical_staged[330] = produced[184];
        self.canonical_staged[331] = produced[185];
        self.canonical_staged[332] = produced[186];
        self.canonical_staged[168] = produced[187];
        self.canonical_staged[333] = produced[188];
        self.canonical_staged[172] = produced[189];
        self.canonical_staged[175] = produced[190];
        self.canonical_staged[178] = produced[191];
        self.canonical_staged[335] = produced[192];
        self.canonical_staged[179] = produced[193];
        self.canonical_staged[185] = produced[194];
        self.canonical_staged[186] = produced[195];
        self.canonical_staged[187] = produced[196];
        self.canonical_staged[188] = produced[197];
        self.canonical_staged[191] = produced[198];
        self.canonical_staged[193] = produced[199];
        self.canonical_staged[194] = produced[200];
        self.canonical_staged[195] = produced[201];
        self.canonical_staged[196] = produced[202];
        self.canonical_staged[197] = produced[203];
        self.canonical_staged[199] = produced[204];
        self.canonical_staged[198] = produced[205];
        self.canonical_staged[206] = produced[206];
        self.canonical_staged[212] = produced[207];
        self.canonical_staged[218] = produced[208];
        self.canonical_staged[220] = produced[209];
        self.canonical_staged[224] = produced[210];
        self.canonical_staged[225] = produced[211];
        self.canonical_staged[229] = produced[212];
        self.canonical_staged[236] = produced[213];
        self.canonical_staged[247] = produced[214];
        self.canonical_staged[250] = produced[215];
        self.canonical_staged[251] = produced[216];
        self.canonical_staged[252] = produced[217];
        self.canonical_staged[345] = produced[218];
        self.canonical_staged[255] = produced[219];
        self.canonical_staged[254] = produced[220];
        self.canonical_staged[336] = produced[221];
        self.canonical_staged[337] = produced[222];
        self.canonical_staged[256] = produced[223];
        self.canonical_staged[257] = produced[224];
        self.canonical_staged[258] = produced[225];
        self.canonical_staged[353] = produced[226];
        self.canonical_staged[259] = produced[227];
        self.canonical_staged[260] = produced[228];
        self.canonical_staged[261] = produced[229];
        self.canonical_staged[262] = produced[230];
        self.canonical_staged[263] = produced[231];
        self.canonical_staged[264] = produced[232];
        self.canonical_staged[265] = produced[233];
        self.canonical_staged[266] = produced[234];
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
            let multiplicity = self.multiplicity;
            let temperature = ctx.temperature();
            let staged = &*self.canonical_staged;
                let v0 = staged[309];
                let v1 = temperature;
                let v2 = parameters[9];
                let mut out3: f64 = 0.0;
                if v0 != 0.0 {
                } else {
                    let v3 = v1 + v2;
                    out3 = v3;
                }
            [out3]
        };
        self.canonical_staged[321] = produced[0];
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
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 22330 => 0usize, 22333 => 1usize, 22336 => 2usize, 22339 => 3usize, 22341 => 4usize, 22344 => 5usize, 22348 => 6usize, 22518 => 7usize, _ => usize::MAX };
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
            let v0 = staged[287];
            let v1 = staged[309];
            let v2 = temperature;
            let v3 = node_potentials[4];
            let v5 = parameters[9];
            let v7 = 1e0f64;
            let v8 = staged[321];
            let v9 = 0e0f64;
            let v12 = staged[12];
            let v19 = 2.5e-5f64;
            let v22 = 2e0f64;
            let v24 = 1e0f64;
            let v29 = 5e-1f64;
            let v32 = staged[13];
            let v36 = 8.61708e-5f64;
            let v39 = parameters[299];
            let v46 = parameters[300];
            let v52 = parameters[55];
            let v54 = -1e0f64;
            let v56 = 3.0015e2f64;
            let v67 = parameters[54];
            let v70 = 2e0f64;
            let v77 = staged[14];
            let v90 = staged[15];
            let v95 = 1e-38f64;
            let v106 = staged[16];
            let v123 = parameters[52];
            let v146 = 4e-8f64;
            let v158 = staged[322];
            let v159 = staged[323];
            let v160 = staged[324];
            let v165 = parameters[53];
            let v167 = staged[21];
            let v170 = staged[22];
            let v177 = staged[23];
            let v209 = staged[24];
            let v211 = staged[259];
            let v215 = staged[25];
            let v218 = staged[26];
            let v221 = 9e-1f64;
            let v226 = 4e-6f64;
            let v236 = 1e0f64;
            let v238 = 9.000011111097395e-1f64;
            let v244 = parameters[159];
            let v248 = 1e-6f64;
            let v253 = 4e-6f64;
            let v263 = staged[27];
            let v266 = staged[28];
            let v274 = 4e-6f64;
            let v284 = staged[29];
            let v287 = staged[30];
            let v289 = staged[260];
            let v293 = staged[31];
            let v296 = staged[32];
            let v298 = staged[261];
            let v302 = staged[33];
            let v305 = staged[34];
            let v313 = 4e-6f64;
            let v323 = staged[35];
            let v331 = 4e-6f64;
            let v342 = 9.000011111097395e-1f64;
            let v344 = staged[36];
            let v347 = 1e3f64;
            let v349 = staged[18];
            let v351 = staged[20];
            let v358 = 4e-6f64;
            let v369 = 9.000011111097395e-1f64;
            let v371 = staged[37];
            let v377 = 4e-6f64;
            let v388 = 9.000011111097395e-1f64;
            let v390 = staged[38];
            let v396 = parameters[309];
            let v399 = -9e-1f64;
            let v401 = 1e-4f64;
            let v403 = -9e-1f64;
            let v406 = -9e-1f64;
            let v413 = -3.6e-4f64;
            let v423 = -9e-1f64;
            let v426 = staged[39];
            let v429 = staged[40];
            let v437 = 4e-6f64;
            let v448 = 9.000011111097395e-1f64;
            let v450 = staged[41];
            let v453 = parameters[121];
            let v457 = staged[42];
            let v464 = 4e-6f64;
            let v475 = staged[43];
            let v478 = staged[44];
            let v480 = staged[45];
            let v483 = staged[46];
            let v489 = staged[47];
            let v500 = staged[48];
            let v502 = staged[49];
            let v505 = staged[50];
            let v507 = staged[51];
            let v516 = 4e-6f64;
            let v526 = staged[52];
            let v530 = staged[53];
            let v533 = staged[54];
            let v535 = staged[262];
            let v539 = staged[55];
            let v542 = staged[56];
            let v550 = 4e-6f64;
            let v560 = staged[57];
            let v563 = staged[58];
            let v571 = 4e-6f64;
            let v581 = staged[59];
            let v590 = staged[60];
            let v596 = staged[61];
            let v599 = node_potentials[8];
            let v600 = node_potentials[6];
            let v602 = 1e0f64;
            let v604 = 1e0f64;
            let v609 = node_potentials[5];
            let v611 = 1e0f64;
            let v623 = node_potentials[3];
            let v625 = 1e0f64;
            let v643 = 0e0f64;
            let v650 = -1e0f64;
            let v663 = node_potentials[7];
            let v665 = 1e0f64;
            let v680 = 4e-4f64;
            let v686 = 2e-2f64;
            let v703 = staged[62];
            let v706 = staged[63];
            let v713 = staged[64];
            let v719 = staged[65];
            let v722 = staged[66];
            let v729 = 3.141592653589793e0f64;
            let v733 = staged[67];
            let v736 = staged[68];
            let v738 = staged[69];
            let v744 = 4e1f64;
            let v761 = staged[70];
            let v787 = parameters[83];
            let v813 = staged[71];
            let v820 = staged[72];
            let v830 = staged[73];
            let v842 = staged[325];
            let v843 = staged[74];
            let v849 = staged[77];
            let v857 = staged[80];
            let v872 = staged[326];
            let v884 = staged[75];
            let v889 = staged[76];
            let v903 = staged[78];
            let v908 = staged[79];
            let v913 = staged[81];
            let v915 = staged[82];
            let v921 = 4e-4f64;
            let v931 = staged[83];
            let v940 = Lanes([0e0f64; 3]);
            let v951 = staged[84];
            let v953 = 1e-2f64;
            let v958 = staged[85];
            let v971 = -1.2e0f64;
            let v974 = staged[86];
            let v977 = staged[88];
            let v994 = 4e-6f64;
            let v1004 = 4e-1f64;
            let v1006 = staged[89];
            let v1009 = staged[90];
            let v1010 = staged[91];
            let v1020 = staged[92];
            let v1030 = staged[93];
            let v1049 = staged[94];
            let v1059 = staged[95];
            let v1062 = staged[96];
            let v1064 = staged[263];
            let v1075 = staged[97];
            let v1078 = parameters[70];
            let v1081 = staged[98];
            let v1088 = parameters[66];
            let v1091 = parameters[67];
            let v1100 = staged[99];
            let v1103 = staged[100];
            let v1105 = parameters[69];
            let v1124 = staged[102];
            let v1131 = staged[101];
            let v1134 = staged[105];
            let v1154 = staged[106];
            let v1164 = parameters[10];
            let v1166 = 3.20438e-19f64;
            let v1169 = parameters[49];
            let v1174 = staged[107];
            let v1184 = 3.675753940198048e0f64;
            let v1202 = staged[108];
            let v1209 = 3.947841e1f64;
            let v1217 = staged[109];
            let v1222 = staged[110];
            let v1227 = staged[111];
            let v1242 = staged[112];
            let v1247 = staged[113];
            let v1252 = staged[114];
            let v1277 = 6.534e-2f64;
            let v1281 = 8.57973e0f64;
            let v1287 = 7.895683e1f64;
            let v1296 = -4e0f64;
            let v1332 = 2.8985507246376816e0f64;
            let v1343 = 5e1f64;
            let v1415 = -2e0f64;
            let v1441 = 0e0f64;
            let v1453 = -2e0f64;
            let v1465 = -4e0f64;
            let v1480 = staged[115];
            let v1507 = -1e1f64;
            let v1511 = 1e1f64;
            let v1543 = -2e0f64;
            let v1555 = -4e0f64;
            let v1596 = -1e1f64;
            let v1605 = 4e0f64;
            let v1612 = 1.05e0f64;
            let v1671 = -5e-1f64;
            let v1678 = 2.5e-1f64;
            let v1710 = -2.5e-1f64;
            let v1777 = staged[116];
            let v1786 = staged[117];
            let v1794 = -1e0f64;
            let v1878 = -5e-1f64;
            let v1916 = -2.5e-1f64;
            let v1981 = staged[118];
            let v1997 = -1e0f64;
            let v2081 = -5e-1f64;
            let v2119 = -2.5e-1f64;
            let v2184 = staged[119];
            let v2200 = -1e0f64;
            let v2284 = -5e-1f64;
            let v2322 = -2.5e-1f64;
            let v2387 = staged[120];
            let v2403 = -1e0f64;
            let v2487 = -5e-1f64;
            let v2525 = -2.5e-1f64;
            let v2590 = staged[121];
            let v2606 = -1e0f64;
            let v2734 = staged[103];
            let v2741 = staged[122];
            let v2750 = staged[7];
            let v2769 = staged[123];
            let v2774 = staged[124];
            let v2779 = 1e-3f64;
            let v2789 = staged[125];
            let v2792 = staged[126];
            let v2810 = staged[127];
            let v2813 = staged[128];
            let v2848 = staged[129];
            let v2851 = staged[130];
            let v2881 = staged[131];
            let v2891 = parameters[11];
            let v2899 = staged[132];
            let v2901 = staged[264];
            let v2905 = staged[133];
            let v2908 = staged[134];
            let v2915 = staged[135];
            let v2918 = staged[136];
            let v2935 = staged[137];
            let v2959 = staged[138];
            let v3004 = Lanes([0e0f64; 5]);
            let v3005 = staged[327];
            let v3015 = staged[144];
            let v3018 = staged[145];
            let v3024 = staged[146];
            let v3030 = staged[147];
            let v3034 = staged[139];
            let v3054 = staged[140];
            let v3057 = staged[141];
            let v3059 = staged[142];
            let v3062 = parameters[2];
            let v3091 = staged[143];
            let v3114 = staged[148];
            let v3128 = 3e0f64;
            let v3176 = 4.0000000000000007e-10f64;
            let v3203 = staged[149];
            let v3205 = staged[265];
            let v3334 = -4e0f64;
            let v3367 = 2.8985507246376816e0f64;
            let v3444 = -2e0f64;
            let v3481 = -2e0f64;
            let v3493 = -4e0f64;
            let v3534 = -1e1f64;
            let v3569 = -2e0f64;
            let v3581 = -4e0f64;
            let v3622 = -1e1f64;
            let v3694 = -5e-1f64;
            let v3732 = -2.5e-1f64;
            let v3799 = staged[150];
            let v3815 = -1e0f64;
            let v3899 = -5e-1f64;
            let v3937 = -2.5e-1f64;
            let v4002 = staged[151];
            let v4018 = -1e0f64;
            let v4102 = -5e-1f64;
            let v4140 = -2.5e-1f64;
            let v4205 = staged[152];
            let v4221 = -1e0f64;
            let v4305 = -5e-1f64;
            let v4343 = -2.5e-1f64;
            let v4408 = staged[153];
            let v4424 = -1e0f64;
            let v4508 = -5e-1f64;
            let v4546 = -2.5e-1f64;
            let v4611 = staged[154];
            let v4627 = -1e0f64;
            let v4780 = 6.25e-4f64;
            let v4783 = staged[328];
            let v4786 = staged[155];
            let v4796 = parameters[162];
            let v4813 = staged[156];
            let v4818 = staged[329];
            let v4821 = staged[157];
            let v4831 = parameters[189];
            let v4936 = staged[158];
            let v5001 = staged[159];
            let v5035 = staged[160];
            let v5084 = staged[161];
            let v5096 = staged[266];
            let v5137 = 8e-1f64;
            let v5151 = 2e-1f64;
            let v5165 = parameters[109];
            let v5172 = staged[162];
            let v5175 = staged[163];
            let v5182 = staged[164];
            let v5210 = staged[165];
            let v5232 = staged[166];
            let v5235 = staged[330];
            let v5236 = staged[167];
            let v5289 = staged[331];
            let v5290 = staged[332];
            let v5297 = staged[333];
            let v5298 = parameters[213];
            let v5301 = staged[168];
            let v5311 = staged[169];
            let v5338 = staged[170];
            let v5463 = staged[171];
            let v5482 = staged[172];
            let v5485 = staged[173];
            let v5489 = staged[174];
            let v5539 = staged[175];
            let v5542 = staged[176];
            let v5546 = staged[177];
            let v5606 = staged[334];
            let v5643 = 1.6666666666666666e-1f64;
            let v5650 = 1.6666666666666666e-1f64;
            let v5655 = staged[335];
            let v5664 = staged[178];
            let v5680 = staged[336];
            let v5681 = staged[337];
            let v5682 = Lanes([0e0f64; 4]);
            let v5683 = Lanes([0e0f64; 4]);
            let v5684 = staged[179];
            let v5686 = staged[180];
            let v5688 = staged[181];
            let v5692 = staged[183];
            let v5694 = staged[184];
            let v5696 = staged[185];
            let v5698 = 3.4531302e-11f64;
            let v5700 = staged[338];
            let v5702 = staged[186];
            let v5727 = staged[187];
            let v5730 = staged[188];
            let v5742 = parameters[268];
            let v5744 = staged[189];
            let v5747 = parameters[269];
            let v5757 = 8e-2f64;
            let v5771 = parameters[265];
            let v5781 = staged[190];
            let v5786 = staged[191];
            let v5801 = parameters[270];
            let v5805 = parameters[271];
            let v5815 = 8e-2f64;
            let v5829 = parameters[266];
            let v5839 = staged[192];
            let v5844 = staged[193];
            let v5850 = staged[194];
            let v5853 = staged[195];
            let v5864 = staged[196];
            let v5869 = staged[197];
            let v5873 = staged[198];
            let v5875 = 8e1f64;
            let v5880 = staged[339];
            let v5888 = staged[199];
            let v5908 = 1.804851387e-35f64;
            let v5913 = staged[200];
            let v5915 = staged[201];
            let v5940 = staged[202];
            let v5943 = staged[203];
            let v5946 = staged[204];
            let v5950 = staged[205];
            let v5960 = staged[206];
            let v5983 = staged[207];
            let v6009 = Lanes([0e0f64; 3]);
            let v6014 = 6e-1f64;
            let v6045 = staged[340];
            let v6050 = 8e-2f64;
            let v6083 = staged[208];
            let v6086 = staged[209];
            let v6089 = staged[210];
            let v6093 = staged[211];
            let v6103 = staged[212];
            let v6120 = staged[213];
            let v6125 = staged[214];
            let v6128 = staged[215];
            let v6131 = staged[216];
            let v6135 = staged[217];
            let v6164 = staged[218];
            let v6184 = 1e-1f64;
            let v6186 = staged[219];
            let v6209 = 2e-4f64;
            let v6234 = staged[220];
            let v6249 = staged[221];
            let v6252 = staged[222];
            let v6255 = staged[223];
            let v6259 = staged[224];
            let v6278 = staged[341];
            let v6279 = parameters[234];
            let v6322 = staged[225];
            let v6337 = staged[226];
            let v6340 = staged[227];
            let v6343 = staged[228];
            let v6356 = parameters[235];
            let v6396 = staged[229];
            let v6403 = staged[342];
            let v6406 = staged[230];
            let v6412 = staged[231];
            let v6417 = staged[232];
            let v6425 = staged[233];
            let v6431 = 4e-4f64;
            let v6453 = staged[234];
            let v6459 = staged[235];
            let v6485 = staged[236];
            let v6489 = staged[237];
            let v6495 = staged[238];
            let v6500 = staged[239];
            let v6513 = 4e-4f64;
            let v6535 = staged[240];
            let v6543 = staged[241];
            let v6568 = staged[343];
            let v6569 = staged[252];
            let v6575 = staged[242];
            let v6577 = parameters[287];
            let v6585 = staged[344];
            let v6587 = staged[243];
            let v6589 = staged[244];
            let v6592 = staged[245];
            let v6594 = parameters[288];
            let v6599 = staged[246];
            let v6606 = 4.112842231783458e-57f64;
            let v6611 = 1e10f64;
            let v6613 = staged[247];
            let v6616 = 1.60219e-19f64;
            let v6621 = staged[248];
            let v6631 = parameters[289];
            let v6636 = staged[249];
            let v6643 = parameters[290];
            let v6652 = staged[250];
            let v6660 = staged[251];
            let v6678 = staged[345];
            let v6683 = staged[254];
            let v6693 = staged[255];
            let v6712 = 1e-12f64;
            let v6782 = ddt_scale();
            let v6812 = staged[346];
            let v6813 = Lanes([0e0f64; 5]);
            let v6814 = Lanes([0e0f64; 5]);
            let v6823 = node_potentials[0];
            let v6825 = 1e0f64;
            let v6835 = node_potentials[2];
            let v6837 = 1e0f64;
            let v6861 = Lanes([0e0f64; 6]);
            let v6864 = staged[347];
            let v6865 = Lanes([0e0f64; 2]);
            let v6866 = node_potentials[1];
            let v6868 = 1e0f64;
            let v6872 = staged[256];
            let v6881 = 0e0f64;
            let v6882 = 0e0f64;
            let v6883 = 0e0f64;
            let v6884 = 0e0f64;
            let v6889 = staged[348];
            let v6890 = Lanes([0e0f64; 7]);
            let v6959 = staged[257];
            let v6962 = staged[258];
            let v7114 = 0e0f64;
            let v7115 = 0e0f64;
            let v10: f64;
            let v11: f64;
            if v1 != 0.0 {
                let v6 = (v2 + v3) + v5;
                v10 = v6;
                v11 = v7;
            } else {
                v10 = v8;
                v11 = v9;
            }
            let v13 = if v10 > v12 { 1.0 } else { 0.0 };
            let v15 = v10 - v12;
            let v17 = v11 * v15;
            let v21 = ((v15 * v15) + v19).sqrt();
            let v30 = v29 * ((v10 + v12) - v21);
            let v31 = (v11 - ((v17 + v17) * (v24 / (v22 * v21)))) * v29;
            let v33 = v30 / v32;
            let v34 = v31 / v32;
            let v35 = v30 - v32;
            let v37 = v36 * v30;
            let v38 = v31 * v36;
            let v40 = v39 * v30;
            let v47 = v30 + v46;
            let v48 = (v40 * v30) / v47;
            let v53 = v52 - v48;
            let v55 = (((((v31 * v39) * v30) + (v31 * v40)) - (v31 * v48)) / v47) * v54;
            let v57 = v30 / v56;
            let v58 = v31 / v56;
            let v59 = v57.sqrt();
            let v68 = v67 * (v57 * v59);
            let v71 = v70 * v37;
            let v72 = v38 * v70;
            let v73 = v53 / v71;
            let v78 = v77 - v73;
            let v80 = rspice_limited_exp(v78);
            let v83 = v68 * v80;
            let v86 = ((((v58 * v59) + ((v58 * (v24 / (v22 * v59))) * v57)) * v67) * v80) + (((((v55 - (v72 * v73)) / v71) * v54) * (rspice_limited_exp_derivative(v78))) * v68);
            let v87 = v83 * v83;
            let v88 = v86 * v83;
            let v91 = v90 / v87;
            let v96 = if v91 >= v95 { v91 } else { v95 };
            let v99 = v96.ln();
            let v102 = v37 * v99;
            let v105 = (v38 * v99) + (((((((v88 + v88) * v91) * v54) / v87) * (if v91 >= v95 { 1.0 } else { 0.0 })) * (v24 / v96)) * v37);
            let v107 = v106 / v83;
            let v111 = if v107 >= v95 { v107 } else { v95 };
            let v114 = v111.ln();
            let v117 = v37 * v114;
            let v120 = (v38 * v114) + ((((((v86 * v107) * v54) / v83) * (if v107 >= v95 { 1.0 } else { 0.0 })) * (v24 / v111)) * v37);
            let v121 = v29 * v53;
            let v122 = v55 * v29;
            let v124 = v123 / v83;
            let v128 = if v124 >= v95 { v124 } else { v95 };
            let v131 = v128.ln();
            let v137 = (v38 * v131) + ((((((v86 * v124) * v54) / v83) * (if v124 >= v95 { 1.0 } else { 0.0 })) * (v24 / v128)) * v37);
            let v138 = v121 - (v37 * v131);
            let v148 = ((v138 * v138) + v146).sqrt();
            let v156 = v121 - (v29 * (v138 + v148));
            let v157 = v122 - (((v122 - v137) + ((((v122 - v137) * v138) + ((v122 - v137) * v138)) * (v24 / (v22 * v148)))) * v29);
            let v161: f64;
            let v162: f64;
            if v158 != 0.0 {
                let v354: f64;
                let v355: f64;
                if v159 != 0.0 {
                    let v350 = v349 + v156;
                    v354 = v350;
                    v355 = v157;
                } else {
                    let v352 = v351 - v156;
                    let v353 = v157 * v54;
                    v354 = v352;
                    v355 = v353;
                }
                v161 = v354;
                v162 = v355;
            } else {
                v161 = v160;
                v162 = v9;
            }
            let v163 = v53 / v70;
            let v164 = v55 / v70;
            let v166 = v165 + v163;
            let v171 = v170 * (v167 - v166);
            let v172 = (v164 * v54) * v170;
            let v175 = v170 * (v161 - v166);
            let v176 = (v162 - v164) * v170;
            let v178 = v177 / v83;
            let v182 = if v178 >= v95 { v178 } else { v95 };
            let v185 = v182.ln();
            let v188 = v37 * v185;
            let v191 = (v38 * v185) + ((((((v86 * v178) * v54) / v83) * (if v178 >= v95 { 1.0 } else { 0.0 })) * (v24 / v182)) * v37);
            let v199 = v166 - (v170 * (if v163 <= v188 { v163 } else { v188 }));
            let v200 = v164 - ((v191 + ((v164 - v191) * (if v163 <= v188 { 1.0 } else { 0.0 }))) * v170);
            let v203 = v170 * (v167 - v199);
            let v204 = (v200 * v54) * v170;
            let v207 = v170 * (v161 - v199);
            let v208 = (v162 - v200) * v170;
            let v216 = v215 * (v33.powf(v209));
            let v220 = v31 * v218;
            let v222 = v221 + (v218 * v35);
            let v224 = v220 * v222;
            let v228 = ((v222 * v222) + v226).sqrt();
            let v239 = (v236 + (v29 * (v222 + v228))) - v238;
            let v240 = v216 * v239;
            let v243 = (((v34 * (v209 * (v33.powf(v211)))) * v215) * v239) + (((v220 + ((v224 + v224) * (v24 / (v22 * v228)))) * v29) * v216);
            let v246 = v31 * v244;
            let v249 = (v236 + (v244 * v35)) - v248;
            let v251 = v246 * v249;
            let v255 = ((v249 * v249) + v253).sqrt();
            let v264 = v263 * (v29 * (v249 + v255));
            let v265 = ((v246 + ((v251 + v251) * (v24 / (v22 * v255)))) * v29) * v263;
            let v268 = v31 * v266;
            let v270 = (v236 + (v266 * v35)) - v248;
            let v272 = v268 * v270;
            let v276 = ((v270 * v270) + v274).sqrt();
            let v285 = v284 * (v29 * (v270 + v276));
            let v286 = ((v268 + ((v272 + v272) * (v24 / (v22 * v276)))) * v29) * v284;
            let v294 = v293 * (v33.powf(v287));
            let v295 = (v34 * (v287 * (v33.powf(v289)))) * v293;
            let v303 = v302 * (v33.powf(v296));
            let v304 = (v34 * (v296 * (v33.powf(v298)))) * v302;
            let v307 = v31 * v305;
            let v309 = (v236 + (v305 * v35)) - v248;
            let v311 = v307 * v309;
            let v315 = ((v309 * v309) + v313).sqrt();
            let v321 = v29 * (v309 + v315);
            let v322 = (v307 + ((v311 + v311) * (v24 / (v22 * v315)))) * v29;
            let v326 = v221 - (v323 * v35);
            let v327 = (v31 * v323) * v54;
            let v328 = v326 * v326;
            let v329 = v327 * v326;
            let v330 = v329 + v329;
            let v333 = (v328 + v331).sqrt();
            let v345 = v344 * ((v236 + (v29 * (v326 + v333))) - v342);
            let v346 = ((v327 + (v330 * (v24 / (v22 * v333)))) * v29) * v344;
            let v348 = if v345 < v347 { 1.0 } else { 0.0 };
            let v356: f64;
            let v357: f64;
            if v348 != 0.0 {
                v356 = v347;
                v357 = v9;
            } else {
                v356 = v345;
                v357 = v346;
            }
            let v360 = (v328 + v358).sqrt();
            let v372 = v371 * ((v236 + (v29 * (v326 + v360))) - v369);
            let v373 = ((v327 + (v330 * (v24 / (v22 * v360)))) * v29) * v371;
            let v374 = if v372 < v347 { 1.0 } else { 0.0 };
            let v375: f64;
            let v376: f64;
            if v374 != 0.0 {
                v375 = v347;
                v376 = v9;
            } else {
                v375 = v372;
                v376 = v373;
            }
            let v379 = (v328 + v377).sqrt();
            let v391 = v390 * ((v236 + (v29 * (v326 + v379))) - v388);
            let v392 = ((v327 + (v330 * (v24 / (v22 * v379)))) * v29) * v390;
            let v393 = if v391 < v347 { 1.0 } else { 0.0 };
            let v394: f64;
            let v395: f64;
            if v393 != 0.0 {
                v394 = v347;
                v395 = v9;
            } else {
                v394 = v391;
                v395 = v392;
            }
            let v397 = v396 * v35;
            let v398 = v31 * v396;
            let v405 = (v397 - v403) - v401;
            let v408 = (v397 - v406) - v401;
            let v415 = ((v405 * v408) - v413).sqrt();
            let v427 = v426 * (v236 + (v423 + (v29 * (((v397 - v399) - v401) + v415))));
            let v428 = ((v398 + (((v398 * v408) + (v398 * v405)) * (v24 / (v22 * v415)))) * v29) * v426;
            let v432 = v221 - (v429 * v35);
            let v433 = (v31 * v429) * v54;
            let v435 = v433 * v432;
            let v439 = ((v432 * v432) + v437).sqrt();
            let v451 = v450 * ((v236 + (v29 * (v432 + v439))) - v448);
            let v452 = ((v433 + ((v435 + v435) * (v24 / (v22 * v439)))) * v29) * v450;
            let v459 = (v31 * v453) * v457;
            let v460 = (v457 * (v236 + (v453 * v35))) - v70;
            let v462 = v459 * v460;
            let v466 = ((v460 * v460) + v464).sqrt();
            let v473 = (v459 + ((v462 + v462) * (v24 / (v22 * v466)))) * v29;
            let v474 = (v29 * (v460 + v466)) + v70;
            let v477 = v31 * v475;
            let v479 = v478 + (v475 * v35);
            let v482 = v31 * v480;
            let v485 = ((v480 * v35) - v483) - v248;
            let v487 = v482 * v485;
            let v491 = ((v485 * v485) - v489).sqrt();
            let v498 = (v482 + ((v487 + v487) * (v24 / (v22 * v491)))) * v29;
            let v501 = v500 + (v483 + (v29 * (v485 + v491)));
            let v504 = v31 * v502;
            let v506 = v505 + (v502 * v35);
            let v511 = (v31 * v507) * v54;
            let v512 = (v236 - (v507 * v35)) - v248;
            let v514 = v511 * v512;
            let v518 = ((v512 * v512) + v516).sqrt();
            let v527 = v526 * (v29 * (v512 + v518));
            let v528 = ((v511 + ((v514 + v514) * (v24 / (v22 * v518)))) * v29) * v526;
            let v529 = v33 - v236;
            let v531 = v530 * v529;
            let v532 = v34 * v530;
            let v540 = v539 * (v33.powf(v533));
            let v541 = (v34 * (v533 * (v33.powf(v535)))) * v539;
            let v544 = v31 * v542;
            let v546 = (v236 + (v542 * v35)) - v248;
            let v548 = v544 * v546;
            let v552 = ((v546 * v546) + v550).sqrt();
            let v561 = v560 * (v29 * (v546 + v552));
            let v562 = ((v544 + ((v548 + v548) * (v24 / (v22 * v552)))) * v29) * v560;
            let v565 = v31 * v563;
            let v567 = (v236 + (v563 * v35)) - v248;
            let v569 = v565 * v567;
            let v573 = ((v567 * v567) + v571).sqrt();
            let v582 = v581 * (v29 * (v567 + v573));
            let v583 = ((v565 + ((v569 + v569) * (v24 / (v22 * v573)))) * v29) * v581;
            let v584 = if v33 >= v95 { v33 } else { v95 };
            let v591 = v590 * (v584.ln());
            let v593 = rspice_limited_exp(v591);
            let v595 = (((v34 * (if v33 >= v95 { 1.0 } else { 0.0 })) * (v24 / v584)) * v590) * (rspice_limited_exp_derivative(v591));
            let v597 = v596 * v593;
            let v598 = v595 * v596;
            let v607 = v170 * (v599 - v600);
            let v608 = ((Lanes([0.0, v602])) - (Lanes([v604, 0.0]))) * v170;
            let v610 = v609 - v600;
            let v612 = Lanes([v611, 0.0]);
            let v613 = Lanes([0.0, v604]);
            let v614 = v612 - v613;
            let v615 = v170 * v610;
            let v616 = v614 * v170;
            let v621 = v170 * (v599 - v609);
            let v622 = ((Lanes([0.0, v602])) - (Lanes([v611, 0.0]))) * v170;
            let v626 = Lanes([v625, 0.0]);
            let v627 = Lanes([0.0, v604]);
            let v629 = v170 * (v623 - v600);
            let v630 = (v626 - v627) * v170;
            let v632 = Lanes([v625, 0.0]);
            let v633 = Lanes([0.0, v611]);
            let v635 = v170 * (v623 - v609);
            let v636 = (v632 - v633) * v170;
            let v641 = v170 * (v599 - v623);
            let v642 = ((Lanes([0.0, v602])) - (Lanes([v625, 0.0]))) * v170;
            let v644 = if v615 < v643 { 1.0 } else { 0.0 };
            let v654: f64;
            let v655: f64;
            let v656: f64;
            let v657: f64;
            let v658: f64;
            let v659: Lanes<2>;
            let v660: Lanes<3>;
            let v661: Lanes<3>;
            let v662: Lanes<3>;
            if v644 != 0.0 {
                let v645 = -v615;
                let v646 = v616 * v54;
                let v647 = Lanes([v636[0], v636[1], 0.0]);
                let v648 = Lanes([v622[0], 0.0, v622[1]]);
                let v649 = Lanes([v630[0], 0.0, v630[1]]);
                v654 = v645;
                v655 = v635;
                v656 = v621;
                v657 = v629;
                v658 = v650;
                v659 = v646;
                v660 = v647;
                v661 = v648;
                v662 = v649;
            } else {
                let v651 = Lanes([v630[0], 0.0, v630[1]]);
                let v652 = Lanes([0.0, v608[0], v608[1]]);
                let v653 = Lanes([v636[0], v636[1], 0.0]);
                v654 = v615;
                v655 = v629;
                v656 = v607;
                v657 = v635;
                v658 = v236;
                v659 = v616;
                v660 = v651;
                v661 = v652;
                v662 = v653;
            }
            let v664 = v663 - v609;
            let v668 = (Lanes([0.0, v665])) - (Lanes([v611, 0.0]));
            let v669 = v170 * v664;
            let v670 = v668 * v170;
            let v671 = v663 - v600;
            let v674 = (Lanes([0.0, v665])) - (Lanes([v604, 0.0]));
            let v675 = v170 * v671;
            let v676 = v674 * v170;
            let v678 = v659 * v654;
            let v682 = ((v654 * v654) + v680).sqrt();
            let v685 = (v678 + v678) * (v24 / (v22 * v682));
            let v687 = v682 - v686;
            let v690 = v29 * (v687 - v654);
            let v691 = (v685 - v659) * v29;
            let v692 = v655 + v690;
            let v694 = v660 + (Lanes([0.0, v691[0], v691[1]]));
            let v695 = v656 - v171;
            let v698 = (Lanes([0.0, v661[0], v661[1], v661[2]])) - (Lanes([v172, 0.0, 0.0, 0.0]));
            let v699 = v655 - v175;
            let v700 = Lanes([v660[0], 0.0, v660[1], v660[2]]);
            let v702 = v700 - (Lanes([0.0, v176, 0.0, 0.0]));
            let v705 = v698 * v703;
            let v708 = v702 * v706;
            let v723 = v722 + (v719 * ((((v695 * v703) + (v699 * v706)) / v713) + v690));
            let v735 = (((((((Lanes([0.0, v705[0], v705[1], v705[2], v705[3]])) + (Lanes([v708[0], v708[1], v708[2], v708[3], 0.0]))) / v713) + (Lanes([0.0, 0.0, v691[0], v691[1], 0.0]))) * v719) * (v24 / (v24 + (v723 * v723)))) / v729) * v733;
            let v737 = v736 + ((((v723.atan()) / v729) + v29) * v733);
            let v739 = v738 / v737;
            let v742 = ((v735 * v739) * v54) / v737;
            let v743 = v739 + v248;
            let v745 = if v743 < v744 { 1.0 } else { 0.0 };
            let v759: f64;
            let v760: Lanes<5>;
            if v745 != 0.0 {
                let v749 = (v743.cosh()) - v236;
                let v750 = v29 / v749;
                let v753 = (((v742 * (v743.sinh())) * v750) * v54) / v749;
                v759 = v750;
                v760 = v753;
            } else {
                let v754 = -v743;
                let v756 = rspice_limited_exp(v754);
                let v758 = (v742 * v54) * (rspice_limited_exp_derivative(v754));
                v759 = v756;
                v760 = v758;
            }
            let v762 = v761 / v737;
            let v765 = ((v735 * v762) * v54) / v737;
            let v766 = v762 + v248;
            let v767 = if v766 < v744 { 1.0 } else { 0.0 };
            let v781: f64;
            let v782: Lanes<5>;
            if v767 != 0.0 {
                let v771 = (v766.cosh()) - v236;
                let v772 = v29 / v771;
                let v775 = (((v765 * (v766.sinh())) * v772) * v54) / v771;
                v781 = v772;
                v782 = v775;
            } else {
                let v776 = -v766;
                let v778 = rspice_limited_exp(v776);
                let v780 = (v765 * v54) * (rspice_limited_exp_derivative(v776));
                v781 = v778;
                v782 = v780;
            }
            let v811: f64;
            let v812: Lanes<5>;
            if v767 != 0.0 {
                let v790 = v236 + (v787 * ((v766.cosh()) - v70));
                let v791 = if v790 >= v248 { v790 } else { v248 };
                let v794 = v236 / v791;
                let v797 = (((((v765 * (v766.sinh())) * v787) * (if v790 >= v248 { 1.0 } else { 0.0 })) * v794) * v54) / v791;
                v811 = v794;
                v812 = v797;
            } else {
                let v798 = -v766;
                let v800 = rspice_limited_exp(v798);
                let v802 = (v765 * v54) * (rspice_limited_exp_derivative(v798));
                let v803 = v800 + v787;
                let v804 = if v803 >= v248 { v803 } else { v248 };
                let v807 = v800 / v804;
                let v810 = (v802 - ((v802 * (if v803 >= v248 { 1.0 } else { 0.0 })) * v807)) / v804;
                v811 = v807;
                v812 = v810;
            }
            let v814 = v813 / v737;
            let v817 = ((v735 * v814) * v54) / v737;
            let v818 = v814 + v248;
            let v819 = if v818 < v744 { 1.0 } else { 0.0 };
            let v840: f64;
            let v841: Lanes<5>;
            if v819 != 0.0 {
                let v825 = (v818.cosh()) - v236;
                let v826 = (v29 * v820) / v825;
                let v829 = (((v817 * (v818.sinh())) * v826) * v54) / v825;
                let v831 = v826 + v830;
                v840 = v831;
                v841 = v829;
            } else {
                let v832 = -v818;
                let v838 = ((v817 * v54) * (rspice_limited_exp_derivative(v832))) * v820;
                let v839 = (v820 * (rspice_limited_exp(v832))) + v830;
                v840 = v839;
                v841 = v838;
            }
            let v855: f64;
            let v856: Lanes<5>;
            if v842 != 0.0 {
                let v844 = v843 / v737;
                let v847 = ((v735 * v844) * v54) / v737;
                let v848 = if v844 > v744 { 1.0 } else { 0.0 };
                let v882: f64;
                let v883: Lanes<5>;
                if v848 != 0.0 {
                    let v876 = (rspice_limited_exp(v844)) / v70;
                    let v877 = (v847 * (rspice_limited_exp_derivative(v844))) / v70;
                    v882 = v876;
                    v883 = v877;
                } else {
                    let v880 = v847 * (v844.sinh());
                    let v881 = (v844.cosh()) - v236;
                    v882 = v881;
                    v883 = v880;
                }
                let v885 = v884 / v882;
                let v890 = v889 - v885;
                let v891 = (((v883 * v885) * v54) / v882) * v54;
                v855 = v890;
                v856 = v891;
            } else {
                let v850 = v849 / v737;
                let v853 = ((v735 * v850) * v54) / v737;
                let v854 = if v850 > v744 { 1.0 } else { 0.0 };
                let v901: f64;
                let v902: Lanes<5>;
                if v854 != 0.0 {
                    let v895 = (rspice_limited_exp(v850)) / v70;
                    let v896 = (v853 * (rspice_limited_exp_derivative(v850))) / v70;
                    v901 = v895;
                    v902 = v896;
                } else {
                    let v899 = v853 * (v850.sinh());
                    let v900 = (v850.cosh()) - v236;
                    v901 = v900;
                    v902 = v899;
                }
                let v904 = v903 / v901;
                let v909 = v908 - v904;
                let v910 = (((v902 * v904) * v54) / v901) * v54;
                v855 = v909;
                v856 = v910;
            }
            let v858 = v855 - v857;
            let v860 = v856 * v858;
            let v863 = ((v858 * v858) + v401).sqrt();
            let v870 = (v856 + ((v860 + v860) * (v24 / (v22 * v863)))) * v29;
            let v871 = v857 + (v29 * (v858 + v863));
            let v941: f64;
            let v942: Lanes<3>;
            if v872 != 0.0 {
                let v916 = v915 * ((v170 * v692) - v913);
                let v917 = (v694 * v170) * v915;
                let v919 = v917 * v916;
                let v923 = ((v916 * v916) + v921).sqrt();
                let v935 = (v236 + ((v29 * (v916 + v923)) / v931)).sqrt();
                let v938 = (((v917 + ((v919 + v919) * (v24 / (v22 * v923)))) * v29) / v931) * (v24 / (v22 * v935));
                let v939 = v935 - v236;
                v941 = v939;
                v942 = v938;
            } else {
                v941 = v643;
                v942 = v940;
            }
            let v943 = v931 * v941;
            let v950 = (((v942 * v931) * v941) + (v942 * v943)) * v54;
            let v954 = ((-(v943 * v941)) - v951) - v953;
            let v956 = v950 * v954;
            let v960 = ((v954 * v954) - v958).sqrt();
            let v973 = v691 * v54;
            let v975 = v974 * v871;
            let v979 = (((v950 + ((v956 + v956) * (v24 / (v22 * v960)))) * v29) * v54) * v977;
            let v983 = (v699 - (v977 * (-(v951 + (v29 * (v954 + v960)))))) - (v971 - v690);
            let v986 = v975 * v983;
            let v988 = ((v702 - (Lanes([v979[0], 0.0, v979[1], v979[2]]))) - (Lanes([0.0, 0.0, v973[0], v973[1]]))) * v975;
            let v990 = ((v870 * v974) * v983) + (Lanes([v988[0], v988[1], v988[2], v988[3], 0.0]));
            let v992 = v694 * v692;
            let v996 = ((v692 * v692) + v994).sqrt();
            let v1002 = v29 * (v692 + v996);
            let v1003 = (v694 + ((v992 + v992) * (v24 / (v22 * v996)))) * v29;
            let v1007 = (v1004 + v117) + v1006;
            let v1008 = if v1007 < v643 { 1.0 } else { 0.0 };
            let v1018: f64;
            let v1019: f64;
            if v1008 != 0.0 {
                v1018 = v643;
                v1019 = v9;
            } else {
                let v1011 = v1009 * v1010;
                let v1012 = v1007.sqrt();
                let v1016 = v1011 * v1012;
                let v1017 = (v120 * (v24 / (v22 * v1012))) * v1011;
                v1018 = v1016;
                v1019 = v1017;
            }
            let v1021 = v1020 * v759;
            let v1023 = v102 - v1007;
            let v1032 = v694 * v1030;
            let v1037 = -(v427 + (v1030 * v692));
            let v1039 = v1037 * v781;
            let v1040 = (((Lanes([0.0, v428, 0.0, 0.0])) + (Lanes([v1032[0], 0.0, v1032[1], v1032[2]]))) * v54) * v781;
            let v1044 = v687 + v953;
            let v1045 = v1044.sqrt();
            let v1052 = v687 + (v1049 * v1045);
            let v1056 = (v685 + ((v685 * (v24 / (v22 * v1045))) * v1049)) * v1039;
            let v1060 = v1059 * v811;
            let v1063 = v1044.powf(v1062);
            let v1070 = (v685 * (v1062 * (v1044.powf(v1064)))) * v1060;
            let v1077 = v685 * v1075;
            let v1082 = v1081 + (v1078 * v1002);
            let v1085 = v685 * v1082;
            let v1092 = v1091 * v692;
            let v1099 = (v694 * v1088) + (((v694 * v1091) * v692) + (v694 * v1092));
            let v1106 = v1105 * v692;
            let v1114 = ((v1103 + (v1100 * v692)) + (v1106 * v692)) + (v1082 * v687);
            let v1118 = (((v694 * v1100) + (((v694 * v1105) * v692) + (v694 * v1106))) + (((v1003 * v1078) * v687) + (Lanes([0.0, v1085[0], v1085[1]])))) * v759;
            let v1125 = v1124 + (((v1088 * v692) + (v1092 * v692)) + (v759 * v1114));
            let v1132 = (v37 * v1125) / v1131;
            let v1133 = ((Lanes([0.0, (v38 * v1125), 0.0, 0.0, 0.0])) + (((Lanes([v1099[0], 0.0, v1099[1], v1099[2], 0.0])) + ((v760 * v1114) + (Lanes([v1118[0], 0.0, v1118[1], v1118[2], 0.0])))) * v37)) / v1131;
            let v1135 = v1134 * v692;
            let v1138 = (v694 * v1134) * v529;
            let v1145 = (Lanes([0.0, v532, 0.0, 0.0])) + ((Lanes([v1138[0], 0.0, v1138[1], v1138[2]])) + (Lanes([0.0, (v34 * v1135), 0.0, 0.0])));
            let v1159 = ((((((v1021 * v1023) + ((v1039 * v1052) + (v1060 * v1063))) + v1018) + (v1075 * v687)) + v1154) + (v531 + (v1135 * v529))) + v986;
            let v1160 = (((((((v760 * v1020) * v1023) + (Lanes([0.0, ((v105 - v120) * v1021), 0.0, 0.0, 0.0]))) + (((((Lanes([v1040[0], v1040[1], v1040[2], v1040[3], 0.0])) + (v782 * v1037)) * v1052) + (Lanes([0.0, 0.0, v1056[0], v1056[1], 0.0]))) + (((v812 * v1059) * v1063) + (Lanes([0.0, 0.0, v1070[0], v1070[1], 0.0]))))) + (Lanes([0.0, v1019, 0.0, 0.0, 0.0]))) + (Lanes([0.0, 0.0, v1077[0], v1077[1], 0.0]))) + (Lanes([v1145[0], v1145[1], v1145[2], v1145[3], 0.0]))) + v990;
            let v1162 = Lanes([0.0, v698[0], v698[1], v698[2], v698[3]]);
            let v1163 = v1162 - v1160;
            let v1165 = (v695 - v1159) + v1164;
            let v1175 = v1174 * v37;
            let v1177 = (((v1166 * v83) * v1169) * v1169) / v1175;
            let v1180 = ((((v86 * v1166) * v1169) * v1169) - ((v38 * v1174) * v1177)) / v1175;
            let v1181 = v1177.ln();
            let v1183 = v1180 * (v24 / v1177);
            let v1185 = v1184 - v1181;
            let v1186 = v1183 * v54;
            let v1187 = v1165 / v1132;
            let v1190 = (v1163 - (v1133 * v1187)) / v1132;
            let v1191 = v699 - v1159;
            let v1193 = (Lanes([v702[0], v702[1], v702[2], v702[3], 0.0])) - v1160;
            let v1194 = v1191 + v1164;
            let v1195 = v1194 / v1132;
            let v1198 = (v1193 - (v1133 * v1195)) / v1132;
            let v1199 = v1187 - v1185;
            let v1200 = Lanes([0.0, v1186, 0.0, 0.0, 0.0]);
            let v1201 = v1190 - v1200;
            let v1203 = v1202 * v1199;
            let v1210 = (v1203 * v1199) + v1209;
            let v1214 = (v1210.ln()) - v1181;
            let v1215 = Lanes([0.0, v1183, 0.0, 0.0, 0.0]);
            let v1216 = ((((v1201 * v1202) * v1199) + (v1201 * v1203)) * (v24 / v1210)) - v1215;
            let v1223 = (v1214 + (v1217 * v1195)) / v1222;
            let v1224 = (v1216 + (v1198 * v1217)) / v1222;
            let v1230 = v1195 + (v1227 * (v1187 - v1195));
            let v1232 = if v1230 <= v1214 { v1230 } else { v1214 };
            let v1237 = if v1232 <= v1185 { v1232 } else { v1185 };
            let v1241 = v1200 + (((v1216 + (((v1198 + ((v1190 - v1198) * v1227)) - v1216) * (if v1230 <= v1214 { 1.0 } else { 0.0 }))) - v1200) * (if v1232 <= v1185 { 1.0 } else { 0.0 }));
            let v1248 = (v1237 + (v1242 * v1187)) / v1247;
            let v1249 = (v1241 + (v1190 * v1242)) / v1247;
            let v1250 = v1195 - v1223;
            let v1251 = v1198 - v1224;
            let v1253 = v1252 * v1250;
            let v1259 = v1223.exp();
            let v1266 = (v1253 * v1250) - (v1177 * v1259);
            let v1267 = (((v1251 * v1252) * v1250) + (v1251 * v1253)) - ((Lanes([0.0, (v1180 * v1259), 0.0, 0.0, 0.0])) + ((v1224 * v1259) * v1177));
            let v1268 = if v1266 < v643 { 1.0 } else { 0.0 };
            let v1347: f64;
            let v1348: Lanes<5>;
            if v1268 != 0.0 {
                let v1271 = (v1195 - v1237) * v1217;
                let v1272 = (v1198 - v1241) * v1217;
                let v1273 = v744 * v1242;
                let v1274 = v1273 + v1271;
                let v1275 = v1273 * v1271;
                let v1276 = v1272 * v1273;
                let v1279 = v1272 * v1277;
                let v1280 = (v1277 * v1274) + v236;
                let v1285 = (v1272 * v1281) + v1276;
                let v1286 = ((v1274 * v1281) + v1275) + v1209;
                let v1292 = (v1287 * v1274) + (v1209 * v1275);
                let v1297 = v1296 * v1280;
                let v1304 = v1285 * v1286;
                let v1308 = ((v1297 * v1292) + (v1286 * v1286)).sqrt();
                let v1314 = v70 * v1280;
                let v1316 = ((-v1286) + v1308) / v1314;
                let v1335 = ((-((v1187 - (((v1185 * v1247) - v1237) / v1242)) + v70)) / v1332).exp();
                let v1337 = v236 - v1335;
                let v1339 = v1316 * v1337;
                let v1344 = if v1339 <= v1343 { v1339 } else { v1343 };
                let v1346 = ((((((v1285 * v54) + (((((v1279 * v1296) * v1292) + (((v1272 * v1287) + (v1276 * v1209)) * v1297)) + (v1304 + v1304)) * (v24 / (v22 * v1308)))) - ((v1279 * v70) * v1316)) / v1314) * v1337) + ((((((v1190 - (((Lanes([0.0, (v1186 * v1247), 0.0, 0.0, 0.0])) - v1241) / v1242)) * v54) / v1332) * v1335) * v54) * v1316)) * (if v1339 <= v1343 { 1.0 } else { 0.0 });
                v1347 = v1344;
                v1348 = v1346;
            } else {
                v1347 = v1266;
                v1348 = v1267;
            }
            let v1349 = if v1187 >= v1185 { v1187 } else { v1185 };
            let v1352 = v1200 + (v1201 * (if v1187 >= v1185 { 1.0 } else { 0.0 }));
            let v1353 = v1349 - v1185;
            let v1354 = v1352 - v1200;
            let v1355 = v1202 * v1353;
            let v1361 = (v1355 * v1353) + v1209;
            let v1367 = v1185 * v1247;
            let v1370 = Lanes([0.0, (v1186 * v1247), 0.0, 0.0, 0.0]);
            let v1374 = ((v1367 - v1237) / v1242) - v1185;
            let v1375 = ((v1370 - v1241) / v1242) - v1200;
            let v1376 = v1202 * v1374;
            let v1382 = (v1376 * v1374) + v1209;
            let v1390 = ((v1361.ln()) - v1181) - (((v1382.ln()) - v1181) - v1185);
            let v1391 = (((((v1354 * v1202) * v1353) + (v1354 * v1355)) * (v24 / v1361)) - v1215) - ((((((v1375 * v1202) * v1374) + (v1375 * v1376)) * (v24 / v1382)) - v1215) - v1200);
            let v1392 = v1349 - v1390;
            let v1393 = v1352 - v1391;
            let v1394 = -v1177;
            let v1395 = v1180 * v54;
            let v1396 = v1390.exp();
            let v1398 = v1394 * v1396;
            let v1402 = (Lanes([0.0, (v1395 * v1396), 0.0, 0.0, 0.0])) + ((v1391 * v1396) * v1394);
            let v1403 = v1202 * v1392;
            let v1404 = v1393 * v1202;
            let v1418 = (v1415 * v1403) + v1398;
            let v1420 = (-(((v1403 * v1392) + v1398) - v1347)) / v1418;
            let v1424 = v1390 + v1420;
            let v1425 = v1391 + (((((((v1404 * v1392) + (v1393 * v1403)) + v1402) - v1348) * v54) - (((v1404 * v1415) + v1402) * v1420)) / v1418);
            let v1426 = v1349 - v1424;
            let v1427 = v1352 - v1425;
            let v1428 = v1202 * v1426;
            let v1429 = v1427 * v1202;
            let v1434 = (v1428 * v1426) - v1347;
            let v1435 = ((v1429 * v1426) + (v1427 * v1428)) - v1348;
            let v1436 = v236 / v1434;
            let v1439 = ((v1435 * v1436) * v54) / v1434;
            let v1440 = v1434.abs();
            let v1451 = ((v1440.ln()) - v1181) - v1424;
            let v1454 = v1453 * v1428;
            let v1460 = (v1454 * v1436) - v236;
            let v1461 = v236 / v1460;
            let v1464 = (((((v1429 * v1453) * v1436) + (v1439 * v1454)) * v1461) * v54) / v1460;
            let v1466 = v1465 * v1428;
            let v1468 = v1466 * v1428;
            let v1472 = v1468 * v1436;
            let v1483 = (v1472 * v1436) + (v1480 * v1436);
            let v1485 = v1451 * v1461;
            let v1488 = (((((v1435 * ((v22 * (if v1434 >= v1441 { 1.0 } else { 0.0 })) - v24)) * (v24 / v1440)) - v1215) - v1425) * v1461) + (v1464 * v1451);
            let v1491 = v29 * v1485;
            let v1493 = v1491 * v1485;
            let v1497 = v1493 * v1483;
            let v1505 = (-v1485) - (v1497 * v1461);
            let v1508 = if v1505 >= v1507 { v1505 } else { v1507 };
            let v1515 = v1424 + (if v1508 <= v1511 { v1508 } else { v1511 });
            let v1516 = v1425 + ((((v1488 * v54) - (((((((v1488 * v29) * v1485) + (v1488 * v1491)) * v1483) + (((((((((v1429 * v1465) * v1428) + (v1429 * v1466)) * v1436) + (v1439 * v1468)) * v1436) + (v1439 * v1472)) + (v1439 * v1480)) * v1493)) * v1461) + (v1464 * v1497))) * (if v1505 >= v1507 { 1.0 } else { 0.0 })) * (if v1508 <= v1511 { 1.0 } else { 0.0 }));
            let v1517 = v1349 - v1515;
            let v1518 = v1352 - v1516;
            let v1519 = v1202 * v1517;
            let v1520 = v1518 * v1202;
            let v1525 = (v1519 * v1517) - v1347;
            let v1526 = ((v1520 * v1517) + (v1518 * v1519)) - v1348;
            let v1527 = v236 / v1525;
            let v1530 = ((v1526 * v1527) * v54) / v1525;
            let v1531 = v1525.abs();
            let v1541 = ((v1531.ln()) - v1181) - v1515;
            let v1544 = v1543 * v1519;
            let v1550 = (v1544 * v1527) - v236;
            let v1551 = v236 / v1550;
            let v1554 = (((((v1520 * v1543) * v1527) + (v1530 * v1544)) * v1551) * v54) / v1550;
            let v1556 = v1555 * v1519;
            let v1558 = v1556 * v1519;
            let v1562 = v1558 * v1527;
            let v1572 = (v1562 * v1527) + (v1480 * v1527);
            let v1574 = v1541 * v1551;
            let v1577 = (((((v1526 * ((v22 * (if v1525 >= v1441 { 1.0 } else { 0.0 })) - v24)) * (v24 / v1531)) - v1215) - v1516) * v1551) + (v1554 * v1541);
            let v1580 = v29 * v1574;
            let v1582 = v1580 * v1574;
            let v1586 = v1582 * v1572;
            let v1594 = (-v1574) - (v1586 * v1551);
            let v1597 = if v1594 >= v1596 { v1594 } else { v1596 };
            let v1603 = v1515 + (if v1597 <= v1511 { v1597 } else { v1511 });
            let v1606 = v1185 - v1605;
            let v1607 = if v1603 >= v1606 { v1603 } else { v1606 };
            let v1611 = v1200 + (((v1516 + ((((v1577 * v54) - (((((((v1577 * v29) * v1574) + (v1577 * v1580)) * v1572) + (((((((((v1520 * v1555) * v1519) + (v1520 * v1556)) * v1527) + (v1530 * v1558)) * v1527) + (v1530 * v1562)) + (v1530 * v1480)) * v1582)) * v1551) + (v1554 * v1586))) * (if v1594 >= v1596 { 1.0 } else { 0.0 })) * (if v1597 <= v1511 { 1.0 } else { 0.0 }))) - v1200) * (if v1603 >= v1606 { 1.0 } else { 0.0 }));
            let v1617 = (v1248 - (v1612 * v1607)).exp();
            let v1619 = v236 + v1617;
            let v1623 = v1248 - (v1619.ln());
            let v1625 = if v1623 <= v1607 { v1623 } else { v1607 };
            let v1629 = v1611 + (((v1249 - (((v1249 - (v1611 * v1612)) * v1617) * (v24 / v1619))) - v1611) * (if v1623 <= v1607 { 1.0 } else { 0.0 }));
            let v1630 = v1187 - v1625;
            let v1631 = v1190 - v1629;
            let v1632 = v1242 * v1630;
            let v1633 = v1631 * v1242;
            let v1634 = v1625.exp();
            let v1636 = v1394 * v1634;
            let v1640 = (Lanes([0.0, (v1395 * v1634), 0.0, 0.0, 0.0])) + ((v1629 * v1634) * v1394);
            let v1642 = v1633 * v1632;
            let v1644 = (v1632 * v1632) + v1636;
            let v1645 = (v1642 + v1642) + v1640;
            let v1646 = if v1644 < v643 { 1.0 } else { 0.0 };
            let v1715: f64;
            let v1716: f64;
            let v1717: f64;
            let v1718: f64;
            let v1719: f64;
            let v1720: Lanes<5>;
            let v1721: Lanes<5>;
            let v1722: Lanes<5>;
            let v1723: Lanes<5>;
            let v1724: Lanes<5>;
            if v1646 != 0.0 {
                let v1649 = (-v1644).sqrt();
                let v1652 = (v1645 * v54) * (v24 / (v22 * v1649));
                let v1653 = v29 * v1649;
                let v1654 = v1652 * v29;
                let v1655 = v1653.sin();
                let v1656 = v1653.cos();
                let v1658 = v236 / v1655;
                let v1661 = (((v1654 * v1656) * v1658) * v54) / v1655;
                let v1662 = v1658 * v1658;
                let v1663 = v1661 * v1658;
                let v1664 = v1663 + v1663;
                let v1667 = v1656 * v1658;
                let v1670 = ((v1654 * (v54 * v1655)) * v1658) + (v1661 * v1656);
                let v1674 = (v1671 * v1667) / v1649;
                let v1677 = ((v1670 * v1671) - (v1652 * v1674)) / v1649;
                let v1681 = (v1678 * v1662) + v1674;
                let v1682 = (v1664 * v1678) + v1677;
                v1715 = v1649;
                v1716 = v1667;
                v1717 = v1662;
                v1718 = v1674;
                v1719 = v1681;
                v1720 = v1652;
                v1721 = v1670;
                v1722 = v1664;
                v1723 = v1677;
                v1724 = v1682;
            } else {
                let v1683 = v1644.sqrt();
                let v1686 = v1645 * (v24 / (v22 * v1683));
                let v1687 = v29 * v1683;
                let v1689 = v1687.sinh();
                let v1692 = v236 / v1689;
                let v1696 = v1692 * v1692;
                let v1697 = (((((v1686 * v29) * (v1687.cosh())) * v1692) * v54) / v1689) * v1692;
                let v1698 = v1697 + v1697;
                let v1700 = (v236 + v1696).sqrt();
                let v1703 = v1698 * (v24 / (v22 * v1700));
                let v1706 = (v29 * v1700) / v1683;
                let v1709 = ((v1703 * v29) - (v1686 * v1706)) / v1683;
                let v1713 = (v1710 * v1696) + v1706;
                let v1714 = (v1698 * v1710) + v1709;
                v1715 = v1683;
                v1716 = v1700;
                v1717 = v1696;
                v1718 = v1706;
                v1719 = v1713;
                v1720 = v1686;
                v1721 = v1703;
                v1722 = v1698;
                v1723 = v1709;
                v1724 = v1714;
            }
            let v1729 = v1632 + (v1715 * v1716);
            let v1730 = v1633 + ((v1720 * v1716) + (v1721 * v1715));
            let v1731 = v236 / v1729;
            let v1734 = ((v1730 * v1731) * v54) / v1729;
            let v1735 = v1195 - v1187;
            let v1736 = v1198 - v1190;
            let v1739 = v1644 * v1717;
            let v1743 = v1739 * v1731;
            let v1747 = v1743 * v1731;
            let v1751 = v1747.abs();
            let v1759 = (v1735 + v1630) - (v1751.ln());
            let v1760 = (v1736 + v1631) - ((((((((v1645 * v1717) + (v1722 * v1644)) * v1731) + (v1734 * v1739)) * v1731) + (v1734 * v1743)) * ((v22 * (if v1747 >= v1441 { 1.0 } else { 0.0 })) - v24)) * (v24 / v1751));
            let v1763 = (v1217 * v1759) + v1632;
            let v1771 = v236 / v1644;
            let v1775 = v1771 - v1718;
            let v1780 = (v1777 * v1632) + v1636;
            let v1781 = (v1633 * v1777) + v1640;
            let v1782 = v1719 * v1780;
            let v1785 = (v1724 * v1780) + (v1781 * v1719);
            let v1787 = v1786 + v1782;
            let v1800 = (v1794 + (v70 * (v1787 * v1731))) - (v1775 * v1780);
            let v1818 = v1782 - v1242;
            let v1827 = ((v1636 - (v1242 * (v1632 + v1729))) + (v1632 * v1782)) + (v1217 * ((v1800 * v1729) + (v1759 * v1818)));
            let v1831 = (-(v1636 + (v1729 * v1763))) / v1827;
            let v1835 = v1625 + v1831;
            let v1836 = v1629 + ((((v1640 + ((v1730 * v1763) + (((v1760 * v1217) + v1633) * v1729))) * v54) - ((((v1640 - ((v1633 + v1730) * v1242)) + ((v1633 * v1782) + (v1785 * v1632))) + ((((((((v1785 * v1731) + (v1734 * v1787)) * v70) - ((((((v1645 * v1771) * v54) / v1644) - v1723) * v1780) + (v1781 * v1775))) * v1729) + (v1730 * v1800)) + ((v1760 * v1818) + (v1785 * v1759))) * v1217)) * v1831)) / v1827);
            let v1837 = v1187 - v1835;
            let v1838 = v1190 - v1836;
            let v1839 = v1242 * v1837;
            let v1840 = v1838 * v1242;
            let v1841 = v1835.exp();
            let v1843 = v1394 * v1841;
            let v1847 = (Lanes([0.0, (v1395 * v1841), 0.0, 0.0, 0.0])) + ((v1836 * v1841) * v1394);
            let v1849 = v1840 * v1839;
            let v1851 = (v1839 * v1839) + v1843;
            let v1852 = (v1849 + v1849) + v1847;
            let v1853 = if v1851 < v643 { 1.0 } else { 0.0 };
            let v1921: f64;
            let v1922: f64;
            let v1923: f64;
            let v1924: f64;
            let v1925: f64;
            let v1926: Lanes<5>;
            let v1927: Lanes<5>;
            let v1928: Lanes<5>;
            let v1929: Lanes<5>;
            let v1930: Lanes<5>;
            if v1853 != 0.0 {
                let v1856 = (-v1851).sqrt();
                let v1859 = (v1852 * v54) * (v24 / (v22 * v1856));
                let v1860 = v29 * v1856;
                let v1861 = v1859 * v29;
                let v1862 = v1860.sin();
                let v1863 = v1860.cos();
                let v1865 = v236 / v1862;
                let v1868 = (((v1861 * v1863) * v1865) * v54) / v1862;
                let v1869 = v1865 * v1865;
                let v1870 = v1868 * v1865;
                let v1871 = v1870 + v1870;
                let v1874 = v1863 * v1865;
                let v1877 = ((v1861 * (v54 * v1862)) * v1865) + (v1868 * v1863);
                let v1881 = (v1878 * v1874) / v1856;
                let v1884 = ((v1877 * v1878) - (v1859 * v1881)) / v1856;
                let v1887 = (v1678 * v1869) + v1881;
                let v1888 = (v1871 * v1678) + v1884;
                v1921 = v1856;
                v1922 = v1874;
                v1923 = v1869;
                v1924 = v1881;
                v1925 = v1887;
                v1926 = v1859;
                v1927 = v1877;
                v1928 = v1871;
                v1929 = v1884;
                v1930 = v1888;
            } else {
                let v1889 = v1851.sqrt();
                let v1892 = v1852 * (v24 / (v22 * v1889));
                let v1893 = v29 * v1889;
                let v1895 = v1893.sinh();
                let v1898 = v236 / v1895;
                let v1902 = v1898 * v1898;
                let v1903 = (((((v1892 * v29) * (v1893.cosh())) * v1898) * v54) / v1895) * v1898;
                let v1904 = v1903 + v1903;
                let v1906 = (v236 + v1902).sqrt();
                let v1909 = v1904 * (v24 / (v22 * v1906));
                let v1912 = (v29 * v1906) / v1889;
                let v1915 = ((v1909 * v29) - (v1892 * v1912)) / v1889;
                let v1919 = (v1916 * v1902) + v1912;
                let v1920 = (v1904 * v1916) + v1915;
                v1921 = v1889;
                v1922 = v1906;
                v1923 = v1902;
                v1924 = v1912;
                v1925 = v1919;
                v1926 = v1892;
                v1927 = v1909;
                v1928 = v1904;
                v1929 = v1915;
                v1930 = v1920;
            }
            let v1935 = v1839 + (v1921 * v1922);
            let v1936 = v1840 + ((v1926 * v1922) + (v1927 * v1921));
            let v1937 = v236 / v1935;
            let v1940 = ((v1936 * v1937) * v54) / v1935;
            let v1943 = v1851 * v1923;
            let v1947 = v1943 * v1937;
            let v1951 = v1947 * v1937;
            let v1955 = v1951.abs();
            let v1963 = (v1735 + v1837) - (v1955.ln());
            let v1964 = (v1736 + v1838) - ((((((((v1852 * v1923) + (v1928 * v1851)) * v1937) + (v1940 * v1943)) * v1937) + (v1940 * v1947)) * ((v22 * (if v1951 >= v1441 { 1.0 } else { 0.0 })) - v24)) * (v24 / v1955));
            let v1967 = (v1217 * v1963) + v1839;
            let v1975 = v236 / v1851;
            let v1979 = v1975 - v1924;
            let v1984 = (v1981 * v1839) + v1843;
            let v1985 = (v1840 * v1981) + v1847;
            let v1986 = v1925 * v1984;
            let v1989 = (v1930 * v1984) + (v1985 * v1925);
            let v1990 = v1786 + v1986;
            let v2003 = (v1997 + (v70 * (v1990 * v1937))) - (v1979 * v1984);
            let v2021 = v1986 - v1242;
            let v2030 = ((v1843 - (v1242 * (v1839 + v1935))) + (v1839 * v1986)) + (v1217 * ((v2003 * v1935) + (v1963 * v2021)));
            let v2034 = (-(v1843 + (v1935 * v1967))) / v2030;
            let v2038 = v1835 + v2034;
            let v2039 = v1836 + ((((v1847 + ((v1936 * v1967) + (((v1964 * v1217) + v1840) * v1935))) * v54) - ((((v1847 - ((v1840 + v1936) * v1242)) + ((v1840 * v1986) + (v1989 * v1839))) + ((((((((v1989 * v1937) + (v1940 * v1990)) * v70) - ((((((v1852 * v1975) * v54) / v1851) - v1929) * v1984) + (v1985 * v1979))) * v1935) + (v1936 * v2003)) + ((v1964 * v2021) + (v1989 * v1963))) * v1217)) * v2034)) / v2030);
            let v2040 = v1187 - v2038;
            let v2041 = v1190 - v2039;
            let v2042 = v1242 * v2040;
            let v2043 = v2041 * v1242;
            let v2044 = v2038.exp();
            let v2046 = v1394 * v2044;
            let v2050 = (Lanes([0.0, (v1395 * v2044), 0.0, 0.0, 0.0])) + ((v2039 * v2044) * v1394);
            let v2052 = v2043 * v2042;
            let v2054 = (v2042 * v2042) + v2046;
            let v2055 = (v2052 + v2052) + v2050;
            let v2056 = if v2054 < v643 { 1.0 } else { 0.0 };
            let v2124: f64;
            let v2125: f64;
            let v2126: f64;
            let v2127: f64;
            let v2128: f64;
            let v2129: Lanes<5>;
            let v2130: Lanes<5>;
            let v2131: Lanes<5>;
            let v2132: Lanes<5>;
            let v2133: Lanes<5>;
            if v2056 != 0.0 {
                let v2059 = (-v2054).sqrt();
                let v2062 = (v2055 * v54) * (v24 / (v22 * v2059));
                let v2063 = v29 * v2059;
                let v2064 = v2062 * v29;
                let v2065 = v2063.sin();
                let v2066 = v2063.cos();
                let v2068 = v236 / v2065;
                let v2071 = (((v2064 * v2066) * v2068) * v54) / v2065;
                let v2072 = v2068 * v2068;
                let v2073 = v2071 * v2068;
                let v2074 = v2073 + v2073;
                let v2077 = v2066 * v2068;
                let v2080 = ((v2064 * (v54 * v2065)) * v2068) + (v2071 * v2066);
                let v2084 = (v2081 * v2077) / v2059;
                let v2087 = ((v2080 * v2081) - (v2062 * v2084)) / v2059;
                let v2090 = (v1678 * v2072) + v2084;
                let v2091 = (v2074 * v1678) + v2087;
                v2124 = v2059;
                v2125 = v2077;
                v2126 = v2072;
                v2127 = v2084;
                v2128 = v2090;
                v2129 = v2062;
                v2130 = v2080;
                v2131 = v2074;
                v2132 = v2087;
                v2133 = v2091;
            } else {
                let v2092 = v2054.sqrt();
                let v2095 = v2055 * (v24 / (v22 * v2092));
                let v2096 = v29 * v2092;
                let v2098 = v2096.sinh();
                let v2101 = v236 / v2098;
                let v2105 = v2101 * v2101;
                let v2106 = (((((v2095 * v29) * (v2096.cosh())) * v2101) * v54) / v2098) * v2101;
                let v2107 = v2106 + v2106;
                let v2109 = (v236 + v2105).sqrt();
                let v2112 = v2107 * (v24 / (v22 * v2109));
                let v2115 = (v29 * v2109) / v2092;
                let v2118 = ((v2112 * v29) - (v2095 * v2115)) / v2092;
                let v2122 = (v2119 * v2105) + v2115;
                let v2123 = (v2107 * v2119) + v2118;
                v2124 = v2092;
                v2125 = v2109;
                v2126 = v2105;
                v2127 = v2115;
                v2128 = v2122;
                v2129 = v2095;
                v2130 = v2112;
                v2131 = v2107;
                v2132 = v2118;
                v2133 = v2123;
            }
            let v2138 = v2042 + (v2124 * v2125);
            let v2139 = v2043 + ((v2129 * v2125) + (v2130 * v2124));
            let v2140 = v236 / v2138;
            let v2143 = ((v2139 * v2140) * v54) / v2138;
            let v2146 = v2054 * v2126;
            let v2150 = v2146 * v2140;
            let v2154 = v2150 * v2140;
            let v2158 = v2154.abs();
            let v2166 = (v1735 + v2040) - (v2158.ln());
            let v2167 = (v1736 + v2041) - ((((((((v2055 * v2126) + (v2131 * v2054)) * v2140) + (v2143 * v2146)) * v2140) + (v2143 * v2150)) * ((v22 * (if v2154 >= v1441 { 1.0 } else { 0.0 })) - v24)) * (v24 / v2158));
            let v2170 = (v1217 * v2166) + v2042;
            let v2178 = v236 / v2054;
            let v2182 = v2178 - v2127;
            let v2187 = (v2184 * v2042) + v2046;
            let v2188 = (v2043 * v2184) + v2050;
            let v2189 = v2128 * v2187;
            let v2192 = (v2133 * v2187) + (v2188 * v2128);
            let v2193 = v1786 + v2189;
            let v2206 = (v2200 + (v70 * (v2193 * v2140))) - (v2182 * v2187);
            let v2224 = v2189 - v1242;
            let v2233 = ((v2046 - (v1242 * (v2042 + v2138))) + (v2042 * v2189)) + (v1217 * ((v2206 * v2138) + (v2166 * v2224)));
            let v2237 = (-(v2046 + (v2138 * v2170))) / v2233;
            let v2241 = v2038 + v2237;
            let v2242 = v2039 + ((((v2050 + ((v2139 * v2170) + (((v2167 * v1217) + v2043) * v2138))) * v54) - ((((v2050 - ((v2043 + v2139) * v1242)) + ((v2043 * v2189) + (v2192 * v2042))) + ((((((((v2192 * v2140) + (v2143 * v2193)) * v70) - ((((((v2055 * v2178) * v54) / v2054) - v2132) * v2187) + (v2188 * v2182))) * v2138) + (v2139 * v2206)) + ((v2167 * v2224) + (v2192 * v2166))) * v1217)) * v2237)) / v2233);
            let v2243 = v1187 - v2241;
            let v2244 = v1190 - v2242;
            let v2245 = v1242 * v2243;
            let v2246 = v2244 * v1242;
            let v2247 = v2241.exp();
            let v2249 = v1394 * v2247;
            let v2253 = (Lanes([0.0, (v1395 * v2247), 0.0, 0.0, 0.0])) + ((v2242 * v2247) * v1394);
            let v2255 = v2246 * v2245;
            let v2257 = (v2245 * v2245) + v2249;
            let v2258 = (v2255 + v2255) + v2253;
            let v2259 = if v2257 < v643 { 1.0 } else { 0.0 };
            let v2327: f64;
            let v2328: f64;
            let v2329: f64;
            let v2330: f64;
            let v2331: f64;
            let v2332: Lanes<5>;
            let v2333: Lanes<5>;
            let v2334: Lanes<5>;
            let v2335: Lanes<5>;
            let v2336: Lanes<5>;
            if v2259 != 0.0 {
                let v2262 = (-v2257).sqrt();
                let v2265 = (v2258 * v54) * (v24 / (v22 * v2262));
                let v2266 = v29 * v2262;
                let v2267 = v2265 * v29;
                let v2268 = v2266.sin();
                let v2269 = v2266.cos();
                let v2271 = v236 / v2268;
                let v2274 = (((v2267 * v2269) * v2271) * v54) / v2268;
                let v2275 = v2271 * v2271;
                let v2276 = v2274 * v2271;
                let v2277 = v2276 + v2276;
                let v2280 = v2269 * v2271;
                let v2283 = ((v2267 * (v54 * v2268)) * v2271) + (v2274 * v2269);
                let v2287 = (v2284 * v2280) / v2262;
                let v2290 = ((v2283 * v2284) - (v2265 * v2287)) / v2262;
                let v2293 = (v1678 * v2275) + v2287;
                let v2294 = (v2277 * v1678) + v2290;
                v2327 = v2262;
                v2328 = v2280;
                v2329 = v2275;
                v2330 = v2287;
                v2331 = v2293;
                v2332 = v2265;
                v2333 = v2283;
                v2334 = v2277;
                v2335 = v2290;
                v2336 = v2294;
            } else {
                let v2295 = v2257.sqrt();
                let v2298 = v2258 * (v24 / (v22 * v2295));
                let v2299 = v29 * v2295;
                let v2301 = v2299.sinh();
                let v2304 = v236 / v2301;
                let v2308 = v2304 * v2304;
                let v2309 = (((((v2298 * v29) * (v2299.cosh())) * v2304) * v54) / v2301) * v2304;
                let v2310 = v2309 + v2309;
                let v2312 = (v236 + v2308).sqrt();
                let v2315 = v2310 * (v24 / (v22 * v2312));
                let v2318 = (v29 * v2312) / v2295;
                let v2321 = ((v2315 * v29) - (v2298 * v2318)) / v2295;
                let v2325 = (v2322 * v2308) + v2318;
                let v2326 = (v2310 * v2322) + v2321;
                v2327 = v2295;
                v2328 = v2312;
                v2329 = v2308;
                v2330 = v2318;
                v2331 = v2325;
                v2332 = v2298;
                v2333 = v2315;
                v2334 = v2310;
                v2335 = v2321;
                v2336 = v2326;
            }
            let v2341 = v2245 + (v2327 * v2328);
            let v2342 = v2246 + ((v2332 * v2328) + (v2333 * v2327));
            let v2343 = v236 / v2341;
            let v2346 = ((v2342 * v2343) * v54) / v2341;
            let v2349 = v2257 * v2329;
            let v2353 = v2349 * v2343;
            let v2357 = v2353 * v2343;
            let v2361 = v2357.abs();
            let v2369 = (v1735 + v2243) - (v2361.ln());
            let v2370 = (v1736 + v2244) - ((((((((v2258 * v2329) + (v2334 * v2257)) * v2343) + (v2346 * v2349)) * v2343) + (v2346 * v2353)) * ((v22 * (if v2357 >= v1441 { 1.0 } else { 0.0 })) - v24)) * (v24 / v2361));
            let v2373 = (v1217 * v2369) + v2245;
            let v2381 = v236 / v2257;
            let v2385 = v2381 - v2330;
            let v2390 = (v2387 * v2245) + v2249;
            let v2391 = (v2246 * v2387) + v2253;
            let v2392 = v2331 * v2390;
            let v2395 = (v2336 * v2390) + (v2391 * v2331);
            let v2396 = v1786 + v2392;
            let v2409 = (v2403 + (v70 * (v2396 * v2343))) - (v2385 * v2390);
            let v2427 = v2392 - v1242;
            let v2436 = ((v2249 - (v1242 * (v2245 + v2341))) + (v2245 * v2392)) + (v1217 * ((v2409 * v2341) + (v2369 * v2427)));
            let v2440 = (-(v2249 + (v2341 * v2373))) / v2436;
            let v2444 = v2241 + v2440;
            let v2445 = v2242 + ((((v2253 + ((v2342 * v2373) + (((v2370 * v1217) + v2246) * v2341))) * v54) - ((((v2253 - ((v2246 + v2342) * v1242)) + ((v2246 * v2392) + (v2395 * v2245))) + ((((((((v2395 * v2343) + (v2346 * v2396)) * v70) - ((((((v2258 * v2381) * v54) / v2257) - v2335) * v2390) + (v2391 * v2385))) * v2341) + (v2342 * v2409)) + ((v2370 * v2427) + (v2395 * v2369))) * v1217)) * v2440)) / v2436);
            let v2446 = v1187 - v2444;
            let v2447 = v1190 - v2445;
            let v2448 = v1242 * v2446;
            let v2449 = v2447 * v1242;
            let v2450 = v2444.exp();
            let v2452 = v1394 * v2450;
            let v2456 = (Lanes([0.0, (v1395 * v2450), 0.0, 0.0, 0.0])) + ((v2445 * v2450) * v1394);
            let v2458 = v2449 * v2448;
            let v2460 = (v2448 * v2448) + v2452;
            let v2461 = (v2458 + v2458) + v2456;
            let v2462 = if v2460 < v643 { 1.0 } else { 0.0 };
            let v2530: f64;
            let v2531: f64;
            let v2532: f64;
            let v2533: f64;
            let v2534: f64;
            let v2535: Lanes<5>;
            let v2536: Lanes<5>;
            let v2537: Lanes<5>;
            let v2538: Lanes<5>;
            let v2539: Lanes<5>;
            if v2462 != 0.0 {
                let v2465 = (-v2460).sqrt();
                let v2468 = (v2461 * v54) * (v24 / (v22 * v2465));
                let v2469 = v29 * v2465;
                let v2470 = v2468 * v29;
                let v2471 = v2469.sin();
                let v2472 = v2469.cos();
                let v2474 = v236 / v2471;
                let v2477 = (((v2470 * v2472) * v2474) * v54) / v2471;
                let v2478 = v2474 * v2474;
                let v2479 = v2477 * v2474;
                let v2480 = v2479 + v2479;
                let v2483 = v2472 * v2474;
                let v2486 = ((v2470 * (v54 * v2471)) * v2474) + (v2477 * v2472);
                let v2490 = (v2487 * v2483) / v2465;
                let v2493 = ((v2486 * v2487) - (v2468 * v2490)) / v2465;
                let v2496 = (v1678 * v2478) + v2490;
                let v2497 = (v2480 * v1678) + v2493;
                v2530 = v2465;
                v2531 = v2483;
                v2532 = v2478;
                v2533 = v2490;
                v2534 = v2496;
                v2535 = v2468;
                v2536 = v2486;
                v2537 = v2480;
                v2538 = v2493;
                v2539 = v2497;
            } else {
                let v2498 = v2460.sqrt();
                let v2501 = v2461 * (v24 / (v22 * v2498));
                let v2502 = v29 * v2498;
                let v2504 = v2502.sinh();
                let v2507 = v236 / v2504;
                let v2511 = v2507 * v2507;
                let v2512 = (((((v2501 * v29) * (v2502.cosh())) * v2507) * v54) / v2504) * v2507;
                let v2513 = v2512 + v2512;
                let v2515 = (v236 + v2511).sqrt();
                let v2518 = v2513 * (v24 / (v22 * v2515));
                let v2521 = (v29 * v2515) / v2498;
                let v2524 = ((v2518 * v29) - (v2501 * v2521)) / v2498;
                let v2528 = (v2525 * v2511) + v2521;
                let v2529 = (v2513 * v2525) + v2524;
                v2530 = v2498;
                v2531 = v2515;
                v2532 = v2511;
                v2533 = v2521;
                v2534 = v2528;
                v2535 = v2501;
                v2536 = v2518;
                v2537 = v2513;
                v2538 = v2524;
                v2539 = v2529;
            }
            let v2544 = v2448 + (v2530 * v2531);
            let v2545 = v2449 + ((v2535 * v2531) + (v2536 * v2530));
            let v2546 = v236 / v2544;
            let v2549 = ((v2545 * v2546) * v54) / v2544;
            let v2552 = v2460 * v2532;
            let v2556 = v2552 * v2546;
            let v2560 = v2556 * v2546;
            let v2564 = v2560.abs();
            let v2572 = (v1735 + v2446) - (v2564.ln());
            let v2573 = (v1736 + v2447) - ((((((((v2461 * v2532) + (v2537 * v2460)) * v2546) + (v2549 * v2552)) * v2546) + (v2549 * v2556)) * ((v22 * (if v2560 >= v1441 { 1.0 } else { 0.0 })) - v24)) * (v24 / v2564));
            let v2576 = (v1217 * v2572) + v2448;
            let v2584 = v236 / v2460;
            let v2588 = v2584 - v2533;
            let v2593 = (v2590 * v2448) + v2452;
            let v2594 = (v2449 * v2590) + v2456;
            let v2595 = v2534 * v2593;
            let v2598 = (v2539 * v2593) + (v2594 * v2534);
            let v2599 = v1786 + v2595;
            let v2612 = (v2606 + (v70 * (v2599 * v2546))) - (v2588 * v2593);
            let v2630 = v2595 - v1242;
            let v2639 = ((v2452 - (v1242 * (v2448 + v2544))) + (v2448 * v2595)) + (v1217 * ((v2612 * v2544) + (v2572 * v2630)));
            let v2643 = (-(v2452 + (v2544 * v2576))) / v2639;
            let v2647 = v2444 + v2643;
            let v2648 = v2445 + ((((v2456 + ((v2545 * v2576) + (((v2573 * v1217) + v2449) * v2544))) * v54) - ((((v2456 - ((v2449 + v2545) * v1242)) + ((v2449 * v2595) + (v2598 * v2448))) + ((((((((v2598 * v2546) + (v2549 * v2599)) * v70) - ((((((v2461 * v2584) * v54) / v2460) - v2538) * v2593) + (v2594 * v2588))) * v2544) + (v2545 * v2612)) + ((v2573 * v2630) + (v2598 * v2572))) * v1217)) * v2643)) / v2639);
            let v2649 = v1187 - v2647;
            let v2650 = v1190 - v2648;
            let v2651 = v2647.exp();
            let v2653 = v1177 * v2651;
            let v2657 = (Lanes([0.0, (v1180 * v2651), 0.0, 0.0, 0.0])) + ((v2648 * v2651) * v1177);
            let v2658 = v1202 * v2649;
            let v2664 = (v2658 * v2649) - v2653;
            let v2665 = (((v2650 * v1202) * v2649) + (v2650 * v2658)) - v2657;
            let v2666 = if v2664 < v643 { 1.0 } else { 0.0 };
            let v2712: f64;
            let v2713: f64;
            let v2714: Lanes<5>;
            let v2715: Lanes<5>;
            if v2666 != 0.0 {
                let v2669 = (-v2664).sqrt();
                let v2672 = (v2665 * v54) * (v24 / (v22 * v2669));
                let v2673 = v29 * v2669;
                let v2674 = v2672 * v29;
                let v2675 = v2673.tan();
                let v2676 = v2673.cos();
                let v2680 = v2669 / v2675;
                let v2683 = (v2672 - ((v2674 * (v24 / (v2676 * v2676))) * v2680)) / v2675;
                let v2684 = v2673.sin();
                let v2685 = v2674 * v2676;
                let v2686 = -v2684;
                let v2688 = v2686 * v2684;
                let v2691 = ((v2685 * v54) * v2684) + (v2685 * v2686);
                v2712 = v2680;
                v2713 = v2688;
                v2714 = v2683;
                v2715 = v2691;
            } else {
                let v2692 = v2664.sqrt();
                let v2695 = v2665 * (v24 / (v22 * v2692));
                let v2696 = v29 * v2692;
                let v2697 = v2695 * v29;
                let v2698 = v2696.sinh();
                let v2701 = v2698 * v2698;
                let v2702 = (v2697 * (v2696.cosh())) * v2698;
                let v2703 = v2702 + v2702;
                let v2704 = v2696.tanh();
                let v2708 = v2692 / v2704;
                let v2711 = (v2695 - ((v2697 * (v24 - (v2704 * v2704))) * v2708)) / v2704;
                v2712 = v2708;
                v2713 = v2701;
                v2714 = v2711;
                v2715 = v2703;
            }
            let v2720 = v2713 * v2653;
            let v2724 = v2664 / v2720;
            let v2728 = v236 - v2724;
            let v2730 = ((v1242 * v2649) - v2712) / v2728;
            let v2733 = (((v2650 * v1242) - v2714) - ((((v2665 - (((v2715 * v2653) + (v2657 * v2713)) * v2724)) / v2720) * v54) * v2730)) / v2728;
            let v2735 = v2649 * v2734;
            let v2737 = v2735 * v1132;
            let v2740 = ((v2650 * v2734) * v1132) + (v1133 * v2735);
            let v2742 = v2730 * v2741;
            let v2744 = v2742 * v1132;
            let v2747 = ((v2733 * v2741) * v1132) + (v1133 * v2742);
            let v2748 = v2744 - v2737;
            let v2749 = v2747 - v2740;
            let v2751 = v2750 * v1132;
            let v2753 = v2748 / v2751;
            let v2757 = v1195 - v2753;
            let v2758 = v1198 - ((v2749 - ((v1133 * v2750) * v2753)) / v2751);
            let v2759 = v2647 + v2757;
            let v2765 = (v2759 * v1132) / v70;
            let v2766 = (((v2648 + v2758) * v1132) + (v1133 * v2759)) / v70;
            let v2767 = v2744 / v2734;
            let v2768 = v2747 / v2734;
            let v2773 = (v2740 * v2769) / v2734;
            let v2775 = ((v2769 * v2737) / v2734) + v2774;
            let v2777 = v2773 * v2775;
            let v2781 = ((v2775 * v2775) + v2779).sqrt();
            let v2790 = v2789 * (v29 * (v2775 + v2781));
            let v2796 = (v2749 * v2792) / v2750;
            let v2797 = ((v2792 * v2748) / v2750) + v2774;
            let v2799 = v2796 * v2797;
            let v2802 = ((v2797 * v2797) + v2779).sqrt();
            let v2811 = v2810 * (v29 * (v2797 + v2802));
            let v2814 = v2767 / v2813;
            let v2822 = v29 * (v236 + (v2814.abs()));
            let v2823 = ((v2768 / v2813) * ((v22 * (if v2814 >= v1441 { 1.0 } else { 0.0 })) - v24)) * v29;
            let v2824 = v2822.powf(v303);
            let v2825 = v303 - v24;
            let v2835 = v660 * v264;
            let v2840 = v285 + (v655 * v264);
            let v2841 = Lanes([0.0, v286, 0.0, 0.0]);
            let v2843 = v2790.abs();
            let v2852 = v2851 + (v2848 * v655);
            let v2853 = v2843.powf(v2852);
            let v2860 = (v660 * v2848) * (v2853 * (v2843.ln()));
            let v2864 = (v2841 + ((Lanes([v2835[0], 0.0, v2835[1], v2835[2]])) + (Lanes([0.0, (v265 * v655), 0.0, 0.0])))) * v2853;
            let v2868 = v294 / v2824;
            let v2874 = ((Lanes([v2864[0], v2864[1], v2864[2], v2864[3], 0.0])) + (((((((v2773 + ((v2777 + v2777) * (v24 / (v22 * v2781)))) * v29) * v2789) * ((v22 * (if v2790 >= v1441 { 1.0 } else { 0.0 })) - v24)) * (v2852 * (v2843.powf((v2852 - v24))))) + (Lanes([v2860[0], 0.0, v2860[1], v2860[2], 0.0]))) * v2840)) + (((Lanes([0.0, v295, 0.0, 0.0, 0.0])) - (((v2823 * (v303 * (v2822.powf(v2825)))) + (Lanes([0.0, (v304 * (v2824 * (v2822.ln()))), 0.0, 0.0, 0.0]))) * v2868)) / v2824);
            let v2875 = v236 + ((v2840 * v2853) + v2868);
            let v2877 = v2875 - v236;
            let v2879 = v2874 * v2877;
            let v2883 = ((v2877 * v2877) + v2881).sqrt();
            let v2892 = (v29 * ((v2875 + v236) + v2883)) / v2891;
            let v2894 = v240 / v2892;
            let v2896 = Lanes([0.0, v243, 0.0, 0.0, 0.0]);
            let v2900 = v2822.powf(v2899);
            let v2909 = v2908 + (v655 * v2905);
            let v2910 = v2811.abs();
            let v2919 = v2918 + (v2915 * v655);
            let v2920 = v2910.powf(v2919);
            let v2927 = (v660 * v2915) * (v2920 * (v2910.ln()));
            let v2931 = (v660 * v2905) * v2920;
            let v2936 = v2935 / v2900;
            let v2941 = ((Lanes([v2931[0], 0.0, v2931[1], v2931[2], 0.0])) + (((((((v2796 + ((v2799 + v2799) * (v24 / (v22 * v2802)))) * v29) * v2810) * ((v22 * (if v2811 >= v1441 { 1.0 } else { 0.0 })) - v24)) * (v2919 * (v2910.powf((v2919 - v24))))) + (Lanes([v2927[0], 0.0, v2927[1], v2927[2], 0.0]))) * v2909)) + ((((v2823 * (v2899 * (v2822.powf(v2901)))) * v2936) * v54) / v2900);
            let v2942 = v236 + ((v2909 * v2920) + v2936);
            let v2944 = v2942 - v236;
            let v2946 = v2941 * v2944;
            let v2949 = ((v2944 * v2944) + v2881).sqrt();
            let v2957 = (v29 * ((v2942 + v236) + v2949)) / v2891;
            let v2960 = v2959 / v2957;
            let v2972 = (v1165 - (v2737 / v2734)) / v1132;
            let v2976 = v2972.exp();
            let v2977 = (((v1163 - (v2740 / v2734)) - (v1133 * v2972)) / v1132) * v2976;
            let v2978 = (v1191 - (v2748 / v2750)) / v1132;
            let v2982 = v2978.exp();
            let v2983 = (((v1193 - (v2749 / v2750)) - (v1133 * v2978)) / v1132) * v2982;
            let v2984 = v2976 + v2982;
            let v2985 = v2977 + v2983;
            let v2986 = v2976 / v2984;
            let v2990 = v2982 / v2984;
            let v3002 = (v2986 * v2894) + (v2990 * v2960);
            let v3003 = ((((v2977 - (v2985 * v2986)) / v2984) * v2894) + (((v2896 - ((((v2874 + ((v2879 + v2879) * (v24 / (v22 * v2883)))) * v29) / v2891) * v2894)) / v2892) * v2986)) + ((((v2983 - (v2985 * v2990)) / v2984) * v2960) + (((((((v2941 + ((v2946 + v2946) * (v24 / (v22 * v2949)))) * v29) / v2891) * v2960) * v54) / v2957) * v2990));
            let v3006: f64;
            let v3007: Lanes<5>;
            if v0 != 0.0 {
                v3006 = v643;
                v3007 = v3004;
            } else {
                let v3102: f64;
                let v3103: Lanes<5>;
                if v3005 != 0.0 {
                    let v3037 = v236 + (v3034 * v2767);
                    let v3038 = v236 / v3037;
                    let v3041 = (((v2768 * v3034) * v3038) * v54) / v3037;
                    let v3043 = v3041 * v3038;
                    let v3046 = ((v3038 * v3038) + v953).sqrt();
                    let v3063 = ((v3057 + (v3054 * (v29 * (v3038 + v3046)))) * v3059) * v3062;
                    let v3065 = v3063 * v321;
                    let v3069 = ((((((v3041 + ((v3043 + v3043) * (v24 / (v22 * v3046)))) * v29) * v3054) * v3059) * v3062) * v321) + (Lanes([0.0, (v322 * v3063), 0.0, 0.0, 0.0]));
                    v3102 = v3065;
                    v3103 = v3069;
                } else {
                    let v3072 = v236 + (v3034 * v2767);
                    let v3073 = v236 / v3072;
                    let v3076 = (((v2768 * v3034) * v3073) * v54) / v3072;
                    let v3078 = v3076 * v3073;
                    let v3081 = ((v3073 * v3073) + v953).sqrt();
                    let v3095 = ((v3091 + (v3054 * (v29 * (v3073 + v3081)))) * v3059) * v3062;
                    let v3097 = v3095 * v321;
                    let v3101 = ((((((v3076 + ((v3078 + v3078) * (v24 / (v22 * v3081)))) * v29) * v3054) * v3059) * v3062) * v321) + (Lanes([0.0, (v322 * v3095), 0.0, 0.0, 0.0]));
                    v3102 = v3097;
                    v3103 = v3101;
                }
                v3006 = v3102;
                v3007 = v3103;
            }
            let v3008 = v70 * v356;
            let v3010 = v3008 / v3002;
            let v3016 = v3010 * v3015;
            let v3017 = (((Lanes([0.0, (v357 * v70), 0.0, 0.0, 0.0])) - (v3003 * v3010)) / v3002) * v3015;
            let v3020 = v1003 * v3018;
            let v3031 = v3030 * ((v2767 + (v3018 * v1002)) + (v71 * v3024));
            let v3032 = ((v2768 + (Lanes([v3020[0], 0.0, v3020[1], v3020[2], 0.0]))) + (Lanes([0.0, (v72 * v3024), 0.0, 0.0, 0.0]))) * v3030;
            let v3033 = if v3006 == v643 { 1.0 } else { 0.0 };
            let v3170: f64;
            let v3171: Lanes<5>;
            if v3033 != 0.0 {
                let v3108 = v3016 + v3031;
                let v3110 = (v3016 * v3031) / v3108;
                let v3113 = (((v3017 * v3031) + (v3032 * v3016)) - ((v3017 + v3032) * v3110)) / v3108;
                v3170 = v3110;
                v3171 = v3113;
            } else {
                let v3117 = (v3114 * v356) * v2734;
                let v3119 = v3117 * v3006;
                let v3123 = (Lanes([0.0, (((v357 * v3114) * v2734) * v3006), 0.0, 0.0, 0.0])) + (v3007 * v3117);
                let v3124 = v70 * v3119;
                let v3125 = v3123 * v70;
                let v3129 = v3128 * v3031;
                let v3135 = (v3031 + v3016) + (v3129 * v3119);
                let v3136 = (v3032 + v3017) + (((v3032 * v3128) * v3119) + (v3123 * v3129));
                let v3137 = v70 * v3031;
                let v3143 = v3016 + (v3137 * v3119);
                let v3145 = v3031 * v3143;
                let v3150 = v3136 * v3135;
                let v3152 = v70 * v3124;
                let v3160 = ((v3135 * v3135) - (v3152 * v3145)).sqrt();
                let v3166 = (v3135 - v3160) / v3124;
                let v3169 = ((v3136 - (((v3150 + v3150) - (((v3125 * v70) * v3145) + (((v3032 * v3143) + ((v3017 + (((v3032 * v70) * v3119) + (v3123 * v3137))) * v3031)) * v3152))) * (v24 / (v22 * v3160)))) - (v3125 * v3166)) / v3124;
                v3170 = v3166;
                v3171 = v3169;
            }
            let v3172 = v3170 - v2779;
            let v3174 = v3171 * v3172;
            let v3178 = ((v3172 * v3172) + v3176).sqrt();
            let v3185 = (v3171 + ((v3174 + v3174) * (v24 / (v22 * v3178)))) * v29;
            let v3186 = (v29 * (v3172 + v3178)) + v2779;
            let v3187 = v654 / v3186;
            let v3189 = Lanes([0.0, 0.0, v659[0], v659[1], 0.0]);
            let v3192 = v3187.powf(v474);
            let v3202 = v236 + v3192;
            let v3204 = v3202.powf(v3203);
            let v3209 = v654 / v3204;
            let v3212 = (v3189 - ((((((v3189 - (v3185 * v3187)) / v3186) * (v474 * (v3187.powf((v474 - v24))))) + (Lanes([0.0, (v473 * (v3192 * (v3187.ln()))), 0.0, 0.0, 0.0]))) * (v3203 * (v3202.powf(v3205)))) * v3209)) / v3204;
            let v3213 = if v3209 > v654 { 1.0 } else { 0.0 };
            let v3214: f64;
            let v3215: Lanes<5>;
            if v3213 != 0.0 {
                v3214 = v654;
                v3215 = v3189;
            } else {
                v3214 = v3209;
                v3215 = v3212;
            }
            let v3218 = (v1165 - v3214) / v1132;
            let v3221 = ((v1163 - v3215) - (v1133 * v3218)) / v1132;
            let v3224 = (v1194 - v3214) / v1132;
            let v3227 = ((v1193 - v3215) - (v1133 * v3224)) / v1132;
            let v3228 = v3218 - v1185;
            let v3229 = v3221 - v1200;
            let v3230 = v1202 * v3228;
            let v3236 = (v3230 * v3228) + v1209;
            let v3240 = (v3236.ln()) - v1181;
            let v3241 = ((((v3229 * v1202) * v3228) + (v3229 * v3230)) * (v24 / v3236)) - v1215;
            let v3246 = ((v1367 - v2757) / v1242) - v1185;
            let v3247 = ((v1370 - v2758) / v1242) - v1200;
            let v3248 = v1202 * v3246;
            let v3254 = (v3248 * v3246) + v1209;
            let v3268 = ((v3240 - (((v3254.ln()) - v1181) - v1185)) + (v1217 * v3224)) / v1222;
            let v3269 = ((v3241 - ((((((v3247 * v1202) * v3246) + (v3247 * v3248)) * (v24 / v3254)) - v1215) - v1200)) + (v3227 * v1217)) / v1222;
            let v3274 = v3224 + (v1227 * (v3218 - v3224));
            let v3276 = if v3274 <= v3240 { v3274 } else { v3240 };
            let v3281 = if v3276 <= v1185 { v3276 } else { v1185 };
            let v3285 = v1200 + (((v3241 + (((v3227 + ((v3221 - v3227) * v1227)) - v3241) * (if v3274 <= v3240 { 1.0 } else { 0.0 }))) - v1200) * (if v3276 <= v1185 { 1.0 } else { 0.0 }));
            let v3290 = (v3281 + (v1242 * v3218)) / v1247;
            let v3291 = (v3285 + (v3221 * v1242)) / v1247;
            let v3292 = v3224 - v3268;
            let v3293 = v3227 - v3269;
            let v3294 = v1252 * v3292;
            let v3300 = v3268.exp();
            let v3307 = (v3294 * v3292) - (v1177 * v3300);
            let v3308 = (((v3293 * v1252) * v3292) + (v3293 * v3294)) - ((Lanes([0.0, (v1180 * v3300), 0.0, 0.0, 0.0])) + ((v3269 * v3300) * v1177));
            let v3309 = if v3307 < v643 { 1.0 } else { 0.0 };
            let v3381: f64;
            let v3382: Lanes<5>;
            if v3309 != 0.0 {
                let v3312 = (v3224 - v3281) * v1217;
                let v3313 = (v3227 - v3285) * v1217;
                let v3314 = v744 * v1242;
                let v3315 = v3314 + v3312;
                let v3316 = v3314 * v3312;
                let v3317 = v3313 * v3314;
                let v3319 = v3313 * v1277;
                let v3320 = (v1277 * v3315) + v236;
                let v3324 = (v3313 * v1281) + v3317;
                let v3325 = ((v3315 * v1281) + v3316) + v1209;
                let v3330 = (v1287 * v3315) + (v1209 * v3316);
                let v3335 = v3334 * v3320;
                let v3342 = v3324 * v3325;
                let v3346 = ((v3335 * v3330) + (v3325 * v3325)).sqrt();
                let v3352 = v70 * v3320;
                let v3354 = ((-v3325) + v3346) / v3352;
                let v3370 = ((-((v3218 - ((v1367 - v3281) / v1242)) + v70)) / v3367).exp();
                let v3372 = v236 - v3370;
                let v3374 = v3354 * v3372;
                let v3378 = if v3374 <= v1343 { v3374 } else { v1343 };
                let v3380 = ((((((v3324 * v54) + (((((v3319 * v3334) * v3330) + (((v3313 * v1287) + (v3317 * v1209)) * v3335)) + (v3342 + v3342)) * (v24 / (v22 * v3346)))) - ((v3319 * v70) * v3354)) / v3352) * v3372) + ((((((v3221 - ((v1370 - v3285) / v1242)) * v54) / v3367) * v3370) * v54) * v3354)) * (if v3374 <= v1343 { 1.0 } else { 0.0 });
                v3381 = v3378;
                v3382 = v3380;
            } else {
                v3381 = v3307;
                v3382 = v3308;
            }
            let v3383 = if v3218 >= v1185 { v3218 } else { v1185 };
            let v3386 = v1200 + (v3229 * (if v3218 >= v1185 { 1.0 } else { 0.0 }));
            let v3387 = v3383 - v1185;
            let v3388 = v3386 - v1200;
            let v3389 = v1202 * v3387;
            let v3395 = (v3389 * v3387) + v1209;
            let v3405 = ((v1367 - v3281) / v1242) - v1185;
            let v3406 = ((v1370 - v3285) / v1242) - v1200;
            let v3407 = v1202 * v3405;
            let v3413 = (v3407 * v3405) + v1209;
            let v3421 = ((v3395.ln()) - v1181) - (((v3413.ln()) - v1181) - v1185);
            let v3422 = (((((v3388 * v1202) * v3387) + (v3388 * v3389)) * (v24 / v3395)) - v1215) - ((((((v3406 * v1202) * v3405) + (v3406 * v3407)) * (v24 / v3413)) - v1215) - v1200);
            let v3423 = v3383 - v3421;
            let v3424 = v3386 - v3422;
            let v3425 = v3421.exp();
            let v3427 = v1394 * v3425;
            let v3431 = (Lanes([0.0, (v1395 * v3425), 0.0, 0.0, 0.0])) + ((v3422 * v3425) * v1394);
            let v3432 = v1202 * v3423;
            let v3433 = v3424 * v1202;
            let v3447 = (v3444 * v3432) + v3427;
            let v3449 = (-(((v3432 * v3423) + v3427) - v3381)) / v3447;
            let v3453 = v3421 + v3449;
            let v3454 = v3422 + (((((((v3433 * v3423) + (v3424 * v3432)) + v3431) - v3382) * v54) - (((v3433 * v3444) + v3431) * v3449)) / v3447);
            let v3455 = v3383 - v3453;
            let v3456 = v3386 - v3454;
            let v3457 = v1202 * v3455;
            let v3458 = v3456 * v1202;
            let v3463 = (v3457 * v3455) - v3381;
            let v3464 = ((v3458 * v3455) + (v3456 * v3457)) - v3382;
            let v3465 = v236 / v3463;
            let v3468 = ((v3464 * v3465) * v54) / v3463;
            let v3469 = v3463.abs();
            let v3479 = ((v3469.ln()) - v1181) - v3453;
            let v3482 = v3481 * v3457;
            let v3488 = (v3482 * v3465) - v236;
            let v3489 = v236 / v3488;
            let v3492 = (((((v3458 * v3481) * v3465) + (v3468 * v3482)) * v3489) * v54) / v3488;
            let v3494 = v3493 * v3457;
            let v3496 = v3494 * v3457;
            let v3500 = v3496 * v3465;
            let v3510 = (v3500 * v3465) + (v1480 * v3465);
            let v3512 = v3479 * v3489;
            let v3515 = (((((v3464 * ((v22 * (if v3463 >= v1441 { 1.0 } else { 0.0 })) - v24)) * (v24 / v3469)) - v1215) - v3454) * v3489) + (v3492 * v3479);
            let v3518 = v29 * v3512;
            let v3520 = v3518 * v3512;
            let v3524 = v3520 * v3510;
            let v3532 = (-v3512) - (v3524 * v3489);
            let v3535 = if v3532 >= v3534 { v3532 } else { v3534 };
            let v3541 = v3453 + (if v3535 <= v1511 { v3535 } else { v1511 });
            let v3542 = v3454 + ((((v3515 * v54) - (((((((v3515 * v29) * v3512) + (v3515 * v3518)) * v3510) + (((((((((v3458 * v3493) * v3457) + (v3458 * v3494)) * v3465) + (v3468 * v3496)) * v3465) + (v3468 * v3500)) + (v3468 * v1480)) * v3520)) * v3489) + (v3492 * v3524))) * (if v3532 >= v3534 { 1.0 } else { 0.0 })) * (if v3535 <= v1511 { 1.0 } else { 0.0 }));
            let v3543 = v3383 - v3541;
            let v3544 = v3386 - v3542;
            let v3545 = v1202 * v3543;
            let v3546 = v3544 * v1202;
            let v3551 = (v3545 * v3543) - v3381;
            let v3552 = ((v3546 * v3543) + (v3544 * v3545)) - v3382;
            let v3553 = v236 / v3551;
            let v3556 = ((v3552 * v3553) * v54) / v3551;
            let v3557 = v3551.abs();
            let v3567 = ((v3557.ln()) - v1181) - v3541;
            let v3570 = v3569 * v3545;
            let v3576 = (v3570 * v3553) - v236;
            let v3577 = v236 / v3576;
            let v3580 = (((((v3546 * v3569) * v3553) + (v3556 * v3570)) * v3577) * v54) / v3576;
            let v3582 = v3581 * v3545;
            let v3584 = v3582 * v3545;
            let v3588 = v3584 * v3553;
            let v3598 = (v3588 * v3553) + (v1480 * v3553);
            let v3600 = v3567 * v3577;
            let v3603 = (((((v3552 * ((v22 * (if v3551 >= v1441 { 1.0 } else { 0.0 })) - v24)) * (v24 / v3557)) - v1215) - v3542) * v3577) + (v3580 * v3567);
            let v3606 = v29 * v3600;
            let v3608 = v3606 * v3600;
            let v3612 = v3608 * v3598;
            let v3620 = (-v3600) - (v3612 * v3577);
            let v3623 = if v3620 >= v3622 { v3620 } else { v3622 };
            let v3629 = v3541 + (if v3623 <= v1511 { v3623 } else { v1511 });
            let v3631 = if v3629 >= v1606 { v3629 } else { v1606 };
            let v3635 = v1200 + (((v3542 + ((((v3603 * v54) - (((((((v3603 * v29) * v3600) + (v3603 * v3606)) * v3598) + (((((((((v3546 * v3581) * v3545) + (v3546 * v3582)) * v3553) + (v3556 * v3584)) * v3553) + (v3556 * v3588)) + (v3556 * v1480)) * v3608)) * v3577) + (v3580 * v3612))) * (if v3620 >= v3622 { 1.0 } else { 0.0 })) * (if v3623 <= v1511 { 1.0 } else { 0.0 }))) - v1200) * (if v3629 >= v1606 { 1.0 } else { 0.0 }));
            let v3640 = (v3290 - (v1612 * v3631)).exp();
            let v3642 = v236 + v3640;
            let v3646 = v3290 - (v3642.ln());
            let v3648 = if v3646 <= v3631 { v3646 } else { v3631 };
            let v3652 = v3635 + (((v3291 - (((v3291 - (v3635 * v1612)) * v3640) * (v24 / v3642))) - v3635) * (if v3646 <= v3631 { 1.0 } else { 0.0 }));
            let v3653 = v3218 - v3648;
            let v3654 = v3221 - v3652;
            let v3655 = v1242 * v3653;
            let v3656 = v3654 * v1242;
            let v3657 = v3648.exp();
            let v3659 = v1394 * v3657;
            let v3663 = (Lanes([0.0, (v1395 * v3657), 0.0, 0.0, 0.0])) + ((v3652 * v3657) * v1394);
            let v3665 = v3656 * v3655;
            let v3667 = (v3655 * v3655) + v3659;
            let v3668 = (v3665 + v3665) + v3663;
            let v3669 = if v3667 < v643 { 1.0 } else { 0.0 };
            let v3737: f64;
            let v3738: f64;
            let v3739: f64;
            let v3740: f64;
            let v3741: f64;
            let v3742: Lanes<5>;
            let v3743: Lanes<5>;
            let v3744: Lanes<5>;
            let v3745: Lanes<5>;
            let v3746: Lanes<5>;
            if v3669 != 0.0 {
                let v3672 = (-v3667).sqrt();
                let v3675 = (v3668 * v54) * (v24 / (v22 * v3672));
                let v3676 = v29 * v3672;
                let v3677 = v3675 * v29;
                let v3678 = v3676.sin();
                let v3679 = v3676.cos();
                let v3681 = v236 / v3678;
                let v3684 = (((v3677 * v3679) * v3681) * v54) / v3678;
                let v3685 = v3681 * v3681;
                let v3686 = v3684 * v3681;
                let v3687 = v3686 + v3686;
                let v3690 = v3679 * v3681;
                let v3693 = ((v3677 * (v54 * v3678)) * v3681) + (v3684 * v3679);
                let v3697 = (v3694 * v3690) / v3672;
                let v3700 = ((v3693 * v3694) - (v3675 * v3697)) / v3672;
                let v3703 = (v1678 * v3685) + v3697;
                let v3704 = (v3687 * v1678) + v3700;
                v3737 = v3672;
                v3738 = v3690;
                v3739 = v3685;
                v3740 = v3697;
                v3741 = v3703;
                v3742 = v3675;
                v3743 = v3693;
                v3744 = v3687;
                v3745 = v3700;
                v3746 = v3704;
            } else {
                let v3705 = v3667.sqrt();
                let v3708 = v3668 * (v24 / (v22 * v3705));
                let v3709 = v29 * v3705;
                let v3711 = v3709.sinh();
                let v3714 = v236 / v3711;
                let v3718 = v3714 * v3714;
                let v3719 = (((((v3708 * v29) * (v3709.cosh())) * v3714) * v54) / v3711) * v3714;
                let v3720 = v3719 + v3719;
                let v3722 = (v236 + v3718).sqrt();
                let v3725 = v3720 * (v24 / (v22 * v3722));
                let v3728 = (v29 * v3722) / v3705;
                let v3731 = ((v3725 * v29) - (v3708 * v3728)) / v3705;
                let v3735 = (v3732 * v3718) + v3728;
                let v3736 = (v3720 * v3732) + v3731;
                v3737 = v3705;
                v3738 = v3722;
                v3739 = v3718;
                v3740 = v3728;
                v3741 = v3735;
                v3742 = v3708;
                v3743 = v3725;
                v3744 = v3720;
                v3745 = v3731;
                v3746 = v3736;
            }
            let v3751 = v3655 + (v3737 * v3738);
            let v3752 = v3656 + ((v3742 * v3738) + (v3743 * v3737));
            let v3753 = v236 / v3751;
            let v3756 = ((v3752 * v3753) * v54) / v3751;
            let v3757 = v3224 - v3218;
            let v3758 = v3227 - v3221;
            let v3761 = v3667 * v3739;
            let v3765 = v3761 * v3753;
            let v3769 = v3765 * v3753;
            let v3773 = v3769.abs();
            let v3781 = (v3757 + v3653) - (v3773.ln());
            let v3782 = (v3758 + v3654) - ((((((((v3668 * v3739) + (v3744 * v3667)) * v3753) + (v3756 * v3761)) * v3753) + (v3756 * v3765)) * ((v22 * (if v3769 >= v1441 { 1.0 } else { 0.0 })) - v24)) * (v24 / v3773));
            let v3785 = (v1217 * v3781) + v3655;
            let v3793 = v236 / v3667;
            let v3797 = v3793 - v3740;
            let v3802 = (v3799 * v3655) + v3659;
            let v3803 = (v3656 * v3799) + v3663;
            let v3804 = v3741 * v3802;
            let v3807 = (v3746 * v3802) + (v3803 * v3741);
            let v3808 = v1786 + v3804;
            let v3821 = (v3815 + (v70 * (v3808 * v3753))) - (v3797 * v3802);
            let v3839 = v3804 - v1242;
            let v3848 = ((v3659 - (v1242 * (v3655 + v3751))) + (v3655 * v3804)) + (v1217 * ((v3821 * v3751) + (v3781 * v3839)));
            let v3852 = (-(v3659 + (v3751 * v3785))) / v3848;
            let v3856 = v3648 + v3852;
            let v3857 = v3652 + ((((v3663 + ((v3752 * v3785) + (((v3782 * v1217) + v3656) * v3751))) * v54) - ((((v3663 - ((v3656 + v3752) * v1242)) + ((v3656 * v3804) + (v3807 * v3655))) + ((((((((v3807 * v3753) + (v3756 * v3808)) * v70) - ((((((v3668 * v3793) * v54) / v3667) - v3745) * v3802) + (v3803 * v3797))) * v3751) + (v3752 * v3821)) + ((v3782 * v3839) + (v3807 * v3781))) * v1217)) * v3852)) / v3848);
            let v3858 = v3218 - v3856;
            let v3859 = v3221 - v3857;
            let v3860 = v1242 * v3858;
            let v3861 = v3859 * v1242;
            let v3862 = v3856.exp();
            let v3864 = v1394 * v3862;
            let v3868 = (Lanes([0.0, (v1395 * v3862), 0.0, 0.0, 0.0])) + ((v3857 * v3862) * v1394);
            let v3870 = v3861 * v3860;
            let v3872 = (v3860 * v3860) + v3864;
            let v3873 = (v3870 + v3870) + v3868;
            let v3874 = if v3872 < v643 { 1.0 } else { 0.0 };
            let v3942: f64;
            let v3943: f64;
            let v3944: f64;
            let v3945: f64;
            let v3946: f64;
            let v3947: Lanes<5>;
            let v3948: Lanes<5>;
            let v3949: Lanes<5>;
            let v3950: Lanes<5>;
            let v3951: Lanes<5>;
            if v3874 != 0.0 {
                let v3877 = (-v3872).sqrt();
                let v3880 = (v3873 * v54) * (v24 / (v22 * v3877));
                let v3881 = v29 * v3877;
                let v3882 = v3880 * v29;
                let v3883 = v3881.sin();
                let v3884 = v3881.cos();
                let v3886 = v236 / v3883;
                let v3889 = (((v3882 * v3884) * v3886) * v54) / v3883;
                let v3890 = v3886 * v3886;
                let v3891 = v3889 * v3886;
                let v3892 = v3891 + v3891;
                let v3895 = v3884 * v3886;
                let v3898 = ((v3882 * (v54 * v3883)) * v3886) + (v3889 * v3884);
                let v3902 = (v3899 * v3895) / v3877;
                let v3905 = ((v3898 * v3899) - (v3880 * v3902)) / v3877;
                let v3908 = (v1678 * v3890) + v3902;
                let v3909 = (v3892 * v1678) + v3905;
                v3942 = v3877;
                v3943 = v3895;
                v3944 = v3890;
                v3945 = v3902;
                v3946 = v3908;
                v3947 = v3880;
                v3948 = v3898;
                v3949 = v3892;
                v3950 = v3905;
                v3951 = v3909;
            } else {
                let v3910 = v3872.sqrt();
                let v3913 = v3873 * (v24 / (v22 * v3910));
                let v3914 = v29 * v3910;
                let v3916 = v3914.sinh();
                let v3919 = v236 / v3916;
                let v3923 = v3919 * v3919;
                let v3924 = (((((v3913 * v29) * (v3914.cosh())) * v3919) * v54) / v3916) * v3919;
                let v3925 = v3924 + v3924;
                let v3927 = (v236 + v3923).sqrt();
                let v3930 = v3925 * (v24 / (v22 * v3927));
                let v3933 = (v29 * v3927) / v3910;
                let v3936 = ((v3930 * v29) - (v3913 * v3933)) / v3910;
                let v3940 = (v3937 * v3923) + v3933;
                let v3941 = (v3925 * v3937) + v3936;
                v3942 = v3910;
                v3943 = v3927;
                v3944 = v3923;
                v3945 = v3933;
                v3946 = v3940;
                v3947 = v3913;
                v3948 = v3930;
                v3949 = v3925;
                v3950 = v3936;
                v3951 = v3941;
            }
            let v3956 = v3860 + (v3942 * v3943);
            let v3957 = v3861 + ((v3947 * v3943) + (v3948 * v3942));
            let v3958 = v236 / v3956;
            let v3961 = ((v3957 * v3958) * v54) / v3956;
            let v3964 = v3872 * v3944;
            let v3968 = v3964 * v3958;
            let v3972 = v3968 * v3958;
            let v3976 = v3972.abs();
            let v3984 = (v3757 + v3858) - (v3976.ln());
            let v3985 = (v3758 + v3859) - ((((((((v3873 * v3944) + (v3949 * v3872)) * v3958) + (v3961 * v3964)) * v3958) + (v3961 * v3968)) * ((v22 * (if v3972 >= v1441 { 1.0 } else { 0.0 })) - v24)) * (v24 / v3976));
            let v3988 = (v1217 * v3984) + v3860;
            let v3996 = v236 / v3872;
            let v4000 = v3996 - v3945;
            let v4005 = (v4002 * v3860) + v3864;
            let v4006 = (v3861 * v4002) + v3868;
            let v4007 = v3946 * v4005;
            let v4010 = (v3951 * v4005) + (v4006 * v3946);
            let v4011 = v1786 + v4007;
            let v4024 = (v4018 + (v70 * (v4011 * v3958))) - (v4000 * v4005);
            let v4042 = v4007 - v1242;
            let v4051 = ((v3864 - (v1242 * (v3860 + v3956))) + (v3860 * v4007)) + (v1217 * ((v4024 * v3956) + (v3984 * v4042)));
            let v4055 = (-(v3864 + (v3956 * v3988))) / v4051;
            let v4059 = v3856 + v4055;
            let v4060 = v3857 + ((((v3868 + ((v3957 * v3988) + (((v3985 * v1217) + v3861) * v3956))) * v54) - ((((v3868 - ((v3861 + v3957) * v1242)) + ((v3861 * v4007) + (v4010 * v3860))) + ((((((((v4010 * v3958) + (v3961 * v4011)) * v70) - ((((((v3873 * v3996) * v54) / v3872) - v3950) * v4005) + (v4006 * v4000))) * v3956) + (v3957 * v4024)) + ((v3985 * v4042) + (v4010 * v3984))) * v1217)) * v4055)) / v4051);
            let v4061 = v3218 - v4059;
            let v4062 = v3221 - v4060;
            let v4063 = v1242 * v4061;
            let v4064 = v4062 * v1242;
            let v4065 = v4059.exp();
            let v4067 = v1394 * v4065;
            let v4071 = (Lanes([0.0, (v1395 * v4065), 0.0, 0.0, 0.0])) + ((v4060 * v4065) * v1394);
            let v4073 = v4064 * v4063;
            let v4075 = (v4063 * v4063) + v4067;
            let v4076 = (v4073 + v4073) + v4071;
            let v4077 = if v4075 < v643 { 1.0 } else { 0.0 };
            let v4145: f64;
            let v4146: f64;
            let v4147: f64;
            let v4148: f64;
            let v4149: f64;
            let v4150: Lanes<5>;
            let v4151: Lanes<5>;
            let v4152: Lanes<5>;
            let v4153: Lanes<5>;
            let v4154: Lanes<5>;
            if v4077 != 0.0 {
                let v4080 = (-v4075).sqrt();
                let v4083 = (v4076 * v54) * (v24 / (v22 * v4080));
                let v4084 = v29 * v4080;
                let v4085 = v4083 * v29;
                let v4086 = v4084.sin();
                let v4087 = v4084.cos();
                let v4089 = v236 / v4086;
                let v4092 = (((v4085 * v4087) * v4089) * v54) / v4086;
                let v4093 = v4089 * v4089;
                let v4094 = v4092 * v4089;
                let v4095 = v4094 + v4094;
                let v4098 = v4087 * v4089;
                let v4101 = ((v4085 * (v54 * v4086)) * v4089) + (v4092 * v4087);
                let v4105 = (v4102 * v4098) / v4080;
                let v4108 = ((v4101 * v4102) - (v4083 * v4105)) / v4080;
                let v4111 = (v1678 * v4093) + v4105;
                let v4112 = (v4095 * v1678) + v4108;
                v4145 = v4080;
                v4146 = v4098;
                v4147 = v4093;
                v4148 = v4105;
                v4149 = v4111;
                v4150 = v4083;
                v4151 = v4101;
                v4152 = v4095;
                v4153 = v4108;
                v4154 = v4112;
            } else {
                let v4113 = v4075.sqrt();
                let v4116 = v4076 * (v24 / (v22 * v4113));
                let v4117 = v29 * v4113;
                let v4119 = v4117.sinh();
                let v4122 = v236 / v4119;
                let v4126 = v4122 * v4122;
                let v4127 = (((((v4116 * v29) * (v4117.cosh())) * v4122) * v54) / v4119) * v4122;
                let v4128 = v4127 + v4127;
                let v4130 = (v236 + v4126).sqrt();
                let v4133 = v4128 * (v24 / (v22 * v4130));
                let v4136 = (v29 * v4130) / v4113;
                let v4139 = ((v4133 * v29) - (v4116 * v4136)) / v4113;
                let v4143 = (v4140 * v4126) + v4136;
                let v4144 = (v4128 * v4140) + v4139;
                v4145 = v4113;
                v4146 = v4130;
                v4147 = v4126;
                v4148 = v4136;
                v4149 = v4143;
                v4150 = v4116;
                v4151 = v4133;
                v4152 = v4128;
                v4153 = v4139;
                v4154 = v4144;
            }
            let v4159 = v4063 + (v4145 * v4146);
            let v4160 = v4064 + ((v4150 * v4146) + (v4151 * v4145));
            let v4161 = v236 / v4159;
            let v4164 = ((v4160 * v4161) * v54) / v4159;
            let v4167 = v4075 * v4147;
            let v4171 = v4167 * v4161;
            let v4175 = v4171 * v4161;
            let v4179 = v4175.abs();
            let v4187 = (v3757 + v4061) - (v4179.ln());
            let v4188 = (v3758 + v4062) - ((((((((v4076 * v4147) + (v4152 * v4075)) * v4161) + (v4164 * v4167)) * v4161) + (v4164 * v4171)) * ((v22 * (if v4175 >= v1441 { 1.0 } else { 0.0 })) - v24)) * (v24 / v4179));
            let v4191 = (v1217 * v4187) + v4063;
            let v4199 = v236 / v4075;
            let v4203 = v4199 - v4148;
            let v4208 = (v4205 * v4063) + v4067;
            let v4209 = (v4064 * v4205) + v4071;
            let v4210 = v4149 * v4208;
            let v4213 = (v4154 * v4208) + (v4209 * v4149);
            let v4214 = v1786 + v4210;
            let v4227 = (v4221 + (v70 * (v4214 * v4161))) - (v4203 * v4208);
            let v4245 = v4210 - v1242;
            let v4254 = ((v4067 - (v1242 * (v4063 + v4159))) + (v4063 * v4210)) + (v1217 * ((v4227 * v4159) + (v4187 * v4245)));
            let v4258 = (-(v4067 + (v4159 * v4191))) / v4254;
            let v4262 = v4059 + v4258;
            let v4263 = v4060 + ((((v4071 + ((v4160 * v4191) + (((v4188 * v1217) + v4064) * v4159))) * v54) - ((((v4071 - ((v4064 + v4160) * v1242)) + ((v4064 * v4210) + (v4213 * v4063))) + ((((((((v4213 * v4161) + (v4164 * v4214)) * v70) - ((((((v4076 * v4199) * v54) / v4075) - v4153) * v4208) + (v4209 * v4203))) * v4159) + (v4160 * v4227)) + ((v4188 * v4245) + (v4213 * v4187))) * v1217)) * v4258)) / v4254);
            let v4264 = v3218 - v4262;
            let v4265 = v3221 - v4263;
            let v4266 = v1242 * v4264;
            let v4267 = v4265 * v1242;
            let v4268 = v4262.exp();
            let v4270 = v1394 * v4268;
            let v4274 = (Lanes([0.0, (v1395 * v4268), 0.0, 0.0, 0.0])) + ((v4263 * v4268) * v1394);
            let v4276 = v4267 * v4266;
            let v4278 = (v4266 * v4266) + v4270;
            let v4279 = (v4276 + v4276) + v4274;
            let v4280 = if v4278 < v643 { 1.0 } else { 0.0 };
            let v4348: f64;
            let v4349: f64;
            let v4350: f64;
            let v4351: f64;
            let v4352: f64;
            let v4353: Lanes<5>;
            let v4354: Lanes<5>;
            let v4355: Lanes<5>;
            let v4356: Lanes<5>;
            let v4357: Lanes<5>;
            if v4280 != 0.0 {
                let v4283 = (-v4278).sqrt();
                let v4286 = (v4279 * v54) * (v24 / (v22 * v4283));
                let v4287 = v29 * v4283;
                let v4288 = v4286 * v29;
                let v4289 = v4287.sin();
                let v4290 = v4287.cos();
                let v4292 = v236 / v4289;
                let v4295 = (((v4288 * v4290) * v4292) * v54) / v4289;
                let v4296 = v4292 * v4292;
                let v4297 = v4295 * v4292;
                let v4298 = v4297 + v4297;
                let v4301 = v4290 * v4292;
                let v4304 = ((v4288 * (v54 * v4289)) * v4292) + (v4295 * v4290);
                let v4308 = (v4305 * v4301) / v4283;
                let v4311 = ((v4304 * v4305) - (v4286 * v4308)) / v4283;
                let v4314 = (v1678 * v4296) + v4308;
                let v4315 = (v4298 * v1678) + v4311;
                v4348 = v4283;
                v4349 = v4301;
                v4350 = v4296;
                v4351 = v4308;
                v4352 = v4314;
                v4353 = v4286;
                v4354 = v4304;
                v4355 = v4298;
                v4356 = v4311;
                v4357 = v4315;
            } else {
                let v4316 = v4278.sqrt();
                let v4319 = v4279 * (v24 / (v22 * v4316));
                let v4320 = v29 * v4316;
                let v4322 = v4320.sinh();
                let v4325 = v236 / v4322;
                let v4329 = v4325 * v4325;
                let v4330 = (((((v4319 * v29) * (v4320.cosh())) * v4325) * v54) / v4322) * v4325;
                let v4331 = v4330 + v4330;
                let v4333 = (v236 + v4329).sqrt();
                let v4336 = v4331 * (v24 / (v22 * v4333));
                let v4339 = (v29 * v4333) / v4316;
                let v4342 = ((v4336 * v29) - (v4319 * v4339)) / v4316;
                let v4346 = (v4343 * v4329) + v4339;
                let v4347 = (v4331 * v4343) + v4342;
                v4348 = v4316;
                v4349 = v4333;
                v4350 = v4329;
                v4351 = v4339;
                v4352 = v4346;
                v4353 = v4319;
                v4354 = v4336;
                v4355 = v4331;
                v4356 = v4342;
                v4357 = v4347;
            }
            let v4362 = v4266 + (v4348 * v4349);
            let v4363 = v4267 + ((v4353 * v4349) + (v4354 * v4348));
            let v4364 = v236 / v4362;
            let v4367 = ((v4363 * v4364) * v54) / v4362;
            let v4370 = v4278 * v4350;
            let v4374 = v4370 * v4364;
            let v4378 = v4374 * v4364;
            let v4382 = v4378.abs();
            let v4390 = (v3757 + v4264) - (v4382.ln());
            let v4391 = (v3758 + v4265) - ((((((((v4279 * v4350) + (v4355 * v4278)) * v4364) + (v4367 * v4370)) * v4364) + (v4367 * v4374)) * ((v22 * (if v4378 >= v1441 { 1.0 } else { 0.0 })) - v24)) * (v24 / v4382));
            let v4394 = (v1217 * v4390) + v4266;
            let v4402 = v236 / v4278;
            let v4406 = v4402 - v4351;
            let v4411 = (v4408 * v4266) + v4270;
            let v4412 = (v4267 * v4408) + v4274;
            let v4413 = v4352 * v4411;
            let v4416 = (v4357 * v4411) + (v4412 * v4352);
            let v4417 = v1786 + v4413;
            let v4430 = (v4424 + (v70 * (v4417 * v4364))) - (v4406 * v4411);
            let v4448 = v4413 - v1242;
            let v4457 = ((v4270 - (v1242 * (v4266 + v4362))) + (v4266 * v4413)) + (v1217 * ((v4430 * v4362) + (v4390 * v4448)));
            let v4461 = (-(v4270 + (v4362 * v4394))) / v4457;
            let v4465 = v4262 + v4461;
            let v4466 = v4263 + ((((v4274 + ((v4363 * v4394) + (((v4391 * v1217) + v4267) * v4362))) * v54) - ((((v4274 - ((v4267 + v4363) * v1242)) + ((v4267 * v4413) + (v4416 * v4266))) + ((((((((v4416 * v4364) + (v4367 * v4417)) * v70) - ((((((v4279 * v4402) * v54) / v4278) - v4356) * v4411) + (v4412 * v4406))) * v4362) + (v4363 * v4430)) + ((v4391 * v4448) + (v4416 * v4390))) * v1217)) * v4461)) / v4457);
            let v4467 = v3218 - v4465;
            let v4468 = v3221 - v4466;
            let v4469 = v1242 * v4467;
            let v4470 = v4468 * v1242;
            let v4471 = v4465.exp();
            let v4473 = v1394 * v4471;
            let v4477 = (Lanes([0.0, (v1395 * v4471), 0.0, 0.0, 0.0])) + ((v4466 * v4471) * v1394);
            let v4479 = v4470 * v4469;
            let v4481 = (v4469 * v4469) + v4473;
            let v4482 = (v4479 + v4479) + v4477;
            let v4483 = if v4481 < v643 { 1.0 } else { 0.0 };
            let v4551: f64;
            let v4552: f64;
            let v4553: f64;
            let v4554: f64;
            let v4555: f64;
            let v4556: Lanes<5>;
            let v4557: Lanes<5>;
            let v4558: Lanes<5>;
            let v4559: Lanes<5>;
            let v4560: Lanes<5>;
            if v4483 != 0.0 {
                let v4486 = (-v4481).sqrt();
                let v4489 = (v4482 * v54) * (v24 / (v22 * v4486));
                let v4490 = v29 * v4486;
                let v4491 = v4489 * v29;
                let v4492 = v4490.sin();
                let v4493 = v4490.cos();
                let v4495 = v236 / v4492;
                let v4498 = (((v4491 * v4493) * v4495) * v54) / v4492;
                let v4499 = v4495 * v4495;
                let v4500 = v4498 * v4495;
                let v4501 = v4500 + v4500;
                let v4504 = v4493 * v4495;
                let v4507 = ((v4491 * (v54 * v4492)) * v4495) + (v4498 * v4493);
                let v4511 = (v4508 * v4504) / v4486;
                let v4514 = ((v4507 * v4508) - (v4489 * v4511)) / v4486;
                let v4517 = (v1678 * v4499) + v4511;
                let v4518 = (v4501 * v1678) + v4514;
                v4551 = v4486;
                v4552 = v4504;
                v4553 = v4499;
                v4554 = v4511;
                v4555 = v4517;
                v4556 = v4489;
                v4557 = v4507;
                v4558 = v4501;
                v4559 = v4514;
                v4560 = v4518;
            } else {
                let v4519 = v4481.sqrt();
                let v4522 = v4482 * (v24 / (v22 * v4519));
                let v4523 = v29 * v4519;
                let v4525 = v4523.sinh();
                let v4528 = v236 / v4525;
                let v4532 = v4528 * v4528;
                let v4533 = (((((v4522 * v29) * (v4523.cosh())) * v4528) * v54) / v4525) * v4528;
                let v4534 = v4533 + v4533;
                let v4536 = (v236 + v4532).sqrt();
                let v4539 = v4534 * (v24 / (v22 * v4536));
                let v4542 = (v29 * v4536) / v4519;
                let v4545 = ((v4539 * v29) - (v4522 * v4542)) / v4519;
                let v4549 = (v4546 * v4532) + v4542;
                let v4550 = (v4534 * v4546) + v4545;
                v4551 = v4519;
                v4552 = v4536;
                v4553 = v4532;
                v4554 = v4542;
                v4555 = v4549;
                v4556 = v4522;
                v4557 = v4539;
                v4558 = v4534;
                v4559 = v4545;
                v4560 = v4550;
            }
            let v4565 = v4469 + (v4551 * v4552);
            let v4566 = v4470 + ((v4556 * v4552) + (v4557 * v4551));
            let v4567 = v236 / v4565;
            let v4570 = ((v4566 * v4567) * v54) / v4565;
            let v4573 = v4481 * v4553;
            let v4577 = v4573 * v4567;
            let v4581 = v4577 * v4567;
            let v4585 = v4581.abs();
            let v4593 = (v3757 + v4467) - (v4585.ln());
            let v4594 = (v3758 + v4468) - ((((((((v4482 * v4553) + (v4558 * v4481)) * v4567) + (v4570 * v4573)) * v4567) + (v4570 * v4577)) * ((v22 * (if v4581 >= v1441 { 1.0 } else { 0.0 })) - v24)) * (v24 / v4585));
            let v4597 = (v1217 * v4593) + v4469;
            let v4605 = v236 / v4481;
            let v4609 = v4605 - v4554;
            let v4614 = (v4611 * v4469) + v4473;
            let v4615 = (v4470 * v4611) + v4477;
            let v4616 = v4555 * v4614;
            let v4619 = (v4560 * v4614) + (v4615 * v4555);
            let v4620 = v1786 + v4616;
            let v4633 = (v4627 + (v70 * (v4620 * v4567))) - (v4609 * v4614);
            let v4651 = v4616 - v1242;
            let v4660 = ((v4473 - (v1242 * (v4469 + v4565))) + (v4469 * v4616)) + (v1217 * ((v4633 * v4565) + (v4593 * v4651)));
            let v4664 = (-(v4473 + (v4565 * v4597))) / v4660;
            let v4668 = v4465 + v4664;
            let v4669 = v4466 + ((((v4477 + ((v4566 * v4597) + (((v4594 * v1217) + v4470) * v4565))) * v54) - ((((v4477 - ((v4470 + v4566) * v1242)) + ((v4470 * v4616) + (v4619 * v4469))) + ((((((((v4619 * v4567) + (v4570 * v4620)) * v70) - ((((((v4482 * v4605) * v54) / v4481) - v4559) * v4614) + (v4615 * v4609))) * v4565) + (v4566 * v4633)) + ((v4594 * v4651) + (v4619 * v4593))) * v1217)) * v4664)) / v4660);
            let v4670 = v3218 - v4668;
            let v4671 = v3221 - v4669;
            let v4672 = v4668.exp();
            let v4674 = v1177 * v4672;
            let v4678 = (Lanes([0.0, (v1180 * v4672), 0.0, 0.0, 0.0])) + ((v4669 * v4672) * v1177);
            let v4679 = v1202 * v4670;
            let v4685 = (v4679 * v4670) - v4674;
            let v4686 = (((v4671 * v1202) * v4670) + (v4671 * v4679)) - v4678;
            let v4687 = if v4685 < v643 { 1.0 } else { 0.0 };
            let v4733: f64;
            let v4734: f64;
            let v4735: Lanes<5>;
            let v4736: Lanes<5>;
            if v4687 != 0.0 {
                let v4690 = (-v4685).sqrt();
                let v4693 = (v4686 * v54) * (v24 / (v22 * v4690));
                let v4694 = v29 * v4690;
                let v4695 = v4693 * v29;
                let v4696 = v4694.tan();
                let v4697 = v4694.cos();
                let v4701 = v4690 / v4696;
                let v4704 = (v4693 - ((v4695 * (v24 / (v4697 * v4697))) * v4701)) / v4696;
                let v4705 = v4694.sin();
                let v4706 = v4695 * v4697;
                let v4707 = -v4705;
                let v4709 = v4707 * v4705;
                let v4712 = ((v4706 * v54) * v4705) + (v4706 * v4707);
                v4733 = v4701;
                v4734 = v4709;
                v4735 = v4704;
                v4736 = v4712;
            } else {
                let v4713 = v4685.sqrt();
                let v4716 = v4686 * (v24 / (v22 * v4713));
                let v4717 = v29 * v4713;
                let v4718 = v4716 * v29;
                let v4719 = v4717.sinh();
                let v4722 = v4719 * v4719;
                let v4723 = (v4718 * (v4717.cosh())) * v4719;
                let v4724 = v4723 + v4723;
                let v4725 = v4717.tanh();
                let v4729 = v4713 / v4725;
                let v4732 = (v4716 - ((v4718 * (v24 - (v4725 * v4725))) * v4729)) / v4725;
                v4733 = v4729;
                v4734 = v4722;
                v4735 = v4732;
                v4736 = v4724;
            }
            let v4741 = v4734 * v4674;
            let v4745 = v4685 / v4741;
            let v4749 = v236 - v4745;
            let v4751 = ((v1242 * v4670) - v4733) / v4749;
            let v4754 = (((v4671 * v1242) - v4735) - ((((v4686 - (((v4736 * v4674) + (v4678 * v4734)) * v4745)) / v4741) * v54) * v4751)) / v4749;
            let v4755 = v4670 * v2734;
            let v4757 = v4755 * v1132;
            let v4760 = ((v4671 * v2734) * v1132) + (v1133 * v4755);
            let v4761 = v4751 * v2741;
            let v4763 = v4761 * v1132;
            let v4766 = ((v4754 * v2741) * v1132) + (v1133 * v4761);
            let v4767 = v4763 - v4757;
            let v4768 = v4766 - v4760;
            let v4769 = v4763 / v2734;
            let v4770 = v4766 / v2734;
            let v4773 = v29 * (v2767 + v4769);
            let v4774 = (v2768 + v4770) * v29;
            let v4775 = v2767 - v4769;
            let v4776 = v2768 - v4770;
            let v4777 = v3214 * v3214;
            let v4781 = v4777 / v4780;
            let v4782 = (v3215 * (v70 * v3214)) / v4780;
            let v4816: f64;
            let v4817: Lanes<5>;
            if v4783 != 0.0 {
                let v4789 = -v4781;
                let v4799 = (v4796 * (v236 - (rspice_limited_exp(v4789)))) * v29;
                let v4801 = v2737 - v4757;
                let v4809 = ((v2737 + v4757) / v4786) + ((v4799 * v4801) / v2734);
                let v4810 = ((v2740 + v4760) / v4786) + ((((((((v4782 * v54) * (rspice_limited_exp_derivative(v4789))) * v54) * v4796) * v29) * v4801) + ((v2740 - v4760) * v4799)) / v2734);
                v4816 = v4809;
                v4817 = v4810;
            } else {
                let v4814 = (v2737 + v4757) / v4813;
                let v4815 = (v2740 + v4760) / v4813;
                v4816 = v4814;
                v4817 = v4815;
            }
            let v4850: f64;
            let v4851: Lanes<5>;
            if v4818 != 0.0 {
                let v4824 = -v4781;
                let v4834 = (v4831 * (v236 - (rspice_limited_exp(v4824)))) * v29;
                let v4836 = v2748 - v4767;
                let v4844 = ((v2748 + v4767) / v4821) + ((v4834 * v4836) / v2750);
                let v4845 = ((v2749 + v4768) / v4821) + ((((((((v4782 * v54) * (rspice_limited_exp_derivative(v4824))) * v54) * v4831) * v29) * v4836) + ((v2749 - v4768) * v4834)) / v2750);
                v4850 = v4844;
                v4851 = v4845;
            } else {
                let v4848 = (v2748 + v4767) / v4821;
                let v4849 = (v2749 + v4768) / v4821;
                v4850 = v4848;
                v4851 = v4849;
            }
            let v4853 = v4817 * v2769;
            let v4854 = (v2769 * v4816) + v2774;
            let v4856 = v4853 * v4854;
            let v4859 = ((v4854 * v4854) + v2779).sqrt();
            let v4867 = v2789 * (v29 * (v4854 + v4859));
            let v4870 = v4851 * v2792;
            let v4871 = (v2792 * v4850) + v2774;
            let v4873 = v4870 * v4871;
            let v4876 = ((v4871 * v4871) + v2779).sqrt();
            let v4884 = v2810 * (v29 * (v4871 + v4876));
            let v4886 = v4773 / v2813;
            let v4894 = v29 * (v236 + (v4886.abs()));
            let v4895 = ((v4774 / v2813) * ((v22 * (if v4886 >= v1441 { 1.0 } else { 0.0 })) - v24)) * v29;
            let v4896 = v4894.powf(v303);
            let v4906 = v694 * v264;
            let v4911 = v285 + (v692 * v264);
            let v4913 = v4867.abs();
            let v4920 = v2851 + (v2848 * v692);
            let v4921 = v4913.powf(v4920);
            let v4928 = (v694 * v2848) * (v4921 * (v4913.ln()));
            let v4932 = (v2841 + ((Lanes([v4906[0], 0.0, v4906[1], v4906[2]])) + (Lanes([0.0, (v265 * v692), 0.0, 0.0])))) * v4921;
            let v4938 = v694 * v4936;
            let v4942 = (Lanes([0.0, v295, 0.0, 0.0])) + (Lanes([v4938[0], 0.0, v4938[1], v4938[2]]));
            let v4943 = (v294 + (v692 * v4936)) / v4896;
            let v4949 = ((Lanes([v4932[0], v4932[1], v4932[2], v4932[3], 0.0])) + (((((((v4853 + ((v4856 + v4856) * (v24 / (v22 * v4859)))) * v29) * v2789) * ((v22 * (if v4867 >= v1441 { 1.0 } else { 0.0 })) - v24)) * (v4920 * (v4913.powf((v4920 - v24))))) + (Lanes([v4928[0], 0.0, v4928[1], v4928[2], 0.0]))) * v4911)) + (((Lanes([v4942[0], v4942[1], v4942[2], v4942[3], 0.0])) - (((v4895 * (v303 * (v4894.powf(v2825)))) + (Lanes([0.0, (v304 * (v4896 * (v4894.ln()))), 0.0, 0.0, 0.0]))) * v4943)) / v4896);
            let v4950 = v236 + ((v4911 * v4921) + v4943);
            let v4952 = v4950 - v236;
            let v4954 = v4949 * v4952;
            let v4957 = ((v4952 * v4952) + v2881).sqrt();
            let v4965 = (v29 * ((v4950 + v236) + v4957)) / v2891;
            let v4967 = v240 / v4965;
            let v4971 = v4894.powf(v2899);
            let v4977 = v2908 + (v692 * v2905);
            let v4978 = v4884.abs();
            let v4985 = v2918 + (v2915 * v692);
            let v4986 = v4978.powf(v4985);
            let v4993 = (v694 * v2915) * (v4986 * (v4978.ln()));
            let v4997 = (v694 * v2905) * v4986;
            let v5003 = v694 * v5001;
            let v5005 = (v2935 + (v692 * v5001)) / v4971;
            let v5011 = ((Lanes([v4997[0], 0.0, v4997[1], v4997[2], 0.0])) + (((((((v4870 + ((v4873 + v4873) * (v24 / (v22 * v4876)))) * v29) * v2810) * ((v22 * (if v4884 >= v1441 { 1.0 } else { 0.0 })) - v24)) * (v4985 * (v4978.powf((v4985 - v24))))) + (Lanes([v4993[0], 0.0, v4993[1], v4993[2], 0.0]))) * v4977)) + (((Lanes([v5003[0], 0.0, v5003[1], v5003[2], 0.0])) - ((v4895 * (v2899 * (v4894.powf(v2901)))) * v5005)) / v4971);
            let v5012 = v236 + ((v4977 * v4986) + v5005);
            let v5014 = v5012 - v236;
            let v5016 = v5011 * v5014;
            let v5019 = ((v5014 * v5014) + v2881).sqrt();
            let v5027 = (v29 * ((v5012 + v236) + v5019)) / v2891;
            let v5029 = v2959 / v5027;
            let v5033 = v2737 + v4757;
            let v5034 = v2740 + v4760;
            let v5040 = v2748 + v4767;
            let v5041 = v2749 + v4768;
            let v5046 = (v1165 - (v5033 / v5035)) / v1132;
            let v5050 = v5046.exp();
            let v5051 = (((v1163 - (v5034 / v5035)) - (v1133 * v5046)) / v1132) * v5050;
            let v5052 = (v1191 - (v5040 / v4821)) / v1132;
            let v5056 = v5052.exp();
            let v5057 = (((v1193 - (v5041 / v4821)) - (v1133 * v5052)) / v1132) * v5056;
            let v5058 = v5050 + v5056;
            let v5059 = v5051 + v5057;
            let v5060 = v5050 / v5058;
            let v5064 = v5056 / v5058;
            let v5076 = (v5060 * v4967) + (v5064 * v5029);
            let v5077 = ((((v5051 - (v5059 * v5060)) / v5058) * v4967) + (((v2896 - ((((v4949 + ((v4954 + v4954) * (v24 / (v22 * v4957)))) * v29) / v2891) * v4967)) / v4965) * v5060)) + ((((v5057 - (v5059 * v5064)) / v5058) * v5029) + (((((((v5011 + ((v5016 + v5016) * (v24 / (v22 * v5019)))) * v29) / v2891) * v5029) * v54) / v5027) * v5064));
            let v5082 = ((v5076 * v2734) * v3114) / v3015;
            let v5083 = ((v5077 * v2734) * v3114) / v3015;
            let v5088 = v2789 * (v2774 + (v5084 * v4773));
            let v5090 = v5088.abs();
            let v5095 = v5090.powf(v2851);
            let v5104 = (Lanes([0.0, (v286 * v5095), 0.0, 0.0, 0.0])) + (((((v4774 * v5084) * v2789) * ((v22 * (if v5088 >= v1441 { 1.0 } else { 0.0 })) - v24)) * (v2851 * (v5090.powf(v5096)))) * v285);
            let v5105 = v236 + (v285 * v5095);
            let v5107 = v5105 - v236;
            let v5109 = v5104 * v5107;
            let v5112 = ((v5107 * v5107) + v2881).sqrt();
            let v5120 = (v29 * ((v5105 + v236) + v5112)) / v2891;
            let v5124 = (v70 * v375) / v5076;
            let v5129 = v5124 * v3015;
            let v5133 = v694 * v451;
            let v5136 = (Lanes([0.0, (v452 * v692), 0.0, 0.0])) + (Lanes([v5133[0], 0.0, v5133[1], v5133[2]]));
            let v5138 = v5137 + (v451 * v692);
            let v5140 = v5136 * v5138;
            let v5143 = ((v5138 * v5138) + v953).sqrt();
            let v5152 = v5151 + (v29 * (v5138 + v5143));
            let v5153 = v4775 / v5129;
            let v5157 = v5153 * v5152;
            let v5159 = ((v5136 + ((v5140 + v5140) * (v24 / (v22 * v5143)))) * v29) * v5153;
            let v5163 = ((((v4776 - (((((Lanes([0.0, (v376 * v70), 0.0, 0.0, 0.0])) - (v5077 * v5124)) / v5076) * v3015) * v5153)) / v5129) * v5152) + (Lanes([v5159[0], v5159[1], v5159[2], v5159[3], 0.0]))) * v5157;
            let v5167 = (v5165 + (v5157 * v5157)).sqrt();
            let v5177 = v1003 * v5175;
            let v5184 = v694 * v5182;
            let v5188 = v29 * ((v527 - (v5175 * v1002)) - (v5182 * v692));
            let v5190 = v5188 * v4773;
            let v5191 = ((((Lanes([0.0, v528, 0.0, 0.0])) - (Lanes([v5177[0], 0.0, v5177[1], v5177[2]]))) - (Lanes([v5184[0], 0.0, v5184[1], v5184[2]]))) * v29) * v4773;
            let v5195 = v5190 * v4775;
            let v5203 = ((v236 + v5167) / v5172) + (v5195 * v4775);
            let v5204 = (((v5163 + v5163) * (v24 / (v22 * v5167))) / v5172) + ((((((Lanes([v5191[0], v5191[1], v5191[2], v5191[3], 0.0])) + (v4774 * v5188)) * v4775) + (v4776 * v5190)) * v4775) + (v4776 * v5195));
            let v5206 = v5203 - v236;
            let v5208 = v5204 * v5206;
            let v5212 = ((v5206 * v5206) + v5210).sqrt();
            let v5218 = v29 * ((v5203 + v236) + v5212);
            let v5219 = (v5204 + ((v5208 + v5208) * (v24 / (v22 * v5212)))) * v29;
            let v5220 = v70 * v394;
            let v5227 = (v5220 * v5120) / v240;
            let v5233 = v5227 * v5232;
            let v5234 = ((((Lanes([0.0, ((v395 * v70) * v5120), 0.0, 0.0, 0.0])) + ((((v5104 + ((v5109 + v5109) * (v24 / (v22 * v5112)))) * v29) / v2891) * v5220)) - (Lanes([0.0, (v243 * v5227), 0.0, 0.0, 0.0]))) / v240) * v5232;
            let v5256: f64;
            let v5257: Lanes<5>;
            if v5235 != 0.0 {
                let v5239 = (v5236 * v4773) / v3016;
                let v5242 = ((v4774 * v5236) - (v3017 * v5239)) / v3016;
                let v5243 = v236 + v5239;
                v5256 = v5243;
                v5257 = v5242;
            } else {
                let v5246 = (v5236 * v4773) / v3016;
                let v5250 = v236 - v5246;
                let v5252 = v236 / v5250;
                let v5255 = ((((((v4774 * v5236) - (v3017 * v5246)) / v3016) * v54) * v5252) * v54) / v5250;
                v5256 = v5252;
                v5257 = v5255;
            }
            let v5258 = v654 - v3214;
            let v5259 = v3189 - v3215;
            let v5260 = v4773 + v71;
            let v5262 = v4774 + (Lanes([0.0, v72, 0.0, 0.0, 0.0]));
            let v5263 = if v840 > v643 { 1.0 } else { 0.0 };
            let v5287: f64;
            let v5288: Lanes<5>;
            if v5263 != 0.0 {
                let v5264 = v3186 + v5260;
                let v5266 = v5260 / v5264;
                let v5270 = v5260 / v840;
                let v5274 = v5270 * v5266;
                let v5278 = v5274 * v5256;
                let v5282 = v5258 / v5278;
                let v5285 = (v5259 - (((((((v5262 - (v841 * v5270)) / v840) * v5266) + (((v5262 - ((v3185 + v5262) * v5266)) / v5264) * v5270)) * v5256) + (v5257 * v5274)) * v5282)) / v5278;
                let v5286 = v236 + v5282;
                v5287 = v5286;
                v5288 = v5285;
            } else {
                v5287 = v236;
                v5288 = v3004;
            }
            let v5291: f64;
            let v5292: Lanes<5>;
            if v5289 != 0.0 {
                let v5314: f64;
                let v5315: Lanes<5>;
                if v5290 != 0.0 {
                    let v5302 = v5301 - (v5298 * v4773);
                    let v5304 = v236 / v5302;
                    let v5307 = ((((v4774 * v5298) * v54) * v5304) * v54) / v5302;
                    v5314 = v5304;
                    v5315 = v5307;
                } else {
                    let v5312 = v5311 * (v236 + (v5298 * v4773));
                    let v5313 = (v4774 * v5298) * v5311;
                    v5314 = v5312;
                    v5315 = v5313;
                }
                let v5316 = v5258 / v5314;
                let v5320 = v3186 + v3016;
                let v5322 = v5316 / v5320;
                let v5326 = v236 + v5322;
                let v5327 = if v5326 >= v95 { v5326 } else { v95 };
                let v5330 = v5327.ln();
                let v5336 = (v5315 * v5330) + (((((((v5259 - (v5315 * v5316)) / v5314) - ((v3185 + v3017) * v5322)) / v5320) * (if v5326 >= v95 { 1.0 } else { 0.0 })) * (v24 / v5327)) * v5314);
                let v5337 = v236 + (v5314 * v5330);
                v5291 = v5337;
                v5292 = v5336;
            } else {
                v5291 = v236;
                v5292 = v3004;
            }
            let v5293 = v5287 * v5291;
            let v5296 = (v5288 * v5291) + (v5292 * v5287);
            let v5357: f64;
            let v5358: Lanes<5>;
            if v5297 != 0.0 {
                let v5341 = v3186 + v5233;
                let v5343 = (v5258 / v5338) / v5341;
                let v5347 = v236 + v5343;
                let v5348 = if v5347 >= v95 { v5347 } else { v95 };
                let v5355 = (((((v5259 / v5338) - ((v3185 + v5234) * v5343)) / v5341) * (if v5347 >= v95 { 1.0 } else { 0.0 })) * (v24 / v5348)) * v5338;
                let v5356 = v236 + (v5338 * (v5348.ln()));
                v5357 = v5356;
                v5358 = v5355;
            } else {
                v5357 = v236;
                v5358 = v3004;
            }
            let v5359 = if v479 != v643 { 1.0 } else { 0.0 };
            let v5394: f64;
            let v5395: Lanes<5>;
            if v5359 != 0.0 {
                let v5360 = v506 * v4775;
                let v5369 = v501 + (v5360 * v4775);
                let v5372 = if v643 >= v5369 { v643 } else { v5369 };
                let v5382 = (v5372 * v4773) + (v70 * v1132);
                let v5384 = v479 / v5382;
                let v5389 = -v5384;
                let v5391 = rspice_limited_exp(v5389);
                let v5393 = ((((Lanes([0.0, v477, 0.0, 0.0, 0.0])) - (((((((Lanes([0.0, v498, 0.0, 0.0, 0.0])) + ((((Lanes([0.0, (v504 * v4775), 0.0, 0.0, 0.0])) + (v4776 * v506)) * v4775) + (v4776 * v5360))) * (v24 - (if v643 >= v5369 { 1.0 } else { 0.0 }))) * v4773) + (v4774 * v5372)) + (v1133 * v70)) * v5384)) / v5382) * v54) * (rspice_limited_exp_derivative(v5389));
                v5394 = v5391;
                v5395 = v5393;
            } else {
                v5394 = v236;
                v5395 = v3004;
            }
            let v5396 = v2730 - v4751;
            let v5399 = v2733 * v2730;
            let v5402 = v4754 * v4751;
            let v5404 = (v2730 * v2730) - (v4751 * v4751);
            let v5406 = v2741 * v1132;
            let v5407 = v1133 * v2741;
            let v5408 = v5406 * v70;
            let v5410 = v5408 * v37;
            let v5419 = v5406 * v2741;
            let v5425 = (v5419 * v1132) * v29;
            let v5433 = (v5410 * v5396) + ((v5425 * v5404) / v2734);
            let v5434 = (((((v5407 * v70) * v37) + (Lanes([0.0, (v38 * v5408), 0.0, 0.0, 0.0]))) * v5396) + ((v2733 - v4754) * v5410)) + (((((((v5407 * v2741) * v1132) + (v1133 * v5419)) * v29) * v5404) + (((v5399 + v5399) - (v5402 + v5402)) * v5425)) / v2734);
            let v5435 = v4773 + v37;
            let v5437 = v4774 + (Lanes([0.0, v38, 0.0, 0.0, 0.0]));
            let v5607: f64;
            let v5608: f64;
            let v5609: f64;
            let v5610: Lanes<5>;
            let v5611: Lanes<4>;
            let v5612: Lanes<4>;
            if v0 != 0.0 {
                let v5438 = v607 - v203;
                let v5441 = (Lanes([0.0, v608[0], v608[1]])) - (Lanes([v204, 0.0, 0.0]));
                let v5443 = v5441 * v5438;
                let v5446 = ((v5438 * v5438) + v401).sqrt();
                let v5456 = v236 + (v3034 * (v29 * (v5438 + v5446)));
                let v5457 = v236 / v5456;
                let v5460 = (((((v5441 + ((v5443 + v5443) * (v24 / (v22 * v5446)))) * v29) * v3034) * v5457) * v54) / v5456;
                let v5465 = (v630 * v29) * v5463;
                let v5466 = v5457 - ((v29 * v629) * v5463);
                let v5469 = (Lanes([0.0, v5460[0], v5460[1], v5460[2]])) - (Lanes([v5465[0], 0.0, v5465[1], 0.0]));
                let v5471 = v5469 * v5466;
                let v5474 = ((v5466 * v5466) + v953).sqrt();
                let v5490 = v5489 + ((v5485 + (v5482 * (v29 * (v5466 + v5474)))) * v3059);
                let v5491 = v321 * v5490;
                let v5495 = (Lanes([0.0, (v322 * v5490), 0.0, 0.0])) + (((((v5469 + ((v5471 + v5471) * (v24 / (v22 * v5474)))) * v29) * v5482) * v3059) * v321);
                let v5496 = v621 - v203;
                let v5499 = (Lanes([0.0, v622[0], v622[1]])) - (Lanes([v204, 0.0, 0.0]));
                let v5501 = v5499 * v5496;
                let v5504 = ((v5496 * v5496) + v401).sqrt();
                let v5514 = v236 + (v3034 * (v29 * (v5496 + v5504)));
                let v5515 = v236 / v5514;
                let v5518 = (((((v5499 + ((v5501 + v5501) * (v24 / (v22 * v5504)))) * v29) * v3034) * v5515) * v54) / v5514;
                let v5522 = (v636 * v29) * v5463;
                let v5523 = v5515 - ((v29 * v635) * v5463);
                let v5526 = (Lanes([0.0, v5518[0], v5518[1], v5518[2]])) - (Lanes([v5522[0], 0.0, v5522[1], 0.0]));
                let v5528 = v5526 * v5523;
                let v5531 = ((v5523 * v5523) + v953).sqrt();
                let v5547 = v5546 + ((v5542 + (v5539 * (v29 * (v5523 + v5531)))) * v3059);
                let v5548 = v321 * v5547;
                let v5552 = (Lanes([0.0, (v322 * v5547), 0.0, 0.0])) + (((((v5526 + ((v5528 + v5528) * (v24 / (v22 * v5531)))) * v29) * v5539) * v3059) * v321);
                v5607 = v236;
                v5608 = v5548;
                v5609 = v5491;
                v5610 = v3004;
                v5611 = v5552;
                v5612 = v5495;
            } else {
                let v5555 = v236 + (v3034 * v4773);
                let v5556 = v236 / v5555;
                let v5565 = ((v662 + v660) * v29) * v5463;
                let v5566 = v5556 - ((v29 * (v657 + v655)) * v5463);
                let v5568 = ((((v4774 * v3034) * v5556) * v54) / v5555) - (Lanes([v5565[0], 0.0, v5565[1], v5565[2], 0.0]));
                let v5570 = v5568 * v5566;
                let v5573 = ((v5566 * v5566) + v953).sqrt();
                let v5574 = v22 * v5573;
                let v5577 = v5566 + v5573;
                let v5584 = (v3057 + (v3054 * (v29 * v5577))) * v3059;
                let v5586 = v321 * v5584;
                let v5591 = v3062 * v5082;
                let v5597 = (v5591 * v5435) / v5218;
                let v5600 = ((((v5083 * v3062) * v5435) + (v5437 * v5591)) - (v5219 * v5597)) / v5218;
                let v5604 = (v5600 * v5586) + (((Lanes([0.0, (v322 * v5584), 0.0, 0.0, 0.0])) + (((((v5568 + ((v5570 + v5570) * (v24 / v5574))) * v29) * v3054) * v3059) * v321)) * v5597);
                let v5605 = v236 + (v5597 * v5586);
                let v5678: f64;
                let v5679: Lanes<5>;
                if v5606 != 0.0 {
                    let v5665 = v5664 + (v3054 * (v29 * v5577));
                    let v5671 = (v321 * v5665) * v3059;
                    let v5676 = (v5600 * v5671) + ((((Lanes([0.0, (v322 * v5665), 0.0, 0.0, 0.0])) + ((((v5568 + ((v5570 + v5570) * (v24 / v5574))) * v29) * v3054) * v321)) * v3059) * v5597);
                    let v5677 = v236 + (v5597 * v5671);
                    v5678 = v5677;
                    v5679 = v5676;
                } else {
                    v5678 = v5605;
                    v5679 = v5604;
                }
                v5607 = v5678;
                v5608 = v5680;
                v5609 = v5681;
                v5610 = v5679;
                v5611 = v5682;
                v5612 = v5683;
            }
            let v5613 = v5082 / v2734;
            let v5615 = v5613 * v5433;
            let v5619 = v5615 * v5293;
            let v5627 = v5218 * v5607;
            let v5631 = (v5619 * v5394) / v5627;
            let v5635 = v3062 * v5631;
            let v5636 = (((((((((v5083 / v2734) * v5433) + (v5434 * v5613)) * v5293) + (v5296 * v5615)) * v5394) + (v5395 * v5619)) - (((v5219 * v5607) + (v5610 * v5218)) * v5631)) / v5627) * v3062;
            let v5637 = v5033 / v70;
            let v5638 = v5034 / v70;
            let v5644 = v5643 * (v2744 + (v70 * v4763));
            let v5645 = (v2747 + (v4766 * v70)) * v5643;
            let v5651 = v5650 * ((v70 * v2744) + v4763);
            let v5652 = ((v2747 * v70) + v4766) * v5650;
            let v5653 = v5040 / v70;
            let v5654 = v5041 / v70;
            let v5701: f64;
            if v5655 != 0.0 {
                let v5699 = v5698 / (v5696 + (((v1169 / (v236 + (((v4773 + v5684) / v5686).powf(v5688)))) * v5692) / v5694));
                v5701 = v5699;
            } else {
                v5701 = v5700;
            }
            let v5703 = v5702 / v5357;
            let v5706 = ((v5358 * v5703) * v54) / v5357;
            let v5707 = v5637 * v5703;
            let v5710 = (v5638 * v5703) + (v5706 * v5637);
            let v5711 = -v5644;
            let v5713 = v5711 * v5703;
            let v5716 = ((v5645 * v54) * v5703) + (v5706 * v5711);
            let v5717 = v5653 * v5703;
            let v5720 = (v5654 * v5703) + (v5706 * v5653);
            let v5721 = -v5651;
            let v5723 = v5721 * v5703;
            let v5726 = ((v5652 * v54) * v5703) + (v5706 * v5721);
            let v5729 = v674 * v5727;
            let v5732 = v668 * v5730;
            let v5733 = v675 - v203;
            let v5736 = (Lanes([0.0, v676[0], v676[1]])) - (Lanes([v204, 0.0, 0.0]));
            let v5749 = (((Lanes([v630[0], 0.0, v630[1]])) - (Lanes([0.0, v208, 0.0]))) * v5744) * v5747;
            let v5750 = (v5733 + v686) + ((v5744 * ((v629 - v207) - v5742)) * v5747);
            let v5751 = Lanes([0.0, v5736[0], v5736[1], v5736[2]]);
            let v5753 = v5751 + (Lanes([v5749[0], v5749[1], v5749[2], 0.0]));
            let v5755 = v5753 * v5750;
            let v5759 = ((v5750 * v5750) + v5757).sqrt();
            let v5765 = v29 * (v5750 - v5759);
            let v5766 = (v5753 - ((v5755 + v5755) * (v24 / (v22 * v5759)))) * v29;
            let v5776 = (v236 - ((v1605 * v5765) / v5771)).sqrt();
            let v5792 = v669 - v203;
            let v5795 = (Lanes([0.0, v670[0], v670[1]])) - (Lanes([v204, 0.0, 0.0]));
            let v5807 = (((Lanes([v636[0], 0.0, v636[1]])) - (Lanes([0.0, v208, 0.0]))) * v5744) * v5805;
            let v5808 = (v5792 + v686) + ((v5744 * ((v635 - v207) - v5801)) * v5805);
            let v5809 = Lanes([0.0, v5795[0], v5795[1], v5795[2]]);
            let v5811 = v5809 + (Lanes([v5807[0], v5807[1], v5807[2], 0.0]));
            let v5813 = v5811 * v5808;
            let v5817 = ((v5808 * v5808) + v5815).sqrt();
            let v5823 = v29 * (v5808 - v5817);
            let v5824 = (v5811 - ((v5813 + v5813) * (v24 / (v22 * v5817)))) * v29;
            let v5834 = (v236 - ((v1605 * v5823) / v5829)).sqrt();
            let v5852 = v674 * v5850;
            let v5855 = v668 * v5853;
            let v5856 = ((v5727 * v671) + (v5786 * ((v5733 - v5765) - (v5781 * (v5776 - v236))))) + (v5850 * v671);
            let v5858 = ((Lanes([0.0, 0.0, v5729[0], v5729[1]])) + (((v5751 - v5766) - (((((v5766 * v1605) / v5771) * v54) * (v24 / (v22 * v5776))) * v5781)) * v5786)) + (Lanes([0.0, 0.0, v5852[0], v5852[1]]));
            let v5859 = ((v5730 * v664) + (v5844 * ((v5792 - v5823) - (v5839 * (v5834 - v236))))) + (v5853 * v664);
            let v5861 = ((Lanes([0.0, 0.0, v5732[0], v5732[1]])) + (((v5809 - v5824) - (((((v5824 * v1605) / v5829) * v54) * (v24 / (v22 * v5834))) * v5839)) * v5844)) + (Lanes([0.0, 0.0, v5855[0], v5855[1]]));
            let v5865 = v5864 * (v600 - v623);
            let v5866 = (v627 - v626) * v5864;
            let v5870 = v5869 * (v609 - v623);
            let v5871 = (v633 - v632) * v5869;
            let v5874 = if v5873 != 0.0 || (if v540 <= v643 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v5878: f64;
            let v5879: Lanes<5>;
            if v5874 != 0.0 {
                v5878 = v643;
                v5879 = v3004;
            } else {
                let v5877 = if v5258 > (v540 / v5875) { 1.0 } else { 0.0 };
                let v5911: f64;
                let v5912: Lanes<5>;
                if v5877 != 0.0 {
                    let v5883 = (-v540) / v5258;
                    let v5889 = v5888 * v5258;
                    let v5891 = v5889 * v5635;
                    let v5895 = rspice_limited_exp(v5883);
                    let v5898 = v5891 * v5895;
                    let v5901 = ((((v5259 * v5888) * v5635) + (v5636 * v5889)) * v5895) + (((((Lanes([0.0, (v541 * v54), 0.0, 0.0, 0.0])) - (v5259 * v5883)) / v5258) * (rspice_limited_exp_derivative(v5883))) * v5891);
                    v5911 = v5898;
                    v5912 = v5901;
                } else {
                    let v5902 = v5888 * v5258;
                    let v5909 = (v5902 * v5635) * v5908;
                    let v5910 = (((v5259 * v5888) * v5635) + (v5636 * v5902)) * v5908;
                    v5911 = v5909;
                    v5912 = v5910;
                }
                v5878 = v5911;
                v5879 = v5912;
            }
            let v6010: f64;
            let v6011: f64;
            let v6012: Lanes<5>;
            let v6013: Lanes<3>;
            if v5880 != 0.0 {
                let v5918 = ((v4773 - v5913) / v5915) / v37;
                let v5923 = v5915 * v37;
                let v5928 = v236 + (rspice_limited_exp(v5918));
                let v5929 = if v5928 >= v95 { v5928 } else { v95 };
                let v5932 = v5929.ln();
                let v5935 = v5923 * v5932;
                let v5949 = v236 + (v5946 * v4773);
                let v5951 = v5950 * (v5943 - (v5940 * v4773));
                let v5953 = v5951 * v5949;
                let v5957 = rspice_limited_exp(v5953);
                let v5961 = v5960 * v641;
                let v5963 = v5961 * v5935;
                let v5964 = (v642 * v5960) * v5935;
                let v5968 = v5963 * v5957;
                let v5972 = v5968 * v593;
                let v5976 = (((((Lanes([v5964[0], 0.0, 0.0, 0.0, v5964[1]])) + (((Lanes([0.0, ((v38 * v5915) * v5932), 0.0, 0.0, 0.0])) + (((((((v4774 / v5915) - (Lanes([0.0, (v38 * v5918), 0.0, 0.0, 0.0]))) / v37) * (rspice_limited_exp_derivative(v5918))) * (if v5928 >= v95 { 1.0 } else { 0.0 })) * (v24 / v5929)) * v5923)) * v5961)) * v5957) + (((((((v4774 * v5940) * v54) * v5950) * v5949) + ((v4774 * v5946) * v5951)) * (rspice_limited_exp_derivative(v5953))) * v5963)) * v593) + (Lanes([0.0, (v595 * v5968), 0.0, 0.0, 0.0]));
                let v5977 = v171 - v117;
                let v5978 = v172 - v120;
                let v5979 = v5977 - v641;
                let v5982 = (Lanes([0.0, v5978, 0.0])) - (Lanes([v642[0], 0.0, v642[1]]));
                let v5986 = (v5979 / v5983) / v37;
                let v5991 = v5983 * v37;
                let v5996 = v236 + (rspice_limited_exp(v5986));
                let v5997 = if v5996 >= v95 { v5996 } else { v95 };
                let v6000 = v5997.ln();
                let v6003 = v5991 * v6000;
                let v6007 = (Lanes([0.0, ((v38 * v5983) * v6000), 0.0])) + (((((((v5982 / v5983) - (Lanes([0.0, (v38 * v5986), 0.0]))) / v37) * (rspice_limited_exp_derivative(v5986))) * (if v5996 >= v95 { 1.0 } else { 0.0 })) * (v24 / v5997)) * v5991);
                let v6008 = if v5977 <= v643 { 1.0 } else { 0.0 };
                let v6081: f64;
                let v6082: Lanes<3>;
                if v6008 != 0.0 {
                    let v6046 = v5979 - v686;
                    let v6048 = v5982 * v6046;
                    let v6056 = ((v6046 * v6046) - (v6050 * v5977)).sqrt();
                    let v6062 = v29 * (v6046 + v6056);
                    let v6063 = (v5982 + (((v6048 + v6048) - (Lanes([0.0, (v5978 * v6050), 0.0]))) * (v24 / (v22 * v6056)))) * v29;
                    v6081 = v6062;
                    v6082 = v6063;
                } else {
                    let v6064 = v5979 - v686;
                    let v6066 = v5982 * v6064;
                    let v6073 = ((v6064 * v6064) + (v6050 * v5977)).sqrt();
                    let v6079 = v29 * (v6064 + v6073);
                    let v6080 = (v5982 + (((v6066 + v6066) + (Lanes([0.0, (v5978 * v6050), 0.0]))) * (v24 / (v22 * v6073)))) * v29;
                    v6081 = v6079;
                    v6082 = v6080;
                }
                let v6092 = v236 + (v6089 * v6081);
                let v6094 = v6093 * (v6086 - (v6083 * v6081));
                let v6096 = v6094 * v6092;
                let v6100 = rspice_limited_exp(v6096);
                let v6104 = v6103 * v641;
                let v6106 = v6104 * v6003;
                let v6107 = (v642 * v6103) * v6003;
                let v6111 = v6106 * v6100;
                let v6115 = v6111 * v593;
                let v6119 = (((((Lanes([v6107[0], 0.0, v6107[1]])) + (v6007 * v6104)) * v6100) + (((((((v6082 * v6083) * v54) * v6093) * v6092) + ((v6082 * v6089) * v6094)) * (rspice_limited_exp_derivative(v6096))) * v6106)) * v593) + (Lanes([0.0, (v595 * v6111), 0.0]));
                v6010 = v5972;
                v6011 = v6115;
                v6012 = v5976;
                v6013 = v6119;
            } else {
                v6010 = v643;
                v6011 = v643;
                v6012 = v3004;
                v6013 = v6009;
            }
            let v6016 = v616 * v6014;
            let v6017 = (v6014 * v615) / v37;
            let v6023 = v6017.tanh();
            let v6028 = ((((Lanes([0.0, v6016[0], v6016[1]])) - (Lanes([(v38 * v6017), 0.0, 0.0]))) / v37) * (v24 - (v6023 * v6023))) * v29;
            let v6029 = v29 + (v29 * v6023);
            let v6030 = v236 - v6029;
            let v6032 = v6010 + v6011;
            let v6034 = v6012 + (Lanes([v6013[0], v6013[1], 0.0, 0.0, v6013[2]]));
            let v6035 = v6029 * v6032;
            let v6036 = v6028 * v6032;
            let v6039 = (Lanes([0.0, v6036[0], v6036[1], v6036[2], 0.0])) + (v6034 * v6029);
            let v6040 = v6030 * v6032;
            let v6041 = (v6028 * v54) * v6032;
            let v6044 = (Lanes([0.0, v6041[0], v6041[1], v6041[2], 0.0])) + (v6034 * v6030);
            let v6270: f64;
            let v6271: f64;
            let v6272: f64;
            let v6273: f64;
            let v6274: Lanes<5>;
            let v6275: Lanes<5>;
            let v6276: Lanes<5>;
            let v6277: Lanes<5>;
            if v6045 != 0.0 {
                let v6123 = v695 - (v6120 * v2765);
                let v6124 = v1162 - (v2766 * v6120);
                let v6134 = v236 + (v6131 * v6123);
                let v6136 = v6135 * (v6128 - (v6125 * v6123));
                let v6138 = v6136 * v6134;
                let v6142 = rspice_limited_exp(v6138);
                let v6150 = v685 * v29;
                let v6160 = ((Lanes([v630[0], 0.0, v630[1]])) + (Lanes([v636[0], v636[1], 0.0]))) * v29;
                let v6161 = (v641 + (v29 * v687)) + (v29 * (v629 + v635));
                let v6165 = v6164 * (v4773 * v6142);
                let v6167 = v6165 * v6161;
                let v6169 = (((Lanes([v642[0], 0.0, 0.0, v642[1]])) + (Lanes([0.0, v6150[0], v6150[1], 0.0]))) + (Lanes([v6160[0], v6160[1], v6160[2], 0.0]))) * v6165;
                let v6172 = v6167 * v593;
                let v6176 = ((((((v4774 * v6142) + (((((((v6124 * v6125) * v54) * v6135) * v6134) + ((v6124 * v6131) * v6136)) * (rspice_limited_exp_derivative(v6138))) * v4773)) * v6164) * v6161) + (Lanes([v6169[0], 0.0, v6169[1], v6169[2], v6169[3]]))) * v593) + (Lanes([0.0, (v595 * v6167), 0.0, 0.0, 0.0]));
                let v6177 = v3215 * v3214;
                let v6180 = (v4777 + v953).sqrt();
                let v6187 = v6186 * (v6180 - v6184);
                let v6188 = ((v6177 + v6177) * (v24 / (v22 * v6180))) * v6186;
                let v6189 = -v6187;
                let v6191 = rspice_limited_exp(v6189);
                let v6193 = (v6188 * v54) * (rspice_limited_exp_derivative(v6189));
                let v6197 = ((v6187 + v6191) - v236) + v401;
                let v6198 = v6187 + v236;
                let v6205 = (v236 - (v6198 * v6191)) + v401;
                let v6207 = v6188 * v6187;
                let v6208 = v6207 + v6207;
                let v6210 = (v6187 * v6187) + v6209;
                let v6215 = (v6172 * v6205) / v6210;
                let v6218 = (((v6176 * v6205) + ((((v6188 * v6191) + (v6193 * v6198)) * v54) * v6172)) - (v6208 * v6215)) / v6210;
                let v6223 = (v6172 * v6197) / v6210;
                let v6226 = (((v6176 * v6197) + ((v6188 + v6193) * v6172)) - (v6208 * v6223)) / v6210;
                let v6230 = (Lanes([0.0, v608[0], v608[1]])) - (Lanes([v204, 0.0, 0.0]));
                let v6231 = v655 - v207;
                let v6233 = v700 - (Lanes([0.0, v208, 0.0, 0.0]));
                let v6236 = v6233 * v6234;
                let v6237 = (v607 - v203) + (v6234 * v6231);
                let v6242 = ((Lanes([0.0, v6230[0], 0.0, v6230[1], v6230[2]])) + (Lanes([v6236[0], v6236[1], v6236[2], v6236[3], 0.0]))) * v6237;
                let v6245 = ((v6237 * v6237) + v401).sqrt();
                let v6248 = (v6242 + v6242) * (v24 / (v22 * v6245));
                let v6258 = v236 + (v6255 * v6245);
                let v6260 = v6259 * (v6252 - (v6249 * v6245));
                let v6262 = v6260 * v6258;
                let v6266 = rspice_limited_exp(v6262);
                let v6268 = (((((v6248 * v6249) * v54) * v6259) * v6258) + ((v6248 * v6255) * v6260)) * (rspice_limited_exp_derivative(v6262));
                let v6269 = if v658 > v643 { 1.0 } else { 0.0 };
                let v6314: f64;
                let v6315: f64;
                let v6316: Lanes<5>;
                let v6317: Lanes<5>;
                if v6269 != 0.0 {
                    let v6280 = v597 * v6279;
                    let v6282 = v6280 * v607;
                    let v6284 = v608 * v6280;
                    let v6288 = v6282 * v6245;
                    let v6289 = ((Lanes([((v598 * v6279) * v607), 0.0, 0.0])) + (Lanes([0.0, v6284[0], v6284[1]]))) * v6245;
                    let v6293 = v6288 * v6266;
                    let v6296 = (((Lanes([0.0, v6289[0], 0.0, v6289[1], v6289[2]])) + (v6248 * v6282)) * v6266) + (v6268 * v6288);
                    v6314 = v6293;
                    v6315 = v643;
                    v6316 = v6296;
                    v6317 = v3004;
                } else {
                    let v6297 = v597 * v6279;
                    let v6299 = v6297 * v607;
                    let v6301 = v608 * v6297;
                    let v6305 = v6299 * v6245;
                    let v6306 = ((Lanes([((v598 * v6279) * v607), 0.0, 0.0])) + (Lanes([0.0, v6301[0], v6301[1]]))) * v6245;
                    let v6310 = v6305 * v6266;
                    let v6313 = (((Lanes([0.0, v6306[0], 0.0, v6306[1], v6306[2]])) + (v6248 * v6299)) * v6266) + (v6268 * v6305);
                    v6314 = v643;
                    v6315 = v6310;
                    v6316 = v3004;
                    v6317 = v6313;
                }
                let v6321 = (Lanes([0.0, v622[0], v622[1]])) - (Lanes([v204, 0.0, 0.0]));
                let v6324 = v6233 * v6322;
                let v6325 = (v621 - v203) + (v6322 * v6231);
                let v6330 = ((Lanes([0.0, v6321[0], v6321[1], 0.0, v6321[2]])) + (Lanes([v6324[0], v6324[1], v6324[2], v6324[3], 0.0]))) * v6325;
                let v6333 = ((v6325 * v6325) + v401).sqrt();
                let v6336 = (v6330 + v6330) * (v24 / (v22 * v6333));
                let v6346 = v236 + (v6343 * v6333);
                let v6347 = v6259 * (v6340 - (v6337 * v6333));
                let v6349 = v6347 * v6346;
                let v6353 = rspice_limited_exp(v6349);
                let v6355 = (((((v6336 * v6337) * v54) * v6259) * v6346) + ((v6336 * v6343) * v6347)) * (rspice_limited_exp_derivative(v6349));
                let v6391: f64;
                let v6392: f64;
                let v6393: Lanes<5>;
                let v6394: Lanes<5>;
                if v6269 != 0.0 {
                    let v6357 = v597 * v6356;
                    let v6359 = v6357 * v621;
                    let v6361 = v622 * v6357;
                    let v6365 = v6359 * v6333;
                    let v6366 = ((Lanes([((v598 * v6356) * v621), 0.0, 0.0])) + (Lanes([0.0, v6361[0], v6361[1]]))) * v6333;
                    let v6370 = v6365 * v6353;
                    let v6373 = (((Lanes([0.0, v6366[0], v6366[1], 0.0, v6366[2]])) + (v6336 * v6359)) * v6353) + (v6355 * v6365);
                    v6391 = v6314;
                    v6392 = v6370;
                    v6393 = v6316;
                    v6394 = v6373;
                } else {
                    let v6374 = v597 * v6356;
                    let v6376 = v6374 * v621;
                    let v6378 = v622 * v6374;
                    let v6382 = v6376 * v6333;
                    let v6383 = ((Lanes([((v598 * v6356) * v621), 0.0, 0.0])) + (Lanes([0.0, v6378[0], v6378[1]]))) * v6333;
                    let v6387 = v6382 * v6353;
                    let v6390 = (((Lanes([0.0, v6383[0], v6383[1], 0.0, v6383[2]])) + (v6336 * v6376)) * v6353) + (v6355 * v6382);
                    v6391 = v6387;
                    v6392 = v6315;
                    v6393 = v6390;
                    v6394 = v6317;
                }
                v6270 = v6215;
                v6271 = v6223;
                v6272 = v6391;
                v6273 = v6392;
                v6274 = v6218;
                v6275 = v6226;
                v6276 = v6393;
                v6277 = v6394;
            } else {
                v6270 = v643;
                v6271 = v643;
                v6272 = v643;
                v6273 = v643;
                v6274 = v3004;
                v6275 = v3004;
                v6276 = v3004;
                v6277 = v3004;
            }
            let v6398: f64;
            let v6399: f64;
            let v6400: Lanes<5>;
            let v6401: Lanes<5>;
            if v6278 != 0.0 {
                let v6397 = if v6396 != 0.0 || (if v561 <= v643 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v6477: f64;
                let v6478: Lanes<5>;
                if v6397 != 0.0 {
                    v6477 = v643;
                    v6478 = v3004;
                } else {
                    let v6405 = v622 * v54;
                    let v6411 = (Lanes([0.0, v6405[0], v6405[1]])) + (Lanes([v204, 0.0, 0.0]));
                    let v6413 = v6412 * v974;
                    let v6420 = (v700 - (Lanes([0.0, v208, 0.0, 0.0]))) * v6413;
                    let v6426 = ((((-v621) - v6406) + v203) + (v6413 * ((v655 - v207) - v6417))) / v6425;
                    let v6427 = ((Lanes([0.0, v6411[0], v6411[1], 0.0, v6411[2]])) + (Lanes([v6420[0], v6420[1], v6420[2], v6420[3], 0.0]))) / v6425;
                    let v6429 = v6427 * v6426;
                    let v6433 = ((v6426 * v6426) + v6431).sqrt();
                    let v6439 = v29 * (v6426 + v6433);
                    let v6440 = (v6427 + ((v6429 + v6429) * (v24 / (v22 * v6433)))) * v29;
                    let v6441 = v6439 + v2779;
                    let v6442 = v561 / v6441;
                    let v6447 = if v6439 >= v95 { v6439 } else { v95 };
                    let v6454 = v6453 * (v6447.ln());
                    let v6460 = v6459 * v3114;
                    let v6461 = v6460 * (rspice_limited_exp(v6454));
                    let v6463 = -v6442;
                    let v6465 = rspice_limited_exp(v6463);
                    let v6468 = v6461 * v6465;
                    let v6472 = v6468 * v615;
                    let v6474 = v616 * v6468;
                    let v6476 = ((((((((v6440 * (if v6439 >= v95 { 1.0 } else { 0.0 })) * (v24 / v6447)) * v6453) * (rspice_limited_exp_derivative(v6454))) * v6460) * v6465) + ((((((Lanes([0.0, v562, 0.0, 0.0, 0.0])) - (v6440 * v6442)) / v6441) * v54) * (rspice_limited_exp_derivative(v6463))) * v6461)) * v615) + (Lanes([0.0, 0.0, v6474[0], v6474[1], 0.0]));
                    v6477 = v6472;
                    v6478 = v6476;
                }
                let v6479 = if v658 > v643 { 1.0 } else { 0.0 };
                let v6480: f64;
                let v6481: f64;
                let v6482: Lanes<5>;
                let v6483: Lanes<5>;
                if v6479 != 0.0 {
                    v6480 = v6477;
                    v6481 = v643;
                    v6482 = v6478;
                    v6483 = v3004;
                } else {
                    v6480 = v643;
                    v6481 = v6477;
                    v6482 = v3004;
                    v6483 = v6478;
                }
                let v6486 = if v6485 != 0.0 || (if v582 <= v643 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v6562: f64;
                let v6563: Lanes<5>;
                if v6486 != 0.0 {
                    v6562 = v643;
                    v6563 = v3004;
                } else {
                    let v6488 = v608 * v54;
                    let v6494 = (Lanes([0.0, v6488[0], v6488[1]])) + (Lanes([v204, 0.0, 0.0]));
                    let v6496 = v6495 * v974;
                    let v6503 = (v700 - (Lanes([0.0, v208, 0.0, 0.0]))) * v6496;
                    let v6508 = ((((-v607) - v6489) + v203) + (v6496 * ((v655 - v207) - v6500))) / v6425;
                    let v6509 = ((Lanes([0.0, v6494[0], 0.0, v6494[1], v6494[2]])) + (Lanes([v6503[0], v6503[1], v6503[2], v6503[3], 0.0]))) / v6425;
                    let v6511 = v6509 * v6508;
                    let v6515 = ((v6508 * v6508) + v6513).sqrt();
                    let v6521 = v29 * (v6508 + v6515);
                    let v6522 = (v6509 + ((v6511 + v6511) * (v24 / (v22 * v6515)))) * v29;
                    let v6523 = v6521 + v2779;
                    let v6524 = v582 / v6523;
                    let v6529 = if v6521 >= v95 { v6521 } else { v95 };
                    let v6536 = v6535 * (v6529.ln());
                    let v6538 = rspice_limited_exp(v6536);
                    let v6546 = ((-v615) * v6543) * v3114;
                    let v6548 = v6546 * v6538;
                    let v6549 = (((v616 * v54) * v6543) * v3114) * v6538;
                    let v6553 = -v6524;
                    let v6555 = rspice_limited_exp(v6553);
                    let v6558 = v6548 * v6555;
                    let v6561 = (((Lanes([0.0, 0.0, v6549[0], v6549[1], 0.0])) + (((((v6522 * (if v6521 >= v95 { 1.0 } else { 0.0 })) * (v24 / v6529)) * v6535) * (rspice_limited_exp_derivative(v6536))) * v6546)) * v6555) + ((((((Lanes([0.0, v583, 0.0, 0.0, 0.0])) - (v6522 * v6524)) / v6523) * v54) * (rspice_limited_exp_derivative(v6553))) * v6548);
                    v6562 = v6558;
                    v6563 = v6561;
                }
                let v6564: f64;
                let v6565: f64;
                let v6566: Lanes<5>;
                let v6567: Lanes<5>;
                if v6479 != 0.0 {
                    v6564 = v6480;
                    v6565 = v6562;
                    v6566 = v6482;
                    v6567 = v6563;
                } else {
                    v6564 = v6562;
                    v6565 = v6481;
                    v6566 = v6563;
                    v6567 = v6483;
                }
                v6398 = v6564;
                v6399 = v6565;
                v6400 = v6566;
                v6401 = v6567;
            } else {
                v6398 = v643;
                v6399 = v643;
                v6400 = v3004;
                v6401 = v3004;
            }
            let v6402 = v3008 / v5076;
            if v6403 != 0.0 {
                let v6584: f64;
                if v6568 != 0.0 {
                    v6584 = v643;
                } else {
                    let v6582 = v6575 * ((if (((v5258 / v6575) + v6577) / v6402) >= v95 { (((v5258 / v6575) + v6577) / v6402) } else { v95 }).ln());
                    let v6583 = if v6582 < v643 { 1.0 } else { 0.0 };
                    let v6586: f64;
                    if v6583 != 0.0 {
                        v6586 = v643;
                    } else {
                        v6586 = v6582;
                    }
                    v6584 = v6586;
                }
                let v6605: f64;
                if v6585 != 0.0 {
                    let v6595 = (v6592 / (v236 + ((v4816 / v6587).powf(v6589)))) / v6594;
                    let v6597 = v6595 - v236;
                    let v6604 = v6594 * (v29 * ((v6595 + v236) + (((v6597 * v6597) + v6599).sqrt())));
                    v6605 = v6604;
                } else {
                    v6605 = v6594;
                }
                let v6617 = (v5701 * v2767) / v6616;
                let v6619 = (v5701 * v4769) / v6616;
                let v6623 = (v37 / v6616) * (v5701 + v6621);
                let v6625 = v6619 + v6623;
                let v6667 = if ((((((v6605 * v6616) * v37) / ((v6660 * v6623) * v6623)) * v5635) * v5635) + ((((((v6606 * v37) * (v5635.abs())) * v5076) / ((v6611 * v5701) * v6613)) * (((v6605 * ((if ((v6617 + v6623) / v6625) >= v95 { ((v6617 + v6623) / v6625) } else { v95 }).ln())) + (v6631 * (v6617 - v6619))) + (v6636 * ((v6617 * v6617) - (v6619 * v6619))))) + (((((((v6616 * v37) * v5635) * v5635) / v6652) * v6584) * ((v6605 + (v6631 * v6619)) + ((v6643 * v6619) * v6619))) / (v6625 * v6625)))) > v643 { 1.0 } else { 0.0 };
            } else {
            }
            let v6570 = v6569 * v5707;
            let v6571 = v5710 * v6569;
            let v6572 = v3062 * v5717;
            let v6573 = v5720 * v3062;
            let v6574 = if v658 > v643 { 1.0 } else { 0.0 };
            let v6672: f64;
            let v6673: Lanes<5>;
            if v6574 != 0.0 {
                let v6668 = v3062 * v5713;
                let v6669 = v5716 * v3062;
                v6672 = v6668;
                v6673 = v6669;
            } else {
                let v6670 = v3062 * v5723;
                let v6671 = v5726 * v3062;
                v6672 = v6670;
                v6673 = v6671;
            }
            let v6674 = v3062 * v5856;
            let v6675 = v5858 * v3062;
            let v6676 = v3062 * v5859;
            let v6677 = v5861 * v3062;
            let v6696: f64;
            let v6697: Lanes<5>;
            if v6678 != 0.0 {
                let v6684 = v6683 * v37;
                let v6694 = v6693 * ((v5082 * v4773) + (v6684 * v5082));
                let v6695 = (((v5083 * v4773) + (v4774 * v5082)) + ((Lanes([0.0, ((v38 * v6683) * v5082), 0.0, 0.0, 0.0])) + (v5083 * v6684))) * v6693;
                v6696 = v6694;
                v6697 = v6695;
            } else {
                v6696 = v643;
                v6697 = v3004;
            }
            let v6698 = v3062 * v6398;
            let v6699 = v6400 * v3062;
            let v6700 = v3062 * v6399;
            let v6701 = v6401 * v3062;
            let v6702 = v3062 * v6270;
            let v6703 = v6274 * v3062;
            let v6704 = v3062 * v6271;
            let v6705 = v6275 * v3062;
            let v6706 = v3062 * v6272;
            let v6707 = v6276 * v3062;
            let v6708 = v3062 * v6273;
            let v6709 = v6277 * v3062;
            let v6757: f64;
            let v6758: f64;
            let v6759: f64;
            let v6760: f64;
            let v6761: f64;
            let v6762: f64;
            let v6763: f64;
            let v6764: f64;
            let v6765: f64;
            let v6766: f64;
            let v6767: Lanes<5>;
            let v6768: Lanes<5>;
            let v6769: Lanes<5>;
            let v6770: Lanes<5>;
            let v6771: Lanes<5>;
            let v6772: Lanes<5>;
            let v6773: Lanes<5>;
            let v6774: Lanes<5>;
            let v6775: Lanes<5>;
            let v6776: Lanes<5>;
            if v6574 != 0.0 {
                let v6713 = ctx.simparam_or("gmin", v6712);
                let v6715 = v614 * v6713;
                let v6716 = (v170 * v5635) + (v6713 * v610);
                let v6718 = (v5636 * v170) + (Lanes([0.0, 0.0, v6715[0], v6715[1], 0.0]));
                let v6721 = v170 * (v6698 + v5878);
                let v6722 = (v6699 + v5879) * v170;
                let v6723 = v170 * v6700;
                let v6724 = v6701 * v170;
                let v6727 = v170 * (v6704 + v6706);
                let v6728 = (v6705 + v6707) * v170;
                let v6731 = v170 * (v6702 + v6708);
                let v6732 = (v6703 + v6709) * v170;
                v6757 = v6716;
                v6758 = v6721;
                v6759 = v6723;
                v6760 = v6727;
                v6761 = v6731;
                v6762 = v643;
                v6763 = v643;
                v6764 = v643;
                v6765 = v643;
                v6766 = v643;
                v6767 = v6718;
                v6768 = v6722;
                v6769 = v6724;
                v6770 = v6728;
                v6771 = v6732;
                v6772 = v3004;
                v6773 = v3004;
                v6774 = v3004;
                v6775 = v3004;
                v6776 = v3004;
            } else {
                let v6735 = ctx.simparam_or("gmin", v6712);
                let v6739 = (v613 - v612) * v6735;
                let v6740 = (v170 * v5635) + (v6735 * (v600 - v609));
                let v6742 = (v5636 * v170) + (Lanes([0.0, 0.0, v6739[0], v6739[1], 0.0]));
                let v6745 = v170 * (v6698 + v5878);
                let v6746 = (v6699 + v5879) * v170;
                let v6747 = v170 * v6700;
                let v6748 = v6701 * v170;
                let v6751 = v170 * (v6704 + v6706);
                let v6752 = (v6705 + v6707) * v170;
                let v6755 = v170 * (v6702 + v6708);
                let v6756 = (v6703 + v6709) * v170;
                v6757 = v643;
                v6758 = v643;
                v6759 = v643;
                v6760 = v643;
                v6761 = v643;
                v6762 = v6740;
                v6763 = v6745;
                v6764 = v6747;
                v6765 = v6751;
                v6766 = v6755;
                v6767 = v3004;
                v6768 = v3004;
                v6769 = v3004;
                v6770 = v3004;
                v6771 = v3004;
                v6772 = v6742;
                v6773 = v6746;
                v6774 = v6748;
                v6775 = v6752;
                v6776 = v6756;
            }
            let v6777 = v170 * v6035;
            let v6778 = v6039 * v170;
            let v6779 = v170 * v6040;
            let v6780 = v6044 * v170;
            let v6784 = v170 * (ddt(22330, v6672));
            let v6785 = (v6673 * v6782) * v170;
            let v6786 = v170 * v6672;
            let v6787 = v6673 * v170;
            let v6788 = ddt(22333, v6570);
            let v6789 = v6571 * v6782;
            let v6792 = v170 * (ddt(22336, v6572));
            let v6793 = (v6573 * v6782) * v170;
            let v6794 = v170 * v6572;
            let v6795 = v6573 * v170;
            let v6796 = ddt(22339, v6674);
            let v6797 = v6675 * v6782;
            let v6798 = ddt(22341, v6676);
            let v6799 = v6677 * v6782;
            let v6802 = v170 * (ddt(22344, v5865));
            let v6803 = (v5866 * v6782) * v170;
            let v6804 = v170 * v5865;
            let v6805 = v5866 * v170;
            let v6808 = v170 * (ddt(22348, v5870));
            let v6809 = (v5871 * v6782) * v170;
            let v6810 = v170 * v5870;
            let v6811 = v5871 * v170;
            let v6847: f64;
            let v6848: f64;
            let v6849: Lanes<5>;
            let v6850: Lanes<5>;
            if v6812 != 0.0 {
                v6847 = v643;
                v6848 = v643;
                v6849 = v6813;
                v6850 = v6814;
            } else {
                let v6815 = v236 / v5608;
                let v6819 = v236 / v5609;
                let v6824 = v6823 - v609;
                let v6829 = v6824 * v6815;
                let v6830 = ((Lanes([v6825, 0.0])) - (Lanes([0.0, v611]))) * v6815;
                let v6831 = (((v5611 * v6815) * v54) / v5608) * v6824;
                let v6834 = (Lanes([v6830[0], 0.0, 0.0, v6830[1], 0.0])) + (Lanes([0.0, v6831[0], v6831[1], v6831[2], v6831[3]]));
                let v6836 = v6835 - v600;
                let v6841 = v6836 * v6819;
                let v6842 = ((Lanes([v6837, 0.0])) - (Lanes([0.0, v604]))) * v6819;
                let v6843 = (((v5612 * v6819) * v54) / v5609) * v6836;
                let v6846 = (Lanes([v6842[0], 0.0, 0.0, v6842[1], 0.0])) + (Lanes([0.0, v6843[0], v6843[1], v6843[2], v6843[3]]));
                v6847 = v6829;
                v6848 = v6841;
                v6849 = v6834;
                v6850 = v6846;
            }
            let v6862: f64;
            let v6863: Lanes<6>;
            if v6678 != 0.0 {
                let v6851 = v663 - v599;
                let v6855 = v6851 * v6696;
                let v6856 = ((Lanes([v665, 0.0])) - (Lanes([0.0, v602]))) * v6696;
                let v6857 = v6697 * v6851;
                let v6860 = (Lanes([0.0, 0.0, 0.0, 0.0, v6856[0], v6856[1]])) + (Lanes([v6857[0], v6857[1], v6857[2], v6857[3], 0.0, v6857[4]]));
                v6862 = v6855;
                v6863 = v6860;
            } else {
                v6862 = v643;
                v6863 = v6861;
            }
            let v6875: f64;
            let v6876: Lanes<2>;
            if v6864 != 0.0 {
                v6875 = v643;
                v6876 = v6865;
            } else {
                let v6873 = (v6866 - v663) * v6872;
                let v6874 = ((Lanes([v6868, 0.0])) - (Lanes([0.0, v665]))) * v6872;
                v6875 = v6873;
                v6876 = v6874;
            }
            let v6877: f64;
            let v6878: f64;
            let v6879: f64;
            let v6880: f64;
            if v6045 != 0.0 {
                let v6885: f64;
                let v6886: f64;
                let v6887: f64;
                let v6888: f64;
                if v6574 != 0.0 {
                    v6885 = v6881;
                    v6886 = v6882;
                    v6887 = v643;
                    v6888 = v643;
                } else {
                    v6885 = v643;
                    v6886 = v643;
                    v6887 = v6883;
                    v6888 = v6884;
                }
                v6877 = v6885;
                v6878 = v6886;
                v6879 = v6887;
                v6880 = v6888;
            } else {
                v6877 = v643;
                v6878 = v643;
                v6879 = v643;
                v6880 = v643;
            }
            let v6891: f64;
            let v6892: f64;
            let v6893: f64;
            let v6894: f64;
            let v6895: f64;
            let v6896: Lanes<7>;
            let v6897: Lanes<5>;
            let v6898: f64;
            let v6899: f64;
            let v6900: f64;
            if v1 != 0.0 {
                let v6955: f64;
                let v6956: f64;
                let v6957: Lanes<7>;
                let v6958: Lanes<5>;
                if v6889 != 0.0 {
                    let v6901 = v170 * v658;
                    let v6902 = v6901 * v610;
                    let v6905 = (v614 * v6901) * v5635;
                    let v6908 = (Lanes([0.0, 0.0, v6905[0], v6905[1], 0.0])) + (v5636 * v6902);
                    let v6909 = v6823 - v609;
                    let v6914 = ((Lanes([v6825, 0.0])) - (Lanes([0.0, v611]))) * v6909;
                    let v6915 = v6914 + v6914;
                    let v6916 = (v6909 * v6909) / v5608;
                    let v6917 = v5611 * v6916;
                    let v6921 = ((Lanes([v6915[0], 0.0, 0.0, v6915[1], 0.0])) - (Lanes([0.0, v6917[0], v6917[1], v6917[2], v6917[3]]))) / v5608;
                    let v6925 = (Lanes([0.0, v6908[0], v6908[1], v6908[2], v6908[3], v6908[4]])) + (Lanes([v6921[0], v6921[1], v6921[2], v6921[3], 0.0, v6921[4]]));
                    let v6926 = v6835 - v600;
                    let v6931 = ((Lanes([v6837, 0.0])) - (Lanes([0.0, v604]))) * v6926;
                    let v6932 = v6931 + v6931;
                    let v6933 = (v6926 * v6926) / v5609;
                    let v6934 = v5612 * v6933;
                    let v6938 = ((Lanes([v6932[0], 0.0, 0.0, v6932[1], 0.0])) - (Lanes([0.0, v6934[0], v6934[1], v6934[2], v6934[3]]))) / v5609;
                    let v6943 = -(((v6902 * v5635) + v6916) + v6933);
                    let v6944 = ((Lanes([v6925[0], 0.0, v6925[1], v6925[2], v6925[3], v6925[4], v6925[5]])) + (Lanes([0.0, v6938[0], v6938[1], v6938[2], 0.0, v6938[3], v6938[4]]))) * v54;
                    v6955 = v6943;
                    v6956 = v643;
                    v6957 = v6944;
                    v6958 = v3004;
                } else {
                    let v6945 = v170 * v658;
                    let v6946 = v6945 * v610;
                    let v6949 = (v614 * v6945) * v5635;
                    let v6953 = -(v6946 * v5635);
                    let v6954 = ((Lanes([0.0, 0.0, v6949[0], v6949[1], 0.0])) + (v5636 * v6946)) * v54;
                    v6955 = v643;
                    v6956 = v6953;
                    v6957 = v6890;
                    v6958 = v6954;
                }
                let v6960 = v3 * v6959;
                let v6961 = v7 * v6959;
                let v6963 = v3 * v6962;
                let v6964 = v7 * v6962;
                let v6965 = ddt(22518, v6963);
                let v6966 = v6964 * v6782;
                v6891 = v6955;
                v6892 = v6956;
                v6893 = v6960;
                v6894 = v6965;
                v6895 = v6963;
                v6896 = v6957;
                v6897 = v6958;
                v6898 = v6961;
                v6899 = v6966;
                v6900 = v6964;
            } else {
                v6891 = v643;
                v6892 = v643;
                v6893 = v643;
                v6894 = v643;
                v6895 = v643;
                v6896 = v6890;
                v6897 = v3004;
                v6898 = v9;
                v6899 = v9;
                v6900 = v9;
            }
            let v6967 = v6571[2];
            let v6968 = v6571[4];
            let v6969 = v6571[3];
            let v6970 = v6571[0];
            let v6971 = v6795[2];
            let v6972 = v6795[4];
            let v6973 = v6795[3];
            let v6974 = v6795[0];
            let v6975 = v6787[2];
            let v6976 = v6787[4];
            let v6977 = v6787[3];
            let v6978 = v6787[0];
            let v6979 = v6767[0];
            let v6980 = v6767[1];
            let v6981 = v6767[2];
            let v6982 = v6767[3];
            let v6983 = v6767[4];
            let v6984 = v6768[0];
            let v6985 = v6768[1];
            let v6986 = v6768[2];
            let v6987 = v6768[3];
            let v6988 = v6768[4];
            let v6989 = v6769[0];
            let v6990 = v6769[1];
            let v6991 = v6769[2];
            let v6992 = v6769[3];
            let v6993 = v6769[4];
            let v6994 = v6770[0];
            let v6995 = v6770[1];
            let v6996 = v6770[2];
            let v6997 = v6770[3];
            let v6998 = v6770[4];
            let v6999 = v6771[0];
            let v7000 = v6771[1];
            let v7001 = v6771[2];
            let v7002 = v6771[3];
            let v7003 = v6771[4];
            let v7004 = v6772[0];
            let v7005 = v6772[1];
            let v7006 = v6772[2];
            let v7007 = v6772[3];
            let v7008 = v6772[4];
            let v7009 = v6773[0];
            let v7010 = v6773[1];
            let v7011 = v6773[2];
            let v7012 = v6773[3];
            let v7013 = v6773[4];
            let v7014 = v6774[0];
            let v7015 = v6774[1];
            let v7016 = v6774[2];
            let v7017 = v6774[3];
            let v7018 = v6774[4];
            let v7019 = v6775[0];
            let v7020 = v6775[1];
            let v7021 = v6775[2];
            let v7022 = v6775[3];
            let v7023 = v6775[4];
            let v7024 = v6776[0];
            let v7025 = v6776[1];
            let v7026 = v6776[2];
            let v7027 = v6776[3];
            let v7028 = v6776[4];
            let v7029 = v6778[0];
            let v7030 = v6778[1];
            let v7031 = v6778[2];
            let v7032 = v6778[3];
            let v7033 = v6778[4];
            let v7034 = v6780[0];
            let v7035 = v6780[1];
            let v7036 = v6780[2];
            let v7037 = v6780[3];
            let v7038 = v6780[4];
            let v7039 = v6785[0];
            let v7040 = v6785[1];
            let v7041 = v6785[2];
            let v7042 = v6785[3];
            let v7043 = v6785[4];
            let v7044 = v6789[0];
            let v7045 = v6789[1];
            let v7046 = v6789[2];
            let v7047 = v6789[3];
            let v7048 = v6789[4];
            let v7049 = v6793[0];
            let v7050 = v6793[1];
            let v7051 = v6793[2];
            let v7052 = v6793[3];
            let v7053 = v6793[4];
            let v7054 = v6797[0];
            let v7055 = v6797[1];
            let v7056 = v6797[2];
            let v7057 = v6797[3];
            let v7058 = v6799[0];
            let v7059 = v6799[1];
            let v7060 = v6799[2];
            let v7061 = v6799[3];
            let v7062 = v6803[0];
            let v7063 = v6803[1];
            let v7064 = v6809[0];
            let v7065 = v6809[1];
            let v7066 = v6849[0];
            let v7067 = v6849[1];
            let v7068 = v6849[2];
            let v7069 = v6849[3];
            let v7070 = v6849[4];
            let v7071 = v6850[0];
            let v7072 = v6850[1];
            let v7073 = v6850[2];
            let v7074 = v6850[3];
            let v7075 = v6850[4];
            let v7076 = v6863[0];
            let v7077 = v6863[1];
            let v7078 = v6863[2];
            let v7079 = v6863[3];
            let v7080 = v6863[4];
            let v7081 = v6863[5];
            let v7082 = v6876[0];
            let v7083 = v6876[1];
            let v7084 = v6896[0];
            let v7085 = v6896[1];
            let v7086 = v6896[2];
            let v7087 = v6896[3];
            let v7088 = v6896[4];
            let v7089 = v6896[5];
            let v7090 = v6896[6];
            let v7091 = v6897[0];
            let v7092 = v6897[1];
            let v7093 = v6897[2];
            let v7094 = v6897[3];
            let v7095 = v6897[4];
            let v7096 = v6898;
            let v7097 = v6899;
            let v7098 = v6787[1];
            let v7099 = v6571[1];
            let v7100 = v6795[1];
            let v7101 = v6675[0];
            let v7102 = v6675[1];
            let v7103 = v6675[2];
            let v7104 = v6675[3];
            let v7105 = v6677[0];
            let v7106 = v6677[1];
            let v7107 = v6677[2];
            let v7108 = v6677[3];
            let v7109 = v6805[0];
            let v7110 = v6805[1];
            let v7111 = v6811[0];
            let v7112 = v6811[1];
            let v7113 = v6900;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * (v6757),
            [3, 4, 5, 6, 8],
            [v6979, v6980, v6981, v6982, v6983],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * (v6758),
            [3, 4, 5, 6, 8],
            [v6984, v6985, v6986, v6987, v6988],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(5),
            multiplicity * (v6759),
            [3, 4, 5, 6, 8],
            [v6989, v6990, v6991, v6992, v6993],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (v6760),
            [3, 4, 5, 6, 8],
            [v6994, v6995, v6996, v6997, v6998],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(5),
            multiplicity * (v6761),
            [3, 4, 5, 6, 8],
            [v6999, v7000, v7001, v7002, v7003],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(5),
            multiplicity * (v6762),
            [3, 4, 5, 6, 8],
            [v7004, v7005, v7006, v7007, v7008],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(5),
            multiplicity * (v6763),
            [3, 4, 5, 6, 8],
            [v7009, v7010, v7011, v7012, v7013],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * (v6764),
            [3, 4, 5, 6, 8],
            [v7014, v7015, v7016, v7017, v7018],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(5),
            multiplicity * (v6765),
            [3, 4, 5, 6, 8],
            [v7019, v7020, v7021, v7022, v7023],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (v6766),
            [3, 4, 5, 6, 8],
            [v7024, v7025, v7026, v7027, v7028],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (v6777),
            [3, 4, 5, 6, 8],
            [v7029, v7030, v7031, v7032, v7033],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(5),
            multiplicity * (v6779),
            [3, 4, 5, 6, 8],
            [v7034, v7035, v7036, v7037, v7038],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * (v6784),
            [3, 4, 5, 6, 8],
            [v7039, v7040, v7041, v7042, v7043],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (v6788),
            [3, 4, 5, 6, 8],
            [v7044, v7045, v7046, v7047, v7048],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(3),
            Some(6),
            multiplicity * (v6792),
            [3, 4, 5, 6, 8],
            [v7049, v7050, v7051, v7052, v7053],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(6),
            multiplicity * (v6796),
            [3, 4, 6, 7],
            [v7054, v7055, v7056, v7057],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(5),
            multiplicity * (v6798),
            [3, 4, 5, 7],
            [v7058, v7059, v7060, v7061],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(6),
            Some(3),
            multiplicity * (v6802),
            [3, 6],
            [v7062, v7063],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(5),
            Some(3),
            multiplicity * (v6808),
            [3, 5],
            [v7064, v7065],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(0), Some(5), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            staged[349],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(2), Some(6), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            staged[350],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(0),
            Some(5),
            multiplicity * (v6847),
            [0, 3, 4, 5, 8],
            [v7066, v7067, v7068, v7069, v7070],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(6),
            multiplicity * (v6848),
            [2, 3, 4, 6, 8],
            [v7071, v7072, v7073, v7074, v7075],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(5),
            multiplicity * (staged[351]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(2),
            Some(6),
            multiplicity * (staged[352]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(8),
            multiplicity * (v6862),
            [3, 4, 5, 6, 7, 8],
            [v7076, v7077, v7078, v7079, v7080, v7081],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(7), Some(8), 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            staged[353],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(1), Some(7), 3, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            3,
            staged[354],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(7),
            multiplicity * (v6875),
            [1, 7],
            [v7082, v7083],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(7),
            multiplicity * (staged[355]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(6),
            multiplicity * (v7114),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(6),
            multiplicity * (v7115),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(6),
            multiplicity * (v6877),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(5),
            multiplicity * (v6878),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(5),
            multiplicity * (v6879),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(6),
            multiplicity * (v6880),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(6),
            multiplicity * (staged[356]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(5),
            multiplicity * (staged[357]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(4),
            None,
            multiplicity * (v6891),
            [0, 2, 3, 4, 5, 6, 8],
            [v7084, v7085, v7086, v7087, v7088, v7089, v7090],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            None,
            multiplicity * (v6892),
            [3, 4, 5, 6, 8],
            [v7091, v7092, v7093, v7094, v7095],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (v6893),
            [4],
            [v7096],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (v6894),
            [4],
            [v7097],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(4), None, 4, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            4,
            staged[358],
            [],
            [],
            [],
            [],
        );
        self.canonical_reactive[0] = v6757;
        self.canonical_reactive[1] = v6758;
        self.canonical_reactive[2] = v6759;
        self.canonical_reactive[3] = v6760;
        self.canonical_reactive[4] = v6761;
        self.canonical_reactive[5] = v6762;
        self.canonical_reactive[6] = v6763;
        self.canonical_reactive[7] = v6764;
        self.canonical_reactive[8] = v6765;
        self.canonical_reactive[9] = v6766;
        self.canonical_reactive[10] = v6777;
        self.canonical_reactive[11] = v6779;
        self.canonical_reactive[12] = v6786;
        self.canonical_reactive[13] = v6978;
        self.canonical_reactive[14] = v7098;
        self.canonical_reactive[15] = v6975;
        self.canonical_reactive[16] = v6977;
        self.canonical_reactive[17] = v6976;
        self.canonical_reactive[18] = v6570;
        self.canonical_reactive[19] = v6970;
        self.canonical_reactive[20] = v7099;
        self.canonical_reactive[21] = v6967;
        self.canonical_reactive[22] = v6969;
        self.canonical_reactive[23] = v6968;
        self.canonical_reactive[24] = v6794;
        self.canonical_reactive[25] = v6974;
        self.canonical_reactive[26] = v7100;
        self.canonical_reactive[27] = v6971;
        self.canonical_reactive[28] = v6973;
        self.canonical_reactive[29] = v6972;
        self.canonical_reactive[30] = v6674;
        self.canonical_reactive[31] = v7101;
        self.canonical_reactive[32] = v7102;
        self.canonical_reactive[33] = v7103;
        self.canonical_reactive[34] = v7104;
        self.canonical_reactive[35] = v6676;
        self.canonical_reactive[36] = v7105;
        self.canonical_reactive[37] = v7106;
        self.canonical_reactive[38] = v7107;
        self.canonical_reactive[39] = v7108;
        self.canonical_reactive[40] = v6804;
        self.canonical_reactive[41] = v7109;
        self.canonical_reactive[42] = v7110;
        self.canonical_reactive[43] = v6810;
        self.canonical_reactive[44] = v7111;
        self.canonical_reactive[45] = v7112;
        self.canonical_reactive[46] = staged[349];
        self.canonical_reactive[47] = staged[350];
        self.canonical_reactive[48] = v6847;
        self.canonical_reactive[49] = v6848;
        self.canonical_reactive[50] = staged[351];
        self.canonical_reactive[51] = staged[352];
        self.canonical_reactive[52] = v6862;
        self.canonical_reactive[53] = staged[353];
        self.canonical_reactive[54] = staged[354];
        self.canonical_reactive[55] = v6875;
        self.canonical_reactive[56] = staged[355];
        self.canonical_reactive[57] = v7114;
        self.canonical_reactive[58] = v7115;
        self.canonical_reactive[59] = v6877;
        self.canonical_reactive[60] = v6878;
        self.canonical_reactive[61] = v6879;
        self.canonical_reactive[62] = v6880;
        self.canonical_reactive[63] = staged[356];
        self.canonical_reactive[64] = staged[357];
        self.canonical_reactive[65] = v6891;
        self.canonical_reactive[66] = v6892;
        self.canonical_reactive[67] = v6893;
        self.canonical_reactive[68] = v6895;
        self.canonical_reactive[69] = v7113;
        self.canonical_reactive[70] = staged[358];
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(6),
            &[3, 4, 5, 6, 8],
            &[cached[13], cached[14], cached[15], cached[16], cached[17]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(6),
            &[3, 4, 5, 6, 8],
            &[cached[19], cached[20], cached[21], cached[22], cached[23]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            Some(6),
            &[3, 4, 5, 6, 8],
            &[cached[25], cached[26], cached[27], cached[28], cached[29]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(6),
            &[3, 4, 6, 7],
            &[cached[31], cached[32], cached[33], cached[34]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(5),
            &[3, 4, 5, 7],
            &[cached[36], cached[37], cached[38], cached[39]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(3),
            &[3, 6],
            &[cached[41], cached[42]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(3),
            &[3, 5],
            &[cached[44], cached[45]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            None,
            &[4],
            &[cached[69]],
            &[],
            &[],
            multiplicity,
        );
    }

}

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
        let produced: [f64; 62] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let v0 = parameters[7];
                let v1 = 1e3f64;
                let v3 = parameters[20];
                let v4 = 3.9e0f64;
                let v6 = 3.453e-11f64;
                let v8 = parameters[19];
                let v10 = 3.348580862e-29f64;
                let v11 = parameters[29];
                let v15 = 3.348580862e-29f64;
                let v16 = parameters[54];
                let v20 = parameters[30];
                let v21 = 0e0f64;
                let v23 = 2.3807972e0f64;
                let v25 = 6.666666666666666e-1f64;
                let v28 = parameters[17];
                let v32 = 1.2514650134837189e0f64;
                let v35 = 3.333333333333333e-1f64;
                let v36 = parameters[48];
                let v38 = 5e-1f64;
                let v41 = 1e-9f64;
                let v43 = parameters[11];
                let v44 = -2.73e2f64;
                let v46 = -2.73e2f64;
                let v48 = parameters[8];
                let v50 = parameters[9];
                let v52 = 2.7315e2f64;
                let v54 = parameters[1];
                let v55 = parameters[12];
                let v57 = parameters[13];
                let v59 = parameters[0];
                let v60 = parameters[14];
                let v62 = parameters[15];
                let v64 = parameters[31];
                let v66 = parameters[32];
                let v70 = parameters[35];
                let v72 = parameters[34];
                let v75 = 2e0f64;
                let v77 = parameters[16];
                let v78 = parameters[2];
                let v79 = 1e0f64;
                let v81 = 9e0f64;
                let v83 = 3e0f64;
                let v87 = parameters[33];
                let v90 = 1.2e1f64;
                let v92 = parameters[66];
                let v94 = parameters[49];
                let v95 = parameters[55];
                let v98 = 1e12f64;
                let v100 = parameters[56];
                let v102 = parameters[53];
                let v106 = parameters[60];
                let v110 = parameters[61];
                let v115 = parameters[50];
                let v117 = parameters[51];
                let v119 = 2.918995620956536e-49f64;
                let v122 = 1.3333333333333333e0f64;
                let v124 = 1.05457168e-34f64;
                let v127 = 2.918995620956536e-49f64;
                let v130 = 1.3333333333333333e0f64;
                let v134 = parameters[59];
                let v136 = 1e-1f64;
                let v145 = -4.95e-1f64;
                let v146 = parameters[58];
                let v150 = parameters[64];
                let v152 = -4.95e-1f64;
                let v153 = parameters[63];
                let v157 = 7.5e-1f64;
                let v159 = 1e27f64;
                let v162 = parameters[18];
                let v164 = parameters[21];
                let v168 = 3.7e-1f64;
                let v174 = -1e0f64;
                let v177 = 0e0f64;
                let v178 = 0e0f64;
                let v179 = 0e0f64;
                let v180 = 0e0f64;
                let v185 = 0e0f64;
                let v186 = 0e0f64;
                let v189 = 0e0f64;
                let v190 = 0e0f64;
                let v191 = 0e0f64;
                let v192 = 0e0f64;
                let v193 = 0e0f64;
                let v2 = if v0 != v1 { 1.0 } else { 0.0 };
                let v9 = (v6 * (v3 / v4)) / v8;
                let v14 = ((v10 * v11).sqrt()) / v9;
                let v19 = ((v15 * v16).sqrt()) / v9;
                let v22 = if v20 > v21 { 1.0 } else { 0.0 };
                let v30: f64;
                if v22 != 0.0 {
                    let v27 = (v23 * v20) * (v9.powf(v25));
                    let v29 = if v28 < v21 { 1.0 } else { 0.0 };
                    let v34: f64;
                    if v29 != 0.0 {
                        let v33 = v32 * v27;
                        v34 = v33;
                    } else {
                        v34 = v27;
                    }
                    v30 = v34;
                } else {
                    v30 = v21;
                }
                let v31 = if v28 < v21 { 1.0 } else { 0.0 };
                let v40: f64;
                if v31 != 0.0 {
                    let v37 = v35 * v36;
                    v40 = v37;
                } else {
                    let v39 = v38 * v36;
                    v40 = v39;
                }
                let v42 = v8 / v41;
                let v45 = if v43 > v44 { 1.0 } else { 0.0 };
                let v47: f64;
                if v45 != 0.0 {
                    v47 = v43;
                } else {
                    v47 = v46;
                }
                let v49 = if v47 < v48 { 1.0 } else { 0.0 };
                let v51 = if v47 > v50 { 1.0 } else { 0.0 };
                let v53 = v52 + v47;
                let v56 = if v54 < v55 { 1.0 } else { 0.0 };
                let v58 = if v54 > v57 { 1.0 } else { 0.0 };
                let v61 = if v59 < v60 { 1.0 } else { 0.0 };
                let v63 = if v59 > v62 { 1.0 } else { 0.0 };
                let v65 = v54 + v64;
                let v67 = v59 + v66;
                let v68 = if v65 <= v21 { 1.0 } else { 0.0 };
                let v69 = if v67 <= v21 { 1.0 } else { 0.0 };
                let v76 = v75 * ((v70 * v59) + (v72 * v54));
                if v77 != 0.0 {
                    let v85 = (v83 + ((v78 - v79) * v81)) * v54;
                    let v86 = v59 * v54;
                    let v88 = v59 + v87;
                    let v89 = v75 * v88;
                    let v91 = v90 * v88;
                } else {
                }
                let v93 = if v92 == v21 { 1.0 } else { 0.0 };
                let v137: f64;
                let v138: f64;
                let v139: f64;
                let v140: f64;
                let v141: f64;
                let v142: f64;
                let v143: f64;
                let v144: f64;
                if v94 != 0.0 {
                    let v99 = ((v95 * v67) * v65) * v98;
                    let v105 = (((v75 * v100) * v102) * v67) * v98;
                    let v109 = ((v106 * v67) * v65) * v98;
                    let v114 = (((v75 * v110) * v102) * v67) * v98;
                    let v116 = v79 / v115;
                    let v118 = v79 / v117;
                    let v126 = ((v122 * ((v119 * v115).sqrt())) / v124) * v8;
                    let v133 = ((v130 * ((v127 * v117).sqrt())) / v124) * v8;
                    let v135 = if v134 < v21 { 1.0 } else { 0.0 };
                    let v149: f64;
                    if v135 != 0.0 {
                        let v148 = (v145 * v146) / v134;
                        v149 = v148;
                    } else {
                        v149 = v21;
                    }
                    let v151 = if v150 < v21 { 1.0 } else { 0.0 };
                    let v156: f64;
                    if v151 != 0.0 {
                        let v155 = (v152 * v153) / v150;
                        v156 = v155;
                    } else {
                        v156 = v21;
                    }
                    v137 = v118;
                    v138 = v156;
                    v139 = v133;
                    v140 = v116;
                    v141 = v149;
                    v142 = v126;
                    v143 = v133;
                    v144 = v126;
                } else {
                    v137 = v136;
                    v138 = v21;
                    v139 = v21;
                    v140 = v136;
                    v141 = v21;
                    v142 = v21;
                    v143 = v21;
                    v144 = v21;
                }
                if v22 != 0.0 {
                    let v158 = v157 * v30;
                } else {
                }
                let v160 = if v11 < v159 { 1.0 } else { 0.0 };
                if v160 != 0.0 {
                    let v163 = (-v28) * v162;
                } else {
                }
                let v165 = if v164 < v79 { 1.0 } else { 0.0 };
                if v160 != 0.0 {
                    let v167 = (-v28) * v162;
                } else {
                }
                let v170 = v79 + (v168 * v42);
                let v171 = if v30 > v21 { 1.0 } else { 0.0 };
                let v172 = if v92 == v75 { 1.0 } else { 0.0 };
                let v175 = if (v162 * v28) == v174 { 1.0 } else { 0.0 };
                let v176 = if v94 != v21 { 1.0 } else { 0.0 };
                let v181: f64;
                let v182: f64;
                let v183: f64;
                let v184: f64;
                if v77 != 0.0 {
                    v181 = v21;
                    v182 = v21;
                    v183 = v21;
                    v184 = v21;
                } else {
                    v181 = v177;
                    v182 = v178;
                    v183 = v179;
                    v184 = v180;
                }
                let v187: f64;
                let v188: f64;
                if v94 != 0.0 {
                    v187 = v185;
                    v188 = v186;
                } else {
                    v187 = v21;
                    v188 = v21;
                }
                let v194: f64;
                let v195: f64;
                let v196: f64;
                let v197: f64;
                let v198: f64;
                if v77 != 0.0 {
                    v194 = v189;
                    v195 = v190;
                    v196 = v191;
                    v197 = v192;
                    v198 = v193;
                } else {
                    v194 = v21;
                    v195 = v21;
                    v196 = v21;
                    v197 = v21;
                    v198 = v21;
                }
            [v2, v9, v14, v19, v22, v29, v31, v45, v49, v51, v53, v56, v58, v61, v63, v65, v67, v68, v69, v76, v85, v86, v89, v91, v93, v99, v105, v109, v114, v135, v151, v30, v158, v160, v163, v165, v167, v170, v40, v171, v172, v175, v176, v137, v138, v139, v140, v141, v142, v143, v144, v181, v182, v183, v184, v187, v188, v194, v195, v196, v197, v198]
        };
        self.canonical_staged[72] = produced[0];
        self.canonical_staged[14] = produced[1];
        self.canonical_staged[1] = produced[2];
        self.canonical_staged[2] = produced[3];
        self.canonical_staged[73] = produced[4];
        self.canonical_staged[74] = produced[5];
        self.canonical_staged[75] = produced[6];
        self.canonical_staged[76] = produced[7];
        self.canonical_staged[77] = produced[8];
        self.canonical_staged[78] = produced[9];
        self.canonical_staged[0] = produced[10];
        self.canonical_staged[81] = produced[11];
        self.canonical_staged[82] = produced[12];
        self.canonical_staged[83] = produced[13];
        self.canonical_staged[84] = produced[14];
        self.canonical_staged[65] = produced[15];
        self.canonical_staged[66] = produced[16];
        self.canonical_staged[85] = produced[17];
        self.canonical_staged[86] = produced[18];
        self.canonical_staged[67] = produced[19];
        self.canonical_staged[3] = produced[20];
        self.canonical_staged[4] = produced[21];
        self.canonical_staged[5] = produced[22];
        self.canonical_staged[6] = produced[23];
        self.canonical_staged[89] = produced[24];
        self.canonical_staged[7] = produced[25];
        self.canonical_staged[8] = produced[26];
        self.canonical_staged[9] = produced[27];
        self.canonical_staged[10] = produced[28];
        self.canonical_staged[99] = produced[29];
        self.canonical_staged[100] = produced[30];
        self.canonical_staged[36] = produced[31];
        self.canonical_staged[15] = produced[32];
        self.canonical_staged[101] = produced[33];
        self.canonical_staged[20] = produced[34];
        self.canonical_staged[29] = produced[35];
        self.canonical_staged[30] = produced[36];
        self.canonical_staged[31] = produced[37];
        self.canonical_staged[34] = produced[38];
        self.canonical_staged[102] = produced[39];
        self.canonical_staged[103] = produced[40];
        self.canonical_staged[104] = produced[41];
        self.canonical_staged[39] = produced[42];
        self.canonical_staged[46] = produced[43];
        self.canonical_staged[47] = produced[44];
        self.canonical_staged[50] = produced[45];
        self.canonical_staged[53] = produced[46];
        self.canonical_staged[54] = produced[47];
        self.canonical_staged[57] = produced[48];
        self.canonical_staged[61] = produced[49];
        self.canonical_staged[63] = produced[50];
        self.canonical_staged[116] = produced[51];
        self.canonical_staged[117] = produced[52];
        self.canonical_staged[118] = produced[53];
        self.canonical_staged[119] = produced[54];
        self.canonical_staged[120] = produced[55];
        self.canonical_staged[121] = produced[56];
        self.canonical_staged[122] = produced[57];
        self.canonical_staged[123] = produced[58];
        self.canonical_staged[124] = produced[59];
        self.canonical_staged[125] = produced[60];
        self.canonical_staged[126] = produced[61];
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
        let produced: [f64; 65] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let temperature = ctx.temperature();
            let staged = &*self.canonical_staged;
                let v0 = temperature;
                let v1 = parameters[3];
                let v3 = 2.7315e2f64;
                let v5 = parameters[8];
                let v7 = parameters[9];
                let v11 = staged[0];
                let v15 = 1.3806505e-23f64;
                let v17 = 1.6021918e-19f64;
                let v19 = 1e2f64;
                let v22 = 1e0f64;
                let v24 = parameters[42];
                let v26 = parameters[23];
                let v28 = parameters[43];
                let v30 = parameters[36];
                let v32 = parameters[44];
                let v34 = parameters[37];
                let v36 = parameters[45];
                let v38 = parameters[38];
                let v40 = parameters[46];
                let v42 = parameters[39];
                let v44 = parameters[47];
                let v46 = parameters[40];
                let v48 = 3.05e-7f64;
                let v50 = 9.025e-5f64;
                let v53 = 1.179e0f64;
                let v55 = 4.5e-4f64;
                let v57 = 1.045e0f64;
                let v59 = 1.4e-3f64;
                let v61 = 5.23e-1f64;
                let v63 = 1.48e-6f64;
                let v68 = 9e4f64;
                let v70 = 1e-3f64;
                let v74 = 2.5e25f64;
                let v78 = 2e0f64;
                let v80 = parameters[24];
                let v85 = parameters[29];
                let v90 = 6e0f64;
                let v94 = staged[1];
                let v98 = 7.071067811865475e-1f64;
                let v102 = 1e-5f64;
                let v105 = staged[2];
                let v111 = 4.6051701859880916e2f64;
                let v116 = 5e-1f64;
                let v118 = 3.333333333333333e-1f64;
                let v125 = 1e-200f64;
                let v128 = parameters[16];
                let v129 = parameters[0];
                let v131 = staged[3];
                let v133 = staged[4];
                let v135 = staged[5];
                let v137 = parameters[1];
                let v139 = staged[6];
                let v142 = 0e0f64;
                let v148 = 1e3f64;
                let v165 = 2e1f64;
                let v172 = 1.2e1f64;
                let v177 = parameters[49];
                let v178 = parameters[52];
                let v180 = staged[7];
                let v182 = staged[8];
                let v184 = staged[9];
                let v186 = staged[10];
                let v196 = parameters[17];
                let v203 = parameters[57];
                let v205 = parameters[62];
                let v207 = 7.324648775608221e-1f64;
                let v209 = 1.25e0f64;
                let v212 = staged[104];
                let v213 = parameters[18];
                let v219 = staged[39];
                let v226 = parameters[64];
                let v228 = 1.0f64;
                let v231 = parameters[59];
                let v236 = 0.0f64;
                let v4 = (v0 + v1) - v3;
                let v6 = if v4 < v5 { 1.0 } else { 0.0 };
                let v8 = if v4 > v7 { 1.0 } else { 0.0 };
                let v9 = v4 + v3;
                let v10 = v9 * v9;
                let v13 = v9 / v11;
                let v14 = v11 / v9;
                let v18 = (v9 * v15) / v17;
                let v21 = (v19 * v18) * v18;
                let v23 = v22 / v18;
                let v27 = v26 + ((v9 - v11) * v24);
                let v31 = v30 * (v14.powf(v28));
                let v35 = v34 * (v14.powf(v32));
                let v39 = v38 * (v14.powf(v36));
                let v43 = v42 * (v14.powf(v40));
                let v47 = v46 * (v13.powf(v44));
                let v54 = v53 - (v9 * (v50 + (v9 * v48)));
                let v72 = (if ((((v57 + (v55 * v9)) * ((v61 + (v59 * v9)) - (v63 * v10))) * v10) / v68) >= v70 { ((((v57 + (v55 * v9)) * ((v61 + (v59 * v9)) - (v63 * v10))) * v10) / v68) } else { v70 }).sqrt();
                let v77 = v22 / ((v74 * v72) * (v72.sqrt()));
                let v79 = v78 * v18;
                let v84 = v54 + (v79 * ((v80 * v77).ln()));
                let v92 = v54 + (v90 * v18);
                let v93 = v23.sqrt();
                let v95 = v94 * v93;
                let v96 = v95 * v95;
                let v97 = v22 / v96;
                let v100 = v22 + (v95 * v98);
                let v101 = v22 / v100;
                let v103 = v102 * v100;
                let v104 = (v54 + (v79 * ((v85 * v77).ln()))) * v23;
                let v106 = v105 * v93;
                let v107 = v106 * v106;
                let v109 = v22 + (v106 * v98);
                let v110 = v102 * v109;
                let v112 = if v104 < v111 { 1.0 } else { 0.0 };
                let v127: f64;
                if v112 != 0.0 {
                    let v114 = (-v104).exp();
                    v127 = v114;
                } else {
                    let v115 = v104 - v111;
                    let v126 = v125 / (v22 + (v115 * (v22 + ((v116 * v115) * (v22 + (v115 * v118))))));
                    v127 = v126;
                }
                let v143: f64;
                let v144: f64;
                let v145: f64;
                let v146: f64;
                let v147: f64;
                if v128 != 0.0 {
                    let v132 = (v31 * v129) / v131;
                    let v134 = v35 / v133;
                    let v136 = v39 / v135;
                    let v140 = (v43 * v137) / v139;
                    let v141 = if v132 > v70 { 1.0 } else { 0.0 };
                    let v150: f64;
                    if v141 != 0.0 {
                        let v149 = if v132 < v148 { 1.0 } else { 0.0 };
                        let v152: f64;
                        if v149 != 0.0 {
                            v152 = v132;
                        } else {
                            v152 = v148;
                        }
                        v150 = v152;
                    } else {
                        v150 = v70;
                    }
                    let v151 = if v134 > v70 { 1.0 } else { 0.0 };
                    let v154: f64;
                    if v151 != 0.0 {
                        let v153 = if v134 < v19 { 1.0 } else { 0.0 };
                        let v156: f64;
                        if v153 != 0.0 {
                            v156 = v134;
                        } else {
                            v156 = v19;
                        }
                        v154 = v156;
                    } else {
                        v154 = v70;
                    }
                    let v155 = if v136 > v70 { 1.0 } else { 0.0 };
                    let v158: f64;
                    if v155 != 0.0 {
                        let v157 = if v136 < v148 { 1.0 } else { 0.0 };
                        let v160: f64;
                        if v157 != 0.0 {
                            v160 = v136;
                        } else {
                            v160 = v148;
                        }
                        v158 = v160;
                    } else {
                        v158 = v70;
                    }
                    let v159 = if v140 > v70 { 1.0 } else { 0.0 };
                    let v162: f64;
                    if v159 != 0.0 {
                        let v161 = if v140 < v148 { 1.0 } else { 0.0 };
                        let v164: f64;
                        if v161 != 0.0 {
                            v164 = v140;
                        } else {
                            v164 = v148;
                        }
                        v162 = v164;
                    } else {
                        v162 = v70;
                    }
                    let v163 = if v47 > v70 { 1.0 } else { 0.0 };
                    let v167: f64;
                    if v163 != 0.0 {
                        let v166 = if v47 < v165 { 1.0 } else { 0.0 };
                        let v176: f64;
                        if v166 != 0.0 {
                            v176 = v47;
                        } else {
                            v176 = v165;
                        }
                        v167 = v176;
                    } else {
                        v167 = v70;
                    }
                    let v168 = v22 / v150;
                    let v169 = v22 / v154;
                    let v170 = v22 / v158;
                    let v171 = v22 / v162;
                    let v175 = ((v172 * v167) * v129) / v137;
                    v143 = v168;
                    v144 = v169;
                    v145 = v170;
                    v146 = v171;
                    v147 = v175;
                } else {
                    v143 = v142;
                    v144 = v142;
                    v145 = v142;
                    v146 = v142;
                    v147 = v142;
                }
                let v188: f64;
                let v189: f64;
                let v190: f64;
                let v191: f64;
                let v192: f64;
                let v193: f64;
                let v194: f64;
                let v195: f64;
                if v177 != 0.0 {
                    let v179 = v13.powf(v178);
                    let v181 = v180 * v179;
                    let v183 = v182 * v179;
                    let v185 = v184 * v179;
                    let v187 = v186 * v179;
                    let v199 = v116 * ((v196 * v84) + v54);
                    let v202 = v116 * ((v196 * v92) + v54);
                    let v204 = v203 * v18;
                    let v206 = v205 * v18;
                    v188 = v183;
                    v189 = v187;
                    v190 = v206;
                    v191 = v202;
                    v192 = v199;
                    v193 = v204;
                    v194 = v181;
                    v195 = v185;
                } else {
                    v188 = v142;
                    v189 = v142;
                    v190 = v142;
                    v191 = v142;
                    v192 = v142;
                    v193 = v142;
                    v194 = v142;
                    v195 = v142;
                }
                let v210 = v209 + (v106 * v207);
                let v211 = v14.sqrt();
                let v215: f64;
                if v212 != 0.0 {
                    let v214 = v213 * v54;
                    v215 = v214;
                } else {
                    v215 = v142;
                }
                let v216 = if v188 > v142 { 1.0 } else { 0.0 };
                let v217 = if v189 > v142 { 1.0 } else { 0.0 };
                let v218 = if v216 != 0.0 || v217 != 0.0 { 1.0 } else { 0.0 };
                let v220 = if v219 != 0.0 && v218 != 0.0 { 1.0 } else { 0.0 };
                if v177 != 0.0 {
                    if v218 != 0.0 {
                        let v222 = if (if v213 == v22 { 1.0 } else { 0.0 }) != 0.0 && v217 != 0.0 { 1.0 } else { 0.0 };
                        if v222 != 0.0 {
                            let v227 = if v226 < v142 { 1.0 } else { 0.0 };
                            if v228 != 0.0 {
                                let v229 = v54 - v191;
                            } else {
                                let v230 = v54 - v192;
                            }
                        } else {
                        }
                        if v216 != 0.0 {
                            let v232 = if v231 < v142 { 1.0 } else { 0.0 };
                        } else {
                        }
                    } else {
                    }
                    let v223 = if v194 > v142 { 1.0 } else { 0.0 };
                    let v224 = if v195 > v142 { 1.0 } else { 0.0 };
                    let v225 = if v223 != 0.0 || v224 != 0.0 { 1.0 } else { 0.0 };
                    if v225 != 0.0 {
                        let v234 = if (if v213 == v22 { 1.0 } else { 0.0 }) != 0.0 && v224 != 0.0 { 1.0 } else { 0.0 };
                        if v234 != 0.0 {
                            let v235 = if v226 < v142 { 1.0 } else { 0.0 };
                            if v236 != 0.0 {
                                let v237 = v54 - v191;
                            } else {
                                let v238 = v54 - v192;
                            }
                        } else {
                        }
                        if v223 != 0.0 {
                            let v239 = if v231 < v142 { 1.0 } else { 0.0 };
                        } else {
                        }
                    } else {
                    }
                } else {
                }
            [v6, v8, v14, v18, v21, v23, v27, v54, v77, v79, v93, v95, v96, v97, v100, v101, v103, v104, v106, v107, v109, v110, v112, v141, v149, v151, v153, v155, v157, v159, v161, v163, v166, v143, v144, v145, v146, v147, v210, v127, v211, v215, v188, v216, v189, v218, v220, v222, v190, v227, v191, v229, v192, v230, v193, v232, v194, v223, v195, v225, v234, v235, v237, v238, v239]
        };
        self.canonical_staged[79] = produced[0];
        self.canonical_staged[80] = produced[1];
        self.canonical_staged[32] = produced[2];
        self.canonical_staged[19] = produced[3];
        self.canonical_staged[35] = produced[4];
        self.canonical_staged[17] = produced[5];
        self.canonical_staged[18] = produced[6];
        self.canonical_staged[13] = produced[7];
        self.canonical_staged[11] = produced[8];
        self.canonical_staged[12] = produced[9];
        self.canonical_staged[16] = produced[10];
        self.canonical_staged[24] = produced[11];
        self.canonical_staged[25] = produced[12];
        self.canonical_staged[26] = produced[13];
        self.canonical_staged[27] = produced[14];
        self.canonical_staged[22] = produced[15];
        self.canonical_staged[21] = produced[16];
        self.canonical_staged[28] = produced[17];
        self.canonical_staged[44] = produced[18];
        self.canonical_staged[43] = produced[19];
        self.canonical_staged[41] = produced[20];
        self.canonical_staged[40] = produced[21];
        self.canonical_staged[87] = produced[22];
        self.canonical_staged[88] = produced[23];
        self.canonical_staged[90] = produced[24];
        self.canonical_staged[91] = produced[25];
        self.canonical_staged[92] = produced[26];
        self.canonical_staged[93] = produced[27];
        self.canonical_staged[94] = produced[28];
        self.canonical_staged[95] = produced[29];
        self.canonical_staged[96] = produced[30];
        self.canonical_staged[97] = produced[31];
        self.canonical_staged[98] = produced[32];
        self.canonical_staged[68] = produced[33];
        self.canonical_staged[69] = produced[34];
        self.canonical_staged[71] = produced[35];
        self.canonical_staged[70] = produced[36];
        self.canonical_staged[37] = produced[37];
        self.canonical_staged[42] = produced[38];
        self.canonical_staged[23] = produced[39];
        self.canonical_staged[33] = produced[40];
        self.canonical_staged[38] = produced[41];
        self.canonical_staged[58] = produced[42];
        self.canonical_staged[109] = produced[43];
        self.canonical_staged[51] = produced[44];
        self.canonical_staged[106] = produced[45];
        self.canonical_staged[105] = produced[46];
        self.canonical_staged[107] = produced[47];
        self.canonical_staged[45] = produced[48];
        self.canonical_staged[110] = produced[49];
        self.canonical_staged[55] = produced[50];
        self.canonical_staged[48] = produced[51];
        self.canonical_staged[56] = produced[52];
        self.canonical_staged[49] = produced[53];
        self.canonical_staged[52] = produced[54];
        self.canonical_staged[111] = produced[55];
        self.canonical_staged[64] = produced[56];
        self.canonical_staged[113] = produced[57];
        self.canonical_staged[62] = produced[58];
        self.canonical_staged[108] = produced[59];
        self.canonical_staged[112] = produced[60];
        self.canonical_staged[114] = produced[61];
        self.canonical_staged[59] = produced[62];
        self.canonical_staged[60] = produced[63];
        self.canonical_staged[115] = produced[64];
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
        let staged = &*self.canonical_staged;
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 16846 => 0usize, 16850 => 1usize, 16852 => 2usize, _ => usize::MAX };
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
            let v0 = staged[73];
            let v1 = parameters[16];
            let v2 = parameters[49];
            let v3 = node_potentials[4];
            let v4 = node_potentials[5];
            let v6 = Lanes([1e0f64; 1]);
            let v8 = Lanes([1e0f64; 1]);
            let v11 = parameters[27];
            let v13 = parameters[17];
            let v16 = 1e-16f64;
            let v21 = parameters[28];
            let v24 = 2e0f64;
            let v26 = 1e0f64;
            let v31 = 5e-1f64;
            let v34 = 0e0f64;
            let v36 = -1e0f64;
            let v41 = parameters[26];
            let v44 = 1e0f64;
            let v46 = parameters[25];
            let v65 = 1e-32f64;
            let v76 = 1e-6f64;
            let v92 = parameters[24];
            let v95 = 1e23f64;
            let v98 = staged[11];
            let v104 = staged[12];
            let v107 = staged[13];
            let v109 = 3.348580862e-29f64;
            let v116 = staged[14];
            let v129 = 5e-7f64;
            let v136 = 1e-3f64;
            let v155 = 6.666666666666666e-1f64;
            let v157 = -3.3333333333333337e-1f64;
            let v161 = staged[15];
            let v166 = 1.3333333333333333e0f64;
            let v182 = staged[16];
            let v192 = 7.071067811865475e-1f64;
            let v200 = 1e-5f64;
            let v202 = staged[17];
            let v205 = 4.6051701859880916e2f64;
            let v214 = 3.333333333333333e-1f64;
            let v228 = 1e-200f64;
            let v235 = 7.324648775608221e-1f64;
            let v238 = 1.25e0f64;
            let v240 = staged[18];
            let v250 = 1.666666666666667e-1f64;
            let v282 = staged[101];
            let v291 = 1e1f64;
            let v293 = 6e0f64;
            let v298 = 6.4e1f64;
            let v320 = 2e0f64;
            let v398 = 2.3025850929940458e2f64;
            let v400 = 7.324648775608221e-1f64;
            let v434 = -2.3025850929940458e2f64;
            let v456 = 1e100f64;
            let v540 = -2.3025850929940458e2f64;
            let v543 = -2.3025850929940458e2f64;
            let v547 = -2.3025850929940458e2f64;
            let v562 = 1e-100f64;
            let v575 = 2.5e-1f64;
            let v592 = 3e0f64;
            let v600 = 5e0f64;
            let v656 = 1e-40f64;
            let v693 = 1e-120f64;
            let v705 = 2.5e0f64;
            let v712 = 2.23606797749979e0f64;
            let v925 = staged[19];
            let v930 = staged[20];
            let v936 = staged[21];
            let v943 = staged[29];
            let v945 = staged[22];
            let v951 = staged[23];
            let v955 = staged[24];
            let v1005 = staged[25];
            let v1015 = staged[26];
            let v1089 = staged[27];
            let v1105 = -2.3025850929940458e2f64;
            let v1199 = -2.3025850929940458e2f64;
            let v1202 = -2.3025850929940458e2f64;
            let v1206 = -2.3025850929940458e2f64;
            let v1243 = staged[28];
            let v1332 = 2.5e0f64;
            let v1339 = 2.23606797749979e0f64;
            let v1719 = -2.3025850929940458e2f64;
            let v1824 = -2.3025850929940458e2f64;
            let v1827 = -2.3025850929940458e2f64;
            let v1831 = -2.3025850929940458e2f64;
            let v1983 = 2.5e0f64;
            let v1990 = 2.23606797749979e0f64;
            let v2203 = Lanes([0e0f64; 2]);
            let v2209 = node_potentials[6];
            let v2212 = Lanes([1e0f64; 1]);
            let v2347 = 1.75e0f64;
            let v2672 = 4e0f64;
            let v2697 = -2.3025850929940458e2f64;
            let v2762 = -2.3025850929940458e2f64;
            let v2765 = -2.3025850929940458e2f64;
            let v2769 = -2.3025850929940458e2f64;
            let v2810 = staged[30];
            let v2817 = Lanes([0e0f64; 3]);
            let v2979 = -2.3025850929940458e2f64;
            let v3073 = -2.3025850929940458e2f64;
            let v3076 = -2.3025850929940458e2f64;
            let v3080 = -2.3025850929940458e2f64;
            let v3205 = 2.5e0f64;
            let v3212 = 2.23606797749979e0f64;
            let v3705 = -2.3025850929940458e2f64;
            let v3770 = -2.3025850929940458e2f64;
            let v3773 = -2.3025850929940458e2f64;
            let v3777 = -2.3025850929940458e2f64;
            let v3881 = 1.62e0f64;
            let v3888 = staged[31];
            let v3893 = staged[32];
            let v3896 = staged[33];
            let v4023 = staged[34];
            let v4028 = staged[102];
            let v4070 = staged[35];
            let v4072 = -1.666666666666667e-1f64;
            let v4074 = -1.1666666666666667e0f64;
            let v4078 = staged[36];
            let v4094 = 1e-2f64;
            let v4110 = -1e0f64;
            let v4136 = 4e-2f64;
            let v4146 = staged[37];
            let v4149 = parameters[41];
            let v4168 = 5e-3f64;
            let v4175 = 1e-1f64;
            let v4183 = node_potentials[1];
            let v4186 = Lanes([1e0f64; 1]);
            let v4189 = staged[38];
            let v4195 = staged[105];
            let v4197 = staged[40];
            let v4199 = Lanes([0e0f64; 2]);
            let v4204 = staged[41];
            let v4215 = staged[42];
            let v4255 = staged[43];
            let v4374 = staged[44];
            let v4457 = -2.3025850929940458e2f64;
            let v4512 = -2.3025850929940458e2f64;
            let v4515 = -2.3025850929940458e2f64;
            let v4519 = -2.3025850929940458e2f64;
            let v4558 = staged[106];
            let v4567 = staged[65];
            let v4570 = staged[66];
            let v4579 = parameters[22];
            let v4582 = node_potentials[3];
            let v4584 = Lanes([1e0f64; 1]);
            let v4588 = staged[67];
            let v4592 = staged[107];
            let v4595 = staged[108];
            let v4598 = staged[45];
            let v4605 = staged[109];
            let v4631 = staged[46];
            let v4634 = staged[110];
            let v4645 = 5e-3f64;
            let v4652 = 1e-1f64;
            let v4660 = staged[47];
            let v4666 = 1.0f64;
            let v4695 = 5e-7f64;
            let v4702 = 1e-3f64;
            let v4712 = staged[48];
            let v4722 = staged[49];
            let v4756 = parameters[64];
            let v4759 = parameters[63];
            let v4765 = -1.5e0f64;
            let v4767 = staged[50];
            let v4786 = -2.3025850929940458e2f64;
            let v4790 = staged[51];
            let v4803 = -2.3025850929940458e2f64;
            let v4806 = -2.3025850929940458e2f64;
            let v4810 = -2.3025850929940458e2f64;
            let v4833 = staged[52];
            let v4865 = staged[53];
            let v4868 = staged[111];
            let v4879 = 5e-3f64;
            let v4886 = 1e-1f64;
            let v4894 = staged[54];
            let v4900 = 1.0f64;
            let v4929 = 5e-7f64;
            let v4936 = 1e-3f64;
            let v4946 = staged[55];
            let v4954 = staged[56];
            let v4986 = parameters[59];
            let v4989 = parameters[58];
            let v4995 = -1.5e0f64;
            let v4997 = staged[57];
            let v5004 = -2.3025850929940458e2f64;
            let v5008 = staged[58];
            let v5021 = -2.3025850929940458e2f64;
            let v5024 = -2.3025850929940458e2f64;
            let v5028 = -2.3025850929940458e2f64;
            let v5073 = staged[112];
            let v5084 = staged[113];
            let v5112 = staged[114];
            let v5123 = 5e-3f64;
            let v5130 = 1e-1f64;
            let v5143 = 0.0f64;
            let v5172 = 5e-7f64;
            let v5179 = 1e-3f64;
            let v5189 = staged[59];
            let v5199 = staged[60];
            let v5241 = -1.5e0f64;
            let v5243 = staged[61];
            let v5262 = -2.3025850929940458e2f64;
            let v5266 = staged[62];
            let v5279 = -2.3025850929940458e2f64;
            let v5282 = -2.3025850929940458e2f64;
            let v5286 = -2.3025850929940458e2f64;
            let v5342 = staged[115];
            let v5353 = 5e-3f64;
            let v5360 = 1e-1f64;
            let v5373 = 0.0f64;
            let v5402 = 5e-7f64;
            let v5409 = 1e-3f64;
            let v5465 = -1.5e0f64;
            let v5467 = staged[63];
            let v5474 = -2.3025850929940458e2f64;
            let v5478 = staged[64];
            let v5491 = -2.3025850929940458e2f64;
            let v5494 = -2.3025850929940458e2f64;
            let v5498 = -2.3025850929940458e2f64;
            let v5537 = node_potentials[0];
            let v5539 = Lanes([1e0f64; 1]);
            let v5543 = staged[68];
            let v5550 = staged[69];
            let v5557 = staged[70];
            let v5565 = node_potentials[2];
            let v5568 = Lanes([1e0f64; 1]);
            let v5571 = staged[71];
            let v5574 = Lanes([0e0f64; 2]);
            let v5575 = Lanes([0e0f64; 2]);
            let v5576 = Lanes([0e0f64; 4]);
            let v5577 = Lanes([0e0f64; 2]);
            let v5592 = parameters[65];
            let v5595 = ddt_scale();
            let v5605 = parameters[10];
            let v5 = v3 - v4;
            let v14 = v13 * (v5 - v11);
            let v15 = ((Lanes([v6[0], 0.0])) - (Lanes([0.0, v8[0]]))) * v13;
            let v17 = if v14 > v16 { 1.0 } else { 0.0 };
            let v39: f64;
            let v40: Lanes<2>;
            if v17 != 0.0 {
                let v19 = v15 * v14;
                let v23 = ((v14 * v14) + v21).sqrt();
                let v32 = v31 * (v14 + v23);
                let v33 = (v15 + ((v19 + v19) * (v26 / (v24 * v23)))) * v31;
                v39 = v32;
                v40 = v33;
            } else {
                let v35 = v34 - v14;
                let v37 = v15 * v36;
                let v38 = if v35 > v16 { 1.0 } else { 0.0 };
                let v71: f64;
                let v72: Lanes<2>;
                if v38 != 0.0 {
                    let v52 = v37 * v35;
                    let v55 = ((v35 * v35) + v21).sqrt();
                    let v59 = v35 + v55;
                    let v61 = (v31 * v21) / v59;
                    let v64 = (((v37 + ((v52 + v52) * (v26 / (v24 * v55)))) * v61) * v36) / v59;
                    v71 = v61;
                    v72 = v64;
                } else {
                    let v69 = v31 * (v14 + ((v65 + v21).sqrt()));
                    let v70 = v15 * v31;
                    v71 = v69;
                    v72 = v70;
                }
                v39 = v71;
                v40 = v72;
            }
            let v43 = v40 * v41;
            let v45 = v44 + (v41 * v39);
            let v47 = v46 - v45;
            let v48 = v43 * v36;
            let v49 = if v47 > v16 { 1.0 } else { 0.0 };
            let v90: f64;
            let v91: Lanes<2>;
            if v49 != 0.0 {
                let v74 = v48 * v47;
                let v78 = ((v47 * v47) + v76).sqrt();
                let v86 = v46 - (v31 * (v47 + v78));
                let v87 = ((v48 + ((v74 + v74) * (v26 / (v24 * v78)))) * v31) * v36;
                v90 = v86;
                v91 = v87;
            } else {
                let v88 = v45 - v46;
                let v89 = if v88 > v16 { 1.0 } else { 0.0 };
                let v142: f64;
                let v143: Lanes<2>;
                if v89 != 0.0 {
                    let v120 = v43 * v88;
                    let v123 = ((v88 * v88) + v76).sqrt();
                    let v127 = v88 + v123;
                    let v130 = v129 / v127;
                    let v134 = v46 - v130;
                    let v135 = ((((v43 + ((v120 + v120) * (v26 / (v24 * v123)))) * v130) * v36) / v127) * v36;
                    v142 = v134;
                    v143 = v135;
                } else {
                    let v140 = v46 - (v31 * (v47 + v136));
                    let v141 = (v48 * v31) * v36;
                    v142 = v140;
                    v143 = v141;
                }
                v90 = v142;
                v91 = v143;
            }
            let v93 = v92 * v90;
            let v94 = v91 * v92;
            let v96 = v93 / v95;
            let v97 = v94 / v95;
            let v99 = v93 * v98;
            let v106 = ((v94 * v98) * (v26 / v99)) * v104;
            let v108 = v107 + (v104 * (v99.ln()));
            let v112 = (v109 * v93).sqrt();
            let v117 = v112 / v116;
            let v118 = ((v94 * v109) * (v26 / (v24 * v112))) / v116;
            let v178: f64;
            let v179: f64;
            let v180: Lanes<2>;
            let v181: Lanes<2>;
            if v0 != 0.0 {
                let v144 = v117 * v117;
                let v145 = v118 * v117;
                let v151 = (v144 * v108).sqrt();
                let v154 = (((v145 + v145) * v108) + (v106 * v144)) * (v26 / (v24 * v151));
                let v162 = v161 * (v151.powf(v155));
                let v163 = (v154 * (v155 * (v151.powf(v157)))) * v161;
                let v164 = v108 + v162;
                let v165 = v106 + v163;
                let v169 = (v166 * v162) / v151;
                let v173 = v44 + v169;
                let v174 = v117 * v173;
                let v177 = (v118 * v173) + ((((v163 * v166) - (v154 * v169)) / v151) * v117);
                v178 = v174;
                v179 = v164;
                v180 = v177;
                v181 = v165;
            } else {
                v178 = v117;
                v179 = v108;
                v180 = v118;
                v181 = v106;
            }
            let v183 = v178 * v182;
            let v184 = v180 * v182;
            let v185 = v183 * v183;
            let v186 = v184 * v183;
            let v187 = v186 + v186;
            let v188 = v44 / v185;
            let v191 = ((v187 * v188) * v36) / v185;
            let v194 = v184 * v192;
            let v195 = v44 + (v183 * v192);
            let v196 = v44 / v195;
            let v199 = ((v194 * v196) * v36) / v195;
            let v201 = v200 * v195;
            let v203 = v179 * v202;
            let v204 = v181 * v202;
            let v206 = if v203 < v205 { 1.0 } else { 0.0 };
            let v233: f64;
            let v234: Lanes<2>;
            if v206 != 0.0 {
                let v209 = (-v203).exp();
                let v210 = (v204 * v36) * v209;
                v233 = v209;
                v234 = v210;
            } else {
                let v211 = v203 - v205;
                let v212 = v31 * v211;
                let v217 = v44 + (v211 * v214);
                let v222 = v44 + (v212 * v217);
                let v227 = v44 + (v211 * v222);
                let v229 = v228 / v227;
                let v232 = ((((v204 * v222) + ((((v204 * v31) * v217) + ((v204 * v214) * v212)) * v211)) * v229) * v36) / v227;
                v233 = v229;
                v234 = v232;
            }
            let v237 = v184 * v235;
            let v239 = v238 + (v183 * v235);
            let v242 = v13 * (v5 - v240);
            let v243 = v242 * v202;
            let v244 = v15 * v202;
            let v246 = if (v243.abs()) <= v201 { 1.0 } else { 0.0 };
            let v280: f64;
            let v281: Lanes<2>;
            if v246 != 0.0 {
                let v248 = v199 * v196;
                let v253 = ((v196 * v196) * v250) * v192;
                let v255 = v243 * v196;
                let v259 = v44 - v233;
                let v261 = v243 * v259;
                let v265 = v261 * v183;
                let v273 = v44 + (v265 * v253);
                let v274 = v255 * v273;
                let v277 = (((v244 * v196) + (v199 * v243)) * v273) + (((((((v244 * v259) + ((v234 * v36) * v243)) * v183) + (v184 * v261)) * v253) + ((((v248 + v248) * v250) * v192) * v265)) * v255);
                v280 = v274;
                v281 = v277;
            } else {
                let v279 = if v243 < (-v201) { 1.0 } else { 0.0 };
                let v436: f64;
                let v437: Lanes<2>;
                if v279 != 0.0 {
                    let v283 = -v243;
                    let v284 = v244 * v36;
                    let v285 = v238 * v283;
                    let v287 = v285 * v196;
                    let v290 = ((v284 * v238) * v196) + (v199 * v285);
                    let v294 = v287 - v293;
                    let v296 = v290 * v294;
                    let v300 = ((v294 * v294) + v298).sqrt();
                    let v306 = v31 * ((v287 + v291) - v300);
                    let v307 = (v290 - ((v296 + v296) * (v26 / (v24 * v300)))) * v31;
                    let v308 = v283 - v306;
                    let v309 = v284 - v307;
                    let v311 = v309 * v308;
                    let v313 = v306 + v44;
                    let v318 = (v308 * v308) + (v185 * v313);
                    let v319 = (v311 + v311) + ((v187 * v313) + (v307 * v185));
                    let v323 = (v320 * v308) - v185;
                    let v324 = (v309 * v320) - v187;
                    let v327 = v318 * v188;
                    let v334 = (-v306) + (v327.ln());
                    let v335 = (v307 * v36) + (((v319 * v188) + (v191 * v318)) * (v26 / v327));
                    let v336 = v318 + v323;
                    let v337 = v319 + v324;
                    let v339 = v337 * v336;
                    let v341 = v31 * v323;
                    let v347 = (v341 * v323) - v318;
                    let v353 = (v336 * v336) + (v347 * v334);
                    let v354 = (v339 + v339) + ((((((v324 * v31) * v323) + (v324 * v341)) - v319) * v334) + (v335 * v347));
                    let v355 = v318 * v336;
                    let v363 = v336 * v334;
                    let v371 = (v363 * v334) / v353;
                    let v375 = v371 * v323;
                    let v380 = v324 * v323;
                    let v384 = ((v323 * v323) * v214) - v318;
                    let v390 = v353 + (v375 * v384);
                    let v392 = (v355 * v334) / v390;
                    let v396 = v306 + v392;
                    let v397 = v307 + ((((((v319 * v336) + (v337 * v318)) * v334) + (v335 * v355)) - ((v354 + ((((((((((v337 * v334) + (v335 * v336)) * v334) + (v335 * v363)) - (v354 * v371)) / v353) * v323) + (v324 * v371)) * v384) + ((((v380 + v380) * v214) - v319) * v375))) * v392)) / v390);
                    let v399 = if v396 < v398 { 1.0 } else { 0.0 };
                    let v459: f64;
                    let v460: Lanes<2>;
                    if v399 != 0.0 {
                        let v438 = v396.exp();
                        let v439 = v397 * v438;
                        v459 = v438;
                        v460 = v439;
                    } else {
                        let v440 = v396 - v398;
                        let v441 = v31 * v440;
                        let v445 = v44 + (v440 * v214);
                        let v450 = v44 + (v441 * v445);
                        let v457 = v456 * (v44 + (v440 * v450));
                        let v458 = ((v397 * v450) + ((((v397 * v31) * v445) + ((v397 * v214) * v441)) * v440)) * v456;
                        v459 = v457;
                        v460 = v458;
                    }
                    let v461 = v44 / v459;
                    let v465 = v283 - v396;
                    let v466 = v284 - v397;
                    let v467 = v233 * v461;
                    let v470 = (v234 * v461) + ((((v460 * v461) * v36) / v459) * v233);
                    let v476 = ((v459 - v44) - v467) + v233;
                    let v482 = (v320 * v465) + (v185 * v476);
                    let v483 = (v466 * v320) + ((v187 * v476) + (((v460 - v470) + v234) * v185));
                    let v485 = v466 * v465;
                    let v492 = v396 - v44;
                    let v497 = (((v459 - v396) - v44) + v467) + (v233 * v492);
                    let v505 = v459 + v467;
                    let v511 = v320 - (v185 * v505);
                    let v514 = v483 * v482;
                    let v516 = v320 * ((v465 * v465) - (v185 * v497));
                    let v517 = ((v485 + v485) - ((v187 * v497) + ((((v460 - v397) + v470) + ((v234 * v492) + (v397 * v233))) * v185))) * v320;
                    let v526 = ((v482 * v482) - (v516 * v511)).sqrt();
                    let v530 = v482 + v526;
                    let v532 = v516 / v530;
                    let v536 = (-v396) - v532;
                    let v537 = (v397 * v36) - ((v517 - ((v483 + (((v514 + v514) - ((v517 * v511) + ((((v187 * v505) + ((v460 + v470) * v185)) * v36) * v516))) * (v26 / (v24 * v526)))) * v532)) / v530);
                    v436 = v536;
                    v437 = v537;
                } else {
                    let v403 = v238 + (v183 * v400);
                    let v404 = v44 / v403;
                    let v407 = (((v184 * v400) * v404) * v36) / v403;
                    let v408 = v195 * v238;
                    let v414 = (v408 * v404) - v44;
                    let v415 = v414 * v404;
                    let v419 = v243 * v196;
                    let v427 = v44 + (v415 * v243);
                    let v432 = -(v419 * v427);
                    let v433 = ((((v244 * v196) + (v199 * v243)) * v427) + ((((((((v194 * v238) * v404) + (v407 * v408)) * v404) + (v407 * v414)) * v243) + (v244 * v415)) * v419)) * v36;
                    let v435 = if v432 > v434 { 1.0 } else { 0.0 };
                    let v567: f64;
                    let v568: Lanes<2>;
                    if v435 != 0.0 {
                        let v538 = v432.exp();
                        let v539 = v433 * v538;
                        v567 = v538;
                        v568 = v539;
                    } else {
                        let v541 = v540 - v432;
                        let v542 = v433 * v36;
                        let v545 = v31 * (v543 - v432);
                        let v551 = v44 + ((v547 - v432) * v214);
                        let v556 = v44 + (v545 * v551);
                        let v561 = v44 + (v541 * v556);
                        let v563 = v562 / v561;
                        let v566 = ((((v542 * v556) + ((((v542 * v31) * v551) + ((v542 * v214) * v545)) * v541)) * v563) * v36) / v561;
                        v567 = v563;
                        v568 = v566;
                    }
                    let v571 = v185 * v31;
                    let v572 = v187 * v31;
                    let v582 = ((v243 + (v185 * v575)) - (v44 - v567)).sqrt();
                    let v590 = (v243 + v571) - (v183 * v582);
                    let v591 = (v244 + v572) - ((v184 * v582) + ((((v244 + (v187 * v575)) - (v568 * v36)) * (v26 / (v24 * v582))) * v183));
                    let v593 = v203 + v592;
                    let v594 = v593 - v590;
                    let v595 = v204 - v591;
                    let v596 = if v594 > v16 { 1.0 } else { 0.0 };
                    let v615: f64;
                    let v616: Lanes<2>;
                    if v596 != 0.0 {
                        let v598 = v595 * v594;
                        let v602 = ((v594 * v594) + v600).sqrt();
                        let v610 = v593 - (v31 * (v594 + v602));
                        let v611 = v204 - ((v595 + ((v598 + v598) * (v26 / (v24 * v602)))) * v31);
                        v615 = v610;
                        v616 = v611;
                    } else {
                        let v612 = v590 - v593;
                        let v613 = v591 - v204;
                        let v614 = if v612 > v16 { 1.0 } else { 0.0 };
                        let v718: f64;
                        let v719: Lanes<2>;
                        if v614 != 0.0 {
                            let v696 = v613 * v612;
                            let v699 = ((v612 * v612) + v600).sqrt();
                            let v703 = v612 + v699;
                            let v706 = v705 / v703;
                            let v710 = v593 - v706;
                            let v711 = v204 - ((((v613 + ((v696 + v696) * (v26 / (v24 * v699)))) * v706) * v36) / v703);
                            v718 = v710;
                            v719 = v711;
                        } else {
                            let v716 = v593 - (v31 * (v594 + v712));
                            let v717 = v204 - (v595 * v31);
                            v718 = v716;
                            v719 = v717;
                        }
                        v615 = v718;
                        v616 = v719;
                    }
                    let v618 = v204 * v593;
                    let v621 = ((v593 * v593) + v600).sqrt();
                    let v629 = v615 - (v31 * (v593 - v621));
                    let v630 = v616 - ((v204 - ((v618 + v618) * (v26 / (v24 * v621)))) * v31);
                    let v631 = v243 - v629;
                    let v632 = v244 - v630;
                    let v635 = (-v629).exp();
                    let v636 = (v630 * v36) * v635;
                    let v638 = v632 * v631;
                    let v643 = v629 + v44;
                    let v648 = ((v635 + v629) - v44) - (v233 * v643);
                    let v654 = (v631 * v631) - (v185 * v648);
                    let v657 = if v656 >= v654 { v656 } else { v654 };
                    let v660 = ((v638 + v638) - ((v187 * v648) + (((v636 + v630) - ((v234 * v643) + (v630 * v233))) * v185))) * (v26 - (if v656 >= v654 { 1.0 } else { 0.0 }));
                    let v665 = v44 - (v571 * v635);
                    let v666 = ((v572 * v635) + (v636 * v571)) * v36;
                    let v671 = (v44 - v635) - v233;
                    let v677 = (v320 * v631) + (v185 * v671);
                    let v678 = (v632 * v320) + ((v187 * v671) + (((v636 * v36) - v234) * v185));
                    let v681 = v657 / v185;
                    let v688 = (v203 - v629) + (v681.ln());
                    let v689 = (v204 - v630) + (((v660 - (v187 * v681)) / v185) * (v26 / v681));
                    let v690 = v657 + v677;
                    let v691 = v660 + v678;
                    let v694 = if (v688.abs()) < v693 { 1.0 } else { 0.0 };
                    let v784: f64;
                    let v785: Lanes<2>;
                    if v694 != 0.0 {
                        v784 = v629;
                        v785 = v630;
                    } else {
                        let v721 = v691 * v690;
                        let v723 = v31 * v677;
                        let v729 = v657 * v665;
                        let v732 = (v660 * v665) + (v666 * v657);
                        let v733 = (v723 * v677) - v729;
                        let v739 = (v690 * v690) + (v733 * v688);
                        let v740 = (v721 + v721) + ((((((v678 * v31) * v677) + (v678 * v723)) - v732) * v688) + (v689 * v733));
                        let v741 = v657 * v690;
                        let v749 = v690 * v688;
                        let v757 = (v749 * v688) / v739;
                        let v761 = v757 * v677;
                        let v766 = v678 * v677;
                        let v770 = ((v677 * v677) * v214) - v729;
                        let v776 = v739 + (v761 * v770);
                        let v778 = (v741 * v688) / v776;
                        let v782 = v629 + v778;
                        let v783 = v630 + ((((((v660 * v690) + (v691 * v657)) * v688) + (v689 * v741)) - ((v740 + ((((((((((v691 * v688) + (v689 * v690)) * v688) + (v689 * v749)) - (v740 * v757)) / v739) * v677) + (v678 * v757)) * v770) + ((((v766 + v766) * v214) - v732) * v761))) * v778)) / v776);
                        v784 = v782;
                        v785 = v783;
                    }
                    let v786 = if v784 < v398 { 1.0 } else { 0.0 };
                    let v799: f64;
                    let v800: f64;
                    let v801: Lanes<2>;
                    let v802: Lanes<2>;
                    if v786 != 0.0 {
                        let v787 = v784.exp();
                        let v788 = v785 * v787;
                        let v789 = v44 / v787;
                        let v792 = ((v788 * v789) * v36) / v787;
                        let v793 = v233 * v787;
                        let v796 = (v234 * v787) + (v788 * v233);
                        v799 = v789;
                        v800 = v793;
                        v801 = v792;
                        v802 = v796;
                    } else {
                        let v798 = if v784 > (v203 - v398) { 1.0 } else { 0.0 };
                        let v921: f64;
                        let v922: f64;
                        let v923: Lanes<2>;
                        let v924: Lanes<2>;
                        if v798 != 0.0 {
                            let v873 = (v784 - v203).exp();
                            let v874 = (v785 - v204) * v873;
                            let v875 = v233 / v873;
                            let v878 = (v234 - (v874 * v875)) / v873;
                            v921 = v875;
                            v922 = v873;
                            v923 = v878;
                            v924 = v874;
                        } else {
                            let v880 = v204 - v785;
                            let v881 = (v203 - v784) - v398;
                            let v882 = v31 * v881;
                            let v886 = v44 + (v881 * v214);
                            let v891 = v44 + (v882 * v886);
                            let v896 = v44 + (v881 * v891);
                            let v897 = v562 / v896;
                            let v900 = ((((v880 * v891) + ((((v880 * v31) * v886) + ((v880 * v214) * v882)) * v881)) * v897) * v36) / v896;
                            let v901 = v784 - v398;
                            let v902 = v31 * v901;
                            let v906 = v44 + (v901 * v214);
                            let v911 = v44 + (v902 * v906);
                            let v916 = v44 + (v901 * v911);
                            let v917 = v562 / v916;
                            let v920 = ((((v785 * v911) + ((((v785 * v31) * v906) + ((v785 * v214) * v902)) * v901)) * v917) * v36) / v916;
                            v921 = v917;
                            v922 = v897;
                            v923 = v920;
                            v924 = v900;
                        }
                        v799 = v921;
                        v800 = v922;
                        v801 = v923;
                        v802 = v924;
                    }
                    let v803 = v243 - v784;
                    let v804 = v244 - v785;
                    let v811 = ((v44 - v799) + v800) - v233;
                    let v817 = (v320 * v803) + (v185 * v811);
                    let v818 = (v804 * v320) + ((v187 * v811) + ((((v801 * v36) + v802) - v234) * v185));
                    let v820 = v804 * v803;
                    let v827 = v784 + v44;
                    let v832 = (((v799 + v784) - v44) + v800) - (v233 * v827);
                    let v840 = v799 + v800;
                    let v846 = v320 - (v185 * v840);
                    let v849 = v818 * v817;
                    let v851 = v320 * ((v803 * v803) - (v185 * v832));
                    let v852 = ((v820 + v820) - ((v187 * v832) + ((((v801 + v785) + v802) - ((v234 * v827) + (v785 * v233))) * v185))) * v320;
                    let v859 = ((v817 * v817) - (v851 * v846)).sqrt();
                    let v863 = v817 + v859;
                    let v865 = v851 / v863;
                    let v869 = v784 + v865;
                    let v870 = v785 + ((v852 - ((v818 + (((v849 + v849) - ((v852 * v846) + ((((v187 * v840) + ((v801 + v802) * v185)) * v36) * v851))) * (v26 / (v24 * v859)))) * v865)) / v863);
                    v436 = v869;
                    v437 = v870;
                }
                v280 = v436;
                v281 = v437;
            }
            let v938: f64;
            let v939: f64;
            let v940: Lanes<2>;
            let v941: Lanes<2>;
            if v282 != 0.0 {
                let v933 = (v930 * (v242 - (v280 * v925))) * v202;
                let v934 = ((v15 - (v281 * v925)) * v930) * v202;
                let v937 = if (v933.abs()) <= v936 { 1.0 } else { 0.0 };
                let v967: f64;
                let v968: Lanes<2>;
                if v937 != 0.0 {
                    let v948 = ((v945 * v945) * v250) * v192;
                    let v949 = v933 * v945;
                    let v952 = v44 - v951;
                    let v960 = v44 + (((v933 * v952) * v955) * v948);
                    let v961 = v949 * v960;
                    let v964 = ((v934 * v945) * v960) + ((((v934 * v952) * v955) * v948) * v949);
                    v967 = v961;
                    v968 = v964;
                } else {
                    let v966 = if v933 < (-v936) { 1.0 } else { 0.0 };
                    let v1107: f64;
                    let v1108: Lanes<2>;
                    if v966 != 0.0 {
                        let v979 = -v933;
                        let v980 = v934 * v36;
                        let v983 = (v238 * v979) * v945;
                        let v984 = (v980 * v238) * v945;
                        let v986 = v983 - v293;
                        let v988 = v984 * v986;
                        let v991 = ((v986 * v986) + v298).sqrt();
                        let v997 = v31 * ((v983 + v291) - v991);
                        let v998 = (v984 - ((v988 + v988) * (v26 / (v24 * v991)))) * v31;
                        let v999 = v979 - v997;
                        let v1000 = v980 - v998;
                        let v1002 = v1000 * v999;
                        let v1008 = (v999 * v999) + (v1005 * (v997 + v44));
                        let v1009 = (v1002 + v1002) + (v998 * v1005);
                        let v1011 = v1000 * v320;
                        let v1012 = (v320 * v999) - v1005;
                        let v1016 = v1008 * v1015;
                        let v1021 = (-v997) + (v1016.ln());
                        let v1022 = (v998 * v36) + ((v1009 * v1015) * (v26 / v1016));
                        let v1023 = v1008 + v1012;
                        let v1024 = v1009 + v1011;
                        let v1026 = v1024 * v1023;
                        let v1028 = v31 * v1012;
                        let v1034 = (v1028 * v1012) - v1008;
                        let v1040 = (v1023 * v1023) + (v1034 * v1021);
                        let v1041 = (v1026 + v1026) + ((((((v1011 * v31) * v1012) + (v1011 * v1028)) - v1009) * v1021) + (v1022 * v1034));
                        let v1042 = v1008 * v1023;
                        let v1050 = v1023 * v1021;
                        let v1058 = (v1050 * v1021) / v1040;
                        let v1062 = v1058 * v1012;
                        let v1067 = v1011 * v1012;
                        let v1071 = ((v1012 * v1012) * v214) - v1008;
                        let v1077 = v1040 + (v1062 * v1071);
                        let v1079 = (v1042 * v1021) / v1077;
                        let v1083 = v997 + v1079;
                        let v1084 = v998 + ((((((v1009 * v1023) + (v1024 * v1008)) * v1021) + (v1022 * v1042)) - ((v1041 + ((((((((((v1024 * v1021) + (v1022 * v1023)) * v1021) + (v1022 * v1050)) - (v1041 * v1058)) / v1040) * v1012) + (v1011 * v1058)) * v1071) + ((((v1067 + v1067) * v214) - v1009) * v1062))) * v1079)) / v1077);
                        let v1085 = if v1083 < v398 { 1.0 } else { 0.0 };
                        let v1129: f64;
                        let v1130: Lanes<2>;
                        if v1085 != 0.0 {
                            let v1109 = v1083.exp();
                            let v1110 = v1084 * v1109;
                            v1129 = v1109;
                            v1130 = v1110;
                        } else {
                            let v1111 = v1083 - v398;
                            let v1112 = v31 * v1111;
                            let v1116 = v44 + (v1111 * v214);
                            let v1121 = v44 + (v1112 * v1116);
                            let v1127 = v456 * (v44 + (v1111 * v1121));
                            let v1128 = ((v1084 * v1121) + ((((v1084 * v31) * v1116) + ((v1084 * v214) * v1112)) * v1111)) * v456;
                            v1129 = v1127;
                            v1130 = v1128;
                        }
                        let v1131 = v44 / v1129;
                        let v1135 = v979 - v1083;
                        let v1136 = v980 - v1084;
                        let v1137 = v951 * v1131;
                        let v1138 = (((v1130 * v1131) * v36) / v1129) * v951;
                        let v1147 = (v320 * v1135) + (v1005 * (((v1129 - v44) - v1137) + v951));
                        let v1148 = (v1136 * v320) + ((v1130 - v1138) * v1005);
                        let v1150 = v1136 * v1135;
                        let v1170 = v320 - (v1005 * (v1129 + v1137));
                        let v1173 = v1148 * v1147;
                        let v1175 = v320 * ((v1135 * v1135) - (v1005 * ((((v1129 - v1083) - v44) + v1137) + (v951 * (v1083 - v44)))));
                        let v1176 = ((v1150 + v1150) - ((((v1130 - v1084) + v1138) + (v1084 * v951)) * v1005)) * v320;
                        let v1185 = ((v1147 * v1147) - (v1175 * v1170)).sqrt();
                        let v1189 = v1147 + v1185;
                        let v1191 = v1175 / v1189;
                        let v1195 = (-v1083) - v1191;
                        let v1196 = (v1084 * v36) - ((v1176 - ((v1148 + (((v1173 + v1173) - ((v1176 * v1170) + ((((v1130 + v1138) * v1005) * v36) * v1175))) * (v26 / (v24 * v1185)))) * v1191)) / v1189);
                        v1107 = v1195;
                        v1108 = v1196;
                    } else {
                        let v1088 = v44 / (v238 + (v955 * v400));
                        let v1093 = (((v1089 * v238) * v1088) - v44) * v1088;
                        let v1094 = v933 * v945;
                        let v1098 = v44 + (v1093 * v933);
                        let v1103 = -(v1094 * v1098);
                        let v1104 = (((v934 * v945) * v1098) + ((v934 * v1093) * v1094)) * v36;
                        let v1106 = if v1103 > v1105 { 1.0 } else { 0.0 };
                        let v1225: f64;
                        let v1226: Lanes<2>;
                        if v1106 != 0.0 {
                            let v1197 = v1103.exp();
                            let v1198 = v1104 * v1197;
                            v1225 = v1197;
                            v1226 = v1198;
                        } else {
                            let v1200 = v1199 - v1103;
                            let v1201 = v1104 * v36;
                            let v1204 = v31 * (v1202 - v1103);
                            let v1210 = v44 + ((v1206 - v1103) * v214);
                            let v1215 = v44 + (v1204 * v1210);
                            let v1220 = v44 + (v1200 * v1215);
                            let v1221 = v562 / v1220;
                            let v1224 = ((((v1201 * v1215) + ((((v1201 * v31) * v1210) + ((v1201 * v214) * v1204)) * v1200)) * v1221) * v36) / v1220;
                            v1225 = v1221;
                            v1226 = v1224;
                        }
                        let v1229 = v1005 * v31;
                        let v1235 = ((v933 + (v1005 * v575)) - (v44 - v1225)).sqrt();
                        let v1241 = (v933 + v1229) - (v955 * v1235);
                        let v1242 = v934 - (((v934 - (v1226 * v36)) * (v26 / (v24 * v1235))) * v955);
                        let v1244 = v1243 + v592;
                        let v1245 = v1244 - v1241;
                        let v1246 = v1242 * v36;
                        let v1247 = if v1245 > v16 { 1.0 } else { 0.0 };
                        let v1264: f64;
                        let v1265: Lanes<2>;
                        if v1247 != 0.0 {
                            let v1249 = v1246 * v1245;
                            let v1252 = ((v1245 * v1245) + v600).sqrt();
                            let v1260 = v1244 - (v31 * (v1245 + v1252));
                            let v1261 = ((v1246 + ((v1249 + v1249) * (v26 / (v24 * v1252)))) * v31) * v36;
                            v1264 = v1260;
                            v1265 = v1261;
                        } else {
                            let v1262 = v1241 - v1244;
                            let v1263 = if v1262 > v16 { 1.0 } else { 0.0 };
                            let v1345: f64;
                            let v1346: Lanes<2>;
                            if v1263 != 0.0 {
                                let v1323 = v1242 * v1262;
                                let v1326 = ((v1262 * v1262) + v600).sqrt();
                                let v1330 = v1262 + v1326;
                                let v1333 = v1332 / v1330;
                                let v1337 = v1244 - v1333;
                                let v1338 = ((((v1242 + ((v1323 + v1323) * (v26 / (v24 * v1326)))) * v1333) * v36) / v1330) * v36;
                                v1345 = v1337;
                                v1346 = v1338;
                            } else {
                                let v1343 = v1244 - (v31 * (v1245 + v1339));
                                let v1344 = (v1246 * v31) * v36;
                                v1345 = v1343;
                                v1346 = v1344;
                            }
                            v1264 = v1345;
                            v1265 = v1346;
                        }
                        let v1271 = v1264 - (v31 * (v1244 - (((v1244 * v1244) + v600).sqrt())));
                        let v1272 = v933 - v1271;
                        let v1273 = v934 - v1265;
                        let v1275 = v1265 * v36;
                        let v1276 = (-v1271).exp();
                        let v1277 = v1275 * v1276;
                        let v1279 = v1273 * v1272;
                        let v1291 = (v1272 * v1272) - (v1005 * (((v1276 + v1271) - v44) - (v951 * (v1271 + v44))));
                        let v1293 = if v656 >= v1291 { v656 } else { v1291 };
                        let v1296 = ((v1279 + v1279) - (((v1277 + v1265) - (v1265 * v951)) * v1005)) * (v26 - (if v656 >= v1291 { 1.0 } else { 0.0 }));
                        let v1299 = v44 - (v1229 * v1276);
                        let v1300 = (v1277 * v1229) * v36;
                        let v1308 = (v320 * v1272) + (v1005 * ((v44 - v1276) - v951));
                        let v1309 = (v1273 * v320) + ((v1277 * v36) * v1005);
                        let v1311 = v1293 / v1005;
                        let v1316 = (v1243 - v1271) + (v1311.ln());
                        let v1317 = v1275 + ((v1296 / v1005) * (v26 / v1311));
                        let v1318 = v1293 + v1308;
                        let v1319 = v1296 + v1309;
                        let v1321 = if (v1316.abs()) < v693 { 1.0 } else { 0.0 };
                        let v1411: f64;
                        let v1412: Lanes<2>;
                        if v1321 != 0.0 {
                            v1411 = v1271;
                            v1412 = v1265;
                        } else {
                            let v1348 = v1319 * v1318;
                            let v1350 = v31 * v1308;
                            let v1356 = v1293 * v1299;
                            let v1359 = (v1296 * v1299) + (v1300 * v1293);
                            let v1360 = (v1350 * v1308) - v1356;
                            let v1366 = (v1318 * v1318) + (v1360 * v1316);
                            let v1367 = (v1348 + v1348) + ((((((v1309 * v31) * v1308) + (v1309 * v1350)) - v1359) * v1316) + (v1317 * v1360));
                            let v1368 = v1293 * v1318;
                            let v1376 = v1318 * v1316;
                            let v1384 = (v1376 * v1316) / v1366;
                            let v1388 = v1384 * v1308;
                            let v1393 = v1309 * v1308;
                            let v1397 = ((v1308 * v1308) * v214) - v1356;
                            let v1403 = v1366 + (v1388 * v1397);
                            let v1405 = (v1368 * v1316) / v1403;
                            let v1409 = v1271 + v1405;
                            let v1410 = v1265 + ((((((v1296 * v1318) + (v1319 * v1293)) * v1316) + (v1317 * v1368)) - ((v1367 + ((((((((((v1319 * v1316) + (v1317 * v1318)) * v1316) + (v1317 * v1376)) - (v1367 * v1384)) / v1366) * v1308) + (v1309 * v1384)) * v1397) + ((((v1393 + v1393) * v214) - v1359) * v1388))) * v1405)) / v1403);
                            v1411 = v1409;
                            v1412 = v1410;
                        }
                        let v1413 = if v1411 < v398 { 1.0 } else { 0.0 };
                        let v1424: f64;
                        let v1425: f64;
                        let v1426: Lanes<2>;
                        let v1427: Lanes<2>;
                        if v1413 != 0.0 {
                            let v1414 = v1411.exp();
                            let v1415 = v1412 * v1414;
                            let v1416 = v44 / v1414;
                            let v1419 = ((v1415 * v1416) * v36) / v1414;
                            let v1420 = v951 * v1414;
                            let v1421 = v1415 * v951;
                            v1424 = v1416;
                            v1425 = v1420;
                            v1426 = v1419;
                            v1427 = v1421;
                        } else {
                            let v1423 = if v1411 > (v1243 - v398) { 1.0 } else { 0.0 };
                            let v1536: f64;
                            let v1537: f64;
                            let v1538: Lanes<2>;
                            let v1539: Lanes<2>;
                            if v1423 != 0.0 {
                                let v1488 = (v1411 - v1243).exp();
                                let v1489 = v1412 * v1488;
                                let v1490 = v951 / v1488;
                                let v1493 = ((v1489 * v1490) * v36) / v1488;
                                v1536 = v1490;
                                v1537 = v1488;
                                v1538 = v1493;
                                v1539 = v1489;
                            } else {
                                let v1495 = v1412 * v36;
                                let v1496 = (v1243 - v1411) - v398;
                                let v1497 = v31 * v1496;
                                let v1501 = v44 + (v1496 * v214);
                                let v1506 = v44 + (v1497 * v1501);
                                let v1511 = v44 + (v1496 * v1506);
                                let v1512 = v562 / v1511;
                                let v1515 = ((((v1495 * v1506) + ((((v1495 * v31) * v1501) + ((v1495 * v214) * v1497)) * v1496)) * v1512) * v36) / v1511;
                                let v1516 = v1411 - v398;
                                let v1517 = v31 * v1516;
                                let v1521 = v44 + (v1516 * v214);
                                let v1526 = v44 + (v1517 * v1521);
                                let v1531 = v44 + (v1516 * v1526);
                                let v1532 = v562 / v1531;
                                let v1535 = ((((v1412 * v1526) + ((((v1412 * v31) * v1521) + ((v1412 * v214) * v1517)) * v1516)) * v1532) * v36) / v1531;
                                v1536 = v1532;
                                v1537 = v1512;
                                v1538 = v1535;
                                v1539 = v1515;
                            }
                            v1424 = v1536;
                            v1425 = v1537;
                            v1426 = v1538;
                            v1427 = v1539;
                        }
                        let v1428 = v933 - v1411;
                        let v1429 = v934 - v1412;
                        let v1439 = (v320 * v1428) + (v1005 * (((v44 - v1424) + v1425) - v951));
                        let v1440 = (v1429 * v320) + (((v1426 * v36) + v1427) * v1005);
                        let v1442 = v1429 * v1428;
                        let v1462 = v320 - (v1005 * (v1424 + v1425));
                        let v1465 = v1440 * v1439;
                        let v1467 = v320 * ((v1428 * v1428) - (v1005 * ((((v1424 + v1411) - v44) + v1425) - (v951 * (v1411 + v44)))));
                        let v1468 = ((v1442 + v1442) - ((((v1426 + v1412) + v1427) - (v1412 * v951)) * v1005)) * v320;
                        let v1475 = ((v1439 * v1439) - (v1467 * v1462)).sqrt();
                        let v1479 = v1439 + v1475;
                        let v1481 = v1467 / v1479;
                        let v1485 = v1411 + v1481;
                        let v1486 = v1412 + ((v1468 - ((v1440 + (((v1465 + v1465) - ((v1468 * v1462) + ((((v1426 + v1427) * v1005) * v36) * v1467))) * (v26 / (v24 * v1475)))) * v1481)) / v1479);
                        v1107 = v1485;
                        v1108 = v1486;
                    }
                    v967 = v1107;
                    v968 = v1108;
                }
                let v975 = (v242 - ((v930 * v967) * v925)) / v925;
                let v976 = (v15 - ((v968 * v930) * v925)) / v925;
                let v978 = if (v975.abs()) <= v201 { 1.0 } else { 0.0 };
                let v1572: f64;
                let v1573: Lanes<2>;
                if v978 != 0.0 {
                    let v1541 = v199 * v196;
                    let v1545 = ((v196 * v196) * v250) * v192;
                    let v1547 = v975 * v196;
                    let v1551 = v44 - v233;
                    let v1553 = v975 * v1551;
                    let v1557 = v1553 * v183;
                    let v1565 = v44 + (v1557 * v1545);
                    let v1566 = v1547 * v1565;
                    let v1569 = (((v976 * v196) + (v199 * v975)) * v1565) + (((((((v976 * v1551) + ((v234 * v36) * v975)) * v183) + (v184 * v1553)) * v1545) + ((((v1541 + v1541) * v250) * v192) * v1557)) * v1547);
                    v1572 = v1566;
                    v1573 = v1569;
                } else {
                    let v1571 = if v975 < (-v201) { 1.0 } else { 0.0 };
                    let v1721: f64;
                    let v1722: Lanes<2>;
                    if v1571 != 0.0 {
                        let v1574 = -v975;
                        let v1575 = v976 * v36;
                        let v1576 = v238 * v1574;
                        let v1578 = v1576 * v196;
                        let v1581 = ((v1575 * v238) * v196) + (v199 * v1576);
                        let v1583 = v1578 - v293;
                        let v1585 = v1581 * v1583;
                        let v1588 = ((v1583 * v1583) + v298).sqrt();
                        let v1594 = v31 * ((v1578 + v291) - v1588);
                        let v1595 = (v1581 - ((v1585 + v1585) * (v26 / (v24 * v1588)))) * v31;
                        let v1596 = v1574 - v1594;
                        let v1597 = v1575 - v1595;
                        let v1599 = v1597 * v1596;
                        let v1601 = v1594 + v44;
                        let v1606 = (v1596 * v1596) + (v185 * v1601);
                        let v1607 = (v1599 + v1599) + ((v187 * v1601) + (v1595 * v185));
                        let v1610 = (v320 * v1596) - v185;
                        let v1611 = (v1597 * v320) - v187;
                        let v1614 = v1606 * v188;
                        let v1621 = (-v1594) + (v1614.ln());
                        let v1622 = (v1595 * v36) + (((v1607 * v188) + (v191 * v1606)) * (v26 / v1614));
                        let v1623 = v1606 + v1610;
                        let v1624 = v1607 + v1611;
                        let v1626 = v1624 * v1623;
                        let v1628 = v31 * v1610;
                        let v1634 = (v1628 * v1610) - v1606;
                        let v1640 = (v1623 * v1623) + (v1634 * v1621);
                        let v1641 = (v1626 + v1626) + ((((((v1611 * v31) * v1610) + (v1611 * v1628)) - v1607) * v1621) + (v1622 * v1634));
                        let v1642 = v1606 * v1623;
                        let v1650 = v1623 * v1621;
                        let v1658 = (v1650 * v1621) / v1640;
                        let v1662 = v1658 * v1610;
                        let v1667 = v1611 * v1610;
                        let v1671 = ((v1610 * v1610) * v214) - v1606;
                        let v1677 = v1640 + (v1662 * v1671);
                        let v1679 = (v1642 * v1621) / v1677;
                        let v1683 = v1594 + v1679;
                        let v1684 = v1595 + ((((((v1607 * v1623) + (v1624 * v1606)) * v1621) + (v1622 * v1642)) - ((v1641 + ((((((((((v1624 * v1621) + (v1622 * v1623)) * v1621) + (v1622 * v1650)) - (v1641 * v1658)) / v1640) * v1610) + (v1611 * v1658)) * v1671) + ((((v1667 + v1667) * v214) - v1607) * v1662))) * v1679)) / v1677);
                        let v1685 = if v1683 < v398 { 1.0 } else { 0.0 };
                        let v1743: f64;
                        let v1744: Lanes<2>;
                        if v1685 != 0.0 {
                            let v1723 = v1683.exp();
                            let v1724 = v1684 * v1723;
                            v1743 = v1723;
                            v1744 = v1724;
                        } else {
                            let v1725 = v1683 - v398;
                            let v1726 = v31 * v1725;
                            let v1730 = v44 + (v1725 * v214);
                            let v1735 = v44 + (v1726 * v1730);
                            let v1741 = v456 * (v44 + (v1725 * v1735));
                            let v1742 = ((v1684 * v1735) + ((((v1684 * v31) * v1730) + ((v1684 * v214) * v1726)) * v1725)) * v456;
                            v1743 = v1741;
                            v1744 = v1742;
                        }
                        let v1745 = v44 / v1743;
                        let v1749 = v1574 - v1683;
                        let v1750 = v1575 - v1684;
                        let v1751 = v233 * v1745;
                        let v1754 = (v234 * v1745) + ((((v1744 * v1745) * v36) / v1743) * v233);
                        let v1760 = ((v1743 - v44) - v1751) + v233;
                        let v1766 = (v320 * v1749) + (v185 * v1760);
                        let v1767 = (v1750 * v320) + ((v187 * v1760) + (((v1744 - v1754) + v234) * v185));
                        let v1769 = v1750 * v1749;
                        let v1776 = v1683 - v44;
                        let v1781 = (((v1743 - v1683) - v44) + v1751) + (v233 * v1776);
                        let v1789 = v1743 + v1751;
                        let v1795 = v320 - (v185 * v1789);
                        let v1798 = v1767 * v1766;
                        let v1800 = v320 * ((v1749 * v1749) - (v185 * v1781));
                        let v1801 = ((v1769 + v1769) - ((v187 * v1781) + ((((v1744 - v1684) + v1754) + ((v234 * v1776) + (v1684 * v233))) * v185))) * v320;
                        let v1810 = ((v1766 * v1766) - (v1800 * v1795)).sqrt();
                        let v1814 = v1766 + v1810;
                        let v1816 = v1800 / v1814;
                        let v1820 = (-v1683) - v1816;
                        let v1821 = (v1684 * v36) - ((v1801 - ((v1767 + (((v1798 + v1798) - ((v1801 * v1795) + ((((v187 * v1789) + ((v1744 + v1754) * v185)) * v36) * v1800))) * (v26 / (v24 * v1810)))) * v1816)) / v1814);
                        v1721 = v1820;
                        v1722 = v1821;
                    } else {
                        let v1688 = v238 + (v183 * v400);
                        let v1689 = v44 / v1688;
                        let v1692 = (((v184 * v400) * v1689) * v36) / v1688;
                        let v1693 = v195 * v238;
                        let v1699 = (v1693 * v1689) - v44;
                        let v1700 = v1699 * v1689;
                        let v1704 = v975 * v196;
                        let v1712 = v44 + (v1700 * v975);
                        let v1717 = -(v1704 * v1712);
                        let v1718 = ((((v976 * v196) + (v199 * v975)) * v1712) + ((((((((v194 * v238) * v1689) + (v1692 * v1693)) * v1689) + (v1692 * v1699)) * v975) + (v976 * v1700)) * v1704)) * v36;
                        let v1720 = if v1717 > v1719 { 1.0 } else { 0.0 };
                        let v1850: f64;
                        let v1851: Lanes<2>;
                        if v1720 != 0.0 {
                            let v1822 = v1717.exp();
                            let v1823 = v1718 * v1822;
                            v1850 = v1822;
                            v1851 = v1823;
                        } else {
                            let v1825 = v1824 - v1717;
                            let v1826 = v1718 * v36;
                            let v1829 = v31 * (v1827 - v1717);
                            let v1835 = v44 + ((v1831 - v1717) * v214);
                            let v1840 = v44 + (v1829 * v1835);
                            let v1845 = v44 + (v1825 * v1840);
                            let v1846 = v562 / v1845;
                            let v1849 = ((((v1826 * v1840) + ((((v1826 * v31) * v1835) + ((v1826 * v214) * v1829)) * v1825)) * v1846) * v36) / v1845;
                            v1850 = v1846;
                            v1851 = v1849;
                        }
                        let v1854 = v185 * v31;
                        let v1855 = v187 * v31;
                        let v1864 = ((v975 + (v185 * v575)) - (v44 - v1850)).sqrt();
                        let v1872 = (v975 + v1854) - (v183 * v1864);
                        let v1873 = (v976 + v1855) - ((v184 * v1864) + ((((v976 + (v187 * v575)) - (v1851 * v36)) * (v26 / (v24 * v1864))) * v183));
                        let v1874 = v203 + v592;
                        let v1875 = v1874 - v1872;
                        let v1876 = v204 - v1873;
                        let v1877 = if v1875 > v16 { 1.0 } else { 0.0 };
                        let v1895: f64;
                        let v1896: Lanes<2>;
                        if v1877 != 0.0 {
                            let v1879 = v1876 * v1875;
                            let v1882 = ((v1875 * v1875) + v600).sqrt();
                            let v1890 = v1874 - (v31 * (v1875 + v1882));
                            let v1891 = v204 - ((v1876 + ((v1879 + v1879) * (v26 / (v24 * v1882)))) * v31);
                            v1895 = v1890;
                            v1896 = v1891;
                        } else {
                            let v1892 = v1872 - v1874;
                            let v1893 = v1873 - v204;
                            let v1894 = if v1892 > v16 { 1.0 } else { 0.0 };
                            let v1996: f64;
                            let v1997: Lanes<2>;
                            if v1894 != 0.0 {
                                let v1974 = v1893 * v1892;
                                let v1977 = ((v1892 * v1892) + v600).sqrt();
                                let v1981 = v1892 + v1977;
                                let v1984 = v1983 / v1981;
                                let v1988 = v1874 - v1984;
                                let v1989 = v204 - ((((v1893 + ((v1974 + v1974) * (v26 / (v24 * v1977)))) * v1984) * v36) / v1981);
                                v1996 = v1988;
                                v1997 = v1989;
                            } else {
                                let v1994 = v1874 - (v31 * (v1875 + v1990));
                                let v1995 = v204 - (v1876 * v31);
                                v1996 = v1994;
                                v1997 = v1995;
                            }
                            v1895 = v1996;
                            v1896 = v1997;
                        }
                        let v1898 = v204 * v1874;
                        let v1901 = ((v1874 * v1874) + v600).sqrt();
                        let v1909 = v1895 - (v31 * (v1874 - v1901));
                        let v1910 = v1896 - ((v204 - ((v1898 + v1898) * (v26 / (v24 * v1901)))) * v31);
                        let v1911 = v975 - v1909;
                        let v1912 = v976 - v1910;
                        let v1915 = (-v1909).exp();
                        let v1916 = (v1910 * v36) * v1915;
                        let v1918 = v1912 * v1911;
                        let v1923 = v1909 + v44;
                        let v1928 = ((v1915 + v1909) - v44) - (v233 * v1923);
                        let v1934 = (v1911 * v1911) - (v185 * v1928);
                        let v1936 = if v656 >= v1934 { v656 } else { v1934 };
                        let v1939 = ((v1918 + v1918) - ((v187 * v1928) + (((v1916 + v1910) - ((v234 * v1923) + (v1910 * v233))) * v185))) * (v26 - (if v656 >= v1934 { 1.0 } else { 0.0 }));
                        let v1944 = v44 - (v1854 * v1915);
                        let v1945 = ((v1855 * v1915) + (v1916 * v1854)) * v36;
                        let v1950 = (v44 - v1915) - v233;
                        let v1956 = (v320 * v1911) + (v185 * v1950);
                        let v1957 = (v1912 * v320) + ((v187 * v1950) + (((v1916 * v36) - v234) * v185));
                        let v1960 = v1936 / v185;
                        let v1967 = (v203 - v1909) + (v1960.ln());
                        let v1968 = (v204 - v1910) + (((v1939 - (v187 * v1960)) / v185) * (v26 / v1960));
                        let v1969 = v1936 + v1956;
                        let v1970 = v1939 + v1957;
                        let v1972 = if (v1967.abs()) < v693 { 1.0 } else { 0.0 };
                        let v2062: f64;
                        let v2063: Lanes<2>;
                        if v1972 != 0.0 {
                            v2062 = v1909;
                            v2063 = v1910;
                        } else {
                            let v1999 = v1970 * v1969;
                            let v2001 = v31 * v1956;
                            let v2007 = v1936 * v1944;
                            let v2010 = (v1939 * v1944) + (v1945 * v1936);
                            let v2011 = (v2001 * v1956) - v2007;
                            let v2017 = (v1969 * v1969) + (v2011 * v1967);
                            let v2018 = (v1999 + v1999) + ((((((v1957 * v31) * v1956) + (v1957 * v2001)) - v2010) * v1967) + (v1968 * v2011));
                            let v2019 = v1936 * v1969;
                            let v2027 = v1969 * v1967;
                            let v2035 = (v2027 * v1967) / v2017;
                            let v2039 = v2035 * v1956;
                            let v2044 = v1957 * v1956;
                            let v2048 = ((v1956 * v1956) * v214) - v2007;
                            let v2054 = v2017 + (v2039 * v2048);
                            let v2056 = (v2019 * v1967) / v2054;
                            let v2060 = v1909 + v2056;
                            let v2061 = v1910 + ((((((v1939 * v1969) + (v1970 * v1936)) * v1967) + (v1968 * v2019)) - ((v2018 + ((((((((((v1970 * v1967) + (v1968 * v1969)) * v1967) + (v1968 * v2027)) - (v2018 * v2035)) / v2017) * v1956) + (v1957 * v2035)) * v2048) + ((((v2044 + v2044) * v214) - v2010) * v2039))) * v2056)) / v2054);
                            v2062 = v2060;
                            v2063 = v2061;
                        }
                        let v2064 = if v2062 < v398 { 1.0 } else { 0.0 };
                        let v2077: f64;
                        let v2078: f64;
                        let v2079: Lanes<2>;
                        let v2080: Lanes<2>;
                        if v2064 != 0.0 {
                            let v2065 = v2062.exp();
                            let v2066 = v2063 * v2065;
                            let v2067 = v44 / v2065;
                            let v2070 = ((v2066 * v2067) * v36) / v2065;
                            let v2071 = v233 * v2065;
                            let v2074 = (v234 * v2065) + (v2066 * v233);
                            v2077 = v2067;
                            v2078 = v2071;
                            v2079 = v2070;
                            v2080 = v2074;
                        } else {
                            let v2076 = if v2062 > (v203 - v398) { 1.0 } else { 0.0 };
                            let v2199: f64;
                            let v2200: f64;
                            let v2201: Lanes<2>;
                            let v2202: Lanes<2>;
                            if v2076 != 0.0 {
                                let v2151 = (v2062 - v203).exp();
                                let v2152 = (v2063 - v204) * v2151;
                                let v2153 = v233 / v2151;
                                let v2156 = (v234 - (v2152 * v2153)) / v2151;
                                v2199 = v2153;
                                v2200 = v2151;
                                v2201 = v2156;
                                v2202 = v2152;
                            } else {
                                let v2158 = v204 - v2063;
                                let v2159 = (v203 - v2062) - v398;
                                let v2160 = v31 * v2159;
                                let v2164 = v44 + (v2159 * v214);
                                let v2169 = v44 + (v2160 * v2164);
                                let v2174 = v44 + (v2159 * v2169);
                                let v2175 = v562 / v2174;
                                let v2178 = ((((v2158 * v2169) + ((((v2158 * v31) * v2164) + ((v2158 * v214) * v2160)) * v2159)) * v2175) * v36) / v2174;
                                let v2179 = v2062 - v398;
                                let v2180 = v31 * v2179;
                                let v2184 = v44 + (v2179 * v214);
                                let v2189 = v44 + (v2180 * v2184);
                                let v2194 = v44 + (v2179 * v2189);
                                let v2195 = v562 / v2194;
                                let v2198 = ((((v2063 * v2189) + ((((v2063 * v31) * v2184) + ((v2063 * v214) * v2180)) * v2179)) * v2195) * v36) / v2194;
                                v2199 = v2195;
                                v2200 = v2175;
                                v2201 = v2198;
                                v2202 = v2178;
                            }
                            v2077 = v2199;
                            v2078 = v2200;
                            v2079 = v2201;
                            v2080 = v2202;
                        }
                        let v2081 = v975 - v2062;
                        let v2082 = v976 - v2063;
                        let v2089 = ((v44 - v2077) + v2078) - v233;
                        let v2095 = (v320 * v2081) + (v185 * v2089);
                        let v2096 = (v2082 * v320) + ((v187 * v2089) + ((((v2079 * v36) + v2080) - v234) * v185));
                        let v2098 = v2082 * v2081;
                        let v2105 = v2062 + v44;
                        let v2110 = (((v2077 + v2062) - v44) + v2078) - (v233 * v2105);
                        let v2118 = v2077 + v2078;
                        let v2124 = v320 - (v185 * v2118);
                        let v2127 = v2096 * v2095;
                        let v2129 = v320 * ((v2081 * v2081) - (v185 * v2110));
                        let v2130 = ((v2098 + v2098) - ((v187 * v2110) + ((((v2079 + v2063) + v2080) - ((v234 * v2105) + (v2063 * v233))) * v185))) * v320;
                        let v2137 = ((v2095 * v2095) - (v2129 * v2124)).sqrt();
                        let v2141 = v2095 + v2137;
                        let v2143 = v2129 / v2141;
                        let v2147 = v2062 + v2143;
                        let v2148 = v2063 + ((v2130 - ((v2096 + (((v2127 + v2127) - ((v2130 * v2124) + ((((v187 * v2118) + ((v2079 + v2080) * v185)) * v36) * v2129))) * (v26 / (v24 * v2137)))) * v2143)) / v2141);
                        v1721 = v2147;
                        v1722 = v2148;
                    }
                    v1572 = v1721;
                    v1573 = v1722;
                }
                v938 = v975;
                v939 = v1572;
                v940 = v976;
                v941 = v1573;
            } else {
                v938 = v243;
                v939 = v280;
                v940 = v244;
                v941 = v281;
            }
            let v944 = if (if v938 <= v34 { 1.0 } else { 0.0 }) != 0.0 || v943 != 0.0 { 1.0 } else { 0.0 };
            let v2205: f64;
            let v2206: Lanes<2>;
            if v944 != 0.0 {
                v2205 = v34;
                v2206 = v2203;
            } else {
                let v2204 = if v939 < v398 { 1.0 } else { 0.0 };
                let v2238: f64;
                let v2239: f64;
                let v2240: Lanes<2>;
                let v2241: Lanes<2>;
                if v2204 != 0.0 {
                    let v2219 = v939.exp();
                    let v2221 = v44 / v2219;
                    let v2224 = (((v941 * v2219) * v2221) * v36) / v2219;
                    let v2225 = v44 / v2221;
                    let v2231 = (v2225 - v939) - v44;
                    let v2232 = v233 * v2231;
                    let v2235 = (v234 * v2231) + (((((v2224 * v2225) * v36) / v2221) - v941) * v233);
                    v2238 = v2221;
                    v2239 = v2232;
                    v2240 = v2224;
                    v2241 = v2235;
                } else {
                    let v2237 = if v939 > (v203 - v398) { 1.0 } else { 0.0 };
                    let v2307: f64;
                    let v2308: f64;
                    let v2309: Lanes<2>;
                    let v2310: Lanes<2>;
                    if v2237 != 0.0 {
                        let v2245 = (v939 - v203).exp();
                        let v2246 = (v941 - v204) * v2245;
                        let v2247 = v233 / v2245;
                        let v2250 = (v234 - (v2246 * v2247)) / v2245;
                        let v2251 = v939 + v44;
                        let v2256 = v2245 - (v233 * v2251);
                        let v2257 = v2246 - ((v234 * v2251) + (v941 * v233));
                        v2307 = v2247;
                        v2308 = v2256;
                        v2309 = v2250;
                        v2310 = v2257;
                    } else {
                        let v2259 = v204 - v941;
                        let v2260 = (v203 - v939) - v398;
                        let v2261 = v31 * v2260;
                        let v2265 = v44 + (v2260 * v214);
                        let v2270 = v44 + (v2261 * v2265);
                        let v2275 = v44 + (v2260 * v2270);
                        let v2276 = v562 / v2275;
                        let v2280 = v939 - v398;
                        let v2281 = v31 * v2280;
                        let v2285 = v44 + (v2280 * v214);
                        let v2290 = v44 + (v2281 * v2285);
                        let v2295 = v44 + (v2280 * v2290);
                        let v2296 = v562 / v2295;
                        let v2299 = ((((v941 * v2290) + ((((v941 * v31) * v2285) + ((v941 * v214) * v2281)) * v2280)) * v2296) * v36) / v2295;
                        let v2300 = v939 + v44;
                        let v2305 = v2276 - (v233 * v2300);
                        let v2306 = (((((v2259 * v2270) + ((((v2259 * v31) * v2265) + ((v2259 * v214) * v2261)) * v2260)) * v2276) * v36) / v2275) - ((v234 * v2300) + (v941 * v233));
                        v2307 = v2296;
                        v2308 = v2305;
                        v2309 = v2299;
                        v2310 = v2306;
                    }
                    v2238 = v2307;
                    v2239 = v2308;
                    v2240 = v2309;
                    v2241 = v2310;
                }
                let v2242 = if v939 < v200 { 1.0 } else { 0.0 };
                let v2372: f64;
                let v2373: f64;
                let v2374: f64;
                let v2375: Lanes<2>;
                let v2376: Lanes<2>;
                let v2377: Lanes<2>;
                if v2242 != 0.0 {
                    let v2311 = v31 * v939;
                    let v2313 = v2311 * v939;
                    let v2317 = v214 * v939;
                    let v2321 = v44 - (v575 * v939);
                    let v2327 = v44 - (v2317 * v2321);
                    let v2328 = (((v941 * v214) * v2321) + (((v941 * v575) * v36) * v2317)) * v36;
                    let v2329 = v2313 * v2327;
                    let v2332 = ((((v941 * v31) * v939) + (v941 * v2311)) * v2327) + (v2328 * v2313);
                    let v2333 = v250 * v233;
                    let v2335 = v2333 * v939;
                    let v2339 = v2335 * v939;
                    let v2343 = v2339 * v939;
                    let v2350 = v44 + (v2347 * v939);
                    let v2351 = v2343 * v2350;
                    let v2354 = ((((((((v234 * v250) * v939) + (v941 * v2333)) * v939) + (v941 * v2335)) * v939) + (v941 * v2339)) * v2350) + ((v941 * v2347) * v2343);
                    let v2355 = v2327.sqrt();
                    let v2359 = v192 * v939;
                    let v2361 = v2359 * v2355;
                    let v2364 = ((v941 * v192) * v2355) + ((v2328 * (v26 / (v24 * v2355))) * v2359);
                    v2372 = v2329;
                    v2373 = v2351;
                    v2374 = v2361;
                    v2375 = v2332;
                    v2376 = v2354;
                    v2377 = v2364;
                } else {
                    let v2366 = (v939 - v44) + v2238;
                    let v2367 = v941 + v2240;
                    let v2368 = v2366.sqrt();
                    let v2371 = v2367 * (v26 / (v24 * v2368));
                    v2372 = v2366;
                    v2373 = v2239;
                    v2374 = v2368;
                    v2375 = v2367;
                    v2376 = v2241;
                    v2377 = v2371;
                }
                let v2380 = (v2372 + v2373).sqrt();
                let v2388 = v925 * v185;
                let v2398 = (v183 * v2380) + (v183 * v2374);
                let v2400 = (v2388 * v2373) / v2398;
                let v2403 = ((((v187 * v925) * v2373) + (v2376 * v2388)) - ((((v184 * v2380) + (((v2375 + v2376) * (v26 / (v24 * v2380))) * v183)) + ((v184 * v2374) + (v2377 * v183))) * v2400)) / v2398;
                v2205 = v2400;
                v2206 = v2403;
            }
            let v2207 = -v2205;
            let v2208 = v2206 * v36;
            let v2210 = v242 + v2209;
            let v2211 = Lanes([v15[0], v15[1], 0.0]);
            let v2213 = Lanes([0.0, 0.0, v2212[0]]);
            let v2214 = v2211 + v2213;
            let v2215 = v2210 * v202;
            let v2216 = v2214 * v202;
            let v2218 = if (v2215.abs()) <= v201 { 1.0 } else { 0.0 };
            let v2410: f64;
            let v2411: Lanes<3>;
            if v2218 != 0.0 {
                let v2404 = v2215 / v195;
                let v2405 = v194 * v2404;
                let v2408 = (v2216 - (Lanes([v2405[0], v2405[1], 0.0]))) / v195;
                v2410 = v2404;
                v2411 = v2408;
            } else {
                let v2409 = if v2215 > v201 { 1.0 } else { 0.0 };
                let v2556: f64;
                let v2557: Lanes<3>;
                if v2409 != 0.0 {
                    let v2416 = (v195 * v238) / v239;
                    let v2421 = (v2416 - v44) / v239;
                    let v2425 = v2215 / v195;
                    let v2426 = v194 * v2425;
                    let v2431 = (((((v194 * v238) - (v237 * v2416)) / v239) - (v237 * v2421)) / v239) * v2215;
                    let v2435 = v44 + (v2421 * v2215);
                    let v2436 = v2425 * v2435;
                    let v2439 = (((v2216 - (Lanes([v2426[0], v2426[1], 0.0]))) / v195) * v2435) + (((Lanes([v2431[0], v2431[1], 0.0])) + (v2216 * v2421)) * v2425);
                    let v2440 = if v2436 < v205 { 1.0 } else { 0.0 };
                    let v2582: f64;
                    let v2583: Lanes<3>;
                    if v2440 != 0.0 {
                        let v2560 = (-v2436).exp();
                        let v2561 = (v2439 * v36) * v2560;
                        v2582 = v2560;
                        v2583 = v2561;
                    } else {
                        let v2562 = v2436 - v205;
                        let v2563 = v31 * v2562;
                        let v2567 = v44 + (v2562 * v214);
                        let v2572 = v44 + (v2563 * v2567);
                        let v2577 = v44 + (v2562 * v2572);
                        let v2578 = v228 / v2577;
                        let v2581 = ((((v2439 * v2572) + ((((v2439 * v31) * v2567) + ((v2439 * v214) * v2563)) * v2562)) * v2578) * v36) / v2577;
                        v2582 = v2578;
                        v2583 = v2581;
                    }
                    let v2586 = v31 * v185;
                    let v2587 = v187 * v31;
                    let v2592 = v187 * v575;
                    let v2598 = ((v2215 + (v575 * v185)) - (v44 - v2582)).sqrt();
                    let v2603 = v184 * v2598;
                    let v2607 = (v2215 + v2586) - (v183 * v2598);
                    let v2608 = (v2216 + (Lanes([v2587[0], v2587[1], 0.0]))) - ((Lanes([v2603[0], v2603[1], 0.0])) + ((((v2216 + (Lanes([v2592[0], v2592[1], 0.0]))) - (v2583 * v36)) * (v26 / (v24 * v2598))) * v183));
                    let v2609 = if v2607 < v205 { 1.0 } else { 0.0 };
                    let v2634: f64;
                    let v2635: Lanes<3>;
                    if v2609 != 0.0 {
                        let v2612 = (-v2607).exp();
                        let v2613 = (v2608 * v36) * v2612;
                        v2634 = v2612;
                        v2635 = v2613;
                    } else {
                        let v2614 = v2607 - v205;
                        let v2615 = v31 * v2614;
                        let v2619 = v44 + (v2614 * v214);
                        let v2624 = v44 + (v2615 * v2619);
                        let v2629 = v44 + (v2614 * v2624);
                        let v2630 = v228 / v2629;
                        let v2633 = ((((v2608 * v2624) + ((((v2608 * v31) * v2619) + ((v2608 * v214) * v2615)) * v2614)) * v2630) * v36) / v2629;
                        v2634 = v2630;
                        v2635 = v2633;
                    }
                    let v2637 = v2587 * v2634;
                    let v2643 = v2215 - v2607;
                    let v2644 = v2216 - v2608;
                    let v2647 = v44 - v2634;
                    let v2650 = v187 * v2647;
                    let v2654 = (v320 * v2643) + (v185 * v2647);
                    let v2655 = (v2644 * v320) + ((Lanes([v2650[0], v2650[1], 0.0])) + ((v2635 * v36) * v185));
                    let v2657 = v2644 * v2643;
                    let v2660 = (v2607 - v44) + v2634;
                    let v2663 = v187 * v2660;
                    let v2667 = (v2643 * v2643) - (v185 * v2660);
                    let v2668 = (v2657 + v2657) - ((Lanes([v2663[0], v2663[1], 0.0])) + ((v2608 + v2635) * v185));
                    let v2670 = v2655 * v2654;
                    let v2673 = v2672 * (v44 - (v2586 * v2634));
                    let v2683 = ((v2654 * v2654) - (v2673 * v2667)).sqrt();
                    let v2687 = v2654 + v2683;
                    let v2689 = (v320 * v2667) / v2687;
                    let v2693 = v2607 + v2689;
                    let v2694 = v2608 + (((v2668 * v320) - ((v2655 + (((v2670 + v2670) - ((((((Lanes([v2637[0], v2637[1], 0.0])) + (v2635 * v2586)) * v36) * v2672) * v2667) + (v2668 * v2673))) * (v26 / (v24 * v2683)))) * v2689)) / v2687);
                    v2556 = v2693;
                    v2557 = v2694;
                } else {
                    let v2441 = -v2215;
                    let v2442 = v2216 * v36;
                    let v2445 = (v238 * v2441) / v195;
                    let v2446 = v194 * v2445;
                    let v2449 = ((v2442 * v238) - (Lanes([v2446[0], v2446[1], 0.0]))) / v195;
                    let v2451 = v2445 - v293;
                    let v2453 = v2449 * v2451;
                    let v2456 = ((v2451 * v2451) + v298).sqrt();
                    let v2462 = v31 * ((v2445 + v291) - v2456);
                    let v2463 = (v2449 - ((v2453 + v2453) * (v26 / (v24 * v2456)))) * v31;
                    let v2464 = v2441 - v2462;
                    let v2465 = v2442 - v2463;
                    let v2467 = v2465 * v2464;
                    let v2469 = v2462 + v44;
                    let v2471 = v187 * v2469;
                    let v2475 = (v2464 * v2464) + (v185 * v2469);
                    let v2476 = (v2467 + v2467) + ((Lanes([v2471[0], v2471[1], 0.0])) + (v2463 * v185));
                    let v2479 = (v320 * v2464) - v185;
                    let v2481 = (v2465 * v320) - (Lanes([v187[0], v187[1], 0.0]));
                    let v2482 = v2475 / v185;
                    let v2483 = v187 * v2482;
                    let v2490 = (v2482.ln()) - v2462;
                    let v2491 = (((v2476 - (Lanes([v2483[0], v2483[1], 0.0]))) / v185) * (v26 / v2482)) - v2463;
                    let v2492 = v2475 + v2479;
                    let v2493 = v2476 + v2481;
                    let v2495 = v2493 * v2492;
                    let v2497 = v31 * v2479;
                    let v2503 = (v2497 * v2479) - v2475;
                    let v2509 = (v2492 * v2492) + (v2503 * v2490);
                    let v2510 = (v2495 + v2495) + ((((((v2481 * v31) * v2479) + (v2481 * v2497)) - v2476) * v2490) + (v2491 * v2503));
                    let v2511 = v2475 * v2492;
                    let v2519 = v2492 * v2490;
                    let v2527 = (v2519 * v2490) / v2509;
                    let v2531 = v2527 * v2479;
                    let v2536 = v2481 * v2479;
                    let v2540 = ((v2479 * v2479) * v214) - v2475;
                    let v2546 = v2509 + (v2531 * v2540);
                    let v2548 = (v2511 * v2490) / v2546;
                    let v2552 = v2462 + v2548;
                    let v2553 = v2463 + ((((((v2476 * v2492) + (v2493 * v2475)) * v2490) + (v2491 * v2511)) - ((v2510 + ((((((((((v2493 * v2490) + (v2491 * v2492)) * v2490) + (v2491 * v2519)) - (v2510 * v2527)) / v2509) * v2479) + (v2481 * v2527)) * v2540) + ((((v2536 + v2536) * v214) - v2476) * v2531))) * v2548)) / v2546);
                    let v2555 = if (v2552.abs()) < v398 { 1.0 } else { 0.0 };
                    let v2699: f64;
                    let v2700: Lanes<3>;
                    if v2555 != 0.0 {
                        let v2695 = v2552.exp();
                        let v2696 = v2553 * v2695;
                        v2699 = v2695;
                        v2700 = v2696;
                    } else {
                        let v2698 = if v2552 < v2697 { 1.0 } else { 0.0 };
                        let v2806: f64;
                        let v2807: Lanes<3>;
                        if v2698 != 0.0 {
                            let v2763 = v2762 - v2552;
                            let v2764 = v2553 * v36;
                            let v2767 = v31 * (v2765 - v2552);
                            let v2773 = v44 + ((v2769 - v2552) * v214);
                            let v2778 = v44 + (v2767 * v2773);
                            let v2783 = v44 + (v2763 * v2778);
                            let v2784 = v562 / v2783;
                            let v2787 = ((((v2764 * v2778) + ((((v2764 * v31) * v2773) + ((v2764 * v214) * v2767)) * v2763)) * v2784) * v36) / v2783;
                            v2806 = v2784;
                            v2807 = v2787;
                        } else {
                            let v2788 = v2552 - v398;
                            let v2789 = v31 * v2788;
                            let v2793 = v44 + (v2788 * v214);
                            let v2798 = v44 + (v2789 * v2793);
                            let v2804 = v456 * (v44 + (v2788 * v2798));
                            let v2805 = ((v2553 * v2798) + ((((v2553 * v31) * v2793) + ((v2553 * v214) * v2789)) * v2788)) * v456;
                            v2806 = v2804;
                            v2807 = v2805;
                        }
                        v2699 = v2806;
                        v2700 = v2807;
                    }
                    let v2701 = v31 * v185;
                    let v2704 = (v187 * v31) * v2699;
                    let v2710 = v2441 - v2552;
                    let v2711 = v2442 - v2553;
                    let v2714 = v2699 - v44;
                    let v2716 = v187 * v2714;
                    let v2720 = (v320 * v2710) + (v185 * v2714);
                    let v2721 = (v2711 * v320) + ((Lanes([v2716[0], v2716[1], 0.0])) + (v2700 * v185));
                    let v2723 = v2711 * v2710;
                    let v2726 = (v2552 + v44) - v2699;
                    let v2729 = v187 * v2726;
                    let v2733 = (v2710 * v2710) + (v185 * v2726);
                    let v2734 = (v2723 + v2723) + ((Lanes([v2729[0], v2729[1], 0.0])) + ((v2553 - v2700) * v185));
                    let v2736 = v2721 * v2720;
                    let v2738 = v2672 * (v44 - (v2701 * v2699));
                    let v2748 = ((v2720 * v2720) - (v2738 * v2733)).sqrt();
                    let v2752 = v2720 + v2748;
                    let v2754 = (v320 * v2733) / v2752;
                    let v2760 = -(v2552 + v2754);
                    let v2761 = (v2553 + (((v2734 * v320) - ((v2721 + (((v2736 + v2736) - ((((((Lanes([v2704[0], v2704[1], 0.0])) + (v2700 * v2701)) * v36) * v2672) * v2733) + (v2734 * v2738))) * (v26 / (v24 * v2748)))) * v2754)) / v2752)) * v36;
                    v2556 = v2760;
                    v2557 = v2761;
                }
                v2410 = v2556;
                v2411 = v2557;
            }
            let v2412 = v2410 * v925;
            let v2413 = v2411 * v925;
            let v2818: f64;
            let v2819: f64;
            let v2820: f64;
            let v2821: Lanes<3>;
            let v2822: Lanes<3>;
            let v2823: Lanes<3>;
            if v282 != 0.0 {
                let v2813 = (v2810 * (v242 - v2412)) * v202;
                let v2814 = ((v2211 - v2413) * v2810) * v202;
                let v2816 = if (v2813.abs()) <= v936 { 1.0 } else { 0.0 };
                let v2844: f64;
                let v2845: Lanes<3>;
                if v2816 != 0.0 {
                    let v2827 = ((v945 * v945) * v250) * v192;
                    let v2828 = v2813 * v945;
                    let v2830 = v44 - v951;
                    let v2837 = v44 + (((v2813 * v2830) * v955) * v2827);
                    let v2838 = v2828 * v2837;
                    let v2841 = ((v2814 * v945) * v2837) + ((((v2814 * v2830) * v955) * v2827) * v2828);
                    v2844 = v2838;
                    v2845 = v2841;
                } else {
                    let v2843 = if v2813 < (-v936) { 1.0 } else { 0.0 };
                    let v2981: f64;
                    let v2982: Lanes<3>;
                    if v2843 != 0.0 {
                        let v2856 = -v2813;
                        let v2857 = v2814 * v36;
                        let v2860 = (v238 * v2856) * v945;
                        let v2861 = (v2857 * v238) * v945;
                        let v2863 = v2860 - v293;
                        let v2865 = v2861 * v2863;
                        let v2868 = ((v2863 * v2863) + v298).sqrt();
                        let v2874 = v31 * ((v2860 + v291) - v2868);
                        let v2875 = (v2861 - ((v2865 + v2865) * (v26 / (v24 * v2868)))) * v31;
                        let v2876 = v2856 - v2874;
                        let v2877 = v2857 - v2875;
                        let v2879 = v2877 * v2876;
                        let v2884 = (v2876 * v2876) + (v1005 * (v2874 + v44));
                        let v2885 = (v2879 + v2879) + (v2875 * v1005);
                        let v2887 = v2877 * v320;
                        let v2888 = (v320 * v2876) - v1005;
                        let v2891 = v2884 * v1015;
                        let v2896 = (-v2874) + (v2891.ln());
                        let v2897 = (v2875 * v36) + ((v2885 * v1015) * (v26 / v2891));
                        let v2898 = v2884 + v2888;
                        let v2899 = v2885 + v2887;
                        let v2901 = v2899 * v2898;
                        let v2903 = v31 * v2888;
                        let v2909 = (v2903 * v2888) - v2884;
                        let v2915 = (v2898 * v2898) + (v2909 * v2896);
                        let v2916 = (v2901 + v2901) + ((((((v2887 * v31) * v2888) + (v2887 * v2903)) - v2885) * v2896) + (v2897 * v2909));
                        let v2917 = v2884 * v2898;
                        let v2925 = v2898 * v2896;
                        let v2933 = (v2925 * v2896) / v2915;
                        let v2937 = v2933 * v2888;
                        let v2942 = v2887 * v2888;
                        let v2946 = ((v2888 * v2888) * v214) - v2884;
                        let v2952 = v2915 + (v2937 * v2946);
                        let v2954 = (v2917 * v2896) / v2952;
                        let v2958 = v2874 + v2954;
                        let v2959 = v2875 + ((((((v2885 * v2898) + (v2899 * v2884)) * v2896) + (v2897 * v2917)) - ((v2916 + ((((((((((v2899 * v2896) + (v2897 * v2898)) * v2896) + (v2897 * v2925)) - (v2916 * v2933)) / v2915) * v2888) + (v2887 * v2933)) * v2946) + ((((v2942 + v2942) * v214) - v2885) * v2937))) * v2954)) / v2952);
                        let v2960 = if v2958 < v398 { 1.0 } else { 0.0 };
                        let v3003: f64;
                        let v3004: Lanes<3>;
                        if v2960 != 0.0 {
                            let v2983 = v2958.exp();
                            let v2984 = v2959 * v2983;
                            v3003 = v2983;
                            v3004 = v2984;
                        } else {
                            let v2985 = v2958 - v398;
                            let v2986 = v31 * v2985;
                            let v2990 = v44 + (v2985 * v214);
                            let v2995 = v44 + (v2986 * v2990);
                            let v3001 = v456 * (v44 + (v2985 * v2995));
                            let v3002 = ((v2959 * v2995) + ((((v2959 * v31) * v2990) + ((v2959 * v214) * v2986)) * v2985)) * v456;
                            v3003 = v3001;
                            v3004 = v3002;
                        }
                        let v3005 = v44 / v3003;
                        let v3009 = v2856 - v2958;
                        let v3010 = v2857 - v2959;
                        let v3011 = v951 * v3005;
                        let v3012 = (((v3004 * v3005) * v36) / v3003) * v951;
                        let v3021 = (v320 * v3009) + (v1005 * (((v3003 - v44) - v3011) + v951));
                        let v3022 = (v3010 * v320) + ((v3004 - v3012) * v1005);
                        let v3024 = v3010 * v3009;
                        let v3044 = v320 - (v1005 * (v3003 + v3011));
                        let v3047 = v3022 * v3021;
                        let v3049 = v320 * ((v3009 * v3009) - (v1005 * ((((v3003 - v2958) - v44) + v3011) + (v951 * (v2958 - v44)))));
                        let v3050 = ((v3024 + v3024) - ((((v3004 - v2959) + v3012) + (v2959 * v951)) * v1005)) * v320;
                        let v3059 = ((v3021 * v3021) - (v3049 * v3044)).sqrt();
                        let v3063 = v3021 + v3059;
                        let v3065 = v3049 / v3063;
                        let v3069 = (-v2958) - v3065;
                        let v3070 = (v2959 * v36) - ((v3050 - ((v3022 + (((v3047 + v3047) - ((v3050 * v3044) + ((((v3004 + v3012) * v1005) * v36) * v3049))) * (v26 / (v24 * v3059)))) * v3065)) / v3063);
                        v2981 = v3069;
                        v2982 = v3070;
                    } else {
                        let v2963 = v44 / (v238 + (v955 * v400));
                        let v2967 = (((v1089 * v238) * v2963) - v44) * v2963;
                        let v2968 = v2813 * v945;
                        let v2972 = v44 + (v2967 * v2813);
                        let v2977 = -(v2968 * v2972);
                        let v2978 = (((v2814 * v945) * v2972) + ((v2814 * v2967) * v2968)) * v36;
                        let v2980 = if v2977 > v2979 { 1.0 } else { 0.0 };
                        let v3099: f64;
                        let v3100: Lanes<3>;
                        if v2980 != 0.0 {
                            let v3071 = v2977.exp();
                            let v3072 = v2978 * v3071;
                            v3099 = v3071;
                            v3100 = v3072;
                        } else {
                            let v3074 = v3073 - v2977;
                            let v3075 = v2978 * v36;
                            let v3078 = v31 * (v3076 - v2977);
                            let v3084 = v44 + ((v3080 - v2977) * v214);
                            let v3089 = v44 + (v3078 * v3084);
                            let v3094 = v44 + (v3074 * v3089);
                            let v3095 = v562 / v3094;
                            let v3098 = ((((v3075 * v3089) + ((((v3075 * v31) * v3084) + ((v3075 * v214) * v3078)) * v3074)) * v3095) * v36) / v3094;
                            v3099 = v3095;
                            v3100 = v3098;
                        }
                        let v3103 = v1005 * v31;
                        let v3109 = ((v2813 + (v1005 * v575)) - (v44 - v3099)).sqrt();
                        let v3115 = (v2813 + v3103) - (v955 * v3109);
                        let v3116 = v2814 - (((v2814 - (v3100 * v36)) * (v26 / (v24 * v3109))) * v955);
                        let v3117 = v1243 + v592;
                        let v3118 = v3117 - v3115;
                        let v3119 = v3116 * v36;
                        let v3120 = if v3118 > v16 { 1.0 } else { 0.0 };
                        let v3137: f64;
                        let v3138: Lanes<3>;
                        if v3120 != 0.0 {
                            let v3122 = v3119 * v3118;
                            let v3125 = ((v3118 * v3118) + v600).sqrt();
                            let v3133 = v3117 - (v31 * (v3118 + v3125));
                            let v3134 = ((v3119 + ((v3122 + v3122) * (v26 / (v24 * v3125)))) * v31) * v36;
                            v3137 = v3133;
                            v3138 = v3134;
                        } else {
                            let v3135 = v3115 - v3117;
                            let v3136 = if v3135 > v16 { 1.0 } else { 0.0 };
                            let v3218: f64;
                            let v3219: Lanes<3>;
                            if v3136 != 0.0 {
                                let v3196 = v3116 * v3135;
                                let v3199 = ((v3135 * v3135) + v600).sqrt();
                                let v3203 = v3135 + v3199;
                                let v3206 = v3205 / v3203;
                                let v3210 = v3117 - v3206;
                                let v3211 = ((((v3116 + ((v3196 + v3196) * (v26 / (v24 * v3199)))) * v3206) * v36) / v3203) * v36;
                                v3218 = v3210;
                                v3219 = v3211;
                            } else {
                                let v3216 = v3117 - (v31 * (v3118 + v3212));
                                let v3217 = (v3119 * v31) * v36;
                                v3218 = v3216;
                                v3219 = v3217;
                            }
                            v3137 = v3218;
                            v3138 = v3219;
                        }
                        let v3144 = v3137 - (v31 * (v3117 - (((v3117 * v3117) + v600).sqrt())));
                        let v3145 = v2813 - v3144;
                        let v3146 = v2814 - v3138;
                        let v3148 = v3138 * v36;
                        let v3149 = (-v3144).exp();
                        let v3150 = v3148 * v3149;
                        let v3152 = v3146 * v3145;
                        let v3164 = (v3145 * v3145) - (v1005 * (((v3149 + v3144) - v44) - (v951 * (v3144 + v44))));
                        let v3166 = if v656 >= v3164 { v656 } else { v3164 };
                        let v3169 = ((v3152 + v3152) - (((v3150 + v3138) - (v3138 * v951)) * v1005)) * (v26 - (if v656 >= v3164 { 1.0 } else { 0.0 }));
                        let v3172 = v44 - (v3103 * v3149);
                        let v3173 = (v3150 * v3103) * v36;
                        let v3181 = (v320 * v3145) + (v1005 * ((v44 - v3149) - v951));
                        let v3182 = (v3146 * v320) + ((v3150 * v36) * v1005);
                        let v3184 = v3166 / v1005;
                        let v3189 = (v1243 - v3144) + (v3184.ln());
                        let v3190 = v3148 + ((v3169 / v1005) * (v26 / v3184));
                        let v3191 = v3166 + v3181;
                        let v3192 = v3169 + v3182;
                        let v3194 = if (v3189.abs()) < v693 { 1.0 } else { 0.0 };
                        let v3284: f64;
                        let v3285: Lanes<3>;
                        if v3194 != 0.0 {
                            v3284 = v3144;
                            v3285 = v3138;
                        } else {
                            let v3221 = v3192 * v3191;
                            let v3223 = v31 * v3181;
                            let v3229 = v3166 * v3172;
                            let v3232 = (v3169 * v3172) + (v3173 * v3166);
                            let v3233 = (v3223 * v3181) - v3229;
                            let v3239 = (v3191 * v3191) + (v3233 * v3189);
                            let v3240 = (v3221 + v3221) + ((((((v3182 * v31) * v3181) + (v3182 * v3223)) - v3232) * v3189) + (v3190 * v3233));
                            let v3241 = v3166 * v3191;
                            let v3249 = v3191 * v3189;
                            let v3257 = (v3249 * v3189) / v3239;
                            let v3261 = v3257 * v3181;
                            let v3266 = v3182 * v3181;
                            let v3270 = ((v3181 * v3181) * v214) - v3229;
                            let v3276 = v3239 + (v3261 * v3270);
                            let v3278 = (v3241 * v3189) / v3276;
                            let v3282 = v3144 + v3278;
                            let v3283 = v3138 + ((((((v3169 * v3191) + (v3192 * v3166)) * v3189) + (v3190 * v3241)) - ((v3240 + ((((((((((v3192 * v3189) + (v3190 * v3191)) * v3189) + (v3190 * v3249)) - (v3240 * v3257)) / v3239) * v3181) + (v3182 * v3257)) * v3270) + ((((v3266 + v3266) * v214) - v3232) * v3261))) * v3278)) / v3276);
                            v3284 = v3282;
                            v3285 = v3283;
                        }
                        let v3286 = if v3284 < v398 { 1.0 } else { 0.0 };
                        let v3297: f64;
                        let v3298: f64;
                        let v3299: Lanes<3>;
                        let v3300: Lanes<3>;
                        if v3286 != 0.0 {
                            let v3287 = v3284.exp();
                            let v3288 = v3285 * v3287;
                            let v3289 = v44 / v3287;
                            let v3292 = ((v3288 * v3289) * v36) / v3287;
                            let v3293 = v951 * v3287;
                            let v3294 = v3288 * v951;
                            v3297 = v3289;
                            v3298 = v3293;
                            v3299 = v3292;
                            v3300 = v3294;
                        } else {
                            let v3296 = if v3284 > (v1243 - v398) { 1.0 } else { 0.0 };
                            let v3409: f64;
                            let v3410: f64;
                            let v3411: Lanes<3>;
                            let v3412: Lanes<3>;
                            if v3296 != 0.0 {
                                let v3361 = (v3284 - v1243).exp();
                                let v3362 = v3285 * v3361;
                                let v3363 = v951 / v3361;
                                let v3366 = ((v3362 * v3363) * v36) / v3361;
                                v3409 = v3363;
                                v3410 = v3361;
                                v3411 = v3366;
                                v3412 = v3362;
                            } else {
                                let v3368 = v3285 * v36;
                                let v3369 = (v1243 - v3284) - v398;
                                let v3370 = v31 * v3369;
                                let v3374 = v44 + (v3369 * v214);
                                let v3379 = v44 + (v3370 * v3374);
                                let v3384 = v44 + (v3369 * v3379);
                                let v3385 = v562 / v3384;
                                let v3388 = ((((v3368 * v3379) + ((((v3368 * v31) * v3374) + ((v3368 * v214) * v3370)) * v3369)) * v3385) * v36) / v3384;
                                let v3389 = v3284 - v398;
                                let v3390 = v31 * v3389;
                                let v3394 = v44 + (v3389 * v214);
                                let v3399 = v44 + (v3390 * v3394);
                                let v3404 = v44 + (v3389 * v3399);
                                let v3405 = v562 / v3404;
                                let v3408 = ((((v3285 * v3399) + ((((v3285 * v31) * v3394) + ((v3285 * v214) * v3390)) * v3389)) * v3405) * v36) / v3404;
                                v3409 = v3405;
                                v3410 = v3385;
                                v3411 = v3408;
                                v3412 = v3388;
                            }
                            v3297 = v3409;
                            v3298 = v3410;
                            v3299 = v3411;
                            v3300 = v3412;
                        }
                        let v3301 = v2813 - v3284;
                        let v3302 = v2814 - v3285;
                        let v3312 = (v320 * v3301) + (v1005 * (((v44 - v3297) + v3298) - v951));
                        let v3313 = (v3302 * v320) + (((v3299 * v36) + v3300) * v1005);
                        let v3315 = v3302 * v3301;
                        let v3335 = v320 - (v1005 * (v3297 + v3298));
                        let v3338 = v3313 * v3312;
                        let v3340 = v320 * ((v3301 * v3301) - (v1005 * ((((v3297 + v3284) - v44) + v3298) - (v951 * (v3284 + v44)))));
                        let v3341 = ((v3315 + v3315) - ((((v3299 + v3285) + v3300) - (v3285 * v951)) * v1005)) * v320;
                        let v3348 = ((v3312 * v3312) - (v3340 * v3335)).sqrt();
                        let v3352 = v3312 + v3348;
                        let v3354 = v3340 / v3352;
                        let v3358 = v3284 + v3354;
                        let v3359 = v3285 + ((v3341 - ((v3313 + (((v3338 + v3338) - ((v3341 * v3335) + ((((v3299 + v3300) * v1005) * v36) * v3340))) * (v26 / (v24 * v3348)))) * v3354)) / v3352);
                        v2981 = v3358;
                        v2982 = v3359;
                    }
                    v2844 = v2981;
                    v2845 = v2982;
                }
                let v2848 = (v2810 * v2844) * v925;
                let v2849 = (v2845 * v2810) * v925;
                let v2852 = (v2210 - v2848) / v925;
                let v2853 = (v2214 - v2849) / v925;
                let v2855 = if (v2852.abs()) <= v201 { 1.0 } else { 0.0 };
                let v3419: f64;
                let v3420: Lanes<3>;
                if v2855 != 0.0 {
                    let v3413 = v2852 / v195;
                    let v3414 = v194 * v3413;
                    let v3417 = (v2853 - (Lanes([v3414[0], v3414[1], 0.0]))) / v195;
                    v3419 = v3413;
                    v3420 = v3417;
                } else {
                    let v3418 = if v2852 > v201 { 1.0 } else { 0.0 };
                    let v3565: f64;
                    let v3566: Lanes<3>;
                    if v3418 != 0.0 {
                        let v3425 = (v195 * v238) / v239;
                        let v3430 = (v3425 - v44) / v239;
                        let v3434 = v2852 / v195;
                        let v3435 = v194 * v3434;
                        let v3440 = (((((v194 * v238) - (v237 * v3425)) / v239) - (v237 * v3430)) / v239) * v2852;
                        let v3444 = v44 + (v3430 * v2852);
                        let v3445 = v3434 * v3444;
                        let v3448 = (((v2853 - (Lanes([v3435[0], v3435[1], 0.0]))) / v195) * v3444) + (((Lanes([v3440[0], v3440[1], 0.0])) + (v2853 * v3430)) * v3434);
                        let v3449 = if v3445 < v205 { 1.0 } else { 0.0 };
                        let v3591: f64;
                        let v3592: Lanes<3>;
                        if v3449 != 0.0 {
                            let v3569 = (-v3445).exp();
                            let v3570 = (v3448 * v36) * v3569;
                            v3591 = v3569;
                            v3592 = v3570;
                        } else {
                            let v3571 = v3445 - v205;
                            let v3572 = v31 * v3571;
                            let v3576 = v44 + (v3571 * v214);
                            let v3581 = v44 + (v3572 * v3576);
                            let v3586 = v44 + (v3571 * v3581);
                            let v3587 = v228 / v3586;
                            let v3590 = ((((v3448 * v3581) + ((((v3448 * v31) * v3576) + ((v3448 * v214) * v3572)) * v3571)) * v3587) * v36) / v3586;
                            v3591 = v3587;
                            v3592 = v3590;
                        }
                        let v3595 = v31 * v185;
                        let v3596 = v187 * v31;
                        let v3601 = v187 * v575;
                        let v3607 = ((v2852 + (v575 * v185)) - (v44 - v3591)).sqrt();
                        let v3612 = v184 * v3607;
                        let v3616 = (v2852 + v3595) - (v183 * v3607);
                        let v3617 = (v2853 + (Lanes([v3596[0], v3596[1], 0.0]))) - ((Lanes([v3612[0], v3612[1], 0.0])) + ((((v2853 + (Lanes([v3601[0], v3601[1], 0.0]))) - (v3592 * v36)) * (v26 / (v24 * v3607))) * v183));
                        let v3618 = if v3616 < v205 { 1.0 } else { 0.0 };
                        let v3643: f64;
                        let v3644: Lanes<3>;
                        if v3618 != 0.0 {
                            let v3621 = (-v3616).exp();
                            let v3622 = (v3617 * v36) * v3621;
                            v3643 = v3621;
                            v3644 = v3622;
                        } else {
                            let v3623 = v3616 - v205;
                            let v3624 = v31 * v3623;
                            let v3628 = v44 + (v3623 * v214);
                            let v3633 = v44 + (v3624 * v3628);
                            let v3638 = v44 + (v3623 * v3633);
                            let v3639 = v228 / v3638;
                            let v3642 = ((((v3617 * v3633) + ((((v3617 * v31) * v3628) + ((v3617 * v214) * v3624)) * v3623)) * v3639) * v36) / v3638;
                            v3643 = v3639;
                            v3644 = v3642;
                        }
                        let v3646 = v3596 * v3643;
                        let v3652 = v2852 - v3616;
                        let v3653 = v2853 - v3617;
                        let v3656 = v44 - v3643;
                        let v3659 = v187 * v3656;
                        let v3663 = (v320 * v3652) + (v185 * v3656);
                        let v3664 = (v3653 * v320) + ((Lanes([v3659[0], v3659[1], 0.0])) + ((v3644 * v36) * v185));
                        let v3666 = v3653 * v3652;
                        let v3669 = (v3616 - v44) + v3643;
                        let v3672 = v187 * v3669;
                        let v3676 = (v3652 * v3652) - (v185 * v3669);
                        let v3677 = (v3666 + v3666) - ((Lanes([v3672[0], v3672[1], 0.0])) + ((v3617 + v3644) * v185));
                        let v3679 = v3664 * v3663;
                        let v3681 = v2672 * (v44 - (v3595 * v3643));
                        let v3691 = ((v3663 * v3663) - (v3681 * v3676)).sqrt();
                        let v3695 = v3663 + v3691;
                        let v3697 = (v320 * v3676) / v3695;
                        let v3701 = v3616 + v3697;
                        let v3702 = v3617 + (((v3677 * v320) - ((v3664 + (((v3679 + v3679) - ((((((Lanes([v3646[0], v3646[1], 0.0])) + (v3644 * v3595)) * v36) * v2672) * v3676) + (v3677 * v3681))) * (v26 / (v24 * v3691)))) * v3697)) / v3695);
                        v3565 = v3701;
                        v3566 = v3702;
                    } else {
                        let v3450 = -v2852;
                        let v3451 = v2853 * v36;
                        let v3454 = (v238 * v3450) / v195;
                        let v3455 = v194 * v3454;
                        let v3458 = ((v3451 * v238) - (Lanes([v3455[0], v3455[1], 0.0]))) / v195;
                        let v3460 = v3454 - v293;
                        let v3462 = v3458 * v3460;
                        let v3465 = ((v3460 * v3460) + v298).sqrt();
                        let v3471 = v31 * ((v3454 + v291) - v3465);
                        let v3472 = (v3458 - ((v3462 + v3462) * (v26 / (v24 * v3465)))) * v31;
                        let v3473 = v3450 - v3471;
                        let v3474 = v3451 - v3472;
                        let v3476 = v3474 * v3473;
                        let v3478 = v3471 + v44;
                        let v3480 = v187 * v3478;
                        let v3484 = (v3473 * v3473) + (v185 * v3478);
                        let v3485 = (v3476 + v3476) + ((Lanes([v3480[0], v3480[1], 0.0])) + (v3472 * v185));
                        let v3488 = (v320 * v3473) - v185;
                        let v3490 = (v3474 * v320) - (Lanes([v187[0], v187[1], 0.0]));
                        let v3491 = v3484 / v185;
                        let v3492 = v187 * v3491;
                        let v3499 = (v3491.ln()) - v3471;
                        let v3500 = (((v3485 - (Lanes([v3492[0], v3492[1], 0.0]))) / v185) * (v26 / v3491)) - v3472;
                        let v3501 = v3484 + v3488;
                        let v3502 = v3485 + v3490;
                        let v3504 = v3502 * v3501;
                        let v3506 = v31 * v3488;
                        let v3512 = (v3506 * v3488) - v3484;
                        let v3518 = (v3501 * v3501) + (v3512 * v3499);
                        let v3519 = (v3504 + v3504) + ((((((v3490 * v31) * v3488) + (v3490 * v3506)) - v3485) * v3499) + (v3500 * v3512));
                        let v3520 = v3484 * v3501;
                        let v3528 = v3501 * v3499;
                        let v3536 = (v3528 * v3499) / v3518;
                        let v3540 = v3536 * v3488;
                        let v3545 = v3490 * v3488;
                        let v3549 = ((v3488 * v3488) * v214) - v3484;
                        let v3555 = v3518 + (v3540 * v3549);
                        let v3557 = (v3520 * v3499) / v3555;
                        let v3561 = v3471 + v3557;
                        let v3562 = v3472 + ((((((v3485 * v3501) + (v3502 * v3484)) * v3499) + (v3500 * v3520)) - ((v3519 + ((((((((((v3502 * v3499) + (v3500 * v3501)) * v3499) + (v3500 * v3528)) - (v3519 * v3536)) / v3518) * v3488) + (v3490 * v3536)) * v3549) + ((((v3545 + v3545) * v214) - v3485) * v3540))) * v3557)) / v3555);
                        let v3564 = if (v3561.abs()) < v398 { 1.0 } else { 0.0 };
                        let v3707: f64;
                        let v3708: Lanes<3>;
                        if v3564 != 0.0 {
                            let v3703 = v3561.exp();
                            let v3704 = v3562 * v3703;
                            v3707 = v3703;
                            v3708 = v3704;
                        } else {
                            let v3706 = if v3561 < v3705 { 1.0 } else { 0.0 };
                            let v3814: f64;
                            let v3815: Lanes<3>;
                            if v3706 != 0.0 {
                                let v3771 = v3770 - v3561;
                                let v3772 = v3562 * v36;
                                let v3775 = v31 * (v3773 - v3561);
                                let v3781 = v44 + ((v3777 - v3561) * v214);
                                let v3786 = v44 + (v3775 * v3781);
                                let v3791 = v44 + (v3771 * v3786);
                                let v3792 = v562 / v3791;
                                let v3795 = ((((v3772 * v3786) + ((((v3772 * v31) * v3781) + ((v3772 * v214) * v3775)) * v3771)) * v3792) * v36) / v3791;
                                v3814 = v3792;
                                v3815 = v3795;
                            } else {
                                let v3796 = v3561 - v398;
                                let v3797 = v31 * v3796;
                                let v3801 = v44 + (v3796 * v214);
                                let v3806 = v44 + (v3797 * v3801);
                                let v3812 = v456 * (v44 + (v3796 * v3806));
                                let v3813 = ((v3562 * v3806) + ((((v3562 * v31) * v3801) + ((v3562 * v214) * v3797)) * v3796)) * v456;
                                v3814 = v3812;
                                v3815 = v3813;
                            }
                            v3707 = v3814;
                            v3708 = v3815;
                        }
                        let v3709 = v31 * v185;
                        let v3712 = (v187 * v31) * v3707;
                        let v3718 = v3450 - v3561;
                        let v3719 = v3451 - v3562;
                        let v3722 = v3707 - v44;
                        let v3724 = v187 * v3722;
                        let v3728 = (v320 * v3718) + (v185 * v3722);
                        let v3729 = (v3719 * v320) + ((Lanes([v3724[0], v3724[1], 0.0])) + (v3708 * v185));
                        let v3731 = v3719 * v3718;
                        let v3734 = (v3561 + v44) - v3707;
                        let v3737 = v187 * v3734;
                        let v3741 = (v3718 * v3718) + (v185 * v3734);
                        let v3742 = (v3731 + v3731) + ((Lanes([v3737[0], v3737[1], 0.0])) + ((v3562 - v3708) * v185));
                        let v3744 = v3729 * v3728;
                        let v3746 = v2672 * (v44 - (v3709 * v3707));
                        let v3756 = ((v3728 * v3728) - (v3746 * v3741)).sqrt();
                        let v3760 = v3728 + v3756;
                        let v3762 = (v320 * v3741) / v3760;
                        let v3768 = -(v3561 + v3762);
                        let v3769 = (v3562 + (((v3742 * v320) - ((v3729 + (((v3744 + v3744) - ((((((Lanes([v3712[0], v3712[1], 0.0])) + (v3708 * v3709)) * v36) * v2672) * v3741) + (v3742 * v3746))) * (v26 / (v24 * v3756)))) * v3762)) / v3760)) * v36;
                        v3565 = v3768;
                        v3566 = v3769;
                    }
                    v3419 = v3565;
                    v3420 = v3566;
                }
                let v3421 = v3419 * v925;
                let v3422 = v3420 * v925;
                v2818 = v3419;
                v2819 = v3421;
                v2820 = v2848;
                v2821 = v3420;
                v2822 = v3422;
                v2823 = v2849;
            } else {
                v2818 = v2410;
                v2819 = v2412;
                v2820 = v34;
                v2821 = v2411;
                v2822 = v2413;
                v2823 = v2817;
            }
            let v2824 = if v2818 < v398 { 1.0 } else { 0.0 };
            let v3824: f64;
            let v3825: Lanes<3>;
            if v2824 != 0.0 {
                let v3816 = v2818.exp();
                let v3818 = v44 / v3816;
                let v3821 = (((v2821 * v3816) * v3818) * v36) / v3816;
                v3824 = v3818;
                v3825 = v3821;
            } else {
                let v3823 = if v2818 > (v203 - v398) { 1.0 } else { 0.0 };
                let v3858: f64;
                let v3859: Lanes<3>;
                if v3823 != 0.0 {
                    let v3831 = (v203 - v2818).exp();
                    let v3833 = v233 * v3831;
                    let v3834 = v234 * v3831;
                    let v3837 = (Lanes([v3834[0], v3834[1], 0.0])) + ((((Lanes([v204[0], v204[1], 0.0])) - v2821) * v3831) * v233);
                    v3858 = v3833;
                    v3859 = v3837;
                } else {
                    let v3838 = v2818 - v398;
                    let v3839 = v31 * v3838;
                    let v3843 = v44 + (v3838 * v214);
                    let v3848 = v44 + (v3839 * v3843);
                    let v3853 = v44 + (v3838 * v3848);
                    let v3854 = v562 / v3853;
                    let v3857 = ((((v2821 * v3848) + ((((v2821 * v31) * v3843) + ((v2821 * v214) * v3839)) * v3838)) * v3854) * v36) / v3853;
                    v3858 = v3854;
                    v3859 = v3857;
                }
                v3824 = v3858;
                v3825 = v3859;
            }
            let v3827 = if v2818 < (-v201) { 1.0 } else { 0.0 };
            let v3871: f64;
            let v3872: Lanes<3>;
            if v3827 != 0.0 {
                let v3863 = ((v3824 + v2818) - v44).sqrt();
                let v3867 = -v3863;
                let v3868 = ((v3825 + v2821) * (v26 / (v24 * v3863))) * v36;
                v3871 = v3867;
                v3872 = v3868;
            } else {
                let v3870 = if (v2818.abs()) <= v201 { 1.0 } else { 0.0 };
                let v3937: f64;
                let v3938: Lanes<3>;
                if v3870 != 0.0 {
                    let v3908 = v214 * v2818;
                    let v3912 = v44 - (v575 * v2818);
                    let v3920 = v192 * v2818;
                    let v3922 = (v44 - (v3908 * v3912)).sqrt();
                    let v3926 = v3920 * v3922;
                    let v3929 = ((v2821 * v192) * v3922) + ((((((v2821 * v214) * v3912) + (((v2821 * v575) * v36) * v3908)) * v36) * (v26 / (v24 * v3922))) * v3920);
                    v3937 = v3926;
                    v3938 = v3929;
                } else {
                    let v3933 = ((v2818 - v44) + v3824).sqrt();
                    let v3936 = (v2821 + v3825) * (v26 / (v24 * v3933));
                    v3937 = v3933;
                    v3938 = v3936;
                }
                v3871 = v3937;
                v3872 = v3938;
            }
            let v3873 = v925 * v3871;
            let v3875 = v3873 * v183;
            let v3877 = v184 * v3873;
            let v3879 = ((v3872 * v925) * v183) + (Lanes([v3877[0], v3877[1], 0.0]));
            let v3880 = v44 + v96;
            let v3882 = v3881 * v3880;
            let v3901 = ((((((v3882 * v3880) * v3888) * v3888) * v3893) * v3896) * v925) * v925;
            let v3902 = ((((((((v97 * v3881) * v3880) + (v97 * v3882)) * v3888) * v3888) * v3893) * v3896) * v925) * v925;
            let v3903 = -v3875;
            let v3904 = v3879 * v36;
            let v3905 = v3875 - v3903;
            let v3906 = v3879 - v3904;
            let v3907 = if v3905 > v16 { 1.0 } else { 0.0 };
            let v3958: f64;
            let v3959: Lanes<3>;
            if v3907 != 0.0 {
                let v3940 = v3906 * v3905;
                let v3945 = ((v3905 * v3905) + v3901).sqrt();
                let v3953 = v3903 + (v31 * (v3905 + v3945));
                let v3954 = v3904 + ((v3906 + (((v3940 + v3940) + (Lanes([v3902[0], v3902[1], 0.0]))) * (v26 / (v24 * v3945)))) * v31);
                v3958 = v3953;
                v3959 = v3954;
            } else {
                let v3955 = v3903 - v3875;
                let v3956 = v3904 - v3879;
                let v3957 = if v3955 > v16 { 1.0 } else { 0.0 };
                let v3998: f64;
                let v3999: Lanes<3>;
                if v3957 != 0.0 {
                    let v3966 = v3902 * v31;
                    let v3968 = v3956 * v3955;
                    let v3973 = ((v3955 * v3955) + v3901).sqrt();
                    let v3977 = v3955 + v3973;
                    let v3979 = (v31 * v3901) / v3977;
                    let v3984 = v3903 + v3979;
                    let v3985 = v3904 + (((Lanes([v3966[0], v3966[1], 0.0])) - ((v3956 + (((v3968 + v3968) + (Lanes([v3902[0], v3902[1], 0.0]))) * (v26 / (v24 * v3973)))) * v3979)) / v3977);
                    v3998 = v3984;
                    v3999 = v3985;
                } else {
                    let v3987 = (v65 + v3901).sqrt();
                    let v3990 = v3902 * (v26 / (v24 * v3987));
                    let v3996 = v3903 + (v31 * (v3905 + v3987));
                    let v3997 = v3904 + ((v3906 + (Lanes([v3990[0], v3990[1], 0.0]))) * v31);
                    v3998 = v3996;
                    v3999 = v3997;
                }
                v3958 = v3998;
                v3959 = v3999;
            }
            let v3960 = -v2209;
            let v3961 = v2212 * v36;
            let v3962 = v3960 - v2209;
            let v3963 = v3961 - v2212;
            let v3964 = if v3962 > v16 { 1.0 } else { 0.0 };
            let v4021: f64;
            let v4022: Lanes<3>;
            if v3964 != 0.0 {
                let v4001 = v3963 * v3962;
                let v4002 = v4001 + v4001;
                let v4007 = ((v3962 * v3962) + v3901).sqrt();
                let v4016 = v2209 + (v31 * (v3962 + v4007));
                let v4017 = v2213 + (((Lanes([0.0, 0.0, v3963[0]])) + (((Lanes([0.0, 0.0, v4002[0]])) + (Lanes([v3902[0], v3902[1], 0.0]))) * (v26 / (v24 * v4007)))) * v31);
                v4021 = v4016;
                v4022 = v4017;
            } else {
                let v4018 = v2209 - v3960;
                let v4019 = v2212 - v3961;
                let v4020 = if v4018 > v16 { 1.0 } else { 0.0 };
                let v4065: f64;
                let v4066: Lanes<3>;
                if v4020 != 0.0 {
                    let v4030 = v3902 * v31;
                    let v4032 = v4019 * v4018;
                    let v4033 = v4032 + v4032;
                    let v4038 = ((v4018 * v4018) + v3901).sqrt();
                    let v4042 = v4018 + v4038;
                    let v4045 = (v31 * v3901) / v4042;
                    let v4050 = v2209 + v4045;
                    let v4051 = v2213 + (((Lanes([v4030[0], v4030[1], 0.0])) - (((Lanes([0.0, 0.0, v4019[0]])) + (((Lanes([0.0, 0.0, v4033[0]])) + (Lanes([v3902[0], v3902[1], 0.0]))) * (v26 / (v24 * v4038)))) * v4045)) / v4042);
                    v4065 = v4050;
                    v4066 = v4051;
                } else {
                    let v4053 = (v65 + v3901).sqrt();
                    let v4056 = v3902 * (v26 / (v24 * v4053));
                    let v4063 = v2209 + (v31 * (v3962 + v4053));
                    let v4064 = v2213 + (((Lanes([0.0, 0.0, v3963[0]])) + (Lanes([v4056[0], v4056[1], 0.0]))) * v31);
                    v4065 = v4063;
                    v4066 = v4064;
                }
                v4021 = v4065;
                v4022 = v4066;
            }
            let v4026 = v3958 + (v4023 * v4021);
            let v4027 = v3959 + (v4022 * v4023);
            let v4086: f64;
            let v4087: Lanes<3>;
            if v4028 != 0.0 {
                let v4068 = v4027 * v4026;
                let v4071 = (v4026 * v4026) + v4070;
                let v4081 = v44 + (v4078 * (v4071.powf(v4072)));
                let v4082 = v116 / v4081;
                let v4085 = (((((v4068 + v4068) * (v4072 * (v4071.powf(v4074)))) * v4078) * v4082) * v36) / v4081;
                v4086 = v4082;
                v4087 = v4085;
            } else {
                v4086 = v116;
                v4087 = v2817;
            }
            let v4088 = v291 - v939;
            let v4089 = v941 * v36;
            let v4090 = if v4088 > v16 { 1.0 } else { 0.0 };
            let v4108: f64;
            let v4109: Lanes<2>;
            if v4090 != 0.0 {
                let v4092 = v4089 * v4088;
                let v4096 = ((v4088 * v4088) + v4094).sqrt();
                let v4104 = v291 - (v31 * (v4088 + v4096));
                let v4105 = ((v4089 + ((v4092 + v4092) * (v26 / (v24 * v4096)))) * v31) * v36;
                v4108 = v4104;
                v4109 = v4105;
            } else {
                let v4106 = v939 - v291;
                let v4107 = if v4106 > v16 { 1.0 } else { 0.0 };
                let v4181: f64;
                let v4182: Lanes<2>;
                if v4107 != 0.0 {
                    let v4159 = v941 * v4106;
                    let v4162 = ((v4106 * v4106) + v4094).sqrt();
                    let v4166 = v4106 + v4162;
                    let v4169 = v4168 / v4166;
                    let v4173 = v291 - v4169;
                    let v4174 = ((((v941 + ((v4159 + v4159) * (v26 / (v24 * v4162)))) * v4169) * v36) / v4166) * v36;
                    v4181 = v4173;
                    v4182 = v4174;
                } else {
                    let v4179 = v291 - (v31 * (v4088 + v4175));
                    let v4180 = (v4089 * v31) * v36;
                    v4181 = v4179;
                    v4182 = v4180;
                }
                v4108 = v4181;
                v4109 = v4182;
            }
            let v4113 = (v4110 * v4108).exp();
            let v4117 = (v925 * v4113).sqrt();
            let v4121 = v178 * v4086;
            let v4122 = v180 * v4086;
            let v4128 = ((((v4109 * v4110) * v4113) * v925) * (v26 / (v24 * v4117))) * v4121;
            let v4134 = v15 * v242;
            let v4138 = ((v242 * v242) + v4136).sqrt();
            let v4152 = v44 + (v4149 * (v31 * ((-v242) + v4138)));
            let v4153 = (v4146 * (v4121 * v4117)) / v4152;
            let v4154 = ((((v15 * v36) + ((v4134 + v4134) * (v26 / (v24 * v4138)))) * v31) * v4149) * v4153;
            let v4157 = ((((((Lanes([v4122[0], v4122[1], 0.0])) + (v4087 * v178)) * v4117) + (Lanes([v4128[0], v4128[1], 0.0]))) * v4146) - (Lanes([v4154[0], v4154[1], 0.0]))) / v4152;
            let v4184 = v3 - v4183;
            let v4192 = ((Lanes([0.0, v6[0]])) - (Lanes([v4186[0], 0.0]))) * v13;
            let v4193 = (v13 * (v4184 - v4189)) * v202;
            let v4194 = v4192 * v202;
            let v4200: f64;
            let v4201: f64;
            let v4202: Lanes<2>;
            let v4203: Lanes<2>;
            if v4195 != 0.0 {
                let v4198 = if (v4193.abs()) <= v4197 { 1.0 } else { 0.0 };
                let v4208: f64;
                let v4209: Lanes<2>;
                if v4198 != 0.0 {
                    let v4205 = v4193 / v4204;
                    let v4206 = v4194 / v4204;
                    v4208 = v4205;
                    v4209 = v4206;
                } else {
                    let v4207 = if v4193 > v4197 { 1.0 } else { 0.0 };
                    let v4334: f64;
                    let v4335: Lanes<2>;
                    if v4207 != 0.0 {
                        let v4218 = (((v4204 * v238) / v4215) - v44) / v4215;
                        let v4219 = v4193 / v4204;
                        let v4223 = v44 + (v4218 * v4193);
                        let v4224 = v4219 * v4223;
                        let v4227 = ((v4194 / v4204) * v4223) + ((v4194 * v4218) * v4219);
                        let v4228 = if v4224 < v205 { 1.0 } else { 0.0 };
                        let v4360: f64;
                        let v4361: Lanes<2>;
                        if v4228 != 0.0 {
                            let v4338 = (-v4224).exp();
                            let v4339 = (v4227 * v36) * v4338;
                            v4360 = v4338;
                            v4361 = v4339;
                        } else {
                            let v4340 = v4224 - v205;
                            let v4341 = v31 * v4340;
                            let v4345 = v44 + (v4340 * v214);
                            let v4350 = v44 + (v4341 * v4345);
                            let v4355 = v44 + (v4340 * v4350);
                            let v4356 = v228 / v4355;
                            let v4359 = ((((v4227 * v4350) + ((((v4227 * v31) * v4345) + ((v4227 * v214) * v4341)) * v4340)) * v4356) * v36) / v4355;
                            v4360 = v4356;
                            v4361 = v4359;
                        }
                        let v4364 = v31 * v4255;
                        let v4370 = ((v4193 + (v575 * v4255)) - (v44 - v4360)).sqrt();
                        let v4377 = (v4193 + v4364) - (v4374 * v4370);
                        let v4378 = v4194 - (((v4194 - (v4361 * v36)) * (v26 / (v24 * v4370))) * v4374);
                        let v4379 = if v4377 < v205 { 1.0 } else { 0.0 };
                        let v4404: f64;
                        let v4405: Lanes<2>;
                        if v4379 != 0.0 {
                            let v4382 = (-v4377).exp();
                            let v4383 = (v4378 * v36) * v4382;
                            v4404 = v4382;
                            v4405 = v4383;
                        } else {
                            let v4384 = v4377 - v205;
                            let v4385 = v31 * v4384;
                            let v4389 = v44 + (v4384 * v214);
                            let v4394 = v44 + (v4385 * v4389);
                            let v4399 = v44 + (v4384 * v4394);
                            let v4400 = v228 / v4399;
                            let v4403 = ((((v4378 * v4394) + ((((v4378 * v31) * v4389) + ((v4378 * v214) * v4385)) * v4384)) * v4400) * v36) / v4399;
                            v4404 = v4400;
                            v4405 = v4403;
                        }
                        let v4410 = v4193 - v4377;
                        let v4411 = v4194 - v4378;
                        let v4418 = (v320 * v4410) + (v4255 * (v44 - v4404));
                        let v4419 = (v4411 * v320) + ((v4405 * v36) * v4255);
                        let v4421 = v4411 * v4410;
                        let v4428 = (v4410 * v4410) - (v4255 * ((v4377 - v44) + v4404));
                        let v4429 = (v4421 + v4421) - ((v4378 + v4405) * v4255);
                        let v4431 = v4419 * v4418;
                        let v4433 = v2672 * (v44 - (v4364 * v4404));
                        let v4443 = ((v4418 * v4418) - (v4433 * v4428)).sqrt();
                        let v4447 = v4418 + v4443;
                        let v4449 = (v320 * v4428) / v4447;
                        let v4453 = v4377 + v4449;
                        let v4454 = v4378 + (((v4429 * v320) - ((v4419 + (((v4431 + v4431) - (((((v4405 * v4364) * v36) * v2672) * v4428) + (v4429 * v4433))) * (v26 / (v24 * v4443)))) * v4449)) / v4447);
                        v4334 = v4453;
                        v4335 = v4454;
                    } else {
                        let v4229 = -v4193;
                        let v4230 = v4194 * v36;
                        let v4233 = (v238 * v4229) / v4204;
                        let v4234 = (v4230 * v238) / v4204;
                        let v4236 = v4233 - v293;
                        let v4238 = v4234 * v4236;
                        let v4241 = ((v4236 * v4236) + v298).sqrt();
                        let v4247 = v31 * ((v4233 + v291) - v4241);
                        let v4248 = (v4234 - ((v4238 + v4238) * (v26 / (v24 * v4241)))) * v31;
                        let v4249 = v4229 - v4247;
                        let v4250 = v4230 - v4248;
                        let v4252 = v4250 * v4249;
                        let v4258 = (v4249 * v4249) + (v4255 * (v4247 + v44));
                        let v4259 = (v4252 + v4252) + (v4248 * v4255);
                        let v4261 = v4250 * v320;
                        let v4262 = (v320 * v4249) - v4255;
                        let v4263 = v4258 / v4255;
                        let v4268 = (v4263.ln()) - v4247;
                        let v4269 = ((v4259 / v4255) * (v26 / v4263)) - v4248;
                        let v4270 = v4258 + v4262;
                        let v4271 = v4259 + v4261;
                        let v4273 = v4271 * v4270;
                        let v4275 = v31 * v4262;
                        let v4281 = (v4275 * v4262) - v4258;
                        let v4287 = (v4270 * v4270) + (v4281 * v4268);
                        let v4288 = (v4273 + v4273) + ((((((v4261 * v31) * v4262) + (v4261 * v4275)) - v4259) * v4268) + (v4269 * v4281));
                        let v4289 = v4258 * v4270;
                        let v4297 = v4270 * v4268;
                        let v4305 = (v4297 * v4268) / v4287;
                        let v4309 = v4305 * v4262;
                        let v4314 = v4261 * v4262;
                        let v4318 = ((v4262 * v4262) * v214) - v4258;
                        let v4324 = v4287 + (v4309 * v4318);
                        let v4326 = (v4289 * v4268) / v4324;
                        let v4330 = v4247 + v4326;
                        let v4331 = v4248 + ((((((v4259 * v4270) + (v4271 * v4258)) * v4268) + (v4269 * v4289)) - ((v4288 + ((((((((((v4271 * v4268) + (v4269 * v4270)) * v4268) + (v4269 * v4297)) - (v4288 * v4305)) / v4287) * v4262) + (v4261 * v4305)) * v4318) + ((((v4314 + v4314) * v214) - v4259) * v4309))) * v4326)) / v4324);
                        let v4333 = if (v4330.abs()) < v398 { 1.0 } else { 0.0 };
                        let v4459: f64;
                        let v4460: Lanes<2>;
                        if v4333 != 0.0 {
                            let v4455 = v4330.exp();
                            let v4456 = v4331 * v4455;
                            v4459 = v4455;
                            v4460 = v4456;
                        } else {
                            let v4458 = if v4330 < v4457 { 1.0 } else { 0.0 };
                            let v4556: f64;
                            let v4557: Lanes<2>;
                            if v4458 != 0.0 {
                                let v4513 = v4512 - v4330;
                                let v4514 = v4331 * v36;
                                let v4517 = v31 * (v4515 - v4330);
                                let v4523 = v44 + ((v4519 - v4330) * v214);
                                let v4528 = v44 + (v4517 * v4523);
                                let v4533 = v44 + (v4513 * v4528);
                                let v4534 = v562 / v4533;
                                let v4537 = ((((v4514 * v4528) + ((((v4514 * v31) * v4523) + ((v4514 * v214) * v4517)) * v4513)) * v4534) * v36) / v4533;
                                v4556 = v4534;
                                v4557 = v4537;
                            } else {
                                let v4538 = v4330 - v398;
                                let v4539 = v31 * v4538;
                                let v4543 = v44 + (v4538 * v214);
                                let v4548 = v44 + (v4539 * v4543);
                                let v4554 = v456 * (v44 + (v4538 * v4548));
                                let v4555 = ((v4331 * v4548) + ((((v4331 * v31) * v4543) + ((v4331 * v214) * v4539)) * v4538)) * v456;
                                v4556 = v4554;
                                v4557 = v4555;
                            }
                            v4459 = v4556;
                            v4460 = v4557;
                        }
                        let v4461 = v31 * v4255;
                        let v4466 = v4229 - v4330;
                        let v4467 = v4230 - v4331;
                        let v4473 = (v320 * v4466) + (v4255 * (v4459 - v44));
                        let v4474 = (v4467 * v320) + (v4460 * v4255);
                        let v4476 = v4467 * v4466;
                        let v4483 = (v4466 * v4466) + (v4255 * ((v4330 + v44) - v4459));
                        let v4484 = (v4476 + v4476) + ((v4331 - v4460) * v4255);
                        let v4486 = v4474 * v4473;
                        let v4488 = v2672 * (v44 - (v4461 * v4459));
                        let v4498 = ((v4473 * v4473) - (v4488 * v4483)).sqrt();
                        let v4502 = v4473 + v4498;
                        let v4504 = (v320 * v4483) / v4502;
                        let v4510 = -(v4330 + v4504);
                        let v4511 = (v4331 + (((v4484 * v320) - ((v4474 + (((v4486 + v4486) - (((((v4460 * v4461) * v36) * v2672) * v4483) + (v4484 * v4488))) * (v26 / (v24 * v4498)))) * v4504)) / v4502)) * v36;
                        v4334 = v4510;
                        v4335 = v4511;
                    }
                    v4208 = v4334;
                    v4209 = v4335;
                }
                let v4212 = v925 * (v4193 - v4208);
                let v4213 = (v4194 - v4209) * v925;
                v4200 = v4212;
                v4201 = v4208;
                v4202 = v4213;
                v4203 = v4209;
            } else {
                v4200 = v34;
                v4201 = v34;
                v4202 = v4199;
                v4203 = v4199;
            }
            let v4559: f64;
            let v4560: f64;
            let v4561: Lanes<3>;
            let v4562: Lanes<2>;
            if v2 != 0.0 {
                let v4593: f64;
                let v4594: Lanes<2>;
                if v4558 != 0.0 {
                    let v4591 = v13 * v4184;
                    let v4603: f64;
                    let v4604: Lanes<2>;
                    if v4592 != 0.0 {
                        let v4597 = v4202 * v13;
                        let v4599 = (v13 * v4200) + v4598;
                        let v4600 = v34 - v4599;
                        let v4601 = v4597 * v36;
                        let v4602 = if v4600 > v16 { 1.0 } else { 0.0 };
                        let v4621: f64;
                        let v4622: Lanes<2>;
                        if v4602 != 0.0 {
                            let v4607 = v4601 * v4600;
                            let v4610 = ((v4600 * v4600) + v4094).sqrt();
                            let v4618 = v4599 + (v31 * (v4600 + v4610));
                            let v4619 = v4597 + ((v4601 + ((v4607 + v4607) * (v26 / (v24 * v4610)))) * v31);
                            v4621 = v4618;
                            v4622 = v4619;
                        } else {
                            let v4620 = if v4599 > v16 { 1.0 } else { 0.0 };
                            let v4658: f64;
                            let v4659: Lanes<2>;
                            if v4620 != 0.0 {
                                let v4636 = v4597 * v4599;
                                let v4639 = ((v4599 * v4599) + v4094).sqrt();
                                let v4643 = v4599 + v4639;
                                let v4646 = v4645 / v4643;
                                let v4650 = v4599 + v4646;
                                let v4651 = v4597 + ((((v4597 + ((v4636 + v4636) * (v26 / (v24 * v4639)))) * v4646) * v36) / v4643);
                                v4658 = v4650;
                                v4659 = v4651;
                            } else {
                                let v4656 = v4599 + (v31 * (v4600 + v4652));
                                let v4657 = v4597 + (v4601 * v31);
                                v4658 = v4656;
                                v4659 = v4657;
                            }
                            v4621 = v4658;
                            v4622 = v4659;
                        }
                        let v4624 = v4202 * v4200;
                        let v4627 = ((v4200 * v4200) + v76).sqrt();
                        let v4632 = v4627 * v4631;
                        let v4633 = ((v4624 + v4624) * (v26 / (v24 * v4627))) * v4631;
                        let v4664: f64;
                        let v4665: Lanes<2>;
                        if v4634 != 0.0 {
                            let v4661 = v4660 - v4632;
                            let v4662 = v4633 * v36;
                            let v4663 = if v4661 > v16 { 1.0 } else { 0.0 };
                            let v4683: f64;
                            let v4684: Lanes<2>;
                            if v4663 != 0.0 {
                                let v4668 = v4662 * v4661;
                                let v4671 = ((v4661 * v4661) + v76).sqrt();
                                let v4679 = v4660 - (v31 * (v4661 + v4671));
                                let v4680 = ((v4662 + ((v4668 + v4668) * (v26 / (v24 * v4671)))) * v31) * v36;
                                v4683 = v4679;
                                v4684 = v4680;
                            } else {
                                let v4681 = v4632 - v4660;
                                let v4682 = if v4681 > v16 { 1.0 } else { 0.0 };
                                let v4708: f64;
                                let v4709: Lanes<2>;
                                if v4682 != 0.0 {
                                    let v4686 = v4633 * v4681;
                                    let v4689 = ((v4681 * v4681) + v76).sqrt();
                                    let v4693 = v4681 + v4689;
                                    let v4696 = v4695 / v4693;
                                    let v4700 = v4660 - v4696;
                                    let v4701 = ((((v4633 + ((v4686 + v4686) * (v26 / (v24 * v4689)))) * v4696) * v36) / v4693) * v36;
                                    v4708 = v4700;
                                    v4709 = v4701;
                                } else {
                                    let v4706 = v4660 - (v31 * (v4661 + v4702));
                                    let v4707 = (v4662 * v31) * v36;
                                    v4708 = v4706;
                                    v4709 = v4707;
                                }
                                v4683 = v4708;
                                v4684 = v4709;
                            }
                            v4664 = v4683;
                            v4665 = v4684;
                        } else {
                            v4664 = v4632;
                            v4665 = v4633;
                        }
                        let v4730: f64;
                        let v4731: Lanes<2>;
                        if v4666 != 0.0 {
                            let v4718 = -((v13 * v4201) + ((v4712 + v4621) * v202));
                            let v4719 = ((v4203 * v13) + (v4622 * v202)) * v36;
                            v4730 = v4718;
                            v4731 = v4719;
                        } else {
                            let v4728 = -((v13 * v4201) + ((v4722 + v4621) * v202));
                            let v4729 = ((v4203 * v13) + (v4622 * v202)) * v36;
                            v4730 = v4728;
                            v4731 = v4729;
                        }
                        let v4732 = if v4730 < v398 { 1.0 } else { 0.0 };
                        let v4739: f64;
                        let v4740: Lanes<2>;
                        if v4732 != 0.0 {
                            let v4733 = v4730.exp();
                            let v4735 = v44 + v4733;
                            let v4736 = v4735.ln();
                            let v4738 = (v4731 * v4733) * (v26 / v4735);
                            v4739 = v4736;
                            v4740 = v4738;
                        } else {
                            v4739 = v4730;
                            v4740 = v4731;
                        }
                        let v4745 = v4730 + ((v13 * v4591) * v202);
                        let v4746 = v4731 + ((v4192 * v13) * v202);
                        let v4747 = if v4745 < v398 { 1.0 } else { 0.0 };
                        let v4754: f64;
                        let v4755: Lanes<2>;
                        if v4747 != 0.0 {
                            let v4748 = v4745.exp();
                            let v4750 = v44 + v4748;
                            let v4751 = v4750.ln();
                            let v4753 = (v4746 * v4748) * (v26 / v4750);
                            v4754 = v4751;
                            v4755 = v4753;
                        } else {
                            v4754 = v4745;
                            v4755 = v4746;
                        }
                        let v4760 = v4759 + (v4756 * v4664);
                        let v4768 = v4767 * (v4765 + (v4664 * v4760));
                        let v4769 = ((v4665 * v4760) + ((v4665 * v4756) * v4664)) * v4767;
                        let v4770 = if v4768 > v34 { 1.0 } else { 0.0 };
                        let v4788: f64;
                        let v4789: Lanes<2>;
                        if v4770 != 0.0 {
                            let v4771 = v31 * v4768;
                            let v4775 = v44 + (v4768 * v214);
                            let v4780 = v44 + (v4771 * v4775);
                            let v4784 = (v4769 * v4780) + ((((v4769 * v31) * v4775) + ((v4769 * v214) * v4771)) * v4768);
                            let v4785 = v44 + (v4768 * v4780);
                            v4788 = v4785;
                            v4789 = v4784;
                        } else {
                            let v4787 = if v4768 > v4786 { 1.0 } else { 0.0 };
                            let v4829: f64;
                            let v4830: Lanes<2>;
                            if v4787 != 0.0 {
                                let v4801 = v4768.exp();
                                let v4802 = v4769 * v4801;
                                v4829 = v4801;
                                v4830 = v4802;
                            } else {
                                let v4804 = v4803 - v4768;
                                let v4805 = v4769 * v36;
                                let v4808 = v31 * (v4806 - v4768);
                                let v4814 = v44 + ((v4810 - v4768) * v214);
                                let v4819 = v44 + (v4808 * v4814);
                                let v4824 = v44 + (v4804 * v4819);
                                let v4825 = v562 / v4824;
                                let v4828 = ((((v4805 * v4819) + ((((v4805 * v31) * v4814) + ((v4805 * v214) * v4808)) * v4804)) * v4825) * v36) / v4824;
                                v4829 = v4825;
                                v4830 = v4828;
                            }
                            v4788 = v4829;
                            v4789 = v4830;
                        }
                        let v4793 = (v4790 * v4788) * v13;
                        let v4795 = v4754 - v4739;
                        let v4797 = v4793 * v4795;
                        let v4800 = (((v4789 * v4790) * v13) * v4795) + ((v4755 - v4740) * v4793);
                        v4603 = v4797;
                        v4604 = v4800;
                    } else {
                        v4603 = v34;
                        v4604 = v4199;
                    }
                    let v4836: f64;
                    let v4837: Lanes<2>;
                    if v4605 != 0.0 {
                        let v4832 = v4202 * v13;
                        let v4834 = (v13 * v4200) + v4833;
                        let v4835 = if v4834 > v16 { 1.0 } else { 0.0 };
                        let v4855: f64;
                        let v4856: Lanes<2>;
                        if v4835 != 0.0 {
                            let v4839 = v4832 * v4834;
                            let v4842 = ((v4834 * v4834) + v4094).sqrt();
                            let v4850 = v4834 - (v31 * (v4834 + v4842));
                            let v4851 = v4832 - ((v4832 + ((v4839 + v4839) * (v26 / (v24 * v4842)))) * v31);
                            v4855 = v4850;
                            v4856 = v4851;
                        } else {
                            let v4852 = v34 - v4834;
                            let v4853 = v4832 * v36;
                            let v4854 = if v4852 > v16 { 1.0 } else { 0.0 };
                            let v4892: f64;
                            let v4893: Lanes<2>;
                            if v4854 != 0.0 {
                                let v4870 = v4853 * v4852;
                                let v4873 = ((v4852 * v4852) + v4094).sqrt();
                                let v4877 = v4852 + v4873;
                                let v4880 = v4879 / v4877;
                                let v4884 = v4834 - v4880;
                                let v4885 = v4832 - ((((v4853 + ((v4870 + v4870) * (v26 / (v24 * v4873)))) * v4880) * v36) / v4877);
                                v4892 = v4884;
                                v4893 = v4885;
                            } else {
                                let v4890 = v4834 - (v31 * (v4834 + v4886));
                                let v4891 = v4832 - (v4832 * v31);
                                v4892 = v4890;
                                v4893 = v4891;
                            }
                            v4855 = v4892;
                            v4856 = v4893;
                        }
                        let v4858 = v4202 * v4200;
                        let v4861 = ((v4200 * v4200) + v76).sqrt();
                        let v4866 = v4861 * v4865;
                        let v4867 = ((v4858 + v4858) * (v26 / (v24 * v4861))) * v4865;
                        let v4898: f64;
                        let v4899: Lanes<2>;
                        if v4868 != 0.0 {
                            let v4895 = v4894 - v4866;
                            let v4896 = v4867 * v36;
                            let v4897 = if v4895 > v16 { 1.0 } else { 0.0 };
                            let v4917: f64;
                            let v4918: Lanes<2>;
                            if v4897 != 0.0 {
                                let v4902 = v4896 * v4895;
                                let v4905 = ((v4895 * v4895) + v76).sqrt();
                                let v4913 = v4894 - (v31 * (v4895 + v4905));
                                let v4914 = ((v4896 + ((v4902 + v4902) * (v26 / (v24 * v4905)))) * v31) * v36;
                                v4917 = v4913;
                                v4918 = v4914;
                            } else {
                                let v4915 = v4866 - v4894;
                                let v4916 = if v4915 > v16 { 1.0 } else { 0.0 };
                                let v4942: f64;
                                let v4943: Lanes<2>;
                                if v4916 != 0.0 {
                                    let v4920 = v4867 * v4915;
                                    let v4923 = ((v4915 * v4915) + v76).sqrt();
                                    let v4927 = v4915 + v4923;
                                    let v4930 = v4929 / v4927;
                                    let v4934 = v4894 - v4930;
                                    let v4935 = ((((v4867 + ((v4920 + v4920) * (v26 / (v24 * v4923)))) * v4930) * v36) / v4927) * v36;
                                    v4942 = v4934;
                                    v4943 = v4935;
                                } else {
                                    let v4940 = v4894 - (v31 * (v4895 + v4936));
                                    let v4941 = (v4896 * v31) * v36;
                                    v4942 = v4940;
                                    v4943 = v4941;
                                }
                                v4917 = v4942;
                                v4918 = v4943;
                            }
                            v4898 = v4917;
                            v4899 = v4918;
                        } else {
                            v4898 = v4866;
                            v4899 = v4867;
                        }
                        let v4960: f64;
                        let v4961: Lanes<2>;
                        if v4900 != 0.0 {
                            let v4950 = (v13 * v4201) + ((v4855 - v4946) * v202);
                            let v4951 = (v4203 * v13) + (v4856 * v202);
                            v4960 = v4950;
                            v4961 = v4951;
                        } else {
                            let v4958 = (v13 * v4201) + ((v4855 - v4954) * v202);
                            let v4959 = (v4203 * v13) + (v4856 * v202);
                            v4960 = v4958;
                            v4961 = v4959;
                        }
                        let v4962 = if v4960 < v398 { 1.0 } else { 0.0 };
                        let v4969: f64;
                        let v4970: Lanes<2>;
                        if v4962 != 0.0 {
                            let v4963 = v4960.exp();
                            let v4965 = v44 + v4963;
                            let v4966 = v4965.ln();
                            let v4968 = (v4961 * v4963) * (v26 / v4965);
                            v4969 = v4966;
                            v4970 = v4968;
                        } else {
                            v4969 = v4960;
                            v4970 = v4961;
                        }
                        let v4975 = v4960 - ((v13 * v4591) * v202);
                        let v4976 = v4961 - ((v4192 * v13) * v202);
                        let v4977 = if v4975 < v398 { 1.0 } else { 0.0 };
                        let v4984: f64;
                        let v4985: Lanes<2>;
                        if v4977 != 0.0 {
                            let v4978 = v4975.exp();
                            let v4980 = v44 + v4978;
                            let v4981 = v4980.ln();
                            let v4983 = (v4976 * v4978) * (v26 / v4980);
                            v4984 = v4981;
                            v4985 = v4983;
                        } else {
                            v4984 = v4975;
                            v4985 = v4976;
                        }
                        let v4990 = v4989 + (v4986 * v4898);
                        let v4998 = v4997 * (v4995 + (v4898 * v4990));
                        let v4999 = ((v4899 * v4990) + ((v4899 * v4986) * v4898)) * v4997;
                        let v5001 = if (v4998.abs()) < v398 { 1.0 } else { 0.0 };
                        let v5006: f64;
                        let v5007: Lanes<2>;
                        if v5001 != 0.0 {
                            let v5002 = v4998.exp();
                            let v5003 = v4999 * v5002;
                            v5006 = v5002;
                            v5007 = v5003;
                        } else {
                            let v5005 = if v4998 < v5004 { 1.0 } else { 0.0 };
                            let v5065: f64;
                            let v5066: Lanes<2>;
                            if v5005 != 0.0 {
                                let v5022 = v5021 - v4998;
                                let v5023 = v4999 * v36;
                                let v5026 = v31 * (v5024 - v4998);
                                let v5032 = v44 + ((v5028 - v4998) * v214);
                                let v5037 = v44 + (v5026 * v5032);
                                let v5042 = v44 + (v5022 * v5037);
                                let v5043 = v562 / v5042;
                                let v5046 = ((((v5023 * v5037) + ((((v5023 * v31) * v5032) + ((v5023 * v214) * v5026)) * v5022)) * v5043) * v36) / v5042;
                                v5065 = v5043;
                                v5066 = v5046;
                            } else {
                                let v5047 = v4998 - v398;
                                let v5048 = v31 * v5047;
                                let v5052 = v44 + (v5047 * v214);
                                let v5057 = v44 + (v5048 * v5052);
                                let v5063 = v456 * (v44 + (v5047 * v5057));
                                let v5064 = ((v4999 * v5057) + ((((v4999 * v31) * v5052) + ((v4999 * v214) * v5048)) * v5047)) * v456;
                                v5065 = v5063;
                                v5066 = v5064;
                            }
                            v5006 = v5065;
                            v5007 = v5066;
                        }
                        let v5011 = (v5008 * v5006) * v13;
                        let v5013 = v4969 - v4984;
                        let v5019 = v4603 + (v5011 * v5013);
                        let v5020 = v4604 + ((((v5007 * v5008) * v13) * v5013) + ((v4970 - v4985) * v5011));
                        v4836 = v5019;
                        v4837 = v5020;
                    } else {
                        v4836 = v4603;
                        v4837 = v4604;
                    }
                    v4593 = v4836;
                    v4594 = v4837;
                } else {
                    v4593 = v34;
                    v4594 = v4199;
                }
                let v5074: f64;
                let v5075: Lanes<3>;
                if v4595 != 0.0 {
                    let v5067 = v13 * v5;
                    let v5071 = (v938 - v2818) * v925;
                    let v5072 = ((Lanes([v940[0], v940[1], 0.0])) - v2821) * v925;
                    let v5082: f64;
                    let v5083: Lanes<3>;
                    if v5073 != 0.0 {
                        let v5077 = v5072 * v13;
                        let v5078 = (v13 * v5071) + v4598;
                        let v5079 = v34 - v5078;
                        let v5080 = v5077 * v36;
                        let v5081 = if v5079 > v16 { 1.0 } else { 0.0 };
                        let v5100: f64;
                        let v5101: Lanes<3>;
                        if v5081 != 0.0 {
                            let v5086 = v5080 * v5079;
                            let v5089 = ((v5079 * v5079) + v4094).sqrt();
                            let v5097 = v5078 + (v31 * (v5079 + v5089));
                            let v5098 = v5077 + ((v5080 + ((v5086 + v5086) * (v26 / (v24 * v5089)))) * v31);
                            v5100 = v5097;
                            v5101 = v5098;
                        } else {
                            let v5099 = if v5078 > v16 { 1.0 } else { 0.0 };
                            let v5136: f64;
                            let v5137: Lanes<3>;
                            if v5099 != 0.0 {
                                let v5114 = v5077 * v5078;
                                let v5117 = ((v5078 * v5078) + v4094).sqrt();
                                let v5121 = v5078 + v5117;
                                let v5124 = v5123 / v5121;
                                let v5128 = v5078 + v5124;
                                let v5129 = v5077 + ((((v5077 + ((v5114 + v5114) * (v26 / (v24 * v5117)))) * v5124) * v36) / v5121);
                                v5136 = v5128;
                                v5137 = v5129;
                            } else {
                                let v5134 = v5078 + (v31 * (v5079 + v5130));
                                let v5135 = v5077 + (v5080 * v31);
                                v5136 = v5134;
                                v5137 = v5135;
                            }
                            v5100 = v5136;
                            v5101 = v5137;
                        }
                        let v5103 = v5072 * v5071;
                        let v5106 = ((v5071 * v5071) + v76).sqrt();
                        let v5110 = v5106 * v4631;
                        let v5111 = ((v5103 + v5103) * (v26 / (v24 * v5106))) * v4631;
                        let v5141: f64;
                        let v5142: Lanes<3>;
                        if v5112 != 0.0 {
                            let v5138 = v4660 - v5110;
                            let v5139 = v5111 * v36;
                            let v5140 = if v5138 > v16 { 1.0 } else { 0.0 };
                            let v5160: f64;
                            let v5161: Lanes<3>;
                            if v5140 != 0.0 {
                                let v5145 = v5139 * v5138;
                                let v5148 = ((v5138 * v5138) + v76).sqrt();
                                let v5156 = v4660 - (v31 * (v5138 + v5148));
                                let v5157 = ((v5139 + ((v5145 + v5145) * (v26 / (v24 * v5148)))) * v31) * v36;
                                v5160 = v5156;
                                v5161 = v5157;
                            } else {
                                let v5158 = v5110 - v4660;
                                let v5159 = if v5158 > v16 { 1.0 } else { 0.0 };
                                let v5185: f64;
                                let v5186: Lanes<3>;
                                if v5159 != 0.0 {
                                    let v5163 = v5111 * v5158;
                                    let v5166 = ((v5158 * v5158) + v76).sqrt();
                                    let v5170 = v5158 + v5166;
                                    let v5173 = v5172 / v5170;
                                    let v5177 = v4660 - v5173;
                                    let v5178 = ((((v5111 + ((v5163 + v5163) * (v26 / (v24 * v5166)))) * v5173) * v36) / v5170) * v36;
                                    v5185 = v5177;
                                    v5186 = v5178;
                                } else {
                                    let v5183 = v4660 - (v31 * (v5138 + v5179));
                                    let v5184 = (v5139 * v31) * v36;
                                    v5185 = v5183;
                                    v5186 = v5184;
                                }
                                v5160 = v5185;
                                v5161 = v5186;
                            }
                            v5141 = v5160;
                            v5142 = v5161;
                        } else {
                            v5141 = v5110;
                            v5142 = v5111;
                        }
                        let v5207: f64;
                        let v5208: Lanes<3>;
                        if v5143 != 0.0 {
                            let v5195 = -((v13 * v2818) + ((v5189 + v5100) * v202));
                            let v5196 = ((v2821 * v13) + (v5101 * v202)) * v36;
                            v5207 = v5195;
                            v5208 = v5196;
                        } else {
                            let v5205 = -((v13 * v2818) + ((v5199 + v5100) * v202));
                            let v5206 = ((v2821 * v13) + (v5101 * v202)) * v36;
                            v5207 = v5205;
                            v5208 = v5206;
                        }
                        let v5209 = if v5207 < v398 { 1.0 } else { 0.0 };
                        let v5216: f64;
                        let v5217: Lanes<3>;
                        if v5209 != 0.0 {
                            let v5210 = v5207.exp();
                            let v5212 = v44 + v5210;
                            let v5213 = v5212.ln();
                            let v5215 = (v5208 * v5210) * (v26 / v5212);
                            v5216 = v5213;
                            v5217 = v5215;
                        } else {
                            v5216 = v5207;
                            v5217 = v5208;
                        }
                        let v5221 = (v15 * v13) * v202;
                        let v5222 = v5207 + ((v13 * v5067) * v202);
                        let v5224 = v5208 + (Lanes([v5221[0], v5221[1], 0.0]));
                        let v5225 = if v5222 < v398 { 1.0 } else { 0.0 };
                        let v5232: f64;
                        let v5233: Lanes<3>;
                        if v5225 != 0.0 {
                            let v5226 = v5222.exp();
                            let v5228 = v44 + v5226;
                            let v5229 = v5228.ln();
                            let v5231 = (v5224 * v5226) * (v26 / v5228);
                            v5232 = v5229;
                            v5233 = v5231;
                        } else {
                            v5232 = v5222;
                            v5233 = v5224;
                        }
                        let v5236 = v4759 + (v4756 * v5141);
                        let v5244 = v5243 * (v5241 + (v5141 * v5236));
                        let v5245 = ((v5142 * v5236) + ((v5142 * v4756) * v5141)) * v5243;
                        let v5246 = if v5244 > v34 { 1.0 } else { 0.0 };
                        let v5264: f64;
                        let v5265: Lanes<3>;
                        if v5246 != 0.0 {
                            let v5247 = v31 * v5244;
                            let v5251 = v44 + (v5244 * v214);
                            let v5256 = v44 + (v5247 * v5251);
                            let v5260 = (v5245 * v5256) + ((((v5245 * v31) * v5251) + ((v5245 * v214) * v5247)) * v5244);
                            let v5261 = v44 + (v5244 * v5256);
                            v5264 = v5261;
                            v5265 = v5260;
                        } else {
                            let v5263 = if v5244 > v5262 { 1.0 } else { 0.0 };
                            let v5305: f64;
                            let v5306: Lanes<3>;
                            if v5263 != 0.0 {
                                let v5277 = v5244.exp();
                                let v5278 = v5245 * v5277;
                                v5305 = v5277;
                                v5306 = v5278;
                            } else {
                                let v5280 = v5279 - v5244;
                                let v5281 = v5245 * v36;
                                let v5284 = v31 * (v5282 - v5244);
                                let v5290 = v44 + ((v5286 - v5244) * v214);
                                let v5295 = v44 + (v5284 * v5290);
                                let v5300 = v44 + (v5280 * v5295);
                                let v5301 = v562 / v5300;
                                let v5304 = ((((v5281 * v5295) + ((((v5281 * v31) * v5290) + ((v5281 * v214) * v5284)) * v5280)) * v5301) * v36) / v5300;
                                v5305 = v5301;
                                v5306 = v5304;
                            }
                            v5264 = v5305;
                            v5265 = v5306;
                        }
                        let v5269 = (v5266 * v5264) * v13;
                        let v5271 = v5232 - v5216;
                        let v5273 = v5269 * v5271;
                        let v5276 = (((v5265 * v5266) * v13) * v5271) + ((v5233 - v5217) * v5269);
                        v5082 = v5273;
                        v5083 = v5276;
                    } else {
                        v5082 = v34;
                        v5083 = v2817;
                    }
                    let v5311: f64;
                    let v5312: Lanes<3>;
                    if v5084 != 0.0 {
                        let v5308 = v5072 * v13;
                        let v5309 = (v13 * v5071) + v4833;
                        let v5310 = if v5309 > v16 { 1.0 } else { 0.0 };
                        let v5330: f64;
                        let v5331: Lanes<3>;
                        if v5310 != 0.0 {
                            let v5314 = v5308 * v5309;
                            let v5317 = ((v5309 * v5309) + v4094).sqrt();
                            let v5325 = v5309 - (v31 * (v5309 + v5317));
                            let v5326 = v5308 - ((v5308 + ((v5314 + v5314) * (v26 / (v24 * v5317)))) * v31);
                            v5330 = v5325;
                            v5331 = v5326;
                        } else {
                            let v5327 = v34 - v5309;
                            let v5328 = v5308 * v36;
                            let v5329 = if v5327 > v16 { 1.0 } else { 0.0 };
                            let v5366: f64;
                            let v5367: Lanes<3>;
                            if v5329 != 0.0 {
                                let v5344 = v5328 * v5327;
                                let v5347 = ((v5327 * v5327) + v4094).sqrt();
                                let v5351 = v5327 + v5347;
                                let v5354 = v5353 / v5351;
                                let v5358 = v5309 - v5354;
                                let v5359 = v5308 - ((((v5328 + ((v5344 + v5344) * (v26 / (v24 * v5347)))) * v5354) * v36) / v5351);
                                v5366 = v5358;
                                v5367 = v5359;
                            } else {
                                let v5364 = v5309 - (v31 * (v5309 + v5360));
                                let v5365 = v5308 - (v5308 * v31);
                                v5366 = v5364;
                                v5367 = v5365;
                            }
                            v5330 = v5366;
                            v5331 = v5367;
                        }
                        let v5333 = v5072 * v5071;
                        let v5336 = ((v5071 * v5071) + v76).sqrt();
                        let v5340 = v5336 * v4865;
                        let v5341 = ((v5333 + v5333) * (v26 / (v24 * v5336))) * v4865;
                        let v5371: f64;
                        let v5372: Lanes<3>;
                        if v5342 != 0.0 {
                            let v5368 = v4894 - v5340;
                            let v5369 = v5341 * v36;
                            let v5370 = if v5368 > v16 { 1.0 } else { 0.0 };
                            let v5390: f64;
                            let v5391: Lanes<3>;
                            if v5370 != 0.0 {
                                let v5375 = v5369 * v5368;
                                let v5378 = ((v5368 * v5368) + v76).sqrt();
                                let v5386 = v4894 - (v31 * (v5368 + v5378));
                                let v5387 = ((v5369 + ((v5375 + v5375) * (v26 / (v24 * v5378)))) * v31) * v36;
                                v5390 = v5386;
                                v5391 = v5387;
                            } else {
                                let v5388 = v5340 - v4894;
                                let v5389 = if v5388 > v16 { 1.0 } else { 0.0 };
                                let v5415: f64;
                                let v5416: Lanes<3>;
                                if v5389 != 0.0 {
                                    let v5393 = v5341 * v5388;
                                    let v5396 = ((v5388 * v5388) + v76).sqrt();
                                    let v5400 = v5388 + v5396;
                                    let v5403 = v5402 / v5400;
                                    let v5407 = v4894 - v5403;
                                    let v5408 = ((((v5341 + ((v5393 + v5393) * (v26 / (v24 * v5396)))) * v5403) * v36) / v5400) * v36;
                                    v5415 = v5407;
                                    v5416 = v5408;
                                } else {
                                    let v5413 = v4894 - (v31 * (v5368 + v5409));
                                    let v5414 = (v5369 * v31) * v36;
                                    v5415 = v5413;
                                    v5416 = v5414;
                                }
                                v5390 = v5415;
                                v5391 = v5416;
                            }
                            v5371 = v5390;
                            v5372 = v5391;
                        } else {
                            v5371 = v5340;
                            v5372 = v5341;
                        }
                        let v5431: f64;
                        let v5432: Lanes<3>;
                        if v5373 != 0.0 {
                            let v5422 = (v13 * v2818) + ((v5330 - v4946) * v202);
                            let v5423 = (v2821 * v13) + (v5331 * v202);
                            v5431 = v5422;
                            v5432 = v5423;
                        } else {
                            let v5429 = (v13 * v2818) + ((v5330 - v4954) * v202);
                            let v5430 = (v2821 * v13) + (v5331 * v202);
                            v5431 = v5429;
                            v5432 = v5430;
                        }
                        let v5433 = if v5431 < v398 { 1.0 } else { 0.0 };
                        let v5440: f64;
                        let v5441: Lanes<3>;
                        if v5433 != 0.0 {
                            let v5434 = v5431.exp();
                            let v5436 = v44 + v5434;
                            let v5437 = v5436.ln();
                            let v5439 = (v5432 * v5434) * (v26 / v5436);
                            v5440 = v5437;
                            v5441 = v5439;
                        } else {
                            v5440 = v5431;
                            v5441 = v5432;
                        }
                        let v5445 = (v15 * v13) * v202;
                        let v5446 = v5431 - ((v13 * v5067) * v202);
                        let v5448 = v5432 - (Lanes([v5445[0], v5445[1], 0.0]));
                        let v5449 = if v5446 < v398 { 1.0 } else { 0.0 };
                        let v5456: f64;
                        let v5457: Lanes<3>;
                        if v5449 != 0.0 {
                            let v5450 = v5446.exp();
                            let v5452 = v44 + v5450;
                            let v5453 = v5452.ln();
                            let v5455 = (v5448 * v5450) * (v26 / v5452);
                            v5456 = v5453;
                            v5457 = v5455;
                        } else {
                            v5456 = v5446;
                            v5457 = v5448;
                        }
                        let v5460 = v4989 + (v4986 * v5371);
                        let v5468 = v5467 * (v5465 + (v5371 * v5460));
                        let v5469 = ((v5372 * v5460) + ((v5372 * v4986) * v5371)) * v5467;
                        let v5471 = if (v5468.abs()) < v398 { 1.0 } else { 0.0 };
                        let v5476: f64;
                        let v5477: Lanes<3>;
                        if v5471 != 0.0 {
                            let v5472 = v5468.exp();
                            let v5473 = v5469 * v5472;
                            v5476 = v5472;
                            v5477 = v5473;
                        } else {
                            let v5475 = if v5468 < v5474 { 1.0 } else { 0.0 };
                            let v5535: f64;
                            let v5536: Lanes<3>;
                            if v5475 != 0.0 {
                                let v5492 = v5491 - v5468;
                                let v5493 = v5469 * v36;
                                let v5496 = v31 * (v5494 - v5468);
                                let v5502 = v44 + ((v5498 - v5468) * v214);
                                let v5507 = v44 + (v5496 * v5502);
                                let v5512 = v44 + (v5492 * v5507);
                                let v5513 = v562 / v5512;
                                let v5516 = ((((v5493 * v5507) + ((((v5493 * v31) * v5502) + ((v5493 * v214) * v5496)) * v5492)) * v5513) * v36) / v5512;
                                v5535 = v5513;
                                v5536 = v5516;
                            } else {
                                let v5517 = v5468 - v398;
                                let v5518 = v31 * v5517;
                                let v5522 = v44 + (v5517 * v214);
                                let v5527 = v44 + (v5518 * v5522);
                                let v5533 = v456 * (v44 + (v5517 * v5527));
                                let v5534 = ((v5469 * v5527) + ((((v5469 * v31) * v5522) + ((v5469 * v214) * v5518)) * v5517)) * v456;
                                v5535 = v5533;
                                v5536 = v5534;
                            }
                            v5476 = v5535;
                            v5477 = v5536;
                        }
                        let v5481 = (v5478 * v5476) * v13;
                        let v5483 = v5440 - v5456;
                        let v5489 = v5082 + (v5481 * v5483);
                        let v5490 = v5083 + ((((v5477 * v5478) * v13) * v5483) + ((v5441 - v5457) * v5481));
                        v5311 = v5489;
                        v5312 = v5490;
                    } else {
                        v5311 = v5082;
                        v5312 = v5083;
                    }
                    v5074 = v5311;
                    v5075 = v5312;
                } else {
                    v5074 = v34;
                    v5075 = v2817;
                }
                v4559 = v5074;
                v4560 = v4593;
                v4561 = v5075;
                v4562 = v4594;
            } else {
                v4559 = v34;
                v4560 = v34;
                v4561 = v2817;
                v4562 = v4199;
            }
            let v4571 = (((v242 - v2819) - v2820) * v4567) * v4570;
            let v4577 = (v4571 * v4086) * v13;
            let v4578 = ((((((v2211 - v2822) - v2823) * v4567) * v4570) * v4086) + (v4087 * v4571)) * v13;
            let v4580 = v4579 * v2209;
            let v4581 = v2212 * v4579;
            let v4589 = v4588 * (v4582 - v4183);
            let v4590 = ((Lanes([0.0, v4584[0]])) - (Lanes([v4186[0], 0.0]))) * v4588;
            let v5578: f64;
            let v5579: f64;
            let v5580: f64;
            let v5581: f64;
            let v5582: Lanes<2>;
            let v5583: Lanes<2>;
            let v5584: Lanes<4>;
            let v5585: Lanes<2>;
            if v1 != 0.0 {
                let v5544 = (v5537 - v4582) * v5543;
                let v5545 = ((Lanes([v5539[0], 0.0])) - (Lanes([0.0, v4584[0]]))) * v5543;
                let v5551 = (v4582 - v3) * v5550;
                let v5552 = ((Lanes([v4584[0], 0.0])) - (Lanes([0.0, v6[0]]))) * v5550;
                let v5553 = v4 - v4183;
                let v5558 = v5557 + v4153;
                let v5559 = v5553 * v5558;
                let v5560 = ((Lanes([0.0, v8[0]])) - (Lanes([v4186[0], 0.0]))) * v5558;
                let v5561 = v4157 * v5553;
                let v5564 = (Lanes([v5560[0], 0.0, v5560[1], 0.0])) + (Lanes([0.0, v5561[0], v5561[1], v5561[2]]));
                let v5572 = (v4183 - v5565) * v5571;
                let v5573 = ((Lanes([v4186[0], 0.0])) - (Lanes([0.0, v5568[0]]))) * v5571;
                v5578 = v5544;
                v5579 = v5551;
                v5580 = v5559;
                v5581 = v5572;
                v5582 = v5545;
                v5583 = v5552;
                v5584 = v5564;
                v5585 = v5573;
            } else {
                v5578 = v34;
                v5579 = v34;
                v5580 = v34;
                v5581 = v34;
                v5582 = v5574;
                v5583 = v5575;
                v5584 = v5576;
                v5585 = v5577;
            }
            let v5586 = v13 * v4559;
            let v5587 = v4561 * v13;
            let v5588 = v13 * v4560;
            let v5589 = v4562 * v13;
            let v5593 = if ((v4559 + v4560).abs()) > v5592 { 1.0 } else { 0.0 };
            let v5594 = ddt(16846, v4577);
            let v5596 = v4578 * v5595;
            let v5597 = -v2207;
            let v5598 = v2208 * v36;
            let v5599 = ddt(16850, v4580);
            let v5600 = v4581 * v5595;
            let v5601 = ddt(16852, v4589);
            let v5602 = v4590 * v5595;
            let v5606 = if ((v5537 - v5565).abs()) > v5605 { 1.0 } else { 0.0 };
            let v5607 = v4578[0];
            let v5608 = v2212[0];
            let v5609 = v5582[0];
            let v5610 = v5582[1];
            let v5611 = v5583[0];
            let v5612 = v5583[1];
            let v5613 = v5584[0];
            let v5614 = v5584[1];
            let v5615 = v5584[2];
            let v5616 = v5584[3];
            let v5617 = v5585[0];
            let v5618 = v5585[1];
            let v5619 = v5587[0];
            let v5620 = v5587[1];
            let v5621 = v5587[2];
            let v5622 = v5589[0];
            let v5623 = v5589[1];
            let v5624 = v5596[0];
            let v5625 = v5596[1];
            let v5626 = v5596[2];
            let v5627 = v5598[0];
            let v5628 = v5598[1];
            let v5629 = v5600[0];
            let v5630 = v5602[0];
            let v5631 = v5602[1];
            let v5632 = v4578[1];
            let v5633 = v4578[2];
            let v5634 = v4581[0];
            let v5635 = v4590[0];
            let v5636 = v4590[1];
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(6),
            None,
            multiplicity * (v2209),
            [6],
            [v5608],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(0),
            Some(3),
            multiplicity * (v5578),
            [0, 3],
            [v5609, v5610],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(3),
            Some(4),
            multiplicity * (v5579),
            [3, 4],
            [v5611, v5612],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(1),
            multiplicity * (v5580),
            [1, 4, 5, 6],
            [v5613, v5614, v5615, v5616],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(2),
            multiplicity * (v5581),
            [1, 2],
            [v5617, v5618],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(0), Some(3), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            staged[116],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(3), Some(4), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            staged[117],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(5), Some(1), 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            staged[118],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(1), Some(2), 3, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            3,
            staged[119],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(4),
            Some(5),
            multiplicity * (v5586),
            [4, 5, 6],
            [v5619, v5620, v5621],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(4),
            Some(1),
            multiplicity * (v5588),
            [1, 4],
            [v5622, v5623],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(4),
            Some(5),
            multiplicity * (v5594),
            [4, 5, 6],
            [v5624, v5625, v5626],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(6),
            None,
            multiplicity * (v5597),
            [4, 5],
            [v5627, v5628],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(6),
            None,
            multiplicity * (v5599),
            [6],
            [v5629],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(3),
            Some(1),
            multiplicity * (v5601),
            [1, 3],
            [v5630, v5631],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(5),
            multiplicity * (staged[120]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(1),
            multiplicity * (staged[121]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(3),
            multiplicity * (staged[122]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(3),
            Some(4),
            multiplicity * (staged[123]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(2),
            multiplicity * (staged[124]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(1),
            multiplicity * (staged[125]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(1),
            multiplicity * (staged[126]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = v2209;
        self.canonical_reactive[1] = v5578;
        self.canonical_reactive[2] = v5579;
        self.canonical_reactive[3] = v5580;
        self.canonical_reactive[4] = v5581;
        self.canonical_reactive[5] = staged[116];
        self.canonical_reactive[6] = staged[117];
        self.canonical_reactive[7] = staged[118];
        self.canonical_reactive[8] = staged[119];
        self.canonical_reactive[9] = v5586;
        self.canonical_reactive[10] = v5588;
        self.canonical_reactive[11] = v4577;
        self.canonical_reactive[12] = v5607;
        self.canonical_reactive[13] = v5632;
        self.canonical_reactive[14] = v5633;
        self.canonical_reactive[15] = v5597;
        self.canonical_reactive[16] = v4580;
        self.canonical_reactive[17] = v5634;
        self.canonical_reactive[18] = v4589;
        self.canonical_reactive[19] = v5635;
        self.canonical_reactive[20] = v5636;
        self.canonical_reactive[21] = staged[120];
        self.canonical_reactive[22] = staged[121];
        self.canonical_reactive[23] = staged[122];
        self.canonical_reactive[24] = staged[123];
        self.canonical_reactive[25] = staged[124];
        self.canonical_reactive[26] = staged[125];
        self.canonical_reactive[27] = staged[126];
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            Some(5),
            &[4, 5, 6],
            &[cached[12], cached[13], cached[14]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            None,
            &[6],
            &[cached[17]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            Some(1),
            &[1, 3],
            &[cached[19], cached[20]],
            &[],
            &[],
            multiplicity,
        );
    }

}

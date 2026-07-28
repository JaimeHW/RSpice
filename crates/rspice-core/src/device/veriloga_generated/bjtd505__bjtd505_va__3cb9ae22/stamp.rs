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
        let produced: [f64; 104] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let v0 = parameters[3];
                let v1 = 1e0f64;
                let v3 = 1.23e8f64;
                let v4 = 7.03e7f64;
                let v5 = 2.04e8f64;
                let v6 = 1.58e8f64;
                let v9 = parameters[32];
                let v11 = parameters[4];
                let v12 = 2.7315e2f64;
                let v14 = parameters[137];
                let v15 = 0e0f64;
                let v17 = 1e-12f64;
                let v19 = parameters[1];
                let v22 = 2e0f64;
                let v23 = parameters[66];
                let v27 = parameters[114];
                let v30 = parameters[115];
                let v33 = parameters[113];
                let v35 = 5e-2f64;
                let v37 = 1e-1f64;
                let v53 = parameters[65];
                let v55 = parameters[71];
                let v59 = parameters[117];
                let v62 = parameters[118];
                let v65 = parameters[116];
                let v83 = parameters[70];
                let v85 = parameters[82];
                let v88 = 8.617086918058125e-5f64;
                let v91 = parameters[74];
                let v93 = parameters[97];
                let v94 = parameters[95];
                let v96 = parameters[121];
                let v98 = parameters[122];
                let v100 = 4e0f64;
                let v103 = parameters[120];
                let v105 = parameters[104];
                let v108 = parameters[102];
                let v110 = parameters[20];
                let v112 = 6e0f64;
                let v114 = parameters[112];
                let v116 = parameters[31];
                let v119 = parameters[109];
                let v121 = parameters[96];
                let v124 = parameters[110];
                let v126 = parameters[23];
                let v128 = parameters[106];
                let v130 = parameters[105];
                let v132 = parameters[107];
                let v136 = parameters[111];
                let v138 = parameters[22];
                let v141 = parameters[133];
                let v144 = parameters[119];
                let v148 = parameters[98];
                let v150 = parameters[86];
                let v151 = parameters[87];
                let v153 = parameters[99];
                let v155 = parameters[56];
                let v157 = parameters[57];
                let v159 = parameters[58];
                let v161 = -1e0f64;
                let v163 = 3e0f64;
                let v167 = 1e0f64;
                let v169 = parameters[73];
                let v172 = -1e0f64;
                let v174 = parameters[75];
                let v178 = parameters[91];
                let v180 = parameters[92];
                let v183 = parameters[33];
                let v185 = parameters[34];
                let v188 = parameters[35];
                let v190 = parameters[36];
                let v193 = parameters[5];
                let v199 = parameters[83];
                let v201 = parameters[81];
                let v205 = parameters[80];
                let v213 = parameters[67];
                let v215 = parameters[76];
                let v217 = parameters[84];
                let v219 = parameters[78];
                let v225 = parameters[6];
                let v227 = 5e-1f64;
                let v232 = parameters[94];
                let v234 = parameters[93];
                let v236 = parameters[68];
                let v238 = parameters[77];
                let v240 = 0e0f64;
                let v242 = 0e0f64;
                let v244 = parameters[129];
                let v246 = parameters[130];
                let v250 = 0e0f64;
                let v251 = 0e0f64;
                let v262 = 0e0f64;
                let v263 = 0e0f64;
                let v264 = 0e0f64;
                let v265 = 0e0f64;
                let v266 = 0e0f64;
                let v272 = 0e0f64;
                let v273 = 0e0f64;
                let v274 = 0e0f64;
                let v2 = if v0 == v1 { 1.0 } else { 0.0 };
                let v7: f64;
                let v8: f64;
                if v2 != 0.0 {
                    v7 = v3;
                    v8 = v4;
                } else {
                    v7 = v5;
                    v8 = v6;
                }
                let v10 = v1 - v9;
                let v13 = v11 + v12;
                let v16 = if v14 == v15 { 1.0 } else { 0.0 };
                let v18: f64;
                if v16 != 0.0 {
                    v18 = v17;
                } else {
                    v18 = v14;
                }
                let v20 = v18 * v19;
                let v21 = v1 / v20;
                let v25 = v22.powf((v22 - v23));
                let v26 = v1 / v25;
                let v34 = v33 + (((v27 * v13) * v13) / (v13 + v30));
                let v38 = (v34 - v35) / v37;
                let v39 = if v34 < v35 { 1.0 } else { 0.0 };
                let v51: f64;
                if v39 != 0.0 {
                    let v44 = v35 + (v37 * ((v1 + (v38.exp())).ln()));
                    v51 = v44;
                } else {
                    let v50 = v34 + (v37 * ((v1 + ((-v38).exp())).ln()));
                    v51 = v50;
                }
                let v52 = v1 / v33;
                let v54 = v1 / v53;
                let v57 = v22.powf((v22 - v55));
                let v58 = v1 / v57;
                let v66 = v65 + (((v59 * v13) * v13) / (v13 + v62));
                let v68 = (v66 - v35) / v37;
                let v69 = if v66 < v35 { 1.0 } else { 0.0 };
                let v81: f64;
                if v69 != 0.0 {
                    let v74 = v35 + (v37 * ((v1 + (v68.exp())).ln()));
                    v81 = v74;
                } else {
                    let v80 = v66 + (v37 * ((v1 + ((-v68).exp())).ln()));
                    v81 = v80;
                }
                let v82 = v1 / v65;
                let v84 = v1 / v83;
                let v87 = v1 - (v1 / v85);
                let v90 = v1 / (v88 * v13);
                let v92 = v1 - v91;
                let v95 = v93 - v94;
                let v97 = if v96 != v15 { 1.0 } else { 0.0 };
                let v99 = if v98 != v15 { 1.0 } else { 0.0 };
                let v104 = ((v100 - v93) - v94) + v103;
                let v106 = -v105;
                let v107 = v1 - v93;
                let v109 = v1 - v108;
                let v113 = v112 - (v22 * v110);
                let v115 = -v114;
                let v118 = v112 - (v22 * v116);
                let v120 = -v119;
                let v123 = (v100 - v121) + v103;
                let v125 = -v124;
                let v127 = if v126 == v1 { 1.0 } else { 0.0 };
                if v127 != 0.0 {
                    let v129 = -v128;
                    let v131 = -v130;
                    let v133 = -v132;
                } else {
                }
                let v135 = (v100 - v108) + v103;
                let v137 = -v136;
                let v140 = v112 - (v22 * v138);
                let v142 = v100 / v141;
                let v143 = v93 - v22;
                let v145 = -v144;
                let v147 = (v94 + v93) - v1;
                let v149 = v148 - v1;
                let v152 = v150 + v151;
                let v154 = v153 - v1;
                let v156 = if v155 > v15 { 1.0 } else { 0.0 };
                let v158 = if v157 > v15 { 1.0 } else { 0.0 };
                let v160 = if v159 > v15 { 1.0 } else { 0.0 };
                let v165 = v1 - (v163.powf((v161 / v23)));
                let v166 = v1 - v23;
                let v168 = v166 - v167;
                let v170 = if v169 == v1 { 1.0 } else { 0.0 };
                if v170 != 0.0 {
                } else {
                    let v171 = if v169 == v22 { 1.0 } else { 0.0 };
                }
                let v173 = v172 / v55;
                let v175 = v174 - v167;
                let v176 = v1 - v55;
                let v177 = v176 - v167;
                let v179 = if v178 == v15 { 1.0 } else { 0.0 };
                if v127 != 0.0 {
                } else {
                    let v181 = if v180 == v15 { 1.0 } else { 0.0 };
                    if v181 != 0.0 {
                    } else {
                        let v182 = v1 - v180;
                    }
                }
                let v187 = if (if v183 > v15 { 1.0 } else { 0.0 }) != 0.0 && (if v185 > v15 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v192 = if (if v188 > v15 { 1.0 } else { 0.0 }) != 0.0 && (if v190 > v15 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v195 = if v9 > v15 { 1.0 } else { 0.0 };
                let v196 = if (if v193 > v15 { 1.0 } else { 0.0 }) != 0.0 && v195 != 0.0 { 1.0 } else { 0.0 };
                if v196 != 0.0 {
                    let v197 = v9 * v22;
                    let v198 = if v193 == v1 { 1.0 } else { 0.0 };
                } else {
                }
                let v200 = if v199 == v1 { 1.0 } else { 0.0 };
                if v200 != 0.0 {
                    let v204 = v1 / (v1 - (v87.powf(v201)));
                    let v206 = v87 * v205;
                    let v212 = (((v204 * v204) * (v87.powf((v201 - v1)))) * v201) / v205;
                } else {
                }
                let v214 = v1 - v213;
                let v216 = v1 - v215;
                let v218 = v1 / v217;
                let v220 = if v219 == v15 { 1.0 } else { 0.0 };
                let v224 = if (if (if v193 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v193 == v163 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v195 != 0.0 { 1.0 } else { 0.0 };
                if v224 != 0.0 {
                    if v220 != 0.0 {
                        let v228 = v227 * v9;
                    } else {
                        let v229 = v22 * v9;
                    }
                } else {
                }
                let v226 = if v225 == v1 { 1.0 } else { 0.0 };
                if v226 != 0.0 {
                    let v230 = -v23;
                    let v231 = v230 - v167;
                    let v233 = v1 - v232;
                    let v235 = v1 - v234;
                } else {
                }
                let v237 = v0 * v236;
                let v239 = v0 * v238;
                let v241: f64;
                if v158 != 0.0 {
                    v241 = v15;
                } else {
                    v241 = v240;
                }
                let v243: f64;
                if v160 != 0.0 {
                    v243 = v15;
                } else {
                    v243 = v242;
                }
                let v245 = if v244 > v15 { 1.0 } else { 0.0 };
                let v247 = if v246 == v1 { 1.0 } else { 0.0 };
                if v247 != 0.0 {
                } else {
                    let v248 = if v246 == v22 { 1.0 } else { 0.0 };
                }
                let v249 = if v9 == v15 { 1.0 } else { 0.0 };
                let v252: f64;
                let v253: f64;
                if v127 != 0.0 {
                    v252 = v250;
                    v253 = v15;
                } else {
                    v252 = v15;
                    v253 = v251;
                }
                let v254: f64;
                let v255: f64;
                let v256: f64;
                let v257: f64;
                let v258: f64;
                let v259: f64;
                let v260: f64;
                let v261: f64;
                if v158 != 0.0 {
                    let v267: f64;
                    let v268: f64;
                    let v269: f64;
                    let v270: f64;
                    let v271: f64;
                    if v160 != 0.0 {
                        v267 = v262;
                        v268 = v15;
                        v269 = v263;
                        v270 = v264;
                        v271 = v15;
                    } else {
                        v267 = v15;
                        v268 = v265;
                        v269 = v15;
                        v270 = v15;
                        v271 = v266;
                    }
                    v254 = v267;
                    v255 = v268;
                    v256 = v15;
                    v257 = v15;
                    v258 = v269;
                    v259 = v270;
                    v260 = v271;
                    v261 = v15;
                } else {
                    let v275: f64;
                    let v276: f64;
                    let v277: f64;
                    if v160 != 0.0 {
                        v275 = v272;
                        v276 = v15;
                        v277 = v273;
                    } else {
                        v275 = v15;
                        v276 = v274;
                        v277 = v15;
                    }
                    v254 = v15;
                    v255 = v15;
                    v256 = v275;
                    v257 = v276;
                    v258 = v15;
                    v259 = v15;
                    v260 = v15;
                    v261 = v277;
                }
                let v278 = if v19 != v1 { 1.0 } else { 0.0 };
            [v2, v10, v13, v16, v20, v21, v25, v26, v39, v52, v54, v57, v58, v69, v82, v84, v90, v51, v81, v92, v95, v97, v99, v104, v106, v107, v109, v113, v115, v118, v120, v123, v125, v127, v129, v131, v133, v135, v137, v140, v142, v143, v145, v147, v149, v152, v154, v7, v156, v158, v160, v165, v166, v170, v171, v173, v176, v179, v181, v182, v187, v192, v196, v197, v198, v200, v204, v206, v212, v8, v214, v216, v218, v220, v224, v228, v229, v226, v230, v233, v235, v237, v239, v245, v247, v248, v249, v254, v255, v256, v257, v278, v241, v243, v252, v253, v258, v259, v260, v261, v168, v175, v177, v231]
        };
        self.canonical_staged[142] = produced[0];
        self.canonical_staged[90] = produced[1];
        self.canonical_staged[0] = produced[2];
        self.canonical_staged[143] = produced[3];
        self.canonical_staged[5] = produced[4];
        self.canonical_staged[35] = produced[5];
        self.canonical_staged[76] = produced[6];
        self.canonical_staged[80] = produced[7];
        self.canonical_staged[144] = produced[8];
        self.canonical_staged[24] = produced[9];
        self.canonical_staged[25] = produced[10];
        self.canonical_staged[83] = produced[11];
        self.canonical_staged[87] = produced[12];
        self.canonical_staged[145] = produced[13];
        self.canonical_staged[26] = produced[14];
        self.canonical_staged[27] = produced[15];
        self.canonical_staged[1] = produced[16];
        self.canonical_staged[2] = produced[17];
        self.canonical_staged[3] = produced[18];
        self.canonical_staged[4] = produced[19];
        self.canonical_staged[6] = produced[20];
        self.canonical_staged[156] = produced[21];
        self.canonical_staged[158] = produced[22];
        self.canonical_staged[7] = produced[23];
        self.canonical_staged[8] = produced[24];
        self.canonical_staged[9] = produced[25];
        self.canonical_staged[10] = produced[26];
        self.canonical_staged[11] = produced[27];
        self.canonical_staged[12] = produced[28];
        self.canonical_staged[13] = produced[29];
        self.canonical_staged[14] = produced[30];
        self.canonical_staged[15] = produced[31];
        self.canonical_staged[16] = produced[32];
        self.canonical_staged[161] = produced[33];
        self.canonical_staged[17] = produced[34];
        self.canonical_staged[18] = produced[35];
        self.canonical_staged[19] = produced[36];
        self.canonical_staged[20] = produced[37];
        self.canonical_staged[21] = produced[38];
        self.canonical_staged[22] = produced[39];
        self.canonical_staged[23] = produced[40];
        self.canonical_staged[28] = produced[41];
        self.canonical_staged[29] = produced[42];
        self.canonical_staged[30] = produced[43];
        self.canonical_staged[31] = produced[44];
        self.canonical_staged[32] = produced[45];
        self.canonical_staged[33] = produced[46];
        self.canonical_staged[34] = produced[47];
        self.canonical_staged[163] = produced[48];
        self.canonical_staged[165] = produced[49];
        self.canonical_staged[167] = produced[50];
        self.canonical_staged[42] = produced[51];
        self.canonical_staged[46] = produced[52];
        self.canonical_staged[169] = produced[53];
        self.canonical_staged[170] = produced[54];
        self.canonical_staged[48] = produced[55];
        self.canonical_staged[50] = produced[56];
        self.canonical_staged[171] = produced[57];
        self.canonical_staged[172] = produced[58];
        self.canonical_staged[68] = produced[59];
        self.canonical_staged[75] = produced[60];
        self.canonical_staged[81] = produced[61];
        self.canonical_staged[173] = produced[62];
        self.canonical_staged[91] = produced[63];
        self.canonical_staged[174] = produced[64];
        self.canonical_staged[175] = produced[65];
        self.canonical_staged[98] = produced[66];
        self.canonical_staged[96] = produced[67];
        self.canonical_staged[97] = produced[68];
        self.canonical_staged[102] = produced[69];
        self.canonical_staged[106] = produced[70];
        self.canonical_staged[113] = produced[71];
        self.canonical_staged[114] = produced[72];
        self.canonical_staged[176] = produced[73];
        self.canonical_staged[177] = produced[74];
        self.canonical_staged[124] = produced[75];
        self.canonical_staged[126] = produced[76];
        self.canonical_staged[178] = produced[77];
        self.canonical_staged[128] = produced[78];
        self.canonical_staged[129] = produced[79];
        self.canonical_staged[130] = produced[80];
        self.canonical_staged[131] = produced[81];
        self.canonical_staged[132] = produced[82];
        self.canonical_staged[179] = produced[83];
        self.canonical_staged[180] = produced[84];
        self.canonical_staged[181] = produced[85];
        self.canonical_staged[182] = produced[86];
        self.canonical_staged[188] = produced[87];
        self.canonical_staged[191] = produced[88];
        self.canonical_staged[193] = produced[89];
        self.canonical_staged[195] = produced[90];
        self.canonical_staged[183] = produced[91];
        self.canonical_staged[184] = produced[92];
        self.canonical_staged[185] = produced[93];
        self.canonical_staged[186] = produced[94];
        self.canonical_staged[187] = produced[95];
        self.canonical_staged[189] = produced[96];
        self.canonical_staged[190] = produced[97];
        self.canonical_staged[192] = produced[98];
        self.canonical_staged[194] = produced[99];
        self.canonical_staged[137] = produced[100];
        self.canonical_staged[138] = produced[101];
        self.canonical_staged[139] = produced[102];
        self.canonical_staged[141] = produced[103];
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
        let produced: [f64; 92] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let temperature = ctx.temperature();
            let staged = &*self.canonical_staged;
                let v0 = temperature;
                let v1 = parameters[0];
                let v3 = staged[0];
                let v5 = 8.617086918058125e-5f64;
                let v7 = 1e0f64;
                let v9 = staged[1];
                let v13 = parameters[114];
                let v16 = parameters[115];
                let v19 = staged[2];
                let v21 = 5e-2f64;
                let v23 = 1e-1f64;
                let v38 = parameters[117];
                let v41 = parameters[118];
                let v44 = staged[3];
                let v61 = -3e0f64;
                let v64 = parameters[65];
                let v68 = parameters[104];
                let v86 = -3e0f64;
                let v89 = parameters[63];
                let v92 = parameters[109];
                let v110 = -3e0f64;
                let v113 = parameters[79];
                let v132 = -3e0f64;
                let v135 = parameters[70];
                let v154 = -3e0f64;
                let v174 = -3e0f64;
                let v177 = parameters[26];
                let v180 = parameters[108];
                let v201 = parameters[66];
                let v204 = parameters[71];
                let v206 = parameters[64];
                let v210 = staged[4];
                let v212 = parameters[74];
                let v215 = parameters[69];
                let v218 = parameters[96];
                let v221 = parameters[53];
                let v223 = staged[5];
                let v226 = staged[6];
                let v229 = parameters[55];
                let v231 = parameters[100];
                let v234 = parameters[54];
                let v238 = parameters[101];
                let v241 = parameters[56];
                let v243 = parameters[103];
                let v246 = parameters[57];
                let v248 = parameters[58];
                let v250 = parameters[98];
                let v253 = parameters[59];
                let v255 = staged[156];
                let v256 = parameters[121];
                let v259 = parameters[9];
                let v262 = 1e-3f64;
                let v266 = staged[158];
                let v279 = 6.931471805599453e-4f64;
                let v281 = parameters[122];
                let v284 = parameters[10];
                let v290 = parameters[123];
                let v293 = parameters[42];
                let v296 = 0e0f64;
                let v310 = 6.931471805599453e-4f64;
                let v312 = 1e-6f64;
                let v316 = 5e-7f64;
                let v321 = 5e-1f64;
                let v324 = staged[7];
                let v328 = parameters[8];
                let v330 = staged[8];
                let v335 = staged[9];
                let v338 = parameters[11];
                let v340 = staged[10];
                let v343 = parameters[29];
                let v345 = staged[11];
                let v348 = parameters[19];
                let v350 = staged[12];
                let v352 = parameters[20];
                let v356 = staged[13];
                let v359 = parameters[30];
                let v361 = staged[14];
                let v363 = parameters[31];
                let v367 = staged[15];
                let v369 = parameters[16];
                let v372 = parameters[15];
                let v374 = staged[16];
                let v379 = parameters[18];
                let v382 = parameters[17];
                let v387 = staged[161];
                let v388 = staged[17];
                let v392 = parameters[24];
                let v394 = staged[18];
                let v397 = parameters[27];
                let v399 = staged[19];
                let v403 = parameters[25];
                let v408 = staged[20];
                let v411 = parameters[28];
                let v413 = staged[21];
                let v417 = staged[22];
                let v420 = parameters[21];
                let v422 = parameters[22];
                let v426 = staged[23];
                let v429 = parameters[132];
                let v431 = parameters[133];
                let v436 = parameters[138];
                let v438 = parameters[140];
                let v442 = staged[24];
                let v444 = -5e-1f64;
                let v447 = parameters[34];
                let v456 = parameters[33];
                let v460 = staged[25];
                let v467 = staged[26];
                let v469 = -5e-1f64;
                let v472 = parameters[36];
                let v481 = parameters[35];
                let v485 = staged[27];
                let v492 = parameters[95];
                let v495 = parameters[13];
                let v498 = parameters[12];
                let v501 = staged[28];
                let v504 = parameters[85];
                let v506 = staged[29];
                let v510 = staged[30];
                let v513 = parameters[86];
                let v515 = staged[31];
                let v518 = parameters[87];
                let v521 = parameters[88];
                let v523 = staged[32];
                let v525 = staged[33];
                let v528 = parameters[89];
                let v530 = 3e2f64;
                let v532 = 5.25e2f64;
                let v534 = 7.2e-4f64;
                let v537 = 1.6e-6f64;
                let v541 = staged[34];
                let v543 = 1.081e0f64;
                let v546 = parameters[91];
                let v548 = staged[163];
                let v550 = staged[35];
                let v553 = staged[165];
                let v558 = staged[167];
                let v564 = staged[42];
                let v567 = staged[46];
                let v569 = 2e0f64;
                let v573 = staged[48];
                let v577 = staged[50];
                let v579 = 4e0f64;
                let v583 = 1e0f64;
                let v585 = staged[171];
                let v589 = parameters[14];
                let v596 = staged[173];
                let v597 = staged[91];
                let v599 = staged[174];
                let v600 = parameters[32];
                let v608 = staged[106];
                let v610 = parameters[67];
                let v612 = parameters[76];
                let v619 = staged[114];
                let v622 = parameters[84];
                let v628 = staged[176];
                let v630 = staged[177];
                let v632 = staged[124];
                let v634 = staged[126];
                let v2 = v0 + v1;
                let v4 = v2 / v3;
                let v6 = v5 * v2;
                let v8 = v7 / v6;
                let v10 = v8 - v9;
                let v11 = v2 - v3;
                let v12 = v4.ln();
                let v20 = v19 - (((v13 * v2) * v2) / (v2 + v16));
                let v24 = (v20 - v21) / v23;
                let v25 = if v20 < v21 { 1.0 } else { 0.0 };
                let v37: f64;
                if v25 != 0.0 {
                    let v30 = v21 + (v23 * ((v7 + (v24.exp())).ln()));
                    v37 = v30;
                } else {
                    let v36 = v20 + (v23 * ((v7 + ((-v24).exp())).ln()));
                    v37 = v36;
                }
                let v45 = v44 - (((v38 * v2) * v2) / (v2 + v41));
                let v47 = (v45 - v21) / v23;
                let v48 = if v45 < v21 { 1.0 } else { 0.0 };
                let v60: f64;
                if v48 != 0.0 {
                    let v53 = v21 + (v23 * ((v7 + (v47.exp())).ln()));
                    v60 = v53;
                } else {
                    let v59 = v45 + (v23 * ((v7 + ((-v47).exp())).ln()));
                    v60 = v59;
                }
                let v67 = v7 - v4;
                let v70 = (((v61 * v6) * v12) + (v64 * v4)) + (v67 * v68);
                let v72 = (v21 - v70) / v6;
                let v73 = if v21 < v70 { 1.0 } else { 0.0 };
                let v85: f64;
                if v73 != 0.0 {
                    let v78 = v70 + (v6 * ((v7 + (v72.exp())).ln()));
                    v85 = v78;
                } else {
                    let v84 = v21 + (v6 * ((v7 + ((-v72).exp())).ln()));
                    v85 = v84;
                }
                let v93 = v67 * v92;
                let v94 = (((v86 * v6) * v12) + (v89 * v4)) + v93;
                let v96 = (v21 - v94) / v6;
                let v97 = if v21 < v94 { 1.0 } else { 0.0 };
                let v109: f64;
                if v97 != 0.0 {
                    let v102 = v94 + (v6 * ((v7 + (v96.exp())).ln()));
                    v109 = v102;
                } else {
                    let v108 = v21 + (v6 * ((v7 + ((-v96).exp())).ln()));
                    v109 = v108;
                }
                let v116 = (((v110 * v6) * v12) + (v113 * v4)) + v93;
                let v118 = (v21 - v116) / v6;
                let v119 = if v21 < v116 { 1.0 } else { 0.0 };
                let v131: f64;
                if v119 != 0.0 {
                    let v124 = v116 + (v6 * ((v7 + (v118.exp())).ln()));
                    v131 = v124;
                } else {
                    let v130 = v21 + (v6 * ((v7 + ((-v118).exp())).ln()));
                    v131 = v130;
                }
                let v136 = v135 * v4;
                let v138 = (((v132 * v6) * v12) + v136) + v93;
                let v140 = (v21 - v138) / v6;
                let v141 = if v21 < v138 { 1.0 } else { 0.0 };
                let v153: f64;
                if v141 != 0.0 {
                    let v146 = v138 + (v6 * ((v7 + (v140.exp())).ln()));
                    v153 = v146;
                } else {
                    let v152 = v21 + (v6 * ((v7 + ((-v140).exp())).ln()));
                    v153 = v152;
                }
                let v158 = (((v154 * v6) * v12) + v136) + v93;
                let v160 = (v21 - v158) / v6;
                let v161 = if v21 < v158 { 1.0 } else { 0.0 };
                let v173: f64;
                if v161 != 0.0 {
                    let v166 = v158 + (v6 * ((v7 + (v160.exp())).ln()));
                    v173 = v166;
                } else {
                    let v172 = v21 + (v6 * ((v7 + ((-v160).exp())).ln()));
                    v173 = v172;
                }
                let v182 = (((v174 * v6) * v12) + (v177 * v4)) + (v67 * v180);
                let v184 = (v21 - v182) / v6;
                let v185 = if v21 < v182 { 1.0 } else { 0.0 };
                let v197: f64;
                if v185 != 0.0 {
                    let v190 = v182 + (v6 * ((v7 + (v184.exp())).ln()));
                    v197 = v190;
                } else {
                    let v196 = v21 + (v6 * ((v7 + ((-v184).exp())).ln()));
                    v197 = v196;
                }
                let v198 = v7 / v85;
                let v199 = v7 / v173;
                let v202 = (v64 * v198).powf(v201);
                let v205 = (v135 * v199).powf(v204);
                let v207 = v206 * v202;
                let v213 = (v210 * ((v135 / v153).powf(v204))) + v212;
                let v214 = v7 / v213;
                let v216 = v215 * v213;
                let v217 = v212 * v214;
                let v222 = v221 * ((v12 * v218).exp());
                let v224 = if v222 < v223 { 1.0 } else { 0.0 };
                let v225: f64;
                if v224 != 0.0 {
                    v225 = v223;
                } else {
                    v225 = v222;
                }
                let v230 = v229 * ((v12 * v226).exp());
                let v235 = v234 * ((v12 * v231).exp());
                let v236 = if v235 < v223 { 1.0 } else { 0.0 };
                let v237: f64;
                if v236 != 0.0 {
                    v237 = v223;
                } else {
                    v237 = v235;
                }
                let v242 = v241 * ((v12 * v238).exp());
                let v245 = (v12 * v243).exp();
                let v247 = v246 * v245;
                let v249 = v248 * v245;
                let v254 = v253 * ((v12 * v250).exp());
                let v265: f64;
                if v255 != 0.0 {
                    let v260 = v259 * (v7 + (v11 * v256));
                    let v263 = (v260 - v7) / v262;
                    let v264 = if v260 < v7 { 1.0 } else { 0.0 };
                    let v278: f64;
                    if v264 != 0.0 {
                        let v271 = v7 + (v262 * ((v7 + (v263.exp())).ln()));
                        v278 = v271;
                    } else {
                        let v277 = v260 + (v262 * ((v7 + ((-v263).exp())).ln()));
                        v278 = v277;
                    }
                    let v280 = v278 - v279;
                    v265 = v280;
                } else {
                    v265 = v259;
                }
                let v289: f64;
                if v266 != 0.0 {
                    let v285 = v284 * (v7 + (v11 * v281));
                    let v287 = (v285 - v7) / v262;
                    let v288 = if v285 < v7 { 1.0 } else { 0.0 };
                    let v309: f64;
                    if v288 != 0.0 {
                        let v302 = v7 + (v262 * ((v7 + (v287.exp())).ln()));
                        v309 = v302;
                    } else {
                        let v308 = v285 + (v262 * ((v7 + ((-v287).exp())).ln()));
                        v309 = v308;
                    }
                    let v311 = v309 - v310;
                    v289 = v311;
                } else {
                    v289 = v284;
                }
                let v294 = v293 * (v7 + (v290 * v11));
                let v295 = v294 * v294;
                let v297 = if v294 < v296 { 1.0 } else { 0.0 };
                let v323: f64;
                if v297 != 0.0 {
                    let v317 = v316 / (((v295 + v312).sqrt()) - v294);
                    v323 = v317;
                } else {
                    let v322 = v321 * (((v295 + v312).sqrt()) + v294);
                    v323 = v322;
                }
                let v334 = (v328 * (((v12 * v324) / v265).exp())) * (((v330 * v10) / v265).exp());
                let v339 = v338 * ((v12 * v335).exp());
                let v344 = v343 * ((v12 * v340).exp());
                let v351 = v350 * v10;
                let v355 = (v348 * ((v12 * v345).exp())) * ((v351 / v352).exp());
                let v366 = (v359 * ((v12 * v356).exp())) * (((v361 * v10) / v363).exp());
                let v368 = v12 * v367;
                let v375 = v374 * v10;
                let v378 = (v372 * ((v368 / v369).exp())) * ((v375 / v369).exp());
                let v386 = (v382 * ((v368 / v379).exp())) * ((v375 / v379).exp());
                let v405: f64;
                let v406: f64;
                let v407: f64;
                if v387 != 0.0 {
                    let v393 = v392 * (((v388 * v10) / v369).exp());
                    let v398 = v397 * ((v394 * v10).exp());
                    let v404 = v403 * (((v399 * v10) / v379).exp());
                    v405 = v393;
                    v406 = v398;
                    v407 = v404;
                } else {
                    v405 = v296;
                    v406 = v296;
                    v407 = v296;
                }
                let v416 = (v411 * ((v12 * v408).exp())) * ((v413 * v10).exp());
                let v425 = (v420 * ((v12 * v417).exp())) * ((v351 / v422).exp());
                let v434 = (v429 * ((v12 * v426).exp())) * ((v351 / v431).exp());
                let v441 = (v436 * (v4.sqrt())) * ((v438 * v11).exp());
                let v445 = (v37 * v442).powf(v444);
                let v446 = v7 / v202;
                let v455 = (((((((v447 * v37) * v37) * v445) * v446) * v64) * v198) * v442) * v442;
                let v466 = ((((((v456 * v445) * v85) * v85) * v460) * v460) * v202) * ((v447 - v455).exp());
                let v470 = (v60 * v467).powf(v469);
                let v480 = (((((((v472 * v60) * v60) * v470) * (v7 / v205)) * v135) * v199) * v467) * v467;
                let v491 = ((((((v481 * v470) * v173) * v173) * v485) * v485) * v205) * ((v472 - v480).exp());
                let v494 = (v12 * v492).exp();
                let v497 = (v495 * v494) * v214;
                let v500 = (v498 * v494) * v446;
                let v509 = (v504 * ((v12 * v501).exp())) * ((v506 * v10).exp());
                let v514 = v513 * ((v12 * v510).exp());
                let v519 = v518 * ((v12 * v515).exp());
                let v520 = v514 + v519;
                let v524 = (v521 * v520) / v523;
                let v529 = v528 * ((v12 * v525).exp());
                let v531 = v2 - v530;
                let v533 = if v2 < v532 { 1.0 } else { 0.0 };
                let v545: f64;
                if v533 != 0.0 {
                    let v542 = v541 * ((v7 + (v534 * v531)) - ((v537 * v531) * v531));
                    v545 = v542;
                } else {
                    let v544 = v541 * v543;
                    v545 = v544;
                }
                let v547 = v546 * v494;
                let v552: f64;
                if v548 != 0.0 {
                    let v549 = v7 / v242;
                    let v551 = if v549 > v550 { 1.0 } else { 0.0 };
                    let v554: f64;
                    if v551 != 0.0 {
                        v554 = v550;
                    } else {
                        v554 = v549;
                    }
                    v552 = v554;
                } else {
                    v552 = v296;
                }
                let v557: f64;
                if v553 != 0.0 {
                    let v555 = v7 / v247;
                    let v556 = if v555 > v550 { 1.0 } else { 0.0 };
                    let v559: f64;
                    if v556 != 0.0 {
                        v559 = v550;
                    } else {
                        v559 = v555;
                    }
                    v557 = v559;
                } else {
                    v557 = v296;
                }
                let v562: f64;
                if v558 != 0.0 {
                    let v560 = v7 / v249;
                    let v561 = if v560 > v550 { 1.0 } else { 0.0 };
                    let v563: f64;
                    if v561 != 0.0 {
                        v563 = v550;
                    } else {
                        v563 = v560;
                    }
                    v562 = v563;
                } else {
                    v562 = v296;
                }
                let v565 = v85 * v564;
                let v566 = v23 * v85;
                let v568 = v85 / v567;
                let v571 = v7 - v217;
                let v572 = (v569 - v217) / v571;
                let v576 = v153 * (v7 - (v572.powf(v573)));
                let v578 = v153 / v577;
                let v581 = (v579 * v334) / v339;
                let v582 = v7 / v289;
                let v584 = v582 - v583;
                if v585 != 0.0 {
                } else {
                    let v588 = ((v547 * v8).exp()) - v7;
                }
                let v590 = v589 * v334;
                if v387 != 0.0 {
                    let v591 = v405 * v569;
                } else {
                }
                if v387 != 0.0 {
                    let v592 = v407 * v569;
                } else {
                }
                let v593 = v569 * v416;
                let v595 = (v579 * v416) / v344;
                if v596 != 0.0 {
                    let v598 = v597 * v416;
                    if v599 != 0.0 {
                        let v602 = (v600 * v416) * v242;
                        let v606 = v6 * (v569 - ((v602 * v8).ln()));
                    } else {
                    }
                } else {
                }
                let v607 = v569 * v6;
                let v609 = v608 * v207;
                let v611 = v610 * v207;
                let v613 = v612 * v216;
                let v614 = v514 * v339;
                let v615 = v321 * v614;
                let v616 = v23 * v153;
                let v621 = (v509 * v339) * ((v334 / v339).powf(v619));
                let v623 = v622 * v6;
                let v626 = ((v579 * v519) * v6) / v254;
                let v627 = v321 * v626;
                if v628 != 0.0 {
                    let v629 = v524 * v321;
                } else {
                    let v631 = v593 * v529;
                }
                if v630 != 0.0 {
                    if v628 != 0.0 {
                        let v633 = v632 * v524;
                    } else {
                        let v636 = (v634 * v416) * v529;
                    }
                } else {
                }
            [v6, v8, v25, v48, v73, v97, v119, v141, v161, v185, v198, v199, v153, v216, v217, v224, v230, v236, v242, v254, v264, v288, v297, v265, v334, v355, v366, v378, v386, v425, v434, v441, v37, v455, v466, v60, v480, v491, v497, v500, v514, v520, v533, v547, v551, v556, v561, v109, v565, v566, v568, v571, v572, v576, v578, v581, v582, v588, v590, v197, v591, v406, v592, v593, v595, v598, v602, v606, v607, v323, v545, v237, v225, v609, v611, v613, v614, v615, v616, v621, v623, v626, v627, v629, v131, v631, v633, v636, v552, v557, v562, v584]
        };
        self.canonical_staged[39] = produced[0];
        self.canonical_staged[36] = produced[1];
        self.canonical_staged[146] = produced[2];
        self.canonical_staged[147] = produced[3];
        self.canonical_staged[148] = produced[4];
        self.canonical_staged[149] = produced[5];
        self.canonical_staged[150] = produced[6];
        self.canonical_staged[151] = produced[7];
        self.canonical_staged[152] = produced[8];
        self.canonical_staged[153] = produced[9];
        self.canonical_staged[45] = produced[10];
        self.canonical_staged[82] = produced[11];
        self.canonical_staged[41] = produced[12];
        self.canonical_staged[112] = produced[13];
        self.canonical_staged[54] = produced[14];
        self.canonical_staged[154] = produced[15];
        self.canonical_staged[99] = produced[16];
        self.canonical_staged[155] = produced[17];
        self.canonical_staged[94] = produced[18];
        self.canonical_staged[40] = produced[19];
        self.canonical_staged[157] = produced[20];
        self.canonical_staged[159] = produced[21];
        self.canonical_staged[160] = produced[22];
        self.canonical_staged[37] = produced[23];
        self.canonical_staged[62] = produced[24];
        self.canonical_staged[71] = produced[25];
        self.canonical_staged[73] = produced[26];
        self.canonical_staged[65] = produced[27];
        self.canonical_staged[69] = produced[28];
        self.canonical_staged[72] = produced[29];
        self.canonical_staged[74] = produced[30];
        self.canonical_staged[63] = produced[31];
        self.canonical_staged[78] = produced[32];
        self.canonical_staged[77] = produced[33];
        self.canonical_staged[79] = produced[34];
        self.canonical_staged[85] = produced[35];
        self.canonical_staged[84] = produced[36];
        self.canonical_staged[86] = produced[37];
        self.canonical_staged[58] = produced[38];
        self.canonical_staged[57] = produced[39];
        self.canonical_staged[136] = produced[40];
        self.canonical_staged[121] = produced[41];
        self.canonical_staged[162] = produced[42];
        self.canonical_staged[59] = produced[43];
        self.canonical_staged[164] = produced[44];
        self.canonical_staged[166] = produced[45];
        self.canonical_staged[168] = produced[46];
        self.canonical_staged[38] = produced[47];
        self.canonical_staged[43] = produced[48];
        self.canonical_staged[44] = produced[49];
        self.canonical_staged[47] = produced[50];
        self.canonical_staged[53] = produced[51];
        self.canonical_staged[52] = produced[52];
        self.canonical_staged[49] = produced[53];
        self.canonical_staged[51] = produced[54];
        self.canonical_staged[55] = produced[55];
        self.canonical_staged[56] = produced[56];
        self.canonical_staged[60] = produced[57];
        self.canonical_staged[61] = produced[58];
        self.canonical_staged[64] = produced[59];
        self.canonical_staged[66] = produced[60];
        self.canonical_staged[67] = produced[61];
        self.canonical_staged[70] = produced[62];
        self.canonical_staged[88] = produced[63];
        self.canonical_staged[89] = produced[64];
        self.canonical_staged[92] = produced[65];
        self.canonical_staged[95] = produced[66];
        self.canonical_staged[93] = produced[67];
        self.canonical_staged[100] = produced[68];
        self.canonical_staged[101] = produced[69];
        self.canonical_staged[103] = produced[70];
        self.canonical_staged[104] = produced[71];
        self.canonical_staged[105] = produced[72];
        self.canonical_staged[107] = produced[73];
        self.canonical_staged[108] = produced[74];
        self.canonical_staged[109] = produced[75];
        self.canonical_staged[118] = produced[76];
        self.canonical_staged[110] = produced[77];
        self.canonical_staged[111] = produced[78];
        self.canonical_staged[116] = produced[79];
        self.canonical_staged[115] = produced[80];
        self.canonical_staged[119] = produced[81];
        self.canonical_staged[117] = produced[82];
        self.canonical_staged[120] = produced[83];
        self.canonical_staged[122] = produced[84];
        self.canonical_staged[123] = produced[85];
        self.canonical_staged[125] = produced[86];
        self.canonical_staged[127] = produced[87];
        self.canonical_staged[133] = produced[88];
        self.canonical_staged[134] = produced[89];
        self.canonical_staged[135] = produced[90];
        self.canonical_staged[140] = produced[91];
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
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 12033 => 0usize, 12039 => 1usize, 12049 => 2usize, 12055 => 3usize, 12063 => 4usize, 12071 => 5usize, 12091 => 6usize, 12110 => 7usize, 12363 => 8usize, _ => usize::MAX };
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
            let v2 = staged[161];
            let v3 = staged[165];
            let v4 = staged[167];
            let v5 = node_potentials[5];
            let v6 = node_potentials[6];
            let v8 = Lanes([1e0f64; 1]);
            let v10 = Lanes([1e0f64; 1]);
            let v13 = parameters[3];
            let v16 = node_potentials[7];
            let v19 = Lanes([1e0f64; 1]);
            let v24 = node_potentials[3];
            let v27 = Lanes([1e0f64; 1]);
            let v32 = node_potentials[4];
            let v34 = Lanes([1e0f64; 1]);
            let v52 = node_potentials[2];
            let v54 = Lanes([1e0f64; 1]);
            let v60 = node_potentials[1];
            let v62 = Lanes([1e0f64; 1]);
            let v74 = node_potentials[0];
            let v77 = Lanes([1e0f64; 1]);
            let v82 = node_potentials[9];
            let v84 = Lanes([1e0f64; 1]);
            let v90 = node_potentials[8];
            let v92 = Lanes([1e0f64; 1]);
            let v111 = -1e0f64;
            let v128 = staged[36];
            let v131 = parameters[134];
            let v137 = 1e0f64;
            let v145 = staged[37];
            let v194 = staged[38];
            let v244 = 4e0f64;
            let v249 = 2e0f64;
            let v251 = 1e0f64;
            let v261 = 2e0f64;
            let v269 = parameters[136];
            let v271 = Lanes([0e0f64; 2]);
            let v289 = staged[39];
            let v295 = staged[40];
            let v299 = 1e2f64;
            let v308 = 1e-5f64;
            let v312 = 1e-40f64;
            let v332 = staged[43];
            let v334 = staged[44];
            let v347 = 5e-1f64;
            let v364 = 2e-1f64;
            let v394 = parameters[61];
            let v395 = parameters[60];
            let v415 = parameters[62];
            let v442 = -1e0f64;
            let v540 = parameters[135];
            let v542 = Lanes([0e0f64; 3]);
            let v573 = parameters[72];
            let v575 = staged[41];
            let v576 = 1e-1f64;
            let v656 = staged[45];
            let v661 = staged[46];
            let v663 = staged[137];
            let v669 = staged[47];
            let v674 = 3e0f64;
            let v679 = staged[169];
            let v681 = staged[170];
            let v684 = staged[49];
            let v725 = parameters[75];
            let v727 = staged[138];
            let v735 = staged[50];
            let v737 = staged[139];
            let v747 = staged[51];
            let v750 = staged[52];
            let v761 = staged[53];
            let v764 = staged[54];
            let v770 = staged[55];
            let v783 = staged[56];
            let v785 = staged[140];
            let v801 = staged[171];
            let v802 = staged[57];
            let v806 = staged[58];
            let v816 = staged[59];
            let v837 = staged[60];
            let v846 = 1.0000000000000002e-2f64;
            let v854 = 5.000000000000001e-3f64;
            let v881 = staged[61];
            let v884 = staged[62];
            let v895 = 1e-4f64;
            let v921 = parameters[139];
            let v935 = staged[63];
            let v938 = parameters[141];
            let v940 = 1e-3f64;
            let v968 = parameters[142];
            let v980 = parameters[16];
            let v993 = staged[64];
            let v997 = staged[172];
            let v1002 = parameters[18];
            let v1017 = 1e3f64;
            let v1019 = 4e1f64;
            let v1025 = 2.3538526683702e17f64;
            let v1031 = staged[65];
            let v1034 = staged[66];
            let v1062 = staged[67];
            let v1082 = staged[68];
            let v1090 = parameters[92];
            let v1121 = staged[69];
            let v1126 = parameters[20];
            let v1142 = staged[70];
            let v1169 = staged[71];
            let v1172 = parameters[22];
            let v1186 = staged[72];
            let v1189 = parameters[31];
            let v1203 = staged[73];
            let v1206 = parameters[133];
            let v1220 = staged[74];
            let v1223 = staged[75];
            let v1227 = staged[76];
            let v1234 = staged[77];
            let v1238 = Lanes([0e0f64; 2]);
            let v1242 = staged[81];
            let v1258 = 1e-30f64;
            let v1264 = -2e0f64;
            let v1265 = parameters[66];
            let v1283 = 6e0f64;
            let v1301 = 1.6666666666666666e-1f64;
            let v1308 = staged[78];
            let v1315 = -1e-3f64;
            let v1324 = 3.333333333333333e-1f64;
            let v1327 = 2.5e-1f64;
            let v1342 = staged[79];
            let v1356 = staged[80];
            let v1381 = staged[82];
            let v1392 = staged[83];
            let v1399 = staged[84];
            let v1431 = staged[88];
            let v1434 = staged[89];
            let v1447 = staged[173];
            let v1465 = -2e0f64;
            let v1466 = parameters[71];
            let v1507 = staged[85];
            let v1514 = -1e-3f64;
            let v1539 = staged[86];
            let v1553 = staged[87];
            let v1578 = staged[90];
            let v1582 = staged[92];
            let v1597 = staged[174];
            let v1598 = Lanes([0e0f64; 8]);
            let v1605 = staged[175];
            let v1606 = staged[93];
            let v1618 = 1.21e-2f64;
            let v1626 = 6.05e-3f64;
            let v1642 = staged[94];
            let v1645 = staged[95];
            let v1657 = -1e0f64;
            let v1660 = -1e0f64;
            let v1667 = -1e0f64;
            let v1670 = Lanes([0e0f64; 3]);
            let v1706 = 1e-12f64;
            let v1712 = -1e0f64;
            let v1717 = 5e-13f64;
            let v1727 = -1e0f64;
            let v1736 = staged[96];
            let v1738 = parameters[80];
            let v1741 = parameters[81];
            let v1754 = staged[97];
            let v1757 = staged[98];
            let v1761 = 1.0000000000000002e-2f64;
            let v1769 = 5.000000000000001e-3f64;
            let v1789 = staged[99];
            let v1794 = staged[5];
            let v1796 = Lanes([0e0f64; 4]);
            let v1802 = staged[100];
            let v1814 = parameters[38];
            let v1819 = parameters[43];
            let v1827 = parameters[41];
            let v1849 = staged[101];
            let v1851 = parameters[40];
            let v1869 = parameters[39];
            let v1881 = parameters[45];
            let v1883 = parameters[44];
            let v1901 = parameters[7];
            let v1952 = parameters[46];
            let v2021 = 1e-7f64;
            let v2028 = staged[102];
            let v2029 = staged[103];
            let v2088 = parameters[47];
            let v2096 = parameters[48];
            let v2110 = parameters[51];
            let v2115 = parameters[50];
            let v2148 = parameters[49];
            let v2175 = parameters[52];
            let v2179 = staged[104];
            let v2195 = staged[105];
            let v2211 = 1e-6f64;
            let v2259 = staged[107];
            let v2308 = staged[108];
            let v2311 = staged[109];
            let v2314 = staged[110];
            let v2330 = staged[111];
            let v2382 = staged[112];
            let v2385 = staged[113];
            let v2446 = parameters[32];
            let v2449 = staged[115];
            let v2462 = staged[116];
            let v2465 = staged[117];
            let v2476 = staged[176];
            let v2477 = staged[118];
            let v2480 = staged[119];
            let v2485 = staged[120];
            let v2488 = staged[121];
            let v2491 = staged[122];
            let v2493 = parameters[90];
            let v2501 = staged[177];
            let v2511 = staged[123];
            let v2532 = staged[178];
            let v2564 = staged[125];
            let v2587 = staged[127];
            let v2602 = staged[128];
            let v2604 = staged[141];
            let v2610 = Lanes([0e0f64; 5]);
            let v2621 = parameters[1];
            let v2717 = staged[129];
            let v2720 = parameters[94];
            let v2726 = parameters[93];
            let v2731 = staged[130];
            let v2754 = -1e0f64;
            let v2782 = ddt_scale();
            let v2810 = staged[131];
            let v2817 = staged[132];
            let v2830 = staged[133];
            let v2863 = staged[134];
            let v2868 = Lanes([0e0f64; 2]);
            let v2873 = staged[135];
            let v2878 = Lanes([0e0f64; 2]);
            let v2887 = staged[179];
            let v2893 = 0e0f64;
            let v2907 = staged[136];
            let v2916 = staged[180];
            let v2919 = staged[181];
            let v2924 = parameters[131];
            let v2935 = node_potentials[10];
            let v2937 = Lanes([1e0f64; 1]);
            let v3056 = 0e0f64;
            let v3057 = 0e0f64;
            let v3058 = 0e0f64;
            let v3059 = 0e0f64;
            let v3060 = 0e0f64;
            let v3061 = 0e0f64;
            let v3062 = 0e0f64;
            let v3063 = 0e0f64;
            let v3064 = 0e0f64;
            let v3065 = 0e0f64;
            let v3066 = 0e0f64;
            let v3067 = 0e0f64;
            let v3068 = 0e0f64;
            let v3069 = 0e0f64;
            let v3070 = 0e0f64;
            let v1 = ctx.simparam_or("gmin", v0);
            let v14 = v13 * (v5 - v6);
            let v15 = ((Lanes([v8[0], 0.0])) - (Lanes([0.0, v10[0]]))) * v13;
            let v22 = v13 * (v5 - v16);
            let v23 = ((Lanes([v8[0], 0.0])) - (Lanes([0.0, v19[0]]))) * v13;
            let v30 = v13 * (v5 - v24);
            let v31 = ((Lanes([0.0, v8[0]])) - (Lanes([v27[0], 0.0]))) * v13;
            let v38 = v13 * (v32 - v24);
            let v39 = ((Lanes([0.0, v34[0]])) - (Lanes([v27[0], 0.0]))) * v13;
            let v44 = v13 * (v32 - v5);
            let v45 = ((Lanes([v34[0], 0.0])) - (Lanes([0.0, v8[0]]))) * v13;
            let v50 = v13 * (v6 - v16);
            let v51 = ((Lanes([v10[0], 0.0])) - (Lanes([0.0, v19[0]]))) * v13;
            let v58 = v13 * (v52 - v24);
            let v59 = ((Lanes([v54[0], 0.0])) - (Lanes([0.0, v27[0]]))) * v13;
            let v66 = v13 * (v60 - v32);
            let v67 = ((Lanes([v62[0], 0.0])) - (Lanes([0.0, v34[0]]))) * v13;
            let v72 = v13 * (v60 - v52);
            let v73 = ((Lanes([v62[0], 0.0])) - (Lanes([0.0, v54[0]]))) * v13;
            let v80 = v13 * (v60 - v74);
            let v81 = ((Lanes([0.0, v62[0]])) - (Lanes([v77[0], 0.0]))) * v13;
            let v88 = v13 * (v82 - v6);
            let v89 = ((Lanes([0.0, v84[0]])) - (Lanes([v10[0], 0.0]))) * v13;
            let v96 = v13 * (v90 - v82);
            let v97 = ((Lanes([v92[0], 0.0])) - (Lanes([0.0, v84[0]]))) * v13;
            let v101 = (Lanes([v45[0], v45[1], 0.0])) + (Lanes([0.0, v23[0], v23[1]]));
            let v105 = (Lanes([v101[0], v101[1], 0.0, v101[2]])) - (Lanes([0.0, 0.0, v51[0], v51[1]]));
            let v106 = ((v44 + v22) - v50) - v88;
            let v109 = (Lanes([v105[0], v105[1], v105[2], v105[3], 0.0])) - (Lanes([0.0, 0.0, v89[0], 0.0, v89[1]]));
            let v112 = v81 * v111;
            let v116 = (Lanes([v112[0], v112[1], 0.0])) + (Lanes([0.0, v67[0], v67[1]]));
            let v120 = (Lanes([v116[0], v116[1], v116[2], 0.0, 0.0, 0.0, 0.0])) + (Lanes([0.0, 0.0, v109[0], v109[1], v109[2], v109[3], v109[4]]));
            let v121 = (((-v80) + v66) + v106) - v96;
            let v124 = (Lanes([v120[0], v120[1], v120[2], v120[3], v120[4], v120[5], 0.0, v120[6]])) - (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, v97[0], v97[1]]));
            let v125 = v80 + v121;
            let v127 = (Lanes([v81[0], v81[1], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + v124;
            let v129 = v22 * v128;
            let v130 = v23 * v128;
            let v132 = if v129 < v131 { 1.0 } else { 0.0 };
            let v141: f64;
            let v142: Lanes<2>;
            if v132 != 0.0 {
                let v133 = v129.exp();
                let v134 = v130 * v133;
                v141 = v133;
                v142 = v134;
            } else {
                let v135 = v131.exp();
                let v139 = v135 * (v137 + (v129 - v131));
                let v140 = v130 * v135;
                v141 = v139;
                v142 = v140;
            }
            let v143 = v30 * v128;
            let v144 = v31 * v128;
            let v146 = v143 / v145;
            let v147 = v144 / v145;
            let v148 = if v146 < v131 { 1.0 } else { 0.0 };
            let v156: f64;
            let v157: Lanes<2>;
            if v148 != 0.0 {
                let v149 = v146.exp();
                let v150 = v147 * v149;
                v156 = v149;
                v157 = v150;
            } else {
                let v151 = v131.exp();
                let v154 = v151 * (v137 + (v146 - v131));
                let v155 = v147 * v151;
                v156 = v154;
                v157 = v155;
            }
            let v158 = v106 * v128;
            let v159 = v109 * v128;
            let v160 = if v158 < v131 { 1.0 } else { 0.0 };
            let v168: f64;
            let v169: Lanes<5>;
            if v160 != 0.0 {
                let v161 = v158.exp();
                let v162 = v159 * v161;
                v168 = v161;
                v169 = v162;
            } else {
                let v163 = v131.exp();
                let v166 = v163 * (v137 + (v158 - v131));
                let v167 = v159 * v163;
                v168 = v166;
                v169 = v167;
            }
            let v170 = v44 * v128;
            let v171 = v45 * v128;
            let v172 = if v170 < v131 { 1.0 } else { 0.0 };
            let v180: f64;
            let v181: Lanes<2>;
            if v172 != 0.0 {
                let v173 = v170.exp();
                let v174 = v171 * v173;
                v180 = v173;
                v181 = v174;
            } else {
                let v175 = v131.exp();
                let v178 = v175 * (v137 + (v170 - v131));
                let v179 = v171 * v175;
                v180 = v178;
                v181 = v179;
            }
            let v182 = v125 * v128;
            let v183 = v127 * v128;
            let v184 = if v182 < v131 { 1.0 } else { 0.0 };
            let v192: f64;
            let v193: Lanes<8>;
            if v184 != 0.0 {
                let v185 = v182.exp();
                let v186 = v183 * v185;
                v192 = v185;
                v193 = v186;
            } else {
                let v187 = v131.exp();
                let v190 = v187 * (v137 + (v182 - v131));
                let v191 = v183 * v187;
                v192 = v190;
                v193 = v191;
            }
            let v196 = (v125 - v194) * v128;
            let v197 = if v196 < v131 { 1.0 } else { 0.0 };
            let v205: f64;
            let v206: Lanes<8>;
            if v197 != 0.0 {
                let v198 = v196.exp();
                let v199 = v183 * v198;
                v205 = v198;
                v206 = v199;
            } else {
                let v200 = v131.exp();
                let v203 = v200 * (v137 + (v196 - v131));
                let v204 = v183 * v200;
                v205 = v203;
                v206 = v204;
            }
            let v208 = (v106 - v194) * v128;
            let v209 = if v208 < v131 { 1.0 } else { 0.0 };
            let v217: f64;
            let v218: Lanes<5>;
            if v209 != 0.0 {
                let v210 = v208.exp();
                let v211 = v159 * v210;
                v217 = v210;
                v218 = v211;
            } else {
                let v212 = v131.exp();
                let v215 = v212 * (v137 + (v208 - v131));
                let v216 = v159 * v212;
                v217 = v215;
                v218 = v216;
            }
            let v220 = (v22 - v194) * v128;
            let v221 = if v220 < v131 { 1.0 } else { 0.0 };
            let v229: f64;
            let v230: Lanes<2>;
            if v221 != 0.0 {
                let v222 = v220.exp();
                let v223 = v130 * v222;
                v229 = v222;
                v230 = v223;
            } else {
                let v224 = v131.exp();
                let v227 = v224 * (v137 + (v220 - v131));
                let v228 = v130 * v224;
                v229 = v227;
                v230 = v228;
            }
            let v232 = (v14 - v194) * v128;
            let v233 = v15 * v128;
            let v234 = if v232 < v131 { 1.0 } else { 0.0 };
            let v242: f64;
            let v243: Lanes<2>;
            if v234 != 0.0 {
                let v235 = v232.exp();
                let v236 = v233 * v235;
                v242 = v235;
                v243 = v236;
            } else {
                let v237 = v131.exp();
                let v240 = v237 * (v137 + (v232 - v131));
                let v241 = v233 * v237;
                v242 = v240;
                v243 = v241;
            }
            let v248 = (v137 + (v244 * v229)).sqrt();
            let v253 = (v230 * v244) * (v251 / (v249 * v248));
            let v257 = (v137 + (v244 * v242)).sqrt();
            let v260 = (v243 * v244) * (v251 / (v249 * v257));
            let v264 = v137 + v257;
            let v265 = (v261 * v242) / v264;
            let v268 = ((v243 * v261) - (v260 * v265)) / v264;
            let v270 = if v265 < v269 { 1.0 } else { 0.0 };
            let v272: f64;
            let v273: Lanes<2>;
            if v270 != 0.0 {
                v272 = v269;
                v273 = v271;
            } else {
                v272 = v265;
                v273 = v268;
            }
            let v275 = Lanes([v253[0], 0.0, v253[1]]);
            let v278 = v248 + v137;
            let v279 = v278 / v264;
            let v280 = v260 * v279;
            let v290 = v289 * ((v248 - v257) - (v279.ln()));
            let v291 = ((v275 - (Lanes([v260[0], v260[1], 0.0]))) - (((v275 - (Lanes([v280[0], v280[1], 0.0]))) / v264) * (v251 / v279))) * v289;
            let v293 = Lanes([0.0, v51[0], v51[1]]);
            let v296 = (v290 + v50) / v295;
            let v297 = (v291 + v293) / v295;
            let v298 = if v296 > v0 { 1.0 } else { 0.0 };
            let v318: f64;
            let v319: f64;
            let v320: f64;
            let v321: f64;
            let v322: f64;
            let v323: f64;
            let v324: f64;
            let v325: Lanes<3>;
            let v326: Lanes<3>;
            let v327: Lanes<3>;
            let v328: Lanes<3>;
            let v329: Lanes<3>;
            let v330: Lanes<3>;
            let v331: Lanes<3>;
            if v298 != 0.0 {
                let v300 = if v14 < v299 { 1.0 } else { 0.0 };
                let v344: f64;
                let v345: Lanes<2>;
                if v300 != 0.0 {
                    v344 = v14;
                    v345 = v15;
                } else {
                    let v339 = v137 + (v14 - v299);
                    let v342 = v15 * (v251 / v339);
                    let v343 = v299 + (v339.ln());
                    v344 = v343;
                    v345 = v342;
                }
                let v346 = v261 * v289;
                let v350 = (v347 * v296) * v295;
                let v351 = (v297 * v347) * v295;
                let v354 = (v350 * v128) + v137;
                let v361 = (v194 + (v346 * (v354.ln()))) - v344;
                let v363 = (((v351 * v128) * (v251 / v354)) * v346) - (Lanes([v345[0], v345[1], 0.0]));
                let v365 = v364 * v194;
                let v366 = v365 * v365;
                let v367 = v361 * v361;
                let v368 = v363 * v361;
                let v369 = v368 + v368;
                let v370 = if v361 < v0 { 1.0 } else { 0.0 };
                let v392: f64;
                let v393: Lanes<3>;
                if v370 != 0.0 {
                    let v373 = (v367 + v366).sqrt();
                    let v377 = v373 - v361;
                    let v379 = (v347 * v366) / v377;
                    let v382 = ((((v369 * (v251 / (v249 * v373))) - v363) * v379) * v111) / v377;
                    v392 = v379;
                    v393 = v382;
                } else {
                    let v384 = (v367 + v366).sqrt();
                    let v390 = v347 * (v384 + v361);
                    let v391 = ((v369 * (v251 / (v249 * v384))) + v363) * v347;
                    v392 = v390;
                    v393 = v391;
                }
                let v396 = v394 * v395;
                let v397 = v392 + v396;
                let v404 = v395 * (v392 + (v394 * v295));
                let v406 = (v392 * v397) / v404;
                let v409 = (((v393 * v397) + (v393 * v392)) - ((v393 * v395) * v406)) / v404;
                let v410 = v296 / v406;
                let v413 = (v297 - (v409 * v410)) / v406;
                let v416 = (v410 - v137) / v415;
                let v417 = v413 / v415;
                let v418 = if v410 < v137 { 1.0 } else { 0.0 };
                let v440: f64;
                let v441: Lanes<3>;
                if v418 != 0.0 {
                    let v419 = v416.exp();
                    let v421 = v137 + v419;
                    let v426 = ((v417 * v419) * (v251 / v421)) * v415;
                    let v427 = v137 + (v415 * (v421.ln()));
                    v440 = v427;
                    v441 = v426;
                } else {
                    let v430 = (-v416).exp();
                    let v432 = v137 + v430;
                    let v438 = v410 + (v415 * (v432.ln()));
                    let v439 = v413 + ((((v417 * v111) * v430) * (v251 / v432)) * v415);
                    v440 = v438;
                    v441 = v439;
                }
                let v448 = v137 + (v415 * ((v137 + ((v442 / v415).exp())).ln()));
                let v449 = v440 / v448;
                let v450 = v441 / v448;
                let v451 = v392 / v396;
                let v452 = v393 / v396;
                let v453 = v244 * v449;
                let v455 = v453 * v451;
                let v459 = v137 + v451;
                let v465 = (v137 + (v455 * v459)).sqrt();
                let v470 = v261 * v449;
                let v472 = v470 * v459;
                let v476 = (v137 + v465) / v472;
                let v479 = (((((((v450 * v244) * v451) + (v452 * v453)) * v459) + (v452 * v455)) * (v251 / (v249 * v465))) - ((((v450 * v261) * v459) + (v452 * v470)) * v476)) / v472;
                let v482 = v272 * v476;
                let v483 = v273 * v476;
                let v486 = (Lanes([v483[0], v483[1], 0.0])) + (v479 * v272);
                let v489 = v137 + v482;
                let v490 = ((v137 - v476) + v482) / v489;
                let v493 = (((v479 * v111) + v486) - (v486 * v490)) / v489;
                let v498 = (v350 * v490) * v128;
                let v499 = ((v351 * v490) + (v493 * v350)) * v128;
                let v505 = (v272 + v498) + v137;
                let v507 = v273 * v505;
                let v511 = (v261 * v498) + (v272 * v505);
                let v512 = (v499 * v261) + ((Lanes([v507[0], v507[1], 0.0])) + (((Lanes([v273[0], v273[1], 0.0])) + v499) * v272));
                let v514 = v347 * (v498 - v137);
                let v515 = v499 * v347;
                let v517 = v515 * v514;
                let v519 = (v514 * v514) + v511;
                let v520 = (v517 + v517) + v512;
                let v521 = if v498 >= v137 { 1.0 } else { 0.0 };
                let v538: f64;
                let v539: Lanes<3>;
                if v521 != 0.0 {
                    let v522 = v519.sqrt();
                    let v526 = v514 + v522;
                    let v527 = v515 + (v520 * (v251 / (v249 * v522)));
                    v538 = v526;
                    v539 = v527;
                } else {
                    let v528 = v519.sqrt();
                    let v532 = v528 - v514;
                    let v534 = v511 / v532;
                    let v537 = (v512 - (((v520 * (v251 / (v249 * v528))) - v515) * v534)) / v532;
                    v538 = v534;
                    v539 = v537;
                }
                let v541 = if v538 < v540 { 1.0 } else { 0.0 };
                let v543: f64;
                let v544: Lanes<3>;
                if v541 != 0.0 {
                    v543 = v540;
                    v544 = v542;
                } else {
                    v543 = v538;
                    v544 = v539;
                }
                let v545 = v543 + v137;
                let v551 = (v194 * v128).exp();
                let v552 = (v543 * v545) * v551;
                let v553 = ((v544 * v545) + (v544 * v543)) * v551;
                let v554 = v347 * v395;
                let v556 = v554 * (v296 - v394);
                let v557 = v297 * v554;
                let v559 = (v395 * v295) * v394;
                let v563 = v557 * v556;
                let v567 = ((v556 * v556) + (v559 * v296)).sqrt();
                let v571 = v556 + v567;
                let v572 = v557 + (((v563 + v563) + (v297 * v559)) * (v251 / (v249 * v567)));
                let v574 = if v573 == v0 { 1.0 } else { 0.0 };
                let v589: f64;
                let v590: Lanes<3>;
                if v574 != 0.0 {
                    let v577 = v575 * v576;
                    v589 = v577;
                    v590 = v542;
                } else {
                    let v580 = v296 + v406;
                    let v582 = (v261 * v296) / v580;
                    let v587 = v575 * (v576 + v582);
                    let v588 = (((v297 * v261) - ((v297 + v409) * v582)) / v580) * v575;
                    v589 = v587;
                    v590 = v588;
                }
                let v593 = v394 + v296;
                let v594 = (v394 * v296) / v593;
                let v597 = ((v297 * v394) - (v297 * v594)) / v593;
                let v598 = v394 / v593;
                let v601 = ((v297 * v598) * v111) / v593;
                v318 = v571;
                v319 = v589;
                v320 = v598;
                v321 = v552;
                v322 = v490;
                v323 = v594;
                v324 = v543;
                v325 = v572;
                v326 = v590;
                v327 = v601;
                v328 = v553;
                v329 = v493;
                v330 = v597;
                v331 = v544;
            } else {
                let v303 = (v261 * v229) / v278;
                let v306 = ((v230 * v261) - (v253 * v303)) / v278;
                let v317 = if (if (v50.abs()) < (v308 * v289) { 1.0 } else { 0.0 }) != 0.0 || (if (v290.abs()) < ((v312 * v289) * (v248 + v257)) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v623: f64;
                let v624: Lanes<3>;
                if v317 != 0.0 {
                    let v606 = v347 * (v303 + v272);
                    let v607 = ((Lanes([v306[0], 0.0, v306[1]])) + (Lanes([v273[0], v273[1], 0.0]))) * v347;
                    let v608 = v606 + v137;
                    let v609 = v606 / v608;
                    let v612 = (v607 - (v607 * v609)) / v608;
                    v623 = v609;
                    v624 = v612;
                } else {
                    let v616 = (v290 + v22) - v14;
                    let v619 = v290 / v616;
                    let v622 = (v291 - (((v291 + (Lanes([v23[0], 0.0, v23[1]]))) - (Lanes([v15[0], v15[1], 0.0]))) * v619)) / v616;
                    v623 = v619;
                    v624 = v622;
                }
                let v625 = v576 * v575;
                let v628 = v137 - (v296 / v394);
                let v629 = (v297 / v394) * v111;
                let v630 = Lanes([v142[0], 0.0, v142[1]]);
                let v631 = Lanes([v306[0], 0.0, v306[1]]);
                v318 = v50;
                v319 = v625;
                v320 = v628;
                v321 = v141;
                v322 = v623;
                v323 = v296;
                v324 = v303;
                v325 = v293;
                v326 = v542;
                v327 = v629;
                v328 = v630;
                v329 = v624;
                v330 = v297;
                v331 = v631;
            }
            let v335 = (v30 - v332) / v334;
            let v336 = v31 / v334;
            let v337 = if v30 < v332 { 1.0 } else { 0.0 };
            let v654: f64;
            let v655: Lanes<2>;
            if v337 != 0.0 {
                let v632 = v335.exp();
                let v634 = v137 + v632;
                let v640 = v30 - (v334 * (v634.ln()));
                let v641 = v31 - (((v336 * v632) * (v251 / v634)) * v334);
                v654 = v640;
                v655 = v641;
            } else {
                let v644 = (-v335).exp();
                let v646 = v137 + v644;
                let v652 = v332 - (v334 * (v646.ln()));
                let v653 = ((((v336 * v111) * v644) * (v251 / v646)) * v334) * v111;
                v654 = v652;
                v655 = v653;
            }
            let v659 = v137 - (v654 * v656);
            let v660 = (v655 * v656) * v111;
            let v662 = v659.powf(v661);
            let v666 = v660 * (v661 * (v659.powf(v663)));
            let v677 = (v669 * (v137 - v662)) + (v674 * (v30 - v654));
            let v678 = ((v666 * v111) * v669) + ((v31 - v655) * v674);
            let v682: f64;
            let v683: Lanes<3>;
            if v679 != 0.0 {
                let v680 = Lanes([v15[0], v15[1], 0.0]);
                v682 = v14;
                v683 = v680;
            } else {
                let v695: f64;
                let v696: Lanes<3>;
                if v681 != 0.0 {
                    let v691 = v14 + v318;
                    let v693 = (Lanes([v15[0], v15[1], 0.0])) + v325;
                    v695 = v691;
                    v696 = v693;
                } else {
                    let v694 = Lanes([v23[0], 0.0, v23[1]]);
                    v695 = v22;
                    v696 = v694;
                }
                v682 = v695;
                v683 = v696;
            }
            let v686 = (v682 - v684) / v319;
            let v689 = (v683 - (v326 * v686)) / v319;
            let v690 = if v682 < v684 { 1.0 } else { 0.0 };
            let v723: f64;
            let v724: Lanes<3>;
            if v690 != 0.0 {
                let v697 = v686.exp();
                let v699 = v137 + v697;
                let v700 = v699.ln();
                let v707 = v682 - (v319 * v700);
                let v708 = v683 - ((v326 * v700) + (((v689 * v697) * (v251 / v699)) * v319));
                v723 = v707;
                v724 = v708;
            } else {
                let v711 = (-v686).exp();
                let v713 = v137 + v711;
                let v714 = v713.ln();
                let v721 = v684 - (v319 * v714);
                let v722 = ((v326 * v714) + ((((v689 * v111) * v711) * (v251 / v713)) * v319)) * v111;
                v723 = v721;
                v724 = v722;
            }
            let v726 = v320.powf(v725);
            let v730 = v327 * (v725 * (v320.powf(v727)));
            let v733 = v137 - (v723 / v575);
            let v736 = v733.powf(v735);
            let v751 = v726 * v750;
            let v753 = v682 - v723;
            let v766 = v15 * v764;
            let v767 = (v761 * ((v747 * (v137 - (v726 * v736))) + (v751 * v753))) + (v764 * v14);
            let v769 = ((((((v730 * v736) + ((((v724 / v575) * v111) * (v735 * (v733.powf(v737)))) * v726)) * v111) * v747) + (((v730 * v750) * v753) + ((v683 - v724) * v751))) * v761) + (Lanes([v766[0], v766[1], 0.0]));
            let v771 = v770 * v156;
            let v772 = v157 * v770;
            let v774 = (v137 + v771).sqrt();
            let v777 = v772 * (v251 / (v249 * v774));
            let v778 = v137 + v774;
            let v779 = v771 / v778;
            let v782 = (v772 - (v777 * v779)) / v778;
            let v784 = v321.powf(v783);
            let v788 = v328 * (v783 * (v321.powf(v785)));
            let v789 = v770 * v784;
            let v790 = v788 * v770;
            let v792 = (v137 + v789).sqrt();
            let v796 = v137 + v792;
            let v797 = v789 / v796;
            let v800 = (v790 - ((v790 * (v251 / (v249 * v792))) * v797)) / v796;
            let v840: f64;
            let v841: Lanes<4>;
            if v801 != 0.0 {
                let v804 = v678 / v802;
                let v808 = v769 / v806;
                let v809 = (v137 + (v677 / v802)) + (v767 / v806);
                let v812 = (Lanes([v804[0], v804[1], 0.0, 0.0])) + (Lanes([0.0, v808[0], v808[1], v808[2]]));
                v840 = v809;
                v841 = v812;
            } else {
                let v829 = ((((v677 / v802) + v137) * v816) * v128).exp();
                let v830 = (((v678 / v802) * v816) * v128) * v829;
                let v831 = ((((-v767) / v806) * v816) * v128).exp();
                let v832 = ((((v769 * v111) / v806) * v816) * v128) * v831;
                let v838 = (v829 - v831) / v837;
                let v839 = ((Lanes([v830[0], v830[1], 0.0, 0.0])) - (Lanes([0.0, v832[0], v832[1], v832[2]]))) / v837;
                v840 = v838;
                v841 = v839;
            }
            let v842 = v840 * v840;
            let v843 = v841 * v840;
            let v844 = v843 + v843;
            let v845 = if v840 < v0 { 1.0 } else { 0.0 };
            let v868: f64;
            let v869: Lanes<4>;
            if v845 != 0.0 {
                let v848 = (v842 + v846).sqrt();
                let v852 = v848 - v840;
                let v855 = v854 / v852;
                let v858 = ((((v844 * (v251 / (v249 * v848))) - v841) * v855) * v111) / v852;
                v868 = v855;
                v869 = v858;
            } else {
                let v860 = (v842 + v846).sqrt();
                let v866 = v347 * (v860 + v840);
                let v867 = ((v844 * (v251 / (v249 * v860))) + v841) * v347;
                v868 = v866;
                v869 = v867;
            }
            let v875 = ((Lanes([v782[0], v782[1], 0.0, 0.0])) + (Lanes([0.0, v800[0], v800[1], v800[2]]))) * v347;
            let v876 = v137 + (v347 * (v779 + v797));
            let v877 = v868 * v876;
            let v880 = (v869 * v876) + (v875 * v868);
            let v882 = v881 * v784;
            let v883 = v788 * v881;
            let v885 = v884 * v156;
            let v886 = v157 * v884;
            let v888 = Lanes([v886[0], v886[1], 0.0, 0.0]);
            let v889 = Lanes([0.0, v883[0], v883[1], v883[2]]);
            let v891 = (v885 - v882) / v877;
            let v894 = ((v888 - v889) - (v880 * v891)) / v877;
            let v896 = v30 / v895;
            let v897 = v31 / v895;
            let v898 = if v30 < v0 { 1.0 } else { 0.0 };
            let v919: f64;
            let v920: Lanes<2>;
            if v898 != 0.0 {
                let v899 = v896.exp();
                let v901 = v137 + v899;
                let v905 = v895 * (v901.ln());
                let v906 = ((v897 * v899) * (v251 / v901)) * v895;
                v919 = v905;
                v920 = v906;
            } else {
                let v909 = (-v896).exp();
                let v911 = v137 + v909;
                let v917 = v30 + (v895 * (v911.ln()));
                let v918 = v31 + ((((v897 * v111) * v909) * (v251 / v911)) * v895);
                v919 = v917;
                v920 = v918;
            }
            let v922 = v919 / v921;
            let v923 = v920 / v921;
            let v924 = if v922 < v131 { 1.0 } else { 0.0 };
            let v932: f64;
            let v933: Lanes<2>;
            if v924 != 0.0 {
                let v925 = v922.exp();
                let v926 = v923 * v925;
                v932 = v925;
                v933 = v926;
            } else {
                let v927 = v131.exp();
                let v930 = v927 * (v137 + (v922 - v131));
                let v931 = v923 * v927;
                v932 = v930;
                v933 = v931;
            }
            let v936 = v935 * (v932 - v137);
            let v937 = v933 * v935;
            let v941 = (v30 - v938) / v940;
            let v942 = v31 / v940;
            let v943 = if v30 < v938 { 1.0 } else { 0.0 };
            let v966: f64;
            let v967: Lanes<2>;
            if v943 != 0.0 {
                let v944 = v941.exp();
                let v946 = v137 + v944;
                let v952 = v30 - (v940 * (v946.ln()));
                let v953 = v31 - (((v942 * v944) * (v251 / v946)) * v940);
                v966 = v952;
                v967 = v953;
            } else {
                let v956 = (-v941).exp();
                let v958 = v137 + v956;
                let v964 = v938 - (v940 * (v958.ln()));
                let v965 = ((((v942 * v111) * v956) * (v251 / v958)) * v940) * v111;
                v966 = v964;
                v967 = v965;
            }
            let v969 = v968 * v966;
            let v971 = v938 - v966;
            let v973 = v971 * v971;
            let v976 = v969 * v973;
            let v979 = ((v967 * v968) * v973) + (((v967 * v111) * (v261 * v971)) * v969);
            let v981 = v143 / v980;
            let v982 = v144 / v980;
            let v983 = if v981 < v131 { 1.0 } else { 0.0 };
            let v991: f64;
            let v992: Lanes<2>;
            if v983 != 0.0 {
                let v984 = v981.exp();
                let v985 = v982 * v984;
                v991 = v984;
                v992 = v985;
            } else {
                let v986 = v131.exp();
                let v989 = v986 * (v137 + (v981 - v131));
                let v990 = v982 * v986;
                v991 = v989;
                v992 = v990;
            }
            let v998: f64;
            let v999: Lanes<4>;
            if v2 != 0.0 {
                let v995 = (v30 - v993) * v128;
                let v996 = if v995 < v131 { 1.0 } else { 0.0 };
                let v1013: f64;
                let v1014: Lanes<2>;
                if v996 != 0.0 {
                    let v1006 = v995.exp();
                    let v1007 = v144 * v1006;
                    v1013 = v1006;
                    v1014 = v1007;
                } else {
                    let v1008 = v131.exp();
                    let v1011 = v1008 * (v137 + (v995 - v131));
                    let v1012 = v144 * v1008;
                    v1013 = v1011;
                    v1014 = v1012;
                }
                let v1016 = v894 / v884;
                let v1018 = (v891 / v884) - v1017;
                let v1020 = if v1018 < v1019 { 1.0 } else { 0.0 };
                let v1028: f64;
                let v1029: Lanes<4>;
                if v1020 != 0.0 {
                    let v1021 = v1018.exp();
                    let v1022 = v1016 * v1021;
                    v1028 = v1021;
                    v1029 = v1022;
                } else {
                    let v1026 = v1025 * (v137 + (v1018 - v1019));
                    let v1027 = v1016 * v1025;
                    v1028 = v1026;
                    v1029 = v1027;
                }
                let v1030 = v991 - v137;
                let v1033 = v992 * v1031;
                let v1040 = (v137 + (v244 * v1013)).sqrt();
                let v1044 = v137 + v1040;
                let v1045 = (v1034 * v1030) / v1044;
                let v1051 = v137 + (v767 / v806);
                let v1053 = (((v992 * v1034) - (((v1014 * v244) * (v251 / (v249 * v1040))) * v1045)) / v1044) * v1051;
                let v1054 = (v769 / v806) * v1045;
                let v1063 = v1062 * (v321 - v137);
                let v1066 = (v328 * v1062) * v1028;
                let v1070 = v137 + v1028;
                let v1071 = (v1063 * v1028) / v1070;
                let v1075 = ((v1031 * v1030) + (v1045 * v1051)) + v1071;
                let v1076 = ((Lanes([v1033[0], v1033[1], 0.0, 0.0])) + ((Lanes([v1053[0], v1053[1], 0.0, 0.0])) + (Lanes([0.0, v1054[0], v1054[1], v1054[2]])))) + ((((Lanes([0.0, v1066[0], v1066[1], v1066[2]])) + (v1029 * v1063)) - (v1029 * v1071)) / v1070);
                v998 = v1075;
                v999 = v1076;
            } else {
                let v1106: f64;
                let v1107: Lanes<4>;
                if v997 != 0.0 {
                    let v1078 = v1031 * (v991 - v137);
                    let v1079 = v992 * v1031;
                    let v1080 = Lanes([v1079[0], v1079[1], 0.0, 0.0]);
                    v1106 = v1078;
                    v1107 = v1080;
                } else {
                    let v1084 = v992 * v1082;
                    let v1091 = v1090 * ((v991 + v321) - v261);
                    let v1095 = v137 + (v767 / v806);
                    let v1098 = (v769 / v806) * v1091;
                    let v1104 = v1031 * ((v1082 * (v991 - v137)) + (v1091 * v1095));
                    let v1105 = ((Lanes([v1084[0], v1084[1], 0.0, 0.0])) + (((((Lanes([v992[0], v992[1], 0.0, 0.0])) + (Lanes([0.0, v328[0], v328[1], v328[2]]))) * v1090) * v1095) + (Lanes([0.0, v1098[0], v1098[1], v1098[2]])))) * v1031;
                    v1106 = v1104;
                    v1107 = v1105;
                }
                v998 = v1106;
                v999 = v1107;
            }
            let v1000 = v38 * v128;
            let v1001 = v39 * v128;
            let v1003 = v1000 / v1002;
            let v1004 = v1001 / v1002;
            let v1005 = if v1003 < v131 { 1.0 } else { 0.0 };
            let v1115: f64;
            let v1116: Lanes<2>;
            if v1005 != 0.0 {
                let v1108 = v1003.exp();
                let v1109 = v1004 * v1108;
                v1115 = v1108;
                v1116 = v1109;
            } else {
                let v1110 = v131.exp();
                let v1113 = v1110 * (v137 + (v1003 - v131));
                let v1114 = v1004 * v1110;
                v1115 = v1113;
                v1116 = v1114;
            }
            let v1124: f64;
            let v1125: Lanes<2>;
            if v2 != 0.0 {
                let v1118 = (v38 - v993) * v128;
                let v1119 = if v1118 < v131 { 1.0 } else { 0.0 };
                let v1137: f64;
                let v1138: Lanes<2>;
                if v1119 != 0.0 {
                    let v1130 = v1118.exp();
                    let v1131 = v1001 * v1130;
                    v1137 = v1130;
                    v1138 = v1131;
                } else {
                    let v1132 = v131.exp();
                    let v1135 = v1132 * (v137 + (v1118 - v131));
                    let v1136 = v1001 * v1132;
                    v1137 = v1135;
                    v1138 = v1136;
                }
                let v1139 = v1115 - v137;
                let v1148 = (v137 + (v244 * v1137)).sqrt();
                let v1152 = v137 + v1148;
                let v1153 = (v1142 * v1139) / v1152;
                let v1157 = (v1121 * v1139) + v1153;
                let v1158 = (v1116 * v1121) + (((v1116 * v1142) - (((v1138 * v244) * (v251 / (v249 * v1148))) * v1153)) / v1152);
                v1124 = v1157;
                v1125 = v1158;
            } else {
                let v1122 = v1121 * (v1115 - v137);
                let v1123 = v1116 * v1121;
                v1124 = v1122;
                v1125 = v1123;
            }
            let v1127 = v143 / v1126;
            let v1128 = v144 / v1126;
            let v1129 = if v1127 < v131 { 1.0 } else { 0.0 };
            let v1166: f64;
            let v1167: Lanes<2>;
            if v1129 != 0.0 {
                let v1159 = v1127.exp();
                let v1160 = v1128 * v1159;
                v1166 = v1159;
                v1167 = v1160;
            } else {
                let v1161 = v131.exp();
                let v1164 = v1161 * (v137 + (v1127 - v131));
                let v1165 = v1128 * v1161;
                v1166 = v1164;
                v1167 = v1165;
            }
            let v1170 = v1169 * (v1166 - v137);
            let v1171 = v1167 * v1169;
            let v1173 = v1000 / v1172;
            let v1174 = v1001 / v1172;
            let v1175 = if v1173 < v131 { 1.0 } else { 0.0 };
            let v1183: f64;
            let v1184: Lanes<2>;
            if v1175 != 0.0 {
                let v1176 = v1173.exp();
                let v1177 = v1174 * v1176;
                v1183 = v1176;
                v1184 = v1177;
            } else {
                let v1178 = v131.exp();
                let v1181 = v1178 * (v137 + (v1173 - v131));
                let v1182 = v1174 * v1178;
                v1183 = v1181;
                v1184 = v1182;
            }
            let v1187 = v1186 * (v1183 - v137);
            let v1188 = v1184 * v1186;
            let v1190 = v158 / v1189;
            let v1191 = v159 / v1189;
            let v1192 = if v1190 < v131 { 1.0 } else { 0.0 };
            let v1200: f64;
            let v1201: Lanes<5>;
            if v1192 != 0.0 {
                let v1193 = v1190.exp();
                let v1194 = v1191 * v1193;
                v1200 = v1193;
                v1201 = v1194;
            } else {
                let v1195 = v131.exp();
                let v1198 = v1195 * (v137 + (v1190 - v131));
                let v1199 = v1191 * v1195;
                v1200 = v1198;
                v1201 = v1199;
            }
            let v1204 = v1203 * (v1200 - v137);
            let v1205 = v1201 * v1203;
            let v1207 = v1000 / v1206;
            let v1208 = v1001 / v1206;
            let v1209 = if v1207 < v131 { 1.0 } else { 0.0 };
            let v1217: f64;
            let v1218: Lanes<2>;
            if v1209 != 0.0 {
                let v1210 = v1207.exp();
                let v1211 = v1208 * v1210;
                v1217 = v1210;
                v1218 = v1211;
            } else {
                let v1212 = v131.exp();
                let v1215 = v1212 * (v137 + (v1207 - v131));
                let v1216 = v1208 * v1212;
                v1217 = v1215;
                v1218 = v1216;
            }
            let v1221 = v1220 * (v1217 - v137);
            let v1222 = v1218 * v1220;
            let v1224 = if v1223 != 0.0 && v898 != 0.0 { 1.0 } else { 0.0 };
            let v1239: f64;
            let v1240: Lanes<2>;
            if v1224 != 0.0 {
                let v1225 = v261 * v662;
                let v1228 = v1227 / v1225;
                let v1235 = v1234 * (v137 - v1228);
                let v1236 = (((((v666 * v261) * v1228) * v111) / v1225) * v111) * v1234;
                let v1237 = if v1235 < v131 { 1.0 } else { 0.0 };
                let v1251: f64;
                let v1252: Lanes<2>;
                if v1237 != 0.0 {
                    let v1244 = v1235.exp();
                    let v1245 = v1236 * v1244;
                    v1251 = v1244;
                    v1252 = v1245;
                } else {
                    let v1246 = v131.exp();
                    let v1249 = v1246 * (v137 + (v1235 - v131));
                    let v1250 = v1236 * v1246;
                    v1251 = v1249;
                    v1252 = v1250;
                }
                let v1253 = v30 * v656;
                let v1254 = v31 * v656;
                let v1256 = v1254 * v1253;
                let v1260 = ((v1253 * v1253) + v1258).sqrt();
                let v1266 = v1264 - v1265;
                let v1267 = v1260.powf(v1266);
                let v1276 = v1265 - v137;
                let v1284 = v1283 * v1253;
                let v1286 = v1284 * v1253;
                let v1290 = v1276 + v1253;
                let v1295 = (v1265 * ((v137 - (v1265 * v1265)) - ((v674 * v1253) * v1276))) - (v1286 * v1290);
                let v1309 = v1308 * ((v1267 * v1295) * v1301);
                let v1311 = ((v30 * v1227) * v1234) / v1309;
                let v1314 = (((v31 * v1227) * v1234) - ((((((((v1256 + v1256) * (v251 / (v249 * v1260))) * (v1266 * (v1260.powf((v1266 - v251))))) * v1295) + ((((((v1254 * v674) * v1276) * v111) * v1265) - (((((v1254 * v1283) * v1253) + (v1254 * v1284)) * v1290) + (v1254 * v1286))) * v1267)) * v1301) * v1308) * v1311)) / v1309;
                let v1316 = if v1311 < v1315 { 1.0 } else { 0.0 };
                let v1340: f64;
                let v1341: Lanes<2>;
                if v1316 != 0.0 {
                    let v1317 = if v1311 < v131 { 1.0 } else { 0.0 };
                    let v1366: f64;
                    let v1367: Lanes<2>;
                    if v1317 != 0.0 {
                        let v1359 = v1311.exp();
                        let v1360 = v1314 * v1359;
                        v1366 = v1359;
                        v1367 = v1360;
                    } else {
                        let v1361 = v131.exp();
                        let v1364 = v1361 * (v137 + (v1311 - v131));
                        let v1365 = v1314 * v1361;
                        v1366 = v1364;
                        v1367 = v1365;
                    }
                    let v1368 = -v30;
                    let v1372 = (v137 - v1366) / v1311;
                    let v1376 = v137 + v1372;
                    let v1377 = v1368 * v1376;
                    let v1380 = ((v31 * v111) * v1376) + ((((v1367 * v111) - (v1314 * v1372)) / v1311) * v1368);
                    v1340 = v1377;
                    v1341 = v1380;
                } else {
                    let v1318 = v30 * v347;
                    let v1320 = v1318 * v1311;
                    let v1325 = v1311 * v1324;
                    let v1330 = v137 + (v1327 * v1311);
                    let v1335 = v137 + (v1325 * v1330);
                    let v1336 = v1320 * v1335;
                    let v1339 = ((((v31 * v347) * v1311) + (v1314 * v1318)) * v1335) + ((((v1314 * v1324) * v1330) + ((v1314 * v1327) * v1325)) * v1320);
                    v1340 = v1336;
                    v1341 = v1339;
                }
                let v1343 = v261 * v1342;
                let v1344 = v1343 * v1340;
                let v1346 = v1344 * v662;
                let v1357 = ((v1346 * v1251) * v656) * v1356;
                let v1358 = ((((((v1341 * v1343) * v662) + (v666 * v1344)) * v1251) + (v1252 * v1346)) * v656) * v1356;
                v1239 = v1357;
                v1240 = v1358;
            } else {
                v1239 = v0;
                v1240 = v1238;
            }
            let v1243 = if v1242 != 0.0 && (if v14 < v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v1403: f64;
            let v1404: Lanes<2>;
            if v1243 != 0.0 {
                let v1382 = v14 * v1381;
                let v1383 = v15 * v1381;
                let v1384 = v137 - v1382;
                let v1386 = v1384.powf(v735);
                let v1389 = (v1383 * v111) * (v735 * (v1384.powf(v737)));
                let v1390 = v261 * v1386;
                let v1393 = v1392 / v1390;
                let v1400 = v1399 * (v137 - v1393);
                let v1401 = (((((v1389 * v261) * v1393) * v111) / v1390) * v111) * v1399;
                let v1402 = if v1400 < v131 { 1.0 } else { 0.0 };
                let v1455: f64;
                let v1456: Lanes<2>;
                if v1402 != 0.0 {
                    let v1448 = v1400.exp();
                    let v1449 = v1401 * v1448;
                    v1455 = v1448;
                    v1456 = v1449;
                } else {
                    let v1450 = v131.exp();
                    let v1453 = v1450 * (v137 + (v1400 - v131));
                    let v1454 = v1401 * v1450;
                    v1455 = v1453;
                    v1456 = v1454;
                }
                let v1458 = v1383 * v1382;
                let v1461 = ((v1382 * v1382) + v1258).sqrt();
                let v1467 = v1465 - v1466;
                let v1468 = v1461.powf(v1467);
                let v1477 = v1466 - v137;
                let v1484 = v1283 * v1382;
                let v1486 = v1484 * v1382;
                let v1490 = v1477 + v1382;
                let v1495 = (v1466 * ((v137 - (v1466 * v1466)) - ((v674 * v1382) * v1477))) - (v1486 * v1490);
                let v1508 = v1507 * ((v1468 * v1495) * v1301);
                let v1510 = ((v14 * v1392) * v1399) / v1508;
                let v1513 = (((v15 * v1392) * v1399) - ((((((((v1458 + v1458) * (v251 / (v249 * v1461))) * (v1467 * (v1461.powf((v1467 - v251))))) * v1495) + ((((((v1383 * v674) * v1477) * v111) * v1466) - (((((v1383 * v1283) * v1382) + (v1383 * v1484)) * v1490) + (v1383 * v1486))) * v1468)) * v1301) * v1507) * v1510)) / v1508;
                let v1515 = if v1510 < v1514 { 1.0 } else { 0.0 };
                let v1537: f64;
                let v1538: Lanes<2>;
                if v1515 != 0.0 {
                    let v1516 = if v1510 < v131 { 1.0 } else { 0.0 };
                    let v1563: f64;
                    let v1564: Lanes<2>;
                    if v1516 != 0.0 {
                        let v1556 = v1510.exp();
                        let v1557 = v1513 * v1556;
                        v1563 = v1556;
                        v1564 = v1557;
                    } else {
                        let v1558 = v131.exp();
                        let v1561 = v1558 * (v137 + (v1510 - v131));
                        let v1562 = v1513 * v1558;
                        v1563 = v1561;
                        v1564 = v1562;
                    }
                    let v1565 = -v14;
                    let v1569 = (v137 - v1563) / v1510;
                    let v1573 = v137 + v1569;
                    let v1574 = v1565 * v1573;
                    let v1577 = ((v15 * v111) * v1573) + ((((v1564 * v111) - (v1513 * v1569)) / v1510) * v1565);
                    v1537 = v1574;
                    v1538 = v1577;
                } else {
                    let v1517 = v14 * v347;
                    let v1519 = v1517 * v1510;
                    let v1523 = v1510 * v1324;
                    let v1527 = v137 + (v1327 * v1510);
                    let v1532 = v137 + (v1523 * v1527);
                    let v1533 = v1519 * v1532;
                    let v1536 = ((((v15 * v347) * v1510) + (v1513 * v1517)) * v1532) + ((((v1513 * v1324) * v1527) + ((v1513 * v1327) * v1523)) * v1519);
                    v1537 = v1533;
                    v1538 = v1536;
                }
                let v1540 = v261 * v1539;
                let v1541 = v1540 * v1537;
                let v1543 = v1541 * v1386;
                let v1554 = ((v1543 * v1455) * v1381) * v1553;
                let v1555 = ((((((v1538 * v1540) * v1386) + (v1389 * v1541)) * v1455) + (v1456 * v1543)) * v1381) * v1553;
                v1403 = v1554;
                v1404 = v1555;
            } else {
                v1403 = v0;
                v1404 = v271;
            }
            let v1405 = v770 * v168;
            let v1406 = v169 * v770;
            let v1407 = v244 * v217;
            let v1408 = v218 * v244;
            let v1411 = (v137 + v1405).sqrt();
            let v1415 = v137 + v1411;
            let v1416 = (v1405 - v770) / v1415;
            let v1419 = (v1406 - ((v1406 * (v251 / (v249 * v1411))) * v1416)) / v1415;
            let v1421 = (v137 + v1407).sqrt();
            let v1425 = v137 + v1421;
            let v1426 = v1407 / v1425;
            let v1429 = (v1408 - ((v1408 * (v251 / (v249 * v1421))) * v1426)) / v1425;
            let v1438 = (v137 + (v1434 * v168)).sqrt();
            let v1442 = v137 + v1438;
            let v1443 = (v1431 * (v168 - v137)) / v1442;
            let v1446 = ((v169 * v1431) - (((v169 * v1434) * (v251 / (v249 * v1438))) * v1443)) / v1442;
            let v1599: f64;
            let v1600: f64;
            let v1601: f64;
            let v1602: Lanes<5>;
            let v1603: Lanes<8>;
            let v1604: Lanes<8>;
            if v1447 != 0.0 {
                let v1579 = v1443 * v1578;
                let v1580 = v1446 * v1578;
                let v1588 = (v137 + (v1434 * v192)).sqrt();
                let v1592 = v137 + v1588;
                let v1593 = (v1582 * (v192 - v137)) / v1592;
                let v1596 = ((v193 * v1582) - (((v193 * v1434) * (v251 / (v249 * v1588))) * v1593)) / v1592;
                let v1612: f64;
                let v1613: Lanes<8>;
                if v1597 != 0.0 {
                    let v1607 = v125 - v1606;
                    let v1608 = v1607 * v1607;
                    let v1609 = v127 * v1607;
                    let v1610 = v1609 + v1609;
                    let v1611 = if v1607 < v0 { 1.0 } else { 0.0 };
                    let v1640: f64;
                    let v1641: Lanes<8>;
                    if v1611 != 0.0 {
                        let v1620 = (v1608 + v1618).sqrt();
                        let v1624 = v1620 - v1607;
                        let v1627 = v1626 / v1624;
                        let v1630 = ((((v1610 * (v251 / (v249 * v1620))) - v127) * v1627) * v111) / v1624;
                        v1640 = v1627;
                        v1641 = v1630;
                    } else {
                        let v1632 = (v1608 + v1618).sqrt();
                        let v1638 = v347 * (v1632 + v1607);
                        let v1639 = ((v1610 * (v251 / (v249 * v1632))) + v127) * v347;
                        v1640 = v1638;
                        v1641 = v1639;
                    }
                    let v1647 = (v1645 + (v1593 * v1642)) + v1640;
                    let v1649 = v1640 / v1647;
                    let v1652 = (v1641 - (((v1596 * v1642) + v1641) * v1649)) / v1647;
                    v1612 = v1649;
                    v1613 = v1652;
                } else {
                    v1612 = v137;
                    v1613 = v1598;
                }
                let v1614 = v1612 * v1593;
                let v1617 = (v1613 * v1593) + (v1596 * v1612);
                v1599 = v1579;
                v1600 = v1614;
                v1601 = v1612;
                v1602 = v1580;
                v1603 = v1617;
                v1604 = v1613;
            } else {
                v1599 = v1443;
                v1600 = v0;
                v1601 = v137;
                v1602 = v1446;
                v1603 = v1598;
                v1604 = v1598;
            }
            let v1671: f64;
            let v1672: Lanes<3>;
            if v1605 != 0.0 {
                let v1653 = v44 + v14;
                let v1656 = (Lanes([v45[0], v45[1], 0.0])) + (Lanes([0.0, v15[0], v15[1]]));
                let v1661 = (v1657 * v1653) * v1660;
                let v1663 = v1661 * v1653;
                let v1666 = (((v1656 * v1657) * v1660) * v1653) + (v1656 * v1661);
                let v1669 = if (v1667 * v1653) < v0 { 1.0 } else { 0.0 };
                let v1734: f64;
                let v1735: Lanes<3>;
                if v1669 != 0.0 {
                    let v1708 = (v1663 + v1706).sqrt();
                    let v1715 = v1708 - (v1712 * v1653);
                    let v1718 = v1717 / v1715;
                    let v1721 = ((((v1666 * (v251 / (v249 * v1708))) - (v1656 * v1712)) * v1718) * v111) / v1715;
                    v1734 = v1718;
                    v1735 = v1721;
                } else {
                    let v1723 = (v1663 + v1706).sqrt();
                    let v1732 = v347 * (v1723 + (v1727 * v1653));
                    let v1733 = ((v1666 * (v251 / (v249 * v1723))) + (v1656 * v1727)) * v347;
                    v1734 = v1732;
                    v1735 = v1733;
                }
                let v1737 = if v1734 < v1736 { 1.0 } else { 0.0 };
                let v1759: f64;
                let v1760: Lanes<3>;
                if v1737 != 0.0 {
                    let v1739 = v1734 / v1738;
                    let v1747 = v137 - (v1739.powf(v1741));
                    let v1749 = v137 / v1747;
                    let v1752 = (((((v1735 / v1738) * (v1741 * (v1739.powf((v1741 - v251))))) * v111) * v1749) * v111) / v1747;
                    v1759 = v1749;
                    v1760 = v1752;
                } else {
                    let v1756 = v1735 * v1754;
                    let v1758 = v1757 + ((v1734 - v1736) * v1754);
                    v1759 = v1758;
                    v1760 = v1756;
                }
                v1671 = v1759;
                v1672 = v1760;
            } else {
                v1671 = v137;
                v1672 = v1670;
            }
            let v1673 = v1403 * v1671;
            let v1674 = v1404 * v1671;
            let v1677 = (Lanes([0.0, v1674[0], v1674[1]])) + (v1672 * v1403);
            let v1678 = v1599 * v1671;
            let v1680 = v1672 * v1599;
            let v1682 = (v1602 * v1671) + (Lanes([v1680[0], v1680[1], v1680[2], 0.0, 0.0]));
            let v1683 = v1204 * v1671;
            let v1685 = v1672 * v1204;
            let v1687 = (v1205 * v1671) + (Lanes([v1685[0], v1685[1], v1685[2], 0.0, 0.0]));
            let v1688 = v1600 * v1671;
            let v1690 = v1672 * v1600;
            let v1692 = (v1603 * v1671) + (Lanes([0.0, 0.0, v1690[0], v1690[1], v1690[2], 0.0, 0.0, 0.0]));
            let v1694 = v678 / v802;
            let v1697 = v769 / v806;
            let v1698 = (v137 + (v677 / v802)) + (v767 / v806);
            let v1701 = (Lanes([v1694[0], v1694[1], 0.0, 0.0])) + (Lanes([0.0, v1697[0], v1697[1], v1697[2]]));
            let v1702 = v1698 * v1698;
            let v1703 = v1701 * v1698;
            let v1704 = v1703 + v1703;
            let v1705 = if v1698 < v0 { 1.0 } else { 0.0 };
            let v1783: f64;
            let v1784: Lanes<4>;
            if v1705 != 0.0 {
                let v1763 = (v1702 + v1761).sqrt();
                let v1767 = v1763 - v1698;
                let v1770 = v1769 / v1767;
                let v1773 = ((((v1704 * (v251 / (v249 * v1763))) - v1701) * v1770) * v111) / v1767;
                v1783 = v1770;
                v1784 = v1773;
            } else {
                let v1775 = (v1702 + v1761).sqrt();
                let v1781 = v347 * (v1775 + v1698);
                let v1782 = ((v1704 * (v251 / (v249 * v1775))) + v1701) * v347;
                v1783 = v1781;
                v1784 = v1782;
            }
            let v1785 = v1783 * v876;
            let v1790 = v1789 / v1785;
            let v1793 = ((((v1784 * v876) + (v875 * v1783)) * v1790) * v111) / v1785;
            let v1795 = if v1790 < v1794 { 1.0 } else { 0.0 };
            let v1797: f64;
            let v1798: Lanes<4>;
            if v1795 != 0.0 {
                v1797 = v1794;
                v1798 = v1796;
            } else {
                v1797 = v1790;
                v1798 = v1793;
            }
            let v1799 = v674 * v1797;
            let v1800 = v1798 * v674;
            let v1806 = (v181 * v1802) + v45;
            let v1807 = ((v1802 * (v180 - v137)) + v44) / v1799;
            let v1808 = v1800 * v1807;
            let v1812 = ((Lanes([0.0, v1806[0], v1806[1], 0.0, 0.0])) - (Lanes([v1808[0], 0.0, v1808[1], v1808[2], v1808[3]]))) / v1799;
            let v1813 = if v891 > v0 { 1.0 } else { 0.0 };
            let v1816: f64;
            let v1817: Lanes<4>;
            if v1813 != 0.0 {
                let v1815 = if v1814 == v137 { 1.0 } else { 0.0 };
                let v1822: f64;
                let v1823: Lanes<4>;
                if v1815 != 0.0 {
                    let v1820 = if v14 < v1819 { 1.0 } else { 0.0 };
                    let v1831: f64;
                    let v1832: Lanes<4>;
                    if v1820 != 0.0 {
                        let v1828 = (-v891) / v1827;
                        let v1829 = (v894 * v111) / v1827;
                        let v1830 = if v1828 < v131 { 1.0 } else { 0.0 };
                        let v1840: f64;
                        let v1841: Lanes<4>;
                        if v1830 != 0.0 {
                            let v1833 = v1828.exp();
                            let v1834 = v1829 * v1833;
                            v1840 = v1833;
                            v1841 = v1834;
                        } else {
                            let v1835 = v131.exp();
                            let v1838 = v1835 * (v137 + (v1828 - v131));
                            let v1839 = v1829 * v1835;
                            v1840 = v1838;
                            v1841 = v1839;
                        }
                        let v1842 = v1819 - v14;
                        let v1844 = v1842 * v1840;
                        let v1845 = (v15 * v111) * v1840;
                        let v1848 = (Lanes([0.0, v1845[0], v1845[1], 0.0])) + (v1841 * v1842);
                        let v1850 = -v1849;
                        let v1857 = v1850 * (v1844.powf(v1851));
                        let v1858 = (v1848 * (v1851 * (v1844.powf((v1851 - v251))))) * v1850;
                        let v1859 = if v1857 < v131 { 1.0 } else { 0.0 };
                        let v1867: f64;
                        let v1868: Lanes<4>;
                        if v1859 != 0.0 {
                            let v1860 = v1857.exp();
                            let v1861 = v1858 * v1860;
                            v1867 = v1860;
                            v1868 = v1861;
                        } else {
                            let v1862 = v131.exp();
                            let v1865 = v1862 * (v137 + (v1857 - v131));
                            let v1866 = v1858 * v1862;
                            v1867 = v1865;
                            v1868 = v1866;
                        }
                        let v1870 = v1869 / v1849;
                        let v1871 = v1870 * v1844;
                        let v1873 = v1871 * v1867;
                        let v1876 = ((v1848 * v1870) * v1867) + (v1868 * v1871);
                        v1831 = v1873;
                        v1832 = v1876;
                    } else {
                        v1831 = v0;
                        v1832 = v1796;
                    }
                    v1822 = v1831;
                    v1823 = v1832;
                } else {
                    let v1821 = if v1814 == v261 { 1.0 } else { 0.0 };
                    let v1879: f64;
                    let v1880: Lanes<4>;
                    if v1821 != 0.0 {
                        let v1877 = if v14 < v194 { 1.0 } else { 0.0 };
                        let v1903: f64;
                        let v1904: Lanes<4>;
                        if v1877 != 0.0 {
                            let v1885 = (v261 * v1881) / (v1883 * v1883);
                            let v1886 = v194 - v14;
                            let v1887 = v15 * v111;
                            let v1888 = v1886 / v320;
                            let v1890 = Lanes([v1887[0], v1887[1], 0.0]);
                            let v1897 = ((v261 * v1888) / v1885).sqrt();
                            let v1900 = ((((v1890 - (v327 * v1888)) / v320) * v261) / v1885) * (v251 / (v249 * v1897));
                            let v1902 = if v1901 == v0 { 1.0 } else { 0.0 };
                            let v1915: f64;
                            let v1916: Lanes<3>;
                            if v1902 != 0.0 {
                                v1915 = v1883;
                                v1916 = v542;
                            } else {
                                let v1907 = v137 - (v347 * v322);
                                let v1908 = (v329 * v347) * v111;
                                let v1909 = v1883 * v1907;
                                let v1911 = v1909 * v1907;
                                let v1914 = ((v1908 * v1883) * v1907) + (v1908 * v1909);
                                v1915 = v1911;
                                v1916 = v1914;
                            }
                            let v1922 = v1900 * v1897;
                            let v1925 = v1916 * v1915;
                            let v1929 = ((v1897 * v1897) + (v1915 * v1915)).sqrt();
                            let v1933 = (v1897 * v1915) / v1929;
                            let v1936 = (((v1900 * v1915) + (v1916 * v1897)) - ((((v1922 + v1922) + (v1925 + v1925)) * (v251 / (v249 * v1929))) * v1933)) / v1929;
                            let v1937 = v1886 / v1933;
                            let v1940 = (v1890 - (v1936 * v1937)) / v1933;
                            let v1941 = v347 * v1933;
                            let v1942 = v1936 * v347;
                            let v1943 = v1941 * v1885;
                            let v1944 = v1942 * v1885;
                            let v1949 = v1937 + (v1943 * v320);
                            let v1950 = v1940 + ((v1944 * v320) + (v327 * v1943));
                            let v2011: f64;
                            let v2012: Lanes<4>;
                            if v1902 != 0.0 {
                                let v1951 = Lanes([0.0, v1950[0], v1950[1], v1950[2]]);
                                v2011 = v1949;
                                v2012 = v1951;
                            } else {
                                let v1953 = v261 * v1952;
                                let v1963 = v394 * (v137 + (v1953 * (v137 + (v261 * v322))));
                                let v1965 = v891 / v1963;
                                let v1966 = (((v329 * v261) * v1953) * v394) * v1965;
                                let v1970 = ((v137 + v1952) / (v137 + v1953)) - v1965;
                                let v1973 = v1944 * v1970;
                                let v1977 = v1937 - (v1943 * v1970);
                                let v1979 = (Lanes([0.0, v1940[0], v1940[1], v1940[2]])) - ((Lanes([0.0, v1973[0], v1973[1], v1973[2]])) + ((((v894 - (Lanes([0.0, v1966[0], v1966[1], v1966[2]]))) / v1963) * v111) * v1943));
                                let v1980 = v1977 - v1949;
                                let v1981 = Lanes([0.0, v1950[0], v1950[1], v1950[2]]);
                                let v1984 = (v1979 - v1981) * v1980;
                                let v1986 = v576 * v1937;
                                let v1988 = v1986 * v1937;
                                let v1997 = (((((v1940 * v576) * v1937) + (v1940 * v1986)) * v323) + (v330 * v1988)) / v394;
                                let v2003 = ((v1980 * v1980) + ((v1988 * v323) / v394)).sqrt();
                                let v2009 = v347 * ((v1977 + v1949) + v2003);
                                let v2010 = ((v1979 + v1981) + (((v1984 + v1984) + (Lanes([0.0, v1997[0], v1997[1], v1997[2]]))) * (v251 / (v249 * v2003)))) * v347;
                                v2011 = v2009;
                                v2012 = v2010;
                            }
                            let v2016 = (v2011 - v1937) / v2011;
                            let v2019 = ((v2012 - (Lanes([0.0, v1940[0], v1940[1], v1940[2]]))) - (v2012 * v2016)) / v2011;
                            let v2022 = if (v2016.abs()) > v2021 { 1.0 } else { 0.0 };
                            let v2076: f64;
                            let v2077: Lanes<4>;
                            if v2022 != 0.0 {
                                let v2023 = v1941 / v2016;
                                let v2027 = ((Lanes([0.0, v1942[0], v1942[1], v1942[2]])) - (v2019 * v2023)) / v2016;
                                let v2030 = v2028 / v2029;
                                let v2031 = v2030 * v2011;
                                let v2033 = v2031 * v2023;
                                let v2038 = (-v2029) / v2011;
                                let v2041 = ((v2012 * v2038) * v111) / v2011;
                                let v2042 = v2038.exp();
                                let v2044 = v1915 / v2023;
                                let v2049 = v137 + v2044;
                                let v2054 = (v2038 * v2049).exp();
                                let v2056 = v2042 - v2054;
                                let v2058 = v2033 * v2056;
                                let v2061 = ((((v2012 * v2030) * v2023) + (v2027 * v2031)) * v2056) + (((v2041 * v2042) - (((v2041 * v2049) + ((((Lanes([0.0, v1916[0], v1916[1], v1916[2]])) - (v2027 * v2044)) / v2023) * v2038)) * v2054)) * v2033);
                                v2076 = v2058;
                                v2077 = v2061;
                            } else {
                                let v2062 = v2028 * v1915;
                                let v2065 = (-v2029) / v2011;
                                let v2069 = v2065.exp();
                                let v2071 = v2062 * v2069;
                                let v2072 = (v1916 * v2028) * v2069;
                                let v2075 = (Lanes([0.0, v2072[0], v2072[1], v2072[2]])) + (((((v2012 * v2065) * v111) / v2011) * v2069) * v2062);
                                v2076 = v2071;
                                v2077 = v2075;
                            }
                            v1903 = v2076;
                            v1904 = v2077;
                        } else {
                            v1903 = v0;
                            v1904 = v1796;
                        }
                        v1879 = v1903;
                        v1880 = v1904;
                    } else {
                        let v1878 = if v1814 == v674 { 1.0 } else { 0.0 };
                        let v2079: f64;
                        let v2080: Lanes<4>;
                        if v1878 != 0.0 {
                            let v2078 = if v14 < v1819 { 1.0 } else { 0.0 };
                            let v2108: f64;
                            let v2109: Lanes<4>;
                            if v2078 != 0.0 {
                                let v2081 = v1819 - v14;
                                let v2082 = v15 * v111;
                                let v2083 = v2081.powf(v1851);
                                let v2089 = v2088 + v891;
                                let v2090 = v891 / v2089;
                                let v2094 = v137 - v2090;
                                let v2097 = v2094.powf(v2096);
                                let v2102 = v2083 * v2097;
                                let v2103 = (v2082 * (v1851 * (v2081.powf((v1851 - v251))))) * v2097;
                                let v2106 = (Lanes([0.0, v2103[0], v2103[1], 0.0])) + (((((v894 - (v894 * v2090)) / v2089) * v111) * (v2096 * (v2094.powf((v2096 - v251))))) * v2083);
                                let v2107 = if v1901 == v0 { 1.0 } else { 0.0 };
                                let v2119: f64;
                                let v2120: Lanes<4>;
                                if v2107 != 0.0 {
                                    v2119 = v2102;
                                    v2120 = v2106;
                                } else {
                                    let v2112 = (v891 - v2110) / v2088;
                                    let v2113 = v894 / v2088;
                                    let v2116 = (v2112 - v137) / v2115;
                                    let v2117 = v2113 / v2115;
                                    let v2118 = if v2112 < v137 { 1.0 } else { 0.0 };
                                    let v2146: f64;
                                    let v2147: Lanes<4>;
                                    if v2118 != 0.0 {
                                        let v2125 = v2116.exp();
                                        let v2127 = v137 + v2125;
                                        let v2132 = ((v2117 * v2125) * (v251 / v2127)) * v2115;
                                        let v2133 = v137 + (v2115 * (v2127.ln()));
                                        v2146 = v2133;
                                        v2147 = v2132;
                                    } else {
                                        let v2136 = (-v2116).exp();
                                        let v2138 = v137 + v2136;
                                        let v2144 = v2112 + (v2115 * (v2138.ln()));
                                        let v2145 = v2113 + ((((v2117 * v111) * v2136) * (v251 / v2138)) * v2115);
                                        v2146 = v2144;
                                        v2147 = v2145;
                                    }
                                    let v2149 = v2146.powf(v2148);
                                    let v2154 = v2102 * v2149;
                                    let v2157 = (v2106 * v2149) + ((v2147 * (v2148 * (v2146.powf((v2148 - v251))))) * v2102);
                                    v2119 = v2154;
                                    v2120 = v2157;
                                }
                                let v2121 = -v1849;
                                let v2122 = v2121 * v2119;
                                let v2123 = v2120 * v2121;
                                let v2124 = if v2122 < v131 { 1.0 } else { 0.0 };
                                let v2165: f64;
                                let v2166: Lanes<4>;
                                if v2124 != 0.0 {
                                    let v2158 = v2122.exp();
                                    let v2159 = v2123 * v2158;
                                    v2165 = v2158;
                                    v2166 = v2159;
                                } else {
                                    let v2160 = v131.exp();
                                    let v2163 = v2160 * (v137 + (v2122 - v131));
                                    let v2164 = v2123 * v2160;
                                    v2165 = v2163;
                                    v2166 = v2164;
                                }
                                let v2167 = v1869 / v1849;
                                let v2168 = v2167 * v2081;
                                let v2170 = v2168 * v2165;
                                let v2171 = (v2082 * v2167) * v2165;
                                let v2174 = (Lanes([0.0, v2171[0], v2171[1], 0.0])) + (v2166 * v2168);
                                v2108 = v2170;
                                v2109 = v2174;
                            } else {
                                v2108 = v0;
                                v2109 = v1796;
                            }
                            v2079 = v2108;
                            v2080 = v2109;
                        } else {
                            v2079 = v0;
                            v2080 = v1796;
                        }
                        v1879 = v2079;
                        v1880 = v2080;
                    }
                    v1822 = v1879;
                    v1823 = v1880;
                }
                let v1824 = if v1822 > v0 { 1.0 } else { 0.0 };
                let v2177: f64;
                let v2178: Lanes<4>;
                if v1824 != 0.0 {
                    let v2176 = if v2175 == v137 { 1.0 } else { 0.0 };
                    let v2207: f64;
                    let v2208: Lanes<4>;
                    if v2176 != 0.0 {
                        let v2180 = v2179 + v1799;
                        let v2181 = v891 * v2180;
                        let v2185 = v289 / v2181;
                        let v2196 = v2195 / v2180;
                        let v2200 = (v2185 + ((v877 / v884) * v1031)) + v2196;
                        let v2201 = ((((((v894 * v2180) + (v1800 * v891)) * v2185) * v111) / v2181) + ((v880 / v884) * v1031)) + (((v1800 * v2196) * v111) / v2180);
                        let v2202 = if v1814 == v674 { 1.0 } else { 0.0 };
                        let v2229: f64;
                        let v2230: Lanes<4>;
                        if v2202 != 0.0 {
                            let v2212 = (v1822 - v2200) / v2211;
                            let v2213 = (v1823 - v2201) / v2211;
                            let v2214 = if v1822 < v2200 { 1.0 } else { 0.0 };
                            let v2253: f64;
                            let v2254: Lanes<4>;
                            if v2214 != 0.0 {
                                let v2231 = v2212.exp();
                                let v2233 = v137 + v2231;
                                let v2239 = v1822 - (v2211 * (v2233.ln()));
                                let v2240 = v1823 - (((v2213 * v2231) * (v251 / v2233)) * v2211);
                                v2253 = v2239;
                                v2254 = v2240;
                            } else {
                                let v2243 = (-v2212).exp();
                                let v2245 = v137 + v2243;
                                let v2251 = v2200 - (v2211 * (v2245.ln()));
                                let v2252 = v2201 - ((((v2213 * v111) * v2243) * (v251 / v2245)) * v2211);
                                v2253 = v2251;
                                v2254 = v2252;
                            }
                            let v2255 = v891 * v2253;
                            let v2258 = (v894 * v2253) + (v2254 * v891);
                            v2229 = v2255;
                            v2230 = v2258;
                        } else {
                            let v2215 = v891 * v1822;
                            let v2223 = v1822 + v2200;
                            let v2225 = (v2215 * v2200) / v2223;
                            let v2228 = (((((v894 * v1822) + (v1823 * v891)) * v2200) + (v2201 * v2215)) - ((v1823 + v2201) * v2225)) / v2223;
                            v2229 = v2225;
                            v2230 = v2228;
                        }
                        v2207 = v2229;
                        v2208 = v2230;
                    } else {
                        let v2203 = v891 * v1822;
                        let v2206 = (v894 * v1822) + (v1823 * v891);
                        v2207 = v2203;
                        v2208 = v2206;
                    }
                    v2177 = v2207;
                    v2178 = v2208;
                } else {
                    v2177 = v0;
                    v2178 = v1796;
                }
                v1816 = v2177;
                v1817 = v2178;
            } else {
                v1816 = v0;
                v1817 = v1796;
            }
            let v1818 = if v321 > v0 { 1.0 } else { 0.0 };
            let v2260 = v2259 * v677;
            let v2261 = v678 * v2259;
            let v2263 = (v38 - v332) / v334;
            let v2264 = v39 / v334;
            let v2265 = if v38 < v332 { 1.0 } else { 0.0 };
            let v2288: f64;
            let v2289: Lanes<2>;
            if v2265 != 0.0 {
                let v2266 = v2263.exp();
                let v2268 = v137 + v2266;
                let v2274 = v38 - (v334 * (v2268.ln()));
                let v2275 = v39 - (((v2264 * v2266) * (v251 / v2268)) * v334);
                v2288 = v2274;
                v2289 = v2275;
            } else {
                let v2278 = (-v2263).exp();
                let v2280 = v137 + v2278;
                let v2286 = v332 - (v334 * (v2280.ln()));
                let v2287 = ((((v2264 * v111) * v2278) * (v251 / v2280)) * v334) * v111;
                v2288 = v2286;
                v2289 = v2287;
            }
            let v2292 = v137 - (v2288 * v656);
            let v2309 = v2308 * ((v669 * (v137 - (v2292.powf(v661)))) + (v674 * (v38 - v2288)));
            let v2310 = ((((((v2289 * v656) * v111) * (v661 * (v2292.powf(v663)))) * v111) * v669) + ((v39 - v2289) * v674)) * v2308;
            let v2312 = v2311 * v767;
            let v2313 = v769 * v2311;
            let v2315 = v2314 * v779;
            let v2317 = v2315 * v1783;
            let v2318 = (v782 * v2314) * v1783;
            let v2321 = (Lanes([v2318[0], v2318[1], 0.0, 0.0])) + (v1784 * v2315);
            let v2322 = v2314 * v797;
            let v2324 = v2322 * v1783;
            let v2325 = (v800 * v2314) * v1783;
            let v2328 = (Lanes([0.0, v2325[0], v2325[1], v2325[2]])) + (v1784 * v2322);
            let v2331 = (v106 - v684) / v2330;
            let v2332 = v109 / v2330;
            let v2333 = if v106 < v684 { 1.0 } else { 0.0 };
            let v2356: f64;
            let v2357: Lanes<5>;
            if v2333 != 0.0 {
                let v2334 = v2331.exp();
                let v2336 = v137 + v2334;
                let v2342 = v106 - (v2330 * (v2336.ln()));
                let v2343 = v109 - (((v2332 * v2334) * (v251 / v2336)) * v2330);
                v2356 = v2342;
                v2357 = v2343;
            } else {
                let v2346 = (-v2331).exp();
                let v2348 = v137 + v2346;
                let v2354 = v684 - (v2330 * (v2348.ln()));
                let v2355 = ((((v2332 * v111) * v2346) * (v251 / v2348)) * v2330) * v111;
                v2356 = v2354;
                v2357 = v2355;
            }
            let v2360 = v137 - (v2356 / v575);
            let v2388 = ((v2382 * ((v761 * ((v747 * (v137 - (v2360.powf(v735)))) + (v750 * (v106 - v2356)))) + (v764 * v106))) * v2385) * v1578;
            let v2389 = ((((((((((v2357 / v575) * v111) * (v735 * (v2360.powf(v737)))) * v111) * v747) + ((v109 - v2357) * v750)) * v761) + (v109 * v764)) * v2382) * v2385) * v1578;
            let v2391 = (v125 - v684) / v2330;
            let v2392 = v127 / v2330;
            let v2393 = if v125 < v684 { 1.0 } else { 0.0 };
            let v2416: f64;
            let v2417: Lanes<8>;
            if v2393 != 0.0 {
                let v2394 = v2391.exp();
                let v2396 = v137 + v2394;
                let v2402 = v125 - (v2330 * (v2396.ln()));
                let v2403 = v127 - (((v2392 * v2394) * (v251 / v2396)) * v2330);
                v2416 = v2402;
                v2417 = v2403;
            } else {
                let v2406 = (-v2391).exp();
                let v2408 = v137 + v2406;
                let v2414 = v684 - (v2330 * (v2408.ln()));
                let v2415 = ((((v2392 * v111) * v2406) * (v251 / v2408)) * v2330) * v111;
                v2416 = v2414;
                v2417 = v2415;
            }
            let v2420 = v137 - (v2416 / v575);
            let v2447 = ((v2382 * ((v761 * ((v747 * (v137 - (v2420.powf(v735)))) + (v750 * (v125 - v2416)))) + (v764 * v125))) * v2385) * v2446;
            let v2448 = ((((((((((v2417 / v575) * v111) * (v735 * (v2420.powf(v737)))) * v111) * v747) + ((v127 - v2417) * v750)) * v761) + (v127 * v764)) * v2382) * v2385) * v2446;
            let v2450 = v30 / v2449;
            let v2451 = v31 / v2449;
            let v2452 = if v2450 < v131 { 1.0 } else { 0.0 };
            let v2460: f64;
            let v2461: Lanes<2>;
            if v2452 != 0.0 {
                let v2453 = v2450.exp();
                let v2454 = v2451 * v2453;
                v2460 = v2453;
                v2461 = v2454;
            } else {
                let v2455 = v131.exp();
                let v2458 = v2455 * (v137 + (v2450 - v131));
                let v2459 = v2451 * v2455;
                v2460 = v2458;
                v2461 = v2459;
            }
            let v2463 = v2462 * v2460;
            let v2464 = v2461 * v2462;
            let v2466 = v2465 * v322;
            let v2471 = (v324 + v272) + v261;
            let v2472 = v2466 * v2471;
            let v2475 = ((v329 * v2465) * v2471) + ((v331 + (Lanes([v273[0], v273[1], 0.0]))) * v2466);
            let v2499: f64;
            let v2500: Lanes<5>;
            if v2476 != 0.0 {
                let v2489 = (v2485 * ((v2477 * v1416) + (v2480 * v1426))) / v2488;
                let v2490 = (((v1419 * v2477) + (v1429 * v2480)) * v2485) / v2488;
                v2499 = v2489;
                v2500 = v2490;
            } else {
                let v2496 = ((v106 - v2491) / v2493) * v128;
                let v2497 = (v109 / v2493) * v128;
                let v2498 = if v2496 < v131 { 1.0 } else { 0.0 };
                let v2509: f64;
                let v2510: Lanes<5>;
                if v2498 != 0.0 {
                    let v2502 = v2496.exp();
                    let v2503 = v2497 * v2502;
                    v2509 = v2502;
                    v2510 = v2503;
                } else {
                    let v2504 = v131.exp();
                    let v2507 = v2504 * (v137 + (v2496 - v131));
                    let v2508 = v2497 * v2504;
                    v2509 = v2507;
                    v2510 = v2508;
                }
                let v2517 = (v137 + (v244 * v2509)).sqrt();
                let v2521 = v137 + v2517;
                let v2522 = (v2511 * v168) / v2521;
                let v2525 = ((v169 * v2511) - (((v2510 * v244) * (v251 / (v249 * v2517))) * v2522)) / v2521;
                v2499 = v2522;
                v2500 = v2525;
            }
            let v2528: f64;
            let v2529: f64;
            let v2530: Lanes<8>;
            let v2531: Lanes<5>;
            if v2501 != 0.0 {
                let v2526 = v2499 * v1578;
                let v2527 = v2500 * v1578;
                let v2572: f64;
                let v2573: Lanes<8>;
                if v2476 != 0.0 {
                    let v2533 = v770 * v192;
                    let v2534 = v193 * v770;
                    let v2537 = (v137 + v2533).sqrt();
                    let v2541 = v137 + v2537;
                    let v2542 = (v2533 - v770) / v2541;
                    let v2546 = v244 * v205;
                    let v2547 = v206 * v244;
                    let v2549 = (v137 + v2546).sqrt();
                    let v2553 = v137 + v2549;
                    let v2554 = v2546 / v2553;
                    let v2567 = (v2564 * ((v2477 * v2542) + (v2480 * v2554))) / v2488;
                    let v2568 = (((((v2534 - ((v2534 * (v251 / (v249 * v2537))) * v2542)) / v2541) * v2477) + (((v2547 - ((v2547 * (v251 / (v249 * v2549))) * v2554)) / v2553) * v2480)) * v2564) / v2488;
                    v2572 = v2567;
                    v2573 = v2568;
                } else {
                    let v2570 = (v125 - v2491) * v128;
                    let v2571 = if v2570 < v131 { 1.0 } else { 0.0 };
                    let v2585: f64;
                    let v2586: Lanes<8>;
                    if v2571 != 0.0 {
                        let v2578 = v2570.exp();
                        let v2579 = v183 * v2578;
                        v2585 = v2578;
                        v2586 = v2579;
                    } else {
                        let v2580 = v131.exp();
                        let v2583 = v2580 * (v137 + (v2570 - v131));
                        let v2584 = v183 * v2580;
                        v2585 = v2583;
                        v2586 = v2584;
                    }
                    let v2593 = (v137 + (v244 * v2585)).sqrt();
                    let v2597 = v137 + v2593;
                    let v2598 = (v2587 * v192) / v2597;
                    let v2601 = ((v193 * v2587) - (((v2586 * v244) * (v251 / (v249 * v2593))) * v2598)) / v2597;
                    v2572 = v2598;
                    v2573 = v2601;
                }
                let v2574 = v1601 * v2572;
                let v2577 = (v1604 * v2572) + (v2573 * v1601);
                v2528 = v2574;
                v2529 = v2526;
                v2530 = v2577;
                v2531 = v2527;
            } else {
                v2528 = v0;
                v2529 = v2499;
                v2530 = v1598;
                v2531 = v2500;
            }
            let v2611: f64;
            let v2612: f64;
            let v2613: f64;
            let v2614: f64;
            let v2615: Lanes<4>;
            let v2616: Lanes<2>;
            let v2617: Lanes<4>;
            let v2618: Lanes<5>;
            if v2532 != 0.0 {
                let v2607 = v660 * (v2602 * (v659.powf(v2604)));
                let v2608 = (v659.powf(v2602)) - v674;
                let v2609 = if v335 < v0 { 1.0 } else { 0.0 };
                let v2673: f64;
                let v2674: Lanes<2>;
                if v2609 != 0.0 {
                    let v2657 = v335.exp();
                    let v2659 = v137 + v2657;
                    let v2660 = v137 / v2659;
                    let v2663 = (((v336 * v2657) * v2660) * v111) / v2659;
                    v2673 = v2660;
                    v2674 = v2663;
                } else {
                    let v2666 = (-v335).exp();
                    let v2667 = (v336 * v111) * v2666;
                    let v2668 = v137 + v2666;
                    let v2669 = v2666 / v2668;
                    let v2672 = (v2667 - (v2667 * v2669)) / v2668;
                    v2673 = v2669;
                    v2674 = v2672;
                }
                let v2681 = ((v2607 * v2673) + (v2674 * v2608)) * v2259;
                let v2684 = (v771 * v128) / v145;
                let v2686 = v347 / v774;
                let v2690 = v2684 * v2686;
                let v2694 = v2314 * v1783;
                let v2698 = ((((v772 * v128) / v145) * v2686) + ((((v777 * v2686) * v111) / v774) * v2684)) * v2694;
                let v2702 = v2464 / v2449;
                let v2703 = v364 * v44;
                let v2708 = ((v2259 * ((v2608 * v2673) + v674)) + (v2694 * v2690)) + (v2463 / v2449);
                let v2711 = v2703 * v2708;
                let v2712 = (v45 * v364) * v2708;
                let v2713 = (((Lanes([v2681[0], v2681[1], 0.0, 0.0])) + (((v1784 * v2314) * v2690) + (Lanes([v2698[0], v2698[1], 0.0, 0.0])))) + (Lanes([v2702[0], v2702[1], 0.0, 0.0]))) * v2703;
                let v2716 = (Lanes([0.0, v2712[0], v2712[1], 0.0, 0.0])) + (Lanes([v2713[0], 0.0, v2713[1], v2713[2], v2713[3]]));
                let v2718 = v2717 * v2463;
                let v2719 = v2464 * v2717;
                let v2722 = v2464 * v2720;
                let v2723 = v2317 + (v2720 * v2463);
                let v2725 = v2321 + (Lanes([v2722[0], v2722[1], 0.0, 0.0]));
                let v2729 = (v2726 * v2723) + v2324;
                let v2730 = (v2725 * v2726) + v2328;
                let v2732 = v2731 * v2723;
                let v2733 = v2725 * v2731;
                v2611 = v2732;
                v2612 = v2718;
                v2613 = v2729;
                v2614 = v2711;
                v2615 = v2733;
                v2616 = v2719;
                v2617 = v2730;
                v2618 = v2716;
            } else {
                v2611 = v2317;
                v2612 = v2463;
                v2613 = v2324;
                v2614 = v0;
                v2615 = v2321;
                v2616 = v2464;
                v2617 = v2328;
                v2618 = v2610;
            }
            let v2622 = (v13 * v296) * v2621;
            let v2623 = (v297 * v13) * v2621;
            let v2626 = (v13 * v891) * v2621;
            let v2627 = (v894 * v13) * v2621;
            let v2634 = (v13 * ((v1124 + v1187) + v1221)) * v2621;
            let v2635 = (((v1125 + v1188) + v1222) * v13) * v2621;
            let v2640 = v31 * v1;
            let v2655 = (v13 * (((((v998 + v1170) + (v1 * v30)) - v1239) + v976) + v936)) * v2621;
            let v2656 = ((((((v999 + (Lanes([v1171[0], v1171[1], 0.0, 0.0]))) + (Lanes([v2640[0], v2640[1], 0.0, 0.0]))) - (Lanes([v1240[0], v1240[1], 0.0, 0.0]))) + (Lanes([v979[0], v979[1], 0.0, 0.0]))) + (Lanes([v937[0], v937[1], 0.0, 0.0]))) * v13) * v2621;
            let v2746: f64;
            let v2747: f64;
            let v2748: Lanes<3>;
            let v2749: Lanes<3>;
            if v2 != 0.0 {
                let v2738 = (v13 * (-v1673)) * v2621;
                let v2739 = ((v1677 * v111) * v13) * v2621;
                v2746 = v2738;
                v2747 = v0;
                v2748 = v2739;
                v2749 = v1670;
            } else {
                let v2744 = (v13 * (-v1673)) * v2621;
                let v2745 = ((v1677 * v111) * v13) * v2621;
                v2746 = v0;
                v2747 = v2744;
                v2748 = v1670;
                v2749 = v2745;
            }
            let v2752 = (v13 * v1807) * v2621;
            let v2753 = (v1812 * v13) * v2621;
            let v2759 = (v13 * (v2754 * v1816)) * v2621;
            let v2760 = ((v1817 * v2754) * v13) * v2621;
            let v2765 = ((v13 * v58) / v2195) * v2621;
            let v2766 = ((v59 * v13) / v2195) * v2621;
            let v2771 = ((v13 * v66) / v2179) * v2621;
            let v2772 = ((v67 * v13) / v2179) * v2621;
            let v2784 = (ddt(12033, (v13 * ((v2260 + v2611) + v2612)))) * v2621;
            let v2785 = (((((Lanes([v2261[0], v2261[1], 0.0, 0.0])) + v2615) + (Lanes([v2616[0], v2616[1], 0.0, 0.0]))) * v13) * v2782) * v2621;
            let v2790 = (ddt(12039, (v13 * v2309))) * v2621;
            let v2791 = ((v2310 * v13) * v2782) * v2621;
            let v2802 = (ddt(12049, (v13 * ((v2312 + v2613) + v2472)))) * v2621;
            let v2803 = (((((Lanes([0.0, v2313[0], v2313[1], v2313[2]])) + v2617) + (Lanes([0.0, v2475[0], v2475[1], v2475[2]]))) * v13) * v2782) * v2621;
            let v2808 = (ddt(12055, (v13 * v2614))) * v2621;
            let v2809 = ((v2618 * v13) * v2782) * v2621;
            let v2815 = (ddt(12063, (v2810 * v72))) * v2621;
            let v2816 = ((v73 * v2810) * v2782) * v2621;
            let v2822 = (ddt(12071, (v2817 * v80))) * v2621;
            let v2823 = ((v81 * v2817) * v2782) * v2621;
            let v2826 = (v13 * v1688) * v2621;
            let v2827 = (v1692 * v13) * v2621;
            let v2833 = ((v13 * v121) * v2830) * v2621;
            let v2834 = ((v124 * v13) * v2830) * v2621;
            let v2841 = (ddt(12091, (v13 * (v2447 + v2528)))) * v2621;
            let v2842 = (((v2448 + v2530) * v13) * v2782) * v2621;
            let v2851 = (v13 * ((v1683 + (v1 * v106)) + v1678)) * v2621;
            let v2852 = (((v1687 + (v109 * v1)) + v1682) * v13) * v2621;
            let v2859 = (ddt(12110, (v13 * (v2388 + v2529)))) * v2621;
            let v2860 = (((v2389 + v2531) * v13) * v2782) * v2621;
            let v2869: f64;
            let v2870: Lanes<2>;
            if v3 != 0.0 {
                let v2866 = ((v13 * v96) * v2863) * v2621;
                let v2867 = ((v97 * v13) * v2863) * v2621;
                v2869 = v2866;
                v2870 = v2867;
            } else {
                v2869 = v0;
                v2870 = v2868;
            }
            let v2879: f64;
            let v2880: Lanes<2>;
            if v4 != 0.0 {
                let v2876 = ((v13 * v88) * v2873) * v2621;
                let v2877 = ((v89 * v13) * v2873) * v2621;
                v2879 = v2876;
                v2880 = v2877;
            } else {
                v2879 = v0;
                v2880 = v2878;
            }
            let v2883 = (v885 + v882) / v877;
            let v2886 = ((v888 + v889) - (v880 * v2883)) / v877;
            let v2898: f64;
            let v2899: Lanes<4>;
            if v2887 != 0.0 {
                let v2888 = v1816 / v2883;
                let v2892 = v2888.abs();
                let v2897 = ((v1817 - (v2886 * v2888)) / v2883) * ((v249 * (if v2888 >= v2893 { 1.0 } else { 0.0 })) - v251);
                v2898 = v2892;
                v2899 = v2897;
            } else {
                v2898 = v0;
                v2899 = v1796;
            }
            let v2900 = if v2883 > v0 { 1.0 } else { 0.0 };
            let v2914: f64;
            let v2915: Lanes<4>;
            if v2900 != 0.0 {
                let v2903 = (v2611 + v2613) / v2883;
                let v2906 = ((v2615 + v2617) - (v2886 * v2903)) / v2883;
                v2914 = v2903;
                v2915 = v2906;
            } else {
                let v2908 = v2907 * v1783;
                let v2910 = v2908 * v877;
                let v2913 = ((v1784 * v2907) * v877) + (v880 * v2908);
                v2914 = v2910;
                v2915 = v2913;
            }
            let v2920: f64;
            let v2921: Lanes<4>;
            if v2916 != 0.0 {
                let v2917 = v2726 * v2914;
                let v2918 = v2915 * v2726;
                v2920 = v2917;
                v2921 = v2918;
            } else {
                let v2927: f64;
                let v2928: Lanes<4>;
                if v2919 != 0.0 {
                    let v2925 = v2924 * v2914;
                    let v2926 = v2915 * v2924;
                    v2927 = v2925;
                    v2928 = v2926;
                } else {
                    v2927 = v0;
                    v2928 = v1796;
                }
                v2920 = v2927;
                v2921 = v2928;
            }
            let v2923 = if (v998 + v1124) < v0 { 1.0 } else { 0.0 };
            let v2931 = if ((v1170 + v1187) + v1221) < v0 { 1.0 } else { 0.0 };
            let v2932 = if v1683 < v0 { 1.0 } else { 0.0 };
            let v2933 = if v1678 < v0 { 1.0 } else { 0.0 };
            let v2934 = if v1688 < v0 { 1.0 } else { 0.0 };
            let v2936 = ddt(12363, v2935);
            let v2939 = v2920 * v2936;
            let v2940 = v2921 * v2936;
            let v2941 = (v2937 * v2782) * v2920;
            let v2944 = (Lanes([v2940[0], v2940[1], v2940[2], v2940[3], 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, v2941[0]]));
            let v2945 = v2898 * v2935;
            let v2946 = v2899 * v2935;
            let v2947 = v2937 * v2898;
            let v2950 = (Lanes([v2946[0], v2946[1], v2946[2], v2946[3], 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, v2947[0]]));
            let v2955 = if ((((v2771 + v2815) + v2822) + v2826) + v2841) == v0 { 1.0 } else { 0.0 };
            let v2956 = v2623[0];
            let v2957 = v2623[1];
            let v2958 = v2623[2];
            let v2959 = v2627[0];
            let v2960 = v2627[1];
            let v2961 = v2627[2];
            let v2962 = v2627[3];
            let v2963 = v2635[0];
            let v2964 = v2635[1];
            let v2965 = v2656[0];
            let v2966 = v2656[1];
            let v2967 = v2656[2];
            let v2968 = v2656[3];
            let v2969 = v2748[0];
            let v2970 = v2748[1];
            let v2971 = v2748[2];
            let v2972 = v2749[0];
            let v2973 = v2749[1];
            let v2974 = v2749[2];
            let v2975 = v2753[0];
            let v2976 = v2753[1];
            let v2977 = v2753[2];
            let v2978 = v2753[3];
            let v2979 = v2753[4];
            let v2980 = v2760[0];
            let v2981 = v2760[1];
            let v2982 = v2760[2];
            let v2983 = v2760[3];
            let v2984 = v2766[0];
            let v2985 = v2766[1];
            let v2986 = v2772[0];
            let v2987 = v2772[1];
            let v2988 = v2785[0];
            let v2989 = v2785[1];
            let v2990 = v2785[2];
            let v2991 = v2785[3];
            let v2992 = v2791[0];
            let v2993 = v2791[1];
            let v2994 = v2803[0];
            let v2995 = v2803[1];
            let v2996 = v2803[2];
            let v2997 = v2803[3];
            let v2998 = v2809[0];
            let v2999 = v2809[1];
            let v3000 = v2809[2];
            let v3001 = v2809[3];
            let v3002 = v2809[4];
            let v3003 = v2816[0];
            let v3004 = v2816[1];
            let v3005 = v2823[0];
            let v3006 = v2823[1];
            let v3007 = v2827[0];
            let v3008 = v2827[1];
            let v3009 = v2827[2];
            let v3010 = v2827[3];
            let v3011 = v2827[4];
            let v3012 = v2827[5];
            let v3013 = v2827[6];
            let v3014 = v2827[7];
            let v3015 = v2834[0];
            let v3016 = v2834[1];
            let v3017 = v2834[2];
            let v3018 = v2834[3];
            let v3019 = v2834[4];
            let v3020 = v2834[5];
            let v3021 = v2834[6];
            let v3022 = v2834[7];
            let v3023 = v2842[0];
            let v3024 = v2842[1];
            let v3025 = v2842[2];
            let v3026 = v2842[3];
            let v3027 = v2842[4];
            let v3028 = v2842[5];
            let v3029 = v2842[6];
            let v3030 = v2842[7];
            let v3031 = v2852[0];
            let v3032 = v2852[1];
            let v3033 = v2852[2];
            let v3034 = v2852[3];
            let v3035 = v2852[4];
            let v3036 = v2860[0];
            let v3037 = v2860[1];
            let v3038 = v2860[2];
            let v3039 = v2860[3];
            let v3040 = v2860[4];
            let v3041 = v2870[0];
            let v3042 = v2870[1];
            let v3043 = v2880[0];
            let v3044 = v2880[1];
            let v3045 = v2937[0];
            let v3046 = v2944[0];
            let v3047 = v2944[1];
            let v3048 = v2944[2];
            let v3049 = v2944[3];
            let v3050 = v2944[4];
            let v3051 = v2950[0];
            let v3052 = v2950[1];
            let v3053 = v2950[2];
            let v3054 = v2950[3];
            let v3055 = v2950[4];
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(6),
            Some(7),
            multiplicity * (v2622),
            [5, 6, 7],
            [v2956, v2957, v2958],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(3),
            multiplicity * (v2626),
            [3, 5, 6, 7],
            [v2959, v2960, v2961, v2962],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(4),
            Some(3),
            multiplicity * (v2634),
            [3, 4],
            [v2963, v2964],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(3),
            multiplicity * (v2655),
            [3, 5, 6, 7],
            [v2965, v2966, v2967, v2968],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(6),
            multiplicity * (v2746),
            [4, 5, 6],
            [v2969, v2970, v2971],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(7),
            multiplicity * (v2747),
            [4, 5, 6],
            [v2972, v2973, v2974],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(5),
            multiplicity * (v2752),
            [3, 4, 5, 6, 7],
            [v2975, v2976, v2977, v2978, v2979],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(7),
            multiplicity * (v2759),
            [3, 5, 6, 7],
            [v2980, v2981, v2982, v2983],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(2),
            Some(3),
            multiplicity * (v2765),
            [2, 3],
            [v2984, v2985],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(4),
            multiplicity * (v2771),
            [1, 4],
            [v2986, v2987],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(3),
            multiplicity * (v2784),
            [3, 5, 6, 7],
            [v2988, v2989, v2990, v2991],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(4),
            Some(3),
            multiplicity * (v2790),
            [3, 4],
            [v2992, v2993],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(7),
            multiplicity * (v2802),
            [3, 5, 6, 7],
            [v2994, v2995, v2996, v2997],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(5),
            multiplicity * (v2808),
            [3, 4, 5, 6, 7],
            [v2998, v2999, v3000, v3001, v3002],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(2),
            multiplicity * (v2815),
            [1, 2],
            [v3003, v3004],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(0),
            multiplicity * (v2822),
            [0, 1],
            [v3005, v3006],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(1),
            Some(8),
            multiplicity * (v2826),
            [0, 1, 4, 5, 6, 7, 8, 9],
            [v3007, v3008, v3009, v3010, v3011, v3012, v3013, v3014],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(0),
            Some(8),
            multiplicity * (v2833),
            [0, 1, 4, 5, 6, 7, 8, 9],
            [v3015, v3016, v3017, v3018, v3019, v3020, v3021, v3022],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(1),
            Some(8),
            multiplicity * (v2841),
            [0, 1, 4, 5, 6, 7, 8, 9],
            [v3023, v3024, v3025, v3026, v3027, v3028, v3029, v3030],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(9),
            multiplicity * (v2851),
            [4, 5, 6, 7, 9],
            [v3031, v3032, v3033, v3034, v3035],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(9),
            multiplicity * (v2859),
            [4, 5, 6, 7, 9],
            [v3036, v3037, v3038, v3039, v3040],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(8),
            Some(9),
            multiplicity * (v2869),
            [8, 9],
            [v3041, v3042],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(8), Some(9), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            staged[184],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(9),
            Some(6),
            multiplicity * (v2879),
            [6, 9],
            [v3043, v3044],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(9), Some(6), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            staged[185],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(10),
            None,
            multiplicity * (v3056),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(10),
            None,
            multiplicity * (v2935),
            [10],
            [v3045],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(3),
            multiplicity * (v2939),
            [3, 5, 6, 7, 10],
            [v3046, v3047, v3048, v3049, v3050],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(5),
            multiplicity * (v2945),
            [3, 5, 6, 7, 10],
            [v3051, v3052, v3053, v3054, v3055],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(7),
            Some(3),
            multiplicity * (v2935),
            [10],
            [v3045],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(5),
            multiplicity * (v3057),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(3),
            multiplicity * (v3058),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(2),
            Some(3),
            multiplicity * (v3059),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(4),
            multiplicity * (v3060),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(5),
            multiplicity * (v3061),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(3),
            multiplicity * (v3062),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(3),
            multiplicity * (v3063),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(3),
            multiplicity * (v3064),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(9),
            multiplicity * (v3065),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(9),
            multiplicity * (v3066),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(9),
            multiplicity * (v3067),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(9),
            multiplicity * (v3068),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(8),
            multiplicity * (v3069),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(8),
            multiplicity * (v3070),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(5),
            multiplicity * (staged[186]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(5),
            multiplicity * (staged[187]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(8),
            multiplicity * (staged[188]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(9),
            multiplicity * (staged[189]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(6),
            multiplicity * (staged[190]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(8),
            multiplicity * (staged[191]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(6),
            multiplicity * (staged[192]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(9),
            multiplicity * (staged[193]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(6),
            multiplicity * (staged[194]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(6),
            multiplicity * (staged[195]),
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

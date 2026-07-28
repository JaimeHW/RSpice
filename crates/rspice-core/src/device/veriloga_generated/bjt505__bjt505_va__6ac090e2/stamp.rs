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
        let produced: [f64; 121] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let v0 = parameters[3];
                let v1 = 1e0f64;
                let v3 = 1.23e8f64;
                let v4 = 7.03e7f64;
                let v5 = 2.04e8f64;
                let v6 = 1.58e8f64;
                let v9 = parameters[33];
                let v11 = parameters[4];
                let v12 = 2.7315e2f64;
                let v14 = parameters[150];
                let v15 = 0e0f64;
                let v17 = 1e-12f64;
                let v19 = parameters[1];
                let v22 = parameters[134];
                let v24 = 2e0f64;
                let v25 = parameters[67];
                let v29 = parameters[115];
                let v32 = parameters[116];
                let v35 = parameters[114];
                let v37 = 5e-2f64;
                let v39 = 1e-1f64;
                let v55 = parameters[66];
                let v57 = parameters[72];
                let v61 = parameters[118];
                let v64 = parameters[119];
                let v67 = parameters[117];
                let v85 = parameters[71];
                let v87 = parameters[83];
                let v90 = 8.617086918058125e-5f64;
                let v93 = parameters[75];
                let v95 = parameters[98];
                let v96 = parameters[96];
                let v98 = parameters[122];
                let v100 = parameters[123];
                let v102 = 4e0f64;
                let v105 = parameters[121];
                let v107 = parameters[105];
                let v110 = parameters[103];
                let v112 = parameters[21];
                let v114 = 6e0f64;
                let v116 = parameters[113];
                let v118 = parameters[32];
                let v121 = parameters[110];
                let v123 = parameters[97];
                let v126 = parameters[111];
                let v128 = parameters[24];
                let v130 = parameters[107];
                let v132 = parameters[106];
                let v134 = parameters[108];
                let v138 = parameters[112];
                let v140 = parameters[23];
                let v143 = parameters[146];
                let v145 = parameters[141];
                let v147 = parameters[140];
                let v149 = 5e-1f64;
                let v150 = parameters[142];
                let v152 = 3.5e0f64;
                let v157 = parameters[120];
                let v161 = parameters[99];
                let v163 = parameters[87];
                let v164 = parameters[88];
                let v166 = parameters[100];
                let v168 = parameters[57];
                let v170 = parameters[58];
                let v172 = parameters[59];
                let v174 = -1e0f64;
                let v176 = 3e0f64;
                let v180 = 1e0f64;
                let v182 = parameters[74];
                let v185 = -1e0f64;
                let v187 = parameters[76];
                let v191 = parameters[92];
                let v193 = parameters[93];
                let v196 = parameters[34];
                let v198 = parameters[35];
                let v201 = parameters[36];
                let v203 = parameters[37];
                let v206 = parameters[8];
                let v208 = parameters[143];
                let v215 = parameters[144];
                let v217 = parameters[5];
                let v222 = parameters[84];
                let v231 = parameters[82];
                let v235 = parameters[81];
                let v243 = parameters[68];
                let v245 = parameters[77];
                let v247 = -1e0f64;
                let v248 = parameters[139];
                let v254 = parameters[85];
                let v256 = parameters[79];
                let v262 = parameters[6];
                let v268 = parameters[95];
                let v270 = parameters[94];
                let v272 = parameters[69];
                let v274 = parameters[78];
                let v276 = 0e0f64;
                let v278 = 0e0f64;
                let v280 = parameters[130];
                let v282 = parameters[131];
                let v286 = 0e0f64;
                let v287 = 0e0f64;
                let v298 = 0e0f64;
                let v299 = 0e0f64;
                let v300 = 0e0f64;
                let v301 = 0e0f64;
                let v302 = 0e0f64;
                let v308 = 0e0f64;
                let v309 = 0e0f64;
                let v310 = 0e0f64;
                let mut out131: f64 = 0.0;
                let mut out133: f64 = 0.0;
                let mut out135: f64 = 0.0;
                let mut out184: f64 = 0.0;
                let mut out194: f64 = 0.0;
                let mut out195: f64 = 0.0;
                let mut out209: f64 = 0.0;
                let mut out211: f64 = 0.0;
                let mut out212: f64 = 0.0;
                let mut out214: f64 = 0.0;
                let mut out221: f64 = 0.0;
                let mut out226: f64 = 0.0;
                let mut out229: f64 = 0.0;
                let mut out230: f64 = 0.0;
                let mut out234: f64 = 0.0;
                let mut out236: f64 = 0.0;
                let mut out242: f64 = 0.0;
                let mut out264: f64 = 0.0;
                let mut out265: f64 = 0.0;
                let mut out266: f64 = 0.0;
                let mut out267: f64 = 0.0;
                let mut out269: f64 = 0.0;
                let mut out271: f64 = 0.0;
                let mut out284: f64 = 0.0;
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
                let v23 = if v22 > v15 { 1.0 } else { 0.0 };
                let v27 = v24.powf((v24 - v25));
                let v28 = v1 / v27;
                let v36 = v35 + (((v29 * v13) * v13) / (v13 + v32));
                let v40 = (v36 - v37) / v39;
                let v41 = if v36 < v37 { 1.0 } else { 0.0 };
                let v53: f64;
                if v41 != 0.0 {
                    let v46 = v37 + (v39 * ((v1 + (v40.exp())).ln()));
                    v53 = v46;
                } else {
                    let v52 = v36 + (v39 * ((v1 + ((-v40).exp())).ln()));
                    v53 = v52;
                }
                let v54 = v1 / v35;
                let v56 = v1 / v55;
                let v59 = v24.powf((v24 - v57));
                let v60 = v1 / v59;
                let v68 = v67 + (((v61 * v13) * v13) / (v13 + v64));
                let v70 = (v68 - v37) / v39;
                let v71 = if v68 < v37 { 1.0 } else { 0.0 };
                let v83: f64;
                if v71 != 0.0 {
                    let v76 = v37 + (v39 * ((v1 + (v70.exp())).ln()));
                    v83 = v76;
                } else {
                    let v82 = v68 + (v39 * ((v1 + ((-v70).exp())).ln()));
                    v83 = v82;
                }
                let v84 = v1 / v67;
                let v86 = v1 / v85;
                let v89 = v1 - (v1 / v87);
                let v92 = v1 / (v90 * v13);
                let v94 = v1 - v93;
                let v97 = v95 - v96;
                let v99 = if v98 != v15 { 1.0 } else { 0.0 };
                let v101 = if v100 != v15 { 1.0 } else { 0.0 };
                let v106 = ((v102 - v95) - v96) + v105;
                let v108 = -v107;
                let v109 = v1 - v95;
                let v111 = v1 - v110;
                let v115 = v114 - (v24 * v112);
                let v117 = -v116;
                let v120 = v114 - (v24 * v118);
                let v122 = -v121;
                let v125 = (v102 - v123) + v105;
                let v127 = -v126;
                let v129 = if v128 == v1 { 1.0 } else { 0.0 };
                if v129 != 0.0 {
                    let v131 = -v130;
                    out131 = v131;
                    let v133 = -v132;
                    out133 = v133;
                    let v135 = -v134;
                    out135 = v135;
                } else {
                }
                let v137 = (v102 - v110) + v105;
                let v139 = -v138;
                let v142 = v114 - (v24 * v140);
                let v144 = v102 / v143;
                let v146 = v102 - v145;
                let v148 = -v147;
                let v153 = v152 - (v149 * v150);
                let v154 = v1 - v145;
                let v155 = v1 - v150;
                let v156 = v95 - v24;
                let v158 = -v157;
                let v160 = (v96 + v95) - v1;
                let v162 = v161 - v1;
                let v165 = v163 + v164;
                let v167 = v166 - v1;
                let v169 = if v168 > v15 { 1.0 } else { 0.0 };
                let v171 = if v170 > v15 { 1.0 } else { 0.0 };
                let v173 = if v172 > v15 { 1.0 } else { 0.0 };
                let v178 = v1 - (v176.powf((v174 / v25)));
                let v179 = v1 - v25;
                let v181 = v179 - v180;
                let v183 = if v182 == v1 { 1.0 } else { 0.0 };
                if v183 != 0.0 {
                } else {
                    let v184 = if v182 == v24 { 1.0 } else { 0.0 };
                    out184 = v184;
                }
                let v186 = v185 / v57;
                let v188 = v187 - v180;
                let v189 = v1 - v57;
                let v190 = v189 - v180;
                let v192 = if v191 == v15 { 1.0 } else { 0.0 };
                if v129 != 0.0 {
                } else {
                    let v194 = if v193 == v15 { 1.0 } else { 0.0 };
                    out194 = v194;
                    if v194 != 0.0 {
                    } else {
                        let v195 = v1 - v193;
                        out195 = v195;
                    }
                }
                let v200 = if (if v196 > v15 { 1.0 } else { 0.0 }) != 0.0 && (if v198 > v15 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v205 = if (if v201 > v15 { 1.0 } else { 0.0 }) != 0.0 && (if v203 > v15 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v207 = if v206 == v1 { 1.0 } else { 0.0 };
                if v207 != 0.0 {
                    let v209 = v208 * v24;
                    out209 = v209;
                    let v211 = (v1 - v208) * v24;
                    out211 = v211;
                } else {
                    let v212 = v208 * v24;
                    out212 = v212;
                    let v214 = (v1 - v208) * v24;
                    out214 = v214;
                }
                let v216 = v215 * v102;
                let v219 = if v9 > v15 { 1.0 } else { 0.0 };
                let v220 = if (if v217 > v15 { 1.0 } else { 0.0 }) != 0.0 && v219 != 0.0 { 1.0 } else { 0.0 };
                if v220 != 0.0 {
                    let v221 = v9 * v24;
                    out221 = v221;
                    if v207 != 0.0 {
                        let v226 = ((v1 - v208) * v9) * v24;
                        out226 = v226;
                    } else {
                        let v229 = ((v1 - v208) * v9) * v24;
                        out229 = v229;
                    }
                    let v230 = if v217 == v1 { 1.0 } else { 0.0 };
                    out230 = v230;
                } else {
                }
                let v223 = if v222 == v1 { 1.0 } else { 0.0 };
                if v223 != 0.0 {
                    let v234 = v1 / (v1 - (v89.powf(v231)));
                    out234 = v234;
                    let v236 = v89 * v235;
                    out236 = v236;
                    let v242 = (((v234 * v234) * (v89.powf((v231 - v1)))) * v231) / v235;
                    out242 = v242;
                } else {
                }
                let v244 = v1 - v243;
                let v246 = v1 - v245;
                let v251 = v1 - (v24.powf((v247 / v248)));
                let v252 = v1 - v248;
                let v253 = v252 - v180;
                let v255 = v1 / v254;
                let v257 = if v256 == v15 { 1.0 } else { 0.0 };
                let v261 = if (if (if v217 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v217 == v176 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v219 != 0.0 { 1.0 } else { 0.0 };
                if v261 != 0.0 {
                    if v257 != 0.0 {
                        let v264 = v149 * v9;
                        out264 = v264;
                    } else {
                        let v265 = v24 * v9;
                        out265 = v265;
                    }
                } else {
                }
                let v263 = if v262 == v1 { 1.0 } else { 0.0 };
                if v263 != 0.0 {
                    let v266 = -v25;
                    out266 = v266;
                    let v267 = v266 - v180;
                    out267 = v267;
                    let v269 = v1 - v268;
                    out269 = v269;
                    let v271 = v1 - v270;
                    out271 = v271;
                } else {
                }
                let v273 = v0 * v272;
                let v275 = v0 * v274;
                let v277: f64;
                if v171 != 0.0 {
                    v277 = v15;
                } else {
                    v277 = v276;
                }
                let v279: f64;
                if v173 != 0.0 {
                    v279 = v15;
                } else {
                    v279 = v278;
                }
                let v281 = if v280 > v15 { 1.0 } else { 0.0 };
                let v283 = if v282 == v1 { 1.0 } else { 0.0 };
                if v283 != 0.0 {
                } else {
                    let v284 = if v282 == v24 { 1.0 } else { 0.0 };
                    out284 = v284;
                }
                let v285 = if v9 == v15 { 1.0 } else { 0.0 };
                let v288: f64;
                let v289: f64;
                if v129 != 0.0 {
                    v288 = v286;
                    v289 = v15;
                } else {
                    v288 = v15;
                    v289 = v287;
                }
                let v290: f64;
                let v291: f64;
                let v292: f64;
                let v293: f64;
                let v294: f64;
                let v295: f64;
                let v296: f64;
                let v297: f64;
                if v171 != 0.0 {
                    let v303: f64;
                    let v304: f64;
                    let v305: f64;
                    let v306: f64;
                    let v307: f64;
                    if v173 != 0.0 {
                        v303 = v298;
                        v304 = v15;
                        v305 = v299;
                        v306 = v300;
                        v307 = v15;
                    } else {
                        v303 = v15;
                        v304 = v301;
                        v305 = v15;
                        v306 = v15;
                        v307 = v302;
                    }
                    v290 = v303;
                    v291 = v304;
                    v292 = v15;
                    v293 = v15;
                    v294 = v305;
                    v295 = v306;
                    v296 = v307;
                    v297 = v15;
                } else {
                    let v311: f64;
                    let v312: f64;
                    let v313: f64;
                    if v173 != 0.0 {
                        v311 = v308;
                        v312 = v15;
                        v313 = v309;
                    } else {
                        v311 = v15;
                        v312 = v310;
                        v313 = v15;
                    }
                    v290 = v15;
                    v291 = v15;
                    v292 = v311;
                    v293 = v312;
                    v294 = v15;
                    v295 = v15;
                    v296 = v15;
                    v297 = v313;
                }
                let v314 = if v19 != v1 { 1.0 } else { 0.0 };
            [v2, v10, v13, v16, v20, v21, v23, v27, v28, v41, v54, v56, v59, v60, v71, v84, v86, v92, v53, v83, v94, v97, v99, v101, v106, v108, v109, v111, v115, v117, v120, v122, v125, v127, v129, out131, out133, out135, v137, v139, v142, v144, v146, v148, v153, v154, v155, v156, v158, v160, v162, v165, v167, v7, v169, v171, v173, v178, v179, v183, out184, v186, v189, v192, out194, out195, v200, v205, v207, out209, out211, out212, out214, v216, v220, out221, out226, out229, out230, v223, out234, out236, out242, v8, v244, v246, v251, v252, v255, v257, v261, out264, out265, v263, out266, out269, out271, v273, v275, v281, v283, out284, v285, v290, v291, v292, v293, v314, v277, v279, v288, v289, v294, v295, v296, v297, v181, v188, v190, v253, out267]
        };
        self.canonical_staged[174] = produced[0];
        self.canonical_staged[108] = produced[1];
        self.canonical_staged[0] = produced[2];
        self.canonical_staged[175] = produced[3];
        self.canonical_staged[5] = produced[4];
        self.canonical_staged[40] = produced[5];
        self.canonical_staged[176] = produced[6];
        self.canonical_staged[81] = produced[7];
        self.canonical_staged[85] = produced[8];
        self.canonical_staged[177] = produced[9];
        self.canonical_staged[24] = produced[10];
        self.canonical_staged[25] = produced[11];
        self.canonical_staged[88] = produced[12];
        self.canonical_staged[92] = produced[13];
        self.canonical_staged[178] = produced[14];
        self.canonical_staged[26] = produced[15];
        self.canonical_staged[27] = produced[16];
        self.canonical_staged[1] = produced[17];
        self.canonical_staged[2] = produced[18];
        self.canonical_staged[3] = produced[19];
        self.canonical_staged[4] = produced[20];
        self.canonical_staged[6] = produced[21];
        self.canonical_staged[190] = produced[22];
        self.canonical_staged[192] = produced[23];
        self.canonical_staged[7] = produced[24];
        self.canonical_staged[8] = produced[25];
        self.canonical_staged[9] = produced[26];
        self.canonical_staged[10] = produced[27];
        self.canonical_staged[11] = produced[28];
        self.canonical_staged[12] = produced[29];
        self.canonical_staged[13] = produced[30];
        self.canonical_staged[14] = produced[31];
        self.canonical_staged[15] = produced[32];
        self.canonical_staged[16] = produced[33];
        self.canonical_staged[195] = produced[34];
        self.canonical_staged[17] = produced[35];
        self.canonical_staged[18] = produced[36];
        self.canonical_staged[19] = produced[37];
        self.canonical_staged[20] = produced[38];
        self.canonical_staged[21] = produced[39];
        self.canonical_staged[22] = produced[40];
        self.canonical_staged[23] = produced[41];
        self.canonical_staged[28] = produced[42];
        self.canonical_staged[29] = produced[43];
        self.canonical_staged[30] = produced[44];
        self.canonical_staged[31] = produced[45];
        self.canonical_staged[32] = produced[46];
        self.canonical_staged[33] = produced[47];
        self.canonical_staged[34] = produced[48];
        self.canonical_staged[35] = produced[49];
        self.canonical_staged[36] = produced[50];
        self.canonical_staged[37] = produced[51];
        self.canonical_staged[38] = produced[52];
        self.canonical_staged[39] = produced[53];
        self.canonical_staged[197] = produced[54];
        self.canonical_staged[199] = produced[55];
        self.canonical_staged[201] = produced[56];
        self.canonical_staged[47] = produced[57];
        self.canonical_staged[51] = produced[58];
        self.canonical_staged[203] = produced[59];
        self.canonical_staged[204] = produced[60];
        self.canonical_staged[53] = produced[61];
        self.canonical_staged[55] = produced[62];
        self.canonical_staged[205] = produced[63];
        self.canonical_staged[206] = produced[64];
        self.canonical_staged[73] = produced[65];
        self.canonical_staged[80] = produced[66];
        self.canonical_staged[86] = produced[67];
        self.canonical_staged[207] = produced[68];
        self.canonical_staged[95] = produced[69];
        self.canonical_staged[98] = produced[70];
        self.canonical_staged[100] = produced[71];
        self.canonical_staged[103] = produced[72];
        self.canonical_staged[106] = produced[73];
        self.canonical_staged[208] = produced[74];
        self.canonical_staged[109] = produced[75];
        self.canonical_staged[111] = produced[76];
        self.canonical_staged[114] = produced[77];
        self.canonical_staged[210] = produced[78];
        self.canonical_staged[209] = produced[79];
        self.canonical_staged[122] = produced[80];
        self.canonical_staged[120] = produced[81];
        self.canonical_staged[121] = produced[82];
        self.canonical_staged[126] = produced[83];
        self.canonical_staged[130] = produced[84];
        self.canonical_staged[137] = produced[85];
        self.canonical_staged[138] = produced[86];
        self.canonical_staged[141] = produced[87];
        self.canonical_staged[145] = produced[88];
        self.canonical_staged[211] = produced[89];
        self.canonical_staged[212] = produced[90];
        self.canonical_staged[155] = produced[91];
        self.canonical_staged[157] = produced[92];
        self.canonical_staged[213] = produced[93];
        self.canonical_staged[159] = produced[94];
        self.canonical_staged[160] = produced[95];
        self.canonical_staged[161] = produced[96];
        self.canonical_staged[162] = produced[97];
        self.canonical_staged[163] = produced[98];
        self.canonical_staged[214] = produced[99];
        self.canonical_staged[215] = produced[100];
        self.canonical_staged[216] = produced[101];
        self.canonical_staged[217] = produced[102];
        self.canonical_staged[223] = produced[103];
        self.canonical_staged[226] = produced[104];
        self.canonical_staged[228] = produced[105];
        self.canonical_staged[230] = produced[106];
        self.canonical_staged[218] = produced[107];
        self.canonical_staged[219] = produced[108];
        self.canonical_staged[220] = produced[109];
        self.canonical_staged[221] = produced[110];
        self.canonical_staged[222] = produced[111];
        self.canonical_staged[224] = produced[112];
        self.canonical_staged[225] = produced[113];
        self.canonical_staged[227] = produced[114];
        self.canonical_staged[229] = produced[115];
        self.canonical_staged[168] = produced[116];
        self.canonical_staged[169] = produced[117];
        self.canonical_staged[170] = produced[118];
        self.canonical_staged[172] = produced[119];
        self.canonical_staged[173] = produced[120];
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
        let produced: [f64; 110] = {
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
                let v13 = parameters[115];
                let v16 = parameters[116];
                let v19 = staged[2];
                let v21 = 5e-2f64;
                let v23 = 1e-1f64;
                let v38 = parameters[118];
                let v41 = parameters[119];
                let v44 = staged[3];
                let v61 = -3e0f64;
                let v64 = parameters[66];
                let v68 = parameters[105];
                let v86 = -3e0f64;
                let v89 = parameters[64];
                let v92 = parameters[110];
                let v110 = -3e0f64;
                let v113 = parameters[80];
                let v132 = -3e0f64;
                let v135 = parameters[71];
                let v154 = -3e0f64;
                let v174 = -3e0f64;
                let v177 = parameters[27];
                let v180 = parameters[109];
                let v198 = -3e0f64;
                let v201 = parameters[138];
                let v204 = parameters[140];
                let v225 = parameters[67];
                let v228 = parameters[72];
                let v230 = parameters[65];
                let v233 = parameters[139];
                let v235 = parameters[137];
                let v239 = staged[4];
                let v241 = parameters[75];
                let v244 = parameters[70];
                let v247 = parameters[97];
                let v250 = parameters[54];
                let v252 = staged[5];
                let v255 = staged[6];
                let v258 = parameters[56];
                let v260 = parameters[101];
                let v263 = parameters[55];
                let v267 = parameters[102];
                let v270 = parameters[57];
                let v272 = parameters[104];
                let v275 = parameters[58];
                let v277 = parameters[59];
                let v279 = parameters[99];
                let v282 = parameters[60];
                let v284 = staged[190];
                let v285 = parameters[122];
                let v288 = parameters[10];
                let v291 = 1e-3f64;
                let v295 = staged[192];
                let v308 = 6.931471805599453e-4f64;
                let v310 = parameters[123];
                let v313 = parameters[11];
                let v319 = parameters[124];
                let v322 = parameters[43];
                let v325 = 0e0f64;
                let v339 = 6.931471805599453e-4f64;
                let v341 = 1e-6f64;
                let v345 = 5e-7f64;
                let v350 = 5e-1f64;
                let v353 = staged[7];
                let v357 = parameters[9];
                let v359 = staged[8];
                let v364 = staged[9];
                let v367 = parameters[12];
                let v369 = staged[10];
                let v372 = parameters[30];
                let v374 = staged[11];
                let v377 = parameters[20];
                let v379 = staged[12];
                let v381 = parameters[21];
                let v385 = staged[13];
                let v388 = parameters[31];
                let v390 = staged[14];
                let v392 = parameters[32];
                let v396 = staged[15];
                let v398 = parameters[17];
                let v401 = parameters[16];
                let v403 = staged[16];
                let v408 = parameters[19];
                let v411 = parameters[18];
                let v416 = staged[195];
                let v417 = staged[17];
                let v421 = parameters[25];
                let v423 = staged[18];
                let v426 = parameters[28];
                let v428 = staged[19];
                let v432 = parameters[26];
                let v437 = staged[20];
                let v440 = parameters[29];
                let v442 = staged[21];
                let v446 = staged[22];
                let v449 = parameters[22];
                let v451 = parameters[23];
                let v455 = staged[23];
                let v458 = parameters[145];
                let v460 = parameters[146];
                let v465 = parameters[151];
                let v467 = parameters[153];
                let v471 = staged[24];
                let v473 = -5e-1f64;
                let v476 = parameters[35];
                let v485 = parameters[34];
                let v489 = staged[25];
                let v496 = staged[26];
                let v498 = -5e-1f64;
                let v501 = parameters[37];
                let v510 = parameters[36];
                let v514 = staged[27];
                let v521 = parameters[96];
                let v524 = parameters[14];
                let v527 = parameters[13];
                let v530 = staged[28];
                let v533 = parameters[133];
                let v535 = staged[29];
                let v539 = staged[30];
                let v542 = parameters[134];
                let v545 = staged[31];
                let v548 = parameters[135];
                let v550 = staged[32];
                let v553 = parameters[136];
                let v555 = staged[33];
                let v558 = parameters[86];
                let v560 = staged[34];
                let v564 = staged[35];
                let v567 = parameters[87];
                let v569 = staged[36];
                let v572 = parameters[88];
                let v575 = parameters[89];
                let v577 = staged[37];
                let v579 = staged[38];
                let v582 = parameters[90];
                let v584 = 3e2f64;
                let v586 = 5.25e2f64;
                let v588 = 7.2e-4f64;
                let v591 = 1.6e-6f64;
                let v595 = staged[39];
                let v597 = 1.081e0f64;
                let v600 = parameters[92];
                let v602 = staged[197];
                let v604 = staged[40];
                let v607 = staged[199];
                let v612 = staged[201];
                let v618 = staged[47];
                let v621 = staged[51];
                let v623 = 2e0f64;
                let v627 = staged[53];
                let v631 = staged[55];
                let v633 = 4e0f64;
                let v637 = 1e0f64;
                let v639 = staged[205];
                let v643 = parameters[15];
                let v650 = staged[207];
                let v651 = staged[95];
                let v655 = staged[98];
                let v657 = staged[100];
                let v661 = staged[103];
                let v665 = staged[106];
                let v667 = staged[208];
                let v668 = staged[109];
                let v670 = staged[111];
                let v674 = staged[114];
                let v678 = staged[210];
                let v680 = parameters[33];
                let v688 = staged[130];
                let v690 = parameters[68];
                let v692 = parameters[77];
                let v698 = staged[138];
                let v700 = staged[141];
                let v704 = staged[145];
                let v707 = parameters[85];
                let v713 = staged[211];
                let v715 = staged[212];
                let v717 = staged[155];
                let v719 = staged[157];
                let mut out293: f64 = 0.0;
                let mut out317: f64 = 0.0;
                let mut out605: f64 = 0.0;
                let mut out610: f64 = 0.0;
                let mut out615: f64 = 0.0;
                let mut out642: f64 = 0.0;
                let mut out645: f64 = 0.0;
                let mut out646: f64 = 0.0;
                let mut out652: f64 = 0.0;
                let mut out654: f64 = 0.0;
                let mut out656: f64 = 0.0;
                let mut out658: f64 = 0.0;
                let mut out660: f64 = 0.0;
                let mut out662: f64 = 0.0;
                let mut out669: f64 = 0.0;
                let mut out671: f64 = 0.0;
                let mut out673: f64 = 0.0;
                let mut out675: f64 = 0.0;
                let mut out677: f64 = 0.0;
                let mut out682: f64 = 0.0;
                let mut out686: f64 = 0.0;
                let mut out714: f64 = 0.0;
                let mut out716: f64 = 0.0;
                let mut out718: f64 = 0.0;
                let mut out721: f64 = 0.0;
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
                let v206 = (((v198 * v6) * v12) + (v201 * v4)) + (v67 * v204);
                let v208 = (v21 - v206) / v6;
                let v209 = if v21 < v206 { 1.0 } else { 0.0 };
                let v221: f64;
                if v209 != 0.0 {
                    let v214 = v206 + (v6 * ((v7 + (v208.exp())).ln()));
                    v221 = v214;
                } else {
                    let v220 = v21 + (v6 * ((v7 + ((-v208).exp())).ln()));
                    v221 = v220;
                }
                let v222 = v7 / v85;
                let v223 = v7 / v173;
                let v226 = (v64 * v222).powf(v225);
                let v229 = (v135 * v223).powf(v228);
                let v231 = v230 * v226;
                let v236 = v235 * ((v201 / v221).powf(v233));
                let v242 = (v239 * ((v135 / v153).powf(v228))) + v241;
                let v243 = v7 / v242;
                let v245 = v244 * v242;
                let v246 = v241 * v243;
                let v251 = v250 * ((v12 * v247).exp());
                let v253 = if v251 < v252 { 1.0 } else { 0.0 };
                let v254: f64;
                if v253 != 0.0 {
                    v254 = v252;
                } else {
                    v254 = v251;
                }
                let v259 = v258 * ((v12 * v255).exp());
                let v264 = v263 * ((v12 * v260).exp());
                let v265 = if v264 < v252 { 1.0 } else { 0.0 };
                let v266: f64;
                if v265 != 0.0 {
                    v266 = v252;
                } else {
                    v266 = v264;
                }
                let v271 = v270 * ((v12 * v267).exp());
                let v274 = (v12 * v272).exp();
                let v276 = v275 * v274;
                let v278 = v277 * v274;
                let v283 = v282 * ((v12 * v279).exp());
                let v294: f64;
                if v284 != 0.0 {
                    let v289 = v288 * (v7 + (v11 * v285));
                    let v292 = (v289 - v7) / v291;
                    let v293 = if v289 < v7 { 1.0 } else { 0.0 };
                    out293 = v293;
                    let v307: f64;
                    if v293 != 0.0 {
                        let v300 = v7 + (v291 * ((v7 + (v292.exp())).ln()));
                        v307 = v300;
                    } else {
                        let v306 = v289 + (v291 * ((v7 + ((-v292).exp())).ln()));
                        v307 = v306;
                    }
                    let v309 = v307 - v308;
                    v294 = v309;
                } else {
                    v294 = v288;
                }
                let v318: f64;
                if v295 != 0.0 {
                    let v314 = v313 * (v7 + (v11 * v310));
                    let v316 = (v314 - v7) / v291;
                    let v317 = if v314 < v7 { 1.0 } else { 0.0 };
                    out317 = v317;
                    let v338: f64;
                    if v317 != 0.0 {
                        let v331 = v7 + (v291 * ((v7 + (v316.exp())).ln()));
                        v338 = v331;
                    } else {
                        let v337 = v314 + (v291 * ((v7 + ((-v316).exp())).ln()));
                        v338 = v337;
                    }
                    let v340 = v338 - v339;
                    v318 = v340;
                } else {
                    v318 = v313;
                }
                let v323 = v322 * (v7 + (v319 * v11));
                let v324 = v323 * v323;
                let v326 = if v323 < v325 { 1.0 } else { 0.0 };
                let v352: f64;
                if v326 != 0.0 {
                    let v346 = v345 / (((v324 + v341).sqrt()) - v323);
                    v352 = v346;
                } else {
                    let v351 = v350 * (((v324 + v341).sqrt()) + v323);
                    v352 = v351;
                }
                let v363 = (v357 * (((v12 * v353) / v294).exp())) * (((v359 * v10) / v294).exp());
                let v368 = v367 * ((v12 * v364).exp());
                let v373 = v372 * ((v12 * v369).exp());
                let v380 = v379 * v10;
                let v384 = (v377 * ((v12 * v374).exp())) * ((v380 / v381).exp());
                let v395 = (v388 * ((v12 * v385).exp())) * (((v390 * v10) / v392).exp());
                let v397 = v12 * v396;
                let v404 = v403 * v10;
                let v407 = (v401 * ((v397 / v398).exp())) * ((v404 / v398).exp());
                let v415 = (v411 * ((v397 / v408).exp())) * ((v404 / v408).exp());
                let v434: f64;
                let v435: f64;
                let v436: f64;
                if v416 != 0.0 {
                    let v422 = v421 * (((v417 * v10) / v398).exp());
                    let v427 = v426 * ((v423 * v10).exp());
                    let v433 = v432 * (((v428 * v10) / v408).exp());
                    v434 = v422;
                    v435 = v427;
                    v436 = v433;
                } else {
                    v434 = v325;
                    v435 = v325;
                    v436 = v325;
                }
                let v445 = (v440 * ((v12 * v437).exp())) * ((v442 * v10).exp());
                let v454 = (v449 * ((v12 * v446).exp())) * ((v380 / v451).exp());
                let v463 = (v458 * ((v12 * v455).exp())) * ((v380 / v460).exp());
                let v470 = (v465 * (v4.sqrt())) * ((v467 * v11).exp());
                let v474 = (v37 * v471).powf(v473);
                let v475 = v7 / v226;
                let v484 = (((((((v476 * v37) * v37) * v474) * v475) * v64) * v222) * v471) * v471;
                let v495 = ((((((v485 * v474) * v85) * v85) * v489) * v489) * v226) * ((v476 - v484).exp());
                let v499 = (v60 * v496).powf(v498);
                let v509 = (((((((v501 * v60) * v60) * v499) * (v7 / v229)) * v135) * v223) * v496) * v496;
                let v520 = ((((((v510 * v499) * v173) * v173) * v514) * v514) * v229) * ((v501 - v509).exp());
                let v523 = (v12 * v521).exp();
                let v526 = (v524 * v523) * v243;
                let v529 = (v527 * v523) * v475;
                let v537 = (v535 * v10).exp();
                let v538 = (v533 * ((v12 * v530).exp())) * v537;
                let v544 = (v542 * ((v12 * v539).exp())) * v537;
                let v549 = v548 * ((v12 * v545).exp());
                let v554 = v553 * ((v12 * v550).exp());
                let v563 = (v558 * ((v12 * v555).exp())) * ((v560 * v10).exp());
                let v568 = v567 * ((v12 * v564).exp());
                let v573 = v572 * ((v12 * v569).exp());
                let v574 = v568 + v573;
                let v578 = (v575 * v574) / v577;
                let v583 = v582 * ((v12 * v579).exp());
                let v585 = v2 - v584;
                let v587 = if v2 < v586 { 1.0 } else { 0.0 };
                let v599: f64;
                if v587 != 0.0 {
                    let v596 = v595 * ((v7 + (v588 * v585)) - ((v591 * v585) * v585));
                    v599 = v596;
                } else {
                    let v598 = v595 * v597;
                    v599 = v598;
                }
                let v601 = v600 * v523;
                let v606: f64;
                if v602 != 0.0 {
                    let v603 = v7 / v271;
                    let v605 = if v603 > v604 { 1.0 } else { 0.0 };
                    out605 = v605;
                    let v608: f64;
                    if v605 != 0.0 {
                        v608 = v604;
                    } else {
                        v608 = v603;
                    }
                    v606 = v608;
                } else {
                    v606 = v325;
                }
                let v611: f64;
                if v607 != 0.0 {
                    let v609 = v7 / v276;
                    let v610 = if v609 > v604 { 1.0 } else { 0.0 };
                    out610 = v610;
                    let v613: f64;
                    if v610 != 0.0 {
                        v613 = v604;
                    } else {
                        v613 = v609;
                    }
                    v611 = v613;
                } else {
                    v611 = v325;
                }
                let v616: f64;
                if v612 != 0.0 {
                    let v614 = v7 / v278;
                    let v615 = if v614 > v604 { 1.0 } else { 0.0 };
                    out615 = v615;
                    let v617: f64;
                    if v615 != 0.0 {
                        v617 = v604;
                    } else {
                        v617 = v614;
                    }
                    v616 = v617;
                } else {
                    v616 = v325;
                }
                let v619 = v85 * v618;
                let v620 = v23 * v85;
                let v622 = v85 / v621;
                let v625 = v7 - v246;
                let v626 = (v623 - v246) / v625;
                let v630 = v153 * (v7 - (v626.powf(v627)));
                let v632 = v153 / v631;
                let v635 = (v633 * v363) / v368;
                let v636 = v7 / v318;
                let v638 = v636 - v637;
                if v639 != 0.0 {
                } else {
                    let v642 = ((v601 * v8).exp()) - v7;
                    out642 = v642;
                }
                let v644 = v643 * v363;
                if v416 != 0.0 {
                    let v645 = v434 * v623;
                    out645 = v645;
                } else {
                }
                if v416 != 0.0 {
                    let v646 = v436 * v623;
                    out646 = v646;
                } else {
                }
                let v647 = v623 * v445;
                let v649 = (v633 * v445) / v373;
                if v650 != 0.0 {
                    let v652 = v651 * v538;
                    out652 = v652;
                    let v654 = v633 * (v538 / v549);
                    out654 = v654;
                    let v656 = v655 * v538;
                    out656 = v656;
                } else {
                    let v658 = v657 * v538;
                    out658 = v658;
                    let v660 = v633 * (v538 / v549);
                    out660 = v660;
                    let v662 = v661 * v538;
                    out662 = v662;
                }
                let v663 = v623 * v544;
                let v666 = v665 * (v544 / v554);
                if v667 != 0.0 {
                    let v669 = v668 * v445;
                    out669 = v669;
                    if v650 != 0.0 {
                        let v671 = v670 * v538;
                        out671 = v671;
                        let v673 = (v633 * v538) / v549;
                        out673 = v673;
                    } else {
                        let v675 = v674 * v538;
                        out675 = v675;
                        let v677 = (v633 * v538) / v549;
                        out677 = v677;
                    }
                    if v678 != 0.0 {
                        let v682 = (v680 * (v445 + v538)) * v271;
                        out682 = v682;
                        let v686 = v6 * (v623 - ((v682 * v8).ln()));
                        out686 = v686;
                    } else {
                    }
                } else {
                }
                let v687 = v623 * v6;
                let v689 = v688 * v231;
                let v691 = v690 * v231;
                let v693 = v692 * v245;
                let v694 = v568 * v368;
                let v695 = v350 * v694;
                let v696 = v23 * v153;
                let v697 = v23 * v221;
                let v699 = v221 * v698;
                let v701 = v221 / v700;
                let v706 = (v563 * v368) * ((v363 / v368).powf(v704));
                let v708 = v707 * v6;
                let v711 = ((v633 * v573) * v6) / v283;
                let v712 = v350 * v711;
                if v713 != 0.0 {
                    let v714 = v578 * v350;
                    out714 = v714;
                } else {
                    let v716 = v647 * v583;
                    out716 = v716;
                }
                if v715 != 0.0 {
                    if v713 != 0.0 {
                        let v718 = v717 * v578;
                        out718 = v718;
                    } else {
                        let v721 = (v719 * v445) * v583;
                        out721 = v721;
                    }
                } else {
                }
            [v6, v8, v25, v48, v73, v97, v119, v141, v161, v185, v209, v222, v223, v221, v236, v153, v245, v246, v253, v259, v265, v271, v283, out293, out317, v326, v294, v363, v384, v395, v407, v415, v454, v463, v470, v37, v484, v495, v60, v509, v520, v526, v529, v568, v574, v587, v601, out605, out610, out615, v109, v619, v620, v622, v625, v626, v630, v632, v635, v636, out642, v644, v197, out645, v435, out646, v647, v649, out652, out654, out656, out658, out660, out662, v663, v666, out669, out671, out673, out675, out677, out682, out686, v687, v352, v599, v266, v254, v689, v691, v693, v694, v695, v696, v697, v699, v701, v706, v708, v711, v712, out714, v131, out716, out718, out721, v606, v611, v616, v638]
        };
        self.canonical_staged[44] = produced[0];
        self.canonical_staged[41] = produced[1];
        self.canonical_staged[179] = produced[2];
        self.canonical_staged[180] = produced[3];
        self.canonical_staged[181] = produced[4];
        self.canonical_staged[182] = produced[5];
        self.canonical_staged[183] = produced[6];
        self.canonical_staged[184] = produced[7];
        self.canonical_staged[185] = produced[8];
        self.canonical_staged[186] = produced[9];
        self.canonical_staged[187] = produced[10];
        self.canonical_staged[50] = produced[11];
        self.canonical_staged[87] = produced[12];
        self.canonical_staged[142] = produced[13];
        self.canonical_staged[144] = produced[14];
        self.canonical_staged[46] = produced[15];
        self.canonical_staged[136] = produced[16];
        self.canonical_staged[59] = produced[17];
        self.canonical_staged[188] = produced[18];
        self.canonical_staged[123] = produced[19];
        self.canonical_staged[189] = produced[20];
        self.canonical_staged[118] = produced[21];
        self.canonical_staged[45] = produced[22];
        self.canonical_staged[191] = produced[23];
        self.canonical_staged[193] = produced[24];
        self.canonical_staged[194] = produced[25];
        self.canonical_staged[42] = produced[26];
        self.canonical_staged[67] = produced[27];
        self.canonical_staged[76] = produced[28];
        self.canonical_staged[78] = produced[29];
        self.canonical_staged[70] = produced[30];
        self.canonical_staged[74] = produced[31];
        self.canonical_staged[77] = produced[32];
        self.canonical_staged[79] = produced[33];
        self.canonical_staged[68] = produced[34];
        self.canonical_staged[83] = produced[35];
        self.canonical_staged[82] = produced[36];
        self.canonical_staged[84] = produced[37];
        self.canonical_staged[90] = produced[38];
        self.canonical_staged[89] = produced[39];
        self.canonical_staged[91] = produced[40];
        self.canonical_staged[63] = produced[41];
        self.canonical_staged[62] = produced[42];
        self.canonical_staged[167] = produced[43];
        self.canonical_staged[152] = produced[44];
        self.canonical_staged[196] = produced[45];
        self.canonical_staged[64] = produced[46];
        self.canonical_staged[198] = produced[47];
        self.canonical_staged[200] = produced[48];
        self.canonical_staged[202] = produced[49];
        self.canonical_staged[43] = produced[50];
        self.canonical_staged[48] = produced[51];
        self.canonical_staged[49] = produced[52];
        self.canonical_staged[52] = produced[53];
        self.canonical_staged[58] = produced[54];
        self.canonical_staged[57] = produced[55];
        self.canonical_staged[54] = produced[56];
        self.canonical_staged[56] = produced[57];
        self.canonical_staged[60] = produced[58];
        self.canonical_staged[61] = produced[59];
        self.canonical_staged[65] = produced[60];
        self.canonical_staged[66] = produced[61];
        self.canonical_staged[69] = produced[62];
        self.canonical_staged[71] = produced[63];
        self.canonical_staged[72] = produced[64];
        self.canonical_staged[75] = produced[65];
        self.canonical_staged[93] = produced[66];
        self.canonical_staged[94] = produced[67];
        self.canonical_staged[96] = produced[68];
        self.canonical_staged[97] = produced[69];
        self.canonical_staged[99] = produced[70];
        self.canonical_staged[101] = produced[71];
        self.canonical_staged[102] = produced[72];
        self.canonical_staged[104] = produced[73];
        self.canonical_staged[105] = produced[74];
        self.canonical_staged[107] = produced[75];
        self.canonical_staged[110] = produced[76];
        self.canonical_staged[112] = produced[77];
        self.canonical_staged[113] = produced[78];
        self.canonical_staged[115] = produced[79];
        self.canonical_staged[116] = produced[80];
        self.canonical_staged[119] = produced[81];
        self.canonical_staged[117] = produced[82];
        self.canonical_staged[124] = produced[83];
        self.canonical_staged[125] = produced[84];
        self.canonical_staged[127] = produced[85];
        self.canonical_staged[128] = produced[86];
        self.canonical_staged[129] = produced[87];
        self.canonical_staged[131] = produced[88];
        self.canonical_staged[132] = produced[89];
        self.canonical_staged[133] = produced[90];
        self.canonical_staged[149] = produced[91];
        self.canonical_staged[134] = produced[92];
        self.canonical_staged[135] = produced[93];
        self.canonical_staged[140] = produced[94];
        self.canonical_staged[139] = produced[95];
        self.canonical_staged[143] = produced[96];
        self.canonical_staged[147] = produced[97];
        self.canonical_staged[146] = produced[98];
        self.canonical_staged[150] = produced[99];
        self.canonical_staged[148] = produced[100];
        self.canonical_staged[151] = produced[101];
        self.canonical_staged[153] = produced[102];
        self.canonical_staged[154] = produced[103];
        self.canonical_staged[156] = produced[104];
        self.canonical_staged[158] = produced[105];
        self.canonical_staged[164] = produced[106];
        self.canonical_staged[165] = produced[107];
        self.canonical_staged[166] = produced[108];
        self.canonical_staged[171] = produced[109];
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
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 13137 => 0usize, 13143 => 1usize, 13153 => 2usize, 13159 => 3usize, 13165 => 4usize, 13173 => 5usize, 13181 => 6usize, 13201 => 7usize, 13220 => 8usize, 13491 => 9usize, _ => usize::MAX };
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
            let v2 = staged[176];
            let v4 = staged[195];
            let v5 = staged[199];
            let v6 = staged[201];
            let v7 = node_potentials[6];
            let v8 = node_potentials[7];
            let v10 = Lanes([1e0f64; 1]);
            let v12 = Lanes([1e0f64; 1]);
            let v15 = parameters[3];
            let v18 = node_potentials[8];
            let v21 = Lanes([1e0f64; 1]);
            let v26 = node_potentials[4];
            let v29 = Lanes([1e0f64; 1]);
            let v34 = node_potentials[5];
            let v36 = Lanes([1e0f64; 1]);
            let v48 = node_potentials[3];
            let v50 = Lanes([1e0f64; 1]);
            let v62 = node_potentials[2];
            let v64 = Lanes([1e0f64; 1]);
            let v70 = node_potentials[1];
            let v72 = Lanes([1e0f64; 1]);
            let v84 = node_potentials[0];
            let v87 = Lanes([1e0f64; 1]);
            let v92 = node_potentials[10];
            let v94 = Lanes([1e0f64; 1]);
            let v100 = node_potentials[9];
            let v102 = Lanes([1e0f64; 1]);
            let v121 = -1e0f64;
            let v146 = staged[41];
            let v149 = parameters[147];
            let v155 = 1e0f64;
            let v163 = staged[42];
            let v248 = staged[43];
            let v298 = 4e0f64;
            let v303 = 2e0f64;
            let v305 = 1e0f64;
            let v315 = 2e0f64;
            let v323 = parameters[149];
            let v325 = Lanes([0e0f64; 2]);
            let v343 = staged[44];
            let v349 = staged[45];
            let v353 = 1e2f64;
            let v362 = 1e-5f64;
            let v366 = 1e-40f64;
            let v386 = staged[48];
            let v388 = staged[49];
            let v401 = 5e-1f64;
            let v418 = 2e-1f64;
            let v448 = parameters[62];
            let v449 = parameters[61];
            let v469 = parameters[63];
            let v496 = -1e0f64;
            let v594 = parameters[148];
            let v596 = Lanes([0e0f64; 3]);
            let v627 = parameters[73];
            let v629 = staged[46];
            let v630 = 1e-1f64;
            let v710 = staged[50];
            let v715 = staged[51];
            let v717 = staged[168];
            let v723 = staged[52];
            let v728 = 3e0f64;
            let v733 = staged[203];
            let v735 = staged[204];
            let v738 = staged[54];
            let v779 = parameters[76];
            let v781 = staged[169];
            let v789 = staged[55];
            let v791 = staged[170];
            let v801 = staged[56];
            let v804 = staged[57];
            let v815 = staged[58];
            let v818 = staged[59];
            let v824 = staged[60];
            let v837 = staged[61];
            let v839 = staged[171];
            let v855 = staged[205];
            let v856 = staged[62];
            let v860 = staged[63];
            let v870 = staged[64];
            let v891 = staged[65];
            let v900 = 1.0000000000000002e-2f64;
            let v908 = 5.000000000000001e-3f64;
            let v935 = staged[66];
            let v938 = staged[67];
            let v949 = 1e-4f64;
            let v975 = parameters[152];
            let v989 = staged[68];
            let v992 = parameters[154];
            let v994 = 1e-3f64;
            let v1022 = parameters[155];
            let v1034 = parameters[17];
            let v1047 = staged[69];
            let v1051 = staged[206];
            let v1056 = parameters[19];
            let v1071 = 1e3f64;
            let v1073 = 4e1f64;
            let v1079 = 2.3538526683702e17f64;
            let v1085 = staged[70];
            let v1088 = staged[71];
            let v1116 = staged[72];
            let v1136 = staged[73];
            let v1144 = parameters[93];
            let v1175 = staged[74];
            let v1180 = parameters[21];
            let v1196 = staged[75];
            let v1223 = staged[76];
            let v1226 = parameters[23];
            let v1240 = staged[77];
            let v1243 = parameters[32];
            let v1257 = staged[78];
            let v1260 = parameters[146];
            let v1274 = staged[79];
            let v1277 = staged[80];
            let v1281 = staged[81];
            let v1288 = staged[82];
            let v1292 = Lanes([0e0f64; 2]);
            let v1296 = staged[86];
            let v1312 = 1e-30f64;
            let v1318 = -2e0f64;
            let v1319 = parameters[67];
            let v1337 = 6e0f64;
            let v1355 = 1.6666666666666666e-1f64;
            let v1362 = staged[83];
            let v1369 = -1e-3f64;
            let v1378 = 3.333333333333333e-1f64;
            let v1381 = 2.5e-1f64;
            let v1396 = staged[84];
            let v1410 = staged[85];
            let v1435 = staged[87];
            let v1446 = staged[88];
            let v1453 = staged[89];
            let v1485 = staged[93];
            let v1488 = staged[94];
            let v1501 = staged[207];
            let v1519 = -2e0f64;
            let v1520 = parameters[72];
            let v1561 = staged[90];
            let v1568 = -1e-3f64;
            let v1593 = staged[91];
            let v1607 = staged[92];
            let v1636 = staged[96];
            let v1639 = parameters[144];
            let v1645 = staged[97];
            let v1662 = staged[99];
            let v1683 = staged[101];
            let v1686 = staged[102];
            let v1699 = staged[104];
            let v1721 = staged[105];
            let v1724 = staged[107];
            let v1741 = staged[208];
            let v1742 = staged[108];
            let v1748 = staged[110];
            let v1763 = Lanes([0e0f64; 9]);
            let v1774 = staged[209];
            let v1779 = staged[112];
            let v1787 = staged[113];
            let v1800 = staged[115];
            let v1803 = staged[116];
            let v1819 = staged[210];
            let v1820 = staged[117];
            let v1837 = 1.21e-2f64;
            let v1845 = 6.05e-3f64;
            let v1864 = staged[118];
            let v1867 = staged[119];
            let v1880 = -1e0f64;
            let v1883 = -1e0f64;
            let v1890 = -1e0f64;
            let v1893 = Lanes([0e0f64; 3]);
            let v1929 = 1e-12f64;
            let v1935 = -1e0f64;
            let v1940 = 5e-13f64;
            let v1950 = -1e0f64;
            let v1959 = staged[120];
            let v1961 = parameters[81];
            let v1964 = parameters[82];
            let v1977 = staged[121];
            let v1980 = staged[122];
            let v1984 = 1.0000000000000002e-2f64;
            let v1992 = 5.000000000000001e-3f64;
            let v2012 = staged[123];
            let v2017 = staged[5];
            let v2019 = Lanes([0e0f64; 4]);
            let v2025 = staged[124];
            let v2037 = parameters[39];
            let v2042 = parameters[44];
            let v2050 = parameters[42];
            let v2072 = staged[125];
            let v2074 = parameters[41];
            let v2092 = parameters[40];
            let v2104 = parameters[46];
            let v2106 = parameters[45];
            let v2124 = parameters[7];
            let v2175 = parameters[47];
            let v2244 = 1e-7f64;
            let v2251 = staged[126];
            let v2252 = staged[127];
            let v2311 = parameters[48];
            let v2319 = parameters[49];
            let v2333 = parameters[52];
            let v2338 = parameters[51];
            let v2371 = parameters[50];
            let v2398 = parameters[53];
            let v2402 = staged[128];
            let v2418 = staged[129];
            let v2434 = 1e-6f64;
            let v2482 = staged[131];
            let v2531 = staged[132];
            let v2534 = staged[133];
            let v2537 = staged[134];
            let v2553 = staged[135];
            let v2605 = staged[136];
            let v2608 = staged[137];
            let v2669 = parameters[33];
            let v2672 = staged[139];
            let v2674 = staged[140];
            let v2702 = staged[142];
            let v2707 = staged[141];
            let v2709 = staged[172];
            let v2715 = staged[143];
            let v2724 = staged[144];
            let v2727 = staged[146];
            let v2740 = staged[147];
            let v2743 = staged[148];
            let v2754 = staged[211];
            let v2755 = staged[149];
            let v2758 = staged[150];
            let v2763 = staged[151];
            let v2766 = staged[152];
            let v2769 = staged[153];
            let v2771 = parameters[91];
            let v2779 = staged[212];
            let v2789 = staged[154];
            let v2810 = staged[213];
            let v2842 = staged[156];
            let v2866 = staged[158];
            let v2881 = staged[159];
            let v2883 = staged[173];
            let v2889 = Lanes([0e0f64; 5]);
            let v2900 = parameters[1];
            let v2996 = staged[160];
            let v2999 = parameters[95];
            let v3005 = parameters[94];
            let v3010 = staged[161];
            let v3049 = -1e0f64;
            let v3077 = ddt_scale();
            let v3121 = staged[162];
            let v3130 = staged[163];
            let v3145 = staged[164];
            let v3183 = staged[165];
            let v3188 = Lanes([0e0f64; 2]);
            let v3193 = staged[166];
            let v3198 = Lanes([0e0f64; 2]);
            let v3207 = staged[214];
            let v3213 = 0e0f64;
            let v3227 = staged[167];
            let v3236 = staged[215];
            let v3239 = staged[216];
            let v3244 = parameters[132];
            let v3255 = node_potentials[11];
            let v3257 = Lanes([1e0f64; 1]);
            let v3448 = 0e0f64;
            let v3449 = 0e0f64;
            let v3450 = 0e0f64;
            let v3451 = 0e0f64;
            let v3452 = 0e0f64;
            let v3453 = 0e0f64;
            let v3454 = 0e0f64;
            let v3455 = 0e0f64;
            let v3456 = 0e0f64;
            let v3457 = 0e0f64;
            let v3458 = 0e0f64;
            let v3459 = 0e0f64;
            let v3460 = 0e0f64;
            let v3461 = 0e0f64;
            let v3462 = 0e0f64;
            let v3463 = 0e0f64;
            let v3464 = 0e0f64;
            let v3465 = 0e0f64;
            let v1 = ctx.simparam_or("gmin", v0);
            let v3: f64;
            if v2 != 0.0 {
                v3 = v1;
            } else {
                v3 = v0;
            }
            let v16 = v15 * (v7 - v8);
            let v17 = ((Lanes([v10[0], 0.0])) - (Lanes([0.0, v12[0]]))) * v15;
            let v24 = v15 * (v7 - v18);
            let v25 = ((Lanes([v10[0], 0.0])) - (Lanes([0.0, v21[0]]))) * v15;
            let v32 = v15 * (v7 - v26);
            let v33 = ((Lanes([0.0, v10[0]])) - (Lanes([v29[0], 0.0]))) * v15;
            let v40 = v15 * (v34 - v26);
            let v41 = ((Lanes([0.0, v36[0]])) - (Lanes([v29[0], 0.0]))) * v15;
            let v46 = v15 * (v34 - v7);
            let v47 = ((Lanes([v36[0], 0.0])) - (Lanes([0.0, v10[0]]))) * v15;
            let v54 = v15 * (v48 - v8);
            let v55 = ((Lanes([v50[0], 0.0])) - (Lanes([0.0, v12[0]]))) * v15;
            let v60 = v15 * (v8 - v18);
            let v61 = ((Lanes([v12[0], 0.0])) - (Lanes([0.0, v21[0]]))) * v15;
            let v68 = v15 * (v62 - v26);
            let v69 = ((Lanes([v64[0], 0.0])) - (Lanes([0.0, v29[0]]))) * v15;
            let v76 = v15 * (v70 - v34);
            let v77 = ((Lanes([v72[0], 0.0])) - (Lanes([0.0, v36[0]]))) * v15;
            let v82 = v15 * (v70 - v62);
            let v83 = ((Lanes([v72[0], 0.0])) - (Lanes([0.0, v64[0]]))) * v15;
            let v90 = v15 * (v70 - v84);
            let v91 = ((Lanes([0.0, v72[0]])) - (Lanes([v87[0], 0.0]))) * v15;
            let v98 = v15 * (v92 - v8);
            let v99 = ((Lanes([0.0, v94[0]])) - (Lanes([v12[0], 0.0]))) * v15;
            let v106 = v15 * (v100 - v92);
            let v107 = ((Lanes([v102[0], 0.0])) - (Lanes([0.0, v94[0]]))) * v15;
            let v111 = (Lanes([v47[0], v47[1], 0.0])) + (Lanes([0.0, v25[0], v25[1]]));
            let v115 = (Lanes([v111[0], v111[1], 0.0, v111[2]])) - (Lanes([0.0, 0.0, v61[0], v61[1]]));
            let v116 = ((v46 + v24) - v60) - v98;
            let v119 = (Lanes([v115[0], v115[1], v115[2], v115[3], 0.0])) - (Lanes([0.0, 0.0, v99[0], 0.0, v99[1]]));
            let v122 = v91 * v121;
            let v126 = (Lanes([v122[0], v122[1], 0.0])) + (Lanes([0.0, v77[0], v77[1]]));
            let v130 = (Lanes([v126[0], v126[1], v126[2], 0.0, 0.0, 0.0, 0.0])) + (Lanes([0.0, 0.0, v119[0], v119[1], v119[2], v119[3], v119[4]]));
            let v131 = (((-v90) + v76) + v116) - v106;
            let v134 = (Lanes([v130[0], v130[1], v130[2], v130[3], v130[4], v130[5], 0.0, v130[6]])) - (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, v107[0], v107[1]]));
            let v135 = v90 + v131;
            let v137 = (Lanes([v91[0], v91[1], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + v134;
            let v138 = v54 - v98;
            let v141 = (Lanes([v55[0], v55[1], 0.0])) - (Lanes([0.0, v99[0], v99[1]]));
            let v142 = v138 - v106;
            let v145 = (Lanes([v141[0], v141[1], 0.0, v141[2]])) - (Lanes([0.0, 0.0, v107[0], v107[1]]));
            let v147 = v24 * v146;
            let v148 = v25 * v146;
            let v150 = if v147 < v149 { 1.0 } else { 0.0 };
            let v159: f64;
            let v160: Lanes<2>;
            if v150 != 0.0 {
                let v151 = v147.exp();
                let v152 = v148 * v151;
                v159 = v151;
                v160 = v152;
            } else {
                let v153 = v149.exp();
                let v157 = v153 * (v155 + (v147 - v149));
                let v158 = v148 * v153;
                v159 = v157;
                v160 = v158;
            }
            let v161 = v32 * v146;
            let v162 = v33 * v146;
            let v164 = v161 / v163;
            let v165 = v162 / v163;
            let v166 = if v164 < v149 { 1.0 } else { 0.0 };
            let v174: f64;
            let v175: Lanes<2>;
            if v166 != 0.0 {
                let v167 = v164.exp();
                let v168 = v165 * v167;
                v174 = v167;
                v175 = v168;
            } else {
                let v169 = v149.exp();
                let v172 = v169 * (v155 + (v164 - v149));
                let v173 = v165 * v169;
                v174 = v172;
                v175 = v173;
            }
            let v176 = v116 * v146;
            let v177 = v119 * v146;
            let v178 = if v176 < v149 { 1.0 } else { 0.0 };
            let v186: f64;
            let v187: Lanes<5>;
            if v178 != 0.0 {
                let v179 = v176.exp();
                let v180 = v177 * v179;
                v186 = v179;
                v187 = v180;
            } else {
                let v181 = v149.exp();
                let v184 = v181 * (v155 + (v176 - v149));
                let v185 = v177 * v181;
                v186 = v184;
                v187 = v185;
            }
            let v188 = v46 * v146;
            let v189 = v47 * v146;
            let v190 = if v188 < v149 { 1.0 } else { 0.0 };
            let v198: f64;
            let v199: Lanes<2>;
            if v190 != 0.0 {
                let v191 = v188.exp();
                let v192 = v189 * v191;
                v198 = v191;
                v199 = v192;
            } else {
                let v193 = v149.exp();
                let v196 = v193 * (v155 + (v188 - v149));
                let v197 = v189 * v193;
                v198 = v196;
                v199 = v197;
            }
            let v200 = v135 * v146;
            let v201 = v137 * v146;
            let v202 = if v200 < v149 { 1.0 } else { 0.0 };
            let v210: f64;
            let v211: Lanes<8>;
            if v202 != 0.0 {
                let v203 = v200.exp();
                let v204 = v201 * v203;
                v210 = v203;
                v211 = v204;
            } else {
                let v205 = v149.exp();
                let v208 = v205 * (v155 + (v200 - v149));
                let v209 = v201 * v205;
                v210 = v208;
                v211 = v209;
            }
            let v212 = v54 * v146;
            let v213 = v55 * v146;
            let v214 = if v212 < v149 { 1.0 } else { 0.0 };
            let v222: f64;
            let v223: Lanes<2>;
            if v214 != 0.0 {
                let v215 = v212.exp();
                let v216 = v213 * v215;
                v222 = v215;
                v223 = v216;
            } else {
                let v217 = v149.exp();
                let v220 = v217 * (v155 + (v212 - v149));
                let v221 = v213 * v217;
                v222 = v220;
                v223 = v221;
            }
            let v224 = v142 * v146;
            let v225 = v145 * v146;
            let v226 = if v224 < v149 { 1.0 } else { 0.0 };
            let v234: f64;
            let v235: Lanes<4>;
            if v226 != 0.0 {
                let v227 = v224.exp();
                let v228 = v225 * v227;
                v234 = v227;
                v235 = v228;
            } else {
                let v229 = v149.exp();
                let v232 = v229 * (v155 + (v224 - v149));
                let v233 = v225 * v229;
                v234 = v232;
                v235 = v233;
            }
            let v236 = v138 * v146;
            let v237 = v141 * v146;
            let v238 = if v236 < v149 { 1.0 } else { 0.0 };
            let v246: f64;
            let v247: Lanes<3>;
            if v238 != 0.0 {
                let v239 = v236.exp();
                let v240 = v237 * v239;
                v246 = v239;
                v247 = v240;
            } else {
                let v241 = v149.exp();
                let v244 = v241 * (v155 + (v236 - v149));
                let v245 = v237 * v241;
                v246 = v244;
                v247 = v245;
            }
            let v250 = (v135 - v248) * v146;
            let v251 = if v250 < v149 { 1.0 } else { 0.0 };
            let v259: f64;
            let v260: Lanes<8>;
            if v251 != 0.0 {
                let v252 = v250.exp();
                let v253 = v201 * v252;
                v259 = v252;
                v260 = v253;
            } else {
                let v254 = v149.exp();
                let v257 = v254 * (v155 + (v250 - v149));
                let v258 = v201 * v254;
                v259 = v257;
                v260 = v258;
            }
            let v262 = (v116 - v248) * v146;
            let v263 = if v262 < v149 { 1.0 } else { 0.0 };
            let v271: f64;
            let v272: Lanes<5>;
            if v263 != 0.0 {
                let v264 = v262.exp();
                let v265 = v177 * v264;
                v271 = v264;
                v272 = v265;
            } else {
                let v266 = v149.exp();
                let v269 = v266 * (v155 + (v262 - v149));
                let v270 = v177 * v266;
                v271 = v269;
                v272 = v270;
            }
            let v274 = (v24 - v248) * v146;
            let v275 = if v274 < v149 { 1.0 } else { 0.0 };
            let v283: f64;
            let v284: Lanes<2>;
            if v275 != 0.0 {
                let v276 = v274.exp();
                let v277 = v148 * v276;
                v283 = v276;
                v284 = v277;
            } else {
                let v278 = v149.exp();
                let v281 = v278 * (v155 + (v274 - v149));
                let v282 = v148 * v278;
                v283 = v281;
                v284 = v282;
            }
            let v286 = (v16 - v248) * v146;
            let v287 = v17 * v146;
            let v288 = if v286 < v149 { 1.0 } else { 0.0 };
            let v296: f64;
            let v297: Lanes<2>;
            if v288 != 0.0 {
                let v289 = v286.exp();
                let v290 = v287 * v289;
                v296 = v289;
                v297 = v290;
            } else {
                let v291 = v149.exp();
                let v294 = v291 * (v155 + (v286 - v149));
                let v295 = v287 * v291;
                v296 = v294;
                v297 = v295;
            }
            let v302 = (v155 + (v298 * v283)).sqrt();
            let v307 = (v284 * v298) * (v305 / (v303 * v302));
            let v311 = (v155 + (v298 * v296)).sqrt();
            let v314 = (v297 * v298) * (v305 / (v303 * v311));
            let v318 = v155 + v311;
            let v319 = (v315 * v296) / v318;
            let v322 = ((v297 * v315) - (v314 * v319)) / v318;
            let v324 = if v319 < v323 { 1.0 } else { 0.0 };
            let v326: f64;
            let v327: Lanes<2>;
            if v324 != 0.0 {
                v326 = v323;
                v327 = v325;
            } else {
                v326 = v319;
                v327 = v322;
            }
            let v329 = Lanes([v307[0], 0.0, v307[1]]);
            let v332 = v302 + v155;
            let v333 = v332 / v318;
            let v334 = v314 * v333;
            let v344 = v343 * ((v302 - v311) - (v333.ln()));
            let v345 = ((v329 - (Lanes([v314[0], v314[1], 0.0]))) - (((v329 - (Lanes([v334[0], v334[1], 0.0]))) / v318) * (v305 / v333))) * v343;
            let v347 = Lanes([0.0, v61[0], v61[1]]);
            let v350 = (v344 + v60) / v349;
            let v351 = (v345 + v347) / v349;
            let v352 = if v350 > v0 { 1.0 } else { 0.0 };
            let v372: f64;
            let v373: f64;
            let v374: f64;
            let v375: f64;
            let v376: f64;
            let v377: f64;
            let v378: f64;
            let v379: Lanes<3>;
            let v380: Lanes<3>;
            let v381: Lanes<3>;
            let v382: Lanes<3>;
            let v383: Lanes<3>;
            let v384: Lanes<3>;
            let v385: Lanes<3>;
            if v352 != 0.0 {
                let v354 = if v16 < v353 { 1.0 } else { 0.0 };
                let v398: f64;
                let v399: Lanes<2>;
                if v354 != 0.0 {
                    v398 = v16;
                    v399 = v17;
                } else {
                    let v393 = v155 + (v16 - v353);
                    let v396 = v17 * (v305 / v393);
                    let v397 = v353 + (v393.ln());
                    v398 = v397;
                    v399 = v396;
                }
                let v400 = v315 * v343;
                let v404 = (v401 * v350) * v349;
                let v405 = (v351 * v401) * v349;
                let v408 = (v404 * v146) + v155;
                let v415 = (v248 + (v400 * (v408.ln()))) - v398;
                let v417 = (((v405 * v146) * (v305 / v408)) * v400) - (Lanes([v399[0], v399[1], 0.0]));
                let v419 = v418 * v248;
                let v420 = v419 * v419;
                let v421 = v415 * v415;
                let v422 = v417 * v415;
                let v423 = v422 + v422;
                let v424 = if v415 < v0 { 1.0 } else { 0.0 };
                let v446: f64;
                let v447: Lanes<3>;
                if v424 != 0.0 {
                    let v427 = (v421 + v420).sqrt();
                    let v431 = v427 - v415;
                    let v433 = (v401 * v420) / v431;
                    let v436 = ((((v423 * (v305 / (v303 * v427))) - v417) * v433) * v121) / v431;
                    v446 = v433;
                    v447 = v436;
                } else {
                    let v438 = (v421 + v420).sqrt();
                    let v444 = v401 * (v438 + v415);
                    let v445 = ((v423 * (v305 / (v303 * v438))) + v417) * v401;
                    v446 = v444;
                    v447 = v445;
                }
                let v450 = v448 * v449;
                let v451 = v446 + v450;
                let v458 = v449 * (v446 + (v448 * v349));
                let v460 = (v446 * v451) / v458;
                let v463 = (((v447 * v451) + (v447 * v446)) - ((v447 * v449) * v460)) / v458;
                let v464 = v350 / v460;
                let v467 = (v351 - (v463 * v464)) / v460;
                let v470 = (v464 - v155) / v469;
                let v471 = v467 / v469;
                let v472 = if v464 < v155 { 1.0 } else { 0.0 };
                let v494: f64;
                let v495: Lanes<3>;
                if v472 != 0.0 {
                    let v473 = v470.exp();
                    let v475 = v155 + v473;
                    let v480 = ((v471 * v473) * (v305 / v475)) * v469;
                    let v481 = v155 + (v469 * (v475.ln()));
                    v494 = v481;
                    v495 = v480;
                } else {
                    let v484 = (-v470).exp();
                    let v486 = v155 + v484;
                    let v492 = v464 + (v469 * (v486.ln()));
                    let v493 = v467 + ((((v471 * v121) * v484) * (v305 / v486)) * v469);
                    v494 = v492;
                    v495 = v493;
                }
                let v502 = v155 + (v469 * ((v155 + ((v496 / v469).exp())).ln()));
                let v503 = v494 / v502;
                let v504 = v495 / v502;
                let v505 = v446 / v450;
                let v506 = v447 / v450;
                let v507 = v298 * v503;
                let v509 = v507 * v505;
                let v513 = v155 + v505;
                let v519 = (v155 + (v509 * v513)).sqrt();
                let v524 = v315 * v503;
                let v526 = v524 * v513;
                let v530 = (v155 + v519) / v526;
                let v533 = (((((((v504 * v298) * v505) + (v506 * v507)) * v513) + (v506 * v509)) * (v305 / (v303 * v519))) - ((((v504 * v315) * v513) + (v506 * v524)) * v530)) / v526;
                let v536 = v326 * v530;
                let v537 = v327 * v530;
                let v540 = (Lanes([v537[0], v537[1], 0.0])) + (v533 * v326);
                let v543 = v155 + v536;
                let v544 = ((v155 - v530) + v536) / v543;
                let v547 = (((v533 * v121) + v540) - (v540 * v544)) / v543;
                let v552 = (v404 * v544) * v146;
                let v553 = ((v405 * v544) + (v547 * v404)) * v146;
                let v559 = (v326 + v552) + v155;
                let v561 = v327 * v559;
                let v565 = (v315 * v552) + (v326 * v559);
                let v566 = (v553 * v315) + ((Lanes([v561[0], v561[1], 0.0])) + (((Lanes([v327[0], v327[1], 0.0])) + v553) * v326));
                let v568 = v401 * (v552 - v155);
                let v569 = v553 * v401;
                let v571 = v569 * v568;
                let v573 = (v568 * v568) + v565;
                let v574 = (v571 + v571) + v566;
                let v575 = if v552 >= v155 { 1.0 } else { 0.0 };
                let v592: f64;
                let v593: Lanes<3>;
                if v575 != 0.0 {
                    let v576 = v573.sqrt();
                    let v580 = v568 + v576;
                    let v581 = v569 + (v574 * (v305 / (v303 * v576)));
                    v592 = v580;
                    v593 = v581;
                } else {
                    let v582 = v573.sqrt();
                    let v586 = v582 - v568;
                    let v588 = v565 / v586;
                    let v591 = (v566 - (((v574 * (v305 / (v303 * v582))) - v569) * v588)) / v586;
                    v592 = v588;
                    v593 = v591;
                }
                let v595 = if v592 < v594 { 1.0 } else { 0.0 };
                let v597: f64;
                let v598: Lanes<3>;
                if v595 != 0.0 {
                    v597 = v594;
                    v598 = v596;
                } else {
                    v597 = v592;
                    v598 = v593;
                }
                let v599 = v597 + v155;
                let v605 = (v248 * v146).exp();
                let v606 = (v597 * v599) * v605;
                let v607 = ((v598 * v599) + (v598 * v597)) * v605;
                let v608 = v401 * v449;
                let v610 = v608 * (v350 - v448);
                let v611 = v351 * v608;
                let v613 = (v449 * v349) * v448;
                let v617 = v611 * v610;
                let v621 = ((v610 * v610) + (v613 * v350)).sqrt();
                let v625 = v610 + v621;
                let v626 = v611 + (((v617 + v617) + (v351 * v613)) * (v305 / (v303 * v621)));
                let v628 = if v627 == v0 { 1.0 } else { 0.0 };
                let v643: f64;
                let v644: Lanes<3>;
                if v628 != 0.0 {
                    let v631 = v629 * v630;
                    v643 = v631;
                    v644 = v596;
                } else {
                    let v634 = v350 + v460;
                    let v636 = (v315 * v350) / v634;
                    let v641 = v629 * (v630 + v636);
                    let v642 = (((v351 * v315) - ((v351 + v463) * v636)) / v634) * v629;
                    v643 = v641;
                    v644 = v642;
                }
                let v647 = v448 + v350;
                let v648 = (v448 * v350) / v647;
                let v651 = ((v351 * v448) - (v351 * v648)) / v647;
                let v652 = v448 / v647;
                let v655 = ((v351 * v652) * v121) / v647;
                v372 = v625;
                v373 = v643;
                v374 = v652;
                v375 = v606;
                v376 = v544;
                v377 = v648;
                v378 = v597;
                v379 = v626;
                v380 = v644;
                v381 = v655;
                v382 = v607;
                v383 = v547;
                v384 = v651;
                v385 = v598;
            } else {
                let v357 = (v315 * v283) / v332;
                let v360 = ((v284 * v315) - (v307 * v357)) / v332;
                let v371 = if (if (v60.abs()) < (v362 * v343) { 1.0 } else { 0.0 }) != 0.0 || (if (v344.abs()) < ((v366 * v343) * (v302 + v311)) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v677: f64;
                let v678: Lanes<3>;
                if v371 != 0.0 {
                    let v660 = v401 * (v357 + v326);
                    let v661 = ((Lanes([v360[0], 0.0, v360[1]])) + (Lanes([v327[0], v327[1], 0.0]))) * v401;
                    let v662 = v660 + v155;
                    let v663 = v660 / v662;
                    let v666 = (v661 - (v661 * v663)) / v662;
                    v677 = v663;
                    v678 = v666;
                } else {
                    let v670 = (v344 + v24) - v16;
                    let v673 = v344 / v670;
                    let v676 = (v345 - (((v345 + (Lanes([v25[0], 0.0, v25[1]]))) - (Lanes([v17[0], v17[1], 0.0]))) * v673)) / v670;
                    v677 = v673;
                    v678 = v676;
                }
                let v679 = v630 * v629;
                let v682 = v155 - (v350 / v448);
                let v683 = (v351 / v448) * v121;
                let v684 = Lanes([v160[0], 0.0, v160[1]]);
                let v685 = Lanes([v360[0], 0.0, v360[1]]);
                v372 = v60;
                v373 = v679;
                v374 = v682;
                v375 = v159;
                v376 = v677;
                v377 = v350;
                v378 = v357;
                v379 = v347;
                v380 = v596;
                v381 = v683;
                v382 = v684;
                v383 = v678;
                v384 = v351;
                v385 = v685;
            }
            let v389 = (v32 - v386) / v388;
            let v390 = v33 / v388;
            let v391 = if v32 < v386 { 1.0 } else { 0.0 };
            let v708: f64;
            let v709: Lanes<2>;
            if v391 != 0.0 {
                let v686 = v389.exp();
                let v688 = v155 + v686;
                let v694 = v32 - (v388 * (v688.ln()));
                let v695 = v33 - (((v390 * v686) * (v305 / v688)) * v388);
                v708 = v694;
                v709 = v695;
            } else {
                let v698 = (-v389).exp();
                let v700 = v155 + v698;
                let v706 = v386 - (v388 * (v700.ln()));
                let v707 = ((((v390 * v121) * v698) * (v305 / v700)) * v388) * v121;
                v708 = v706;
                v709 = v707;
            }
            let v713 = v155 - (v708 * v710);
            let v714 = (v709 * v710) * v121;
            let v716 = v713.powf(v715);
            let v720 = v714 * (v715 * (v713.powf(v717)));
            let v731 = (v723 * (v155 - v716)) + (v728 * (v32 - v708));
            let v732 = ((v720 * v121) * v723) + ((v33 - v709) * v728);
            let v736: f64;
            let v737: Lanes<3>;
            if v733 != 0.0 {
                let v734 = Lanes([v17[0], v17[1], 0.0]);
                v736 = v16;
                v737 = v734;
            } else {
                let v749: f64;
                let v750: Lanes<3>;
                if v735 != 0.0 {
                    let v745 = v16 + v372;
                    let v747 = (Lanes([v17[0], v17[1], 0.0])) + v379;
                    v749 = v745;
                    v750 = v747;
                } else {
                    let v748 = Lanes([v25[0], 0.0, v25[1]]);
                    v749 = v24;
                    v750 = v748;
                }
                v736 = v749;
                v737 = v750;
            }
            let v740 = (v736 - v738) / v373;
            let v743 = (v737 - (v380 * v740)) / v373;
            let v744 = if v736 < v738 { 1.0 } else { 0.0 };
            let v777: f64;
            let v778: Lanes<3>;
            if v744 != 0.0 {
                let v751 = v740.exp();
                let v753 = v155 + v751;
                let v754 = v753.ln();
                let v761 = v736 - (v373 * v754);
                let v762 = v737 - ((v380 * v754) + (((v743 * v751) * (v305 / v753)) * v373));
                v777 = v761;
                v778 = v762;
            } else {
                let v765 = (-v740).exp();
                let v767 = v155 + v765;
                let v768 = v767.ln();
                let v775 = v738 - (v373 * v768);
                let v776 = ((v380 * v768) + ((((v743 * v121) * v765) * (v305 / v767)) * v373)) * v121;
                v777 = v775;
                v778 = v776;
            }
            let v780 = v374.powf(v779);
            let v784 = v381 * (v779 * (v374.powf(v781)));
            let v787 = v155 - (v777 / v629);
            let v790 = v787.powf(v789);
            let v805 = v780 * v804;
            let v807 = v736 - v777;
            let v820 = v17 * v818;
            let v821 = (v815 * ((v801 * (v155 - (v780 * v790))) + (v805 * v807))) + (v818 * v16);
            let v823 = ((((((v784 * v790) + ((((v778 / v629) * v121) * (v789 * (v787.powf(v791)))) * v780)) * v121) * v801) + (((v784 * v804) * v807) + ((v737 - v778) * v805))) * v815) + (Lanes([v820[0], v820[1], 0.0]));
            let v825 = v824 * v174;
            let v826 = v175 * v824;
            let v828 = (v155 + v825).sqrt();
            let v831 = v826 * (v305 / (v303 * v828));
            let v832 = v155 + v828;
            let v833 = v825 / v832;
            let v836 = (v826 - (v831 * v833)) / v832;
            let v838 = v375.powf(v837);
            let v842 = v382 * (v837 * (v375.powf(v839)));
            let v843 = v824 * v838;
            let v844 = v842 * v824;
            let v846 = (v155 + v843).sqrt();
            let v850 = v155 + v846;
            let v851 = v843 / v850;
            let v854 = (v844 - ((v844 * (v305 / (v303 * v846))) * v851)) / v850;
            let v894: f64;
            let v895: Lanes<4>;
            if v855 != 0.0 {
                let v858 = v732 / v856;
                let v862 = v823 / v860;
                let v863 = (v155 + (v731 / v856)) + (v821 / v860);
                let v866 = (Lanes([v858[0], v858[1], 0.0, 0.0])) + (Lanes([0.0, v862[0], v862[1], v862[2]]));
                v894 = v863;
                v895 = v866;
            } else {
                let v883 = ((((v731 / v856) + v155) * v870) * v146).exp();
                let v884 = (((v732 / v856) * v870) * v146) * v883;
                let v885 = ((((-v821) / v860) * v870) * v146).exp();
                let v886 = ((((v823 * v121) / v860) * v870) * v146) * v885;
                let v892 = (v883 - v885) / v891;
                let v893 = ((Lanes([v884[0], v884[1], 0.0, 0.0])) - (Lanes([0.0, v886[0], v886[1], v886[2]]))) / v891;
                v894 = v892;
                v895 = v893;
            }
            let v896 = v894 * v894;
            let v897 = v895 * v894;
            let v898 = v897 + v897;
            let v899 = if v894 < v0 { 1.0 } else { 0.0 };
            let v922: f64;
            let v923: Lanes<4>;
            if v899 != 0.0 {
                let v902 = (v896 + v900).sqrt();
                let v906 = v902 - v894;
                let v909 = v908 / v906;
                let v912 = ((((v898 * (v305 / (v303 * v902))) - v895) * v909) * v121) / v906;
                v922 = v909;
                v923 = v912;
            } else {
                let v914 = (v896 + v900).sqrt();
                let v920 = v401 * (v914 + v894);
                let v921 = ((v898 * (v305 / (v303 * v914))) + v895) * v401;
                v922 = v920;
                v923 = v921;
            }
            let v929 = ((Lanes([v836[0], v836[1], 0.0, 0.0])) + (Lanes([0.0, v854[0], v854[1], v854[2]]))) * v401;
            let v930 = v155 + (v401 * (v833 + v851));
            let v931 = v922 * v930;
            let v934 = (v923 * v930) + (v929 * v922);
            let v936 = v935 * v838;
            let v937 = v842 * v935;
            let v939 = v938 * v174;
            let v940 = v175 * v938;
            let v942 = Lanes([v940[0], v940[1], 0.0, 0.0]);
            let v943 = Lanes([0.0, v937[0], v937[1], v937[2]]);
            let v945 = (v939 - v936) / v931;
            let v948 = ((v942 - v943) - (v934 * v945)) / v931;
            let v950 = v32 / v949;
            let v951 = v33 / v949;
            let v952 = if v32 < v0 { 1.0 } else { 0.0 };
            let v973: f64;
            let v974: Lanes<2>;
            if v952 != 0.0 {
                let v953 = v950.exp();
                let v955 = v155 + v953;
                let v959 = v949 * (v955.ln());
                let v960 = ((v951 * v953) * (v305 / v955)) * v949;
                v973 = v959;
                v974 = v960;
            } else {
                let v963 = (-v950).exp();
                let v965 = v155 + v963;
                let v971 = v32 + (v949 * (v965.ln()));
                let v972 = v33 + ((((v951 * v121) * v963) * (v305 / v965)) * v949);
                v973 = v971;
                v974 = v972;
            }
            let v976 = v973 / v975;
            let v977 = v974 / v975;
            let v978 = if v976 < v149 { 1.0 } else { 0.0 };
            let v986: f64;
            let v987: Lanes<2>;
            if v978 != 0.0 {
                let v979 = v976.exp();
                let v980 = v977 * v979;
                v986 = v979;
                v987 = v980;
            } else {
                let v981 = v149.exp();
                let v984 = v981 * (v155 + (v976 - v149));
                let v985 = v977 * v981;
                v986 = v984;
                v987 = v985;
            }
            let v990 = v989 * (v986 - v155);
            let v991 = v987 * v989;
            let v995 = (v32 - v992) / v994;
            let v996 = v33 / v994;
            let v997 = if v32 < v992 { 1.0 } else { 0.0 };
            let v1020: f64;
            let v1021: Lanes<2>;
            if v997 != 0.0 {
                let v998 = v995.exp();
                let v1000 = v155 + v998;
                let v1006 = v32 - (v994 * (v1000.ln()));
                let v1007 = v33 - (((v996 * v998) * (v305 / v1000)) * v994);
                v1020 = v1006;
                v1021 = v1007;
            } else {
                let v1010 = (-v995).exp();
                let v1012 = v155 + v1010;
                let v1018 = v992 - (v994 * (v1012.ln()));
                let v1019 = ((((v996 * v121) * v1010) * (v305 / v1012)) * v994) * v121;
                v1020 = v1018;
                v1021 = v1019;
            }
            let v1023 = v1022 * v1020;
            let v1025 = v992 - v1020;
            let v1027 = v1025 * v1025;
            let v1030 = v1023 * v1027;
            let v1033 = ((v1021 * v1022) * v1027) + (((v1021 * v121) * (v315 * v1025)) * v1023);
            let v1035 = v161 / v1034;
            let v1036 = v162 / v1034;
            let v1037 = if v1035 < v149 { 1.0 } else { 0.0 };
            let v1045: f64;
            let v1046: Lanes<2>;
            if v1037 != 0.0 {
                let v1038 = v1035.exp();
                let v1039 = v1036 * v1038;
                v1045 = v1038;
                v1046 = v1039;
            } else {
                let v1040 = v149.exp();
                let v1043 = v1040 * (v155 + (v1035 - v149));
                let v1044 = v1036 * v1040;
                v1045 = v1043;
                v1046 = v1044;
            }
            let v1052: f64;
            let v1053: Lanes<4>;
            if v4 != 0.0 {
                let v1049 = (v32 - v1047) * v146;
                let v1050 = if v1049 < v149 { 1.0 } else { 0.0 };
                let v1067: f64;
                let v1068: Lanes<2>;
                if v1050 != 0.0 {
                    let v1060 = v1049.exp();
                    let v1061 = v162 * v1060;
                    v1067 = v1060;
                    v1068 = v1061;
                } else {
                    let v1062 = v149.exp();
                    let v1065 = v1062 * (v155 + (v1049 - v149));
                    let v1066 = v162 * v1062;
                    v1067 = v1065;
                    v1068 = v1066;
                }
                let v1070 = v948 / v938;
                let v1072 = (v945 / v938) - v1071;
                let v1074 = if v1072 < v1073 { 1.0 } else { 0.0 };
                let v1082: f64;
                let v1083: Lanes<4>;
                if v1074 != 0.0 {
                    let v1075 = v1072.exp();
                    let v1076 = v1070 * v1075;
                    v1082 = v1075;
                    v1083 = v1076;
                } else {
                    let v1080 = v1079 * (v155 + (v1072 - v1073));
                    let v1081 = v1070 * v1079;
                    v1082 = v1080;
                    v1083 = v1081;
                }
                let v1084 = v1045 - v155;
                let v1087 = v1046 * v1085;
                let v1094 = (v155 + (v298 * v1067)).sqrt();
                let v1098 = v155 + v1094;
                let v1099 = (v1088 * v1084) / v1098;
                let v1105 = v155 + (v821 / v860);
                let v1107 = (((v1046 * v1088) - (((v1068 * v298) * (v305 / (v303 * v1094))) * v1099)) / v1098) * v1105;
                let v1108 = (v823 / v860) * v1099;
                let v1117 = v1116 * (v375 - v155);
                let v1120 = (v382 * v1116) * v1082;
                let v1124 = v155 + v1082;
                let v1125 = (v1117 * v1082) / v1124;
                let v1129 = ((v1085 * v1084) + (v1099 * v1105)) + v1125;
                let v1130 = ((Lanes([v1087[0], v1087[1], 0.0, 0.0])) + ((Lanes([v1107[0], v1107[1], 0.0, 0.0])) + (Lanes([0.0, v1108[0], v1108[1], v1108[2]])))) + ((((Lanes([0.0, v1120[0], v1120[1], v1120[2]])) + (v1083 * v1117)) - (v1083 * v1125)) / v1124);
                v1052 = v1129;
                v1053 = v1130;
            } else {
                let v1160: f64;
                let v1161: Lanes<4>;
                if v1051 != 0.0 {
                    let v1132 = v1085 * (v1045 - v155);
                    let v1133 = v1046 * v1085;
                    let v1134 = Lanes([v1133[0], v1133[1], 0.0, 0.0]);
                    v1160 = v1132;
                    v1161 = v1134;
                } else {
                    let v1138 = v1046 * v1136;
                    let v1145 = v1144 * ((v1045 + v375) - v315);
                    let v1149 = v155 + (v821 / v860);
                    let v1152 = (v823 / v860) * v1145;
                    let v1158 = v1085 * ((v1136 * (v1045 - v155)) + (v1145 * v1149));
                    let v1159 = ((Lanes([v1138[0], v1138[1], 0.0, 0.0])) + (((((Lanes([v1046[0], v1046[1], 0.0, 0.0])) + (Lanes([0.0, v382[0], v382[1], v382[2]]))) * v1144) * v1149) + (Lanes([0.0, v1152[0], v1152[1], v1152[2]])))) * v1085;
                    v1160 = v1158;
                    v1161 = v1159;
                }
                v1052 = v1160;
                v1053 = v1161;
            }
            let v1054 = v40 * v146;
            let v1055 = v41 * v146;
            let v1057 = v1054 / v1056;
            let v1058 = v1055 / v1056;
            let v1059 = if v1057 < v149 { 1.0 } else { 0.0 };
            let v1169: f64;
            let v1170: Lanes<2>;
            if v1059 != 0.0 {
                let v1162 = v1057.exp();
                let v1163 = v1058 * v1162;
                v1169 = v1162;
                v1170 = v1163;
            } else {
                let v1164 = v149.exp();
                let v1167 = v1164 * (v155 + (v1057 - v149));
                let v1168 = v1058 * v1164;
                v1169 = v1167;
                v1170 = v1168;
            }
            let v1178: f64;
            let v1179: Lanes<2>;
            if v4 != 0.0 {
                let v1172 = (v40 - v1047) * v146;
                let v1173 = if v1172 < v149 { 1.0 } else { 0.0 };
                let v1191: f64;
                let v1192: Lanes<2>;
                if v1173 != 0.0 {
                    let v1184 = v1172.exp();
                    let v1185 = v1055 * v1184;
                    v1191 = v1184;
                    v1192 = v1185;
                } else {
                    let v1186 = v149.exp();
                    let v1189 = v1186 * (v155 + (v1172 - v149));
                    let v1190 = v1055 * v1186;
                    v1191 = v1189;
                    v1192 = v1190;
                }
                let v1193 = v1169 - v155;
                let v1202 = (v155 + (v298 * v1191)).sqrt();
                let v1206 = v155 + v1202;
                let v1207 = (v1196 * v1193) / v1206;
                let v1211 = (v1175 * v1193) + v1207;
                let v1212 = (v1170 * v1175) + (((v1170 * v1196) - (((v1192 * v298) * (v305 / (v303 * v1202))) * v1207)) / v1206);
                v1178 = v1211;
                v1179 = v1212;
            } else {
                let v1176 = v1175 * (v1169 - v155);
                let v1177 = v1170 * v1175;
                v1178 = v1176;
                v1179 = v1177;
            }
            let v1181 = v161 / v1180;
            let v1182 = v162 / v1180;
            let v1183 = if v1181 < v149 { 1.0 } else { 0.0 };
            let v1220: f64;
            let v1221: Lanes<2>;
            if v1183 != 0.0 {
                let v1213 = v1181.exp();
                let v1214 = v1182 * v1213;
                v1220 = v1213;
                v1221 = v1214;
            } else {
                let v1215 = v149.exp();
                let v1218 = v1215 * (v155 + (v1181 - v149));
                let v1219 = v1182 * v1215;
                v1220 = v1218;
                v1221 = v1219;
            }
            let v1224 = v1223 * (v1220 - v155);
            let v1225 = v1221 * v1223;
            let v1227 = v1054 / v1226;
            let v1228 = v1055 / v1226;
            let v1229 = if v1227 < v149 { 1.0 } else { 0.0 };
            let v1237: f64;
            let v1238: Lanes<2>;
            if v1229 != 0.0 {
                let v1230 = v1227.exp();
                let v1231 = v1228 * v1230;
                v1237 = v1230;
                v1238 = v1231;
            } else {
                let v1232 = v149.exp();
                let v1235 = v1232 * (v155 + (v1227 - v149));
                let v1236 = v1228 * v1232;
                v1237 = v1235;
                v1238 = v1236;
            }
            let v1241 = v1240 * (v1237 - v155);
            let v1242 = v1238 * v1240;
            let v1244 = v176 / v1243;
            let v1245 = v177 / v1243;
            let v1246 = if v1244 < v149 { 1.0 } else { 0.0 };
            let v1254: f64;
            let v1255: Lanes<5>;
            if v1246 != 0.0 {
                let v1247 = v1244.exp();
                let v1248 = v1245 * v1247;
                v1254 = v1247;
                v1255 = v1248;
            } else {
                let v1249 = v149.exp();
                let v1252 = v1249 * (v155 + (v1244 - v149));
                let v1253 = v1245 * v1249;
                v1254 = v1252;
                v1255 = v1253;
            }
            let v1258 = v1257 * (v1254 - v155);
            let v1259 = v1255 * v1257;
            let v1261 = v1054 / v1260;
            let v1262 = v1055 / v1260;
            let v1263 = if v1261 < v149 { 1.0 } else { 0.0 };
            let v1271: f64;
            let v1272: Lanes<2>;
            if v1263 != 0.0 {
                let v1264 = v1261.exp();
                let v1265 = v1262 * v1264;
                v1271 = v1264;
                v1272 = v1265;
            } else {
                let v1266 = v149.exp();
                let v1269 = v1266 * (v155 + (v1261 - v149));
                let v1270 = v1262 * v1266;
                v1271 = v1269;
                v1272 = v1270;
            }
            let v1275 = v1274 * (v1271 - v155);
            let v1276 = v1272 * v1274;
            let v1278 = if v1277 != 0.0 && v952 != 0.0 { 1.0 } else { 0.0 };
            let v1293: f64;
            let v1294: Lanes<2>;
            if v1278 != 0.0 {
                let v1279 = v315 * v716;
                let v1282 = v1281 / v1279;
                let v1289 = v1288 * (v155 - v1282);
                let v1290 = (((((v720 * v315) * v1282) * v121) / v1279) * v121) * v1288;
                let v1291 = if v1289 < v149 { 1.0 } else { 0.0 };
                let v1305: f64;
                let v1306: Lanes<2>;
                if v1291 != 0.0 {
                    let v1298 = v1289.exp();
                    let v1299 = v1290 * v1298;
                    v1305 = v1298;
                    v1306 = v1299;
                } else {
                    let v1300 = v149.exp();
                    let v1303 = v1300 * (v155 + (v1289 - v149));
                    let v1304 = v1290 * v1300;
                    v1305 = v1303;
                    v1306 = v1304;
                }
                let v1307 = v32 * v710;
                let v1308 = v33 * v710;
                let v1310 = v1308 * v1307;
                let v1314 = ((v1307 * v1307) + v1312).sqrt();
                let v1320 = v1318 - v1319;
                let v1321 = v1314.powf(v1320);
                let v1330 = v1319 - v155;
                let v1338 = v1337 * v1307;
                let v1340 = v1338 * v1307;
                let v1344 = v1330 + v1307;
                let v1349 = (v1319 * ((v155 - (v1319 * v1319)) - ((v728 * v1307) * v1330))) - (v1340 * v1344);
                let v1363 = v1362 * ((v1321 * v1349) * v1355);
                let v1365 = ((v32 * v1281) * v1288) / v1363;
                let v1368 = (((v33 * v1281) * v1288) - ((((((((v1310 + v1310) * (v305 / (v303 * v1314))) * (v1320 * (v1314.powf((v1320 - v305))))) * v1349) + ((((((v1308 * v728) * v1330) * v121) * v1319) - (((((v1308 * v1337) * v1307) + (v1308 * v1338)) * v1344) + (v1308 * v1340))) * v1321)) * v1355) * v1362) * v1365)) / v1363;
                let v1370 = if v1365 < v1369 { 1.0 } else { 0.0 };
                let v1394: f64;
                let v1395: Lanes<2>;
                if v1370 != 0.0 {
                    let v1371 = if v1365 < v149 { 1.0 } else { 0.0 };
                    let v1420: f64;
                    let v1421: Lanes<2>;
                    if v1371 != 0.0 {
                        let v1413 = v1365.exp();
                        let v1414 = v1368 * v1413;
                        v1420 = v1413;
                        v1421 = v1414;
                    } else {
                        let v1415 = v149.exp();
                        let v1418 = v1415 * (v155 + (v1365 - v149));
                        let v1419 = v1368 * v1415;
                        v1420 = v1418;
                        v1421 = v1419;
                    }
                    let v1422 = -v32;
                    let v1426 = (v155 - v1420) / v1365;
                    let v1430 = v155 + v1426;
                    let v1431 = v1422 * v1430;
                    let v1434 = ((v33 * v121) * v1430) + ((((v1421 * v121) - (v1368 * v1426)) / v1365) * v1422);
                    v1394 = v1431;
                    v1395 = v1434;
                } else {
                    let v1372 = v32 * v401;
                    let v1374 = v1372 * v1365;
                    let v1379 = v1365 * v1378;
                    let v1384 = v155 + (v1381 * v1365);
                    let v1389 = v155 + (v1379 * v1384);
                    let v1390 = v1374 * v1389;
                    let v1393 = ((((v33 * v401) * v1365) + (v1368 * v1372)) * v1389) + ((((v1368 * v1378) * v1384) + ((v1368 * v1381) * v1379)) * v1374);
                    v1394 = v1390;
                    v1395 = v1393;
                }
                let v1397 = v315 * v1396;
                let v1398 = v1397 * v1394;
                let v1400 = v1398 * v716;
                let v1411 = ((v1400 * v1305) * v710) * v1410;
                let v1412 = ((((((v1395 * v1397) * v716) + (v720 * v1398)) * v1305) + (v1306 * v1400)) * v710) * v1410;
                v1293 = v1411;
                v1294 = v1412;
            } else {
                v1293 = v0;
                v1294 = v1292;
            }
            let v1297 = if v1296 != 0.0 && (if v16 < v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v1457: f64;
            let v1458: Lanes<2>;
            if v1297 != 0.0 {
                let v1436 = v16 * v1435;
                let v1437 = v17 * v1435;
                let v1438 = v155 - v1436;
                let v1440 = v1438.powf(v789);
                let v1443 = (v1437 * v121) * (v789 * (v1438.powf(v791)));
                let v1444 = v315 * v1440;
                let v1447 = v1446 / v1444;
                let v1454 = v1453 * (v155 - v1447);
                let v1455 = (((((v1443 * v315) * v1447) * v121) / v1444) * v121) * v1453;
                let v1456 = if v1454 < v149 { 1.0 } else { 0.0 };
                let v1509: f64;
                let v1510: Lanes<2>;
                if v1456 != 0.0 {
                    let v1502 = v1454.exp();
                    let v1503 = v1455 * v1502;
                    v1509 = v1502;
                    v1510 = v1503;
                } else {
                    let v1504 = v149.exp();
                    let v1507 = v1504 * (v155 + (v1454 - v149));
                    let v1508 = v1455 * v1504;
                    v1509 = v1507;
                    v1510 = v1508;
                }
                let v1512 = v1437 * v1436;
                let v1515 = ((v1436 * v1436) + v1312).sqrt();
                let v1521 = v1519 - v1520;
                let v1522 = v1515.powf(v1521);
                let v1531 = v1520 - v155;
                let v1538 = v1337 * v1436;
                let v1540 = v1538 * v1436;
                let v1544 = v1531 + v1436;
                let v1549 = (v1520 * ((v155 - (v1520 * v1520)) - ((v728 * v1436) * v1531))) - (v1540 * v1544);
                let v1562 = v1561 * ((v1522 * v1549) * v1355);
                let v1564 = ((v16 * v1446) * v1453) / v1562;
                let v1567 = (((v17 * v1446) * v1453) - ((((((((v1512 + v1512) * (v305 / (v303 * v1515))) * (v1521 * (v1515.powf((v1521 - v305))))) * v1549) + ((((((v1437 * v728) * v1531) * v121) * v1520) - (((((v1437 * v1337) * v1436) + (v1437 * v1538)) * v1544) + (v1437 * v1540))) * v1522)) * v1355) * v1561) * v1564)) / v1562;
                let v1569 = if v1564 < v1568 { 1.0 } else { 0.0 };
                let v1591: f64;
                let v1592: Lanes<2>;
                if v1569 != 0.0 {
                    let v1570 = if v1564 < v149 { 1.0 } else { 0.0 };
                    let v1617: f64;
                    let v1618: Lanes<2>;
                    if v1570 != 0.0 {
                        let v1610 = v1564.exp();
                        let v1611 = v1567 * v1610;
                        v1617 = v1610;
                        v1618 = v1611;
                    } else {
                        let v1612 = v149.exp();
                        let v1615 = v1612 * (v155 + (v1564 - v149));
                        let v1616 = v1567 * v1612;
                        v1617 = v1615;
                        v1618 = v1616;
                    }
                    let v1619 = -v16;
                    let v1623 = (v155 - v1617) / v1564;
                    let v1627 = v155 + v1623;
                    let v1628 = v1619 * v1627;
                    let v1631 = ((v17 * v121) * v1627) + ((((v1618 * v121) - (v1567 * v1623)) / v1564) * v1619);
                    v1591 = v1628;
                    v1592 = v1631;
                } else {
                    let v1571 = v16 * v401;
                    let v1573 = v1571 * v1564;
                    let v1577 = v1564 * v1378;
                    let v1581 = v155 + (v1381 * v1564);
                    let v1586 = v155 + (v1577 * v1581);
                    let v1587 = v1573 * v1586;
                    let v1590 = ((((v17 * v401) * v1564) + (v1567 * v1571)) * v1586) + ((((v1567 * v1378) * v1581) + ((v1567 * v1381) * v1577)) * v1573);
                    v1591 = v1587;
                    v1592 = v1590;
                }
                let v1594 = v315 * v1593;
                let v1595 = v1594 * v1591;
                let v1597 = v1595 * v1440;
                let v1608 = ((v1597 * v1509) * v1435) * v1607;
                let v1609 = ((((((v1592 * v1594) * v1440) + (v1443 * v1595)) * v1509) + (v1510 * v1597)) * v1435) * v1607;
                v1457 = v1608;
                v1458 = v1609;
            } else {
                v1457 = v0;
                v1458 = v325;
            }
            let v1459 = v824 * v186;
            let v1460 = v187 * v824;
            let v1461 = v298 * v271;
            let v1462 = v272 * v298;
            let v1465 = (v155 + v1459).sqrt();
            let v1469 = v155 + v1465;
            let v1470 = (v1459 - v824) / v1469;
            let v1473 = (v1460 - ((v1460 * (v305 / (v303 * v1465))) * v1470)) / v1469;
            let v1475 = (v155 + v1461).sqrt();
            let v1479 = v155 + v1475;
            let v1480 = v1461 / v1479;
            let v1483 = (v1462 - ((v1462 * (v305 / (v303 * v1475))) * v1480)) / v1479;
            let v1484 = v186 - v155;
            let v1492 = (v155 + (v1488 * v186)).sqrt();
            let v1496 = v155 + v1492;
            let v1497 = (v1485 * v1484) / v1496;
            let v1500 = ((v187 * v1485) - (((v187 * v1488) * (v305 / (v303 * v1492))) * v1497)) / v1496;
            let v1716: f64;
            let v1717: f64;
            let v1718: Lanes<6>;
            let v1719: Lanes<4>;
            if v1501 != 0.0 {
                let v1633 = Lanes([0.0, v160[0], 0.0, v160[1]]);
                let v1641 = v223 * v1639;
                let v1649 = (v155 + (v1645 * (v159 + (v1639 * v222)))).sqrt();
                let v1653 = v155 + v1649;
                let v1654 = (v1636 * (v159 - v222)) / v1653;
                let v1657 = (((v1633 - (Lanes([v223[0], 0.0, v223[1], 0.0]))) * v1636) - ((((v1633 + (Lanes([v1641[0], 0.0, v1641[1], 0.0]))) * v1645) * (v305 / (v303 * v1649))) * v1654)) / v1653;
                let v1659 = Lanes([0.0, v187[0], v187[1], v187[2], v187[3], v187[4]]);
                let v1666 = v247 * v1639;
                let v1673 = (v155 + (v1645 * (v186 + (v1639 * v246)))).sqrt();
                let v1677 = v155 + v1673;
                let v1678 = (v1662 * (v186 - v246)) / v1677;
                let v1681 = (((v1659 - (Lanes([v247[0], 0.0, 0.0, v247[1], 0.0, v247[2]]))) * v1662) - ((((v1659 + (Lanes([v1666[0], 0.0, 0.0, v1666[1], 0.0, v1666[2]]))) * v1645) * (v305 / (v303 * v1673))) * v1678)) / v1677;
                v1716 = v1678;
                v1717 = v1654;
                v1718 = v1681;
                v1719 = v1657;
            } else {
                let v1690 = (v155 + (v1686 * v159)).sqrt();
                let v1694 = v155 + v1690;
                let v1695 = (v1683 * (v159 - v155)) / v1694;
                let v1698 = ((v160 * v1683) - (((v160 * v1686) * (v305 / (v303 * v1690))) * v1695)) / v1694;
                let v1705 = (v155 + (v1686 * v186)).sqrt();
                let v1709 = v155 + v1705;
                let v1710 = (v1699 * v1484) / v1709;
                let v1713 = ((v187 * v1699) - (((v187 * v1686) * (v305 / (v303 * v1705))) * v1710)) / v1709;
                let v1714 = Lanes([0.0, v1713[0], v1713[1], v1713[2], v1713[3], v1713[4]]);
                let v1715 = Lanes([0.0, v1698[0], 0.0, v1698[1]]);
                v1716 = v1710;
                v1717 = v1695;
                v1718 = v1714;
                v1719 = v1715;
            }
            let v1728 = (v155 + (v1724 * v222)).sqrt();
            let v1732 = v155 + v1728;
            let v1733 = (v1721 * (v222 - v155)) / v1732;
            let v1739 = v1733 + (v54 * v3);
            let v1740 = (((v223 * v1721) - (((v223 * v1724) * (v305 / (v303 * v1728))) * v1733)) / v1732) + (v55 * v3);
            let v1764: f64;
            let v1765: f64;
            let v1766: f64;
            let v1767: f64;
            let v1768: f64;
            let v1769: Lanes<5>;
            let v1770: Lanes<9>;
            let v1771: Lanes<9>;
            let v1772: Lanes<6>;
            let v1773: Lanes<9>;
            if v1741 != 0.0 {
                let v1743 = v1497 * v1742;
                let v1744 = v1500 * v1742;
                let v1745 = v1716 * v1742;
                let v1746 = v1718 * v1742;
                let v1747 = v210 - v155;
                let v1754 = (v155 + (v1488 * v210)).sqrt();
                let v1758 = v155 + v1754;
                let v1759 = (v1748 * v1747) / v1758;
                let v1762 = ((v211 * v1748) - (((v211 * v1488) * (v305 / (v303 * v1754))) * v1759)) / v1758;
                let v1817: f64;
                let v1818: Lanes<9>;
                if v1501 != 0.0 {
                    let v1776 = Lanes([v211[0], v211[1], 0.0, v211[2], v211[3], v211[4], v211[5], v211[6], v211[7]]);
                    let v1783 = v235 * v1639;
                    let v1791 = (v155 + (v1787 * (v210 + (v1639 * v234)))).sqrt();
                    let v1795 = v155 + v1791;
                    let v1796 = (v1779 * (v210 - v234)) / v1795;
                    let v1799 = (((v1776 - (Lanes([0.0, 0.0, v235[0], 0.0, 0.0, v235[1], 0.0, v235[2], v235[3]]))) * v1779) - ((((v1776 + (Lanes([0.0, 0.0, v1783[0], 0.0, 0.0, v1783[1], 0.0, v1783[2], v1783[3]]))) * v1787) * (v305 / (v303 * v1791))) * v1796)) / v1795;
                    v1817 = v1796;
                    v1818 = v1799;
                } else {
                    let v1807 = (v155 + (v1803 * v210)).sqrt();
                    let v1811 = v155 + v1807;
                    let v1812 = (v1800 * v1747) / v1811;
                    let v1815 = ((v211 * v1800) - (((v211 * v1803) * (v305 / (v303 * v1807))) * v1812)) / v1811;
                    let v1816 = Lanes([v1815[0], v1815[1], 0.0, v1815[2], v1815[3], v1815[4], v1815[5], v1815[6], v1815[7]]);
                    v1817 = v1812;
                    v1818 = v1816;
                }
                let v1826: f64;
                let v1827: Lanes<9>;
                if v1819 != 0.0 {
                    let v1821 = v135 - v1820;
                    let v1822 = v1821 * v1821;
                    let v1823 = v137 * v1821;
                    let v1824 = v1823 + v1823;
                    let v1825 = if v1821 < v0 { 1.0 } else { 0.0 };
                    let v1859: f64;
                    let v1860: Lanes<8>;
                    if v1825 != 0.0 {
                        let v1839 = (v1822 + v1837).sqrt();
                        let v1843 = v1839 - v1821;
                        let v1846 = v1845 / v1843;
                        let v1849 = ((((v1824 * (v305 / (v303 * v1839))) - v137) * v1846) * v121) / v1843;
                        v1859 = v1846;
                        v1860 = v1849;
                    } else {
                        let v1851 = (v1822 + v1837).sqrt();
                        let v1857 = v401 * (v1851 + v1821);
                        let v1858 = ((v1824 * (v305 / (v303 * v1851))) + v137) * v401;
                        v1859 = v1857;
                        v1860 = v1858;
                    }
                    let v1869 = (v1867 + ((v1759 + v1817) * v1864)) + v1859;
                    let v1870 = Lanes([v1860[0], v1860[1], 0.0, v1860[2], v1860[3], v1860[4], v1860[5], v1860[6], v1860[7]]);
                    let v1872 = v1859 / v1869;
                    let v1875 = (v1870 - (((((Lanes([v1762[0], v1762[1], 0.0, v1762[2], v1762[3], v1762[4], v1762[5], v1762[6], v1762[7]])) + v1818) * v1864) + v1870) * v1872)) / v1869;
                    v1826 = v1872;
                    v1827 = v1875;
                } else {
                    v1826 = v155;
                    v1827 = v1763;
                }
                let v1828 = v1826 * v1759;
                let v1830 = v1762 * v1826;
                let v1832 = (v1827 * v1759) + (Lanes([v1830[0], v1830[1], 0.0, v1830[2], v1830[3], v1830[4], v1830[5], v1830[6], v1830[7]]));
                let v1833 = v1826 * v1817;
                let v1836 = (v1827 * v1817) + (v1818 * v1826);
                v1764 = v1743;
                v1765 = v1828;
                v1766 = v1826;
                v1767 = v1745;
                v1768 = v1833;
                v1769 = v1744;
                v1770 = v1832;
                v1771 = v1827;
                v1772 = v1746;
                v1773 = v1836;
            } else {
                v1764 = v1497;
                v1765 = v0;
                v1766 = v155;
                v1767 = v1716;
                v1768 = v0;
                v1769 = v1500;
                v1770 = v1763;
                v1771 = v1763;
                v1772 = v1718;
                v1773 = v1763;
            }
            let v1894: f64;
            let v1895: Lanes<3>;
            if v1774 != 0.0 {
                let v1876 = v46 + v16;
                let v1879 = (Lanes([v47[0], v47[1], 0.0])) + (Lanes([0.0, v17[0], v17[1]]));
                let v1884 = (v1880 * v1876) * v1883;
                let v1886 = v1884 * v1876;
                let v1889 = (((v1879 * v1880) * v1883) * v1876) + (v1879 * v1884);
                let v1892 = if (v1890 * v1876) < v0 { 1.0 } else { 0.0 };
                let v1957: f64;
                let v1958: Lanes<3>;
                if v1892 != 0.0 {
                    let v1931 = (v1886 + v1929).sqrt();
                    let v1938 = v1931 - (v1935 * v1876);
                    let v1941 = v1940 / v1938;
                    let v1944 = ((((v1889 * (v305 / (v303 * v1931))) - (v1879 * v1935)) * v1941) * v121) / v1938;
                    v1957 = v1941;
                    v1958 = v1944;
                } else {
                    let v1946 = (v1886 + v1929).sqrt();
                    let v1955 = v401 * (v1946 + (v1950 * v1876));
                    let v1956 = ((v1889 * (v305 / (v303 * v1946))) + (v1879 * v1950)) * v401;
                    v1957 = v1955;
                    v1958 = v1956;
                }
                let v1960 = if v1957 < v1959 { 1.0 } else { 0.0 };
                let v1982: f64;
                let v1983: Lanes<3>;
                if v1960 != 0.0 {
                    let v1962 = v1957 / v1961;
                    let v1970 = v155 - (v1962.powf(v1964));
                    let v1972 = v155 / v1970;
                    let v1975 = (((((v1958 / v1961) * (v1964 * (v1962.powf((v1964 - v305))))) * v121) * v1972) * v121) / v1970;
                    v1982 = v1972;
                    v1983 = v1975;
                } else {
                    let v1979 = v1958 * v1977;
                    let v1981 = v1980 + ((v1957 - v1959) * v1977);
                    v1982 = v1981;
                    v1983 = v1979;
                }
                v1894 = v1982;
                v1895 = v1983;
            } else {
                v1894 = v155;
                v1895 = v1893;
            }
            let v1896 = v1457 * v1894;
            let v1897 = v1458 * v1894;
            let v1900 = (Lanes([0.0, v1897[0], v1897[1]])) + (v1895 * v1457);
            let v1901 = v1764 * v1894;
            let v1903 = v1895 * v1764;
            let v1905 = (v1769 * v1894) + (Lanes([v1903[0], v1903[1], v1903[2], 0.0, 0.0]));
            let v1906 = v1258 * v1894;
            let v1908 = v1895 * v1258;
            let v1910 = (v1259 * v1894) + (Lanes([v1908[0], v1908[1], v1908[2], 0.0, 0.0]));
            let v1911 = v1765 * v1894;
            let v1913 = v1895 * v1765;
            let v1915 = (v1770 * v1894) + (Lanes([0.0, 0.0, 0.0, v1913[0], v1913[1], v1913[2], 0.0, 0.0, 0.0]));
            let v1917 = v732 / v856;
            let v1920 = v823 / v860;
            let v1921 = (v155 + (v731 / v856)) + (v821 / v860);
            let v1924 = (Lanes([v1917[0], v1917[1], 0.0, 0.0])) + (Lanes([0.0, v1920[0], v1920[1], v1920[2]]));
            let v1925 = v1921 * v1921;
            let v1926 = v1924 * v1921;
            let v1927 = v1926 + v1926;
            let v1928 = if v1921 < v0 { 1.0 } else { 0.0 };
            let v2006: f64;
            let v2007: Lanes<4>;
            if v1928 != 0.0 {
                let v1986 = (v1925 + v1984).sqrt();
                let v1990 = v1986 - v1921;
                let v1993 = v1992 / v1990;
                let v1996 = ((((v1927 * (v305 / (v303 * v1986))) - v1924) * v1993) * v121) / v1990;
                v2006 = v1993;
                v2007 = v1996;
            } else {
                let v1998 = (v1925 + v1984).sqrt();
                let v2004 = v401 * (v1998 + v1921);
                let v2005 = ((v1927 * (v305 / (v303 * v1998))) + v1924) * v401;
                v2006 = v2004;
                v2007 = v2005;
            }
            let v2008 = v2006 * v930;
            let v2013 = v2012 / v2008;
            let v2016 = ((((v2007 * v930) + (v929 * v2006)) * v2013) * v121) / v2008;
            let v2018 = if v2013 < v2017 { 1.0 } else { 0.0 };
            let v2020: f64;
            let v2021: Lanes<4>;
            if v2018 != 0.0 {
                v2020 = v2017;
                v2021 = v2019;
            } else {
                v2020 = v2013;
                v2021 = v2016;
            }
            let v2022 = v728 * v2020;
            let v2023 = v2021 * v728;
            let v2029 = (v199 * v2025) + v47;
            let v2030 = ((v2025 * (v198 - v155)) + v46) / v2022;
            let v2031 = v2023 * v2030;
            let v2035 = ((Lanes([0.0, v2029[0], v2029[1], 0.0, 0.0])) - (Lanes([v2031[0], 0.0, v2031[1], v2031[2], v2031[3]]))) / v2022;
            let v2036 = if v945 > v0 { 1.0 } else { 0.0 };
            let v2039: f64;
            let v2040: Lanes<4>;
            if v2036 != 0.0 {
                let v2038 = if v2037 == v155 { 1.0 } else { 0.0 };
                let v2045: f64;
                let v2046: Lanes<4>;
                if v2038 != 0.0 {
                    let v2043 = if v16 < v2042 { 1.0 } else { 0.0 };
                    let v2054: f64;
                    let v2055: Lanes<4>;
                    if v2043 != 0.0 {
                        let v2051 = (-v945) / v2050;
                        let v2052 = (v948 * v121) / v2050;
                        let v2053 = if v2051 < v149 { 1.0 } else { 0.0 };
                        let v2063: f64;
                        let v2064: Lanes<4>;
                        if v2053 != 0.0 {
                            let v2056 = v2051.exp();
                            let v2057 = v2052 * v2056;
                            v2063 = v2056;
                            v2064 = v2057;
                        } else {
                            let v2058 = v149.exp();
                            let v2061 = v2058 * (v155 + (v2051 - v149));
                            let v2062 = v2052 * v2058;
                            v2063 = v2061;
                            v2064 = v2062;
                        }
                        let v2065 = v2042 - v16;
                        let v2067 = v2065 * v2063;
                        let v2068 = (v17 * v121) * v2063;
                        let v2071 = (Lanes([0.0, v2068[0], v2068[1], 0.0])) + (v2064 * v2065);
                        let v2073 = -v2072;
                        let v2080 = v2073 * (v2067.powf(v2074));
                        let v2081 = (v2071 * (v2074 * (v2067.powf((v2074 - v305))))) * v2073;
                        let v2082 = if v2080 < v149 { 1.0 } else { 0.0 };
                        let v2090: f64;
                        let v2091: Lanes<4>;
                        if v2082 != 0.0 {
                            let v2083 = v2080.exp();
                            let v2084 = v2081 * v2083;
                            v2090 = v2083;
                            v2091 = v2084;
                        } else {
                            let v2085 = v149.exp();
                            let v2088 = v2085 * (v155 + (v2080 - v149));
                            let v2089 = v2081 * v2085;
                            v2090 = v2088;
                            v2091 = v2089;
                        }
                        let v2093 = v2092 / v2072;
                        let v2094 = v2093 * v2067;
                        let v2096 = v2094 * v2090;
                        let v2099 = ((v2071 * v2093) * v2090) + (v2091 * v2094);
                        v2054 = v2096;
                        v2055 = v2099;
                    } else {
                        v2054 = v0;
                        v2055 = v2019;
                    }
                    v2045 = v2054;
                    v2046 = v2055;
                } else {
                    let v2044 = if v2037 == v315 { 1.0 } else { 0.0 };
                    let v2102: f64;
                    let v2103: Lanes<4>;
                    if v2044 != 0.0 {
                        let v2100 = if v16 < v248 { 1.0 } else { 0.0 };
                        let v2126: f64;
                        let v2127: Lanes<4>;
                        if v2100 != 0.0 {
                            let v2108 = (v315 * v2104) / (v2106 * v2106);
                            let v2109 = v248 - v16;
                            let v2110 = v17 * v121;
                            let v2111 = v2109 / v374;
                            let v2113 = Lanes([v2110[0], v2110[1], 0.0]);
                            let v2120 = ((v315 * v2111) / v2108).sqrt();
                            let v2123 = ((((v2113 - (v381 * v2111)) / v374) * v315) / v2108) * (v305 / (v303 * v2120));
                            let v2125 = if v2124 == v0 { 1.0 } else { 0.0 };
                            let v2138: f64;
                            let v2139: Lanes<3>;
                            if v2125 != 0.0 {
                                v2138 = v2106;
                                v2139 = v596;
                            } else {
                                let v2130 = v155 - (v401 * v376);
                                let v2131 = (v383 * v401) * v121;
                                let v2132 = v2106 * v2130;
                                let v2134 = v2132 * v2130;
                                let v2137 = ((v2131 * v2106) * v2130) + (v2131 * v2132);
                                v2138 = v2134;
                                v2139 = v2137;
                            }
                            let v2145 = v2123 * v2120;
                            let v2148 = v2139 * v2138;
                            let v2152 = ((v2120 * v2120) + (v2138 * v2138)).sqrt();
                            let v2156 = (v2120 * v2138) / v2152;
                            let v2159 = (((v2123 * v2138) + (v2139 * v2120)) - ((((v2145 + v2145) + (v2148 + v2148)) * (v305 / (v303 * v2152))) * v2156)) / v2152;
                            let v2160 = v2109 / v2156;
                            let v2163 = (v2113 - (v2159 * v2160)) / v2156;
                            let v2164 = v401 * v2156;
                            let v2165 = v2159 * v401;
                            let v2166 = v2164 * v2108;
                            let v2167 = v2165 * v2108;
                            let v2172 = v2160 + (v2166 * v374);
                            let v2173 = v2163 + ((v2167 * v374) + (v381 * v2166));
                            let v2234: f64;
                            let v2235: Lanes<4>;
                            if v2125 != 0.0 {
                                let v2174 = Lanes([0.0, v2173[0], v2173[1], v2173[2]]);
                                v2234 = v2172;
                                v2235 = v2174;
                            } else {
                                let v2176 = v315 * v2175;
                                let v2186 = v448 * (v155 + (v2176 * (v155 + (v315 * v376))));
                                let v2188 = v945 / v2186;
                                let v2189 = (((v383 * v315) * v2176) * v448) * v2188;
                                let v2193 = ((v155 + v2175) / (v155 + v2176)) - v2188;
                                let v2196 = v2167 * v2193;
                                let v2200 = v2160 - (v2166 * v2193);
                                let v2202 = (Lanes([0.0, v2163[0], v2163[1], v2163[2]])) - ((Lanes([0.0, v2196[0], v2196[1], v2196[2]])) + ((((v948 - (Lanes([0.0, v2189[0], v2189[1], v2189[2]]))) / v2186) * v121) * v2166));
                                let v2203 = v2200 - v2172;
                                let v2204 = Lanes([0.0, v2173[0], v2173[1], v2173[2]]);
                                let v2207 = (v2202 - v2204) * v2203;
                                let v2209 = v630 * v2160;
                                let v2211 = v2209 * v2160;
                                let v2220 = (((((v2163 * v630) * v2160) + (v2163 * v2209)) * v377) + (v384 * v2211)) / v448;
                                let v2226 = ((v2203 * v2203) + ((v2211 * v377) / v448)).sqrt();
                                let v2232 = v401 * ((v2200 + v2172) + v2226);
                                let v2233 = ((v2202 + v2204) + (((v2207 + v2207) + (Lanes([0.0, v2220[0], v2220[1], v2220[2]]))) * (v305 / (v303 * v2226)))) * v401;
                                v2234 = v2232;
                                v2235 = v2233;
                            }
                            let v2239 = (v2234 - v2160) / v2234;
                            let v2242 = ((v2235 - (Lanes([0.0, v2163[0], v2163[1], v2163[2]]))) - (v2235 * v2239)) / v2234;
                            let v2245 = if (v2239.abs()) > v2244 { 1.0 } else { 0.0 };
                            let v2299: f64;
                            let v2300: Lanes<4>;
                            if v2245 != 0.0 {
                                let v2246 = v2164 / v2239;
                                let v2250 = ((Lanes([0.0, v2165[0], v2165[1], v2165[2]])) - (v2242 * v2246)) / v2239;
                                let v2253 = v2251 / v2252;
                                let v2254 = v2253 * v2234;
                                let v2256 = v2254 * v2246;
                                let v2261 = (-v2252) / v2234;
                                let v2264 = ((v2235 * v2261) * v121) / v2234;
                                let v2265 = v2261.exp();
                                let v2267 = v2138 / v2246;
                                let v2272 = v155 + v2267;
                                let v2277 = (v2261 * v2272).exp();
                                let v2279 = v2265 - v2277;
                                let v2281 = v2256 * v2279;
                                let v2284 = ((((v2235 * v2253) * v2246) + (v2250 * v2254)) * v2279) + (((v2264 * v2265) - (((v2264 * v2272) + ((((Lanes([0.0, v2139[0], v2139[1], v2139[2]])) - (v2250 * v2267)) / v2246) * v2261)) * v2277)) * v2256);
                                v2299 = v2281;
                                v2300 = v2284;
                            } else {
                                let v2285 = v2251 * v2138;
                                let v2288 = (-v2252) / v2234;
                                let v2292 = v2288.exp();
                                let v2294 = v2285 * v2292;
                                let v2295 = (v2139 * v2251) * v2292;
                                let v2298 = (Lanes([0.0, v2295[0], v2295[1], v2295[2]])) + (((((v2235 * v2288) * v121) / v2234) * v2292) * v2285);
                                v2299 = v2294;
                                v2300 = v2298;
                            }
                            v2126 = v2299;
                            v2127 = v2300;
                        } else {
                            v2126 = v0;
                            v2127 = v2019;
                        }
                        v2102 = v2126;
                        v2103 = v2127;
                    } else {
                        let v2101 = if v2037 == v728 { 1.0 } else { 0.0 };
                        let v2302: f64;
                        let v2303: Lanes<4>;
                        if v2101 != 0.0 {
                            let v2301 = if v16 < v2042 { 1.0 } else { 0.0 };
                            let v2331: f64;
                            let v2332: Lanes<4>;
                            if v2301 != 0.0 {
                                let v2304 = v2042 - v16;
                                let v2305 = v17 * v121;
                                let v2306 = v2304.powf(v2074);
                                let v2312 = v2311 + v945;
                                let v2313 = v945 / v2312;
                                let v2317 = v155 - v2313;
                                let v2320 = v2317.powf(v2319);
                                let v2325 = v2306 * v2320;
                                let v2326 = (v2305 * (v2074 * (v2304.powf((v2074 - v305))))) * v2320;
                                let v2329 = (Lanes([0.0, v2326[0], v2326[1], 0.0])) + (((((v948 - (v948 * v2313)) / v2312) * v121) * (v2319 * (v2317.powf((v2319 - v305))))) * v2306);
                                let v2330 = if v2124 == v0 { 1.0 } else { 0.0 };
                                let v2342: f64;
                                let v2343: Lanes<4>;
                                if v2330 != 0.0 {
                                    v2342 = v2325;
                                    v2343 = v2329;
                                } else {
                                    let v2335 = (v945 - v2333) / v2311;
                                    let v2336 = v948 / v2311;
                                    let v2339 = (v2335 - v155) / v2338;
                                    let v2340 = v2336 / v2338;
                                    let v2341 = if v2335 < v155 { 1.0 } else { 0.0 };
                                    let v2369: f64;
                                    let v2370: Lanes<4>;
                                    if v2341 != 0.0 {
                                        let v2348 = v2339.exp();
                                        let v2350 = v155 + v2348;
                                        let v2355 = ((v2340 * v2348) * (v305 / v2350)) * v2338;
                                        let v2356 = v155 + (v2338 * (v2350.ln()));
                                        v2369 = v2356;
                                        v2370 = v2355;
                                    } else {
                                        let v2359 = (-v2339).exp();
                                        let v2361 = v155 + v2359;
                                        let v2367 = v2335 + (v2338 * (v2361.ln()));
                                        let v2368 = v2336 + ((((v2340 * v121) * v2359) * (v305 / v2361)) * v2338);
                                        v2369 = v2367;
                                        v2370 = v2368;
                                    }
                                    let v2372 = v2369.powf(v2371);
                                    let v2377 = v2325 * v2372;
                                    let v2380 = (v2329 * v2372) + ((v2370 * (v2371 * (v2369.powf((v2371 - v305))))) * v2325);
                                    v2342 = v2377;
                                    v2343 = v2380;
                                }
                                let v2344 = -v2072;
                                let v2345 = v2344 * v2342;
                                let v2346 = v2343 * v2344;
                                let v2347 = if v2345 < v149 { 1.0 } else { 0.0 };
                                let v2388: f64;
                                let v2389: Lanes<4>;
                                if v2347 != 0.0 {
                                    let v2381 = v2345.exp();
                                    let v2382 = v2346 * v2381;
                                    v2388 = v2381;
                                    v2389 = v2382;
                                } else {
                                    let v2383 = v149.exp();
                                    let v2386 = v2383 * (v155 + (v2345 - v149));
                                    let v2387 = v2346 * v2383;
                                    v2388 = v2386;
                                    v2389 = v2387;
                                }
                                let v2390 = v2092 / v2072;
                                let v2391 = v2390 * v2304;
                                let v2393 = v2391 * v2388;
                                let v2394 = (v2305 * v2390) * v2388;
                                let v2397 = (Lanes([0.0, v2394[0], v2394[1], 0.0])) + (v2389 * v2391);
                                v2331 = v2393;
                                v2332 = v2397;
                            } else {
                                v2331 = v0;
                                v2332 = v2019;
                            }
                            v2302 = v2331;
                            v2303 = v2332;
                        } else {
                            v2302 = v0;
                            v2303 = v2019;
                        }
                        v2102 = v2302;
                        v2103 = v2303;
                    }
                    v2045 = v2102;
                    v2046 = v2103;
                }
                let v2047 = if v2045 > v0 { 1.0 } else { 0.0 };
                let v2400: f64;
                let v2401: Lanes<4>;
                if v2047 != 0.0 {
                    let v2399 = if v2398 == v155 { 1.0 } else { 0.0 };
                    let v2430: f64;
                    let v2431: Lanes<4>;
                    if v2399 != 0.0 {
                        let v2403 = v2402 + v2022;
                        let v2404 = v945 * v2403;
                        let v2408 = v343 / v2404;
                        let v2419 = v2418 / v2403;
                        let v2423 = (v2408 + ((v931 / v938) * v1085)) + v2419;
                        let v2424 = ((((((v948 * v2403) + (v2023 * v945)) * v2408) * v121) / v2404) + ((v934 / v938) * v1085)) + (((v2023 * v2419) * v121) / v2403);
                        let v2425 = if v2037 == v728 { 1.0 } else { 0.0 };
                        let v2452: f64;
                        let v2453: Lanes<4>;
                        if v2425 != 0.0 {
                            let v2435 = (v2045 - v2423) / v2434;
                            let v2436 = (v2046 - v2424) / v2434;
                            let v2437 = if v2045 < v2423 { 1.0 } else { 0.0 };
                            let v2476: f64;
                            let v2477: Lanes<4>;
                            if v2437 != 0.0 {
                                let v2454 = v2435.exp();
                                let v2456 = v155 + v2454;
                                let v2462 = v2045 - (v2434 * (v2456.ln()));
                                let v2463 = v2046 - (((v2436 * v2454) * (v305 / v2456)) * v2434);
                                v2476 = v2462;
                                v2477 = v2463;
                            } else {
                                let v2466 = (-v2435).exp();
                                let v2468 = v155 + v2466;
                                let v2474 = v2423 - (v2434 * (v2468.ln()));
                                let v2475 = v2424 - ((((v2436 * v121) * v2466) * (v305 / v2468)) * v2434);
                                v2476 = v2474;
                                v2477 = v2475;
                            }
                            let v2478 = v945 * v2476;
                            let v2481 = (v948 * v2476) + (v2477 * v945);
                            v2452 = v2478;
                            v2453 = v2481;
                        } else {
                            let v2438 = v945 * v2045;
                            let v2446 = v2045 + v2423;
                            let v2448 = (v2438 * v2423) / v2446;
                            let v2451 = (((((v948 * v2045) + (v2046 * v945)) * v2423) + (v2424 * v2438)) - ((v2046 + v2424) * v2448)) / v2446;
                            v2452 = v2448;
                            v2453 = v2451;
                        }
                        v2430 = v2452;
                        v2431 = v2453;
                    } else {
                        let v2426 = v945 * v2045;
                        let v2429 = (v948 * v2045) + (v2046 * v945);
                        v2430 = v2426;
                        v2431 = v2429;
                    }
                    v2400 = v2430;
                    v2401 = v2431;
                } else {
                    v2400 = v0;
                    v2401 = v2019;
                }
                v2039 = v2400;
                v2040 = v2401;
            } else {
                v2039 = v0;
                v2040 = v2019;
            }
            let v2041 = if v375 > v0 { 1.0 } else { 0.0 };
            let v2483 = v2482 * v731;
            let v2484 = v732 * v2482;
            let v2486 = (v40 - v386) / v388;
            let v2487 = v41 / v388;
            let v2488 = if v40 < v386 { 1.0 } else { 0.0 };
            let v2511: f64;
            let v2512: Lanes<2>;
            if v2488 != 0.0 {
                let v2489 = v2486.exp();
                let v2491 = v155 + v2489;
                let v2497 = v40 - (v388 * (v2491.ln()));
                let v2498 = v41 - (((v2487 * v2489) * (v305 / v2491)) * v388);
                v2511 = v2497;
                v2512 = v2498;
            } else {
                let v2501 = (-v2486).exp();
                let v2503 = v155 + v2501;
                let v2509 = v386 - (v388 * (v2503.ln()));
                let v2510 = ((((v2487 * v121) * v2501) * (v305 / v2503)) * v388) * v121;
                v2511 = v2509;
                v2512 = v2510;
            }
            let v2515 = v155 - (v2511 * v710);
            let v2532 = v2531 * ((v723 * (v155 - (v2515.powf(v715)))) + (v728 * (v40 - v2511)));
            let v2533 = ((((((v2512 * v710) * v121) * (v715 * (v2515.powf(v717)))) * v121) * v723) + ((v41 - v2512) * v728)) * v2531;
            let v2535 = v2534 * v821;
            let v2536 = v823 * v2534;
            let v2538 = v2537 * v833;
            let v2540 = v2538 * v2006;
            let v2541 = (v836 * v2537) * v2006;
            let v2544 = (Lanes([v2541[0], v2541[1], 0.0, 0.0])) + (v2007 * v2538);
            let v2545 = v2537 * v851;
            let v2547 = v2545 * v2006;
            let v2548 = (v854 * v2537) * v2006;
            let v2551 = (Lanes([0.0, v2548[0], v2548[1], v2548[2]])) + (v2007 * v2545);
            let v2554 = (v116 - v738) / v2553;
            let v2555 = v119 / v2553;
            let v2556 = if v116 < v738 { 1.0 } else { 0.0 };
            let v2579: f64;
            let v2580: Lanes<5>;
            if v2556 != 0.0 {
                let v2557 = v2554.exp();
                let v2559 = v155 + v2557;
                let v2565 = v116 - (v2553 * (v2559.ln()));
                let v2566 = v119 - (((v2555 * v2557) * (v305 / v2559)) * v2553);
                v2579 = v2565;
                v2580 = v2566;
            } else {
                let v2569 = (-v2554).exp();
                let v2571 = v155 + v2569;
                let v2577 = v738 - (v2553 * (v2571.ln()));
                let v2578 = ((((v2555 * v121) * v2569) * (v305 / v2571)) * v2553) * v121;
                v2579 = v2577;
                v2580 = v2578;
            }
            let v2583 = v155 - (v2579 / v629);
            let v2611 = ((v2605 * ((v815 * ((v801 * (v155 - (v2583.powf(v789)))) + (v804 * (v116 - v2579)))) + (v818 * v116))) * v2608) * v1742;
            let v2612 = ((((((((((v2580 / v629) * v121) * (v789 * (v2583.powf(v791)))) * v121) * v801) + ((v119 - v2580) * v804)) * v815) + (v119 * v818)) * v2605) * v2608) * v1742;
            let v2614 = (v135 - v738) / v2553;
            let v2615 = v137 / v2553;
            let v2616 = if v135 < v738 { 1.0 } else { 0.0 };
            let v2639: f64;
            let v2640: Lanes<8>;
            if v2616 != 0.0 {
                let v2617 = v2614.exp();
                let v2619 = v155 + v2617;
                let v2625 = v135 - (v2553 * (v2619.ln()));
                let v2626 = v137 - (((v2615 * v2617) * (v305 / v2619)) * v2553);
                v2639 = v2625;
                v2640 = v2626;
            } else {
                let v2629 = (-v2614).exp();
                let v2631 = v155 + v2629;
                let v2637 = v738 - (v2553 * (v2631.ln()));
                let v2638 = ((((v2615 * v121) * v2629) * (v305 / v2631)) * v2553) * v121;
                v2639 = v2637;
                v2640 = v2638;
            }
            let v2643 = v155 - (v2639 / v629);
            let v2670 = ((v2605 * ((v815 * ((v801 * (v155 - (v2643.powf(v789)))) + (v804 * (v135 - v2639)))) + (v818 * v135))) * v2608) * v2669;
            let v2671 = ((((((((((v2640 / v629) * v121) * (v789 * (v2643.powf(v791)))) * v121) * v801) + ((v137 - v2640) * v804)) * v815) + (v137 * v818)) * v2605) * v2608) * v2669;
            let v2675 = (v54 - v2672) / v2674;
            let v2676 = v55 / v2674;
            let v2677 = if v54 < v2672 { 1.0 } else { 0.0 };
            let v2700: f64;
            let v2701: Lanes<2>;
            if v2677 != 0.0 {
                let v2678 = v2675.exp();
                let v2680 = v155 + v2678;
                let v2686 = v54 - (v2674 * (v2680.ln()));
                let v2687 = v55 - (((v2676 * v2678) * (v305 / v2680)) * v2674);
                v2700 = v2686;
                v2701 = v2687;
            } else {
                let v2690 = (-v2675).exp();
                let v2692 = v155 + v2690;
                let v2698 = v2672 - (v2674 * (v2692.ln()));
                let v2699 = ((((v2676 * v121) * v2690) * (v305 / v2692)) * v2674) * v121;
                v2700 = v2698;
                v2701 = v2699;
            }
            let v2705 = v155 - (v2700 / v2702);
            let v2725 = v2724 * ((v2715 * (v155 - (v2705.powf(v2707)))) + (v315 * (v54 - v2700)));
            let v2726 = ((((((v2701 / v2702) * v121) * (v2707 * (v2705.powf(v2709)))) * v121) * v2715) + ((v55 - v2701) * v315)) * v2724;
            let v2728 = v32 / v2727;
            let v2729 = v33 / v2727;
            let v2730 = if v2728 < v149 { 1.0 } else { 0.0 };
            let v2738: f64;
            let v2739: Lanes<2>;
            if v2730 != 0.0 {
                let v2731 = v2728.exp();
                let v2732 = v2729 * v2731;
                v2738 = v2731;
                v2739 = v2732;
            } else {
                let v2733 = v149.exp();
                let v2736 = v2733 * (v155 + (v2728 - v149));
                let v2737 = v2729 * v2733;
                v2738 = v2736;
                v2739 = v2737;
            }
            let v2741 = v2740 * v2738;
            let v2742 = v2739 * v2740;
            let v2744 = v2743 * v376;
            let v2749 = (v378 + v326) + v315;
            let v2750 = v2744 * v2749;
            let v2753 = ((v383 * v2743) * v2749) + ((v385 + (Lanes([v327[0], v327[1], 0.0]))) * v2744);
            let v2777: f64;
            let v2778: Lanes<5>;
            if v2754 != 0.0 {
                let v2767 = (v2763 * ((v2755 * v1470) + (v2758 * v1480))) / v2766;
                let v2768 = (((v1473 * v2755) + (v1483 * v2758)) * v2763) / v2766;
                v2777 = v2767;
                v2778 = v2768;
            } else {
                let v2774 = ((v116 - v2769) / v2771) * v146;
                let v2775 = (v119 / v2771) * v146;
                let v2776 = if v2774 < v149 { 1.0 } else { 0.0 };
                let v2787: f64;
                let v2788: Lanes<5>;
                if v2776 != 0.0 {
                    let v2780 = v2774.exp();
                    let v2781 = v2775 * v2780;
                    v2787 = v2780;
                    v2788 = v2781;
                } else {
                    let v2782 = v149.exp();
                    let v2785 = v2782 * (v155 + (v2774 - v149));
                    let v2786 = v2775 * v2782;
                    v2787 = v2785;
                    v2788 = v2786;
                }
                let v2795 = (v155 + (v298 * v2787)).sqrt();
                let v2799 = v155 + v2795;
                let v2800 = (v2789 * v186) / v2799;
                let v2803 = ((v187 * v2789) - (((v2788 * v298) * (v305 / (v303 * v2795))) * v2800)) / v2799;
                v2777 = v2800;
                v2778 = v2803;
            }
            let v2806: f64;
            let v2807: f64;
            let v2808: Lanes<9>;
            let v2809: Lanes<5>;
            if v2779 != 0.0 {
                let v2804 = v2777 * v1742;
                let v2805 = v2778 * v1742;
                let v2850: f64;
                let v2851: Lanes<8>;
                if v2754 != 0.0 {
                    let v2811 = v824 * v210;
                    let v2812 = v211 * v824;
                    let v2815 = (v155 + v2811).sqrt();
                    let v2819 = v155 + v2815;
                    let v2820 = (v2811 - v824) / v2819;
                    let v2824 = v298 * v259;
                    let v2825 = v260 * v298;
                    let v2827 = (v155 + v2824).sqrt();
                    let v2831 = v155 + v2827;
                    let v2832 = v2824 / v2831;
                    let v2845 = (v2842 * ((v2755 * v2820) + (v2758 * v2832))) / v2766;
                    let v2846 = (((((v2812 - ((v2812 * (v305 / (v303 * v2815))) * v2820)) / v2819) * v2755) + (((v2825 - ((v2825 * (v305 / (v303 * v2827))) * v2832)) / v2831) * v2758)) * v2842) / v2766;
                    v2850 = v2845;
                    v2851 = v2846;
                } else {
                    let v2848 = (v135 - v2769) * v146;
                    let v2849 = if v2848 < v149 { 1.0 } else { 0.0 };
                    let v2864: f64;
                    let v2865: Lanes<8>;
                    if v2849 != 0.0 {
                        let v2857 = v2848.exp();
                        let v2858 = v201 * v2857;
                        v2864 = v2857;
                        v2865 = v2858;
                    } else {
                        let v2859 = v149.exp();
                        let v2862 = v2859 * (v155 + (v2848 - v149));
                        let v2863 = v201 * v2859;
                        v2864 = v2862;
                        v2865 = v2863;
                    }
                    let v2872 = (v155 + (v298 * v2864)).sqrt();
                    let v2876 = v155 + v2872;
                    let v2877 = (v2866 * v210) / v2876;
                    let v2880 = ((v211 * v2866) - (((v2865 * v298) * (v305 / (v303 * v2872))) * v2877)) / v2876;
                    v2850 = v2877;
                    v2851 = v2880;
                }
                let v2852 = v1766 * v2850;
                let v2854 = v2851 * v1766;
                let v2856 = (v1771 * v2850) + (Lanes([v2854[0], v2854[1], 0.0, v2854[2], v2854[3], v2854[4], v2854[5], v2854[6], v2854[7]]));
                v2806 = v2852;
                v2807 = v2804;
                v2808 = v2856;
                v2809 = v2805;
            } else {
                v2806 = v0;
                v2807 = v2777;
                v2808 = v1763;
                v2809 = v2778;
            }
            let v2890: f64;
            let v2891: f64;
            let v2892: f64;
            let v2893: f64;
            let v2894: Lanes<4>;
            let v2895: Lanes<2>;
            let v2896: Lanes<4>;
            let v2897: Lanes<5>;
            if v2810 != 0.0 {
                let v2886 = v714 * (v2881 * (v713.powf(v2883)));
                let v2887 = (v713.powf(v2881)) - v728;
                let v2888 = if v389 < v0 { 1.0 } else { 0.0 };
                let v2952: f64;
                let v2953: Lanes<2>;
                if v2888 != 0.0 {
                    let v2936 = v389.exp();
                    let v2938 = v155 + v2936;
                    let v2939 = v155 / v2938;
                    let v2942 = (((v390 * v2936) * v2939) * v121) / v2938;
                    v2952 = v2939;
                    v2953 = v2942;
                } else {
                    let v2945 = (-v389).exp();
                    let v2946 = (v390 * v121) * v2945;
                    let v2947 = v155 + v2945;
                    let v2948 = v2945 / v2947;
                    let v2951 = (v2946 - (v2946 * v2948)) / v2947;
                    v2952 = v2948;
                    v2953 = v2951;
                }
                let v2960 = ((v2886 * v2952) + (v2953 * v2887)) * v2482;
                let v2963 = (v825 * v146) / v163;
                let v2965 = v401 / v828;
                let v2969 = v2963 * v2965;
                let v2973 = v2537 * v2006;
                let v2977 = ((((v826 * v146) / v163) * v2965) + ((((v831 * v2965) * v121) / v828) * v2963)) * v2973;
                let v2981 = v2742 / v2727;
                let v2982 = v418 * v46;
                let v2987 = ((v2482 * ((v2887 * v2952) + v728)) + (v2973 * v2969)) + (v2741 / v2727);
                let v2990 = v2982 * v2987;
                let v2991 = (v47 * v418) * v2987;
                let v2992 = (((Lanes([v2960[0], v2960[1], 0.0, 0.0])) + (((v2007 * v2537) * v2969) + (Lanes([v2977[0], v2977[1], 0.0, 0.0])))) + (Lanes([v2981[0], v2981[1], 0.0, 0.0]))) * v2982;
                let v2995 = (Lanes([0.0, v2991[0], v2991[1], 0.0, 0.0])) + (Lanes([v2992[0], 0.0, v2992[1], v2992[2], v2992[3]]));
                let v2997 = v2996 * v2741;
                let v2998 = v2742 * v2996;
                let v3001 = v2742 * v2999;
                let v3002 = v2540 + (v2999 * v2741);
                let v3004 = v2544 + (Lanes([v3001[0], v3001[1], 0.0, 0.0]));
                let v3008 = (v3005 * v3002) + v2547;
                let v3009 = (v3004 * v3005) + v2551;
                let v3011 = v3010 * v3002;
                let v3012 = v3004 * v3010;
                v2890 = v3011;
                v2891 = v2997;
                v2892 = v3008;
                v2893 = v2990;
                v2894 = v3012;
                v2895 = v2998;
                v2896 = v3009;
                v2897 = v2995;
            } else {
                v2890 = v2540;
                v2891 = v2741;
                v2892 = v2547;
                v2893 = v0;
                v2894 = v2544;
                v2895 = v2742;
                v2896 = v2551;
                v2897 = v2889;
            }
            let v2901 = (v15 * v350) * v2900;
            let v2902 = (v351 * v15) * v2900;
            let v2905 = (v15 * v945) * v2900;
            let v2906 = (v948 * v15) * v2900;
            let v2913 = (v15 * ((v1178 + v1241) + v1275)) * v2900;
            let v2914 = (((v1179 + v1242) + v1276) * v15) * v2900;
            let v2919 = v33 * v1;
            let v2934 = (v15 * (((((v1052 + v1224) + (v1 * v32)) - v1293) + v1030) + v990)) * v2900;
            let v2935 = ((((((v1053 + (Lanes([v1225[0], v1225[1], 0.0, 0.0]))) + (Lanes([v2919[0], v2919[1], 0.0, 0.0]))) - (Lanes([v1294[0], v1294[1], 0.0, 0.0]))) + (Lanes([v1033[0], v1033[1], 0.0, 0.0]))) + (Lanes([v991[0], v991[1], 0.0, 0.0]))) * v15) * v2900;
            let v3025: f64;
            let v3026: f64;
            let v3027: Lanes<3>;
            let v3028: Lanes<3>;
            if v4 != 0.0 {
                let v3017 = (v15 * (-v1896)) * v2900;
                let v3018 = ((v1900 * v121) * v15) * v2900;
                v3025 = v3017;
                v3026 = v0;
                v3027 = v3018;
                v3028 = v1893;
            } else {
                let v3023 = (v15 * (-v1896)) * v2900;
                let v3024 = ((v1900 * v121) * v15) * v2900;
                v3025 = v0;
                v3026 = v3023;
                v3027 = v1893;
                v3028 = v3024;
            }
            let v3031 = (v15 * v1767) * v2900;
            let v3032 = (v1772 * v15) * v2900;
            let v3035 = (v15 * v1717) * v2900;
            let v3036 = (v1719 * v15) * v2900;
            let v3039 = (v15 * v1768) * v2900;
            let v3040 = (v1773 * v15) * v2900;
            let v3043 = (v15 * v1739) * v2900;
            let v3044 = (v1740 * v15) * v2900;
            let v3047 = (v15 * v2030) * v2900;
            let v3048 = (v2035 * v15) * v2900;
            let v3054 = (v15 * (v3049 * v2039)) * v2900;
            let v3055 = ((v2040 * v3049) * v15) * v2900;
            let v3060 = ((v15 * v68) / v2418) * v2900;
            let v3061 = ((v69 * v15) / v2418) * v2900;
            let v3066 = ((v15 * v76) / v2402) * v2900;
            let v3067 = ((v77 * v15) / v2402) * v2900;
            let v3074 = v15 * ((v2483 + v2890) + v2891);
            let v3075 = (((Lanes([v2484[0], v2484[1], 0.0, 0.0])) + v2894) + (Lanes([v2895[0], v2895[1], 0.0, 0.0]))) * v15;
            let v3079 = (ddt(13137, v3074)) * v2900;
            let v3080 = (v3075 * v3077) * v2900;
            let v3081 = v3074 * v2900;
            let v3082 = v3075 * v2900;
            let v3083 = v15 * v2532;
            let v3084 = v2533 * v15;
            let v3087 = (ddt(13143, v3083)) * v2900;
            let v3088 = (v3084 * v3077) * v2900;
            let v3089 = v3083 * v2900;
            let v3090 = v3084 * v2900;
            let v3097 = v15 * ((v2535 + v2892) + v2750);
            let v3098 = (((Lanes([0.0, v2536[0], v2536[1], v2536[2]])) + v2896) + (Lanes([0.0, v2753[0], v2753[1], v2753[2]]))) * v15;
            let v3101 = (ddt(13153, v3097)) * v2900;
            let v3102 = (v3098 * v3077) * v2900;
            let v3103 = v3097 * v2900;
            let v3104 = v3098 * v2900;
            let v3105 = v15 * v2725;
            let v3106 = v2726 * v15;
            let v3109 = (ddt(13159, v3105)) * v2900;
            let v3110 = (v3106 * v3077) * v2900;
            let v3111 = v3105 * v2900;
            let v3112 = v3106 * v2900;
            let v3113 = v15 * v2893;
            let v3114 = v2897 * v15;
            let v3117 = (ddt(13165, v3113)) * v2900;
            let v3118 = (v3114 * v3077) * v2900;
            let v3119 = v3113 * v2900;
            let v3120 = v3114 * v2900;
            let v3122 = v3121 * v82;
            let v3123 = v83 * v3121;
            let v3126 = (ddt(13173, v3122)) * v2900;
            let v3127 = (v3123 * v3077) * v2900;
            let v3128 = v3122 * v2900;
            let v3129 = v3123 * v2900;
            let v3131 = v3130 * v90;
            let v3132 = v91 * v3130;
            let v3135 = (ddt(13181, v3131)) * v2900;
            let v3136 = (v3132 * v3077) * v2900;
            let v3137 = v3131 * v2900;
            let v3138 = v3132 * v2900;
            let v3141 = (v15 * v1911) * v2900;
            let v3142 = (v1915 * v15) * v2900;
            let v3148 = ((v15 * v131) * v3145) * v2900;
            let v3149 = ((v134 * v15) * v3145) * v2900;
            let v3153 = v15 * (v2670 + v2806);
            let v3154 = ((Lanes([v2671[0], v2671[1], 0.0, v2671[2], v2671[3], v2671[4], v2671[5], v2671[6], v2671[7]])) + v2808) * v15;
            let v3157 = (ddt(13201, v3153)) * v2900;
            let v3158 = (v3154 * v3077) * v2900;
            let v3159 = v3153 * v2900;
            let v3160 = v3154 * v2900;
            let v3169 = (v15 * ((v1906 + (v1 * v116)) + v1901)) * v2900;
            let v3170 = (((v1910 + (v119 * v1)) + v1905) * v15) * v2900;
            let v3173 = v15 * (v2611 + v2807);
            let v3174 = (v2612 + v2809) * v15;
            let v3177 = (ddt(13220, v3173)) * v2900;
            let v3178 = (v3174 * v3077) * v2900;
            let v3179 = v3173 * v2900;
            let v3180 = v3174 * v2900;
            let v3189: f64;
            let v3190: Lanes<2>;
            if v5 != 0.0 {
                let v3186 = ((v15 * v106) * v3183) * v2900;
                let v3187 = ((v107 * v15) * v3183) * v2900;
                v3189 = v3186;
                v3190 = v3187;
            } else {
                v3189 = v0;
                v3190 = v3188;
            }
            let v3199: f64;
            let v3200: Lanes<2>;
            if v6 != 0.0 {
                let v3196 = ((v15 * v98) * v3193) * v2900;
                let v3197 = ((v99 * v15) * v3193) * v2900;
                v3199 = v3196;
                v3200 = v3197;
            } else {
                v3199 = v0;
                v3200 = v3198;
            }
            let v3203 = (v939 + v936) / v931;
            let v3206 = ((v942 + v943) - (v934 * v3203)) / v931;
            let v3218: f64;
            let v3219: Lanes<4>;
            if v3207 != 0.0 {
                let v3208 = v2039 / v3203;
                let v3212 = v3208.abs();
                let v3217 = ((v2040 - (v3206 * v3208)) / v3203) * ((v303 * (if v3208 >= v3213 { 1.0 } else { 0.0 })) - v305);
                v3218 = v3212;
                v3219 = v3217;
            } else {
                v3218 = v0;
                v3219 = v2019;
            }
            let v3220 = if v3203 > v0 { 1.0 } else { 0.0 };
            let v3234: f64;
            let v3235: Lanes<4>;
            if v3220 != 0.0 {
                let v3223 = (v2890 + v2892) / v3203;
                let v3226 = ((v2894 + v2896) - (v3206 * v3223)) / v3203;
                v3234 = v3223;
                v3235 = v3226;
            } else {
                let v3228 = v3227 * v2006;
                let v3230 = v3228 * v931;
                let v3233 = ((v2007 * v3227) * v931) + (v934 * v3228);
                v3234 = v3230;
                v3235 = v3233;
            }
            let v3240: f64;
            let v3241: Lanes<4>;
            if v3236 != 0.0 {
                let v3237 = v3005 * v3234;
                let v3238 = v3235 * v3005;
                v3240 = v3237;
                v3241 = v3238;
            } else {
                let v3247: f64;
                let v3248: Lanes<4>;
                if v3239 != 0.0 {
                    let v3245 = v3244 * v3234;
                    let v3246 = v3235 * v3244;
                    v3247 = v3245;
                    v3248 = v3246;
                } else {
                    v3247 = v0;
                    v3248 = v2019;
                }
                v3240 = v3247;
                v3241 = v3248;
            }
            let v3243 = if (v1052 + v1178) < v0 { 1.0 } else { 0.0 };
            let v3251 = if ((v1224 + v1241) + v1275) < v0 { 1.0 } else { 0.0 };
            let v3252 = if v1906 < v0 { 1.0 } else { 0.0 };
            let v3253 = if v1901 < v0 { 1.0 } else { 0.0 };
            let v3254 = if v1911 < v0 { 1.0 } else { 0.0 };
            let v3256 = ddt(13491, v3255);
            let v3259 = v3240 * v3256;
            let v3260 = v3241 * v3256;
            let v3261 = (v3257 * v3077) * v3240;
            let v3264 = (Lanes([v3260[0], v3260[1], v3260[2], v3260[3], 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, v3261[0]]));
            let v3265 = v3240 * v3255;
            let v3266 = v3241 * v3255;
            let v3267 = v3257 * v3240;
            let v3270 = (Lanes([v3266[0], v3266[1], v3266[2], v3266[3], 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, v3267[0]]));
            let v3271 = v3218 * v3255;
            let v3272 = v3219 * v3255;
            let v3273 = v3257 * v3218;
            let v3276 = (Lanes([v3272[0], v3272[1], v3272[2], v3272[3], 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, v3273[0]]));
            let v3282 = if (((((v3039 + v3066) + v3126) + v3135) + v3141) + v3157) == v0 { 1.0 } else { 0.0 };
            let v3283 = v2902[0];
            let v3284 = v2902[1];
            let v3285 = v2902[2];
            let v3286 = v2906[0];
            let v3287 = v2906[1];
            let v3288 = v2906[2];
            let v3289 = v2906[3];
            let v3290 = v2914[0];
            let v3291 = v2914[1];
            let v3292 = v2935[0];
            let v3293 = v2935[1];
            let v3294 = v2935[2];
            let v3295 = v2935[3];
            let v3296 = v3027[0];
            let v3297 = v3027[1];
            let v3298 = v3027[2];
            let v3299 = v3028[0];
            let v3300 = v3028[1];
            let v3301 = v3028[2];
            let v3302 = v3032[0];
            let v3303 = v3032[1];
            let v3304 = v3032[2];
            let v3305 = v3032[3];
            let v3306 = v3032[4];
            let v3307 = v3032[5];
            let v3308 = v3036[0];
            let v3309 = v3036[1];
            let v3310 = v3036[2];
            let v3311 = v3036[3];
            let v3312 = v3040[0];
            let v3313 = v3040[1];
            let v3314 = v3040[2];
            let v3315 = v3040[3];
            let v3316 = v3040[4];
            let v3317 = v3040[5];
            let v3318 = v3040[6];
            let v3319 = v3040[7];
            let v3320 = v3040[8];
            let v3321 = v3044[0];
            let v3322 = v3044[1];
            let v3323 = v3048[0];
            let v3324 = v3048[1];
            let v3325 = v3048[2];
            let v3326 = v3048[3];
            let v3327 = v3048[4];
            let v3328 = v3055[0];
            let v3329 = v3055[1];
            let v3330 = v3055[2];
            let v3331 = v3055[3];
            let v3332 = v3061[0];
            let v3333 = v3061[1];
            let v3334 = v3067[0];
            let v3335 = v3067[1];
            let v3336 = v3080[0];
            let v3337 = v3080[1];
            let v3338 = v3080[2];
            let v3339 = v3080[3];
            let v3340 = v3088[0];
            let v3341 = v3088[1];
            let v3342 = v3102[0];
            let v3343 = v3102[1];
            let v3344 = v3102[2];
            let v3345 = v3102[3];
            let v3346 = v3110[0];
            let v3347 = v3110[1];
            let v3348 = v3118[0];
            let v3349 = v3118[1];
            let v3350 = v3118[2];
            let v3351 = v3118[3];
            let v3352 = v3118[4];
            let v3353 = v3127[0];
            let v3354 = v3127[1];
            let v3355 = v3136[0];
            let v3356 = v3136[1];
            let v3357 = v3142[0];
            let v3358 = v3142[1];
            let v3359 = v3142[2];
            let v3360 = v3142[3];
            let v3361 = v3142[4];
            let v3362 = v3142[5];
            let v3363 = v3142[6];
            let v3364 = v3142[7];
            let v3365 = v3142[8];
            let v3366 = v3149[0];
            let v3367 = v3149[1];
            let v3368 = v3149[2];
            let v3369 = v3149[3];
            let v3370 = v3149[4];
            let v3371 = v3149[5];
            let v3372 = v3149[6];
            let v3373 = v3149[7];
            let v3374 = v3158[0];
            let v3375 = v3158[1];
            let v3376 = v3158[2];
            let v3377 = v3158[3];
            let v3378 = v3158[4];
            let v3379 = v3158[5];
            let v3380 = v3158[6];
            let v3381 = v3158[7];
            let v3382 = v3158[8];
            let v3383 = v3170[0];
            let v3384 = v3170[1];
            let v3385 = v3170[2];
            let v3386 = v3170[3];
            let v3387 = v3170[4];
            let v3388 = v3178[0];
            let v3389 = v3178[1];
            let v3390 = v3178[2];
            let v3391 = v3178[3];
            let v3392 = v3178[4];
            let v3393 = v3190[0];
            let v3394 = v3190[1];
            let v3395 = v3200[0];
            let v3396 = v3200[1];
            let v3397 = v3257[0];
            let v3398 = v3264[0];
            let v3399 = v3264[1];
            let v3400 = v3264[2];
            let v3401 = v3264[3];
            let v3402 = v3264[4];
            let v3403 = v3276[0];
            let v3404 = v3276[1];
            let v3405 = v3276[2];
            let v3406 = v3276[3];
            let v3407 = v3276[4];
            let v3408 = v3082[0];
            let v3409 = v3082[1];
            let v3410 = v3082[2];
            let v3411 = v3082[3];
            let v3412 = v3090[0];
            let v3413 = v3090[1];
            let v3414 = v3104[0];
            let v3415 = v3104[1];
            let v3416 = v3104[2];
            let v3417 = v3104[3];
            let v3418 = v3112[0];
            let v3419 = v3112[1];
            let v3420 = v3120[0];
            let v3421 = v3120[1];
            let v3422 = v3120[2];
            let v3423 = v3120[3];
            let v3424 = v3120[4];
            let v3425 = v3129[0];
            let v3426 = v3129[1];
            let v3427 = v3138[0];
            let v3428 = v3138[1];
            let v3429 = v3160[0];
            let v3430 = v3160[1];
            let v3431 = v3160[2];
            let v3432 = v3160[3];
            let v3433 = v3160[4];
            let v3434 = v3160[5];
            let v3435 = v3160[6];
            let v3436 = v3160[7];
            let v3437 = v3160[8];
            let v3438 = v3180[0];
            let v3439 = v3180[1];
            let v3440 = v3180[2];
            let v3441 = v3180[3];
            let v3442 = v3180[4];
            let v3443 = v3270[0];
            let v3444 = v3270[1];
            let v3445 = v3270[2];
            let v3446 = v3270[3];
            let v3447 = v3270[4];
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(8),
            multiplicity * (v2901),
            [6, 7, 8],
            [v3283, v3284, v3285],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(4),
            multiplicity * (v2905),
            [4, 6, 7, 8],
            [v3286, v3287, v3288, v3289],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(5),
            Some(4),
            multiplicity * (v2913),
            [4, 5],
            [v3290, v3291],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(4),
            multiplicity * (v2934),
            [4, 6, 7, 8],
            [v3292, v3293, v3294, v3295],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(6),
            Some(7),
            multiplicity * (v3025),
            [5, 6, 7],
            [v3296, v3297, v3298],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(6),
            Some(8),
            multiplicity * (v3026),
            [5, 6, 7],
            [v3299, v3300, v3301],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(3),
            multiplicity * (v3031),
            [3, 5, 6, 7, 8, 10],
            [v3302, v3303, v3304, v3305, v3306, v3307],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(3),
            multiplicity * (v3035),
            [3, 6, 7, 8],
            [v3308, v3309, v3310, v3311],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(3),
            multiplicity * (v3039),
            [0, 1, 3, 5, 6, 7, 8, 9, 10],
            [v3312, v3313, v3314, v3315, v3316, v3317, v3318, v3319, v3320],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(3),
            Some(7),
            multiplicity * (v3043),
            [3, 7],
            [v3321, v3322],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * (v3047),
            [4, 5, 6, 7, 8],
            [v3323, v3324, v3325, v3326, v3327],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(8),
            multiplicity * (v3054),
            [4, 6, 7, 8],
            [v3328, v3329, v3330, v3331],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(2),
            Some(4),
            multiplicity * (v3060),
            [2, 4],
            [v3332, v3333],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(5),
            multiplicity * (v3066),
            [1, 5],
            [v3334, v3335],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(4),
            multiplicity * (v3079),
            [4, 6, 7, 8],
            [v3336, v3337, v3338, v3339],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(5),
            Some(4),
            multiplicity * (v3087),
            [4, 5],
            [v3340, v3341],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(8),
            multiplicity * (v3101),
            [4, 6, 7, 8],
            [v3342, v3343, v3344, v3345],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(3),
            Some(7),
            multiplicity * (v3109),
            [3, 7],
            [v3346, v3347],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * (v3117),
            [4, 5, 6, 7, 8],
            [v3348, v3349, v3350, v3351, v3352],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(2),
            multiplicity * (v3126),
            [1, 2],
            [v3353, v3354],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(0),
            multiplicity * (v3135),
            [0, 1],
            [v3355, v3356],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(9),
            multiplicity * (v3141),
            [0, 1, 3, 5, 6, 7, 8, 9, 10],
            [v3357, v3358, v3359, v3360, v3361, v3362, v3363, v3364, v3365],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(0),
            Some(9),
            multiplicity * (v3148),
            [0, 1, 5, 6, 7, 8, 9, 10],
            [v3366, v3367, v3368, v3369, v3370, v3371, v3372, v3373],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(9),
            multiplicity * (v3157),
            [0, 1, 3, 5, 6, 7, 8, 9, 10],
            [v3374, v3375, v3376, v3377, v3378, v3379, v3380, v3381, v3382],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(10),
            multiplicity * (v3169),
            [5, 6, 7, 8, 10],
            [v3383, v3384, v3385, v3386, v3387],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(10),
            multiplicity * (v3177),
            [5, 6, 7, 8, 10],
            [v3388, v3389, v3390, v3391, v3392],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(9),
            Some(10),
            multiplicity * (v3189),
            [9, 10],
            [v3393, v3394],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(9), Some(10), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            staged[219],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(10),
            Some(7),
            multiplicity * (v3199),
            [7, 10],
            [v3395, v3396],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(10), Some(7), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            staged[220],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(11),
            None,
            multiplicity * (v3448),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(11),
            None,
            multiplicity * (v3255),
            [11],
            [v3397],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(4),
            multiplicity * (v3259),
            [4, 6, 7, 8, 11],
            [v3398, v3399, v3400, v3401, v3402],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (v3271),
            [4, 6, 7, 8, 11],
            [v3403, v3404, v3405, v3406, v3407],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(8),
            Some(4),
            multiplicity * (v3255),
            [11],
            [v3397],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(6),
            multiplicity * (v3449),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(4),
            multiplicity * (v3450),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(2),
            Some(4),
            multiplicity * (v3451),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(5),
            multiplicity * (v3452),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(6),
            multiplicity * (v3453),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(4),
            multiplicity * (v3454),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(4),
            multiplicity * (v3455),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(4),
            multiplicity * (v3456),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(10),
            multiplicity * (v3457),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(10),
            multiplicity * (v3458),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(10),
            multiplicity * (v3459),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(10),
            multiplicity * (v3460),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(9),
            multiplicity * (v3461),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(9),
            multiplicity * (v3462),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(6),
            multiplicity * (staged[221]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(6),
            multiplicity * (staged[222]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(3),
            multiplicity * (v3463),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(3),
            multiplicity * (v3464),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(3),
            multiplicity * (v3465),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(9),
            multiplicity * (staged[223]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(10),
            multiplicity * (staged[224]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(10),
            Some(7),
            multiplicity * (staged[225]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(9),
            multiplicity * (staged[226]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(7),
            multiplicity * (staged[227]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(10),
            multiplicity * (staged[228]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(10),
            Some(7),
            multiplicity * (staged[229]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(7),
            multiplicity * (staged[230]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = v2901;
        self.canonical_reactive[1] = v2905;
        self.canonical_reactive[2] = v2913;
        self.canonical_reactive[3] = v2934;
        self.canonical_reactive[4] = v3025;
        self.canonical_reactive[5] = v3026;
        self.canonical_reactive[6] = v3031;
        self.canonical_reactive[7] = v3035;
        self.canonical_reactive[8] = v3039;
        self.canonical_reactive[9] = v3043;
        self.canonical_reactive[10] = v3047;
        self.canonical_reactive[11] = v3054;
        self.canonical_reactive[12] = v3060;
        self.canonical_reactive[13] = v3066;
        self.canonical_reactive[14] = v3081;
        self.canonical_reactive[15] = v3408;
        self.canonical_reactive[16] = v3409;
        self.canonical_reactive[17] = v3410;
        self.canonical_reactive[18] = v3411;
        self.canonical_reactive[19] = v3089;
        self.canonical_reactive[20] = v3412;
        self.canonical_reactive[21] = v3413;
        self.canonical_reactive[22] = v3103;
        self.canonical_reactive[23] = v3414;
        self.canonical_reactive[24] = v3415;
        self.canonical_reactive[25] = v3416;
        self.canonical_reactive[26] = v3417;
        self.canonical_reactive[27] = v3111;
        self.canonical_reactive[28] = v3418;
        self.canonical_reactive[29] = v3419;
        self.canonical_reactive[30] = v3119;
        self.canonical_reactive[31] = v3420;
        self.canonical_reactive[32] = v3421;
        self.canonical_reactive[33] = v3422;
        self.canonical_reactive[34] = v3423;
        self.canonical_reactive[35] = v3424;
        self.canonical_reactive[36] = v3128;
        self.canonical_reactive[37] = v3425;
        self.canonical_reactive[38] = v3426;
        self.canonical_reactive[39] = v3137;
        self.canonical_reactive[40] = v3427;
        self.canonical_reactive[41] = v3428;
        self.canonical_reactive[42] = v3141;
        self.canonical_reactive[43] = v3148;
        self.canonical_reactive[44] = v3159;
        self.canonical_reactive[45] = v3429;
        self.canonical_reactive[46] = v3430;
        self.canonical_reactive[47] = v3431;
        self.canonical_reactive[48] = v3432;
        self.canonical_reactive[49] = v3433;
        self.canonical_reactive[50] = v3434;
        self.canonical_reactive[51] = v3435;
        self.canonical_reactive[52] = v3436;
        self.canonical_reactive[53] = v3437;
        self.canonical_reactive[54] = v3169;
        self.canonical_reactive[55] = v3179;
        self.canonical_reactive[56] = v3438;
        self.canonical_reactive[57] = v3439;
        self.canonical_reactive[58] = v3440;
        self.canonical_reactive[59] = v3441;
        self.canonical_reactive[60] = v3442;
        self.canonical_reactive[61] = v3189;
        self.canonical_reactive[62] = staged[219];
        self.canonical_reactive[63] = v3199;
        self.canonical_reactive[64] = staged[220];
        self.canonical_reactive[65] = v3448;
        self.canonical_reactive[66] = v3255;
        self.canonical_reactive[67] = v3265;
        self.canonical_reactive[68] = v3443;
        self.canonical_reactive[69] = v3444;
        self.canonical_reactive[70] = v3445;
        self.canonical_reactive[71] = v3446;
        self.canonical_reactive[72] = v3447;
        self.canonical_reactive[73] = v3271;
        self.canonical_reactive[74] = v3255;
        self.canonical_reactive[75] = v3449;
        self.canonical_reactive[76] = v3450;
        self.canonical_reactive[77] = v3451;
        self.canonical_reactive[78] = v3452;
        self.canonical_reactive[79] = v3453;
        self.canonical_reactive[80] = v3454;
        self.canonical_reactive[81] = v3455;
        self.canonical_reactive[82] = v3456;
        self.canonical_reactive[83] = v3457;
        self.canonical_reactive[84] = v3458;
        self.canonical_reactive[85] = v3459;
        self.canonical_reactive[86] = v3460;
        self.canonical_reactive[87] = v3461;
        self.canonical_reactive[88] = v3462;
        self.canonical_reactive[89] = staged[221];
        self.canonical_reactive[90] = staged[222];
        self.canonical_reactive[91] = v3463;
        self.canonical_reactive[92] = v3464;
        self.canonical_reactive[93] = v3465;
        self.canonical_reactive[94] = staged[223];
        self.canonical_reactive[95] = staged[224];
        self.canonical_reactive[96] = staged[225];
        self.canonical_reactive[97] = staged[226];
        self.canonical_reactive[98] = staged[227];
        self.canonical_reactive[99] = staged[228];
        self.canonical_reactive[100] = staged[229];
        self.canonical_reactive[101] = staged[230];
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(4),
            &[4, 6, 7, 8],
            &[cached[15], cached[16], cached[17], cached[18]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(4),
            &[4, 5],
            &[cached[20], cached[21]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(8),
            &[4, 6, 7, 8],
            &[cached[23], cached[24], cached[25], cached[26]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            Some(7),
            &[3, 7],
            &[cached[28], cached[29]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(6),
            &[4, 5, 6, 7, 8],
            &[cached[31], cached[32], cached[33], cached[34], cached[35]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(2),
            &[1, 2],
            &[cached[37], cached[38]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(0),
            &[0, 1],
            &[cached[40], cached[41]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(9),
            &[0, 1, 3, 5, 6, 7, 8, 9, 10],
            &[cached[45], cached[46], cached[47], cached[48], cached[49], cached[50], cached[51], cached[52], cached[53]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(10),
            &[5, 6, 7, 8, 10],
            &[cached[56], cached[57], cached[58], cached[59], cached[60]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(4),
            &[4, 6, 7, 8, 11],
            &[cached[68], cached[69], cached[70], cached[71], cached[72]],
            &[],
            &[],
            multiplicity,
        );
    }

}

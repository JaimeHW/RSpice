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
        let produced: [f64; 172] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let v0 = parameters[0];
                let v1 = 3.1e2f64;
                let v3 = 1.3806226e-23f64;
                let v4 = 1.6021918e-19f64;
                let v5 = 1.380649e-23f64;
                let v6 = 1.602176634e-19f64;
                let v9 = parameters[146];
                let v10 = 2.7315e2f64;
                let v13 = 3e2f64;
                let v16 = 1e0f64;
                let v18 = parameters[121];
                let v22 = parameters[122];
                let v24 = parameters[131];
                let v26 = parameters[117];
                let v29 = parameters[118];
                let v32 = parameters[119];
                let v36 = 5e-1f64;
                let v44 = parameters[120];
                let v48 = 3e0f64;
                let v51 = parameters[130];
                let v53 = parameters[138];
                let v55 = 1.5e0f64;
                let v57 = parameters[107];
                let v59 = parameters[52];
                let v60 = parameters[106];
                let v66 = 0e0f64;
                let v72 = parameters[105];
                let v73 = parameters[104];
                let v76 = parameters[22];
                let v81 = 7e-1f64;
                let v83 = parameters[32];
                let v85 = parameters[47];
                let v89 = parameters[86];
                let v91 = parameters[88];
                let v93 = parameters[87];
                let v96 = parameters[66];
                let v100 = parameters[115];
                let v101 = 1e-2f64;
                let v103 = parameters[116];
                let v110 = 1.7e8f64;
                let v111 = 1e9f64;
                let v127 = 6e0f64;
                let v139 = parameters[39];
                let v141 = 2e0f64;
                let v143 = parameters[40];
                let v147 = -5e-1f64;
                let v154 = parameters[42];
                let v158 = parameters[17];
                let v162 = parameters[48];
                let v166 = -5e-1f64;
                let v173 = parameters[50];
                let v178 = parameters[8];
                let v181 = 1e-5f64;
                let v184 = parameters[125];
                let v191 = parameters[79];
                let v195 = parameters[37];
                let v197 = parameters[43];
                let v200 = parameters[44];
                let v204 = -5e-1f64;
                let v211 = parameters[46];
                let v214 = parameters[21];
                let v216 = parameters[27];
                let v218 = 1.0f64;
                let v220 = parameters[53];
                let v224 = -5e-1f64;
                let v231 = parameters[55];
                let v234 = parameters[57];
                let v240 = parameters[63];
                let v243 = parameters[58];
                let v247 = -5e-1f64;
                let v254 = 2.4e0f64;
                let v259 = -5e-1f64;
                let v266 = parameters[60];
                let v271 = parameters[62];
                let v273 = parameters[141];
                let v275 = parameters[142];
                let v276 = parameters[149];
                let v285 = -5e-1f64;
                let v296 = parameters[14];
                let v302 = -5e-1f64;
                let v315 = -5e-1f64;
                let v328 = -5e-1f64;
                let v337 = 1.0f64;
                let v342 = -5e-1f64;
                let v358 = -5e-1f64;
                let v369 = -5e-1f64;
                let v385 = -5e-1f64;
                let v396 = parameters[16];
                let v398 = parameters[51];
                let v399 = 1e2f64;
                let v401 = parameters[10];
                let v403 = -8.754687373538999e-1f64;
                let v404 = parameters[49];
                let v409 = parameters[85];
                let v412 = 3.2e2f64;
                let v414 = parameters[18];
                let v416 = parameters[20];
                let v418 = parameters[56];
                let v420 = parameters[25];
                let v422 = parameters[61];
                let v424 = parameters[65];
                let v426 = parameters[97];
                let v428 = parameters[101];
                let v430 = parameters[99];
                let v436 = parameters[89];
                let v440 = parameters[93];
                let v442 = 0e0f64;
                let v444 = parameters[29];
                let v446 = parameters[148];
                let v450 = parameters[90];
                let v454 = 0e0f64;
                let v456 = parameters[95];
                let v460 = 0e0f64;
                let v462 = parameters[96];
                let v466 = 0e0f64;
                let v468 = parameters[102];
                let v472 = parameters[103];
                let v474 = 0e0f64;
                let v479 = parameters[145];
                let v481 = 0e0f64;
                let v483 = 0e0f64;
                let v485 = 0e0f64;
                let v487 = 0e0f64;
                let v489 = 0e0f64;
                let v491 = 0e0f64;
                let v493 = parameters[112];
                let v494 = -1e0f64;
                let v496 = 0e0f64;
                let v497 = 0e0f64;
                let v500 = 0e0f64;
                let v502 = 0e0f64;
                let v504 = parameters[109];
                let v510 = 0e0f64;
                let v511 = 0e0f64;
                let v519 = 0e0f64;
                let v520 = 0e0f64;
                let mut out98: f64 = 0.0;
                let mut out109: f64 = 0.0;
                let mut out122: f64 = 0.0;
                let mut out153: f64 = 0.0;
                let mut out155: f64 = 0.0;
                let mut out156: f64 = 0.0;
                let mut out172: f64 = 0.0;
                let mut out174: f64 = 0.0;
                let mut out175: f64 = 0.0;
                let mut out210: f64 = 0.0;
                let mut out212: f64 = 0.0;
                let mut out213: f64 = 0.0;
                let mut out230: f64 = 0.0;
                let mut out232: f64 = 0.0;
                let mut out233: f64 = 0.0;
                let mut out235: f64 = 0.0;
                let mut out236: f64 = 0.0;
                let mut out253: f64 = 0.0;
                let mut out265: f64 = 0.0;
                let mut out267: f64 = 0.0;
                let mut out268: f64 = 0.0;
                let mut out269: f64 = 0.0;
                let mut out270: f64 = 0.0;
                let mut out272: f64 = 0.0;
                let mut out291: f64 = 0.0;
                let mut out292: f64 = 0.0;
                let mut out293: f64 = 0.0;
                let mut out294: f64 = 0.0;
                let mut out295: f64 = 0.0;
                let mut out308: f64 = 0.0;
                let mut out309: f64 = 0.0;
                let mut out310: f64 = 0.0;
                let mut out321: f64 = 0.0;
                let mut out322: f64 = 0.0;
                let mut out323: f64 = 0.0;
                let mut out334: f64 = 0.0;
                let mut out335: f64 = 0.0;
                let mut out336: f64 = 0.0;
                let mut out348: f64 = 0.0;
                let mut out349: f64 = 0.0;
                let mut out350: f64 = 0.0;
                let mut out351: f64 = 0.0;
                let mut out352: f64 = 0.0;
                let mut out353: f64 = 0.0;
                let mut out364: f64 = 0.0;
                let mut out375: f64 = 0.0;
                let mut out376: f64 = 0.0;
                let mut out377: f64 = 0.0;
                let mut out378: f64 = 0.0;
                let mut out379: f64 = 0.0;
                let mut out380: f64 = 0.0;
                let mut out391: f64 = 0.0;
                let mut out392: f64 = 0.0;
                let mut out393: f64 = 0.0;
                let mut out394: f64 = 0.0;
                let mut out395: f64 = 0.0;
                let mut out425: f64 = 0.0;
                let mut out429: f64 = 0.0;
                let mut out433: f64 = 0.0;
                let mut out435: f64 = 0.0;
                let mut out441: f64 = 0.0;
                let mut out447: f64 = 0.0;
                let mut out448: f64 = 0.0;
                let mut out473: f64 = 0.0;
                let mut out480: f64 = 0.0;
                let mut out518: f64 = 0.0;
                let v2 = if v0 <= v1 { 1.0 } else { 0.0 };
                let v7: f64;
                let v8: f64;
                if v2 != 0.0 {
                    v7 = v3;
                    v8 = v4;
                } else {
                    v7 = v5;
                    v8 = v6;
                }
                let v11 = v9 + v10;
                let v12 = v7 / v8;
                let v14 = v12 * v13;
                let v15 = v12 * v11;
                let v17 = v16 / v15;
                let v21 = (v18 * v11) * (v11.ln());
                let v23 = v22 * v11;
                let v25 = v24 * v11;
                let v28 = (v26 + v21) + v23;
                let v37 = (v28 + ((v29 + v21) + v23)) * v36;
                let v39 = (v28 + ((v32 + v21) + v23)) * v36;
                let v41 = (v26 + v29) * v36;
                let v43 = (v26 + v32) * v36;
                let v46 = (v44 + v32) * v36;
                let v49 = v48 - (v18 / v12);
                let v50 = v49 + v16;
                let v52 = v50 - v51;
                let v54 = v50 - v53;
                let v56 = v49 - v55;
                let v62 = (v16 - v57) * (v59 + v60);
                let v63 = if v62 >= v60 { 1.0 } else { 0.0 };
                let v68: f64;
                let v69: f64;
                let v70: f64;
                let v71: f64;
                if v63 != 0.0 {
                    let v64 = v62 - v60;
                    let v65 = v59 - v64;
                    v68 = v64;
                    v69 = v65;
                    v70 = v66;
                    v71 = v60;
                } else {
                    let v67 = v60 - v62;
                    v68 = v66;
                    v69 = v59;
                    v70 = v67;
                    v71 = v62;
                }
                let v74 = v72 * v73;
                let v75 = v73 - v74;
                let v77 = if v76 != v66 { 1.0 } else { 0.0 };
                let v79: f64;
                if v77 != 0.0 {
                    let v78 = v16 / v76;
                    v79 = v78;
                } else {
                    v79 = v66;
                }
                let v80 = if v0 <= v13 { 1.0 } else { 0.0 };
                let v82: f64;
                if v80 != 0.0 {
                    v82 = v66;
                } else {
                    v82 = v81;
                }
                let v86 = if v85 > v66 { 1.0 } else { 0.0 };
                let v87 = if (if v83 > v66 { 1.0 } else { 0.0 }) != 0.0 && v86 != 0.0 { 1.0 } else { 0.0 };
                let v88: f64;
                if v87 != 0.0 {
                    v88 = v16;
                } else {
                    v88 = v66;
                }
                let v90 = if v89 != v66 { 1.0 } else { 0.0 };
                let v99: f64;
                if v90 != 0.0 {
                    let v98 = if (if (if v91 == v66 { 1.0 } else { 0.0 }) != 0.0 && (if v93 == v66 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v96 == v66 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    out98 = v98;
                    let v106: f64;
                    if v98 != 0.0 {
                        v106 = v66;
                    } else {
                        v106 = v89;
                    }
                    v99 = v106;
                } else {
                    v99 = v89;
                }
                let v105 = if (if v100 >= v101 { 1.0 } else { 0.0 }) != 0.0 || (if v103 >= v101 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v112: f64;
                let v113: f64;
                let v114: f64;
                let v115: f64;
                let v116: f64;
                let v117: f64;
                let v118: f64;
                let v119: f64;
                if v105 != 0.0 {
                    let v108 = v36 * (v100 - v103);
                    let v109 = if v103 < v100 { 1.0 } else { 0.0 };
                    out109 = v109;
                    let v120: f64;
                    let v121: f64;
                    if v109 != 0.0 {
                        v120 = v103;
                        v121 = v100;
                    } else {
                        v120 = v100;
                        v121 = v103;
                    }
                    let v122 = if v120 < v101 { 1.0 } else { 0.0 };
                    out122 = v122;
                    let v134: f64;
                    let v135: f64;
                    let v136: f64;
                    let v137: f64;
                    let v138: f64;
                    if v122 != 0.0 {
                        let v124 = (v16 + v121).ln();
                        v134 = v124;
                        v135 = v110;
                        v136 = v111;
                        v137 = v110;
                        v138 = v111;
                    } else {
                        let v125 = v16 / v100;
                        let v126 = v16 / v103;
                        let v128 = v100 / v127;
                        let v129 = v103 / v127;
                        let v133 = ((v16 + v100) / (v16 + v103)).ln();
                        v134 = v133;
                        v135 = v128;
                        v136 = v126;
                        v137 = v129;
                        v138 = v125;
                    }
                    v112 = v108;
                    v113 = v134;
                    v114 = v120;
                    v115 = v121;
                    v116 = v135;
                    v117 = v136;
                    v118 = v137;
                    v119 = v138;
                } else {
                    v112 = v66;
                    v113 = v66;
                    v114 = v103;
                    v115 = v100;
                    v116 = v110;
                    v117 = v111;
                    v118 = v110;
                    v119 = v111;
                }
                let v140 = if v139 > v66 { 1.0 } else { 0.0 };
                if v140 != 0.0 {
                    let v153 = (v141 * v15) * (((((v143 * v36) * v17).exp()) - (((v147 * v143) * v17).exp())).ln());
                    out153 = v153;
                    let v155 = v154.abs();
                    out155 = v155;
                    let v156 = if v154 > v66 { 1.0 } else { 0.0 };
                    out156 = v156;
                } else {
                }
                let v157 = v29 * v17;
                let v159 = v49 / v158;
                let v160 = v41 * v17;
                if v86 != 0.0 {
                    let v172 = (v141 * v15) * (((((v162 * v36) * v17).exp()) - (((v166 * v162) * v17).exp())).ln());
                    out172 = v172;
                    let v174 = v173.abs();
                    out174 = v174;
                    let v175 = if v173 > v66 { 1.0 } else { 0.0 };
                    out175 = v175;
                } else {
                }
                let v176 = v32 * v17;
                let v177 = v26 * v17;
                let v183 = if v80 != 0.0 && (if ((v178 - v16).abs()) < v181 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v185 = v184 * v17;
                let v187 = (v26 - v29) * v17;
                let v189 = (v26 - v32) * v17;
                let v190 = v51 - v25;
                let v192 = if v191 > v66 { 1.0 } else { 0.0 };
                let v193 = v51 - v16;
                let v194 = if v88 == v16 { 1.0 } else { 0.0 };
                let v196 = if v195 > v66 { 1.0 } else { 0.0 };
                let v198 = if v197 > v66 { 1.0 } else { 0.0 };
                if v198 != 0.0 {
                    let v210 = (v141 * v15) * (((((v200 * v36) * v17).exp()) - (((v204 * v200) * v17).exp())).ln());
                    out210 = v210;
                    let v212 = v211.abs();
                    out212 = v212;
                    let v213 = if v211 > v66 { 1.0 } else { 0.0 };
                    out213 = v213;
                } else {
                }
                let v215 = v49 / v214;
                let v217 = if v216 > v66 { 1.0 } else { 0.0 };
                if v218 != 0.0 {
                    let v230 = (v141 * v15) * (((((v220 * v36) * v17).exp()) - (((v224 * v220) * v17).exp())).ln());
                    out230 = v230;
                    let v232 = v231.abs();
                    out232 = v232;
                    let v233 = if v231 > v66 { 1.0 } else { 0.0 };
                    out233 = v233;
                } else {
                }
                let v237: f64;
                if v80 != 0.0 {
                    let v235 = if v234 > v66 { 1.0 } else { 0.0 };
                    out235 = v235;
                    if v235 != 0.0 {
                        let v253 = (v141 * v15) * (((((v243 * v36) * v17).exp()) - (((v247 * v243) * v17).exp())).ln());
                        out253 = v253;
                    } else {
                    }
                    v237 = v254;
                } else {
                    let v236 = if v234 > v66 { 1.0 } else { 0.0 };
                    out236 = v236;
                    if v236 != 0.0 {
                        let v265 = (v141 * v15) * (((((v243 * v36) * v17).exp()) - (((v259 * v243) * v17).exp())).ln());
                        out265 = v265;
                        let v267 = -v266;
                        out267 = v267;
                        let v268 = v267.abs();
                        out268 = v268;
                        let v269 = if v267 > v66 { 1.0 } else { 0.0 };
                        out269 = v269;
                    } else {
                        let v270 = -v266;
                        out270 = v270;
                    }
                    v237 = v266;
                }
                let v238 = v44 * v17;
                let v239 = v53 - v16;
                let v241 = if v240 > v66 { 1.0 } else { 0.0 };
                if v241 != 0.0 {
                    let v272 = if v271 > v66 { 1.0 } else { 0.0 };
                    out272 = v272;
                    if v272 != 0.0 {
                        let v291 = (v141 * v15) * (((((v240 * v36) * v17).exp()) - (((v285 * v240) * v17).exp())).ln());
                        out291 = v291;
                        let v292 = -v237;
                        out292 = v292;
                        let v293 = v292.abs();
                        out293 = v293;
                        let v294 = if v292 > v66 { 1.0 } else { 0.0 };
                        out294 = v294;
                    } else {
                        let v295 = -v237;
                        out295 = v295;
                    }
                } else {
                }
                let v277 = if v275 >= v276 { 1.0 } else { 0.0 };
                let v279 = if v275 > v66 { 1.0 } else { 0.0 };
                let v280 = if (if (if v273 != v66 { 1.0 } else { 0.0 }) != 0.0 && v277 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v279 != 0.0 { 1.0 } else { 0.0 };
                if v280 != 0.0 {
                    if v140 != 0.0 {
                        let v308 = (v141 * v15) * (((((v143 * v36) * v17).exp()) - (((v302 * v143) * v17).exp())).ln());
                        out308 = v308;
                        let v309 = v154.abs();
                        out309 = v309;
                        let v310 = if v154 > v66 { 1.0 } else { 0.0 };
                        out310 = v310;
                    } else {
                    }
                    if v86 != 0.0 {
                        let v321 = (v141 * v15) * (((((v162 * v36) * v17).exp()) - (((v315 * v162) * v17).exp())).ln());
                        out321 = v321;
                        let v322 = v173.abs();
                        out322 = v322;
                        let v323 = if v173 > v66 { 1.0 } else { 0.0 };
                        out323 = v323;
                    } else {
                    }
                    if v198 != 0.0 {
                        let v334 = (v141 * v15) * (((((v200 * v36) * v17).exp()) - (((v328 * v200) * v17).exp())).ln());
                        out334 = v334;
                        let v335 = v211.abs();
                        out335 = v335;
                        let v336 = if v211 > v66 { 1.0 } else { 0.0 };
                        out336 = v336;
                    } else {
                    }
                    if v337 != 0.0 {
                        let v348 = (v141 * v15) * (((((v220 * v36) * v17).exp()) - (((v342 * v220) * v17).exp())).ln());
                        out348 = v348;
                        let v349 = v231.abs();
                        out349 = v349;
                        let v350 = if v231 > v66 { 1.0 } else { 0.0 };
                        out350 = v350;
                    } else {
                    }
                    let v353: f64;
                    if v80 != 0.0 {
                        let v351 = if v234 > v66 { 1.0 } else { 0.0 };
                        out351 = v351;
                        if v351 != 0.0 {
                            let v364 = (v141 * v15) * (((((v243 * v36) * v17).exp()) - (((v358 * v243) * v17).exp())).ln());
                            out364 = v364;
                        } else {
                        }
                        v353 = v254;
                    } else {
                        let v352 = if v234 > v66 { 1.0 } else { 0.0 };
                        out352 = v352;
                        if v352 != 0.0 {
                            let v375 = (v141 * v15) * (((((v243 * v36) * v17).exp()) - (((v369 * v243) * v17).exp())).ln());
                            out375 = v375;
                            let v376 = -v266;
                            out376 = v376;
                            let v377 = v376.abs();
                            out377 = v377;
                            let v378 = if v376 > v66 { 1.0 } else { 0.0 };
                            out378 = v378;
                        } else {
                            let v379 = -v266;
                            out379 = v379;
                        }
                        v353 = v266;
                    }
                    out353 = v353;
                    if v241 != 0.0 {
                        let v380 = if v271 > v66 { 1.0 } else { 0.0 };
                        out380 = v380;
                        if v380 != 0.0 {
                            let v391 = (v141 * v15) * (((((v240 * v36) * v17).exp()) - (((v385 * v240) * v17).exp())).ln());
                            out391 = v391;
                            let v392 = -v353;
                            out392 = v392;
                            let v393 = v392.abs();
                            out393 = v393;
                            let v394 = if v392 > v66 { 1.0 } else { 0.0 };
                            out394 = v394;
                        } else {
                            let v395 = -v353;
                            out395 = v395;
                        }
                    } else {
                    }
                } else {
                }
                let v297 = if v296 > v66 { 1.0 } else { 0.0 };
                let v397 = if v396 > v66 { 1.0 } else { 0.0 };
                let v400 = if v398 < v399 { 1.0 } else { 0.0 };
                let v402 = if v401 > v66 { 1.0 } else { 0.0 };
                let v407 = v16 - ((v403 / v404).exp());
                let v408 = -v404;
                let v410 = if v409 > v66 { 1.0 } else { 0.0 };
                let v411 = if v0 >= v1 { 1.0 } else { 0.0 };
                let v413 = if v0 >= v412 { 1.0 } else { 0.0 };
                let v415 = if v414 > v66 { 1.0 } else { 0.0 };
                let v417 = if v416 > v66 { 1.0 } else { 0.0 };
                let v419 = if v418 < v399 { 1.0 } else { 0.0 };
                let v421 = if v420 > v66 { 1.0 } else { 0.0 };
                let v423 = if v422 < v399 { 1.0 } else { 0.0 };
                if v241 != 0.0 {
                    let v425 = if v424 < v399 { 1.0 } else { 0.0 };
                    out425 = v425;
                } else {
                }
                let v427 = if v426 > v66 { 1.0 } else { 0.0 };
                if v427 != 0.0 {
                    let v429 = if v428 > v66 { 1.0 } else { 0.0 };
                    out429 = v429;
                } else {
                }
                let v431 = if v430 > v66 { 1.0 } else { 0.0 };
                let v432 = if v277 != 0.0 && v279 != 0.0 { 1.0 } else { 0.0 };
                if v432 != 0.0 {
                    let v433 = if v273 == v16 { 1.0 } else { 0.0 };
                    out433 = v433;
                    if v433 != 0.0 {
                    } else {
                        let v435 = if v273 == v141 { 1.0 } else { 0.0 };
                        out435 = v435;
                    }
                } else {
                }
                let v434 = if v99 != v66 { 1.0 } else { 0.0 };
                let v439 = if (if v436 >= v276 { 1.0 } else { 0.0 }) != 0.0 && (if v436 > v66 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v443: f64;
                if v439 != 0.0 {
                    let v441 = if v440 > v66 { 1.0 } else { 0.0 };
                    out441 = v441;
                    v443 = v66;
                } else {
                    v443 = v442;
                }
                let v445 = if v444 == v16 { 1.0 } else { 0.0 };
                if v445 != 0.0 {
                    let v447 = -v446;
                    out447 = v447;
                } else {
                    let v448 = -v446;
                    out448 = v448;
                }
                let v449 = -v446;
                let v453 = if (if v450 >= v276 { 1.0 } else { 0.0 }) != 0.0 && (if v450 > v66 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v455: f64;
                if v453 != 0.0 {
                    v455 = v66;
                } else {
                    v455 = v454;
                }
                let v459 = if (if v456 >= v276 { 1.0 } else { 0.0 }) != 0.0 && (if v456 > v66 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v461: f64;
                if v459 != 0.0 {
                    v461 = v66;
                } else {
                    v461 = v460;
                }
                let v465 = if (if v462 >= v276 { 1.0 } else { 0.0 }) != 0.0 && (if v462 > v66 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v467: f64;
                if v465 != 0.0 {
                    v467 = v66;
                } else {
                    v467 = v466;
                }
                let v471 = if (if v468 >= v276 { 1.0 } else { 0.0 }) != 0.0 && (if v468 > v66 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v475: f64;
                if v471 != 0.0 {
                    let v473 = if v472 > v66 { 1.0 } else { 0.0 };
                    out473 = v473;
                    v475 = v66;
                } else {
                    v475 = v474;
                }
                let v478 = if (if (if v273 >= v16 { 1.0 } else { 0.0 }) != 0.0 && v277 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v279 != 0.0 { 1.0 } else { 0.0 };
                let v482: f64;
                if v478 != 0.0 {
                    let v480 = if v479 > v66 { 1.0 } else { 0.0 };
                    out480 = v480;
                    v482 = v66;
                } else {
                    v482 = v481;
                }
                let v484: f64;
                if v453 != 0.0 {
                    v484 = v483;
                } else {
                    v484 = v66;
                }
                let v486: f64;
                if v439 != 0.0 {
                    v486 = v485;
                } else {
                    v486 = v66;
                }
                let v488: f64;
                if v465 != 0.0 {
                    v488 = v487;
                } else {
                    v488 = v66;
                }
                let v490: f64;
                if v459 != 0.0 {
                    v490 = v489;
                } else {
                    v490 = v66;
                }
                let v492: f64;
                if v471 != 0.0 {
                    v492 = v491;
                } else {
                    v492 = v66;
                }
                let v495 = if v493 == v494 { 1.0 } else { 0.0 };
                let v498: f64;
                let v499: f64;
                if v495 != 0.0 {
                    v498 = v496;
                    v499 = v66;
                } else {
                    v498 = v66;
                    v499 = v497;
                }
                let v501: f64;
                if v459 != 0.0 {
                    v501 = v500;
                } else {
                    v501 = v66;
                }
                let v503: f64;
                if v413 != 0.0 {
                    v503 = v502;
                } else {
                    v503 = v66;
                }
                let v509 = if (if v504 == v16 { 1.0 } else { 0.0 }) != 0.0 && (if (if v91 > v66 { 1.0 } else { 0.0 }) != 0.0 && (if v93 > v66 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v512: f64;
                let v513: f64;
                let v514: f64;
                let v515: f64;
                if v509 != 0.0 {
                    let v518 = (v141 * v93) - (v91 * v91);
                    out518 = v518;
                    v512 = v519;
                    v513 = v520;
                    v514 = v66;
                    v515 = v66;
                } else {
                    v512 = v66;
                    v513 = v66;
                    v514 = v510;
                    v515 = v511;
                }
            [v2, v11, v12, v14, v37, v39, v41, v43, v46, v49, v52, v54, v56, v63, v74, v75, v77, v80, v86, v87, v90, out98, v105, out109, out122, v140, out153, out155, out156, v157, v159, v160, out172, out174, out175, v176, v177, v183, v185, v187, v189, v190, v192, v193, v194, v196, v198, out210, out212, out213, v215, v217, v82, out230, out232, out233, v68, v69, out235, out253, out236, out265, out267, out268, out269, out270, v238, v239, v241, out272, out291, v237, out292, out293, out294, out295, v280, out308, out309, out310, out321, out322, out323, out334, out335, out336, out348, out349, out350, out351, out364, out352, out375, out376, out377, out378, out379, out380, out391, out353, out392, out393, out394, out395, v297, v397, v400, v402, v407, v408, v410, v411, v413, v112, v113, v114, v115, v116, v117, v118, v119, v79, v415, v417, v419, v421, v423, out425, v427, out429, v431, v432, out433, out435, v434, v439, out441, v445, out447, out448, v449, v70, v71, v453, v459, v465, v471, out473, v478, out480, v495, v509, out518, v484, v488, v492, v443, v455, v461, v467, v475, v482, v486, v490, v498, v499, v501, v503, v512, v513, v514, v515]
        };
        self.canonical_staged[84] = produced[0];
        self.canonical_staged[1] = produced[1];
        self.canonical_staged[0] = produced[2];
        self.canonical_staged[61] = produced[3];
        self.canonical_staged[28] = produced[4];
        self.canonical_staged[20] = produced[5];
        self.canonical_staged[3] = produced[6];
        self.canonical_staged[9] = produced[7];
        self.canonical_staged[41] = produced[8];
        self.canonical_staged[4] = produced[9];
        self.canonical_staged[10] = produced[10];
        self.canonical_staged[39] = produced[11];
        self.canonical_staged[44] = produced[12];
        self.canonical_staged[85] = produced[13];
        self.canonical_staged[80] = produced[14];
        self.canonical_staged[81] = produced[15];
        self.canonical_staged[86] = produced[16];
        self.canonical_staged[73] = produced[17];
        self.canonical_staged[19] = produced[18];
        self.canonical_staged[87] = produced[19];
        self.canonical_staged[88] = produced[20];
        self.canonical_staged[89] = produced[21];
        self.canonical_staged[90] = produced[22];
        self.canonical_staged[91] = produced[23];
        self.canonical_staged[93] = produced[24];
        self.canonical_staged[33] = produced[25];
        self.canonical_staged[2] = produced[26];
        self.canonical_staged[96] = produced[27];
        self.canonical_staged[95] = produced[28];
        self.canonical_staged[5] = produced[29];
        self.canonical_staged[6] = produced[30];
        self.canonical_staged[7] = produced[31];
        self.canonical_staged[8] = produced[32];
        self.canonical_staged[98] = produced[33];
        self.canonical_staged[97] = produced[34];
        self.canonical_staged[11] = produced[35];
        self.canonical_staged[12] = produced[36];
        self.canonical_staged[99] = produced[37];
        self.canonical_staged[13] = produced[38];
        self.canonical_staged[14] = produced[39];
        self.canonical_staged[15] = produced[40];
        self.canonical_staged[16] = produced[41];
        self.canonical_staged[100] = produced[42];
        self.canonical_staged[17] = produced[43];
        self.canonical_staged[101] = produced[44];
        self.canonical_staged[18] = produced[45];
        self.canonical_staged[30] = produced[46];
        self.canonical_staged[24] = produced[47];
        self.canonical_staged[103] = produced[48];
        self.canonical_staged[102] = produced[49];
        self.canonical_staged[25] = produced[50];
        self.canonical_staged[27] = produced[51];
        self.canonical_staged[26] = produced[52];
        self.canonical_staged[36] = produced[53];
        self.canonical_staged[105] = produced[54];
        self.canonical_staged[104] = produced[55];
        self.canonical_staged[37] = produced[56];
        self.canonical_staged[38] = produced[57];
        self.canonical_staged[106] = produced[58];
        self.canonical_staged[40] = produced[59];
        self.canonical_staged[107] = produced[60];
        self.canonical_staged[42] = produced[61];
        self.canonical_staged[43] = produced[62];
        self.canonical_staged[111] = produced[63];
        self.canonical_staged[109] = produced[64];
        self.canonical_staged[110] = produced[65];
        self.canonical_staged[45] = produced[66];
        self.canonical_staged[46] = produced[67];
        self.canonical_staged[108] = produced[68];
        self.canonical_staged[112] = produced[69];
        self.canonical_staged[47] = produced[70];
        self.canonical_staged[113] = produced[71];
        self.canonical_staged[48] = produced[72];
        self.canonical_staged[117] = produced[73];
        self.canonical_staged[115] = produced[74];
        self.canonical_staged[116] = produced[75];
        self.canonical_staged[114] = produced[76];
        self.canonical_staged[50] = produced[77];
        self.canonical_staged[166] = produced[78];
        self.canonical_staged[165] = produced[79];
        self.canonical_staged[51] = produced[80];
        self.canonical_staged[168] = produced[81];
        self.canonical_staged[167] = produced[82];
        self.canonical_staged[52] = produced[83];
        self.canonical_staged[170] = produced[84];
        self.canonical_staged[169] = produced[85];
        self.canonical_staged[53] = produced[86];
        self.canonical_staged[172] = produced[87];
        self.canonical_staged[171] = produced[88];
        self.canonical_staged[173] = produced[89];
        self.canonical_staged[54] = produced[90];
        self.canonical_staged[174] = produced[91];
        self.canonical_staged[55] = produced[92];
        self.canonical_staged[56] = produced[93];
        self.canonical_staged[177] = produced[94];
        self.canonical_staged[175] = produced[95];
        self.canonical_staged[176] = produced[96];
        self.canonical_staged[178] = produced[97];
        self.canonical_staged[57] = produced[98];
        self.canonical_staged[179] = produced[99];
        self.canonical_staged[58] = produced[100];
        self.canonical_staged[182] = produced[101];
        self.canonical_staged[180] = produced[102];
        self.canonical_staged[181] = produced[103];
        self.canonical_staged[164] = produced[104];
        self.canonical_staged[183] = produced[105];
        self.canonical_staged[184] = produced[106];
        self.canonical_staged[185] = produced[107];
        self.canonical_staged[59] = produced[108];
        self.canonical_staged[60] = produced[109];
        self.canonical_staged[62] = produced[110];
        self.canonical_staged[72] = produced[111];
        self.canonical_staged[63] = produced[112];
        self.canonical_staged[64] = produced[113];
        self.canonical_staged[65] = produced[114];
        self.canonical_staged[66] = produced[115];
        self.canonical_staged[67] = produced[116];
        self.canonical_staged[68] = produced[117];
        self.canonical_staged[69] = produced[118];
        self.canonical_staged[70] = produced[119];
        self.canonical_staged[71] = produced[120];
        self.canonical_staged[74] = produced[121];
        self.canonical_staged[186] = produced[122];
        self.canonical_staged[187] = produced[123];
        self.canonical_staged[188] = produced[124];
        self.canonical_staged[189] = produced[125];
        self.canonical_staged[190] = produced[126];
        self.canonical_staged[191] = produced[127];
        self.canonical_staged[192] = produced[128];
        self.canonical_staged[193] = produced[129];
        self.canonical_staged[194] = produced[130];
        self.canonical_staged[195] = produced[131];
        self.canonical_staged[196] = produced[132];
        self.canonical_staged[198] = produced[133];
        self.canonical_staged[197] = produced[134];
        self.canonical_staged[199] = produced[135];
        self.canonical_staged[200] = produced[136];
        self.canonical_staged[201] = produced[137];
        self.canonical_staged[75] = produced[138];
        self.canonical_staged[76] = produced[139];
        self.canonical_staged[77] = produced[140];
        self.canonical_staged[78] = produced[141];
        self.canonical_staged[79] = produced[142];
        self.canonical_staged[202] = produced[143];
        self.canonical_staged[203] = produced[144];
        self.canonical_staged[204] = produced[145];
        self.canonical_staged[205] = produced[146];
        self.canonical_staged[206] = produced[147];
        self.canonical_staged[207] = produced[148];
        self.canonical_staged[208] = produced[149];
        self.canonical_staged[209] = produced[150];
        self.canonical_staged[210] = produced[151];
        self.canonical_staged[82] = produced[152];
        self.canonical_staged[83] = produced[153];
        self.canonical_staged[218] = produced[154];
        self.canonical_staged[220] = produced[155];
        self.canonical_staged[211] = produced[156];
        self.canonical_staged[212] = produced[157];
        self.canonical_staged[213] = produced[158];
        self.canonical_staged[214] = produced[159];
        self.canonical_staged[215] = produced[160];
        self.canonical_staged[216] = produced[161];
        self.canonical_staged[217] = produced[162];
        self.canonical_staged[219] = produced[163];
        self.canonical_staged[221] = produced[164];
        self.canonical_staged[222] = produced[165];
        self.canonical_staged[223] = produced[166];
        self.canonical_staged[224] = produced[167];
        self.canonical_staged[225] = produced[168];
        self.canonical_staged[226] = produced[169];
        self.canonical_staged[227] = produced[170];
        self.canonical_staged[228] = produced[171];
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
        let produced: [f64; 57] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let temperature = ctx.temperature();
            let staged = &*self.canonical_staged;
                let v0 = staged[73];
                let v1 = temperature;
                let v2 = parameters[147];
                let v4 = 7.314999999999998e1f64;
                let v6 = 7.314999999999998e1f64;
                let v7 = 6e2f64;
                let v10 = staged[0];
                let v12 = 1e0f64;
                let v14 = staged[1];
                let v19 = parameters[121];
                let v23 = parameters[122];
                let v25 = parameters[117];
                let v28 = parameters[118];
                let v31 = parameters[119];
                let v35 = 5e-1f64;
                let v39 = staged[33];
                let v40 = 6e2f64;
                let v42 = staged[2];
                let v45 = staged[3];
                let v48 = staged[4];
                let v52 = 2e0f64;
                let v57 = 4e0f64;
                let v66 = parameters[40];
                let v69 = parameters[41];
                let v72 = parameters[39];
                let v74 = staged[95];
                let v75 = parameters[42];
                let v79 = parameters[124];
                let v82 = staged[5];
                let v86 = parameters[14];
                let v88 = staged[6];
                let v90 = staged[7];
                let v92 = parameters[17];
                let v96 = parameters[16];
                let v98 = staged[19];
                let v101 = staged[96];
                let v103 = staged[8];
                let v106 = staged[9];
                let v124 = parameters[48];
                let v127 = parameters[49];
                let v130 = parameters[47];
                let v132 = staged[97];
                let v133 = parameters[50];
                let v139 = staged[98];
                let v141 = 2.4e0f64;
                let v143 = staged[10];
                let v145 = staged[11];
                let v149 = parameters[23];
                let v156 = parameters[2];
                let v158 = parameters[123];
                let v160 = staged[12];
                let v164 = parameters[1];
                let v166 = parameters[126];
                let v169 = parameters[10];
                let v171 = staged[99];
                let v172 = parameters[125];
                let v174 = parameters[127];
                let v180 = parameters[9];
                let v188 = parameters[8];
                let v191 = staged[13];
                let v194 = parameters[3];
                let v196 = staged[14];
                let v199 = parameters[4];
                let v201 = staged[15];
                let v204 = parameters[6];
                let v206 = staged[16];
                let v209 = parameters[75];
                let v211 = parameters[130];
                let v214 = parameters[74];
                let v217 = staged[100];
                let v218 = parameters[133];
                let v221 = parameters[79];
                let v223 = parameters[78];
                let v224 = parameters[132];
                let v230 = parameters[128];
                let v233 = parameters[129];
                let v237 = parameters[66];
                let v239 = staged[17];
                let v242 = parameters[71];
                let v244 = staged[101];
                let v245 = parameters[139];
                let v248 = parameters[32];
                let v250 = parameters[140];
                let v253 = parameters[33];
                let v257 = parameters[134];
                let v260 = parameters[89];
                let v262 = staged[30];
                let v263 = staged[24];
                let v283 = parameters[44];
                let v286 = parameters[45];
                let v289 = parameters[43];
                let v291 = staged[102];
                let v292 = parameters[46];
                let v296 = parameters[18];
                let v298 = staged[25];
                let v300 = parameters[21];
                let v304 = parameters[20];
                let v308 = staged[103];
                let v312 = parameters[31];
                let v315 = parameters[30];
                let v317 = 1.0f64;
                let v318 = staged[36];
                let v338 = parameters[53];
                let v341 = parameters[54];
                let v344 = staged[104];
                let v345 = parameters[55];
                let v351 = staged[105];
                let v354 = staged[37];
                let v356 = staged[38];
                let v358 = staged[39];
                let v362 = parameters[25];
                let v364 = staged[106];
                let v365 = staged[107];
                let v369 = staged[44];
                let v371 = staged[45];
                let v375 = parameters[99];
                let v379 = parameters[97];
                let v381 = staged[46];
                let v384 = parameters[101];
                let v386 = staged[108];
                let v387 = staged[40];
                let v390 = staged[41];
                let v408 = parameters[58];
                let v411 = parameters[59];
                let v414 = parameters[57];
                let v416 = 0.0f64;
                let v417 = -2.4e0f64;
                let v421 = -2.4e0f64;
                let v424 = 2.4e0f64;
                let v426 = staged[42];
                let v451 = staged[109];
                let v452 = staged[110];
                let v456 = staged[43];
                let v459 = staged[111];
                let v461 = staged[112];
                let v462 = parameters[62];
                let v463 = parameters[63];
                let v464 = staged[113];
                let v468 = parameters[136];
                let v471 = parameters[96];
                let v473 = parameters[135];
                let v476 = parameters[90];
                let v478 = parameters[137];
                let v481 = parameters[95];
                let v483 = parameters[143];
                let v486 = parameters[142];
                let v488 = parameters[144];
                let v492 = staged[47];
                let v514 = parameters[64];
                let v518 = staged[115];
                let v519 = staged[116];
                let v523 = staged[48];
                let v526 = staged[117];
                let mut out8: f64 = 0.0;
                let v3 = v1 + v2;
                let v5 = if v3 < v4 { 1.0 } else { 0.0 };
                let v9: f64;
                if v5 != 0.0 {
                    v9 = v6;
                } else {
                    let v8 = if v3 > v7 { 1.0 } else { 0.0 };
                    out8 = v8;
                    let v41: f64;
                    if v8 != 0.0 {
                        v41 = v40;
                    } else {
                        v41 = v3;
                    }
                    v9 = v41;
                }
                let v11 = v10 * v9;
                let v13 = v12 / v11;
                let v15 = v9 - v14;
                let v16 = v14 / v9;
                let v17 = v9 / v14;
                let v18 = v17.ln();
                let v22 = (v19 * v9) * (v9.ln());
                let v24 = v23 * v9;
                let v27 = (v25 + v22) + v24;
                let v36 = (v27 + ((v28 + v22) + v24)) * v35;
                let v38 = (v27 + ((v31 + v22) + v24)) * v35;
                let v76: f64;
                let v77: f64;
                let v78: f64;
                if v39 != 0.0 {
                    let v51 = ((v42 * v17) + (v45 * (v12 - v17))) - ((v48 * v11) * v18);
                    let v65 = v51 + ((v52 * v11) * ((v35 * (v12 + ((v12 + (v57 * (((-v51) * v13).exp()))).sqrt()))).ln()));
                    let v73 = v72 * ((v69 * ((v66 / v65).ln())).exp());
                    let v102: f64;
                    if v74 != 0.0 {
                        let v100 = (v75 * v65) / v66;
                        v102 = v100;
                    } else {
                        v102 = v101;
                    }
                    v76 = v65;
                    v77 = v73;
                    v78 = v102;
                } else {
                    v76 = v66;
                    v77 = v72;
                    v78 = v75;
                }
                let v81 = v12 - v16;
                let v85 = ((v79 * v18) + (v82 * v81)).exp();
                let v87 = v86 * v85;
                let v91 = v90 * v81;
                let v97 = v96 * (((v88 * v18) + (v91 / v92)).exp());
                let v134: f64;
                let v135: f64;
                let v136: f64;
                if v98 != 0.0 {
                    let v111 = ((v103 * v17) + (v106 * (v12 - v17))) - ((v48 * v11) * v18);
                    let v123 = v111 + ((v52 * v11) * ((v35 * (v12 + ((v12 + (v57 * (((-v111) * v13).exp()))).sqrt()))).ln()));
                    let v131 = v130 * ((v127 * ((v124 / v123).ln())).exp());
                    let v140: f64;
                    if v132 != 0.0 {
                        let v138 = (v133 * v123) / v124;
                        v140 = v138;
                    } else {
                        v140 = v139;
                    }
                    v134 = v123;
                    v135 = v131;
                    v136 = v140;
                } else {
                    v134 = v124;
                    v135 = v130;
                    v136 = v133;
                }
                let v142: f64;
                if v0 != 0.0 {
                    v142 = v141;
                } else {
                    v142 = v136;
                }
                let v146 = v145 * v81;
                let v150 = v149 * (((v143 * v18) + v146).exp());
                let v151 = v76 / v66;
                let v157 = v156 * (v52 - ((v69 * (v151.ln())).exp()));
                let v165 = v164 * (((v158 * v18) + (v160 * v81)).exp());
                let v170 = v169 * ((v166 * v18).exp());
                let v190: f64;
                if v171 != 0.0 {
                    let v181 = v180 * (((v172 * v13) * (((v174 * v18).exp()) - v12)).exp());
                    v190 = v181;
                } else {
                    let v189 = v188 * (((v172 * v13) * (((v174 * v18).exp()) - v12)).exp());
                    v190 = v189;
                }
                let v195 = v194 * ((v191 * v81).exp());
                let v200 = v199 * ((v196 * v81).exp());
                let v205 = v204 * ((v201 * v81).exp());
                let v210 = v209 * ((v206 * v18).exp());
                let v216 = v12 / (v214 * ((v211 * v18).exp()));
                let v228: f64;
                let v229: f64;
                if v217 != 0.0 {
                    let v222 = v221 * (v12 - (v218 * v15));
                    v228 = v222;
                    v229 = v223;
                } else {
                    let v227 = v223 * (v12 + (v224 * v15));
                    v228 = v221;
                    v229 = v227;
                }
                let v238 = v237 * ((v12 + (v230 * v15)) + ((v233 * v15) * v15));
                let v243 = v242 * ((v239 * v18).exp());
                let v255: f64;
                let v256: f64;
                if v244 != 0.0 {
                    let v249 = v248 * ((v245 * v15).exp());
                    let v254 = v253 * ((v250 * v15).exp());
                    v255 = v254;
                    v256 = v249;
                } else {
                    v255 = v253;
                    v256 = v248;
                }
                let v261 = v260 * ((v257 * v18).exp());
                let v293: f64;
                let v294: f64;
                let v295: f64;
                if v262 != 0.0 {
                    let v270 = ((v263 * v17) + (v45 * (v12 - v17))) - ((v48 * v11) * v18);
                    let v282 = v270 + ((v52 * v11) * ((v35 * (v12 + ((v12 + (v57 * (((-v270) * v13).exp()))).sqrt()))).ln()));
                    let v290 = v289 * ((v286 * ((v283 / v282).ln())).exp());
                    let v309: f64;
                    if v291 != 0.0 {
                        let v307 = (v292 * v282) / v283;
                        v309 = v307;
                    } else {
                        v309 = v308;
                    }
                    v293 = v282;
                    v294 = v290;
                    v295 = v309;
                } else {
                    v293 = v283;
                    v294 = v289;
                    v295 = v292;
                }
                let v297 = v296 * v85;
                let v305 = v304 * (((v298 * v18) + (v91 / v300)).exp());
                let v316 = v315 * (((-(v76 - v66)) / v312).exp());
                let v346: f64;
                let v347: f64;
                let v348: f64;
                if v317 != 0.0 {
                    let v325 = ((v318 * v17) + (v106 * (v12 - v17))) - ((v48 * v11) * v18);
                    let v337 = v325 + ((v52 * v11) * ((v35 * (v12 + ((v12 + (v57 * (((-v325) * v13).exp()))).sqrt()))).ln()));
                    let v343 = (v341 * ((v338 / v337).ln())).exp();
                    let v352: f64;
                    if v344 != 0.0 {
                        let v350 = (v345 * v337) / v338;
                        v352 = v350;
                    } else {
                        v352 = v351;
                    }
                    v346 = v343;
                    v347 = v337;
                    v348 = v352;
                } else {
                    v346 = v12;
                    v347 = v338;
                    v348 = v345;
                }
                let v353: f64;
                if v0 != 0.0 {
                    v353 = v141;
                } else {
                    v353 = v348;
                }
                let v355 = v346 * v354;
                let v357 = v346 * v356;
                let v363 = v362 * (((v358 * v18) + v146).exp());
                let v366: f64;
                let v367: f64;
                let v368: f64;
                if v0 != 0.0 {
                    let v418: f64;
                    let v419: f64;
                    let v420: f64;
                    if v364 != 0.0 {
                        let v395 = ((v387 * v17) + (v390 * (v12 - v17))) - ((v48 * v11) * v18);
                        let v407 = v395 + ((v52 * v11) * ((v35 * (v12 + ((v12 + (v57 * (((-v395) * v13).exp()))).sqrt()))).ln()));
                        let v415 = v414 * ((v411 * ((v408 / v407).ln())).exp());
                        let v425: f64;
                        if v416 != 0.0 {
                            let v423 = (v421 * v407) / v408;
                            v425 = v423;
                        } else {
                            v425 = v424;
                        }
                        v418 = v415;
                        v419 = v407;
                        v420 = v425;
                    } else {
                        v418 = v414;
                        v419 = v408;
                        v420 = v417;
                    }
                    v366 = v418;
                    v367 = v419;
                    v368 = v420;
                } else {
                    let v453: f64;
                    let v454: f64;
                    let v455: f64;
                    if v365 != 0.0 {
                        let v433 = ((v426 * v17) + (v390 * (v12 - v17))) - ((v48 * v11) * v18);
                        let v445 = v433 + ((v52 * v11) * ((v35 * (v12 + ((v12 + (v57 * (((-v433) * v13).exp()))).sqrt()))).ln()));
                        let v450 = v414 * ((v411 * ((v408 / v445).ln())).exp());
                        let v460: f64;
                        if v451 != 0.0 {
                            let v458 = (v456 * v445) / v408;
                            v460 = v458;
                        } else {
                            v460 = v459;
                        }
                        v453 = v450;
                        v454 = v445;
                        v455 = v460;
                    } else {
                        v453 = v414;
                        v454 = v408;
                        v455 = v452;
                    }
                    v366 = v453;
                    v367 = v454;
                    v368 = v455;
                }
                let v370 = v369 * v18;
                let v376 = v375 * ((v370 + (v371 * v81)).exp());
                let v380 = v379 * ((v370 + v146).exp());
                let v385 = v384 * ((v381 * v18).exp());
                let v465: f64;
                let v466: f64;
                let v467: f64;
                if v386 != 0.0 {
                    let v520: f64;
                    let v521: f64;
                    let v522: f64;
                    if v461 != 0.0 {
                        let v499 = ((v492 * v17) + (v390 * (v12 - v17))) - ((v48 * v11) * v18);
                        let v511 = v499 + ((v52 * v11) * ((v35 * (v12 + ((v12 + (v57 * (((-v499) * v13).exp()))).sqrt()))).ln()));
                        let v517 = v462 * ((v514 * ((v463 / v511).ln())).exp());
                        let v527: f64;
                        if v518 != 0.0 {
                            let v525 = (v523 * v511) / v463;
                            v527 = v525;
                        } else {
                            v527 = v526;
                        }
                        v520 = v517;
                        v521 = v511;
                        v522 = v527;
                    } else {
                        v520 = v462;
                        v521 = v463;
                        v522 = v519;
                    }
                    v465 = v520;
                    v466 = v521;
                    v467 = v522;
                } else {
                    v465 = v462;
                    v466 = v463;
                    v467 = v464;
                }
                let v472 = v471 * ((v468 * v18).exp());
                let v477 = v476 * ((v473 * v18).exp());
                let v482 = v481 * ((v478 * v18).exp());
                let v491 = (v486 * ((v483 * v18).exp())) * (v12 + (v488 * v15));
            [v3, v5, out8, v11, v13, v36, v38, v87, v97, v150, v76, v151, v157, v165, v170, v195, v200, v205, v210, v216, v238, v243, v134, v135, v261, v297, v305, v293, v294, v77, v316, v355, v357, v363, v376, v380, v385, v472, v477, v482, v491, v78, v142, v190, v228, v229, v255, v256, v295, v347, v353, v366, v367, v368, v465, v466, v467]
        };
        self.canonical_staged[49] = produced[0];
        self.canonical_staged[92] = produced[1];
        self.canonical_staged[94] = produced[2];
        self.canonical_staged[118] = produced[3];
        self.canonical_staged[122] = produced[4];
        self.canonical_staged[29] = produced[5];
        self.canonical_staged[21] = produced[6];
        self.canonical_staged[119] = produced[7];
        self.canonical_staged[120] = produced[8];
        self.canonical_staged[138] = produced[9];
        self.canonical_staged[123] = produced[10];
        self.canonical_staged[35] = produced[11];
        self.canonical_staged[128] = produced[12];
        self.canonical_staged[121] = produced[13];
        self.canonical_staged[126] = produced[14];
        self.canonical_staged[134] = produced[15];
        self.canonical_staged[136] = produced[16];
        self.canonical_staged[137] = produced[17];
        self.canonical_staged[132] = produced[18];
        self.canonical_staged[133] = produced[19];
        self.canonical_staged[129] = produced[20];
        self.canonical_staged[135] = produced[21];
        self.canonical_staged[22] = produced[22];
        self.canonical_staged[23] = produced[23];
        self.canonical_staged[141] = produced[24];
        self.canonical_staged[142] = produced[25];
        self.canonical_staged[143] = produced[26];
        self.canonical_staged[31] = produced[27];
        self.canonical_staged[32] = produced[28];
        self.canonical_staged[34] = produced[29];
        self.canonical_staged[145] = produced[30];
        self.canonical_staged[150] = produced[31];
        self.canonical_staged[146] = produced[32];
        self.canonical_staged[149] = produced[33];
        self.canonical_staged[159] = produced[34];
        self.canonical_staged[157] = produced[35];
        self.canonical_staged[158] = produced[36];
        self.canonical_staged[161] = produced[37];
        self.canonical_staged[162] = produced[38];
        self.canonical_staged[160] = produced[39];
        self.canonical_staged[163] = produced[40];
        self.canonical_staged[124] = produced[41];
        self.canonical_staged[125] = produced[42];
        self.canonical_staged[127] = produced[43];
        self.canonical_staged[130] = produced[44];
        self.canonical_staged[131] = produced[45];
        self.canonical_staged[139] = produced[46];
        self.canonical_staged[140] = produced[47];
        self.canonical_staged[144] = produced[48];
        self.canonical_staged[147] = produced[49];
        self.canonical_staged[148] = produced[50];
        self.canonical_staged[151] = produced[51];
        self.canonical_staged[152] = produced[52];
        self.canonical_staged[153] = produced[53];
        self.canonical_staged[154] = produced[54];
        self.canonical_staged[155] = produced[55];
        self.canonical_staged[156] = produced[56];
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
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13]), ctx.node_voltage(self.nodes[14])];
        let branch_unknown_flows = [ctx.branch_current(self.branches[0]), ctx.branch_current(self.branches[1]), ctx.branch_current(self.branches[2]), ctx.branch_current(self.branches[3]), ctx.branch_current(self.branches[4]), ctx.branch_current(self.branches[5])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 20499 => 0usize, 20514 => 1usize, 20535 => 2usize, 20560 => 3usize, 20569 => 4usize, 20573 => 5usize, 20577 => 6usize, 20581 => 7usize, 20618 => 8usize, 20622 => 9usize, 20626 => 10usize, 20654 => 11usize, 20658 => 12usize, 20675 => 13usize, 20703 => 15usize, 20706 => 16usize, 20709 => 17usize, 20699 => 14usize, 20916 => 18usize, 20924 => 19usize, _ => usize::MAX };
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
            let v1 = node_potentials[6];
            let v3 = Lanes([1e0f64; 1]);
            let v5 = Lanes([1e0f64; 1]);
            let v8 = parameters[148];
            let v11 = node_potentials[5];
            let v14 = Lanes([1e0f64; 1]);
            let v23 = node_potentials[7];
            let v25 = Lanes([1e0f64; 1]);
            let v37 = node_potentials[1];
            let v39 = Lanes([1e0f64; 1]);
            let v45 = node_potentials[9];
            let v47 = Lanes([1e0f64; 1]);
            let v53 = node_potentials[3];
            let v54 = node_potentials[0];
            let v56 = Lanes([1e0f64; 1]);
            let v58 = Lanes([1e0f64; 1]);
            let v63 = 0e0f64;
            let v65 = staged[73];
            let v66 = staged[33];
            let v67 = staged[19];
            let v68 = staged[99];
            let v69 = staged[100];
            let v70 = staged[101];
            let v72 = staged[18];
            let v74 = parameters[48];
            let v77 = 1e0f64;
            let v80 = staged[30];
            let v81 = staged[20];
            let v82 = staged[21];
            let v84 = staged[22];
            let v88 = staged[23];
            let v90 = parameters[47];
            let v92 = parameters[37];
            let v96 = parameters[38];
            let v100 = staged[26];
            let v104 = staged[27];
            let v106 = staged[28];
            let v107 = staged[29];
            let v109 = parameters[29];
            let v112 = parameters[44];
            let v117 = staged[31];
            let v119 = staged[32];
            let v120 = parameters[43];
            let v127 = -1.5e0f64;
            let v133 = parameters[40];
            let v138 = parameters[27];
            let v140 = parameters[28];
            let v142 = staged[34];
            let v143 = parameters[39];
            let v147 = staged[35];
            let v151 = -1.5e0f64;
            let v157 = staged[108];
            let v158 = staged[114];
            let v159 = staged[49];
            let v160 = node_potentials[4];
            let v162 = 7.314999999999998e1f64;
            let v164 = staged[118];
            let v165 = staged[119];
            let v166 = staged[120];
            let v167 = staged[121];
            let v168 = staged[122];
            let v169 = staged[123];
            let v170 = staged[124];
            let v171 = staged[125];
            let v172 = staged[126];
            let v173 = staged[127];
            let v174 = staged[128];
            let v175 = staged[129];
            let v176 = staged[130];
            let v177 = staged[131];
            let v178 = staged[132];
            let v179 = staged[133];
            let v180 = staged[134];
            let v181 = staged[135];
            let v182 = staged[136];
            let v183 = staged[137];
            let v184 = staged[138];
            let v185 = staged[139];
            let v186 = staged[140];
            let v187 = staged[141];
            let v188 = staged[142];
            let v189 = staged[143];
            let v190 = staged[144];
            let v191 = staged[145];
            let v192 = staged[146];
            let v193 = staged[147];
            let v194 = staged[148];
            let v195 = staged[149];
            let v196 = staged[150];
            let v197 = staged[151];
            let v198 = staged[152];
            let v199 = staged[153];
            let v200 = staged[154];
            let v201 = staged[155];
            let v202 = staged[156];
            let v203 = staged[157];
            let v204 = staged[158];
            let v205 = staged[159];
            let v206 = staged[160];
            let v207 = staged[161];
            let v208 = staged[162];
            let v209 = staged[163];
            let v210 = Lanes([0e0f64; 1]);
            let v321 = staged[164];
            let v322 = 7.314999999999998e1f64;
            let v323 = 6e2f64;
            let v327 = staged[0];
            let v332 = -1e0f64;
            let v335 = staged[1];
            let v344 = 1e0f64;
            let v347 = parameters[121];
            let v357 = parameters[122];
            let v360 = parameters[117];
            let v364 = parameters[118];
            let v367 = parameters[119];
            let v372 = 5e-1f64;
            let v377 = 6e2f64;
            let v378 = Lanes([1e0f64; 1]);
            let v381 = staged[50];
            let v386 = staged[3];
            let v391 = staged[4];
            let v400 = 2e0f64;
            let v411 = 4e0f64;
            let v416 = 2e0f64;
            let v439 = parameters[41];
            let v446 = staged[165];
            let v447 = parameters[42];
            let v454 = parameters[124];
            let v459 = staged[5];
            let v466 = parameters[14];
            let v469 = staged[6];
            let v472 = staged[7];
            let v475 = parameters[17];
            let v482 = parameters[16];
            let v489 = staged[166];
            let v492 = staged[51];
            let v497 = staged[9];
            let v546 = parameters[49];
            let v553 = staged[167];
            let v554 = parameters[50];
            let v565 = staged[168];
            let v568 = 2.4e0f64;
            let v571 = staged[10];
            let v574 = staged[11];
            let v581 = parameters[23];
            let v595 = parameters[2];
            let v598 = parameters[123];
            let v601 = staged[12];
            let v608 = parameters[1];
            let v611 = parameters[126];
            let v616 = parameters[10];
            let v619 = parameters[125];
            let v622 = parameters[127];
            let v634 = parameters[9];
            let v650 = parameters[8];
            let v655 = staged[13];
            let v660 = parameters[3];
            let v663 = staged[14];
            let v668 = parameters[4];
            let v671 = staged[15];
            let v676 = parameters[6];
            let v679 = staged[16];
            let v684 = parameters[75];
            let v687 = parameters[130];
            let v692 = parameters[74];
            let v699 = parameters[133];
            let v704 = parameters[79];
            let v707 = parameters[78];
            let v708 = parameters[132];
            let v718 = parameters[128];
            let v722 = parameters[129];
            let v731 = parameters[66];
            let v734 = staged[17];
            let v739 = parameters[71];
            let v742 = parameters[139];
            let v747 = parameters[32];
            let v750 = parameters[140];
            let v755 = parameters[33];
            let v768 = parameters[134];
            let v773 = parameters[89];
            let v814 = staged[52];
            let v867 = parameters[45];
            let v874 = staged[169];
            let v875 = parameters[46];
            let v882 = parameters[18];
            let v885 = staged[25];
            let v888 = parameters[21];
            let v895 = parameters[20];
            let v902 = staged[170];
            let v920 = parameters[31];
            let v925 = parameters[30];
            let v928 = 1.0f64;
            let v953 = -1.5e0f64;
            let v955 = -2.5e0f64;
            let v1001 = -1.5e0f64;
            let v1003 = -2.5e0f64;
            let v1019 = staged[53];
            let v1065 = parameters[53];
            let v1073 = parameters[54];
            let v1078 = staged[171];
            let v1079 = parameters[55];
            let v1090 = staged[172];
            let v1095 = staged[37];
            let v1098 = staged[38];
            let v1101 = staged[39];
            let v1108 = parameters[25];
            let v1111 = staged[173];
            let v1112 = staged[174];
            let v1119 = staged[44];
            let v1122 = staged[45];
            let v1129 = parameters[99];
            let v1136 = parameters[97];
            let v1139 = staged[46];
            let v1144 = parameters[101];
            let v1147 = staged[54];
            let v1152 = staged[41];
            let v1194 = parameters[58];
            let v1202 = parameters[59];
            let v1207 = parameters[57];
            let v1210 = 0.0f64;
            let v1211 = -2.4e0f64;
            let v1218 = -2.4e0f64;
            let v1223 = 2.4e0f64;
            let v1226 = staged[55];
            let v1285 = staged[175];
            let v1286 = staged[176];
            let v1293 = staged[56];
            let v1298 = staged[177];
            let v1301 = staged[178];
            let v1302 = parameters[62];
            let v1303 = parameters[63];
            let v1304 = staged[179];
            let v1311 = parameters[136];
            let v1316 = parameters[96];
            let v1319 = parameters[135];
            let v1324 = parameters[90];
            let v1327 = parameters[137];
            let v1332 = parameters[95];
            let v1335 = parameters[143];
            let v1340 = parameters[142];
            let v1343 = parameters[144];
            let v1351 = staged[57];
            let v1404 = parameters[64];
            let v1411 = staged[180];
            let v1412 = staged[181];
            let v1419 = staged[58];
            let v1424 = staged[182];
            let v1427 = parameters[15];
            let v1436 = 8e1f64;
            let v1438 = Lanes([0e0f64; 3]);
            let v1441 = staged[183];
            let v1477 = parameters[13];
            let v1546 = 1.921812e0f64;
            let v1632 = staged[184];
            let v1639 = staged[185];
            let v1641 = parameters[51];
            let v1689 = Lanes([0e0f64; 3]);
            let v1715 = 1e-1f64;
            let v2004 = parameters[11];
            let v2057 = 1e-3f64;
            let v2069 = parameters[12];
            let v2076 = 5e-2f64;
            let v2103 = staged[59];
            let v2148 = staged[60];
            let v2168 = parameters[67];
            let v2175 = parameters[68];
            let v2239 = staged[61];
            let v2245 = parameters[80];
            let v2272 = parameters[77];
            let v2292 = parameters[76];
            let v2298 = parameters[81];
            let v2314 = staged[62];
            let v2335 = staged[72];
            let v2347 = parameters[85];
            let v2403 = 1e-6f64;
            let v2406 = staged[63];
            let v2415 = parameters[70];
            let v2420 = parameters[69];
            let v2432 = parameters[83];
            let v2435 = Lanes([0e0f64; 4]);
            let v2446 = 1e-5f64;
            let v2460 = -1e10f64;
            let v2466 = parameters[73];
            let v2512 = parameters[72];
            let v2525 = parameters[82];
            let v2570 = parameters[115];
            let v2571 = 1e-2f64;
            let v2573 = parameters[116];
            let v2577 = 5e-3f64;
            let v2583 = -1e10f64;
            let v2589 = parameters[84];
            let v2597 = -2e0f64;
            let v2643 = staged[64];
            let v2664 = staged[65];
            let v2669 = staged[66];
            let v2686 = staged[68];
            let v2767 = staged[67];
            let v2777 = 2.5e-1f64;
            let v2834 = staged[69];
            let v2865 = staged[70];
            let v2866 = staged[71];
            let v2895 = -2e0f64;
            let v2941 = parameters[5];
            let v3012 = parameters[7];
            let v3056 = parameters[93];
            let v3079 = 1e2f64;
            let v3188 = 3e-1f64;
            let v3192 = 0e0f64;
            let v3203 = -1e10f64;
            let v3319 = -1e10f64;
            let v3332 = -2e0f64;
            let v3621 = -2e0f64;
            let v3788 = -1e10f64;
            let v3904 = -1e10f64;
            let v3917 = -2e0f64;
            let v4206 = -2e0f64;
            let v4271 = parameters[24];
            let v4357 = staged[74];
            let v4361 = parameters[35];
            let v4376 = parameters[36];
            let v4489 = parameters[34];
            let v4498 = 1e-4f64;
            let v4522 = parameters[92];
            let v4558 = staged[186];
            let v4564 = parameters[91];
            let v4598 = parameters[94];
            let v4615 = parameters[19];
            let v4625 = Lanes([0e0f64; 3]);
            let v4628 = staged[187];
            let v4793 = Lanes([0e0f64; 4]);
            let v4807 = staged[188];
            let v4910 = staged[189];
            let v4912 = parameters[56];
            let v4960 = Lanes([0e0f64; 3]);
            let v5186 = parameters[26];
            let v5220 = staged[190];
            let v5269 = Lanes([0e0f64; 3]);
            let v5500 = parameters[61];
            let v5548 = Lanes([0e0f64; 3]);
            let v5774 = staged[191];
            let v5780 = staged[192];
            let v5786 = parameters[65];
            let v5834 = Lanes([0e0f64; 3]);
            let v6060 = parameters[98];
            let v6088 = staged[193];
            let v6089 = Lanes([0e0f64; 4]);
            let v6094 = staged[194];
            let v6106 = parameters[100];
            let v6118 = staged[195];
            let v6137 = staged[196];
            let v6138 = 0e0f64;
            let v6139 = Lanes([0e0f64; 9]);
            let v6142 = staged[197];
            let v6159 = staged[198];
            let v6219 = parameters[149];
            let v6242 = node_potentials[2];
            let v6245 = Lanes([1e0f64; 1]);
            let v6310 = node_potentials[11];
            let v6312 = Lanes([1e0f64; 1]);
            let v6323 = node_potentials[10];
            let v6326 = Lanes([1e0f64; 1]);
            let v6337 = parameters[88];
            let v6344 = 3e0f64;
            let v6353 = node_potentials[12];
            let v6355 = Lanes([1e0f64; 1]);
            let v6364 = parameters[87];
            let v6375 = Lanes([0e0f64; 1]);
            let v6376 = Lanes([0e0f64; 1]);
            let v6377 = Lanes([0e0f64; 1]);
            let v6414 = ddt_scale();
            let v6437 = staged[199];
            let v6444 = staged[200];
            let v6445 = Lanes([0e0f64; 5]);
            let v6452 = staged[201];
            let v6459 = staged[75];
            let v6462 = staged[76];
            let v6469 = staged[77];
            let v6488 = staged[78];
            let v6497 = staged[79];
            let v6502 = staged[202];
            let v6513 = Lanes([0e0f64; 3]);
            let v6516 = staged[203];
            let v6527 = Lanes([0e0f64; 3]);
            let v6530 = staged[204];
            let v6541 = Lanes([0e0f64; 3]);
            let v6548 = staged[80];
            let v6557 = staged[81];
            let v6566 = parameters[108];
            let v6591 = staged[205];
            let v6596 = Lanes([0e0f64; 2]);
            let v6609 = parameters[102];
            let v6612 = staged[206];
            let v6613 = Lanes([0e0f64; 2]);
            let v6620 = staged[207];
            let v6621 = parameters[103];
            let v6637 = staged[208];
            let v6650 = parameters[145];
            let v6659 = staged[210];
            let v6661 = node_potentials[13];
            let v6662 = node_potentials[14];
            let v6663 = Lanes([0e0f64; 1]);
            let v6664 = Lanes([0e0f64; 5]);
            let v6665 = Lanes([0e0f64; 5]);
            let v6666 = Lanes([0e0f64; 1]);
            let v6667 = Lanes([1e0f64; 1]);
            let v6668 = Lanes([1e0f64; 1]);
            let v6692 = staged[83];
            let v6694 = branch_unknown_flows[1];
            let v6702 = 1e9f64;
            let v6707 = staged[82];
            let v7004 = 0e0f64;
            let v7005 = 0e0f64;
            let v7006 = 0e0f64;
            let v7007 = 0e0f64;
            let v7008 = 0e0f64;
            let v7009 = 0e0f64;
            let v2 = v0 - v1;
            let v7 = (Lanes([0.0, v3[0]])) - (Lanes([v5[0], 0.0]));
            let v9 = v8 * v2;
            let v10 = v7 * v8;
            let v12 = v0 - v11;
            let v16 = (Lanes([0.0, v3[0]])) - (Lanes([v14[0], 0.0]));
            let v17 = v8 * v12;
            let v18 = v16 * v8;
            let v19 = v9 - v17;
            let v22 = (Lanes([0.0, v10[0], v10[1]])) - (Lanes([v18[0], 0.0, v18[1]]));
            let v29 = v8 * (v23 - v1);
            let v30 = ((Lanes([0.0, v25[0]])) - (Lanes([v5[0], 0.0]))) * v8;
            let v31 = v23 - v11;
            let v34 = (Lanes([0.0, v25[0]])) - (Lanes([v14[0], 0.0]));
            let v35 = v8 * v31;
            let v36 = v34 * v8;
            let v38 = v37 - v11;
            let v42 = (Lanes([v39[0], 0.0])) - (Lanes([0.0, v14[0]]));
            let v43 = v8 * v38;
            let v44 = v42 * v8;
            let v46 = v45 - v11;
            let v50 = (Lanes([0.0, v47[0]])) - (Lanes([v14[0], 0.0]));
            let v51 = v8 * v46;
            let v52 = v50 * v8;
            let v61 = v8 * (v53 - v54);
            let v62 = ((Lanes([0.0, v56[0]])) - (Lanes([v58[0], 0.0]))) * v8;
            let v64 = ctx.simparam_or("gmin", v63);
            let v73 = if v72 != 0.0 && (if v17 < v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v78: f64;
            let v79: f64;
            if v73 != 0.0 {
                let v76 = if v67 != 0.0 && (if v74 > v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v98: f64;
                let v99: f64;
                if v76 != 0.0 {
                    let v83 = v81 / v82;
                    let v85 = v84 / v74;
                    let v91 = (((v83.sqrt()) * v85) * v88) / v90;
                    let v94 = (v92 * v91) * v85;
                    let v97 = v96 / (v91 * v83);
                    v98 = v94;
                    v99 = v97;
                } else {
                    v98 = v92;
                    v99 = v96;
                }
                v78 = v98;
                v79 = v99;
            } else {
                v78 = v63;
                v79 = v77;
            }
            let v105 = if v104 != 0.0 && (if (if v29 < v100 { 1.0 } else { 0.0 }) != 0.0 || (if v9 < v100 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v115: f64;
            let v116: f64;
            if v105 != 0.0 {
                let v108 = v106 / v107;
                let v114 = if (if (if v109 == v77 { 1.0 } else { 0.0 }) != 0.0 && v80 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v112 > v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v136: f64;
                let v137: f64;
                if v114 != 0.0 {
                    let v118 = v117 / v112;
                    let v125 = (((v119 / v120) * (v108.sqrt())) * v118) * v118;
                    let v130 = ((v120 / v119) * (v108.powf(v127))) / v118;
                    v136 = v125;
                    v137 = v130;
                } else {
                    let v135 = if (if (if v109 == v63 { 1.0 } else { 0.0 }) != 0.0 && v66 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v133 > v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v155: f64;
                    let v156: f64;
                    if v135 != 0.0 {
                        let v149 = (((v142 / v143) * (v108.sqrt())) * v147) * v147;
                        let v154 = ((v143 / v142) * (v108.powf(v151))) / v147;
                        v155 = v149;
                        v156 = v154;
                    } else {
                        v155 = v77;
                        v156 = v77;
                    }
                    v136 = v155;
                    v137 = v156;
                }
                let v139 = v138 * v136;
                let v141 = v140 * v137;
                v115 = v139;
                v116 = v141;
            } else {
                v115 = v63;
                v116 = v77;
            }
            let v211: f64;
            let v212: f64;
            let v213: f64;
            let v214: f64;
            let v215: f64;
            let v216: f64;
            let v217: f64;
            let v218: f64;
            let v219: f64;
            let v220: f64;
            let v221: f64;
            let v222: f64;
            let v223: f64;
            let v224: f64;
            let v225: f64;
            let v226: f64;
            let v227: f64;
            let v228: f64;
            let v229: f64;
            let v230: f64;
            let v231: f64;
            let v232: f64;
            let v233: f64;
            let v234: f64;
            let v235: f64;
            let v236: f64;
            let v237: f64;
            let v238: f64;
            let v239: f64;
            let v240: f64;
            let v241: f64;
            let v242: f64;
            let v243: f64;
            let v244: f64;
            let v245: f64;
            let v246: f64;
            let v247: f64;
            let v248: f64;
            let v249: f64;
            let v250: f64;
            let v251: f64;
            let v252: f64;
            let v253: f64;
            let v254: f64;
            let v255: f64;
            let v256: f64;
            let v257: f64;
            let v258: f64;
            let v259: f64;
            let v260: f64;
            let v261: f64;
            let v262: f64;
            let v263: f64;
            let v264: f64;
            let v265: f64;
            let v266: Lanes<1>;
            let v267: Lanes<1>;
            let v268: Lanes<1>;
            let v269: Lanes<1>;
            let v270: Lanes<1>;
            let v271: Lanes<1>;
            let v272: Lanes<1>;
            let v273: Lanes<1>;
            let v274: Lanes<1>;
            let v275: Lanes<1>;
            let v276: Lanes<1>;
            let v277: Lanes<1>;
            let v278: Lanes<1>;
            let v279: Lanes<1>;
            let v280: Lanes<1>;
            let v281: Lanes<1>;
            let v282: Lanes<1>;
            let v283: Lanes<1>;
            let v284: Lanes<1>;
            let v285: Lanes<1>;
            let v286: Lanes<1>;
            let v287: Lanes<1>;
            let v288: Lanes<1>;
            let v289: Lanes<1>;
            let v290: Lanes<1>;
            let v291: Lanes<1>;
            let v292: Lanes<1>;
            let v293: Lanes<1>;
            let v294: Lanes<1>;
            let v295: Lanes<1>;
            let v296: Lanes<1>;
            let v297: Lanes<1>;
            let v298: Lanes<1>;
            let v299: Lanes<1>;
            let v300: Lanes<1>;
            let v301: Lanes<1>;
            let v302: Lanes<1>;
            let v303: Lanes<1>;
            let v304: Lanes<1>;
            let v305: Lanes<1>;
            let v306: Lanes<1>;
            let v307: Lanes<1>;
            let v308: Lanes<1>;
            let v309: Lanes<1>;
            let v310: Lanes<1>;
            let v311: Lanes<1>;
            let v312: Lanes<1>;
            let v313: Lanes<1>;
            let v314: Lanes<1>;
            let v315: Lanes<1>;
            let v316: Lanes<1>;
            let v317: Lanes<1>;
            let v318: Lanes<1>;
            let v319: Lanes<1>;
            let v320: Lanes<1>;
            if v158 != 0.0 {
                let v161 = v159 + v160;
                let v163 = if v161 < v162 { 1.0 } else { 0.0 };
                let v325: f64;
                let v326: Lanes<1>;
                if v163 != 0.0 {
                    v325 = v322;
                    v326 = v210;
                } else {
                    let v324 = if v161 > v323 { 1.0 } else { 0.0 };
                    let v379: f64;
                    let v380: Lanes<1>;
                    if v324 != 0.0 {
                        v379 = v377;
                        v380 = v210;
                    } else {
                        v379 = v161;
                        v380 = v378;
                    }
                    v325 = v379;
                    v326 = v380;
                }
                let v328 = v327 * v325;
                let v329 = v326 * v327;
                let v330 = v77 / v328;
                let v334 = ((v329 * v330) * v332) / v328;
                let v336 = v325 - v335;
                let v337 = v335 / v325;
                let v340 = ((v326 * v337) * v332) / v325;
                let v341 = v325 / v335;
                let v342 = v326 / v335;
                let v343 = v341.ln();
                let v346 = v342 * (v344 / v341);
                let v348 = v347 * v325;
                let v350 = v325.ln();
                let v353 = v348 * v350;
                let v358 = v357 * v325;
                let v362 = (v360 + v353) + v358;
                let v363 = (((v326 * v347) * v350) + ((v326 * (v344 / v325)) * v348)) + (v326 * v357);
                let v373 = (v362 + ((v364 + v353) + v358)) * v372;
                let v374 = (v363 + v363) * v372;
                let v376 = (v362 + ((v367 + v353) + v358)) * v372;
                let v448: f64;
                let v449: f64;
                let v450: f64;
                let v451: Lanes<1>;
                let v452: Lanes<1>;
                let v453: Lanes<1>;
                if v66 != 0.0 {
                    let v392 = v391 * v328;
                    let v398 = ((v381 * v341) + (v386 * (v77 - v341))) - (v392 * v343);
                    let v399 = ((v342 * v381) + ((v342 * v332) * v386)) - (((v329 * v391) * v343) + (v346 * v392));
                    let v401 = v400 * v328;
                    let v403 = -v398;
                    let v409 = (v403 * v330).exp();
                    let v415 = (v77 + (v411 * v409)).sqrt();
                    let v421 = v372 * (v77 + v415);
                    let v423 = v421.ln();
                    let v430 = v398 + (v401 * v423);
                    let v431 = v399 + (((v329 * v400) * v423) + (((((((((v399 * v332) * v330) + (v334 * v403)) * v409) * v411) * (v344 / (v416 * v415))) * v372) * (v344 / v421)) * v401));
                    let v432 = v133 / v430;
                    let v442 = (v439 * (v432.ln())).exp();
                    let v444 = v143 * v442;
                    let v445 = ((((((v431 * v432) * v332) / v430) * (v344 / v432)) * v439) * v442) * v143;
                    let v490: f64;
                    let v491: Lanes<1>;
                    if v446 != 0.0 {
                        let v487 = (v447 * v430) / v133;
                        let v488 = (v431 * v447) / v133;
                        v490 = v487;
                        v491 = v488;
                    } else {
                        v490 = v489;
                        v491 = v210;
                    }
                    v448 = v430;
                    v449 = v444;
                    v450 = v490;
                    v451 = v431;
                    v452 = v445;
                    v453 = v491;
                } else {
                    v448 = v133;
                    v449 = v143;
                    v450 = v447;
                    v451 = v210;
                    v452 = v210;
                    v453 = v210;
                }
                let v457 = v77 - v337;
                let v458 = v340 * v332;
                let v464 = ((v454 * v343) + (v459 * v457)).exp();
                let v465 = ((v346 * v454) + (v458 * v459)) * v464;
                let v467 = v466 * v464;
                let v468 = v465 * v466;
                let v473 = v472 * v457;
                let v474 = v458 * v472;
                let v480 = ((v469 * v343) + (v473 / v475)).exp();
                let v483 = v482 * v480;
                let v484 = (((v346 * v469) + (v474 / v475)) * v480) * v482;
                let v555: f64;
                let v556: f64;
                let v557: f64;
                let v558: Lanes<1>;
                let v559: Lanes<1>;
                let v560: Lanes<1>;
                if v67 != 0.0 {
                    let v502 = v391 * v328;
                    let v508 = ((v492 * v341) + (v497 * (v77 - v341))) - (v502 * v343);
                    let v509 = ((v342 * v492) + ((v342 * v332) * v497)) - (((v329 * v391) * v343) + (v346 * v502));
                    let v510 = v400 * v328;
                    let v512 = -v508;
                    let v518 = (v512 * v330).exp();
                    let v523 = (v77 + (v411 * v518)).sqrt();
                    let v528 = v372 * (v77 + v523);
                    let v530 = v528.ln();
                    let v537 = v508 + (v510 * v530);
                    let v538 = v509 + (((v329 * v400) * v530) + (((((((((v509 * v332) * v330) + (v334 * v512)) * v518) * v411) * (v344 / (v416 * v523))) * v372) * (v344 / v528)) * v510));
                    let v539 = v74 / v537;
                    let v549 = (v546 * (v539.ln())).exp();
                    let v551 = v90 * v549;
                    let v552 = ((((((v538 * v539) * v332) / v537) * (v344 / v539)) * v546) * v549) * v90;
                    let v566: f64;
                    let v567: Lanes<1>;
                    if v553 != 0.0 {
                        let v563 = (v554 * v537) / v74;
                        let v564 = (v538 * v554) / v74;
                        v566 = v563;
                        v567 = v564;
                    } else {
                        v566 = v565;
                        v567 = v210;
                    }
                    v555 = v537;
                    v556 = v551;
                    v557 = v566;
                    v558 = v538;
                    v559 = v552;
                    v560 = v567;
                } else {
                    v555 = v74;
                    v556 = v90;
                    v557 = v554;
                    v558 = v210;
                    v559 = v210;
                    v560 = v210;
                }
                let v569: f64;
                let v570: Lanes<1>;
                if v65 != 0.0 {
                    v569 = v568;
                    v570 = v210;
                } else {
                    v569 = v557;
                    v570 = v560;
                }
                let v575 = v574 * v457;
                let v576 = v458 * v574;
                let v579 = ((v571 * v343) + v575).exp();
                let v582 = v581 * v579;
                let v583 = (((v346 * v571) + v576) * v579) * v581;
                let v584 = v448 / v133;
                let v585 = v451 / v133;
                let v591 = (v439 * (v584.ln())).exp();
                let v596 = v595 * (v400 - v591);
                let v597 = ((((v585 * (v344 / v584)) * v439) * v591) * v332) * v595;
                let v606 = ((v598 * v343) + (v601 * v457)).exp();
                let v609 = v608 * v606;
                let v610 = (((v346 * v598) + (v458 * v601)) * v606) * v608;
                let v614 = (v611 * v343).exp();
                let v617 = v616 * v614;
                let v618 = ((v346 * v611) * v614) * v616;
                let v653: f64;
                let v654: Lanes<1>;
                if v68 != 0.0 {
                    let v620 = v619 * v330;
                    let v625 = (v622 * v343).exp();
                    let v627 = v625 - v77;
                    let v632 = (v620 * v627).exp();
                    let v635 = v634 * v632;
                    let v636 = ((((v334 * v619) * v627) + (((v346 * v622) * v625) * v620)) * v632) * v634;
                    v653 = v635;
                    v654 = v636;
                } else {
                    let v637 = v619 * v330;
                    let v641 = (v622 * v343).exp();
                    let v643 = v641 - v77;
                    let v648 = (v637 * v643).exp();
                    let v651 = v650 * v648;
                    let v652 = ((((v334 * v619) * v643) + (((v346 * v622) * v641) * v637)) * v648) * v650;
                    v653 = v651;
                    v654 = v652;
                }
                let v658 = (v655 * v457).exp();
                let v661 = v660 * v658;
                let v662 = ((v458 * v655) * v658) * v660;
                let v666 = (v663 * v457).exp();
                let v669 = v668 * v666;
                let v670 = ((v458 * v663) * v666) * v668;
                let v674 = (v671 * v457).exp();
                let v677 = v676 * v674;
                let v678 = ((v458 * v671) * v674) * v676;
                let v682 = (v679 * v343).exp();
                let v685 = v684 * v682;
                let v686 = ((v346 * v679) * v682) * v684;
                let v690 = (v687 * v343).exp();
                let v693 = v692 * v690;
                let v695 = v77 / v693;
                let v698 = (((((v346 * v687) * v690) * v692) * v695) * v332) / v693;
                let v714: f64;
                let v715: f64;
                let v716: Lanes<1>;
                let v717: Lanes<1>;
                if v69 != 0.0 {
                    let v705 = v704 * (v77 - (v699 * v336));
                    let v706 = ((v326 * v699) * v332) * v704;
                    v714 = v705;
                    v715 = v707;
                    v716 = v706;
                    v717 = v210;
                } else {
                    let v712 = v707 * (v77 + (v708 * v336));
                    let v713 = (v326 * v708) * v707;
                    v714 = v704;
                    v715 = v712;
                    v716 = v210;
                    v717 = v713;
                }
                let v723 = v722 * v336;
                let v732 = v731 * ((v77 + (v718 * v336)) + (v723 * v336));
                let v733 = ((v326 * v718) + (((v326 * v722) * v336) + (v326 * v723))) * v731;
                let v737 = (v734 * v343).exp();
                let v740 = v739 * v737;
                let v741 = ((v346 * v734) * v737) * v739;
                let v758: f64;
                let v759: f64;
                let v760: Lanes<1>;
                let v761: Lanes<1>;
                if v70 != 0.0 {
                    let v745 = (v742 * v336).exp();
                    let v748 = v747 * v745;
                    let v749 = ((v326 * v742) * v745) * v747;
                    let v753 = (v750 * v336).exp();
                    let v756 = v755 * v753;
                    let v757 = ((v326 * v750) * v753) * v755;
                    v758 = v756;
                    v759 = v748;
                    v760 = v757;
                    v761 = v749;
                } else {
                    v758 = v755;
                    v759 = v747;
                    v760 = v210;
                    v761 = v210;
                }
                let v764: f64;
                let v765: f64;
                let v766: Lanes<1>;
                let v767: Lanes<1>;
                if v73 != 0.0 {
                    let v763 = if v67 != 0.0 && (if v74 > v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v810: f64;
                    let v811: f64;
                    let v812: Lanes<1>;
                    let v813: Lanes<1>;
                    if v763 != 0.0 {
                        let v776 = v81 / v376;
                        let v779 = ((v374 * v776) * v332) / v376;
                        let v780 = v555 / v74;
                        let v781 = v558 / v74;
                        let v782 = v776.sqrt();
                        let v786 = v782 * v780;
                        let v794 = (v786 * v556) / v90;
                        let v795 = (((((v779 * (v344 / (v416 * v782))) * v780) + (v781 * v782)) * v556) + (v559 * v786)) / v90;
                        let v796 = v92 * v794;
                        let v798 = v796 * v780;
                        let v801 = ((v795 * v92) * v780) + (v781 * v796);
                        let v802 = v794 * v776;
                        let v806 = v96 / v802;
                        let v809 = ((((v795 * v776) + (v779 * v794)) * v806) * v332) / v802;
                        v810 = v798;
                        v811 = v806;
                        v812 = v801;
                        v813 = v809;
                    } else {
                        v810 = v92;
                        v811 = v96;
                        v812 = v210;
                        v813 = v210;
                    }
                    v764 = v810;
                    v765 = v811;
                    v766 = v812;
                    v767 = v813;
                } else {
                    v764 = v63;
                    v765 = v77;
                    v766 = v210;
                    v767 = v210;
                }
                let v771 = (v768 * v343).exp();
                let v774 = v773 * v771;
                let v775 = ((v346 * v768) * v771) * v773;
                let v876: f64;
                let v877: f64;
                let v878: f64;
                let v879: Lanes<1>;
                let v880: Lanes<1>;
                let v881: Lanes<1>;
                if v80 != 0.0 {
                    let v823 = v391 * v328;
                    let v829 = ((v814 * v341) + (v386 * (v77 - v341))) - (v823 * v343);
                    let v830 = ((v342 * v814) + ((v342 * v332) * v386)) - (((v329 * v391) * v343) + (v346 * v823));
                    let v831 = v400 * v328;
                    let v833 = -v829;
                    let v839 = (v833 * v330).exp();
                    let v844 = (v77 + (v411 * v839)).sqrt();
                    let v849 = v372 * (v77 + v844);
                    let v851 = v849.ln();
                    let v858 = v829 + (v831 * v851);
                    let v859 = v830 + (((v329 * v400) * v851) + (((((((((v830 * v332) * v330) + (v334 * v833)) * v839) * v411) * (v344 / (v416 * v844))) * v372) * (v344 / v849)) * v831));
                    let v860 = v112 / v858;
                    let v870 = (v867 * (v860.ln())).exp();
                    let v872 = v120 * v870;
                    let v873 = ((((((v859 * v860) * v332) / v858) * (v344 / v860)) * v867) * v870) * v120;
                    let v903: f64;
                    let v904: Lanes<1>;
                    if v874 != 0.0 {
                        let v900 = (v875 * v858) / v112;
                        let v901 = (v859 * v875) / v112;
                        v903 = v900;
                        v904 = v901;
                    } else {
                        v903 = v902;
                        v904 = v210;
                    }
                    v876 = v858;
                    v877 = v872;
                    v878 = v903;
                    v879 = v859;
                    v880 = v873;
                    v881 = v904;
                } else {
                    v876 = v112;
                    v877 = v120;
                    v878 = v875;
                    v879 = v210;
                    v880 = v210;
                    v881 = v210;
                }
                let v883 = v882 * v464;
                let v884 = v465 * v882;
                let v893 = ((v885 * v343) + (v473 / v888)).exp();
                let v896 = v895 * v893;
                let v897 = (((v346 * v885) + (v474 / v888)) * v893) * v895;
                let v913: f64;
                let v914: f64;
                let v915: Lanes<1>;
                let v916: Lanes<1>;
                if v105 != 0.0 {
                    let v905 = v106 / v373;
                    let v908 = ((v374 * v905) * v332) / v373;
                    let v912 = if (if (if v109 == v77 { 1.0 } else { 0.0 }) != 0.0 && v80 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v112 > v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v971: f64;
                    let v972: f64;
                    let v973: Lanes<1>;
                    let v974: Lanes<1>;
                    if v912 != 0.0 {
                        let v929 = v876 / v112;
                        let v930 = v879 / v112;
                        let v931 = v877 / v120;
                        let v933 = v905.sqrt();
                        let v937 = v931 * v933;
                        let v941 = v937 * v929;
                        let v945 = v941 * v929;
                        let v948 = ((((((v880 / v120) * v933) + ((v908 * (v344 / (v416 * v933))) * v931)) * v929) + (v930 * v937)) * v929) + (v930 * v941);
                        let v949 = v120 / v877;
                        let v954 = v905.powf(v953);
                        let v963 = (v949 * v954) / v929;
                        let v966 = ((((((v880 * v949) * v332) / v877) * v954) + ((v908 * (v953 * (v905.powf(v955)))) * v949)) - (v930 * v963)) / v929;
                        v971 = v945;
                        v972 = v963;
                        v973 = v948;
                        v974 = v966;
                    } else {
                        let v970 = if (if (if v109 == v63 { 1.0 } else { 0.0 }) != 0.0 && v66 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v133 > v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v1015: f64;
                        let v1016: f64;
                        let v1017: Lanes<1>;
                        let v1018: Lanes<1>;
                        if v970 != 0.0 {
                            let v979 = v449 / v143;
                            let v981 = v905.sqrt();
                            let v985 = v979 * v981;
                            let v989 = v985 * v584;
                            let v993 = v989 * v584;
                            let v996 = ((((((v452 / v143) * v981) + ((v908 * (v344 / (v416 * v981))) * v979)) * v584) + (v585 * v985)) * v584) + (v585 * v989);
                            let v997 = v143 / v449;
                            let v1002 = v905.powf(v1001);
                            let v1011 = (v997 * v1002) / v584;
                            let v1014 = ((((((v452 * v997) * v332) / v449) * v1002) + ((v908 * (v1001 * (v905.powf(v1003)))) * v997)) - (v585 * v1011)) / v584;
                            v1015 = v993;
                            v1016 = v1011;
                            v1017 = v996;
                            v1018 = v1014;
                        } else {
                            v1015 = v77;
                            v1016 = v77;
                            v1017 = v210;
                            v1018 = v210;
                        }
                        v971 = v1015;
                        v972 = v1016;
                        v973 = v1017;
                        v974 = v1018;
                    }
                    let v975 = v138 * v971;
                    let v976 = v973 * v138;
                    let v977 = v140 * v972;
                    let v978 = v974 * v140;
                    v913 = v975;
                    v914 = v977;
                    v915 = v976;
                    v916 = v978;
                } else {
                    v913 = v63;
                    v914 = v77;
                    v915 = v210;
                    v916 = v210;
                }
                let v923 = ((-(v448 - v133)) / v920).exp();
                let v926 = v925 * v923;
                let v927 = (((v451 * v332) / v920) * v923) * v925;
                let v1080: f64;
                let v1081: f64;
                let v1082: f64;
                let v1083: Lanes<1>;
                let v1084: Lanes<1>;
                let v1085: Lanes<1>;
                if v928 != 0.0 {
                    let v1028 = v391 * v328;
                    let v1034 = ((v1019 * v341) + (v497 * (v77 - v341))) - (v1028 * v343);
                    let v1035 = ((v342 * v1019) + ((v342 * v332) * v497)) - (((v329 * v391) * v343) + (v346 * v1028));
                    let v1036 = v400 * v328;
                    let v1038 = -v1034;
                    let v1044 = (v1038 * v330).exp();
                    let v1049 = (v77 + (v411 * v1044)).sqrt();
                    let v1054 = v372 * (v77 + v1049);
                    let v1056 = v1054.ln();
                    let v1063 = v1034 + (v1036 * v1056);
                    let v1064 = v1035 + (((v329 * v400) * v1056) + (((((((((v1035 * v332) * v330) + (v334 * v1038)) * v1044) * v411) * (v344 / (v416 * v1049))) * v372) * (v344 / v1054)) * v1036));
                    let v1066 = v1065 / v1063;
                    let v1076 = (v1073 * (v1066.ln())).exp();
                    let v1077 = (((((v1064 * v1066) * v332) / v1063) * (v344 / v1066)) * v1073) * v1076;
                    let v1091: f64;
                    let v1092: Lanes<1>;
                    if v1078 != 0.0 {
                        let v1088 = (v1079 * v1063) / v1065;
                        let v1089 = (v1064 * v1079) / v1065;
                        v1091 = v1088;
                        v1092 = v1089;
                    } else {
                        v1091 = v1090;
                        v1092 = v210;
                    }
                    v1080 = v1076;
                    v1081 = v1063;
                    v1082 = v1091;
                    v1083 = v1077;
                    v1084 = v1064;
                    v1085 = v1092;
                } else {
                    v1080 = v77;
                    v1081 = v1065;
                    v1082 = v1079;
                    v1083 = v210;
                    v1084 = v210;
                    v1085 = v210;
                }
                let v1093: f64;
                let v1094: Lanes<1>;
                if v65 != 0.0 {
                    v1093 = v568;
                    v1094 = v210;
                } else {
                    v1093 = v1082;
                    v1094 = v1085;
                }
                let v1096 = v1080 * v1095;
                let v1097 = v1083 * v1095;
                let v1099 = v1080 * v1098;
                let v1100 = v1083 * v1098;
                let v1106 = ((v1101 * v343) + v575).exp();
                let v1109 = v1108 * v1106;
                let v1110 = (((v346 * v1101) + v576) * v1106) * v1108;
                let v1113: f64;
                let v1114: f64;
                let v1115: f64;
                let v1116: Lanes<1>;
                let v1117: Lanes<1>;
                let v1118: Lanes<1>;
                if v65 != 0.0 {
                    let v1212: f64;
                    let v1213: f64;
                    let v1214: f64;
                    let v1215: Lanes<1>;
                    let v1216: Lanes<1>;
                    let v1217: Lanes<1>;
                    if v1111 != 0.0 {
                        let v1157 = v391 * v328;
                        let v1163 = ((v1147 * v341) + (v1152 * (v77 - v341))) - (v1157 * v343);
                        let v1164 = ((v342 * v1147) + ((v342 * v332) * v1152)) - (((v329 * v391) * v343) + (v346 * v1157));
                        let v1165 = v400 * v328;
                        let v1167 = -v1163;
                        let v1173 = (v1167 * v330).exp();
                        let v1178 = (v77 + (v411 * v1173)).sqrt();
                        let v1183 = v372 * (v77 + v1178);
                        let v1185 = v1183.ln();
                        let v1192 = v1163 + (v1165 * v1185);
                        let v1193 = v1164 + (((v329 * v400) * v1185) + (((((((((v1164 * v332) * v330) + (v334 * v1167)) * v1173) * v411) * (v344 / (v416 * v1178))) * v372) * (v344 / v1183)) * v1165));
                        let v1195 = v1194 / v1192;
                        let v1205 = (v1202 * (v1195.ln())).exp();
                        let v1208 = v1207 * v1205;
                        let v1209 = ((((((v1193 * v1195) * v332) / v1192) * (v344 / v1195)) * v1202) * v1205) * v1207;
                        let v1224: f64;
                        let v1225: Lanes<1>;
                        if v1210 != 0.0 {
                            let v1221 = (v1218 * v1192) / v1194;
                            let v1222 = (v1193 * v1218) / v1194;
                            v1224 = v1221;
                            v1225 = v1222;
                        } else {
                            v1224 = v1223;
                            v1225 = v210;
                        }
                        v1212 = v1208;
                        v1213 = v1192;
                        v1214 = v1224;
                        v1215 = v1209;
                        v1216 = v1193;
                        v1217 = v1225;
                    } else {
                        v1212 = v1207;
                        v1213 = v1194;
                        v1214 = v1211;
                        v1215 = v210;
                        v1216 = v210;
                        v1217 = v210;
                    }
                    v1113 = v1212;
                    v1114 = v1213;
                    v1115 = v1214;
                    v1116 = v1215;
                    v1117 = v1216;
                    v1118 = v1217;
                } else {
                    let v1287: f64;
                    let v1288: f64;
                    let v1289: f64;
                    let v1290: Lanes<1>;
                    let v1291: Lanes<1>;
                    let v1292: Lanes<1>;
                    if v1112 != 0.0 {
                        let v1235 = v391 * v328;
                        let v1241 = ((v1226 * v341) + (v1152 * (v77 - v341))) - (v1235 * v343);
                        let v1242 = ((v342 * v1226) + ((v342 * v332) * v1152)) - (((v329 * v391) * v343) + (v346 * v1235));
                        let v1243 = v400 * v328;
                        let v1245 = -v1241;
                        let v1251 = (v1245 * v330).exp();
                        let v1256 = (v77 + (v411 * v1251)).sqrt();
                        let v1261 = v372 * (v77 + v1256);
                        let v1263 = v1261.ln();
                        let v1270 = v1241 + (v1243 * v1263);
                        let v1271 = v1242 + (((v329 * v400) * v1263) + (((((((((v1242 * v332) * v330) + (v334 * v1245)) * v1251) * v411) * (v344 / (v416 * v1256))) * v372) * (v344 / v1261)) * v1243));
                        let v1272 = v1194 / v1270;
                        let v1281 = (v1202 * (v1272.ln())).exp();
                        let v1283 = v1207 * v1281;
                        let v1284 = ((((((v1271 * v1272) * v332) / v1270) * (v344 / v1272)) * v1202) * v1281) * v1207;
                        let v1299: f64;
                        let v1300: Lanes<1>;
                        if v1285 != 0.0 {
                            let v1296 = (v1293 * v1270) / v1194;
                            let v1297 = (v1271 * v1293) / v1194;
                            v1299 = v1296;
                            v1300 = v1297;
                        } else {
                            v1299 = v1298;
                            v1300 = v210;
                        }
                        v1287 = v1283;
                        v1288 = v1270;
                        v1289 = v1299;
                        v1290 = v1284;
                        v1291 = v1271;
                        v1292 = v1300;
                    } else {
                        v1287 = v1207;
                        v1288 = v1194;
                        v1289 = v1286;
                        v1290 = v210;
                        v1291 = v210;
                        v1292 = v210;
                    }
                    v1113 = v1287;
                    v1114 = v1288;
                    v1115 = v1289;
                    v1116 = v1290;
                    v1117 = v1291;
                    v1118 = v1292;
                }
                let v1120 = v1119 * v343;
                let v1121 = v346 * v1119;
                let v1127 = (v1120 + (v1122 * v457)).exp();
                let v1130 = v1129 * v1127;
                let v1131 = ((v1121 + (v458 * v1122)) * v1127) * v1129;
                let v1134 = (v1120 + v575).exp();
                let v1137 = v1136 * v1134;
                let v1138 = ((v1121 + v576) * v1134) * v1136;
                let v1142 = (v1139 * v343).exp();
                let v1145 = v1144 * v1142;
                let v1146 = ((v346 * v1139) * v1142) * v1144;
                let v1305: f64;
                let v1306: f64;
                let v1307: f64;
                let v1308: Lanes<1>;
                let v1309: Lanes<1>;
                let v1310: Lanes<1>;
                if v157 != 0.0 {
                    let v1413: f64;
                    let v1414: f64;
                    let v1415: f64;
                    let v1416: Lanes<1>;
                    let v1417: Lanes<1>;
                    let v1418: Lanes<1>;
                    if v1301 != 0.0 {
                        let v1360 = v391 * v328;
                        let v1366 = ((v1351 * v341) + (v1152 * (v77 - v341))) - (v1360 * v343);
                        let v1367 = ((v342 * v1351) + ((v342 * v332) * v1152)) - (((v329 * v391) * v343) + (v346 * v1360));
                        let v1368 = v400 * v328;
                        let v1370 = -v1366;
                        let v1376 = (v1370 * v330).exp();
                        let v1381 = (v77 + (v411 * v1376)).sqrt();
                        let v1386 = v372 * (v77 + v1381);
                        let v1388 = v1386.ln();
                        let v1395 = v1366 + (v1368 * v1388);
                        let v1396 = v1367 + (((v329 * v400) * v1388) + (((((((((v1367 * v332) * v330) + (v334 * v1370)) * v1376) * v411) * (v344 / (v416 * v1381))) * v372) * (v344 / v1386)) * v1368));
                        let v1397 = v1303 / v1395;
                        let v1407 = (v1404 * (v1397.ln())).exp();
                        let v1409 = v1302 * v1407;
                        let v1410 = ((((((v1396 * v1397) * v332) / v1395) * (v344 / v1397)) * v1404) * v1407) * v1302;
                        let v1425: f64;
                        let v1426: Lanes<1>;
                        if v1411 != 0.0 {
                            let v1422 = (v1419 * v1395) / v1303;
                            let v1423 = (v1396 * v1419) / v1303;
                            v1425 = v1422;
                            v1426 = v1423;
                        } else {
                            v1425 = v1424;
                            v1426 = v210;
                        }
                        v1413 = v1409;
                        v1414 = v1395;
                        v1415 = v1425;
                        v1416 = v1410;
                        v1417 = v1396;
                        v1418 = v1426;
                    } else {
                        v1413 = v1302;
                        v1414 = v1303;
                        v1415 = v1412;
                        v1416 = v210;
                        v1417 = v210;
                        v1418 = v210;
                    }
                    v1305 = v1413;
                    v1306 = v1414;
                    v1307 = v1415;
                    v1308 = v1416;
                    v1309 = v1417;
                    v1310 = v1418;
                } else {
                    v1305 = v1302;
                    v1306 = v1303;
                    v1307 = v1304;
                    v1308 = v210;
                    v1309 = v210;
                    v1310 = v210;
                }
                let v1314 = (v1311 * v343).exp();
                let v1317 = v1316 * v1314;
                let v1318 = ((v346 * v1311) * v1314) * v1316;
                let v1322 = (v1319 * v343).exp();
                let v1325 = v1324 * v1322;
                let v1326 = ((v346 * v1319) * v1322) * v1324;
                let v1330 = (v1327 * v343).exp();
                let v1333 = v1332 * v1330;
                let v1334 = ((v346 * v1327) * v1330) * v1332;
                let v1338 = (v1335 * v343).exp();
                let v1341 = v1340 * v1338;
                let v1346 = v77 + (v1343 * v336);
                let v1347 = v1341 * v1346;
                let v1350 = ((((v346 * v1335) * v1338) * v1340) * v1346) + ((v326 * v1343) * v1341);
                v211 = v328;
                v212 = v467;
                v213 = v483;
                v214 = v609;
                v215 = v330;
                v216 = v449;
                v217 = v448;
                v218 = v450;
                v219 = v556;
                v220 = v555;
                v221 = v569;
                v222 = v617;
                v223 = v653;
                v224 = v596;
                v225 = v732;
                v226 = v714;
                v227 = v715;
                v228 = v685;
                v229 = v695;
                v230 = v661;
                v231 = v740;
                v232 = v669;
                v233 = v677;
                v234 = v582;
                v235 = v764;
                v236 = v765;
                v237 = v758;
                v238 = v759;
                v239 = v774;
                v240 = v883;
                v241 = v896;
                v242 = v877;
                v243 = v876;
                v244 = v878;
                v245 = v913;
                v246 = v914;
                v247 = v926;
                v248 = v1099;
                v249 = v1081;
                v250 = v1093;
                v251 = v1109;
                v252 = v1096;
                v253 = v1113;
                v254 = v1114;
                v255 = v1115;
                v256 = v1305;
                v257 = v1306;
                v258 = v1307;
                v259 = v1137;
                v260 = v1145;
                v261 = v1130;
                v262 = v1333;
                v263 = v1317;
                v264 = v1325;
                v265 = v1347;
                v266 = v329;
                v267 = v468;
                v268 = v484;
                v269 = v610;
                v270 = v334;
                v271 = v452;
                v272 = v451;
                v273 = v453;
                v274 = v559;
                v275 = v558;
                v276 = v570;
                v277 = v618;
                v278 = v654;
                v279 = v597;
                v280 = v733;
                v281 = v716;
                v282 = v717;
                v283 = v686;
                v284 = v698;
                v285 = v662;
                v286 = v741;
                v287 = v670;
                v288 = v678;
                v289 = v583;
                v290 = v766;
                v291 = v767;
                v292 = v760;
                v293 = v761;
                v294 = v775;
                v295 = v884;
                v296 = v897;
                v297 = v880;
                v298 = v879;
                v299 = v881;
                v300 = v915;
                v301 = v916;
                v302 = v927;
                v303 = v1100;
                v304 = v1084;
                v305 = v1094;
                v306 = v1110;
                v307 = v1097;
                v308 = v1116;
                v309 = v1117;
                v310 = v1118;
                v311 = v1308;
                v312 = v1309;
                v313 = v1310;
                v314 = v1138;
                v315 = v1146;
                v316 = v1131;
                v317 = v1334;
                v318 = v1318;
                v319 = v1326;
                v320 = v1350;
            } else {
                v211 = v164;
                v212 = v165;
                v213 = v166;
                v214 = v167;
                v215 = v168;
                v216 = v142;
                v217 = v169;
                v218 = v170;
                v219 = v88;
                v220 = v84;
                v221 = v171;
                v222 = v172;
                v223 = v173;
                v224 = v174;
                v225 = v175;
                v226 = v176;
                v227 = v177;
                v228 = v178;
                v229 = v179;
                v230 = v180;
                v231 = v181;
                v232 = v182;
                v233 = v183;
                v234 = v184;
                v235 = v78;
                v236 = v79;
                v237 = v185;
                v238 = v186;
                v239 = v187;
                v240 = v188;
                v241 = v189;
                v242 = v119;
                v243 = v117;
                v244 = v190;
                v245 = v115;
                v246 = v116;
                v247 = v191;
                v248 = v192;
                v249 = v193;
                v250 = v194;
                v251 = v195;
                v252 = v196;
                v253 = v197;
                v254 = v198;
                v255 = v199;
                v256 = v200;
                v257 = v201;
                v258 = v202;
                v259 = v203;
                v260 = v204;
                v261 = v205;
                v262 = v206;
                v263 = v207;
                v264 = v208;
                v265 = v209;
                v266 = v210;
                v267 = v210;
                v268 = v210;
                v269 = v210;
                v270 = v210;
                v271 = v210;
                v272 = v210;
                v273 = v210;
                v274 = v210;
                v275 = v210;
                v276 = v210;
                v277 = v210;
                v278 = v210;
                v279 = v210;
                v280 = v210;
                v281 = v210;
                v282 = v210;
                v283 = v210;
                v284 = v210;
                v285 = v210;
                v286 = v210;
                v287 = v210;
                v288 = v210;
                v289 = v210;
                v290 = v210;
                v291 = v210;
                v292 = v210;
                v293 = v210;
                v294 = v210;
                v295 = v210;
                v296 = v210;
                v297 = v210;
                v298 = v210;
                v299 = v210;
                v300 = v210;
                v301 = v210;
                v302 = v210;
                v303 = v210;
                v304 = v210;
                v305 = v210;
                v306 = v210;
                v307 = v210;
                v308 = v210;
                v309 = v210;
                v310 = v210;
                v311 = v210;
                v312 = v210;
                v313 = v210;
                v314 = v210;
                v315 = v210;
                v316 = v210;
                v317 = v210;
                v318 = v210;
                v319 = v210;
                v320 = v210;
            }
            let v1439: f64;
            let v1440: Lanes<3>;
            if v321 != 0.0 {
                let v1428 = v1427 * v211;
                let v1430 = v9 / v1428;
                let v1431 = (v266 * v1427) * v1430;
                let v1435 = ((Lanes([0.0, v10[0], v10[1]])) - (Lanes([v1431[0], 0.0, 0.0]))) / v1428;
                let v1437 = if v1430 > v1436 { 1.0 } else { 0.0 };
                let v1444: f64;
                let v1445: f64;
                let v1446: Lanes<3>;
                let v1447: Lanes<3>;
                if v1437 != 0.0 {
                    let v1443 = v77 + (v1430 - v1436);
                    v1444 = v1443;
                    v1445 = v1436;
                    v1446 = v1435;
                    v1447 = v1438;
                } else {
                    v1444 = v77;
                    v1445 = v1430;
                    v1446 = v1438;
                    v1447 = v1435;
                }
                let v1448 = rspice_limexp(v1445);
                let v1454 = (v1444 * v1448) - v77;
                let v1455 = v212 * v1454;
                let v1456 = v267 * v1454;
                let v1459 = (Lanes([v1456[0], 0.0, 0.0])) + (((v1446 * v1448) + ((v1447 * v1448) * v1444)) * v212);
                v1439 = v1455;
                v1440 = v1459;
            } else {
                v1439 = v63;
                v1440 = v1438;
            }
            let v1469: f64;
            let v1470: Lanes<3>;
            if v1441 != 0.0 {
                let v1460 = v475 * v211;
                let v1462 = v9 / v1460;
                let v1463 = (v266 * v475) * v1462;
                let v1467 = ((Lanes([0.0, v10[0], v10[1]])) - (Lanes([v1463[0], 0.0, 0.0]))) / v1460;
                let v1468 = if v1462 > v1436 { 1.0 } else { 0.0 };
                let v1503: f64;
                let v1504: f64;
                let v1505: Lanes<3>;
                let v1506: Lanes<3>;
                if v1468 != 0.0 {
                    let v1502 = v77 + (v1462 - v1436);
                    v1503 = v1502;
                    v1504 = v1436;
                    v1505 = v1467;
                    v1506 = v1438;
                } else {
                    v1503 = v77;
                    v1504 = v1462;
                    v1505 = v1438;
                    v1506 = v1467;
                }
                let v1507 = rspice_limexp(v1504);
                let v1513 = (v1503 * v1507) - v77;
                let v1514 = v213 * v1513;
                let v1515 = v268 * v1513;
                let v1518 = (Lanes([v1515[0], 0.0, 0.0])) + (((v1505 * v1507) + ((v1506 * v1507) * v1503)) * v213);
                v1469 = v1514;
                v1470 = v1518;
            } else {
                v1469 = v63;
                v1470 = v1438;
            }
            let v1472 = v10 * v215;
            let v1473 = v270 * v9;
            let v1480 = rspice_limexp(((v9 * v215) / v1477));
            let v1482 = v214 * v1480;
            let v1483 = v269 * v1480;
            let v1486 = (Lanes([v1483[0], 0.0, 0.0])) + (((((Lanes([0.0, v1472[0], v1472[1]])) + (Lanes([v1473[0], 0.0, 0.0]))) / v1477) * v1480) * v214);
            let v1488 = v18 * v215;
            let v1489 = v270 * v17;
            let v1493 = rspice_limexp((v17 * v215));
            let v1495 = v214 * v1493;
            let v1496 = v269 * v1493;
            let v1499 = (Lanes([v1496[0], 0.0, 0.0])) + ((((Lanes([0.0, v1488[0], v1488[1]])) + (Lanes([v1489[0], 0.0, 0.0]))) * v1493) * v214);
            let v1500 = if v216 > v63 { 1.0 } else { 0.0 };
            let v1628: f64;
            let v1629: f64;
            let v1630: Lanes<3>;
            let v1631: Lanes<3>;
            if v1500 != 0.0 {
                let v1526 = ((-(v218.ln())) / v439).exp();
                let v1528 = v77 - v1526;
                let v1530 = v217 * v1528;
                let v1533 = (v272 * v1528) + ((((((v273 * (v344 / v218)) * v332) / v439) * v1526) * v332) * v217);
                let v1534 = v1530 - v9;
                let v1535 = Lanes([v1533[0], 0.0, 0.0]);
                let v1536 = Lanes([0.0, v10[0], v10[1]]);
                let v1538 = v1534 * v215;
                let v1540 = v270 * v1534;
                let v1542 = ((v1535 - v1536) * v215) + (Lanes([v1540[0], 0.0, 0.0]));
                let v1544 = v1542 * v1538;
                let v1548 = ((v1538 * v1538) + v1546).sqrt();
                let v1551 = (v1544 + v1544) * (v344 / (v416 * v1548));
                let v1554 = (v1538 + v1548) * v372;
                let v1555 = (v1542 + v1551) * v372;
                let v1557 = v266 * v1554;
                let v1561 = v1530 - (v211 * v1554);
                let v1562 = v1535 - ((Lanes([v1557[0], 0.0, 0.0])) + (v1555 * v211));
                let v1563 = v1554 / v1548;
                let v1566 = (v1555 - (v1551 * v1563)) / v1548;
                let v1567 = v1561 / v217;
                let v1568 = v272 * v1567;
                let v1572 = v77 - v1567;
                let v1574 = v1572.ln();
                let v1576 = (((v1562 - (Lanes([v1568[0], 0.0, 0.0]))) / v217) * v332) * (v344 / v1572);
                let v1577 = -v439;
                let v1580 = (v1577 * v1574).exp();
                let v1586 = v77 - v1563;
                let v1589 = v273 * v1586;
                let v1593 = (v1580 * v1563) + (v218 * v1586);
                let v1595 = v216 * v1593;
                let v1596 = v271 * v1593;
                let v1599 = (Lanes([v1596[0], 0.0, 0.0])) + ((((((v1576 * v1577) * v1580) * v1563) + (v1566 * v1580)) + ((Lanes([v1589[0], 0.0, 0.0])) + ((v1566 * v332) * v218))) * v216);
                let v1600 = v77 - v439;
                let v1603 = (v1574 * v1600).exp();
                let v1605 = v77 - v1603;
                let v1608 = v272 * v1605;
                let v1614 = v9 - v1561;
                let v1617 = v273 * v1614;
                let v1621 = ((v217 * v1605) / v1600) + (v218 * v1614);
                let v1623 = v216 * v1621;
                let v1624 = v271 * v1621;
                let v1627 = (Lanes([v1624[0], 0.0, 0.0])) + (((((Lanes([v1608[0], 0.0, 0.0])) + ((((v1576 * v1600) * v1603) * v332) * v217)) / v1600) + ((Lanes([v1617[0], 0.0, 0.0])) + ((v1536 - v1562) * v218))) * v216);
                v1628 = v1623;
                v1629 = v1595;
                v1630 = v1627;
                v1631 = v1599;
            } else {
                v1628 = v63;
                v1629 = v63;
                v1630 = v1438;
                v1631 = v1438;
            }
            let v1635: f64;
            let v1636: f64;
            let v1637: Lanes<3>;
            let v1638: Lanes<3>;
            if v1632 != 0.0 {
                let v1633 = if v219 > v63 { 1.0 } else { 0.0 };
                let v1690: f64;
                let v1691: f64;
                let v1692: Lanes<3>;
                let v1693: Lanes<3>;
                if v1633 != 0.0 {
                    let v1640 = v546 / v411;
                    let v1642 = v1641 - v220;
                    let v1643 = v275 * v332;
                    let v1651 = ((-(v221.ln())) / v546).exp();
                    let v1653 = v77 - v1651;
                    let v1655 = v220 * v1653;
                    let v1658 = (v275 * v1653) + ((((((v276 * (v344 / v221)) * v332) / v546) * v1651) * v332) * v220);
                    let v1659 = v221 * v219;
                    let v1662 = (v276 * v219) + (v274 * v221);
                    let v1663 = v1640 - v546;
                    let v1664 = v1641 / v220;
                    let v1673 = (v1663 * (v1664.ln())).exp();
                    let v1675 = v219 * v1673;
                    let v1678 = (v274 * v1673) + (((((((v275 * v1664) * v332) / v220) * (v344 / v1664)) * v1663) * v1673) * v219);
                    let v1679 = v1655 - v17;
                    let v1680 = Lanes([v1658[0], 0.0, 0.0]);
                    let v1681 = Lanes([0.0, v18[0], v18[1]]);
                    let v1683 = v1679 * v215;
                    let v1685 = v270 * v1679;
                    let v1687 = ((v1680 - v1681) * v215) + (Lanes([v1685[0], 0.0, 0.0]));
                    let v1688 = if v1683 < v1436 { 1.0 } else { 0.0 };
                    let v1711: f64;
                    let v1712: f64;
                    let v1713: Lanes<3>;
                    let v1714: Lanes<3>;
                    if v1688 != 0.0 {
                        let v1694 = v1683.exp();
                        let v1695 = v1687 * v1694;
                        let v1696 = v77 + v1694;
                        let v1697 = v1694 / v1696;
                        let v1700 = (v1695 - (v1695 * v1697)) / v1696;
                        let v1701 = v1696.ln();
                        let v1705 = v266 * v1701;
                        let v1709 = v1655 - (v211 * v1701);
                        let v1710 = v1680 - ((Lanes([v1705[0], 0.0, 0.0])) + ((v1695 * (v344 / v1696)) * v211));
                        v1711 = v1709;
                        v1712 = v1697;
                        v1713 = v1710;
                        v1714 = v1700;
                    } else {
                        v1711 = v17;
                        v1712 = v77;
                        v1713 = v1681;
                        v1714 = v1689;
                    }
                    let v1720 = (v1715 * v1642) + (v411 * v211);
                    let v1721 = (v1643 * v1715) + (v266 * v411);
                    let v1725 = (v1642 + v1711) / v1720;
                    let v1726 = v1721 * v1725;
                    let v1729 = (((Lanes([v1643[0], 0.0, 0.0])) + v1713) - (Lanes([v1726[0], 0.0, 0.0]))) / v1720;
                    let v1730 = if v1725 < v1436 { 1.0 } else { 0.0 };
                    let v1764: f64;
                    let v1765: f64;
                    let v1766: Lanes<3>;
                    let v1767: Lanes<3>;
                    if v1730 != 0.0 {
                        let v1731 = v1725.exp();
                        let v1732 = v1729 * v1731;
                        let v1733 = v77 + v1731;
                        let v1734 = v1731 / v1733;
                        let v1737 = (v1732 - (v1732 * v1734)) / v1733;
                        let v1739 = v1643 * v332;
                        let v1747 = (-(v1642 + v1655)) / v1720;
                        let v1751 = v1747.exp();
                        let v1752 = ((((v1643 + v1658) * v332) - (v1721 * v1747)) / v1720) * v1751;
                        let v1753 = (v1733.ln()) - v1751;
                        let v1757 = v1721 * v1753;
                        let v1761 = (-v1642) + (v1720 * v1753);
                        let v1763 = (Lanes([v1739[0], 0.0, 0.0])) + ((Lanes([v1757[0], 0.0, 0.0])) + (((v1732 * (v344 / v1733)) - (Lanes([v1752[0], 0.0, 0.0]))) * v1720));
                        v1764 = v1761;
                        v1765 = v1734;
                        v1766 = v1763;
                        v1767 = v1737;
                    } else {
                        v1764 = v1711;
                        v1765 = v77;
                        v1766 = v1713;
                        v1767 = v1689;
                    }
                    let v1768 = v17 - v1711;
                    let v1770 = v1711 / v220;
                    let v1771 = v275 * v1770;
                    let v1775 = v77 - v1770;
                    let v1777 = v1775.ln();
                    let v1779 = (((v1713 - (Lanes([v1771[0], 0.0, 0.0]))) / v220) * v332) * (v344 / v1775);
                    let v1780 = v1764 / v220;
                    let v1781 = v275 * v1780;
                    let v1785 = v77 - v1780;
                    let v1787 = v1785.ln();
                    let v1789 = (((v1766 - (Lanes([v1781[0], 0.0, 0.0]))) / v220) * v332) * (v344 / v1785);
                    let v1790 = v77 - v546;
                    let v1791 = v77 - v1640;
                    let v1792 = -v546;
                    let v1795 = (v1787 * v1792).exp();
                    let v1797 = v219 * v1795;
                    let v1798 = v274 * v1795;
                    let v1802 = v1797 * v1712;
                    let v1810 = -v1640;
                    let v1813 = (v1777 * v1810).exp();
                    let v1815 = v1675 * v1813;
                    let v1816 = v1678 * v1813;
                    let v1820 = v77 - v1765;
                    let v1826 = v77 - v1712;
                    let v1829 = v1662 * v1826;
                    let v1835 = ((v1802 * v1765) + (v1815 * v1820)) + (v1659 * v1826);
                    let v1836 = (((((((Lanes([v1798[0], 0.0, 0.0])) + (((v1789 * v1792) * v1795) * v219)) * v1712) + (v1714 * v1797)) * v1765) + (v1767 * v1802)) + ((((Lanes([v1816[0], 0.0, 0.0])) + (((v1779 * v1810) * v1813) * v1675)) * v1820) + ((v1767 * v332) * v1815))) + ((Lanes([v1829[0], 0.0, 0.0])) + ((v1714 * v332) * v1659));
                    let v1839 = (v1787 * v1790).exp();
                    let v1841 = v77 - v1839;
                    let v1844 = v274 * v1841;
                    let v1852 = (v1777 * v1791).exp();
                    let v1854 = v77 - v1852;
                    let v1857 = v1678 * v1854;
                    let v1865 = (v1787 * v1791).exp();
                    let v1867 = v77 - v1865;
                    let v1870 = v1678 * v1867;
                    let v1878 = (((v219 * v1841) / v1790) + ((v1675 * v1854) / v1791)) - ((v1675 * v1867) / v1791);
                    let v1882 = v275 * v1878;
                    let v1886 = v1662 * v1768;
                    let v1890 = (v1878 * v220) + (v1659 * v1768);
                    let v1891 = (((((((Lanes([v1844[0], 0.0, 0.0])) + ((((v1789 * v1790) * v1839) * v332) * v219)) / v1790) + (((Lanes([v1857[0], 0.0, 0.0])) + ((((v1779 * v1791) * v1852) * v332) * v1675)) / v1791)) - (((Lanes([v1870[0], 0.0, 0.0])) + ((((v1789 * v1791) * v1865) * v332) * v1675)) / v1791)) * v220) + (Lanes([v1882[0], 0.0, 0.0]))) + ((Lanes([v1886[0], 0.0, 0.0])) + ((v1681 - v1713) * v1659));
                    v1690 = v1890;
                    v1691 = v1835;
                    v1692 = v1891;
                    v1693 = v1836;
                } else {
                    v1690 = v63;
                    v1691 = v63;
                    v1692 = v1689;
                    v1693 = v1689;
                }
                v1635 = v1690;
                v1636 = v1691;
                v1637 = v1692;
                v1638 = v1693;
            } else {
                let v1634 = if v219 > v63 { 1.0 } else { 0.0 };
                let v2000: f64;
                let v2001: f64;
                let v2002: Lanes<3>;
                let v2003: Lanes<3>;
                if v1634 != 0.0 {
                    let v1899 = ((-(v221.ln())) / v546).exp();
                    let v1901 = v77 - v1899;
                    let v1903 = v220 * v1901;
                    let v1906 = (v275 * v1901) + ((((((v276 * (v344 / v221)) * v332) / v546) * v1899) * v332) * v220);
                    let v1907 = v1903 - v17;
                    let v1908 = Lanes([v1906[0], 0.0, 0.0]);
                    let v1909 = Lanes([0.0, v18[0], v18[1]]);
                    let v1911 = v1907 * v215;
                    let v1913 = v270 * v1907;
                    let v1915 = ((v1908 - v1909) * v215) + (Lanes([v1913[0], 0.0, 0.0]));
                    let v1917 = v1915 * v1911;
                    let v1920 = ((v1911 * v1911) + v1546).sqrt();
                    let v1923 = (v1917 + v1917) * (v344 / (v416 * v1920));
                    let v1926 = (v1911 + v1920) * v372;
                    let v1927 = (v1915 + v1923) * v372;
                    let v1929 = v266 * v1926;
                    let v1933 = v1903 - (v211 * v1926);
                    let v1934 = v1908 - ((Lanes([v1929[0], 0.0, 0.0])) + (v1927 * v211));
                    let v1935 = v1926 / v1920;
                    let v1938 = (v1927 - (v1923 * v1935)) / v1920;
                    let v1939 = v1933 / v220;
                    let v1940 = v275 * v1939;
                    let v1944 = v77 - v1939;
                    let v1946 = v1944.ln();
                    let v1948 = (((v1934 - (Lanes([v1940[0], 0.0, 0.0]))) / v220) * v332) * (v344 / v1944);
                    let v1949 = -v546;
                    let v1952 = (v1949 * v1946).exp();
                    let v1958 = v77 - v1935;
                    let v1961 = v276 * v1958;
                    let v1965 = (v1952 * v1935) + (v221 * v1958);
                    let v1967 = v219 * v1965;
                    let v1968 = v274 * v1965;
                    let v1971 = (Lanes([v1968[0], 0.0, 0.0])) + ((((((v1948 * v1949) * v1952) * v1935) + (v1938 * v1952)) + ((Lanes([v1961[0], 0.0, 0.0])) + ((v1938 * v332) * v221))) * v219);
                    let v1972 = v77 - v546;
                    let v1975 = (v1946 * v1972).exp();
                    let v1977 = v77 - v1975;
                    let v1980 = v275 * v1977;
                    let v1986 = v17 - v1933;
                    let v1989 = v276 * v1986;
                    let v1993 = ((v220 * v1977) / v1972) + (v221 * v1986);
                    let v1995 = v219 * v1993;
                    let v1996 = v274 * v1993;
                    let v1999 = (Lanes([v1996[0], 0.0, 0.0])) + (((((Lanes([v1980[0], 0.0, 0.0])) + ((((v1948 * v1972) * v1975) * v332) * v220)) / v1972) + ((Lanes([v1989[0], 0.0, 0.0])) + ((v1909 - v1934) * v221))) * v219);
                    v2000 = v1995;
                    v2001 = v1967;
                    v2002 = v1999;
                    v2003 = v1971;
                } else {
                    v2000 = v63;
                    v2001 = v63;
                    v2002 = v1689;
                    v2003 = v1689;
                }
                v1635 = v2000;
                v1636 = v2001;
                v1637 = v2002;
                v1638 = v2003;
            }
            let v2060: f64;
            let v2061: Lanes<3>;
            if v1639 != 0.0 {
                let v2005 = v2004 * v211;
                let v2006 = v266 * v2004;
                let v2008 = Lanes([v272[0], 0.0, 0.0]);
                let v2011 = (v217 - v9) / v2005;
                let v2012 = v2006 * v2011;
                let v2015 = ((v2008 - (Lanes([0.0, v10[0], v10[1]]))) - (Lanes([v2012[0], 0.0, 0.0]))) / v2005;
                let v2017 = v2015 * v2011;
                let v2020 = ((v2011 * v2011) + v1546).sqrt();
                let v2024 = v2011 + v2020;
                let v2027 = v2006 * v2024;
                let v2035 = (v217 - ((v2005 * v2024) * v372)) / v217;
                let v2036 = v272 * v2035;
                let v2040 = v77 - v2035;
                let v2047 = (v439 * (v2040.ln())).exp();
                let v2049 = v77 - v2047;
                let v2051 = v222 * v2049;
                let v2052 = v277 * v2049;
                let v2055 = (Lanes([v2052[0], 0.0, 0.0])) + (((((((((v2008 - (((Lanes([v2027[0], 0.0, 0.0])) + ((v2015 + ((v2017 + v2017) * (v344 / (v416 * v2020)))) * v2005)) * v372)) - (Lanes([v2036[0], 0.0, 0.0]))) / v217) * v332) * (v344 / v2040)) * v439) * v2047) * v332) * v222);
                let v2058 = if (v2051.abs()) > v2057 { 1.0 } else { 0.0 };
                let v2200: f64;
                let v2201: Lanes<3>;
                if v2058 != 0.0 {
                    let v2180 = v2051.exp();
                    let v2182 = v2180 - v77;
                    let v2184 = v278 * v2182;
                    let v2188 = (v223 * v2182) / v2051;
                    let v2191 = (((Lanes([v2184[0], 0.0, 0.0])) + ((v2055 * v2180) * v223)) - (v2055 * v2188)) / v2051;
                    v2200 = v2188;
                    v2201 = v2191;
                } else {
                    let v2194 = v77 + (v2051 * v372);
                    let v2195 = v223 * v2194;
                    let v2196 = v278 * v2194;
                    let v2199 = (Lanes([v2196[0], 0.0, 0.0])) + ((v2055 * v372) * v223);
                    v2200 = v2195;
                    v2201 = v2199;
                }
                v2060 = v2200;
                v2061 = v2201;
            } else {
                let v2059 = Lanes([v278[0], 0.0, 0.0]);
                v2060 = v223;
                v2061 = v2059;
            }
            let v2068 = (Lanes([v279[0], 0.0, 0.0])) + ((v2061 * v1628) + (v1630 * v2060));
            let v2071 = v1637 * v2069;
            let v2077 = v2076 * v224;
            let v2078 = v279 * v2076;
            let v2079 = ((v224 + (v2060 * v1628)) + (v2069 * v1635)) / v2077;
            let v2080 = v2078 * v2079;
            let v2083 = (((Lanes([v2068[0], 0.0, v2068[1], v2068[2]])) + (Lanes([v2071[0], v2071[1], 0.0, v2071[2]]))) - (Lanes([v2080[0], 0.0, 0.0, 0.0]))) / v2077;
            let v2084 = v2079 - v77;
            let v2086 = v2083 * v2084;
            let v2089 = ((v2084 * v2084) + v1546).sqrt();
            let v2097 = v77 + ((v2084 + v2089) * v372);
            let v2098 = v2077 * v2097;
            let v2099 = v2078 * v2097;
            let v2102 = (Lanes([v2099[0], 0.0, 0.0, 0.0])) + (((v2083 + ((v2086 + v2086) * (v344 / (v416 * v2089)))) * v372) * v2077);
            let v2104 = v220 * v2103;
            let v2105 = v275 * v2103;
            let v2106 = v2104 - v17;
            let v2107 = Lanes([v2105[0], 0.0, 0.0]);
            let v2108 = Lanes([0.0, v18[0], v18[1]]);
            let v2110 = v2106 * v215;
            let v2112 = v270 * v2106;
            let v2114 = ((v2107 - v2108) * v215) + (Lanes([v2112[0], 0.0, 0.0]));
            let v2116 = v2114 * v2110;
            let v2119 = ((v2110 * v2110) + v1546).sqrt();
            let v2122 = (v2116 + v2116) * (v344 / (v416 * v2119));
            let v2125 = (v2110 + v2119) * v372;
            let v2126 = (v2114 + v2122) * v372;
            let v2128 = v266 * v2125;
            let v2134 = v2125 / v2119;
            let v2137 = (v2126 - (v2122 * v2134)) / v2119;
            let v2138 = (v2104 - (v211 * v2125)) / v220;
            let v2139 = v275 * v2138;
            let v2143 = v77 - v2138;
            let v2151 = (v2148 * (v2143.ln())).exp();
            let v2161 = (v2151 * v2134) + (v568 * (v77 - v2134));
            let v2162 = (((((((((v2107 - ((Lanes([v2128[0], 0.0, 0.0])) + (v2126 * v211))) - (Lanes([v2139[0], 0.0, 0.0]))) / v220) * v332) * (v344 / v2143)) * v2148) * v2151) * v2134) + (v2137 * v2151)) + ((v2137 * v332) * v568);
            let v2163 = v77 / v2161;
            let v2178 = (v225 + (v2168 * (v2163 - v77))) + (v2175 * (v2161 - v77));
            let v2179 = ((Lanes([v280[0], 0.0, 0.0])) + ((((v2162 * v2163) * v332) / v2161) * v2168)) + (v2162 * v2175);
            let v2210: f64;
            let v2211: Lanes<4>;
            if v69 != 0.0 {
                let v2202 = v226 - v17;
                let v2204 = (Lanes([v281[0], 0.0, 0.0])) - v2108;
                let v2205 = Lanes([v2204[0], v2204[1], 0.0, v2204[2]]);
                v2210 = v2202;
                v2211 = v2205;
            } else {
                let v2206 = v19 - v227;
                let v2209 = (Lanes([0.0, v22[0], v22[1], v22[2]])) - (Lanes([v282[0], 0.0, 0.0, 0.0]));
                v2210 = v2206;
                v2211 = v2209;
            }
            let v2257: f64;
            let v2258: Lanes<4>;
            if v65 != 0.0 {
                let v2212 = v2210 - v211;
                let v2213 = Lanes([v266[0], 0.0, 0.0, 0.0]);
                let v2215 = v2212 * v215;
                let v2217 = v270 * v2212;
                let v2219 = ((v2211 - v2213) * v215) + (Lanes([v2217[0], 0.0, 0.0, 0.0]));
                let v2221 = v2219 * v2215;
                let v2224 = ((v2215 * v2215) + v1546).sqrt();
                let v2230 = (v2215 + v2224) * v372;
                let v2233 = v266 * v2230;
                let v2237 = v211 + (v211 * v2230);
                let v2238 = v2213 + ((Lanes([v2233[0], 0.0, 0.0, 0.0])) + (((v2219 + ((v2221 + v2221) * (v344 / (v416 * v2224)))) * v372) * v211));
                v2257 = v2237;
                v2258 = v2238;
            } else {
                let v2240 = v2210 / v2239;
                let v2241 = v2211 / v2239;
                let v2243 = v2241 * v2240;
                let v2247 = ((v2240 * v2240) + v2245).sqrt();
                let v2255 = v2239 * ((v2240 + v2247) * v372);
                let v2256 = ((v2241 + ((v2243 + v2243) * (v344 / (v416 * v2247)))) * v372) * v2239;
                v2257 = v2255;
                v2258 = v2256;
            }
            let v2259 = v2257 / v228;
            let v2260 = v283 * v2259;
            let v2266 = v284 * v2257;
            let v2275 = (v2272 * (v2259.ln())).exp();
            let v2277 = v77 + v2275;
            let v2283 = ((v2277.ln()) / v2272).exp();
            let v2285 = (v2257 * v229) / v2283;
            let v2293 = (v2257 - v228) / v2292;
            let v2294 = (v2258 - (Lanes([v283[0], 0.0, 0.0, 0.0]))) / v2292;
            let v2296 = v2294 * v2293;
            let v2300 = ((v2293 * v2293) + v2298).sqrt();
            let v2308 = v77 + (v372 * (v2293 + v2300));
            let v2309 = v2285 * v2308;
            let v2312 = (((((v2258 * v229) + (Lanes([v2266[0], 0.0, 0.0, 0.0]))) - (((((((((v2258 - (Lanes([v2260[0], 0.0, 0.0, 0.0]))) / v228) * (v344 / v2259)) * v2272) * v2275) * (v344 / v2277)) / v2272) * v2283) * v2285)) / v2283) * v2308) + (((v2294 + ((v2296 + v2296) * (v344 / (v416 * v2300)))) * v372) * v2285);
            let v2315 = if (if v2178 > v63 { 1.0 } else { 0.0 }) != 0.0 || v2314 != 0.0 { 1.0 } else { 0.0 };
            let v2318: f64;
            let v2319: Lanes<4>;
            if v2315 != 0.0 {
                let v2316 = v372 * v2098;
                let v2317 = v2102 * v372;
                let v2385: f64;
                let v2386: Lanes<4>;
                if v65 != 0.0 {
                    let v2337 = v2317 * v2316;
                    let v2340 = v2179 * v1482;
                    let v2341 = v1486 * v2178;
                    let v2349 = v1499 * v2347;
                    let v2353 = (((v2316 * v2316) + (v2178 * v1482)) + (v2347 * v1495)).sqrt();
                    let v2357 = v2316 + v2353;
                    let v2358 = v2317 + ((((v2337 + v2337) + ((Lanes([v2340[0], v2340[1], 0.0, v2340[2]])) + (Lanes([v2341[0], 0.0, v2341[1], v2341[2]])))) + (Lanes([v2349[0], v2349[1], 0.0, v2349[2]]))) * (v344 / (v416 * v2353)));
                    v2385 = v2357;
                    v2386 = v2358;
                } else {
                    let v2360 = v2317 * v2316;
                    let v2362 = v230 * v225;
                    let v2367 = ((v285 * v225) + (v280 * v230)) * v1482;
                    let v2370 = (Lanes([v2367[0], 0.0, 0.0])) + (v1486 * v2362);
                    let v2375 = v1499 * v2347;
                    let v2379 = (((v2316 * v2316) + (v2362 * v1482)) + (v2347 * v1495)).sqrt();
                    let v2383 = v2316 + v2379;
                    let v2384 = v2317 + ((((v2360 + v2360) + (Lanes([v2370[0], 0.0, v2370[1], v2370[2]]))) + (Lanes([v2375[0], v2375[1], 0.0, v2375[2]]))) * (v344 / (v416 * v2379)));
                    v2385 = v2383;
                    v2386 = v2384;
                }
                v2318 = v2385;
                v2319 = v2386;
            } else {
                v2318 = v2098;
                v2319 = v2102;
            }
            let v2320 = v1482 / v2318;
            let v2322 = Lanes([v1486[0], 0.0, v1486[1], v1486[2]]);
            let v2324 = (v2322 - (v2319 * v2320)) / v2318;
            let v2325 = v1495 / v2318;
            let v2327 = Lanes([v1499[0], v1499[1], 0.0, v1499[2]]);
            let v2329 = (v2327 - (v2319 * v2325)) / v2318;
            let v2330 = v2178 * v2320;
            let v2331 = v2179 * v2320;
            let v2334 = (Lanes([v2331[0], v2331[1], 0.0, v2331[2]])) + (v2324 * v2178);
            let v2401: f64;
            let v2402: Lanes<4>;
            if v2335 != 0.0 {
                let v2387 = v230 * v225;
                let v2391 = v2387 * v2320;
                let v2392 = ((v285 * v225) + (v280 * v230)) * v2320;
                let v2395 = (Lanes([v2392[0], 0.0, 0.0, 0.0])) + (v2324 * v2387);
                v2401 = v2391;
                v2402 = v2395;
            } else {
                let v2396 = v230 * v2330;
                let v2397 = v285 * v2330;
                let v2400 = (Lanes([v2397[0], 0.0, 0.0, 0.0])) + (v2334 * v230);
                v2401 = v2396;
                v2402 = v2400;
            }
            let v2404 = v2403 * v2309;
            let v2407 = if (if v2320 >= v2404 { 1.0 } else { 0.0 }) != 0.0 || v2406 != 0.0 { 1.0 } else { 0.0 };
            let v2436: f64;
            let v2437: f64;
            let v2438: f64;
            let v2439: f64;
            let v2440: Lanes<4>;
            let v2441: Lanes<4>;
            let v2442: Lanes<4>;
            let v2443: Lanes<4>;
            if v2407 != 0.0 {
                let v2408 = v2320 / v2309;
                let v2411 = (v2324 - (v2312 * v2408)) / v2309;
                let v2418 = (v2415 * (v2408.ln())).exp();
                let v2421 = v2420 * v2418;
                let v2422 = (((v2411 * (v344 / v2408)) * v2415) * v2418) * v2420;
                let v2427 = v77 + v2415;
                let v2428 = (v2421 * v2320) / v2427;
                let v2429 = ((v2422 * v2320) + (v2324 * v2421)) / v2427;
                let v2433 = if v2432 < (v2076 * (v684 / v692)) { 1.0 } else { 0.0 };
                let v2462: f64;
                let v2463: f64;
                let v2464: Lanes<4>;
                let v2465: Lanes<4>;
                if v2433 != 0.0 {
                    v2462 = v63;
                    v2463 = v63;
                    v2464 = v2435;
                    v2465 = v2435;
                } else {
                    let v2458 = (v2320 - v2309) / v2432;
                    let v2459 = (v2324 - v2312) / v2432;
                    let v2461 = if v2458 < v2460 { 1.0 } else { 0.0 };
                    let v2584: f64;
                    let v2585: Lanes<4>;
                    if v2461 != 0.0 {
                        v2584 = v2583;
                        v2585 = v2435;
                    } else {
                        v2584 = v2458;
                        v2585 = v2459;
                    }
                    let v2587 = v2585 * v2584;
                    let v2591 = ((v2584 * v2584) + v2589).sqrt();
                    let v2594 = (v2587 + v2587) * (v344 / (v416 * v2591));
                    let v2595 = v2584 + v2591;
                    let v2596 = v2585 + v2594;
                    let v2598 = v2597 / v2595;
                    let v2602 = v2598.exp();
                    let v2604 = v2525 * v2602;
                    let v2605 = ((((v2596 * v2598) * v332) / v2595) * v2602) * v2525;
                    let v2608 = v2432 * v2591;
                    let v2610 = v2608 * v2595;
                    let v2614 = (v400 * v2604) / v2610;
                    let v2617 = ((v2605 * v400) - ((((v2594 * v2432) * v2595) + (v2596 * v2608)) * v2614)) / v2610;
                    v2462 = v2604;
                    v2463 = v2614;
                    v2464 = v2605;
                    v2465 = v2617;
                }
                let v2467 = v77 - v2466;
                let v2468 = v2467 * v231;
                let v2469 = v286 * v2467;
                let v2471 = v2464 * v215;
                let v2472 = v270 * v2462;
                let v2475 = (v2462 * v215).exp();
                let v2476 = (v2471 + (Lanes([v2472[0], 0.0, 0.0, 0.0]))) * v2475;
                let v2477 = v2475 - v77;
                let v2478 = v2468 * v2477;
                let v2479 = v2469 * v2477;
                let v2482 = (Lanes([v2479[0], 0.0, 0.0, 0.0])) + (v2476 * v2468);
                let v2483 = v2468 * v2320;
                let v2484 = v2469 * v2320;
                let v2488 = v2483 * v2475;
                let v2492 = v2488 * v215;
                let v2494 = v270 * v2488;
                let v2501 = v2478 + (v2492 * v2463);
                let v2502 = v2482 + ((((((((Lanes([v2484[0], 0.0, 0.0, 0.0])) + (v2324 * v2468)) * v2475) + (v2476 * v2483)) * v215) + (Lanes([v2494[0], 0.0, 0.0, 0.0]))) * v2463) + (v2465 * v2492));
                let v2503 = v77 / v2408;
                let v2507 = v77 - v2503;
                let v2508 = (((v2411 * v2503) * v332) / v2408) * v332;
                let v2510 = v2508 * v2507;
                let v2514 = ((v2507 * v2507) + v2512).sqrt();
                let v2517 = (v2510 + v2510) * (v344 / (v416 * v2514));
                let v2522 = v77 + ((v77 + v2512).sqrt());
                let v2523 = (v2507 + v2514) / v2522;
                let v2524 = (v2508 + v2517) / v2522;
                let v2526 = v2462 - v2525;
                let v2528 = v270 * v2526;
                let v2531 = (v2526 * v215).exp();
                let v2532 = (v2471 + (Lanes([v2528[0], 0.0, 0.0, 0.0]))) * v2531;
                let v2533 = v231 * v2523;
                let v2534 = v286 * v2523;
                let v2538 = v2533 * v2523;
                let v2542 = v2538 * v2531;
                let v2545 = (((((Lanes([v2534[0], 0.0, 0.0, 0.0])) + (v2524 * v231)) * v2523) + (v2524 * v2533)) * v2531) + (v2532 * v2538);
                let v2546 = v2408 * v2514;
                let v2550 = v400 / v2546;
                let v2555 = v215 * v2320;
                let v2556 = v270 * v2320;
                let v2564 = (v77 + v2550) + (v2555 * v2463);
                let v2566 = v2542 * v2564;
                let v2569 = (v2545 * v2564) + (((((((v2411 * v2514) + (v2517 * v2408)) * v2550) * v332) / v2546) + ((((Lanes([v2556[0], 0.0, 0.0, 0.0])) + (v2324 * v215)) * v2463) + (v2465 * v2555))) * v2542);
                let v2582 = if (if (if (if v2570 < v2571 { 1.0 } else { 0.0 }) != 0.0 && (if v2573 < v2571 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (v2523 * v2570) < v2577 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (v2523 * v2573) < v2577 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v2646: f64;
                let v2647: f64;
                let v2648: Lanes<4>;
                let v2649: Lanes<4>;
                if v2582 != 0.0 {
                    let v2618 = v2466 * v2542;
                    let v2620 = v2618 * v2320;
                    let v2623 = ((v2545 * v2466) * v2320) + (v2324 * v2618);
                    let v2624 = v2466 * v2566;
                    let v2625 = v2569 * v2466;
                    v2646 = v2620;
                    v2647 = v2624;
                    v2648 = v2623;
                    v2649 = v2625;
                } else {
                    let v2626 = v77 - v2523;
                    let v2627 = v2524 * v332;
                    let v2628 = v2626 - v77;
                    let v2629 = v77 - v2507;
                    let v2635 = v2514 * v2320;
                    let v2639 = (v2628 * v2629) / v2635;
                    let v2642 = (((v2627 * v2629) + ((v2508 * v332) * v2628)) - (((v2517 * v2320) + (v2324 * v2514)) * v2639)) / v2635;
                    let v2645 = if (v2643.abs()) > v2057 { 1.0 } else { 0.0 };
                    let v2725: f64;
                    let v2726: f64;
                    let v2727: Lanes<4>;
                    let v2728: Lanes<4>;
                    if v2645 != 0.0 {
                        let v2667 = (v2628 * v2664).exp();
                        let v2668 = (v2627 * v2664) * v2667;
                        let v2670 = if v2669 < v2571 { 1.0 } else { 0.0 };
                        let v2922: f64;
                        let v2923: f64;
                        let v2924: Lanes<4>;
                        let v2925: Lanes<4>;
                        if v2670 != 0.0 {
                            let v2768 = v2667 * v2767;
                            let v2769 = v2668 * v2767;
                            let v2770 = (v77 - v2667) / v2768;
                            let v2773 = ((v2668 * v332) - (v2769 * v2770)) / v2768;
                            let v2774 = v2767 * v2770;
                            let v2775 = v2773 * v2767;
                            let v2776 = v77 + v2774;
                            let v2778 = v2777 * v2767;
                            let v2781 = v372 + (v2778 * v2770);
                            let v2797 = ((v400 * ((v2774 * v2781) - (v372 * (v2776.ln())))) / v2767) / v2767;
                            let v2798 = (((((v2775 * v2781) + ((v2773 * v2778) * v2774)) - ((v2775 * (v344 / v2776)) * v372)) * v400) / v2767) / v2767;
                            let v2799 = -v2664;
                            let v2802 = (v2799 * v2639) / v2768;
                            let v2806 = v77 + v2776;
                            let v2807 = v2806 * v2770;
                            let v2815 = (v2807 * v2802) / v2776;
                            let v2818 = (((((v2775 * v2770) + (v2773 * v2806)) * v2802) + ((((v2642 * v2799) - (v2769 * v2802)) / v2768) * v2807)) - (v2775 * v2815)) / v2776;
                            v2922 = v2797;
                            v2923 = v2815;
                            v2924 = v2798;
                            v2925 = v2818;
                        } else {
                            let v2821 = v2573 - (v2667 * v2570);
                            let v2822 = (v2668 * v2570) * v332;
                            let v2824 = (v2667 - v77) / v2821;
                            let v2827 = (v2668 - (v2822 * v2824)) / v2821;
                            let v2829 = v2827 * v2573;
                            let v2830 = v77 + (v2573 * v2824);
                            let v2835 = v2686 * v2834;
                            let v2836 = v372 - v2835;
                            let v2841 = v2686 * v2824;
                            let v2842 = v2827 * v2686;
                            let v2843 = v2835 + v2841;
                            let v2850 = v2836 / v2830;
                            let v2860 = v2827 * v2570;
                            let v2861 = v77 + (v2570 * v2824);
                            let v2867 = v2865 * v2866;
                            let v2868 = v372 - v2867;
                            let v2873 = v2865 * v2824;
                            let v2874 = v2827 * v2865;
                            let v2875 = v2867 + v2873;
                            let v2882 = v2868 / v2861;
                            let v2893 = (((((v2830.ln()) * v2836) * v2834) + (v2843 * v2824)) - ((((v2861.ln()) * v2868) * v2866) + (v2875 * v2824))) / v2643;
                            let v2894 = (((((v2829 * (v344 / v2830)) * v2836) * v2834) + ((v2842 * v2824) + (v2827 * v2843))) - ((((v2860 * (v344 / v2861)) * v2868) * v2866) + ((v2874 * v2824) + (v2827 * v2875)))) / v2643;
                            let v2897 = v2821 * v2821;
                            let v2898 = v2822 * v2821;
                            let v2900 = (v2895 * v2643) / v2897;
                            let v2908 = (v2900 * v2667) * v2664;
                            let v2910 = v2908 * v2639;
                            let v2914 = ((v2850 + v2835) + (v2841 * v400)) - ((v2882 + v2867) + (v2873 * v400));
                            let v2920 = (v2914 * v2910) / v2643;
                            let v2921 = (((((((v2829 * v2850) * v332) / v2830) + (v2842 * v400)) - ((((v2860 * v2882) * v332) / v2861) + (v2874 * v400))) * v2910) + ((((((((((v2898 + v2898) * v2900) * v332) / v2897) * v2667) + (v2668 * v2900)) * v2664) * v2639) + (v2642 * v2908)) * v2914)) / v2643;
                            v2922 = v2893;
                            v2923 = v2920;
                            v2924 = v2894;
                            v2925 = v2921;
                        }
                        v2725 = v2922;
                        v2726 = v2923;
                        v2727 = v2924;
                        v2728 = v2925;
                    } else {
                        let v2674 = v2627 * v2570;
                        let v2675 = v77 + (v2626 * v2570);
                        let v2676 = (v77 - v2626) / v2675;
                        let v2679 = ((v2627 * v332) - (v2674 * v2676)) / v2675;
                        let v2681 = v2679 * v2570;
                        let v2682 = v77 + (v2570 * v2676);
                        let v2683 = v2676 * v2676;
                        let v2684 = v2679 * v2676;
                        let v2687 = v2686 * v400;
                        let v2690 = v77 + (v2687 * v2676);
                        let v2695 = (v2683 * v2690) / v2682;
                        let v2698 = ((((v2684 + v2684) * v2690) + ((v2679 * v2687) * v2683)) - (v2681 * v2695)) / v2682;
                        let v2699 = -v2639;
                        let v2705 = (v2699 * v2682) / v2675;
                        let v2709 = v2682 * v2682;
                        let v2710 = v2681 * v2682;
                        let v2712 = v77 / v2709;
                        let v2716 = v77 + v2712;
                        let v2717 = v2676 * v2716;
                        let v2721 = v2717 * v2705;
                        let v2724 = (((v2679 * v2716) + (((((v2710 + v2710) * v2712) * v332) / v2709) * v2676)) * v2705) + ((((((v2642 * v332) * v2682) + (v2681 * v2699)) - (v2674 * v2705)) / v2675) * v2717);
                        v2725 = v2695;
                        v2726 = v2721;
                        v2727 = v2698;
                        v2728 = v2724;
                    }
                    let v2729 = v2466 * v231;
                    let v2731 = v2729 * v2531;
                    let v2732 = (v286 * v2466) * v2531;
                    let v2735 = (Lanes([v2732[0], 0.0, 0.0, 0.0])) + (v2532 * v2729);
                    let v2736 = v2731 * v2725;
                    let v2739 = (v2735 * v2725) + (v2727 * v2731);
                    let v2740 = v2736 * v2320;
                    let v2743 = (v2739 * v2320) + (v2324 * v2736);
                    let v2744 = v2740 * v2463;
                    let v2750 = v270 * v2744;
                    let v2755 = v2731 * v2320;
                    let v2763 = (v2736 + (v2744 * v215)) + (v2755 * v2726);
                    let v2764 = (v2739 + ((((v2743 * v2463) + (v2465 * v2740)) * v215) + (Lanes([v2750[0], 0.0, 0.0, 0.0])))) + ((((v2735 * v2320) + (v2324 * v2731)) * v2726) + (v2728 * v2755));
                    v2646 = v2740;
                    v2647 = v2763;
                    v2648 = v2743;
                    v2649 = v2764;
                }
                let v2650 = v2467 * v2542;
                let v2656 = v2467 * v2566;
                let v2657 = v2569 * v2467;
                let v2662 = (v2478 * v2320) + (v2650 * v2320);
                let v2663 = ((v2482 * v2320) + (v2324 * v2478)) + (((v2545 * v2467) * v2320) + (v2324 * v2650));
                let v2996: f64;
                let v2997: f64;
                let v2998: f64;
                let v2999: Lanes<4>;
                let v3000: Lanes<4>;
                let v3001: Lanes<4>;
                if v2335 != 0.0 {
                    let v2930 = ((v2330 + v2662) + v2428) + v2646;
                    let v2931 = ((v2334 + v2663) + v2429) + v2648;
                    let v2939 = ((v2178 + (v2501 + v2656)) + v2421) + v2647;
                    let v2940 = (((Lanes([v2179[0], v2179[1], 0.0, v2179[2]])) + (v2502 + v2657)) + v2422) + v2649;
                    let v2947 = v287 * v2428;
                    let v2954 = v288 * v2646;
                    let v2958 = ((v2401 + (v2941 * v2662)) + (v232 * v2428)) + (v233 * v2646);
                    let v2959 = ((v2402 + (v2663 * v2941)) + ((Lanes([v2947[0], 0.0, 0.0, 0.0])) + (v2429 * v232))) + ((Lanes([v2954[0], 0.0, 0.0, 0.0])) + (v2648 * v233));
                    v2996 = v2958;
                    v2997 = v2930;
                    v2998 = v2939;
                    v2999 = v2959;
                    v3000 = v2931;
                    v3001 = v2940;
                } else {
                    let v2961 = v285 * v2330;
                    let v2968 = v287 * v2428;
                    let v2975 = v288 * v2646;
                    let v2979 = (((v230 * v2330) + v2662) + (v232 * v2428)) + (v233 * v2646);
                    let v2980 = ((((Lanes([v2961[0], 0.0, 0.0, 0.0])) + (v2334 * v230)) + v2663) + ((Lanes([v2968[0], 0.0, 0.0, 0.0])) + (v2429 * v232))) + ((Lanes([v2975[0], 0.0, 0.0, 0.0])) + (v2648 * v233));
                    let v2985 = ((v2330 + v2662) + v2428) + v2646;
                    let v2986 = ((v2334 + v2663) + v2429) + v2648;
                    let v2994 = ((v2178 + (v2501 + v2656)) + v2421) + v2647;
                    let v2995 = (((Lanes([v2179[0], v2179[1], 0.0, v2179[2]])) + (v2502 + v2657)) + v2422) + v2649;
                    v2996 = v2979;
                    v2997 = v2985;
                    v2998 = v2994;
                    v2999 = v2980;
                    v3000 = v2986;
                    v3001 = v2995;
                }
                v2436 = v2996;
                v2437 = v2997;
                v2438 = v2662;
                v2439 = v2998;
                v2440 = v2999;
                v2441 = v3000;
                v2442 = v2663;
                v2443 = v3001;
            } else {
                let v2434 = Lanes([v2179[0], v2179[1], 0.0, v2179[2]]);
                v2436 = v2401;
                v2437 = v2330;
                v2438 = v63;
                v2439 = v2178;
                v2440 = v2402;
                v2441 = v2334;
                v2442 = v2435;
                v2443 = v2434;
            }
            let v2444 = v2347 * v2325;
            let v2445 = v2329 * v2347;
            let v2455 = if (if v2335 != 0.0 && (if v2436 > ((ctx.simparam_or("reltol", v2446)) * v2318) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v65 != 0.0 && (if v2437 > ((ctx.simparam_or("reltol", v2446)) * v2318) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v3017: f64;
            let v3018: f64;
            let v3019: f64;
            let v3020: f64;
            let v3021: f64;
            let v3022: f64;
            let v3023: Lanes<4>;
            let v3024: Lanes<4>;
            let v3025: Lanes<4>;
            let v3026: Lanes<4>;
            let v3027: Lanes<4>;
            let v3028: Lanes<4>;
            if v2455 != 0.0 {
                let v3006 = (v2330 * v2436).sqrt();
                let v3015 = (v2098 + v3006) + (v3012 * v2444);
                let v3016 = (v2102 + (((v2334 * v2436) + (v2440 * v2330)) * (v344 / (v416 * v3006)))) + (v2445 * v3012);
                let mut v3070: f64 = 0.0;
                let mut v3071: f64 = 0.0;
                let mut v3072: f64 = 0.0;
                let mut v3073: Lanes<4> = Lanes([0.0; 4]);
                v3070 = v3015;
                v3071 = v3015;
                v3072 = v63;
                v3073 = v3016;
                loop {
                    let v3081 = if (if (v3070.abs()) >= ((ctx.simparam_or("reltol", v2446)) * (v3071.abs())) { 1.0 } else { 0.0 }) != 0.0 && (if v3072 <= v3079 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v3081 == 0.0 {
                        break;
                    }
                    let v3082 = v1482 / v3071;
                    let v3085 = (v2322 - (v3073 * v3082)) / v3071;
                    let v3086 = v1495 / v3071;
                    let v3089 = (v2327 - (v3073 * v3086)) / v3071;
                    let v3090 = v2178 * v3082;
                    let v3091 = v2179 * v3082;
                    let v3094 = (Lanes([v3091[0], v3091[1], 0.0, v3091[2]])) + (v3085 * v2178);
                    let v3128: f64;
                    let v3129: f64;
                    let v3130: Lanes<4>;
                    let v3131: Lanes<3>;
                    if v2335 != 0.0 {
                        let v3108 = v230 * v225;
                        let v3111 = (v285 * v225) + (v280 * v230);
                        let v3112 = v3108 * v3082;
                        let v3113 = v3111 * v3082;
                        let v3116 = (Lanes([v3113[0], 0.0, 0.0, 0.0])) + (v3085 * v3108);
                        let v3117 = Lanes([v3111[0], 0.0, 0.0]);
                        v3128 = v3112;
                        v3129 = v3108;
                        v3130 = v3116;
                        v3131 = v3117;
                    } else {
                        let v3118 = v230 * v3090;
                        let v3119 = v285 * v3090;
                        let v3122 = (Lanes([v3119[0], 0.0, 0.0, 0.0])) + (v3094 * v230);
                        let v3123 = v230 * v2178;
                        let v3124 = v285 * v2178;
                        let v3127 = (Lanes([v3124[0], 0.0, 0.0])) + (v2179 * v230);
                        v3128 = v3118;
                        v3129 = v3123;
                        v3130 = v3122;
                        v3131 = v3127;
                    }
                    let v3133 = if (if v3082 >= v2404 { 1.0 } else { 0.0 }) != 0.0 || v2406 != 0.0 { 1.0 } else { 0.0 };
                    let v3158: f64;
                    let v3159: f64;
                    let v3160: Lanes<4>;
                    let v3161: Lanes<4>;
                    if v3133 != 0.0 {
                        let v3134 = v3082 / v2309;
                        let v3137 = (v3085 - (v2312 * v3134)) / v2309;
                        let v3143 = (v2415 * (v3134.ln())).exp();
                        let v3145 = v2420 * v3143;
                        let v3146 = (((v3137 * (v344 / v3134)) * v2415) * v3143) * v2420;
                        let v3151 = v77 + v2415;
                        let v3152 = (v3145 * v3082) / v3151;
                        let v3153 = ((v3146 * v3082) + (v3085 * v3145)) / v3151;
                        let v3156 = if v2432 < (v2076 * (v684 / v692)) { 1.0 } else { 0.0 };
                        let v3205: f64;
                        let v3206: f64;
                        let v3207: Lanes<4>;
                        let v3208: Lanes<4>;
                        if v3156 != 0.0 {
                            v3205 = v63;
                            v3206 = v63;
                            v3207 = v2435;
                            v3208 = v2435;
                        } else {
                            let v3201 = (v3082 - v2309) / v2432;
                            let v3202 = (v3085 - v2312) / v2432;
                            let v3204 = if v3201 < v3203 { 1.0 } else { 0.0 };
                            let v3320: f64;
                            let v3321: Lanes<4>;
                            if v3204 != 0.0 {
                                v3320 = v3319;
                                v3321 = v2435;
                            } else {
                                v3320 = v3201;
                                v3321 = v3202;
                            }
                            let v3323 = v3321 * v3320;
                            let v3326 = ((v3320 * v3320) + v2589).sqrt();
                            let v3329 = (v3323 + v3323) * (v344 / (v416 * v3326));
                            let v3330 = v3320 + v3326;
                            let v3331 = v3321 + v3329;
                            let v3333 = v3332 / v3330;
                            let v3337 = v3333.exp();
                            let v3339 = v2525 * v3337;
                            let v3340 = ((((v3331 * v3333) * v332) / v3330) * v3337) * v2525;
                            let v3343 = v2432 * v3326;
                            let v3345 = v3343 * v3330;
                            let v3349 = (v400 * v3339) / v3345;
                            let v3352 = ((v3340 * v400) - ((((v3329 * v2432) * v3330) + (v3331 * v3343)) * v3349)) / v3345;
                            v3205 = v3339;
                            v3206 = v3349;
                            v3207 = v3340;
                            v3208 = v3352;
                        }
                        let v3209 = v77 - v2466;
                        let v3210 = v3209 * v231;
                        let v3211 = v286 * v3209;
                        let v3213 = v3207 * v215;
                        let v3214 = v270 * v3205;
                        let v3217 = (v3205 * v215).exp();
                        let v3218 = (v3213 + (Lanes([v3214[0], 0.0, 0.0, 0.0]))) * v3217;
                        let v3219 = v3217 - v77;
                        let v3220 = v3210 * v3219;
                        let v3221 = v3211 * v3219;
                        let v3224 = (Lanes([v3221[0], 0.0, 0.0, 0.0])) + (v3218 * v3210);
                        let v3225 = v3210 * v3082;
                        let v3226 = v3211 * v3082;
                        let v3230 = v3225 * v3217;
                        let v3234 = v3230 * v215;
                        let v3236 = v270 * v3230;
                        let v3243 = v3220 + (v3234 * v3206);
                        let v3244 = v3224 + ((((((((Lanes([v3226[0], 0.0, 0.0, 0.0])) + (v3085 * v3210)) * v3217) + (v3218 * v3225)) * v215) + (Lanes([v3236[0], 0.0, 0.0, 0.0]))) * v3206) + (v3208 * v3234));
                        let v3245 = v77 / v3134;
                        let v3249 = v77 - v3245;
                        let v3250 = (((v3137 * v3245) * v332) / v3134) * v332;
                        let v3252 = v3250 * v3249;
                        let v3255 = ((v3249 * v3249) + v2512).sqrt();
                        let v3258 = (v3252 + v3252) * (v344 / (v416 * v3255));
                        let v3263 = v77 + ((v77 + v2512).sqrt());
                        let v3264 = (v3249 + v3255) / v3263;
                        let v3265 = (v3250 + v3258) / v3263;
                        let v3266 = v3205 - v2525;
                        let v3268 = v270 * v3266;
                        let v3271 = (v3266 * v215).exp();
                        let v3272 = (v3213 + (Lanes([v3268[0], 0.0, 0.0, 0.0]))) * v3271;
                        let v3273 = v231 * v3264;
                        let v3274 = v286 * v3264;
                        let v3278 = v3273 * v3264;
                        let v3282 = v3278 * v3271;
                        let v3285 = (((((Lanes([v3274[0], 0.0, 0.0, 0.0])) + (v3265 * v231)) * v3264) + (v3265 * v3273)) * v3271) + (v3272 * v3278);
                        let v3286 = v3134 * v3255;
                        let v3290 = v400 / v3286;
                        let v3295 = v215 * v3082;
                        let v3296 = v270 * v3082;
                        let v3304 = (v77 + v3290) + (v3295 * v3206);
                        let v3306 = v3282 * v3304;
                        let v3309 = (v3285 * v3304) + (((((((v3137 * v3255) + (v3258 * v3134)) * v3290) * v332) / v3286) + ((((Lanes([v3296[0], 0.0, 0.0, 0.0])) + (v3085 * v215)) * v3206) + (v3208 * v3295))) * v3282);
                        let v3318 = if (if (if (if v2570 < v2571 { 1.0 } else { 0.0 }) != 0.0 && (if v2573 < v2571 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (v3264 * v2570) < v2577 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (v3264 * v2573) < v2577 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v3380: f64;
                        let v3381: f64;
                        let v3382: Lanes<4>;
                        let v3383: Lanes<4>;
                        if v3318 != 0.0 {
                            let v3353 = v2466 * v3282;
                            let v3355 = v3353 * v3082;
                            let v3358 = ((v3285 * v2466) * v3082) + (v3085 * v3353);
                            let v3359 = v2466 * v3306;
                            let v3360 = v3309 * v2466;
                            v3380 = v3355;
                            v3381 = v3359;
                            v3382 = v3358;
                            v3383 = v3360;
                        } else {
                            let v3361 = v77 - v3264;
                            let v3362 = v3265 * v332;
                            let v3363 = v3361 - v77;
                            let v3364 = v77 - v3249;
                            let v3370 = v3255 * v3082;
                            let v3374 = (v3363 * v3364) / v3370;
                            let v3377 = (((v3362 * v3364) + ((v3250 * v332) * v3363)) - (((v3258 * v3082) + (v3085 * v3255)) * v3374)) / v3370;
                            let v3379 = if (v2643.abs()) > v2057 { 1.0 } else { 0.0 };
                            let v3456: f64;
                            let v3457: f64;
                            let v3458: Lanes<4>;
                            let v3459: Lanes<4>;
                            if v3379 != 0.0 {
                                let v3400 = (v3363 * v2664).exp();
                                let v3401 = (v3362 * v2664) * v3400;
                                let v3402 = if v2669 < v2571 { 1.0 } else { 0.0 };
                                let v3648: f64;
                                let v3649: f64;
                                let v3650: Lanes<4>;
                                let v3651: Lanes<4>;
                                if v3402 != 0.0 {
                                    let v3498 = v3400 * v2767;
                                    let v3499 = v3401 * v2767;
                                    let v3500 = (v77 - v3400) / v3498;
                                    let v3503 = ((v3401 * v332) - (v3499 * v3500)) / v3498;
                                    let v3504 = v2767 * v3500;
                                    let v3505 = v3503 * v2767;
                                    let v3506 = v77 + v3504;
                                    let v3507 = v2777 * v2767;
                                    let v3510 = v372 + (v3507 * v3500);
                                    let v3526 = ((v400 * ((v3504 * v3510) - (v372 * (v3506.ln())))) / v2767) / v2767;
                                    let v3527 = (((((v3505 * v3510) + ((v3503 * v3507) * v3504)) - ((v3505 * (v344 / v3506)) * v372)) * v400) / v2767) / v2767;
                                    let v3528 = -v2664;
                                    let v3531 = (v3528 * v3374) / v3498;
                                    let v3535 = v77 + v3506;
                                    let v3536 = v3535 * v3500;
                                    let v3544 = (v3536 * v3531) / v3506;
                                    let v3547 = (((((v3505 * v3500) + (v3503 * v3535)) * v3531) + ((((v3377 * v3528) - (v3499 * v3531)) / v3498) * v3536)) - (v3505 * v3544)) / v3506;
                                    v3648 = v3526;
                                    v3649 = v3544;
                                    v3650 = v3527;
                                    v3651 = v3547;
                                } else {
                                    let v3550 = v2573 - (v3400 * v2570);
                                    let v3551 = (v3401 * v2570) * v332;
                                    let v3553 = (v3400 - v77) / v3550;
                                    let v3556 = (v3401 - (v3551 * v3553)) / v3550;
                                    let v3558 = v3556 * v2573;
                                    let v3559 = v77 + (v2573 * v3553);
                                    let v3563 = v2686 * v2834;
                                    let v3564 = v372 - v3563;
                                    let v3569 = v2686 * v3553;
                                    let v3570 = v3556 * v2686;
                                    let v3571 = v3563 + v3569;
                                    let v3578 = v3564 / v3559;
                                    let v3588 = v3556 * v2570;
                                    let v3589 = v77 + (v2570 * v3553);
                                    let v3593 = v2865 * v2866;
                                    let v3594 = v372 - v3593;
                                    let v3599 = v2865 * v3553;
                                    let v3600 = v3556 * v2865;
                                    let v3601 = v3593 + v3599;
                                    let v3608 = v3594 / v3589;
                                    let v3619 = (((((v3559.ln()) * v3564) * v2834) + (v3571 * v3553)) - ((((v3589.ln()) * v3594) * v2866) + (v3601 * v3553))) / v2643;
                                    let v3620 = (((((v3558 * (v344 / v3559)) * v3564) * v2834) + ((v3570 * v3553) + (v3556 * v3571))) - ((((v3588 * (v344 / v3589)) * v3594) * v2866) + ((v3600 * v3553) + (v3556 * v3601)))) / v2643;
                                    let v3623 = v3550 * v3550;
                                    let v3624 = v3551 * v3550;
                                    let v3626 = (v3621 * v2643) / v3623;
                                    let v3634 = (v3626 * v3400) * v2664;
                                    let v3636 = v3634 * v3374;
                                    let v3640 = ((v3578 + v3563) + (v3569 * v400)) - ((v3608 + v3593) + (v3599 * v400));
                                    let v3646 = (v3640 * v3636) / v2643;
                                    let v3647 = (((((((v3558 * v3578) * v332) / v3559) + (v3570 * v400)) - ((((v3588 * v3608) * v332) / v3589) + (v3600 * v400))) * v3636) + ((((((((((v3624 + v3624) * v3626) * v332) / v3623) * v3400) + (v3401 * v3626)) * v2664) * v3374) + (v3377 * v3634)) * v3640)) / v2643;
                                    v3648 = v3619;
                                    v3649 = v3646;
                                    v3650 = v3620;
                                    v3651 = v3647;
                                }
                                v3456 = v3648;
                                v3457 = v3649;
                                v3458 = v3650;
                                v3459 = v3651;
                            } else {
                                let v3406 = v3362 * v2570;
                                let v3407 = v77 + (v3361 * v2570);
                                let v3408 = (v77 - v3361) / v3407;
                                let v3411 = ((v3362 * v332) - (v3406 * v3408)) / v3407;
                                let v3413 = v3411 * v2570;
                                let v3414 = v77 + (v2570 * v3408);
                                let v3415 = v3408 * v3408;
                                let v3416 = v3411 * v3408;
                                let v3418 = v2686 * v400;
                                let v3421 = v77 + (v3418 * v3408);
                                let v3426 = (v3415 * v3421) / v3414;
                                let v3429 = ((((v3416 + v3416) * v3421) + ((v3411 * v3418) * v3415)) - (v3413 * v3426)) / v3414;
                                let v3430 = -v3374;
                                let v3436 = (v3430 * v3414) / v3407;
                                let v3440 = v3414 * v3414;
                                let v3441 = v3413 * v3414;
                                let v3443 = v77 / v3440;
                                let v3447 = v77 + v3443;
                                let v3448 = v3408 * v3447;
                                let v3452 = v3448 * v3436;
                                let v3455 = (((v3411 * v3447) + (((((v3441 + v3441) * v3443) * v332) / v3440) * v3408)) * v3436) + ((((((v3377 * v332) * v3414) + (v3413 * v3430)) - (v3406 * v3436)) / v3407) * v3448);
                                v3456 = v3426;
                                v3457 = v3452;
                                v3458 = v3429;
                                v3459 = v3455;
                            }
                            let v3460 = v2466 * v231;
                            let v3462 = v3460 * v3271;
                            let v3463 = (v286 * v2466) * v3271;
                            let v3466 = (Lanes([v3463[0], 0.0, 0.0, 0.0])) + (v3272 * v3460);
                            let v3467 = v3462 * v3456;
                            let v3470 = (v3466 * v3456) + (v3458 * v3462);
                            let v3471 = v3467 * v3082;
                            let v3474 = (v3470 * v3082) + (v3085 * v3467);
                            let v3475 = v3471 * v3206;
                            let v3481 = v270 * v3475;
                            let v3486 = v3462 * v3082;
                            let v3494 = (v3467 + (v3475 * v215)) + (v3486 * v3457);
                            let v3495 = (v3470 + ((((v3474 * v3206) + (v3208 * v3471)) * v215) + (Lanes([v3481[0], 0.0, 0.0, 0.0])))) + ((((v3466 * v3082) + (v3085 * v3462)) * v3457) + (v3459 * v3486));
                            v3380 = v3471;
                            v3381 = v3494;
                            v3382 = v3474;
                            v3383 = v3495;
                        }
                        let v3384 = v3209 * v3282;
                        let v3390 = v3209 * v3306;
                        let v3391 = v3309 * v3209;
                        let v3396 = (v3220 * v3082) + (v3384 * v3082);
                        let v3397 = ((v3224 * v3082) + (v3085 * v3220)) + (((v3285 * v3209) * v3082) + (v3085 * v3384));
                        let v3736: f64;
                        let v3737: f64;
                        let v3738: Lanes<4>;
                        let v3739: Lanes<4>;
                        if v2335 != 0.0 {
                            let v3659 = v287 * v3152;
                            let v3666 = v288 * v3380;
                            let v3670 = ((v3128 + (v2941 * v3396)) + (v232 * v3152)) + (v233 * v3380);
                            let v3671 = ((v3130 + (v3397 * v2941)) + ((Lanes([v3659[0], 0.0, 0.0, 0.0])) + (v3153 * v232))) + ((Lanes([v3666[0], 0.0, 0.0, 0.0])) + (v3382 * v233));
                            let v3678 = v287 * v3145;
                            let v3685 = v288 * v3381;
                            let v3689 = ((v3129 + (v2941 * (v3243 + v3390))) + (v232 * v3145)) + (v233 * v3381);
                            let v3690 = (((Lanes([v3131[0], v3131[1], 0.0, v3131[2]])) + ((v3244 + v3391) * v2941)) + ((Lanes([v3678[0], 0.0, 0.0, 0.0])) + (v3146 * v232))) + ((Lanes([v3685[0], 0.0, 0.0, 0.0])) + (v3383 * v233));
                            v3736 = v3670;
                            v3737 = v3689;
                            v3738 = v3671;
                            v3739 = v3690;
                        } else {
                            let v3692 = v285 * v3090;
                            let v3699 = v287 * v3152;
                            let v3706 = v288 * v3380;
                            let v3710 = (((v230 * v3090) + v3396) + (v232 * v3152)) + (v233 * v3380);
                            let v3711 = ((((Lanes([v3692[0], 0.0, 0.0, 0.0])) + (v3094 * v230)) + v3397) + ((Lanes([v3699[0], 0.0, 0.0, 0.0])) + (v3153 * v232))) + ((Lanes([v3706[0], 0.0, 0.0, 0.0])) + (v3382 * v233));
                            let v3713 = v285 * v2178;
                            let v3716 = (Lanes([v3713[0], 0.0, 0.0])) + (v2179 * v230);
                            let v3723 = v287 * v3145;
                            let v3730 = v288 * v3381;
                            let v3734 = (((v230 * v2178) + (v3243 + v3390)) + (v232 * v3145)) + (v233 * v3381);
                            let v3735 = (((Lanes([v3716[0], v3716[1], 0.0, v3716[2]])) + (v3244 + v3391)) + ((Lanes([v3723[0], 0.0, 0.0, 0.0])) + (v3146 * v232))) + ((Lanes([v3730[0], 0.0, 0.0, 0.0])) + (v3383 * v233));
                            v3736 = v3710;
                            v3737 = v3734;
                            v3738 = v3711;
                            v3739 = v3735;
                        }
                        v3158 = v3736;
                        v3159 = v3737;
                        v3160 = v3738;
                        v3161 = v3739;
                    } else {
                        let v3157 = Lanes([v3131[0], v3131[1], 0.0, v3131[2]]);
                        v3158 = v3128;
                        v3159 = v3129;
                        v3160 = v3130;
                        v3161 = v3157;
                    }
                    let v3162 = v3012 * v2347;
                    let v3163 = v3162 * v3086;
                    let v3164 = v3089 * v3162;
                    let v3179 = ((v3159 * v3082) + v3163) / v3071;
                    let v3183 = v77 + v3179;
                    let v3184 = (-(v3071 - ((v2098 + v3158) + v3163))) / v3183;
                    let v3187 = (((v3073 - ((v2102 + v3160) + v3164)) * v332) - ((((((v3161 * v3082) + (v3085 * v3159)) + v3164) - (v3073 * v3179)) / v3071) * v3184)) / v3183;
                    let v3189 = v3188 * v3071;
                    let v3191 = v3189.abs();
                    let v3196 = (v3073 * v3188) * ((v416 * (if v3189 >= v3192 { 1.0 } else { 0.0 })) - v344);
                    let v3198 = if (v3184.abs()) > v3191 { 1.0 } else { 0.0 };
                    let v3741: f64;
                    let v3742: Lanes<4>;
                    if v3198 != 0.0 {
                        let v3740 = if v3184 >= v63 { 1.0 } else { 0.0 };
                        let v3748: f64;
                        let v3749: Lanes<4>;
                        if v3740 != 0.0 {
                            v3748 = v3191;
                            v3749 = v3196;
                        } else {
                            let v3746 = -v3191;
                            let v3747 = v3196 * v332;
                            v3748 = v3746;
                            v3749 = v3747;
                        }
                        v3741 = v3748;
                        v3742 = v3749;
                    } else {
                        v3741 = v3184;
                        v3742 = v3187;
                    }
                    let v3743 = v3071 + v3741;
                    let v3744 = v3073 + v3742;
                    let v3745 = v3072 + v77;
                    v3070 = v3741;
                    v3071 = v3743;
                    v3072 = v3745;
                    v3073 = v3744;
                }
                let v3095 = v1482 / v3071;
                let v3098 = (v2322 - (v3073 * v3095)) / v3071;
                let v3099 = v1495 / v3071;
                let v3102 = (v2327 - (v3073 * v3099)) / v3071;
                let v3103 = v2178 * v3095;
                let v3104 = v2179 * v3095;
                let v3107 = (Lanes([v3104[0], v3104[1], 0.0, v3104[2]])) + (v3098 * v2178);
                let v3751 = if (if v3095 >= v2404 { 1.0 } else { 0.0 }) != 0.0 || v2406 != 0.0 { 1.0 } else { 0.0 };
                let v3776: f64;
                let v3777: f64;
                let v3778: f64;
                let v3779: Lanes<4>;
                let v3780: Lanes<4>;
                let v3781: Lanes<4>;
                if v3751 != 0.0 {
                    let v3752 = v3095 / v2309;
                    let v3755 = (v3098 - (v2312 * v3752)) / v2309;
                    let v3761 = (v2415 * (v3752.ln())).exp();
                    let v3763 = v2420 * v3761;
                    let v3764 = (((v3755 * (v344 / v3752)) * v2415) * v3761) * v2420;
                    let v3769 = v77 + v2415;
                    let v3770 = (v3763 * v3095) / v3769;
                    let v3771 = ((v3764 * v3095) + (v3098 * v3763)) / v3769;
                    let v3774 = if v2432 < (v2076 * (v684 / v692)) { 1.0 } else { 0.0 };
                    let v3790: f64;
                    let v3791: f64;
                    let v3792: Lanes<4>;
                    let v3793: Lanes<4>;
                    if v3774 != 0.0 {
                        v3790 = v63;
                        v3791 = v63;
                        v3792 = v2435;
                        v3793 = v2435;
                    } else {
                        let v3786 = (v3095 - v2309) / v2432;
                        let v3787 = (v3098 - v2312) / v2432;
                        let v3789 = if v3786 < v3788 { 1.0 } else { 0.0 };
                        let v3905: f64;
                        let v3906: Lanes<4>;
                        if v3789 != 0.0 {
                            v3905 = v3904;
                            v3906 = v2435;
                        } else {
                            v3905 = v3786;
                            v3906 = v3787;
                        }
                        let v3908 = v3906 * v3905;
                        let v3911 = ((v3905 * v3905) + v2589).sqrt();
                        let v3914 = (v3908 + v3908) * (v344 / (v416 * v3911));
                        let v3915 = v3905 + v3911;
                        let v3916 = v3906 + v3914;
                        let v3918 = v3917 / v3915;
                        let v3922 = v3918.exp();
                        let v3924 = v2525 * v3922;
                        let v3925 = ((((v3916 * v3918) * v332) / v3915) * v3922) * v2525;
                        let v3928 = v2432 * v3911;
                        let v3930 = v3928 * v3915;
                        let v3934 = (v400 * v3924) / v3930;
                        let v3937 = ((v3925 * v400) - ((((v3914 * v2432) * v3915) + (v3916 * v3928)) * v3934)) / v3930;
                        v3790 = v3924;
                        v3791 = v3934;
                        v3792 = v3925;
                        v3793 = v3937;
                    }
                    let v3794 = v77 - v2466;
                    let v3795 = v3794 * v231;
                    let v3796 = v286 * v3794;
                    let v3798 = v3792 * v215;
                    let v3799 = v270 * v3790;
                    let v3802 = (v3790 * v215).exp();
                    let v3803 = (v3798 + (Lanes([v3799[0], 0.0, 0.0, 0.0]))) * v3802;
                    let v3804 = v3802 - v77;
                    let v3805 = v3795 * v3804;
                    let v3806 = v3796 * v3804;
                    let v3809 = (Lanes([v3806[0], 0.0, 0.0, 0.0])) + (v3803 * v3795);
                    let v3810 = v3795 * v3095;
                    let v3811 = v3796 * v3095;
                    let v3815 = v3810 * v3802;
                    let v3819 = v3815 * v215;
                    let v3821 = v270 * v3815;
                    let v3828 = v3805 + (v3819 * v3791);
                    let v3829 = v3809 + ((((((((Lanes([v3811[0], 0.0, 0.0, 0.0])) + (v3098 * v3795)) * v3802) + (v3803 * v3810)) * v215) + (Lanes([v3821[0], 0.0, 0.0, 0.0]))) * v3791) + (v3793 * v3819));
                    let v3830 = v77 / v3752;
                    let v3834 = v77 - v3830;
                    let v3835 = (((v3755 * v3830) * v332) / v3752) * v332;
                    let v3837 = v3835 * v3834;
                    let v3840 = ((v3834 * v3834) + v2512).sqrt();
                    let v3843 = (v3837 + v3837) * (v344 / (v416 * v3840));
                    let v3848 = v77 + ((v77 + v2512).sqrt());
                    let v3849 = (v3834 + v3840) / v3848;
                    let v3850 = (v3835 + v3843) / v3848;
                    let v3851 = v3790 - v2525;
                    let v3853 = v270 * v3851;
                    let v3856 = (v3851 * v215).exp();
                    let v3857 = (v3798 + (Lanes([v3853[0], 0.0, 0.0, 0.0]))) * v3856;
                    let v3858 = v231 * v3849;
                    let v3859 = v286 * v3849;
                    let v3863 = v3858 * v3849;
                    let v3867 = v3863 * v3856;
                    let v3870 = (((((Lanes([v3859[0], 0.0, 0.0, 0.0])) + (v3850 * v231)) * v3849) + (v3850 * v3858)) * v3856) + (v3857 * v3863);
                    let v3871 = v3752 * v3840;
                    let v3875 = v400 / v3871;
                    let v3880 = v215 * v3095;
                    let v3881 = v270 * v3095;
                    let v3889 = (v77 + v3875) + (v3880 * v3791);
                    let v3891 = v3867 * v3889;
                    let v3894 = (v3870 * v3889) + (((((((v3755 * v3840) + (v3843 * v3752)) * v3875) * v332) / v3871) + ((((Lanes([v3881[0], 0.0, 0.0, 0.0])) + (v3098 * v215)) * v3791) + (v3793 * v3880))) * v3867);
                    let v3903 = if (if (if (if v2570 < v2571 { 1.0 } else { 0.0 }) != 0.0 && (if v2573 < v2571 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (v3849 * v2570) < v2577 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (v3849 * v2573) < v2577 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v3965: f64;
                    let v3966: f64;
                    let v3967: Lanes<4>;
                    let v3968: Lanes<4>;
                    if v3903 != 0.0 {
                        let v3938 = v2466 * v3867;
                        let v3940 = v3938 * v3095;
                        let v3943 = ((v3870 * v2466) * v3095) + (v3098 * v3938);
                        let v3944 = v2466 * v3891;
                        let v3945 = v3894 * v2466;
                        v3965 = v3940;
                        v3966 = v3944;
                        v3967 = v3943;
                        v3968 = v3945;
                    } else {
                        let v3946 = v77 - v3849;
                        let v3947 = v3850 * v332;
                        let v3948 = v3946 - v77;
                        let v3949 = v77 - v3834;
                        let v3955 = v3840 * v3095;
                        let v3959 = (v3948 * v3949) / v3955;
                        let v3962 = (((v3947 * v3949) + ((v3835 * v332) * v3948)) - (((v3843 * v3095) + (v3098 * v3840)) * v3959)) / v3955;
                        let v3964 = if (v2643.abs()) > v2057 { 1.0 } else { 0.0 };
                        let v4041: f64;
                        let v4042: f64;
                        let v4043: Lanes<4>;
                        let v4044: Lanes<4>;
                        if v3964 != 0.0 {
                            let v3985 = (v3948 * v2664).exp();
                            let v3986 = (v3947 * v2664) * v3985;
                            let v3987 = if v2669 < v2571 { 1.0 } else { 0.0 };
                            let v4233: f64;
                            let v4234: f64;
                            let v4235: Lanes<4>;
                            let v4236: Lanes<4>;
                            if v3987 != 0.0 {
                                let v4083 = v3985 * v2767;
                                let v4084 = v3986 * v2767;
                                let v4085 = (v77 - v3985) / v4083;
                                let v4088 = ((v3986 * v332) - (v4084 * v4085)) / v4083;
                                let v4089 = v2767 * v4085;
                                let v4090 = v4088 * v2767;
                                let v4091 = v77 + v4089;
                                let v4092 = v2777 * v2767;
                                let v4095 = v372 + (v4092 * v4085);
                                let v4111 = ((v400 * ((v4089 * v4095) - (v372 * (v4091.ln())))) / v2767) / v2767;
                                let v4112 = (((((v4090 * v4095) + ((v4088 * v4092) * v4089)) - ((v4090 * (v344 / v4091)) * v372)) * v400) / v2767) / v2767;
                                let v4113 = -v2664;
                                let v4116 = (v4113 * v3959) / v4083;
                                let v4120 = v77 + v4091;
                                let v4121 = v4120 * v4085;
                                let v4129 = (v4121 * v4116) / v4091;
                                let v4132 = (((((v4090 * v4085) + (v4088 * v4120)) * v4116) + ((((v3962 * v4113) - (v4084 * v4116)) / v4083) * v4121)) - (v4090 * v4129)) / v4091;
                                v4233 = v4111;
                                v4234 = v4129;
                                v4235 = v4112;
                                v4236 = v4132;
                            } else {
                                let v4135 = v2573 - (v3985 * v2570);
                                let v4136 = (v3986 * v2570) * v332;
                                let v4138 = (v3985 - v77) / v4135;
                                let v4141 = (v3986 - (v4136 * v4138)) / v4135;
                                let v4143 = v4141 * v2573;
                                let v4144 = v77 + (v2573 * v4138);
                                let v4148 = v2686 * v2834;
                                let v4149 = v372 - v4148;
                                let v4154 = v2686 * v4138;
                                let v4155 = v4141 * v2686;
                                let v4156 = v4148 + v4154;
                                let v4163 = v4149 / v4144;
                                let v4173 = v4141 * v2570;
                                let v4174 = v77 + (v2570 * v4138);
                                let v4178 = v2865 * v2866;
                                let v4179 = v372 - v4178;
                                let v4184 = v2865 * v4138;
                                let v4185 = v4141 * v2865;
                                let v4186 = v4178 + v4184;
                                let v4193 = v4179 / v4174;
                                let v4204 = (((((v4144.ln()) * v4149) * v2834) + (v4156 * v4138)) - ((((v4174.ln()) * v4179) * v2866) + (v4186 * v4138))) / v2643;
                                let v4205 = (((((v4143 * (v344 / v4144)) * v4149) * v2834) + ((v4155 * v4138) + (v4141 * v4156))) - ((((v4173 * (v344 / v4174)) * v4179) * v2866) + ((v4185 * v4138) + (v4141 * v4186)))) / v2643;
                                let v4208 = v4135 * v4135;
                                let v4209 = v4136 * v4135;
                                let v4211 = (v4206 * v2643) / v4208;
                                let v4219 = (v4211 * v3985) * v2664;
                                let v4221 = v4219 * v3959;
                                let v4225 = ((v4163 + v4148) + (v4154 * v400)) - ((v4193 + v4178) + (v4184 * v400));
                                let v4231 = (v4225 * v4221) / v2643;
                                let v4232 = (((((((v4143 * v4163) * v332) / v4144) + (v4155 * v400)) - ((((v4173 * v4193) * v332) / v4174) + (v4185 * v400))) * v4221) + ((((((((((v4209 + v4209) * v4211) * v332) / v4208) * v3985) + (v3986 * v4211)) * v2664) * v3959) + (v3962 * v4219)) * v4225)) / v2643;
                                v4233 = v4204;
                                v4234 = v4231;
                                v4235 = v4205;
                                v4236 = v4232;
                            }
                            v4041 = v4233;
                            v4042 = v4234;
                            v4043 = v4235;
                            v4044 = v4236;
                        } else {
                            let v3991 = v3947 * v2570;
                            let v3992 = v77 + (v3946 * v2570);
                            let v3993 = (v77 - v3946) / v3992;
                            let v3996 = ((v3947 * v332) - (v3991 * v3993)) / v3992;
                            let v3998 = v3996 * v2570;
                            let v3999 = v77 + (v2570 * v3993);
                            let v4000 = v3993 * v3993;
                            let v4001 = v3996 * v3993;
                            let v4003 = v2686 * v400;
                            let v4006 = v77 + (v4003 * v3993);
                            let v4011 = (v4000 * v4006) / v3999;
                            let v4014 = ((((v4001 + v4001) * v4006) + ((v3996 * v4003) * v4000)) - (v3998 * v4011)) / v3999;
                            let v4015 = -v3959;
                            let v4021 = (v4015 * v3999) / v3992;
                            let v4025 = v3999 * v3999;
                            let v4026 = v3998 * v3999;
                            let v4028 = v77 / v4025;
                            let v4032 = v77 + v4028;
                            let v4033 = v3993 * v4032;
                            let v4037 = v4033 * v4021;
                            let v4040 = (((v3996 * v4032) + (((((v4026 + v4026) * v4028) * v332) / v4025) * v3993)) * v4021) + ((((((v3962 * v332) * v3999) + (v3998 * v4015)) - (v3991 * v4021)) / v3992) * v4033);
                            v4041 = v4011;
                            v4042 = v4037;
                            v4043 = v4014;
                            v4044 = v4040;
                        }
                        let v4045 = v2466 * v231;
                        let v4047 = v4045 * v3856;
                        let v4048 = (v286 * v2466) * v3856;
                        let v4051 = (Lanes([v4048[0], 0.0, 0.0, 0.0])) + (v3857 * v4045);
                        let v4052 = v4047 * v4041;
                        let v4055 = (v4051 * v4041) + (v4043 * v4047);
                        let v4056 = v4052 * v3095;
                        let v4059 = (v4055 * v3095) + (v3098 * v4052);
                        let v4060 = v4056 * v3791;
                        let v4066 = v270 * v4060;
                        let v4071 = v4047 * v3095;
                        let v4079 = (v4052 + (v4060 * v215)) + (v4071 * v4042);
                        let v4080 = (v4055 + ((((v4059 * v3791) + (v3793 * v4056)) * v215) + (Lanes([v4066[0], 0.0, 0.0, 0.0])))) + ((((v4051 * v3095) + (v3098 * v4047)) * v4042) + (v4044 * v4071));
                        v3965 = v4056;
                        v3966 = v4079;
                        v3967 = v4059;
                        v3968 = v4080;
                    }
                    let v3969 = v3794 * v3867;
                    let v3975 = v3794 * v3891;
                    let v3976 = v3894 * v3794;
                    let v3981 = (v3805 * v3095) + (v3969 * v3095);
                    let v3982 = ((v3809 * v3095) + (v3098 * v3805)) + (((v3870 * v3794) * v3095) + (v3098 * v3969));
                    let v4267: f64;
                    let v4268: f64;
                    let v4269: Lanes<4>;
                    let v4270: Lanes<4>;
                    if v2335 != 0.0 {
                        let v4241 = ((v3103 + v3981) + v3770) + v3965;
                        let v4242 = ((v3107 + v3982) + v3771) + v3967;
                        let v4250 = ((v2178 + (v3828 + v3975)) + v3763) + v3966;
                        let v4251 = (((Lanes([v2179[0], v2179[1], 0.0, v2179[2]])) + (v3829 + v3976)) + v3764) + v3968;
                        v4267 = v4241;
                        v4268 = v4250;
                        v4269 = v4242;
                        v4270 = v4251;
                    } else {
                        let v4256 = ((v3103 + v3981) + v3770) + v3965;
                        let v4257 = ((v3107 + v3982) + v3771) + v3967;
                        let v4265 = ((v2178 + (v3828 + v3975)) + v3763) + v3966;
                        let v4266 = (((Lanes([v2179[0], v2179[1], 0.0, v2179[2]])) + (v3829 + v3976)) + v3764) + v3968;
                        v4267 = v4256;
                        v4268 = v4265;
                        v4269 = v4257;
                        v4270 = v4266;
                    }
                    v3776 = v4267;
                    v3777 = v3981;
                    v3778 = v4268;
                    v3779 = v4269;
                    v3780 = v3982;
                    v3781 = v4270;
                } else {
                    let v3775 = Lanes([v2179[0], v2179[1], 0.0, v2179[2]]);
                    v3776 = v3103;
                    v3777 = v63;
                    v3778 = v2178;
                    v3779 = v3107;
                    v3780 = v2435;
                    v3781 = v3775;
                }
                let v3782 = v2347 * v3099;
                let v3783 = v3102 * v2347;
                v3017 = v3095;
                v3018 = v3099;
                v3019 = v3776;
                v3020 = v3782;
                v3021 = v3777;
                v3022 = v3778;
                v3023 = v3098;
                v3024 = v3102;
                v3025 = v3779;
                v3026 = v3783;
                v3027 = v3780;
                v3028 = v3781;
            } else {
                v3017 = v2320;
                v3018 = v2325;
                v3019 = v2437;
                v3020 = v2444;
                v3021 = v2438;
                v3022 = v2439;
                v3023 = v2324;
                v3024 = v2329;
                v3025 = v2441;
                v3026 = v2445;
                v3027 = v2442;
                v3028 = v2443;
            }
            let v3029 = v3017 - v3018;
            let v3030 = v3023 - v3024;
            let v3031 = v2178 * v3017;
            let v3032 = v2179 * v3017;
            let v3038 = v270 * v3031;
            let v3041 = v2347 * v3018;
            let v3045 = v270 * v3041;
            let v3057 = v3056 * (((v1629 + v1636) + (v3031 * v215)) + (v3041 * v215));
            let v3059 = v23 - v0;
            let v3062 = (Lanes([v25[0], 0.0])) - (Lanes([0.0, v3[0]]));
            let v3063 = v3057 * v3059;
            let v3064 = (((((Lanes([v1631[0], 0.0, v1631[1], v1631[2]])) + (Lanes([v1638[0], v1638[1], 0.0, v1638[2]]))) + ((((Lanes([v3032[0], v3032[1], 0.0, v3032[2]])) + (v3023 * v2178)) * v215) + (Lanes([v3038[0], 0.0, 0.0, 0.0])))) + (((v3024 * v2347) * v215) + (Lanes([v3045[0], 0.0, 0.0, 0.0])))) * v3056) * v3059;
            let v3065 = v3062 * v3057;
            let v3068 = (Lanes([v3064[0], v3064[1], v3064[2], 0.0, v3064[3]])) + (Lanes([0.0, 0.0, 0.0, v3065[0], v3065[1]]));
            let v3069 = if v581 > v63 { 1.0 } else { 0.0 };
            let v4280: f64;
            let v4281: Lanes<3>;
            if v3069 != 0.0 {
                let v4272 = v4271 * v211;
                let v4274 = v17 / v4272;
                let v4275 = (v266 * v4271) * v4274;
                let v4278 = (v2108 - (Lanes([v4275[0], 0.0, 0.0]))) / v4272;
                let v4279 = if v4274 > v1436 { 1.0 } else { 0.0 };
                let v4284: f64;
                let v4285: f64;
                let v4286: Lanes<3>;
                let v4287: Lanes<3>;
                if v4279 != 0.0 {
                    let v4283 = v77 + (v4274 - v1436);
                    v4284 = v4283;
                    v4285 = v1436;
                    v4286 = v4278;
                    v4287 = v1689;
                } else {
                    v4284 = v77;
                    v4285 = v4274;
                    v4286 = v1689;
                    v4287 = v4278;
                }
                let v4288 = rspice_limexp(v4285);
                let v4294 = (v4284 * v4288) - v77;
                let v4295 = v234 * v4294;
                let v4296 = v289 * v4294;
                let v4299 = (Lanes([v4296[0], 0.0, 0.0])) + (((v4286 * v4288) + ((v4287 * v4288) * v4284)) * v234);
                v4280 = v4295;
                v4281 = v4299;
            } else {
                v4280 = v63;
                v4281 = v1689;
            }
            let v4303: f64;
            let v4304: Lanes<3>;
            if v73 != 0.0 {
                let v4302 = if (if v219 > v63 { 1.0 } else { 0.0 }) != 0.0 && (if v220 > v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v4349: f64;
                let v4350: Lanes<3>;
                if v4302 != 0.0 {
                    let v4306 = (v77 / v546) - v77;
                    let v4307 = v1636 / v219;
                    let v4308 = v274 * v4307;
                    let v4317 = (v4306 * (v4307.ln())).exp();
                    let v4318 = ((((v1638 - (Lanes([v4308[0], 0.0, 0.0]))) / v219) * (v344 / v4307)) * v4306) * v4317;
                    let v4319 = -v235;
                    let v4322 = (v290 * v332) * v17;
                    let v4323 = v18 * v4319;
                    let v4327 = v220 * v4317;
                    let v4328 = v275 * v4317;
                    let v4332 = (v4319 * v17) / v4327;
                    let v4336 = -v236;
                    let v4339 = (v291 * v332) * v4317;
                    let v4343 = (v4336 * v4317).exp();
                    let v4345 = v4332 * v4343;
                    let v4348 = (((((Lanes([v4322[0], 0.0, 0.0])) + (Lanes([0.0, v4323[0], v4323[1]]))) - (((Lanes([v4328[0], 0.0, 0.0])) + (v4318 * v220)) * v4332)) / v4327) * v4343) + ((((Lanes([v4339[0], 0.0, 0.0])) + (v4318 * v4336)) * v4343) * v4332);
                    v4349 = v4345;
                    v4350 = v4348;
                } else {
                    v4349 = v63;
                    v4350 = v1689;
                }
                v4303 = v4349;
                v4304 = v4350;
            } else {
                v4303 = v63;
                v4304 = v1689;
            }
            let v4355: f64;
            let v4356: Lanes<4>;
            if v70 != 0.0 {
                let v4351 = v220 - v17;
                let v4353 = (Lanes([v275[0], 0.0, 0.0])) - v2108;
                let v4354 = if v4351 > v63 { 1.0 } else { 0.0 };
                let v4363: f64;
                let v4364: Lanes<4>;
                if v4354 != 0.0 {
                    let v4362 = if v4361 > v63 { 1.0 } else { 0.0 };
                    let v4412: f64;
                    let v4413: Lanes<4>;
                    if v4362 != 0.0 {
                        let v4365 = v1636 / v219;
                        let v4366 = v274 * v4365;
                        let v4370 = v4361 * v228;
                        let v4375 = ((v283 * v4361) * v229) + (v284 * v4370);
                        let v4379 = (v4370 * v229) + (v4376 * v3017);
                        let v4384 = (v4365 / v1715).exp();
                        let v4385 = (((v1638 - (Lanes([v4366[0], 0.0, 0.0]))) / v219) / v1715) * v4384;
                        let v4387 = v3017 / v4379;
                        let v4393 = (v77 - v4387) / v1715;
                        let v4400 = (v4384 - v400) + (v400 * (v4393.cosh()));
                        let v4408 = (v1715 * (v4400.ln())).sqrt();
                        let v4411 = ((((Lanes([v4385[0], v4385[1], 0.0, v4385[2]])) + ((((((v3023 - (((Lanes([v4375[0], 0.0, 0.0, 0.0])) + (v3023 * v4376)) * v4387)) / v4379) * v332) / v1715) * (v4393.sinh())) * v400)) * (v344 / v4400)) * v1715) * (v344 / (v416 * v4408));
                        v4412 = v4408;
                        v4413 = v4411;
                    } else {
                        v4412 = v77;
                        v4413 = v2435;
                    }
                    let v4414 = v237 / v1636;
                    let v4418 = ((Lanes([v292[0], 0.0, 0.0])) - (v1638 * v4414)) / v1636;
                    let v4419 = v237 / v219;
                    let v4422 = (v292 - (v274 * v4419)) / v219;
                    let v4423 = if v4351 > v4419 { 1.0 } else { 0.0 };
                    let v4487: f64;
                    let v4488: Lanes<4>;
                    if v4423 != 0.0 {
                        let v4425 = v4418 * v332;
                        let v4426 = v4419 * v4412;
                        let v4427 = v4422 * v4412;
                        let v4431 = (-v4414) / v4426;
                        let v4436 = v4431.exp();
                        let v4438 = v238 * v4436;
                        let v4439 = v293 * v4436;
                        let v4443 = v4414 / v4419;
                        let v4444 = v4422 * v4443;
                        let v4448 = v77 + v4443;
                        let v4449 = v4351 - v4419;
                        let v4450 = Lanes([v4422[0], 0.0, 0.0]);
                        let v4456 = v4419 + (v4448 * v4449);
                        let v4458 = v4438 * v4456;
                        let v4460 = (v4450 + ((((v4418 - (Lanes([v4444[0], 0.0, 0.0]))) / v4419) * v4449) + ((v4353 - v4450) * v4448))) * v4438;
                        let v4462 = (((Lanes([v4439[0], 0.0, 0.0, 0.0])) + (((((Lanes([v4425[0], v4425[1], 0.0, v4425[2]])) - (((Lanes([v4427[0], 0.0, 0.0, 0.0])) + (v4413 * v4419)) * v4431)) / v4426) * v4436) * v238)) * v4456) + (Lanes([v4460[0], v4460[1], 0.0, v4460[2]]));
                        v4487 = v4458;
                        v4488 = v4462;
                    } else {
                        let v4463 = v238 * v4351;
                        let v4464 = v293 * v4351;
                        let v4469 = v4418 * v332;
                        let v4470 = v4351 * v4412;
                        let v4471 = v4353 * v4412;
                        let v4475 = (-v4414) / v4470;
                        let v4480 = v4475.exp();
                        let v4482 = v4463 * v4480;
                        let v4483 = ((Lanes([v4464[0], 0.0, 0.0])) + (v4353 * v238)) * v4480;
                        let v4486 = (Lanes([v4483[0], v4483[1], 0.0, v4483[2]])) + (((((Lanes([v4469[0], v4469[1], 0.0, v4469[2]])) - (((Lanes([v4471[0], v4471[1], 0.0, v4471[2]])) + (v4413 * v4351)) * v4475)) / v4470) * v4480) * v4463);
                        v4487 = v4482;
                        v4488 = v4486;
                    }
                    let v4490 = if v4489 > v63 { 1.0 } else { 0.0 };
                    let v4520: f64;
                    let v4521: Lanes<4>;
                    if v4490 != 0.0 {
                        let v4493 = v77 - (v4489 * v4487);
                        let v4494 = (v4488 * v4489) * v332;
                        let v4496 = v4494 * v4493;
                        let v4500 = ((v4493 * v4493) + v4498).sqrt();
                        let v4506 = v372 * (v4493 + v4500);
                        let v4512 = (v3017 * v4487) / v4506;
                        let v4515 = (((v3023 * v4487) + (v4488 * v3017)) - (((v4494 + ((v4496 + v4496) * (v344 / (v416 * v4500)))) * v372) * v4512)) / v4506;
                        v4520 = v4512;
                        v4521 = v4515;
                    } else {
                        let v4516 = v3017 * v4487;
                        let v4519 = (v3023 * v4487) + (v4488 * v3017);
                        v4520 = v4516;
                        v4521 = v4519;
                    }
                    v4363 = v4520;
                    v4364 = v4521;
                } else {
                    v4363 = v63;
                    v4364 = v2435;
                }
                v4355 = v4363;
                v4356 = v4364;
            } else {
                v4355 = v63;
                v4356 = v2435;
            }
            let v4358 = v3021 * v4357;
            let v4359 = v3027 * v4357;
            let v4360 = if v239 > v63 { 1.0 } else { 0.0 };
            let v4556: f64;
            let v4557: Lanes<4>;
            if v4360 != 0.0 {
                let v4523 = v77 + v4522;
                let v4524 = v4523 * v224;
                let v4527 = Lanes([v1630[0], 0.0, v1630[1], v1630[2]]);
                let v4532 = ((v1628 + v1635) + v3019) / v4524;
                let v4533 = (v279 * v4523) * v4532;
                let v4536 = (((v4527 + (Lanes([v1637[0], v1637[1], 0.0, v1637[2]]))) + v3025) - (Lanes([v4533[0], 0.0, 0.0, 0.0]))) / v4524;
                let v4537 = v77 + v4532;
                let v4539 = v4536 * v4537;
                let v4542 = ((v4537 * v4537) + v2571).sqrt();
                let v4548 = v372 * (v4537 + v4542);
                let v4550 = v239 / v4548;
                let v4554 = ((Lanes([v294[0], 0.0, 0.0, 0.0])) - (((v4536 + ((v4539 + v4539) * (v344 / (v416 * v4542)))) * v372) * v4550)) / v4548;
                let v4555 = if v1439 > v63 { 1.0 } else { 0.0 };
                let v4573: f64;
                let v4574: Lanes<4>;
                if v4555 != 0.0 {
                    let v4561 = v1440 * v4550;
                    let v4565 = (v4550 * v1439) * v4564;
                    let v4567 = v4565 * v215;
                    let v4569 = v270 * v4565;
                    let v4571 = ((((v4554 * v1439) + (Lanes([v4561[0], 0.0, v4561[1], v4561[2]]))) * v4564) * v215) + (Lanes([v4569[0], 0.0, 0.0, 0.0]));
                    let v4572 = if v4567 < v2403 { 1.0 } else { 0.0 };
                    let v4596: f64;
                    let v4597: Lanes<4>;
                    if v4572 != 0.0 {
                        let v4578 = v77 - (v372 * v4567);
                        let v4580 = v4550 * v4578;
                        let v4583 = (v4554 * v4578) + (((v4571 * v372) * v332) * v4550);
                        v4596 = v4580;
                        v4597 = v4583;
                    } else {
                        let v4584 = v77 + v4567;
                        let v4585 = v4584.ln();
                        let v4592 = (v4550 * v4585) / v4567;
                        let v4595 = (((v4554 * v4585) + ((v4571 * (v344 / v4584)) * v4550)) - (v4571 * v4592)) / v4567;
                        v4596 = v4592;
                        v4597 = v4595;
                    }
                    v4573 = v4596;
                    v4574 = v4597;
                } else {
                    v4573 = v4550;
                    v4574 = v4554;
                }
                let v4575 = if v3019 > v63 { 1.0 } else { 0.0 };
                let v4613: f64;
                let v4614: Lanes<4>;
                if v4575 != 0.0 {
                    let v4601 = v1628 + (v3019 * v4598);
                    let v4607 = v1628 + v3019;
                    let v4609 = (v4573 * v4601) / v4607;
                    let v4612 = (((v4574 * v4601) + ((v4527 + (v3025 * v4598)) * v4573)) - ((v4527 + v3025) * v4609)) / v4607;
                    v4613 = v4609;
                    v4614 = v4612;
                } else {
                    v4613 = v4573;
                    v4614 = v4574;
                }
                v4556 = v4613;
                v4557 = v4614;
            } else {
                v4556 = v63;
                v4557 = v2435;
            }
            let v4626: f64;
            let v4627: Lanes<3>;
            if v4558 != 0.0 {
                let v4616 = v4615 * v211;
                let v4618 = v29 / v4616;
                let v4619 = (v266 * v4615) * v4618;
                let v4623 = ((Lanes([0.0, v30[0], v30[1]])) - (Lanes([v4619[0], 0.0, 0.0]))) / v4616;
                let v4624 = if v4618 > v1436 { 1.0 } else { 0.0 };
                let v4631: f64;
                let v4632: f64;
                let v4633: Lanes<3>;
                let v4634: Lanes<3>;
                if v4624 != 0.0 {
                    let v4630 = v77 + (v4618 - v1436);
                    v4631 = v4630;
                    v4632 = v1436;
                    v4633 = v4623;
                    v4634 = v4625;
                } else {
                    v4631 = v77;
                    v4632 = v4618;
                    v4633 = v4625;
                    v4634 = v4623;
                }
                let v4635 = rspice_limexp(v4632);
                let v4641 = (v4631 * v4635) - v77;
                let v4642 = v240 * v4641;
                let v4643 = v295 * v4641;
                let v4646 = (Lanes([v4643[0], 0.0, 0.0])) + (((v4633 * v4635) + ((v4634 * v4635) * v4631)) * v240);
                v4626 = v4642;
                v4627 = v4646;
            } else {
                v4626 = v63;
                v4627 = v4625;
            }
            let v4656: f64;
            let v4657: Lanes<3>;
            if v4628 != 0.0 {
                let v4647 = v888 * v211;
                let v4649 = v29 / v4647;
                let v4650 = (v266 * v888) * v4649;
                let v4654 = ((Lanes([0.0, v30[0], v30[1]])) - (Lanes([v4650[0], 0.0, 0.0]))) / v4647;
                let v4655 = if v4649 > v1436 { 1.0 } else { 0.0 };
                let v4661: f64;
                let v4662: f64;
                let v4663: Lanes<3>;
                let v4664: Lanes<3>;
                if v4655 != 0.0 {
                    let v4660 = v77 + (v4649 - v1436);
                    v4661 = v4660;
                    v4662 = v1436;
                    v4663 = v4654;
                    v4664 = v4625;
                } else {
                    v4661 = v77;
                    v4662 = v4649;
                    v4663 = v4625;
                    v4664 = v4654;
                }
                let v4665 = rspice_limexp(v4662);
                let v4671 = (v4661 * v4665) - v77;
                let v4672 = v241 * v4671;
                let v4673 = v296 * v4671;
                let v4676 = (Lanes([v4673[0], 0.0, 0.0])) + (((v4663 * v4665) + ((v4664 * v4665) * v4661)) * v241);
                v4656 = v4672;
                v4657 = v4676;
            } else {
                v4656 = v63;
                v4657 = v4625;
            }
            let v4658 = if v242 > v63 { 1.0 } else { 0.0 };
            let v4785: f64;
            let v4786: f64;
            let v4787: Lanes<3>;
            let v4788: Lanes<3>;
            if v4658 != 0.0 {
                let v4684 = ((-(v244.ln())) / v867).exp();
                let v4686 = v77 - v4684;
                let v4688 = v243 * v4686;
                let v4691 = (v298 * v4686) + ((((((v299 * (v344 / v244)) * v332) / v867) * v4684) * v332) * v243);
                let v4692 = v4688 - v29;
                let v4693 = Lanes([v4691[0], 0.0, 0.0]);
                let v4694 = Lanes([0.0, v30[0], v30[1]]);
                let v4696 = v4692 * v215;
                let v4698 = v270 * v4692;
                let v4700 = ((v4693 - v4694) * v215) + (Lanes([v4698[0], 0.0, 0.0]));
                let v4702 = v4700 * v4696;
                let v4705 = ((v4696 * v4696) + v1546).sqrt();
                let v4708 = (v4702 + v4702) * (v344 / (v416 * v4705));
                let v4711 = (v4696 + v4705) * v372;
                let v4712 = (v4700 + v4708) * v372;
                let v4714 = v266 * v4711;
                let v4718 = v4688 - (v211 * v4711);
                let v4719 = v4693 - ((Lanes([v4714[0], 0.0, 0.0])) + (v4712 * v211));
                let v4720 = v4711 / v4705;
                let v4723 = (v4712 - (v4708 * v4720)) / v4705;
                let v4724 = v4718 / v243;
                let v4725 = v298 * v4724;
                let v4729 = v77 - v4724;
                let v4731 = v4729.ln();
                let v4733 = (((v4719 - (Lanes([v4725[0], 0.0, 0.0]))) / v243) * v332) * (v344 / v4729);
                let v4734 = -v867;
                let v4737 = (v4734 * v4731).exp();
                let v4743 = v77 - v4720;
                let v4746 = v299 * v4743;
                let v4750 = (v4737 * v4720) + (v244 * v4743);
                let v4752 = v242 * v4750;
                let v4753 = v297 * v4750;
                let v4756 = (Lanes([v4753[0], 0.0, 0.0])) + ((((((v4733 * v4734) * v4737) * v4720) + (v4723 * v4737)) + ((Lanes([v4746[0], 0.0, 0.0])) + ((v4723 * v332) * v244))) * v242);
                let v4757 = v77 - v867;
                let v4760 = (v4731 * v4757).exp();
                let v4762 = v77 - v4760;
                let v4765 = v298 * v4762;
                let v4771 = v29 - v4718;
                let v4774 = v299 * v4771;
                let v4778 = ((v243 * v4762) / v4757) + (v244 * v4771);
                let v4780 = v242 * v4778;
                let v4781 = v297 * v4778;
                let v4784 = (Lanes([v4781[0], 0.0, 0.0])) + (((((Lanes([v4765[0], 0.0, 0.0])) + ((((v4733 * v4757) * v4760) * v332) * v243)) / v4757) + ((Lanes([v4774[0], 0.0, 0.0])) + ((v4694 - v4719) * v244))) * v242);
                v4785 = v4752;
                v4786 = v4780;
                v4787 = v4756;
                v4788 = v4784;
            } else {
                v4785 = v63;
                v4786 = v63;
                v4787 = v4625;
                v4788 = v4625;
            }
            let v4794: f64;
            let v4795: Lanes<4>;
            if v105 != 0.0 {
                let v4792 = if (if (if v109 == v77 { 1.0 } else { 0.0 }) != 0.0 && v4658 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v243 > v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v4857: f64;
                let v4858: Lanes<4>;
                if v4792 != 0.0 {
                    let v4809 = v77 - (v77 / v867);
                    let v4810 = v4785 / v242;
                    let v4811 = v297 * v4810;
                    let v4820 = (v4809 * (v4810.ln())).exp();
                    let v4821 = ((((v4787 - (Lanes([v4811[0], 0.0, 0.0]))) / v242) * (v344 / v4810)) * v4809) * v4820;
                    let v4822 = v29 / v243;
                    let v4823 = v298 * v4822;
                    let v4828 = -v4822;
                    let v4830 = v4828 * v245;
                    let v4832 = v300 * v4828;
                    let v4835 = v4830 * v4820;
                    let v4840 = v301 * v332;
                    let v4841 = (-v246) / v4820;
                    let v4846 = v4841.exp();
                    let v4848 = v4835 * v4846;
                    let v4851 = (((((((((Lanes([0.0, v30[0], v30[1]])) - (Lanes([v4823[0], 0.0, 0.0]))) / v243) * v332) * v245) + (Lanes([v4832[0], 0.0, 0.0]))) * v4820) + (v4821 * v4830)) * v4846) + (((((Lanes([v4840[0], 0.0, 0.0])) - (v4821 * v4841)) / v4820) * v4846) * v4835);
                    let v4852 = Lanes([v4851[0], v4851[1], v4851[2], 0.0]);
                    v4857 = v4848;
                    v4858 = v4852;
                } else {
                    let v4856 = if (if (if v109 == v63 { 1.0 } else { 0.0 }) != 0.0 && v1500 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v217 > v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v4903: f64;
                    let v4904: Lanes<3>;
                    if v4856 != 0.0 {
                        let v4860 = v77 - (v77 / v439);
                        let v4861 = v1629 / v216;
                        let v4862 = v271 * v4861;
                        let v4871 = (v4860 * (v4861.ln())).exp();
                        let v4872 = ((((v1631 - (Lanes([v4862[0], 0.0, 0.0]))) / v216) * (v344 / v4861)) * v4860) * v4871;
                        let v4873 = v9 / v217;
                        let v4874 = v272 * v4873;
                        let v4879 = -v4873;
                        let v4881 = v4879 * v245;
                        let v4883 = v300 * v4879;
                        let v4886 = v4881 * v4871;
                        let v4891 = v301 * v332;
                        let v4892 = (-v246) / v4871;
                        let v4897 = v4892.exp();
                        let v4899 = v4886 * v4897;
                        let v4902 = (((((((((Lanes([0.0, v10[0], v10[1]])) - (Lanes([v4874[0], 0.0, 0.0]))) / v217) * v332) * v245) + (Lanes([v4883[0], 0.0, 0.0]))) * v4871) + (v4872 * v4881)) * v4897) + (((((Lanes([v4891[0], 0.0, 0.0])) - (v4872 * v4892)) / v4871) * v4897) * v4886);
                        v4903 = v4899;
                        v4904 = v4902;
                    } else {
                        v4903 = v63;
                        v4904 = v1438;
                    }
                    let v4905 = Lanes([v4904[0], v4904[1], 0.0, v4904[2]]);
                    v4857 = v4903;
                    v4858 = v4905;
                }
                v4794 = v4857;
                v4795 = v4858;
            } else {
                v4794 = v63;
                v4795 = v4793;
            }
            let v4798 = (v9 / v920).exp();
            let v4800 = v4798 - v77;
            let v4801 = v247 * v4800;
            let v4802 = v302 * v4800;
            let v4803 = ((v10 / v920) * v4798) * v247;
            let v4806 = (Lanes([v4802[0], 0.0, 0.0])) + (Lanes([0.0, v4803[0], v4803[1]]));
            let v4908: f64;
            let v4909: Lanes<3>;
            if v4807 != 0.0 {
                let v4906 = if v248 > v63 { 1.0 } else { 0.0 };
                let v4961: f64;
                let v4962: Lanes<3>;
                if v4906 != 0.0 {
                    let v4911 = v1073 / v411;
                    let v4913 = v4912 - v249;
                    let v4914 = v304 * v332;
                    let v4922 = ((-(v250.ln())) / v1073).exp();
                    let v4924 = v77 - v4922;
                    let v4926 = v249 * v4924;
                    let v4929 = (v304 * v4924) + ((((((v305 * (v344 / v250)) * v332) / v1073) * v4922) * v332) * v249);
                    let v4930 = v250 * v248;
                    let v4933 = (v305 * v248) + (v303 * v250);
                    let v4934 = v4911 - v1073;
                    let v4935 = v4912 / v249;
                    let v4944 = (v4934 * (v4935.ln())).exp();
                    let v4946 = v248 * v4944;
                    let v4949 = (v303 * v4944) + (((((((v304 * v4935) * v332) / v249) * (v344 / v4935)) * v4934) * v4944) * v248);
                    let v4950 = v4926 - v35;
                    let v4951 = Lanes([v4929[0], 0.0, 0.0]);
                    let v4952 = Lanes([0.0, v36[0], v36[1]]);
                    let v4954 = v4950 * v215;
                    let v4956 = v270 * v4950;
                    let v4958 = ((v4951 - v4952) * v215) + (Lanes([v4956[0], 0.0, 0.0]));
                    let v4959 = if v4954 < v1436 { 1.0 } else { 0.0 };
                    let v4976: f64;
                    let v4977: Lanes<3>;
                    if v4959 != 0.0 {
                        let v4963 = v4954.exp();
                        let v4965 = v77 + v4963;
                        let v4966 = v4965.ln();
                        let v4970 = v266 * v4966;
                        let v4974 = v4926 - (v211 * v4966);
                        let v4975 = v4951 - ((Lanes([v4970[0], 0.0, 0.0])) + (((v4958 * v4963) * (v344 / v4965)) * v211));
                        v4976 = v4974;
                        v4977 = v4975;
                    } else {
                        v4976 = v35;
                        v4977 = v4952;
                    }
                    let v4982 = (v1715 * v4913) + (v411 * v211);
                    let v4983 = (v4914 * v1715) + (v266 * v411);
                    let v4987 = (v4913 + v4976) / v4982;
                    let v4988 = v4983 * v4987;
                    let v4991 = (((Lanes([v4914[0], 0.0, 0.0])) + v4977) - (Lanes([v4988[0], 0.0, 0.0]))) / v4982;
                    let v4992 = if v4987 < v1436 { 1.0 } else { 0.0 };
                    let v5022: f64;
                    let v5023: Lanes<3>;
                    if v4992 != 0.0 {
                        let v4993 = v4987.exp();
                        let v4995 = v77 + v4993;
                        let v4997 = v4914 * v332;
                        let v5005 = (-(v4913 + v4926)) / v4982;
                        let v5009 = v5005.exp();
                        let v5010 = ((((v4914 + v4929) * v332) - (v4983 * v5005)) / v4982) * v5009;
                        let v5011 = (v4995.ln()) - v5009;
                        let v5015 = v4983 * v5011;
                        let v5019 = (-v4913) + (v4982 * v5011);
                        let v5021 = (Lanes([v4997[0], 0.0, 0.0])) + ((Lanes([v5015[0], 0.0, 0.0])) + ((((v4991 * v4993) * (v344 / v4995)) - (Lanes([v5010[0], 0.0, 0.0]))) * v4982));
                        v5022 = v5019;
                        v5023 = v5021;
                    } else {
                        v5022 = v4976;
                        v5023 = v4977;
                    }
                    let v5024 = v35 - v4976;
                    let v5026 = v4976 / v249;
                    let v5027 = v304 * v5026;
                    let v5031 = v77 - v5026;
                    let v5036 = v5022 / v249;
                    let v5037 = v304 * v5036;
                    let v5041 = v77 - v5036;
                    let v5043 = v5041.ln();
                    let v5045 = (((v5023 - (Lanes([v5037[0], 0.0, 0.0]))) / v249) * v332) * (v344 / v5041);
                    let v5046 = v77 - v1073;
                    let v5047 = v77 - v4911;
                    let v5050 = (v5043 * v5046).exp();
                    let v5052 = v77 - v5050;
                    let v5055 = v303 * v5052;
                    let v5063 = ((v5031.ln()) * v5047).exp();
                    let v5065 = v77 - v5063;
                    let v5068 = v4949 * v5065;
                    let v5076 = (v5043 * v5047).exp();
                    let v5078 = v77 - v5076;
                    let v5081 = v4949 * v5078;
                    let v5089 = (((v248 * v5052) / v5046) + ((v4946 * v5065) / v5047)) - ((v4946 * v5078) / v5047);
                    let v5093 = v304 * v5089;
                    let v5097 = v4933 * v5024;
                    let v5101 = (v5089 * v249) + (v4930 * v5024);
                    let v5102 = (((((((Lanes([v5055[0], 0.0, 0.0])) + ((((v5045 * v5046) * v5050) * v332) * v248)) / v5046) + (((Lanes([v5068[0], 0.0, 0.0])) + ((((((((v4977 - (Lanes([v5027[0], 0.0, 0.0]))) / v249) * v332) * (v344 / v5031)) * v5047) * v5063) * v332) * v4946)) / v5047)) - (((Lanes([v5081[0], 0.0, 0.0])) + ((((v5045 * v5047) * v5076) * v332) * v4946)) / v5047)) * v249) + (Lanes([v5093[0], 0.0, 0.0]))) + ((Lanes([v5097[0], 0.0, 0.0])) + ((v4952 - v4977) * v4930));
                    v4961 = v5101;
                    v4962 = v5102;
                } else {
                    v4961 = v63;
                    v4962 = v4960;
                }
                v4908 = v4961;
                v4909 = v4962;
            } else {
                let v4907 = if v248 > v63 { 1.0 } else { 0.0 };
                let v5184: f64;
                let v5185: Lanes<3>;
                if v4907 != 0.0 {
                    let v5110 = ((-(v250.ln())) / v1073).exp();
                    let v5112 = v77 - v5110;
                    let v5114 = v249 * v5112;
                    let v5117 = (v304 * v5112) + ((((((v305 * (v344 / v250)) * v332) / v1073) * v5110) * v332) * v249);
                    let v5118 = v5114 - v35;
                    let v5119 = Lanes([v5117[0], 0.0, 0.0]);
                    let v5120 = Lanes([0.0, v36[0], v36[1]]);
                    let v5122 = v5118 * v215;
                    let v5124 = v270 * v5118;
                    let v5126 = ((v5119 - v5120) * v215) + (Lanes([v5124[0], 0.0, 0.0]));
                    let v5128 = v5126 * v5122;
                    let v5131 = ((v5122 * v5122) + v1546).sqrt();
                    let v5137 = (v5122 + v5131) * v372;
                    let v5140 = v266 * v5137;
                    let v5144 = v5114 - (v211 * v5137);
                    let v5145 = v5119 - ((Lanes([v5140[0], 0.0, 0.0])) + (((v5126 + ((v5128 + v5128) * (v344 / (v416 * v5131)))) * v372) * v211));
                    let v5146 = v5144 / v249;
                    let v5147 = v304 * v5146;
                    let v5151 = v77 - v5146;
                    let v5156 = v77 - v1073;
                    let v5159 = ((v5151.ln()) * v5156).exp();
                    let v5161 = v77 - v5159;
                    let v5164 = v304 * v5161;
                    let v5170 = v35 - v5144;
                    let v5173 = v305 * v5170;
                    let v5177 = ((v249 * v5161) / v5156) + (v250 * v5170);
                    let v5179 = v248 * v5177;
                    let v5180 = v303 * v5177;
                    let v5183 = (Lanes([v5180[0], 0.0, 0.0])) + (((((Lanes([v5164[0], 0.0, 0.0])) + ((((((((v5145 - (Lanes([v5147[0], 0.0, 0.0]))) / v249) * v332) * (v344 / v5151)) * v5156) * v5159) * v332) * v249)) / v5156) + ((Lanes([v5173[0], 0.0, 0.0])) + ((v5120 - v5145) * v250))) * v248);
                    v5184 = v5179;
                    v5185 = v5183;
                } else {
                    v5184 = v63;
                    v5185 = v4960;
                }
                v4908 = v5184;
                v4909 = v5185;
            }
            let v5196: f64;
            let v5197: Lanes<3>;
            if v4910 != 0.0 {
                let v5187 = v5186 * v211;
                let v5189 = v35 / v5187;
                let v5190 = (v266 * v5186) * v5189;
                let v5194 = ((Lanes([0.0, v36[0], v36[1]])) - (Lanes([v5190[0], 0.0, 0.0]))) / v5187;
                let v5195 = if v5189 > v1436 { 1.0 } else { 0.0 };
                let v5200: f64;
                let v5201: f64;
                let v5202: Lanes<3>;
                let v5203: Lanes<3>;
                if v5195 != 0.0 {
                    let v5199 = v77 + (v5189 - v1436);
                    v5200 = v5199;
                    v5201 = v1436;
                    v5202 = v5194;
                    v5203 = v4960;
                } else {
                    v5200 = v77;
                    v5201 = v5189;
                    v5202 = v4960;
                    v5203 = v5194;
                }
                let v5204 = rspice_limexp(v5201);
                let v5210 = (v5200 * v5204) - v77;
                let v5211 = v251 * v5210;
                let v5212 = v306 * v5210;
                let v5215 = (Lanes([v5212[0], 0.0, 0.0])) + (((v5202 * v5204) + ((v5203 * v5204) * v5200)) * v251);
                v5196 = v5211;
                v5197 = v5215;
            } else {
                v5196 = v63;
                v5197 = v4960;
            }
            let v5218: f64;
            let v5219: Lanes<3>;
            if v4807 != 0.0 {
                let v5216 = if v252 > v63 { 1.0 } else { 0.0 };
                let v5270: f64;
                let v5271: Lanes<3>;
                if v5216 != 0.0 {
                    let v5221 = v1073 / v411;
                    let v5222 = v4912 - v249;
                    let v5223 = v304 * v332;
                    let v5231 = ((-(v250.ln())) / v1073).exp();
                    let v5233 = v77 - v5231;
                    let v5235 = v249 * v5233;
                    let v5238 = (v304 * v5233) + ((((((v305 * (v344 / v250)) * v332) / v1073) * v5231) * v332) * v249);
                    let v5239 = v250 * v252;
                    let v5242 = (v305 * v252) + (v307 * v250);
                    let v5243 = v5221 - v1073;
                    let v5244 = v4912 / v249;
                    let v5253 = (v5243 * (v5244.ln())).exp();
                    let v5255 = v252 * v5253;
                    let v5258 = (v307 * v5253) + (((((((v304 * v5244) * v332) / v249) * (v344 / v5244)) * v5243) * v5253) * v252);
                    let v5259 = v5235 - v43;
                    let v5260 = Lanes([0.0, v5238[0], 0.0]);
                    let v5261 = Lanes([v44[0], 0.0, v44[1]]);
                    let v5263 = v5259 * v215;
                    let v5265 = v270 * v5259;
                    let v5267 = ((v5260 - v5261) * v215) + (Lanes([0.0, v5265[0], 0.0]));
                    let v5268 = if v5263 < v1436 { 1.0 } else { 0.0 };
                    let v5285: f64;
                    let v5286: Lanes<3>;
                    if v5268 != 0.0 {
                        let v5272 = v5263.exp();
                        let v5274 = v77 + v5272;
                        let v5275 = v5274.ln();
                        let v5279 = v266 * v5275;
                        let v5283 = v5235 - (v211 * v5275);
                        let v5284 = v5260 - ((Lanes([0.0, v5279[0], 0.0])) + (((v5267 * v5272) * (v344 / v5274)) * v211));
                        v5285 = v5283;
                        v5286 = v5284;
                    } else {
                        v5285 = v43;
                        v5286 = v5261;
                    }
                    let v5291 = (v1715 * v5222) + (v411 * v211);
                    let v5292 = (v5223 * v1715) + (v266 * v411);
                    let v5296 = (v5222 + v5285) / v5291;
                    let v5297 = v5292 * v5296;
                    let v5300 = (((Lanes([0.0, v5223[0], 0.0])) + v5286) - (Lanes([0.0, v5297[0], 0.0]))) / v5291;
                    let v5301 = if v5296 < v1436 { 1.0 } else { 0.0 };
                    let v5331: f64;
                    let v5332: Lanes<3>;
                    if v5301 != 0.0 {
                        let v5302 = v5296.exp();
                        let v5304 = v77 + v5302;
                        let v5306 = v5223 * v332;
                        let v5314 = (-(v5222 + v5235)) / v5291;
                        let v5318 = v5314.exp();
                        let v5319 = ((((v5223 + v5238) * v332) - (v5292 * v5314)) / v5291) * v5318;
                        let v5320 = (v5304.ln()) - v5318;
                        let v5324 = v5292 * v5320;
                        let v5328 = (-v5222) + (v5291 * v5320);
                        let v5330 = (Lanes([0.0, v5306[0], 0.0])) + ((Lanes([0.0, v5324[0], 0.0])) + ((((v5300 * v5302) * (v344 / v5304)) - (Lanes([0.0, v5319[0], 0.0]))) * v5291));
                        v5331 = v5328;
                        v5332 = v5330;
                    } else {
                        v5331 = v5285;
                        v5332 = v5286;
                    }
                    let v5333 = v43 - v5285;
                    let v5335 = v5285 / v249;
                    let v5336 = v304 * v5335;
                    let v5340 = v77 - v5335;
                    let v5345 = v5331 / v249;
                    let v5346 = v304 * v5345;
                    let v5350 = v77 - v5345;
                    let v5352 = v5350.ln();
                    let v5354 = (((v5332 - (Lanes([0.0, v5346[0], 0.0]))) / v249) * v332) * (v344 / v5350);
                    let v5355 = v77 - v1073;
                    let v5356 = v77 - v5221;
                    let v5359 = (v5352 * v5355).exp();
                    let v5361 = v77 - v5359;
                    let v5364 = v307 * v5361;
                    let v5372 = ((v5340.ln()) * v5356).exp();
                    let v5374 = v77 - v5372;
                    let v5377 = v5258 * v5374;
                    let v5385 = (v5352 * v5356).exp();
                    let v5387 = v77 - v5385;
                    let v5390 = v5258 * v5387;
                    let v5398 = (((v252 * v5361) / v5355) + ((v5255 * v5374) / v5356)) - ((v5255 * v5387) / v5356);
                    let v5402 = v304 * v5398;
                    let v5406 = v5242 * v5333;
                    let v5410 = (v5398 * v249) + (v5239 * v5333);
                    let v5411 = (((((((Lanes([0.0, v5364[0], 0.0])) + ((((v5354 * v5355) * v5359) * v332) * v252)) / v5355) + (((Lanes([0.0, v5377[0], 0.0])) + ((((((((v5286 - (Lanes([0.0, v5336[0], 0.0]))) / v249) * v332) * (v344 / v5340)) * v5356) * v5372) * v332) * v5255)) / v5356)) - (((Lanes([0.0, v5390[0], 0.0])) + ((((v5354 * v5356) * v5385) * v332) * v5255)) / v5356)) * v249) + (Lanes([0.0, v5402[0], 0.0]))) + ((Lanes([0.0, v5406[0], 0.0])) + ((v5261 - v5286) * v5239));
                    v5270 = v5410;
                    v5271 = v5411;
                } else {
                    v5270 = v63;
                    v5271 = v5269;
                }
                v5218 = v5270;
                v5219 = v5271;
            } else {
                let v5217 = if v252 > v63 { 1.0 } else { 0.0 };
                let v5493: f64;
                let v5494: Lanes<3>;
                if v5217 != 0.0 {
                    let v5419 = ((-(v250.ln())) / v1073).exp();
                    let v5421 = v77 - v5419;
                    let v5423 = v249 * v5421;
                    let v5426 = (v304 * v5421) + ((((((v305 * (v344 / v250)) * v332) / v1073) * v5419) * v332) * v249);
                    let v5427 = v5423 - v43;
                    let v5428 = Lanes([0.0, v5426[0], 0.0]);
                    let v5429 = Lanes([v44[0], 0.0, v44[1]]);
                    let v5431 = v5427 * v215;
                    let v5433 = v270 * v5427;
                    let v5435 = ((v5428 - v5429) * v215) + (Lanes([0.0, v5433[0], 0.0]));
                    let v5437 = v5435 * v5431;
                    let v5440 = ((v5431 * v5431) + v1546).sqrt();
                    let v5446 = (v5431 + v5440) * v372;
                    let v5449 = v266 * v5446;
                    let v5453 = v5423 - (v211 * v5446);
                    let v5454 = v5428 - ((Lanes([0.0, v5449[0], 0.0])) + (((v5435 + ((v5437 + v5437) * (v344 / (v416 * v5440)))) * v372) * v211));
                    let v5455 = v5453 / v249;
                    let v5456 = v304 * v5455;
                    let v5460 = v77 - v5455;
                    let v5465 = v77 - v1073;
                    let v5468 = ((v5460.ln()) * v5465).exp();
                    let v5470 = v77 - v5468;
                    let v5473 = v304 * v5470;
                    let v5479 = v43 - v5453;
                    let v5482 = v305 * v5479;
                    let v5486 = ((v249 * v5470) / v5465) + (v250 * v5479);
                    let v5488 = v252 * v5486;
                    let v5489 = v307 * v5486;
                    let v5492 = (Lanes([0.0, v5489[0], 0.0])) + (((((Lanes([0.0, v5473[0], 0.0])) + ((((((((v5454 - (Lanes([0.0, v5456[0], 0.0]))) / v249) * v332) * (v344 / v5460)) * v5465) * v5468) * v332) * v249)) / v5465) + ((Lanes([0.0, v5482[0], 0.0])) + ((v5429 - v5454) * v250))) * v252);
                    v5493 = v5488;
                    v5494 = v5492;
                } else {
                    v5493 = v63;
                    v5494 = v5269;
                }
                v5218 = v5493;
                v5219 = v5494;
            }
            let v5497: f64;
            let v5498: Lanes<3>;
            if v5220 != 0.0 {
                let v5495 = if v253 > v63 { 1.0 } else { 0.0 };
                let v5549: f64;
                let v5550: Lanes<3>;
                if v5495 != 0.0 {
                    let v5499 = v1202 / v411;
                    let v5501 = v5500 - v254;
                    let v5502 = v309 * v332;
                    let v5510 = ((-(v255.ln())) / v1202).exp();
                    let v5512 = v77 - v5510;
                    let v5514 = v254 * v5512;
                    let v5517 = (v309 * v5512) + ((((((v310 * (v344 / v255)) * v332) / v1202) * v5510) * v332) * v254);
                    let v5518 = v255 * v253;
                    let v5521 = (v310 * v253) + (v308 * v255);
                    let v5522 = v5499 - v1202;
                    let v5523 = v5500 / v254;
                    let v5532 = (v5522 * (v5523.ln())).exp();
                    let v5534 = v253 * v5532;
                    let v5537 = (v308 * v5532) + (((((((v309 * v5523) * v332) / v254) * (v344 / v5523)) * v5522) * v5532) * v253);
                    let v5538 = v5514 - v51;
                    let v5539 = Lanes([v5517[0], 0.0, 0.0]);
                    let v5540 = Lanes([0.0, v52[0], v52[1]]);
                    let v5542 = v5538 * v215;
                    let v5544 = v270 * v5538;
                    let v5546 = ((v5539 - v5540) * v215) + (Lanes([v5544[0], 0.0, 0.0]));
                    let v5547 = if v5542 < v1436 { 1.0 } else { 0.0 };
                    let v5564: f64;
                    let v5565: Lanes<3>;
                    if v5547 != 0.0 {
                        let v5551 = v5542.exp();
                        let v5553 = v77 + v5551;
                        let v5554 = v5553.ln();
                        let v5558 = v266 * v5554;
                        let v5562 = v5514 - (v211 * v5554);
                        let v5563 = v5539 - ((Lanes([v5558[0], 0.0, 0.0])) + (((v5546 * v5551) * (v344 / v5553)) * v211));
                        v5564 = v5562;
                        v5565 = v5563;
                    } else {
                        v5564 = v51;
                        v5565 = v5540;
                    }
                    let v5570 = (v1715 * v5501) + (v411 * v211);
                    let v5571 = (v5502 * v1715) + (v266 * v411);
                    let v5575 = (v5501 + v5564) / v5570;
                    let v5576 = v5571 * v5575;
                    let v5579 = (((Lanes([v5502[0], 0.0, 0.0])) + v5565) - (Lanes([v5576[0], 0.0, 0.0]))) / v5570;
                    let v5580 = if v5575 < v1436 { 1.0 } else { 0.0 };
                    let v5610: f64;
                    let v5611: Lanes<3>;
                    if v5580 != 0.0 {
                        let v5581 = v5575.exp();
                        let v5583 = v77 + v5581;
                        let v5585 = v5502 * v332;
                        let v5593 = (-(v5501 + v5514)) / v5570;
                        let v5597 = v5593.exp();
                        let v5598 = ((((v5502 + v5517) * v332) - (v5571 * v5593)) / v5570) * v5597;
                        let v5599 = (v5583.ln()) - v5597;
                        let v5603 = v5571 * v5599;
                        let v5607 = (-v5501) + (v5570 * v5599);
                        let v5609 = (Lanes([v5585[0], 0.0, 0.0])) + ((Lanes([v5603[0], 0.0, 0.0])) + ((((v5579 * v5581) * (v344 / v5583)) - (Lanes([v5598[0], 0.0, 0.0]))) * v5570));
                        v5610 = v5607;
                        v5611 = v5609;
                    } else {
                        v5610 = v5564;
                        v5611 = v5565;
                    }
                    let v5612 = v51 - v5564;
                    let v5614 = v5564 / v254;
                    let v5615 = v309 * v5614;
                    let v5619 = v77 - v5614;
                    let v5624 = v5610 / v254;
                    let v5625 = v309 * v5624;
                    let v5629 = v77 - v5624;
                    let v5631 = v5629.ln();
                    let v5633 = (((v5611 - (Lanes([v5625[0], 0.0, 0.0]))) / v254) * v332) * (v344 / v5629);
                    let v5634 = v77 - v1202;
                    let v5635 = v77 - v5499;
                    let v5638 = (v5631 * v5634).exp();
                    let v5640 = v77 - v5638;
                    let v5643 = v308 * v5640;
                    let v5651 = ((v5619.ln()) * v5635).exp();
                    let v5653 = v77 - v5651;
                    let v5656 = v5537 * v5653;
                    let v5664 = (v5631 * v5635).exp();
                    let v5666 = v77 - v5664;
                    let v5669 = v5537 * v5666;
                    let v5677 = (((v253 * v5640) / v5634) + ((v5534 * v5653) / v5635)) - ((v5534 * v5666) / v5635);
                    let v5681 = v309 * v5677;
                    let v5685 = v5521 * v5612;
                    let v5689 = (v5677 * v254) + (v5518 * v5612);
                    let v5690 = (((((((Lanes([v5643[0], 0.0, 0.0])) + ((((v5633 * v5634) * v5638) * v332) * v253)) / v5634) + (((Lanes([v5656[0], 0.0, 0.0])) + ((((((((v5565 - (Lanes([v5615[0], 0.0, 0.0]))) / v254) * v332) * (v344 / v5619)) * v5635) * v5651) * v332) * v5534)) / v5635)) - (((Lanes([v5669[0], 0.0, 0.0])) + ((((v5633 * v5635) * v5664) * v332) * v5534)) / v5635)) * v254) + (Lanes([v5681[0], 0.0, 0.0]))) + ((Lanes([v5685[0], 0.0, 0.0])) + ((v5540 - v5565) * v5518));
                    v5549 = v5689;
                    v5550 = v5690;
                } else {
                    v5549 = v63;
                    v5550 = v5548;
                }
                v5497 = v5549;
                v5498 = v5550;
            } else {
                let v5496 = if v253 > v63 { 1.0 } else { 0.0 };
                let v5772: f64;
                let v5773: Lanes<3>;
                if v5496 != 0.0 {
                    let v5698 = ((-(v255.ln())) / v1202).exp();
                    let v5700 = v77 - v5698;
                    let v5702 = v254 * v5700;
                    let v5705 = (v309 * v5700) + ((((((v310 * (v344 / v255)) * v332) / v1202) * v5698) * v332) * v254);
                    let v5706 = v5702 - v51;
                    let v5707 = Lanes([v5705[0], 0.0, 0.0]);
                    let v5708 = Lanes([0.0, v52[0], v52[1]]);
                    let v5710 = v5706 * v215;
                    let v5712 = v270 * v5706;
                    let v5714 = ((v5707 - v5708) * v215) + (Lanes([v5712[0], 0.0, 0.0]));
                    let v5716 = v5714 * v5710;
                    let v5719 = ((v5710 * v5710) + v1546).sqrt();
                    let v5725 = (v5710 + v5719) * v372;
                    let v5728 = v266 * v5725;
                    let v5732 = v5702 - (v211 * v5725);
                    let v5733 = v5707 - ((Lanes([v5728[0], 0.0, 0.0])) + (((v5714 + ((v5716 + v5716) * (v344 / (v416 * v5719)))) * v372) * v211));
                    let v5734 = v5732 / v254;
                    let v5735 = v309 * v5734;
                    let v5739 = v77 - v5734;
                    let v5744 = v77 - v1202;
                    let v5747 = ((v5739.ln()) * v5744).exp();
                    let v5749 = v77 - v5747;
                    let v5752 = v309 * v5749;
                    let v5758 = v51 - v5732;
                    let v5761 = v310 * v5758;
                    let v5765 = ((v254 * v5749) / v5744) + (v255 * v5758);
                    let v5767 = v253 * v5765;
                    let v5768 = v308 * v5765;
                    let v5771 = (Lanes([v5768[0], 0.0, 0.0])) + (((((Lanes([v5752[0], 0.0, 0.0])) + ((((((((v5733 - (Lanes([v5735[0], 0.0, 0.0]))) / v254) * v332) * (v344 / v5739)) * v5744) * v5747) * v332) * v254)) / v5744) + ((Lanes([v5761[0], 0.0, 0.0])) + ((v5708 - v5733) * v255))) * v253);
                    v5772 = v5767;
                    v5773 = v5771;
                } else {
                    v5772 = v63;
                    v5773 = v5548;
                }
                v5497 = v5772;
                v5498 = v5773;
            }
            let v5778: f64;
            let v5779: Lanes<3>;
            if v157 != 0.0 {
                let v5783: f64;
                let v5784: Lanes<3>;
                if v5774 != 0.0 {
                    let v5781 = if v256 > v63 { 1.0 } else { 0.0 };
                    let v5835: f64;
                    let v5836: Lanes<3>;
                    if v5781 != 0.0 {
                        let v5785 = v1404 / v411;
                        let v5787 = v5786 - v257;
                        let v5788 = v312 * v332;
                        let v5796 = ((-(v258.ln())) / v1404).exp();
                        let v5798 = v77 - v5796;
                        let v5800 = v257 * v5798;
                        let v5803 = (v312 * v5798) + ((((((v313 * (v344 / v258)) * v332) / v1404) * v5796) * v332) * v257);
                        let v5804 = v258 * v256;
                        let v5807 = (v313 * v256) + (v311 * v258);
                        let v5808 = v5785 - v1404;
                        let v5809 = v5786 / v257;
                        let v5818 = (v5808 * (v5809.ln())).exp();
                        let v5820 = v256 * v5818;
                        let v5823 = (v311 * v5818) + (((((((v312 * v5809) * v332) / v257) * (v344 / v5809)) * v5808) * v5818) * v256);
                        let v5824 = v5800 - v61;
                        let v5825 = Lanes([0.0, 0.0, v5803[0]]);
                        let v5826 = Lanes([v62[0], v62[1], 0.0]);
                        let v5828 = v5824 * v215;
                        let v5830 = v270 * v5824;
                        let v5832 = ((v5825 - v5826) * v215) + (Lanes([0.0, 0.0, v5830[0]]));
                        let v5833 = if v5828 < v1436 { 1.0 } else { 0.0 };
                        let v5850: f64;
                        let v5851: Lanes<3>;
                        if v5833 != 0.0 {
                            let v5837 = v5828.exp();
                            let v5839 = v77 + v5837;
                            let v5840 = v5839.ln();
                            let v5844 = v266 * v5840;
                            let v5848 = v5800 - (v211 * v5840);
                            let v5849 = v5825 - ((Lanes([0.0, 0.0, v5844[0]])) + (((v5832 * v5837) * (v344 / v5839)) * v211));
                            v5850 = v5848;
                            v5851 = v5849;
                        } else {
                            v5850 = v61;
                            v5851 = v5826;
                        }
                        let v5856 = (v1715 * v5787) + (v411 * v211);
                        let v5857 = (v5788 * v1715) + (v266 * v411);
                        let v5861 = (v5787 + v5850) / v5856;
                        let v5862 = v5857 * v5861;
                        let v5865 = (((Lanes([0.0, 0.0, v5788[0]])) + v5851) - (Lanes([0.0, 0.0, v5862[0]]))) / v5856;
                        let v5866 = if v5861 < v1436 { 1.0 } else { 0.0 };
                        let v5896: f64;
                        let v5897: Lanes<3>;
                        if v5866 != 0.0 {
                            let v5867 = v5861.exp();
                            let v5869 = v77 + v5867;
                            let v5871 = v5788 * v332;
                            let v5879 = (-(v5787 + v5800)) / v5856;
                            let v5883 = v5879.exp();
                            let v5884 = ((((v5788 + v5803) * v332) - (v5857 * v5879)) / v5856) * v5883;
                            let v5885 = (v5869.ln()) - v5883;
                            let v5889 = v5857 * v5885;
                            let v5893 = (-v5787) + (v5856 * v5885);
                            let v5895 = (Lanes([0.0, 0.0, v5871[0]])) + ((Lanes([0.0, 0.0, v5889[0]])) + ((((v5865 * v5867) * (v344 / v5869)) - (Lanes([0.0, 0.0, v5884[0]]))) * v5856));
                            v5896 = v5893;
                            v5897 = v5895;
                        } else {
                            v5896 = v5850;
                            v5897 = v5851;
                        }
                        let v5898 = v61 - v5850;
                        let v5900 = v5850 / v257;
                        let v5901 = v312 * v5900;
                        let v5905 = v77 - v5900;
                        let v5910 = v5896 / v257;
                        let v5911 = v312 * v5910;
                        let v5915 = v77 - v5910;
                        let v5917 = v5915.ln();
                        let v5919 = (((v5897 - (Lanes([0.0, 0.0, v5911[0]]))) / v257) * v332) * (v344 / v5915);
                        let v5920 = v77 - v1404;
                        let v5921 = v77 - v5785;
                        let v5924 = (v5917 * v5920).exp();
                        let v5926 = v77 - v5924;
                        let v5929 = v311 * v5926;
                        let v5937 = ((v5905.ln()) * v5921).exp();
                        let v5939 = v77 - v5937;
                        let v5942 = v5823 * v5939;
                        let v5950 = (v5917 * v5921).exp();
                        let v5952 = v77 - v5950;
                        let v5955 = v5823 * v5952;
                        let v5963 = (((v256 * v5926) / v5920) + ((v5820 * v5939) / v5921)) - ((v5820 * v5952) / v5921);
                        let v5967 = v312 * v5963;
                        let v5971 = v5807 * v5898;
                        let v5975 = (v5963 * v257) + (v5804 * v5898);
                        let v5976 = (((((((Lanes([0.0, 0.0, v5929[0]])) + ((((v5919 * v5920) * v5924) * v332) * v256)) / v5920) + (((Lanes([0.0, 0.0, v5942[0]])) + ((((((((v5851 - (Lanes([0.0, 0.0, v5901[0]]))) / v257) * v332) * (v344 / v5905)) * v5921) * v5937) * v332) * v5820)) / v5921)) - (((Lanes([0.0, 0.0, v5955[0]])) + ((((v5919 * v5921) * v5950) * v332) * v5820)) / v5921)) * v257) + (Lanes([0.0, 0.0, v5967[0]]))) + ((Lanes([0.0, 0.0, v5971[0]])) + ((v5826 - v5851) * v5804));
                        v5835 = v5975;
                        v5836 = v5976;
                    } else {
                        v5835 = v63;
                        v5836 = v5834;
                    }
                    v5783 = v5835;
                    v5784 = v5836;
                } else {
                    let v5782 = if v256 > v63 { 1.0 } else { 0.0 };
                    let v6058: f64;
                    let v6059: Lanes<3>;
                    if v5782 != 0.0 {
                        let v5984 = ((-(v258.ln())) / v1404).exp();
                        let v5986 = v77 - v5984;
                        let v5988 = v257 * v5986;
                        let v5991 = (v312 * v5986) + ((((((v313 * (v344 / v258)) * v332) / v1404) * v5984) * v332) * v257);
                        let v5992 = v5988 - v61;
                        let v5993 = Lanes([0.0, 0.0, v5991[0]]);
                        let v5994 = Lanes([v62[0], v62[1], 0.0]);
                        let v5996 = v5992 * v215;
                        let v5998 = v270 * v5992;
                        let v6000 = ((v5993 - v5994) * v215) + (Lanes([0.0, 0.0, v5998[0]]));
                        let v6002 = v6000 * v5996;
                        let v6005 = ((v5996 * v5996) + v1546).sqrt();
                        let v6011 = (v5996 + v6005) * v372;
                        let v6014 = v266 * v6011;
                        let v6018 = v5988 - (v211 * v6011);
                        let v6019 = v5993 - ((Lanes([0.0, 0.0, v6014[0]])) + (((v6000 + ((v6002 + v6002) * (v344 / (v416 * v6005)))) * v372) * v211));
                        let v6020 = v6018 / v257;
                        let v6021 = v312 * v6020;
                        let v6025 = v77 - v6020;
                        let v6030 = v77 - v1404;
                        let v6033 = ((v6025.ln()) * v6030).exp();
                        let v6035 = v77 - v6033;
                        let v6038 = v312 * v6035;
                        let v6044 = v61 - v6018;
                        let v6047 = v313 * v6044;
                        let v6051 = ((v257 * v6035) / v6030) + (v258 * v6044);
                        let v6053 = v256 * v6051;
                        let v6054 = v311 * v6051;
                        let v6057 = (Lanes([0.0, 0.0, v6054[0]])) + (((((Lanes([0.0, 0.0, v6038[0]])) + ((((((((v6019 - (Lanes([0.0, 0.0, v6021[0]]))) / v257) * v332) * (v344 / v6025)) * v6030) * v6033) * v332) * v257)) / v6030) + ((Lanes([0.0, 0.0, v6047[0]])) + ((v5994 - v6019) * v258))) * v256);
                        v6058 = v6053;
                        v6059 = v6057;
                    } else {
                        v6058 = v63;
                        v6059 = v5834;
                    }
                    v5783 = v6058;
                    v5784 = v6059;
                }
                v5778 = v5783;
                v5779 = v5784;
            } else {
                let v5775 = v1302 * v61;
                let v5776 = v62 * v1302;
                let v5777 = Lanes([v5776[0], v5776[1], 0.0]);
                v5778 = v5775;
                v5779 = v5777;
            }
            let v6090: f64;
            let v6091: f64;
            let v6092: Lanes<3>;
            let v6093: Lanes<4>;
            if v5780 != 0.0 {
                let v6061 = v6060 * v211;
                let v6062 = v266 * v6060;
                let v6063 = v35 / v6061;
                let v6064 = v6062 * v6063;
                let v6069 = rspice_limexp(v6063);
                let v6070 = (((Lanes([0.0, v36[0], v36[1]])) - (Lanes([v6064[0], 0.0, 0.0]))) / v6061) * v6069;
                let v6071 = v51 / v6061;
                let v6072 = v6062 * v6071;
                let v6077 = rspice_limexp(v6071);
                let v6078 = (((Lanes([0.0, v52[0], v52[1]])) - (Lanes([v6072[0], 0.0, 0.0]))) / v6061) * v6077;
                let v6079 = v6069 - v6077;
                let v6083 = v259 * v6079;
                let v6084 = v314 * v6079;
                let v6087 = (Lanes([v6084[0], 0.0, 0.0, 0.0])) + (((Lanes([v6070[0], v6070[1], v6070[2], 0.0])) - (Lanes([v6078[0], v6078[1], 0.0, v6078[2]]))) * v259);
                let v6104: f64;
                let v6105: Lanes<3>;
                if v6088 != 0.0 {
                    let v6095 = v260 * v259;
                    let v6099 = v6095 * v6069;
                    let v6100 = ((v315 * v259) + (v314 * v260)) * v6069;
                    let v6103 = (Lanes([v6100[0], 0.0, 0.0])) + (v6070 * v6095);
                    v6104 = v6099;
                    v6105 = v6103;
                } else {
                    v6104 = v63;
                    v6105 = v4960;
                }
                v6090 = v6104;
                v6091 = v6083;
                v6092 = v6105;
                v6093 = v6087;
            } else {
                v6090 = v63;
                v6091 = v63;
                v6092 = v4960;
                v6093 = v6089;
            }
            let v6116: f64;
            let v6117: Lanes<3>;
            if v6094 != 0.0 {
                let v6107 = v6106 * v211;
                let v6109 = v51 / v6107;
                let v6110 = (v266 * v6106) * v6109;
                let v6114 = ((Lanes([0.0, v52[0], v52[1]])) - (Lanes([v6110[0], 0.0, 0.0]))) / v6107;
                let v6115 = if v6109 > v1436 { 1.0 } else { 0.0 };
                let v6121: f64;
                let v6122: f64;
                let v6123: Lanes<3>;
                let v6124: Lanes<3>;
                if v6115 != 0.0 {
                    let v6120 = v77 + (v6109 - v1436);
                    v6121 = v6120;
                    v6122 = v1436;
                    v6123 = v6114;
                    v6124 = v5548;
                } else {
                    v6121 = v77;
                    v6122 = v6109;
                    v6123 = v5548;
                    v6124 = v6114;
                }
                let v6125 = rspice_limexp(v6122);
                let v6131 = (v6121 * v6125) - v77;
                let v6132 = v261 * v6131;
                let v6133 = v316 * v6131;
                let v6136 = (Lanes([v6133[0], 0.0, 0.0])) + (((v6123 * v6125) + ((v6124 * v6125) * v6121)) * v261);
                v6116 = v6132;
                v6117 = v6136;
            } else {
                v6116 = v63;
                v6117 = v5548;
            }
            let v6140: f64;
            let v6141: Lanes<9>;
            if v6118 != 0.0 {
                let v6160: f64;
                let v6161: Lanes<9>;
                if v6137 != 0.0 {
                    let v6144 = v22 * v3029;
                    let v6148 = v220 - v17;
                    let v6152 = ((Lanes([v275[0], 0.0, 0.0])) - v2108) * v4355;
                    let v6156 = (v19 * v3029) + (v6148 * v4355);
                    let v6157 = ((Lanes([0.0, v6144[0], v6144[1], v6144[2]])) + (v3030 * v19)) + ((Lanes([v6152[0], v6152[1], 0.0, v6152[2]])) + (v4356 * v6148));
                    let v6158 = Lanes([0.0, 0.0, 0.0, v6157[0], v6157[1], v6157[2], 0.0, v6157[3], 0.0]);
                    v6160 = v6156;
                    v6161 = v6158;
                } else {
                    let v6223: f64;
                    let v6224: Lanes<9>;
                    if v6159 != 0.0 {
                        let v6163 = v22 * v3029;
                        let v6167 = v220 - v17;
                        let v6171 = ((Lanes([v275[0], 0.0, 0.0])) - v2108) * v4355;
                        let v6179 = v10 * v1439;
                        let v6181 = (v1440 * v9) + (Lanes([0.0, v6179[0], v6179[1]]));
                        let v6187 = v18 * v4280;
                        let v6189 = (v4281 * v17) + (Lanes([0.0, v6187[0], v6187[1]]));
                        let v6192 = ((((Lanes([0.0, v6163[0], v6163[1], v6163[2]])) + (v3030 * v19)) + ((Lanes([v6171[0], v6171[1], 0.0, v6171[2]])) + (v4356 * v6167))) + (Lanes([v6181[0], 0.0, v6181[1], v6181[2]]))) + (Lanes([v6189[0], v6189[1], 0.0, v6189[2]]));
                        let v6195 = v30 * v4626;
                        let v6197 = (v4627 * v29) + (Lanes([0.0, v6195[0], v6195[1]]));
                        let v6204 = v36 * v5196;
                        let v6206 = (v5197 * v35) + (Lanes([0.0, v6204[0], v6204[1]]));
                        let v6209 = ((Lanes([v6192[0], v6192[1], v6192[2], 0.0, v6192[3]])) + (Lanes([v6197[0], 0.0, v6197[1], v6197[2], 0.0]))) + (Lanes([v6206[0], v6206[1], 0.0, v6206[2], 0.0]));
                        let v6212 = v52 * v6116;
                        let v6214 = (v6117 * v51) + (Lanes([0.0, v6212[0], v6212[1]]));
                        let v6215 = ((((((v19 * v3029) + (v6167 * v4355)) + (v1439 * v9)) + (v4280 * v17)) + (v4626 * v29)) + (v5196 * v35)) + (v6116 * v51);
                        let v6218 = (Lanes([v6209[0], v6209[1], v6209[2], v6209[3], v6209[4], 0.0])) + (Lanes([v6214[0], v6214[1], 0.0, 0.0, 0.0, v6214[2]]));
                        let v6222 = if (if v4556 >= v6219 { 1.0 } else { 0.0 }) != 0.0 && (if v4556 > v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v6237: f64;
                        let v6238: Lanes<6>;
                        if v6222 != 0.0 {
                            let v6226 = v3062 * v3059;
                            let v6227 = v6226 + v6226;
                            let v6228 = (v3059 * v3059) / v4556;
                            let v6229 = v4557 * v6228;
                            let v6233 = ((Lanes([0.0, 0.0, 0.0, v6227[0], v6227[1]])) - (Lanes([v6229[0], v6229[1], v6229[2], 0.0, v6229[3]]))) / v4556;
                            let v6234 = v6215 + v6228;
                            let v6236 = v6218 + (Lanes([v6233[0], v6233[1], v6233[2], v6233[3], v6233[4], 0.0]));
                            v6237 = v6234;
                            v6238 = v6236;
                        } else {
                            v6237 = v6215;
                            v6238 = v6218;
                        }
                        let v6241 = if (if v262 >= v6219 { 1.0 } else { 0.0 }) != 0.0 && (if v262 > v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v6262: f64;
                        let v6263: Lanes<7>;
                        if v6241 != 0.0 {
                            let v6243 = v1 - v6242;
                            let v6249 = ((Lanes([0.0, v5[0]])) - (Lanes([v6245[0], 0.0]))) * v6243;
                            let v6250 = v6249 + v6249;
                            let v6251 = (v6243 * v6243) / v262;
                            let v6252 = v317 * v6251;
                            let v6256 = ((Lanes([v6250[0], 0.0, v6250[1]])) - (Lanes([0.0, v6252[0], 0.0]))) / v262;
                            let v6257 = v6237 + v6251;
                            let v6260 = (Lanes([0.0, v6238[0], v6238[1], v6238[2], v6238[3], v6238[4], v6238[5]])) + (Lanes([v6256[0], v6256[1], 0.0, v6256[2], 0.0, 0.0, 0.0]));
                            v6262 = v6257;
                            v6263 = v6260;
                        } else {
                            let v6261 = Lanes([0.0, v6238[0], v6238[1], v6238[2], v6238[3], v6238[4], v6238[5]]);
                            v6262 = v6237;
                            v6263 = v6261;
                        }
                        let v6266 = if (if v263 >= v6219 { 1.0 } else { 0.0 }) != 0.0 && (if v263 > v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v6285: f64;
                        let v6286: Lanes<8>;
                        if v6266 != 0.0 {
                            let v6267 = v11 - v54;
                            let v6272 = ((Lanes([0.0, v14[0]])) - (Lanes([v58[0], 0.0]))) * v6267;
                            let v6273 = v6272 + v6272;
                            let v6274 = (v6267 * v6267) / v263;
                            let v6275 = v318 * v6274;
                            let v6279 = ((Lanes([v6273[0], 0.0, v6273[1]])) - (Lanes([0.0, v6275[0], 0.0]))) / v263;
                            let v6280 = v6262 + v6274;
                            let v6283 = (Lanes([0.0, v6263[0], v6263[1], v6263[2], v6263[3], v6263[4], v6263[5], v6263[6]])) + (Lanes([v6279[0], 0.0, v6279[1], v6279[2], 0.0, 0.0, 0.0, 0.0]));
                            v6285 = v6280;
                            v6286 = v6283;
                        } else {
                            let v6284 = Lanes([0.0, v6263[0], v6263[1], v6263[2], v6263[3], v6263[4], v6263[5], v6263[6]]);
                            v6285 = v6262;
                            v6286 = v6284;
                        }
                        let v6289 = if (if v264 >= v6219 { 1.0 } else { 0.0 }) != 0.0 && (if v264 > v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v6308: f64;
                        let v6309: Lanes<9>;
                        if v6289 != 0.0 {
                            let v6290 = v37 - v23;
                            let v6295 = ((Lanes([v39[0], 0.0])) - (Lanes([0.0, v25[0]]))) * v6290;
                            let v6296 = v6295 + v6295;
                            let v6297 = (v6290 * v6290) / v264;
                            let v6298 = v319 * v6297;
                            let v6302 = ((Lanes([v6296[0], 0.0, v6296[1]])) - (Lanes([0.0, v6298[0], 0.0]))) / v264;
                            let v6303 = v6285 + v6297;
                            let v6306 = (Lanes([v6286[0], 0.0, v6286[1], v6286[2], v6286[3], v6286[4], v6286[5], v6286[6], v6286[7]])) + (Lanes([0.0, v6302[0], 0.0, v6302[1], 0.0, 0.0, v6302[2], 0.0, 0.0]));
                            v6308 = v6303;
                            v6309 = v6306;
                        } else {
                            let v6307 = Lanes([v6286[0], 0.0, v6286[1], v6286[2], v6286[3], v6286[4], v6286[5], v6286[6], v6286[7]]);
                            v6308 = v6285;
                            v6309 = v6307;
                        }
                        v6223 = v6308;
                        v6224 = v6309;
                    } else {
                        v6223 = v63;
                        v6224 = v6139;
                    }
                    v6160 = v6223;
                    v6161 = v6224;
                }
                v6140 = v6160;
                v6141 = v6161;
            } else {
                v6140 = v6138;
                v6141 = v6139;
            }
            let v6378: f64;
            let v6379: f64;
            let v6380: f64;
            let v6381: f64;
            let v6382: f64;
            let v6383: f64;
            let v6384: f64;
            let v6385: f64;
            let v6386: Lanes<5>;
            let v6387: Lanes<5>;
            let v6388: Lanes<6>;
            let v6389: Lanes<1>;
            let v6390: Lanes<6>;
            let v6391: Lanes<1>;
            let v6392: Lanes<5>;
            let v6393: Lanes<1>;
            if v6142 != 0.0 {
                let v6313 = Lanes([0.0, 0.0, 0.0, 0.0, v6312[0]]);
                let v6316 = (v6310 - v3017) / v3022;
                let v6317 = v3028 * v6316;
                let v6321 = v6316 * v731;
                let v6322 = (((v6313 - (Lanes([v3023[0], v3023[1], v3023[2], v3023[3], 0.0]))) - (Lanes([v6317[0], v6317[1], v6317[2], v6317[3], 0.0]))) / v3022) * v731;
                let v6328 = (Lanes([0.0, v6312[0]])) - (Lanes([v6326[0], 0.0]));
                let v6329 = (v6310 - v6323) / v3022;
                let v6330 = v3028 * v6329;
                let v6335 = v6329 * v731;
                let v6336 = (((Lanes([0.0, 0.0, 0.0, 0.0, v6328[0], v6328[1]])) - (Lanes([v6330[0], v6330[1], v6330[2], v6330[3], 0.0, 0.0]))) / v3022) * v731;
                let v6340 = (v6337 * v6323) * v731;
                let v6341 = (v6326 * v6337) * v731;
                let v6347 = ((v6337 * v6310) / v6344) * v731;
                let v6348 = ((v6312 * v6337) / v6344) * v731;
                let v6349 = v731 / v3022;
                let v6354 = v6353 - v3019;
                let v6356 = Lanes([0.0, 0.0, 0.0, 0.0, v6355[0]]);
                let v6359 = v6354 * v6349;
                let v6361 = (((v3028 * v6349) * v332) / v3022) * v6354;
                let v6363 = ((v6356 - (Lanes([v3025[0], v3025[1], v3025[2], v3025[3], 0.0]))) * v6349) + (Lanes([v6361[0], v6361[1], v6361[2], v6361[3], 0.0]));
                let v6367 = (v6364 * v6353) * v731;
                let v6368 = (v6355 * v6364) * v731;
                let v6369 = Lanes([v6322[0], v6322[1], v6322[2], v6322[3], 0.0, v6322[4]]);
                v6378 = v6353;
                v6379 = v6310;
                v6380 = v6321;
                v6381 = v6340;
                v6382 = v6335;
                v6383 = v6347;
                v6384 = v6359;
                v6385 = v6367;
                v6386 = v6356;
                v6387 = v6313;
                v6388 = v6369;
                v6389 = v6341;
                v6390 = v6336;
                v6391 = v6348;
                v6392 = v6363;
                v6393 = v6368;
            } else {
                let v6370 = Lanes([v3025[0], v3025[1], v3025[2], v3025[3], 0.0]);
                let v6371 = Lanes([v3023[0], v3023[1], v3023[2], v3023[3], 0.0]);
                let v6372 = Lanes([0.0, 0.0, 0.0, 0.0, v6326[0], 0.0]);
                let v6373 = Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v6312[0]]);
                let v6374 = Lanes([0.0, 0.0, 0.0, 0.0, v6355[0]]);
                v6378 = v3019;
                v6379 = v3017;
                v6380 = v6323;
                v6381 = v63;
                v6382 = v6310;
                v6383 = v63;
                v6384 = v6353;
                v6385 = v63;
                v6386 = v6370;
                v6387 = v6371;
                v6388 = v6372;
                v6389 = v6375;
                v6390 = v6373;
                v6391 = v6376;
                v6392 = v6374;
                v6393 = v6377;
            }
            let v6397 = (v1440 + v1470) + v4806;
            let v6404 = v7 * v64;
            let v6405 = (v8 * (((v1439 + v1469) + v4801) + v4358)) + (v64 * v2);
            let v6407 = (((Lanes([v6397[0], 0.0, v6397[1], v6397[2]])) + v4359) * v8) + (Lanes([0.0, 0.0, v6404[0], v6404[1]]));
            let v6411 = v8 * (v6378 + v1628);
            let v6412 = (v6386 + (Lanes([v1630[0], 0.0, v1630[1], v1630[2], 0.0]))) * v8;
            let v6413 = ddt(20499, v6411);
            let v6415 = v6412 * v6414;
            let v6422 = v16 * v64;
            let v6423 = (v8 * (v4280 - v4355)) + (v64 * v12);
            let v6425 = (((Lanes([v4281[0], v4281[1], 0.0, v4281[2]])) - v4356) * v8) + (Lanes([0.0, v6422[0], 0.0, v6422[1]]));
            let v6429 = v8 * (v3020 + v1635);
            let v6430 = (v3026 + (Lanes([v1637[0], v1637[1], 0.0, v1637[2]]))) * v8;
            let v6431 = ddt(20514, v6429);
            let v6432 = v6430 * v6414;
            let v6433 = v8 * v6379;
            let v6434 = v6387 * v8;
            let v6435 = v8 * v3018;
            let v6436 = v3024 * v8;
            let v6446: f64;
            let v6447: f64;
            let v6448: f64;
            let v6449: Lanes<5>;
            let v6450: Lanes<5>;
            let v6451: Lanes<5>;
            if v6437 != 0.0 {
                let v6438 = v3059 / v4556;
                let v6439 = v4557 * v6438;
                let v6443 = ((Lanes([0.0, 0.0, 0.0, v3062[0], v3062[1]])) - (Lanes([v6439[0], v6439[1], v6439[2], 0.0, v6439[3]]))) / v4556;
                let v6455: f64;
                let v6456: f64;
                let v6457: Lanes<5>;
                let v6458: Lanes<5>;
                if v6444 != 0.0 {
                    let v6453 = ddt(20535, v3063);
                    let v6454 = v3068 * v6414;
                    v6455 = v6453;
                    v6456 = v3063;
                    v6457 = v6454;
                    v6458 = v3068;
                } else {
                    v6455 = v63;
                    v6456 = v63;
                    v6457 = v6445;
                    v6458 = v6445;
                }
                v6446 = v6438;
                v6447 = v6455;
                v6448 = v6456;
                v6449 = v6443;
                v6450 = v6457;
                v6451 = v6458;
            } else {
                v6446 = v63;
                v6447 = v63;
                v6448 = v63;
                v6449 = v6445;
                v6450 = v6445;
                v6451 = v6445;
            }
            let v6465: f64;
            let v6466: f64;
            let v6467: Lanes<4>;
            let v6468: Lanes<4>;
            if v6452 != 0.0 {
                let v6460 = v6459 * v4794;
                let v6461 = v4795 * v6459;
                v6465 = v6460;
                v6466 = v63;
                v6467 = v6461;
                v6468 = v4793;
            } else {
                let v6463 = v6462 * v4794;
                let v6464 = v4795 * v6462;
                v6465 = v63;
                v6466 = v6463;
                v6467 = v4793;
                v6468 = v6464;
            }
            let v6470 = v6469 * v4303;
            let v6471 = v4304 * v6469;
            let v6474 = v8 * (v4626 + v4656);
            let v6475 = (v4627 + v4657) * v8;
            let v6476 = v8 * v4786;
            let v6477 = v4788 * v8;
            let v6478 = ddt(20560, v6476);
            let v6479 = v6477 * v6414;
            let v6480 = v8 * v5196;
            let v6481 = v5197 * v8;
            let v6484 = v8 * (v4908 + v6090);
            let v6485 = (v4909 + v6092) * v8;
            let v6486 = ddt(20569, v6484);
            let v6487 = v6485 * v6414;
            let v6489 = v6488 * v31;
            let v6490 = v34 * v6488;
            let v6491 = ddt(20573, v6489);
            let v6492 = v6490 * v6414;
            let v6493 = v8 * v5218;
            let v6494 = v5219 * v8;
            let v6495 = ddt(20577, v6493);
            let v6496 = v6494 * v6414;
            let v6498 = v6497 * v38;
            let v6499 = v42 * v6497;
            let v6500 = ddt(20581, v6498);
            let v6501 = v6499 * v6414;
            let v6514: f64;
            let v6515: Lanes<3>;
            if v6502 != 0.0 {
                let v6506 = (Lanes([v39[0], 0.0])) - (Lanes([0.0, v25[0]]));
                let v6507 = (v37 - v23) / v264;
                let v6508 = v319 * v6507;
                let v6512 = ((Lanes([v6506[0], 0.0, v6506[1]])) - (Lanes([0.0, v6508[0], 0.0]))) / v264;
                v6514 = v6507;
                v6515 = v6512;
            } else {
                v6514 = v63;
                v6515 = v6513;
            }
            let v6528: f64;
            let v6529: Lanes<3>;
            if v6516 != 0.0 {
                let v6520 = (Lanes([0.0, v5[0]])) - (Lanes([v6245[0], 0.0]));
                let v6521 = (v1 - v6242) / v262;
                let v6522 = v317 * v6521;
                let v6526 = ((Lanes([v6520[0], 0.0, v6520[1]])) - (Lanes([0.0, v6522[0], 0.0]))) / v262;
                v6528 = v6521;
                v6529 = v6526;
            } else {
                v6528 = v63;
                v6529 = v6527;
            }
            let v6542: f64;
            let v6543: Lanes<3>;
            if v6530 != 0.0 {
                let v6534 = (Lanes([0.0, v14[0]])) - (Lanes([v58[0], 0.0]));
                let v6535 = (v11 - v54) / v263;
                let v6536 = v318 * v6535;
                let v6540 = ((Lanes([v6534[0], 0.0, v6534[1]])) - (Lanes([0.0, v6536[0], 0.0]))) / v263;
                v6542 = v6535;
                v6543 = v6540;
            } else {
                v6542 = v63;
                v6543 = v6541;
            }
            let v6549 = v6548 * (v23 - v6242);
            let v6550 = ((Lanes([0.0, v25[0]])) - (Lanes([v6245[0], 0.0]))) * v6548;
            let v6551 = ddt(20618, v6549);
            let v6552 = v6550 * v6414;
            let v6558 = v6557 * (v37 - v6242);
            let v6559 = ((Lanes([v39[0], 0.0])) - (Lanes([0.0, v6245[0]]))) * v6557;
            let v6560 = ddt(20622, v6558);
            let v6561 = v6559 * v6414;
            let v6567 = v6566 * (v54 - v6242);
            let v6568 = ((Lanes([v58[0], 0.0])) - (Lanes([0.0, v6245[0]]))) * v6566;
            let v6569 = ddt(20626, v6567);
            let v6570 = v6568 * v6414;
            let v6571 = v8 * v6091;
            let v6572 = v6093 * v8;
            let v6575: f64;
            let v6576: f64;
            let v6577: f64;
            let v6578: f64;
            let v6579: Lanes<3>;
            let v6580: Lanes<2>;
            let v6581: Lanes<3>;
            let v6582: Lanes<2>;
            if v2406 != 0.0 {
                let v6597: f64;
                let v6598: f64;
                let v6599: Lanes<3>;
                let v6600: Lanes<2>;
                if v6094 != 0.0 {
                    let v6592 = v8 * v6116;
                    let v6593 = v6117 * v8;
                    let v6594 = v64 * v46;
                    let v6595 = v50 * v64;
                    v6597 = v6592;
                    v6598 = v6594;
                    v6599 = v6593;
                    v6600 = v6595;
                } else {
                    v6597 = v63;
                    v6598 = v63;
                    v6599 = v5548;
                    v6600 = v6596;
                }
                v6575 = v6597;
                v6576 = v6598;
                v6577 = v63;
                v6578 = v63;
                v6579 = v6599;
                v6580 = v6600;
                v6581 = v5548;
                v6582 = v6596;
            } else {
                let v6573 = v8 * v6116;
                let v6574 = v6117 * v8;
                let v6603: f64;
                let v6604: Lanes<2>;
                if v2335 != 0.0 {
                    let v6601 = v64 * v46;
                    let v6602 = v50 * v64;
                    v6603 = v6601;
                    v6604 = v6602;
                } else {
                    v6603 = v63;
                    v6604 = v6596;
                }
                v6575 = v63;
                v6576 = v63;
                v6577 = v6573;
                v6578 = v6603;
                v6579 = v5548;
                v6580 = v6596;
                v6581 = v6574;
                v6582 = v6604;
            }
            let v6583 = v8 * v5497;
            let v6584 = v5498 * v8;
            let v6585 = ddt(20654, v6583);
            let v6586 = v6584 * v6414;
            let v6587 = v8 * v5778;
            let v6588 = v5779 * v8;
            let v6589 = ddt(20658, v6587);
            let v6590 = v6588 * v6414;
            let v6614: f64;
            let v6615: f64;
            let v6616: f64;
            let v6617: Lanes<2>;
            let v6618: Lanes<2>;
            let v6619: Lanes<2>;
            if v6591 != 0.0 {
                let v6605 = v45 - v53;
                let v6608 = (Lanes([0.0, v47[0]])) - (Lanes([v56[0], 0.0]));
                let v6610 = v6605 / v6609;
                let v6611 = v6608 / v6609;
                let v6626: f64;
                let v6627: f64;
                let v6628: Lanes<2>;
                let v6629: Lanes<2>;
                if v6612 != 0.0 {
                    let v6622 = v6621 * v6605;
                    let v6623 = v6608 * v6621;
                    let v6624 = ddt(20675, v6622);
                    let v6625 = v6623 * v6414;
                    v6626 = v6624;
                    v6627 = v6622;
                    v6628 = v6625;
                    v6629 = v6623;
                } else {
                    v6626 = v63;
                    v6627 = v63;
                    v6628 = v6613;
                    v6629 = v6613;
                }
                v6614 = v6610;
                v6615 = v6626;
                v6616 = v6627;
                v6617 = v6611;
                v6618 = v6628;
                v6619 = v6629;
            } else {
                v6614 = v63;
                v6615 = v63;
                v6616 = v63;
                v6617 = v6613;
                v6618 = v6613;
                v6619 = v6613;
            }
            let v6638: f64;
            let v6639: f64;
            let v6640: f64;
            let v6641: Lanes<9>;
            let v6642: Lanes<1>;
            let v6643: Lanes<1>;
            if v6620 != 0.0 {
                let v6630 = v160 / v265;
                let v6633 = (v378 - (v320 * v6630)) / v265;
                let v6634 = v6630 - v6140;
                let v6636 = (Lanes([0.0, 0.0, 0.0, v6633[0], 0.0, 0.0, 0.0, 0.0, 0.0])) - v6141;
                let v6655: f64;
                let v6656: f64;
                let v6657: Lanes<1>;
                let v6658: Lanes<1>;
                if v6637 != 0.0 {
                    let v6651 = v6650 * v160;
                    let v6652 = v378 * v6650;
                    let v6653 = ddt(20699, v6651);
                    let v6654 = v6652 * v6414;
                    v6655 = v6653;
                    v6656 = v6651;
                    v6657 = v6654;
                    v6658 = v6652;
                } else {
                    v6655 = v63;
                    v6656 = v63;
                    v6657 = v210;
                    v6658 = v210;
                }
                v6638 = v6634;
                v6639 = v6655;
                v6640 = v6656;
                v6641 = v6636;
                v6642 = v6657;
                v6643 = v6658;
            } else {
                v6638 = v63;
                v6639 = v63;
                v6640 = v63;
                v6641 = v6139;
                v6642 = v210;
                v6643 = v210;
            }
            let v6644 = ddt(20703, v6381);
            let v6645 = v6389 * v6414;
            let v6646 = ddt(20706, v6383);
            let v6647 = v6391 * v6414;
            let v6648 = ddt(20709, v6385);
            let v6649 = v6393 * v6414;
            let v6669: f64;
            let v6670: f64;
            let v6671: f64;
            let v6672: f64;
            let v6673: f64;
            let v6674: f64;
            let v6675: f64;
            let v6676: f64;
            let v6677: f64;
            let v6678: f64;
            let v6679: Lanes<1>;
            let v6680: Lanes<1>;
            let v6681: Lanes<5>;
            let v6682: Lanes<5>;
            let v6683: Lanes<1>;
            let v6684: Lanes<1>;
            let v6685: Lanes<1>;
            let v6686: Lanes<1>;
            let v6687: Lanes<5>;
            let v6688: Lanes<5>;
            if v6659 != 0.0 {
                let v6660 = if v1439 > v63 { 1.0 } else { 0.0 };
                let v6703: f64;
                let v6704: Lanes<4>;
                if v6660 != 0.0 {
                    let v6697 = v3029 / v1439;
                    let v6698 = v1440 * v6697;
                    let v6701 = (v3030 - (Lanes([v6698[0], 0.0, v6698[1], v6698[2]]))) / v1439;
                    v6703 = v6697;
                    v6704 = v6701;
                } else {
                    v6703 = v6702;
                    v6704 = v2435;
                }
                let v6705 = v3022 * v6337;
                let v6706 = v3028 * v6337;
                let v6708 = v6703 * v6707;
                let v6709 = v6704 * v6707;
                let v6710 = if v6708 > v63 { 1.0 } else { 0.0 };
                let v6719: f64;
                let v6720: Lanes<4>;
                if v6710 != 0.0 {
                    let v6711 = v6708.sqrt();
                    let v6715 = v3022 * v6711;
                    let v6718 = (v3028 * v6711) + ((v6709 * (v344 / (v416 * v6711))) * v3022);
                    v6719 = v6715;
                    v6720 = v6718;
                } else {
                    v6719 = v63;
                    v6720 = v2435;
                }
                let v6721 = -v6661;
                let v6722 = v6667 * v332;
                let v6723 = ddt(20916, v6661);
                let v6725 = v6719 * v6723;
                let v6726 = v6720 * v6723;
                let v6727 = (v6667 * v6414) * v6719;
                let v6730 = (Lanes([v6726[0], v6726[1], v6726[2], v6726[3], 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, v6727[0]]));
                let v6731 = v6719 * v6661;
                let v6732 = v6720 * v6661;
                let v6733 = v6667 * v6719;
                let v6736 = (Lanes([v6732[0], v6732[1], v6732[2], v6732[3], 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, v6733[0]]));
                let v6737 = ddt(20924, v6662);
                let v6739 = v6705 * v6737;
                let v6740 = v6706 * v6737;
                let v6741 = (v6668 * v6414) * v6705;
                let v6744 = (Lanes([v6740[0], v6740[1], v6740[2], v6740[3], 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, v6741[0]]));
                let v6745 = v6705 * v6662;
                let v6746 = v6706 * v6662;
                let v6747 = v6668 * v6705;
                let v6750 = (Lanes([v6746[0], v6746[1], v6746[2], v6746[3], 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, v6747[0]]));
                let v6751 = -v6662;
                let v6752 = v6668 * v332;
                v6669 = v6721;
                v6670 = v6661;
                v6671 = v6725;
                v6672 = v6739;
                v6673 = v6751;
                v6674 = v6662;
                v6675 = v63;
                v6676 = v63;
                v6677 = v6731;
                v6678 = v6745;
                v6679 = v6722;
                v6680 = v6667;
                v6681 = v6730;
                v6682 = v6744;
                v6683 = v6752;
                v6684 = v6668;
                v6685 = v6663;
                v6686 = v6666;
                v6687 = v6736;
                v6688 = v6750;
            } else {
                v6669 = v63;
                v6670 = v63;
                v6671 = v63;
                v6672 = v63;
                v6673 = v63;
                v6674 = v63;
                v6675 = v6661;
                v6676 = v6662;
                v6677 = v63;
                v6678 = v63;
                v6679 = v6663;
                v6680 = v6663;
                v6681 = v6664;
                v6682 = v6665;
                v6683 = v6666;
                v6684 = v6666;
                v6685 = v6667;
                v6686 = v6668;
                v6687 = v6664;
                v6688 = v6665;
            }
            let v6696 = if (((((v6495 + v6500) + v6514) + v6560) + v6692) + v6694) != v63 { 1.0 } else { 0.0 };
            let v6772 = (((((-(v1440[1])) - (v1470[1])) - ((-(v4795[1])) - (v4806[1]))) + (-(v4359[2]))) - (-(v4356[2]))) + ((-(v4627[1])) - (v4657[1]));
            let v6774 = if (v6772.abs()) > v64 { 1.0 } else { 0.0 };
            if v6774 != 0.0 {
            } else {
                let v6775 = if v6772 >= v63 { 1.0 } else { 0.0 };
            }
            let v6781 = -(v4356[1]);
            let v6789 = ((((-(v4281[1])) - (-(v4304[1]))) - v6781) + (-(v4359[1]))) + (-(v5197[1]));
            let v6791 = if (v6789.abs()) > v64 { 1.0 } else { 0.0 };
            if v6791 != 0.0 {
            } else {
                let v6792 = if v6789 >= v63 { 1.0 } else { 0.0 };
            }
            let v6794 = (v3030[1]) - v6781;
            let v6796 = if (v6794.abs()) > v64 { 1.0 } else { 0.0 };
            if v6796 != 0.0 {
            } else {
                let v6797 = if v6794 >= v63 { 1.0 } else { 0.0 };
            }
            let v6798 = v6407[0];
            let v6799 = v6407[1];
            let v6800 = v6407[2];
            let v6801 = v6407[3];
            let v6802 = v6415[0];
            let v6803 = v6415[1];
            let v6804 = v6415[2];
            let v6805 = v6415[3];
            let v6806 = v6415[4];
            let v6807 = v6425[0];
            let v6808 = v6425[1];
            let v6809 = v6425[2];
            let v6810 = v6425[3];
            let v6811 = v6432[0];
            let v6812 = v6432[1];
            let v6813 = v6432[2];
            let v6814 = v6432[3];
            let v6815 = v6434[0];
            let v6816 = v6434[1];
            let v6817 = v6434[2];
            let v6818 = v6434[3];
            let v6819 = v6434[4];
            let v6820 = v6436[0];
            let v6821 = v6436[1];
            let v6822 = v6436[2];
            let v6823 = v6436[3];
            let v6824 = v6449[0];
            let v6825 = v6449[1];
            let v6826 = v6449[2];
            let v6827 = v6449[3];
            let v6828 = v6449[4];
            let v6829 = v6450[0];
            let v6830 = v6450[1];
            let v6831 = v6450[2];
            let v6832 = v6450[3];
            let v6833 = v6450[4];
            let v6834 = v6467[0];
            let v6835 = v6467[1];
            let v6836 = v6467[2];
            let v6837 = v6467[3];
            let v6838 = v6468[0];
            let v6839 = v6468[1];
            let v6840 = v6468[2];
            let v6841 = v6468[3];
            let v6842 = v6471[0];
            let v6843 = v6471[1];
            let v6844 = v6471[2];
            let v6845 = v6475[0];
            let v6846 = v6475[1];
            let v6847 = v6475[2];
            let v6848 = v6479[0];
            let v6849 = v6479[1];
            let v6850 = v6479[2];
            let v6851 = v6481[0];
            let v6852 = v6481[1];
            let v6853 = v6481[2];
            let v6854 = v6487[0];
            let v6855 = v6487[1];
            let v6856 = v6487[2];
            let v6857 = v6492[0];
            let v6858 = v6492[1];
            let v6859 = v6496[0];
            let v6860 = v6496[1];
            let v6861 = v6496[2];
            let v6862 = v6501[0];
            let v6863 = v6501[1];
            let v6864 = v6515[0];
            let v6865 = v6515[1];
            let v6866 = v6515[2];
            let v6867 = v6529[0];
            let v6868 = v6529[1];
            let v6869 = v6529[2];
            let v6870 = v6543[0];
            let v6871 = v6543[1];
            let v6872 = v6543[2];
            let v6873 = v6552[0];
            let v6874 = v6552[1];
            let v6875 = v6561[0];
            let v6876 = v6561[1];
            let v6877 = v6570[0];
            let v6878 = v6570[1];
            let v6879 = v6572[0];
            let v6880 = v6572[1];
            let v6881 = v6572[2];
            let v6882 = v6572[3];
            let v6883 = v6579[0];
            let v6884 = v6579[1];
            let v6885 = v6579[2];
            let v6886 = v6580[0];
            let v6887 = v6580[1];
            let v6888 = v6581[0];
            let v6889 = v6581[1];
            let v6890 = v6581[2];
            let v6891 = v6582[0];
            let v6892 = v6582[1];
            let v6893 = v6586[0];
            let v6894 = v6586[1];
            let v6895 = v6586[2];
            let v6896 = v6590[0];
            let v6897 = v6590[1];
            let v6898 = v6590[2];
            let v6899 = v6617[0];
            let v6900 = v6617[1];
            let v6901 = v6618[0];
            let v6902 = v6618[1];
            let v6903 = v6641[0];
            let v6904 = v6641[1];
            let v6905 = v6641[2];
            let v6906 = v6641[3];
            let v6907 = v6641[4];
            let v6908 = v6641[5];
            let v6909 = v6641[6];
            let v6910 = v6641[7];
            let v6911 = v6641[8];
            let v6912 = v6642[0];
            let v6913 = v6388[0];
            let v6914 = v6388[1];
            let v6915 = v6388[2];
            let v6916 = v6388[3];
            let v6917 = v6388[4];
            let v6918 = v6388[5];
            let v6919 = v6645[0];
            let v6920 = v6390[0];
            let v6921 = v6390[1];
            let v6922 = v6390[2];
            let v6923 = v6390[3];
            let v6924 = v6390[4];
            let v6925 = v6390[5];
            let v6926 = v6647[0];
            let v6927 = v6392[0];
            let v6928 = v6392[1];
            let v6929 = v6392[2];
            let v6930 = v6392[3];
            let v6931 = v6392[4];
            let v6932 = v6649[0];
            let v6933 = v6679[0];
            let v6934 = v6680[0];
            let v6935 = v6681[0];
            let v6936 = v6681[1];
            let v6937 = v6681[2];
            let v6938 = v6681[3];
            let v6939 = v6681[4];
            let v6940 = v6682[0];
            let v6941 = v6682[1];
            let v6942 = v6682[2];
            let v6943 = v6682[3];
            let v6944 = v6682[4];
            let v6945 = v6683[0];
            let v6946 = v6684[0];
            let v6947 = v6685[0];
            let v6948 = v6686[0];
            let v6949 = v6412[0];
            let v6950 = v6412[1];
            let v6951 = v6412[2];
            let v6952 = v6412[3];
            let v6953 = v6412[4];
            let v6954 = v6430[0];
            let v6955 = v6430[1];
            let v6956 = v6430[2];
            let v6957 = v6430[3];
            let v6958 = v6451[0];
            let v6959 = v6451[1];
            let v6960 = v6451[2];
            let v6961 = v6451[3];
            let v6962 = v6451[4];
            let v6963 = v6477[0];
            let v6964 = v6477[1];
            let v6965 = v6477[2];
            let v6966 = v6485[0];
            let v6967 = v6485[1];
            let v6968 = v6485[2];
            let v6969 = v6490[0];
            let v6970 = v6490[1];
            let v6971 = v6494[0];
            let v6972 = v6494[1];
            let v6973 = v6494[2];
            let v6974 = v6499[0];
            let v6975 = v6499[1];
            let v6976 = v6550[0];
            let v6977 = v6550[1];
            let v6978 = v6559[0];
            let v6979 = v6559[1];
            let v6980 = v6568[0];
            let v6981 = v6568[1];
            let v6982 = v6584[0];
            let v6983 = v6584[1];
            let v6984 = v6584[2];
            let v6985 = v6588[0];
            let v6986 = v6588[1];
            let v6987 = v6588[2];
            let v6988 = v6619[0];
            let v6989 = v6619[1];
            let v6990 = v6643[0];
            let v6991 = v6389[0];
            let v6992 = v6391[0];
            let v6993 = v6393[0];
            let v6994 = v6687[0];
            let v6995 = v6687[1];
            let v6996 = v6687[2];
            let v6997 = v6687[3];
            let v6998 = v6687[4];
            let v6999 = v6688[0];
            let v7000 = v6688[1];
            let v7001 = v6688[2];
            let v7002 = v6688[3];
            let v7003 = v6688[4];
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(6),
            multiplicity * (v6405),
            [4, 5, 6, 8],
            [v6798, v6799, v6800, v6801],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (v6413),
            [4, 5, 6, 8, 12],
            [v6802, v6803, v6804, v6805, v6806],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(5),
            multiplicity * (v6423),
            [4, 5, 6, 8],
            [v6807, v6808, v6809, v6810],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(5),
            multiplicity * (v6431),
            [4, 5, 6, 8],
            [v6811, v6812, v6813, v6814],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * (v6433),
            [4, 5, 6, 8, 11],
            [v6815, v6816, v6817, v6818, v6819],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(5),
            multiplicity * (v6435),
            [4, 5, 6, 8],
            [v6820, v6821, v6822, v6823],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(8),
            multiplicity * (v6446),
            [4, 5, 6, 7, 8],
            [v6824, v6825, v6826, v6827, v6828],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(8),
            multiplicity * (v6447),
            [4, 5, 6, 7, 8],
            [v6829, v6830, v6831, v6832, v6833],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(7), Some(8), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            staged[211],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(6),
            multiplicity * (v6465),
            [4, 6, 7, 8],
            [v6834, v6835, v6836, v6837],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(6),
            multiplicity * (v6466),
            [4, 6, 7, 8],
            [v6838, v6839, v6840, v6841],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(8),
            Some(5),
            multiplicity * (v6470),
            [4, 5, 8],
            [v6842, v6843, v6844],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(6),
            multiplicity * (v6474),
            [4, 6, 7],
            [v6845, v6846, v6847],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(6),
            multiplicity * (v6478),
            [4, 6, 7],
            [v6848, v6849, v6850],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(5),
            multiplicity * (v6480),
            [4, 5, 7],
            [v6851, v6852, v6853],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(5),
            multiplicity * (v6486),
            [4, 5, 7],
            [v6854, v6855, v6856],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(7),
            Some(5),
            multiplicity * (v6491),
            [5, 7],
            [v6857, v6858],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(5),
            multiplicity * (v6495),
            [1, 4, 5],
            [v6859, v6860, v6861],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(5),
            multiplicity * (v6500),
            [1, 5],
            [v6862, v6863],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(7),
            multiplicity * (v6514),
            [1, 4, 7],
            [v6864, v6865, v6866],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(1), Some(7), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            staged[212],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(6),
            Some(2),
            multiplicity * (v6528),
            [2, 4, 6],
            [v6867, v6868, v6869],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(6), Some(2), 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            staged[213],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(0),
            multiplicity * (v6542),
            [0, 4, 5],
            [v6870, v6871, v6872],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(5), Some(0), 3, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            3,
            staged[214],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(7),
            Some(2),
            multiplicity * (v6551),
            [2, 7],
            [v6873, v6874],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(2),
            multiplicity * (v6560),
            [1, 2],
            [v6875, v6876],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(0),
            Some(2),
            multiplicity * (v6569),
            [0, 2],
            [v6877, v6878],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(9),
            multiplicity * (v6571),
            [4, 5, 7, 9],
            [v6879, v6880, v6881, v6882],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(9),
            Some(5),
            multiplicity * (v6575),
            [4, 5, 9],
            [v6883, v6884, v6885],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(9),
            Some(5),
            multiplicity * (v6576),
            [5, 9],
            [v6886, v6887],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(9),
            Some(5),
            multiplicity * (v6577),
            [4, 5, 9],
            [v6888, v6889, v6890],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(9),
            Some(5),
            multiplicity * (v6578),
            [5, 9],
            [v6891, v6892],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(9),
            Some(5),
            multiplicity * (v6585),
            [4, 5, 9],
            [v6893, v6894, v6895],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(3),
            Some(0),
            multiplicity * (v6589),
            [0, 3, 4],
            [v6896, v6897, v6898],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(9),
            Some(3),
            multiplicity * (v6614),
            [3, 9],
            [v6899, v6900],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(9),
            Some(3),
            multiplicity * (v6615),
            [3, 9],
            [v6901, v6902],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(9), Some(3), 4, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            4,
            staged[215],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            None,
            multiplicity * (v6638),
            [0, 1, 2, 4, 5, 6, 7, 8, 9],
            [v6903, v6904, v6905, v6906, v6907, v6908, v6909, v6910, v6911],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (v6639),
            [4],
            [v6912],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(4), None, 5, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            5,
            staged[216],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            None,
            multiplicity * (v6380),
            [4, 5, 6, 8, 10, 11],
            [v6913, v6914, v6915, v6916, v6917, v6918],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(10),
            None,
            multiplicity * (v6644),
            [10],
            [v6919],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            None,
            multiplicity * (v6382),
            [4, 5, 6, 8, 10, 11],
            [v6920, v6921, v6922, v6923, v6924, v6925],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(11),
            None,
            multiplicity * (v6646),
            [11],
            [v6926],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(12),
            None,
            multiplicity * (v6384),
            [4, 5, 6, 8, 12],
            [v6927, v6928, v6929, v6930, v6931],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(12),
            None,
            multiplicity * (v6648),
            [12],
            [v6932],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(7),
            multiplicity * (staged[83]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(8),
            multiplicity * (staged[217]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(0),
            multiplicity * (staged[218]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(2),
            multiplicity * (staged[219]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(3),
            multiplicity * (staged[220]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(6),
            multiplicity * (staged[221]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
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
            Some(2),
            multiplicity * (staged[223]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(6),
            multiplicity * (staged[224]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(6),
            multiplicity * (v7004),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(8),
            multiplicity * (v7005),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(5),
            multiplicity * (v7006),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(5),
            multiplicity * (v7007),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(5),
            multiplicity * (v7008),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(5),
            multiplicity * (v7009),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(13),
            None,
            multiplicity * (staged[225]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(13),
            None,
            multiplicity * (v6669),
            [13],
            [v6933],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(8),
            Some(6),
            multiplicity * (v6670),
            [13],
            [v6934],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (v6671),
            [4, 5, 6, 8, 13],
            [v6935, v6936, v6937, v6938, v6939],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (v6672),
            [4, 5, 6, 8, 14],
            [v6940, v6941, v6942, v6943, v6944],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(14),
            None,
            multiplicity * (staged[226]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(14),
            None,
            multiplicity * (v6673),
            [14],
            [v6945],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(5),
            Some(6),
            multiplicity * (v6674),
            [14],
            [v6946],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(6),
            multiplicity * (staged[227]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(6),
            multiplicity * (staged[228]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(13),
            None,
            multiplicity * (v6675),
            [13],
            [v6947],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(14),
            None,
            multiplicity * (v6676),
            [14],
            [v6948],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = v6405;
        self.canonical_reactive[1] = v6411;
        self.canonical_reactive[2] = v6949;
        self.canonical_reactive[3] = v6950;
        self.canonical_reactive[4] = v6951;
        self.canonical_reactive[5] = v6952;
        self.canonical_reactive[6] = v6953;
        self.canonical_reactive[7] = v6423;
        self.canonical_reactive[8] = v6429;
        self.canonical_reactive[9] = v6954;
        self.canonical_reactive[10] = v6955;
        self.canonical_reactive[11] = v6956;
        self.canonical_reactive[12] = v6957;
        self.canonical_reactive[13] = v6433;
        self.canonical_reactive[14] = v6435;
        self.canonical_reactive[15] = v6446;
        self.canonical_reactive[16] = v6448;
        self.canonical_reactive[17] = v6958;
        self.canonical_reactive[18] = v6959;
        self.canonical_reactive[19] = v6960;
        self.canonical_reactive[20] = v6961;
        self.canonical_reactive[21] = v6962;
        self.canonical_reactive[22] = staged[211];
        self.canonical_reactive[23] = v6465;
        self.canonical_reactive[24] = v6466;
        self.canonical_reactive[25] = v6470;
        self.canonical_reactive[26] = v6474;
        self.canonical_reactive[27] = v6476;
        self.canonical_reactive[28] = v6963;
        self.canonical_reactive[29] = v6964;
        self.canonical_reactive[30] = v6965;
        self.canonical_reactive[31] = v6480;
        self.canonical_reactive[32] = v6484;
        self.canonical_reactive[33] = v6966;
        self.canonical_reactive[34] = v6967;
        self.canonical_reactive[35] = v6968;
        self.canonical_reactive[36] = v6489;
        self.canonical_reactive[37] = v6969;
        self.canonical_reactive[38] = v6970;
        self.canonical_reactive[39] = v6493;
        self.canonical_reactive[40] = v6971;
        self.canonical_reactive[41] = v6972;
        self.canonical_reactive[42] = v6973;
        self.canonical_reactive[43] = v6498;
        self.canonical_reactive[44] = v6974;
        self.canonical_reactive[45] = v6975;
        self.canonical_reactive[46] = v6514;
        self.canonical_reactive[47] = staged[212];
        self.canonical_reactive[48] = v6528;
        self.canonical_reactive[49] = staged[213];
        self.canonical_reactive[50] = v6542;
        self.canonical_reactive[51] = staged[214];
        self.canonical_reactive[52] = v6549;
        self.canonical_reactive[53] = v6976;
        self.canonical_reactive[54] = v6977;
        self.canonical_reactive[55] = v6558;
        self.canonical_reactive[56] = v6978;
        self.canonical_reactive[57] = v6979;
        self.canonical_reactive[58] = v6567;
        self.canonical_reactive[59] = v6980;
        self.canonical_reactive[60] = v6981;
        self.canonical_reactive[61] = v6571;
        self.canonical_reactive[62] = v6575;
        self.canonical_reactive[63] = v6576;
        self.canonical_reactive[64] = v6577;
        self.canonical_reactive[65] = v6578;
        self.canonical_reactive[66] = v6583;
        self.canonical_reactive[67] = v6982;
        self.canonical_reactive[68] = v6983;
        self.canonical_reactive[69] = v6984;
        self.canonical_reactive[70] = v6587;
        self.canonical_reactive[71] = v6985;
        self.canonical_reactive[72] = v6986;
        self.canonical_reactive[73] = v6987;
        self.canonical_reactive[74] = v6614;
        self.canonical_reactive[75] = v6616;
        self.canonical_reactive[76] = v6988;
        self.canonical_reactive[77] = v6989;
        self.canonical_reactive[78] = staged[215];
        self.canonical_reactive[79] = v6638;
        self.canonical_reactive[80] = v6640;
        self.canonical_reactive[81] = v6990;
        self.canonical_reactive[82] = staged[216];
        self.canonical_reactive[83] = v6380;
        self.canonical_reactive[84] = v6381;
        self.canonical_reactive[85] = v6991;
        self.canonical_reactive[86] = v6382;
        self.canonical_reactive[87] = v6383;
        self.canonical_reactive[88] = v6992;
        self.canonical_reactive[89] = v6384;
        self.canonical_reactive[90] = v6385;
        self.canonical_reactive[91] = v6993;
        self.canonical_reactive[92] = staged[83];
        self.canonical_reactive[93] = staged[217];
        self.canonical_reactive[94] = staged[218];
        self.canonical_reactive[95] = staged[219];
        self.canonical_reactive[96] = staged[220];
        self.canonical_reactive[97] = staged[221];
        self.canonical_reactive[98] = staged[222];
        self.canonical_reactive[99] = staged[223];
        self.canonical_reactive[100] = staged[224];
        self.canonical_reactive[101] = v7004;
        self.canonical_reactive[102] = v7005;
        self.canonical_reactive[103] = v7006;
        self.canonical_reactive[104] = v7007;
        self.canonical_reactive[105] = v7008;
        self.canonical_reactive[106] = v7009;
        self.canonical_reactive[107] = staged[225];
        self.canonical_reactive[108] = v6669;
        self.canonical_reactive[109] = v6670;
        self.canonical_reactive[110] = v6677;
        self.canonical_reactive[111] = v6994;
        self.canonical_reactive[112] = v6995;
        self.canonical_reactive[113] = v6996;
        self.canonical_reactive[114] = v6997;
        self.canonical_reactive[115] = v6998;
        self.canonical_reactive[116] = v6678;
        self.canonical_reactive[117] = v6999;
        self.canonical_reactive[118] = v7000;
        self.canonical_reactive[119] = v7001;
        self.canonical_reactive[120] = v7002;
        self.canonical_reactive[121] = v7003;
        self.canonical_reactive[122] = staged[226];
        self.canonical_reactive[123] = v6673;
        self.canonical_reactive[124] = v6674;
        self.canonical_reactive[125] = staged[227];
        self.canonical_reactive[126] = staged[228];
        self.canonical_reactive[127] = v6675;
        self.canonical_reactive[128] = v6676;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(6),
            &[4, 5, 6, 8, 12],
            &[cached[2], cached[3], cached[4], cached[5], cached[6]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(5),
            &[4, 5, 6, 8],
            &[cached[9], cached[10], cached[11], cached[12]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(8),
            &[4, 5, 6, 7, 8],
            &[cached[17], cached[18], cached[19], cached[20], cached[21]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(6),
            &[4, 6, 7],
            &[cached[28], cached[29], cached[30]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(5),
            &[4, 5, 7],
            &[cached[33], cached[34], cached[35]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(5),
            &[5, 7],
            &[cached[37], cached[38]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(5),
            &[1, 4, 5],
            &[cached[40], cached[41], cached[42]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(5),
            &[1, 5],
            &[cached[44], cached[45]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(2),
            &[2, 7],
            &[cached[53], cached[54]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(2),
            &[1, 2],
            &[cached[56], cached[57]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(0),
            Some(2),
            &[0, 2],
            &[cached[59], cached[60]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(9),
            Some(5),
            &[4, 5, 9],
            &[cached[67], cached[68], cached[69]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            Some(0),
            &[0, 3, 4],
            &[cached[71], cached[72], cached[73]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(9),
            Some(3),
            &[3, 9],
            &[cached[76], cached[77]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            None,
            &[4],
            &[cached[81]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(10),
            None,
            &[10],
            &[cached[85]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(11),
            None,
            &[11],
            &[cached[88]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(12),
            None,
            &[12],
            &[cached[91]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(6),
            &[4, 5, 6, 8, 13],
            &[cached[111], cached[112], cached[113], cached[114], cached[115]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(6),
            &[4, 5, 6, 8, 14],
            &[cached[117], cached[118], cached[119], cached[120], cached[121]],
            &[],
            &[],
            multiplicity,
        );
    }

}

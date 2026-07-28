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
        let produced: [f64; 202] = {
            let parameters = &self.params.values;
            let parameter_given = &*self.param_given;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let v0 = parameters[239];
                let v1 = 0e0f64;
                let v3 = parameters[274];
                let v6 = parameters[17];
                let v7 = 1e0f64;
                let v9 = parameters[207];
                let v12 = parameters[18];
                let v14 = parameters[228];
                let v18 = parameters[201];
                let v21 = parameters[165];
                let v23 = parameters[167];
                let v28 = parameters[162];
                let v30 = parameters[164];
                let v34 = if parameter_given[177] { 1.0 } else { 0.0 };
                let v35 = parameters[177];
                let v36 = parameters[227];
                let v37 = parameters[230];
                let v39 = 5e9f64;
                let v42 = 2.1e0f64;
                let v44 = 1.0f64;
                let v46 = 2.1e0f64;
                let v50 = 1.0000000000000005e-4f64;
                let v52 = 1.0f64;
                let v54 = parameters[34];
                let v55 = 1e-2f64;
                let v57 = parameters[59];
                let v58 = 1e-6f64;
                let v60 = parameters[101];
                let v62 = parameters[192];
                let v65 = parameters[231];
                let v67 = parameters[237];
                let v69 = parameters[238];
                let v71 = parameters[40];
                let v73 = parameters[236];
                let v75 = parameters[197];
                let v77 = parameters[306];
                let v79 = parameters[307];
                let v81 = parameters[189];
                let v82 = 1e4f64;
                let v84 = parameters[147];
                let v86 = parameters[196];
                let v87 = 1e1f64;
                let v89 = parameters[222];
                let v90 = 2.7315e2f64;
                let v92 = parameters[9];
                let v94 = parameters[1];
                let v95 = parameters[5];
                let v97 = parameters[0];
                let v98 = 1e6f64;
                let v102 = parameters[63];
                let v104 = parameters[62];
                let v108 = parameters[65];
                let v110 = parameters[64];
                let v113 = parameters[149];
                let v115 = parameters[148];
                let v119 = parameters[151];
                let v121 = parameters[150];
                let v126 = parameters[155];
                let v128 = parameters[154];
                let v131 = parameters[157];
                let v133 = parameters[156];
                let v136 = parameters[152];
                let v139 = 2e0f64;
                let v141 = parameters[153];
                let v143 = parameters[41];
                let v147 = parameters[42];
                let v155 = parameters[304];
                let v156 = parameters[12];
                let v158 = parameters[11];
                let v160 = parameters[305];
                let v161 = parameters[13];
                let v166 = 1e21f64;
                let v168 = 1e4f64;
                let v170 = 1.0f64;
                let v171 = 0.0f64;
                let v172 = 2.5e-1f64;
                let v175 = 1e-50f64;
                let v178 = 1e-1f64;
                let v181 = 2.1e0f64;
                let v183 = 1.0f64;
                let v185 = 0.0f64;
                let v187 = 3e0f64;
                let v188 = 0.0f64;
                let v190 = 4e0f64;
                let v197 = 4e25f64;
                let v198 = -4e25f64;
                let v204 = 5e-1f64;
                let v206 = 1e21f64;
                let v210 = 1e21f64;
                let v212 = 1e4f64;
                let v214 = 1.0f64;
                let v215 = 4e25f64;
                let v216 = -4e25f64;
                let v223 = 1e21f64;
                let v225 = parameters[88];
                let v227 = parameters[86];
                let v229 = parameters[91];
                let v231 = parameters[90];
                let v235 = parameters[89];
                let v237 = parameters[87];
                let v239 = parameters[93];
                let v241 = parameters[92];
                let v245 = parameters[291];
                let v247 = parameters[289];
                let v249 = parameters[294];
                let v251 = parameters[293];
                let v255 = parameters[292];
                let v257 = parameters[290];
                let v259 = parameters[296];
                let v261 = parameters[295];
                let v265 = parameters[110];
                let v267 = parameters[107];
                let v270 = parameters[106];
                let v272 = parameters[109];
                let v274 = parameters[108];
                let v278 = parameters[286];
                let v280 = parameters[285];
                let v283 = parameters[283];
                let v285 = parameters[288];
                let v287 = parameters[287];
                let v291 = parameters[233];
                let v293 = parameters[232];
                let v298 = 1e-3f64;
                let v312 = parameters[32];
                let v313 = parameters[235];
                let v315 = parameters[234];
                let v322 = parameters[61];
                let v324 = parameters[60];
                let v329 = parameters[43];
                let v332 = parameters[44];
                let v337 = parameters[6];
                let v339 = parameters[7];
                let v344 = parameters[8];
                let v375 = parameters[166];
                let v384 = parameters[191];
                let v386 = parameters[190];
                let v408 = parameters[169];
                let v411 = parameters[168];
                let v413 = parameters[170];
                let v432 = parameters[58];
                let v447 = 1.6021918e-19f64;
                let v449 = 1.034943e-10f64;
                let v454 = parameters[242];
                let v458 = parameters[244];
                let v461 = parameters[243];
                let v463 = parameters[248];
                let v465 = parameters[247];
                let v468 = parameters[246];
                let v483 = 1.04e16f64;
                let v486 = 5.1702525384001115e-2f64;
                let v490 = 5.1702525384001115e-2f64;
                let v494 = parameters[77];
                let v496 = parameters[75];
                let v498 = parameters[116];
                let v500 = parameters[115];
                let v504 = parameters[117];
                let v507 = parameters[179];
                let v509 = parameters[180];
                let v512 = parameters[25];
                let v514 = parameters[2];
                let v517 = parameters[3];
                let v519 = parameters[48];
                let v521 = parameters[4];
                let v527 = 1e3f64;
                let v529 = parameters[132];
                let v531 = parameters[131];
                let v534 = parameters[127];
                let v536 = parameters[126];
                let v539 = parameters[125];
                let v541 = parameters[124];
                let v544 = parameters[121];
                let v546 = parameters[120];
                let v549 = parameters[118];
                let v551 = parameters[122];
                let v554 = parameters[119];
                let v557 = parameters[46];
                let v559 = parameters[47];
                let v562 = parameters[135];
                let v564 = parameters[134];
                let v567 = parameters[133];
                let v569 = parameters[130];
                let v571 = parameters[129];
                let v574 = parameters[128];
                let v576 = 1.2919089961638799e9f64;
                let v580 = parameters[28];
                let v583 = 1e3f64;
                let v585 = parameters[24];
                let v586 = parameters[31];
                let v587 = 5e0f64;
                let v589 = 6e0f64;
                let v591 = 1e-7f64;
                let v593 = 9.025e-5f64;
                let v596 = parameters[37];
                let v599 = 1.3806226e-23f64;
                let v602 = parameters[202];
                let v603 = 1e0f64;
                let v605 = parameters[96];
                let v607 = parameters[95];
                let v610 = parameters[249];
                let v612 = parameters[98];
                let v614 = parameters[97];
                let v618 = parameters[100];
                let v620 = parameters[99];
                let v624 = parameters[278];
                let v626 = parameters[277];
                let v629 = parameters[276];
                let v631 = parameters[282];
                let v633 = parameters[281];
                let v637 = parameters[280];
                let v639 = parameters[279];
                let v643 = parameters[163];
                let v660 = parameters[113];
                let v662 = parameters[112];
                let v665 = parameters[111];
                let v667 = parameters[182];
                let v669 = parameters[181];
                let v672 = parameters[186];
                let v674 = parameters[185];
                let v678 = parameters[188];
                let v680 = parameters[187];
                let v684 = parameters[184];
                let v686 = parameters[183];
                let v691 = 4e-6f64;
                let v696 = 1e-13f64;
                let v700 = parameters[103];
                let v702 = parameters[102];
                let v709 = 3.2043836e-19f64;
                let v715 = parameters[251];
                let v716 = parameters[252];
                let v718 = parameters[38];
                let v722 = 2.2204460492503132e-17f64;
                let v739 = parameters[49];
                let v741 = parameters[51];
                let v743 = parameters[50];
                let v747 = parameters[53];
                let v749 = parameters[52];
                let v753 = parameters[54];
                let v758 = 1e-12f64;
                let v802 = 1.414213562373095e0f64;
                let v805 = 3.453133e-11f64;
                let v806 = parameters[226];
                let v809 = parameters[229];
                let v812 = -1.6021918e-19f64;
                let v818 = 1e-9f64;
                let v825 = parameters[255];
                let v827 = parameters[254];
                let v831 = 1.0f64;
                let v832 = 2e-3f64;
                let v833 = -2e-3f64;
                let v837 = 9.5e-1f64;
                let v839 = 3.8e0f64;
                let v843 = 3.2043836e-19f64;
                let v852 = parameters[55];
                let v854 = parameters[68];
                let v856 = parameters[297];
                let v858 = parameters[57];
                let v862 = 5e-3f64;
                let v867 = parameters[71];
                let v869 = parameters[72];
                let v871 = parameters[74];
                let v873 = parameters[56];
                let v878 = parameters[104];
                let v885 = 1.0f64;
                let v886 = 2e-1f64;
                let v887 = -2e-1f64;
                let v889 = 9.9e-1f64;
                let v891 = 0.0f64;
                let v904 = 1e2f64;
                let v906 = parameters[83];
                let v908 = parameters[82];
                let v911 = parameters[81];
                let v913 = 1.034943e-12f64;
                let v915 = parameters[80];
                let v917 = parameters[79];
                let v920 = parameters[78];
                let v923 = parameters[216];
                let v925 = parameters[85];
                let v927 = parameters[301];
                let v929 = parameters[300];
                let v932 = parameters[299];
                let v934 = 1.17e1f64;
                let v936 = parameters[94];
                let v940 = parameters[302];
                let v942 = parameters[275];
                let v945 = 9.999999999999978e-1f64;
                let v946 = parameters[114];
                let v948 = 1.0000000000000022e0f64;
                let v951 = 1.9999999999999978e0f64;
                let v953 = 2.000000000000002e0f64;
                let v956 = 9.999999999999978e-1f64;
                let v958 = 1.0000000000000022e0f64;
                let v963 = 1.9999999999999978e0f64;
                let v965 = 2.000000000000002e0f64;
                let v968 = 9.999999999999978e-1f64;
                let v970 = 1.0000000000000022e0f64;
                let v973 = -1e0f64;
                let v977 = 1.9999999999999978e0f64;
                let v979 = 2.000000000000002e0f64;
                let v982 = 9.999999999999978e-1f64;
                let v984 = 1.0000000000000022e0f64;
                let v989 = 1.9999999999999978e0f64;
                let v991 = 2.000000000000002e0f64;
                let v994 = -1e0f64;
                let v999 = parameters[240];
                let v1002 = parameters[312];
                let v1004 = parameters[315];
                let v1006 = parameters[314];
                let v1008 = parameters[313];
                let v1010 = parameters[308];
                let v1013 = parameters[322];
                let v1018 = parameters[317];
                let v1020 = parameters[319];
                let v1022 = parameters[320];
                let v1024 = parameters[331];
                let v1026 = parameters[330];
                let v1029 = parameters[329];
                let v1031 = parameters[328];
                let v1034 = parameters[327];
                let v1036 = parameters[326];
                let v1039 = parameters[311];
                let v1043 = parameters[309];
                let v1050 = parameters[316];
                let v1052 = parameters[318];
                let v1064 = parameters[310];
                let v1070 = 8e0f64;
                let v1072 = 0e0f64;
                let v1074 = 0e0f64;
                let v1076 = 0e0f64;
                let v1078 = 0e0f64;
                let v1080 = 0e0f64;
                let v1082 = 0e0f64;
                let v1084 = parameters[27];
                let v1085 = parameters[15];
                let v1087 = parameters[16];
                let v1089 = 0e0f64;
                let v1091 = 0e0f64;
                let v1092 = 0e0f64;
                let v2 = if v0 != v1 { 1.0 } else { 0.0 };
                let v5: f64;
                if v2 != 0.0 {
                    let v4 = if v3 <= v1 { 1.0 } else { 0.0 };
                    let v8: f64;
                    if v4 != 0.0 {
                        v8 = v7;
                    } else {
                        v8 = v1;
                    }
                    v5 = v8;
                } else {
                    v5 = v1;
                }
                let v11: f64;
                if v6 != 0.0 {
                    let v10 = if v9 <= v1 { 1.0 } else { 0.0 };
                    let v13: f64;
                    if v10 != 0.0 {
                        v13 = v7;
                    } else {
                        v13 = v5;
                    }
                    v11 = v13;
                } else {
                    v11 = v5;
                }
                let v16: f64;
                if v12 != 0.0 {
                    let v15 = if v14 <= v1 { 1.0 } else { 0.0 };
                    let v17: f64;
                    if v15 != 0.0 {
                        v17 = v7;
                    } else {
                        v17 = v11;
                    }
                    v16 = v17;
                } else {
                    v16 = v11;
                }
                let v20: f64;
                if v12 != 0.0 {
                    let v19 = if v18 <= v1 { 1.0 } else { 0.0 };
                    let v26: f64;
                    if v19 != 0.0 {
                        v26 = v7;
                    } else {
                        v26 = v16;
                    }
                    v20 = v26;
                } else {
                    v20 = v16;
                }
                let v25 = if (if v21 == v1 { 1.0 } else { 0.0 }) != 0.0 && (if v23 < v1 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v27: f64;
                if v25 != 0.0 {
                    v27 = v7;
                } else {
                    v27 = v20;
                }
                let v32 = if (if v28 == v1 { 1.0 } else { 0.0 }) != 0.0 && (if v30 < v1 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v33: f64;
                if v32 != 0.0 {
                    v33 = v7;
                } else {
                    v33 = v27;
                }
                let v41: f64;
                if v34 != 0.0 {
                    v41 = v35;
                } else {
                    let v40 = v39 / (v36 * v37);
                    v41 = v40;
                }
                let v45 = if (if v41 < v42 { 1.0 } else { 0.0 }) != 0.0 && v44 != 0.0 { 1.0 } else { 0.0 };
                let v53: f64;
                if v45 != 0.0 {
                    let v47 = v46 - v41;
                    let v48 = v47 * v47;
                    let v51 = (v48 * v48) + v50;
                    let v174: f64;
                    if v52 != 0.0 {
                        let v184: f64;
                        if v171 != 0.0 {
                            v184 = v7;
                        } else {
                            let v186: f64;
                            if v183 != 0.0 {
                                v186 = v139;
                            } else {
                                let v189: f64;
                                if v185 != 0.0 {
                                    v189 = v187;
                                } else {
                                    let v191: f64;
                                    if v188 != 0.0 {
                                        v191 = v190;
                                    } else {
                                        v191 = v1;
                                    }
                                    v189 = v191;
                                }
                                v186 = v189;
                            }
                            v184 = v186;
                        }
                        let mut v192: f64 = 0.0;
                        let mut v193: f64 = 0.0;
                        v192 = v1;
                        v193 = v51;
                        loop {
                            let v194 = if v192 < v184 { 1.0 } else { 0.0 };
                            if v194 == 0.0 {
                                break;
                            }
                            let v195 = v193.sqrt();
                            let v196 = v192 + v7;
                            v192 = v196;
                            v193 = v195;
                        }
                        v174 = v193;
                    } else {
                        let v173 = v51.powf(v172);
                        v174 = v173;
                    }
                    let v182 = v181 - ((v47 * v178) * (v7 / (v174 + v175)));
                    v53 = v182;
                } else {
                    v53 = v41;
                }
                let v56 = v54 * v55;
                let v59 = v57 / v58;
                let v61 = v60 * v55;
                let v63 = v62 / v58;
                let v66 = v65 / v58;
                let v72 = v71 / v58;
                let v74 = v73 / v58;
                let v76 = v75 / v55;
                let v80 = v79 / v58;
                let v83 = v81 * v82;
                let v88 = v86 / v87;
                let v91 = v89 + v90;
                let v93 = v92 + v90;
                let v96 = v94 / v95;
                let v99 = v97 * v98;
                let v100 = v96 * v98;
                let v101 = v100 * v99;
                let v105 = v104 / (v101.powf(v102));
                let v106 = v97 + v105;
                let v111 = v110 / (v101.powf(v108));
                let v112 = v106 * v98;
                let v118 = (v96 + v105) * v98;
                let v125 = ((v84 / v58) * (v7 + (v115 / (v112.powf(v113))))) * (v7 + (v121 / (v118.powf(v119))));
                let v140 = v139 * ((v136 * (v7 + (v128 / (v112.powf(v126))))) * (v7 + (v133 / (v118.powf(v131)))));
                let v142 = v140 * v141;
                let v146 = (v96 - (v139 * v143)) - v142;
                let v150 = (v96 - (v139 * v147)) - v142;
                let v151 = v146 * v95;
                let v152 = v150 * v95;
                let v153 = (v67 * v55) / v151;
                let v154 = (v69 / v55) * v152;
                let v163 = (v158 + (v155 * v156)) + (v160 * v161);
                let v169 = (((v37 / v58) + ((v77 / v58) * v163)) - v166) - v168;
                let v199: f64;
                if v170 != 0.0 {
                    v199 = v197;
                } else {
                    v199 = v198;
                }
                let v207 = v206 + (v204 * (v169 + (((v169 * v169) + v199).sqrt())));
                let v213 = ((v59 + (v80 * v163)) - v210) - v212;
                let v217: f64;
                if v214 != 0.0 {
                    v217 = v215;
                } else {
                    v217 = v216;
                }
                let v224 = v223 + (v204 * (v213 + (((v213 * v213) + v217).sqrt())));
                let v234 = (v227 * (v99.powf(v225))) * (v7 + (v231 / (v99.powf(v229))));
                let v244 = (v237 * (v99.powf(v235))) * (v7 + (v241 / (v99.powf(v239))));
                let v254 = (v247 * (v99.powf(v245))) * (v7 + (v251 / (v99.powf(v249))));
                let v264 = (v257 * (v99.powf(v255))) * (v7 + (v261 / (v99.powf(v259))));
                let v277 = (v270 * (v7 + (v267 / (v99.powf(v265))))) * (v7 + (v274 / (v100.powf(v272))));
                let v290 = (v283 * (v7 + (v280 / (v99.powf(v278))))) * (v7 + (v287 / (v100.powf(v285))));
                let v299 = v66 * v298;
                let v300 = ((v66 * (v7 + (v293 / (v99.powf(v291))))) - v74) - v299;
                let v302 = (v190 * v74) * v299;
                let v303 = if v302 > v1 { 1.0 } else { 0.0 };
                let v305: f64;
                if v303 != 0.0 {
                    v305 = v302;
                } else {
                    let v304 = -v302;
                    v305 = v304;
                }
                let v311 = v74 + (v204 * (v300 + (((v300 * v300) + v305).sqrt())));
                let v321: f64;
                if v312 != 0.0 {
                    let v320 = ((v311 * (v7 + (v315 / (v100.powf(v313))))) - v74) - v299;
                    let v350: f64;
                    if v303 != 0.0 {
                        v350 = v302;
                    } else {
                        let v349 = -v302;
                        v350 = v349;
                    }
                    let v356 = v74 + (v204 * (v320 + (((v320 * v320) + v350).sqrt())));
                    v321 = v356;
                } else {
                    v321 = v311;
                }
                let v327 = v224 * (v7 + (v324 / (v100.powf(v322))));
                let v328 = v204 * v97;
                let v336 = v139 / ((v7 / (v329 + v328)) + (v7 / (v332 + v328)));
                let v348 = if (if (if v337 > v1 { 1.0 } else { 0.0 }) != 0.0 && (if v339 > v1 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if v95 == v7 { 1.0 } else { 0.0 }) != 0.0 || (if (if v95 > v7 { 1.0 } else { 0.0 }) != 0.0 && (if v344 > v1 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v357: f64;
                if v348 != 0.0 {
                    let mut v359: f64 = 0.0;
                    let mut v360: f64 = 0.0;
                    v359 = v1;
                    v360 = v1;
                    loop {
                        let v361 = if v359 < v95 { 1.0 } else { 0.0 };
                        if v361 == 0.0 {
                            break;
                        }
                        let v364 = v359 * (v344 + v97);
                        let v371 = (v360 + (v7 / ((v337 + v328) + v364))) + (v7 / ((v339 + v328) + v364));
                        let v372 = v359 + v7;
                        v359 = v372;
                        v360 = v371;
                    }
                    let v374 = (v139 * v95) / v360;
                    v357 = v374;
                } else {
                    v357 = v1;
                }
                let v358 = if v357 > v1 { 1.0 } else { 0.0 };
                let v382: f64;
                let v383: f64;
                if v358 != 0.0 {
                    let v377 = v7 / (v7 + v375);
                    let v378 = v21 / v357;
                    let v380 = if v23 == v1 { 1.0 } else { 0.0 };
                    let v381 = if (if v378 == v1 { 1.0 } else { 0.0 }) != 0.0 && v380 != 0.0 { 1.0 } else { 0.0 };
                    let v396: f64;
                    if v381 != 0.0 {
                        v396 = v7;
                    } else {
                        let v395 = v378.powf(v23);
                        v396 = v395;
                    }
                    let v397 = v21 / v336;
                    let v399 = if (if v397 == v1 { 1.0 } else { 0.0 }) != 0.0 && v380 != 0.0 { 1.0 } else { 0.0 };
                    let v401: f64;
                    if v399 != 0.0 {
                        v401 = v7;
                    } else {
                        let v400 = v397.powf(v23);
                        v401 = v400;
                    }
                    let v407 = (v327 * (v7 + (v377 * v396))) / (v7 + (v377 * v401));
                    let v410 = v7 / (v7 + v408);
                    let v422 = (v207 * (v7 + (v410 * ((v411 / v357).powf(v413))))) / (v7 + (v410 * ((v411 / v336).powf(v413))));
                    v382 = v422;
                    v383 = v407;
                } else {
                    v382 = v207;
                    v383 = v327;
                }
                let v389 = v63 / v382;
                let v391 = (v389 - (v7 + (v386 / (v100.powf(v384))))) - v55;
                let v393 = (v190 * v389) * v55;
                let v394 = if v393 > v1 { 1.0 } else { 0.0 };
                let v424: f64;
                if v394 != 0.0 {
                    v424 = v393;
                } else {
                    let v423 = -v393;
                    v424 = v423;
                }
                let v431 = v382 * (v389 - (v204 * (v391 + (((v391 * v391) + v424).sqrt()))));
                let v435 = if (if v97 > v432 { 1.0 } else { 0.0 }) != 0.0 || (if v432 <= v1 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v446: f64;
                if v435 != 0.0 {
                    let v440 = ((v431 * (v97 - v432)) + (v383 * v432)) / v97;
                    v446 = v440;
                } else {
                    let v445 = v383 + (((v383 - v431) * (v432 - v97)) / v432);
                    v446 = v445;
                }
                let v448 = v447 * v446;
                let v450 = v448 * v449;
                let v451 = v139 * v450;
                let v453 = (v447 * v321) * v449;
                let v457 = v0 * (v99.powf((-v454)));
                let v462 = v461 * (v99.powf((-v458)));
                let v469 = v468 * ((v99 + v463).powf((-v465)));
                let v473 = if (if v97 <= (v139 * v432) { 1.0 } else { 0.0 }) != 0.0 && (if v432 > v1 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v482: f64;
                if v473 != 0.0 {
                    let v481 = ((((v139 * v383) - (((v383 - v431) * v97) / v432)) - v431) / v431).ln();
                    v482 = v481;
                } else {
                    v482 = v1;
                }
                let v487 = v486 * ((v446 / v483).ln());
                let v491 = v490 * ((v431 / v483).ln());
                let v497 = ((v7 + (v7 / v99)).powf(v494)) * v496;
                let v499 = v498 * v99;
                let v506 = (((v499 * v500) / (v499 + v500)) + v504) + v175;
                let v511 = v7 + ((v99.powf(v507)) * v509);
                let v513 = if v512 == v7 { 1.0 } else { 0.0 };
                let v528: f64;
                if v513 != 0.0 {
                    let v525 = (v519 * (v517 + (v146 / (v187 * v514)))) / ((v514 * (v97 - v521)) * v95);
                    let v526 = if v525 > v298 { 1.0 } else { 0.0 };
                    let v584: f64;
                    if v526 != 0.0 {
                        let v582 = v7 / v525;
                        v584 = v582;
                    } else {
                        v584 = v583;
                    }
                    v528 = v584;
                } else {
                    v528 = v527;
                }
                let v533 = v7 + (v531 / (v100.powf(v529)));
                let v540 = v539 * (v7 + (v536 / (v99.powf(v534))));
                let v543 = v99 / (v99 + v541);
                let v550 = v549 * (v7 + (v546 / (v99.powf(v544))));
                let v555 = v554 * (v7 + (v551 / v99));
                let v561 = ((v82 * v152) * v557) / (v99.powf(v559));
                let v568 = v567 * (v7 + (v564 / (v99.powf(v562))));
                let v575 = v574 * (v7 + (v571 / (v99.powf(v569))));
                let v578 = (v576 / v446).sqrt();
                let v581 = if v580 != 0.0 && (if v67 > v1 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v588 = if v586 >= v587 { 1.0 } else { 0.0 };
                let v590 = if v586 >= v589 { 1.0 } else { 0.0 };
                let v597 = v596 - (v91 * (v593 + (v91 * v591)));
                let v598 = v91 * v91;
                let v601 = v447 / (v599 * v91);
                let v604 = v602 - v603;
                let v623 = ((v610 * (v7 + (v607 / (v100.powf(v605))))) * (v7 + (v614 / (v99.powf(v612))))) * (v7 + (v620 / (v101.powf(v618))));
                let v642 = ((v629 * (v7 + (v626 / (v100.powf(v624))))) * (v7 + (v633 / (v99.powf(v631))))) * (v7 + (v639 / (v101.powf(v637))));
                let v658: f64;
                let v659: f64;
                if v358 != 0.0 {
                    let v645 = v7 / (v7 + v643);
                    let v651 = v7 + (v645 * ((v28 / v357).powf(v30)));
                    let v654 = v7 + (v645 * ((v28 / v336).powf(v30)));
                    let v655 = (v623 * v651) / v654;
                    let v657 = (v642 * v651) / v654;
                    v658 = v655;
                    v659 = v657;
                } else {
                    v658 = v623;
                    v659 = v642;
                }
                let v666 = v665 * (v7 + (v662 / (v99.powf(v660))));
                let v689 = (((v7 + (v669 / (v99.powf(v667)))) * (v7 + (v674 / (v99.powf(v672))))) * (v7 + (v680 / (v100.powf(v678))))) * (v7 + (v686 / (v101.powf(v684))));
                let v697 = (v204 * (v689 + (((v689 * v689) + v691).sqrt()))) + v696;
                let v698 = if v697 < v1 { 1.0 } else { 0.0 };
                let v699: f64;
                if v698 != 0.0 {
                    v699 = v1;
                } else {
                    v699 = v697;
                }
                let v705 = v699 * v56;
                let v706 = v61 * (v7 + (v702 / (v99.powf(v700))));
                let v708 = (v597 / v139) * v601;
                let v712 = ((v709 * v125) * v449).sqrt();
                let v714 = v7 / (v125 * v125);
                let v720 = (v718 / (v715 + v716)) * v97;
                let v724 = ((v718 * v298) + v722).abs();
                let v725 = if v718 > v1 { 1.0 } else { 0.0 };
                let v736: f64;
                if v725 != 0.0 {
                    let v727 = (v718 - v720) - v724;
                    let v729 = (v190 * v718) * v724;
                    let v730 = if v729 > v1 { 1.0 } else { 0.0 };
                    let v764: f64;
                    if v730 != 0.0 {
                        v764 = v729;
                    } else {
                        let v763 = -v729;
                        v764 = v763;
                    }
                    let v770 = v718 - (v204 * (v727 + (((v727 * v727) + v764).sqrt())));
                    v736 = v770;
                } else {
                    let v732 = (v720 - v718) - v724;
                    let v734 = (v190 * v718) * v724;
                    let v735 = if v734 > v1 { 1.0 } else { 0.0 };
                    let v772: f64;
                    if v735 != 0.0 {
                        v772 = v734;
                    } else {
                        let v771 = -v734;
                        v772 = v771;
                    }
                    let v778 = v718 + (v204 * (v732 + (((v732 * v732) + v772).sqrt())));
                    v736 = v778;
                }
                let v738 = v97 - (v139 * v736);
                let v740 = -v739;
                let v752 = v740 * (v7 + (v749 / (v99.powf(v747))));
                let v756 = -(v739 + (v753 * v99));
                let v759 = ((v740 * (v7 + (v743 / (v99.powf(v741))))) - v752) - v758;
                let v761 = (v190 * v752) * v758;
                let v762 = if v761 > v1 { 1.0 } else { 0.0 };
                let v780: f64;
                if v762 != 0.0 {
                    v780 = v761;
                } else {
                    let v779 = -v761;
                    v780 = v779;
                }
                let v788 = ((v752 + (v204 * (v759 + (((v759 * v759) + v780).sqrt())))) - v756) - v758;
                let v790 = (v190 * v756) * v758;
                let v791 = if v790 > v1 { 1.0 } else { 0.0 };
                let v793: f64;
                if v791 != 0.0 {
                    v793 = v790;
                } else {
                    let v792 = -v790;
                    v793 = v792;
                }
                let v800 = -(v756 + (v204 * (v788 + (((v788 * v788) + v793).sqrt()))));
                let v801 = v449 / v448;
                let v803 = v448 * v802;
                let v804 = v139 * v453;
                let v807 = v805 / v806;
                let v808 = v806 / v805;
                let v810 = v805 / v809;
                let v811 = v809 / v805;
                let v814 = (v812 * v431) * v36;
                let v815 = v449 / v36;
                let v816 = v7 / v815;
                let v817 = v811 + v816;
                let v819 = if v146 < v818 { 1.0 } else { 0.0 };
                let v820: f64;
                if v819 != 0.0 {
                    v820 = v7;
                } else {
                    v820 = v1;
                }
                let v821 = if v150 < v818 { 1.0 } else { 0.0 };
                let v822: f64;
                if v821 != 0.0 {
                    v822 = v7;
                } else {
                    v822 = v820;
                }
                let v823 = if v738 < v818 { 1.0 } else { 0.0 };
                let v824: f64;
                if v823 != 0.0 {
                    v824 = v7;
                } else {
                    v824 = v822;
                }
                let v826 = v825 * v204;
                let v828 = if v827 > v826 { 1.0 } else { 0.0 };
                let v829: f64;
                if v828 != 0.0 {
                    v829 = v826;
                } else {
                    v829 = v827;
                }
                let v830 = v487 + v800;
                let v834: f64;
                if v831 != 0.0 {
                    v834 = v832;
                } else {
                    v834 = v833;
                }
                let v836 = (v451 * v487).sqrt();
                let v838 = v837 * v487;
                let v841 = (v839 * v487) * v298;
                let v842 = if v432 != v1 { 1.0 } else { 0.0 };
                if v842 != 0.0 {
                    let v847 = (((v843 * v431) * v449) * v491).sqrt();
                    let v848 = v491 + v800;
                    let v851 = (v139 * v36) / (v432 * v432);
                    let v853 = v852 - v487;
                    let v855 = v854 / v432;
                } else {
                }
                let v857 = if v856 != v1 { 1.0 } else { 0.0 };
                if v857 != 0.0 {
                    let v863 = (v190 * v487) * v862;
                    let v864 = if v863 > v1 { 1.0 } else { 0.0 };
                    let v866: f64;
                    if v864 != 0.0 {
                        v866 = v863;
                    } else {
                        let v865 = -v863;
                        v866 = v865;
                    }
                } else {
                }
                let v859 = v97 - v858;
                let v860 = v859 * v859;
                let v868 = v867 / v97;
                let v870 = if v869 > v1 { 1.0 } else { 0.0 };
                if v870 != 0.0 {
                    let v872 = v139 * v871;
                    let v876 = (v869 * v36) / (v328 + v873);
                } else {
                }
                let v877 = v83 / v146;
                let v879 = v878 / v100;
                let v880 = if v496 == v1 { 1.0 } else { 0.0 };
                let v881: f64;
                if v880 != 0.0 {
                    v881 = v1;
                } else {
                    v881 = v7;
                }
                let v882 = if v881 == v1 { 1.0 } else { 0.0 };
                if v882 != 0.0 {
                } else {
                    let v888: f64;
                    if v885 != 0.0 {
                        v888 = v886;
                    } else {
                        v888 = v887;
                    }
                }
                let v884 = (v431 / v321).ln();
                let v890 = v889 * v36;
                if v891 != 0.0 {
                    let v894 = ((-v814) * v816) / v139;
                } else {
                    let v895 = v36 / v449;
                    let v896 = v7 / v810;
                }
                let v897 = v506 - v7;
                let v898 = v897 - v603;
                let v900 = (v7 / v506) - v7;
                let v901 = v900 - v603;
                let v902 = -v152;
                let v903 = v902 * v738;
                let v905 = v809 * v904;
                let v914 = (v911 * (v7 + (v908 / (v99.powf(v906))))) / v913;
                let v922 = (v920 * (v7 + (v917 / (v99.powf(v915))))) / v913;
                let v924 = v923.sqrt();
                let v926 = v925 - v603;
                let v931 = v7 + (v929 / (v99.powf(v927)));
                let v933 = v932 * v931;
                if v312 != 0.0 {
                    let v935 = v934 * v905;
                } else {
                }
                let v937 = v936 - v603;
                let v938 = v277 - v603;
                if v312 != 0.0 {
                    let v939 = v934 * v905;
                } else {
                    let v941 = v940 * v931;
                }
                let v943 = v942 - v603;
                let v944 = v290 - v603;
                let v950 = if (if v945 <= v946 { 1.0 } else { 0.0 }) != 0.0 && (if v946 <= v948 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v950 != 0.0 {
                } else {
                    let v955 = if (if v951 <= v946 { 1.0 } else { 0.0 }) != 0.0 && (if v946 <= v953 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v955 != 0.0 {
                    } else {
                        let v961 = v946 - v7;
                        let v962 = v961 - v603;
                    }
                }
                let v960 = if (if v956 <= v946 { 1.0 } else { 0.0 }) != 0.0 && (if v946 <= v958 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v960 != 0.0 {
                } else {
                    let v967 = if (if v963 <= v946 { 1.0 } else { 0.0 }) != 0.0 && (if v946 <= v965 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v967 != 0.0 {
                    } else {
                        let v975 = (v973 / v946) - v7;
                        let v976 = v975 - v603;
                    }
                }
                let v972 = if (if v968 <= v946 { 1.0 } else { 0.0 }) != 0.0 && (if v946 <= v970 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v972 != 0.0 {
                } else {
                    let v981 = if (if v977 <= v946 { 1.0 } else { 0.0 }) != 0.0 && (if v946 <= v979 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v981 != 0.0 {
                    } else {
                        let v987 = v946 - v7;
                        let v988 = v987 - v603;
                    }
                }
                let v986 = if (if v982 <= v946 { 1.0 } else { 0.0 }) != 0.0 && (if v946 <= v984 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v986 != 0.0 {
                } else {
                    let v993 = if (if v989 <= v946 { 1.0 } else { 0.0 }) != 0.0 && (if v946 <= v991 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v993 != 0.0 {
                    } else {
                        let v996 = (v994 / v946) - v7;
                        let v997 = v996 - v603;
                    }
                }
                if v2 != 0.0 {
                    let v1000 = v999 - v603;
                } else {
                }
                let v998 = if v468 != v1 { 1.0 } else { 0.0 };
                if v2 != 0.0 {
                    let v1001 = v999 - v603;
                } else {
                }
                let v1003 = if v1002 == v7 { 1.0 } else { 0.0 };
                if v1003 != 0.0 {
                    let v1005 = v1004 / v58;
                    let v1007 = if v1006 > v1 { 1.0 } else { 0.0 };
                    let v1012: f64;
                    if v1007 != 0.0 {
                        let v1011 = v1006 * v1010;
                        v1012 = v1011;
                    } else {
                        v1012 = v1;
                    }
                    let v1017 = ((v1013 * v1013) + (v718 * v718)).sqrt();
                    let v1019 = v1018 / v82;
                    let v1021 = v1020 / v904;
                    let v1023 = v1022 - v603;
                    let v1028 = v7 + (v1026 / (v99.powf(v1024)));
                    let v1033 = v7 + (v1031 / (v99.powf(v1029)));
                    let v1038 = v7 + (v1036 / (v100.powf(v1034)));
                    let v1041 = (v447 / v1039) * v1017;
                } else {
                }
                let v1009 = if v1008 == v7 { 1.0 } else { 0.0 };
                if v1009 != 0.0 {
                    let v1042 = if v1006 > v1 { 1.0 } else { 0.0 };
                    let v1045: f64;
                    if v1042 != 0.0 {
                        let v1044 = v1006 * v1043;
                        v1045 = v1044;
                    } else {
                        v1045 = v1;
                    }
                    let v1049 = ((v1013 * v1013) + (v718 * v718)).sqrt();
                    let v1051 = v1050 / v82;
                    let v1053 = v1052 / v904;
                    let v1054 = v1022 - v603;
                    let v1057 = v7 + (v1026 / (v99.powf(v1024)));
                    let v1060 = v7 + (v1031 / (v99.powf(v1029)));
                    let v1063 = v7 + (v1036 / (v100.powf(v1034)));
                    let v1066 = (v447 / v1064) * v1049;
                } else {
                }
                let v1068: f64;
                let v1069: f64;
                if v581 != 0.0 {
                    let v1067 = v7 / v153;
                    v1068 = v154;
                    v1069 = v1067;
                } else {
                    v1068 = v1;
                    v1069 = v1;
                }
                let v1071 = if v586 >= v1070 { 1.0 } else { 0.0 };
                let v1073: f64;
                if v1002 != 0.0 {
                    v1073 = v1;
                } else {
                    v1073 = v1072;
                }
                let v1075: f64;
                if v1008 != 0.0 {
                    v1075 = v1;
                } else {
                    v1075 = v1074;
                }
                let v1077: f64;
                if v1002 != 0.0 {
                    v1077 = v1076;
                } else {
                    v1077 = v1;
                }
                let v1079: f64;
                if v1008 != 0.0 {
                    v1079 = v1078;
                } else {
                    v1079 = v1;
                }
                let v1081: f64;
                if v512 != 0.0 {
                    v1081 = v1;
                } else {
                    v1081 = v1080;
                }
                let v1083: f64;
                if v581 != 0.0 {
                    v1083 = v1;
                } else {
                    v1083 = v1082;
                }
                let v1088 = if (if v1084 != 0.0 && v1085 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v1087 != 0.0 { 1.0 } else { 0.0 };
                let v1090: f64;
                if v1088 != 0.0 {
                    v1090 = v1;
                } else {
                    v1090 = v1089;
                }
                let v1093: f64;
                let v1094: f64;
                if v585 != 0.0 {
                    v1093 = v1;
                    v1094 = v1;
                } else {
                    v1093 = v1091;
                    v1094 = v1092;
                }
            [v2, v4, v10, v15, v19, v25, v32, v33, v45, v194, v72, v76, v88, v91, v93, v99, v106, v111, v140, v146, v151, v152, v234, v244, v254, v264, v277, v290, v303, v348, v361, v358, v381, v399, v394, v431, v435, v446, v448, v450, v321, v453, v457, v462, v469, v473, v487, v497, v511, v513, v526, v533, v540, v543, v550, v555, v561, v568, v575, v578, v581, v588, v590, v597, v598, v666, v658, v659, v482, v698, v705, v706, v708, v712, v714, v725, v730, v735, v738, v762, v791, v800, v801, v803, v804, v807, v808, v810, v811, v814, v815, v816, v817, v819, v821, v823, v824, v828, v829, v830, v834, v836, v838, v841, v842, v847, v848, v851, v853, v855, v857, v864, v866, v860, v868, v870, v872, v876, v877, v879, v880, v882, v888, v884, v890, v894, v895, v896, v897, v900, v53, v902, v903, v914, v922, v924, v933, v935, v939, v941, v950, v955, v961, v960, v967, v975, v972, v981, v987, v986, v993, v996, v998, v1003, v1005, v1007, v1012, v1019, v1021, v1028, v1033, v1038, v1041, v1009, v1042, v1045, v1051, v1053, v1057, v1060, v1063, v1066, v1071, v528, v1068, v1069, v1088, v1073, v1075, v1077, v1079, v1081, v1083, v1090, v1093, v1094, v604, v898, v901, v926, v937, v938, v943, v944, v962, v976, v988, v997, v1000, v1001, v1023, v1054]
        };
        self.canonical_staged[137] = produced[0];
        self.canonical_staged[138] = produced[1];
        self.canonical_staged[139] = produced[2];
        self.canonical_staged[140] = produced[3];
        self.canonical_staged[141] = produced[4];
        self.canonical_staged[142] = produced[5];
        self.canonical_staged[143] = produced[6];
        self.canonical_staged[144] = produced[7];
        self.canonical_staged[145] = produced[8];
        self.canonical_staged[146] = produced[9];
        self.canonical_staged[99] = produced[10];
        self.canonical_staged[97] = produced[11];
        self.canonical_staged[98] = produced[12];
        self.canonical_staged[2] = produced[13];
        self.canonical_staged[161] = produced[14];
        self.canonical_staged[96] = produced[15];
        self.canonical_staged[93] = produced[16];
        self.canonical_staged[41] = produced[17];
        self.canonical_staged[95] = produced[18];
        self.canonical_staged[56] = produced[19];
        self.canonical_staged[58] = produced[20];
        self.canonical_staged[72] = produced[21];
        self.canonical_staged[80] = produced[22];
        self.canonical_staged[79] = produced[23];
        self.canonical_staged[85] = produced[24];
        self.canonical_staged[84] = produced[25];
        self.canonical_staged[78] = produced[26];
        self.canonical_staged[83] = produced[27];
        self.canonical_staged[147] = produced[28];
        self.canonical_staged[148] = produced[29];
        self.canonical_staged[150] = produced[30];
        self.canonical_staged[149] = produced[31];
        self.canonical_staged[151] = produced[32];
        self.canonical_staged[153] = produced[33];
        self.canonical_staged[152] = produced[34];
        self.canonical_staged[13] = produced[35];
        self.canonical_staged[154] = produced[36];
        self.canonical_staged[100] = produced[37];
        self.canonical_staged[19] = produced[38];
        self.canonical_staged[60] = produced[39];
        self.canonical_staged[17] = produced[40];
        self.canonical_staged[47] = produced[41];
        self.canonical_staged[90] = produced[42];
        self.canonical_staged[91] = produced[43];
        self.canonical_staged[92] = produced[44];
        self.canonical_staged[155] = produced[45];
        self.canonical_staged[22] = produced[46];
        self.canonical_staged[42] = produced[47];
        self.canonical_staged[70] = produced[48];
        self.canonical_staged[156] = produced[49];
        self.canonical_staged[157] = produced[50];
        self.canonical_staged[64] = produced[51];
        self.canonical_staged[61] = produced[52];
        self.canonical_staged[63] = produced[53];
        self.canonical_staged[59] = produced[54];
        self.canonical_staged[65] = produced[55];
        self.canonical_staged[102] = produced[56];
        self.canonical_staged[55] = produced[57];
        self.canonical_staged[62] = produced[58];
        self.canonical_staged[94] = produced[59];
        self.canonical_staged[158] = produced[60];
        self.canonical_staged[159] = produced[61];
        self.canonical_staged[160] = produced[62];
        self.canonical_staged[3] = produced[63];
        self.canonical_staged[1] = produced[64];
        self.canonical_staged[4] = produced[65];
        self.canonical_staged[5] = produced[66];
        self.canonical_staged[6] = produced[67];
        self.canonical_staged[7] = produced[68];
        self.canonical_staged[162] = produced[69];
        self.canonical_staged[9] = produced[70];
        self.canonical_staged[8] = produced[71];
        self.canonical_staged[10] = produced[72];
        self.canonical_staged[11] = produced[73];
        self.canonical_staged[12] = produced[74];
        self.canonical_staged[163] = produced[75];
        self.canonical_staged[164] = produced[76];
        self.canonical_staged[165] = produced[77];
        self.canonical_staged[57] = produced[78];
        self.canonical_staged[166] = produced[79];
        self.canonical_staged[167] = produced[80];
        self.canonical_staged[21] = produced[81];
        self.canonical_staged[14] = produced[82];
        self.canonical_staged[15] = produced[83];
        self.canonical_staged[16] = produced[84];
        self.canonical_staged[23] = produced[85];
        self.canonical_staged[20] = produced[86];
        self.canonical_staged[49] = produced[87];
        self.canonical_staged[45] = produced[88];
        self.canonical_staged[48] = produced[89];
        self.canonical_staged[66] = produced[90];
        self.canonical_staged[51] = produced[91];
        self.canonical_staged[46] = produced[92];
        self.canonical_staged[168] = produced[93];
        self.canonical_staged[169] = produced[94];
        self.canonical_staged[170] = produced[95];
        self.canonical_staged[171] = produced[96];
        self.canonical_staged[172] = produced[97];
        self.canonical_staged[18] = produced[98];
        self.canonical_staged[24] = produced[99];
        self.canonical_staged[25] = produced[100];
        self.canonical_staged[26] = produced[101];
        self.canonical_staged[27] = produced[102];
        self.canonical_staged[28] = produced[103];
        self.canonical_staged[173] = produced[104];
        self.canonical_staged[29] = produced[105];
        self.canonical_staged[30] = produced[106];
        self.canonical_staged[31] = produced[107];
        self.canonical_staged[32] = produced[108];
        self.canonical_staged[33] = produced[109];
        self.canonical_staged[174] = produced[110];
        self.canonical_staged[175] = produced[111];
        self.canonical_staged[34] = produced[112];
        self.canonical_staged[35] = produced[113];
        self.canonical_staged[36] = produced[114];
        self.canonical_staged[176] = produced[115];
        self.canonical_staged[37] = produced[116];
        self.canonical_staged[38] = produced[117];
        self.canonical_staged[39] = produced[118];
        self.canonical_staged[40] = produced[119];
        self.canonical_staged[177] = produced[120];
        self.canonical_staged[178] = produced[121];
        self.canonical_staged[43] = produced[122];
        self.canonical_staged[44] = produced[123];
        self.canonical_staged[50] = produced[124];
        self.canonical_staged[52] = produced[125];
        self.canonical_staged[53] = produced[126];
        self.canonical_staged[54] = produced[127];
        self.canonical_staged[67] = produced[128];
        self.canonical_staged[68] = produced[129];
        self.canonical_staged[69] = produced[130];
        self.canonical_staged[101] = produced[131];
        self.canonical_staged[71] = produced[132];
        self.canonical_staged[76] = produced[133];
        self.canonical_staged[75] = produced[134];
        self.canonical_staged[73] = produced[135];
        self.canonical_staged[74] = produced[136];
        self.canonical_staged[77] = produced[137];
        self.canonical_staged[81] = produced[138];
        self.canonical_staged[82] = produced[139];
        self.canonical_staged[179] = produced[140];
        self.canonical_staged[180] = produced[141];
        self.canonical_staged[86] = produced[142];
        self.canonical_staged[181] = produced[143];
        self.canonical_staged[182] = produced[144];
        self.canonical_staged[87] = produced[145];
        self.canonical_staged[183] = produced[146];
        self.canonical_staged[184] = produced[147];
        self.canonical_staged[88] = produced[148];
        self.canonical_staged[185] = produced[149];
        self.canonical_staged[186] = produced[150];
        self.canonical_staged[89] = produced[151];
        self.canonical_staged[187] = produced[152];
        self.canonical_staged[188] = produced[153];
        self.canonical_staged[109] = produced[154];
        self.canonical_staged[189] = produced[155];
        self.canonical_staged[110] = produced[156];
        self.canonical_staged[103] = produced[157];
        self.canonical_staged[104] = produced[158];
        self.canonical_staged[105] = produced[159];
        self.canonical_staged[107] = produced[160];
        self.canonical_staged[106] = produced[161];
        self.canonical_staged[108] = produced[162];
        self.canonical_staged[190] = produced[163];
        self.canonical_staged[191] = produced[164];
        self.canonical_staged[117] = produced[165];
        self.canonical_staged[111] = produced[166];
        self.canonical_staged[112] = produced[167];
        self.canonical_staged[113] = produced[168];
        self.canonical_staged[115] = produced[169];
        self.canonical_staged[114] = produced[170];
        self.canonical_staged[116] = produced[171];
        self.canonical_staged[192] = produced[172];
        self.canonical_staged[118] = produced[173];
        self.canonical_staged[119] = produced[174];
        self.canonical_staged[120] = produced[175];
        self.canonical_staged[193] = produced[176];
        self.canonical_staged[194] = produced[177];
        self.canonical_staged[195] = produced[178];
        self.canonical_staged[196] = produced[179];
        self.canonical_staged[197] = produced[180];
        self.canonical_staged[198] = produced[181];
        self.canonical_staged[199] = produced[182];
        self.canonical_staged[200] = produced[183];
        self.canonical_staged[201] = produced[184];
        self.canonical_staged[202] = produced[185];
        self.canonical_staged[121] = produced[186];
        self.canonical_staged[122] = produced[187];
        self.canonical_staged[123] = produced[188];
        self.canonical_staged[124] = produced[189];
        self.canonical_staged[125] = produced[190];
        self.canonical_staged[126] = produced[191];
        self.canonical_staged[127] = produced[192];
        self.canonical_staged[128] = produced[193];
        self.canonical_staged[129] = produced[194];
        self.canonical_staged[130] = produced[195];
        self.canonical_staged[131] = produced[196];
        self.canonical_staged[132] = produced[197];
        self.canonical_staged[133] = produced[198];
        self.canonical_staged[134] = produced[199];
        self.canonical_staged[135] = produced[200];
        self.canonical_staged[136] = produced[201];
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
                let v0 = staged[145];
                let v1 = 1.0f64;
                let v2 = staged[146];
                let v3 = staged[148];
                let v4 = staged[150];
                let v5 = if parameter_given[9] { 1.0 } else { 0.0 };
                let v6 = staged[161];
                let v7 = temperature;
                let v9 = parameters[10];
                if v0 != 0.0 {
                    if v1 != 0.0 {
                        loop {
                            if v2 == 0.0 {
                                break;
                            }
                        }
                    } else {
                    }
                } else {
                }
                if v3 != 0.0 {
                    loop {
                        if v4 == 0.0 {
                            break;
                        }
                    }
                } else {
                }
                let v8: f64;
                if v5 != 0.0 {
                    v8 = v6;
                } else {
                    v8 = v7;
                }
                let v10 = v8 + v9;
            [v10]
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
                let v0 = staged[145];
                let v1 = 1.0f64;
                let v2 = staged[146];
                let v3 = staged[148];
                let v4 = staged[150];
                if v0 != 0.0 {
                    if v1 != 0.0 {
                        loop {
                            if v2 == 0.0 {
                                break;
                            }
                        }
                    } else {
                    }
                } else {
                }
                if v3 != 0.0 {
                    loop {
                        if v4 == 0.0 {
                            break;
                        }
                    }
                } else {
                }
            [0.0]
        };
    }

    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        self.canonical_instance_stage(ctx);
        self.canonical_temperature_stage(ctx);
        self.canonical_timestep_stage(ctx);
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let multiplicity = self.multiplicity;
        let staged = &*self.canonical_staged;
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 45332 => 0usize, 45338 => 1usize, 45344 => 2usize, 45375 => 3usize, 45379 => 4usize, 45429 => 5usize, 45445 => 6usize, 45453 => 7usize, 45459 => 8usize, _ => usize::MAX };
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
            let v0 = staged[137];
            let v1 = parameters[17];
            let v2 = parameters[18];
            let v3 = staged[145];
            let v4 = 1.0f64;
            let v5 = staged[146];
            let v6 = parameters[32];
            let v7 = staged[148];
            let v8 = staged[150];
            let v9 = node_potentials[5];
            let v10 = node_potentials[12];
            let v12 = Lanes([1e0f64; 1]);
            let v14 = Lanes([1e0f64; 1]);
            let v17 = parameters[33];
            let v20 = node_potentials[11];
            let v22 = Lanes([1e0f64; 1]);
            let v28 = node_potentials[6];
            let v30 = Lanes([1e0f64; 1]);
            let v36 = node_potentials[2];
            let v39 = Lanes([1e0f64; 1]);
            let v44 = node_potentials[0];
            let v46 = Lanes([1e0f64; 1]);
            let v58 = staged[158];
            let v59 = node_potentials[4];
            let v60 = 0e0f64;
            let v62 = Lanes([0e0f64; 1]);
            let v65 = parameters[24];
            let v66 = Lanes([1e0f64; 1]);
            let v69 = 1e-9f64;
            let v70 = node_potentials[8];
            let v72 = Lanes([1e0f64; 1]);
            let v74 = node_potentials[9];
            let v76 = Lanes([1e0f64; 1]);
            let v78 = Lanes([0e0f64; 1]);
            let v79 = Lanes([0e0f64; 1]);
            let v89 = 1e0f64;
            let v95 = -1e0f64;
            let v111 = -1e0f64;
            let v127 = staged[0];
            let v132 = staged[1];
            let v134 = staged[2];
            let v136 = parameters[35];
            let v139 = staged[3];
            let v142 = parameters[36];
            let v147 = 1.3806226e-23f64;
            let v150 = 1.6021918e-19f64;
            let v164 = parameters[202];
            let v166 = staged[121];
            let v170 = parameters[201];
            let v174 = parameters[253];
            let v181 = staged[4];
            let v184 = 1e0f64;
            let v193 = staged[5];
            let v196 = staged[6];
            let v199 = staged[7];
            let v202 = 4e-1f64;
            let v205 = 1e-2f64;
            let v208 = 1.8000000000000002e-2f64;
            let v210 = 1e-1f64;
            let v223 = staged[8];
            let v228 = staged[9];
            let v236 = 2e0f64;
            let v246 = 1.5e0f64;
            let v249 = 1.04e16f64;
            let v254 = 2e0f64;
            let v261 = staged[10];
            let v273 = staged[11];
            let v282 = staged[12];
            let v287 = staged[13];
            let v299 = staged[14];
            let v306 = staged[15];
            let v309 = staged[16];
            let v321 = staged[17];
            let v327 = staged[18];
            let v330 = parameters[255];
            let v351 = 1.0f64;
            let v352 = Lanes([0e0f64; 3]);
            let v366 = parameters[216];
            let v369 = 1.984126984126984e-4f64;
            let v372 = 1.388888888888889e-3f64;
            let v378 = 8.333333333333333e-3f64;
            let v384 = 4.1666666666666664e-2f64;
            let v390 = 1.6666666666666666e-1f64;
            let v396 = 5e-1f64;
            let v407 = 1e-12f64;
            let v409 = 0.0f64;
            let v410 = 1.25e-1f64;
            let v412 = -8.75e-1f64;
            let v418 = 1e-50f64;
            let v439 = 0.0f64;
            let v441 = 1.0f64;
            let v443 = 3e0f64;
            let v444 = 0.0f64;
            let v446 = 4e0f64;
            let v470 = staged[19];
            let v472 = 1.034943e-10f64;
            let v474 = staged[20];
            let v477 = staged[21];
            let v494 = 4e-6f64;
            let v502 = 5e-1f64;
            let v505 = 1e-13f64;
            let v508 = Lanes([0e0f64; 5]);
            let v527 = 5e-2f64;
            let v529 = 1.0f64;
            let v530 = 2.0000000000000004e-2f64;
            let v531 = -2.0000000000000004e-2f64;
            let v577 = parameters[193];
            let v579 = parameters[195];
            let v582 = parameters[194];
            let v586 = staged[22];
            let v589 = staged[23];
            let v591 = staged[24];
            let v603 = Lanes([0e0f64; 4]);
            let v612 = 4e-8f64;
            let v622 = 1.0000000000000002e-14f64;
            let v633 = 1e-3f64;
            let v650 = 1e-4f64;
            let v674 = 1e12f64;
            let v676 = parameters[226];
            let v681 = 3.453133e-11f64;
            let v705 = staged[25];
            let v717 = staged[26];
            let v725 = staged[27];
            let v732 = staged[28];
            let v750 = staged[173];
            let v751 = staged[29];
            let v754 = staged[30];
            let v758 = staged[31];
            let v761 = staged[32];
            let v764 = staged[33];
            let v767 = parameters[66];
            let v769 = parameters[67];
            let v789 = staged[174];
            let v795 = 2.5e-1f64;
            let v806 = 5e-3f64;
            let v813 = parameters[227];
            let v818 = parameters[55];
            let v826 = staged[35];
            let v832 = 4e-6f64;
            let v842 = 1e-13f64;
            let v845 = -1e0f64;
            let v896 = 4e-6f64;
            let v906 = 1e-13f64;
            let v911 = 2.220446049250313e-15f64;
            let v938 = staged[34];
            let v951 = parameters[297];
            let v957 = staged[36];
            let v960 = parameters[69];
            let v962 = parameters[70];
            let v967 = parameters[250];
            let v977 = staged[176];
            let v980 = staged[37];
            let v982 = parameters[73];
            let v989 = staged[38];
            let v992 = Lanes([0e0f64; 4]);
            let v995 = staged[39];
            let v1005 = staged[40];
            let v1015 = staged[41];
            let v1018 = staged[178];
            let v1019 = parameters[76];
            let v1021 = -3e0f64;
            let v1030 = staged[44];
            let v1044 = parameters[29];
            let v1052 = 4.000000000000001e-2f64;
            let v1062 = 1.0000000000000001e-11f64;
            let v1065 = 3.7037037037037035e-2f64;
            let v1068 = 3.333333333333333e-1f64;
            let v1080 = 1.48148111111111e-1f64;
            let v1083 = 4.02052934513951e-2f64;
            let v1089 = 3.333333333333333e-1f64;
            let v1105 = staged[42];
            let v1114 = staged[43];
            let v1144 = staged[45];
            let v1193 = staged[46];
            let v1219 = 2.220446049250313e-15f64;
            let v1221 = 2.220446049250313e-15f64;
            let v1247 = 8e-4f64;
            let v1265 = 5e2f64;
            let v1270 = 1e-8f64;
            let v1272 = 1.2919089961638799e9f64;
            let v1276 = staged[50];
            let v1287 = staged[47];
            let v1294 = -1e-8f64;
            let v1299 = 4e-12f64;
            let v1307 = 1e-16f64;
            let v1330 = staged[48];
            let v1359 = staged[49];
            let v1365 = -1e0f64;
            let v1380 = staged[51];
            let v1414 = parameters[298];
            let v1426 = 0.0f64;
            let v1427 = staged[52];
            let v1446 = parameters[15];
            let v1448 = 2e-1f64;
            let v1458 = 3.3163543761348e-29f64;
            let v1514 = -1e-1f64;
            let v1517 = -1e-1f64;
            let v1519 = 1.2919089961638799e9f64;
            let v1547 = 2.220446049250313e-15f64;
            let v1549 = 2.220446049250313e-15f64;
            let v1565 = 2.220446049250313e-15f64;
            let v1586 = 2.220446049250313e-15f64;
            let v1590 = 1.2919089961638799e9f64;
            let v1595 = 2.220446049250313e-15f64;
            let v1659 = 2.220446049250313e-15f64;
            let v1725 = -1e0f64;
            let v1783 = -1e-8f64;
            let v1792 = 4.0000000000000004e-20f64;
            let v1809 = 1.0000000000000001e-20f64;
            let v1874 = 1e-13f64;
            let v1943 = -1e0f64;
            let v2015 = -1e-8f64;
            let v2024 = 4.0000000000000004e-20f64;
            let v2041 = 1.0000000000000001e-20f64;
            let v2182 = -1e0f64;
            let v2203 = 2.220446049250313e-15f64;
            let v2229 = staged[53];
            let v2231 = staged[54];
            let v2326 = 1.5e-1f64;
            let v2329 = 1.0f64;
            let v2340 = 2.25e-2f64;
            let v2342 = 1.0f64;
            let v2345 = 1.0f64;
            let v2347 = -5e-1f64;
            let v2349 = 5e-1f64;
            let v2367 = 0.0f64;
            let v2369 = 0.0f64;
            let v2371 = 0.0f64;
            let v2383 = 1.2919089961638799e9f64;
            let v2394 = 2.220446049250313e-15f64;
            let v2423 = 2.220446049250313e-15f64;
            let v2428 = 2.220446049250313e-15f64;
            let v2475 = 2.220446049250313e-15f64;
            let v2541 = 1.0f64;
            let v2558 = 1.0f64;
            let v2620 = -1e-8f64;
            let v2637 = -1e0f64;
            let v2705 = 1e-10f64;
            let v2726 = 1.0f64;
            let v2768 = -1e-8f64;
            let v2793 = -1e0f64;
            let v2869 = 0.0f64;
            let v2870 = 1e-10f64;
            let v2890 = 0.0f64;
            let v2932 = -1e-8f64;
            let v2957 = -1e0f64;
            let v3039 = 2.25e-2f64;
            let v3041 = 1.0f64;
            let v3044 = 1.0f64;
            let v3046 = -5e-1f64;
            let v3048 = 5e-1f64;
            let v3066 = 0.0f64;
            let v3068 = 0.0f64;
            let v3070 = 0.0f64;
            let v3082 = staged[55];
            let v3089 = 3.2043836e-19f64;
            let v3189 = parameters[136];
            let v3227 = Lanes([0e0f64; 6]);
            let v3247 = 1.0f64;
            let v3263 = staged[56];
            let v3282 = 3.0000000000000002e-2f64;
            let v3295 = staged[57];
            let v3316 = 2.220446049250313e-15f64;
            let v3328 = 2.220446049250313e-15f64;
            let v3355 = 1.3e0f64;
            let v3361 = 3e-2f64;
            let v3385 = 1e2f64;
            let v3387 = staged[58];
            let v3390 = parameters[26];
            let v3392 = parameters[141];
            let v3400 = parameters[144];
            let v3411 = parameters[143];
            let v3419 = staged[59];
            let v3426 = 9.9e1f64;
            let v3438 = 4.12e0f64;
            let v3453 = 4e-6f64;
            let v3463 = 1e-13f64;
            let v3478 = parameters[142];
            let v3487 = -3.4e1f64;
            let v3504 = 7.38905609893065e0f64;
            let v3531 = staged[60];
            let v3538 = staged[61];
            let v3553 = 4e-6f64;
            let v3563 = 1e-13f64;
            let v3568 = parameters[16];
            let v3573 = staged[62];
            let v3593 = parameters[123];
            let v3599 = staged[63];
            let v3600 = staged[64];
            let v3609 = 4e-4f64;
            let v3619 = 1e-12f64;
            let v3625 = staged[65];
            let v3645 = parameters[140];
            let v3652 = 2.4665765749313358e0f64;
            let v3654 = 4.1046315303568966e26f64;
            let v3658 = 2.1633307652783932e-2f64;
            let v3670 = parameters[139];
            let v3682 = 3.3163543761348e-29f64;
            let v3734 = parameters[27];
            let v3739 = parameters[138];
            let v3741 = parameters[137];
            let v3751 = node_potentials[10];
            let v3753 = Lanes([1e0f64; 1]);
            let v3770 = -3.7477e0f64;
            let v3774 = -4.8303e0f64;
            let v3814 = -1e-8f64;
            let v3854 = 1e-9f64;
            let v3883 = 8e1f64;
            let v3935 = 1.4142135623730951e0f64;
            let v3942 = 1.4142135623730951e0f64;
            let v3952 = 5.540622384e34f64;
            let v3961 = -1e-8f64;
            let v3995 = 1e-8f64;
            let v4175 = 1.4142135623730951e0f64;
            let v4182 = 1.4142135623730951e0f64;
            let v4211 = -1e-8f64;
            let v4245 = 1e-8f64;
            let v4416 = 1.4142135623730951e0f64;
            let v4423 = 1.4142135623730951e0f64;
            let v4515 = 0e0f64;
            let v4535 = staged[66];
            let v4543 = 0e0f64;
            let v4732 = 0e0f64;
            let v4754 = 2.5e1f64;
            let v4755 = 4e1f64;
            let v4760 = 2e1f64;
            let v4763 = 1e1f64;
            let v4766 = 5e0f64;
            let v4849 = 1.0000000000000002e-2f64;
            let v4859 = 5.0000000000000005e-12f64;
            let v4880 = 4e-4f64;
            let v4890 = 1e-12f64;
            let v4900 = staged[67];
            let v4902 = staged[122];
            let v4911 = staged[68];
            let v4913 = staged[123];
            let v4927 = 0.0f64;
            let v4953 = 1.15e0f64;
            let v4960 = 1.15e0f64;
            let v4981 = 1.15e0f64;
            let v4999 = 5e-13f64;
            let v5005 = -1e0f64;
            let v5013 = 2.220446049250313e-15f64;
            let v5041 = 2.220446049250313e-15f64;
            let v5046 = 2.220446049250313e-15f64;
            let v5093 = 2.220446049250313e-15f64;
            let v5169 = 2e2f64;
            let v5229 = -1e-8f64;
            let v5238 = 4e-12f64;
            let v5255 = 1e-16f64;
            let v5389 = -1e0f64;
            let v5468 = -1e-8f64;
            let v5477 = 4e-12f64;
            let v5494 = 1e-16f64;
            let v5634 = -1e0f64;
            let v5700 = -1e-8f64;
            let v5740 = 1e-9f64;
            let v5820 = 1.4142135623730951e0f64;
            let v5827 = 1.4142135623730951e0f64;
            let v5845 = -1e-8f64;
            let v5879 = 1e-8f64;
            let v6062 = 1.4142135623730951e0f64;
            let v6069 = 1.4142135623730951e0f64;
            let v6098 = -1e-8f64;
            let v6132 = 1e-8f64;
            let v6306 = 1.4142135623730951e0f64;
            let v6313 = 1.4142135623730951e0f64;
            let v6690 = -1e0f64;
            let v6744 = -5e-1f64;
            let v6751 = 1e-18f64;
            let v6792 = -5e-1f64;
            let v6795 = -5e-1f64;
            let v6799 = staged[69];
            let v6800 = 2.220446049250313e-15f64;
            let v6802 = parameters[178];
            let v6803 = 2.220446049250313e-15f64;
            let v6816 = 2.220446049250313e-15f64;
            let v6832 = parameters[176];
            let v6843 = 2.220446049250313e-15f64;
            let v6850 = staged[70];
            let v6853 = 2.220446049250313e-15f64;
            let v6857 = 2.220446049250313e-15f64;
            let v6866 = 4e-6f64;
            let v6876 = 1e-13f64;
            let v6902 = 1e9f64;
            let v6948 = staged[71];
            let v6957 = staged[72];
            let v6967 = parameters[217];
            let v6970 = 1.984126984126984e-4f64;
            let v6973 = 1.388888888888889e-3f64;
            let v6979 = 8.333333333333333e-3f64;
            let v6985 = 4.1666666666666664e-2f64;
            let v6991 = 1.6666666666666666e-1f64;
            let v6997 = 5e-1f64;
            let v7008 = 2.220446049250313e-15f64;
            let v7010 = 2.220446049250313e-15f64;
            let v7015 = 1e4f64;
            let v7033 = 4e-12f64;
            let v7043 = 1e-16f64;
            let v7056 = staged[73];
            let v7058 = parameters[85];
            let v7060 = staged[124];
            let v7064 = parameters[84];
            let v7068 = staged[74];
            let v7073 = staged[75];
            let v7076 = staged[76];
            let v7098 = 3.9e0f64;
            let v7101 = staged[77];
            let v7115 = 3.6e7f64;
            let v7125 = 3e-7f64;
            let v7130 = parameters[94];
            let v7132 = staged[125];
            let v7136 = staged[78];
            let v7138 = staged[126];
            let v7144 = staged[79];
            let v7147 = 1e11f64;
            let v7150 = staged[80];
            let v7163 = parameters[105];
            let v7178 = staged[81];
            let v7184 = 4e-12f64;
            let v7194 = 1e-16f64;
            let v7202 = 3.6e3f64;
            let v7212 = 3e-9f64;
            let v7233 = staged[82];
            let v7240 = -5e-1f64;
            let v7255 = parameters[275];
            let v7257 = staged[127];
            let v7261 = staged[83];
            let v7263 = staged[128];
            let v7269 = staged[84];
            let v7274 = staged[85];
            let v7287 = parameters[284];
            let v7340 = staged[179];
            let v7341 = staged[180];
            let v7349 = staged[181];
            let v7350 = staged[86];
            let v7352 = staged[129];
            let v7362 = staged[182];
            let v7408 = staged[183];
            let v7417 = staged[87];
            let v7419 = staged[130];
            let v7429 = staged[184];
            let v7437 = staged[185];
            let v7438 = staged[88];
            let v7440 = staged[131];
            let v7450 = staged[186];
            let v7490 = staged[89];
            let v7492 = staged[132];
            let v7508 = 1.984126984126984e-4f64;
            let v7511 = 1.388888888888889e-3f64;
            let v7517 = 8.333333333333333e-3f64;
            let v7523 = 4.1666666666666664e-2f64;
            let v7529 = 1.6666666666666666e-1f64;
            let v7535 = 5e-1f64;
            let v7548 = 1.1e0f64;
            let v7554 = 1.0000000000000002e-2f64;
            let v7564 = 5.0000000000000005e-12f64;
            let v7569 = staged[187];
            let v7572 = staged[90];
            let v7575 = parameters[240];
            let v7577 = staged[133];
            let v7586 = parameters[241];
            let v7590 = staged[91];
            let v7608 = staged[92];
            let v7634 = parameters[245];
            let v7666 = 1.984126984126984e-4f64;
            let v7669 = 1.388888888888889e-3f64;
            let v7675 = 8.333333333333333e-3f64;
            let v7681 = 4.1666666666666664e-2f64;
            let v7687 = 1.6666666666666666e-1f64;
            let v7693 = 5e-1f64;
            let v7711 = 1.0000000000000002e-2f64;
            let v7721 = 5.0000000000000005e-12f64;
            let v7734 = staged[134];
            let v7788 = parameters[22];
            let v7808 = 1.0f64;
            let v7816 = 0.0f64;
            let v7817 = 2.5e-1f64;
            let v7819 = -7.5e-1f64;
            let v7840 = 1.0f64;
            let v7842 = 0.0f64;
            let v7844 = 0.0f64;
            let v7856 = staged[93];
            let v7857 = parameters[57];
            let v7868 = staged[94];
            let v7878 = parameters[159];
            let v7881 = parameters[158];
            let v7888 = parameters[161];
            let v7891 = parameters[160];
            let v7947 = parameters[20];
            let v7949 = parameters[23];
            let v7952 = -1e0f64;
            let v7999 = 4e-4f64;
            let v8009 = 1e-12f64;
            let v8076 = 4e-12f64;
            let v8086 = 1e-16f64;
            let v8126 = 4e-4f64;
            let v8136 = 1e-12f64;
            let v8141 = 2.220446049250313e-15f64;
            let v8150 = 4e-4f64;
            let v8160 = 1e-12f64;
            let v8165 = 2.220446049250313e-15f64;
            let v8183 = 4.000000000000001e-2f64;
            let v8193 = 1.0000000000000001e-11f64;
            let v8198 = 2.220446049250313e-15f64;
            let v8218 = 1e0f64;
            let v8220 = 1.0f64;
            let v8221 = 0.0f64;
            let v8222 = 1.25e-1f64;
            let v8224 = -8.75e-1f64;
            let v8239 = staged[95];
            let v8240 = parameters[5];
            let v8265 = 0.0f64;
            let v8267 = 1.0f64;
            let v8269 = 0.0f64;
            let v8300 = 4e-6f64;
            let v8310 = 1e-13f64;
            let v8326 = 4e-6f64;
            let v8336 = 1e-13f64;
            let v8343 = 2.220446049250313e-15f64;
            let v8345 = 2.220446049250313e-15f64;
            let v8368 = 4e-6f64;
            let v8378 = 1e-13f64;
            let v8386 = parameters[145];
            let v8420 = 4e-4f64;
            let v8430 = 1e-12f64;
            let v8455 = parameters[146];
            let v8475 = 4.000000000000001e-2f64;
            let v8485 = 1.0000000000000001e-11f64;
            let v8519 = 4.000000000000001e-2f64;
            let v8529 = 1.0000000000000001e-11f64;
            let v8599 = Lanes([0e0f64; 3]);
            let v8613 = 2.220446049250313e-15f64;
            let v8615 = parameters[256];
            let v8618 = parameters[258];
            let v8624 = parameters[206];
            let v8635 = parameters[205];
            let v8641 = parameters[207];
            let v8654 = 4e-4f64;
            let v8664 = 1e-12f64;
            let v8669 = parameters[211];
            let v8673 = parameters[212];
            let v8679 = parameters[260];
            let v8689 = parameters[210];
            let v8690 = 1e6f64;
            let v8693 = staged[96];
            let v8694 = parameters[259];
            let v8709 = 4e-6f64;
            let v8719 = 1e-13f64;
            let v8743 = parameters[209];
            let v8746 = parameters[208];
            let v8760 = parameters[204];
            let v8769 = -3.4e1f64;
            let v8771 = parameters[203];
            let v8790 = parameters[257];
            let v8824 = -1e0f64;
            let v8853 = -1e0f64;
            let v8860 = parameters[261];
            let v8868 = parameters[215];
            let v8875 = 4e-4f64;
            let v8885 = 1e-12f64;
            let v8891 = parameters[214];
            let v8893 = parameters[263];
            let v8903 = -3.4e1f64;
            let v8907 = parameters[264];
            let v8909 = parameters[265];
            let v8926 = parameters[213];
            let v8930 = parameters[262];
            let v8942 = parameters[269];
            let v8949 = parameters[268];
            let v8956 = 4e-4f64;
            let v8966 = 1e-12f64;
            let v8972 = parameters[267];
            let v8974 = parameters[271];
            let v8984 = -3.4e1f64;
            let v8988 = parameters[272];
            let v8990 = parameters[273];
            let v9012 = parameters[266];
            let v9016 = parameters[270];
            let v9064 = parameters[199];
            let v9066 = parameters[198];
            let v9072 = parameters[200];
            let v9078 = parameters[228];
            let v9084 = 4e-4f64;
            let v9094 = 1e-12f64;
            let v9101 = staged[97];
            let v9111 = -3.4e1f64;
            let v9113 = staged[98];
            let v9190 = 4e-4f64;
            let v9200 = 1e-12f64;
            let v9217 = -3.4e1f64;
            let v9281 = 2.220446049250313e-15f64;
            let v9284 = parameters[45];
            let v9291 = parameters[19];
            let v9293 = parameters[175];
            let v9296 = staged[99];
            let v9299 = 2.220446049250313e-15f64;
            let v9303 = 1e-15f64;
            let v9320 = staged[100];
            let v9325 = 0e0f64;
            let v9330 = 1e0f64;
            let v9332 = if parameter_given[173] { 1.0 } else { 0.0 };
            let v9336 = if parameter_given[174] { 1.0 } else { 0.0 };
            let v9361 = Lanes([0e0f64; 3]);
            let v9375 = -0e0f64;
            let v9401 = parameters[39];
            let v9424 = 4.242640687119285e0f64;
            let v9428 = 8e0f64;
            let v9448 = 9e0f64;
            let v9457 = 9.899494936611664e0f64;
            let v9473 = 4.9787068367863944e-2f64;
            let v9487 = 2.220446049250313e-15f64;
            let v9500 = -9.899494936611664e0f64;
            let v9520 = -9.899494936611664e0f64;
            let v9526 = 3.333333333333333e-1f64;
            let v9528 = -6.666666666666667e-1f64;
            let v9532 = 1.2e1f64;
            let v9535 = -5.65685424949238e0f64;
            let v9543 = 1.414213562373095e0f64;
            let v9586 = Lanes([0e0f64; 4]);
            let v9587 = 2.220446049250313e-15f64;
            let v9630 = 2.220446049250313e-15f64;
            let v9632 = 2.220446049250313e-15f64;
            let v9663 = 7.071067811865476e-1f64;
            let v9673 = -1.047839336957922e-1f64;
            let v9676 = 5.286687693921294e-4f64;
            let v9679 = -5.151950988020902e1f64;
            let v9682 = 1.8773541122053122e-2f64;
            let v9688 = 2.8160311683079683e-2f64;
            let v9691 = 1.0979672760764175e-2f64;
            let v9693 = 7.930031540881942e-4f64;
            let v9718 = -6.666666666666667e-1f64;
            let v9725 = -6.666666666666667e-1f64;
            let v9733 = -3.7209791878387604e0f64;
            let v9749 = parameters[30];
            let v9819 = 2.220446049250313e-15f64;
            let v9861 = 6.0000000000000005e-2f64;
            let v9865 = 6.0000000000000005e-2f64;
            let v9888 = 2.220446049250313e-15f64;
            let v9940 = 4.1e1f64;
            let v9957 = 6.115288895133179e-3f64;
            let v9960 = -7.053654284009761e-2f64;
            let v9966 = 2.9693154855771e-1f64;
            let v9976 = -2.8214617136039044e-1f64;
            let v9982 = 8.907946456731299e-1f64;
            let v10012 = 6.36964918866352e-5f64;
            let v10015 = -1.63730162779191e-3f64;
            let v10021 = 1.78800506338833e-2f64;
            let v10027 = -1.17851130197758e-1f64;
            let v10033 = 7.07106781186548e-1f64;
            let v10041 = -6.54920651116764e-3f64;
            let v10047 = 5.3640151901649905e-2f64;
            let v10053 = -2.35702260395516e-1f64;
            let v10112 = -1e0f64;
            let v10176 = 4.1e1f64;
            let v10197 = 5e-2f64;
            let v10212 = -1e0f64;
            let v10248 = 2.220446049250313e-15f64;
            let v10266 = 0e0f64;
            let v10282 = 1e0f64;
            let v10294 = -0e0f64;
            let v10326 = 4.242640687119285e0f64;
            let v10357 = 9.899494936611664e0f64;
            let v10373 = 4.9787068367863944e-2f64;
            let v10387 = 2.220446049250313e-15f64;
            let v10393 = -9.899494936611664e0f64;
            let v10413 = -9.899494936611664e0f64;
            let v10420 = -6.666666666666667e-1f64;
            let v10426 = -5.65685424949238e0f64;
            let v10476 = 2.220446049250313e-15f64;
            let v10519 = 2.220446049250313e-15f64;
            let v10521 = 2.220446049250313e-15f64;
            let v10552 = 7.071067811865476e-1f64;
            let v10562 = -1.047839336957922e-1f64;
            let v10565 = 5.286687693921294e-4f64;
            let v10568 = -5.151950988020902e1f64;
            let v10571 = 1.8773541122053122e-2f64;
            let v10577 = 2.8160311683079683e-2f64;
            let v10580 = 1.0979672760764175e-2f64;
            let v10582 = 7.930031540881942e-4f64;
            let v10607 = -6.666666666666667e-1f64;
            let v10614 = -6.666666666666667e-1f64;
            let v10622 = -3.7209791878387604e0f64;
            let v10707 = 2.220446049250313e-15f64;
            let v10749 = 6.0000000000000005e-2f64;
            let v10753 = 6.0000000000000005e-2f64;
            let v10776 = 2.220446049250313e-15f64;
            let v10822 = 4.1e1f64;
            let v10841 = -7.053654284009761e-2f64;
            let v10856 = -2.8214617136039044e-1f64;
            let v10862 = 8.907946456731299e-1f64;
            let v10894 = -1.63730162779191e-3f64;
            let v10905 = -1.17851130197758e-1f64;
            let v10918 = -6.54920651116764e-3f64;
            let v10924 = 5.3640151901649905e-2f64;
            let v10930 = -2.35702260395516e-1f64;
            let v10989 = -1e0f64;
            let v11053 = 4.1e1f64;
            let v11074 = 5e-2f64;
            let v11089 = -1e0f64;
            let v11125 = 2.220446049250313e-15f64;
            let v11146 = parameters[174];
            let v11148 = parameters[173];
            let v11151 = staged[101];
            let v11206 = parameters[21];
            let v11210 = parameters[223];
            let v11211 = parameters[224];
            let v11241 = parameters[225];
            let v11247 = -2e0f64;
            let v11255 = 2.220446049250313e-15f64;
            let v11268 = 1e5f64;
            let v11271 = 9.999999999999978e-1f64;
            let v11272 = parameters[114];
            let v11274 = 1.0000000000000022e0f64;
            let v11284 = 1.9999999999999978e0f64;
            let v11286 = 2.000000000000002e0f64;
            let v11296 = -1e0f64;
            let v11333 = 6e0f64;
            let v11380 = 1.5e1f64;
            let v11420 = 4.2e1f64;
            let v11492 = 3.872983346207417e0f64;
            let v11535 = if parameter_given[172] { 1.0 } else { 0.0 };
            let v11536 = parameters[172];
            let v11538 = parameters[0];
            let v11546 = Lanes([0e0f64; 4]);
            let v11549 = 2.1983327444149834e-11f64;
            let v11551 = parameters[171];
            let v11571 = -5e-1f64;
            let v11576 = -5e-1f64;
            let v11592 = parameters[303];
            let v11603 = Lanes([0e0f64; 8]);
            let v11636 = parameters[46];
            let v11649 = 2.069886e-10f64;
            let v11652 = 1.3e0f64;
            let v11664 = staged[102];
            let v11672 = parameters[14];
            let v11752 = 5.5224904e-23f64;
            let v11761 = 1e-6f64;
            let v11765 = 1.898893985185185e-20f64;
            let v11777 = 2.220446049250313e-15f64;
            let v11779 = 2.220446049250313e-15f64;
            let v11809 = 6.666666666666667e-1f64;
            let v11865 = 5e-1f64;
            let v11874 = 5e-1f64;
            let v11879 = staged[188];
            let v11880 = Lanes([0e0f64; 3]);
            let v11883 = staged[190];
            let v11890 = parameters[320];
            let v11892 = staged[135];
            let v11896 = staged[103];
            let v11901 = 1.8e0f64;
            let v11905 = parameters[321];
            let v11910 = staged[104];
            let v11915 = parameters[325];
            let v11918 = parameters[324];
            let v11920 = staged[105];
            let v11923 = staged[106];
            let v11926 = staged[107];
            let v11930 = parameters[311];
            let v11954 = 9.999999999999978e-1f64;
            let v11956 = 1.0000000000000022e0f64;
            let v11959 = 1.9999999999999978e0f64;
            let v11961 = 2.000000000000002e0f64;
            let v11971 = 9.999999999999978e-1f64;
            let v11973 = 1.0000000000000022e0f64;
            let v11993 = 1.9999999999999978e0f64;
            let v11995 = 2.000000000000002e0f64;
            let v12005 = staged[108];
            let v12008 = staged[109];
            let v12020 = -1e0f64;
            let v12050 = staged[110];
            let v12057 = Lanes([0e0f64; 3]);
            let v12067 = staged[136];
            let v12071 = staged[111];
            let v12083 = staged[112];
            let v12090 = parameters[323];
            let v12092 = staged[113];
            let v12095 = staged[114];
            let v12098 = staged[115];
            let v12102 = parameters[310];
            let v12126 = 9.999999999999978e-1f64;
            let v12128 = 1.0000000000000022e0f64;
            let v12131 = 1.9999999999999978e0f64;
            let v12133 = 2.000000000000002e0f64;
            let v12143 = 9.999999999999978e-1f64;
            let v12145 = 1.0000000000000022e0f64;
            let v12165 = 1.9999999999999978e0f64;
            let v12167 = 2.000000000000002e0f64;
            let v12177 = staged[116];
            let v12191 = -1e0f64;
            let v12221 = staged[117];
            let v12229 = Lanes([0e0f64; 2]);
            let v12230 = Lanes([0e0f64; 7]);
            let v12231 = Lanes([0e0f64; 7]);
            let v12275 = 5e-1f64;
            let v12332 = parameters[312];
            let v12344 = parameters[313];
            let v12361 = ddt_scale();
            let v12381 = node_potentials[7];
            let v12384 = Lanes([1e0f64; 1]);
            let v12405 = parameters[25];
            let v12406 = node_potentials[1];
            let v12408 = Lanes([1e0f64; 1]);
            let v12412 = staged[118];
            let v12415 = Lanes([0e0f64; 2]);
            let v12420 = staged[119];
            let v12428 = staged[120];
            let v12436 = staged[193];
            let v12596 = 0e0f64;
            let v12597 = 0e0f64;
            let v12598 = 0e0f64;
            let v12599 = 0e0f64;
            let v12600 = 0e0f64;
            let v12601 = 0e0f64;
            let v12602 = 0e0f64;
            if v3 != 0.0 {
                if v4 != 0.0 {
                    loop {
                        if v5 == 0.0 {
                            break;
                        }
                    }
                } else {
                }
            } else {
            }
            if v7 != 0.0 {
                loop {
                    if v8 == 0.0 {
                        break;
                    }
                }
            } else {
            }
            let v18 = v17 * (v9 - v10);
            let v19 = ((Lanes([v12[0], 0.0])) - (Lanes([0.0, v14[0]]))) * v17;
            let v26 = v17 * (v20 - v10);
            let v27 = ((Lanes([v22[0], 0.0])) - (Lanes([0.0, v14[0]]))) * v17;
            let v34 = v17 * (v28 - v10);
            let v35 = ((Lanes([v30[0], 0.0])) - (Lanes([0.0, v14[0]]))) * v17;
            let v42 = v17 * (v9 - v36);
            let v43 = ((Lanes([0.0, v12[0]])) - (Lanes([v39[0], 0.0]))) * v17;
            let v50 = v17 * (v44 - v36);
            let v51 = ((Lanes([v46[0], 0.0])) - (Lanes([0.0, v39[0]]))) * v17;
            let v56 = v17 * (v28 - v36);
            let v57 = ((Lanes([0.0, v30[0]])) - (Lanes([v39[0], 0.0]))) * v17;
            let v63: f64;
            let v64: Lanes<1>;
            if v58 != 0.0 {
                let v61 = if v59 > v60 { 1.0 } else { 0.0 };
                let v67: f64;
                let v68: Lanes<1>;
                if v61 != 0.0 {
                    v67 = v59;
                    v68 = v66;
                } else {
                    v67 = v60;
                    v68 = v62;
                }
                v63 = v67;
                v64 = v68;
            } else {
                v63 = v60;
                v64 = v62;
            }
            let v80: f64;
            let v81: f64;
            let v82: Lanes<1>;
            let v83: Lanes<1>;
            if v65 != 0.0 {
                let v71 = v69 * v70;
                let v73 = v72 * v69;
                let v75 = v69 * v74;
                let v77 = v76 * v69;
                v80 = v71;
                v81 = v75;
                v82 = v73;
                v83 = v77;
            } else {
                v80 = v60;
                v81 = v60;
                v82 = v78;
                v83 = v79;
            }
            let v84 = if v26 >= v60 { 1.0 } else { 0.0 };
            let v112: f64;
            let v113: f64;
            let v114: f64;
            let v115: f64;
            let v116: f64;
            let v117: f64;
            let v118: f64;
            let v119: f64;
            let v120: f64;
            let v121: Lanes<3>;
            let v122: Lanes<2>;
            let v123: Lanes<3>;
            let v124: Lanes<3>;
            let v125: Lanes<2>;
            let v126: Lanes<3>;
            if v84 != 0.0 {
                let v85 = Lanes([v35[0], 0.0, v35[1]]);
                let v86 = Lanes([v19[0], 0.0, v19[1]]);
                let v87 = Lanes([0.0, v43[0], v43[1]]);
                let v88 = Lanes([0.0, v57[0], v57[1]]);
                v112 = v34;
                v113 = v26;
                v114 = v18;
                v115 = v89;
                v116 = v60;
                v117 = v42;
                v118 = v50;
                v119 = v89;
                v120 = v56;
                v121 = v85;
                v122 = v27;
                v123 = v86;
                v124 = v87;
                v125 = v51;
                v126 = v88;
            } else {
                let v90 = v18 - v26;
                let v93 = (Lanes([v19[0], 0.0, v19[1]])) - (Lanes([0.0, v27[0], v27[1]]));
                let v94 = -v26;
                let v96 = v27 * v95;
                let v97 = v34 - v26;
                let v100 = (Lanes([v35[0], 0.0, v35[1]])) - (Lanes([0.0, v27[0], v27[1]]));
                let v101 = v42 - v50;
                let v104 = (Lanes([0.0, v43[0], v43[1]])) - (Lanes([v51[0], v51[1], 0.0]));
                let v105 = -v50;
                let v106 = v51 * v95;
                let v107 = v56 - v50;
                let v110 = (Lanes([0.0, v57[0], v57[1]])) - (Lanes([v51[0], v51[1], 0.0]));
                v112 = v97;
                v113 = v94;
                v114 = v90;
                v115 = v60;
                v116 = v89;
                v117 = v101;
                v118 = v105;
                v119 = v111;
                v120 = v107;
                v121 = v100;
                v122 = v96;
                v123 = v93;
                v124 = v104;
                v125 = v106;
                v126 = v110;
            }
            let v128 = v127 + v63;
            let v130 = v64 * v128;
            let v135 = v128 - v134;
            let v145 = (v139 - (v136 * v135)) - (v142 * ((v128 * v128) - v132));
            let v146 = ((v64 * v136) * v95) - ((v130 + v130) * v142);
            let v148 = v147 * v128;
            let v151 = v150 / v148;
            let v154 = (((v64 * v147) * v151) * v95) / v148;
            let v155 = v151 * v151;
            let v156 = v154 * v151;
            let v157 = v156 + v156;
            let v158 = v89 / v151;
            let v161 = ((v154 * v158) * v95) / v151;
            let v162 = v128 / v134;
            let v163 = v64 / v134;
            let v171 = v170 * (v162.powf(v164));
            let v172 = (v163 * (v164 * (v162.powf(v166)))) * v170;
            let v173 = v162 - v89;
            let v175 = v174 * v173;
            let v182 = v181 + (v175 * v173);
            let v183 = v162.powf(v182);
            let v192 = (v163 * (v182 * (v162.powf((v182 - v184))))) + ((((v163 * v174) * v173) + (v163 * v175)) * (v183 * (v162.ln())));
            let v194 = v183 / v193;
            let v195 = v192 / v193;
            let v197 = v183 / v196;
            let v198 = v192 / v196;
            let v200 = v199 * v158;
            let v201 = v161 * v199;
            let v203 = v202 * v162;
            let v204 = v163 * v202;
            let v211 = v210 * v162;
            let v213 = v211 * v162;
            let v216 = ((v163 * v210) * v162) + (v163 * v211);
            let v221 = v89 - v162;
            let v222 = v163 * v95;
            let v226 = ((v208 + (v203 * v205)) + (v213 * v205)) - (v223 * v221);
            let v229 = v228 / v226;
            let v233 = v205 * v229;
            let v234 = ((((((v204 * v205) + (v216 * v205)) - (v222 * v223)) * v229) * v95) / v226) * v205;
            let v235 = v145.sqrt();
            let v239 = v146 * (v184 / (v236 * v235));
            let v240 = v145 * v235;
            let v243 = (v146 * v235) + (v239 * v145);
            let v244 = v162.sqrt();
            let v250 = v249 * (v162 * v244);
            let v255 = (-v145) / v254;
            let v263 = ((v255 * v151) + v261).exp();
            let v265 = v250 * v263;
            let v268 = (((v163 * (v246 * v244)) * v249) * v263) + ((((((v146 * v95) / v254) * v151) + (v154 * v255)) * v263) * v250);
            let v269 = v158.sqrt();
            let v274 = v273 * v269;
            let v275 = (v161 * (v184 / (v236 * v269))) * v273;
            let v276 = v274 * v274;
            let v277 = v275 * v274;
            let v278 = v277 + v277;
            let v279 = v265 * v265;
            let v280 = v268 * v265;
            let v281 = v280 + v280;
            let v283 = v279 * v282;
            let v284 = v281 * v282;
            let v285 = v254 * v158;
            let v286 = v161 * v254;
            let v288 = v287 / v265;
            let v292 = v288.ln();
            let v295 = v285 * v292;
            let v298 = (v286 * v292) + (((((v268 * v288) * v95) / v265) * (v184 / v288)) * v285);
            let v302 = (v299 * v158).sqrt();
            let v307 = v306 * v302;
            let v308 = ((v161 * v299) * (v184 / (v236 * v302))) * v306;
            let v312 = (v309 * v158).sqrt();
            let v315 = (v161 * v309) * (v184 / (v236 * v312));
            let v316 = v265 / v287;
            let v318 = v316 * v316;
            let v319 = (v268 / v287) * v316;
            let v320 = v319 + v319;
            let v322 = v265 / v321;
            let v324 = v322 * v322;
            let v325 = (v268 / v321) * v322;
            let v326 = v325 + v325;
            let v328 = if v112 > v327 { 1.0 } else { 0.0 };
            let v353: f64;
            let v354: f64;
            let v355: Lanes<3>;
            let v356: Lanes<3>;
            if v328 != 0.0 {
                let v329 = v112 - v327;
                let v331 = v330 - v327;
                let v332 = v329 * v329;
                let v333 = v121 * v329;
                let v334 = v333 + v333;
                let v335 = v331 * v331;
                let v336 = v332 * v332;
                let v337 = v334 * v332;
                let v340 = v336 * v332;
                let v348 = ((((v337 + v337) * v332) + (v334 * v336)) * v332) + (v334 * v340);
                let v349 = ((v335 * v335) * v335) * v335;
                let v350 = (v340 * v332) + v349;
                let v416: f64;
                let v417: Lanes<3>;
                if v351 != 0.0 {
                    let v440: f64;
                    if v409 != 0.0 {
                        v440 = v89;
                    } else {
                        let v442: f64;
                        if v439 != 0.0 {
                            v442 = v254;
                        } else {
                            let v445: f64;
                            if v441 != 0.0 {
                                v445 = v443;
                            } else {
                                let v447: f64;
                                if v444 != 0.0 {
                                    v447 = v446;
                                } else {
                                    v447 = v60;
                                }
                                v445 = v447;
                            }
                            v442 = v445;
                        }
                        v440 = v442;
                    }
                    let mut v448: f64 = 0.0;
                    let mut v449: f64 = 0.0;
                    let mut v450: Lanes<3> = Lanes([0.0; 3]);
                    v448 = v60;
                    v449 = v350;
                    v450 = v348;
                    loop {
                        let v451 = if v448 < v440 { 1.0 } else { 0.0 };
                        if v451 == 0.0 {
                            break;
                        }
                        let v452 = v449.sqrt();
                        let v455 = v450 * (v184 / (v236 * v452));
                        let v456 = v448 + v89;
                        v448 = v456;
                        v449 = v452;
                        v450 = v455;
                    }
                    v416 = v449;
                    v417 = v450;
                } else {
                    let v411 = v350.powf(v410);
                    let v415 = v348 * (v410 * (v350.powf(v412)));
                    v416 = v411;
                    v417 = v415;
                }
                let v419 = v416 + v418;
                let v420 = v89 / v419;
                let v423 = ((v417 * v420) * v95) / v419;
                let v424 = v329 * v331;
                let v429 = ((v121 * v331) * v420) + (v423 * v424);
                let v430 = v331 * v349;
                let v433 = v350 + v418;
                let v434 = (v430 * v420) / v433;
                let v437 = ((v423 * v430) - (v348 * v434)) / v433;
                let v438 = v327 + (v424 * v420);
                v353 = v438;
                v354 = v434;
                v355 = v429;
                v356 = v437;
            } else {
                v353 = v112;
                v354 = v89;
                v355 = v121;
                v356 = v352;
            }
            let v359 = v122 * v354;
            let v367 = (v254 * ((v354 * v113) / v254)) / v366;
            let v368 = ((((v356 * v113) + (Lanes([0.0, v359[0], v359[1]]))) / v254) * v254) / v366;
            let v373 = v372 + (v367 * v369);
            let v379 = v378 + (v367 * v373);
            let v385 = v384 + (v367 * v379);
            let v391 = v390 + (v367 * v385);
            let v397 = v396 + (v367 * v391);
            let v402 = v89 + (v367 * v397);
            let v403 = v366 / v402;
            let v406 = ((((v368 * v397) + (((v368 * v391) + (((v368 * v385) + (((v368 * v379) + (((v368 * v373) + ((v368 * v369) * v367)) * v367)) * v367)) * v367)) * v367)) * v403) * v95) / v402;
            let v408 = if v403 < v407 { 1.0 } else { 0.0 };
            let v457: f64;
            let v458: Lanes<3>;
            if v408 != 0.0 {
                v457 = v407;
                v458 = v352;
            } else {
                v457 = v403;
                v458 = v406;
            }
            let v459 = v353 + v457;
            let v460 = v355 + v458;
            let v463 = v113 + (v254 * v457);
            let v465 = (Lanes([0.0, v122[0], v122[1]])) + (v458 * v254);
            let v466 = v114 + v457;
            let v467 = Lanes([v123[0], 0.0, v123[1], v123[2]]);
            let v469 = v467 + (Lanes([0.0, v458[0], v458[1], v458[2]]));
            let v473 = (v254 * v470) * v472;
            let v476 = (v473 * v474) * v474;
            let v478 = v114 - v477;
            let v479 = v254 / v476;
            let v483 = (Lanes([0.0, v123[0], v123[1], v123[2]])) - (Lanes([v161[0], 0.0, 0.0, 0.0]));
            let v486 = Lanes([0.0, 0.0, v355[0], v355[1], v355[2]]);
            let v489 = ((Lanes([v483[0], v483[1], 0.0, v483[2], v483[3]])) - v486) * v479;
            let v490 = v89 + (v479 * ((v478 - v158) - v353));
            let v492 = v489 * v490;
            let v496 = ((v490 * v490) + v494).sqrt();
            let v504 = (v489 + ((v492 + v492) * (v184 / (v236 * v496)))) * v502;
            let v506 = (v502 * (v490 + v496)) + v505;
            let v507 = if v506 < v60 { 1.0 } else { 0.0 };
            let v509: f64;
            let v510: Lanes<5>;
            if v507 != 0.0 {
                v509 = v60;
                v510 = v508;
            } else {
                v509 = v506;
                v510 = v504;
            }
            let v512 = (v509 + v418).sqrt();
            let v521 = Lanes([0.0, v123[0], 0.0, v123[1], v123[2]]);
            let v525 = (v521 + (((v510 * (v184 / (v236 * v512))) * v95) * v476)) - (Lanes([v298[0], 0.0, 0.0, 0.0, 0.0]));
            let v528 = (((v478 + (v476 * (v89 - v512))) - v295) - v210) - v527;
            let v532: f64;
            if v529 != 0.0 {
                v532 = v530;
            } else {
                v532 = v531;
            }
            let v534 = v525 * v528;
            let v537 = ((v528 * v528) + v532).sqrt();
            let v545 = v210 + (v502 * (v528 + v537));
            let v546 = v113 / v545;
            let v548 = Lanes([0.0, 0.0, 0.0, v122[0], v122[1]]);
            let v550 = (v548 - (((v525 + ((v534 + v534) * (v184 / (v236 * v537)))) * v502) * v546)) / v545;
            let v551 = v546 * v546;
            let v552 = v550 * v546;
            let v553 = v552 + v552;
            let v559 = v553 * v551;
            let v566 = (((v89 + v546) + v551) + (v551 * v546)) + (v551 * v551);
            let v568 = v89 / v566;
            let v572 = v89 - v568;
            let v574 = v572 * v572;
            let v575 = (((((((v550 + v553) + ((v553 * v546) + (v550 * v551))) + (v559 + v559)) * v568) * v95) / v566) * v95) * v572;
            let v576 = v575 + v575;
            let v584 = if (if (if v577 == v60 { 1.0 } else { 0.0 }) != 0.0 && (if v579 == v60 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v582 == v60 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v585: f64;
            if v584 != 0.0 {
                v585 = v60;
            } else {
                v585 = v89;
            }
            let v592 = v591 + (((v473 * v586).sqrt()) / v589);
            let v593 = if v585 == v60 { 1.0 } else { 0.0 };
            let v625: f64;
            let v626: f64;
            let v627: f64;
            let v628: Lanes<4>;
            let v629: Lanes<5>;
            let v630: Lanes<4>;
            if v593 != 0.0 {
                let v596 = (v307 * v474) * v474;
                let v598 = v596 * v307;
                let v601 = (((v308 * v474) * v474) * v307) + (v308 * v596);
                let v602 = Lanes([v601[0], 0.0, 0.0, 0.0, 0.0]);
                v625 = v474;
                v626 = v598;
                v627 = v589;
                v628 = v603;
                v629 = v602;
                v630 = v603;
            } else {
                let v606 = v467 - (Lanes([0.0, v355[0], v355[1], v355[2]]));
                let v608 = ((v114 - v353) - v592) + v582;
                let v610 = v606 * v608;
                let v614 = ((v608 * v608) + v612).sqrt();
                let v621 = (v606 + ((v610 + v610) * (v184 / (v236 * v614)))) * v502;
                let v623 = (v502 * (v608 + v614)) + v622;
                let v624 = if v623 < v60 { 1.0 } else { 0.0 };
                let v635: f64;
                let v636: Lanes<4>;
                if v624 != 0.0 {
                    v635 = v60;
                    v636 = v603;
                } else {
                    v635 = v623;
                    v636 = v621;
                }
                let v637 = v89 / v635;
                let v640 = ((v636 * v637) * v95) / v635;
                let v642 = v254 * (v592.abs());
                let v644 = (v477 - v592) + v582;
                let v645 = if v644 > v642 { 1.0 } else { 0.0 };
                let v646: f64;
                if v645 != 0.0 {
                    v646 = v644;
                } else {
                    v646 = v642;
                }
                let v647 = v89 / v646;
                let v649 = v640 * v95;
                let v651 = (v647 - v637) - v650;
                let v653 = (v446 * v647) * v650;
                let v654 = if v653 > v60 { 1.0 } else { 0.0 };
                let v656: f64;
                if v654 != 0.0 {
                    v656 = v653;
                } else {
                    let v655 = -v653;
                    v656 = v655;
                }
                let v658 = v649 * v651;
                let v661 = ((v651 * v651) + v656).sqrt();
                let v672 = (((v649 + ((v658 + v658) * (v184 / (v236 * v661)))) * v502) * v95) * v577;
                let v673 = (v577 * (v647 - (v502 * (v651 + v661)))) + v579;
                let v677 = if (v673 * v674) < v676 { 1.0 } else { 0.0 };
                let v678: f64;
                let v679: Lanes<4>;
                if v677 != 0.0 {
                    v678 = v60;
                    v679 = v603;
                } else {
                    v678 = v673;
                    v679 = v672;
                }
                let v680 = v676 + v678;
                let v682 = v681 / v680;
                let v685 = ((v679 * v682) * v95) / v680;
                let v686 = v680 / v681;
                let v687 = v679 / v681;
                let v688 = v307 * v307;
                let v689 = v308 * v307;
                let v691 = v688 * v686;
                let v692 = (v689 + v689) * v686;
                let v693 = v687 * v688;
                let v697 = v691 * v686;
                let v699 = v687 * v691;
                let v701 = (((Lanes([v692[0], 0.0, 0.0, 0.0, 0.0])) + (Lanes([0.0, v693[0], v693[1], v693[2], v693[3]]))) * v686) + (Lanes([0.0, v699[0], v699[1], v699[2], v699[3]]));
                v625 = v686;
                v626 = v697;
                v627 = v682;
                v628 = v687;
                v629 = v701;
                v630 = v685;
            }
            let v632 = v460 * v95;
            let v634 = (v502 - v459) - v633;
            let v703 = v632 * v634;
            let v707 = ((v634 * v634) + v705).sqrt();
            let v719 = v628 * v717;
            let v721 = (v591 + (v717 * v625)) + v200;
            let v724 = (Lanes([0.0, v719[0], v719[1], v719[2], v719[3]])) + (Lanes([v201[0], 0.0, 0.0, 0.0, 0.0]));
            let v727 = (((v632 + ((v703 + v703) * (v184 / (v236 * v707)))) * v502) * v95) * v95;
            let v728 = (v725 - (v502 - (v502 * (v634 + v707)))) - v633;
            let v730 = v727 * v728;
            let v734 = ((v728 * v728) + v732).sqrt();
            let v744 = v586 - (v725 - (v502 * (v728 + v734)));
            let v745 = (((v727 + ((v730 + v730) * (v184 / (v236 * v734)))) * v502) * v95) * v95;
            let v746 = v744.sqrt();
            let v749 = v745 * (v184 / (v236 * v746));
            let v787: f64;
            let v788: Lanes<5>;
            if v750 != 0.0 {
                let v753 = v628 * v751;
                let v762 = ((v472 * v625) * v758) * v761;
                let v772 = (v767 + (v764 * v744)) + (v769 * v463);
                let v774 = v721 - (v754 + (v751 * v625));
                let v777 = v774 * v762;
                let v779 = (((v628 * v472) * v758) * v761) * v774;
                let v782 = v777 * v772;
                let v784 = ((v745 * v764) + (v465 * v769)) * v777;
                let v786 = ((((v724 - (Lanes([0.0, v753[0], v753[1], v753[2], v753[3]]))) * v762) + (Lanes([0.0, v779[0], v779[1], v779[2], v779[3]]))) * v772) + (Lanes([0.0, 0.0, v784[0], v784[1], v784[2]]));
                v787 = v782;
                v788 = v786;
            } else {
                v787 = v60;
                v788 = v508;
            }
            let v809: f64;
            let v810: Lanes<5>;
            if v789 != 0.0 {
                let v792 = v154 * v626;
                let v800 = (Lanes([v161[0], 0.0, 0.0, 0.0, 0.0])) - (((v629 * v151) + (Lanes([v792[0], 0.0, 0.0, 0.0, 0.0]))) * v795);
                let v802 = ((v158 - ((v626 * v151) * v795)) + v477) + v418;
                let v805 = (Lanes([0.0, v469[0], v469[1], v469[2], v469[3]])) - v800;
                let v807 = (v466 - v802) - v806;
                let v808 = if v802 >= v60 { 1.0 } else { 0.0 };
                let v846: f64;
                if v808 != 0.0 {
                    v846 = v89;
                } else {
                    v846 = v845;
                }
                let v848 = v805 * v807;
                let v850 = v846 * v446;
                let v857 = ((v807 * v807) + ((v850 * v802) * v806)).sqrt();
                let v866 = v800 + ((v805 + (((v848 + v848) + ((v800 * v850) * v806)) * (v184 / (v236 * v857)))) * v502);
                let v867 = (v802 + (v502 * (v807 + v857))) - v477;
                let v868 = v446 / v626;
                let v872 = v868 * v158;
                let v874 = v161 * v868;
                let v877 = v872 * v158;
                let v879 = v161 * v872;
                let v883 = v154 * v867;
                let v887 = (v151 * v867) - v89;
                let v891 = (((Lanes([v883[0], 0.0, 0.0, 0.0, 0.0])) + (v866 * v151)) * v877) + ((((((((v629 * v868) * v95) / v626) * v158) + (Lanes([v874[0], 0.0, 0.0, 0.0, 0.0]))) * v158) + (Lanes([v879[0], 0.0, 0.0, 0.0, 0.0]))) * v887);
                let v892 = v89 + (v887 * v877);
                let v894 = v891 * v892;
                let v898 = ((v892 * v892) + v896).sqrt();
                let v905 = (v891 + ((v894 + v894) * (v184 / (v236 * v898)))) * v502;
                let v907 = (v502 * (v892 + v898)) + v906;
                let v908 = if v907 < v60 { 1.0 } else { 0.0 };
                let v909: f64;
                let v910: Lanes<5>;
                if v908 != 0.0 {
                    v909 = v60;
                    v910 = v508;
                } else {
                    v909 = v907;
                    v910 = v905;
                }
                let v913 = (v909 + v911).sqrt();
                let v917 = v626 * v502;
                let v919 = v917 * v151;
                let v921 = v154 * v917;
                let v924 = v89 - v913;
                let v933 = (v866 + (((((v629 * v502) * v151) + (Lanes([v921[0], 0.0, 0.0, 0.0, 0.0]))) * v924) + (((v910 * (v184 / (v236 * v913))) * v95) * v919))) * v95;
                let v934 = (v586 - (v867 + (v919 * v924))) - v806;
                let v936 = v933 * v934;
                let v940 = ((v934 * v934) + v938).sqrt();
                let v953 = (((v933 + ((v936 + v936) * (v184 / (v236 * v940)))) * v502) * v95) * v951;
                let v954 = v586 + (v951 * ((v586 - (v502 * (v934 + v940))) - v586));
                v809 = v954;
                v810 = v953;
            } else {
                v809 = v586;
                v810 = v508;
            }
            let v811 = v625 * v472;
            let v812 = v628 * v472;
            let v816 = (v811 * v813) * v254;
            let v819 = v818 - v809;
            let v820 = v810 * v95;
            let v822 = ((v812 * v813) * v254) * v819;
            let v827 = (v816 * v819) / v826;
            let v828 = ((Lanes([0.0, v822[0], v822[1], v822[2], v822[3]])) + (v820 * v816)) / v826;
            let v830 = v355 * v353;
            let v834 = ((v353 * v353) + v832).sqrt();
            let v841 = (v355 + ((v830 + v830) * (v184 / (v236 * v834)))) * v502;
            let v843 = (v502 * (v353 + v834)) + v842;
            let v844 = if v843 < v60 { 1.0 } else { 0.0 };
            let v955: f64;
            let v956: Lanes<3>;
            if v844 != 0.0 {
                v955 = v60;
                v956 = v352;
            } else {
                v955 = v843;
                v956 = v841;
            }
            let v970 = ((v960 + (v957 * v744)) + (v962 * v463)) + (v967 * v955);
            let v972 = v827 * v970;
            let v974 = (((v745 * v957) + (v465 * v962)) + (v956 * v967)) * v827;
            let v976 = (v828 * v970) + (Lanes([0.0, 0.0, v974[0], v974[1], v974[2]]));
            let v993: f64;
            let v994: Lanes<4>;
            if v977 != 0.0 {
                let v979 = v146 + v298;
                let v984 = v465 * v982;
                let v990 = (((v145 + v295) - v980) + (v982 * v463)) * v989;
                let v991 = ((Lanes([v979[0], 0.0, 0.0, 0.0])) + (Lanes([0.0, v984[0], v984[1], v984[2]]))) * v989;
                v993 = v990;
                v994 = v991;
            } else {
                v993 = v60;
                v994 = v992;
            }
            let v996 = v627 + v995;
            let v997 = v89 / v996;
            let v1004 = (v628 - (((v630 * v997) * v95) / v996)) * v717;
            let v1007 = v972 + v787;
            let v1008 = v976 + v788;
            let v1014 = (v1008 + (Lanes([0.0, v1004[0], v1004[1], v1004[2], v1004[3]]))) + (Lanes([v994[0], 0.0, v994[1], v994[2], v994[3]]));
            let v1016 = ((v1007 + ((v717 * (v625 - v997)) + v1005)) + v993) + v1015;
            let v1017 = v721 - v1016;
            let v1023: f64;
            let v1024: Lanes<4>;
            if v1018 != 0.0 {
                v1023 = v60;
                v1024 = v603;
            } else {
                let v1020 = v466 - v1019;
                let v1022 = if v1020 < v1021 { 1.0 } else { 0.0 };
                let v1046: f64;
                let v1047: Lanes<4>;
                if v1022 != 0.0 {
                    v1046 = v60;
                    v1047 = v603;
                } else {
                    let v1045 = if v1020 < v60 { 1.0 } else { 0.0 };
                    let v1101: f64;
                    let v1102: Lanes<4>;
                    if v1045 != 0.0 {
                        let v1069 = v1068 + (v1020 * v1065);
                        let v1074 = v89 + (v1020 * v1069);
                        let v1078 = (v469 * v1074) + (((v469 * v1069) + ((v469 * v1065) * v1020)) * v1020);
                        let v1079 = v89 + (v1020 * v1074);
                        v1101 = v1079;
                        v1102 = v1078;
                    } else {
                        let v1084 = v1083 + (v1020 * v1080);
                        let v1090 = v1089 + (v1020 * v1084);
                        let v1095 = v89 + (v1020 * v1090);
                        let v1099 = (v469 * v1095) + (((v469 * v1090) + (((v469 * v1084) + ((v469 * v1080) * v1020)) * v1020)) * v1020);
                        let v1100 = v89 + (v1020 * v1095);
                        v1101 = v1100;
                        v1102 = v1099;
                    }
                    v1046 = v1101;
                    v1047 = v1102;
                }
                let v1048 = v1046 - v89;
                let v1050 = v1047 * v1048;
                let v1054 = ((v1048 * v1048) + v1052).sqrt();
                let v1061 = (v1047 + ((v1050 + v1050) * (v184 / (v236 * v1054)))) * v502;
                let v1063 = (v502 * (v1048 + v1054)) + v1062;
                let v1064 = if v1063 < v60 { 1.0 } else { 0.0 };
                let v1103: f64;
                let v1104: Lanes<4>;
                if v1064 != 0.0 {
                    v1103 = v60;
                    v1104 = v603;
                } else {
                    v1103 = v1063;
                    v1104 = v1061;
                }
                let v1109 = (v1104 * v1105) * v95;
                let v1110 = (v89 - (v1103 * v1105)) - v527;
                let v1112 = v1109 * v1110;
                let v1116 = ((v1110 * v1110) + v1114).sqrt();
                let v1124 = v89 - (v502 * (v1110 + v1116));
                let v1125 = ((v1109 + ((v1112 + v1112) * (v184 / (v236 * v1116)))) * v502) * v95;
                v1023 = v1124;
                v1024 = v1125;
            }
            let v1027 = (v478 + v1016) - v1023;
            let v1028 = Lanes([0.0, v1024[0], v1024[1], v1024[2], v1024[3]]);
            let v1029 = (v521 + v1014) - v1028;
            let v1031 = v158 * v1030;
            let v1032 = v161 * v1030;
            let v1034 = (v477 - v1016) + v1023;
            let v1035 = v307 * v625;
            let v1036 = v308 * v625;
            let v1037 = v628 * v307;
            let v1041 = v1035 * v1035;
            let v1042 = ((Lanes([v1036[0], 0.0, 0.0, 0.0, 0.0])) + (Lanes([0.0, v1037[0], v1037[1], v1037[2], v1037[3]]))) * v1035;
            let v1043 = v1042 + v1042;
            let v1134: f64;
            let v1135: Lanes<4>;
            if v1044 != 0.0 {
                let v1126 = v459 + v1031;
                let v1129 = (Lanes([0.0, v460[0], v460[1], v460[2]])) + (Lanes([v1032[0], 0.0, 0.0, 0.0]));
                v1134 = v1126;
                v1135 = v1129;
            } else {
                let v1130 = v353 + v1031;
                let v1133 = (Lanes([0.0, v355[0], v355[1], v355[2]])) + (Lanes([v1032[0], 0.0, 0.0, 0.0]));
                v1134 = v1130;
                v1135 = v1133;
            }
            let v1136 = if v1134 < v60 { 1.0 } else { 0.0 };
            if v1136 != 0.0 {
                let v1137 = v321 / v287;
                let v1138 = v1137 + v89;
                let v1142 = (v158 - v1134) + (v1137 * (v158 + v1134));
                let v1146 = ((v312 * v312) * v1144) * v1144;
                let v1149 = v1146 * v151;
                let v1150 = ((v254 * v1142) * v1138) - v1149;
                let v1160 = if ((v1150 * v1150) - (((v446 * v1138) * v1138) * (((v1142 * v1142) + (v1149 * v1134)) + v1146))) >= v418 { 1.0 } else { 0.0 };
            } else {
                let v1163 = v307 * v307;
                let v1167 = -(v158 + (v254 * v1134));
                let v1169 = v89 + ((v1163 * v151) / ((v312 * v312) * v151));
                let v1175 = (((v1163 * v1144) * v1144) * v151) - ((v254 * v1167) * v1169);
                let v1182 = if ((v1175 * v1175) - ((((v446 * v1169) * v1169) * v1167) * v1167)) >= v418 { 1.0 } else { 0.0 };
            }
            let v1183 = v254 / v151;
            let v1186 = ((v154 * v1183) * v95) / v151;
            let v1189 = v1183 * ((v321 / v265).ln());
            let v1191 = v315 * v312;
            let v1196 = ((v312 * v312) * v1193) * v1193;
            let v1197 = ((v1191 + v1191) * v1193) * v1193;
            let v1198 = -v1134;
            let v1199 = v1135 * v95;
            let v1202 = v1196 * v151;
            let v1205 = (v1197 * v151) + (v154 * v1196);
            let v1206 = (v254 * v1198) + v1202;
            let v1208 = (v1199 * v254) + (Lanes([v1205[0], 0.0, 0.0, 0.0]));
            let v1210 = v1198 * v1198;
            let v1211 = v1199 * v1198;
            let v1212 = v1211 + v1211;
            let v1214 = Lanes([v1197[0], 0.0, 0.0, 0.0]);
            let v1217 = (v1212 + v1214) * v446;
            let v1218 = (v1206 * v1206) - (v446 * (v1210 + v1196));
            let v1220 = if v1218 >= v1219 { 1.0 } else { 0.0 };
            let v1222: f64;
            if v1220 != 0.0 {
                v1222 = v1218;
            } else {
                v1222 = v1221;
            }
            let v1225 = (v1206 - (v1222.sqrt())) / v254;
            let v1226 = v1210 / v1196;
            let v1227 = v1197 * v1226;
            let v1230 = (v1212 - (Lanes([v1227[0], 0.0, 0.0, 0.0]))) / v1196;
            let v1231 = v1226 / v324;
            let v1232 = v326 * v1231;
            let v1233 = Lanes([v1232[0], 0.0, 0.0, 0.0]);
            let v1235 = v184 / v1231;
            let v1236 = v254 / v1198;
            let v1240 = v151 + v1236;
            let v1241 = Lanes([v154[0], 0.0, 0.0, 0.0]);
            let v1243 = (v1231.ln()) / v1240;
            let v1244 = (v1241 + (((v1199 * v1236) * v95) / v1198)) * v1243;
            let v1245 = if v1225 < v1189 { 1.0 } else { 0.0 };
            let v1252: f64;
            if v1245 != 0.0 {
                v1252 = v1225;
            } else {
                let v1248 = (v1243 - v1225) - v1247;
                let v1250 = (v446 * v1243) * v1247;
                let v1251 = if v1250 > v60 { 1.0 } else { 0.0 };
                let v1254: f64;
                if v1251 != 0.0 {
                    v1254 = v1250;
                } else {
                    let v1253 = -v1250;
                    v1254 = v1253;
                }
                let v1260 = v1243 - (v502 * (v1248 + (((v1248 * v1248) + v1254).sqrt())));
                v1252 = v1260;
            }
            let mut v1261: f64 = 0.0;
            let mut v1262: f64 = 0.0;
            let mut v1263: f64 = 0.0;
            let mut v1264: f64 = 0.0;
            v1261 = v60;
            v1262 = v1252;
            v1263 = v60;
            v1264 = v60;
            loop {
                let v1266 = if v1261 < v1265 { 1.0 } else { 0.0 };
                if v1266 == 0.0 {
                    break;
                }
                let v1267 = v151 * v1262;
                let v1269 = (-v1267).exp();
                let v1271 = if v1262 > v1270 { 1.0 } else { 0.0 };
                let v1296: f64;
                let v1297: f64;
                if v1271 != 0.0 {
                    let v1278 = v1267.exp();
                    let v1286 = (-v312) * ((((v1269 + v1267) - v89) + (v324 * (v1278 - v89))).sqrt());
                    let v1293 = (v1287 / v1286) * (((-v1269) + v89) + (v324 * v1278));
                    v1296 = v1286;
                    v1297 = v1293;
                } else {
                    let v1295 = if v1262 < v1294 { 1.0 } else { 0.0 };
                    let v1326: f64;
                    let v1327: f64;
                    if v1295 != 0.0 {
                        let v1313 = v312 * (((v1269 + v1267) - v89).sqrt());
                        let v1317 = (v1287 / v1313) * ((-v1269) + v89);
                        v1326 = v1313;
                        v1327 = v1317;
                    } else {
                        let v1322 = ((-((v1287 / v151).sqrt())) * v151) * v1262;
                        let v1325 = -((v1287 * v151).sqrt());
                        v1326 = v1322;
                        v1327 = v1325;
                    }
                    v1296 = v1326;
                    v1297 = v1327;
                }
                let v1301 = ((v1296 * v1296) + v1299).sqrt();
                let v1304 = v502 * (v89 + (v1296 / v1301));
                let v1308 = (v502 * (v1296 + v1301)) + v1307;
                let v1309 = if v1308 < v60 { 1.0 } else { 0.0 };
                let v1328: f64;
                let v1329: f64;
                if v1309 != 0.0 {
                    v1328 = v60;
                    v1329 = v60;
                } else {
                    v1328 = v1308;
                    v1329 = v1304;
                }
                let v1331 = -v1330;
                let v1333 = (v1331 - v1328) - v69;
                let v1335 = (v446 * v1331) * v69;
                let v1336 = if v1335 > v60 { 1.0 } else { 0.0 };
                let v1338: f64;
                if v1336 != 0.0 {
                    v1338 = v1335;
                } else {
                    let v1337 = -v1335;
                    v1338 = v1337;
                }
                let v1341 = ((v1333 * v1333) + v1338).sqrt();
                let v1347 = v1331 - (v502 * (v1333 + v1341));
                let v1354 = ((((v1347 * v1347) / v254) / v472) / v150) / v287;
                let v1369 = v1262 - (((((-v1262) + (v1296 / v1359)) - v1134) + v1354) / ((v1365 + (v1297 / v1359)) + (((v254 * v1354) * (v1329 * (v1297 * (v502 * (v89 + (v1333 / v1341)))))) / v1347)));
                let v1372 = if ((v1369 - v1262).abs()) < v633 { 1.0 } else { 0.0 };
                let v1373: f64;
                if v1372 != 0.0 {
                    v1373 = v1265;
                } else {
                    v1373 = v1261;
                }
                let v1374 = v1373 + v89;
                v1261 = v1374;
                v1262 = v1369;
                v1263 = v1354;
                v1264 = v1296;
            }
            let v1277 = if (((v1272 * v1263) / v287).sqrt()) > v1276 { 1.0 } else { 0.0 };
            let v1419: f64;
            let v1420: f64;
            let v1421: f64;
            let v1422: f64;
            let v1423: Lanes<5>;
            let v1424: Lanes<5>;
            let v1425: Lanes<5>;
            if v1277 != 0.0 {
                let v1375 = v89 / v627;
                let v1378 = ((v630 * v1375) * v95) / v627;
                let v1379 = v89 / v1359;
                let v1382 = (v1375 + v1380) + v1379;
                let v1383 = v89 / v1382;
                let v1384 = v1378 * v1383;
                let v1386 = (v1384 * v95) / v1382;
                let v1390 = v89 - (v1383 * v1375);
                let v1396 = v1198 + ((v1379 + (v502 * v1380)) * (-v1330));
                let v1397 = v1383 * v1396;
                let v1398 = v1386 * v1396;
                let v1399 = v1199 * v1383;
                let v1404 = v1378 * v1397;
                let v1408 = (v1375 * v1397) / v1390;
                let v1409 = (((v1386 * v1375) + v1384) * v95) * v1408;
                let v1412 = (((Lanes([0.0, v1404[0], v1404[1], v1404[2], v1404[3]])) + (((Lanes([0.0, v1398[0], v1398[1], v1398[2], v1398[3]])) + (Lanes([v1399[0], 0.0, v1399[1], v1399[2], v1399[3]]))) * v1375)) - (Lanes([0.0, v1409[0], v1409[1], v1409[2], v1409[3]]))) / v1390;
                let v1413 = v1034 + v1408;
                let v1417 = v1027 - (v1414 * v1408);
                let v1418 = v1029 - (v1412 * v1414);
                v1419 = v1413;
                v1420 = v1417;
                v1421 = v1408;
                v1422 = v1417;
                v1423 = v1418;
                v1424 = v1412;
                v1425 = v1418;
            } else {
                v1419 = v1034;
                v1420 = v1027;
                v1421 = v60;
                v1422 = v1027;
                v1423 = v1029;
                v1424 = v508;
                v1425 = v1029;
            }
            let v1432: f64;
            let v1433: f64;
            let v1434: f64;
            let v1435: f64;
            let v1436: f64;
            let v1437: f64;
            let v1438: f64;
            let v1439: f64;
            let v1440: Lanes<5>;
            let v1441: Lanes<5>;
            let v1442: Lanes<5>;
            let v1443: Lanes<5>;
            let v1444: Lanes<5>;
            let v1445: Lanes<5>;
            if v1426 != 0.0 {
                let v1430 = (v1427 + v158) - (v1264 * v1380);
                let v1431 = Lanes([v1135[0], 0.0, v1135[1], v1135[2], v1135[3]]);
                v1432 = v60;
                v1433 = v60;
                v1434 = v1134;
                v1435 = v60;
                v1436 = v60;
                v1437 = v60;
                v1438 = v1430;
                v1439 = v60;
                v1440 = v508;
                v1441 = v508;
                v1442 = v1431;
                v1443 = v508;
                v1444 = v508;
                v1445 = v508;
            } else {
                let v1452: f64;
                if v1136 != 0.0 {
                    let mut v1455: f64 = 0.0;
                    let mut v1456: f64 = 0.0;
                    v1455 = v89;
                    v1456 = v60;
                    loop {
                        let v1457 = if v1455 <= v1265 { 1.0 } else { 0.0 };
                        if v1457 == 0.0 {
                            break;
                        }
                        let v1460 = v1359 / (v1458 * v321);
                        let v1462 = v89 + (v1359 * v1380);
                        let v1469 = v254 * v1460;
                        let v1470 = v1469 * v627;
                        let v1471 = v1470 * v627;
                        let v1479 = (v254 * v1359) * v627;
                        let v1482 = ((v1479 * v254) * v1460) * v627;
                        let v1497 = ((((v1359 * v1359) + ((((v1462 * v1462) - ((v446 * v1460) * (v1359 * ((((v502 * (-v1330)) * v1380) + v158) + v1134)))) * v627) * v627)) + (v1479 * (v1462 + (v1469 * v1330)))) + (v1482 * v1456)).sqrt();
                        let v1500 = v89 / v1471;
                        let v1506 = (-(v1500 * ((((v1359 + (v1462 * v627)) + (v1470 * v1330)) + (v1471 * v1456)) - v1497))) / (v1500 * (v1471 - (v1482 / (v254 * v1497))));
                        let v1508 = if (v1506.abs()) < v407 { 1.0 } else { 0.0 };
                        let v1510: f64;
                        let v1511: f64;
                        if v1508 != 0.0 {
                            v1510 = v1506;
                            v1511 = v1265;
                        } else {
                            let v1509 = if v1506 > v210 { 1.0 } else { 0.0 };
                            let v1516: f64;
                            if v1509 != 0.0 {
                                v1516 = v210;
                            } else {
                                let v1515 = if v1506 < v1514 { 1.0 } else { 0.0 };
                                let v1518: f64;
                                if v1515 != 0.0 {
                                    v1518 = v1517;
                                } else {
                                    v1518 = v1506;
                                }
                                v1516 = v1518;
                            }
                            v1510 = v1516;
                            v1511 = v1455;
                        }
                        let v1512 = v1456 + v1510;
                        let v1513 = v1511 + v89;
                        v1455 = v1513;
                        v1456 = v1512;
                    }
                    v1452 = v1456;
                } else {
                    v1452 = v60;
                }
                let v1454 = if v114 < (v1419 + v1452) { 1.0 } else { 0.0 };
                let v1524: f64;
                let v1525: f64;
                let v1526: Lanes<4>;
                if v1454 != 0.0 {
                    let v1523 = if (((v1519 * v1263) / v287).sqrt()) < v813 { 1.0 } else { 0.0 };
                    let v1588: f64;
                    let v1589: Lanes<4>;
                    if v1523 != 0.0 {
                        let v1550 = v1198 + v1549;
                        let v1552 = (v254 * v1550) + v1202;
                        let v1554 = v1208 * v1552;
                        let v1556 = v1550 * v1550;
                        let v1557 = v1199 * v1550;
                        let v1558 = v1557 + v1557;
                        let v1563 = (v1552 * v1552) - (v446 * (v1556 + v1196));
                        let v1564 = (v1554 + v1554) - ((v1558 + v1214) * v446);
                        let v1566 = if v1563 >= v1565 { 1.0 } else { 0.0 };
                        let v1596: f64;
                        let v1597: Lanes<4>;
                        if v1566 != 0.0 {
                            v1596 = v1563;
                            v1597 = v1564;
                        } else {
                            v1596 = v1595;
                            v1597 = v992;
                        }
                        let v1598 = v1596.sqrt();
                        let v1604 = (v1552 - v1598) / v254;
                        let v1605 = (v1208 - (v1597 * (v184 / (v236 * v1598)))) / v254;
                        let v1606 = v1556 / v1196;
                        let v1607 = v1197 * v1606;
                        let v1611 = v1606 / v324;
                        let v1612 = v326 * v1611;
                        let v1619 = v254 / v1550;
                        let v1623 = v151 + v1619;
                        let v1625 = (v1611.ln()) / v1623;
                        let v1628 = ((((((v1558 - (Lanes([v1607[0], 0.0, 0.0, 0.0]))) / v1196) - (Lanes([v1612[0], 0.0, 0.0, 0.0]))) / v324) * (v184 / v1611)) - ((v1241 + (((v1199 * v1619) * v95) / v1550)) * v1625)) / v1623;
                        let v1629 = if v1604 < v1189 { 1.0 } else { 0.0 };
                        let v1638: f64;
                        let v1639: Lanes<4>;
                        if v1629 != 0.0 {
                            v1638 = v1604;
                            v1639 = v1605;
                        } else {
                            let v1631 = v1628 - v1605;
                            let v1632 = (v1625 - v1604) - v1247;
                            let v1635 = (v446 * v1625) * v1247;
                            let v1636 = (v1628 * v446) * v1247;
                            let v1637 = if v1635 > v60 { 1.0 } else { 0.0 };
                            let v1642: f64;
                            let v1643: Lanes<4>;
                            if v1637 != 0.0 {
                                v1642 = v1635;
                                v1643 = v1636;
                            } else {
                                let v1640 = -v1635;
                                let v1641 = v1636 * v95;
                                v1642 = v1640;
                                v1643 = v1641;
                            }
                            let v1645 = v1631 * v1632;
                            let v1649 = ((v1632 * v1632) + v1642).sqrt();
                            let v1657 = v1625 - (v502 * (v1632 + v1649));
                            let v1658 = v1628 - ((v1631 + (((v1645 + v1645) + v1643) * (v184 / (v236 * v1649)))) * v502);
                            v1638 = v1657;
                            v1639 = v1658;
                        }
                        v1588 = v1638;
                        v1589 = v1639;
                    } else {
                        let v1571 = -(v1134 - (((v1330 / v254) * v813) / v472));
                        let v1573 = (v254 * v1571) + v1202;
                        let v1575 = v1208 * v1573;
                        let v1577 = v1571 * v1571;
                        let v1578 = v1199 * v1571;
                        let v1579 = v1578 + v1578;
                        let v1584 = (v1573 * v1573) - (v446 * (v1577 + v1196));
                        let v1585 = (v1575 + v1575) - ((v1579 + v1214) * v446);
                        let v1587 = if v1584 >= v1586 { 1.0 } else { 0.0 };
                        let v1660: f64;
                        let v1661: Lanes<4>;
                        if v1587 != 0.0 {
                            v1660 = v1584;
                            v1661 = v1585;
                        } else {
                            v1660 = v1659;
                            v1661 = v992;
                        }
                        let v1662 = v1660.sqrt();
                        let v1668 = (v1573 - v1662) / v254;
                        let v1669 = (v1208 - (v1661 * (v184 / (v236 * v1662)))) / v254;
                        let v1670 = v1577 / v1196;
                        let v1671 = v1197 * v1670;
                        let v1675 = v1670 / v324;
                        let v1676 = v326 * v1675;
                        let v1683 = v254 / v1571;
                        let v1687 = v151 + v1683;
                        let v1689 = (v1675.ln()) / v1687;
                        let v1692 = ((((((v1579 - (Lanes([v1671[0], 0.0, 0.0, 0.0]))) / v1196) - (Lanes([v1676[0], 0.0, 0.0, 0.0]))) / v324) * (v184 / v1675)) - ((v1241 + (((v1199 * v1683) * v95) / v1571)) * v1689)) / v1687;
                        let v1693 = if v1668 < v1189 { 1.0 } else { 0.0 };
                        let v1702: f64;
                        let v1703: Lanes<4>;
                        if v1693 != 0.0 {
                            v1702 = v1668;
                            v1703 = v1669;
                        } else {
                            let v1695 = v1692 - v1669;
                            let v1696 = (v1689 - v1668) - v1247;
                            let v1699 = (v446 * v1689) * v1247;
                            let v1700 = (v1692 * v446) * v1247;
                            let v1701 = if v1699 > v60 { 1.0 } else { 0.0 };
                            let v1706: f64;
                            let v1707: Lanes<4>;
                            if v1701 != 0.0 {
                                v1706 = v1699;
                                v1707 = v1700;
                            } else {
                                let v1704 = -v1699;
                                let v1705 = v1700 * v95;
                                v1706 = v1704;
                                v1707 = v1705;
                            }
                            let v1709 = v1695 * v1696;
                            let v1713 = ((v1696 * v1696) + v1706).sqrt();
                            let v1721 = v1689 - (v502 * (v1696 + v1713));
                            let v1722 = v1692 - ((v1695 + (((v1709 + v1709) + v1707) * (v184 / (v236 * v1713)))) * v502);
                            v1702 = v1721;
                            v1703 = v1722;
                        }
                        v1588 = v1702;
                        v1589 = v1703;
                    }
                    let v1594 = if (((v1590 * v1263) / v287).sqrt()) < v813 { 1.0 } else { 0.0 };
                    let v1723: f64;
                    let v1724: Lanes<4>;
                    if v1594 != 0.0 {
                        let mut v1726: f64 = 0.0;
                        let mut v1727: f64 = 0.0;
                        let mut v1728: f64 = 0.0;
                        let mut v1729: Lanes<4> = Lanes([0.0; 4]);
                        let mut v1730: Lanes<4> = Lanes([0.0; 4]);
                        v1726 = v60;
                        v1727 = v1588;
                        v1728 = v60;
                        v1729 = v1589;
                        v1730 = v992;
                        loop {
                            let v1731 = if v1726 < v1265 { 1.0 } else { 0.0 };
                            if v1731 == 0.0 {
                                break;
                            }
                            let v1732 = v151 * v1727;
                            let v1733 = v154 * v1727;
                            let v1736 = (Lanes([v1733[0], 0.0, 0.0, 0.0])) + (v1729 * v151);
                            let v1739 = (-v1732).exp();
                            let v1740 = (v1736 * v95) * v1739;
                            let v1741 = if v1727 > v1270 { 1.0 } else { 0.0 };
                            let v1785: f64;
                            let v1786: f64;
                            let v1787: Lanes<4>;
                            let v1788: Lanes<4>;
                            if v1741 != 0.0 {
                                let v1742 = v1732.exp();
                                let v1744 = -v312;
                                let v1749 = v1742 - v89;
                                let v1751 = v326 * v1749;
                                let v1752 = (v1736 * v1742) * v324;
                                let v1757 = (((v1739 + v1732) - v89) + (v324 * v1749)).sqrt();
                                let v1761 = v1744 * v1757;
                                let v1762 = (v315 * v95) * v1757;
                                let v1765 = (Lanes([v1762[0], 0.0, 0.0, 0.0])) + ((((v1740 + v1736) + ((Lanes([v1751[0], 0.0, 0.0, 0.0])) + v1752)) * (v184 / (v236 * v1757))) * v1744);
                                let v1766 = v1287 / v1761;
                                let v1774 = v326 * v1742;
                                let v1777 = ((-v1739) + v89) + (v324 * v1742);
                                let v1779 = v1766 * v1777;
                                let v1782 = ((((v1765 * v1766) * v95) / v1761) * v1777) + (((v1740 * v95) + ((Lanes([v1774[0], 0.0, 0.0, 0.0])) + v1752)) * v1766);
                                v1785 = v1761;
                                v1786 = v1779;
                                v1787 = v1765;
                                v1788 = v1782;
                            } else {
                                let v1784 = if v1727 < v1783 { 1.0 } else { 0.0 };
                                let v1863: f64;
                                let v1864: f64;
                                let v1865: Lanes<4>;
                                let v1866: Lanes<4>;
                                if v1784 != 0.0 {
                                    let v1815 = ((v1739 + v1732) - v89).sqrt();
                                    let v1819 = v312 * v1815;
                                    let v1820 = v315 * v1815;
                                    let v1823 = (Lanes([v1820[0], 0.0, 0.0, 0.0])) + (((v1740 + v1736) * (v184 / (v236 * v1815))) * v312);
                                    let v1824 = v1287 / v1819;
                                    let v1830 = (-v1739) + v89;
                                    let v1831 = v1824 * v1830;
                                    let v1834 = ((((v1823 * v1824) * v95) / v1819) * v1830) + ((v1740 * v95) * v1824);
                                    v1863 = v1819;
                                    v1864 = v1831;
                                    v1865 = v1823;
                                    v1866 = v1834;
                                } else {
                                    let v1835 = v1287 / v151;
                                    let v1839 = v1835.sqrt();
                                    let v1843 = -v1839;
                                    let v1845 = v1843 * v151;
                                    let v1849 = v1845 * v1727;
                                    let v1850 = (((((((v154 * v1835) * v95) / v151) * (v184 / (v236 * v1839))) * v95) * v151) + (v154 * v1843)) * v1727;
                                    let v1853 = (Lanes([v1850[0], 0.0, 0.0, 0.0])) + (v1729 * v1845);
                                    let v1856 = (v1287 * v151).sqrt();
                                    let v1860 = -v1856;
                                    let v1861 = ((v154 * v1287) * (v184 / (v236 * v1856))) * v95;
                                    let v1862 = Lanes([v1861[0], 0.0, 0.0, 0.0]);
                                    v1863 = v1849;
                                    v1864 = v1860;
                                    v1865 = v1853;
                                    v1866 = v1862;
                                }
                                v1785 = v1863;
                                v1786 = v1864;
                                v1787 = v1865;
                                v1788 = v1866;
                            }
                            let v1790 = v1787 * v1785;
                            let v1794 = ((v1785 * v1785) + v1792).sqrt();
                            let v1797 = (v1790 + v1790) * (v184 / (v236 * v1794));
                            let v1798 = v1785 / v1794;
                            let v1803 = v502 * (v89 + v1798);
                            let v1804 = ((v1787 - (v1797 * v1798)) / v1794) * v502;
                            let v1808 = (v1787 + v1797) * v502;
                            let v1810 = (v502 * (v1785 + v1794)) + v1809;
                            let v1811 = if v1810 < v60 { 1.0 } else { 0.0 };
                            let v1867: f64;
                            let v1868: f64;
                            let v1869: Lanes<4>;
                            let v1870: Lanes<4>;
                            if v1811 != 0.0 {
                                v1867 = v60;
                                v1868 = v60;
                                v1869 = v992;
                                v1870 = v992;
                            } else {
                                v1867 = v1810;
                                v1868 = v1803;
                                v1869 = v1808;
                                v1870 = v1804;
                            }
                            let v1871 = -v1330;
                            let v1873 = v1869 * v95;
                            let v1875 = (v1871 - v1867) - v1874;
                            let v1877 = (v446 * v1871) * v1874;
                            let v1878 = if v1877 > v60 { 1.0 } else { 0.0 };
                            let v1880: f64;
                            if v1878 != 0.0 {
                                v1880 = v1877;
                            } else {
                                let v1879 = -v1877;
                                v1880 = v1879;
                            }
                            let v1882 = v1873 * v1875;
                            let v1885 = ((v1875 * v1875) + v1880).sqrt();
                            let v1888 = (v1882 + v1882) * (v184 / (v236 * v1885));
                            let v1889 = v1875 / v1885;
                            let v1894 = v502 * (v89 + v1889);
                            let v1900 = v1871 - (v502 * (v1875 + v1885));
                            let v1901 = ((v1873 + v1888) * v502) * v95;
                            let v1902 = v1786 * v1894;
                            let v1906 = v1868 * v1902;
                            let v1911 = v1901 * v1900;
                            let v1919 = ((((v1900 * v1900) / v254) / v472) / v150) / v287;
                            let v1920 = ((((v1911 + v1911) / v254) / v472) / v150) / v287;
                            let v1921 = v254 * v1919;
                            let v1927 = (v1921 * v1906) / v1900;
                            let v1945 = (v1943 + (v1786 / v1359)) + v1927;
                            let v1947 = ((((-v1727) + (v1785 / v1359)) - v1134) + v1919) / v1945;
                            let v1951 = v1727 - v1947;
                            let v1952 = v1729 - ((((((v1729 * v95) + (v1787 / v1359)) - v1135) + v1920) - (((v1788 / v1359) + (((((v1920 * v254) * v1906) + (((v1870 * v1902) + (((v1788 * v1894) + ((((v1873 - (v1888 * v1889)) / v1885) * v502) * v1786)) * v1868)) * v1921)) - (v1901 * v1927)) / v1900)) * v1947)) / v1945);
                            let v1955 = if ((v1951 - v1727).abs()) < v633 { 1.0 } else { 0.0 };
                            let v1956: f64;
                            if v1955 != 0.0 {
                                v1956 = v1265;
                            } else {
                                v1956 = v1726;
                            }
                            let v1957 = v1956 + v89;
                            v1726 = v1957;
                            v1727 = v1951;
                            v1728 = v1785;
                            v1729 = v1952;
                            v1730 = v1787;
                        }
                        v1723 = v1728;
                        v1724 = v1730;
                    } else {
                        let mut v1958: f64 = 0.0;
                        let mut v1959: f64 = 0.0;
                        let mut v1960: f64 = 0.0;
                        let mut v1961: Lanes<4> = Lanes([0.0; 4]);
                        let mut v1962: Lanes<4> = Lanes([0.0; 4]);
                        v1958 = v60;
                        v1959 = v1588;
                        v1960 = v60;
                        v1961 = v1589;
                        v1962 = v992;
                        loop {
                            let v1963 = if v1958 < v1265 { 1.0 } else { 0.0 };
                            if v1963 == 0.0 {
                                break;
                            }
                            let v1964 = v151 * v1959;
                            let v1965 = v154 * v1959;
                            let v1968 = (Lanes([v1965[0], 0.0, 0.0, 0.0])) + (v1961 * v151);
                            let v1971 = (-v1964).exp();
                            let v1972 = (v1968 * v95) * v1971;
                            let v1973 = if v1959 > v1270 { 1.0 } else { 0.0 };
                            let v2017: f64;
                            let v2018: f64;
                            let v2019: Lanes<4>;
                            let v2020: Lanes<4>;
                            if v1973 != 0.0 {
                                let v1974 = v1964.exp();
                                let v1976 = -v312;
                                let v1981 = v1974 - v89;
                                let v1983 = v326 * v1981;
                                let v1984 = (v1968 * v1974) * v324;
                                let v1989 = (((v1971 + v1964) - v89) + (v324 * v1981)).sqrt();
                                let v1993 = v1976 * v1989;
                                let v1994 = (v315 * v95) * v1989;
                                let v1997 = (Lanes([v1994[0], 0.0, 0.0, 0.0])) + ((((v1972 + v1968) + ((Lanes([v1983[0], 0.0, 0.0, 0.0])) + v1984)) * (v184 / (v236 * v1989))) * v1976);
                                let v1998 = v1287 / v1993;
                                let v2006 = v326 * v1974;
                                let v2009 = ((-v1971) + v89) + (v324 * v1974);
                                let v2011 = v1998 * v2009;
                                let v2014 = ((((v1997 * v1998) * v95) / v1993) * v2009) + (((v1972 * v95) + ((Lanes([v2006[0], 0.0, 0.0, 0.0])) + v1984)) * v1998);
                                v2017 = v1993;
                                v2018 = v2011;
                                v2019 = v1997;
                                v2020 = v2014;
                            } else {
                                let v2016 = if v1959 < v2015 { 1.0 } else { 0.0 };
                                let v2095: f64;
                                let v2096: f64;
                                let v2097: Lanes<4>;
                                let v2098: Lanes<4>;
                                if v2016 != 0.0 {
                                    let v2047 = ((v1971 + v1964) - v89).sqrt();
                                    let v2051 = v312 * v2047;
                                    let v2052 = v315 * v2047;
                                    let v2055 = (Lanes([v2052[0], 0.0, 0.0, 0.0])) + (((v1972 + v1968) * (v184 / (v236 * v2047))) * v312);
                                    let v2056 = v1287 / v2051;
                                    let v2062 = (-v1971) + v89;
                                    let v2063 = v2056 * v2062;
                                    let v2066 = ((((v2055 * v2056) * v95) / v2051) * v2062) + ((v1972 * v95) * v2056);
                                    v2095 = v2051;
                                    v2096 = v2063;
                                    v2097 = v2055;
                                    v2098 = v2066;
                                } else {
                                    let v2067 = v1287 / v151;
                                    let v2071 = v2067.sqrt();
                                    let v2075 = -v2071;
                                    let v2077 = v2075 * v151;
                                    let v2081 = v2077 * v1959;
                                    let v2082 = (((((((v154 * v2067) * v95) / v151) * (v184 / (v236 * v2071))) * v95) * v151) + (v154 * v2075)) * v1959;
                                    let v2085 = (Lanes([v2082[0], 0.0, 0.0, 0.0])) + (v1961 * v2077);
                                    let v2088 = (v1287 * v151).sqrt();
                                    let v2092 = -v2088;
                                    let v2093 = ((v154 * v1287) * (v184 / (v236 * v2088))) * v95;
                                    let v2094 = Lanes([v2093[0], 0.0, 0.0, 0.0]);
                                    v2095 = v2081;
                                    v2096 = v2092;
                                    v2097 = v2085;
                                    v2098 = v2094;
                                }
                                v2017 = v2095;
                                v2018 = v2096;
                                v2019 = v2097;
                                v2020 = v2098;
                            }
                            let v2022 = v2019 * v2017;
                            let v2026 = ((v2017 * v2017) + v2024).sqrt();
                            let v2029 = (v2022 + v2022) * (v184 / (v236 * v2026));
                            let v2030 = v2017 / v2026;
                            let v2035 = v502 * (v89 + v2030);
                            let v2036 = ((v2019 - (v2029 * v2030)) / v2026) * v502;
                            let v2040 = (v2019 + v2029) * v502;
                            let v2042 = (v502 * (v2017 + v2026)) + v2041;
                            let v2043 = if v2042 < v60 { 1.0 } else { 0.0 };
                            let v2099: f64;
                            let v2100: f64;
                            let v2101: Lanes<4>;
                            let v2102: Lanes<4>;
                            if v2043 != 0.0 {
                                v2099 = v60;
                                v2100 = v60;
                                v2101 = v992;
                                v2102 = v992;
                            } else {
                                v2099 = v2042;
                                v2100 = v2035;
                                v2101 = v2040;
                                v2102 = v2036;
                            }
                            let v2103 = -v1330;
                            let v2105 = v2101 * v95;
                            let v2106 = (v2103 - v2099) - v1874;
                            let v2108 = (v446 * v2103) * v1874;
                            let v2109 = if v2108 > v60 { 1.0 } else { 0.0 };
                            let v2111: f64;
                            if v2109 != 0.0 {
                                v2111 = v2108;
                            } else {
                                let v2110 = -v2108;
                                v2111 = v2110;
                            }
                            let v2113 = v2105 * v2106;
                            let v2116 = ((v2106 * v2106) + v2111).sqrt();
                            let v2119 = (v2113 + v2113) * (v184 / (v236 * v2116));
                            let v2120 = v2106 / v2116;
                            let v2125 = v502 * (v89 + v2120);
                            let v2131 = v2103 - (v502 * (v2106 + v2116));
                            let v2132 = ((v2105 + v2119) * v502) * v95;
                            let v2133 = v2018 * v2125;
                            let v2137 = v2100 * v2133;
                            let v2142 = v2132 * v2131;
                            let v2150 = ((((v2131 * v2131) / v254) / v472) / v150) / v287;
                            let v2151 = ((((v2142 + v2142) / v254) / v472) / v150) / v287;
                            let v2152 = v254 * v2150;
                            let v2158 = (v2152 * v2137) / v2131;
                            let v2190 = ((v2182 + (v2018 / v1359)) + ((v2018 * v813) / v472)) + v2158;
                            let v2192 = (((((v60 - v1959) + (v2017 / v1359)) + (((v2017 + (v1330 / v254)) * v813) / v472)) - v1134) + v2150) / v2190;
                            let v2196 = v1959 - v2192;
                            let v2197 = v1961 - (((((((v1961 * v95) + (v2019 / v1359)) + ((v2019 * v813) / v472)) - v1135) + v2151) - ((((v2020 / v1359) + ((v2020 * v813) / v472)) + (((((v2151 * v254) * v2137) + (((v2102 * v2133) + (((v2020 * v2125) + ((((v2105 - (v2119 * v2120)) / v2116) * v502) * v2018)) * v2100)) * v2152)) - (v2132 * v2158)) / v2131)) * v2192)) / v2190);
                            let v2200 = if ((v2196 - v1959).abs()) < v633 { 1.0 } else { 0.0 };
                            let v2201: f64;
                            if v2200 != 0.0 {
                                v2201 = v1265;
                            } else {
                                v2201 = v1958;
                            }
                            let v2202 = v2201 + v89;
                            v1958 = v2202;
                            v1959 = v2196;
                            v1960 = v2017;
                            v1961 = v2197;
                            v1962 = v2019;
                        }
                        v1723 = v1960;
                        v1724 = v1962;
                    }
                    v1524 = v1723;
                    v1525 = v1725;
                    v1526 = v1724;
                } else {
                    v1524 = v60;
                    v1525 = v60;
                    v1526 = v992;
                }
                let v1527 = v1420 - v353;
                let v1530 = v154 * v1527;
                let v1537 = v1041 * v155;
                let v1539 = v157 * v1041;
                let v1542 = (v446 * ((v151 * v1527) - v89)) / v1537;
                let v1545 = ((((Lanes([v1530[0], 0.0, 0.0, 0.0, 0.0])) + ((v1423 - v486) * v151)) * v446) - (((v1043 * v155) + (Lanes([v1539[0], 0.0, 0.0, 0.0, 0.0]))) * v1542)) / v1537;
                let v1546 = v89 + v1542;
                let v1548 = if v1546 >= v1547 { 1.0 } else { 0.0 };
                let v2204: f64;
                let v2205: Lanes<5>;
                if v1548 != 0.0 {
                    v2204 = v1546;
                    v2205 = v1545;
                } else {
                    v2204 = v2203;
                    v2205 = v508;
                }
                let v2208 = v154 * v1041;
                let v2211 = (v1041 * v151) * v502;
                let v2213 = v2204.sqrt();
                let v2217 = v89 - v2213;
                let v2223 = v1420 + (v2211 * v2217);
                let v2224 = v1423 + (((((v1043 * v151) + (Lanes([v2208[0], 0.0, 0.0, 0.0, 0.0]))) * v502) * v2217) + (((v2205 * (v184 / (v236 * v2213))) * v95) * v2211));
                let v2225 = v89 / v627;
                let v2232 = (v2225 + v2229) + v2231;
                let v2233 = v89 / v2232;
                let v2236 = (((((v630 * v2225) * v95) / v627) * v2233) * v95) / v2232;
                let v2237 = v114 - v1421;
                let v2238 = if v2237 <= v1017 { 1.0 } else { 0.0 };
                let v2253: f64;
                let v2254: Lanes<5>;
                if v2238 != 0.0 {
                    let v2239 = if v2223 > v60 { 1.0 } else { 0.0 };
                    let v2272: f64;
                    let v2273: Lanes<5>;
                    if v2239 != 0.0 {
                        let v2265 = ((v150 * v287) * v254) * v472;
                        let v2268 = (v2265 * v2223).sqrt();
                        let v2271 = (v2224 * v2265) * (v184 / (v236 * v2268));
                        v2272 = v2268;
                        v2273 = v2271;
                    } else {
                        v2272 = v60;
                        v2273 = v508;
                    }
                    let v2274 = if v1330 <= v2272 { 1.0 } else { 0.0 };
                    let v2275: f64;
                    let v2276: Lanes<5>;
                    if v2274 != 0.0 {
                        v2275 = v1330;
                        v2276 = v508;
                    } else {
                        v2275 = v2272;
                        v2276 = v2273;
                    }
                    let v2281 = v2231 + (v502 * v2229);
                    let v2286 = (v1420 - v1134) + (v2281 * (-v2275));
                    let v2288 = v2233 * v2286;
                    let v2289 = v2236 * v2286;
                    let v2292 = (Lanes([0.0, v2289[0], v2289[1], v2289[2], v2289[3]])) + (((v1423 - (Lanes([v1135[0], 0.0, v1135[1], v1135[2], v1135[3]]))) + ((v2276 * v95) * v2281)) * v2233);
                    v2253 = v2288;
                    v2254 = v2292;
                } else {
                    let v2247 = (v1420 - v1134) + ((v2231 + (v502 * v2229)) * (-v1330));
                    let v2248 = v2233 * v2247;
                    let v2249 = v2236 * v2247;
                    let v2252 = (Lanes([0.0, v2249[0], v2249[1], v2249[2], v2249[3]])) + ((v1423 - (Lanes([v1135[0], 0.0, v1135[1], v1135[2], v1135[3]]))) * v2233);
                    v2253 = v2248;
                    v2254 = v2252;
                }
                let v2255 = v2253 / v627;
                let v2256 = v630 * v2255;
                let v2260 = v1420 - v2255;
                let v2261 = v1423 - ((v2254 - (Lanes([0.0, v2256[0], v2256[1], v2256[2], v2256[3]]))) / v627);
                let v2262 = if v2237 > v1017 { 1.0 } else { 0.0 };
                let v2331: f64;
                let v2332: Lanes<5>;
                if v2262 != 0.0 {
                    let v2293 = v89 / v318;
                    let v2296 = ((v320 * v2293) * v95) / v318;
                    let v2297 = v2293 / v626;
                    let v2302 = v1420 - v1421;
                    let v2303 = v1423 - v1424;
                    let v2304 = v2297 * v2302;
                    let v2308 = v2304 * v2302;
                    let v2312 = v254 / v2302;
                    let v2316 = v151 + v2312;
                    let v2322 = (v2308.ln()) / v2316;
                    let v2325 = (((((((((Lanes([v2296[0], 0.0, 0.0, 0.0, 0.0])) - (v629 * v2297)) / v626) * v2302) + (v2303 * v2297)) * v2302) + (v2303 * v2304)) * (v184 / v2308)) - (((Lanes([v154[0], 0.0, 0.0, 0.0, 0.0])) + (((v2303 * v2312) * v95) / v2302)) * v2322)) / v2316;
                    let v2327 = v2322 - v2326;
                    let v2330 = if (if v2260 > v2327 { 1.0 } else { 0.0 }) != 0.0 && v2329 != 0.0 { 1.0 } else { 0.0 };
                    let v2343: f64;
                    let v2344: Lanes<5>;
                    if v2330 != 0.0 {
                        let v2335 = v2261 - v2325;
                        let v2336 = (v2260 - v2322) + v2326;
                        let v2338 = v2335 * v2336;
                        let v2339 = v2338 + v2338;
                        let v2341 = (v2336 * v2336) + v2340;
                        let v2352: f64;
                        let v2353: Lanes<5>;
                        if v2342 != 0.0 {
                            let v2368: f64;
                            if v2345 != 0.0 {
                                v2368 = v89;
                            } else {
                                let v2370: f64;
                                if v2367 != 0.0 {
                                    v2370 = v254;
                                } else {
                                    let v2372: f64;
                                    if v2369 != 0.0 {
                                        v2372 = v443;
                                    } else {
                                        let v2373: f64;
                                        if v2371 != 0.0 {
                                            v2373 = v446;
                                        } else {
                                            v2373 = v60;
                                        }
                                        v2372 = v2373;
                                    }
                                    v2370 = v2372;
                                }
                                v2368 = v2370;
                            }
                            let mut v2374: f64 = 0.0;
                            let mut v2375: f64 = 0.0;
                            let mut v2376: Lanes<5> = Lanes([0.0; 5]);
                            v2374 = v60;
                            v2375 = v2341;
                            v2376 = v2339;
                            loop {
                                let v2377 = if v2374 < v2368 { 1.0 } else { 0.0 };
                                if v2377 == 0.0 {
                                    break;
                                }
                                let v2378 = v2375.sqrt();
                                let v2381 = v2376 * (v184 / (v236 * v2378));
                                let v2382 = v2374 + v89;
                                v2374 = v2382;
                                v2375 = v2378;
                                v2376 = v2381;
                            }
                            v2352 = v2375;
                            v2353 = v2376;
                        } else {
                            let v2346 = v2341.sqrt();
                            let v2351 = v2339 * (v2349 * (v2341.powf(v2347)));
                            v2352 = v2346;
                            v2353 = v2351;
                        }
                        let v2354 = v2352 + v418;
                        let v2355 = v89 / v2354;
                        let v2359 = v2336 * v2326;
                        let v2365 = v2327 + (v2359 * v2355);
                        let v2366 = v2325 + (((v2335 * v2326) * v2355) + ((((v2353 * v2355) * v95) / v2354) * v2359));
                        v2343 = v2365;
                        v2344 = v2366;
                    } else {
                        v2343 = v2260;
                        v2344 = v2261;
                    }
                    v2331 = v2343;
                    v2332 = v2344;
                } else {
                    v2331 = v2260;
                    v2332 = v2261;
                }
                let v2333 = if v2331 > v60 { 1.0 } else { 0.0 };
                let v2387: f64;
                if v2333 != 0.0 {
                    let v2386 = ((v2383 * v2331) / v287).sqrt();
                    v2387 = v2386;
                } else {
                    v2387 = v60;
                }
                let v2388 = if v2387 < v813 { 1.0 } else { 0.0 };
                let v2389: f64;
                if v2388 != 0.0 {
                    v2389 = v89;
                } else {
                    v2389 = v254;
                }
                let v2390 = if v2389 == v89 { 1.0 } else { 0.0 };
                let v2425: f64;
                let v2426: Lanes<5>;
                if v2390 != 0.0 {
                    let v2391 = v1208 * v1206;
                    let v2393 = (v2391 + v2391) - v1217;
                    let v2395 = if v1218 >= v2394 { 1.0 } else { 0.0 };
                    let v2429: f64;
                    let v2430: Lanes<4>;
                    if v2395 != 0.0 {
                        v2429 = v1218;
                        v2430 = v2393;
                    } else {
                        v2429 = v2428;
                        v2430 = v992;
                    }
                    let v2431 = v2429.sqrt();
                    let v2437 = (v1206 - v2431) / v254;
                    let v2438 = (v1208 - (v2430 * (v184 / (v236 * v2431)))) / v254;
                    let v2443 = ((((v1230 - v1233) / v324) * v1235) - v1244) / v1240;
                    let v2444 = if v2437 < v1189 { 1.0 } else { 0.0 };
                    let v2453: f64;
                    let v2454: Lanes<4>;
                    if v2444 != 0.0 {
                        v2453 = v2437;
                        v2454 = v2438;
                    } else {
                        let v2446 = v2443 - v2438;
                        let v2447 = (v1243 - v2437) - v1247;
                        let v2450 = (v446 * v1243) * v1247;
                        let v2451 = (v2443 * v446) * v1247;
                        let v2452 = if v2450 > v60 { 1.0 } else { 0.0 };
                        let v2458: f64;
                        let v2459: Lanes<4>;
                        if v2452 != 0.0 {
                            v2458 = v2450;
                            v2459 = v2451;
                        } else {
                            let v2456 = -v2450;
                            let v2457 = v2451 * v95;
                            v2458 = v2456;
                            v2459 = v2457;
                        }
                        let v2461 = v2446 * v2447;
                        let v2465 = ((v2447 * v2447) + v2458).sqrt();
                        let v2473 = v1243 - (v502 * (v2447 + v2465));
                        let v2474 = v2443 - ((v2446 + (((v2461 + v2461) + v2459) * (v184 / (v236 * v2465)))) * v502);
                        v2453 = v2473;
                        v2454 = v2474;
                    }
                    let v2455 = Lanes([v2454[0], 0.0, v2454[1], v2454[2], v2454[3]]);
                    v2425 = v2453;
                    v2426 = v2455;
                } else {
                    let v2403 = -((v1134 - v2331) - (((v1330 / v254) * v813) / v472));
                    let v2404 = ((Lanes([v1135[0], 0.0, v1135[1], v1135[2], v1135[3]])) - v2332) * v95;
                    let v2407 = (v254 * v2403) + v1202;
                    let v2409 = (v2404 * v254) + (Lanes([v1205[0], 0.0, 0.0, 0.0, 0.0]));
                    let v2411 = v2409 * v2407;
                    let v2413 = v2403 * v2403;
                    let v2414 = v2404 * v2403;
                    let v2415 = v2414 + v2414;
                    let v2421 = (v2407 * v2407) - (v446 * (v2413 + v1196));
                    let v2422 = (v2411 + v2411) - ((v2415 + (Lanes([v1197[0], 0.0, 0.0, 0.0, 0.0]))) * v446);
                    let v2424 = if v2421 >= v2423 { 1.0 } else { 0.0 };
                    let v2476: f64;
                    let v2477: Lanes<5>;
                    if v2424 != 0.0 {
                        v2476 = v2421;
                        v2477 = v2422;
                    } else {
                        v2476 = v2475;
                        v2477 = v508;
                    }
                    let v2478 = v2476.sqrt();
                    let v2484 = (v2407 - v2478) / v254;
                    let v2485 = (v2409 - (v2477 * (v184 / (v236 * v2478)))) / v254;
                    let v2486 = v2413 / v1196;
                    let v2487 = v1197 * v2486;
                    let v2491 = v2486 / v324;
                    let v2492 = v326 * v2491;
                    let v2499 = v254 / v2403;
                    let v2503 = v151 + v2499;
                    let v2506 = (v2491.ln()) / v2503;
                    let v2509 = ((((((v2415 - (Lanes([v2487[0], 0.0, 0.0, 0.0, 0.0]))) / v1196) - (Lanes([v2492[0], 0.0, 0.0, 0.0, 0.0]))) / v324) * (v184 / v2491)) - (((Lanes([v154[0], 0.0, 0.0, 0.0, 0.0])) + (((v2404 * v2499) * v95) / v2403)) * v2506)) / v2503;
                    let v2510 = if v2484 < v1189 { 1.0 } else { 0.0 };
                    let v2519: f64;
                    let v2520: Lanes<5>;
                    if v2510 != 0.0 {
                        v2519 = v2484;
                        v2520 = v2485;
                    } else {
                        let v2512 = v2509 - v2485;
                        let v2513 = (v2506 - v2484) - v1247;
                        let v2516 = (v446 * v2506) * v1247;
                        let v2517 = (v2509 * v446) * v1247;
                        let v2518 = if v2516 > v60 { 1.0 } else { 0.0 };
                        let v2523: f64;
                        let v2524: Lanes<5>;
                        if v2518 != 0.0 {
                            v2523 = v2516;
                            v2524 = v2517;
                        } else {
                            let v2521 = -v2516;
                            let v2522 = v2517 * v95;
                            v2523 = v2521;
                            v2524 = v2522;
                        }
                        let v2526 = v2512 * v2513;
                        let v2530 = ((v2513 * v2513) + v2523).sqrt();
                        let v2538 = v2506 - (v502 * (v2513 + v2530));
                        let v2539 = v2509 - ((v2512 + (((v2526 + v2526) + v2524) * (v184 / (v236 * v2530)))) * v502);
                        v2519 = v2538;
                        v2520 = v2539;
                    }
                    v2425 = v2519;
                    v2426 = v2520;
                }
                let v2427 = if v2390 != 0.0 && v60 != 0.0 { 1.0 } else { 0.0 };
                let v2542: f64;
                let v2543: f64;
                let v2544: f64;
                let v2545: Lanes<5>;
                let v2546: Lanes<5>;
                let v2547: Lanes<5>;
                if v2427 != 0.0 {
                    let v2540 = Lanes([v1526[0], 0.0, v1526[1], v1526[2], v1526[3]]);
                    let mut v2560: f64 = 0.0;
                    let mut v2561: f64 = 0.0;
                    let mut v2562: f64 = 0.0;
                    let mut v2563: Lanes<5> = Lanes([0.0; 5]);
                    let mut v2564: Lanes<5> = Lanes([0.0; 5]);
                    v2560 = v60;
                    v2561 = v2425;
                    v2562 = v1524;
                    v2563 = v2426;
                    v2564 = v2540;
                    loop {
                        let v2565 = if v2560 < v1265 { 1.0 } else { 0.0 };
                        if v2565 == 0.0 {
                            break;
                        }
                        let v2566 = v151 * v2561;
                        let v2567 = v154 * v2561;
                        let v2570 = (Lanes([v2567[0], 0.0, 0.0, 0.0, 0.0])) + (v2563 * v151);
                        let v2573 = (-v2566).exp();
                        let v2574 = (v2570 * v95) * v2573;
                        let v2575 = if v2561 > v1270 { 1.0 } else { 0.0 };
                        let v2622: f64;
                        let v2623: f64;
                        let v2624: Lanes<5>;
                        let v2625: Lanes<5>;
                        if v2575 != 0.0 {
                            let v2579 = v2566.exp();
                            let v2581 = -v312;
                            let v2586 = v2579 - v89;
                            let v2588 = v326 * v2586;
                            let v2589 = (v2570 * v2579) * v324;
                            let v2594 = (((v2573 + v2566) - v89) + (v324 * v2586)).sqrt();
                            let v2598 = v2581 * v2594;
                            let v2599 = (v315 * v95) * v2594;
                            let v2602 = (Lanes([v2599[0], 0.0, 0.0, 0.0, 0.0])) + ((((v2574 + v2570) + ((Lanes([v2588[0], 0.0, 0.0, 0.0, 0.0])) + v2589)) * (v184 / (v236 * v2594))) * v2581);
                            let v2603 = v1287 / v2598;
                            let v2611 = v326 * v2579;
                            let v2614 = ((-v2573) + v89) + (v324 * v2579);
                            let v2616 = v2603 * v2614;
                            let v2619 = ((((v2602 * v2603) * v95) / v2598) * v2614) + (((v2574 * v95) + ((Lanes([v2611[0], 0.0, 0.0, 0.0, 0.0])) + v2589)) * v2603);
                            v2622 = v2598;
                            v2623 = v2616;
                            v2624 = v2602;
                            v2625 = v2619;
                        } else {
                            let v2621 = if v2561 < v2620 { 1.0 } else { 0.0 };
                            let v2699: f64;
                            let v2700: f64;
                            let v2701: Lanes<5>;
                            let v2702: Lanes<5>;
                            if v2621 != 0.0 {
                                let v2651 = ((v2573 + v2566) - v89).sqrt();
                                let v2655 = v312 * v2651;
                                let v2656 = v315 * v2651;
                                let v2659 = (Lanes([v2656[0], 0.0, 0.0, 0.0, 0.0])) + (((v2574 + v2570) * (v184 / (v236 * v2651))) * v312);
                                let v2660 = v1287 / v2655;
                                let v2666 = (-v2573) + v89;
                                let v2667 = v2660 * v2666;
                                let v2670 = ((((v2659 * v2660) * v95) / v2655) * v2666) + ((v2574 * v95) * v2660);
                                v2699 = v2655;
                                v2700 = v2667;
                                v2701 = v2659;
                                v2702 = v2670;
                            } else {
                                let v2671 = v1287 / v151;
                                let v2675 = v2671.sqrt();
                                let v2679 = -v2675;
                                let v2681 = v2679 * v151;
                                let v2685 = v2681 * v2561;
                                let v2686 = (((((((v154 * v2671) * v95) / v151) * (v184 / (v236 * v2675))) * v95) * v151) + (v154 * v2679)) * v2561;
                                let v2689 = (Lanes([v2686[0], 0.0, 0.0, 0.0, 0.0])) + (v2563 * v2681);
                                let v2692 = (v1287 * v151).sqrt();
                                let v2696 = -v2692;
                                let v2697 = ((v154 * v1287) * (v184 / (v236 * v2692))) * v95;
                                let v2698 = Lanes([v2697[0], 0.0, 0.0, 0.0, 0.0]);
                                v2699 = v2685;
                                v2700 = v2696;
                                v2701 = v2689;
                                v2702 = v2698;
                            }
                            v2622 = v2699;
                            v2623 = v2700;
                            v2624 = v2701;
                            v2625 = v2702;
                        }
                        let v2638 = v2637 + (v2623 / v1359);
                        let v2639 = (((-v2561) + (v2622 / v1359)) - v1134) / v2638;
                        let v2643 = v2561 - v2639;
                        let v2644 = v2563 - (((((v2563 * v95) + (v2624 / v1359)) - (Lanes([v1135[0], 0.0, v1135[1], v1135[2], v1135[3]]))) - ((v2625 / v1359) * v2639)) / v2638);
                        let v2647 = if ((v2643 - v2561).abs()) < v633 { 1.0 } else { 0.0 };
                        let v2703: f64;
                        if v2647 != 0.0 {
                            v2703 = v1265;
                        } else {
                            v2703 = v2560;
                        }
                        let v2704 = v2703 + v89;
                        v2560 = v2704;
                        v2561 = v2643;
                        v2562 = v2622;
                        v2563 = v2644;
                        v2564 = v2624;
                    }
                    let v2576 = v1134 + v2561;
                    let v2578 = (Lanes([v1135[0], 0.0, v1135[1], v1135[2], v1135[3]])) + v2563;
                    v2542 = v2576;
                    v2543 = v2562;
                    v2544 = v60;
                    v2545 = v2578;
                    v2546 = v2564;
                    v2547 = v508;
                } else {
                    let v2706: f64;
                    let v2707: f64;
                    let v2708: Lanes<5>;
                    if v2541 != 0.0 {
                        v2706 = v2260;
                        v2707 = v2705;
                        v2708 = v2261;
                    } else {
                        v2706 = v2331;
                        v2707 = v633;
                        v2708 = v2332;
                    }
                    let v2709 = Lanes([v1526[0], 0.0, v1526[1], v1526[2], v1526[3]]);
                    let mut v2710: f64 = 0.0;
                    let mut v2711: f64 = 0.0;
                    let mut v2712: f64 = 0.0;
                    let mut v2713: Lanes<5> = Lanes([0.0; 5]);
                    let mut v2714: Lanes<5> = Lanes([0.0; 5]);
                    v2710 = v60;
                    v2711 = v2425;
                    v2712 = v1524;
                    v2713 = v2426;
                    v2714 = v2709;
                    loop {
                        let v2715 = if v2710 < v1265 { 1.0 } else { 0.0 };
                        if v2715 == 0.0 {
                            break;
                        }
                        let v2716 = v151 * v2711;
                        let v2717 = v154 * v2711;
                        let v2720 = (Lanes([v2717[0], 0.0, 0.0, 0.0, 0.0])) + (v2713 * v151);
                        let v2723 = (-v2716).exp();
                        let v2724 = (v2720 * v95) * v2723;
                        let v2725 = if v2711 > v1270 { 1.0 } else { 0.0 };
                        let v2770: f64;
                        let v2771: f64;
                        let v2772: Lanes<5>;
                        let v2773: Lanes<5>;
                        if v2725 != 0.0 {
                            let v2727 = v2716.exp();
                            let v2729 = -v312;
                            let v2734 = v2727 - v89;
                            let v2736 = v326 * v2734;
                            let v2737 = (v2720 * v2727) * v324;
                            let v2742 = (((v2723 + v2716) - v89) + (v324 * v2734)).sqrt();
                            let v2746 = v2729 * v2742;
                            let v2747 = (v315 * v95) * v2742;
                            let v2750 = (Lanes([v2747[0], 0.0, 0.0, 0.0, 0.0])) + ((((v2724 + v2720) + ((Lanes([v2736[0], 0.0, 0.0, 0.0, 0.0])) + v2737)) * (v184 / (v236 * v2742))) * v2729);
                            let v2751 = v1287 / v2746;
                            let v2759 = v326 * v2727;
                            let v2762 = ((-v2723) + v89) + (v324 * v2727);
                            let v2764 = v2751 * v2762;
                            let v2767 = ((((v2750 * v2751) * v95) / v2746) * v2762) + (((v2724 * v95) + ((Lanes([v2759[0], 0.0, 0.0, 0.0, 0.0])) + v2737)) * v2751);
                            v2770 = v2746;
                            v2771 = v2764;
                            v2772 = v2750;
                            v2773 = v2767;
                        } else {
                            let v2769 = if v2711 < v2768 { 1.0 } else { 0.0 };
                            let v2861: f64;
                            let v2862: f64;
                            let v2863: Lanes<5>;
                            let v2864: Lanes<5>;
                            if v2769 != 0.0 {
                                let v2813 = ((v2723 + v2716) - v89).sqrt();
                                let v2817 = v312 * v2813;
                                let v2818 = v315 * v2813;
                                let v2821 = (Lanes([v2818[0], 0.0, 0.0, 0.0, 0.0])) + (((v2724 + v2720) * (v184 / (v236 * v2813))) * v312);
                                let v2822 = v1287 / v2817;
                                let v2828 = (-v2723) + v89;
                                let v2829 = v2822 * v2828;
                                let v2832 = ((((v2821 * v2822) * v95) / v2817) * v2828) + ((v2724 * v95) * v2822);
                                v2861 = v2817;
                                v2862 = v2829;
                                v2863 = v2821;
                                v2864 = v2832;
                            } else {
                                let v2833 = v1287 / v151;
                                let v2837 = v2833.sqrt();
                                let v2841 = -v2837;
                                let v2843 = v2841 * v151;
                                let v2847 = v2843 * v2711;
                                let v2848 = (((((((v154 * v2833) * v95) / v151) * (v184 / (v236 * v2837))) * v95) * v151) + (v154 * v2841)) * v2711;
                                let v2851 = (Lanes([v2848[0], 0.0, 0.0, 0.0, 0.0])) + (v2713 * v2843);
                                let v2854 = (v1287 * v151).sqrt();
                                let v2858 = -v2854;
                                let v2859 = ((v154 * v1287) * (v184 / (v236 * v2854))) * v95;
                                let v2860 = Lanes([v2859[0], 0.0, 0.0, 0.0, 0.0]);
                                v2861 = v2847;
                                v2862 = v2858;
                                v2863 = v2851;
                                v2864 = v2860;
                            }
                            v2770 = v2861;
                            v2771 = v2862;
                            v2772 = v2863;
                            v2773 = v2864;
                        }
                        let v2799 = (v2793 + (v2771 / v1359)) + ((v2771 * v813) / v472);
                        let v2801 = ((((v2706 - v2711) + (v2770 / v1359)) + (((v2770 + (v1330 / v254)) * v813) / v472)) - v1134) / v2799;
                        let v2805 = v2711 - v2801;
                        let v2806 = v2713 - ((((((v2708 - v2713) + (v2772 / v1359)) + ((v2772 * v813) / v472)) - (Lanes([v1135[0], 0.0, v1135[1], v1135[2], v1135[3]]))) - (((v2773 / v1359) + ((v2773 * v813) / v472)) * v2801)) / v2799);
                        let v2809 = if ((v2805 - v2711).abs()) < v2707 { 1.0 } else { 0.0 };
                        let v2865: f64;
                        if v2809 != 0.0 {
                            v2865 = v1265;
                        } else {
                            v2865 = v2710;
                        }
                        let v2866 = v2865 + v89;
                        v2710 = v2866;
                        v2711 = v2805;
                        v2712 = v2770;
                        v2713 = v2806;
                        v2714 = v2772;
                    }
                    let v2867: f64;
                    let v2868: Lanes<5>;
                    if v2726 != 0.0 {
                        v2867 = v2712;
                        v2868 = v2714;
                    } else {
                        v2867 = v60;
                        v2868 = v508;
                    }
                    let v2871: f64;
                    let v2872: f64;
                    let v2873: Lanes<5>;
                    if v2869 != 0.0 {
                        v2871 = v2260;
                        v2872 = v2870;
                        v2873 = v2261;
                    } else {
                        v2871 = v2331;
                        v2872 = v633;
                        v2873 = v2332;
                    }
                    let mut v2874: f64 = 0.0;
                    let mut v2875: f64 = 0.0;
                    let mut v2876: f64 = 0.0;
                    let mut v2877: Lanes<5> = Lanes([0.0; 5]);
                    let mut v2878: Lanes<5> = Lanes([0.0; 5]);
                    v2874 = v60;
                    v2875 = v2711;
                    v2876 = v2712;
                    v2877 = v2713;
                    v2878 = v2714;
                    loop {
                        let v2879 = if v2874 < v1265 { 1.0 } else { 0.0 };
                        if v2879 == 0.0 {
                            break;
                        }
                        let v2880 = v151 * v2875;
                        let v2881 = v154 * v2875;
                        let v2884 = (Lanes([v2881[0], 0.0, 0.0, 0.0, 0.0])) + (v2877 * v151);
                        let v2887 = (-v2880).exp();
                        let v2888 = (v2884 * v95) * v2887;
                        let v2889 = if v2875 > v1270 { 1.0 } else { 0.0 };
                        let v2934: f64;
                        let v2935: f64;
                        let v2936: Lanes<5>;
                        let v2937: Lanes<5>;
                        if v2889 != 0.0 {
                            let v2891 = v2880.exp();
                            let v2893 = -v312;
                            let v2898 = v2891 - v89;
                            let v2900 = v326 * v2898;
                            let v2901 = (v2884 * v2891) * v324;
                            let v2906 = (((v2887 + v2880) - v89) + (v324 * v2898)).sqrt();
                            let v2910 = v2893 * v2906;
                            let v2911 = (v315 * v95) * v2906;
                            let v2914 = (Lanes([v2911[0], 0.0, 0.0, 0.0, 0.0])) + ((((v2888 + v2884) + ((Lanes([v2900[0], 0.0, 0.0, 0.0, 0.0])) + v2901)) * (v184 / (v236 * v2906))) * v2893);
                            let v2915 = v1287 / v2910;
                            let v2923 = v326 * v2891;
                            let v2926 = ((-v2887) + v89) + (v324 * v2891);
                            let v2928 = v2915 * v2926;
                            let v2931 = ((((v2914 * v2915) * v95) / v2910) * v2926) + (((v2888 * v95) + ((Lanes([v2923[0], 0.0, 0.0, 0.0, 0.0])) + v2901)) * v2915);
                            v2934 = v2910;
                            v2935 = v2928;
                            v2936 = v2914;
                            v2937 = v2931;
                        } else {
                            let v2933 = if v2875 < v2932 { 1.0 } else { 0.0 };
                            let v3025: f64;
                            let v3026: f64;
                            let v3027: Lanes<5>;
                            let v3028: Lanes<5>;
                            if v2933 != 0.0 {
                                let v2977 = ((v2887 + v2880) - v89).sqrt();
                                let v2981 = v312 * v2977;
                                let v2982 = v315 * v2977;
                                let v2985 = (Lanes([v2982[0], 0.0, 0.0, 0.0, 0.0])) + (((v2888 + v2884) * (v184 / (v236 * v2977))) * v312);
                                let v2986 = v1287 / v2981;
                                let v2992 = (-v2887) + v89;
                                let v2993 = v2986 * v2992;
                                let v2996 = ((((v2985 * v2986) * v95) / v2981) * v2992) + ((v2888 * v95) * v2986);
                                v3025 = v2981;
                                v3026 = v2993;
                                v3027 = v2985;
                                v3028 = v2996;
                            } else {
                                let v2997 = v1287 / v151;
                                let v3001 = v2997.sqrt();
                                let v3005 = -v3001;
                                let v3007 = v3005 * v151;
                                let v3011 = v3007 * v2875;
                                let v3012 = (((((((v154 * v2997) * v95) / v151) * (v184 / (v236 * v3001))) * v95) * v151) + (v154 * v3005)) * v2875;
                                let v3015 = (Lanes([v3012[0], 0.0, 0.0, 0.0, 0.0])) + (v2877 * v3007);
                                let v3018 = (v1287 * v151).sqrt();
                                let v3022 = -v3018;
                                let v3023 = ((v154 * v1287) * (v184 / (v236 * v3018))) * v95;
                                let v3024 = Lanes([v3023[0], 0.0, 0.0, 0.0, 0.0]);
                                v3025 = v3011;
                                v3026 = v3022;
                                v3027 = v3015;
                                v3028 = v3024;
                            }
                            v2934 = v3025;
                            v2935 = v3026;
                            v2936 = v3027;
                            v2937 = v3028;
                        }
                        let v2963 = (v2957 + (v2935 / v1359)) + ((v2935 * v813) / v472);
                        let v2965 = ((((v2871 - v2875) + (v2934 / v1359)) + (((v2934 + (v1330 / v254)) * v813) / v472)) - v1134) / v2963;
                        let v2969 = v2875 - v2965;
                        let v2970 = v2877 - ((((((v2873 - v2877) + (v2936 / v1359)) + ((v2936 * v813) / v472)) - (Lanes([v1135[0], 0.0, v1135[1], v1135[2], v1135[3]]))) - (((v2937 / v1359) + ((v2937 * v813) / v472)) * v2965)) / v2963);
                        let v2973 = if ((v2969 - v2875).abs()) < v2872 { 1.0 } else { 0.0 };
                        let v3029: f64;
                        if v2973 != 0.0 {
                            v3029 = v1265;
                        } else {
                            v3029 = v2874;
                        }
                        let v3030 = v3029 + v89;
                        v2874 = v3030;
                        v2875 = v2969;
                        v2876 = v2934;
                        v2877 = v2970;
                        v2878 = v2936;
                    }
                    let v3031: f64;
                    let v3032: Lanes<5>;
                    if v2890 != 0.0 {
                        v3031 = v2876;
                        v3032 = v2878;
                    } else {
                        v3031 = v2867;
                        v3032 = v2868;
                    }
                    v2542 = v2875;
                    v2543 = v2876;
                    v2544 = v3031;
                    v2545 = v2877;
                    v2546 = v2878;
                    v2547 = v3032;
                }
                let v2550 = (Lanes([v1135[0], 0.0, v1135[1], v1135[2], v1135[3]])) + v2545;
                let v2551 = (v1134 + v2542) - v205;
                let v2554 = v2551 - (v2543 / v1359);
                let v2555 = v2550 - (v2546 / v1359);
                let v2556 = v2331 - v2326;
                let v2559 = if (if v2554 > v2556 { 1.0 } else { 0.0 }) != 0.0 && v2558 != 0.0 { 1.0 } else { 0.0 };
                let v3042: f64;
                let v3043: Lanes<5>;
                if v2559 != 0.0 {
                    let v3034 = v2555 - v2332;
                    let v3035 = (v2554 - v2331) + v2326;
                    let v3037 = v3034 * v3035;
                    let v3038 = v3037 + v3037;
                    let v3040 = (v3035 * v3035) + v3039;
                    let v3051: f64;
                    let v3052: Lanes<5>;
                    if v3041 != 0.0 {
                        let v3067: f64;
                        if v3044 != 0.0 {
                            v3067 = v89;
                        } else {
                            let v3069: f64;
                            if v3066 != 0.0 {
                                v3069 = v254;
                            } else {
                                let v3071: f64;
                                if v3068 != 0.0 {
                                    v3071 = v443;
                                } else {
                                    let v3072: f64;
                                    if v3070 != 0.0 {
                                        v3072 = v446;
                                    } else {
                                        v3072 = v60;
                                    }
                                    v3071 = v3072;
                                }
                                v3069 = v3071;
                            }
                            v3067 = v3069;
                        }
                        let mut v3073: f64 = 0.0;
                        let mut v3074: f64 = 0.0;
                        let mut v3075: Lanes<5> = Lanes([0.0; 5]);
                        v3073 = v60;
                        v3074 = v3040;
                        v3075 = v3038;
                        loop {
                            let v3076 = if v3073 < v3067 { 1.0 } else { 0.0 };
                            if v3076 == 0.0 {
                                break;
                            }
                            let v3077 = v3074.sqrt();
                            let v3080 = v3075 * (v184 / (v236 * v3077));
                            let v3081 = v3073 + v89;
                            v3073 = v3081;
                            v3074 = v3077;
                            v3075 = v3080;
                        }
                        v3051 = v3074;
                        v3052 = v3075;
                    } else {
                        let v3045 = v3040.sqrt();
                        let v3050 = v3038 * (v3048 * (v3040.powf(v3046)));
                        v3051 = v3045;
                        v3052 = v3050;
                    }
                    let v3053 = v3051 + v418;
                    let v3054 = v89 / v3053;
                    let v3058 = v3035 * v2326;
                    let v3064 = v2556 + (v3058 * v3054);
                    let v3065 = v2332 + (((v3034 * v2326) * v3054) + ((((v3052 * v3054) * v95) / v3053) * v3058));
                    v3042 = v3064;
                    v3043 = v3065;
                } else {
                    v3042 = v2554;
                    v3043 = v2555;
                }
                v1432 = v2331;
                v1433 = v3042;
                v1434 = v2551;
                v1435 = v2544;
                v1436 = v2260;
                v1437 = v1525;
                v1438 = v60;
                v1439 = v2543;
                v1440 = v2332;
                v1441 = v3043;
                v1442 = v2550;
                v1443 = v2547;
                v1444 = v2261;
                v1445 = v2546;
            }
            let v1451 = if (if v1446 == v89 { 1.0 } else { 0.0 }) != 0.0 && (if v114 > (v1419 + v1448) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v3228: f64;
            let v3229: f64;
            let v3230: f64;
            let v3231: f64;
            let v3232: f64;
            let v3233: f64;
            let v3234: f64;
            let v3235: f64;
            let v3236: Lanes<6>;
            let v3237: Lanes<5>;
            let v3238: Lanes<1>;
            let v3239: Lanes<1>;
            let v3240: Lanes<5>;
            let v3241: Lanes<5>;
            let v3242: Lanes<5>;
            let v3243: Lanes<6>;
            if v1451 != 0.0 {
                let v3087 = ((v466 - v3082) + v1016) - v1023;
                let v3088 = ((Lanes([0.0, v469[0], v469[1], v469[2], v469[3]])) + v1014) - v1028;
                let v3092 = ((v3089 * v287) * v472) / v151;
                let v3096 = v3092.sqrt();
                let v3099 = (((v154 * v3092) * v95) / v151) * (v184 / (v236 * v3096));
                let v3102 = (v279 / v287) / v287;
                let v3103 = (v281 / v287) / v287;
                let v3105 = v3099 * v3096;
                let v3106 = v3105 + v3105;
                let v3107 = (v3096 * v3096) / v627;
                let v3108 = v630 * v3107;
                let v3113 = v3107 / v627;
                let v3114 = v630 * v3113;
                let v3117 = ((((Lanes([v3106[0], 0.0, 0.0, 0.0, 0.0])) - (Lanes([0.0, v3108[0], v3108[1], v3108[2], v3108[3]]))) / v627) - (Lanes([0.0, v3114[0], v3114[1], v3114[2], v3114[3]]))) / v627;
                let v3120 = v154 * v3113;
                let v3123 = (v3113 * v151) / v254;
                let v3124 = ((v3117 * v151) + (Lanes([v3120[0], 0.0, 0.0, 0.0, 0.0]))) / v254;
                let v3127 = v154 * v3123;
                let v3130 = (v3123 * v151) * v254;
                let v3133 = v154 * v3087;
                let v3140 = (v446 * ((v151 * v3087) - v89)) / v3130;
                let v3145 = (v89 + v3140).sqrt();
                let v3149 = v89 - v3145;
                let v3157 = v89 / v3102;
                let v3160 = ((v3103 * v3157) * v95) / v3102;
                let v3161 = v3157 / v3113;
                let v3166 = v3087 * v3087;
                let v3167 = v3088 * v3087;
                let v3169 = v3161 * v3166;
                let v3176 = v254 / v3087;
                let v3180 = v151 + v3176;
                let v3183 = (v3169.ln()) / v3180;
                let v3186 = (((((((Lanes([v3160[0], 0.0, 0.0, 0.0, 0.0])) - (v3117 * v3161)) / v3113) * v3166) + ((v3167 + v3167) * v3161)) * (v184 / v3169)) - (((Lanes([v154[0], 0.0, 0.0, 0.0, 0.0])) + (((v3088 * v3176) * v95) / v3087)) * v3183)) / v3180;
                let v3188 = v3186 - (v3088 + ((v3124 * v3149) + ((((((((Lanes([v3133[0], 0.0, 0.0, 0.0, 0.0])) + (v3088 * v151)) * v446) - ((((v3124 * v151) + (Lanes([v3127[0], 0.0, 0.0, 0.0, 0.0]))) * v254) * v3140)) / v3130) * (v184 / (v236 * v3145))) * v95) * v3123)));
                let v3190 = (v3183 - (v3087 + (v3123 * v3149))) - v3189;
                let v3192 = v3188 * v3190;
                let v3194 = v446 * v3189;
                let v3199 = ((v3190 * v3190) + (v3194 * v3183)).sqrt();
                let v3207 = v3183 - (v502 * (v3190 + v3199));
                let v3208 = v3186 - ((v3188 + (((v3192 + v3192) + (v3186 * v3194)) * (v184 / (v236 * v3199)))) * v502);
                let v3209 = v151 * v3207;
                let v3210 = v154 * v3207;
                let v3213 = (Lanes([v3210[0], 0.0, 0.0, 0.0, 0.0])) + (v3208 * v151);
                let v3214 = v3209.exp();
                let v3216 = v3209 - v89;
                let v3218 = v3103 * v3214;
                let v3222 = v3216 + (v3102 * v3214);
                let v3223 = v3213 + ((Lanes([v3218[0], 0.0, 0.0, 0.0, 0.0])) + ((v3213 * v3214) * v3102));
                let v3226 = if (if v3222 > v60 { 1.0 } else { 0.0 }) != 0.0 && (if v3216 > v60 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v3318: f64;
                let v3319: f64;
                let v3320: f64;
                let v3321: f64;
                let v3322: f64;
                let v3323: Lanes<6>;
                let v3324: Lanes<5>;
                let v3325: Lanes<5>;
                let v3326: Lanes<5>;
                let v3327: Lanes<6>;
                if v3226 != 0.0 {
                    let v3248 = v3222.sqrt();
                    let v3252 = v3216.sqrt();
                    let v3256 = v3248 - v3252;
                    let v3258 = v3096 * v3256;
                    let v3259 = v3099 * v3256;
                    let v3265 = (v254 * v3263) / v151;
                    let v3269 = -v151;
                    let v3270 = v154 * v95;
                    let v3272 = v3270 * v463;
                    let v3273 = v465 * v3269;
                    let v3277 = (v3269 * v463).exp();
                    let v3280 = -(v3277 - v89);
                    let v3283 = v3265 * v3282;
                    let v3285 = v3283 * v3258;
                    let v3286 = ((((v154 * v3265) * v95) / v151) * v3282) * v3258;
                    let v3292 = ((((Lanes([v3272[0], 0.0, 0.0, 0.0])) + (Lanes([0.0, v3273[0], v3273[1], v3273[2]]))) * v3277) * v95) * v3285;
                    let v3296 = (v3285 * v3280) / v3295;
                    let v3297 = ((((Lanes([v3286[0], 0.0, 0.0, 0.0, 0.0])) + (((Lanes([v3259[0], 0.0, 0.0, 0.0, 0.0])) + (((v3223 * (v184 / (v236 * v3248))) - (v3213 * (v184 / (v236 * v3252)))) * v3096)) * v3283)) * v3280) + (Lanes([v3292[0], 0.0, v3292[1], v3292[2], v3292[3]]))) / v3295;
                    let v3299 = v154 * v1420;
                    let v3306 = v1041 * v155;
                    let v3308 = v157 * v1041;
                    let v3311 = (v446 * ((v151 * v1420) - v89)) / v3306;
                    let v3314 = ((((Lanes([v3299[0], 0.0, 0.0, 0.0, 0.0])) + (v1423 * v151)) * v446) - (((v1043 * v155) + (Lanes([v3308[0], 0.0, 0.0, 0.0, 0.0]))) * v3311)) / v3306;
                    let v3315 = v89 + v3311;
                    let v3317 = if v3315 < v3316 { 1.0 } else { 0.0 };
                    let v3329: f64;
                    let v3330: Lanes<5>;
                    if v3317 != 0.0 {
                        v3329 = v3328;
                        v3330 = v508;
                    } else {
                        v3329 = v3315;
                        v3330 = v3314;
                    }
                    let v3333 = v154 * v1041;
                    let v3336 = (v1041 * v151) * v502;
                    let v3338 = v3329.sqrt();
                    let v3342 = v89 - v3338;
                    let v3348 = v1420 + (v3336 * v3342);
                    let v3349 = v1423 + (((((v1043 * v151) + (Lanes([v3333[0], 0.0, 0.0, 0.0, 0.0]))) * v502) * v3342) + (((v3330 * (v184 / (v236 * v3338))) * v95) * v3336));
                    let v3350 = v3348 - v3207;
                    let v3351 = v3349 - v3208;
                    let v3352 = if v3350 < v60 { 1.0 } else { 0.0 };
                    let v3353: f64;
                    let v3354: Lanes<5>;
                    if v3352 != 0.0 {
                        v3353 = v60;
                        v3354 = v508;
                    } else {
                        v3353 = v3350;
                        v3354 = v3351;
                    }
                    let v3356 = v3355 * v3353;
                    let v3357 = v3354 * v3355;
                    let v3360 = v3357 - (Lanes([0.0, 0.0, v465[0], v465[1], v465[2]]));
                    let v3362 = (v3356 - v463) - v3361;
                    let v3364 = v3360 * v3362;
                    let v3372 = ((v3362 * v3362) + ((v446 * v3356) * v3361)).sqrt();
                    let v3380 = v3356 - (v502 * (v3362 + v3372));
                    let v3381 = v3357 - ((v3360 + (((v3364 + v3364) + ((v3357 * v446) * v3361)) * (v184 / (v236 * v3372)))) * v502);
                    let v3382 = if v3380 > v3353 { 1.0 } else { 0.0 };
                    let v3383: f64;
                    let v3384: Lanes<5>;
                    if v3382 != 0.0 {
                        v3383 = v3353;
                        v3384 = v3354;
                    } else {
                        v3383 = v3380;
                        v3384 = v3381;
                    }
                    let v3386 = v676 * v3385;
                    let v3388 = v3387 * v3385;
                    let v3389 = v3295 * v3385;
                    let v3391 = if v3390 == v60 { 1.0 } else { 0.0 };
                    let v3417: f64;
                    let v3418: Lanes<5>;
                    if v3391 != 0.0 {
                        v3417 = v60;
                        v3418 = v508;
                    } else {
                        let v3395 = ((v3392 * v150) * v3388) * v3389;
                        let v3396 = v3395 / v235;
                        let v3399 = ((v239 * v3396) * v95) / v235;
                        let v3402 = v460 * v3400;
                        let v3415 = (-(((((v3400 * v459) + v972) + v787) + v145) + v3411)) / v3386;
                        let v3416 = (((((Lanes([0.0, 0.0, v3402[0], v3402[1], v3402[2]])) + v976) + v788) + (Lanes([v146[0], 0.0, 0.0, 0.0, 0.0]))) * v95) / v3386;
                        let mut v3423: f64 = 0.0;
                        let mut v3424: f64 = 0.0;
                        let mut v3425: Lanes<5> = Lanes([0.0; 5]);
                        v3423 = v60;
                        v3424 = v60;
                        v3425 = v508;
                        loop {
                            let v3427 = if v3423 <= v3426 { 1.0 } else { 0.0 };
                            if v3427 == 0.0 {
                                break;
                            }
                            let v3428 = v3423 / v3385;
                            let v3436 = (v1422 + v457) - ((v3383 * v3428) + v3207);
                            let v3437 = (v1425 + (Lanes([0.0, 0.0, v458[0], v458[1], v458[2]]))) - ((v3384 * v3428) + v3208);
                            let v3441 = v89 - (v3436 / v3438);
                            let v3442 = (v3437 / v3438) * v95;
                            let v3445 = v3415 + (v3436 / v3386);
                            let v3446 = v3416 + (v3437 / v3386);
                            let v3447 = v3445 * v3445;
                            let v3448 = v3446 * v3445;
                            let v3449 = v3448 + v3448;
                            let v3451 = v3442 * v3441;
                            let v3455 = ((v3441 * v3441) + v3453).sqrt();
                            let v3462 = (v3442 + ((v3451 + v3451) * (v184 / (v236 * v3455)))) * v502;
                            let v3464 = (v502 * (v3441 + v3455)) + v3463;
                            let v3465 = if v3464 < v60 { 1.0 } else { 0.0 };
                            let v3466: f64;
                            let v3467: Lanes<5>;
                            if v3465 != 0.0 {
                                v3466 = v60;
                                v3467 = v508;
                            } else {
                                v3466 = v3464;
                                v3467 = v3462;
                            }
                            let v3468 = v3466.sqrt();
                            let v3479 = v3478 * (v89 - (v3468 * v3466));
                            let v3480 = ((((v3467 * (v184 / (v236 * v3468))) * v3466) + (v3467 * v3468)) * v95) * v3478;
                            let v3483 = (-v3479) / v3445;
                            let v3486 = ((v3480 * v95) - (v3446 * v3483)) / v3445;
                            let v3488 = if v3483 < v3487 { 1.0 } else { 0.0 };
                            let v3491: f64;
                            let v3492: Lanes<5>;
                            if v3488 != 0.0 {
                                v3491 = v60;
                                v3492 = v508;
                            } else {
                                let v3489 = v3483.exp();
                                let v3490 = v3486 * v3489;
                                v3491 = v3489;
                                v3492 = v3490;
                            }
                            let v3493 = v795 * v3396;
                            let v3495 = v3493 * v3479;
                            let v3496 = (v3399 * v795) * v3479;
                            let v3505 = (v3495 * v3479) * v3504;
                            let v3506 = ((((Lanes([v3496[0], 0.0, 0.0, 0.0, 0.0])) + (v3480 * v3493)) * v3479) + (v3480 * v3495)) * v3504;
                            let v3509 = if ((v254 * v3445) + v3479) < v60 { 1.0 } else { 0.0 };
                            let v3519: f64;
                            let v3520: Lanes<5>;
                            if v3509 != 0.0 {
                                v3519 = v3505;
                                v3520 = v3506;
                            } else {
                                let v3510 = v3395 * v3447;
                                let v3512 = v3510 * v3491;
                                let v3515 = ((v3449 * v3395) * v3491) + (v3492 * v3510);
                                let v3518 = if (if v3512 < v3505 { 1.0 } else { 0.0 }) != 0.0 || (if v3445 < v60 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let v3524: f64;
                                let v3525: Lanes<5>;
                                if v3518 != 0.0 {
                                    v3524 = v3505;
                                    v3525 = v3506;
                                } else {
                                    v3524 = v3512;
                                    v3525 = v3515;
                                }
                                v3519 = v3524;
                                v3520 = v3525;
                            }
                            let v3521 = v3424 + v3519;
                            let v3522 = v3425 + v3520;
                            let v3523 = if v3519 < v69 { 1.0 } else { 0.0 };
                            let v3526: f64;
                            if v3523 != 0.0 {
                                v3526 = v3385;
                            } else {
                                v3526 = v3423;
                            }
                            let v3527 = v3526 + v89;
                            v3423 = v3527;
                            v3424 = v3521;
                            v3425 = v3522;
                        }
                        v3417 = v3424;
                        v3418 = v3425;
                    }
                    let v3422 = if (if v3419 <= v60 { 1.0 } else { 0.0 }) != 0.0 || (if v233 <= v60 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v3566: f64;
                    let v3567: Lanes<5>;
                    if v3422 != 0.0 {
                        v3566 = v60;
                        v3567 = v508;
                    } else {
                        let v3528 = v627 * v627;
                        let v3529 = v630 * v627;
                        let v3530 = v3529 + v3529;
                        let v3532 = v254 / v3531;
                        let v3533 = v3532 * v3528;
                        let v3540 = v460 * v3538;
                        let v3541 = (v3087 - v158) - (v3538 * v459);
                        let v3545 = (v3530 * v3532) * v3541;
                        let v3548 = (Lanes([0.0, v3545[0], v3545[1], v3545[2], v3545[3]])) + (((v3088 - (Lanes([v161[0], 0.0, 0.0, 0.0, 0.0]))) - (Lanes([0.0, 0.0, v3540[0], v3540[1], v3540[2]]))) * v3533);
                        let v3549 = v89 + (v3533 * v3541);
                        let v3551 = v3548 * v3549;
                        let v3555 = ((v3549 * v3549) + v3553).sqrt();
                        let v3562 = (v3548 + ((v3551 + v3551) * (v184 / (v236 * v3555)))) * v502;
                        let v3564 = (v502 * (v3549 + v3555)) + v3563;
                        let v3565 = if v3564 < v60 { 1.0 } else { 0.0 };
                        let v3570: f64;
                        let v3571: Lanes<5>;
                        if v3565 != 0.0 {
                            v3570 = v60;
                            v3571 = v508;
                        } else {
                            v3570 = v3564;
                            v3571 = v3562;
                        }
                        let v3576 = v3531 / v3528;
                        let v3580 = (v3570 + v418).sqrt();
                        let v3584 = v89 - v3580;
                        let v3587 = (((v3530 * v3576) * v95) / v3528) * v3584;
                        let v3595 = v465 * v3593;
                        let v3601 = v3599 * v3600;
                        let v3604 = ((v3593 * v463) + v3207) - (v3601 * ((v3087 * v3573) + (v3576 * v3584)));
                        let v3605 = ((Lanes([0.0, 0.0, v3595[0], v3595[1], v3595[2]])) + v3208) - (((v3088 * v3573) + ((Lanes([0.0, v3587[0], v3587[1], v3587[2], v3587[3]])) + (((v3571 * (v184 / (v236 * v3580))) * v95) * v3576))) * v3601);
                        let v3607 = v3605 * v3604;
                        let v3611 = ((v3604 * v3604) + v3609).sqrt();
                        let v3618 = (v3605 + ((v3607 + v3607) * (v184 / (v236 * v3611)))) * v502;
                        let v3620 = (v502 * (v3604 + v3611)) + v3619;
                        let v3621 = if v3620 < v60 { 1.0 } else { 0.0 };
                        let v3622: f64;
                        let v3623: Lanes<5>;
                        if v3621 != 0.0 {
                            v3622 = v60;
                            v3623 = v508;
                        } else {
                            v3622 = v3620;
                            v3623 = v3618;
                        }
                        let v3624 = v3622 + v418;
                        let v3627 = (-v3625) / v3624;
                        let v3631 = v3627.exp();
                        let v3633 = v3419 * v3624;
                        let v3635 = v3633 * v3296;
                        let v3639 = v3635 * v3631;
                        let v3642 = ((((v3623 * v3419) * v3296) + (v3297 * v3633)) * v3631) + (((((v3623 * v3627) * v95) / v3624) * v3631) * v3635);
                        v3566 = v3639;
                        v3567 = v3642;
                    }
                    let v3569 = if v3568 == v89 { 1.0 } else { 0.0 };
                    let v3735: f64;
                    let v3736: f64;
                    let v3737: Lanes<6>;
                    let v3738: Lanes<6>;
                    if v3569 != 0.0 {
                        let v3644 = (v150 * v813) * v3387;
                        let v3648 = (v3269 * v3645).exp();
                        let v3655 = v3654 + (v3652 * v287);
                        let v3656 = (v3644 * v3648) * v3655;
                        let v3659 = v3658 / v3656;
                        let v3663 = v3566 + v3417;
                        let v3667 = (((((((v3270 * v3645) * v3648) * v3644) * v3655) * v3659) * v95) / v3656) * v3663;
                        let v3671 = v3670 * v158;
                        let v3673 = v89 + (v3663 * v3659);
                        let v3674 = v3673.ln();
                        let v3678 = (v161 * v3670) * v3674;
                        let v3683 = v3682 * v287;
                        let v3686 = (v3683 * v158).sqrt();
                        let v3690 = v3207 - (v3671 * v3674);
                        let v3691 = v3208 - ((Lanes([v3678[0], 0.0, 0.0, 0.0, 0.0])) + (((((v3567 + v3418) * v3659) + (Lanes([v3667[0], 0.0, 0.0, 0.0, 0.0]))) * (v184 / v3673)) * v3671));
                        let v3693 = v3270 * v3690;
                        let v3697 = (v3269 * v3690).exp();
                        let v3701 = v154 * v3690;
                        let v3707 = ((v3697 - v89) + (v151 * v3690)).sqrt();
                        let v3712 = v3270 * v3207;
                        let v3716 = (v3269 * v3207).exp();
                        let v3721 = ((v3716 - v89) + v3209).sqrt();
                        let v3725 = -v3686;
                        let v3727 = v3707 - v3721;
                        let v3729 = v3725 * v3727;
                        let v3730 = (((v161 * v3683) * (v184 / (v236 * v3686))) * v95) * v3727;
                        let v3733 = (Lanes([v3730[0], 0.0, 0.0, 0.0, 0.0])) + (((((((Lanes([v3693[0], 0.0, 0.0, 0.0, 0.0])) + (v3691 * v3269)) * v3697) + ((Lanes([v3701[0], 0.0, 0.0, 0.0, 0.0])) + (v3691 * v151))) * (v184 / (v236 * v3707))) - (((((Lanes([v3712[0], 0.0, 0.0, 0.0, 0.0])) + (v3208 * v3269)) * v3716) + v3213) * (v184 / (v236 * v3721)))) * v3725);
                        let v3765: f64;
                        let v3766: f64;
                        let v3767: Lanes<6>;
                        let v3768: Lanes<6>;
                        if v3734 != 0.0 {
                            let v3740 = v3566 + v3739;
                            let v3742 = v3741 / v3740;
                            let v3746 = v3742 * v627;
                            let v3748 = v630 * v3742;
                            let v3752 = v69 * v3751;
                            let v3754 = v3753 * v69;
                            let v3756 = Lanes([0.0, 0.0, 0.0, v3754[0], 0.0, 0.0]);
                            let v3759 = (v3752 - v3729) / v3746;
                            let v3760 = (((((v3567 * v3742) * v95) / v3740) * v627) + (Lanes([0.0, v3748[0], v3748[1], v3748[2], v3748[3]]))) * v3759;
                            let v3763 = ((v3756 - (Lanes([v3733[0], v3733[1], v3733[2], 0.0, v3733[3], v3733[4]]))) - (Lanes([v3760[0], v3760[1], v3760[2], 0.0, v3760[3], v3760[4]]))) / v3746;
                            v3765 = v3752;
                            v3766 = v3759;
                            v3767 = v3756;
                            v3768 = v3763;
                        } else {
                            let v3764 = Lanes([v3733[0], v3733[1], v3733[2], 0.0, v3733[3], v3733[4]]);
                            v3765 = v3729;
                            v3766 = v60;
                            v3767 = v3764;
                            v3768 = v3227;
                        }
                        v3735 = v3765;
                        v3736 = v3766;
                        v3737 = v3767;
                        v3738 = v3768;
                    } else {
                        v3735 = v60;
                        v3736 = v60;
                        v3737 = v3227;
                        v3738 = v3227;
                    }
                    v3318 = v3735;
                    v3319 = v3348;
                    v3320 = v3566;
                    v3321 = v3207;
                    v3322 = v3736;
                    v3323 = v3737;
                    v3324 = v3349;
                    v3325 = v3567;
                    v3326 = v3208;
                    v3327 = v3738;
                } else {
                    v3318 = v60;
                    v3319 = v1436;
                    v3320 = v60;
                    v3321 = v60;
                    v3322 = v60;
                    v3323 = v3227;
                    v3324 = v1444;
                    v3325 = v508;
                    v3326 = v508;
                    v3327 = v3227;
                }
                v3228 = v3318;
                v3229 = v3319;
                v3230 = v3102;
                v3231 = v3096;
                v3232 = v3320;
                v3233 = v3087;
                v3234 = v3321;
                v3235 = v3322;
                v3236 = v3323;
                v3237 = v3324;
                v3238 = v3103;
                v3239 = v3099;
                v3240 = v3325;
                v3241 = v3088;
                v3242 = v3326;
                v3243 = v3327;
            } else {
                v3228 = v60;
                v3229 = v1436;
                v3230 = v283;
                v3231 = v274;
                v3232 = v60;
                v3233 = v60;
                v3234 = v60;
                v3235 = v60;
                v3236 = v3227;
                v3237 = v1444;
                v3238 = v284;
                v3239 = v275;
                v3240 = v508;
                v3241 = v508;
                v3242 = v508;
                v3243 = v3227;
            }
            let v3244 = v1434 - v1134;
            let v3246 = v1442 - (Lanes([v1135[0], 0.0, v1135[1], v1135[2], v1135[3]]));
            let v3776: f64;
            let v3777: f64;
            let v3778: f64;
            if v3247 != 0.0 {
                let v3769 = -v1330;
                let v3771 = v3770 * v1330;
                v3776 = v1330;
                v3777 = v3771;
                v3778 = v3769;
            } else {
                let v3772 = v246 * v1330;
                let v3773 = -v3772;
                let v3775 = v3774 * v1330;
                v3776 = v3772;
                v3777 = v3775;
                v3778 = v3773;
            }
            let v3779 = Lanes([v3246[0], v3246[1], v3246[2], 0.0, v3246[3], v3246[4]]);
            let v3780 = Lanes([v1440[0], v1440[1], v1440[2], 0.0, v1440[3], v1440[4]]);
            let v3781 = Lanes([v1441[0], v1441[1], v1441[2], 0.0, v1441[3], v1441[4]]);
            let v3782 = Lanes([v1445[0], v1445[1], v1445[2], 0.0, v1445[3], v1445[4]]);
            let mut v3783: f64 = 0.0;
            let mut v3784: f64 = 0.0;
            let mut v3785: f64 = 0.0;
            let mut v3786: f64 = 0.0;
            let mut v3787: f64 = 0.0;
            let mut v3788: f64 = 0.0;
            let mut v3789: f64 = 0.0;
            let mut v3790: f64 = 0.0;
            let mut v3791: f64 = 0.0;
            let mut v3792: f64 = 0.0;
            let mut v3793: f64 = 0.0;
            let mut v3794: f64 = 0.0;
            let mut v3795: Lanes<6> = Lanes([0.0; 6]);
            let mut v3796: Lanes<6> = Lanes([0.0; 6]);
            let mut v3797: Lanes<6> = Lanes([0.0; 6]);
            let mut v3798: Lanes<6> = Lanes([0.0; 6]);
            let mut v3799: Lanes<6> = Lanes([0.0; 6]);
            let mut v3800: Lanes<6> = Lanes([0.0; 6]);
            let mut v3801: Lanes<6> = Lanes([0.0; 6]);
            let mut v3802: Lanes<6> = Lanes([0.0; 6]);
            let mut v3803: Lanes<6> = Lanes([0.0; 6]);
            v3783 = v89;
            v3784 = v3244;
            v3785 = v1432;
            v3786 = v1433;
            v3787 = v60;
            v3788 = v60;
            v3789 = v60;
            v3790 = v60;
            v3791 = v1433;
            v3792 = v60;
            v3793 = v1439;
            v3794 = v60;
            v3795 = v3779;
            v3796 = v3780;
            v3797 = v3781;
            v3798 = v3227;
            v3799 = v3227;
            v3800 = v3781;
            v3801 = v3227;
            v3802 = v3782;
            v3803 = v3227;
            loop {
                let v3804 = if v3783 <= v1265 { 1.0 } else { 0.0 };
                if v3804 == 0.0 {
                    break;
                }
                let v3805 = v151 * v3784;
                let v3806 = v154 * v3784;
                let v3809 = (Lanes([v3806[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v3795 * v151);
                let v3812 = (-v3805).exp();
                let v3813 = (v3809 * v95) * v3812;
                let v3815 = if v3784 < v3814 { 1.0 } else { 0.0 };
                let v3856: f64;
                let v3857: f64;
                let v3858: Lanes<6>;
                let v3859: Lanes<6>;
                if v3815 != 0.0 {
                    let v3817 = v3805.exp();
                    let v3822 = v3817 - v89;
                    let v3824 = v326 * v3822;
                    let v3825 = (v3809 * v3817) * v324;
                    let v3830 = (((v3812 + v3805) - v89) + (v324 * v3822)).sqrt();
                    let v3834 = v312 * v3830;
                    let v3835 = v315 * v3830;
                    let v3838 = (Lanes([v3835[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((v3813 + v3809) + ((Lanes([v3824[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + v3825)) * (v184 / (v236 * v3830))) * v312);
                    let v3843 = v326 * v3817;
                    let v3850 = (v1287 * (((-v3812) + v89) + (v324 * v3817))) / v3834;
                    let v3853 = ((((v3813 * v95) + ((Lanes([v3843[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + v3825)) * v1287) - (v3838 * v3850)) / v3834;
                    v3856 = v3834;
                    v3857 = v3850;
                    v3858 = v3838;
                    v3859 = v3853;
                } else {
                    let v3855 = if v3784 > v3854 { 1.0 } else { 0.0 };
                    let v3946: f64;
                    let v3947: f64;
                    let v3948: Lanes<6>;
                    let v3949: Lanes<6>;
                    if v3855 != 0.0 {
                        let v3885 = v3805.exp();
                        let v3886 = v3809 * v3885;
                        let v3887 = -v312;
                        let v3894 = (v3885 - v3805) - v89;
                        let v3896 = v326 * v3894;
                        let v3902 = (((v3812 + v3805) - v89) + (v324 * v3894)).sqrt();
                        let v3906 = v3887 * v3902;
                        let v3907 = (v315 * v95) * v3902;
                        let v3910 = (Lanes([v3907[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((v3813 + v3809) + ((Lanes([v3896[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v3886 - v3809) * v324))) * (v184 / (v236 * v3902))) * v3887);
                        let v3914 = v3885 - v89;
                        let v3916 = v326 * v3914;
                        let v3924 = (v1287 * (((-v3812) + v89) + (v324 * v3914))) / v3906;
                        let v3927 = ((((v3813 * v95) + ((Lanes([v3916[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v3886 * v324))) * v1287) - (v3910 * v3924)) / v3906;
                        v3946 = v3906;
                        v3947 = v3924;
                        v3948 = v3910;
                        v3949 = v3927;
                    } else {
                        let v3928 = -v312;
                        let v3929 = v315 * v95;
                        let v3931 = v3929 * v3805;
                        let v3936 = (v3928 * v3805) / v3935;
                        let v3937 = ((Lanes([v3931[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v3809 * v3928)) / v3935;
                        let v3943 = (v3928 * v151) / v3942;
                        let v3944 = ((v3929 * v151) + (v154 * v3928)) / v3942;
                        let v3945 = Lanes([v3944[0], 0.0, 0.0, 0.0, 0.0, 0.0]);
                        v3946 = v3936;
                        v3947 = v3943;
                        v3948 = v3937;
                        v3949 = v3945;
                    }
                    v3856 = v3946;
                    v3857 = v3947;
                    v3858 = v3948;
                    v3859 = v3949;
                }
                let v3867 = ((v3784 - (v3856 / v1359)) + v353) + v1031;
                let v3869 = ((v3795 - (v3858 / v1359)) + (Lanes([0.0, 0.0, v355[0], 0.0, v355[1], v355[2]]))) + (Lanes([v1032[0], 0.0, 0.0, 0.0, 0.0, 0.0]));
                let v3872 = v89 - (v3857 / v1359);
                let v3873 = (v3859 / v1359) * v95;
                let v3874 = v3785 - v3786;
                let v3876 = v151 * v3874;
                let v3877 = v154 * v3874;
                let v3880 = (Lanes([v3877[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v3796 - v3797) * v151);
                let v3881 = -v3876;
                let v3882 = v3880 * v95;
                let v3884 = if v3881 >= v3883 { 1.0 } else { 0.0 };
                let v3957: f64;
                let v3958: f64;
                let v3959: Lanes<6>;
                let v3960: Lanes<6>;
                if v3884 != 0.0 {
                    let v3953 = v3952 * ((v89 + v3881) - v3883);
                    let v3954 = v3882 * v3952;
                    v3957 = v3953;
                    v3958 = v3952;
                    v3959 = v3954;
                    v3960 = v3227;
                } else {
                    let v3955 = v3881.exp();
                    let v3956 = v3882 * v3955;
                    v3957 = v3955;
                    v3958 = v3955;
                    v3959 = v3956;
                    v3960 = v3956;
                }
                let v3962 = if v3874 < v3961 { 1.0 } else { 0.0 };
                let v3997: f64;
                let v3998: f64;
                let v3999: f64;
                let v4000: f64;
                let v4001: f64;
                let v4002: f64;
                let v4003: Lanes<6>;
                let v4004: Lanes<6>;
                let v4005: Lanes<6>;
                let v4006: Lanes<6>;
                let v4007: Lanes<6>;
                let v4008: Lanes<6>;
                if v3962 != 0.0 {
                    let v3966 = ((v3957 + v3876) - v89).sqrt();
                    let v3969 = (v3959 + v3880) * (v184 / (v236 * v3966));
                    let v3970 = v307 * v3966;
                    let v3971 = v308 * v3966;
                    let v3974 = (Lanes([v3971[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v3969 * v307);
                    let v3975 = v307 * v151;
                    let v3981 = (-v3958) + v89;
                    let v3983 = ((v308 * v151) + (v154 * v307)) * v3981;
                    let v3987 = v254 * v3966;
                    let v3989 = (v3975 * v3981) / v3987;
                    let v3992 = (((Lanes([v3983[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v3960 * v95) * v3975)) - ((v3969 * v254) * v3989)) / v3987;
                    let v3993 = -v3989;
                    let v3994 = v3992 * v95;
                    v3997 = v60;
                    v3998 = v3970;
                    v3999 = v60;
                    v4000 = v3989;
                    v4001 = v60;
                    v4002 = v3993;
                    v4003 = v3227;
                    v4004 = v3974;
                    v4005 = v3227;
                    v4006 = v3992;
                    v4007 = v3227;
                    v4008 = v3994;
                } else {
                    let v3996 = if v3874 > v3995 { 1.0 } else { 0.0 };
                    let v4189: f64;
                    let v4190: f64;
                    let v4191: f64;
                    let v4192: f64;
                    let v4193: f64;
                    let v4194: f64;
                    let v4195: Lanes<6>;
                    let v4196: Lanes<6>;
                    let v4197: Lanes<6>;
                    let v4198: Lanes<6>;
                    let v4199: Lanes<6>;
                    let v4200: Lanes<6>;
                    if v3996 != 0.0 {
                        let v4022 = ((v3957 + v3876) - v89).sqrt();
                        let v4025 = (v3959 + v3880) * (v184 / (v236 * v4022));
                        let v4026 = -v307;
                        let v4027 = v308 * v95;
                        let v4028 = v4026 * v4022;
                        let v4029 = v4027 * v4022;
                        let v4032 = (Lanes([v4029[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v4025 * v4026);
                        let v4033 = v4026 * v151;
                        let v4039 = (-v3958) + v89;
                        let v4041 = ((v4027 * v151) + (v154 * v4026)) * v4039;
                        let v4045 = v254 * v4022;
                        let v4047 = (v4033 * v4039) / v4045;
                        let v4050 = (((Lanes([v4041[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v3960 * v95) * v4033)) - ((v4025 * v254) * v4047)) / v4045;
                        let v4051 = -v4047;
                        let v4052 = v4050 * v95;
                        let v4053 = v3876.exp();
                        let v4054 = v3880 * v4053;
                        let v4056 = v154 * v3786;
                        let v4060 = (v151 * v3786).exp();
                        let v4061 = ((Lanes([v4056[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v3797 * v151)) * v4060;
                        let v4063 = v4032 * v4028;
                        let v4065 = v307 * v307;
                        let v4066 = v308 * v307;
                        let v4067 = v4066 + v4066;
                        let v4068 = (v4028 * v4028) / v4065;
                        let v4069 = v4067 * v4068;
                        let v4073 = v254 * v318;
                        let v4075 = v4073 * v4060;
                        let v4076 = (v320 * v254) * v4060;
                        let v4082 = (v4053 - v3876) - v89;
                        let v4089 = (v4068 + (v4075 * v4082)).sqrt();
                        let v4092 = ((((v4063 + v4063) - (Lanes([v4069[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v4065) + ((((Lanes([v4076[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v4061 * v4073)) * v4082) + ((v4054 - v3880) * v4075))) * (v184 / (v236 * v4089));
                        let v4093 = v254 * v4028;
                        let v4094 = v4032 * v254;
                        let v4099 = (v4093 * v4047) / v4065;
                        let v4100 = v4067 * v4099;
                        let v4104 = v254 * v151;
                        let v4106 = v4104 * v318;
                        let v4110 = v4106 * v4060;
                        let v4111 = (((v154 * v254) * v318) + (v320 * v4104)) * v4060;
                        let v4114 = (Lanes([v4111[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v4061 * v4106);
                        let v4115 = v4053 - v89;
                        let v4122 = v254 * v4089;
                        let v4123 = v4092 * v254;
                        let v4124 = (v4099 + (v4110 * v4115)) / v4122;
                        let v4132 = (v4093 * v4051) / v4065;
                        let v4133 = v4067 * v4132;
                        let v4143 = (v4132 - (v4110 * v3876)) / v4122;
                        let v4148 = v4027 * v4089;
                        let v4152 = (v4026 * v4089) - v4028;
                        let v4153 = ((Lanes([v4148[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v4092 * v4026)) - v4032;
                        let v4155 = v4027 * v4124;
                        let v4159 = (v4026 * v4124) - v4047;
                        let v4160 = ((Lanes([v4155[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((((((v4094 * v4047) + (v4050 * v4093)) - (Lanes([v4100[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v4065) + ((v4114 * v4115) + (v4054 * v4110))) - (v4123 * v4124)) / v4122) * v4026)) - v4050;
                        let v4162 = v4027 * v4143;
                        let v4166 = (v4026 * v4143) - v4051;
                        let v4167 = ((Lanes([v4162[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((((((v4094 * v4051) + (v4052 * v4093)) - (Lanes([v4133[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v4065) - ((v4114 * v3876) + (v3880 * v4110))) - (v4123 * v4143)) / v4122) * v4026)) - v4052;
                        v4189 = v4152;
                        v4190 = v4028;
                        v4191 = v4159;
                        v4192 = v4047;
                        v4193 = v4166;
                        v4194 = v4051;
                        v4195 = v4153;
                        v4196 = v4032;
                        v4197 = v4160;
                        v4198 = v4050;
                        v4199 = v4167;
                        v4200 = v4052;
                    } else {
                        let v4168 = -v307;
                        let v4169 = v308 * v95;
                        let v4171 = v4169 * v3876;
                        let v4176 = (v4168 * v3876) / v4175;
                        let v4177 = ((Lanes([v4171[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v3880 * v4168)) / v4175;
                        let v4183 = (v4168 * v151) / v4182;
                        let v4184 = ((v4169 * v151) + (v154 * v4168)) / v4182;
                        let v4185 = -v4183;
                        let v4186 = v4184 * v95;
                        let v4187 = Lanes([v4184[0], 0.0, 0.0, 0.0, 0.0, 0.0]);
                        let v4188 = Lanes([v4186[0], 0.0, 0.0, 0.0, 0.0, 0.0]);
                        v4189 = v60;
                        v4190 = v4176;
                        v4191 = v60;
                        v4192 = v4183;
                        v4193 = v60;
                        v4194 = v4185;
                        v4195 = v3227;
                        v4196 = v4177;
                        v4197 = v3227;
                        v4198 = v4187;
                        v4199 = v3227;
                        v4200 = v4188;
                    }
                    v3997 = v4189;
                    v3998 = v4190;
                    v3999 = v4191;
                    v4000 = v4192;
                    v4001 = v4193;
                    v4002 = v4194;
                    v4003 = v4195;
                    v4004 = v4196;
                    v4005 = v4197;
                    v4006 = v4198;
                    v4007 = v4199;
                    v4008 = v4200;
                }
                let v4009 = v3867 - v3786;
                let v4011 = v151 * v4009;
                let v4012 = v154 * v4009;
                let v4015 = (Lanes([v4012[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v3869 - v3797) * v151);
                let v4016 = -v4011;
                let v4017 = v4015 * v95;
                let v4018 = if v4016 >= v3883 { 1.0 } else { 0.0 };
                let v4207: f64;
                let v4208: f64;
                let v4209: Lanes<6>;
                let v4210: Lanes<6>;
                if v4018 != 0.0 {
                    let v4203 = v3952 * ((v89 + v4016) - v3883);
                    let v4204 = v4017 * v3952;
                    v4207 = v4203;
                    v4208 = v3952;
                    v4209 = v4204;
                    v4210 = v3227;
                } else {
                    let v4205 = v4016.exp();
                    let v4206 = v4017 * v4205;
                    v4207 = v4205;
                    v4208 = v4205;
                    v4209 = v4206;
                    v4210 = v4206;
                }
                let v4212 = if v4009 < v4211 { 1.0 } else { 0.0 };
                let v4247: f64;
                let v4248: f64;
                let v4249: f64;
                let v4250: f64;
                let v4251: f64;
                let v4252: f64;
                let v4253: Lanes<6>;
                let v4254: Lanes<6>;
                let v4255: Lanes<6>;
                let v4256: Lanes<6>;
                let v4257: Lanes<6>;
                let v4258: Lanes<6>;
                if v4212 != 0.0 {
                    let v4216 = ((v4207 + v4011) - v89).sqrt();
                    let v4219 = (v4209 + v4015) * (v184 / (v236 * v4216));
                    let v4220 = v307 * v4216;
                    let v4221 = v308 * v4216;
                    let v4224 = (Lanes([v4221[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v4219 * v307);
                    let v4225 = v307 * v151;
                    let v4231 = (-v4208) + v89;
                    let v4233 = ((v308 * v151) + (v154 * v307)) * v4231;
                    let v4237 = v254 * v4216;
                    let v4239 = (v4225 * v4231) / v4237;
                    let v4242 = (((Lanes([v4233[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v4210 * v95) * v4225)) - ((v4219 * v254) * v4239)) / v4237;
                    let v4243 = -v4239;
                    let v4244 = v4242 * v95;
                    v4247 = v60;
                    v4248 = v4220;
                    v4249 = v60;
                    v4250 = v4243;
                    v4251 = v60;
                    v4252 = v4239;
                    v4253 = v3227;
                    v4254 = v4224;
                    v4255 = v3227;
                    v4256 = v4244;
                    v4257 = v3227;
                    v4258 = v4242;
                } else {
                    let v4246 = if v4009 > v4245 { 1.0 } else { 0.0 };
                    let v4430: f64;
                    let v4431: f64;
                    let v4432: f64;
                    let v4433: f64;
                    let v4434: f64;
                    let v4435: f64;
                    let v4436: Lanes<6>;
                    let v4437: Lanes<6>;
                    let v4438: Lanes<6>;
                    let v4439: Lanes<6>;
                    let v4440: Lanes<6>;
                    let v4441: Lanes<6>;
                    if v4246 != 0.0 {
                        let v4263 = ((v4207 + v4011) - v89).sqrt();
                        let v4266 = (v4209 + v4015) * (v184 / (v236 * v4263));
                        let v4267 = -v307;
                        let v4268 = v308 * v95;
                        let v4269 = v4267 * v4263;
                        let v4270 = v4268 * v4263;
                        let v4273 = (Lanes([v4270[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v4266 * v4267);
                        let v4274 = v4267 * v151;
                        let v4280 = (-v4208) + v89;
                        let v4282 = ((v4268 * v151) + (v154 * v4267)) * v4280;
                        let v4286 = v254 * v4263;
                        let v4288 = (v4274 * v4280) / v4286;
                        let v4291 = (((Lanes([v4282[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v4210 * v95) * v4274)) - ((v4266 * v254) * v4288)) / v4286;
                        let v4292 = -v4288;
                        let v4293 = v4291 * v95;
                        let v4294 = v4011.exp();
                        let v4295 = v4015 * v4294;
                        let v4297 = v154 * v3786;
                        let v4301 = (v151 * v3786).exp();
                        let v4302 = ((Lanes([v4297[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v3797 * v151)) * v4301;
                        let v4304 = v4273 * v4269;
                        let v4306 = v307 * v307;
                        let v4307 = v308 * v307;
                        let v4308 = v4307 + v4307;
                        let v4309 = (v4269 * v4269) / v4306;
                        let v4310 = v4308 * v4309;
                        let v4314 = v254 * v318;
                        let v4316 = v4314 * v4301;
                        let v4317 = (v320 * v254) * v4301;
                        let v4323 = (v4294 - v4011) - v89;
                        let v4330 = (v4309 + (v4316 * v4323)).sqrt();
                        let v4333 = ((((v4304 + v4304) - (Lanes([v4310[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v4306) + ((((Lanes([v4317[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v4302 * v4314)) * v4323) + ((v4295 - v4015) * v4316))) * (v184 / (v236 * v4330));
                        let v4334 = v254 * v4269;
                        let v4335 = v4273 * v254;
                        let v4340 = (v4334 * v4288) / v4306;
                        let v4341 = v4308 * v4340;
                        let v4345 = v254 * v151;
                        let v4347 = v4345 * v318;
                        let v4351 = v4347 * v4301;
                        let v4352 = (((v154 * v254) * v318) + (v320 * v4345)) * v4301;
                        let v4355 = (Lanes([v4352[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v4302 * v4347);
                        let v4356 = v4294 - v89;
                        let v4363 = v254 * v4330;
                        let v4364 = v4333 * v254;
                        let v4365 = (v4340 + (v4351 * v4356)) / v4363;
                        let v4373 = (v4334 * v4292) / v4306;
                        let v4374 = v4308 * v4373;
                        let v4384 = (v4373 - (v4351 * v4011)) / v4363;
                        let v4389 = v4268 * v4330;
                        let v4393 = (v4267 * v4330) - v4269;
                        let v4394 = ((Lanes([v4389[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v4333 * v4267)) - v4273;
                        let v4396 = v4268 * v4365;
                        let v4400 = (v4267 * v4365) - v4288;
                        let v4401 = ((Lanes([v4396[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((((((v4335 * v4288) + (v4291 * v4334)) - (Lanes([v4341[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v4306) + ((v4355 * v4356) + (v4295 * v4351))) - (v4364 * v4365)) / v4363) * v4267)) - v4291;
                        let v4403 = v4268 * v4384;
                        let v4407 = (v4267 * v4384) - v4292;
                        let v4408 = ((Lanes([v4403[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((((((v4335 * v4292) + (v4293 * v4334)) - (Lanes([v4374[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v4306) - ((v4355 * v4011) + (v4015 * v4351))) - (v4364 * v4384)) / v4363) * v4267)) - v4293;
                        v4430 = v4393;
                        v4431 = v4269;
                        v4432 = v4407;
                        v4433 = v4292;
                        v4434 = v4400;
                        v4435 = v4288;
                        v4436 = v4394;
                        v4437 = v4273;
                        v4438 = v4408;
                        v4439 = v4293;
                        v4440 = v4401;
                        v4441 = v4291;
                    } else {
                        let v4409 = -v307;
                        let v4410 = v308 * v95;
                        let v4412 = v4410 * v4011;
                        let v4417 = (v4409 * v4011) / v4416;
                        let v4418 = ((Lanes([v4412[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v4015 * v4409)) / v4416;
                        let v4424 = (v4409 * v151) / v4423;
                        let v4425 = ((v4410 * v151) + (v154 * v4409)) / v4423;
                        let v4426 = -v4424;
                        let v4427 = v4425 * v95;
                        let v4428 = Lanes([v4427[0], 0.0, 0.0, 0.0, 0.0, 0.0]);
                        let v4429 = Lanes([v4425[0], 0.0, 0.0, 0.0, 0.0, 0.0]);
                        v4430 = v60;
                        v4431 = v4417;
                        v4432 = v60;
                        v4433 = v4426;
                        v4434 = v60;
                        v4435 = v4424;
                        v4436 = v3227;
                        v4437 = v4418;
                        v4438 = v3227;
                        v4439 = v4428;
                        v4440 = v3227;
                        v4441 = v4429;
                    }
                    v4247 = v4430;
                    v4248 = v4431;
                    v4249 = v4432;
                    v4250 = v4433;
                    v4251 = v4434;
                    v4252 = v4435;
                    v4253 = v4436;
                    v4254 = v4437;
                    v4255 = v4438;
                    v4256 = v4439;
                    v4257 = v4440;
                    v4258 = v4441;
                }
                let v4259 = if v3787 == v89 { 1.0 } else { 0.0 };
                let v4500: f64;
                let v4501: f64;
                let v4502: f64;
                let v4503: f64;
                let v4504: f64;
                let v4505: f64;
                let v4506: Lanes<6>;
                let v4507: Lanes<6>;
                let v4508: Lanes<6>;
                if v4259 != 0.0 {
                    v4500 = v1265;
                    v4501 = v3784;
                    v4502 = v3785;
                    v4503 = v3786;
                    v4504 = v3787;
                    v4505 = v3783;
                    v4506 = v3795;
                    v4507 = v3796;
                    v4508 = v3797;
                } else {
                    let v4455 = (((((v3856 + v3997) + v3998) + v4247) + v4248) + v3228) / v627;
                    let v4456 = v630 * v4455;
                    let v4460 = (v3785 - v1420) - v4455;
                    let v4461 = (v3796 - (Lanes([v1423[0], v1423[1], v1423[2], 0.0, v1423[3], v1423[4]]))) - (((((((v3858 + v4003) + v4004) + v4253) + v4254) + v3236) - (Lanes([0.0, v4456[0], v4456[1], 0.0, v4456[2], v4456[3]]))) / v627);
                    let v4464 = (v3999 + v4000) / v627;
                    let v4465 = v630 * v4464;
                    let v4469 = v89 - v4464;
                    let v4470 = (((v4005 + v4006) - (Lanes([0.0, v4465[0], v4465[1], 0.0, v4465[2], v4465[3]]))) / v627) * v95;
                    let v4479 = (-(((v4001 + v4002) + v4249) + v4250)) / v627;
                    let v4480 = v630 * v4479;
                    let v4483 = (((((v4007 + v4008) + v4255) + v4256) * v95) - (Lanes([0.0, v4480[0], v4480[1], 0.0, v4480[2], v4480[3]]))) / v627;
                    let v4484 = v4251 + v4252;
                    let v4494 = (-(v3857 + (v4484 * v3872))) / v627;
                    let v4495 = v630 * v4494;
                    let v4498 = (((v3859 + (((v4257 + v4258) * v3872) + (v3873 * v4484))) * v95) - (Lanes([0.0, v4495[0], v4495[1], 0.0, v4495[2], v4495[3]]))) / v627;
                    let v4499 = if v3856 <= v3776 { 1.0 } else { 0.0 };
                    if v4499 != 0.0 {
                    } else {
                        let v4510 = if v3856 <= v3777 { 1.0 } else { 0.0 };
                    }
                    let v4513 = (-v1435) / v1330;
                    let v4514 = (v1443 * v95) / v1330;
                    let v4520 = (-(v4513 * v4515)).exp();
                    let v4522 = v89 + v4520;
                    let v4523 = v89 / v4522;
                    let v4531 = (((((((v4514 * v4515) * v95) * v4520) * v4523) * v95) / v4522) * v3778) * v95;
                    let v4536 = (v3998 + (-(v1330 + (v4523 * v3778)))) / v4535;
                    let v4537 = (v4004 + (Lanes([v4531[0], v4531[1], v4531[2], 0.0, v4531[3], v4531[4]]))) / v4535;
                    let v4538 = v4000 / v4535;
                    let v4539 = v4006 / v4535;
                    let v4540 = v4002 / v4535;
                    let v4541 = v4008 / v4535;
                    let v4542 = v60 / v4535;
                    let v4548 = (-(v4513 * v4543)).exp();
                    let v4550 = v89 + v4548;
                    let v4551 = v89 / v4550;
                    let v4556 = ((((((v4514 * v4543) * v95) * v4548) * v4551) * v95) / v4550) * v3778;
                    let v4560 = (v4248 + (v4551 * v3778)) / v4535;
                    let v4561 = (v4254 + (Lanes([v4556[0], v4556[1], v4556[2], 0.0, v4556[3], v4556[4]]))) / v4535;
                    let v4562 = v4250 / v4535;
                    let v4563 = v4256 / v4535;
                    let v4568 = (v4252 * v3872) / v4535;
                    let v4569 = ((v4258 * v3872) + (v3873 * v4252)) / v4535;
                    let v4570 = v4469 * v4540;
                    let v4573 = (v4470 * v4540) + (v4541 * v4469);
                    let v4578 = v4469 * v4542;
                    let v4579 = v4470 * v4542;
                    let v4586 = v4479 * v4538;
                    let v4589 = (v4483 * v4538) + (v4539 * v4479);
                    let v4596 = v4494 * v4538;
                    let v4599 = (v4498 * v4538) + (v4539 * v4494);
                    let v4604 = (((v4570 * v4568) - (v4578 * v4562)) - (v4586 * v4568)) + (v4596 * v4562);
                    let v4605 = ((((v4573 * v4568) + (v4569 * v4570)) - ((v4579 * v4562) + (v4563 * v4578))) - ((v4589 * v4568) + (v4569 * v4586))) + ((v4599 * v4562) + (v4563 * v4596));
                    let v4606 = if v4604 > v60 { 1.0 } else { 0.0 };
                    let v4617: f64;
                    let v4618: Lanes<6>;
                    if v4606 != 0.0 {
                        let v4607 = v4604 + v418;
                        let v4608 = v89 / v4607;
                        let v4611 = ((v4605 * v4608) * v95) / v4607;
                        v4617 = v4608;
                        v4618 = v4611;
                    } else {
                        let v4612 = v4604 - v418;
                        let v4613 = v89 / v4612;
                        let v4616 = ((v4605 * v4613) * v95) / v4612;
                        v4617 = v4613;
                        v4618 = v4616;
                    }
                    let v4625 = (v4540 * v4568) - (v4542 * v4562);
                    let v4635 = (v4494 * v4562) - (v4479 * v4568);
                    let v4643 = (v4479 * v4542) - (v4494 * v4540);
                    let v4645 = -v4538;
                    let v4647 = v4645 * v4568;
                    let v4651 = v4469 * v4568;
                    let v4655 = v4596 - v4578;
                    let v4657 = v4538 * v4562;
                    let v4661 = -v4469;
                    let v4663 = v4661 * v4562;
                    let v4667 = v4570 - v4586;
                    let v4669 = -v4617;
                    let v4670 = v4618 * v95;
                    let v4685 = ((v4625 * v4460) + (v4635 * v4536)) + (v4643 * v4560);
                    let v4687 = v4669 * v4685;
                    let v4690 = (v4670 * v4685) + ((((((((v4541 * v4568) + (v4569 * v4540)) - (v4563 * v4542)) * v4460) + (v4461 * v4625)) + (((((v4498 * v4562) + (v4563 * v4494)) - ((v4483 * v4568) + (v4569 * v4479))) * v4536) + (v4537 * v4635))) + ((((v4483 * v4542) - ((v4498 * v4540) + (v4541 * v4494))) * v4560) + (v4561 * v4643))) * v4669);
                    let v4705 = ((v4647 * v4460) + (v4651 * v4536)) + (v4655 * v4560);
                    let v4707 = v4669 * v4705;
                    let v4710 = (v4670 * v4705) + ((((((((v4539 * v95) * v4568) + (v4569 * v4645)) * v4460) + (v4461 * v4647)) + ((((v4470 * v4568) + (v4569 * v4469)) * v4536) + (v4537 * v4651))) + (((v4599 - v4579) * v4560) + (v4561 * v4655))) * v4669);
                    let v4725 = ((v4657 * v4460) + (v4663 * v4536)) + (v4667 * v4560);
                    let v4727 = v4669 * v4725;
                    let v4730 = (v4670 * v4725) + (((((((v4539 * v4562) + (v4563 * v4538)) * v4460) + (v4461 * v4657)) + (((((v4470 * v95) * v4562) + (v4563 * v4661)) * v4536) + (v4537 * v4663))) + (((v4573 - v4589) * v4560) + (v4561 * v4667))) * v4669);
                    let v4731 = v4687.abs();
                    let v4736 = v4690 * ((v236 * (if v4687 >= v4732 { 1.0 } else { 0.0 })) - v184);
                    let v4737 = v4707.abs();
                    let v4741 = v4710 * ((v236 * (if v4707 >= v4732 { 1.0 } else { 0.0 })) - v184);
                    let v4742 = if v4731 < v4737 { 1.0 } else { 0.0 };
                    let v4743: f64;
                    let v4744: Lanes<6>;
                    if v4742 != 0.0 {
                        v4743 = v4737;
                        v4744 = v4741;
                    } else {
                        v4743 = v4731;
                        v4744 = v4736;
                    }
                    let v4745 = v4727.abs();
                    let v4749 = v4730 * ((v236 * (if v4727 >= v4732 { 1.0 } else { 0.0 })) - v184);
                    let v4750 = if v4743 < v4745 { 1.0 } else { 0.0 };
                    let v4751: f64;
                    let v4752: Lanes<6>;
                    if v4750 != 0.0 {
                        v4751 = v4745;
                        v4752 = v4749;
                    } else {
                        v4751 = v4743;
                        v4752 = v4744;
                    }
                    let v4753 = if v3783 > v3883 { 1.0 } else { 0.0 };
                    let v4757: f64;
                    if v4753 != 0.0 {
                        v4757 = v4754;
                    } else {
                        let v4756 = if v3783 > v4755 { 1.0 } else { 0.0 };
                        let v4762: f64;
                        if v4756 != 0.0 {
                            v4762 = v4754;
                        } else {
                            let v4761 = if v3783 > v4760 { 1.0 } else { 0.0 };
                            let v4765: f64;
                            if v4761 != 0.0 {
                                v4765 = v4754;
                            } else {
                                let v4764 = if v3783 > v4763 { 1.0 } else { 0.0 };
                                let v4767: f64;
                                if v4764 != 0.0 {
                                    v4767 = v4766;
                                } else {
                                    v4767 = v89;
                                }
                                v4765 = v4767;
                            }
                            v4762 = v4765;
                        }
                        v4757 = v4762;
                    }
                    let v4758 = v210 / v4757;
                    let v4759 = if v4751 > v4758 { 1.0 } else { 0.0 };
                    let v4784: f64;
                    let v4785: f64;
                    let v4786: f64;
                    let v4787: Lanes<6>;
                    let v4788: Lanes<6>;
                    let v4789: Lanes<6>;
                    if v4759 != 0.0 {
                        let v4768 = v4758 / v4751;
                        let v4771 = ((v4752 * v4768) * v95) / v4751;
                        let v4772 = v4687 * v4768;
                        let v4775 = (v4690 * v4768) + (v4771 * v4687);
                        let v4776 = v4707 * v4768;
                        let v4779 = (v4710 * v4768) + (v4771 * v4707);
                        let v4780 = v4727 * v4768;
                        let v4783 = (v4730 * v4768) + (v4771 * v4727);
                        v4784 = v4772;
                        v4785 = v4776;
                        v4786 = v4780;
                        v4787 = v4775;
                        v4788 = v4779;
                        v4789 = v4783;
                    } else {
                        v4784 = v4687;
                        v4785 = v4707;
                        v4786 = v4727;
                        v4787 = v4690;
                        v4788 = v4710;
                        v4789 = v4730;
                    }
                    let v4790 = v3785 + v4784;
                    let v4791 = v3796 + v4787;
                    let v4792 = v3786 + v4785;
                    let v4793 = v3797 + v4788;
                    let v4794 = v3784 + v4786;
                    let v4795 = v3795 + v4789;
                    let v4797 = if v4751 < (v407 * v4757) { 1.0 } else { 0.0 };
                    let v4798: f64;
                    if v4797 != 0.0 {
                        v4798 = v89;
                    } else {
                        v4798 = v3787;
                    }
                    v4500 = v3783;
                    v4501 = v4794;
                    v4502 = v4790;
                    v4503 = v4792;
                    v4504 = v4798;
                    v4505 = v3788;
                    v4506 = v4795;
                    v4507 = v4791;
                    v4508 = v4793;
                }
                let v4509 = v4500 + v89;
                v3783 = v4509;
                v3784 = v4501;
                v3785 = v4502;
                v3786 = v4503;
                v3787 = v4504;
                v3788 = v4505;
                v3789 = v3997;
                v3790 = v4247;
                v3791 = v3867;
                v3792 = v3998;
                v3793 = v3856;
                v3794 = v4248;
                v3795 = v4506;
                v3796 = v4507;
                v3797 = v4508;
                v3798 = v4003;
                v3799 = v4253;
                v3800 = v3869;
                v3801 = v4004;
                v3802 = v3858;
                v3803 = v4254;
            }
            let v3816 = if v3788 > v60 { 1.0 } else { 0.0 };
            let v4799: f64;
            let v4800: f64;
            if v3816 != 0.0 {
                v4799 = v3788;
                v4800 = v60;
            } else {
                v4799 = v3783;
                v4800 = v3788;
            }
            let v4801 = if v4799 > v1265 { 1.0 } else { 0.0 };
            let v4802: f64;
            let v4803: f64;
            let v4804: f64;
            let v4805: f64;
            let v4806: Lanes<6>;
            let v4807: Lanes<6>;
            let v4808: Lanes<6>;
            let v4809: Lanes<6>;
            if v4801 != 0.0 {
                v4802 = v1432;
                v4803 = v1433;
                v4804 = v1433;
                v4805 = v3244;
                v4806 = v3780;
                v4807 = v3781;
                v4808 = v3781;
                v4809 = v3779;
            } else {
                v4802 = v3785;
                v4803 = v3791;
                v4804 = v3786;
                v4805 = v3784;
                v4806 = v3796;
                v4807 = v3800;
                v4808 = v3797;
                v4809 = v3795;
            }
            let v4810 = -v3789;
            let v4811 = v3798 * v95;
            let v4812 = if v4810 <= v418 { 1.0 } else { 0.0 };
            let v4813: f64;
            let v4814: f64;
            let v4815: Lanes<6>;
            if v4812 != 0.0 {
                v4813 = v418;
                v4814 = v89;
                v4815 = v3227;
            } else {
                v4813 = v4810;
                v4814 = v60;
                v4815 = v4811;
            }
            let v4816 = -v3790;
            let v4817 = v3799 * v95;
            let v4818 = if v4816 <= v418 { 1.0 } else { 0.0 };
            let v4819: f64;
            let v4820: Lanes<6>;
            if v4818 != 0.0 {
                v4819 = v418;
                v4820 = v3227;
            } else {
                v4819 = v4816;
                v4820 = v4817;
            }
            let v4821 = v4813 * v625;
            let v4823 = v628 * v4813;
            let v4825 = (v4815 * v625) + (Lanes([0.0, v4823[0], v4823[1], 0.0, v4823[2], v4823[3]]));
            let v4826 = v627 * v627;
            let v4827 = v630 * v627;
            let v4828 = v4827 + v4827;
            let v4829 = v3531 / v4826;
            let v4832 = ((v4828 * v4829) * v95) / v4826;
            let v4833 = v1420 - v158;
            let v4834 = Lanes([v161[0], 0.0, 0.0, 0.0, 0.0]);
            let v4836 = v254 / v4829;
            let v4841 = (((v4832 * v4836) * v95) / v4829) * v4833;
            let v4844 = (Lanes([0.0, v4841[0], v4841[1], v4841[2], v4841[3]])) + ((v1423 - v4834) * v4836);
            let v4845 = v89 + (v4836 * v4833);
            let v4847 = v4844 * v4845;
            let v4851 = ((v4845 * v4845) + v4849).sqrt();
            let v4858 = (v4844 + ((v4847 + v4847) * (v184 / (v236 * v4851)))) * v502;
            let v4860 = (v502 * (v4845 + v4851)) + v4859;
            let v4861 = if v4860 < v60 { 1.0 } else { 0.0 };
            let v4862: f64;
            let v4863: Lanes<5>;
            if v4861 != 0.0 {
                v4862 = v60;
                v4863 = v508;
            } else {
                v4862 = v4860;
                v4863 = v4858;
            }
            let v4864 = v4862.sqrt();
            let v4868 = v89 - v4864;
            let v4871 = v4832 * v4868;
            let v4875 = v1420 + (v4829 * v4868);
            let v4876 = v1423 + ((Lanes([0.0, v4871[0], v4871[1], v4871[2], v4871[3]])) + (((v4863 * (v184 / (v236 * v4864))) * v95) * v4829));
            let v4878 = v4876 * v4875;
            let v4882 = ((v4875 * v4875) + v4880).sqrt();
            let v4889 = (v4876 + ((v4878 + v4878) * (v184 / (v236 * v4882)))) * v502;
            let v4891 = (v502 * (v4875 + v4882)) + v4890;
            let v4892 = if v4891 < v60 { 1.0 } else { 0.0 };
            let v4893: f64;
            let v4894: Lanes<5>;
            if v4892 != 0.0 {
                v4893 = v60;
                v4894 = v508;
            } else {
                v4893 = v4891;
                v4894 = v4889;
            }
            let v4895 = v113 / v4893;
            let v4898 = (v548 - (v4894 * v4895)) / v4893;
            let v4899 = v4895 + v418;
            let v4901 = v4899.powf(v4900);
            let v4909 = ((v4898 * (v4900 * (v4899.powf(v4902)))) * v4899) + (v4898 * v4901);
            let v4910 = v89 + (v4901 * v4899);
            let v4912 = v4910.powf(v4911);
            let v4917 = v4912 * v4910;
            let v4921 = v113 / v4917;
            let v4924 = (v548 - ((((v4909 * (v4911 * (v4910.powf(v4913)))) * v4910) + (v4909 * v4912)) * v4921)) / v4917;
            let v4925 = if v4921 < v60 { 1.0 } else { 0.0 };
            let v4929: f64;
            let v4930: f64;
            let v4931: f64;
            let v4932: f64;
            let v4933: Lanes<6>;
            let v4934: Lanes<6>;
            let v4935: Lanes<6>;
            let v4936: Lanes<6>;
            if v4925 != 0.0 {
                v4929 = v4802;
                v4930 = v4803;
                v4931 = v4805;
                v4932 = v60;
                v4933 = v4806;
                v4934 = v4807;
                v4935 = v4809;
                v4936 = v3227;
            } else {
                let v4928 = if v4927 != 0.0 || (if v4821 < v407 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v4943: f64;
                let v4944: f64;
                let v4945: f64;
                let v4946: f64;
                let v4947: Lanes<6>;
                let v4948: Lanes<6>;
                let v4949: Lanes<6>;
                let v4950: Lanes<6>;
                if v4928 != 0.0 {
                    let v4938 = Lanes([v1135[0], 0.0, v1135[1], 0.0, v1135[2], v1135[3]]);
                    v4943 = v60;
                    v4944 = v60;
                    v4945 = v1134;
                    v4946 = v60;
                    v4947 = v3227;
                    v4948 = v3227;
                    v4949 = v4938;
                    v4950 = v3227;
                } else {
                    let v4939 = v3229 - v4802;
                    let v4941 = (Lanes([v3237[0], v3237[1], v3237[2], 0.0, v3237[3], v3237[4]])) - v4806;
                    let v4942 = if v4939 >= v60 { 1.0 } else { 0.0 };
                    let v4951: f64;
                    let v4952: Lanes<6>;
                    if v4942 != 0.0 {
                        v4951 = v4939;
                        v4952 = v4941;
                    } else {
                        v4951 = v60;
                        v4952 = v3227;
                    }
                    let v4957 = Lanes([v4924[0], v4924[1], v4924[2], 0.0, v4924[3], v4924[4]]);
                    let v4958 = (v4952 * v4953) - v4957;
                    let v4959 = ((v4953 * v4951) - v4921) - v3361;
                    let v4965 = (v446 * (v4960 * v4951)) * v3361;
                    let v4966 = ((v4952 * v4960) * v446) * v3361;
                    let v4967 = if v4965 > v60 { 1.0 } else { 0.0 };
                    let v4970: f64;
                    let v4971: Lanes<6>;
                    if v4967 != 0.0 {
                        v4970 = v4965;
                        v4971 = v4966;
                    } else {
                        let v4968 = -v4965;
                        let v4969 = v4966 * v95;
                        v4970 = v4968;
                        v4971 = v4969;
                    }
                    let v4973 = v4958 * v4959;
                    let v4977 = ((v4959 * v4959) + v4970).sqrt();
                    let v4988 = (v4981 * v4951) - (v502 * (v4959 + v4977));
                    let v4989 = (v4952 * v4981) - ((v4958 + (((v4973 + v4973) + v4971) * (v184 / (v236 * v4977)))) * v502);
                    let v4990 = if v4988 <= v4951 { 1.0 } else { 0.0 };
                    let v4991: f64;
                    let v4992: Lanes<6>;
                    if v4990 != 0.0 {
                        v4991 = v4988;
                        v4992 = v4989;
                    } else {
                        v4991 = v4951;
                        v4992 = v4952;
                    }
                    let v4993 = if v4991 < v60 { 1.0 } else { 0.0 };
                    let v4995: f64;
                    let v4996: Lanes<6>;
                    if v4993 != 0.0 {
                        v4995 = v60;
                        v4996 = v3227;
                    } else {
                        let v4994 = if v4991 > v4921 { 1.0 } else { 0.0 };
                        let v5001: f64;
                        let v5002: Lanes<6>;
                        if v4994 != 0.0 {
                            v5001 = v4921;
                            v5002 = v4957;
                        } else {
                            v5001 = v4991;
                            v5002 = v4992;
                        }
                        v4995 = v5001;
                        v4996 = v5002;
                    }
                    let v4997 = v4802 + v4995;
                    let v4998 = v4806 + v4996;
                    let v5000 = if v4997 < v4999 { 1.0 } else { 0.0 };
                    let v5003: f64;
                    let v5004: Lanes<6>;
                    if v5000 != 0.0 {
                        v5003 = v4999;
                        v5004 = v3227;
                    } else {
                        v5003 = v4997;
                        v5004 = v4998;
                    }
                    let v5006 = if v1437 == v5005 { 1.0 } else { 0.0 };
                    let v5007: f64;
                    let v5008: Lanes<6>;
                    if v5006 != 0.0 {
                        v5007 = v4802;
                        v5008 = v4806;
                    } else {
                        v5007 = v5003;
                        v5008 = v5004;
                    }
                    let v5009 = if v5007 < v1438 { 1.0 } else { 0.0 };
                    let v5043: f64;
                    let v5044: Lanes<6>;
                    if v5009 != 0.0 {
                        let v5010 = v1208 * v1206;
                        let v5012 = (v5010 + v5010) - v1217;
                        let v5014 = if v1218 >= v5013 { 1.0 } else { 0.0 };
                        let v5047: f64;
                        let v5048: Lanes<4>;
                        if v5014 != 0.0 {
                            v5047 = v1218;
                            v5048 = v5012;
                        } else {
                            v5047 = v5046;
                            v5048 = v992;
                        }
                        let v5049 = v5047.sqrt();
                        let v5055 = (v1206 - v5049) / v254;
                        let v5056 = (v1208 - (v5048 * (v184 / (v236 * v5049)))) / v254;
                        let v5061 = ((((v1230 - v1233) / v324) * v1235) - v1244) / v1240;
                        let v5062 = if v5055 < v1189 { 1.0 } else { 0.0 };
                        let v5071: f64;
                        let v5072: Lanes<4>;
                        if v5062 != 0.0 {
                            v5071 = v5055;
                            v5072 = v5056;
                        } else {
                            let v5064 = v5061 - v5056;
                            let v5065 = (v1243 - v5055) - v1247;
                            let v5068 = (v446 * v1243) * v1247;
                            let v5069 = (v5061 * v446) * v1247;
                            let v5070 = if v5068 > v60 { 1.0 } else { 0.0 };
                            let v5076: f64;
                            let v5077: Lanes<4>;
                            if v5070 != 0.0 {
                                v5076 = v5068;
                                v5077 = v5069;
                            } else {
                                let v5074 = -v5068;
                                let v5075 = v5069 * v95;
                                v5076 = v5074;
                                v5077 = v5075;
                            }
                            let v5079 = v5064 * v5065;
                            let v5083 = ((v5065 * v5065) + v5076).sqrt();
                            let v5091 = v1243 - (v502 * (v5065 + v5083));
                            let v5092 = v5061 - ((v5064 + (((v5079 + v5079) + v5077) * (v184 / (v236 * v5083)))) * v502);
                            v5071 = v5091;
                            v5072 = v5092;
                        }
                        let v5073 = Lanes([v5072[0], 0.0, v5072[1], 0.0, v5072[2], v5072[3]]);
                        v5043 = v5071;
                        v5044 = v5073;
                    } else {
                        let v5021 = -((v1134 - v5007) - ((v1330 / v254) * v1380));
                        let v5022 = ((Lanes([v1135[0], 0.0, v1135[1], 0.0, v1135[2], v1135[3]])) - v5008) * v95;
                        let v5025 = (v254 * v5021) + v1202;
                        let v5027 = (v5022 * v254) + (Lanes([v1205[0], 0.0, 0.0, 0.0, 0.0, 0.0]));
                        let v5029 = v5027 * v5025;
                        let v5031 = v5021 * v5021;
                        let v5032 = v5022 * v5021;
                        let v5033 = v5032 + v5032;
                        let v5039 = (v5025 * v5025) - (v446 * (v5031 + v1196));
                        let v5040 = (v5029 + v5029) - ((v5033 + (Lanes([v1197[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) * v446);
                        let v5042 = if v5039 >= v5041 { 1.0 } else { 0.0 };
                        let v5094: f64;
                        let v5095: Lanes<6>;
                        if v5042 != 0.0 {
                            v5094 = v5039;
                            v5095 = v5040;
                        } else {
                            v5094 = v5093;
                            v5095 = v3227;
                        }
                        let v5096 = v5094.sqrt();
                        let v5102 = (v5025 - v5096) / v254;
                        let v5103 = (v5027 - (v5095 * (v184 / (v236 * v5096)))) / v254;
                        let v5104 = v5031 / v1196;
                        let v5105 = v1197 * v5104;
                        let v5109 = v5104 / v324;
                        let v5110 = v326 * v5109;
                        let v5117 = v254 / v5021;
                        let v5121 = v151 + v5117;
                        let v5124 = (v5109.ln()) / v5121;
                        let v5127 = ((((((v5033 - (Lanes([v5105[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v1196) - (Lanes([v5110[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v324) * (v184 / v5109)) - (((Lanes([v154[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (((v5022 * v5117) * v95) / v5021)) * v5124)) / v5121;
                        let v5128 = if v5102 < v1189 { 1.0 } else { 0.0 };
                        let v5137: f64;
                        let v5138: Lanes<6>;
                        if v5128 != 0.0 {
                            v5137 = v5102;
                            v5138 = v5103;
                        } else {
                            let v5130 = v5127 - v5103;
                            let v5131 = (v5124 - v5102) - v1247;
                            let v5134 = (v446 * v5124) * v1247;
                            let v5135 = (v5127 * v446) * v1247;
                            let v5136 = if v5134 > v60 { 1.0 } else { 0.0 };
                            let v5141: f64;
                            let v5142: Lanes<6>;
                            if v5136 != 0.0 {
                                v5141 = v5134;
                                v5142 = v5135;
                            } else {
                                let v5139 = -v5134;
                                let v5140 = v5135 * v95;
                                v5141 = v5139;
                                v5142 = v5140;
                            }
                            let v5144 = v5130 * v5131;
                            let v5148 = ((v5131 * v5131) + v5141).sqrt();
                            let v5156 = v5124 - (v502 * (v5131 + v5148));
                            let v5157 = v5127 - ((v5130 + (((v5144 + v5144) + v5142) * (v184 / (v236 * v5148)))) * v502);
                            v5137 = v5156;
                            v5138 = v5157;
                        }
                        v5043 = v5137;
                        v5044 = v5138;
                    }
                    let v5045 = if v5009 != 0.0 && v60 != 0.0 { 1.0 } else { 0.0 };
                    let v5158: f64;
                    let v5159: f64;
                    let v5160: f64;
                    let v5161: Lanes<6>;
                    let v5162: Lanes<6>;
                    let v5163: Lanes<6>;
                    if v5045 != 0.0 {
                        let mut v5164: f64 = 0.0;
                        let mut v5165: f64 = 0.0;
                        let mut v5166: f64 = 0.0;
                        let mut v5167: Lanes<6> = Lanes([0.0; 6]);
                        let mut v5168: Lanes<6> = Lanes([0.0; 6]);
                        v5164 = v60;
                        v5165 = v5043;
                        v5166 = v60;
                        v5167 = v5044;
                        v5168 = v3227;
                        loop {
                            let v5170 = if v5164 < v5169 { 1.0 } else { 0.0 };
                            if v5170 == 0.0 {
                                break;
                            }
                            let v5171 = v151 * v5165;
                            let v5172 = v154 * v5165;
                            let v5175 = (Lanes([v5172[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v5167 * v151);
                            let v5178 = (-v5171).exp();
                            let v5179 = (v5175 * v95) * v5178;
                            let v5180 = if v5165 > v1270 { 1.0 } else { 0.0 };
                            let v5231: f64;
                            let v5232: f64;
                            let v5233: Lanes<6>;
                            let v5234: Lanes<6>;
                            if v5180 != 0.0 {
                                let v5188 = v5171.exp();
                                let v5190 = -v312;
                                let v5195 = v5188 - v89;
                                let v5197 = v326 * v5195;
                                let v5198 = (v5175 * v5188) * v324;
                                let v5203 = (((v5178 + v5171) - v89) + (v324 * v5195)).sqrt();
                                let v5207 = v5190 * v5203;
                                let v5208 = (v315 * v95) * v5203;
                                let v5211 = (Lanes([v5208[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((v5179 + v5175) + ((Lanes([v5197[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + v5198)) * (v184 / (v236 * v5203))) * v5190);
                                let v5212 = v1287 / v5207;
                                let v5220 = v326 * v5188;
                                let v5223 = ((-v5178) + v89) + (v324 * v5188);
                                let v5225 = v5212 * v5223;
                                let v5228 = ((((v5211 * v5212) * v95) / v5207) * v5223) + (((v5179 * v95) + ((Lanes([v5220[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + v5198)) * v5212);
                                v5231 = v5207;
                                v5232 = v5225;
                                v5233 = v5211;
                                v5234 = v5228;
                            } else {
                                let v5230 = if v5165 < v5229 { 1.0 } else { 0.0 };
                                let v5309: f64;
                                let v5310: f64;
                                let v5311: Lanes<6>;
                                let v5312: Lanes<6>;
                                if v5230 != 0.0 {
                                    let v5261 = ((v5178 + v5171) - v89).sqrt();
                                    let v5265 = v312 * v5261;
                                    let v5266 = v315 * v5261;
                                    let v5269 = (Lanes([v5266[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (((v5179 + v5175) * (v184 / (v236 * v5261))) * v312);
                                    let v5270 = v1287 / v5265;
                                    let v5276 = (-v5178) + v89;
                                    let v5277 = v5270 * v5276;
                                    let v5280 = ((((v5269 * v5270) * v95) / v5265) * v5276) + ((v5179 * v95) * v5270);
                                    v5309 = v5265;
                                    v5310 = v5277;
                                    v5311 = v5269;
                                    v5312 = v5280;
                                } else {
                                    let v5281 = v1287 / v151;
                                    let v5285 = v5281.sqrt();
                                    let v5289 = -v5285;
                                    let v5291 = v5289 * v151;
                                    let v5295 = v5291 * v5165;
                                    let v5296 = (((((((v154 * v5281) * v95) / v151) * (v184 / (v236 * v5285))) * v95) * v151) + (v154 * v5289)) * v5165;
                                    let v5299 = (Lanes([v5296[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v5167 * v5291);
                                    let v5302 = (v1287 * v151).sqrt();
                                    let v5306 = -v5302;
                                    let v5307 = ((v154 * v1287) * (v184 / (v236 * v5302))) * v95;
                                    let v5308 = Lanes([v5307[0], 0.0, 0.0, 0.0, 0.0, 0.0]);
                                    v5309 = v5295;
                                    v5310 = v5306;
                                    v5311 = v5299;
                                    v5312 = v5308;
                                }
                                v5231 = v5309;
                                v5232 = v5310;
                                v5233 = v5311;
                                v5234 = v5312;
                            }
                            let v5236 = v5233 * v5231;
                            let v5240 = ((v5231 * v5231) + v5238).sqrt();
                            let v5243 = (v5236 + v5236) * (v184 / (v236 * v5240));
                            let v5244 = v5231 / v5240;
                            let v5249 = v502 * (v89 + v5244);
                            let v5250 = ((v5233 - (v5243 * v5244)) / v5240) * v502;
                            let v5254 = (v5233 + v5243) * v502;
                            let v5256 = (v502 * (v5231 + v5240)) + v5255;
                            let v5257 = if v5256 < v60 { 1.0 } else { 0.0 };
                            let v5313: f64;
                            let v5314: f64;
                            let v5315: Lanes<6>;
                            let v5316: Lanes<6>;
                            if v5257 != 0.0 {
                                v5313 = v60;
                                v5314 = v60;
                                v5315 = v3227;
                                v5316 = v3227;
                            } else {
                                v5313 = v5256;
                                v5314 = v5249;
                                v5315 = v5254;
                                v5316 = v5250;
                            }
                            let v5317 = -v1330;
                            let v5319 = v5315 * v95;
                            let v5320 = (v5317 - v5313) - v69;
                            let v5322 = (v446 * v5317) * v69;
                            let v5323 = if v5322 > v60 { 1.0 } else { 0.0 };
                            let v5325: f64;
                            if v5323 != 0.0 {
                                v5325 = v5322;
                            } else {
                                let v5324 = -v5322;
                                v5325 = v5324;
                            }
                            let v5327 = v5319 * v5320;
                            let v5330 = ((v5320 * v5320) + v5325).sqrt();
                            let v5333 = (v5327 + v5327) * (v184 / (v236 * v5330));
                            let v5334 = v5320 / v5330;
                            let v5339 = v502 * (v89 + v5334);
                            let v5345 = v5317 - (v502 * (v5320 + v5330));
                            let v5346 = ((v5319 + v5333) * v502) * v95;
                            let v5347 = v5232 * v5339;
                            let v5351 = v5314 * v5347;
                            let v5356 = v5346 * v5345;
                            let v5364 = ((((v5345 * v5345) / v254) / v472) / v150) / v287;
                            let v5365 = ((((v5356 + v5356) / v254) / v472) / v150) / v287;
                            let v5366 = v254 * v5364;
                            let v5372 = (v5366 * v5351) / v5345;
                            let v5391 = (v5389 + (v5232 / v1359)) + v5372;
                            let v5393 = ((((-v5165) + (v5231 / v1359)) - v1134) + v5364) / v5391;
                            let v5397 = v5165 - v5393;
                            let v5398 = v5167 - ((((((v5167 * v95) + (v5233 / v1359)) - (Lanes([v1135[0], 0.0, v1135[1], 0.0, v1135[2], v1135[3]]))) + v5365) - (((v5234 / v1359) + (((((v5365 * v254) * v5351) + (((v5316 * v5347) + (((v5234 * v5339) + ((((v5319 - (v5333 * v5334)) / v5330) * v502) * v5232)) * v5314)) * v5366)) - (v5346 * v5372)) / v5345)) * v5393)) / v5391);
                            let v5401 = if ((v5397 - v5165).abs()) < v407 { 1.0 } else { 0.0 };
                            let v5402: f64;
                            if v5401 != 0.0 {
                                v5402 = v5169;
                            } else {
                                v5402 = v5164;
                            }
                            let v5403 = v5402 + v89;
                            v5164 = v5403;
                            v5165 = v5397;
                            v5166 = v5231;
                            v5167 = v5398;
                            v5168 = v5233;
                        }
                        let v5181 = v1134 + v5165;
                        let v5183 = (Lanes([v1135[0], 0.0, v1135[1], 0.0, v1135[2], v1135[3]])) + v5167;
                        let v5186 = v5181 - (v5166 / v1359);
                        let v5187 = v5183 - (v5168 / v1359);
                        v5158 = v5186;
                        v5159 = v5181;
                        v5160 = v5166;
                        v5161 = v5187;
                        v5162 = v5183;
                        v5163 = v5168;
                    } else {
                        let mut v5404: f64 = 0.0;
                        let mut v5405: f64 = 0.0;
                        let mut v5406: f64 = 0.0;
                        let mut v5407: Lanes<6> = Lanes([0.0; 6]);
                        let mut v5408: Lanes<6> = Lanes([0.0; 6]);
                        v5404 = v60;
                        v5405 = v5043;
                        v5406 = v60;
                        v5407 = v5044;
                        v5408 = v3227;
                        loop {
                            let v5409 = if v5404 < v5169 { 1.0 } else { 0.0 };
                            if v5409 == 0.0 {
                                break;
                            }
                            let v5410 = v151 * v5405;
                            let v5411 = v154 * v5405;
                            let v5414 = (Lanes([v5411[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v5407 * v151);
                            let v5417 = (-v5410).exp();
                            let v5418 = (v5414 * v95) * v5417;
                            let v5419 = if v5405 > v1270 { 1.0 } else { 0.0 };
                            let v5470: f64;
                            let v5471: f64;
                            let v5472: Lanes<6>;
                            let v5473: Lanes<6>;
                            if v5419 != 0.0 {
                                let v5427 = v5410.exp();
                                let v5429 = -v312;
                                let v5434 = v5427 - v89;
                                let v5436 = v326 * v5434;
                                let v5437 = (v5414 * v5427) * v324;
                                let v5442 = (((v5417 + v5410) - v89) + (v324 * v5434)).sqrt();
                                let v5446 = v5429 * v5442;
                                let v5447 = (v315 * v95) * v5442;
                                let v5450 = (Lanes([v5447[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((v5418 + v5414) + ((Lanes([v5436[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + v5437)) * (v184 / (v236 * v5442))) * v5429);
                                let v5451 = v1287 / v5446;
                                let v5459 = v326 * v5427;
                                let v5462 = ((-v5417) + v89) + (v324 * v5427);
                                let v5464 = v5451 * v5462;
                                let v5467 = ((((v5450 * v5451) * v95) / v5446) * v5462) + (((v5418 * v95) + ((Lanes([v5459[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + v5437)) * v5451);
                                v5470 = v5446;
                                v5471 = v5464;
                                v5472 = v5450;
                                v5473 = v5467;
                            } else {
                                let v5469 = if v5405 < v5468 { 1.0 } else { 0.0 };
                                let v5548: f64;
                                let v5549: f64;
                                let v5550: Lanes<6>;
                                let v5551: Lanes<6>;
                                if v5469 != 0.0 {
                                    let v5500 = ((v5417 + v5410) - v89).sqrt();
                                    let v5504 = v312 * v5500;
                                    let v5505 = v315 * v5500;
                                    let v5508 = (Lanes([v5505[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (((v5418 + v5414) * (v184 / (v236 * v5500))) * v312);
                                    let v5509 = v1287 / v5504;
                                    let v5515 = (-v5417) + v89;
                                    let v5516 = v5509 * v5515;
                                    let v5519 = ((((v5508 * v5509) * v95) / v5504) * v5515) + ((v5418 * v95) * v5509);
                                    v5548 = v5504;
                                    v5549 = v5516;
                                    v5550 = v5508;
                                    v5551 = v5519;
                                } else {
                                    let v5520 = v1287 / v151;
                                    let v5524 = v5520.sqrt();
                                    let v5528 = -v5524;
                                    let v5530 = v5528 * v151;
                                    let v5534 = v5530 * v5405;
                                    let v5535 = (((((((v154 * v5520) * v95) / v151) * (v184 / (v236 * v5524))) * v95) * v151) + (v154 * v5528)) * v5405;
                                    let v5538 = (Lanes([v5535[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v5407 * v5530);
                                    let v5541 = (v1287 * v151).sqrt();
                                    let v5545 = -v5541;
                                    let v5546 = ((v154 * v1287) * (v184 / (v236 * v5541))) * v95;
                                    let v5547 = Lanes([v5546[0], 0.0, 0.0, 0.0, 0.0, 0.0]);
                                    v5548 = v5534;
                                    v5549 = v5545;
                                    v5550 = v5538;
                                    v5551 = v5547;
                                }
                                v5470 = v5548;
                                v5471 = v5549;
                                v5472 = v5550;
                                v5473 = v5551;
                            }
                            let v5475 = v5472 * v5470;
                            let v5479 = ((v5470 * v5470) + v5477).sqrt();
                            let v5482 = (v5475 + v5475) * (v184 / (v236 * v5479));
                            let v5483 = v5470 / v5479;
                            let v5488 = v502 * (v89 + v5483);
                            let v5489 = ((v5472 - (v5482 * v5483)) / v5479) * v502;
                            let v5493 = (v5472 + v5482) * v502;
                            let v5495 = (v502 * (v5470 + v5479)) + v5494;
                            let v5496 = if v5495 < v60 { 1.0 } else { 0.0 };
                            let v5552: f64;
                            let v5553: f64;
                            let v5554: Lanes<6>;
                            let v5555: Lanes<6>;
                            if v5496 != 0.0 {
                                v5552 = v60;
                                v5553 = v60;
                                v5554 = v3227;
                                v5555 = v3227;
                            } else {
                                v5552 = v5495;
                                v5553 = v5488;
                                v5554 = v5493;
                                v5555 = v5489;
                            }
                            let v5556 = -v1330;
                            let v5558 = v5554 * v95;
                            let v5559 = (v5556 - v5552) - v69;
                            let v5561 = (v446 * v5556) * v69;
                            let v5562 = if v5561 > v60 { 1.0 } else { 0.0 };
                            let v5564: f64;
                            if v5562 != 0.0 {
                                v5564 = v5561;
                            } else {
                                let v5563 = -v5561;
                                v5564 = v5563;
                            }
                            let v5566 = v5558 * v5559;
                            let v5569 = ((v5559 * v5559) + v5564).sqrt();
                            let v5572 = (v5566 + v5566) * (v184 / (v236 * v5569));
                            let v5573 = v5559 / v5569;
                            let v5578 = v502 * (v89 + v5573);
                            let v5584 = v5556 - (v502 * (v5559 + v5569));
                            let v5585 = ((v5558 + v5572) * v502) * v95;
                            let v5586 = v5471 * v5578;
                            let v5590 = v5553 * v5586;
                            let v5595 = v5585 * v5584;
                            let v5603 = ((((v5584 * v5584) / v254) / v472) / v150) / v287;
                            let v5604 = ((((v5595 + v5595) / v254) / v472) / v150) / v287;
                            let v5605 = v254 * v5603;
                            let v5611 = (v5605 * v5590) / v5584;
                            let v5640 = ((v5634 + (v5471 / v1359)) + (v5471 * v1380)) + v5611;
                            let v5642 = (((((v5007 - v5405) + (v5470 / v1359)) + ((v5470 + (v1330 / v254)) * v1380)) - v1134) + v5603) / v5640;
                            let v5646 = v5405 - v5642;
                            let v5647 = v5407 - (((((((v5008 - v5407) + (v5472 / v1359)) + (v5472 * v1380)) - (Lanes([v1135[0], 0.0, v1135[1], 0.0, v1135[2], v1135[3]]))) + v5604) - ((((v5473 / v1359) + (v5473 * v1380)) + (((((v5604 * v254) * v5590) + (((v5555 * v5586) + (((v5473 * v5578) + ((((v5558 - (v5572 * v5573)) / v5569) * v502) * v5471)) * v5553)) * v5605)) - (v5585 * v5611)) / v5584)) * v5642)) / v5640);
                            let v5650 = if ((v5646 - v5405).abs()) < v407 { 1.0 } else { 0.0 };
                            let v5651: f64;
                            if v5650 != 0.0 {
                                v5651 = v5169;
                            } else {
                                v5651 = v5404;
                            }
                            let v5652 = v5651 + v89;
                            v5404 = v5652;
                            v5405 = v5646;
                            v5406 = v5470;
                            v5407 = v5647;
                            v5408 = v5472;
                        }
                        let v5420 = v1134 + v5405;
                        let v5422 = (Lanes([v1135[0], 0.0, v1135[1], 0.0, v1135[2], v1135[3]])) + v5407;
                        let v5425 = v5420 - (v5406 / v1359);
                        let v5426 = v5422 - (v5408 / v1359);
                        v5158 = v5425;
                        v5159 = v5420;
                        v5160 = v5406;
                        v5161 = v5426;
                        v5162 = v5422;
                        v5163 = v5408;
                    }
                    v4943 = v5007;
                    v4944 = v5158;
                    v4945 = v5159;
                    v4946 = v5160;
                    v4947 = v5008;
                    v4948 = v5161;
                    v4949 = v5162;
                    v4950 = v5163;
                }
                v4929 = v4943;
                v4930 = v4944;
                v4931 = v4945;
                v4932 = v4946;
                v4933 = v4947;
                v4934 = v4948;
                v4935 = v4949;
                v4936 = v4950;
            }
            let v4937 = if v4821 < v407 { 1.0 } else { 0.0 };
            let v5657: f64;
            let v5658: f64;
            let v5659: f64;
            let v5660: f64;
            let v5661: Lanes<6>;
            let v5662: Lanes<6>;
            let v5663: Lanes<6>;
            let v5664: Lanes<6>;
            if v4937 != 0.0 {
                v5657 = v4802;
                v5658 = v4803;
                v5659 = v4805;
                v5660 = v4804;
                v5661 = v4806;
                v5662 = v4807;
                v5663 = v4809;
                v5664 = v4808;
            } else {
                let v5653 = v4931 - v1134;
                let v5655 = v4935 - (Lanes([v1135[0], 0.0, v1135[1], 0.0, v1135[2], v1135[3]]));
                let v5656 = if v4930 < v4929 { 1.0 } else { 0.0 };
                let v5666: f64;
                let v5667: Lanes<6>;
                if v5656 != 0.0 {
                    v5666 = v4930;
                    v5667 = v4934;
                } else {
                    v5666 = v4929;
                    v5667 = v4933;
                }
                v5657 = v4929;
                v5658 = v4930;
                v5659 = v5653;
                v5660 = v5666;
                v5661 = v4933;
                v5662 = v4934;
                v5663 = v5655;
                v5664 = v5667;
            }
            let v5665 = if v1437 < v60 { 1.0 } else { 0.0 };
            let v5668: f64;
            if v5665 != 0.0 {
                v5668 = v89;
            } else {
                v5668 = v60;
            }
            let mut v5669: f64 = 0.0;
            let mut v5670: f64 = 0.0;
            let mut v5671: f64 = 0.0;
            let mut v5672: f64 = 0.0;
            let mut v5673: f64 = 0.0;
            let mut v5674: f64 = 0.0;
            let mut v5675: f64 = 0.0;
            let mut v5676: f64 = 0.0;
            let mut v5677: f64 = 0.0;
            let mut v5678: f64 = 0.0;
            let mut v5679: f64 = 0.0;
            let mut v5680: f64 = 0.0;
            let mut v5681: Lanes<6> = Lanes([0.0; 6]);
            let mut v5682: Lanes<6> = Lanes([0.0; 6]);
            let mut v5683: Lanes<6> = Lanes([0.0; 6]);
            let mut v5684: Lanes<6> = Lanes([0.0; 6]);
            let mut v5685: Lanes<6> = Lanes([0.0; 6]);
            let mut v5686: Lanes<6> = Lanes([0.0; 6]);
            let mut v5687: Lanes<6> = Lanes([0.0; 6]);
            let mut v5688: Lanes<6> = Lanes([0.0; 6]);
            let mut v5689: Lanes<6> = Lanes([0.0; 6]);
            v5669 = v89;
            v5670 = v5659;
            v5671 = v5657;
            v5672 = v5660;
            v5673 = v5668;
            v5674 = v4800;
            v5675 = v5658;
            v5676 = v60;
            v5677 = v60;
            v5678 = v60;
            v5679 = v4932;
            v5680 = v60;
            v5681 = v5663;
            v5682 = v5661;
            v5683 = v5664;
            v5684 = v5662;
            v5685 = v3227;
            v5686 = v3227;
            v5687 = v3227;
            v5688 = v4936;
            v5689 = v3227;
            loop {
                let v5690 = if v5669 <= v5169 { 1.0 } else { 0.0 };
                if v5690 == 0.0 {
                    break;
                }
                let v5691 = v151 * v5670;
                let v5692 = v154 * v5670;
                let v5695 = (Lanes([v5692[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v5681 * v151);
                let v5698 = (-v5691).exp();
                let v5699 = (v5695 * v95) * v5698;
                let v5701 = if v5670 < v5700 { 1.0 } else { 0.0 };
                let v5742: f64;
                let v5743: f64;
                let v5744: Lanes<6>;
                let v5745: Lanes<6>;
                if v5701 != 0.0 {
                    let v5703 = v5691.exp();
                    let v5708 = v5703 - v89;
                    let v5710 = v326 * v5708;
                    let v5711 = (v5695 * v5703) * v324;
                    let v5716 = (((v5698 + v5691) - v89) + (v324 * v5708)).sqrt();
                    let v5720 = v312 * v5716;
                    let v5721 = v315 * v5716;
                    let v5724 = (Lanes([v5721[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((v5699 + v5695) + ((Lanes([v5710[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + v5711)) * (v184 / (v236 * v5716))) * v312);
                    let v5729 = v326 * v5703;
                    let v5736 = (v1287 * (((-v5698) + v89) + (v324 * v5703))) / v5720;
                    let v5739 = ((((v5699 * v95) + ((Lanes([v5729[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + v5711)) * v1287) - (v5724 * v5736)) / v5720;
                    v5742 = v5720;
                    v5743 = v5736;
                    v5744 = v5724;
                    v5745 = v5739;
                } else {
                    let v5741 = if v5670 > v5740 { 1.0 } else { 0.0 };
                    let v5831: f64;
                    let v5832: f64;
                    let v5833: Lanes<6>;
                    let v5834: Lanes<6>;
                    if v5741 != 0.0 {
                        let v5770 = v5691.exp();
                        let v5771 = v5695 * v5770;
                        let v5772 = -v312;
                        let v5779 = (v5770 - v5691) - v89;
                        let v5781 = v326 * v5779;
                        let v5787 = (((v5698 + v5691) - v89) + (v324 * v5779)).sqrt();
                        let v5791 = v5772 * v5787;
                        let v5792 = (v315 * v95) * v5787;
                        let v5795 = (Lanes([v5792[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((v5699 + v5695) + ((Lanes([v5781[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v5771 - v5695) * v324))) * (v184 / (v236 * v5787))) * v5772);
                        let v5799 = v5770 - v89;
                        let v5801 = v326 * v5799;
                        let v5809 = (v1287 * (((-v5698) + v89) + (v324 * v5799))) / v5791;
                        let v5812 = ((((v5699 * v95) + ((Lanes([v5801[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v5771 * v324))) * v1287) - (v5795 * v5809)) / v5791;
                        v5831 = v5791;
                        v5832 = v5809;
                        v5833 = v5795;
                        v5834 = v5812;
                    } else {
                        let v5813 = -v312;
                        let v5814 = v315 * v95;
                        let v5816 = v5814 * v5691;
                        let v5821 = (v5813 * v5691) / v5820;
                        let v5822 = ((Lanes([v5816[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v5695 * v5813)) / v5820;
                        let v5828 = (v5813 * v151) / v5827;
                        let v5829 = ((v5814 * v151) + (v154 * v5813)) / v5827;
                        let v5830 = Lanes([v5829[0], 0.0, 0.0, 0.0, 0.0, 0.0]);
                        v5831 = v5821;
                        v5832 = v5828;
                        v5833 = v5822;
                        v5834 = v5830;
                    }
                    v5742 = v5831;
                    v5743 = v5832;
                    v5744 = v5833;
                    v5745 = v5834;
                }
                let v5753 = ((v5670 - (v5742 / v1359)) + v353) + v1031;
                let v5755 = ((v5681 - (v5744 / v1359)) + (Lanes([0.0, 0.0, v355[0], 0.0, v355[1], v355[2]]))) + (Lanes([v1032[0], 0.0, 0.0, 0.0, 0.0, 0.0]));
                let v5758 = v89 - (v5743 / v1359);
                let v5759 = (v5745 / v1359) * v95;
                let v5760 = v5671 - v5672;
                let v5762 = v151 * v5760;
                let v5763 = v154 * v5760;
                let v5766 = (Lanes([v5763[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v5682 - v5683) * v151);
                let v5767 = -v5762;
                let v5768 = v5766 * v95;
                let v5769 = if v5767 >= v3883 { 1.0 } else { 0.0 };
                let v5841: f64;
                let v5842: f64;
                let v5843: Lanes<6>;
                let v5844: Lanes<6>;
                if v5769 != 0.0 {
                    let v5837 = v3952 * ((v89 + v5767) - v3883);
                    let v5838 = v5768 * v3952;
                    v5841 = v5837;
                    v5842 = v3952;
                    v5843 = v5838;
                    v5844 = v3227;
                } else {
                    let v5839 = v5767.exp();
                    let v5840 = v5768 * v5839;
                    v5841 = v5839;
                    v5842 = v5839;
                    v5843 = v5840;
                    v5844 = v5840;
                }
                let v5846 = if v5760 < v5845 { 1.0 } else { 0.0 };
                let v5881: f64;
                let v5882: f64;
                let v5883: f64;
                let v5884: f64;
                let v5885: f64;
                let v5886: f64;
                let v5887: Lanes<6>;
                let v5888: Lanes<6>;
                let v5889: Lanes<6>;
                let v5890: Lanes<6>;
                let v5891: Lanes<6>;
                let v5892: Lanes<6>;
                if v5846 != 0.0 {
                    let v5850 = ((v5841 + v5762) - v89).sqrt();
                    let v5853 = (v5843 + v5766) * (v184 / (v236 * v5850));
                    let v5854 = v307 * v5850;
                    let v5855 = v308 * v5850;
                    let v5858 = (Lanes([v5855[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v5853 * v307);
                    let v5859 = v307 * v151;
                    let v5865 = (-v5842) + v89;
                    let v5867 = ((v308 * v151) + (v154 * v307)) * v5865;
                    let v5871 = v254 * v5850;
                    let v5873 = (v5859 * v5865) / v5871;
                    let v5876 = (((Lanes([v5867[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v5844 * v95) * v5859)) - ((v5853 * v254) * v5873)) / v5871;
                    let v5877 = -v5873;
                    let v5878 = v5876 * v95;
                    v5881 = v60;
                    v5882 = v5854;
                    v5883 = v60;
                    v5884 = v5873;
                    v5885 = v60;
                    v5886 = v5877;
                    v5887 = v3227;
                    v5888 = v5858;
                    v5889 = v3227;
                    v5890 = v5876;
                    v5891 = v3227;
                    v5892 = v5878;
                } else {
                    let v5880 = if v5760 > v5879 { 1.0 } else { 0.0 };
                    let v6076: f64;
                    let v6077: f64;
                    let v6078: f64;
                    let v6079: f64;
                    let v6080: f64;
                    let v6081: f64;
                    let v6082: Lanes<6>;
                    let v6083: Lanes<6>;
                    let v6084: Lanes<6>;
                    let v6085: Lanes<6>;
                    let v6086: Lanes<6>;
                    let v6087: Lanes<6>;
                    if v5880 != 0.0 {
                        let v5906 = ((v5841 + v5762) - v89).sqrt();
                        let v5909 = (v5843 + v5766) * (v184 / (v236 * v5906));
                        let v5910 = -v307;
                        let v5911 = v308 * v95;
                        let v5912 = v5910 * v5906;
                        let v5913 = v5911 * v5906;
                        let v5916 = (Lanes([v5913[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v5909 * v5910);
                        let v5917 = v5910 * v151;
                        let v5923 = (-v5842) + v89;
                        let v5925 = ((v5911 * v151) + (v154 * v5910)) * v5923;
                        let v5929 = v254 * v5906;
                        let v5931 = (v5917 * v5923) / v5929;
                        let v5934 = (((Lanes([v5925[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v5844 * v95) * v5917)) - ((v5909 * v254) * v5931)) / v5929;
                        let v5935 = -v5931;
                        let v5936 = v5934 * v95;
                        let v5937 = v5762.exp();
                        let v5938 = v5766 * v5937;
                        let v5939 = v5672 - v4921;
                        let v5943 = v154 * v5939;
                        let v5947 = (v151 * v5939).exp();
                        let v5948 = ((Lanes([v5943[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v5683 - (Lanes([v4924[0], v4924[1], v4924[2], 0.0, v4924[3], v4924[4]]))) * v151)) * v5947;
                        let v5950 = v5916 * v5912;
                        let v5952 = v307 * v307;
                        let v5953 = v308 * v307;
                        let v5954 = v5953 + v5953;
                        let v5955 = (v5912 * v5912) / v5952;
                        let v5956 = v5954 * v5955;
                        let v5960 = v254 * v318;
                        let v5962 = v5960 * v5947;
                        let v5963 = (v320 * v254) * v5947;
                        let v5969 = (v5937 - v5762) - v89;
                        let v5976 = (v5955 + (v5962 * v5969)).sqrt();
                        let v5979 = ((((v5950 + v5950) - (Lanes([v5956[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v5952) + ((((Lanes([v5963[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v5948 * v5960)) * v5969) + ((v5938 - v5766) * v5962))) * (v184 / (v236 * v5976));
                        let v5980 = v254 * v5912;
                        let v5981 = v5916 * v254;
                        let v5986 = (v5980 * v5931) / v5952;
                        let v5987 = v5954 * v5986;
                        let v5991 = v254 * v151;
                        let v5993 = v5991 * v318;
                        let v5997 = v5993 * v5947;
                        let v5998 = (((v154 * v254) * v318) + (v320 * v5991)) * v5947;
                        let v6001 = (Lanes([v5998[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v5948 * v5993);
                        let v6002 = v5937 - v89;
                        let v6009 = v254 * v5976;
                        let v6010 = v5979 * v254;
                        let v6011 = (v5986 + (v5997 * v6002)) / v6009;
                        let v6019 = (v5980 * v5935) / v5952;
                        let v6020 = v5954 * v6019;
                        let v6030 = (v6019 - (v5997 * v5762)) / v6009;
                        let v6035 = v5911 * v5976;
                        let v6039 = (v5910 * v5976) - v5912;
                        let v6040 = ((Lanes([v6035[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v5979 * v5910)) - v5916;
                        let v6042 = v5911 * v6011;
                        let v6046 = (v5910 * v6011) - v5931;
                        let v6047 = ((Lanes([v6042[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((((((v5981 * v5931) + (v5934 * v5980)) - (Lanes([v5987[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v5952) + ((v6001 * v6002) + (v5938 * v5997))) - (v6010 * v6011)) / v6009) * v5910)) - v5934;
                        let v6049 = v5911 * v6030;
                        let v6053 = (v5910 * v6030) - v5935;
                        let v6054 = ((Lanes([v6049[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((((((v5981 * v5935) + (v5936 * v5980)) - (Lanes([v6020[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v5952) - ((v6001 * v5762) + (v5766 * v5997))) - (v6010 * v6030)) / v6009) * v5910)) - v5936;
                        v6076 = v6039;
                        v6077 = v5912;
                        v6078 = v6046;
                        v6079 = v5931;
                        v6080 = v6053;
                        v6081 = v5935;
                        v6082 = v6040;
                        v6083 = v5916;
                        v6084 = v6047;
                        v6085 = v5934;
                        v6086 = v6054;
                        v6087 = v5936;
                    } else {
                        let v6055 = -v307;
                        let v6056 = v308 * v95;
                        let v6058 = v6056 * v5762;
                        let v6063 = (v6055 * v5762) / v6062;
                        let v6064 = ((Lanes([v6058[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v5766 * v6055)) / v6062;
                        let v6070 = (v6055 * v151) / v6069;
                        let v6071 = ((v6056 * v151) + (v154 * v6055)) / v6069;
                        let v6072 = -v6070;
                        let v6073 = v6071 * v95;
                        let v6074 = Lanes([v6071[0], 0.0, 0.0, 0.0, 0.0, 0.0]);
                        let v6075 = Lanes([v6073[0], 0.0, 0.0, 0.0, 0.0, 0.0]);
                        v6076 = v60;
                        v6077 = v6063;
                        v6078 = v60;
                        v6079 = v6070;
                        v6080 = v60;
                        v6081 = v6072;
                        v6082 = v3227;
                        v6083 = v6064;
                        v6084 = v3227;
                        v6085 = v6074;
                        v6086 = v3227;
                        v6087 = v6075;
                    }
                    v5881 = v6076;
                    v5882 = v6077;
                    v5883 = v6078;
                    v5884 = v6079;
                    v5885 = v6080;
                    v5886 = v6081;
                    v5887 = v6082;
                    v5888 = v6083;
                    v5889 = v6084;
                    v5890 = v6085;
                    v5891 = v6086;
                    v5892 = v6087;
                }
                let v5893 = v5753 - v5672;
                let v5895 = v151 * v5893;
                let v5896 = v154 * v5893;
                let v5899 = (Lanes([v5896[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v5755 - v5683) * v151);
                let v5900 = -v5895;
                let v5901 = v5899 * v95;
                let v5902 = if v5900 >= v3883 { 1.0 } else { 0.0 };
                let v6094: f64;
                let v6095: f64;
                let v6096: Lanes<6>;
                let v6097: Lanes<6>;
                if v5902 != 0.0 {
                    let v6090 = v3952 * ((v89 + v5900) - v3883);
                    let v6091 = v5901 * v3952;
                    v6094 = v6090;
                    v6095 = v3952;
                    v6096 = v6091;
                    v6097 = v3227;
                } else {
                    let v6092 = v5900.exp();
                    let v6093 = v5901 * v6092;
                    v6094 = v6092;
                    v6095 = v6092;
                    v6096 = v6093;
                    v6097 = v6093;
                }
                let v6099 = if v5893 < v6098 { 1.0 } else { 0.0 };
                let v6134: f64;
                let v6135: f64;
                let v6136: f64;
                let v6137: f64;
                let v6138: f64;
                let v6139: f64;
                let v6140: Lanes<6>;
                let v6141: Lanes<6>;
                let v6142: Lanes<6>;
                let v6143: Lanes<6>;
                let v6144: Lanes<6>;
                let v6145: Lanes<6>;
                if v6099 != 0.0 {
                    let v6103 = ((v6094 + v5895) - v89).sqrt();
                    let v6106 = (v6096 + v5899) * (v184 / (v236 * v6103));
                    let v6107 = v307 * v6103;
                    let v6108 = v308 * v6103;
                    let v6111 = (Lanes([v6108[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v6106 * v307);
                    let v6112 = v307 * v151;
                    let v6118 = (-v6095) + v89;
                    let v6120 = ((v308 * v151) + (v154 * v307)) * v6118;
                    let v6124 = v254 * v6103;
                    let v6126 = (v6112 * v6118) / v6124;
                    let v6129 = (((Lanes([v6120[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v6097 * v95) * v6112)) - ((v6106 * v254) * v6126)) / v6124;
                    let v6130 = -v6126;
                    let v6131 = v6129 * v95;
                    v6134 = v60;
                    v6135 = v6107;
                    v6136 = v60;
                    v6137 = v6130;
                    v6138 = v60;
                    v6139 = v6126;
                    v6140 = v3227;
                    v6141 = v6111;
                    v6142 = v3227;
                    v6143 = v6131;
                    v6144 = v3227;
                    v6145 = v6129;
                } else {
                    let v6133 = if v5893 > v6132 { 1.0 } else { 0.0 };
                    let v6320: f64;
                    let v6321: f64;
                    let v6322: f64;
                    let v6323: f64;
                    let v6324: f64;
                    let v6325: f64;
                    let v6326: Lanes<6>;
                    let v6327: Lanes<6>;
                    let v6328: Lanes<6>;
                    let v6329: Lanes<6>;
                    let v6330: Lanes<6>;
                    let v6331: Lanes<6>;
                    if v6133 != 0.0 {
                        let v6150 = ((v6094 + v5895) - v89).sqrt();
                        let v6153 = (v6096 + v5899) * (v184 / (v236 * v6150));
                        let v6154 = -v307;
                        let v6155 = v308 * v95;
                        let v6156 = v6154 * v6150;
                        let v6157 = v6155 * v6150;
                        let v6160 = (Lanes([v6157[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v6153 * v6154);
                        let v6161 = v6154 * v151;
                        let v6167 = (-v6095) + v89;
                        let v6169 = ((v6155 * v151) + (v154 * v6154)) * v6167;
                        let v6173 = v254 * v6150;
                        let v6175 = (v6161 * v6167) / v6173;
                        let v6178 = (((Lanes([v6169[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v6097 * v95) * v6161)) - ((v6153 * v254) * v6175)) / v6173;
                        let v6179 = -v6175;
                        let v6180 = v6178 * v95;
                        let v6181 = v5895.exp();
                        let v6182 = v5899 * v6181;
                        let v6183 = v5672 - v4921;
                        let v6187 = v154 * v6183;
                        let v6191 = (v151 * v6183).exp();
                        let v6192 = ((Lanes([v6187[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v5683 - (Lanes([v4924[0], v4924[1], v4924[2], 0.0, v4924[3], v4924[4]]))) * v151)) * v6191;
                        let v6194 = v6160 * v6156;
                        let v6196 = v307 * v307;
                        let v6197 = v308 * v307;
                        let v6198 = v6197 + v6197;
                        let v6199 = (v6156 * v6156) / v6196;
                        let v6200 = v6198 * v6199;
                        let v6204 = v254 * v318;
                        let v6206 = v6204 * v6191;
                        let v6207 = (v320 * v254) * v6191;
                        let v6213 = (v6181 - v5895) - v89;
                        let v6220 = (v6199 + (v6206 * v6213)).sqrt();
                        let v6223 = ((((v6194 + v6194) - (Lanes([v6200[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v6196) + ((((Lanes([v6207[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v6192 * v6204)) * v6213) + ((v6182 - v5899) * v6206))) * (v184 / (v236 * v6220));
                        let v6224 = v254 * v6156;
                        let v6225 = v6160 * v254;
                        let v6230 = (v6224 * v6175) / v6196;
                        let v6231 = v6198 * v6230;
                        let v6235 = v254 * v151;
                        let v6237 = v6235 * v318;
                        let v6241 = v6237 * v6191;
                        let v6242 = (((v154 * v254) * v318) + (v320 * v6235)) * v6191;
                        let v6245 = (Lanes([v6242[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v6192 * v6237);
                        let v6246 = v6181 - v89;
                        let v6253 = v254 * v6220;
                        let v6254 = v6223 * v254;
                        let v6255 = (v6230 + (v6241 * v6246)) / v6253;
                        let v6263 = (v6224 * v6179) / v6196;
                        let v6264 = v6198 * v6263;
                        let v6274 = (v6263 - (v6241 * v5895)) / v6253;
                        let v6279 = v6155 * v6220;
                        let v6283 = (v6154 * v6220) - v6156;
                        let v6284 = ((Lanes([v6279[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v6223 * v6154)) - v6160;
                        let v6286 = v6155 * v6255;
                        let v6290 = (v6154 * v6255) - v6175;
                        let v6291 = ((Lanes([v6286[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((((((v6225 * v6175) + (v6178 * v6224)) - (Lanes([v6231[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v6196) + ((v6245 * v6246) + (v6182 * v6241))) - (v6254 * v6255)) / v6253) * v6154)) - v6178;
                        let v6293 = v6155 * v6274;
                        let v6297 = (v6154 * v6274) - v6179;
                        let v6298 = ((Lanes([v6293[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((((((v6225 * v6179) + (v6180 * v6224)) - (Lanes([v6264[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v6196) - ((v6245 * v5895) + (v5899 * v6241))) - (v6254 * v6274)) / v6253) * v6154)) - v6180;
                        v6320 = v6283;
                        v6321 = v6156;
                        v6322 = v6297;
                        v6323 = v6179;
                        v6324 = v6290;
                        v6325 = v6175;
                        v6326 = v6284;
                        v6327 = v6160;
                        v6328 = v6298;
                        v6329 = v6180;
                        v6330 = v6291;
                        v6331 = v6178;
                    } else {
                        let v6299 = -v307;
                        let v6300 = v308 * v95;
                        let v6302 = v6300 * v5895;
                        let v6307 = (v6299 * v5895) / v6306;
                        let v6308 = ((Lanes([v6302[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v5899 * v6299)) / v6306;
                        let v6314 = (v6299 * v151) / v6313;
                        let v6315 = ((v6300 * v151) + (v154 * v6299)) / v6313;
                        let v6316 = -v6314;
                        let v6317 = v6315 * v95;
                        let v6318 = Lanes([v6317[0], 0.0, 0.0, 0.0, 0.0, 0.0]);
                        let v6319 = Lanes([v6315[0], 0.0, 0.0, 0.0, 0.0, 0.0]);
                        v6320 = v60;
                        v6321 = v6307;
                        v6322 = v60;
                        v6323 = v6316;
                        v6324 = v60;
                        v6325 = v6314;
                        v6326 = v3227;
                        v6327 = v6308;
                        v6328 = v3227;
                        v6329 = v6318;
                        v6330 = v3227;
                        v6331 = v6319;
                    }
                    v6134 = v6320;
                    v6135 = v6321;
                    v6136 = v6322;
                    v6137 = v6323;
                    v6138 = v6324;
                    v6139 = v6325;
                    v6140 = v6326;
                    v6141 = v6327;
                    v6142 = v6328;
                    v6143 = v6329;
                    v6144 = v6330;
                    v6145 = v6331;
                }
                let v6146 = if v5673 == v89 { 1.0 } else { 0.0 };
                let v6390: f64;
                let v6391: f64;
                let v6392: f64;
                let v6393: f64;
                let v6394: f64;
                let v6395: f64;
                let v6396: Lanes<6>;
                let v6397: Lanes<6>;
                let v6398: Lanes<6>;
                if v6146 != 0.0 {
                    v6390 = v5169;
                    v6391 = v5670;
                    v6392 = v5671;
                    v6393 = v5672;
                    v6394 = v5673;
                    v6395 = v5669;
                    v6396 = v5681;
                    v6397 = v5682;
                    v6398 = v5683;
                } else {
                    let v6345 = (((((v5742 + v5881) + v5882) + v6134) + v6135) + v3228) / v627;
                    let v6346 = v630 * v6345;
                    let v6350 = (v5671 - v1420) - v6345;
                    let v6351 = (v5682 - (Lanes([v1423[0], v1423[1], v1423[2], 0.0, v1423[3], v1423[4]]))) - (((((((v5744 + v5887) + v5888) + v6140) + v6141) + v3236) - (Lanes([0.0, v6346[0], v6346[1], 0.0, v6346[2], v6346[3]]))) / v627);
                    let v6354 = (v5883 + v5884) / v627;
                    let v6355 = v630 * v6354;
                    let v6359 = v89 - v6354;
                    let v6360 = (((v5889 + v5890) - (Lanes([0.0, v6355[0], v6355[1], 0.0, v6355[2], v6355[3]]))) / v627) * v95;
                    let v6369 = (-(((v5885 + v5886) + v6136) + v6137)) / v627;
                    let v6370 = v630 * v6369;
                    let v6373 = (((((v5891 + v5892) + v6142) + v6143) * v95) - (Lanes([0.0, v6370[0], v6370[1], 0.0, v6370[2], v6370[3]]))) / v627;
                    let v6374 = v6138 + v6139;
                    let v6384 = (-(v5743 + (v6374 * v5758))) / v627;
                    let v6385 = v630 * v6384;
                    let v6388 = (((v5745 + (((v6144 + v6145) * v5758) + (v5759 * v6374))) * v95) - (Lanes([0.0, v6385[0], v6385[1], 0.0, v6385[2], v6385[3]]))) / v627;
                    let v6389 = if v5742 <= v3776 { 1.0 } else { 0.0 };
                    if v6389 != 0.0 {
                    } else {
                        let v6400 = if v5742 <= v3777 { 1.0 } else { 0.0 };
                    }
                    let v6403 = (-v1435) / v1330;
                    let v6404 = (v1443 * v95) / v1330;
                    let v6409 = (-(v6403 * v4515)).exp();
                    let v6411 = v89 + v6409;
                    let v6412 = v89 / v6411;
                    let v6420 = (((((((v6404 * v4515) * v95) * v6409) * v6412) * v95) / v6411) * v3778) * v95;
                    let v6424 = (v5882 + (-(v1330 + (v6412 * v3778)))) / v4535;
                    let v6425 = (v5888 + (Lanes([v6420[0], v6420[1], v6420[2], 0.0, v6420[3], v6420[4]]))) / v4535;
                    let v6426 = v5884 / v4535;
                    let v6427 = v5890 / v4535;
                    let v6428 = v5886 / v4535;
                    let v6429 = v5892 / v4535;
                    let v6430 = v60 / v4535;
                    let v6435 = (-(v6403 * v4543)).exp();
                    let v6437 = v89 + v6435;
                    let v6438 = v89 / v6437;
                    let v6443 = ((((((v6404 * v4543) * v95) * v6435) * v6438) * v95) / v6437) * v3778;
                    let v6447 = (v6135 + (v6438 * v3778)) / v4535;
                    let v6448 = (v6141 + (Lanes([v6443[0], v6443[1], v6443[2], 0.0, v6443[3], v6443[4]]))) / v4535;
                    let v6449 = v6137 / v4535;
                    let v6450 = v6143 / v4535;
                    let v6455 = (v6139 * v5758) / v4535;
                    let v6456 = ((v6145 * v5758) + (v5759 * v6139)) / v4535;
                    let v6457 = v6359 * v6428;
                    let v6460 = (v6360 * v6428) + (v6429 * v6359);
                    let v6465 = v6359 * v6430;
                    let v6466 = v6360 * v6430;
                    let v6473 = v6369 * v6426;
                    let v6476 = (v6373 * v6426) + (v6427 * v6369);
                    let v6483 = v6384 * v6426;
                    let v6486 = (v6388 * v6426) + (v6427 * v6384);
                    let v6491 = (((v6457 * v6455) - (v6465 * v6449)) - (v6473 * v6455)) + (v6483 * v6449);
                    let v6492 = ((((v6460 * v6455) + (v6456 * v6457)) - ((v6466 * v6449) + (v6450 * v6465))) - ((v6476 * v6455) + (v6456 * v6473))) + ((v6486 * v6449) + (v6450 * v6483));
                    let v6493 = if v6491 > v60 { 1.0 } else { 0.0 };
                    let v6504: f64;
                    let v6505: Lanes<6>;
                    if v6493 != 0.0 {
                        let v6494 = v6491 + v418;
                        let v6495 = v89 / v6494;
                        let v6498 = ((v6492 * v6495) * v95) / v6494;
                        v6504 = v6495;
                        v6505 = v6498;
                    } else {
                        let v6499 = v6491 - v418;
                        let v6500 = v89 / v6499;
                        let v6503 = ((v6492 * v6500) * v95) / v6499;
                        v6504 = v6500;
                        v6505 = v6503;
                    }
                    let v6512 = (v6428 * v6455) - (v6430 * v6449);
                    let v6522 = (v6384 * v6449) - (v6369 * v6455);
                    let v6530 = (v6369 * v6430) - (v6384 * v6428);
                    let v6532 = -v6426;
                    let v6534 = v6532 * v6455;
                    let v6538 = v6359 * v6455;
                    let v6542 = v6483 - v6465;
                    let v6544 = v6426 * v6449;
                    let v6548 = -v6359;
                    let v6550 = v6548 * v6449;
                    let v6554 = v6457 - v6473;
                    let v6556 = -v6504;
                    let v6557 = v6505 * v95;
                    let v6572 = ((v6512 * v6350) + (v6522 * v6424)) + (v6530 * v6447);
                    let v6574 = v6556 * v6572;
                    let v6577 = (v6557 * v6572) + ((((((((v6429 * v6455) + (v6456 * v6428)) - (v6450 * v6430)) * v6350) + (v6351 * v6512)) + (((((v6388 * v6449) + (v6450 * v6384)) - ((v6373 * v6455) + (v6456 * v6369))) * v6424) + (v6425 * v6522))) + ((((v6373 * v6430) - ((v6388 * v6428) + (v6429 * v6384))) * v6447) + (v6448 * v6530))) * v6556);
                    let v6592 = ((v6534 * v6350) + (v6538 * v6424)) + (v6542 * v6447);
                    let v6594 = v6556 * v6592;
                    let v6597 = (v6557 * v6592) + ((((((((v6427 * v95) * v6455) + (v6456 * v6532)) * v6350) + (v6351 * v6534)) + ((((v6360 * v6455) + (v6456 * v6359)) * v6424) + (v6425 * v6538))) + (((v6486 - v6466) * v6447) + (v6448 * v6542))) * v6556);
                    let v6612 = ((v6544 * v6350) + (v6550 * v6424)) + (v6554 * v6447);
                    let v6614 = v6556 * v6612;
                    let v6617 = (v6557 * v6612) + (((((((v6427 * v6449) + (v6450 * v6426)) * v6350) + (v6351 * v6544)) + (((((v6360 * v95) * v6449) + (v6450 * v6548)) * v6424) + (v6425 * v6550))) + (((v6460 - v6476) * v6447) + (v6448 * v6554))) * v6556);
                    let v6618 = v6574.abs();
                    let v6622 = v6577 * ((v236 * (if v6574 >= v4732 { 1.0 } else { 0.0 })) - v184);
                    let v6623 = v6594.abs();
                    let v6627 = v6597 * ((v236 * (if v6594 >= v4732 { 1.0 } else { 0.0 })) - v184);
                    let v6628 = if v6618 < v6623 { 1.0 } else { 0.0 };
                    let v6629: f64;
                    let v6630: Lanes<6>;
                    if v6628 != 0.0 {
                        v6629 = v6623;
                        v6630 = v6627;
                    } else {
                        v6629 = v6618;
                        v6630 = v6622;
                    }
                    let v6631 = v6614.abs();
                    let v6635 = v6617 * ((v236 * (if v6614 >= v4732 { 1.0 } else { 0.0 })) - v184);
                    let v6636 = if v6629 < v6631 { 1.0 } else { 0.0 };
                    let v6637: f64;
                    let v6638: Lanes<6>;
                    if v6636 != 0.0 {
                        v6637 = v6631;
                        v6638 = v6635;
                    } else {
                        v6637 = v6629;
                        v6638 = v6630;
                    }
                    let v6639 = if v5669 > v3883 { 1.0 } else { 0.0 };
                    let v6641: f64;
                    if v6639 != 0.0 {
                        v6641 = v4754;
                    } else {
                        let v6640 = if v5669 > v4755 { 1.0 } else { 0.0 };
                        let v6645: f64;
                        if v6640 != 0.0 {
                            v6645 = v4754;
                        } else {
                            let v6644 = if v5669 > v4760 { 1.0 } else { 0.0 };
                            let v6647: f64;
                            if v6644 != 0.0 {
                                v6647 = v4754;
                            } else {
                                let v6646 = if v5669 > v4763 { 1.0 } else { 0.0 };
                                let v6648: f64;
                                if v6646 != 0.0 {
                                    v6648 = v4766;
                                } else {
                                    v6648 = v89;
                                }
                                v6647 = v6648;
                            }
                            v6645 = v6647;
                        }
                        v6641 = v6645;
                    }
                    let v6642 = v210 / v6641;
                    let v6643 = if v6637 > v6642 { 1.0 } else { 0.0 };
                    let v6665: f64;
                    let v6666: f64;
                    let v6667: f64;
                    let v6668: Lanes<6>;
                    let v6669: Lanes<6>;
                    let v6670: Lanes<6>;
                    if v6643 != 0.0 {
                        let v6649 = v6642 / v6637;
                        let v6652 = ((v6638 * v6649) * v95) / v6637;
                        let v6653 = v6574 * v6649;
                        let v6656 = (v6577 * v6649) + (v6652 * v6574);
                        let v6657 = v6594 * v6649;
                        let v6660 = (v6597 * v6649) + (v6652 * v6594);
                        let v6661 = v6614 * v6649;
                        let v6664 = (v6617 * v6649) + (v6652 * v6614);
                        v6665 = v6653;
                        v6666 = v6657;
                        v6667 = v6661;
                        v6668 = v6656;
                        v6669 = v6660;
                        v6670 = v6664;
                    } else {
                        v6665 = v6574;
                        v6666 = v6594;
                        v6667 = v6614;
                        v6668 = v6577;
                        v6669 = v6597;
                        v6670 = v6617;
                    }
                    let v6671 = v5671 + v6665;
                    let v6672 = v5682 + v6668;
                    let v6673 = v5672 + v6666;
                    let v6674 = v5683 + v6669;
                    let v6675 = v5670 + v6667;
                    let v6676 = v5681 + v6670;
                    let v6678 = if v6637 < (v407 * v6641) { 1.0 } else { 0.0 };
                    let v6679: f64;
                    if v6678 != 0.0 {
                        v6679 = v89;
                    } else {
                        v6679 = v5673;
                    }
                    v6390 = v5669;
                    v6391 = v6675;
                    v6392 = v6671;
                    v6393 = v6673;
                    v6394 = v6679;
                    v6395 = v5674;
                    v6396 = v6676;
                    v6397 = v6672;
                    v6398 = v6674;
                }
                let v6399 = v6390 + v89;
                v5669 = v6399;
                v5670 = v6391;
                v5671 = v6392;
                v5672 = v6393;
                v5673 = v6394;
                v5674 = v6395;
                v5675 = v5753;
                v5676 = v5881;
                v5677 = v6134;
                v5678 = v5882;
                v5679 = v5742;
                v5680 = v6135;
                v5681 = v6396;
                v5682 = v6397;
                v5683 = v6398;
                v5684 = v5755;
                v5685 = v5887;
                v5686 = v6140;
                v5687 = v5888;
                v5688 = v5744;
                v5689 = v6141;
            }
            let v5702 = if v5674 > v60 { 1.0 } else { 0.0 };
            let v6680: f64;
            if v5702 != 0.0 {
                v6680 = v5674;
            } else {
                v6680 = v5669;
            }
            let v6681 = if v6680 > v5169 { 1.0 } else { 0.0 };
            let v6682: f64;
            let v6683: f64;
            let v6684: f64;
            let v6685: Lanes<6>;
            let v6686: Lanes<6>;
            let v6687: Lanes<6>;
            if v6681 != 0.0 {
                v6682 = v5657;
                v6683 = v5658;
                v6684 = v5659;
                v6685 = v5661;
                v6686 = v5662;
                v6687 = v5663;
            } else {
                v6682 = v5671;
                v6683 = v5675;
                v6684 = v5670;
                v6685 = v5682;
                v6686 = v5684;
                v6687 = v5681;
            }
            let v6688 = v6682 - v4802;
            let v6689 = v6685 - v4806;
            let v6693 = if (if v1437 <= v6690 { 1.0 } else { 0.0 }) != 0.0 || (if v4802 < v60 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v6694: f64;
            if v6693 != 0.0 {
                v6694 = v89;
            } else {
                v6694 = v4814;
            }
            let v6695 = v6683 - v4803;
            let v6696 = v6686 - v4807;
            let v6697 = v5676 - v3789;
            let v6698 = v5685 - v3798;
            let v6699 = v5676 + v3789;
            let v6700 = v5685 + v3798;
            let v6701 = v151 * v6699;
            let v6702 = v154 * v6699;
            let v6712 = v6697 - ((v6701 * v6688) * v502);
            let v6713 = v6698 - (((((Lanes([v6702[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v6700 * v151)) * v6688) + (v6689 * v6701)) * v502);
            let v6716 = v5677 + v3790;
            let v6717 = v5686 + v3799;
            let v6718 = v151 * v6716;
            let v6719 = v154 * v6716;
            let v6729 = (v5677 - v3790) - ((v6718 * v6695) * v502);
            let v6730 = (v5686 - v3799) - (((((Lanes([v6719[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v6717 * v151)) * v6695) + (v6696 * v6718)) * v502);
            let v6732 = if v113 == v60 { 1.0 } else { 0.0 };
            let v6733 = if (if v6712 < v60 { 1.0 } else { 0.0 }) != 0.0 || v6732 != 0.0 { 1.0 } else { 0.0 };
            let v6734: f64;
            let v6735: Lanes<6>;
            if v6733 != 0.0 {
                v6734 = v60;
                v6735 = v3227;
            } else {
                v6734 = v6712;
                v6735 = v6713;
            }
            let v6737 = if (if v6729 < v60 { 1.0 } else { 0.0 }) != 0.0 || v6732 != 0.0 { 1.0 } else { 0.0 };
            let v6738: f64;
            let v6739: Lanes<6>;
            if v6737 != 0.0 {
                v6738 = v60;
                v6739 = v3227;
            } else {
                v6738 = v6729;
                v6739 = v6730;
            }
            let v6740 = v6734 + v6738;
            let v6741 = v6735 + v6739;
            let v6742 = v5678 + v3792;
            let v6743 = v5687 + v3801;
            let v6745 = v6744 * v6742;
            let v6746 = v6743 * v6744;
            let v6747 = v6688 + v407;
            let v6748 = -v6697;
            let v6749 = v6698 * v95;
            let v6752 = if (-v6748) < v6751 { 1.0 } else { 0.0 };
            let v6753: f64;
            let v6754: Lanes<6>;
            if v6752 != 0.0 {
                v6753 = v60;
                v6754 = v3227;
            } else {
                v6753 = v6748;
                v6754 = v6749;
            }
            let v6759 = v151 * v627;
            let v6760 = v154 * v627;
            let v6761 = v630 * v151;
            let v6764 = (Lanes([v6760[0], 0.0, 0.0, 0.0, 0.0])) + (Lanes([0.0, v6761[0], v6761[1], v6761[2], v6761[3]]));
            let v6765 = v6759 * v6747;
            let v6766 = v6764 * v6747;
            let v6770 = v6765 * v6747;
            let v6774 = (v254 * (-v6753)) / v6770;
            let v6778 = v89 + v6774;
            let v6783 = (v6778 * v6747) / v4821;
            let v6787 = v89 - v6783;
            let v6788 = ((((((((v6754 * v95) * v254) - (((((Lanes([v6766[0], v6766[1], v6766[2], 0.0, v6766[3], v6766[4]])) + (v6689 * v6759)) * v6747) + (v6689 * v6765)) * v6774)) / v6770) * v6747) + (v6689 * v6778)) - (v4825 * v6783)) / v4821) * v95;
            let v6789 = if v6787 <= v60 { 1.0 } else { 0.0 };
            let v6790: f64;
            let v6791: Lanes<6>;
            if v6789 != 0.0 {
                v6790 = v60;
                v6791 = v3227;
            } else {
                v6790 = v6787;
                v6791 = v6788;
            }
            let v6793 = v6792 * v6699;
            let v6794 = v6700 * v6792;
            let v6796 = v6795 * v6716;
            let v6797 = v6717 * v6795;
            let v6798 = if v6694 == v60 { 1.0 } else { 0.0 };
            let v6806: f64;
            let v6807: f64;
            let v6808: Lanes<6>;
            let v6809: Lanes<6>;
            if v6798 != 0.0 {
                let v6805 = if (if v6799 < v6800 { 1.0 } else { 0.0 }) != 0.0 && (if v6802 < v6803 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v6846: f64;
                let v6847: f64;
                let v6848: Lanes<6>;
                let v6849: Lanes<6>;
                if v6805 != 0.0 {
                    let v6813 = v4802 + v463;
                    let v6815 = v4806 + (Lanes([0.0, 0.0, v465[0], 0.0, v465[1], v465[2]]));
                    let v6818 = if v6682 > (v6813 - v6816) { 1.0 } else { 0.0 };
                    let v6855: f64;
                    let v6856: Lanes<6>;
                    if v6818 != 0.0 {
                        let v6854 = v6813 - v6853;
                        v6855 = v6854;
                        v6856 = v6815;
                    } else {
                        v6855 = v6682;
                        v6856 = v6685;
                    }
                    v6846 = v60;
                    v6847 = v6855;
                    v6848 = v3227;
                    v6849 = v6856;
                } else {
                    let v6824 = (v6799 * v470) + ((v6802 * v4813) / v813);
                    let v6825 = v472 / v6824;
                    let v6828 = ((((v4815 * v6802) / v813) * v6825) * v95) / v6824;
                    let v6835 = v89 - v6832;
                    let v6838 = (v6832 * (v113 + v4802)) + (v6835 * v6682);
                    let v6839 = (((Lanes([0.0, 0.0, 0.0, 0.0, v122[0], v122[1]])) + v4806) * v6832) + (v6685 * v6835);
                    let v6840 = v4802 + v463;
                    let v6842 = v4806 + (Lanes([0.0, 0.0, v465[0], 0.0, v465[1], v465[2]]));
                    let v6845 = if v6838 > (v6840 - v6843) { 1.0 } else { 0.0 };
                    let v6859: f64;
                    let v6860: Lanes<6>;
                    if v6845 != 0.0 {
                        let v6858 = v6840 - v6857;
                        v6859 = v6858;
                        v6860 = v6842;
                    } else {
                        v6859 = v6838;
                        v6860 = v6839;
                    }
                    let v6861 = v6859 - v6682;
                    let v6862 = v6860 - v6685;
                    let v6864 = v6862 * v6861;
                    let v6868 = ((v6861 * v6861) + v6866).sqrt();
                    let v6875 = (v6862 + ((v6864 + v6864) * (v184 / (v236 * v6868)))) * v502;
                    let v6877 = (v502 * (v6861 + v6868)) + v6876;
                    let v6878 = if v6877 < v60 { 1.0 } else { 0.0 };
                    let v6879: f64;
                    let v6880: Lanes<6>;
                    if v6878 != 0.0 {
                        v6879 = v60;
                        v6880 = v3227;
                    } else {
                        v6879 = v6877;
                        v6880 = v6875;
                    }
                    let v6881 = v151 * v4813;
                    let v6882 = v154 * v4813;
                    let v6886 = v6740 / v6881;
                    let v6893 = v254 * (v470 / v472);
                    let v6894 = v6893 * v6879;
                    let v6895 = v6880 * v6893;
                    let v6907 = (((v254 * v6886) + (v6894 * v6825)) + (v6902 * v6825)) / v3295;
                    let v6909 = v6907 * v6825;
                    let v6912 = (((((((v6741 - (((Lanes([v6882[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v4815 * v151)) * v6886)) / v6881) * v254) + ((v6895 * v6825) + (v6828 * v6894))) + (v6828 * v6902)) / v3295) * v6825) + (v6828 * v6907);
                    let v6914 = v446 * (v6894 + v6902);
                    let v6916 = v6914 * v6825;
                    let v6925 = v6912 * v6909;
                    let v6929 = ((v6909 * v6909) + (v6916 * v6825)).sqrt();
                    let v6937 = v502 * ((-v6909) + v6929);
                    let v6939 = v574 * v6937;
                    let v6940 = v576 * v6937;
                    let v6943 = (Lanes([v6940[0], v6940[1], v6940[2], 0.0, v6940[3], v6940[4]])) + ((((v6912 * v95) + (((v6925 + v6925) + (((((v6895 * v446) * v6825) + (v6828 * v6914)) * v6825) + (v6828 * v6916))) * (v184 / (v236 * v6929)))) * v502) * v574);
                    v6846 = v6939;
                    v6847 = v6859;
                    v6848 = v6943;
                    v6849 = v6860;
                }
                let v6851 = v6846 * v6850;
                let v6852 = v6848 * v6850;
                v6806 = v6851;
                v6807 = v6847;
                v6808 = v6852;
                v6809 = v6849;
            } else {
                v6806 = v60;
                v6807 = v60;
                v6808 = v3227;
                v6809 = v3227;
            }
            let v6810 = v3295 - v6806;
            let v6811 = v6808 * v95;
            let v6812 = if v6810 < v69 { 1.0 } else { 0.0 };
            let v6944: f64;
            let v6945: Lanes<6>;
            if v6812 != 0.0 {
                v6944 = v69;
                v6945 = v3227;
            } else {
                v6944 = v6810;
                v6945 = v6811;
            }
            let v6949 = v6948 * (v6793 + v6796);
            let v6950 = (v6794 + v6797) * v6948;
            let v6958 = ((v502 * (v3793 + v5679)) * v3295) * v6957;
            let v6959 = (((v3802 + v5688) * v502) * v3295) * v6957;
            let v6960 = v113 - v6688;
            let v6961 = Lanes([0.0, 0.0, 0.0, 0.0, v122[0], v122[1]]);
            let v6962 = v6961 - v6689;
            let v6968 = (v254 * (v6960 / v254)) / v6967;
            let v6969 = ((v6962 / v254) * v254) / v6967;
            let v6974 = v6973 + (v6968 * v6970);
            let v6980 = v6979 + (v6968 * v6974);
            let v6986 = v6985 + (v6968 * v6980);
            let v6992 = v6991 + (v6968 * v6986);
            let v6998 = v6997 + (v6968 * v6992);
            let v7003 = v89 + (v6968 * v6998);
            let v7004 = v6967 / v7003;
            let v7007 = ((((v6969 * v6998) + (((v6969 * v6992) + (((v6969 * v6986) + (((v6969 * v6980) + (((v6969 * v6974) + ((v6969 * v6970) * v6968)) * v6968)) * v6968)) * v6968)) * v6968)) * v7004) * v95) / v7003;
            let v7009 = if v7004 < v7008 { 1.0 } else { 0.0 };
            let v7011: f64;
            let v7012: Lanes<6>;
            if v7009 != 0.0 {
                v7011 = v7010;
                v7012 = v3227;
            } else {
                v7011 = v7004;
                v7012 = v7007;
            }
            let v7013 = v4802 + v7011;
            let v7014 = v4806 + v7012;
            let v7016 = v5676 / v7015;
            let v7017 = v5685 / v7015;
            let v7018 = v5677 / v7015;
            let v7019 = v5686 / v7015;
            let v7020 = v3794 / v7015;
            let v7021 = v3803 / v7015;
            let v7022 = v5680 / v7015;
            let v7023 = v5689 / v7015;
            let v7024 = v6793 / v7015;
            let v7025 = v6794 / v7015;
            let v7026 = v6796 / v7015;
            let v7027 = v6797 / v7015;
            let v7028 = v6745 / v7015;
            let v7029 = v6746 / v7015;
            let v7031 = v6689 * v6688;
            let v7035 = ((v6688 * v6688) + v7033).sqrt();
            let v7042 = (v6689 + ((v7031 + v7031) * (v184 / (v236 * v7035)))) * v502;
            let v7044 = (v502 * (v6688 + v7035)) + v7043;
            let v7045 = if v7044 < v60 { 1.0 } else { 0.0 };
            let v7046: f64;
            let v7047: Lanes<6>;
            if v7045 != 0.0 {
                v7046 = v60;
                v7047 = v3227;
            } else {
                v7046 = v7044;
                v7047 = v7042;
            }
            let v7049 = v7047 * v7046;
            let v7052 = ((v7046 * v7046) + v366).sqrt();
            let v7057 = v7052 - v7056;
            let v7067 = v89 + ((v7057.powf(v7058)) * v7064);
            let v7081 = ((v7073 * v7028) + (v7076 * (v7024 - (v7068 * v7018)))) / v7067;
            let v7084 = (((v7029 * v7073) + ((v7025 - (v7019 * v7068)) * v7076)) - (((((v7049 + v7049) * (v184 / (v236 * v7052))) * (v7058 * (v7057.powf(v7060)))) * v7064) * v7081)) / v7067;
            let v7106: f64;
            let v7107: f64;
            let v7108: f64;
            let v7109: Lanes<6>;
            let v7110: Lanes<6>;
            let v7111: Lanes<6>;
            if v6 != 0.0 {
                let v7087 = (v4803 + v6683) * v502;
                let v7088 = (v4807 + v6686) * v502;
                let v7091 = (v4805 + v6684) * v502;
                let v7092 = (v4809 + v6687) * v502;
                let v7104 = v7081 + ((v7098 * ((v7087 - v7091) - v1134)) / v7101);
                let v7105 = v7084 + ((((v7088 - v7092) - (Lanes([v1135[0], 0.0, v1135[1], 0.0, v1135[2], v1135[3]]))) * v7098) / v7101);
                v7106 = v7104;
                v7107 = v7087;
                v7108 = v7091;
                v7109 = v7105;
                v7110 = v7088;
                v7111 = v7092;
            } else {
                v7106 = v7081;
                v7107 = v60;
                v7108 = v60;
                v7109 = v7084;
                v7110 = v3227;
                v7111 = v3227;
            }
            let v7113 = v7109 * v7106;
            let v7117 = ((v7106 * v7106) + v7115).sqrt();
            let v7124 = (v7109 + ((v7113 + v7113) * (v184 / (v236 * v7117)))) * v502;
            let v7126 = (v502 * (v7106 + v7117)) + v7125;
            let v7127 = if v7126 < v60 { 1.0 } else { 0.0 };
            let v7128: f64;
            let v7129: Lanes<6>;
            if v7127 != 0.0 {
                v7128 = v60;
                v7129 = v3227;
            } else {
                v7128 = v7126;
                v7129 = v7124;
            }
            let v7131 = v7128.powf(v7130);
            let v7151 = v7150 + ((v7144 * (v7024 / v150)) / v7147);
            let v7152 = v89 / v7151;
            let v7157 = v195 * v7131;
            let v7166 = (v7152 + (v194 * v7131)) + ((v7128.powf(v7136)) / v7163);
            let v7168 = v89 / v7166;
            let v7172 = v7168 * v650;
            let v7173 = (((((((((((v7025 / v150) * v7144) / v7147) * v7152) * v95) / v7151) + ((Lanes([v7157[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v7129 * (v7130 * (v7128.powf(v7132)))) * v194))) + ((v7129 * (v7136 * (v7128.powf(v7138)))) / v7163)) * v7168) * v95) / v7166) * v650;
            let v7197: f64;
            let v7198: Lanes<6>;
            if v6 != 0.0 {
                let v7179 = (v7098 * (v7107 - v7108)) / v7178;
                let v7180 = ((v7110 - v7111) * v7098) / v7178;
                v7197 = v7179;
                v7198 = v7180;
            } else {
                let v7182 = v6696 * v6695;
                let v7186 = ((v6695 * v6695) + v7184).sqrt();
                let v7193 = (v6696 + ((v7182 + v7182) * (v184 / (v236 * v7186)))) * v502;
                let v7195 = (v502 * (v6695 + v7186)) + v7194;
                let v7196 = if v7195 < v60 { 1.0 } else { 0.0 };
                let v7215: f64;
                let v7216: Lanes<6>;
                if v7196 != 0.0 {
                    v7215 = v60;
                    v7216 = v3227;
                } else {
                    v7215 = v7195;
                    v7216 = v7193;
                }
                let v7218 = v7216 * v7215;
                let v7221 = ((v7215 * v7215) + v366).sqrt();
                let v7225 = v7221 - v7056;
                let v7232 = v89 + ((v7225.powf(v7058)) * v7064);
                let v7249 = ((v7073 * (v7240 * (v7022 + v7020))) + (v7076 * (v7026 - (v7233 * v7016)))) / v7232;
                let v7252 = (((((v7023 + v7021) * v7240) * v7073) + ((v7027 - (v7017 * v7233)) * v7076)) - (((((v7218 + v7218) * (v184 / (v236 * v7221))) * (v7058 * (v7225.powf(v7060)))) * v7064) * v7249)) / v7232;
                v7197 = v7249;
                v7198 = v7252;
            }
            let v7200 = v7198 * v7197;
            let v7204 = ((v7197 * v7197) + v7202).sqrt();
            let v7211 = (v7198 + ((v7200 + v7200) * (v184 / (v236 * v7204)))) * v502;
            let v7213 = (v502 * (v7197 + v7204)) + v7212;
            let v7214 = if v7213 < v60 { 1.0 } else { 0.0 };
            let v7253: f64;
            let v7254: Lanes<6>;
            if v7214 != 0.0 {
                v7253 = v60;
                v7254 = v3227;
            } else {
                v7253 = v7213;
                v7254 = v7211;
            }
            let v7256 = v7253.powf(v7255);
            let v7275 = v7274 + ((v7269 * (v7026 / v150)) / v7147);
            let v7276 = v89 / v7275;
            let v7281 = v198 * v7256;
            let v7290 = (v7276 + (v197 * v7256)) + ((v7253.powf(v7261)) / v7287);
            let v7292 = v89 / v7290;
            let v7296 = v7292 * v650;
            let v7297 = (((((((((((v7027 / v150) * v7269) / v7147) * v7276) * v95) / v7275) + ((Lanes([v7281[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v7254 * (v7255 * (v7253.powf(v7257)))) * v197))) + ((v7254 * (v7261 * (v7253.powf(v7263)))) / v7287)) * v7292) * v95) / v7290) * v650;
            let v7298 = v1448 * v233;
            let v7299 = v234 * v1448;
            let v7300 = v7298 / v7172;
            let v7302 = Lanes([v7299[0], 0.0, 0.0, 0.0, 0.0, 0.0]);
            let v7305 = v4813 + v418;
            let v7306 = v151 * v7305;
            let v7307 = v154 * v7305;
            let v7311 = v7306 * v6944;
            let v7315 = v6734 / v7311;
            let v7320 = ((v6735 - (((((Lanes([v7307[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v4815 * v151)) * v6944) + (v6945 * v7306)) * v7315)) / v7311) * v7315;
            let v7323 = ((v7302 - (v7173 * v7300)) / v7172) * v7300;
            let v7327 = ((v7315 * v7315) + (v7300 * v7300)).sqrt();
            let v7330 = ((v7320 + v7320) + (v7323 + v7323)) * (v184 / (v236 * v7327));
            let v7335 = (v7172 * v7327) / v233;
            let v7336 = v234 * v7335;
            let v7339 = (((v7173 * v7327) + (v7330 * v7172)) - (Lanes([v7336[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v233;
            let v7342: f64;
            let v7343: Lanes<6>;
            if v7340 != 0.0 {
                v7342 = v89;
                v7343 = v3227;
            } else {
                let v7356: f64;
                let v7357: Lanes<6>;
                if v7341 != 0.0 {
                    v7356 = v7335;
                    v7357 = v7339;
                } else {
                    let v7351 = v7335.powf(v7350);
                    let v7355 = v7339 * (v7350 * (v7335.powf(v7352)));
                    v7356 = v7351;
                    v7357 = v7355;
                }
                v7342 = v7356;
                v7343 = v7357;
            }
            let v7347 = (v7339 * v7342) + (v7343 * v7335);
            let v7348 = v89 + (v7335 * v7342);
            let v7363: f64;
            let v7364: Lanes<6>;
            if v7349 != 0.0 {
                let v7358 = v89 / v7348;
                let v7361 = ((v7347 * v7358) * v95) / v7348;
                v7363 = v7358;
                v7364 = v7361;
            } else {
                let v7427: f64;
                let v7428: Lanes<6>;
                if v7362 != 0.0 {
                    let v7409 = v7348.sqrt();
                    let v7413 = v89 / v7409;
                    let v7416 = (((v7347 * (v184 / (v236 * v7409))) * v7413) * v95) / v7409;
                    v7427 = v7413;
                    v7428 = v7416;
                } else {
                    let v7418 = v7348.powf(v7417);
                    let v7423 = v7348 * v7418;
                    let v7426 = (v7347 * v7418) + ((v7347 * (v7417 * (v7348.powf(v7419)))) * v7348);
                    v7427 = v7423;
                    v7428 = v7426;
                }
                v7363 = v7427;
                v7364 = v7428;
            }
            let v7365 = v7172 * v7363;
            let v7368 = (v7173 * v7363) + (v7364 * v7172);
            let v7369 = v7298 / v7296;
            let v7373 = v4819 + v418;
            let v7374 = v151 * v7373;
            let v7375 = v154 * v7373;
            let v7379 = v7374 * v6944;
            let v7383 = v6738 / v7379;
            let v7388 = ((v6739 - (((((Lanes([v7375[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v4820 * v151)) * v6944) + (v6945 * v7374)) * v7383)) / v7379) * v7383;
            let v7391 = ((v7302 - (v7297 * v7369)) / v7296) * v7369;
            let v7395 = ((v7383 * v7383) + (v7369 * v7369)).sqrt();
            let v7403 = (v7296 * v7395) / v233;
            let v7404 = v234 * v7403;
            let v7407 = (((v7297 * v7395) + ((((v7388 + v7388) + (v7391 + v7391)) * (v184 / (v236 * v7395))) * v7296)) - (Lanes([v7404[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v233;
            let v7430: f64;
            let v7431: Lanes<6>;
            if v7408 != 0.0 {
                v7430 = v89;
                v7431 = v3227;
            } else {
                let v7444: f64;
                let v7445: Lanes<6>;
                if v7429 != 0.0 {
                    v7444 = v7403;
                    v7445 = v7407;
                } else {
                    let v7439 = v7403.powf(v7438);
                    let v7443 = v7407 * (v7438 * (v7403.powf(v7440)));
                    v7444 = v7439;
                    v7445 = v7443;
                }
                v7430 = v7444;
                v7431 = v7445;
            }
            let v7435 = (v7407 * v7430) + (v7431 * v7403);
            let v7436 = v89 + (v7403 * v7430);
            let v7451: f64;
            let v7452: Lanes<6>;
            if v7437 != 0.0 {
                let v7446 = v89 / v7436;
                let v7449 = ((v7435 * v7446) * v95) / v7436;
                v7451 = v7446;
                v7452 = v7449;
            } else {
                let v7500: f64;
                let v7501: Lanes<6>;
                if v7450 != 0.0 {
                    let v7482 = v7436.sqrt();
                    let v7486 = v89 / v7482;
                    let v7489 = (((v7435 * (v184 / (v236 * v7482))) * v7486) * v95) / v7482;
                    v7500 = v7486;
                    v7501 = v7489;
                } else {
                    let v7491 = v7436.powf(v7490);
                    let v7496 = v7436 * v7491;
                    let v7499 = (v7435 * v7491) + ((v7435 * (v7490 * (v7436.powf(v7492)))) * v7436);
                    v7500 = v7496;
                    v7501 = v7499;
                }
                v7451 = v7500;
                v7452 = v7501;
            }
            let v7453 = v7296 * v7451;
            let v7456 = (v7297 * v7451) + (v7452 * v7296);
            let v7458 = v161 * v3387;
            let v7459 = (v3387 * v158) / v6810;
            let v7463 = ((Lanes([v7458[0], 0.0, 0.0, 0.0, 0.0, 0.0])) - (v6811 * v7459)) / v6810;
            let v7464 = v7459 * v6734;
            let v7472 = v7459 * v6738;
            let v7480 = (v7464 * v7365) + (v7472 * v7453);
            let v7481 = ((((v7463 * v6734) + (v6735 * v7459)) * v7365) + (v7368 * v7464)) + ((((v7463 * v6738) + (v6739 * v7459)) * v7453) + (v7456 * v7472));
            let v7567: f64;
            let v7568: Lanes<6>;
            if v0 != 0.0 {
                let v7506 = (v254 * (v502 * v6960)) / v205;
                let v7507 = ((v6962 * v502) * v254) / v205;
                let v7512 = v7511 + (v7506 * v7508);
                let v7518 = v7517 + (v7506 * v7512);
                let v7524 = v7523 + (v7506 * v7518);
                let v7530 = v7529 + (v7506 * v7524);
                let v7536 = v7535 + (v7506 * v7530);
                let v7541 = v89 + (v7506 * v7536);
                let v7542 = v205 / v7541;
                let v7546 = v4802 + v7542;
                let v7547 = v4806 + (((((v7507 * v7536) + (((v7507 * v7530) + (((v7507 * v7524) + (((v7507 * v7518) + (((v7507 * v7512) + ((v7507 * v7508) * v7506)) * v7506)) * v7506)) * v7506)) * v7506)) * v7542) * v95) / v7541);
                let v7549 = v7548 - v7546;
                let v7550 = v7547 * v95;
                let v7552 = v7550 * v7549;
                let v7556 = ((v7549 * v7549) + v7554).sqrt();
                let v7563 = (v7550 + ((v7552 + v7552) * (v184 / (v236 * v7556)))) * v502;
                let v7565 = (v502 * (v7549 + v7556)) + v7564;
                let v7566 = if v7565 < v60 { 1.0 } else { 0.0 };
                let v7570: f64;
                let v7571: Lanes<6>;
                if v7566 != 0.0 {
                    v7570 = v60;
                    v7571 = v3227;
                } else {
                    v7570 = v7565;
                    v7571 = v7563;
                }
                let v7573 = v6759 * v7572;
                let v7576 = v7570.powf(v7575);
                let v7581 = v7573 * v7576;
                let v7582 = (v6764 * v7572) * v7576;
                let v7588 = v465 * v7586;
                let v7591 = v463 * v7590;
                let v7593 = v7546 - v459;
                let v7597 = (v465 * v7590) * v7593;
                let v7601 = (v89 + (v463 * v7586)) + (v7591 * v7593);
                let v7604 = v7581 * v7601;
                let v7607 = (((Lanes([v7582[0], v7582[1], v7582[2], 0.0, v7582[3], v7582[4]])) + ((v7571 * (v7575 * (v7570.powf(v7577)))) * v7573)) * v7601) + (((Lanes([0.0, 0.0, v7588[0], 0.0, v7588[1], v7588[2]])) + ((Lanes([0.0, 0.0, v7597[0], 0.0, v7597[1], v7597[2]])) + ((v7547 - (Lanes([0.0, 0.0, v460[0], 0.0, v460[1], v460[2]]))) * v7591))) * v7581);
                v7567 = v7604;
                v7568 = v7607;
            } else {
                v7567 = v60;
                v7568 = v3227;
            }
            let v7616: f64;
            let v7617: Lanes<5>;
            if v7569 != 0.0 {
                let v7609 = v6759 * v7608;
                let v7611 = v7609 * v463;
                let v7613 = v465 * v7609;
                let v7615 = ((v6764 * v7608) * v463) + (Lanes([0.0, 0.0, v7613[0], v7613[1], v7613[2]]));
                v7616 = v7611;
                v7617 = v7615;
            } else {
                v7616 = v60;
                v7617 = v508;
            }
            let v7618 = v7567 + v7616;
            let v7619 = Lanes([v7617[0], v7617[1], v7617[2], 0.0, v7617[3], v7617[4]]);
            let v7620 = v7568 + v7619;
            let v7621 = if v7618 > v60 { 1.0 } else { 0.0 };
            let v7652: f64;
            let v7653: f64;
            let v7654: f64;
            let v7655: Lanes<6>;
            let v7656: Lanes<4>;
            let v7657: Lanes<6>;
            if v7621 != 0.0 {
                let v7622 = v6688 * v7618;
                let v7626 = v7459 * v7622;
                let v7630 = v7626 * v7365;
                let v7633 = (((v7463 * v7622) + (((v6689 * v7618) + (v7620 * v6688)) * v7459)) * v7365) + (v7368 * v7626);
                let v7635 = -v7634;
                let v7638 = (v7635 * v1134).exp();
                let v7640 = v89 + v7638;
                let v7641 = v89 / v7640;
                let v7644 = ((((v1135 * v7635) * v7638) * v7641) * v95) / v7640;
                let v7645 = v89 - v7641;
                let v7647 = v7645 * v7630;
                let v7648 = (v7644 * v95) * v7630;
                let v7651 = (Lanes([v7648[0], 0.0, v7648[1], 0.0, v7648[2], v7648[3]])) + (v7633 * v7645);
                v7652 = v7630;
                v7653 = v7641;
                v7654 = v7647;
                v7655 = v7633;
                v7656 = v7644;
                v7657 = v7651;
            } else {
                v7652 = v60;
                v7653 = v60;
                v7654 = v60;
                v7655 = v3227;
                v7656 = v992;
                v7657 = v3227;
            }
            let v7724: f64;
            let v7725: Lanes<6>;
            if v0 != 0.0 {
                let v7664 = (v254 * (v502 * (v113 - v6695))) / v205;
                let v7665 = (((v6961 - v6696) * v502) * v254) / v205;
                let v7670 = v7669 + (v7664 * v7666);
                let v7676 = v7675 + (v7664 * v7670);
                let v7682 = v7681 + (v7664 * v7676);
                let v7688 = v7687 + (v7664 * v7682);
                let v7694 = v7693 + (v7664 * v7688);
                let v7699 = v89 + (v7664 * v7694);
                let v7700 = v205 / v7699;
                let v7704 = v4803 + v7700;
                let v7705 = v4807 + (((((v7665 * v7694) + (((v7665 * v7688) + (((v7665 * v7682) + (((v7665 * v7676) + (((v7665 * v7670) + ((v7665 * v7666) * v7664)) * v7664)) * v7664)) * v7664)) * v7664)) * v7700) * v95) / v7699);
                let v7706 = v7548 - v7704;
                let v7707 = v7705 * v95;
                let v7709 = v7707 * v7706;
                let v7713 = ((v7706 * v7706) + v7711).sqrt();
                let v7720 = (v7707 + ((v7709 + v7709) * (v184 / (v236 * v7713)))) * v502;
                let v7722 = (v502 * (v7706 + v7713)) + v7721;
                let v7723 = if v7722 < v60 { 1.0 } else { 0.0 };
                let v7729: f64;
                let v7730: Lanes<6>;
                if v7723 != 0.0 {
                    v7729 = v60;
                    v7730 = v3227;
                } else {
                    v7729 = v7722;
                    v7730 = v7720;
                }
                let v7731 = v6759 * v7572;
                let v7733 = v7729.powf(v7575);
                let v7738 = v7731 * v7733;
                let v7739 = (v6764 * v7572) * v7733;
                let v7744 = v465 * v7586;
                let v7746 = v463 * v7590;
                let v7748 = v7704 - v459;
                let v7752 = (v465 * v7590) * v7748;
                let v7756 = (v89 + (v463 * v7586)) + (v7746 * v7748);
                let v7759 = v7738 * v7756;
                let v7762 = (((Lanes([v7739[0], v7739[1], v7739[2], 0.0, v7739[3], v7739[4]])) + ((v7730 * (v7575 * (v7729.powf(v7734)))) * v7731)) * v7756) + (((Lanes([0.0, 0.0, v7744[0], 0.0, v7744[1], v7744[2]])) + ((Lanes([0.0, 0.0, v7752[0], 0.0, v7752[1], v7752[2]])) + ((v7705 - (Lanes([0.0, 0.0, v460[0], 0.0, v460[1], v460[2]]))) * v7746))) * v7738);
                v7724 = v7759;
                v7725 = v7762;
            } else {
                v7724 = v60;
                v7725 = v3227;
            }
            let v7726 = v7724 + v7616;
            let v7727 = v7725 + v7619;
            let v7728 = if v7726 > v60 { 1.0 } else { 0.0 };
            let v7782: f64;
            let v7783: Lanes<6>;
            if v7728 != 0.0 {
                let v7763 = v6695 * v7726;
                let v7767 = v7459 * v7763;
                let v7771 = v7767 * v7453;
                let v7774 = (((v7463 * v7763) + (((v6696 * v7726) + (v7727 * v6695)) * v7459)) * v7453) + (v7456 * v7767);
                let v7775 = v7652 * v527;
                let v7776 = v7655 * v527;
                let v7777 = v7652 - v7775;
                let v7778 = v7655 - v7776;
                let v7781 = if (if v7771 > v7777 { 1.0 } else { 0.0 }) != 0.0 && (if v7775 >= v60 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v7809: f64;
                let v7810: Lanes<6>;
                if v7781 != 0.0 {
                    let v7792 = (v7771 - v7652) + v7775;
                    let v7793 = (v7774 - v7655) + v7776;
                    let v7794 = v7792 * v7792;
                    let v7795 = v7793 * v7792;
                    let v7797 = v7775 * v7775;
                    let v7798 = v7776 * v7775;
                    let v7801 = (v7795 + v7795) * v7794;
                    let v7804 = (v7798 + v7798) * v7797;
                    let v7806 = (v7794 * v7794) + (v7797 * v7797);
                    let v7807 = (v7801 + v7801) + (v7804 + v7804);
                    let v7823: f64;
                    let v7824: Lanes<6>;
                    if v7808 != 0.0 {
                        let v7841: f64;
                        if v7816 != 0.0 {
                            v7841 = v89;
                        } else {
                            let v7843: f64;
                            if v7840 != 0.0 {
                                v7843 = v254;
                            } else {
                                let v7845: f64;
                                if v7842 != 0.0 {
                                    v7845 = v443;
                                } else {
                                    let v7846: f64;
                                    if v7844 != 0.0 {
                                        v7846 = v446;
                                    } else {
                                        v7846 = v60;
                                    }
                                    v7845 = v7846;
                                }
                                v7843 = v7845;
                            }
                            v7841 = v7843;
                        }
                        let mut v7847: f64 = 0.0;
                        let mut v7848: f64 = 0.0;
                        let mut v7849: Lanes<6> = Lanes([0.0; 6]);
                        v7847 = v60;
                        v7848 = v7806;
                        v7849 = v7807;
                        loop {
                            let v7850 = if v7847 < v7841 { 1.0 } else { 0.0 };
                            if v7850 == 0.0 {
                                break;
                            }
                            let v7851 = v7848.sqrt();
                            let v7854 = v7849 * (v184 / (v236 * v7851));
                            let v7855 = v7847 + v89;
                            v7847 = v7855;
                            v7848 = v7851;
                            v7849 = v7854;
                        }
                        v7823 = v7848;
                        v7824 = v7849;
                    } else {
                        let v7818 = v7806.powf(v7817);
                        let v7822 = v7807 * (v7817 * (v7806.powf(v7819)));
                        v7823 = v7818;
                        v7824 = v7822;
                    }
                    let v7825 = v7823 + v418;
                    let v7826 = v89 / v7825;
                    let v7830 = v7792 * v7775;
                    let v7838 = v7777 + (v7830 * v7826);
                    let v7839 = v7778 + ((((v7793 * v7775) + (v7776 * v7792)) * v7826) + ((((v7824 * v7826) * v95) / v7825) * v7830));
                    v7809 = v7838;
                    v7810 = v7839;
                } else {
                    v7809 = v7771;
                    v7810 = v7774;
                }
                let v7811 = v7653 * v7809;
                let v7812 = v7656 * v7809;
                let v7815 = (Lanes([v7812[0], 0.0, v7812[1], 0.0, v7812[2], v7812[3]])) + (v7810 * v7653);
                v7782 = v7811;
                v7783 = v7815;
            } else {
                v7782 = v60;
                v7783 = v3227;
            }
            let v7786 = v7480 + (v7654 + v7782);
            let v7787 = v7481 + (v7657 + v7783);
            let v7789 = if v7788 != v60 { 1.0 } else { 0.0 };
            let v7945: f64;
            let v7946: Lanes<6>;
            if v7789 != 0.0 {
                let v7858 = v7856 - v7857;
                let v7860 = v89 / (v7858 * v7858);
                let v7861 = v254 * v819;
                let v7865 = v812 * v7861;
                let v7871 = ((v7861 * v811) * v7868) * v7860;
                let v7873 = v7871 * v746;
                let v7875 = v749 * v7871;
                let v7882 = v7881 + (v7878 * v463);
                let v7883 = v7873 * v7882;
                let v7885 = (v465 * v7878) * v7873;
                let v7887 = ((((((((v820 * v254) * v811) + (Lanes([0.0, v7865[0], v7865[1], v7865[2], v7865[3]]))) * v7868) * v7860) * v746) + (Lanes([0.0, 0.0, v7875[0], v7875[1], v7875[2]]))) * v7882) + (Lanes([0.0, 0.0, v7885[0], v7885[1], v7885[2]]));
                let v7893 = (v122 * v7888) * v95;
                let v7897 = v469 + (Lanes([0.0, 0.0, v7893[0], v7893[1]]));
                let v7898 = ((v466 - v477) + (v7891 - (v7888 * v113))) + v7883;
                let v7900 = (Lanes([0.0, v7897[0], v7897[1], v7897[2], v7897[3]])) + v7887;
                let v7901 = v276 * v625;
                let v7902 = v278 * v625;
                let v7903 = v628 * v276;
                let v7907 = v7901 * v625;
                let v7909 = v628 * v7901;
                let v7911 = (((Lanes([v7902[0], 0.0, 0.0, 0.0, 0.0])) + (Lanes([0.0, v7903[0], v7903[1], v7903[2], v7903[3]]))) * v625) + (Lanes([0.0, v7909[0], v7909[1], v7909[2], v7909[3]]));
                let v7914 = v154 * v7907;
                let v7917 = (v7907 * v151) * v502;
                let v7918 = ((v7911 * v151) + (Lanes([v7914[0], 0.0, 0.0, 0.0, 0.0]))) * v502;
                let v7921 = v154 * v7917;
                let v7924 = (v7917 * v151) * v254;
                let v7925 = ((v7918 * v151) + (Lanes([v7921[0], 0.0, 0.0, 0.0, 0.0]))) * v254;
                let v7926 = v151 * v795;
                let v7930 = (v154 * v795) * v7907;
                let v7938 = (v4834 - ((v7911 * v7926) + (Lanes([v7930[0], 0.0, 0.0, 0.0, 0.0])))) - v7887;
                let v7939 = ((((v158 - (v7907 * v7926)) + v477) - v7891) - v7883) + v418;
                let v7942 = (Lanes([0.0, v469[0], v469[1], v469[2], v469[3]])) - v7938;
                let v7943 = (v466 - v7939) - v806;
                let v7944 = if v7939 >= v60 { 1.0 } else { 0.0 };
                let v7953: f64;
                if v7944 != 0.0 {
                    v7953 = v89;
                } else {
                    v7953 = v7952;
                }
                let v7955 = v7942 * v7943;
                let v7957 = v7953 * v446;
                let v7964 = ((v7943 * v7943) + ((v7957 * v7939) * v806)).sqrt();
                let v7978 = ((((v7939 + (v502 * (v7943 + v7964))) - v477) + v7891) + v7883) - v459;
                let v7979 = Lanes([0.0, 0.0, v460[0], v460[1], v460[2]]);
                let v7982 = v154 * v7978;
                let v7986 = (v151 * v7978) - v89;
                let v7987 = v446 / v7924;
                let v7994 = (((Lanes([v7982[0], 0.0, 0.0, 0.0, 0.0])) + ((((v7938 + ((v7942 + (((v7955 + v7955) + ((v7938 * v7957) * v806)) * (v184 / (v236 * v7964)))) * v502)) + v7887) - v7979) * v151)) * v7987) + ((((v7925 * v7987) * v95) / v7924) * v7986);
                let v7995 = v89 + (v7986 * v7987);
                let v7997 = v7994 * v7995;
                let v8001 = ((v7995 * v7995) + v7999).sqrt();
                let v8008 = (v7994 + ((v7997 + v7997) * (v184 / (v236 * v8001)))) * v502;
                let v8010 = (v502 * (v7995 + v8001)) + v8009;
                let v8011 = if v8010 < v60 { 1.0 } else { 0.0 };
                let v8012: f64;
                let v8013: Lanes<5>;
                if v8011 != 0.0 {
                    v8012 = v60;
                    v8013 = v508;
                } else {
                    v8012 = v8010;
                    v8013 = v8008;
                }
                let v8015 = (v8012 + v418).sqrt();
                let v8019 = v89 - v8015;
                let v8025 = v7898 + (v7917 * v8019);
                let v8026 = v7900 + ((v7918 * v8019) + (((v8013 * (v184 / (v236 * v8015))) * v95) * v7917));
                let v8027 = v7898 + v418;
                let v8028 = v254 / v8027;
                let v8032 = v151 + v8028;
                let v8035 = v89 / v8032;
                let v8039 = v89 / v3230;
                let v8042 = ((v3238 * v8039) * v95) / v3230;
                let v8043 = v8039 / v7907;
                let v8048 = v7898 * v7898;
                let v8049 = v7900 * v7898;
                let v8051 = v8043 * v8048;
                let v8055 = v8051.ln();
                let v8058 = v8055 * v8035;
                let v8061 = (((((((Lanes([v8042[0], 0.0, 0.0, 0.0, 0.0])) - (v7911 * v8043)) / v7907) * v8048) + ((v8049 + v8049) * v8043)) * (v184 / v8051)) * v8035) + ((((((Lanes([v154[0], 0.0, 0.0, 0.0, 0.0])) + (((v7900 * v8028) * v95) / v8027)) * v8035) * v95) / v8032) * v8055);
                let v8063 = v8061 - v8026;
                let v8064 = (v8058 - v8025) - v3189;
                let v8066 = v8063 * v8064;
                let v8068 = v446 * v3189;
                let v8071 = (v8064 * v8064) + (v8068 * v8058);
                let v8072 = (v8066 + v8066) + (v8061 * v8068);
                let v8074 = v8072 * v8071;
                let v8078 = ((v8071 * v8071) + v8076).sqrt();
                let v8085 = (v8072 + ((v8074 + v8074) * (v184 / (v236 * v8078)))) * v502;
                let v8087 = (v502 * (v8071 + v8078)) + v8086;
                let v8088 = if v8087 < v60 { 1.0 } else { 0.0 };
                let v8089: f64;
                let v8090: Lanes<5>;
                if v8088 != 0.0 {
                    v8089 = v60;
                    v8090 = v508;
                } else {
                    v8089 = v8087;
                    v8090 = v8085;
                }
                let v8091 = v8089.sqrt();
                let v8099 = v8058 - (v502 * (v8064 + v8091));
                let v8100 = v8061 - ((v8063 + (v8090 * (v184 / (v236 * v8091)))) * v502);
                let v8102 = v154 * v8099;
                let v8106 = (v151 * v8099).exp();
                let v8109 = v3238 * v8106;
                let v8113 = v8099 - v459;
                let v8116 = v154 * v8113;
                let v8119 = (Lanes([v8116[0], 0.0, 0.0, 0.0, 0.0])) + ((v8100 - v7979) * v151);
                let v8120 = (v151 * v8113) - v89;
                let v8121 = v8120 + (v3230 * v8106);
                let v8122 = v8119 + ((Lanes([v8109[0], 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([v8102[0], 0.0, 0.0, 0.0, 0.0])) + (v8100 * v151)) * v8106) * v3230));
                let v8124 = v8122 * v8121;
                let v8128 = ((v8121 * v8121) + v8126).sqrt();
                let v8135 = (v8122 + ((v8124 + v8124) * (v184 / (v236 * v8128)))) * v502;
                let v8137 = (v502 * (v8121 + v8128)) + v8136;
                let v8138 = if v8137 < v60 { 1.0 } else { 0.0 };
                let v8139: f64;
                let v8140: Lanes<5>;
                if v8138 != 0.0 {
                    v8139 = v60;
                    v8140 = v508;
                } else {
                    v8139 = v8137;
                    v8140 = v8135;
                }
                let v8143 = (v8139 + v8141).sqrt();
                let v8146 = v8140 * (v184 / (v236 * v8143));
                let v8148 = v8119 * v8120;
                let v8152 = ((v8120 * v8120) + v8150).sqrt();
                let v8159 = (v8119 + ((v8148 + v8148) * (v184 / (v236 * v8152)))) * v502;
                let v8161 = (v502 * (v8120 + v8152)) + v8160;
                let v8162 = if v8161 < v60 { 1.0 } else { 0.0 };
                let v8163: f64;
                let v8164: Lanes<5>;
                if v8162 != 0.0 {
                    v8163 = v60;
                    v8164 = v508;
                } else {
                    v8163 = v8161;
                    v8164 = v8159;
                }
                let v8167 = (v8163 + v8165).sqrt();
                let v8171 = v8143 - v8167;
                let v8173 = v3231 * v8171;
                let v8174 = v3239 * v8171;
                let v8177 = (Lanes([v8174[0], 0.0, 0.0, 0.0, 0.0])) + ((v8146 - (v8164 * (v184 / (v236 * v8167)))) * v3231);
                let v8178 = v8025 - v8099;
                let v8179 = v8026 - v8100;
                let v8181 = v8179 * v8178;
                let v8185 = ((v8178 * v8178) + v8183).sqrt();
                let v8192 = (v8179 + ((v8181 + v8181) * (v184 / (v236 * v8185)))) * v502;
                let v8194 = (v502 * (v8178 + v8185)) + v8193;
                let v8195 = if v8194 < v60 { 1.0 } else { 0.0 };
                let v8196: f64;
                let v8197: Lanes<5>;
                if v8195 != 0.0 {
                    v8196 = v60;
                    v8197 = v508;
                } else {
                    v8196 = v8194;
                    v8197 = v8192;
                }
                let v8199 = v8196 + v8198;
                let v8200 = v113 / v8199;
                let v8203 = (v548 - (v8197 * v8200)) / v8199;
                let v8204 = v8200 * v8200;
                let v8205 = v8203 * v8200;
                let v8206 = v8205 + v8205;
                let v8207 = v8204 * v8204;
                let v8208 = v8206 * v8204;
                let v8210 = v8207 * v8204;
                let v8217 = ((((v8208 + v8208) * v8204) + (v8206 * v8207)) * v8204) + (v8206 * v8210);
                let v8219 = (v8210 * v8204) + v8218;
                let v8228: f64;
                let v8229: Lanes<5>;
                if v8220 != 0.0 {
                    let v8266: f64;
                    if v8221 != 0.0 {
                        v8266 = v89;
                    } else {
                        let v8268: f64;
                        if v8265 != 0.0 {
                            v8268 = v254;
                        } else {
                            let v8270: f64;
                            if v8267 != 0.0 {
                                v8270 = v443;
                            } else {
                                let v8271: f64;
                                if v8269 != 0.0 {
                                    v8271 = v446;
                                } else {
                                    v8271 = v60;
                                }
                                v8270 = v8271;
                            }
                            v8268 = v8270;
                        }
                        v8266 = v8268;
                    }
                    let mut v8272: f64 = 0.0;
                    let mut v8273: f64 = 0.0;
                    let mut v8274: Lanes<5> = Lanes([0.0; 5]);
                    v8272 = v60;
                    v8273 = v8219;
                    v8274 = v8217;
                    loop {
                        let v8275 = if v8272 < v8266 { 1.0 } else { 0.0 };
                        if v8275 == 0.0 {
                            break;
                        }
                        let v8276 = v8273.sqrt();
                        let v8279 = v8274 * (v184 / (v236 * v8276));
                        let v8280 = v8272 + v89;
                        v8272 = v8280;
                        v8273 = v8276;
                        v8274 = v8279;
                    }
                    v8228 = v8273;
                    v8229 = v8274;
                } else {
                    let v8223 = v8219.powf(v8222);
                    let v8227 = v8217 * (v8222 * (v8219.powf(v8224)));
                    v8228 = v8223;
                    v8229 = v8227;
                }
                let v8230 = v8228 + v418;
                let v8231 = v89 / v8230;
                let v8235 = v8200 * v8231;
                let v8241 = v8239 * v8240;
                let v8242 = v8241 * v158;
                let v8244 = v8242 * v7365;
                let v8245 = (v161 * v8241) * v7365;
                let v8249 = v8244 * v8173;
                let v8251 = v8177 * v8244;
                let v8256 = ((v8203 * v8231) + ((((v8229 * v8231) * v95) / v8230) * v8200)) * v8249;
                let v8259 = (v8249 * v8235) / v6944;
                let v8263 = v7786 + v8259;
                let v8264 = v7787 + ((((((((Lanes([v8245[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v7368 * v8242)) * v8173) + (Lanes([v8251[0], v8251[1], v8251[2], 0.0, v8251[3], v8251[4]]))) * v8235) + (Lanes([v8256[0], v8256[1], v8256[2], 0.0, v8256[3], v8256[4]]))) - (v6945 * v8259)) / v6944);
                v7945 = v8263;
                v7946 = v8264;
            } else {
                v7945 = v7786;
                v7946 = v7787;
            }
            let v7950 = if v7949 != v60 { 1.0 } else { 0.0 };
            let v7951 = if (if v7947 != v60 { 1.0 } else { 0.0 }) != 0.0 && v7950 != 0.0 { 1.0 } else { 0.0 };
            let v8313: f64;
            let v8314: f64;
            let v8315: f64;
            let v8316: f64;
            let v8317: Lanes<6>;
            let v8318: Lanes<6>;
            let v8319: Lanes<6>;
            if v7951 != 0.0 {
                let v8281 = v4821 * v4821;
                let v8282 = v4825 * v4821;
                let v8283 = v8282 + v8282;
                let v8284 = v285 * v625;
                let v8285 = v286 * v625;
                let v8286 = v628 * v285;
                let v8291 = ((Lanes([v8285[0], 0.0, 0.0, 0.0, 0.0])) + (Lanes([0.0, v8286[0], v8286[1], v8286[2], v8286[3]]))) * v6740;
                let v8295 = v8281 - (v8284 * v6740);
                let v8296 = v8283 - ((Lanes([v8291[0], v8291[1], v8291[2], 0.0, v8291[3], v8291[4]])) + (v6741 * v8284));
                let v8298 = v8283 * v8281;
                let v8302 = ((v8281 * v8281) + v8300).sqrt();
                let v8309 = (v8283 + ((v8298 + v8298) * (v184 / (v236 * v8302)))) * v502;
                let v8311 = (v502 * (v8281 + v8302)) + v8310;
                let v8312 = if v8311 < v60 { 1.0 } else { 0.0 };
                let v8321: f64;
                let v8322: Lanes<6>;
                if v8312 != 0.0 {
                    v8321 = v60;
                    v8322 = v3227;
                } else {
                    v8321 = v8311;
                    v8322 = v8309;
                }
                let v8324 = v8296 * v8295;
                let v8328 = ((v8295 * v8295) + v8326).sqrt();
                let v8335 = (v8296 + ((v8324 + v8324) * (v184 / (v236 * v8328)))) * v502;
                let v8337 = (v502 * (v8295 + v8328)) + v8336;
                let v8338 = if v8337 < v60 { 1.0 } else { 0.0 };
                let v8339: f64;
                let v8340: Lanes<6>;
                if v8338 != 0.0 {
                    v8339 = v60;
                    v8340 = v3227;
                } else {
                    v8339 = v8337;
                    v8340 = v8335;
                }
                let v8341 = v8321 - v8339;
                let v8342 = v8322 - v8340;
                let v8347 = if (if v4813 < v8343 { 1.0 } else { 0.0 }) != 0.0 || (if v8341 < v8345 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8348: f64;
                if v8347 != 0.0 {
                    v8348 = v60;
                } else {
                    v8348 = v89;
                }
                v8313 = v8348;
                v8314 = v8339;
                v8315 = v8321;
                v8316 = v8341;
                v8317 = v8340;
                v8318 = v8322;
                v8319 = v8342;
            } else {
                v8313 = v60;
                v8314 = v60;
                v8315 = v60;
                v8316 = v60;
                v8317 = v3227;
                v8318 = v3227;
                v8319 = v3227;
            }
            let v8320 = if v3232 > v60 { 1.0 } else { 0.0 };
            let v8382: f64;
            let v8383: Lanes<6>;
            if v8320 != 0.0 {
                let v8349 = v254 / v3531;
                let v8350 = v8349 * v4826;
                let v8355 = v460 * v3538;
                let v8356 = (v3233 - v158) - (v3538 * v459);
                let v8360 = (v4828 * v8349) * v8356;
                let v8363 = (Lanes([0.0, v8360[0], v8360[1], v8360[2], v8360[3]])) + (((v3241 - v4834) - (Lanes([0.0, 0.0, v8355[0], v8355[1], v8355[2]]))) * v8350);
                let v8364 = v89 + (v8350 * v8356);
                let v8366 = v8363 * v8364;
                let v8370 = ((v8364 * v8364) + v8368).sqrt();
                let v8377 = (v8363 + ((v8366 + v8366) * (v184 / (v236 * v8370)))) * v502;
                let v8379 = (v502 * (v8364 + v8370)) + v8378;
                let v8380 = if v8379 < v60 { 1.0 } else { 0.0 };
                let v8389: f64;
                let v8390: Lanes<5>;
                if v8380 != 0.0 {
                    v8389 = v60;
                    v8390 = v508;
                } else {
                    v8389 = v8379;
                    v8390 = v8377;
                }
                let v8394 = (v8389 + v418).sqrt();
                let v8398 = v89 - v8394;
                let v8401 = v4832 * v8398;
                let v8408 = v465 * v3593;
                let v8412 = v3599 * v3600;
                let v8415 = ((v3593 * v463) + v3234) - (v8412 * ((v3233 * v3573) + (v4829 * v8398)));
                let v8416 = ((Lanes([0.0, 0.0, v8408[0], v8408[1], v8408[2]])) + v3242) - (((v3241 * v3573) + ((Lanes([0.0, v8401[0], v8401[1], v8401[2], v8401[3]])) + (((v8390 * (v184 / (v236 * v8394))) * v95) * v4829))) * v8412);
                let v8418 = v8416 * v8415;
                let v8422 = ((v8415 * v8415) + v8420).sqrt();
                let v8429 = (v8416 + ((v8418 + v8418) * (v184 / (v236 * v8422)))) * v502;
                let v8431 = (v502 * (v8415 + v8422)) + v8430;
                let v8432 = if v8431 < v60 { 1.0 } else { 0.0 };
                let v8433: f64;
                let v8434: Lanes<5>;
                if v8432 != 0.0 {
                    v8433 = v60;
                    v8434 = v508;
                } else {
                    v8433 = v8431;
                    v8434 = v8429;
                }
                let v8435 = v8433 + v418;
                let v8437 = (-v3625) / v8435;
                let v8441 = v8437.exp();
                let v8443 = v3419 * v8435;
                let v8445 = v8443 * v7945;
                let v8446 = (v8434 * v3419) * v7945;
                let v8450 = v8445 * v8441;
                let v8452 = ((((v8434 * v8437) * v95) / v8435) * v8441) * v8445;
                let v8454 = (((Lanes([v8446[0], v8446[1], v8446[2], 0.0, v8446[3], v8446[4]])) + (v7946 * v8443)) * v8441) + (Lanes([v8452[0], v8452[1], v8452[2], 0.0, v8452[3], v8452[4]]));
                v8382 = v8450;
                v8383 = v8454;
            } else {
                let v8381 = Lanes([v3240[0], v3240[1], v3240[2], 0.0, v3240[3], v3240[4]]);
                v8382 = v3232;
                v8383 = v8381;
            }
            let v8388 = if (if v6798 != 0.0 && (if v8382 > v60 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v8386 != v60 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v8488: f64;
            let v8489: Lanes<6>;
            if v8388 != 0.0 {
                let v8459 = v8386 * (v89 + (v8455 * v1016));
                let v8461 = v8459 * v8382;
                let v8462 = ((v1014 * v8455) * v8386) * v8382;
                let v8465 = (Lanes([v8462[0], v8462[1], v8462[2], 0.0, v8462[3], v8462[4]])) + (v8383 * v8459);
                let v8467 = v154 * v4802;
                let v8470 = (Lanes([v8467[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v4806 * v151);
                let v8471 = (v151 * v4802) - v89;
                let v8473 = v8470 * v8471;
                let v8477 = ((v8471 * v8471) + v8475).sqrt();
                let v8484 = (v8470 + ((v8473 + v8473) * (v184 / (v236 * v8477)))) * v502;
                let v8486 = (v502 * (v8471 + v8477)) + v8485;
                let v8487 = if v8486 < v60 { 1.0 } else { 0.0 };
                let v8500: f64;
                let v8501: Lanes<6>;
                if v8487 != 0.0 {
                    v8500 = v60;
                    v8501 = v3227;
                } else {
                    v8500 = v8486;
                    v8501 = v8484;
                }
                let v8502 = v8500.sqrt();
                let v8505 = v8501 * (v184 / (v236 * v8502));
                let v8506 = v8500 * v8502;
                let v8509 = (v8501 * v8502) + (v8505 * v8500);
                let v8511 = v154 * v6682;
                let v8514 = (Lanes([v8511[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v6685 * v151);
                let v8515 = (v151 * v6682) - v89;
                let v8517 = v8514 * v8515;
                let v8521 = ((v8515 * v8515) + v8519).sqrt();
                let v8528 = (v8514 + ((v8517 + v8517) * (v184 / (v236 * v8521)))) * v502;
                let v8530 = (v502 * (v8515 + v8521)) + v8529;
                let v8531 = if v8530 < v60 { 1.0 } else { 0.0 };
                let v8532: f64;
                let v8533: Lanes<6>;
                if v8531 != 0.0 {
                    v8532 = v60;
                    v8533 = v3227;
                } else {
                    v8532 = v8530;
                    v8533 = v8528;
                }
                let v8534 = v8532.sqrt();
                let v8537 = v8533 * (v184 / (v236 * v8534));
                let v8538 = v8532 * v8534;
                let v8542 = v151 * v8461;
                let v8543 = v154 * v8461;
                let v8546 = (Lanes([v8543[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v8465 * v151);
                let v8547 = v8542 / v8500;
                let v8550 = (v8546 - (v8501 * v8547)) / v8500;
                let v8551 = v8542 / v8532;
                let v8554 = (v8546 - (v8533 * v8551)) / v8532;
                let v8563 = (v8538 * v8551) - (v8506 * v8547);
                let v8566 = v308 * v8563;
                let v8570 = v307 * v502;
                let v8572 = -v8534;
                let v8582 = (v8572 * v8551) + (v8502 * v8547);
                let v8585 = (v308 * v502) * v8582;
                let v8589 = (v307 * v8563) + (v8570 * v8582);
                let v8591 = v7459 * v8589;
                let v8595 = v8591 * v7365;
                let v8598 = (((v7463 * v8589) + ((((Lanes([v8566[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((((v8533 * v8534) + (v8537 * v8532)) * v8551) + (v8554 * v8538)) - ((v8509 * v8547) + (v8550 * v8506))) * v307)) + ((Lanes([v8585[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (((((v8537 * v95) * v8551) + (v8554 * v8572)) + ((v8505 * v8547) + (v8550 * v8502))) * v8570))) * v7459)) * v7365) + (v7368 * v8591);
                v8488 = v8595;
                v8489 = v8598;
            } else {
                v8488 = v60;
                v8489 = v3227;
            }
            let v8490 = v676 * v3385;
            let v8491 = v627 / v7015;
            let v8492 = v630 / v7015;
            let v8493 = v3295 * v3385;
            let v8494 = v3387 * v3385;
            let v8495 = v7327 / v3385;
            let v8496 = v7330 / v3385;
            let v8497 = v307 / v7015;
            let v8498 = v308 / v7015;
            let v8499 = if v1 == v60 { 1.0 } else { 0.0 };
            let v8600: f64;
            let v8601: f64;
            let v8602: f64;
            let v8603: f64;
            let v8604: f64;
            let v8605: Lanes<4>;
            let v8606: Lanes<6>;
            let v8607: Lanes<3>;
            let v8608: Lanes<3>;
            if v8499 != 0.0 {
                v8600 = v60;
                v8601 = v60;
                v8602 = v60;
                v8603 = v60;
                v8604 = v60;
                v8605 = v603;
                v8606 = v3227;
                v8607 = v8599;
                v8608 = v8599;
            } else {
                let v8667: f64;
                let v8668: Lanes<6>;
                if v6798 != 0.0 {
                    let v8619 = -v8618;
                    let v8621 = v355 * v8619;
                    let v8634 = (Lanes([0.0, v469[0], v469[1], v469[2], v469[3]])) + (((Lanes([0.0, 0.0, v8621[0], v8621[1], v8621[2]])) + ((v1014 - v1028) * v8624)) / v8493);
                    let v8638 = ((v466 - (v8615 * v477)) + (((v8619 * v353) + (v8624 * (v1016 - v1023))) / v8493)) - (((v7013 + v463) - v8613) * v8635);
                    let v8644 = v89 + (v8495 / v8641);
                    let v8649 = (v8644 * v8638) / v8490;
                    let v8650 = (((v8496 / v8641) * v8638) + (((Lanes([v8634[0], v8634[1], v8634[2], 0.0, v8634[3], v8634[4]])) - ((v7014 + (Lanes([0.0, 0.0, v465[0], 0.0, v465[1], v465[2]]))) * v8635)) * v8644)) / v8490;
                    let v8652 = v8650 * v8649;
                    let v8656 = ((v8649 * v8649) + v8654).sqrt();
                    let v8663 = (v8650 + ((v8652 + v8652) * (v184 / (v236 * v8656)))) * v502;
                    let v8665 = (v502 * (v8649 + v8656)) + v8664;
                    let v8666 = if v8665 < v60 { 1.0 } else { 0.0 };
                    let v8704: f64;
                    let v8705: Lanes<6>;
                    if v8666 != 0.0 {
                        v8704 = v60;
                        v8705 = v3227;
                    } else {
                        v8704 = v8665;
                        v8705 = v8663;
                    }
                    let v8707 = v469 * v466;
                    let v8711 = ((v466 * v466) + v8709).sqrt();
                    let v8718 = (v469 + ((v8707 + v8707) * (v184 / (v236 * v8711)))) * v502;
                    let v8720 = (v502 * (v466 + v8711)) + v8719;
                    let v8721 = if v8720 < v60 { 1.0 } else { 0.0 };
                    let v8722: f64;
                    let v8723: Lanes<4>;
                    if v8721 != 0.0 {
                        v8722 = v60;
                        v8723 = v603;
                    } else {
                        v8722 = v8720;
                        v8723 = v8718;
                    }
                    let v8725 = (v8722 - v366) / v210;
                    let v8728 = (v8723 / v210) * v8725;
                    let v8730 = v89 + (v8725 * v8725);
                    let v8731 = v89 / v8730;
                    let v8735 = v89 - v8731;
                    let v8737 = v8704 * v8735;
                    let v8739 = (((((v8728 + v8728) * v8731) * v95) / v8730) * v95) * v8704;
                    let v8741 = (v8705 * v8735) + (Lanes([0.0, v8739[0], v8739[1], 0.0, v8739[2], v8739[3]]));
                    let v8742 = v8493 * v8494;
                    let v8745 = v8743 / (v8743 + v8742);
                    let v8747 = v8746 + v463;
                    let v8748 = v8746 / v8747;
                    let v8751 = ((v465 * v8748) * v95) / v8747;
                    let v8753 = v8741 * v8737;
                    let v8755 = (v8737 * v8737) + v418;
                    let v8756 = v89 / v8755;
                    let v8761 = -v8760;
                    let v8762 = v8761 * v240;
                    let v8764 = v8762 * v8756;
                    let v8765 = (v243 * v8761) * v8756;
                    let v8768 = (Lanes([v8765[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (((((v8753 + v8753) * v8756) * v95) / v8755) * v8762);
                    let v8770 = if v8764 < v8769 { 1.0 } else { 0.0 };
                    let v8822: f64;
                    let v8823: Lanes<6>;
                    if v8770 != 0.0 {
                        v8822 = v60;
                        v8823 = v3227;
                    } else {
                        let v8772 = v8771 / v235;
                        let v8778 = (v8772 * v150) * v8742;
                        let v8781 = v8492 * v407;
                        let v8785 = (v7024 + (v8491 * v407)) / v8497;
                        let v8786 = v8498 * v8785;
                        let v8791 = v8785.powf(v8790);
                        let v8796 = v8764.exp();
                        let v8798 = v8796 * v8778;
                        let v8800 = (((((v239 * v8772) * v95) / v235) * v150) * v8742) * v8796;
                        let v8803 = v8798 * v8791;
                        let v8807 = v8803 * v8737;
                        let v8811 = v8807 * v8737;
                        let v8815 = v8745 * v8748;
                        let v8817 = v8815 * v8811;
                        let v8818 = (v8751 * v8745) * v8811;
                        let v8821 = (Lanes([0.0, 0.0, v8818[0], 0.0, v8818[1], v8818[2]])) + ((((((((((v8768 * v8796) * v8778) + (Lanes([v8800[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) * v8791) + (((((v7025 + (Lanes([0.0, v8781[0], v8781[1], 0.0, v8781[2], v8781[3]]))) - (Lanes([v8786[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v8497) * (v8790 * (v8785.powf((v8790 - v184))))) * v8798)) * v8737) + (v8741 * v8803)) * v8737) + (v8741 * v8807)) * v8815);
                        v8822 = v8817;
                        v8823 = v8821;
                    }
                    v8667 = v8822;
                    v8668 = v8823;
                } else {
                    v8667 = v60;
                    v8668 = v3227;
                }
                let v8670 = -v8669;
                let v8677 = (v8490 * ((v8670 * v114) + v8673)).exp();
                let v8680 = v8679 * v114;
                let v8683 = (v89 / v8490) / v8490;
                let v8685 = (v123 * v8679) * v8680;
                let v8687 = (v8680 * v8680) * v8683;
                let v8696 = ((v8689 / v8690) * v8494) * (v8693.powf(v8694));
                let v8697 = v8696 * v8677;
                let v8699 = v8697 * v8687;
                let v8702 = (((((v123 * v8670) * v8490) * v8677) * v8696) * v8687) + (((v8685 + v8685) * v8683) * v8697);
                let v8703 = if v8680 >= v60 { 1.0 } else { 0.0 };
                let v8827: f64;
                let v8828: Lanes<3>;
                if v8703 != 0.0 {
                    let v8825 = v8699 * v8824;
                    let v8826 = v8702 * v8824;
                    v8827 = v8825;
                    v8828 = v8826;
                } else {
                    v8827 = v8699;
                    v8828 = v8702;
                }
                let v8829 = v114 - v113;
                let v8831 = v123 - (Lanes([0.0, v122[0], v122[1]]));
                let v8837 = (v8490 * ((v8670 * v8829) + v8673)).exp();
                let v8839 = v8679 * v8829;
                let v8842 = (v8831 * v8679) * v8839;
                let v8844 = (v8839 * v8839) * v8683;
                let v8846 = v8696 * v8837;
                let v8848 = v8846 * v8844;
                let v8851 = (((((v8831 * v8670) * v8490) * v8837) * v8696) * v8844) + (((v8842 + v8842) * v8683) * v8846);
                let v8852 = if v8839 >= v60 { 1.0 } else { 0.0 };
                let v8856: f64;
                let v8857: Lanes<3>;
                if v8852 != 0.0 {
                    let v8854 = v8848 * v8853;
                    let v8855 = v8851 * v8853;
                    v8856 = v8854;
                    v8857 = v8855;
                } else {
                    v8856 = v8848;
                    v8857 = v8851;
                }
                let v8858 = -v114;
                let v8859 = v123 * v95;
                let v8862 = v355 * v8860;
                let v8864 = Lanes([v8859[0], 0.0, v8859[1], v8859[2]]);
                let v8870 = (((v8858 + (v8860 * v353)) + v477) + v8868) / v8490;
                let v8871 = (v8864 + (Lanes([0.0, v8862[0], v8862[1], v8862[2]]))) / v8490;
                let v8873 = v8871 * v8870;
                let v8877 = ((v8870 * v8870) + v8875).sqrt();
                let v8884 = (v8871 + ((v8873 + v8873) * (v184 / (v236 * v8877)))) * v502;
                let v8886 = (v502 * (v8870 + v8877)) + v8885;
                let v8887 = if v8886 < v60 { 1.0 } else { 0.0 };
                let v8888: f64;
                let v8889: Lanes<4>;
                if v8887 != 0.0 {
                    v8888 = v60;
                    v8889 = v603;
                } else {
                    v8888 = v8886;
                    v8889 = v8884;
                }
                let v8890 = v8888 + v418;
                let v8894 = v8890.powf(v8893);
                let v8899 = (-v8891) / v8894;
                let v8902 = (((v8889 * (v8893 * (v8890.powf((v8893 - v184))))) * v8899) * v95) / v8894;
                let v8904 = if v8899 < v8903 { 1.0 } else { 0.0 };
                let v8916: f64;
                let v8917: Lanes<4>;
                if v8904 != 0.0 {
                    v8916 = v60;
                    v8917 = v603;
                } else {
                    let v8905 = v8899.exp();
                    let v8906 = v8902 * v8905;
                    let v8908 = v8693 + v8907;
                    let v8911 = v8908 * v633;
                    let v8912 = (v8908 - v8909) - v8911;
                    let v8914 = (v446 * v8909) * v8911;
                    let v8915 = if v8914 > v60 { 1.0 } else { 0.0 };
                    let v8919: f64;
                    if v8915 != 0.0 {
                        v8919 = v8914;
                    } else {
                        let v8918 = -v8914;
                        v8919 = v8918;
                    }
                    let v8929 = (((v8909 + (v502 * (v8912 + (((v8912 * v8912) + v8919).sqrt())))) * v8926) / v8690) * v8494;
                    let v8936 = v8929 * (v8890.powf(v8930));
                    let v8938 = v8936 * v8905;
                    let v8941 = (((v8889 * (v8930 * (v8890.powf((v8930 - v184))))) * v8929) * v8905) + (v8906 * v8936);
                    let v8944 = v355 * v8942;
                    let v8951 = (((v8858 + (v8942 * v353)) + v477) + v8949) / v8490;
                    let v8952 = (v8864 + (Lanes([0.0, v8944[0], v8944[1], v8944[2]]))) / v8490;
                    let v8954 = v8952 * v8951;
                    let v8958 = ((v8951 * v8951) + v8956).sqrt();
                    let v8965 = (v8952 + ((v8954 + v8954) * (v184 / (v236 * v8958)))) * v502;
                    let v8967 = (v502 * (v8951 + v8958)) + v8966;
                    let v8968 = if v8967 < v60 { 1.0 } else { 0.0 };
                    let v8969: f64;
                    let v8970: Lanes<4>;
                    if v8968 != 0.0 {
                        v8969 = v60;
                        v8970 = v603;
                    } else {
                        v8969 = v8967;
                        v8970 = v8965;
                    }
                    let v8971 = v8969 + v418;
                    let v8975 = v8971.powf(v8974);
                    let v8980 = (-v8972) / v8975;
                    let v8983 = (((v8970 * (v8974 * (v8971.powf((v8974 - v184))))) * v8980) * v95) / v8975;
                    let v8985 = if v8980 < v8984 { 1.0 } else { 0.0 };
                    let v8997: f64;
                    let v8998: Lanes<4>;
                    if v8985 != 0.0 {
                        v8997 = v60;
                        v8998 = v603;
                    } else {
                        let v8986 = v8980.exp();
                        let v8987 = v8983 * v8986;
                        let v8989 = v8693 + v8988;
                        let v8992 = v8989 * v633;
                        let v8993 = (v8989 - v8990) - v8992;
                        let v8995 = (v446 * v8990) * v8992;
                        let v8996 = if v8995 > v60 { 1.0 } else { 0.0 };
                        let v9005: f64;
                        if v8996 != 0.0 {
                            v9005 = v8995;
                        } else {
                            let v9004 = -v8995;
                            v9005 = v9004;
                        }
                        let v9015 = (((v8990 + (v502 * (v8993 + (((v8993 * v8993) + v9005).sqrt())))) * v9012) / v8690) * v8494;
                        let v9022 = v9015 * (v8971.powf(v9016));
                        let v9024 = v9022 * v8986;
                        let v9027 = (((v8970 * (v9016 * (v8971.powf((v9016 - v184))))) * v9015) * v8986) + (v8987 * v9022);
                        v8997 = v9024;
                        v8998 = v9027;
                    }
                    let v8999 = -v8938;
                    let v9000 = v8941 * v95;
                    let v9001 = v8999 * v633;
                    let v9002 = v9000 * v633;
                    let v9003 = if v9001 < v418 { 1.0 } else { 0.0 };
                    let v9028: f64;
                    let v9029: Lanes<4>;
                    if v9003 != 0.0 {
                        v9028 = v418;
                        v9029 = v603;
                    } else {
                        v9028 = v9001;
                        v9029 = v9002;
                    }
                    let v9030 = -v8997;
                    let v9031 = v8998 * v95;
                    let v9034 = (v8999 - v9030) - v9028;
                    let v9035 = (v9000 - v9031) - v9029;
                    let v9036 = v446 * v9030;
                    let v9038 = v9036 * v9028;
                    let v9041 = ((v9031 * v446) * v9028) + (v9029 * v9036);
                    let v9042 = if v9038 > v60 { 1.0 } else { 0.0 };
                    let v9045: f64;
                    let v9046: Lanes<4>;
                    if v9042 != 0.0 {
                        v9045 = v9038;
                        v9046 = v9041;
                    } else {
                        let v9043 = -v9038;
                        let v9044 = v9041 * v95;
                        v9045 = v9043;
                        v9046 = v9044;
                    }
                    let v9048 = v9035 * v9034;
                    let v9052 = ((v9034 * v9034) + v9045).sqrt();
                    let v9062 = -(v9030 + (v502 * (v9034 + v9052)));
                    let v9063 = (v9031 + ((v9035 + (((v9048 + v9048) + v9046) * (v184 / (v236 * v9052)))) * v502)) * v95;
                    v8916 = v9062;
                    v8917 = v9063;
                }
                v8600 = v8916;
                v8601 = v502;
                v8602 = v8667;
                v8603 = v8856;
                v8604 = v8827;
                v8605 = v8917;
                v8606 = v8668;
                v8607 = v8857;
                v8608 = v8828;
            }
            let v8609 = if v2 == v60 { 1.0 } else { 0.0 };
            let v9097: f64;
            let v9098: Lanes<5>;
            if v8609 != 0.0 {
                v9097 = v60;
                v9098 = v508;
            } else {
                let v9068 = v122 * v9066;
                let v9071 = (Lanes([0.0, v9068[0], v9068[1]])) - v123;
                let v9079 = (((v9066 * (v113 + v9064)) - v114) - (v1007 * v9072)) / v9078;
                let v9080 = ((Lanes([0.0, v9071[0], 0.0, v9071[1], v9071[2]])) - (v1008 * v9072)) / v9078;
                let v9082 = v9080 * v9079;
                let v9086 = ((v9079 * v9079) + v9084).sqrt();
                let v9093 = (v9080 + ((v9082 + v9082) * (v184 / (v236 * v9086)))) * v502;
                let v9095 = (v502 * (v9079 + v9086)) + v9094;
                let v9096 = if v9095 < v60 { 1.0 } else { 0.0 };
                let v9099: f64;
                let v9100: Lanes<5>;
                if v9096 != 0.0 {
                    v9099 = v60;
                    v9100 = v508;
                } else {
                    v9099 = v9095;
                    v9100 = v9093;
                }
                let v9102 = -v9101;
                let v9104 = v243 * v9102;
                let v9105 = v9099 + v418;
                let v9106 = (v9102 * v240) / v9105;
                let v9110 = ((Lanes([v9104[0], 0.0, 0.0, 0.0, 0.0])) - (v9100 * v9106)) / v9105;
                let v9112 = if v9106 < v9111 { 1.0 } else { 0.0 };
                let v9167: f64;
                let v9168: Lanes<5>;
                if v9112 != 0.0 {
                    v9167 = v60;
                    v9168 = v508;
                } else {
                    let v9114 = v9113 / v235;
                    let v9120 = (v9114 * v150) * v3387;
                    let v9122 = v9120 * v9099;
                    let v9123 = (((((v239 * v9114) * v95) / v235) * v150) * v3387) * v9099;
                    let v9127 = v9122 * v9099;
                    let v9131 = v9106.exp();
                    let v9137 = -v151;
                    let v9140 = (v154 * v95) * v113;
                    let v9141 = v122 * v9137;
                    let v9145 = (v9137 * v113).exp();
                    let v9147 = v89 + v9145;
                    let v9148 = (v9127 * v9131) / v9147;
                    let v9149 = (((Lanes([v9140[0], 0.0, 0.0])) + (Lanes([0.0, v9141[0], v9141[1]]))) * v9145) * v9148;
                    let v9154 = (-v3295) / v171;
                    let v9158 = v9154.exp();
                    let v9160 = v89 - v9158;
                    let v9162 = v9148 / v9160;
                    let v9163 = (((((v172 * v9154) * v95) / v171) * v9158) * v95) * v9162;
                    let v9166 = (((((((((Lanes([v9123[0], 0.0, 0.0, 0.0, 0.0])) + (v9100 * v9120)) * v9099) + (v9100 * v9122)) * v9131) + ((v9110 * v9131) * v9127)) - (Lanes([v9149[0], 0.0, 0.0, v9149[1], v9149[2]]))) / v9147) - (Lanes([v9163[0], 0.0, 0.0, 0.0, 0.0]))) / v9160;
                    v9167 = v9162;
                    v9168 = v9166;
                }
                v9097 = v9167;
                v9098 = v9168;
            }
            let v9203: f64;
            let v9204: Lanes<5>;
            if v8609 != 0.0 {
                v9203 = v60;
                v9204 = v508;
            } else {
                let v9173 = (v122 * v95) * v9066;
                let v9179 = (Lanes([0.0, v9173[0], v9173[1]])) - (v123 - (Lanes([0.0, v122[0], v122[1]])));
                let v9185 = (((v9066 * ((-v113) + v9064)) - (v114 - v113)) - (v1007 * v9072)) / v9078;
                let v9186 = ((Lanes([0.0, v9179[0], 0.0, v9179[1], v9179[2]])) - (v1008 * v9072)) / v9078;
                let v9188 = v9186 * v9185;
                let v9192 = ((v9185 * v9185) + v9190).sqrt();
                let v9199 = (v9186 + ((v9188 + v9188) * (v184 / (v236 * v9192)))) * v502;
                let v9201 = (v502 * (v9185 + v9192)) + v9200;
                let v9202 = if v9201 < v60 { 1.0 } else { 0.0 };
                let v9206: f64;
                let v9207: Lanes<5>;
                if v9202 != 0.0 {
                    v9206 = v60;
                    v9207 = v508;
                } else {
                    v9206 = v9201;
                    v9207 = v9199;
                }
                let v9208 = -v9101;
                let v9210 = v243 * v9208;
                let v9211 = v9206 + v418;
                let v9212 = (v9208 * v240) / v9211;
                let v9216 = ((Lanes([v9210[0], 0.0, 0.0, 0.0, 0.0])) - (v9207 * v9212)) / v9211;
                let v9218 = if v9212 < v9217 { 1.0 } else { 0.0 };
                let v9270: f64;
                let v9271: Lanes<5>;
                if v9218 != 0.0 {
                    v9270 = v60;
                    v9271 = v508;
                } else {
                    let v9219 = v9113 / v235;
                    let v9225 = (v9219 * v150) * v3387;
                    let v9227 = v9225 * v9206;
                    let v9228 = (((((v239 * v9219) * v95) / v235) * v150) * v3387) * v9206;
                    let v9232 = v9227 * v9206;
                    let v9236 = v9212.exp();
                    let v9243 = v154 * v113;
                    let v9244 = v122 * v151;
                    let v9248 = (v151 * v113).exp();
                    let v9250 = v89 + v9248;
                    let v9251 = (v9232 * v9236) / v9250;
                    let v9252 = (((Lanes([v9243[0], 0.0, 0.0])) + (Lanes([0.0, v9244[0], v9244[1]]))) * v9248) * v9251;
                    let v9257 = (-v3295) / v171;
                    let v9261 = v9257.exp();
                    let v9263 = v89 - v9261;
                    let v9265 = v9251 / v9263;
                    let v9266 = (((((v172 * v9257) * v95) / v171) * v9261) * v95) * v9265;
                    let v9269 = (((((((((Lanes([v9228[0], 0.0, 0.0, 0.0, 0.0])) + (v9207 * v9225)) * v9206) + (v9207 * v9227)) * v9236) + ((v9216 * v9236) * v9232)) - (Lanes([v9252[0], 0.0, 0.0, v9252[1], v9252[2]]))) / v9250) - (Lanes([v9266[0], 0.0, 0.0, 0.0, 0.0]))) / v9263;
                    v9270 = v9265;
                    v9271 = v9269;
                }
                v9203 = v9270;
                v9204 = v9271;
            }
            let v9205 = if v6694 != v60 { 1.0 } else { 0.0 };
            let v9286: f64;
            let v9287: f64;
            let v9288: Lanes<6>;
            let v9289: Lanes<6>;
            if v9205 != 0.0 {
                let v9272 = v113 + v4802;
                let v9273 = v6961 + v4806;
                let v9276 = v89 - v6832;
                let v9279 = (v6832 * v9272) + (v9276 * v6682);
                let v9280 = (v9273 * v6832) + (v6685 * v9276);
                let v9283 = if v9279 > (v9272 - v9281) { 1.0 } else { 0.0 };
                let v9301: f64;
                let v9302: Lanes<6>;
                if v9283 != 0.0 {
                    let v9300 = v9272 - v9299;
                    v9301 = v9300;
                    v9302 = v9273;
                } else {
                    v9301 = v9279;
                    v9302 = v9280;
                }
                v9286 = v9301;
                v9287 = v60;
                v9288 = v9302;
                v9289 = v3227;
            } else {
                let v9285 = if v9284 != v60 { 1.0 } else { 0.0 };
                let v9305: f64;
                let v9306: Lanes<6>;
                if v9285 != 0.0 {
                    let v9304 = if v6740 > v9303 { 1.0 } else { 0.0 };
                    let v9318: f64;
                    let v9319: Lanes<6>;
                    if v9304 != 0.0 {
                        let v9309 = v161 * v6740;
                        let v9314 = ((v6740 * v158) / v3295) / v4813;
                        let v9317 = ((((v6741 * v158) + (Lanes([v9309[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v3295) - (v4815 * v9314)) / v4813;
                        v9318 = v9314;
                        v9319 = v9317;
                    } else {
                        v9318 = v60;
                        v9319 = v3227;
                    }
                    v9305 = v9318;
                    v9306 = v9319;
                } else {
                    v9305 = v60;
                    v9306 = v3227;
                }
                v9286 = v6807;
                v9287 = v9305;
                v9288 = v6809;
                v9289 = v9306;
            }
            let v9290 = v89 / v589;
            let v9294 = if v9293 > v60 { 1.0 } else { 0.0 };
            let v9298 = if (if (if v9291 >= v89 { 1.0 } else { 0.0 }) != 0.0 && v9294 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v9296 > v60 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v9340: f64;
            let v9341: f64;
            let v9342: f64;
            let v9343: f64;
            let v9344: f64;
            let v9345: f64;
            let v9346: Lanes<3>;
            let v9347: Lanes<3>;
            let v9348: Lanes<4>;
            let v9349: Lanes<4>;
            let v9350: Lanes<4>;
            let v9351: Lanes<4>;
            if v9298 != 0.0 {
                let v9322 = (v9296 / v9320).sqrt();
                let v9323 = v307 * v9322;
                let v9324 = v308 * v9322;
                let v9327 = v115 + (v9325 * v116);
                let v9329 = v116 + (v9325 * v115);
                let v9362: f64;
                let v9363: Lanes<3>;
                if v9330 != 0.0 {
                    let v9359 = (v115 * v117) + (v116 * (v117 - v118));
                    let v9360 = (v124 * v115) + ((v124 - (Lanes([v125[0], v125[1], 0.0]))) * v116);
                    v9362 = v9359;
                    v9363 = v9360;
                } else {
                    v9362 = v60;
                    v9363 = v9361;
                }
                let v9373: f64;
                let v9374: Lanes<3>;
                if v9325 != 0.0 {
                    let v9371 = (v116 * v117) + (v115 * (v117 - v118));
                    let v9372 = (v124 * v116) + ((v124 - (Lanes([v125[0], v125[1], 0.0]))) * v115);
                    v9373 = v9371;
                    v9374 = v9372;
                } else {
                    v9373 = v9362;
                    v9374 = v9363;
                }
                let v9376 = if v9375 > v327 { 1.0 } else { 0.0 };
                let v9391: f64;
                if v9376 != 0.0 {
                    let v9378 = v330 - v327;
                    let v9379 = (v9375 - v327) / v9378;
                    let v9380 = v9379 * v9379;
                    let v9390 = v327 + (v9378 * (v89 - (v89 / ((((v89 + v9379) + v9380) + (v9380 * v9379)) + (v9380 * v9380)))));
                    v9391 = v9390;
                } else {
                    v9391 = v9375;
                }
                let v9393 = (-v9391) - v407;
                let v9394 = v9323 * v9290;
                let v9395 = v9324 * v9290;
                let v9396 = v9394 * v9394;
                let v9397 = v9395 * v9394;
                let v9398 = v9397 + v9397;
                let v9400 = v9374 * v95;
                let v9402 = (-v9373) + v9401;
                let v9403 = v9296 / v265;
                let v9407 = v9403.ln();
                let v9410 = v1183 * v9407;
                let v9413 = (v1186 * v9407) + (((((v268 * v9403) * v95) / v265) * (v184 / v9403)) * v1183);
                let v9414 = -v9393;
                let v9415 = if v9402 < v9414 { 1.0 } else { 0.0 };
                let v9489: f64;
                let v9490: f64;
                let v9491: f64;
                let v9492: f64;
                let v9493: f64;
                let v9494: Lanes<4>;
                let v9495: Lanes<4>;
                let v9496: Lanes<4>;
                let v9497: Lanes<4>;
                let v9498: Lanes<4>;
                if v9415 != 0.0 {
                    let v9416 = v151 * v9323;
                    let v9420 = v589 / v9416;
                    let v9423 = ((((v154 * v9323) + (v9324 * v151)) * v9420) * v95) / v9416;
                    let v9426 = v9423 * v9424;
                    let v9427 = v254 + (v9424 * v9420);
                    let v9429 = v9428 * v9427;
                    let v9431 = v9429 * v9427;
                    let v9435 = v9431 * v9427;
                    let v9438 = ((((v9426 * v9428) * v9427) + (v9426 * v9429)) * v9427) + (v9426 * v9431);
                    let v9439 = v145 - v9410;
                    let v9440 = v146 - v9413;
                    let v9441 = v9402 + v9393;
                    let v9443 = v154 * v9441;
                    let v9444 = v9400 * v151;
                    let v9449 = v9448 * v9420;
                    let v9451 = (v151 * v9441) - v254;
                    let v9452 = v9449 * v9451;
                    let v9453 = (v9423 * v9448) * v9451;
                    let v9456 = (Lanes([0.0, 0.0, v9453[0], 0.0])) + (((Lanes([0.0, 0.0, v9443[0], 0.0])) + (Lanes([v9444[0], v9444[1], 0.0, v9444[2]]))) * v9449);
                    let v9458 = v9457 - v9452;
                    let v9459 = v9456 * v95;
                    let v9460 = v9458 * v9458;
                    let v9461 = v9459 * v9458;
                    let v9462 = v9461 + v9461;
                    let v9464 = if v9435 < (v9460 * v1270) { 1.0 } else { 0.0 };
                    let v9524: f64;
                    let v9525: Lanes<4>;
                    if v9464 != 0.0 {
                        let v9503 = v9438 * v502;
                        let v9504 = (v502 * v9435) / v9458;
                        let v9511 = ((v9500 + v9458) + v9504) + v9452;
                        let v9512 = (v9459 + (((Lanes([0.0, 0.0, v9503[0], 0.0])) - (v9459 * v9504)) / v9458)) + v9456;
                        v9524 = v9511;
                        v9525 = v9512;
                    } else {
                        let v9516 = (v9435 + v9460).sqrt();
                        let v9522 = (v9520 + v9516) + v9452;
                        let v9523 = (((Lanes([0.0, 0.0, v9438[0], 0.0])) + v9462) * (v184 / (v236 * v9516))) + v9456;
                        v9524 = v9522;
                        v9525 = v9523;
                    }
                    let v9527 = v9524.powf(v9526);
                    let v9531 = v9525 * (v9526 * (v9524.powf(v9528)));
                    let v9537 = (v9423 * v9532) * v95;
                    let v9544 = v9543 * v9527;
                    let v9552 = (((v9535 - (v9532 * v9420)) + (v254 * v9527)) + (v9544 * v9527)) / v9527;
                    let v9558 = v161 * v9552;
                    let v9560 = ((((((Lanes([0.0, 0.0, v9537[0], 0.0])) + (v9531 * v254)) + (((v9531 * v9543) * v9527) + (v9531 * v9544))) - (v9531 * v9552)) / v9527) * v158) + (Lanes([0.0, 0.0, v9558[0], 0.0]));
                    let v9562 = ((v9552 * v158) - v9393) + v9393;
                    let v9563 = v9562 / v9439;
                    let v9564 = v9440 * v9563;
                    let v9569 = ((v9560 - (Lanes([0.0, 0.0, v9564[0], 0.0]))) / v9439) * v9563;
                    let v9572 = (v89 + (v9563 * v9563)).sqrt();
                    let v9576 = v9562 / v9572;
                    let v9584 = v589 * (v9402 - (v9576 - v9393));
                    let v9585 = ((Lanes([v9400[0], v9400[1], 0.0, v9400[2]])) - ((v9560 - (((v9569 + v9569) * (v184 / (v236 * v9572))) * v9576)) / v9572)) * v589;
                    v9489 = v9584;
                    v9490 = v9584;
                    v9491 = v60;
                    v9492 = v60;
                    v9493 = v60;
                    v9494 = v9585;
                    v9495 = v9585;
                    v9496 = v9586;
                    v9497 = v9586;
                    v9498 = v9586;
                } else {
                    let v9465 = v9402 + v9393;
                    let v9467 = v154 * v9465;
                    let v9468 = v9400 * v151;
                    let v9470 = Lanes([v9468[0], v9468[1], 0.0, v9468[2]]);
                    let v9471 = (Lanes([0.0, 0.0, v9467[0], 0.0])) + v9470;
                    let v9472 = (v151 * v9465) - v89;
                    let v9477 = v9396 * v155;
                    let v9480 = (v9398 * v155) + (v157 * v9396);
                    let v9481 = (v446 * (v9472 + v9473)) / v9477;
                    let v9482 = v9480 * v9481;
                    let v9485 = ((v9471 * v446) - (Lanes([0.0, 0.0, v9482[0], 0.0]))) / v9477;
                    let v9486 = v89 + v9481;
                    let v9488 = if v9486 < v9487 { 1.0 } else { 0.0 };
                    let v9588: f64;
                    let v9589: Lanes<4>;
                    if v9488 != 0.0 {
                        v9588 = v9587;
                        v9589 = v9586;
                    } else {
                        v9588 = v9486;
                        v9589 = v9485;
                    }
                    let v9594 = (v9396 * v151) / v254;
                    let v9595 = ((v9398 * v151) + (v154 * v9396)) / v254;
                    let v9596 = v9588.sqrt();
                    let v9600 = v89 - v9596;
                    let v9603 = v9595 * v9600;
                    let v9608 = Lanes([v9400[0], v9400[1], 0.0, v9400[2]]);
                    let v9610 = (v9402 + (v9594 * v9600)) + v9393;
                    let v9612 = v154 * v9610;
                    let v9618 = (-(v151 * v9610)).exp();
                    let v9624 = (v446 * (v9472 + v9618)) / v9477;
                    let v9625 = v9480 * v9624;
                    let v9628 = (((v9471 + ((((Lanes([0.0, 0.0, v9612[0], 0.0])) + ((v9608 + ((Lanes([0.0, 0.0, v9603[0], 0.0])) + (((v9589 * (v184 / (v236 * v9596))) * v95) * v9594))) * v151)) * v95) * v9618)) * v446) - (Lanes([0.0, 0.0, v9625[0], 0.0]))) / v9477;
                    let v9629 = v89 + v9624;
                    let v9631 = if v9629 < v9630 { 1.0 } else { 0.0 };
                    let v9633: f64;
                    let v9634: Lanes<4>;
                    if v9631 != 0.0 {
                        v9633 = v9632;
                        v9634 = v9586;
                    } else {
                        v9633 = v9629;
                        v9634 = v9628;
                    }
                    let v9635 = v9633.sqrt();
                    let v9639 = v89 - v9635;
                    let v9642 = v9595 * v9639;
                    let v9648 = (v9402 + (v9594 * v9639)) + v9393;
                    let v9649 = v151 * v9648;
                    let v9650 = v154 * v9648;
                    let v9653 = (Lanes([0.0, 0.0, v9650[0], 0.0])) + ((v9608 + ((Lanes([0.0, 0.0, v9642[0], 0.0])) + (((v9634 * (v184 / (v236 * v9635))) * v95) * v9594))) * v151);
                    let v9654 = if v9649 < v443 { 1.0 } else { 0.0 };
                    let v9747: f64;
                    let v9748: Lanes<4>;
                    if v9654 != 0.0 {
                        let v9655 = v151 * v9394;
                        let v9659 = v89 / v9655;
                        let v9662 = ((((v154 * v9394) + (v9395 * v151)) * v9659) * v95) / v9655;
                        let v9664 = v9663 + v9659;
                        let v9666 = v9400 * v95;
                        let v9667 = (-v9465) / v9394;
                        let v9668 = v9395 * v9667;
                        let v9681 = ((v9662 * v9673) / v9676) * v95;
                        let v9685 = (v9679 - ((v9673 * v9664) / v9676)) + (v9667 / v9682);
                        let v9687 = (Lanes([0.0, 0.0, v9681[0], 0.0])) + ((((Lanes([v9666[0], v9666[1], 0.0, v9666[2]])) - (Lanes([0.0, 0.0, v9668[0], 0.0]))) / v9394) / v9682);
                        let v9694 = ((v9688 * v9664) - v9691) / v9693;
                        let v9695 = (v9662 * v9688) / v9693;
                        let v9697 = v9687 * v9685;
                        let v9699 = v9694 * v9694;
                        let v9700 = v9695 * v9694;
                        let v9705 = ((v9700 + v9700) * v9694) + (v9695 * v9699);
                        let v9709 = ((v9685 * v9685) + (v9699 * v9694)).sqrt();
                        let v9712 = ((v9697 + v9697) + (Lanes([0.0, 0.0, v9705[0], 0.0]))) * (v184 / (v236 * v9709));
                        let v9715 = (-v9685) + v9709;
                        let v9722 = v9685 + v9709;
                        let v9734 = ((v9715.powf(v9526)) + (-(v9722.powf(v9526)))) - v9733;
                        let v9737 = v161 * v9734;
                        let v9741 = ((v9734 * v158) - v9393) + v9393;
                        let v9742 = v151 * v9741;
                        let v9743 = v154 * v9741;
                        let v9746 = (Lanes([0.0, 0.0, v9743[0], 0.0])) + (((((((v9687 * v95) + v9712) * (v9526 * (v9715.powf(v9718)))) + (((v9687 + v9712) * (v9526 * (v9722.powf(v9725)))) * v95)) * v158) + (Lanes([0.0, 0.0, v9737[0], 0.0]))) * v151);
                        v9747 = v9742;
                        v9748 = v9746;
                    } else {
                        v9747 = v9649;
                        v9748 = v9653;
                    }
                    let v9750 = if v9749 > v60 { 1.0 } else { 0.0 };
                    let v9804: f64;
                    let v9805: Lanes<4>;
                    if v9750 != 0.0 {
                        let v9751 = v9465 + v210;
                        let v9754 = (v151 * v9414).exp();
                        let v9756 = v9754 + v418;
                        let v9757 = v265 / v9296;
                        let v9759 = v9757 * v9757;
                        let v9760 = (v268 / v9296) * v9757;
                        let v9761 = v9760 + v9760;
                        let v9762 = v9759 * v9756;
                        let v9766 = v151 * v9751;
                        let v9767 = v154 * v9751;
                        let v9769 = (Lanes([0.0, 0.0, v9767[0], 0.0])) + v9470;
                        let v9770 = v9762 * v9477;
                        let v9773 = (((v9761 * v9756) + (((v154 * v9414) * v9754) * v9759)) * v9477) + (v9480 * v9762);
                        let v9775 = v9769 * v9766;
                        let v9777 = v9770 + (v9766 * v9766);
                        let v9778 = Lanes([0.0, 0.0, v9773[0], 0.0]);
                        let v9783 = v9759 * v9477;
                        let v9787 = v9783.ln();
                        let v9789 = ((v9761 * v9477) + (v9480 * v9759)) * (v184 / v9783);
                        let v9791 = Lanes([0.0, 0.0, v9789[0], 0.0]);
                        let v9793 = v151 * v9393;
                        let v9794 = v154 * v9393;
                        let v9796 = Lanes([0.0, 0.0, v9794[0], 0.0]);
                        let v9799 = v9769 - ((((v9778 + (v9775 + v9775)) * (v184 / v9777)) - v9791) + v9796);
                        let v9800 = (v9766 - (((v9777.ln()) - v9787) + v9793)) - v89;
                        let v9801 = v446 * v9766;
                        let v9802 = v9769 * v446;
                        let v9803 = if v9801 > v60 { 1.0 } else { 0.0 };
                        let v9823: f64;
                        let v9824: Lanes<4>;
                        if v9803 != 0.0 {
                            v9823 = v9801;
                            v9824 = v9802;
                        } else {
                            let v9821 = -v9801;
                            let v9822 = v9802 * v95;
                            v9823 = v9821;
                            v9824 = v9822;
                        }
                        let v9826 = v9799 * v9800;
                        let v9830 = ((v9800 * v9800) + v9823).sqrt();
                        let v9843 = v154 * v210;
                        let v9844 = (v9766 - (v9766 - (v502 * (v9800 + v9830)))) + (v151 * v210);
                        let v9848 = ((v9769 - (v9769 - ((v9799 + (((v9826 + v9826) + v9824) * (v184 / (v236 * v9830)))) * v502))) + (Lanes([0.0, 0.0, v9843[0], 0.0]))) * v9844;
                        let v9850 = v9770 + (v9844 * v9844);
                        let v9857 = ((v9850.ln()) - v9787) + v9793;
                        let v9858 = (((v9778 + (v9848 + v9848)) * (v184 / v9850)) - v9791) + v9796;
                        let v9860 = v9858 - v9748;
                        let v9862 = (v9857 - v9747) - v9861;
                        let v9866 = (v446 * v9857) * v9865;
                        let v9867 = (v9858 * v446) * v9865;
                        let v9868 = if v9866 > v60 { 1.0 } else { 0.0 };
                        let v9871: f64;
                        let v9872: Lanes<4>;
                        if v9868 != 0.0 {
                            v9871 = v9866;
                            v9872 = v9867;
                        } else {
                            let v9869 = -v9866;
                            let v9870 = v9867 * v95;
                            v9871 = v9869;
                            v9872 = v9870;
                        }
                        let v9874 = v9860 * v9862;
                        let v9878 = ((v9862 * v9862) + v9871).sqrt();
                        let v9886 = v9857 - (v502 * (v9862 + v9878));
                        let v9887 = v9858 - ((v9860 + (((v9874 + v9874) + v9872) * (v184 / (v236 * v9878)))) * v502);
                        v9804 = v9886;
                        v9805 = v9887;
                    } else {
                        v9804 = v9747;
                        v9805 = v9748;
                    }
                    let v9806 = v9804 / v151;
                    let v9807 = v154 * v9806;
                    let v9810 = (v9805 - (Lanes([0.0, 0.0, v9807[0], 0.0]))) / v151;
                    let v9811 = v9806 - v9393;
                    let v9815 = (-v9804).exp();
                    let v9817 = (v9804 - v89) + v9815;
                    let v9818 = v9805 + ((v9805 * v95) * v9815);
                    let v9820 = if v9817 < v9819 { 1.0 } else { 0.0 };
                    let v9889: f64;
                    let v9890: Lanes<4>;
                    if v9820 != 0.0 {
                        v9889 = v9888;
                        v9890 = v9586;
                    } else {
                        v9889 = v9817;
                        v9890 = v9818;
                    }
                    let v9891 = v9889.sqrt();
                    let v9895 = v9323 * v9891;
                    let v9896 = v9324 * v9891;
                    let v9899 = (Lanes([0.0, 0.0, v9896[0], 0.0])) + ((v9890 * (v184 / (v236 * v9891))) * v9323);
                    let v9902 = v589 * (v9402 - v9811);
                    let v9903 = (v9608 - v9810) * v589;
                    let v9904 = if v9749 == v89 { 1.0 } else { 0.0 };
                    let v9918: f64;
                    let v9919: f64;
                    let v9920: f64;
                    let v9921: f64;
                    let v9922: f64;
                    let v9923: Lanes<4>;
                    let v9924: Lanes<4>;
                    let v9925: Lanes<4>;
                    let v9926: Lanes<4>;
                    let v9927: Lanes<4>;
                    if v9904 != 0.0 {
                        let v9907 = (v151 * v9414).exp();
                        let v9908 = (v154 * v9414) * v9907;
                        let v9909 = v265 / v9296;
                        let v9911 = v9909 * v9909;
                        let v9912 = (v268 / v9296) * v9909;
                        let v9913 = v9912 + v9912;
                        let v9914 = v9911 * v9907;
                        let v9917 = (v9913 * v9907) + (v9908 * v9911);
                        let mut v9928: f64 = 0.0;
                        let mut v9929: f64 = 0.0;
                        let mut v9930: f64 = 0.0;
                        let mut v9931: f64 = 0.0;
                        let mut v9932: f64 = 0.0;
                        let mut v9933: f64 = 0.0;
                        let mut v9934: f64 = 0.0;
                        let mut v9935: Lanes<4> = Lanes([0.0; 4]);
                        let mut v9936: Lanes<4> = Lanes([0.0; 4]);
                        let mut v9937: Lanes<4> = Lanes([0.0; 4]);
                        let mut v9938: Lanes<4> = Lanes([0.0; 4]);
                        let mut v9939: Lanes<4> = Lanes([0.0; 4]);
                        v9928 = v89;
                        v9929 = v9811;
                        v9930 = v60;
                        v9931 = v9804;
                        v9932 = v60;
                        v9933 = v60;
                        v9934 = v60;
                        v9935 = v9810;
                        v9936 = v9805;
                        v9937 = v9586;
                        v9938 = v9586;
                        v9939 = v9586;
                        loop {
                            let v9941 = if v9928 <= v9940 { 1.0 } else { 0.0 };
                            if v9941 == 0.0 {
                                break;
                            }
                            let v9942 = v9929 + v9393;
                            let v9943 = v151 * v9942;
                            let v9944 = v154 * v9942;
                            let v9945 = v9935 * v151;
                            let v9947 = (Lanes([0.0, 0.0, v9944[0], 0.0])) + v9945;
                            let v9948 = if v9943 < v4766 { 1.0 } else { 0.0 };
                            let v10090: f64;
                            let v10091: f64;
                            let v10092: f64;
                            let v10093: f64;
                            let v10094: Lanes<4>;
                            let v10095: Lanes<4>;
                            let v10096: Lanes<4>;
                            let v10097: Lanes<4>;
                            if v9948 != 0.0 {
                                let v9950 = v9943 * v9943;
                                let v9951 = v9947 * v9943;
                                let v9952 = v9951 + v9951;
                                let v9953 = v9950 * v9943;
                                let v9961 = v9960 + (v9943 * v9957);
                                let v9967 = v9966 + (v9943 * v9961);
                                let v9968 = v9953 * v9967;
                                let v9971 = (((v9952 * v9943) + (v9947 * v9950)) * v9967) + (((v9947 * v9961) + ((v9947 * v9957) * v9943)) * v9953);
                                let v9972 = v9943 * v4766;
                                let v9973 = v9947 * v4766;
                                let v9977 = v9976 + (v9972 * v9957);
                                let v9983 = v9982 + (v9943 * v9977);
                                let v9984 = v9950 * v9983;
                                let v9988 = v9914 * v9968;
                                let v9989 = v9917 * v9968;
                                let v9993 = v9988 * v9968;
                                let v9996 = (((Lanes([0.0, 0.0, v9989[0], 0.0])) + (v9971 * v9914)) * v9968) + (v9971 * v9988);
                                let v10001 = (v9914 * v151) * v254;
                                let v10003 = v10001 * v9968;
                                let v10004 = (((v9917 * v151) + (v154 * v9914)) * v254) * v9968;
                                let v10016 = v10015 + (v9943 * v10012);
                                let v10022 = v10021 + (v9943 * v10016);
                                let v10028 = v10027 + (v9943 * v10022);
                                let v10034 = v10033 + (v9943 * v10028);
                                let v10035 = v9943 * v10034;
                                let v10038 = (v9947 * v10034) + (((v9947 * v10028) + (((v9947 * v10022) + (((v9947 * v10016) + ((v9947 * v10012) * v9943)) * v9943)) * v9943)) * v9943);
                                let v10042 = v10041 + (v9972 * v10012);
                                let v10048 = v10047 + (v9943 * v10042);
                                let v10054 = v10053 + (v9943 * v10048);
                                let v10059 = v10033 + (v9943 * v10054);
                                let v10061 = v10038 * v10035;
                                let v10066 = (((v10035 * v10035) + v9993) + v418).sqrt();
                                let v10069 = ((v10061 + v10061) + v9996) * (v184 / (v236 * v10066));
                                let v10071 = v154 * v10059;
                                let v10075 = (v151 * v10059) * v254;
                                let v10083 = v10066 + v10066;
                                let v10085 = ((v10075 * v10035) + (v10003 * v9984)) / v10083;
                                let v10088 = (((((((Lanes([0.0, 0.0, v10071[0], 0.0])) + (((v9947 * v10054) + (((v9947 * v10048) + (((v9947 * v10042) + ((v9973 * v10012) * v9943)) * v9943)) * v9943)) * v151)) * v254) * v10035) + (v10038 * v10075)) + ((((Lanes([0.0, 0.0, v10004[0], 0.0])) + (v9971 * v10001)) * v9984) + (((v9952 * v9983) + (((v9947 * v9977) + ((v9973 * v9957) * v9943)) * v9950)) * v10003))) - ((v10069 + v10069) * v10085)) / v10083;
                                v10090 = v10066;
                                v10091 = v10085;
                                v10092 = v10035;
                                v10093 = v9993;
                                v10094 = v10069;
                                v10095 = v10088;
                                v10096 = v10038;
                                v10097 = v9996;
                            } else {
                                let v10089 = if v9943 < v3883 { 1.0 } else { 0.0 };
                                let v10156: f64;
                                let v10157: f64;
                                let v10158: Lanes<4>;
                                let v10159: Lanes<4>;
                                if v10089 != 0.0 {
                                    let v10116 = v9943.exp();
                                    let v10117 = v9947 * v10116;
                                    let v10118 = v10116 - v89;
                                    let v10119 = v9914 * v10118;
                                    let v10120 = v9917 * v10118;
                                    let v10123 = (Lanes([0.0, 0.0, v10120[0], 0.0])) + (v10117 * v9914);
                                    let v10124 = v9914 * v151;
                                    let v10128 = v10124 * v10116;
                                    let v10129 = ((v9917 * v151) + (v154 * v9914)) * v10116;
                                    let v10132 = (Lanes([0.0, 0.0, v10129[0], 0.0])) + (v10117 * v10124);
                                    v10156 = v10119;
                                    v10157 = v10128;
                                    v10158 = v10123;
                                    v10159 = v10132;
                                } else {
                                    let v10134 = v154 * v9929;
                                    let v10137 = (v151 * v9929).exp();
                                    let v10138 = ((Lanes([0.0, 0.0, v10134[0], 0.0])) + v9945) * v10137;
                                    let v10139 = v10137 - v9907;
                                    let v10142 = v9911 * v10139;
                                    let v10143 = v9913 * v10139;
                                    let v10146 = (Lanes([0.0, 0.0, v10143[0], 0.0])) + ((v10138 - (Lanes([0.0, 0.0, v9908[0], 0.0]))) * v9911);
                                    let v10147 = v9911 * v151;
                                    let v10151 = v10147 * v10137;
                                    let v10152 = ((v9913 * v151) + (v154 * v9911)) * v10137;
                                    let v10155 = (Lanes([0.0, 0.0, v10152[0], 0.0])) + (v10138 * v10147);
                                    v10156 = v10142;
                                    v10157 = v10151;
                                    v10158 = v10146;
                                    v10159 = v10155;
                                }
                                let v10163 = ((v9943 - v89) + v10156).sqrt();
                                let v10166 = (v9947 + v10158) * (v184 / (v236 * v10163));
                                let v10170 = (v151 + v10157) / v10163;
                                let v10174 = v10170 * v502;
                                let v10175 = ((((Lanes([0.0, 0.0, v154[0], 0.0])) + v10159) - (v10166 * v10170)) / v10163) * v502;
                                v10090 = v10163;
                                v10091 = v10174;
                                v10092 = v9932;
                                v10093 = v10156;
                                v10094 = v10166;
                                v10095 = v10175;
                                v10096 = v9937;
                                v10097 = v10158;
                            }
                            let v10101 = v9395 * v10090;
                            let v10105 = (v9402 - v9929) - (v9394 * v10090);
                            let v10106 = (v9608 - v9935) - ((Lanes([0.0, 0.0, v10101[0], 0.0])) + (v10094 * v9394));
                            let v10108 = v9395 * v10091;
                            let v10113 = v10112 - (v9394 * v10091);
                            let v10114 = ((Lanes([0.0, 0.0, v10108[0], 0.0])) + (v10095 * v9394)) * v95;
                            let v10115 = if v9930 == v89 { 1.0 } else { 0.0 };
                            let v10189: f64;
                            let v10190: f64;
                            let v10191: f64;
                            let v10192: Lanes<4>;
                            if v10115 != 0.0 {
                                v10189 = v10176;
                                v10190 = v9929;
                                v10191 = v9930;
                                v10192 = v9935;
                            } else {
                                let v10179 = (-v10105) / v10113;
                                let v10182 = ((v10106 * v95) - (v10114 * v10179)) / v10113;
                                let v10183 = v9929.abs();
                                let v10187 = v9935 * ((v236 * (if v9929 >= v4732 { 1.0 } else { 0.0 })) - v184);
                                let v10188 = if v89 >= v10183 { 1.0 } else { 0.0 };
                                let v10194: f64;
                                let v10195: Lanes<4>;
                                if v10188 != 0.0 {
                                    v10194 = v89;
                                    v10195 = v9586;
                                } else {
                                    v10194 = v10183;
                                    v10195 = v10187;
                                }
                                let v10198 = v10197 * (v89 + v10194);
                                let v10199 = v10195 * v10197;
                                let v10201 = if (v10179.abs()) > v10198 { 1.0 } else { 0.0 };
                                let v10203: f64;
                                let v10204: Lanes<4>;
                                if v10201 != 0.0 {
                                    let v10202 = if v10179 >= v60 { 1.0 } else { 0.0 };
                                    let v10213: f64;
                                    if v10202 != 0.0 {
                                        v10213 = v89;
                                    } else {
                                        v10213 = v10212;
                                    }
                                    let v10214 = v10198 * v10213;
                                    let v10215 = v10199 * v10213;
                                    v10203 = v10214;
                                    v10204 = v10215;
                                } else {
                                    v10203 = v10179;
                                    v10204 = v10182;
                                }
                                let v10205 = v9929 + v10203;
                                let v10206 = v9935 + v10204;
                                let v10211 = if (if (v10203.abs()) <= v407 { 1.0 } else { 0.0 }) != 0.0 && (if (v10105.abs()) <= v1270 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let v10216: f64;
                                if v10211 != 0.0 {
                                    v10216 = v89;
                                } else {
                                    v10216 = v9930;
                                }
                                v10189 = v9928;
                                v10190 = v10205;
                                v10191 = v10216;
                                v10192 = v10206;
                            }
                            let v10193 = v10189 + v89;
                            v9928 = v10193;
                            v9929 = v10190;
                            v9930 = v10191;
                            v9931 = v9943;
                            v9932 = v10092;
                            v9933 = v10090;
                            v9934 = v10093;
                            v9935 = v10192;
                            v9936 = v9947;
                            v9937 = v10096;
                            v9938 = v10094;
                            v9939 = v10097;
                        }
                        let v9949 = if v9930 == v60 { 1.0 } else { 0.0 };
                        let v10217 = if v9931 < v4766 { 1.0 } else { 0.0 };
                        let v10224: f64;
                        let v10225: Lanes<4>;
                        if v10217 != 0.0 {
                            let v10218 = if v9931 < v443 { 1.0 } else { 0.0 };
                            let v10249 = v9932 + v10248;
                            v10224 = v10249;
                            v10225 = v9937;
                        } else {
                            let v10220 = (v9931 - v89).sqrt();
                            let v10223 = v9936 * (v184 / (v236 * v10220));
                            v10224 = v10220;
                            v10225 = v10223;
                        }
                        let v10226 = v9323 * v10224;
                        let v10227 = v9324 * v10224;
                        let v10230 = (Lanes([0.0, 0.0, v10227[0], 0.0])) + (v10225 * v9323);
                        let v10231 = v9933 + v10224;
                        let v10233 = v89 / v10231;
                        let v10237 = v9323 * v9934;
                        let v10238 = v9324 * v9934;
                        let v10246 = v10226 + (v10237 * v10233);
                        let v10247 = v10230 + ((((Lanes([0.0, 0.0, v10238[0], 0.0])) + (v9939 * v9323)) * v10233) + (((((v9938 + v10225) * v10233) * v95) / v10231) * v10237));
                        v9918 = v10246;
                        v9919 = v10226;
                        v9920 = v9932;
                        v9921 = v9933;
                        v9922 = v9934;
                        v9923 = v10247;
                        v9924 = v10230;
                        v9925 = v9937;
                        v9926 = v9938;
                        v9927 = v9939;
                    } else {
                        v9918 = v9902;
                        v9919 = v9895;
                        v9920 = v60;
                        v9921 = v60;
                        v9922 = v60;
                        v9923 = v9903;
                        v9924 = v9899;
                        v9925 = v9586;
                        v9926 = v9586;
                        v9927 = v9586;
                    }
                    v9489 = v9918;
                    v9490 = v9919;
                    v9491 = v9920;
                    v9492 = v9921;
                    v9493 = v9922;
                    v9494 = v9923;
                    v9495 = v9924;
                    v9496 = v9925;
                    v9497 = v9926;
                    v9498 = v9927;
                }
                let v9499 = v6957 * v9293;
                let v10254: f64;
                let v10255: f64;
                let v10256: Lanes<4>;
                let v10257: Lanes<4>;
                if v9327 != 0.0 {
                    let v10250 = v9499 * v9489;
                    let v10251 = v9494 * v9499;
                    let v10252 = v9499 * v9490;
                    let v10253 = v9495 * v9499;
                    v10254 = v10250;
                    v10255 = v10252;
                    v10256 = v10251;
                    v10257 = v10253;
                } else {
                    v10254 = v60;
                    v10255 = v60;
                    v10256 = v9586;
                    v10257 = v9586;
                }
                let v10262: f64;
                let v10263: f64;
                let v10264: Lanes<4>;
                let v10265: Lanes<4>;
                if v9329 != 0.0 {
                    let v10258 = v9499 * v9489;
                    let v10259 = v9494 * v9499;
                    let v10260 = v9499 * v9490;
                    let v10261 = v9495 * v9499;
                    v10262 = v10258;
                    v10263 = v10260;
                    v10264 = v10259;
                    v10265 = v10261;
                } else {
                    v10262 = v60;
                    v10263 = v60;
                    v10264 = v9586;
                    v10265 = v9586;
                }
                let v10268 = (v10266 * v115) + v116;
                let v10270 = (v10266 * v116) + v115;
                let v10280: f64;
                let v10281: Lanes<3>;
                if v10266 != 0.0 {
                    let v10278 = (v115 * v117) + (v116 * (v117 - v118));
                    let v10279 = (v124 * v115) + ((v124 - (Lanes([v125[0], v125[1], 0.0]))) * v116);
                    v10280 = v10278;
                    v10281 = v10279;
                } else {
                    v10280 = v9373;
                    v10281 = v9374;
                }
                let v10292: f64;
                let v10293: Lanes<3>;
                if v10282 != 0.0 {
                    let v10290 = (v116 * v117) + (v115 * (v117 - v118));
                    let v10291 = (v124 * v116) + ((v124 - (Lanes([v125[0], v125[1], 0.0]))) * v115);
                    v10292 = v10290;
                    v10293 = v10291;
                } else {
                    v10292 = v10280;
                    v10293 = v10281;
                }
                let v10295 = if v10294 > v327 { 1.0 } else { 0.0 };
                let v10310: f64;
                if v10295 != 0.0 {
                    let v10297 = v330 - v327;
                    let v10298 = (v10294 - v327) / v10297;
                    let v10299 = v10298 * v10298;
                    let v10309 = v327 + (v10297 * (v89 - (v89 / ((((v89 + v10298) + v10299) + (v10299 * v10298)) + (v10299 * v10299)))));
                    v10310 = v10309;
                } else {
                    v10310 = v10294;
                }
                let v10312 = (-v10310) - v407;
                let v10314 = v10293 * v95;
                let v10315 = (-v10292) + v9401;
                let v10316 = -v10312;
                let v10317 = if v10315 < v10316 { 1.0 } else { 0.0 };
                let v10389: f64;
                let v10390: f64;
                let v10391: Lanes<4>;
                let v10392: Lanes<4>;
                if v10317 != 0.0 {
                    let v10318 = v151 * v9323;
                    let v10322 = v589 / v10318;
                    let v10325 = ((((v154 * v9323) + (v9324 * v151)) * v10322) * v95) / v10318;
                    let v10328 = v10325 * v10326;
                    let v10329 = v254 + (v10326 * v10322);
                    let v10330 = v9428 * v10329;
                    let v10332 = v10330 * v10329;
                    let v10336 = v10332 * v10329;
                    let v10339 = ((((v10328 * v9428) * v10329) + (v10328 * v10330)) * v10329) + (v10328 * v10332);
                    let v10340 = v145 - v9410;
                    let v10341 = v146 - v9413;
                    let v10342 = v10315 + v10312;
                    let v10344 = v154 * v10342;
                    let v10345 = v10314 * v151;
                    let v10349 = v9448 * v10322;
                    let v10351 = (v151 * v10342) - v254;
                    let v10352 = v10349 * v10351;
                    let v10353 = (v10325 * v9448) * v10351;
                    let v10356 = (Lanes([0.0, 0.0, v10353[0], 0.0])) + (((Lanes([0.0, 0.0, v10344[0], 0.0])) + (Lanes([v10345[0], v10345[1], 0.0, v10345[2]]))) * v10349);
                    let v10358 = v10357 - v10352;
                    let v10359 = v10356 * v95;
                    let v10360 = v10358 * v10358;
                    let v10361 = v10359 * v10358;
                    let v10362 = v10361 + v10361;
                    let v10364 = if v10336 < (v10360 * v1270) { 1.0 } else { 0.0 };
                    let v10417: f64;
                    let v10418: Lanes<4>;
                    if v10364 != 0.0 {
                        let v10396 = v10339 * v502;
                        let v10397 = (v502 * v10336) / v10358;
                        let v10404 = ((v10393 + v10358) + v10397) + v10352;
                        let v10405 = (v10359 + (((Lanes([0.0, 0.0, v10396[0], 0.0])) - (v10359 * v10397)) / v10358)) + v10356;
                        v10417 = v10404;
                        v10418 = v10405;
                    } else {
                        let v10409 = (v10336 + v10360).sqrt();
                        let v10415 = (v10413 + v10409) + v10352;
                        let v10416 = (((Lanes([0.0, 0.0, v10339[0], 0.0])) + v10362) * (v184 / (v236 * v10409))) + v10356;
                        v10417 = v10415;
                        v10418 = v10416;
                    }
                    let v10419 = v10417.powf(v9526);
                    let v10423 = v10418 * (v9526 * (v10417.powf(v10420)));
                    let v10428 = (v10325 * v9532) * v95;
                    let v10434 = v9543 * v10419;
                    let v10442 = (((v10426 - (v9532 * v10322)) + (v254 * v10419)) + (v10434 * v10419)) / v10419;
                    let v10448 = v161 * v10442;
                    let v10450 = ((((((Lanes([0.0, 0.0, v10428[0], 0.0])) + (v10423 * v254)) + (((v10423 * v9543) * v10419) + (v10423 * v10434))) - (v10423 * v10442)) / v10419) * v158) + (Lanes([0.0, 0.0, v10448[0], 0.0]));
                    let v10452 = ((v10442 * v158) - v10312) + v10312;
                    let v10453 = v10452 / v10340;
                    let v10454 = v10341 * v10453;
                    let v10459 = ((v10450 - (Lanes([0.0, 0.0, v10454[0], 0.0]))) / v10340) * v10453;
                    let v10462 = (v89 + (v10453 * v10453)).sqrt();
                    let v10466 = v10452 / v10462;
                    let v10474 = v589 * (v10315 - (v10466 - v10312));
                    let v10475 = ((Lanes([v10314[0], v10314[1], 0.0, v10314[2]])) - ((v10450 - (((v10459 + v10459) * (v184 / (v236 * v10462))) * v10466)) / v10462)) * v589;
                    v10389 = v10474;
                    v10390 = v10474;
                    v10391 = v10475;
                    v10392 = v10475;
                } else {
                    let v10365 = v10315 + v10312;
                    let v10367 = v154 * v10365;
                    let v10368 = v10314 * v151;
                    let v10370 = Lanes([v10368[0], v10368[1], 0.0, v10368[2]]);
                    let v10371 = (Lanes([0.0, 0.0, v10367[0], 0.0])) + v10370;
                    let v10372 = (v151 * v10365) - v89;
                    let v10377 = v9396 * v155;
                    let v10380 = (v9398 * v155) + (v157 * v9396);
                    let v10381 = (v446 * (v10372 + v10373)) / v10377;
                    let v10382 = v10380 * v10381;
                    let v10385 = ((v10371 * v446) - (Lanes([0.0, 0.0, v10382[0], 0.0]))) / v10377;
                    let v10386 = v89 + v10381;
                    let v10388 = if v10386 < v10387 { 1.0 } else { 0.0 };
                    let v10477: f64;
                    let v10478: Lanes<4>;
                    if v10388 != 0.0 {
                        v10477 = v10476;
                        v10478 = v9586;
                    } else {
                        v10477 = v10386;
                        v10478 = v10385;
                    }
                    let v10483 = (v9396 * v151) / v254;
                    let v10484 = ((v9398 * v151) + (v154 * v9396)) / v254;
                    let v10485 = v10477.sqrt();
                    let v10489 = v89 - v10485;
                    let v10492 = v10484 * v10489;
                    let v10497 = Lanes([v10314[0], v10314[1], 0.0, v10314[2]]);
                    let v10499 = (v10315 + (v10483 * v10489)) + v10312;
                    let v10501 = v154 * v10499;
                    let v10507 = (-(v151 * v10499)).exp();
                    let v10513 = (v446 * (v10372 + v10507)) / v10377;
                    let v10514 = v10380 * v10513;
                    let v10517 = (((v10371 + ((((Lanes([0.0, 0.0, v10501[0], 0.0])) + ((v10497 + ((Lanes([0.0, 0.0, v10492[0], 0.0])) + (((v10478 * (v184 / (v236 * v10485))) * v95) * v10483))) * v151)) * v95) * v10507)) * v446) - (Lanes([0.0, 0.0, v10514[0], 0.0]))) / v10377;
                    let v10518 = v89 + v10513;
                    let v10520 = if v10518 < v10519 { 1.0 } else { 0.0 };
                    let v10522: f64;
                    let v10523: Lanes<4>;
                    if v10520 != 0.0 {
                        v10522 = v10521;
                        v10523 = v9586;
                    } else {
                        v10522 = v10518;
                        v10523 = v10517;
                    }
                    let v10524 = v10522.sqrt();
                    let v10528 = v89 - v10524;
                    let v10531 = v10484 * v10528;
                    let v10537 = (v10315 + (v10483 * v10528)) + v10312;
                    let v10538 = v151 * v10537;
                    let v10539 = v154 * v10537;
                    let v10542 = (Lanes([0.0, 0.0, v10539[0], 0.0])) + ((v10497 + ((Lanes([0.0, 0.0, v10531[0], 0.0])) + (((v10523 * (v184 / (v236 * v10524))) * v95) * v10483))) * v151);
                    let v10543 = if v10538 < v443 { 1.0 } else { 0.0 };
                    let v10636: f64;
                    let v10637: Lanes<4>;
                    if v10543 != 0.0 {
                        let v10544 = v151 * v9394;
                        let v10548 = v89 / v10544;
                        let v10551 = ((((v154 * v9394) + (v9395 * v151)) * v10548) * v95) / v10544;
                        let v10553 = v10552 + v10548;
                        let v10555 = v10314 * v95;
                        let v10556 = (-v10365) / v9394;
                        let v10557 = v9395 * v10556;
                        let v10570 = ((v10551 * v10562) / v10565) * v95;
                        let v10574 = (v10568 - ((v10562 * v10553) / v10565)) + (v10556 / v10571);
                        let v10576 = (Lanes([0.0, 0.0, v10570[0], 0.0])) + ((((Lanes([v10555[0], v10555[1], 0.0, v10555[2]])) - (Lanes([0.0, 0.0, v10557[0], 0.0]))) / v9394) / v10571);
                        let v10583 = ((v10577 * v10553) - v10580) / v10582;
                        let v10584 = (v10551 * v10577) / v10582;
                        let v10586 = v10576 * v10574;
                        let v10588 = v10583 * v10583;
                        let v10589 = v10584 * v10583;
                        let v10594 = ((v10589 + v10589) * v10583) + (v10584 * v10588);
                        let v10598 = ((v10574 * v10574) + (v10588 * v10583)).sqrt();
                        let v10601 = ((v10586 + v10586) + (Lanes([0.0, 0.0, v10594[0], 0.0]))) * (v184 / (v236 * v10598));
                        let v10604 = (-v10574) + v10598;
                        let v10611 = v10574 + v10598;
                        let v10623 = ((v10604.powf(v9526)) + (-(v10611.powf(v9526)))) - v10622;
                        let v10626 = v161 * v10623;
                        let v10630 = ((v10623 * v158) - v10312) + v10312;
                        let v10631 = v151 * v10630;
                        let v10632 = v154 * v10630;
                        let v10635 = (Lanes([0.0, 0.0, v10632[0], 0.0])) + (((((((v10576 * v95) + v10601) * (v9526 * (v10604.powf(v10607)))) + (((v10576 + v10601) * (v9526 * (v10611.powf(v10614)))) * v95)) * v158) + (Lanes([0.0, 0.0, v10626[0], 0.0]))) * v151);
                        v10636 = v10631;
                        v10637 = v10635;
                    } else {
                        v10636 = v10538;
                        v10637 = v10542;
                    }
                    let v10638 = if v9749 > v60 { 1.0 } else { 0.0 };
                    let v10692: f64;
                    let v10693: Lanes<4>;
                    if v10638 != 0.0 {
                        let v10639 = v10365 + v210;
                        let v10642 = (v151 * v10316).exp();
                        let v10644 = v10642 + v418;
                        let v10645 = v265 / v9296;
                        let v10647 = v10645 * v10645;
                        let v10648 = (v268 / v9296) * v10645;
                        let v10649 = v10648 + v10648;
                        let v10650 = v10647 * v10644;
                        let v10654 = v151 * v10639;
                        let v10655 = v154 * v10639;
                        let v10657 = (Lanes([0.0, 0.0, v10655[0], 0.0])) + v10370;
                        let v10658 = v10650 * v10377;
                        let v10661 = (((v10649 * v10644) + (((v154 * v10316) * v10642) * v10647)) * v10377) + (v10380 * v10650);
                        let v10663 = v10657 * v10654;
                        let v10665 = v10658 + (v10654 * v10654);
                        let v10666 = Lanes([0.0, 0.0, v10661[0], 0.0]);
                        let v10671 = v10647 * v10377;
                        let v10675 = v10671.ln();
                        let v10677 = ((v10649 * v10377) + (v10380 * v10647)) * (v184 / v10671);
                        let v10679 = Lanes([0.0, 0.0, v10677[0], 0.0]);
                        let v10681 = v151 * v10312;
                        let v10682 = v154 * v10312;
                        let v10684 = Lanes([0.0, 0.0, v10682[0], 0.0]);
                        let v10687 = v10657 - ((((v10666 + (v10663 + v10663)) * (v184 / v10665)) - v10679) + v10684);
                        let v10688 = (v10654 - (((v10665.ln()) - v10675) + v10681)) - v89;
                        let v10689 = v446 * v10654;
                        let v10690 = v10657 * v446;
                        let v10691 = if v10689 > v60 { 1.0 } else { 0.0 };
                        let v10711: f64;
                        let v10712: Lanes<4>;
                        if v10691 != 0.0 {
                            v10711 = v10689;
                            v10712 = v10690;
                        } else {
                            let v10709 = -v10689;
                            let v10710 = v10690 * v95;
                            v10711 = v10709;
                            v10712 = v10710;
                        }
                        let v10714 = v10687 * v10688;
                        let v10718 = ((v10688 * v10688) + v10711).sqrt();
                        let v10731 = v154 * v210;
                        let v10732 = (v10654 - (v10654 - (v502 * (v10688 + v10718)))) + (v151 * v210);
                        let v10736 = ((v10657 - (v10657 - ((v10687 + (((v10714 + v10714) + v10712) * (v184 / (v236 * v10718)))) * v502))) + (Lanes([0.0, 0.0, v10731[0], 0.0]))) * v10732;
                        let v10738 = v10658 + (v10732 * v10732);
                        let v10745 = ((v10738.ln()) - v10675) + v10681;
                        let v10746 = (((v10666 + (v10736 + v10736)) * (v184 / v10738)) - v10679) + v10684;
                        let v10748 = v10746 - v10637;
                        let v10750 = (v10745 - v10636) - v10749;
                        let v10754 = (v446 * v10745) * v10753;
                        let v10755 = (v10746 * v446) * v10753;
                        let v10756 = if v10754 > v60 { 1.0 } else { 0.0 };
                        let v10759: f64;
                        let v10760: Lanes<4>;
                        if v10756 != 0.0 {
                            v10759 = v10754;
                            v10760 = v10755;
                        } else {
                            let v10757 = -v10754;
                            let v10758 = v10755 * v95;
                            v10759 = v10757;
                            v10760 = v10758;
                        }
                        let v10762 = v10748 * v10750;
                        let v10766 = ((v10750 * v10750) + v10759).sqrt();
                        let v10774 = v10745 - (v502 * (v10750 + v10766));
                        let v10775 = v10746 - ((v10748 + (((v10762 + v10762) + v10760) * (v184 / (v236 * v10766)))) * v502);
                        v10692 = v10774;
                        v10693 = v10775;
                    } else {
                        v10692 = v10636;
                        v10693 = v10637;
                    }
                    let v10694 = v10692 / v151;
                    let v10695 = v154 * v10694;
                    let v10698 = (v10693 - (Lanes([0.0, 0.0, v10695[0], 0.0]))) / v151;
                    let v10699 = v10694 - v10312;
                    let v10703 = (-v10692).exp();
                    let v10705 = (v10692 - v89) + v10703;
                    let v10706 = v10693 + ((v10693 * v95) * v10703);
                    let v10708 = if v10705 < v10707 { 1.0 } else { 0.0 };
                    let v10777: f64;
                    let v10778: Lanes<4>;
                    if v10708 != 0.0 {
                        v10777 = v10776;
                        v10778 = v9586;
                    } else {
                        v10777 = v10705;
                        v10778 = v10706;
                    }
                    let v10779 = v10777.sqrt();
                    let v10783 = v9323 * v10779;
                    let v10784 = v9324 * v10779;
                    let v10787 = (Lanes([0.0, 0.0, v10784[0], 0.0])) + ((v10778 * (v184 / (v236 * v10779))) * v9323);
                    let v10790 = v589 * (v10315 - v10699);
                    let v10791 = (v10497 - v10698) * v589;
                    let v10792 = if v9749 == v89 { 1.0 } else { 0.0 };
                    let v10806: f64;
                    let v10807: f64;
                    let v10808: Lanes<4>;
                    let v10809: Lanes<4>;
                    if v10792 != 0.0 {
                        let v10795 = (v151 * v10316).exp();
                        let v10796 = (v154 * v10316) * v10795;
                        let v10797 = v265 / v9296;
                        let v10799 = v10797 * v10797;
                        let v10800 = (v268 / v9296) * v10797;
                        let v10801 = v10800 + v10800;
                        let v10802 = v10799 * v10795;
                        let v10805 = (v10801 * v10795) + (v10796 * v10799);
                        let mut v10810: f64 = 0.0;
                        let mut v10811: f64 = 0.0;
                        let mut v10812: f64 = 0.0;
                        let mut v10813: f64 = 0.0;
                        let mut v10814: f64 = 0.0;
                        let mut v10815: f64 = 0.0;
                        let mut v10816: f64 = 0.0;
                        let mut v10817: Lanes<4> = Lanes([0.0; 4]);
                        let mut v10818: Lanes<4> = Lanes([0.0; 4]);
                        let mut v10819: Lanes<4> = Lanes([0.0; 4]);
                        let mut v10820: Lanes<4> = Lanes([0.0; 4]);
                        let mut v10821: Lanes<4> = Lanes([0.0; 4]);
                        v10810 = v89;
                        v10811 = v10699;
                        v10812 = v60;
                        v10813 = v10692;
                        v10814 = v9491;
                        v10815 = v9492;
                        v10816 = v9493;
                        v10817 = v10698;
                        v10818 = v10693;
                        v10819 = v9496;
                        v10820 = v9497;
                        v10821 = v9498;
                        loop {
                            let v10823 = if v10810 <= v10822 { 1.0 } else { 0.0 };
                            if v10823 == 0.0 {
                                break;
                            }
                            let v10824 = v10811 + v10312;
                            let v10825 = v151 * v10824;
                            let v10826 = v154 * v10824;
                            let v10827 = v10817 * v151;
                            let v10829 = (Lanes([0.0, 0.0, v10826[0], 0.0])) + v10827;
                            let v10830 = if v10825 < v4766 { 1.0 } else { 0.0 };
                            let v10967: f64;
                            let v10968: f64;
                            let v10969: f64;
                            let v10970: f64;
                            let v10971: Lanes<4>;
                            let v10972: Lanes<4>;
                            let v10973: Lanes<4>;
                            let v10974: Lanes<4>;
                            if v10830 != 0.0 {
                                let v10832 = v10825 * v10825;
                                let v10833 = v10829 * v10825;
                                let v10834 = v10833 + v10833;
                                let v10835 = v10832 * v10825;
                                let v10842 = v10841 + (v10825 * v9957);
                                let v10847 = v9966 + (v10825 * v10842);
                                let v10848 = v10835 * v10847;
                                let v10851 = (((v10834 * v10825) + (v10829 * v10832)) * v10847) + (((v10829 * v10842) + ((v10829 * v9957) * v10825)) * v10835);
                                let v10852 = v10825 * v4766;
                                let v10853 = v10829 * v4766;
                                let v10857 = v10856 + (v10852 * v9957);
                                let v10863 = v10862 + (v10825 * v10857);
                                let v10864 = v10832 * v10863;
                                let v10868 = v10802 * v10848;
                                let v10869 = v10805 * v10848;
                                let v10873 = v10868 * v10848;
                                let v10876 = (((Lanes([0.0, 0.0, v10869[0], 0.0])) + (v10851 * v10802)) * v10848) + (v10851 * v10868);
                                let v10881 = (v10802 * v151) * v254;
                                let v10883 = v10881 * v10848;
                                let v10884 = (((v10805 * v151) + (v154 * v10802)) * v254) * v10848;
                                let v10895 = v10894 + (v10825 * v10012);
                                let v10900 = v10021 + (v10825 * v10895);
                                let v10906 = v10905 + (v10825 * v10900);
                                let v10911 = v10033 + (v10825 * v10906);
                                let v10912 = v10825 * v10911;
                                let v10915 = (v10829 * v10911) + (((v10829 * v10906) + (((v10829 * v10900) + (((v10829 * v10895) + ((v10829 * v10012) * v10825)) * v10825)) * v10825)) * v10825);
                                let v10919 = v10918 + (v10852 * v10012);
                                let v10925 = v10924 + (v10825 * v10919);
                                let v10931 = v10930 + (v10825 * v10925);
                                let v10936 = v10033 + (v10825 * v10931);
                                let v10938 = v10915 * v10912;
                                let v10943 = (((v10912 * v10912) + v10873) + v418).sqrt();
                                let v10946 = ((v10938 + v10938) + v10876) * (v184 / (v236 * v10943));
                                let v10948 = v154 * v10936;
                                let v10952 = (v151 * v10936) * v254;
                                let v10960 = v10943 + v10943;
                                let v10962 = ((v10952 * v10912) + (v10883 * v10864)) / v10960;
                                let v10965 = (((((((Lanes([0.0, 0.0, v10948[0], 0.0])) + (((v10829 * v10931) + (((v10829 * v10925) + (((v10829 * v10919) + ((v10853 * v10012) * v10825)) * v10825)) * v10825)) * v151)) * v254) * v10912) + (v10915 * v10952)) + ((((Lanes([0.0, 0.0, v10884[0], 0.0])) + (v10851 * v10881)) * v10864) + (((v10834 * v10863) + (((v10829 * v10857) + ((v10853 * v9957) * v10825)) * v10832)) * v10883))) - ((v10946 + v10946) * v10962)) / v10960;
                                v10967 = v10943;
                                v10968 = v10962;
                                v10969 = v10912;
                                v10970 = v10873;
                                v10971 = v10946;
                                v10972 = v10965;
                                v10973 = v10915;
                                v10974 = v10876;
                            } else {
                                let v10966 = if v10825 < v3883 { 1.0 } else { 0.0 };
                                let v11033: f64;
                                let v11034: f64;
                                let v11035: Lanes<4>;
                                let v11036: Lanes<4>;
                                if v10966 != 0.0 {
                                    let v10993 = v10825.exp();
                                    let v10994 = v10829 * v10993;
                                    let v10995 = v10993 - v89;
                                    let v10996 = v10802 * v10995;
                                    let v10997 = v10805 * v10995;
                                    let v11000 = (Lanes([0.0, 0.0, v10997[0], 0.0])) + (v10994 * v10802);
                                    let v11001 = v10802 * v151;
                                    let v11005 = v11001 * v10993;
                                    let v11006 = ((v10805 * v151) + (v154 * v10802)) * v10993;
                                    let v11009 = (Lanes([0.0, 0.0, v11006[0], 0.0])) + (v10994 * v11001);
                                    v11033 = v10996;
                                    v11034 = v11005;
                                    v11035 = v11000;
                                    v11036 = v11009;
                                } else {
                                    let v11011 = v154 * v10811;
                                    let v11014 = (v151 * v10811).exp();
                                    let v11015 = ((Lanes([0.0, 0.0, v11011[0], 0.0])) + v10827) * v11014;
                                    let v11016 = v11014 - v10795;
                                    let v11019 = v10799 * v11016;
                                    let v11020 = v10801 * v11016;
                                    let v11023 = (Lanes([0.0, 0.0, v11020[0], 0.0])) + ((v11015 - (Lanes([0.0, 0.0, v10796[0], 0.0]))) * v10799);
                                    let v11024 = v10799 * v151;
                                    let v11028 = v11024 * v11014;
                                    let v11029 = ((v10801 * v151) + (v154 * v10799)) * v11014;
                                    let v11032 = (Lanes([0.0, 0.0, v11029[0], 0.0])) + (v11015 * v11024);
                                    v11033 = v11019;
                                    v11034 = v11028;
                                    v11035 = v11023;
                                    v11036 = v11032;
                                }
                                let v11040 = ((v10825 - v89) + v11033).sqrt();
                                let v11043 = (v10829 + v11035) * (v184 / (v236 * v11040));
                                let v11047 = (v151 + v11034) / v11040;
                                let v11051 = v11047 * v502;
                                let v11052 = ((((Lanes([0.0, 0.0, v154[0], 0.0])) + v11036) - (v11043 * v11047)) / v11040) * v502;
                                v10967 = v11040;
                                v10968 = v11051;
                                v10969 = v10814;
                                v10970 = v11033;
                                v10971 = v11043;
                                v10972 = v11052;
                                v10973 = v10819;
                                v10974 = v11035;
                            }
                            let v10978 = v9395 * v10967;
                            let v10982 = (v10315 - v10811) - (v9394 * v10967);
                            let v10983 = (v10497 - v10817) - ((Lanes([0.0, 0.0, v10978[0], 0.0])) + (v10971 * v9394));
                            let v10985 = v9395 * v10968;
                            let v10990 = v10989 - (v9394 * v10968);
                            let v10991 = ((Lanes([0.0, 0.0, v10985[0], 0.0])) + (v10972 * v9394)) * v95;
                            let v10992 = if v10812 == v89 { 1.0 } else { 0.0 };
                            let v11066: f64;
                            let v11067: f64;
                            let v11068: f64;
                            let v11069: Lanes<4>;
                            if v10992 != 0.0 {
                                v11066 = v11053;
                                v11067 = v10811;
                                v11068 = v10812;
                                v11069 = v10817;
                            } else {
                                let v11056 = (-v10982) / v10990;
                                let v11059 = ((v10983 * v95) - (v10991 * v11056)) / v10990;
                                let v11060 = v10811.abs();
                                let v11064 = v10817 * ((v236 * (if v10811 >= v4732 { 1.0 } else { 0.0 })) - v184);
                                let v11065 = if v89 >= v11060 { 1.0 } else { 0.0 };
                                let v11071: f64;
                                let v11072: Lanes<4>;
                                if v11065 != 0.0 {
                                    v11071 = v89;
                                    v11072 = v9586;
                                } else {
                                    v11071 = v11060;
                                    v11072 = v11064;
                                }
                                let v11075 = v11074 * (v89 + v11071);
                                let v11076 = v11072 * v11074;
                                let v11078 = if (v11056.abs()) > v11075 { 1.0 } else { 0.0 };
                                let v11080: f64;
                                let v11081: Lanes<4>;
                                if v11078 != 0.0 {
                                    let v11079 = if v11056 >= v60 { 1.0 } else { 0.0 };
                                    let v11090: f64;
                                    if v11079 != 0.0 {
                                        v11090 = v89;
                                    } else {
                                        v11090 = v11089;
                                    }
                                    let v11091 = v11075 * v11090;
                                    let v11092 = v11076 * v11090;
                                    v11080 = v11091;
                                    v11081 = v11092;
                                } else {
                                    v11080 = v11056;
                                    v11081 = v11059;
                                }
                                let v11082 = v10811 + v11080;
                                let v11083 = v10817 + v11081;
                                let v11088 = if (if (v11080.abs()) <= v407 { 1.0 } else { 0.0 }) != 0.0 && (if (v10982.abs()) <= v1270 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let v11093: f64;
                                if v11088 != 0.0 {
                                    v11093 = v89;
                                } else {
                                    v11093 = v10812;
                                }
                                v11066 = v10810;
                                v11067 = v11082;
                                v11068 = v11093;
                                v11069 = v11083;
                            }
                            let v11070 = v11066 + v89;
                            v10810 = v11070;
                            v10811 = v11067;
                            v10812 = v11068;
                            v10813 = v10825;
                            v10814 = v10969;
                            v10815 = v10967;
                            v10816 = v10970;
                            v10817 = v11069;
                            v10818 = v10829;
                            v10819 = v10973;
                            v10820 = v10971;
                            v10821 = v10974;
                        }
                        let v10831 = if v10812 == v60 { 1.0 } else { 0.0 };
                        let v11094 = if v10813 < v4766 { 1.0 } else { 0.0 };
                        let v11101: f64;
                        let v11102: Lanes<4>;
                        if v11094 != 0.0 {
                            let v11095 = if v10813 < v443 { 1.0 } else { 0.0 };
                            let v11126 = v10814 + v11125;
                            v11101 = v11126;
                            v11102 = v10819;
                        } else {
                            let v11097 = (v10813 - v89).sqrt();
                            let v11100 = v10818 * (v184 / (v236 * v11097));
                            v11101 = v11097;
                            v11102 = v11100;
                        }
                        let v11103 = v9323 * v11101;
                        let v11104 = v9324 * v11101;
                        let v11107 = (Lanes([0.0, 0.0, v11104[0], 0.0])) + (v11102 * v9323);
                        let v11108 = v10815 + v11101;
                        let v11110 = v89 / v11108;
                        let v11114 = v9323 * v10816;
                        let v11115 = v9324 * v10816;
                        let v11123 = v11103 + (v11114 * v11110);
                        let v11124 = v11107 + ((((Lanes([0.0, 0.0, v11115[0], 0.0])) + (v10821 * v9323)) * v11110) + (((((v10820 + v11102) * v11110) * v95) / v11108) * v11114));
                        v10806 = v11123;
                        v10807 = v11103;
                        v10808 = v11124;
                        v10809 = v11107;
                    } else {
                        v10806 = v10790;
                        v10807 = v10783;
                        v10808 = v10791;
                        v10809 = v10787;
                    }
                    v10389 = v10806;
                    v10390 = v10807;
                    v10391 = v10808;
                    v10392 = v10809;
                }
                let v11131: f64;
                let v11132: f64;
                let v11133: Lanes<4>;
                let v11134: Lanes<4>;
                if v10268 != 0.0 {
                    let v11127 = v9499 * v10389;
                    let v11128 = v10391 * v9499;
                    let v11129 = v9499 * v10390;
                    let v11130 = v10392 * v9499;
                    v11131 = v11127;
                    v11132 = v11129;
                    v11133 = v11128;
                    v11134 = v11130;
                } else {
                    v11131 = v10254;
                    v11132 = v10255;
                    v11133 = v10256;
                    v11134 = v10257;
                }
                let v11139: f64;
                let v11140: f64;
                let v11141: Lanes<4>;
                let v11142: Lanes<4>;
                if v10270 != 0.0 {
                    let v11135 = v9499 * v10389;
                    let v11136 = v10391 * v9499;
                    let v11137 = v9499 * v10390;
                    let v11138 = v10392 * v9499;
                    v11139 = v11135;
                    v11140 = v11137;
                    v11141 = v11136;
                    v11142 = v11138;
                } else {
                    v11139 = v10262;
                    v11140 = v10263;
                    v11141 = v10264;
                    v11142 = v10265;
                }
                let v11145 = (v116 * v9336) + (v115 * v9332);
                let v11159: f64;
                let v11160: Lanes<3>;
                if v11145 != 0.0 {
                    let v11153 = -(((v116 * v11146) + (v115 * v11148)) * v11151);
                    let v11157 = v11153 * (v114 - v113);
                    let v11158 = (v123 - (Lanes([0.0, v122[0], v122[1]]))) * v11153;
                    v11159 = v11157;
                    v11160 = v11158;
                } else {
                    v11159 = v60;
                    v11160 = v8599;
                }
                let v11163 = (v115 * v9336) + (v116 * v9332);
                let v11171: f64;
                let v11172: Lanes<3>;
                if v11163 != 0.0 {
                    let v11168 = -(((v115 * v11146) + (v116 * v11148)) * v11151);
                    let v11169 = v11168 * v114;
                    let v11170 = v123 * v11168;
                    v11171 = v11169;
                    v11172 = v11170;
                } else {
                    v11171 = v60;
                    v11172 = v8599;
                }
                v9340 = v11159;
                v9341 = v11171;
                v9342 = v11131;
                v9343 = v11139;
                v9344 = v11140;
                v9345 = v11132;
                v9346 = v11160;
                v9347 = v11172;
                v9348 = v11133;
                v9349 = v11141;
                v9350 = v11142;
                v9351 = v11134;
            } else {
                let v9331 = if v119 == v89 { 1.0 } else { 0.0 };
                let v9333 = if v9332 == 0.0 { 1.0 } else { 0.0 };
                let v9335 = if v119 != v89 { 1.0 } else { 0.0 };
                let v9337 = if v9336 == 0.0 { 1.0 } else { 0.0 };
                let v9339 = if (if v9331 != 0.0 && v9333 != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v9335 != 0.0 && v9337 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v11177: f64;
                if v9339 != 0.0 {
                    let v11190: f64;
                    if v9294 != 0.0 {
                        let v11189 = ((-v589) * v9293) * v6957;
                        v11190 = v11189;
                    } else {
                        v11190 = v60;
                    }
                    v11177 = v11190;
                } else {
                    let v11176 = ((v116 * v11146) + (v115 * v11148)) * v11151;
                    v11177 = v11176;
                }
                let v11178 = -v11177;
                let v11182 = v11178 * (v114 - v113);
                let v11183 = (v123 - (Lanes([0.0, v122[0], v122[1]]))) * v11178;
                let v11186 = if (if v9331 != 0.0 && v9337 != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v9335 != 0.0 && v9333 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v11198: f64;
                if v11186 != 0.0 {
                    let v11193 = ((-v589) * v9293) * v6957;
                    v11198 = v11193;
                } else {
                    let v11197 = ((v115 * v11146) + (v116 * v11148)) * v11151;
                    v11198 = v11197;
                }
                let v11199 = -v11198;
                let v11200 = v11199 * v114;
                let v11201 = v123 * v11199;
                v9340 = v11182;
                v9341 = v11200;
                v9342 = v60;
                v9343 = v60;
                v9344 = v60;
                v9345 = v60;
                v9346 = v11183;
                v9347 = v11201;
                v9348 = v9586;
                v9349 = v9586;
                v9350 = v9586;
                v9351 = v9586;
            }
            let v11202: f64;
            let v11203: f64;
            let v11204: Lanes<6>;
            let v11205: Lanes<4>;
            if v65 != 0.0 {
                let v11239: f64;
                let v11240: Lanes<6>;
                if v6798 != 0.0 {
                    let v11212 = v11210 * v11211;
                    let v11213 = v11212 * v6944;
                    let v11225 = v11211 * v6944;
                    let v11233 = (((v7365 * v4821) * v11210) + (v11225 * v6944)) + v418;
                    let v11234 = (v11213 * v6944) / v11233;
                    let v11237 = ((((v6945 * v11212) * v6944) + (v6945 * v11213)) - (((((v7368 * v4821) + (v4825 * v7365)) * v11210) + (((v6945 * v11211) * v6944) + (v6945 * v11225))) * v11234)) / v11233;
                    v11239 = v11234;
                    v11240 = v11237;
                } else {
                    let v11238 = v11210 + v418;
                    v11239 = v11238;
                    v11240 = v3227;
                }
                let v11244 = (v11241 * v627) / v7015;
                let v11245 = (v630 * v11241) / v7015;
                v11202 = v11239;
                v11203 = v11244;
                v11204 = v11240;
                v11205 = v11245;
            } else {
                v11202 = v60;
                v11203 = v60;
                v11204 = v3227;
                v11205 = v603;
            }
            let v11208 = if v6694 == 0.0 { 1.0 } else { 0.0 };
            let v11209 = if (if v11206 != v60 { 1.0 } else { 0.0 }) != 0.0 && v11208 != 0.0 { 1.0 } else { 0.0 };
            if v11209 != 0.0 {
                let v11246 = v4813 / v150;
                let v11256 = if (((((((v11247 * v6949) / v150) / v6944) / v6957) - v11246) - v11246).abs()) > v11255 { 1.0 } else { 0.0 };
            } else {
            }
            let v11257 = if v7950 != 0.0 && v11208 != 0.0 { 1.0 } else { 0.0 };
            let v11277: f64;
            let v11278: f64;
            let v11279: Lanes<6>;
            let v11280: Lanes<6>;
            if v11257 != 0.0 {
                let v11260 = (v9286 - v4802) / v6944;
                let v11269 = (v7172 * v11260) / v11268;
                let v11270 = ((v7173 * v11260) + ((((v9288 - v4806) - (v6945 * v11260)) / v6944) * v7172)) / v11268;
                let v11276 = if (if v11271 <= v11272 { 1.0 } else { 0.0 }) != 0.0 && (if v11272 <= v11274 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v11289: f64;
                let v11290: Lanes<6>;
                if v11276 != 0.0 {
                    v11289 = v89;
                    v11290 = v3227;
                } else {
                    let v11288 = if (if v11284 <= v11272 { 1.0 } else { 0.0 }) != 0.0 && (if v11272 <= v11286 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v11406: f64;
                    let v11407: Lanes<6>;
                    if v11288 != 0.0 {
                        v11406 = v11269;
                        v11407 = v11270;
                    } else {
                        let v11400 = v11272 - v89;
                        let v11401 = v11269.powf(v11400);
                        let v11405 = v11270 * (v11400 * (v11269.powf((v11400 - v184))));
                        v11406 = v11401;
                        v11407 = v11405;
                    }
                    v11289 = v11406;
                    v11290 = v11407;
                }
                let v11294 = (v11270 * v11289) + (v11290 * v11269);
                let v11295 = v89 + (v11269 * v11289);
                let v11298 = (v11296 / v11272) - v89;
                let v11299 = v11295.powf(v11298);
                let v11304 = v7172 * v11295;
                let v11308 = v11304 * v11299;
                let v11311 = (((v7173 * v11295) + (v11294 * v7172)) * v11299) + ((v11294 * (v11298 * (v11295.powf((v11298 - v184))))) * v11304);
                let v11314 = (v7365 + v11308) / v254;
                let v11315 = (v7368 + v11311) / v254;
                let v11316 = v6790 * v6790;
                let v11317 = v6791 * v6790;
                let v11318 = v11317 + v11317;
                let v11319 = v3387 * v627;
                let v11321 = v11319 * v4821;
                let v11322 = (v630 * v3387) * v4821;
                let v11326 = v11321 * v7365;
                let v11330 = v443 * v6790;
                let v11331 = v6791 * v443;
                let v11336 = (v89 + v11330) + (v11333 * v11316);
                let v11338 = v11336 * v11308;
                let v11351 = (v443 + (v446 * v6790)) + (v443 * v11316);
                let v11353 = v11351 * v11308;
                let v11364 = (v11333 + v11330) + v11316;
                let v11366 = v11364 * v7365;
                let v11374 = ((v11338 * v11308) + (v11353 * v7365)) + (v11366 * v7365);
                let v11381 = v11380 * v6944;
                let v11383 = v89 + v6790;
                let v11384 = v11381 * v11383;
                let v11388 = v11384 * v11314;
                let v11392 = v11388 * v11314;
                let v11396 = (v11326 * v11374) / v11392;
                let v11399 = (((((((Lanes([0.0, v11322[0], v11322[1], 0.0, v11322[2], v11322[3]])) + (v4825 * v11319)) * v7365) + (v7368 * v11321)) * v11374) + ((((((((v11331 + (v11318 * v11333)) * v11308) + (v11311 * v11336)) * v11308) + (v11311 * v11338)) + ((((((v6791 * v446) + (v11318 * v443)) * v11308) + (v11311 * v11351)) * v7365) + (v7368 * v11353))) + (((((v11331 + v11318) * v7365) + (v7368 * v11364)) * v7365) + (v7368 * v11366))) * v11326)) - ((((((((v6945 * v11380) * v11383) + (v6791 * v11381)) * v11314) + (v11315 * v11384)) * v11314) + (v11315 * v11388)) * v11396)) / v11392;
                v11277 = v11396;
                v11278 = v11308;
                v11279 = v11399;
                v11280 = v11311;
            } else {
                v11277 = v60;
                v11278 = v60;
                v11279 = v3227;
                v11280 = v3227;
            }
            let v11283 = if (if v7951 != 0.0 && (if v8313 == v89 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v11208 != 0.0 { 1.0 } else { 0.0 };
            let v11525: f64;
            let v11526: f64;
            let v11527: f64;
            let v11528: f64;
            let v11529: Lanes<6>;
            let v11530: Lanes<6>;
            let v11531: Lanes<6>;
            let v11532: Lanes<6>;
            if v11283 != 0.0 {
                let v11408 = v8314.sqrt();
                let v11411 = v8317 * (v184 / (v236 * v11408));
                let v11412 = v4821 + v11408;
                let v11413 = v4825 + v11411;
                let v11415 = v8318 * v8315;
                let v11418 = v8317 * v8314;
                let v11421 = v11420 * v8315;
                let v11433 = v4760 * v11408;
                let v11435 = v11433 * v4821;
                let v11439 = v8315 + v8314;
                let v11445 = ((v11421 * v8314) + (v446 * ((v8315 * v8315) + (v8314 * v8314)))) + (v11435 * v11439);
                let v11446 = ((((v8318 * v11420) * v8314) + (v8317 * v11421)) + (((v11415 + v11415) + (v11418 + v11418)) * v446)) + (((((v11411 * v4760) * v4821) + (v4825 * v11433)) * v11439) + ((v8318 + v8317) * v11435));
                let v11447 = v11412 * v11412;
                let v11448 = v11413 * v11412;
                let v11450 = v11447 * v11447;
                let v11451 = (v11448 + v11448) * v11447;
                let v11453 = v11450 * v11412;
                let v11457 = v11445 / v11453;
                let v11460 = (v11446 - ((((v11451 + v11451) * v11412) + (v11413 * v11450)) * v11457)) / v11453;
                let v11461 = v3387 / v6944;
                let v11465 = v11461 * v7365;
                let v11469 = v11465 * v627;
                let v11471 = v630 * v11465;
                let v11473 = ((((((v6945 * v11461) * v95) / v6944) * v7365) + (v7368 * v11461)) * v627) + (Lanes([0.0, v11471[0], v11471[1], 0.0, v11471[2], v11471[3]]));
                let v11474 = v11469 * v4821;
                let v11478 = v11277 / v11474;
                let v11482 = v446 * v4821;
                let v11490 = (v8315 + (v11482 * v11408)) + v8314;
                let v11493 = v11492 * v8316;
                let v11499 = v11333 * v11412;
                let v11501 = v11478 * v11412;
                let v11505 = v11501 * v4821;
                let v11513 = (v11505 * v11445).sqrt();
                let v11517 = v11499 * v11513;
                let v11521 = (v11493 * v11490) / v11517;
                let v11524 = ((((v8319 * v11492) * v11490) + (((v8318 + (((v4825 * v446) * v11408) + (v11411 * v11482))) + v8317) * v11493)) - ((((v11413 * v11333) * v11513) + ((((((((((v11279 - (((v11473 * v4821) + (v4825 * v11469)) * v11478)) / v11474) * v11412) + (v11413 * v11478)) * v4821) + (v4825 * v11501)) * v11445) + (v11446 * v11505)) * (v184 / (v236 * v11513))) * v11499)) * v11521)) / v11517;
                v11525 = v11469;
                v11526 = v11408;
                v11527 = v11457;
                v11528 = v11521;
                v11529 = v11473;
                v11530 = v11411;
                v11531 = v11460;
                v11532 = v11524;
            } else {
                v11525 = v407;
                v11526 = v60;
                v11527 = v60;
                v11528 = v60;
                v11529 = v3227;
                v11530 = v3227;
                v11531 = v3227;
                v11532 = v3227;
            }
            let v11533 = v7945 + v8488;
            let v11534 = v7946 + v8489;
            let v11547: f64;
            let v11548: Lanes<4>;
            if v11535 != 0.0 {
                let v11539 = (-v11536) * v11538;
                let v11544 = v11539 * (v117 - v120);
                let v11545 = ((Lanes([v124[0], v124[1], v124[2], 0.0])) - (Lanes([v126[0], v126[1], 0.0, v126[2]]))) * v11539;
                v11547 = v11544;
                v11548 = v11545;
            } else {
                v11547 = v60;
                v11548 = v11546;
            }
            let v11555 = (v11549 * v6957) * ((v89 + (v11551 / v676)).ln());
            let v11560 = (v124 - (Lanes([v125[0], v125[1], 0.0]))) * v11555;
            let v11562 = v124 * v11555;
            let v11563 = v9340 + (v11555 * (v117 - v118));
            let v11566 = (Lanes([0.0, 0.0, v9346[0], v9346[1], v9346[2]])) + (Lanes([v11560[0], v11560[1], v11560[2], 0.0, 0.0]));
            let v11567 = v9341 + (v11555 * v117);
            let v11570 = (Lanes([0.0, 0.0, v9347[0], v9347[1], v9347[2]])) + (Lanes([v11562[0], v11562[1], v11562[2], 0.0, 0.0]));
            let v11580 = v6948 * (v210 * v1359);
            let v11581 = v11580 * v120;
            let v11582 = v126 * v11580;
            let v11586 = v11580 * (v120 - v118);
            let v11587 = (v126 - (Lanes([v125[0], v125[1], 0.0]))) * v11580;
            let v11588 = v6948 * (v11571 * v6742);
            let v11589 = (v6743 * v11571) * v6948;
            let v11590 = v6948 * (v11576 * (v3794 + v5680));
            let v11591 = ((v3803 + v5689) * v11576) * v6948;
            let v11597: f64;
            let v11598: f64;
            let v11599: Lanes<6>;
            let v11600: Lanes<6>;
            if v11592 != 0.0 {
                v11597 = v6949;
                v11598 = v60;
                v11599 = v6950;
                v11600 = v3227;
            } else {
                let v11595 = (v6949 + v11588) + v11590;
                let v11596 = (v6950 + v11589) + v11591;
                v11597 = v11595;
                v11598 = v6958;
                v11599 = v11596;
                v11600 = v6959;
            }
            let v11601 = v11597 * v502;
            let v11602 = v11599 * v502;
            let v11618: f64;
            let v11619: f64;
            let v11620: f64;
            let v11621: f64;
            let v11622: f64;
            let v11623: Lanes<6>;
            let v11624: Lanes<8>;
            let v11625: Lanes<8>;
            let v11626: Lanes<6>;
            let v11627: Lanes<6>;
            if v65 != 0.0 {
                v11618 = v60;
                v11619 = v60;
                v11620 = v60;
                v11621 = v11597;
                v11622 = v11598;
                v11623 = v3227;
                v11624 = v11603;
                v11625 = v11603;
                v11626 = v11599;
                v11627 = v11600;
            } else {
                let v11606 = (-v11598) - v11597;
                let v11607 = (v11600 * v95) - v11599;
                let v11608 = v11601 + v11586;
                let v11611 = (Lanes([0.0, 0.0, v11602[0], v11602[1], v11602[2], v11602[3], v11602[4], v11602[5]])) + (Lanes([v11587[0], v11587[1], 0.0, 0.0, v11587[2], 0.0, 0.0, 0.0]));
                let v11613 = v11599 - v11602;
                let v11614 = (v11597 - v11601) + v11581;
                let v11617 = (Lanes([0.0, 0.0, v11613[0], v11613[1], v11613[2], v11613[3], v11613[4], v11613[5]])) + (Lanes([v11582[0], v11582[1], 0.0, 0.0, v11582[2], 0.0, 0.0, 0.0]));
                v11618 = v11606;
                v11619 = v11608;
                v11620 = v11614;
                v11621 = v60;
                v11622 = v60;
                v11623 = v11607;
                v11624 = v11611;
                v11625 = v11617;
                v11626 = v3227;
                v11627 = v3227;
            }
            let v11628 = if v9284 == v60 { 1.0 } else { 0.0 };
            let v11634: f64;
            let v11635: Lanes<6>;
            if v11628 != 0.0 {
                v11634 = v60;
                v11635 = v3227;
            } else {
                let v11631 = (v9287 * v3295) + v4802;
                let v11632 = (v9289 * v3295) + v4806;
                let v11633 = if v11631 > v9286 { 1.0 } else { 0.0 };
                let v11638: f64;
                let v11639: Lanes<6>;
                if v11633 != 0.0 {
                    v11638 = v9286;
                    v11639 = v9288;
                } else {
                    v11638 = v11631;
                    v11639 = v11632;
                }
                let v11640 = v113 + v4802;
                let v11641 = v6961 + v4806;
                let v11644 = v89 - v6832;
                let v11655 = (v472 * v6957) * (((v11649 / v470).sqrt()) * v11652);
                let v11662 = (((v11640 - ((v6832 * v11640) + (v11644 * v11638))) / v9284) - v9287) * v11655;
                let v11663 = (((v11641 - ((v11641 * v6832) + (v11639 * v11644))) / v9284) - v9289) * v11655;
                v11634 = v11662;
                v11635 = v11663;
            }
            let v11637 = if v11636 != v60 { 1.0 } else { 0.0 };
            let v11670: f64;
            let v11671: Lanes<6>;
            if v11637 != 0.0 {
                let v11666 = v355 * v11664;
                let v11667 = v11634 + (v11664 * v353);
                let v11669 = v11635 + (Lanes([0.0, 0.0, v11666[0], 0.0, v11666[1], v11666[2]]));
                v11670 = v11667;
                v11671 = v11669;
            } else {
                v11670 = v11634;
                v11671 = v11635;
            }
            let v11673 = if v11672 == v89 { 1.0 } else { 0.0 };
            let v11713: f64;
            let v11714: f64;
            let v11715: f64;
            let v11716: Lanes<8>;
            let v11717: Lanes<8>;
            let v11718: Lanes<8>;
            if v11673 != 0.0 {
                let v11675 = v11566 + v11570;
                let v11679 = (Lanes([v11675[0], v11675[1], v11675[2], 0.0, v11675[3], v11675[4]])) - (Lanes([v11548[0], v11548[1], v11548[2], v11548[3], 0.0, 0.0]));
                let v11682 = Lanes([0.0, 0.0, v11671[0], v11671[1], v11671[2], v11671[3], v11671[4], v11671[5]]);
                let v11690 = v11618 + (((((v11563 + v11567) - v11547) - v11670) - v9342) - v9343);
                let v11692 = (Lanes([0.0, 0.0, v11623[0], v11623[1], v11623[2], v11623[3], v11623[4], v11623[5]])) + ((((Lanes([v11679[0], v11679[1], 0.0, v11679[2], v11679[3], 0.0, v11679[4], v11679[5]])) - v11682) - (Lanes([v9348[0], v9348[1], v9348[2], v9348[3], 0.0, 0.0, 0.0, 0.0]))) - (Lanes([v9349[0], v9349[1], v9349[2], v9349[3], 0.0, 0.0, 0.0, 0.0])));
                let v11694 = v11566 * v95;
                let v11701 = v11619 + (((-v11563) + v11670) + v9344);
                let v11702 = v11624 + (((Lanes([v11694[0], v11694[1], 0.0, v11694[2], 0.0, 0.0, v11694[3], v11694[4]])) + v11682) + (Lanes([v9350[0], v9350[1], v9350[2], v9350[3], 0.0, 0.0, 0.0, 0.0])));
                let v11704 = v11570 * v95;
                let v11708 = (Lanes([v11704[0], v11704[1], 0.0, v11704[2], v11704[3], v11704[4]])) + (Lanes([v9351[0], v9351[1], v9351[2], v9351[3], 0.0, 0.0]));
                let v11709 = v11620 + ((-v11567) + v9345);
                let v11711 = v11625 + (Lanes([v11708[0], v11708[1], v11708[2], v11708[3], 0.0, 0.0, v11708[4], v11708[5]]));
                v11713 = v11690;
                v11714 = v11701;
                v11715 = v11709;
                v11716 = v11692;
                v11717 = v11702;
                v11718 = v11711;
            } else {
                let v11712 = Lanes([0.0, 0.0, v11623[0], v11623[1], v11623[2], v11623[3], v11623[4], v11623[5]]);
                v11713 = v11618;
                v11714 = v11619;
                v11715 = v11620;
                v11716 = v11712;
                v11717 = v11624;
                v11718 = v11625;
            }
            let v11719 = -v8600;
            let v11720 = v8605 * v95;
            let v11721 = if v119 == v89 { 1.0 } else { 0.0 };
            let v11733: f64;
            let v11734: Lanes<6>;
            if v11721 != 0.0 {
                let v11724 = (v8601 * v8602) - v8603;
                let v11726 = (v8606 * v8601) - (Lanes([0.0, v8607[0], 0.0, 0.0, v8607[1], v8607[2]]));
                v11733 = v11724;
                v11734 = v11726;
            } else {
                let v11727 = v89 - v8601;
                let v11730 = (v11727 * v8602) - v8604;
                let v11732 = (v8606 * v11727) - (Lanes([0.0, v8608[0], 0.0, 0.0, v8608[1], v8608[2]]));
                v11733 = v11730;
                v11734 = v11732;
            }
            let v11746: f64;
            let v11747: Lanes<6>;
            if v11721 != 0.0 {
                let v11735 = v89 - v8601;
                let v11738 = (v11735 * v8602) - v8604;
                let v11740 = (v8606 * v11735) - (Lanes([0.0, v8608[0], 0.0, 0.0, v8608[1], v8608[2]]));
                v11746 = v11738;
                v11747 = v11740;
            } else {
                let v11743 = (v8601 * v8602) - v8603;
                let v11745 = (v8606 * v8601) - (Lanes([0.0, v8607[0], 0.0, 0.0, v8607[1], v8607[2]]));
                v11746 = v11743;
                v11747 = v11745;
            }
            let v11748: f64;
            let v11749: Lanes<5>;
            if v11721 != 0.0 {
                v11748 = v9097;
                v11749 = v9098;
            } else {
                v11748 = v9203;
                v11749 = v9204;
            }
            let v11750: f64;
            let v11751: Lanes<5>;
            if v11721 != 0.0 {
                v11750 = v9203;
                v11751 = v9204;
            } else {
                v11750 = v9097;
                v11751 = v9098;
            }
            let v11753 = v11752 * v128;
            let v11754 = v64 * v11752;
            let v11756 = v17 * (v11716[6]);
            let v11758 = v17 * (v11716[7]);
            let v11759 = if v119 > v60 { 1.0 } else { 0.0 };
            let v11760: f64;
            if v11759 != 0.0 {
                v11760 = v11758;
            } else {
                v11760 = v11756;
            }
            let v11782: f64;
            let v11783: f64;
            let v11784: Lanes<6>;
            let v11785: Lanes<6>;
            if v11283 != 0.0 {
                let v11764 = ((v11761 * v627) * v6957) * v3295;
                let v11771 = ((v161 * v11765) * v11760) * v11760;
                let v11772 = (((v11765 * v158) * v11760) * v11760) / v11525;
                let v11776 = ((Lanes([v11771[0], 0.0, 0.0, 0.0, 0.0, 0.0])) - (v11529 * v11772)) / v11525;
                let v11781 = if (if v8316 > v11777 { 1.0 } else { 0.0 }) != 0.0 && (if v113 > v11779 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v11836: f64;
                let v11837: Lanes<6>;
                if v11781 != 0.0 {
                    let v11794 = v7172 / v7365;
                    let v11797 = (v7173 - (v7368 * v11794)) / v7365;
                    let v11798 = v7172 / v11278;
                    let v11804 = (v11798 - v11794) / v113;
                    let v11805 = v122 * v11804;
                    let v11810 = v11809 * v11804;
                    let v11818 = (v8315 + (v4821 * v11526)) + v8314;
                    let v11824 = v4821 + v11526;
                    let v11826 = (v11810 * v11818) / v11824;
                    let v11830 = v11794 + v11826;
                    let v11831 = v11797 + ((((((((((v7173 - (v11280 * v11798)) / v11278) - v11797) - (Lanes([0.0, 0.0, 0.0, 0.0, v11805[0], v11805[1]]))) / v113) * v11809) * v11818) + (((v8318 + ((v4825 * v11526) + (v11530 * v4821))) + v8317) * v11810)) - ((v4825 + v11530) * v11826)) / v11824);
                    v11836 = v11830;
                    v11837 = v11831;
                } else {
                    let v11832 = v7172 / v11278;
                    let v11835 = (v7173 - (v11280 * v11832)) / v11278;
                    v11836 = v11832;
                    v11837 = v11835;
                }
                let v11838 = v11772 * v11527;
                let v11842 = v11838 * v11836;
                let v11845 = (((v11776 * v11527) + (v11531 * v11772)) * v11836) + (v11837 * v11838);
                let v11846 = if v11842 < v60 { 1.0 } else { 0.0 };
                let v11847: f64;
                let v11848: Lanes<6>;
                if v11846 != 0.0 {
                    v11847 = v60;
                    v11848 = v3227;
                } else {
                    v11847 = v11842;
                    v11848 = v11845;
                }
                let v11850 = if (-v11760) > v11764 { 1.0 } else { 0.0 };
                let v11851: f64;
                let v11852: Lanes<6>;
                if v11850 != 0.0 {
                    v11851 = v11847;
                    v11852 = v11848;
                } else {
                    v11851 = v60;
                    v11852 = v3227;
                }
                let v11853: f64;
                let v11854: Lanes<6>;
                if v11850 != 0.0 {
                    v11853 = v11528;
                    v11854 = v11532;
                } else {
                    v11853 = v60;
                    v11854 = v3227;
                }
                v11782 = v11853;
                v11783 = v11851;
                v11784 = v11854;
                v11785 = v11852;
            } else {
                v11782 = v60;
                v11783 = v60;
                v11784 = v3227;
                v11785 = v3227;
            }
            let v11786 = v11753 * v11277;
            let v11787 = v11754 * v11277;
            let v11790 = (Lanes([v11787[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v11279 * v11753);
            let v11793 = if (if v11786 > v60 { 1.0 } else { 0.0 }) != 0.0 && (if v11783 > v60 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v11863: f64;
            let v11864: Lanes<6>;
            if v11793 != 0.0 {
                let v11855 = v11783 / v11786;
                let v11859 = v11855.sqrt();
                let v11862 = ((v11785 - (v11790 * v11855)) / v11786) * (v184 / (v236 * v11859));
                v11863 = v11859;
                v11864 = v11862;
            } else {
                v11863 = v60;
                v11864 = v3227;
            }
            let v11870: f64;
            let v11871: Lanes<6>;
            if v11759 != 0.0 {
                let v11866 = v11863 * v11865;
                let v11867 = v11864 * v11865;
                v11870 = v11866;
                v11871 = v11867;
            } else {
                let v11868 = v11863 * v502;
                let v11869 = v11864 * v502;
                v11870 = v11868;
                v11871 = v11869;
            }
            let v11877: f64;
            let v11878: Lanes<6>;
            if v11759 != 0.0 {
                let v11872 = v11863 * v502;
                let v11873 = v11864 * v502;
                v11877 = v11872;
                v11878 = v11873;
            } else {
                let v11875 = v11863 * v11874;
                let v11876 = v11864 * v11874;
                v11877 = v11875;
                v11878 = v11876;
            }
            let v11881: f64;
            let v11882: Lanes<3>;
            if v11879 != 0.0 {
                let v11888 = v17 * (v10 - v36);
                let v11891 = v162.powf(v11890);
                let v11897 = v11896 / v11891;
                let v11908 = ((v11901 + v203) + v213) - (v11905 * v221);
                let v11911 = v11910 / v11908;
                let v11917 = v64 * v11915;
                let v11919 = v11918 + (v11915 * v135);
                let v11921 = v11897 * v11920;
                let v11922 = ((((v163 * (v11890 * (v162.powf(v11892)))) * v11897) * v95) / v11891) * v11920;
                let v11928 = ((((((v204 + v216) - (v222 * v11905)) * v11911) * v95) / v11908) * v11923) * v11926;
                let v11929 = ((v11911 * v11923) * v11926) + v418;
                let v11931 = v11888 / v11930;
                let v11933 = v11921 * v11931;
                let v11934 = v11922 * v11931;
                let v11935 = ((((Lanes([0.0, v14[0]])) - (Lanes([v39[0], 0.0]))) * v17) / v11930) * v11921;
                let v11938 = (Lanes([0.0, v11934[0], 0.0])) + (Lanes([v11935[0], 0.0, v11935[1]]));
                let v11939 = if v11888 >= v60 { 1.0 } else { 0.0 };
                let v11952: f64;
                let v11953: Lanes<3>;
                if v11939 != 0.0 {
                    let v11940 = v11933 / v11929;
                    let v11941 = v11928 * v11940;
                    let v11944 = (v11938 - (Lanes([0.0, v11941[0], 0.0]))) / v11929;
                    v11952 = v11940;
                    v11953 = v11944;
                } else {
                    let v11947 = (-v11933) / v11929;
                    let v11948 = v11928 * v11947;
                    let v11951 = ((v11938 * v95) - (Lanes([0.0, v11948[0], 0.0]))) / v11929;
                    v11952 = v11947;
                    v11953 = v11951;
                }
                let v11958 = if (if v11954 <= v11919 { 1.0 } else { 0.0 }) != 0.0 && (if v11919 <= v11956 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v11964: f64;
                let v11965: Lanes<3>;
                if v11958 != 0.0 {
                    v11964 = v89;
                    v11965 = v11880;
                } else {
                    let v11963 = if (if v11959 <= v11919 { 1.0 } else { 0.0 }) != 0.0 && (if v11919 <= v11961 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v11987: f64;
                    let v11988: Lanes<3>;
                    if v11963 != 0.0 {
                        v11987 = v11952;
                        v11988 = v11953;
                    } else {
                        let v11976 = v11919 - v89;
                        let v11977 = v11952.powf(v11976);
                        let v11984 = v11917 * (v11977 * (v11952.ln()));
                        let v11986 = (v11953 * (v11976 * (v11952.powf((v11976 - v184))))) + (Lanes([0.0, v11984[0], 0.0]));
                        v11987 = v11977;
                        v11988 = v11986;
                    }
                    v11964 = v11987;
                    v11965 = v11988;
                }
                let v11969 = (v11953 * v11964) + (v11965 * v11952);
                let v11970 = v89 + (v11952 * v11964);
                let v11975 = if (if v11971 <= v11919 { 1.0 } else { 0.0 }) != 0.0 && (if v11919 <= v11973 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v11998: f64;
                let v11999: Lanes<3>;
                if v11975 != 0.0 {
                    let v11989 = v89 / v11970;
                    let v11992 = ((v11969 * v11989) * v95) / v11970;
                    v11998 = v11989;
                    v11999 = v11992;
                } else {
                    let v11997 = if (if v11993 <= v11919 { 1.0 } else { 0.0 }) != 0.0 && (if v11919 <= v11995 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v12040: f64;
                    let v12041: Lanes<3>;
                    if v11997 != 0.0 {
                        let v12012 = v11970.sqrt();
                        let v12016 = v89 / v12012;
                        let v12019 = (((v11969 * (v184 / (v236 * v12012))) * v12016) * v95) / v12012;
                        v12040 = v12016;
                        v12041 = v12019;
                    } else {
                        let v12021 = v12020 / v11919;
                        let v12025 = v12021 - v89;
                        let v12026 = v11970.powf(v12025);
                        let v12033 = (((v11917 * v12021) * v95) / v11919) * (v12026 * (v11970.ln()));
                        let v12036 = v11970 * v12026;
                        let v12039 = (v11969 * v12026) + (((v11969 * (v12025 * (v11970.powf((v12025 - v184))))) + (Lanes([0.0, v12033[0], 0.0]))) * v11970);
                        v12040 = v12036;
                        v12041 = v12039;
                    }
                    v11998 = v12040;
                    v11999 = v12041;
                }
                let v12001 = v11922 * v11998;
                let v12009 = (v12005 * (v11921 * v11998)) * v12008;
                let v12010 = (((Lanes([0.0, v12001[0], 0.0])) + (v11999 * v11921)) * v12005) * v12008;
                let v12011 = if v12009 <= v60 { 1.0 } else { 0.0 };
                let v12042: f64;
                let v12043: Lanes<3>;
                if v12011 != 0.0 {
                    v12042 = v418;
                    v12043 = v11880;
                } else {
                    v12042 = v12009;
                    v12043 = v12010;
                }
                let v12044 = v89 / v12042;
                let v12049 = (((v12043 * v12044) * v95) / v12042) / v3387;
                let v12051 = (v12044 / v3387) + v12050;
                let v12053 = if (if v12051 > v650 { 1.0 } else { 0.0 }) != 0.0 && v7950 != 0.0 { 1.0 } else { 0.0 };
                let v12054 = if v12051 < v650 { 1.0 } else { 0.0 };
                let v12055: f64;
                let v12056: Lanes<3>;
                if v12054 != 0.0 {
                    v12055 = v650;
                    v12056 = v11880;
                } else {
                    v12055 = v12051;
                    v12056 = v12049;
                }
                v11881 = v12055;
                v11882 = v12056;
            } else {
                v11881 = v60;
                v11882 = v11880;
            }
            let v12058: f64;
            let v12059: Lanes<3>;
            if v11883 != 0.0 {
                let v12064 = v17 * (v44 - v20);
                let v12066 = v162.powf(v11890);
                let v12072 = v12071 / v12066;
                let v12081 = ((v11901 + v203) + v213) - (v11905 * v221);
                let v12084 = v12083 / v12081;
                let v12089 = v64 * v11915;
                let v12091 = v12090 + (v11915 * v135);
                let v12093 = v12072 * v12092;
                let v12094 = ((((v163 * (v11890 * (v162.powf(v12067)))) * v12072) * v95) / v12066) * v12092;
                let v12100 = ((((((v204 + v216) - (v222 * v11905)) * v12084) * v95) / v12081) * v12095) * v12098;
                let v12101 = ((v12084 * v12095) * v12098) + v418;
                let v12103 = v12064 / v12102;
                let v12105 = v12093 * v12103;
                let v12106 = v12094 * v12103;
                let v12107 = ((((Lanes([v46[0], 0.0])) - (Lanes([0.0, v22[0]]))) * v17) / v12102) * v12093;
                let v12110 = (Lanes([0.0, v12106[0], 0.0])) + (Lanes([v12107[0], 0.0, v12107[1]]));
                let v12111 = if v12064 >= v60 { 1.0 } else { 0.0 };
                let v12124: f64;
                let v12125: Lanes<3>;
                if v12111 != 0.0 {
                    let v12112 = v12105 / v12101;
                    let v12113 = v12100 * v12112;
                    let v12116 = (v12110 - (Lanes([0.0, v12113[0], 0.0]))) / v12101;
                    v12124 = v12112;
                    v12125 = v12116;
                } else {
                    let v12119 = (-v12105) / v12101;
                    let v12120 = v12100 * v12119;
                    let v12123 = ((v12110 * v95) - (Lanes([0.0, v12120[0], 0.0]))) / v12101;
                    v12124 = v12119;
                    v12125 = v12123;
                }
                let v12130 = if (if v12126 <= v12091 { 1.0 } else { 0.0 }) != 0.0 && (if v12091 <= v12128 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v12136: f64;
                let v12137: Lanes<3>;
                if v12130 != 0.0 {
                    v12136 = v89;
                    v12137 = v12057;
                } else {
                    let v12135 = if (if v12131 <= v12091 { 1.0 } else { 0.0 }) != 0.0 && (if v12091 <= v12133 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v12159: f64;
                    let v12160: Lanes<3>;
                    if v12135 != 0.0 {
                        v12159 = v12124;
                        v12160 = v12125;
                    } else {
                        let v12148 = v12091 - v89;
                        let v12149 = v12124.powf(v12148);
                        let v12156 = v12089 * (v12149 * (v12124.ln()));
                        let v12158 = (v12125 * (v12148 * (v12124.powf((v12148 - v184))))) + (Lanes([0.0, v12156[0], 0.0]));
                        v12159 = v12149;
                        v12160 = v12158;
                    }
                    v12136 = v12159;
                    v12137 = v12160;
                }
                let v12141 = (v12125 * v12136) + (v12137 * v12124);
                let v12142 = v89 + (v12124 * v12136);
                let v12147 = if (if v12143 <= v12091 { 1.0 } else { 0.0 }) != 0.0 && (if v12091 <= v12145 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v12170: f64;
                let v12171: Lanes<3>;
                if v12147 != 0.0 {
                    let v12161 = v89 / v12142;
                    let v12164 = ((v12141 * v12161) * v95) / v12142;
                    v12170 = v12161;
                    v12171 = v12164;
                } else {
                    let v12169 = if (if v12165 <= v12091 { 1.0 } else { 0.0 }) != 0.0 && (if v12091 <= v12167 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v12211: f64;
                    let v12212: Lanes<3>;
                    if v12169 != 0.0 {
                        let v12183 = v12142.sqrt();
                        let v12187 = v89 / v12183;
                        let v12190 = (((v12141 * (v184 / (v236 * v12183))) * v12187) * v95) / v12183;
                        v12211 = v12187;
                        v12212 = v12190;
                    } else {
                        let v12192 = v12191 / v12091;
                        let v12196 = v12192 - v89;
                        let v12197 = v12142.powf(v12196);
                        let v12204 = (((v12089 * v12192) * v95) / v12091) * (v12197 * (v12142.ln()));
                        let v12207 = v12142 * v12197;
                        let v12210 = (v12141 * v12197) + (((v12141 * (v12196 * (v12142.powf((v12196 - v184))))) + (Lanes([0.0, v12204[0], 0.0]))) * v12142);
                        v12211 = v12207;
                        v12212 = v12210;
                    }
                    v12170 = v12211;
                    v12171 = v12212;
                }
                let v12173 = v12094 * v12170;
                let v12180 = (v12177 * (v12093 * v12170)) * v9296;
                let v12181 = (((Lanes([0.0, v12173[0], 0.0])) + (v12171 * v12093)) * v12177) * v9296;
                let v12182 = if v12180 <= v60 { 1.0 } else { 0.0 };
                let v12213: f64;
                let v12214: Lanes<3>;
                if v12182 != 0.0 {
                    v12213 = v418;
                    v12214 = v12057;
                } else {
                    v12213 = v12180;
                    v12214 = v12181;
                }
                let v12215 = v89 / v12213;
                let v12220 = (((v12214 * v12215) * v95) / v12213) / v3387;
                let v12222 = (v12215 / v3387) + v12221;
                let v12224 = if (if v12222 > v650 { 1.0 } else { 0.0 }) != 0.0 && v7950 != 0.0 { 1.0 } else { 0.0 };
                let v12225 = if v12222 < v650 { 1.0 } else { 0.0 };
                let v12226: f64;
                let v12227: Lanes<3>;
                if v12225 != 0.0 {
                    v12226 = v650;
                    v12227 = v12057;
                } else {
                    v12226 = v12222;
                    v12227 = v12220;
                }
                v12058 = v12226;
                v12059 = v12227;
            } else {
                v12058 = v60;
                v12059 = v12057;
            }
            let v12232: f64;
            let v12233: f64;
            let v12234: f64;
            let v12235: f64;
            let v12236: f64;
            let v12237: f64;
            let v12238: Lanes<1>;
            let v12239: Lanes<1>;
            let v12240: Lanes<2>;
            let v12241: Lanes<1>;
            let v12242: Lanes<7>;
            let v12243: Lanes<7>;
            if v65 != 0.0 {
                let v12228 = if v11202 < v6751 { 1.0 } else { 0.0 };
                let v12244: f64;
                let v12245: Lanes<6>;
                if v12228 != 0.0 {
                    v12244 = v6751;
                    v12245 = v3227;
                } else {
                    v12244 = v11202;
                    v12245 = v11204;
                }
                let v12246 = if v11203 < v6751 { 1.0 } else { 0.0 };
                let v12247: f64;
                let v12248: Lanes<4>;
                if v12246 != 0.0 {
                    v12247 = v6751;
                    v12248 = v603;
                } else {
                    v12247 = v11203;
                    v12248 = v11205;
                }
                let v12253 = (v80 - v11621) / v12244;
                let v12254 = v12245 * v12253;
                let v12257 = (((Lanes([0.0, 0.0, 0.0, v82[0], 0.0, 0.0, 0.0])) - (Lanes([v11626[0], v11626[1], v11626[2], 0.0, v11626[3], v11626[4], v11626[5]]))) - (Lanes([v12254[0], v12254[1], v12254[2], 0.0, v12254[3], v12254[4], v12254[5]]))) / v12244;
                let v12262 = (v81 - v11622) / v12247;
                let v12263 = v12248 * v12262;
                let v12266 = (((Lanes([0.0, 0.0, 0.0, v83[0], 0.0, 0.0, 0.0])) - (Lanes([v11627[0], v11627[1], v11627[2], 0.0, v11627[3], v11627[4], v11627[5]]))) - (Lanes([0.0, v12263[0], v12263[1], 0.0, 0.0, v12263[2], v12263[3]]))) / v12247;
                let v12268 = v82 * v95;
                let v12269 = (-v80) - v81;
                let v12272 = (Lanes([v12268[0], 0.0])) - (Lanes([0.0, v83[0]]));
                let v12273 = v80 * v502;
                let v12274 = v82 * v502;
                let v12276 = v80 * v12275;
                let v12277 = v82 * v12275;
                v12232 = v12273;
                v12233 = v12276;
                v12234 = v12269;
                v12235 = v81;
                v12236 = v12253;
                v12237 = v12262;
                v12238 = v12274;
                v12239 = v12277;
                v12240 = v12272;
                v12241 = v83;
                v12242 = v12257;
                v12243 = v12266;
            } else {
                v12232 = v60;
                v12233 = v60;
                v12234 = v60;
                v12235 = v60;
                v12236 = v60;
                v12237 = v60;
                v12238 = v78;
                v12239 = v78;
                v12240 = v12229;
                v12241 = v79;
                v12242 = v12230;
                v12243 = v12231;
            }
            let v12292: f64;
            let v12293: f64;
            let v12294: f64;
            let v12295: f64;
            let v12296: f64;
            let v12297: f64;
            let v12298: Lanes<6>;
            let v12299: Lanes<6>;
            let v12300: Lanes<6>;
            let v12301: Lanes<8>;
            let v12302: Lanes<1>;
            let v12303: Lanes<8>;
            if v11721 != 0.0 {
                let v12282 = -((v11713 + v11714) + v11715);
                let v12283 = ((v11716 + v11717) + v11718) * v95;
                v12292 = v11533;
                v12293 = v8382;
                v12294 = v60;
                v12295 = v11714;
                v12296 = v12232;
                v12297 = v12282;
                v12298 = v11534;
                v12299 = v8383;
                v12300 = v3227;
                v12301 = v11717;
                v12302 = v12238;
                v12303 = v12283;
            } else {
                let v12284 = -v11533;
                let v12285 = v11534 * v95;
                let v12290 = -((v11713 + v11714) + v11715);
                let v12291 = ((v11716 + v11717) + v11718) * v95;
                let v12304: f64;
                let v12305: Lanes<1>;
                if v65 != 0.0 {
                    v12304 = v12233;
                    v12305 = v12239;
                } else {
                    v12304 = v12232;
                    v12305 = v12238;
                }
                v12292 = v12284;
                v12293 = v60;
                v12294 = v8382;
                v12295 = v11715;
                v12296 = v12304;
                v12297 = v12290;
                v12298 = v12285;
                v12299 = v3227;
                v12300 = v8383;
                v12301 = v11718;
                v12302 = v12305;
                v12303 = v12291;
            }
            let v12311: f64;
            let v12312: Lanes<6>;
            if v58 != 0.0 {
                let v12306 = v11533 * v113;
                let v12308 = v122 * v11533;
                let v12310 = (v11534 * v113) + (Lanes([0.0, 0.0, 0.0, 0.0, v12308[0], v12308[1]]));
                v12311 = v12306;
                v12312 = v12310;
            } else {
                v12311 = v60;
                v12312 = v3227;
            }
            let v12313 = if v119 != v89 { 1.0 } else { 0.0 };
            let v12314 = v17 * v12292;
            let v12315 = v12298 * v17;
            let v12319 = v17 * (v11748 + v12293);
            let v12320 = ((Lanes([v11749[0], v11749[1], v11749[2], 0.0, v11749[3], v11749[4]])) + v12299) * v17;
            let v12324 = v17 * (v11750 + v12294);
            let v12325 = ((Lanes([v11751[0], v11751[1], v11751[2], 0.0, v11751[3], v11751[4]])) + v12300) * v17;
            let v12326 = v17 * v11746;
            let v12327 = v11747 * v17;
            let v12328 = v17 * v11733;
            let v12329 = v11734 * v17;
            let v12330 = v17 * v11719;
            let v12331 = v11720 * v17;
            let v12342: f64;
            let v12343: Lanes<3>;
            if v12332 != 0.0 {
                let v12336 = (Lanes([0.0, v14[0]])) - (Lanes([v39[0], 0.0]));
                let v12337 = (v10 - v36) / v11881;
                let v12341 = ((Lanes([v12336[0], 0.0, v12336[1]])) - (v11882 * v12337)) / v11881;
                v12342 = v12337;
                v12343 = v12341;
            } else {
                v12342 = v60;
                v12343 = v11880;
            }
            let v12354: f64;
            let v12355: Lanes<3>;
            if v12344 != 0.0 {
                let v12348 = (Lanes([v46[0], 0.0])) - (Lanes([0.0, v22[0]]));
                let v12349 = (v44 - v20) / v12058;
                let v12353 = ((Lanes([v12348[0], 0.0, v12348[1]])) - (v12059 * v12349)) / v12058;
                v12354 = v12349;
                v12355 = v12353;
            } else {
                v12354 = v60;
                v12355 = v12057;
            }
            let v12363 = v17 * (ddt(45332, (v11713 + v12234)));
            let v12364 = (((Lanes([v11716[0], v11716[1], v11716[2], v11716[3], v11716[4], 0.0, 0.0, v11716[5], v11716[6], v11716[7]])) + (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v12240[0], v12240[1], 0.0, 0.0, 0.0]))) * v12361) * v17;
            let v12371 = v17 * (ddt(45338, (v12295 + v12296)));
            let v12372 = (((Lanes([v12301[0], v12301[1], v12301[2], v12301[3], v12301[4], 0.0, v12301[5], v12301[6], v12301[7]])) + (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v12302[0], 0.0, 0.0, 0.0]))) * v12361) * v17;
            let v12379 = v17 * (ddt(45344, (v12297 + v12235)));
            let v12380 = (((Lanes([v12303[0], v12303[1], v12303[2], v12303[3], v12303[4], 0.0, v12303[5], v12303[6], v12303[7]])) + (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v12241[0], 0.0, 0.0, 0.0]))) * v12361) * v17;
            let v12382 = v11782 * v12381;
            let v12383 = v11784 * v12381;
            let v12385 = v12384 * v11782;
            let v12388 = (Lanes([v12383[0], v12383[1], v12383[2], 0.0, v12383[3], v12383[4], v12383[5]])) + (Lanes([0.0, 0.0, 0.0, v12385[0], 0.0, 0.0, 0.0]));
            let v12389 = v12381 * v11870;
            let v12390 = v12384 * v11870;
            let v12391 = v11871 * v12381;
            let v12394 = (Lanes([0.0, 0.0, 0.0, v12390[0], 0.0, 0.0, 0.0])) + (Lanes([v12391[0], v12391[1], v12391[2], 0.0, v12391[3], v12391[4], v12391[5]]));
            let v12395 = ddt(45375, v12389);
            let v12396 = v12394 * v12361;
            let v12397 = v12381 * v11877;
            let v12398 = v12384 * v11877;
            let v12399 = v11878 * v12381;
            let v12402 = (Lanes([0.0, 0.0, 0.0, v12398[0], 0.0, 0.0, 0.0])) + (Lanes([v12399[0], v12399[1], v12399[2], 0.0, v12399[3], v12399[4], v12399[5]]));
            let v12403 = ddt(45379, v12397);
            let v12404 = v12402 * v12361;
            let v12416: f64;
            let v12417: Lanes<2>;
            if v12405 != 0.0 {
                let v12413 = v12412 * (v12406 - v9);
                let v12414 = ((Lanes([v12408[0], 0.0])) - (Lanes([0.0, v12[0]]))) * v12412;
                v12416 = v12413;
                v12417 = v12414;
            } else {
                v12416 = v60;
                v12417 = v12415;
            }
            let v12434: f64;
            let v12435: Lanes<6>;
            if v58 != 0.0 {
                let v12424 = (v66 * v12420) * v12361;
                let v12430 = v66 * v12428;
                let v12431 = ((-v12311) + (ddt(45429, (v12420 * v59)))) + (v59 * v12428);
                let v12433 = ((v12312 * v95) + (Lanes([v12424[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) + (Lanes([v12430[0], 0.0, 0.0, 0.0, 0.0, 0.0]));
                v12434 = v12431;
                v12435 = v12433;
            } else {
                v12434 = v60;
                v12435 = v3227;
            }
            let v12444: f64;
            let v12445: Lanes<6>;
            if v12436 != 0.0 {
                let v12440 = (v3753 * v69) * v12361;
                let v12441 = v3235 + (ddt(45445, (v69 * v3751)));
                let v12443 = v3243 + (Lanes([0.0, 0.0, 0.0, v12440[0], 0.0, 0.0]));
                v12444 = v12441;
                v12445 = v12443;
            } else {
                v12444 = v60;
                v12445 = v3227;
            }
            let v12460: f64;
            let v12461: f64;
            let v12462: Lanes<7>;
            let v12463: Lanes<7>;
            if v65 != 0.0 {
                let v12449 = (v72 * v69) * v12361;
                let v12450 = v12236 + (ddt(45453, (v69 * v70)));
                let v12452 = v12242 + (Lanes([0.0, 0.0, 0.0, v12449[0], 0.0, 0.0, 0.0]));
                let v12456 = (v76 * v69) * v12361;
                let v12457 = v12237 + (ddt(45459, (v69 * v74)));
                let v12459 = v12243 + (Lanes([0.0, 0.0, 0.0, v12456[0], 0.0, 0.0, 0.0]));
                v12460 = v12450;
                v12461 = v12457;
                v12462 = v12452;
                v12463 = v12459;
            } else {
                v12460 = v60;
                v12461 = v60;
                v12462 = v12230;
                v12463 = v12231;
            }
            let v12464 = v12315[0];
            let v12465 = v12315[1];
            let v12466 = v12315[2];
            let v12467 = v12315[3];
            let v12468 = v12315[4];
            let v12469 = v12315[5];
            let v12470 = v12320[0];
            let v12471 = v12320[1];
            let v12472 = v12320[2];
            let v12473 = v12320[3];
            let v12474 = v12320[4];
            let v12475 = v12320[5];
            let v12476 = v12325[0];
            let v12477 = v12325[1];
            let v12478 = v12325[2];
            let v12479 = v12325[3];
            let v12480 = v12325[4];
            let v12481 = v12325[5];
            let v12482 = v12327[0];
            let v12483 = v12327[1];
            let v12484 = v12327[2];
            let v12485 = v12327[3];
            let v12486 = v12327[4];
            let v12487 = v12327[5];
            let v12488 = v12329[0];
            let v12489 = v12329[1];
            let v12490 = v12329[2];
            let v12491 = v12329[3];
            let v12492 = v12329[4];
            let v12493 = v12329[5];
            let v12494 = v12331[0];
            let v12495 = v12331[1];
            let v12496 = v12331[2];
            let v12497 = v12331[3];
            let v12498 = v12343[0];
            let v12499 = v12343[1];
            let v12500 = v12343[2];
            let v12501 = v12355[0];
            let v12502 = v12355[1];
            let v12503 = v12355[2];
            let v12504 = v12364[0];
            let v12505 = v12364[1];
            let v12506 = v12364[2];
            let v12507 = v12364[3];
            let v12508 = v12364[4];
            let v12509 = v12364[5];
            let v12510 = v12364[6];
            let v12511 = v12364[7];
            let v12512 = v12364[8];
            let v12513 = v12364[9];
            let v12514 = v12372[0];
            let v12515 = v12372[1];
            let v12516 = v12372[2];
            let v12517 = v12372[3];
            let v12518 = v12372[4];
            let v12519 = v12372[5];
            let v12520 = v12372[6];
            let v12521 = v12372[7];
            let v12522 = v12372[8];
            let v12523 = v12380[0];
            let v12524 = v12380[1];
            let v12525 = v12380[2];
            let v12526 = v12380[3];
            let v12527 = v12380[4];
            let v12528 = v12380[5];
            let v12529 = v12380[6];
            let v12530 = v12380[7];
            let v12531 = v12380[8];
            let v12532 = v12384[0];
            let v12533 = v12388[0];
            let v12534 = v12388[1];
            let v12535 = v12388[2];
            let v12536 = v12388[3];
            let v12537 = v12388[4];
            let v12538 = v12388[5];
            let v12539 = v12388[6];
            let v12540 = v12396[0];
            let v12541 = v12396[1];
            let v12542 = v12396[2];
            let v12543 = v12396[3];
            let v12544 = v12396[4];
            let v12545 = v12396[5];
            let v12546 = v12396[6];
            let v12547 = v12404[0];
            let v12548 = v12404[1];
            let v12549 = v12404[2];
            let v12550 = v12404[3];
            let v12551 = v12404[4];
            let v12552 = v12404[5];
            let v12553 = v12404[6];
            let v12554 = v12417[0];
            let v12555 = v12417[1];
            let v12556 = v12435[0];
            let v12557 = v12435[1];
            let v12558 = v12435[2];
            let v12559 = v12435[3];
            let v12560 = v12435[4];
            let v12561 = v12435[5];
            let v12562 = v12445[0];
            let v12563 = v12445[1];
            let v12564 = v12445[2];
            let v12565 = v12445[3];
            let v12566 = v12445[4];
            let v12567 = v12445[5];
            let v12568 = v12462[0];
            let v12569 = v12462[1];
            let v12570 = v12462[2];
            let v12571 = v12462[3];
            let v12572 = v12462[4];
            let v12573 = v12462[5];
            let v12574 = v12462[6];
            let v12575 = v12463[0];
            let v12576 = v12463[1];
            let v12577 = v12463[2];
            let v12578 = v12463[3];
            let v12579 = v12463[4];
            let v12580 = v12463[5];
            let v12581 = v12463[6];
            let v12582 = v12394[0];
            let v12583 = v12394[1];
            let v12584 = v12394[2];
            let v12585 = v12394[3];
            let v12586 = v12394[4];
            let v12587 = v12394[5];
            let v12588 = v12394[6];
            let v12589 = v12402[0];
            let v12590 = v12402[1];
            let v12591 = v12402[2];
            let v12592 = v12402[3];
            let v12593 = v12402[4];
            let v12594 = v12402[5];
            let v12595 = v12402[6];
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(12),
            multiplicity * (v12314),
            [4, 5, 6, 10, 11, 12],
            [v12464, v12465, v12466, v12467, v12468, v12469],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(12),
            multiplicity * (v12319),
            [4, 5, 6, 10, 11, 12],
            [v12470, v12471, v12472, v12473, v12474, v12475],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(12),
            Some(11),
            multiplicity * (v12324),
            [4, 5, 6, 10, 11, 12],
            [v12476, v12477, v12478, v12479, v12480, v12481],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(12),
            multiplicity * (v12326),
            [4, 5, 6, 10, 11, 12],
            [v12482, v12483, v12484, v12485, v12486, v12487],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(11),
            multiplicity * (v12328),
            [4, 5, 6, 10, 11, 12],
            [v12488, v12489, v12490, v12491, v12492, v12493],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (v12330),
            [5, 6, 11, 12],
            [v12494, v12495, v12496, v12497],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(12),
            Some(2),
            multiplicity * (v12342),
            [2, 4, 12],
            [v12498, v12499, v12500],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(12), Some(2), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            staged[194],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(0),
            Some(11),
            multiplicity * (v12354),
            [0, 4, 11],
            [v12501, v12502, v12503],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(0), Some(11), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            staged[195],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(12),
            multiplicity * (v12363),
            [0, 2, 4, 5, 6, 8, 9, 10, 11, 12],
            [v12504, v12505, v12506, v12507, v12508, v12509, v12510, v12511, v12512, v12513],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(11),
            Some(12),
            multiplicity * (v12371),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [v12514, v12515, v12516, v12517, v12518, v12519, v12520, v12521, v12522],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(12),
            multiplicity * (v12379),
            [0, 2, 4, 5, 6, 9, 10, 11, 12],
            [v12523, v12524, v12525, v12526, v12527, v12528, v12529, v12530, v12531],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(11),
            Some(12),
            multiplicity * (v12596),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(7),
            None,
            multiplicity * (v12381),
            [7],
            [v12532],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            None,
            multiplicity * (v12597),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(11),
            Some(12),
            multiplicity * (v12598),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(11),
            Some(12),
            multiplicity * (v12382),
            [4, 5, 6, 7, 10, 11, 12],
            [v12533, v12534, v12535, v12536, v12537, v12538, v12539],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(5),
            Some(12),
            multiplicity * (v12395),
            [4, 5, 6, 7, 10, 11, 12],
            [v12540, v12541, v12542, v12543, v12544, v12545, v12546],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(5),
            Some(11),
            multiplicity * (v12403),
            [4, 5, 6, 7, 10, 11, 12],
            [v12547, v12548, v12549, v12550, v12551, v12552, v12553],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(12),
            Some(2),
            multiplicity * (staged[196]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(11),
            multiplicity * (staged[197]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(11),
            multiplicity * (v12599),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(12),
            multiplicity * (v12600),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(6),
            multiplicity * (v12601),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(5),
            multiplicity * (v12416),
            [1, 5],
            [v12554, v12555],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(1), Some(5), 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            staged[198],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(3), Some(6), 3, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            3,
            v12602,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(4),
            None,
            multiplicity * (v12434),
            [4, 5, 6, 10, 11, 12],
            [v12556, v12557, v12558, v12559, v12560, v12561],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(4), None, 4, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            4,
            staged[199],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            None,
            multiplicity * (v12444),
            [4, 5, 6, 10, 11, 12],
            [v12562, v12563, v12564, v12565, v12566, v12567],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(10), None, 5, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            5,
            staged[200],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(8),
            None,
            multiplicity * (v12460),
            [4, 5, 6, 8, 10, 11, 12],
            [v12568, v12569, v12570, v12571, v12572, v12573, v12574],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(9),
            None,
            multiplicity * (v12461),
            [4, 5, 6, 9, 10, 11, 12],
            [v12575, v12576, v12577, v12578, v12579, v12580, v12581],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(8), None, 6, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            6,
            staged[201],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(9), None, 7, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            7,
            staged[202],
            [],
            [],
            [],
            [],
        );
        self.canonical_reactive[0] = v12314;
        self.canonical_reactive[1] = v12319;
        self.canonical_reactive[2] = v12324;
        self.canonical_reactive[3] = v12326;
        self.canonical_reactive[4] = v12328;
        self.canonical_reactive[5] = v12330;
        self.canonical_reactive[6] = v12342;
        self.canonical_reactive[7] = staged[194];
        self.canonical_reactive[8] = v12354;
        self.canonical_reactive[9] = staged[195];
        self.canonical_reactive[10] = v12363;
        self.canonical_reactive[11] = v12371;
        self.canonical_reactive[12] = v12379;
        self.canonical_reactive[13] = v12596;
        self.canonical_reactive[14] = v12381;
        self.canonical_reactive[15] = v12597;
        self.canonical_reactive[16] = v12598;
        self.canonical_reactive[17] = v12382;
        self.canonical_reactive[18] = v12389;
        self.canonical_reactive[19] = v12582;
        self.canonical_reactive[20] = v12583;
        self.canonical_reactive[21] = v12584;
        self.canonical_reactive[22] = v12585;
        self.canonical_reactive[23] = v12586;
        self.canonical_reactive[24] = v12587;
        self.canonical_reactive[25] = v12588;
        self.canonical_reactive[26] = v12397;
        self.canonical_reactive[27] = v12589;
        self.canonical_reactive[28] = v12590;
        self.canonical_reactive[29] = v12591;
        self.canonical_reactive[30] = v12592;
        self.canonical_reactive[31] = v12593;
        self.canonical_reactive[32] = v12594;
        self.canonical_reactive[33] = v12595;
        self.canonical_reactive[34] = staged[196];
        self.canonical_reactive[35] = staged[197];
        self.canonical_reactive[36] = v12599;
        self.canonical_reactive[37] = v12600;
        self.canonical_reactive[38] = v12601;
        self.canonical_reactive[39] = v12416;
        self.canonical_reactive[40] = staged[198];
        self.canonical_reactive[41] = v12602;
        self.canonical_reactive[42] = v12434;
        self.canonical_reactive[43] = staged[199];
        self.canonical_reactive[44] = v12444;
        self.canonical_reactive[45] = staged[200];
        self.canonical_reactive[46] = v12460;
        self.canonical_reactive[47] = v12461;
        self.canonical_reactive[48] = staged[201];
        self.canonical_reactive[49] = staged[202];
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(12),
            &[4, 5, 6, 7, 10, 11, 12],
            &[cached[19], cached[20], cached[21], cached[22], cached[23], cached[24], cached[25]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(11),
            &[4, 5, 6, 7, 10, 11, 12],
            &[cached[27], cached[28], cached[29], cached[30], cached[31], cached[32], cached[33]],
            &[],
            &[],
            multiplicity,
        );
    }

}

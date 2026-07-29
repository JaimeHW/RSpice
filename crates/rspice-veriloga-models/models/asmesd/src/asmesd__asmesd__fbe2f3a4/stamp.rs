#![allow(dead_code, non_snake_case, unused_imports, unused_mut, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

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
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let parameters = &self.params.values;
        let multiplicity = self.multiplicity;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 3355 => 0usize, 3525 => 1usize, 3871 => 2usize, 3902 => 3usize, 3909 => 4usize, 4101 => 5usize, 4107 => 6usize, 4113 => 7usize, 4119 => 8usize, 4125 => 9usize, 4131 => 10usize, 4136 => 11usize, 4140 => 12usize, _ => usize::MAX };
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
            let v1 = temperature;
            let v2 = node_potentials[3];
            let v4 = parameters[45];
            let v6 = 2.7315e2f64;
            let v7 = 1.3e3f64;
            let v8 = 1.7314999999999998e2f64;
            let v10 = 1.7314999999999998e2f64;
            let v13 = 1.3e3f64;
            let v14 = 1.7314999999999998e2f64;
            let v16 = 1.7314999999999998e2f64;
            let v19 = 1e0f64;
            let v20 = 0.0f64;
            let v21 = parameters[26];
            let v23 = parameters[43];
            let v24 = parameters[42];
            let v26 = parameters[29];
            let v27 = node_potentials[5];
            let v28 = node_potentials[4];
            let v31 = parameters[79];
            let v34 = parameters[80];
            let v38 = parameters[25];
            let v40 = 8.6170869e-5f64;
            let v44 = parameters[77];
            let v47 = parameters[52];
            let v50 = parameters[60];
            let v52 = parameters[53];
            let v56 = parameters[62];
            let v60 = parameters[54];
            let v64 = parameters[63];
            let v68 = parameters[22];
            let v70 = parameters[21];
            let v75 = parameters[23];
            let v77 = parameters[0];
            let v80 = parameters[2];
            let v83 = parameters[58];
            let v84 = parameters[59];
            let v89 = parameters[64];
            let v90 = parameters[65];
            let v95 = parameters[47];
            let v96 = parameters[7];
            let v100 = parameters[5];
            let v101 = parameters[6];
            let v105 = parameters[9];
            let v106 = parameters[10];
            let v110 = parameters[56];
            let v111 = parameters[55];
            let v115 = parameters[16];
            let v116 = parameters[69];
            let v117 = parameters[74];
            let v118 = 3.0015e2f64;
            let v121 = 1.16e0f64;
            let v122 = 7.02e-4f64;
            let v125 = 1.108e3f64;
            let v130 = 1.3806226e-23f64;
            let v134 = 1.3454442398941469e20f64;
            let v138 = 1.5e0f64;
            let v141 = 1.6021918e-19f64;
            let v145 = parameters[17];
            let v150 = parameters[18];
            let v151 = 4e-4f64;
            let v168 = 1.3454442398941469e20f64;
            let v173 = parameters[70];
            let v178 = parameters[71];
            let v191 = 1.3454442398941469e20f64;
            let v196 = parameters[75];
            let v201 = parameters[76];
            let v214 = node_potentials[2];
            let v217 = node_potentials[6];
            let v220 = node_potentials[1];
            let v228 = parameters[1];
            let v233 = parameters[11];
            let v238 = 8e1f64;
            let v246 = 3.7e1f64;
            let v248 = -3.7e1f64;
            let v255 = -3.7e1f64;
            let v269 = parameters[8];
            let v277 = parameters[4];
            let v279 = 1e-3f64;
            let v281 = -1e0f64;
            let v284 = parameters[3];
            let v302 = parameters[57];
            let v315 = -3.7e1f64;
            let v322 = -3.7e1f64;
            let v342 = parameters[61];
            let v359 = -3.7e1f64;
            let v366 = -3.7e1f64;
            let v403 = -3.7e1f64;
            let v410 = -3.7e1f64;
            let v430 = node_potentials[9];
            let v433 = 1e-6f64;
            let v435 = parameters[83];
            let v440 = 1e-9f64;
            let v454 = parameters[81];
            let v465 = 4e0f64;
            let v469 = parameters[82];
            let v472 = 2e0f64;
            let v478 = parameters[84];
            let v484 = parameters[48];
            let v487 = parameters[49];
            let v490 = parameters[50];
            let v493 = parameters[51];
            let v496 = parameters[12];
            let v497 = parameters[37];
            let v504 = parameters[66];
            let v505 = parameters[78];
            let v509 = parameters[14];
            let v510 = parameters[38];
            let v518 = parameters[40];
            let v521 = parameters[39];
            let v527 = parameters[19];
            let v528 = parameters[41];
            let v533 = parameters[73];
            let v535 = parameters[32];
            let v540 = node_potentials[8];
            let v544 = parameters[20];
            let v546 = parameters[44];
            let v550 = 0e0f64;
            let v551 = parameters[31];
            let v554 = parameters[13];
            let v556 = parameters[67];
            let v558 = parameters[15];
            let v572 = 5e-1f64;
            let v579 = parameters[24];
            let v583 = -1e0f64;
            let v618 = -1e0f64;
            let v649 = parameters[72];
            let v654 = -1e0f64;
            let v686 = parameters[68];
            let v691 = 3.141592653589793e0f64;
            let v693 = 1.8e2f64;
            let v697 = parameters[30];
            let v699 = parameters[33];
            let v702 = -1e0f64;
            let v706 = node_potentials[0];
            let v712 = parameters[34];
            let v715 = 0e0f64;
            let v718 = parameters[35];
            let v721 = -1e0f64;
            let v729 = node_potentials[7];
            let v735 = parameters[36];
            let v738 = -1e0f64;
            let v740 = -1e0f64;
            let v748 = 0e0f64;
            let v749 = 0e0f64;
            let v750 = 0e0f64;
            let v768 = parameters[46];
            let v777 = 0e0f64;
            let v778 = 0e0f64;
            let v788 = 0e0f64;
            let v789 = 0e0f64;
            let v800 = 0e0f64;
            let v801 = 0e0f64;
            let v836 = parameters[28];
            let v838 = parameters[27];
            let v842 = 0e0f64;
            let v844 = 0e0f64;
            let v845 = 0e0f64;
            let v893 = 1e0f64;
            let v894 = 1e0f64;
            let v895 = 1e0f64;
            let v896 = 1e0f64;
            let v897 = 1e0f64;
            let v898 = 1e0f64;
            let v899 = 1e0f64;
            let v900 = 1e0f64;
            let v901 = 1e0f64;
            let v902 = 1e0f64;
            let v903 = 1e0f64;
            let v980 = 0e0f64;
            let v987 = -1e0f64;
            let v1170 = Lanes([0e0f64; 3]);
            let v1212 = 0e0f64;
            let v1214 = 2e0f64;
            let v1313 = Lanes([0e0f64; 3]);
            let v1431 = ddt_scale();
            let v1587 = Lanes([0e0f64; 6]);
            let v1588 = 0e0f64;
            let v1589 = Lanes([0e0f64; 3]);
            let v1716 = Lanes([0e0f64; 3]);
            let v1771 = Lanes([0e0f64; 4]);
            let v1773 = Lanes([0e0f64; 7]);
            let v1799 = Lanes([0e0f64; 2]);
            let v1800 = 0e0f64;
            let v1869 = Lanes([0e0f64; 4]);
            let v1875 = Lanes([0e0f64; 3]);
            let v1881 = Lanes([0e0f64; 3]);
            let v5 = (v1 + v2) + v4;
            let v9 = if v5 > v8 { 1.0 } else { 0.0 };
            let v11: f64;
            if v9 != 0.0 {
                v11 = v5;
            } else {
                v11 = v10;
            }
            let v12 = if v7 < v11 { 1.0 } else { 0.0 };
            let v18: f64;
            let v904: f64;
            if v12 != 0.0 {
                v18 = v13;
                v904 = v980;
            } else {
                let v15 = if v5 > v14 { 1.0 } else { 0.0 };
                let v17: f64;
                let v905: f64;
                if v15 != 0.0 {
                    v17 = v5;
                    v905 = v894;
                } else {
                    v17 = v16;
                    v905 = v980;
                }
                v18 = v17;
                v904 = v905;
            }
            if v20 != 0.0 {
            } else {
            }
            let v22 = if v18 > v21 { 1.0 } else { 0.0 };
            if v22 != 0.0 {
            } else {
            }
            let v25 = v23 * v24;
            let v29 = v27 - v28;
            let v983 = (Lanes([0.0, v895])) - (Lanes([v896, 0.0]));
            let v30 = v26 * v29;
            let v984 = v983 * v26;
            let v33 = -(if v30 <= v0 { v30 } else { v0 });
            let v37 = v19 + (v31 * (v33.powf(v34)));
            let v39 = v38 + v6;
            let v41 = v40 * v18;
            let v994 = v904 * v40;
            let v42 = v18 / v39;
            let v995 = v904 / v39;
            let v43 = v42.ln();
            let v997 = v995 * (v893 / v42);
            let v46 = (v44 * v43).exp();
            let v999 = (v997 * v44) * v46;
            let v48 = v47 * v46;
            let v49 = v48 * v37;
            let v1002 = ((((v984 * (if v30 <= v0 { 1.0 } else { 0.0 })) * v987) * (v34 * (v33.powf((v34 - v893))))) * v31) * v48;
            let v1005 = (Lanes([((v999 * v47) * v37), 0.0, 0.0])) + (Lanes([0.0, v1002[0], v1002[1]]));
            let v51 = v50 * v46;
            let v1006 = v999 * v50;
            let v53 = if v52 > v0 { 1.0 } else { 0.0 };
            let v55: f64;
            if v53 != 0.0 {
                let v54 = v19 / v52;
                v55 = v54;
            } else {
                v55 = v0;
            }
            let v57 = if v56 > v0 { 1.0 } else { 0.0 };
            let v59: f64;
            if v57 != 0.0 {
                let v58 = v19 / v56;
                v59 = v58;
            } else {
                v59 = v0;
            }
            let v61 = if v60 > v0 { 1.0 } else { 0.0 };
            let v63: f64;
            if v61 != 0.0 {
                let v62 = v19 / v60;
                v63 = v62;
            } else {
                v63 = v0;
            }
            let v65 = if v64 > v0 { 1.0 } else { 0.0 };
            let v67: f64;
            if v65 != 0.0 {
                let v66 = v19 / v64;
                v67 = v66;
            } else {
                v67 = v0;
            }
            let v71 = v42 - v19;
            let v73 = (v70 * v71) / v41;
            let v74 = (v68 * v43) + v73;
            let v1012 = (v997 * v68) + (((v995 * v70) - (v994 * v73)) / v41);
            let v78 = v74.exp();
            let v79 = v77 * v78;
            let v1015 = (v1012 * v78) * v77;
            let v81 = (v75 * v43).exp();
            let v82 = v80 * v81;
            let v1017 = ((v997 * v75) * v81) * v80;
            let v86 = (v74 / v84).exp();
            let v88 = (v83 * v86) / v46;
            let v1023 = ((((v1012 / v84) * v86) * v83) - (v999 * v88)) / v46;
            let v92 = (v74 / v90).exp();
            let v94 = (v89 * v92) / v46;
            let v1029 = ((((v1012 / v90) * v92) * v89) - (v999 * v94)) / v46;
            let v99 = v95 * (v19 + (v96 * v71));
            let v1031 = (v995 * v96) * v95;
            let v104 = v100 * (v19 + (v101 * v71));
            let v1033 = (v995 * v101) * v100;
            let v109 = v105 * (v19 + (v106 * v71));
            let v1035 = (v995 * v106) * v105;
            let v114 = v110 * (v19 + (v111 * v71));
            let v1037 = (v995 * v111) * v110;
            let v119 = v39 / v118;
            let v120 = v18 / v118;
            let v1038 = v904 / v118;
            let v123 = v122 * v18;
            let v126 = v125 + v18;
            let v127 = (v123 * v18) / v126;
            let v1047 = ((((((v904 * v122) * v18) + (v904 * v123)) - (v904 * v127)) / v126) * v987) * v987;
            let v132 = v130 * (v18 + v18);
            let v133 = (-(v121 - v127)) / v132;
            let v1050 = ((v904 + v904) * v130) * v133;
            let v137 = -(v41 + v41);
            let v1054 = (v994 + v994) * v987;
            let v140 = v138 * (v120.ln());
            let v1057 = (v1038 * (v893 / v120)) * v138;
            let v143 = v140 + (v141 * (v133 + v134));
            let v144 = v137 * v143;
            let v1062 = (v1054 * v143) + ((v1057 + (((v1047 - v1050) / v132) * v141)) * v137);
            let v147 = (v145 - v144) / v119;
            let v1064 = (v1062 * v987) / v119;
            let v149 = (v145 - v147) / v147;
            let v153 = v151 * (v39 - v118);
            let v156 = v19 + (v150 * (v153 - v149));
            let v157 = v115 / v156;
            let v159 = (v120 * v147) + v144;
            let v1077 = ((v1038 * v147) + (v1064 * v120)) + v1062;
            let v161 = (v159 - v147) / v147;
            let v163 = v151 * (v18 - v118);
            let v1082 = v904 * v151;
            let v166 = v19 + (v150 * (v163 - v161));
            let v167 = v157 * v166;
            let v1087 = (((((((((v1064 * v987) - (v1064 * v149)) / v147) * v987) * v150) * v157) * v987) / v156) * v166) + (((v1082 - (((v1077 - v1064) - (v1064 * v161)) / v147)) * v150) * v157);
            let v171 = v140 + (v141 * (v133 + v168));
            let v172 = v137 * v171;
            let v1094 = (v1054 * v171) + ((v1057 + (((v1047 - v1050) / v132) * v141)) * v137);
            let v175 = (v173 - v172) / v119;
            let v1096 = (v1094 * v987) / v119;
            let v177 = (v173 - v175) / v175;
            let v181 = v19 + (v178 * (v153 - v177));
            let v182 = v116 / v181;
            let v184 = (v120 * v175) + v172;
            let v1109 = ((v1038 * v175) + (v1096 * v120)) + v1094;
            let v186 = (v184 - v175) / v175;
            let v189 = v19 + (v178 * (v163 - v186));
            let v190 = v182 * v189;
            let v1118 = (((((((((v1096 * v987) - (v1096 * v177)) / v175) * v987) * v178) * v182) * v987) / v181) * v189) + (((v1082 - (((v1109 - v1096) - (v1096 * v186)) / v175)) * v178) * v182);
            let v194 = v140 + (v141 * (v133 + v191));
            let v195 = v137 * v194;
            let v1125 = (v1054 * v194) + ((v1057 + (((v1047 - v1050) / v132) * v141)) * v137);
            let v198 = (v196 - v195) / v119;
            let v1127 = (v1125 * v987) / v119;
            let v200 = (v196 - v198) / v198;
            let v204 = v19 + (v201 * (v153 - v200));
            let v205 = v117 / v204;
            let v207 = (v120 * v198) + v195;
            let v1140 = ((v1038 * v198) + (v1127 * v120)) + v1125;
            let v209 = (v207 - v198) / v198;
            let v212 = v19 + (v201 * (v163 - v209));
            let v213 = v205 * v212;
            let v1149 = (((((((((v1127 * v987) - (v1127 * v200)) / v198) * v987) * v201) * v205) * v987) / v204) * v212) + (((v1082 - (((v1140 - v1127) - (v1127 * v209)) / v198)) * v201) * v205);
            let v216 = v26 * (v214 - v28);
            let v1153 = ((Lanes([v897, 0.0])) - (Lanes([0.0, v896]))) * v26;
            let v218 = v27 - v217;
            let v1156 = (Lanes([v895, 0.0])) - (Lanes([0.0, v898]));
            let v219 = v26 * v218;
            let v1157 = v1156 * v26;
            let v222 = v26 * (v220 - v28);
            let v1161 = ((Lanes([v899, 0.0])) - (Lanes([0.0, v896]))) * v26;
            let v223 = v220 - v27;
            let v1164 = (Lanes([v899, 0.0])) - (Lanes([0.0, v895]));
            let v224 = v26 * v223;
            let v1165 = v1164 * v26;
            let v225 = v214 - v217;
            let v1168 = (Lanes([v897, 0.0])) - (Lanes([0.0, v898]));
            let v226 = v26 * v225;
            let v1169 = v1168 * v26;
            let v227 = if v79 > v0 { 1.0 } else { 0.0 };
            let v444: f64;
            let v906: Lanes<3>;
            if v227 != 0.0 {
                let v229 = v228 * v41;
                let v230 = v219 / v229;
                let v1176 = ((Lanes([0.0, v1157[0], v1157[1]])) - (Lanes([((v994 * v228) * v230), 0.0, 0.0]))) / v229;
                let v1177 = v1157 * v987;
                let v234 = v233 * v41;
                let v1181 = v994 * v233;
                let v235 = ((-v219) - v104) / v234;
                let v1185 = (((Lanes([0.0, v1177[0], v1177[1]])) - (Lanes([v1033, 0.0, 0.0]))) - (Lanes([(v1181 * v235), 0.0, 0.0]))) / v234;
                let v237 = (-v104) / v234;
                let v1189 = ((v1033 * v987) - (v1181 * v237)) / v234;
                let v239 = if v230 > v238 { 1.0 } else { 0.0 };
                let v242: f64;
                let v243: f64;
                let v907: Lanes<3>;
                let v908: Lanes<3>;
                if v239 != 0.0 {
                    let v241 = v19 + (v230 - v238);
                    v242 = v241;
                    v243 = v238;
                    v907 = v1176;
                    v908 = v1170;
                } else {
                    v242 = v19;
                    v243 = v230;
                    v907 = v1170;
                    v908 = v1176;
                }
                let v244 = v243.exp();
                let v245 = v242 * v244;
                let v1193 = (v907 * v244) + ((v908 * v244) * v242);
                let v247 = if v235 >= v246 { 1.0 } else { 0.0 };
                let v261: f64;
                let v909: Lanes<3>;
                if v247 != 0.0 {
                    v261 = v235;
                    v909 = v1185;
                } else {
                    let v249 = if v235 <= v248 { 1.0 } else { 0.0 };
                    let v262: f64;
                    let v910: Lanes<3>;
                    if v249 != 0.0 {
                        let v250 = v235.exp();
                        let v1197 = v1185 * v250;
                        v262 = v250;
                        v910 = v1197;
                    } else {
                        let v251 = v235.exp();
                        let v252 = v251 + v19;
                        let v253 = v252.ln();
                        let v1196 = (v1185 * v251) * (v893 / v252);
                        v262 = v253;
                        v910 = v1196;
                    }
                    v261 = v262;
                    v909 = v910;
                }
                let v254 = if v237 >= v246 { 1.0 } else { 0.0 };
                let v263: f64;
                let v911: f64;
                if v254 != 0.0 {
                    v263 = v237;
                    v911 = v1189;
                } else {
                    let v256 = if v237 <= v255 { 1.0 } else { 0.0 };
                    let v264: f64;
                    let v912: f64;
                    if v256 != 0.0 {
                        let v257 = v237.exp();
                        let v1201 = v1189 * v257;
                        v264 = v257;
                        v912 = v1201;
                    } else {
                        let v258 = v237.exp();
                        let v259 = v258 + v19;
                        let v260 = v259.ln();
                        let v1200 = (v1189 * v258) * (v893 / v259);
                        v264 = v260;
                        v912 = v1200;
                    }
                    v263 = v264;
                    v911 = v912;
                }
                let v265 = v261 - v263;
                let v266 = v245 - v19;
                let v270 = v219.abs();
                let v271 = v270.powf(v109);
                let v1221 = (v1157 * ((v1214 * (if v219 >= v1212 { 1.0 } else { 0.0 })) - v893)) * (v109 * (v270.powf((v109 - v893))));
                let v273 = v19 + (v269 * v271);
                let v274 = (v99 * v265) / v273;
                let v275 = (v79 * v266) - v274;
                let v1232 = ((Lanes([(v1015 * v266), 0.0, 0.0])) + (v1193 * v79)) - ((((Lanes([(v1031 * v265), 0.0, 0.0])) + ((v909 - (Lanes([v911, 0.0, 0.0]))) * v99)) - ((((Lanes([0.0, v1221[0], v1221[1]])) + (Lanes([(v1035 * (v271 * (v270.ln()))), 0.0, 0.0]))) * v269) * v274)) / v273);
                v444 = v275;
                v906 = v1232;
            } else {
                v444 = v0;
                v906 = v1170;
            }
            let v276 = if v82 > v0 { 1.0 } else { 0.0 };
            let v445: f64;
            let v913: Lanes<3>;
            if v276 != 0.0 {
                let v278 = v277 - v219;
                let v280 = if v278 >= v279 { v278 } else { v279 };
                let v1237 = (v1157 * v281) * v277;
                let v285 = v284 * v41;
                let v286 = v285 * v280;
                let v1240 = ((v1157 * v987) * (if v278 >= v279 { 1.0 } else { 0.0 })) * v285;
                let v287 = ((v281 * v219) * v277) / v286;
                let v1247 = ((Lanes([0.0, v1237[0], v1237[1]])) - (((Lanes([((v994 * v284) * v280), 0.0, 0.0])) + (Lanes([0.0, v1240[0], v1240[1]]))) * v287)) / v286;
                let v288 = if v287 > v238 { 1.0 } else { 0.0 };
                let v291: f64;
                let v292: f64;
                let v914: Lanes<3>;
                let v915: Lanes<3>;
                if v288 != 0.0 {
                    let v290 = v19 + (v287 - v238);
                    v291 = v290;
                    v292 = v238;
                    v914 = v1247;
                    v915 = v1170;
                } else {
                    v291 = v19;
                    v292 = v287;
                    v914 = v1170;
                    v915 = v1247;
                }
                let v293 = v292.exp();
                let v295 = (v291 * v293) - v19;
                let v296 = v82 * v295;
                let v1255 = (Lanes([(v1017 * v295), 0.0, 0.0])) + (((v914 * v293) + ((v915 * v293) * v291)) * v82);
                v445 = v296;
                v913 = v1255;
            } else {
                v445 = v0;
                v913 = v1170;
            }
            let v297 = if v88 > v0 { 1.0 } else { 0.0 };
            let v448: f64;
            let v916: Lanes<3>;
            if v297 != 0.0 {
                let v298 = v84 * v41;
                let v299 = v219 / v298;
                let v1261 = ((Lanes([0.0, v1157[0], v1157[1]])) - (Lanes([((v994 * v84) * v299), 0.0, 0.0]))) / v298;
                let v1262 = v1157 * v987;
                let v303 = v302 * v41;
                let v1266 = v994 * v302;
                let v304 = ((-v219) - v104) / v303;
                let v1270 = (((Lanes([0.0, v1262[0], v1262[1]])) - (Lanes([v1033, 0.0, 0.0]))) - (Lanes([(v1266 * v304), 0.0, 0.0]))) / v303;
                let v306 = (-v104) / v303;
                let v1274 = ((v1033 * v987) - (v1266 * v306)) / v303;
                let v307 = if v299 > v238 { 1.0 } else { 0.0 };
                let v310: f64;
                let v311: f64;
                let v917: Lanes<3>;
                let v918: Lanes<3>;
                if v307 != 0.0 {
                    let v309 = v19 + (v299 - v238);
                    v310 = v309;
                    v311 = v238;
                    v917 = v1261;
                    v918 = v1170;
                } else {
                    v310 = v19;
                    v311 = v299;
                    v917 = v1170;
                    v918 = v1261;
                }
                let v312 = v311.exp();
                let v313 = v310 * v312;
                let v1278 = (v917 * v312) + ((v918 * v312) * v310);
                let v314 = if v304 >= v246 { 1.0 } else { 0.0 };
                let v328: f64;
                let v919: Lanes<3>;
                if v314 != 0.0 {
                    v328 = v304;
                    v919 = v1270;
                } else {
                    let v316 = if v304 <= v315 { 1.0 } else { 0.0 };
                    let v329: f64;
                    let v920: Lanes<3>;
                    if v316 != 0.0 {
                        let v317 = v304.exp();
                        let v1282 = v1270 * v317;
                        v329 = v317;
                        v920 = v1282;
                    } else {
                        let v318 = v304.exp();
                        let v319 = v318 + v19;
                        let v320 = v319.ln();
                        let v1281 = (v1270 * v318) * (v893 / v319);
                        v329 = v320;
                        v920 = v1281;
                    }
                    v328 = v329;
                    v919 = v920;
                }
                let v321 = if v306 >= v246 { 1.0 } else { 0.0 };
                let v330: f64;
                let v921: f64;
                if v321 != 0.0 {
                    v330 = v306;
                    v921 = v1274;
                } else {
                    let v323 = if v306 <= v322 { 1.0 } else { 0.0 };
                    let v331: f64;
                    let v922: f64;
                    if v323 != 0.0 {
                        let v324 = v306.exp();
                        let v1286 = v1274 * v324;
                        v331 = v324;
                        v922 = v1286;
                    } else {
                        let v325 = v306.exp();
                        let v326 = v325 + v19;
                        let v327 = v326.ln();
                        let v1285 = (v1274 * v325) * (v893 / v326);
                        v331 = v327;
                        v922 = v1285;
                    }
                    v330 = v331;
                    v921 = v922;
                }
                let v333 = v313 - v19;
                let v336 = v219.abs();
                let v337 = v336.powf(v109);
                let v1301 = (v1157 * ((v1214 * (if v219 >= v1212 { 1.0 } else { 0.0 })) - v893)) * (v109 * (v336.powf((v109 - v893))));
                let v339 = v19 + (v269 * v337);
                let v340 = (v0 * (v328 - v330)) / v339;
                let v341 = (v88 * v333) - v340;
                let v1312 = ((Lanes([(v1023 * v333), 0.0, 0.0])) + (v1278 * v88)) - ((((v919 - (Lanes([v921, 0.0, 0.0]))) * v0) - ((((Lanes([0.0, v1301[0], v1301[1]])) + (Lanes([(v1035 * (v337 * (v336.ln()))), 0.0, 0.0]))) * v269) * v340)) / v339);
                v448 = v341;
                v916 = v1312;
            } else {
                v448 = v0;
                v916 = v1170;
            }
            let v450: f64;
            let v923: Lanes<3>;
            if v227 != 0.0 {
                let v343 = v342 * v41;
                let v344 = v30 / v343;
                let v1319 = ((Lanes([0.0, v984[0], v984[1]])) - (Lanes([((v994 * v342) * v344), 0.0, 0.0]))) / v343;
                let v1320 = v984 * v987;
                let v347 = v302 * v41;
                let v1324 = v994 * v302;
                let v348 = ((-v30) - v104) / v347;
                let v1328 = (((Lanes([0.0, v1320[0], v1320[1]])) - (Lanes([v1033, 0.0, 0.0]))) - (Lanes([(v1324 * v348), 0.0, 0.0]))) / v347;
                let v350 = (-v104) / v347;
                let v1332 = ((v1033 * v987) - (v1324 * v350)) / v347;
                let v351 = if v344 > v238 { 1.0 } else { 0.0 };
                let v354: f64;
                let v355: f64;
                let v924: Lanes<3>;
                let v925: Lanes<3>;
                if v351 != 0.0 {
                    let v353 = v19 + (v344 - v238);
                    v354 = v353;
                    v355 = v238;
                    v924 = v1319;
                    v925 = v1313;
                } else {
                    v354 = v19;
                    v355 = v344;
                    v924 = v1313;
                    v925 = v1319;
                }
                let v356 = v355.exp();
                let v357 = v354 * v356;
                let v1336 = (v924 * v356) + ((v925 * v356) * v354);
                let v358 = if v348 >= v246 { 1.0 } else { 0.0 };
                let v372: f64;
                let v926: Lanes<3>;
                if v358 != 0.0 {
                    v372 = v348;
                    v926 = v1328;
                } else {
                    let v360 = if v348 <= v359 { 1.0 } else { 0.0 };
                    let v373: f64;
                    let v927: Lanes<3>;
                    if v360 != 0.0 {
                        let v361 = v348.exp();
                        let v1340 = v1328 * v361;
                        v373 = v361;
                        v927 = v1340;
                    } else {
                        let v362 = v348.exp();
                        let v363 = v362 + v19;
                        let v364 = v363.ln();
                        let v1339 = (v1328 * v362) * (v893 / v363);
                        v373 = v364;
                        v927 = v1339;
                    }
                    v372 = v373;
                    v926 = v927;
                }
                let v365 = if v350 >= v246 { 1.0 } else { 0.0 };
                let v374: f64;
                let v928: f64;
                if v365 != 0.0 {
                    v374 = v350;
                    v928 = v1332;
                } else {
                    let v367 = if v350 <= v366 { 1.0 } else { 0.0 };
                    let v375: f64;
                    let v929: f64;
                    if v367 != 0.0 {
                        let v368 = v350.exp();
                        let v1344 = v1332 * v368;
                        v375 = v368;
                        v929 = v1344;
                    } else {
                        let v369 = v350.exp();
                        let v370 = v369 + v19;
                        let v371 = v370.ln();
                        let v1343 = (v1332 * v369) * (v893 / v370);
                        v375 = v371;
                        v929 = v1343;
                    }
                    v374 = v375;
                    v928 = v929;
                }
                let v376 = v372 - v374;
                let v377 = v357 - v19;
                let v380 = v30.abs();
                let v381 = v380.powf(v109);
                let v1362 = (v984 * ((v1214 * (if v30 >= v1212 { 1.0 } else { 0.0 })) - v893)) * (v109 * (v380.powf((v109 - v893))));
                let v383 = v19 + (v269 * v381);
                let v384 = (v114 * v376) / v383;
                let v385 = (v79 * v377) - v384;
                let v1373 = ((Lanes([(v1015 * v377), 0.0, 0.0])) + (v1336 * v79)) - ((((Lanes([(v1037 * v376), 0.0, 0.0])) + ((v926 - (Lanes([v928, 0.0, 0.0]))) * v114)) - ((((Lanes([0.0, v1362[0], v1362[1]])) + (Lanes([(v1035 * (v381 * (v380.ln()))), 0.0, 0.0]))) * v269) * v384)) / v383);
                v450 = v385;
                v923 = v1373;
            } else {
                v450 = v0;
                v923 = v1313;
            }
            let v386 = if v94 > v0 { 1.0 } else { 0.0 };
            let v452: f64;
            let v930: Lanes<3>;
            if v386 != 0.0 {
                let v387 = v90 * v41;
                let v388 = v30 / v387;
                let v1379 = ((Lanes([0.0, v984[0], v984[1]])) - (Lanes([((v994 * v90) * v388), 0.0, 0.0]))) / v387;
                let v1380 = v984 * v987;
                let v391 = v302 * v41;
                let v1384 = v994 * v302;
                let v392 = ((-v30) - v104) / v391;
                let v1388 = (((Lanes([0.0, v1380[0], v1380[1]])) - (Lanes([v1033, 0.0, 0.0]))) - (Lanes([(v1384 * v392), 0.0, 0.0]))) / v391;
                let v394 = (-v104) / v391;
                let v1392 = ((v1033 * v987) - (v1384 * v394)) / v391;
                let v395 = if v388 > v238 { 1.0 } else { 0.0 };
                let v398: f64;
                let v399: f64;
                let v931: Lanes<3>;
                let v932: Lanes<3>;
                if v395 != 0.0 {
                    let v397 = v19 + (v388 - v238);
                    v398 = v397;
                    v399 = v238;
                    v931 = v1379;
                    v932 = v1313;
                } else {
                    v398 = v19;
                    v399 = v388;
                    v931 = v1313;
                    v932 = v1379;
                }
                let v400 = v399.exp();
                let v401 = v398 * v400;
                let v1396 = (v931 * v400) + ((v932 * v400) * v398);
                let v402 = if v392 >= v246 { 1.0 } else { 0.0 };
                let v416: f64;
                let v933: Lanes<3>;
                if v402 != 0.0 {
                    v416 = v392;
                    v933 = v1388;
                } else {
                    let v404 = if v392 <= v403 { 1.0 } else { 0.0 };
                    let v417: f64;
                    let v934: Lanes<3>;
                    if v404 != 0.0 {
                        let v405 = v392.exp();
                        let v1400 = v1388 * v405;
                        v417 = v405;
                        v934 = v1400;
                    } else {
                        let v406 = v392.exp();
                        let v407 = v406 + v19;
                        let v408 = v407.ln();
                        let v1399 = (v1388 * v406) * (v893 / v407);
                        v417 = v408;
                        v934 = v1399;
                    }
                    v416 = v417;
                    v933 = v934;
                }
                let v409 = if v394 >= v246 { 1.0 } else { 0.0 };
                let v418: f64;
                let v935: f64;
                if v409 != 0.0 {
                    v418 = v394;
                    v935 = v1392;
                } else {
                    let v411 = if v394 <= v410 { 1.0 } else { 0.0 };
                    let v419: f64;
                    let v936: f64;
                    if v411 != 0.0 {
                        let v412 = v394.exp();
                        let v1404 = v1392 * v412;
                        v419 = v412;
                        v936 = v1404;
                    } else {
                        let v413 = v394.exp();
                        let v414 = v413 + v19;
                        let v415 = v414.ln();
                        let v1403 = (v1392 * v413) * (v893 / v414);
                        v419 = v415;
                        v936 = v1403;
                    }
                    v418 = v419;
                    v935 = v936;
                }
                let v421 = v401 - v19;
                let v424 = v30.abs();
                let v427 = v19 + (v269 * (v424.powf(v105)));
                let v428 = (v0 * (v416 - v418)) / v427;
                let v1421 = (((v984 * ((v1214 * (if v30 >= v1212 { 1.0 } else { 0.0 })) - v893)) * (v105 * (v424.powf((v105 - v893))))) * v269) * v428;
                let v429 = (v94 * v421) - v428;
                let v1425 = ((Lanes([(v1029 * v421), 0.0, 0.0])) + (v1396 * v94)) - ((((v933 - (Lanes([v935, 0.0, 0.0]))) * v0) - (Lanes([0.0, v1421[0], v1421[1]]))) / v427);
                v452 = v429;
                v930 = v1425;
            } else {
                v452 = v0;
                v930 = v1313;
            }
            let v1426 = Lanes([v1157[0], v1157[1], 0.0]);
            let v1427 = Lanes([0.0, 0.0, v900]);
            let v432 = -(v219 - v430);
            let v1429 = (v1426 - v1427) * v987;
            let v434 = v430 * v433;
            let v1430 = v900 * v433;
            let v437 = v435 * (ddt(3355, v430));
            let v1433 = (v900 * v1431) * v435;
            let v885 = v435 * v430;
            let v1434 = v900 * v435;
            let v439 = v219.abs();
            let v441 = if v439 >= v440 { v439 } else { v440 };
            let v442 = (if v430 <= v219 { v430 } else { v219 }) / v441;
            let v1445 = ((v1157 * ((v1214 * (if v219 >= v1212 { 1.0 } else { 0.0 })) - v893)) * (if v439 >= v440 { 1.0 } else { 0.0 })) * v442;
            let v443 = v442.abs();
            let v1453 = v906 - v913;
            let v447 = (v444 - v445) / v49;
            let v1454 = v1005 * v447;
            let v449 = v447 + v448;
            let v1460 = (((Lanes([v1453[0], 0.0, v1453[1], v1453[2]])) - (Lanes([v1454[0], v1454[1], v1454[2], 0.0]))) / v49) + (Lanes([v916[0], 0.0, v916[1], v916[2]]));
            let v451 = v450 / v51;
            let v453 = v451 + v452;
            let v1465 = ((v923 - (Lanes([(v1006 * v451), 0.0, 0.0]))) / v51) + v930;
            let v457 = v63 * (v19 + (v30 * v454));
            let v1468 = v906 * v457;
            let v1469 = ((v984 * v454) * v63) * v444;
            let v1473 = v923 * v67;
            let v1477 = (v1157 * v59) * v987;
            let v1478 = v984 * v55;
            let v467 = v19 + (v465 * ((v444 * v457) + (v450 * v67)));
            let v468 = v467.abs();
            let v471 = v19 + (v468.powf(v469));
            let v1491 = ((Lanes([0.0, v1477[0], v1477[1]])) - (Lanes([v1478[0], v1478[1], 0.0]))) * v472;
            let v474 = (v472 * ((v19 - (v219 * v59)) - (v30 * v55))) / v471;
            let v1495 = ((Lanes([0.0, v1491[0], v1491[1], v1491[2]])) - (((((((Lanes([v1468[0], 0.0, v1468[1], v1468[2]])) + (Lanes([0.0, v1469[0], v1469[1], 0.0]))) + (Lanes([v1473[0], v1473[1], v1473[2], 0.0]))) * v465) * ((v1214 * (if v467 >= v1212 { 1.0 } else { 0.0 })) - v893)) * (v469 * (v468.powf((v469 - v893))))) * v474)) / v471;
            let v475 = v450 * v474;
            let v1496 = v923 * v474;
            let v1499 = (Lanes([v1496[0], v1496[1], v1496[2], 0.0])) + (v1495 * v450);
            let v476 = v444 * v474;
            let v1500 = v906 * v474;
            let v1503 = (Lanes([v1500[0], 0.0, v1500[1], v1500[2]])) + (v1495 * v444);
            let v1504 = v1503 * v443;
            let v1505 = ((((v1426 + ((v1427 - v1426) * (if v430 <= v219 { 1.0 } else { 0.0 }))) - (Lanes([v1445[0], v1445[1], 0.0]))) / v441) * ((v1214 * (if v442 >= v1212 { 1.0 } else { 0.0 })) - v893)) * v476;
            let v480 = v19 - v478;
            let v481 = v480 * v444;
            let v1511 = (v906 * v480) * v474;
            let v1514 = (Lanes([v1511[0], 0.0, v1511[1], v1511[2]])) + (v1495 * v481);
            let v483 = ((v476 * v443) * v478) + (v481 * v474);
            let v1516 = (((Lanes([v1504[0], v1504[1], v1504[2], v1504[3], 0.0])) + (Lanes([0.0, 0.0, v1505[0], v1505[1], v1505[2]]))) * v478) + (Lanes([v1514[0], v1514[1], v1514[2], v1514[3], 0.0]));
            let v485 = v224 / v484;
            let v486 = v485.abs();
            let v489 = v19 + (v486.powf(v487));
            let v491 = v226 / v490;
            let v492 = v491.abs();
            let v495 = v19 + (v492.powf(v493));
            let v499 = (v43 * v497).exp();
            let v500 = v496 * v499;
            let v501 = v19 / v487;
            let v502 = v489.powf(v501);
            let v503 = v500 * v502;
            let v1543 = ((((v1165 / v484) * ((v1214 * (if v485 >= v1212 { 1.0 } else { 0.0 })) - v893)) * (v487 * (v486.powf((v487 - v893))))) * (v501 * (v489.powf((v501 - v893))))) * v500;
            let v1546 = (Lanes([0.0, ((((v997 * v497) * v499) * v496) * v502), 0.0])) + (Lanes([v1543[0], 0.0, v1543[1]]));
            let v507 = (v43 * v505).exp();
            let v508 = v504 * v507;
            let v1549 = ((v997 * v505) * v507) * v504;
            let v512 = (v43 * v510).exp();
            let v513 = v509 * v512;
            let v514 = v19 / v493;
            let v515 = v495.powf(v514);
            let v516 = v513 * v515;
            let v1558 = ((((v1169 / v490) * ((v1214 * (if v491 >= v1212 { 1.0 } else { 0.0 })) - v893)) * (v493 * (v492.powf((v493 - v893))))) * (v514 * (v495.powf((v514 - v893))))) * v513;
            let v1561 = (Lanes([0.0, ((((v997 * v510) * v512) * v509) * v515), 0.0])) + (Lanes([v1558[0], 0.0, v1558[1]]));
            let v517 = v220 - v214;
            let v1564 = (Lanes([v899, 0.0])) - (Lanes([0.0, v897]));
            let v519 = v517 / v518;
            let v520 = v519.abs();
            let v523 = v19 + (v520.powf(v521));
            let v524 = v19 / v521;
            let v531 = v527 * (v19 + (v528 * ((v523.powf(v524)) - v19)));
            let v1579 = (((((v1564 / v518) * ((v1214 * (if v519 >= v1212 { 1.0 } else { 0.0 })) - v893)) * (v521 * (v520.powf((v521 - v893))))) * (v524 * (v523.powf((v524 - v893))))) * v528) * v527;
            let v532 = v531 * v444;
            let v1580 = v1579 * v444;
            let v1581 = v906 * v531;
            let v1584 = (Lanes([v1580[0], v1580[1], 0.0, 0.0, 0.0])) + (Lanes([0.0, 0.0, v1581[0], v1581[1], v1581[2]]));
            let v534 = v533 * v475;
            let v1585 = v1499 * v533;
            let v536 = if v535 == v19 { 1.0 } else { 0.0 };
            let v553: f64;
            let v846: f64;
            let v847: f64;
            let v848: f64;
            let v849: f64;
            let v887: f64;
            let v937: Lanes<4>;
            let v938: Lanes<6>;
            let v939: f64;
            let v940: Lanes<3>;
            let v941: Lanes<3>;
            if v536 != 0.0 {
                let v537 = v444 / v49;
                let v1590 = v1005 * v537;
                let v538 = -v537;
                let v539 = v538 * v531;
                let v1596 = ((((Lanes([v906[0], 0.0, v906[1], v906[2]])) - (Lanes([v1590[0], v1590[1], v1590[2], 0.0]))) / v49) * v987) * v531;
                let v1597 = v1579 * v538;
                let v1600 = (Lanes([0.0, 0.0, v1596[0], v1596[1], v1596[2], v1596[3]])) + (Lanes([v1597[0], v1597[1], 0.0, 0.0, 0.0, 0.0]));
                let v541 = ddt(3525, v540);
                let v542 = v531 * v541;
                let v1602 = v1579 * v541;
                let v1606 = (Lanes([v1602[0], v1602[1], 0.0])) + (Lanes([0.0, 0.0, ((v901 * v1431) * v531)]));
                let v886 = v531 * v540;
                let v1607 = v1579 * v540;
                let v1611 = (Lanes([v1607[0], v1607[1], 0.0])) + (Lanes([0.0, 0.0, (v901 * v531)]));
                let v545 = (v540.abs()) / v544;
                let v548 = v19 + (v545.powf(v546));
                let v549 = v503 / v548;
                let v1625 = ((Lanes([v1546[0], v1546[1], v1546[2], 0.0])) - (Lanes([0.0, 0.0, 0.0, ((((v901 * ((v1214 * (if v540 >= v1212 { 1.0 } else { 0.0 })) - v893)) / v544) * (v546 * (v545.powf((v546 - v893))))) * v549)]))) / v548;
                v553 = v549;
                v846 = v539;
                v847 = v540;
                v848 = v542;
                v849 = v0;
                v887 = v886;
                v937 = v1625;
                v938 = v1600;
                v939 = v901;
                v940 = v1606;
                v941 = v1611;
            } else {
                let v1586 = Lanes([v1546[0], v1546[1], v1546[2], 0.0]);
                v553 = v503;
                v846 = v0;
                v847 = v0;
                v848 = v0;
                v849 = v550;
                v887 = v0;
                v937 = v1586;
                v938 = v1587;
                v939 = v1588;
                v940 = v1589;
                v941 = v1589;
            }
            let v552 = if v551 == v19 { 1.0 } else { 0.0 };
            let v771: f64;
            let v782: f64;
            let v794: f64;
            let v942: Lanes<4>;
            let v943: Lanes<3>;
            let v944: f64;
            if v552 != 0.0 {
                let v555 = v553 + v554;
                let v557 = v508 + v556;
                let v559 = v516 + v558;
                v771 = v555;
                v782 = v559;
                v794 = v557;
                v942 = v937;
                v943 = v1561;
                v944 = v1549;
            } else {
                v771 = v553;
                v782 = v516;
                v794 = v508;
                v942 = v937;
                v943 = v1561;
                v944 = v1549;
            }
            let v560 = if v216 <= v0 { 1.0 } else { 0.0 };
            let v826: f64;
            let v945: Lanes<3>;
            if v560 != 0.0 {
                let v561 = v213 * v207;
                let v562 = v19 - v201;
                let v563 = v216 / v207;
                let v564 = v19 - v563;
                let v567 = (v562 * (v564.ln())).exp();
                let v568 = v19 - v567;
                let v570 = (v561 * v568) / v562;
                let v1658 = ((Lanes([0.0, (((v1149 * v207) + (v1140 * v213)) * v568), 0.0])) + (((((((((Lanes([v1153[0], 0.0, v1153[1]])) - (Lanes([0.0, (v1140 * v563), 0.0]))) / v207) * v987) * (v893 / v564)) * v562) * v567) * v987) * v561)) / v562;
                v826 = v570;
                v945 = v1658;
            } else {
                let v571 = v213 * v216;
                let v1627 = v1153 * v213;
                let v573 = v572 * v201;
                let v1631 = v1153 * v573;
                let v575 = (v573 * v216) / v207;
                let v576 = v19 + v575;
                let v577 = v571 * v576;
                let v1639 = (((Lanes([0.0, (v1149 * v216), 0.0])) + (Lanes([v1627[0], 0.0, v1627[1]]))) * v576) + ((((Lanes([v1631[0], 0.0, v1631[1]])) - (Lanes([0.0, (v1140 * v575), 0.0]))) / v207) * v571);
                v826 = v577;
                v945 = v1639;
            }
            let v581 = v219 + ((-v159) * v579);
            let v1661 = Lanes([0.0, v1157[0], v1157[1]]);
            let v1663 = v1661 + (Lanes([((v1077 * v987) * v579), 0.0, 0.0]));
            let v582 = if v581 > v0 { 1.0 } else { 0.0 };
            let v610: f64;
            let v611: f64;
            let v946: Lanes<3>;
            let v947: Lanes<3>;
            if v582 != 0.0 {
                let v585 = v19 - v579;
                let v588 = ((v583 - v150) * (v585.ln())).exp();
                let v591 = v19 - ((v588 * v585) * v585);
                let v593 = v19 - v150;
                let v594 = (v159 * v591) / v593;
                let v595 = v572 * v150;
                let v597 = (v595 * v581) / v159;
                let v598 = v585 + v597;
                let v600 = (v581 * v598) * v588;
                let v1689 = ((v1663 * v598) + ((((v1663 * v595) - (Lanes([(v1077 * v597), 0.0, 0.0]))) / v159) * v581)) * v588;
                let v1690 = Lanes([((v1077 * v591) / v593), 0.0, 0.0]);
                v610 = v594;
                v611 = v600;
                v946 = v1690;
                v947 = v1689;
            } else {
                let v601 = v19 - v150;
                let v602 = v219 / v159;
                let v603 = v19 - v602;
                let v606 = (v601 * (v603.ln())).exp();
                let v607 = v19 - v606;
                let v609 = (v159 * v607) / v601;
                let v1678 = ((Lanes([(v1077 * v607), 0.0, 0.0])) + ((((((((v1661 - (Lanes([(v1077 * v602), 0.0, 0.0]))) / v159) * v987) * (v893 / v603)) * v601) * v606) * v987) * v159)) / v601;
                v610 = v609;
                v611 = v0;
                v946 = v1678;
                v947 = v1170;
            }
            let v612 = v610 + v611;
            let v613 = v167 * v612;
            let v1695 = (Lanes([(v1087 * v612), 0.0, 0.0])) + ((v946 + v947) * v167);
            let v615 = (-v184) * v579;
            let v1697 = (v1109 * v987) * v579;
            let v616 = v222 + v615;
            let v1698 = Lanes([v1161[0], 0.0, v1161[1]]);
            let v1700 = v1698 + (Lanes([0.0, v1697, 0.0]));
            let v617 = if v616 > v0 { 1.0 } else { 0.0 };
            let v645: f64;
            let v646: f64;
            let v948: Lanes<3>;
            let v949: Lanes<3>;
            if v617 != 0.0 {
                let v620 = v19 - v579;
                let v623 = ((v618 - v178) * (v620.ln())).exp();
                let v626 = v19 - ((v623 * v620) * v620);
                let v628 = v19 - v178;
                let v629 = (v184 * v626) / v628;
                let v630 = v572 * v178;
                let v632 = (v630 * v616) / v184;
                let v633 = v620 + v632;
                let v635 = (v616 * v633) * v623;
                let v1727 = ((v1700 * v633) + ((((v1700 * v630) - (Lanes([0.0, (v1109 * v632), 0.0]))) / v184) * v616)) * v623;
                let v1728 = Lanes([0.0, ((v1109 * v626) / v628), 0.0]);
                v645 = v629;
                v646 = v635;
                v948 = v1728;
                v949 = v1727;
            } else {
                let v636 = v19 - v178;
                let v637 = v222 / v184;
                let v638 = v19 - v637;
                let v641 = (v636 * (v638.ln())).exp();
                let v642 = v19 - v641;
                let v644 = (v184 * v642) / v636;
                let v1715 = ((Lanes([0.0, (v1109 * v642), 0.0])) + ((((((((v1698 - (Lanes([0.0, (v1109 * v637), 0.0]))) / v184) * v987) * (v893 / v638)) * v636) * v641) * v987) * v184)) / v636;
                v645 = v644;
                v646 = v0;
                v948 = v1715;
                v949 = v1716;
            }
            let v647 = v645 + v646;
            let v650 = v19 - v649;
            let v651 = v650 * (v190 * v647);
            let v1734 = ((Lanes([0.0, (v1118 * v647), 0.0])) + ((v948 + v949) * v190)) * v650;
            let v652 = v30 + v615;
            let v1735 = Lanes([0.0, v984[0], v984[1]]);
            let v1737 = v1735 + (Lanes([v1697, 0.0, 0.0]));
            let v653 = if v652 > v0 { 1.0 } else { 0.0 };
            let v681: f64;
            let v682: f64;
            let v950: Lanes<3>;
            let v951: Lanes<3>;
            if v653 != 0.0 {
                let v656 = v19 - v579;
                let v659 = ((v654 - v178) * (v656.ln())).exp();
                let v662 = v19 - ((v659 * v656) * v656);
                let v664 = v19 - v178;
                let v665 = (v184 * v662) / v664;
                let v666 = v572 * v178;
                let v668 = (v666 * v652) / v184;
                let v669 = v656 + v668;
                let v671 = (v652 * v669) * v659;
                let v1763 = ((v1737 * v669) + ((((v1737 * v666) - (Lanes([(v1109 * v668), 0.0, 0.0]))) / v184) * v652)) * v659;
                let v1764 = Lanes([((v1109 * v662) / v664), 0.0, 0.0]);
                v681 = v665;
                v682 = v671;
                v950 = v1764;
                v951 = v1763;
            } else {
                let v672 = v19 - v178;
                let v673 = v30 / v184;
                let v674 = v19 - v673;
                let v677 = (v672 * (v674.ln())).exp();
                let v678 = v19 - v677;
                let v680 = (v184 * v678) / v672;
                let v1752 = ((Lanes([(v1109 * v678), 0.0, 0.0])) + ((((((((v1735 - (Lanes([(v1109 * v673), 0.0, 0.0]))) / v184) * v987) * (v893 / v674)) * v672) * v677) * v987) * v184)) / v672;
                v681 = v680;
                v682 = v0;
                v950 = v1752;
                v951 = v1313;
            }
            let v683 = v681 + v682;
            let v685 = v649 * (v190 * v683);
            let v1770 = ((Lanes([(v1118 * v683), 0.0, 0.0])) + ((v950 + v951) * v190)) * v649;
            let v689 = if (if v686 != v0 { 1.0 } else { 0.0 }) != 0.0 && (if v527 != v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v830: f64;
            let v952: Lanes<4>;
            if v689 != 0.0 {
                let v695 = (((v26 * v686) * v691) / v693) * v527;
                let v696 = v695 * v476;
                let v1772 = v1503 * v695;
                v830 = v696;
                v952 = v1772;
            } else {
                v830 = v0;
                v952 = v1771;
            }
            let v700 = if v699 > v0 { 1.0 } else { 0.0 };
            let v701 = if (if v697 == v19 { 1.0 } else { 0.0 }) != 0.0 && v700 != 0.0 { 1.0 } else { 0.0 };
            let v850: f64;
            let v851: f64;
            let v852: f64;
            let v853: f64;
            let v854: f64;
            let v856: f64;
            let v858: f64;
            let v860: f64;
            let v862: f64;
            let v864: f64;
            let v867: f64;
            let v870: f64;
            let v873: f64;
            let v888: f64;
            let v890: f64;
            let v892: f64;
            let v953: Lanes<7>;
            let v954: f64;
            let v955: f64;
            let v956: Lanes<7>;
            let v957: Lanes<2>;
            let v958: f64;
            let v959: f64;
            let v960: f64;
            let v961: Lanes<7>;
            let v962: f64;
            let v963: f64;
            let v964: f64;
            if v701 != 0.0 {
                let v703 = v449 * v517;
                let v1835 = v1460 * v517;
                let v1836 = v1564 * v449;
                let v1844 = (((Lanes([0.0, 0.0, v1835[0], v1835[1], v1835[2], v1835[3]])) + (Lanes([v1836[0], v1836[1], 0.0, 0.0, 0.0, 0.0]))) * ((v1214 * (if v703 >= v1212 { 1.0 } else { 0.0 })) - v893)) * v702;
                let v707 = v220 - v706;
                let v708 = v453 * v707;
                let v1848 = v1465 * v707;
                let v1849 = ((Lanes([0.0, v899])) - (Lanes([v902, 0.0]))) * v453;
                let v1856 = ((Lanes([0.0, 0.0, v1848[0], v1848[1], v1848[2]])) + (Lanes([v1849[0], v1849[1], 0.0, 0.0, 0.0]))) * ((v1214 * (if v708 >= v1212 { 1.0 } else { 0.0 })) - v893);
                let v710 = (v702 * (v703.abs())) - (v708.abs());
                let v1859 = (Lanes([0.0, v1844[0], v1844[1], v1844[2], v1844[3], v1844[4], v1844[5]])) - (Lanes([v1856[0], v1856[1], 0.0, v1856[2], v1856[3], v1856[4], 0.0]));
                let v711 = v2 / v699;
                let v1860 = v894 / v699;
                let v713 = v2 * v712;
                let v1861 = v894 * v712;
                let v714 = ddt(3871, v713);
                let v1862 = v1861 * v1431;
                v850 = v710;
                v851 = v711;
                v852 = v714;
                v853 = v715;
                v854 = v0;
                v856 = v0;
                v858 = v0;
                v860 = v0;
                v862 = v0;
                v864 = v0;
                v867 = v0;
                v870 = v0;
                v873 = v0;
                v888 = v713;
                v890 = v0;
                v892 = v0;
                v953 = v1859;
                v954 = v1860;
                v955 = v1862;
                v956 = v1773;
                v957 = v1799;
                v958 = v980;
                v959 = v1800;
                v960 = v1800;
                v961 = v1773;
                v962 = v1861;
                v963 = v980;
                v964 = v1800;
            } else {
                let v720 = if (if (if v697 == v472 { 1.0 } else { 0.0 }) != 0.0 && v700 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v718 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v855: f64;
                let v857: f64;
                let v859: f64;
                let v861: f64;
                let v863: f64;
                let v865: f64;
                let v868: f64;
                let v871: f64;
                let v874: f64;
                let v889: f64;
                let v891: f64;
                let v965: Lanes<7>;
                let v966: Lanes<2>;
                let v967: f64;
                let v968: f64;
                let v969: f64;
                let v970: Lanes<7>;
                let v971: f64;
                let v972: f64;
                if v720 != 0.0 {
                    let v722 = v449 * v517;
                    let v1801 = v1460 * v517;
                    let v1802 = v1564 * v449;
                    let v1810 = (((Lanes([0.0, 0.0, v1801[0], v1801[1], v1801[2], v1801[3]])) + (Lanes([v1802[0], v1802[1], 0.0, 0.0, 0.0, 0.0]))) * ((v1214 * (if v722 >= v1212 { 1.0 } else { 0.0 })) - v893)) * v721;
                    let v725 = v220 - v706;
                    let v726 = v453 * v725;
                    let v1814 = v1465 * v725;
                    let v1815 = ((Lanes([0.0, v899])) - (Lanes([v902, 0.0]))) * v453;
                    let v1822 = ((Lanes([0.0, 0.0, v1814[0], v1814[1], v1814[2]])) + (Lanes([v1815[0], v1815[1], 0.0, 0.0, 0.0]))) * ((v1214 * (if v726 >= v1212 { 1.0 } else { 0.0 })) - v893);
                    let v728 = (v721 * (v722.abs())) - (v726.abs());
                    let v1825 = (Lanes([0.0, v1810[0], v1810[1], v1810[2], v1810[3], v1810[4], v1810[5]])) - (Lanes([v1822[0], v1822[1], 0.0, v1822[2], v1822[3], v1822[4], 0.0]));
                    let v731 = (v2 - v729) / v699;
                    let v1829 = ((Lanes([v894, 0.0])) - (Lanes([0.0, v903]))) / v699;
                    let v732 = v712 * v2;
                    let v1830 = v894 * v712;
                    let v733 = ddt(3902, v732);
                    let v1831 = v1830 * v1431;
                    let v734 = v729 / v718;
                    let v1832 = v903 / v718;
                    let v736 = v735 * v729;
                    let v1833 = v903 * v735;
                    let v737 = ddt(3909, v736);
                    let v1834 = v1833 * v1431;
                    v855 = v728;
                    v857 = v731;
                    v859 = v733;
                    v861 = v734;
                    v863 = v737;
                    v865 = v0;
                    v868 = v0;
                    v871 = v0;
                    v874 = v0;
                    v889 = v732;
                    v891 = v736;
                    v965 = v1825;
                    v966 = v1829;
                    v967 = v1831;
                    v968 = v1832;
                    v969 = v1834;
                    v970 = v1773;
                    v971 = v1830;
                    v972 = v1833;
                } else {
                    let v739 = if v697 == v738 { 1.0 } else { 0.0 };
                    let v866: f64;
                    let v869: f64;
                    let v872: f64;
                    let v875: f64;
                    let v973: Lanes<7>;
                    if v739 != 0.0 {
                        let v741 = v449 * v517;
                        let v1774 = v1460 * v517;
                        let v1775 = v1564 * v449;
                        let v1783 = (((Lanes([0.0, 0.0, v1774[0], v1774[1], v1774[2], v1774[3]])) + (Lanes([v1775[0], v1775[1], 0.0, 0.0, 0.0, 0.0]))) * ((v1214 * (if v741 >= v1212 { 1.0 } else { 0.0 })) - v893)) * v740;
                        let v744 = v220 - v706;
                        let v745 = v453 * v744;
                        let v1787 = v1465 * v744;
                        let v1788 = ((Lanes([0.0, v899])) - (Lanes([v902, 0.0]))) * v453;
                        let v1795 = ((Lanes([0.0, 0.0, v1787[0], v1787[1], v1787[2]])) + (Lanes([v1788[0], v1788[1], 0.0, 0.0, 0.0]))) * ((v1214 * (if v745 >= v1212 { 1.0 } else { 0.0 })) - v893);
                        let v747 = (v740 * (v741.abs())) - (v745.abs());
                        let v1798 = (Lanes([0.0, v1783[0], v1783[1], v1783[2], v1783[3], v1783[4], v1783[5]])) - (Lanes([v1795[0], v1795[1], 0.0, v1795[2], v1795[3], v1795[4], 0.0]));
                        v866 = v747;
                        v869 = v748;
                        v872 = v0;
                        v875 = v0;
                        v973 = v1798;
                    } else {
                        v866 = v0;
                        v869 = v0;
                        v872 = v749;
                        v875 = v750;
                        v973 = v1773;
                    }
                    v855 = v0;
                    v857 = v0;
                    v859 = v0;
                    v861 = v0;
                    v863 = v0;
                    v865 = v866;
                    v868 = v869;
                    v871 = v872;
                    v874 = v875;
                    v889 = v0;
                    v891 = v0;
                    v965 = v1773;
                    v966 = v1799;
                    v967 = v980;
                    v968 = v1800;
                    v969 = v1800;
                    v970 = v973;
                    v971 = v980;
                    v972 = v1800;
                }
                v850 = v0;
                v851 = v0;
                v852 = v0;
                v853 = v0;
                v854 = v855;
                v856 = v857;
                v858 = v859;
                v860 = v861;
                v862 = v863;
                v864 = v865;
                v867 = v868;
                v870 = v871;
                v873 = v874;
                v888 = v0;
                v890 = v889;
                v892 = v891;
                v953 = v1773;
                v954 = v980;
                v955 = v980;
                v956 = v965;
                v957 = v966;
                v958 = v967;
                v959 = v968;
                v960 = v969;
                v961 = v970;
                v962 = v980;
                v963 = v971;
                v964 = v972;
            }
            let v751 = ctx.simparam_or("gmin", v0);
            let v752 = v751 * v218;
            let v1863 = v1156 * v751;
            let v753 = ctx.simparam_or("gmin", v0);
            let v754 = v753 * v29;
            let v1864 = v983 * v753;
            let v755 = ctx.simparam_or("gmin", v0);
            let v757 = v755 * (v28 - v217);
            let v1868 = ((Lanes([v896, 0.0])) - (Lanes([0.0, v898]))) * v755;
            let v760 = (v496 + (v551 * v554)) / v25;
            let v763 = (v509 + (v551 * v558)) / v25;
            let v766 = (v504 + (v551 * v556)) / v25;
            let v770 = if (if v760 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v760 >= v768 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v876: f64;
            let v877: f64;
            let v878: f64;
            let v974: Lanes<4>;
            if v770 != 0.0 {
                let v772 = v771 / v25;
                let v1870 = v942 / v25;
                let v773 = if v772 > v768 { 1.0 } else { 0.0 };
                let v774: f64;
                let v975: Lanes<4>;
                if v773 != 0.0 {
                    v774 = v772;
                    v975 = v1870;
                } else {
                    v774 = v768;
                    v975 = v1869;
                }
                let v775 = v223 / v774;
                let v1874 = ((Lanes([v1164[0], 0.0, v1164[1], 0.0])) - (v975 * v775)) / v774;
                let v776 = if v772 >= v768 { 1.0 } else { 0.0 };
                if v776 != 0.0 {
                } else {
                }
                v876 = v775;
                v877 = v777;
                v878 = v0;
                v974 = v1874;
            } else {
                v876 = v0;
                v877 = v0;
                v878 = v778;
                v974 = v1869;
            }
            let v781 = if (if v763 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v763 >= v768 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v879: f64;
            let v880: f64;
            let v881: f64;
            let v976: Lanes<3>;
            if v781 != 0.0 {
                let v783 = v782 / v25;
                let v1876 = v943 / v25;
                let v784 = if v783 > v768 { 1.0 } else { 0.0 };
                let v785: f64;
                let v977: Lanes<3>;
                if v784 != 0.0 {
                    v785 = v783;
                    v977 = v1876;
                } else {
                    v785 = v768;
                    v977 = v1875;
                }
                let v786 = v225 / v785;
                let v1880 = ((Lanes([v1168[0], 0.0, v1168[1]])) - (v977 * v786)) / v785;
                let v787 = if v783 >= v768 { 1.0 } else { 0.0 };
                if v787 != 0.0 {
                } else {
                }
                v879 = v786;
                v880 = v788;
                v881 = v0;
                v976 = v1880;
            } else {
                v879 = v0;
                v880 = v0;
                v881 = v789;
                v976 = v1875;
            }
            let v792 = if (if v766 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v766 >= v768 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v882: f64;
            let v883: f64;
            let v884: f64;
            let v978: Lanes<3>;
            if v792 != 0.0 {
                let v793 = v706 - v28;
                let v1884 = (Lanes([v902, 0.0])) - (Lanes([0.0, v896]));
                let v795 = v794 / v25;
                let v1885 = v944 / v25;
                let v796 = if v795 > v768 { 1.0 } else { 0.0 };
                let v797: f64;
                let v979: f64;
                if v796 != 0.0 {
                    v797 = v795;
                    v979 = v1885;
                } else {
                    v797 = v768;
                    v979 = v980;
                }
                let v798 = v793 / v797;
                let v1890 = ((Lanes([v1884[0], 0.0, v1884[1]])) - (Lanes([0.0, (v979 * v798), 0.0]))) / v797;
                let v799 = if v795 >= v768 { 1.0 } else { 0.0 };
                if v799 != 0.0 {
                } else {
                }
                v882 = v798;
                v883 = v800;
                v884 = v0;
                v978 = v1890;
            } else {
                v882 = v0;
                v883 = v0;
                v884 = v801;
                v978 = v1881;
            }
            let v802 = v26 * v449;
            let v803 = v802 * v25;
            let v1892 = (v1460 * v26) * v25;
            let v805 = (v26 * v453) * v25;
            let v1894 = (v1465 * v26) * v25;
            let v808 = v26 * ((-v475) * v25);
            let v1897 = ((v1499 * v987) * v25) * v26;
            let v810 = (v26 * v483) * v25;
            let v1899 = (v1516 * v26) * v25;
            let v812 = (v26 * v613) * v25;
            let v1901 = (v1695 * v26) * v25;
            let v813 = ddt(4101, v812);
            let v1902 = v1901 * v1431;
            let v815 = (v26 * v532) * v25;
            let v1904 = (v1584 * v26) * v25;
            let v816 = ddt(4107, v815);
            let v1905 = v1904 * v1431;
            let v818 = (v26 * v651) * v25;
            let v1907 = (v1734 * v26) * v25;
            let v819 = ddt(4113, v818);
            let v1908 = v1907 * v1431;
            let v821 = (v26 * v685) * v25;
            let v1910 = (v1770 * v26) * v25;
            let v822 = ddt(4119, v821);
            let v1911 = v1910 * v1431;
            let v824 = (v26 * v534) * v25;
            let v1913 = (v1585 * v26) * v25;
            let v825 = ddt(4125, v824);
            let v1914 = v1913 * v1431;
            let v828 = (v26 * v826) * v25;
            let v1916 = (v945 * v26) * v25;
            let v829 = ddt(4131, v828);
            let v1917 = v1916 * v1431;
            let v832 = (-v830) * v25;
            let v1919 = (v952 * v987) * v25;
            let v833 = ddt(4136, v832);
            let v1920 = v1919 * v1431;
            let v834 = v830 * v25;
            let v1921 = v952 * v25;
            let v835 = ddt(4140, v834);
            let v1922 = v1921 * v1431;
            let v841 = if (if (if v836 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v838 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) > v0 { 1.0 } else { 0.0 };
            if v841 != 0.0 {
            } else {
            }
            let v843 = if v802 >= v0 { 1.0 } else { 0.0 };
            if v843 != 0.0 {
            } else {
            }
            let v1923 = v1429[0];
            let v1924 = v1429[1];
            let v1925 = v1429[2];
            let v1926 = v1430;
            let v1927 = v1433;
            let v1928 = v938[0];
            let v1929 = v938[1];
            let v1930 = v938[2];
            let v1931 = v938[3];
            let v1932 = v938[4];
            let v1933 = v938[5];
            let v1934 = v939;
            let v1935 = v940[0];
            let v1936 = v940[1];
            let v1937 = v940[2];
            let v1938 = v953[0];
            let v1939 = v953[1];
            let v1940 = v953[2];
            let v1941 = v953[3];
            let v1942 = v953[4];
            let v1943 = v953[5];
            let v1944 = v953[6];
            let v1945 = v954;
            let v1946 = v955;
            let v1947 = v956[0];
            let v1948 = v956[1];
            let v1949 = v956[2];
            let v1950 = v956[3];
            let v1951 = v956[4];
            let v1952 = v956[5];
            let v1953 = v956[6];
            let v1954 = v957[0];
            let v1955 = v957[1];
            let v1956 = v958;
            let v1957 = v959;
            let v1958 = v960;
            let v1959 = v961[0];
            let v1960 = v961[1];
            let v1961 = v961[2];
            let v1962 = v961[3];
            let v1963 = v961[4];
            let v1964 = v961[5];
            let v1965 = v961[6];
            let v1966 = v1863[0];
            let v1967 = v1863[1];
            let v1968 = v1864[0];
            let v1969 = v1864[1];
            let v1970 = v1868[0];
            let v1971 = v1868[1];
            let v1972 = v974[0];
            let v1973 = v974[1];
            let v1974 = v974[2];
            let v1975 = v974[3];
            let v1976 = v976[0];
            let v1977 = v976[1];
            let v1978 = v976[2];
            let v1979 = v978[0];
            let v1980 = v978[1];
            let v1981 = v978[2];
            let v1982 = v1892[0];
            let v1983 = v1892[1];
            let v1984 = v1892[2];
            let v1985 = v1892[3];
            let v1986 = v1894[0];
            let v1987 = v1894[1];
            let v1988 = v1894[2];
            let v1989 = v1897[0];
            let v1990 = v1897[1];
            let v1991 = v1897[2];
            let v1992 = v1897[3];
            let v1993 = v1899[0];
            let v1994 = v1899[1];
            let v1995 = v1899[2];
            let v1996 = v1899[3];
            let v1997 = v1899[4];
            let v1998 = v1902[0];
            let v1999 = v1902[1];
            let v2000 = v1902[2];
            let v2001 = v1905[0];
            let v2002 = v1905[1];
            let v2003 = v1905[2];
            let v2004 = v1905[3];
            let v2005 = v1905[4];
            let v2006 = v1908[0];
            let v2007 = v1908[1];
            let v2008 = v1908[2];
            let v2009 = v1911[0];
            let v2010 = v1911[1];
            let v2011 = v1911[2];
            let v2012 = v1914[0];
            let v2013 = v1914[1];
            let v2014 = v1914[2];
            let v2015 = v1914[3];
            let v2016 = v1917[0];
            let v2017 = v1917[1];
            let v2018 = v1917[2];
            let v2019 = v1920[0];
            let v2020 = v1920[1];
            let v2021 = v1920[2];
            let v2022 = v1920[3];
            let v2023 = v1922[0];
            let v2024 = v1922[1];
            let v2025 = v1922[2];
            let v2026 = v1922[3];
            let v2027 = v1434;
            let v2028 = v941[0];
            let v2029 = v941[1];
            let v2030 = v941[2];
            let v2031 = v962;
            let v2032 = v963;
            let v2033 = v964;
            let v2034 = v1901[0];
            let v2035 = v1901[1];
            let v2036 = v1901[2];
            let v2037 = v1904[0];
            let v2038 = v1904[1];
            let v2039 = v1904[2];
            let v2040 = v1904[3];
            let v2041 = v1904[4];
            let v2042 = v1907[0];
            let v2043 = v1907[1];
            let v2044 = v1907[2];
            let v2045 = v1910[0];
            let v2046 = v1910[1];
            let v2047 = v1910[2];
            let v2048 = v1913[0];
            let v2049 = v1913[1];
            let v2050 = v1913[2];
            let v2051 = v1913[3];
            let v2052 = v1916[0];
            let v2053 = v1916[1];
            let v2054 = v1916[2];
            let v2055 = v1919[0];
            let v2056 = v1919[1];
            let v2057 = v1919[2];
            let v2058 = v1919[3];
            let v2059 = v1921[0];
            let v2060 = v1921[1];
            let v2061 = v1921[2];
            let v2062 = v1921[3];
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(9),
            None,
            multiplicity * (v432),
            [5, 6, 9],
            [v1923, v1924, v1925],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(9),
            None,
            multiplicity * (v434),
            [9],
            [v1926],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(9),
            None,
            multiplicity * (v437),
            [9],
            [v1927],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            None,
            multiplicity * (v846),
            [1, 2, 3, 4, 5, 6],
            [v1928, v1929, v1930, v1931, v1932, v1933],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(8),
            None,
            multiplicity * (v847),
            [8],
            [v1934],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(8),
            None,
            multiplicity * (v848),
            [1, 2, 8],
            [v1935, v1936, v1937],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(8), None, 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            v849,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(3),
            None,
            multiplicity * (v850),
            [0, 1, 2, 3, 4, 5, 6],
            [v1938, v1939, v1940, v1941, v1942, v1943, v1944],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (v851),
            [3],
            [v1945],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (v852),
            [3],
            [v1946],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(7), None, 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            v853,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(3),
            None,
            multiplicity * (v854),
            [0, 1, 2, 3, 4, 5, 6],
            [v1947, v1948, v1949, v1950, v1951, v1952, v1953],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(3),
            Some(7),
            multiplicity * (v856),
            [3, 7],
            [v1954, v1955],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (v858),
            [3],
            [v1956],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(7),
            None,
            multiplicity * (v860),
            [7],
            [v1957],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(7),
            None,
            multiplicity * (v862),
            [7],
            [v1958],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(3),
            None,
            multiplicity * (v864),
            [0, 1, 2, 3, 4, 5, 6],
            [v1959, v1960, v1961, v1962, v1963, v1964, v1965],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(7), None, 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            v867,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(3), None, 3, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            3,
            v870,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(7), None, 4, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            4,
            v873,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(5),
            Some(6),
            multiplicity * (v752),
            [5, 6],
            [v1966, v1967],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(5),
            Some(4),
            multiplicity * (v754),
            [4, 5],
            [v1968, v1969],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(4),
            Some(6),
            multiplicity * (v757),
            [4, 6],
            [v1970, v1971],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(5),
            multiplicity * (v876),
            [1, 3, 5, 8],
            [v1972, v1973, v1974, v1975],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(5),
            multiplicity * (v877),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(1), Some(5), 5, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            5,
            v878,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(2),
            Some(6),
            multiplicity * (v879),
            [2, 3, 6],
            [v1976, v1977, v1978],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(2),
            Some(6),
            multiplicity * (v880),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(2), Some(6), 6, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            6,
            v881,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(0),
            Some(4),
            multiplicity * (v882),
            [0, 3, 4],
            [v1979, v1980, v1981],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(4),
            multiplicity * (v883),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(0), Some(4), 7, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            7,
            v884,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (v803),
            [3, 4, 5, 6],
            [v1982, v1983, v1984, v1985],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(4),
            multiplicity * (v805),
            [3, 4, 5],
            [v1986, v1987, v1988],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            Some(6),
            multiplicity * (v808),
            [3, 4, 5, 6],
            [v1989, v1990, v1991, v1992],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(6),
            multiplicity * (v810),
            [3, 4, 5, 6, 9],
            [v1993, v1994, v1995, v1996, v1997],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(6),
            multiplicity * (v813),
            [3, 5, 6],
            [v1998, v1999, v2000],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * (v816),
            [1, 2, 3, 5, 6],
            [v2001, v2002, v2003, v2004, v2005],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(4),
            multiplicity * (v819),
            [1, 3, 4],
            [v2006, v2007, v2008],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(4),
            multiplicity * (v822),
            [3, 4, 5],
            [v2009, v2010, v2011],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (v825),
            [3, 4, 5, 6],
            [v2012, v2013, v2014, v2015],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(2),
            Some(4),
            multiplicity * (v829),
            [2, 3, 4],
            [v2016, v2017, v2018],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (v833),
            [3, 4, 5, 6],
            [v2019, v2020, v2021, v2022],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (v835),
            [3, 4, 5, 6],
            [v2023, v2024, v2025, v2026],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(6),
            multiplicity * (v842),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(6),
            multiplicity * (v844),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(6),
            multiplicity * (v845),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = v432;
        self.canonical_reactive[1] = v434;
        self.canonical_reactive[2] = v885;
        self.canonical_reactive[3] = v2027;
        self.canonical_reactive[4] = v846;
        self.canonical_reactive[5] = v847;
        self.canonical_reactive[6] = v887;
        self.canonical_reactive[7] = v2028;
        self.canonical_reactive[8] = v2029;
        self.canonical_reactive[9] = v2030;
        self.canonical_reactive[10] = v849;
        self.canonical_reactive[11] = v850;
        self.canonical_reactive[12] = v851;
        self.canonical_reactive[13] = v888;
        self.canonical_reactive[14] = v2031;
        self.canonical_reactive[15] = v853;
        self.canonical_reactive[16] = v854;
        self.canonical_reactive[17] = v856;
        self.canonical_reactive[18] = v890;
        self.canonical_reactive[19] = v2032;
        self.canonical_reactive[20] = v860;
        self.canonical_reactive[21] = v892;
        self.canonical_reactive[22] = v2033;
        self.canonical_reactive[23] = v864;
        self.canonical_reactive[24] = v867;
        self.canonical_reactive[25] = v870;
        self.canonical_reactive[26] = v873;
        self.canonical_reactive[27] = v752;
        self.canonical_reactive[28] = v754;
        self.canonical_reactive[29] = v757;
        self.canonical_reactive[30] = v876;
        self.canonical_reactive[31] = v877;
        self.canonical_reactive[32] = v878;
        self.canonical_reactive[33] = v879;
        self.canonical_reactive[34] = v880;
        self.canonical_reactive[35] = v881;
        self.canonical_reactive[36] = v882;
        self.canonical_reactive[37] = v883;
        self.canonical_reactive[38] = v884;
        self.canonical_reactive[39] = v803;
        self.canonical_reactive[40] = v805;
        self.canonical_reactive[41] = v808;
        self.canonical_reactive[42] = v810;
        self.canonical_reactive[43] = v812;
        self.canonical_reactive[44] = v2034;
        self.canonical_reactive[45] = v2035;
        self.canonical_reactive[46] = v2036;
        self.canonical_reactive[47] = v815;
        self.canonical_reactive[48] = v2037;
        self.canonical_reactive[49] = v2038;
        self.canonical_reactive[50] = v2039;
        self.canonical_reactive[51] = v2040;
        self.canonical_reactive[52] = v2041;
        self.canonical_reactive[53] = v818;
        self.canonical_reactive[54] = v2042;
        self.canonical_reactive[55] = v2043;
        self.canonical_reactive[56] = v2044;
        self.canonical_reactive[57] = v821;
        self.canonical_reactive[58] = v2045;
        self.canonical_reactive[59] = v2046;
        self.canonical_reactive[60] = v2047;
        self.canonical_reactive[61] = v824;
        self.canonical_reactive[62] = v2048;
        self.canonical_reactive[63] = v2049;
        self.canonical_reactive[64] = v2050;
        self.canonical_reactive[65] = v2051;
        self.canonical_reactive[66] = v828;
        self.canonical_reactive[67] = v2052;
        self.canonical_reactive[68] = v2053;
        self.canonical_reactive[69] = v2054;
        self.canonical_reactive[70] = v832;
        self.canonical_reactive[71] = v2055;
        self.canonical_reactive[72] = v2056;
        self.canonical_reactive[73] = v2057;
        self.canonical_reactive[74] = v2058;
        self.canonical_reactive[75] = v834;
        self.canonical_reactive[76] = v2059;
        self.canonical_reactive[77] = v2060;
        self.canonical_reactive[78] = v2061;
        self.canonical_reactive[79] = v2062;
        self.canonical_reactive[80] = v842;
        self.canonical_reactive[81] = v844;
        self.canonical_reactive[82] = v845;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(9),
            None,
            &[9],
            &[cached[3]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            None,
            &[1, 2, 8],
            &[cached[7], cached[8], cached[9]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            None,
            &[3],
            &[cached[14]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            None,
            &[3],
            &[cached[19]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            None,
            &[7],
            &[cached[22]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(6),
            &[3, 5, 6],
            &[cached[44], cached[45], cached[46]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(6),
            &[1, 2, 3, 5, 6],
            &[cached[48], cached[49], cached[50], cached[51], cached[52]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(4),
            &[1, 3, 4],
            &[cached[54], cached[55], cached[56]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(4),
            &[3, 4, 5],
            &[cached[58], cached[59], cached[60]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(4),
            &[3, 4, 5, 6],
            &[cached[62], cached[63], cached[64], cached[65]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(2),
            Some(4),
            &[2, 3, 4],
            &[cached[67], cached[68], cached[69]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(6),
            &[3, 4, 5, 6],
            &[cached[71], cached[72], cached[73], cached[74]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(4),
            &[3, 4, 5, 6],
            &[cached[76], cached[77], cached[78], cached[79]],
            &[],
            &[],
            multiplicity,
        );
    }

}

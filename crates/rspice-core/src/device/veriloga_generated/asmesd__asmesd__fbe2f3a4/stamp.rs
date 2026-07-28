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
            let v890 = 1e0f64;
            let v891 = Lanes([1e0f64; 1]);
            let v892 = Lanes([1e0f64; 1]);
            let v893 = Lanes([1e0f64; 1]);
            let v894 = Lanes([1e0f64; 1]);
            let v895 = Lanes([1e0f64; 1]);
            let v896 = Lanes([1e0f64; 1]);
            let v897 = Lanes([1e0f64; 1]);
            let v898 = Lanes([1e0f64; 1]);
            let v899 = Lanes([1e0f64; 1]);
            let v900 = Lanes([1e0f64; 1]);
            let v976 = Lanes([0e0f64; 1]);
            let v983 = -1e0f64;
            let v1166 = Lanes([0e0f64; 3]);
            let v1208 = 0e0f64;
            let v1210 = 2e0f64;
            let v1309 = Lanes([0e0f64; 3]);
            let v1427 = ddt_scale();
            let v1582 = Lanes([0e0f64; 6]);
            let v1583 = Lanes([0e0f64; 1]);
            let v1584 = Lanes([0e0f64; 3]);
            let v1706 = Lanes([0e0f64; 3]);
            let v1761 = Lanes([0e0f64; 4]);
            let v1763 = Lanes([0e0f64; 7]);
            let v1789 = Lanes([0e0f64; 2]);
            let v1790 = Lanes([0e0f64; 1]);
            let v1859 = Lanes([0e0f64; 4]);
            let v1865 = Lanes([0e0f64; 3]);
            let v1871 = Lanes([0e0f64; 3]);
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
            let v901: Lanes<1>;
            if v12 != 0.0 {
                v18 = v13;
                v901 = v976;
            } else {
                let v15 = if v5 > v14 { 1.0 } else { 0.0 };
                let v17: f64;
                let v902: Lanes<1>;
                if v15 != 0.0 {
                    v17 = v5;
                    v902 = v891;
                } else {
                    v17 = v16;
                    v902 = v976;
                }
                v18 = v17;
                v901 = v902;
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
            let v979 = (Lanes([0.0, v892[0]])) - (Lanes([v893[0], 0.0]));
            let v30 = v26 * v29;
            let v980 = v979 * v26;
            let v33 = -(if v30 <= v0 { v30 } else { v0 });
            let v37 = v19 + (v31 * (v33.powf(v34)));
            let v39 = v38 + v6;
            let v41 = v40 * v18;
            let v990 = v901 * v40;
            let v42 = v18 / v39;
            let v991 = v901 / v39;
            let v43 = v42.ln();
            let v993 = v991 * (v890 / v42);
            let v46 = (v44 * v43).exp();
            let v995 = (v993 * v44) * v46;
            let v48 = v47 * v46;
            let v49 = v48 * v37;
            let v997 = (v995 * v47) * v37;
            let v998 = ((((v980 * (if v30 <= v0 { 1.0 } else { 0.0 })) * v983) * (v34 * (v33.powf((v34 - v890))))) * v31) * v48;
            let v1001 = (Lanes([v997[0], 0.0, 0.0])) + (Lanes([0.0, v998[0], v998[1]]));
            let v51 = v50 * v46;
            let v1002 = v995 * v50;
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
            let v1008 = (v993 * v68) + (((v991 * v70) - (v990 * v73)) / v41);
            let v78 = v74.exp();
            let v79 = v77 * v78;
            let v1011 = (v1008 * v78) * v77;
            let v81 = (v75 * v43).exp();
            let v82 = v80 * v81;
            let v1013 = ((v993 * v75) * v81) * v80;
            let v86 = (v74 / v84).exp();
            let v88 = (v83 * v86) / v46;
            let v1019 = ((((v1008 / v84) * v86) * v83) - (v995 * v88)) / v46;
            let v92 = (v74 / v90).exp();
            let v94 = (v89 * v92) / v46;
            let v1025 = ((((v1008 / v90) * v92) * v89) - (v995 * v94)) / v46;
            let v99 = v95 * (v19 + (v96 * v71));
            let v1027 = (v991 * v96) * v95;
            let v104 = v100 * (v19 + (v101 * v71));
            let v1029 = (v991 * v101) * v100;
            let v109 = v105 * (v19 + (v106 * v71));
            let v1031 = (v991 * v106) * v105;
            let v114 = v110 * (v19 + (v111 * v71));
            let v1033 = (v991 * v111) * v110;
            let v119 = v39 / v118;
            let v120 = v18 / v118;
            let v1034 = v901 / v118;
            let v123 = v122 * v18;
            let v126 = v125 + v18;
            let v127 = (v123 * v18) / v126;
            let v1043 = ((((((v901 * v122) * v18) + (v901 * v123)) - (v901 * v127)) / v126) * v983) * v983;
            let v132 = v130 * (v18 + v18);
            let v133 = (-(v121 - v127)) / v132;
            let v1046 = ((v901 + v901) * v130) * v133;
            let v137 = -(v41 + v41);
            let v1050 = (v990 + v990) * v983;
            let v140 = v138 * (v120.ln());
            let v1053 = (v1034 * (v890 / v120)) * v138;
            let v143 = v140 + (v141 * (v133 + v134));
            let v144 = v137 * v143;
            let v1058 = (v1050 * v143) + ((v1053 + (((v1043 - v1046) / v132) * v141)) * v137);
            let v147 = (v145 - v144) / v119;
            let v1060 = (v1058 * v983) / v119;
            let v149 = (v145 - v147) / v147;
            let v153 = v151 * (v39 - v118);
            let v156 = v19 + (v150 * (v153 - v149));
            let v157 = v115 / v156;
            let v159 = (v120 * v147) + v144;
            let v1073 = ((v1034 * v147) + (v1060 * v120)) + v1058;
            let v161 = (v159 - v147) / v147;
            let v163 = v151 * (v18 - v118);
            let v1078 = v901 * v151;
            let v166 = v19 + (v150 * (v163 - v161));
            let v167 = v157 * v166;
            let v1083 = (((((((((v1060 * v983) - (v1060 * v149)) / v147) * v983) * v150) * v157) * v983) / v156) * v166) + (((v1078 - (((v1073 - v1060) - (v1060 * v161)) / v147)) * v150) * v157);
            let v171 = v140 + (v141 * (v133 + v168));
            let v172 = v137 * v171;
            let v1090 = (v1050 * v171) + ((v1053 + (((v1043 - v1046) / v132) * v141)) * v137);
            let v175 = (v173 - v172) / v119;
            let v1092 = (v1090 * v983) / v119;
            let v177 = (v173 - v175) / v175;
            let v181 = v19 + (v178 * (v153 - v177));
            let v182 = v116 / v181;
            let v184 = (v120 * v175) + v172;
            let v1105 = ((v1034 * v175) + (v1092 * v120)) + v1090;
            let v186 = (v184 - v175) / v175;
            let v189 = v19 + (v178 * (v163 - v186));
            let v190 = v182 * v189;
            let v1114 = (((((((((v1092 * v983) - (v1092 * v177)) / v175) * v983) * v178) * v182) * v983) / v181) * v189) + (((v1078 - (((v1105 - v1092) - (v1092 * v186)) / v175)) * v178) * v182);
            let v194 = v140 + (v141 * (v133 + v191));
            let v195 = v137 * v194;
            let v1121 = (v1050 * v194) + ((v1053 + (((v1043 - v1046) / v132) * v141)) * v137);
            let v198 = (v196 - v195) / v119;
            let v1123 = (v1121 * v983) / v119;
            let v200 = (v196 - v198) / v198;
            let v204 = v19 + (v201 * (v153 - v200));
            let v205 = v117 / v204;
            let v207 = (v120 * v198) + v195;
            let v1136 = ((v1034 * v198) + (v1123 * v120)) + v1121;
            let v209 = (v207 - v198) / v198;
            let v212 = v19 + (v201 * (v163 - v209));
            let v213 = v205 * v212;
            let v1145 = (((((((((v1123 * v983) - (v1123 * v200)) / v198) * v983) * v201) * v205) * v983) / v204) * v212) + (((v1078 - (((v1136 - v1123) - (v1123 * v209)) / v198)) * v201) * v205);
            let v216 = v26 * (v214 - v28);
            let v1149 = ((Lanes([v894[0], 0.0])) - (Lanes([0.0, v893[0]]))) * v26;
            let v218 = v27 - v217;
            let v1152 = (Lanes([v892[0], 0.0])) - (Lanes([0.0, v895[0]]));
            let v219 = v26 * v218;
            let v1153 = v1152 * v26;
            let v222 = v26 * (v220 - v28);
            let v1157 = ((Lanes([v896[0], 0.0])) - (Lanes([0.0, v893[0]]))) * v26;
            let v223 = v220 - v27;
            let v1160 = (Lanes([v896[0], 0.0])) - (Lanes([0.0, v892[0]]));
            let v224 = v26 * v223;
            let v1161 = v1160 * v26;
            let v225 = v214 - v217;
            let v1164 = (Lanes([v894[0], 0.0])) - (Lanes([0.0, v895[0]]));
            let v226 = v26 * v225;
            let v1165 = v1164 * v26;
            let v227 = if v79 > v0 { 1.0 } else { 0.0 };
            let v444: f64;
            let v903: Lanes<3>;
            if v227 != 0.0 {
                let v229 = v228 * v41;
                let v230 = v219 / v229;
                let v1168 = (v990 * v228) * v230;
                let v1172 = ((Lanes([0.0, v1153[0], v1153[1]])) - (Lanes([v1168[0], 0.0, 0.0]))) / v229;
                let v1173 = v1153 * v983;
                let v234 = v233 * v41;
                let v1177 = v990 * v233;
                let v235 = ((-v219) - v104) / v234;
                let v1178 = v1177 * v235;
                let v1181 = (((Lanes([0.0, v1173[0], v1173[1]])) - (Lanes([v1029[0], 0.0, 0.0]))) - (Lanes([v1178[0], 0.0, 0.0]))) / v234;
                let v237 = (-v104) / v234;
                let v1185 = ((v1029 * v983) - (v1177 * v237)) / v234;
                let v239 = if v230 > v238 { 1.0 } else { 0.0 };
                let v242: f64;
                let v243: f64;
                let v904: Lanes<3>;
                let v905: Lanes<3>;
                if v239 != 0.0 {
                    let v241 = v19 + (v230 - v238);
                    v242 = v241;
                    v243 = v238;
                    v904 = v1172;
                    v905 = v1166;
                } else {
                    v242 = v19;
                    v243 = v230;
                    v904 = v1166;
                    v905 = v1172;
                }
                let v244 = v243.exp();
                let v245 = v242 * v244;
                let v1189 = (v904 * v244) + ((v905 * v244) * v242);
                let v247 = if v235 >= v246 { 1.0 } else { 0.0 };
                let v261: f64;
                let v906: Lanes<3>;
                if v247 != 0.0 {
                    v261 = v235;
                    v906 = v1181;
                } else {
                    let v249 = if v235 <= v248 { 1.0 } else { 0.0 };
                    let v262: f64;
                    let v907: Lanes<3>;
                    if v249 != 0.0 {
                        let v250 = v235.exp();
                        let v1193 = v1181 * v250;
                        v262 = v250;
                        v907 = v1193;
                    } else {
                        let v251 = v235.exp();
                        let v252 = v251 + v19;
                        let v253 = v252.ln();
                        let v1192 = (v1181 * v251) * (v890 / v252);
                        v262 = v253;
                        v907 = v1192;
                    }
                    v261 = v262;
                    v906 = v907;
                }
                let v254 = if v237 >= v246 { 1.0 } else { 0.0 };
                let v263: f64;
                let v908: Lanes<1>;
                if v254 != 0.0 {
                    v263 = v237;
                    v908 = v1185;
                } else {
                    let v256 = if v237 <= v255 { 1.0 } else { 0.0 };
                    let v264: f64;
                    let v909: Lanes<1>;
                    if v256 != 0.0 {
                        let v257 = v237.exp();
                        let v1197 = v1185 * v257;
                        v264 = v257;
                        v909 = v1197;
                    } else {
                        let v258 = v237.exp();
                        let v259 = v258 + v19;
                        let v260 = v259.ln();
                        let v1196 = (v1185 * v258) * (v890 / v259);
                        v264 = v260;
                        v909 = v1196;
                    }
                    v263 = v264;
                    v908 = v909;
                }
                let v265 = v261 - v263;
                let v266 = v245 - v19;
                let v1200 = v1011 * v266;
                let v1204 = v1027 * v265;
                let v270 = v219.abs();
                let v271 = v270.powf(v109);
                let v1217 = (v1153 * ((v1210 * (if v219 >= v1208 { 1.0 } else { 0.0 })) - v890)) * (v109 * (v270.powf((v109 - v890))));
                let v1220 = v1031 * (v271 * (v270.ln()));
                let v273 = v19 + (v269 * v271);
                let v274 = (v99 * v265) / v273;
                let v275 = (v79 * v266) - v274;
                let v1228 = ((Lanes([v1200[0], 0.0, 0.0])) + (v1189 * v79)) - ((((Lanes([v1204[0], 0.0, 0.0])) + ((v906 - (Lanes([v908[0], 0.0, 0.0]))) * v99)) - ((((Lanes([0.0, v1217[0], v1217[1]])) + (Lanes([v1220[0], 0.0, 0.0]))) * v269) * v274)) / v273);
                v444 = v275;
                v903 = v1228;
            } else {
                v444 = v0;
                v903 = v1166;
            }
            let v276 = if v82 > v0 { 1.0 } else { 0.0 };
            let v445: f64;
            let v910: Lanes<3>;
            if v276 != 0.0 {
                let v278 = v277 - v219;
                let v280 = if v278 >= v279 { v278 } else { v279 };
                let v1233 = (v1153 * v281) * v277;
                let v285 = v284 * v41;
                let v286 = v285 * v280;
                let v1235 = (v990 * v284) * v280;
                let v1236 = ((v1153 * v983) * (if v278 >= v279 { 1.0 } else { 0.0 })) * v285;
                let v287 = ((v281 * v219) * v277) / v286;
                let v1243 = ((Lanes([0.0, v1233[0], v1233[1]])) - (((Lanes([v1235[0], 0.0, 0.0])) + (Lanes([0.0, v1236[0], v1236[1]]))) * v287)) / v286;
                let v288 = if v287 > v238 { 1.0 } else { 0.0 };
                let v291: f64;
                let v292: f64;
                let v911: Lanes<3>;
                let v912: Lanes<3>;
                if v288 != 0.0 {
                    let v290 = v19 + (v287 - v238);
                    v291 = v290;
                    v292 = v238;
                    v911 = v1243;
                    v912 = v1166;
                } else {
                    v291 = v19;
                    v292 = v287;
                    v911 = v1166;
                    v912 = v1243;
                }
                let v293 = v292.exp();
                let v295 = (v291 * v293) - v19;
                let v296 = v82 * v295;
                let v1248 = v1013 * v295;
                let v1251 = (Lanes([v1248[0], 0.0, 0.0])) + (((v911 * v293) + ((v912 * v293) * v291)) * v82);
                v445 = v296;
                v910 = v1251;
            } else {
                v445 = v0;
                v910 = v1166;
            }
            let v297 = if v88 > v0 { 1.0 } else { 0.0 };
            let v448: f64;
            let v913: Lanes<3>;
            if v297 != 0.0 {
                let v298 = v84 * v41;
                let v299 = v219 / v298;
                let v1253 = (v990 * v84) * v299;
                let v1257 = ((Lanes([0.0, v1153[0], v1153[1]])) - (Lanes([v1253[0], 0.0, 0.0]))) / v298;
                let v1258 = v1153 * v983;
                let v303 = v302 * v41;
                let v1262 = v990 * v302;
                let v304 = ((-v219) - v104) / v303;
                let v1263 = v1262 * v304;
                let v1266 = (((Lanes([0.0, v1258[0], v1258[1]])) - (Lanes([v1029[0], 0.0, 0.0]))) - (Lanes([v1263[0], 0.0, 0.0]))) / v303;
                let v306 = (-v104) / v303;
                let v1270 = ((v1029 * v983) - (v1262 * v306)) / v303;
                let v307 = if v299 > v238 { 1.0 } else { 0.0 };
                let v310: f64;
                let v311: f64;
                let v914: Lanes<3>;
                let v915: Lanes<3>;
                if v307 != 0.0 {
                    let v309 = v19 + (v299 - v238);
                    v310 = v309;
                    v311 = v238;
                    v914 = v1257;
                    v915 = v1166;
                } else {
                    v310 = v19;
                    v311 = v299;
                    v914 = v1166;
                    v915 = v1257;
                }
                let v312 = v311.exp();
                let v313 = v310 * v312;
                let v1274 = (v914 * v312) + ((v915 * v312) * v310);
                let v314 = if v304 >= v246 { 1.0 } else { 0.0 };
                let v328: f64;
                let v916: Lanes<3>;
                if v314 != 0.0 {
                    v328 = v304;
                    v916 = v1266;
                } else {
                    let v316 = if v304 <= v315 { 1.0 } else { 0.0 };
                    let v329: f64;
                    let v917: Lanes<3>;
                    if v316 != 0.0 {
                        let v317 = v304.exp();
                        let v1278 = v1266 * v317;
                        v329 = v317;
                        v917 = v1278;
                    } else {
                        let v318 = v304.exp();
                        let v319 = v318 + v19;
                        let v320 = v319.ln();
                        let v1277 = (v1266 * v318) * (v890 / v319);
                        v329 = v320;
                        v917 = v1277;
                    }
                    v328 = v329;
                    v916 = v917;
                }
                let v321 = if v306 >= v246 { 1.0 } else { 0.0 };
                let v330: f64;
                let v918: Lanes<1>;
                if v321 != 0.0 {
                    v330 = v306;
                    v918 = v1270;
                } else {
                    let v323 = if v306 <= v322 { 1.0 } else { 0.0 };
                    let v331: f64;
                    let v919: Lanes<1>;
                    if v323 != 0.0 {
                        let v324 = v306.exp();
                        let v1282 = v1270 * v324;
                        v331 = v324;
                        v919 = v1282;
                    } else {
                        let v325 = v306.exp();
                        let v326 = v325 + v19;
                        let v327 = v326.ln();
                        let v1281 = (v1270 * v325) * (v890 / v326);
                        v331 = v327;
                        v919 = v1281;
                    }
                    v330 = v331;
                    v918 = v919;
                }
                let v333 = v313 - v19;
                let v1285 = v1019 * v333;
                let v336 = v219.abs();
                let v337 = v336.powf(v109);
                let v1297 = (v1153 * ((v1210 * (if v219 >= v1208 { 1.0 } else { 0.0 })) - v890)) * (v109 * (v336.powf((v109 - v890))));
                let v1300 = v1031 * (v337 * (v336.ln()));
                let v339 = v19 + (v269 * v337);
                let v340 = (v0 * (v328 - v330)) / v339;
                let v341 = (v88 * v333) - v340;
                let v1308 = ((Lanes([v1285[0], 0.0, 0.0])) + (v1274 * v88)) - ((((v916 - (Lanes([v918[0], 0.0, 0.0]))) * v0) - ((((Lanes([0.0, v1297[0], v1297[1]])) + (Lanes([v1300[0], 0.0, 0.0]))) * v269) * v340)) / v339);
                v448 = v341;
                v913 = v1308;
            } else {
                v448 = v0;
                v913 = v1166;
            }
            let v450: f64;
            let v920: Lanes<3>;
            if v227 != 0.0 {
                let v343 = v342 * v41;
                let v344 = v30 / v343;
                let v1311 = (v990 * v342) * v344;
                let v1315 = ((Lanes([0.0, v980[0], v980[1]])) - (Lanes([v1311[0], 0.0, 0.0]))) / v343;
                let v1316 = v980 * v983;
                let v347 = v302 * v41;
                let v1320 = v990 * v302;
                let v348 = ((-v30) - v104) / v347;
                let v1321 = v1320 * v348;
                let v1324 = (((Lanes([0.0, v1316[0], v1316[1]])) - (Lanes([v1029[0], 0.0, 0.0]))) - (Lanes([v1321[0], 0.0, 0.0]))) / v347;
                let v350 = (-v104) / v347;
                let v1328 = ((v1029 * v983) - (v1320 * v350)) / v347;
                let v351 = if v344 > v238 { 1.0 } else { 0.0 };
                let v354: f64;
                let v355: f64;
                let v921: Lanes<3>;
                let v922: Lanes<3>;
                if v351 != 0.0 {
                    let v353 = v19 + (v344 - v238);
                    v354 = v353;
                    v355 = v238;
                    v921 = v1315;
                    v922 = v1309;
                } else {
                    v354 = v19;
                    v355 = v344;
                    v921 = v1309;
                    v922 = v1315;
                }
                let v356 = v355.exp();
                let v357 = v354 * v356;
                let v1332 = (v921 * v356) + ((v922 * v356) * v354);
                let v358 = if v348 >= v246 { 1.0 } else { 0.0 };
                let v372: f64;
                let v923: Lanes<3>;
                if v358 != 0.0 {
                    v372 = v348;
                    v923 = v1324;
                } else {
                    let v360 = if v348 <= v359 { 1.0 } else { 0.0 };
                    let v373: f64;
                    let v924: Lanes<3>;
                    if v360 != 0.0 {
                        let v361 = v348.exp();
                        let v1336 = v1324 * v361;
                        v373 = v361;
                        v924 = v1336;
                    } else {
                        let v362 = v348.exp();
                        let v363 = v362 + v19;
                        let v364 = v363.ln();
                        let v1335 = (v1324 * v362) * (v890 / v363);
                        v373 = v364;
                        v924 = v1335;
                    }
                    v372 = v373;
                    v923 = v924;
                }
                let v365 = if v350 >= v246 { 1.0 } else { 0.0 };
                let v374: f64;
                let v925: Lanes<1>;
                if v365 != 0.0 {
                    v374 = v350;
                    v925 = v1328;
                } else {
                    let v367 = if v350 <= v366 { 1.0 } else { 0.0 };
                    let v375: f64;
                    let v926: Lanes<1>;
                    if v367 != 0.0 {
                        let v368 = v350.exp();
                        let v1340 = v1328 * v368;
                        v375 = v368;
                        v926 = v1340;
                    } else {
                        let v369 = v350.exp();
                        let v370 = v369 + v19;
                        let v371 = v370.ln();
                        let v1339 = (v1328 * v369) * (v890 / v370);
                        v375 = v371;
                        v926 = v1339;
                    }
                    v374 = v375;
                    v925 = v926;
                }
                let v376 = v372 - v374;
                let v377 = v357 - v19;
                let v1343 = v1011 * v377;
                let v1347 = v1033 * v376;
                let v380 = v30.abs();
                let v381 = v380.powf(v109);
                let v1358 = (v980 * ((v1210 * (if v30 >= v1208 { 1.0 } else { 0.0 })) - v890)) * (v109 * (v380.powf((v109 - v890))));
                let v1361 = v1031 * (v381 * (v380.ln()));
                let v383 = v19 + (v269 * v381);
                let v384 = (v114 * v376) / v383;
                let v385 = (v79 * v377) - v384;
                let v1369 = ((Lanes([v1343[0], 0.0, 0.0])) + (v1332 * v79)) - ((((Lanes([v1347[0], 0.0, 0.0])) + ((v923 - (Lanes([v925[0], 0.0, 0.0]))) * v114)) - ((((Lanes([0.0, v1358[0], v1358[1]])) + (Lanes([v1361[0], 0.0, 0.0]))) * v269) * v384)) / v383);
                v450 = v385;
                v920 = v1369;
            } else {
                v450 = v0;
                v920 = v1309;
            }
            let v386 = if v94 > v0 { 1.0 } else { 0.0 };
            let v452: f64;
            let v927: Lanes<3>;
            if v386 != 0.0 {
                let v387 = v90 * v41;
                let v388 = v30 / v387;
                let v1371 = (v990 * v90) * v388;
                let v1375 = ((Lanes([0.0, v980[0], v980[1]])) - (Lanes([v1371[0], 0.0, 0.0]))) / v387;
                let v1376 = v980 * v983;
                let v391 = v302 * v41;
                let v1380 = v990 * v302;
                let v392 = ((-v30) - v104) / v391;
                let v1381 = v1380 * v392;
                let v1384 = (((Lanes([0.0, v1376[0], v1376[1]])) - (Lanes([v1029[0], 0.0, 0.0]))) - (Lanes([v1381[0], 0.0, 0.0]))) / v391;
                let v394 = (-v104) / v391;
                let v1388 = ((v1029 * v983) - (v1380 * v394)) / v391;
                let v395 = if v388 > v238 { 1.0 } else { 0.0 };
                let v398: f64;
                let v399: f64;
                let v928: Lanes<3>;
                let v929: Lanes<3>;
                if v395 != 0.0 {
                    let v397 = v19 + (v388 - v238);
                    v398 = v397;
                    v399 = v238;
                    v928 = v1375;
                    v929 = v1309;
                } else {
                    v398 = v19;
                    v399 = v388;
                    v928 = v1309;
                    v929 = v1375;
                }
                let v400 = v399.exp();
                let v401 = v398 * v400;
                let v1392 = (v928 * v400) + ((v929 * v400) * v398);
                let v402 = if v392 >= v246 { 1.0 } else { 0.0 };
                let v416: f64;
                let v930: Lanes<3>;
                if v402 != 0.0 {
                    v416 = v392;
                    v930 = v1384;
                } else {
                    let v404 = if v392 <= v403 { 1.0 } else { 0.0 };
                    let v417: f64;
                    let v931: Lanes<3>;
                    if v404 != 0.0 {
                        let v405 = v392.exp();
                        let v1396 = v1384 * v405;
                        v417 = v405;
                        v931 = v1396;
                    } else {
                        let v406 = v392.exp();
                        let v407 = v406 + v19;
                        let v408 = v407.ln();
                        let v1395 = (v1384 * v406) * (v890 / v407);
                        v417 = v408;
                        v931 = v1395;
                    }
                    v416 = v417;
                    v930 = v931;
                }
                let v409 = if v394 >= v246 { 1.0 } else { 0.0 };
                let v418: f64;
                let v932: Lanes<1>;
                if v409 != 0.0 {
                    v418 = v394;
                    v932 = v1388;
                } else {
                    let v411 = if v394 <= v410 { 1.0 } else { 0.0 };
                    let v419: f64;
                    let v933: Lanes<1>;
                    if v411 != 0.0 {
                        let v412 = v394.exp();
                        let v1400 = v1388 * v412;
                        v419 = v412;
                        v933 = v1400;
                    } else {
                        let v413 = v394.exp();
                        let v414 = v413 + v19;
                        let v415 = v414.ln();
                        let v1399 = (v1388 * v413) * (v890 / v414);
                        v419 = v415;
                        v933 = v1399;
                    }
                    v418 = v419;
                    v932 = v933;
                }
                let v421 = v401 - v19;
                let v1403 = v1025 * v421;
                let v424 = v30.abs();
                let v427 = v19 + (v269 * (v424.powf(v105)));
                let v428 = (v0 * (v416 - v418)) / v427;
                let v1417 = (((v980 * ((v1210 * (if v30 >= v1208 { 1.0 } else { 0.0 })) - v890)) * (v105 * (v424.powf((v105 - v890))))) * v269) * v428;
                let v429 = (v94 * v421) - v428;
                let v1421 = ((Lanes([v1403[0], 0.0, 0.0])) + (v1392 * v94)) - ((((v930 - (Lanes([v932[0], 0.0, 0.0]))) * v0) - (Lanes([0.0, v1417[0], v1417[1]]))) / v427);
                v452 = v429;
                v927 = v1421;
            } else {
                v452 = v0;
                v927 = v1309;
            }
            let v1422 = Lanes([v1153[0], v1153[1], 0.0]);
            let v1423 = Lanes([0.0, 0.0, v897[0]]);
            let v432 = -(v219 - v430);
            let v1425 = (v1422 - v1423) * v983;
            let v434 = v430 * v433;
            let v1426 = v897 * v433;
            let v437 = v435 * (ddt(3355, v430));
            let v1429 = (v897 * v1427) * v435;
            let v439 = v219.abs();
            let v441 = if v439 >= v440 { v439 } else { v440 };
            let v442 = (if v430 <= v219 { v430 } else { v219 }) / v441;
            let v1440 = ((v1153 * ((v1210 * (if v219 >= v1208 { 1.0 } else { 0.0 })) - v890)) * (if v439 >= v440 { 1.0 } else { 0.0 })) * v442;
            let v443 = v442.abs();
            let v1448 = v903 - v910;
            let v447 = (v444 - v445) / v49;
            let v1449 = v1001 * v447;
            let v449 = v447 + v448;
            let v1455 = (((Lanes([v1448[0], 0.0, v1448[1], v1448[2]])) - (Lanes([v1449[0], v1449[1], v1449[2], 0.0]))) / v49) + (Lanes([v913[0], 0.0, v913[1], v913[2]]));
            let v451 = v450 / v51;
            let v1456 = v1002 * v451;
            let v453 = v451 + v452;
            let v1460 = ((v920 - (Lanes([v1456[0], 0.0, 0.0]))) / v51) + v927;
            let v457 = v63 * (v19 + (v30 * v454));
            let v1463 = v903 * v457;
            let v1464 = ((v980 * v454) * v63) * v444;
            let v1468 = v920 * v67;
            let v1472 = (v1153 * v59) * v983;
            let v1473 = v980 * v55;
            let v467 = v19 + (v465 * ((v444 * v457) + (v450 * v67)));
            let v468 = v467.abs();
            let v471 = v19 + (v468.powf(v469));
            let v1486 = ((Lanes([0.0, v1472[0], v1472[1]])) - (Lanes([v1473[0], v1473[1], 0.0]))) * v472;
            let v474 = (v472 * ((v19 - (v219 * v59)) - (v30 * v55))) / v471;
            let v1490 = ((Lanes([0.0, v1486[0], v1486[1], v1486[2]])) - (((((((Lanes([v1463[0], 0.0, v1463[1], v1463[2]])) + (Lanes([0.0, v1464[0], v1464[1], 0.0]))) + (Lanes([v1468[0], v1468[1], v1468[2], 0.0]))) * v465) * ((v1210 * (if v467 >= v1208 { 1.0 } else { 0.0 })) - v890)) * (v469 * (v468.powf((v469 - v890))))) * v474)) / v471;
            let v475 = v450 * v474;
            let v1491 = v920 * v474;
            let v1494 = (Lanes([v1491[0], v1491[1], v1491[2], 0.0])) + (v1490 * v450);
            let v476 = v444 * v474;
            let v1495 = v903 * v474;
            let v1498 = (Lanes([v1495[0], 0.0, v1495[1], v1495[2]])) + (v1490 * v444);
            let v1499 = v1498 * v443;
            let v1500 = ((((v1422 + ((v1423 - v1422) * (if v430 <= v219 { 1.0 } else { 0.0 }))) - (Lanes([v1440[0], v1440[1], 0.0]))) / v441) * ((v1210 * (if v442 >= v1208 { 1.0 } else { 0.0 })) - v890)) * v476;
            let v480 = v19 - v478;
            let v481 = v480 * v444;
            let v1506 = (v903 * v480) * v474;
            let v1509 = (Lanes([v1506[0], 0.0, v1506[1], v1506[2]])) + (v1490 * v481);
            let v483 = ((v476 * v443) * v478) + (v481 * v474);
            let v1511 = (((Lanes([v1499[0], v1499[1], v1499[2], v1499[3], 0.0])) + (Lanes([0.0, 0.0, v1500[0], v1500[1], v1500[2]]))) * v478) + (Lanes([v1509[0], v1509[1], v1509[2], v1509[3], 0.0]));
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
            let v1537 = (((v993 * v497) * v499) * v496) * v502;
            let v1538 = ((((v1161 / v484) * ((v1210 * (if v485 >= v1208 { 1.0 } else { 0.0 })) - v890)) * (v487 * (v486.powf((v487 - v890))))) * (v501 * (v489.powf((v501 - v890))))) * v500;
            let v1541 = (Lanes([0.0, v1537[0], 0.0])) + (Lanes([v1538[0], 0.0, v1538[1]]));
            let v507 = (v43 * v505).exp();
            let v508 = v504 * v507;
            let v1544 = ((v993 * v505) * v507) * v504;
            let v512 = (v43 * v510).exp();
            let v513 = v509 * v512;
            let v514 = v19 / v493;
            let v515 = v495.powf(v514);
            let v516 = v513 * v515;
            let v1552 = (((v993 * v510) * v512) * v509) * v515;
            let v1553 = ((((v1165 / v490) * ((v1210 * (if v491 >= v1208 { 1.0 } else { 0.0 })) - v890)) * (v493 * (v492.powf((v493 - v890))))) * (v514 * (v495.powf((v514 - v890))))) * v513;
            let v1556 = (Lanes([0.0, v1552[0], 0.0])) + (Lanes([v1553[0], 0.0, v1553[1]]));
            let v517 = v220 - v214;
            let v1559 = (Lanes([v896[0], 0.0])) - (Lanes([0.0, v894[0]]));
            let v519 = v517 / v518;
            let v520 = v519.abs();
            let v523 = v19 + (v520.powf(v521));
            let v524 = v19 / v521;
            let v531 = v527 * (v19 + (v528 * ((v523.powf(v524)) - v19)));
            let v1574 = (((((v1559 / v518) * ((v1210 * (if v519 >= v1208 { 1.0 } else { 0.0 })) - v890)) * (v521 * (v520.powf((v521 - v890))))) * (v524 * (v523.powf((v524 - v890))))) * v528) * v527;
            let v532 = v531 * v444;
            let v1575 = v1574 * v444;
            let v1576 = v903 * v531;
            let v1579 = (Lanes([v1575[0], v1575[1], 0.0, 0.0, 0.0])) + (Lanes([0.0, 0.0, v1576[0], v1576[1], v1576[2]]));
            let v534 = v533 * v475;
            let v1580 = v1494 * v533;
            let v536 = if v535 == v19 { 1.0 } else { 0.0 };
            let v553: f64;
            let v846: f64;
            let v847: f64;
            let v848: f64;
            let v849: f64;
            let v934: Lanes<4>;
            let v935: Lanes<6>;
            let v936: Lanes<1>;
            let v937: Lanes<3>;
            if v536 != 0.0 {
                let v537 = v444 / v49;
                let v1585 = v1001 * v537;
                let v538 = -v537;
                let v539 = v538 * v531;
                let v1591 = ((((Lanes([v903[0], 0.0, v903[1], v903[2]])) - (Lanes([v1585[0], v1585[1], v1585[2], 0.0]))) / v49) * v983) * v531;
                let v1592 = v1574 * v538;
                let v1595 = (Lanes([0.0, 0.0, v1591[0], v1591[1], v1591[2], v1591[3]])) + (Lanes([v1592[0], v1592[1], 0.0, 0.0, 0.0, 0.0]));
                let v541 = ddt(3525, v540);
                let v542 = v531 * v541;
                let v1597 = v1574 * v541;
                let v1598 = (v898 * v1427) * v531;
                let v1601 = (Lanes([v1597[0], v1597[1], 0.0])) + (Lanes([0.0, 0.0, v1598[0]]));
                let v545 = (v540.abs()) / v544;
                let v548 = v19 + (v545.powf(v546));
                let v549 = v503 / v548;
                let v1611 = (((v898 * ((v1210 * (if v540 >= v1208 { 1.0 } else { 0.0 })) - v890)) / v544) * (v546 * (v545.powf((v546 - v890))))) * v549;
                let v1615 = ((Lanes([v1541[0], v1541[1], v1541[2], 0.0])) - (Lanes([0.0, 0.0, 0.0, v1611[0]]))) / v548;
                v553 = v549;
                v846 = v539;
                v847 = v540;
                v848 = v542;
                v849 = v0;
                v934 = v1615;
                v935 = v1595;
                v936 = v898;
                v937 = v1601;
            } else {
                let v1581 = Lanes([v1541[0], v1541[1], v1541[2], 0.0]);
                v553 = v503;
                v846 = v0;
                v847 = v0;
                v848 = v0;
                v849 = v550;
                v934 = v1581;
                v935 = v1582;
                v936 = v1583;
                v937 = v1584;
            }
            let v552 = if v551 == v19 { 1.0 } else { 0.0 };
            let v771: f64;
            let v782: f64;
            let v794: f64;
            let v938: Lanes<4>;
            let v939: Lanes<3>;
            let v940: Lanes<1>;
            if v552 != 0.0 {
                let v555 = v553 + v554;
                let v557 = v508 + v556;
                let v559 = v516 + v558;
                v771 = v555;
                v782 = v559;
                v794 = v557;
                v938 = v934;
                v939 = v1556;
                v940 = v1544;
            } else {
                v771 = v553;
                v782 = v516;
                v794 = v508;
                v938 = v934;
                v939 = v1556;
                v940 = v1544;
            }
            let v560 = if v216 <= v0 { 1.0 } else { 0.0 };
            let v826: f64;
            let v941: Lanes<3>;
            if v560 != 0.0 {
                let v561 = v213 * v207;
                let v562 = v19 - v201;
                let v563 = v216 / v207;
                let v1633 = v1136 * v563;
                let v564 = v19 - v563;
                let v567 = (v562 * (v564.ln())).exp();
                let v568 = v19 - v567;
                let v1644 = ((v1145 * v207) + (v1136 * v213)) * v568;
                let v570 = (v561 * v568) / v562;
                let v1648 = ((Lanes([0.0, v1644[0], 0.0])) + (((((((((Lanes([v1149[0], 0.0, v1149[1]])) - (Lanes([0.0, v1633[0], 0.0]))) / v207) * v983) * (v890 / v564)) * v562) * v567) * v983) * v561)) / v562;
                v826 = v570;
                v941 = v1648;
            } else {
                let v571 = v213 * v216;
                let v1616 = v1145 * v216;
                let v1617 = v1149 * v213;
                let v573 = v572 * v201;
                let v1621 = v1149 * v573;
                let v575 = (v573 * v216) / v207;
                let v1622 = v1136 * v575;
                let v576 = v19 + v575;
                let v577 = v571 * v576;
                let v1629 = (((Lanes([0.0, v1616[0], 0.0])) + (Lanes([v1617[0], 0.0, v1617[1]]))) * v576) + ((((Lanes([v1621[0], 0.0, v1621[1]])) - (Lanes([0.0, v1622[0], 0.0]))) / v207) * v571);
                v826 = v577;
                v941 = v1629;
            }
            let v1650 = (v1073 * v983) * v579;
            let v581 = v219 + ((-v159) * v579);
            let v1651 = Lanes([0.0, v1153[0], v1153[1]]);
            let v1653 = v1651 + (Lanes([v1650[0], 0.0, 0.0]));
            let v582 = if v581 > v0 { 1.0 } else { 0.0 };
            let v610: f64;
            let v611: f64;
            let v942: Lanes<3>;
            let v943: Lanes<3>;
            if v582 != 0.0 {
                let v585 = v19 - v579;
                let v588 = ((v583 - v150) * (v585.ln())).exp();
                let v591 = v19 - ((v588 * v585) * v585);
                let v593 = v19 - v150;
                let v594 = (v159 * v591) / v593;
                let v1670 = (v1073 * v591) / v593;
                let v595 = v572 * v150;
                let v597 = (v595 * v581) / v159;
                let v1672 = v1073 * v597;
                let v598 = v585 + v597;
                let v600 = (v581 * v598) * v588;
                let v1679 = ((v1653 * v598) + ((((v1653 * v595) - (Lanes([v1672[0], 0.0, 0.0]))) / v159) * v581)) * v588;
                let v1680 = Lanes([v1670[0], 0.0, 0.0]);
                v610 = v594;
                v611 = v600;
                v942 = v1680;
                v943 = v1679;
            } else {
                let v601 = v19 - v150;
                let v602 = v219 / v159;
                let v1654 = v1073 * v602;
                let v603 = v19 - v602;
                let v606 = (v601 * (v603.ln())).exp();
                let v607 = v19 - v606;
                let v1664 = v1073 * v607;
                let v609 = (v159 * v607) / v601;
                let v1668 = ((Lanes([v1664[0], 0.0, 0.0])) + ((((((((v1651 - (Lanes([v1654[0], 0.0, 0.0]))) / v159) * v983) * (v890 / v603)) * v601) * v606) * v983) * v159)) / v601;
                v610 = v609;
                v611 = v0;
                v942 = v1668;
                v943 = v1166;
            }
            let v612 = v610 + v611;
            let v613 = v167 * v612;
            let v1682 = v1083 * v612;
            let v1685 = (Lanes([v1682[0], 0.0, 0.0])) + ((v942 + v943) * v167);
            let v615 = (-v184) * v579;
            let v1687 = (v1105 * v983) * v579;
            let v616 = v222 + v615;
            let v1688 = Lanes([v1157[0], 0.0, v1157[1]]);
            let v1690 = v1688 + (Lanes([0.0, v1687[0], 0.0]));
            let v617 = if v616 > v0 { 1.0 } else { 0.0 };
            let v645: f64;
            let v646: f64;
            let v944: Lanes<3>;
            let v945: Lanes<3>;
            if v617 != 0.0 {
                let v620 = v19 - v579;
                let v623 = ((v618 - v178) * (v620.ln())).exp();
                let v626 = v19 - ((v623 * v620) * v620);
                let v628 = v19 - v178;
                let v629 = (v184 * v626) / v628;
                let v1708 = (v1105 * v626) / v628;
                let v630 = v572 * v178;
                let v632 = (v630 * v616) / v184;
                let v1710 = v1105 * v632;
                let v633 = v620 + v632;
                let v635 = (v616 * v633) * v623;
                let v1717 = ((v1690 * v633) + ((((v1690 * v630) - (Lanes([0.0, v1710[0], 0.0]))) / v184) * v616)) * v623;
                let v1718 = Lanes([0.0, v1708[0], 0.0]);
                v645 = v629;
                v646 = v635;
                v944 = v1718;
                v945 = v1717;
            } else {
                let v636 = v19 - v178;
                let v637 = v222 / v184;
                let v1691 = v1105 * v637;
                let v638 = v19 - v637;
                let v641 = (v636 * (v638.ln())).exp();
                let v642 = v19 - v641;
                let v1701 = v1105 * v642;
                let v644 = (v184 * v642) / v636;
                let v1705 = ((Lanes([0.0, v1701[0], 0.0])) + ((((((((v1688 - (Lanes([0.0, v1691[0], 0.0]))) / v184) * v983) * (v890 / v638)) * v636) * v641) * v983) * v184)) / v636;
                v645 = v644;
                v646 = v0;
                v944 = v1705;
                v945 = v1706;
            }
            let v647 = v645 + v646;
            let v1720 = v1114 * v647;
            let v650 = v19 - v649;
            let v651 = v650 * (v190 * v647);
            let v1724 = ((Lanes([0.0, v1720[0], 0.0])) + ((v944 + v945) * v190)) * v650;
            let v652 = v30 + v615;
            let v1725 = Lanes([0.0, v980[0], v980[1]]);
            let v1727 = v1725 + (Lanes([v1687[0], 0.0, 0.0]));
            let v653 = if v652 > v0 { 1.0 } else { 0.0 };
            let v681: f64;
            let v682: f64;
            let v946: Lanes<3>;
            let v947: Lanes<3>;
            if v653 != 0.0 {
                let v656 = v19 - v579;
                let v659 = ((v654 - v178) * (v656.ln())).exp();
                let v662 = v19 - ((v659 * v656) * v656);
                let v664 = v19 - v178;
                let v665 = (v184 * v662) / v664;
                let v1744 = (v1105 * v662) / v664;
                let v666 = v572 * v178;
                let v668 = (v666 * v652) / v184;
                let v1746 = v1105 * v668;
                let v669 = v656 + v668;
                let v671 = (v652 * v669) * v659;
                let v1753 = ((v1727 * v669) + ((((v1727 * v666) - (Lanes([v1746[0], 0.0, 0.0]))) / v184) * v652)) * v659;
                let v1754 = Lanes([v1744[0], 0.0, 0.0]);
                v681 = v665;
                v682 = v671;
                v946 = v1754;
                v947 = v1753;
            } else {
                let v672 = v19 - v178;
                let v673 = v30 / v184;
                let v1728 = v1105 * v673;
                let v674 = v19 - v673;
                let v677 = (v672 * (v674.ln())).exp();
                let v678 = v19 - v677;
                let v1738 = v1105 * v678;
                let v680 = (v184 * v678) / v672;
                let v1742 = ((Lanes([v1738[0], 0.0, 0.0])) + ((((((((v1725 - (Lanes([v1728[0], 0.0, 0.0]))) / v184) * v983) * (v890 / v674)) * v672) * v677) * v983) * v184)) / v672;
                v681 = v680;
                v682 = v0;
                v946 = v1742;
                v947 = v1309;
            }
            let v683 = v681 + v682;
            let v1756 = v1114 * v683;
            let v685 = v649 * (v190 * v683);
            let v1760 = ((Lanes([v1756[0], 0.0, 0.0])) + ((v946 + v947) * v190)) * v649;
            let v689 = if (if v686 != v0 { 1.0 } else { 0.0 }) != 0.0 && (if v527 != v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v830: f64;
            let v948: Lanes<4>;
            if v689 != 0.0 {
                let v695 = (((v26 * v686) * v691) / v693) * v527;
                let v696 = v695 * v476;
                let v1762 = v1498 * v695;
                v830 = v696;
                v948 = v1762;
            } else {
                v830 = v0;
                v948 = v1761;
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
            let v885: f64;
            let v887: f64;
            let v889: f64;
            let v949: Lanes<7>;
            let v950: Lanes<1>;
            let v951: Lanes<1>;
            let v952: Lanes<7>;
            let v953: Lanes<2>;
            let v954: Lanes<1>;
            let v955: Lanes<1>;
            let v956: Lanes<1>;
            let v957: Lanes<7>;
            let v958: Lanes<1>;
            let v959: Lanes<1>;
            let v960: Lanes<1>;
            if v701 != 0.0 {
                let v703 = v449 * v517;
                let v1825 = v1455 * v517;
                let v1826 = v1559 * v449;
                let v1834 = (((Lanes([0.0, 0.0, v1825[0], v1825[1], v1825[2], v1825[3]])) + (Lanes([v1826[0], v1826[1], 0.0, 0.0, 0.0, 0.0]))) * ((v1210 * (if v703 >= v1208 { 1.0 } else { 0.0 })) - v890)) * v702;
                let v707 = v220 - v706;
                let v708 = v453 * v707;
                let v1838 = v1460 * v707;
                let v1839 = ((Lanes([0.0, v896[0]])) - (Lanes([v899[0], 0.0]))) * v453;
                let v1846 = ((Lanes([0.0, 0.0, v1838[0], v1838[1], v1838[2]])) + (Lanes([v1839[0], v1839[1], 0.0, 0.0, 0.0]))) * ((v1210 * (if v708 >= v1208 { 1.0 } else { 0.0 })) - v890);
                let v710 = (v702 * (v703.abs())) - (v708.abs());
                let v1849 = (Lanes([0.0, v1834[0], v1834[1], v1834[2], v1834[3], v1834[4], v1834[5]])) - (Lanes([v1846[0], v1846[1], 0.0, v1846[2], v1846[3], v1846[4], 0.0]));
                let v711 = v2 / v699;
                let v1850 = v891 / v699;
                let v713 = v2 * v712;
                let v1851 = v891 * v712;
                let v714 = ddt(3871, v713);
                let v1852 = v1851 * v1427;
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
                v885 = v713;
                v887 = v0;
                v889 = v0;
                v949 = v1849;
                v950 = v1850;
                v951 = v1852;
                v952 = v1763;
                v953 = v1789;
                v954 = v976;
                v955 = v1790;
                v956 = v1790;
                v957 = v1763;
                v958 = v1851;
                v959 = v976;
                v960 = v1790;
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
                let v886: f64;
                let v888: f64;
                let v961: Lanes<7>;
                let v962: Lanes<2>;
                let v963: Lanes<1>;
                let v964: Lanes<1>;
                let v965: Lanes<1>;
                let v966: Lanes<7>;
                let v967: Lanes<1>;
                let v968: Lanes<1>;
                if v720 != 0.0 {
                    let v722 = v449 * v517;
                    let v1791 = v1455 * v517;
                    let v1792 = v1559 * v449;
                    let v1800 = (((Lanes([0.0, 0.0, v1791[0], v1791[1], v1791[2], v1791[3]])) + (Lanes([v1792[0], v1792[1], 0.0, 0.0, 0.0, 0.0]))) * ((v1210 * (if v722 >= v1208 { 1.0 } else { 0.0 })) - v890)) * v721;
                    let v725 = v220 - v706;
                    let v726 = v453 * v725;
                    let v1804 = v1460 * v725;
                    let v1805 = ((Lanes([0.0, v896[0]])) - (Lanes([v899[0], 0.0]))) * v453;
                    let v1812 = ((Lanes([0.0, 0.0, v1804[0], v1804[1], v1804[2]])) + (Lanes([v1805[0], v1805[1], 0.0, 0.0, 0.0]))) * ((v1210 * (if v726 >= v1208 { 1.0 } else { 0.0 })) - v890);
                    let v728 = (v721 * (v722.abs())) - (v726.abs());
                    let v1815 = (Lanes([0.0, v1800[0], v1800[1], v1800[2], v1800[3], v1800[4], v1800[5]])) - (Lanes([v1812[0], v1812[1], 0.0, v1812[2], v1812[3], v1812[4], 0.0]));
                    let v731 = (v2 - v729) / v699;
                    let v1819 = ((Lanes([v891[0], 0.0])) - (Lanes([0.0, v900[0]]))) / v699;
                    let v732 = v712 * v2;
                    let v1820 = v891 * v712;
                    let v733 = ddt(3902, v732);
                    let v1821 = v1820 * v1427;
                    let v734 = v729 / v718;
                    let v1822 = v900 / v718;
                    let v736 = v735 * v729;
                    let v1823 = v900 * v735;
                    let v737 = ddt(3909, v736);
                    let v1824 = v1823 * v1427;
                    v855 = v728;
                    v857 = v731;
                    v859 = v733;
                    v861 = v734;
                    v863 = v737;
                    v865 = v0;
                    v868 = v0;
                    v871 = v0;
                    v874 = v0;
                    v886 = v732;
                    v888 = v736;
                    v961 = v1815;
                    v962 = v1819;
                    v963 = v1821;
                    v964 = v1822;
                    v965 = v1824;
                    v966 = v1763;
                    v967 = v1820;
                    v968 = v1823;
                } else {
                    let v739 = if v697 == v738 { 1.0 } else { 0.0 };
                    let v866: f64;
                    let v869: f64;
                    let v872: f64;
                    let v875: f64;
                    let v969: Lanes<7>;
                    if v739 != 0.0 {
                        let v741 = v449 * v517;
                        let v1764 = v1455 * v517;
                        let v1765 = v1559 * v449;
                        let v1773 = (((Lanes([0.0, 0.0, v1764[0], v1764[1], v1764[2], v1764[3]])) + (Lanes([v1765[0], v1765[1], 0.0, 0.0, 0.0, 0.0]))) * ((v1210 * (if v741 >= v1208 { 1.0 } else { 0.0 })) - v890)) * v740;
                        let v744 = v220 - v706;
                        let v745 = v453 * v744;
                        let v1777 = v1460 * v744;
                        let v1778 = ((Lanes([0.0, v896[0]])) - (Lanes([v899[0], 0.0]))) * v453;
                        let v1785 = ((Lanes([0.0, 0.0, v1777[0], v1777[1], v1777[2]])) + (Lanes([v1778[0], v1778[1], 0.0, 0.0, 0.0]))) * ((v1210 * (if v745 >= v1208 { 1.0 } else { 0.0 })) - v890);
                        let v747 = (v740 * (v741.abs())) - (v745.abs());
                        let v1788 = (Lanes([0.0, v1773[0], v1773[1], v1773[2], v1773[3], v1773[4], v1773[5]])) - (Lanes([v1785[0], v1785[1], 0.0, v1785[2], v1785[3], v1785[4], 0.0]));
                        v866 = v747;
                        v869 = v748;
                        v872 = v0;
                        v875 = v0;
                        v969 = v1788;
                    } else {
                        v866 = v0;
                        v869 = v0;
                        v872 = v749;
                        v875 = v750;
                        v969 = v1763;
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
                    v886 = v0;
                    v888 = v0;
                    v961 = v1763;
                    v962 = v1789;
                    v963 = v976;
                    v964 = v1790;
                    v965 = v1790;
                    v966 = v969;
                    v967 = v976;
                    v968 = v1790;
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
                v885 = v0;
                v887 = v886;
                v889 = v888;
                v949 = v1763;
                v950 = v976;
                v951 = v976;
                v952 = v961;
                v953 = v962;
                v954 = v963;
                v955 = v964;
                v956 = v965;
                v957 = v966;
                v958 = v976;
                v959 = v967;
                v960 = v968;
            }
            let v751 = ctx.simparam_or("gmin", v0);
            let v752 = v751 * v218;
            let v1853 = v1152 * v751;
            let v753 = ctx.simparam_or("gmin", v0);
            let v754 = v753 * v29;
            let v1854 = v979 * v753;
            let v755 = ctx.simparam_or("gmin", v0);
            let v757 = v755 * (v28 - v217);
            let v1858 = ((Lanes([v893[0], 0.0])) - (Lanes([0.0, v895[0]]))) * v755;
            let v760 = (v496 + (v551 * v554)) / v25;
            let v763 = (v509 + (v551 * v558)) / v25;
            let v766 = (v504 + (v551 * v556)) / v25;
            let v770 = if (if v760 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v760 >= v768 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v876: f64;
            let v877: f64;
            let v878: f64;
            let v970: Lanes<4>;
            if v770 != 0.0 {
                let v772 = v771 / v25;
                let v1860 = v938 / v25;
                let v773 = if v772 > v768 { 1.0 } else { 0.0 };
                let v774: f64;
                let v971: Lanes<4>;
                if v773 != 0.0 {
                    v774 = v772;
                    v971 = v1860;
                } else {
                    v774 = v768;
                    v971 = v1859;
                }
                let v775 = v223 / v774;
                let v1864 = ((Lanes([v1160[0], 0.0, v1160[1], 0.0])) - (v971 * v775)) / v774;
                let v776 = if v772 >= v768 { 1.0 } else { 0.0 };
                if v776 != 0.0 {
                } else {
                }
                v876 = v775;
                v877 = v777;
                v878 = v0;
                v970 = v1864;
            } else {
                v876 = v0;
                v877 = v0;
                v878 = v778;
                v970 = v1859;
            }
            let v781 = if (if v763 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v763 >= v768 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v879: f64;
            let v880: f64;
            let v881: f64;
            let v972: Lanes<3>;
            if v781 != 0.0 {
                let v783 = v782 / v25;
                let v1866 = v939 / v25;
                let v784 = if v783 > v768 { 1.0 } else { 0.0 };
                let v785: f64;
                let v973: Lanes<3>;
                if v784 != 0.0 {
                    v785 = v783;
                    v973 = v1866;
                } else {
                    v785 = v768;
                    v973 = v1865;
                }
                let v786 = v225 / v785;
                let v1870 = ((Lanes([v1164[0], 0.0, v1164[1]])) - (v973 * v786)) / v785;
                let v787 = if v783 >= v768 { 1.0 } else { 0.0 };
                if v787 != 0.0 {
                } else {
                }
                v879 = v786;
                v880 = v788;
                v881 = v0;
                v972 = v1870;
            } else {
                v879 = v0;
                v880 = v0;
                v881 = v789;
                v972 = v1865;
            }
            let v792 = if (if v766 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v766 >= v768 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v882: f64;
            let v883: f64;
            let v884: f64;
            let v974: Lanes<3>;
            if v792 != 0.0 {
                let v793 = v706 - v28;
                let v1874 = (Lanes([v899[0], 0.0])) - (Lanes([0.0, v893[0]]));
                let v795 = v794 / v25;
                let v1875 = v940 / v25;
                let v796 = if v795 > v768 { 1.0 } else { 0.0 };
                let v797: f64;
                let v975: Lanes<1>;
                if v796 != 0.0 {
                    v797 = v795;
                    v975 = v1875;
                } else {
                    v797 = v768;
                    v975 = v976;
                }
                let v798 = v793 / v797;
                let v1876 = v975 * v798;
                let v1880 = ((Lanes([v1874[0], 0.0, v1874[1]])) - (Lanes([0.0, v1876[0], 0.0]))) / v797;
                let v799 = if v795 >= v768 { 1.0 } else { 0.0 };
                if v799 != 0.0 {
                } else {
                }
                v882 = v798;
                v883 = v800;
                v884 = v0;
                v974 = v1880;
            } else {
                v882 = v0;
                v883 = v0;
                v884 = v801;
                v974 = v1871;
            }
            let v802 = v26 * v449;
            let v803 = v802 * v25;
            let v1882 = (v1455 * v26) * v25;
            let v805 = (v26 * v453) * v25;
            let v1884 = (v1460 * v26) * v25;
            let v808 = v26 * ((-v475) * v25);
            let v1887 = ((v1494 * v983) * v25) * v26;
            let v810 = (v26 * v483) * v25;
            let v1889 = (v1511 * v26) * v25;
            let v812 = (v26 * v613) * v25;
            let v1891 = (v1685 * v26) * v25;
            let v813 = ddt(4101, v812);
            let v1892 = v1891 * v1427;
            let v815 = (v26 * v532) * v25;
            let v1894 = (v1579 * v26) * v25;
            let v816 = ddt(4107, v815);
            let v1895 = v1894 * v1427;
            let v818 = (v26 * v651) * v25;
            let v1897 = (v1724 * v26) * v25;
            let v819 = ddt(4113, v818);
            let v1898 = v1897 * v1427;
            let v821 = (v26 * v685) * v25;
            let v1900 = (v1760 * v26) * v25;
            let v822 = ddt(4119, v821);
            let v1901 = v1900 * v1427;
            let v824 = (v26 * v534) * v25;
            let v1903 = (v1580 * v26) * v25;
            let v825 = ddt(4125, v824);
            let v1904 = v1903 * v1427;
            let v828 = (v26 * v826) * v25;
            let v1906 = (v941 * v26) * v25;
            let v829 = ddt(4131, v828);
            let v1907 = v1906 * v1427;
            let v832 = (-v830) * v25;
            let v1909 = (v948 * v983) * v25;
            let v833 = ddt(4136, v832);
            let v1910 = v1909 * v1427;
            let v834 = v830 * v25;
            let v1911 = v948 * v25;
            let v835 = ddt(4140, v834);
            let v1912 = v1911 * v1427;
            let v841 = if (if (if v836 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v838 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) > v0 { 1.0 } else { 0.0 };
            if v841 != 0.0 {
            } else {
            }
            let v843 = if v802 >= v0 { 1.0 } else { 0.0 };
            if v843 != 0.0 {
            } else {
            }
            let v1913 = v1425[0];
            let v1914 = v1425[1];
            let v1915 = v1425[2];
            let v1916 = v1426[0];
            let v1917 = v1429[0];
            let v1918 = v935[0];
            let v1919 = v935[1];
            let v1920 = v935[2];
            let v1921 = v935[3];
            let v1922 = v935[4];
            let v1923 = v935[5];
            let v1924 = v936[0];
            let v1925 = v937[0];
            let v1926 = v937[1];
            let v1927 = v937[2];
            let v1928 = v949[0];
            let v1929 = v949[1];
            let v1930 = v949[2];
            let v1931 = v949[3];
            let v1932 = v949[4];
            let v1933 = v949[5];
            let v1934 = v949[6];
            let v1935 = v950[0];
            let v1936 = v951[0];
            let v1937 = v952[0];
            let v1938 = v952[1];
            let v1939 = v952[2];
            let v1940 = v952[3];
            let v1941 = v952[4];
            let v1942 = v952[5];
            let v1943 = v952[6];
            let v1944 = v953[0];
            let v1945 = v953[1];
            let v1946 = v954[0];
            let v1947 = v955[0];
            let v1948 = v956[0];
            let v1949 = v957[0];
            let v1950 = v957[1];
            let v1951 = v957[2];
            let v1952 = v957[3];
            let v1953 = v957[4];
            let v1954 = v957[5];
            let v1955 = v957[6];
            let v1956 = v1853[0];
            let v1957 = v1853[1];
            let v1958 = v1854[0];
            let v1959 = v1854[1];
            let v1960 = v1858[0];
            let v1961 = v1858[1];
            let v1962 = v970[0];
            let v1963 = v970[1];
            let v1964 = v970[2];
            let v1965 = v970[3];
            let v1966 = v972[0];
            let v1967 = v972[1];
            let v1968 = v972[2];
            let v1969 = v974[0];
            let v1970 = v974[1];
            let v1971 = v974[2];
            let v1972 = v1882[0];
            let v1973 = v1882[1];
            let v1974 = v1882[2];
            let v1975 = v1882[3];
            let v1976 = v1884[0];
            let v1977 = v1884[1];
            let v1978 = v1884[2];
            let v1979 = v1887[0];
            let v1980 = v1887[1];
            let v1981 = v1887[2];
            let v1982 = v1887[3];
            let v1983 = v1889[0];
            let v1984 = v1889[1];
            let v1985 = v1889[2];
            let v1986 = v1889[3];
            let v1987 = v1889[4];
            let v1988 = v1892[0];
            let v1989 = v1892[1];
            let v1990 = v1892[2];
            let v1991 = v1895[0];
            let v1992 = v1895[1];
            let v1993 = v1895[2];
            let v1994 = v1895[3];
            let v1995 = v1895[4];
            let v1996 = v1898[0];
            let v1997 = v1898[1];
            let v1998 = v1898[2];
            let v1999 = v1901[0];
            let v2000 = v1901[1];
            let v2001 = v1901[2];
            let v2002 = v1904[0];
            let v2003 = v1904[1];
            let v2004 = v1904[2];
            let v2005 = v1904[3];
            let v2006 = v1907[0];
            let v2007 = v1907[1];
            let v2008 = v1907[2];
            let v2009 = v1910[0];
            let v2010 = v1910[1];
            let v2011 = v1910[2];
            let v2012 = v1910[3];
            let v2013 = v1912[0];
            let v2014 = v1912[1];
            let v2015 = v1912[2];
            let v2016 = v1912[3];
            let v2017 = v958[0];
            let v2018 = v959[0];
            let v2019 = v960[0];
            let v2020 = v1891[0];
            let v2021 = v1891[1];
            let v2022 = v1891[2];
            let v2023 = v1894[0];
            let v2024 = v1894[1];
            let v2025 = v1894[2];
            let v2026 = v1894[3];
            let v2027 = v1894[4];
            let v2028 = v1897[0];
            let v2029 = v1897[1];
            let v2030 = v1897[2];
            let v2031 = v1900[0];
            let v2032 = v1900[1];
            let v2033 = v1900[2];
            let v2034 = v1903[0];
            let v2035 = v1903[1];
            let v2036 = v1903[2];
            let v2037 = v1903[3];
            let v2038 = v1906[0];
            let v2039 = v1906[1];
            let v2040 = v1906[2];
            let v2041 = v1909[0];
            let v2042 = v1909[1];
            let v2043 = v1909[2];
            let v2044 = v1909[3];
            let v2045 = v1911[0];
            let v2046 = v1911[1];
            let v2047 = v1911[2];
            let v2048 = v1911[3];
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(9),
            None,
            multiplicity * (v432),
            [5, 6, 9],
            [v1913, v1914, v1915],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(9),
            None,
            multiplicity * (v434),
            [9],
            [v1916],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(9),
            None,
            multiplicity * (v437),
            [9],
            [v1917],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            None,
            multiplicity * (v846),
            [1, 2, 3, 4, 5, 6],
            [v1918, v1919, v1920, v1921, v1922, v1923],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(8),
            None,
            multiplicity * (v847),
            [8],
            [v1924],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(8),
            None,
            multiplicity * (v848),
            [1, 2, 8],
            [v1925, v1926, v1927],
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
            [v1928, v1929, v1930, v1931, v1932, v1933, v1934],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (v851),
            [3],
            [v1935],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (v852),
            [3],
            [v1936],
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
            [v1937, v1938, v1939, v1940, v1941, v1942, v1943],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(3),
            Some(7),
            multiplicity * (v856),
            [3, 7],
            [v1944, v1945],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (v858),
            [3],
            [v1946],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(7),
            None,
            multiplicity * (v860),
            [7],
            [v1947],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(7),
            None,
            multiplicity * (v862),
            [7],
            [v1948],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(3),
            None,
            multiplicity * (v864),
            [0, 1, 2, 3, 4, 5, 6],
            [v1949, v1950, v1951, v1952, v1953, v1954, v1955],
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
            [v1956, v1957],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(5),
            Some(4),
            multiplicity * (v754),
            [4, 5],
            [v1958, v1959],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(4),
            Some(6),
            multiplicity * (v757),
            [4, 6],
            [v1960, v1961],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(5),
            multiplicity * (v876),
            [1, 3, 5, 8],
            [v1962, v1963, v1964, v1965],
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
            [v1966, v1967, v1968],
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
            [v1969, v1970, v1971],
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
            [v1972, v1973, v1974, v1975],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(4),
            multiplicity * (v805),
            [3, 4, 5],
            [v1976, v1977, v1978],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            Some(6),
            multiplicity * (v808),
            [3, 4, 5, 6],
            [v1979, v1980, v1981, v1982],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(6),
            multiplicity * (v810),
            [3, 4, 5, 6, 9],
            [v1983, v1984, v1985, v1986, v1987],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(6),
            multiplicity * (v813),
            [3, 5, 6],
            [v1988, v1989, v1990],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * (v816),
            [1, 2, 3, 5, 6],
            [v1991, v1992, v1993, v1994, v1995],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(4),
            multiplicity * (v819),
            [1, 3, 4],
            [v1996, v1997, v1998],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(4),
            multiplicity * (v822),
            [3, 4, 5],
            [v1999, v2000, v2001],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (v825),
            [3, 4, 5, 6],
            [v2002, v2003, v2004, v2005],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(2),
            Some(4),
            multiplicity * (v829),
            [2, 3, 4],
            [v2006, v2007, v2008],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (v833),
            [3, 4, 5, 6],
            [v2009, v2010, v2011, v2012],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (v835),
            [3, 4, 5, 6],
            [v2013, v2014, v2015, v2016],
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
        self.canonical_reactive[2] = v437;
        self.canonical_reactive[3] = v846;
        self.canonical_reactive[4] = v847;
        self.canonical_reactive[5] = v848;
        self.canonical_reactive[6] = v849;
        self.canonical_reactive[7] = v850;
        self.canonical_reactive[8] = v851;
        self.canonical_reactive[9] = v885;
        self.canonical_reactive[10] = v2017;
        self.canonical_reactive[11] = v853;
        self.canonical_reactive[12] = v854;
        self.canonical_reactive[13] = v856;
        self.canonical_reactive[14] = v887;
        self.canonical_reactive[15] = v2018;
        self.canonical_reactive[16] = v860;
        self.canonical_reactive[17] = v889;
        self.canonical_reactive[18] = v2019;
        self.canonical_reactive[19] = v864;
        self.canonical_reactive[20] = v867;
        self.canonical_reactive[21] = v870;
        self.canonical_reactive[22] = v873;
        self.canonical_reactive[23] = v752;
        self.canonical_reactive[24] = v754;
        self.canonical_reactive[25] = v757;
        self.canonical_reactive[26] = v876;
        self.canonical_reactive[27] = v877;
        self.canonical_reactive[28] = v878;
        self.canonical_reactive[29] = v879;
        self.canonical_reactive[30] = v880;
        self.canonical_reactive[31] = v881;
        self.canonical_reactive[32] = v882;
        self.canonical_reactive[33] = v883;
        self.canonical_reactive[34] = v884;
        self.canonical_reactive[35] = v803;
        self.canonical_reactive[36] = v805;
        self.canonical_reactive[37] = v808;
        self.canonical_reactive[38] = v810;
        self.canonical_reactive[39] = v812;
        self.canonical_reactive[40] = v2020;
        self.canonical_reactive[41] = v2021;
        self.canonical_reactive[42] = v2022;
        self.canonical_reactive[43] = v815;
        self.canonical_reactive[44] = v2023;
        self.canonical_reactive[45] = v2024;
        self.canonical_reactive[46] = v2025;
        self.canonical_reactive[47] = v2026;
        self.canonical_reactive[48] = v2027;
        self.canonical_reactive[49] = v818;
        self.canonical_reactive[50] = v2028;
        self.canonical_reactive[51] = v2029;
        self.canonical_reactive[52] = v2030;
        self.canonical_reactive[53] = v821;
        self.canonical_reactive[54] = v2031;
        self.canonical_reactive[55] = v2032;
        self.canonical_reactive[56] = v2033;
        self.canonical_reactive[57] = v824;
        self.canonical_reactive[58] = v2034;
        self.canonical_reactive[59] = v2035;
        self.canonical_reactive[60] = v2036;
        self.canonical_reactive[61] = v2037;
        self.canonical_reactive[62] = v828;
        self.canonical_reactive[63] = v2038;
        self.canonical_reactive[64] = v2039;
        self.canonical_reactive[65] = v2040;
        self.canonical_reactive[66] = v832;
        self.canonical_reactive[67] = v2041;
        self.canonical_reactive[68] = v2042;
        self.canonical_reactive[69] = v2043;
        self.canonical_reactive[70] = v2044;
        self.canonical_reactive[71] = v834;
        self.canonical_reactive[72] = v2045;
        self.canonical_reactive[73] = v2046;
        self.canonical_reactive[74] = v2047;
        self.canonical_reactive[75] = v2048;
        self.canonical_reactive[76] = v842;
        self.canonical_reactive[77] = v844;
        self.canonical_reactive[78] = v845;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            None,
            &[3],
            &[cached[10]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            None,
            &[3],
            &[cached[15]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            None,
            &[7],
            &[cached[18]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(6),
            &[3, 5, 6],
            &[cached[40], cached[41], cached[42]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(6),
            &[1, 2, 3, 5, 6],
            &[cached[44], cached[45], cached[46], cached[47], cached[48]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(4),
            &[1, 3, 4],
            &[cached[50], cached[51], cached[52]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(4),
            &[3, 4, 5],
            &[cached[54], cached[55], cached[56]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(4),
            &[3, 4, 5, 6],
            &[cached[58], cached[59], cached[60], cached[61]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(2),
            Some(4),
            &[2, 3, 4],
            &[cached[63], cached[64], cached[65]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(6),
            &[3, 4, 5, 6],
            &[cached[67], cached[68], cached[69], cached[70]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(4),
            &[3, 4, 5, 6],
            &[cached[72], cached[73], cached[74], cached[75]],
            &[],
            &[],
            multiplicity,
        );
    }

}

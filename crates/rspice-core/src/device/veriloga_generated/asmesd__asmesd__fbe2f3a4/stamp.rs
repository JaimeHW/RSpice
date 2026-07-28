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
            let v885 = 1e0f64;
            let v886 = Lanes([1e0f64; 1]);
            let v887 = Lanes([1e0f64; 1]);
            let v888 = Lanes([1e0f64; 1]);
            let v889 = Lanes([1e0f64; 1]);
            let v890 = Lanes([1e0f64; 1]);
            let v891 = Lanes([1e0f64; 1]);
            let v892 = Lanes([1e0f64; 1]);
            let v893 = Lanes([1e0f64; 1]);
            let v894 = Lanes([1e0f64; 1]);
            let v895 = Lanes([1e0f64; 1]);
            let v966 = Lanes([0e0f64; 1]);
            let v973 = -1e0f64;
            let v1156 = Lanes([0e0f64; 3]);
            let v1198 = 0e0f64;
            let v1200 = 2e0f64;
            let v1299 = Lanes([0e0f64; 3]);
            let v1417 = ddt_scale();
            let v1572 = Lanes([0e0f64; 6]);
            let v1573 = Lanes([0e0f64; 1]);
            let v1574 = Lanes([0e0f64; 3]);
            let v1696 = Lanes([0e0f64; 3]);
            let v1751 = Lanes([0e0f64; 4]);
            let v1753 = Lanes([0e0f64; 7]);
            let v1779 = Lanes([0e0f64; 2]);
            let v1780 = Lanes([0e0f64; 1]);
            let v1849 = Lanes([0e0f64; 4]);
            let v1855 = Lanes([0e0f64; 3]);
            let v1861 = Lanes([0e0f64; 3]);
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
            let v896: Lanes<1>;
            if v12 != 0.0 {
                v18 = v13;
                v896 = v966;
            } else {
                let v15 = if v5 > v14 { 1.0 } else { 0.0 };
                let v17: f64;
                let v897: Lanes<1>;
                if v15 != 0.0 {
                    v17 = v5;
                    v897 = v886;
                } else {
                    v17 = v16;
                    v897 = v966;
                }
                v18 = v17;
                v896 = v897;
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
            let v969 = (Lanes([0.0, v887[0]])) - (Lanes([v888[0], 0.0]));
            let v30 = v26 * v29;
            let v970 = v969 * v26;
            let v33 = -(if v30 <= v0 { v30 } else { v0 });
            let v37 = v19 + (v31 * (v33.powf(v34)));
            let v39 = v38 + v6;
            let v41 = v40 * v18;
            let v980 = v896 * v40;
            let v42 = v18 / v39;
            let v981 = v896 / v39;
            let v43 = v42.ln();
            let v983 = v981 * (v885 / v42);
            let v46 = (v44 * v43).exp();
            let v985 = (v983 * v44) * v46;
            let v48 = v47 * v46;
            let v49 = v48 * v37;
            let v987 = (v985 * v47) * v37;
            let v988 = ((((v970 * (if v30 <= v0 { 1.0 } else { 0.0 })) * v973) * (v34 * (v33.powf((v34 - v885))))) * v31) * v48;
            let v991 = (Lanes([v987[0], 0.0, 0.0])) + (Lanes([0.0, v988[0], v988[1]]));
            let v51 = v50 * v46;
            let v992 = v985 * v50;
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
            let v998 = (v983 * v68) + (((v981 * v70) - (v980 * v73)) / v41);
            let v78 = v74.exp();
            let v79 = v77 * v78;
            let v1001 = (v998 * v78) * v77;
            let v81 = (v75 * v43).exp();
            let v82 = v80 * v81;
            let v1003 = ((v983 * v75) * v81) * v80;
            let v86 = (v74 / v84).exp();
            let v88 = (v83 * v86) / v46;
            let v1009 = ((((v998 / v84) * v86) * v83) - (v985 * v88)) / v46;
            let v92 = (v74 / v90).exp();
            let v94 = (v89 * v92) / v46;
            let v1015 = ((((v998 / v90) * v92) * v89) - (v985 * v94)) / v46;
            let v99 = v95 * (v19 + (v96 * v71));
            let v1017 = (v981 * v96) * v95;
            let v104 = v100 * (v19 + (v101 * v71));
            let v1019 = (v981 * v101) * v100;
            let v109 = v105 * (v19 + (v106 * v71));
            let v1021 = (v981 * v106) * v105;
            let v114 = v110 * (v19 + (v111 * v71));
            let v1023 = (v981 * v111) * v110;
            let v119 = v39 / v118;
            let v120 = v18 / v118;
            let v1024 = v896 / v118;
            let v123 = v122 * v18;
            let v126 = v125 + v18;
            let v127 = (v123 * v18) / v126;
            let v1033 = ((((((v896 * v122) * v18) + (v896 * v123)) - (v896 * v127)) / v126) * v973) * v973;
            let v132 = v130 * (v18 + v18);
            let v133 = (-(v121 - v127)) / v132;
            let v1036 = ((v896 + v896) * v130) * v133;
            let v137 = -(v41 + v41);
            let v1040 = (v980 + v980) * v973;
            let v140 = v138 * (v120.ln());
            let v1043 = (v1024 * (v885 / v120)) * v138;
            let v143 = v140 + (v141 * (v133 + v134));
            let v144 = v137 * v143;
            let v1048 = (v1040 * v143) + ((v1043 + (((v1033 - v1036) / v132) * v141)) * v137);
            let v147 = (v145 - v144) / v119;
            let v1050 = (v1048 * v973) / v119;
            let v149 = (v145 - v147) / v147;
            let v153 = v151 * (v39 - v118);
            let v156 = v19 + (v150 * (v153 - v149));
            let v157 = v115 / v156;
            let v159 = (v120 * v147) + v144;
            let v1063 = ((v1024 * v147) + (v1050 * v120)) + v1048;
            let v161 = (v159 - v147) / v147;
            let v163 = v151 * (v18 - v118);
            let v1068 = v896 * v151;
            let v166 = v19 + (v150 * (v163 - v161));
            let v167 = v157 * v166;
            let v1073 = (((((((((v1050 * v973) - (v1050 * v149)) / v147) * v973) * v150) * v157) * v973) / v156) * v166) + (((v1068 - (((v1063 - v1050) - (v1050 * v161)) / v147)) * v150) * v157);
            let v171 = v140 + (v141 * (v133 + v168));
            let v172 = v137 * v171;
            let v1080 = (v1040 * v171) + ((v1043 + (((v1033 - v1036) / v132) * v141)) * v137);
            let v175 = (v173 - v172) / v119;
            let v1082 = (v1080 * v973) / v119;
            let v177 = (v173 - v175) / v175;
            let v181 = v19 + (v178 * (v153 - v177));
            let v182 = v116 / v181;
            let v184 = (v120 * v175) + v172;
            let v1095 = ((v1024 * v175) + (v1082 * v120)) + v1080;
            let v186 = (v184 - v175) / v175;
            let v189 = v19 + (v178 * (v163 - v186));
            let v190 = v182 * v189;
            let v1104 = (((((((((v1082 * v973) - (v1082 * v177)) / v175) * v973) * v178) * v182) * v973) / v181) * v189) + (((v1068 - (((v1095 - v1082) - (v1082 * v186)) / v175)) * v178) * v182);
            let v194 = v140 + (v141 * (v133 + v191));
            let v195 = v137 * v194;
            let v1111 = (v1040 * v194) + ((v1043 + (((v1033 - v1036) / v132) * v141)) * v137);
            let v198 = (v196 - v195) / v119;
            let v1113 = (v1111 * v973) / v119;
            let v200 = (v196 - v198) / v198;
            let v204 = v19 + (v201 * (v153 - v200));
            let v205 = v117 / v204;
            let v207 = (v120 * v198) + v195;
            let v1126 = ((v1024 * v198) + (v1113 * v120)) + v1111;
            let v209 = (v207 - v198) / v198;
            let v212 = v19 + (v201 * (v163 - v209));
            let v213 = v205 * v212;
            let v1135 = (((((((((v1113 * v973) - (v1113 * v200)) / v198) * v973) * v201) * v205) * v973) / v204) * v212) + (((v1068 - (((v1126 - v1113) - (v1113 * v209)) / v198)) * v201) * v205);
            let v216 = v26 * (v214 - v28);
            let v1139 = ((Lanes([v889[0], 0.0])) - (Lanes([0.0, v888[0]]))) * v26;
            let v218 = v27 - v217;
            let v1142 = (Lanes([v887[0], 0.0])) - (Lanes([0.0, v890[0]]));
            let v219 = v26 * v218;
            let v1143 = v1142 * v26;
            let v222 = v26 * (v220 - v28);
            let v1147 = ((Lanes([v891[0], 0.0])) - (Lanes([0.0, v888[0]]))) * v26;
            let v223 = v220 - v27;
            let v1150 = (Lanes([v891[0], 0.0])) - (Lanes([0.0, v887[0]]));
            let v224 = v26 * v223;
            let v1151 = v1150 * v26;
            let v225 = v214 - v217;
            let v1154 = (Lanes([v889[0], 0.0])) - (Lanes([0.0, v890[0]]));
            let v226 = v26 * v225;
            let v1155 = v1154 * v26;
            let v227 = if v79 > v0 { 1.0 } else { 0.0 };
            let v444: f64;
            let v898: Lanes<3>;
            if v227 != 0.0 {
                let v229 = v228 * v41;
                let v230 = v219 / v229;
                let v1158 = (v980 * v228) * v230;
                let v1162 = ((Lanes([0.0, v1143[0], v1143[1]])) - (Lanes([v1158[0], 0.0, 0.0]))) / v229;
                let v1163 = v1143 * v973;
                let v234 = v233 * v41;
                let v1167 = v980 * v233;
                let v235 = ((-v219) - v104) / v234;
                let v1168 = v1167 * v235;
                let v1171 = (((Lanes([0.0, v1163[0], v1163[1]])) - (Lanes([v1019[0], 0.0, 0.0]))) - (Lanes([v1168[0], 0.0, 0.0]))) / v234;
                let v237 = (-v104) / v234;
                let v1175 = ((v1019 * v973) - (v1167 * v237)) / v234;
                let v239 = if v230 > v238 { 1.0 } else { 0.0 };
                let v242: f64;
                let v243: f64;
                let v899: Lanes<3>;
                let v900: Lanes<3>;
                if v239 != 0.0 {
                    let v241 = v19 + (v230 - v238);
                    v242 = v241;
                    v243 = v238;
                    v899 = v1162;
                    v900 = v1156;
                } else {
                    v242 = v19;
                    v243 = v230;
                    v899 = v1156;
                    v900 = v1162;
                }
                let v244 = v243.exp();
                let v245 = v242 * v244;
                let v1179 = (v899 * v244) + ((v900 * v244) * v242);
                let v247 = if v235 >= v246 { 1.0 } else { 0.0 };
                let v261: f64;
                let v901: Lanes<3>;
                if v247 != 0.0 {
                    v261 = v235;
                    v901 = v1171;
                } else {
                    let v249 = if v235 <= v248 { 1.0 } else { 0.0 };
                    let v262: f64;
                    let v902: Lanes<3>;
                    if v249 != 0.0 {
                        let v250 = v235.exp();
                        let v1183 = v1171 * v250;
                        v262 = v250;
                        v902 = v1183;
                    } else {
                        let v251 = v235.exp();
                        let v252 = v251 + v19;
                        let v253 = v252.ln();
                        let v1182 = (v1171 * v251) * (v885 / v252);
                        v262 = v253;
                        v902 = v1182;
                    }
                    v261 = v262;
                    v901 = v902;
                }
                let v254 = if v237 >= v246 { 1.0 } else { 0.0 };
                let v263: f64;
                let v903: Lanes<1>;
                if v254 != 0.0 {
                    v263 = v237;
                    v903 = v1175;
                } else {
                    let v256 = if v237 <= v255 { 1.0 } else { 0.0 };
                    let v264: f64;
                    let v904: Lanes<1>;
                    if v256 != 0.0 {
                        let v257 = v237.exp();
                        let v1187 = v1175 * v257;
                        v264 = v257;
                        v904 = v1187;
                    } else {
                        let v258 = v237.exp();
                        let v259 = v258 + v19;
                        let v260 = v259.ln();
                        let v1186 = (v1175 * v258) * (v885 / v259);
                        v264 = v260;
                        v904 = v1186;
                    }
                    v263 = v264;
                    v903 = v904;
                }
                let v265 = v261 - v263;
                let v266 = v245 - v19;
                let v1190 = v1001 * v266;
                let v1194 = v1017 * v265;
                let v270 = v219.abs();
                let v271 = v270.powf(v109);
                let v1207 = (v1143 * ((v1200 * (if v219 >= v1198 { 1.0 } else { 0.0 })) - v885)) * (v109 * (v270.powf((v109 - v885))));
                let v1210 = v1021 * (v271 * (v270.ln()));
                let v273 = v19 + (v269 * v271);
                let v274 = (v99 * v265) / v273;
                let v275 = (v79 * v266) - v274;
                let v1218 = ((Lanes([v1190[0], 0.0, 0.0])) + (v1179 * v79)) - ((((Lanes([v1194[0], 0.0, 0.0])) + ((v901 - (Lanes([v903[0], 0.0, 0.0]))) * v99)) - ((((Lanes([0.0, v1207[0], v1207[1]])) + (Lanes([v1210[0], 0.0, 0.0]))) * v269) * v274)) / v273);
                v444 = v275;
                v898 = v1218;
            } else {
                v444 = v0;
                v898 = v1156;
            }
            let v276 = if v82 > v0 { 1.0 } else { 0.0 };
            let v445: f64;
            let v905: Lanes<3>;
            if v276 != 0.0 {
                let v278 = v277 - v219;
                let v280 = if v278 >= v279 { v278 } else { v279 };
                let v1223 = (v1143 * v281) * v277;
                let v285 = v284 * v41;
                let v286 = v285 * v280;
                let v1225 = (v980 * v284) * v280;
                let v1226 = ((v1143 * v973) * (if v278 >= v279 { 1.0 } else { 0.0 })) * v285;
                let v287 = ((v281 * v219) * v277) / v286;
                let v1233 = ((Lanes([0.0, v1223[0], v1223[1]])) - (((Lanes([v1225[0], 0.0, 0.0])) + (Lanes([0.0, v1226[0], v1226[1]]))) * v287)) / v286;
                let v288 = if v287 > v238 { 1.0 } else { 0.0 };
                let v291: f64;
                let v292: f64;
                let v906: Lanes<3>;
                let v907: Lanes<3>;
                if v288 != 0.0 {
                    let v290 = v19 + (v287 - v238);
                    v291 = v290;
                    v292 = v238;
                    v906 = v1233;
                    v907 = v1156;
                } else {
                    v291 = v19;
                    v292 = v287;
                    v906 = v1156;
                    v907 = v1233;
                }
                let v293 = v292.exp();
                let v295 = (v291 * v293) - v19;
                let v296 = v82 * v295;
                let v1238 = v1003 * v295;
                let v1241 = (Lanes([v1238[0], 0.0, 0.0])) + (((v906 * v293) + ((v907 * v293) * v291)) * v82);
                v445 = v296;
                v905 = v1241;
            } else {
                v445 = v0;
                v905 = v1156;
            }
            let v297 = if v88 > v0 { 1.0 } else { 0.0 };
            let v448: f64;
            let v908: Lanes<3>;
            if v297 != 0.0 {
                let v298 = v84 * v41;
                let v299 = v219 / v298;
                let v1243 = (v980 * v84) * v299;
                let v1247 = ((Lanes([0.0, v1143[0], v1143[1]])) - (Lanes([v1243[0], 0.0, 0.0]))) / v298;
                let v1248 = v1143 * v973;
                let v303 = v302 * v41;
                let v1252 = v980 * v302;
                let v304 = ((-v219) - v104) / v303;
                let v1253 = v1252 * v304;
                let v1256 = (((Lanes([0.0, v1248[0], v1248[1]])) - (Lanes([v1019[0], 0.0, 0.0]))) - (Lanes([v1253[0], 0.0, 0.0]))) / v303;
                let v306 = (-v104) / v303;
                let v1260 = ((v1019 * v973) - (v1252 * v306)) / v303;
                let v307 = if v299 > v238 { 1.0 } else { 0.0 };
                let v310: f64;
                let v311: f64;
                let v909: Lanes<3>;
                let v910: Lanes<3>;
                if v307 != 0.0 {
                    let v309 = v19 + (v299 - v238);
                    v310 = v309;
                    v311 = v238;
                    v909 = v1247;
                    v910 = v1156;
                } else {
                    v310 = v19;
                    v311 = v299;
                    v909 = v1156;
                    v910 = v1247;
                }
                let v312 = v311.exp();
                let v313 = v310 * v312;
                let v1264 = (v909 * v312) + ((v910 * v312) * v310);
                let v314 = if v304 >= v246 { 1.0 } else { 0.0 };
                let v328: f64;
                let v911: Lanes<3>;
                if v314 != 0.0 {
                    v328 = v304;
                    v911 = v1256;
                } else {
                    let v316 = if v304 <= v315 { 1.0 } else { 0.0 };
                    let v329: f64;
                    let v912: Lanes<3>;
                    if v316 != 0.0 {
                        let v317 = v304.exp();
                        let v1268 = v1256 * v317;
                        v329 = v317;
                        v912 = v1268;
                    } else {
                        let v318 = v304.exp();
                        let v319 = v318 + v19;
                        let v320 = v319.ln();
                        let v1267 = (v1256 * v318) * (v885 / v319);
                        v329 = v320;
                        v912 = v1267;
                    }
                    v328 = v329;
                    v911 = v912;
                }
                let v321 = if v306 >= v246 { 1.0 } else { 0.0 };
                let v330: f64;
                let v913: Lanes<1>;
                if v321 != 0.0 {
                    v330 = v306;
                    v913 = v1260;
                } else {
                    let v323 = if v306 <= v322 { 1.0 } else { 0.0 };
                    let v331: f64;
                    let v914: Lanes<1>;
                    if v323 != 0.0 {
                        let v324 = v306.exp();
                        let v1272 = v1260 * v324;
                        v331 = v324;
                        v914 = v1272;
                    } else {
                        let v325 = v306.exp();
                        let v326 = v325 + v19;
                        let v327 = v326.ln();
                        let v1271 = (v1260 * v325) * (v885 / v326);
                        v331 = v327;
                        v914 = v1271;
                    }
                    v330 = v331;
                    v913 = v914;
                }
                let v333 = v313 - v19;
                let v1275 = v1009 * v333;
                let v336 = v219.abs();
                let v337 = v336.powf(v109);
                let v1287 = (v1143 * ((v1200 * (if v219 >= v1198 { 1.0 } else { 0.0 })) - v885)) * (v109 * (v336.powf((v109 - v885))));
                let v1290 = v1021 * (v337 * (v336.ln()));
                let v339 = v19 + (v269 * v337);
                let v340 = (v0 * (v328 - v330)) / v339;
                let v341 = (v88 * v333) - v340;
                let v1298 = ((Lanes([v1275[0], 0.0, 0.0])) + (v1264 * v88)) - ((((v911 - (Lanes([v913[0], 0.0, 0.0]))) * v0) - ((((Lanes([0.0, v1287[0], v1287[1]])) + (Lanes([v1290[0], 0.0, 0.0]))) * v269) * v340)) / v339);
                v448 = v341;
                v908 = v1298;
            } else {
                v448 = v0;
                v908 = v1156;
            }
            let v450: f64;
            let v915: Lanes<3>;
            if v227 != 0.0 {
                let v343 = v342 * v41;
                let v344 = v30 / v343;
                let v1301 = (v980 * v342) * v344;
                let v1305 = ((Lanes([0.0, v970[0], v970[1]])) - (Lanes([v1301[0], 0.0, 0.0]))) / v343;
                let v1306 = v970 * v973;
                let v347 = v302 * v41;
                let v1310 = v980 * v302;
                let v348 = ((-v30) - v104) / v347;
                let v1311 = v1310 * v348;
                let v1314 = (((Lanes([0.0, v1306[0], v1306[1]])) - (Lanes([v1019[0], 0.0, 0.0]))) - (Lanes([v1311[0], 0.0, 0.0]))) / v347;
                let v350 = (-v104) / v347;
                let v1318 = ((v1019 * v973) - (v1310 * v350)) / v347;
                let v351 = if v344 > v238 { 1.0 } else { 0.0 };
                let v354: f64;
                let v355: f64;
                let v916: Lanes<3>;
                let v917: Lanes<3>;
                if v351 != 0.0 {
                    let v353 = v19 + (v344 - v238);
                    v354 = v353;
                    v355 = v238;
                    v916 = v1305;
                    v917 = v1299;
                } else {
                    v354 = v19;
                    v355 = v344;
                    v916 = v1299;
                    v917 = v1305;
                }
                let v356 = v355.exp();
                let v357 = v354 * v356;
                let v1322 = (v916 * v356) + ((v917 * v356) * v354);
                let v358 = if v348 >= v246 { 1.0 } else { 0.0 };
                let v372: f64;
                let v918: Lanes<3>;
                if v358 != 0.0 {
                    v372 = v348;
                    v918 = v1314;
                } else {
                    let v360 = if v348 <= v359 { 1.0 } else { 0.0 };
                    let v373: f64;
                    let v919: Lanes<3>;
                    if v360 != 0.0 {
                        let v361 = v348.exp();
                        let v1326 = v1314 * v361;
                        v373 = v361;
                        v919 = v1326;
                    } else {
                        let v362 = v348.exp();
                        let v363 = v362 + v19;
                        let v364 = v363.ln();
                        let v1325 = (v1314 * v362) * (v885 / v363);
                        v373 = v364;
                        v919 = v1325;
                    }
                    v372 = v373;
                    v918 = v919;
                }
                let v365 = if v350 >= v246 { 1.0 } else { 0.0 };
                let v374: f64;
                let v920: Lanes<1>;
                if v365 != 0.0 {
                    v374 = v350;
                    v920 = v1318;
                } else {
                    let v367 = if v350 <= v366 { 1.0 } else { 0.0 };
                    let v375: f64;
                    let v921: Lanes<1>;
                    if v367 != 0.0 {
                        let v368 = v350.exp();
                        let v1330 = v1318 * v368;
                        v375 = v368;
                        v921 = v1330;
                    } else {
                        let v369 = v350.exp();
                        let v370 = v369 + v19;
                        let v371 = v370.ln();
                        let v1329 = (v1318 * v369) * (v885 / v370);
                        v375 = v371;
                        v921 = v1329;
                    }
                    v374 = v375;
                    v920 = v921;
                }
                let v376 = v372 - v374;
                let v377 = v357 - v19;
                let v1333 = v1001 * v377;
                let v1337 = v1023 * v376;
                let v380 = v30.abs();
                let v381 = v380.powf(v109);
                let v1348 = (v970 * ((v1200 * (if v30 >= v1198 { 1.0 } else { 0.0 })) - v885)) * (v109 * (v380.powf((v109 - v885))));
                let v1351 = v1021 * (v381 * (v380.ln()));
                let v383 = v19 + (v269 * v381);
                let v384 = (v114 * v376) / v383;
                let v385 = (v79 * v377) - v384;
                let v1359 = ((Lanes([v1333[0], 0.0, 0.0])) + (v1322 * v79)) - ((((Lanes([v1337[0], 0.0, 0.0])) + ((v918 - (Lanes([v920[0], 0.0, 0.0]))) * v114)) - ((((Lanes([0.0, v1348[0], v1348[1]])) + (Lanes([v1351[0], 0.0, 0.0]))) * v269) * v384)) / v383);
                v450 = v385;
                v915 = v1359;
            } else {
                v450 = v0;
                v915 = v1299;
            }
            let v386 = if v94 > v0 { 1.0 } else { 0.0 };
            let v452: f64;
            let v922: Lanes<3>;
            if v386 != 0.0 {
                let v387 = v90 * v41;
                let v388 = v30 / v387;
                let v1361 = (v980 * v90) * v388;
                let v1365 = ((Lanes([0.0, v970[0], v970[1]])) - (Lanes([v1361[0], 0.0, 0.0]))) / v387;
                let v1366 = v970 * v973;
                let v391 = v302 * v41;
                let v1370 = v980 * v302;
                let v392 = ((-v30) - v104) / v391;
                let v1371 = v1370 * v392;
                let v1374 = (((Lanes([0.0, v1366[0], v1366[1]])) - (Lanes([v1019[0], 0.0, 0.0]))) - (Lanes([v1371[0], 0.0, 0.0]))) / v391;
                let v394 = (-v104) / v391;
                let v1378 = ((v1019 * v973) - (v1370 * v394)) / v391;
                let v395 = if v388 > v238 { 1.0 } else { 0.0 };
                let v398: f64;
                let v399: f64;
                let v923: Lanes<3>;
                let v924: Lanes<3>;
                if v395 != 0.0 {
                    let v397 = v19 + (v388 - v238);
                    v398 = v397;
                    v399 = v238;
                    v923 = v1365;
                    v924 = v1299;
                } else {
                    v398 = v19;
                    v399 = v388;
                    v923 = v1299;
                    v924 = v1365;
                }
                let v400 = v399.exp();
                let v401 = v398 * v400;
                let v1382 = (v923 * v400) + ((v924 * v400) * v398);
                let v402 = if v392 >= v246 { 1.0 } else { 0.0 };
                let v416: f64;
                let v925: Lanes<3>;
                if v402 != 0.0 {
                    v416 = v392;
                    v925 = v1374;
                } else {
                    let v404 = if v392 <= v403 { 1.0 } else { 0.0 };
                    let v417: f64;
                    let v926: Lanes<3>;
                    if v404 != 0.0 {
                        let v405 = v392.exp();
                        let v1386 = v1374 * v405;
                        v417 = v405;
                        v926 = v1386;
                    } else {
                        let v406 = v392.exp();
                        let v407 = v406 + v19;
                        let v408 = v407.ln();
                        let v1385 = (v1374 * v406) * (v885 / v407);
                        v417 = v408;
                        v926 = v1385;
                    }
                    v416 = v417;
                    v925 = v926;
                }
                let v409 = if v394 >= v246 { 1.0 } else { 0.0 };
                let v418: f64;
                let v927: Lanes<1>;
                if v409 != 0.0 {
                    v418 = v394;
                    v927 = v1378;
                } else {
                    let v411 = if v394 <= v410 { 1.0 } else { 0.0 };
                    let v419: f64;
                    let v928: Lanes<1>;
                    if v411 != 0.0 {
                        let v412 = v394.exp();
                        let v1390 = v1378 * v412;
                        v419 = v412;
                        v928 = v1390;
                    } else {
                        let v413 = v394.exp();
                        let v414 = v413 + v19;
                        let v415 = v414.ln();
                        let v1389 = (v1378 * v413) * (v885 / v414);
                        v419 = v415;
                        v928 = v1389;
                    }
                    v418 = v419;
                    v927 = v928;
                }
                let v421 = v401 - v19;
                let v1393 = v1015 * v421;
                let v424 = v30.abs();
                let v427 = v19 + (v269 * (v424.powf(v105)));
                let v428 = (v0 * (v416 - v418)) / v427;
                let v1407 = (((v970 * ((v1200 * (if v30 >= v1198 { 1.0 } else { 0.0 })) - v885)) * (v105 * (v424.powf((v105 - v885))))) * v269) * v428;
                let v429 = (v94 * v421) - v428;
                let v1411 = ((Lanes([v1393[0], 0.0, 0.0])) + (v1382 * v94)) - ((((v925 - (Lanes([v927[0], 0.0, 0.0]))) * v0) - (Lanes([0.0, v1407[0], v1407[1]]))) / v427);
                v452 = v429;
                v922 = v1411;
            } else {
                v452 = v0;
                v922 = v1299;
            }
            let v1412 = Lanes([v1143[0], v1143[1], 0.0]);
            let v1413 = Lanes([0.0, 0.0, v892[0]]);
            let v432 = -(v219 - v430);
            let v1415 = (v1412 - v1413) * v973;
            let v434 = v430 * v433;
            let v1416 = v892 * v433;
            let v437 = v435 * (ddt(3355, v430));
            let v1419 = (v892 * v1417) * v435;
            let v439 = v219.abs();
            let v441 = if v439 >= v440 { v439 } else { v440 };
            let v442 = (if v430 <= v219 { v430 } else { v219 }) / v441;
            let v1430 = ((v1143 * ((v1200 * (if v219 >= v1198 { 1.0 } else { 0.0 })) - v885)) * (if v439 >= v440 { 1.0 } else { 0.0 })) * v442;
            let v443 = v442.abs();
            let v1438 = v898 - v905;
            let v447 = (v444 - v445) / v49;
            let v1439 = v991 * v447;
            let v449 = v447 + v448;
            let v1445 = (((Lanes([v1438[0], 0.0, v1438[1], v1438[2]])) - (Lanes([v1439[0], v1439[1], v1439[2], 0.0]))) / v49) + (Lanes([v908[0], 0.0, v908[1], v908[2]]));
            let v451 = v450 / v51;
            let v1446 = v992 * v451;
            let v453 = v451 + v452;
            let v1450 = ((v915 - (Lanes([v1446[0], 0.0, 0.0]))) / v51) + v922;
            let v457 = v63 * (v19 + (v30 * v454));
            let v1453 = v898 * v457;
            let v1454 = ((v970 * v454) * v63) * v444;
            let v1458 = v915 * v67;
            let v1462 = (v1143 * v59) * v973;
            let v1463 = v970 * v55;
            let v467 = v19 + (v465 * ((v444 * v457) + (v450 * v67)));
            let v468 = v467.abs();
            let v471 = v19 + (v468.powf(v469));
            let v1476 = ((Lanes([0.0, v1462[0], v1462[1]])) - (Lanes([v1463[0], v1463[1], 0.0]))) * v472;
            let v474 = (v472 * ((v19 - (v219 * v59)) - (v30 * v55))) / v471;
            let v1480 = ((Lanes([0.0, v1476[0], v1476[1], v1476[2]])) - (((((((Lanes([v1453[0], 0.0, v1453[1], v1453[2]])) + (Lanes([0.0, v1454[0], v1454[1], 0.0]))) + (Lanes([v1458[0], v1458[1], v1458[2], 0.0]))) * v465) * ((v1200 * (if v467 >= v1198 { 1.0 } else { 0.0 })) - v885)) * (v469 * (v468.powf((v469 - v885))))) * v474)) / v471;
            let v475 = v450 * v474;
            let v1481 = v915 * v474;
            let v1484 = (Lanes([v1481[0], v1481[1], v1481[2], 0.0])) + (v1480 * v450);
            let v476 = v444 * v474;
            let v1485 = v898 * v474;
            let v1488 = (Lanes([v1485[0], 0.0, v1485[1], v1485[2]])) + (v1480 * v444);
            let v1489 = v1488 * v443;
            let v1490 = ((((v1412 + ((v1413 - v1412) * (if v430 <= v219 { 1.0 } else { 0.0 }))) - (Lanes([v1430[0], v1430[1], 0.0]))) / v441) * ((v1200 * (if v442 >= v1198 { 1.0 } else { 0.0 })) - v885)) * v476;
            let v480 = v19 - v478;
            let v481 = v480 * v444;
            let v1496 = (v898 * v480) * v474;
            let v1499 = (Lanes([v1496[0], 0.0, v1496[1], v1496[2]])) + (v1480 * v481);
            let v483 = ((v476 * v443) * v478) + (v481 * v474);
            let v1501 = (((Lanes([v1489[0], v1489[1], v1489[2], v1489[3], 0.0])) + (Lanes([0.0, 0.0, v1490[0], v1490[1], v1490[2]]))) * v478) + (Lanes([v1499[0], v1499[1], v1499[2], v1499[3], 0.0]));
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
            let v1527 = (((v983 * v497) * v499) * v496) * v502;
            let v1528 = ((((v1151 / v484) * ((v1200 * (if v485 >= v1198 { 1.0 } else { 0.0 })) - v885)) * (v487 * (v486.powf((v487 - v885))))) * (v501 * (v489.powf((v501 - v885))))) * v500;
            let v1531 = (Lanes([0.0, v1527[0], 0.0])) + (Lanes([v1528[0], 0.0, v1528[1]]));
            let v507 = (v43 * v505).exp();
            let v508 = v504 * v507;
            let v1534 = ((v983 * v505) * v507) * v504;
            let v512 = (v43 * v510).exp();
            let v513 = v509 * v512;
            let v514 = v19 / v493;
            let v515 = v495.powf(v514);
            let v516 = v513 * v515;
            let v1542 = (((v983 * v510) * v512) * v509) * v515;
            let v1543 = ((((v1155 / v490) * ((v1200 * (if v491 >= v1198 { 1.0 } else { 0.0 })) - v885)) * (v493 * (v492.powf((v493 - v885))))) * (v514 * (v495.powf((v514 - v885))))) * v513;
            let v1546 = (Lanes([0.0, v1542[0], 0.0])) + (Lanes([v1543[0], 0.0, v1543[1]]));
            let v517 = v220 - v214;
            let v1549 = (Lanes([v891[0], 0.0])) - (Lanes([0.0, v889[0]]));
            let v519 = v517 / v518;
            let v520 = v519.abs();
            let v523 = v19 + (v520.powf(v521));
            let v524 = v19 / v521;
            let v531 = v527 * (v19 + (v528 * ((v523.powf(v524)) - v19)));
            let v1564 = (((((v1549 / v518) * ((v1200 * (if v519 >= v1198 { 1.0 } else { 0.0 })) - v885)) * (v521 * (v520.powf((v521 - v885))))) * (v524 * (v523.powf((v524 - v885))))) * v528) * v527;
            let v532 = v531 * v444;
            let v1565 = v1564 * v444;
            let v1566 = v898 * v531;
            let v1569 = (Lanes([v1565[0], v1565[1], 0.0, 0.0, 0.0])) + (Lanes([0.0, 0.0, v1566[0], v1566[1], v1566[2]]));
            let v534 = v533 * v475;
            let v1570 = v1484 * v533;
            let v536 = if v535 == v19 { 1.0 } else { 0.0 };
            let v553: f64;
            let v846: f64;
            let v847: f64;
            let v848: f64;
            let v849: f64;
            let v929: Lanes<4>;
            let v930: Lanes<6>;
            let v931: Lanes<1>;
            let v932: Lanes<3>;
            if v536 != 0.0 {
                let v537 = v444 / v49;
                let v1575 = v991 * v537;
                let v538 = -v537;
                let v539 = v538 * v531;
                let v1581 = ((((Lanes([v898[0], 0.0, v898[1], v898[2]])) - (Lanes([v1575[0], v1575[1], v1575[2], 0.0]))) / v49) * v973) * v531;
                let v1582 = v1564 * v538;
                let v1585 = (Lanes([0.0, 0.0, v1581[0], v1581[1], v1581[2], v1581[3]])) + (Lanes([v1582[0], v1582[1], 0.0, 0.0, 0.0, 0.0]));
                let v541 = ddt(3525, v540);
                let v542 = v531 * v541;
                let v1587 = v1564 * v541;
                let v1588 = (v893 * v1417) * v531;
                let v1591 = (Lanes([v1587[0], v1587[1], 0.0])) + (Lanes([0.0, 0.0, v1588[0]]));
                let v545 = (v540.abs()) / v544;
                let v548 = v19 + (v545.powf(v546));
                let v549 = v503 / v548;
                let v1601 = (((v893 * ((v1200 * (if v540 >= v1198 { 1.0 } else { 0.0 })) - v885)) / v544) * (v546 * (v545.powf((v546 - v885))))) * v549;
                let v1605 = ((Lanes([v1531[0], v1531[1], v1531[2], 0.0])) - (Lanes([0.0, 0.0, 0.0, v1601[0]]))) / v548;
                v553 = v549;
                v846 = v539;
                v847 = v540;
                v848 = v542;
                v849 = v0;
                v929 = v1605;
                v930 = v1585;
                v931 = v893;
                v932 = v1591;
            } else {
                let v1571 = Lanes([v1531[0], v1531[1], v1531[2], 0.0]);
                v553 = v503;
                v846 = v0;
                v847 = v0;
                v848 = v0;
                v849 = v550;
                v929 = v1571;
                v930 = v1572;
                v931 = v1573;
                v932 = v1574;
            }
            let v552 = if v551 == v19 { 1.0 } else { 0.0 };
            let v771: f64;
            let v782: f64;
            let v794: f64;
            let v933: Lanes<4>;
            let v934: Lanes<3>;
            let v935: Lanes<1>;
            if v552 != 0.0 {
                let v555 = v553 + v554;
                let v557 = v508 + v556;
                let v559 = v516 + v558;
                v771 = v555;
                v782 = v559;
                v794 = v557;
                v933 = v929;
                v934 = v1546;
                v935 = v1534;
            } else {
                v771 = v553;
                v782 = v516;
                v794 = v508;
                v933 = v929;
                v934 = v1546;
                v935 = v1534;
            }
            let v560 = if v216 <= v0 { 1.0 } else { 0.0 };
            let v826: f64;
            let v936: Lanes<3>;
            if v560 != 0.0 {
                let v561 = v213 * v207;
                let v562 = v19 - v201;
                let v563 = v216 / v207;
                let v1623 = v1126 * v563;
                let v564 = v19 - v563;
                let v567 = (v562 * (v564.ln())).exp();
                let v568 = v19 - v567;
                let v1634 = ((v1135 * v207) + (v1126 * v213)) * v568;
                let v570 = (v561 * v568) / v562;
                let v1638 = ((Lanes([0.0, v1634[0], 0.0])) + (((((((((Lanes([v1139[0], 0.0, v1139[1]])) - (Lanes([0.0, v1623[0], 0.0]))) / v207) * v973) * (v885 / v564)) * v562) * v567) * v973) * v561)) / v562;
                v826 = v570;
                v936 = v1638;
            } else {
                let v571 = v213 * v216;
                let v1606 = v1135 * v216;
                let v1607 = v1139 * v213;
                let v573 = v572 * v201;
                let v1611 = v1139 * v573;
                let v575 = (v573 * v216) / v207;
                let v1612 = v1126 * v575;
                let v576 = v19 + v575;
                let v577 = v571 * v576;
                let v1619 = (((Lanes([0.0, v1606[0], 0.0])) + (Lanes([v1607[0], 0.0, v1607[1]]))) * v576) + ((((Lanes([v1611[0], 0.0, v1611[1]])) - (Lanes([0.0, v1612[0], 0.0]))) / v207) * v571);
                v826 = v577;
                v936 = v1619;
            }
            let v1640 = (v1063 * v973) * v579;
            let v581 = v219 + ((-v159) * v579);
            let v1641 = Lanes([0.0, v1143[0], v1143[1]]);
            let v1643 = v1641 + (Lanes([v1640[0], 0.0, 0.0]));
            let v582 = if v581 > v0 { 1.0 } else { 0.0 };
            let v610: f64;
            let v611: f64;
            let v937: Lanes<3>;
            let v938: Lanes<3>;
            if v582 != 0.0 {
                let v585 = v19 - v579;
                let v588 = ((v583 - v150) * (v585.ln())).exp();
                let v591 = v19 - ((v588 * v585) * v585);
                let v593 = v19 - v150;
                let v594 = (v159 * v591) / v593;
                let v1660 = (v1063 * v591) / v593;
                let v595 = v572 * v150;
                let v597 = (v595 * v581) / v159;
                let v1662 = v1063 * v597;
                let v598 = v585 + v597;
                let v600 = (v581 * v598) * v588;
                let v1669 = ((v1643 * v598) + ((((v1643 * v595) - (Lanes([v1662[0], 0.0, 0.0]))) / v159) * v581)) * v588;
                let v1670 = Lanes([v1660[0], 0.0, 0.0]);
                v610 = v594;
                v611 = v600;
                v937 = v1670;
                v938 = v1669;
            } else {
                let v601 = v19 - v150;
                let v602 = v219 / v159;
                let v1644 = v1063 * v602;
                let v603 = v19 - v602;
                let v606 = (v601 * (v603.ln())).exp();
                let v607 = v19 - v606;
                let v1654 = v1063 * v607;
                let v609 = (v159 * v607) / v601;
                let v1658 = ((Lanes([v1654[0], 0.0, 0.0])) + ((((((((v1641 - (Lanes([v1644[0], 0.0, 0.0]))) / v159) * v973) * (v885 / v603)) * v601) * v606) * v973) * v159)) / v601;
                v610 = v609;
                v611 = v0;
                v937 = v1658;
                v938 = v1156;
            }
            let v612 = v610 + v611;
            let v613 = v167 * v612;
            let v1672 = v1073 * v612;
            let v1675 = (Lanes([v1672[0], 0.0, 0.0])) + ((v937 + v938) * v167);
            let v615 = (-v184) * v579;
            let v1677 = (v1095 * v973) * v579;
            let v616 = v222 + v615;
            let v1678 = Lanes([v1147[0], 0.0, v1147[1]]);
            let v1680 = v1678 + (Lanes([0.0, v1677[0], 0.0]));
            let v617 = if v616 > v0 { 1.0 } else { 0.0 };
            let v645: f64;
            let v646: f64;
            let v939: Lanes<3>;
            let v940: Lanes<3>;
            if v617 != 0.0 {
                let v620 = v19 - v579;
                let v623 = ((v618 - v178) * (v620.ln())).exp();
                let v626 = v19 - ((v623 * v620) * v620);
                let v628 = v19 - v178;
                let v629 = (v184 * v626) / v628;
                let v1698 = (v1095 * v626) / v628;
                let v630 = v572 * v178;
                let v632 = (v630 * v616) / v184;
                let v1700 = v1095 * v632;
                let v633 = v620 + v632;
                let v635 = (v616 * v633) * v623;
                let v1707 = ((v1680 * v633) + ((((v1680 * v630) - (Lanes([0.0, v1700[0], 0.0]))) / v184) * v616)) * v623;
                let v1708 = Lanes([0.0, v1698[0], 0.0]);
                v645 = v629;
                v646 = v635;
                v939 = v1708;
                v940 = v1707;
            } else {
                let v636 = v19 - v178;
                let v637 = v222 / v184;
                let v1681 = v1095 * v637;
                let v638 = v19 - v637;
                let v641 = (v636 * (v638.ln())).exp();
                let v642 = v19 - v641;
                let v1691 = v1095 * v642;
                let v644 = (v184 * v642) / v636;
                let v1695 = ((Lanes([0.0, v1691[0], 0.0])) + ((((((((v1678 - (Lanes([0.0, v1681[0], 0.0]))) / v184) * v973) * (v885 / v638)) * v636) * v641) * v973) * v184)) / v636;
                v645 = v644;
                v646 = v0;
                v939 = v1695;
                v940 = v1696;
            }
            let v647 = v645 + v646;
            let v1710 = v1104 * v647;
            let v650 = v19 - v649;
            let v651 = v650 * (v190 * v647);
            let v1714 = ((Lanes([0.0, v1710[0], 0.0])) + ((v939 + v940) * v190)) * v650;
            let v652 = v30 + v615;
            let v1715 = Lanes([0.0, v970[0], v970[1]]);
            let v1717 = v1715 + (Lanes([v1677[0], 0.0, 0.0]));
            let v653 = if v652 > v0 { 1.0 } else { 0.0 };
            let v681: f64;
            let v682: f64;
            let v941: Lanes<3>;
            let v942: Lanes<3>;
            if v653 != 0.0 {
                let v656 = v19 - v579;
                let v659 = ((v654 - v178) * (v656.ln())).exp();
                let v662 = v19 - ((v659 * v656) * v656);
                let v664 = v19 - v178;
                let v665 = (v184 * v662) / v664;
                let v1734 = (v1095 * v662) / v664;
                let v666 = v572 * v178;
                let v668 = (v666 * v652) / v184;
                let v1736 = v1095 * v668;
                let v669 = v656 + v668;
                let v671 = (v652 * v669) * v659;
                let v1743 = ((v1717 * v669) + ((((v1717 * v666) - (Lanes([v1736[0], 0.0, 0.0]))) / v184) * v652)) * v659;
                let v1744 = Lanes([v1734[0], 0.0, 0.0]);
                v681 = v665;
                v682 = v671;
                v941 = v1744;
                v942 = v1743;
            } else {
                let v672 = v19 - v178;
                let v673 = v30 / v184;
                let v1718 = v1095 * v673;
                let v674 = v19 - v673;
                let v677 = (v672 * (v674.ln())).exp();
                let v678 = v19 - v677;
                let v1728 = v1095 * v678;
                let v680 = (v184 * v678) / v672;
                let v1732 = ((Lanes([v1728[0], 0.0, 0.0])) + ((((((((v1715 - (Lanes([v1718[0], 0.0, 0.0]))) / v184) * v973) * (v885 / v674)) * v672) * v677) * v973) * v184)) / v672;
                v681 = v680;
                v682 = v0;
                v941 = v1732;
                v942 = v1299;
            }
            let v683 = v681 + v682;
            let v1746 = v1104 * v683;
            let v685 = v649 * (v190 * v683);
            let v1750 = ((Lanes([v1746[0], 0.0, 0.0])) + ((v941 + v942) * v190)) * v649;
            let v689 = if (if v686 != v0 { 1.0 } else { 0.0 }) != 0.0 && (if v527 != v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v830: f64;
            let v943: Lanes<4>;
            if v689 != 0.0 {
                let v695 = (((v26 * v686) * v691) / v693) * v527;
                let v696 = v695 * v476;
                let v1752 = v1488 * v695;
                v830 = v696;
                v943 = v1752;
            } else {
                v830 = v0;
                v943 = v1751;
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
            let v944: Lanes<7>;
            let v945: Lanes<1>;
            let v946: Lanes<1>;
            let v947: Lanes<7>;
            let v948: Lanes<2>;
            let v949: Lanes<1>;
            let v950: Lanes<1>;
            let v951: Lanes<1>;
            let v952: Lanes<7>;
            if v701 != 0.0 {
                let v703 = v449 * v517;
                let v1815 = v1445 * v517;
                let v1816 = v1549 * v449;
                let v1824 = (((Lanes([0.0, 0.0, v1815[0], v1815[1], v1815[2], v1815[3]])) + (Lanes([v1816[0], v1816[1], 0.0, 0.0, 0.0, 0.0]))) * ((v1200 * (if v703 >= v1198 { 1.0 } else { 0.0 })) - v885)) * v702;
                let v707 = v220 - v706;
                let v708 = v453 * v707;
                let v1828 = v1450 * v707;
                let v1829 = ((Lanes([0.0, v891[0]])) - (Lanes([v894[0], 0.0]))) * v453;
                let v1836 = ((Lanes([0.0, 0.0, v1828[0], v1828[1], v1828[2]])) + (Lanes([v1829[0], v1829[1], 0.0, 0.0, 0.0]))) * ((v1200 * (if v708 >= v1198 { 1.0 } else { 0.0 })) - v885);
                let v710 = (v702 * (v703.abs())) - (v708.abs());
                let v1839 = (Lanes([0.0, v1824[0], v1824[1], v1824[2], v1824[3], v1824[4], v1824[5]])) - (Lanes([v1836[0], v1836[1], 0.0, v1836[2], v1836[3], v1836[4], 0.0]));
                let v711 = v2 / v699;
                let v1840 = v886 / v699;
                let v714 = ddt(3871, (v2 * v712));
                let v1842 = (v886 * v712) * v1417;
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
                v944 = v1839;
                v945 = v1840;
                v946 = v1842;
                v947 = v1753;
                v948 = v1779;
                v949 = v966;
                v950 = v1780;
                v951 = v1780;
                v952 = v1753;
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
                let v953: Lanes<7>;
                let v954: Lanes<2>;
                let v955: Lanes<1>;
                let v956: Lanes<1>;
                let v957: Lanes<1>;
                let v958: Lanes<7>;
                if v720 != 0.0 {
                    let v722 = v449 * v517;
                    let v1781 = v1445 * v517;
                    let v1782 = v1549 * v449;
                    let v1790 = (((Lanes([0.0, 0.0, v1781[0], v1781[1], v1781[2], v1781[3]])) + (Lanes([v1782[0], v1782[1], 0.0, 0.0, 0.0, 0.0]))) * ((v1200 * (if v722 >= v1198 { 1.0 } else { 0.0 })) - v885)) * v721;
                    let v725 = v220 - v706;
                    let v726 = v453 * v725;
                    let v1794 = v1450 * v725;
                    let v1795 = ((Lanes([0.0, v891[0]])) - (Lanes([v894[0], 0.0]))) * v453;
                    let v1802 = ((Lanes([0.0, 0.0, v1794[0], v1794[1], v1794[2]])) + (Lanes([v1795[0], v1795[1], 0.0, 0.0, 0.0]))) * ((v1200 * (if v726 >= v1198 { 1.0 } else { 0.0 })) - v885);
                    let v728 = (v721 * (v722.abs())) - (v726.abs());
                    let v1805 = (Lanes([0.0, v1790[0], v1790[1], v1790[2], v1790[3], v1790[4], v1790[5]])) - (Lanes([v1802[0], v1802[1], 0.0, v1802[2], v1802[3], v1802[4], 0.0]));
                    let v731 = (v2 - v729) / v699;
                    let v1809 = ((Lanes([v886[0], 0.0])) - (Lanes([0.0, v895[0]]))) / v699;
                    let v733 = ddt(3902, (v712 * v2));
                    let v1811 = (v886 * v712) * v1417;
                    let v734 = v729 / v718;
                    let v1812 = v895 / v718;
                    let v737 = ddt(3909, (v735 * v729));
                    let v1814 = (v895 * v735) * v1417;
                    v855 = v728;
                    v857 = v731;
                    v859 = v733;
                    v861 = v734;
                    v863 = v737;
                    v865 = v0;
                    v868 = v0;
                    v871 = v0;
                    v874 = v0;
                    v953 = v1805;
                    v954 = v1809;
                    v955 = v1811;
                    v956 = v1812;
                    v957 = v1814;
                    v958 = v1753;
                } else {
                    let v739 = if v697 == v738 { 1.0 } else { 0.0 };
                    let v866: f64;
                    let v869: f64;
                    let v872: f64;
                    let v875: f64;
                    let v959: Lanes<7>;
                    if v739 != 0.0 {
                        let v741 = v449 * v517;
                        let v1754 = v1445 * v517;
                        let v1755 = v1549 * v449;
                        let v1763 = (((Lanes([0.0, 0.0, v1754[0], v1754[1], v1754[2], v1754[3]])) + (Lanes([v1755[0], v1755[1], 0.0, 0.0, 0.0, 0.0]))) * ((v1200 * (if v741 >= v1198 { 1.0 } else { 0.0 })) - v885)) * v740;
                        let v744 = v220 - v706;
                        let v745 = v453 * v744;
                        let v1767 = v1450 * v744;
                        let v1768 = ((Lanes([0.0, v891[0]])) - (Lanes([v894[0], 0.0]))) * v453;
                        let v1775 = ((Lanes([0.0, 0.0, v1767[0], v1767[1], v1767[2]])) + (Lanes([v1768[0], v1768[1], 0.0, 0.0, 0.0]))) * ((v1200 * (if v745 >= v1198 { 1.0 } else { 0.0 })) - v885);
                        let v747 = (v740 * (v741.abs())) - (v745.abs());
                        let v1778 = (Lanes([0.0, v1763[0], v1763[1], v1763[2], v1763[3], v1763[4], v1763[5]])) - (Lanes([v1775[0], v1775[1], 0.0, v1775[2], v1775[3], v1775[4], 0.0]));
                        v866 = v747;
                        v869 = v748;
                        v872 = v0;
                        v875 = v0;
                        v959 = v1778;
                    } else {
                        v866 = v0;
                        v869 = v0;
                        v872 = v749;
                        v875 = v750;
                        v959 = v1753;
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
                    v953 = v1753;
                    v954 = v1779;
                    v955 = v966;
                    v956 = v1780;
                    v957 = v1780;
                    v958 = v959;
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
                v944 = v1753;
                v945 = v966;
                v946 = v966;
                v947 = v953;
                v948 = v954;
                v949 = v955;
                v950 = v956;
                v951 = v957;
                v952 = v958;
            }
            let v751 = ctx.simparam_or("gmin", v0);
            let v752 = v751 * v218;
            let v1843 = v1142 * v751;
            let v753 = ctx.simparam_or("gmin", v0);
            let v754 = v753 * v29;
            let v1844 = v969 * v753;
            let v755 = ctx.simparam_or("gmin", v0);
            let v757 = v755 * (v28 - v217);
            let v1848 = ((Lanes([v888[0], 0.0])) - (Lanes([0.0, v890[0]]))) * v755;
            let v760 = (v496 + (v551 * v554)) / v25;
            let v763 = (v509 + (v551 * v558)) / v25;
            let v766 = (v504 + (v551 * v556)) / v25;
            let v770 = if (if v760 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v760 >= v768 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v876: f64;
            let v877: f64;
            let v878: f64;
            let v960: Lanes<4>;
            if v770 != 0.0 {
                let v772 = v771 / v25;
                let v1850 = v933 / v25;
                let v773 = if v772 > v768 { 1.0 } else { 0.0 };
                let v774: f64;
                let v961: Lanes<4>;
                if v773 != 0.0 {
                    v774 = v772;
                    v961 = v1850;
                } else {
                    v774 = v768;
                    v961 = v1849;
                }
                let v775 = v223 / v774;
                let v1854 = ((Lanes([v1150[0], 0.0, v1150[1], 0.0])) - (v961 * v775)) / v774;
                let v776 = if v772 >= v768 { 1.0 } else { 0.0 };
                if v776 != 0.0 {
                } else {
                }
                v876 = v775;
                v877 = v777;
                v878 = v0;
                v960 = v1854;
            } else {
                v876 = v0;
                v877 = v0;
                v878 = v778;
                v960 = v1849;
            }
            let v781 = if (if v763 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v763 >= v768 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v879: f64;
            let v880: f64;
            let v881: f64;
            let v962: Lanes<3>;
            if v781 != 0.0 {
                let v783 = v782 / v25;
                let v1856 = v934 / v25;
                let v784 = if v783 > v768 { 1.0 } else { 0.0 };
                let v785: f64;
                let v963: Lanes<3>;
                if v784 != 0.0 {
                    v785 = v783;
                    v963 = v1856;
                } else {
                    v785 = v768;
                    v963 = v1855;
                }
                let v786 = v225 / v785;
                let v1860 = ((Lanes([v1154[0], 0.0, v1154[1]])) - (v963 * v786)) / v785;
                let v787 = if v783 >= v768 { 1.0 } else { 0.0 };
                if v787 != 0.0 {
                } else {
                }
                v879 = v786;
                v880 = v788;
                v881 = v0;
                v962 = v1860;
            } else {
                v879 = v0;
                v880 = v0;
                v881 = v789;
                v962 = v1855;
            }
            let v792 = if (if v766 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v766 >= v768 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v882: f64;
            let v883: f64;
            let v884: f64;
            let v964: Lanes<3>;
            if v792 != 0.0 {
                let v793 = v706 - v28;
                let v1864 = (Lanes([v894[0], 0.0])) - (Lanes([0.0, v888[0]]));
                let v795 = v794 / v25;
                let v1865 = v935 / v25;
                let v796 = if v795 > v768 { 1.0 } else { 0.0 };
                let v797: f64;
                let v965: Lanes<1>;
                if v796 != 0.0 {
                    v797 = v795;
                    v965 = v1865;
                } else {
                    v797 = v768;
                    v965 = v966;
                }
                let v798 = v793 / v797;
                let v1866 = v965 * v798;
                let v1870 = ((Lanes([v1864[0], 0.0, v1864[1]])) - (Lanes([0.0, v1866[0], 0.0]))) / v797;
                let v799 = if v795 >= v768 { 1.0 } else { 0.0 };
                if v799 != 0.0 {
                } else {
                }
                v882 = v798;
                v883 = v800;
                v884 = v0;
                v964 = v1870;
            } else {
                v882 = v0;
                v883 = v0;
                v884 = v801;
                v964 = v1861;
            }
            let v802 = v26 * v449;
            let v803 = v802 * v25;
            let v1872 = (v1445 * v26) * v25;
            let v805 = (v26 * v453) * v25;
            let v1874 = (v1450 * v26) * v25;
            let v808 = v26 * ((-v475) * v25);
            let v1877 = ((v1484 * v973) * v25) * v26;
            let v810 = (v26 * v483) * v25;
            let v1879 = (v1501 * v26) * v25;
            let v812 = (v26 * v613) * v25;
            let v1881 = (v1675 * v26) * v25;
            let v813 = ddt(4101, v812);
            let v1882 = v1881 * v1417;
            let v815 = (v26 * v532) * v25;
            let v1884 = (v1569 * v26) * v25;
            let v816 = ddt(4107, v815);
            let v1885 = v1884 * v1417;
            let v818 = (v26 * v651) * v25;
            let v1887 = (v1714 * v26) * v25;
            let v819 = ddt(4113, v818);
            let v1888 = v1887 * v1417;
            let v821 = (v26 * v685) * v25;
            let v1890 = (v1750 * v26) * v25;
            let v822 = ddt(4119, v821);
            let v1891 = v1890 * v1417;
            let v824 = (v26 * v534) * v25;
            let v1893 = (v1570 * v26) * v25;
            let v825 = ddt(4125, v824);
            let v1894 = v1893 * v1417;
            let v828 = (v26 * v826) * v25;
            let v1896 = (v936 * v26) * v25;
            let v829 = ddt(4131, v828);
            let v1897 = v1896 * v1417;
            let v832 = (-v830) * v25;
            let v1899 = (v943 * v973) * v25;
            let v833 = ddt(4136, v832);
            let v1900 = v1899 * v1417;
            let v834 = v830 * v25;
            let v1901 = v943 * v25;
            let v835 = ddt(4140, v834);
            let v1902 = v1901 * v1417;
            let v841 = if (if (if v836 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v838 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) > v0 { 1.0 } else { 0.0 };
            if v841 != 0.0 {
            } else {
            }
            let v843 = if v802 >= v0 { 1.0 } else { 0.0 };
            if v843 != 0.0 {
            } else {
            }
            let v1903 = v1415[0];
            let v1904 = v1415[1];
            let v1905 = v1415[2];
            let v1906 = v1416[0];
            let v1907 = v1419[0];
            let v1908 = v930[0];
            let v1909 = v930[1];
            let v1910 = v930[2];
            let v1911 = v930[3];
            let v1912 = v930[4];
            let v1913 = v930[5];
            let v1914 = v931[0];
            let v1915 = v932[0];
            let v1916 = v932[1];
            let v1917 = v932[2];
            let v1918 = v944[0];
            let v1919 = v944[1];
            let v1920 = v944[2];
            let v1921 = v944[3];
            let v1922 = v944[4];
            let v1923 = v944[5];
            let v1924 = v944[6];
            let v1925 = v945[0];
            let v1926 = v946[0];
            let v1927 = v947[0];
            let v1928 = v947[1];
            let v1929 = v947[2];
            let v1930 = v947[3];
            let v1931 = v947[4];
            let v1932 = v947[5];
            let v1933 = v947[6];
            let v1934 = v948[0];
            let v1935 = v948[1];
            let v1936 = v949[0];
            let v1937 = v950[0];
            let v1938 = v951[0];
            let v1939 = v952[0];
            let v1940 = v952[1];
            let v1941 = v952[2];
            let v1942 = v952[3];
            let v1943 = v952[4];
            let v1944 = v952[5];
            let v1945 = v952[6];
            let v1946 = v1843[0];
            let v1947 = v1843[1];
            let v1948 = v1844[0];
            let v1949 = v1844[1];
            let v1950 = v1848[0];
            let v1951 = v1848[1];
            let v1952 = v960[0];
            let v1953 = v960[1];
            let v1954 = v960[2];
            let v1955 = v960[3];
            let v1956 = v962[0];
            let v1957 = v962[1];
            let v1958 = v962[2];
            let v1959 = v964[0];
            let v1960 = v964[1];
            let v1961 = v964[2];
            let v1962 = v1872[0];
            let v1963 = v1872[1];
            let v1964 = v1872[2];
            let v1965 = v1872[3];
            let v1966 = v1874[0];
            let v1967 = v1874[1];
            let v1968 = v1874[2];
            let v1969 = v1877[0];
            let v1970 = v1877[1];
            let v1971 = v1877[2];
            let v1972 = v1877[3];
            let v1973 = v1879[0];
            let v1974 = v1879[1];
            let v1975 = v1879[2];
            let v1976 = v1879[3];
            let v1977 = v1879[4];
            let v1978 = v1882[0];
            let v1979 = v1882[1];
            let v1980 = v1882[2];
            let v1981 = v1885[0];
            let v1982 = v1885[1];
            let v1983 = v1885[2];
            let v1984 = v1885[3];
            let v1985 = v1885[4];
            let v1986 = v1888[0];
            let v1987 = v1888[1];
            let v1988 = v1888[2];
            let v1989 = v1891[0];
            let v1990 = v1891[1];
            let v1991 = v1891[2];
            let v1992 = v1894[0];
            let v1993 = v1894[1];
            let v1994 = v1894[2];
            let v1995 = v1894[3];
            let v1996 = v1897[0];
            let v1997 = v1897[1];
            let v1998 = v1897[2];
            let v1999 = v1900[0];
            let v2000 = v1900[1];
            let v2001 = v1900[2];
            let v2002 = v1900[3];
            let v2003 = v1902[0];
            let v2004 = v1902[1];
            let v2005 = v1902[2];
            let v2006 = v1902[3];
            let v2007 = v1881[0];
            let v2008 = v1881[1];
            let v2009 = v1881[2];
            let v2010 = v1884[0];
            let v2011 = v1884[1];
            let v2012 = v1884[2];
            let v2013 = v1884[3];
            let v2014 = v1884[4];
            let v2015 = v1887[0];
            let v2016 = v1887[1];
            let v2017 = v1887[2];
            let v2018 = v1890[0];
            let v2019 = v1890[1];
            let v2020 = v1890[2];
            let v2021 = v1893[0];
            let v2022 = v1893[1];
            let v2023 = v1893[2];
            let v2024 = v1893[3];
            let v2025 = v1896[0];
            let v2026 = v1896[1];
            let v2027 = v1896[2];
            let v2028 = v1899[0];
            let v2029 = v1899[1];
            let v2030 = v1899[2];
            let v2031 = v1899[3];
            let v2032 = v1901[0];
            let v2033 = v1901[1];
            let v2034 = v1901[2];
            let v2035 = v1901[3];
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(9),
            None,
            multiplicity * (v432),
            [5, 6, 9],
            [v1903, v1904, v1905],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(9),
            None,
            multiplicity * (v434),
            [9],
            [v1906],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(9),
            None,
            multiplicity * (v437),
            [9],
            [v1907],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            None,
            multiplicity * (v846),
            [1, 2, 3, 4, 5, 6],
            [v1908, v1909, v1910, v1911, v1912, v1913],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(8),
            None,
            multiplicity * (v847),
            [8],
            [v1914],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(8),
            None,
            multiplicity * (v848),
            [1, 2, 8],
            [v1915, v1916, v1917],
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
            [v1918, v1919, v1920, v1921, v1922, v1923, v1924],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (v851),
            [3],
            [v1925],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (v852),
            [3],
            [v1926],
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
            [v1927, v1928, v1929, v1930, v1931, v1932, v1933],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(3),
            Some(7),
            multiplicity * (v856),
            [3, 7],
            [v1934, v1935],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (v858),
            [3],
            [v1936],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(7),
            None,
            multiplicity * (v860),
            [7],
            [v1937],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(7),
            None,
            multiplicity * (v862),
            [7],
            [v1938],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(3),
            None,
            multiplicity * (v864),
            [0, 1, 2, 3, 4, 5, 6],
            [v1939, v1940, v1941, v1942, v1943, v1944, v1945],
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
            [v1946, v1947],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(5),
            Some(4),
            multiplicity * (v754),
            [4, 5],
            [v1948, v1949],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(4),
            Some(6),
            multiplicity * (v757),
            [4, 6],
            [v1950, v1951],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(5),
            multiplicity * (v876),
            [1, 3, 5, 8],
            [v1952, v1953, v1954, v1955],
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
            [v1956, v1957, v1958],
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
            [v1959, v1960, v1961],
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
            [v1962, v1963, v1964, v1965],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(4),
            multiplicity * (v805),
            [3, 4, 5],
            [v1966, v1967, v1968],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            Some(6),
            multiplicity * (v808),
            [3, 4, 5, 6],
            [v1969, v1970, v1971, v1972],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(6),
            multiplicity * (v810),
            [3, 4, 5, 6, 9],
            [v1973, v1974, v1975, v1976, v1977],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(6),
            multiplicity * (v813),
            [3, 5, 6],
            [v1978, v1979, v1980],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * (v816),
            [1, 2, 3, 5, 6],
            [v1981, v1982, v1983, v1984, v1985],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(4),
            multiplicity * (v819),
            [1, 3, 4],
            [v1986, v1987, v1988],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(4),
            multiplicity * (v822),
            [3, 4, 5],
            [v1989, v1990, v1991],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (v825),
            [3, 4, 5, 6],
            [v1992, v1993, v1994, v1995],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(2),
            Some(4),
            multiplicity * (v829),
            [2, 3, 4],
            [v1996, v1997, v1998],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (v833),
            [3, 4, 5, 6],
            [v1999, v2000, v2001, v2002],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (v835),
            [3, 4, 5, 6],
            [v2003, v2004, v2005, v2006],
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
        self.canonical_reactive[9] = v852;
        self.canonical_reactive[10] = v853;
        self.canonical_reactive[11] = v854;
        self.canonical_reactive[12] = v856;
        self.canonical_reactive[13] = v858;
        self.canonical_reactive[14] = v860;
        self.canonical_reactive[15] = v862;
        self.canonical_reactive[16] = v864;
        self.canonical_reactive[17] = v867;
        self.canonical_reactive[18] = v870;
        self.canonical_reactive[19] = v873;
        self.canonical_reactive[20] = v752;
        self.canonical_reactive[21] = v754;
        self.canonical_reactive[22] = v757;
        self.canonical_reactive[23] = v876;
        self.canonical_reactive[24] = v877;
        self.canonical_reactive[25] = v878;
        self.canonical_reactive[26] = v879;
        self.canonical_reactive[27] = v880;
        self.canonical_reactive[28] = v881;
        self.canonical_reactive[29] = v882;
        self.canonical_reactive[30] = v883;
        self.canonical_reactive[31] = v884;
        self.canonical_reactive[32] = v803;
        self.canonical_reactive[33] = v805;
        self.canonical_reactive[34] = v808;
        self.canonical_reactive[35] = v810;
        self.canonical_reactive[36] = v812;
        self.canonical_reactive[37] = v2007;
        self.canonical_reactive[38] = v2008;
        self.canonical_reactive[39] = v2009;
        self.canonical_reactive[40] = v815;
        self.canonical_reactive[41] = v2010;
        self.canonical_reactive[42] = v2011;
        self.canonical_reactive[43] = v2012;
        self.canonical_reactive[44] = v2013;
        self.canonical_reactive[45] = v2014;
        self.canonical_reactive[46] = v818;
        self.canonical_reactive[47] = v2015;
        self.canonical_reactive[48] = v2016;
        self.canonical_reactive[49] = v2017;
        self.canonical_reactive[50] = v821;
        self.canonical_reactive[51] = v2018;
        self.canonical_reactive[52] = v2019;
        self.canonical_reactive[53] = v2020;
        self.canonical_reactive[54] = v824;
        self.canonical_reactive[55] = v2021;
        self.canonical_reactive[56] = v2022;
        self.canonical_reactive[57] = v2023;
        self.canonical_reactive[58] = v2024;
        self.canonical_reactive[59] = v828;
        self.canonical_reactive[60] = v2025;
        self.canonical_reactive[61] = v2026;
        self.canonical_reactive[62] = v2027;
        self.canonical_reactive[63] = v832;
        self.canonical_reactive[64] = v2028;
        self.canonical_reactive[65] = v2029;
        self.canonical_reactive[66] = v2030;
        self.canonical_reactive[67] = v2031;
        self.canonical_reactive[68] = v834;
        self.canonical_reactive[69] = v2032;
        self.canonical_reactive[70] = v2033;
        self.canonical_reactive[71] = v2034;
        self.canonical_reactive[72] = v2035;
        self.canonical_reactive[73] = v842;
        self.canonical_reactive[74] = v844;
        self.canonical_reactive[75] = v845;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(6),
            &[3, 5, 6],
            &[cached[37], cached[38], cached[39]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(6),
            &[1, 2, 3, 5, 6],
            &[cached[41], cached[42], cached[43], cached[44], cached[45]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(4),
            &[1, 3, 4],
            &[cached[47], cached[48], cached[49]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(4),
            &[3, 4, 5],
            &[cached[51], cached[52], cached[53]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(4),
            &[3, 4, 5, 6],
            &[cached[55], cached[56], cached[57], cached[58]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(2),
            Some(4),
            &[2, 3, 4],
            &[cached[60], cached[61], cached[62]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(6),
            &[3, 4, 5, 6],
            &[cached[64], cached[65], cached[66], cached[67]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(4),
            &[3, 4, 5, 6],
            &[cached[69], cached[70], cached[71], cached[72]],
            &[],
            &[],
            multiplicity,
        );
    }

}

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
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = 0usize;
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
            let v1 = 1.602e-19f64;
            let v2 = 8.9e0f64;
            let v3 = 8.85418e-12f64;
            let v4 = 7.8802202e-11f64;
            let v5 = 1e0f64;
            let v6 = 2e0f64;
            let v7 = 8.353992494899963e17f64;
            let v8 = 1.8e25f64;
            let v9 = parameters[34];
            let v11 = temperature;
            let v12 = node_potentials[4];
            let v14 = parameters[6];
            let v16 = 5.618214e-19f64;
            let v17 = 9.09e-4f64;
            let v20 = 8.3e2f64;
            let v25 = parameters[12];
            let v27 = 8.5e0f64;
            let v33 = 3.333333333333333e-1f64;
            let v34 = 5e-1f64;
            let v36 = parameters[22];
            let v37 = parameters[27];
            let v40 = 2.59e-2f64;
            let v42 = parameters[3];
            let v43 = parameters[0];
            let v45 = 6.434283176858164e24f64;
            let v47 = 3.204e-19f64;
            let v52 = parameters[11];
            let v54 = 1e-38f64;
            let v57 = parameters[5];
            let v59 = node_potentials[1];
            let v60 = node_potentials[6];
            let v63 = node_potentials[5];
            let v66 = parameters[31];
            let v68 = 8e1f64;
            let v77 = 6.931471805599453e-1f64;
            let v84 = 1e-1f64;
            let v85 = parameters[18];
            let v86 = parameters[19];
            let v88 = parameters[20];
            let v91 = parameters[21];
            let v99 = 6.250000000000001e-4f64;
            let v109 = parameters[7];
            let v111 = parameters[8];
            let v112 = parameters[9];
            let v120 = 2.52482255208e-29f64;
            let v126 = 3e0f64;
            let v127 = 1.4142135623730951e0f64;
            let v133 = 6e0f64;
            let v163 = 1e0f64;
            let v175 = parameters[13];
            let v178 = 6.666e-1f64;
            let v197 = 6.666666666666666e-1f64;
            let v199 = 3.333333333333333e-1f64;
            let v206 = 3.333333333333333e-1f64;
            let v208 = -6.666666666666666e-1f64;
            let v214 = 6.666666666666666e-1f64;
            let v216 = 3.333333333333333e-1f64;
            let v222 = 3.333333333333333e-1f64;
            let v224 = -6.666666666666666e-1f64;
            let v231 = 3.333333333333333e-1f64;
            let v233 = -6.666666666666666e-1f64;
            let v238 = 6.666666666666666e-1f64;
            let v240 = 3.333333333333333e-1f64;
            let v248 = 5e1f64;
            let v255 = 6.666666666666666e-1f64;
            let v257 = 3.333333333333333e-1f64;
            let v264 = 3.333333333333333e-1f64;
            let v266 = -6.666666666666666e-1f64;
            let v272 = 6.666666666666666e-1f64;
            let v274 = 3.333333333333333e-1f64;
            let v280 = 3.333333333333333e-1f64;
            let v282 = -6.666666666666666e-1f64;
            let v289 = 3.333333333333333e-1f64;
            let v291 = -6.666666666666666e-1f64;
            let v296 = 6.666666666666666e-1f64;
            let v298 = 3.333333333333333e-1f64;
            let v311 = 6.666666666666666e-1f64;
            let v313 = 3.333333333333333e-1f64;
            let v318 = 3.333333333333333e-1f64;
            let v320 = -6.666666666666666e-1f64;
            let v325 = 6.666666666666666e-1f64;
            let v327 = 3.333333333333333e-1f64;
            let v331 = 3.333333333333333e-1f64;
            let v333 = -6.666666666666666e-1f64;
            let v338 = 6.666666666666666e-1f64;
            let v340 = 3.333333333333333e-1f64;
            let v344 = 3.333333333333333e-1f64;
            let v346 = -6.666666666666666e-1f64;
            let v352 = 3.333333333333333e-1f64;
            let v354 = -6.666666666666666e-1f64;
            let v358 = 6.666666666666666e-1f64;
            let v360 = 3.333333333333333e-1f64;
            let v375 = 1e0f64;
            let v384 = parameters[14];
            let v389 = parameters[30];
            let v390 = parameters[23];
            let v391 = parameters[24];
            let v394 = parameters[26];
            let v395 = parameters[25];
            let v396 = parameters[15];
            let v397 = parameters[28];
            let v414 = 4e0f64;
            let v422 = parameters[17];
            let v448 = 4e0f64;
            let v453 = parameters[16];
            let v480 = parameters[32];
            let v487 = parameters[33];
            let v512 = 6.666666666666666e-1f64;
            let v514 = 3.333333333333333e-1f64;
            let v521 = 3.333333333333333e-1f64;
            let v523 = -6.666666666666666e-1f64;
            let v529 = 6.666666666666666e-1f64;
            let v531 = 3.333333333333333e-1f64;
            let v537 = 3.333333333333333e-1f64;
            let v539 = -6.666666666666666e-1f64;
            let v546 = 3.333333333333333e-1f64;
            let v548 = -6.666666666666666e-1f64;
            let v553 = 6.666666666666666e-1f64;
            let v555 = 3.333333333333333e-1f64;
            let v569 = 6.666666666666666e-1f64;
            let v571 = 3.333333333333333e-1f64;
            let v578 = 3.333333333333333e-1f64;
            let v580 = -6.666666666666666e-1f64;
            let v586 = 6.666666666666666e-1f64;
            let v588 = 3.333333333333333e-1f64;
            let v594 = 3.333333333333333e-1f64;
            let v596 = -6.666666666666666e-1f64;
            let v603 = 3.333333333333333e-1f64;
            let v605 = -6.666666666666666e-1f64;
            let v610 = 6.666666666666666e-1f64;
            let v612 = 3.333333333333333e-1f64;
            let v625 = 6.666666666666666e-1f64;
            let v627 = 3.333333333333333e-1f64;
            let v632 = 3.333333333333333e-1f64;
            let v634 = -6.666666666666666e-1f64;
            let v639 = 6.666666666666666e-1f64;
            let v641 = 3.333333333333333e-1f64;
            let v645 = 3.333333333333333e-1f64;
            let v647 = -6.666666666666666e-1f64;
            let v652 = 6.666666666666666e-1f64;
            let v654 = 3.333333333333333e-1f64;
            let v658 = 3.333333333333333e-1f64;
            let v660 = -6.666666666666666e-1f64;
            let v666 = 3.333333333333333e-1f64;
            let v668 = -6.666666666666666e-1f64;
            let v672 = 6.666666666666666e-1f64;
            let v674 = 3.333333333333333e-1f64;
            let v689 = 1e0f64;
            let v712 = 8e-1f64;
            let v715 = 1.2e0f64;
            let v734 = 2.5000000000000005e-3f64;
            let v756 = 5.625e-7f64;
            let v779 = parameters[4];
            let v791 = parameters[37];
            let v792 = parameters[39];
            let v793 = parameters[44];
            let v797 = parameters[45];
            let v800 = parameters[38];
            let v801 = parameters[46];
            let v806 = parameters[1];
            let v810 = parameters[2];
            let v815 = 9.6e-1f64;
            let v818 = parameters[47];
            let v823 = parameters[40];
            let v825 = parameters[43];
            let v829 = parameters[42];
            let v834 = parameters[41];
            let v850 = 0e0f64;
            let v851 = parameters[35];
            let v860 = parameters[36];
            let v864 = 1e9f64;
            let v870 = 1e0f64;
            let v871 = Lanes([1e0f64; 1]);
            let v872 = Lanes([1e0f64; 1]);
            let v873 = Lanes([1e0f64; 1]);
            let v874 = Lanes([1e0f64; 1]);
            let v895 = -1e0f64;
            let v939 = 2e0f64;
            let v1060 = -3.3340000000000003e-1f64;
            let v1098 = -6.666666666666667e-1f64;
            let v1108 = -1.6666666666666665e0f64;
            let v1121 = -6.666666666666667e-1f64;
            let v1131 = -1.6666666666666665e0f64;
            let v1144 = -6.666666666666667e-1f64;
            let v1154 = -1.6666666666666665e0f64;
            let v1167 = -1.6666666666666665e0f64;
            let v1177 = -6.666666666666667e-1f64;
            let v1207 = -6.666666666666667e-1f64;
            let v1220 = -1.6666666666666665e0f64;
            let v1234 = -6.666666666666667e-1f64;
            let v1247 = -1.6666666666666665e0f64;
            let v1263 = -1.6666666666666665e0f64;
            let v1274 = -6.666666666666667e-1f64;
            let v1300 = -6.666666666666667e-1f64;
            let v1313 = -1.6666666666666665e0f64;
            let v1327 = -6.666666666666667e-1f64;
            let v1340 = -1.6666666666666665e0f64;
            let v1356 = -1.6666666666666665e0f64;
            let v1367 = -6.666666666666667e-1f64;
            let v1581 = -6.666666666666667e-1f64;
            let v1591 = -1.6666666666666665e0f64;
            let v1604 = -6.666666666666667e-1f64;
            let v1614 = -1.6666666666666665e0f64;
            let v1627 = -6.666666666666667e-1f64;
            let v1637 = -1.6666666666666665e0f64;
            let v1650 = -1.6666666666666665e0f64;
            let v1660 = -6.666666666666667e-1f64;
            let v1690 = -6.666666666666667e-1f64;
            let v1703 = -1.6666666666666665e0f64;
            let v1717 = -6.666666666666667e-1f64;
            let v1730 = -1.6666666666666665e0f64;
            let v1746 = -1.6666666666666665e0f64;
            let v1757 = -6.666666666666667e-1f64;
            let v1783 = -6.666666666666667e-1f64;
            let v1796 = -1.6666666666666665e0f64;
            let v1810 = -6.666666666666667e-1f64;
            let v1823 = -1.6666666666666665e0f64;
            let v1839 = -1.6666666666666665e0f64;
            let v1850 = -6.666666666666667e-1f64;
            let v2065 = Lanes([0e0f64; 4]);
            let v2116 = Lanes([0e0f64; 1]);
            let v2133 = ddt_scale();
            let v10 = if v9 == v5 { 1.0 } else { 0.0 };
            if v10 != 0.0 {
            } else {
            }
            let v13 = v11 + v12;
            let v15 = v13 / v14;
            let v887 = v871 / v14;
            let v21 = v13 + v20;
            let v22 = (v17 * (v13 * v13)) / v21;
            let v24 = v16 - (v22 * v1);
            let v896 = (((((v871 * (v6 * v13)) * v17) - (v871 * v22)) / v21) * v1) * v895;
            let v31 = ((v27 * v25) + (v2 * (v5 - v25))) * v3;
            let v32 = if v9 != v5 { 1.0 } else { 0.0 };
            let v35: f64;
            if v32 != 0.0 {
                v35 = v33;
            } else {
                v35 = v34;
            }
            let v39 = v36 * (v15.powf(v37));
            let v901 = (v887 * (v37 * (v15.powf((v37 - v870))))) * v36;
            let v41 = v40 * v15;
            let v902 = v887 * v40;
            let v44 = v5 / v41;
            let v905 = ((v902 * v44) * v895) / v41;
            let v48 = v47 * v41;
            let v49 = (-v24) / v48;
            let v50 = v49.exp();
            let v51 = v45 * v50;
            let v53 = v52 / v51;
            let v55 = if v53 >= v54 { v53 } else { v54 };
            let v56 = v55.ln();
            let v919 = (((((((((v896 * v895) - ((v902 * v47) * v49)) / v48) * v50) * v45) * v53) * v895) / v51) * (if v53 >= v54 { 1.0 } else { 0.0 })) * (v870 / v55);
            let v58 = v31 / v57;
            let v61 = v59 - v60;
            let v922 = (Lanes([v872[0], 0.0])) - (Lanes([0.0, v873[0]]));
            let v62 = v60 - v60;
            let v923 = v873 - v873;
            let v64 = v63 - v60;
            let v926 = (Lanes([v874[0], 0.0])) - (Lanes([0.0, v873[0]]));
            let v65 = v64 - v62;
            let v927 = Lanes([0.0, v923[0]]);
            let v928 = v926 - v927;
            let v67 = v66 * v65;
            let v929 = v928 * v66;
            let v69 = if v67 > v68 { 1.0 } else { 0.0 };
            let v74: f64;
            let v875: Lanes<2>;
            if v69 != 0.0 {
                v74 = v67;
                v875 = v929;
            } else {
                let v70 = v67.exp();
                let v71 = v5 + v70;
                let v72 = v71.ln();
                let v932 = (v929 * v70) * (v870 / v71);
                v74 = v72;
                v875 = v932;
            }
            let v73 = v6 / v66;
            let v934 = (v875 * v73) - v928;
            let v79 = ((v73 * v74) - v65) - (v73 * v77);
            let v83 = -(v62 + (v34 * (v65 - v79)));
            let v938 = (v927 + ((v928 - v934) * v34)) * v895;
            let v943 = ((v934 * v88) - (v938 * v91)) / v58;
            let v95 = v5 + ((((v85 + v86) + (v88 * v79)) - (v91 * v83)) / v58);
            let v97 = v95 - v5;
            let v944 = v943 * v97;
            let v101 = ((v97 * v97) + v99).sqrt();
            let v103 = v34 * ((v95 + v5) + v101);
            let v104 = v103 * v41;
            let v951 = ((v943 + ((v944 + v944) * (v870 / (v939 * v101)))) * v34) * v41;
            let v952 = v902 * v103;
            let v955 = (Lanes([0.0, v951[0], v951[1]])) + (Lanes([v952[0], 0.0, 0.0]));
            let v105 = v5 / v104;
            let v958 = ((v955 * v105) * v895) / v104;
            let v106 = v64 * v105;
            let v959 = v926 * v105;
            let v962 = (Lanes([0.0, v959[0], v959[1]])) + (v958 * v64);
            let v963 = v922 * v105;
            let v964 = v958 * v61;
            let v108 = v62 * v105;
            let v968 = v923 * v105;
            let v971 = (Lanes([0.0, 0.0, v968[0]])) + (v958 * v62);
            let v972 = v958 * v109;
            let v115 = -(v111 + (v112 * v83));
            let v116 = v115 * v79;
            let v980 = ((((v938 * v112) * v895) * v79) + (v934 * v115)) * v105;
            let v983 = (Lanes([0.0, v980[0], v980[1]])) + (v958 * v116);
            let v119 = ((v61 * v105) - (v109 * v105)) - (v116 * v105);
            let v985 = (((Lanes([v963[0], 0.0, 0.0, v963[1]])) + (Lanes([0.0, v964[0], v964[1], v964[2]]))) - (Lanes([0.0, v972[0], v972[1], v972[2]]))) - (Lanes([0.0, v983[0], v983[1], v983[2]]));
            let v121 = v120 * v52;
            let v123 = (v121 * v44).sqrt();
            let v124 = v123 / v58;
            let v990 = ((v905 * v121) * (v870 / (v939 * v123))) / v58;
            let v993 = (v990 / v127) * v126;
            let v131 = (v34 * v119) - (v126 * (v5 + (v124 / v127)));
            let v995 = (v985 * v34) - (Lanes([0.0, v993[0], 0.0, 0.0]));
            let v996 = v995 * v131;
            let v136 = ((v131 * v131) + (v133 * v119)).sqrt();
            let v137 = v131 + v136;
            let v1003 = v995 + (((v996 + v996) + (v985 * v133)) * (v870 / (v939 * v136)));
            let v138 = if v119 < v0 { 1.0 } else { 0.0 };
            let v159: f64;
            let v876: Lanes<4>;
            if v138 != 0.0 {
                let v140 = (v119 - v137) / v124;
                let v1022 = v990 * v140;
                let v1027 = (((v985 - v1003) - (Lanes([0.0, v1022[0], 0.0, 0.0]))) / v124) * v140;
                let v143 = (v5 - v137) + (v140 * v140);
                let v144 = if v143 >= v54 { v143 } else { v54 };
                let v146 = -(v144.ln());
                let v1034 = ((((v1003 * v895) + (v1027 + v1027)) * (if v143 >= v54 { 1.0 } else { 0.0 })) * (v870 / v144)) * v895;
                v159 = v146;
                v876 = v1034;
            } else {
                let v147 = -v137;
                let v148 = rspice_limited_exp(v147);
                let v1006 = (v1003 * v895) * (rspice_limited_exp_derivative(v147));
                let v149 = v34 * v124;
                let v1007 = v990 * v34;
                let v1009 = v1007 * v149;
                let v1010 = v1009 + v1009;
                let v154 = (((v119 - v5) + v148) + (v149 * v149)).sqrt();
                let v155 = v154 - v149;
                let v1018 = ((((v985 + v1006) + (Lanes([0.0, v1010[0], 0.0, 0.0]))) * (v870 / (v939 * v154))) - (Lanes([0.0, v1007[0], 0.0, 0.0]))) * v155;
                let v158 = ((v155 * v155) + v5) - v148;
                let v1020 = (v1018 + v1018) - v1006;
                v159 = v158;
                v876 = v1020;
            }
            let v161 = v159 - v5;
            let v1035 = v876 * v161;
            let v165 = ((v161 * v161) + v163).sqrt();
            let v168 = (v34 * ((v159 + v5) + v165)).sqrt();
            let v1044 = ((v876 + ((v1035 + v1035) * (v870 / (v939 * v165)))) * v34) * (v870 / (v939 * v168));
            let v169 = v6 * v168;
            let v170 = v124 / v169;
            let v1047 = Lanes([0.0, v990[0], 0.0, 0.0]);
            let v1049 = (v1047 - ((v1044 * v6) * v170)) / v169;
            let v171 = v5 + v170;
            let v173 = (v6 * v171) * v58;
            let v174 = v173 * v41;
            let v1053 = v902 * v173;
            let v1055 = (((v1049 * v6) * v58) * v41) + (Lanes([0.0, v1053[0], 0.0, 0.0]));
            let v176 = v175 / v41;
            let v177 = v7 * v41;
            let v1059 = v902 * v7;
            let v179 = v177.powf(v178);
            let v180 = v176 * v179;
            let v1066 = ((((v902 * v176) * v895) / v41) * v179) + ((v1059 * (v178 * (v177.powf(v1060)))) * v176);
            let v183 = v1 * v41;
            let v184 = v24 / v183;
            let v1070 = (v896 - ((v902 * v1) * v184)) / v183;
            let v187 = (v159 - (((v52 / v8).ln()) + v184)) + v56;
            let v1073 = Lanes([0.0, v919[0], 0.0, 0.0]);
            let v1074 = (v876 - (Lanes([0.0, v1070[0], 0.0, 0.0]))) + v1073;
            let v188 = v187 - v108;
            let v1075 = Lanes([0.0, v971[0], v971[1], v971[2]]);
            let v1076 = v1074 - v1075;
            let v189 = v58 * v171;
            let v190 = v1 / v189;
            let v1082 = v1059 * v190;
            let v192 = (v190 * v177) / v41;
            let v1085 = v902 * v192;
            let v1088 = (((((((v1049 * v58) * v190) * v895) / v189) * v177) + (Lanes([0.0, v1082[0], 0.0, 0.0]))) - (Lanes([0.0, v1085[0], 0.0, 0.0]))) / v41;
            let v193 = if v188 <= v0 { 1.0 } else { 0.0 };
            let v368: f64;
            let v877: Lanes<4>;
            if v193 != 0.0 {
                let v194 = v188.exp();
                let v195 = v133 / v194;
                let v1297 = (((v1076 * v194) * v195) * v895) / v194;
                let v198 = v197 * v180;
                let v200 = v195.powf(v199);
                let v1304 = (v1066 * v197) * v200;
                let v203 = v5 + v188;
                let v207 = v206 * v180;
                let v209 = v195.powf(v208);
                let v1317 = (v1066 * v206) * v209;
                let v211 = (v203 + (v195.ln())) - (v207 * v209);
                let v212 = ((v195 + v192) + (v198 * v200)) / v211;
                let v1324 = (((v1297 + v1088) + ((Lanes([0.0, v1304[0], 0.0, 0.0])) + ((v1297 * (v199 * (v195.powf(v1300)))) * v198))) - (((v1076 + (v1297 * (v870 / v195))) - ((Lanes([0.0, v1317[0], 0.0, 0.0])) + ((v1297 * (v208 * (v195.powf(v1313)))) * v207))) * v212)) / v211;
                let v215 = v214 * v180;
                let v217 = v212.powf(v216);
                let v1331 = (v1066 * v214) * v217;
                let v223 = v222 * v180;
                let v225 = v212.powf(v224);
                let v1344 = (v1066 * v222) * v225;
                let v227 = (v203 + (v212.ln())) - (v223 * v225);
                let v228 = ((v212 + v192) + (v215 * v217)) / v227;
                let v1351 = (((v1324 + v1088) + ((Lanes([0.0, v1331[0], 0.0, 0.0])) + ((v1324 * (v216 * (v212.powf(v1327)))) * v215))) - (((v1076 + (v1324 * (v870 / v212))) - ((Lanes([0.0, v1344[0], 0.0, 0.0])) + ((v1324 * (v224 * (v212.powf(v1340)))) * v223))) * v228)) / v227;
                let v232 = v231 * v180;
                let v234 = v228.powf(v233);
                let v1360 = (v1066 * v231) * v234;
                let v239 = v238 * v180;
                let v241 = v228.powf(v240);
                let v1371 = (v1066 * v238) * v241;
                let v243 = (v228 + v192) + (v239 * v241);
                let v244 = ((v203 + (v228.ln())) - (v232 * v234)) / v243;
                let v245 = v1 * v177;
                let v1380 = (v1059 * v1) * v244;
                let v247 = (v245 * v244) / v174;
                let v1386 = (((Lanes([0.0, v1380[0], 0.0, 0.0])) + (((((v1076 + (v1351 * (v870 / v228))) - ((Lanes([0.0, v1360[0], 0.0, 0.0])) + ((v1351 * (v233 * (v228.powf(v1356)))) * v232))) - (((v1351 + v1088) + ((Lanes([0.0, v1371[0], 0.0, 0.0])) + ((v1351 * (v240 * (v228.powf(v1367)))) * v239))) * v244)) / v243) * v245)) - (v1055 * v247)) / v174;
                v368 = v247;
                v877 = v1386;
            } else {
                let v249 = if v188 < v248 { 1.0 } else { 0.0 };
                let v369: f64;
                let v878: Lanes<4>;
                if v249 != 0.0 {
                    let v250 = v192 + v180;
                    let v251 = v188 / v250;
                    let v252 = v251 + v34;
                    let v253 = v5 / v252;
                    let v1204 = ((((v1076 - ((v1088 + (Lanes([0.0, v1066[0], 0.0, 0.0]))) * v251)) / v250) * v253) * v895) / v252;
                    let v256 = v255 * v180;
                    let v258 = v253.powf(v257);
                    let v1211 = (v1066 * v255) * v258;
                    let v261 = v5 + v188;
                    let v265 = v264 * v180;
                    let v267 = v253.powf(v266);
                    let v1224 = (v1066 * v264) * v267;
                    let v269 = (v261 + (v253.ln())) - (v265 * v267);
                    let v270 = ((v253 + v192) + (v256 * v258)) / v269;
                    let v1231 = (((v1204 + v1088) + ((Lanes([0.0, v1211[0], 0.0, 0.0])) + ((v1204 * (v257 * (v253.powf(v1207)))) * v256))) - (((v1076 + (v1204 * (v870 / v253))) - ((Lanes([0.0, v1224[0], 0.0, 0.0])) + ((v1204 * (v266 * (v253.powf(v1220)))) * v265))) * v270)) / v269;
                    let v273 = v272 * v180;
                    let v275 = v270.powf(v274);
                    let v1238 = (v1066 * v272) * v275;
                    let v281 = v280 * v180;
                    let v283 = v270.powf(v282);
                    let v1251 = (v1066 * v280) * v283;
                    let v285 = (v261 + (v270.ln())) - (v281 * v283);
                    let v286 = ((v270 + v192) + (v273 * v275)) / v285;
                    let v1258 = (((v1231 + v1088) + ((Lanes([0.0, v1238[0], 0.0, 0.0])) + ((v1231 * (v274 * (v270.powf(v1234)))) * v273))) - (((v1076 + (v1231 * (v870 / v270))) - ((Lanes([0.0, v1251[0], 0.0, 0.0])) + ((v1231 * (v282 * (v270.powf(v1247)))) * v281))) * v286)) / v285;
                    let v290 = v289 * v180;
                    let v292 = v286.powf(v291);
                    let v1267 = (v1066 * v289) * v292;
                    let v297 = v296 * v180;
                    let v299 = v286.powf(v298);
                    let v1278 = (v1066 * v296) * v299;
                    let v301 = (v286 + v192) + (v297 * v299);
                    let v302 = ((v261 + (v286.ln())) - (v290 * v292)) / v301;
                    let v303 = v1 * v177;
                    let v1287 = (v1059 * v1) * v302;
                    let v305 = (v303 * v302) / v174;
                    let v1293 = (((Lanes([0.0, v1287[0], 0.0, 0.0])) + (((((v1076 + (v1258 * (v870 / v286))) - ((Lanes([0.0, v1267[0], 0.0, 0.0])) + ((v1258 * (v291 * (v286.powf(v1263)))) * v290))) - (((v1258 + v1088) + ((Lanes([0.0, v1278[0], 0.0, 0.0])) + ((v1258 * (v298 * (v286.powf(v1274)))) * v297))) * v302)) / v301) * v303)) - (v1055 * v305)) / v174;
                    v369 = v305;
                    v878 = v1293;
                } else {
                    let v306 = v192 + v180;
                    let v307 = v188 / v306;
                    let v308 = v307 + v34;
                    let v309 = v126 / v308;
                    let v1096 = ((((v1076 - ((v1088 + (Lanes([0.0, v1066[0], 0.0, 0.0]))) * v307)) / v306) * v309) * v895) / v308;
                    let v310 = v5 + v192;
                    let v312 = v311 * v180;
                    let v314 = v309.powf(v313);
                    let v1102 = (v1066 * v311) * v314;
                    let v317 = v34 + v188;
                    let v319 = v318 * v180;
                    let v321 = v309.powf(v320);
                    let v1112 = (v1066 * v318) * v321;
                    let v323 = v317 - (v319 * v321);
                    let v324 = (v310 + (v312 * v314)) / v323;
                    let v1119 = ((v1088 + ((Lanes([0.0, v1102[0], 0.0, 0.0])) + ((v1096 * (v313 * (v309.powf(v1098)))) * v312))) - ((v1076 - ((Lanes([0.0, v1112[0], 0.0, 0.0])) + ((v1096 * (v320 * (v309.powf(v1108)))) * v319))) * v324)) / v323;
                    let v326 = v325 * v180;
                    let v328 = v324.powf(v327);
                    let v1125 = (v1066 * v325) * v328;
                    let v332 = v331 * v180;
                    let v334 = v324.powf(v333);
                    let v1135 = (v1066 * v331) * v334;
                    let v336 = v317 - (v332 * v334);
                    let v337 = (v310 + (v326 * v328)) / v336;
                    let v1142 = ((v1088 + ((Lanes([0.0, v1125[0], 0.0, 0.0])) + ((v1119 * (v327 * (v324.powf(v1121)))) * v326))) - ((v1076 - ((Lanes([0.0, v1135[0], 0.0, 0.0])) + ((v1119 * (v333 * (v324.powf(v1131)))) * v332))) * v337)) / v336;
                    let v339 = v338 * v180;
                    let v341 = v337.powf(v340);
                    let v1148 = (v1066 * v338) * v341;
                    let v345 = v344 * v180;
                    let v347 = v337.powf(v346);
                    let v1158 = (v1066 * v344) * v347;
                    let v349 = v317 - (v345 * v347);
                    let v350 = (v310 + (v339 * v341)) / v349;
                    let v1165 = ((v1088 + ((Lanes([0.0, v1148[0], 0.0, 0.0])) + ((v1142 * (v340 * (v337.powf(v1144)))) * v339))) - ((v1076 - ((Lanes([0.0, v1158[0], 0.0, 0.0])) + ((v1142 * (v346 * (v337.powf(v1154)))) * v345))) * v350)) / v349;
                    let v353 = v352 * v180;
                    let v355 = v350.powf(v354);
                    let v1171 = (v1066 * v352) * v355;
                    let v359 = v358 * v180;
                    let v361 = v350.powf(v360);
                    let v1181 = (v1066 * v358) * v361;
                    let v363 = v310 + (v359 * v361);
                    let v364 = ((v5 + v188) - (v353 * v355)) / v363;
                    let v365 = v1 * v177;
                    let v1190 = (v1059 * v1) * v364;
                    let v367 = (v365 * v364) / v174;
                    let v1196 = (((Lanes([0.0, v1190[0], 0.0, 0.0])) + ((((v1076 - ((Lanes([0.0, v1171[0], 0.0, 0.0])) + ((v1165 * (v354 * (v350.powf(v1167)))) * v353))) - ((v1088 + ((Lanes([0.0, v1181[0], 0.0, 0.0])) + ((v1165 * (v360 * (v350.powf(v1177)))) * v359))) * v364)) / v363) * v365)) - (v1055 * v367)) / v174;
                    v369 = v367;
                    v878 = v1196;
                }
                v368 = v369;
                v877 = v878;
            }
            let v370 = v6 * v368;
            let v1387 = v877 * v6;
            let v371 = v159 - v370;
            let v1388 = v876 - v1387;
            let v373 = v371 - v5;
            let v1389 = v1388 * v373;
            let v377 = ((v373 * v373) + v375).sqrt();
            let v380 = (v34 * ((v371 + v5) + v377)).sqrt();
            let v381 = v168 + v380;
            let v382 = v124 / v381;
            let v1402 = (v1047 - ((v1044 + (((v1388 + ((v1389 + v1389) * (v870 / (v939 * v377)))) * v34) * (v870 / (v939 * v380)))) * v382)) / v381;
            let v383 = v5 + v382;
            let v386 = v58 / (v384 * v4);
            let v387 = v119 - v159;
            let v1403 = v985 - v876;
            let v1405 = v389 - v870;
            let v1406 = v938 * v391;
            let v393 = v390 + (v391 * v83);
            let v1407 = v394 - v870;
            let v398 = -v397;
            let v403 = (v396 * (v15.powf(v398))) * v43;
            let v404 = (v6 * v104) / v403;
            let v1416 = (((v887 * (v398 * (v15.powf((v398 - v870))))) * v396) * v43) * v404;
            let v1419 = ((v955 * v6) - (Lanes([v1416[0], 0.0, 0.0]))) / v403;
            let v405 = v404 * v404;
            let v1420 = v1419 * v404;
            let v1421 = v1420 + v1420;
            let v406 = v6 * v404;
            let v1422 = v1419 * v6;
            let v407 = v6 + v404;
            let v408 = v406 * v368;
            let v1423 = v1422 * v368;
            let v1426 = (Lanes([0.0, v1423[0], v1423[1], v1423[2]])) + (v877 * v406);
            let v410 = (v368 * v368) + v368;
            let v1429 = v1422 * v410;
            let v1433 = Lanes([0.0, v1419[0], v1419[1], v1419[2]]);
            let v1435 = v1419 * v407;
            let v1436 = v1435 + v1435;
            let v417 = ((v407 * v407) + (v414 * v408)).sqrt();
            let v418 = (v407 + v408) + v417;
            let v419 = (v406 * v410) / v418;
            let v1446 = (((Lanes([0.0, v1429[0], v1429[1], v1429[2]])) + (((v877 * v370) + v877) * v406)) - (((v1433 + v1426) + (((Lanes([0.0, v1436[0], v1436[1], v1436[2]])) + (v1426 * v414)) * (v870 / (v939 * v417)))) * v419)) / v418;
            let v420 = v368 - v419;
            let v1447 = v877 - v1446;
            let v421 = v420 * v420;
            let v1448 = v1447 * v420;
            let v1449 = v1448 + v1448;
            let v423 = v6 - v422;
            let v426 = (v6 * v419) + (v419.ln());
            let v1454 = v1419 * v420;
            let v428 = v5 + (v404 * v420);
            let v430 = v404 * v423;
            let v1462 = (v1419 * v423) * v420;
            let v432 = v84 + (v430 * v420);
            let v435 = ((v6 * v405) * v423) * v423;
            let v1469 = (((v1421 * v6) * v423) * v423) * v421;
            let v437 = (v435 * v421) / v432;
            let v1476 = v1421 * v421;
            let v441 = ((v5 + v437) + (v405 * v421)).sqrt();
            let v442 = (v426 * v428) / v441;
            let v444 = ((v159 - v56) - v442) - v108;
            let v1488 = ((v876 - v1073) - ((((((v1446 * v6) + (v1446 * (v870 / v419))) * v428) + (((Lanes([0.0, v1454[0], v1454[1], v1454[2]])) + (v1447 * v404)) * v426)) - (((((((Lanes([0.0, v1469[0], v1469[1], v1469[2]])) + (v1449 * v435)) - (((Lanes([0.0, v1462[0], v1462[1], v1462[2]])) + (v1447 * v430)) * v437)) / v432) + ((Lanes([0.0, v1476[0], v1476[1], v1476[2]])) + (v1449 * v405))) * (v870 / (v939 * v441))) * v442)) / v441)) - v1075;
            let v446 = v444 - v126;
            let v1489 = v1488 * v446;
            let v450 = ((v446 * v446) + v448).sqrt();
            let v452 = v34 * ((v444 + v126) + v450);
            let v1495 = (v1488 + ((v1489 + v1489) * (v870 / (v939 * v450)))) * v34;
            let v454 = v453 / v422;
            let v458 = v368 + v5;
            let v459 = (v454 * ((v414 * v419) + v422)) / v458;
            let v460 = v106 - v108;
            let v461 = v414 * v459;
            let v1502 = ((((v1446 * v414) * v454) - (v877 * v459)) / v458) * v414;
            let v462 = v461 / v452;
            let v464 = (v5 + v462).sqrt();
            let v465 = v460 * v464;
            let v1509 = (v962 - v971) * v464;
            let v1512 = (Lanes([0.0, v1509[0], v1509[1], v1509[2]])) + ((((v1502 - (v1495 * v462)) / v452) * (v870 / (v939 * v464))) * v460);
            let v466 = v465 + v452;
            let v1514 = (v1512 + v1495) * v466;
            let v468 = v461 * v452;
            let v1518 = (v1502 * v452) + (v1495 * v461);
            let v470 = ((v466 * v466) + v468).sqrt();
            let v471 = v465 - v452;
            let v1524 = (v1512 - v1495) * v471;
            let v474 = ((v471 * v471) + v468).sqrt();
            let v477 = (v34 * (v470 - v474)) + v108;
            let v1532 = (((((v1514 + v1514) + v1518) * (v870 / (v939 * v470))) - (((v1524 + v1524) + v1518) * (v870 / (v939 * v474)))) * v34) + v1075;
            let v481 = ((v34 * v404) * v43) / v480;
            let v482 = v106 - v477;
            let v483 = v481 * v482;
            let v1538 = (((v1419 * v34) * v43) / v480) * v482;
            let v1541 = (Lanes([0.0, v1538[0], v1538[1], v1538[2]])) + (((Lanes([0.0, v962[0], v962[1], v962[2]])) - v1532) * v481);
            let v486 = v480 / (v43 - (v6 * v480));
            let v488 = v487 * v480;
            let v1542 = v1541 * v483;
            let v491 = v6 * v486;
            let v495 = (((v483 * v483) + (v491 * v483)) + v5).sqrt();
            let v497 = v486 + v5;
            let v498 = ((v486 + v483) + v495) / v497;
            let v500 = v488 * (v498.ln());
            let v1553 = (((v1541 + (((v1542 + v1542) + (v1541 * v491)) * (v870 / (v939 * v495)))) / v497) * (v870 / v498)) * v488;
            let v501 = v187 - v477;
            let v1554 = v1074 - v1532;
            let v502 = (v6 * v383) * v58;
            let v503 = v502 * v41;
            let v1557 = v902 * v502;
            let v1559 = (((v1402 * v6) * v58) * v41) + (Lanes([0.0, v1557[0], 0.0, 0.0]));
            let v504 = v58 * v383;
            let v505 = v1 / v504;
            let v1565 = v1059 * v505;
            let v507 = (v505 * v177) / v41;
            let v1568 = v902 * v507;
            let v1571 = (((((((v1402 * v58) * v505) * v895) / v504) * v177) + (Lanes([0.0, v1565[0], 0.0, 0.0]))) - (Lanes([0.0, v1568[0], 0.0, 0.0]))) / v41;
            let v508 = if v501 <= v0 { 1.0 } else { 0.0 };
            let v682: f64;
            let v879: Lanes<4>;
            if v508 != 0.0 {
                let v509 = v501.exp();
                let v510 = v133 / v509;
                let v1780 = (((v1554 * v509) * v510) * v895) / v509;
                let v513 = v512 * v180;
                let v515 = v510.powf(v514);
                let v1787 = (v1066 * v512) * v515;
                let v518 = v5 + v501;
                let v522 = v521 * v180;
                let v524 = v510.powf(v523);
                let v1800 = (v1066 * v521) * v524;
                let v526 = (v518 + (v510.ln())) - (v522 * v524);
                let v527 = ((v510 + v507) + (v513 * v515)) / v526;
                let v1807 = (((v1780 + v1571) + ((Lanes([0.0, v1787[0], 0.0, 0.0])) + ((v1780 * (v514 * (v510.powf(v1783)))) * v513))) - (((v1554 + (v1780 * (v870 / v510))) - ((Lanes([0.0, v1800[0], 0.0, 0.0])) + ((v1780 * (v523 * (v510.powf(v1796)))) * v522))) * v527)) / v526;
                let v530 = v529 * v180;
                let v532 = v527.powf(v531);
                let v1814 = (v1066 * v529) * v532;
                let v538 = v537 * v180;
                let v540 = v527.powf(v539);
                let v1827 = (v1066 * v537) * v540;
                let v542 = (v518 + (v527.ln())) - (v538 * v540);
                let v543 = ((v527 + v507) + (v530 * v532)) / v542;
                let v1834 = (((v1807 + v1571) + ((Lanes([0.0, v1814[0], 0.0, 0.0])) + ((v1807 * (v531 * (v527.powf(v1810)))) * v530))) - (((v1554 + (v1807 * (v870 / v527))) - ((Lanes([0.0, v1827[0], 0.0, 0.0])) + ((v1807 * (v539 * (v527.powf(v1823)))) * v538))) * v543)) / v542;
                let v547 = v546 * v180;
                let v549 = v543.powf(v548);
                let v1843 = (v1066 * v546) * v549;
                let v554 = v553 * v180;
                let v556 = v543.powf(v555);
                let v1854 = (v1066 * v553) * v556;
                let v558 = (v543 + v507) + (v554 * v556);
                let v559 = ((v518 + (v543.ln())) - (v547 * v549)) / v558;
                let v560 = v1 * v177;
                let v1863 = (v1059 * v1) * v559;
                let v562 = (v560 * v559) / v503;
                let v1869 = (((Lanes([0.0, v1863[0], 0.0, 0.0])) + (((((v1554 + (v1834 * (v870 / v543))) - ((Lanes([0.0, v1843[0], 0.0, 0.0])) + ((v1834 * (v548 * (v543.powf(v1839)))) * v547))) - (((v1834 + v1571) + ((Lanes([0.0, v1854[0], 0.0, 0.0])) + ((v1834 * (v555 * (v543.powf(v1850)))) * v554))) * v559)) / v558) * v560)) - (v1559 * v562)) / v503;
                v682 = v562;
                v879 = v1869;
            } else {
                let v563 = if v501 < v248 { 1.0 } else { 0.0 };
                let v683: f64;
                let v880: Lanes<4>;
                if v563 != 0.0 {
                    let v564 = v507 + v180;
                    let v565 = v501 / v564;
                    let v566 = v565 + v34;
                    let v567 = v5 / v566;
                    let v1687 = ((((v1554 - ((v1571 + (Lanes([0.0, v1066[0], 0.0, 0.0]))) * v565)) / v564) * v567) * v895) / v566;
                    let v570 = v569 * v180;
                    let v572 = v567.powf(v571);
                    let v1694 = (v1066 * v569) * v572;
                    let v575 = v5 + v501;
                    let v579 = v578 * v180;
                    let v581 = v567.powf(v580);
                    let v1707 = (v1066 * v578) * v581;
                    let v583 = (v575 + (v567.ln())) - (v579 * v581);
                    let v584 = ((v567 + v507) + (v570 * v572)) / v583;
                    let v1714 = (((v1687 + v1571) + ((Lanes([0.0, v1694[0], 0.0, 0.0])) + ((v1687 * (v571 * (v567.powf(v1690)))) * v570))) - (((v1554 + (v1687 * (v870 / v567))) - ((Lanes([0.0, v1707[0], 0.0, 0.0])) + ((v1687 * (v580 * (v567.powf(v1703)))) * v579))) * v584)) / v583;
                    let v587 = v586 * v180;
                    let v589 = v584.powf(v588);
                    let v1721 = (v1066 * v586) * v589;
                    let v595 = v594 * v180;
                    let v597 = v584.powf(v596);
                    let v1734 = (v1066 * v594) * v597;
                    let v599 = (v575 + (v584.ln())) - (v595 * v597);
                    let v600 = ((v584 + v507) + (v587 * v589)) / v599;
                    let v1741 = (((v1714 + v1571) + ((Lanes([0.0, v1721[0], 0.0, 0.0])) + ((v1714 * (v588 * (v584.powf(v1717)))) * v587))) - (((v1554 + (v1714 * (v870 / v584))) - ((Lanes([0.0, v1734[0], 0.0, 0.0])) + ((v1714 * (v596 * (v584.powf(v1730)))) * v595))) * v600)) / v599;
                    let v604 = v603 * v180;
                    let v606 = v600.powf(v605);
                    let v1750 = (v1066 * v603) * v606;
                    let v611 = v610 * v180;
                    let v613 = v600.powf(v612);
                    let v1761 = (v1066 * v610) * v613;
                    let v615 = (v600 + v507) + (v611 * v613);
                    let v616 = ((v575 + (v600.ln())) - (v604 * v606)) / v615;
                    let v617 = v1 * v177;
                    let v1770 = (v1059 * v1) * v616;
                    let v619 = (v617 * v616) / v503;
                    let v1776 = (((Lanes([0.0, v1770[0], 0.0, 0.0])) + (((((v1554 + (v1741 * (v870 / v600))) - ((Lanes([0.0, v1750[0], 0.0, 0.0])) + ((v1741 * (v605 * (v600.powf(v1746)))) * v604))) - (((v1741 + v1571) + ((Lanes([0.0, v1761[0], 0.0, 0.0])) + ((v1741 * (v612 * (v600.powf(v1757)))) * v611))) * v616)) / v615) * v617)) - (v1559 * v619)) / v503;
                    v683 = v619;
                    v880 = v1776;
                } else {
                    let v620 = v507 + v180;
                    let v621 = v501 / v620;
                    let v622 = v621 + v34;
                    let v623 = v126 / v622;
                    let v1579 = ((((v1554 - ((v1571 + (Lanes([0.0, v1066[0], 0.0, 0.0]))) * v621)) / v620) * v623) * v895) / v622;
                    let v624 = v5 + v507;
                    let v626 = v625 * v180;
                    let v628 = v623.powf(v627);
                    let v1585 = (v1066 * v625) * v628;
                    let v631 = v34 + v501;
                    let v633 = v632 * v180;
                    let v635 = v623.powf(v634);
                    let v1595 = (v1066 * v632) * v635;
                    let v637 = v631 - (v633 * v635);
                    let v638 = (v624 + (v626 * v628)) / v637;
                    let v1602 = ((v1571 + ((Lanes([0.0, v1585[0], 0.0, 0.0])) + ((v1579 * (v627 * (v623.powf(v1581)))) * v626))) - ((v1554 - ((Lanes([0.0, v1595[0], 0.0, 0.0])) + ((v1579 * (v634 * (v623.powf(v1591)))) * v633))) * v638)) / v637;
                    let v640 = v639 * v180;
                    let v642 = v638.powf(v641);
                    let v1608 = (v1066 * v639) * v642;
                    let v646 = v645 * v180;
                    let v648 = v638.powf(v647);
                    let v1618 = (v1066 * v645) * v648;
                    let v650 = v631 - (v646 * v648);
                    let v651 = (v624 + (v640 * v642)) / v650;
                    let v1625 = ((v1571 + ((Lanes([0.0, v1608[0], 0.0, 0.0])) + ((v1602 * (v641 * (v638.powf(v1604)))) * v640))) - ((v1554 - ((Lanes([0.0, v1618[0], 0.0, 0.0])) + ((v1602 * (v647 * (v638.powf(v1614)))) * v646))) * v651)) / v650;
                    let v653 = v652 * v180;
                    let v655 = v651.powf(v654);
                    let v1631 = (v1066 * v652) * v655;
                    let v659 = v658 * v180;
                    let v661 = v651.powf(v660);
                    let v1641 = (v1066 * v658) * v661;
                    let v663 = v631 - (v659 * v661);
                    let v664 = (v624 + (v653 * v655)) / v663;
                    let v1648 = ((v1571 + ((Lanes([0.0, v1631[0], 0.0, 0.0])) + ((v1625 * (v654 * (v651.powf(v1627)))) * v653))) - ((v1554 - ((Lanes([0.0, v1641[0], 0.0, 0.0])) + ((v1625 * (v660 * (v651.powf(v1637)))) * v659))) * v664)) / v663;
                    let v667 = v666 * v180;
                    let v669 = v664.powf(v668);
                    let v1654 = (v1066 * v666) * v669;
                    let v673 = v672 * v180;
                    let v675 = v664.powf(v674);
                    let v1664 = (v1066 * v672) * v675;
                    let v677 = v624 + (v673 * v675);
                    let v678 = ((v5 + v501) - (v667 * v669)) / v677;
                    let v679 = v1 * v177;
                    let v1673 = (v1059 * v1) * v678;
                    let v681 = (v679 * v678) / v503;
                    let v1679 = (((Lanes([0.0, v1673[0], 0.0, 0.0])) + ((((v1554 - ((Lanes([0.0, v1654[0], 0.0, 0.0])) + ((v1648 * (v668 * (v664.powf(v1650)))) * v667))) - ((v1571 + ((Lanes([0.0, v1664[0], 0.0, 0.0])) + ((v1648 * (v674 * (v664.powf(v1660)))) * v673))) * v678)) / v677) * v679)) - (v1559 * v681)) / v503;
                    v683 = v681;
                    v880 = v1679;
                }
                v682 = v683;
                v879 = v880;
            }
            let v685 = (v159 - v368) - v682;
            let v1871 = (v876 - v877) - v879;
            let v687 = v685 - v5;
            let v1872 = v1871 * v687;
            let v691 = ((v687 * v687) + v689).sqrt();
            let v694 = (v34 * ((v685 + v5) + v691)).sqrt();
            let v695 = v168 + v694;
            let v696 = v124 / v695;
            let v1885 = (v1047 - ((v1044 + (((v1871 + ((v1872 + v1872) * (v870 / (v939 * v691)))) * v34) * (v870 / (v939 * v694)))) * v696)) / v695;
            let v697 = v5 + v696;
            let v698 = v368 - v682;
            let v1886 = v877 - v879;
            let v699 = v698 * v698;
            let v1887 = v1886 * v698;
            let v700 = v458 + v682;
            let v1889 = v877 + v879;
            let v701 = v5 / v700;
            let v1892 = ((v1889 * v701) * v895) / v700;
            let v702 = v699 * v701;
            let v1895 = ((v1887 + v1887) * v701) + (v1892 * v699);
            let v703 = v697 - v5;
            let v706 = (v368 + v682) + (v33 * v702);
            let v708 = v387 - (v703 * v706);
            let v709 = v33 * v697;
            let v1902 = v1885 * v33;
            let v710 = v702 * v701;
            let v1905 = (v1895 * v701) + (v1892 * v702);
            let v718 = v34 * ((v5 + (v712 * v368)) + (v715 * v682));
            let v720 = (v370 + v682) + (v718 * v710);
            let v728 = v34 * ((v5 + (v715 * v368)) + (v712 * v682));
            let v730 = (v368 + (v6 * v682)) + (v728 * v710);
            let v732 = v104 * v708;
            let v1931 = v955 * v708;
            let v1934 = (Lanes([0.0, v1931[0], v1931[1], v1931[2]])) + ((v1403 - ((v1885 * v706) + ((v1889 + (v1895 * v33)) * v703))) * v104);
            let v1935 = v1934 * v732;
            let v736 = ((v732 * v732) + v734).sqrt();
            let v738 = v34 * (v732 + v736);
            let v1941 = (v1934 + ((v1935 + v1935) * (v870 / (v939 * v736)))) * v34;
            let v739 = (v709 * v720) + (v709 * v730);
            let v740 = v104 * v739;
            let v1943 = v955 * v739;
            let v1946 = (Lanes([0.0, v1943[0], v1943[1], v1943[2]])) + ((((v1902 * v720) + (((v1387 + v879) + (((((v877 * v712) + (v879 * v715)) * v34) * v710) + (v1905 * v718))) * v709)) + ((v1902 * v730) + (((v877 + (v879 * v6)) + (((((v877 * v715) + (v879 * v712)) * v34) * v710) + (v1905 * v728))) * v709))) * v104);
            let v743 = v386 * (v738 + (v35 * v740));
            let v744 = v740 / v738;
            let v746 = v34 * (v5 + v744);
            let v747 = v746.powf(v389);
            let v748 = v743.powf(v394);
            let v1960 = v1406 * v748;
            let v750 = v395 / v747;
            let v1967 = ((Lanes([0.0, 0.0, v1960[0], v1960[1]])) + ((((v1941 + (v1946 * v35)) * v386) * (v394 * (v743.powf(v1407)))) * v393)) + (((((((v1946 - (v1941 * v744)) / v738) * v34) * (v389 * (v746.powf(v1405)))) * v750) * v895) / v747);
            let v752 = v5 + ((v393 * v748) + v750);
            let v754 = v752 - v5;
            let v1968 = v1967 * v754;
            let v758 = ((v754 * v754) + v756).sqrt();
            let v760 = v34 * ((v752 + v5) + v758);
            let v1974 = (v1967 + ((v1968 + v1968) * (v870 / (v939 * v758)))) * v34;
            let v761 = v404 / v760;
            let v762 = v6 * v761;
            let v763 = v762 * v698;
            let v1981 = ((((v1433 - (v1974 * v761)) / v760) * v6) * v698) + (v1886 * v762);
            let v764 = v763 * v763;
            let v1982 = v1981 * v763;
            let v766 = (v5 + v764).sqrt();
            let v1986 = (v1982 + v1982) * (v870 / (v939 * v766));
            let v767 = if v763 != v0 { 1.0 } else { 0.0 };
            let v776: f64;
            let v881: Lanes<4>;
            if v767 != 0.0 {
                let v768 = v5 / v763;
                let v769 = v763.asinh();
                let v772 = v34 * (v766 + (v768 * v769));
                let v2003 = (v1986 + (((((v1981 * v768) * v895) / v763) * v769) + ((v1981 * (v870 / ((v870 + v764).sqrt()))) * v768))) * v34;
                v776 = v772;
                v881 = v2003;
            } else {
                let v773 = v5 / v766;
                let v775 = v34 * (v766 + v773);
                let v1991 = (v1986 + (((v1986 * v773) * v895) / v766)) * v34;
                v776 = v775;
                v881 = v1991;
            }
            let v777 = v760 * v776;
            let v2006 = (v1974 * v776) + (v881 * v760);
            let v778 = v39 / v777;
            let v780 = v6 * v779;
            let v781 = v780 * v697;
            let v784 = v43 - v500;
            let v2016 = v1553 * v895;
            let v785 = ((v781 * v778) * v42) / v784;
            let v786 = v785 * v58;
            let v787 = v786 * v104;
            let v2022 = v955 * v786;
            let v788 = v787 * v104;
            let v2026 = v955 * v787;
            let v789 = v698 * v700;
            let v790 = v788 * v789;
            let v2034 = ((((((((((((v1885 * v780) * v778) + ((((Lanes([0.0, v901[0], 0.0, 0.0])) - (v2006 * v778)) / v777) * v781)) * v42) - (v2016 * v785)) / v784) * v58) * v104) + (Lanes([0.0, v2022[0], v2022[1], v2022[2]]))) * v104) + (Lanes([0.0, v2026[0], v2026[1], v2026[2]]))) * v789) + (((v1886 * v700) + (v1889 * v698)) * v788);
            let v796 = v5 - (v793 * (v15 - v5));
            let v2036 = (v887 * v793) * v895;
            let v805 = (v1 * v791) * v42;
            let v807 = v805 * (v800 * (v15.powf(v801)));
            let v2047 = ((v887 * (v801 * (v15.powf((v801 - v870))))) * v800) * v805;
            let v808 = v806 / v807;
            let v809 = v808 * v796;
            let v2053 = ((((v2047 * v808) * v895) / v807) * v796) + (v2036 * v808);
            let v811 = v810 / v807;
            let v812 = v811 * v796;
            let v2059 = ((((v2047 * v811) * v895) / v807) * v796) + (v2036 * v811);
            let v813 = v805 * (v792 * (v796.powf(v797)));
            let v814 = v790 / v813;
            let v2061 = (((v2036 * (v797 * (v796.powf((v797 - v870))))) * v792) * v805) * v814;
            let v2064 = (v2034 - (Lanes([0.0, v2061[0], 0.0, 0.0]))) / v813;
            let v816 = if v814 >= v815 { 1.0 } else { 0.0 };
            let v817: f64;
            let v882: Lanes<4>;
            if v816 != 0.0 {
                v817 = v815;
                v882 = v2065;
            } else {
                v817 = v814;
                v882 = v2064;
            }
            let v820 = v5 - (v817.powf(v818));
            let v821 = v5 / v818;
            let v822 = v820.powf(v821);
            let v2074 = ((v882 * (v818 * (v817.powf((v818 - v870))))) * v895) * (v821 * (v820.powf((v821 - v870))));
            let v824 = v823 / v42;
            let v826 = v825 / v42;
            let v827 = v809 / v822;
            let v828 = v812 / v822;
            let v2084 = (v887 * v829) * v826;
            let v833 = (v826 * (v5 + (v829 * v15))) + v827;
            let v2086 = (Lanes([0.0, v2084[0], 0.0, 0.0])) + (((Lanes([0.0, v2053[0], 0.0, 0.0])) - (v2074 * v827)) / v822);
            let v2088 = (v887 * v834) * v824;
            let v838 = (v824 * (v5 + (v834 * v15))) + v828;
            let v2090 = (Lanes([0.0, v2088[0], 0.0, 0.0])) + (((Lanes([0.0, v2059[0], 0.0, 0.0])) - (v2074 * v828)) / v822);
            let v839 = v36 / v777;
            let v842 = ((v839 * v58) * v42) / v784;
            let v843 = v842 * v740;
            let v844 = v838 + v833;
            let v846 = v5 + (v843 * v844);
            let v847 = v790 / v846;
            let v2108 = (v2034 - ((((((((((((v2006 * v839) * v895) / v777) * v58) * v42) - (v2016 * v842)) / v784) * v740) + (v1946 * v842)) * v844) + ((v2090 + v2086) * v843)) * v847)) / v846;
            let v848 = v833 * v847;
            let v2111 = (v2086 * v847) + (v2108 * v833);
            let v849 = v838 * v847;
            let v2114 = (v2090 * v847) + (v2108 * v838);
            let v852 = if v851 != v0 { 1.0 } else { 0.0 };
            let v866: f64;
            let v867: f64;
            let v868: f64;
            let v869: f64;
            let v883: Lanes<1>;
            let v884: Lanes<4>;
            let v885: Lanes<1>;
            let v886: Lanes<1>;
            if v852 != 0.0 {
                let v2118 = v926 * v847;
                let v854 = v847 * v847;
                let v2121 = v2108 * v847;
                let v2122 = v2121 + v2121;
                let v859 = -(((v847 * v64) + (v854 * v833)) + (v854 * v838));
                let v2131 = ((((v2108 * v64) + (Lanes([0.0, 0.0, v2118[0], v2118[1]]))) + ((v2122 * v833) + (v2086 * v854))) + ((v2122 * v838) + (v2090 * v854))) * v895;
                let v862 = ddt(4922, (v860 * v12));
                let v2134 = (v871 * v860) * v2133;
                let v863 = v12 / v851;
                let v2135 = v871 / v851;
                v866 = v862;
                v867 = v859;
                v868 = v863;
                v869 = v0;
                v883 = v2134;
                v884 = v2131;
                v885 = v2135;
                v886 = v2116;
            } else {
                let v865 = v12 * v864;
                let v2115 = v871 * v864;
                v866 = v0;
                v867 = v0;
                v868 = v0;
                v869 = v865;
                v883 = v2116;
                v884 = v2065;
                v885 = v2116;
                v886 = v2115;
            }
            let v2136 = v2114[0];
            let v2137 = v2114[1];
            let v2138 = v2114[2];
            let v2139 = v2114[3];
            let v2140 = v2108[0];
            let v2141 = v2108[1];
            let v2142 = v2108[2];
            let v2143 = v2108[3];
            let v2144 = v2111[0];
            let v2145 = v2111[1];
            let v2146 = v2111[2];
            let v2147 = v2111[3];
            let v2148 = v883[0];
            let v2149 = v884[0];
            let v2150 = v884[1];
            let v2151 = v884[2];
            let v2152 = v884[3];
            let v2153 = v885[0];
            let v2154 = v886[0];
        stamper.stamp_potential_branch_local(Some(6), Some(2), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<4, 0>(
            0,
            v849,
            [1, 4, 5, 6],
            [v2136, v2137, v2138, v2139],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (v847),
            [1, 4, 5, 6],
            [v2140, v2141, v2142, v2143],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(0), Some(5), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<4, 0>(
            1,
            v848,
            [1, 4, 5, 6],
            [v2144, v2145, v2146, v2147],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(3), Some(9), 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            v850,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (v866),
            [4],
            [v2148],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            None,
            multiplicity * (v867),
            [1, 4, 5, 6],
            [v2149, v2150, v2151, v2152],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (v868),
            [4],
            [v2153],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (v869),
            [4],
            [v2154],
            [],
            [],
            multiplicity,
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
    }

}

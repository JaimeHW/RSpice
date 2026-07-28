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
            let v871 = 1e0f64;
            let v872 = Lanes([1e0f64; 1]);
            let v873 = Lanes([1e0f64; 1]);
            let v874 = Lanes([1e0f64; 1]);
            let v875 = Lanes([1e0f64; 1]);
            let v897 = -1e0f64;
            let v941 = 2e0f64;
            let v1062 = -3.3340000000000003e-1f64;
            let v1100 = -6.666666666666667e-1f64;
            let v1110 = -1.6666666666666665e0f64;
            let v1123 = -6.666666666666667e-1f64;
            let v1133 = -1.6666666666666665e0f64;
            let v1146 = -6.666666666666667e-1f64;
            let v1156 = -1.6666666666666665e0f64;
            let v1169 = -1.6666666666666665e0f64;
            let v1179 = -6.666666666666667e-1f64;
            let v1209 = -6.666666666666667e-1f64;
            let v1222 = -1.6666666666666665e0f64;
            let v1236 = -6.666666666666667e-1f64;
            let v1249 = -1.6666666666666665e0f64;
            let v1265 = -1.6666666666666665e0f64;
            let v1276 = -6.666666666666667e-1f64;
            let v1302 = -6.666666666666667e-1f64;
            let v1315 = -1.6666666666666665e0f64;
            let v1329 = -6.666666666666667e-1f64;
            let v1342 = -1.6666666666666665e0f64;
            let v1358 = -1.6666666666666665e0f64;
            let v1369 = -6.666666666666667e-1f64;
            let v1583 = -6.666666666666667e-1f64;
            let v1593 = -1.6666666666666665e0f64;
            let v1606 = -6.666666666666667e-1f64;
            let v1616 = -1.6666666666666665e0f64;
            let v1629 = -6.666666666666667e-1f64;
            let v1639 = -1.6666666666666665e0f64;
            let v1652 = -1.6666666666666665e0f64;
            let v1662 = -6.666666666666667e-1f64;
            let v1692 = -6.666666666666667e-1f64;
            let v1705 = -1.6666666666666665e0f64;
            let v1719 = -6.666666666666667e-1f64;
            let v1732 = -1.6666666666666665e0f64;
            let v1748 = -1.6666666666666665e0f64;
            let v1759 = -6.666666666666667e-1f64;
            let v1785 = -6.666666666666667e-1f64;
            let v1798 = -1.6666666666666665e0f64;
            let v1812 = -6.666666666666667e-1f64;
            let v1825 = -1.6666666666666665e0f64;
            let v1841 = -1.6666666666666665e0f64;
            let v1852 = -6.666666666666667e-1f64;
            let v2067 = Lanes([0e0f64; 4]);
            let v2118 = Lanes([0e0f64; 1]);
            let v2135 = ddt_scale();
            let v10 = if v9 == v5 { 1.0 } else { 0.0 };
            if v10 != 0.0 {
            } else {
            }
            let v13 = v11 + v12;
            let v15 = v13 / v14;
            let v889 = v872 / v14;
            let v21 = v13 + v20;
            let v22 = (v17 * (v13 * v13)) / v21;
            let v24 = v16 - (v22 * v1);
            let v898 = (((((v872 * (v6 * v13)) * v17) - (v872 * v22)) / v21) * v1) * v897;
            let v31 = ((v27 * v25) + (v2 * (v5 - v25))) * v3;
            let v32 = if v9 != v5 { 1.0 } else { 0.0 };
            let v35: f64;
            if v32 != 0.0 {
                v35 = v33;
            } else {
                v35 = v34;
            }
            let v39 = v36 * (v15.powf(v37));
            let v903 = (v889 * (v37 * (v15.powf((v37 - v871))))) * v36;
            let v41 = v40 * v15;
            let v904 = v889 * v40;
            let v44 = v5 / v41;
            let v907 = ((v904 * v44) * v897) / v41;
            let v48 = v47 * v41;
            let v49 = (-v24) / v48;
            let v50 = v49.exp();
            let v51 = v45 * v50;
            let v53 = v52 / v51;
            let v55 = if v53 >= v54 { v53 } else { v54 };
            let v56 = v55.ln();
            let v921 = (((((((((v898 * v897) - ((v904 * v47) * v49)) / v48) * v50) * v45) * v53) * v897) / v51) * (if v53 >= v54 { 1.0 } else { 0.0 })) * (v871 / v55);
            let v58 = v31 / v57;
            let v61 = v59 - v60;
            let v924 = (Lanes([v873[0], 0.0])) - (Lanes([0.0, v874[0]]));
            let v62 = v60 - v60;
            let v925 = v874 - v874;
            let v64 = v63 - v60;
            let v928 = (Lanes([v875[0], 0.0])) - (Lanes([0.0, v874[0]]));
            let v65 = v64 - v62;
            let v929 = Lanes([0.0, v925[0]]);
            let v930 = v928 - v929;
            let v67 = v66 * v65;
            let v931 = v930 * v66;
            let v69 = if v67 > v68 { 1.0 } else { 0.0 };
            let v74: f64;
            let v876: Lanes<2>;
            if v69 != 0.0 {
                v74 = v67;
                v876 = v931;
            } else {
                let v70 = v67.exp();
                let v71 = v5 + v70;
                let v72 = v71.ln();
                let v934 = (v931 * v70) * (v871 / v71);
                v74 = v72;
                v876 = v934;
            }
            let v73 = v6 / v66;
            let v936 = (v876 * v73) - v930;
            let v79 = ((v73 * v74) - v65) - (v73 * v77);
            let v83 = -(v62 + (v34 * (v65 - v79)));
            let v940 = (v929 + ((v930 - v936) * v34)) * v897;
            let v945 = ((v936 * v88) - (v940 * v91)) / v58;
            let v95 = v5 + ((((v85 + v86) + (v88 * v79)) - (v91 * v83)) / v58);
            let v97 = v95 - v5;
            let v946 = v945 * v97;
            let v101 = ((v97 * v97) + v99).sqrt();
            let v103 = v34 * ((v95 + v5) + v101);
            let v104 = v103 * v41;
            let v953 = ((v945 + ((v946 + v946) * (v871 / (v941 * v101)))) * v34) * v41;
            let v954 = v904 * v103;
            let v957 = (Lanes([0.0, v953[0], v953[1]])) + (Lanes([v954[0], 0.0, 0.0]));
            let v105 = v5 / v104;
            let v960 = ((v957 * v105) * v897) / v104;
            let v106 = v64 * v105;
            let v961 = v928 * v105;
            let v964 = (Lanes([0.0, v961[0], v961[1]])) + (v960 * v64);
            let v965 = v924 * v105;
            let v966 = v960 * v61;
            let v108 = v62 * v105;
            let v970 = v925 * v105;
            let v973 = (Lanes([0.0, 0.0, v970[0]])) + (v960 * v62);
            let v974 = v960 * v109;
            let v115 = -(v111 + (v112 * v83));
            let v116 = v115 * v79;
            let v982 = ((((v940 * v112) * v897) * v79) + (v936 * v115)) * v105;
            let v985 = (Lanes([0.0, v982[0], v982[1]])) + (v960 * v116);
            let v119 = ((v61 * v105) - (v109 * v105)) - (v116 * v105);
            let v987 = (((Lanes([v965[0], 0.0, 0.0, v965[1]])) + (Lanes([0.0, v966[0], v966[1], v966[2]]))) - (Lanes([0.0, v974[0], v974[1], v974[2]]))) - (Lanes([0.0, v985[0], v985[1], v985[2]]));
            let v121 = v120 * v52;
            let v123 = (v121 * v44).sqrt();
            let v124 = v123 / v58;
            let v992 = ((v907 * v121) * (v871 / (v941 * v123))) / v58;
            let v995 = (v992 / v127) * v126;
            let v131 = (v34 * v119) - (v126 * (v5 + (v124 / v127)));
            let v997 = (v987 * v34) - (Lanes([0.0, v995[0], 0.0, 0.0]));
            let v998 = v997 * v131;
            let v136 = ((v131 * v131) + (v133 * v119)).sqrt();
            let v137 = v131 + v136;
            let v1005 = v997 + (((v998 + v998) + (v987 * v133)) * (v871 / (v941 * v136)));
            let v138 = if v119 < v0 { 1.0 } else { 0.0 };
            let v159: f64;
            let v877: Lanes<4>;
            if v138 != 0.0 {
                let v140 = (v119 - v137) / v124;
                let v1024 = v992 * v140;
                let v1029 = (((v987 - v1005) - (Lanes([0.0, v1024[0], 0.0, 0.0]))) / v124) * v140;
                let v143 = (v5 - v137) + (v140 * v140);
                let v144 = if v143 >= v54 { v143 } else { v54 };
                let v146 = -(v144.ln());
                let v1036 = ((((v1005 * v897) + (v1029 + v1029)) * (if v143 >= v54 { 1.0 } else { 0.0 })) * (v871 / v144)) * v897;
                v159 = v146;
                v877 = v1036;
            } else {
                let v147 = -v137;
                let v148 = rspice_limited_exp(v147);
                let v1008 = (v1005 * v897) * (rspice_limited_exp_derivative(v147));
                let v149 = v34 * v124;
                let v1009 = v992 * v34;
                let v1011 = v1009 * v149;
                let v1012 = v1011 + v1011;
                let v154 = (((v119 - v5) + v148) + (v149 * v149)).sqrt();
                let v155 = v154 - v149;
                let v1020 = ((((v987 + v1008) + (Lanes([0.0, v1012[0], 0.0, 0.0]))) * (v871 / (v941 * v154))) - (Lanes([0.0, v1009[0], 0.0, 0.0]))) * v155;
                let v158 = ((v155 * v155) + v5) - v148;
                let v1022 = (v1020 + v1020) - v1008;
                v159 = v158;
                v877 = v1022;
            }
            let v161 = v159 - v5;
            let v1037 = v877 * v161;
            let v165 = ((v161 * v161) + v163).sqrt();
            let v168 = (v34 * ((v159 + v5) + v165)).sqrt();
            let v1046 = ((v877 + ((v1037 + v1037) * (v871 / (v941 * v165)))) * v34) * (v871 / (v941 * v168));
            let v169 = v6 * v168;
            let v170 = v124 / v169;
            let v1049 = Lanes([0.0, v992[0], 0.0, 0.0]);
            let v1051 = (v1049 - ((v1046 * v6) * v170)) / v169;
            let v171 = v5 + v170;
            let v173 = (v6 * v171) * v58;
            let v174 = v173 * v41;
            let v1055 = v904 * v173;
            let v1057 = (((v1051 * v6) * v58) * v41) + (Lanes([0.0, v1055[0], 0.0, 0.0]));
            let v176 = v175 / v41;
            let v177 = v7 * v41;
            let v1061 = v904 * v7;
            let v179 = v177.powf(v178);
            let v180 = v176 * v179;
            let v1068 = ((((v904 * v176) * v897) / v41) * v179) + ((v1061 * (v178 * (v177.powf(v1062)))) * v176);
            let v183 = v1 * v41;
            let v184 = v24 / v183;
            let v1072 = (v898 - ((v904 * v1) * v184)) / v183;
            let v187 = (v159 - (((v52 / v8).ln()) + v184)) + v56;
            let v1075 = Lanes([0.0, v921[0], 0.0, 0.0]);
            let v1076 = (v877 - (Lanes([0.0, v1072[0], 0.0, 0.0]))) + v1075;
            let v188 = v187 - v108;
            let v1077 = Lanes([0.0, v973[0], v973[1], v973[2]]);
            let v1078 = v1076 - v1077;
            let v189 = v58 * v171;
            let v190 = v1 / v189;
            let v1084 = v1061 * v190;
            let v192 = (v190 * v177) / v41;
            let v1087 = v904 * v192;
            let v1090 = (((((((v1051 * v58) * v190) * v897) / v189) * v177) + (Lanes([0.0, v1084[0], 0.0, 0.0]))) - (Lanes([0.0, v1087[0], 0.0, 0.0]))) / v41;
            let v193 = if v188 <= v0 { 1.0 } else { 0.0 };
            let v368: f64;
            let v878: Lanes<4>;
            if v193 != 0.0 {
                let v194 = v188.exp();
                let v195 = v133 / v194;
                let v1299 = (((v1078 * v194) * v195) * v897) / v194;
                let v198 = v197 * v180;
                let v200 = v195.powf(v199);
                let v1306 = (v1068 * v197) * v200;
                let v203 = v5 + v188;
                let v207 = v206 * v180;
                let v209 = v195.powf(v208);
                let v1319 = (v1068 * v206) * v209;
                let v211 = (v203 + (v195.ln())) - (v207 * v209);
                let v212 = ((v195 + v192) + (v198 * v200)) / v211;
                let v1326 = (((v1299 + v1090) + ((Lanes([0.0, v1306[0], 0.0, 0.0])) + ((v1299 * (v199 * (v195.powf(v1302)))) * v198))) - (((v1078 + (v1299 * (v871 / v195))) - ((Lanes([0.0, v1319[0], 0.0, 0.0])) + ((v1299 * (v208 * (v195.powf(v1315)))) * v207))) * v212)) / v211;
                let v215 = v214 * v180;
                let v217 = v212.powf(v216);
                let v1333 = (v1068 * v214) * v217;
                let v223 = v222 * v180;
                let v225 = v212.powf(v224);
                let v1346 = (v1068 * v222) * v225;
                let v227 = (v203 + (v212.ln())) - (v223 * v225);
                let v228 = ((v212 + v192) + (v215 * v217)) / v227;
                let v1353 = (((v1326 + v1090) + ((Lanes([0.0, v1333[0], 0.0, 0.0])) + ((v1326 * (v216 * (v212.powf(v1329)))) * v215))) - (((v1078 + (v1326 * (v871 / v212))) - ((Lanes([0.0, v1346[0], 0.0, 0.0])) + ((v1326 * (v224 * (v212.powf(v1342)))) * v223))) * v228)) / v227;
                let v232 = v231 * v180;
                let v234 = v228.powf(v233);
                let v1362 = (v1068 * v231) * v234;
                let v239 = v238 * v180;
                let v241 = v228.powf(v240);
                let v1373 = (v1068 * v238) * v241;
                let v243 = (v228 + v192) + (v239 * v241);
                let v244 = ((v203 + (v228.ln())) - (v232 * v234)) / v243;
                let v245 = v1 * v177;
                let v1382 = (v1061 * v1) * v244;
                let v247 = (v245 * v244) / v174;
                let v1388 = (((Lanes([0.0, v1382[0], 0.0, 0.0])) + (((((v1078 + (v1353 * (v871 / v228))) - ((Lanes([0.0, v1362[0], 0.0, 0.0])) + ((v1353 * (v233 * (v228.powf(v1358)))) * v232))) - (((v1353 + v1090) + ((Lanes([0.0, v1373[0], 0.0, 0.0])) + ((v1353 * (v240 * (v228.powf(v1369)))) * v239))) * v244)) / v243) * v245)) - (v1057 * v247)) / v174;
                v368 = v247;
                v878 = v1388;
            } else {
                let v249 = if v188 < v248 { 1.0 } else { 0.0 };
                let v369: f64;
                let v879: Lanes<4>;
                if v249 != 0.0 {
                    let v250 = v192 + v180;
                    let v251 = v188 / v250;
                    let v252 = v251 + v34;
                    let v253 = v5 / v252;
                    let v1206 = ((((v1078 - ((v1090 + (Lanes([0.0, v1068[0], 0.0, 0.0]))) * v251)) / v250) * v253) * v897) / v252;
                    let v256 = v255 * v180;
                    let v258 = v253.powf(v257);
                    let v1213 = (v1068 * v255) * v258;
                    let v261 = v5 + v188;
                    let v265 = v264 * v180;
                    let v267 = v253.powf(v266);
                    let v1226 = (v1068 * v264) * v267;
                    let v269 = (v261 + (v253.ln())) - (v265 * v267);
                    let v270 = ((v253 + v192) + (v256 * v258)) / v269;
                    let v1233 = (((v1206 + v1090) + ((Lanes([0.0, v1213[0], 0.0, 0.0])) + ((v1206 * (v257 * (v253.powf(v1209)))) * v256))) - (((v1078 + (v1206 * (v871 / v253))) - ((Lanes([0.0, v1226[0], 0.0, 0.0])) + ((v1206 * (v266 * (v253.powf(v1222)))) * v265))) * v270)) / v269;
                    let v273 = v272 * v180;
                    let v275 = v270.powf(v274);
                    let v1240 = (v1068 * v272) * v275;
                    let v281 = v280 * v180;
                    let v283 = v270.powf(v282);
                    let v1253 = (v1068 * v280) * v283;
                    let v285 = (v261 + (v270.ln())) - (v281 * v283);
                    let v286 = ((v270 + v192) + (v273 * v275)) / v285;
                    let v1260 = (((v1233 + v1090) + ((Lanes([0.0, v1240[0], 0.0, 0.0])) + ((v1233 * (v274 * (v270.powf(v1236)))) * v273))) - (((v1078 + (v1233 * (v871 / v270))) - ((Lanes([0.0, v1253[0], 0.0, 0.0])) + ((v1233 * (v282 * (v270.powf(v1249)))) * v281))) * v286)) / v285;
                    let v290 = v289 * v180;
                    let v292 = v286.powf(v291);
                    let v1269 = (v1068 * v289) * v292;
                    let v297 = v296 * v180;
                    let v299 = v286.powf(v298);
                    let v1280 = (v1068 * v296) * v299;
                    let v301 = (v286 + v192) + (v297 * v299);
                    let v302 = ((v261 + (v286.ln())) - (v290 * v292)) / v301;
                    let v303 = v1 * v177;
                    let v1289 = (v1061 * v1) * v302;
                    let v305 = (v303 * v302) / v174;
                    let v1295 = (((Lanes([0.0, v1289[0], 0.0, 0.0])) + (((((v1078 + (v1260 * (v871 / v286))) - ((Lanes([0.0, v1269[0], 0.0, 0.0])) + ((v1260 * (v291 * (v286.powf(v1265)))) * v290))) - (((v1260 + v1090) + ((Lanes([0.0, v1280[0], 0.0, 0.0])) + ((v1260 * (v298 * (v286.powf(v1276)))) * v297))) * v302)) / v301) * v303)) - (v1057 * v305)) / v174;
                    v369 = v305;
                    v879 = v1295;
                } else {
                    let v306 = v192 + v180;
                    let v307 = v188 / v306;
                    let v308 = v307 + v34;
                    let v309 = v126 / v308;
                    let v1098 = ((((v1078 - ((v1090 + (Lanes([0.0, v1068[0], 0.0, 0.0]))) * v307)) / v306) * v309) * v897) / v308;
                    let v310 = v5 + v192;
                    let v312 = v311 * v180;
                    let v314 = v309.powf(v313);
                    let v1104 = (v1068 * v311) * v314;
                    let v317 = v34 + v188;
                    let v319 = v318 * v180;
                    let v321 = v309.powf(v320);
                    let v1114 = (v1068 * v318) * v321;
                    let v323 = v317 - (v319 * v321);
                    let v324 = (v310 + (v312 * v314)) / v323;
                    let v1121 = ((v1090 + ((Lanes([0.0, v1104[0], 0.0, 0.0])) + ((v1098 * (v313 * (v309.powf(v1100)))) * v312))) - ((v1078 - ((Lanes([0.0, v1114[0], 0.0, 0.0])) + ((v1098 * (v320 * (v309.powf(v1110)))) * v319))) * v324)) / v323;
                    let v326 = v325 * v180;
                    let v328 = v324.powf(v327);
                    let v1127 = (v1068 * v325) * v328;
                    let v332 = v331 * v180;
                    let v334 = v324.powf(v333);
                    let v1137 = (v1068 * v331) * v334;
                    let v336 = v317 - (v332 * v334);
                    let v337 = (v310 + (v326 * v328)) / v336;
                    let v1144 = ((v1090 + ((Lanes([0.0, v1127[0], 0.0, 0.0])) + ((v1121 * (v327 * (v324.powf(v1123)))) * v326))) - ((v1078 - ((Lanes([0.0, v1137[0], 0.0, 0.0])) + ((v1121 * (v333 * (v324.powf(v1133)))) * v332))) * v337)) / v336;
                    let v339 = v338 * v180;
                    let v341 = v337.powf(v340);
                    let v1150 = (v1068 * v338) * v341;
                    let v345 = v344 * v180;
                    let v347 = v337.powf(v346);
                    let v1160 = (v1068 * v344) * v347;
                    let v349 = v317 - (v345 * v347);
                    let v350 = (v310 + (v339 * v341)) / v349;
                    let v1167 = ((v1090 + ((Lanes([0.0, v1150[0], 0.0, 0.0])) + ((v1144 * (v340 * (v337.powf(v1146)))) * v339))) - ((v1078 - ((Lanes([0.0, v1160[0], 0.0, 0.0])) + ((v1144 * (v346 * (v337.powf(v1156)))) * v345))) * v350)) / v349;
                    let v353 = v352 * v180;
                    let v355 = v350.powf(v354);
                    let v1173 = (v1068 * v352) * v355;
                    let v359 = v358 * v180;
                    let v361 = v350.powf(v360);
                    let v1183 = (v1068 * v358) * v361;
                    let v363 = v310 + (v359 * v361);
                    let v364 = ((v5 + v188) - (v353 * v355)) / v363;
                    let v365 = v1 * v177;
                    let v1192 = (v1061 * v1) * v364;
                    let v367 = (v365 * v364) / v174;
                    let v1198 = (((Lanes([0.0, v1192[0], 0.0, 0.0])) + ((((v1078 - ((Lanes([0.0, v1173[0], 0.0, 0.0])) + ((v1167 * (v354 * (v350.powf(v1169)))) * v353))) - ((v1090 + ((Lanes([0.0, v1183[0], 0.0, 0.0])) + ((v1167 * (v360 * (v350.powf(v1179)))) * v359))) * v364)) / v363) * v365)) - (v1057 * v367)) / v174;
                    v369 = v367;
                    v879 = v1198;
                }
                v368 = v369;
                v878 = v879;
            }
            let v370 = v6 * v368;
            let v1389 = v878 * v6;
            let v371 = v159 - v370;
            let v1390 = v877 - v1389;
            let v373 = v371 - v5;
            let v1391 = v1390 * v373;
            let v377 = ((v373 * v373) + v375).sqrt();
            let v380 = (v34 * ((v371 + v5) + v377)).sqrt();
            let v381 = v168 + v380;
            let v382 = v124 / v381;
            let v1404 = (v1049 - ((v1046 + (((v1390 + ((v1391 + v1391) * (v871 / (v941 * v377)))) * v34) * (v871 / (v941 * v380)))) * v382)) / v381;
            let v383 = v5 + v382;
            let v386 = v58 / (v384 * v4);
            let v387 = v119 - v159;
            let v1405 = v987 - v877;
            let v1407 = v389 - v871;
            let v1408 = v940 * v391;
            let v393 = v390 + (v391 * v83);
            let v1409 = v394 - v871;
            let v398 = -v397;
            let v403 = (v396 * (v15.powf(v398))) * v43;
            let v404 = (v6 * v104) / v403;
            let v1418 = (((v889 * (v398 * (v15.powf((v398 - v871))))) * v396) * v43) * v404;
            let v1421 = ((v957 * v6) - (Lanes([v1418[0], 0.0, 0.0]))) / v403;
            let v405 = v404 * v404;
            let v1422 = v1421 * v404;
            let v1423 = v1422 + v1422;
            let v406 = v6 * v404;
            let v1424 = v1421 * v6;
            let v407 = v6 + v404;
            let v408 = v406 * v368;
            let v1425 = v1424 * v368;
            let v1428 = (Lanes([0.0, v1425[0], v1425[1], v1425[2]])) + (v878 * v406);
            let v410 = (v368 * v368) + v368;
            let v1431 = v1424 * v410;
            let v1435 = Lanes([0.0, v1421[0], v1421[1], v1421[2]]);
            let v1437 = v1421 * v407;
            let v1438 = v1437 + v1437;
            let v417 = ((v407 * v407) + (v414 * v408)).sqrt();
            let v418 = (v407 + v408) + v417;
            let v419 = (v406 * v410) / v418;
            let v1448 = (((Lanes([0.0, v1431[0], v1431[1], v1431[2]])) + (((v878 * v370) + v878) * v406)) - (((v1435 + v1428) + (((Lanes([0.0, v1438[0], v1438[1], v1438[2]])) + (v1428 * v414)) * (v871 / (v941 * v417)))) * v419)) / v418;
            let v420 = v368 - v419;
            let v1449 = v878 - v1448;
            let v421 = v420 * v420;
            let v1450 = v1449 * v420;
            let v1451 = v1450 + v1450;
            let v423 = v6 - v422;
            let v426 = (v6 * v419) + (v419.ln());
            let v1456 = v1421 * v420;
            let v428 = v5 + (v404 * v420);
            let v430 = v404 * v423;
            let v1464 = (v1421 * v423) * v420;
            let v432 = v84 + (v430 * v420);
            let v435 = ((v6 * v405) * v423) * v423;
            let v1471 = (((v1423 * v6) * v423) * v423) * v421;
            let v437 = (v435 * v421) / v432;
            let v1478 = v1423 * v421;
            let v441 = ((v5 + v437) + (v405 * v421)).sqrt();
            let v442 = (v426 * v428) / v441;
            let v444 = ((v159 - v56) - v442) - v108;
            let v1490 = ((v877 - v1075) - ((((((v1448 * v6) + (v1448 * (v871 / v419))) * v428) + (((Lanes([0.0, v1456[0], v1456[1], v1456[2]])) + (v1449 * v404)) * v426)) - (((((((Lanes([0.0, v1471[0], v1471[1], v1471[2]])) + (v1451 * v435)) - (((Lanes([0.0, v1464[0], v1464[1], v1464[2]])) + (v1449 * v430)) * v437)) / v432) + ((Lanes([0.0, v1478[0], v1478[1], v1478[2]])) + (v1451 * v405))) * (v871 / (v941 * v441))) * v442)) / v441)) - v1077;
            let v446 = v444 - v126;
            let v1491 = v1490 * v446;
            let v450 = ((v446 * v446) + v448).sqrt();
            let v452 = v34 * ((v444 + v126) + v450);
            let v1497 = (v1490 + ((v1491 + v1491) * (v871 / (v941 * v450)))) * v34;
            let v454 = v453 / v422;
            let v458 = v368 + v5;
            let v459 = (v454 * ((v414 * v419) + v422)) / v458;
            let v460 = v106 - v108;
            let v461 = v414 * v459;
            let v1504 = ((((v1448 * v414) * v454) - (v878 * v459)) / v458) * v414;
            let v462 = v461 / v452;
            let v464 = (v5 + v462).sqrt();
            let v465 = v460 * v464;
            let v1511 = (v964 - v973) * v464;
            let v1514 = (Lanes([0.0, v1511[0], v1511[1], v1511[2]])) + ((((v1504 - (v1497 * v462)) / v452) * (v871 / (v941 * v464))) * v460);
            let v466 = v465 + v452;
            let v1516 = (v1514 + v1497) * v466;
            let v468 = v461 * v452;
            let v1520 = (v1504 * v452) + (v1497 * v461);
            let v470 = ((v466 * v466) + v468).sqrt();
            let v471 = v465 - v452;
            let v1526 = (v1514 - v1497) * v471;
            let v474 = ((v471 * v471) + v468).sqrt();
            let v477 = (v34 * (v470 - v474)) + v108;
            let v1534 = (((((v1516 + v1516) + v1520) * (v871 / (v941 * v470))) - (((v1526 + v1526) + v1520) * (v871 / (v941 * v474)))) * v34) + v1077;
            let v481 = ((v34 * v404) * v43) / v480;
            let v482 = v106 - v477;
            let v483 = v481 * v482;
            let v1540 = (((v1421 * v34) * v43) / v480) * v482;
            let v1543 = (Lanes([0.0, v1540[0], v1540[1], v1540[2]])) + (((Lanes([0.0, v964[0], v964[1], v964[2]])) - v1534) * v481);
            let v486 = v480 / (v43 - (v6 * v480));
            let v488 = v487 * v480;
            let v1544 = v1543 * v483;
            let v491 = v6 * v486;
            let v495 = (((v483 * v483) + (v491 * v483)) + v5).sqrt();
            let v497 = v486 + v5;
            let v498 = ((v486 + v483) + v495) / v497;
            let v500 = v488 * (v498.ln());
            let v1555 = (((v1543 + (((v1544 + v1544) + (v1543 * v491)) * (v871 / (v941 * v495)))) / v497) * (v871 / v498)) * v488;
            let v501 = v187 - v477;
            let v1556 = v1076 - v1534;
            let v502 = (v6 * v383) * v58;
            let v503 = v502 * v41;
            let v1559 = v904 * v502;
            let v1561 = (((v1404 * v6) * v58) * v41) + (Lanes([0.0, v1559[0], 0.0, 0.0]));
            let v504 = v58 * v383;
            let v505 = v1 / v504;
            let v1567 = v1061 * v505;
            let v507 = (v505 * v177) / v41;
            let v1570 = v904 * v507;
            let v1573 = (((((((v1404 * v58) * v505) * v897) / v504) * v177) + (Lanes([0.0, v1567[0], 0.0, 0.0]))) - (Lanes([0.0, v1570[0], 0.0, 0.0]))) / v41;
            let v508 = if v501 <= v0 { 1.0 } else { 0.0 };
            let v682: f64;
            let v880: Lanes<4>;
            if v508 != 0.0 {
                let v509 = v501.exp();
                let v510 = v133 / v509;
                let v1782 = (((v1556 * v509) * v510) * v897) / v509;
                let v513 = v512 * v180;
                let v515 = v510.powf(v514);
                let v1789 = (v1068 * v512) * v515;
                let v518 = v5 + v501;
                let v522 = v521 * v180;
                let v524 = v510.powf(v523);
                let v1802 = (v1068 * v521) * v524;
                let v526 = (v518 + (v510.ln())) - (v522 * v524);
                let v527 = ((v510 + v507) + (v513 * v515)) / v526;
                let v1809 = (((v1782 + v1573) + ((Lanes([0.0, v1789[0], 0.0, 0.0])) + ((v1782 * (v514 * (v510.powf(v1785)))) * v513))) - (((v1556 + (v1782 * (v871 / v510))) - ((Lanes([0.0, v1802[0], 0.0, 0.0])) + ((v1782 * (v523 * (v510.powf(v1798)))) * v522))) * v527)) / v526;
                let v530 = v529 * v180;
                let v532 = v527.powf(v531);
                let v1816 = (v1068 * v529) * v532;
                let v538 = v537 * v180;
                let v540 = v527.powf(v539);
                let v1829 = (v1068 * v537) * v540;
                let v542 = (v518 + (v527.ln())) - (v538 * v540);
                let v543 = ((v527 + v507) + (v530 * v532)) / v542;
                let v1836 = (((v1809 + v1573) + ((Lanes([0.0, v1816[0], 0.0, 0.0])) + ((v1809 * (v531 * (v527.powf(v1812)))) * v530))) - (((v1556 + (v1809 * (v871 / v527))) - ((Lanes([0.0, v1829[0], 0.0, 0.0])) + ((v1809 * (v539 * (v527.powf(v1825)))) * v538))) * v543)) / v542;
                let v547 = v546 * v180;
                let v549 = v543.powf(v548);
                let v1845 = (v1068 * v546) * v549;
                let v554 = v553 * v180;
                let v556 = v543.powf(v555);
                let v1856 = (v1068 * v553) * v556;
                let v558 = (v543 + v507) + (v554 * v556);
                let v559 = ((v518 + (v543.ln())) - (v547 * v549)) / v558;
                let v560 = v1 * v177;
                let v1865 = (v1061 * v1) * v559;
                let v562 = (v560 * v559) / v503;
                let v1871 = (((Lanes([0.0, v1865[0], 0.0, 0.0])) + (((((v1556 + (v1836 * (v871 / v543))) - ((Lanes([0.0, v1845[0], 0.0, 0.0])) + ((v1836 * (v548 * (v543.powf(v1841)))) * v547))) - (((v1836 + v1573) + ((Lanes([0.0, v1856[0], 0.0, 0.0])) + ((v1836 * (v555 * (v543.powf(v1852)))) * v554))) * v559)) / v558) * v560)) - (v1561 * v562)) / v503;
                v682 = v562;
                v880 = v1871;
            } else {
                let v563 = if v501 < v248 { 1.0 } else { 0.0 };
                let v683: f64;
                let v881: Lanes<4>;
                if v563 != 0.0 {
                    let v564 = v507 + v180;
                    let v565 = v501 / v564;
                    let v566 = v565 + v34;
                    let v567 = v5 / v566;
                    let v1689 = ((((v1556 - ((v1573 + (Lanes([0.0, v1068[0], 0.0, 0.0]))) * v565)) / v564) * v567) * v897) / v566;
                    let v570 = v569 * v180;
                    let v572 = v567.powf(v571);
                    let v1696 = (v1068 * v569) * v572;
                    let v575 = v5 + v501;
                    let v579 = v578 * v180;
                    let v581 = v567.powf(v580);
                    let v1709 = (v1068 * v578) * v581;
                    let v583 = (v575 + (v567.ln())) - (v579 * v581);
                    let v584 = ((v567 + v507) + (v570 * v572)) / v583;
                    let v1716 = (((v1689 + v1573) + ((Lanes([0.0, v1696[0], 0.0, 0.0])) + ((v1689 * (v571 * (v567.powf(v1692)))) * v570))) - (((v1556 + (v1689 * (v871 / v567))) - ((Lanes([0.0, v1709[0], 0.0, 0.0])) + ((v1689 * (v580 * (v567.powf(v1705)))) * v579))) * v584)) / v583;
                    let v587 = v586 * v180;
                    let v589 = v584.powf(v588);
                    let v1723 = (v1068 * v586) * v589;
                    let v595 = v594 * v180;
                    let v597 = v584.powf(v596);
                    let v1736 = (v1068 * v594) * v597;
                    let v599 = (v575 + (v584.ln())) - (v595 * v597);
                    let v600 = ((v584 + v507) + (v587 * v589)) / v599;
                    let v1743 = (((v1716 + v1573) + ((Lanes([0.0, v1723[0], 0.0, 0.0])) + ((v1716 * (v588 * (v584.powf(v1719)))) * v587))) - (((v1556 + (v1716 * (v871 / v584))) - ((Lanes([0.0, v1736[0], 0.0, 0.0])) + ((v1716 * (v596 * (v584.powf(v1732)))) * v595))) * v600)) / v599;
                    let v604 = v603 * v180;
                    let v606 = v600.powf(v605);
                    let v1752 = (v1068 * v603) * v606;
                    let v611 = v610 * v180;
                    let v613 = v600.powf(v612);
                    let v1763 = (v1068 * v610) * v613;
                    let v615 = (v600 + v507) + (v611 * v613);
                    let v616 = ((v575 + (v600.ln())) - (v604 * v606)) / v615;
                    let v617 = v1 * v177;
                    let v1772 = (v1061 * v1) * v616;
                    let v619 = (v617 * v616) / v503;
                    let v1778 = (((Lanes([0.0, v1772[0], 0.0, 0.0])) + (((((v1556 + (v1743 * (v871 / v600))) - ((Lanes([0.0, v1752[0], 0.0, 0.0])) + ((v1743 * (v605 * (v600.powf(v1748)))) * v604))) - (((v1743 + v1573) + ((Lanes([0.0, v1763[0], 0.0, 0.0])) + ((v1743 * (v612 * (v600.powf(v1759)))) * v611))) * v616)) / v615) * v617)) - (v1561 * v619)) / v503;
                    v683 = v619;
                    v881 = v1778;
                } else {
                    let v620 = v507 + v180;
                    let v621 = v501 / v620;
                    let v622 = v621 + v34;
                    let v623 = v126 / v622;
                    let v1581 = ((((v1556 - ((v1573 + (Lanes([0.0, v1068[0], 0.0, 0.0]))) * v621)) / v620) * v623) * v897) / v622;
                    let v624 = v5 + v507;
                    let v626 = v625 * v180;
                    let v628 = v623.powf(v627);
                    let v1587 = (v1068 * v625) * v628;
                    let v631 = v34 + v501;
                    let v633 = v632 * v180;
                    let v635 = v623.powf(v634);
                    let v1597 = (v1068 * v632) * v635;
                    let v637 = v631 - (v633 * v635);
                    let v638 = (v624 + (v626 * v628)) / v637;
                    let v1604 = ((v1573 + ((Lanes([0.0, v1587[0], 0.0, 0.0])) + ((v1581 * (v627 * (v623.powf(v1583)))) * v626))) - ((v1556 - ((Lanes([0.0, v1597[0], 0.0, 0.0])) + ((v1581 * (v634 * (v623.powf(v1593)))) * v633))) * v638)) / v637;
                    let v640 = v639 * v180;
                    let v642 = v638.powf(v641);
                    let v1610 = (v1068 * v639) * v642;
                    let v646 = v645 * v180;
                    let v648 = v638.powf(v647);
                    let v1620 = (v1068 * v645) * v648;
                    let v650 = v631 - (v646 * v648);
                    let v651 = (v624 + (v640 * v642)) / v650;
                    let v1627 = ((v1573 + ((Lanes([0.0, v1610[0], 0.0, 0.0])) + ((v1604 * (v641 * (v638.powf(v1606)))) * v640))) - ((v1556 - ((Lanes([0.0, v1620[0], 0.0, 0.0])) + ((v1604 * (v647 * (v638.powf(v1616)))) * v646))) * v651)) / v650;
                    let v653 = v652 * v180;
                    let v655 = v651.powf(v654);
                    let v1633 = (v1068 * v652) * v655;
                    let v659 = v658 * v180;
                    let v661 = v651.powf(v660);
                    let v1643 = (v1068 * v658) * v661;
                    let v663 = v631 - (v659 * v661);
                    let v664 = (v624 + (v653 * v655)) / v663;
                    let v1650 = ((v1573 + ((Lanes([0.0, v1633[0], 0.0, 0.0])) + ((v1627 * (v654 * (v651.powf(v1629)))) * v653))) - ((v1556 - ((Lanes([0.0, v1643[0], 0.0, 0.0])) + ((v1627 * (v660 * (v651.powf(v1639)))) * v659))) * v664)) / v663;
                    let v667 = v666 * v180;
                    let v669 = v664.powf(v668);
                    let v1656 = (v1068 * v666) * v669;
                    let v673 = v672 * v180;
                    let v675 = v664.powf(v674);
                    let v1666 = (v1068 * v672) * v675;
                    let v677 = v624 + (v673 * v675);
                    let v678 = ((v5 + v501) - (v667 * v669)) / v677;
                    let v679 = v1 * v177;
                    let v1675 = (v1061 * v1) * v678;
                    let v681 = (v679 * v678) / v503;
                    let v1681 = (((Lanes([0.0, v1675[0], 0.0, 0.0])) + ((((v1556 - ((Lanes([0.0, v1656[0], 0.0, 0.0])) + ((v1650 * (v668 * (v664.powf(v1652)))) * v667))) - ((v1573 + ((Lanes([0.0, v1666[0], 0.0, 0.0])) + ((v1650 * (v674 * (v664.powf(v1662)))) * v673))) * v678)) / v677) * v679)) - (v1561 * v681)) / v503;
                    v683 = v681;
                    v881 = v1681;
                }
                v682 = v683;
                v880 = v881;
            }
            let v685 = (v159 - v368) - v682;
            let v1873 = (v877 - v878) - v880;
            let v687 = v685 - v5;
            let v1874 = v1873 * v687;
            let v691 = ((v687 * v687) + v689).sqrt();
            let v694 = (v34 * ((v685 + v5) + v691)).sqrt();
            let v695 = v168 + v694;
            let v696 = v124 / v695;
            let v1887 = (v1049 - ((v1046 + (((v1873 + ((v1874 + v1874) * (v871 / (v941 * v691)))) * v34) * (v871 / (v941 * v694)))) * v696)) / v695;
            let v697 = v5 + v696;
            let v698 = v368 - v682;
            let v1888 = v878 - v880;
            let v699 = v698 * v698;
            let v1889 = v1888 * v698;
            let v700 = v458 + v682;
            let v1891 = v878 + v880;
            let v701 = v5 / v700;
            let v1894 = ((v1891 * v701) * v897) / v700;
            let v702 = v699 * v701;
            let v1897 = ((v1889 + v1889) * v701) + (v1894 * v699);
            let v703 = v697 - v5;
            let v706 = (v368 + v682) + (v33 * v702);
            let v708 = v387 - (v703 * v706);
            let v709 = v33 * v697;
            let v1904 = v1887 * v33;
            let v710 = v702 * v701;
            let v1907 = (v1897 * v701) + (v1894 * v702);
            let v718 = v34 * ((v5 + (v712 * v368)) + (v715 * v682));
            let v720 = (v370 + v682) + (v718 * v710);
            let v728 = v34 * ((v5 + (v715 * v368)) + (v712 * v682));
            let v730 = (v368 + (v6 * v682)) + (v728 * v710);
            let v732 = v104 * v708;
            let v1933 = v957 * v708;
            let v1936 = (Lanes([0.0, v1933[0], v1933[1], v1933[2]])) + ((v1405 - ((v1887 * v706) + ((v1891 + (v1897 * v33)) * v703))) * v104);
            let v1937 = v1936 * v732;
            let v736 = ((v732 * v732) + v734).sqrt();
            let v738 = v34 * (v732 + v736);
            let v1943 = (v1936 + ((v1937 + v1937) * (v871 / (v941 * v736)))) * v34;
            let v739 = (v709 * v720) + (v709 * v730);
            let v740 = v104 * v739;
            let v1945 = v957 * v739;
            let v1948 = (Lanes([0.0, v1945[0], v1945[1], v1945[2]])) + ((((v1904 * v720) + (((v1389 + v880) + (((((v878 * v712) + (v880 * v715)) * v34) * v710) + (v1907 * v718))) * v709)) + ((v1904 * v730) + (((v878 + (v880 * v6)) + (((((v878 * v715) + (v880 * v712)) * v34) * v710) + (v1907 * v728))) * v709))) * v104);
            let v743 = v386 * (v738 + (v35 * v740));
            let v744 = v740 / v738;
            let v746 = v34 * (v5 + v744);
            let v747 = v746.powf(v389);
            let v748 = v743.powf(v394);
            let v1962 = v1408 * v748;
            let v750 = v395 / v747;
            let v1969 = ((Lanes([0.0, 0.0, v1962[0], v1962[1]])) + ((((v1943 + (v1948 * v35)) * v386) * (v394 * (v743.powf(v1409)))) * v393)) + (((((((v1948 - (v1943 * v744)) / v738) * v34) * (v389 * (v746.powf(v1407)))) * v750) * v897) / v747);
            let v752 = v5 + ((v393 * v748) + v750);
            let v754 = v752 - v5;
            let v1970 = v1969 * v754;
            let v758 = ((v754 * v754) + v756).sqrt();
            let v760 = v34 * ((v752 + v5) + v758);
            let v1976 = (v1969 + ((v1970 + v1970) * (v871 / (v941 * v758)))) * v34;
            let v761 = v404 / v760;
            let v762 = v6 * v761;
            let v763 = v762 * v698;
            let v1983 = ((((v1435 - (v1976 * v761)) / v760) * v6) * v698) + (v1888 * v762);
            let v764 = v763 * v763;
            let v1984 = v1983 * v763;
            let v766 = (v5 + v764).sqrt();
            let v1988 = (v1984 + v1984) * (v871 / (v941 * v766));
            let v767 = if v763 != v0 { 1.0 } else { 0.0 };
            let v776: f64;
            let v882: Lanes<4>;
            if v767 != 0.0 {
                let v768 = v5 / v763;
                let v769 = v763.asinh();
                let v772 = v34 * (v766 + (v768 * v769));
                let v2005 = (v1988 + (((((v1983 * v768) * v897) / v763) * v769) + ((v1983 * (v871 / ((v871 + v764).sqrt()))) * v768))) * v34;
                v776 = v772;
                v882 = v2005;
            } else {
                let v773 = v5 / v766;
                let v775 = v34 * (v766 + v773);
                let v1993 = (v1988 + (((v1988 * v773) * v897) / v766)) * v34;
                v776 = v775;
                v882 = v1993;
            }
            let v777 = v760 * v776;
            let v2008 = (v1976 * v776) + (v882 * v760);
            let v778 = v39 / v777;
            let v780 = v6 * v779;
            let v781 = v780 * v697;
            let v784 = v43 - v500;
            let v2018 = v1555 * v897;
            let v785 = ((v781 * v778) * v42) / v784;
            let v786 = v785 * v58;
            let v787 = v786 * v104;
            let v2024 = v957 * v786;
            let v788 = v787 * v104;
            let v2028 = v957 * v787;
            let v789 = v698 * v700;
            let v790 = v788 * v789;
            let v2036 = ((((((((((((v1887 * v780) * v778) + ((((Lanes([0.0, v903[0], 0.0, 0.0])) - (v2008 * v778)) / v777) * v781)) * v42) - (v2018 * v785)) / v784) * v58) * v104) + (Lanes([0.0, v2024[0], v2024[1], v2024[2]]))) * v104) + (Lanes([0.0, v2028[0], v2028[1], v2028[2]]))) * v789) + (((v1888 * v700) + (v1891 * v698)) * v788);
            let v796 = v5 - (v793 * (v15 - v5));
            let v2038 = (v889 * v793) * v897;
            let v805 = (v1 * v791) * v42;
            let v807 = v805 * (v800 * (v15.powf(v801)));
            let v2049 = ((v889 * (v801 * (v15.powf((v801 - v871))))) * v800) * v805;
            let v808 = v806 / v807;
            let v809 = v808 * v796;
            let v2055 = ((((v2049 * v808) * v897) / v807) * v796) + (v2038 * v808);
            let v811 = v810 / v807;
            let v812 = v811 * v796;
            let v2061 = ((((v2049 * v811) * v897) / v807) * v796) + (v2038 * v811);
            let v813 = v805 * (v792 * (v796.powf(v797)));
            let v814 = v790 / v813;
            let v2063 = (((v2038 * (v797 * (v796.powf((v797 - v871))))) * v792) * v805) * v814;
            let v2066 = (v2036 - (Lanes([0.0, v2063[0], 0.0, 0.0]))) / v813;
            let v816 = if v814 >= v815 { 1.0 } else { 0.0 };
            let v817: f64;
            let v883: Lanes<4>;
            if v816 != 0.0 {
                v817 = v815;
                v883 = v2067;
            } else {
                v817 = v814;
                v883 = v2066;
            }
            let v820 = v5 - (v817.powf(v818));
            let v821 = v5 / v818;
            let v822 = v820.powf(v821);
            let v2076 = ((v883 * (v818 * (v817.powf((v818 - v871))))) * v897) * (v821 * (v820.powf((v821 - v871))));
            let v824 = v823 / v42;
            let v826 = v825 / v42;
            let v827 = v809 / v822;
            let v828 = v812 / v822;
            let v2086 = (v889 * v829) * v826;
            let v833 = (v826 * (v5 + (v829 * v15))) + v827;
            let v2088 = (Lanes([0.0, v2086[0], 0.0, 0.0])) + (((Lanes([0.0, v2055[0], 0.0, 0.0])) - (v2076 * v827)) / v822);
            let v2090 = (v889 * v834) * v824;
            let v838 = (v824 * (v5 + (v834 * v15))) + v828;
            let v2092 = (Lanes([0.0, v2090[0], 0.0, 0.0])) + (((Lanes([0.0, v2061[0], 0.0, 0.0])) - (v2076 * v828)) / v822);
            let v839 = v36 / v777;
            let v842 = ((v839 * v58) * v42) / v784;
            let v843 = v842 * v740;
            let v844 = v838 + v833;
            let v846 = v5 + (v843 * v844);
            let v847 = v790 / v846;
            let v2110 = (v2036 - ((((((((((((v2008 * v839) * v897) / v777) * v58) * v42) - (v2018 * v842)) / v784) * v740) + (v1948 * v842)) * v844) + ((v2092 + v2088) * v843)) * v847)) / v846;
            let v848 = v833 * v847;
            let v2113 = (v2088 * v847) + (v2110 * v833);
            let v849 = v838 * v847;
            let v2116 = (v2092 * v847) + (v2110 * v838);
            let v852 = if v851 != v0 { 1.0 } else { 0.0 };
            let v866: f64;
            let v867: f64;
            let v868: f64;
            let v869: f64;
            let v870: f64;
            let v884: Lanes<1>;
            let v885: Lanes<4>;
            let v886: Lanes<1>;
            let v887: Lanes<1>;
            let v888: Lanes<1>;
            if v852 != 0.0 {
                let v2120 = v928 * v847;
                let v854 = v847 * v847;
                let v2123 = v2110 * v847;
                let v2124 = v2123 + v2123;
                let v859 = -(((v847 * v64) + (v854 * v833)) + (v854 * v838));
                let v2133 = ((((v2110 * v64) + (Lanes([0.0, 0.0, v2120[0], v2120[1]]))) + ((v2124 * v833) + (v2088 * v854))) + ((v2124 * v838) + (v2092 * v854))) * v897;
                let v861 = v860 * v12;
                let v2134 = v872 * v860;
                let v862 = ddt(4922, v861);
                let v2136 = v2134 * v2135;
                let v863 = v12 / v851;
                let v2137 = v872 / v851;
                v866 = v862;
                v867 = v859;
                v868 = v863;
                v869 = v0;
                v870 = v861;
                v884 = v2136;
                v885 = v2133;
                v886 = v2137;
                v887 = v2118;
                v888 = v2134;
            } else {
                let v865 = v12 * v864;
                let v2117 = v872 * v864;
                v866 = v0;
                v867 = v0;
                v868 = v0;
                v869 = v865;
                v870 = v0;
                v884 = v2118;
                v885 = v2067;
                v886 = v2118;
                v887 = v2117;
                v888 = v2118;
            }
            let v2138 = v2116[0];
            let v2139 = v2116[1];
            let v2140 = v2116[2];
            let v2141 = v2116[3];
            let v2142 = v2110[0];
            let v2143 = v2110[1];
            let v2144 = v2110[2];
            let v2145 = v2110[3];
            let v2146 = v2113[0];
            let v2147 = v2113[1];
            let v2148 = v2113[2];
            let v2149 = v2113[3];
            let v2150 = v884[0];
            let v2151 = v885[0];
            let v2152 = v885[1];
            let v2153 = v885[2];
            let v2154 = v885[3];
            let v2155 = v886[0];
            let v2156 = v887[0];
            let v2157 = v888[0];
        stamper.stamp_potential_branch_local(Some(6), Some(2), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<4, 0>(
            0,
            v849,
            [1, 4, 5, 6],
            [v2138, v2139, v2140, v2141],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (v847),
            [1, 4, 5, 6],
            [v2142, v2143, v2144, v2145],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(0), Some(5), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<4, 0>(
            1,
            v848,
            [1, 4, 5, 6],
            [v2146, v2147, v2148, v2149],
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
            [v2150],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            None,
            multiplicity * (v867),
            [1, 4, 5, 6],
            [v2151, v2152, v2153, v2154],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (v868),
            [4],
            [v2155],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (v869),
            [4],
            [v2156],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = v849;
        self.canonical_reactive[1] = v847;
        self.canonical_reactive[2] = v848;
        self.canonical_reactive[3] = v850;
        self.canonical_reactive[4] = v870;
        self.canonical_reactive[5] = v2157;
        self.canonical_reactive[6] = v867;
        self.canonical_reactive[7] = v868;
        self.canonical_reactive[8] = v869;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            None,
            &[4],
            &[cached[5]],
            &[],
            &[],
            multiplicity,
        );
    }

}

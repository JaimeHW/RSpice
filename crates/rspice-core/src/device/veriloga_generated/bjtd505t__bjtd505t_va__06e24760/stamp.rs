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
            let slot = match operator { 12393 => 0usize, 12461 => 1usize, 12467 => 2usize, 12477 => 3usize, 12483 => 4usize, 12491 => 5usize, 12499 => 6usize, 12519 => 7usize, 12538 => 8usize, 12791 => 9usize, _ => usize::MAX };
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
            let v1 = parameters[3];
            let v2 = 1e0f64;
            let v4 = 7.03e7f64;
            let v5 = 1.23e8f64;
            let v6 = 1.58e8f64;
            let v7 = 2.04e8f64;
            let v8 = parameters[32];
            let v10 = parameters[4];
            let v11 = 2.7315e2f64;
            let v13 = temperature;
            let v14 = parameters[0];
            let v17 = parameters[141];
            let v19 = 1e-12f64;
            let v21 = parameters[1];
            let v24 = 1e-3f64;
            let v25 = 2e0f64;
            let v26 = parameters[66];
            let v30 = parameters[113];
            let v31 = parameters[114];
            let v34 = parameters[115];
            let v38 = 5e-2f64;
            let v40 = 1e-1f64;
            let v55 = parameters[65];
            let v57 = parameters[70];
            let v58 = parameters[71];
            let v62 = parameters[116];
            let v63 = parameters[117];
            let v66 = parameters[118];
            let v86 = parameters[82];
            let v89 = node_potentials[3];
            let v95 = parameters[124];
            let v104 = 8.617086918058125e-5f64;
            let v152 = 3e0f64;
            let v153 = -3e0f64;
            let v159 = parameters[104];
            let v176 = -3e0f64;
            let v179 = parameters[63];
            let v182 = parameters[109];
            let v199 = -3e0f64;
            let v202 = parameters[79];
            let v220 = -3e0f64;
            let v240 = -3e0f64;
            let v259 = -3e0f64;
            let v262 = parameters[26];
            let v265 = parameters[108];
            let v290 = parameters[64];
            let v292 = parameters[74];
            let v300 = parameters[69];
            let v303 = parameters[53];
            let v304 = parameters[96];
            let v309 = parameters[55];
            let v310 = parameters[97];
            let v311 = parameters[95];
            let v316 = parameters[54];
            let v317 = parameters[100];
            let v322 = parameters[56];
            let v323 = parameters[101];
            let v327 = parameters[57];
            let v328 = parameters[103];
            let v332 = parameters[58];
            let v334 = parameters[59];
            let v335 = parameters[98];
            let v339 = parameters[121];
            let v341 = parameters[9];
            let v360 = 6.931471805599453e-4f64;
            let v362 = parameters[122];
            let v364 = parameters[10];
            let v383 = 6.931471805599453e-4f64;
            let v385 = parameters[42];
            let v386 = parameters[123];
            let v390 = 1e-6f64;
            let v393 = 5e-1f64;
            let v394 = 5e-7f64;
            let v403 = parameters[8];
            let v404 = 4e0f64;
            let v407 = parameters[120];
            let v419 = parameters[11];
            let v424 = parameters[29];
            let v425 = parameters[102];
            let v430 = parameters[19];
            let v431 = 6e0f64;
            let v432 = parameters[20];
            let v438 = parameters[112];
            let v444 = parameters[30];
            let v445 = parameters[31];
            let v456 = parameters[15];
            let v460 = parameters[16];
            let v464 = parameters[110];
            let v470 = parameters[17];
            let v471 = parameters[18];
            let v478 = parameters[23];
            let v480 = parameters[24];
            let v481 = parameters[106];
            let v487 = parameters[27];
            let v488 = parameters[105];
            let v493 = parameters[25];
            let v494 = parameters[107];
            let v500 = parameters[28];
            let v506 = parameters[111];
            let v511 = parameters[21];
            let v512 = parameters[22];
            let v521 = parameters[136];
            let v522 = parameters[137];
            let v530 = parameters[142];
            let v533 = parameters[144];
            let v539 = -5e-1f64;
            let v542 = parameters[34];
            let v551 = parameters[33];
            let v563 = -5e-1f64;
            let v566 = parameters[36];
            let v575 = parameters[35];
            let v587 = parameters[13];
            let v590 = parameters[12];
            let v593 = parameters[85];
            let v598 = parameters[119];
            let v603 = parameters[86];
            let v609 = parameters[87];
            let v614 = parameters[88];
            let v619 = parameters[89];
            let v620 = parameters[99];
            let v625 = 3e2f64;
            let v627 = 5.25e2f64;
            let v630 = 7.2e-4f64;
            let v633 = 1.6e-6f64;
            let v638 = 1.081e0f64;
            let v640 = parameters[91];
            let v642 = parameters[133];
            let v644 = parameters[135];
            let v656 = node_potentials[6];
            let v657 = node_potentials[7];
            let v660 = node_potentials[8];
            let v663 = node_potentials[4];
            let v666 = node_potentials[5];
            let v673 = node_potentials[2];
            let v676 = node_potentials[1];
            let v681 = node_potentials[0];
            let v684 = node_potentials[10];
            let v687 = node_potentials[9];
            let v699 = parameters[138];
            let v779 = parameters[140];
            let v790 = 1e2f64;
            let v806 = 2e-1f64;
            let v821 = parameters[61];
            let v822 = parameters[60];
            let v832 = parameters[62];
            let v847 = -1e0f64;
            let v890 = parameters[139];
            let v908 = parameters[72];
            let v924 = 1e-5f64;
            let v928 = 1e-40f64;
            let v944 = -1e0f64;
            let v975 = parameters[73];
            let v983 = -1e0f64;
            let v1007 = parameters[75];
            let v1062 = 1.0000000000000002e-2f64;
            let v1066 = 5.000000000000001e-3f64;
            let v1080 = parameters[14];
            let v1086 = 1e-4f64;
            let v1100 = parameters[143];
            let v1111 = parameters[145];
            let v1126 = parameters[146];
            let v1149 = 1e3f64;
            let v1151 = 4e1f64;
            let v1154 = 2.3538526683702e17f64;
            let v1182 = parameters[92];
            let v1284 = 1e-30f64;
            let v1287 = -2e0f64;
            let v1303 = 1.6666666666666666e-1f64;
            let v1309 = -1e-3f64;
            let v1325 = 3.333333333333333e-1f64;
            let v1327 = 2.5e-1f64;
            let v1362 = -2e0f64;
            let v1383 = -1e-3f64;
            let v1436 = parameters[5];
            let v1459 = 1.21e-2f64;
            let v1462 = 6.05e-3f64;
            let v1478 = parameters[83];
            let v1481 = 1e-6f64;
            let v1482 = 1e-12f64;
            let v1483 = -1e0f64;
            let v1485 = -1e0f64;
            let v1488 = -1e0f64;
            let v1491 = 5e-13f64;
            let v1494 = -1e0f64;
            let v1500 = -1e0f64;
            let v1504 = parameters[81];
            let v1508 = parameters[80];
            let v1538 = 1.0000000000000002e-2f64;
            let v1541 = 5.000000000000001e-3f64;
            let v1563 = parameters[38];
            let v1565 = parameters[43];
            let v1568 = parameters[41];
            let v1581 = parameters[40];
            let v1590 = parameters[39];
            let v1597 = parameters[45];
            let v1599 = parameters[44];
            let v1607 = parameters[7];
            let v1627 = parameters[46];
            let v1657 = 1e-7f64;
            let v1683 = parameters[47];
            let v1687 = parameters[48];
            let v1691 = parameters[51];
            let v1695 = parameters[50];
            let v1710 = parameters[49];
            let v1734 = parameters[52];
            let v1831 = parameters[67];
            let v1860 = parameters[76];
            let v1931 = parameters[84];
            let v1954 = parameters[78];
            let v1964 = parameters[90];
            let v2027 = parameters[6];
            let v2055 = parameters[94];
            let v2060 = parameters[93];
            let v2081 = -1e0f64;
            let v2091 = parameters[134];
            let v2097 = parameters[132];
            let v2121 = -1e0f64;
            let v2144 = parameters[68];
            let v2149 = parameters[77];
            let v2176 = 0e0f64;
            let v2180 = 0e0f64;
            let v2183 = parameters[129];
            let v2193 = parameters[130];
            let v2198 = parameters[131];
            let v2209 = 0e0f64;
            let v2210 = node_potentials[11];
            let v2216 = 0e0f64;
            let v2217 = 0e0f64;
            let v2218 = 0e0f64;
            let v2219 = 0e0f64;
            let v2220 = 0e0f64;
            let v2221 = 0e0f64;
            let v2222 = 0e0f64;
            let v2223 = 0e0f64;
            let v2224 = 0e0f64;
            let v2225 = 0e0f64;
            let v2226 = 0e0f64;
            let v2227 = 0e0f64;
            let v2228 = 0e0f64;
            let v2229 = 0e0f64;
            let v2230 = 0e0f64;
            let v2231 = 0e0f64;
            let v2232 = 0e0f64;
            let v2233 = 0e0f64;
            let v2234 = 0e0f64;
            let v2235 = 0e0f64;
            let v2236 = 0e0f64;
            let v2237 = 0e0f64;
            let v2238 = 0e0f64;
            let v2239 = 0e0f64;
            let v2270 = 1e0f64;
            let v2271 = Lanes([1e0f64; 1]);
            let v2272 = Lanes([1e0f64; 1]);
            let v2273 = Lanes([1e0f64; 1]);
            let v2274 = Lanes([1e0f64; 1]);
            let v2275 = Lanes([1e0f64; 1]);
            let v2276 = Lanes([1e0f64; 1]);
            let v2277 = Lanes([1e0f64; 1]);
            let v2278 = Lanes([1e0f64; 1]);
            let v2279 = Lanes([1e0f64; 1]);
            let v2280 = Lanes([1e0f64; 1]);
            let v2281 = Lanes([1e0f64; 1]);
            let v2282 = Lanes([1e0f64; 1]);
            let v2420 = -1e0f64;
            let v2657 = Lanes([0e0f64; 1]);
            let v2704 = 2e0f64;
            let v2818 = -1.5e0f64;
            let v2859 = -1.5e0f64;
            let v3102 = Lanes([0e0f64; 3]);
            let v3127 = 0e0f64;
            let v3148 = Lanes([0e0f64; 4]);
            let v3746 = Lanes([0e0f64; 3]);
            let v3977 = Lanes([0e0f64; 9]);
            let v4036 = Lanes([0e0f64; 3]);
            let v4118 = Lanes([0e0f64; 5]);
            let v4857 = Lanes([0e0f64; 6]);
            let v4932 = Lanes([0e0f64; 4]);
            let v4956 = ddt_scale();
            let v5021 = Lanes([0e0f64; 3]);
            let v5029 = Lanes([0e0f64; 3]);
            let v3 = if v1 == v2 { 1.0 } else { 0.0 };
            let v629: f64;
            let v1660: f64;
            if v3 != 0.0 {
                v629 = v5;
                v1660 = v4;
            } else {
                v629 = v7;
                v1660 = v6;
            }
            let v9 = v2 - v8;
            let v12 = v10 + v11;
            let v15 = v13 + v14;
            let v16 = ctx.simparam_or("gmin", v0);
            let v18 = if v17 == v0 { 1.0 } else { 0.0 };
            let v20: f64;
            if v18 != 0.0 {
                v20 = v19;
            } else {
                v20 = v17;
            }
            let v22 = v20 * v21;
            let v23 = v2 / v22;
            let v28 = v25.powf((v25 - v26));
            let v29 = v2 / v28;
            let v37 = v30 + (((v31 * v12) * v12) / (v12 + v34));
            let v41 = (v37 - v38) / v40;
            let v42 = if v37 < v38 { 1.0 } else { 0.0 };
            let v112: f64;
            if v42 != 0.0 {
                let v47 = v38 + (v40 * ((v2 + (v41.exp())).ln()));
                v112 = v47;
            } else {
                let v53 = v37 + (v40 * ((v2 + ((-v41).exp())).ln()));
                v112 = v53;
            }
            let v54 = v2 / v30;
            let v56 = v2 / v55;
            let v60 = v25.powf((v25 - v58));
            let v61 = v2 / v60;
            let v69 = v62 + (((v63 * v12) * v12) / (v12 + v66));
            let v71 = (v69 - v38) / v40;
            let v72 = if v69 < v38 { 1.0 } else { 0.0 };
            let v132: f64;
            if v72 != 0.0 {
                let v77 = v38 + (v40 * ((v2 + (v71.exp())).ln()));
                v132 = v77;
            } else {
                let v83 = v69 + (v40 * ((v2 + ((-v71).exp())).ln()));
                v132 = v83;
            }
            let v84 = v2 / v62;
            let v85 = v2 / v57;
            let v88 = v2 - (v2 / v86);
            let v90 = if v89 < v0 { 1.0 } else { 0.0 };
            let v94: f64;
            let v2283: Lanes<1>;
            if v90 != 0.0 {
                let v91 = v2 - v89;
                let v93 = -(v91.ln());
                let v2424 = ((v2271 * v2420) * (v2270 / v91)) * v2420;
                v94 = v93;
                v2283 = v2424;
            } else {
                v94 = v89;
                v2283 = v2271;
            }
            let v96 = if v94 < v95 { 1.0 } else { 0.0 };
            let v101: f64;
            let v2284: Lanes<1>;
            if v96 != 0.0 {
                v101 = v94;
                v2284 = v2283;
            } else {
                let v98 = v2 + (v94 - v95);
                let v2426 = v2283 * (v2270 / v98);
                let v100 = v95 + (v98.ln());
                v101 = v100;
                v2284 = v2426;
            }
            let v102 = v15 + v101;
            let v103 = v102 / v12;
            let v2427 = v2284 / v12;
            let v105 = v104 * v102;
            let v2428 = v2284 * v104;
            let v107 = v2 / v105;
            let v2431 = ((v2428 * v107) * v2420) / v105;
            let v109 = v107 - (v2 / (v104 * v12));
            let v110 = v102 - v12;
            let v111 = v103.ln();
            let v2433 = v2427 * (v2270 / v103);
            let v113 = v31 * v102;
            let v115 = v102 + v34;
            let v116 = (v113 * v102) / v115;
            let v117 = v112 - v116;
            let v2441 = (((((v2284 * v31) * v102) + (v2284 * v113)) - (v2284 * v116)) / v115) * v2420;
            let v119 = (v117 - v38) / v40;
            let v2442 = v2441 / v40;
            let v120 = if v117 < v38 { 1.0 } else { 0.0 };
            let v537: f64;
            let v2285: Lanes<1>;
            if v120 != 0.0 {
                let v121 = v119.exp();
                let v122 = v2 + v121;
                let v2452 = ((v2442 * v121) * (v2270 / v122)) * v40;
                let v125 = v38 + (v40 * (v122.ln()));
                v537 = v125;
                v2285 = v2452;
            } else {
                let v127 = (-v119).exp();
                let v128 = v2 + v127;
                let v131 = v117 + (v40 * (v128.ln()));
                let v2448 = v2441 + ((((v2442 * v2420) * v127) * (v2270 / v128)) * v40);
                v537 = v131;
                v2285 = v2448;
            }
            let v133 = v63 * v102;
            let v135 = v102 + v66;
            let v136 = (v133 * v102) / v135;
            let v137 = v132 - v136;
            let v2460 = (((((v2284 * v63) * v102) + (v2284 * v133)) - (v2284 * v136)) / v135) * v2420;
            let v139 = (v137 - v38) / v40;
            let v2461 = v2460 / v40;
            let v140 = if v137 < v38 { 1.0 } else { 0.0 };
            let v561: f64;
            let v2286: Lanes<1>;
            if v140 != 0.0 {
                let v141 = v139.exp();
                let v142 = v2 + v141;
                let v2471 = ((v2461 * v141) * (v2270 / v142)) * v40;
                let v145 = v38 + (v40 * (v142.ln()));
                v561 = v145;
                v2286 = v2471;
            } else {
                let v147 = (-v139).exp();
                let v148 = v2 + v147;
                let v151 = v137 + (v40 * (v148.ln()));
                let v2467 = v2460 + ((((v2461 * v2420) * v147) * (v2270 / v148)) * v40);
                v561 = v151;
                v2286 = v2467;
            }
            let v154 = v153 * v105;
            let v158 = v2 - v103;
            let v2478 = v2427 * v2420;
            let v161 = ((v154 * v111) + (v55 * v103)) + (v158 * v159);
            let v2480 = ((((v2428 * v153) * v111) + (v2433 * v154)) + (v2427 * v55)) + (v2478 * v159);
            let v163 = (v38 - v161) / v105;
            let v2484 = ((v2480 * v2420) - (v2428 * v163)) / v105;
            let v164 = if v38 < v161 { 1.0 } else { 0.0 };
            let v282: f64;
            let v2287: Lanes<1>;
            if v164 != 0.0 {
                let v165 = v163.exp();
                let v166 = v2 + v165;
                let v167 = v166.ln();
                let v169 = v161 + (v105 * v167);
                let v2498 = v2480 + ((v2428 * v167) + (((v2484 * v165) * (v2270 / v166)) * v105));
                v282 = v169;
                v2287 = v2498;
            } else {
                let v171 = (-v163).exp();
                let v172 = v2 + v171;
                let v173 = v172.ln();
                let v2491 = (v2428 * v173) + ((((v2484 * v2420) * v171) * (v2270 / v172)) * v105);
                let v175 = v38 + (v105 * v173);
                v282 = v175;
                v2287 = v2491;
            }
            let v177 = v176 * v105;
            let v183 = v158 * v182;
            let v2505 = v2478 * v182;
            let v184 = ((v177 * v111) + (v179 * v103)) + v183;
            let v2506 = ((((v2428 * v176) * v111) + (v2433 * v177)) + (v2427 * v179)) + v2505;
            let v186 = (v38 - v184) / v105;
            let v2510 = ((v2506 * v2420) - (v2428 * v186)) / v105;
            let v187 = if v38 < v184 { 1.0 } else { 0.0 };
            let v735: f64;
            let v2288: Lanes<1>;
            if v187 != 0.0 {
                let v188 = v186.exp();
                let v189 = v2 + v188;
                let v190 = v189.ln();
                let v192 = v184 + (v105 * v190);
                let v2524 = v2506 + ((v2428 * v190) + (((v2510 * v188) * (v2270 / v189)) * v105));
                v735 = v192;
                v2288 = v2524;
            } else {
                let v194 = (-v186).exp();
                let v195 = v2 + v194;
                let v196 = v195.ln();
                let v2517 = (v2428 * v196) + ((((v2510 * v2420) * v194) * (v2270 / v195)) * v105);
                let v198 = v38 + (v105 * v196);
                v735 = v198;
                v2288 = v2517;
            }
            let v200 = v199 * v105;
            let v205 = ((v200 * v111) + (v202 * v103)) + v183;
            let v2531 = ((((v2428 * v199) * v111) + (v2433 * v200)) + (v2427 * v202)) + v2505;
            let v207 = (v38 - v205) / v105;
            let v2535 = ((v2531 * v2420) - (v2428 * v207)) / v105;
            let v208 = if v38 < v205 { 1.0 } else { 0.0 };
            let v1962: f64;
            let v2289: Lanes<1>;
            if v208 != 0.0 {
                let v209 = v207.exp();
                let v210 = v2 + v209;
                let v211 = v210.ln();
                let v213 = v205 + (v105 * v211);
                let v2549 = v2531 + ((v2428 * v211) + (((v2535 * v209) * (v2270 / v210)) * v105));
                v1962 = v213;
                v2289 = v2549;
            } else {
                let v215 = (-v207).exp();
                let v216 = v2 + v215;
                let v217 = v216.ln();
                let v2542 = (v2428 * v217) + ((((v2535 * v2420) * v215) * (v2270 / v216)) * v105);
                let v219 = v38 + (v105 * v217);
                v1962 = v219;
                v2289 = v2542;
            }
            let v221 = v220 * v105;
            let v223 = v57 * v103;
            let v2554 = v2427 * v57;
            let v225 = ((v221 * v111) + v223) + v183;
            let v2556 = ((((v2428 * v220) * v111) + (v2433 * v221)) + v2554) + v2505;
            let v227 = (v38 - v225) / v105;
            let v2560 = ((v2556 * v2420) - (v2428 * v227)) / v105;
            let v228 = if v38 < v225 { 1.0 } else { 0.0 };
            let v294: f64;
            let v2290: Lanes<1>;
            if v228 != 0.0 {
                let v229 = v227.exp();
                let v230 = v2 + v229;
                let v231 = v230.ln();
                let v233 = v225 + (v105 * v231);
                let v2574 = v2556 + ((v2428 * v231) + (((v2560 * v229) * (v2270 / v230)) * v105));
                v294 = v233;
                v2290 = v2574;
            } else {
                let v235 = (-v227).exp();
                let v236 = v2 + v235;
                let v237 = v236.ln();
                let v2567 = (v2428 * v237) + ((((v2560 * v2420) * v235) * (v2270 / v236)) * v105);
                let v239 = v38 + (v105 * v237);
                v294 = v239;
                v2290 = v2567;
            }
            let v241 = v240 * v105;
            let v244 = ((v241 * v111) + v223) + v183;
            let v2580 = ((((v2428 * v240) * v111) + (v2433 * v241)) + v2554) + v2505;
            let v246 = (v38 - v244) / v105;
            let v2584 = ((v2580 * v2420) - (v2428 * v246)) / v105;
            let v247 = if v38 < v244 { 1.0 } else { 0.0 };
            let v284: f64;
            let v2291: Lanes<1>;
            if v247 != 0.0 {
                let v248 = v246.exp();
                let v249 = v2 + v248;
                let v250 = v249.ln();
                let v252 = v244 + (v105 * v250);
                let v2598 = v2580 + ((v2428 * v250) + (((v2584 * v248) * (v2270 / v249)) * v105));
                v284 = v252;
                v2291 = v2598;
            } else {
                let v254 = (-v246).exp();
                let v255 = v2 + v254;
                let v256 = v255.ln();
                let v2591 = (v2428 * v256) + ((((v2584 * v2420) * v254) * (v2270 / v255)) * v105);
                let v258 = v38 + (v105 * v256);
                v284 = v258;
                v2291 = v2591;
            }
            let v260 = v259 * v105;
            let v267 = ((v260 * v111) + (v262 * v103)) + (v158 * v265);
            let v2606 = ((((v2428 * v259) * v111) + (v2433 * v260)) + (v2427 * v262)) + (v2478 * v265);
            let v269 = (v38 - v267) / v105;
            let v2610 = ((v2606 * v2420) - (v2428 * v269)) / v105;
            let v270 = if v38 < v267 { 1.0 } else { 0.0 };
            let v1139: f64;
            let v2292: Lanes<1>;
            if v270 != 0.0 {
                let v271 = v269.exp();
                let v272 = v2 + v271;
                let v273 = v272.ln();
                let v275 = v267 + (v105 * v273);
                let v2624 = v2606 + ((v2428 * v273) + (((v2610 * v271) * (v2270 / v272)) * v105));
                v1139 = v275;
                v2292 = v2624;
            } else {
                let v277 = (-v269).exp();
                let v278 = v2 + v277;
                let v279 = v278.ln();
                let v2617 = (v2428 * v279) + ((((v2610 * v2420) * v277) * (v2270 / v278)) * v105);
                let v281 = v38 + (v105 * v279);
                v1139 = v281;
                v2292 = v2617;
            }
            let v283 = v2 / v282;
            let v2627 = ((v2287 * v283) * v2420) / v282;
            let v285 = v2 / v284;
            let v2630 = ((v2291 * v285) * v2420) / v284;
            let v286 = v55 * v283;
            let v287 = v286.powf(v26);
            let v2635 = (v2627 * v55) * (v26 * (v286.powf((v26 - v2270))));
            let v288 = v57 * v285;
            let v289 = v288.powf(v58);
            let v2637 = v58 - v2270;
            let v2640 = (v2630 * v57) * (v58 * (v288.powf(v2637)));
            let v291 = v290 * v287;
            let v2641 = v2635 * v290;
            let v293 = v2 - v292;
            let v295 = v57 / v294;
            let v2648 = ((((v2290 * v295) * v2420) / v294) * (v58 * (v295.powf(v2637)))) * v293;
            let v298 = (v293 * (v295.powf(v58))) + v292;
            let v299 = v2 / v298;
            let v2651 = ((v2648 * v299) * v2420) / v298;
            let v301 = v300 * v298;
            let v2652 = v2648 * v300;
            let v302 = v292 * v299;
            let v2653 = v2651 * v292;
            let v306 = (v111 * v304).exp();
            let v307 = v303 * v306;
            let v2656 = ((v2433 * v304) * v306) * v303;
            let v308 = if v307 < v22 { 1.0 } else { 0.0 };
            let v1743: f64;
            let v2293: Lanes<1>;
            if v308 != 0.0 {
                v1743 = v22;
                v2293 = v2657;
            } else {
                v1743 = v307;
                v2293 = v2656;
            }
            let v312 = v310 - v311;
            let v314 = (v111 * v312).exp();
            let v315 = v309 * v314;
            let v2660 = ((v2433 * v312) * v314) * v309;
            let v319 = (v111 * v317).exp();
            let v320 = v316 * v319;
            let v2663 = ((v2433 * v317) * v319) * v316;
            let v321 = if v320 < v22 { 1.0 } else { 0.0 };
            let v1736: f64;
            let v2294: Lanes<1>;
            if v321 != 0.0 {
                v1736 = v22;
                v2294 = v2657;
            } else {
                v1736 = v320;
                v2294 = v2663;
            }
            let v325 = (v111 * v323).exp();
            let v326 = v322 * v325;
            let v2666 = ((v2433 * v323) * v325) * v322;
            let v330 = (v111 * v328).exp();
            let v2668 = (v2433 * v328) * v330;
            let v331 = v327 * v330;
            let v2669 = v2668 * v327;
            let v333 = v332 * v330;
            let v2670 = v2668 * v332;
            let v337 = (v111 * v335).exp();
            let v338 = v334 * v337;
            let v2673 = ((v2433 * v335) * v337) * v334;
            let v340 = if v339 != v0 { 1.0 } else { 0.0 };
            let v410: f64;
            let v2295: Lanes<1>;
            if v340 != 0.0 {
                let v344 = v341 * (v2 + (v110 * v339));
                let v2675 = (v2284 * v339) * v341;
                let v346 = (v344 - v2) / v24;
                let v2676 = v2675 / v24;
                let v347 = if v344 < v2 { 1.0 } else { 0.0 };
                let v359: f64;
                let v2296: Lanes<1>;
                if v347 != 0.0 {
                    let v348 = v346.exp();
                    let v349 = v2 + v348;
                    let v2686 = ((v2676 * v348) * (v2270 / v349)) * v24;
                    let v352 = v2 + (v24 * (v349.ln()));
                    v359 = v352;
                    v2296 = v2686;
                } else {
                    let v354 = (-v346).exp();
                    let v355 = v2 + v354;
                    let v358 = v344 + (v24 * (v355.ln()));
                    let v2682 = v2675 + ((((v2676 * v2420) * v354) * (v2270 / v355)) * v24);
                    v359 = v358;
                    v2296 = v2682;
                }
                let v361 = v359 - v360;
                v410 = v361;
                v2295 = v2296;
            } else {
                v410 = v341;
                v2295 = v2657;
            }
            let v363 = if v362 != v0 { 1.0 } else { 0.0 };
            let v1034: f64;
            let v2297: Lanes<1>;
            if v363 != 0.0 {
                let v367 = v364 * (v2 + (v110 * v362));
                let v2688 = (v2284 * v362) * v364;
                let v369 = (v367 - v2) / v24;
                let v2689 = v2688 / v24;
                let v370 = if v367 < v2 { 1.0 } else { 0.0 };
                let v382: f64;
                let v2298: Lanes<1>;
                if v370 != 0.0 {
                    let v371 = v369.exp();
                    let v372 = v2 + v371;
                    let v2699 = ((v2689 * v371) * (v2270 / v372)) * v24;
                    let v375 = v2 + (v24 * (v372.ln()));
                    v382 = v375;
                    v2298 = v2699;
                } else {
                    let v377 = (-v369).exp();
                    let v378 = v2 + v377;
                    let v381 = v367 + (v24 * (v378.ln()));
                    let v2695 = v2688 + ((((v2689 * v2420) * v377) * (v2270 / v378)) * v24);
                    v382 = v381;
                    v2298 = v2695;
                }
                let v384 = v382 - v383;
                v1034 = v384;
                v2297 = v2298;
            } else {
                v1034 = v364;
                v2297 = v2657;
            }
            let v389 = v385 * (v2 + (v386 * v110));
            let v2701 = (v2284 * v386) * v385;
            let v391 = v389 * v389;
            let v2702 = v2701 * v389;
            let v2703 = v2702 + v2702;
            let v392 = if v389 < v0 { 1.0 } else { 0.0 };
            let v1579: f64;
            let v2299: Lanes<1>;
            if v392 != 0.0 {
                let v396 = (v391 + v390).sqrt();
                let v397 = v396 - v389;
                let v398 = v394 / v397;
                let v2716 = ((((v2703 * (v2270 / (v2704 * v396))) - v2701) * v398) * v2420) / v397;
                v1579 = v398;
                v2299 = v2716;
            } else {
                let v400 = (v391 + v390).sqrt();
                let v402 = v393 * (v400 + v389);
                let v2709 = ((v2703 * (v2270 / (v2704 * v400))) + v2701) * v393;
                v1579 = v402;
                v2299 = v2709;
            }
            let v408 = ((v404 - v310) - v311) + v407;
            let v411 = (v111 * v408) / v410;
            let v412 = v411.exp();
            let v413 = v403 * v412;
            let v414 = -v159;
            let v416 = (v414 * v109) / v410;
            let v417 = v416.exp();
            let v418 = v413 * v417;
            let v2730 = ((((((v2433 * v408) - (v2295 * v411)) / v410) * v412) * v403) * v417) + (((((v2431 * v414) - (v2295 * v416)) / v410) * v417) * v413);
            let v420 = v2 - v310;
            let v422 = (v111 * v420).exp();
            let v423 = v419 * v422;
            let v2733 = ((v2433 * v420) * v422) * v419;
            let v426 = v2 - v425;
            let v428 = (v111 * v426).exp();
            let v429 = v424 * v428;
            let v2736 = ((v2433 * v426) * v428) * v424;
            let v434 = v431 - (v25 * v432);
            let v436 = (v111 * v434).exp();
            let v437 = v430 * v436;
            let v439 = -v438;
            let v440 = v439 * v109;
            let v2740 = v2431 * v439;
            let v442 = (v440 / v432).exp();
            let v443 = v437 * v442;
            let v2745 = ((((v2433 * v434) * v436) * v430) * v442) + (((v2740 / v432) * v442) * v437);
            let v447 = v431 - (v25 * v445);
            let v449 = (v111 * v447).exp();
            let v450 = v444 * v449;
            let v451 = -v182;
            let v454 = ((v451 * v109) / v445).exp();
            let v455 = v450 * v454;
            let v2754 = ((((v2433 * v447) * v449) * v444) * v454) + ((((v2431 * v451) / v445) * v454) * v450);
            let v458 = (v404 - v304) + v407;
            let v459 = v111 * v458;
            let v2755 = v2433 * v458;
            let v462 = (v459 / v460).exp();
            let v463 = v456 * v462;
            let v465 = -v464;
            let v466 = v465 * v109;
            let v2759 = v2431 * v465;
            let v468 = (v466 / v460).exp();
            let v469 = v463 * v468;
            let v2764 = ((((v2755 / v460) * v462) * v456) * v468) + (((v2759 / v460) * v468) * v463);
            let v473 = (v459 / v471).exp();
            let v474 = v470 * v473;
            let v476 = (v466 / v471).exp();
            let v477 = v474 * v476;
            let v2772 = ((((v2755 / v471) * v473) * v470) * v476) + (((v2759 / v471) * v476) * v474);
            let v479 = if v478 == v2 { 1.0 } else { 0.0 };
            let v1161: f64;
            let v1174: f64;
            let v1216: f64;
            let v2300: Lanes<1>;
            let v2301: Lanes<1>;
            let v2302: Lanes<1>;
            if v479 != 0.0 {
                let v482 = -v481;
                let v485 = ((v482 * v109) / v460).exp();
                let v486 = v480 * v485;
                let v2776 = (((v2431 * v482) / v460) * v485) * v480;
                let v489 = -v488;
                let v491 = (v489 * v109).exp();
                let v492 = v487 * v491;
                let v2779 = ((v2431 * v489) * v491) * v487;
                let v495 = -v494;
                let v498 = ((v495 * v109) / v471).exp();
                let v499 = v493 * v498;
                let v2783 = (((v2431 * v495) / v471) * v498) * v493;
                v1161 = v486;
                v1174 = v492;
                v1216 = v499;
                v2300 = v2776;
                v2301 = v2779;
                v2302 = v2783;
            } else {
                v1161 = v0;
                v1174 = v0;
                v1216 = v0;
                v2300 = v2657;
                v2301 = v2657;
                v2302 = v2657;
            }
            let v502 = (v404 - v425) + v407;
            let v504 = (v111 * v502).exp();
            let v505 = v500 * v504;
            let v507 = -v506;
            let v509 = (v507 * v109).exp();
            let v510 = v505 * v509;
            let v2791 = ((((v2433 * v502) * v504) * v500) * v509) + (((v2431 * v507) * v509) * v505);
            let v514 = v431 - (v25 * v512);
            let v516 = (v111 * v514).exp();
            let v517 = v511 * v516;
            let v519 = (v440 / v512).exp();
            let v520 = v517 * v519;
            let v2799 = ((((v2433 * v514) * v516) * v511) * v519) + (((v2740 / v512) * v519) * v517);
            let v523 = v404 / v522;
            let v525 = (v111 * v523).exp();
            let v526 = v521 * v525;
            let v528 = (v440 / v522).exp();
            let v529 = v526 * v528;
            let v2807 = ((((v2433 * v523) * v525) * v521) * v528) + (((v2740 / v522) * v528) * v526);
            let v531 = v103.sqrt();
            let v532 = v530 * v531;
            let v535 = (v533 * v110).exp();
            let v536 = v532 * v535;
            let v2816 = (((v2427 * (v2270 / (v2704 * v531))) * v530) * v535) + (((v2284 * v533) * v535) * v532);
            let v538 = v537 * v54;
            let v540 = v538.powf(v539);
            let v2821 = (v2285 * v54) * (v539 * (v538.powf(v2818)));
            let v541 = v2 / v287;
            let v2824 = ((v2635 * v541) * v2420) / v287;
            let v543 = v542 * v537;
            let v544 = v543 * v537;
            let v545 = v544 * v540;
            let v547 = (v545 * v541) * v55;
            let v550 = ((v547 * v283) * v54) * v54;
            let v2840 = (((((((((((v2285 * v542) * v537) + (v2285 * v543)) * v540) + (v2821 * v544)) * v541) + (v2824 * v545)) * v55) * v283) + (v2627 * v547)) * v54) * v54;
            let v552 = v551 * v540;
            let v553 = v552 * v282;
            let v556 = ((v553 * v282) * v56) * v56;
            let v557 = v556 * v287;
            let v559 = (v542 - v550).exp();
            let v560 = v557 * v559;
            let v2857 = ((((((((((v2821 * v551) * v282) + (v2287 * v552)) * v282) + (v2287 * v553)) * v56) * v56) * v287) + (v2635 * v556)) * v559) + (((v2840 * v2420) * v559) * v557);
            let v562 = v561 * v84;
            let v564 = v562.powf(v563);
            let v2862 = (v2286 * v84) * (v563 * (v562.powf(v2859)));
            let v565 = v2 / v289;
            let v567 = v566 * v561;
            let v568 = v567 * v561;
            let v569 = v568 * v564;
            let v571 = (v569 * v565) * v57;
            let v574 = ((v571 * v285) * v84) * v84;
            let v2881 = (((((((((((v2286 * v566) * v561) + (v2286 * v567)) * v564) + (v2862 * v568)) * v565) + ((((v2640 * v565) * v2420) / v289) * v569)) * v57) * v285) + (v2630 * v571)) * v84) * v84;
            let v576 = v575 * v564;
            let v577 = v576 * v284;
            let v580 = ((v577 * v284) * v85) * v85;
            let v581 = v580 * v289;
            let v583 = (v566 - v574).exp();
            let v584 = v581 * v583;
            let v2898 = ((((((((((v2862 * v575) * v284) + (v2291 * v576)) * v284) + (v2291 * v577)) * v85) * v85) * v289) + (v2640 * v580)) * v583) + (((v2881 * v2420) * v583) * v581);
            let v586 = (v111 * v311).exp();
            let v2900 = (v2433 * v311) * v586;
            let v588 = v587 * v586;
            let v589 = v588 * v299;
            let v2904 = ((v2900 * v587) * v299) + (v2651 * v588);
            let v591 = v590 * v586;
            let v592 = v591 * v541;
            let v2908 = ((v2900 * v590) * v541) + (v2824 * v591);
            let v594 = v310 - v25;
            let v596 = (v111 * v594).exp();
            let v597 = v593 * v596;
            let v599 = -v598;
            let v601 = (v599 * v109).exp();
            let v602 = v597 * v601;
            let v2916 = ((((v2433 * v594) * v596) * v593) * v601) + (((v2431 * v599) * v601) * v597);
            let v605 = (v311 + v310) - v2;
            let v607 = (v111 * v605).exp();
            let v608 = v603 * v607;
            let v2919 = ((v2433 * v605) * v607) * v603;
            let v610 = v335 - v2;
            let v612 = (v111 * v610).exp();
            let v613 = v609 * v612;
            let v2922 = ((v2433 * v610) * v612) * v609;
            let v615 = v608 + v613;
            let v2923 = v2919 + v2922;
            let v617 = v603 + v609;
            let v618 = (v614 * v615) / v617;
            let v2925 = (v2923 * v614) / v617;
            let v621 = v620 - v2;
            let v623 = (v111 * v621).exp();
            let v624 = v619 * v623;
            let v2928 = ((v2433 * v621) * v623) * v619;
            let v626 = v102 - v625;
            let v628 = if v102 < v627 { 1.0 } else { 0.0 };
            let v1661: f64;
            let v2303: Lanes<1>;
            if v628 != 0.0 {
                let v634 = v633 * v626;
                let v637 = v629 * ((v2 + (v630 * v626)) - (v634 * v626));
                let v2935 = ((v2284 * v630) - (((v2284 * v633) * v626) + (v2284 * v634))) * v629;
                v1661 = v637;
                v2303 = v2935;
            } else {
                let v639 = v629 * v638;
                v1661 = v639;
                v2303 = v2657;
            }
            let v641 = v640 * v586;
            let v2936 = v2900 * v640;
            let v646 = v642 * ((v15 / v12).powf(v644));
            let v647 = if v322 > v0 { 1.0 } else { 0.0 };
            let v1786: f64;
            let v2304: Lanes<1>;
            if v647 != 0.0 {
                let v648 = v2 / v326;
                let v2939 = ((v2666 * v648) * v2420) / v326;
                let v649 = if v648 > v23 { 1.0 } else { 0.0 };
                let v1787: f64;
                let v2305: Lanes<1>;
                if v649 != 0.0 {
                    v1787 = v23;
                    v2305 = v2657;
                } else {
                    v1787 = v648;
                    v2305 = v2939;
                }
                v1786 = v1787;
                v2304 = v2305;
            } else {
                v1786 = v0;
                v2304 = v2657;
            }
            let v650 = if v327 > v0 { 1.0 } else { 0.0 };
            let v1791: f64;
            let v2306: Lanes<1>;
            if v650 != 0.0 {
                let v651 = v2 / v331;
                let v2942 = ((v2669 * v651) * v2420) / v331;
                let v652 = if v651 > v23 { 1.0 } else { 0.0 };
                let v1792: f64;
                let v2307: Lanes<1>;
                if v652 != 0.0 {
                    v1792 = v23;
                    v2307 = v2657;
                } else {
                    v1792 = v651;
                    v2307 = v2942;
                }
                v1791 = v1792;
                v2306 = v2307;
            } else {
                v1791 = v0;
                v2306 = v2657;
            }
            let v653 = if v332 > v0 { 1.0 } else { 0.0 };
            let v1796: f64;
            let v2308: Lanes<1>;
            if v653 != 0.0 {
                let v654 = v2 / v333;
                let v2945 = ((v2670 * v654) * v2420) / v333;
                let v655 = if v654 > v23 { 1.0 } else { 0.0 };
                let v1797: f64;
                let v2309: Lanes<1>;
                if v655 != 0.0 {
                    v1797 = v23;
                    v2309 = v2657;
                } else {
                    v1797 = v654;
                    v2309 = v2945;
                }
                v1796 = v1797;
                v2308 = v2309;
            } else {
                v1796 = v0;
                v2308 = v2657;
            }
            let v659 = v1 * (v656 - v657);
            let v2949 = ((Lanes([v2272[0], 0.0])) - (Lanes([0.0, v2273[0]]))) * v1;
            let v662 = v1 * (v656 - v660);
            let v2953 = ((Lanes([v2272[0], 0.0])) - (Lanes([0.0, v2274[0]]))) * v1;
            let v665 = v1 * (v656 - v663);
            let v2957 = ((Lanes([0.0, v2272[0]])) - (Lanes([v2275[0], 0.0]))) * v1;
            let v668 = v1 * (v666 - v663);
            let v2961 = ((Lanes([0.0, v2276[0]])) - (Lanes([v2275[0], 0.0]))) * v1;
            let v670 = v1 * (v666 - v656);
            let v2965 = ((Lanes([v2276[0], 0.0])) - (Lanes([0.0, v2272[0]]))) * v1;
            let v672 = v1 * (v657 - v660);
            let v2969 = ((Lanes([v2273[0], 0.0])) - (Lanes([0.0, v2274[0]]))) * v1;
            let v675 = v1 * (v673 - v663);
            let v2973 = ((Lanes([v2277[0], 0.0])) - (Lanes([0.0, v2275[0]]))) * v1;
            let v678 = v1 * (v676 - v666);
            let v2977 = ((Lanes([v2278[0], 0.0])) - (Lanes([0.0, v2276[0]]))) * v1;
            let v680 = v1 * (v676 - v673);
            let v2981 = ((Lanes([v2278[0], 0.0])) - (Lanes([0.0, v2277[0]]))) * v1;
            let v683 = v1 * (v676 - v681);
            let v2985 = ((Lanes([0.0, v2278[0]])) - (Lanes([v2279[0], 0.0]))) * v1;
            let v686 = v1 * (v684 - v657);
            let v2989 = ((Lanes([0.0, v2280[0]])) - (Lanes([v2273[0], 0.0]))) * v1;
            let v689 = v1 * (v687 - v684);
            let v2993 = ((Lanes([v2281[0], 0.0])) - (Lanes([0.0, v2280[0]]))) * v1;
            let v2996 = (Lanes([v2965[0], v2965[1], 0.0])) + (Lanes([0.0, v2953[0], v2953[1]]));
            let v2999 = (Lanes([v2996[0], v2996[1], 0.0, v2996[2]])) - (Lanes([0.0, 0.0, v2969[0], v2969[1]]));
            let v692 = ((v670 + v662) - v672) - v686;
            let v3002 = (Lanes([v2999[0], v2999[1], v2999[2], v2999[3], 0.0])) - (Lanes([0.0, 0.0, v2989[0], 0.0, v2989[1]]));
            let v3003 = v2985 * v2420;
            let v3006 = (Lanes([v3003[0], v3003[1], 0.0])) + (Lanes([0.0, v2977[0], v2977[1]]));
            let v3009 = (Lanes([v3006[0], v3006[1], v3006[2], 0.0, 0.0, 0.0, 0.0])) + (Lanes([0.0, 0.0, v3002[0], v3002[1], v3002[2], v3002[3], v3002[4]]));
            let v696 = (((-v683) + v678) + v692) - v689;
            let v3012 = (Lanes([v3009[0], v3009[1], v3009[2], v3009[3], v3009[4], v3009[5], 0.0, v3009[6]])) - (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, v2993[0], v2993[1]]));
            let v697 = v683 + v696;
            let v3014 = (Lanes([v2985[0], v2985[1], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + v3012;
            let v698 = v662 * v107;
            let v3015 = v2953 * v107;
            let v3016 = v2431 * v662;
            let v3019 = (Lanes([0.0, v3015[0], v3015[1]])) + (Lanes([v3016[0], 0.0, 0.0]));
            let v700 = if v698 < v699 { 1.0 } else { 0.0 };
            let v922: f64;
            let v2310: Lanes<3>;
            if v700 != 0.0 {
                let v701 = v698.exp();
                let v3021 = v3019 * v701;
                v922 = v701;
                v2310 = v3021;
            } else {
                let v702 = v699.exp();
                let v705 = v702 * (v2 + (v698 - v699));
                let v3020 = v3019 * v702;
                v922 = v705;
                v2310 = v3020;
            }
            let v706 = v665 * v107;
            let v3022 = v2957 * v107;
            let v3023 = v2431 * v665;
            let v3026 = (Lanes([0.0, v3022[0], v3022[1]])) + (Lanes([v3023[0], 0.0, 0.0]));
            let v707 = v706 / v410;
            let v3027 = v2295 * v707;
            let v3030 = (v3026 - (Lanes([v3027[0], 0.0, 0.0]))) / v410;
            let v708 = if v707 < v699 { 1.0 } else { 0.0 };
            let v1027: f64;
            let v2311: Lanes<3>;
            if v708 != 0.0 {
                let v709 = v707.exp();
                let v3032 = v3030 * v709;
                v1027 = v709;
                v2311 = v3032;
            } else {
                let v710 = v699.exp();
                let v713 = v710 * (v2 + (v707 - v699));
                let v3031 = v3030 * v710;
                v1027 = v713;
                v2311 = v3031;
            }
            let v714 = v692 * v107;
            let v3033 = v3002 * v107;
            let v3034 = v2431 * v692;
            let v3037 = (Lanes([0.0, v3033[0], v3033[1], v3033[2], v3033[3], v3033[4]])) + (Lanes([v3034[0], 0.0, 0.0, 0.0, 0.0, 0.0]));
            let v715 = if v714 < v699 { 1.0 } else { 0.0 };
            let v1413: f64;
            let v2312: Lanes<6>;
            if v715 != 0.0 {
                let v716 = v714.exp();
                let v3039 = v3037 * v716;
                v1413 = v716;
                v2312 = v3039;
            } else {
                let v717 = v699.exp();
                let v720 = v717 * (v2 + (v714 - v699));
                let v3038 = v3037 * v717;
                v1413 = v720;
                v2312 = v3038;
            }
            let v721 = v670 * v107;
            let v3040 = v2965 * v107;
            let v3041 = v2431 * v670;
            let v3044 = (Lanes([0.0, v3040[0], v3040[1]])) + (Lanes([v3041[0], 0.0, 0.0]));
            let v722 = if v721 < v699 { 1.0 } else { 0.0 };
            let v1557: f64;
            let v2313: Lanes<3>;
            if v722 != 0.0 {
                let v723 = v721.exp();
                let v3046 = v3044 * v723;
                v1557 = v723;
                v2313 = v3046;
            } else {
                let v724 = v699.exp();
                let v727 = v724 * (v2 + (v721 - v699));
                let v3045 = v3044 * v724;
                v1557 = v727;
                v2313 = v3045;
            }
            let v728 = v697 * v107;
            let v3047 = v3014 * v107;
            let v3048 = v2431 * v697;
            let v3051 = (Lanes([v3047[0], v3047[1], 0.0, v3047[2], v3047[3], v3047[4], v3047[5], v3047[6], v3047[7]])) + (Lanes([0.0, 0.0, v3048[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]));
            let v729 = if v728 < v699 { 1.0 } else { 0.0 };
            let v1443: f64;
            let v2314: Lanes<9>;
            if v729 != 0.0 {
                let v730 = v728.exp();
                let v3053 = v3051 * v730;
                v1443 = v730;
                v2314 = v3053;
            } else {
                let v731 = v699.exp();
                let v734 = v731 * (v2 + (v728 - v699));
                let v3052 = v3051 * v731;
                v1443 = v734;
                v2314 = v3052;
            }
            let v736 = v697 - v735;
            let v3054 = Lanes([v3014[0], v3014[1], 0.0, v3014[2], v3014[3], v3014[4], v3014[5], v3014[6], v3014[7]]);
            let v737 = v736 * v107;
            let v3058 = v2431 * v736;
            let v3060 = ((v3054 - (Lanes([0.0, 0.0, v2288[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) * v107) + (Lanes([0.0, 0.0, v3058[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]));
            let v738 = if v737 < v699 { 1.0 } else { 0.0 };
            let v1993: f64;
            let v2315: Lanes<9>;
            if v738 != 0.0 {
                let v739 = v737.exp();
                let v3062 = v3060 * v739;
                v1993 = v739;
                v2315 = v3062;
            } else {
                let v740 = v699.exp();
                let v743 = v740 * (v2 + (v737 - v699));
                let v3061 = v3060 * v740;
                v1993 = v743;
                v2315 = v3061;
            }
            let v744 = v692 - v735;
            let v3063 = Lanes([0.0, v3002[0], v3002[1], v3002[2], v3002[3], v3002[4]]);
            let v745 = v744 * v107;
            let v3067 = v2431 * v744;
            let v3069 = ((v3063 - (Lanes([v2288[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) * v107) + (Lanes([v3067[0], 0.0, 0.0, 0.0, 0.0, 0.0]));
            let v746 = if v745 < v699 { 1.0 } else { 0.0 };
            let v1415: f64;
            let v2316: Lanes<6>;
            if v746 != 0.0 {
                let v747 = v745.exp();
                let v3071 = v3069 * v747;
                v1415 = v747;
                v2316 = v3071;
            } else {
                let v748 = v699.exp();
                let v751 = v748 * (v2 + (v745 - v699));
                let v3070 = v3069 * v748;
                v1415 = v751;
                v2316 = v3070;
            }
            let v752 = v662 - v735;
            let v753 = v752 * v107;
            let v3076 = v2431 * v752;
            let v3078 = (((Lanes([0.0, v2953[0], v2953[1]])) - (Lanes([v2288[0], 0.0, 0.0]))) * v107) + (Lanes([v3076[0], 0.0, 0.0]));
            let v754 = if v753 < v699 { 1.0 } else { 0.0 };
            let v768: f64;
            let v2317: Lanes<3>;
            if v754 != 0.0 {
                let v755 = v753.exp();
                let v3080 = v3078 * v755;
                v768 = v755;
                v2317 = v3080;
            } else {
                let v756 = v699.exp();
                let v759 = v756 * (v2 + (v753 - v699));
                let v3079 = v3078 * v756;
                v768 = v759;
                v2317 = v3079;
            }
            let v760 = v659 - v735;
            let v3081 = Lanes([0.0, v2949[0], v2949[1]]);
            let v3082 = Lanes([v2288[0], 0.0, 0.0]);
            let v761 = v760 * v107;
            let v3085 = v2431 * v760;
            let v3087 = ((v3081 - v3082) * v107) + (Lanes([v3085[0], 0.0, 0.0]));
            let v762 = if v761 < v699 { 1.0 } else { 0.0 };
            let v772: f64;
            let v2318: Lanes<3>;
            if v762 != 0.0 {
                let v763 = v761.exp();
                let v3089 = v3087 * v763;
                v772 = v763;
                v2318 = v3089;
            } else {
                let v764 = v699.exp();
                let v767 = v764 * (v2 + (v761 - v699));
                let v3088 = v3087 * v764;
                v772 = v767;
                v2318 = v3088;
            }
            let v771 = (v2 + (v404 * v768)).sqrt();
            let v3093 = (v2317 * v404) * (v2270 / (v2704 * v771));
            let v775 = (v2 + (v404 * v772)).sqrt();
            let v3097 = (v2318 * v404) * (v2270 / (v2704 * v775));
            let v777 = v2 + v775;
            let v778 = (v25 * v772) / v777;
            let v3101 = ((v2318 * v25) - (v3097 * v778)) / v777;
            let v780 = if v778 < v779 { 1.0 } else { 0.0 };
            let v867: f64;
            let v2319: Lanes<3>;
            if v780 != 0.0 {
                v867 = v779;
                v2319 = v3102;
            } else {
                v867 = v778;
                v2319 = v3101;
            }
            let v3103 = Lanes([v3093[0], v3093[1], 0.0, v3093[2]]);
            let v782 = v771 + v2;
            let v783 = v782 / v777;
            let v3106 = v3097 * v783;
            let v785 = (v771 - v775) - (v783.ln());
            let v786 = v105 * v785;
            let v3113 = v2428 * v785;
            let v3116 = (Lanes([v3113[0], 0.0, 0.0, 0.0])) + (((v3103 - (Lanes([v3097[0], v3097[1], v3097[2], 0.0]))) - (((v3103 - (Lanes([v3106[0], v3106[1], v3106[2], 0.0]))) / v777) * (v2270 / v783))) * v105);
            let v3117 = Lanes([0.0, 0.0, v2969[0], v2969[1]]);
            let v788 = (v786 + v672) / v338;
            let v3119 = v2673 * v788;
            let v3122 = ((v3116 + v3117) - (Lanes([v3119[0], 0.0, 0.0, 0.0]))) / v338;
            let v789 = if v788 > v0 { 1.0 } else { 0.0 };
            let v978: f64;
            let v991: f64;
            let v1006: f64;
            let v1033: f64;
            let v1609: f64;
            let v1645: f64;
            let v1950: f64;
            let v2320: Lanes<4>;
            let v2321: Lanes<4>;
            let v2322: Lanes<4>;
            let v2323: Lanes<4>;
            let v2324: Lanes<4>;
            let v2325: Lanes<4>;
            let v2326: Lanes<4>;
            if v789 != 0.0 {
                let v791 = if v659 < v790 { 1.0 } else { 0.0 };
                let v804: f64;
                let v2327: Lanes<2>;
                if v791 != 0.0 {
                    v804 = v659;
                    v2327 = v2949;
                } else {
                    let v793 = v2 + (v659 - v790);
                    let v3150 = v2949 * (v2270 / v793);
                    let v795 = v790 + (v793.ln());
                    v804 = v795;
                    v2327 = v3150;
                }
                let v796 = v25 * v105;
                let v797 = v393 * v788;
                let v798 = v797 * v338;
                let v3154 = v2673 * v797;
                let v3156 = ((v3122 * v393) * v338) + (Lanes([v3154[0], 0.0, 0.0, 0.0]));
                let v3158 = v2431 * v798;
                let v800 = (v798 * v107) + v2;
                let v801 = v800.ln();
                let v3163 = (v2428 * v25) * v801;
                let v805 = (v735 + (v796 * v801)) - v804;
                let v3170 = ((Lanes([v2288[0], 0.0, 0.0, 0.0])) + ((Lanes([v3163[0], 0.0, 0.0, 0.0])) + ((((v3156 * v107) + (Lanes([v3158[0], 0.0, 0.0, 0.0]))) * (v2270 / v800)) * v796))) - (Lanes([0.0, v2327[0], v2327[1], 0.0]));
                let v807 = v806 * v735;
                let v808 = v807 * v807;
                let v3172 = (v2288 * v806) * v807;
                let v3173 = v3172 + v3172;
                let v809 = v805 * v805;
                let v3174 = v3170 * v805;
                let v3175 = v3174 + v3174;
                let v810 = if v805 < v0 { 1.0 } else { 0.0 };
                let v820: f64;
                let v2328: Lanes<4>;
                if v810 != 0.0 {
                    let v3183 = v3173 * v393;
                    let v813 = (v809 + v808).sqrt();
                    let v814 = v813 - v805;
                    let v815 = (v393 * v808) / v814;
                    let v3193 = ((Lanes([v3183[0], 0.0, 0.0, 0.0])) - ((((v3175 + (Lanes([v3173[0], 0.0, 0.0, 0.0]))) * (v2270 / (v2704 * v813))) - v3170) * v815)) / v814;
                    v820 = v815;
                    v2328 = v3193;
                } else {
                    let v817 = (v809 + v808).sqrt();
                    let v819 = v393 * (v817 + v805);
                    let v3182 = (((v3175 + (Lanes([v3173[0], 0.0, 0.0, 0.0]))) * (v2270 / (v2704 * v817))) + v3170) * v393;
                    v820 = v819;
                    v2328 = v3182;
                }
                let v823 = v821 * v822;
                let v824 = v820 + v823;
                let v3197 = v2673 * v821;
                let v828 = v822 * (v820 + (v821 * v338));
                let v829 = (v820 * v824) / v828;
                let v3203 = (((v2328 * v824) + (v2328 * v820)) - (((v2328 + (Lanes([v3197[0], 0.0, 0.0, 0.0]))) * v822) * v829)) / v828;
                let v830 = v788 / v829;
                let v3206 = (v3122 - (v3203 * v830)) / v829;
                let v833 = (v830 - v2) / v832;
                let v3207 = v3206 / v832;
                let v834 = if v830 < v2 { 1.0 } else { 0.0 };
                let v846: f64;
                let v2329: Lanes<4>;
                if v834 != 0.0 {
                    let v835 = v833.exp();
                    let v836 = v2 + v835;
                    let v3217 = ((v3207 * v835) * (v2270 / v836)) * v832;
                    let v839 = v2 + (v832 * (v836.ln()));
                    v846 = v839;
                    v2329 = v3217;
                } else {
                    let v841 = (-v833).exp();
                    let v842 = v2 + v841;
                    let v845 = v830 + (v832 * (v842.ln()));
                    let v3213 = v3206 + ((((v3207 * v2420) * v841) * (v2270 / v842)) * v832);
                    v846 = v845;
                    v2329 = v3213;
                }
                let v853 = v2 + (v832 * ((v2 + ((v847 / v832).exp())).ln()));
                let v854 = v846 / v853;
                let v3218 = v2329 / v853;
                let v855 = v820 / v823;
                let v3219 = v2328 / v823;
                let v856 = v404 * v854;
                let v857 = v856 * v855;
                let v858 = v2 + v855;
                let v861 = (v2 + (v857 * v858)).sqrt();
                let v863 = v25 * v854;
                let v864 = v863 * v858;
                let v865 = (v2 + v861) / v864;
                let v3236 = (((((((v3218 * v404) * v855) + (v3219 * v856)) * v858) + (v3219 * v857)) * (v2270 / (v2704 * v861))) - ((((v3218 * v25) * v858) + (v3219 * v863)) * v865)) / v864;
                let v868 = v867 * v865;
                let v3238 = v2319 * v865;
                let v3241 = (Lanes([v3238[0], v3238[1], v3238[2], 0.0])) + (v3236 * v867);
                let v870 = v2 + v868;
                let v871 = ((v2 - v865) + v868) / v870;
                let v3245 = (((v3236 * v2420) + v3241) - (v3241 * v871)) / v870;
                let v872 = v798 * v871;
                let v873 = v872 * v107;
                let v3250 = v2431 * v872;
                let v3252 = (((v3156 * v871) + (v3245 * v798)) * v107) + (Lanes([v3250[0], 0.0, 0.0, 0.0]));
                let v876 = (v867 + v873) + v2;
                let v3256 = v2319 * v876;
                let v878 = (v25 * v873) + (v867 * v876);
                let v3260 = (v3252 * v25) + ((Lanes([v3256[0], v3256[1], v3256[2], 0.0])) + (((Lanes([v2319[0], v2319[1], v2319[2], 0.0])) + v3252) * v867));
                let v880 = v393 * (v873 - v2);
                let v3261 = v3252 * v393;
                let v3262 = v3261 * v880;
                let v882 = (v880 * v880) + v878;
                let v3264 = (v3262 + v3262) + v3260;
                let v883 = if v873 >= v2 { 1.0 } else { 0.0 };
                let v889: f64;
                let v2330: Lanes<4>;
                if v883 != 0.0 {
                    let v884 = v882.sqrt();
                    let v885 = v880 + v884;
                    let v3275 = v3261 + (v3264 * (v2270 / (v2704 * v884)));
                    v889 = v885;
                    v2330 = v3275;
                } else {
                    let v886 = v882.sqrt();
                    let v887 = v886 - v880;
                    let v888 = v878 / v887;
                    let v3271 = (v3260 - (((v3264 * (v2270 / (v2704 * v886))) - v3261) * v888)) / v887;
                    v889 = v888;
                    v2330 = v3271;
                }
                let v891 = if v889 < v890 { 1.0 } else { 0.0 };
                let v892: f64;
                let v2331: Lanes<4>;
                if v891 != 0.0 {
                    v892 = v890;
                    v2331 = v3148;
                } else {
                    v892 = v889;
                    v2331 = v2330;
                }
                let v893 = v892 + v2;
                let v894 = v892 * v893;
                let v896 = (v735 * v107).exp();
                let v897 = v894 * v896;
                let v3284 = (((v2288 * v107) + (v2431 * v735)) * v896) * v894;
                let v3286 = (((v2331 * v893) + (v2331 * v892)) * v896) + (Lanes([v3284[0], 0.0, 0.0, 0.0]));
                let v898 = v393 * v822;
                let v900 = v898 * (v788 - v821);
                let v3287 = v3122 * v898;
                let v902 = (v822 * v338) * v821;
                let v3290 = ((v2673 * v822) * v821) * v788;
                let v3294 = v3287 * v900;
                let v906 = ((v900 * v900) + (v902 * v788)).sqrt();
                let v907 = v900 + v906;
                let v3300 = v3287 + (((v3294 + v3294) + ((Lanes([v3290[0], 0.0, 0.0, 0.0])) + (v3122 * v902))) * (v2270 / (v2704 * v906)));
                let v909 = if v908 == v0 { 1.0 } else { 0.0 };
                let v992: f64;
                let v2332: Lanes<4>;
                if v909 != 0.0 {
                    let v910 = v294 * v40;
                    let v3310 = v2290 * v40;
                    let v3311 = Lanes([v3310[0], 0.0, 0.0, 0.0]);
                    v992 = v910;
                    v2332 = v3311;
                } else {
                    let v912 = v788 + v829;
                    let v913 = (v25 * v788) / v912;
                    let v914 = v40 + v913;
                    let v915 = v294 * v914;
                    let v3306 = v2290 * v914;
                    let v3309 = (Lanes([v3306[0], 0.0, 0.0, 0.0])) + ((((v3122 * v25) - ((v3122 + v3203) * v913)) / v912) * v294);
                    v992 = v915;
                    v2332 = v3309;
                }
                let v917 = v821 + v788;
                let v918 = (v821 * v788) / v917;
                let v3315 = ((v3122 * v821) - (v3122 * v918)) / v917;
                let v919 = v821 / v917;
                let v3318 = ((v3122 * v919) * v2420) / v917;
                v978 = v907;
                v991 = v992;
                v1006 = v919;
                v1033 = v897;
                v1609 = v871;
                v1645 = v918;
                v1950 = v892;
                v2320 = v3300;
                v2321 = v2332;
                v2322 = v3318;
                v2323 = v3286;
                v2324 = v3245;
                v2325 = v3315;
                v2326 = v2331;
            } else {
                let v921 = (v25 * v768) / v782;
                let v3126 = ((v2317 * v25) - (v3093 * v921)) / v782;
                let v933 = if (if (v672.abs()) < (v924 * v105) { 1.0 } else { 0.0 }) != 0.0 || (if (v786.abs()) < ((v928 * v105) * (v771 + v775)) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v1610: f64;
                let v2333: Lanes<4>;
                if v933 != 0.0 {
                    let v935 = v393 * (v921 + v867);
                    let v3138 = ((Lanes([v3126[0], v3126[1], 0.0, v3126[2]])) + (Lanes([v2319[0], v2319[1], v2319[2], 0.0]))) * v393;
                    let v936 = v935 + v2;
                    let v937 = v935 / v936;
                    let v3141 = (v3138 - (v3138 * v937)) / v936;
                    v1610 = v937;
                    v2333 = v3141;
                } else {
                    let v939 = (v786 + v662) - v659;
                    let v940 = v786 / v939;
                    let v3134 = (v3116 - (((v3116 + (Lanes([0.0, v2953[0], 0.0, v2953[1]]))) - (Lanes([0.0, v2949[0], v2949[1], 0.0]))) * v940)) / v939;
                    v1610 = v940;
                    v2333 = v3134;
                }
                let v941 = v40 * v294;
                let v3142 = v2290 * v40;
                let v943 = v2 - (v788 / v821);
                let v3144 = (v3122 / v821) * v2420;
                let v3145 = Lanes([v3142[0], 0.0, 0.0, 0.0]);
                let v3146 = Lanes([v2310[0], v2310[1], 0.0, v2310[2]]);
                let v3147 = Lanes([v3126[0], v3126[1], 0.0, v3126[2]]);
                v978 = v672;
                v991 = v941;
                v1006 = v943;
                v1033 = v922;
                v1609 = v1610;
                v1645 = v788;
                v1950 = v921;
                v2320 = v3117;
                v2321 = v3145;
                v2322 = v3144;
                v2323 = v3146;
                v2324 = v2333;
                v2325 = v3122;
                v2326 = v3147;
            }
            let v947 = v2 - (v152.powf((v944 / v26)));
            let v948 = v282 * v947;
            let v3319 = v2287 * v947;
            let v949 = v40 * v282;
            let v3320 = v2287 * v40;
            let v3321 = Lanes([0.0, v2957[0], v2957[1]]);
            let v3322 = Lanes([v3319[0], 0.0, 0.0]);
            let v951 = (v665 - v948) / v949;
            let v3324 = v3320 * v951;
            let v3327 = ((v3321 - v3322) - (Lanes([v3324[0], 0.0, 0.0]))) / v949;
            let v952 = if v665 < v948 { 1.0 } else { 0.0 };
            let v964: f64;
            let v2334: Lanes<3>;
            if v952 != 0.0 {
                let v953 = v951.exp();
                let v954 = v2 + v953;
                let v955 = v954.ln();
                let v3340 = v3320 * v955;
                let v957 = v665 - (v949 * v955);
                let v3344 = v3321 - ((Lanes([v3340[0], 0.0, 0.0])) + (((v3327 * v953) * (v2270 / v954)) * v949));
                v964 = v957;
                v2334 = v3344;
            } else {
                let v959 = (-v951).exp();
                let v960 = v2 + v959;
                let v961 = v960.ln();
                let v3332 = v3320 * v961;
                let v963 = v948 - (v949 * v961);
                let v3336 = v3322 - ((Lanes([v3332[0], 0.0, 0.0])) + ((((v3327 * v2420) * v959) * (v2270 / v960)) * v949));
                v964 = v963;
                v2334 = v3336;
            }
            let v3346 = v2627 * v964;
            let v966 = v2 - (v964 * v283);
            let v3349 = ((v2334 * v283) + (Lanes([v3346[0], 0.0, 0.0]))) * v2420;
            let v967 = v2 - v26;
            let v968 = v966.powf(v967);
            let v3350 = v967 - v2270;
            let v3353 = v3349 * (v967 * (v966.powf(v3350)));
            let v969 = v282 / v967;
            let v3354 = v2287 / v967;
            let v970 = v2 - v968;
            let v3356 = v3354 * v970;
            let v974 = (v969 * v970) + (v152 * (v665 - v964));
            let v3362 = ((Lanes([v3356[0], 0.0, 0.0])) + ((v3353 * v2420) * v969)) + ((v3321 - v2334) * v152);
            let v976 = if v975 == v2 { 1.0 } else { 0.0 };
            let v988: f64;
            let v2335: Lanes<4>;
            if v976 != 0.0 {
                let v3366 = Lanes([0.0, v2949[0], v2949[1], 0.0]);
                v988 = v659;
                v2335 = v3366;
            } else {
                let v977 = if v975 == v25 { 1.0 } else { 0.0 };
                let v989: f64;
                let v2336: Lanes<4>;
                if v977 != 0.0 {
                    let v979 = v659 + v978;
                    let v3365 = (Lanes([0.0, v2949[0], v2949[1], 0.0])) + v2320;
                    v989 = v979;
                    v2336 = v3365;
                } else {
                    let v3363 = Lanes([0.0, v2953[0], 0.0, v2953[1]]);
                    v989 = v662;
                    v2336 = v3363;
                }
                v988 = v989;
                v2335 = v2336;
            }
            let v3367 = v2653 * v2420;
            let v981 = v2 - v302;
            let v982 = (v25 - v302) / v981;
            let v3370 = (v3367 - (v3367 * v982)) / v981;
            let v984 = v983 / v58;
            let v986 = v2 - (v982.powf(v984));
            let v987 = v294 * v986;
            let v3378 = (v2290 * v986) + (((v3370 * (v984 * (v982.powf((v984 - v2270))))) * v2420) * v294);
            let v3379 = Lanes([v3378[0], 0.0, 0.0, 0.0]);
            let v993 = (v988 - v987) / v991;
            let v3383 = ((v2335 - v3379) - (v2321 * v993)) / v991;
            let v994 = if v988 < v987 { 1.0 } else { 0.0 };
            let v1011: f64;
            let v2337: Lanes<4>;
            if v994 != 0.0 {
                let v995 = v993.exp();
                let v996 = v2 + v995;
                let v997 = v996.ln();
                let v999 = v988 - (v991 * v997);
                let v3398 = v2335 - ((v2321 * v997) + (((v3383 * v995) * (v2270 / v996)) * v991));
                v1011 = v999;
                v2337 = v3398;
            } else {
                let v1001 = (-v993).exp();
                let v1002 = v2 + v1001;
                let v1003 = v1002.ln();
                let v1005 = v987 - (v991 * v1003);
                let v3391 = v3379 - ((v2321 * v1003) + ((((v3383 * v2420) * v1001) * (v2270 / v1002)) * v991));
                v1011 = v1005;
                v2337 = v3391;
            }
            let v1008 = v1006.powf(v1007);
            let v3402 = v2322 * (v1007 * (v1006.powf((v1007 - v2270))));
            let v1009 = v2 - v58;
            let v1010 = v294 / v1009;
            let v3403 = v2290 / v1009;
            let v1012 = v1011 / v294;
            let v3404 = v2290 * v1012;
            let v1013 = v2 - v1012;
            let v1014 = v1013.powf(v1009);
            let v3409 = v1009 - v2270;
            let v1016 = v2 - (v1008 * v1014);
            let v3417 = v3403 * v1016;
            let v1018 = v1008 * v982;
            let v3422 = v3370 * v1008;
            let v1019 = v988 - v1011;
            let v1021 = (v1010 * v1016) + (v1018 * v1019);
            let v3430 = v3367 * v1021;
            let v3434 = v2653 * v659;
            let v3435 = v2949 * v302;
            let v3438 = (Lanes([v3434[0], 0.0, 0.0])) + (Lanes([0.0, v3435[0], v3435[1]]));
            let v1024 = (v981 * v1021) + (v302 * v659);
            let v3440 = ((Lanes([v3430[0], 0.0, 0.0, 0.0])) + ((((Lanes([v3417[0], 0.0, 0.0, 0.0])) + ((((v3402 * v1014) + (((((v2337 - (Lanes([v3404[0], 0.0, 0.0, 0.0]))) / v294) * v2420) * (v1009 * (v1013.powf(v3409)))) * v1008)) * v2420) * v1010)) + ((((v3402 * v982) + (Lanes([v3422[0], 0.0, 0.0, 0.0]))) * v1019) + ((v2335 - v2337) * v1018))) * v981)) + (Lanes([v3438[0], v3438[1], v3438[2], 0.0]));
            let v1026 = (v404 * v418) / v423;
            let v3444 = ((v2730 * v404) - (v2733 * v1026)) / v423;
            let v1028 = v1026 * v1027;
            let v3445 = v3444 * v1027;
            let v3448 = (Lanes([v3445[0], 0.0, 0.0])) + (v2311 * v1026);
            let v1030 = (v2 + v1028).sqrt();
            let v3451 = v3448 * (v2270 / (v2704 * v1030));
            let v1031 = v2 + v1030;
            let v1032 = v1028 / v1031;
            let v3454 = (v3448 - (v3451 * v1032)) / v1031;
            let v1035 = v2 / v1034;
            let v1036 = v1033.powf(v1035);
            let v3462 = v1033.ln();
            let v3464 = (((v2297 * v1035) * v2420) / v1034) * (v1036 * v3462);
            let v3466 = (v2323 * (v1035 * (v1033.powf((v1035 - v2270))))) + (Lanes([v3464[0], 0.0, 0.0, 0.0]));
            let v1037 = v1026 * v1036;
            let v3467 = v3444 * v1036;
            let v3470 = (Lanes([v3467[0], 0.0, 0.0, 0.0])) + (v3466 * v1026);
            let v1039 = (v2 + v1037).sqrt();
            let v1040 = v2 + v1039;
            let v1041 = v1037 / v1040;
            let v3476 = (v3470 - ((v3470 * (v2270 / (v2704 * v1039))) * v1041)) / v1040;
            let v1042 = if v640 == v0 { 1.0 } else { 0.0 };
            let v1063: f64;
            let v2338: Lanes<5>;
            if v1042 != 0.0 {
                let v1043 = v974 / v592;
                let v3515 = v2908 * v1043;
                let v3518 = (v3362 - (Lanes([v3515[0], 0.0, 0.0]))) / v592;
                let v1045 = v1024 / v589;
                let v3519 = v2904 * v1045;
                let v3522 = (v3440 - (Lanes([v3519[0], 0.0, 0.0, 0.0]))) / v589;
                let v1046 = (v2 + v1043) + v1045;
                let v3525 = (Lanes([v3518[0], v3518[1], v3518[2], 0.0, 0.0])) + (Lanes([v3522[0], 0.0, v3522[1], v3522[2], v3522[3]]));
                v1063 = v1046;
                v2338 = v3525;
            } else {
                let v1047 = v974 / v592;
                let v3477 = v2908 * v1047;
                let v1048 = v1047 + v2;
                let v1049 = v1048 * v641;
                let v3482 = v2936 * v1048;
                let v3486 = v2431 * v1049;
                let v1052 = (-v1024) / v589;
                let v3490 = v2904 * v1052;
                let v1053 = v1052 * v641;
                let v3495 = v2936 * v1052;
                let v3499 = v2431 * v1053;
                let v1055 = (v1049 * v107).exp();
                let v3502 = ((((((v3362 - (Lanes([v3477[0], 0.0, 0.0]))) / v592) * v641) + (Lanes([v3482[0], 0.0, 0.0]))) * v107) + (Lanes([v3486[0], 0.0, 0.0]))) * v1055;
                let v1056 = (v1053 * v107).exp();
                let v3503 = (((((((v3440 * v2420) - (Lanes([v3490[0], 0.0, 0.0, 0.0]))) / v589) * v641) + (Lanes([v3495[0], 0.0, 0.0, 0.0]))) * v107) + (Lanes([v3499[0], 0.0, 0.0, 0.0]))) * v1056;
                let v1059 = (v641 * v107).exp();
                let v1060 = v1059 - v2;
                let v1061 = (v1055 - v1056) / v1060;
                let v3511 = (((v2936 * v107) + (v2431 * v641)) * v1059) * v1061;
                let v3514 = (((Lanes([v3502[0], v3502[1], v3502[2], 0.0, 0.0])) - (Lanes([v3503[0], 0.0, v3503[1], v3503[2], v3503[3]]))) - (Lanes([v3511[0], 0.0, 0.0, 0.0, 0.0]))) / v1060;
                v1063 = v1061;
                v2338 = v3514;
            }
            let v1064 = v1063 * v1063;
            let v3526 = v2338 * v1063;
            let v3527 = v3526 + v3526;
            let v1065 = if v1063 < v0 { 1.0 } else { 0.0 };
            let v1075: f64;
            let v2339: Lanes<5>;
            if v1065 != 0.0 {
                let v1068 = (v1064 + v1062).sqrt();
                let v1069 = v1068 - v1063;
                let v1070 = v1066 / v1069;
                let v3539 = ((((v3527 * (v2270 / (v2704 * v1068))) - v2338) * v1070) * v2420) / v1069;
                v1075 = v1070;
                v2339 = v3539;
            } else {
                let v1072 = (v1064 + v1062).sqrt();
                let v1074 = v393 * (v1072 + v1063);
                let v3532 = ((v3527 * (v2270 / (v2704 * v1072))) + v2338) * v393;
                v1075 = v1074;
                v2339 = v3532;
            }
            let v3543 = ((Lanes([v3454[0], v3454[1], v3454[2], 0.0, 0.0])) + (Lanes([v3476[0], 0.0, v3476[1], v3476[2], v3476[3]]))) * v393;
            let v1078 = v2 + (v393 * (v1032 + v1041));
            let v1079 = v1075 * v1078;
            let v3546 = (v2339 * v1078) + (v3543 * v1075);
            let v1081 = v1080 * v418;
            let v1082 = v1081 * v1036;
            let v3548 = (v2730 * v1080) * v1036;
            let v3551 = (Lanes([v3548[0], 0.0, 0.0, 0.0])) + (v3466 * v1081);
            let v1083 = v418 * v1027;
            let v3552 = v2730 * v1027;
            let v3555 = (Lanes([v3552[0], 0.0, 0.0])) + (v2311 * v418);
            let v3556 = Lanes([v3555[0], v3555[1], v3555[2], 0.0, 0.0]);
            let v3557 = Lanes([v3551[0], 0.0, v3551[1], v3551[2], v3551[3]]);
            let v1085 = (v1083 - v1082) / v1079;
            let v3561 = ((v3556 - v3557) - (v3546 * v1085)) / v1079;
            let v1087 = v665 / v1086;
            let v3562 = v2957 / v1086;
            let v1088 = if v665 < v0 { 1.0 } else { 0.0 };
            let v1099: f64;
            let v2340: Lanes<2>;
            if v1088 != 0.0 {
                let v1089 = v1087.exp();
                let v1090 = v2 + v1089;
                let v1092 = v1086 * (v1090.ln());
                let v3572 = ((v3562 * v1089) * (v2270 / v1090)) * v1086;
                v1099 = v1092;
                v2340 = v3572;
            } else {
                let v1094 = (-v1087).exp();
                let v1095 = v2 + v1094;
                let v1098 = v665 + (v1086 * (v1095.ln()));
                let v3568 = v2957 + ((((v3562 * v2420) * v1094) * (v2270 / v1095)) * v1086);
                v1099 = v1098;
                v2340 = v3568;
            }
            let v1101 = v1099 / v1100;
            let v3573 = v2340 / v1100;
            let v1102 = if v1101 < v699 { 1.0 } else { 0.0 };
            let v1108: f64;
            let v2341: Lanes<2>;
            if v1102 != 0.0 {
                let v1103 = v1101.exp();
                let v3575 = v3573 * v1103;
                v1108 = v1103;
                v2341 = v3575;
            } else {
                let v1104 = v699.exp();
                let v1107 = v1104 * (v2 + (v1101 - v699));
                let v3574 = v3573 * v1104;
                v1108 = v1107;
                v2341 = v3574;
            }
            let v1109 = v1108 - v2;
            let v1110 = v536 * v1109;
            let v3576 = v2816 * v1109;
            let v3577 = v2341 * v536;
            let v3580 = (Lanes([v3576[0], 0.0, 0.0])) + (Lanes([0.0, v3577[0], v3577[1]]));
            let v1113 = (v665 - v1111) / v24;
            let v3581 = v2957 / v24;
            let v1114 = if v665 < v1111 { 1.0 } else { 0.0 };
            let v1127: f64;
            let v2342: Lanes<2>;
            if v1114 != 0.0 {
                let v1115 = v1113.exp();
                let v1116 = v2 + v1115;
                let v1119 = v665 - (v24 * (v1116.ln()));
                let v3592 = v2957 - (((v3581 * v1115) * (v2270 / v1116)) * v24);
                v1127 = v1119;
                v2342 = v3592;
            } else {
                let v1121 = (-v1113).exp();
                let v1122 = v2 + v1121;
                let v1125 = v1111 - (v24 * (v1122.ln()));
                let v3587 = ((((v3581 * v2420) * v1121) * (v2270 / v1122)) * v24) * v2420;
                v1127 = v1125;
                v2342 = v3587;
            }
            let v1128 = v1126 * v1127;
            let v1129 = v1111 - v1127;
            let v1130 = v1129 * v1129;
            let v1131 = v1128 * v1130;
            let v3599 = ((v2342 * v1126) * v1130) + (((v2342 * v2420) * (v25 * v1129)) * v1128);
            let v1132 = v706 / v460;
            let v3600 = v3026 / v460;
            let v1133 = if v1132 < v699 { 1.0 } else { 0.0 };
            let v1158: f64;
            let v2343: Lanes<3>;
            if v1133 != 0.0 {
                let v1134 = v1132.exp();
                let v3602 = v3600 * v1134;
                v1158 = v1134;
                v2343 = v3602;
            } else {
                let v1135 = v699.exp();
                let v1138 = v1135 * (v2 + (v1132 - v699));
                let v3601 = v3600 * v1135;
                v1158 = v1138;
                v2343 = v3601;
            }
            let v1805: f64;
            let v2344: Lanes<5>;
            if v479 != 0.0 {
                let v1140 = v665 - v1139;
                let v1141 = v1140 * v107;
                let v3630 = v2431 * v1140;
                let v3632 = ((v3321 - (Lanes([v2292[0], 0.0, 0.0]))) * v107) + (Lanes([v3630[0], 0.0, 0.0]));
                let v1142 = if v1141 < v699 { 1.0 } else { 0.0 };
                let v1164: f64;
                let v2345: Lanes<3>;
                if v1142 != 0.0 {
                    let v1143 = v1141.exp();
                    let v3634 = v3632 * v1143;
                    v1164 = v1143;
                    v2345 = v3634;
                } else {
                    let v1144 = v699.exp();
                    let v1147 = v1144 * (v2 + (v1141 - v699));
                    let v3633 = v3632 * v1144;
                    v1164 = v1147;
                    v2345 = v3633;
                }
                let v1148 = v1085 / v418;
                let v3635 = v2730 * v1148;
                let v3638 = (v3561 - (Lanes([v3635[0], 0.0, 0.0, 0.0, 0.0]))) / v418;
                let v1150 = v1148 - v1149;
                let v1152 = if v1150 < v1151 { 1.0 } else { 0.0 };
                let v1177: f64;
                let v2346: Lanes<5>;
                if v1152 != 0.0 {
                    let v1153 = v1150.exp();
                    let v3640 = v3638 * v1153;
                    v1177 = v1153;
                    v2346 = v3640;
                } else {
                    let v1157 = v1154 * (v2 + (v1150 - v1151));
                    let v3639 = v3638 * v1154;
                    v1177 = v1157;
                    v2346 = v3639;
                }
                let v1159 = v1158 - v2;
                let v3641 = v2764 * v1159;
                let v3644 = (Lanes([v3641[0], 0.0, 0.0])) + (v2343 * v469);
                let v1162 = v1161 * v25;
                let v3646 = (v2300 * v25) * v1159;
                let v1167 = (v2 + (v404 * v1164)).sqrt();
                let v1168 = v2 + v1167;
                let v1169 = (v1162 * v1159) / v1168;
                let v1170 = v1024 / v589;
                let v3657 = v2904 * v1170;
                let v1171 = v2 + v1170;
                let v3661 = ((((Lanes([v3646[0], 0.0, 0.0])) + (v2343 * v1162)) - (((v2345 * v404) * (v2270 / (v2704 * v1167))) * v1169)) / v1168) * v1171;
                let v3662 = ((v3440 - (Lanes([v3657[0], 0.0, 0.0, 0.0]))) / v589) * v1169;
                let v1175 = v1033 - v2;
                let v1176 = v1174 * v1175;
                let v3668 = v2301 * v1175;
                let v3672 = ((Lanes([v3668[0], 0.0, 0.0, 0.0])) + (v2323 * v1174)) * v1177;
                let v1179 = v2 + v1177;
                let v1180 = (v1176 * v1177) / v1179;
                let v1181 = ((v469 * v1159) + (v1169 * v1171)) + v1180;
                let v3679 = ((Lanes([v3644[0], v3644[1], v3644[2], 0.0, 0.0])) + ((Lanes([v3661[0], v3661[1], v3661[2], 0.0, 0.0])) + (Lanes([v3662[0], 0.0, v3662[1], v3662[2], v3662[3]])))) + ((((Lanes([v3672[0], 0.0, v3672[1], v3672[2], v3672[3]])) + (v2346 * v1176)) - (v2346 * v1180)) / v1179);
                v1805 = v1181;
                v2344 = v3679;
            } else {
                let v1183 = if v1182 == v0 { 1.0 } else { 0.0 };
                let v1806: f64;
                let v2347: Lanes<5>;
                if v1183 != 0.0 {
                    let v1184 = v1158 - v2;
                    let v1185 = v469 * v1184;
                    let v3622 = v2764 * v1184;
                    let v3625 = (Lanes([v3622[0], 0.0, 0.0])) + (v2343 * v469);
                    let v3626 = Lanes([v3625[0], v3625[1], v3625[2], 0.0, 0.0]);
                    v1806 = v1185;
                    v2347 = v3626;
                } else {
                    let v1186 = v2 - v1182;
                    let v3603 = v2343 * v1186;
                    let v1191 = v1182 * ((v1158 + v1033) - v25);
                    let v1192 = v1024 / v589;
                    let v3608 = v2904 * v1192;
                    let v1193 = v2 + v1192;
                    let v3613 = ((v3440 - (Lanes([v3608[0], 0.0, 0.0, 0.0]))) / v589) * v1191;
                    let v1195 = (v1186 * (v1158 - v2)) + (v1191 * v1193);
                    let v1196 = v469 * v1195;
                    let v3618 = v2764 * v1195;
                    let v3621 = (Lanes([v3618[0], 0.0, 0.0, 0.0, 0.0])) + (((Lanes([v3603[0], v3603[1], v3603[2], 0.0, 0.0])) + (((((Lanes([v2343[0], v2343[1], v2343[2], 0.0, 0.0])) + (Lanes([v2323[0], 0.0, v2323[1], v2323[2], v2323[3]]))) * v1182) * v1193) + (Lanes([v3613[0], 0.0, v3613[1], v3613[2], v3613[3]])))) * v469);
                    v1806 = v1196;
                    v2347 = v3621;
                }
                v1805 = v1806;
                v2344 = v2347;
            }
            let v1197 = v668 * v107;
            let v3680 = v2961 * v107;
            let v3681 = v2431 * v668;
            let v3684 = (Lanes([0.0, v3680[0], v3680[1]])) + (Lanes([v3681[0], 0.0, 0.0]));
            let v1198 = v1197 / v471;
            let v3685 = v3684 / v471;
            let v1199 = if v1198 < v699 { 1.0 } else { 0.0 };
            let v1213: f64;
            let v2348: Lanes<3>;
            if v1199 != 0.0 {
                let v1200 = v1198.exp();
                let v3687 = v3685 * v1200;
                v1213 = v1200;
                v2348 = v3687;
            } else {
                let v1201 = v699.exp();
                let v1204 = v1201 * (v2 + (v1198 - v699));
                let v3686 = v3685 * v1201;
                v1213 = v1204;
                v2348 = v3686;
            }
            let v1819: f64;
            let v2349: Lanes<3>;
            if v479 != 0.0 {
                let v1205 = v668 - v1139;
                let v1206 = v1205 * v107;
                let v3696 = v2431 * v1205;
                let v3698 = (((Lanes([0.0, v2961[0], v2961[1]])) - (Lanes([v2292[0], 0.0, 0.0]))) * v107) + (Lanes([v3696[0], 0.0, 0.0]));
                let v1207 = if v1206 < v699 { 1.0 } else { 0.0 };
                let v1219: f64;
                let v2350: Lanes<3>;
                if v1207 != 0.0 {
                    let v1208 = v1206.exp();
                    let v3700 = v3698 * v1208;
                    v1219 = v1208;
                    v2350 = v3700;
                } else {
                    let v1209 = v699.exp();
                    let v1212 = v1209 * (v2 + (v1206 - v699));
                    let v3699 = v3698 * v1209;
                    v1219 = v1212;
                    v2350 = v3699;
                }
                let v1214 = v1213 - v2;
                let v3701 = v2772 * v1214;
                let v1217 = v1216 * v25;
                let v3706 = (v2302 * v25) * v1214;
                let v1222 = (v2 + (v404 * v1219)).sqrt();
                let v1223 = v2 + v1222;
                let v1224 = (v1217 * v1214) / v1223;
                let v1225 = (v477 * v1214) + v1224;
                let v3717 = ((Lanes([v3701[0], 0.0, 0.0])) + (v2348 * v477)) + ((((Lanes([v3706[0], 0.0, 0.0])) + (v2348 * v1217)) - (((v2350 * v404) * (v2270 / (v2704 * v1222))) * v1224)) / v1223);
                v1819 = v1225;
                v2349 = v3717;
            } else {
                let v1226 = v1213 - v2;
                let v1227 = v477 * v1226;
                let v3688 = v2772 * v1226;
                let v3691 = (Lanes([v3688[0], 0.0, 0.0])) + (v2348 * v477);
                v1819 = v1227;
                v2349 = v3691;
            }
            let v1228 = v706 / v432;
            let v3718 = v3026 / v432;
            let v1229 = if v1228 < v699 { 1.0 } else { 0.0 };
            let v1235: f64;
            let v2351: Lanes<3>;
            if v1229 != 0.0 {
                let v1230 = v1228.exp();
                let v3720 = v3718 * v1230;
                v1235 = v1230;
                v2351 = v3720;
            } else {
                let v1231 = v699.exp();
                let v1234 = v1231 * (v2 + (v1228 - v699));
                let v3719 = v3718 * v1231;
                v1235 = v1234;
                v2351 = v3719;
            }
            let v1236 = v1235 - v2;
            let v1237 = v443 * v1236;
            let v3721 = v2745 * v1236;
            let v3724 = (Lanes([v3721[0], 0.0, 0.0])) + (v2351 * v443);
            let v1238 = v1197 / v512;
            let v3725 = v3684 / v512;
            let v1239 = if v1238 < v699 { 1.0 } else { 0.0 };
            let v1245: f64;
            let v2352: Lanes<3>;
            if v1239 != 0.0 {
                let v1240 = v1238.exp();
                let v3727 = v3725 * v1240;
                v1245 = v1240;
                v2352 = v3727;
            } else {
                let v1241 = v699.exp();
                let v1244 = v1241 * (v2 + (v1238 - v699));
                let v3726 = v3725 * v1241;
                v1245 = v1244;
                v2352 = v3726;
            }
            let v1246 = v1245 - v2;
            let v1247 = v520 * v1246;
            let v3728 = v2799 * v1246;
            let v3731 = (Lanes([v3728[0], 0.0, 0.0])) + (v2352 * v520);
            let v1248 = v714 / v445;
            let v3732 = v3037 / v445;
            let v1249 = if v1248 < v699 { 1.0 } else { 0.0 };
            let v1255: f64;
            let v2353: Lanes<6>;
            if v1249 != 0.0 {
                let v1250 = v1248.exp();
                let v3734 = v3732 * v1250;
                v1255 = v1250;
                v2353 = v3734;
            } else {
                let v1251 = v699.exp();
                let v1254 = v1251 * (v2 + (v1248 - v699));
                let v3733 = v3732 * v1251;
                v1255 = v1254;
                v2353 = v3733;
            }
            let v1256 = v1255 - v2;
            let v1257 = v455 * v1256;
            let v3735 = v2754 * v1256;
            let v3738 = (Lanes([v3735[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2353 * v455);
            let v1258 = v1197 / v522;
            let v3739 = v3684 / v522;
            let v1259 = if v1258 < v699 { 1.0 } else { 0.0 };
            let v1265: f64;
            let v2354: Lanes<3>;
            if v1259 != 0.0 {
                let v1260 = v1258.exp();
                let v3741 = v3739 * v1260;
                v1265 = v1260;
                v2354 = v3741;
            } else {
                let v1261 = v699.exp();
                let v1264 = v1261 * (v2 + (v1258 - v699));
                let v3740 = v3739 * v1261;
                v1265 = v1264;
                v2354 = v3740;
            }
            let v1266 = v1265 - v2;
            let v1267 = v529 * v1266;
            let v3742 = v2807 * v1266;
            let v3745 = (Lanes([v3742[0], 0.0, 0.0])) + (v2354 * v529);
            let v1271 = if (if (if v551 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v542 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v1088 != 0.0 { 1.0 } else { 0.0 };
            let v1810: f64;
            let v2355: Lanes<3>;
            if v1271 != 0.0 {
                let v1272 = v25 * v968;
                let v1273 = v28 / v1272;
                let v1274 = v2 - v1273;
                let v1275 = v550 * v1274;
                let v3752 = v2840 * v1274;
                let v3755 = (Lanes([v3752[0], 0.0, 0.0])) + ((((((v3353 * v25) * v1273) * v2420) / v1272) * v2420) * v550);
                let v1276 = if v1275 < v699 { 1.0 } else { 0.0 };
                let v1337: f64;
                let v2356: Lanes<3>;
                if v1276 != 0.0 {
                    let v1277 = v1275.exp();
                    let v3757 = v3755 * v1277;
                    v1337 = v1277;
                    v2356 = v3757;
                } else {
                    let v1278 = v699.exp();
                    let v1281 = v1278 * (v2 + (v1275 - v699));
                    let v3756 = v3755 * v1278;
                    v1337 = v1281;
                    v2356 = v3756;
                }
                let v1282 = v665 * v283;
                let v3758 = v2957 * v283;
                let v3759 = v2627 * v665;
                let v3762 = (Lanes([0.0, v3758[0], v3758[1]])) + (Lanes([v3759[0], 0.0, 0.0]));
                let v3763 = v3762 * v1282;
                let v1286 = ((v1282 * v1282) + v1284).sqrt();
                let v1288 = v1287 - v26;
                let v1289 = v1286.powf(v1288);
                let v1293 = v26 - v2;
                let v1297 = v431 * v1282;
                let v1298 = v1297 * v1282;
                let v1299 = v1293 + v1282;
                let v1301 = (v26 * ((v2 - (v26 * v26)) - ((v152 * v1282) * v1293))) - (v1298 * v1299);
                let v1304 = (v1289 * v1301) * v1303;
                let v1305 = v665 * v28;
                let v3789 = (v2957 * v28) * v550;
                let v3790 = v2840 * v1305;
                let v1307 = v537 * v1304;
                let v3794 = v2285 * v1304;
                let v1308 = (v1305 * v550) / v1307;
                let v3800 = (((Lanes([0.0, v3789[0], v3789[1]])) + (Lanes([v3790[0], 0.0, 0.0]))) - (((Lanes([v3794[0], 0.0, 0.0])) + (((((((v3763 + v3763) * (v2270 / (v2704 * v1286))) * (v1288 * (v1286.powf((v1288 - v2270))))) * v1301) + ((((((v3762 * v152) * v1293) * v2420) * v26) - (((((v3762 * v431) * v1282) + (v3762 * v1297)) * v1299) + (v3762 * v1298))) * v1289)) * v1303) * v537)) * v1308)) / v1307;
                let v1310 = if v1308 < v1309 { 1.0 } else { 0.0 };
                let v1334: f64;
                let v2357: Lanes<3>;
                if v1310 != 0.0 {
                    let v1311 = if v1308 < v699 { 1.0 } else { 0.0 };
                    let v1318: f64;
                    let v2358: Lanes<3>;
                    if v1311 != 0.0 {
                        let v1312 = v1308.exp();
                        let v3815 = v3800 * v1312;
                        v1318 = v1312;
                        v2358 = v3815;
                    } else {
                        let v1313 = v699.exp();
                        let v1316 = v1313 * (v2 + (v1308 - v699));
                        let v3814 = v3800 * v1313;
                        v1318 = v1316;
                        v2358 = v3814;
                    }
                    let v1317 = -v665;
                    let v1320 = (v2 - v1318) / v1308;
                    let v1321 = v2 + v1320;
                    let v1322 = v1317 * v1321;
                    let v3821 = (v2957 * v2420) * v1321;
                    let v3824 = (Lanes([0.0, v3821[0], v3821[1]])) + ((((v2358 * v2420) - (v3800 * v1320)) / v1308) * v1317);
                    v1334 = v1322;
                    v2357 = v3824;
                } else {
                    let v1323 = v665 * v393;
                    let v1324 = v1323 * v1308;
                    let v3802 = (v2957 * v393) * v1308;
                    let v1326 = v1308 * v1325;
                    let v1329 = v2 + (v1327 * v1308);
                    let v1331 = v2 + (v1326 * v1329);
                    let v1332 = v1324 * v1331;
                    let v3813 = (((Lanes([0.0, v3802[0], v3802[1]])) + (v3800 * v1323)) * v1331) + ((((v3800 * v1325) * v1329) + ((v3800 * v1327) * v1326)) * v1324);
                    v1334 = v1332;
                    v2357 = v3813;
                }
                let v1333 = v25 * v560;
                let v1335 = v1333 * v1334;
                let v3826 = (v2857 * v25) * v1334;
                let v1336 = v1335 * v968;
                let v1338 = v1336 * v1337;
                let v3837 = v2627 * v1338;
                let v1340 = (v1338 * v283) * v29;
                let v3840 = ((((((((Lanes([v3826[0], 0.0, 0.0])) + (v2357 * v1333)) * v968) + (v3353 * v1335)) * v1337) + (v2356 * v1336)) * v283) + (Lanes([v3837[0], 0.0, 0.0]))) * v29;
                v1810 = v1340;
                v2355 = v3840;
            } else {
                v1810 = v0;
                v2355 = v3746;
            }
            let v1345 = if (if (if v575 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v566 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v659 < v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v1525: f64;
            let v2359: Lanes<3>;
            if v1345 != 0.0 {
                let v1346 = v659 * v285;
                let v3841 = v2949 * v285;
                let v3842 = v2630 * v659;
                let v3845 = (Lanes([0.0, v3841[0], v3841[1]])) + (Lanes([v3842[0], 0.0, 0.0]));
                let v1347 = v2 - v1346;
                let v1348 = v1347.powf(v1009);
                let v3849 = (v3845 * v2420) * (v1009 * (v1347.powf(v3409)));
                let v1349 = v25 * v1348;
                let v1350 = v60 / v1349;
                let v1351 = v2 - v1350;
                let v1352 = v574 * v1351;
                let v3855 = v2881 * v1351;
                let v3858 = (Lanes([v3855[0], 0.0, 0.0])) + ((((((v3849 * v25) * v1350) * v2420) / v1349) * v2420) * v574);
                let v1353 = if v1352 < v699 { 1.0 } else { 0.0 };
                let v1409: f64;
                let v2360: Lanes<3>;
                if v1353 != 0.0 {
                    let v1354 = v1352.exp();
                    let v3860 = v3858 * v1354;
                    v1409 = v1354;
                    v2360 = v3860;
                } else {
                    let v1355 = v699.exp();
                    let v1358 = v1355 * (v2 + (v1352 - v699));
                    let v3859 = v3858 * v1355;
                    v1409 = v1358;
                    v2360 = v3859;
                }
                let v3861 = v3845 * v1346;
                let v1361 = ((v1346 * v1346) + v1284).sqrt();
                let v1363 = v1362 - v58;
                let v1364 = v1361.powf(v1363);
                let v1368 = v58 - v2;
                let v1372 = v431 * v1346;
                let v1373 = v1372 * v1346;
                let v1374 = v1368 + v1346;
                let v1376 = (v58 * ((v2 - (v58 * v58)) - ((v152 * v1346) * v1368))) - (v1373 * v1374);
                let v1378 = (v1364 * v1376) * v1303;
                let v1379 = v659 * v60;
                let v3887 = (v2949 * v60) * v574;
                let v3888 = v2881 * v1379;
                let v1381 = v561 * v1378;
                let v3892 = v2286 * v1378;
                let v1382 = (v1379 * v574) / v1381;
                let v3898 = (((Lanes([0.0, v3887[0], v3887[1]])) + (Lanes([v3888[0], 0.0, 0.0]))) - (((Lanes([v3892[0], 0.0, 0.0])) + (((((((v3861 + v3861) * (v2270 / (v2704 * v1361))) * (v1363 * (v1361.powf((v1363 - v2270))))) * v1376) + ((((((v3845 * v152) * v1368) * v2420) * v58) - (((((v3845 * v431) * v1346) + (v3845 * v1372)) * v1374) + (v3845 * v1373))) * v1364)) * v1303) * v561)) * v1382)) / v1381;
                let v1384 = if v1382 < v1383 { 1.0 } else { 0.0 };
                let v1406: f64;
                let v2361: Lanes<3>;
                if v1384 != 0.0 {
                    let v1385 = if v1382 < v699 { 1.0 } else { 0.0 };
                    let v1392: f64;
                    let v2362: Lanes<3>;
                    if v1385 != 0.0 {
                        let v1386 = v1382.exp();
                        let v3913 = v3898 * v1386;
                        v1392 = v1386;
                        v2362 = v3913;
                    } else {
                        let v1387 = v699.exp();
                        let v1390 = v1387 * (v2 + (v1382 - v699));
                        let v3912 = v3898 * v1387;
                        v1392 = v1390;
                        v2362 = v3912;
                    }
                    let v1391 = -v659;
                    let v1394 = (v2 - v1392) / v1382;
                    let v1395 = v2 + v1394;
                    let v1396 = v1391 * v1395;
                    let v3919 = (v2949 * v2420) * v1395;
                    let v3922 = (Lanes([0.0, v3919[0], v3919[1]])) + ((((v2362 * v2420) - (v3898 * v1394)) / v1382) * v1391);
                    v1406 = v1396;
                    v2361 = v3922;
                } else {
                    let v1397 = v659 * v393;
                    let v1398 = v1397 * v1382;
                    let v3900 = (v2949 * v393) * v1382;
                    let v1399 = v1382 * v1325;
                    let v1401 = v2 + (v1327 * v1382);
                    let v1403 = v2 + (v1399 * v1401);
                    let v1404 = v1398 * v1403;
                    let v3911 = (((Lanes([0.0, v3900[0], v3900[1]])) + (v3898 * v1397)) * v1403) + ((((v3898 * v1325) * v1401) + ((v3898 * v1327) * v1399)) * v1398);
                    v1406 = v1404;
                    v2361 = v3911;
                }
                let v1405 = v25 * v584;
                let v1407 = v1405 * v1406;
                let v3924 = (v2898 * v25) * v1406;
                let v1408 = v1407 * v1348;
                let v1410 = v1408 * v1409;
                let v3935 = v2630 * v1410;
                let v1412 = (v1410 * v285) * v61;
                let v3938 = ((((((((Lanes([v3924[0], 0.0, 0.0])) + (v2361 * v1405)) * v1348) + (v3849 * v1407)) * v1409) + (v2360 * v1408)) * v285) + (Lanes([v3935[0], 0.0, 0.0]))) * v61;
                v1525 = v1412;
                v2359 = v3938;
            } else {
                v1525 = v0;
                v2359 = v3102;
            }
            let v1414 = v1026 * v1413;
            let v3939 = v3444 * v1413;
            let v3942 = (Lanes([v3939[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2312 * v1026);
            let v1416 = v404 * v1415;
            let v3943 = v2316 * v404;
            let v1419 = (v2 + v1414).sqrt();
            let v1420 = v2 + v1419;
            let v1421 = (v1414 - v1026) / v1420;
            let v3951 = ((v3942 - (Lanes([v3444[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) - ((v3942 * (v2270 / (v2704 * v1419))) * v1421)) / v1420;
            let v1423 = (v2 + v1416).sqrt();
            let v1424 = v2 + v1423;
            let v1425 = v1416 / v1424;
            let v3957 = (v3943 - ((v3943 * (v2270 / (v2704 * v1423))) * v1425)) / v1424;
            let v1426 = v25 * v510;
            let v3958 = v2791 * v25;
            let v1427 = v1413 - v2;
            let v3959 = v3958 * v1427;
            let v1430 = (v404 * v510) / v429;
            let v3966 = ((v2791 * v404) - (v2736 * v1430)) / v429;
            let v3967 = v3966 * v1413;
            let v1433 = (v2 + (v1430 * v1413)).sqrt();
            let v1434 = v2 + v1433;
            let v1435 = (v1426 * v1427) / v1434;
            let v3976 = (((Lanes([v3959[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2312 * v1426)) - ((((Lanes([v3967[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2312 * v1430)) * (v2270 / (v2704 * v1433))) * v1435)) / v1434;
            let v1438 = if v8 > v0 { 1.0 } else { 0.0 };
            let v1439 = if (if v1436 > v0 { 1.0 } else { 0.0 }) != 0.0 && v1438 != 0.0 { 1.0 } else { 0.0 };
            let v1529: f64;
            let v1532: f64;
            let v2024: f64;
            let v2363: Lanes<6>;
            let v2364: Lanes<9>;
            let v2365: Lanes<9>;
            if v1439 != 0.0 {
                let v1440 = v1435 * v9;
                let v3978 = v3976 * v9;
                let v1441 = v8 * v25;
                let v1442 = v1441 * v510;
                let v1444 = v1443 - v2;
                let v3980 = (v2791 * v1441) * v1444;
                let v3984 = v3966 * v1443;
                let v1448 = (v2 + (v1430 * v1443)).sqrt();
                let v1449 = v2 + v1448;
                let v1450 = (v1442 * v1444) / v1449;
                let v3993 = (((Lanes([0.0, 0.0, v3980[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2314 * v1442)) - ((((Lanes([0.0, 0.0, v3984[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2314 * v1430)) * (v2270 / (v2704 * v1448))) * v1450)) / v1449;
                let v1451 = if v1436 == v2 { 1.0 } else { 0.0 };
                let v1476: f64;
                let v2366: Lanes<9>;
                if v1451 != 0.0 {
                    let v1452 = v8 * v510;
                    let v1453 = v1452 * v326;
                    let v3997 = ((v2791 * v8) * v326) + (v2666 * v1452);
                    let v1454 = v1453 * v107;
                    let v1456 = v25 - (v1454.ln());
                    let v4006 = (v2428 * v1456) + (((((v3997 * v107) + (v2431 * v1453)) * (v2270 / v1454)) * v2420) * v105);
                    let v1458 = v697 - (v105 * v1456);
                    let v4008 = v3054 - (Lanes([0.0, 0.0, v4006[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]));
                    let v1460 = v1458 * v1458;
                    let v4009 = v4008 * v1458;
                    let v4010 = v4009 + v4009;
                    let v1461 = if v1458 < v0 { 1.0 } else { 0.0 };
                    let v1471: f64;
                    let v2367: Lanes<9>;
                    if v1461 != 0.0 {
                        let v1464 = (v1460 + v1459).sqrt();
                        let v1465 = v1464 - v1458;
                        let v1466 = v1462 / v1465;
                        let v4022 = ((((v4010 * (v2270 / (v2704 * v1464))) - v4008) * v1466) * v2420) / v1465;
                        v1471 = v1466;
                        v2367 = v4022;
                    } else {
                        let v1468 = (v1460 + v1459).sqrt();
                        let v1470 = v393 * (v1468 + v1458);
                        let v4015 = ((v4010 * (v2270 / (v2704 * v1468))) + v4008) * v393;
                        v1471 = v1470;
                        v2367 = v4015;
                    }
                    let v4024 = v2666 * v1450;
                    let v1474 = (v1453 + (v1450 * v326)) + v1471;
                    let v1475 = v1471 / v1474;
                    let v4032 = (v2367 - ((((Lanes([0.0, 0.0, v3997[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v3993 * v326) + (Lanes([0.0, 0.0, v4024[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])))) + v2367) * v1475)) / v1474;
                    v1476 = v1475;
                    v2366 = v4032;
                } else {
                    v1476 = v2;
                    v2366 = v3977;
                }
                let v1477 = v1476 * v1450;
                let v4035 = (v2366 * v1450) + (v3993 * v1476);
                v1529 = v1440;
                v1532 = v1477;
                v2024 = v1476;
                v2363 = v3978;
                v2364 = v4035;
                v2365 = v2366;
            } else {
                v1529 = v1435;
                v1532 = v0;
                v2024 = v2;
                v2363 = v3976;
                v2364 = v3977;
                v2365 = v3977;
            }
            let v1479 = if v1478 == v2 { 1.0 } else { 0.0 };
            let v1526: f64;
            let v2368: Lanes<3>;
            if v1479 != 0.0 {
                let v1480 = v670 + v659;
                let v4039 = (Lanes([v2965[0], v2965[1], 0.0])) + (Lanes([0.0, v2949[0], v2949[1]]));
                let v1486 = (v1483 * v1480) * v1485;
                let v1487 = v1486 * v1480;
                let v4044 = (((v4039 * v1483) * v1485) * v1480) + (v4039 * v1486);
                let v1490 = if (v1488 * v1480) < v0 { 1.0 } else { 0.0 };
                let v1516: f64;
                let v2369: Lanes<3>;
                if v1490 != 0.0 {
                    let v1493 = (v1487 + v1482).sqrt();
                    let v1496 = v1493 - (v1494 * v1480);
                    let v1497 = v1491 / v1496;
                    let v4058 = ((((v4044 * (v2270 / (v2704 * v1493))) - (v4039 * v1494)) * v1497) * v2420) / v1496;
                    v1516 = v1497;
                    v2369 = v4058;
                } else {
                    let v1499 = (v1487 + v1482).sqrt();
                    let v1503 = v393 * (v1499 + (v1500 * v1480));
                    let v4050 = ((v4044 * (v2270 / (v2704 * v1499))) + (v4039 * v1500)) * v393;
                    v1516 = v1503;
                    v2369 = v4050;
                }
                let v1507 = v2 / (v2 - (v88.powf(v1504)));
                let v1509 = v88 * v1508;
                let v1515 = (((v1507 * v1507) * (v88.powf((v1504 - v2)))) * v1504) / v1508;
                let v1517 = if v1516 < v1509 { 1.0 } else { 0.0 };
                let v1527: f64;
                let v2370: Lanes<3>;
                if v1517 != 0.0 {
                    let v1518 = v1516 / v1508;
                    let v1520 = v2 - (v1518.powf(v1504));
                    let v1521 = v2 / v1520;
                    let v4068 = (((((v2369 / v1508) * (v1504 * (v1518.powf((v1504 - v2270))))) * v2420) * v1521) * v2420) / v1520;
                    v1527 = v1521;
                    v2370 = v4068;
                } else {
                    let v4059 = v2369 * v1515;
                    let v1524 = v1507 + ((v1516 - v1509) * v1515);
                    v1527 = v1524;
                    v2370 = v4059;
                }
                v1526 = v1527;
                v2368 = v2370;
            } else {
                v1526 = v2;
                v2368 = v4036;
            }
            let v1528 = v1525 * v1526;
            let v4069 = v2359 * v1526;
            let v4070 = v2368 * v1525;
            let v4073 = (Lanes([v4069[0], 0.0, v4069[1], v4069[2]])) + (Lanes([0.0, v4070[0], v4070[1], v4070[2]]));
            let v1530 = v1529 * v1526;
            let v4075 = v2368 * v1529;
            let v4077 = (v2363 * v1526) + (Lanes([0.0, v4075[0], v4075[1], v4075[2], 0.0, 0.0]));
            let v1531 = v1257 * v1526;
            let v4079 = v2368 * v1257;
            let v4081 = (v3738 * v1526) + (Lanes([0.0, v4079[0], v4079[1], v4079[2], 0.0, 0.0]));
            let v1533 = v1532 * v1526;
            let v4083 = v2368 * v1532;
            let v4085 = (v2364 * v1526) + (Lanes([0.0, 0.0, 0.0, v4083[0], v4083[1], v4083[2], 0.0, 0.0, 0.0]));
            let v1534 = v974 / v592;
            let v4086 = v2908 * v1534;
            let v4089 = (v3362 - (Lanes([v4086[0], 0.0, 0.0]))) / v592;
            let v1536 = v1024 / v589;
            let v4090 = v2904 * v1536;
            let v4093 = (v3440 - (Lanes([v4090[0], 0.0, 0.0, 0.0]))) / v589;
            let v1537 = (v2 + v1534) + v1536;
            let v4096 = (Lanes([v4089[0], v4089[1], v4089[2], 0.0, 0.0])) + (Lanes([v4093[0], 0.0, v4093[1], v4093[2], v4093[3]]));
            let v1539 = v1537 * v1537;
            let v4097 = v4096 * v1537;
            let v4098 = v4097 + v4097;
            let v1540 = if v1537 < v0 { 1.0 } else { 0.0 };
            let v1550: f64;
            let v2371: Lanes<5>;
            if v1540 != 0.0 {
                let v1543 = (v1539 + v1538).sqrt();
                let v1544 = v1543 - v1537;
                let v1545 = v1541 / v1544;
                let v4110 = ((((v4098 * (v2270 / (v2704 * v1543))) - v4096) * v1545) * v2420) / v1544;
                v1550 = v1545;
                v2371 = v4110;
            } else {
                let v1547 = (v1539 + v1538).sqrt();
                let v1549 = v393 * (v1547 + v1537);
                let v4103 = ((v4098 * (v2270 / (v2704 * v1547))) + v4096) * v393;
                v1550 = v1549;
                v2371 = v4103;
            }
            let v1551 = v1550 * v1078;
            let v1552 = v315 / v1551;
            let v4117 = ((Lanes([v2660[0], 0.0, 0.0, 0.0, 0.0])) - (((v2371 * v1078) + (v3543 * v1550)) * v1552)) / v1551;
            let v1553 = if v1552 < v22 { 1.0 } else { 0.0 };
            let v1554: f64;
            let v2372: Lanes<5>;
            if v1553 != 0.0 {
                v1554 = v22;
                v2372 = v4118;
            } else {
                v1554 = v1552;
                v2372 = v4117;
            }
            let v1555 = v152 * v1554;
            let v4119 = v2372 * v152;
            let v1556 = v25 * v105;
            let v1558 = v1557 - v2;
            let v4121 = (v2428 * v25) * v1558;
            let v4126 = ((Lanes([v4121[0], 0.0, 0.0])) + (v2313 * v1556)) + (Lanes([0.0, v2965[0], v2965[1]]));
            let v1561 = ((v1556 * v1558) + v670) / v1555;
            let v4127 = v4119 * v1561;
            let v4131 = ((Lanes([v4126[0], 0.0, v4126[1], v4126[2], 0.0, 0.0])) - (Lanes([v4127[0], v4127[1], 0.0, v4127[2], v4127[3], v4127[4]]))) / v1555;
            let v1562 = if v1085 > v0 { 1.0 } else { 0.0 };
            let v1776: f64;
            let v2373: Lanes<5>;
            if v1562 != 0.0 {
                let v1564 = if v1563 == v2 { 1.0 } else { 0.0 };
                let v1726: f64;
                let v2374: Lanes<5>;
                if v1564 != 0.0 {
                    let v1566 = if v659 < v1565 { 1.0 } else { 0.0 };
                    let v1727: f64;
                    let v2375: Lanes<5>;
                    if v1566 != 0.0 {
                        let v1569 = (-v1085) / v1568;
                        let v4310 = (v3561 * v2420) / v1568;
                        let v1570 = if v1569 < v699 { 1.0 } else { 0.0 };
                        let v1577: f64;
                        let v2376: Lanes<5>;
                        if v1570 != 0.0 {
                            let v1571 = v1569.exp();
                            let v4312 = v4310 * v1571;
                            v1577 = v1571;
                            v2376 = v4312;
                        } else {
                            let v1572 = v699.exp();
                            let v1575 = v1572 * (v2 + (v1569 - v699));
                            let v4311 = v4310 * v1572;
                            v1577 = v1575;
                            v2376 = v4311;
                        }
                        let v1576 = v1565 - v659;
                        let v1578 = v1576 * v1577;
                        let v4314 = (v2949 * v2420) * v1577;
                        let v4317 = (Lanes([0.0, 0.0, v4314[0], v4314[1], 0.0])) + (v2376 * v1576);
                        let v1580 = -v1579;
                        let v1582 = v1578.powf(v1581);
                        let v1583 = v1580 * v1582;
                        let v4323 = (v2299 * v2420) * v1582;
                        let v4326 = (Lanes([v4323[0], 0.0, 0.0, 0.0, 0.0])) + ((v4317 * (v1581 * (v1578.powf((v1581 - v2270))))) * v1580);
                        let v1584 = if v1583 < v699 { 1.0 } else { 0.0 };
                        let v1593: f64;
                        let v2377: Lanes<5>;
                        if v1584 != 0.0 {
                            let v1585 = v1583.exp();
                            let v4328 = v4326 * v1585;
                            v1593 = v1585;
                            v2377 = v4328;
                        } else {
                            let v1586 = v699.exp();
                            let v1589 = v1586 * (v2 + (v1583 - v699));
                            let v4327 = v4326 * v1586;
                            v1593 = v1589;
                            v2377 = v4327;
                        }
                        let v1591 = v1590 / v1579;
                        let v1592 = v1591 * v1578;
                        let v4332 = (((v2299 * v1591) * v2420) / v1579) * v1578;
                        let v1594 = v1592 * v1593;
                        let v4338 = (((Lanes([v4332[0], 0.0, 0.0, 0.0, 0.0])) + (v4317 * v1591)) * v1593) + (v2377 * v1592);
                        v1727 = v1594;
                        v2375 = v4338;
                    } else {
                        v1727 = v0;
                        v2375 = v4118;
                    }
                    v1726 = v1727;
                    v2374 = v2375;
                } else {
                    let v1595 = if v1563 == v25 { 1.0 } else { 0.0 };
                    let v1728: f64;
                    let v2378: Lanes<5>;
                    if v1595 != 0.0 {
                        let v1596 = if v659 < v735 { 1.0 } else { 0.0 };
                        let v1729: f64;
                        let v2379: Lanes<5>;
                        if v1596 != 0.0 {
                            let v1601 = (v25 * v1597) / (v1599 * v1599);
                            let v1602 = v735 - v659;
                            let v4187 = v3082 - v3081;
                            let v1603 = v1602 / v1006;
                            let v4189 = Lanes([v4187[0], v4187[1], v4187[2], 0.0]);
                            let v1606 = ((v25 * v1603) / v1601).sqrt();
                            let v4196 = ((((v4189 - (v2322 * v1603)) / v1006) * v25) / v1601) * (v2270 / (v2704 * v1606));
                            let v1608 = if v1607 == v0 { 1.0 } else { 0.0 };
                            let v1615: f64;
                            let v2380: Lanes<4>;
                            if v1608 != 0.0 {
                                v1615 = v1599;
                                v2380 = v3148;
                            } else {
                                let v1612 = v2 - (v393 * v1609);
                                let v4198 = (v2324 * v393) * v2420;
                                let v1613 = v1599 * v1612;
                                let v1614 = v1613 * v1612;
                                let v4202 = ((v4198 * v1599) * v1612) + (v4198 * v1613);
                                v1615 = v1614;
                                v2380 = v4202;
                            }
                            let v4206 = v4196 * v1606;
                            let v4208 = v2380 * v1615;
                            let v1620 = ((v1606 * v1606) + (v1615 * v1615)).sqrt();
                            let v1621 = (v1606 * v1615) / v1620;
                            let v4216 = (((v4196 * v1615) + (v2380 * v1606)) - ((((v4206 + v4206) + (v4208 + v4208)) * (v2270 / (v2704 * v1620))) * v1621)) / v1620;
                            let v1622 = v1602 / v1621;
                            let v4219 = (v4189 - (v4216 * v1622)) / v1621;
                            let v1623 = v393 * v1621;
                            let v4220 = v4216 * v393;
                            let v1624 = v1623 * v1601;
                            let v4221 = v4220 * v1601;
                            let v1626 = v1622 + (v1624 * v1006);
                            let v4225 = v4219 + ((v4221 * v1006) + (v2322 * v1624));
                            let v1653: f64;
                            let v2381: Lanes<5>;
                            if v1608 != 0.0 {
                                let v4260 = Lanes([v4225[0], 0.0, v4225[1], v4225[2], v4225[3]]);
                                v1653 = v1626;
                                v2381 = v4260;
                            } else {
                                let v1628 = v25 * v1627;
                                let v1636 = v821 * (v2 + (v1628 * (v2 + (v25 * v1609))));
                                let v1637 = v1085 / v1636;
                                let v4229 = (((v2324 * v25) * v1628) * v821) * v1637;
                                let v1638 = ((v2 + v1627) / (v2 + v1628)) - v1637;
                                let v4234 = v4221 * v1638;
                                let v1640 = v1622 - (v1624 * v1638);
                                let v4239 = (Lanes([v4219[0], 0.0, v4219[1], v4219[2], v4219[3]])) - ((Lanes([v4234[0], 0.0, v4234[1], v4234[2], v4234[3]])) + ((((v3561 - (Lanes([v4229[0], 0.0, v4229[1], v4229[2], v4229[3]]))) / v1636) * v2420) * v1624));
                                let v1641 = v1640 - v1626;
                                let v4240 = Lanes([v4225[0], 0.0, v4225[1], v4225[2], v4225[3]]);
                                let v4242 = (v4239 - v4240) * v1641;
                                let v1643 = v40 * v1622;
                                let v1644 = v1643 * v1622;
                                let v4251 = (((((v4219 * v40) * v1622) + (v4219 * v1643)) * v1645) + (v2325 * v1644)) / v821;
                                let v1650 = ((v1641 * v1641) + ((v1644 * v1645) / v821)).sqrt();
                                let v1652 = v393 * ((v1640 + v1626) + v1650);
                                let v4259 = ((v4239 + v4240) + (((v4242 + v4242) + (Lanes([v4251[0], 0.0, v4251[1], v4251[2], v4251[3]]))) * (v2270 / (v2704 * v1650)))) * v393;
                                v1653 = v1652;
                                v2381 = v4259;
                            }
                            let v1655 = (v1653 - v1622) / v1653;
                            let v4265 = ((v2381 - (Lanes([v4219[0], 0.0, v4219[1], v4219[2], v4219[3]]))) - (v2381 * v1655)) / v1653;
                            let v1658 = if (v1655.abs()) > v1657 { 1.0 } else { 0.0 };
                            let v1730: f64;
                            let v2382: Lanes<5>;
                            if v1658 != 0.0 {
                                let v1659 = v1623 / v1655;
                                let v4280 = ((Lanes([v4220[0], 0.0, v4220[1], v4220[2], v4220[3]])) - (v4265 * v1659)) / v1655;
                                let v1662 = v1660 / v1661;
                                let v1663 = v1662 * v1653;
                                let v4284 = (((v2303 * v1662) * v2420) / v1661) * v1653;
                                let v1664 = v1663 * v1659;
                                let v4291 = v2303 * v2420;
                                let v1666 = (-v1661) / v1653;
                                let v4295 = ((Lanes([v4291[0], 0.0, 0.0, 0.0, 0.0])) - (v2381 * v1666)) / v1653;
                                let v1667 = v1666.exp();
                                let v1668 = v1615 / v1659;
                                let v1669 = v2 + v1668;
                                let v1671 = (v1666 * v1669).exp();
                                let v1672 = v1667 - v1671;
                                let v1673 = v1664 * v1672;
                                let v4308 = (((((Lanes([v4284[0], 0.0, 0.0, 0.0, 0.0])) + (v2381 * v1662)) * v1659) + (v4280 * v1663)) * v1672) + (((v4295 * v1667) - (((v4295 * v1669) + ((((Lanes([v2380[0], 0.0, v2380[1], v2380[2], v2380[3]])) - (v4280 * v1668)) / v1659) * v1666)) * v1671)) * v1664);
                                v1730 = v1673;
                                v2382 = v4308;
                            } else {
                                let v1674 = v1660 * v1615;
                                let v4267 = v2303 * v2420;
                                let v1676 = (-v1661) / v1653;
                                let v1677 = v1676.exp();
                                let v1678 = v1674 * v1677;
                                let v4273 = (v2380 * v1660) * v1677;
                                let v4276 = (Lanes([v4273[0], 0.0, v4273[1], v4273[2], v4273[3]])) + (((((Lanes([v4267[0], 0.0, 0.0, 0.0, 0.0])) - (v2381 * v1676)) / v1653) * v1677) * v1674);
                                v1730 = v1678;
                                v2382 = v4276;
                            }
                            v1729 = v1730;
                            v2379 = v2382;
                        } else {
                            v1729 = v0;
                            v2379 = v4118;
                        }
                        v1728 = v1729;
                        v2378 = v2379;
                    } else {
                        let v1679 = if v1563 == v152 { 1.0 } else { 0.0 };
                        let v1731: f64;
                        let v2383: Lanes<5>;
                        if v1679 != 0.0 {
                            let v1680 = if v659 < v1565 { 1.0 } else { 0.0 };
                            let v1732: f64;
                            let v2384: Lanes<5>;
                            if v1680 != 0.0 {
                                let v1681 = v1565 - v659;
                                let v4132 = v2949 * v2420;
                                let v1682 = v1681.powf(v1581);
                                let v1684 = v1683 + v1085;
                                let v1685 = v1085 / v1684;
                                let v1686 = v2 - v1685;
                                let v1688 = v1686.powf(v1687);
                                let v1689 = v1682 * v1688;
                                let v4145 = (v4132 * (v1581 * (v1681.powf((v1581 - v2270))))) * v1688;
                                let v4148 = (Lanes([0.0, 0.0, v4145[0], v4145[1], 0.0])) + (((((v3561 - (v3561 * v1685)) / v1684) * v2420) * (v1687 * (v1686.powf((v1687 - v2270))))) * v1682);
                                let v1690 = if v1607 == v0 { 1.0 } else { 0.0 };
                                let v1714: f64;
                                let v2385: Lanes<5>;
                                if v1690 != 0.0 {
                                    v1714 = v1689;
                                    v2385 = v4148;
                                } else {
                                    let v1693 = (v1085 - v1691) / v1683;
                                    let v4149 = v3561 / v1683;
                                    let v1696 = (v1693 - v2) / v1695;
                                    let v4150 = v4149 / v1695;
                                    let v1697 = if v1693 < v2 { 1.0 } else { 0.0 };
                                    let v1709: f64;
                                    let v2386: Lanes<5>;
                                    if v1697 != 0.0 {
                                        let v1698 = v1696.exp();
                                        let v1699 = v2 + v1698;
                                        let v4160 = ((v4150 * v1698) * (v2270 / v1699)) * v1695;
                                        let v1702 = v2 + (v1695 * (v1699.ln()));
                                        v1709 = v1702;
                                        v2386 = v4160;
                                    } else {
                                        let v1704 = (-v1696).exp();
                                        let v1705 = v2 + v1704;
                                        let v1708 = v1693 + (v1695 * (v1705.ln()));
                                        let v4156 = v4149 + ((((v4150 * v2420) * v1704) * (v2270 / v1705)) * v1695);
                                        v1709 = v1708;
                                        v2386 = v4156;
                                    }
                                    let v1711 = v1709.powf(v1710);
                                    let v1712 = v1689 * v1711;
                                    let v4167 = (v4148 * v1711) + ((v2386 * (v1710 * (v1709.powf((v1710 - v2270))))) * v1689);
                                    v1714 = v1712;
                                    v2385 = v4167;
                                }
                                let v1713 = -v1579;
                                let v1715 = v1713 * v1714;
                                let v4169 = (v2299 * v2420) * v1714;
                                let v4172 = (Lanes([v4169[0], 0.0, 0.0, 0.0, 0.0])) + (v2385 * v1713);
                                let v1716 = if v1715 < v699 { 1.0 } else { 0.0 };
                                let v1724: f64;
                                let v2387: Lanes<5>;
                                if v1716 != 0.0 {
                                    let v1717 = v1715.exp();
                                    let v4174 = v4172 * v1717;
                                    v1724 = v1717;
                                    v2387 = v4174;
                                } else {
                                    let v1718 = v699.exp();
                                    let v1721 = v1718 * (v2 + (v1715 - v699));
                                    let v4173 = v4172 * v1718;
                                    v1724 = v1721;
                                    v2387 = v4173;
                                }
                                let v1722 = v1590 / v1579;
                                let v1723 = v1722 * v1681;
                                let v4178 = (((v2299 * v1722) * v2420) / v1579) * v1681;
                                let v4179 = v4132 * v1722;
                                let v1725 = v1723 * v1724;
                                let v4183 = ((Lanes([v4178[0], 0.0, 0.0])) + (Lanes([0.0, v4179[0], v4179[1]]))) * v1724;
                                let v4186 = (Lanes([v4183[0], 0.0, v4183[1], v4183[2], 0.0])) + (v2387 * v1723);
                                v1732 = v1725;
                                v2384 = v4186;
                            } else {
                                v1732 = v0;
                                v2384 = v4118;
                            }
                            v1731 = v1732;
                            v2383 = v2384;
                        } else {
                            v1731 = v0;
                            v2383 = v4118;
                        }
                        v1728 = v1731;
                        v2378 = v2383;
                    }
                    v1726 = v1728;
                    v2374 = v2378;
                }
                let v1733 = if v1726 > v0 { 1.0 } else { 0.0 };
                let v1777: f64;
                let v2388: Lanes<5>;
                if v1733 != 0.0 {
                    let v1735 = if v1734 == v2 { 1.0 } else { 0.0 };
                    let v1778: f64;
                    let v2389: Lanes<5>;
                    if v1735 != 0.0 {
                        let v1737 = v1736 + v1555;
                        let v4343 = (Lanes([v2294[0], 0.0, 0.0, 0.0, 0.0])) + v4119;
                        let v1738 = v1085 * v1737;
                        let v1739 = v105 / v1738;
                        let v1740 = v1079 / v418;
                        let v4351 = v2730 * v1740;
                        let v4356 = v2764 * v1740;
                        let v1744 = v1743 / v1737;
                        let v1745 = (v1739 + (v1740 * v469)) + v1744;
                        let v4364 = ((((Lanes([v2428[0], 0.0, 0.0, 0.0, 0.0])) - (((v3561 * v1737) + (v4343 * v1085)) * v1739)) / v1738) + ((((v3546 - (Lanes([v4351[0], 0.0, 0.0, 0.0, 0.0]))) / v418) * v469) + (Lanes([v4356[0], 0.0, 0.0, 0.0, 0.0])))) + (((Lanes([v2293[0], 0.0, 0.0, 0.0, 0.0])) - (v4343 * v1744)) / v1737);
                        let v1746 = if v1563 == v152 { 1.0 } else { 0.0 };
                        let v1779: f64;
                        let v2390: Lanes<5>;
                        if v1746 != 0.0 {
                            let v1748 = (v1726 - v1745) / v1481;
                            let v4376 = (v2374 - v4364) / v1481;
                            let v1749 = if v1726 < v1745 { 1.0 } else { 0.0 };
                            let v1761: f64;
                            let v2391: Lanes<5>;
                            if v1749 != 0.0 {
                                let v1750 = v1748.exp();
                                let v1751 = v2 + v1750;
                                let v1754 = v1726 - (v1481 * (v1751.ln()));
                                let v4387 = v2374 - (((v4376 * v1750) * (v2270 / v1751)) * v1481);
                                v1761 = v1754;
                                v2391 = v4387;
                            } else {
                                let v1756 = (-v1748).exp();
                                let v1757 = v2 + v1756;
                                let v1760 = v1745 - (v1481 * (v1757.ln()));
                                let v4382 = v4364 - ((((v4376 * v2420) * v1756) * (v2270 / v1757)) * v1481);
                                v1761 = v1760;
                                v2391 = v4382;
                            }
                            let v1762 = v1085 * v1761;
                            let v4390 = (v3561 * v1761) + (v2391 * v1085);
                            v1779 = v1762;
                            v2390 = v4390;
                        } else {
                            let v1763 = v1085 * v1726;
                            let v1765 = v1726 + v1745;
                            let v1766 = (v1763 * v1745) / v1765;
                            let v4374 = (((((v3561 * v1726) + (v2374 * v1085)) * v1745) + (v4364 * v1763)) - ((v2374 + v4364) * v1766)) / v1765;
                            v1779 = v1766;
                            v2390 = v4374;
                        }
                        v1778 = v1779;
                        v2389 = v2390;
                    } else {
                        let v1767 = v1085 * v1726;
                        let v4341 = (v3561 * v1726) + (v2374 * v1085);
                        v1778 = v1767;
                        v2389 = v4341;
                    }
                    v1777 = v1778;
                    v2388 = v2389;
                } else {
                    v1777 = v0;
                    v2388 = v4118;
                }
                v1776 = v1777;
                v2373 = v2388;
            } else {
                v1776 = v0;
                v2373 = v4118;
            }
            let v1768 = if v1033 > v0 { 1.0 } else { 0.0 };
            let v1770: f64;
            let v2392: Lanes<4>;
            if v1768 != 0.0 {
                let v1769 = v105 * v3462;
                let v4394 = v2428 * v3462;
                let v4397 = (Lanes([v4394[0], 0.0, 0.0, 0.0])) + ((v2323 * (v2270 / v1033)) * v105);
                v1770 = v1769;
                v2392 = v4397;
            } else {
                let v4391 = Lanes([0.0, v2953[0], 0.0, v2953[1]]);
                v1770 = v662;
                v2392 = v4391;
            }
            let v1816: f64;
            let v2393: Lanes<3>;
            if v479 != 0.0 {
                let v4399 = Lanes([v2949[0], v2949[1], 0.0]);
                v1816 = v659;
                v2393 = v4399;
            } else {
                let v4398 = Lanes([v2953[0], 0.0, v2953[1]]);
                v1816 = v662;
                v2393 = v4398;
            }
            let v1771 = v665 - v1770;
            let v1773 = v1770 - v659;
            let v4410 = (v3122 * v1773) + ((v2392 - (Lanes([0.0, v2949[0], v2949[1], 0.0]))) * v788);
            let v4414 = v2392 * v1776;
            let v4417 = (((v3561 * v1771) + (((Lanes([0.0, v2957[0], v2957[1], 0.0, 0.0])) - (Lanes([v2392[0], 0.0, v2392[1], v2392[2], v2392[3]]))) * v1085)) + (Lanes([v4410[0], 0.0, v4410[1], v4410[2], v4410[3]]))) - ((v2373 * v1770) + (Lanes([v4414[0], 0.0, v4414[1], v4414[2], v4414[3]])));
            let v4418 = v2973 * v675;
            let v4419 = v4418 + v4418;
            let v1783 = (v675 * v675) / v1743;
            let v4420 = v2293 * v1783;
            let v4424 = ((Lanes([v4419[0], 0.0, v4419[1]])) - (Lanes([0.0, v4420[0], 0.0]))) / v1743;
            let v4427 = (Lanes([0.0, v4417[0], v4417[1], v4417[2], v4417[3], v4417[4]])) + (Lanes([v4424[0], v4424[1], v4424[2], 0.0, 0.0, 0.0]));
            let v1785 = v696 * v696;
            let v4428 = v3012 * v696;
            let v4430 = (v4428 + v4428) * v1786;
            let v4431 = v2304 * v1785;
            let v4434 = (Lanes([v4430[0], v4430[1], 0.0, v4430[2], v4430[3], v4430[4], v4430[5], v4430[6], v4430[7]])) + (Lanes([0.0, 0.0, v4431[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]));
            let v1790 = v689 * v689;
            let v4438 = v2993 * v689;
            let v4440 = (v4438 + v4438) * v1791;
            let v4441 = v2306 * v1790;
            let v4444 = (Lanes([0.0, v4440[0], v4440[1]])) + (Lanes([v4441[0], 0.0, 0.0]));
            let v1795 = v686 * v686;
            let v4447 = v2989 * v686;
            let v4449 = (v4447 + v4447) * v1796;
            let v4450 = v2308 * v1795;
            let v4453 = (Lanes([0.0, v4449[0], v4449[1]])) + (Lanes([v4450[0], 0.0, 0.0]));
            let v4456 = v2977 * v678;
            let v4457 = v4456 + v4456;
            let v1801 = (v678 * v678) / v1736;
            let v4458 = v2294 * v1801;
            let v4462 = ((Lanes([v4457[0], 0.0, v4457[1]])) - (Lanes([0.0, v4458[0], 0.0]))) / v1736;
            let v4466 = v2965 * v1561;
            let v4468 = (v4131 * v670) + (Lanes([0.0, 0.0, v4466[0], v4466[1], 0.0, 0.0]));
            let v4473 = v2957 * v16;
            let v1813 = ((((v1805 + v1237) + (v16 * v665)) - v1810) + v1131) + v1110;
            let v4481 = ((((v2344 + (Lanes([v3724[0], v3724[1], v3724[2], 0.0, 0.0]))) + (Lanes([0.0, v4473[0], v4473[1], 0.0, 0.0]))) - (Lanes([v2355[0], v2355[1], v2355[2], 0.0, 0.0]))) + (Lanes([0.0, v3599[0], v3599[1], 0.0, 0.0]))) + (Lanes([v3580[0], v3580[1], v3580[2], 0.0, 0.0]));
            let v4483 = v2957 * v1813;
            let v4485 = (v4481 * v665) + (Lanes([0.0, v4483[0], v4483[1], 0.0, 0.0]));
            let v4488 = v4073 * v1816;
            let v4489 = v2393 * v1528;
            let v4492 = (Lanes([v4488[0], v4488[1], v4488[2], v4488[3], 0.0])) + (Lanes([0.0, 0.0, v4489[0], v4489[1], v4489[2]]));
            let v1821 = (v1819 + v1247) + v1267;
            let v4496 = (v2349 + v3731) + v3745;
            let v4498 = v2961 * v1821;
            let v4500 = (v4496 * v668) + (Lanes([0.0, v4498[0], v4498[1]]));
            let v1825 = v16 * v692;
            let v4504 = v3002 * v16;
            let v1826 = (v1530 + v1531) + v1825;
            let v4505 = Lanes([0.0, v4504[0], v4504[1], v4504[2], v4504[3], v4504[4]]);
            let v4508 = v3002 * v1826;
            let v4510 = (((v4077 + v4081) + v4505) * v692) + (Lanes([0.0, v4508[0], v4508[1], v4508[2], v4508[3], v4508[4]]));
            let v4514 = v3014 * v1533;
            let v4516 = (v4085 * v697) + (Lanes([v4514[0], v4514[1], 0.0, v4514[2], v4514[3], v4514[4], v4514[5], v4514[6], v4514[7]]));
            let v1830 = (((((((((((((v1085 * v1771) + (v788 * v1773)) - (v1776 * v1770)) + v1783) + (v1785 * v1786)) + (v1790 * v1791)) + (v1795 * v1796)) + v1801) + (v1561 * v670)) + (v1813 * v665)) - (v1528 * v1816)) + (v1821 * v668)) + (v1826 * v692)) + (v1533 * v697);
            let v4518 = ((((((((((Lanes([0.0, 0.0, v4427[0], v4427[1], v4427[2], 0.0, v4427[3], v4427[4], v4427[5], 0.0, 0.0])) + (Lanes([v4434[0], v4434[1], 0.0, v4434[2], 0.0, v4434[3], v4434[4], v4434[5], v4434[6], v4434[7], v4434[8]]))) + (Lanes([0.0, 0.0, 0.0, v4444[0], 0.0, 0.0, 0.0, 0.0, 0.0, v4444[1], v4444[2]]))) + (Lanes([0.0, 0.0, 0.0, v4453[0], 0.0, 0.0, 0.0, v4453[1], 0.0, 0.0, v4453[2]]))) + (Lanes([0.0, v4462[0], 0.0, v4462[1], 0.0, v4462[2], 0.0, 0.0, 0.0, 0.0, 0.0]))) + (Lanes([0.0, 0.0, 0.0, v4468[0], v4468[1], v4468[2], v4468[3], v4468[4], v4468[5], 0.0, 0.0]))) + (Lanes([0.0, 0.0, 0.0, v4485[0], v4485[1], 0.0, v4485[2], v4485[3], v4485[4], 0.0, 0.0]))) - (Lanes([0.0, 0.0, 0.0, v4492[0], 0.0, v4492[1], v4492[2], v4492[3], v4492[4], 0.0, 0.0]))) + (Lanes([0.0, 0.0, 0.0, v4500[0], v4500[1], v4500[2], 0.0, 0.0, 0.0, 0.0, 0.0]))) + (Lanes([0.0, 0.0, 0.0, v4510[0], 0.0, v4510[1], v4510[2], v4510[3], v4510[4], 0.0, v4510[5]]))) + (Lanes([v4516[0], v4516[1], 0.0, v4516[2], 0.0, v4516[3], v4516[4], v4516[5], v4516[6], v4516[7], v4516[8]]));
            let v1832 = v2 - v1831;
            let v1833 = v1832 * v291;
            let v4519 = v2641 * v1832;
            let v1834 = v1833 * v974;
            let v4520 = v4519 * v974;
            let v4523 = (Lanes([v4520[0], 0.0, 0.0])) + (v3362 * v1833);
            let v4524 = Lanes([0.0, v2961[0], v2961[1]]);
            let v4525 = Lanes([v3319[0], 0.0, 0.0]);
            let v1836 = (v668 - v948) / v949;
            let v4527 = v3320 * v1836;
            let v4530 = ((v4524 - v4525) - (Lanes([v4527[0], 0.0, 0.0]))) / v949;
            let v1837 = if v668 < v948 { 1.0 } else { 0.0 };
            let v1850: f64;
            let v2394: Lanes<3>;
            if v1837 != 0.0 {
                let v1838 = v1836.exp();
                let v1839 = v2 + v1838;
                let v1840 = v1839.ln();
                let v4543 = v3320 * v1840;
                let v1842 = v668 - (v949 * v1840);
                let v4547 = v4524 - ((Lanes([v4543[0], 0.0, 0.0])) + (((v4530 * v1838) * (v2270 / v1839)) * v949));
                v1850 = v1842;
                v2394 = v4547;
            } else {
                let v1844 = (-v1836).exp();
                let v1845 = v2 + v1844;
                let v1846 = v1845.ln();
                let v4535 = v3320 * v1846;
                let v1848 = v948 - (v949 * v1846);
                let v4539 = v4525 - ((Lanes([v4535[0], 0.0, 0.0])) + ((((v4530 * v2420) * v1844) * (v2270 / v1845)) * v949));
                v1850 = v1848;
                v2394 = v4539;
            }
            let v1849 = v1831 * v291;
            let v4550 = v2627 * v1850;
            let v1852 = v2 - (v1850 * v283);
            let v1854 = v2 - (v1852.powf(v967));
            let v4558 = v3354 * v1854;
            let v1858 = (v969 * v1854) + (v152 * (v668 - v1850));
            let v1859 = v1849 * v1858;
            let v4565 = (v2641 * v1831) * v1858;
            let v4568 = (Lanes([v4565[0], 0.0, 0.0])) + ((((Lanes([v4558[0], 0.0, 0.0])) + ((((((v2394 * v283) + (Lanes([v4550[0], 0.0, 0.0]))) * v2420) * (v967 * (v1852.powf(v3350)))) * v2420) * v969)) + ((v4524 - v2394) * v152)) * v1849);
            let v1861 = v1860 * v301;
            let v1862 = v1861 * v1024;
            let v4570 = (v2652 * v1860) * v1024;
            let v4573 = (Lanes([v4570[0], 0.0, 0.0, 0.0])) + (v3440 * v1861);
            let v1863 = v608 * v423;
            let v4576 = (v2919 * v423) + (v2733 * v608);
            let v1864 = v393 * v1863;
            let v4577 = v4576 * v393;
            let v1865 = v1864 * v1032;
            let v4578 = v4577 * v1032;
            let v1866 = v1865 * v1550;
            let v4582 = ((Lanes([v4578[0], 0.0, 0.0])) + (v3454 * v1864)) * v1550;
            let v4585 = (Lanes([v4582[0], v4582[1], v4582[2], 0.0, 0.0])) + (v2371 * v1865);
            let v1867 = v1864 * v1041;
            let v4586 = v4577 * v1041;
            let v1868 = v1867 * v1550;
            let v4590 = ((Lanes([v4586[0], 0.0, 0.0, 0.0])) + (v3476 * v1864)) * v1550;
            let v4593 = (Lanes([v4590[0], 0.0, v4590[1], v4590[2], v4590[3]])) + (v2371 * v1867);
            let v1869 = v40 * v294;
            let v4594 = v2290 * v40;
            let v4595 = Lanes([v3378[0], 0.0, 0.0, 0.0, 0.0, 0.0]);
            let v1871 = (v692 - v987) / v1869;
            let v4597 = v4594 * v1871;
            let v4600 = ((v3063 - v4595) - (Lanes([v4597[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v1869;
            let v1872 = if v692 < v987 { 1.0 } else { 0.0 };
            let v1884: f64;
            let v2395: Lanes<6>;
            if v1872 != 0.0 {
                let v1873 = v1871.exp();
                let v1874 = v2 + v1873;
                let v1875 = v1874.ln();
                let v4613 = v4594 * v1875;
                let v1877 = v692 - (v1869 * v1875);
                let v4617 = v3063 - ((Lanes([v4613[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (((v4600 * v1873) * (v2270 / v1874)) * v1869));
                v1884 = v1877;
                v2395 = v4617;
            } else {
                let v1879 = (-v1871).exp();
                let v1880 = v2 + v1879;
                let v1881 = v1880.ln();
                let v4605 = v4594 * v1881;
                let v1883 = v987 - (v1869 * v1881);
                let v4609 = v4595 - ((Lanes([v4605[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((v4600 * v2420) * v1879) * (v2270 / v1880)) * v1869));
                v1884 = v1883;
                v2395 = v4609;
            }
            let v1885 = v1884 / v294;
            let v4618 = v2290 * v1885;
            let v1886 = v2 - v1885;
            let v1888 = v2 - (v1886.powf(v1009));
            let v4627 = v3403 * v1888;
            let v1890 = v692 - v1884;
            let v4632 = v3370 * v1890;
            let v1892 = (v1010 * v1888) + (v982 * v1890);
            let v4637 = v3367 * v1892;
            let v4641 = v2653 * v692;
            let v4642 = v3002 * v302;
            let v1895 = (v981 * v1892) + (v302 * v692);
            let v4647 = v2652 * v1895;
            let v1897 = v2 - v1860;
            let v1899 = ((v301 * v1895) * v1897) * v9;
            let v4652 = (((Lanes([v4647[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([v4637[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([v4627[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((((v2395 - (Lanes([v4618[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v294) * v2420) * (v1009 * (v1886.powf(v3409)))) * v2420) * v1010)) + ((Lanes([v4632[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v3063 - v2395) * v982))) * v981)) + ((Lanes([v4641[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (Lanes([0.0, v4642[0], v4642[1], v4642[2], v4642[3], v4642[4]])))) * v301)) * v1897) * v9;
            let v4653 = Lanes([0.0, 0.0, v3378[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
            let v1901 = (v697 - v987) / v1869;
            let v4655 = v4594 * v1901;
            let v4658 = ((v3054 - v4653) - (Lanes([0.0, 0.0, v4655[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) / v1869;
            let v1902 = if v697 < v987 { 1.0 } else { 0.0 };
            let v1914: f64;
            let v2396: Lanes<9>;
            if v1902 != 0.0 {
                let v1903 = v1901.exp();
                let v1904 = v2 + v1903;
                let v1905 = v1904.ln();
                let v4671 = v4594 * v1905;
                let v1907 = v697 - (v1869 * v1905);
                let v4675 = v3054 - ((Lanes([0.0, 0.0, v4671[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (((v4658 * v1903) * (v2270 / v1904)) * v1869));
                v1914 = v1907;
                v2396 = v4675;
            } else {
                let v1909 = (-v1901).exp();
                let v1910 = v2 + v1909;
                let v1911 = v1910.ln();
                let v4663 = v4594 * v1911;
                let v1913 = v987 - (v1869 * v1911);
                let v4667 = v4653 - ((Lanes([0.0, 0.0, v4663[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((v4658 * v2420) * v1909) * (v2270 / v1910)) * v1869));
                v1914 = v1913;
                v2396 = v4667;
            }
            let v1915 = v1914 / v294;
            let v4676 = v2290 * v1915;
            let v1916 = v2 - v1915;
            let v1918 = v2 - (v1916.powf(v1009));
            let v4685 = v3403 * v1918;
            let v1920 = v697 - v1914;
            let v4690 = v3370 * v1920;
            let v1922 = (v1010 * v1918) + (v982 * v1920);
            let v4695 = v3367 * v1922;
            let v4699 = v2653 * v697;
            let v4700 = v3014 * v302;
            let v1925 = (v981 * v1922) + (v302 * v697);
            let v4705 = v2652 * v1925;
            let v1928 = ((v301 * v1925) * v1897) * v8;
            let v4710 = (((Lanes([0.0, 0.0, v4705[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([0.0, 0.0, v4695[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([0.0, 0.0, v4685[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((((v2396 - (Lanes([0.0, 0.0, v4676[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) / v294) * v2420) * (v1009 * (v1916.powf(v3409)))) * v2420) * v1010)) + ((Lanes([0.0, 0.0, v4690[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v3054 - v2396) * v982))) * v981)) + ((Lanes([0.0, 0.0, v4699[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (Lanes([v4700[0], v4700[1], 0.0, v4700[2], v4700[3], v4700[4], v4700[5], v4700[6], v4700[7]])))) * v301)) * v1897) * v8;
            let v1929 = v602 * v423;
            let v1930 = v418 / v423;
            let v1932 = v2 / v1931;
            let v1933 = v1930.powf(v1932);
            let v1934 = v1929 * v1933;
            let v4723 = (((v2916 * v423) + (v2733 * v602)) * v1933) + ((((v2730 - (v2733 * v1930)) / v423) * (v1932 * (v1930.powf((v1932 - v2270))))) * v1929);
            let v1935 = v1931 * v105;
            let v4724 = v2428 * v1931;
            let v1936 = v665 / v1935;
            let v4725 = v4724 * v1936;
            let v4728 = (v3321 - (Lanes([v4725[0], 0.0, 0.0]))) / v1935;
            let v1937 = if v1936 < v699 { 1.0 } else { 0.0 };
            let v1943: f64;
            let v2397: Lanes<3>;
            if v1937 != 0.0 {
                let v1938 = v1936.exp();
                let v4730 = v4728 * v1938;
                v1943 = v1938;
                v2397 = v4730;
            } else {
                let v1939 = v699.exp();
                let v1942 = v1939 * (v2 + (v1936 - v699));
                let v4729 = v4728 * v1939;
                v1943 = v1942;
                v2397 = v4729;
            }
            let v1944 = v1934 * v1943;
            let v4731 = v4723 * v1943;
            let v4734 = (Lanes([v4731[0], 0.0, 0.0])) + (v2397 * v1934);
            let v1945 = v404 * v613;
            let v1947 = (v1945 * v105) / v338;
            let v4741 = ((((v2922 * v404) * v105) + (v2428 * v1945)) - (v2673 * v1947)) / v338;
            let v1948 = v393 * v1947;
            let v1949 = v1948 * v1609;
            let v4743 = (v4741 * v393) * v1609;
            let v1952 = (v1950 + v867) + v25;
            let v1953 = v1949 * v1952;
            let v4751 = (((Lanes([v4743[0], 0.0, 0.0, 0.0])) + (v2324 * v1948)) * v1952) + ((v2326 + (Lanes([v2319[0], v2319[1], v2319[2], 0.0]))) * v1949);
            let v1955 = if v1954 == v0 { 1.0 } else { 0.0 };
            let v1985: f64;
            let v2398: Lanes<6>;
            if v1955 != 0.0 {
                let v1956 = v618 * v393;
                let v4776 = v4576 * v1421;
                let v4780 = v4741 * v1425;
                let v1959 = (v1863 * v1421) + (v1947 * v1425);
                let v4785 = (v2925 * v393) * v1959;
                let v1961 = (v1956 * v1959) / v615;
                let v4789 = v2923 * v1961;
                let v4792 = (((Lanes([v4785[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([v4776[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v3951 * v1863)) + ((Lanes([v4780[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v3957 * v1947))) * v1956)) - (Lanes([v4789[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v615;
                v1985 = v1961;
                v2398 = v4792;
            } else {
                let v1965 = (v692 - v1962) / v1964;
                let v1966 = v1965 * v107;
                let v4756 = v2431 * v1965;
                let v4758 = (((v3063 - (Lanes([v2289[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v1964) * v107) + (Lanes([v4756[0], 0.0, 0.0, 0.0, 0.0, 0.0]));
                let v1967 = if v1966 < v699 { 1.0 } else { 0.0 };
                let v1975: f64;
                let v2399: Lanes<6>;
                if v1967 != 0.0 {
                    let v1968 = v1966.exp();
                    let v4760 = v4758 * v1968;
                    v1975 = v1968;
                    v2399 = v4760;
                } else {
                    let v1969 = v699.exp();
                    let v1972 = v1969 * (v2 + (v1966 - v699));
                    let v4759 = v4758 * v1969;
                    v1975 = v1972;
                    v2399 = v4759;
                }
                let v1973 = v1426 * v624;
                let v4764 = ((v3958 * v624) + (v2928 * v1426)) * v1413;
                let v1978 = (v2 + (v404 * v1975)).sqrt();
                let v1979 = v2 + v1978;
                let v1980 = (v1973 * v1413) / v1979;
                let v4774 = (((Lanes([v4764[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2312 * v1973)) - (((v2399 * v404) * (v2270 / (v2704 * v1978))) * v1980)) / v1979;
                v1985 = v1980;
                v2398 = v4774;
            }
            let v1984 = if (if (if v1436 == v2 { 1.0 } else { 0.0 }) != 0.0 || (if v1436 == v152 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v1438 != 0.0 { 1.0 } else { 0.0 };
            let v2159: f64;
            let v2168: f64;
            let v2400: Lanes<9>;
            let v2401: Lanes<6>;
            if v1984 != 0.0 {
                let v1986 = v1985 * v9;
                let v4793 = v2398 * v9;
                let v2025: f64;
                let v2402: Lanes<9>;
                if v1955 != 0.0 {
                    let v1987 = v1026 * v1443;
                    let v4817 = v3444 * v1443;
                    let v4820 = (Lanes([0.0, 0.0, v4817[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2314 * v1026);
                    let v1990 = (v2 + v1987).sqrt();
                    let v1991 = v2 + v1990;
                    let v1992 = (v1987 - v1026) / v1991;
                    let v1994 = v404 * v1993;
                    let v4829 = v2315 * v404;
                    let v1996 = (v2 + v1994).sqrt();
                    let v1997 = v2 + v1996;
                    let v1998 = v1994 / v1997;
                    let v1999 = v393 * v8;
                    let v2000 = v1999 * v618;
                    let v4837 = v4576 * v1992;
                    let v4841 = v4741 * v1998;
                    let v2003 = (v1863 * v1992) + (v1947 * v1998);
                    let v4846 = (v2925 * v1999) * v2003;
                    let v2005 = (v2000 * v2003) / v615;
                    let v4850 = v2923 * v2005;
                    let v4853 = (((Lanes([0.0, 0.0, v4846[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([0.0, 0.0, v4837[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((v4820 - (Lanes([0.0, 0.0, v3444[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) - ((v4820 * (v2270 / (v2704 * v1990))) * v1992)) / v1991) * v1863)) + ((Lanes([0.0, 0.0, v4841[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (((v4829 - ((v4829 * (v2270 / (v2704 * v1996))) * v1998)) / v1997) * v1947))) * v2000)) - (Lanes([0.0, 0.0, v4850[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) / v615;
                    v2025 = v2005;
                    v2402 = v4853;
                } else {
                    let v2006 = v697 - v1962;
                    let v2007 = v2006 * v107;
                    let v4797 = v2431 * v2006;
                    let v4799 = ((v3054 - (Lanes([0.0, 0.0, v2289[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) * v107) + (Lanes([0.0, 0.0, v4797[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]));
                    let v2008 = if v2007 < v699 { 1.0 } else { 0.0 };
                    let v2018: f64;
                    let v2403: Lanes<9>;
                    if v2008 != 0.0 {
                        let v2009 = v2007.exp();
                        let v4801 = v4799 * v2009;
                        v2018 = v2009;
                        v2403 = v4801;
                    } else {
                        let v2010 = v699.exp();
                        let v2013 = v2010 * (v2 + (v2007 - v699));
                        let v4800 = v4799 * v2010;
                        v2018 = v2013;
                        v2403 = v4800;
                    }
                    let v2014 = v25 * v8;
                    let v2015 = v2014 * v510;
                    let v2016 = v2015 * v624;
                    let v4806 = (((v2791 * v2014) * v624) + (v2928 * v2015)) * v1443;
                    let v2021 = (v2 + (v404 * v2018)).sqrt();
                    let v2022 = v2 + v2021;
                    let v2023 = (v2016 * v1443) / v2022;
                    let v4816 = (((Lanes([0.0, 0.0, v4806[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2314 * v2016)) - (((v2403 * v404) * (v2270 / (v2704 * v2021))) * v2023)) / v2022;
                    v2025 = v2023;
                    v2402 = v4816;
                }
                let v2026 = v2024 * v2025;
                let v4856 = (v2365 * v2025) + (v2402 * v2024);
                v2159 = v2026;
                v2168 = v1986;
                v2400 = v4856;
                v2401 = v4793;
            } else {
                v2159 = v0;
                v2168 = v1985;
                v2400 = v3977;
                v2401 = v2398;
            }
            let v2028 = if v2027 == v2 { 1.0 } else { 0.0 };
            let v2124: f64;
            let v2126: f64;
            let v2134: f64;
            let v2140: f64;
            let v2404: Lanes<5>;
            let v2405: Lanes<3>;
            let v2406: Lanes<5>;
            let v2407: Lanes<6>;
            if v2028 != 0.0 {
                let v2029 = -v26;
                let v4861 = v3349 * (v2029 * (v966.powf((v2029 - v2270))));
                let v2031 = (v966.powf(v2029)) - v152;
                let v2032 = if v951 < v0 { 1.0 } else { 0.0 };
                let v2040: f64;
                let v2408: Lanes<3>;
                if v2032 != 0.0 {
                    let v2033 = v951.exp();
                    let v2034 = v2 + v2033;
                    let v2035 = v2 / v2034;
                    let v4870 = (((v3327 * v2033) * v2035) * v2420) / v2034;
                    v2040 = v2035;
                    v2408 = v4870;
                } else {
                    let v2037 = (-v951).exp();
                    let v4863 = (v3327 * v2420) * v2037;
                    let v2038 = v2 + v2037;
                    let v2039 = v2037 / v2038;
                    let v4866 = (v4863 - (v4863 * v2039)) / v2038;
                    v2040 = v2039;
                    v2408 = v4866;
                }
                let v2042 = (v2031 * v2040) + v152;
                let v4874 = v4519 * v2042;
                let v4877 = (Lanes([v4874[0], 0.0, 0.0])) + (((v4861 * v2040) + (v2408 * v2031)) * v1833);
                let v4879 = v2431 * v1028;
                let v2045 = (v1028 * v107) / v410;
                let v4882 = v2295 * v2045;
                let v2046 = v393 / v1030;
                let v2047 = v2045 * v2046;
                let v2048 = v1864 * v1550;
                let v4892 = v4577 * v1550;
                let v4897 = ((((((v3448 * v107) + (Lanes([v4879[0], 0.0, 0.0]))) - (Lanes([v4882[0], 0.0, 0.0]))) / v410) * v2046) + ((((v3451 * v2046) * v2420) / v1030) * v2045)) * v2048;
                let v2050 = v1944 / v1935;
                let v4900 = v4724 * v2050;
                let v4903 = (v4734 - (Lanes([v4900[0], 0.0, 0.0]))) / v1935;
                let v2051 = v806 * v670;
                let v2053 = ((v1833 * v2042) + (v2048 * v2047)) + v2050;
                let v2054 = v2051 * v2053;
                let v4909 = (v2965 * v806) * v2053;
                let v4910 = (((Lanes([v4877[0], v4877[1], v4877[2], 0.0, 0.0])) + ((((Lanes([v4892[0], 0.0, 0.0, 0.0, 0.0])) + (v2371 * v1864)) * v2047) + (Lanes([v4897[0], v4897[1], v4897[2], 0.0, 0.0])))) + (Lanes([v4903[0], v4903[1], v4903[2], 0.0, 0.0]))) * v2051;
                let v4913 = (Lanes([0.0, 0.0, v4909[0], v4909[1], 0.0, 0.0])) + (Lanes([v4910[0], v4910[1], 0.0, v4910[2], v4910[3], v4910[4]]));
                let v2056 = v2 - v2055;
                let v2057 = v2056 * v1944;
                let v4914 = v4734 * v2056;
                let v4915 = v4734 * v2055;
                let v2059 = v1866 + (v2055 * v1944);
                let v4917 = v4585 + (Lanes([v4915[0], v4915[1], v4915[2], 0.0, 0.0]));
                let v2062 = (v2060 * v2059) + v1868;
                let v4919 = (v4917 * v2060) + v4593;
                let v2063 = v2 - v2060;
                let v2064 = v2063 * v2059;
                let v4920 = v4917 * v2063;
                v2124 = v2064;
                v2126 = v2057;
                v2134 = v2062;
                v2140 = v2054;
                v2404 = v4920;
                v2405 = v4914;
                v2406 = v4919;
                v2407 = v4913;
            } else {
                v2124 = v1866;
                v2126 = v1944;
                v2134 = v1868;
                v2140 = v0;
                v2404 = v4585;
                v2405 = v4734;
                v2406 = v4593;
                v2407 = v4857;
            }
            let v2066 = (v1 * v788) * v21;
            let v4922 = (v3122 * v1) * v21;
            let v2068 = (v1 * v1085) * v21;
            let v4924 = (v3561 * v1) * v21;
            let v2070 = (v1 * v1821) * v21;
            let v4926 = (v4496 * v1) * v21;
            let v2072 = (v1 * v1813) * v21;
            let v4928 = (v4481 * v1) * v21;
            let v2254: f64;
            let v2255: f64;
            let v2409: Lanes<4>;
            let v2410: Lanes<4>;
            if v479 != 0.0 {
                let v2075 = (v1 * (-v1528)) * v21;
                let v4935 = ((v4073 * v2420) * v1) * v21;
                v2254 = v2075;
                v2255 = v0;
                v2409 = v4935;
                v2410 = v4932;
            } else {
                let v2078 = (v1 * (-v1528)) * v21;
                let v4931 = ((v4073 * v2420) * v1) * v21;
                v2254 = v0;
                v2255 = v2078;
                v2409 = v4932;
                v2410 = v4931;
            }
            let v2080 = (v1 * v1561) * v21;
            let v4937 = (v4131 * v1) * v21;
            let v2084 = (v1 * (v2081 * v1776)) * v21;
            let v4940 = ((v2373 * v2081) * v1) * v21;
            let v4941 = v2973 * v1;
            let v2086 = (v1 * v675) / v1743;
            let v4942 = v2293 * v2086;
            let v2087 = v2086 * v21;
            let v4947 = (((Lanes([v4941[0], 0.0, v4941[1]])) - (Lanes([0.0, v4942[0], 0.0]))) / v1743) * v21;
            let v4948 = v2977 * v1;
            let v2089 = (v1 * v678) / v1736;
            let v4949 = v2294 * v2089;
            let v2090 = v2089 * v21;
            let v4954 = (((Lanes([v4948[0], 0.0, v4948[1]])) - (Lanes([0.0, v4949[0], 0.0]))) / v1736) * v21;
            let v2094 = (ddt(12393, (v2091 * v89))) * v21;
            let v4958 = ((v2271 * v2091) * v4956) * v21;
            let v2095 = v2 - v644;
            let v2096 = if v642 > v22 { 1.0 } else { 0.0 };
            let v2118: f64;
            let v2411: Lanes<1>;
            if v2096 != 0.0 {
                let v2098 = if v2097 == v0 { 1.0 } else { 0.0 };
                let v2119: f64;
                let v2412: Lanes<1>;
                if v2098 != 0.0 {
                    let v2100 = (v89 / v646) * v21;
                    let v4971 = (v2271 / v646) * v21;
                    v2119 = v2100;
                    v2412 = v4971;
                } else {
                    let v2102 = if (v2095.abs()) < v1481 { 1.0 } else { 0.0 };
                    let v2120: f64;
                    let v2413: Lanes<1>;
                    if v2102 != 0.0 {
                        let v2104 = (v15 / v646) * v21;
                        let v2106 = v2 + (v89 / v15);
                        let v2108 = v2104 * (v2106.ln());
                        let v4969 = ((v2271 / v15) * (v2270 / v2106)) * v2104;
                        v2120 = v2108;
                        v2413 = v4969;
                    } else {
                        let v2111 = (v15 / (v2095 * v646)) * v21;
                        let v2113 = v2 + (v89 / v15);
                        let v2116 = v2111 * ((v2113.powf(v2095)) - v2);
                        let v4965 = ((v2271 / v15) * (v2095 * (v2113.powf((v2095 - v2270))))) * v2111;
                        v2120 = v2116;
                        v2413 = v4965;
                    }
                    v2119 = v2120;
                    v2412 = v2413;
                }
                v2118 = v2119;
                v2411 = v2412;
            } else {
                let v2117 = v89 / v20;
                let v4959 = v2271 / v20;
                v2118 = v2117;
                v2411 = v4959;
            }
            let v2123 = (v2121 * v1830) * v21;
            let v4973 = (v4518 * v2121) * v21;
            let v2130 = (ddt(12461, (v1 * ((v1834 + v2124) + v2126)))) * v21;
            let v4980 = (((((Lanes([v4523[0], v4523[1], v4523[2], 0.0, 0.0])) + v2404) + (Lanes([v2405[0], v2405[1], v2405[2], 0.0, 0.0]))) * v1) * v4956) * v21;
            let v2133 = (ddt(12467, (v1 * v1859))) * v21;
            let v4983 = ((v4568 * v1) * v4956) * v21;
            let v2139 = (ddt(12477, (v1 * ((v1862 + v2134) + v1953)))) * v21;
            let v4990 = (((((Lanes([v4573[0], 0.0, v4573[1], v4573[2], v4573[3]])) + v2406) + (Lanes([v4751[0], 0.0, v4751[1], v4751[2], v4751[3]]))) * v1) * v4956) * v21;
            let v2143 = (ddt(12483, (v1 * v2140))) * v21;
            let v4993 = ((v2407 * v1) * v4956) * v21;
            let v2145 = v1 * v2144;
            let v2148 = (ddt(12491, (v2145 * v680))) * v21;
            let v4996 = ((v2981 * v2145) * v4956) * v21;
            let v2150 = v1 * v2149;
            let v2153 = (ddt(12499, (v2150 * v683))) * v21;
            let v4999 = ((v2985 * v2150) * v4956) * v21;
            let v2155 = (v1 * v1533) * v21;
            let v5001 = (v4085 * v1) * v21;
            let v2156 = v1 * v696;
            let v5003 = (v3012 * v1) * v1786;
            let v5004 = v2304 * v2156;
            let v2158 = (v2156 * v1786) * v21;
            let v5008 = ((Lanes([v5003[0], v5003[1], 0.0, v5003[2], v5003[3], v5003[4], v5003[5], v5003[6], v5003[7]])) + (Lanes([0.0, 0.0, v5004[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) * v21;
            let v2163 = (ddt(12519, (v1 * (v1928 + v2159)))) * v21;
            let v5012 = (((v4710 + v2400) * v1) * v4956) * v21;
            let v2167 = (v1 * ((v1531 + v1825) + v1530)) * v21;
            let v5016 = (((v4081 + v4505) + v4077) * v1) * v21;
            let v2172 = (ddt(12538, (v1 * (v1899 + v2168)))) * v21;
            let v5020 = (((v4652 + v2401) * v1) * v4956) * v21;
            let v2256: f64;
            let v2257: f64;
            let v2414: Lanes<3>;
            if v650 != 0.0 {
                let v2173 = v1 * v689;
                let v5023 = (v2993 * v1) * v1791;
                let v5024 = v2306 * v2173;
                let v2175 = (v2173 * v1791) * v21;
                let v5028 = ((Lanes([0.0, v5023[0], v5023[1]])) + (Lanes([v5024[0], 0.0, 0.0]))) * v21;
                v2256 = v2175;
                v2257 = v0;
                v2414 = v5028;
            } else {
                v2256 = v0;
                v2257 = v2176;
                v2414 = v5021;
            }
            let v2258: f64;
            let v2259: f64;
            let v2415: Lanes<3>;
            if v653 != 0.0 {
                let v2177 = v1 * v686;
                let v5031 = (v2989 * v1) * v1796;
                let v5032 = v2308 * v2177;
                let v2179 = (v2177 * v1796) * v21;
                let v5036 = ((Lanes([0.0, v5031[0], v5031[1]])) + (Lanes([v5032[0], 0.0, 0.0]))) * v21;
                v2258 = v2179;
                v2259 = v0;
                v2415 = v5036;
            } else {
                v2258 = v0;
                v2259 = v2180;
                v2415 = v5029;
            }
            let v2182 = (v1083 + v1082) / v1079;
            let v5040 = ((v3556 + v3557) - (v3546 * v2182)) / v1079;
            let v2184 = if v2183 > v0 { 1.0 } else { 0.0 };
            let v2187: f64;
            let v2416: Lanes<5>;
            if v2184 != 0.0 {
                let v2185 = v1776 / v2182;
                let v2186 = v2185.abs();
                let v5047 = ((v2373 - (v5040 * v2185)) / v2182) * ((v2704 * (if v2185 >= v3127 { 1.0 } else { 0.0 })) - v2270);
                v2187 = v2186;
                v2416 = v5047;
            } else {
                v2187 = v0;
                v2416 = v4118;
            }
            let v2188 = if v2182 > v0 { 1.0 } else { 0.0 };
            let v2195: f64;
            let v2417: Lanes<5>;
            if v2188 != 0.0 {
                let v2190 = (v2124 + v2134) / v2182;
                let v5058 = ((v2404 + v2406) - (v5040 * v2190)) / v2182;
                v2195 = v2190;
                v2417 = v5058;
            } else {
                let v2191 = v608 * v1550;
                let v5048 = v2919 * v1550;
                let v2192 = v2191 * v1079;
                let v5054 = (((Lanes([v5048[0], 0.0, 0.0, 0.0, 0.0])) + (v2371 * v608)) * v1079) + (v3546 * v2191);
                v2195 = v2192;
                v2417 = v5054;
            }
            let v2194 = if v2193 == v2 { 1.0 } else { 0.0 };
            let v2211: f64;
            let v2418: Lanes<5>;
            if v2194 != 0.0 {
                let v2196 = v2060 * v2195;
                let v5060 = v2417 * v2060;
                v2211 = v2196;
                v2418 = v5060;
            } else {
                let v2197 = if v2193 == v25 { 1.0 } else { 0.0 };
                let v2212: f64;
                let v2419: Lanes<5>;
                if v2197 != 0.0 {
                    let v2199 = v2198 * v2195;
                    let v5059 = v2417 * v2198;
                    v2212 = v2199;
                    v2419 = v5059;
                } else {
                    v2212 = v0;
                    v2419 = v4118;
                }
                v2211 = v2212;
                v2418 = v2419;
            }
            let v2201 = if (v1805 + v1819) < v0 { 1.0 } else { 0.0 };
            if v2201 != 0.0 {
            } else {
            }
            let v2204 = if ((v1237 + v1247) + v1267) < v0 { 1.0 } else { 0.0 };
            if v2204 != 0.0 {
            } else {
            }
            let v2205 = if v1531 < v0 { 1.0 } else { 0.0 };
            if v2205 != 0.0 {
            } else {
            }
            let v2206 = if v1530 < v0 { 1.0 } else { 0.0 };
            if v2206 != 0.0 {
            } else {
            }
            let v2207 = if v8 == v0 { 1.0 } else { 0.0 };
            if v2207 != 0.0 {
            } else {
            }
            let v2208 = if v1533 < v0 { 1.0 } else { 0.0 };
            if v2208 != 0.0 {
            } else {
            }
            let v2213 = ddt(12791, v2210);
            let v2214 = v2211 * v2213;
            let v5062 = v2418 * v2213;
            let v5063 = (v2282 * v4956) * v2211;
            let v5066 = (Lanes([v5062[0], v5062[1], v5062[2], v5062[3], v5062[4], 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v5063[0]]));
            let v2215 = v2187 * v2210;
            let v5067 = v2416 * v2210;
            let v5068 = v2282 * v2187;
            let v5071 = (Lanes([v5067[0], v5067[1], v5067[2], v5067[3], v5067[4], 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v5068[0]]));
            let v2260: f64;
            let v2261: f64;
            if v479 != 0.0 {
                v2260 = v2230;
                v2261 = v0;
            } else {
                v2260 = v0;
                v2261 = v2231;
            }
            let v2240: f64;
            let v2242: f64;
            let v2244: f64;
            let v2246: f64;
            let v2262: f64;
            let v2264: f64;
            let v2266: f64;
            let v2268: f64;
            if v650 != 0.0 {
                let v2241: f64;
                let v2243: f64;
                let v2263: f64;
                let v2265: f64;
                let v2267: f64;
                if v653 != 0.0 {
                    v2241 = v2232;
                    v2243 = v0;
                    v2263 = v2233;
                    v2265 = v2234;
                    v2267 = v0;
                } else {
                    v2241 = v0;
                    v2243 = v2235;
                    v2263 = v0;
                    v2265 = v0;
                    v2267 = v2236;
                }
                v2240 = v2241;
                v2242 = v2243;
                v2244 = v0;
                v2246 = v0;
                v2262 = v2263;
                v2264 = v2265;
                v2266 = v2267;
                v2268 = v0;
            } else {
                let v2245: f64;
                let v2247: f64;
                let v2269: f64;
                if v653 != 0.0 {
                    v2245 = v2237;
                    v2247 = v0;
                    v2269 = v2238;
                } else {
                    v2245 = v0;
                    v2247 = v2239;
                    v2269 = v0;
                }
                v2240 = v0;
                v2242 = v0;
                v2244 = v2245;
                v2246 = v2247;
                v2262 = v0;
                v2264 = v0;
                v2266 = v0;
                v2268 = v2269;
            }
            let v2252 = if ((((v2090 + v2148) + v2153) + v2155) + v2163) == v0 { 1.0 } else { 0.0 };
            if v2252 != 0.0 {
            } else {
            }
            let v2253 = if v21 != v2 { 1.0 } else { 0.0 };
            if v2253 != 0.0 {
            } else {
            }
            let v5072 = v4922[0];
            let v5073 = v4922[1];
            let v5074 = v4922[2];
            let v5075 = v4922[3];
            let v5076 = v4924[0];
            let v5077 = v4924[1];
            let v5078 = v4924[2];
            let v5079 = v4924[3];
            let v5080 = v4924[4];
            let v5081 = v4926[0];
            let v5082 = v4926[1];
            let v5083 = v4926[2];
            let v5084 = v4928[0];
            let v5085 = v4928[1];
            let v5086 = v4928[2];
            let v5087 = v4928[3];
            let v5088 = v4928[4];
            let v5089 = v2409[0];
            let v5090 = v2409[1];
            let v5091 = v2409[2];
            let v5092 = v2409[3];
            let v5093 = v2410[0];
            let v5094 = v2410[1];
            let v5095 = v2410[2];
            let v5096 = v2410[3];
            let v5097 = v4937[0];
            let v5098 = v4937[1];
            let v5099 = v4937[2];
            let v5100 = v4937[3];
            let v5101 = v4937[4];
            let v5102 = v4937[5];
            let v5103 = v4940[0];
            let v5104 = v4940[1];
            let v5105 = v4940[2];
            let v5106 = v4940[3];
            let v5107 = v4940[4];
            let v5108 = v4947[0];
            let v5109 = v4947[1];
            let v5110 = v4947[2];
            let v5111 = v4954[0];
            let v5112 = v4954[1];
            let v5113 = v4954[2];
            let v5114 = v2411[0];
            let v5115 = v4958[0];
            let v5116 = v4973[0];
            let v5117 = v4973[1];
            let v5118 = v4973[2];
            let v5119 = v4973[3];
            let v5120 = v4973[4];
            let v5121 = v4973[5];
            let v5122 = v4973[6];
            let v5123 = v4973[7];
            let v5124 = v4973[8];
            let v5125 = v4973[9];
            let v5126 = v4973[10];
            let v5127 = v4980[0];
            let v5128 = v4980[1];
            let v5129 = v4980[2];
            let v5130 = v4980[3];
            let v5131 = v4980[4];
            let v5132 = v4983[0];
            let v5133 = v4983[1];
            let v5134 = v4983[2];
            let v5135 = v4990[0];
            let v5136 = v4990[1];
            let v5137 = v4990[2];
            let v5138 = v4990[3];
            let v5139 = v4990[4];
            let v5140 = v4993[0];
            let v5141 = v4993[1];
            let v5142 = v4993[2];
            let v5143 = v4993[3];
            let v5144 = v4993[4];
            let v5145 = v4993[5];
            let v5146 = v4996[0];
            let v5147 = v4996[1];
            let v5148 = v4999[0];
            let v5149 = v4999[1];
            let v5150 = v5001[0];
            let v5151 = v5001[1];
            let v5152 = v5001[2];
            let v5153 = v5001[3];
            let v5154 = v5001[4];
            let v5155 = v5001[5];
            let v5156 = v5001[6];
            let v5157 = v5001[7];
            let v5158 = v5001[8];
            let v5159 = v5008[0];
            let v5160 = v5008[1];
            let v5161 = v5008[2];
            let v5162 = v5008[3];
            let v5163 = v5008[4];
            let v5164 = v5008[5];
            let v5165 = v5008[6];
            let v5166 = v5008[7];
            let v5167 = v5008[8];
            let v5168 = v5012[0];
            let v5169 = v5012[1];
            let v5170 = v5012[2];
            let v5171 = v5012[3];
            let v5172 = v5012[4];
            let v5173 = v5012[5];
            let v5174 = v5012[6];
            let v5175 = v5012[7];
            let v5176 = v5012[8];
            let v5177 = v5016[0];
            let v5178 = v5016[1];
            let v5179 = v5016[2];
            let v5180 = v5016[3];
            let v5181 = v5016[4];
            let v5182 = v5016[5];
            let v5183 = v5020[0];
            let v5184 = v5020[1];
            let v5185 = v5020[2];
            let v5186 = v5020[3];
            let v5187 = v5020[4];
            let v5188 = v5020[5];
            let v5189 = v2414[0];
            let v5190 = v2414[1];
            let v5191 = v2414[2];
            let v5192 = v2415[0];
            let v5193 = v2415[1];
            let v5194 = v2415[2];
            let v5195 = v2282[0];
            let v5196 = v5066[0];
            let v5197 = v5066[1];
            let v5198 = v5066[2];
            let v5199 = v5066[3];
            let v5200 = v5066[4];
            let v5201 = v5066[5];
            let v5202 = v5071[0];
            let v5203 = v5071[1];
            let v5204 = v5071[2];
            let v5205 = v5071[3];
            let v5206 = v5071[4];
            let v5207 = v5071[5];
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(8),
            multiplicity * (v2066),
            [3, 6, 7, 8],
            [v5072, v5073, v5074, v5075],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(4),
            multiplicity * (v2068),
            [3, 4, 6, 7, 8],
            [v5076, v5077, v5078, v5079, v5080],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(4),
            multiplicity * (v2070),
            [3, 4, 5],
            [v5081, v5082, v5083],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(4),
            multiplicity * (v2072),
            [3, 4, 6, 7, 8],
            [v5084, v5085, v5086, v5087, v5088],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(7),
            multiplicity * (v2254),
            [3, 5, 6, 7],
            [v5089, v5090, v5091, v5092],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(8),
            multiplicity * (v2255),
            [3, 5, 6, 7],
            [v5093, v5094, v5095, v5096],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(6),
            multiplicity * (v2080),
            [3, 4, 5, 6, 7, 8],
            [v5097, v5098, v5099, v5100, v5101, v5102],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(8),
            multiplicity * (v2084),
            [3, 4, 6, 7, 8],
            [v5103, v5104, v5105, v5106, v5107],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(2),
            Some(4),
            multiplicity * (v2087),
            [2, 3, 4],
            [v5108, v5109, v5110],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(5),
            multiplicity * (v2090),
            [1, 3, 5],
            [v5111, v5112, v5113],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (v2118),
            [3],
            [v5114],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (v2094),
            [3],
            [v5115],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<11, 0>(
            Some(3),
            None,
            multiplicity * (v2123),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            [v5116, v5117, v5118, v5119, v5120, v5121, v5122, v5123, v5124, v5125, v5126],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(4),
            multiplicity * (v2130),
            [3, 4, 6, 7, 8],
            [v5127, v5128, v5129, v5130, v5131],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(4),
            multiplicity * (v2133),
            [3, 4, 5],
            [v5132, v5133, v5134],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(8),
            multiplicity * (v2139),
            [3, 4, 6, 7, 8],
            [v5135, v5136, v5137, v5138, v5139],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(6),
            multiplicity * (v2143),
            [3, 4, 5, 6, 7, 8],
            [v5140, v5141, v5142, v5143, v5144, v5145],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(2),
            multiplicity * (v2148),
            [1, 2],
            [v5146, v5147],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(0),
            multiplicity * (v2153),
            [0, 1],
            [v5148, v5149],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(9),
            multiplicity * (v2155),
            [0, 1, 3, 5, 6, 7, 8, 9, 10],
            [v5150, v5151, v5152, v5153, v5154, v5155, v5156, v5157, v5158],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(0),
            Some(9),
            multiplicity * (v2158),
            [0, 1, 3, 5, 6, 7, 8, 9, 10],
            [v5159, v5160, v5161, v5162, v5163, v5164, v5165, v5166, v5167],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(9),
            multiplicity * (v2163),
            [0, 1, 3, 5, 6, 7, 8, 9, 10],
            [v5168, v5169, v5170, v5171, v5172, v5173, v5174, v5175, v5176],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(10),
            multiplicity * (v2167),
            [3, 5, 6, 7, 8, 10],
            [v5177, v5178, v5179, v5180, v5181, v5182],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(10),
            multiplicity * (v2172),
            [3, 5, 6, 7, 8, 10],
            [v5183, v5184, v5185, v5186, v5187, v5188],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(9),
            Some(10),
            multiplicity * (v2256),
            [3, 9, 10],
            [v5189, v5190, v5191],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(9), Some(10), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            v2257,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(10),
            Some(7),
            multiplicity * (v2258),
            [3, 7, 10],
            [v5192, v5193, v5194],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(10), Some(7), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            v2259,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(11),
            None,
            multiplicity * (v2209),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(11),
            None,
            multiplicity * (v2210),
            [11],
            [v5195],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(4),
            multiplicity * (v2214),
            [3, 4, 6, 7, 8, 11],
            [v5196, v5197, v5198, v5199, v5200, v5201],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            Some(6),
            multiplicity * (v2215),
            [3, 4, 6, 7, 8, 11],
            [v5202, v5203, v5204, v5205, v5206, v5207],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(8),
            Some(4),
            multiplicity * (v2210),
            [11],
            [v5195],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(6),
            multiplicity * (v2216),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(4),
            multiplicity * (v2217),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(2),
            Some(4),
            multiplicity * (v2218),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(5),
            multiplicity * (v2219),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(6),
            multiplicity * (v2220),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(4),
            multiplicity * (v2221),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(4),
            multiplicity * (v2222),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(4),
            multiplicity * (v2223),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(10),
            multiplicity * (v2224),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(10),
            multiplicity * (v2225),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(10),
            multiplicity * (v2226),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(10),
            multiplicity * (v2227),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(9),
            multiplicity * (v2228),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(9),
            multiplicity * (v2229),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(6),
            multiplicity * (v2260),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(6),
            multiplicity * (v2261),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(9),
            multiplicity * (v2240),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(10),
            multiplicity * (v2262),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(10),
            Some(7),
            multiplicity * (v2264),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(9),
            multiplicity * (v2242),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(7),
            multiplicity * (v2266),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(10),
            multiplicity * (v2244),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(10),
            Some(7),
            multiplicity * (v2268),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(7),
            multiplicity * (v2246),
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

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
            let v2280 = 1e0f64;
            let v2281 = 1e0f64;
            let v2282 = 1e0f64;
            let v2283 = 1e0f64;
            let v2284 = 1e0f64;
            let v2285 = 1e0f64;
            let v2286 = 1e0f64;
            let v2287 = 1e0f64;
            let v2288 = 1e0f64;
            let v2289 = 1e0f64;
            let v2290 = 1e0f64;
            let v2291 = 1e0f64;
            let v2292 = 1e0f64;
            let v2430 = -1e0f64;
            let v2667 = 0e0f64;
            let v2714 = 2e0f64;
            let v2828 = -1.5e0f64;
            let v2869 = -1.5e0f64;
            let v3112 = Lanes([0e0f64; 3]);
            let v3137 = 0e0f64;
            let v3158 = Lanes([0e0f64; 4]);
            let v3756 = Lanes([0e0f64; 3]);
            let v3987 = Lanes([0e0f64; 9]);
            let v4046 = Lanes([0e0f64; 3]);
            let v4128 = Lanes([0e0f64; 5]);
            let v4867 = Lanes([0e0f64; 6]);
            let v4942 = Lanes([0e0f64; 4]);
            let v4966 = ddt_scale();
            let v5040 = Lanes([0e0f64; 3]);
            let v5048 = Lanes([0e0f64; 3]);
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
            let v2293: f64;
            if v90 != 0.0 {
                let v91 = v2 - v89;
                let v93 = -(v91.ln());
                let v2434 = ((v2281 * v2430) * (v2280 / v91)) * v2430;
                v94 = v93;
                v2293 = v2434;
            } else {
                v94 = v89;
                v2293 = v2281;
            }
            let v96 = if v94 < v95 { 1.0 } else { 0.0 };
            let v101: f64;
            let v2294: f64;
            if v96 != 0.0 {
                v101 = v94;
                v2294 = v2293;
            } else {
                let v98 = v2 + (v94 - v95);
                let v2436 = v2293 * (v2280 / v98);
                let v100 = v95 + (v98.ln());
                v101 = v100;
                v2294 = v2436;
            }
            let v102 = v15 + v101;
            let v103 = v102 / v12;
            let v2437 = v2294 / v12;
            let v105 = v104 * v102;
            let v2438 = v2294 * v104;
            let v107 = v2 / v105;
            let v2441 = ((v2438 * v107) * v2430) / v105;
            let v109 = v107 - (v2 / (v104 * v12));
            let v110 = v102 - v12;
            let v111 = v103.ln();
            let v2443 = v2437 * (v2280 / v103);
            let v113 = v31 * v102;
            let v115 = v102 + v34;
            let v116 = (v113 * v102) / v115;
            let v117 = v112 - v116;
            let v2451 = (((((v2294 * v31) * v102) + (v2294 * v113)) - (v2294 * v116)) / v115) * v2430;
            let v119 = (v117 - v38) / v40;
            let v2452 = v2451 / v40;
            let v120 = if v117 < v38 { 1.0 } else { 0.0 };
            let v537: f64;
            let v2295: f64;
            if v120 != 0.0 {
                let v121 = v119.exp();
                let v122 = v2 + v121;
                let v2462 = ((v2452 * v121) * (v2280 / v122)) * v40;
                let v125 = v38 + (v40 * (v122.ln()));
                v537 = v125;
                v2295 = v2462;
            } else {
                let v127 = (-v119).exp();
                let v128 = v2 + v127;
                let v131 = v117 + (v40 * (v128.ln()));
                let v2458 = v2451 + ((((v2452 * v2430) * v127) * (v2280 / v128)) * v40);
                v537 = v131;
                v2295 = v2458;
            }
            let v133 = v63 * v102;
            let v135 = v102 + v66;
            let v136 = (v133 * v102) / v135;
            let v137 = v132 - v136;
            let v2470 = (((((v2294 * v63) * v102) + (v2294 * v133)) - (v2294 * v136)) / v135) * v2430;
            let v139 = (v137 - v38) / v40;
            let v2471 = v2470 / v40;
            let v140 = if v137 < v38 { 1.0 } else { 0.0 };
            let v561: f64;
            let v2296: f64;
            if v140 != 0.0 {
                let v141 = v139.exp();
                let v142 = v2 + v141;
                let v2481 = ((v2471 * v141) * (v2280 / v142)) * v40;
                let v145 = v38 + (v40 * (v142.ln()));
                v561 = v145;
                v2296 = v2481;
            } else {
                let v147 = (-v139).exp();
                let v148 = v2 + v147;
                let v151 = v137 + (v40 * (v148.ln()));
                let v2477 = v2470 + ((((v2471 * v2430) * v147) * (v2280 / v148)) * v40);
                v561 = v151;
                v2296 = v2477;
            }
            let v154 = v153 * v105;
            let v158 = v2 - v103;
            let v2488 = v2437 * v2430;
            let v161 = ((v154 * v111) + (v55 * v103)) + (v158 * v159);
            let v2490 = ((((v2438 * v153) * v111) + (v2443 * v154)) + (v2437 * v55)) + (v2488 * v159);
            let v163 = (v38 - v161) / v105;
            let v2494 = ((v2490 * v2430) - (v2438 * v163)) / v105;
            let v164 = if v38 < v161 { 1.0 } else { 0.0 };
            let v282: f64;
            let v2297: f64;
            if v164 != 0.0 {
                let v165 = v163.exp();
                let v166 = v2 + v165;
                let v167 = v166.ln();
                let v169 = v161 + (v105 * v167);
                let v2508 = v2490 + ((v2438 * v167) + (((v2494 * v165) * (v2280 / v166)) * v105));
                v282 = v169;
                v2297 = v2508;
            } else {
                let v171 = (-v163).exp();
                let v172 = v2 + v171;
                let v173 = v172.ln();
                let v2501 = (v2438 * v173) + ((((v2494 * v2430) * v171) * (v2280 / v172)) * v105);
                let v175 = v38 + (v105 * v173);
                v282 = v175;
                v2297 = v2501;
            }
            let v177 = v176 * v105;
            let v183 = v158 * v182;
            let v2515 = v2488 * v182;
            let v184 = ((v177 * v111) + (v179 * v103)) + v183;
            let v2516 = ((((v2438 * v176) * v111) + (v2443 * v177)) + (v2437 * v179)) + v2515;
            let v186 = (v38 - v184) / v105;
            let v2520 = ((v2516 * v2430) - (v2438 * v186)) / v105;
            let v187 = if v38 < v184 { 1.0 } else { 0.0 };
            let v735: f64;
            let v2298: f64;
            if v187 != 0.0 {
                let v188 = v186.exp();
                let v189 = v2 + v188;
                let v190 = v189.ln();
                let v192 = v184 + (v105 * v190);
                let v2534 = v2516 + ((v2438 * v190) + (((v2520 * v188) * (v2280 / v189)) * v105));
                v735 = v192;
                v2298 = v2534;
            } else {
                let v194 = (-v186).exp();
                let v195 = v2 + v194;
                let v196 = v195.ln();
                let v2527 = (v2438 * v196) + ((((v2520 * v2430) * v194) * (v2280 / v195)) * v105);
                let v198 = v38 + (v105 * v196);
                v735 = v198;
                v2298 = v2527;
            }
            let v200 = v199 * v105;
            let v205 = ((v200 * v111) + (v202 * v103)) + v183;
            let v2541 = ((((v2438 * v199) * v111) + (v2443 * v200)) + (v2437 * v202)) + v2515;
            let v207 = (v38 - v205) / v105;
            let v2545 = ((v2541 * v2430) - (v2438 * v207)) / v105;
            let v208 = if v38 < v205 { 1.0 } else { 0.0 };
            let v1962: f64;
            let v2299: f64;
            if v208 != 0.0 {
                let v209 = v207.exp();
                let v210 = v2 + v209;
                let v211 = v210.ln();
                let v213 = v205 + (v105 * v211);
                let v2559 = v2541 + ((v2438 * v211) + (((v2545 * v209) * (v2280 / v210)) * v105));
                v1962 = v213;
                v2299 = v2559;
            } else {
                let v215 = (-v207).exp();
                let v216 = v2 + v215;
                let v217 = v216.ln();
                let v2552 = (v2438 * v217) + ((((v2545 * v2430) * v215) * (v2280 / v216)) * v105);
                let v219 = v38 + (v105 * v217);
                v1962 = v219;
                v2299 = v2552;
            }
            let v221 = v220 * v105;
            let v223 = v57 * v103;
            let v2564 = v2437 * v57;
            let v225 = ((v221 * v111) + v223) + v183;
            let v2566 = ((((v2438 * v220) * v111) + (v2443 * v221)) + v2564) + v2515;
            let v227 = (v38 - v225) / v105;
            let v2570 = ((v2566 * v2430) - (v2438 * v227)) / v105;
            let v228 = if v38 < v225 { 1.0 } else { 0.0 };
            let v294: f64;
            let v2300: f64;
            if v228 != 0.0 {
                let v229 = v227.exp();
                let v230 = v2 + v229;
                let v231 = v230.ln();
                let v233 = v225 + (v105 * v231);
                let v2584 = v2566 + ((v2438 * v231) + (((v2570 * v229) * (v2280 / v230)) * v105));
                v294 = v233;
                v2300 = v2584;
            } else {
                let v235 = (-v227).exp();
                let v236 = v2 + v235;
                let v237 = v236.ln();
                let v2577 = (v2438 * v237) + ((((v2570 * v2430) * v235) * (v2280 / v236)) * v105);
                let v239 = v38 + (v105 * v237);
                v294 = v239;
                v2300 = v2577;
            }
            let v241 = v240 * v105;
            let v244 = ((v241 * v111) + v223) + v183;
            let v2590 = ((((v2438 * v240) * v111) + (v2443 * v241)) + v2564) + v2515;
            let v246 = (v38 - v244) / v105;
            let v2594 = ((v2590 * v2430) - (v2438 * v246)) / v105;
            let v247 = if v38 < v244 { 1.0 } else { 0.0 };
            let v284: f64;
            let v2301: f64;
            if v247 != 0.0 {
                let v248 = v246.exp();
                let v249 = v2 + v248;
                let v250 = v249.ln();
                let v252 = v244 + (v105 * v250);
                let v2608 = v2590 + ((v2438 * v250) + (((v2594 * v248) * (v2280 / v249)) * v105));
                v284 = v252;
                v2301 = v2608;
            } else {
                let v254 = (-v246).exp();
                let v255 = v2 + v254;
                let v256 = v255.ln();
                let v2601 = (v2438 * v256) + ((((v2594 * v2430) * v254) * (v2280 / v255)) * v105);
                let v258 = v38 + (v105 * v256);
                v284 = v258;
                v2301 = v2601;
            }
            let v260 = v259 * v105;
            let v267 = ((v260 * v111) + (v262 * v103)) + (v158 * v265);
            let v2616 = ((((v2438 * v259) * v111) + (v2443 * v260)) + (v2437 * v262)) + (v2488 * v265);
            let v269 = (v38 - v267) / v105;
            let v2620 = ((v2616 * v2430) - (v2438 * v269)) / v105;
            let v270 = if v38 < v267 { 1.0 } else { 0.0 };
            let v1139: f64;
            let v2302: f64;
            if v270 != 0.0 {
                let v271 = v269.exp();
                let v272 = v2 + v271;
                let v273 = v272.ln();
                let v275 = v267 + (v105 * v273);
                let v2634 = v2616 + ((v2438 * v273) + (((v2620 * v271) * (v2280 / v272)) * v105));
                v1139 = v275;
                v2302 = v2634;
            } else {
                let v277 = (-v269).exp();
                let v278 = v2 + v277;
                let v279 = v278.ln();
                let v2627 = (v2438 * v279) + ((((v2620 * v2430) * v277) * (v2280 / v278)) * v105);
                let v281 = v38 + (v105 * v279);
                v1139 = v281;
                v2302 = v2627;
            }
            let v283 = v2 / v282;
            let v2637 = ((v2297 * v283) * v2430) / v282;
            let v285 = v2 / v284;
            let v2640 = ((v2301 * v285) * v2430) / v284;
            let v286 = v55 * v283;
            let v287 = v286.powf(v26);
            let v2645 = (v2637 * v55) * (v26 * (v286.powf((v26 - v2280))));
            let v288 = v57 * v285;
            let v289 = v288.powf(v58);
            let v2647 = v58 - v2280;
            let v2650 = (v2640 * v57) * (v58 * (v288.powf(v2647)));
            let v291 = v290 * v287;
            let v2651 = v2645 * v290;
            let v293 = v2 - v292;
            let v295 = v57 / v294;
            let v2658 = ((((v2300 * v295) * v2430) / v294) * (v58 * (v295.powf(v2647)))) * v293;
            let v298 = (v293 * (v295.powf(v58))) + v292;
            let v299 = v2 / v298;
            let v2661 = ((v2658 * v299) * v2430) / v298;
            let v301 = v300 * v298;
            let v2662 = v2658 * v300;
            let v302 = v292 * v299;
            let v2663 = v2661 * v292;
            let v306 = (v111 * v304).exp();
            let v307 = v303 * v306;
            let v2666 = ((v2443 * v304) * v306) * v303;
            let v308 = if v307 < v22 { 1.0 } else { 0.0 };
            let v1743: f64;
            let v2303: f64;
            if v308 != 0.0 {
                v1743 = v22;
                v2303 = v2667;
            } else {
                v1743 = v307;
                v2303 = v2666;
            }
            let v312 = v310 - v311;
            let v314 = (v111 * v312).exp();
            let v315 = v309 * v314;
            let v2670 = ((v2443 * v312) * v314) * v309;
            let v319 = (v111 * v317).exp();
            let v320 = v316 * v319;
            let v2673 = ((v2443 * v317) * v319) * v316;
            let v321 = if v320 < v22 { 1.0 } else { 0.0 };
            let v1736: f64;
            let v2304: f64;
            if v321 != 0.0 {
                v1736 = v22;
                v2304 = v2667;
            } else {
                v1736 = v320;
                v2304 = v2673;
            }
            let v325 = (v111 * v323).exp();
            let v326 = v322 * v325;
            let v2676 = ((v2443 * v323) * v325) * v322;
            let v330 = (v111 * v328).exp();
            let v2678 = (v2443 * v328) * v330;
            let v331 = v327 * v330;
            let v2679 = v2678 * v327;
            let v333 = v332 * v330;
            let v2680 = v2678 * v332;
            let v337 = (v111 * v335).exp();
            let v338 = v334 * v337;
            let v2683 = ((v2443 * v335) * v337) * v334;
            let v340 = if v339 != v0 { 1.0 } else { 0.0 };
            let v410: f64;
            let v2305: f64;
            if v340 != 0.0 {
                let v344 = v341 * (v2 + (v110 * v339));
                let v2685 = (v2294 * v339) * v341;
                let v346 = (v344 - v2) / v24;
                let v2686 = v2685 / v24;
                let v347 = if v344 < v2 { 1.0 } else { 0.0 };
                let v359: f64;
                let v2306: f64;
                if v347 != 0.0 {
                    let v348 = v346.exp();
                    let v349 = v2 + v348;
                    let v2696 = ((v2686 * v348) * (v2280 / v349)) * v24;
                    let v352 = v2 + (v24 * (v349.ln()));
                    v359 = v352;
                    v2306 = v2696;
                } else {
                    let v354 = (-v346).exp();
                    let v355 = v2 + v354;
                    let v358 = v344 + (v24 * (v355.ln()));
                    let v2692 = v2685 + ((((v2686 * v2430) * v354) * (v2280 / v355)) * v24);
                    v359 = v358;
                    v2306 = v2692;
                }
                let v361 = v359 - v360;
                v410 = v361;
                v2305 = v2306;
            } else {
                v410 = v341;
                v2305 = v2667;
            }
            let v363 = if v362 != v0 { 1.0 } else { 0.0 };
            let v1034: f64;
            let v2307: f64;
            if v363 != 0.0 {
                let v367 = v364 * (v2 + (v110 * v362));
                let v2698 = (v2294 * v362) * v364;
                let v369 = (v367 - v2) / v24;
                let v2699 = v2698 / v24;
                let v370 = if v367 < v2 { 1.0 } else { 0.0 };
                let v382: f64;
                let v2308: f64;
                if v370 != 0.0 {
                    let v371 = v369.exp();
                    let v372 = v2 + v371;
                    let v2709 = ((v2699 * v371) * (v2280 / v372)) * v24;
                    let v375 = v2 + (v24 * (v372.ln()));
                    v382 = v375;
                    v2308 = v2709;
                } else {
                    let v377 = (-v369).exp();
                    let v378 = v2 + v377;
                    let v381 = v367 + (v24 * (v378.ln()));
                    let v2705 = v2698 + ((((v2699 * v2430) * v377) * (v2280 / v378)) * v24);
                    v382 = v381;
                    v2308 = v2705;
                }
                let v384 = v382 - v383;
                v1034 = v384;
                v2307 = v2308;
            } else {
                v1034 = v364;
                v2307 = v2667;
            }
            let v389 = v385 * (v2 + (v386 * v110));
            let v2711 = (v2294 * v386) * v385;
            let v391 = v389 * v389;
            let v2712 = v2711 * v389;
            let v2713 = v2712 + v2712;
            let v392 = if v389 < v0 { 1.0 } else { 0.0 };
            let v1579: f64;
            let v2309: f64;
            if v392 != 0.0 {
                let v396 = (v391 + v390).sqrt();
                let v397 = v396 - v389;
                let v398 = v394 / v397;
                let v2726 = ((((v2713 * (v2280 / (v2714 * v396))) - v2711) * v398) * v2430) / v397;
                v1579 = v398;
                v2309 = v2726;
            } else {
                let v400 = (v391 + v390).sqrt();
                let v402 = v393 * (v400 + v389);
                let v2719 = ((v2713 * (v2280 / (v2714 * v400))) + v2711) * v393;
                v1579 = v402;
                v2309 = v2719;
            }
            let v408 = ((v404 - v310) - v311) + v407;
            let v411 = (v111 * v408) / v410;
            let v412 = v411.exp();
            let v413 = v403 * v412;
            let v414 = -v159;
            let v416 = (v414 * v109) / v410;
            let v417 = v416.exp();
            let v418 = v413 * v417;
            let v2740 = ((((((v2443 * v408) - (v2305 * v411)) / v410) * v412) * v403) * v417) + (((((v2441 * v414) - (v2305 * v416)) / v410) * v417) * v413);
            let v420 = v2 - v310;
            let v422 = (v111 * v420).exp();
            let v423 = v419 * v422;
            let v2743 = ((v2443 * v420) * v422) * v419;
            let v426 = v2 - v425;
            let v428 = (v111 * v426).exp();
            let v429 = v424 * v428;
            let v2746 = ((v2443 * v426) * v428) * v424;
            let v434 = v431 - (v25 * v432);
            let v436 = (v111 * v434).exp();
            let v437 = v430 * v436;
            let v439 = -v438;
            let v440 = v439 * v109;
            let v2750 = v2441 * v439;
            let v442 = (v440 / v432).exp();
            let v443 = v437 * v442;
            let v2755 = ((((v2443 * v434) * v436) * v430) * v442) + (((v2750 / v432) * v442) * v437);
            let v447 = v431 - (v25 * v445);
            let v449 = (v111 * v447).exp();
            let v450 = v444 * v449;
            let v451 = -v182;
            let v454 = ((v451 * v109) / v445).exp();
            let v455 = v450 * v454;
            let v2764 = ((((v2443 * v447) * v449) * v444) * v454) + ((((v2441 * v451) / v445) * v454) * v450);
            let v458 = (v404 - v304) + v407;
            let v459 = v111 * v458;
            let v2765 = v2443 * v458;
            let v462 = (v459 / v460).exp();
            let v463 = v456 * v462;
            let v465 = -v464;
            let v466 = v465 * v109;
            let v2769 = v2441 * v465;
            let v468 = (v466 / v460).exp();
            let v469 = v463 * v468;
            let v2774 = ((((v2765 / v460) * v462) * v456) * v468) + (((v2769 / v460) * v468) * v463);
            let v473 = (v459 / v471).exp();
            let v474 = v470 * v473;
            let v476 = (v466 / v471).exp();
            let v477 = v474 * v476;
            let v2782 = ((((v2765 / v471) * v473) * v470) * v476) + (((v2769 / v471) * v476) * v474);
            let v479 = if v478 == v2 { 1.0 } else { 0.0 };
            let v1161: f64;
            let v1174: f64;
            let v1216: f64;
            let v2310: f64;
            let v2311: f64;
            let v2312: f64;
            if v479 != 0.0 {
                let v482 = -v481;
                let v485 = ((v482 * v109) / v460).exp();
                let v486 = v480 * v485;
                let v2786 = (((v2441 * v482) / v460) * v485) * v480;
                let v489 = -v488;
                let v491 = (v489 * v109).exp();
                let v492 = v487 * v491;
                let v2789 = ((v2441 * v489) * v491) * v487;
                let v495 = -v494;
                let v498 = ((v495 * v109) / v471).exp();
                let v499 = v493 * v498;
                let v2793 = (((v2441 * v495) / v471) * v498) * v493;
                v1161 = v486;
                v1174 = v492;
                v1216 = v499;
                v2310 = v2786;
                v2311 = v2789;
                v2312 = v2793;
            } else {
                v1161 = v0;
                v1174 = v0;
                v1216 = v0;
                v2310 = v2667;
                v2311 = v2667;
                v2312 = v2667;
            }
            let v502 = (v404 - v425) + v407;
            let v504 = (v111 * v502).exp();
            let v505 = v500 * v504;
            let v507 = -v506;
            let v509 = (v507 * v109).exp();
            let v510 = v505 * v509;
            let v2801 = ((((v2443 * v502) * v504) * v500) * v509) + (((v2441 * v507) * v509) * v505);
            let v514 = v431 - (v25 * v512);
            let v516 = (v111 * v514).exp();
            let v517 = v511 * v516;
            let v519 = (v440 / v512).exp();
            let v520 = v517 * v519;
            let v2809 = ((((v2443 * v514) * v516) * v511) * v519) + (((v2750 / v512) * v519) * v517);
            let v523 = v404 / v522;
            let v525 = (v111 * v523).exp();
            let v526 = v521 * v525;
            let v528 = (v440 / v522).exp();
            let v529 = v526 * v528;
            let v2817 = ((((v2443 * v523) * v525) * v521) * v528) + (((v2750 / v522) * v528) * v526);
            let v531 = v103.sqrt();
            let v532 = v530 * v531;
            let v535 = (v533 * v110).exp();
            let v536 = v532 * v535;
            let v2826 = (((v2437 * (v2280 / (v2714 * v531))) * v530) * v535) + (((v2294 * v533) * v535) * v532);
            let v538 = v537 * v54;
            let v540 = v538.powf(v539);
            let v2831 = (v2295 * v54) * (v539 * (v538.powf(v2828)));
            let v541 = v2 / v287;
            let v2834 = ((v2645 * v541) * v2430) / v287;
            let v543 = v542 * v537;
            let v544 = v543 * v537;
            let v545 = v544 * v540;
            let v547 = (v545 * v541) * v55;
            let v550 = ((v547 * v283) * v54) * v54;
            let v2850 = (((((((((((v2295 * v542) * v537) + (v2295 * v543)) * v540) + (v2831 * v544)) * v541) + (v2834 * v545)) * v55) * v283) + (v2637 * v547)) * v54) * v54;
            let v552 = v551 * v540;
            let v553 = v552 * v282;
            let v556 = ((v553 * v282) * v56) * v56;
            let v557 = v556 * v287;
            let v559 = (v542 - v550).exp();
            let v560 = v557 * v559;
            let v2867 = ((((((((((v2831 * v551) * v282) + (v2297 * v552)) * v282) + (v2297 * v553)) * v56) * v56) * v287) + (v2645 * v556)) * v559) + (((v2850 * v2430) * v559) * v557);
            let v562 = v561 * v84;
            let v564 = v562.powf(v563);
            let v2872 = (v2296 * v84) * (v563 * (v562.powf(v2869)));
            let v565 = v2 / v289;
            let v567 = v566 * v561;
            let v568 = v567 * v561;
            let v569 = v568 * v564;
            let v571 = (v569 * v565) * v57;
            let v574 = ((v571 * v285) * v84) * v84;
            let v2891 = (((((((((((v2296 * v566) * v561) + (v2296 * v567)) * v564) + (v2872 * v568)) * v565) + ((((v2650 * v565) * v2430) / v289) * v569)) * v57) * v285) + (v2640 * v571)) * v84) * v84;
            let v576 = v575 * v564;
            let v577 = v576 * v284;
            let v580 = ((v577 * v284) * v85) * v85;
            let v581 = v580 * v289;
            let v583 = (v566 - v574).exp();
            let v584 = v581 * v583;
            let v2908 = ((((((((((v2872 * v575) * v284) + (v2301 * v576)) * v284) + (v2301 * v577)) * v85) * v85) * v289) + (v2650 * v580)) * v583) + (((v2891 * v2430) * v583) * v581);
            let v586 = (v111 * v311).exp();
            let v2910 = (v2443 * v311) * v586;
            let v588 = v587 * v586;
            let v589 = v588 * v299;
            let v2914 = ((v2910 * v587) * v299) + (v2661 * v588);
            let v591 = v590 * v586;
            let v592 = v591 * v541;
            let v2918 = ((v2910 * v590) * v541) + (v2834 * v591);
            let v594 = v310 - v25;
            let v596 = (v111 * v594).exp();
            let v597 = v593 * v596;
            let v599 = -v598;
            let v601 = (v599 * v109).exp();
            let v602 = v597 * v601;
            let v2926 = ((((v2443 * v594) * v596) * v593) * v601) + (((v2441 * v599) * v601) * v597);
            let v605 = (v311 + v310) - v2;
            let v607 = (v111 * v605).exp();
            let v608 = v603 * v607;
            let v2929 = ((v2443 * v605) * v607) * v603;
            let v610 = v335 - v2;
            let v612 = (v111 * v610).exp();
            let v613 = v609 * v612;
            let v2932 = ((v2443 * v610) * v612) * v609;
            let v615 = v608 + v613;
            let v2933 = v2929 + v2932;
            let v617 = v603 + v609;
            let v618 = (v614 * v615) / v617;
            let v2935 = (v2933 * v614) / v617;
            let v621 = v620 - v2;
            let v623 = (v111 * v621).exp();
            let v624 = v619 * v623;
            let v2938 = ((v2443 * v621) * v623) * v619;
            let v626 = v102 - v625;
            let v628 = if v102 < v627 { 1.0 } else { 0.0 };
            let v1661: f64;
            let v2313: f64;
            if v628 != 0.0 {
                let v634 = v633 * v626;
                let v637 = v629 * ((v2 + (v630 * v626)) - (v634 * v626));
                let v2945 = ((v2294 * v630) - (((v2294 * v633) * v626) + (v2294 * v634))) * v629;
                v1661 = v637;
                v2313 = v2945;
            } else {
                let v639 = v629 * v638;
                v1661 = v639;
                v2313 = v2667;
            }
            let v641 = v640 * v586;
            let v2946 = v2910 * v640;
            let v646 = v642 * ((v15 / v12).powf(v644));
            let v647 = if v322 > v0 { 1.0 } else { 0.0 };
            let v1786: f64;
            let v2314: f64;
            if v647 != 0.0 {
                let v648 = v2 / v326;
                let v2949 = ((v2676 * v648) * v2430) / v326;
                let v649 = if v648 > v23 { 1.0 } else { 0.0 };
                let v1787: f64;
                let v2315: f64;
                if v649 != 0.0 {
                    v1787 = v23;
                    v2315 = v2667;
                } else {
                    v1787 = v648;
                    v2315 = v2949;
                }
                v1786 = v1787;
                v2314 = v2315;
            } else {
                v1786 = v0;
                v2314 = v2667;
            }
            let v650 = if v327 > v0 { 1.0 } else { 0.0 };
            let v1791: f64;
            let v2316: f64;
            if v650 != 0.0 {
                let v651 = v2 / v331;
                let v2952 = ((v2679 * v651) * v2430) / v331;
                let v652 = if v651 > v23 { 1.0 } else { 0.0 };
                let v1792: f64;
                let v2317: f64;
                if v652 != 0.0 {
                    v1792 = v23;
                    v2317 = v2667;
                } else {
                    v1792 = v651;
                    v2317 = v2952;
                }
                v1791 = v1792;
                v2316 = v2317;
            } else {
                v1791 = v0;
                v2316 = v2667;
            }
            let v653 = if v332 > v0 { 1.0 } else { 0.0 };
            let v1796: f64;
            let v2318: f64;
            if v653 != 0.0 {
                let v654 = v2 / v333;
                let v2955 = ((v2680 * v654) * v2430) / v333;
                let v655 = if v654 > v23 { 1.0 } else { 0.0 };
                let v1797: f64;
                let v2319: f64;
                if v655 != 0.0 {
                    v1797 = v23;
                    v2319 = v2667;
                } else {
                    v1797 = v654;
                    v2319 = v2955;
                }
                v1796 = v1797;
                v2318 = v2319;
            } else {
                v1796 = v0;
                v2318 = v2667;
            }
            let v659 = v1 * (v656 - v657);
            let v2959 = ((Lanes([v2282, 0.0])) - (Lanes([0.0, v2283]))) * v1;
            let v662 = v1 * (v656 - v660);
            let v2963 = ((Lanes([v2282, 0.0])) - (Lanes([0.0, v2284]))) * v1;
            let v665 = v1 * (v656 - v663);
            let v2967 = ((Lanes([0.0, v2282])) - (Lanes([v2285, 0.0]))) * v1;
            let v668 = v1 * (v666 - v663);
            let v2971 = ((Lanes([0.0, v2286])) - (Lanes([v2285, 0.0]))) * v1;
            let v670 = v1 * (v666 - v656);
            let v2975 = ((Lanes([v2286, 0.0])) - (Lanes([0.0, v2282]))) * v1;
            let v672 = v1 * (v657 - v660);
            let v2979 = ((Lanes([v2283, 0.0])) - (Lanes([0.0, v2284]))) * v1;
            let v675 = v1 * (v673 - v663);
            let v2983 = ((Lanes([v2287, 0.0])) - (Lanes([0.0, v2285]))) * v1;
            let v678 = v1 * (v676 - v666);
            let v2987 = ((Lanes([v2288, 0.0])) - (Lanes([0.0, v2286]))) * v1;
            let v680 = v1 * (v676 - v673);
            let v2991 = ((Lanes([v2288, 0.0])) - (Lanes([0.0, v2287]))) * v1;
            let v683 = v1 * (v676 - v681);
            let v2995 = ((Lanes([0.0, v2288])) - (Lanes([v2289, 0.0]))) * v1;
            let v686 = v1 * (v684 - v657);
            let v2999 = ((Lanes([0.0, v2290])) - (Lanes([v2283, 0.0]))) * v1;
            let v689 = v1 * (v687 - v684);
            let v3003 = ((Lanes([v2291, 0.0])) - (Lanes([0.0, v2290]))) * v1;
            let v3006 = (Lanes([v2975[0], v2975[1], 0.0])) + (Lanes([0.0, v2963[0], v2963[1]]));
            let v3009 = (Lanes([v3006[0], v3006[1], 0.0, v3006[2]])) - (Lanes([0.0, 0.0, v2979[0], v2979[1]]));
            let v692 = ((v670 + v662) - v672) - v686;
            let v3012 = (Lanes([v3009[0], v3009[1], v3009[2], v3009[3], 0.0])) - (Lanes([0.0, 0.0, v2999[0], 0.0, v2999[1]]));
            let v3013 = v2995 * v2430;
            let v3016 = (Lanes([v3013[0], v3013[1], 0.0])) + (Lanes([0.0, v2987[0], v2987[1]]));
            let v3019 = (Lanes([v3016[0], v3016[1], v3016[2], 0.0, 0.0, 0.0, 0.0])) + (Lanes([0.0, 0.0, v3012[0], v3012[1], v3012[2], v3012[3], v3012[4]]));
            let v696 = (((-v683) + v678) + v692) - v689;
            let v3022 = (Lanes([v3019[0], v3019[1], v3019[2], v3019[3], v3019[4], v3019[5], 0.0, v3019[6]])) - (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, v3003[0], v3003[1]]));
            let v697 = v683 + v696;
            let v3024 = (Lanes([v2995[0], v2995[1], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + v3022;
            let v698 = v662 * v107;
            let v3025 = v2963 * v107;
            let v3029 = (Lanes([0.0, v3025[0], v3025[1]])) + (Lanes([(v2441 * v662), 0.0, 0.0]));
            let v700 = if v698 < v699 { 1.0 } else { 0.0 };
            let v922: f64;
            let v2320: Lanes<3>;
            if v700 != 0.0 {
                let v701 = v698.exp();
                let v3031 = v3029 * v701;
                v922 = v701;
                v2320 = v3031;
            } else {
                let v702 = v699.exp();
                let v705 = v702 * (v2 + (v698 - v699));
                let v3030 = v3029 * v702;
                v922 = v705;
                v2320 = v3030;
            }
            let v706 = v665 * v107;
            let v3032 = v2967 * v107;
            let v3036 = (Lanes([0.0, v3032[0], v3032[1]])) + (Lanes([(v2441 * v665), 0.0, 0.0]));
            let v707 = v706 / v410;
            let v3040 = (v3036 - (Lanes([(v2305 * v707), 0.0, 0.0]))) / v410;
            let v708 = if v707 < v699 { 1.0 } else { 0.0 };
            let v1027: f64;
            let v2321: Lanes<3>;
            if v708 != 0.0 {
                let v709 = v707.exp();
                let v3042 = v3040 * v709;
                v1027 = v709;
                v2321 = v3042;
            } else {
                let v710 = v699.exp();
                let v713 = v710 * (v2 + (v707 - v699));
                let v3041 = v3040 * v710;
                v1027 = v713;
                v2321 = v3041;
            }
            let v714 = v692 * v107;
            let v3043 = v3012 * v107;
            let v3047 = (Lanes([0.0, v3043[0], v3043[1], v3043[2], v3043[3], v3043[4]])) + (Lanes([(v2441 * v692), 0.0, 0.0, 0.0, 0.0, 0.0]));
            let v715 = if v714 < v699 { 1.0 } else { 0.0 };
            let v1413: f64;
            let v2322: Lanes<6>;
            if v715 != 0.0 {
                let v716 = v714.exp();
                let v3049 = v3047 * v716;
                v1413 = v716;
                v2322 = v3049;
            } else {
                let v717 = v699.exp();
                let v720 = v717 * (v2 + (v714 - v699));
                let v3048 = v3047 * v717;
                v1413 = v720;
                v2322 = v3048;
            }
            let v721 = v670 * v107;
            let v3050 = v2975 * v107;
            let v3054 = (Lanes([0.0, v3050[0], v3050[1]])) + (Lanes([(v2441 * v670), 0.0, 0.0]));
            let v722 = if v721 < v699 { 1.0 } else { 0.0 };
            let v1557: f64;
            let v2323: Lanes<3>;
            if v722 != 0.0 {
                let v723 = v721.exp();
                let v3056 = v3054 * v723;
                v1557 = v723;
                v2323 = v3056;
            } else {
                let v724 = v699.exp();
                let v727 = v724 * (v2 + (v721 - v699));
                let v3055 = v3054 * v724;
                v1557 = v727;
                v2323 = v3055;
            }
            let v728 = v697 * v107;
            let v3057 = v3024 * v107;
            let v3061 = (Lanes([v3057[0], v3057[1], 0.0, v3057[2], v3057[3], v3057[4], v3057[5], v3057[6], v3057[7]])) + (Lanes([0.0, 0.0, (v2441 * v697), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]));
            let v729 = if v728 < v699 { 1.0 } else { 0.0 };
            let v1443: f64;
            let v2324: Lanes<9>;
            if v729 != 0.0 {
                let v730 = v728.exp();
                let v3063 = v3061 * v730;
                v1443 = v730;
                v2324 = v3063;
            } else {
                let v731 = v699.exp();
                let v734 = v731 * (v2 + (v728 - v699));
                let v3062 = v3061 * v731;
                v1443 = v734;
                v2324 = v3062;
            }
            let v736 = v697 - v735;
            let v3064 = Lanes([v3024[0], v3024[1], 0.0, v3024[2], v3024[3], v3024[4], v3024[5], v3024[6], v3024[7]]);
            let v737 = v736 * v107;
            let v3070 = ((v3064 - (Lanes([0.0, 0.0, v2298, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) * v107) + (Lanes([0.0, 0.0, (v2441 * v736), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]));
            let v738 = if v737 < v699 { 1.0 } else { 0.0 };
            let v1993: f64;
            let v2325: Lanes<9>;
            if v738 != 0.0 {
                let v739 = v737.exp();
                let v3072 = v3070 * v739;
                v1993 = v739;
                v2325 = v3072;
            } else {
                let v740 = v699.exp();
                let v743 = v740 * (v2 + (v737 - v699));
                let v3071 = v3070 * v740;
                v1993 = v743;
                v2325 = v3071;
            }
            let v744 = v692 - v735;
            let v3073 = Lanes([0.0, v3012[0], v3012[1], v3012[2], v3012[3], v3012[4]]);
            let v745 = v744 * v107;
            let v3079 = ((v3073 - (Lanes([v2298, 0.0, 0.0, 0.0, 0.0, 0.0]))) * v107) + (Lanes([(v2441 * v744), 0.0, 0.0, 0.0, 0.0, 0.0]));
            let v746 = if v745 < v699 { 1.0 } else { 0.0 };
            let v1415: f64;
            let v2326: Lanes<6>;
            if v746 != 0.0 {
                let v747 = v745.exp();
                let v3081 = v3079 * v747;
                v1415 = v747;
                v2326 = v3081;
            } else {
                let v748 = v699.exp();
                let v751 = v748 * (v2 + (v745 - v699));
                let v3080 = v3079 * v748;
                v1415 = v751;
                v2326 = v3080;
            }
            let v752 = v662 - v735;
            let v753 = v752 * v107;
            let v3088 = (((Lanes([0.0, v2963[0], v2963[1]])) - (Lanes([v2298, 0.0, 0.0]))) * v107) + (Lanes([(v2441 * v752), 0.0, 0.0]));
            let v754 = if v753 < v699 { 1.0 } else { 0.0 };
            let v768: f64;
            let v2327: Lanes<3>;
            if v754 != 0.0 {
                let v755 = v753.exp();
                let v3090 = v3088 * v755;
                v768 = v755;
                v2327 = v3090;
            } else {
                let v756 = v699.exp();
                let v759 = v756 * (v2 + (v753 - v699));
                let v3089 = v3088 * v756;
                v768 = v759;
                v2327 = v3089;
            }
            let v760 = v659 - v735;
            let v3091 = Lanes([0.0, v2959[0], v2959[1]]);
            let v3092 = Lanes([v2298, 0.0, 0.0]);
            let v761 = v760 * v107;
            let v3097 = ((v3091 - v3092) * v107) + (Lanes([(v2441 * v760), 0.0, 0.0]));
            let v762 = if v761 < v699 { 1.0 } else { 0.0 };
            let v772: f64;
            let v2328: Lanes<3>;
            if v762 != 0.0 {
                let v763 = v761.exp();
                let v3099 = v3097 * v763;
                v772 = v763;
                v2328 = v3099;
            } else {
                let v764 = v699.exp();
                let v767 = v764 * (v2 + (v761 - v699));
                let v3098 = v3097 * v764;
                v772 = v767;
                v2328 = v3098;
            }
            let v771 = (v2 + (v404 * v768)).sqrt();
            let v3103 = (v2327 * v404) * (v2280 / (v2714 * v771));
            let v775 = (v2 + (v404 * v772)).sqrt();
            let v3107 = (v2328 * v404) * (v2280 / (v2714 * v775));
            let v777 = v2 + v775;
            let v778 = (v25 * v772) / v777;
            let v3111 = ((v2328 * v25) - (v3107 * v778)) / v777;
            let v780 = if v778 < v779 { 1.0 } else { 0.0 };
            let v867: f64;
            let v2329: Lanes<3>;
            if v780 != 0.0 {
                v867 = v779;
                v2329 = v3112;
            } else {
                v867 = v778;
                v2329 = v3111;
            }
            let v3113 = Lanes([v3103[0], v3103[1], 0.0, v3103[2]]);
            let v782 = v771 + v2;
            let v783 = v782 / v777;
            let v3116 = v3107 * v783;
            let v785 = (v771 - v775) - (v783.ln());
            let v786 = v105 * v785;
            let v3126 = (Lanes([(v2438 * v785), 0.0, 0.0, 0.0])) + (((v3113 - (Lanes([v3107[0], v3107[1], v3107[2], 0.0]))) - (((v3113 - (Lanes([v3116[0], v3116[1], v3116[2], 0.0]))) / v777) * (v2280 / v783))) * v105);
            let v3127 = Lanes([0.0, 0.0, v2979[0], v2979[1]]);
            let v788 = (v786 + v672) / v338;
            let v3132 = ((v3126 + v3127) - (Lanes([(v2683 * v788), 0.0, 0.0, 0.0]))) / v338;
            let v789 = if v788 > v0 { 1.0 } else { 0.0 };
            let v978: f64;
            let v991: f64;
            let v1006: f64;
            let v1033: f64;
            let v1609: f64;
            let v1645: f64;
            let v1950: f64;
            let v2330: Lanes<4>;
            let v2331: Lanes<4>;
            let v2332: Lanes<4>;
            let v2333: Lanes<4>;
            let v2334: Lanes<4>;
            let v2335: Lanes<4>;
            let v2336: Lanes<4>;
            if v789 != 0.0 {
                let v791 = if v659 < v790 { 1.0 } else { 0.0 };
                let v804: f64;
                let v2337: Lanes<2>;
                if v791 != 0.0 {
                    v804 = v659;
                    v2337 = v2959;
                } else {
                    let v793 = v2 + (v659 - v790);
                    let v3160 = v2959 * (v2280 / v793);
                    let v795 = v790 + (v793.ln());
                    v804 = v795;
                    v2337 = v3160;
                }
                let v796 = v25 * v105;
                let v797 = v393 * v788;
                let v798 = v797 * v338;
                let v3166 = ((v3132 * v393) * v338) + (Lanes([(v2683 * v797), 0.0, 0.0, 0.0]));
                let v800 = (v798 * v107) + v2;
                let v801 = v800.ln();
                let v805 = (v735 + (v796 * v801)) - v804;
                let v3180 = ((Lanes([v2298, 0.0, 0.0, 0.0])) + ((Lanes([((v2438 * v25) * v801), 0.0, 0.0, 0.0])) + ((((v3166 * v107) + (Lanes([(v2441 * v798), 0.0, 0.0, 0.0]))) * (v2280 / v800)) * v796))) - (Lanes([0.0, v2337[0], v2337[1], 0.0]));
                let v807 = v806 * v735;
                let v808 = v807 * v807;
                let v3182 = (v2298 * v806) * v807;
                let v3183 = v3182 + v3182;
                let v809 = v805 * v805;
                let v3184 = v3180 * v805;
                let v3185 = v3184 + v3184;
                let v810 = if v805 < v0 { 1.0 } else { 0.0 };
                let v820: f64;
                let v2338: Lanes<4>;
                if v810 != 0.0 {
                    let v813 = (v809 + v808).sqrt();
                    let v814 = v813 - v805;
                    let v815 = (v393 * v808) / v814;
                    let v3203 = ((Lanes([(v3183 * v393), 0.0, 0.0, 0.0])) - ((((v3185 + (Lanes([v3183, 0.0, 0.0, 0.0]))) * (v2280 / (v2714 * v813))) - v3180) * v815)) / v814;
                    v820 = v815;
                    v2338 = v3203;
                } else {
                    let v817 = (v809 + v808).sqrt();
                    let v819 = v393 * (v817 + v805);
                    let v3192 = (((v3185 + (Lanes([v3183, 0.0, 0.0, 0.0]))) * (v2280 / (v2714 * v817))) + v3180) * v393;
                    v820 = v819;
                    v2338 = v3192;
                }
                let v823 = v821 * v822;
                let v824 = v820 + v823;
                let v828 = v822 * (v820 + (v821 * v338));
                let v829 = (v820 * v824) / v828;
                let v3213 = (((v2338 * v824) + (v2338 * v820)) - (((v2338 + (Lanes([(v2683 * v821), 0.0, 0.0, 0.0]))) * v822) * v829)) / v828;
                let v830 = v788 / v829;
                let v3216 = (v3132 - (v3213 * v830)) / v829;
                let v833 = (v830 - v2) / v832;
                let v3217 = v3216 / v832;
                let v834 = if v830 < v2 { 1.0 } else { 0.0 };
                let v846: f64;
                let v2339: Lanes<4>;
                if v834 != 0.0 {
                    let v835 = v833.exp();
                    let v836 = v2 + v835;
                    let v3227 = ((v3217 * v835) * (v2280 / v836)) * v832;
                    let v839 = v2 + (v832 * (v836.ln()));
                    v846 = v839;
                    v2339 = v3227;
                } else {
                    let v841 = (-v833).exp();
                    let v842 = v2 + v841;
                    let v845 = v830 + (v832 * (v842.ln()));
                    let v3223 = v3216 + ((((v3217 * v2430) * v841) * (v2280 / v842)) * v832);
                    v846 = v845;
                    v2339 = v3223;
                }
                let v853 = v2 + (v832 * ((v2 + ((v847 / v832).exp())).ln()));
                let v854 = v846 / v853;
                let v3228 = v2339 / v853;
                let v855 = v820 / v823;
                let v3229 = v2338 / v823;
                let v856 = v404 * v854;
                let v857 = v856 * v855;
                let v858 = v2 + v855;
                let v861 = (v2 + (v857 * v858)).sqrt();
                let v863 = v25 * v854;
                let v864 = v863 * v858;
                let v865 = (v2 + v861) / v864;
                let v3246 = (((((((v3228 * v404) * v855) + (v3229 * v856)) * v858) + (v3229 * v857)) * (v2280 / (v2714 * v861))) - ((((v3228 * v25) * v858) + (v3229 * v863)) * v865)) / v864;
                let v868 = v867 * v865;
                let v3248 = v2329 * v865;
                let v3251 = (Lanes([v3248[0], v3248[1], v3248[2], 0.0])) + (v3246 * v867);
                let v870 = v2 + v868;
                let v871 = ((v2 - v865) + v868) / v870;
                let v3255 = (((v3246 * v2430) + v3251) - (v3251 * v871)) / v870;
                let v872 = v798 * v871;
                let v873 = v872 * v107;
                let v3262 = (((v3166 * v871) + (v3255 * v798)) * v107) + (Lanes([(v2441 * v872), 0.0, 0.0, 0.0]));
                let v876 = (v867 + v873) + v2;
                let v3266 = v2329 * v876;
                let v878 = (v25 * v873) + (v867 * v876);
                let v3270 = (v3262 * v25) + ((Lanes([v3266[0], v3266[1], v3266[2], 0.0])) + (((Lanes([v2329[0], v2329[1], v2329[2], 0.0])) + v3262) * v867));
                let v880 = v393 * (v873 - v2);
                let v3271 = v3262 * v393;
                let v3272 = v3271 * v880;
                let v882 = (v880 * v880) + v878;
                let v3274 = (v3272 + v3272) + v3270;
                let v883 = if v873 >= v2 { 1.0 } else { 0.0 };
                let v889: f64;
                let v2340: Lanes<4>;
                if v883 != 0.0 {
                    let v884 = v882.sqrt();
                    let v885 = v880 + v884;
                    let v3285 = v3271 + (v3274 * (v2280 / (v2714 * v884)));
                    v889 = v885;
                    v2340 = v3285;
                } else {
                    let v886 = v882.sqrt();
                    let v887 = v886 - v880;
                    let v888 = v878 / v887;
                    let v3281 = (v3270 - (((v3274 * (v2280 / (v2714 * v886))) - v3271) * v888)) / v887;
                    v889 = v888;
                    v2340 = v3281;
                }
                let v891 = if v889 < v890 { 1.0 } else { 0.0 };
                let v892: f64;
                let v2341: Lanes<4>;
                if v891 != 0.0 {
                    v892 = v890;
                    v2341 = v3158;
                } else {
                    v892 = v889;
                    v2341 = v2340;
                }
                let v893 = v892 + v2;
                let v894 = v892 * v893;
                let v896 = (v735 * v107).exp();
                let v897 = v894 * v896;
                let v3296 = (((v2341 * v893) + (v2341 * v892)) * v896) + (Lanes([((((v2298 * v107) + (v2441 * v735)) * v896) * v894), 0.0, 0.0, 0.0]));
                let v898 = v393 * v822;
                let v900 = v898 * (v788 - v821);
                let v3297 = v3132 * v898;
                let v902 = (v822 * v338) * v821;
                let v3304 = v3297 * v900;
                let v906 = ((v900 * v900) + (v902 * v788)).sqrt();
                let v907 = v900 + v906;
                let v3310 = v3297 + (((v3304 + v3304) + ((Lanes([(((v2683 * v822) * v821) * v788), 0.0, 0.0, 0.0])) + (v3132 * v902))) * (v2280 / (v2714 * v906)));
                let v909 = if v908 == v0 { 1.0 } else { 0.0 };
                let v992: f64;
                let v2342: Lanes<4>;
                if v909 != 0.0 {
                    let v910 = v294 * v40;
                    let v3321 = Lanes([(v2300 * v40), 0.0, 0.0, 0.0]);
                    v992 = v910;
                    v2342 = v3321;
                } else {
                    let v912 = v788 + v829;
                    let v913 = (v25 * v788) / v912;
                    let v914 = v40 + v913;
                    let v915 = v294 * v914;
                    let v3319 = (Lanes([(v2300 * v914), 0.0, 0.0, 0.0])) + ((((v3132 * v25) - ((v3132 + v3213) * v913)) / v912) * v294);
                    v992 = v915;
                    v2342 = v3319;
                }
                let v917 = v821 + v788;
                let v918 = (v821 * v788) / v917;
                let v3325 = ((v3132 * v821) - (v3132 * v918)) / v917;
                let v919 = v821 / v917;
                let v3328 = ((v3132 * v919) * v2430) / v917;
                v978 = v907;
                v991 = v992;
                v1006 = v919;
                v1033 = v897;
                v1609 = v871;
                v1645 = v918;
                v1950 = v892;
                v2330 = v3310;
                v2331 = v2342;
                v2332 = v3328;
                v2333 = v3296;
                v2334 = v3255;
                v2335 = v3325;
                v2336 = v2341;
            } else {
                let v921 = (v25 * v768) / v782;
                let v3136 = ((v2327 * v25) - (v3103 * v921)) / v782;
                let v933 = if (if (v672.abs()) < (v924 * v105) { 1.0 } else { 0.0 }) != 0.0 || (if (v786.abs()) < ((v928 * v105) * (v771 + v775)) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v1610: f64;
                let v2343: Lanes<4>;
                if v933 != 0.0 {
                    let v935 = v393 * (v921 + v867);
                    let v3148 = ((Lanes([v3136[0], v3136[1], 0.0, v3136[2]])) + (Lanes([v2329[0], v2329[1], v2329[2], 0.0]))) * v393;
                    let v936 = v935 + v2;
                    let v937 = v935 / v936;
                    let v3151 = (v3148 - (v3148 * v937)) / v936;
                    v1610 = v937;
                    v2343 = v3151;
                } else {
                    let v939 = (v786 + v662) - v659;
                    let v940 = v786 / v939;
                    let v3144 = (v3126 - (((v3126 + (Lanes([0.0, v2963[0], 0.0, v2963[1]]))) - (Lanes([0.0, v2959[0], v2959[1], 0.0]))) * v940)) / v939;
                    v1610 = v940;
                    v2343 = v3144;
                }
                let v941 = v40 * v294;
                let v943 = v2 - (v788 / v821);
                let v3154 = (v3132 / v821) * v2430;
                let v3155 = Lanes([(v2300 * v40), 0.0, 0.0, 0.0]);
                let v3156 = Lanes([v2320[0], v2320[1], 0.0, v2320[2]]);
                let v3157 = Lanes([v3136[0], v3136[1], 0.0, v3136[2]]);
                v978 = v672;
                v991 = v941;
                v1006 = v943;
                v1033 = v922;
                v1609 = v1610;
                v1645 = v788;
                v1950 = v921;
                v2330 = v3127;
                v2331 = v3155;
                v2332 = v3154;
                v2333 = v3156;
                v2334 = v2343;
                v2335 = v3132;
                v2336 = v3157;
            }
            let v947 = v2 - (v152.powf((v944 / v26)));
            let v948 = v282 * v947;
            let v3329 = v2297 * v947;
            let v949 = v40 * v282;
            let v3330 = v2297 * v40;
            let v3331 = Lanes([0.0, v2967[0], v2967[1]]);
            let v3332 = Lanes([v3329, 0.0, 0.0]);
            let v951 = (v665 - v948) / v949;
            let v3337 = ((v3331 - v3332) - (Lanes([(v3330 * v951), 0.0, 0.0]))) / v949;
            let v952 = if v665 < v948 { 1.0 } else { 0.0 };
            let v964: f64;
            let v2344: Lanes<3>;
            if v952 != 0.0 {
                let v953 = v951.exp();
                let v954 = v2 + v953;
                let v955 = v954.ln();
                let v957 = v665 - (v949 * v955);
                let v3354 = v3331 - ((Lanes([(v3330 * v955), 0.0, 0.0])) + (((v3337 * v953) * (v2280 / v954)) * v949));
                v964 = v957;
                v2344 = v3354;
            } else {
                let v959 = (-v951).exp();
                let v960 = v2 + v959;
                let v961 = v960.ln();
                let v963 = v948 - (v949 * v961);
                let v3346 = v3332 - ((Lanes([(v3330 * v961), 0.0, 0.0])) + ((((v3337 * v2430) * v959) * (v2280 / v960)) * v949));
                v964 = v963;
                v2344 = v3346;
            }
            let v966 = v2 - (v964 * v283);
            let v3359 = ((v2344 * v283) + (Lanes([(v2637 * v964), 0.0, 0.0]))) * v2430;
            let v967 = v2 - v26;
            let v968 = v966.powf(v967);
            let v3360 = v967 - v2280;
            let v3363 = v3359 * (v967 * (v966.powf(v3360)));
            let v969 = v282 / v967;
            let v3364 = v2297 / v967;
            let v970 = v2 - v968;
            let v974 = (v969 * v970) + (v152 * (v665 - v964));
            let v3372 = ((Lanes([(v3364 * v970), 0.0, 0.0])) + ((v3363 * v2430) * v969)) + ((v3331 - v2344) * v152);
            let v976 = if v975 == v2 { 1.0 } else { 0.0 };
            let v988: f64;
            let v2345: Lanes<4>;
            if v976 != 0.0 {
                let v3376 = Lanes([0.0, v2959[0], v2959[1], 0.0]);
                v988 = v659;
                v2345 = v3376;
            } else {
                let v977 = if v975 == v25 { 1.0 } else { 0.0 };
                let v989: f64;
                let v2346: Lanes<4>;
                if v977 != 0.0 {
                    let v979 = v659 + v978;
                    let v3375 = (Lanes([0.0, v2959[0], v2959[1], 0.0])) + v2330;
                    v989 = v979;
                    v2346 = v3375;
                } else {
                    let v3373 = Lanes([0.0, v2963[0], 0.0, v2963[1]]);
                    v989 = v662;
                    v2346 = v3373;
                }
                v988 = v989;
                v2345 = v2346;
            }
            let v3377 = v2663 * v2430;
            let v981 = v2 - v302;
            let v982 = (v25 - v302) / v981;
            let v3380 = (v3377 - (v3377 * v982)) / v981;
            let v984 = v983 / v58;
            let v986 = v2 - (v982.powf(v984));
            let v987 = v294 * v986;
            let v3388 = (v2300 * v986) + (((v3380 * (v984 * (v982.powf((v984 - v2280))))) * v2430) * v294);
            let v3389 = Lanes([v3388, 0.0, 0.0, 0.0]);
            let v993 = (v988 - v987) / v991;
            let v3393 = ((v2345 - v3389) - (v2331 * v993)) / v991;
            let v994 = if v988 < v987 { 1.0 } else { 0.0 };
            let v1011: f64;
            let v2347: Lanes<4>;
            if v994 != 0.0 {
                let v995 = v993.exp();
                let v996 = v2 + v995;
                let v997 = v996.ln();
                let v999 = v988 - (v991 * v997);
                let v3408 = v2345 - ((v2331 * v997) + (((v3393 * v995) * (v2280 / v996)) * v991));
                v1011 = v999;
                v2347 = v3408;
            } else {
                let v1001 = (-v993).exp();
                let v1002 = v2 + v1001;
                let v1003 = v1002.ln();
                let v1005 = v987 - (v991 * v1003);
                let v3401 = v3389 - ((v2331 * v1003) + ((((v3393 * v2430) * v1001) * (v2280 / v1002)) * v991));
                v1011 = v1005;
                v2347 = v3401;
            }
            let v1008 = v1006.powf(v1007);
            let v3412 = v2332 * (v1007 * (v1006.powf((v1007 - v2280))));
            let v1009 = v2 - v58;
            let v1010 = v294 / v1009;
            let v3413 = v2300 / v1009;
            let v1012 = v1011 / v294;
            let v1013 = v2 - v1012;
            let v1014 = v1013.powf(v1009);
            let v3419 = v1009 - v2280;
            let v1016 = v2 - (v1008 * v1014);
            let v1018 = v1008 * v982;
            let v1019 = v988 - v1011;
            let v1021 = (v1010 * v1016) + (v1018 * v1019);
            let v3445 = v2959 * v302;
            let v3448 = (Lanes([(v2663 * v659), 0.0, 0.0])) + (Lanes([0.0, v3445[0], v3445[1]]));
            let v1024 = (v981 * v1021) + (v302 * v659);
            let v3450 = ((Lanes([(v3377 * v1021), 0.0, 0.0, 0.0])) + ((((Lanes([(v3413 * v1016), 0.0, 0.0, 0.0])) + ((((v3412 * v1014) + (((((v2347 - (Lanes([(v2300 * v1012), 0.0, 0.0, 0.0]))) / v294) * v2430) * (v1009 * (v1013.powf(v3419)))) * v1008)) * v2430) * v1010)) + ((((v3412 * v982) + (Lanes([(v3380 * v1008), 0.0, 0.0, 0.0]))) * v1019) + ((v2345 - v2347) * v1018))) * v981)) + (Lanes([v3448[0], v3448[1], v3448[2], 0.0]));
            let v1026 = (v404 * v418) / v423;
            let v3454 = ((v2740 * v404) - (v2743 * v1026)) / v423;
            let v1028 = v1026 * v1027;
            let v3458 = (Lanes([(v3454 * v1027), 0.0, 0.0])) + (v2321 * v1026);
            let v1030 = (v2 + v1028).sqrt();
            let v3461 = v3458 * (v2280 / (v2714 * v1030));
            let v1031 = v2 + v1030;
            let v1032 = v1028 / v1031;
            let v3464 = (v3458 - (v3461 * v1032)) / v1031;
            let v1035 = v2 / v1034;
            let v1036 = v1033.powf(v1035);
            let v3472 = v1033.ln();
            let v3476 = (v2333 * (v1035 * (v1033.powf((v1035 - v2280))))) + (Lanes([((((v2307 * v1035) * v2430) / v1034) * (v1036 * v3472)), 0.0, 0.0, 0.0]));
            let v1037 = v1026 * v1036;
            let v3480 = (Lanes([(v3454 * v1036), 0.0, 0.0, 0.0])) + (v3476 * v1026);
            let v1039 = (v2 + v1037).sqrt();
            let v1040 = v2 + v1039;
            let v1041 = v1037 / v1040;
            let v3486 = (v3480 - ((v3480 * (v2280 / (v2714 * v1039))) * v1041)) / v1040;
            let v1042 = if v640 == v0 { 1.0 } else { 0.0 };
            let v1063: f64;
            let v2348: Lanes<5>;
            if v1042 != 0.0 {
                let v1043 = v974 / v592;
                let v3528 = (v3372 - (Lanes([(v2918 * v1043), 0.0, 0.0]))) / v592;
                let v1045 = v1024 / v589;
                let v3532 = (v3450 - (Lanes([(v2914 * v1045), 0.0, 0.0, 0.0]))) / v589;
                let v1046 = (v2 + v1043) + v1045;
                let v3535 = (Lanes([v3528[0], v3528[1], v3528[2], 0.0, 0.0])) + (Lanes([v3532[0], 0.0, v3532[1], v3532[2], v3532[3]]));
                v1063 = v1046;
                v2348 = v3535;
            } else {
                let v1047 = v974 / v592;
                let v1048 = v1047 + v2;
                let v1049 = v1048 * v641;
                let v1052 = (-v1024) / v589;
                let v1053 = v1052 * v641;
                let v1055 = (v1049 * v107).exp();
                let v3512 = ((((((v3372 - (Lanes([(v2918 * v1047), 0.0, 0.0]))) / v592) * v641) + (Lanes([(v2946 * v1048), 0.0, 0.0]))) * v107) + (Lanes([(v2441 * v1049), 0.0, 0.0]))) * v1055;
                let v1056 = (v1053 * v107).exp();
                let v3513 = (((((((v3450 * v2430) - (Lanes([(v2914 * v1052), 0.0, 0.0, 0.0]))) / v589) * v641) + (Lanes([(v2946 * v1052), 0.0, 0.0, 0.0]))) * v107) + (Lanes([(v2441 * v1053), 0.0, 0.0, 0.0]))) * v1056;
                let v1059 = (v641 * v107).exp();
                let v1060 = v1059 - v2;
                let v1061 = (v1055 - v1056) / v1060;
                let v3524 = (((Lanes([v3512[0], v3512[1], v3512[2], 0.0, 0.0])) - (Lanes([v3513[0], 0.0, v3513[1], v3513[2], v3513[3]]))) - (Lanes([((((v2946 * v107) + (v2441 * v641)) * v1059) * v1061), 0.0, 0.0, 0.0, 0.0]))) / v1060;
                v1063 = v1061;
                v2348 = v3524;
            }
            let v1064 = v1063 * v1063;
            let v3536 = v2348 * v1063;
            let v3537 = v3536 + v3536;
            let v1065 = if v1063 < v0 { 1.0 } else { 0.0 };
            let v1075: f64;
            let v2349: Lanes<5>;
            if v1065 != 0.0 {
                let v1068 = (v1064 + v1062).sqrt();
                let v1069 = v1068 - v1063;
                let v1070 = v1066 / v1069;
                let v3549 = ((((v3537 * (v2280 / (v2714 * v1068))) - v2348) * v1070) * v2430) / v1069;
                v1075 = v1070;
                v2349 = v3549;
            } else {
                let v1072 = (v1064 + v1062).sqrt();
                let v1074 = v393 * (v1072 + v1063);
                let v3542 = ((v3537 * (v2280 / (v2714 * v1072))) + v2348) * v393;
                v1075 = v1074;
                v2349 = v3542;
            }
            let v3553 = ((Lanes([v3464[0], v3464[1], v3464[2], 0.0, 0.0])) + (Lanes([v3486[0], 0.0, v3486[1], v3486[2], v3486[3]]))) * v393;
            let v1078 = v2 + (v393 * (v1032 + v1041));
            let v1079 = v1075 * v1078;
            let v3556 = (v2349 * v1078) + (v3553 * v1075);
            let v1081 = v1080 * v418;
            let v1082 = v1081 * v1036;
            let v3561 = (Lanes([((v2740 * v1080) * v1036), 0.0, 0.0, 0.0])) + (v3476 * v1081);
            let v1083 = v418 * v1027;
            let v3565 = (Lanes([(v2740 * v1027), 0.0, 0.0])) + (v2321 * v418);
            let v3566 = Lanes([v3565[0], v3565[1], v3565[2], 0.0, 0.0]);
            let v3567 = Lanes([v3561[0], 0.0, v3561[1], v3561[2], v3561[3]]);
            let v1085 = (v1083 - v1082) / v1079;
            let v3571 = ((v3566 - v3567) - (v3556 * v1085)) / v1079;
            let v1087 = v665 / v1086;
            let v3572 = v2967 / v1086;
            let v1088 = if v665 < v0 { 1.0 } else { 0.0 };
            let v1099: f64;
            let v2350: Lanes<2>;
            if v1088 != 0.0 {
                let v1089 = v1087.exp();
                let v1090 = v2 + v1089;
                let v1092 = v1086 * (v1090.ln());
                let v3582 = ((v3572 * v1089) * (v2280 / v1090)) * v1086;
                v1099 = v1092;
                v2350 = v3582;
            } else {
                let v1094 = (-v1087).exp();
                let v1095 = v2 + v1094;
                let v1098 = v665 + (v1086 * (v1095.ln()));
                let v3578 = v2967 + ((((v3572 * v2430) * v1094) * (v2280 / v1095)) * v1086);
                v1099 = v1098;
                v2350 = v3578;
            }
            let v1101 = v1099 / v1100;
            let v3583 = v2350 / v1100;
            let v1102 = if v1101 < v699 { 1.0 } else { 0.0 };
            let v1108: f64;
            let v2351: Lanes<2>;
            if v1102 != 0.0 {
                let v1103 = v1101.exp();
                let v3585 = v3583 * v1103;
                v1108 = v1103;
                v2351 = v3585;
            } else {
                let v1104 = v699.exp();
                let v1107 = v1104 * (v2 + (v1101 - v699));
                let v3584 = v3583 * v1104;
                v1108 = v1107;
                v2351 = v3584;
            }
            let v1109 = v1108 - v2;
            let v1110 = v536 * v1109;
            let v3587 = v2351 * v536;
            let v3590 = (Lanes([(v2826 * v1109), 0.0, 0.0])) + (Lanes([0.0, v3587[0], v3587[1]]));
            let v1113 = (v665 - v1111) / v24;
            let v3591 = v2967 / v24;
            let v1114 = if v665 < v1111 { 1.0 } else { 0.0 };
            let v1127: f64;
            let v2352: Lanes<2>;
            if v1114 != 0.0 {
                let v1115 = v1113.exp();
                let v1116 = v2 + v1115;
                let v1119 = v665 - (v24 * (v1116.ln()));
                let v3602 = v2967 - (((v3591 * v1115) * (v2280 / v1116)) * v24);
                v1127 = v1119;
                v2352 = v3602;
            } else {
                let v1121 = (-v1113).exp();
                let v1122 = v2 + v1121;
                let v1125 = v1111 - (v24 * (v1122.ln()));
                let v3597 = ((((v3591 * v2430) * v1121) * (v2280 / v1122)) * v24) * v2430;
                v1127 = v1125;
                v2352 = v3597;
            }
            let v1128 = v1126 * v1127;
            let v1129 = v1111 - v1127;
            let v1130 = v1129 * v1129;
            let v1131 = v1128 * v1130;
            let v3609 = ((v2352 * v1126) * v1130) + (((v2352 * v2430) * (v25 * v1129)) * v1128);
            let v1132 = v706 / v460;
            let v3610 = v3036 / v460;
            let v1133 = if v1132 < v699 { 1.0 } else { 0.0 };
            let v1158: f64;
            let v2353: Lanes<3>;
            if v1133 != 0.0 {
                let v1134 = v1132.exp();
                let v3612 = v3610 * v1134;
                v1158 = v1134;
                v2353 = v3612;
            } else {
                let v1135 = v699.exp();
                let v1138 = v1135 * (v2 + (v1132 - v699));
                let v3611 = v3610 * v1135;
                v1158 = v1138;
                v2353 = v3611;
            }
            let v1805: f64;
            let v2354: Lanes<5>;
            if v479 != 0.0 {
                let v1140 = v665 - v1139;
                let v1141 = v1140 * v107;
                let v3642 = ((v3331 - (Lanes([v2302, 0.0, 0.0]))) * v107) + (Lanes([(v2441 * v1140), 0.0, 0.0]));
                let v1142 = if v1141 < v699 { 1.0 } else { 0.0 };
                let v1164: f64;
                let v2355: Lanes<3>;
                if v1142 != 0.0 {
                    let v1143 = v1141.exp();
                    let v3644 = v3642 * v1143;
                    v1164 = v1143;
                    v2355 = v3644;
                } else {
                    let v1144 = v699.exp();
                    let v1147 = v1144 * (v2 + (v1141 - v699));
                    let v3643 = v3642 * v1144;
                    v1164 = v1147;
                    v2355 = v3643;
                }
                let v1148 = v1085 / v418;
                let v3648 = (v3571 - (Lanes([(v2740 * v1148), 0.0, 0.0, 0.0, 0.0]))) / v418;
                let v1150 = v1148 - v1149;
                let v1152 = if v1150 < v1151 { 1.0 } else { 0.0 };
                let v1177: f64;
                let v2356: Lanes<5>;
                if v1152 != 0.0 {
                    let v1153 = v1150.exp();
                    let v3650 = v3648 * v1153;
                    v1177 = v1153;
                    v2356 = v3650;
                } else {
                    let v1157 = v1154 * (v2 + (v1150 - v1151));
                    let v3649 = v3648 * v1154;
                    v1177 = v1157;
                    v2356 = v3649;
                }
                let v1159 = v1158 - v2;
                let v3654 = (Lanes([(v2774 * v1159), 0.0, 0.0])) + (v2353 * v469);
                let v1162 = v1161 * v25;
                let v1167 = (v2 + (v404 * v1164)).sqrt();
                let v1168 = v2 + v1167;
                let v1169 = (v1162 * v1159) / v1168;
                let v1170 = v1024 / v589;
                let v1171 = v2 + v1170;
                let v3671 = ((((Lanes([((v2310 * v25) * v1159), 0.0, 0.0])) + (v2353 * v1162)) - (((v2355 * v404) * (v2280 / (v2714 * v1167))) * v1169)) / v1168) * v1171;
                let v3672 = ((v3450 - (Lanes([(v2914 * v1170), 0.0, 0.0, 0.0]))) / v589) * v1169;
                let v1175 = v1033 - v2;
                let v1176 = v1174 * v1175;
                let v3682 = ((Lanes([(v2311 * v1175), 0.0, 0.0, 0.0])) + (v2333 * v1174)) * v1177;
                let v1179 = v2 + v1177;
                let v1180 = (v1176 * v1177) / v1179;
                let v1181 = ((v469 * v1159) + (v1169 * v1171)) + v1180;
                let v3689 = ((Lanes([v3654[0], v3654[1], v3654[2], 0.0, 0.0])) + ((Lanes([v3671[0], v3671[1], v3671[2], 0.0, 0.0])) + (Lanes([v3672[0], 0.0, v3672[1], v3672[2], v3672[3]])))) + ((((Lanes([v3682[0], 0.0, v3682[1], v3682[2], v3682[3]])) + (v2356 * v1176)) - (v2356 * v1180)) / v1179);
                v1805 = v1181;
                v2354 = v3689;
            } else {
                let v1183 = if v1182 == v0 { 1.0 } else { 0.0 };
                let v1806: f64;
                let v2357: Lanes<5>;
                if v1183 != 0.0 {
                    let v1184 = v1158 - v2;
                    let v1185 = v469 * v1184;
                    let v3635 = (Lanes([(v2774 * v1184), 0.0, 0.0])) + (v2353 * v469);
                    let v3636 = Lanes([v3635[0], v3635[1], v3635[2], 0.0, 0.0]);
                    v1806 = v1185;
                    v2357 = v3636;
                } else {
                    let v1186 = v2 - v1182;
                    let v3613 = v2353 * v1186;
                    let v1191 = v1182 * ((v1158 + v1033) - v25);
                    let v1192 = v1024 / v589;
                    let v1193 = v2 + v1192;
                    let v3623 = ((v3450 - (Lanes([(v2914 * v1192), 0.0, 0.0, 0.0]))) / v589) * v1191;
                    let v1195 = (v1186 * (v1158 - v2)) + (v1191 * v1193);
                    let v1196 = v469 * v1195;
                    let v3631 = (Lanes([(v2774 * v1195), 0.0, 0.0, 0.0, 0.0])) + (((Lanes([v3613[0], v3613[1], v3613[2], 0.0, 0.0])) + (((((Lanes([v2353[0], v2353[1], v2353[2], 0.0, 0.0])) + (Lanes([v2333[0], 0.0, v2333[1], v2333[2], v2333[3]]))) * v1182) * v1193) + (Lanes([v3623[0], 0.0, v3623[1], v3623[2], v3623[3]])))) * v469);
                    v1806 = v1196;
                    v2357 = v3631;
                }
                v1805 = v1806;
                v2354 = v2357;
            }
            let v1197 = v668 * v107;
            let v3690 = v2971 * v107;
            let v3694 = (Lanes([0.0, v3690[0], v3690[1]])) + (Lanes([(v2441 * v668), 0.0, 0.0]));
            let v1198 = v1197 / v471;
            let v3695 = v3694 / v471;
            let v1199 = if v1198 < v699 { 1.0 } else { 0.0 };
            let v1213: f64;
            let v2358: Lanes<3>;
            if v1199 != 0.0 {
                let v1200 = v1198.exp();
                let v3697 = v3695 * v1200;
                v1213 = v1200;
                v2358 = v3697;
            } else {
                let v1201 = v699.exp();
                let v1204 = v1201 * (v2 + (v1198 - v699));
                let v3696 = v3695 * v1201;
                v1213 = v1204;
                v2358 = v3696;
            }
            let v1819: f64;
            let v2359: Lanes<3>;
            if v479 != 0.0 {
                let v1205 = v668 - v1139;
                let v1206 = v1205 * v107;
                let v3708 = (((Lanes([0.0, v2971[0], v2971[1]])) - (Lanes([v2302, 0.0, 0.0]))) * v107) + (Lanes([(v2441 * v1205), 0.0, 0.0]));
                let v1207 = if v1206 < v699 { 1.0 } else { 0.0 };
                let v1219: f64;
                let v2360: Lanes<3>;
                if v1207 != 0.0 {
                    let v1208 = v1206.exp();
                    let v3710 = v3708 * v1208;
                    v1219 = v1208;
                    v2360 = v3710;
                } else {
                    let v1209 = v699.exp();
                    let v1212 = v1209 * (v2 + (v1206 - v699));
                    let v3709 = v3708 * v1209;
                    v1219 = v1212;
                    v2360 = v3709;
                }
                let v1214 = v1213 - v2;
                let v1217 = v1216 * v25;
                let v1222 = (v2 + (v404 * v1219)).sqrt();
                let v1223 = v2 + v1222;
                let v1224 = (v1217 * v1214) / v1223;
                let v1225 = (v477 * v1214) + v1224;
                let v3727 = ((Lanes([(v2782 * v1214), 0.0, 0.0])) + (v2358 * v477)) + ((((Lanes([((v2312 * v25) * v1214), 0.0, 0.0])) + (v2358 * v1217)) - (((v2360 * v404) * (v2280 / (v2714 * v1222))) * v1224)) / v1223);
                v1819 = v1225;
                v2359 = v3727;
            } else {
                let v1226 = v1213 - v2;
                let v1227 = v477 * v1226;
                let v3701 = (Lanes([(v2782 * v1226), 0.0, 0.0])) + (v2358 * v477);
                v1819 = v1227;
                v2359 = v3701;
            }
            let v1228 = v706 / v432;
            let v3728 = v3036 / v432;
            let v1229 = if v1228 < v699 { 1.0 } else { 0.0 };
            let v1235: f64;
            let v2361: Lanes<3>;
            if v1229 != 0.0 {
                let v1230 = v1228.exp();
                let v3730 = v3728 * v1230;
                v1235 = v1230;
                v2361 = v3730;
            } else {
                let v1231 = v699.exp();
                let v1234 = v1231 * (v2 + (v1228 - v699));
                let v3729 = v3728 * v1231;
                v1235 = v1234;
                v2361 = v3729;
            }
            let v1236 = v1235 - v2;
            let v1237 = v443 * v1236;
            let v3734 = (Lanes([(v2755 * v1236), 0.0, 0.0])) + (v2361 * v443);
            let v1238 = v1197 / v512;
            let v3735 = v3694 / v512;
            let v1239 = if v1238 < v699 { 1.0 } else { 0.0 };
            let v1245: f64;
            let v2362: Lanes<3>;
            if v1239 != 0.0 {
                let v1240 = v1238.exp();
                let v3737 = v3735 * v1240;
                v1245 = v1240;
                v2362 = v3737;
            } else {
                let v1241 = v699.exp();
                let v1244 = v1241 * (v2 + (v1238 - v699));
                let v3736 = v3735 * v1241;
                v1245 = v1244;
                v2362 = v3736;
            }
            let v1246 = v1245 - v2;
            let v1247 = v520 * v1246;
            let v3741 = (Lanes([(v2809 * v1246), 0.0, 0.0])) + (v2362 * v520);
            let v1248 = v714 / v445;
            let v3742 = v3047 / v445;
            let v1249 = if v1248 < v699 { 1.0 } else { 0.0 };
            let v1255: f64;
            let v2363: Lanes<6>;
            if v1249 != 0.0 {
                let v1250 = v1248.exp();
                let v3744 = v3742 * v1250;
                v1255 = v1250;
                v2363 = v3744;
            } else {
                let v1251 = v699.exp();
                let v1254 = v1251 * (v2 + (v1248 - v699));
                let v3743 = v3742 * v1251;
                v1255 = v1254;
                v2363 = v3743;
            }
            let v1256 = v1255 - v2;
            let v1257 = v455 * v1256;
            let v3748 = (Lanes([(v2764 * v1256), 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2363 * v455);
            let v1258 = v1197 / v522;
            let v3749 = v3694 / v522;
            let v1259 = if v1258 < v699 { 1.0 } else { 0.0 };
            let v1265: f64;
            let v2364: Lanes<3>;
            if v1259 != 0.0 {
                let v1260 = v1258.exp();
                let v3751 = v3749 * v1260;
                v1265 = v1260;
                v2364 = v3751;
            } else {
                let v1261 = v699.exp();
                let v1264 = v1261 * (v2 + (v1258 - v699));
                let v3750 = v3749 * v1261;
                v1265 = v1264;
                v2364 = v3750;
            }
            let v1266 = v1265 - v2;
            let v1267 = v529 * v1266;
            let v3755 = (Lanes([(v2817 * v1266), 0.0, 0.0])) + (v2364 * v529);
            let v1271 = if (if (if v551 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v542 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v1088 != 0.0 { 1.0 } else { 0.0 };
            let v1810: f64;
            let v2365: Lanes<3>;
            if v1271 != 0.0 {
                let v1272 = v25 * v968;
                let v1273 = v28 / v1272;
                let v1274 = v2 - v1273;
                let v1275 = v550 * v1274;
                let v3765 = (Lanes([(v2850 * v1274), 0.0, 0.0])) + ((((((v3363 * v25) * v1273) * v2430) / v1272) * v2430) * v550);
                let v1276 = if v1275 < v699 { 1.0 } else { 0.0 };
                let v1337: f64;
                let v2366: Lanes<3>;
                if v1276 != 0.0 {
                    let v1277 = v1275.exp();
                    let v3767 = v3765 * v1277;
                    v1337 = v1277;
                    v2366 = v3767;
                } else {
                    let v1278 = v699.exp();
                    let v1281 = v1278 * (v2 + (v1275 - v699));
                    let v3766 = v3765 * v1278;
                    v1337 = v1281;
                    v2366 = v3766;
                }
                let v1282 = v665 * v283;
                let v3768 = v2967 * v283;
                let v3772 = (Lanes([0.0, v3768[0], v3768[1]])) + (Lanes([(v2637 * v665), 0.0, 0.0]));
                let v3773 = v3772 * v1282;
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
                let v3799 = (v2967 * v28) * v550;
                let v1307 = v537 * v1304;
                let v1308 = (v1305 * v550) / v1307;
                let v3810 = (((Lanes([0.0, v3799[0], v3799[1]])) + (Lanes([(v2850 * v1305), 0.0, 0.0]))) - (((Lanes([(v2295 * v1304), 0.0, 0.0])) + (((((((v3773 + v3773) * (v2280 / (v2714 * v1286))) * (v1288 * (v1286.powf((v1288 - v2280))))) * v1301) + ((((((v3772 * v152) * v1293) * v2430) * v26) - (((((v3772 * v431) * v1282) + (v3772 * v1297)) * v1299) + (v3772 * v1298))) * v1289)) * v1303) * v537)) * v1308)) / v1307;
                let v1310 = if v1308 < v1309 { 1.0 } else { 0.0 };
                let v1334: f64;
                let v2367: Lanes<3>;
                if v1310 != 0.0 {
                    let v1311 = if v1308 < v699 { 1.0 } else { 0.0 };
                    let v1318: f64;
                    let v2368: Lanes<3>;
                    if v1311 != 0.0 {
                        let v1312 = v1308.exp();
                        let v3825 = v3810 * v1312;
                        v1318 = v1312;
                        v2368 = v3825;
                    } else {
                        let v1313 = v699.exp();
                        let v1316 = v1313 * (v2 + (v1308 - v699));
                        let v3824 = v3810 * v1313;
                        v1318 = v1316;
                        v2368 = v3824;
                    }
                    let v1317 = -v665;
                    let v1320 = (v2 - v1318) / v1308;
                    let v1321 = v2 + v1320;
                    let v1322 = v1317 * v1321;
                    let v3831 = (v2967 * v2430) * v1321;
                    let v3834 = (Lanes([0.0, v3831[0], v3831[1]])) + ((((v2368 * v2430) - (v3810 * v1320)) / v1308) * v1317);
                    v1334 = v1322;
                    v2367 = v3834;
                } else {
                    let v1323 = v665 * v393;
                    let v1324 = v1323 * v1308;
                    let v3812 = (v2967 * v393) * v1308;
                    let v1326 = v1308 * v1325;
                    let v1329 = v2 + (v1327 * v1308);
                    let v1331 = v2 + (v1326 * v1329);
                    let v1332 = v1324 * v1331;
                    let v3823 = (((Lanes([0.0, v3812[0], v3812[1]])) + (v3810 * v1323)) * v1331) + ((((v3810 * v1325) * v1329) + ((v3810 * v1327) * v1326)) * v1324);
                    v1334 = v1332;
                    v2367 = v3823;
                }
                let v1333 = v25 * v560;
                let v1335 = v1333 * v1334;
                let v1336 = v1335 * v968;
                let v1338 = v1336 * v1337;
                let v1340 = (v1338 * v283) * v29;
                let v3850 = ((((((((Lanes([((v2867 * v25) * v1334), 0.0, 0.0])) + (v2367 * v1333)) * v968) + (v3363 * v1335)) * v1337) + (v2366 * v1336)) * v283) + (Lanes([(v2637 * v1338), 0.0, 0.0]))) * v29;
                v1810 = v1340;
                v2365 = v3850;
            } else {
                v1810 = v0;
                v2365 = v3756;
            }
            let v1345 = if (if (if v575 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v566 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v659 < v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v1525: f64;
            let v2369: Lanes<3>;
            if v1345 != 0.0 {
                let v1346 = v659 * v285;
                let v3851 = v2959 * v285;
                let v3855 = (Lanes([0.0, v3851[0], v3851[1]])) + (Lanes([(v2640 * v659), 0.0, 0.0]));
                let v1347 = v2 - v1346;
                let v1348 = v1347.powf(v1009);
                let v3859 = (v3855 * v2430) * (v1009 * (v1347.powf(v3419)));
                let v1349 = v25 * v1348;
                let v1350 = v60 / v1349;
                let v1351 = v2 - v1350;
                let v1352 = v574 * v1351;
                let v3868 = (Lanes([(v2891 * v1351), 0.0, 0.0])) + ((((((v3859 * v25) * v1350) * v2430) / v1349) * v2430) * v574);
                let v1353 = if v1352 < v699 { 1.0 } else { 0.0 };
                let v1409: f64;
                let v2370: Lanes<3>;
                if v1353 != 0.0 {
                    let v1354 = v1352.exp();
                    let v3870 = v3868 * v1354;
                    v1409 = v1354;
                    v2370 = v3870;
                } else {
                    let v1355 = v699.exp();
                    let v1358 = v1355 * (v2 + (v1352 - v699));
                    let v3869 = v3868 * v1355;
                    v1409 = v1358;
                    v2370 = v3869;
                }
                let v3871 = v3855 * v1346;
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
                let v3897 = (v2959 * v60) * v574;
                let v1381 = v561 * v1378;
                let v1382 = (v1379 * v574) / v1381;
                let v3908 = (((Lanes([0.0, v3897[0], v3897[1]])) + (Lanes([(v2891 * v1379), 0.0, 0.0]))) - (((Lanes([(v2296 * v1378), 0.0, 0.0])) + (((((((v3871 + v3871) * (v2280 / (v2714 * v1361))) * (v1363 * (v1361.powf((v1363 - v2280))))) * v1376) + ((((((v3855 * v152) * v1368) * v2430) * v58) - (((((v3855 * v431) * v1346) + (v3855 * v1372)) * v1374) + (v3855 * v1373))) * v1364)) * v1303) * v561)) * v1382)) / v1381;
                let v1384 = if v1382 < v1383 { 1.0 } else { 0.0 };
                let v1406: f64;
                let v2371: Lanes<3>;
                if v1384 != 0.0 {
                    let v1385 = if v1382 < v699 { 1.0 } else { 0.0 };
                    let v1392: f64;
                    let v2372: Lanes<3>;
                    if v1385 != 0.0 {
                        let v1386 = v1382.exp();
                        let v3923 = v3908 * v1386;
                        v1392 = v1386;
                        v2372 = v3923;
                    } else {
                        let v1387 = v699.exp();
                        let v1390 = v1387 * (v2 + (v1382 - v699));
                        let v3922 = v3908 * v1387;
                        v1392 = v1390;
                        v2372 = v3922;
                    }
                    let v1391 = -v659;
                    let v1394 = (v2 - v1392) / v1382;
                    let v1395 = v2 + v1394;
                    let v1396 = v1391 * v1395;
                    let v3929 = (v2959 * v2430) * v1395;
                    let v3932 = (Lanes([0.0, v3929[0], v3929[1]])) + ((((v2372 * v2430) - (v3908 * v1394)) / v1382) * v1391);
                    v1406 = v1396;
                    v2371 = v3932;
                } else {
                    let v1397 = v659 * v393;
                    let v1398 = v1397 * v1382;
                    let v3910 = (v2959 * v393) * v1382;
                    let v1399 = v1382 * v1325;
                    let v1401 = v2 + (v1327 * v1382);
                    let v1403 = v2 + (v1399 * v1401);
                    let v1404 = v1398 * v1403;
                    let v3921 = (((Lanes([0.0, v3910[0], v3910[1]])) + (v3908 * v1397)) * v1403) + ((((v3908 * v1325) * v1401) + ((v3908 * v1327) * v1399)) * v1398);
                    v1406 = v1404;
                    v2371 = v3921;
                }
                let v1405 = v25 * v584;
                let v1407 = v1405 * v1406;
                let v1408 = v1407 * v1348;
                let v1410 = v1408 * v1409;
                let v1412 = (v1410 * v285) * v61;
                let v3948 = ((((((((Lanes([((v2908 * v25) * v1406), 0.0, 0.0])) + (v2371 * v1405)) * v1348) + (v3859 * v1407)) * v1409) + (v2370 * v1408)) * v285) + (Lanes([(v2640 * v1410), 0.0, 0.0]))) * v61;
                v1525 = v1412;
                v2369 = v3948;
            } else {
                v1525 = v0;
                v2369 = v3112;
            }
            let v1414 = v1026 * v1413;
            let v3952 = (Lanes([(v3454 * v1413), 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2322 * v1026);
            let v1416 = v404 * v1415;
            let v3953 = v2326 * v404;
            let v1419 = (v2 + v1414).sqrt();
            let v1420 = v2 + v1419;
            let v1421 = (v1414 - v1026) / v1420;
            let v3961 = ((v3952 - (Lanes([v3454, 0.0, 0.0, 0.0, 0.0, 0.0]))) - ((v3952 * (v2280 / (v2714 * v1419))) * v1421)) / v1420;
            let v1423 = (v2 + v1416).sqrt();
            let v1424 = v2 + v1423;
            let v1425 = v1416 / v1424;
            let v3967 = (v3953 - ((v3953 * (v2280 / (v2714 * v1423))) * v1425)) / v1424;
            let v1426 = v25 * v510;
            let v3968 = v2801 * v25;
            let v1427 = v1413 - v2;
            let v1430 = (v404 * v510) / v429;
            let v3976 = ((v2801 * v404) - (v2746 * v1430)) / v429;
            let v1433 = (v2 + (v1430 * v1413)).sqrt();
            let v1434 = v2 + v1433;
            let v1435 = (v1426 * v1427) / v1434;
            let v3986 = (((Lanes([(v3968 * v1427), 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2322 * v1426)) - ((((Lanes([(v3976 * v1413), 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2322 * v1430)) * (v2280 / (v2714 * v1433))) * v1435)) / v1434;
            let v1438 = if v8 > v0 { 1.0 } else { 0.0 };
            let v1439 = if (if v1436 > v0 { 1.0 } else { 0.0 }) != 0.0 && v1438 != 0.0 { 1.0 } else { 0.0 };
            let v1529: f64;
            let v1532: f64;
            let v2024: f64;
            let v2373: Lanes<6>;
            let v2374: Lanes<9>;
            let v2375: Lanes<9>;
            if v1439 != 0.0 {
                let v1440 = v1435 * v9;
                let v3988 = v3986 * v9;
                let v1441 = v8 * v25;
                let v1442 = v1441 * v510;
                let v1444 = v1443 - v2;
                let v1448 = (v2 + (v1430 * v1443)).sqrt();
                let v1449 = v2 + v1448;
                let v1450 = (v1442 * v1444) / v1449;
                let v4003 = (((Lanes([0.0, 0.0, ((v2801 * v1441) * v1444), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2324 * v1442)) - ((((Lanes([0.0, 0.0, (v3976 * v1443), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2324 * v1430)) * (v2280 / (v2714 * v1448))) * v1450)) / v1449;
                let v1451 = if v1436 == v2 { 1.0 } else { 0.0 };
                let v1476: f64;
                let v2376: Lanes<9>;
                if v1451 != 0.0 {
                    let v1452 = v8 * v510;
                    let v1453 = v1452 * v326;
                    let v4007 = ((v2801 * v8) * v326) + (v2676 * v1452);
                    let v1454 = v1453 * v107;
                    let v1456 = v25 - (v1454.ln());
                    let v1458 = v697 - (v105 * v1456);
                    let v4018 = v3064 - (Lanes([0.0, 0.0, ((v2438 * v1456) + (((((v4007 * v107) + (v2441 * v1453)) * (v2280 / v1454)) * v2430) * v105)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]));
                    let v1460 = v1458 * v1458;
                    let v4019 = v4018 * v1458;
                    let v4020 = v4019 + v4019;
                    let v1461 = if v1458 < v0 { 1.0 } else { 0.0 };
                    let v1471: f64;
                    let v2377: Lanes<9>;
                    if v1461 != 0.0 {
                        let v1464 = (v1460 + v1459).sqrt();
                        let v1465 = v1464 - v1458;
                        let v1466 = v1462 / v1465;
                        let v4032 = ((((v4020 * (v2280 / (v2714 * v1464))) - v4018) * v1466) * v2430) / v1465;
                        v1471 = v1466;
                        v2377 = v4032;
                    } else {
                        let v1468 = (v1460 + v1459).sqrt();
                        let v1470 = v393 * (v1468 + v1458);
                        let v4025 = ((v4020 * (v2280 / (v2714 * v1468))) + v4018) * v393;
                        v1471 = v1470;
                        v2377 = v4025;
                    }
                    let v1474 = (v1453 + (v1450 * v326)) + v1471;
                    let v1475 = v1471 / v1474;
                    let v4042 = (v2377 - ((((Lanes([0.0, 0.0, v4007, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v4003 * v326) + (Lanes([0.0, 0.0, (v2676 * v1450), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])))) + v2377) * v1475)) / v1474;
                    v1476 = v1475;
                    v2376 = v4042;
                } else {
                    v1476 = v2;
                    v2376 = v3987;
                }
                let v1477 = v1476 * v1450;
                let v4045 = (v2376 * v1450) + (v4003 * v1476);
                v1529 = v1440;
                v1532 = v1477;
                v2024 = v1476;
                v2373 = v3988;
                v2374 = v4045;
                v2375 = v2376;
            } else {
                v1529 = v1435;
                v1532 = v0;
                v2024 = v2;
                v2373 = v3986;
                v2374 = v3987;
                v2375 = v3987;
            }
            let v1479 = if v1478 == v2 { 1.0 } else { 0.0 };
            let v1526: f64;
            let v2378: Lanes<3>;
            if v1479 != 0.0 {
                let v1480 = v670 + v659;
                let v4049 = (Lanes([v2975[0], v2975[1], 0.0])) + (Lanes([0.0, v2959[0], v2959[1]]));
                let v1486 = (v1483 * v1480) * v1485;
                let v1487 = v1486 * v1480;
                let v4054 = (((v4049 * v1483) * v1485) * v1480) + (v4049 * v1486);
                let v1490 = if (v1488 * v1480) < v0 { 1.0 } else { 0.0 };
                let v1516: f64;
                let v2379: Lanes<3>;
                if v1490 != 0.0 {
                    let v1493 = (v1487 + v1482).sqrt();
                    let v1496 = v1493 - (v1494 * v1480);
                    let v1497 = v1491 / v1496;
                    let v4068 = ((((v4054 * (v2280 / (v2714 * v1493))) - (v4049 * v1494)) * v1497) * v2430) / v1496;
                    v1516 = v1497;
                    v2379 = v4068;
                } else {
                    let v1499 = (v1487 + v1482).sqrt();
                    let v1503 = v393 * (v1499 + (v1500 * v1480));
                    let v4060 = ((v4054 * (v2280 / (v2714 * v1499))) + (v4049 * v1500)) * v393;
                    v1516 = v1503;
                    v2379 = v4060;
                }
                let v1507 = v2 / (v2 - (v88.powf(v1504)));
                let v1509 = v88 * v1508;
                let v1515 = (((v1507 * v1507) * (v88.powf((v1504 - v2)))) * v1504) / v1508;
                let v1517 = if v1516 < v1509 { 1.0 } else { 0.0 };
                let v1527: f64;
                let v2380: Lanes<3>;
                if v1517 != 0.0 {
                    let v1518 = v1516 / v1508;
                    let v1520 = v2 - (v1518.powf(v1504));
                    let v1521 = v2 / v1520;
                    let v4078 = (((((v2379 / v1508) * (v1504 * (v1518.powf((v1504 - v2280))))) * v2430) * v1521) * v2430) / v1520;
                    v1527 = v1521;
                    v2380 = v4078;
                } else {
                    let v4069 = v2379 * v1515;
                    let v1524 = v1507 + ((v1516 - v1509) * v1515);
                    v1527 = v1524;
                    v2380 = v4069;
                }
                v1526 = v1527;
                v2378 = v2380;
            } else {
                v1526 = v2;
                v2378 = v4046;
            }
            let v1528 = v1525 * v1526;
            let v4079 = v2369 * v1526;
            let v4080 = v2378 * v1525;
            let v4083 = (Lanes([v4079[0], 0.0, v4079[1], v4079[2]])) + (Lanes([0.0, v4080[0], v4080[1], v4080[2]]));
            let v1530 = v1529 * v1526;
            let v4085 = v2378 * v1529;
            let v4087 = (v2373 * v1526) + (Lanes([0.0, v4085[0], v4085[1], v4085[2], 0.0, 0.0]));
            let v1531 = v1257 * v1526;
            let v4089 = v2378 * v1257;
            let v4091 = (v3748 * v1526) + (Lanes([0.0, v4089[0], v4089[1], v4089[2], 0.0, 0.0]));
            let v1533 = v1532 * v1526;
            let v4093 = v2378 * v1532;
            let v4095 = (v2374 * v1526) + (Lanes([0.0, 0.0, 0.0, v4093[0], v4093[1], v4093[2], 0.0, 0.0, 0.0]));
            let v1534 = v974 / v592;
            let v4099 = (v3372 - (Lanes([(v2918 * v1534), 0.0, 0.0]))) / v592;
            let v1536 = v1024 / v589;
            let v4103 = (v3450 - (Lanes([(v2914 * v1536), 0.0, 0.0, 0.0]))) / v589;
            let v1537 = (v2 + v1534) + v1536;
            let v4106 = (Lanes([v4099[0], v4099[1], v4099[2], 0.0, 0.0])) + (Lanes([v4103[0], 0.0, v4103[1], v4103[2], v4103[3]]));
            let v1539 = v1537 * v1537;
            let v4107 = v4106 * v1537;
            let v4108 = v4107 + v4107;
            let v1540 = if v1537 < v0 { 1.0 } else { 0.0 };
            let v1550: f64;
            let v2381: Lanes<5>;
            if v1540 != 0.0 {
                let v1543 = (v1539 + v1538).sqrt();
                let v1544 = v1543 - v1537;
                let v1545 = v1541 / v1544;
                let v4120 = ((((v4108 * (v2280 / (v2714 * v1543))) - v4106) * v1545) * v2430) / v1544;
                v1550 = v1545;
                v2381 = v4120;
            } else {
                let v1547 = (v1539 + v1538).sqrt();
                let v1549 = v393 * (v1547 + v1537);
                let v4113 = ((v4108 * (v2280 / (v2714 * v1547))) + v4106) * v393;
                v1550 = v1549;
                v2381 = v4113;
            }
            let v1551 = v1550 * v1078;
            let v1552 = v315 / v1551;
            let v4127 = ((Lanes([v2670, 0.0, 0.0, 0.0, 0.0])) - (((v2381 * v1078) + (v3553 * v1550)) * v1552)) / v1551;
            let v1553 = if v1552 < v22 { 1.0 } else { 0.0 };
            let v1554: f64;
            let v2382: Lanes<5>;
            if v1553 != 0.0 {
                v1554 = v22;
                v2382 = v4128;
            } else {
                v1554 = v1552;
                v2382 = v4127;
            }
            let v1555 = v152 * v1554;
            let v4129 = v2382 * v152;
            let v1556 = v25 * v105;
            let v1558 = v1557 - v2;
            let v4136 = ((Lanes([((v2438 * v25) * v1558), 0.0, 0.0])) + (v2323 * v1556)) + (Lanes([0.0, v2975[0], v2975[1]]));
            let v1561 = ((v1556 * v1558) + v670) / v1555;
            let v4137 = v4129 * v1561;
            let v4141 = ((Lanes([v4136[0], 0.0, v4136[1], v4136[2], 0.0, 0.0])) - (Lanes([v4137[0], v4137[1], 0.0, v4137[2], v4137[3], v4137[4]]))) / v1555;
            let v1562 = if v1085 > v0 { 1.0 } else { 0.0 };
            let v1776: f64;
            let v2383: Lanes<5>;
            if v1562 != 0.0 {
                let v1564 = if v1563 == v2 { 1.0 } else { 0.0 };
                let v1726: f64;
                let v2384: Lanes<5>;
                if v1564 != 0.0 {
                    let v1566 = if v659 < v1565 { 1.0 } else { 0.0 };
                    let v1727: f64;
                    let v2385: Lanes<5>;
                    if v1566 != 0.0 {
                        let v1569 = (-v1085) / v1568;
                        let v4320 = (v3571 * v2430) / v1568;
                        let v1570 = if v1569 < v699 { 1.0 } else { 0.0 };
                        let v1577: f64;
                        let v2386: Lanes<5>;
                        if v1570 != 0.0 {
                            let v1571 = v1569.exp();
                            let v4322 = v4320 * v1571;
                            v1577 = v1571;
                            v2386 = v4322;
                        } else {
                            let v1572 = v699.exp();
                            let v1575 = v1572 * (v2 + (v1569 - v699));
                            let v4321 = v4320 * v1572;
                            v1577 = v1575;
                            v2386 = v4321;
                        }
                        let v1576 = v1565 - v659;
                        let v1578 = v1576 * v1577;
                        let v4324 = (v2959 * v2430) * v1577;
                        let v4327 = (Lanes([0.0, 0.0, v4324[0], v4324[1], 0.0])) + (v2386 * v1576);
                        let v1580 = -v1579;
                        let v1582 = v1578.powf(v1581);
                        let v1583 = v1580 * v1582;
                        let v4336 = (Lanes([((v2309 * v2430) * v1582), 0.0, 0.0, 0.0, 0.0])) + ((v4327 * (v1581 * (v1578.powf((v1581 - v2280))))) * v1580);
                        let v1584 = if v1583 < v699 { 1.0 } else { 0.0 };
                        let v1593: f64;
                        let v2387: Lanes<5>;
                        if v1584 != 0.0 {
                            let v1585 = v1583.exp();
                            let v4338 = v4336 * v1585;
                            v1593 = v1585;
                            v2387 = v4338;
                        } else {
                            let v1586 = v699.exp();
                            let v1589 = v1586 * (v2 + (v1583 - v699));
                            let v4337 = v4336 * v1586;
                            v1593 = v1589;
                            v2387 = v4337;
                        }
                        let v1591 = v1590 / v1579;
                        let v1592 = v1591 * v1578;
                        let v1594 = v1592 * v1593;
                        let v4348 = (((Lanes([((((v2309 * v1591) * v2430) / v1579) * v1578), 0.0, 0.0, 0.0, 0.0])) + (v4327 * v1591)) * v1593) + (v2387 * v1592);
                        v1727 = v1594;
                        v2385 = v4348;
                    } else {
                        v1727 = v0;
                        v2385 = v4128;
                    }
                    v1726 = v1727;
                    v2384 = v2385;
                } else {
                    let v1595 = if v1563 == v25 { 1.0 } else { 0.0 };
                    let v1728: f64;
                    let v2388: Lanes<5>;
                    if v1595 != 0.0 {
                        let v1596 = if v659 < v735 { 1.0 } else { 0.0 };
                        let v1729: f64;
                        let v2389: Lanes<5>;
                        if v1596 != 0.0 {
                            let v1601 = (v25 * v1597) / (v1599 * v1599);
                            let v1602 = v735 - v659;
                            let v4197 = v3092 - v3091;
                            let v1603 = v1602 / v1006;
                            let v4199 = Lanes([v4197[0], v4197[1], v4197[2], 0.0]);
                            let v1606 = ((v25 * v1603) / v1601).sqrt();
                            let v4206 = ((((v4199 - (v2332 * v1603)) / v1006) * v25) / v1601) * (v2280 / (v2714 * v1606));
                            let v1608 = if v1607 == v0 { 1.0 } else { 0.0 };
                            let v1615: f64;
                            let v2390: Lanes<4>;
                            if v1608 != 0.0 {
                                v1615 = v1599;
                                v2390 = v3158;
                            } else {
                                let v1612 = v2 - (v393 * v1609);
                                let v4208 = (v2334 * v393) * v2430;
                                let v1613 = v1599 * v1612;
                                let v1614 = v1613 * v1612;
                                let v4212 = ((v4208 * v1599) * v1612) + (v4208 * v1613);
                                v1615 = v1614;
                                v2390 = v4212;
                            }
                            let v4216 = v4206 * v1606;
                            let v4218 = v2390 * v1615;
                            let v1620 = ((v1606 * v1606) + (v1615 * v1615)).sqrt();
                            let v1621 = (v1606 * v1615) / v1620;
                            let v4226 = (((v4206 * v1615) + (v2390 * v1606)) - ((((v4216 + v4216) + (v4218 + v4218)) * (v2280 / (v2714 * v1620))) * v1621)) / v1620;
                            let v1622 = v1602 / v1621;
                            let v4229 = (v4199 - (v4226 * v1622)) / v1621;
                            let v1623 = v393 * v1621;
                            let v4230 = v4226 * v393;
                            let v1624 = v1623 * v1601;
                            let v4231 = v4230 * v1601;
                            let v1626 = v1622 + (v1624 * v1006);
                            let v4235 = v4229 + ((v4231 * v1006) + (v2332 * v1624));
                            let v1653: f64;
                            let v2391: Lanes<5>;
                            if v1608 != 0.0 {
                                let v4270 = Lanes([v4235[0], 0.0, v4235[1], v4235[2], v4235[3]]);
                                v1653 = v1626;
                                v2391 = v4270;
                            } else {
                                let v1628 = v25 * v1627;
                                let v1636 = v821 * (v2 + (v1628 * (v2 + (v25 * v1609))));
                                let v1637 = v1085 / v1636;
                                let v4239 = (((v2334 * v25) * v1628) * v821) * v1637;
                                let v1638 = ((v2 + v1627) / (v2 + v1628)) - v1637;
                                let v4244 = v4231 * v1638;
                                let v1640 = v1622 - (v1624 * v1638);
                                let v4249 = (Lanes([v4229[0], 0.0, v4229[1], v4229[2], v4229[3]])) - ((Lanes([v4244[0], 0.0, v4244[1], v4244[2], v4244[3]])) + ((((v3571 - (Lanes([v4239[0], 0.0, v4239[1], v4239[2], v4239[3]]))) / v1636) * v2430) * v1624));
                                let v1641 = v1640 - v1626;
                                let v4250 = Lanes([v4235[0], 0.0, v4235[1], v4235[2], v4235[3]]);
                                let v4252 = (v4249 - v4250) * v1641;
                                let v1643 = v40 * v1622;
                                let v1644 = v1643 * v1622;
                                let v4261 = (((((v4229 * v40) * v1622) + (v4229 * v1643)) * v1645) + (v2335 * v1644)) / v821;
                                let v1650 = ((v1641 * v1641) + ((v1644 * v1645) / v821)).sqrt();
                                let v1652 = v393 * ((v1640 + v1626) + v1650);
                                let v4269 = ((v4249 + v4250) + (((v4252 + v4252) + (Lanes([v4261[0], 0.0, v4261[1], v4261[2], v4261[3]]))) * (v2280 / (v2714 * v1650)))) * v393;
                                v1653 = v1652;
                                v2391 = v4269;
                            }
                            let v1655 = (v1653 - v1622) / v1653;
                            let v4275 = ((v2391 - (Lanes([v4229[0], 0.0, v4229[1], v4229[2], v4229[3]]))) - (v2391 * v1655)) / v1653;
                            let v1658 = if (v1655.abs()) > v1657 { 1.0 } else { 0.0 };
                            let v1730: f64;
                            let v2392: Lanes<5>;
                            if v1658 != 0.0 {
                                let v1659 = v1623 / v1655;
                                let v4290 = ((Lanes([v4230[0], 0.0, v4230[1], v4230[2], v4230[3]])) - (v4275 * v1659)) / v1655;
                                let v1662 = v1660 / v1661;
                                let v1663 = v1662 * v1653;
                                let v1664 = v1663 * v1659;
                                let v1666 = (-v1661) / v1653;
                                let v4305 = ((Lanes([(v2313 * v2430), 0.0, 0.0, 0.0, 0.0])) - (v2391 * v1666)) / v1653;
                                let v1667 = v1666.exp();
                                let v1668 = v1615 / v1659;
                                let v1669 = v2 + v1668;
                                let v1671 = (v1666 * v1669).exp();
                                let v1672 = v1667 - v1671;
                                let v1673 = v1664 * v1672;
                                let v4318 = (((((Lanes([((((v2313 * v1662) * v2430) / v1661) * v1653), 0.0, 0.0, 0.0, 0.0])) + (v2391 * v1662)) * v1659) + (v4290 * v1663)) * v1672) + (((v4305 * v1667) - (((v4305 * v1669) + ((((Lanes([v2390[0], 0.0, v2390[1], v2390[2], v2390[3]])) - (v4290 * v1668)) / v1659) * v1666)) * v1671)) * v1664);
                                v1730 = v1673;
                                v2392 = v4318;
                            } else {
                                let v1674 = v1660 * v1615;
                                let v1676 = (-v1661) / v1653;
                                let v1677 = v1676.exp();
                                let v1678 = v1674 * v1677;
                                let v4283 = (v2390 * v1660) * v1677;
                                let v4286 = (Lanes([v4283[0], 0.0, v4283[1], v4283[2], v4283[3]])) + (((((Lanes([(v2313 * v2430), 0.0, 0.0, 0.0, 0.0])) - (v2391 * v1676)) / v1653) * v1677) * v1674);
                                v1730 = v1678;
                                v2392 = v4286;
                            }
                            v1729 = v1730;
                            v2389 = v2392;
                        } else {
                            v1729 = v0;
                            v2389 = v4128;
                        }
                        v1728 = v1729;
                        v2388 = v2389;
                    } else {
                        let v1679 = if v1563 == v152 { 1.0 } else { 0.0 };
                        let v1731: f64;
                        let v2393: Lanes<5>;
                        if v1679 != 0.0 {
                            let v1680 = if v659 < v1565 { 1.0 } else { 0.0 };
                            let v1732: f64;
                            let v2394: Lanes<5>;
                            if v1680 != 0.0 {
                                let v1681 = v1565 - v659;
                                let v4142 = v2959 * v2430;
                                let v1682 = v1681.powf(v1581);
                                let v1684 = v1683 + v1085;
                                let v1685 = v1085 / v1684;
                                let v1686 = v2 - v1685;
                                let v1688 = v1686.powf(v1687);
                                let v1689 = v1682 * v1688;
                                let v4155 = (v4142 * (v1581 * (v1681.powf((v1581 - v2280))))) * v1688;
                                let v4158 = (Lanes([0.0, 0.0, v4155[0], v4155[1], 0.0])) + (((((v3571 - (v3571 * v1685)) / v1684) * v2430) * (v1687 * (v1686.powf((v1687 - v2280))))) * v1682);
                                let v1690 = if v1607 == v0 { 1.0 } else { 0.0 };
                                let v1714: f64;
                                let v2395: Lanes<5>;
                                if v1690 != 0.0 {
                                    v1714 = v1689;
                                    v2395 = v4158;
                                } else {
                                    let v1693 = (v1085 - v1691) / v1683;
                                    let v4159 = v3571 / v1683;
                                    let v1696 = (v1693 - v2) / v1695;
                                    let v4160 = v4159 / v1695;
                                    let v1697 = if v1693 < v2 { 1.0 } else { 0.0 };
                                    let v1709: f64;
                                    let v2396: Lanes<5>;
                                    if v1697 != 0.0 {
                                        let v1698 = v1696.exp();
                                        let v1699 = v2 + v1698;
                                        let v4170 = ((v4160 * v1698) * (v2280 / v1699)) * v1695;
                                        let v1702 = v2 + (v1695 * (v1699.ln()));
                                        v1709 = v1702;
                                        v2396 = v4170;
                                    } else {
                                        let v1704 = (-v1696).exp();
                                        let v1705 = v2 + v1704;
                                        let v1708 = v1693 + (v1695 * (v1705.ln()));
                                        let v4166 = v4159 + ((((v4160 * v2430) * v1704) * (v2280 / v1705)) * v1695);
                                        v1709 = v1708;
                                        v2396 = v4166;
                                    }
                                    let v1711 = v1709.powf(v1710);
                                    let v1712 = v1689 * v1711;
                                    let v4177 = (v4158 * v1711) + ((v2396 * (v1710 * (v1709.powf((v1710 - v2280))))) * v1689);
                                    v1714 = v1712;
                                    v2395 = v4177;
                                }
                                let v1713 = -v1579;
                                let v1715 = v1713 * v1714;
                                let v4182 = (Lanes([((v2309 * v2430) * v1714), 0.0, 0.0, 0.0, 0.0])) + (v2395 * v1713);
                                let v1716 = if v1715 < v699 { 1.0 } else { 0.0 };
                                let v1724: f64;
                                let v2397: Lanes<5>;
                                if v1716 != 0.0 {
                                    let v1717 = v1715.exp();
                                    let v4184 = v4182 * v1717;
                                    v1724 = v1717;
                                    v2397 = v4184;
                                } else {
                                    let v1718 = v699.exp();
                                    let v1721 = v1718 * (v2 + (v1715 - v699));
                                    let v4183 = v4182 * v1718;
                                    v1724 = v1721;
                                    v2397 = v4183;
                                }
                                let v1722 = v1590 / v1579;
                                let v1723 = v1722 * v1681;
                                let v4189 = v4142 * v1722;
                                let v1725 = v1723 * v1724;
                                let v4193 = ((Lanes([((((v2309 * v1722) * v2430) / v1579) * v1681), 0.0, 0.0])) + (Lanes([0.0, v4189[0], v4189[1]]))) * v1724;
                                let v4196 = (Lanes([v4193[0], 0.0, v4193[1], v4193[2], 0.0])) + (v2397 * v1723);
                                v1732 = v1725;
                                v2394 = v4196;
                            } else {
                                v1732 = v0;
                                v2394 = v4128;
                            }
                            v1731 = v1732;
                            v2393 = v2394;
                        } else {
                            v1731 = v0;
                            v2393 = v4128;
                        }
                        v1728 = v1731;
                        v2388 = v2393;
                    }
                    v1726 = v1728;
                    v2384 = v2388;
                }
                let v1733 = if v1726 > v0 { 1.0 } else { 0.0 };
                let v1777: f64;
                let v2398: Lanes<5>;
                if v1733 != 0.0 {
                    let v1735 = if v1734 == v2 { 1.0 } else { 0.0 };
                    let v1778: f64;
                    let v2399: Lanes<5>;
                    if v1735 != 0.0 {
                        let v1737 = v1736 + v1555;
                        let v4353 = (Lanes([v2304, 0.0, 0.0, 0.0, 0.0])) + v4129;
                        let v1738 = v1085 * v1737;
                        let v1739 = v105 / v1738;
                        let v1740 = v1079 / v418;
                        let v1744 = v1743 / v1737;
                        let v1745 = (v1739 + (v1740 * v469)) + v1744;
                        let v4374 = ((((Lanes([v2438, 0.0, 0.0, 0.0, 0.0])) - (((v3571 * v1737) + (v4353 * v1085)) * v1739)) / v1738) + ((((v3556 - (Lanes([(v2740 * v1740), 0.0, 0.0, 0.0, 0.0]))) / v418) * v469) + (Lanes([(v2774 * v1740), 0.0, 0.0, 0.0, 0.0])))) + (((Lanes([v2303, 0.0, 0.0, 0.0, 0.0])) - (v4353 * v1744)) / v1737);
                        let v1746 = if v1563 == v152 { 1.0 } else { 0.0 };
                        let v1779: f64;
                        let v2400: Lanes<5>;
                        if v1746 != 0.0 {
                            let v1748 = (v1726 - v1745) / v1481;
                            let v4386 = (v2384 - v4374) / v1481;
                            let v1749 = if v1726 < v1745 { 1.0 } else { 0.0 };
                            let v1761: f64;
                            let v2401: Lanes<5>;
                            if v1749 != 0.0 {
                                let v1750 = v1748.exp();
                                let v1751 = v2 + v1750;
                                let v1754 = v1726 - (v1481 * (v1751.ln()));
                                let v4397 = v2384 - (((v4386 * v1750) * (v2280 / v1751)) * v1481);
                                v1761 = v1754;
                                v2401 = v4397;
                            } else {
                                let v1756 = (-v1748).exp();
                                let v1757 = v2 + v1756;
                                let v1760 = v1745 - (v1481 * (v1757.ln()));
                                let v4392 = v4374 - ((((v4386 * v2430) * v1756) * (v2280 / v1757)) * v1481);
                                v1761 = v1760;
                                v2401 = v4392;
                            }
                            let v1762 = v1085 * v1761;
                            let v4400 = (v3571 * v1761) + (v2401 * v1085);
                            v1779 = v1762;
                            v2400 = v4400;
                        } else {
                            let v1763 = v1085 * v1726;
                            let v1765 = v1726 + v1745;
                            let v1766 = (v1763 * v1745) / v1765;
                            let v4384 = (((((v3571 * v1726) + (v2384 * v1085)) * v1745) + (v4374 * v1763)) - ((v2384 + v4374) * v1766)) / v1765;
                            v1779 = v1766;
                            v2400 = v4384;
                        }
                        v1778 = v1779;
                        v2399 = v2400;
                    } else {
                        let v1767 = v1085 * v1726;
                        let v4351 = (v3571 * v1726) + (v2384 * v1085);
                        v1778 = v1767;
                        v2399 = v4351;
                    }
                    v1777 = v1778;
                    v2398 = v2399;
                } else {
                    v1777 = v0;
                    v2398 = v4128;
                }
                v1776 = v1777;
                v2383 = v2398;
            } else {
                v1776 = v0;
                v2383 = v4128;
            }
            let v1768 = if v1033 > v0 { 1.0 } else { 0.0 };
            let v1770: f64;
            let v2402: Lanes<4>;
            if v1768 != 0.0 {
                let v1769 = v105 * v3472;
                let v4407 = (Lanes([(v2438 * v3472), 0.0, 0.0, 0.0])) + ((v2333 * (v2280 / v1033)) * v105);
                v1770 = v1769;
                v2402 = v4407;
            } else {
                let v4401 = Lanes([0.0, v2963[0], 0.0, v2963[1]]);
                v1770 = v662;
                v2402 = v4401;
            }
            let v1816: f64;
            let v2403: Lanes<3>;
            if v479 != 0.0 {
                let v4409 = Lanes([v2959[0], v2959[1], 0.0]);
                v1816 = v659;
                v2403 = v4409;
            } else {
                let v4408 = Lanes([v2963[0], 0.0, v2963[1]]);
                v1816 = v662;
                v2403 = v4408;
            }
            let v1771 = v665 - v1770;
            let v1773 = v1770 - v659;
            let v4420 = (v3132 * v1773) + ((v2402 - (Lanes([0.0, v2959[0], v2959[1], 0.0]))) * v788);
            let v4424 = v2402 * v1776;
            let v4427 = (((v3571 * v1771) + (((Lanes([0.0, v2967[0], v2967[1], 0.0, 0.0])) - (Lanes([v2402[0], 0.0, v2402[1], v2402[2], v2402[3]]))) * v1085)) + (Lanes([v4420[0], 0.0, v4420[1], v4420[2], v4420[3]]))) - ((v2383 * v1770) + (Lanes([v4424[0], 0.0, v4424[1], v4424[2], v4424[3]])));
            let v4428 = v2983 * v675;
            let v4429 = v4428 + v4428;
            let v1783 = (v675 * v675) / v1743;
            let v4434 = ((Lanes([v4429[0], 0.0, v4429[1]])) - (Lanes([0.0, (v2303 * v1783), 0.0]))) / v1743;
            let v4437 = (Lanes([0.0, v4427[0], v4427[1], v4427[2], v4427[3], v4427[4]])) + (Lanes([v4434[0], v4434[1], v4434[2], 0.0, 0.0, 0.0]));
            let v1785 = v696 * v696;
            let v4438 = v3022 * v696;
            let v4440 = (v4438 + v4438) * v1786;
            let v4444 = (Lanes([v4440[0], v4440[1], 0.0, v4440[2], v4440[3], v4440[4], v4440[5], v4440[6], v4440[7]])) + (Lanes([0.0, 0.0, (v2314 * v1785), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]));
            let v1790 = v689 * v689;
            let v4448 = v3003 * v689;
            let v4450 = (v4448 + v4448) * v1791;
            let v4454 = (Lanes([0.0, v4450[0], v4450[1]])) + (Lanes([(v2316 * v1790), 0.0, 0.0]));
            let v1795 = v686 * v686;
            let v4457 = v2999 * v686;
            let v4459 = (v4457 + v4457) * v1796;
            let v4463 = (Lanes([0.0, v4459[0], v4459[1]])) + (Lanes([(v2318 * v1795), 0.0, 0.0]));
            let v4466 = v2987 * v678;
            let v4467 = v4466 + v4466;
            let v1801 = (v678 * v678) / v1736;
            let v4472 = ((Lanes([v4467[0], 0.0, v4467[1]])) - (Lanes([0.0, (v2304 * v1801), 0.0]))) / v1736;
            let v4476 = v2975 * v1561;
            let v4478 = (v4141 * v670) + (Lanes([0.0, 0.0, v4476[0], v4476[1], 0.0, 0.0]));
            let v4483 = v2967 * v16;
            let v1813 = ((((v1805 + v1237) + (v16 * v665)) - v1810) + v1131) + v1110;
            let v4491 = ((((v2354 + (Lanes([v3734[0], v3734[1], v3734[2], 0.0, 0.0]))) + (Lanes([0.0, v4483[0], v4483[1], 0.0, 0.0]))) - (Lanes([v2365[0], v2365[1], v2365[2], 0.0, 0.0]))) + (Lanes([0.0, v3609[0], v3609[1], 0.0, 0.0]))) + (Lanes([v3590[0], v3590[1], v3590[2], 0.0, 0.0]));
            let v4493 = v2967 * v1813;
            let v4495 = (v4491 * v665) + (Lanes([0.0, v4493[0], v4493[1], 0.0, 0.0]));
            let v4498 = v4083 * v1816;
            let v4499 = v2403 * v1528;
            let v4502 = (Lanes([v4498[0], v4498[1], v4498[2], v4498[3], 0.0])) + (Lanes([0.0, 0.0, v4499[0], v4499[1], v4499[2]]));
            let v1821 = (v1819 + v1247) + v1267;
            let v4506 = (v2359 + v3741) + v3755;
            let v4508 = v2971 * v1821;
            let v4510 = (v4506 * v668) + (Lanes([0.0, v4508[0], v4508[1]]));
            let v1825 = v16 * v692;
            let v4514 = v3012 * v16;
            let v1826 = (v1530 + v1531) + v1825;
            let v4515 = Lanes([0.0, v4514[0], v4514[1], v4514[2], v4514[3], v4514[4]]);
            let v4518 = v3012 * v1826;
            let v4520 = (((v4087 + v4091) + v4515) * v692) + (Lanes([0.0, v4518[0], v4518[1], v4518[2], v4518[3], v4518[4]]));
            let v4524 = v3024 * v1533;
            let v4526 = (v4095 * v697) + (Lanes([v4524[0], v4524[1], 0.0, v4524[2], v4524[3], v4524[4], v4524[5], v4524[6], v4524[7]]));
            let v1830 = (((((((((((((v1085 * v1771) + (v788 * v1773)) - (v1776 * v1770)) + v1783) + (v1785 * v1786)) + (v1790 * v1791)) + (v1795 * v1796)) + v1801) + (v1561 * v670)) + (v1813 * v665)) - (v1528 * v1816)) + (v1821 * v668)) + (v1826 * v692)) + (v1533 * v697);
            let v4528 = ((((((((((Lanes([0.0, 0.0, v4437[0], v4437[1], v4437[2], 0.0, v4437[3], v4437[4], v4437[5], 0.0, 0.0])) + (Lanes([v4444[0], v4444[1], 0.0, v4444[2], 0.0, v4444[3], v4444[4], v4444[5], v4444[6], v4444[7], v4444[8]]))) + (Lanes([0.0, 0.0, 0.0, v4454[0], 0.0, 0.0, 0.0, 0.0, 0.0, v4454[1], v4454[2]]))) + (Lanes([0.0, 0.0, 0.0, v4463[0], 0.0, 0.0, 0.0, v4463[1], 0.0, 0.0, v4463[2]]))) + (Lanes([0.0, v4472[0], 0.0, v4472[1], 0.0, v4472[2], 0.0, 0.0, 0.0, 0.0, 0.0]))) + (Lanes([0.0, 0.0, 0.0, v4478[0], v4478[1], v4478[2], v4478[3], v4478[4], v4478[5], 0.0, 0.0]))) + (Lanes([0.0, 0.0, 0.0, v4495[0], v4495[1], 0.0, v4495[2], v4495[3], v4495[4], 0.0, 0.0]))) - (Lanes([0.0, 0.0, 0.0, v4502[0], 0.0, v4502[1], v4502[2], v4502[3], v4502[4], 0.0, 0.0]))) + (Lanes([0.0, 0.0, 0.0, v4510[0], v4510[1], v4510[2], 0.0, 0.0, 0.0, 0.0, 0.0]))) + (Lanes([0.0, 0.0, 0.0, v4520[0], 0.0, v4520[1], v4520[2], v4520[3], v4520[4], 0.0, v4520[5]]))) + (Lanes([v4526[0], v4526[1], 0.0, v4526[2], 0.0, v4526[3], v4526[4], v4526[5], v4526[6], v4526[7], v4526[8]]));
            let v1832 = v2 - v1831;
            let v1833 = v1832 * v291;
            let v4529 = v2651 * v1832;
            let v1834 = v1833 * v974;
            let v4533 = (Lanes([(v4529 * v974), 0.0, 0.0])) + (v3372 * v1833);
            let v4534 = Lanes([0.0, v2971[0], v2971[1]]);
            let v4535 = Lanes([v3329, 0.0, 0.0]);
            let v1836 = (v668 - v948) / v949;
            let v4540 = ((v4534 - v4535) - (Lanes([(v3330 * v1836), 0.0, 0.0]))) / v949;
            let v1837 = if v668 < v948 { 1.0 } else { 0.0 };
            let v1850: f64;
            let v2404: Lanes<3>;
            if v1837 != 0.0 {
                let v1838 = v1836.exp();
                let v1839 = v2 + v1838;
                let v1840 = v1839.ln();
                let v1842 = v668 - (v949 * v1840);
                let v4557 = v4534 - ((Lanes([(v3330 * v1840), 0.0, 0.0])) + (((v4540 * v1838) * (v2280 / v1839)) * v949));
                v1850 = v1842;
                v2404 = v4557;
            } else {
                let v1844 = (-v1836).exp();
                let v1845 = v2 + v1844;
                let v1846 = v1845.ln();
                let v1848 = v948 - (v949 * v1846);
                let v4549 = v4535 - ((Lanes([(v3330 * v1846), 0.0, 0.0])) + ((((v4540 * v2430) * v1844) * (v2280 / v1845)) * v949));
                v1850 = v1848;
                v2404 = v4549;
            }
            let v1849 = v1831 * v291;
            let v1852 = v2 - (v1850 * v283);
            let v1854 = v2 - (v1852.powf(v967));
            let v1858 = (v969 * v1854) + (v152 * (v668 - v1850));
            let v1859 = v1849 * v1858;
            let v4578 = (Lanes([((v2651 * v1831) * v1858), 0.0, 0.0])) + ((((Lanes([(v3364 * v1854), 0.0, 0.0])) + ((((((v2404 * v283) + (Lanes([(v2637 * v1850), 0.0, 0.0]))) * v2430) * (v967 * (v1852.powf(v3360)))) * v2430) * v969)) + ((v4534 - v2404) * v152)) * v1849);
            let v1861 = v1860 * v301;
            let v1862 = v1861 * v1024;
            let v4583 = (Lanes([((v2662 * v1860) * v1024), 0.0, 0.0, 0.0])) + (v3450 * v1861);
            let v1863 = v608 * v423;
            let v4586 = (v2929 * v423) + (v2743 * v608);
            let v1864 = v393 * v1863;
            let v4587 = v4586 * v393;
            let v1865 = v1864 * v1032;
            let v1866 = v1865 * v1550;
            let v4592 = ((Lanes([(v4587 * v1032), 0.0, 0.0])) + (v3464 * v1864)) * v1550;
            let v4595 = (Lanes([v4592[0], v4592[1], v4592[2], 0.0, 0.0])) + (v2381 * v1865);
            let v1867 = v1864 * v1041;
            let v1868 = v1867 * v1550;
            let v4600 = ((Lanes([(v4587 * v1041), 0.0, 0.0, 0.0])) + (v3486 * v1864)) * v1550;
            let v4603 = (Lanes([v4600[0], 0.0, v4600[1], v4600[2], v4600[3]])) + (v2381 * v1867);
            let v1869 = v40 * v294;
            let v4604 = v2300 * v40;
            let v4605 = Lanes([v3388, 0.0, 0.0, 0.0, 0.0, 0.0]);
            let v1871 = (v692 - v987) / v1869;
            let v4610 = ((v3073 - v4605) - (Lanes([(v4604 * v1871), 0.0, 0.0, 0.0, 0.0, 0.0]))) / v1869;
            let v1872 = if v692 < v987 { 1.0 } else { 0.0 };
            let v1884: f64;
            let v2405: Lanes<6>;
            if v1872 != 0.0 {
                let v1873 = v1871.exp();
                let v1874 = v2 + v1873;
                let v1875 = v1874.ln();
                let v1877 = v692 - (v1869 * v1875);
                let v4627 = v3073 - ((Lanes([(v4604 * v1875), 0.0, 0.0, 0.0, 0.0, 0.0])) + (((v4610 * v1873) * (v2280 / v1874)) * v1869));
                v1884 = v1877;
                v2405 = v4627;
            } else {
                let v1879 = (-v1871).exp();
                let v1880 = v2 + v1879;
                let v1881 = v1880.ln();
                let v1883 = v987 - (v1869 * v1881);
                let v4619 = v4605 - ((Lanes([(v4604 * v1881), 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((v4610 * v2430) * v1879) * (v2280 / v1880)) * v1869));
                v1884 = v1883;
                v2405 = v4619;
            }
            let v1885 = v1884 / v294;
            let v1886 = v2 - v1885;
            let v1888 = v2 - (v1886.powf(v1009));
            let v1890 = v692 - v1884;
            let v1892 = (v1010 * v1888) + (v982 * v1890);
            let v4652 = v3012 * v302;
            let v1895 = (v981 * v1892) + (v302 * v692);
            let v1897 = v2 - v1860;
            let v1899 = ((v301 * v1895) * v1897) * v9;
            let v4662 = (((Lanes([(v2662 * v1895), 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([(v3377 * v1892), 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([(v3413 * v1888), 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((((v2405 - (Lanes([(v2300 * v1885), 0.0, 0.0, 0.0, 0.0, 0.0]))) / v294) * v2430) * (v1009 * (v1886.powf(v3419)))) * v2430) * v1010)) + ((Lanes([(v3380 * v1890), 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v3073 - v2405) * v982))) * v981)) + ((Lanes([(v2663 * v692), 0.0, 0.0, 0.0, 0.0, 0.0])) + (Lanes([0.0, v4652[0], v4652[1], v4652[2], v4652[3], v4652[4]])))) * v301)) * v1897) * v9;
            let v4663 = Lanes([0.0, 0.0, v3388, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
            let v1901 = (v697 - v987) / v1869;
            let v4668 = ((v3064 - v4663) - (Lanes([0.0, 0.0, (v4604 * v1901), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) / v1869;
            let v1902 = if v697 < v987 { 1.0 } else { 0.0 };
            let v1914: f64;
            let v2406: Lanes<9>;
            if v1902 != 0.0 {
                let v1903 = v1901.exp();
                let v1904 = v2 + v1903;
                let v1905 = v1904.ln();
                let v1907 = v697 - (v1869 * v1905);
                let v4685 = v3064 - ((Lanes([0.0, 0.0, (v4604 * v1905), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (((v4668 * v1903) * (v2280 / v1904)) * v1869));
                v1914 = v1907;
                v2406 = v4685;
            } else {
                let v1909 = (-v1901).exp();
                let v1910 = v2 + v1909;
                let v1911 = v1910.ln();
                let v1913 = v987 - (v1869 * v1911);
                let v4677 = v4663 - ((Lanes([0.0, 0.0, (v4604 * v1911), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((v4668 * v2430) * v1909) * (v2280 / v1910)) * v1869));
                v1914 = v1913;
                v2406 = v4677;
            }
            let v1915 = v1914 / v294;
            let v1916 = v2 - v1915;
            let v1918 = v2 - (v1916.powf(v1009));
            let v1920 = v697 - v1914;
            let v1922 = (v1010 * v1918) + (v982 * v1920);
            let v4710 = v3024 * v302;
            let v1925 = (v981 * v1922) + (v302 * v697);
            let v1928 = ((v301 * v1925) * v1897) * v8;
            let v4720 = (((Lanes([0.0, 0.0, (v2662 * v1925), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([0.0, 0.0, (v3377 * v1922), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([0.0, 0.0, (v3413 * v1918), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((((v2406 - (Lanes([0.0, 0.0, (v2300 * v1915), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) / v294) * v2430) * (v1009 * (v1916.powf(v3419)))) * v2430) * v1010)) + ((Lanes([0.0, 0.0, (v3380 * v1920), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v3064 - v2406) * v982))) * v981)) + ((Lanes([0.0, 0.0, (v2663 * v697), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (Lanes([v4710[0], v4710[1], 0.0, v4710[2], v4710[3], v4710[4], v4710[5], v4710[6], v4710[7]])))) * v301)) * v1897) * v8;
            let v1929 = v602 * v423;
            let v1930 = v418 / v423;
            let v1932 = v2 / v1931;
            let v1933 = v1930.powf(v1932);
            let v1934 = v1929 * v1933;
            let v4733 = (((v2926 * v423) + (v2743 * v602)) * v1933) + ((((v2740 - (v2743 * v1930)) / v423) * (v1932 * (v1930.powf((v1932 - v2280))))) * v1929);
            let v1935 = v1931 * v105;
            let v4734 = v2438 * v1931;
            let v1936 = v665 / v1935;
            let v4738 = (v3331 - (Lanes([(v4734 * v1936), 0.0, 0.0]))) / v1935;
            let v1937 = if v1936 < v699 { 1.0 } else { 0.0 };
            let v1943: f64;
            let v2407: Lanes<3>;
            if v1937 != 0.0 {
                let v1938 = v1936.exp();
                let v4740 = v4738 * v1938;
                v1943 = v1938;
                v2407 = v4740;
            } else {
                let v1939 = v699.exp();
                let v1942 = v1939 * (v2 + (v1936 - v699));
                let v4739 = v4738 * v1939;
                v1943 = v1942;
                v2407 = v4739;
            }
            let v1944 = v1934 * v1943;
            let v4744 = (Lanes([(v4733 * v1943), 0.0, 0.0])) + (v2407 * v1934);
            let v1945 = v404 * v613;
            let v1947 = (v1945 * v105) / v338;
            let v4751 = ((((v2932 * v404) * v105) + (v2438 * v1945)) - (v2683 * v1947)) / v338;
            let v1948 = v393 * v1947;
            let v1949 = v1948 * v1609;
            let v1952 = (v1950 + v867) + v25;
            let v1953 = v1949 * v1952;
            let v4761 = (((Lanes([((v4751 * v393) * v1609), 0.0, 0.0, 0.0])) + (v2334 * v1948)) * v1952) + ((v2336 + (Lanes([v2329[0], v2329[1], v2329[2], 0.0]))) * v1949);
            let v1955 = if v1954 == v0 { 1.0 } else { 0.0 };
            let v1985: f64;
            let v2408: Lanes<6>;
            if v1955 != 0.0 {
                let v1956 = v618 * v393;
                let v1959 = (v1863 * v1421) + (v1947 * v1425);
                let v1961 = (v1956 * v1959) / v615;
                let v4802 = (((Lanes([((v2935 * v393) * v1959), 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([(v4586 * v1421), 0.0, 0.0, 0.0, 0.0, 0.0])) + (v3961 * v1863)) + ((Lanes([(v4751 * v1425), 0.0, 0.0, 0.0, 0.0, 0.0])) + (v3967 * v1947))) * v1956)) - (Lanes([(v2933 * v1961), 0.0, 0.0, 0.0, 0.0, 0.0]))) / v615;
                v1985 = v1961;
                v2408 = v4802;
            } else {
                let v1965 = (v692 - v1962) / v1964;
                let v1966 = v1965 * v107;
                let v4768 = (((v3073 - (Lanes([v2299, 0.0, 0.0, 0.0, 0.0, 0.0]))) / v1964) * v107) + (Lanes([(v2441 * v1965), 0.0, 0.0, 0.0, 0.0, 0.0]));
                let v1967 = if v1966 < v699 { 1.0 } else { 0.0 };
                let v1975: f64;
                let v2409: Lanes<6>;
                if v1967 != 0.0 {
                    let v1968 = v1966.exp();
                    let v4770 = v4768 * v1968;
                    v1975 = v1968;
                    v2409 = v4770;
                } else {
                    let v1969 = v699.exp();
                    let v1972 = v1969 * (v2 + (v1966 - v699));
                    let v4769 = v4768 * v1969;
                    v1975 = v1972;
                    v2409 = v4769;
                }
                let v1973 = v1426 * v624;
                let v1978 = (v2 + (v404 * v1975)).sqrt();
                let v1979 = v2 + v1978;
                let v1980 = (v1973 * v1413) / v1979;
                let v4784 = (((Lanes([(((v3968 * v624) + (v2938 * v1426)) * v1413), 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2322 * v1973)) - (((v2409 * v404) * (v2280 / (v2714 * v1978))) * v1980)) / v1979;
                v1985 = v1980;
                v2408 = v4784;
            }
            let v1984 = if (if (if v1436 == v2 { 1.0 } else { 0.0 }) != 0.0 || (if v1436 == v152 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v1438 != 0.0 { 1.0 } else { 0.0 };
            let v2159: f64;
            let v2168: f64;
            let v2410: Lanes<9>;
            let v2411: Lanes<6>;
            if v1984 != 0.0 {
                let v1986 = v1985 * v9;
                let v4803 = v2408 * v9;
                let v2025: f64;
                let v2412: Lanes<9>;
                if v1955 != 0.0 {
                    let v1987 = v1026 * v1443;
                    let v4830 = (Lanes([0.0, 0.0, (v3454 * v1443), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2324 * v1026);
                    let v1990 = (v2 + v1987).sqrt();
                    let v1991 = v2 + v1990;
                    let v1992 = (v1987 - v1026) / v1991;
                    let v1994 = v404 * v1993;
                    let v4839 = v2325 * v404;
                    let v1996 = (v2 + v1994).sqrt();
                    let v1997 = v2 + v1996;
                    let v1998 = v1994 / v1997;
                    let v1999 = v393 * v8;
                    let v2000 = v1999 * v618;
                    let v2003 = (v1863 * v1992) + (v1947 * v1998);
                    let v2005 = (v2000 * v2003) / v615;
                    let v4863 = (((Lanes([0.0, 0.0, ((v2935 * v1999) * v2003), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([0.0, 0.0, (v4586 * v1992), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((v4830 - (Lanes([0.0, 0.0, v3454, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) - ((v4830 * (v2280 / (v2714 * v1990))) * v1992)) / v1991) * v1863)) + ((Lanes([0.0, 0.0, (v4751 * v1998), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (((v4839 - ((v4839 * (v2280 / (v2714 * v1996))) * v1998)) / v1997) * v1947))) * v2000)) - (Lanes([0.0, 0.0, (v2933 * v2005), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) / v615;
                    v2025 = v2005;
                    v2412 = v4863;
                } else {
                    let v2006 = v697 - v1962;
                    let v2007 = v2006 * v107;
                    let v4809 = ((v3064 - (Lanes([0.0, 0.0, v2299, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) * v107) + (Lanes([0.0, 0.0, (v2441 * v2006), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]));
                    let v2008 = if v2007 < v699 { 1.0 } else { 0.0 };
                    let v2018: f64;
                    let v2413: Lanes<9>;
                    if v2008 != 0.0 {
                        let v2009 = v2007.exp();
                        let v4811 = v4809 * v2009;
                        v2018 = v2009;
                        v2413 = v4811;
                    } else {
                        let v2010 = v699.exp();
                        let v2013 = v2010 * (v2 + (v2007 - v699));
                        let v4810 = v4809 * v2010;
                        v2018 = v2013;
                        v2413 = v4810;
                    }
                    let v2014 = v25 * v8;
                    let v2015 = v2014 * v510;
                    let v2016 = v2015 * v624;
                    let v2021 = (v2 + (v404 * v2018)).sqrt();
                    let v2022 = v2 + v2021;
                    let v2023 = (v2016 * v1443) / v2022;
                    let v4826 = (((Lanes([0.0, 0.0, ((((v2801 * v2014) * v624) + (v2938 * v2015)) * v1443), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2324 * v2016)) - (((v2413 * v404) * (v2280 / (v2714 * v2021))) * v2023)) / v2022;
                    v2025 = v2023;
                    v2412 = v4826;
                }
                let v2026 = v2024 * v2025;
                let v4866 = (v2375 * v2025) + (v2412 * v2024);
                v2159 = v2026;
                v2168 = v1986;
                v2410 = v4866;
                v2411 = v4803;
            } else {
                v2159 = v0;
                v2168 = v1985;
                v2410 = v3987;
                v2411 = v2408;
            }
            let v2028 = if v2027 == v2 { 1.0 } else { 0.0 };
            let v2124: f64;
            let v2126: f64;
            let v2134: f64;
            let v2140: f64;
            let v2414: Lanes<5>;
            let v2415: Lanes<3>;
            let v2416: Lanes<5>;
            let v2417: Lanes<6>;
            if v2028 != 0.0 {
                let v2029 = -v26;
                let v4871 = v3359 * (v2029 * (v966.powf((v2029 - v2280))));
                let v2031 = (v966.powf(v2029)) - v152;
                let v2032 = if v951 < v0 { 1.0 } else { 0.0 };
                let v2040: f64;
                let v2418: Lanes<3>;
                if v2032 != 0.0 {
                    let v2033 = v951.exp();
                    let v2034 = v2 + v2033;
                    let v2035 = v2 / v2034;
                    let v4880 = (((v3337 * v2033) * v2035) * v2430) / v2034;
                    v2040 = v2035;
                    v2418 = v4880;
                } else {
                    let v2037 = (-v951).exp();
                    let v4873 = (v3337 * v2430) * v2037;
                    let v2038 = v2 + v2037;
                    let v2039 = v2037 / v2038;
                    let v4876 = (v4873 - (v4873 * v2039)) / v2038;
                    v2040 = v2039;
                    v2418 = v4876;
                }
                let v2042 = (v2031 * v2040) + v152;
                let v4887 = (Lanes([(v4529 * v2042), 0.0, 0.0])) + (((v4871 * v2040) + (v2418 * v2031)) * v1833);
                let v2045 = (v1028 * v107) / v410;
                let v2046 = v393 / v1030;
                let v2047 = v2045 * v2046;
                let v2048 = v1864 * v1550;
                let v4907 = ((((((v3458 * v107) + (Lanes([(v2441 * v1028), 0.0, 0.0]))) - (Lanes([(v2305 * v2045), 0.0, 0.0]))) / v410) * v2046) + ((((v3461 * v2046) * v2430) / v1030) * v2045)) * v2048;
                let v2050 = v1944 / v1935;
                let v4913 = (v4744 - (Lanes([(v4734 * v2050), 0.0, 0.0]))) / v1935;
                let v2051 = v806 * v670;
                let v2053 = ((v1833 * v2042) + (v2048 * v2047)) + v2050;
                let v2054 = v2051 * v2053;
                let v4919 = (v2975 * v806) * v2053;
                let v4920 = (((Lanes([v4887[0], v4887[1], v4887[2], 0.0, 0.0])) + ((((Lanes([(v4587 * v1550), 0.0, 0.0, 0.0, 0.0])) + (v2381 * v1864)) * v2047) + (Lanes([v4907[0], v4907[1], v4907[2], 0.0, 0.0])))) + (Lanes([v4913[0], v4913[1], v4913[2], 0.0, 0.0]))) * v2051;
                let v4923 = (Lanes([0.0, 0.0, v4919[0], v4919[1], 0.0, 0.0])) + (Lanes([v4920[0], v4920[1], 0.0, v4920[2], v4920[3], v4920[4]]));
                let v2056 = v2 - v2055;
                let v2057 = v2056 * v1944;
                let v4924 = v4744 * v2056;
                let v4925 = v4744 * v2055;
                let v2059 = v1866 + (v2055 * v1944);
                let v4927 = v4595 + (Lanes([v4925[0], v4925[1], v4925[2], 0.0, 0.0]));
                let v2062 = (v2060 * v2059) + v1868;
                let v4929 = (v4927 * v2060) + v4603;
                let v2063 = v2 - v2060;
                let v2064 = v2063 * v2059;
                let v4930 = v4927 * v2063;
                v2124 = v2064;
                v2126 = v2057;
                v2134 = v2062;
                v2140 = v2054;
                v2414 = v4930;
                v2415 = v4924;
                v2416 = v4929;
                v2417 = v4923;
            } else {
                v2124 = v1866;
                v2126 = v1944;
                v2134 = v1868;
                v2140 = v0;
                v2414 = v4595;
                v2415 = v4744;
                v2416 = v4603;
                v2417 = v4867;
            }
            let v2066 = (v1 * v788) * v21;
            let v4932 = (v3132 * v1) * v21;
            let v2068 = (v1 * v1085) * v21;
            let v4934 = (v3571 * v1) * v21;
            let v2070 = (v1 * v1821) * v21;
            let v4936 = (v4506 * v1) * v21;
            let v2072 = (v1 * v1813) * v21;
            let v4938 = (v4491 * v1) * v21;
            let v2254: f64;
            let v2255: f64;
            let v2419: Lanes<4>;
            let v2420: Lanes<4>;
            if v479 != 0.0 {
                let v2075 = (v1 * (-v1528)) * v21;
                let v4945 = ((v4083 * v2430) * v1) * v21;
                v2254 = v2075;
                v2255 = v0;
                v2419 = v4945;
                v2420 = v4942;
            } else {
                let v2078 = (v1 * (-v1528)) * v21;
                let v4941 = ((v4083 * v2430) * v1) * v21;
                v2254 = v0;
                v2255 = v2078;
                v2419 = v4942;
                v2420 = v4941;
            }
            let v2080 = (v1 * v1561) * v21;
            let v4947 = (v4141 * v1) * v21;
            let v2084 = (v1 * (v2081 * v1776)) * v21;
            let v4950 = ((v2383 * v2081) * v1) * v21;
            let v4951 = v2983 * v1;
            let v2086 = (v1 * v675) / v1743;
            let v2087 = v2086 * v21;
            let v4957 = (((Lanes([v4951[0], 0.0, v4951[1]])) - (Lanes([0.0, (v2303 * v2086), 0.0]))) / v1743) * v21;
            let v4958 = v2987 * v1;
            let v2089 = (v1 * v678) / v1736;
            let v2090 = v2089 * v21;
            let v4964 = (((Lanes([v4958[0], 0.0, v4958[1]])) - (Lanes([0.0, (v2304 * v2089), 0.0]))) / v1736) * v21;
            let v2092 = v2091 * v89;
            let v4965 = v2281 * v2091;
            let v2094 = (ddt(12393, v2092)) * v21;
            let v4968 = (v4965 * v4966) * v21;
            let v2270 = v2092 * v21;
            let v4969 = v4965 * v21;
            let v2095 = v2 - v644;
            let v2096 = if v642 > v22 { 1.0 } else { 0.0 };
            let v2118: f64;
            let v2421: f64;
            if v2096 != 0.0 {
                let v2098 = if v2097 == v0 { 1.0 } else { 0.0 };
                let v2119: f64;
                let v2422: f64;
                if v2098 != 0.0 {
                    let v2100 = (v89 / v646) * v21;
                    let v4982 = (v2281 / v646) * v21;
                    v2119 = v2100;
                    v2422 = v4982;
                } else {
                    let v2102 = if (v2095.abs()) < v1481 { 1.0 } else { 0.0 };
                    let v2120: f64;
                    let v2423: f64;
                    if v2102 != 0.0 {
                        let v2104 = (v15 / v646) * v21;
                        let v2106 = v2 + (v89 / v15);
                        let v2108 = v2104 * (v2106.ln());
                        let v4980 = ((v2281 / v15) * (v2280 / v2106)) * v2104;
                        v2120 = v2108;
                        v2423 = v4980;
                    } else {
                        let v2111 = (v15 / (v2095 * v646)) * v21;
                        let v2113 = v2 + (v89 / v15);
                        let v2116 = v2111 * ((v2113.powf(v2095)) - v2);
                        let v4976 = ((v2281 / v15) * (v2095 * (v2113.powf((v2095 - v2280))))) * v2111;
                        v2120 = v2116;
                        v2423 = v4976;
                    }
                    v2119 = v2120;
                    v2422 = v2423;
                }
                v2118 = v2119;
                v2421 = v2422;
            } else {
                let v2117 = v89 / v20;
                let v4970 = v2281 / v20;
                v2118 = v2117;
                v2421 = v4970;
            }
            let v2123 = (v2121 * v1830) * v21;
            let v4984 = (v4528 * v2121) * v21;
            let v2128 = v1 * ((v1834 + v2124) + v2126);
            let v4989 = (((Lanes([v4533[0], v4533[1], v4533[2], 0.0, 0.0])) + v2414) + (Lanes([v2415[0], v2415[1], v2415[2], 0.0, 0.0]))) * v1;
            let v2130 = (ddt(12461, v2128)) * v21;
            let v4991 = (v4989 * v4966) * v21;
            let v2271 = v2128 * v21;
            let v4992 = v4989 * v21;
            let v2131 = v1 * v1859;
            let v4993 = v4578 * v1;
            let v2133 = (ddt(12467, v2131)) * v21;
            let v4995 = (v4993 * v4966) * v21;
            let v2272 = v2131 * v21;
            let v4996 = v4993 * v21;
            let v2137 = v1 * ((v1862 + v2134) + v1953);
            let v5001 = (((Lanes([v4583[0], 0.0, v4583[1], v4583[2], v4583[3]])) + v2416) + (Lanes([v4761[0], 0.0, v4761[1], v4761[2], v4761[3]]))) * v1;
            let v2139 = (ddt(12477, v2137)) * v21;
            let v5003 = (v5001 * v4966) * v21;
            let v2273 = v2137 * v21;
            let v5004 = v5001 * v21;
            let v2141 = v1 * v2140;
            let v5005 = v2417 * v1;
            let v2143 = (ddt(12483, v2141)) * v21;
            let v5007 = (v5005 * v4966) * v21;
            let v2274 = v2141 * v21;
            let v5008 = v5005 * v21;
            let v2145 = v1 * v2144;
            let v2146 = v2145 * v680;
            let v5009 = v2991 * v2145;
            let v2148 = (ddt(12491, v2146)) * v21;
            let v5011 = (v5009 * v4966) * v21;
            let v2275 = v2146 * v21;
            let v5012 = v5009 * v21;
            let v2150 = v1 * v2149;
            let v2151 = v2150 * v683;
            let v5013 = v2995 * v2150;
            let v2153 = (ddt(12499, v2151)) * v21;
            let v5015 = (v5013 * v4966) * v21;
            let v2276 = v2151 * v21;
            let v5016 = v5013 * v21;
            let v2155 = (v1 * v1533) * v21;
            let v5018 = (v4095 * v1) * v21;
            let v2156 = v1 * v696;
            let v5020 = (v3022 * v1) * v1786;
            let v2158 = (v2156 * v1786) * v21;
            let v5025 = ((Lanes([v5020[0], v5020[1], 0.0, v5020[2], v5020[3], v5020[4], v5020[5], v5020[6], v5020[7]])) + (Lanes([0.0, 0.0, (v2314 * v2156), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) * v21;
            let v2161 = v1 * (v1928 + v2159);
            let v5027 = (v4720 + v2410) * v1;
            let v2163 = (ddt(12519, v2161)) * v21;
            let v5029 = (v5027 * v4966) * v21;
            let v2277 = v2161 * v21;
            let v5030 = v5027 * v21;
            let v2167 = (v1 * ((v1531 + v1825) + v1530)) * v21;
            let v5034 = (((v4091 + v4515) + v4087) * v1) * v21;
            let v2170 = v1 * (v1899 + v2168);
            let v5036 = (v4662 + v2411) * v1;
            let v2172 = (ddt(12538, v2170)) * v21;
            let v5038 = (v5036 * v4966) * v21;
            let v2278 = v2170 * v21;
            let v5039 = v5036 * v21;
            let v2256: f64;
            let v2257: f64;
            let v2424: Lanes<3>;
            if v650 != 0.0 {
                let v2173 = v1 * v689;
                let v5042 = (v3003 * v1) * v1791;
                let v2175 = (v2173 * v1791) * v21;
                let v5047 = ((Lanes([0.0, v5042[0], v5042[1]])) + (Lanes([(v2316 * v2173), 0.0, 0.0]))) * v21;
                v2256 = v2175;
                v2257 = v0;
                v2424 = v5047;
            } else {
                v2256 = v0;
                v2257 = v2176;
                v2424 = v5040;
            }
            let v2258: f64;
            let v2259: f64;
            let v2425: Lanes<3>;
            if v653 != 0.0 {
                let v2177 = v1 * v686;
                let v5050 = (v2999 * v1) * v1796;
                let v2179 = (v2177 * v1796) * v21;
                let v5055 = ((Lanes([0.0, v5050[0], v5050[1]])) + (Lanes([(v2318 * v2177), 0.0, 0.0]))) * v21;
                v2258 = v2179;
                v2259 = v0;
                v2425 = v5055;
            } else {
                v2258 = v0;
                v2259 = v2180;
                v2425 = v5048;
            }
            let v2182 = (v1083 + v1082) / v1079;
            let v5059 = ((v3566 + v3567) - (v3556 * v2182)) / v1079;
            let v2184 = if v2183 > v0 { 1.0 } else { 0.0 };
            let v2187: f64;
            let v2426: Lanes<5>;
            if v2184 != 0.0 {
                let v2185 = v1776 / v2182;
                let v2186 = v2185.abs();
                let v5066 = ((v2383 - (v5059 * v2185)) / v2182) * ((v2714 * (if v2185 >= v3137 { 1.0 } else { 0.0 })) - v2280);
                v2187 = v2186;
                v2426 = v5066;
            } else {
                v2187 = v0;
                v2426 = v4128;
            }
            let v2188 = if v2182 > v0 { 1.0 } else { 0.0 };
            let v2195: f64;
            let v2427: Lanes<5>;
            if v2188 != 0.0 {
                let v2190 = (v2124 + v2134) / v2182;
                let v5077 = ((v2414 + v2416) - (v5059 * v2190)) / v2182;
                v2195 = v2190;
                v2427 = v5077;
            } else {
                let v2191 = v608 * v1550;
                let v2192 = v2191 * v1079;
                let v5073 = (((Lanes([(v2929 * v1550), 0.0, 0.0, 0.0, 0.0])) + (v2381 * v608)) * v1079) + (v3556 * v2191);
                v2195 = v2192;
                v2427 = v5073;
            }
            let v2194 = if v2193 == v2 { 1.0 } else { 0.0 };
            let v2211: f64;
            let v2428: Lanes<5>;
            if v2194 != 0.0 {
                let v2196 = v2060 * v2195;
                let v5079 = v2427 * v2060;
                v2211 = v2196;
                v2428 = v5079;
            } else {
                let v2197 = if v2193 == v25 { 1.0 } else { 0.0 };
                let v2212: f64;
                let v2429: Lanes<5>;
                if v2197 != 0.0 {
                    let v2199 = v2198 * v2195;
                    let v5078 = v2427 * v2198;
                    v2212 = v2199;
                    v2429 = v5078;
                } else {
                    v2212 = v0;
                    v2429 = v4128;
                }
                v2211 = v2212;
                v2428 = v2429;
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
            let v5081 = v2428 * v2213;
            let v5085 = (Lanes([v5081[0], v5081[1], v5081[2], v5081[3], v5081[4], 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, ((v2292 * v4966) * v2211)]));
            let v2279 = v2211 * v2210;
            let v5086 = v2428 * v2210;
            let v5090 = (Lanes([v5086[0], v5086[1], v5086[2], v5086[3], v5086[4], 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, (v2292 * v2211)]));
            let v2215 = v2187 * v2210;
            let v5091 = v2426 * v2210;
            let v5095 = (Lanes([v5091[0], v5091[1], v5091[2], v5091[3], v5091[4], 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, (v2292 * v2187)]));
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
            let v5096 = v4932[0];
            let v5097 = v4932[1];
            let v5098 = v4932[2];
            let v5099 = v4932[3];
            let v5100 = v4934[0];
            let v5101 = v4934[1];
            let v5102 = v4934[2];
            let v5103 = v4934[3];
            let v5104 = v4934[4];
            let v5105 = v4936[0];
            let v5106 = v4936[1];
            let v5107 = v4936[2];
            let v5108 = v4938[0];
            let v5109 = v4938[1];
            let v5110 = v4938[2];
            let v5111 = v4938[3];
            let v5112 = v4938[4];
            let v5113 = v2419[0];
            let v5114 = v2419[1];
            let v5115 = v2419[2];
            let v5116 = v2419[3];
            let v5117 = v2420[0];
            let v5118 = v2420[1];
            let v5119 = v2420[2];
            let v5120 = v2420[3];
            let v5121 = v4947[0];
            let v5122 = v4947[1];
            let v5123 = v4947[2];
            let v5124 = v4947[3];
            let v5125 = v4947[4];
            let v5126 = v4947[5];
            let v5127 = v4950[0];
            let v5128 = v4950[1];
            let v5129 = v4950[2];
            let v5130 = v4950[3];
            let v5131 = v4950[4];
            let v5132 = v4957[0];
            let v5133 = v4957[1];
            let v5134 = v4957[2];
            let v5135 = v4964[0];
            let v5136 = v4964[1];
            let v5137 = v4964[2];
            let v5138 = v2421;
            let v5139 = v4968;
            let v5140 = v4984[0];
            let v5141 = v4984[1];
            let v5142 = v4984[2];
            let v5143 = v4984[3];
            let v5144 = v4984[4];
            let v5145 = v4984[5];
            let v5146 = v4984[6];
            let v5147 = v4984[7];
            let v5148 = v4984[8];
            let v5149 = v4984[9];
            let v5150 = v4984[10];
            let v5151 = v4991[0];
            let v5152 = v4991[1];
            let v5153 = v4991[2];
            let v5154 = v4991[3];
            let v5155 = v4991[4];
            let v5156 = v4995[0];
            let v5157 = v4995[1];
            let v5158 = v4995[2];
            let v5159 = v5003[0];
            let v5160 = v5003[1];
            let v5161 = v5003[2];
            let v5162 = v5003[3];
            let v5163 = v5003[4];
            let v5164 = v5007[0];
            let v5165 = v5007[1];
            let v5166 = v5007[2];
            let v5167 = v5007[3];
            let v5168 = v5007[4];
            let v5169 = v5007[5];
            let v5170 = v5011[0];
            let v5171 = v5011[1];
            let v5172 = v5015[0];
            let v5173 = v5015[1];
            let v5174 = v5018[0];
            let v5175 = v5018[1];
            let v5176 = v5018[2];
            let v5177 = v5018[3];
            let v5178 = v5018[4];
            let v5179 = v5018[5];
            let v5180 = v5018[6];
            let v5181 = v5018[7];
            let v5182 = v5018[8];
            let v5183 = v5025[0];
            let v5184 = v5025[1];
            let v5185 = v5025[2];
            let v5186 = v5025[3];
            let v5187 = v5025[4];
            let v5188 = v5025[5];
            let v5189 = v5025[6];
            let v5190 = v5025[7];
            let v5191 = v5025[8];
            let v5192 = v5029[0];
            let v5193 = v5029[1];
            let v5194 = v5029[2];
            let v5195 = v5029[3];
            let v5196 = v5029[4];
            let v5197 = v5029[5];
            let v5198 = v5029[6];
            let v5199 = v5029[7];
            let v5200 = v5029[8];
            let v5201 = v5034[0];
            let v5202 = v5034[1];
            let v5203 = v5034[2];
            let v5204 = v5034[3];
            let v5205 = v5034[4];
            let v5206 = v5034[5];
            let v5207 = v5038[0];
            let v5208 = v5038[1];
            let v5209 = v5038[2];
            let v5210 = v5038[3];
            let v5211 = v5038[4];
            let v5212 = v5038[5];
            let v5213 = v2424[0];
            let v5214 = v2424[1];
            let v5215 = v2424[2];
            let v5216 = v2425[0];
            let v5217 = v2425[1];
            let v5218 = v2425[2];
            let v5219 = v2292;
            let v5220 = v5085[0];
            let v5221 = v5085[1];
            let v5222 = v5085[2];
            let v5223 = v5085[3];
            let v5224 = v5085[4];
            let v5225 = v5085[5];
            let v5226 = v5095[0];
            let v5227 = v5095[1];
            let v5228 = v5095[2];
            let v5229 = v5095[3];
            let v5230 = v5095[4];
            let v5231 = v5095[5];
            let v5232 = v4969;
            let v5233 = v4992[0];
            let v5234 = v4992[1];
            let v5235 = v4992[2];
            let v5236 = v4992[3];
            let v5237 = v4992[4];
            let v5238 = v4996[0];
            let v5239 = v4996[1];
            let v5240 = v4996[2];
            let v5241 = v5004[0];
            let v5242 = v5004[1];
            let v5243 = v5004[2];
            let v5244 = v5004[3];
            let v5245 = v5004[4];
            let v5246 = v5008[0];
            let v5247 = v5008[1];
            let v5248 = v5008[2];
            let v5249 = v5008[3];
            let v5250 = v5008[4];
            let v5251 = v5008[5];
            let v5252 = v5012[0];
            let v5253 = v5012[1];
            let v5254 = v5016[0];
            let v5255 = v5016[1];
            let v5256 = v5030[0];
            let v5257 = v5030[1];
            let v5258 = v5030[2];
            let v5259 = v5030[3];
            let v5260 = v5030[4];
            let v5261 = v5030[5];
            let v5262 = v5030[6];
            let v5263 = v5030[7];
            let v5264 = v5030[8];
            let v5265 = v5039[0];
            let v5266 = v5039[1];
            let v5267 = v5039[2];
            let v5268 = v5039[3];
            let v5269 = v5039[4];
            let v5270 = v5039[5];
            let v5271 = v5090[0];
            let v5272 = v5090[1];
            let v5273 = v5090[2];
            let v5274 = v5090[3];
            let v5275 = v5090[4];
            let v5276 = v5090[5];
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(8),
            multiplicity * (v2066),
            [3, 6, 7, 8],
            [v5096, v5097, v5098, v5099],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(4),
            multiplicity * (v2068),
            [3, 4, 6, 7, 8],
            [v5100, v5101, v5102, v5103, v5104],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(4),
            multiplicity * (v2070),
            [3, 4, 5],
            [v5105, v5106, v5107],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(4),
            multiplicity * (v2072),
            [3, 4, 6, 7, 8],
            [v5108, v5109, v5110, v5111, v5112],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(7),
            multiplicity * (v2254),
            [3, 5, 6, 7],
            [v5113, v5114, v5115, v5116],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(8),
            multiplicity * (v2255),
            [3, 5, 6, 7],
            [v5117, v5118, v5119, v5120],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(6),
            multiplicity * (v2080),
            [3, 4, 5, 6, 7, 8],
            [v5121, v5122, v5123, v5124, v5125, v5126],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(8),
            multiplicity * (v2084),
            [3, 4, 6, 7, 8],
            [v5127, v5128, v5129, v5130, v5131],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(2),
            Some(4),
            multiplicity * (v2087),
            [2, 3, 4],
            [v5132, v5133, v5134],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(5),
            multiplicity * (v2090),
            [1, 3, 5],
            [v5135, v5136, v5137],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (v2118),
            [3],
            [v5138],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (v2094),
            [3],
            [v5139],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<11, 0>(
            Some(3),
            None,
            multiplicity * (v2123),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            [v5140, v5141, v5142, v5143, v5144, v5145, v5146, v5147, v5148, v5149, v5150],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(4),
            multiplicity * (v2130),
            [3, 4, 6, 7, 8],
            [v5151, v5152, v5153, v5154, v5155],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(4),
            multiplicity * (v2133),
            [3, 4, 5],
            [v5156, v5157, v5158],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(8),
            multiplicity * (v2139),
            [3, 4, 6, 7, 8],
            [v5159, v5160, v5161, v5162, v5163],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(6),
            multiplicity * (v2143),
            [3, 4, 5, 6, 7, 8],
            [v5164, v5165, v5166, v5167, v5168, v5169],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(2),
            multiplicity * (v2148),
            [1, 2],
            [v5170, v5171],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(0),
            multiplicity * (v2153),
            [0, 1],
            [v5172, v5173],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(9),
            multiplicity * (v2155),
            [0, 1, 3, 5, 6, 7, 8, 9, 10],
            [v5174, v5175, v5176, v5177, v5178, v5179, v5180, v5181, v5182],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(0),
            Some(9),
            multiplicity * (v2158),
            [0, 1, 3, 5, 6, 7, 8, 9, 10],
            [v5183, v5184, v5185, v5186, v5187, v5188, v5189, v5190, v5191],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(9),
            multiplicity * (v2163),
            [0, 1, 3, 5, 6, 7, 8, 9, 10],
            [v5192, v5193, v5194, v5195, v5196, v5197, v5198, v5199, v5200],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(10),
            multiplicity * (v2167),
            [3, 5, 6, 7, 8, 10],
            [v5201, v5202, v5203, v5204, v5205, v5206],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(10),
            multiplicity * (v2172),
            [3, 5, 6, 7, 8, 10],
            [v5207, v5208, v5209, v5210, v5211, v5212],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(9),
            Some(10),
            multiplicity * (v2256),
            [3, 9, 10],
            [v5213, v5214, v5215],
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
            [v5216, v5217, v5218],
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
            [v5219],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(4),
            multiplicity * (v2214),
            [3, 4, 6, 7, 8, 11],
            [v5220, v5221, v5222, v5223, v5224, v5225],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            Some(6),
            multiplicity * (v2215),
            [3, 4, 6, 7, 8, 11],
            [v5226, v5227, v5228, v5229, v5230, v5231],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(8),
            Some(4),
            multiplicity * (v2210),
            [11],
            [v5219],
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
        self.canonical_reactive[0] = v2066;
        self.canonical_reactive[1] = v2068;
        self.canonical_reactive[2] = v2070;
        self.canonical_reactive[3] = v2072;
        self.canonical_reactive[4] = v2254;
        self.canonical_reactive[5] = v2255;
        self.canonical_reactive[6] = v2080;
        self.canonical_reactive[7] = v2084;
        self.canonical_reactive[8] = v2087;
        self.canonical_reactive[9] = v2090;
        self.canonical_reactive[10] = v2118;
        self.canonical_reactive[11] = v2270;
        self.canonical_reactive[12] = v5232;
        self.canonical_reactive[13] = v2123;
        self.canonical_reactive[14] = v2271;
        self.canonical_reactive[15] = v5233;
        self.canonical_reactive[16] = v5234;
        self.canonical_reactive[17] = v5235;
        self.canonical_reactive[18] = v5236;
        self.canonical_reactive[19] = v5237;
        self.canonical_reactive[20] = v2272;
        self.canonical_reactive[21] = v5238;
        self.canonical_reactive[22] = v5239;
        self.canonical_reactive[23] = v5240;
        self.canonical_reactive[24] = v2273;
        self.canonical_reactive[25] = v5241;
        self.canonical_reactive[26] = v5242;
        self.canonical_reactive[27] = v5243;
        self.canonical_reactive[28] = v5244;
        self.canonical_reactive[29] = v5245;
        self.canonical_reactive[30] = v2274;
        self.canonical_reactive[31] = v5246;
        self.canonical_reactive[32] = v5247;
        self.canonical_reactive[33] = v5248;
        self.canonical_reactive[34] = v5249;
        self.canonical_reactive[35] = v5250;
        self.canonical_reactive[36] = v5251;
        self.canonical_reactive[37] = v2275;
        self.canonical_reactive[38] = v5252;
        self.canonical_reactive[39] = v5253;
        self.canonical_reactive[40] = v2276;
        self.canonical_reactive[41] = v5254;
        self.canonical_reactive[42] = v5255;
        self.canonical_reactive[43] = v2155;
        self.canonical_reactive[44] = v2158;
        self.canonical_reactive[45] = v2277;
        self.canonical_reactive[46] = v5256;
        self.canonical_reactive[47] = v5257;
        self.canonical_reactive[48] = v5258;
        self.canonical_reactive[49] = v5259;
        self.canonical_reactive[50] = v5260;
        self.canonical_reactive[51] = v5261;
        self.canonical_reactive[52] = v5262;
        self.canonical_reactive[53] = v5263;
        self.canonical_reactive[54] = v5264;
        self.canonical_reactive[55] = v2167;
        self.canonical_reactive[56] = v2278;
        self.canonical_reactive[57] = v5265;
        self.canonical_reactive[58] = v5266;
        self.canonical_reactive[59] = v5267;
        self.canonical_reactive[60] = v5268;
        self.canonical_reactive[61] = v5269;
        self.canonical_reactive[62] = v5270;
        self.canonical_reactive[63] = v2256;
        self.canonical_reactive[64] = v2257;
        self.canonical_reactive[65] = v2258;
        self.canonical_reactive[66] = v2259;
        self.canonical_reactive[67] = v2209;
        self.canonical_reactive[68] = v2210;
        self.canonical_reactive[69] = v2279;
        self.canonical_reactive[70] = v5271;
        self.canonical_reactive[71] = v5272;
        self.canonical_reactive[72] = v5273;
        self.canonical_reactive[73] = v5274;
        self.canonical_reactive[74] = v5275;
        self.canonical_reactive[75] = v5276;
        self.canonical_reactive[76] = v2215;
        self.canonical_reactive[77] = v2210;
        self.canonical_reactive[78] = v2216;
        self.canonical_reactive[79] = v2217;
        self.canonical_reactive[80] = v2218;
        self.canonical_reactive[81] = v2219;
        self.canonical_reactive[82] = v2220;
        self.canonical_reactive[83] = v2221;
        self.canonical_reactive[84] = v2222;
        self.canonical_reactive[85] = v2223;
        self.canonical_reactive[86] = v2224;
        self.canonical_reactive[87] = v2225;
        self.canonical_reactive[88] = v2226;
        self.canonical_reactive[89] = v2227;
        self.canonical_reactive[90] = v2228;
        self.canonical_reactive[91] = v2229;
        self.canonical_reactive[92] = v2260;
        self.canonical_reactive[93] = v2261;
        self.canonical_reactive[94] = v2240;
        self.canonical_reactive[95] = v2262;
        self.canonical_reactive[96] = v2264;
        self.canonical_reactive[97] = v2242;
        self.canonical_reactive[98] = v2266;
        self.canonical_reactive[99] = v2244;
        self.canonical_reactive[100] = v2268;
        self.canonical_reactive[101] = v2246;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            None,
            &[3],
            &[cached[12]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(4),
            &[3, 4, 6, 7, 8],
            &[cached[15], cached[16], cached[17], cached[18], cached[19]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(4),
            &[3, 4, 5],
            &[cached[21], cached[22], cached[23]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(8),
            &[3, 4, 6, 7, 8],
            &[cached[25], cached[26], cached[27], cached[28], cached[29]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(6),
            &[3, 4, 5, 6, 7, 8],
            &[cached[31], cached[32], cached[33], cached[34], cached[35], cached[36]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(2),
            &[1, 2],
            &[cached[38], cached[39]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(0),
            &[0, 1],
            &[cached[41], cached[42]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(9),
            &[0, 1, 3, 5, 6, 7, 8, 9, 10],
            &[cached[46], cached[47], cached[48], cached[49], cached[50], cached[51], cached[52], cached[53], cached[54]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(10),
            &[3, 5, 6, 7, 8, 10],
            &[cached[57], cached[58], cached[59], cached[60], cached[61], cached[62]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(4),
            &[3, 4, 6, 7, 8, 11],
            &[cached[70], cached[71], cached[72], cached[73], cached[74], cached[75]],
            &[],
            &[],
            multiplicity,
        );
    }

}

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
        let parameter_given = &*self.param_given;
        let multiplicity = self.multiplicity;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13]), ctx.node_voltage(self.nodes[14]), ctx.node_voltage(self.nodes[15]), ctx.node_voltage(self.nodes[16]), ctx.node_voltage(self.nodes[17]), ctx.node_voltage(self.nodes[18])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 73821 => 0usize, 73825 => 1usize, 73829 => 2usize, 73902 => 3usize, 73906 => 4usize, 73967 => 5usize, 73987 => 6usize, 73993 => 7usize, 74024 => 8usize, 74030 => 9usize, 74051 => 10usize, 74074 => 11usize, 74094 => 12usize, 74100 => 13usize, 74106 => 14usize, _ => usize::MAX };
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
            let v1 = 0e0f64;
            let v2 = 0e0f64;
            let v3 = parameters[43];
            let v4 = 1e0f64;
            let v6 = 1e-12f64;
            let v7 = parameters[237];
            let v8 = 5e-1f64;
            let v9 = parameters[51];
            let v10 = 1e1f64;
            let v13 = 2e2f64;
            let v14 = parameters[52];
            let v15 = 1e-2f64;
            let v17 = parameters[73];
            let v18 = 1e-6f64;
            let v20 = parameters[104];
            let v22 = parameters[201];
            let v24 = 1e-4f64;
            let v25 = parameters[240];
            let v27 = parameters[241];
            let v29 = parameters[242];
            let v31 = parameters[243];
            let v33 = parameters[59];
            let v35 = parameters[284];
            let v37 = parameters[148];
            let v39 = parameters[198];
            let v41 = parameters[70];
            let v43 = parameters[83];
            let v45 = parameters[84];
            let v47 = parameters[85];
            let v49 = parameters[80];
            let v51 = parameters[81];
            let v53 = parameters[82];
            let v55 = parameters[250];
            let v56 = 1e6f64;
            let v58 = parameters[232];
            let v59 = 2.7315e2f64;
            let v61 = parameters[58];
            let v62 = parameters[15];
            let v63 = 1e2f64;
            let v65 = parameters[46];
            let v66 = parameters[34];
            let v67 = if parameter_given[190] { 1.0 } else { 0.0 };
            let v68 = parameters[190];
            let v69 = 5e9f64;
            let v73 = 2e0f64;
            let v74 = 1e-1f64;
            let v75 = 2.1e0f64;
            let v77 = 1.0f64;
            let v79 = 2.1e0f64;
            let v83 = 1.0000000000000005e-4f64;
            let v85 = 4e0f64;
            let v86 = 8e0f64;
            let v87 = 1.0f64;
            let v88 = 0.0f64;
            let v89 = 1.0f64;
            let v90 = 0.0f64;
            let v91 = 3e0f64;
            let v92 = 0.0f64;
            let v102 = 2.5e-1f64;
            let v108 = 2.1e0f64;
            let v110 = parameters[55];
            let v111 = 9.025e-5f64;
            let v112 = 1e-7f64;
            let v117 = parameters[236];
            let v118 = 1.034943e-10f64;
            let v121 = 3.453133e-11f64;
            let v124 = parameters[239];
            let v128 = parameters[0];
            let v129 = parameters[56];
            let v132 = parameters[57];
            let v135 = parameters[40];
            let v139 = parameters[1];
            let v140 = parameters[9];
            let v142 = parameters[60];
            let v144 = parameters[295];
            let v146 = parameters[61];
            let v153 = parameters[18];
            let v167 = parameters[107];
            let v168 = parameters[108];
            let v169 = parameters[111];
            let v174 = parameters[109];
            let v175 = parameters[110];
            let v183 = parameters[72];
            let v187 = parameters[74];
            let v188 = parameters[75];
            let v193 = parameters[62];
            let v197 = parameters[63];
            let v202 = 1.6021918e-19f64;
            let v203 = 1.3806226e-23f64;
            let v208 = parameters[244];
            let v209 = parameters[247];
            let v213 = parameters[251];
            let v214 = parameters[252];
            let v218 = parameters[248];
            let v220 = parameters[249];
            let v224 = 3.2043836e-19f64;
            let v232 = parameters[91];
            let v234 = parameters[89];
            let v236 = parameters[68];
            let v237 = parameters[76];
            let v238 = parameters[77];
            let v242 = parameters[78];
            let v243 = parameters[79];
            let v246 = parameters[149];
            let v247 = parameters[150];
            let v249 = parameters[151];
            let v254 = parameters[152];
            let v255 = parameters[153];
            let v259 = parameters[192];
            let v261 = parameters[193];
            let v264 = parameters[67];
            let v265 = parameters[7];
            let v266 = parameters[6];
            let v271 = parameters[8];
            let v276 = parameters[44];
            let v278 = parameters[130];
            let v279 = parameters[131];
            let v283 = parameters[124];
            let v284 = parameters[125];
            let v285 = parameters[126];
            let v290 = parameters[123];
            let v293 = parameters[117];
            let v294 = parameters[119];
            let v295 = parameters[120];
            let v300 = parameters[118];
            let v301 = parameters[121];
            let v306 = parameters[127];
            let v307 = parameters[128];
            let v308 = parameters[129];
            let v320 = parameters[132];
            let v321 = parameters[133];
            let v334 = parameters[65];
            let v336 = parameters[66];
            let v339 = parameters[134];
            let v340 = parameters[135];
            let v341 = parameters[136];
            let v350 = parameters[115];
            let v352 = parameters[114];
            let v356 = parameters[116];
            let v358 = 1e-50f64;
            let v361 = parameters[50];
            let v362 = parameters[253];
            let v364 = if parameter_given[168] { 1.0 } else { 0.0 };
            let v365 = if parameter_given[169] { 1.0 } else { 0.0 };
            let v366 = if parameter_given[170] { 1.0 } else { 0.0 };
            let v367 = if parameter_given[294] { 1.0 } else { 0.0 };
            let v368 = if parameter_given[293] { 1.0 } else { 0.0 };
            let v369 = if parameter_given[13] { 1.0 } else { 0.0 };
            let v370 = if parameter_given[14] { 1.0 } else { 0.0 };
            let v371 = if parameter_given[23] { 1.0 } else { 0.0 };
            let v372 = if parameter_given[22] { 1.0 } else { 0.0 };
            let v373 = if parameter_given[16] { 1.0 } else { 0.0 };
            let v374 = parameters[17];
            let v378 = parameters[13];
            let v379 = parameters[14];
            let v380 = parameters[16];
            let v384 = parameters[10];
            let v386 = parameters[11];
            let v391 = parameters[12];
            let v414 = parameters[162];
            let v417 = parameters[161];
            let v419 = parameters[163];
            let v429 = parameters[199];
            let v430 = parameters[200];
            let v434 = parameters[202];
            let v435 = parameters[203];
            let v455 = parameters[165];
            let v458 = parameters[164];
            let v460 = parameters[166];
            let v500 = 5.1702525384001115e-2f64;
            let v501 = 1.04e16f64;
            let v505 = 5.1702525384001115e-2f64;
            let v506 = 1.04e16f64;
            let v510 = 1.2919089961638799e9f64;
            let v513 = parameters[194];
            let v514 = parameters[195];
            let v518 = parameters[196];
            let v519 = parameters[197];
            let v525 = 1e-3f64;
            let v526 = 4e-6f64;
            let v531 = 1e-10f64;
            let v532 = 1e-13f64;
            let v535 = parameters[35];
            let v539 = 1e3f64;
            let v540 = 1e3f64;
            let v541 = parameters[261];
            let v543 = parameters[289];
            let v545 = parameters[288];
            let v548 = parameters[262];
            let v550 = parameters[290];
            let v552 = 1e4f64;
            let v553 = 1e4f64;
            let v556 = parameters[291];
            let v558 = 1e4f64;
            let v561 = parameters[24];
            let v562 = parameters[23];
            let v563 = parameters[20];
            let v565 = parameters[19];
            let v568 = parameters[22];
            let v569 = parameters[21];
            let v576 = parameters[294];
            let v581 = parameters[293];
            let v597 = node_potentials[6];
            let v598 = node_potentials[7];
            let v601 = node_potentials[11];
            let v604 = node_potentials[12];
            let v607 = node_potentials[0];
            let v608 = node_potentials[2];
            let v611 = 1e-9f64;
            let v612 = 1e-5f64;
            let v613 = node_potentials[18];
            let v615 = 1e-5f64;
            let v616 = node_potentials[13];
            let v618 = 1e-5f64;
            let v619 = node_potentials[15];
            let v621 = 1e-5f64;
            let v622 = node_potentials[16];
            let v624 = 1e-5f64;
            let v626 = parameters[38];
            let v630 = node_potentials[10];
            let v635 = -1e0f64;
            let v639 = 5e0f64;
            let v641 = 6e0f64;
            let v643 = temperature;
            let v651 = parameters[53];
            let v654 = parameters[54];
            let v661 = parameters[254];
            let v662 = parameters[98];
            let v663 = parameters[99];
            let v668 = parameters[100];
            let v669 = parameters[101];
            let v674 = parameters[102];
            let v675 = parameters[103];
            let v680 = parameters[159];
            let v683 = parameters[158];
            let v686 = parameters[160];
            let v695 = parameters[112];
            let v702 = 1.8e0f64;
            let v703 = 4e-1f64;
            let v715 = 1.04e16f64;
            let v716 = 1.5e0f64;
            let v743 = 1.414213562373095e0f64;
            let v758 = 1.2919089961638799e9f64;
            let v760 = 1.2919089961638799e9f64;
            let v771 = 8e-1f64;
            let v772 = 1.2e0f64;
            let v791 = 1.0f64;
            let v792 = 0.0f64;
            let v793 = 0.0f64;
            let v794 = 1.0f64;
            let v795 = 0.0f64;
            let v805 = 1.25e-1f64;
            let v816 = 2e1f64;
            let v822 = -2e1f64;
            let v824 = -2e1f64;
            let v827 = -2e1f64;
            let v829 = -2e1f64;
            let v835 = parameters[226];
            let v837 = 5e-1f64;
            let v838 = 1.6666666666666666e-1f64;
            let v839 = 4.1666666666666664e-2f64;
            let v840 = 8.333333333333333e-3f64;
            let v841 = 1.388888888888889e-3f64;
            let v842 = 1.984126984126984e-4f64;
            let v856 = 5e-12f64;
            let v878 = 4e-6f64;
            let v883 = 1e-13f64;
            let v894 = 5e-2f64;
            let v896 = 2.0000000000000004e-2f64;
            let v897 = 1.0f64;
            let v898 = -2.0000000000000004e-2f64;
            let v917 = parameters[204];
            let v919 = parameters[206];
            let v922 = parameters[205];
            let v939 = 4e-8f64;
            let v944 = 1.0000000000000002e-14f64;
            let v971 = 1e12f64;
            let v986 = 2e-3f64;
            let v987 = 1.0f64;
            let v988 = -2e-3f64;
            let v999 = 2.069886e-10f64;
            let v1030 = 2.069886e-10f64;
            let v1047 = 9.5e-1f64;
            let v1052 = 3.8e0f64;
            let v1063 = 3.2043836e-19f64;
            let v1082 = parameters[69];
            let v1097 = parameters[71];
            let v1109 = parameters[86];
            let v1112 = parameters[88];
            let v1115 = parameters[87];
            let v1129 = parameters[105];
            let v1142 = parameters[90];
            let v1144 = -3e0f64;
            let v1147 = 3.333333333333333e-1f64;
            let v1148 = 2.7e1f64;
            let v1149 = 3.7037037037037035e-2f64;
            let v1156 = 3.333333333333333e-1f64;
            let v1157 = 4.02052934513951e-2f64;
            let v1158 = 1.48148111111111e-1f64;
            let v1171 = 4.000000000000001e-2f64;
            let v1176 = 1.0000000000000001e-11f64;
            let v1183 = 2e-1f64;
            let v1184 = 1.0f64;
            let v1185 = -2e-1f64;
            let v1203 = 7e0f64;
            let v1218 = -1.6021918e-19f64;
            let v1221 = -1.6021918e-19f64;
            let v1226 = 1e-5f64;
            let v1228 = parameters[39];
            let v1249 = 2.220446049250313e-15f64;
            let v1251 = 2.220446049250313e-15f64;
            let v1265 = 8e-4f64;
            let v1300 = -1e-9f64;
            let v1368 = -1e0f64;
            let v1381 = 1.2919089961638799e9f64;
            let v1385 = 9.9e-1f64;
            let v1405 = 5e-1f64;
            let v1406 = 1.6666666666666666e-1f64;
            let v1407 = 4.1666666666666664e-2f64;
            let v1408 = 8.333333333333333e-3f64;
            let v1409 = 1.388888888888889e-3f64;
            let v1410 = 1.984126984126984e-4f64;
            let v1443 = 1.0f64;
            let v1444 = 0.0f64;
            let v1445 = 1.0f64;
            let v1446 = 0.0f64;
            let v1447 = 0.0f64;
            let v1457 = 2.5e-1f64;
            let v1476 = 1.0f64;
            let v1477 = 0.0f64;
            let v1478 = 1.0f64;
            let v1479 = 0.0f64;
            let v1480 = 0.0f64;
            let v1490 = 2.5e-1f64;
            let v1508 = 0.0f64;
            let v1517 = 2.220446049250313e-15f64;
            let v1519 = 2.220446049250313e-15f64;
            let v1531 = 1.3094570021973102e-2f64;
            let v1535 = 8.1e1f64;
            let v1538 = -2.916e3f64;
            let v1544 = 1.458e3f64;
            let v1545 = 5.4e1f64;
            let v1557 = 3.333333333333333e-1f64;
            let v1559 = 1.259921049894873e0f64;
            let v1564 = 2.6456684199469993e-1f64;
            let v1610 = 1.2919089961638799e9f64;
            let v1656 = 9.8e-1f64;
            let v1660 = 1.0f64;
            let v1666 = 2.560000000000001e-2f64;
            let v1668 = 1.0f64;
            let v1669 = 0.0f64;
            let v1670 = 1.0f64;
            let v1671 = 0.0f64;
            let v1672 = 0.0f64;
            let v1682 = 2.5e-1f64;
            let v1700 = -1.6e0f64;
            let v1702 = 6e-1f64;
            let v1738 = 2.220446049250313e-15f64;
            let v1740 = 2.220446049250313e-15f64;
            let v1787 = -1e-9f64;
            let v1860 = -1e0f64;
            let v1881 = parameters[25];
            let v1884 = 2e-1f64;
            let v1891 = parameters[137];
            let v1892 = 3.2043836e-19f64;
            let v1947 = 3.0000000000000002e-2f64;
            let v1964 = 2.220446049250313e-15f64;
            let v1966 = 2.220446049250313e-15f64;
            let v1976 = 1.3e0f64;
            let v1980 = 3e-2f64;
            let v1995 = parameters[36];
            let v1997 = 4.12e0f64;
            let v1998 = parameters[142];
            let v2003 = parameters[145];
            let v2008 = parameters[144];
            let v2013 = 9.9e1f64;
            let v2026 = 4e-6f64;
            let v2031 = 1e-13f64;
            let v2034 = parameters[143];
            let v2042 = -3.4e1f64;
            let v2045 = 2.5e-1f64;
            let v2049 = 7.38905609893065e0f64;
            let v2081 = 4e-6f64;
            let v2086 = 1e-13f64;
            let v2093 = 0e0f64;
            let v2098 = parameters[122];
            let v2103 = 0e0f64;
            let v2108 = 4e-4f64;
            let v2113 = 1e-12f64;
            let v2117 = 0e0f64;
            let v2144 = 1.0f64;
            let v2145 = 0.0f64;
            let v2146 = 0.0f64;
            let v2147 = 1.0f64;
            let v2148 = 0.0f64;
            let v2158 = 1.25e-1f64;
            let v2179 = 4e-6f64;
            let v2184 = 1e-13f64;
            let v2199 = parameters[26];
            let v2203 = parameters[141];
            let v2207 = 4.1046315303568966e26f64;
            let v2208 = 2.4665765749313358e0f64;
            let v2211 = 2.1633307652783932e-2f64;
            let v2218 = parameters[140];
            let v2223 = 3.3163543761348e-29f64;
            let v2242 = parameters[37];
            let v2243 = parameters[138];
            let v2244 = parameters[139];
            let v2248 = 1e-5f64;
            let v2249 = node_potentials[17];
            let v2263 = -1e-9f64;
            let v2321 = 5e2f64;
            let v2323 = 1.403592217853e217f64;
            let v2325 = 6e1f64;
            let v2328 = 1.14200738981568e26f64;
            let v2337 = -1e-9f64;
            let v2377 = 1.0f64;
            let v2378 = 0.0f64;
            let v2379 = 1.0f64;
            let v2380 = 0.0f64;
            let v2381 = 0.0f64;
            let v2391 = 2.5e-1f64;
            let v2421 = 1.0f64;
            let v2422 = 0.0f64;
            let v2423 = 1.0f64;
            let v2424 = 0.0f64;
            let v2425 = 0.0f64;
            let v2435 = 2.5e-1f64;
            let v2475 = -1e0f64;
            let v2480 = -1e0f64;
            let v2530 = 8e1f64;
            let v2532 = 1.25e2f64;
            let v2533 = 4e1f64;
            let v2536 = 2.5e1f64;
            let v2586 = -5e-1f64;
            let v2592 = 5e-1f64;
            let v2620 = 1.0f64;
            let v2621 = 0.0f64;
            let v2622 = 0.0f64;
            let v2623 = 1.0f64;
            let v2624 = 0.0f64;
            let v2634 = 1.25e-1f64;
            let v2647 = 4e-4f64;
            let v2652 = 1e-12f64;
            let v2668 = 0.0f64;
            let v2677 = 1.3e0f64;
            let v2681 = 1.3e0f64;
            let v2691 = 1.3e0f64;
            let v2704 = 2.220446049250313e-15f64;
            let v2706 = 2.220446049250313e-15f64;
            let v2738 = 2.220446049250313e-15f64;
            let v2740 = 2.220446049250313e-15f64;
            let v2765 = 1.2919089961638799e9f64;
            let v2769 = 1.2919089961638799e9f64;
            let v2796 = -1e-9f64;
            let v2864 = -1e0f64;
            let v2904 = -1e-9f64;
            let v2977 = -1e0f64;
            let v3020 = -1e-9f64;
            let v3094 = -1e-9f64;
            let v3134 = 1.0f64;
            let v3135 = 0.0f64;
            let v3136 = 1.0f64;
            let v3137 = 0.0f64;
            let v3138 = 0.0f64;
            let v3148 = 2.5e-1f64;
            let v3178 = 1.0f64;
            let v3179 = 0.0f64;
            let v3180 = 1.0f64;
            let v3181 = 0.0f64;
            let v3182 = 0.0f64;
            let v3192 = 2.5e-1f64;
            let v3234 = -1e0f64;
            let v3239 = -1e0f64;
            let v3340 = -5e-1f64;
            let v3361 = 1.0f64;
            let v3362 = 0.0f64;
            let v3363 = 1.0f64;
            let v3364 = 0.0f64;
            let v3365 = 0.0f64;
            let v3385 = 1.0f64;
            let v3386 = 0.0f64;
            let v3387 = 1.0f64;
            let v3388 = 0.0f64;
            let v3389 = 0.0f64;
            let v3399 = 2.5e-1f64;
            let v3417 = 1e-5f64;
            let v3419 = 1.0f64;
            let v3421 = 1e-5f64;
            let v3425 = 1.0000000000000004e-20f64;
            let v3427 = 1.0f64;
            let v3428 = 0.0f64;
            let v3429 = 1.0f64;
            let v3430 = 0.0f64;
            let v3431 = 0.0f64;
            let v3441 = 2.5e-1f64;
            let v3447 = 1e-5f64;
            let v3453 = 2.220446049250313e-15f64;
            let v3455 = 2.220446049250313e-15f64;
            let v3457 = -5e-1f64;
            let v3477 = -1e0f64;
            let v3488 = 4.242640687119285e0f64;
            let v3495 = 9e0f64;
            let v3498 = 9.899494936611664e0f64;
            let v3501 = 1e-8f64;
            let v3504 = -9.899494936611664e0f64;
            let v3512 = -9.899494936611664e0f64;
            let v3517 = -5.65685424949238e0f64;
            let v3518 = 1.2e1f64;
            let v3537 = 0.0f64;
            let v3545 = 2.220446049250313e-15f64;
            let v3547 = 2.220446049250313e-15f64;
            let v3558 = 1.3094570021973102e-2f64;
            let v3564 = -2.916e3f64;
            let v3586 = 2.6456684199469993e-1f64;
            let v3613 = 2.5e-12f64;
            let v3625 = 1e-5f64;
            let v3647 = 2.01e2f64;
            let v3667 = 1e-16f64;
            let v3679 = 5e-3f64;
            let v3743 = -1e0f64;
            let v3746 = -1e0f64;
            let v3753 = 1.01e0f64;
            let v3802 = 2.01e2f64;
            let v3805 = 5e-2f64;
            let v3814 = -1e0f64;
            let v3833 = 2.220446049250313e-15f64;
            let v3835 = 2.220446049250313e-15f64;
            let v3847 = -1e0f64;
            let v3885 = 1.0f64;
            let v3886 = 0.0f64;
            let v3887 = 0.0f64;
            let v3888 = 1.0f64;
            let v3889 = 0.0f64;
            let v3899 = 1.25e-1f64;
            let v3912 = 4e-4f64;
            let v3917 = 1e-12f64;
            let v3935 = 0.0f64;
            let v3937 = 1.0f64;
            let v3942 = 1.3e0f64;
            let v3946 = 1.3e0f64;
            let v3956 = 1.3e0f64;
            let v3972 = 2.01e2f64;
            let v4062 = -1e0f64;
            let v4111 = 2.01e2f64;
            let v4114 = 5e-2f64;
            let v4123 = -1e0f64;
            let v4141 = 2.220446049250313e-15f64;
            let v4240 = 1e0f64;
            let v4242 = 1.0f64;
            let v4243 = 0.0f64;
            let v4244 = 0.0f64;
            let v4245 = 1.0f64;
            let v4246 = 0.0f64;
            let v4256 = 1.25e-1f64;
            let v4265 = 2.220446049250313e-15f64;
            let v4267 = 2.220446049250313e-15f64;
            let v4269 = 6.666666666666667e-1f64;
            let v4294 = -5e-1f64;
            let v4316 = 5.0000001e-1f64;
            let v4325 = 2.220446049250313e-15f64;
            let v4327 = parameters[191];
            let v4328 = 2.220446049250313e-15f64;
            let v4337 = 2.220446049250313e-15f64;
            let v4340 = 2.220446049250313e-15f64;
            let v4351 = parameters[189];
            let v4358 = 2.220446049250313e-15f64;
            let v4361 = 2.220446049250313e-15f64;
            let v4366 = 4e-6f64;
            let v4371 = 1e-13f64;
            let v4383 = 1e5f64;
            let v4384 = 1e9f64;
            let v4431 = 5e-1f64;
            let v4446 = parameters[227];
            let v4448 = 5e-1f64;
            let v4449 = 1.6666666666666666e-1f64;
            let v4450 = 4.1666666666666664e-2f64;
            let v4451 = 8.333333333333333e-3f64;
            let v4452 = 1.388888888888889e-3f64;
            let v4453 = 1.984126984126984e-4f64;
            let v4467 = 2.220446049250313e-15f64;
            let v4469 = 2.220446049250313e-15f64;
            let v4472 = 1.034943e-12f64;
            let v4475 = parameters[92];
            let v4477 = parameters[93];
            let v4479 = parameters[94];
            let v4488 = 3.6e7f64;
            let v4493 = 3e-7f64;
            let v4497 = parameters[97];
            let v4505 = parameters[95];
            let v4506 = parameters[96];
            let v4508 = 1e11f64;
            let v4514 = parameters[106];
            let v4523 = 4e-100f64;
            let v4528 = 1.0000000000000001e-60f64;
            let v4542 = 9.999999999999978e-1f64;
            let v4543 = parameters[113];
            let v4545 = 1.0000000000000022e0f64;
            let v4548 = 1.9999999999999978e0f64;
            let v4550 = 2.000000000000002e0f64;
            let v4559 = 9.999999999999978e-1f64;
            let v4561 = 1.0000000000000022e0f64;
            let v4565 = 1.9999999999999978e0f64;
            let v4567 = 2.000000000000002e0f64;
            let v4572 = -1e0f64;
            let v4584 = parameters[281];
            let v4591 = 5e-1f64;
            let v4592 = 1.6666666666666666e-1f64;
            let v4593 = 4.1666666666666664e-2f64;
            let v4594 = 8.333333333333333e-3f64;
            let v4595 = 1.388888888888889e-3f64;
            let v4596 = 1.984126984126984e-4f64;
            let v4610 = 1.1e0f64;
            let v4614 = 1.0000000000000002e-2f64;
            let v4619 = 5.0000000000000005e-12f64;
            let v4625 = parameters[245];
            let v4628 = parameters[246];
            let v4652 = parameters[33];
            let v4663 = parameters[154];
            let v4664 = parameters[155];
            let v4668 = parameters[156];
            let v4669 = parameters[157];
            let v4691 = -1e0f64;
            let v4712 = 4e-4f64;
            let v4717 = 1e-12f64;
            let v4739 = 2e-3f64;
            let v4742 = 8e-3f64;
            let v4757 = 4e-4f64;
            let v4762 = 1e-12f64;
            let v4766 = 2.220446049250313e-15f64;
            let v4770 = 4e-4f64;
            let v4775 = 1e-12f64;
            let v4779 = 2.220446049250313e-15f64;
            let v4788 = 4.000000000000001e-2f64;
            let v4793 = 1.0000000000000001e-11f64;
            let v4797 = 2.220446049250313e-15f64;
            let v4804 = 1e0f64;
            let v4806 = 1.0f64;
            let v4807 = 0.0f64;
            let v4808 = 0.0f64;
            let v4809 = 1.0f64;
            let v4810 = 0.0f64;
            let v4820 = 1.25e-1f64;
            let v4833 = parameters[30];
            let v4835 = parameters[32];
            let v4846 = 4e-6f64;
            let v4851 = 1e-13f64;
            let v4855 = 4e-6f64;
            let v4860 = 1e-13f64;
            let v4866 = 2.220446049250313e-15f64;
            let v4868 = 2.220446049250313e-15f64;
            let v4874 = parameters[285];
            let v4877 = parameters[286];
            let v4880 = parameters[283];
            let v4887 = 3.2043836e-19f64;
            let v4897 = -2.5e-1f64;
            let v4909 = 2.220446049250313e-15f64;
            let v4911 = 2.220446049250313e-15f64;
            let v4922 = 1.0f64;
            let v4926 = 1.3094570021973102e-2f64;
            let v4932 = -2.916e3f64;
            let v4954 = 2.6456684199469993e-1f64;
            let v4989 = parameters[287];
            let v5050 = 1.0f64;
            let v5056 = 2.560000000000001e-2f64;
            let v5058 = 1.0f64;
            let v5059 = 0.0f64;
            let v5060 = 1.0f64;
            let v5061 = 0.0f64;
            let v5062 = 0.0f64;
            let v5072 = 2.5e-1f64;
            let v5079 = 2.5e-12f64;
            let v5101 = 1.3e0f64;
            let v5105 = 1.3e0f64;
            let v5115 = 1.3e0f64;
            let v5124 = parameters[282];
            let v5137 = 4.242640687119285e0f64;
            let v5146 = 9.899494936611664e0f64;
            let v5151 = -9.899494936611664e0f64;
            let v5159 = -9.899494936611664e0f64;
            let v5164 = -5.65685424949238e0f64;
            let v5201 = 2.01e2f64;
            let v5332 = 2.01e2f64;
            let v5335 = 5e-2f64;
            let v5344 = -1e0f64;
            let v5365 = -1e0f64;
            let v5380 = 7.071067811865475e-1f64;
            let v5392 = 4e-12f64;
            let v5397 = 1e-16f64;
            let v5426 = 3.2043836e-19f64;
            let v5441 = 1.0f64;
            let v5442 = 1.0f64;
            let v5443 = 0.0f64;
            let v5444 = 0.0f64;
            let v5445 = 0.0f64;
            let v5455 = 5e-1f64;
            let v5463 = 2.220446049250313e-15f64;
            let v5474 = parameters[45];
            let v5486 = parameters[48];
            let v5495 = parameters[49];
            let v5504 = 4e-6f64;
            let v5509 = 1e-13f64;
            let v5526 = 4e-4f64;
            let v5531 = 1e-12f64;
            let v5564 = 1.0f64;
            let v5565 = 0.0f64;
            let v5566 = 0.0f64;
            let v5567 = 1.0f64;
            let v5568 = 0.0f64;
            let v5578 = 1.25e-1f64;
            let v5599 = 4e-6f64;
            let v5604 = 1e-13f64;
            let v5628 = 4.1046315303568966e26f64;
            let v5629 = 2.4665765749313358e0f64;
            let v5632 = 2.1633307652783932e-2f64;
            let v5660 = 3.3163543761348e-29f64;
            let v5685 = parameters[47];
            let v5705 = 1e-5f64;
            let v5712 = parameters[146];
            let v5720 = parameters[147];
            let v5730 = 4.000000000000001e-2f64;
            let v5735 = 1.0000000000000001e-11f64;
            let v5746 = 4.000000000000001e-2f64;
            let v5751 = 1.0000000000000001e-11f64;
            let v5788 = parameters[27];
            let v5791 = 2.220446049250313e-15f64;
            let v5794 = parameters[216];
            let v5799 = parameters[215];
            let v5804 = parameters[217];
            let v5810 = 4e-4f64;
            let v5815 = 1e-12f64;
            let v5819 = 4e-6f64;
            let v5824 = 1e-13f64;
            let v5837 = parameters[219];
            let v5840 = parameters[218];
            let v5845 = parameters[214];
            let v5849 = -3.4e1f64;
            let v5852 = parameters[213];
            let v5867 = parameters[221];
            let v5870 = parameters[222];
            let v5877 = parameters[220];
            let v5883 = -1e0f64;
            let v5896 = -1e0f64;
            let v5901 = parameters[225];
            let v5905 = 4e-4f64;
            let v5910 = 1e-12f64;
            let v5915 = parameters[224];
            let v5918 = -3.4e1f64;
            let v5921 = parameters[223];
            let v5927 = parameters[28];
            let v5929 = parameters[209];
            let v5930 = parameters[210];
            let v5934 = parameters[211];
            let v5940 = 4e-4f64;
            let v5945 = 1e-12f64;
            let v5951 = parameters[208];
            let v5955 = -3.4e1f64;
            let v5958 = parameters[207];
            let v5969 = parameters[212];
            let v5984 = 4e-4f64;
            let v5989 = 1e-12f64;
            let v5998 = -3.4e1f64;
            let v6026 = 1.0f64;
            let v6030 = parameters[292];
            let v6031 = 0.0f64;
            let v6039 = 1e0f64;
            let v6040 = 0e0f64;
            let v6070 = 2.220446049250313e-15f64;
            let v6105 = 4.242640687119285e0f64;
            let v6114 = 9.899494936611664e0f64;
            let v6122 = -9.899494936611664e0f64;
            let v6130 = -9.899494936611664e0f64;
            let v6135 = -5.65685424949238e0f64;
            let v6155 = 4.9787068367863944e-2f64;
            let v6164 = 2.220446049250313e-15f64;
            let v6166 = 2.220446049250313e-15f64;
            let v6182 = 2.220446049250313e-15f64;
            let v6184 = 2.220446049250313e-15f64;
            let v6193 = -1.047839336957922e-1f64;
            let v6194 = 7.071067811865476e-1f64;
            let v6200 = -5.151950988020902e1f64;
            let v6202 = 5.286687693921294e-4f64;
            let v6205 = 1.8773541122053122e-2f64;
            let v6208 = 2.8160311683079683e-2f64;
            let v6210 = 1.0979672760764175e-2f64;
            let v6212 = 7.930031540881942e-4f64;
            let v6226 = -3.7209791878387604e0f64;
            let v6271 = 6.0000000000000005e-2f64;
            let v6274 = 6.0000000000000005e-2f64;
            let v6291 = 2.220446049250313e-15f64;
            let v6293 = 2.220446049250313e-15f64;
            let v6299 = parameters[42];
            let v6303 = 4.1e1f64;
            let v6311 = 2.9693154855771e-1f64;
            let v6312 = -7.053654284009761e-2f64;
            let v6313 = 6.115288895133179e-3f64;
            let v6319 = 8.907946456731299e-1f64;
            let v6320 = -2.8214617136039044e-1f64;
            let v6333 = 7.07106781186548e-1f64;
            let v6334 = -1.17851130197758e-1f64;
            let v6335 = 1.78800506338833e-2f64;
            let v6336 = -1.63730162779191e-3f64;
            let v6337 = 6.36964918866352e-5f64;
            let v6347 = -2.35702260395516e-1f64;
            let v6348 = 5.3640151901649905e-2f64;
            let v6349 = -6.54920651116764e-3f64;
            let v6392 = -1e0f64;
            let v6398 = 4.1e1f64;
            let v6401 = 5e-2f64;
            let v6410 = -1e0f64;
            let v6431 = 2.220446049250313e-15f64;
            let v6450 = 1.0f64;
            let v6459 = 0.0f64;
            let v6466 = 0e0f64;
            let v6467 = 1e0f64;
            let v6478 = 2.220446049250313e-15f64;
            let v6505 = 4.242640687119285e0f64;
            let v6514 = 9.899494936611664e0f64;
            let v6522 = -9.899494936611664e0f64;
            let v6530 = -9.899494936611664e0f64;
            let v6535 = -5.65685424949238e0f64;
            let v6555 = 4.9787068367863944e-2f64;
            let v6564 = 2.220446049250313e-15f64;
            let v6566 = 2.220446049250313e-15f64;
            let v6582 = 2.220446049250313e-15f64;
            let v6584 = 2.220446049250313e-15f64;
            let v6593 = -1.047839336957922e-1f64;
            let v6594 = 7.071067811865476e-1f64;
            let v6600 = -5.151950988020902e1f64;
            let v6602 = 5.286687693921294e-4f64;
            let v6605 = 1.8773541122053122e-2f64;
            let v6608 = 2.8160311683079683e-2f64;
            let v6610 = 1.0979672760764175e-2f64;
            let v6612 = 7.930031540881942e-4f64;
            let v6626 = -3.7209791878387604e0f64;
            let v6671 = 6.0000000000000005e-2f64;
            let v6674 = 6.0000000000000005e-2f64;
            let v6691 = 2.220446049250313e-15f64;
            let v6693 = 2.220446049250313e-15f64;
            let v6702 = 4.1e1f64;
            let v6710 = -7.053654284009761e-2f64;
            let v6716 = 8.907946456731299e-1f64;
            let v6717 = -2.8214617136039044e-1f64;
            let v6730 = -1.17851130197758e-1f64;
            let v6731 = -1.63730162779191e-3f64;
            let v6741 = -2.35702260395516e-1f64;
            let v6742 = 5.3640151901649905e-2f64;
            let v6743 = -6.54920651116764e-3f64;
            let v6786 = -1e0f64;
            let v6792 = 4.1e1f64;
            let v6795 = 5e-2f64;
            let v6804 = -1e0f64;
            let v6827 = 2.220446049250313e-15f64;
            let v6850 = 1.0f64;
            let v6857 = 0.0f64;
            let v6870 = parameters[64];
            let v6872 = 2.220446049250313e-15f64;
            let v6875 = 2.220446049250313e-15f64;
            let v6878 = 1e-15f64;
            let v6885 = parameters[29];
            let v6887 = parameters[188];
            let v6890 = parameters[171];
            let v6891 = parameters[172];
            let v6917 = 1e0f64;
            let v6918 = 0e0f64;
            let v6941 = 2.220446049250313e-15f64;
            let v6991 = 4.242640687119285e0f64;
            let v7000 = 9.899494936611664e0f64;
            let v7008 = -9.899494936611664e0f64;
            let v7016 = -9.899494936611664e0f64;
            let v7021 = -5.65685424949238e0f64;
            let v7041 = 4.9787068367863944e-2f64;
            let v7050 = 2.220446049250313e-15f64;
            let v7052 = 2.220446049250313e-15f64;
            let v7068 = 2.220446049250313e-15f64;
            let v7070 = 2.220446049250313e-15f64;
            let v7079 = -1.047839336957922e-1f64;
            let v7080 = 7.071067811865476e-1f64;
            let v7086 = -5.151950988020902e1f64;
            let v7088 = 5.286687693921294e-4f64;
            let v7091 = 1.8773541122053122e-2f64;
            let v7094 = 2.8160311683079683e-2f64;
            let v7096 = 1.0979672760764175e-2f64;
            let v7098 = 7.930031540881942e-4f64;
            let v7112 = -3.7209791878387604e0f64;
            let v7118 = parameters[41];
            let v7159 = 6.0000000000000005e-2f64;
            let v7162 = 6.0000000000000005e-2f64;
            let v7180 = 2.220446049250313e-15f64;
            let v7182 = 2.220446049250313e-15f64;
            let v7195 = 4.1e1f64;
            let v7203 = -7.053654284009761e-2f64;
            let v7209 = 8.907946456731299e-1f64;
            let v7210 = -2.8214617136039044e-1f64;
            let v7223 = -1.17851130197758e-1f64;
            let v7224 = -1.63730162779191e-3f64;
            let v7234 = -2.35702260395516e-1f64;
            let v7235 = 5.3640151901649905e-2f64;
            let v7236 = -6.54920651116764e-3f64;
            let v7279 = -1e0f64;
            let v7285 = 4.1e1f64;
            let v7288 = 5e-2f64;
            let v7297 = -1e0f64;
            let v7318 = 2.220446049250313e-15f64;
            let v7351 = 0e0f64;
            let v7352 = 1e0f64;
            let v7375 = 2.220446049250313e-15f64;
            let v7419 = 4.242640687119285e0f64;
            let v7428 = 9.899494936611664e0f64;
            let v7436 = -9.899494936611664e0f64;
            let v7444 = -9.899494936611664e0f64;
            let v7449 = -5.65685424949238e0f64;
            let v7469 = 4.9787068367863944e-2f64;
            let v7478 = 2.220446049250313e-15f64;
            let v7480 = 2.220446049250313e-15f64;
            let v7496 = 2.220446049250313e-15f64;
            let v7498 = 2.220446049250313e-15f64;
            let v7507 = -1.047839336957922e-1f64;
            let v7508 = 7.071067811865476e-1f64;
            let v7514 = -5.151950988020902e1f64;
            let v7516 = 5.286687693921294e-4f64;
            let v7519 = 1.8773541122053122e-2f64;
            let v7522 = 2.8160311683079683e-2f64;
            let v7524 = 1.0979672760764175e-2f64;
            let v7526 = 7.930031540881942e-4f64;
            let v7540 = -3.7209791878387604e0f64;
            let v7586 = 6.0000000000000005e-2f64;
            let v7589 = 6.0000000000000005e-2f64;
            let v7607 = 2.220446049250313e-15f64;
            let v7609 = 2.220446049250313e-15f64;
            let v7622 = 4.1e1f64;
            let v7630 = -7.053654284009761e-2f64;
            let v7636 = 8.907946456731299e-1f64;
            let v7637 = -2.8214617136039044e-1f64;
            let v7650 = -1.17851130197758e-1f64;
            let v7651 = -1.63730162779191e-3f64;
            let v7661 = -2.35702260395516e-1f64;
            let v7662 = 5.3640151901649905e-2f64;
            let v7663 = -6.54920651116764e-3f64;
            let v7706 = -1e0f64;
            let v7712 = 4.1e1f64;
            let v7715 = 5e-2f64;
            let v7724 = -1e0f64;
            let v7747 = 2.220446049250313e-15f64;
            let v7783 = parameters[170];
            let v7785 = parameters[169];
            let v7876 = parameters[173];
            let v7880 = parameters[175];
            let v7884 = parameters[174];
            let v7888 = parameters[176];
            let v7906 = parameters[177];
            let v7932 = parameters[178];
            let v7958 = parameters[179];
            let v7959 = parameters[2];
            let v7961 = parameters[3];
            let v7963 = parameters[238];
            let v7966 = parameters[5];
            let v7968 = parameters[180];
            let v7971 = parameters[181];
            let v7976 = parameters[185];
            let v7979 = parameters[182];
            let v7993 = parameters[186];
            let v7996 = parameters[183];
            let v8012 = parameters[187];
            let v8015 = parameters[184];
            let v8088 = parameters[4];
            let v8203 = -1.6021918e-19f64;
            let v8228 = -1e0f64;
            let v8231 = -1.6021918e-19f64;
            let v8256 = -1e0f64;
            let v8258 = parameters[233];
            let v8259 = parameters[234];
            let v8272 = parameters[235];
            let v8274 = parameters[31];
            let v8279 = -2e0f64;
            let v8289 = 2.220446049250313e-15f64;
            let v8307 = 9.999999999999978e-1f64;
            let v8309 = 1.0000000000000022e0f64;
            let v8312 = 1.9999999999999978e0f64;
            let v8314 = 2.000000000000002e0f64;
            let v8323 = -1e0f64;
            let v8354 = 1.5e1f64;
            let v8377 = 4.2e1f64;
            let v8402 = 3.872983346207417e0f64;
            let v8423 = parameters[168];
            let v8430 = 2.1983327444149834e-11f64;
            let v8431 = parameters[167];
            let v8463 = 2.1983327444149834e-11f64;
            let v8518 = 2.069886e-10f64;
            let v8521 = 1.3e0f64;
            let v8712 = 1.898893985185185e-20f64;
            let v8718 = 2.220446049250313e-15f64;
            let v8720 = 2.220446049250313e-15f64;
            let v8749 = parameters[259];
            let v8751 = 1.0f64;
            let v8752 = parameters[264];
            let v8754 = parameters[266];
            let v8755 = parameters[268];
            let v8756 = parameters[273];
            let v8757 = parameters[263];
            let v8759 = parameters[255];
            let v8762 = parameters[258];
            let v8765 = parameters[265];
            let v8766 = parameters[267];
            let v8767 = parameters[272];
            let v8769 = parameters[256];
            let v8772 = parameters[257];
            let v8775 = parameters[271];
            let v8784 = parameters[269];
            let v8787 = parameters[270];
            let v8792 = parameters[274];
            let v8795 = parameters[279];
            let v8796 = parameters[280];
            let v8800 = parameters[277];
            let v8801 = parameters[278];
            let v8805 = parameters[275];
            let v8806 = parameters[276];
            let v8822 = 9.999999999999978e-1f64;
            let v8824 = 1.0000000000000022e0f64;
            let v8827 = 1.9999999999999978e0f64;
            let v8829 = 2.000000000000002e0f64;
            let v8839 = 9.999999999999978e-1f64;
            let v8841 = 1.0000000000000022e0f64;
            let v8845 = 1.9999999999999978e0f64;
            let v8847 = 2.000000000000002e0f64;
            let v8852 = -1e0f64;
            let v8875 = parameters[260];
            let v8877 = 0.0f64;
            let v8926 = 9.999999999999978e-1f64;
            let v8928 = 1.0000000000000022e0f64;
            let v8931 = 1.9999999999999978e0f64;
            let v8933 = 2.000000000000002e0f64;
            let v8943 = 9.999999999999978e-1f64;
            let v8945 = 1.0000000000000022e0f64;
            let v8949 = 1.9999999999999978e0f64;
            let v8951 = 2.000000000000002e0f64;
            let v8956 = -1e0f64;
            let v8981 = 1.0000000000000001e-11f64;
            let v8983 = 1.0000000000000001e-11f64;
            let v8985 = 1.0000000000000001e-11f64;
            let v8987 = 1.0000000000000001e-11f64;
            let v9019 = 1.0000000000000001e-11f64;
            let v9021 = 1.0000000000000001e-11f64;
            let v9022 = 1.0000000000000001e-11f64;
            let v9094 = 5.5224904e-23f64;
            let v9103 = 0e0f64;
            let v9106 = 0e0f64;
            let v9114 = 0e0f64;
            let v9124 = node_potentials[14];
            let v9125 = 0e0f64;
            let v9126 = 0e0f64;
            let v9140 = 0e0f64;
            let v9141 = 0e0f64;
            let v9142 = 0e0f64;
            let v9143 = 0e0f64;
            let v9144 = 0e0f64;
            let v9147 = node_potentials[1];
            let v9150 = 0e0f64;
            let v9172 = node_potentials[4];
            let v9177 = 0e0f64;
            let v9180 = node_potentials[9];
            let v9185 = node_potentials[8];
            let v9188 = 0e0f64;
            let v9189 = 0e0f64;
            let v9196 = 1e-5f64;
            let v9199 = 1e-5f64;
            let v9202 = 0e0f64;
            let v9203 = 0e0f64;
            let v9212 = 1e-5f64;
            let v9215 = 0e0f64;
            let v9220 = 0e0f64;
            let v9222 = 1e-5f64;
            let v9225 = 0e0f64;
            let v9233 = 1e-5f64;
            let v9236 = 1e-5f64;
            let v9239 = 1e-5f64;
            let v9242 = 0e0f64;
            let v9243 = 0e0f64;
            let v9244 = 0e0f64;
            let v9245 = 0e0f64;
            let v9246 = 0e0f64;
            let v9247 = 0e0f64;
            let v9360 = 1e0f64;
            let v9361 = Lanes([1e0f64; 1]);
            let v9362 = Lanes([1e0f64; 1]);
            let v9363 = Lanes([1e0f64; 1]);
            let v9364 = Lanes([1e0f64; 1]);
            let v9365 = Lanes([1e0f64; 1]);
            let v9366 = Lanes([1e0f64; 1]);
            let v9367 = Lanes([1e0f64; 1]);
            let v9368 = Lanes([1e0f64; 1]);
            let v9369 = Lanes([1e0f64; 1]);
            let v9370 = Lanes([1e0f64; 1]);
            let v9371 = Lanes([1e0f64; 1]);
            let v9372 = Lanes([1e0f64; 1]);
            let v9373 = Lanes([1e0f64; 1]);
            let v9374 = Lanes([1e0f64; 1]);
            let v9375 = Lanes([1e0f64; 1]);
            let v9376 = Lanes([1e0f64; 1]);
            let v9377 = Lanes([1e0f64; 1]);
            let v10366 = Lanes([0e0f64; 1]);
            let v10367 = Lanes([0e0f64; 1]);
            let v10368 = Lanes([0e0f64; 1]);
            let v10372 = Lanes([0e0f64; 2]);
            let v10373 = Lanes([0e0f64; 2]);
            let v10374 = Lanes([0e0f64; 1]);
            let v10381 = Lanes([0e0f64; 1]);
            let v10382 = -1e0f64;
            let v10427 = 2e0f64;
            let v10496 = Lanes([0e0f64; 3]);
            let v10507 = -8.75e-1f64;
            let v10522 = Lanes([0e0f64; 2]);
            let v10523 = Lanes([0e0f64; 3]);
            let v10571 = Lanes([0e0f64; 5]);
            let v10617 = Lanes([0e0f64; 4]);
            let v10652 = Lanes([0e0f64; 4]);
            let v10922 = -6.666666666666667e-1f64;
            let v10991 = -6.666666666666667e-1f64;
            let v11028 = Lanes([0e0f64; 1]);
            let v11054 = Lanes([0e0f64; 6]);
            let v11123 = -8.75e-1f64;
            let v11296 = 0e0f64;
            let v11379 = -8.75e-1f64;
            let v12040 = -7.5e-1f64;
            let v12057 = -7.5e-1f64;
            let v12114 = -7.5e-1f64;
            let v12629 = -8.75e-1f64;
            let v12835 = -8.75e-1f64;
            let v13263 = -7.5e-1f64;
            let v13304 = -7.5e-1f64;
            let v13507 = -7.5e-1f64;
            let v13554 = -7.5e-1f64;
            let v14242 = -8.75e-1f64;
            let v14453 = -6.666666666666667e-1f64;
            let v14521 = -7.5e-1f64;
            let v14832 = -6.666666666666667e-1f64;
            let v14971 = -5e-1f64;
            let v15055 = -8.75e-1f64;
            let v15757 = -6.666666666666667e-1f64;
            let v15762 = -6.666666666666667e-1f64;
            let v16087 = -6.666666666666667e-1f64;
            let v16257 = -6.666666666666667e-1f64;
            let v16262 = -6.666666666666667e-1f64;
            let v16587 = -6.666666666666667e-1f64;
            let v16821 = -6.666666666666667e-1f64;
            let v16826 = -6.666666666666667e-1f64;
            let v17160 = -6.666666666666667e-1f64;
            let v17349 = -6.666666666666667e-1f64;
            let v17354 = -6.666666666666667e-1f64;
            let v17688 = -6.666666666666667e-1f64;
            let v17762 = Lanes([0e0f64; 3]);
            let v17763 = Lanes([0e0f64; 3]);
            let v18451 = Lanes([0e0f64; 5]);
            let v18626 = Lanes([0e0f64; 3]);
            let v18627 = Lanes([0e0f64; 7]);
            let v18628 = Lanes([0e0f64; 7]);
            let v18653 = Lanes([0e0f64; 7]);
            let v18654 = Lanes([0e0f64; 7]);
            let v18655 = Lanes([0e0f64; 8]);
            let v18733 = ddt_scale();
            let v18781 = Lanes([0e0f64; 2]);
            let v18810 = Lanes([0e0f64; 2]);
            let v18811 = Lanes([0e0f64; 2]);
            let v18812 = Lanes([0e0f64; 2]);
            let v19033 = -7.5e-1f64;
            let v19080 = -7.5e-1f64;
            let v5 = if v3 == v4 { 1.0 } else { 0.0 };
            if v5 != 0.0 {
            } else {
            }
            let v12 = (v9 * v10) % v10;
            let v16 = v14 * v15;
            let v19 = v17 / v18;
            let v21 = v20 * v15;
            let v23 = v22 / v18;
            let v26 = v25 / v18;
            let v28 = v27 / v18;
            let v30 = v29 * v15;
            let v32 = v31 / v15;
            let v34 = v33 / v18;
            let v36 = v35 / v18;
            let v38 = v37 / v18;
            let v40 = v39 / v24;
            let v42 = v41 * v15;
            let v44 = if v43 == v0 { 1.0 } else { 0.0 };
            let v46: f64;
            if v44 != 0.0 {
                v46 = v0;
            } else {
                v46 = v45;
            }
            let v48: f64;
            if v44 != 0.0 {
                v48 = v0;
            } else {
                v48 = v47;
            }
            let v50 = if v49 == v0 { 1.0 } else { 0.0 };
            let v52: f64;
            if v50 != 0.0 {
                v52 = v0;
            } else {
                v52 = v51;
            }
            let v54: f64;
            if v44 != 0.0 {
                v54 = v0;
            } else {
                v54 = v53;
            }
            let v57 = v55 * v56;
            let v60 = v58 + v59;
            let v64 = v62 * v63;
            let v72: f64;
            if v67 != 0.0 {
                v72 = v68;
            } else {
                let v71 = v69 / (v7 * v25);
                v72 = v71;
            }
            let v78 = if (if v72 < v75 { 1.0 } else { 0.0 }) != 0.0 && v77 != 0.0 { 1.0 } else { 0.0 };
            let v4346: f64;
            if v78 != 0.0 {
                let v80 = v79 - v72;
                let v81 = v80 * v80;
                let v84 = (v81 * v81) + v83;
                let v104: f64;
                if v87 != 0.0 {
                    let v98: f64;
                    if v88 != 0.0 {
                        v98 = v4;
                    } else {
                        let v99: f64;
                        if v89 != 0.0 {
                            v99 = v73;
                        } else {
                            let v100: f64;
                            if v90 != 0.0 {
                                v100 = v91;
                            } else {
                                let v101: f64;
                                if v92 != 0.0 {
                                    v101 = v85;
                                } else {
                                    v101 = v0;
                                }
                                v100 = v101;
                            }
                            v99 = v100;
                        }
                        v98 = v99;
                    }
                    let mut v93: f64 = 0.0;
                    let mut v95: f64 = 0.0;
                    v93 = v0;
                    v95 = v84;
                    loop {
                        let v94 = if v93 < v98 { 1.0 } else { 0.0 };
                        if v94 == 0.0 {
                            break;
                        }
                        let v96 = v95.sqrt();
                        let v97 = v93 + v4;
                        v93 = v97;
                        v95 = v96;
                    }
                    v104 = v95;
                } else {
                    let v103 = v84.powf(v102);
                    v104 = v103;
                }
                let v109 = v108 - ((v80 * v74) * (v4 / v104));
                v4346 = v109;
            } else {
                v4346 = v72;
            }
            let v116 = v110 - (v60 * (v111 + (v60 * v112)));
            let v119 = v118 / v7;
            let v120 = v4 / v119;
            let v122 = v121 / v117;
            let v123 = v117 / v121;
            let v125 = v121 / v124;
            let v126 = v124 / v121;
            let v127 = v126 + v120;
            let v131 = v128 - (v73 * v129);
            let v134 = v128 - (v73 * v132);
            let v136 = if v135 == v0 { 1.0 } else { 0.0 };
            let v137: f64;
            if v136 != 0.0 {
                v137 = v128;
            } else {
                v137 = v131;
            }
            let v138 = v137 * v56;
            let v141 = v139 / v140;
            let v143 = if v12 < v4 { 1.0 } else { 0.0 };
            let v145: f64;
            if v143 != 0.0 {
                v145 = v0;
            } else {
                v145 = v144;
            }
            let v147: f64;
            if v143 != 0.0 {
                v147 = v142;
            } else {
                v147 = v146;
            }
            let v148 = if v3 == v0 { 1.0 } else { 0.0 };
            let v161: f64;
            let v163: f64;
            if v148 != 0.0 {
                let v150 = v141 - (v73 * v142);
                let v152 = v141 - (v73 * v147);
                v161 = v150;
                v163 = v152;
            } else {
                let v155 = v141 - (v153 * v145);
                let v156 = v73 - v153;
                let v158 = v155 - (v156 * v142);
                let v160 = v155 - (v156 * v147);
                v161 = v158;
                v163 = v160;
            }
            let v162 = v161 * v140;
            let v164 = v163 * v140;
            let v165 = v141 * v56;
            let v166 = v165 * v138;
            let v179 = (v167 * (v4 + (v168 / (v138.powf(v169))))) * (v4 + (v174 / (v165.powf(v175))));
            let v180 = if v12 > v91 { 1.0 } else { 0.0 };
            let v184 = if v183 > v0 { 1.0 } else { 0.0 };
            let v185 = if (if v180 != 0.0 && (if v19 < v26 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v184 != 0.0 { 1.0 } else { 0.0 };
            let v186: f64;
            if v185 != 0.0 {
                v186 = v26;
            } else {
                v186 = v19;
            }
            let v192 = v186 * (v4 + (v187 / (v165.powf(v188))));
            let v194 = v8 * v128;
            let v201 = v73 / ((v4 / (v193 + v194)) + (v4 / (v197 + v194)));
            let v205 = v202 / (v203 * v60);
            let v207 = (v202 * v28) * v118;
            let v212 = v208 * (v138.powf((-v209)));
            let v217 = v213 * (v138.powf((-v214)));
            let v223 = v218 * ((v138 + v57).powf((-v220)));
            let v227 = ((v224 * v38) * v118).sqrt();
            let v229 = v4 / (v38 * v38);
            let v235 = ((v4 + (v4 / v138)).powf(v232)) * v234;
            let v241 = v137 + (v237 / (v166.powf(v238)));
            let v245 = v242 / (v166.powf(v243));
            let v258 = (v246 * (v4 + (v247 / ((v241 * v56).powf(v249))))) + (v254 / (v165.powf(v255)));
            let v263 = v4 + ((v138.powf(v259)) * v261);
            let v275 = (v264 * (v265 + (v161 / (v91 * v266)))) / ((v266 * (v128 - v271)) * v140);
            let v277 = if v276 <= v0 { 1.0 } else { 0.0 };
            let v2075: f64;
            let v2101: f64;
            let v2102: f64;
            let v2116: f64;
            let v2191: f64;
            let v2195: f64;
            if v277 != 0.0 {
                let v282 = v4 + (v278 / (v165.powf(v279)));
                let v289 = v283 * (v4 + (v284 / (v138.powf(v285))));
                let v292 = v138 / (v138 + v290);
                let v299 = v293 * (v4 + (v294 / (v138.powf(v295))));
                let v304 = v300 * (v4 + (v301 / v138));
                v2075 = v289;
                v2101 = v292;
                v2102 = v282;
                v2116 = v2117;
                v2191 = v304;
                v2195 = v299;
            } else {
                let v305 = v165.powf(v279);
                let v315 = (v306 * (v4 + (v307 / (v138.powf(v308))))) * (v305 / (v305 + v278));
                let v319 = v283 * (v4 + (v284 / (v138.powf(v285))));
                let v325 = v290 * (v4 + (v320 / (v138.powf(v321))));
                let v329 = v293 * (v4 + (v294 / (v138.powf(v295))));
                let v332 = v300 * (v4 + (v301 / v138));
                v2075 = v319;
                v2101 = v325;
                v2102 = v2103;
                v2116 = v315;
                v2191 = v332;
                v2195 = v329;
            }
            let v338 = ((v56 * v164) * v334) / (v138.powf(v336));
            let v345 = v339 * (v4 + (v340 / (v138.powf(v341))));
            let v2092: f64;
            if v277 != 0.0 {
                let v349 = v306 * (v4 + (v307 / (v138.powf(v308))));
                v2092 = v349;
            } else {
                v2092 = v2093;
            }
            let v351 = v350 * v138;
            let v359 = (((v351 * v352) / (v351 + v352)) + v356) + v358;
            let v360 = if v359 < v91 { 1.0 } else { 0.0 };
            let v2657: f64;
            if v360 != 0.0 {
                v2657 = v91;
            } else {
                v2657 = v359;
            }
            let v363 = v361 * v362;
            let v375 = if v374 == v0 { 1.0 } else { 0.0 };
            let v376: f64;
            if v375 != 0.0 {
                v376 = v0;
            } else {
                v376 = v4;
            }
            let v377 = ctx.simparam_or("gmin", v0);
            let v381 = v380 + v59;
            let v382 = v30 / v162;
            let v383 = v32 * v164;
            let v395 = if (if (if v384 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v386 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if v140 == v4 { 1.0 } else { 0.0 }) != 0.0 || (if (if v140 > v4 { 1.0 } else { 0.0 }) != 0.0 && (if v391 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v412: f64;
            if v395 != 0.0 {
                let mut v396: f64 = 0.0;
                let mut v398: f64 = 0.0;
                v396 = v0;
                v398 = v0;
                loop {
                    let v397 = if v396 < v140 { 1.0 } else { 0.0 };
                    if v397 == 0.0 {
                        break;
                    }
                    let v401 = v396 * (v391 + v128);
                    let v408 = (v398 + (v4 / ((v384 + v194) + v401))) + (v4 / ((v386 + v194) + v401));
                    let v409 = v396 + v4;
                    v396 = v409;
                    v398 = v408;
                }
                let v411 = (v73 * v140) / v398;
                v412 = v411;
            } else {
                v412 = v0;
            }
            let v413 = if v412 > v0 { 1.0 } else { 0.0 };
            let v476: f64;
            if v413 != 0.0 {
                let v416 = v4 / (v4 + v414);
                let v428 = (v192 * (v4 + (v416 * ((v417 / v412).powf(v419))))) / (v4 + (v416 * ((v417 / v201).powf(v419))));
                v476 = v428;
            } else {
                v476 = v192;
            }
            let v440 = v23 / v26;
            let v442 = (v440 - ((v4 + (v429 / (v165.powf(v430)))) * (v4 + (v434 / (v138.powf(v435)))))) - v15;
            let v444 = (v85 * v440) * v15;
            let v445 = if v444 > v0 { 1.0 } else { 0.0 };
            let v447: f64;
            if v445 != 0.0 {
                v447 = v444;
            } else {
                let v446 = -v444;
                v447 = v446;
            }
            let v454 = v26 * (v440 - (v8 * (v442 + (((v442 * v442) + v447).sqrt()))));
            let v473: f64;
            if v413 != 0.0 {
                let v457 = v4 / (v4 + v455);
                let v469 = (v454 * (v4 + (v457 * ((v458 / v412).powf(v460))))) / (v4 + (v457 * ((v458 / v201).powf(v460))));
                v473 = v469;
            } else {
                v473 = v454;
            }
            let v472 = if (if v137 > v183 { 1.0 } else { 0.0 }) != 0.0 || (if v183 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v485: f64;
            if v472 != 0.0 {
                let v479 = ((v473 * (v137 - v183)) + (v476 * v183)) / v137;
                v485 = v479;
            } else {
                let v484 = v476 + (((v476 - v473) * (v183 - v137)) / v183);
                v485 = v484;
            }
            let v486 = v202 * v485;
            let v487 = v486 * v118;
            let v488 = v73 * v487;
            let v491 = if (if v137 <= (v73 * v183) { 1.0 } else { 0.0 }) != 0.0 && v184 != 0.0 { 1.0 } else { 0.0 };
            let v698: f64;
            if v491 != 0.0 {
                let v499 = ((((v73 * v476) - (((v476 - v473) * v137) / v183)) - v473) / v473).ln();
                v698 = v499;
            } else {
                v698 = v0;
            }
            let v504 = v500 * ((v485 / v501).ln());
            let v509 = v505 * ((v473 / v506).ln());
            let v512 = (v510 / v485).sqrt();
            let v523 = (v4 + (v513 / (v138.powf(v514)))) * (v4 + (v518 / (v166.powf(v519))));
            let v533 = (v8 * (v523 + (((v523 * v523) + v526).sqrt()))) + v532;
            let v534 = if v533 < v0 { 1.0 } else { 0.0 };
            let v700: f64;
            if v534 != 0.0 {
                v700 = v0;
            } else {
                v700 = v533;
            }
            let v536 = if v535 == v4 { 1.0 } else { 0.0 };
            let v9145: f64;
            if v536 != 0.0 {
                let v537 = if v275 > v525 { 1.0 } else { 0.0 };
                let v9146: f64;
                if v537 != 0.0 {
                    let v538 = v4 / v275;
                    v9146 = v538;
                } else {
                    v9146 = v539;
                }
                v9145 = v9146;
            } else {
                v9145 = v540;
            }
            let v542 = if v541 == v4 { 1.0 } else { 0.0 };
            let v9174: f64;
            if v542 != 0.0 {
                let v546 = (v543 * v162) + v545;
                let v547 = if v546 < v24 { 1.0 } else { 0.0 };
                let v9175: f64;
                if v547 != 0.0 {
                    v9175 = v24;
                } else {
                    v9175 = v546;
                }
                v9174 = v9175;
            } else {
                v9174 = v24;
            }
            let v549 = if v548 == v4 { 1.0 } else { 0.0 };
            let v9178: f64;
            let v9183: f64;
            if v549 != 0.0 {
                let v551 = if v550 < v24 { 1.0 } else { 0.0 };
                let v9184: f64;
                if v551 != 0.0 {
                    v9184 = v553;
                } else {
                    let v555 = v18 + (v4 / v550);
                    v9184 = v555;
                }
                let v557 = if v556 < v24 { 1.0 } else { 0.0 };
                let v9179: f64;
                if v557 != 0.0 {
                    v9179 = v558;
                } else {
                    let v560 = v18 + (v4 / v556);
                    v9179 = v560;
                }
                v9178 = v9179;
                v9183 = v9184;
            } else {
                v9178 = v0;
                v9183 = v0;
            }
            let v3856: f64;
            let v6027: f64;
            let v6894: f64;
            let v7789: f64;
            let v7894: f64;
            let v7898: f64;
            let v8416: f64;
            let v8419: f64;
            let v8437: f64;
            let v8440: f64;
            if v5 != 0.0 {
                let v3857: f64;
                let v6028: f64;
                let v8417: f64;
                let v8420: f64;
                if v561 != 0.0 {
                    let v567: f64;
                    if v371 != 0.0 {
                        v567 = v562;
                    } else {
                        let v566 = (v563 * v140) * v565;
                        v567 = v566;
                    }
                    let v572: f64;
                    if v372 != 0.0 {
                        v572 = v568;
                    } else {
                        let v571 = (v569 * v140) * v565;
                        v572 = v571;
                    }
                    let v574 = if (if v567 > v0 { 1.0 } else { 0.0 }) != 0.0 && v367 != 0.0 { 1.0 } else { 0.0 };
                    let v8418: f64;
                    if v574 != 0.0 {
                        let v577 = (-v567) * v576;
                        v8418 = v577;
                    } else {
                        v8418 = v0;
                    }
                    let v579 = if (if v572 > v0 { 1.0 } else { 0.0 }) != 0.0 && v368 != 0.0 { 1.0 } else { 0.0 };
                    let v3858: f64;
                    let v8421: f64;
                    if v579 != 0.0 {
                        let v582 = (-v572) * v581;
                        v3858 = v0;
                        v8421 = v582;
                    } else {
                        v3858 = v572;
                        v8421 = v0;
                    }
                    v3857 = v3858;
                    v6028 = v567;
                    v8417 = v8418;
                    v8420 = v8421;
                } else {
                    v3857 = v0;
                    v6028 = v0;
                    v8417 = v0;
                    v8420 = v0;
                }
                let v583 = if v565 > v128 { 1.0 } else { 0.0 };
                let v586: f64;
                if v583 != 0.0 {
                    let v585 = v8 * (v565 - v128);
                    v586 = v585;
                } else {
                    v586 = v0;
                }
                let v587 = if v369 == v0 { 1.0 } else { 0.0 };
                let v589: f64;
                if v587 != 0.0 {
                    v589 = v586;
                } else {
                    v589 = v378;
                }
                let v588 = if v370 == v0 { 1.0 } else { 0.0 };
                let v592: f64;
                if v588 != 0.0 {
                    v592 = v586;
                } else {
                    v592 = v379;
                }
                let v590 = v140 * v589;
                let v591 = v162 + v590;
                let v593 = v140 * v592;
                let v594 = v162 + v593;
                let v595 = v164 + v590;
                let v596 = v164 + v593;
                v3856 = v3857;
                v6027 = v6028;
                v6894 = v596;
                v7789 = v595;
                v7894 = v591;
                v7898 = v594;
                v8416 = v8417;
                v8419 = v8420;
                v8437 = v589;
                v8440 = v592;
            } else {
                v3856 = v0;
                v6027 = v0;
                v6894 = v0;
                v7789 = v0;
                v7894 = v0;
                v7898 = v0;
                v8416 = v0;
                v8419 = v0;
                v8437 = v378;
                v8440 = v379;
            }
            let v600 = v361 * (v597 - v598);
            let v10357 = ((Lanes([v9361[0], 0.0])) - (Lanes([0.0, v9362[0]]))) * v361;
            let v603 = v361 * (v601 - v598);
            let v10361 = ((Lanes([0.0, v9363[0]])) - (Lanes([v9362[0], 0.0]))) * v361;
            let v606 = v361 * (v604 - v598);
            let v10365 = ((Lanes([0.0, v9364[0]])) - (Lanes([v9362[0], 0.0]))) * v361;
            let v7874: f64;
            let v7875: f64;
            let v8992: f64;
            let v8999: f64;
            let v9024: f64;
            let v9031: f64;
            let v9378: Lanes<2>;
            let v9379: Lanes<2>;
            let v9380: Lanes<1>;
            let v9381: Lanes<1>;
            let v9382: Lanes<1>;
            let v9383: Lanes<1>;
            if v5 != 0.0 {
                let v610 = v361 * (v604 - v597);
                let v10378 = ((Lanes([0.0, v9364[0]])) - (Lanes([v9361[0], 0.0]))) * v361;
                let v8993: f64;
                let v9000: f64;
                let v9384: Lanes<1>;
                let v9385: Lanes<1>;
                if v66 != 0.0 {
                    let v614 = v612 * v613;
                    let v10379 = v9367 * v612;
                    let v617 = v615 * v616;
                    let v10380 = v9368 * v615;
                    v8993 = v614;
                    v9000 = v617;
                    v9384 = v10379;
                    v9385 = v10380;
                } else {
                    v8993 = v0;
                    v9000 = v0;
                    v9384 = v10374;
                    v9385 = v10366;
                }
                v7874 = v610;
                v7875 = v606;
                v8992 = v8993;
                v8999 = v9000;
                v9024 = v0;
                v9031 = v0;
                v9378 = v10378;
                v9379 = v10365;
                v9380 = v9384;
                v9381 = v9385;
                v9382 = v10367;
                v9383 = v10368;
            } else {
                let v9001: f64;
                let v9025: f64;
                let v9032: f64;
                let v9386: Lanes<1>;
                let v9387: Lanes<1>;
                let v9388: Lanes<1>;
                if v66 != 0.0 {
                    let v620 = v618 * v619;
                    let v10369 = v9369 * v618;
                    let v623 = v621 * v622;
                    let v10370 = v9370 * v621;
                    let v625 = v624 * v616;
                    let v10371 = v9368 * v624;
                    v9001 = v625;
                    v9025 = v620;
                    v9032 = v623;
                    v9386 = v10371;
                    v9387 = v10369;
                    v9388 = v10370;
                } else {
                    v9001 = v0;
                    v9025 = v0;
                    v9032 = v0;
                    v9386 = v10366;
                    v9387 = v10367;
                    v9388 = v10368;
                }
                v7874 = v0;
                v7875 = v0;
                v8992 = v0;
                v8999 = v9001;
                v9024 = v9025;
                v9031 = v9032;
                v9378 = v10372;
                v9379 = v10373;
                v9380 = v10374;
                v9381 = v9386;
                v9382 = v9387;
                v9383 = v9388;
            }
            let v627 = if v626 > v0 { 1.0 } else { 0.0 };
            let v628 = if v30 > v0 { 1.0 } else { 0.0 };
            let v629 = if v627 != 0.0 && v628 != 0.0 { 1.0 } else { 0.0 };
            let v633: f64;
            let v9389: Lanes<1>;
            if v629 != 0.0 {
                let v631 = if v630 > v0 { 1.0 } else { 0.0 };
                let v632: f64;
                let v9390: Lanes<1>;
                if v631 != 0.0 {
                    v632 = v630;
                    v9390 = v9371;
                } else {
                    v632 = v0;
                    v9390 = v10381;
                }
                v633 = v632;
                v9389 = v9390;
            } else {
                v633 = v0;
                v9389 = v10381;
            }
            let v634 = if v600 >= v0 { 1.0 } else { 0.0 };
            let v777: f64;
            let v815: f64;
            let v819: f64;
            let v6041: f64;
            let v6043: f64;
            let v7820: f64;
            let v9391: Lanes<3>;
            let v9392: Lanes<2>;
            let v9393: Lanes<3>;
            if v634 != 0.0 {
                let v10390 = Lanes([0.0, v10365[0], v10365[1]]);
                let v10391 = Lanes([0.0, v10361[0], v10361[1]]);
                v777 = v606;
                v815 = v600;
                v819 = v603;
                v6041 = v4;
                v6043 = v0;
                v7820 = v4;
                v9391 = v10390;
                v9392 = v10357;
                v9393 = v10391;
            } else {
                let v636 = -v600;
                let v10383 = v10357 * v10382;
                let v637 = v603 - v600;
                let v10386 = (Lanes([0.0, v10361[0], v10361[1]])) - (Lanes([v10357[0], v10357[1], 0.0]));
                let v638 = v606 - v600;
                let v10389 = (Lanes([0.0, v10365[0], v10365[1]])) - (Lanes([v10357[0], v10357[1], 0.0]));
                v777 = v638;
                v815 = v636;
                v819 = v637;
                v6041 = v0;
                v6043 = v4;
                v7820 = v635;
                v9391 = v10389;
                v9392 = v10383;
                v9393 = v10386;
            }
            let v640 = if v65 >= v639 { 1.0 } else { 0.0 };
            if v640 != 0.0 {
            } else {
            }
            let v642 = if v65 >= v641 { 1.0 } else { 0.0 };
            if v642 != 0.0 {
            } else {
            }
            let v644: f64;
            if v373 != 0.0 {
                v644 = v381;
            } else {
                v644 = v643;
            }
            let v646: f64;
            if v376 != 0.0 {
                let v645 = v644 + v374;
                v646 = v645;
            } else {
                v646 = v644;
            }
            let v647 = v646 + v633;
            let v648 = v647 - v60;
            let v649 = v647 + v60;
            let v656 = (v116 - (v651 * v648)) - (v654 * (v648 * v649));
            let v10398 = ((v9389 * v651) * v10382) - (((v9389 * v649) + (v9389 * v648)) * v654);
            let v657 = v203 * v647;
            let v658 = v202 / v657;
            let v10402 = (((v9389 * v203) * v658) * v10382) / v657;
            let v659 = v658 * v658;
            let v10403 = v10402 * v658;
            let v10404 = v10403 + v10403;
            let v660 = v4 / v658;
            let v10407 = ((v10402 * v660) * v10382) / v658;
            let v679 = ((v661 * (v4 + (v662 / (v165.powf(v663))))) * (v4 + (v668 / (v138.powf(v669))))) * (v4 + (v674 / (v166.powf(v675))));
            let v682 = v4 / (v4 + v680);
            let v684 = v683 / v64;
            let v688 = if (if v684 == v0 { 1.0 } else { 0.0 }) != 0.0 && (if v686 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v690: f64;
            if v688 != 0.0 {
                v690 = v4;
            } else {
                let v689 = v684.powf(v686);
                v690 = v689;
            }
            let v693 = v679 * (v4 + (v682 * v690));
            let v694 = v647 / v60;
            let v10408 = v9389 / v60;
            let v697 = (v694.powf(v695)) / v693;
            let v10413 = (v10408 * (v695 * (v694.powf((v695 - v9360))))) / v693;
            let v699 = v698 * v660;
            let v10414 = v10407 * v698;
            let v706 = v74 * v694;
            let v708 = (v702 + (v703 * v694)) + (v706 * v694);
            let v10420 = (v10408 * v703) + (((v10408 * v74) * v694) + (v10408 * v706));
            let v709 = v4 - v694;
            let v10421 = v10408 * v10382;
            let v711 = v708 - (v21 * v709);
            let v712 = (v700 * v16) / v711;
            let v10426 = (((v10420 - (v10421 * v21)) * v712) * v10382) / v711;
            let v713 = v656.sqrt();
            let v10430 = v10398 * (v9360 / (v10427 * v713));
            let v714 = v656 * v713;
            let v10433 = (v10398 * v713) + (v10430 * v656);
            let v19475 = v694.sqrt();
            let v718 = v715 * (v694 * v19475);
            let v720 = (-v656) / v73;
            let v725 = ((v720 * v658) + ((v116 / v73) * v205)).exp();
            let v726 = v718 * v725;
            let v10445 = (((v10408 * (v716 * v19475)) * v715) * v725) + ((((((v10398 * v10382) / v73) * v658) + (v10402 * v720)) * v725) * v718);
            let v727 = v660.sqrt();
            let v10448 = v10407 * (v9360 / (v10427 * v727));
            let v728 = v227 * v727;
            let v10449 = v10448 * v227;
            let v729 = v728 * v728;
            let v10450 = v10449 * v728;
            let v10451 = v10450 + v10450;
            let v730 = v726 * v726;
            let v10452 = v10445 * v726;
            let v10453 = v10452 + v10452;
            let v731 = v730 * v229;
            let v10454 = v10453 * v229;
            let v761: f64;
            let v9394: Lanes<1>;
            if v180 != 0.0 {
                let v732 = v73 * v660;
                let v733 = v485 / v726;
                let v734 = v733.ln();
                let v735 = v732 * v734;
                let v10472 = ((v10407 * v73) * v734) + (((((v10445 * v733) * v10382) / v726) * (v9360 / v733)) * v732);
                v761 = v735;
                v9394 = v10472;
            } else {
                let v736 = v73 * v660;
                let v737 = v473 / v726;
                let v738 = v737.ln();
                let v739 = v736 * v738;
                let v10463 = ((v10407 * v73) * v738) + (((((v10445 * v737) * v10382) / v726) * (v9360 / v737)) * v736);
                v761 = v739;
                v9394 = v10463;
            }
            let v740 = v118 / v486;
            let v742 = (v740 * v660).sqrt();
            let v744 = v486 * v743;
            let v745 = v744 * v742;
            let v10477 = ((v10407 * v740) * (v9360 / (v10427 * v742))) * v744;
            let v753: f64;
            let v1235: f64;
            let v1257: f64;
            let v9395: Lanes<1>;
            let v9396: Lanes<1>;
            let v9397: Lanes<1>;
            if v5 != 0.0 {
                let v746 = v726 / v485;
                let v10486 = v10445 / v485;
                v753 = v746;
                v1235 = v0;
                v1257 = v0;
                v9395 = v10486;
                v9396 = v10381;
                v9397 = v10381;
            } else {
                let v747 = v73 * v207;
                let v749 = (v747 * v660).sqrt();
                let v10481 = (v10407 * v747) * (v9360 / (v10427 * v749));
                let v750 = v726 / v28;
                let v751 = v750 * v750;
                let v10483 = (v10445 / v28) * v750;
                let v10484 = v10483 + v10483;
                let v752 = v726 / v473;
                let v10485 = v10445 / v473;
                v753 = v752;
                v1235 = v749;
                v1257 = v751;
                v9395 = v10485;
                v9396 = v10481;
                v9397 = v10484;
            }
            let v754 = v753 * v753;
            let v10487 = v9395 * v753;
            let v10488 = v10487 + v10487;
            let v755 = v740 / v658;
            let v757 = (v73 * v755).sqrt();
            let v10495 = ((((v10402 * v755) * v10382) / v658) * v73) * (v9360 / (v10427 * v757));
            let v759 = v758 / v473;
            let v764 = ((v760 * v761) / v473).sqrt();
            let v765 = if v161 < v611 { 1.0 } else { 0.0 };
            let v770: f64;
            if v765 != 0.0 {
                v770 = v4;
            } else {
                v770 = v0;
            }
            let v766 = if v163 < v611 { 1.0 } else { 0.0 };
            let v769: f64;
            if v766 != 0.0 {
                v769 = v4;
            } else {
                v769 = v770;
            }
            let v767 = if v131 < v611 { 1.0 } else { 0.0 };
            let v768: f64;
            if v767 != 0.0 {
                v768 = v4;
            } else {
                v768 = v769;
            }
            if v768 != 0.0 {
            } else {
            }
            let v773: f64;
            let v774: f64;
            if v5 != 0.0 {
                v773 = v703;
                v774 = v771;
            } else {
                v773 = v771;
                v774 = v772;
            }
            let v775 = v774 * v8;
            let v776 = if v773 > v775 { 1.0 } else { 0.0 };
            let v778: f64;
            if v776 != 0.0 {
                v778 = v775;
            } else {
                v778 = v773;
            }
            let v779 = if v777 > v778 { 1.0 } else { 0.0 };
            let v826: f64;
            let v831: f64;
            let v9398: Lanes<3>;
            let v9399: Lanes<3>;
            if v779 != 0.0 {
                let v780 = v777 - v778;
                let v781 = v774 - v778;
                let v782 = v780 * v780;
                let v10497 = v9391 * v780;
                let v10498 = v10497 + v10497;
                let v783 = v781 * v781;
                let v784 = v782 * v782;
                let v10499 = v10498 * v782;
                let v786 = v784 * v782;
                let v10506 = ((((v10499 + v10499) * v782) + (v10498 * v784)) * v782) + (v10498 * v786);
                let v789 = ((v783 * v783) * v783) * v783;
                let v790 = (v786 * v782) + v789;
                let v807: f64;
                let v9400: Lanes<3>;
                if v791 != 0.0 {
                    let v801: f64;
                    if v792 != 0.0 {
                        v801 = v4;
                    } else {
                        let v802: f64;
                        if v793 != 0.0 {
                            v802 = v73;
                        } else {
                            let v803: f64;
                            if v794 != 0.0 {
                                v803 = v91;
                            } else {
                                let v804: f64;
                                if v795 != 0.0 {
                                    v804 = v85;
                                } else {
                                    v804 = v0;
                                }
                                v803 = v804;
                            }
                            v802 = v803;
                        }
                        v801 = v802;
                    }
                    let mut v796: f64 = 0.0;
                    let mut v798: f64 = 0.0;
                    let mut v9401: Lanes<3> = Lanes([0.0; 3]);
                    v796 = v0;
                    v798 = v790;
                    v9401 = v10506;
                    loop {
                        let v797 = if v796 < v801 { 1.0 } else { 0.0 };
                        if v797 == 0.0 {
                            break;
                        }
                        let v799 = v798.sqrt();
                        let v19263 = v9401 * (v9360 / (v10427 * v799));
                        let v800 = v796 + v4;
                        v796 = v800;
                        v798 = v799;
                        v9401 = v19263;
                    }
                    v807 = v798;
                    v9400 = v9401;
                } else {
                    let v806 = v790.powf(v805);
                    let v10510 = v10506 * (v805 * (v790.powf(v10507)));
                    v807 = v806;
                    v9400 = v10510;
                }
                let v808 = v4 / v807;
                let v10513 = ((v9400 * v808) * v10382) / v807;
                let v809 = v780 * v781;
                let v10517 = ((v9391 * v781) * v808) + (v10513 * v809);
                let v811 = v781 * v789;
                let v813 = (v811 * v808) / v790;
                let v10521 = ((v10513 * v811) - (v10506 * v813)) / v790;
                let v814 = v778 + (v809 * v808);
                v826 = v814;
                v831 = v813;
                v9398 = v10517;
                v9399 = v10521;
            } else {
                v826 = v777;
                v831 = v4;
                v9398 = v9391;
                v9399 = v10496;
            }
            let v817 = if v815 > v816 { 1.0 } else { 0.0 };
            let v818: f64;
            let v9402: Lanes<2>;
            if v817 != 0.0 {
                v818 = v816;
                v9402 = v10522;
            } else {
                v818 = v815;
                v9402 = v9392;
            }
            let v820 = if v819 > v816 { 1.0 } else { 0.0 };
            let v821: f64;
            let v9403: Lanes<3>;
            if v820 != 0.0 {
                v821 = v816;
                v9403 = v10523;
            } else {
                v821 = v819;
                v9403 = v9393;
            }
            let v823 = if v819 < v822 { 1.0 } else { 0.0 };
            let v825: f64;
            let v9404: Lanes<3>;
            if v823 != 0.0 {
                v825 = v824;
                v9404 = v10523;
            } else {
                v825 = v821;
                v9404 = v9403;
            }
            let v828 = if v826 < v827 { 1.0 } else { 0.0 };
            let v830: f64;
            let v9405: Lanes<3>;
            if v828 != 0.0 {
                v830 = v829;
                v9405 = v10496;
            } else {
                v830 = v826;
                v9405 = v9398;
            }
            let v10525 = v9402 * v831;
            let v834 = v73 * ((v831 * v818) / v73);
            let v10529 = (((v9399 * v818) + (Lanes([v10525[0], v10525[1], 0.0]))) / v73) * v73;
            let v836 = v834 / v835;
            let v10530 = v10529 / v835;
            let v844 = v841 + (v836 * v842);
            let v846 = v840 + (v836 * v844);
            let v848 = v839 + (v836 * v846);
            let v850 = v838 + (v836 * v848);
            let v852 = v837 + (v836 * v850);
            let v854 = v4 + (v836 * v852);
            let v855 = v835 / v854;
            let v10549 = ((((v10530 * v852) + (((v10530 * v850) + (((v10530 * v848) + (((v10530 * v846) + (((v10530 * v844) + ((v10530 * v842) * v836)) * v836)) * v836)) * v836)) * v836)) * v855) * v10382) / v854;
            let v857 = if v855 < v856 { 1.0 } else { 0.0 };
            let v858: f64;
            let v9406: Lanes<3>;
            if v857 != 0.0 {
                v858 = v856;
                v9406 = v10496;
            } else {
                v858 = v855;
                v9406 = v10549;
            }
            let v859 = v830 + v858;
            let v10550 = v9405 + v9406;
            let v861 = v818 + (v73 * v858);
            let v10552 = Lanes([v9402[0], v9402[1], 0.0]);
            let v10553 = v10552 + (v9406 * v73);
            let v862 = v825 + v858;
            let v10554 = Lanes([v9404[0], v9404[1], v9404[2], 0.0]);
            let v10556 = v10554 + (Lanes([v9406[0], v9406[1], 0.0, v9406[2]]));
            let v873: f64;
            let v983: f64;
            let v9407: Lanes<3>;
            let v9408: Lanes<3>;
            if v5 != 0.0 {
                v873 = v830;
                v983 = v859;
                v9407 = v9405;
                v9408 = v10550;
            } else {
                let v863 = if v12 < v91 { 1.0 } else { 0.0 };
                let v864: f64;
                let v9409: Lanes<3>;
                if v863 != 0.0 {
                    v864 = v830;
                    v9409 = v9405;
                } else {
                    v864 = v0;
                    v9409 = v10496;
                }
                let v865: f64;
                let v9410: Lanes<3>;
                if v863 != 0.0 {
                    v865 = v859;
                    v9410 = v10550;
                } else {
                    v865 = v0;
                    v9410 = v10496;
                }
                v873 = v864;
                v983 = v865;
                v9407 = v9409;
                v9408 = v9410;
            }
            let v867 = (v73 * v486) * v118;
            let v869 = (v867 * v123) * v123;
            let v870 = v825 - v236;
            let v871 = v73 / v869;
            let v10559 = (Lanes([v9404[0], v9404[1], 0.0, v9404[2]])) - (Lanes([0.0, 0.0, v10407[0], 0.0]));
            let v10563 = ((Lanes([v10559[0], v10559[1], v10559[2], v10559[3], 0.0])) - (Lanes([v9407[0], v9407[1], 0.0, 0.0, v9407[2]]))) * v871;
            let v876 = v4 + (v871 * ((v870 - v660) - v873));
            let v10564 = v10563 * v876;
            let v880 = ((v876 * v876) + v878).sqrt();
            let v10570 = (v10563 + ((v10564 + v10564) * (v9360 / (v10427 * v880)))) * v8;
            let v884 = (v8 * (v876 + v880)) + v883;
            let v885 = if v884 < v0 { 1.0 } else { 0.0 };
            let v886: f64;
            let v9411: Lanes<5>;
            if v885 != 0.0 {
                v886 = v0;
                v9411 = v10571;
            } else {
                v886 = v884;
                v9411 = v10570;
            }
            let v888 = (v886 + v358).sqrt();
            let v10577 = Lanes([v9404[0], v9404[1], 0.0, v9404[2], 0.0]);
            let v10580 = (v10577 + (((v9411 * (v9360 / (v10427 * v888))) * v10382) * v869)) - (Lanes([0.0, 0.0, v9394[0], 0.0, 0.0]));
            let v895 = (((v870 + (v869 * (v4 - v888))) - v761) - v74) - v894;
            let v899: f64;
            if v897 != 0.0 {
                v899 = v896;
            } else {
                v899 = v898;
            }
            let v10581 = v10580 * v895;
            let v902 = ((v895 * v895) + v899).sqrt();
            let v905 = v74 + (v8 * (v895 + v902));
            let v906 = v818 / v905;
            let v10589 = Lanes([v9402[0], v9402[1], 0.0, 0.0, 0.0]);
            let v10591 = (v10589 - (((v10580 + ((v10581 + v10581) * (v9360 / (v10427 * v902)))) * v8) * v906)) / v905;
            let v907 = v906 * v906;
            let v10592 = v10591 * v906;
            let v10593 = v10592 + v10592;
            let v10597 = v10593 * v907;
            let v913 = (((v4 + v906) + v907) + (v907 * v906)) + (v907 * v907);
            let v914 = v4 / v913;
            let v915 = v4 - v914;
            let v916 = v915 * v915;
            let v10606 = (((((((v10591 + v10593) + ((v10593 * v906) + (v10591 * v907))) + (v10597 + v10597)) * v914) * v10382) / v913) * v10382) * v915;
            let v10607 = v10606 + v10606;
            let v924 = if (if (if v917 == v0 { 1.0 } else { 0.0 }) != 0.0 && (if v919 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v922 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v930: f64;
            if v924 != 0.0 {
                v930 = v0;
            } else {
                v930 = v4;
            }
            let v927 = v504 + v236;
            let v929 = v927 + (((v867 * v504).sqrt()) / v122);
            let v931 = if v930 == v0 { 1.0 } else { 0.0 };
            let v1043: f64;
            let v1123: f64;
            let v1206: f64;
            let v9412: Lanes<4>;
            let v9413: Lanes<4>;
            let v9414: Lanes<5>;
            if v931 != 0.0 {
                let v933 = (v745 * v123) * v123;
                let v934 = v933 * v745;
                let v10650 = (((v10477 * v123) * v123) * v745) + (v10477 * v933);
                let v10651 = Lanes([0.0, 0.0, v10650[0], 0.0, 0.0]);
                v1043 = v123;
                v1123 = v122;
                v1206 = v934;
                v9412 = v10617;
                v9413 = v10617;
                v9414 = v10651;
            } else {
                let v10609 = v10554 - (Lanes([v9407[0], v9407[1], 0.0, v9407[2]]));
                let v937 = ((v825 - v873) - v929) + v922;
                let v10610 = v10609 * v937;
                let v941 = ((v937 * v937) + v939).sqrt();
                let v10616 = (v10609 + ((v10610 + v10610) * (v9360 / (v10427 * v941)))) * v8;
                let v945 = (v8 * (v937 + v941)) + v944;
                let v946 = if v945 < v0 { 1.0 } else { 0.0 };
                let v947: f64;
                let v9415: Lanes<4>;
                if v946 != 0.0 {
                    v947 = v0;
                    v9415 = v10617;
                } else {
                    v947 = v945;
                    v9415 = v10616;
                }
                let v948 = v4 / v947;
                let v10620 = ((v9415 * v948) * v10382) / v947;
                let v950 = v73 * (v929.abs());
                let v952 = (v236 - v929) + v922;
                let v953 = if v952 > v950 { 1.0 } else { 0.0 };
                let v954: f64;
                if v953 != 0.0 {
                    v954 = v952;
                } else {
                    v954 = v950;
                }
                let v955 = v4 / v954;
                let v10621 = v10620 * v10382;
                let v957 = (v955 - v948) - v24;
                let v959 = (v85 * v955) * v24;
                let v960 = if v959 > v0 { 1.0 } else { 0.0 };
                let v962: f64;
                if v960 != 0.0 {
                    v962 = v959;
                } else {
                    let v961 = -v959;
                    v962 = v961;
                }
                let v10622 = v10621 * v957;
                let v965 = ((v957 * v957) + v962).sqrt();
                let v10630 = (((v10621 + ((v10622 + v10622) * (v9360 / (v10427 * v965)))) * v8) * v10382) * v917;
                let v970 = (v917 * (v955 - (v8 * (v957 + v965)))) + v919;
                let v973 = if (v970 * v971) < v117 { 1.0 } else { 0.0 };
                let v974: f64;
                let v9416: Lanes<4>;
                if v973 != 0.0 {
                    v974 = v0;
                    v9416 = v10617;
                } else {
                    v974 = v970;
                    v9416 = v10630;
                }
                let v975 = v117 + v974;
                let v976 = v121 / v975;
                let v10633 = ((v9416 * v976) * v10382) / v975;
                let v977 = v975 / v121;
                let v10634 = v9416 / v121;
                let v978 = v745 * v745;
                let v10635 = v10477 * v745;
                let v979 = v978 * v977;
                let v10637 = (v10635 + v10635) * v977;
                let v10638 = v10634 * v978;
                let v980 = v979 * v977;
                let v10643 = v10634 * v979;
                let v10645 = (((Lanes([0.0, 0.0, v10637[0], 0.0, 0.0])) + (Lanes([v10638[0], v10638[1], 0.0, v10638[2], v10638[3]]))) * v977) + (Lanes([v10643[0], v10643[1], 0.0, v10643[2], v10643[3]]));
                v1043 = v977;
                v1123 = v976;
                v1206 = v980;
                v9412 = v10634;
                v9413 = v10633;
                v9414 = v10645;
            }
            let v981 = if v12 < v91 { 1.0 } else { 0.0 };
            let v982 = if v5 != 0.0 || v981 != 0.0 { 1.0 } else { 0.0 };
            let v1032: f64;
            let v9417: Lanes<4>;
            if v982 != 0.0 {
                let v10653 = v9408 * v10382;
                let v985 = (v8 - v983) - v525;
                let v989: f64;
                if v987 != 0.0 {
                    v989 = v986;
                } else {
                    v989 = v988;
                }
                let v10654 = v10653 * v985;
                let v992 = ((v985 * v985) + v989).sqrt();
                let v10661 = ((v10653 + ((v10654 + v10654) * (v9360 / (v10427 * v992)))) * v8) * v10382;
                let v1002 = (((((-v7) * v7) * v486) / v999) + v761) - v660;
                let v10662 = v9394 - v10407;
                let v10664 = Lanes([0.0, 0.0, v10662[0], 0.0]);
                let v10665 = (Lanes([v10661[0], v10661[1], 0.0, v10661[2]])) - v10664;
                let v1004 = ((v8 - (v8 * (v985 + v992))) - v1002) - v525;
                let v1006 = (v85 * v1002) * v525;
                let v10667 = (v10662 * v85) * v525;
                let v1007 = if v1006 > v0 { 1.0 } else { 0.0 };
                let v1009: f64;
                let v9418: Lanes<1>;
                if v1007 != 0.0 {
                    v1009 = v1006;
                    v9418 = v10667;
                } else {
                    let v1008 = -v1006;
                    let v10668 = v10667 * v10382;
                    v1009 = v1008;
                    v9418 = v10668;
                }
                let v10669 = v10665 * v1004;
                let v1012 = ((v1004 * v1004) + v1009).sqrt();
                let v1015 = v1002 + (v8 * (v1004 + v1012));
                let v10678 = v10664 + ((v10665 + (((v10669 + v10669) + (Lanes([0.0, 0.0, v9418[0], 0.0]))) * (v9360 / (v10427 * v1012)))) * v8);
                let v1016 = if v12 > v73 { 1.0 } else { 0.0 };
                let v1033: f64;
                let v9419: Lanes<4>;
                if v1016 != 0.0 {
                    let v10679 = v10678 * v10382;
                    let v1018 = (v504 - v1015) - v525;
                    let v1020 = (v85 * v504) * v525;
                    let v1021 = if v1020 > v0 { 1.0 } else { 0.0 };
                    let v1023: f64;
                    if v1021 != 0.0 {
                        v1023 = v1020;
                    } else {
                        let v1022 = -v1020;
                        v1023 = v1022;
                    }
                    let v10680 = v10679 * v1018;
                    let v1026 = ((v1018 * v1018) + v1023).sqrt();
                    let v1029 = v504 - (v8 * (v1018 + v1026));
                    let v10687 = ((v10679 + ((v10680 + v10680) * (v9360 / (v10427 * v1026)))) * v8) * v10382;
                    v1033 = v1029;
                    v9419 = v10687;
                } else {
                    v1033 = v1015;
                    v9419 = v10678;
                }
                v1032 = v1033;
                v9417 = v9419;
            } else {
                v1032 = v0;
                v9417 = v10652;
            }
            let v1078: f64;
            let v9420: Lanes<4>;
            if v981 != 0.0 {
                v1078 = v7;
                v9420 = v10652;
            } else {
                let v1031 = v1030 / v486;
                let v1036 = (v1031 * (v504 - v1032)).sqrt();
                let v10692 = ((v9417 * v10382) * v1031) * (v9360 / (v10427 * v1036));
                v1078 = v1036;
                v9420 = v10692;
            }
            let v1042: f64;
            let v9421: Lanes<4>;
            if v981 != 0.0 {
                let v1038 = (v488 * v504).sqrt();
                v1042 = v1038;
                v9421 = v10652;
            } else {
                let v1041 = (v488 * (v504 - v1032)).sqrt();
                let v10697 = ((v9417 * v10382) * v488) * (v9360 / (v10427 * v1041));
                v1042 = v1041;
                v9421 = v10697;
            }
            let v10698 = v9421 * v1043;
            let v10699 = v9412 * v1042;
            let v1046 = (v927 + (v1042 * v1043)) + v699;
            let v10704 = ((Lanes([v10698[0], v10698[1], v10698[2], 0.0, v10698[3]])) + (Lanes([v10699[0], v10699[1], 0.0, v10699[2], v10699[3]]))) + (Lanes([0.0, 0.0, v10414[0], 0.0, 0.0]));
            let v1048 = v1047 * v504;
            let v10705 = v9417 * v10382;
            let v1050 = (v1048 - v1032) - v525;
            let v10706 = v10705 * v1050;
            let v1056 = ((v1050 * v1050) + ((v1052 * v504) * v525)).sqrt();
            let v1060 = v504 - (v1048 - (v8 * (v1050 + v1056)));
            let v10714 = (((v10705 + ((v10706 + v10706) * (v9360 / (v10427 * v1056)))) * v8) * v10382) * v10382;
            let v1061 = v1060.sqrt();
            let v10717 = v10714 * (v9360 / (v10427 * v1061));
            let v1062 = if v183 != v0 { 1.0 } else { 0.0 };
            let v1132: f64;
            let v9422: Lanes<5>;
            if v1062 != 0.0 {
                let v1065 = (v1063 * v473) * v118;
                let v1071: f64;
                let v9423: Lanes<4>;
                if v981 != 0.0 {
                    let v1067 = (v1065 * v509).sqrt();
                    v1071 = v1067;
                    v9423 = v10652;
                } else {
                    let v1070 = (v1065 * (v509 - v1032)).sqrt();
                    let v10721 = (v10705 * v1065) * (v9360 / (v10427 * v1070));
                    v1071 = v1070;
                    v9423 = v10721;
                }
                let v10722 = v9423 * v1043;
                let v10723 = v9412 * v1071;
                let v1075 = v118 * v1043;
                let v1077 = v4 / (v183 * v183);
                let v1080 = (v73 * v1078) * v1077;
                let v10730 = (v9412 * v118) * v1080;
                let v10731 = ((v9420 * v73) * v1077) * v1075;
                let v1083 = v1082 - v504;
                let v1084 = (v1075 * v1080) * v1083;
                let v1085 = v1046 - ((v509 + v236) + (v1071 * v1043));
                let v1086 = v54 / v183;
                let v10738 = v10553 * v52;
                let v1090 = (v49 + (v1086 * v1060)) + (v52 * v861);
                let v1091 = v1085 * v1084;
                let v1092 = v1091 * v1090;
                let v10745 = ((v10714 * v1086) + (Lanes([v10738[0], v10738[1], 0.0, v10738[2]]))) * v1091;
                let v10747 = ((((v10704 - ((Lanes([v10722[0], v10722[1], v10722[2], 0.0, v10722[3]])) + (Lanes([v10723[0], v10723[1], 0.0, v10723[2], v10723[3]])))) * v1084) + ((((Lanes([v10730[0], v10730[1], 0.0, v10730[2], v10730[3]])) + (Lanes([v10731[0], v10731[1], v10731[2], 0.0, v10731[3]]))) * v1083) * v1085)) * v1090) + (Lanes([v10745[0], v10745[1], v10745[2], 0.0, v10745[3]]));
                v1132 = v1092;
                v9422 = v10747;
            } else {
                v1132 = v0;
                v9422 = v10571;
            }
            let v1094 = (v118 * v1078) * v73;
            let v10750 = v9412 * v1094;
            let v10751 = ((v9420 * v118) * v73) * v1043;
            let v1096 = v1082 - v504;
            let v1098 = v137 - v1097;
            let v1100 = v4 / (v1098 * v1098);
            let v1102 = ((v1043 * v1094) * v1096) * v1100;
            let v1103 = v48 / v137;
            let v10758 = v10553 * v46;
            let v1107 = (v43 + (v1103 * v1060)) + (v46 * v861);
            let v1108 = v1102 * v1107;
            let v10762 = ((v10714 * v1103) + (Lanes([v10758[0], v10758[1], 0.0, v10758[2]]))) * v1102;
            let v10764 = (((((Lanes([v10750[0], v10750[1], 0.0, v10750[2], v10750[3]])) + (Lanes([v10751[0], v10751[1], v10751[2], 0.0, v10751[3]]))) * v1096) * v1100) * v1107) + (Lanes([v10762[0], v10762[1], v10762[2], 0.0, v10762[3]]));
            let v1110 = if v1109 > v0 { 1.0 } else { 0.0 };
            let v1135: f64;
            let v9424: Lanes<4>;
            if v1110 != 0.0 {
                let v10765 = v10398 + v9394;
                let v10766 = v10553 * v1115;
                let v1121 = (v1109 * v7) / ((v137 * v8) + v42);
                let v1122 = (((v656 + v761) - (v73 * v1112)) + (v1115 * v861)) * v1121;
                let v10770 = ((Lanes([0.0, 0.0, v10765[0], 0.0])) + (Lanes([v10766[0], v10766[1], 0.0, v10766[2]]))) * v1121;
                v1135 = v1122;
                v9424 = v10770;
            } else {
                v1135 = v0;
                v9424 = v10652;
            }
            let v1125 = v1123 + (v40 / v161);
            let v1126 = v4 / v1125;
            let v1127 = v1043 - v1126;
            let v10775 = v9421 * v1127;
            let v10776 = (v9412 - (((v9413 * v1126) * v10382) / v1125)) * v1042;
            let v1133 = v1108 + v1132;
            let v10780 = v10764 + v9422;
            let v10783 = (v10780 + ((Lanes([v10775[0], v10775[1], v10775[2], 0.0, v10775[3]])) + (Lanes([v10776[0], v10776[1], 0.0, v10776[2], v10776[3]])))) + (Lanes([v9424[0], v9424[1], v9424[2], 0.0, v9424[3]]));
            let v1137 = ((v1133 + ((v1042 * v1127) + (v1129 / v165))) + v1135) + v245;
            let v1138 = v1046 - v1137;
            let v1139 = if v234 == v0 { 1.0 } else { 0.0 };
            let v1140: f64;
            if v1139 != 0.0 {
                v1140 = v0;
            } else {
                v1140 = v4;
            }
            let v1141 = if v1140 == v0 { 1.0 } else { 0.0 };
            let v1194: f64;
            let v9425: Lanes<4>;
            if v1141 != 0.0 {
                v1194 = v0;
                v9425 = v10617;
            } else {
                let v1143 = v862 - v1142;
                let v1145 = if v1143 < v1144 { 1.0 } else { 0.0 };
                let v1167: f64;
                let v9426: Lanes<4>;
                if v1145 != 0.0 {
                    v1167 = v0;
                    v9426 = v10617;
                } else {
                    let v1146 = if v1143 < v0 { 1.0 } else { 0.0 };
                    let v1168: f64;
                    let v9427: Lanes<4>;
                    if v1146 != 0.0 {
                        let v1151 = v1147 + (v1143 * v1149);
                        let v1153 = v4 + (v1143 * v1151);
                        let v10800 = (v10556 * v1153) + (((v10556 * v1151) + ((v10556 * v1149) * v1143)) * v1143);
                        let v1155 = v4 + (v1143 * v1153);
                        v1168 = v1155;
                        v9427 = v10800;
                    } else {
                        let v1160 = v1157 + (v1143 * v1158);
                        let v1162 = v1156 + (v1143 * v1160);
                        let v1164 = v4 + (v1143 * v1162);
                        let v10793 = (v10556 * v1164) + (((v10556 * v1162) + (((v10556 * v1160) + ((v10556 * v1158) * v1143)) * v1143)) * v1143);
                        let v1166 = v4 + (v1143 * v1164);
                        v1168 = v1166;
                        v9427 = v10793;
                    }
                    v1167 = v1168;
                    v9426 = v9427;
                }
                let v1169 = v1167 - v4;
                let v10801 = v9426 * v1169;
                let v1173 = ((v1169 * v1169) + v1171).sqrt();
                let v10807 = (v9426 + ((v10801 + v10801) * (v9360 / (v10427 * v1173)))) * v8;
                let v1177 = (v8 * (v1169 + v1173)) + v1176;
                let v1178 = if v1177 < v0 { 1.0 } else { 0.0 };
                let v1179: f64;
                let v9428: Lanes<4>;
                if v1178 != 0.0 {
                    v1179 = v0;
                    v9428 = v10617;
                } else {
                    v1179 = v1177;
                    v9428 = v10807;
                }
                let v10809 = (v9428 * v235) * v10382;
                let v1182 = (v4 - (v1179 * v235)) - v894;
                let v1186: f64;
                if v1184 != 0.0 {
                    v1186 = v1183;
                } else {
                    v1186 = v1185;
                }
                let v10810 = v10809 * v1182;
                let v1189 = ((v1182 * v1182) + v1186).sqrt();
                let v1192 = v4 - (v8 * (v1182 + v1189));
                let v10817 = ((v10809 + ((v10810 + v10810) * (v9360 / (v10427 * v1189)))) * v8) * v10382;
                v1194 = v1192;
                v9425 = v10817;
            }
            let v1195 = (v870 + v1137) - v1194;
            let v10819 = Lanes([v9425[0], v9425[1], 0.0, v9425[2], v9425[3]]);
            let v10820 = (v10577 + v10783) - v10819;
            let v1197 = (v473 / v28).ln();
            let v1198 = v660 * v1197;
            let v10821 = v10407 * v1197;
            let v1200 = (v236 - v1137) + v1194;
            let v1201 = v745 * v1043;
            let v10822 = v10477 * v1043;
            let v10823 = v9412 * v745;
            let v10826 = (Lanes([0.0, 0.0, v10822[0], 0.0, 0.0])) + (Lanes([v10823[0], v10823[1], 0.0, v10823[2], v10823[3]]));
            let v1202 = v1201 * v1201;
            let v10827 = v10826 * v1201;
            let v10828 = v10827 + v10827;
            let v4300: f64;
            let v4302: f64;
            let v4306: f64;
            let v4309: f64;
            let v4320: f64;
            let v4331: f64;
            let v4335: f64;
            let v4343: f64;
            let v4376: f64;
            let v4416: f64;
            let v4423: f64;
            let v4433: f64;
            let v4434: f64;
            let v4440: f64;
            let v4632: f64;
            let v4730: f64;
            let v4782: f64;
            let v4838: f64;
            let v4959: f64;
            let v4968: f64;
            let v4972: f64;
            let v5088: f64;
            let v5496: f64;
            let v5638: f64;
            let v5716: f64;
            let v5776: f64;
            let v8299: f64;
            let v8476: f64;
            let v8481: f64;
            let v8486: f64;
            let v8492: f64;
            let v8559: f64;
            let v8571: f64;
            let v9206: f64;
            let v9429: Lanes<6>;
            let v9430: Lanes<6>;
            let v9431: Lanes<6>;
            let v9432: Lanes<6>;
            let v9433: Lanes<6>;
            let v9434: Lanes<6>;
            let v9435: Lanes<6>;
            let v9436: Lanes<6>;
            let v9437: Lanes<6>;
            let v9438: Lanes<6>;
            let v9439: Lanes<6>;
            let v9440: Lanes<6>;
            let v9441: Lanes<6>;
            let v9442: Lanes<1>;
            let v9443: Lanes<1>;
            let v9444: Lanes<6>;
            let v9445: Lanes<5>;
            let v9446: Lanes<4>;
            let v9447: Lanes<5>;
            let v9448: Lanes<5>;
            let v9449: Lanes<6>;
            let v9450: Lanes<5>;
            let v9451: Lanes<6>;
            let v9452: Lanes<6>;
            let v9453: Lanes<6>;
            let v9454: Lanes<6>;
            let v9455: Lanes<6>;
            let v9456: Lanes<6>;
            let v9457: Lanes<6>;
            let v9458: Lanes<6>;
            let v9459: Lanes<6>;
            if v148 != 0.0 {
                let v1204 = v761 + v4;
                let v1205 = v4 / v754;
                let v11917 = ((v10488 * v1205) * v10382) / v754;
                let v1207 = v1205 / v1206;
                let v11921 = ((Lanes([0.0, 0.0, v11917[0], 0.0, 0.0])) - (v9414 * v1207)) / v1206;
                let v1208 = v1207 * v1204;
                let v11923 = v9394 * v1207;
                let v1209 = v1208 * v1204;
                let v11927 = v9394 * v1208;
                let v1210 = v73 / v1204;
                let v1211 = v658 + v1210;
                let v1213 = (v1209.ln()) / v1211;
                let v11936 = (v10402 + (((v9394 * v1210) * v10382) / v1204)) * v1213;
                let v1215 = (v759 * v1213).sqrt();
                let v11943 = ((((((((v11921 * v1204) + (Lanes([0.0, 0.0, v11923[0], 0.0, 0.0]))) * v1204) + (Lanes([0.0, 0.0, v11927[0], 0.0, 0.0]))) * (v9360 / v1209)) - (Lanes([0.0, 0.0, v11936[0], 0.0, 0.0]))) / v1211) * v759) * (v9360 / (v10427 * v1215));
                let v1216 = if v1215 > v7 { 1.0 } else { 0.0 };
                let v1217: f64;
                let v9460: Lanes<5>;
                if v1216 != 0.0 {
                    v1217 = v7;
                    v9460 = v10571;
                } else {
                    v1217 = v1215;
                    v9460 = v11943;
                }
                let v1219 = v1218 * v473;
                let v1220 = v1219 * v1217;
                let v11944 = v9460 * v1219;
                let v1223 = (v1221 * v473) * v7;
                let v1224 = -v1223;
                let v1225 = v1224 * v525;
                let v1227 = v1224 * v1226;
                let v1239: f64;
                let v9461: Lanes<4>;
                if v1228 != 0.0 {
                    let v1229 = v859 + v1198;
                    let v11950 = (Lanes([v10550[0], v10550[1], 0.0, v10550[2]])) + (Lanes([0.0, 0.0, v10821[0], 0.0]));
                    v1239 = v1229;
                    v9461 = v11950;
                } else {
                    let v1230 = v830 + v1198;
                    let v11947 = (Lanes([v9405[0], v9405[1], 0.0, v9405[2]])) + (Lanes([0.0, 0.0, v10821[0], 0.0]));
                    v1239 = v1230;
                    v9461 = v11947;
                }
                let v1234 = (v73 / v658) * ((v28 / v726).ln());
                let v11951 = v9396 * v1235;
                let v1238 = ((v1235 * v1235) * v127) * v127;
                let v11954 = ((v11951 + v11951) * v127) * v127;
                let v1240 = -v1239;
                let v11955 = v9461 * v10382;
                let v1242 = v1238 * v658;
                let v11959 = (v11954 * v658) + (v10402 * v1238);
                let v1243 = (v73 * v1240) + v1242;
                let v11961 = (v11955 * v73) + (Lanes([0.0, 0.0, v11959[0], 0.0]));
                let v1245 = v1240 * v1240;
                let v11962 = v11955 * v1240;
                let v11963 = v11962 + v11962;
                let v11966 = (v11963 + (Lanes([0.0, 0.0, v11954[0], 0.0]))) * v85;
                let v1248 = (v1243 * v1243) - (v85 * (v1245 + v1238));
                let v1250 = if v1248 >= v1249 { 1.0 } else { 0.0 };
                let v1252: f64;
                if v1250 != 0.0 {
                    v1252 = v1248;
                } else {
                    v1252 = v1251;
                }
                let v1255 = (v1243 - (v1252.sqrt())) / v73;
                let v1256 = v1245 / v1238;
                let v11967 = v11954 * v1256;
                let v11970 = (v11963 - (Lanes([0.0, 0.0, v11967[0], 0.0]))) / v1238;
                let v1258 = v1256 / v1257;
                let v11971 = v9397 * v1258;
                let v11972 = Lanes([0.0, 0.0, v11971[0], 0.0]);
                let v11973 = v9360 / v1258;
                let v1260 = v73 / v1240;
                let v1261 = v658 + v1260;
                let v1262 = (v1258.ln()) / v1261;
                let v11979 = ((Lanes([0.0, 0.0, v10402[0], 0.0])) + (((v11955 * v1260) * v10382) / v1240)) * v1262;
                let v1263 = if v1255 < v1234 { 1.0 } else { 0.0 };
                let v1379: f64;
                if v1263 != 0.0 {
                    v1379 = v1255;
                } else {
                    let v1266 = (v1262 - v1255) - v1265;
                    let v1268 = (v85 * v1262) * v1265;
                    let v1269 = if v1268 > v0 { 1.0 } else { 0.0 };
                    let v1271: f64;
                    if v1269 != 0.0 {
                        v1271 = v1268;
                    } else {
                        let v1270 = -v1268;
                        v1271 = v1270;
                    }
                    let v1277 = v1262 - (v8 * (v1266 + (((v1266 * v1266) + v1271).sqrt())));
                    v1379 = v1277;
                }
                let mut v1278: f64 = 0.0;
                let mut v1280: f64 = 0.0;
                let mut v1380: f64 = 0.0;
                let mut v1504: f64 = 0.0;
                v1278 = v0;
                v1280 = v1379;
                v1380 = v0;
                v1504 = v0;
                loop {
                    let v1279 = if v1278 < v13 { 1.0 } else { 0.0 };
                    if v1279 == 0.0 {
                        break;
                    }
                    let v1281 = v658 * v1280;
                    let v1283 = (-v1281).exp();
                    let v1284 = if v1280 > v611 { 1.0 } else { 0.0 };
                    let v1318: f64;
                    let v1351: f64;
                    if v1284 != 0.0 {
                        let v1285 = v1281.exp();
                        let v1293 = (-v1235) * ((((v1283 + v1281) - v4) + (v1257 * (v1285 - v4))).sqrt());
                        let v1299 = (v207 / v1293) * (((-v1283) + v4) + (v1257 * v1285));
                        v1318 = v1293;
                        v1351 = v1299;
                    } else {
                        let v1301 = if v1280 < v1300 { 1.0 } else { 0.0 };
                        let v1319: f64;
                        let v1352: f64;
                        if v1301 != 0.0 {
                            let v1305 = v1235 * (((v1283 + v1281) - v4).sqrt());
                            let v1309 = (v207 / v1305) * ((-v1283) + v4);
                            v1319 = v1305;
                            v1352 = v1309;
                        } else {
                            let v1314 = ((-((v207 / v658).sqrt())) * v658) * v1280;
                            let v1317 = -((v207 * v658).sqrt());
                            v1319 = v1314;
                            v1352 = v1317;
                        }
                        v1318 = v1319;
                        v1351 = v1352;
                    }
                    let v1324 = ((v1318 * v1318) + ((v85 * v1225) * v1225)).sqrt();
                    let v1327 = v8 * (v4 + (v1318 / v1324));
                    let v1331 = (v8 * (v1318 + v1324)) + (v531 * v1225);
                    let v1332 = if v1331 < v0 { 1.0 } else { 0.0 };
                    let v1333: f64;
                    let v1350: f64;
                    if v1332 != 0.0 {
                        v1333 = v0;
                        v1350 = v0;
                    } else {
                        v1333 = v1331;
                        v1350 = v1327;
                    }
                    let v1335 = (v1224 - v1333) - v1227;
                    let v1337 = (v85 * v1224) * v1227;
                    let v1338 = if v1337 > v0 { 1.0 } else { 0.0 };
                    let v1340: f64;
                    if v1338 != 0.0 {
                        v1340 = v1337;
                    } else {
                        let v1339 = -v1337;
                        v1340 = v1339;
                    }
                    let v1343 = ((v1335 * v1335) + v1340).sqrt();
                    let v1349 = v1224 - (v8 * (v1335 + v1343));
                    let v1359 = ((((v1349 * v1349) / v73) / v118) / v202) / v473;
                    let v1373 = v1280 - (((((-v1280) + (v1318 / v125)) - v1239) + v1359) / ((v1368 + (v1351 / v125)) + (((v73 * v1359) * (v1350 * (v1351 * (v8 * (v4 + (v1335 / v1343)))))) / v1349)));
                    let v1376 = if ((v1373 - v1280).abs()) < v856 { 1.0 } else { 0.0 };
                    let v1377: f64;
                    if v1376 != 0.0 {
                        v1377 = v13;
                    } else {
                        v1377 = v1278;
                    }
                    let v1378 = v1377 + v4;
                    v1278 = v1378;
                    v1280 = v1373;
                    v1380 = v1359;
                    v1504 = v1318;
                }
                let v1387 = if (((v1381 * v1380) / v473).sqrt()) > (v1385 * v7) { 1.0 } else { 0.0 };
                let v1569: f64;
                let v1883: f64;
                let v9462: Lanes<5>;
                if v1387 != 0.0 {
                    let v1388 = v4 / v1123;
                    let v11982 = ((v9413 * v1388) * v10382) / v1123;
                    let v1389 = v7 / v118;
                    let v1390 = v4 / v125;
                    let v1392 = (v1388 + v1389) + v1390;
                    let v1393 = v4 / v1392;
                    let v11983 = v11982 * v1393;
                    let v11985 = (v11983 * v10382) / v1392;
                    let v1395 = v4 - (v1393 * v1388);
                    let v1399 = v1240 + ((v1390 + (v8 * v1389)) * v1224);
                    let v1400 = v1393 * v1399;
                    let v11989 = v11985 * v1399;
                    let v11990 = v11955 * v1393;
                    let v11994 = v11982 * v1400;
                    let v1402 = (v1388 * v1400) / v1395;
                    let v11998 = (((v11985 * v1388) + v11983) * v10382) * v1402;
                    let v12001 = (((Lanes([v11994[0], v11994[1], 0.0, v11994[2], v11994[3]])) + (((Lanes([v11989[0], v11989[1], 0.0, v11989[2], v11989[3]])) + (Lanes([v11990[0], v11990[1], v11990[2], 0.0, v11990[3]]))) * v1388)) - (Lanes([v11998[0], v11998[1], 0.0, v11998[2], v11998[3]]))) / v1395;
                    let v1403 = v1200 + v1402;
                    v1569 = v1402;
                    v1883 = v1403;
                    v9462 = v12001;
                } else {
                    v1569 = v0;
                    v1883 = v1200;
                    v9462 = v10571;
                }
                let v1404 = v834 / v74;
                let v12002 = v10529 / v74;
                let v1412 = v1409 + (v1404 * v1410);
                let v1414 = v1408 + (v1404 * v1412);
                let v1416 = v1407 + (v1404 * v1414);
                let v1418 = v1406 + (v1404 * v1416);
                let v1420 = v1405 + (v1404 * v1418);
                let v1422 = v4 + (v1404 * v1420);
                let v1423 = v74 / v1422;
                let v12021 = ((((v12002 * v1420) + (((v12002 * v1418) + (((v12002 * v1416) + (((v12002 * v1414) + (((v12002 * v1412) + ((v12002 * v1410) * v1404)) * v1404)) * v1404)) * v1404)) * v1404)) * v1423) * v10382) / v1422;
                let v1424 = if v1423 < v856 { 1.0 } else { 0.0 };
                let v1425: f64;
                let v9463: Lanes<3>;
                if v1424 != 0.0 {
                    v1425 = v856;
                    v9463 = v10496;
                } else {
                    v1425 = v1423;
                    v9463 = v12021;
                }
                let v12023 = v10554 + (Lanes([v9463[0], v9463[1], 0.0, v9463[2]]));
                let v1429 = (((v825 + v1425) - v236) + v1137) - v1194;
                let v1430 = v716 * v761;
                let v1431 = v1217 / v1430;
                let v12028 = (v9394 * v716) * v1431;
                let v1432 = v1431 * v1429;
                let v12034 = (((v9460 - (Lanes([0.0, 0.0, v12028[0], 0.0, 0.0]))) / v1430) * v1429) + ((((Lanes([v12023[0], v12023[1], 0.0, v12023[2], v12023[3]])) + v10783) - v10819) * v1431);
                let v1433 = v7 * v1203;
                let v1436 = if (if v1432 < v1433 { 1.0 } else { 0.0 }) != 0.0 && (if v1433 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v1464: f64;
                let v9464: Lanes<5>;
                if v1436 != 0.0 {
                    let v1437 = v1433 - v1432;
                    let v12035 = v12034 * v10382;
                    let v1438 = v1437 * v1437;
                    let v12036 = v12035 * v1437;
                    let v1439 = v1433 * v1433;
                    let v12038 = (v12036 + v12036) * v1438;
                    let v12039 = v12038 + v12038;
                    let v1442 = (v1438 * v1438) + (v1439 * v1439);
                    let v1459: f64;
                    let v9465: Lanes<5>;
                    if v1443 != 0.0 {
                        let v1453: f64;
                        if v1444 != 0.0 {
                            v1453 = v4;
                        } else {
                            let v1454: f64;
                            if v1445 != 0.0 {
                                v1454 = v73;
                            } else {
                                let v1455: f64;
                                if v1446 != 0.0 {
                                    v1455 = v91;
                                } else {
                                    let v1456: f64;
                                    if v1447 != 0.0 {
                                        v1456 = v85;
                                    } else {
                                        v1456 = v0;
                                    }
                                    v1455 = v1456;
                                }
                                v1454 = v1455;
                            }
                            v1453 = v1454;
                        }
                        let mut v1448: f64 = 0.0;
                        let mut v1450: f64 = 0.0;
                        let mut v9466: Lanes<5> = Lanes([0.0; 5]);
                        v1448 = v0;
                        v1450 = v1442;
                        v9466 = v12039;
                        loop {
                            let v1449 = if v1448 < v1453 { 1.0 } else { 0.0 };
                            if v1449 == 0.0 {
                                break;
                            }
                            let v1451 = v1450.sqrt();
                            let v19260 = v9466 * (v9360 / (v10427 * v1451));
                            let v1452 = v1448 + v4;
                            v1448 = v1452;
                            v1450 = v1451;
                            v9466 = v19260;
                        }
                        v1459 = v1450;
                        v9465 = v9466;
                    } else {
                        let v1458 = v1442.powf(v1457);
                        let v12043 = v12039 * (v1457 * (v1442.powf(v12040)));
                        v1459 = v1458;
                        v9465 = v12043;
                    }
                    let v1460 = v4 / v1459;
                    let v1461 = v1437 * v1433;
                    let v1463 = v1433 - (v1461 * v1460);
                    let v12051 = (((v12035 * v1433) * v1460) + ((((v9465 * v1460) * v10382) / v1459) * v1461)) * v10382;
                    v1464 = v1463;
                    v9464 = v12051;
                } else {
                    v1464 = v1432;
                    v9464 = v12034;
                }
                let v1465 = v1217 - v7;
                let v1468 = if (if v1464 > v1465 { 1.0 } else { 0.0 }) != 0.0 && (if v7 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v1497: f64;
                let v9467: Lanes<5>;
                if v1468 != 0.0 {
                    let v12052 = v9464 - v9460;
                    let v1470 = (v1464 - v1217) + v7;
                    let v1471 = v1470 * v1470;
                    let v12053 = v12052 * v1470;
                    let v1472 = v7 * v7;
                    let v12055 = (v12053 + v12053) * v1471;
                    let v12056 = v12055 + v12055;
                    let v1475 = (v1471 * v1471) + (v1472 * v1472);
                    let v1492: f64;
                    let v9468: Lanes<5>;
                    if v1476 != 0.0 {
                        let v1486: f64;
                        if v1477 != 0.0 {
                            v1486 = v4;
                        } else {
                            let v1487: f64;
                            if v1478 != 0.0 {
                                v1487 = v73;
                            } else {
                                let v1488: f64;
                                if v1479 != 0.0 {
                                    v1488 = v91;
                                } else {
                                    let v1489: f64;
                                    if v1480 != 0.0 {
                                        v1489 = v85;
                                    } else {
                                        v1489 = v0;
                                    }
                                    v1488 = v1489;
                                }
                                v1487 = v1488;
                            }
                            v1486 = v1487;
                        }
                        let mut v1481: f64 = 0.0;
                        let mut v1483: f64 = 0.0;
                        let mut v9469: Lanes<5> = Lanes([0.0; 5]);
                        v1481 = v0;
                        v1483 = v1475;
                        v9469 = v12056;
                        loop {
                            let v1482 = if v1481 < v1486 { 1.0 } else { 0.0 };
                            if v1482 == 0.0 {
                                break;
                            }
                            let v1484 = v1483.sqrt();
                            let v19257 = v9469 * (v9360 / (v10427 * v1484));
                            let v1485 = v1481 + v4;
                            v1481 = v1485;
                            v1483 = v1484;
                            v9469 = v19257;
                        }
                        v1492 = v1483;
                        v9468 = v9469;
                    } else {
                        let v1491 = v1475.powf(v1490);
                        let v12060 = v12056 * (v1490 * (v1475.powf(v12057)));
                        v1492 = v1491;
                        v9468 = v12060;
                    }
                    let v1493 = v4 / v1492;
                    let v1494 = v1470 * v7;
                    let v1496 = v1465 + (v1494 * v1493);
                    let v12068 = v9460 + (((v12052 * v7) * v1493) + ((((v9468 * v1493) * v10382) / v1492) * v1494));
                    v1497 = v1496;
                    v9467 = v12068;
                } else {
                    v1497 = v1464;
                    v9467 = v9464;
                }
                let v1499 = (-v1497) * v486;
                let v12070 = (v9467 * v10382) * v486;
                let v1507 = ((((v1224 * v7) / v73) / v118) + v660) - ((v1504 * v7) / v118);
                let v2253: f64;
                let v2254: f64;
                let v2255: f64;
                let v2580: f64;
                let v2595: f64;
                let v2673: f64;
                let v3326: f64;
                let v5089: f64;
                let v9470: Lanes<5>;
                let v9471: Lanes<5>;
                let v9472: Lanes<5>;
                let v9473: Lanes<5>;
                let v9474: Lanes<5>;
                let v9475: Lanes<5>;
                if v1508 != 0.0 {
                    let v1509 = if v0 < v1507 { 1.0 } else { 0.0 };
                    let v1510: f64;
                    if v1509 != 0.0 {
                        v1510 = v4;
                    } else {
                        v1510 = v73;
                    }
                    v2253 = v0;
                    v2254 = v0;
                    v2255 = v0;
                    v2580 = v1510;
                    v2595 = v0;
                    v2673 = v0;
                    v3326 = v0;
                    v5089 = v0;
                    v9470 = v10571;
                    v9471 = v10571;
                    v9472 = v10571;
                    v9473 = v10571;
                    v9474 = v10571;
                    v9475 = v10571;
                } else {
                    let v1516 = v4 + ((v85 * ((v658 * v1195) - v4)) / (v1202 * v659));
                    let v1518 = if v1516 >= v1517 { 1.0 } else { 0.0 };
                    let v1520: f64;
                    if v1518 != 0.0 {
                        v1520 = v1516;
                    } else {
                        v1520 = v1519;
                    }
                    let v1526 = v1195 + (((v1202 * v658) * v8) * (v4 - (v1520.sqrt())));
                    let v1528 = if (v658 * v1526) < v91 { 1.0 } else { 0.0 };
                    let v1607: f64;
                    if v1528 != 0.0 {
                        let v1534 = v4 / ((v1531 * v658) * v1201);
                        let v1537 = v1535 + (v91 * v1534);
                        let v1542 = (v1148 * v1534) * (v658 * (v1195 - v830));
                        let v1549 = (v1544 - (v1535 * (v1545 + v1534))) + v1542;
                        let v1558 = (((v1538 - (v1535 * v1534)) + v1542) + (((((v85 * v1537) * v1537) * v1537) + (v1549 * v1549)).sqrt())).powf(v1557);
                        let v1568 = (((v91 - ((v1559 * v1537) / (v91 * v1558))) + (v1564 * v1558)) * v660) + v830;
                        v1607 = v1568;
                    } else {
                        let v1571 = if (v825 - v1569) <= v1138 { 1.0 } else { 0.0 };
                        let v1608: f64;
                        if v1571 != 0.0 {
                            let v1573 = v7 / v118;
                            let v1574 = v4 / v125;
                            let v1586 = v1195 - (((v4 / (((v4 / v1123) + v1573) + v1574)) * ((v1195 - v1239) + ((v1574 + (v8 * v1573)) * (-v1499)))) / v1123);
                            v1608 = v1586;
                        } else {
                            let v1587 = v1195 - v1569;
                            let v1593 = (((v1207 * v1587) * v1587).ln()) / (v658 + (v73 / v1587));
                            let v1595 = (v1593 - v1526) - v1265;
                            let v1597 = (v85 * v1593) * v1265;
                            let v1598 = if v1597 > v0 { 1.0 } else { 0.0 };
                            let v1600: f64;
                            if v1598 != 0.0 {
                                v1600 = v1597;
                            } else {
                                let v1599 = -v1597;
                                v1600 = v1599;
                            }
                            let v1606 = v1593 - (v8 * (v1595 + (((v1595 * v1595) + v1600).sqrt())));
                            v1608 = v1606;
                        }
                        v1607 = v1608;
                    }
                    let v1609 = if v1607 > v0 { 1.0 } else { 0.0 };
                    let v1614: f64;
                    if v1609 != 0.0 {
                        let v1613 = ((v1610 * v1607) / v473).sqrt();
                        v1614 = v1613;
                    } else {
                        v1614 = v0;
                    }
                    let v1615 = if v1614 < v7 { 1.0 } else { 0.0 };
                    let v2581: f64;
                    if v1615 != 0.0 {
                        v2581 = v4;
                    } else {
                        v2581 = v73;
                    }
                    let v1617 = if (v825 - v1569) <= v1138 { 1.0 } else { 0.0 };
                    let v1689: f64;
                    let v1692: f64;
                    let v9476: Lanes<5>;
                    let v9477: Lanes<5>;
                    if v1617 != 0.0 {
                        let v1618 = v4 / v1123;
                        let v1619 = v7 / v118;
                        let v1620 = v4 / v125;
                        let v1622 = (v1618 + v1619) + v1620;
                        let v1623 = v4 / v1622;
                        let v1626 = v1620 + (v8 * v1619);
                        let v1629 = (v1195 - v1239) + (v1626 * (-v1499));
                        let v12140 = ((((((v9413 * v1618) * v10382) / v1123) * v1623) * v10382) / v1622) * v1629;
                        let v1631 = (v1623 * v1629) / v1123;
                        let v12144 = v9413 * v1631;
                        let v1632 = v1195 - v1631;
                        let v12148 = v10820 - ((((Lanes([v12140[0], v12140[1], 0.0, v12140[2], v12140[3]])) + (((v10820 - (Lanes([v9461[0], v9461[1], v9461[2], 0.0, v9461[3]]))) + ((v12070 * v10382) * v1626)) * v1623)) - (Lanes([v12144[0], v12144[1], 0.0, v12144[2], v12144[3]]))) / v1123);
                        v1689 = v1632;
                        v1692 = v1632;
                        v9476 = v12148;
                        v9477 = v12148;
                    } else {
                        let v1633 = v4 / v1123;
                        let v1634 = v7 / v118;
                        let v1635 = v4 / v125;
                        let v1637 = (v1633 + v1634) + v1635;
                        let v1638 = v4 / v1637;
                        let v1641 = v1635 + (v8 * v1634);
                        let v1644 = (v1195 - v1239) + (v1641 * (-v1499));
                        let v12082 = ((((((v9413 * v1633) * v10382) / v1123) * v1638) * v10382) / v1637) * v1644;
                        let v1646 = (v1638 * v1644) / v1123;
                        let v12086 = v9413 * v1646;
                        let v1647 = v1195 - v1646;
                        let v12090 = v10820 - ((((Lanes([v12082[0], v12082[1], 0.0, v12082[2], v12082[3]])) + (((v10820 - (Lanes([v9461[0], v9461[1], v9461[2], 0.0, v9461[3]]))) + ((v12070 * v10382) * v1641)) * v1638)) - (Lanes([v12086[0], v12086[1], 0.0, v12086[2], v12086[3]]))) / v1123);
                        let v1648 = v1195 - v1569;
                        let v12091 = v10820 - v9462;
                        let v1649 = if v1648 > v0 { 1.0 } else { 0.0 };
                        let v1690: f64;
                        let v9478: Lanes<5>;
                        if v1649 != 0.0 {
                            let v1650 = v1207 * v1648;
                            let v1651 = v1650 * v1648;
                            let v1652 = v73 / v1648;
                            let v1653 = v658 + v1652;
                            let v1655 = (v1651.ln()) / v1653;
                            let v1657 = v1655 * v1656;
                            let v12108 = (((((((v11921 * v1648) + (v12091 * v1207)) * v1648) + (v12091 * v1650)) * (v9360 / v1651)) - (((Lanes([0.0, 0.0, v10402[0], 0.0, 0.0])) + (((v12091 * v1652) * v10382) / v1648)) * v1655)) / v1653) * v1656;
                            let v1658 = v1657 - v703;
                            let v1661 = if (if v1647 > v1658 { 1.0 } else { 0.0 }) != 0.0 && v1660 != 0.0 { 1.0 } else { 0.0 };
                            let v1691: f64;
                            let v9479: Lanes<5>;
                            if v1661 != 0.0 {
                                let v12109 = v12090 - v12108;
                                let v1663 = (v1647 - v1657) + v703;
                                let v1664 = v1663 * v1663;
                                let v12110 = v12109 * v1663;
                                let v12112 = (v12110 + v12110) * v1664;
                                let v12113 = v12112 + v12112;
                                let v1667 = (v1664 * v1664) + v1666;
                                let v1684: f64;
                                let v9480: Lanes<5>;
                                if v1668 != 0.0 {
                                    let v1678: f64;
                                    if v1669 != 0.0 {
                                        v1678 = v4;
                                    } else {
                                        let v1679: f64;
                                        if v1670 != 0.0 {
                                            v1679 = v73;
                                        } else {
                                            let v1680: f64;
                                            if v1671 != 0.0 {
                                                v1680 = v91;
                                            } else {
                                                let v1681: f64;
                                                if v1672 != 0.0 {
                                                    v1681 = v85;
                                                } else {
                                                    v1681 = v0;
                                                }
                                                v1680 = v1681;
                                            }
                                            v1679 = v1680;
                                        }
                                        v1678 = v1679;
                                    }
                                    let mut v1673: f64 = 0.0;
                                    let mut v1675: f64 = 0.0;
                                    let mut v9481: Lanes<5> = Lanes([0.0; 5]);
                                    v1673 = v0;
                                    v1675 = v1667;
                                    v9481 = v12113;
                                    loop {
                                        let v1674 = if v1673 < v1678 { 1.0 } else { 0.0 };
                                        if v1674 == 0.0 {
                                            break;
                                        }
                                        let v1676 = v1675.sqrt();
                                        let v12128 = v9481 * (v9360 / (v10427 * v1676));
                                        let v1677 = v1673 + v4;
                                        v1673 = v1677;
                                        v1675 = v1676;
                                        v9481 = v12128;
                                    }
                                    v1684 = v1675;
                                    v9480 = v9481;
                                } else {
                                    let v1683 = v1667.powf(v1682);
                                    let v12117 = v12113 * (v1682 * (v1667.powf(v12114)));
                                    v1684 = v1683;
                                    v9480 = v12117;
                                }
                                let v1685 = v4 / v1684;
                                let v1686 = v1663 * v703;
                                let v1688 = v1658 + (v1686 * v1685);
                                let v12125 = v12108 + (((v12109 * v703) * v1685) + ((((v9480 * v1685) * v10382) / v1684) * v1686));
                                v1691 = v1688;
                                v9479 = v12125;
                            } else {
                                v1691 = v1647;
                                v9479 = v12090;
                            }
                            v1690 = v1691;
                            v9478 = v9479;
                        } else {
                            v1690 = v1647;
                            v9478 = v12090;
                        }
                        v1689 = v1690;
                        v1692 = v1647;
                        v9476 = v9478;
                        v9477 = v12090;
                    }
                    let v1693 = v8 * v1223;
                    let v1696 = (v1689 + (v1693 * v120)) - v1239;
                    let v12149 = Lanes([v9461[0], v9461[1], v9461[2], 0.0, v9461[3]]);
                    let v12150 = v9476 - v12149;
                    let v1697 = if v1696 < v0 { 1.0 } else { 0.0 };
                    let v1874: f64;
                    let v9482: Lanes<5>;
                    if v1697 != 0.0 {
                        let v1698 = v1235 * v127;
                        let v1699 = v1698 * v1698;
                        let v12201 = (v9396 * v127) * v1698;
                        let v12202 = v12201 + v12201;
                        let v12203 = v12150 * v1700;
                        let v1703 = (v1700 * v1696) + v1702;
                        let v1705 = v1703 * v525;
                        let v12204 = v12203 * v525;
                        let v1706 = (v1703 - v8) - v1705;
                        let v12205 = v12203 - v12204;
                        let v1707 = v85 * v1703;
                        let v1708 = v1707 * v1705;
                        let v12209 = ((v12203 * v85) * v1705) + (v12204 * v1707);
                        let v1709 = if v1708 > v0 { 1.0 } else { 0.0 };
                        let v1711: f64;
                        let v9483: Lanes<5>;
                        if v1709 != 0.0 {
                            v1711 = v1708;
                            v9483 = v12209;
                        } else {
                            let v1710 = -v1708;
                            let v12210 = v12209 * v10382;
                            v1711 = v1710;
                            v9483 = v12210;
                        }
                        let v12211 = v12205 * v1706;
                        let v1714 = ((v1706 * v1706) + v1711).sqrt();
                        let v1717 = v1703 - (v8 * (v1706 + v1714));
                        let v1718 = v1699 * v1717;
                        let v12220 = v12202 * v1717;
                        let v1719 = v1718 * v659;
                        let v12225 = v10404 * v1718;
                        let v12227 = (((Lanes([0.0, 0.0, v12220[0], 0.0, 0.0])) + ((v12203 - ((v12205 + (((v12211 + v12211) + v9483) * (v9360 / (v10427 * v1714)))) * v8)) * v1699)) * v659) + (Lanes([0.0, 0.0, v12225[0], 0.0, 0.0]));
                        let v1720 = v1719.sqrt();
                        let v1721 = v4 - v1720;
                        let v1723 = v4 - v1719;
                        let v1724 = (v1696 * v1721) / v1723;
                        let v12238 = (((v12150 * v1721) + (((v12227 * (v9360 / (v10427 * v1720))) * v10382) * v1696)) - ((v12227 * v10382) * v1724)) / v1723;
                        v1874 = v1724;
                        v9482 = v12238;
                    } else {
                        let v1730 = -((v1239 - v1689) - (((v1223 / v73) * v7) / v118));
                        let v12152 = (v12149 - v9476) * v10382;
                        let v1732 = (v73 * v1730) + v1242;
                        let v12155 = (v12152 * v73) + (Lanes([0.0, 0.0, v11959[0], 0.0, 0.0]));
                        let v12156 = v12155 * v1732;
                        let v1734 = v1730 * v1730;
                        let v12158 = v12152 * v1730;
                        let v12159 = v12158 + v12158;
                        let v1737 = (v1732 * v1732) - (v85 * (v1734 + v1238));
                        let v12163 = (v12156 + v12156) - ((v12159 + (Lanes([0.0, 0.0, v11954[0], 0.0, 0.0]))) * v85);
                        let v1739 = if v1737 >= v1738 { 1.0 } else { 0.0 };
                        let v1741: f64;
                        let v9484: Lanes<5>;
                        if v1739 != 0.0 {
                            v1741 = v1737;
                            v9484 = v12163;
                        } else {
                            v1741 = v1740;
                            v9484 = v10571;
                        }
                        let v1742 = v1741.sqrt();
                        let v1744 = (v1732 - v1742) / v73;
                        let v12168 = (v12155 - (v9484 * (v9360 / (v10427 * v1742)))) / v73;
                        let v1745 = v1734 / v1238;
                        let v12169 = v11954 * v1745;
                        let v1746 = v1745 / v1257;
                        let v12173 = v9397 * v1746;
                        let v1748 = v73 / v1730;
                        let v1749 = v658 + v1748;
                        let v1750 = (v1746.ln()) / v1749;
                        let v12186 = ((((((v12159 - (Lanes([0.0, 0.0, v12169[0], 0.0, 0.0]))) / v1238) - (Lanes([0.0, 0.0, v12173[0], 0.0, 0.0]))) / v1257) * (v9360 / v1746)) - (((Lanes([0.0, 0.0, v10402[0], 0.0, 0.0])) + (((v12152 * v1748) * v10382) / v1730)) * v1750)) / v1749;
                        let v1751 = if v1744 < v1234 { 1.0 } else { 0.0 };
                        let v1875: f64;
                        let v9485: Lanes<5>;
                        if v1751 != 0.0 {
                            v1875 = v1744;
                            v9485 = v12168;
                        } else {
                            let v12187 = v12186 - v12168;
                            let v1753 = (v1750 - v1744) - v1265;
                            let v1755 = (v85 * v1750) * v1265;
                            let v12189 = (v12186 * v85) * v1265;
                            let v1756 = if v1755 > v0 { 1.0 } else { 0.0 };
                            let v1758: f64;
                            let v9486: Lanes<5>;
                            if v1756 != 0.0 {
                                v1758 = v1755;
                                v9486 = v12189;
                            } else {
                                let v1757 = -v1755;
                                let v12190 = v12189 * v10382;
                                v1758 = v1757;
                                v9486 = v12190;
                            }
                            let v12191 = v12187 * v1753;
                            let v1761 = ((v1753 * v1753) + v1758).sqrt();
                            let v1764 = v1750 - (v8 * (v1753 + v1761));
                            let v12199 = v12186 - ((v12187 + (((v12191 + v12191) + v9486) * (v9360 / (v10427 * v1761)))) * v8);
                            v1875 = v1764;
                            v9485 = v12199;
                        }
                        v1874 = v1875;
                        v9482 = v9485;
                    }
                    let mut v1765: f64 = 0.0;
                    let mut v1767: f64 = 0.0;
                    let mut v1877: f64 = 0.0;
                    let mut v9487: Lanes<5> = Lanes([0.0; 5]);
                    let mut v9488: Lanes<5> = Lanes([0.0; 5]);
                    v1765 = v0;
                    v1767 = v1874;
                    v1877 = v0;
                    v9487 = v9482;
                    v9488 = v10571;
                    loop {
                        let v1766 = if v1765 < v13 { 1.0 } else { 0.0 };
                        if v1766 == 0.0 {
                            break;
                        }
                        let v1768 = v658 * v1767;
                        let v12242 = v10402 * v1767;
                        let v12245 = (Lanes([0.0, 0.0, v12242[0], 0.0, 0.0])) + (v9487 * v658);
                        let v1770 = (-v1768).exp();
                        let v12247 = (v12245 * v10382) * v1770;
                        let v1771 = if v1767 > v611 { 1.0 } else { 0.0 };
                        let v1805: f64;
                        let v1838: f64;
                        let v9489: Lanes<5>;
                        let v9490: Lanes<5>;
                        if v1771 != 0.0 {
                            let v1772 = v1768.exp();
                            let v1773 = -v1235;
                            let v1776 = v1772 - v4;
                            let v12286 = v9397 * v1776;
                            let v12287 = (v12245 * v1772) * v1257;
                            let v1779 = (((v1770 + v1768) - v4) + (v1257 * v1776)).sqrt();
                            let v1780 = v1773 * v1779;
                            let v12294 = (v9396 * v10382) * v1779;
                            let v12297 = (Lanes([0.0, 0.0, v12294[0], 0.0, 0.0])) + ((((v12247 + v12245) + ((Lanes([0.0, 0.0, v12286[0], 0.0, 0.0])) + v12287)) * (v9360 / (v10427 * v1779))) * v1773);
                            let v1781 = v207 / v1780;
                            let v12302 = v9397 * v1772;
                            let v1785 = ((-v1770) + v4) + (v1257 * v1772);
                            let v1786 = v1781 * v1785;
                            let v12308 = ((((v12297 * v1781) * v10382) / v1780) * v1785) + (((v12247 * v10382) + ((Lanes([0.0, 0.0, v12302[0], 0.0, 0.0])) + v12287)) * v1781);
                            v1805 = v1780;
                            v1838 = v1786;
                            v9489 = v12297;
                            v9490 = v12308;
                        } else {
                            let v1788 = if v1767 < v1787 { 1.0 } else { 0.0 };
                            let v1806: f64;
                            let v1839: f64;
                            let v9491: Lanes<5>;
                            let v9492: Lanes<5>;
                            if v1788 != 0.0 {
                                let v1791 = ((v1770 + v1768) - v4).sqrt();
                                let v1792 = v1235 * v1791;
                                let v12272 = v9396 * v1791;
                                let v12275 = (Lanes([0.0, 0.0, v12272[0], 0.0, 0.0])) + (((v12247 + v12245) * (v9360 / (v10427 * v1791))) * v1235);
                                let v1793 = v207 / v1792;
                                let v1795 = (-v1770) + v4;
                                let v1796 = v1793 * v1795;
                                let v12282 = ((((v12275 * v1793) * v10382) / v1792) * v1795) + ((v12247 * v10382) * v1793);
                                v1806 = v1792;
                                v1839 = v1796;
                                v9491 = v12275;
                                v9492 = v12282;
                            } else {
                                let v1797 = v207 / v658;
                                let v1798 = v1797.sqrt();
                                let v1799 = -v1798;
                                let v1800 = v1799 * v658;
                                let v1801 = v1800 * v1767;
                                let v12258 = (((((((v10402 * v1797) * v10382) / v658) * (v9360 / (v10427 * v1798))) * v10382) * v658) + (v10402 * v1799)) * v1767;
                                let v12261 = (Lanes([0.0, 0.0, v12258[0], 0.0, 0.0])) + (v9487 * v1800);
                                let v1803 = (v207 * v658).sqrt();
                                let v1804 = -v1803;
                                let v12266 = ((v10402 * v207) * (v9360 / (v10427 * v1803))) * v10382;
                                let v12267 = Lanes([0.0, 0.0, v12266[0], 0.0, 0.0]);
                                v1806 = v1801;
                                v1839 = v1804;
                                v9491 = v12261;
                                v9492 = v12267;
                            }
                            v1805 = v1806;
                            v1838 = v1839;
                            v9489 = v9491;
                            v9490 = v9492;
                        }
                        let v12309 = v9489 * v1805;
                        let v1811 = ((v1805 * v1805) + ((v85 * v1225) * v1225)).sqrt();
                        let v12313 = (v12309 + v12309) * (v9360 / (v10427 * v1811));
                        let v1812 = v1805 / v1811;
                        let v1814 = v8 * (v4 + v1812);
                        let v12317 = ((v9489 - (v12313 * v1812)) / v1811) * v8;
                        let v12319 = (v9489 + v12313) * v8;
                        let v1818 = (v8 * (v1805 + v1811)) + (v531 * v1225);
                        let v1819 = if v1818 < v0 { 1.0 } else { 0.0 };
                        let v1820: f64;
                        let v1837: f64;
                        let v9493: Lanes<5>;
                        let v9494: Lanes<5>;
                        if v1819 != 0.0 {
                            v1820 = v0;
                            v1837 = v0;
                            v9493 = v10571;
                            v9494 = v10571;
                        } else {
                            v1820 = v1818;
                            v1837 = v1814;
                            v9493 = v12319;
                            v9494 = v12317;
                        }
                        let v12320 = v9493 * v10382;
                        let v1822 = (v1224 - v1820) - v1227;
                        let v1824 = (v85 * v1224) * v1227;
                        let v1825 = if v1824 > v0 { 1.0 } else { 0.0 };
                        let v1827: f64;
                        if v1825 != 0.0 {
                            v1827 = v1824;
                        } else {
                            let v1826 = -v1824;
                            v1827 = v1826;
                        }
                        let v12321 = v12320 * v1822;
                        let v1830 = ((v1822 * v1822) + v1827).sqrt();
                        let v12325 = (v12321 + v12321) * (v9360 / (v10427 * v1830));
                        let v1831 = v1822 / v1830;
                        let v1833 = v8 * (v4 + v1831);
                        let v1836 = v1224 - (v8 * (v1822 + v1830));
                        let v12332 = ((v12320 + v12325) * v8) * v10382;
                        let v1840 = v1838 * v1833;
                        let v1841 = v1837 * v1840;
                        let v12339 = v12332 * v1836;
                        let v1846 = ((((v1836 * v1836) / v73) / v118) / v202) / v473;
                        let v12344 = ((((v12339 + v12339) / v73) / v118) / v202) / v473;
                        let v1847 = v73 * v1846;
                        let v1849 = (v1847 * v1841) / v1836;
                        let v1866 = ((v1860 + (v1838 / v125)) + ((v1838 * v7) / v118)) + v1849;
                        let v1867 = (((((v1689 - v1767) + (v1805 / v125)) + (((v1805 + (v1223 / v73)) * v7) / v118)) - v1239) + v1846) / v1866;
                        let v1868 = v1767 - v1867;
                        let v12368 = v9487 - (((((((v9476 - v9487) + (v9489 / v125)) + ((v9489 * v7) / v118)) - v12149) + v12344) - ((((v9490 / v125) + ((v9490 * v7) / v118)) + (((((v12344 * v73) * v1841) + (((v9494 * v1840) + (((v9490 * v1833) + ((((v12320 - (v12325 * v1831)) / v1830) * v8) * v1838)) * v1837)) * v1847)) - (v12332 * v1849)) / v1836)) * v1867)) / v1866);
                        let v1871 = if ((v1868 - v1767).abs()) < v525 { 1.0 } else { 0.0 };
                        let v1872: f64;
                        if v1871 != 0.0 {
                            v1872 = v13;
                        } else {
                            v1872 = v1765;
                        }
                        let v1873 = v1872 + v4;
                        v1765 = v1873;
                        v1767 = v1868;
                        v1877 = v1805;
                        v9487 = v12368;
                        v9488 = v9489;
                    }
                    let v1876 = v1239 + v1767;
                    let v12239 = v12149 + v9487;
                    let v1880 = v1689 + (v120 * (v1693 + v1877));
                    let v12241 = v9476 + (v9488 * v120);
                    v2253 = v1689;
                    v2254 = v1880;
                    v2255 = v1876;
                    v2580 = v2581;
                    v2595 = v1877;
                    v2673 = v1692;
                    v3326 = v1614;
                    v5089 = v1689;
                    v9470 = v9476;
                    v9471 = v12241;
                    v9472 = v12239;
                    v9473 = v9488;
                    v9474 = v9477;
                    v9475 = v9476;
                }
                let v1887 = if (if v1881 == v4 { 1.0 } else { 0.0 }) != 0.0 && (if v825 > (v1883 + v1884) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v2564: f64;
                let v2671: f64;
                let v4731: f64;
                let v4783: f64;
                let v5639: f64;
                let v5777: f64;
                let v9207: f64;
                let v9495: Lanes<6>;
                let v9496: Lanes<5>;
                let v9497: Lanes<1>;
                let v9498: Lanes<1>;
                let v9499: Lanes<5>;
                let v9500: Lanes<6>;
                if v1887 != 0.0 {
                    let v1890 = ((v862 - v345) + v1137) - v1194;
                    let v12371 = ((Lanes([v10556[0], v10556[1], 0.0, v10556[2], v10556[3]])) + v10783) - v10819;
                    let v1895 = ((v1892 * v473) * v118) / v658;
                    let v1896 = v1895.sqrt();
                    let v12377 = (((v10402 * v1895) * v10382) / v658) * (v9360 / (v10427 * v1896));
                    let v1898 = (v730 / v473) / v473;
                    let v12379 = (v10453 / v473) / v473;
                    let v12380 = v12377 * v1896;
                    let v12381 = v12380 + v12380;
                    let v1900 = (v1896 * v1896) / v1123;
                    let v12382 = v9413 * v1900;
                    let v1901 = v1900 / v1123;
                    let v12387 = v9413 * v1901;
                    let v12390 = ((((Lanes([0.0, 0.0, v12381[0], 0.0, 0.0])) - (Lanes([v12382[0], v12382[1], 0.0, v12382[2], v12382[3]]))) / v1123) - (Lanes([v12387[0], v12387[1], 0.0, v12387[2], v12387[3]]))) / v1123;
                    let v12392 = v10402 * v1901;
                    let v1903 = (v1901 * v658) / v73;
                    let v12395 = ((v12390 * v658) + (Lanes([0.0, 0.0, v12392[0], 0.0, 0.0]))) / v73;
                    let v12397 = v10402 * v1903;
                    let v1905 = (v1903 * v658) * v73;
                    let v12401 = v10402 * v1890;
                    let v1909 = (v85 * ((v658 * v1890) - v4)) / v1905;
                    let v1911 = (v4 + v1909).sqrt();
                    let v1912 = v4 - v1911;
                    let v1915 = v4 / v1898;
                    let v12419 = ((v12379 * v1915) * v10382) / v1898;
                    let v1916 = v1915 / v1901;
                    let v1917 = v1890 * v1890;
                    let v12424 = v12371 * v1890;
                    let v1918 = v1916 * v1917;
                    let v1920 = v73 / v1890;
                    let v1921 = v658 + v1920;
                    let v1922 = (v1918.ln()) / v1921;
                    let v12438 = (((((((Lanes([0.0, 0.0, v12419[0], 0.0, 0.0])) - (v12390 * v1916)) / v1901) * v1917) + ((v12424 + v12424) * v1916)) * (v9360 / v1918)) - (((Lanes([0.0, 0.0, v10402[0], 0.0, 0.0])) + (((v12371 * v1920) * v10382) / v1890)) * v1922)) / v1921;
                    let v12439 = v12438 - (v12371 + ((v12395 * v1912) + ((((((((Lanes([0.0, 0.0, v12401[0], 0.0, 0.0])) + (v12371 * v658)) * v85) - ((((v12395 * v658) + (Lanes([0.0, 0.0, v12397[0], 0.0, 0.0]))) * v73) * v1909)) / v1905) * (v9360 / (v10427 * v1911))) * v10382) * v1903)));
                    let v1924 = (v1922 - (v1890 + (v1903 * v1912))) - v1891;
                    let v12440 = v12439 * v1924;
                    let v1926 = v85 * v1891;
                    let v1929 = ((v1924 * v1924) + (v1926 * v1922)).sqrt();
                    let v1932 = v1922 - (v8 * (v1924 + v1929));
                    let v12449 = v12438 - ((v12439 + (((v12440 + v12440) + (v12438 * v1926)) * (v9360 / (v10427 * v1929)))) * v8);
                    let v1933 = v658 * v1932;
                    let v12450 = v10402 * v1932;
                    let v12453 = (Lanes([0.0, 0.0, v12450[0], 0.0, 0.0])) + (v12449 * v658);
                    let v1934 = v1933.exp();
                    let v1935 = v1933 - v4;
                    let v12455 = v12379 * v1934;
                    let v1937 = v1935 + (v1898 * v1934);
                    let v12459 = v12453 + ((Lanes([0.0, 0.0, v12455[0], 0.0, 0.0])) + ((v12453 * v1934) * v1898));
                    let v1940 = if (if v1937 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v1935 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v2565: f64;
                    let v2672: f64;
                    let v5640: f64;
                    let v5778: f64;
                    let v9208: f64;
                    let v9501: Lanes<6>;
                    let v9502: Lanes<5>;
                    let v9503: Lanes<5>;
                    let v9504: Lanes<6>;
                    if v1940 != 0.0 {
                        let v1941 = v1937.sqrt();
                        let v1942 = v1935.sqrt();
                        let v1943 = v1941 - v1942;
                        let v1944 = v1896 * v1943;
                        let v12467 = v12377 * v1943;
                        let v1946 = (v73 * v161) / v658;
                        let v1948 = -v658;
                        let v12474 = v10402 * v10382;
                        let v12475 = v12474 * v861;
                        let v12476 = v10553 * v1948;
                        let v1950 = (v1948 * v861).exp();
                        let v1952 = -(v1950 - v4);
                        let v1953 = v4 / v131;
                        let v1954 = v1946 * v1947;
                        let v1955 = v1954 * v1944;
                        let v12483 = ((((v10402 * v1946) * v10382) / v658) * v1947) * v1944;
                        let v12488 = ((((Lanes([0.0, 0.0, v12475[0], 0.0])) + (Lanes([v12476[0], v12476[1], 0.0, v12476[2]]))) * v1950) * v10382) * v1955;
                        let v1957 = (v1955 * v1952) * v1953;
                        let v12491 = ((((Lanes([0.0, 0.0, v12483[0], 0.0, 0.0])) + (((Lanes([0.0, 0.0, v12467[0], 0.0, 0.0])) + (((v12459 * (v9360 / (v10427 * v1941))) - (v12453 * (v9360 / (v10427 * v1942)))) * v1896)) * v1954)) * v1952) + (Lanes([v12488[0], v12488[1], v12488[2], 0.0, v12488[3]]))) * v1953;
                        let v12492 = v10402 * v1195;
                        let v1961 = v1202 * v659;
                        let v12498 = v10404 * v1202;
                        let v1962 = (v85 * ((v658 * v1195) - v4)) / v1961;
                        let v12503 = ((((Lanes([0.0, 0.0, v12492[0], 0.0, 0.0])) + (v10820 * v658)) * v85) - (((v10828 * v659) + (Lanes([0.0, 0.0, v12498[0], 0.0, 0.0]))) * v1962)) / v1961;
                        let v1963 = v4 + v1962;
                        let v1965 = if v1963 < v1964 { 1.0 } else { 0.0 };
                        let v1969: f64;
                        let v9505: Lanes<5>;
                        if v1965 != 0.0 {
                            v1969 = v1966;
                            v9505 = v10571;
                        } else {
                            v1969 = v1963;
                            v9505 = v12503;
                        }
                        let v12505 = v10402 * v1202;
                        let v1968 = (v1202 * v658) * v8;
                        let v1970 = v1969.sqrt();
                        let v1971 = v4 - v1970;
                        let v1973 = v1195 + (v1968 * v1971);
                        let v12516 = v10820 + (((((v10828 * v658) + (Lanes([0.0, 0.0, v12505[0], 0.0, 0.0]))) * v8) * v1971) + (((v9505 * (v9360 / (v10427 * v1970))) * v10382) * v1968));
                        let v1974 = v1973 - v1932;
                        let v12517 = v12516 - v12449;
                        let v1975 = if v1974 < v0 { 1.0 } else { 0.0 };
                        let v1977: f64;
                        let v9506: Lanes<5>;
                        if v1975 != 0.0 {
                            v1977 = v0;
                            v9506 = v10571;
                        } else {
                            v1977 = v1974;
                            v9506 = v12517;
                        }
                        let v1978 = v1976 * v1977;
                        let v12518 = v9506 * v1976;
                        let v12520 = v12518 - (Lanes([v10553[0], v10553[1], 0.0, 0.0, v10553[2]]));
                        let v1981 = (v1978 - v861) - v1980;
                        let v12521 = v12520 * v1981;
                        let v1986 = ((v1981 * v1981) + ((v85 * v1978) * v1980)).sqrt();
                        let v1989 = v1978 - (v8 * (v1981 + v1986));
                        let v12531 = v12518 - ((v12520 + (((v12521 + v12521) + ((v12518 * v85) * v1980)) * (v9360 / (v10427 * v1986)))) * v8);
                        let v1990 = if v1989 > v1977 { 1.0 } else { 0.0 };
                        let v1991: f64;
                        let v9507: Lanes<5>;
                        if v1990 != 0.0 {
                            v1991 = v1977;
                            v9507 = v9506;
                        } else {
                            v1991 = v1989;
                            v9507 = v12531;
                        }
                        let v1992 = v117 * v63;
                        let v1993 = v162 * v63;
                        let v1994 = v131 * v63;
                        let v1996 = if v1995 == v0 { 1.0 } else { 0.0 };
                        let v2215: f64;
                        let v9508: Lanes<5>;
                        if v1996 != 0.0 {
                            v2215 = v0;
                            v9508 = v10571;
                        } else {
                            let v2001 = ((v1998 * v202) * v1993) * v1994;
                            let v2002 = v2001 / v713;
                            let v12534 = ((v10430 * v2002) * v10382) / v713;
                            let v12535 = v9408 * v2003;
                            let v2011 = (-(((((v2003 * v983) + v1108) + v1132) + v656) + v2008)) / v1992;
                            let v12542 = (((((Lanes([v12535[0], v12535[1], 0.0, 0.0, v12535[2]])) + v10764) + v9422) + (Lanes([0.0, 0.0, v10398[0], 0.0, 0.0]))) * v10382) / v1992;
                            let mut v2012: f64 = 0.0;
                            let mut v2060: f64 = 0.0;
                            let mut v9509: Lanes<5> = Lanes([0.0; 5]);
                            v2012 = v0;
                            v2060 = v0;
                            v9509 = v10571;
                            loop {
                                let v2014 = if v2012 <= v2013 { 1.0 } else { 0.0 };
                                if v2014 == 0.0 {
                                    break;
                                }
                                let v2015 = v2012 / v63;
                                let v2019 = (v1195 + v858) - ((v1991 * v2015) + v1932);
                                let v12547 = (v10820 + (Lanes([v9406[0], v9406[1], 0.0, 0.0, v9406[2]]))) - ((v9507 * v2015) + v12449);
                                let v2021 = v4 - (v2019 / v1997);
                                let v12549 = (v12547 / v1997) * v10382;
                                let v2023 = v2011 + (v2019 / v1992);
                                let v12551 = v12542 + (v12547 / v1992);
                                let v2024 = v2023 * v2023;
                                let v12552 = v12551 * v2023;
                                let v12553 = v12552 + v12552;
                                let v12554 = v12549 * v2021;
                                let v2028 = ((v2021 * v2021) + v2026).sqrt();
                                let v12560 = (v12549 + ((v12554 + v12554) * (v9360 / (v10427 * v2028)))) * v8;
                                let v2032 = (v8 * (v2021 + v2028)) + v2031;
                                let v2033 = if v2032 < v0 { 1.0 } else { 0.0 };
                                let v2035: f64;
                                let v9510: Lanes<5>;
                                if v2033 != 0.0 {
                                    v2035 = v0;
                                    v9510 = v10571;
                                } else {
                                    v2035 = v2032;
                                    v9510 = v12560;
                                }
                                let v2036 = v2035.sqrt();
                                let v2039 = v2034 * (v4 - (v2036 * v2035));
                                let v12568 = ((((v9510 * (v9360 / (v10427 * v2036))) * v2035) + (v9510 * v2036)) * v10382) * v2034;
                                let v2041 = (-v2039) / v2023;
                                let v12572 = ((v12568 * v10382) - (v12551 * v2041)) / v2023;
                                let v2043 = if v2041 < v2042 { 1.0 } else { 0.0 };
                                let v2055: f64;
                                let v9511: Lanes<5>;
                                if v2043 != 0.0 {
                                    v2055 = v0;
                                    v9511 = v10571;
                                } else {
                                    let v2044 = v2041.exp();
                                    let v12573 = v12572 * v2044;
                                    v2055 = v2044;
                                    v9511 = v12573;
                                }
                                let v2046 = v2045 * v2002;
                                let v2047 = v2046 * v2039;
                                let v12575 = (v12534 * v2045) * v2039;
                                let v2050 = (v2047 * v2039) * v2049;
                                let v12582 = ((((Lanes([0.0, 0.0, v12575[0], 0.0, 0.0])) + (v12568 * v2046)) * v2039) + (v12568 * v2047)) * v2049;
                                let v2053 = if ((v73 * v2023) + v2039) < v0 { 1.0 } else { 0.0 };
                                let v2061: f64;
                                let v9512: Lanes<5>;
                                if v2053 != 0.0 {
                                    v2061 = v2050;
                                    v9512 = v12582;
                                } else {
                                    let v2054 = v2001 * v2024;
                                    let v2056 = v2054 * v2055;
                                    let v12586 = ((v12553 * v2001) * v2055) + (v9511 * v2054);
                                    let v2059 = if (if v2056 < v2050 { 1.0 } else { 0.0 }) != 0.0 || (if v2023 < v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v2062: f64;
                                    let v9513: Lanes<5>;
                                    if v2059 != 0.0 {
                                        v2062 = v2050;
                                        v9513 = v12582;
                                    } else {
                                        v2062 = v2056;
                                        v9513 = v12586;
                                    }
                                    v2061 = v2062;
                                    v9512 = v9513;
                                }
                                let v2063 = v2060 + v2061;
                                let v12587 = v9509 + v9512;
                                let v2064 = if v2061 < v611 { 1.0 } else { 0.0 };
                                let v2065: f64;
                                if v2064 != 0.0 {
                                    v2065 = v63;
                                } else {
                                    v2065 = v2012;
                                }
                                let v2066 = v2065 + v4;
                                v2012 = v2066;
                                v2060 = v2063;
                                v9509 = v12587;
                            }
                            v2215 = v2060;
                            v9508 = v9509;
                        }
                        let v2069 = if (if v293 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v16 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v2214: f64;
                        let v9514: Lanes<5>;
                        if v2069 != 0.0 {
                            v2214 = v0;
                            v9514 = v10571;
                        } else {
                            let v2187: f64;
                            let v9515: Lanes<5>;
                            if v277 != 0.0 {
                                let v2070 = v1123 * v1123;
                                let v12667 = v9413 * v1123;
                                let v12668 = v12667 + v12667;
                                let v2071 = v487 / v2070;
                                let v12671 = ((v12668 * v2071) * v10382) / v2070;
                                let v2072 = v73 / v487;
                                let v2073 = v2072 * v2070;
                                let v12675 = v9408 * v2075;
                                let v2077 = (v1890 - v660) - (v2075 * v983);
                                let v12678 = (v12668 * v2072) * v2077;
                                let v12681 = (Lanes([v12678[0], v12678[1], 0.0, v12678[2], v12678[3]])) + (((v12371 - (Lanes([0.0, 0.0, v10407[0], 0.0, 0.0]))) - (Lanes([v12675[0], v12675[1], 0.0, 0.0, v12675[2]]))) * v2073);
                                let v2079 = v4 + (v2073 * v2077);
                                let v12682 = v12681 * v2079;
                                let v2083 = ((v2079 * v2079) + v2081).sqrt();
                                let v12688 = (v12681 + ((v12682 + v12682) * (v9360 / (v10427 * v2083)))) * v8;
                                let v2087 = (v8 * (v2079 + v2083)) + v2086;
                                let v2088 = if v2087 < v0 { 1.0 } else { 0.0 };
                                let v2089: f64;
                                let v9516: Lanes<5>;
                                if v2088 != 0.0 {
                                    v2089 = v0;
                                    v9516 = v10571;
                                } else {
                                    v2089 = v2087;
                                    v9516 = v12688;
                                }
                                let v2091 = (v2089 + v358).sqrt();
                                let v2095 = v4 - v2091;
                                let v12694 = v12671 * v2095;
                                let v12699 = v10553 * v2098;
                                let v2104 = v2101 * v2102;
                                let v2106 = ((v2098 * v861) + v1932) - (v2104 * ((v1890 * v2092) + (v2071 * v2095)));
                                let v12703 = ((Lanes([v12699[0], v12699[1], 0.0, 0.0, v12699[2]])) + v12449) - (((v12371 * v2092) + ((Lanes([v12694[0], v12694[1], 0.0, v12694[2], v12694[3]])) + (((v9516 * (v9360 / (v10427 * v2091))) * v10382) * v2071))) * v2104);
                                let v12704 = v12703 * v2106;
                                let v2110 = ((v2106 * v2106) + v2108).sqrt();
                                let v12710 = (v12703 + ((v12704 + v12704) * (v9360 / (v10427 * v2110)))) * v8;
                                let v2114 = (v8 * (v2106 + v2110)) + v2113;
                                let v2115 = if v2114 < v0 { 1.0 } else { 0.0 };
                                let v2188: f64;
                                let v9517: Lanes<5>;
                                if v2115 != 0.0 {
                                    v2188 = v0;
                                    v9517 = v10571;
                                } else {
                                    v2188 = v2114;
                                    v9517 = v12710;
                                }
                                v2187 = v2188;
                                v9515 = v9517;
                            } else {
                                let v2118 = v2116 * v1890;
                                let v12588 = v12371 * v2116;
                                let v2119 = v1123 * v1123;
                                let v12589 = v9413 * v1123;
                                let v12590 = v12589 + v12589;
                                let v2120 = v487 / v2119;
                                let v12593 = ((v12590 * v2120) * v10382) / v2119;
                                let v2121 = v73 / v487;
                                let v2122 = v2121 * v2119;
                                let v12594 = v12590 * v2121;
                                let v12597 = v9408 * v2075;
                                let v2125 = (v2118 - v660) - (v2075 * v983);
                                let v12600 = v12594 * v2125;
                                let v12603 = (Lanes([v12600[0], v12600[1], 0.0, v12600[2], v12600[3]])) + (((v12588 - (Lanes([0.0, 0.0, v10407[0], 0.0, 0.0]))) - (Lanes([v12597[0], v12597[1], 0.0, 0.0, v12597[2]]))) * v2122);
                                let v2127 = v4 + (v2122 * v2125);
                                let v2129 = v73 * (v4 + v2122);
                                let v12604 = v12594 * v73;
                                let v2130 = v358 + v2129;
                                let v2133 = if (if v2127 < v2130 { 1.0 } else { 0.0 }) != 0.0 && (if v2129 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let v2165: f64;
                                let v9518: Lanes<5>;
                                if v2133 != 0.0 {
                                    let v2134 = v2130 - v2127;
                                    let v12605 = Lanes([v12604[0], v12604[1], 0.0, v12604[2], v12604[3]]);
                                    let v12606 = v12605 - v12603;
                                    let v2135 = v2134 * v2134;
                                    let v12607 = v12606 * v2134;
                                    let v12608 = v12607 + v12607;
                                    let v2136 = v2129 * v2129;
                                    let v12609 = v12604 * v2129;
                                    let v12610 = v12609 + v12609;
                                    let v2137 = v2135 * v2135;
                                    let v12611 = v12608 * v2135;
                                    let v2138 = v2136 * v2136;
                                    let v12613 = v12610 * v2136;
                                    let v2139 = v2137 * v2135;
                                    let v2140 = v2138 * v2136;
                                    let v12626 = ((((v12613 + v12613) * v2136) + (v12610 * v2138)) * v2136) + (v12610 * v2140);
                                    let v2143 = (v2139 * v2135) + (v2140 * v2136);
                                    let v12628 = (((((v12611 + v12611) * v2135) + (v12608 * v2137)) * v2135) + (v12608 * v2139)) + (Lanes([v12626[0], v12626[1], 0.0, v12626[2], v12626[3]]));
                                    let v2160: f64;
                                    let v9519: Lanes<5>;
                                    if v2144 != 0.0 {
                                        let v2154: f64;
                                        if v2145 != 0.0 {
                                            v2154 = v4;
                                        } else {
                                            let v2155: f64;
                                            if v2146 != 0.0 {
                                                v2155 = v73;
                                            } else {
                                                let v2156: f64;
                                                if v2147 != 0.0 {
                                                    v2156 = v91;
                                                } else {
                                                    let v2157: f64;
                                                    if v2148 != 0.0 {
                                                        v2157 = v85;
                                                    } else {
                                                        v2157 = v0;
                                                    }
                                                    v2156 = v2157;
                                                }
                                                v2155 = v2156;
                                            }
                                            v2154 = v2155;
                                        }
                                        let mut v2149: f64 = 0.0;
                                        let mut v2151: f64 = 0.0;
                                        let mut v9520: Lanes<5> = Lanes([0.0; 5]);
                                        v2149 = v0;
                                        v2151 = v2143;
                                        v9520 = v12628;
                                        loop {
                                            let v2150 = if v2149 < v2154 { 1.0 } else { 0.0 };
                                            if v2150 == 0.0 {
                                                break;
                                            }
                                            let v2152 = v2151.sqrt();
                                            let v12666 = v9520 * (v9360 / (v10427 * v2152));
                                            let v2153 = v2149 + v4;
                                            v2149 = v2153;
                                            v2151 = v2152;
                                            v9520 = v12666;
                                        }
                                        v2160 = v2151;
                                        v9519 = v9520;
                                    } else {
                                        let v2159 = v2143.powf(v2158);
                                        let v12632 = v12628 * (v2158 * (v2143.powf(v12629)));
                                        v2160 = v2159;
                                        v9519 = v12632;
                                    }
                                    let v2161 = v4 / v2160;
                                    let v2162 = v2134 * v2129;
                                    let v12637 = v12604 * v2134;
                                    let v2164 = v2130 - (v2162 * v2161);
                                    let v12643 = v12605 - ((((v12606 * v2129) + (Lanes([v12637[0], v12637[1], 0.0, v12637[2], v12637[3]]))) * v2161) + ((((v9519 * v2161) * v10382) / v2160) * v2162));
                                    v2165 = v2164;
                                    v9518 = v12643;
                                } else {
                                    v2165 = v2127;
                                    v9518 = v12603;
                                }
                                let v2166 = if v2165 <= v0 { 1.0 } else { 0.0 };
                                let v2168: f64;
                                let v9521: Lanes<5>;
                                if v2166 != 0.0 {
                                    v2168 = v0;
                                    v9521 = v10571;
                                } else {
                                    let v2167 = v2165.sqrt();
                                    let v12646 = v9518 * (v9360 / (v10427 * v2167));
                                    v2168 = v2167;
                                    v9521 = v12646;
                                }
                                let v2169 = v4 - v2168;
                                let v12648 = v12593 * v2169;
                                let v2173 = v138 / (v2101 + v138);
                                let v12653 = v10553 * v2098;
                                let v2177 = ((v2098 * v861) + v4) - (v2173 * (v2118 + (v2120 * v2169)));
                                let v12656 = (Lanes([v12653[0], v12653[1], 0.0, 0.0, v12653[2]])) - ((v12588 + ((Lanes([v12648[0], v12648[1], 0.0, v12648[2], v12648[3]])) + ((v9521 * v10382) * v2120))) * v2173);
                                let v12657 = v12656 * v2177;
                                let v2181 = ((v2177 * v2177) + v2179).sqrt();
                                let v12663 = (v12656 + ((v12657 + v12657) * (v9360 / (v10427 * v2181)))) * v8;
                                let v2185 = (v8 * (v2177 + v2181)) + v2184;
                                let v2186 = if v2185 < v0 { 1.0 } else { 0.0 };
                                let v2189: f64;
                                let v9522: Lanes<5>;
                                if v2186 != 0.0 {
                                    v2189 = v0;
                                    v9522 = v10571;
                                } else {
                                    v2189 = v2185;
                                    v9522 = v12663;
                                }
                                v2187 = v2189;
                                v9515 = v9522;
                            }
                            let v2190 = v2187 + v358;
                            let v2193 = (-v2191) / v2190;
                            let v2194 = v2193.exp();
                            let v2196 = v2195 * v2190;
                            let v2197 = v2196 * v1957;
                            let v2198 = v2197 * v2194;
                            let v12721 = ((((v9515 * v2195) * v1957) + (v12491 * v2196)) * v2194) + (((((v9515 * v2193) * v10382) / v2190) * v2194) * v2197);
                            v2214 = v2198;
                            v9514 = v12721;
                        }
                        let v2200 = if v2199 == v4 { 1.0 } else { 0.0 };
                        let v2566: f64;
                        let v9209: f64;
                        let v9523: Lanes<6>;
                        let v9524: Lanes<6>;
                        if v2200 != 0.0 {
                            let v2202 = (v202 * v7) * v162;
                            let v2205 = (v1948 * v2203).exp();
                            let v2210 = v2207 + (v2208 * v473);
                            let v2212 = (v2202 * v2205) * v2210;
                            let v2213 = v2211 / v2212;
                            let v2216 = v2214 + v2215;
                            let v12731 = (((((((v12474 * v2203) * v2205) * v2202) * v2210) * v2213) * v10382) / v2212) * v2216;
                            let v2219 = v2218 * v660;
                            let v2220 = v4 + (v2216 * v2213);
                            let v2221 = v2220.ln();
                            let v12737 = (v10407 * v2218) * v2221;
                            let v2224 = v2223 * v473;
                            let v2226 = (v2224 * v660).sqrt();
                            let v2227 = v1932 - (v2219 * v2221);
                            let v12745 = v12449 - ((Lanes([0.0, 0.0, v12737[0], 0.0, 0.0])) + (((((v9514 + v9508) * v2213) + (Lanes([0.0, 0.0, v12731[0], 0.0, 0.0]))) * (v9360 / v2220)) * v2219));
                            let v12746 = v12474 * v2227;
                            let v2229 = (v1948 * v2227).exp();
                            let v12751 = v10402 * v2227;
                            let v2233 = ((v2229 - v4) + (v658 * v2227)).sqrt();
                            let v12759 = v12474 * v1932;
                            let v2235 = (v1948 * v1932).exp();
                            let v2238 = ((v2235 - v4) + v1933).sqrt();
                            let v2239 = -v2226;
                            let v2240 = v2233 - v2238;
                            let v2241 = v2239 * v2240;
                            let v12770 = (((v10407 * v2224) * (v9360 / (v10427 * v2226))) * v10382) * v2240;
                            let v12773 = (Lanes([0.0, 0.0, v12770[0], 0.0, 0.0])) + (((((((Lanes([0.0, 0.0, v12746[0], 0.0, 0.0])) + (v12745 * v1948)) * v2229) + ((Lanes([0.0, 0.0, v12751[0], 0.0, 0.0])) + (v12745 * v658))) * (v9360 / (v10427 * v2233))) - (((((Lanes([0.0, 0.0, v12759[0], 0.0, 0.0])) + (v12449 * v1948)) * v2235) + v12453) * (v9360 / (v10427 * v2238)))) * v2239);
                            let v2567: f64;
                            let v9210: f64;
                            let v9525: Lanes<6>;
                            let v9526: Lanes<6>;
                            if v2242 != 0.0 {
                                let v2245 = v2214 + v2244;
                                let v2246 = v2243 / v2245;
                                let v2247 = v2246 * v1123;
                                let v12779 = v9413 * v2246;
                                let v2250 = v2248 * v2249;
                                let v12782 = v9372 * v2248;
                                let v12783 = Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v12782[0]]);
                                let v2252 = (v2250 - v2241) / v2247;
                                let v12786 = (((((v9514 * v2246) * v10382) / v2245) * v1123) + (Lanes([v12779[0], v12779[1], 0.0, v12779[2], v12779[3]]))) * v2252;
                                let v12789 = ((v12783 - (Lanes([v12773[0], v12773[1], v12773[2], v12773[3], v12773[4], 0.0]))) - (Lanes([v12786[0], v12786[1], v12786[2], v12786[3], v12786[4], 0.0]))) / v2247;
                                v2567 = v2250;
                                v9210 = v2252;
                                v9525 = v12783;
                                v9526 = v12789;
                            } else {
                                let v12774 = Lanes([v12773[0], v12773[1], v12773[2], v12773[3], v12773[4], 0.0]);
                                v2567 = v2241;
                                v9210 = v0;
                                v9525 = v12774;
                                v9526 = v11054;
                            }
                            v2566 = v2567;
                            v9209 = v9210;
                            v9523 = v9525;
                            v9524 = v9526;
                        } else {
                            v2566 = v0;
                            v9209 = v0;
                            v9523 = v11054;
                            v9524 = v11054;
                        }
                        v2565 = v2566;
                        v2672 = v1973;
                        v5640 = v2214;
                        v5778 = v1947;
                        v9208 = v9209;
                        v9501 = v9523;
                        v9502 = v12516;
                        v9503 = v9514;
                        v9504 = v9524;
                    } else {
                        v2565 = v0;
                        v2672 = v2673;
                        v5640 = v0;
                        v5778 = v0;
                        v9208 = v0;
                        v9501 = v11054;
                        v9502 = v9474;
                        v9503 = v10571;
                        v9504 = v11054;
                    }
                    v2564 = v2565;
                    v2671 = v2672;
                    v4731 = v1898;
                    v4783 = v1896;
                    v5639 = v5640;
                    v5777 = v5778;
                    v9207 = v9208;
                    v9495 = v9501;
                    v9496 = v9502;
                    v9497 = v12379;
                    v9498 = v12377;
                    v9499 = v9503;
                    v9500 = v9504;
                } else {
                    v2564 = v0;
                    v2671 = v2673;
                    v4731 = v731;
                    v4783 = v728;
                    v5639 = v0;
                    v5777 = v0;
                    v9207 = v0;
                    v9495 = v11054;
                    v9496 = v9474;
                    v9497 = v10454;
                    v9498 = v10449;
                    v9499 = v10571;
                    v9500 = v11054;
                }
                let v12790 = Lanes([v9472[0], v9472[1], v9472[2], v9472[3], v9472[4], 0.0]);
                let v12791 = Lanes([v9470[0], v9470[1], v9470[2], v9470[3], v9470[4], 0.0]);
                let v12792 = Lanes([v9471[0], v9471[1], v9471[2], v9471[3], v9471[4], 0.0]);
                let v12793 = Lanes([v9473[0], v9473[1], v9473[2], v9473[3], v9473[4], 0.0]);
                let mut v2256: f64 = 0.0;
                let mut v2258: f64 = 0.0;
                let mut v2294: f64 = 0.0;
                let mut v2316: f64 = 0.0;
                let mut v2450: f64 = 0.0;
                let mut v2568: f64 = 0.0;
                let mut v2573: f64 = 0.0;
                let mut v2584: f64 = 0.0;
                let mut v2587: f64 = 0.0;
                let mut v2594: f64 = 0.0;
                let mut v9527: Lanes<6> = Lanes([0.0; 6]);
                let mut v9528: Lanes<6> = Lanes([0.0; 6]);
                let mut v9529: Lanes<6> = Lanes([0.0; 6]);
                let mut v9530: Lanes<6> = Lanes([0.0; 6]);
                let mut v9531: Lanes<6> = Lanes([0.0; 6]);
                let mut v9532: Lanes<6> = Lanes([0.0; 6]);
                let mut v9533: Lanes<6> = Lanes([0.0; 6]);
                v2256 = v4;
                v2258 = v2255;
                v2294 = v2253;
                v2316 = v2254;
                v2450 = v0;
                v2568 = v0;
                v2573 = v0;
                v2584 = v0;
                v2587 = v0;
                v2594 = v2595;
                v9527 = v12790;
                v9528 = v12791;
                v9529 = v12792;
                v9530 = v11054;
                v9531 = v11054;
                v9532 = v11054;
                v9533 = v12793;
                loop {
                    let v2257 = if v2256 <= v13 { 1.0 } else { 0.0 };
                    if v2257 == 0.0 {
                        break;
                    }
                    let v2259 = v2258 - v1239;
                    let v2260 = v658 * v2259;
                    let v18856 = v10402 * v2259;
                    let v18859 = (Lanes([0.0, 0.0, v18856[0], 0.0, 0.0, 0.0])) + ((v9527 - (Lanes([v9461[0], v9461[1], v9461[2], 0.0, v9461[3], 0.0]))) * v658);
                    let v2262 = (-v2260).exp();
                    let v18861 = (v18859 * v10382) * v2262;
                    let v2264 = if v2259 < v2263 { 1.0 } else { 0.0 };
                    let v2453: f64;
                    let v2466: f64;
                    let v9534: Lanes<6>;
                    let v9535: Lanes<6>;
                    if v2264 != 0.0 {
                        let v2267 = ((v2262 + v2260) - v4).sqrt();
                        let v2268 = v1235 * v2267;
                        let v18901 = v9396 * v2267;
                        let v18904 = (Lanes([0.0, 0.0, v18901[0], 0.0, 0.0, 0.0])) + (((v18861 + v18859) * (v9360 / (v10427 * v2267))) * v1235);
                        let v2272 = (v207 * ((-v2262) + v4)) / v2268;
                        let v18909 = (((v18861 * v10382) * v207) - (v18904 * v2272)) / v2268;
                        v2453 = v2268;
                        v2466 = v2272;
                        v9534 = v18904;
                        v9535 = v18909;
                    } else {
                        let v2273 = if v2259 > v611 { 1.0 } else { 0.0 };
                        let v2454: f64;
                        let v2467: f64;
                        let v9536: Lanes<6>;
                        let v9537: Lanes<6>;
                        if v2273 != 0.0 {
                            let v2274 = v2260.exp();
                            let v18871 = v18859 * v2274;
                            let v2275 = -v1235;
                            let v2279 = (v2274 + v2260) - v4;
                            let v18875 = v9397 * v2279;
                            let v2282 = (((v2262 + v2260) - v4) + (v1257 * v2279)).sqrt();
                            let v2283 = v2275 * v2282;
                            let v18883 = (v9396 * v10382) * v2282;
                            let v18886 = (Lanes([0.0, 0.0, v18883[0], 0.0, 0.0, 0.0])) + ((((v18861 + v18859) + ((Lanes([0.0, 0.0, v18875[0], 0.0, 0.0, 0.0])) + ((v18871 + v18859) * v1257))) * (v9360 / (v10427 * v2282))) * v2275);
                            let v2286 = v2274 + v4;
                            let v18888 = v9397 * v2286;
                            let v2290 = (v207 * (((-v2262) + v4) + (v1257 * v2286))) / v2283;
                            let v18896 = ((((v18861 * v10382) + ((Lanes([0.0, 0.0, v18888[0], 0.0, 0.0, 0.0])) + (v18871 * v1257))) * v207) - (v18886 * v2290)) / v2283;
                            v2454 = v2283;
                            v2467 = v2290;
                            v9536 = v18886;
                            v9537 = v18896;
                        } else {
                            let v2291 = -v1235;
                            let v18862 = v9396 * v10382;
                            let v2292 = v2291 * v2260;
                            let v18863 = v18862 * v2260;
                            let v18866 = (Lanes([0.0, 0.0, v18863[0], 0.0, 0.0, 0.0])) + (v18859 * v2291);
                            let v2293 = v2291 * v658;
                            let v18869 = (v18862 * v658) + (v10402 * v2291);
                            let v18870 = Lanes([0.0, 0.0, v18869[0], 0.0, 0.0, 0.0]);
                            v2454 = v2292;
                            v2467 = v2293;
                            v9536 = v18866;
                            v9537 = v18870;
                        }
                        v2453 = v2454;
                        v2466 = v2467;
                        v9534 = v9536;
                        v9535 = v9537;
                    }
                    let v2295 = v658 * v2294;
                    let v18910 = v10402 * v2294;
                    let v18913 = (Lanes([0.0, 0.0, v18910[0], 0.0, 0.0, 0.0])) + (v9528 * v658);
                    let v2296 = v2295.exp();
                    let v18914 = v18913 * v2296;
                    let v18915 = v12070 * v1499;
                    let v2298 = v745 * v745;
                    let v18917 = v10477 * v745;
                    let v2299 = (v1499 * v1499) / v2298;
                    let v18919 = (v18917 + v18917) * v2299;
                    let v18922 = ((v18915 + v18915) - (Lanes([0.0, 0.0, v18919[0], 0.0, 0.0]))) / v2298;
                    let v2300 = v73 * v754;
                    let v2302 = (v2296 + v2295) - v4;
                    let v18925 = (v10488 * v73) * v2302;
                    let v2305 = (v2299 + (v2300 * v2302)).sqrt();
                    let v18933 = ((Lanes([v18922[0], v18922[1], v18922[2], v18922[3], v18922[4], 0.0])) + ((Lanes([0.0, 0.0, v18925[0], 0.0, 0.0, 0.0])) + ((v18914 + v18913) * v2300))) * (v9360 / (v10427 * v2305));
                    let v2306 = v73 * v658;
                    let v2307 = v2306 * v754;
                    let v2308 = v2296 + v4;
                    let v18938 = (((v10402 * v73) * v754) + (v10488 * v2306)) * v2308;
                    let v2310 = v73 * v2305;
                    let v2311 = (v2307 * v2308) / v2310;
                    let v2312 = -v745;
                    let v18946 = v10477 * v10382;
                    let v18947 = v18946 * v2305;
                    let v2314 = (v2312 * v2305) - v1499;
                    let v18951 = Lanes([v12070[0], v12070[1], v12070[2], v12070[3], v12070[4], 0.0]);
                    let v18952 = ((Lanes([0.0, 0.0, v18947[0], 0.0, 0.0, 0.0])) + (v18933 * v2312)) - v18951;
                    let v2315 = v2312 * v2311;
                    let v18953 = v18946 * v2311;
                    let v18956 = (Lanes([0.0, 0.0, v18953[0], 0.0, 0.0, 0.0])) + (((((Lanes([0.0, 0.0, v18938[0], 0.0, 0.0, 0.0])) + (v18914 * v2307)) - ((v18933 * v73) * v2311)) / v2310) * v2312);
                    let v2318 = (v2316 - v2294) / v1203;
                    let v2319 = v658 * v2318;
                    let v18959 = v10402 * v2318;
                    let v18962 = (Lanes([0.0, 0.0, v18959[0], 0.0, 0.0, 0.0])) + (((v9529 - v9528) / v1203) * v658);
                    let v2320 = -v2319;
                    let v18963 = v18962 * v10382;
                    let v2322 = if v2320 >= v2321 { 1.0 } else { 0.0 };
                    let v2341: f64;
                    let v9538: Lanes<6>;
                    if v2322 != 0.0 {
                        v2341 = v2323;
                        v9538 = v11054;
                    } else {
                        let mut v2324: f64 = 0.0;
                        let mut v2327: f64 = 0.0;
                        let mut v9539: Lanes<6> = Lanes([0.0; 6]);
                        v2324 = v2320;
                        v2327 = v4;
                        v9539 = v18963;
                        loop {
                            let v2326 = if v2324 >= v2325 { 1.0 } else { 0.0 };
                            if v2326 == 0.0 {
                                break;
                            }
                            let v2329 = v2327 * v2328;
                            let v2330 = v2324 - v2325;
                            let edge0 = v2330;
                            let edge1 = v2329;
                            let edge2 = v9539;
                            v2324 = edge0;
                            v2327 = edge1;
                            v9539 = edge2;
                        }
                        let v2331 = v2324.exp();
                        let v2332 = v2327 * v2331;
                        let v18965 = (v9539 * v2331) * v2327;
                        v2341 = v2332;
                        v9538 = v18965;
                    }
                    let v2333 = v2320.exp();
                    let v2336 = ((v2333 + v2319) - v4).sqrt();
                    let v18970 = ((v18963 * v2333) + v18962) * (v9360 / (v10427 * v2336));
                    let v2338 = if v2318 < v2337 { 1.0 } else { 0.0 };
                    let v2364: f64;
                    let v2401: f64;
                    let v2405: f64;
                    let v9540: Lanes<6>;
                    let v9541: Lanes<6>;
                    let v9542: Lanes<6>;
                    if v2338 != 0.0 {
                        let v2339 = v745 * v2336;
                        let v19001 = v10477 * v2336;
                        let v19004 = (Lanes([0.0, 0.0, v19001[0], 0.0, 0.0, 0.0])) + (v18970 * v745);
                        let v2340 = v745 * v658;
                        let v2343 = (-v2341) + v4;
                        let v19009 = ((v10477 * v658) + (v10402 * v745)) * v2343;
                        let v2345 = v73 * v2336;
                        let v2346 = (v2340 * v2343) / v2345;
                        let v2347 = v2346 / v1203;
                        let v19017 = ((((Lanes([0.0, 0.0, v19009[0], 0.0, 0.0, 0.0])) + ((v9538 * v10382) * v2340)) - ((v18970 * v73) * v2346)) / v2345) / v1203;
                        let v2348 = -v2347;
                        let v19018 = v19017 * v10382;
                        v2364 = v2339;
                        v2401 = v2347;
                        v2405 = v2348;
                        v9540 = v19004;
                        v9541 = v19017;
                        v9542 = v19018;
                    } else {
                        let v2349 = if v2318 > v611 { 1.0 } else { 0.0 };
                        let v2365: f64;
                        let v2402: f64;
                        let v2406: f64;
                        let v9543: Lanes<6>;
                        let v9544: Lanes<6>;
                        let v9545: Lanes<6>;
                        if v2349 != 0.0 {
                            let v2350 = v2312 * v2336;
                            let v18983 = v18946 * v2336;
                            let v18986 = (Lanes([0.0, 0.0, v18983[0], 0.0, 0.0, 0.0])) + (v18970 * v2312);
                            let v2351 = v2312 * v658;
                            let v2353 = (-v2341) + v4;
                            let v18991 = ((v18946 * v658) + (v10402 * v2312)) * v2353;
                            let v2355 = v73 * v2336;
                            let v2356 = (v2351 * v2353) / v2355;
                            let v2357 = v2356 / v1203;
                            let v18999 = ((((Lanes([0.0, 0.0, v18991[0], 0.0, 0.0, 0.0])) + ((v9538 * v10382) * v2351)) - ((v18970 * v73) * v2356)) / v2355) / v1203;
                            let v2358 = -v2357;
                            let v19000 = v18999 * v10382;
                            v2365 = v2350;
                            v2402 = v2357;
                            v2406 = v2358;
                            v9543 = v18986;
                            v9544 = v18999;
                            v9545 = v19000;
                        } else {
                            let v18971 = v18946 * v2319;
                            let v2360 = (v2312 * v2319) / v743;
                            let v18975 = ((Lanes([0.0, 0.0, v18971[0], 0.0, 0.0, 0.0])) + (v18962 * v2312)) / v743;
                            let v2362 = (v2312 * v658) / v743;
                            let v18979 = ((v18946 * v658) + (v10402 * v2312)) / v743;
                            let v2363 = -v2362;
                            let v18980 = v18979 * v10382;
                            let v18981 = Lanes([0.0, 0.0, v18979[0], 0.0, 0.0, 0.0]);
                            let v18982 = Lanes([0.0, 0.0, v18980[0], 0.0, 0.0, 0.0]);
                            v2365 = v2360;
                            v2402 = v2362;
                            v2406 = v2363;
                            v9543 = v18975;
                            v9544 = v18981;
                            v9545 = v18982;
                        }
                        v2364 = v2365;
                        v2401 = v2402;
                        v2405 = v2406;
                        v9540 = v9543;
                        v9541 = v9544;
                        v9542 = v9545;
                    }
                    let v2366 = -v1220;
                    let v19019 = v11944 * v10382;
                    let v2367 = v0 - v2366;
                    let v19020 = v19019 * v10382;
                    let v2370 = if (if v2364 > v2367 { 1.0 } else { 0.0 }) != 0.0 && (if v2366 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v2403: f64;
                    let v2408: f64;
                    let v9546: Lanes<6>;
                    let v9547: Lanes<6>;
                    if v2370 != 0.0 {
                        let v2371 = v2364 + v2366;
                        let v19022 = v9540 + (Lanes([v19019[0], v19019[1], v19019[2], v19019[3], v19019[4], 0.0]));
                        let v2372 = v2371 * v2371;
                        let v19023 = v19022 * v2371;
                        let v2373 = v2366 * v2366;
                        let v19025 = v19019 * v2366;
                        let v19027 = (v19023 + v19023) * v2372;
                        let v2375 = v2373 * v2373;
                        let v19029 = (v19025 + v19025) * v2373;
                        let v19030 = v19029 + v19029;
                        let v2376 = (v2372 * v2372) + v2375;
                        let v19032 = (v19027 + v19027) + (Lanes([v19030[0], v19030[1], v19030[2], v19030[3], v19030[4], 0.0]));
                        let v2393: f64;
                        let v9548: Lanes<6>;
                        if v2377 != 0.0 {
                            let v2387: f64;
                            if v2378 != 0.0 {
                                v2387 = v4;
                            } else {
                                let v2388: f64;
                                if v2379 != 0.0 {
                                    v2388 = v73;
                                } else {
                                    let v2389: f64;
                                    if v2380 != 0.0 {
                                        v2389 = v91;
                                    } else {
                                        let v2390: f64;
                                        if v2381 != 0.0 {
                                            v2390 = v85;
                                        } else {
                                            v2390 = v0;
                                        }
                                        v2389 = v2390;
                                    }
                                    v2388 = v2389;
                                }
                                v2387 = v2388;
                            }
                            let mut v2382: f64 = 0.0;
                            let mut v2384: f64 = 0.0;
                            let mut v9549: Lanes<6> = Lanes([0.0; 6]);
                            v2382 = v0;
                            v2384 = v2376;
                            v9549 = v19032;
                            loop {
                                let v2383 = if v2382 < v2387 { 1.0 } else { 0.0 };
                                if v2383 == 0.0 {
                                    break;
                                }
                                let v2385 = v2384.sqrt();
                                let v19254 = v9549 * (v9360 / (v10427 * v2385));
                                let v2386 = v2382 + v4;
                                v2382 = v2386;
                                v2384 = v2385;
                                v9549 = v19254;
                            }
                            v2393 = v2384;
                            v9548 = v9549;
                        } else {
                            let v2392 = v2376.powf(v2391);
                            let v19036 = v19032 * (v2391 * (v2376.powf(v19033)));
                            v2393 = v2392;
                            v9548 = v19036;
                        }
                        let v2394 = v4 / v2393;
                        let v19039 = ((v9548 * v2394) * v10382) / v2393;
                        let v2395 = v2371 * v2366;
                        let v19041 = v19019 * v2371;
                        let v2397 = v2366 * v2375;
                        let v19050 = ((v19019 * v2375) + (v19030 * v2366)) * v2394;
                        let v2399 = (v2397 * v2394) / v2376;
                        let v19056 = (((Lanes([v19050[0], v19050[1], v19050[2], v19050[3], v19050[4], 0.0])) + (v19039 * v2397)) - (v19032 * v2399)) / v2376;
                        let v2400 = v2367 + (v2395 * v2394);
                        let v19058 = (Lanes([v19020[0], v19020[1], v19020[2], v19020[3], v19020[4], 0.0])) + ((((v19022 * v2366) + (Lanes([v19041[0], v19041[1], v19041[2], v19041[3], v19041[4], 0.0]))) * v2394) + (v19039 * v2395));
                        v2403 = v2399;
                        v2408 = v2400;
                        v9546 = v19056;
                        v9547 = v19058;
                    } else {
                        v2403 = v4;
                        v2408 = v2364;
                        v9546 = v11054;
                        v9547 = v9540;
                    }
                    let v2404 = v2401 * v2403;
                    let v19061 = (v9541 * v2403) + (v9546 * v2401);
                    let v2407 = v2405 * v2403;
                    let v19064 = (v9542 * v2403) + (v9546 * v2405);
                    let v2409 = v1223 - v1499;
                    let v19065 = v12070 * v10382;
                    let v2410 = -v2409;
                    let v19066 = v19065 * v10382;
                    let v2411 = v2409 + v2410;
                    let v19067 = v19065 + v19066;
                    let v2414 = if (if v2408 < v2411 { 1.0 } else { 0.0 }) != 0.0 && (if v2410 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v2445: f64;
                    let v2448: f64;
                    let v9550: Lanes<6>;
                    let v9551: Lanes<6>;
                    if v2414 != 0.0 {
                        let v2415 = v2411 - v2408;
                        let v19068 = Lanes([v19067[0], v19067[1], v19067[2], v19067[3], v19067[4], 0.0]);
                        let v19069 = v19068 - v9547;
                        let v2416 = v2415 * v2415;
                        let v19070 = v19069 * v2415;
                        let v2417 = v2410 * v2410;
                        let v19072 = v19066 * v2410;
                        let v19074 = (v19070 + v19070) * v2416;
                        let v2419 = v2417 * v2417;
                        let v19076 = (v19072 + v19072) * v2417;
                        let v19077 = v19076 + v19076;
                        let v2420 = (v2416 * v2416) + v2419;
                        let v19079 = (v19074 + v19074) + (Lanes([v19077[0], v19077[1], v19077[2], v19077[3], v19077[4], 0.0]));
                        let v2437: f64;
                        let v9552: Lanes<6>;
                        if v2421 != 0.0 {
                            let v2431: f64;
                            if v2422 != 0.0 {
                                v2431 = v4;
                            } else {
                                let v2432: f64;
                                if v2423 != 0.0 {
                                    v2432 = v73;
                                } else {
                                    let v2433: f64;
                                    if v2424 != 0.0 {
                                        v2433 = v91;
                                    } else {
                                        let v2434: f64;
                                        if v2425 != 0.0 {
                                            v2434 = v85;
                                        } else {
                                            v2434 = v0;
                                        }
                                        v2433 = v2434;
                                    }
                                    v2432 = v2433;
                                }
                                v2431 = v2432;
                            }
                            let mut v2426: f64 = 0.0;
                            let mut v2428: f64 = 0.0;
                            let mut v9553: Lanes<6> = Lanes([0.0; 6]);
                            v2426 = v0;
                            v2428 = v2420;
                            v9553 = v19079;
                            loop {
                                let v2427 = if v2426 < v2431 { 1.0 } else { 0.0 };
                                if v2427 == 0.0 {
                                    break;
                                }
                                let v2429 = v2428.sqrt();
                                let v19251 = v9553 * (v9360 / (v10427 * v2429));
                                let v2430 = v2426 + v4;
                                v2426 = v2430;
                                v2428 = v2429;
                                v9553 = v19251;
                            }
                            v2437 = v2428;
                            v9552 = v9553;
                        } else {
                            let v2436 = v2420.powf(v2435);
                            let v19083 = v19079 * (v2435 * (v2420.powf(v19080)));
                            v2437 = v2436;
                            v9552 = v19083;
                        }
                        let v2438 = v4 / v2437;
                        let v19086 = ((v9552 * v2438) * v10382) / v2437;
                        let v2439 = v2415 * v2410;
                        let v19088 = v19066 * v2415;
                        let v2441 = v2410 * v2419;
                        let v19097 = ((v19066 * v2419) + (v19077 * v2410)) * v2438;
                        let v2443 = (v2441 * v2438) / v2420;
                        let v19103 = (((Lanes([v19097[0], v19097[1], v19097[2], v19097[3], v19097[4], 0.0])) + (v19086 * v2441)) - (v19079 * v2443)) / v2420;
                        let v2444 = v2411 - (v2439 * v2438);
                        let v19104 = v19068 - ((((v19069 * v2410) + (Lanes([v19088[0], v19088[1], v19088[2], v19088[3], v19088[4], 0.0]))) * v2438) + (v19086 * v2439));
                        v2445 = v2443;
                        v2448 = v2444;
                        v9550 = v19103;
                        v9551 = v19104;
                    } else {
                        v2445 = v4;
                        v2448 = v2408;
                        v9550 = v11054;
                        v9551 = v9547;
                    }
                    let v2446 = v2407 * v2445;
                    let v19107 = (v19064 * v2445) + (v9550 * v2407);
                    let v2447 = v2404 * v2445;
                    let v19110 = (v19061 * v2445) + (v9550 * v2404);
                    let v2449 = v1499 + v2448;
                    let v19111 = v18951 + v9551;
                    let v2451 = if v2450 == v4 { 1.0 } else { 0.0 };
                    let v2557: f64;
                    let v2559: f64;
                    let v2560: f64;
                    let v2561: f64;
                    let v2562: f64;
                    let v2569: f64;
                    let v9554: Lanes<6>;
                    let v9555: Lanes<6>;
                    let v9556: Lanes<6>;
                    if v2451 != 0.0 {
                        v2557 = v13;
                        v2559 = v2258;
                        v2560 = v2294;
                        v2561 = v2316;
                        v2562 = v2450;
                        v2569 = v2256;
                        v9554 = v9527;
                        v9555 = v9528;
                        v9556 = v9529;
                    } else {
                        let v2458 = (((v2453 + v1499) + v2314) + v2448) + v2564;
                        let v19118 = v9412 * v2458;
                        let v2460 = (v2294 - v1195) - (v1043 * v2458);
                        let v19122 = (v9528 - (Lanes([v10820[0], v10820[1], v10820[2], v10820[3], v10820[4], 0.0]))) - ((Lanes([v19118[0], v19118[1], 0.0, v19118[2], v19118[3], 0.0])) + (((((v9534 + v18951) + v18952) + v9551) + v9495) * v1043));
                        let v2461 = v2315 + v2446;
                        let v19124 = v9412 * v2461;
                        let v2463 = v4 - (v1043 * v2461);
                        let v19128 = ((Lanes([v19124[0], v19124[1], 0.0, v19124[2], v19124[3], 0.0])) + ((v18956 + v19107) * v1043)) * v10382;
                        let v2464 = -v1043;
                        let v19129 = v9412 * v10382;
                        let v2465 = v2464 * v2447;
                        let v19130 = v19129 * v2447;
                        let v19133 = (Lanes([v19130[0], v19130[1], 0.0, v19130[2], v19130[3], 0.0])) + (v19110 * v2464);
                        let v2468 = v2464 * v2466;
                        let v19134 = v19129 * v2466;
                        let v19137 = (Lanes([v19134[0], v19134[1], 0.0, v19134[2], v19134[3], 0.0])) + (v9535 * v2464);
                        let v2474 = v2316 - (v2294 + (v120 * ((v8 * v1223) + v2453)));
                        let v19141 = v9529 - (v9528 + (v9534 * v120));
                        let v2476 = -(v120 * v2466);
                        let v19142 = (v9535 * v120) * v10382;
                        let v2479 = (v2258 - v2316) - (v126 * v2453);
                        let v19145 = (v9527 - v9529) - (v9534 * v126);
                        let v2482 = v4 - (v126 * v2466);
                        let v19147 = (v9535 * v126) * v10382;
                        let v2483 = v2463 * v2482;
                        let v19150 = (v19128 * v2482) + (v19147 * v2463);
                        let v2484 = v2463 * v2476;
                        let v19153 = (v19128 * v2476) + (v19142 * v2463);
                        let v2487 = v2465 * v2475;
                        let v19156 = v19133 * v2475;
                        let v2490 = v2468 * v2475;
                        let v19161 = v19137 * v2475;
                        let v2493 = (((v2483 - (v2484 * v2480)) - (v2487 * v2482)) + (v2490 * v2480)) + v358;
                        let v2494 = v4 / v2493;
                        let v2496 = v2482 - (v2476 * v2480);
                        let v2499 = (v2468 * v2480) - (v2465 * v2482);
                        let v2501 = (v2465 * v2476) - v2468;
                        let v2502 = v2490 - v2484;
                        let v2504 = (-v2463) * v2480;
                        let v2505 = v2463 - v2487;
                        let v2506 = -v2494;
                        let v19182 = ((((((v19150 - (v19153 * v2480)) - ((v19156 * v2482) + (v19147 * v2487))) + (v19161 * v2480)) * v2494) * v10382) / v2493) * v10382;
                        let v2511 = ((v2496 * v2460) + (v2499 * v2474)) + (v2501 * v2479);
                        let v2512 = v2506 * v2511;
                        let v19196 = (v19182 * v2511) + ((((((v19147 - (v19142 * v2480)) * v2460) + (v19122 * v2496)) + ((((v19137 * v2480) - ((v19133 * v2482) + (v19147 * v2465))) * v2474) + (v19141 * v2499))) + (((((v19133 * v2476) + (v19142 * v2465)) - v19137) * v2479) + (v19145 * v2501))) * v2506);
                        let v2517 = ((v2482 * v2460) + (v2483 * v2474)) + (v2502 * v2479);
                        let v2518 = v2506 * v2517;
                        let v19210 = (v19182 * v2517) + (((((v19147 * v2460) + (v19122 * v2482)) + ((v19150 * v2474) + (v19141 * v2483))) + (((v19161 - v19153) * v2479) + (v19145 * v2502))) * v2506);
                        let v2522 = (v2460 + (v2504 * v2474)) + (v2505 * v2479);
                        let v2523 = v2506 * v2522;
                        let v19221 = (v19182 * v2522) + (((v19122 + ((((v19128 * v10382) * v2480) * v2474) + (v19141 * v2504))) + (((v19128 - v19156) * v2479) + (v19145 * v2505))) * v2506);
                        let v2524 = v2512.abs();
                        let v19225 = v19196 * ((v10427 * (if v2512 >= v11296 { 1.0 } else { 0.0 })) - v9360);
                        let v2525 = v2518.abs();
                        let v19229 = v19210 * ((v10427 * (if v2518 >= v11296 { 1.0 } else { 0.0 })) - v9360);
                        let v2526 = if v2524 < v2525 { 1.0 } else { 0.0 };
                        let v2527: f64;
                        let v9557: Lanes<6>;
                        if v2526 != 0.0 {
                            v2527 = v2525;
                            v9557 = v19229;
                        } else {
                            v2527 = v2524;
                            v9557 = v19225;
                        }
                        let v2528 = v2523.abs();
                        let v19233 = v19221 * ((v10427 * (if v2523 >= v11296 { 1.0 } else { 0.0 })) - v9360);
                        let v2529 = if v2527 < v2528 { 1.0 } else { 0.0 };
                        let v2538: f64;
                        let v9558: Lanes<6>;
                        if v2529 != 0.0 {
                            v2538 = v2528;
                            v9558 = v19233;
                        } else {
                            v2538 = v2527;
                            v9558 = v9557;
                        }
                        let v2531 = if v2256 > v2530 { 1.0 } else { 0.0 };
                        let v2539: f64;
                        if v2531 != 0.0 {
                            v2539 = v2532;
                        } else {
                            let v2534 = if v2256 > v2533 { 1.0 } else { 0.0 };
                            let v2540: f64;
                            if v2534 != 0.0 {
                                v2540 = v2532;
                            } else {
                                let v2535 = if v2256 > v816 { 1.0 } else { 0.0 };
                                let v2541: f64;
                                if v2535 != 0.0 {
                                    v2541 = v2536;
                                } else {
                                    let v2537 = if v2256 > v10 { 1.0 } else { 0.0 };
                                    let v2542: f64;
                                    if v2537 != 0.0 {
                                        v2542 = v639;
                                    } else {
                                        v2542 = v4;
                                    }
                                    v2541 = v2542;
                                }
                                v2540 = v2541;
                            }
                            v2539 = v2540;
                        }
                        let v2543 = v74 / v2539;
                        let v2544 = if v2538 > v2543 { 1.0 } else { 0.0 };
                        let v2549: f64;
                        let v2551: f64;
                        let v2553: f64;
                        let v9559: Lanes<6>;
                        let v9560: Lanes<6>;
                        let v9561: Lanes<6>;
                        if v2544 != 0.0 {
                            let v2545 = v2543 / v2538;
                            let v19236 = ((v9558 * v2545) * v10382) / v2538;
                            let v2546 = v2512 * v2545;
                            let v19239 = (v19196 * v2545) + (v19236 * v2512);
                            let v2547 = v2518 * v2545;
                            let v19242 = (v19210 * v2545) + (v19236 * v2518);
                            let v2548 = v2523 * v2545;
                            let v19245 = (v19221 * v2545) + (v19236 * v2523);
                            v2549 = v2546;
                            v2551 = v2547;
                            v2553 = v2548;
                            v9559 = v19239;
                            v9560 = v19242;
                            v9561 = v19245;
                        } else {
                            v2549 = v2512;
                            v2551 = v2518;
                            v2553 = v2523;
                            v9559 = v19196;
                            v9560 = v19210;
                            v9561 = v19221;
                        }
                        let v2550 = v2294 + v2549;
                        let v19246 = v9528 + v9559;
                        let v2552 = v2316 + v2551;
                        let v19247 = v9529 + v9560;
                        let v2554 = v2258 + v2553;
                        let v19248 = v9527 + v9561;
                        let v2556 = if v2538 < (v856 * v2539) { 1.0 } else { 0.0 };
                        let v2563: f64;
                        if v2556 != 0.0 {
                            v2563 = v4;
                        } else {
                            v2563 = v2450;
                        }
                        v2557 = v2256;
                        v2559 = v2554;
                        v2560 = v2550;
                        v2561 = v2552;
                        v2562 = v2563;
                        v2569 = v2568;
                        v9554 = v19248;
                        v9555 = v19246;
                        v9556 = v19247;
                    }
                    let v2558 = v2557 + v4;
                    v2256 = v2558;
                    v2258 = v2559;
                    v2294 = v2560;
                    v2316 = v2561;
                    v2450 = v2562;
                    v2568 = v2569;
                    v2573 = v2314;
                    v2584 = v2448;
                    v2587 = v2449;
                    v2594 = v2453;
                    v9527 = v9554;
                    v9528 = v9555;
                    v9529 = v9556;
                    v9530 = v18952;
                    v9531 = v9551;
                    v9532 = v19111;
                    v9533 = v9534;
                }
                let v2570 = if v2568 > v0 { 1.0 } else { 0.0 };
                if v2570 != 0.0 {
                } else {
                }
                let v2571 = if v2450 == v0 { 1.0 } else { 0.0 };
                let v2572: f64;
                let v2598: f64;
                let v2599: f64;
                let v9562: Lanes<6>;
                let v9563: Lanes<6>;
                let v9564: Lanes<6>;
                if v2571 != 0.0 {
                    v2572 = v2253;
                    v2598 = v2254;
                    v2599 = v2255;
                    v9562 = v12791;
                    v9563 = v12792;
                    v9564 = v12790;
                } else {
                    v2572 = v2294;
                    v2598 = v2316;
                    v2599 = v2258;
                    v9562 = v9528;
                    v9563 = v9529;
                    v9564 = v9527;
                }
                let v2574 = -v2573;
                let v12794 = v9530 * v10382;
                let v2575 = if v2574 <= v358 { 1.0 } else { 0.0 };
                let v2576: f64;
                let v9565: Lanes<6>;
                if v2575 != 0.0 {
                    v2576 = v358;
                    v9565 = v11054;
                } else {
                    v2576 = v2574;
                    v9565 = v12794;
                }
                let v2577 = v2576 * v1043;
                let v12796 = v9412 * v2576;
                let v12798 = (v9565 * v1043) + (Lanes([v12796[0], v12796[1], 0.0, v12796[2], v12796[3], 0.0]));
                let v2579 = if (if v2572 <= v0 { 1.0 } else { 0.0 }) != 0.0 && v0 != 0.0 { 1.0 } else { 0.0 };
                let v3461: f64;
                let v3470: f64;
                let v4303: f64;
                let v4307: f64;
                let v4310: f64;
                let v4321: f64;
                let v4332: f64;
                let v4377: f64;
                let v4417: f64;
                let v4424: f64;
                let v4435: f64;
                let v4441: f64;
                let v4839: f64;
                let v5717: f64;
                let v8300: f64;
                let v8477: f64;
                let v8482: f64;
                let v8487: f64;
                let v8493: f64;
                let v9566: Lanes<6>;
                let v9567: Lanes<6>;
                let v9568: Lanes<6>;
                let v9569: Lanes<6>;
                let v9570: Lanes<6>;
                let v9571: Lanes<6>;
                let v9572: Lanes<6>;
                let v9573: Lanes<6>;
                let v9574: Lanes<6>;
                let v9575: Lanes<6>;
                let v9576: Lanes<6>;
                let v9577: Lanes<6>;
                let v9578: Lanes<6>;
                let v9579: Lanes<6>;
                let v9580: Lanes<6>;
                let v9581: Lanes<6>;
                if v2579 != 0.0 {
                    let v2583 = (-v164) * v134;
                    let v2589 = v2586 * ((v1499 + v2584) + v2587);
                    let v13735 = (((Lanes([v12070[0], v12070[1], v12070[2], v12070[3], v12070[4], 0.0])) + v9531) + v9532) * v2586;
                    let v2590 = v2583 * v2589;
                    let v13736 = v13735 * v2583;
                    let v2591 = v2590 * v8;
                    let v13737 = v13736 * v8;
                    let v2593 = v2590 * v2592;
                    let v13738 = v13736 * v2592;
                    let v2597 = (v2594 * v134) * v164;
                    let v13740 = (v9533 * v134) * v164;
                    v3461 = v2580;
                    v3470 = v0;
                    v4303 = v0;
                    v4307 = v0;
                    v4310 = v0;
                    v4321 = v4;
                    v4332 = v2572;
                    v4377 = v0;
                    v4417 = v2589;
                    v4424 = v0;
                    v4435 = v2594;
                    v4441 = v0;
                    v4839 = v0;
                    v5717 = v2598;
                    v8300 = v2572;
                    v8477 = v2590;
                    v8482 = v2597;
                    v8487 = v2591;
                    v8493 = v2593;
                    v9566 = v11054;
                    v9567 = v11054;
                    v9568 = v11054;
                    v9569 = v9562;
                    v9570 = v11054;
                    v9571 = v13735;
                    v9572 = v11054;
                    v9573 = v9533;
                    v9574 = v11054;
                    v9575 = v11054;
                    v9576 = v9563;
                    v9577 = v9562;
                    v9578 = v13736;
                    v9579 = v13740;
                    v9580 = v13737;
                    v9581 = v13738;
                } else {
                    let v2600 = v1123 * v1123;
                    let v12799 = v9413 * v1123;
                    let v2601 = v487 / v2600;
                    let v12803 = (((v12799 + v12799) * v2601) * v10382) / v2600;
                    let v2602 = v73 / v2601;
                    let v12806 = ((v12803 * v2602) * v10382) / v2601;
                    let v2603 = v1195 - v358;
                    let v12807 = v12806 * v2603;
                    let v12810 = (Lanes([v12807[0], v12807[1], 0.0, v12807[2], v12807[3]])) + (v10820 * v2602);
                    let v2605 = v4 + (v2602 * v2603);
                    let v2606 = v4 + v2602;
                    let v2609 = if (if v2605 < v2606 { 1.0 } else { 0.0 }) != 0.0 && (if v2606 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v2641: f64;
                    let v9582: Lanes<5>;
                    if v2609 != 0.0 {
                        let v2610 = v2606 - v2605;
                        let v12811 = Lanes([v12806[0], v12806[1], 0.0, v12806[2], v12806[3]]);
                        let v12812 = v12811 - v12810;
                        let v2611 = v2610 * v2610;
                        let v12813 = v12812 * v2610;
                        let v12814 = v12813 + v12813;
                        let v2612 = v2606 * v2606;
                        let v12815 = v12806 * v2606;
                        let v12816 = v12815 + v12815;
                        let v2613 = v2611 * v2611;
                        let v12817 = v12814 * v2611;
                        let v2614 = v2612 * v2612;
                        let v12819 = v12816 * v2612;
                        let v2615 = v2613 * v2611;
                        let v2616 = v2614 * v2612;
                        let v12832 = ((((v12819 + v12819) * v2612) + (v12816 * v2614)) * v2612) + (v12816 * v2616);
                        let v2619 = (v2615 * v2611) + (v2616 * v2612);
                        let v12834 = (((((v12817 + v12817) * v2611) + (v12814 * v2613)) * v2611) + (v12814 * v2615)) + (Lanes([v12832[0], v12832[1], 0.0, v12832[2], v12832[3]]));
                        let v2636: f64;
                        let v9583: Lanes<5>;
                        if v2620 != 0.0 {
                            let v2630: f64;
                            if v2621 != 0.0 {
                                v2630 = v4;
                            } else {
                                let v2631: f64;
                                if v2622 != 0.0 {
                                    v2631 = v73;
                                } else {
                                    let v2632: f64;
                                    if v2623 != 0.0 {
                                        v2632 = v91;
                                    } else {
                                        let v2633: f64;
                                        if v2624 != 0.0 {
                                            v2633 = v85;
                                        } else {
                                            v2633 = v0;
                                        }
                                        v2632 = v2633;
                                    }
                                    v2631 = v2632;
                                }
                                v2630 = v2631;
                            }
                            let mut v2625: f64 = 0.0;
                            let mut v2627: f64 = 0.0;
                            let mut v9584: Lanes<5> = Lanes([0.0; 5]);
                            v2625 = v0;
                            v2627 = v2619;
                            v9584 = v12834;
                            loop {
                                let v2626 = if v2625 < v2630 { 1.0 } else { 0.0 };
                                if v2626 == 0.0 {
                                    break;
                                }
                                let v2628 = v2627.sqrt();
                                let v13731 = v9584 * (v9360 / (v10427 * v2628));
                                let v2629 = v2625 + v4;
                                v2625 = v2629;
                                v2627 = v2628;
                                v9584 = v13731;
                            }
                            v2636 = v2627;
                            v9583 = v9584;
                        } else {
                            let v2635 = v2619.powf(v2634);
                            let v12838 = v12834 * (v2634 * (v2619.powf(v12835)));
                            v2636 = v2635;
                            v9583 = v12838;
                        }
                        let v2637 = v4 / v2636;
                        let v2638 = v2610 * v2606;
                        let v12843 = v12806 * v2610;
                        let v2640 = v2606 - (v2638 * v2637);
                        let v12849 = v12811 - ((((v12812 * v2606) + (Lanes([v12843[0], v12843[1], 0.0, v12843[2], v12843[3]]))) * v2637) + ((((v9583 * v2637) * v10382) / v2636) * v2638));
                        v2641 = v2640;
                        v9582 = v12849;
                    } else {
                        v2641 = v2605;
                        v9582 = v12810;
                    }
                    let v2642 = v2641.sqrt();
                    let v2643 = v4 - v2642;
                    let v12854 = v12803 * v2643;
                    let v2645 = v1195 + (v2601 * v2643);
                    let v12858 = v10820 + ((Lanes([v12854[0], v12854[1], 0.0, v12854[2], v12854[3]])) + (((v9582 * (v9360 / (v10427 * v2642))) * v10382) * v2601));
                    let v12859 = v12858 * v2645;
                    let v2649 = ((v2645 * v2645) + v2647).sqrt();
                    let v12865 = (v12858 + ((v12859 + v12859) * (v9360 / (v10427 * v2649)))) * v8;
                    let v2653 = (v8 * (v2645 + v2649)) + v2652;
                    let v2654 = if v2653 < v0 { 1.0 } else { 0.0 };
                    let v2655: f64;
                    let v9585: Lanes<5>;
                    if v2654 != 0.0 {
                        v2655 = v0;
                        v9585 = v10571;
                    } else {
                        v2655 = v2653;
                        v9585 = v12865;
                    }
                    let v2656 = v818 / v2655;
                    let v12868 = (v10589 - (v9585 * v2656)) / v2655;
                    let v2658 = v2657 - v4;
                    let v2659 = v2656.powf(v2658);
                    let v12875 = ((v12868 * (v2658 * (v2656.powf((v2658 - v9360))))) * v2656) + (v12868 * v2659);
                    let v2661 = v4 + (v2659 * v2656);
                    let v2663 = (v4 / v2657) - v4;
                    let v2664 = v2661.powf(v2663);
                    let v2665 = v2664 * v2661;
                    let v2666 = v818 / v2665;
                    let v12885 = (v10589 - ((((v12875 * (v2663 * (v2661.powf((v2663 - v9360))))) * v2661) + (v12875 * v2664)) * v2666)) / v2665;
                    let v2667 = if v2666 < v0 { 1.0 } else { 0.0 };
                    let v2998: f64;
                    let v3003: f64;
                    let v3010: f64;
                    let v3325: f64;
                    let v3349: f64;
                    let v3462: f64;
                    let v9586: Lanes<6>;
                    let v9587: Lanes<6>;
                    let v9588: Lanes<6>;
                    let v9589: Lanes<6>;
                    if v2667 != 0.0 {
                        v2998 = v2598;
                        v3003 = v2572;
                        v3010 = v2599;
                        v3325 = v3326;
                        v3349 = v0;
                        v3462 = v2580;
                        v9586 = v9563;
                        v9587 = v9562;
                        v9588 = v9564;
                        v9589 = v11054;
                    } else {
                        let v2999: f64;
                        let v3004: f64;
                        let v3011: f64;
                        let v3327: f64;
                        let v3350: f64;
                        let v3463: f64;
                        let v9590: Lanes<6>;
                        let v9591: Lanes<6>;
                        let v9592: Lanes<6>;
                        let v9593: Lanes<6>;
                        if v2668 != 0.0 {
                            let v2669 = if v0 < v1507 { 1.0 } else { 0.0 };
                            let v2670: f64;
                            if v2669 != 0.0 {
                                v2670 = v4;
                            } else {
                                v2670 = v73;
                            }
                            v2999 = v0;
                            v3004 = v0;
                            v3011 = v0;
                            v3327 = v3326;
                            v3350 = v0;
                            v3463 = v2670;
                            v9590 = v11054;
                            v9591 = v11054;
                            v9592 = v11054;
                            v9593 = v11054;
                        } else {
                            let v2674 = v2671 - v2572;
                            let v12887 = (Lanes([v9496[0], v9496[1], v9496[2], v9496[3], v9496[4], 0.0])) - v9562;
                            let v2675 = if v2674 >= v0 { 1.0 } else { 0.0 };
                            let v2676: f64;
                            let v9594: Lanes<6>;
                            if v2675 != 0.0 {
                                v2676 = v2674;
                                v9594 = v12887;
                            } else {
                                v2676 = v0;
                                v9594 = v11054;
                            }
                            let v12889 = Lanes([v12885[0], v12885[1], v12885[2], v12885[3], v12885[4], 0.0]);
                            let v12890 = (v9594 * v2677) - v12889;
                            let v2680 = ((v2677 * v2676) - v2666) - v1980;
                            let v2684 = (v85 * (v2681 * v2676)) * v1980;
                            let v12893 = ((v9594 * v2681) * v85) * v1980;
                            let v2685 = if v2684 > v0 { 1.0 } else { 0.0 };
                            let v2687: f64;
                            let v9595: Lanes<6>;
                            if v2685 != 0.0 {
                                v2687 = v2684;
                                v9595 = v12893;
                            } else {
                                let v2686 = -v2684;
                                let v12894 = v12893 * v10382;
                                v2687 = v2686;
                                v9595 = v12894;
                            }
                            let v12895 = v12890 * v2680;
                            let v2690 = ((v2680 * v2680) + v2687).sqrt();
                            let v2695 = (v2691 * v2676) - (v8 * (v2680 + v2690));
                            let v12904 = (v9594 * v2691) - ((v12890 + (((v12895 + v12895) + v9595) * (v9360 / (v10427 * v2690)))) * v8);
                            let v2696 = if v2695 <= v2676 { 1.0 } else { 0.0 };
                            let v2697: f64;
                            let v9596: Lanes<6>;
                            if v2696 != 0.0 {
                                v2697 = v2695;
                                v9596 = v12904;
                            } else {
                                v2697 = v2676;
                                v9596 = v9594;
                            }
                            let v2698 = if v2697 < v0 { 1.0 } else { 0.0 };
                            let v2700: f64;
                            let v9597: Lanes<6>;
                            if v2698 != 0.0 {
                                v2700 = v0;
                                v9597 = v11054;
                            } else {
                                let v2699 = if v2697 > v2666 { 1.0 } else { 0.0 };
                                let v2701: f64;
                                let v9598: Lanes<6>;
                                if v2699 != 0.0 {
                                    v2701 = v2666;
                                    v9598 = v12889;
                                } else {
                                    v2701 = v2697;
                                    v9598 = v9596;
                                }
                                v2700 = v2701;
                                v9597 = v9598;
                            }
                            let v2702 = v2572 + v2700;
                            let v12905 = v9562 + v9597;
                            let v2703 = if v2702 < v1507 { 1.0 } else { 0.0 };
                            let v2875: f64;
                            let v9599: Lanes<6>;
                            if v2703 != 0.0 {
                                let v12956 = v11961 * v1243;
                                let v12958 = (v12956 + v12956) - v11966;
                                let v2705 = if v1248 >= v2704 { 1.0 } else { 0.0 };
                                let v2707: f64;
                                let v9600: Lanes<4>;
                                if v2705 != 0.0 {
                                    v2707 = v1248;
                                    v9600 = v12958;
                                } else {
                                    v2707 = v2706;
                                    v9600 = v10652;
                                }
                                let v2708 = v2707.sqrt();
                                let v2710 = (v1243 - v2708) / v73;
                                let v12963 = (v11961 - (v9600 * (v9360 / (v10427 * v2708)))) / v73;
                                let v12968 = ((((v11970 - v11972) / v1257) * v11973) - v11979) / v1261;
                                let v2711 = if v2710 < v1234 { 1.0 } else { 0.0 };
                                let v2876: f64;
                                let v9601: Lanes<4>;
                                if v2711 != 0.0 {
                                    v2876 = v2710;
                                    v9601 = v12963;
                                } else {
                                    let v12969 = v12968 - v12963;
                                    let v2713 = (v1262 - v2710) - v1265;
                                    let v2715 = (v85 * v1262) * v1265;
                                    let v12971 = (v12968 * v85) * v1265;
                                    let v2716 = if v2715 > v0 { 1.0 } else { 0.0 };
                                    let v2718: f64;
                                    let v9602: Lanes<4>;
                                    if v2716 != 0.0 {
                                        v2718 = v2715;
                                        v9602 = v12971;
                                    } else {
                                        let v2717 = -v2715;
                                        let v12972 = v12971 * v10382;
                                        v2718 = v2717;
                                        v9602 = v12972;
                                    }
                                    let v12973 = v12969 * v2713;
                                    let v2721 = ((v2713 * v2713) + v2718).sqrt();
                                    let v2724 = v1262 - (v8 * (v2713 + v2721));
                                    let v12981 = v12968 - ((v12969 + (((v12973 + v12973) + v9602) * (v9360 / (v10427 * v2721)))) * v8);
                                    v2876 = v2724;
                                    v9601 = v12981;
                                }
                                let v12982 = Lanes([v9601[0], v9601[1], v9601[2], 0.0, v9601[3], 0.0]);
                                v2875 = v2876;
                                v9599 = v12982;
                            } else {
                                let v2730 = -((v1239 - v2702) - (((v1223 / v73) * v7) / v118));
                                let v12908 = ((Lanes([v9461[0], v9461[1], v9461[2], 0.0, v9461[3], 0.0])) - v12905) * v10382;
                                let v2732 = (v73 * v2730) + v1242;
                                let v12911 = (v12908 * v73) + (Lanes([0.0, 0.0, v11959[0], 0.0, 0.0, 0.0]));
                                let v12912 = v12911 * v2732;
                                let v2734 = v2730 * v2730;
                                let v12914 = v12908 * v2730;
                                let v12915 = v12914 + v12914;
                                let v2737 = (v2732 * v2732) - (v85 * (v2734 + v1238));
                                let v12919 = (v12912 + v12912) - ((v12915 + (Lanes([0.0, 0.0, v11954[0], 0.0, 0.0, 0.0]))) * v85);
                                let v2739 = if v2737 >= v2738 { 1.0 } else { 0.0 };
                                let v2741: f64;
                                let v9603: Lanes<6>;
                                if v2739 != 0.0 {
                                    v2741 = v2737;
                                    v9603 = v12919;
                                } else {
                                    v2741 = v2740;
                                    v9603 = v11054;
                                }
                                let v2742 = v2741.sqrt();
                                let v2744 = (v2732 - v2742) / v73;
                                let v12924 = (v12911 - (v9603 * (v9360 / (v10427 * v2742)))) / v73;
                                let v2745 = v2734 / v1238;
                                let v12925 = v11954 * v2745;
                                let v2746 = v2745 / v1257;
                                let v12929 = v9397 * v2746;
                                let v2748 = v73 / v2730;
                                let v2749 = v658 + v2748;
                                let v2750 = (v2746.ln()) / v2749;
                                let v12942 = ((((((v12915 - (Lanes([0.0, 0.0, v12925[0], 0.0, 0.0, 0.0]))) / v1238) - (Lanes([0.0, 0.0, v12929[0], 0.0, 0.0, 0.0]))) / v1257) * (v9360 / v2746)) - (((Lanes([0.0, 0.0, v10402[0], 0.0, 0.0, 0.0])) + (((v12908 * v2748) * v10382) / v2730)) * v2750)) / v2749;
                                let v2751 = if v2744 < v1234 { 1.0 } else { 0.0 };
                                let v2877: f64;
                                let v9604: Lanes<6>;
                                if v2751 != 0.0 {
                                    v2877 = v2744;
                                    v9604 = v12924;
                                } else {
                                    let v12943 = v12942 - v12924;
                                    let v2753 = (v2750 - v2744) - v1265;
                                    let v2755 = (v85 * v2750) * v1265;
                                    let v12945 = (v12942 * v85) * v1265;
                                    let v2756 = if v2755 > v0 { 1.0 } else { 0.0 };
                                    let v2758: f64;
                                    let v9605: Lanes<6>;
                                    if v2756 != 0.0 {
                                        v2758 = v2755;
                                        v9605 = v12945;
                                    } else {
                                        let v2757 = -v2755;
                                        let v12946 = v12945 * v10382;
                                        v2758 = v2757;
                                        v9605 = v12946;
                                    }
                                    let v12947 = v12943 * v2753;
                                    let v2761 = ((v2753 * v2753) + v2758).sqrt();
                                    let v2764 = v2750 - (v8 * (v2753 + v2761));
                                    let v12955 = v12942 - ((v12943 + (((v12947 + v12947) + v9605) * (v9360 / (v10427 * v2761)))) * v8);
                                    v2877 = v2764;
                                    v9604 = v12955;
                                }
                                v2875 = v2877;
                                v9599 = v9604;
                            }
                            let v2768 = if ((v2765 * v2702) / v473) > v0 { 1.0 } else { 0.0 };
                            let v3328: f64;
                            if v2768 != 0.0 {
                                let v2772 = ((v2769 * v2702) / v473).sqrt();
                                v3328 = v2772;
                            } else {
                                v3328 = v0;
                            }
                            let v2773 = if v2703 != 0.0 && v0 != 0.0 { 1.0 } else { 0.0 };
                            let v2995: f64;
                            let v3012: f64;
                            let v3351: f64;
                            let v3464: f64;
                            let v9606: Lanes<6>;
                            let v9607: Lanes<6>;
                            let v9608: Lanes<6>;
                            if v2773 != 0.0 {
                                let mut v2774: f64 = 0.0;
                                let mut v2776: f64 = 0.0;
                                let mut v2879: f64 = 0.0;
                                let mut v9609: Lanes<6> = Lanes([0.0; 6]);
                                let mut v9610: Lanes<6> = Lanes([0.0; 6]);
                                v2774 = v0;
                                v2776 = v2875;
                                v2879 = v0;
                                v9609 = v9599;
                                v9610 = v11054;
                                loop {
                                    let v2775 = if v2774 < v13 { 1.0 } else { 0.0 };
                                    if v2775 == 0.0 {
                                        break;
                                    }
                                    let v2777 = v658 * v2776;
                                    let v13119 = v10402 * v2776;
                                    let v13122 = (Lanes([0.0, 0.0, v13119[0], 0.0, 0.0, 0.0])) + (v9609 * v658);
                                    let v2779 = (-v2777).exp();
                                    let v13124 = (v13122 * v10382) * v2779;
                                    let v2780 = if v2776 > v611 { 1.0 } else { 0.0 };
                                    let v2814: f64;
                                    let v2847: f64;
                                    let v9611: Lanes<6>;
                                    let v9612: Lanes<6>;
                                    if v2780 != 0.0 {
                                        let v2781 = v2777.exp();
                                        let v2782 = -v1235;
                                        let v2785 = v2781 - v4;
                                        let v13163 = v9397 * v2785;
                                        let v13164 = (v13122 * v2781) * v1257;
                                        let v2788 = (((v2779 + v2777) - v4) + (v1257 * v2785)).sqrt();
                                        let v2789 = v2782 * v2788;
                                        let v13171 = (v9396 * v10382) * v2788;
                                        let v13174 = (Lanes([0.0, 0.0, v13171[0], 0.0, 0.0, 0.0])) + ((((v13124 + v13122) + ((Lanes([0.0, 0.0, v13163[0], 0.0, 0.0, 0.0])) + v13164)) * (v9360 / (v10427 * v2788))) * v2782);
                                        let v2790 = v207 / v2789;
                                        let v13179 = v9397 * v2781;
                                        let v2794 = ((-v2779) + v4) + (v1257 * v2781);
                                        let v2795 = v2790 * v2794;
                                        let v13185 = ((((v13174 * v2790) * v10382) / v2789) * v2794) + (((v13124 * v10382) + ((Lanes([0.0, 0.0, v13179[0], 0.0, 0.0, 0.0])) + v13164)) * v2790);
                                        v2814 = v2789;
                                        v2847 = v2795;
                                        v9611 = v13174;
                                        v9612 = v13185;
                                    } else {
                                        let v2797 = if v2776 < v2796 { 1.0 } else { 0.0 };
                                        let v2815: f64;
                                        let v2848: f64;
                                        let v9613: Lanes<6>;
                                        let v9614: Lanes<6>;
                                        if v2797 != 0.0 {
                                            let v2800 = ((v2779 + v2777) - v4).sqrt();
                                            let v2801 = v1235 * v2800;
                                            let v13149 = v9396 * v2800;
                                            let v13152 = (Lanes([0.0, 0.0, v13149[0], 0.0, 0.0, 0.0])) + (((v13124 + v13122) * (v9360 / (v10427 * v2800))) * v1235);
                                            let v2802 = v207 / v2801;
                                            let v2804 = (-v2779) + v4;
                                            let v2805 = v2802 * v2804;
                                            let v13159 = ((((v13152 * v2802) * v10382) / v2801) * v2804) + ((v13124 * v10382) * v2802);
                                            v2815 = v2801;
                                            v2848 = v2805;
                                            v9613 = v13152;
                                            v9614 = v13159;
                                        } else {
                                            let v2806 = v207 / v658;
                                            let v2807 = v2806.sqrt();
                                            let v2808 = -v2807;
                                            let v2809 = v2808 * v658;
                                            let v2810 = v2809 * v2776;
                                            let v13135 = (((((((v10402 * v2806) * v10382) / v658) * (v9360 / (v10427 * v2807))) * v10382) * v658) + (v10402 * v2808)) * v2776;
                                            let v13138 = (Lanes([0.0, 0.0, v13135[0], 0.0, 0.0, 0.0])) + (v9609 * v2809);
                                            let v2812 = (v207 * v658).sqrt();
                                            let v2813 = -v2812;
                                            let v13143 = ((v10402 * v207) * (v9360 / (v10427 * v2812))) * v10382;
                                            let v13144 = Lanes([0.0, 0.0, v13143[0], 0.0, 0.0, 0.0]);
                                            v2815 = v2810;
                                            v2848 = v2813;
                                            v9613 = v13138;
                                            v9614 = v13144;
                                        }
                                        v2814 = v2815;
                                        v2847 = v2848;
                                        v9611 = v9613;
                                        v9612 = v9614;
                                    }
                                    let v13186 = v9611 * v2814;
                                    let v2820 = ((v2814 * v2814) + ((v85 * v1225) * v1225)).sqrt();
                                    let v13190 = (v13186 + v13186) * (v9360 / (v10427 * v2820));
                                    let v2821 = v2814 / v2820;
                                    let v2823 = v8 * (v4 + v2821);
                                    let v13194 = ((v9611 - (v13190 * v2821)) / v2820) * v8;
                                    let v13196 = (v9611 + v13190) * v8;
                                    let v2827 = (v8 * (v2814 + v2820)) + (v531 * v1225);
                                    let v2828 = if v2827 < v0 { 1.0 } else { 0.0 };
                                    let v2829: f64;
                                    let v2846: f64;
                                    let v9615: Lanes<6>;
                                    let v9616: Lanes<6>;
                                    if v2828 != 0.0 {
                                        v2829 = v0;
                                        v2846 = v0;
                                        v9615 = v11054;
                                        v9616 = v11054;
                                    } else {
                                        v2829 = v2827;
                                        v2846 = v2823;
                                        v9615 = v13196;
                                        v9616 = v13194;
                                    }
                                    let v13197 = v9615 * v10382;
                                    let v2831 = (v1224 - v2829) - v1227;
                                    let v2833 = (v85 * v1224) * v1227;
                                    let v2834 = if v2833 > v0 { 1.0 } else { 0.0 };
                                    let v2836: f64;
                                    if v2834 != 0.0 {
                                        v2836 = v2833;
                                    } else {
                                        let v2835 = -v2833;
                                        v2836 = v2835;
                                    }
                                    let v13198 = v13197 * v2831;
                                    let v2839 = ((v2831 * v2831) + v2836).sqrt();
                                    let v13202 = (v13198 + v13198) * (v9360 / (v10427 * v2839));
                                    let v2840 = v2831 / v2839;
                                    let v2842 = v8 * (v4 + v2840);
                                    let v2845 = v1224 - (v8 * (v2831 + v2839));
                                    let v13209 = ((v13197 + v13202) * v8) * v10382;
                                    let v2849 = v2847 * v2842;
                                    let v2850 = v2846 * v2849;
                                    let v13216 = v13209 * v2845;
                                    let v2855 = ((((v2845 * v2845) / v73) / v118) / v202) / v473;
                                    let v13221 = ((((v13216 + v13216) / v73) / v118) / v202) / v473;
                                    let v2856 = v73 * v2855;
                                    let v2858 = (v2856 * v2850) / v2845;
                                    let v2867 = (v2864 + (v2847 / v125)) + v2858;
                                    let v2868 = ((((-v2776) + (v2814 / v125)) - v1239) + v2855) / v2867;
                                    let v2869 = v2776 - v2868;
                                    let v13240 = v9609 - ((((((v9609 * v10382) + (v9611 / v125)) - (Lanes([v9461[0], v9461[1], v9461[2], 0.0, v9461[3], 0.0]))) + v13221) - (((v9612 / v125) + (((((v13221 * v73) * v2850) + (((v9616 * v2849) + (((v9612 * v2842) + ((((v13197 - (v13202 * v2840)) / v2839) * v8) * v2847)) * v2846)) * v2856)) - (v13209 * v2858)) / v2845)) * v2868)) / v2867);
                                    let v2872 = if ((v2869 - v2776).abs()) < v856 { 1.0 } else { 0.0 };
                                    let v2873: f64;
                                    if v2872 != 0.0 {
                                        v2873 = v13;
                                    } else {
                                        v2873 = v2774;
                                    }
                                    let v2874 = v2873 + v4;
                                    v2774 = v2874;
                                    v2776 = v2869;
                                    v2879 = v2814;
                                    v9609 = v13240;
                                    v9610 = v9611;
                                }
                                let v2878 = v1239 + v2776;
                                let v13116 = (Lanes([v9461[0], v9461[1], v9461[2], 0.0, v9461[3], 0.0])) + v9609;
                                let v2881 = v2878 - (v2879 / v125);
                                let v13118 = v13116 - (v9610 / v125);
                                v2995 = v2881;
                                v3012 = v2878;
                                v3351 = v2879;
                                v3464 = v4;
                                v9606 = v13118;
                                v9607 = v13116;
                                v9608 = v9610;
                            } else {
                                let mut v2882: f64 = 0.0;
                                let mut v2884: f64 = 0.0;
                                let mut v2992: f64 = 0.0;
                                let mut v9617: Lanes<6> = Lanes([0.0; 6]);
                                let mut v9618: Lanes<6> = Lanes([0.0; 6]);
                                v2882 = v0;
                                v2884 = v2875;
                                v2992 = v0;
                                v9617 = v9599;
                                v9618 = v11054;
                                loop {
                                    let v2883 = if v2882 < v13 { 1.0 } else { 0.0 };
                                    if v2883 == 0.0 {
                                        break;
                                    }
                                    let v2885 = v658 * v2884;
                                    let v12987 = v10402 * v2884;
                                    let v12990 = (Lanes([0.0, 0.0, v12987[0], 0.0, 0.0, 0.0])) + (v9617 * v658);
                                    let v2887 = (-v2885).exp();
                                    let v12992 = (v12990 * v10382) * v2887;
                                    let v2888 = if v2884 > v611 { 1.0 } else { 0.0 };
                                    let v2922: f64;
                                    let v2955: f64;
                                    let v9619: Lanes<6>;
                                    let v9620: Lanes<6>;
                                    if v2888 != 0.0 {
                                        let v2889 = v2885.exp();
                                        let v2890 = -v1235;
                                        let v2893 = v2889 - v4;
                                        let v13031 = v9397 * v2893;
                                        let v13032 = (v12990 * v2889) * v1257;
                                        let v2896 = (((v2887 + v2885) - v4) + (v1257 * v2893)).sqrt();
                                        let v2897 = v2890 * v2896;
                                        let v13039 = (v9396 * v10382) * v2896;
                                        let v13042 = (Lanes([0.0, 0.0, v13039[0], 0.0, 0.0, 0.0])) + ((((v12992 + v12990) + ((Lanes([0.0, 0.0, v13031[0], 0.0, 0.0, 0.0])) + v13032)) * (v9360 / (v10427 * v2896))) * v2890);
                                        let v2898 = v207 / v2897;
                                        let v13047 = v9397 * v2889;
                                        let v2902 = ((-v2887) + v4) + (v1257 * v2889);
                                        let v2903 = v2898 * v2902;
                                        let v13053 = ((((v13042 * v2898) * v10382) / v2897) * v2902) + (((v12992 * v10382) + ((Lanes([0.0, 0.0, v13047[0], 0.0, 0.0, 0.0])) + v13032)) * v2898);
                                        v2922 = v2897;
                                        v2955 = v2903;
                                        v9619 = v13042;
                                        v9620 = v13053;
                                    } else {
                                        let v2905 = if v2884 < v2904 { 1.0 } else { 0.0 };
                                        let v2923: f64;
                                        let v2956: f64;
                                        let v9621: Lanes<6>;
                                        let v9622: Lanes<6>;
                                        if v2905 != 0.0 {
                                            let v2908 = ((v2887 + v2885) - v4).sqrt();
                                            let v2909 = v1235 * v2908;
                                            let v13017 = v9396 * v2908;
                                            let v13020 = (Lanes([0.0, 0.0, v13017[0], 0.0, 0.0, 0.0])) + (((v12992 + v12990) * (v9360 / (v10427 * v2908))) * v1235);
                                            let v2910 = v207 / v2909;
                                            let v2912 = (-v2887) + v4;
                                            let v2913 = v2910 * v2912;
                                            let v13027 = ((((v13020 * v2910) * v10382) / v2909) * v2912) + ((v12992 * v10382) * v2910);
                                            v2923 = v2909;
                                            v2956 = v2913;
                                            v9621 = v13020;
                                            v9622 = v13027;
                                        } else {
                                            let v2914 = v207 / v658;
                                            let v2915 = v2914.sqrt();
                                            let v2916 = -v2915;
                                            let v2917 = v2916 * v658;
                                            let v2918 = v2917 * v2884;
                                            let v13003 = (((((((v10402 * v2914) * v10382) / v658) * (v9360 / (v10427 * v2915))) * v10382) * v658) + (v10402 * v2916)) * v2884;
                                            let v13006 = (Lanes([0.0, 0.0, v13003[0], 0.0, 0.0, 0.0])) + (v9617 * v2917);
                                            let v2920 = (v207 * v658).sqrt();
                                            let v2921 = -v2920;
                                            let v13011 = ((v10402 * v207) * (v9360 / (v10427 * v2920))) * v10382;
                                            let v13012 = Lanes([0.0, 0.0, v13011[0], 0.0, 0.0, 0.0]);
                                            v2923 = v2918;
                                            v2956 = v2921;
                                            v9621 = v13006;
                                            v9622 = v13012;
                                        }
                                        v2922 = v2923;
                                        v2955 = v2956;
                                        v9619 = v9621;
                                        v9620 = v9622;
                                    }
                                    let v13054 = v9619 * v2922;
                                    let v2928 = ((v2922 * v2922) + ((v85 * v1225) * v1225)).sqrt();
                                    let v13058 = (v13054 + v13054) * (v9360 / (v10427 * v2928));
                                    let v2929 = v2922 / v2928;
                                    let v2931 = v8 * (v4 + v2929);
                                    let v13062 = ((v9619 - (v13058 * v2929)) / v2928) * v8;
                                    let v13064 = (v9619 + v13058) * v8;
                                    let v2935 = (v8 * (v2922 + v2928)) + (v531 * v1225);
                                    let v2936 = if v2935 < v0 { 1.0 } else { 0.0 };
                                    let v2937: f64;
                                    let v2954: f64;
                                    let v9623: Lanes<6>;
                                    let v9624: Lanes<6>;
                                    if v2936 != 0.0 {
                                        v2937 = v0;
                                        v2954 = v0;
                                        v9623 = v11054;
                                        v9624 = v11054;
                                    } else {
                                        v2937 = v2935;
                                        v2954 = v2931;
                                        v9623 = v13064;
                                        v9624 = v13062;
                                    }
                                    let v13065 = v9623 * v10382;
                                    let v2939 = (v1224 - v2937) - v1227;
                                    let v2941 = (v85 * v1224) * v1227;
                                    let v2942 = if v2941 > v0 { 1.0 } else { 0.0 };
                                    let v2944: f64;
                                    if v2942 != 0.0 {
                                        v2944 = v2941;
                                    } else {
                                        let v2943 = -v2941;
                                        v2944 = v2943;
                                    }
                                    let v13066 = v13065 * v2939;
                                    let v2947 = ((v2939 * v2939) + v2944).sqrt();
                                    let v13070 = (v13066 + v13066) * (v9360 / (v10427 * v2947));
                                    let v2948 = v2939 / v2947;
                                    let v2950 = v8 * (v4 + v2948);
                                    let v2953 = v1224 - (v8 * (v2939 + v2947));
                                    let v13077 = ((v13065 + v13070) * v8) * v10382;
                                    let v2957 = v2955 * v2950;
                                    let v2958 = v2954 * v2957;
                                    let v13084 = v13077 * v2953;
                                    let v2963 = ((((v2953 * v2953) / v73) / v118) / v202) / v473;
                                    let v13089 = ((((v13084 + v13084) / v73) / v118) / v202) / v473;
                                    let v2964 = v73 * v2963;
                                    let v2966 = (v2964 * v2958) / v2953;
                                    let v2983 = ((v2977 + (v2955 / v125)) + ((v2955 * v7) / v118)) + v2966;
                                    let v2984 = (((((v2702 - v2884) + (v2922 / v125)) + (((v2922 + (v1223 / v73)) * v7) / v118)) - v1239) + v2963) / v2983;
                                    let v2985 = v2884 - v2984;
                                    let v13114 = v9617 - (((((((v12905 - v9617) + (v9619 / v125)) + ((v9619 * v7) / v118)) - (Lanes([v9461[0], v9461[1], v9461[2], 0.0, v9461[3], 0.0]))) + v13089) - ((((v9620 / v125) + ((v9620 * v7) / v118)) + (((((v13089 * v73) * v2958) + (((v9624 * v2957) + (((v9620 * v2950) + ((((v13065 - (v13070 * v2948)) / v2947) * v8) * v2955)) * v2954)) * v2964)) - (v13077 * v2966)) / v2953)) * v2984)) / v2983);
                                    let v2988 = if ((v2985 - v2884).abs()) < v856 { 1.0 } else { 0.0 };
                                    let v2989: f64;
                                    if v2988 != 0.0 {
                                        v2989 = v13;
                                    } else {
                                        v2989 = v2882;
                                    }
                                    let v2990 = v2989 + v4;
                                    v2882 = v2990;
                                    v2884 = v2985;
                                    v2992 = v2922;
                                    v9617 = v13114;
                                    v9618 = v9619;
                                }
                                let v2991 = v1239 + v2884;
                                let v12984 = (Lanes([v9461[0], v9461[1], v9461[2], 0.0, v9461[3], 0.0])) + v9617;
                                let v2994 = v2991 - (v2992 / v125);
                                let v12986 = v12984 - (v9618 / v125);
                                v2995 = v2994;
                                v3012 = v2991;
                                v3351 = v2992;
                                v3464 = v73;
                                v9606 = v12986;
                                v9607 = v12984;
                                v9608 = v9618;
                            }
                            let v2996 = if v2995 < v0 { 1.0 } else { 0.0 };
                            let v3000: f64;
                            let v9625: Lanes<6>;
                            if v2996 != 0.0 {
                                v3000 = v0;
                                v9625 = v11054;
                            } else {
                                v3000 = v2995;
                                v9625 = v9606;
                            }
                            v2999 = v3000;
                            v3004 = v2702;
                            v3011 = v3012;
                            v3327 = v3328;
                            v3350 = v3351;
                            v3463 = v3464;
                            v9590 = v9625;
                            v9591 = v12905;
                            v9592 = v9607;
                            v9593 = v9608;
                        }
                        v2998 = v2999;
                        v3003 = v3004;
                        v3010 = v3011;
                        v3325 = v3327;
                        v3349 = v3350;
                        v3462 = v3463;
                        v9586 = v9590;
                        v9587 = v9591;
                        v9588 = v9592;
                        v9589 = v9593;
                    }
                    let v2997 = if v2572 < v0 { 1.0 } else { 0.0 };
                    let v3002: f64;
                    let v9626: Lanes<6>;
                    if v2997 != 0.0 {
                        v3002 = v2572;
                        v9626 = v9562;
                    } else {
                        v3002 = v3003;
                        v9626 = v9587;
                    }
                    let v3001 = if v2998 < v15 { 1.0 } else { 0.0 };
                    let v3009: f64;
                    let v9627: Lanes<6>;
                    if v3001 != 0.0 {
                        let v3008 = v3002 + (v120 * ((v8 * v1223) + v2594));
                        let v13242 = v9626 + (v9533 * v120);
                        v3009 = v3008;
                        v9627 = v13242;
                    } else {
                        v3009 = v2998;
                        v9627 = v9586;
                    }
                    let mut v3013: f64 = 0.0;
                    let mut v3015: f64 = 0.0;
                    let mut v3051: f64 = 0.0;
                    let mut v3074: f64 = 0.0;
                    let mut v3207: f64 = 0.0;
                    let mut v3319: f64 = 0.0;
                    let mut v3330: f64 = 0.0;
                    let mut v3341: f64 = 0.0;
                    let mut v3348: f64 = 0.0;
                    let mut v9628: Lanes<6> = Lanes([0.0; 6]);
                    let mut v9629: Lanes<6> = Lanes([0.0; 6]);
                    let mut v9630: Lanes<6> = Lanes([0.0; 6]);
                    let mut v9631: Lanes<6> = Lanes([0.0; 6]);
                    let mut v9632: Lanes<6> = Lanes([0.0; 6]);
                    let mut v9633: Lanes<6> = Lanes([0.0; 6]);
                    v3013 = v4;
                    v3015 = v3010;
                    v3051 = v3002;
                    v3074 = v3009;
                    v3207 = v0;
                    v3319 = v0;
                    v3330 = v0;
                    v3341 = v0;
                    v3348 = v3349;
                    v9628 = v9588;
                    v9629 = v9626;
                    v9630 = v9627;
                    v9631 = v11054;
                    v9632 = v11054;
                    v9633 = v9589;
                    loop {
                        let v3014 = if v3013 <= v13 { 1.0 } else { 0.0 };
                        if v3014 == 0.0 {
                            break;
                        }
                        let v3016 = v3015 - v1239;
                        let v3017 = v658 * v3016;
                        let v13328 = v10402 * v3016;
                        let v13331 = (Lanes([0.0, 0.0, v13328[0], 0.0, 0.0, 0.0])) + ((v9628 - (Lanes([v9461[0], v9461[1], v9461[2], 0.0, v9461[3], 0.0]))) * v658);
                        let v3019 = (-v3017).exp();
                        let v13333 = (v13331 * v10382) * v3019;
                        let v3021 = if v3016 < v3020 { 1.0 } else { 0.0 };
                        let v3212: f64;
                        let v3225: f64;
                        let v9634: Lanes<6>;
                        let v9635: Lanes<6>;
                        if v3021 != 0.0 {
                            let v3024 = ((v3019 + v3017) - v4).sqrt();
                            let v3025 = v1235 * v3024;
                            let v13373 = v9396 * v3024;
                            let v13376 = (Lanes([0.0, 0.0, v13373[0], 0.0, 0.0, 0.0])) + (((v13333 + v13331) * (v9360 / (v10427 * v3024))) * v1235);
                            let v3029 = (v207 * ((-v3019) + v4)) / v3025;
                            let v13381 = (((v13333 * v10382) * v207) - (v13376 * v3029)) / v3025;
                            v3212 = v3025;
                            v3225 = v3029;
                            v9634 = v13376;
                            v9635 = v13381;
                        } else {
                            let v3030 = if v3016 > v611 { 1.0 } else { 0.0 };
                            let v3213: f64;
                            let v3226: f64;
                            let v9636: Lanes<6>;
                            let v9637: Lanes<6>;
                            if v3030 != 0.0 {
                                let v3031 = v3017.exp();
                                let v13343 = v13331 * v3031;
                                let v3032 = -v1235;
                                let v3036 = (v3031 + v3017) - v4;
                                let v13347 = v9397 * v3036;
                                let v3039 = (((v3019 + v3017) - v4) + (v1257 * v3036)).sqrt();
                                let v3040 = v3032 * v3039;
                                let v13355 = (v9396 * v10382) * v3039;
                                let v13358 = (Lanes([0.0, 0.0, v13355[0], 0.0, 0.0, 0.0])) + ((((v13333 + v13331) + ((Lanes([0.0, 0.0, v13347[0], 0.0, 0.0, 0.0])) + ((v13343 + v13331) * v1257))) * (v9360 / (v10427 * v3039))) * v3032);
                                let v3043 = v3031 + v4;
                                let v13360 = v9397 * v3043;
                                let v3047 = (v207 * (((-v3019) + v4) + (v1257 * v3043))) / v3040;
                                let v13368 = ((((v13333 * v10382) + ((Lanes([0.0, 0.0, v13360[0], 0.0, 0.0, 0.0])) + (v13343 * v1257))) * v207) - (v13358 * v3047)) / v3040;
                                v3213 = v3040;
                                v3226 = v3047;
                                v9636 = v13358;
                                v9637 = v13368;
                            } else {
                                let v3048 = -v1235;
                                let v13334 = v9396 * v10382;
                                let v3049 = v3048 * v3017;
                                let v13335 = v13334 * v3017;
                                let v13338 = (Lanes([0.0, 0.0, v13335[0], 0.0, 0.0, 0.0])) + (v13331 * v3048);
                                let v3050 = v3048 * v658;
                                let v13341 = (v13334 * v658) + (v10402 * v3048);
                                let v13342 = Lanes([0.0, 0.0, v13341[0], 0.0, 0.0, 0.0]);
                                v3213 = v3049;
                                v3226 = v3050;
                                v9636 = v13338;
                                v9637 = v13342;
                            }
                            v3212 = v3213;
                            v3225 = v3226;
                            v9634 = v9636;
                            v9635 = v9637;
                        }
                        let v3052 = v3051 - v2666;
                        let v13384 = v10402 * v3052;
                        let v3054 = (v658 * v3052).exp();
                        let v13388 = ((Lanes([0.0, 0.0, v13384[0], 0.0, 0.0, 0.0])) + ((v9629 - (Lanes([v12885[0], v12885[1], v12885[2], v12885[3], v12885[4], 0.0]))) * v658)) * v3054;
                        let v13389 = v12070 * v1499;
                        let v3056 = v745 * v745;
                        let v13391 = v10477 * v745;
                        let v3057 = (v1499 * v1499) / v3056;
                        let v13393 = (v13391 + v13391) * v3057;
                        let v13396 = ((v13389 + v13389) - (Lanes([0.0, 0.0, v13393[0], 0.0, 0.0]))) / v3056;
                        let v3058 = v73 * v754;
                        let v3060 = (v3054 + v3017) - v4;
                        let v13399 = (v10488 * v73) * v3060;
                        let v3063 = (v3057 + (v3058 * v3060)).sqrt();
                        let v13407 = ((Lanes([v13396[0], v13396[1], v13396[2], v13396[3], v13396[4], 0.0])) + ((Lanes([0.0, 0.0, v13399[0], 0.0, 0.0, 0.0])) + ((v13388 + v13331) * v3058))) * (v9360 / (v10427 * v3063));
                        let v3064 = v73 * v658;
                        let v3065 = v3064 * v754;
                        let v3066 = v3054 + v4;
                        let v13412 = (((v10402 * v73) * v754) + (v10488 * v3064)) * v3066;
                        let v3068 = v73 * v3063;
                        let v3069 = (v3065 * v3066) / v3068;
                        let v3070 = -v745;
                        let v13420 = v10477 * v10382;
                        let v13421 = v13420 * v3063;
                        let v3072 = (v3070 * v3063) - v1499;
                        let v13425 = Lanes([v12070[0], v12070[1], v12070[2], v12070[3], v12070[4], 0.0]);
                        let v13426 = ((Lanes([0.0, 0.0, v13421[0], 0.0, 0.0, 0.0])) + (v13407 * v3070)) - v13425;
                        let v3073 = v3070 * v3069;
                        let v13427 = v13420 * v3069;
                        let v13430 = (Lanes([0.0, 0.0, v13427[0], 0.0, 0.0, 0.0])) + (((((Lanes([0.0, 0.0, v13412[0], 0.0, 0.0, 0.0])) + (v13388 * v3065)) - ((v13407 * v73) * v3069)) / v3068) * v3070);
                        let v3076 = (v3074 - v3051) / v1203;
                        let v3077 = v658 * v3076;
                        let v13433 = v10402 * v3076;
                        let v13436 = (Lanes([0.0, 0.0, v13433[0], 0.0, 0.0, 0.0])) + (((v9630 - v9629) / v1203) * v658);
                        let v3078 = -v3077;
                        let v13437 = v13436 * v10382;
                        let v3079 = if v3078 >= v2321 { 1.0 } else { 0.0 };
                        let v3090: f64;
                        let v3098: f64;
                        let v9638: Lanes<6>;
                        let v9639: Lanes<6>;
                        if v3079 != 0.0 {
                            let v3082 = v2323 * ((v4 + v3078) - v2321);
                            let v13440 = v13437 * v2323;
                            v3090 = v3082;
                            v3098 = v2323;
                            v9638 = v13440;
                            v9639 = v11054;
                        } else {
                            let mut v3083: f64 = 0.0;
                            let mut v3085: f64 = 0.0;
                            let mut v9640: Lanes<6> = Lanes([0.0; 6]);
                            v3083 = v3078;
                            v3085 = v4;
                            v9640 = v13437;
                            loop {
                                let v3084 = if v3083 >= v2325 { 1.0 } else { 0.0 };
                                if v3084 == 0.0 {
                                    break;
                                }
                                let v3086 = v3085 * v2328;
                                let v3087 = v3083 - v2325;
                                let edge0 = v3087;
                                let edge1 = v3086;
                                let edge2 = v9640;
                                v3083 = edge0;
                                v3085 = edge1;
                                v9640 = edge2;
                            }
                            let v3088 = v3083.exp();
                            let v3089 = v3085 * v3088;
                            let v13439 = (v9640 * v3088) * v3085;
                            v3090 = v3089;
                            v3098 = v3089;
                            v9638 = v13439;
                            v9639 = v13439;
                        }
                        let v3093 = ((v3090 + v3077) - v4).sqrt();
                        let v13444 = (v9638 + v13436) * (v9360 / (v10427 * v3093));
                        let v3095 = if v3076 < v3094 { 1.0 } else { 0.0 };
                        let v3121: f64;
                        let v3158: f64;
                        let v3162: f64;
                        let v9641: Lanes<6>;
                        let v9642: Lanes<6>;
                        let v9643: Lanes<6>;
                        if v3095 != 0.0 {
                            let v3096 = v745 * v3093;
                            let v13475 = v10477 * v3093;
                            let v13478 = (Lanes([0.0, 0.0, v13475[0], 0.0, 0.0, 0.0])) + (v13444 * v745);
                            let v3097 = v745 * v658;
                            let v3100 = (-v3098) + v4;
                            let v13483 = ((v10477 * v658) + (v10402 * v745)) * v3100;
                            let v3102 = v73 * v3093;
                            let v3103 = (v3097 * v3100) / v3102;
                            let v3104 = v3103 / v1203;
                            let v13491 = ((((Lanes([0.0, 0.0, v13483[0], 0.0, 0.0, 0.0])) + ((v9639 * v10382) * v3097)) - ((v13444 * v73) * v3103)) / v3102) / v1203;
                            let v3105 = -v3104;
                            let v13492 = v13491 * v10382;
                            v3121 = v3096;
                            v3158 = v3104;
                            v3162 = v3105;
                            v9641 = v13478;
                            v9642 = v13491;
                            v9643 = v13492;
                        } else {
                            let v3106 = if v3076 > v611 { 1.0 } else { 0.0 };
                            let v3122: f64;
                            let v3159: f64;
                            let v3163: f64;
                            let v9644: Lanes<6>;
                            let v9645: Lanes<6>;
                            let v9646: Lanes<6>;
                            if v3106 != 0.0 {
                                let v3107 = v3070 * v3093;
                                let v13457 = v13420 * v3093;
                                let v13460 = (Lanes([0.0, 0.0, v13457[0], 0.0, 0.0, 0.0])) + (v13444 * v3070);
                                let v3108 = v3070 * v658;
                                let v3110 = (-v3098) + v4;
                                let v13465 = ((v13420 * v658) + (v10402 * v3070)) * v3110;
                                let v3112 = v73 * v3093;
                                let v3113 = (v3108 * v3110) / v3112;
                                let v3114 = v3113 / v1203;
                                let v13473 = ((((Lanes([0.0, 0.0, v13465[0], 0.0, 0.0, 0.0])) + ((v9639 * v10382) * v3108)) - ((v13444 * v73) * v3113)) / v3112) / v1203;
                                let v3115 = -v3114;
                                let v13474 = v13473 * v10382;
                                v3122 = v3107;
                                v3159 = v3114;
                                v3163 = v3115;
                                v9644 = v13460;
                                v9645 = v13473;
                                v9646 = v13474;
                            } else {
                                let v13445 = v13420 * v3077;
                                let v3117 = (v3070 * v3077) / v743;
                                let v13449 = ((Lanes([0.0, 0.0, v13445[0], 0.0, 0.0, 0.0])) + (v13436 * v3070)) / v743;
                                let v3119 = (v3070 * v658) / v743;
                                let v13453 = ((v13420 * v658) + (v10402 * v3070)) / v743;
                                let v3120 = -v3119;
                                let v13454 = v13453 * v10382;
                                let v13455 = Lanes([0.0, 0.0, v13453[0], 0.0, 0.0, 0.0]);
                                let v13456 = Lanes([0.0, 0.0, v13454[0], 0.0, 0.0, 0.0]);
                                v3122 = v3117;
                                v3159 = v3119;
                                v3163 = v3120;
                                v9644 = v13449;
                                v9645 = v13455;
                                v9646 = v13456;
                            }
                            v3121 = v3122;
                            v3158 = v3159;
                            v3162 = v3163;
                            v9641 = v9644;
                            v9642 = v9645;
                            v9643 = v9646;
                        }
                        let v3123 = -v1220;
                        let v13493 = v11944 * v10382;
                        let v3124 = v0 - v3123;
                        let v13494 = v13493 * v10382;
                        let v3127 = if (if v3121 > v3124 { 1.0 } else { 0.0 }) != 0.0 && (if v3123 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v3160: f64;
                        let v3165: f64;
                        let v9647: Lanes<6>;
                        let v9648: Lanes<6>;
                        if v3127 != 0.0 {
                            let v3128 = v3121 + v3123;
                            let v13496 = v9641 + (Lanes([v13493[0], v13493[1], v13493[2], v13493[3], v13493[4], 0.0]));
                            let v3129 = v3128 * v3128;
                            let v13497 = v13496 * v3128;
                            let v3130 = v3123 * v3123;
                            let v13499 = v13493 * v3123;
                            let v13501 = (v13497 + v13497) * v3129;
                            let v3132 = v3130 * v3130;
                            let v13503 = (v13499 + v13499) * v3130;
                            let v13504 = v13503 + v13503;
                            let v3133 = (v3129 * v3129) + v3132;
                            let v13506 = (v13501 + v13501) + (Lanes([v13504[0], v13504[1], v13504[2], v13504[3], v13504[4], 0.0]));
                            let v3150: f64;
                            let v9649: Lanes<6>;
                            if v3134 != 0.0 {
                                let v3144: f64;
                                if v3135 != 0.0 {
                                    v3144 = v4;
                                } else {
                                    let v3145: f64;
                                    if v3136 != 0.0 {
                                        v3145 = v73;
                                    } else {
                                        let v3146: f64;
                                        if v3137 != 0.0 {
                                            v3146 = v91;
                                        } else {
                                            let v3147: f64;
                                            if v3138 != 0.0 {
                                                v3147 = v85;
                                            } else {
                                                v3147 = v0;
                                            }
                                            v3146 = v3147;
                                        }
                                        v3145 = v3146;
                                    }
                                    v3144 = v3145;
                                }
                                let mut v3139: f64 = 0.0;
                                let mut v3141: f64 = 0.0;
                                let mut v9650: Lanes<6> = Lanes([0.0; 6]);
                                v3139 = v0;
                                v3141 = v3133;
                                v9650 = v13506;
                                loop {
                                    let v3140 = if v3139 < v3144 { 1.0 } else { 0.0 };
                                    if v3140 == 0.0 {
                                        break;
                                    }
                                    let v3142 = v3141.sqrt();
                                    let v13728 = v9650 * (v9360 / (v10427 * v3142));
                                    let v3143 = v3139 + v4;
                                    v3139 = v3143;
                                    v3141 = v3142;
                                    v9650 = v13728;
                                }
                                v3150 = v3141;
                                v9649 = v9650;
                            } else {
                                let v3149 = v3133.powf(v3148);
                                let v13510 = v13506 * (v3148 * (v3133.powf(v13507)));
                                v3150 = v3149;
                                v9649 = v13510;
                            }
                            let v3151 = v4 / v3150;
                            let v13513 = ((v9649 * v3151) * v10382) / v3150;
                            let v3152 = v3128 * v3123;
                            let v13515 = v13493 * v3128;
                            let v3154 = v3123 * v3132;
                            let v13524 = ((v13493 * v3132) + (v13504 * v3123)) * v3151;
                            let v3156 = (v3154 * v3151) / v3133;
                            let v13530 = (((Lanes([v13524[0], v13524[1], v13524[2], v13524[3], v13524[4], 0.0])) + (v13513 * v3154)) - (v13506 * v3156)) / v3133;
                            let v3157 = v3124 + (v3152 * v3151);
                            let v13532 = (Lanes([v13494[0], v13494[1], v13494[2], v13494[3], v13494[4], 0.0])) + ((((v13496 * v3123) + (Lanes([v13515[0], v13515[1], v13515[2], v13515[3], v13515[4], 0.0]))) * v3151) + (v13513 * v3152));
                            v3160 = v3156;
                            v3165 = v3157;
                            v9647 = v13530;
                            v9648 = v13532;
                        } else {
                            v3160 = v4;
                            v3165 = v3121;
                            v9647 = v11054;
                            v9648 = v9641;
                        }
                        let v3161 = v3158 * v3160;
                        let v13535 = (v9642 * v3160) + (v9647 * v3158);
                        let v3164 = v3162 * v3160;
                        let v13538 = (v9643 * v3160) + (v9647 * v3162);
                        let v3166 = v1223 - v1499;
                        let v13539 = v12070 * v10382;
                        let v3167 = -v3166;
                        let v13540 = v13539 * v10382;
                        let v3168 = v3166 + v3167;
                        let v13541 = v13539 + v13540;
                        let v3171 = if (if v3165 < v3168 { 1.0 } else { 0.0 }) != 0.0 && (if v3167 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v3202: f64;
                        let v3205: f64;
                        let v9651: Lanes<6>;
                        let v9652: Lanes<6>;
                        if v3171 != 0.0 {
                            let v3172 = v3168 - v3165;
                            let v13542 = Lanes([v13541[0], v13541[1], v13541[2], v13541[3], v13541[4], 0.0]);
                            let v13543 = v13542 - v9648;
                            let v3173 = v3172 * v3172;
                            let v13544 = v13543 * v3172;
                            let v3174 = v3167 * v3167;
                            let v13546 = v13540 * v3167;
                            let v13548 = (v13544 + v13544) * v3173;
                            let v3176 = v3174 * v3174;
                            let v13550 = (v13546 + v13546) * v3174;
                            let v13551 = v13550 + v13550;
                            let v3177 = (v3173 * v3173) + v3176;
                            let v13553 = (v13548 + v13548) + (Lanes([v13551[0], v13551[1], v13551[2], v13551[3], v13551[4], 0.0]));
                            let v3194: f64;
                            let v9653: Lanes<6>;
                            if v3178 != 0.0 {
                                let v3188: f64;
                                if v3179 != 0.0 {
                                    v3188 = v4;
                                } else {
                                    let v3189: f64;
                                    if v3180 != 0.0 {
                                        v3189 = v73;
                                    } else {
                                        let v3190: f64;
                                        if v3181 != 0.0 {
                                            v3190 = v91;
                                        } else {
                                            let v3191: f64;
                                            if v3182 != 0.0 {
                                                v3191 = v85;
                                            } else {
                                                v3191 = v0;
                                            }
                                            v3190 = v3191;
                                        }
                                        v3189 = v3190;
                                    }
                                    v3188 = v3189;
                                }
                                let mut v3183: f64 = 0.0;
                                let mut v3185: f64 = 0.0;
                                let mut v9654: Lanes<6> = Lanes([0.0; 6]);
                                v3183 = v0;
                                v3185 = v3177;
                                v9654 = v13553;
                                loop {
                                    let v3184 = if v3183 < v3188 { 1.0 } else { 0.0 };
                                    if v3184 == 0.0 {
                                        break;
                                    }
                                    let v3186 = v3185.sqrt();
                                    let v13725 = v9654 * (v9360 / (v10427 * v3186));
                                    let v3187 = v3183 + v4;
                                    v3183 = v3187;
                                    v3185 = v3186;
                                    v9654 = v13725;
                                }
                                v3194 = v3185;
                                v9653 = v9654;
                            } else {
                                let v3193 = v3177.powf(v3192);
                                let v13557 = v13553 * (v3192 * (v3177.powf(v13554)));
                                v3194 = v3193;
                                v9653 = v13557;
                            }
                            let v3195 = v4 / v3194;
                            let v13560 = ((v9653 * v3195) * v10382) / v3194;
                            let v3196 = v3172 * v3167;
                            let v13562 = v13540 * v3172;
                            let v3198 = v3167 * v3176;
                            let v13571 = ((v13540 * v3176) + (v13551 * v3167)) * v3195;
                            let v3200 = (v3198 * v3195) / v3177;
                            let v13577 = (((Lanes([v13571[0], v13571[1], v13571[2], v13571[3], v13571[4], 0.0])) + (v13560 * v3198)) - (v13553 * v3200)) / v3177;
                            let v3201 = v3168 - (v3196 * v3195);
                            let v13578 = v13542 - ((((v13543 * v3167) + (Lanes([v13562[0], v13562[1], v13562[2], v13562[3], v13562[4], 0.0]))) * v3195) + (v13560 * v3196));
                            v3202 = v3200;
                            v3205 = v3201;
                            v9651 = v13577;
                            v9652 = v13578;
                        } else {
                            v3202 = v4;
                            v3205 = v3165;
                            v9651 = v11054;
                            v9652 = v9648;
                        }
                        let v3203 = v3164 * v3202;
                        let v13581 = (v13538 * v3202) + (v9651 * v3164);
                        let v3204 = v3161 * v3202;
                        let v13584 = (v13535 * v3202) + (v9651 * v3161);
                        let v3206 = v1499 + v3205;
                        let v13585 = v13425 + v9652;
                        let v3210 = if (if v3207 == v4 { 1.0 } else { 0.0 }) != 0.0 && (if v3013 > v91 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v3312: f64;
                        let v3314: f64;
                        let v3315: f64;
                        let v3316: f64;
                        let v3317: f64;
                        let v3320: f64;
                        let v9655: Lanes<6>;
                        let v9656: Lanes<6>;
                        let v9657: Lanes<6>;
                        if v3210 != 0.0 {
                            v3312 = v13;
                            v3314 = v3015;
                            v3315 = v3051;
                            v3316 = v3074;
                            v3317 = v3207;
                            v3320 = v3013;
                            v9655 = v9628;
                            v9656 = v9629;
                            v9657 = v9630;
                        } else {
                            let v3217 = (((v3212 + v1499) + v3072) + v3205) + v2564;
                            let v13592 = v9412 * v3217;
                            let v3219 = (v3051 - v1195) - (v1043 * v3217);
                            let v13596 = (v9629 - (Lanes([v10820[0], v10820[1], v10820[2], v10820[3], v10820[4], 0.0]))) - ((Lanes([v13592[0], v13592[1], 0.0, v13592[2], v13592[3], 0.0])) + (((((v9634 + v13425) + v13426) + v9652) + v9495) * v1043));
                            let v3220 = v3073 + v3203;
                            let v13598 = v9412 * v3220;
                            let v3222 = v4 - (v1043 * v3220);
                            let v13602 = ((Lanes([v13598[0], v13598[1], 0.0, v13598[2], v13598[3], 0.0])) + ((v13430 + v13581) * v1043)) * v10382;
                            let v3223 = -v1043;
                            let v13603 = v9412 * v10382;
                            let v3224 = v3223 * v3204;
                            let v13604 = v13603 * v3204;
                            let v13607 = (Lanes([v13604[0], v13604[1], 0.0, v13604[2], v13604[3], 0.0])) + (v13584 * v3223);
                            let v3227 = v3223 * v3225;
                            let v13608 = v13603 * v3225;
                            let v13611 = (Lanes([v13608[0], v13608[1], 0.0, v13608[2], v13608[3], 0.0])) + (v9635 * v3223);
                            let v3233 = v3074 - (v3051 + (v120 * ((v8 * v1223) + v3212)));
                            let v13615 = v9630 - (v9629 + (v9634 * v120));
                            let v3235 = -(v120 * v3225);
                            let v13616 = (v9635 * v120) * v10382;
                            let v3238 = (v3015 - v3074) - (v126 * v3212);
                            let v13619 = (v9628 - v9630) - (v9634 * v126);
                            let v3241 = v4 - (v126 * v3225);
                            let v13621 = (v9635 * v126) * v10382;
                            let v3242 = v3222 * v3241;
                            let v13624 = (v13602 * v3241) + (v13621 * v3222);
                            let v3243 = v3222 * v3235;
                            let v13627 = (v13602 * v3235) + (v13616 * v3222);
                            let v3246 = v3224 * v3234;
                            let v13630 = v13607 * v3234;
                            let v3249 = v3227 * v3234;
                            let v13635 = v13611 * v3234;
                            let v3252 = (((v3242 - (v3243 * v3239)) - (v3246 * v3241)) + (v3249 * v3239)) + v358;
                            let v3253 = v4 / v3252;
                            let v3255 = v3241 - (v3235 * v3239);
                            let v3258 = (v3227 * v3239) - (v3224 * v3241);
                            let v3260 = (v3224 * v3235) - v3227;
                            let v3261 = v3249 - v3243;
                            let v3263 = (-v3222) * v3239;
                            let v3264 = v3222 - v3246;
                            let v3265 = -v3253;
                            let v13656 = ((((((v13624 - (v13627 * v3239)) - ((v13630 * v3241) + (v13621 * v3246))) + (v13635 * v3239)) * v3253) * v10382) / v3252) * v10382;
                            let v3270 = ((v3255 * v3219) + (v3258 * v3233)) + (v3260 * v3238);
                            let v3271 = v3265 * v3270;
                            let v13670 = (v13656 * v3270) + ((((((v13621 - (v13616 * v3239)) * v3219) + (v13596 * v3255)) + ((((v13611 * v3239) - ((v13607 * v3241) + (v13621 * v3224))) * v3233) + (v13615 * v3258))) + (((((v13607 * v3235) + (v13616 * v3224)) - v13611) * v3238) + (v13619 * v3260))) * v3265);
                            let v3276 = ((v3241 * v3219) + (v3242 * v3233)) + (v3261 * v3238);
                            let v3277 = v3265 * v3276;
                            let v13684 = (v13656 * v3276) + (((((v13621 * v3219) + (v13596 * v3241)) + ((v13624 * v3233) + (v13615 * v3242))) + (((v13635 - v13627) * v3238) + (v13619 * v3261))) * v3265);
                            let v3281 = (v3219 + (v3263 * v3233)) + (v3264 * v3238);
                            let v3282 = v3265 * v3281;
                            let v13695 = (v13656 * v3281) + (((v13596 + ((((v13602 * v10382) * v3239) * v3233) + (v13615 * v3263))) + (((v13602 - v13630) * v3238) + (v13619 * v3264))) * v3265);
                            let v3283 = v3271.abs();
                            let v13699 = v13670 * ((v10427 * (if v3271 >= v11296 { 1.0 } else { 0.0 })) - v9360);
                            let v3284 = v3277.abs();
                            let v13703 = v13684 * ((v10427 * (if v3277 >= v11296 { 1.0 } else { 0.0 })) - v9360);
                            let v3285 = if v3283 < v3284 { 1.0 } else { 0.0 };
                            let v3286: f64;
                            let v9658: Lanes<6>;
                            if v3285 != 0.0 {
                                v3286 = v3284;
                                v9658 = v13703;
                            } else {
                                v3286 = v3283;
                                v9658 = v13699;
                            }
                            let v3287 = v3282.abs();
                            let v13707 = v13695 * ((v10427 * (if v3282 >= v11296 { 1.0 } else { 0.0 })) - v9360);
                            let v3288 = if v3286 < v3287 { 1.0 } else { 0.0 };
                            let v3293: f64;
                            let v9659: Lanes<6>;
                            if v3288 != 0.0 {
                                v3293 = v3287;
                                v9659 = v13707;
                            } else {
                                v3293 = v3286;
                                v9659 = v9658;
                            }
                            let v3289 = if v3013 > v2530 { 1.0 } else { 0.0 };
                            let v3294: f64;
                            if v3289 != 0.0 {
                                v3294 = v2532;
                            } else {
                                let v3290 = if v3013 > v2533 { 1.0 } else { 0.0 };
                                let v3295: f64;
                                if v3290 != 0.0 {
                                    v3295 = v2532;
                                } else {
                                    let v3291 = if v3013 > v816 { 1.0 } else { 0.0 };
                                    let v3296: f64;
                                    if v3291 != 0.0 {
                                        v3296 = v2536;
                                    } else {
                                        let v3292 = if v3013 > v10 { 1.0 } else { 0.0 };
                                        let v3297: f64;
                                        if v3292 != 0.0 {
                                            v3297 = v639;
                                        } else {
                                            v3297 = v4;
                                        }
                                        v3296 = v3297;
                                    }
                                    v3295 = v3296;
                                }
                                v3294 = v3295;
                            }
                            let v3298 = v74 / v3294;
                            let v3299 = if v3293 > v3298 { 1.0 } else { 0.0 };
                            let v3304: f64;
                            let v3306: f64;
                            let v3308: f64;
                            let v9660: Lanes<6>;
                            let v9661: Lanes<6>;
                            let v9662: Lanes<6>;
                            if v3299 != 0.0 {
                                let v3300 = v3298 / v3293;
                                let v13710 = ((v9659 * v3300) * v10382) / v3293;
                                let v3301 = v3271 * v3300;
                                let v13713 = (v13670 * v3300) + (v13710 * v3271);
                                let v3302 = v3277 * v3300;
                                let v13716 = (v13684 * v3300) + (v13710 * v3277);
                                let v3303 = v3282 * v3300;
                                let v13719 = (v13695 * v3300) + (v13710 * v3282);
                                v3304 = v3301;
                                v3306 = v3302;
                                v3308 = v3303;
                                v9660 = v13713;
                                v9661 = v13716;
                                v9662 = v13719;
                            } else {
                                v3304 = v3271;
                                v3306 = v3277;
                                v3308 = v3282;
                                v9660 = v13670;
                                v9661 = v13684;
                                v9662 = v13695;
                            }
                            let v3305 = v3051 + v3304;
                            let v13720 = v9629 + v9660;
                            let v3307 = v3074 + v3306;
                            let v13721 = v9630 + v9661;
                            let v3309 = v3015 + v3308;
                            let v13722 = v9628 + v9662;
                            let v3311 = if v3293 < (v856 * v3294) { 1.0 } else { 0.0 };
                            let v3318: f64;
                            if v3311 != 0.0 {
                                v3318 = v4;
                            } else {
                                v3318 = v3207;
                            }
                            v3312 = v3013;
                            v3314 = v3309;
                            v3315 = v3305;
                            v3316 = v3307;
                            v3317 = v3318;
                            v3320 = v3319;
                            v9655 = v13722;
                            v9656 = v13720;
                            v9657 = v13721;
                        }
                        let v3313 = v3312 + v4;
                        v3013 = v3313;
                        v3015 = v3314;
                        v3051 = v3315;
                        v3074 = v3316;
                        v3207 = v3317;
                        v3319 = v3320;
                        v3330 = v3072;
                        v3341 = v3206;
                        v3348 = v3212;
                        v9628 = v9655;
                        v9629 = v9656;
                        v9630 = v9657;
                        v9631 = v13426;
                        v9632 = v13585;
                        v9633 = v9634;
                    }
                    let v3321 = if v3319 > v0 { 1.0 } else { 0.0 };
                    if v3321 != 0.0 {
                    } else {
                    }
                    let v3322 = if v3207 == v0 { 1.0 } else { 0.0 };
                    let v3323: f64;
                    let v5718: f64;
                    let v9663: Lanes<6>;
                    let v9664: Lanes<6>;
                    if v3322 != 0.0 {
                        v3323 = v3002;
                        v5718 = v3009;
                        v9663 = v9626;
                        v9664 = v9627;
                    } else {
                        v3323 = v3051;
                        v5718 = v3074;
                        v9663 = v9629;
                        v9664 = v9630;
                    }
                    let v4322: f64;
                    if v2997 != 0.0 {
                        v4322 = v4;
                    } else {
                        v4322 = v0;
                    }
                    let v3324 = v3323 - v2572;
                    let v13243 = v9663 - v9562;
                    let v3329 = v3325 / v118;
                    let v3331 = v3330 - v2573;
                    let v13244 = v9631 - v9530;
                    let v3332 = v3330 + v2573;
                    let v13245 = v9631 + v9530;
                    let v3333 = v658 * v3332;
                    let v13246 = v10402 * v3332;
                    let v3336 = v3331 - ((v3333 * v3324) * v8);
                    let v13254 = v13244 - (((((Lanes([0.0, 0.0, v13246[0], 0.0, 0.0, 0.0])) + (v13245 * v658)) * v3324) + (v13243 * v3333)) * v8);
                    let v3339 = if (if v3336 < v0 { 1.0 } else { 0.0 }) != 0.0 || (if v818 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v4378: f64;
                    let v9665: Lanes<6>;
                    if v3339 != 0.0 {
                        v4378 = v0;
                        v9665 = v11054;
                    } else {
                        v4378 = v3336;
                        v9665 = v13254;
                    }
                    let v3343 = v3340 * (v3341 + v2587);
                    let v13256 = (v9632 + v9532) * v3340;
                    let v3344 = v3324 + v856;
                    let v3357 = v1223 * v1226;
                    let v3359 = if v3357 >= v0 { 1.0 } else { 0.0 };
                    let v3360 = if (if (-(((v3348 * v3348) - (v2594 * v2594)) / (v125 / ((v125 * v3329) + v4)))) < v3357 { 1.0 } else { 0.0 }) != 0.0 && v3359 != 0.0 { 1.0 } else { 0.0 };
                    if v3360 != 0.0 {
                        if v3361 != 0.0 {
                            let v3369: f64;
                            if v3362 != 0.0 {
                                v3369 = v4;
                            } else {
                                let v3370: f64;
                                if v3363 != 0.0 {
                                    v3370 = v73;
                                } else {
                                    let v3371: f64;
                                    if v3364 != 0.0 {
                                        v3371 = v91;
                                    } else {
                                        let v3372: f64;
                                        if v3365 != 0.0 {
                                            v3372 = v85;
                                        } else {
                                            v3372 = v0;
                                        }
                                        v3371 = v3372;
                                    }
                                    v3370 = v3371;
                                }
                                v3369 = v3370;
                            }
                            let mut v3366: f64 = 0.0;
                            v3366 = v0;
                            loop {
                                let v3367 = if v3366 < v3369 { 1.0 } else { 0.0 };
                                if v3367 == 0.0 {
                                    break;
                                }
                                let v3368 = v3366 + v4;
                                v3366 = v3368;
                            }
                        } else {
                        }
                    } else {
                    }
                    let v3375 = if ((v658 * v2599) - v4) > v0 { 1.0 } else { 0.0 };
                    if v3375 != 0.0 {
                    } else {
                    }
                    let v3376 = -v3331;
                    let v13257 = v13244 * v10382;
                    let v3378 = if (if v3376 < v3357 { 1.0 } else { 0.0 }) != 0.0 && v3359 != 0.0 { 1.0 } else { 0.0 };
                    let v3406: f64;
                    let v9666: Lanes<6>;
                    if v3378 != 0.0 {
                        let v3379 = v3357 - v3376;
                        let v13258 = v13257 * v10382;
                        let v3380 = v3379 * v3379;
                        let v13259 = v13258 * v3379;
                        let v3381 = v3357 * v3357;
                        let v13261 = (v13259 + v13259) * v3380;
                        let v13262 = v13261 + v13261;
                        let v3384 = (v3380 * v3380) + (v3381 * v3381);
                        let v3401: f64;
                        let v9667: Lanes<6>;
                        if v3385 != 0.0 {
                            let v3395: f64;
                            if v3386 != 0.0 {
                                v3395 = v4;
                            } else {
                                let v3396: f64;
                                if v3387 != 0.0 {
                                    v3396 = v73;
                                } else {
                                    let v3397: f64;
                                    if v3388 != 0.0 {
                                        v3397 = v91;
                                    } else {
                                        let v3398: f64;
                                        if v3389 != 0.0 {
                                            v3398 = v85;
                                        } else {
                                            v3398 = v0;
                                        }
                                        v3397 = v3398;
                                    }
                                    v3396 = v3397;
                                }
                                v3395 = v3396;
                            }
                            let mut v3390: f64 = 0.0;
                            let mut v3392: f64 = 0.0;
                            let mut v9668: Lanes<6> = Lanes([0.0; 6]);
                            v3390 = v0;
                            v3392 = v3384;
                            v9668 = v13262;
                            loop {
                                let v3391 = if v3390 < v3395 { 1.0 } else { 0.0 };
                                if v3391 == 0.0 {
                                    break;
                                }
                                let v3393 = v3392.sqrt();
                                let v13325 = v9668 * (v9360 / (v10427 * v3393));
                                let v3394 = v3390 + v4;
                                v3390 = v3394;
                                v3392 = v3393;
                                v9668 = v13325;
                            }
                            v3401 = v3392;
                            v9667 = v9668;
                        } else {
                            let v3400 = v3384.powf(v3399);
                            let v13266 = v13262 * (v3399 * (v3384.powf(v13263)));
                            v3401 = v3400;
                            v9667 = v13266;
                        }
                        let v3402 = v4 / v3401;
                        let v3403 = v3379 * v3357;
                        let v3405 = v3357 - (v3403 * v3402);
                        let v13274 = (((v13258 * v3357) * v3402) + ((((v9667 * v3402) * v10382) / v3401) * v3403)) * v10382;
                        v3406 = v3405;
                        v9666 = v13274;
                    } else {
                        v3406 = v3376;
                        v9666 = v13257;
                    }
                    let v3409 = v658 * v1123;
                    let v13277 = v10402 * v1123;
                    let v13278 = v9413 * v658;
                    let v3410 = v3409 * v3344;
                    let v13282 = ((Lanes([0.0, 0.0, v13277[0], 0.0, 0.0])) + (Lanes([v13278[0], v13278[1], 0.0, v13278[2], v13278[3]]))) * v3344;
                    let v3411 = v3410 * v3344;
                    let v3412 = (v73 * (-v3406)) / v3411;
                    let v3413 = v4 + v3412;
                    let v3415 = (v3413 * v3344) / v2577;
                    let v3416 = v4 - v3415;
                    let v13298 = ((((((((v9666 * v10382) * v73) - (((((Lanes([v13282[0], v13282[1], v13282[2], v13282[3], v13282[4], 0.0])) + (v13243 * v3409)) * v3344) + (v13243 * v3410)) * v3412)) / v3411) * v3344) + (v13243 * v3413)) - (v12798 * v3415)) / v2577) * v10382;
                    let v3420 = if (if v3416 < v3417 { 1.0 } else { 0.0 }) != 0.0 && v3419 != 0.0 { 1.0 } else { 0.0 };
                    let v3449: f64;
                    let v9669: Lanes<6>;
                    if v3420 != 0.0 {
                        let v3422 = v3421 - v3416;
                        let v13299 = v13298 * v10382;
                        let v3423 = v3422 * v3422;
                        let v13300 = v13299 * v3422;
                        let v13302 = (v13300 + v13300) * v3423;
                        let v13303 = v13302 + v13302;
                        let v3426 = (v3423 * v3423) + v3425;
                        let v3443: f64;
                        let v9670: Lanes<6>;
                        if v3427 != 0.0 {
                            let v3437: f64;
                            if v3428 != 0.0 {
                                v3437 = v4;
                            } else {
                                let v3438: f64;
                                if v3429 != 0.0 {
                                    v3438 = v73;
                                } else {
                                    let v3439: f64;
                                    if v3430 != 0.0 {
                                        v3439 = v91;
                                    } else {
                                        let v3440: f64;
                                        if v3431 != 0.0 {
                                            v3440 = v85;
                                        } else {
                                            v3440 = v0;
                                        }
                                        v3439 = v3440;
                                    }
                                    v3438 = v3439;
                                }
                                v3437 = v3438;
                            }
                            let mut v3432: f64 = 0.0;
                            let mut v3434: f64 = 0.0;
                            let mut v9671: Lanes<6> = Lanes([0.0; 6]);
                            v3432 = v0;
                            v3434 = v3426;
                            v9671 = v13303;
                            loop {
                                let v3433 = if v3432 < v3437 { 1.0 } else { 0.0 };
                                if v3433 == 0.0 {
                                    break;
                                }
                                let v3435 = v3434.sqrt();
                                let v13322 = v9671 * (v9360 / (v10427 * v3435));
                                let v3436 = v3432 + v4;
                                v3432 = v3436;
                                v3434 = v3435;
                                v9671 = v13322;
                            }
                            v3443 = v3434;
                            v9670 = v9671;
                        } else {
                            let v3442 = v3426.powf(v3441);
                            let v13307 = v13303 * (v3441 * (v3426.powf(v13304)));
                            v3443 = v3442;
                            v9670 = v13307;
                        }
                        let v3444 = v4 / v3443;
                        let v3445 = v3422 * v1226;
                        let v3448 = v3447 - (v3445 * v3444);
                        let v13315 = (((v13299 * v1226) * v3444) + ((((v9670 * v3444) * v10382) / v3443) * v3445)) * v10382;
                        v3449 = v3448;
                        v9669 = v13315;
                    } else {
                        v3449 = v3416;
                        v9669 = v13298;
                    }
                    let v3450 = v4 + v3449;
                    let v13318 = (v9669 * v3450) + (v9669 * v3449);
                    let v3452 = v4 + (v3449 * v3450);
                    let v3454 = if v3450 >= v3453 { 1.0 } else { 0.0 };
                    let v3456: f64;
                    let v9672: Lanes<6>;
                    if v3454 != 0.0 {
                        v3456 = v3450;
                        v9672 = v9669;
                    } else {
                        v3456 = v3455;
                        v9672 = v11054;
                    }
                    let v3458 = v3457 * v3332;
                    let v13319 = v13245 * v3457;
                    v3461 = v3462;
                    v3470 = v3207;
                    v4303 = v3449;
                    v4307 = v3456;
                    v4310 = v3452;
                    v4321 = v4322;
                    v4332 = v3323;
                    v4377 = v4378;
                    v4417 = v3343;
                    v4424 = v3458;
                    v4435 = v3348;
                    v4441 = v3324;
                    v4839 = v2577;
                    v5717 = v5718;
                    v8300 = v0;
                    v8477 = v0;
                    v8482 = v0;
                    v8487 = v0;
                    v8493 = v0;
                    v9566 = v9669;
                    v9567 = v9672;
                    v9568 = v13318;
                    v9569 = v9663;
                    v9570 = v9665;
                    v9571 = v13256;
                    v9572 = v13319;
                    v9573 = v9633;
                    v9574 = v13243;
                    v9575 = v12798;
                    v9576 = v9664;
                    v9577 = v11054;
                    v9578 = v11054;
                    v9579 = v11054;
                    v9580 = v11054;
                    v9581 = v11054;
                }
                let v3459 = if v65 >= v4 { 1.0 } else { 0.0 };
                if v3459 != 0.0 {
                    let v3466 = if (if v2580 == v4 { 1.0 } else { 0.0 }) != 0.0 && (if v3461 == v73 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v3466 != 0.0 {
                    } else {
                    }
                    let v3469 = if (if v2580 == v73 { 1.0 } else { 0.0 }) != 0.0 && (if v3461 == v4 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v3469 != 0.0 {
                    } else {
                    }
                } else {
                }
                if v2571 != 0.0 {
                } else {
                }
                let v3471 = if v3470 == v0 { 1.0 } else { 0.0 };
                if v3471 != 0.0 {
                } else {
                }
                let v3473 = if (v2450 + v3470) < v4 { 1.0 } else { 0.0 };
                if v3473 != 0.0 {
                } else {
                }
                v4300 = v0;
                v4302 = v4303;
                v4306 = v4307;
                v4309 = v4310;
                v4320 = v4321;
                v4331 = v4332;
                v4335 = v2572;
                v4343 = v2576;
                v4376 = v4377;
                v4416 = v4417;
                v4423 = v4424;
                v4433 = v2594;
                v4434 = v4435;
                v4440 = v4441;
                v4632 = v2598;
                v4730 = v4731;
                v4782 = v4783;
                v4838 = v4839;
                v4959 = v1569;
                v4968 = v1239;
                v4972 = v1499;
                v5088 = v5089;
                v5496 = v2564;
                v5638 = v5639;
                v5716 = v5717;
                v5776 = v5777;
                v8299 = v8300;
                v8476 = v8477;
                v8481 = v8482;
                v8486 = v8487;
                v8492 = v8493;
                v8559 = v0;
                v8571 = v0;
                v9206 = v9207;
                v9429 = v9566;
                v9430 = v9567;
                v9431 = v9568;
                v9432 = v9569;
                v9433 = v9562;
                v9434 = v9565;
                v9435 = v9570;
                v9436 = v9571;
                v9437 = v9572;
                v9438 = v9533;
                v9439 = v9573;
                v9440 = v9574;
                v9441 = v9563;
                v9442 = v9497;
                v9443 = v9498;
                v9444 = v9575;
                v9445 = v9462;
                v9446 = v9461;
                v9447 = v12070;
                v9448 = v9475;
                v9449 = v9495;
                v9450 = v9499;
                v9451 = v9576;
                v9452 = v9577;
                v9453 = v9578;
                v9454 = v9579;
                v9455 = v9580;
                v9456 = v9581;
                v9457 = v11054;
                v9458 = v11054;
                v9459 = v9500;
            } else {
                let v3474 = if v764 < v7 { 1.0 } else { 0.0 };
                let v4184: f64;
                if v3474 != 0.0 {
                    v4184 = v4;
                } else {
                    v4184 = v73;
                }
                let v10829 = Lanes([v9405[0], v9405[1], 0.0, 0.0, v9405[2]]);
                let v3476 = if v825 < (v1200 + v830) { 1.0 } else { 0.0 };
                let v3631: f64;
                let v3829: f64;
                let v3938: f64;
                let v5090: f64;
                let v9673: Lanes<5>;
                let v9674: Lanes<5>;
                let v9675: Lanes<5>;
                if v3476 != 0.0 {
                    let v3478 = v73 * v660;
                    let v3480 = (-v363) / v1201;
                    let v3481 = v3480.ln();
                    let v3482 = v3478 * v3481;
                    let v10945 = (v10407 * v73) * v3481;
                    let v10948 = (Lanes([0.0, 0.0, v10945[0], 0.0, 0.0])) + (((((v10826 * v3480) * v10382) / v1201) * (v9360 / v3480)) * v3478);
                    let v3483 = v1195 - v830;
                    let v10950 = v10402 * v3483;
                    let v3485 = v658 * v745;
                    let v3486 = v4 / v3485;
                    let v3487 = v3486 * v1123;
                    let v10960 = (((((v10402 * v745) + (v10477 * v658)) * v3486) * v10382) / v3485) * v1123;
                    let v10961 = v9413 * v3486;
                    let v10964 = (Lanes([0.0, 0.0, v10960[0], 0.0, 0.0])) + (Lanes([v10961[0], v10961[1], 0.0, v10961[2], v10961[3]]));
                    let v10965 = v10964 * v3488;
                    let v3490 = v73 + (v3488 * v3487);
                    let v3491 = v86 * v3490;
                    let v3492 = v3491 * v3490;
                    let v3493 = v3492 * v3490;
                    let v10972 = ((((v10965 * v86) * v3490) + (v10965 * v3491)) * v3490) + (v10965 * v3492);
                    let v3494 = (v658 * v3483) - v73;
                    let v3496 = v3495 * v3487;
                    let v3497 = v3496 * v3494;
                    let v10976 = ((v10964 * v3495) * v3494) + (((Lanes([0.0, 0.0, v10950[0], 0.0, 0.0])) + ((v10820 - v10829) * v658)) * v3496);
                    let v3499 = v3498 - v3497;
                    let v10977 = v10976 * v10382;
                    let v3500 = v3499 * v3499;
                    let v10978 = v10977 * v3499;
                    let v10979 = v10978 + v10978;
                    let v3503 = if v3493 < (v3500 * v3501) { 1.0 } else { 0.0 };
                    let v3515: f64;
                    let v9676: Lanes<5>;
                    if v3503 != 0.0 {
                        let v3507 = (v8 * v3493) / v3499;
                        let v3509 = ((v3504 + v3499) + v3507) + v3497;
                        let v10990 = (v10977 + (((v10972 * v8) - (v10977 * v3507)) / v3499)) + v10976;
                        v3515 = v3509;
                        v9676 = v10990;
                    } else {
                        let v3511 = (v3493 + v3500).sqrt();
                        let v3514 = (v3512 + v3511) + v3497;
                        let v10984 = ((v10972 + v10979) * (v9360 / (v10427 * v3511))) + v10976;
                        v3515 = v3514;
                        v9676 = v10984;
                    }
                    let v3516 = v3515.powf(v1557);
                    let v10994 = v9676 * (v1557 * (v3515.powf(v10991)));
                    let v3523 = v743 * v3516;
                    let v3525 = ((v3517 - (v3518 * v3487)) + (v73 * v3516)) + (v3523 * v3516);
                    let v3526 = v4 / v3516;
                    let v3527 = v3525 * v3526;
                    let v11011 = v10407 * v3527;
                    let v3530 = ((v3527 * v660) + v830) - v830;
                    let v11015 = (((((((((v10964 * v3518) * v10382) + (v10994 * v73)) + (((v10994 * v743) * v3516) + (v10994 * v3523))) * v3526) + ((((v10994 * v3526) * v10382) / v3516) * v3525)) * v660) + (Lanes([0.0, 0.0, v11011[0], 0.0, 0.0]))) + v10829) - v10829;
                    let v3531 = v3530 / v3482;
                    let v11019 = ((v11015 - (v10948 * v3531)) / v3482) * v3531;
                    let v3534 = (v4 + (v3531 * v3531)).sqrt();
                    let v3535 = v3530 / v3534;
                    let v3536 = v3535 + v830;
                    let v11027 = ((v11015 - (((v11019 + v11019) * (v9360 / (v10427 * v3534))) * v3535)) / v3534) + v10829;
                    v3631 = v3536;
                    v3829 = v3477;
                    v3938 = v0;
                    v5090 = v0;
                    v9673 = v11027;
                    v9674 = v10571;
                    v9675 = v10571;
                } else {
                    let v3618: f64;
                    let v3620: f64;
                    let v9677: Lanes<5>;
                    let v9678: Lanes<5>;
                    if v3537 != 0.0 {
                        v3618 = v0;
                        v3620 = v0;
                        v9677 = v10571;
                        v9678 = v10571;
                    } else {
                        let v3538 = v1195 - v830;
                        let v3539 = v658 * v3538;
                        let v10831 = v10402 * v3538;
                        let v10834 = (Lanes([0.0, 0.0, v10831[0], 0.0, 0.0])) + ((v10820 - v10829) * v658);
                        let v3542 = v1202 * v659;
                        let v10837 = v10404 * v1202;
                        let v3543 = (v85 * (v3539 - v4)) / v3542;
                        let v10842 = ((v10834 * v85) - (((v10828 * v659) + (Lanes([0.0, 0.0, v10837[0], 0.0, 0.0]))) * v3543)) / v3542;
                        let v3544 = v4 + v3543;
                        let v3546 = if v3544 >= v3545 { 1.0 } else { 0.0 };
                        let v3548: f64;
                        let v9679: Lanes<5>;
                        if v3546 != 0.0 {
                            v3548 = v3544;
                            v9679 = v10842;
                        } else {
                            v3548 = v3547;
                            v9679 = v10571;
                        }
                        let v10844 = v10402 * v1202;
                        let v3550 = (v1202 * v658) * v8;
                        let v3551 = v3548.sqrt();
                        let v3552 = v4 - v3551;
                        let v3554 = v1195 + (v3550 * v3552);
                        let v10855 = v10820 + (((((v10828 * v658) + (Lanes([0.0, 0.0, v10844[0], 0.0, 0.0]))) * v8) * v3552) + (((v9679 * (v9360 / (v10427 * v3551))) * v10382) * v3550));
                        let v3557 = if (v658 * (v3554 - v830)) < v91 { 1.0 } else { 0.0 };
                        let v3615: f64;
                        let v3621: f64;
                        let v9680: Lanes<5>;
                        let v9681: Lanes<5>;
                        if v3557 != 0.0 {
                            let v3559 = v3558 * v658;
                            let v3560 = v3559 * v1201;
                            let v10893 = (v10402 * v3558) * v1201;
                            let v3561 = v4 / v3560;
                            let v10899 = ((((Lanes([0.0, 0.0, v10893[0], 0.0, 0.0])) + (v10826 * v3559)) * v3561) * v10382) / v3560;
                            let v10900 = v10899 * v91;
                            let v3563 = v1535 + (v91 * v3561);
                            let v3567 = v1148 * v3561;
                            let v3568 = v3567 * v3539;
                            let v10907 = ((v10899 * v1535) * v10382) + (((v10899 * v1148) * v3539) + (v10834 * v3567));
                            let v3573 = (v1544 - (v1535 * (v1545 + v3561))) + v3568;
                            let v10908 = v10907 * v3573;
                            let v3575 = v85 * v3563;
                            let v3576 = v3575 * v3563;
                            let v3579 = ((v3576 * v3563) + (v3573 * v3573)).sqrt();
                            let v3580 = ((v3564 - (v1535 * v3561)) + v3568) + v3579;
                            let v3581 = v3580.powf(v1557);
                            let v10925 = (v10907 + (((((((v10900 * v85) * v3563) + (v10900 * v3575)) * v3563) + (v10900 * v3576)) + (v10908 + v10908)) * (v9360 / (v10427 * v3579)))) * (v1557 * (v3580.powf(v10922)));
                            let v3583 = v91 * v3581;
                            let v3584 = (v1559 * v3563) / v3583;
                            let v3588 = (v91 - v3584) + (v3586 * v3581);
                            let v10935 = v10407 * v3588;
                            let v3590 = (v3588 * v660) + v830;
                            let v10938 = (((((((v10900 * v1559) - ((v10925 * v91) * v3584)) / v3583) * v10382) + (v10925 * v3586)) * v660) + (Lanes([0.0, 0.0, v10935[0], 0.0, 0.0]))) + v10829;
                            v3615 = v3590;
                            v3621 = v3590;
                            v9680 = v10938;
                            v9681 = v10938;
                        } else {
                            let v3591 = if v825 <= v1138 { 1.0 } else { 0.0 };
                            let v3616: f64;
                            let v9682: Lanes<5>;
                            if v3591 != 0.0 {
                                v3616 = v3554;
                                v9682 = v10855;
                            } else {
                                let v3592 = v4 / v754;
                                let v10858 = ((v10488 * v3592) * v10382) / v754;
                                let v3593 = v3592 / v1206;
                                let v3594 = v3593 * v1195;
                                let v3595 = v3594 * v1195;
                                let v3596 = v73 / v1195;
                                let v3597 = v658 + v3596;
                                let v3599 = (v3595.ln()) / v3597;
                                let v10878 = (((((((((Lanes([0.0, 0.0, v10858[0], 0.0, 0.0])) - (v9414 * v3593)) / v1206) * v1195) + (v10820 * v3593)) * v1195) + (v10820 * v3594)) * (v9360 / v3595)) - (((Lanes([0.0, 0.0, v10402[0], 0.0, 0.0])) + (((v10820 * v3596) * v10382) / v1195)) * v3599)) / v3597;
                                let v10879 = v10878 - v10855;
                                let v3601 = (v3599 - v3554) - v1265;
                                let v3603 = (v85 * v3599) * v1265;
                                let v10881 = (v10878 * v85) * v1265;
                                let v3604 = if v3603 > v0 { 1.0 } else { 0.0 };
                                let v3606: f64;
                                let v9683: Lanes<5>;
                                if v3604 != 0.0 {
                                    v3606 = v3603;
                                    v9683 = v10881;
                                } else {
                                    let v3605 = -v3603;
                                    let v10882 = v10881 * v10382;
                                    v3606 = v3605;
                                    v9683 = v10882;
                                }
                                let v10883 = v10879 * v3601;
                                let v3609 = ((v3601 * v3601) + v3606).sqrt();
                                let v3612 = v3599 - (v8 * (v3601 + v3609));
                                let v10891 = v10878 - ((v10879 + (((v10883 + v10883) + v9683) * (v9360 / (v10427 * v3609)))) * v8);
                                v3616 = v3612;
                                v9682 = v10891;
                            }
                            v3615 = v3616;
                            v3621 = v3554;
                            v9680 = v9682;
                            v9681 = v10855;
                        }
                        let v3614 = v830 + v3613;
                        let v3617 = if v3615 < v3614 { 1.0 } else { 0.0 };
                        let v3619: f64;
                        let v9684: Lanes<5>;
                        if v3617 != 0.0 {
                            v3619 = v3614;
                            v9684 = v10829;
                        } else {
                            v3619 = v3615;
                            v9684 = v9680;
                        }
                        v3618 = v3619;
                        v3620 = v3621;
                        v9677 = v9684;
                        v9678 = v9681;
                    }
                    v3631 = v3618;
                    v3829 = v0;
                    v3938 = v3620;
                    v5090 = v3618;
                    v9673 = v9677;
                    v9674 = v9678;
                    v9675 = v9677;
                }
                let v3624 = if (if v1881 == v4 { 1.0 } else { 0.0 }) != 0.0 && (if v2199 == v73 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v3627: f64;
                let v9685: Lanes<1>;
                if v3624 != 0.0 {
                    let v3626 = v3625 * v2249;
                    let v11029 = v9372 * v3625;
                    v3627 = v3626;
                    v9685 = v11029;
                } else {
                    v3627 = v0;
                    v9685 = v11028;
                }
                let v11030 = v10402 * v830;
                let v11031 = v9405 * v658;
                let v3629 = (v658 * v830).exp();
                let v11035 = ((Lanes([0.0, 0.0, v11030[0], 0.0])) + (Lanes([v11031[0], v11031[1], 0.0, v11031[2]]))) * v3629;
                let v3630 = v754 * v3629;
                let v11036 = v10488 * v3629;
                let v11039 = (Lanes([0.0, 0.0, v11036[0], 0.0])) + (v11035 * v754);
                let v3635 = (((v486 * v7) * v7) / v73) / v118;
                let v3638 = ((v73 * v658) * v3635).sqrt();
                let v11044 = ((v10402 * v73) * v3635) * (v9360 / (v10427 * v3638));
                let v3639 = v3638.exp();
                let v3641 = (-v3638).exp();
                let v3643 = (v3639 + v3641) / v73;
                let v3645 = (v3643.ln()) / v3635;
                let v11052 = ((((v11044 * v3639) + ((v11044 * v10382) * v3641)) / v73) * (v9360 / v3643)) / v3635;
                let v11053 = Lanes([v9673[0], v9673[1], v9673[2], v9673[3], v9673[4], 0.0]);
                let mut v3646: f64 = 0.0;
                let mut v3649: f64 = 0.0;
                let mut v3739: f64 = 0.0;
                let mut v3745: f64 = 0.0;
                let mut v3830: f64 = 0.0;
                let mut v3837: f64 = 0.0;
                let mut v3840: f64 = 0.0;
                let mut v4183: f64 = 0.0;
                let mut v9686: Lanes<6> = Lanes([0.0; 6]);
                let mut v9687: Lanes<6> = Lanes([0.0; 6]);
                let mut v9688: Lanes<6> = Lanes([0.0; 6]);
                let mut v9689: Lanes<6> = Lanes([0.0; 6]);
                v3646 = v4;
                v3649 = v3631;
                v3739 = v0;
                v3745 = v3829;
                v3830 = v0;
                v3837 = v0;
                v3840 = v0;
                v4183 = v4184;
                v9686 = v11053;
                v9687 = v11054;
                v9688 = v11054;
                v9689 = v11054;
                loop {
                    let v3648 = if v3646 <= v3647 { 1.0 } else { 0.0 };
                    if v3648 == 0.0 {
                        break;
                    }
                    let v3650 = v3649 - v830;
                    let v11680 = v9686 - (Lanes([v9405[0], v9405[1], 0.0, 0.0, v9405[2], 0.0]));
                    let v3651 = v658 * v3650;
                    let v11681 = v10402 * v3650;
                    let v11684 = (Lanes([0.0, 0.0, v11681[0], 0.0, 0.0, 0.0])) + (v11680 * v658);
                    let v3652 = v3650 - v3635;
                    let v3653 = v3645 * v3652;
                    let v11685 = v11052 * v3652;
                    let v11688 = (Lanes([0.0, 0.0, v11685[0], 0.0, 0.0, 0.0])) + (v11680 * v3645);
                    let v3654 = if v3653 < v2530 { 1.0 } else { 0.0 };
                    let v3664: f64;
                    let v3669: f64;
                    let v9690: Lanes<6>;
                    let v9691: Lanes<6>;
                    if v3654 != 0.0 {
                        let v3655 = v3653.exp();
                        let v11689 = v11688 * v3655;
                        let v3658 = ((-v3645) * v3635).exp();
                        let v11692 = ((v11052 * v10382) * v3635) * v3658;
                        let v11694 = v11689 - (Lanes([0.0, 0.0, v11692[0], 0.0, 0.0, 0.0]));
                        let v3660 = v4 + (v3655 - v3658);
                        let v3662 = (v3660.ln()) / v3645;
                        let v11697 = v11052 * v3662;
                        let v11700 = ((v11694 * (v9360 / v3660)) - (Lanes([0.0, 0.0, v11697[0], 0.0, 0.0, 0.0]))) / v3645;
                        let v3663 = v3655 / v3660;
                        let v11703 = (v11689 - (v11694 * v3663)) / v3660;
                        v3664 = v3662;
                        v3669 = v3663;
                        v9690 = v11700;
                        v9691 = v11703;
                    } else {
                        v3664 = v3652;
                        v3669 = v4;
                        v9690 = v11680;
                        v9691 = v11054;
                    }
                    let v3665 = v658 * v3664;
                    let v11704 = v10402 * v3664;
                    let v11707 = (Lanes([0.0, 0.0, v11704[0], 0.0, 0.0, 0.0])) + (v9690 * v658);
                    let v3666 = v3651.abs();
                    let v3668 = if v3666 < v3667 { 1.0 } else { 0.0 };
                    let v3748: f64;
                    let v3758: f64;
                    let v9692: Lanes<6>;
                    let v9693: Lanes<6>;
                    if v3668 != 0.0 {
                        let v11810 = v9691 * v3669;
                        let v3673 = ((v4 - (v3669 * v3669)) / v73).sqrt();
                        let v11816 = (((v11810 + v11810) * v10382) / v73) * (v9360 / (v10427 * v3673));
                        let v3674 = v3651 * v3673;
                        let v11819 = (v11684 * v3673) + (v11816 * v3651);
                        let v3675 = v658 * v3673;
                        let v11820 = v10402 * v3673;
                        let v11823 = (Lanes([0.0, 0.0, v11820[0], 0.0, 0.0, 0.0])) + (v11816 * v658);
                        let v3676 = if v3651 < v0 { 1.0 } else { 0.0 };
                        let v3749: f64;
                        let v3759: f64;
                        let v9694: Lanes<6>;
                        let v9695: Lanes<6>;
                        if v3676 != 0.0 {
                            let v3677 = -v3674;
                            let v11824 = v11819 * v10382;
                            let v3678 = -v3675;
                            let v11825 = v11823 * v10382;
                            v3749 = v3677;
                            v3759 = v3678;
                            v9694 = v11824;
                            v9695 = v11825;
                        } else {
                            v3749 = v3674;
                            v3759 = v3675;
                            v9694 = v11819;
                            v9695 = v11823;
                        }
                        v3748 = v3749;
                        v3758 = v3759;
                        v9692 = v9694;
                        v9693 = v9695;
                    } else {
                        let v3680 = if v3666 < v3679 { 1.0 } else { 0.0 };
                        let v3750: f64;
                        let v3760: f64;
                        let v9696: Lanes<6>;
                        let v9697: Lanes<6>;
                        if v3680 != 0.0 {
                            let v11732 = v11684 * v3651;
                            let v3682 = (v3651 * v3651) / v73;
                            let v3683 = v3651 / v91;
                            let v11735 = v11684 / v91;
                            let v3684 = v3651 / v85;
                            let v11736 = v11684 / v85;
                            let v3686 = v4 - (v3651 / v639);
                            let v3688 = v4 - (v3684 * v3686);
                            let v3690 = v4 - (v3683 * v3688);
                            let v3692 = v3651 / v73;
                            let v3693 = v4 - v3684;
                            let v3695 = v4 - (v3683 * v3693);
                            let v3697 = v4 - (v3692 * v3695);
                            let v11763 = v11707 * v3665;
                            let v3700 = (v3665 * v3665) / v73;
                            let v3701 = v3665 / v91;
                            let v11766 = v11707 / v91;
                            let v3702 = v3665 / v85;
                            let v11767 = v11707 / v85;
                            let v3704 = v4 - (v3665 / v639);
                            let v3706 = v4 - (v3702 * v3704);
                            let v3708 = v4 - (v3701 * v3706);
                            let v3710 = v3665 / v73;
                            let v3711 = v4 - v3702;
                            let v3713 = v4 - (v3701 * v3711);
                            let v3715 = v4 - (v3710 * v3713);
                            let v3716 = v3665 * v3715;
                            let v3718 = ((v3682 * v3690) - (v3700 * v3708)).sqrt();
                            let v11797 = (((((v11732 + v11732) / v73) * v3690) + ((((v11735 * v3688) + ((((v11736 * v3686) + (((v11684 / v639) * v10382) * v3684)) * v10382) * v3683)) * v10382) * v3682)) - ((((v11763 + v11763) / v73) * v3708) + ((((v11766 * v3706) + ((((v11767 * v3704) + (((v11707 / v639) * v10382) * v3702)) * v10382) * v3701)) * v10382) * v3700))) * (v9360 / (v10427 * v3718));
                            let v3719 = v658 * v8;
                            let v3721 = (v3651 * v3697) - (v3669 * v3716);
                            let v11803 = (v10402 * v8) * v3721;
                            let v3723 = (v3719 * v3721) / v3718;
                            let v11809 = (((Lanes([0.0, 0.0, v11803[0], 0.0, 0.0, 0.0])) + ((((v11684 * v3697) + (((((v11684 / v73) * v3695) + ((((v11735 * v3693) + ((v11736 * v10382) * v3683)) * v10382) * v3692)) * v10382) * v3651)) - ((v9691 * v3716) + (((v11707 * v3715) + (((((v11707 / v73) * v3713) + ((((v11766 * v3711) + ((v11767 * v10382) * v3701)) * v10382) * v3710)) * v10382) * v3665)) * v3669))) * v3719)) - (v11797 * v3723)) / v3718;
                            v3750 = v3718;
                            v3760 = v3723;
                            v9696 = v11797;
                            v9697 = v11809;
                        } else {
                            let v3725 = (-v3651).exp();
                            let v11709 = (v11684 * v10382) * v3725;
                            let v3727 = (-v3665).exp();
                            let v11711 = (v11707 * v10382) * v3727;
                            let v3731 = ((v3651 - v3665) + (v3725 - v3727)).sqrt();
                            let v11717 = ((v11684 - v11707) + (v11709 - v11711)) * (v9360 / (v10427 * v3731));
                            let v3732 = v658 * v8;
                            let v3734 = v4 - v3727;
                            let v3736 = (v4 - v3725) - (v3669 * v3734);
                            let v11725 = (v10402 * v8) * v3736;
                            let v3738 = (v3732 * v3736) / v3731;
                            let v11731 = (((Lanes([0.0, 0.0, v11725[0], 0.0, 0.0, 0.0])) + (((v11709 * v10382) - ((v9691 * v3734) + ((v11711 * v10382) * v3669))) * v3732)) - (v11717 * v3738)) / v3731;
                            v3750 = v3731;
                            v3760 = v3738;
                            v9696 = v11717;
                            v9697 = v11731;
                        }
                        v3748 = v3750;
                        v3758 = v3760;
                        v9692 = v9696;
                        v9693 = v9697;
                    }
                    let v3740 = if v3739 == v4 { 1.0 } else { 0.0 };
                    let v3741 = if v3651 < v0 { 1.0 } else { 0.0 };
                    let v3742 = if v3740 != 0.0 && v3741 != 0.0 { 1.0 } else { 0.0 };
                    let v3744: f64;
                    if v3742 != 0.0 {
                        v3744 = v3743;
                    } else {
                        v3744 = v3745;
                    }
                    let v3747 = if v3744 == v3746 { 1.0 } else { 0.0 };
                    let v3752: f64;
                    let v9698: Lanes<6>;
                    if v3747 != 0.0 {
                        v3752 = v0;
                        v9698 = v11054;
                    } else {
                        let v3751 = v757 * v3748;
                        let v11826 = v10495 * v3748;
                        let v11829 = (Lanes([0.0, 0.0, v11826[0], 0.0, 0.0, 0.0])) + (v9692 * v757);
                        v3752 = v3751;
                        v9698 = v11829;
                    }
                    let v3755 = if v3752 < (v7 * v3753) { 1.0 } else { 0.0 };
                    let v4185: f64;
                    if v3755 != 0.0 {
                        v4185 = v4;
                    } else {
                        v4185 = v73;
                    }
                    let v3756 = v486 * v3752;
                    let v11830 = v9698 * v486;
                    let v3792: f64;
                    let v3798: f64;
                    let v3841: f64;
                    let v9699: Lanes<6>;
                    let v9700: Lanes<6>;
                    let v9701: Lanes<6>;
                    if v3741 != 0.0 {
                        let v3757 = -v3748;
                        let v11883 = v9692 * v10382;
                        let v3761 = -v3758;
                        let v11884 = v9693 * v10382;
                        v3792 = v3757;
                        v3798 = v3761;
                        v3841 = v3840;
                        v9699 = v11883;
                        v9700 = v11884;
                        v9701 = v9689;
                    } else {
                        let v3762 = if v3651 < v112 { 1.0 } else { 0.0 };
                        let v3793: f64;
                        let v3799: f64;
                        let v3842: f64;
                        let v9702: Lanes<6>;
                        let v9703: Lanes<6>;
                        let v9704: Lanes<6>;
                        if v3762 != 0.0 {
                            v3793 = v3748;
                            v3799 = v3758;
                            v3842 = v3840;
                            v9702 = v9692;
                            v9703 = v9693;
                            v9704 = v9689;
                        } else {
                            let v3763 = if v3651 < v2530 { 1.0 } else { 0.0 };
                            let v3781: f64;
                            let v3786: f64;
                            let v9705: Lanes<6>;
                            let v9706: Lanes<6>;
                            if v3763 != 0.0 {
                                let v3764 = v3651.exp();
                                let v11854 = v11684 * v3764;
                                let v3766 = v3764 - (v3651 + v4);
                                let v3767 = v3630 * v3766;
                                let v11856 = v11039 * v3766;
                                let v11859 = (Lanes([v11856[0], v11856[1], v11856[2], 0.0, v11856[3], 0.0])) + ((v11854 - v11684) * v3630);
                                let v3768 = v3630 * v658;
                                let v11861 = v10402 * v3630;
                                let v3769 = v3764 - v4;
                                let v3770 = v3768 * v3769;
                                let v11864 = ((v11039 * v658) + (Lanes([0.0, 0.0, v11861[0], 0.0]))) * v3769;
                                let v11867 = (Lanes([v11864[0], v11864[1], v11864[2], 0.0, v11864[3], 0.0])) + (v11854 * v3768);
                                v3781 = v3767;
                                v3786 = v3770;
                                v9705 = v11859;
                                v9706 = v11867;
                            } else {
                                let v11831 = v10402 * v3649;
                                let v3772 = (v658 * v3649).exp();
                                let v11835 = ((Lanes([0.0, 0.0, v11831[0], 0.0, 0.0, 0.0])) + (v9686 * v658)) * v3772;
                                let v3773 = v3651 + v4;
                                let v11836 = v11035 * v3773;
                                let v3775 = v3772 - (v3629 * v3773);
                                let v3776 = v754 * v3775;
                                let v11841 = v10488 * v3775;
                                let v11844 = (Lanes([0.0, 0.0, v11841[0], 0.0, 0.0, 0.0])) + ((v11835 - ((Lanes([v11836[0], v11836[1], v11836[2], 0.0, v11836[3], 0.0])) + (v11684 * v3629))) * v754);
                                let v3777 = v754 * v658;
                                let v3778 = v3772 - v3629;
                                let v3779 = v3777 * v3778;
                                let v11850 = ((v10488 * v658) + (v10402 * v754)) * v3778;
                                let v11853 = (Lanes([0.0, 0.0, v11850[0], 0.0, 0.0, 0.0])) + ((v11835 - (Lanes([v11035[0], v11035[1], v11035[2], 0.0, v11035[3], 0.0]))) * v3777);
                                v3781 = v3776;
                                v3786 = v3779;
                                v9705 = v11844;
                                v9706 = v11853;
                            }
                            let v11868 = v9692 * v3748;
                            let v3783 = ((v3748 * v3748) + v3781).sqrt();
                            let v11873 = ((v11868 + v11868) + v9705) * (v9360 / (v10427 * v3783));
                            let v3784 = v73 * v3758;
                            let v3789 = (v8 * ((v3784 * v3748) + v3786)) / v3783;
                            let v11882 = ((((((v9693 * v73) * v3748) + (v9692 * v3784)) + v9706) * v8) - (v11873 * v3789)) / v3783;
                            v3793 = v3783;
                            v3799 = v3789;
                            v3842 = v3781;
                            v9702 = v11873;
                            v9703 = v11882;
                            v9704 = v9705;
                        }
                        v3792 = v3793;
                        v3798 = v3799;
                        v3841 = v3842;
                        v9699 = v9702;
                        v9700 = v9703;
                        v9701 = v9704;
                    }
                    let v11885 = v10820 * v10382;
                    let v11888 = v10826 * v3792;
                    let v11893 = v9412 * v3627;
                    let v11894 = v9685 * v1043;
                    let v11897 = (Lanes([v11893[0], v11893[1], v11893[2], v11893[3], 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, v11894[0]]));
                    let v3797 = (((-v1195) + v3649) + (v1201 * v3792)) - (v1043 * v3627);
                    let v11899 = (((Lanes([v11885[0], v11885[1], v11885[2], v11885[3], v11885[4], 0.0])) + v9686) + ((Lanes([v11888[0], v11888[1], v11888[2], v11888[3], v11888[4], 0.0])) + (v9699 * v1201))) - (Lanes([v11897[0], v11897[1], 0.0, v11897[2], v11897[3], v11897[4]]));
                    let v11900 = v10826 * v3798;
                    let v11903 = (Lanes([v11900[0], v11900[1], v11900[2], v11900[3], v11900[4], 0.0])) + (v9700 * v1201);
                    let v3801 = v4 + (v1201 * v3798);
                    let v3824: f64;
                    let v3826: f64;
                    let v3827: f64;
                    let v9707: Lanes<6>;
                    if v3740 != 0.0 {
                        v3824 = v3802;
                        v3826 = v3649;
                        v3827 = v3739;
                        v9707 = v9686;
                    } else {
                        let v3804 = (-v3797) / v3801;
                        let v11907 = ((v11899 * v10382) - (v11903 * v3804)) / v3801;
                        let v3806 = v3649.abs();
                        let v11911 = v9686 * ((v10427 * (if v3649 >= v11296 { 1.0 } else { 0.0 })) - v9360);
                        let v3807 = if v4 >= v3806 { 1.0 } else { 0.0 };
                        let v3808: f64;
                        let v9708: Lanes<6>;
                        if v3807 != 0.0 {
                            v3808 = v4;
                            v9708 = v11054;
                        } else {
                            v3808 = v3806;
                            v9708 = v11911;
                        }
                        let v3810 = v3805 * (v4 + v3808);
                        let v11912 = v9708 * v3805;
                        let v3812 = if (v3804.abs()) > v3810 { 1.0 } else { 0.0 };
                        let v3817: f64;
                        let v9709: Lanes<6>;
                        if v3812 != 0.0 {
                            let v3813 = if v3804 >= v0 { 1.0 } else { 0.0 };
                            let v3815: f64;
                            if v3813 != 0.0 {
                                v3815 = v4;
                            } else {
                                v3815 = v3814;
                            }
                            let v3816 = v3810 * v3815;
                            let v11913 = v11912 * v3815;
                            v3817 = v3816;
                            v9709 = v11913;
                        } else {
                            v3817 = v3804;
                            v9709 = v11907;
                        }
                        let v3818 = v3649 + v3817;
                        let v11914 = v9686 + v9709;
                        let v3823 = if (if (v3817.abs()) <= v856 { 1.0 } else { 0.0 }) != 0.0 && (if (v3797.abs()) <= v3501 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v3828: f64;
                        if v3823 != 0.0 {
                            v3828 = v4;
                        } else {
                            v3828 = v3739;
                        }
                        v3824 = v3646;
                        v3826 = v3818;
                        v3827 = v3828;
                        v9707 = v11914;
                    }
                    let v3825 = v3824 + v4;
                    v3646 = v3825;
                    v3649 = v3826;
                    v3739 = v3827;
                    v3745 = v3744;
                    v3830 = v3756;
                    v3837 = v3792;
                    v3840 = v3841;
                    v4183 = v4185;
                    v9686 = v9707;
                    v9687 = v11830;
                    v9688 = v9699;
                    v9689 = v9701;
                }
                let v3831 = v3830 / v745;
                let v11055 = v10477 * v3831;
                let v11058 = (v9687 - (Lanes([0.0, 0.0, v11055[0], 0.0, 0.0, 0.0]))) / v745;
                let v11059 = v11058 * v3831;
                let v11060 = v11059 + v11059;
                let v3834 = (v3831 * v3831) + v3833;
                let v3836 = v3831 + v3835;
                let v3838 = v3837 + v3836;
                let v3839 = v4 / v3838;
                let v3843 = v745 * v3840;
                let v11065 = v10477 * v3840;
                let v3844 = v3843 * v3839;
                let v11071 = (((Lanes([0.0, 0.0, v11065[0], 0.0, 0.0, 0.0])) + (v9689 * v745)) * v3839) + (((((v9688 + v11058) * v3839) * v10382) / v3838) * v3843);
                let v3845 = -v3844;
                let v11072 = v11071 * v10382;
                let v3846 = v3844 * v1043;
                let v11074 = v9412 * v3844;
                let v11076 = (v11071 * v1043) + (Lanes([v11074[0], v11074[1], 0.0, v11074[2], v11074[3], 0.0]));
                let v3850 = if (if v3745 == v3847 { 1.0 } else { 0.0 }) != 0.0 || (if v3846 <= v6 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v3863: f64;
                let v4136: f64;
                let v4231: f64;
                let v4323: f64;
                let v4334: f64;
                let v4421: f64;
                let v8301: f64;
                let v8478: f64;
                let v8560: f64;
                let v8572: f64;
                let v9710: Lanes<6>;
                let v9711: Lanes<6>;
                let v9712: Lanes<6>;
                let v9713: Lanes<6>;
                let v9714: Lanes<6>;
                let v9715: Lanes<6>;
                let v9716: Lanes<6>;
                if v3850 != 0.0 {
                    let v3851 = v1195 - v3649;
                    let v3852 = v1123 * v3851;
                    let v11079 = v9413 * v3851;
                    let v11082 = (Lanes([v11079[0], v11079[1], 0.0, v11079[2], v11079[3], 0.0])) + (((Lanes([v10820[0], v10820[1], v10820[2], v10820[3], v10820[4], 0.0])) - v9686) * v1123);
                    let v3854 = (-v164) * v134;
                    let v3855 = v3854 * v3852;
                    let v11083 = v11082 * v3854;
                    let v3859 = -v3856;
                    let v3860 = v3859 * v3852;
                    let v11084 = v11082 * v3859;
                    let v3861 = v3860 * v8;
                    let v11085 = v11084 * v8;
                    let v3862 = v3860 - v3861;
                    let v11086 = v11084 - v11085;
                    v3863 = v4;
                    v4136 = v85;
                    v4231 = v0;
                    v4323 = v4;
                    v4334 = v3649;
                    v4421 = v3852;
                    v8301 = v3649;
                    v8478 = v3855;
                    v8560 = v3862;
                    v8572 = v3861;
                    v9710 = v11054;
                    v9711 = v9686;
                    v9712 = v11082;
                    v9713 = v9686;
                    v9714 = v11083;
                    v9715 = v11086;
                    v9716 = v11085;
                } else {
                    v3863 = v0;
                    v4136 = v3745;
                    v4231 = v3846;
                    v4323 = v0;
                    v4334 = v0;
                    v4421 = v0;
                    v8301 = v0;
                    v8478 = v0;
                    v8560 = v0;
                    v8572 = v0;
                    v9710 = v11076;
                    v9711 = v11054;
                    v9712 = v11054;
                    v9713 = v11054;
                    v9714 = v11054;
                    v9715 = v11054;
                    v9716 = v11054;
                }
                let v3864 = if v3863 == v0 { 1.0 } else { 0.0 };
                let v4304: f64;
                let v4308: f64;
                let v4311: f64;
                let v4333: f64;
                let v4379: f64;
                let v4418: f64;
                let v4425: f64;
                let v4442: f64;
                let v9717: Lanes<6>;
                let v9718: Lanes<6>;
                let v9719: Lanes<6>;
                let v9720: Lanes<6>;
                let v9721: Lanes<6>;
                let v9722: Lanes<6>;
                let v9723: Lanes<6>;
                let v9724: Lanes<6>;
                if v3864 != 0.0 {
                    let v3865 = v1123 * v1123;
                    let v11087 = v9413 * v1123;
                    let v3866 = v487 / v3865;
                    let v11091 = (((v11087 + v11087) * v3866) * v10382) / v3865;
                    let v3867 = v73 / v3866;
                    let v11094 = ((v11091 * v3867) * v10382) / v3866;
                    let v3868 = v1195 - v358;
                    let v11095 = v11094 * v3868;
                    let v11098 = (Lanes([v11095[0], v11095[1], 0.0, v11095[2], v11095[3]])) + (v10820 * v3867);
                    let v3870 = v4 + (v3867 * v3868);
                    let v3871 = v4 + v3867;
                    let v3874 = if (if v3870 < v3871 { 1.0 } else { 0.0 }) != 0.0 && (if v3871 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v3906: f64;
                    let v9725: Lanes<5>;
                    if v3874 != 0.0 {
                        let v3875 = v3871 - v3870;
                        let v11099 = Lanes([v11094[0], v11094[1], 0.0, v11094[2], v11094[3]]);
                        let v11100 = v11099 - v11098;
                        let v3876 = v3875 * v3875;
                        let v11101 = v11100 * v3875;
                        let v11102 = v11101 + v11101;
                        let v3877 = v3871 * v3871;
                        let v11103 = v11094 * v3871;
                        let v11104 = v11103 + v11103;
                        let v3878 = v3876 * v3876;
                        let v11105 = v11102 * v3876;
                        let v3879 = v3877 * v3877;
                        let v11107 = v11104 * v3877;
                        let v3880 = v3878 * v3876;
                        let v3881 = v3879 * v3877;
                        let v11120 = ((((v11107 + v11107) * v3877) + (v11104 * v3879)) * v3877) + (v11104 * v3881);
                        let v3884 = (v3880 * v3876) + (v3881 * v3877);
                        let v11122 = (((((v11105 + v11105) * v3876) + (v11102 * v3878)) * v3876) + (v11102 * v3880)) + (Lanes([v11120[0], v11120[1], 0.0, v11120[2], v11120[3]]));
                        let v3901: f64;
                        let v9726: Lanes<5>;
                        if v3885 != 0.0 {
                            let v3895: f64;
                            if v3886 != 0.0 {
                                v3895 = v4;
                            } else {
                                let v3896: f64;
                                if v3887 != 0.0 {
                                    v3896 = v73;
                                } else {
                                    let v3897: f64;
                                    if v3888 != 0.0 {
                                        v3897 = v91;
                                    } else {
                                        let v3898: f64;
                                        if v3889 != 0.0 {
                                            v3898 = v85;
                                        } else {
                                            v3898 = v0;
                                        }
                                        v3897 = v3898;
                                    }
                                    v3896 = v3897;
                                }
                                v3895 = v3896;
                            }
                            let mut v3890: f64 = 0.0;
                            let mut v3892: f64 = 0.0;
                            let mut v9727: Lanes<5> = Lanes([0.0; 5]);
                            v3890 = v0;
                            v3892 = v3884;
                            v9727 = v11122;
                            loop {
                                let v3891 = if v3890 < v3895 { 1.0 } else { 0.0 };
                                if v3891 == 0.0 {
                                    break;
                                }
                                let v3893 = v3892.sqrt();
                                let v11678 = v9727 * (v9360 / (v10427 * v3893));
                                let v3894 = v3890 + v4;
                                v3890 = v3894;
                                v3892 = v3893;
                                v9727 = v11678;
                            }
                            v3901 = v3892;
                            v9726 = v9727;
                        } else {
                            let v3900 = v3884.powf(v3899);
                            let v11126 = v11122 * (v3899 * (v3884.powf(v11123)));
                            v3901 = v3900;
                            v9726 = v11126;
                        }
                        let v3902 = v4 / v3901;
                        let v3903 = v3875 * v3871;
                        let v11131 = v11094 * v3875;
                        let v3905 = v3871 - (v3903 * v3902);
                        let v11137 = v11099 - ((((v11100 * v3871) + (Lanes([v11131[0], v11131[1], 0.0, v11131[2], v11131[3]]))) * v3902) + ((((v9726 * v3902) * v10382) / v3901) * v3903));
                        v3906 = v3905;
                        v9725 = v11137;
                    } else {
                        v3906 = v3870;
                        v9725 = v11098;
                    }
                    let v3907 = v3906.sqrt();
                    let v3908 = v4 - v3907;
                    let v11142 = v11091 * v3908;
                    let v3910 = v1195 + (v3866 * v3908);
                    let v11146 = v10820 + ((Lanes([v11142[0], v11142[1], 0.0, v11142[2], v11142[3]])) + (((v9725 * (v9360 / (v10427 * v3907))) * v10382) * v3866));
                    let v11147 = v11146 * v3910;
                    let v3914 = ((v3910 * v3910) + v3912).sqrt();
                    let v11153 = (v11146 + ((v11147 + v11147) * (v9360 / (v10427 * v3914)))) * v8;
                    let v3918 = (v8 * (v3910 + v3914)) + v3917;
                    let v3919 = if v3918 < v0 { 1.0 } else { 0.0 };
                    let v3920: f64;
                    let v9728: Lanes<5>;
                    if v3919 != 0.0 {
                        v3920 = v0;
                        v9728 = v10571;
                    } else {
                        v3920 = v3918;
                        v9728 = v11153;
                    }
                    let v3921 = v818 / v3920;
                    let v11156 = (v10589 - (v9728 * v3921)) / v3920;
                    let v3922 = v2657 - v4;
                    let v3923 = v3921.powf(v3922);
                    let v11163 = ((v11156 * (v3922 * (v3921.powf((v3922 - v9360))))) * v3921) + (v11156 * v3923);
                    let v3925 = v4 + (v3923 * v3921);
                    let v3927 = (v4 / v2657) - v4;
                    let v3928 = v3925.powf(v3927);
                    let v3929 = v3928 * v3925;
                    let v3930 = v818 / v3929;
                    let v11173 = (v10589 - ((((v11163 * (v3927 * (v3925.powf((v3927 - v9360))))) * v3925) + (v11163 * v3928)) * v3930)) / v3929;
                    let v3931 = v830 - v3930;
                    let v11175 = v10402 * v3931;
                    let v3933 = (v658 * v3931).exp();
                    let v11179 = ((Lanes([0.0, 0.0, v11175[0], 0.0, 0.0])) + ((v10829 - v11173) * v658)) * v3933;
                    let v3934 = if v3930 <= v0 { 1.0 } else { 0.0 };
                    let v3970: f64;
                    let v9729: Lanes<6>;
                    if v3934 != 0.0 {
                        v3970 = v3649;
                        v9729 = v9686;
                    } else {
                        let v3964: f64;
                        let v9730: Lanes<6>;
                        if v3935 != 0.0 {
                            let v3936 = v0 - v3649;
                            let v11180 = v9686 * v10382;
                            v3964 = v3936;
                            v9730 = v11180;
                        } else {
                            v3964 = v0;
                            v9730 = v11054;
                        }
                        let v3963: f64;
                        let v9731: Lanes<6>;
                        if v3937 != 0.0 {
                            let v3939 = v3938 - v3649;
                            let v11182 = (Lanes([v9674[0], v9674[1], v9674[2], v9674[3], v9674[4], 0.0])) - v9686;
                            let v3940 = if v3939 >= v0 { 1.0 } else { 0.0 };
                            let v3941: f64;
                            let v9732: Lanes<6>;
                            if v3940 != 0.0 {
                                v3941 = v3939;
                                v9732 = v11182;
                            } else {
                                v3941 = v0;
                                v9732 = v11054;
                            }
                            let v11185 = (v9732 * v3942) - (Lanes([v11173[0], v11173[1], v11173[2], v11173[3], v11173[4], 0.0]));
                            let v3945 = ((v3942 * v3941) - v3930) - v1980;
                            let v3949 = (v85 * (v3946 * v3941)) * v1980;
                            let v11188 = ((v9732 * v3946) * v85) * v1980;
                            let v3950 = if v3949 > v0 { 1.0 } else { 0.0 };
                            let v3952: f64;
                            let v9733: Lanes<6>;
                            if v3950 != 0.0 {
                                v3952 = v3949;
                                v9733 = v11188;
                            } else {
                                let v3951 = -v3949;
                                let v11189 = v11188 * v10382;
                                v3952 = v3951;
                                v9733 = v11189;
                            }
                            let v11190 = v11185 * v3945;
                            let v3955 = ((v3945 * v3945) + v3952).sqrt();
                            let v3960 = (v3956 * v3941) - (v8 * (v3945 + v3955));
                            let v11199 = (v9732 * v3956) - ((v11185 + (((v11190 + v11190) + v9733) * (v9360 / (v10427 * v3955)))) * v8);
                            let v3961 = if v3960 <= v3941 { 1.0 } else { 0.0 };
                            let v3962: f64;
                            let v9734: Lanes<6>;
                            if v3961 != 0.0 {
                                v3962 = v3960;
                                v9734 = v11199;
                            } else {
                                v3962 = v3941;
                                v9734 = v9732;
                            }
                            v3963 = v3962;
                            v9731 = v9734;
                        } else {
                            v3963 = v3964;
                            v9731 = v9730;
                        }
                        let v3965 = if v3963 < v0 { 1.0 } else { 0.0 };
                        let v3967: f64;
                        let v9735: Lanes<6>;
                        if v3965 != 0.0 {
                            v3967 = v0;
                            v9735 = v11054;
                        } else {
                            let v3966 = if v3963 > v3930 { 1.0 } else { 0.0 };
                            let v3968: f64;
                            let v9736: Lanes<6>;
                            if v3966 != 0.0 {
                                let v11200 = Lanes([v11173[0], v11173[1], v11173[2], v11173[3], v11173[4], 0.0]);
                                v3968 = v3930;
                                v9736 = v11200;
                            } else {
                                v3968 = v3963;
                                v9736 = v9731;
                            }
                            v3967 = v3968;
                            v9735 = v9736;
                        }
                        let v3969 = v3649 + v3967;
                        let v11201 = v9686 + v9735;
                        v3970 = v3969;
                        v9729 = v11201;
                    }
                    let mut v3971: f64 = 0.0;
                    let mut v3974: f64 = 0.0;
                    let mut v4107: f64 = 0.0;
                    let mut v4139: f64 = 0.0;
                    let mut v4143: f64 = 0.0;
                    let mut v4146: f64 = 0.0;
                    let mut v9737: Lanes<6> = Lanes([0.0; 6]);
                    let mut v9738: Lanes<6> = Lanes([0.0; 6]);
                    let mut v9739: Lanes<6> = Lanes([0.0; 6]);
                    let mut v9740: Lanes<6> = Lanes([0.0; 6]);
                    v3971 = v4;
                    v3974 = v3970;
                    v4107 = v0;
                    v4139 = v3830;
                    v4143 = v0;
                    v4146 = v0;
                    v9737 = v9729;
                    v9738 = v9687;
                    v9739 = v11054;
                    v9740 = v11054;
                    loop {
                        let v3973 = if v3971 <= v3972 { 1.0 } else { 0.0 };
                        if v3973 == 0.0 {
                            break;
                        }
                        let v3975 = v3974 - v830;
                        let v11453 = v9737 - (Lanes([v9405[0], v9405[1], 0.0, 0.0, v9405[2], 0.0]));
                        let v3976 = v658 * v3975;
                        let v11454 = v10402 * v3975;
                        let v11457 = (Lanes([0.0, 0.0, v11454[0], 0.0, 0.0, 0.0])) + (v11453 * v658);
                        let v3977 = v3975 - v3635;
                        let v3978 = v3645 * v3977;
                        let v11458 = v11052 * v3977;
                        let v11461 = (Lanes([0.0, 0.0, v11458[0], 0.0, 0.0, 0.0])) + (v11453 * v3645);
                        let v3979 = if v3978 < v2530 { 1.0 } else { 0.0 };
                        let v3989: f64;
                        let v3993: f64;
                        let v9741: Lanes<6>;
                        let v9742: Lanes<6>;
                        if v3979 != 0.0 {
                            let v3980 = v3978.exp();
                            let v11462 = v11461 * v3980;
                            let v3983 = ((-v3645) * v3635).exp();
                            let v11465 = ((v11052 * v10382) * v3635) * v3983;
                            let v11467 = v11462 - (Lanes([0.0, 0.0, v11465[0], 0.0, 0.0, 0.0]));
                            let v3985 = v4 + (v3980 - v3983);
                            let v3987 = (v3985.ln()) / v3645;
                            let v11470 = v11052 * v3987;
                            let v11473 = ((v11467 * (v9360 / v3985)) - (Lanes([0.0, 0.0, v11470[0], 0.0, 0.0, 0.0]))) / v3645;
                            let v3988 = v3980 / v3985;
                            let v11476 = (v11462 - (v11467 * v3988)) / v3985;
                            v3989 = v3987;
                            v3993 = v3988;
                            v9741 = v11473;
                            v9742 = v11476;
                        } else {
                            v3989 = v3977;
                            v3993 = v4;
                            v9741 = v11453;
                            v9742 = v11054;
                        }
                        let v3990 = v658 * v3989;
                        let v11477 = v10402 * v3989;
                        let v11480 = (Lanes([0.0, 0.0, v11477[0], 0.0, 0.0, 0.0])) + (v9741 * v658);
                        let v3991 = v3976.abs();
                        let v3992 = if v3991 < v3667 { 1.0 } else { 0.0 };
                        let v4064: f64;
                        let v4072: f64;
                        let v9743: Lanes<6>;
                        let v9744: Lanes<6>;
                        if v3992 != 0.0 {
                            let v11583 = v9742 * v3993;
                            let v3997 = ((v4 - (v3993 * v3993)) / v73).sqrt();
                            let v11589 = (((v11583 + v11583) * v10382) / v73) * (v9360 / (v10427 * v3997));
                            let v3998 = v3976 * v3997;
                            let v11592 = (v11457 * v3997) + (v11589 * v3976);
                            let v3999 = v658 * v3997;
                            let v11593 = v10402 * v3997;
                            let v11596 = (Lanes([0.0, 0.0, v11593[0], 0.0, 0.0, 0.0])) + (v11589 * v658);
                            let v4000 = if v3976 < v0 { 1.0 } else { 0.0 };
                            let v4065: f64;
                            let v4073: f64;
                            let v9745: Lanes<6>;
                            let v9746: Lanes<6>;
                            if v4000 != 0.0 {
                                let v4001 = -v3998;
                                let v11597 = v11592 * v10382;
                                let v4002 = -v3999;
                                let v11598 = v11596 * v10382;
                                v4065 = v4001;
                                v4073 = v4002;
                                v9745 = v11597;
                                v9746 = v11598;
                            } else {
                                v4065 = v3998;
                                v4073 = v3999;
                                v9745 = v11592;
                                v9746 = v11596;
                            }
                            v4064 = v4065;
                            v4072 = v4073;
                            v9743 = v9745;
                            v9744 = v9746;
                        } else {
                            let v4003 = if v3991 < v3679 { 1.0 } else { 0.0 };
                            let v4066: f64;
                            let v4074: f64;
                            let v9747: Lanes<6>;
                            let v9748: Lanes<6>;
                            if v4003 != 0.0 {
                                let v11505 = v11457 * v3976;
                                let v4005 = (v3976 * v3976) / v73;
                                let v4006 = v3976 / v91;
                                let v11508 = v11457 / v91;
                                let v4007 = v3976 / v85;
                                let v11509 = v11457 / v85;
                                let v4009 = v4 - (v3976 / v639);
                                let v4011 = v4 - (v4007 * v4009);
                                let v4013 = v4 - (v4006 * v4011);
                                let v4015 = v3976 / v73;
                                let v4016 = v4 - v4007;
                                let v4018 = v4 - (v4006 * v4016);
                                let v4020 = v4 - (v4015 * v4018);
                                let v11536 = v11480 * v3990;
                                let v4023 = (v3990 * v3990) / v73;
                                let v4024 = v3990 / v91;
                                let v11539 = v11480 / v91;
                                let v4025 = v3990 / v85;
                                let v11540 = v11480 / v85;
                                let v4027 = v4 - (v3990 / v639);
                                let v4029 = v4 - (v4025 * v4027);
                                let v4031 = v4 - (v4024 * v4029);
                                let v4033 = v3990 / v73;
                                let v4034 = v4 - v4025;
                                let v4036 = v4 - (v4024 * v4034);
                                let v4038 = v4 - (v4033 * v4036);
                                let v4039 = v3990 * v4038;
                                let v4041 = ((v4005 * v4013) - (v4023 * v4031)).sqrt();
                                let v11570 = (((((v11505 + v11505) / v73) * v4013) + ((((v11508 * v4011) + ((((v11509 * v4009) + (((v11457 / v639) * v10382) * v4007)) * v10382) * v4006)) * v10382) * v4005)) - ((((v11536 + v11536) / v73) * v4031) + ((((v11539 * v4029) + ((((v11540 * v4027) + (((v11480 / v639) * v10382) * v4025)) * v10382) * v4024)) * v10382) * v4023))) * (v9360 / (v10427 * v4041));
                                let v4042 = v658 * v8;
                                let v4044 = (v3976 * v4020) - (v3993 * v4039);
                                let v11576 = (v10402 * v8) * v4044;
                                let v4046 = (v4042 * v4044) / v4041;
                                let v11582 = (((Lanes([0.0, 0.0, v11576[0], 0.0, 0.0, 0.0])) + ((((v11457 * v4020) + (((((v11457 / v73) * v4018) + ((((v11508 * v4016) + ((v11509 * v10382) * v4006)) * v10382) * v4015)) * v10382) * v3976)) - ((v9742 * v4039) + (((v11480 * v4038) + (((((v11480 / v73) * v4036) + ((((v11539 * v4034) + ((v11540 * v10382) * v4024)) * v10382) * v4033)) * v10382) * v3990)) * v3993))) * v4042)) - (v11570 * v4046)) / v4041;
                                v4066 = v4041;
                                v4074 = v4046;
                                v9747 = v11570;
                                v9748 = v11582;
                            } else {
                                let v4048 = (-v3976).exp();
                                let v11482 = (v11457 * v10382) * v4048;
                                let v4050 = (-v3990).exp();
                                let v11484 = (v11480 * v10382) * v4050;
                                let v4054 = ((v3976 - v3990) + (v4048 - v4050)).sqrt();
                                let v11490 = ((v11457 - v11480) + (v11482 - v11484)) * (v9360 / (v10427 * v4054));
                                let v4055 = v658 * v8;
                                let v4057 = v4 - v4050;
                                let v4059 = (v4 - v4048) - (v3993 * v4057);
                                let v11498 = (v10402 * v8) * v4059;
                                let v4061 = (v4055 * v4059) / v4054;
                                let v11504 = (((Lanes([0.0, 0.0, v11498[0], 0.0, 0.0, 0.0])) + (((v11482 * v10382) - ((v9742 * v4057) + ((v11484 * v10382) * v3993))) * v4055)) - (v11490 * v4061)) / v4054;
                                v4066 = v4054;
                                v4074 = v4061;
                                v9747 = v11490;
                                v9748 = v11504;
                            }
                            v4064 = v4066;
                            v4072 = v4074;
                            v9743 = v9747;
                            v9744 = v9748;
                        }
                        let v4063 = if v4136 == v4062 { 1.0 } else { 0.0 };
                        let v4068: f64;
                        let v9749: Lanes<6>;
                        if v4063 != 0.0 {
                            v4068 = v0;
                            v9749 = v11054;
                        } else {
                            let v4067 = v757 * v4064;
                            let v11599 = v10495 * v4064;
                            let v11602 = (Lanes([0.0, 0.0, v11599[0], 0.0, 0.0, 0.0])) + (v9743 * v757);
                            v4068 = v4067;
                            v9749 = v11602;
                        }
                        let v4069 = v486 * v4068;
                        let v11603 = v9749 * v486;
                        let v4070 = if v3976 < v0 { 1.0 } else { 0.0 };
                        let v4097: f64;
                        let v4103: f64;
                        let v4147: f64;
                        let v9750: Lanes<6>;
                        let v9751: Lanes<6>;
                        let v9752: Lanes<6>;
                        if v4070 != 0.0 {
                            let v4071 = -v4064;
                            let v11644 = v9743 * v10382;
                            let v4075 = -v4072;
                            let v11645 = v9744 * v10382;
                            v4097 = v4071;
                            v4103 = v4075;
                            v4147 = v4146;
                            v9750 = v11644;
                            v9751 = v11645;
                            v9752 = v9740;
                        } else {
                            let v4076 = if v3976 < v112 { 1.0 } else { 0.0 };
                            let v4098: f64;
                            let v4104: f64;
                            let v4148: f64;
                            let v9753: Lanes<6>;
                            let v9754: Lanes<6>;
                            let v9755: Lanes<6>;
                            if v4076 != 0.0 {
                                v4098 = v4064;
                                v4104 = v4072;
                                v4148 = v4146;
                                v9753 = v9743;
                                v9754 = v9744;
                                v9755 = v9740;
                            } else {
                                let v4077 = v3974 - v3930;
                                let v11606 = v10402 * v4077;
                                let v4079 = (v658 * v4077).exp();
                                let v11610 = ((Lanes([0.0, 0.0, v11606[0], 0.0, 0.0, 0.0])) + ((v9737 - (Lanes([v11173[0], v11173[1], v11173[2], v11173[3], v11173[4], 0.0]))) * v658)) * v4079;
                                let v4080 = v3976 + v4;
                                let v11611 = v11179 * v4080;
                                let v4082 = v4079 - (v3933 * v4080);
                                let v4083 = v754 * v4082;
                                let v11616 = v10488 * v4082;
                                let v11619 = (Lanes([0.0, 0.0, v11616[0], 0.0, 0.0, 0.0])) + ((v11610 - ((Lanes([v11611[0], v11611[1], v11611[2], v11611[3], v11611[4], 0.0])) + (v11457 * v3933))) * v754);
                                let v4084 = v754 * v658;
                                let v4085 = v4079 - v3933;
                                let v11625 = ((v10488 * v658) + (v10402 * v754)) * v4085;
                                let v11629 = v9743 * v4064;
                                let v4089 = ((v4064 * v4064) + v4083).sqrt();
                                let v11634 = ((v11629 + v11629) + v11619) * (v9360 / (v10427 * v4089));
                                let v4090 = v73 * v4072;
                                let v4094 = (v8 * ((v4090 * v4064) + (v4084 * v4085))) / v4089;
                                let v11643 = ((((((v9744 * v73) * v4064) + (v9743 * v4090)) + ((Lanes([0.0, 0.0, v11625[0], 0.0, 0.0, 0.0])) + ((v11610 - (Lanes([v11179[0], v11179[1], v11179[2], v11179[3], v11179[4], 0.0]))) * v4084))) * v8) - (v11634 * v4094)) / v4089;
                                v4098 = v4089;
                                v4104 = v4094;
                                v4148 = v4083;
                                v9753 = v11634;
                                v9754 = v11643;
                                v9755 = v11619;
                            }
                            v4097 = v4098;
                            v4103 = v4104;
                            v4147 = v4148;
                            v9750 = v9753;
                            v9751 = v9754;
                            v9752 = v9755;
                        }
                        let v11646 = v10820 * v10382;
                        let v11649 = v10826 * v4097;
                        let v11654 = v9412 * v3627;
                        let v11655 = v9685 * v1043;
                        let v11658 = (Lanes([v11654[0], v11654[1], v11654[2], v11654[3], 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, v11655[0]]));
                        let v4102 = (((-v1195) + v3974) + (v1201 * v4097)) - (v1043 * v3627);
                        let v11660 = (((Lanes([v11646[0], v11646[1], v11646[2], v11646[3], v11646[4], 0.0])) + v9737) + ((Lanes([v11649[0], v11649[1], v11649[2], v11649[3], v11649[4], 0.0])) + (v9750 * v1201))) - (Lanes([v11658[0], v11658[1], 0.0, v11658[2], v11658[3], v11658[4]]));
                        let v11661 = v10826 * v4103;
                        let v11664 = (Lanes([v11661[0], v11661[1], v11661[2], v11661[3], v11661[4], 0.0])) + (v9751 * v1201);
                        let v4106 = v4 + (v1201 * v4103);
                        let v4110 = if (if v4107 == v4 { 1.0 } else { 0.0 }) != 0.0 && (if v3971 > v91 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v4133: f64;
                        let v4135: f64;
                        let v4137: f64;
                        let v9756: Lanes<6>;
                        if v4110 != 0.0 {
                            v4133 = v4111;
                            v4135 = v3974;
                            v4137 = v4107;
                            v9756 = v9737;
                        } else {
                            let v4113 = (-v4102) / v4106;
                            let v11668 = ((v11660 * v10382) - (v11664 * v4113)) / v4106;
                            let v4115 = v3974.abs();
                            let v11672 = v9737 * ((v10427 * (if v3974 >= v11296 { 1.0 } else { 0.0 })) - v9360);
                            let v4116 = if v4 >= v4115 { 1.0 } else { 0.0 };
                            let v4117: f64;
                            let v9757: Lanes<6>;
                            if v4116 != 0.0 {
                                v4117 = v4;
                                v9757 = v11054;
                            } else {
                                v4117 = v4115;
                                v9757 = v11672;
                            }
                            let v4119 = v4114 * (v4 + v4117);
                            let v11673 = v9757 * v4114;
                            let v4121 = if (v4113.abs()) > v4119 { 1.0 } else { 0.0 };
                            let v4126: f64;
                            let v9758: Lanes<6>;
                            if v4121 != 0.0 {
                                let v4122 = if v4113 >= v0 { 1.0 } else { 0.0 };
                                let v4124: f64;
                                if v4122 != 0.0 {
                                    v4124 = v4;
                                } else {
                                    v4124 = v4123;
                                }
                                let v4125 = v4119 * v4124;
                                let v11674 = v11673 * v4124;
                                v4126 = v4125;
                                v9758 = v11674;
                            } else {
                                v4126 = v4113;
                                v9758 = v11668;
                            }
                            let v4127 = v3974 + v4126;
                            let v11675 = v9737 + v9758;
                            let v4132 = if (if (v4126.abs()) <= v856 { 1.0 } else { 0.0 }) != 0.0 && (if (v4102.abs()) <= v3501 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                            let v4138: f64;
                            if v4132 != 0.0 {
                                v4138 = v4;
                            } else {
                                v4138 = v4107;
                            }
                            v4133 = v3971;
                            v4135 = v4127;
                            v4137 = v4138;
                            v9756 = v11675;
                        }
                        let v4134 = v4133 + v4;
                        v3971 = v4134;
                        v3974 = v4135;
                        v4107 = v4137;
                        v4139 = v4069;
                        v4143 = v4097;
                        v4146 = v4147;
                        v9737 = v9756;
                        v9738 = v11603;
                        v9739 = v9750;
                        v9740 = v9752;
                    }
                    let v4140 = v4139 / v745;
                    let v11202 = v10477 * v4140;
                    let v11205 = (v9738 - (Lanes([0.0, 0.0, v11202[0], 0.0, 0.0, 0.0]))) / v745;
                    let v4144 = v4143 + (v4140 + v4141);
                    let v4145 = v4 / v4144;
                    let v4149 = v745 * v4146;
                    let v11210 = v10477 * v4146;
                    let v4151 = -(v4149 * v4145);
                    let v11217 = ((((Lanes([0.0, 0.0, v11210[0], 0.0, 0.0, 0.0])) + (v9740 * v745)) * v4145) + (((((v9739 + v11205) * v4145) * v10382) / v4144) * v4149)) * v10382;
                    let v4152 = v3974 - v3649;
                    let v11218 = v9737 - v9686;
                    let v4153 = v658 / v3834;
                    let v4156 = ((v4153 * v4152) + v4).sqrt();
                    let v4157 = v4156 + v4;
                    let v4158 = v4 / v4157;
                    let v4159 = v4158 / v3836;
                    let v4161 = v8 * (v3831 + v4140);
                    let v11236 = (v11058 + v11205) * v8;
                    let v11238 = v10820 + (Lanes([0.0, 0.0, v10407[0], 0.0, 0.0]));
                    let v4166 = (v1195 + v660) - (v8 * ((v73 * v3649) + v4152));
                    let v4168 = (-v4161) + v4159;
                    let v4169 = v658 * v1123;
                    let v11246 = v10402 * v1123;
                    let v11247 = v9413 * v658;
                    let v4170 = v658 * v745;
                    let v11254 = ((Lanes([0.0, 0.0, v11246[0], 0.0, 0.0])) + (Lanes([v11247[0], v11247[1], 0.0, v11247[2], v11247[3]]))) * v4166;
                    let v11258 = ((v10402 * v745) + (v10477 * v658)) * v4168;
                    let v4173 = (v4169 * v4166) + (v4170 * v4168);
                    let v11262 = ((Lanes([v11254[0], v11254[1], v11254[2], v11254[3], v11254[4], 0.0])) + (((Lanes([v11238[0], v11238[1], v11238[2], v11238[3], v11238[4], 0.0])) - (((v9686 * v73) + v11218) * v8)) * v4169)) + ((Lanes([0.0, 0.0, v11258[0], 0.0, 0.0, 0.0])) + (((v11236 * v10382) + (((((((((((Lanes([0.0, 0.0, v10402[0], 0.0, 0.0, 0.0])) - (v11060 * v4153)) / v3834) * v4152) + (v11218 * v4153)) * (v9360 / (v10427 * v4156))) * v4158) * v10382) / v4157) - (v11058 * v4159)) / v3836)) * v4170));
                    let v4174 = v4139 + v3830;
                    let v11263 = v9738 + v9687;
                    let v4175 = v4174 / v73;
                    let v11264 = v11263 / v73;
                    let v4176 = v4151 + v3845;
                    let v11265 = v11217 + v11072;
                    let v4178 = (-v4176) / v73;
                    let v11267 = (v11265 * v10382) / v73;
                    let v4179 = v4139 - v3830;
                    let v11268 = v9738 - v9687;
                    let v4181 = -(v4151 - v3845);
                    let v11270 = (v11217 - v11072) * v10382;
                    let v4182 = v745 * v745;
                    let v11271 = v10477 * v745;
                    let v11272 = v11271 + v11271;
                    let v4186 = if v4183 <= v4 { 1.0 } else { 0.0 };
                    let v4197: f64;
                    let v9759: Lanes<6>;
                    if v4186 != 0.0 {
                        let v4187 = v4178 * v658;
                        let v11277 = v10402 * v4178;
                        let v4190 = v4179 * v4179;
                        let v11284 = v11268 * v4179;
                        let v4192 = (v4190 * v4179) / v4182;
                        let v11289 = v11272 * v4192;
                        let v4194 = ((v4187 * v4152) - v4181) - (v4192 / v641);
                        let v11294 = (((((v11267 * v658) + (Lanes([0.0, 0.0, v11277[0], 0.0, 0.0, 0.0]))) * v4152) + (v11218 * v4187)) - v11270) - ((((((v11284 + v11284) * v4179) + (v11268 * v4190)) - (Lanes([0.0, 0.0, v11289[0], 0.0, 0.0, 0.0]))) / v4182) / v641);
                        v4197 = v4194;
                        v9759 = v11294;
                    } else {
                        let v4195 = v4152 * v4173;
                        let v11275 = (v11218 * v4173) + (v11262 * v4152);
                        v4197 = v4195;
                        v9759 = v11275;
                    }
                    let v4199 = if (if v65 >= v4 { 1.0 } else { 0.0 }) != 0.0 && (if v4197 < v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v4224: f64;
                    let v9760: Lanes<6>;
                    if v4199 != 0.0 {
                        v4224 = v0;
                        v9760 = v11054;
                    } else {
                        v4224 = v4197;
                        v9760 = v9759;
                    }
                    let v4419: f64;
                    let v9761: Lanes<6>;
                    if v4186 != 0.0 {
                        let v4201 = if (v4152.abs()) > v18 { 1.0 } else { 0.0 };
                        let v4420: f64;
                        let v9762: Lanes<6>;
                        if v4201 != 0.0 {
                            let v4202 = v4178 * v658;
                            let v11298 = v10402 * v4178;
                            let v4204 = (v4202 * v4152) - v4181;
                            let v4206 = v73 * v4175;
                            let v11308 = v11264 * v73;
                            let v4208 = v1123 / v658;
                            let v11310 = v10402 * v4208;
                            let v4210 = (v4206 * v4175) / v4182;
                            let v11318 = v11272 * v4210;
                            let v11323 = v11268 * v4179;
                            let v4213 = (v4179 * v4179) / v4182;
                            let v11325 = v11272 * v4213;
                            let v4215 = (v4 - v4210) + (v4213 / v10);
                            let v11331 = (((Lanes([v9413[0], v9413[1], 0.0, v9413[2], v9413[3]])) - (Lanes([0.0, 0.0, v11310[0], 0.0, 0.0]))) / v658) * v4215;
                            let v4217 = (v4178 - v4206) + (v4208 * v4215);
                            let v4218 = v4217 * v4179;
                            let v4219 = v4218 * v4179;
                            let v4221 = (v4219 * v4179) / v4182;
                            let v11345 = v11272 * v4221;
                            let v4225 = ((v4175 * v4204) + (v4221 / v641)) / v4224;
                            let v11353 = ((((v11264 * v4204) + ((((((v11267 * v658) + (Lanes([0.0, 0.0, v11298[0], 0.0, 0.0, 0.0]))) * v4152) + (v11218 * v4202)) - v11270) * v4175)) + (((((((((((v11267 - v11308) + ((Lanes([v11331[0], v11331[1], v11331[2], v11331[3], v11331[4], 0.0])) + (((((((v11308 * v4175) + (v11264 * v4206)) - (Lanes([0.0, 0.0, v11318[0], 0.0, 0.0, 0.0]))) / v4182) * v10382) + ((((v11323 + v11323) - (Lanes([0.0, 0.0, v11325[0], 0.0, 0.0, 0.0]))) / v4182) / v10)) * v4208))) * v4179) + (v11268 * v4217)) * v4179) + (v11268 * v4218)) * v4179) + (v11268 * v4219)) - (Lanes([0.0, 0.0, v11345[0], 0.0, 0.0, 0.0]))) / v4182) / v641)) - (v9760 * v4225)) / v4224;
                            v4420 = v4225;
                            v9762 = v11353;
                        } else {
                            v4420 = v4175;
                            v9762 = v11264;
                        }
                        v4419 = v4420;
                        v9761 = v9762;
                    } else {
                        let v4226 = v8 * v4174;
                        let v11295 = v11263 * v8;
                        v4419 = v4226;
                        v9761 = v11295;
                    }
                    let v4227 = v73 * v1201;
                    let v4228 = v4161 - v3836;
                    let v11356 = (v10826 * v73) * v4228;
                    let v4230 = v4152 + (v4227 * v4228);
                    let v4232 = v4 / v4231;
                    let v4235 = v4 - (v4 - (v4230 * v4232));
                    let v11368 = ((((v11218 + ((Lanes([v11356[0], v11356[1], v11356[2], v11356[3], v11356[4], 0.0])) + ((v11236 - v11058) * v4227))) * v4232) + ((((v9710 * v4232) * v10382) / v4231) * v4230)) * v10382) * v10382;
                    let v4236 = v4235 * v4235;
                    let v11369 = v11368 * v4235;
                    let v11370 = v11369 + v11369;
                    let v4237 = v4236 * v4236;
                    let v11371 = v11370 * v4236;
                    let v4238 = v4237 * v4236;
                    let v11378 = ((((v11371 + v11371) * v4236) + (v11370 * v4237)) * v4236) + (v11370 * v4238);
                    let v4241 = (v4238 * v4236) + v4240;
                    let v4258: f64;
                    let v9763: Lanes<6>;
                    if v4242 != 0.0 {
                        let v4252: f64;
                        if v4243 != 0.0 {
                            v4252 = v4;
                        } else {
                            let v4253: f64;
                            if v4244 != 0.0 {
                                v4253 = v73;
                            } else {
                                let v4254: f64;
                                if v4245 != 0.0 {
                                    v4254 = v91;
                                } else {
                                    let v4255: f64;
                                    if v4246 != 0.0 {
                                        v4255 = v85;
                                    } else {
                                        v4255 = v0;
                                    }
                                    v4254 = v4255;
                                }
                                v4253 = v4254;
                            }
                            v4252 = v4253;
                        }
                        let mut v4247: f64 = 0.0;
                        let mut v4249: f64 = 0.0;
                        let mut v9764: Lanes<6> = Lanes([0.0; 6]);
                        v4247 = v0;
                        v4249 = v4241;
                        v9764 = v11378;
                        loop {
                            let v4248 = if v4247 < v4252 { 1.0 } else { 0.0 };
                            if v4248 == 0.0 {
                                break;
                            }
                            let v4250 = v4249.sqrt();
                            let v11451 = v9764 * (v9360 / (v10427 * v4250));
                            let v4251 = v4247 + v4;
                            v4247 = v4251;
                            v4249 = v4250;
                            v9764 = v11451;
                        }
                        v4258 = v4249;
                        v9763 = v9764;
                    } else {
                        let v4257 = v4241.powf(v4256);
                        let v11382 = v11378 * (v4256 * (v4241.powf(v11379)));
                        v4258 = v4257;
                        v9763 = v11382;
                    }
                    let v4259 = v4 / v4258;
                    let v4261 = v4 - (v4235 * v4259);
                    let v11389 = ((v11368 * v4259) + ((((v9763 * v4259) * v10382) / v4258) * v4235)) * v10382;
                    let v4262 = v4 + v4261;
                    let v11392 = (v11389 * v4262) + (v11389 * v4261);
                    let v4264 = v4 + (v4261 * v4262);
                    let v4266 = if v4262 >= v4265 { 1.0 } else { 0.0 };
                    let v4268: f64;
                    let v9765: Lanes<6>;
                    if v4266 != 0.0 {
                        v4268 = v4262;
                        v9765 = v11389;
                    } else {
                        v4268 = v4267;
                        v9765 = v11054;
                    }
                    let v4426: f64;
                    let v9766: Lanes<6>;
                    if v4186 != 0.0 {
                        let v4271 = if (v4152.abs()) > v18 { 1.0 } else { 0.0 };
                        let v4427: f64;
                        let v9767: Lanes<6>;
                        if v4271 != 0.0 {
                            let v11394 = v11267 * v4178;
                            let v11396 = v11270 * v4181;
                            let v4275 = (v4178 * v4178) + ((v4181 * v4181) / v3518);
                            let v4276 = v4275 * v658;
                            let v11401 = v10402 * v4275;
                            let v4281 = v1123 / v658;
                            let v11412 = v10402 * v4281;
                            let v4282 = v4281 * v4179;
                            let v11417 = (((Lanes([v9413[0], v9413[1], 0.0, v9413[2], v9413[3]])) - (Lanes([0.0, 0.0, v11412[0], 0.0, 0.0]))) / v658) * v4179;
                            let v4284 = (v4282 * v4179) / v4182;
                            let v11424 = v11272 * v4284;
                            let v4286 = (v73 * v4178) + (v4284 / v639);
                            let v4287 = v4286 * v4179;
                            let v4288 = v4287 * v4179;
                            let v4290 = (v4288 * v4179) / v4182;
                            let v11439 = v11272 * v4290;
                            let v4293 = (((v4276 * v4152) - (v4178 * v4181)) - (v4290 / v641)) / v4224;
                            let v11447 = (((((((((v11394 + v11394) + ((v11396 + v11396) / v3518)) * v658) + (Lanes([0.0, 0.0, v11401[0], 0.0, 0.0, 0.0]))) * v4152) + (v11218 * v4276)) - ((v11267 * v4181) + (v11270 * v4178))) - (((((((((((v11267 * v73) + (((((((Lanes([v11417[0], v11417[1], v11417[2], v11417[3], v11417[4], 0.0])) + (v11268 * v4281)) * v4179) + (v11268 * v4282)) - (Lanes([0.0, 0.0, v11424[0], 0.0, 0.0, 0.0]))) / v4182) / v639)) * v4179) + (v11268 * v4286)) * v4179) + (v11268 * v4287)) * v4179) + (v11268 * v4288)) - (Lanes([0.0, 0.0, v11439[0], 0.0, 0.0, 0.0]))) / v4182) / v641)) - (v9760 * v4293)) / v4224;
                            v4427 = v4293;
                            v9767 = v11447;
                        } else {
                            v4427 = v4178;
                            v9767 = v11267;
                        }
                        v4426 = v4427;
                        v9766 = v9767;
                    } else {
                        let v4295 = v4294 * v4176;
                        let v11393 = v11265 * v4294;
                        v4426 = v4295;
                        v9766 = v11393;
                    }
                    let v4296 = if v3739 == v0 { 1.0 } else { 0.0 };
                    if v4296 != 0.0 {
                    } else {
                    }
                    let v4297 = if v4107 == v0 { 1.0 } else { 0.0 };
                    if v4297 != 0.0 {
                    } else {
                    }
                    let v4299 = if (v3739 + v4107) < v4 { 1.0 } else { 0.0 };
                    if v4299 != 0.0 {
                    } else {
                    }
                    v4304 = v4261;
                    v4308 = v4268;
                    v4311 = v4264;
                    v4333 = v3974;
                    v4379 = v4224;
                    v4418 = v4419;
                    v4425 = v4426;
                    v4442 = v4152;
                    v9717 = v11389;
                    v9718 = v9765;
                    v9719 = v11392;
                    v9720 = v9737;
                    v9721 = v9760;
                    v9722 = v9761;
                    v9723 = v9766;
                    v9724 = v11218;
                } else {
                    v4304 = v0;
                    v4308 = v0;
                    v4311 = v0;
                    v4333 = v4334;
                    v4379 = v0;
                    v4418 = v4421;
                    v4425 = v0;
                    v4442 = v0;
                    v9717 = v11054;
                    v9718 = v11054;
                    v9719 = v11054;
                    v9720 = v9711;
                    v9721 = v11054;
                    v9722 = v9712;
                    v9723 = v11054;
                    v9724 = v11054;
                }
                let v11448 = Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v9685[0]]);
                v4300 = v3863;
                v4302 = v4304;
                v4306 = v4308;
                v4309 = v4311;
                v4320 = v4323;
                v4331 = v4333;
                v4335 = v3649;
                v4343 = v3844;
                v4376 = v4379;
                v4416 = v4418;
                v4423 = v4425;
                v4433 = v0;
                v4434 = v0;
                v4440 = v4442;
                v4632 = v0;
                v4730 = v731;
                v4782 = v728;
                v4838 = v4231;
                v4959 = v0;
                v4968 = v0;
                v4972 = v0;
                v5088 = v5090;
                v5496 = v3627;
                v5638 = v0;
                v5716 = v0;
                v5776 = v0;
                v8299 = v8301;
                v8476 = v8478;
                v8481 = v0;
                v8486 = v0;
                v8492 = v0;
                v8559 = v8560;
                v8571 = v8572;
                v9206 = v0;
                v9429 = v9717;
                v9430 = v9718;
                v9431 = v9719;
                v9432 = v9720;
                v9433 = v9686;
                v9434 = v11071;
                v9435 = v9721;
                v9436 = v9722;
                v9437 = v9723;
                v9438 = v11054;
                v9439 = v11054;
                v9440 = v9724;
                v9441 = v11054;
                v9442 = v10454;
                v9443 = v10449;
                v9444 = v9710;
                v9445 = v10571;
                v9446 = v10652;
                v9447 = v10571;
                v9448 = v9675;
                v9449 = v11448;
                v9450 = v10571;
                v9451 = v11054;
                v9452 = v9713;
                v9453 = v9714;
                v9454 = v11054;
                v9455 = v11054;
                v9456 = v11054;
                v9457 = v9715;
                v9458 = v9716;
                v9459 = v11054;
            }
            let v4301 = if v4300 == v0 { 1.0 } else { 0.0 };
            let v4871: f64;
            let v5520: f64;
            let v5773: f64;
            let v5775: f64;
            let v5784: f64;
            let v8260: f64;
            let v8280: f64;
            let v8283: f64;
            let v8295: f64;
            let v8304: f64;
            let v8363: f64;
            let v8369: f64;
            let v8373: f64;
            let v8403: f64;
            let v8475: f64;
            let v8479: f64;
            let v8483: f64;
            let v8484: f64;
            let v8490: f64;
            let v9113: f64;
            let v9768: Lanes<6>;
            let v9769: Lanes<6>;
            let v9770: Lanes<6>;
            let v9771: Lanes<6>;
            let v9772: Lanes<6>;
            let v9773: Lanes<6>;
            let v9774: Lanes<6>;
            let v9775: Lanes<6>;
            let v9776: Lanes<6>;
            let v9777: Lanes<6>;
            let v9778: Lanes<6>;
            let v9779: Lanes<6>;
            let v9780: Lanes<6>;
            let v9781: Lanes<6>;
            let v9782: Lanes<6>;
            let v9783: Lanes<6>;
            let v9784: Lanes<6>;
            let v9785: Lanes<6>;
            if v4301 != 0.0 {
                let v4312 = v4306 * v4309;
                let v4314 = (v703 * (v8 + v4302)) / v4312;
                let v4315 = v1702 - v4314;
                let v13748 = (((v9429 * v703) - (((v9430 * v4309) + (v9431 * v4306)) * v4314)) / v4312) * v10382;
                let v4317 = if v4315 > v4316 { 1.0 } else { 0.0 };
                let v4319: f64;
                let v9786: Lanes<6>;
                if v4317 != 0.0 {
                    let v4318 = if v65 >= v4 { 1.0 } else { 0.0 };
                    if v4318 != 0.0 {
                    } else {
                    }
                    v4319 = v8;
                    v9786 = v11054;
                } else {
                    v4319 = v4315;
                    v9786 = v13748;
                }
                let v4324 = if v4320 == v0 { 1.0 } else { 0.0 };
                let v4410: f64;
                let v8296: f64;
                let v9787: Lanes<6>;
                let v9788: Lanes<6>;
                if v4324 != 0.0 {
                    let v4330 = if (if v68 < v4325 { 1.0 } else { 0.0 }) != 0.0 && (if v4327 < v4328 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v4408: f64;
                    let v8297: f64;
                    let v9789: Lanes<6>;
                    let v9790: Lanes<6>;
                    if v4330 != 0.0 {
                        let v4336 = v4335 + v861;
                        let v13814 = v9433 + (Lanes([v10553[0], v10553[1], 0.0, 0.0, v10553[2], 0.0]));
                        let v4339 = if v4331 > (v4336 - v4337) { 1.0 } else { 0.0 };
                        let v8298: f64;
                        let v9791: Lanes<6>;
                        if v4339 != 0.0 {
                            let v4341 = v4336 - v4340;
                            v8298 = v4341;
                            v9791 = v13814;
                        } else {
                            v8298 = v4331;
                            v9791 = v9432;
                        }
                        v4408 = v0;
                        v8297 = v8298;
                        v9789 = v11054;
                        v9790 = v9791;
                    } else {
                        if v5 != 0.0 {
                        } else {
                        }
                        let v4342 = v4 / v7;
                        let v4348 = (v4346 * v486) + (v4327 * (v4343 * v4342));
                        let v4349 = v4 / v4348;
                        let v4350 = v118 * v4349;
                        let v13754 = (((((v9434 * v4342) * v4327) * v4349) * v10382) / v4348) * v118;
                        let v4352 = v4 - v4351;
                        let v4356 = (v4351 * (v818 + v4335)) + (v4352 * v4331);
                        let v13759 = (((Lanes([v9402[0], v9402[1], 0.0, 0.0, 0.0, 0.0])) + v9433) * v4351) + (v9432 * v4352);
                        let v4357 = v4335 + v861;
                        let v13761 = v9433 + (Lanes([v10553[0], v10553[1], 0.0, 0.0, v10553[2], 0.0]));
                        let v4360 = if v4356 > (v4357 - v4358) { 1.0 } else { 0.0 };
                        let v4363: f64;
                        let v9792: Lanes<6>;
                        if v4360 != 0.0 {
                            let v4362 = v4357 - v4361;
                            v4363 = v4362;
                            v9792 = v13761;
                        } else {
                            v4363 = v4356;
                            v9792 = v13759;
                        }
                        let v4364 = v4363 - v4331;
                        let v13762 = v9792 - v9432;
                        let v13763 = v13762 * v4364;
                        let v4368 = ((v4364 * v4364) + v4366).sqrt();
                        let v13769 = (v13762 + ((v13763 + v13763) * (v9360 / (v10427 * v4368)))) * v8;
                        let v4372 = (v8 * (v4364 + v4368)) + v4371;
                        let v4373 = if v4372 < v0 { 1.0 } else { 0.0 };
                        let v4389: f64;
                        let v9793: Lanes<6>;
                        if v4373 != 0.0 {
                            v4389 = v0;
                            v9793 = v11054;
                        } else {
                            v4389 = v4372;
                            v9793 = v13769;
                        }
                        let v4374 = v658 * v4343;
                        let v13770 = v10402 * v4343;
                        let v4375 = v4 / v4374;
                        let v4380 = v4376 * v4375;
                        let v13779 = (v9435 * v4375) + ((((((Lanes([0.0, 0.0, v13770[0], 0.0, 0.0, 0.0])) + (v9434 * v658)) * v4375) * v10382) / v4374) * v4376);
                        let v4381 = if v4380 < v660 { 1.0 } else { 0.0 };
                        let v4386: f64;
                        let v9794: Lanes<6>;
                        if v4381 != 0.0 {
                            let v13780 = Lanes([0.0, 0.0, v10407[0], 0.0, 0.0, 0.0]);
                            v4386 = v660;
                            v9794 = v13780;
                        } else {
                            v4386 = v4380;
                            v9794 = v13779;
                        }
                        let v4385 = v4 / v131;
                        let v4388 = v73 * (v486 / v118);
                        let v4390 = v4388 * v4389;
                        let v13782 = v9793 * v4388;
                        let v4395 = (((v73 * v4386) + (v4390 * v4350)) + (v4384 * v4350)) * v4385;
                        let v4396 = v4395 * v4350;
                        let v13792 = (((((v9794 * v73) + ((v13782 * v4350) + (v13754 * v4390))) + (v13754 * v4384)) * v4385) * v4350) + (v13754 * v4395);
                        let v4398 = v85 * (v4390 + v4384);
                        let v4399 = v4398 * v4350;
                        let v13800 = v13792 * v4396;
                        let v4403 = ((v4396 * v4396) + (v4399 * v4350)).sqrt();
                        let v4406 = v8 * ((-v4396) + v4403);
                        let v4407 = v916 * v4406;
                        let v13809 = v10607 * v4406;
                        let v13812 = (Lanes([v13809[0], v13809[1], v13809[2], v13809[3], v13809[4], 0.0])) + ((((v13792 * v10382) + (((v13800 + v13800) + (((((v13782 * v85) * v4350) + (v13754 * v4398)) * v4350) + (v13754 * v4399))) * (v9360 / (v10427 * v4403)))) * v8) * v916);
                        v4408 = v4407;
                        v8297 = v4363;
                        v9789 = v13812;
                        v9790 = v9792;
                    }
                    let v4409 = v4408 * v263;
                    let v13815 = v9789 * v263;
                    v4410 = v4409;
                    v8296 = v8297;
                    v9787 = v13815;
                    v9788 = v9790;
                } else {
                    v4410 = v0;
                    v8296 = v8299;
                    v9787 = v11054;
                    v9788 = v9452;
                }
                let v4411 = v131 - v4410;
                let v13816 = v9787 * v10382;
                let v4412 = v134 - v4410;
                let v4413 = if v4411 < v611 { 1.0 } else { 0.0 };
                let v4520: f64;
                let v9795: Lanes<6>;
                if v4413 != 0.0 {
                    v4520 = v611;
                    v9795 = v11054;
                } else {
                    v4520 = v4411;
                    v9795 = v13816;
                }
                let v4415 = (-v164) * v134;
                let v4422 = v4415 * v4416;
                let v13817 = v9436 * v4415;
                let v4428 = v4415 * v4423;
                let v13818 = v9437 * v4415;
                let v4429 = v4428 * v8;
                let v13819 = v13818 * v8;
                let v8480: f64;
                let v8485: f64;
                let v8491: f64;
                let v9796: Lanes<6>;
                let v9797: Lanes<6>;
                let v9798: Lanes<6>;
                if v148 != 0.0 {
                    let v4430 = v4422 * v8;
                    let v13820 = v13817 * v8;
                    let v4432 = v4422 * v4431;
                    let v13821 = v13817 * v4431;
                    let v4439 = ((v8 * (v4433 + v4434)) * v134) * v164;
                    let v13825 = (((v9438 + v9439) * v8) * v134) * v164;
                    v8480 = v4439;
                    v8485 = v4430;
                    v8491 = v4432;
                    v9796 = v13825;
                    v9797 = v13820;
                    v9798 = v13821;
                } else {
                    v8480 = v8481;
                    v8485 = v8486;
                    v8491 = v8492;
                    v9796 = v9454;
                    v9797 = v9455;
                    v9798 = v9456;
                }
                let v4443 = v818 - v4440;
                let v13827 = (Lanes([v9402[0], v9402[1], 0.0, 0.0, 0.0, 0.0])) - v9440;
                let v4447 = (v73 * (v4443 / v73)) / v4446;
                let v13830 = ((v13827 / v73) * v73) / v4446;
                let v4455 = v4452 + (v4447 * v4453);
                let v4457 = v4451 + (v4447 * v4455);
                let v4459 = v4450 + (v4447 * v4457);
                let v4461 = v4449 + (v4447 * v4459);
                let v4463 = v4448 + (v4447 * v4461);
                let v4465 = v4 + (v4447 * v4463);
                let v4466 = v4446 / v4465;
                let v13849 = ((((v13830 * v4463) + (((v13830 * v4461) + (((v13830 * v4459) + (((v13830 * v4457) + (((v13830 * v4455) + ((v13830 * v4453) * v4447)) * v4447)) * v4447)) * v4447)) * v4447)) * v4466) * v10382) / v4465;
                let v4468 = if v4466 < v4467 { 1.0 } else { 0.0 };
                let v4470: f64;
                let v9799: Lanes<6>;
                if v4468 != 0.0 {
                    v4470 = v4469;
                    v9799 = v11054;
                } else {
                    v4470 = v4466;
                    v9799 = v13849;
                }
                let v4471 = v4335 + v4470;
                let v13850 = v9433 + v9799;
                let v4474 = v4423 / v552;
                let v13852 = v9437 / v552;
                let v4476 = v4475 / v4472;
                let v4478 = v4477 / v4472;
                let v4482 = v4 + ((v4331 - v4335) * v4479);
                let v4486 = ((v4476 * (v4416 / v552)) + (v4478 * v4474)) / v4482;
                let v13860 = ((((v9436 / v552) * v4476) + (v13852 * v4478)) - (((v9432 - v9433) * v4479) * v4486)) / v4482;
                let v13861 = v13860 * v4486;
                let v4490 = ((v4486 * v4486) + v4488).sqrt();
                let v13867 = (v13860 + ((v13861 + v13861) * (v9360 / (v10427 * v4490)))) * v8;
                let v4494 = (v8 * (v4486 + v4490)) + v4493;
                let v4495 = if v4494 < v0 { 1.0 } else { 0.0 };
                let v4496: f64;
                let v9800: Lanes<6>;
                if v4495 != 0.0 {
                    v4496 = v0;
                    v9800 = v11054;
                } else {
                    v4496 = v4494;
                    v9800 = v13867;
                }
                let v4498 = v4497 - v4;
                let v4499 = v4496.powf(v4498);
                let v4500 = v4499 * v4496;
                let v4501 = v179 - v4;
                let v4502 = v4496.powf(v4501);
                let v4510 = v4505 + ((v4506 * (v4474 / v202)) / v4508);
                let v4511 = v4 / v4510;
                let v13888 = v10413 * v4500;
                let v4516 = (v4511 + (v697 * v4500)) + ((v4502 * v4496) / v4514);
                let v4517 = v4 / v4516;
                let v4518 = v4517 * v24;
                let v13898 = (((((((((((v13852 / v202) * v4506) / v4508) * v4511) * v10382) / v4510) + ((Lanes([0.0, 0.0, v13888[0], 0.0, 0.0, 0.0])) + ((((v9800 * (v4498 * (v4496.powf((v4498 - v9360))))) * v4496) + (v9800 * v4499)) * v697))) + ((((v9800 * (v4501 * (v4496.powf((v4501 - v9360))))) * v4496) + (v9800 * v4502)) / v4514)) * v4517) * v10382) / v4516) * v24;
                let v4519 = v658 * v4343;
                let v13899 = v10402 * v4343;
                let v4521 = v4519 * v4520;
                let v13905 = (((Lanes([0.0, 0.0, v13899[0], 0.0, 0.0, 0.0])) + (v9434 * v658)) * v4520) + (v9795 * v4519);
                let v13906 = v13905 * v4521;
                let v4525 = ((v4521 * v4521) + v4523).sqrt();
                let v13912 = (v13905 + ((v13906 + v13906) * (v9360 / (v10427 * v4525)))) * v8;
                let v4529 = (v8 * (v4521 + v4525)) + v4528;
                let v4530 = if v4529 < v0 { 1.0 } else { 0.0 };
                let v4531: f64;
                let v9801: Lanes<6>;
                if v4530 != 0.0 {
                    v4531 = v0;
                    v9801 = v11054;
                } else {
                    v4531 = v4529;
                    v9801 = v13912;
                }
                let v4532 = v4 / v4531;
                let v4533 = v4376 * v4532;
                let v13919 = v10426 * v1884;
                let v4535 = (v1884 * v712) / v4518;
                let v13924 = ((v9435 * v4532) + ((((v9801 * v4532) * v10382) / v4531) * v4376)) * v4533;
                let v13926 = (((Lanes([0.0, 0.0, v13919[0], 0.0, 0.0, 0.0])) - (v13898 * v4535)) / v4518) * v4535;
                let v4539 = ((v4533 * v4533) + (v4535 * v4535)).sqrt();
                let v13931 = ((v13924 + v13924) + (v13926 + v13926)) * (v9360 / (v10427 * v4539));
                let v4541 = (v4518 * v4539) / v712;
                let v13935 = v10426 * v4541;
                let v13938 = (((v13898 * v4539) + (v13931 * v4518)) - (Lanes([0.0, 0.0, v13935[0], 0.0, 0.0, 0.0]))) / v712;
                let v4547 = if (if v4542 <= v4543 { 1.0 } else { 0.0 }) != 0.0 && (if v4543 <= v4545 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v4555: f64;
                let v9802: Lanes<6>;
                if v4547 != 0.0 {
                    v4555 = v4;
                    v9802 = v11054;
                } else {
                    let v4552 = if (if v4548 <= v4543 { 1.0 } else { 0.0 }) != 0.0 && (if v4543 <= v4550 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v4556: f64;
                    let v9803: Lanes<6>;
                    if v4552 != 0.0 {
                        v4556 = v4541;
                        v9803 = v13938;
                    } else {
                        let v4553 = v4543 - v4;
                        let v4554 = v4541.powf(v4553);
                        let v13942 = v13938 * (v4553 * (v4541.powf((v4553 - v9360))));
                        v4556 = v4554;
                        v9803 = v13942;
                    }
                    v4555 = v4556;
                    v9802 = v9803;
                }
                let v13945 = (v13938 * v4555) + (v9802 * v4541);
                let v4558 = v4 + (v4541 * v4555);
                let v4563 = if (if v4559 <= v4543 { 1.0 } else { 0.0 }) != 0.0 && (if v4543 <= v4561 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v4577: f64;
                let v9804: Lanes<6>;
                if v4563 != 0.0 {
                    let v4564 = v4 / v4558;
                    let v13961 = ((v13945 * v4564) * v10382) / v4558;
                    v4577 = v4564;
                    v9804 = v13961;
                } else {
                    let v4569 = if (if v4565 <= v4543 { 1.0 } else { 0.0 }) != 0.0 && (if v4543 <= v4567 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v4578: f64;
                    let v9805: Lanes<6>;
                    if v4569 != 0.0 {
                        let v4570 = v4558.sqrt();
                        let v4571 = v4 / v4570;
                        let v13958 = (((v13945 * (v9360 / (v10427 * v4570))) * v4571) * v10382) / v4570;
                        v4578 = v4571;
                        v9805 = v13958;
                    } else {
                        let v4574 = (v4572 / v4543) - v4;
                        let v4575 = v4558.powf(v4574);
                        let v4576 = v4558 * v4575;
                        let v13952 = (v13945 * v4575) + ((v13945 * (v4574 * (v4558.powf((v4574 - v9360))))) * v4558);
                        v4578 = v4576;
                        v9805 = v13952;
                    }
                    v4577 = v4578;
                    v9804 = v9805;
                }
                let v4579 = v4518 * v4577;
                let v13964 = (v13898 * v4577) + (v9804 * v4518);
                let v13965 = v10407 * v162;
                let v4581 = (v162 * v660) / v4411;
                let v13969 = ((Lanes([0.0, 0.0, v13965[0], 0.0, 0.0, 0.0])) - (v13816 * v4581)) / v4411;
                let v4582 = v4581 * v4376;
                let v4583 = v4582 * v4579;
                let v13975 = (((v13969 * v4376) + (v9435 * v4581)) * v4579) + (v13964 * v4582);
                let v4587 = if (if v4584 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v208 != v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v4643: f64;
                let v9806: Lanes<6>;
                if v4587 != 0.0 {
                    let v4590 = (v73 * (v8 * v4443)) / v15;
                    let v13978 = ((v13827 * v8) * v73) / v15;
                    let v4598 = v4595 + (v4590 * v4596);
                    let v4600 = v4594 + (v4590 * v4598);
                    let v4602 = v4593 + (v4590 * v4600);
                    let v4604 = v4592 + (v4590 * v4602);
                    let v4606 = v4591 + (v4590 * v4604);
                    let v4608 = v4 + (v4590 * v4606);
                    let v4609 = v15 / v4608;
                    let v4611 = v4335 + v4609;
                    let v13998 = v9433 + (((((v13978 * v4606) + (((v13978 * v4604) + (((v13978 * v4602) + (((v13978 * v4600) + (((v13978 * v4598) + ((v13978 * v4596) * v4590)) * v4590)) * v4590)) * v4590)) * v4590)) * v4609) * v10382) / v4608);
                    let v4612 = v4610 - v4611;
                    let v13999 = v13998 * v10382;
                    let v14000 = v13999 * v4612;
                    let v4616 = ((v4612 * v4612) + v4614).sqrt();
                    let v14006 = (v13999 + ((v14000 + v14000) * (v9360 / (v10427 * v4616)))) * v8;
                    let v4620 = (v8 * (v4612 + v4616)) + v4619;
                    let v4621 = if v4620 < v0 { 1.0 } else { 0.0 };
                    let v4624: f64;
                    let v9807: Lanes<6>;
                    if v4621 != 0.0 {
                        v4624 = v0;
                        v9807 = v11054;
                    } else {
                        v4624 = v4620;
                        v9807 = v14006;
                    }
                    let v4622 = v658 * v212;
                    let v4623 = v1123 * v4622;
                    let v14008 = v9413 * v4622;
                    let v14009 = (v10402 * v212) * v1123;
                    let v4626 = v4624.powf(v4625);
                    let v4627 = v4623 * v4626;
                    let v14017 = ((Lanes([v14008[0], v14008[1], 0.0, v14008[2], v14008[3]])) + (Lanes([0.0, 0.0, v14009[0], 0.0, 0.0]))) * v4626;
                    let v14020 = (Lanes([v14017[0], v14017[1], v14017[2], v14017[3], v14017[4], 0.0])) + ((v9807 * (v4625 * (v4624.powf((v4625 - v9360))))) * v4623);
                    let v14021 = v10553 * v4628;
                    let v4630 = v4 + (v861 * v4628);
                    let v4635: f64;
                    let v9808: Lanes<6>;
                    if v982 != 0.0 {
                        let v4631 = v4611 - v859;
                        let v14024 = v13998 - (Lanes([v10550[0], v10550[1], 0.0, 0.0, v10550[2], 0.0]));
                        v4635 = v4631;
                        v9808 = v14024;
                    } else {
                        let v4633 = v4611 - v4632;
                        let v14022 = v13998 - v9441;
                        v4635 = v4633;
                        v9808 = v14022;
                    }
                    let v4634 = v861 * v217;
                    let v14026 = (v10553 * v217) * v4635;
                    let v4637 = v4630 + (v4634 * v4635);
                    let v4638 = v4627 * v4637;
                    let v14034 = (v14020 * v4637) + (((Lanes([v14021[0], v14021[1], 0.0, 0.0, v14021[2], 0.0])) + ((Lanes([v14026[0], v14026[1], 0.0, 0.0, v14026[2], 0.0])) + (v9808 * v4634))) * v4627);
                    v4643 = v4638;
                    v9806 = v14034;
                } else {
                    v4643 = v0;
                    v9806 = v11054;
                }
                let v4639 = if v218 != v0 { 1.0 } else { 0.0 };
                let v4644: f64;
                let v9809: Lanes<5>;
                if v4639 != 0.0 {
                    let v4640 = v658 * v223;
                    let v4641 = v1123 * v4640;
                    let v14036 = v9413 * v4640;
                    let v14037 = (v10402 * v223) * v1123;
                    let v4642 = v4641 * v861;
                    let v14042 = v10553 * v4641;
                    let v14044 = (((Lanes([v14036[0], v14036[1], 0.0, v14036[2], v14036[3]])) + (Lanes([0.0, 0.0, v14037[0], 0.0, 0.0]))) * v861) + (Lanes([v14042[0], v14042[1], 0.0, 0.0, v14042[2]]));
                    v4644 = v4642;
                    v9809 = v14044;
                } else {
                    v4644 = v0;
                    v9809 = v10571;
                }
                let v4645 = v4643 + v4644;
                let v14046 = v9806 + (Lanes([v9809[0], v9809[1], v9809[2], v9809[3], v9809[4], 0.0]));
                let v4646 = if v4645 > v0 { 1.0 } else { 0.0 };
                let v4650: f64;
                let v9810: Lanes<6>;
                if v4646 != 0.0 {
                    let v4647 = v4440 * v4645;
                    let v4648 = v4581 * v4647;
                    let v4649 = v4648 * v4579;
                    let v14055 = (((v13969 * v4647) + (((v9440 * v4645) + (v14046 * v4440)) * v4581)) * v4579) + (v13964 * v4648);
                    v4650 = v4649;
                    v9810 = v14055;
                } else {
                    v4650 = v0;
                    v9810 = v11054;
                }
                let v4651 = v4583 + v4650;
                let v14056 = v13975 + v9810;
                let v4653 = if v4652 != v0 { 1.0 } else { 0.0 };
                let v4872: f64;
                let v9811: Lanes<6>;
                if v4653 != 0.0 {
                    let v4654 = v241 - v1097;
                    let v4656 = v4 / (v4654 * v4654);
                    let v4657 = v73 * v1096;
                    let v4661 = ((v4657 * (v118 * v1043)) * v512) * v4656;
                    let v4662 = v4661 * v1061;
                    let v14061 = ((((v9412 * v118) * v4657) * v512) * v4656) * v1061;
                    let v14062 = v10717 * v4661;
                    let v4666 = v4663 + (v4664 * v861);
                    let v4667 = v4662 * v4666;
                    let v14068 = (v10553 * v4664) * v4662;
                    let v14070 = (((Lanes([v14061[0], v14061[1], 0.0, v14061[2], v14061[3]])) + (Lanes([v14062[0], v14062[1], v14062[2], 0.0, v14062[3]]))) * v4666) + (Lanes([v14068[0], v14068[1], 0.0, 0.0, v14068[2]]));
                    let v14072 = (v9402 * v4669) * v10382;
                    let v14074 = v10556 + (Lanes([v14072[0], v14072[1], 0.0, 0.0]));
                    let v4674 = ((v862 - v236) + (v4668 - (v4669 * v818))) + v4667;
                    let v14076 = (Lanes([v14074[0], v14074[1], 0.0, v14074[2], v14074[3]])) + v14070;
                    let v4675 = v729 * v1043;
                    let v14077 = v10451 * v1043;
                    let v14078 = v9412 * v729;
                    let v4676 = v4675 * v1043;
                    let v14083 = v9412 * v4675;
                    let v14085 = (((Lanes([0.0, 0.0, v14077[0], 0.0, 0.0])) + (Lanes([v14078[0], v14078[1], 0.0, v14078[2], v14078[3]]))) * v1043) + (Lanes([v14083[0], v14083[1], 0.0, v14083[2], v14083[3]]));
                    let v14087 = v10402 * v4676;
                    let v4678 = (v4676 * v658) * v8;
                    let v14090 = ((v14085 * v658) + (Lanes([0.0, 0.0, v14087[0], 0.0, 0.0]))) * v8;
                    let v14092 = v10402 * v4678;
                    let v4680 = (v4678 * v658) * v73;
                    let v14095 = ((v14090 * v658) + (Lanes([0.0, 0.0, v14092[0], 0.0, 0.0]))) * v73;
                    let v4681 = v658 * v2045;
                    let v14098 = (v10402 * v2045) * v4676;
                    let v14103 = ((Lanes([0.0, 0.0, v10407[0], 0.0, 0.0])) - ((v14085 * v4681) + (Lanes([0.0, 0.0, v14098[0], 0.0, 0.0])))) - v14070;
                    let v4687 = ((((v660 - (v4676 * v4681)) + v236) - v4668) - v4667) + v358;
                    let v14105 = (Lanes([v10556[0], v10556[1], 0.0, v10556[2], v10556[3]])) - v14103;
                    let v4689 = (v862 - v4687) - v3679;
                    let v4690 = if v4687 >= v0 { 1.0 } else { 0.0 };
                    let v4692: f64;
                    if v4690 != 0.0 {
                        v4692 = v4;
                    } else {
                        v4692 = v4691;
                    }
                    let v14106 = v14105 * v4689;
                    let v4694 = v4692 * v85;
                    let v4698 = ((v4689 * v4689) + ((v4694 * v4687) * v3679)).sqrt();
                    let v4705 = ((((v4687 + (v8 * (v4689 + v4698))) - v236) + v4668) + v4667) - v983;
                    let v14118 = Lanes([v9408[0], v9408[1], 0.0, 0.0, v9408[2]]);
                    let v14120 = v10402 * v4705;
                    let v4707 = (v658 * v4705) - v4;
                    let v4708 = v85 / v4680;
                    let v14129 = (((Lanes([0.0, 0.0, v14120[0], 0.0, 0.0])) + ((((v14103 + ((v14105 + (((v14106 + v14106) + ((v14103 * v4694) * v3679)) * (v9360 / (v10427 * v4698)))) * v8)) + v14070) - v14118) * v658)) * v4708) + ((((v14095 * v4708) * v10382) / v4680) * v4707);
                    let v4710 = v4 + (v4707 * v4708);
                    let v14130 = v14129 * v4710;
                    let v4714 = ((v4710 * v4710) + v4712).sqrt();
                    let v14136 = (v14129 + ((v14130 + v14130) * (v9360 / (v10427 * v4714)))) * v8;
                    let v4718 = (v8 * (v4710 + v4714)) + v4717;
                    let v4719 = if v4718 < v0 { 1.0 } else { 0.0 };
                    let v4720: f64;
                    let v9812: Lanes<5>;
                    if v4719 != 0.0 {
                        v4720 = v0;
                        v9812 = v10571;
                    } else {
                        v4720 = v4718;
                        v9812 = v14136;
                    }
                    let v4722 = (v4720 + v358).sqrt();
                    let v4723 = v4 - v4722;
                    let v4725 = v4674 + (v4678 * v4723);
                    let v14144 = v14076 + ((v14090 * v4723) + (((v9812 * (v9360 / (v10427 * v4722))) * v10382) * v4678));
                    let v4726 = v4674 + v358;
                    let v4727 = v73 / v4726;
                    let v4728 = v658 + v4727;
                    let v4729 = v4 / v4728;
                    let v4732 = v4 / v4730;
                    let v14155 = ((v9442 * v4732) * v10382) / v4730;
                    let v4733 = v4732 / v4676;
                    let v4734 = v4674 * v4674;
                    let v14160 = v14076 * v4674;
                    let v4735 = v4733 * v4734;
                    let v4736 = v4735.ln();
                    let v4737 = v4736 * v4729;
                    let v14169 = (((((((Lanes([0.0, 0.0, v14155[0], 0.0, 0.0])) - (v14085 * v4733)) / v4676) * v4734) + ((v14160 + v14160) * v4733)) * (v9360 / v4735)) * v4729) + ((((((Lanes([0.0, 0.0, v10402[0], 0.0, 0.0])) + (((v14076 * v4727) * v10382) / v4726)) * v4729) * v10382) / v4728) * v4736);
                    let v14170 = v14169 - v14144;
                    let v4740 = (v4737 - v4725) - v4739;
                    let v14171 = v14170 * v4740;
                    let v4745 = ((v4740 * v4740) + (v4742 * v4737)).sqrt();
                    let v4748 = v4737 - (v8 * (v4740 + v4745));
                    let v14180 = v14169 - ((v14170 + (((v14171 + v14171) + (v14169 * v4742)) * (v9360 / (v10427 * v4745)))) * v8);
                    let v14181 = v10402 * v4748;
                    let v4750 = (v658 * v4748).exp();
                    let v14186 = v9442 * v4750;
                    let v4752 = v4748 - v983;
                    let v14191 = v10402 * v4752;
                    let v14194 = (Lanes([0.0, 0.0, v14191[0], 0.0, 0.0])) + ((v14180 - v14118) * v658);
                    let v4754 = (v658 * v4752) - v4;
                    let v4755 = v4754 + (v4730 * v4750);
                    let v14195 = v14194 + ((Lanes([0.0, 0.0, v14186[0], 0.0, 0.0])) + ((((Lanes([0.0, 0.0, v14181[0], 0.0, 0.0])) + (v14180 * v658)) * v4750) * v4730));
                    let v14196 = v14195 * v4755;
                    let v4759 = ((v4755 * v4755) + v4757).sqrt();
                    let v14202 = (v14195 + ((v14196 + v14196) * (v9360 / (v10427 * v4759)))) * v8;
                    let v4763 = (v8 * (v4755 + v4759)) + v4762;
                    let v4764 = if v4763 < v0 { 1.0 } else { 0.0 };
                    let v4765: f64;
                    let v9813: Lanes<5>;
                    if v4764 != 0.0 {
                        v4765 = v0;
                        v9813 = v10571;
                    } else {
                        v4765 = v4763;
                        v9813 = v14202;
                    }
                    let v4768 = (v4765 + v4766).sqrt();
                    let v14205 = v9813 * (v9360 / (v10427 * v4768));
                    let v14206 = v14194 * v4754;
                    let v4772 = ((v4754 * v4754) + v4770).sqrt();
                    let v14212 = (v14194 + ((v14206 + v14206) * (v9360 / (v10427 * v4772)))) * v8;
                    let v4776 = (v8 * (v4754 + v4772)) + v4775;
                    let v4777 = if v4776 < v0 { 1.0 } else { 0.0 };
                    let v4778: f64;
                    let v9814: Lanes<5>;
                    if v4777 != 0.0 {
                        v4778 = v0;
                        v9814 = v10571;
                    } else {
                        v4778 = v4776;
                        v9814 = v14212;
                    }
                    let v4781 = (v4778 + v4779).sqrt();
                    let v4784 = v4768 - v4781;
                    let v4785 = v4782 * v4784;
                    let v14217 = v9443 * v4784;
                    let v14220 = (Lanes([0.0, 0.0, v14217[0], 0.0, 0.0])) + ((v14205 - (v9814 * (v9360 / (v10427 * v4781)))) * v4782);
                    let v4786 = v4725 - v4748;
                    let v14221 = v14144 - v14180;
                    let v14222 = v14221 * v4786;
                    let v4790 = ((v4786 * v4786) + v4788).sqrt();
                    let v14228 = (v14221 + ((v14222 + v14222) * (v9360 / (v10427 * v4790)))) * v8;
                    let v4794 = (v8 * (v4786 + v4790)) + v4793;
                    let v4795 = if v4794 < v0 { 1.0 } else { 0.0 };
                    let v4796: f64;
                    let v9815: Lanes<5>;
                    if v4795 != 0.0 {
                        v4796 = v0;
                        v9815 = v10571;
                    } else {
                        v4796 = v4794;
                        v9815 = v14228;
                    }
                    let v4798 = v4796 + v4797;
                    let v4799 = v818 / v4798;
                    let v14231 = (v10589 - (v9815 * v4799)) / v4798;
                    let v4800 = v4799 * v4799;
                    let v14232 = v14231 * v4799;
                    let v14233 = v14232 + v14232;
                    let v4801 = v4800 * v4800;
                    let v14234 = v14233 * v4800;
                    let v4802 = v4801 * v4800;
                    let v14241 = ((((v14234 + v14234) * v4800) + (v14233 * v4801)) * v4800) + (v14233 * v4802);
                    let v4805 = (v4802 * v4800) + v4804;
                    let v4822: f64;
                    let v9816: Lanes<5>;
                    if v4806 != 0.0 {
                        let v4816: f64;
                        if v4807 != 0.0 {
                            v4816 = v4;
                        } else {
                            let v4817: f64;
                            if v4808 != 0.0 {
                                v4817 = v73;
                            } else {
                                let v4818: f64;
                                if v4809 != 0.0 {
                                    v4818 = v91;
                                } else {
                                    let v4819: f64;
                                    if v4810 != 0.0 {
                                        v4819 = v85;
                                    } else {
                                        v4819 = v0;
                                    }
                                    v4818 = v4819;
                                }
                                v4817 = v4818;
                            }
                            v4816 = v4817;
                        }
                        let mut v4811: f64 = 0.0;
                        let mut v4813: f64 = 0.0;
                        let mut v9817: Lanes<5> = Lanes([0.0; 5]);
                        v4811 = v0;
                        v4813 = v4805;
                        v9817 = v14241;
                        loop {
                            let v4812 = if v4811 < v4816 { 1.0 } else { 0.0 };
                            if v4812 == 0.0 {
                                break;
                            }
                            let v4814 = v4813.sqrt();
                            let v18853 = v9817 * (v9360 / (v10427 * v4814));
                            let v4815 = v4811 + v4;
                            v4811 = v4815;
                            v4813 = v4814;
                            v9817 = v18853;
                        }
                        v4822 = v4813;
                        v9816 = v9817;
                    } else {
                        let v4821 = v4805.powf(v4820);
                        let v14245 = v14241 * (v4820 * (v4805.powf(v14242)));
                        v4822 = v4821;
                        v9816 = v14245;
                    }
                    let v4823 = v4 / v4822;
                    let v4824 = v4799 * v4823;
                    let v4826 = (v73 * v258) * v140;
                    let v4827 = v4826 * v660;
                    let v4828 = v4827 * v4579;
                    let v14253 = (v10407 * v4826) * v4579;
                    let v4829 = v4828 * v4785;
                    let v14258 = v14220 * v4828;
                    let v14262 = ((v14231 * v4823) + ((((v9816 * v4823) * v10382) / v4822) * v4799)) * v4829;
                    let v4831 = (v4829 * v4824) / v4520;
                    let v4832 = v4651 + v4831;
                    let v14268 = v14056 + ((((((((Lanes([0.0, 0.0, v14253[0], 0.0, 0.0, 0.0])) + (v13964 * v4827)) * v4785) + (Lanes([v14258[0], v14258[1], v14258[2], v14258[3], v14258[4], 0.0]))) * v4824) + (Lanes([v14262[0], v14262[1], v14262[2], v14262[3], v14262[4], 0.0]))) - (v9795 * v4831)) / v4520);
                    v4872 = v4832;
                    v9811 = v14268;
                } else {
                    v4872 = v4651;
                    v9811 = v14056;
                }
                let v4837 = if (if v4833 != v0 { 1.0 } else { 0.0 }) != 0.0 && (if v4835 != v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8364: f64;
                let v8370: f64;
                let v8374: f64;
                let v8404: f64;
                let v9818: Lanes<6>;
                let v9819: Lanes<6>;
                let v9820: Lanes<6>;
                if v4837 != 0.0 {
                    let v4840 = v4838 * v4838;
                    let v14269 = v9444 * v4838;
                    let v14270 = v14269 + v14269;
                    let v4841 = v73 * v660;
                    let v4842 = v4841 * v1043;
                    let v14272 = (v10407 * v73) * v1043;
                    let v14273 = v9412 * v4841;
                    let v14277 = ((Lanes([0.0, 0.0, v14272[0], 0.0, 0.0])) + (Lanes([v14273[0], v14273[1], 0.0, v14273[2], v14273[3]]))) * v4376;
                    let v4844 = v4840 - (v4842 * v4376);
                    let v14281 = v14270 - ((Lanes([v14277[0], v14277[1], v14277[2], v14277[3], v14277[4], 0.0])) + (v9435 * v4842));
                    let v14282 = v14270 * v4840;
                    let v4848 = ((v4840 * v4840) + v4846).sqrt();
                    let v14288 = (v14270 + ((v14282 + v14282) * (v9360 / (v10427 * v4848)))) * v8;
                    let v4852 = (v8 * (v4840 + v4848)) + v4851;
                    let v4853 = if v4852 < v0 { 1.0 } else { 0.0 };
                    let v4863: f64;
                    let v9821: Lanes<6>;
                    if v4853 != 0.0 {
                        v4863 = v0;
                        v9821 = v11054;
                    } else {
                        v4863 = v4852;
                        v9821 = v14288;
                    }
                    let v14289 = v14281 * v4844;
                    let v4857 = ((v4844 * v4844) + v4855).sqrt();
                    let v14295 = (v14281 + ((v14289 + v14289) * (v9360 / (v10427 * v4857)))) * v8;
                    let v4861 = (v8 * (v4844 + v4857)) + v4860;
                    let v4862 = if v4861 < v0 { 1.0 } else { 0.0 };
                    let v4864: f64;
                    let v9822: Lanes<6>;
                    if v4862 != 0.0 {
                        v4864 = v0;
                        v9822 = v11054;
                    } else {
                        v4864 = v4861;
                        v9822 = v14295;
                    }
                    let v4865 = v4863 - v4864;
                    let v14296 = v9821 - v9822;
                    let v4870 = if (if v4343 < v4866 { 1.0 } else { 0.0 }) != 0.0 || (if v4865 < v4868 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v8365: f64;
                    if v4870 != 0.0 {
                        v8365 = v0;
                    } else {
                        v8365 = v4;
                    }
                    v8364 = v8365;
                    v8370 = v4864;
                    v8374 = v4863;
                    v8404 = v4865;
                    v9818 = v9822;
                    v9819 = v9821;
                    v9820 = v14296;
                } else {
                    v8364 = v0;
                    v8370 = v0;
                    v8374 = v0;
                    v8404 = v0;
                    v9818 = v11054;
                    v9819 = v11054;
                    v9820 = v11054;
                }
                v4871 = v4872;
                v5520 = v4471;
                v5773 = v4581;
                v5775 = v4579;
                v5784 = v4539;
                v8260 = v4520;
                v8280 = v4428;
                v8283 = v4412;
                v8295 = v8296;
                v8304 = v4518;
                v8363 = v8364;
                v8369 = v8370;
                v8373 = v8374;
                v8403 = v8404;
                v8475 = v4422;
                v8479 = v8480;
                v8483 = v4429;
                v8484 = v8485;
                v8490 = v8491;
                v9113 = v4319;
                v9768 = v9811;
                v9769 = v13850;
                v9770 = v13969;
                v9771 = v13964;
                v9772 = v13931;
                v9773 = v9795;
                v9774 = v13818;
                v9775 = v9788;
                v9776 = v13898;
                v9777 = v9818;
                v9778 = v9819;
                v9779 = v9820;
                v9780 = v13817;
                v9781 = v9796;
                v9782 = v13819;
                v9783 = v9797;
                v9784 = v9798;
                v9785 = v9786;
            } else {
                v4871 = v0;
                v5520 = v4;
                v5773 = v4;
                v5775 = v5776;
                v5784 = v0;
                v8260 = v131;
                v8280 = v0;
                v8283 = v0;
                v8295 = v8299;
                v8304 = v0;
                v8363 = v0;
                v8369 = v0;
                v8373 = v0;
                v8403 = v0;
                v8475 = v8476;
                v8479 = v8481;
                v8483 = v0;
                v8484 = v8486;
                v8490 = v8492;
                v9113 = v8;
                v9768 = v11054;
                v9769 = v11054;
                v9770 = v11054;
                v9771 = v11054;
                v9772 = v11054;
                v9773 = v11054;
                v9774 = v11054;
                v9775 = v9452;
                v9776 = v11054;
                v9777 = v11054;
                v9778 = v11054;
                v9779 = v11054;
                v9780 = v9453;
                v9781 = v9454;
                v9782 = v11054;
                v9783 = v9455;
                v9784 = v9456;
                v9785 = v11054;
            }
            let v4876 = if (if v4584 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v4874 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v5615: f64;
            let v6019: f64;
            let v9823: Lanes<6>;
            let v9824: Lanes<6>;
            if v4876 != 0.0 {
                let v4878 = v1195 - v4877;
                let v4879 = v1138 + v4877;
                let v4881 = v36 / v726;
                let v4883 = (v4881 * v485) / v726;
                let v4884 = v4883.ln();
                let v4885 = v660 * v4884;
                let v14308 = (v10407 * v4884) + ((((((((v10445 * v4881) * v10382) / v726) * v485) - (v10445 * v4883)) / v726) * (v9360 / v4883)) * v660);
                let v4886: f64;
                let v9825: Lanes<6>;
                if v5 != 0.0 {
                    let v14309 = Lanes([v9417[0], v9417[1], v9417[2], 0.0, v9417[3], 0.0]);
                    v4886 = v1032;
                    v9825 = v14309;
                } else {
                    v4886 = v4632;
                    v9825 = v9441;
                }
                let v4893 = v485 + v36;
                let v4895 = (((((v4887 * (v4885 - v4886)) / v118) * v485) * v36) / v4893).sqrt();
                let v4896 = v4895 * v137;
                let v14320 = ((((((((Lanes([0.0, 0.0, v14308[0], 0.0, 0.0, 0.0])) - v9825) * v4887) / v118) * v485) * v36) / v4893) * (v9360 / (v10427 * v4895))) * v137;
                let v4898 = v4897 * v4896;
                let v4900 = v818 + v4896;
                let v14325 = Lanes([v9402[0], v9402[1], 0.0, 0.0, 0.0, 0.0]);
                let v4901 = (v4898 * v4896) / v4900;
                let v14329 = ((((v14320 * v4897) * v4896) + (v14320 * v4898)) - ((v14325 + v14320) * v4901)) / v4900;
                let v4902 = v4878 - v4901;
                let v14330 = Lanes([v10820[0], v10820[1], v10820[2], v10820[3], v10820[4], 0.0]);
                let v4903 = v658 * v4902;
                let v14332 = v10402 * v4902;
                let v14335 = (Lanes([0.0, 0.0, v14332[0], 0.0, 0.0, 0.0])) + ((v14330 - v14329) * v658);
                let v4906 = v1202 * v659;
                let v14338 = v10404 * v1202;
                let v4907 = (v85 * (v4903 - v4)) / v4906;
                let v14341 = ((v10828 * v659) + (Lanes([0.0, 0.0, v14338[0], 0.0, 0.0]))) * v4907;
                let v14344 = ((v14335 * v85) - (Lanes([v14341[0], v14341[1], v14341[2], v14341[3], v14341[4], 0.0]))) / v4906;
                let v4908 = v4 + v4907;
                let v4910 = if v4908 >= v4909 { 1.0 } else { 0.0 };
                let v4912: f64;
                let v9826: Lanes<6>;
                if v4910 != 0.0 {
                    v4912 = v4908;
                    v9826 = v14344;
                } else {
                    v4912 = v4911;
                    v9826 = v11054;
                }
                let v14346 = v10402 * v1202;
                let v4914 = (v1202 * v658) * v8;
                let v4915 = v4912.sqrt();
                let v4916 = v4 - v4915;
                let v14354 = (((v10828 * v658) + (Lanes([0.0, 0.0, v14346[0], 0.0, 0.0]))) * v8) * v4916;
                let v4918 = v4878 + (v4914 * v4916);
                let v14358 = v14330 + ((Lanes([v14354[0], v14354[1], v14354[2], v14354[3], v14354[4], 0.0])) + (((v9826 * (v9360 / (v10427 * v4915))) * v10382) * v4914));
                let v4921 = if v825 < ((v236 + v4879) * v8) { 1.0 } else { 0.0 };
                if v4921 != 0.0 {
                } else {
                }
                let v5081: f64;
                let v5093: f64;
                let v9827: Lanes<6>;
                if v4922 != 0.0 {
                    let v4925 = if (v658 * (v4918 - v4901)) < v91 { 1.0 } else { 0.0 };
                    let v5086: f64;
                    let v5096: f64;
                    let v9828: Lanes<6>;
                    if v4925 != 0.0 {
                        let v4927 = v4926 * v658;
                        let v4928 = v4927 * v1201;
                        let v14421 = (v10402 * v4926) * v1201;
                        let v4929 = v4 / v4928;
                        let v14427 = ((((Lanes([0.0, 0.0, v14421[0], 0.0, 0.0])) + (v10826 * v4927)) * v4929) * v10382) / v4928;
                        let v14428 = v14427 * v91;
                        let v4931 = v1535 + (v91 * v4929);
                        let v14430 = (v14427 * v1535) * v10382;
                        let v4935 = v1148 * v4929;
                        let v4936 = v4935 * v4903;
                        let v14432 = (v14427 * v1148) * v4903;
                        let v14437 = (Lanes([v14430[0], v14430[1], v14430[2], v14430[3], v14430[4], 0.0])) + ((Lanes([v14432[0], v14432[1], v14432[2], v14432[3], v14432[4], 0.0])) + (v14335 * v4935));
                        let v4941 = (v1544 - (v1535 * (v1545 + v4929))) + v4936;
                        let v14438 = v14437 * v4941;
                        let v4943 = v85 * v4931;
                        let v4944 = v4943 * v4931;
                        let v14446 = ((((v14428 * v85) * v4931) + (v14428 * v4943)) * v4931) + (v14428 * v4944);
                        let v4947 = ((v4944 * v4931) + (v4941 * v4941)).sqrt();
                        let v4948 = ((v4932 - (v1535 * v4929)) + v4936) + v4947;
                        let v4949 = v4948.powf(v1557);
                        let v14456 = (v14437 + (((Lanes([v14446[0], v14446[1], v14446[2], v14446[3], v14446[4], 0.0])) + (v14438 + v14438)) * (v9360 / (v10427 * v4947)))) * (v1557 * (v4948.powf(v14453)));
                        let v14457 = v14428 * v1559;
                        let v4951 = v91 * v4949;
                        let v4952 = (v1559 * v4931) / v4951;
                        let v4956 = (v91 - v4952) + (v4954 * v4949);
                        let v14467 = v10407 * v4956;
                        let v4958 = (v4956 * v660) + v4901;
                        let v14470 = (((((((Lanes([v14457[0], v14457[1], v14457[2], v14457[3], v14457[4], 0.0])) - ((v14456 * v91) * v4952)) / v4951) * v10382) + (v14456 * v4954)) * v660) + (Lanes([0.0, 0.0, v14467[0], 0.0, 0.0, 0.0]))) + v14329;
                        v5086 = v4958;
                        v5096 = v4958;
                        v9828 = v14470;
                    } else {
                        let v4961 = if (v825 - v4959) <= v4879 { 1.0 } else { 0.0 };
                        let v5087: f64;
                        let v5097: f64;
                        let v9829: Lanes<6>;
                        if v4961 != 0.0 {
                            let v4979: f64;
                            let v9830: Lanes<6>;
                            if v148 != 0.0 {
                                let v4962 = v4 / v1123;
                                let v4963 = v7 / v118;
                                let v4964 = v4 / v125;
                                let v4966 = (v4962 + v4963) + v4964;
                                let v4967 = v4 / v4966;
                                let v4971 = v4964 + (v8 * v4963);
                                let v4975 = (v4878 - v4968) + (v4971 * (-v4972));
                                let v14410 = ((((((v9413 * v4962) * v10382) / v1123) * v4967) * v10382) / v4966) * v4975;
                                let v4977 = (v4967 * v4975) / v1123;
                                let v14414 = v9413 * v4977;
                                let v4978 = v4878 - v4977;
                                let v14418 = v10820 - ((((Lanes([v14410[0], v14410[1], 0.0, v14410[2], v14410[3]])) + (((v10820 - (Lanes([v9446[0], v9446[1], v9446[2], 0.0, v9446[3]]))) + ((v9447 * v10382) * v4971)) * v4967)) - (Lanes([v14414[0], v14414[1], 0.0, v14414[2], v14414[3]]))) / v1123);
                                let v14419 = Lanes([v14418[0], v14418[1], v14418[2], v14418[3], v14418[4], 0.0]);
                                v4979 = v4978;
                                v9830 = v14419;
                            } else {
                                v4979 = v4918;
                                v9830 = v14358;
                            }
                            v5087 = v4979;
                            v5097 = v4979;
                            v9829 = v9830;
                        } else {
                            let v4980 = v4 / v754;
                            let v14362 = ((v10488 * v4980) * v10382) / v754;
                            let v4981 = v4980 / v1206;
                            let v4982 = v4878 - v4959;
                            let v14367 = v10820 - v9445;
                            let v4983 = v4981 * v4982;
                            let v4984 = v4983 * v4982;
                            let v4985 = v73 / v4982;
                            let v4986 = v658 + v4985;
                            let v4988 = (v4984.ln()) / v4986;
                            let v14383 = (((((((((Lanes([0.0, 0.0, v14362[0], 0.0, 0.0])) - (v9414 * v4981)) / v1206) * v4982) + (v14367 * v4981)) * v4982) + (v14367 * v4983)) * (v9360 / v4984)) - (((Lanes([0.0, 0.0, v10402[0], 0.0, 0.0])) + (((v14367 * v4985) * v10382) / v4982)) * v4988)) / v4986;
                            let v4990 = v4988 + v4989;
                            let v14384 = Lanes([v14383[0], v14383[1], v14383[2], v14383[3], v14383[4], 0.0]);
                            let v14385 = v14384 - v14358;
                            let v4992 = (v4990 - v4918) - v1265;
                            let v4994 = (v85 * v4990) * v1265;
                            let v14387 = (v14383 * v85) * v1265;
                            let v4995 = if v4994 > v0 { 1.0 } else { 0.0 };
                            let v4997: f64;
                            let v9831: Lanes<5>;
                            if v4995 != 0.0 {
                                v4997 = v4994;
                                v9831 = v14387;
                            } else {
                                let v4996 = -v4994;
                                let v14388 = v14387 * v10382;
                                v4997 = v4996;
                                v9831 = v14388;
                            }
                            let v14389 = v14385 * v4992;
                            let v5000 = ((v4992 * v4992) + v4997).sqrt();
                            let v5003 = v4990 - (v8 * (v4992 + v5000));
                            let v14398 = v14384 - ((v14385 + (((v14389 + v14389) + (Lanes([v9831[0], v9831[1], v9831[2], v9831[3], v9831[4], 0.0]))) * (v9360 / (v10427 * v5000)))) * v8);
                            v5087 = v5003;
                            v5097 = v4918;
                            v9829 = v14398;
                        }
                        v5086 = v5087;
                        v5096 = v5097;
                        v9828 = v9829;
                    }
                    let v5082: f64;
                    let v5094: f64;
                    let v9832: Lanes<6>;
                    if v148 != 0.0 {
                        let v5005 = if (v825 - v4959) <= v4879 { 1.0 } else { 0.0 };
                        let v5083: f64;
                        let v5095: f64;
                        let v9833: Lanes<5>;
                        if v5005 != 0.0 {
                            let v5006 = v4 / v1123;
                            let v5007 = v7 / v118;
                            let v5008 = v4 / v125;
                            let v5010 = (v5006 + v5007) + v5008;
                            let v5011 = v4 / v5010;
                            let v5014 = v5008 + (v8 * v5007);
                            let v5017 = (v4878 - v4968) + (v5014 * (-v4972));
                            let v14547 = ((((((v9413 * v5006) * v10382) / v1123) * v5011) * v10382) / v5010) * v5017;
                            let v5019 = (v5011 * v5017) / v1123;
                            let v14551 = v9413 * v5019;
                            let v5020 = v4878 - v5019;
                            let v14555 = v10820 - ((((Lanes([v14547[0], v14547[1], 0.0, v14547[2], v14547[3]])) + (((v10820 - (Lanes([v9446[0], v9446[1], v9446[2], 0.0, v9446[3]]))) + ((v9447 * v10382) * v5014)) * v5011)) - (Lanes([v14551[0], v14551[1], 0.0, v14551[2], v14551[3]]))) / v1123);
                            v5083 = v5020;
                            v5095 = v5020;
                            v9833 = v14555;
                        } else {
                            let v5021 = v4 / v1123;
                            let v5022 = v7 / v118;
                            let v5023 = v4 / v125;
                            let v5025 = (v5021 + v5022) + v5023;
                            let v5026 = v4 / v5025;
                            let v5029 = v5023 + (v8 * v5022);
                            let v5032 = (v4878 - v4968) + (v5029 * (-v4972));
                            let v14482 = ((((((v9413 * v5021) * v10382) / v1123) * v5026) * v10382) / v5025) * v5032;
                            let v5034 = (v5026 * v5032) / v1123;
                            let v14486 = v9413 * v5034;
                            let v5035 = v4878 - v5034;
                            let v14490 = v10820 - ((((Lanes([v14482[0], v14482[1], 0.0, v14482[2], v14482[3]])) + (((v10820 - (Lanes([v9446[0], v9446[1], v9446[2], 0.0, v9446[3]]))) + ((v9447 * v10382) * v5029)) * v5026)) - (Lanes([v14486[0], v14486[1], 0.0, v14486[2], v14486[3]]))) / v1123);
                            let v5036 = v4878 - v4959;
                            let v14491 = v10820 - v9445;
                            let v5037 = if v5036 > v0 { 1.0 } else { 0.0 };
                            let v5084: f64;
                            let v9834: Lanes<5>;
                            if v5037 != 0.0 {
                                let v5038 = v4 / v754;
                                let v14494 = ((v10488 * v5038) * v10382) / v754;
                                let v5039 = v5038 / v1206;
                                let v5040 = v5039 * v5036;
                                let v5041 = v5040 * v5036;
                                let v5042 = v73 / v5036;
                                let v5043 = v658 + v5042;
                                let v5045 = (v5041.ln()) / v5043;
                                let v5047 = (v5045 + v4989) * v1656;
                                let v14515 = ((((((((((Lanes([0.0, 0.0, v14494[0], 0.0, 0.0])) - (v9414 * v5039)) / v1206) * v5036) + (v14491 * v5039)) * v5036) + (v14491 * v5040)) * (v9360 / v5041)) - (((Lanes([0.0, 0.0, v10402[0], 0.0, 0.0])) + (((v14491 * v5042) * v10382) / v5036)) * v5045)) / v5043) * v1656;
                                let v5048 = v5047 - v703;
                                let v5051 = if (if v5035 > v5048 { 1.0 } else { 0.0 }) != 0.0 && v5050 != 0.0 { 1.0 } else { 0.0 };
                                let v5085: f64;
                                let v9835: Lanes<5>;
                                if v5051 != 0.0 {
                                    let v14516 = v14490 - v14515;
                                    let v5053 = (v5035 - v5047) + v703;
                                    let v5054 = v5053 * v5053;
                                    let v14517 = v14516 * v5053;
                                    let v14519 = (v14517 + v14517) * v5054;
                                    let v14520 = v14519 + v14519;
                                    let v5057 = (v5054 * v5054) + v5056;
                                    let v5074: f64;
                                    let v9836: Lanes<5>;
                                    if v5058 != 0.0 {
                                        let v5068: f64;
                                        if v5059 != 0.0 {
                                            v5068 = v4;
                                        } else {
                                            let v5069: f64;
                                            if v5060 != 0.0 {
                                                v5069 = v73;
                                            } else {
                                                let v5070: f64;
                                                if v5061 != 0.0 {
                                                    v5070 = v91;
                                                } else {
                                                    let v5071: f64;
                                                    if v5062 != 0.0 {
                                                        v5071 = v85;
                                                    } else {
                                                        v5071 = v0;
                                                    }
                                                    v5070 = v5071;
                                                }
                                                v5069 = v5070;
                                            }
                                            v5068 = v5069;
                                        }
                                        let mut v5063: f64 = 0.0;
                                        let mut v5065: f64 = 0.0;
                                        let mut v9837: Lanes<5> = Lanes([0.0; 5]);
                                        v5063 = v0;
                                        v5065 = v5057;
                                        v9837 = v14520;
                                        loop {
                                            let v5064 = if v5063 < v5068 { 1.0 } else { 0.0 };
                                            if v5064 == 0.0 {
                                                break;
                                            }
                                            let v5066 = v5065.sqrt();
                                            let v14535 = v9837 * (v9360 / (v10427 * v5066));
                                            let v5067 = v5063 + v4;
                                            v5063 = v5067;
                                            v5065 = v5066;
                                            v9837 = v14535;
                                        }
                                        v5074 = v5065;
                                        v9836 = v9837;
                                    } else {
                                        let v5073 = v5057.powf(v5072);
                                        let v14524 = v14520 * (v5072 * (v5057.powf(v14521)));
                                        v5074 = v5073;
                                        v9836 = v14524;
                                    }
                                    let v5075 = v4 / v5074;
                                    let v5076 = v5053 * v703;
                                    let v5078 = v5048 + (v5076 * v5075);
                                    let v14532 = v14515 + (((v14516 * v703) * v5075) + ((((v9836 * v5075) * v10382) / v5074) * v5076));
                                    v5085 = v5078;
                                    v9835 = v14532;
                                } else {
                                    v5085 = v5035;
                                    v9835 = v14490;
                                }
                                v5084 = v5085;
                                v9834 = v9835;
                            } else {
                                v5084 = v5035;
                                v9834 = v14490;
                            }
                            v5083 = v5084;
                            v5095 = v5035;
                            v9833 = v9834;
                        }
                        let v14556 = Lanes([v9833[0], v9833[1], v9833[2], v9833[3], v9833[4], 0.0]);
                        v5082 = v5083;
                        v5094 = v5095;
                        v9832 = v14556;
                    } else {
                        v5082 = v5086;
                        v5094 = v5096;
                        v9832 = v9828;
                    }
                    v5081 = v5082;
                    v5093 = v5094;
                    v9827 = v9832;
                } else {
                    let v14359 = Lanes([v9448[0], v9448[1], v9448[2], v9448[3], v9448[4], 0.0]);
                    v5081 = v5088;
                    v5093 = v4918;
                    v9827 = v14359;
                }
                let v5080 = v4901 + v5079;
                let v5091 = if v5081 < v5080 { 1.0 } else { 0.0 };
                let v5092: f64;
                let v9838: Lanes<6>;
                if v5091 != 0.0 {
                    v5092 = v5080;
                    v9838 = v14329;
                } else {
                    v5092 = v5081;
                    v9838 = v9827;
                }
                if v0 != 0.0 {
                    let v5098 = v5093 - v5092;
                    let v5099 = if v5098 >= v0 { 1.0 } else { 0.0 };
                    let v5100: f64;
                    if v5099 != 0.0 {
                        v5100 = v5098;
                    } else {
                        v5100 = v0;
                    }
                    let v5104 = ((v5101 * v5100) - v4989) - v1980;
                    let v5108 = (v85 * (v5105 * v5100)) * v1980;
                    let v5109 = if v5108 > v0 { 1.0 } else { 0.0 };
                    let v5111: f64;
                    if v5109 != 0.0 {
                        v5111 = v5108;
                    } else {
                        let v5110 = -v5108;
                        v5111 = v5110;
                    }
                    let v5119 = (v5115 * v5100) - (v8 * (v5104 + (((v5104 * v5104) + v5111).sqrt())));
                    let v5120 = if v5119 <= v5100 { 1.0 } else { 0.0 };
                    let v5121: f64;
                    if v5120 != 0.0 {
                        v5121 = v5119;
                    } else {
                        v5121 = v5100;
                    }
                    let v5122 = if v5121 < v0 { 1.0 } else { 0.0 };
                    if v5122 != 0.0 {
                    } else {
                        let v5123 = if v5121 > v818 { 1.0 } else { 0.0 };
                        if v5123 != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                let v5125 = if v5124 == v4 { 1.0 } else { 0.0 };
                let v5360: f64;
                let v9839: Lanes<6>;
                if v5125 != 0.0 {
                    let v5128 = if v825 < ((v1200 + v4901) + v4877) { 1.0 } else { 0.0 };
                    let v5361: f64;
                    let v9840: Lanes<6>;
                    if v5128 != 0.0 {
                        let v5129 = v73 * v660;
                        let v5131 = (-v363) / v1201;
                        let v5132 = v5131.ln();
                        let v5133 = v5129 * v5132;
                        let v14788 = (v10407 * v73) * v5132;
                        let v14791 = (Lanes([0.0, 0.0, v14788[0], 0.0, 0.0])) + (((((v10826 * v5131) * v10382) / v1201) * (v9360 / v5131)) * v5129);
                        let v5134 = v658 * v745;
                        let v5135 = v4 / v5134;
                        let v5136 = v5135 * v1123;
                        let v14798 = (((((v10402 * v745) + (v10477 * v658)) * v5135) * v10382) / v5134) * v1123;
                        let v14799 = v9413 * v5135;
                        let v14802 = (Lanes([0.0, 0.0, v14798[0], 0.0, 0.0])) + (Lanes([v14799[0], v14799[1], 0.0, v14799[2], v14799[3]]));
                        let v14803 = v14802 * v5137;
                        let v5139 = v73 + (v5137 * v5136);
                        let v5140 = v86 * v5139;
                        let v5141 = v5140 * v5139;
                        let v5142 = v5141 * v5139;
                        let v14810 = ((((v14803 * v86) * v5139) + (v14803 * v5140)) * v5139) + (v14803 * v5141);
                        let v5143 = v4903 - v73;
                        let v5144 = v3495 * v5136;
                        let v5145 = v5144 * v5143;
                        let v14812 = (v14802 * v3495) * v5143;
                        let v14815 = (Lanes([v14812[0], v14812[1], v14812[2], v14812[3], v14812[4], 0.0])) + (v14335 * v5144);
                        let v5147 = v5146 - v5145;
                        let v14816 = v14815 * v10382;
                        let v5148 = v5147 * v5147;
                        let v14817 = v14816 * v5147;
                        let v14818 = v14817 + v14817;
                        let v5150 = if v5142 < (v5148 * v3501) { 1.0 } else { 0.0 };
                        let v5162: f64;
                        let v9841: Lanes<6>;
                        if v5150 != 0.0 {
                            let v14825 = v14810 * v8;
                            let v5154 = (v8 * v5142) / v5147;
                            let v5156 = ((v5151 + v5147) + v5154) + v5145;
                            let v14831 = (v14816 + (((Lanes([v14825[0], v14825[1], v14825[2], v14825[3], v14825[4], 0.0])) - (v14816 * v5154)) / v5147)) + v14815;
                            v5162 = v5156;
                            v9841 = v14831;
                        } else {
                            let v5158 = (v5142 + v5148).sqrt();
                            let v5161 = (v5159 + v5158) + v5145;
                            let v14824 = (((Lanes([v14810[0], v14810[1], v14810[2], v14810[3], v14810[4], 0.0])) + v14818) * (v9360 / (v10427 * v5158))) + v14815;
                            v5162 = v5161;
                            v9841 = v14824;
                        }
                        let v5163 = v5162.powf(v1557);
                        let v14835 = v9841 * (v1557 * (v5162.powf(v14832)));
                        let v14837 = (v14802 * v3518) * v10382;
                        let v5169 = v743 * v5163;
                        let v5171 = ((v5164 - (v3518 * v5136)) + (v73 * v5163)) + (v5169 * v5163);
                        let v5172 = v4 / v5163;
                        let v5173 = v5171 * v5172;
                        let v14853 = v10407 * v5173;
                        let v5176 = ((v5173 * v660) + v4901) - v4901;
                        let v14857 = ((((((((Lanes([v14837[0], v14837[1], v14837[2], v14837[3], v14837[4], 0.0])) + (v14835 * v73)) + (((v14835 * v743) * v5163) + (v14835 * v5169))) * v5172) + ((((v14835 * v5172) * v10382) / v5163) * v5171)) * v660) + (Lanes([0.0, 0.0, v14853[0], 0.0, 0.0, 0.0]))) + v14329) - v14329;
                        let v5177 = v5176 / v5133;
                        let v14858 = v14791 * v5177;
                        let v14862 = ((v14857 - (Lanes([v14858[0], v14858[1], v14858[2], v14858[3], v14858[4], 0.0]))) / v5133) * v5177;
                        let v5180 = (v4 + (v5177 * v5177)).sqrt();
                        let v5181 = v5176 / v5180;
                        let v5182 = v5181 + v4901;
                        let v14870 = ((v14857 - (((v14862 + v14862) * (v9360 / (v10427 * v5180))) * v5181)) / v5180) + v14329;
                        v5361 = v5182;
                        v9840 = v14870;
                    } else {
                        let v5183 = v4901 - v4989;
                        let v14557 = v10402 * v5183;
                        let v5185 = (v658 * v5183).exp();
                        let v14561 = ((Lanes([0.0, 0.0, v14557[0], 0.0, 0.0, 0.0])) + (v14329 * v658)) * v5185;
                        let v5189 = (((v486 * v7) * v7) / v73) / v118;
                        let v5192 = ((v73 * v658) * v5189).sqrt();
                        let v14566 = ((v10402 * v73) * v5189) * (v9360 / (v10427 * v5192));
                        let v5193 = v5192.exp();
                        let v5195 = (-v5192).exp();
                        let v5197 = (v5193 + v5195) / v73;
                        let v5199 = (v5197.ln()) / v5189;
                        let v14574 = ((((v14566 * v5193) + ((v14566 * v10382) * v5195)) / v73) * (v9360 / v5197)) / v5189;
                        let mut v5200: f64 = 0.0;
                        let mut v5203: f64 = 0.0;
                        let mut v5291: f64 = 0.0;
                        let mut v9842: Lanes<6> = Lanes([0.0; 6]);
                        v5200 = v4;
                        v5203 = v5092;
                        v5291 = v0;
                        v9842 = v9838;
                        loop {
                            let v5202 = if v5200 <= v5201 { 1.0 } else { 0.0 };
                            if v5202 == 0.0 {
                                break;
                            }
                            let v5204 = v5203 - v4901;
                            let v14575 = v9842 - v14329;
                            let v5205 = v658 * v5204;
                            let v14576 = v10402 * v5204;
                            let v14579 = (Lanes([0.0, 0.0, v14576[0], 0.0, 0.0, 0.0])) + (v14575 * v658);
                            let v5206 = v5204 - v5189;
                            let v5207 = v5199 * v5206;
                            let v14580 = v14574 * v5206;
                            let v14583 = (Lanes([0.0, 0.0, v14580[0], 0.0, 0.0, 0.0])) + (v14575 * v5199);
                            let v5208 = if v5207 < v2530 { 1.0 } else { 0.0 };
                            let v5218: f64;
                            let v5222: f64;
                            let v9843: Lanes<6>;
                            let v9844: Lanes<6>;
                            if v5208 != 0.0 {
                                let v5209 = v5207.exp();
                                let v14584 = v14583 * v5209;
                                let v5212 = ((-v5199) * v5189).exp();
                                let v14587 = ((v14574 * v10382) * v5189) * v5212;
                                let v14589 = v14584 - (Lanes([0.0, 0.0, v14587[0], 0.0, 0.0, 0.0]));
                                let v5214 = v4 + (v5209 - v5212);
                                let v5216 = (v5214.ln()) / v5199;
                                let v14592 = v14574 * v5216;
                                let v14595 = ((v14589 * (v9360 / v5214)) - (Lanes([0.0, 0.0, v14592[0], 0.0, 0.0, 0.0]))) / v5199;
                                let v5217 = v5209 / v5214;
                                let v14598 = (v14584 - (v14589 * v5217)) / v5214;
                                v5218 = v5216;
                                v5222 = v5217;
                                v9843 = v14595;
                                v9844 = v14598;
                            } else {
                                v5218 = v5206;
                                v5222 = v4;
                                v9843 = v14575;
                                v9844 = v11054;
                            }
                            let v5219 = v658 * v5218;
                            let v14599 = v10402 * v5218;
                            let v14602 = (Lanes([0.0, 0.0, v14599[0], 0.0, 0.0, 0.0])) + (v9843 * v658);
                            let v5220 = v5205.abs();
                            let v5221 = if v5220 < v3667 { 1.0 } else { 0.0 };
                            let v5295: f64;
                            let v5299: f64;
                            let v9845: Lanes<6>;
                            let v9846: Lanes<6>;
                            if v5221 != 0.0 {
                                let v14705 = v9844 * v5222;
                                let v5226 = ((v4 - (v5222 * v5222)) / v73).sqrt();
                                let v14711 = (((v14705 + v14705) * v10382) / v73) * (v9360 / (v10427 * v5226));
                                let v5227 = v5205 * v5226;
                                let v14714 = (v14579 * v5226) + (v14711 * v5205);
                                let v5228 = v658 * v5226;
                                let v14715 = v10402 * v5226;
                                let v14718 = (Lanes([0.0, 0.0, v14715[0], 0.0, 0.0, 0.0])) + (v14711 * v658);
                                let v5229 = if v5205 < v0 { 1.0 } else { 0.0 };
                                let v5296: f64;
                                let v5300: f64;
                                let v9847: Lanes<6>;
                                let v9848: Lanes<6>;
                                if v5229 != 0.0 {
                                    let v5230 = -v5227;
                                    let v14719 = v14714 * v10382;
                                    let v5231 = -v5228;
                                    let v14720 = v14718 * v10382;
                                    v5296 = v5230;
                                    v5300 = v5231;
                                    v9847 = v14719;
                                    v9848 = v14720;
                                } else {
                                    v5296 = v5227;
                                    v5300 = v5228;
                                    v9847 = v14714;
                                    v9848 = v14718;
                                }
                                v5295 = v5296;
                                v5299 = v5300;
                                v9845 = v9847;
                                v9846 = v9848;
                            } else {
                                let v5232 = if v5220 < v3679 { 1.0 } else { 0.0 };
                                let v5297: f64;
                                let v5301: f64;
                                let v9849: Lanes<6>;
                                let v9850: Lanes<6>;
                                if v5232 != 0.0 {
                                    let v14627 = v14579 * v5205;
                                    let v5234 = (v5205 * v5205) / v73;
                                    let v5235 = v5205 / v91;
                                    let v14630 = v14579 / v91;
                                    let v5236 = v5205 / v85;
                                    let v14631 = v14579 / v85;
                                    let v5238 = v4 - (v5205 / v639);
                                    let v5240 = v4 - (v5236 * v5238);
                                    let v5242 = v4 - (v5235 * v5240);
                                    let v5244 = v5205 / v73;
                                    let v5245 = v4 - v5236;
                                    let v5247 = v4 - (v5235 * v5245);
                                    let v5249 = v4 - (v5244 * v5247);
                                    let v14658 = v14602 * v5219;
                                    let v5252 = (v5219 * v5219) / v73;
                                    let v5253 = v5219 / v91;
                                    let v14661 = v14602 / v91;
                                    let v5254 = v5219 / v85;
                                    let v14662 = v14602 / v85;
                                    let v5256 = v4 - (v5219 / v639);
                                    let v5258 = v4 - (v5254 * v5256);
                                    let v5260 = v4 - (v5253 * v5258);
                                    let v5262 = v5219 / v73;
                                    let v5263 = v4 - v5254;
                                    let v5265 = v4 - (v5253 * v5263);
                                    let v5267 = v4 - (v5262 * v5265);
                                    let v5268 = v5219 * v5267;
                                    let v5270 = ((v5234 * v5242) - (v5252 * v5260)).sqrt();
                                    let v14692 = (((((v14627 + v14627) / v73) * v5242) + ((((v14630 * v5240) + ((((v14631 * v5238) + (((v14579 / v639) * v10382) * v5236)) * v10382) * v5235)) * v10382) * v5234)) - ((((v14658 + v14658) / v73) * v5260) + ((((v14661 * v5258) + ((((v14662 * v5256) + (((v14602 / v639) * v10382) * v5254)) * v10382) * v5253)) * v10382) * v5252))) * (v9360 / (v10427 * v5270));
                                    let v5271 = v658 * v8;
                                    let v5273 = (v5205 * v5249) - (v5222 * v5268);
                                    let v14698 = (v10402 * v8) * v5273;
                                    let v5275 = (v5271 * v5273) / v5270;
                                    let v14704 = (((Lanes([0.0, 0.0, v14698[0], 0.0, 0.0, 0.0])) + ((((v14579 * v5249) + (((((v14579 / v73) * v5247) + ((((v14630 * v5245) + ((v14631 * v10382) * v5235)) * v10382) * v5244)) * v10382) * v5205)) - ((v9844 * v5268) + (((v14602 * v5267) + (((((v14602 / v73) * v5265) + ((((v14661 * v5263) + ((v14662 * v10382) * v5253)) * v10382) * v5262)) * v10382) * v5219)) * v5222))) * v5271)) - (v14692 * v5275)) / v5270;
                                    v5297 = v5270;
                                    v5301 = v5275;
                                    v9849 = v14692;
                                    v9850 = v14704;
                                } else {
                                    let v5277 = (-v5205).exp();
                                    let v14604 = (v14579 * v10382) * v5277;
                                    let v5279 = (-v5219).exp();
                                    let v14606 = (v14602 * v10382) * v5279;
                                    let v5283 = ((v5205 - v5219) + (v5277 - v5279)).sqrt();
                                    let v14612 = ((v14579 - v14602) + (v14604 - v14606)) * (v9360 / (v10427 * v5283));
                                    let v5284 = v658 * v8;
                                    let v5286 = v4 - v5279;
                                    let v5288 = (v4 - v5277) - (v5222 * v5286);
                                    let v14620 = (v10402 * v8) * v5288;
                                    let v5290 = (v5284 * v5288) / v5283;
                                    let v14626 = (((Lanes([0.0, 0.0, v14620[0], 0.0, 0.0, 0.0])) + (((v14604 * v10382) - ((v9844 * v5286) + ((v14606 * v10382) * v5222))) * v5284)) - (v14612 * v5290)) / v5283;
                                    v5297 = v5283;
                                    v5301 = v5290;
                                    v9849 = v14612;
                                    v9850 = v14626;
                                }
                                v5295 = v5297;
                                v5299 = v5301;
                                v9845 = v9849;
                                v9846 = v9850;
                            }
                            let v5292 = if v5291 == v4 { 1.0 } else { 0.0 };
                            let v5293 = if v5205 < v0 { 1.0 } else { 0.0 };
                            let v5294 = if v5292 != 0.0 && v5293 != 0.0 { 1.0 } else { 0.0 };
                            if v5294 != 0.0 {
                            } else {
                            }
                            let v5324: f64;
                            let v5328: f64;
                            let v9851: Lanes<6>;
                            let v9852: Lanes<6>;
                            if v5293 != 0.0 {
                                let v5298 = -v5295;
                                let v14757 = v9845 * v10382;
                                let v5302 = -v5299;
                                let v14758 = v9846 * v10382;
                                v5324 = v5298;
                                v5328 = v5302;
                                v9851 = v14757;
                                v9852 = v14758;
                            } else {
                                let v5303 = if v5205 < v112 { 1.0 } else { 0.0 };
                                let v5325: f64;
                                let v5329: f64;
                                let v9853: Lanes<6>;
                                let v9854: Lanes<6>;
                                if v5303 != 0.0 {
                                    v5325 = v5295;
                                    v5329 = v5299;
                                    v9853 = v9845;
                                    v9854 = v9846;
                                } else {
                                    let v5304 = v5203 - v4989;
                                    let v14721 = v10402 * v5304;
                                    let v5306 = (v658 * v5304).exp();
                                    let v14725 = ((Lanes([0.0, 0.0, v14721[0], 0.0, 0.0, 0.0])) + (v9842 * v658)) * v5306;
                                    let v5307 = v5205 + v4;
                                    let v5309 = v5306 - (v5185 * v5307);
                                    let v14730 = v10488 * v5309;
                                    let v5311 = v754 * v658;
                                    let v5312 = v5306 - v5185;
                                    let v14738 = ((v10488 * v658) + (v10402 * v754)) * v5312;
                                    let v14742 = v9845 * v5295;
                                    let v5316 = ((v5295 * v5295) + (v754 * v5309)).sqrt();
                                    let v14747 = ((v14742 + v14742) + ((Lanes([0.0, 0.0, v14730[0], 0.0, 0.0, 0.0])) + ((v14725 - ((v14561 * v5307) + (v14579 * v5185))) * v754))) * (v9360 / (v10427 * v5316));
                                    let v5317 = v73 * v5299;
                                    let v5321 = (v8 * ((v5317 * v5295) + (v5311 * v5312))) / v5316;
                                    let v14756 = ((((((v9846 * v73) * v5295) + (v9845 * v5317)) + ((Lanes([0.0, 0.0, v14738[0], 0.0, 0.0, 0.0])) + ((v14725 - v14561) * v5311))) * v8) - (v14747 * v5321)) / v5316;
                                    v5325 = v5316;
                                    v5329 = v5321;
                                    v9853 = v14747;
                                    v9854 = v14756;
                                }
                                v5324 = v5325;
                                v5328 = v5329;
                                v9851 = v9853;
                                v9852 = v9854;
                            }
                            let v14759 = v10820 * v10382;
                            let v14762 = v10826 * v5324;
                            let v5327 = ((-v4878) + v5203) + (v1201 * v5324);
                            let v14766 = ((Lanes([v14759[0], v14759[1], v14759[2], v14759[3], v14759[4], 0.0])) + v9842) + ((Lanes([v14762[0], v14762[1], v14762[2], v14762[3], v14762[4], 0.0])) + (v9851 * v1201));
                            let v14767 = v10826 * v5328;
                            let v14770 = (Lanes([v14767[0], v14767[1], v14767[2], v14767[3], v14767[4], 0.0])) + (v9852 * v1201);
                            let v5331 = v4 + (v1201 * v5328);
                            let v5354: f64;
                            let v5356: f64;
                            let v5357: f64;
                            let v9855: Lanes<6>;
                            if v5292 != 0.0 {
                                v5354 = v5332;
                                v5356 = v5203;
                                v5357 = v5291;
                                v9855 = v9842;
                            } else {
                                let v5334 = (-v5327) / v5331;
                                let v14774 = ((v14766 * v10382) - (v14770 * v5334)) / v5331;
                                let v5336 = v5203.abs();
                                let v14778 = v9842 * ((v10427 * (if v5203 >= v11296 { 1.0 } else { 0.0 })) - v9360);
                                let v5337 = if v4 >= v5336 { 1.0 } else { 0.0 };
                                let v5338: f64;
                                let v9856: Lanes<6>;
                                if v5337 != 0.0 {
                                    v5338 = v4;
                                    v9856 = v11054;
                                } else {
                                    v5338 = v5336;
                                    v9856 = v14778;
                                }
                                let v5340 = v5335 * (v4 + v5338);
                                let v14779 = v9856 * v5335;
                                let v5342 = if (v5334.abs()) > v5340 { 1.0 } else { 0.0 };
                                let v5347: f64;
                                let v9857: Lanes<6>;
                                if v5342 != 0.0 {
                                    let v5343 = if v5334 >= v0 { 1.0 } else { 0.0 };
                                    let v5345: f64;
                                    if v5343 != 0.0 {
                                        v5345 = v4;
                                    } else {
                                        v5345 = v5344;
                                    }
                                    let v5346 = v5340 * v5345;
                                    let v14780 = v14779 * v5345;
                                    v5347 = v5346;
                                    v9857 = v14780;
                                } else {
                                    v5347 = v5334;
                                    v9857 = v14774;
                                }
                                let v5348 = v5203 + v5347;
                                let v14781 = v9842 + v9857;
                                let v5353 = if (if (v5347.abs()) <= v856 { 1.0 } else { 0.0 }) != 0.0 && (if (v5327.abs()) <= v3501 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let v5358: f64;
                                if v5353 != 0.0 {
                                    v5358 = v4;
                                } else {
                                    v5358 = v5291;
                                }
                                v5354 = v5200;
                                v5356 = v5348;
                                v5357 = v5358;
                                v9855 = v14781;
                            }
                            let v5355 = v5354 + v4;
                            v5200 = v5355;
                            v5203 = v5356;
                            v5291 = v5357;
                            v9842 = v9855;
                        }
                        v5361 = v5203;
                        v9840 = v9842;
                    }
                    v5360 = v5361;
                    v9839 = v9840;
                } else {
                    v5360 = v5092;
                    v9839 = v9838;
                }
                let v5359 = -v658;
                let v5362 = v5360 - v4901;
                let v14872 = v9839 - v14329;
                let v5363 = v5359 * v5362;
                let v14873 = (v10402 * v10382) * v5362;
                let v14876 = (Lanes([0.0, 0.0, v14873[0], 0.0, 0.0, 0.0])) + (v14872 * v5359);
                let v5364 = if v5363 >= v0 { 1.0 } else { 0.0 };
                let v5366: f64;
                if v5364 != 0.0 {
                    v5366 = v4;
                } else {
                    v5366 = v5365;
                }
                let v5367 = v5366 * v5363;
                let v14877 = v14876 * v5366;
                let v5368 = v5363.exp();
                let v5370 = (v5368 - v4) - v5363;
                let v14879 = (v14876 * v5368) - v14876;
                let v5371 = if v5363 > v112 { 1.0 } else { 0.0 };
                let v5389: f64;
                let v9858: Lanes<6>;
                if v5371 != 0.0 {
                    let v5372 = -v745;
                    let v5373 = v5370.sqrt();
                    let v5374 = v5372 * v5373;
                    let v14904 = (v10477 * v10382) * v5373;
                    let v14907 = (Lanes([0.0, 0.0, v14904[0], 0.0, 0.0, 0.0])) + ((v14879 * (v9360 / (v10427 * v5373))) * v5372);
                    v5389 = v5374;
                    v9858 = v14907;
                } else {
                    let v5375 = if v5367 > v112 { 1.0 } else { 0.0 };
                    let v5390: f64;
                    let v9859: Lanes<6>;
                    if v5375 != 0.0 {
                        let v5376 = v5370.sqrt();
                        let v5377 = v745 * v5376;
                        let v14896 = v10477 * v5376;
                        let v14899 = (Lanes([0.0, 0.0, v14896[0], 0.0, 0.0, 0.0])) + ((v14879 * (v9360 / (v10427 * v5376))) * v745);
                        v5390 = v5377;
                        v9859 = v14899;
                    } else {
                        let v5378 = -v5366;
                        let v5381 = (v5378 * v5367) * v5380;
                        let v5382 = v5367 * v1557;
                        let v5384 = v4 + (v2045 * v5367);
                        let v5387 = (v4 + (v5382 * v5384)).sqrt();
                        let v5388 = v5381 * v5387;
                        let v14892 = (((v14877 * v5378) * v5380) * v5387) + (((((v14877 * v1557) * v5384) + ((v14877 * v2045) * v5382)) * (v9360 / (v10427 * v5387))) * v5381);
                        v5390 = v5388;
                        v9859 = v14892;
                    }
                    v5389 = v5390;
                    v9858 = v9859;
                }
                let v14908 = v9858 * v5389;
                let v5394 = ((v5389 * v5389) + v5392).sqrt();
                let v14914 = (v9858 + ((v14908 + v14908) * (v9360 / (v10427 * v5394)))) * v8;
                let v5398 = (v8 * (v5389 + v5394)) + v5397;
                let v5399 = if v5398 < v0 { 1.0 } else { 0.0 };
                let v5400: f64;
                let v9860: Lanes<6>;
                if v5399 != 0.0 {
                    v5400 = v0;
                    v9860 = v11054;
                } else {
                    v5400 = v5398;
                    v9860 = v14914;
                }
                let v5401 = v5400 / v486;
                let v14915 = v9860 / v486;
                let v5402 = v5401 - v4880;
                let v5403 = v5401 * v15;
                let v14916 = v14915 * v15;
                let v14917 = v14915 * v5402;
                let v5405 = v85 * v5403;
                let v5408 = ((v5402 * v5402) + (v5405 * v5403)).sqrt();
                let v5412 = (v8 * (v5402 + v5408)) + (v531 * v5403);
                let v14930 = ((v14915 + (((v14917 + v14917) + (((v14916 * v85) * v5403) + (v14916 * v5405))) * (v9360 / (v10427 * v5408)))) * v8) + (v14916 * v531);
                let v5413 = if v5412 < v0 { 1.0 } else { 0.0 };
                let v5414: f64;
                let v9861: Lanes<6>;
                if v5413 != 0.0 {
                    v5414 = v0;
                    v9861 = v11054;
                } else {
                    v5414 = v5412;
                    v9861 = v14930;
                }
                let v5415 = v5414 / v5401;
                let v5417 = (v5415 * v5414) / v5401;
                let v5419 = (v5362 * v5417) + v4901;
                let v14943 = ((v14872 * v5417) + (((((((v9861 - (v14915 * v5415)) / v5401) * v5414) + (v9861 * v5415)) - (v14915 * v5417)) / v5401) * v5362)) + v14329;
                let v14944 = v10402 * v5419;
                let v5421 = (v658 * v5419).exp();
                let v5422 = v5419 - v818;
                let v14950 = v10402 * v5422;
                let v5424 = (v658 * v5422).exp();
                let v5425 = v5421 - v5424;
                let v14955 = (((Lanes([0.0, 0.0, v14944[0], 0.0, 0.0, 0.0])) + (v14943 * v658)) * v5421) - (((Lanes([0.0, 0.0, v14950[0], 0.0, 0.0, 0.0])) + ((v14943 - v14325) * v658)) * v5424);
                let v5429 = ((v5426 * v36) * v118).sqrt();
                let v5430 = v5429 * v727;
                let v14956 = v10448 * v5429;
                let v5431 = v5419 - v4901;
                let v5432 = v658 * v5431;
                let v14958 = v10402 * v5431;
                let v14961 = (Lanes([0.0, 0.0, v14958[0], 0.0, 0.0, 0.0])) + ((v14943 - v14329) * v658);
                let v5433 = v1884 * v658;
                let v14962 = v10402 * v1884;
                let v5436 = if (if v5432 < v5433 { 1.0 } else { 0.0 }) != 0.0 && (if v5433 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v5462: f64;
                let v9862: Lanes<6>;
                if v5436 != 0.0 {
                    let v5437 = v5433 - v5432;
                    let v14963 = Lanes([0.0, 0.0, v14962[0], 0.0, 0.0, 0.0]);
                    let v14964 = v14963 - v14961;
                    let v14965 = v14964 * v5437;
                    let v14967 = v14962 * v5433;
                    let v14968 = v14967 + v14967;
                    let v5440 = (v5437 * v5437) + (v5433 * v5433);
                    let v14970 = (v14965 + v14965) + (Lanes([0.0, 0.0, v14968[0], 0.0, 0.0, 0.0]));
                    let v5457: f64;
                    let v9863: Lanes<6>;
                    if v5441 != 0.0 {
                        let v5451: f64;
                        if v5442 != 0.0 {
                            v5451 = v4;
                        } else {
                            let v5452: f64;
                            if v5443 != 0.0 {
                                v5452 = v73;
                            } else {
                                let v5453: f64;
                                if v5444 != 0.0 {
                                    v5453 = v91;
                                } else {
                                    let v5454: f64;
                                    if v5445 != 0.0 {
                                        v5454 = v85;
                                    } else {
                                        v5454 = v0;
                                    }
                                    v5453 = v5454;
                                }
                                v5452 = v5453;
                            }
                            v5451 = v5452;
                        }
                        let mut v5446: f64 = 0.0;
                        let mut v5448: f64 = 0.0;
                        let mut v9864: Lanes<6> = Lanes([0.0; 6]);
                        v5446 = v0;
                        v5448 = v5440;
                        v9864 = v14970;
                        loop {
                            let v5447 = if v5446 < v5451 { 1.0 } else { 0.0 };
                            if v5447 == 0.0 {
                                break;
                            }
                            let v5449 = v5448.sqrt();
                            let v18850 = v9864 * (v9360 / (v10427 * v5449));
                            let v5450 = v5446 + v4;
                            v5446 = v5450;
                            v5448 = v5449;
                            v9864 = v18850;
                        }
                        v5457 = v5448;
                        v9863 = v9864;
                    } else {
                        let v5456 = v5440.sqrt();
                        let v14974 = v14970 * (v5455 * (v5440.powf(v14971)));
                        v5457 = v5456;
                        v9863 = v14974;
                    }
                    let v5458 = v4 / v5457;
                    let v5459 = v5437 * v5433;
                    let v14979 = v14962 * v5437;
                    let v5461 = v5433 - (v5459 * v5458);
                    let v14985 = v14963 - ((((v14964 * v5433) + (Lanes([0.0, 0.0, v14979[0], 0.0, 0.0, 0.0]))) * v5458) + ((((v9863 * v5458) * v10382) / v5457) * v5459));
                    v5462 = v5461;
                    v9862 = v14985;
                } else {
                    v5462 = v5432;
                    v9862 = v14961;
                }
                let v5465 = (v5462 + v5463).sqrt();
                let v5466 = v5430 * v5465;
                let v14989 = v14956 * v5465;
                let v5468 = (v73 * v660) / v137;
                let v14995 = ((v10407 * v73) / v137) * v5466;
                let v5471 = ((v5468 * v5466) * v4874) * v162;
                let v5473 = v4871 + (v5471 * v5425);
                let v15004 = v9768 + ((((((Lanes([0.0, 0.0, v14995[0], 0.0, 0.0, 0.0])) + (((Lanes([0.0, 0.0, v14989[0], 0.0, 0.0, 0.0])) + ((v9862 * (v9360 / (v10427 * v5465))) * v5430)) * v5468)) * v4874) * v162) * v5425) + (v14955 * v5471));
                v5615 = v5473;
                v6019 = v5389;
                v9823 = v15004;
                v9824 = v9858;
            } else {
                v5615 = v4871;
                v6019 = v4416;
                v9823 = v9768;
                v9824 = v9436;
            }
            let v5476 = if v5 != 0.0 || (if v5474 == v4 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v5635: f64;
            let v9865: Lanes<6>;
            if v5476 != 0.0 {
                let v5479 = if (if v4320 == v4 { 1.0 } else { 0.0 }) != 0.0 || (if v1881 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v5636: f64;
                let v9866: Lanes<6>;
                if v5479 != 0.0 {
                    v5636 = v0;
                    v9866 = v11054;
                } else {
                    let v5482 = if (if v293 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v16 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v5637: f64;
                    let v9867: Lanes<6>;
                    if v5482 != 0.0 {
                        v5637 = v0;
                        v9867 = v11054;
                    } else {
                        let v15008 = ((Lanes([v10556[0], v10556[1], 0.0, v10556[2], v10556[3]])) + v10783) - v10819;
                        let v5487 = (((v862 - v345) + v1137) - v1194) + v5486;
                        let v5607: f64;
                        let v9868: Lanes<6>;
                        if v277 != 0.0 {
                            let v5488 = v1123 * v1123;
                            let v15095 = v9413 * v1123;
                            let v15096 = v15095 + v15095;
                            let v5489 = v487 / v5488;
                            let v15099 = ((v15096 * v5489) * v10382) / v5488;
                            let v5490 = v73 / v487;
                            let v5491 = v5490 * v5488;
                            let v15103 = v9408 * v2075;
                            let v15105 = (v15008 - (Lanes([0.0, 0.0, v10407[0], 0.0, 0.0]))) - (Lanes([v15103[0], v15103[1], 0.0, 0.0, v15103[2]]));
                            let v5500 = ((v5487 - v660) - (v2075 * v983)) - (v2075 * ((v5495 * v5496) / v119));
                            let v15111 = (v15096 * v5490) * v5500;
                            let v15114 = (Lanes([v15111[0], v15111[1], 0.0, v15111[2], v15111[3], 0.0])) + (((Lanes([v15105[0], v15105[1], v15105[2], v15105[3], v15105[4], 0.0])) - (((v9449 * v5495) / v119) * v2075)) * v5491);
                            let v5502 = v4 + (v5491 * v5500);
                            let v15115 = v15114 * v5502;
                            let v5506 = ((v5502 * v5502) + v5504).sqrt();
                            let v15121 = (v15114 + ((v15115 + v15115) * (v9360 / (v10427 * v5506)))) * v8;
                            let v5510 = (v8 * (v5502 + v5506)) + v5509;
                            let v5511 = if v5510 < v0 { 1.0 } else { 0.0 };
                            let v5512: f64;
                            let v9869: Lanes<6>;
                            if v5511 != 0.0 {
                                v5512 = v0;
                                v9869 = v11054;
                            } else {
                                v5512 = v5510;
                                v9869 = v15121;
                            }
                            let v5514 = (v5512 + v358).sqrt();
                            let v15125 = v15008 * v2092;
                            let v5516 = v4 - v5514;
                            let v15127 = v15099 * v5516;
                            let v15133 = v10553 * v2098;
                            let v5522 = v2101 * v2102;
                            let v5524 = ((v2098 * v861) + v5520) - (v5522 * ((v5487 * v2092) + (v5489 * v5516)));
                            let v15137 = ((Lanes([v15133[0], v15133[1], 0.0, 0.0, v15133[2], 0.0])) + v9769) - (((Lanes([v15125[0], v15125[1], v15125[2], v15125[3], v15125[4], 0.0])) + ((Lanes([v15127[0], v15127[1], 0.0, v15127[2], v15127[3], 0.0])) + (((v9869 * (v9360 / (v10427 * v5514))) * v10382) * v5489))) * v5522);
                            let v15138 = v15137 * v5524;
                            let v5528 = ((v5524 * v5524) + v5526).sqrt();
                            let v15144 = (v15137 + ((v15138 + v15138) * (v9360 / (v10427 * v5528)))) * v8;
                            let v5532 = (v8 * (v5524 + v5528)) + v5531;
                            let v5533 = if v5532 < v0 { 1.0 } else { 0.0 };
                            let v5608: f64;
                            let v9870: Lanes<6>;
                            if v5533 != 0.0 {
                                v5608 = v0;
                                v9870 = v11054;
                            } else {
                                v5608 = v5532;
                                v9870 = v15144;
                            }
                            v5607 = v5608;
                            v9868 = v9870;
                        } else {
                            let v5534 = v2116 * v5487;
                            let v15009 = v15008 * v2116;
                            let v5535 = v1123 * v1123;
                            let v15010 = v9413 * v1123;
                            let v15011 = v15010 + v15010;
                            let v5536 = v487 / v5535;
                            let v15014 = ((v15011 * v5536) * v10382) / v5535;
                            let v5537 = v73 / v487;
                            let v5538 = v5537 * v5535;
                            let v15015 = v15011 * v5537;
                            let v15018 = v9408 * v2075;
                            let v15020 = (v15009 - (Lanes([0.0, 0.0, v10407[0], 0.0, 0.0]))) - (Lanes([v15018[0], v15018[1], 0.0, 0.0, v15018[2]]));
                            let v5545 = ((v5534 - v660) - (v2075 * v983)) - (v2075 * ((v5495 * v5496) / v119));
                            let v15026 = v15015 * v5545;
                            let v15029 = (Lanes([v15026[0], v15026[1], 0.0, v15026[2], v15026[3], 0.0])) + (((Lanes([v15020[0], v15020[1], v15020[2], v15020[3], v15020[4], 0.0])) - (((v9449 * v5495) / v119) * v2075)) * v5538);
                            let v5547 = v4 + (v5538 * v5545);
                            let v5549 = v73 * (v4 + v5538);
                            let v15030 = v15015 * v73;
                            let v5550 = v358 + v5549;
                            let v5553 = if (if v5547 < v5550 { 1.0 } else { 0.0 }) != 0.0 && (if v5549 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                            let v5585: f64;
                            let v9871: Lanes<6>;
                            if v5553 != 0.0 {
                                let v5554 = v5550 - v5547;
                                let v15031 = Lanes([v15030[0], v15030[1], 0.0, v15030[2], v15030[3], 0.0]);
                                let v15032 = v15031 - v15029;
                                let v5555 = v5554 * v5554;
                                let v15033 = v15032 * v5554;
                                let v15034 = v15033 + v15033;
                                let v5556 = v5549 * v5549;
                                let v15035 = v15030 * v5549;
                                let v15036 = v15035 + v15035;
                                let v5557 = v5555 * v5555;
                                let v15037 = v15034 * v5555;
                                let v5558 = v5556 * v5556;
                                let v15039 = v15036 * v5556;
                                let v5559 = v5557 * v5555;
                                let v5560 = v5558 * v5556;
                                let v15052 = ((((v15039 + v15039) * v5556) + (v15036 * v5558)) * v5556) + (v15036 * v5560);
                                let v5563 = (v5559 * v5555) + (v5560 * v5556);
                                let v15054 = (((((v15037 + v15037) * v5555) + (v15034 * v5557)) * v5555) + (v15034 * v5559)) + (Lanes([v15052[0], v15052[1], 0.0, v15052[2], v15052[3], 0.0]));
                                let v5580: f64;
                                let v9872: Lanes<6>;
                                if v5564 != 0.0 {
                                    let v5574: f64;
                                    if v5565 != 0.0 {
                                        v5574 = v4;
                                    } else {
                                        let v5575: f64;
                                        if v5566 != 0.0 {
                                            v5575 = v73;
                                        } else {
                                            let v5576: f64;
                                            if v5567 != 0.0 {
                                                v5576 = v91;
                                            } else {
                                                let v5577: f64;
                                                if v5568 != 0.0 {
                                                    v5577 = v85;
                                                } else {
                                                    v5577 = v0;
                                                }
                                                v5576 = v5577;
                                            }
                                            v5575 = v5576;
                                        }
                                        v5574 = v5575;
                                    }
                                    let mut v5569: f64 = 0.0;
                                    let mut v5571: f64 = 0.0;
                                    let mut v9873: Lanes<6> = Lanes([0.0; 6]);
                                    v5569 = v0;
                                    v5571 = v5563;
                                    v9873 = v15054;
                                    loop {
                                        let v5570 = if v5569 < v5574 { 1.0 } else { 0.0 };
                                        if v5570 == 0.0 {
                                            break;
                                        }
                                        let v5572 = v5571.sqrt();
                                        let v15094 = v9873 * (v9360 / (v10427 * v5572));
                                        let v5573 = v5569 + v4;
                                        v5569 = v5573;
                                        v5571 = v5572;
                                        v9873 = v15094;
                                    }
                                    v5580 = v5571;
                                    v9872 = v9873;
                                } else {
                                    let v5579 = v5563.powf(v5578);
                                    let v15058 = v15054 * (v5578 * (v5563.powf(v15055)));
                                    v5580 = v5579;
                                    v9872 = v15058;
                                }
                                let v5581 = v4 / v5580;
                                let v5582 = v5554 * v5549;
                                let v15063 = v15030 * v5554;
                                let v5584 = v5550 - (v5582 * v5581);
                                let v15069 = v15031 - ((((v15032 * v5549) + (Lanes([v15063[0], v15063[1], 0.0, v15063[2], v15063[3], 0.0]))) * v5581) + ((((v9872 * v5581) * v10382) / v5580) * v5582));
                                v5585 = v5584;
                                v9871 = v15069;
                            } else {
                                v5585 = v5547;
                                v9871 = v15029;
                            }
                            let v5586 = if v5585 <= v0 { 1.0 } else { 0.0 };
                            let v5588: f64;
                            let v9874: Lanes<6>;
                            if v5586 != 0.0 {
                                v5588 = v0;
                                v9874 = v11054;
                            } else {
                                let v5587 = v5585.sqrt();
                                let v15072 = v9871 * (v9360 / (v10427 * v5587));
                                v5588 = v5587;
                                v9874 = v15072;
                            }
                            let v5589 = v4 - v5588;
                            let v15074 = v15014 * v5589;
                            let v5593 = v138 / (v2101 + v138);
                            let v15080 = v10553 * v2098;
                            let v5597 = ((v2098 * v861) + v5520) - (v5593 * (v5534 + (v5536 * v5589)));
                            let v15084 = ((Lanes([v15080[0], v15080[1], 0.0, 0.0, v15080[2], 0.0])) + v9769) - (((Lanes([v15009[0], v15009[1], v15009[2], v15009[3], v15009[4], 0.0])) + ((Lanes([v15074[0], v15074[1], 0.0, v15074[2], v15074[3], 0.0])) + ((v9874 * v10382) * v5536))) * v5593);
                            let v15085 = v15084 * v5597;
                            let v5601 = ((v5597 * v5597) + v5599).sqrt();
                            let v15091 = (v15084 + ((v15085 + v15085) * (v9360 / (v10427 * v5601)))) * v8;
                            let v5605 = (v8 * (v5597 + v5601)) + v5604;
                            let v5606 = if v5605 < v0 { 1.0 } else { 0.0 };
                            let v5609: f64;
                            let v9875: Lanes<6>;
                            if v5606 != 0.0 {
                                v5609 = v0;
                                v9875 = v11054;
                            } else {
                                v5609 = v5605;
                                v9875 = v15091;
                            }
                            v5607 = v5609;
                            v9868 = v9875;
                        }
                        let v5610 = v5607 + v358;
                        let v5612 = (-v2191) / v5610;
                        let v5613 = v5612.exp();
                        let v5614 = v2195 * v5610;
                        let v5616 = v5614 * v5615;
                        let v5617 = v5616 * v5613;
                        let v15155 = ((((v9868 * v2195) * v5615) + (v9823 * v5614)) * v5613) + (((((v9868 * v5612) * v10382) / v5610) * v5613) * v5616);
                        v5637 = v5617;
                        v9867 = v15155;
                    }
                    v5636 = v5637;
                    v9866 = v9867;
                }
                v5635 = v5636;
                v9865 = v9866;
            } else {
                let v15005 = Lanes([v9450[0], v9450[1], v9450[2], v9450[3], v9450[4], 0.0]);
                v5635 = v5638;
                v9865 = v15005;
            }
            let v5620 = if (if v1881 == v4 { 1.0 } else { 0.0 }) != 0.0 && (if v2199 == v73 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v5621 = if v5620 != 0.0 && v5 != 0.0 { 1.0 } else { 0.0 };
            let v9205: f64;
            let v9876: Lanes<6>;
            if v5621 != 0.0 {
                let v5623 = (v202 * v7) * v162;
                let v5624 = -v658;
                let v15156 = v10402 * v10382;
                let v5626 = (v5624 * v2203).exp();
                let v5631 = v5628 + (v5629 * v473);
                let v5633 = (v5623 * v5626) * v5631;
                let v5634 = v5632 / v5633;
                let v15165 = (((((((v15156 * v2203) * v5626) * v5623) * v5631) * v5634) * v10382) / v5633) * v5635;
                let v5642 = v2218 * v660;
                let v5643 = v4 + (v5635 * v5634);
                let v5644 = v5643.ln();
                let v15171 = (v10407 * v2218) * v5644;
                let v15175 = Lanes([0.0, 0.0, v9394[0], 0.0, 0.0, 0.0]);
                let v5647 = v761 * v15;
                let v15177 = v9394 * v15;
                let v5648 = (v761 - (v5642 * v5644)) - v5647;
                let v15179 = (v15175 - ((Lanes([0.0, 0.0, v15171[0], 0.0, 0.0, 0.0])) + ((((v9865 * v5634) + (Lanes([0.0, 0.0, v15165[0], 0.0, 0.0, 0.0]))) * (v9360 / v5643)) * v5642))) - (Lanes([0.0, 0.0, v15177[0], 0.0, 0.0, 0.0]));
                let v5649 = v85 * v761;
                let v5650 = v5649 * v5647;
                let v15183 = ((v9394 * v85) * v5647) + (v15177 * v5649);
                let v5651 = if v5650 > v0 { 1.0 } else { 0.0 };
                let v5653: f64;
                let v9877: Lanes<1>;
                if v5651 != 0.0 {
                    v5653 = v5650;
                    v9877 = v15183;
                } else {
                    let v5652 = -v5650;
                    let v15184 = v15183 * v10382;
                    v5653 = v5652;
                    v9877 = v15184;
                }
                let v15185 = v15179 * v5648;
                let v5656 = ((v5648 * v5648) + v5653).sqrt();
                let v5661 = v5660 * v473;
                let v5663 = (v5661 * v660).sqrt();
                let v15198 = (v10407 * v5661) * (v9360 / (v10427 * v5663));
                let v5664 = v5520 - (v761 - (v8 * (v5648 + v5656)));
                let v15199 = v9769 - (v15175 - ((v15179 + (((v15185 + v15185) + (Lanes([0.0, 0.0, v9877[0], 0.0, 0.0, 0.0]))) * (v9360 / (v10427 * v5656)))) * v8));
                let v15200 = v15156 * v5664;
                let v5666 = (v5624 * v5664).exp();
                let v15205 = v10402 * v5664;
                let v5669 = (v5666 - v4) + (v658 * v5664);
                let v15209 = (((Lanes([0.0, 0.0, v15200[0], 0.0, 0.0, 0.0])) + (v15199 * v5624)) * v5666) + ((Lanes([0.0, 0.0, v15205[0], 0.0, 0.0, 0.0])) + (v15199 * v658));
                let v5670 = if v5669 > v0 { 1.0 } else { 0.0 };
                let v5675: f64;
                let v9878: Lanes<6>;
                if v5670 != 0.0 {
                    let v5671 = v5669.sqrt();
                    let v15217 = v15209 * (v9360 / (v10427 * v5671));
                    v5675 = v5671;
                    v9878 = v15217;
                } else {
                    let v5673 = (-v5669).sqrt();
                    let v5674 = -v5673;
                    let v15214 = ((v15209 * v10382) * (v9360 / (v10427 * v5673))) * v10382;
                    v5675 = v5674;
                    v9878 = v15214;
                }
                let v15218 = v15156 * v5520;
                let v5677 = (v5624 * v5520).exp();
                let v15223 = v10402 * v5520;
                let v5681 = ((v5677 - v4) + (v658 * v5520)).sqrt();
                let v5682 = -v5663;
                let v5683 = v5675 - v5681;
                let v15233 = (v15198 * v10382) * v5683;
                let v15237 = ((Lanes([0.0, 0.0, v15233[0], 0.0, 0.0, 0.0])) + ((v9878 - (((((Lanes([0.0, 0.0, v15218[0], 0.0, 0.0, 0.0])) + (v9769 * v5624)) * v5677) + ((Lanes([0.0, 0.0, v15223[0], 0.0, 0.0, 0.0])) + (v9769 * v658))) * (v9360 / (v10427 * v5681)))) * v5682)) * v10382;
                let v5687 = v5685 * v15;
                let v5688 = (v5685 - (v5682 * v5683)) - v5687;
                let v5690 = (v85 * v5685) * v5687;
                let v5691 = if v5690 > v0 { 1.0 } else { 0.0 };
                let v5693: f64;
                if v5691 != 0.0 {
                    v5693 = v5690;
                } else {
                    let v5692 = -v5690;
                    v5693 = v5692;
                }
                let v15238 = v15237 * v5688;
                let v5696 = ((v5688 * v5688) + v5693).sqrt();
                let v5699 = v5685 - (v8 * (v5688 + v5696));
                let v15245 = ((v15237 + ((v15238 + v15238) * (v9360 / (v10427 * v5696)))) * v8) * v10382;
                let v5700 = if v2243 > v0 { 1.0 } else { 0.0 };
                let v5701: f64;
                if v5700 != 0.0 {
                    v5701 = v2243;
                } else {
                    v5701 = v4;
                }
                let v5702 = v5635 + v2244;
                let v5703 = v5701 / v5702;
                let v5704 = v5703 * v1123;
                let v15250 = v9413 * v5703;
                let v15253 = v9372 * v5705;
                let v5708 = ((v5705 * v2249) - v5699) / v5704;
                let v15258 = (((Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v15253[0]])) - v15245) - ((((((v9865 * v5703) * v10382) / v5702) * v1123) + (Lanes([v15250[0], v15250[1], 0.0, v15250[2], v15250[3], 0.0]))) * v5708)) / v5704;
                v9205 = v5708;
                v9876 = v15258;
            } else {
                v9205 = v9206;
                v9876 = v9459;
            }
            let v5709 = if v4320 == v0 { 1.0 } else { 0.0 };
            let v5714 = if (if v5709 != 0.0 && (if v5635 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v5712 != v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v8414: f64;
            let v9879: Lanes<6>;
            if v5714 != 0.0 {
                let v5725: f64;
                let v5741: f64;
                let v9880: Lanes<6>;
                let v9881: Lanes<6>;
                if v981 != 0.0 {
                    v5725 = v0;
                    v5741 = v0;
                    v9880 = v11054;
                    v9881 = v11054;
                } else {
                    let v5715: f64;
                    let v9882: Lanes<6>;
                    if v5 != 0.0 {
                        let v15259 = Lanes([v9405[0], v9405[1], 0.0, 0.0, v9405[2], 0.0]);
                        v5715 = v830;
                        v9882 = v15259;
                    } else {
                        v5715 = v4632;
                        v9882 = v9441;
                    }
                    let v5719: f64;
                    let v9883: Lanes<6>;
                    if v5 != 0.0 {
                        let v15260 = Lanes([v9405[0], v9405[1], 0.0, 0.0, v9405[2], 0.0]);
                        v5719 = v830;
                        v9883 = v15260;
                    } else {
                        v5719 = v5716;
                        v9883 = v9451;
                    }
                    v5725 = v5715;
                    v5741 = v5719;
                    v9880 = v9882;
                    v9881 = v9883;
                }
                let v5723 = v5712 * (v4 + (v5720 * v1137));
                let v5724 = v5723 * v5635;
                let v15263 = ((v10783 * v5720) * v5712) * v5635;
                let v15266 = (Lanes([v15263[0], v15263[1], v15263[2], v15263[3], v15263[4], 0.0])) + (v9865 * v5723);
                let v5726 = v4335 - v5725;
                let v15268 = v10402 * v5726;
                let v15271 = (Lanes([0.0, 0.0, v15268[0], 0.0, 0.0, 0.0])) + ((v9433 - v9880) * v658);
                let v5728 = (v658 * v5726) - v4;
                let v15272 = v15271 * v5728;
                let v5732 = ((v5728 * v5728) + v5730).sqrt();
                let v15278 = (v15271 + ((v15272 + v15272) * (v9360 / (v10427 * v5732)))) * v8;
                let v5736 = (v8 * (v5728 + v5732)) + v5735;
                let v5737 = if v5736 < v0 { 1.0 } else { 0.0 };
                let v5738: f64;
                let v9884: Lanes<6>;
                if v5737 != 0.0 {
                    v5738 = v0;
                    v9884 = v11054;
                } else {
                    v5738 = v5736;
                    v9884 = v15278;
                }
                let v5739 = v5738.sqrt();
                let v15281 = v9884 * (v9360 / (v10427 * v5739));
                let v5740 = v5738 * v5739;
                let v15284 = (v9884 * v5739) + (v15281 * v5738);
                let v5742 = v4331 - v5741;
                let v15286 = v10402 * v5742;
                let v15289 = (Lanes([0.0, 0.0, v15286[0], 0.0, 0.0, 0.0])) + ((v9432 - v9881) * v658);
                let v5744 = (v658 * v5742) - v4;
                let v15290 = v15289 * v5744;
                let v5748 = ((v5744 * v5744) + v5746).sqrt();
                let v15296 = (v15289 + ((v15290 + v15290) * (v9360 / (v10427 * v5748)))) * v8;
                let v5752 = (v8 * (v5744 + v5748)) + v5751;
                let v5753 = if v5752 < v0 { 1.0 } else { 0.0 };
                let v5754: f64;
                let v9885: Lanes<6>;
                if v5753 != 0.0 {
                    v5754 = v0;
                    v9885 = v11054;
                } else {
                    v5754 = v5752;
                    v9885 = v15296;
                }
                let v5755 = v5754.sqrt();
                let v15299 = v9885 * (v9360 / (v10427 * v5755));
                let v5756 = v5754 * v5755;
                let v5757 = v4 / v5738;
                let v5758 = v658 * v5724;
                let v15306 = v10402 * v5724;
                let v15309 = (Lanes([0.0, 0.0, v15306[0], 0.0, 0.0, 0.0])) + (v15266 * v658);
                let v5759 = v5758 * v5757;
                let v15312 = (v15309 * v5757) + ((((v9884 * v5757) * v10382) / v5738) * v5758);
                let v5760 = v4 / v5754;
                let v5761 = v5758 * v5760;
                let v15318 = (v15309 * v5760) + ((((v9885 * v5760) * v10382) / v5754) * v5758);
                let v5764 = (v5756 * v5761) - (v5740 * v5759);
                let v15326 = v10477 * v5764;
                let v5766 = v745 * v8;
                let v5767 = -v5755;
                let v5770 = (v5767 * v5761) + (v5739 * v5759);
                let v15339 = (v10477 * v8) * v5770;
                let v5772 = (v745 * v5764) + (v5766 * v5770);
                let v5774 = v5773 * v5772;
                let v5779 = v5774 * v5775;
                let v15349 = (((v9770 * v5772) + ((((Lanes([0.0, 0.0, v15326[0], 0.0, 0.0, 0.0])) + ((((((v9885 * v5755) + (v15299 * v5754)) * v5761) + (v15318 * v5756)) - ((v15284 * v5759) + (v15312 * v5740))) * v745)) + ((Lanes([0.0, 0.0, v15339[0], 0.0, 0.0, 0.0])) + (((((v15299 * v10382) * v5761) + (v15318 * v5767)) + ((v15281 * v5759) + (v15312 * v5739))) * v5766))) * v5773)) * v5775) + (v9771 * v5774);
                v8414 = v5779;
                v9879 = v15349;
            } else {
                v8414 = v0;
                v9879 = v11054;
            }
            let v5780 = v117 * v63;
            let v5781 = v1123 / v552;
            let v15350 = v9413 / v552;
            let v5782 = v131 * v63;
            let v5783 = v162 * v63;
            let v5785 = v5784 / v63;
            let v15351 = v9772 / v63;
            let v5786 = v4423 / v552;
            let v15352 = v9437 / v552;
            let v5787 = v745 / v552;
            let v15353 = v10477 / v552;
            let v5789 = if v5788 == v0 { 1.0 } else { 0.0 };
            let v8675: f64;
            let v8679: f64;
            let v8680: f64;
            let v8684: f64;
            let v8689: f64;
            let v9886: Lanes<4>;
            let v9887: Lanes<6>;
            let v9888: Lanes<3>;
            let v9889: Lanes<3>;
            if v5789 != 0.0 {
                v8675 = v0;
                v8679 = v0;
                v8680 = v0;
                v8684 = v0;
                v8689 = v0;
                v9886 = v10617;
                v9887 = v11054;
                v9888 = v10523;
                v9889 = v10523;
            } else {
                let v8681: f64;
                let v9890: Lanes<6>;
                if v5709 != 0.0 {
                    let v15360 = (Lanes([v10556[0], v10556[1], 0.0, v10556[2], v10556[3]])) + (((v10783 - v10819) * v5794) * v5782);
                    let v5802 = v4 / v5780;
                    let v5803 = (((v862 - v236) + ((v5794 * (v1137 - v1194)) * v5782)) - (((v5520 + v861) - v5791) * v5799)) * v5802;
                    let v5805 = v4 / v5804;
                    let v5807 = v4 + (v5785 * v5805);
                    let v5808 = v5803 * v5807;
                    let v15368 = ((((Lanes([v15360[0], v15360[1], v15360[2], v15360[3], v15360[4], 0.0])) - ((v9769 + (Lanes([v10553[0], v10553[1], 0.0, 0.0, v10553[2], 0.0]))) * v5799)) * v5802) * v5807) + ((v15351 * v5805) * v5803);
                    let v15369 = v15368 * v5808;
                    let v5812 = ((v5808 * v5808) + v5810).sqrt();
                    let v15375 = (v15368 + ((v15369 + v15369) * (v9360 / (v10427 * v5812)))) * v8;
                    let v5816 = (v8 * (v5808 + v5812)) + v5815;
                    let v5817 = if v5816 < v0 { 1.0 } else { 0.0 };
                    let v5834: f64;
                    let v9891: Lanes<6>;
                    if v5817 != 0.0 {
                        v5834 = v0;
                        v9891 = v11054;
                    } else {
                        v5834 = v5816;
                        v9891 = v15375;
                    }
                    let v15376 = v10556 * v862;
                    let v5821 = ((v862 * v862) + v5819).sqrt();
                    let v15382 = (v10556 + ((v15376 + v15376) * (v9360 / (v10427 * v5821)))) * v8;
                    let v5825 = (v8 * (v862 + v5821)) + v5824;
                    let v5826 = if v5825 < v0 { 1.0 } else { 0.0 };
                    let v5827: f64;
                    let v9892: Lanes<4>;
                    if v5826 != 0.0 {
                        v5827 = v0;
                        v9892 = v10617;
                    } else {
                        v5827 = v5825;
                        v9892 = v15382;
                    }
                    let v5829 = (v5827 - v835) / v74;
                    let v15384 = (v9892 / v74) * v5829;
                    let v5831 = v4 + (v5829 * v5829);
                    let v5832 = v4 / v5831;
                    let v5833 = v4 - v5832;
                    let v5835 = v5834 * v5833;
                    let v15391 = (((((v15384 + v15384) * v5832) * v10382) / v5831) * v10382) * v5834;
                    let v15393 = (v9891 * v5833) + (Lanes([v15391[0], v15391[1], 0.0, v15391[2], v15391[3], 0.0]));
                    let v5836 = v5782 * v5783;
                    let v5839 = v5837 / (v5837 + v5836);
                    let v5841 = v5840 + v861;
                    let v5842 = v5840 / v5841;
                    let v15396 = ((v10553 * v5842) * v10382) / v5841;
                    let v5843 = v5835 + v358;
                    let v5844 = v4 / v5843;
                    let v5846 = -v5845;
                    let v5847 = v5846 * v714;
                    let v5848 = v5847 * v5844;
                    let v15401 = (v10433 * v5846) * v5844;
                    let v15404 = (Lanes([0.0, 0.0, v15401[0], 0.0, 0.0, 0.0])) + ((((v15393 * v5844) * v10382) / v5843) * v5847);
                    let v5850 = if v5848 < v5849 { 1.0 } else { 0.0 };
                    let v8682: f64;
                    let v9893: Lanes<6>;
                    if v5850 != 0.0 {
                        v8682 = v0;
                        v9893 = v11054;
                    } else {
                        let v5851 = v5848.exp();
                        let v5853 = v5852 / v713;
                        let v5855 = (v5853 * v202) * v5836;
                        let v5856 = v4 / v5787;
                        let v15414 = v15350 * v6;
                        let v5858 = v5786 + (v5781 * v6);
                        let v15418 = (((v15353 * v5856) * v10382) / v5787) * v5858;
                        let v5860 = (v5858 * v5856).sqrt();
                        let v5861 = v5851 * v5855;
                        let v15425 = (((((v10430 * v5853) * v10382) / v713) * v202) * v5836) * v5851;
                        let v5862 = v5861 * v5860;
                        let v5863 = v5862 * v5835;
                        let v5864 = v5863 * v5835;
                        let v5865 = v5839 * v5842;
                        let v5866 = v5865 * v5864;
                        let v15438 = (v15396 * v5839) * v5864;
                        let v15441 = (Lanes([v15438[0], v15438[1], 0.0, 0.0, v15438[2], 0.0])) + ((((((((((v15404 * v5851) * v5855) + (Lanes([0.0, 0.0, v15425[0], 0.0, 0.0, 0.0]))) * v5860) + (((((v15352 + (Lanes([v15414[0], v15414[1], 0.0, v15414[2], v15414[3], 0.0]))) * v5856) + (Lanes([0.0, 0.0, v15418[0], 0.0, 0.0, 0.0]))) * (v9360 / (v10427 * v5860))) * v5861)) * v5835) + (v15393 * v5862)) * v5835) + (v15393 * v5863)) * v5865);
                        v8682 = v5866;
                        v9893 = v15441;
                    }
                    v8681 = v8682;
                    v9890 = v9893;
                } else {
                    v8681 = v0;
                    v9890 = v11054;
                }
                let v5868 = -v5867;
                let v5873 = (v5780 * ((v5868 * v825) + v5870)).exp();
                let v5875 = (v825 / v5780) / v5780;
                let v5876 = v825 * v5875;
                let v5879 = (v5877 / v56) * v5783;
                let v5880 = v5879 * v5873;
                let v5881 = v5880 * v5876;
                let v15453 = (((((v9404 * v5868) * v5780) * v5873) * v5879) * v5876) + (((v9404 * v5875) + (((v9404 / v5780) / v5780) * v825)) * v5880);
                let v5882 = if v825 >= v0 { 1.0 } else { 0.0 };
                let v8690: f64;
                let v9894: Lanes<3>;
                if v5882 != 0.0 {
                    let v5884 = v5881 * v5883;
                    let v15454 = v15453 * v5883;
                    v8690 = v5884;
                    v9894 = v15454;
                } else {
                    v8690 = v5881;
                    v9894 = v15453;
                }
                let v5885 = v825 - v818;
                let v15456 = v9404 - (Lanes([v9402[0], v9402[1], 0.0]));
                let v5889 = (v5780 * ((v5868 * v5885) + v5870)).exp();
                let v5891 = (v5885 / v5780) / v5780;
                let v5892 = v5885 * v5891;
                let v5893 = v5879 * v5889;
                let v5894 = v5893 * v5892;
                let v15468 = (((((v15456 * v5868) * v5780) * v5889) * v5879) * v5892) + (((v15456 * v5891) + (((v15456 / v5780) / v5780) * v5885)) * v5893);
                let v5895 = if v5885 >= v0 { 1.0 } else { 0.0 };
                let v8685: f64;
                let v9895: Lanes<3>;
                if v5895 != 0.0 {
                    let v5897 = v5894 * v5896;
                    let v15469 = v15468 * v5896;
                    v8685 = v5897;
                    v9895 = v15469;
                } else {
                    v8685 = v5894;
                    v9895 = v15468;
                }
                let v15470 = v9404 * v10382;
                let v5903 = ((((-v825) + v873) + v236) + v5901) / v5780;
                let v15474 = ((Lanes([v15470[0], v15470[1], v15470[2], 0.0])) + (Lanes([v9407[0], v9407[1], 0.0, v9407[2]]))) / v5780;
                let v15475 = v15474 * v5903;
                let v5907 = ((v5903 * v5903) + v5905).sqrt();
                let v15481 = (v15474 + ((v15475 + v15475) * (v9360 / (v10427 * v5907)))) * v8;
                let v5911 = (v8 * (v5903 + v5907)) + v5910;
                let v5912 = if v5911 < v0 { 1.0 } else { 0.0 };
                let v5913: f64;
                let v9896: Lanes<4>;
                if v5912 != 0.0 {
                    v5913 = v0;
                    v9896 = v10617;
                } else {
                    v5913 = v5911;
                    v9896 = v15481;
                }
                let v5914 = v5913 + v358;
                let v5917 = (-v5915) / v5914;
                let v15484 = ((v9896 * v5917) * v10382) / v5914;
                let v5919 = if v5917 < v5918 { 1.0 } else { 0.0 };
                let v8676: f64;
                let v9897: Lanes<4>;
                if v5919 != 0.0 {
                    v8676 = v0;
                    v9897 = v10617;
                } else {
                    let v5920 = v5917.exp();
                    let v5923 = (v5921 * v5783) * v5782;
                    let v5924 = v5923 * v5914;
                    let v5925 = v5924 * v5914;
                    let v5926 = v5925 * v5920;
                    let v15492 = ((((v9896 * v5923) * v5914) + (v9896 * v5924)) * v5920) + ((v15484 * v5920) * v5925);
                    v8676 = v5926;
                    v9897 = v15492;
                }
                v8675 = v8676;
                v8679 = v8;
                v8680 = v8681;
                v8684 = v8685;
                v8689 = v8690;
                v9886 = v9897;
                v9887 = v9890;
                v9888 = v9895;
                v9889 = v9894;
            }
            let v5928 = if v5927 == v0 { 1.0 } else { 0.0 };
            let v8697: f64;
            let v9898: Lanes<5>;
            if v5928 != 0.0 {
                v8697 = v0;
                v9898 = v10571;
            } else {
                let v15493 = v9402 * v5929;
                let v15495 = (Lanes([v15493[0], v15493[1], 0.0])) - v9404;
                let v5937 = v4 / v117;
                let v5938 = (((v5929 * (v818 + v5930)) - v825) + (v1133 * v5934)) * v5937;
                let v15499 = ((Lanes([v15495[0], v15495[1], 0.0, v15495[2], 0.0])) + (v10780 * v5934)) * v5937;
                let v15500 = v15499 * v5938;
                let v5942 = ((v5938 * v5938) + v5940).sqrt();
                let v15506 = (v15499 + ((v15500 + v15500) * (v9360 / (v10427 * v5942)))) * v8;
                let v5946 = (v8 * (v5938 + v5942)) + v5945;
                let v5947 = if v5946 < v0 { 1.0 } else { 0.0 };
                let v5948: f64;
                let v9899: Lanes<5>;
                if v5947 != 0.0 {
                    v5948 = v0;
                    v9899 = v10571;
                } else {
                    v5948 = v5946;
                    v9899 = v15506;
                }
                let v5949 = v5948 + v358;
                let v5950 = v4 / v5949;
                let v5952 = -v5951;
                let v5953 = v5952 * v714;
                let v5954 = v5953 * v5950;
                let v15511 = (v10433 * v5952) * v5950;
                let v15514 = (Lanes([0.0, 0.0, v15511[0], 0.0, 0.0])) + ((((v9899 * v5950) * v10382) / v5949) * v5953);
                let v5956 = if v5954 < v5955 { 1.0 } else { 0.0 };
                let v5972: f64;
                let v9900: Lanes<5>;
                if v5956 != 0.0 {
                    v5972 = v0;
                    v9900 = v10571;
                } else {
                    let v5957 = v5954.exp();
                    let v5959 = v5958 / v713;
                    let v5961 = (v5959 * v202) * v162;
                    let v5962 = v5961 * v5948;
                    let v15521 = (((((v10430 * v5959) * v10382) / v713) * v202) * v162) * v5948;
                    let v5963 = v5962 * v5948;
                    let v5964 = v5963 * v5957;
                    let v15530 = (((((Lanes([0.0, 0.0, v15521[0], 0.0, 0.0])) + (v9899 * v5961)) * v5948) + (v9899 * v5962)) * v5957) + ((v15514 * v5957) * v5963);
                    v5972 = v5964;
                    v9900 = v15530;
                }
                let v5965 = v818 - v873;
                let v15531 = v10552 - v9407;
                let v5966 = if v5965 > v0 { 1.0 } else { 0.0 };
                let v8698: f64;
                let v9901: Lanes<5>;
                if v5966 != 0.0 {
                    let v5967 = v5965 * v5965;
                    let v15532 = v15531 * v5965;
                    let v5968 = v5967 * v5965;
                    let v15536 = ((v15532 + v15532) * v5965) + (v15531 * v5967);
                    let v5970 = v5968 + v5969;
                    let v5971 = v5968 / v5970;
                    let v5973 = v5972 * v5971;
                    let v15541 = ((v15536 - (v15536 * v5971)) / v5970) * v5972;
                    let v15543 = (v9900 * v5971) + (Lanes([v15541[0], v15541[1], 0.0, 0.0, v15541[2]]));
                    v8698 = v5973;
                    v9901 = v15543;
                } else {
                    v8698 = v0;
                    v9901 = v10571;
                }
                v8697 = v8698;
                v9898 = v9901;
            }
            let v8699: f64;
            let v9902: Lanes<5>;
            if v5928 != 0.0 {
                v8699 = v0;
                v9902 = v10571;
            } else {
                let v15545 = (v9402 * v10382) * v5929;
                let v15549 = (Lanes([v15545[0], v15545[1], 0.0])) - (v9404 - (Lanes([v9402[0], v9402[1], 0.0])));
                let v5981 = v4 / v117;
                let v5982 = (((v5929 * ((-v818) + v5930)) - (v825 - v818)) + (v1133 * v5934)) * v5981;
                let v15553 = ((Lanes([v15549[0], v15549[1], 0.0, v15549[2], 0.0])) + (v10780 * v5934)) * v5981;
                let v15554 = v15553 * v5982;
                let v5986 = ((v5982 * v5982) + v5984).sqrt();
                let v15560 = (v15553 + ((v15554 + v15554) * (v9360 / (v10427 * v5986)))) * v8;
                let v5990 = (v8 * (v5982 + v5986)) + v5989;
                let v5991 = if v5990 < v0 { 1.0 } else { 0.0 };
                let v5992: f64;
                let v9903: Lanes<5>;
                if v5991 != 0.0 {
                    v5992 = v0;
                    v9903 = v10571;
                } else {
                    v5992 = v5990;
                    v9903 = v15560;
                }
                let v5993 = v5992 + v358;
                let v5994 = v4 / v5993;
                let v5995 = -v5951;
                let v5996 = v5995 * v714;
                let v5997 = v5996 * v5994;
                let v15565 = (v10433 * v5995) * v5994;
                let v15568 = (Lanes([0.0, 0.0, v15565[0], 0.0, 0.0])) + ((((v9903 * v5994) * v10382) / v5993) * v5996);
                let v5999 = if v5997 < v5998 { 1.0 } else { 0.0 };
                let v6014: f64;
                let v9904: Lanes<5>;
                if v5999 != 0.0 {
                    v6014 = v0;
                    v9904 = v10571;
                } else {
                    let v6000 = v5997.exp();
                    let v6001 = v4 / v713;
                    let v6004 = ((v5958 * v6001) * v202) * v162;
                    let v6005 = v6004 * v5992;
                    let v15576 = ((((((v10430 * v6001) * v10382) / v713) * v5958) * v202) * v162) * v5992;
                    let v6006 = v6005 * v5992;
                    let v6007 = v6006 * v6000;
                    let v15585 = (((((Lanes([0.0, 0.0, v15576[0], 0.0, 0.0])) + (v9903 * v6004)) * v5992) + (v9903 * v6005)) * v6000) + ((v15568 * v6000) * v6006);
                    v6014 = v6007;
                    v9904 = v15585;
                }
                let v6008 = -v873;
                let v15586 = v9407 * v10382;
                let v6009 = if v6008 > v0 { 1.0 } else { 0.0 };
                let v8700: f64;
                let v9905: Lanes<5>;
                if v6009 != 0.0 {
                    let v6010 = v6008 * v6008;
                    let v15587 = v15586 * v6008;
                    let v6011 = v6010 * v6008;
                    let v15591 = ((v15587 + v15587) * v6008) + (v15586 * v6010);
                    let v6012 = v6011 + v5969;
                    let v6013 = v6011 / v6012;
                    let v6015 = v6014 * v6013;
                    let v15596 = ((v15591 - (v15591 * v6013)) / v6012) * v6014;
                    let v15598 = (v9904 * v6013) + (Lanes([v15596[0], v15596[1], 0.0, 0.0, v15596[2]]));
                    v8700 = v6015;
                    v9905 = v15598;
                } else {
                    v8700 = v0;
                    v9905 = v10571;
                }
                v8699 = v8700;
                v9902 = v9905;
            }
            let v8534: f64;
            let v8542: f64;
            let v8550: f64;
            let v8562: f64;
            let v8574: f64;
            let v8581: f64;
            let v8591: f64;
            let v8598: f64;
            let v9906: Lanes<5>;
            let v9907: Lanes<5>;
            let v9908: Lanes<6>;
            let v9909: Lanes<6>;
            let v9910: Lanes<5>;
            let v9911: Lanes<6>;
            let v9912: Lanes<5>;
            let v9913: Lanes<6>;
            if v5 != 0.0 {
                let v6016 = v4 / v122;
                let v6017 = -v3856;
                let v6018 = v6017 * v4423;
                let v15599 = v9437 * v6017;
                let v6021 = v6018 + (v6017 * v6019);
                let v15601 = v15599 + (v9824 * v6017);
                let v6022 = v6018 * v8;
                let v15602 = v15599 * v8;
                let v6023 = v6018 - v6022;
                let v15603 = v15599 - v15602;
                let v6024 = v6021 * v8;
                let v15604 = v15601 * v8;
                let v6025 = v6021 - v6024;
                let v15605 = v15601 - v15604;
                let v8535: f64;
                let v8543: f64;
                let v8551: f64;
                let v8563: f64;
                let v8575: f64;
                let v8582: f64;
                let v8592: f64;
                let v8599: f64;
                let v9914: Lanes<5>;
                let v9915: Lanes<5>;
                let v9916: Lanes<6>;
                let v9917: Lanes<6>;
                let v9918: Lanes<5>;
                let v9919: Lanes<6>;
                let v9920: Lanes<5>;
                let v9921: Lanes<6>;
                if v561 != 0.0 {
                    let v6033: f64;
                    let v6093: f64;
                    let v6451: f64;
                    if v6026 != 0.0 {
                        let v6029 = v6027 * v8;
                        v6033 = v367;
                        v6093 = v6030;
                        v6451 = v6029;
                    } else {
                        let v6034: f64;
                        let v6094: f64;
                        let v6452: f64;
                        if v6031 != 0.0 {
                            let v6032 = v3856 * v8;
                            v6034 = v4;
                            v6094 = v236;
                            v6452 = v6032;
                        } else {
                            v6034 = v0;
                            v6094 = v0;
                            v6452 = v0;
                        }
                        v6033 = v6034;
                        v6093 = v6094;
                        v6451 = v6452;
                    }
                    let v6035 = if v6033 == v0 { 1.0 } else { 0.0 };
                    let v8536: f64;
                    let v8544: f64;
                    let v8552: f64;
                    let v8564: f64;
                    let v8576: f64;
                    let v8583: f64;
                    let v8593: f64;
                    let v8600: f64;
                    let v9922: Lanes<5>;
                    let v9923: Lanes<5>;
                    let v9924: Lanes<6>;
                    let v9925: Lanes<6>;
                    let v9926: Lanes<5>;
                    let v9927: Lanes<6>;
                    let v9928: Lanes<5>;
                    let v9929: Lanes<6>;
                    if v6035 != 0.0 {
                        let v6037 = (v485 / v485).sqrt();
                        let v6038 = v745 * v6037;
                        let v15606 = v10477 * v6037;
                        let v6046 = (v6041 * v830) + (v6043 * (v830 - v818));
                        let v15610 = (v9405 * v6041) + ((v9405 - v10552) * v6043);
                        let v15614 = (v9402 * v6041) + ((v9402 * v10382) * v6043);
                        let v6052 = v825 - v818;
                        let v15617 = v9404 - (Lanes([v9402[0], v9402[1], 0.0]));
                        let v6054 = (v6041 * v825) + (v6043 * v6052);
                        let v15619 = (v9404 * v6041) + (v15617 * v6043);
                        let v6057 = (v6043 * v825) + (v6041 * v6052);
                        let v15622 = (v9404 * v6043) + (v15617 * v6041);
                        let v6058 = ((v6041 * v818) + (v6043 * (-v818))) - v6046;
                        let v15624 = (Lanes([v15614[0], v15614[1], 0.0])) - v15610;
                        let v6059 = -v6046;
                        let v15625 = v15610 * v10382;
                        let v6061 = v6041 + (v6040 * v6043);
                        let v6063 = v6043 + (v6040 * v6041);
                        let v6066 = (v6061 * v6054) + (v6063 * v6057);
                        let v15628 = (v15619 * v6061) + (v15622 * v6063);
                        let v6072 = -(((v6061 * v6059) + (v6063 * v6058)) + v6070);
                        let v15632 = ((v15625 * v6061) + (v15624 * v6063)) * v10382;
                        let v6073 = if v6072 > v778 { 1.0 } else { 0.0 };
                        let v6088: f64;
                        let v9930: Lanes<3>;
                        if v6073 != 0.0 {
                            let v6075 = v774 - v778;
                            let v6076 = (v6072 - v778) / v6075;
                            let v15633 = v15632 / v6075;
                            let v6077 = v6076 * v6076;
                            let v15634 = v15633 * v6076;
                            let v15635 = v15634 + v15634;
                            let v15639 = v15635 * v6077;
                            let v6083 = (((v4 + v6076) + v6077) + (v6077 * v6076)) + (v6077 * v6077);
                            let v6084 = v4 / v6083;
                            let v15648 = (((((((v15633 + v15635) + ((v15635 * v6076) + (v15633 * v6077))) + (v15639 + v15639)) * v6084) * v10382) / v6083) * v10382) * v6075;
                            let v6087 = v778 + (v6075 * (v4 - v6084));
                            v6088 = v6087;
                            v9930 = v15648;
                        } else {
                            v6088 = v6072;
                            v9930 = v15632;
                        }
                        let v15649 = v9930 * v10382;
                        let v6090 = (-v6088) - v6;
                        let v6091 = v6038 * v6016;
                        let v15650 = v15606 * v6016;
                        let v6092 = v6091 * v6091;
                        let v15651 = v15650 * v6091;
                        let v15652 = v15651 + v15651;
                        let v6095 = v6066 - v6093;
                        let v6096 = v485 / v726;
                        let v6097 = v73 / v658;
                        let v6098 = v6096.ln();
                        let v6099 = v6097 * v6098;
                        let v15663 = ((((v10402 * v6097) * v10382) / v658) * v6098) + (((((v10445 * v6096) * v10382) / v726) * (v9360 / v6096)) * v6097);
                        let v6100 = -v6090;
                        let v15664 = v15649 * v10382;
                        let v6101 = if v6095 < v6100 { 1.0 } else { 0.0 };
                        let v6445: f64;
                        let v6447: f64;
                        let v6824: f64;
                        let v6834: f64;
                        let v6839: f64;
                        let v9931: Lanes<5>;
                        let v9932: Lanes<5>;
                        let v9933: Lanes<5>;
                        let v9934: Lanes<5>;
                        let v9935: Lanes<5>;
                        if v6101 != 0.0 {
                            let v6102 = v658 * v6038;
                            let v6103 = v4 / v6102;
                            let v6104 = v6103 * v122;
                            let v16048 = (((((v10402 * v6038) + (v15606 * v658)) * v6103) * v10382) / v6102) * v122;
                            let v16049 = v16048 * v6105;
                            let v6107 = v73 + (v6105 * v6104);
                            let v6108 = v86 * v6107;
                            let v6109 = v6108 * v6107;
                            let v6110 = v6109 * v6107;
                            let v16056 = ((((v16049 * v86) * v6107) + (v16049 * v6108)) * v6107) + (v16049 * v6109);
                            let v6111 = v656 - v6099;
                            let v16057 = v10398 - v15663;
                            let v6112 = v6095 + v6090;
                            let v16061 = v10402 * v6112;
                            let v16062 = ((Lanes([v15628[0], v15628[1], v15628[2], 0.0])) + (Lanes([v15649[0], v15649[1], 0.0, v15649[2]]))) * v658;
                            let v6115 = v3495 * v6104;
                            let v6116 = (v658 * v6112) - v73;
                            let v6117 = v6115 * v6116;
                            let v16067 = (v16048 * v3495) * v6116;
                            let v16070 = (Lanes([0.0, 0.0, v16067[0], 0.0, 0.0])) + (((Lanes([0.0, 0.0, v16061[0], 0.0, 0.0])) + (Lanes([v16062[0], v16062[1], 0.0, v16062[2], v16062[3]]))) * v6115);
                            let v6118 = v6114 - v6117;
                            let v16071 = v16070 * v10382;
                            let v6119 = v6118 * v6118;
                            let v16072 = v16071 * v6118;
                            let v16073 = v16072 + v16072;
                            let v6121 = if v6110 < (v6119 * v3501) { 1.0 } else { 0.0 };
                            let v6133: f64;
                            let v9936: Lanes<5>;
                            if v6121 != 0.0 {
                                let v16080 = v16056 * v8;
                                let v6125 = (v8 * v6110) / v6118;
                                let v6127 = ((v6122 + v6118) + v6125) + v6117;
                                let v16086 = (v16071 + (((Lanes([0.0, 0.0, v16080[0], 0.0, 0.0])) - (v16071 * v6125)) / v6118)) + v16070;
                                v6133 = v6127;
                                v9936 = v16086;
                            } else {
                                let v6129 = (v6110 + v6119).sqrt();
                                let v6132 = (v6130 + v6129) + v6117;
                                let v16079 = (((Lanes([0.0, 0.0, v16056[0], 0.0, 0.0])) + v16073) * (v9360 / (v10427 * v6129))) + v16070;
                                v6133 = v6132;
                                v9936 = v16079;
                            }
                            let v6134 = v6133.powf(v1557);
                            let v16090 = v9936 * (v1557 * (v6133.powf(v16087)));
                            let v16092 = (v16048 * v3518) * v10382;
                            let v6140 = v743 * v6134;
                            let v6143 = (((v6135 - (v3518 * v6104)) + (v73 * v6134)) + (v6140 * v6134)) / v6134;
                            let v16105 = v10407 * v6143;
                            let v16108 = Lanes([v15649[0], v15649[1], 0.0, 0.0, v15649[2]]);
                            let v6146 = ((v6143 * v660) - v6090) + v6090;
                            let v16110 = ((((((((Lanes([0.0, 0.0, v16092[0], 0.0, 0.0])) + (v16090 * v73)) + (((v16090 * v743) * v6134) + (v16090 * v6140))) - (v16090 * v6143)) / v6134) * v660) + (Lanes([0.0, 0.0, v16105[0], 0.0, 0.0]))) - v16108) + v16108;
                            let v6147 = v6146 / v6111;
                            let v16111 = v16057 * v6147;
                            let v16115 = ((v16110 - (Lanes([0.0, 0.0, v16111[0], 0.0, 0.0]))) / v6111) * v6147;
                            let v6150 = (v4 + (v6147 * v6147)).sqrt();
                            let v6151 = v6146 / v6150;
                            let v6154 = v122 * (v6095 - (v6151 - v6090));
                            let v16126 = ((Lanes([v15628[0], v15628[1], 0.0, v15628[2], 0.0])) - (((v16110 - (((v16115 + v16115) * (v9360 / (v10427 * v6150))) * v6151)) / v6150) - v16108)) * v122;
                            v6445 = v6154;
                            v6447 = v6154;
                            v6824 = v0;
                            v6834 = v0;
                            v6839 = v0;
                            v9931 = v16126;
                            v9932 = v16126;
                            v9933 = v10571;
                            v9934 = v10571;
                            v9935 = v10571;
                        } else {
                            let v6156 = v6095 + v6090;
                            let v15667 = (Lanes([v15628[0], v15628[1], v15628[2], 0.0])) + (Lanes([v15649[0], v15649[1], 0.0, v15649[2]]));
                            let v15668 = v10402 * v6156;
                            let v15669 = v15667 * v658;
                            let v15671 = Lanes([v15669[0], v15669[1], 0.0, v15669[2], v15669[3]]);
                            let v15672 = (Lanes([0.0, 0.0, v15668[0], 0.0, 0.0])) + v15671;
                            let v6158 = (v658 * v6156) - v4;
                            let v6161 = v6092 * v659;
                            let v15676 = (v15652 * v659) + (v10404 * v6092);
                            let v6162 = (v85 * (v6158 + v6155)) / v6161;
                            let v15677 = v15676 * v6162;
                            let v15680 = ((v15672 * v85) - (Lanes([0.0, 0.0, v15677[0], 0.0, 0.0]))) / v6161;
                            let v6163 = v4 + v6162;
                            let v6165 = if v6163 < v6164 { 1.0 } else { 0.0 };
                            let v6169: f64;
                            let v9937: Lanes<5>;
                            if v6165 != 0.0 {
                                v6169 = v6166;
                                v9937 = v10571;
                            } else {
                                v6169 = v6163;
                                v9937 = v15680;
                            }
                            let v6168 = (v6092 * v658) / v73;
                            let v15684 = ((v15652 * v658) + (v10402 * v6092)) / v73;
                            let v6170 = v6169.sqrt();
                            let v6171 = v4 - v6170;
                            let v15689 = v15684 * v6171;
                            let v15693 = Lanes([v15628[0], v15628[1], 0.0, v15628[2], 0.0]);
                            let v6174 = (v6095 + (v6168 * v6171)) + v6090;
                            let v15695 = Lanes([v15649[0], v15649[1], 0.0, 0.0, v15649[2]]);
                            let v15697 = v10402 * v6174;
                            let v6177 = (-(v658 * v6174)).exp();
                            let v6180 = (v85 * (v6158 + v6177)) / v6161;
                            let v15705 = v15676 * v6180;
                            let v15708 = (((v15672 + ((((Lanes([0.0, 0.0, v15697[0], 0.0, 0.0])) + (((v15693 + ((Lanes([0.0, 0.0, v15689[0], 0.0, 0.0])) + (((v9937 * (v9360 / (v10427 * v6170))) * v10382) * v6168))) + v15695) * v658)) * v10382) * v6177)) * v85) - (Lanes([0.0, 0.0, v15705[0], 0.0, 0.0]))) / v6161;
                            let v6181 = v4 + v6180;
                            let v6183 = if v6181 < v6182 { 1.0 } else { 0.0 };
                            let v6185: f64;
                            let v9938: Lanes<5>;
                            if v6183 != 0.0 {
                                v6185 = v6184;
                                v9938 = v10571;
                            } else {
                                v6185 = v6181;
                                v9938 = v15708;
                            }
                            let v6186 = v6185.sqrt();
                            let v6187 = v4 - v6186;
                            let v15713 = v15684 * v6187;
                            let v6190 = (v6095 + (v6168 * v6187)) + v6090;
                            let v6191 = v658 * v6190;
                            let v15719 = v10402 * v6190;
                            let v15722 = (Lanes([0.0, 0.0, v15719[0], 0.0, 0.0])) + (((v15693 + ((Lanes([0.0, 0.0, v15713[0], 0.0, 0.0])) + (((v9938 * (v9360 / (v10427 * v6186))) * v10382) * v6168))) + v15695) * v658);
                            let v6192 = if v6191 < v91 { 1.0 } else { 0.0 };
                            let v6269: f64;
                            let v9939: Lanes<5>;
                            if v6192 != 0.0 {
                                let v6195 = v658 * v6091;
                                let v6196 = v4 / v6195;
                                let v15728 = ((((v10402 * v6091) + (v15650 * v658)) * v6196) * v10382) / v6195;
                                let v6197 = v6194 + v6196;
                                let v15729 = v15667 * v10382;
                                let v6199 = (-v6156) / v6091;
                                let v15730 = v15650 * v6199;
                                let v15737 = ((v15728 * v6193) / v6202) * v10382;
                                let v6207 = (v6200 - ((v6193 * v6197) / v6202)) + (v6199 / v6205);
                                let v15740 = (Lanes([0.0, 0.0, v15737[0], 0.0, 0.0])) + ((((Lanes([v15729[0], v15729[1], 0.0, v15729[2], v15729[3]])) - (Lanes([0.0, 0.0, v15730[0], 0.0, 0.0]))) / v6091) / v6205);
                                let v6213 = ((v6208 * v6197) - v6210) / v6212;
                                let v15742 = (v15728 * v6208) / v6212;
                                let v15743 = v15740 * v6207;
                                let v6215 = v6213 * v6213;
                                let v15745 = v15742 * v6213;
                                let v15749 = ((v15745 + v15745) * v6213) + (v15742 * v6215);
                                let v6218 = ((v6207 * v6207) + (v6215 * v6213)).sqrt();
                                let v15754 = ((v15743 + v15743) + (Lanes([0.0, 0.0, v15749[0], 0.0, 0.0]))) * (v9360 / (v10427 * v6218));
                                let v6220 = (-v6207) + v6218;
                                let v6222 = v6207 + v6218;
                                let v6227 = ((v6220.powf(v1557)) + (-(v6222.powf(v1557)))) - v6226;
                                let v15769 = v10407 * v6227;
                                let v6230 = ((v6227 * v660) - v6090) + v6090;
                                let v6231 = v658 * v6230;
                                let v15774 = v10402 * v6230;
                                let v15777 = (Lanes([0.0, 0.0, v15774[0], 0.0, 0.0])) + (((((((((v15740 * v10382) + v15754) * (v1557 * (v6220.powf(v15757)))) + (((v15740 + v15754) * (v1557 * (v6222.powf(v15762)))) * v10382)) * v660) + (Lanes([0.0, 0.0, v15769[0], 0.0, 0.0]))) - v15695) + v15695) * v658);
                                v6269 = v6231;
                                v9939 = v15777;
                            } else {
                                v6269 = v6191;
                                v9939 = v15722;
                            }
                            let v6232 = v6156 + v74;
                            let v15778 = v10402 * v6100;
                            let v15779 = v15664 * v658;
                            let v6234 = (v658 * v6100).exp();
                            let v15783 = ((Lanes([0.0, 0.0, v15778[0], 0.0])) + (Lanes([v15779[0], v15779[1], 0.0, v15779[2]]))) * v6234;
                            let v6235 = v6234 + v358;
                            let v6236 = v726 / v485;
                            let v6237 = v6236 * v6236;
                            let v15785 = (v10445 / v485) * v6236;
                            let v15786 = v15785 + v15785;
                            let v6238 = v6237 * v6235;
                            let v15787 = v15786 * v6235;
                            let v15788 = v15783 * v6237;
                            let v6239 = v658 * v6232;
                            let v15791 = v10402 * v6232;
                            let v15793 = (Lanes([0.0, 0.0, v15791[0], 0.0, 0.0])) + v15671;
                            let v6240 = v6238 * v6161;
                            let v15795 = v15676 * v6238;
                            let v15797 = (((Lanes([0.0, 0.0, v15787[0], 0.0])) + v15788) * v6161) + (Lanes([0.0, 0.0, v15795[0], 0.0]));
                            let v15798 = v15793 * v6239;
                            let v6242 = v6240 + (v6239 * v6239);
                            let v15800 = Lanes([v15797[0], v15797[1], v15797[2], 0.0, v15797[3]]);
                            let v6244 = v6237 * v6161;
                            let v6245 = v6244.ln();
                            let v15808 = ((v15786 * v6161) + (v15676 * v6237)) * (v9360 / v6244);
                            let v15809 = Lanes([0.0, 0.0, v15808[0], 0.0, 0.0]);
                            let v6247 = v658 * v6090;
                            let v15811 = v10402 * v6090;
                            let v15812 = v15649 * v658;
                            let v15815 = (Lanes([0.0, 0.0, v15811[0], 0.0])) + (Lanes([v15812[0], v15812[1], 0.0, v15812[2]]));
                            let v15816 = Lanes([v15815[0], v15815[1], v15815[2], 0.0, v15815[3]]);
                            let v15818 = v15793 - ((((v15800 + (v15798 + v15798)) * (v9360 / v6242)) - v15809) + v15816);
                            let v6250 = (v6239 - (((v6242.ln()) - v6245) + v6247)) - v4;
                            let v6251 = v85 * v6239;
                            let v15819 = v15793 * v85;
                            let v6252 = if v6251 > v0 { 1.0 } else { 0.0 };
                            let v6254: f64;
                            let v9940: Lanes<5>;
                            if v6252 != 0.0 {
                                v6254 = v6251;
                                v9940 = v15819;
                            } else {
                                let v6253 = -v6251;
                                let v15820 = v15819 * v10382;
                                v6254 = v6253;
                                v9940 = v15820;
                            }
                            let v15821 = v15818 * v6250;
                            let v6257 = ((v6250 * v6250) + v6254).sqrt();
                            let v15831 = v10402 * v74;
                            let v6263 = (v6239 - (v6239 - (v8 * (v6250 + v6257)))) + (v658 * v74);
                            let v15834 = ((v15793 - (v15793 - ((v15818 + (((v15821 + v15821) + v9940) * (v9360 / (v10427 * v6257)))) * v8))) + (Lanes([0.0, 0.0, v15831[0], 0.0, 0.0]))) * v6263;
                            let v6265 = v6240 + (v6263 * v6263);
                            let v6268 = ((v6265.ln()) - v6245) + v6247;
                            let v15840 = (((v15800 + (v15834 + v15834)) * (v9360 / v6265)) - v15809) + v15816;
                            let v15841 = v15840 - v9939;
                            let v6272 = (v6268 - v6269) - v6271;
                            let v6275 = (v85 * v6268) * v6274;
                            let v15843 = (v15840 * v85) * v6274;
                            let v6276 = if v6275 > v0 { 1.0 } else { 0.0 };
                            let v6278: f64;
                            let v9941: Lanes<5>;
                            if v6276 != 0.0 {
                                v6278 = v6275;
                                v9941 = v15843;
                            } else {
                                let v6277 = -v6275;
                                let v15844 = v15843 * v10382;
                                v6278 = v6277;
                                v9941 = v15844;
                            }
                            let v15845 = v15841 * v6272;
                            let v6281 = ((v6272 * v6272) + v6278).sqrt();
                            let v6284 = v6268 - (v8 * (v6272 + v6281));
                            let v15853 = v15840 - ((v15841 + (((v15845 + v15845) + v9941) * (v9360 / (v10427 * v6281)))) * v8);
                            let v6285 = v6284 / v658;
                            let v15854 = v10402 * v6285;
                            let v6286 = v6285 - v6090;
                            let v15858 = ((v15853 - (Lanes([0.0, 0.0, v15854[0], 0.0, 0.0]))) / v658) - v15695;
                            let v6289 = (-v6284).exp();
                            let v6290 = (v6284 - v4) + v6289;
                            let v15861 = v15853 + ((v15853 * v10382) * v6289);
                            let v6292 = if v6290 < v6291 { 1.0 } else { 0.0 };
                            let v6294: f64;
                            let v9942: Lanes<5>;
                            if v6292 != 0.0 {
                                v6294 = v6293;
                                v9942 = v10571;
                            } else {
                                v6294 = v6290;
                                v9942 = v15861;
                            }
                            let v6295 = v6294.sqrt();
                            let v6296 = v6038 * v6295;
                            let v15865 = v15606 * v6295;
                            let v15868 = (Lanes([0.0, 0.0, v15865[0], 0.0, 0.0])) + ((v9942 * (v9360 / (v10427 * v6295))) * v6038);
                            let v6298 = v122 * (v6095 - v6286);
                            let v15870 = (v15693 - v15858) * v122;
                            let v6300 = if v6299 == v4 { 1.0 } else { 0.0 };
                            let v6446: f64;
                            let v6448: f64;
                            let v6825: f64;
                            let v6835: f64;
                            let v6840: f64;
                            let v9943: Lanes<5>;
                            let v9944: Lanes<5>;
                            let v9945: Lanes<5>;
                            let v9946: Lanes<5>;
                            let v9947: Lanes<5>;
                            if v6300 != 0.0 {
                                let v6301 = v6237 * v6234;
                                let v15871 = v15786 * v6234;
                                let v15873 = (Lanes([0.0, 0.0, v15871[0], 0.0])) + v15788;
                                let mut v6302: f64 = 0.0;
                                let mut v6305: f64 = 0.0;
                                let mut v6396: f64 = 0.0;
                                let mut v6426: f64 = 0.0;
                                let mut v6429: f64 = 0.0;
                                let mut v6437: f64 = 0.0;
                                let mut v6440: f64 = 0.0;
                                let mut v9948: Lanes<5> = Lanes([0.0; 5]);
                                let mut v9949: Lanes<5> = Lanes([0.0; 5]);
                                let mut v9950: Lanes<5> = Lanes([0.0; 5]);
                                let mut v9951: Lanes<5> = Lanes([0.0; 5]);
                                let mut v9952: Lanes<5> = Lanes([0.0; 5]);
                                v6302 = v4;
                                v6305 = v6286;
                                v6396 = v0;
                                v6426 = v6284;
                                v6429 = v0;
                                v6437 = v0;
                                v6440 = v0;
                                v9948 = v15858;
                                v9949 = v15853;
                                v9950 = v10571;
                                v9951 = v10571;
                                v9952 = v10571;
                                loop {
                                    let v6304 = if v6302 <= v6303 { 1.0 } else { 0.0 };
                                    if v6304 == 0.0 {
                                        break;
                                    }
                                    let v6306 = v6305 + v6090;
                                    let v6307 = v658 * v6306;
                                    let v15894 = v10402 * v6306;
                                    let v15897 = (Lanes([0.0, 0.0, v15894[0], 0.0, 0.0])) + ((v9948 + v15695) * v658);
                                    let v6308 = if v6307 < v639 { 1.0 } else { 0.0 };
                                    let v6389: f64;
                                    let v6393: f64;
                                    let v6430: f64;
                                    let v6441: f64;
                                    let v9953: Lanes<5>;
                                    let v9954: Lanes<5>;
                                    let v9955: Lanes<5>;
                                    let v9956: Lanes<5>;
                                    if v6308 != 0.0 {
                                        let v6309 = v6307 * v6307;
                                        let v15939 = v15897 * v6307;
                                        let v15940 = v15939 + v15939;
                                        let v6310 = v6309 * v6307;
                                        let v6315 = v6312 + (v6307 * v6313);
                                        let v6317 = v6311 + (v6307 * v6315);
                                        let v6318 = v6310 * v6317;
                                        let v15950 = (((v15940 * v6307) + (v15897 * v6309)) * v6317) + (((v15897 * v6315) + ((v15897 * v6313) * v6307)) * v6310);
                                        let v6321 = v6307 * v639;
                                        let v15951 = v15897 * v639;
                                        let v6323 = v6320 + (v6321 * v6313);
                                        let v6325 = v6319 + (v6307 * v6323);
                                        let v6326 = v6309 * v6325;
                                        let v6327 = v6301 * v6318;
                                        let v15959 = v15873 * v6318;
                                        let v6328 = v6327 * v6318;
                                        let v15965 = (((Lanes([v15959[0], v15959[1], v15959[2], 0.0, v15959[3]])) + (v15950 * v6301)) * v6318) + (v15950 * v6327);
                                        let v15967 = v10402 * v6301;
                                        let v6330 = (v6301 * v658) * v73;
                                        let v6331 = v6330 * v6318;
                                        let v15971 = (((v15873 * v658) + (Lanes([0.0, 0.0, v15967[0], 0.0]))) * v73) * v6318;
                                        let v6339 = v6336 + (v6307 * v6337);
                                        let v6341 = v6335 + (v6307 * v6339);
                                        let v6343 = v6334 + (v6307 * v6341);
                                        let v6345 = v6333 + (v6307 * v6343);
                                        let v6346 = v6307 * v6345;
                                        let v15990 = (v15897 * v6345) + (((v15897 * v6343) + (((v15897 * v6341) + (((v15897 * v6339) + ((v15897 * v6337) * v6307)) * v6307)) * v6307)) * v6307);
                                        let v6351 = v6349 + (v6321 * v6337);
                                        let v6353 = v6348 + (v6307 * v6351);
                                        let v6355 = v6347 + (v6307 * v6353);
                                        let v6357 = v6333 + (v6307 * v6355);
                                        let v16001 = v15990 * v6346;
                                        let v6361 = (((v6346 * v6346) + v6328) + v358).sqrt();
                                        let v16006 = ((v16001 + v16001) + v15965) * (v9360 / (v10427 * v6361));
                                        let v16007 = v10402 * v6357;
                                        let v6363 = (v658 * v6357) * v73;
                                        let v6366 = v6361 + v6361;
                                        let v6367 = ((v6363 * v6346) + (v6331 * v6326)) / v6366;
                                        let v16019 = (((((((Lanes([0.0, 0.0, v16007[0], 0.0, 0.0])) + (((v15897 * v6355) + (((v15897 * v6353) + (((v15897 * v6351) + ((v15951 * v6337) * v6307)) * v6307)) * v6307)) * v658)) * v73) * v6346) + (v15990 * v6363)) + ((((Lanes([v15971[0], v15971[1], v15971[2], 0.0, v15971[3]])) + (v15950 * v6330)) * v6326) + (((v15940 * v6325) + (((v15897 * v6323) + ((v15951 * v6313) * v6307)) * v6309)) * v6331))) - ((v16006 + v16006) * v6367)) / v6366;
                                        v6389 = v6361;
                                        v6393 = v6367;
                                        v6430 = v6346;
                                        v6441 = v6328;
                                        v9953 = v16006;
                                        v9954 = v16019;
                                        v9955 = v15990;
                                        v9956 = v15965;
                                    } else {
                                        let v6368 = if v6307 < v2530 { 1.0 } else { 0.0 };
                                        let v6381: f64;
                                        let v6384: f64;
                                        let v9957: Lanes<5>;
                                        let v9958: Lanes<5>;
                                        if v6368 != 0.0 {
                                            let v6369 = v6307.exp();
                                            let v15916 = v15897 * v6369;
                                            let v6370 = v6369 - v4;
                                            let v6371 = v6301 * v6370;
                                            let v15917 = v15873 * v6370;
                                            let v15920 = (Lanes([v15917[0], v15917[1], v15917[2], 0.0, v15917[3]])) + (v15916 * v6301);
                                            let v6372 = v6301 * v658;
                                            let v15922 = v10402 * v6301;
                                            let v6373 = v6372 * v6369;
                                            let v15925 = ((v15873 * v658) + (Lanes([0.0, 0.0, v15922[0], 0.0]))) * v6369;
                                            let v15928 = (Lanes([v15925[0], v15925[1], v15925[2], 0.0, v15925[3]])) + (v15916 * v6372);
                                            v6381 = v6371;
                                            v6384 = v6373;
                                            v9957 = v15920;
                                            v9958 = v15928;
                                        } else {
                                            let v15898 = v10402 * v6305;
                                            let v6375 = (v658 * v6305).exp();
                                            let v15902 = ((Lanes([0.0, 0.0, v15898[0], 0.0, 0.0])) + (v9948 * v658)) * v6375;
                                            let v6376 = v6375 - v6234;
                                            let v6377 = v6237 * v6376;
                                            let v15905 = v15786 * v6376;
                                            let v15908 = (Lanes([0.0, 0.0, v15905[0], 0.0, 0.0])) + ((v15902 - (Lanes([v15783[0], v15783[1], v15783[2], 0.0, v15783[3]]))) * v6237);
                                            let v6378 = v6237 * v658;
                                            let v6379 = v6378 * v6375;
                                            let v15912 = ((v15786 * v658) + (v10402 * v6237)) * v6375;
                                            let v15915 = (Lanes([0.0, 0.0, v15912[0], 0.0, 0.0])) + (v15902 * v6378);
                                            v6381 = v6377;
                                            v6384 = v6379;
                                            v9957 = v15908;
                                            v9958 = v15915;
                                        }
                                        let v6383 = ((v6307 - v4) + v6381).sqrt();
                                        let v15932 = (v15897 + v9957) * (v9360 / (v10427 * v6383));
                                        let v6386 = (v658 + v6384) / v6383;
                                        let v6387 = v6386 * v8;
                                        let v15938 = ((((Lanes([0.0, 0.0, v10402[0], 0.0, 0.0])) + v9958) - (v15932 * v6386)) / v6383) * v8;
                                        v6389 = v6383;
                                        v6393 = v6387;
                                        v6430 = v0;
                                        v6441 = v6381;
                                        v9953 = v15932;
                                        v9954 = v15938;
                                        v9955 = v10571;
                                        v9956 = v9957;
                                    }
                                    let v16021 = v15650 * v6389;
                                    let v6391 = (v6095 - v6305) - (v6091 * v6389);
                                    let v16025 = (v15693 - v9948) - ((Lanes([0.0, 0.0, v16021[0], 0.0, 0.0])) + (v9953 * v6091));
                                    let v16026 = v15650 * v6393;
                                    let v6395 = v6392 - (v6091 * v6393);
                                    let v16030 = ((Lanes([0.0, 0.0, v16026[0], 0.0, 0.0])) + (v9954 * v6091)) * v10382;
                                    let v6397 = if v6396 == v4 { 1.0 } else { 0.0 };
                                    let v6420: f64;
                                    let v6422: f64;
                                    let v6423: f64;
                                    let v9959: Lanes<5>;
                                    if v6397 != 0.0 {
                                        v6420 = v6398;
                                        v6422 = v6305;
                                        v6423 = v6396;
                                        v9959 = v9948;
                                    } else {
                                        let v6400 = (-v6391) / v6395;
                                        let v16034 = ((v16025 * v10382) - (v16030 * v6400)) / v6395;
                                        let v6402 = v6305.abs();
                                        let v16038 = v9948 * ((v10427 * (if v6305 >= v11296 { 1.0 } else { 0.0 })) - v9360);
                                        let v6403 = if v4 >= v6402 { 1.0 } else { 0.0 };
                                        let v6404: f64;
                                        let v9960: Lanes<5>;
                                        if v6403 != 0.0 {
                                            v6404 = v4;
                                            v9960 = v10571;
                                        } else {
                                            v6404 = v6402;
                                            v9960 = v16038;
                                        }
                                        let v6406 = v6401 * (v4 + v6404);
                                        let v16039 = v9960 * v6401;
                                        let v6408 = if (v6400.abs()) > v6406 { 1.0 } else { 0.0 };
                                        let v6413: f64;
                                        let v9961: Lanes<5>;
                                        if v6408 != 0.0 {
                                            let v6409 = if v6400 >= v0 { 1.0 } else { 0.0 };
                                            let v6411: f64;
                                            if v6409 != 0.0 {
                                                v6411 = v4;
                                            } else {
                                                v6411 = v6410;
                                            }
                                            let v6412 = v6406 * v6411;
                                            let v16040 = v16039 * v6411;
                                            v6413 = v6412;
                                            v9961 = v16040;
                                        } else {
                                            v6413 = v6400;
                                            v9961 = v16034;
                                        }
                                        let v6414 = v6305 + v6413;
                                        let v16041 = v9948 + v9961;
                                        let v6419 = if (if (v6413.abs()) <= v856 { 1.0 } else { 0.0 }) != 0.0 && (if (v6391.abs()) <= v3501 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v6424: f64;
                                        if v6419 != 0.0 {
                                            v6424 = v4;
                                        } else {
                                            v6424 = v6396;
                                        }
                                        v6420 = v6302;
                                        v6422 = v6414;
                                        v6423 = v6424;
                                        v9959 = v16041;
                                    }
                                    let v6421 = v6420 + v4;
                                    v6302 = v6421;
                                    v6305 = v6422;
                                    v6396 = v6423;
                                    v6426 = v6307;
                                    v6429 = v6430;
                                    v6437 = v6389;
                                    v6440 = v6441;
                                    v9948 = v9959;
                                    v9949 = v15897;
                                    v9950 = v9955;
                                    v9951 = v9953;
                                    v9952 = v9956;
                                }
                                let v6425 = if v6396 == v0 { 1.0 } else { 0.0 };
                                if v6425 != 0.0 {
                                } else {
                                }
                                let v6427 = if v6426 < v639 { 1.0 } else { 0.0 };
                                let v6435: f64;
                                let v9962: Lanes<5>;
                                if v6427 != 0.0 {
                                    let v6428 = if v6426 < v91 { 1.0 } else { 0.0 };
                                    if v6428 != 0.0 {
                                    } else {
                                    }
                                    let v6432 = v6429 + v6431;
                                    v6435 = v6432;
                                    v9962 = v9950;
                                } else {
                                    let v6434 = (v6426 - v4).sqrt();
                                    let v15876 = v9949 * (v9360 / (v10427 * v6434));
                                    v6435 = v6434;
                                    v9962 = v15876;
                                }
                                let v6436 = v6038 * v6435;
                                let v15877 = v15606 * v6435;
                                let v15880 = (Lanes([0.0, 0.0, v15877[0], 0.0, 0.0])) + (v9962 * v6038);
                                let v6438 = v6437 + v6435;
                                let v6439 = v4 / v6438;
                                let v6442 = v6038 * v6440;
                                let v15885 = v15606 * v6440;
                                let v6444 = v6436 + (v6442 * v6439);
                                let v15892 = v15880 + ((((Lanes([0.0, 0.0, v15885[0], 0.0, 0.0])) + (v9952 * v6038)) * v6439) + (((((v9951 + v9962) * v6439) * v10382) / v6438) * v6442));
                                v6446 = v6444;
                                v6448 = v6436;
                                v6825 = v6429;
                                v6835 = v6437;
                                v6840 = v6440;
                                v9943 = v15892;
                                v9944 = v15880;
                                v9945 = v9950;
                                v9946 = v9951;
                                v9947 = v9952;
                            } else {
                                v6446 = v6298;
                                v6448 = v6296;
                                v6825 = v0;
                                v6835 = v0;
                                v6840 = v0;
                                v9943 = v15870;
                                v9944 = v15868;
                                v9945 = v10571;
                                v9946 = v10571;
                                v9947 = v10571;
                            }
                            v6445 = v6446;
                            v6447 = v6448;
                            v6824 = v6825;
                            v6834 = v6835;
                            v6839 = v6840;
                            v9931 = v9943;
                            v9932 = v9944;
                            v9933 = v9945;
                            v9934 = v9946;
                            v9935 = v9947;
                        }
                        let v6449 = v6445 - v6447;
                        let v16127 = v9931 - v9932;
                        let v8539: f64;
                        let v8547: f64;
                        let v8554: f64;
                        let v8566: f64;
                        let v8579: f64;
                        let v8585: f64;
                        let v8596: f64;
                        let v8602: f64;
                        let v9963: Lanes<5>;
                        let v9964: Lanes<5>;
                        let v9965: Lanes<6>;
                        let v9966: Lanes<6>;
                        let v9967: Lanes<5>;
                        let v9968: Lanes<6>;
                        let v9969: Lanes<5>;
                        let v9970: Lanes<6>;
                        if v6450 != 0.0 {
                            let v8540: f64;
                            let v8597: f64;
                            let v9971: Lanes<5>;
                            let v9972: Lanes<5>;
                            if v6039 != 0.0 {
                                let v6453 = -v6451;
                                let v6454 = v6453 * v6445;
                                let v16136 = v9931 * v6453;
                                let v6455 = v6453 * v6449;
                                let v16137 = v16127 * v6453;
                                v8540 = v6454;
                                v8597 = v6455;
                                v9971 = v16136;
                                v9972 = v16137;
                            } else {
                                v8540 = v0;
                                v8597 = v0;
                                v9971 = v10571;
                                v9972 = v10571;
                            }
                            let v8548: f64;
                            let v8580: f64;
                            let v9973: Lanes<5>;
                            let v9974: Lanes<5>;
                            if v6040 != 0.0 {
                                let v6456 = -v6451;
                                let v6457 = v6456 * v6445;
                                let v16138 = v9931 * v6456;
                                let v6458 = v6456 * v6449;
                                let v16139 = v16127 * v6456;
                                v8548 = v6457;
                                v8580 = v6458;
                                v9973 = v16138;
                                v9974 = v16139;
                            } else {
                                v8548 = v0;
                                v8580 = v0;
                                v9973 = v10571;
                                v9974 = v10571;
                            }
                            v8539 = v8540;
                            v8547 = v8548;
                            v8554 = v6025;
                            v8566 = v6024;
                            v8579 = v8580;
                            v8585 = v6022;
                            v8596 = v8597;
                            v8602 = v6023;
                            v9963 = v9971;
                            v9964 = v9973;
                            v9965 = v15605;
                            v9966 = v15604;
                            v9967 = v9974;
                            v9968 = v15602;
                            v9969 = v9972;
                            v9970 = v15603;
                        } else {
                            let v8555: f64;
                            let v8567: f64;
                            let v8586: f64;
                            let v8603: f64;
                            let v9975: Lanes<6>;
                            let v9976: Lanes<6>;
                            let v9977: Lanes<6>;
                            let v9978: Lanes<6>;
                            if v6459 != 0.0 {
                                let v8556: f64;
                                let v8604: f64;
                                let v9979: Lanes<6>;
                                let v9980: Lanes<6>;
                                if v6039 != 0.0 {
                                    let v6460 = -v6451;
                                    let v6461 = v6460 * v6445;
                                    let v16128 = v9931 * v6460;
                                    let v6462 = v6460 * v6449;
                                    let v16129 = v16127 * v6460;
                                    let v16130 = Lanes([v16128[0], v16128[1], v16128[2], v16128[3], v16128[4], 0.0]);
                                    let v16131 = Lanes([v16129[0], v16129[1], v16129[2], v16129[3], v16129[4], 0.0]);
                                    v8556 = v6461;
                                    v8604 = v6462;
                                    v9979 = v16130;
                                    v9980 = v16131;
                                } else {
                                    v8556 = v6025;
                                    v8604 = v6023;
                                    v9979 = v15605;
                                    v9980 = v15603;
                                }
                                let v8568: f64;
                                let v8587: f64;
                                let v9981: Lanes<6>;
                                let v9982: Lanes<6>;
                                if v6040 != 0.0 {
                                    let v6463 = -v6451;
                                    let v6464 = v6463 * v6445;
                                    let v16132 = v9931 * v6463;
                                    let v6465 = v6463 * v6449;
                                    let v16133 = v16127 * v6463;
                                    let v16134 = Lanes([v16132[0], v16132[1], v16132[2], v16132[3], v16132[4], 0.0]);
                                    let v16135 = Lanes([v16133[0], v16133[1], v16133[2], v16133[3], v16133[4], 0.0]);
                                    v8568 = v6464;
                                    v8587 = v6465;
                                    v9981 = v16134;
                                    v9982 = v16135;
                                } else {
                                    v8568 = v6024;
                                    v8587 = v6022;
                                    v9981 = v15604;
                                    v9982 = v15602;
                                }
                                v8555 = v8556;
                                v8567 = v8568;
                                v8586 = v8587;
                                v8603 = v8604;
                                v9975 = v9979;
                                v9976 = v9981;
                                v9977 = v9982;
                                v9978 = v9980;
                            } else {
                                v8555 = v6025;
                                v8567 = v6024;
                                v8586 = v6022;
                                v8603 = v6023;
                                v9975 = v15605;
                                v9976 = v15604;
                                v9977 = v15602;
                                v9978 = v15603;
                            }
                            v8539 = v0;
                            v8547 = v0;
                            v8554 = v8555;
                            v8566 = v8567;
                            v8579 = v0;
                            v8585 = v8586;
                            v8596 = v0;
                            v8602 = v8603;
                            v9963 = v10571;
                            v9964 = v10571;
                            v9965 = v9975;
                            v9966 = v9976;
                            v9967 = v10571;
                            v9968 = v9977;
                            v9969 = v10571;
                            v9970 = v9978;
                        }
                        let v6469 = (v6466 * v6041) + v6043;
                        let v6471 = (v6466 * v6043) + v6041;
                        let v6474 = (v6469 * v6054) + (v6471 * v6057);
                        let v16142 = (v15619 * v6469) + (v15622 * v6471);
                        let v6480 = -(((v6469 * v6059) + (v6471 * v6058)) + v6478);
                        let v16146 = ((v15625 * v6469) + (v15624 * v6471)) * v10382;
                        let v6481 = if v6480 > v778 { 1.0 } else { 0.0 };
                        let v6496: f64;
                        let v9983: Lanes<3>;
                        if v6481 != 0.0 {
                            let v6483 = v774 - v778;
                            let v6484 = (v6480 - v778) / v6483;
                            let v16147 = v16146 / v6483;
                            let v6485 = v6484 * v6484;
                            let v16148 = v16147 * v6484;
                            let v16149 = v16148 + v16148;
                            let v16153 = v16149 * v6485;
                            let v6491 = (((v4 + v6484) + v6485) + (v6485 * v6484)) + (v6485 * v6485);
                            let v6492 = v4 / v6491;
                            let v16162 = (((((((v16147 + v16149) + ((v16149 * v6484) + (v16147 * v6485))) + (v16153 + v16153)) * v6492) * v10382) / v6491) * v10382) * v6483;
                            let v6495 = v778 + (v6483 * (v4 - v6492));
                            v6496 = v6495;
                            v9983 = v16162;
                        } else {
                            v6496 = v6480;
                            v9983 = v16146;
                        }
                        let v16163 = v9983 * v10382;
                        let v6498 = (-v6496) - v6;
                        let v6499 = v6474 - v6093;
                        let v6500 = -v6498;
                        let v16164 = v16163 * v10382;
                        let v6501 = if v6499 < v6500 { 1.0 } else { 0.0 };
                        let v6845: f64;
                        let v6847: f64;
                        let v9984: Lanes<5>;
                        let v9985: Lanes<5>;
                        if v6501 != 0.0 {
                            let v6502 = v658 * v6038;
                            let v6503 = v4 / v6502;
                            let v6504 = v6503 * v122;
                            let v16548 = (((((v10402 * v6038) + (v15606 * v658)) * v6503) * v10382) / v6502) * v122;
                            let v16549 = v16548 * v6505;
                            let v6507 = v73 + (v6505 * v6504);
                            let v6508 = v86 * v6507;
                            let v6509 = v6508 * v6507;
                            let v6510 = v6509 * v6507;
                            let v16556 = ((((v16549 * v86) * v6507) + (v16549 * v6508)) * v6507) + (v16549 * v6509);
                            let v6511 = v656 - v6099;
                            let v16557 = v10398 - v15663;
                            let v6512 = v6499 + v6498;
                            let v16561 = v10402 * v6512;
                            let v16562 = ((Lanes([v16142[0], v16142[1], v16142[2], 0.0])) + (Lanes([v16163[0], v16163[1], 0.0, v16163[2]]))) * v658;
                            let v6515 = v3495 * v6504;
                            let v6516 = (v658 * v6512) - v73;
                            let v6517 = v6515 * v6516;
                            let v16567 = (v16548 * v3495) * v6516;
                            let v16570 = (Lanes([0.0, 0.0, v16567[0], 0.0, 0.0])) + (((Lanes([0.0, 0.0, v16561[0], 0.0, 0.0])) + (Lanes([v16562[0], v16562[1], 0.0, v16562[2], v16562[3]]))) * v6515);
                            let v6518 = v6514 - v6517;
                            let v16571 = v16570 * v10382;
                            let v6519 = v6518 * v6518;
                            let v16572 = v16571 * v6518;
                            let v16573 = v16572 + v16572;
                            let v6521 = if v6510 < (v6519 * v3501) { 1.0 } else { 0.0 };
                            let v6533: f64;
                            let v9986: Lanes<5>;
                            if v6521 != 0.0 {
                                let v16580 = v16556 * v8;
                                let v6525 = (v8 * v6510) / v6518;
                                let v6527 = ((v6522 + v6518) + v6525) + v6517;
                                let v16586 = (v16571 + (((Lanes([0.0, 0.0, v16580[0], 0.0, 0.0])) - (v16571 * v6525)) / v6518)) + v16570;
                                v6533 = v6527;
                                v9986 = v16586;
                            } else {
                                let v6529 = (v6510 + v6519).sqrt();
                                let v6532 = (v6530 + v6529) + v6517;
                                let v16579 = (((Lanes([0.0, 0.0, v16556[0], 0.0, 0.0])) + v16573) * (v9360 / (v10427 * v6529))) + v16570;
                                v6533 = v6532;
                                v9986 = v16579;
                            }
                            let v6534 = v6533.powf(v1557);
                            let v16590 = v9986 * (v1557 * (v6533.powf(v16587)));
                            let v16592 = (v16548 * v3518) * v10382;
                            let v6540 = v743 * v6534;
                            let v6543 = (((v6535 - (v3518 * v6504)) + (v73 * v6534)) + (v6540 * v6534)) / v6534;
                            let v16605 = v10407 * v6543;
                            let v16608 = Lanes([v16163[0], v16163[1], 0.0, 0.0, v16163[2]]);
                            let v6546 = ((v6543 * v660) - v6498) + v6498;
                            let v16610 = ((((((((Lanes([0.0, 0.0, v16592[0], 0.0, 0.0])) + (v16590 * v73)) + (((v16590 * v743) * v6534) + (v16590 * v6540))) - (v16590 * v6543)) / v6534) * v660) + (Lanes([0.0, 0.0, v16605[0], 0.0, 0.0]))) - v16608) + v16608;
                            let v6547 = v6546 / v6511;
                            let v16611 = v16557 * v6547;
                            let v16615 = ((v16610 - (Lanes([0.0, 0.0, v16611[0], 0.0, 0.0]))) / v6511) * v6547;
                            let v6550 = (v4 + (v6547 * v6547)).sqrt();
                            let v6551 = v6546 / v6550;
                            let v6554 = v122 * (v6499 - (v6551 - v6498));
                            let v16626 = ((Lanes([v16142[0], v16142[1], 0.0, v16142[2], 0.0])) - (((v16610 - (((v16615 + v16615) * (v9360 / (v10427 * v6550))) * v6551)) / v6550) - v16608)) * v122;
                            v6845 = v6554;
                            v6847 = v6554;
                            v9984 = v16626;
                            v9985 = v16626;
                        } else {
                            let v6556 = v6499 + v6498;
                            let v16167 = (Lanes([v16142[0], v16142[1], v16142[2], 0.0])) + (Lanes([v16163[0], v16163[1], 0.0, v16163[2]]));
                            let v16168 = v10402 * v6556;
                            let v16169 = v16167 * v658;
                            let v16171 = Lanes([v16169[0], v16169[1], 0.0, v16169[2], v16169[3]]);
                            let v16172 = (Lanes([0.0, 0.0, v16168[0], 0.0, 0.0])) + v16171;
                            let v6558 = (v658 * v6556) - v4;
                            let v6561 = v6092 * v659;
                            let v16176 = (v15652 * v659) + (v10404 * v6092);
                            let v6562 = (v85 * (v6558 + v6555)) / v6561;
                            let v16177 = v16176 * v6562;
                            let v16180 = ((v16172 * v85) - (Lanes([0.0, 0.0, v16177[0], 0.0, 0.0]))) / v6561;
                            let v6563 = v4 + v6562;
                            let v6565 = if v6563 < v6564 { 1.0 } else { 0.0 };
                            let v6569: f64;
                            let v9987: Lanes<5>;
                            if v6565 != 0.0 {
                                v6569 = v6566;
                                v9987 = v10571;
                            } else {
                                v6569 = v6563;
                                v9987 = v16180;
                            }
                            let v6568 = (v6092 * v658) / v73;
                            let v16184 = ((v15652 * v658) + (v10402 * v6092)) / v73;
                            let v6570 = v6569.sqrt();
                            let v6571 = v4 - v6570;
                            let v16189 = v16184 * v6571;
                            let v16193 = Lanes([v16142[0], v16142[1], 0.0, v16142[2], 0.0]);
                            let v6574 = (v6499 + (v6568 * v6571)) + v6498;
                            let v16195 = Lanes([v16163[0], v16163[1], 0.0, 0.0, v16163[2]]);
                            let v16197 = v10402 * v6574;
                            let v6577 = (-(v658 * v6574)).exp();
                            let v6580 = (v85 * (v6558 + v6577)) / v6561;
                            let v16205 = v16176 * v6580;
                            let v16208 = (((v16172 + ((((Lanes([0.0, 0.0, v16197[0], 0.0, 0.0])) + (((v16193 + ((Lanes([0.0, 0.0, v16189[0], 0.0, 0.0])) + (((v9987 * (v9360 / (v10427 * v6570))) * v10382) * v6568))) + v16195) * v658)) * v10382) * v6577)) * v85) - (Lanes([0.0, 0.0, v16205[0], 0.0, 0.0]))) / v6561;
                            let v6581 = v4 + v6580;
                            let v6583 = if v6581 < v6582 { 1.0 } else { 0.0 };
                            let v6585: f64;
                            let v9988: Lanes<5>;
                            if v6583 != 0.0 {
                                v6585 = v6584;
                                v9988 = v10571;
                            } else {
                                v6585 = v6581;
                                v9988 = v16208;
                            }
                            let v6586 = v6585.sqrt();
                            let v6587 = v4 - v6586;
                            let v16213 = v16184 * v6587;
                            let v6590 = (v6499 + (v6568 * v6587)) + v6498;
                            let v6591 = v658 * v6590;
                            let v16219 = v10402 * v6590;
                            let v16222 = (Lanes([0.0, 0.0, v16219[0], 0.0, 0.0])) + (((v16193 + ((Lanes([0.0, 0.0, v16213[0], 0.0, 0.0])) + (((v9988 * (v9360 / (v10427 * v6586))) * v10382) * v6568))) + v16195) * v658);
                            let v6592 = if v6591 < v91 { 1.0 } else { 0.0 };
                            let v6669: f64;
                            let v9989: Lanes<5>;
                            if v6592 != 0.0 {
                                let v6595 = v658 * v6091;
                                let v6596 = v4 / v6595;
                                let v16228 = ((((v10402 * v6091) + (v15650 * v658)) * v6596) * v10382) / v6595;
                                let v6597 = v6594 + v6596;
                                let v16229 = v16167 * v10382;
                                let v6599 = (-v6556) / v6091;
                                let v16230 = v15650 * v6599;
                                let v16237 = ((v16228 * v6593) / v6602) * v10382;
                                let v6607 = (v6600 - ((v6593 * v6597) / v6602)) + (v6599 / v6605);
                                let v16240 = (Lanes([0.0, 0.0, v16237[0], 0.0, 0.0])) + ((((Lanes([v16229[0], v16229[1], 0.0, v16229[2], v16229[3]])) - (Lanes([0.0, 0.0, v16230[0], 0.0, 0.0]))) / v6091) / v6605);
                                let v6613 = ((v6608 * v6597) - v6610) / v6612;
                                let v16242 = (v16228 * v6608) / v6612;
                                let v16243 = v16240 * v6607;
                                let v6615 = v6613 * v6613;
                                let v16245 = v16242 * v6613;
                                let v16249 = ((v16245 + v16245) * v6613) + (v16242 * v6615);
                                let v6618 = ((v6607 * v6607) + (v6615 * v6613)).sqrt();
                                let v16254 = ((v16243 + v16243) + (Lanes([0.0, 0.0, v16249[0], 0.0, 0.0]))) * (v9360 / (v10427 * v6618));
                                let v6620 = (-v6607) + v6618;
                                let v6622 = v6607 + v6618;
                                let v6627 = ((v6620.powf(v1557)) + (-(v6622.powf(v1557)))) - v6626;
                                let v16269 = v10407 * v6627;
                                let v6630 = ((v6627 * v660) - v6498) + v6498;
                                let v6631 = v658 * v6630;
                                let v16274 = v10402 * v6630;
                                let v16277 = (Lanes([0.0, 0.0, v16274[0], 0.0, 0.0])) + (((((((((v16240 * v10382) + v16254) * (v1557 * (v6620.powf(v16257)))) + (((v16240 + v16254) * (v1557 * (v6622.powf(v16262)))) * v10382)) * v660) + (Lanes([0.0, 0.0, v16269[0], 0.0, 0.0]))) - v16195) + v16195) * v658);
                                v6669 = v6631;
                                v9989 = v16277;
                            } else {
                                v6669 = v6591;
                                v9989 = v16222;
                            }
                            let v6632 = v6556 + v74;
                            let v16278 = v10402 * v6500;
                            let v16279 = v16164 * v658;
                            let v6634 = (v658 * v6500).exp();
                            let v16283 = ((Lanes([0.0, 0.0, v16278[0], 0.0])) + (Lanes([v16279[0], v16279[1], 0.0, v16279[2]]))) * v6634;
                            let v6635 = v6634 + v358;
                            let v6636 = v726 / v485;
                            let v6637 = v6636 * v6636;
                            let v16285 = (v10445 / v485) * v6636;
                            let v16286 = v16285 + v16285;
                            let v6638 = v6637 * v6635;
                            let v16287 = v16286 * v6635;
                            let v16288 = v16283 * v6637;
                            let v6639 = v658 * v6632;
                            let v16291 = v10402 * v6632;
                            let v16293 = (Lanes([0.0, 0.0, v16291[0], 0.0, 0.0])) + v16171;
                            let v6640 = v6638 * v6561;
                            let v16295 = v16176 * v6638;
                            let v16297 = (((Lanes([0.0, 0.0, v16287[0], 0.0])) + v16288) * v6561) + (Lanes([0.0, 0.0, v16295[0], 0.0]));
                            let v16298 = v16293 * v6639;
                            let v6642 = v6640 + (v6639 * v6639);
                            let v16300 = Lanes([v16297[0], v16297[1], v16297[2], 0.0, v16297[3]]);
                            let v6644 = v6637 * v6561;
                            let v6645 = v6644.ln();
                            let v16308 = ((v16286 * v6561) + (v16176 * v6637)) * (v9360 / v6644);
                            let v16309 = Lanes([0.0, 0.0, v16308[0], 0.0, 0.0]);
                            let v6647 = v658 * v6498;
                            let v16311 = v10402 * v6498;
                            let v16312 = v16163 * v658;
                            let v16315 = (Lanes([0.0, 0.0, v16311[0], 0.0])) + (Lanes([v16312[0], v16312[1], 0.0, v16312[2]]));
                            let v16316 = Lanes([v16315[0], v16315[1], v16315[2], 0.0, v16315[3]]);
                            let v16318 = v16293 - ((((v16300 + (v16298 + v16298)) * (v9360 / v6642)) - v16309) + v16316);
                            let v6650 = (v6639 - (((v6642.ln()) - v6645) + v6647)) - v4;
                            let v6651 = v85 * v6639;
                            let v16319 = v16293 * v85;
                            let v6652 = if v6651 > v0 { 1.0 } else { 0.0 };
                            let v6654: f64;
                            let v9990: Lanes<5>;
                            if v6652 != 0.0 {
                                v6654 = v6651;
                                v9990 = v16319;
                            } else {
                                let v6653 = -v6651;
                                let v16320 = v16319 * v10382;
                                v6654 = v6653;
                                v9990 = v16320;
                            }
                            let v16321 = v16318 * v6650;
                            let v6657 = ((v6650 * v6650) + v6654).sqrt();
                            let v16331 = v10402 * v74;
                            let v6663 = (v6639 - (v6639 - (v8 * (v6650 + v6657)))) + (v658 * v74);
                            let v16334 = ((v16293 - (v16293 - ((v16318 + (((v16321 + v16321) + v9990) * (v9360 / (v10427 * v6657)))) * v8))) + (Lanes([0.0, 0.0, v16331[0], 0.0, 0.0]))) * v6663;
                            let v6665 = v6640 + (v6663 * v6663);
                            let v6668 = ((v6665.ln()) - v6645) + v6647;
                            let v16340 = (((v16300 + (v16334 + v16334)) * (v9360 / v6665)) - v16309) + v16316;
                            let v16341 = v16340 - v9989;
                            let v6672 = (v6668 - v6669) - v6671;
                            let v6675 = (v85 * v6668) * v6674;
                            let v16343 = (v16340 * v85) * v6674;
                            let v6676 = if v6675 > v0 { 1.0 } else { 0.0 };
                            let v6678: f64;
                            let v9991: Lanes<5>;
                            if v6676 != 0.0 {
                                v6678 = v6675;
                                v9991 = v16343;
                            } else {
                                let v6677 = -v6675;
                                let v16344 = v16343 * v10382;
                                v6678 = v6677;
                                v9991 = v16344;
                            }
                            let v16345 = v16341 * v6672;
                            let v6681 = ((v6672 * v6672) + v6678).sqrt();
                            let v6684 = v6668 - (v8 * (v6672 + v6681));
                            let v16353 = v16340 - ((v16341 + (((v16345 + v16345) + v9991) * (v9360 / (v10427 * v6681)))) * v8);
                            let v6685 = v6684 / v658;
                            let v16354 = v10402 * v6685;
                            let v6686 = v6685 - v6498;
                            let v16358 = ((v16353 - (Lanes([0.0, 0.0, v16354[0], 0.0, 0.0]))) / v658) - v16195;
                            let v6689 = (-v6684).exp();
                            let v6690 = (v6684 - v4) + v6689;
                            let v16361 = v16353 + ((v16353 * v10382) * v6689);
                            let v6692 = if v6690 < v6691 { 1.0 } else { 0.0 };
                            let v6694: f64;
                            let v9992: Lanes<5>;
                            if v6692 != 0.0 {
                                v6694 = v6693;
                                v9992 = v10571;
                            } else {
                                v6694 = v6690;
                                v9992 = v16361;
                            }
                            let v6695 = v6694.sqrt();
                            let v6696 = v6038 * v6695;
                            let v16365 = v15606 * v6695;
                            let v16368 = (Lanes([0.0, 0.0, v16365[0], 0.0, 0.0])) + ((v9992 * (v9360 / (v10427 * v6695))) * v6038);
                            let v6698 = v122 * (v6499 - v6686);
                            let v16370 = (v16193 - v16358) * v122;
                            let v6699 = if v6299 == v4 { 1.0 } else { 0.0 };
                            let v6846: f64;
                            let v6848: f64;
                            let v9993: Lanes<5>;
                            let v9994: Lanes<5>;
                            if v6699 != 0.0 {
                                let v6700 = v6637 * v6634;
                                let v16371 = v16286 * v6634;
                                let v16373 = (Lanes([0.0, 0.0, v16371[0], 0.0])) + v16288;
                                let mut v6701: f64 = 0.0;
                                let mut v6704: f64 = 0.0;
                                let mut v6790: f64 = 0.0;
                                let mut v6820: f64 = 0.0;
                                let mut v6823: f64 = 0.0;
                                let mut v6833: f64 = 0.0;
                                let mut v6838: f64 = 0.0;
                                let mut v9995: Lanes<5> = Lanes([0.0; 5]);
                                let mut v9996: Lanes<5> = Lanes([0.0; 5]);
                                let mut v9997: Lanes<5> = Lanes([0.0; 5]);
                                let mut v9998: Lanes<5> = Lanes([0.0; 5]);
                                let mut v9999: Lanes<5> = Lanes([0.0; 5]);
                                v6701 = v4;
                                v6704 = v6686;
                                v6790 = v0;
                                v6820 = v6684;
                                v6823 = v6824;
                                v6833 = v6834;
                                v6838 = v6839;
                                v9995 = v16358;
                                v9996 = v16353;
                                v9997 = v9933;
                                v9998 = v9934;
                                v9999 = v9935;
                                loop {
                                    let v6703 = if v6701 <= v6702 { 1.0 } else { 0.0 };
                                    if v6703 == 0.0 {
                                        break;
                                    }
                                    let v6705 = v6704 + v6498;
                                    let v6706 = v658 * v6705;
                                    let v16394 = v10402 * v6705;
                                    let v16397 = (Lanes([0.0, 0.0, v16394[0], 0.0, 0.0])) + ((v9995 + v16195) * v658);
                                    let v6707 = if v6706 < v639 { 1.0 } else { 0.0 };
                                    let v6783: f64;
                                    let v6787: f64;
                                    let v6826: f64;
                                    let v6841: f64;
                                    let v10000: Lanes<5>;
                                    let v10001: Lanes<5>;
                                    let v10002: Lanes<5>;
                                    let v10003: Lanes<5>;
                                    if v6707 != 0.0 {
                                        let v6708 = v6706 * v6706;
                                        let v16439 = v16397 * v6706;
                                        let v16440 = v16439 + v16439;
                                        let v6709 = v6708 * v6706;
                                        let v6712 = v6710 + (v6706 * v6313);
                                        let v6714 = v6311 + (v6706 * v6712);
                                        let v6715 = v6709 * v6714;
                                        let v16450 = (((v16440 * v6706) + (v16397 * v6708)) * v6714) + (((v16397 * v6712) + ((v16397 * v6313) * v6706)) * v6709);
                                        let v6718 = v6706 * v639;
                                        let v16451 = v16397 * v639;
                                        let v6720 = v6717 + (v6718 * v6313);
                                        let v6722 = v6716 + (v6706 * v6720);
                                        let v6723 = v6708 * v6722;
                                        let v6724 = v6700 * v6715;
                                        let v16459 = v16373 * v6715;
                                        let v6725 = v6724 * v6715;
                                        let v16465 = (((Lanes([v16459[0], v16459[1], v16459[2], 0.0, v16459[3]])) + (v16450 * v6700)) * v6715) + (v16450 * v6724);
                                        let v16467 = v10402 * v6700;
                                        let v6727 = (v6700 * v658) * v73;
                                        let v6728 = v6727 * v6715;
                                        let v16471 = (((v16373 * v658) + (Lanes([0.0, 0.0, v16467[0], 0.0]))) * v73) * v6715;
                                        let v6733 = v6731 + (v6706 * v6337);
                                        let v6735 = v6335 + (v6706 * v6733);
                                        let v6737 = v6730 + (v6706 * v6735);
                                        let v6739 = v6333 + (v6706 * v6737);
                                        let v6740 = v6706 * v6739;
                                        let v16490 = (v16397 * v6739) + (((v16397 * v6737) + (((v16397 * v6735) + (((v16397 * v6733) + ((v16397 * v6337) * v6706)) * v6706)) * v6706)) * v6706);
                                        let v6745 = v6743 + (v6718 * v6337);
                                        let v6747 = v6742 + (v6706 * v6745);
                                        let v6749 = v6741 + (v6706 * v6747);
                                        let v6751 = v6333 + (v6706 * v6749);
                                        let v16501 = v16490 * v6740;
                                        let v6755 = (((v6740 * v6740) + v6725) + v358).sqrt();
                                        let v16506 = ((v16501 + v16501) + v16465) * (v9360 / (v10427 * v6755));
                                        let v16507 = v10402 * v6751;
                                        let v6757 = (v658 * v6751) * v73;
                                        let v6760 = v6755 + v6755;
                                        let v6761 = ((v6757 * v6740) + (v6728 * v6723)) / v6760;
                                        let v16519 = (((((((Lanes([0.0, 0.0, v16507[0], 0.0, 0.0])) + (((v16397 * v6749) + (((v16397 * v6747) + (((v16397 * v6745) + ((v16451 * v6337) * v6706)) * v6706)) * v6706)) * v658)) * v73) * v6740) + (v16490 * v6757)) + ((((Lanes([v16471[0], v16471[1], v16471[2], 0.0, v16471[3]])) + (v16450 * v6727)) * v6723) + (((v16440 * v6722) + (((v16397 * v6720) + ((v16451 * v6313) * v6706)) * v6708)) * v6728))) - ((v16506 + v16506) * v6761)) / v6760;
                                        v6783 = v6755;
                                        v6787 = v6761;
                                        v6826 = v6740;
                                        v6841 = v6725;
                                        v10000 = v16506;
                                        v10001 = v16519;
                                        v10002 = v16490;
                                        v10003 = v16465;
                                    } else {
                                        let v6762 = if v6706 < v2530 { 1.0 } else { 0.0 };
                                        let v6775: f64;
                                        let v6778: f64;
                                        let v10004: Lanes<5>;
                                        let v10005: Lanes<5>;
                                        if v6762 != 0.0 {
                                            let v6763 = v6706.exp();
                                            let v16416 = v16397 * v6763;
                                            let v6764 = v6763 - v4;
                                            let v6765 = v6700 * v6764;
                                            let v16417 = v16373 * v6764;
                                            let v16420 = (Lanes([v16417[0], v16417[1], v16417[2], 0.0, v16417[3]])) + (v16416 * v6700);
                                            let v6766 = v6700 * v658;
                                            let v16422 = v10402 * v6700;
                                            let v6767 = v6766 * v6763;
                                            let v16425 = ((v16373 * v658) + (Lanes([0.0, 0.0, v16422[0], 0.0]))) * v6763;
                                            let v16428 = (Lanes([v16425[0], v16425[1], v16425[2], 0.0, v16425[3]])) + (v16416 * v6766);
                                            v6775 = v6765;
                                            v6778 = v6767;
                                            v10004 = v16420;
                                            v10005 = v16428;
                                        } else {
                                            let v16398 = v10402 * v6704;
                                            let v6769 = (v658 * v6704).exp();
                                            let v16402 = ((Lanes([0.0, 0.0, v16398[0], 0.0, 0.0])) + (v9995 * v658)) * v6769;
                                            let v6770 = v6769 - v6634;
                                            let v6771 = v6637 * v6770;
                                            let v16405 = v16286 * v6770;
                                            let v16408 = (Lanes([0.0, 0.0, v16405[0], 0.0, 0.0])) + ((v16402 - (Lanes([v16283[0], v16283[1], v16283[2], 0.0, v16283[3]]))) * v6637);
                                            let v6772 = v6637 * v658;
                                            let v6773 = v6772 * v6769;
                                            let v16412 = ((v16286 * v658) + (v10402 * v6637)) * v6769;
                                            let v16415 = (Lanes([0.0, 0.0, v16412[0], 0.0, 0.0])) + (v16402 * v6772);
                                            v6775 = v6771;
                                            v6778 = v6773;
                                            v10004 = v16408;
                                            v10005 = v16415;
                                        }
                                        let v6777 = ((v6706 - v4) + v6775).sqrt();
                                        let v16432 = (v16397 + v10004) * (v9360 / (v10427 * v6777));
                                        let v6780 = (v658 + v6778) / v6777;
                                        let v6781 = v6780 * v8;
                                        let v16438 = ((((Lanes([0.0, 0.0, v10402[0], 0.0, 0.0])) + v10005) - (v16432 * v6780)) / v6777) * v8;
                                        v6783 = v6777;
                                        v6787 = v6781;
                                        v6826 = v0;
                                        v6841 = v6775;
                                        v10000 = v16432;
                                        v10001 = v16438;
                                        v10002 = v10571;
                                        v10003 = v10004;
                                    }
                                    let v16521 = v15650 * v6783;
                                    let v6785 = (v6499 - v6704) - (v6091 * v6783);
                                    let v16525 = (v16193 - v9995) - ((Lanes([0.0, 0.0, v16521[0], 0.0, 0.0])) + (v10000 * v6091));
                                    let v16526 = v15650 * v6787;
                                    let v6789 = v6786 - (v6091 * v6787);
                                    let v16530 = ((Lanes([0.0, 0.0, v16526[0], 0.0, 0.0])) + (v10001 * v6091)) * v10382;
                                    let v6791 = if v6790 == v4 { 1.0 } else { 0.0 };
                                    let v6814: f64;
                                    let v6816: f64;
                                    let v6817: f64;
                                    let v10006: Lanes<5>;
                                    if v6791 != 0.0 {
                                        v6814 = v6792;
                                        v6816 = v6704;
                                        v6817 = v6790;
                                        v10006 = v9995;
                                    } else {
                                        let v6794 = (-v6785) / v6789;
                                        let v16534 = ((v16525 * v10382) - (v16530 * v6794)) / v6789;
                                        let v6796 = v6704.abs();
                                        let v16538 = v9995 * ((v10427 * (if v6704 >= v11296 { 1.0 } else { 0.0 })) - v9360);
                                        let v6797 = if v4 >= v6796 { 1.0 } else { 0.0 };
                                        let v6798: f64;
                                        let v10007: Lanes<5>;
                                        if v6797 != 0.0 {
                                            v6798 = v4;
                                            v10007 = v10571;
                                        } else {
                                            v6798 = v6796;
                                            v10007 = v16538;
                                        }
                                        let v6800 = v6795 * (v4 + v6798);
                                        let v16539 = v10007 * v6795;
                                        let v6802 = if (v6794.abs()) > v6800 { 1.0 } else { 0.0 };
                                        let v6807: f64;
                                        let v10008: Lanes<5>;
                                        if v6802 != 0.0 {
                                            let v6803 = if v6794 >= v0 { 1.0 } else { 0.0 };
                                            let v6805: f64;
                                            if v6803 != 0.0 {
                                                v6805 = v4;
                                            } else {
                                                v6805 = v6804;
                                            }
                                            let v6806 = v6800 * v6805;
                                            let v16540 = v16539 * v6805;
                                            v6807 = v6806;
                                            v10008 = v16540;
                                        } else {
                                            v6807 = v6794;
                                            v10008 = v16534;
                                        }
                                        let v6808 = v6704 + v6807;
                                        let v16541 = v9995 + v10008;
                                        let v6813 = if (if (v6807.abs()) <= v856 { 1.0 } else { 0.0 }) != 0.0 && (if (v6785.abs()) <= v3501 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v6818: f64;
                                        if v6813 != 0.0 {
                                            v6818 = v4;
                                        } else {
                                            v6818 = v6790;
                                        }
                                        v6814 = v6701;
                                        v6816 = v6808;
                                        v6817 = v6818;
                                        v10006 = v16541;
                                    }
                                    let v6815 = v6814 + v4;
                                    v6701 = v6815;
                                    v6704 = v6816;
                                    v6790 = v6817;
                                    v6820 = v6706;
                                    v6823 = v6826;
                                    v6833 = v6783;
                                    v6838 = v6841;
                                    v9995 = v10006;
                                    v9996 = v16397;
                                    v9997 = v10002;
                                    v9998 = v10000;
                                    v9999 = v10003;
                                }
                                let v6819 = if v6790 == v0 { 1.0 } else { 0.0 };
                                if v6819 != 0.0 {
                                } else {
                                }
                                let v6821 = if v6820 < v639 { 1.0 } else { 0.0 };
                                let v6831: f64;
                                let v10009: Lanes<5>;
                                if v6821 != 0.0 {
                                    let v6822 = if v6820 < v91 { 1.0 } else { 0.0 };
                                    if v6822 != 0.0 {
                                    } else {
                                    }
                                    let v6828 = v6823 + v6827;
                                    v6831 = v6828;
                                    v10009 = v9997;
                                } else {
                                    let v6830 = (v6820 - v4).sqrt();
                                    let v16376 = v9996 * (v9360 / (v10427 * v6830));
                                    v6831 = v6830;
                                    v10009 = v16376;
                                }
                                let v6832 = v6038 * v6831;
                                let v16377 = v15606 * v6831;
                                let v16380 = (Lanes([0.0, 0.0, v16377[0], 0.0, 0.0])) + (v10009 * v6038);
                                let v6836 = v6833 + v6831;
                                let v6837 = v4 / v6836;
                                let v6842 = v6038 * v6838;
                                let v16385 = v15606 * v6838;
                                let v6844 = v6832 + (v6842 * v6837);
                                let v16392 = v16380 + ((((Lanes([0.0, 0.0, v16385[0], 0.0, 0.0])) + (v9999 * v6038)) * v6837) + (((((v9998 + v10009) * v6837) * v10382) / v6836) * v6842));
                                v6846 = v6844;
                                v6848 = v6832;
                                v9993 = v16392;
                                v9994 = v16380;
                            } else {
                                v6846 = v6698;
                                v6848 = v6696;
                                v9993 = v16370;
                                v9994 = v16368;
                            }
                            v6845 = v6846;
                            v6847 = v6848;
                            v9984 = v9993;
                            v9985 = v9994;
                        }
                        let v6849 = v6845 - v6847;
                        let v16627 = v9984 - v9985;
                        let v8537: f64;
                        let v8545: f64;
                        let v8553: f64;
                        let v8565: f64;
                        let v8577: f64;
                        let v8584: f64;
                        let v8594: f64;
                        let v8601: f64;
                        let v10010: Lanes<5>;
                        let v10011: Lanes<5>;
                        let v10012: Lanes<6>;
                        let v10013: Lanes<6>;
                        let v10014: Lanes<5>;
                        let v10015: Lanes<6>;
                        let v10016: Lanes<5>;
                        let v10017: Lanes<6>;
                        if v6850 != 0.0 {
                            let v8538: f64;
                            let v8595: f64;
                            let v10018: Lanes<5>;
                            let v10019: Lanes<5>;
                            if v6466 != 0.0 {
                                let v6851 = -v6451;
                                let v6852 = v6851 * v6845;
                                let v16636 = v9984 * v6851;
                                let v6853 = v6851 * v6849;
                                let v16637 = v16627 * v6851;
                                v8538 = v6852;
                                v8595 = v6853;
                                v10018 = v16636;
                                v10019 = v16637;
                            } else {
                                v8538 = v8539;
                                v8595 = v8596;
                                v10018 = v9963;
                                v10019 = v9969;
                            }
                            let v8546: f64;
                            let v8578: f64;
                            let v10020: Lanes<5>;
                            let v10021: Lanes<5>;
                            if v6467 != 0.0 {
                                let v6854 = -v6451;
                                let v6855 = v6854 * v6845;
                                let v16638 = v9984 * v6854;
                                let v6856 = v6854 * v6849;
                                let v16639 = v16627 * v6854;
                                v8546 = v6855;
                                v8578 = v6856;
                                v10020 = v16638;
                                v10021 = v16639;
                            } else {
                                v8546 = v8547;
                                v8578 = v8579;
                                v10020 = v9964;
                                v10021 = v9967;
                            }
                            v8537 = v8538;
                            v8545 = v8546;
                            v8553 = v8554;
                            v8565 = v8566;
                            v8577 = v8578;
                            v8584 = v8585;
                            v8594 = v8595;
                            v8601 = v8602;
                            v10010 = v10018;
                            v10011 = v10020;
                            v10012 = v9965;
                            v10013 = v9966;
                            v10014 = v10021;
                            v10015 = v9968;
                            v10016 = v10019;
                            v10017 = v9970;
                        } else {
                            let v8557: f64;
                            let v8569: f64;
                            let v8588: f64;
                            let v8605: f64;
                            let v10022: Lanes<6>;
                            let v10023: Lanes<6>;
                            let v10024: Lanes<6>;
                            let v10025: Lanes<6>;
                            if v6857 != 0.0 {
                                let v8558: f64;
                                let v8606: f64;
                                let v10026: Lanes<6>;
                                let v10027: Lanes<6>;
                                if v6466 != 0.0 {
                                    let v6858 = -v6451;
                                    let v6859 = v6858 * v6845;
                                    let v16628 = v9984 * v6858;
                                    let v6860 = v6858 * v6849;
                                    let v16629 = v16627 * v6858;
                                    let v16630 = Lanes([v16628[0], v16628[1], v16628[2], v16628[3], v16628[4], 0.0]);
                                    let v16631 = Lanes([v16629[0], v16629[1], v16629[2], v16629[3], v16629[4], 0.0]);
                                    v8558 = v6859;
                                    v8606 = v6860;
                                    v10026 = v16630;
                                    v10027 = v16631;
                                } else {
                                    v8558 = v8554;
                                    v8606 = v8602;
                                    v10026 = v9965;
                                    v10027 = v9970;
                                }
                                let v8570: f64;
                                let v8589: f64;
                                let v10028: Lanes<6>;
                                let v10029: Lanes<6>;
                                if v6467 != 0.0 {
                                    let v6861 = -v6451;
                                    let v6862 = v6861 * v6845;
                                    let v16632 = v9984 * v6861;
                                    let v6863 = v6861 * v6849;
                                    let v16633 = v16627 * v6861;
                                    let v16634 = Lanes([v16632[0], v16632[1], v16632[2], v16632[3], v16632[4], 0.0]);
                                    let v16635 = Lanes([v16633[0], v16633[1], v16633[2], v16633[3], v16633[4], 0.0]);
                                    v8570 = v6862;
                                    v8589 = v6863;
                                    v10028 = v16634;
                                    v10029 = v16635;
                                } else {
                                    v8570 = v8566;
                                    v8589 = v8585;
                                    v10028 = v9966;
                                    v10029 = v9968;
                                }
                                v8557 = v8558;
                                v8569 = v8570;
                                v8588 = v8589;
                                v8605 = v8606;
                                v10022 = v10026;
                                v10023 = v10028;
                                v10024 = v10029;
                                v10025 = v10027;
                            } else {
                                v8557 = v8554;
                                v8569 = v8566;
                                v8588 = v8585;
                                v8605 = v8602;
                                v10022 = v9965;
                                v10023 = v9966;
                                v10024 = v9968;
                                v10025 = v9970;
                            }
                            v8537 = v8539;
                            v8545 = v8547;
                            v8553 = v8557;
                            v8565 = v8569;
                            v8577 = v8579;
                            v8584 = v8588;
                            v8594 = v8596;
                            v8601 = v8605;
                            v10010 = v9963;
                            v10011 = v9964;
                            v10012 = v10022;
                            v10013 = v10023;
                            v10014 = v9967;
                            v10015 = v10024;
                            v10016 = v9969;
                            v10017 = v10025;
                        }
                        v8536 = v8537;
                        v8544 = v8545;
                        v8552 = v8553;
                        v8564 = v8565;
                        v8576 = v8577;
                        v8583 = v8584;
                        v8593 = v8594;
                        v8600 = v8601;
                        v9922 = v10010;
                        v9923 = v10011;
                        v9924 = v10012;
                        v9925 = v10013;
                        v9926 = v10014;
                        v9927 = v10015;
                        v9928 = v10016;
                        v9929 = v10017;
                    } else {
                        v8536 = v0;
                        v8544 = v0;
                        v8552 = v6025;
                        v8564 = v6024;
                        v8576 = v0;
                        v8583 = v6022;
                        v8593 = v0;
                        v8600 = v6023;
                        v9922 = v10571;
                        v9923 = v10571;
                        v9924 = v15605;
                        v9925 = v15604;
                        v9926 = v10571;
                        v9927 = v15602;
                        v9928 = v10571;
                        v9929 = v15603;
                    }
                    v8535 = v8536;
                    v8543 = v8544;
                    v8551 = v8552;
                    v8563 = v8564;
                    v8575 = v8576;
                    v8582 = v8583;
                    v8592 = v8593;
                    v8599 = v8600;
                    v9914 = v9922;
                    v9915 = v9923;
                    v9916 = v9924;
                    v9917 = v9925;
                    v9918 = v9926;
                    v9919 = v9927;
                    v9920 = v9928;
                    v9921 = v9929;
                } else {
                    v8535 = v0;
                    v8543 = v0;
                    v8551 = v6025;
                    v8563 = v6024;
                    v8575 = v0;
                    v8582 = v6022;
                    v8592 = v0;
                    v8599 = v6023;
                    v9914 = v10571;
                    v9915 = v10571;
                    v9916 = v15605;
                    v9917 = v15604;
                    v9918 = v10571;
                    v9919 = v15602;
                    v9920 = v10571;
                    v9921 = v15603;
                }
                v8534 = v8535;
                v8542 = v8543;
                v8550 = v8551;
                v8562 = v8563;
                v8574 = v8575;
                v8581 = v8582;
                v8591 = v8592;
                v8598 = v8599;
                v9906 = v9914;
                v9907 = v9915;
                v9908 = v9916;
                v9909 = v9917;
                v9910 = v9918;
                v9911 = v9919;
                v9912 = v9920;
                v9913 = v9921;
            } else {
                v8534 = v0;
                v8542 = v0;
                v8550 = v8559;
                v8562 = v8571;
                v8574 = v0;
                v8581 = v0;
                v8591 = v0;
                v8598 = v0;
                v9906 = v10571;
                v9907 = v10571;
                v9908 = v9457;
                v9909 = v9458;
                v9910 = v10571;
                v9911 = v11054;
                v9912 = v10571;
                v9913 = v11054;
            }
            let v6864 = if v4320 != v0 { 1.0 } else { 0.0 };
            let v8293: f64;
            let v8506: f64;
            let v10030: Lanes<6>;
            let v10031: Lanes<6>;
            if v6864 != 0.0 {
                let v6865 = v818 + v4335;
                let v16652 = (Lanes([v9402[0], v9402[1], 0.0, 0.0, 0.0, 0.0])) + v9433;
                let v6867 = v4 - v4351;
                let v6869 = (v4351 * v6865) + (v6867 * v4331);
                let v16655 = (v16652 * v4351) + (v9432 * v6867);
                let v6871 = if v6870 != v0 { 1.0 } else { 0.0 };
                if v6871 != 0.0 {
                } else {
                }
                let v6874 = if v6869 > (v6865 - v6872) { 1.0 } else { 0.0 };
                let v8294: f64;
                let v10032: Lanes<6>;
                if v6874 != 0.0 {
                    let v6876 = v6865 - v6875;
                    v8294 = v6876;
                    v10032 = v16652;
                } else {
                    v8294 = v6869;
                    v10032 = v16655;
                }
                v8293 = v8294;
                v8506 = v0;
                v10030 = v10032;
                v10031 = v11054;
            } else {
                let v6877 = if v6870 != v0 { 1.0 } else { 0.0 };
                let v8507: f64;
                let v10033: Lanes<6>;
                if v6877 != 0.0 {
                    let v6879 = if v4376 < v6878 { 1.0 } else { 0.0 };
                    let v8508: f64;
                    let v10034: Lanes<6>;
                    if v6879 != 0.0 {
                        v8508 = v0;
                        v10034 = v11054;
                    } else {
                        let v6880 = v660 / v131;
                        let v6881 = v4 / v4343;
                        let v6882 = v4376 * v6880;
                        let v16645 = (v10407 / v131) * v4376;
                        let v6883 = v6882 * v6881;
                        let v16650 = (((v9435 * v6880) + (Lanes([0.0, 0.0, v16645[0], 0.0, 0.0, 0.0]))) * v6881) + ((((v9434 * v6881) * v10382) / v4343) * v6882);
                        v8508 = v6883;
                        v10034 = v16650;
                    }
                    v8507 = v8508;
                    v10033 = v10034;
                } else {
                    v8507 = v0;
                    v10033 = v11054;
                }
                v8293 = v8295;
                v8506 = v8507;
                v10030 = v9775;
                v10031 = v10033;
            }
            let v6884 = v4 / v122;
            let v8449: f64;
            let v8453: f64;
            let v8618: f64;
            let v8624: f64;
            let v8636: f64;
            let v8647: f64;
            let v10035: Lanes<6>;
            let v10036: Lanes<6>;
            let v10037: Lanes<5>;
            let v10038: Lanes<5>;
            let v10039: Lanes<5>;
            let v10040: Lanes<5>;
            if v561 != 0.0 {
                let v6888 = if v6887 > v0 { 1.0 } else { 0.0 };
                let v6889 = if (if v6885 >= v4 { 1.0 } else { 0.0 }) != 0.0 && v6888 != 0.0 { 1.0 } else { 0.0 };
                let v8450: f64;
                let v8454: f64;
                let v8619: f64;
                let v8625: f64;
                let v8637: f64;
                let v8648: f64;
                let v10041: Lanes<6>;
                let v10042: Lanes<6>;
                let v10043: Lanes<5>;
                let v10044: Lanes<5>;
                let v10045: Lanes<5>;
                let v10046: Lanes<5>;
                if v6889 != 0.0 {
                    let v6893 = if (if v34 == v0 { 1.0 } else { 0.0 }) != 0.0 && v6888 != 0.0 { 1.0 } else { 0.0 };
                    let v7796: f64;
                    let v7815: f64;
                    let v8620: f64;
                    let v8626: f64;
                    let v8638: f64;
                    let v8649: f64;
                    let v10047: Lanes<6>;
                    let v10048: Lanes<6>;
                    let v10049: Lanes<5>;
                    let v10050: Lanes<5>;
                    let v10051: Lanes<5>;
                    let v10052: Lanes<5>;
                    if v6893 != 0.0 {
                        let v6897: f64;
                        if v5 != 0.0 {
                            let v6895 = v6894 * v122;
                            v6897 = v6895;
                        } else {
                            let v6896 = v164 * v122;
                            v6897 = v6896;
                        }
                        let v6898 = v6890 * v6897;
                        let v6899 = v6891 + v825;
                        let v6900 = v6898 * v6899;
                        let v6901 = v6887 * v6897;
                        let v6902 = v772 - v4335;
                        let v17734 = v9404 * v6901;
                        let v17736 = (v9404 * v6898) * v6902;
                        let v6905 = (v825 * v6901) - (v6902 * v6900);
                        let v17740 = (Lanes([v17734[0], v17734[1], 0.0, v17734[2], 0.0, 0.0])) - (((v9433 * v10382) * v6900) + (Lanes([v17736[0], v17736[1], 0.0, v17736[2], 0.0, 0.0])));
                        let v17742 = v9404 - (Lanes([v9402[0], v9402[1], 0.0]));
                        let v6907 = v6898 * (v6899 - v818);
                        let v6909 = v772 - (v4331 - v818);
                        let v17747 = v17742 * v6901;
                        let v17748 = (v17742 * v6898) * v6909;
                        let v6913 = ((v825 - v818) * v6901) - (v6907 * v6909);
                        let v17753 = (Lanes([v17747[0], v17747[1], 0.0, v17747[2], 0.0, 0.0])) - ((Lanes([v17748[0], v17748[1], 0.0, v17748[2], 0.0, 0.0])) + (((v9432 - (Lanes([v9402[0], v9402[1], 0.0, 0.0, 0.0, 0.0]))) * v10382) * v6907));
                        v7796 = v6913;
                        v7815 = v6905;
                        v8620 = v0;
                        v8626 = v0;
                        v8638 = v0;
                        v8649 = v0;
                        v10047 = v17753;
                        v10048 = v17740;
                        v10049 = v10571;
                        v10050 = v10571;
                        v10051 = v10571;
                        v10052 = v10571;
                    } else {
                        let v6915 = (v34 / v485).sqrt();
                        let v6916 = v745 * v6915;
                        let v16662 = v10477 * v6915;
                        let v6955: f64;
                        let v6977: f64;
                        let v7339: f64;
                        let v7345: f64;
                        let v10053: Lanes<3>;
                        let v10054: Lanes<4>;
                        if v5 != 0.0 {
                            let v6922 = (v6041 * v830) + (v6043 * (v830 - v818));
                            let v16677 = (v9405 * v6041) + ((v9405 - v10552) * v6043);
                            let v16681 = (v9402 * v6041) + ((v9402 * v10382) * v6043);
                            let v16686 = (v9404 * v6041) + ((v9404 - (Lanes([v9402[0], v9402[1], 0.0]))) * v6043);
                            let v6932 = ((v6041 * v825) + (v6043 * (v825 - v818))) - v6922;
                            let v16691 = (Lanes([v16686[0], v16686[1], v16686[2], 0.0])) - (Lanes([v16677[0], v16677[1], 0.0, v16677[2]]));
                            let v6935 = v6041 + (v6918 * v6043);
                            let v6937 = v6043 + (v6918 * v6041);
                            let v16695 = ((v16677 * v10382) * v6935) + (((Lanes([v16681[0], v16681[1], 0.0])) - v16677) * v6937);
                            let v6942 = ((v6935 * (-v6922)) + (v6937 * (((v6041 * v818) + (v6043 * (-v818))) - v6922))) + v6941;
                            v6955 = v6942;
                            v6977 = v6932;
                            v7339 = v6935;
                            v7345 = v6937;
                            v10053 = v16695;
                            v10054 = v16691;
                        } else {
                            let v6944 = v6041 + (v6918 * v6043);
                            let v6946 = v6043 + (v6918 * v6041);
                            let v6979: f64;
                            let v10055: Lanes<3>;
                            if v6917 != 0.0 {
                                let v6950 = (v6041 * v825) + (v6043 * (v825 - v818));
                                let v16667 = (v9404 * v6041) + ((v9404 - (Lanes([v9402[0], v9402[1], 0.0]))) * v6043);
                                v6979 = v6950;
                                v10055 = v16667;
                            } else {
                                v6979 = v0;
                                v10055 = v10523;
                            }
                            let v6978: f64;
                            let v10056: Lanes<3>;
                            if v6918 != 0.0 {
                                let v6954 = (v6043 * v825) + (v6041 * (v825 - v818));
                                let v16672 = (v9404 * v6043) + ((v9404 - (Lanes([v9402[0], v9402[1], 0.0]))) * v6041);
                                v6978 = v6954;
                                v10056 = v16672;
                            } else {
                                v6978 = v6979;
                                v10056 = v10055;
                            }
                            let v16673 = Lanes([v10056[0], v10056[1], v10056[2], 0.0]);
                            v6955 = v0;
                            v6977 = v6978;
                            v7339 = v6944;
                            v7345 = v6946;
                            v10053 = v10496;
                            v10054 = v16673;
                        }
                        let v6956 = -v6955;
                        let v16696 = v10053 * v10382;
                        let v6957 = if v6956 > v778 { 1.0 } else { 0.0 };
                        let v6972: f64;
                        let v10057: Lanes<3>;
                        if v6957 != 0.0 {
                            let v6959 = v774 - v778;
                            let v6960 = (v6956 - v778) / v6959;
                            let v16697 = v16696 / v6959;
                            let v6961 = v6960 * v6960;
                            let v16698 = v16697 * v6960;
                            let v16699 = v16698 + v16698;
                            let v16703 = v16699 * v6961;
                            let v6967 = (((v4 + v6960) + v6961) + (v6961 * v6960)) + (v6961 * v6961);
                            let v6968 = v4 / v6967;
                            let v16712 = (((((((v16697 + v16699) + ((v16699 * v6960) + (v16697 * v6961))) + (v16703 + v16703)) * v6968) * v10382) / v6967) * v10382) * v6959;
                            let v6971 = v778 + (v6959 * (v4 - v6968));
                            v6972 = v6971;
                            v10057 = v16712;
                        } else {
                            v6972 = v6956;
                            v10057 = v16696;
                        }
                        let v16713 = v10057 * v10382;
                        let v6974 = (-v6972) - v6;
                        let v6975 = v6916 * v6884;
                        let v16714 = v16662 * v6884;
                        let v6976 = v6975 * v6975;
                        let v16715 = v16714 * v6975;
                        let v16716 = v16715 + v16715;
                        let v16717 = v10054 * v10382;
                        let v6981 = (-v6977) + v61;
                        let v6982 = v34 / v726;
                        let v6983 = v73 / v658;
                        let v6984 = v6982.ln();
                        let v6985 = v6983 * v6984;
                        let v16728 = ((((v10402 * v6983) * v10382) / v658) * v6984) + (((((v10445 * v6982) * v10382) / v726) * (v9360 / v6982)) * v6983);
                        let v6986 = -v6974;
                        let v16729 = v16713 * v10382;
                        let v6987 = if v6981 < v6986 { 1.0 } else { 0.0 };
                        let v7332: f64;
                        let v7334: f64;
                        let v7744: f64;
                        let v10058: Lanes<5>;
                        let v10059: Lanes<5>;
                        let v10060: Lanes<5>;
                        if v6987 != 0.0 {
                            let v6988 = v658 * v6916;
                            let v6989 = v4 / v6988;
                            let v6990 = v6989 * v122;
                            let v17122 = (((((v10402 * v6916) + (v16662 * v658)) * v6989) * v10382) / v6988) * v122;
                            let v17123 = v17122 * v6991;
                            let v6993 = v73 + (v6991 * v6990);
                            let v6994 = v86 * v6993;
                            let v6995 = v6994 * v6993;
                            let v6996 = v6995 * v6993;
                            let v17130 = ((((v17123 * v86) * v6993) + (v17123 * v6994)) * v6993) + (v17123 * v6995);
                            let v6997 = v656 - v6985;
                            let v17131 = v10398 - v16728;
                            let v6998 = v6981 + v6974;
                            let v17134 = v10402 * v6998;
                            let v17135 = (v16717 + (Lanes([v16713[0], v16713[1], 0.0, v16713[2]]))) * v658;
                            let v7001 = v3495 * v6990;
                            let v7002 = (v658 * v6998) - v73;
                            let v7003 = v7001 * v7002;
                            let v17140 = (v17122 * v3495) * v7002;
                            let v17143 = (Lanes([0.0, 0.0, v17140[0], 0.0, 0.0])) + (((Lanes([0.0, 0.0, v17134[0], 0.0, 0.0])) + (Lanes([v17135[0], v17135[1], 0.0, v17135[2], v17135[3]]))) * v7001);
                            let v7004 = v7000 - v7003;
                            let v17144 = v17143 * v10382;
                            let v7005 = v7004 * v7004;
                            let v17145 = v17144 * v7004;
                            let v17146 = v17145 + v17145;
                            let v7007 = if v6996 < (v7005 * v3501) { 1.0 } else { 0.0 };
                            let v7019: f64;
                            let v10061: Lanes<5>;
                            if v7007 != 0.0 {
                                let v17153 = v17130 * v8;
                                let v7011 = (v8 * v6996) / v7004;
                                let v7013 = ((v7008 + v7004) + v7011) + v7003;
                                let v17159 = (v17144 + (((Lanes([0.0, 0.0, v17153[0], 0.0, 0.0])) - (v17144 * v7011)) / v7004)) + v17143;
                                v7019 = v7013;
                                v10061 = v17159;
                            } else {
                                let v7015 = (v6996 + v7005).sqrt();
                                let v7018 = (v7016 + v7015) + v7003;
                                let v17152 = (((Lanes([0.0, 0.0, v17130[0], 0.0, 0.0])) + v17146) * (v9360 / (v10427 * v7015))) + v17143;
                                v7019 = v7018;
                                v10061 = v17152;
                            }
                            let v7020 = v7019.powf(v1557);
                            let v17163 = v10061 * (v1557 * (v7019.powf(v17160)));
                            let v17165 = (v17122 * v3518) * v10382;
                            let v7026 = v743 * v7020;
                            let v7029 = (((v7021 - (v3518 * v6990)) + (v73 * v7020)) + (v7026 * v7020)) / v7020;
                            let v17178 = v10407 * v7029;
                            let v17181 = Lanes([v16713[0], v16713[1], 0.0, 0.0, v16713[2]]);
                            let v7032 = ((v7029 * v660) - v6974) + v6974;
                            let v17183 = ((((((((Lanes([0.0, 0.0, v17165[0], 0.0, 0.0])) + (v17163 * v73)) + (((v17163 * v743) * v7020) + (v17163 * v7026))) - (v17163 * v7029)) / v7020) * v660) + (Lanes([0.0, 0.0, v17178[0], 0.0, 0.0]))) - v17181) + v17181;
                            let v7033 = v7032 / v6997;
                            let v17184 = v17131 * v7033;
                            let v17188 = ((v17183 - (Lanes([0.0, 0.0, v17184[0], 0.0, 0.0]))) / v6997) * v7033;
                            let v7036 = (v4 + (v7033 * v7033)).sqrt();
                            let v7037 = v7032 / v7036;
                            let v7040 = v122 * (v6981 - (v7037 - v6974));
                            let v17199 = ((Lanes([v16717[0], v16717[1], 0.0, v16717[2], v16717[3]])) - (((v17183 - (((v17188 + v17188) * (v9360 / (v10427 * v7036))) * v7037)) / v7036) - v17181)) * v122;
                            v7332 = v7040;
                            v7334 = v7040;
                            v7744 = v0;
                            v10058 = v17199;
                            v10059 = v17199;
                            v10060 = v10571;
                        } else {
                            let v7042 = v6981 + v6974;
                            let v16731 = v16717 + (Lanes([v16713[0], v16713[1], 0.0, v16713[2]]));
                            let v16732 = v10402 * v7042;
                            let v16733 = v16731 * v658;
                            let v16735 = Lanes([v16733[0], v16733[1], 0.0, v16733[2], v16733[3]]);
                            let v16736 = (Lanes([0.0, 0.0, v16732[0], 0.0, 0.0])) + v16735;
                            let v7044 = (v658 * v7042) - v4;
                            let v7047 = v6976 * v659;
                            let v16740 = (v16716 * v659) + (v10404 * v6976);
                            let v7048 = (v85 * (v7044 + v7041)) / v7047;
                            let v16741 = v16740 * v7048;
                            let v16744 = ((v16736 * v85) - (Lanes([0.0, 0.0, v16741[0], 0.0, 0.0]))) / v7047;
                            let v7049 = v4 + v7048;
                            let v7051 = if v7049 < v7050 { 1.0 } else { 0.0 };
                            let v7055: f64;
                            let v10062: Lanes<5>;
                            if v7051 != 0.0 {
                                v7055 = v7052;
                                v10062 = v10571;
                            } else {
                                v7055 = v7049;
                                v10062 = v16744;
                            }
                            let v7054 = (v6976 * v658) / v73;
                            let v16748 = ((v16716 * v658) + (v10402 * v6976)) / v73;
                            let v7056 = v7055.sqrt();
                            let v7057 = v4 - v7056;
                            let v16753 = v16748 * v7057;
                            let v16757 = Lanes([v16717[0], v16717[1], 0.0, v16717[2], v16717[3]]);
                            let v7060 = (v6981 + (v7054 * v7057)) + v6974;
                            let v16759 = Lanes([v16713[0], v16713[1], 0.0, 0.0, v16713[2]]);
                            let v16761 = v10402 * v7060;
                            let v7063 = (-(v658 * v7060)).exp();
                            let v7066 = (v85 * (v7044 + v7063)) / v7047;
                            let v16769 = v16740 * v7066;
                            let v16772 = (((v16736 + ((((Lanes([0.0, 0.0, v16761[0], 0.0, 0.0])) + (((v16757 + ((Lanes([0.0, 0.0, v16753[0], 0.0, 0.0])) + (((v10062 * (v9360 / (v10427 * v7056))) * v10382) * v7054))) + v16759) * v658)) * v10382) * v7063)) * v85) - (Lanes([0.0, 0.0, v16769[0], 0.0, 0.0]))) / v7047;
                            let v7067 = v4 + v7066;
                            let v7069 = if v7067 < v7068 { 1.0 } else { 0.0 };
                            let v7071: f64;
                            let v10063: Lanes<5>;
                            if v7069 != 0.0 {
                                v7071 = v7070;
                                v10063 = v10571;
                            } else {
                                v7071 = v7067;
                                v10063 = v16772;
                            }
                            let v7072 = v7071.sqrt();
                            let v7073 = v4 - v7072;
                            let v16777 = v16748 * v7073;
                            let v7076 = (v6981 + (v7054 * v7073)) + v6974;
                            let v7077 = v658 * v7076;
                            let v16783 = v10402 * v7076;
                            let v16786 = (Lanes([0.0, 0.0, v16783[0], 0.0, 0.0])) + (((v16757 + ((Lanes([0.0, 0.0, v16777[0], 0.0, 0.0])) + (((v10063 * (v9360 / (v10427 * v7072))) * v10382) * v7054))) + v16759) * v658);
                            let v7078 = if v7077 < v91 { 1.0 } else { 0.0 };
                            let v7157: f64;
                            let v10064: Lanes<5>;
                            if v7078 != 0.0 {
                                let v7081 = v658 * v6975;
                                let v7082 = v4 / v7081;
                                let v16792 = ((((v10402 * v6975) + (v16714 * v658)) * v7082) * v10382) / v7081;
                                let v7083 = v7080 + v7082;
                                let v16793 = v16731 * v10382;
                                let v7085 = (-v7042) / v6975;
                                let v16794 = v16714 * v7085;
                                let v16801 = ((v16792 * v7079) / v7088) * v10382;
                                let v7093 = (v7086 - ((v7079 * v7083) / v7088)) + (v7085 / v7091);
                                let v16804 = (Lanes([0.0, 0.0, v16801[0], 0.0, 0.0])) + ((((Lanes([v16793[0], v16793[1], 0.0, v16793[2], v16793[3]])) - (Lanes([0.0, 0.0, v16794[0], 0.0, 0.0]))) / v6975) / v7091);
                                let v7099 = ((v7094 * v7083) - v7096) / v7098;
                                let v16806 = (v16792 * v7094) / v7098;
                                let v16807 = v16804 * v7093;
                                let v7101 = v7099 * v7099;
                                let v16809 = v16806 * v7099;
                                let v16813 = ((v16809 + v16809) * v7099) + (v16806 * v7101);
                                let v7104 = ((v7093 * v7093) + (v7101 * v7099)).sqrt();
                                let v16818 = ((v16807 + v16807) + (Lanes([0.0, 0.0, v16813[0], 0.0, 0.0]))) * (v9360 / (v10427 * v7104));
                                let v7106 = (-v7093) + v7104;
                                let v7108 = v7093 + v7104;
                                let v7113 = ((v7106.powf(v1557)) + (-(v7108.powf(v1557)))) - v7112;
                                let v16833 = v10407 * v7113;
                                let v7116 = ((v7113 * v660) - v6974) + v6974;
                                let v7117 = v658 * v7116;
                                let v16838 = v10402 * v7116;
                                let v16841 = (Lanes([0.0, 0.0, v16838[0], 0.0, 0.0])) + (((((((((v16804 * v10382) + v16818) * (v1557 * (v7106.powf(v16821)))) + (((v16804 + v16818) * (v1557 * (v7108.powf(v16826)))) * v10382)) * v660) + (Lanes([0.0, 0.0, v16833[0], 0.0, 0.0]))) - v16759) + v16759) * v658);
                                v7157 = v7117;
                                v10064 = v16841;
                            } else {
                                v7157 = v7077;
                                v10064 = v16786;
                            }
                            let v7119 = if v7118 > v0 { 1.0 } else { 0.0 };
                            let v7173: f64;
                            let v10065: Lanes<5>;
                            if v7119 != 0.0 {
                                let v7120 = v7042 + v74;
                                let v16842 = v10402 * v6986;
                                let v16843 = v16729 * v658;
                                let v7122 = (v658 * v6986).exp();
                                let v7123 = v7122 + v358;
                                let v7124 = v726 / v34;
                                let v7125 = v7124 * v7124;
                                let v16849 = (v10445 / v34) * v7124;
                                let v16850 = v16849 + v16849;
                                let v7126 = v7125 * v7123;
                                let v16851 = v16850 * v7123;
                                let v7127 = v658 * v7120;
                                let v16855 = v10402 * v7120;
                                let v16857 = (Lanes([0.0, 0.0, v16855[0], 0.0, 0.0])) + v16735;
                                let v7128 = v7126 * v7047;
                                let v16859 = v16740 * v7126;
                                let v16861 = (((Lanes([0.0, 0.0, v16851[0], 0.0])) + ((((Lanes([0.0, 0.0, v16842[0], 0.0])) + (Lanes([v16843[0], v16843[1], 0.0, v16843[2]]))) * v7122) * v7125)) * v7047) + (Lanes([0.0, 0.0, v16859[0], 0.0]));
                                let v16862 = v16857 * v7127;
                                let v7130 = v7128 + (v7127 * v7127);
                                let v16864 = Lanes([v16861[0], v16861[1], v16861[2], 0.0, v16861[3]]);
                                let v7132 = v7125 * v7047;
                                let v7133 = v7132.ln();
                                let v16872 = ((v16850 * v7047) + (v16740 * v7125)) * (v9360 / v7132);
                                let v16873 = Lanes([0.0, 0.0, v16872[0], 0.0, 0.0]);
                                let v7135 = v658 * v6974;
                                let v16875 = v10402 * v6974;
                                let v16876 = v16713 * v658;
                                let v16879 = (Lanes([0.0, 0.0, v16875[0], 0.0])) + (Lanes([v16876[0], v16876[1], 0.0, v16876[2]]));
                                let v16880 = Lanes([v16879[0], v16879[1], v16879[2], 0.0, v16879[3]]);
                                let v16882 = v16857 - ((((v16864 + (v16862 + v16862)) * (v9360 / v7130)) - v16873) + v16880);
                                let v7138 = (v7127 - (((v7130.ln()) - v7133) + v7135)) - v4;
                                let v7139 = v85 * v7127;
                                let v16883 = v16857 * v85;
                                let v7140 = if v7139 > v0 { 1.0 } else { 0.0 };
                                let v7142: f64;
                                let v10066: Lanes<5>;
                                if v7140 != 0.0 {
                                    v7142 = v7139;
                                    v10066 = v16883;
                                } else {
                                    let v7141 = -v7139;
                                    let v16884 = v16883 * v10382;
                                    v7142 = v7141;
                                    v10066 = v16884;
                                }
                                let v16885 = v16882 * v7138;
                                let v7145 = ((v7138 * v7138) + v7142).sqrt();
                                let v16895 = v10402 * v74;
                                let v7151 = (v7127 - (v7127 - (v8 * (v7138 + v7145)))) + (v658 * v74);
                                let v16898 = ((v16857 - (v16857 - ((v16882 + (((v16885 + v16885) + v10066) * (v9360 / (v10427 * v7145)))) * v8))) + (Lanes([0.0, 0.0, v16895[0], 0.0, 0.0]))) * v7151;
                                let v7153 = v7128 + (v7151 * v7151);
                                let v7156 = ((v7153.ln()) - v7133) + v7135;
                                let v16904 = (((v16864 + (v16898 + v16898)) * (v9360 / v7153)) - v16873) + v16880;
                                let v16905 = v16904 - v10064;
                                let v7160 = (v7156 - v7157) - v7159;
                                let v7163 = (v85 * v7156) * v7162;
                                let v16907 = (v16904 * v85) * v7162;
                                let v7164 = if v7163 > v0 { 1.0 } else { 0.0 };
                                let v7166: f64;
                                let v10067: Lanes<5>;
                                if v7164 != 0.0 {
                                    v7166 = v7163;
                                    v10067 = v16907;
                                } else {
                                    let v7165 = -v7163;
                                    let v16908 = v16907 * v10382;
                                    v7166 = v7165;
                                    v10067 = v16908;
                                }
                                let v16909 = v16905 * v7160;
                                let v7169 = ((v7160 * v7160) + v7166).sqrt();
                                let v7172 = v7156 - (v8 * (v7160 + v7169));
                                let v16917 = v16904 - ((v16905 + (((v16909 + v16909) + v10067) * (v9360 / (v10427 * v7169)))) * v8);
                                v7173 = v7172;
                                v10065 = v16917;
                            } else {
                                v7173 = v7157;
                                v10065 = v10064;
                            }
                            let v7174 = v7173 / v658;
                            let v16918 = v10402 * v7174;
                            let v7175 = v7174 - v6974;
                            let v16922 = ((v10065 - (Lanes([0.0, 0.0, v16918[0], 0.0, 0.0]))) / v658) - v16759;
                            let v7178 = (-v7173).exp();
                            let v7179 = (v7173 - v4) + v7178;
                            let v16925 = v10065 + ((v10065 * v10382) * v7178);
                            let v7181 = if v7179 < v7180 { 1.0 } else { 0.0 };
                            let v7183: f64;
                            let v10068: Lanes<5>;
                            if v7181 != 0.0 {
                                v7183 = v7182;
                                v10068 = v10571;
                            } else {
                                v7183 = v7179;
                                v10068 = v16925;
                            }
                            let v7184 = v7183.sqrt();
                            let v7185 = v6916 * v7184;
                            let v16929 = v16662 * v7184;
                            let v16932 = (Lanes([0.0, 0.0, v16929[0], 0.0, 0.0])) + ((v10068 * (v9360 / (v10427 * v7184))) * v6916);
                            let v7187 = v122 * (v6981 - v7175);
                            let v16934 = (v16757 - v16922) * v122;
                            let v7188 = if v7118 == v4 { 1.0 } else { 0.0 };
                            let v7333: f64;
                            let v7335: f64;
                            let v7745: f64;
                            let v10069: Lanes<5>;
                            let v10070: Lanes<5>;
                            let v10071: Lanes<5>;
                            if v7188 != 0.0 {
                                let v16935 = v10402 * v6986;
                                let v16936 = v16729 * v658;
                                let v7190 = (v658 * v6986).exp();
                                let v16940 = ((Lanes([0.0, 0.0, v16935[0], 0.0])) + (Lanes([v16936[0], v16936[1], 0.0, v16936[2]]))) * v7190;
                                let v7191 = v726 / v34;
                                let v7192 = v7191 * v7191;
                                let v16942 = (v10445 / v34) * v7191;
                                let v16943 = v16942 + v16942;
                                let v7193 = v7192 * v7190;
                                let v16944 = v16943 * v7190;
                                let v16947 = (Lanes([0.0, 0.0, v16944[0], 0.0])) + (v16940 * v7192);
                                let mut v7194: f64 = 0.0;
                                let mut v7197: f64 = 0.0;
                                let mut v7283: f64 = 0.0;
                                let mut v7313: f64 = 0.0;
                                let mut v7316: f64 = 0.0;
                                let mut v7324: f64 = 0.0;
                                let mut v7327: f64 = 0.0;
                                let mut v10072: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10073: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10074: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10075: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10076: Lanes<5> = Lanes([0.0; 5]);
                                v7194 = v4;
                                v7197 = v7175;
                                v7283 = v0;
                                v7313 = v7173;
                                v7316 = v0;
                                v7324 = v0;
                                v7327 = v0;
                                v10072 = v16922;
                                v10073 = v10065;
                                v10074 = v10571;
                                v10075 = v10571;
                                v10076 = v10571;
                                loop {
                                    let v7196 = if v7194 <= v7195 { 1.0 } else { 0.0 };
                                    if v7196 == 0.0 {
                                        break;
                                    }
                                    let v7198 = v7197 + v6974;
                                    let v7199 = v658 * v7198;
                                    let v16968 = v10402 * v7198;
                                    let v16971 = (Lanes([0.0, 0.0, v16968[0], 0.0, 0.0])) + ((v10072 + v16759) * v658);
                                    let v7200 = if v7199 < v639 { 1.0 } else { 0.0 };
                                    let v7276: f64;
                                    let v7280: f64;
                                    let v7317: f64;
                                    let v7328: f64;
                                    let v10077: Lanes<5>;
                                    let v10078: Lanes<5>;
                                    let v10079: Lanes<5>;
                                    let v10080: Lanes<5>;
                                    if v7200 != 0.0 {
                                        let v7201 = v7199 * v7199;
                                        let v17013 = v16971 * v7199;
                                        let v17014 = v17013 + v17013;
                                        let v7202 = v7201 * v7199;
                                        let v7205 = v7203 + (v7199 * v6313);
                                        let v7207 = v6311 + (v7199 * v7205);
                                        let v7208 = v7202 * v7207;
                                        let v17024 = (((v17014 * v7199) + (v16971 * v7201)) * v7207) + (((v16971 * v7205) + ((v16971 * v6313) * v7199)) * v7202);
                                        let v7211 = v7199 * v639;
                                        let v17025 = v16971 * v639;
                                        let v7213 = v7210 + (v7211 * v6313);
                                        let v7215 = v7209 + (v7199 * v7213);
                                        let v7216 = v7201 * v7215;
                                        let v7217 = v7193 * v7208;
                                        let v17033 = v16947 * v7208;
                                        let v7218 = v7217 * v7208;
                                        let v17039 = (((Lanes([v17033[0], v17033[1], v17033[2], 0.0, v17033[3]])) + (v17024 * v7193)) * v7208) + (v17024 * v7217);
                                        let v17041 = v10402 * v7193;
                                        let v7220 = (v7193 * v658) * v73;
                                        let v7221 = v7220 * v7208;
                                        let v17045 = (((v16947 * v658) + (Lanes([0.0, 0.0, v17041[0], 0.0]))) * v73) * v7208;
                                        let v7226 = v7224 + (v7199 * v6337);
                                        let v7228 = v6335 + (v7199 * v7226);
                                        let v7230 = v7223 + (v7199 * v7228);
                                        let v7232 = v6333 + (v7199 * v7230);
                                        let v7233 = v7199 * v7232;
                                        let v17064 = (v16971 * v7232) + (((v16971 * v7230) + (((v16971 * v7228) + (((v16971 * v7226) + ((v16971 * v6337) * v7199)) * v7199)) * v7199)) * v7199);
                                        let v7238 = v7236 + (v7211 * v6337);
                                        let v7240 = v7235 + (v7199 * v7238);
                                        let v7242 = v7234 + (v7199 * v7240);
                                        let v7244 = v6333 + (v7199 * v7242);
                                        let v17075 = v17064 * v7233;
                                        let v7248 = (((v7233 * v7233) + v7218) + v358).sqrt();
                                        let v17080 = ((v17075 + v17075) + v17039) * (v9360 / (v10427 * v7248));
                                        let v17081 = v10402 * v7244;
                                        let v7250 = (v658 * v7244) * v73;
                                        let v7253 = v7248 + v7248;
                                        let v7254 = ((v7250 * v7233) + (v7221 * v7216)) / v7253;
                                        let v17093 = (((((((Lanes([0.0, 0.0, v17081[0], 0.0, 0.0])) + (((v16971 * v7242) + (((v16971 * v7240) + (((v16971 * v7238) + ((v17025 * v6337) * v7199)) * v7199)) * v7199)) * v658)) * v73) * v7233) + (v17064 * v7250)) + ((((Lanes([v17045[0], v17045[1], v17045[2], 0.0, v17045[3]])) + (v17024 * v7220)) * v7216) + (((v17014 * v7215) + (((v16971 * v7213) + ((v17025 * v6313) * v7199)) * v7201)) * v7221))) - ((v17080 + v17080) * v7254)) / v7253;
                                        v7276 = v7248;
                                        v7280 = v7254;
                                        v7317 = v7233;
                                        v7328 = v7218;
                                        v10077 = v17080;
                                        v10078 = v17093;
                                        v10079 = v17064;
                                        v10080 = v17039;
                                    } else {
                                        let v7255 = if v7199 < v2530 { 1.0 } else { 0.0 };
                                        let v7268: f64;
                                        let v7271: f64;
                                        let v10081: Lanes<5>;
                                        let v10082: Lanes<5>;
                                        if v7255 != 0.0 {
                                            let v7256 = v7199.exp();
                                            let v16990 = v16971 * v7256;
                                            let v7257 = v7256 - v4;
                                            let v7258 = v7193 * v7257;
                                            let v16991 = v16947 * v7257;
                                            let v16994 = (Lanes([v16991[0], v16991[1], v16991[2], 0.0, v16991[3]])) + (v16990 * v7193);
                                            let v7259 = v7193 * v658;
                                            let v16996 = v10402 * v7193;
                                            let v7260 = v7259 * v7256;
                                            let v16999 = ((v16947 * v658) + (Lanes([0.0, 0.0, v16996[0], 0.0]))) * v7256;
                                            let v17002 = (Lanes([v16999[0], v16999[1], v16999[2], 0.0, v16999[3]])) + (v16990 * v7259);
                                            v7268 = v7258;
                                            v7271 = v7260;
                                            v10081 = v16994;
                                            v10082 = v17002;
                                        } else {
                                            let v16972 = v10402 * v7197;
                                            let v7262 = (v658 * v7197).exp();
                                            let v16976 = ((Lanes([0.0, 0.0, v16972[0], 0.0, 0.0])) + (v10072 * v658)) * v7262;
                                            let v7263 = v7262 - v7190;
                                            let v7264 = v7192 * v7263;
                                            let v16979 = v16943 * v7263;
                                            let v16982 = (Lanes([0.0, 0.0, v16979[0], 0.0, 0.0])) + ((v16976 - (Lanes([v16940[0], v16940[1], v16940[2], 0.0, v16940[3]]))) * v7192);
                                            let v7265 = v7192 * v658;
                                            let v7266 = v7265 * v7262;
                                            let v16986 = ((v16943 * v658) + (v10402 * v7192)) * v7262;
                                            let v16989 = (Lanes([0.0, 0.0, v16986[0], 0.0, 0.0])) + (v16976 * v7265);
                                            v7268 = v7264;
                                            v7271 = v7266;
                                            v10081 = v16982;
                                            v10082 = v16989;
                                        }
                                        let v7270 = ((v7199 - v4) + v7268).sqrt();
                                        let v17006 = (v16971 + v10081) * (v9360 / (v10427 * v7270));
                                        let v7273 = (v658 + v7271) / v7270;
                                        let v7274 = v7273 * v8;
                                        let v17012 = ((((Lanes([0.0, 0.0, v10402[0], 0.0, 0.0])) + v10082) - (v17006 * v7273)) / v7270) * v8;
                                        v7276 = v7270;
                                        v7280 = v7274;
                                        v7317 = v0;
                                        v7328 = v7268;
                                        v10077 = v17006;
                                        v10078 = v17012;
                                        v10079 = v10571;
                                        v10080 = v10081;
                                    }
                                    let v17095 = v16714 * v7276;
                                    let v7278 = (v6981 - v7197) - (v6975 * v7276);
                                    let v17099 = (v16757 - v10072) - ((Lanes([0.0, 0.0, v17095[0], 0.0, 0.0])) + (v10077 * v6975));
                                    let v17100 = v16714 * v7280;
                                    let v7282 = v7279 - (v6975 * v7280);
                                    let v17104 = ((Lanes([0.0, 0.0, v17100[0], 0.0, 0.0])) + (v10078 * v6975)) * v10382;
                                    let v7284 = if v7283 == v4 { 1.0 } else { 0.0 };
                                    let v7307: f64;
                                    let v7309: f64;
                                    let v7310: f64;
                                    let v10083: Lanes<5>;
                                    if v7284 != 0.0 {
                                        v7307 = v7285;
                                        v7309 = v7197;
                                        v7310 = v7283;
                                        v10083 = v10072;
                                    } else {
                                        let v7287 = (-v7278) / v7282;
                                        let v17108 = ((v17099 * v10382) - (v17104 * v7287)) / v7282;
                                        let v7289 = v7197.abs();
                                        let v17112 = v10072 * ((v10427 * (if v7197 >= v11296 { 1.0 } else { 0.0 })) - v9360);
                                        let v7290 = if v4 >= v7289 { 1.0 } else { 0.0 };
                                        let v7291: f64;
                                        let v10084: Lanes<5>;
                                        if v7290 != 0.0 {
                                            v7291 = v4;
                                            v10084 = v10571;
                                        } else {
                                            v7291 = v7289;
                                            v10084 = v17112;
                                        }
                                        let v7293 = v7288 * (v4 + v7291);
                                        let v17113 = v10084 * v7288;
                                        let v7295 = if (v7287.abs()) > v7293 { 1.0 } else { 0.0 };
                                        let v7300: f64;
                                        let v10085: Lanes<5>;
                                        if v7295 != 0.0 {
                                            let v7296 = if v7287 >= v0 { 1.0 } else { 0.0 };
                                            let v7298: f64;
                                            if v7296 != 0.0 {
                                                v7298 = v4;
                                            } else {
                                                v7298 = v7297;
                                            }
                                            let v7299 = v7293 * v7298;
                                            let v17114 = v17113 * v7298;
                                            v7300 = v7299;
                                            v10085 = v17114;
                                        } else {
                                            v7300 = v7287;
                                            v10085 = v17108;
                                        }
                                        let v7301 = v7197 + v7300;
                                        let v17115 = v10072 + v10085;
                                        let v7306 = if (if (v7300.abs()) <= v856 { 1.0 } else { 0.0 }) != 0.0 && (if (v7278.abs()) <= v3501 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v7311: f64;
                                        if v7306 != 0.0 {
                                            v7311 = v4;
                                        } else {
                                            v7311 = v7283;
                                        }
                                        v7307 = v7194;
                                        v7309 = v7301;
                                        v7310 = v7311;
                                        v10083 = v17115;
                                    }
                                    let v7308 = v7307 + v4;
                                    v7194 = v7308;
                                    v7197 = v7309;
                                    v7283 = v7310;
                                    v7313 = v7199;
                                    v7316 = v7317;
                                    v7324 = v7276;
                                    v7327 = v7328;
                                    v10072 = v10083;
                                    v10073 = v16971;
                                    v10074 = v10079;
                                    v10075 = v10077;
                                    v10076 = v10080;
                                }
                                let v7312 = if v7283 == v0 { 1.0 } else { 0.0 };
                                if v7312 != 0.0 {
                                } else {
                                }
                                let v7314 = if v7313 < v639 { 1.0 } else { 0.0 };
                                let v7322: f64;
                                let v10086: Lanes<5>;
                                if v7314 != 0.0 {
                                    let v7315 = if v7313 < v91 { 1.0 } else { 0.0 };
                                    if v7315 != 0.0 {
                                    } else {
                                    }
                                    let v7319 = v7316 + v7318;
                                    v7322 = v7319;
                                    v10086 = v10074;
                                } else {
                                    let v7321 = (v7313 - v4).sqrt();
                                    let v16950 = v10073 * (v9360 / (v10427 * v7321));
                                    v7322 = v7321;
                                    v10086 = v16950;
                                }
                                let v7323 = v6916 * v7322;
                                let v16951 = v16662 * v7322;
                                let v16954 = (Lanes([0.0, 0.0, v16951[0], 0.0, 0.0])) + (v10086 * v6916);
                                let v7325 = v7324 + v7322;
                                let v7326 = v4 / v7325;
                                let v7329 = v6916 * v7327;
                                let v16959 = v16662 * v7327;
                                let v7331 = v7323 + (v7329 * v7326);
                                let v16966 = v16954 + ((((Lanes([0.0, 0.0, v16959[0], 0.0, 0.0])) + (v10076 * v6916)) * v7326) + (((((v10075 + v10086) * v7326) * v10382) / v7325) * v7329));
                                v7333 = v7331;
                                v7335 = v7323;
                                v7745 = v7316;
                                v10069 = v16966;
                                v10070 = v16954;
                                v10071 = v10074;
                            } else {
                                v7333 = v7187;
                                v7335 = v7185;
                                v7745 = v0;
                                v10069 = v16934;
                                v10070 = v16932;
                                v10071 = v10571;
                            }
                            v7332 = v7333;
                            v7334 = v7335;
                            v7744 = v7745;
                            v10058 = v10069;
                            v10059 = v10070;
                            v10060 = v10071;
                        }
                        let v7338: f64;
                        if v5 != 0.0 {
                            let v7336 = v6894 * v6887;
                            v7338 = v7336;
                        } else {
                            let v7337 = v164 * v6887;
                            v7338 = v7337;
                        }
                        let v7342 = if (if v7339 != 0.0 && v148 != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v6917 != 0.0 && v5 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v8622: f64;
                        let v8651: f64;
                        let v10087: Lanes<5>;
                        let v10088: Lanes<5>;
                        if v7342 != 0.0 {
                            let v7343 = v7338 * v7332;
                            let v17200 = v10058 * v7338;
                            let v7344 = v7338 * v7334;
                            let v17201 = v10059 * v7338;
                            v8622 = v7343;
                            v8651 = v7344;
                            v10087 = v17200;
                            v10088 = v17201;
                        } else {
                            v8622 = v0;
                            v8651 = v0;
                            v10087 = v10571;
                            v10088 = v10571;
                        }
                        let v7348 = if (if v7345 != 0.0 && v148 != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v6918 != 0.0 && v5 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v8628: f64;
                        let v8640: f64;
                        let v10089: Lanes<5>;
                        let v10090: Lanes<5>;
                        if v7348 != 0.0 {
                            let v7349 = v7338 * v7332;
                            let v17202 = v10058 * v7338;
                            let v7350 = v7338 * v7334;
                            let v17203 = v10059 * v7338;
                            v8628 = v7349;
                            v8640 = v7350;
                            v10089 = v17202;
                            v10090 = v17203;
                        } else {
                            v8628 = v0;
                            v8640 = v0;
                            v10089 = v10571;
                            v10090 = v10571;
                        }
                        let v7389: f64;
                        let v7409: f64;
                        let v7768: f64;
                        let v7774: f64;
                        let v10091: Lanes<3>;
                        let v10092: Lanes<4>;
                        if v5 != 0.0 {
                            let v7356 = (v6041 * v830) + (v6043 * (v830 - v818));
                            let v17219 = (v9405 * v6041) + ((v9405 - v10552) * v6043);
                            let v17223 = (v9402 * v6041) + ((v9402 * v10382) * v6043);
                            let v17228 = (v9404 * v6041) + ((v9404 - (Lanes([v9402[0], v9402[1], 0.0]))) * v6043);
                            let v7366 = ((v6041 * v825) + (v6043 * (v825 - v818))) - v7356;
                            let v17233 = (Lanes([v17228[0], v17228[1], v17228[2], 0.0])) - (Lanes([v17219[0], v17219[1], 0.0, v17219[2]]));
                            let v7369 = (v7351 * v6041) + v6043;
                            let v7371 = (v7351 * v6043) + v6041;
                            let v17237 = ((v17219 * v10382) * v7369) + (((Lanes([v17223[0], v17223[1], 0.0])) - v17219) * v7371);
                            let v7376 = ((v7369 * (-v7356)) + (v7371 * (((v6041 * v818) + (v6043 * (-v818))) - v7356))) + v7375;
                            v7389 = v7376;
                            v7409 = v7366;
                            v7768 = v7369;
                            v7774 = v7371;
                            v10091 = v17237;
                            v10092 = v17233;
                        } else {
                            let v7378 = (v7351 * v6041) + v6043;
                            let v7380 = (v7351 * v6043) + v6041;
                            let v7411: f64;
                            let v10093: Lanes<4>;
                            if v7351 != 0.0 {
                                let v7384 = (v6041 * v825) + (v6043 * (v825 - v818));
                                let v17208 = (v9404 * v6041) + ((v9404 - (Lanes([v9402[0], v9402[1], 0.0]))) * v6043);
                                let v17209 = Lanes([v17208[0], v17208[1], v17208[2], 0.0]);
                                v7411 = v7384;
                                v10093 = v17209;
                            } else {
                                v7411 = v6977;
                                v10093 = v10054;
                            }
                            let v7410: f64;
                            let v10094: Lanes<4>;
                            if v7352 != 0.0 {
                                let v7388 = (v6043 * v825) + (v6041 * (v825 - v818));
                                let v17214 = (v9404 * v6043) + ((v9404 - (Lanes([v9402[0], v9402[1], 0.0]))) * v6041);
                                let v17215 = Lanes([v17214[0], v17214[1], v17214[2], 0.0]);
                                v7410 = v7388;
                                v10094 = v17215;
                            } else {
                                v7410 = v7411;
                                v10094 = v10093;
                            }
                            v7389 = v0;
                            v7409 = v7410;
                            v7768 = v7378;
                            v7774 = v7380;
                            v10091 = v10496;
                            v10092 = v10094;
                        }
                        let v7390 = -v7389;
                        let v17238 = v10091 * v10382;
                        let v7391 = if v7390 > v778 { 1.0 } else { 0.0 };
                        let v7406: f64;
                        let v10095: Lanes<3>;
                        if v7391 != 0.0 {
                            let v7393 = v774 - v778;
                            let v7394 = (v7390 - v778) / v7393;
                            let v17239 = v17238 / v7393;
                            let v7395 = v7394 * v7394;
                            let v17240 = v17239 * v7394;
                            let v17241 = v17240 + v17240;
                            let v17245 = v17241 * v7395;
                            let v7401 = (((v4 + v7394) + v7395) + (v7395 * v7394)) + (v7395 * v7395);
                            let v7402 = v4 / v7401;
                            let v17254 = (((((((v17239 + v17241) + ((v17241 * v7394) + (v17239 * v7395))) + (v17245 + v17245)) * v7402) * v10382) / v7401) * v10382) * v7393;
                            let v7405 = v778 + (v7393 * (v4 - v7402));
                            v7406 = v7405;
                            v10095 = v17254;
                        } else {
                            v7406 = v7390;
                            v10095 = v17238;
                        }
                        let v17255 = v10095 * v10382;
                        let v7408 = (-v7406) - v6;
                        let v17256 = v10092 * v10382;
                        let v7413 = (-v7409) + v61;
                        let v7414 = -v7408;
                        let v17257 = v17255 * v10382;
                        let v7415 = if v7413 < v7414 { 1.0 } else { 0.0 };
                        let v7761: f64;
                        let v7763: f64;
                        let v10096: Lanes<5>;
                        let v10097: Lanes<5>;
                        if v7415 != 0.0 {
                            let v7416 = v658 * v6916;
                            let v7417 = v4 / v7416;
                            let v7418 = v7417 * v122;
                            let v17650 = (((((v10402 * v6916) + (v16662 * v658)) * v7417) * v10382) / v7416) * v122;
                            let v17651 = v17650 * v7419;
                            let v7421 = v73 + (v7419 * v7418);
                            let v7422 = v86 * v7421;
                            let v7423 = v7422 * v7421;
                            let v7424 = v7423 * v7421;
                            let v17658 = ((((v17651 * v86) * v7421) + (v17651 * v7422)) * v7421) + (v17651 * v7423);
                            let v7425 = v656 - v6985;
                            let v17659 = v10398 - v16728;
                            let v7426 = v7413 + v7408;
                            let v17662 = v10402 * v7426;
                            let v17663 = (v17256 + (Lanes([v17255[0], v17255[1], 0.0, v17255[2]]))) * v658;
                            let v7429 = v3495 * v7418;
                            let v7430 = (v658 * v7426) - v73;
                            let v7431 = v7429 * v7430;
                            let v17668 = (v17650 * v3495) * v7430;
                            let v17671 = (Lanes([0.0, 0.0, v17668[0], 0.0, 0.0])) + (((Lanes([0.0, 0.0, v17662[0], 0.0, 0.0])) + (Lanes([v17663[0], v17663[1], 0.0, v17663[2], v17663[3]]))) * v7429);
                            let v7432 = v7428 - v7431;
                            let v17672 = v17671 * v10382;
                            let v7433 = v7432 * v7432;
                            let v17673 = v17672 * v7432;
                            let v17674 = v17673 + v17673;
                            let v7435 = if v7424 < (v7433 * v3501) { 1.0 } else { 0.0 };
                            let v7447: f64;
                            let v10098: Lanes<5>;
                            if v7435 != 0.0 {
                                let v17681 = v17658 * v8;
                                let v7439 = (v8 * v7424) / v7432;
                                let v7441 = ((v7436 + v7432) + v7439) + v7431;
                                let v17687 = (v17672 + (((Lanes([0.0, 0.0, v17681[0], 0.0, 0.0])) - (v17672 * v7439)) / v7432)) + v17671;
                                v7447 = v7441;
                                v10098 = v17687;
                            } else {
                                let v7443 = (v7424 + v7433).sqrt();
                                let v7446 = (v7444 + v7443) + v7431;
                                let v17680 = (((Lanes([0.0, 0.0, v17658[0], 0.0, 0.0])) + v17674) * (v9360 / (v10427 * v7443))) + v17671;
                                v7447 = v7446;
                                v10098 = v17680;
                            }
                            let v7448 = v7447.powf(v1557);
                            let v17691 = v10098 * (v1557 * (v7447.powf(v17688)));
                            let v17693 = (v17650 * v3518) * v10382;
                            let v7454 = v743 * v7448;
                            let v7457 = (((v7449 - (v3518 * v7418)) + (v73 * v7448)) + (v7454 * v7448)) / v7448;
                            let v17706 = v10407 * v7457;
                            let v17709 = Lanes([v17255[0], v17255[1], 0.0, 0.0, v17255[2]]);
                            let v7460 = ((v7457 * v660) - v7408) + v7408;
                            let v17711 = ((((((((Lanes([0.0, 0.0, v17693[0], 0.0, 0.0])) + (v17691 * v73)) + (((v17691 * v743) * v7448) + (v17691 * v7454))) - (v17691 * v7457)) / v7448) * v660) + (Lanes([0.0, 0.0, v17706[0], 0.0, 0.0]))) - v17709) + v17709;
                            let v7461 = v7460 / v7425;
                            let v17712 = v17659 * v7461;
                            let v17716 = ((v17711 - (Lanes([0.0, 0.0, v17712[0], 0.0, 0.0]))) / v7425) * v7461;
                            let v7464 = (v4 + (v7461 * v7461)).sqrt();
                            let v7465 = v7460 / v7464;
                            let v7468 = v122 * (v7413 - (v7465 - v7408));
                            let v17727 = ((Lanes([v17256[0], v17256[1], 0.0, v17256[2], v17256[3]])) - (((v17711 - (((v17716 + v17716) * (v9360 / (v10427 * v7464))) * v7465)) / v7464) - v17709)) * v122;
                            v7761 = v7468;
                            v7763 = v7468;
                            v10096 = v17727;
                            v10097 = v17727;
                        } else {
                            let v7470 = v7413 + v7408;
                            let v17259 = v17256 + (Lanes([v17255[0], v17255[1], 0.0, v17255[2]]));
                            let v17260 = v10402 * v7470;
                            let v17261 = v17259 * v658;
                            let v17263 = Lanes([v17261[0], v17261[1], 0.0, v17261[2], v17261[3]]);
                            let v17264 = (Lanes([0.0, 0.0, v17260[0], 0.0, 0.0])) + v17263;
                            let v7472 = (v658 * v7470) - v4;
                            let v7475 = v6976 * v659;
                            let v17268 = (v16716 * v659) + (v10404 * v6976);
                            let v7476 = (v85 * (v7472 + v7469)) / v7475;
                            let v17269 = v17268 * v7476;
                            let v17272 = ((v17264 * v85) - (Lanes([0.0, 0.0, v17269[0], 0.0, 0.0]))) / v7475;
                            let v7477 = v4 + v7476;
                            let v7479 = if v7477 < v7478 { 1.0 } else { 0.0 };
                            let v7483: f64;
                            let v10099: Lanes<5>;
                            if v7479 != 0.0 {
                                v7483 = v7480;
                                v10099 = v10571;
                            } else {
                                v7483 = v7477;
                                v10099 = v17272;
                            }
                            let v7482 = (v6976 * v658) / v73;
                            let v17276 = ((v16716 * v658) + (v10402 * v6976)) / v73;
                            let v7484 = v7483.sqrt();
                            let v7485 = v4 - v7484;
                            let v17281 = v17276 * v7485;
                            let v17285 = Lanes([v17256[0], v17256[1], 0.0, v17256[2], v17256[3]]);
                            let v7488 = (v7413 + (v7482 * v7485)) + v7408;
                            let v17287 = Lanes([v17255[0], v17255[1], 0.0, 0.0, v17255[2]]);
                            let v17289 = v10402 * v7488;
                            let v7491 = (-(v658 * v7488)).exp();
                            let v7494 = (v85 * (v7472 + v7491)) / v7475;
                            let v17297 = v17268 * v7494;
                            let v17300 = (((v17264 + ((((Lanes([0.0, 0.0, v17289[0], 0.0, 0.0])) + (((v17285 + ((Lanes([0.0, 0.0, v17281[0], 0.0, 0.0])) + (((v10099 * (v9360 / (v10427 * v7484))) * v10382) * v7482))) + v17287) * v658)) * v10382) * v7491)) * v85) - (Lanes([0.0, 0.0, v17297[0], 0.0, 0.0]))) / v7475;
                            let v7495 = v4 + v7494;
                            let v7497 = if v7495 < v7496 { 1.0 } else { 0.0 };
                            let v7499: f64;
                            let v10100: Lanes<5>;
                            if v7497 != 0.0 {
                                v7499 = v7498;
                                v10100 = v10571;
                            } else {
                                v7499 = v7495;
                                v10100 = v17300;
                            }
                            let v7500 = v7499.sqrt();
                            let v7501 = v4 - v7500;
                            let v17305 = v17276 * v7501;
                            let v7504 = (v7413 + (v7482 * v7501)) + v7408;
                            let v7505 = v658 * v7504;
                            let v17311 = v10402 * v7504;
                            let v17314 = (Lanes([0.0, 0.0, v17311[0], 0.0, 0.0])) + (((v17285 + ((Lanes([0.0, 0.0, v17305[0], 0.0, 0.0])) + (((v10100 * (v9360 / (v10427 * v7500))) * v10382) * v7482))) + v17287) * v658);
                            let v7506 = if v7505 < v91 { 1.0 } else { 0.0 };
                            let v7584: f64;
                            let v10101: Lanes<5>;
                            if v7506 != 0.0 {
                                let v7509 = v658 * v6975;
                                let v7510 = v4 / v7509;
                                let v17320 = ((((v10402 * v6975) + (v16714 * v658)) * v7510) * v10382) / v7509;
                                let v7511 = v7508 + v7510;
                                let v17321 = v17259 * v10382;
                                let v7513 = (-v7470) / v6975;
                                let v17322 = v16714 * v7513;
                                let v17329 = ((v17320 * v7507) / v7516) * v10382;
                                let v7521 = (v7514 - ((v7507 * v7511) / v7516)) + (v7513 / v7519);
                                let v17332 = (Lanes([0.0, 0.0, v17329[0], 0.0, 0.0])) + ((((Lanes([v17321[0], v17321[1], 0.0, v17321[2], v17321[3]])) - (Lanes([0.0, 0.0, v17322[0], 0.0, 0.0]))) / v6975) / v7519);
                                let v7527 = ((v7522 * v7511) - v7524) / v7526;
                                let v17334 = (v17320 * v7522) / v7526;
                                let v17335 = v17332 * v7521;
                                let v7529 = v7527 * v7527;
                                let v17337 = v17334 * v7527;
                                let v17341 = ((v17337 + v17337) * v7527) + (v17334 * v7529);
                                let v7532 = ((v7521 * v7521) + (v7529 * v7527)).sqrt();
                                let v17346 = ((v17335 + v17335) + (Lanes([0.0, 0.0, v17341[0], 0.0, 0.0]))) * (v9360 / (v10427 * v7532));
                                let v7534 = (-v7521) + v7532;
                                let v7536 = v7521 + v7532;
                                let v7541 = ((v7534.powf(v1557)) + (-(v7536.powf(v1557)))) - v7540;
                                let v17361 = v10407 * v7541;
                                let v7544 = ((v7541 * v660) - v7408) + v7408;
                                let v7545 = v658 * v7544;
                                let v17366 = v10402 * v7544;
                                let v17369 = (Lanes([0.0, 0.0, v17366[0], 0.0, 0.0])) + (((((((((v17332 * v10382) + v17346) * (v1557 * (v7534.powf(v17349)))) + (((v17332 + v17346) * (v1557 * (v7536.powf(v17354)))) * v10382)) * v660) + (Lanes([0.0, 0.0, v17361[0], 0.0, 0.0]))) - v17287) + v17287) * v658);
                                v7584 = v7545;
                                v10101 = v17369;
                            } else {
                                v7584 = v7505;
                                v10101 = v17314;
                            }
                            let v7546 = if v7118 > v0 { 1.0 } else { 0.0 };
                            let v7600: f64;
                            let v10102: Lanes<5>;
                            if v7546 != 0.0 {
                                let v7547 = v7470 + v74;
                                let v17370 = v10402 * v7414;
                                let v17371 = v17257 * v658;
                                let v7549 = (v658 * v7414).exp();
                                let v7550 = v7549 + v358;
                                let v7551 = v726 / v34;
                                let v7552 = v7551 * v7551;
                                let v17377 = (v10445 / v34) * v7551;
                                let v17378 = v17377 + v17377;
                                let v7553 = v7552 * v7550;
                                let v17379 = v17378 * v7550;
                                let v7554 = v658 * v7547;
                                let v17383 = v10402 * v7547;
                                let v17385 = (Lanes([0.0, 0.0, v17383[0], 0.0, 0.0])) + v17263;
                                let v7555 = v7553 * v7475;
                                let v17387 = v17268 * v7553;
                                let v17389 = (((Lanes([0.0, 0.0, v17379[0], 0.0])) + ((((Lanes([0.0, 0.0, v17370[0], 0.0])) + (Lanes([v17371[0], v17371[1], 0.0, v17371[2]]))) * v7549) * v7552)) * v7475) + (Lanes([0.0, 0.0, v17387[0], 0.0]));
                                let v17390 = v17385 * v7554;
                                let v7557 = v7555 + (v7554 * v7554);
                                let v17392 = Lanes([v17389[0], v17389[1], v17389[2], 0.0, v17389[3]]);
                                let v7559 = v7552 * v7475;
                                let v7560 = v7559.ln();
                                let v17400 = ((v17378 * v7475) + (v17268 * v7552)) * (v9360 / v7559);
                                let v17401 = Lanes([0.0, 0.0, v17400[0], 0.0, 0.0]);
                                let v7562 = v658 * v7408;
                                let v17403 = v10402 * v7408;
                                let v17404 = v17255 * v658;
                                let v17407 = (Lanes([0.0, 0.0, v17403[0], 0.0])) + (Lanes([v17404[0], v17404[1], 0.0, v17404[2]]));
                                let v17408 = Lanes([v17407[0], v17407[1], v17407[2], 0.0, v17407[3]]);
                                let v17410 = v17385 - ((((v17392 + (v17390 + v17390)) * (v9360 / v7557)) - v17401) + v17408);
                                let v7565 = (v7554 - (((v7557.ln()) - v7560) + v7562)) - v4;
                                let v7566 = v85 * v7554;
                                let v17411 = v17385 * v85;
                                let v7567 = if v7566 > v0 { 1.0 } else { 0.0 };
                                let v7569: f64;
                                let v10103: Lanes<5>;
                                if v7567 != 0.0 {
                                    v7569 = v7566;
                                    v10103 = v17411;
                                } else {
                                    let v7568 = -v7566;
                                    let v17412 = v17411 * v10382;
                                    v7569 = v7568;
                                    v10103 = v17412;
                                }
                                let v17413 = v17410 * v7565;
                                let v7572 = ((v7565 * v7565) + v7569).sqrt();
                                let v17423 = v10402 * v74;
                                let v7578 = (v7554 - (v7554 - (v8 * (v7565 + v7572)))) + (v658 * v74);
                                let v17426 = ((v17385 - (v17385 - ((v17410 + (((v17413 + v17413) + v10103) * (v9360 / (v10427 * v7572)))) * v8))) + (Lanes([0.0, 0.0, v17423[0], 0.0, 0.0]))) * v7578;
                                let v7580 = v7555 + (v7578 * v7578);
                                let v7583 = ((v7580.ln()) - v7560) + v7562;
                                let v17432 = (((v17392 + (v17426 + v17426)) * (v9360 / v7580)) - v17401) + v17408;
                                let v17433 = v17432 - v10101;
                                let v7587 = (v7583 - v7584) - v7586;
                                let v7590 = (v85 * v7583) * v7589;
                                let v17435 = (v17432 * v85) * v7589;
                                let v7591 = if v7590 > v0 { 1.0 } else { 0.0 };
                                let v7593: f64;
                                let v10104: Lanes<5>;
                                if v7591 != 0.0 {
                                    v7593 = v7590;
                                    v10104 = v17435;
                                } else {
                                    let v7592 = -v7590;
                                    let v17436 = v17435 * v10382;
                                    v7593 = v7592;
                                    v10104 = v17436;
                                }
                                let v17437 = v17433 * v7587;
                                let v7596 = ((v7587 * v7587) + v7593).sqrt();
                                let v7599 = v7583 - (v8 * (v7587 + v7596));
                                let v17445 = v17432 - ((v17433 + (((v17437 + v17437) + v10104) * (v9360 / (v10427 * v7596)))) * v8);
                                v7600 = v7599;
                                v10102 = v17445;
                            } else {
                                v7600 = v7584;
                                v10102 = v10101;
                            }
                            let v7601 = v7600 / v658;
                            let v17446 = v10402 * v7601;
                            let v7602 = v7601 - v7408;
                            let v17450 = ((v10102 - (Lanes([0.0, 0.0, v17446[0], 0.0, 0.0]))) / v658) - v17287;
                            let v7605 = (-v7600).exp();
                            let v7606 = (v7600 - v4) + v7605;
                            let v17453 = v10102 + ((v10102 * v10382) * v7605);
                            let v7608 = if v7606 < v7607 { 1.0 } else { 0.0 };
                            let v7610: f64;
                            let v10105: Lanes<5>;
                            if v7608 != 0.0 {
                                v7610 = v7609;
                                v10105 = v10571;
                            } else {
                                v7610 = v7606;
                                v10105 = v17453;
                            }
                            let v7611 = v7610.sqrt();
                            let v7612 = v6916 * v7611;
                            let v17457 = v16662 * v7611;
                            let v17460 = (Lanes([0.0, 0.0, v17457[0], 0.0, 0.0])) + ((v10105 * (v9360 / (v10427 * v7611))) * v6916);
                            let v7614 = v122 * (v7413 - v7602);
                            let v17462 = (v17285 - v17450) * v122;
                            let v7615 = if v7118 == v4 { 1.0 } else { 0.0 };
                            let v7762: f64;
                            let v7764: f64;
                            let v10106: Lanes<5>;
                            let v10107: Lanes<5>;
                            if v7615 != 0.0 {
                                let v17463 = v10402 * v7414;
                                let v17464 = v17257 * v658;
                                let v7617 = (v658 * v7414).exp();
                                let v17468 = ((Lanes([0.0, 0.0, v17463[0], 0.0])) + (Lanes([v17464[0], v17464[1], 0.0, v17464[2]]))) * v7617;
                                let v7618 = v726 / v34;
                                let v7619 = v7618 * v7618;
                                let v17470 = (v10445 / v34) * v7618;
                                let v17471 = v17470 + v17470;
                                let v7620 = v7619 * v7617;
                                let v17472 = v17471 * v7617;
                                let v17475 = (Lanes([0.0, 0.0, v17472[0], 0.0])) + (v17468 * v7619);
                                let mut v7621: f64 = 0.0;
                                let mut v7624: f64 = 0.0;
                                let mut v7710: f64 = 0.0;
                                let mut v7740: f64 = 0.0;
                                let mut v7743: f64 = 0.0;
                                let mut v7753: f64 = 0.0;
                                let mut v7756: f64 = 0.0;
                                let mut v10108: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10109: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10110: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10111: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10112: Lanes<5> = Lanes([0.0; 5]);
                                v7621 = v4;
                                v7624 = v7602;
                                v7710 = v0;
                                v7740 = v7600;
                                v7743 = v7744;
                                v7753 = v0;
                                v7756 = v0;
                                v10108 = v17450;
                                v10109 = v10102;
                                v10110 = v10060;
                                v10111 = v10571;
                                v10112 = v10571;
                                loop {
                                    let v7623 = if v7621 <= v7622 { 1.0 } else { 0.0 };
                                    if v7623 == 0.0 {
                                        break;
                                    }
                                    let v7625 = v7624 + v7408;
                                    let v7626 = v658 * v7625;
                                    let v17496 = v10402 * v7625;
                                    let v17499 = (Lanes([0.0, 0.0, v17496[0], 0.0, 0.0])) + ((v10108 + v17287) * v658);
                                    let v7627 = if v7626 < v639 { 1.0 } else { 0.0 };
                                    let v7703: f64;
                                    let v7707: f64;
                                    let v7746: f64;
                                    let v7757: f64;
                                    let v10113: Lanes<5>;
                                    let v10114: Lanes<5>;
                                    let v10115: Lanes<5>;
                                    let v10116: Lanes<5>;
                                    if v7627 != 0.0 {
                                        let v7628 = v7626 * v7626;
                                        let v17541 = v17499 * v7626;
                                        let v17542 = v17541 + v17541;
                                        let v7629 = v7628 * v7626;
                                        let v7632 = v7630 + (v7626 * v6313);
                                        let v7634 = v6311 + (v7626 * v7632);
                                        let v7635 = v7629 * v7634;
                                        let v17552 = (((v17542 * v7626) + (v17499 * v7628)) * v7634) + (((v17499 * v7632) + ((v17499 * v6313) * v7626)) * v7629);
                                        let v7638 = v7626 * v639;
                                        let v17553 = v17499 * v639;
                                        let v7640 = v7637 + (v7638 * v6313);
                                        let v7642 = v7636 + (v7626 * v7640);
                                        let v7643 = v7628 * v7642;
                                        let v7644 = v7620 * v7635;
                                        let v17561 = v17475 * v7635;
                                        let v7645 = v7644 * v7635;
                                        let v17567 = (((Lanes([v17561[0], v17561[1], v17561[2], 0.0, v17561[3]])) + (v17552 * v7620)) * v7635) + (v17552 * v7644);
                                        let v17569 = v10402 * v7620;
                                        let v7647 = (v7620 * v658) * v73;
                                        let v7648 = v7647 * v7635;
                                        let v17573 = (((v17475 * v658) + (Lanes([0.0, 0.0, v17569[0], 0.0]))) * v73) * v7635;
                                        let v7653 = v7651 + (v7626 * v6337);
                                        let v7655 = v6335 + (v7626 * v7653);
                                        let v7657 = v7650 + (v7626 * v7655);
                                        let v7659 = v6333 + (v7626 * v7657);
                                        let v7660 = v7626 * v7659;
                                        let v17592 = (v17499 * v7659) + (((v17499 * v7657) + (((v17499 * v7655) + (((v17499 * v7653) + ((v17499 * v6337) * v7626)) * v7626)) * v7626)) * v7626);
                                        let v7665 = v7663 + (v7638 * v6337);
                                        let v7667 = v7662 + (v7626 * v7665);
                                        let v7669 = v7661 + (v7626 * v7667);
                                        let v7671 = v6333 + (v7626 * v7669);
                                        let v17603 = v17592 * v7660;
                                        let v7675 = (((v7660 * v7660) + v7645) + v358).sqrt();
                                        let v17608 = ((v17603 + v17603) + v17567) * (v9360 / (v10427 * v7675));
                                        let v17609 = v10402 * v7671;
                                        let v7677 = (v658 * v7671) * v73;
                                        let v7680 = v7675 + v7675;
                                        let v7681 = ((v7677 * v7660) + (v7648 * v7643)) / v7680;
                                        let v17621 = (((((((Lanes([0.0, 0.0, v17609[0], 0.0, 0.0])) + (((v17499 * v7669) + (((v17499 * v7667) + (((v17499 * v7665) + ((v17553 * v6337) * v7626)) * v7626)) * v7626)) * v658)) * v73) * v7660) + (v17592 * v7677)) + ((((Lanes([v17573[0], v17573[1], v17573[2], 0.0, v17573[3]])) + (v17552 * v7647)) * v7643) + (((v17542 * v7642) + (((v17499 * v7640) + ((v17553 * v6313) * v7626)) * v7628)) * v7648))) - ((v17608 + v17608) * v7681)) / v7680;
                                        v7703 = v7675;
                                        v7707 = v7681;
                                        v7746 = v7660;
                                        v7757 = v7645;
                                        v10113 = v17608;
                                        v10114 = v17621;
                                        v10115 = v17592;
                                        v10116 = v17567;
                                    } else {
                                        let v7682 = if v7626 < v2530 { 1.0 } else { 0.0 };
                                        let v7695: f64;
                                        let v7698: f64;
                                        let v10117: Lanes<5>;
                                        let v10118: Lanes<5>;
                                        if v7682 != 0.0 {
                                            let v7683 = v7626.exp();
                                            let v17518 = v17499 * v7683;
                                            let v7684 = v7683 - v4;
                                            let v7685 = v7620 * v7684;
                                            let v17519 = v17475 * v7684;
                                            let v17522 = (Lanes([v17519[0], v17519[1], v17519[2], 0.0, v17519[3]])) + (v17518 * v7620);
                                            let v7686 = v7620 * v658;
                                            let v17524 = v10402 * v7620;
                                            let v7687 = v7686 * v7683;
                                            let v17527 = ((v17475 * v658) + (Lanes([0.0, 0.0, v17524[0], 0.0]))) * v7683;
                                            let v17530 = (Lanes([v17527[0], v17527[1], v17527[2], 0.0, v17527[3]])) + (v17518 * v7686);
                                            v7695 = v7685;
                                            v7698 = v7687;
                                            v10117 = v17522;
                                            v10118 = v17530;
                                        } else {
                                            let v17500 = v10402 * v7624;
                                            let v7689 = (v658 * v7624).exp();
                                            let v17504 = ((Lanes([0.0, 0.0, v17500[0], 0.0, 0.0])) + (v10108 * v658)) * v7689;
                                            let v7690 = v7689 - v7617;
                                            let v7691 = v7619 * v7690;
                                            let v17507 = v17471 * v7690;
                                            let v17510 = (Lanes([0.0, 0.0, v17507[0], 0.0, 0.0])) + ((v17504 - (Lanes([v17468[0], v17468[1], v17468[2], 0.0, v17468[3]]))) * v7619);
                                            let v7692 = v7619 * v658;
                                            let v7693 = v7692 * v7689;
                                            let v17514 = ((v17471 * v658) + (v10402 * v7619)) * v7689;
                                            let v17517 = (Lanes([0.0, 0.0, v17514[0], 0.0, 0.0])) + (v17504 * v7692);
                                            v7695 = v7691;
                                            v7698 = v7693;
                                            v10117 = v17510;
                                            v10118 = v17517;
                                        }
                                        let v7697 = ((v7626 - v4) + v7695).sqrt();
                                        let v17534 = (v17499 + v10117) * (v9360 / (v10427 * v7697));
                                        let v7700 = (v658 + v7698) / v7697;
                                        let v7701 = v7700 * v8;
                                        let v17540 = ((((Lanes([0.0, 0.0, v10402[0], 0.0, 0.0])) + v10118) - (v17534 * v7700)) / v7697) * v8;
                                        v7703 = v7697;
                                        v7707 = v7701;
                                        v7746 = v0;
                                        v7757 = v7695;
                                        v10113 = v17534;
                                        v10114 = v17540;
                                        v10115 = v10571;
                                        v10116 = v10117;
                                    }
                                    let v17623 = v16714 * v7703;
                                    let v7705 = (v7413 - v7624) - (v6975 * v7703);
                                    let v17627 = (v17285 - v10108) - ((Lanes([0.0, 0.0, v17623[0], 0.0, 0.0])) + (v10113 * v6975));
                                    let v17628 = v16714 * v7707;
                                    let v7709 = v7706 - (v6975 * v7707);
                                    let v17632 = ((Lanes([0.0, 0.0, v17628[0], 0.0, 0.0])) + (v10114 * v6975)) * v10382;
                                    let v7711 = if v7710 == v4 { 1.0 } else { 0.0 };
                                    let v7734: f64;
                                    let v7736: f64;
                                    let v7737: f64;
                                    let v10119: Lanes<5>;
                                    if v7711 != 0.0 {
                                        v7734 = v7712;
                                        v7736 = v7624;
                                        v7737 = v7710;
                                        v10119 = v10108;
                                    } else {
                                        let v7714 = (-v7705) / v7709;
                                        let v17636 = ((v17627 * v10382) - (v17632 * v7714)) / v7709;
                                        let v7716 = v7624.abs();
                                        let v17640 = v10108 * ((v10427 * (if v7624 >= v11296 { 1.0 } else { 0.0 })) - v9360);
                                        let v7717 = if v4 >= v7716 { 1.0 } else { 0.0 };
                                        let v7718: f64;
                                        let v10120: Lanes<5>;
                                        if v7717 != 0.0 {
                                            v7718 = v4;
                                            v10120 = v10571;
                                        } else {
                                            v7718 = v7716;
                                            v10120 = v17640;
                                        }
                                        let v7720 = v7715 * (v4 + v7718);
                                        let v17641 = v10120 * v7715;
                                        let v7722 = if (v7714.abs()) > v7720 { 1.0 } else { 0.0 };
                                        let v7727: f64;
                                        let v10121: Lanes<5>;
                                        if v7722 != 0.0 {
                                            let v7723 = if v7714 >= v0 { 1.0 } else { 0.0 };
                                            let v7725: f64;
                                            if v7723 != 0.0 {
                                                v7725 = v4;
                                            } else {
                                                v7725 = v7724;
                                            }
                                            let v7726 = v7720 * v7725;
                                            let v17642 = v17641 * v7725;
                                            v7727 = v7726;
                                            v10121 = v17642;
                                        } else {
                                            v7727 = v7714;
                                            v10121 = v17636;
                                        }
                                        let v7728 = v7624 + v7727;
                                        let v17643 = v10108 + v10121;
                                        let v7733 = if (if (v7727.abs()) <= v856 { 1.0 } else { 0.0 }) != 0.0 && (if (v7705.abs()) <= v3501 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v7738: f64;
                                        if v7733 != 0.0 {
                                            v7738 = v4;
                                        } else {
                                            v7738 = v7710;
                                        }
                                        v7734 = v7621;
                                        v7736 = v7728;
                                        v7737 = v7738;
                                        v10119 = v17643;
                                    }
                                    let v7735 = v7734 + v4;
                                    v7621 = v7735;
                                    v7624 = v7736;
                                    v7710 = v7737;
                                    v7740 = v7626;
                                    v7743 = v7746;
                                    v7753 = v7703;
                                    v7756 = v7757;
                                    v10108 = v10119;
                                    v10109 = v17499;
                                    v10110 = v10115;
                                    v10111 = v10113;
                                    v10112 = v10116;
                                }
                                let v7739 = if v7710 == v0 { 1.0 } else { 0.0 };
                                if v7739 != 0.0 {
                                } else {
                                }
                                let v7741 = if v7740 < v639 { 1.0 } else { 0.0 };
                                let v7751: f64;
                                let v10122: Lanes<5>;
                                if v7741 != 0.0 {
                                    let v7742 = if v7740 < v91 { 1.0 } else { 0.0 };
                                    if v7742 != 0.0 {
                                    } else {
                                    }
                                    let v7748 = v7743 + v7747;
                                    v7751 = v7748;
                                    v10122 = v10110;
                                } else {
                                    let v7750 = (v7740 - v4).sqrt();
                                    let v17478 = v10109 * (v9360 / (v10427 * v7750));
                                    v7751 = v7750;
                                    v10122 = v17478;
                                }
                                let v7752 = v6916 * v7751;
                                let v17479 = v16662 * v7751;
                                let v17482 = (Lanes([0.0, 0.0, v17479[0], 0.0, 0.0])) + (v10122 * v6916);
                                let v7754 = v7753 + v7751;
                                let v7755 = v4 / v7754;
                                let v7758 = v6916 * v7756;
                                let v17487 = v16662 * v7756;
                                let v7760 = v7752 + (v7758 * v7755);
                                let v17494 = v17482 + ((((Lanes([0.0, 0.0, v17487[0], 0.0, 0.0])) + (v10112 * v6916)) * v7755) + (((((v10111 + v10122) * v7755) * v10382) / v7754) * v7758));
                                v7762 = v7760;
                                v7764 = v7752;
                                v10106 = v17494;
                                v10107 = v17482;
                            } else {
                                v7762 = v7614;
                                v7764 = v7612;
                                v10106 = v17462;
                                v10107 = v17460;
                            }
                            v7761 = v7762;
                            v7763 = v7764;
                            v10096 = v10106;
                            v10097 = v10107;
                        }
                        let v7767: f64;
                        if v5 != 0.0 {
                            let v7765 = v6894 * v6887;
                            v7767 = v7765;
                        } else {
                            let v7766 = v164 * v6887;
                            v7767 = v7766;
                        }
                        let v7771 = if (if v7768 != 0.0 && v148 != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v7351 != 0.0 && v5 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v8621: f64;
                        let v8650: f64;
                        let v10123: Lanes<5>;
                        let v10124: Lanes<5>;
                        if v7771 != 0.0 {
                            let v7772 = v7767 * v7761;
                            let v17728 = v10096 * v7767;
                            let v7773 = v7767 * v7763;
                            let v17729 = v10097 * v7767;
                            v8621 = v7772;
                            v8650 = v7773;
                            v10123 = v17728;
                            v10124 = v17729;
                        } else {
                            v8621 = v8622;
                            v8650 = v8651;
                            v10123 = v10087;
                            v10124 = v10088;
                        }
                        let v7777 = if (if v7774 != 0.0 && v148 != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v7352 != 0.0 && v5 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v8627: f64;
                        let v8639: f64;
                        let v10125: Lanes<5>;
                        let v10126: Lanes<5>;
                        if v7777 != 0.0 {
                            let v7778 = v7767 * v7761;
                            let v17730 = v10096 * v7767;
                            let v7779 = v7767 * v7763;
                            let v17731 = v10097 * v7767;
                            v8627 = v7778;
                            v8639 = v7779;
                            v10125 = v17730;
                            v10126 = v17731;
                        } else {
                            v8627 = v8628;
                            v8639 = v8640;
                            v10125 = v10089;
                            v10126 = v10090;
                        }
                        v7796 = v0;
                        v7815 = v0;
                        v8620 = v8621;
                        v8626 = v8627;
                        v8638 = v8639;
                        v8649 = v8650;
                        v10047 = v11054;
                        v10048 = v11054;
                        v10049 = v10123;
                        v10050 = v10125;
                        v10051 = v10126;
                        v10052 = v10124;
                    }
                    let v7782 = (v6043 * v366) + (v6041 * v365);
                    let v8451: f64;
                    let v10127: Lanes<6>;
                    if v7782 != 0.0 {
                        let v7787 = (v6043 * v7783) + (v6041 * v7785);
                        let v7797: f64;
                        if v5 != 0.0 {
                            let v7793 = v7787 * (-((v6043 * v6894) + (v6041 * v7789)));
                            v7797 = v7793;
                        } else {
                            let v7795 = v7787 * (-v164);
                            v7797 = v7795;
                        }
                        let v7798 = -v7797;
                        let v17756 = (v9404 - (Lanes([v9402[0], v9402[1], 0.0]))) * v7798;
                        let v7801 = v7796 + (v7798 * (v825 - v818));
                        let v17758 = v10047 + (Lanes([v17756[0], v17756[1], 0.0, v17756[2], 0.0, 0.0]));
                        v8451 = v7801;
                        v10127 = v17758;
                    } else {
                        v8451 = v7796;
                        v10127 = v10047;
                    }
                    let v7804 = (v6041 * v366) + (v6043 * v365);
                    let v8455: f64;
                    let v10128: Lanes<6>;
                    if v7804 != 0.0 {
                        let v7807 = (v6041 * v7783) + (v6043 * v7785);
                        let v7816: f64;
                        if v5 != 0.0 {
                            let v7812 = v7807 * (-((v6041 * v6894) + (v6043 * v7789)));
                            v7816 = v7812;
                        } else {
                            let v7814 = v7807 * (-v164);
                            v7816 = v7814;
                        }
                        let v7817 = -v7816;
                        let v17759 = v9404 * v7817;
                        let v7819 = v7815 + (v7817 * v825);
                        let v17761 = v10048 + (Lanes([v17759[0], v17759[1], 0.0, v17759[2], 0.0, 0.0]));
                        v8455 = v7819;
                        v10128 = v17761;
                    } else {
                        v8455 = v7815;
                        v10128 = v10048;
                    }
                    v8450 = v8451;
                    v8454 = v8455;
                    v8619 = v8620;
                    v8625 = v8626;
                    v8637 = v8638;
                    v8648 = v8649;
                    v10041 = v10127;
                    v10042 = v10128;
                    v10043 = v10049;
                    v10044 = v10050;
                    v10045 = v10051;
                    v10046 = v10052;
                } else {
                    let v7821 = if v7820 == v4 { 1.0 } else { 0.0 };
                    let v7822 = if v365 == 0.0 { 1.0 } else { 0.0 };
                    let v7824 = if v7820 != v4 { 1.0 } else { 0.0 };
                    let v7825 = if v366 == 0.0 { 1.0 } else { 0.0 };
                    let v7827 = if (if v7821 != 0.0 && v7822 != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v7824 != 0.0 && v7825 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v7844: f64;
                    if v7827 != 0.0 {
                        let v7845: f64;
                        if v5 != 0.0 {
                            let v7830 = ((-v122) * v6887) * v7789;
                            v7845 = v7830;
                        } else {
                            let v7833 = ((-v122) * v6887) * v164;
                            v7845 = v7833;
                        }
                        v7844 = v7845;
                    } else {
                        let v7836 = (v6043 * v7783) + (v6041 * v7785);
                        let v7846: f64;
                        if v5 != 0.0 {
                            let v7841 = v7836 * (-((v6043 * v6894) + (v6041 * v7789)));
                            v7846 = v7841;
                        } else {
                            let v7843 = v7836 * (-v164);
                            v7846 = v7843;
                        }
                        v7844 = v7846;
                    }
                    let v7847 = -v7844;
                    let v7849 = v7847 * (v825 - v818);
                    let v16658 = (v9404 - (Lanes([v9402[0], v9402[1], 0.0]))) * v7847;
                    let v7852 = if (if v7821 != 0.0 && v7825 != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v7824 != 0.0 && v7822 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v7869: f64;
                    if v7852 != 0.0 {
                        let v7870: f64;
                        if v5 != 0.0 {
                            let v7855 = ((-v122) * v6887) * v6894;
                            v7870 = v7855;
                        } else {
                            let v7858 = ((-v122) * v6887) * v164;
                            v7870 = v7858;
                        }
                        v7869 = v7870;
                    } else {
                        let v7861 = (v6041 * v7783) + (v6043 * v7785);
                        let v7871: f64;
                        if v5 != 0.0 {
                            let v7866 = v7861 * (-((v6041 * v6894) + (v6043 * v7789)));
                            v7871 = v7866;
                        } else {
                            let v7868 = v7861 * (-v164);
                            v7871 = v7868;
                        }
                        v7869 = v7871;
                    }
                    let v7872 = -v7869;
                    let v7873 = v7872 * v825;
                    let v16659 = v9404 * v7872;
                    let v16660 = Lanes([v16658[0], v16658[1], 0.0, v16658[2], 0.0, 0.0]);
                    let v16661 = Lanes([v16659[0], v16659[1], 0.0, v16659[2], 0.0, 0.0]);
                    v8450 = v7849;
                    v8454 = v7873;
                    v8619 = v0;
                    v8625 = v0;
                    v8637 = v0;
                    v8648 = v0;
                    v10041 = v16660;
                    v10042 = v16661;
                    v10043 = v10571;
                    v10044 = v10571;
                    v10045 = v10571;
                    v10046 = v10571;
                }
                v8449 = v8450;
                v8453 = v8454;
                v8618 = v8619;
                v8624 = v8625;
                v8636 = v8637;
                v8647 = v8648;
                v10035 = v10041;
                v10036 = v10042;
                v10037 = v10043;
                v10038 = v10044;
                v10039 = v10045;
                v10040 = v10046;
            } else {
                v8449 = v0;
                v8453 = v0;
                v8618 = v0;
                v8624 = v0;
                v8636 = v0;
                v8647 = v0;
                v10035 = v11054;
                v10036 = v11054;
                v10037 = v10571;
                v10038 = v10571;
                v10039 = v10571;
                v10040 = v10571;
            }
            let v8668: f64;
            let v8669: f64;
            let v8670: f64;
            let v8672: f64;
            let v10129: Lanes<3>;
            let v10130: Lanes<3>;
            let v10131: Lanes<2>;
            let v10132: Lanes<2>;
            if v5 != 0.0 {
                let v7879 = (v116 * v205) - (v656 * v658);
                let v17767 = ((v10398 * v658) + (v10402 * v656)) * v10382;
                let v7881 = v694.ln();
                let v17769 = v10408 * (v9360 / v694);
                let v7886 = ((v7879 + (v7880 * v7881)) / v7884).exp();
                let v7887 = v7876 * v7886;
                let v17774 = (((v17767 + (v17769 * v7880)) / v7884) * v7886) * v7876;
                let v7892 = ((v7879 + (v7888 * v7881)) / v7884).exp();
                let v7893 = v7876 * v7892;
                let v17779 = (((v17767 + (v17769 * v7888)) / v7884) * v7892) * v7876;
                let v7895 = v7894 * v7;
                let v7896 = v7895 * v7887;
                let v17780 = v17774 * v7895;
                let v7897 = v7895 * v7893;
                let v17781 = v17779 * v7895;
                let v7899 = v7898 * v7;
                let v7900 = v7899 * v7887;
                let v17782 = v17774 * v7899;
                let v7901 = v7899 * v7893;
                let v17783 = v17779 * v7899;
                let v17784 = v10408 * v694;
                let v7903 = v7896 + v358;
                let v7904 = v7900 + v358;
                let v7905 = v7884 / v658;
                let v17788 = ((v10402 * v7905) * v10382) / v658;
                let v7907 = v7906 * (v694 * v694);
                let v17789 = (v17784 + v17784) * v7906;
                let v7908 = v7907 / v7903;
                let v7909 = v4 + v7908;
                let v7910 = v7909.ln();
                let v7911 = v7905 * v7910;
                let v17797 = (v17788 * v7910) + ((((v17789 - (v17780 * v7908)) / v7903) * (v9360 / v7909)) * v7905);
                let v7912 = v7907 / v7904;
                let v7913 = v4 + v7912;
                let v7914 = v7913.ln();
                let v7915 = v7905 * v7914;
                let v17805 = (v17788 * v7914) + ((((v17789 - (v17782 * v7912)) / v7904) * (v9360 / v7913)) * v7905);
                let v7916 = v7884 * v660;
                let v17806 = v10407 * v7884;
                let v7917 = if v7874 < v7911 { 1.0 } else { 0.0 };
                let v7931: f64;
                let v10133: Lanes<3>;
                if v7917 != 0.0 {
                    let v7918 = v7874 / v7916;
                    let v17829 = v17806 * v7918;
                    let v7919 = v7918.exp();
                    let v7920 = v7919 - v4;
                    let v7921 = v7896 * v7920;
                    let v17835 = v17780 * v7920;
                    let v17838 = (Lanes([0.0, v17835[0], 0.0])) + (((((Lanes([v9378[0], 0.0, v9378[1]])) - (Lanes([0.0, v17829[0], 0.0]))) / v7916) * v7919) * v7896);
                    v7931 = v7921;
                    v10133 = v17838;
                } else {
                    let v7922 = v7911 / v7916;
                    let v7923 = v7922.exp();
                    let v17810 = ((v17797 - (v17806 * v7922)) / v7916) * v7923;
                    let v7924 = v7923 - v4;
                    let v17813 = (v17780 * v7924) + (v17810 * v7896);
                    let v7926 = v7896 / v7916;
                    let v7927 = v7926 * v7923;
                    let v7928 = v7874 - v7911;
                    let v17823 = ((((v17780 - (v17806 * v7926)) / v7916) * v7923) + (v17810 * v7926)) * v7928;
                    let v7930 = (v7896 * v7924) + (v7927 * v7928);
                    let v17828 = (Lanes([0.0, v17813[0], 0.0])) + ((Lanes([0.0, v17823[0], 0.0])) + (((Lanes([v9378[0], 0.0, v9378[1]])) - (Lanes([0.0, v17797[0], 0.0]))) * v7927));
                    v7931 = v7930;
                    v10133 = v17828;
                }
                let v7933 = v7932 * v7874;
                let v17840 = (v9378 * v7932) * v7897;
                let v17841 = v17781 * v7933;
                let v7935 = v7931 + (v7933 * v7897);
                let v17845 = v10133 + ((Lanes([v17840[0], 0.0, v17840[1]])) + (Lanes([0.0, v17841[0], 0.0])));
                let v7936 = if v7875 < v7915 { 1.0 } else { 0.0 };
                let v7950: f64;
                let v10134: Lanes<3>;
                if v7936 != 0.0 {
                    let v7937 = v7875 / v7916;
                    let v17868 = v17806 * v7937;
                    let v7938 = v7937.exp();
                    let v7939 = v7938 - v4;
                    let v7940 = v7900 * v7939;
                    let v17874 = v17782 * v7939;
                    let v17877 = (Lanes([0.0, v17874[0], 0.0])) + (((((Lanes([v9379[0], 0.0, v9379[1]])) - (Lanes([0.0, v17868[0], 0.0]))) / v7916) * v7938) * v7900);
                    v7950 = v7940;
                    v10134 = v17877;
                } else {
                    let v7941 = v7915 / v7916;
                    let v7942 = v7941.exp();
                    let v17849 = ((v17805 - (v17806 * v7941)) / v7916) * v7942;
                    let v7943 = v7942 - v4;
                    let v17852 = (v17782 * v7943) + (v17849 * v7900);
                    let v7945 = v7900 / v7916;
                    let v7946 = v7945 * v7942;
                    let v7947 = v7875 - v7915;
                    let v17862 = ((((v17782 - (v17806 * v7945)) / v7916) * v7942) + (v17849 * v7945)) * v7947;
                    let v7949 = (v7900 * v7943) + (v7946 * v7947);
                    let v17867 = (Lanes([0.0, v17852[0], 0.0])) + ((Lanes([0.0, v17862[0], 0.0])) + (((Lanes([v9379[0], 0.0, v9379[1]])) - (Lanes([0.0, v17805[0], 0.0]))) * v7946));
                    v7950 = v7949;
                    v10134 = v17867;
                }
                let v7951 = v7932 * v7875;
                let v17879 = (v9379 * v7932) * v7901;
                let v17880 = v17783 * v7951;
                let v17885 = v9378 * v377;
                let v7955 = v7935 + (v377 * v7874);
                let v17887 = v17845 + (Lanes([v17885[0], 0.0, v17885[1]]));
                let v17888 = v9379 * v377;
                let v7957 = (v7950 + (v7951 * v7901)) + (v377 * v7875);
                let v17890 = (v10134 + ((Lanes([v17879[0], 0.0, v17879[1]])) + (Lanes([0.0, v17880[0], 0.0])))) + (Lanes([v17888[0], 0.0, v17888[1]]));
                let v7960 = v7958 * v7959;
                let v7962 = v7958 * v7961;
                let v7964 = v7 - v7963;
                let v7965 = if v7964 <= v0 { 1.0 } else { 0.0 };
                let v7974: f64;
                let v8094: f64;
                if v7965 != 0.0 {
                    v7974 = v0;
                    v8094 = v0;
                } else {
                    v7974 = v7962;
                    v8094 = v7960;
                }
                let v7967 = if v7966 > v6894 { 1.0 } else { 0.0 };
                let v8209: f64;
                let v10135: Lanes<2>;
                if v7967 != 0.0 {
                    let v7970 = v7968 * (v7966 - v6894);
                    let v7972 = v7971 * v6894;
                    let v7973 = if v7875 < v0 { 1.0 } else { 0.0 };
                    let v8210: f64;
                    let v10136: Lanes<2>;
                    if v7973 != 0.0 {
                        let v7975 = if v7974 > v0 { 1.0 } else { 0.0 };
                        let v8002: f64;
                        let v10137: Lanes<2>;
                        if v7975 != 0.0 {
                            let v7978 = v4 - (v7875 / v7976);
                            let v17939 = (v9379 / v7976) * v10382;
                            let v7980 = if v7979 == v8 { 1.0 } else { 0.0 };
                            let v7986: f64;
                            let v10138: Lanes<2>;
                            if v7980 != 0.0 {
                                let v7981 = v7978.sqrt();
                                let v7982 = v4 / v7981;
                                let v17949 = (((v17939 * (v9360 / (v10427 * v7981))) * v7982) * v10382) / v7981;
                                v7986 = v7982;
                                v10138 = v17949;
                            } else {
                                let v7983 = -v7979;
                                let v7984 = v7978.powf(v7983);
                                let v17943 = v17939 * (v7983 * (v7978.powf((v7983 - v9360))));
                                v7986 = v7984;
                                v10138 = v17943;
                            }
                            let v7985 = v7976 * v7974;
                            let v7990 = v4 - v7979;
                            let v7991 = (v7985 * (v4 - (v7978 * v7986))) / v7990;
                            let v17955 = ((((v17939 * v7986) + (v10138 * v7978)) * v10382) * v7985) / v7990;
                            v8002 = v7991;
                            v10137 = v17955;
                        } else {
                            v8002 = v0;
                            v10137 = v10373;
                        }
                        let v7992 = if v7970 > v0 { 1.0 } else { 0.0 };
                        let v8021: f64;
                        let v10139: Lanes<2>;
                        if v7992 != 0.0 {
                            let v7995 = v4 - (v7875 / v7993);
                            let v17957 = (v9379 / v7993) * v10382;
                            let v7997 = if v7996 == v8 { 1.0 } else { 0.0 };
                            let v8004: f64;
                            let v10140: Lanes<2>;
                            if v7997 != 0.0 {
                                let v7998 = v7995.sqrt();
                                let v7999 = v4 / v7998;
                                let v17967 = (((v17957 * (v9360 / (v10427 * v7998))) * v7999) * v10382) / v7998;
                                v8004 = v7999;
                                v10140 = v17967;
                            } else {
                                let v8000 = -v7996;
                                let v8001 = v7995.powf(v8000);
                                let v17961 = v17957 * (v8000 * (v7995.powf((v8000 - v9360))));
                                v8004 = v8001;
                                v10140 = v17961;
                            }
                            let v8003 = v7993 * v7970;
                            let v8008 = v4 - v7996;
                            let v8010 = v8002 + ((v8003 * (v4 - (v7995 * v8004))) / v8008);
                            let v17974 = v10137 + (((((v17957 * v8004) + (v10140 * v7995)) * v10382) * v8003) / v8008);
                            v8021 = v8010;
                            v10139 = v17974;
                        } else {
                            v8021 = v8002;
                            v10139 = v10137;
                        }
                        let v8011 = if v7972 > v0 { 1.0 } else { 0.0 };
                        let v8211: f64;
                        let v10141: Lanes<2>;
                        if v8011 != 0.0 {
                            let v8014 = v4 - (v7875 / v8012);
                            let v17976 = (v9379 / v8012) * v10382;
                            let v8016 = if v8015 == v8 { 1.0 } else { 0.0 };
                            let v8023: f64;
                            let v10142: Lanes<2>;
                            if v8016 != 0.0 {
                                let v8017 = v8014.sqrt();
                                let v8018 = v4 / v8017;
                                let v17986 = (((v17976 * (v9360 / (v10427 * v8017))) * v8018) * v10382) / v8017;
                                v8023 = v8018;
                                v10142 = v17986;
                            } else {
                                let v8019 = -v8015;
                                let v8020 = v8014.powf(v8019);
                                let v17980 = v17976 * (v8019 * (v8014.powf((v8019 - v9360))));
                                v8023 = v8020;
                                v10142 = v17980;
                            }
                            let v8022 = v8012 * v7972;
                            let v8027 = v4 - v8015;
                            let v8029 = v8021 + ((v8022 * (v4 - (v8014 * v8023))) / v8027);
                            let v17993 = v10139 + (((((v17976 * v8023) + (v10142 * v8014)) * v10382) * v8022) / v8027);
                            v8211 = v8029;
                            v10141 = v17993;
                        } else {
                            v8211 = v8021;
                            v10141 = v10139;
                        }
                        v8210 = v8211;
                        v10136 = v10141;
                    } else {
                        let v8039 = (((v7974 * v7979) / v7976) + ((v7970 * v7996) / v7993)) + ((v7972 * v8015) / v8012);
                        let v8042 = ((v7974 + v7970) + v7972) + ((v7875 * v8) * v8039);
                        let v8043 = v7875 * v8042;
                        let v17937 = (v9379 * v8042) + (((v9379 * v8) * v8039) * v7875);
                        v8210 = v8043;
                        v10136 = v17937;
                    }
                    v8209 = v8210;
                    v10135 = v10136;
                } else {
                    let v8044 = v7971 * v7966;
                    let v8045 = if v7875 < v0 { 1.0 } else { 0.0 };
                    let v8212: f64;
                    let v10143: Lanes<2>;
                    if v8045 != 0.0 {
                        let v8046 = if v7974 > v0 { 1.0 } else { 0.0 };
                        let v8069: f64;
                        let v10144: Lanes<2>;
                        if v8046 != 0.0 {
                            let v8048 = v4 - (v7875 / v7976);
                            let v17897 = (v9379 / v7976) * v10382;
                            let v8049 = if v7979 == v8 { 1.0 } else { 0.0 };
                            let v8055: f64;
                            let v10145: Lanes<2>;
                            if v8049 != 0.0 {
                                let v8050 = v8048.sqrt();
                                let v8051 = v4 / v8050;
                                let v17907 = (((v17897 * (v9360 / (v10427 * v8050))) * v8051) * v10382) / v8050;
                                v8055 = v8051;
                                v10145 = v17907;
                            } else {
                                let v8052 = -v7979;
                                let v8053 = v8048.powf(v8052);
                                let v17901 = v17897 * (v8052 * (v8048.powf((v8052 - v9360))));
                                v8055 = v8053;
                                v10145 = v17901;
                            }
                            let v8054 = v7976 * v7974;
                            let v8059 = v4 - v7979;
                            let v8060 = (v8054 * (v4 - (v8048 * v8055))) / v8059;
                            let v17913 = ((((v17897 * v8055) + (v10145 * v8048)) * v10382) * v8054) / v8059;
                            v8069 = v8060;
                            v10144 = v17913;
                        } else {
                            v8069 = v0;
                            v10144 = v10373;
                        }
                        let v8061 = if v8044 > v0 { 1.0 } else { 0.0 };
                        let v8213: f64;
                        let v10146: Lanes<2>;
                        if v8061 != 0.0 {
                            let v8063 = v4 - (v7875 / v8012);
                            let v17915 = (v9379 / v8012) * v10382;
                            let v8064 = if v8015 == v8 { 1.0 } else { 0.0 };
                            let v8071: f64;
                            let v10147: Lanes<2>;
                            if v8064 != 0.0 {
                                let v8065 = v8063.sqrt();
                                let v8066 = v4 / v8065;
                                let v17925 = (((v17915 * (v9360 / (v10427 * v8065))) * v8066) * v10382) / v8065;
                                v8071 = v8066;
                                v10147 = v17925;
                            } else {
                                let v8067 = -v8015;
                                let v8068 = v8063.powf(v8067);
                                let v17919 = v17915 * (v8067 * (v8063.powf((v8067 - v9360))));
                                v8071 = v8068;
                                v10147 = v17919;
                            }
                            let v8070 = v8012 * v8044;
                            let v8075 = v4 - v8015;
                            let v8077 = v8069 + ((v8070 * (v4 - (v8063 * v8071))) / v8075);
                            let v17932 = v10144 + (((((v17915 * v8071) + (v10147 * v8063)) * v10382) * v8070) / v8075);
                            v8213 = v8077;
                            v10146 = v17932;
                        } else {
                            v8213 = v8069;
                            v10146 = v10144;
                        }
                        v8212 = v8213;
                        v10143 = v10146;
                    } else {
                        let v8083 = ((v7974 * v7979) / v7976) + ((v8044 * v8015) / v8012);
                        let v8086 = (v7974 + v8044) + ((v7875 * v8) * v8083);
                        let v8087 = v7875 * v8086;
                        let v17895 = (v9379 * v8086) + (((v9379 * v8) * v8083) * v7875);
                        v8212 = v8087;
                        v10143 = v17895;
                    }
                    v8209 = v8212;
                    v10135 = v10143;
                }
                let v8089 = if v8088 > v7789 { 1.0 } else { 0.0 };
                let v8237: f64;
                let v10148: Lanes<2>;
                if v8089 != 0.0 {
                    let v8091 = v7968 * (v8088 - v7789);
                    let v8092 = v7971 * v7789;
                    let v8093 = if v7874 < v0 { 1.0 } else { 0.0 };
                    let v8238: f64;
                    let v10149: Lanes<2>;
                    if v8093 != 0.0 {
                        let v8095 = if v8094 > v0 { 1.0 } else { 0.0 };
                        let v8118: f64;
                        let v10150: Lanes<2>;
                        if v8095 != 0.0 {
                            let v8097 = v4 - (v7874 / v7976);
                            let v18042 = (v9378 / v7976) * v10382;
                            let v8098 = if v7979 == v8 { 1.0 } else { 0.0 };
                            let v8104: f64;
                            let v10151: Lanes<2>;
                            if v8098 != 0.0 {
                                let v8099 = v8097.sqrt();
                                let v8100 = v4 / v8099;
                                let v18052 = (((v18042 * (v9360 / (v10427 * v8099))) * v8100) * v10382) / v8099;
                                v8104 = v8100;
                                v10151 = v18052;
                            } else {
                                let v8101 = -v7979;
                                let v8102 = v8097.powf(v8101);
                                let v18046 = v18042 * (v8101 * (v8097.powf((v8101 - v9360))));
                                v8104 = v8102;
                                v10151 = v18046;
                            }
                            let v8103 = v7976 * v8094;
                            let v8108 = v4 - v7979;
                            let v8109 = (v8103 * (v4 - (v8097 * v8104))) / v8108;
                            let v18058 = ((((v18042 * v8104) + (v10151 * v8097)) * v10382) * v8103) / v8108;
                            v8118 = v8109;
                            v10150 = v18058;
                        } else {
                            v8118 = v0;
                            v10150 = v10372;
                        }
                        let v8110 = if v8091 > v0 { 1.0 } else { 0.0 };
                        let v8135: f64;
                        let v10152: Lanes<2>;
                        if v8110 != 0.0 {
                            let v8112 = v4 - (v7874 / v7993);
                            let v18060 = (v9378 / v7993) * v10382;
                            let v8113 = if v7996 == v8 { 1.0 } else { 0.0 };
                            let v8120: f64;
                            let v10153: Lanes<2>;
                            if v8113 != 0.0 {
                                let v8114 = v8112.sqrt();
                                let v8115 = v4 / v8114;
                                let v18070 = (((v18060 * (v9360 / (v10427 * v8114))) * v8115) * v10382) / v8114;
                                v8120 = v8115;
                                v10153 = v18070;
                            } else {
                                let v8116 = -v7996;
                                let v8117 = v8112.powf(v8116);
                                let v18064 = v18060 * (v8116 * (v8112.powf((v8116 - v9360))));
                                v8120 = v8117;
                                v10153 = v18064;
                            }
                            let v8119 = v7993 * v8091;
                            let v8124 = v4 - v7996;
                            let v8126 = v8118 + ((v8119 * (v4 - (v8112 * v8120))) / v8124);
                            let v18077 = v10150 + (((((v18060 * v8120) + (v10153 * v8112)) * v10382) * v8119) / v8124);
                            v8135 = v8126;
                            v10152 = v18077;
                        } else {
                            v8135 = v8118;
                            v10152 = v10150;
                        }
                        let v8127 = if v8092 > v0 { 1.0 } else { 0.0 };
                        let v8239: f64;
                        let v10154: Lanes<2>;
                        if v8127 != 0.0 {
                            let v8129 = v4 - (v7874 / v8012);
                            let v18079 = (v9378 / v8012) * v10382;
                            let v8130 = if v8015 == v8 { 1.0 } else { 0.0 };
                            let v8137: f64;
                            let v10155: Lanes<2>;
                            if v8130 != 0.0 {
                                let v8131 = v8129.sqrt();
                                let v8132 = v4 / v8131;
                                let v18089 = (((v18079 * (v9360 / (v10427 * v8131))) * v8132) * v10382) / v8131;
                                v8137 = v8132;
                                v10155 = v18089;
                            } else {
                                let v8133 = -v8015;
                                let v8134 = v8129.powf(v8133);
                                let v18083 = v18079 * (v8133 * (v8129.powf((v8133 - v9360))));
                                v8137 = v8134;
                                v10155 = v18083;
                            }
                            let v8136 = v8012 * v8092;
                            let v8141 = v4 - v8015;
                            let v8143 = v8135 + ((v8136 * (v4 - (v8129 * v8137))) / v8141);
                            let v18096 = v10152 + (((((v18079 * v8137) + (v10155 * v8129)) * v10382) * v8136) / v8141);
                            v8239 = v8143;
                            v10154 = v18096;
                        } else {
                            v8239 = v8135;
                            v10154 = v10152;
                        }
                        v8238 = v8239;
                        v10149 = v10154;
                    } else {
                        let v8153 = (((v8094 * v7979) / v7976) + ((v8091 * v7996) / v7993)) + ((v8092 * v8015) / v8012);
                        let v8156 = ((v8094 + v8091) + v8092) + ((v7874 * v8) * v8153);
                        let v8157 = v7874 * v8156;
                        let v18040 = (v9378 * v8156) + (((v9378 * v8) * v8153) * v7874);
                        v8238 = v8157;
                        v10149 = v18040;
                    }
                    v8237 = v8238;
                    v10148 = v10149;
                } else {
                    let v8158 = v7971 * v8088;
                    let v8159 = if v7874 < v0 { 1.0 } else { 0.0 };
                    let v8240: f64;
                    let v10156: Lanes<2>;
                    if v8159 != 0.0 {
                        let v8160 = if v8094 > v0 { 1.0 } else { 0.0 };
                        let v8183: f64;
                        let v10157: Lanes<2>;
                        if v8160 != 0.0 {
                            let v8162 = v4 - (v7874 / v7976);
                            let v18000 = (v9378 / v7976) * v10382;
                            let v8163 = if v7979 == v8 { 1.0 } else { 0.0 };
                            let v8169: f64;
                            let v10158: Lanes<2>;
                            if v8163 != 0.0 {
                                let v8164 = v8162.sqrt();
                                let v8165 = v4 / v8164;
                                let v18010 = (((v18000 * (v9360 / (v10427 * v8164))) * v8165) * v10382) / v8164;
                                v8169 = v8165;
                                v10158 = v18010;
                            } else {
                                let v8166 = -v7979;
                                let v8167 = v8162.powf(v8166);
                                let v18004 = v18000 * (v8166 * (v8162.powf((v8166 - v9360))));
                                v8169 = v8167;
                                v10158 = v18004;
                            }
                            let v8168 = v7976 * v8094;
                            let v8173 = v4 - v7979;
                            let v8174 = (v8168 * (v4 - (v8162 * v8169))) / v8173;
                            let v18016 = ((((v18000 * v8169) + (v10158 * v8162)) * v10382) * v8168) / v8173;
                            v8183 = v8174;
                            v10157 = v18016;
                        } else {
                            v8183 = v0;
                            v10157 = v10372;
                        }
                        let v8175 = if v8158 > v0 { 1.0 } else { 0.0 };
                        let v8241: f64;
                        let v10159: Lanes<2>;
                        if v8175 != 0.0 {
                            let v8177 = v4 - (v7874 / v8012);
                            let v18018 = (v9378 / v8012) * v10382;
                            let v8178 = if v8015 == v8 { 1.0 } else { 0.0 };
                            let v8185: f64;
                            let v10160: Lanes<2>;
                            if v8178 != 0.0 {
                                let v8179 = v8177.sqrt();
                                let v8180 = v4 / v8179;
                                let v18028 = (((v18018 * (v9360 / (v10427 * v8179))) * v8180) * v10382) / v8179;
                                v8185 = v8180;
                                v10160 = v18028;
                            } else {
                                let v8181 = -v8015;
                                let v8182 = v8177.powf(v8181);
                                let v18022 = v18018 * (v8181 * (v8177.powf((v8181 - v9360))));
                                v8185 = v8182;
                                v10160 = v18022;
                            }
                            let v8184 = v8012 * v8158;
                            let v8189 = v4 - v8015;
                            let v8191 = v8183 + ((v8184 * (v4 - (v8177 * v8185))) / v8189);
                            let v18035 = v10157 + (((((v18018 * v8185) + (v10160 * v8177)) * v10382) * v8184) / v8189);
                            v8241 = v8191;
                            v10159 = v18035;
                        } else {
                            v8241 = v8183;
                            v10159 = v10157;
                        }
                        v8240 = v8241;
                        v10156 = v10159;
                    } else {
                        let v8197 = ((v8094 * v7979) / v7976) + ((v8158 * v8015) / v8012);
                        let v8200 = (v8094 + v8158) + ((v7874 * v8) * v8197);
                        let v8201 = v7874 * v8200;
                        let v17998 = (v9378 * v8200) + (((v9378 * v8) * v8197) * v7874);
                        v8240 = v8201;
                        v10156 = v17998;
                    }
                    v8237 = v8240;
                    v10148 = v10156;
                }
                let v8202 = if v7974 > v0 { 1.0 } else { 0.0 };
                let v8673: f64;
                let v10161: Lanes<2>;
                if v8202 != 0.0 {
                    let v8207 = -(((v8203 * v473) * v7964) * v7961);
                    let v8208 = v525 * v8207;
                    let v18098 = (v10135 * v10382) * v10382;
                    let v8216 = (v8207 - (-v8209)) - v8208;
                    let v8218 = (v85 * v8207) * v8208;
                    let v8219 = if v8218 > v0 { 1.0 } else { 0.0 };
                    let v8221: f64;
                    if v8219 != 0.0 {
                        v8221 = v8218;
                    } else {
                        let v8220 = -v8218;
                        v8221 = v8220;
                    }
                    let v18099 = v18098 * v8216;
                    let v8224 = ((v8216 * v8216) + v8221).sqrt();
                    let v8229 = (v8207 - (v8 * (v8216 + v8224))) * v8228;
                    let v18107 = (((v18098 + ((v18099 + v18099) * (v9360 / (v10427 * v8224)))) * v8) * v10382) * v8228;
                    v8673 = v8229;
                    v10161 = v18107;
                } else {
                    v8673 = v8209;
                    v10161 = v10135;
                }
                let v8230 = if v8094 > v0 { 1.0 } else { 0.0 };
                let v8671: f64;
                let v10162: Lanes<2>;
                if v8230 != 0.0 {
                    let v8235 = -(((v8231 * v473) * v7964) * v7959);
                    let v8236 = v525 * v8235;
                    let v18109 = (v10148 * v10382) * v10382;
                    let v8244 = (v8235 - (-v8237)) - v8236;
                    let v8246 = (v85 * v8235) * v8236;
                    let v8247 = if v8246 > v0 { 1.0 } else { 0.0 };
                    let v8249: f64;
                    if v8247 != 0.0 {
                        v8249 = v8246;
                    } else {
                        let v8248 = -v8246;
                        v8249 = v8248;
                    }
                    let v18110 = v18109 * v8244;
                    let v8252 = ((v8244 * v8244) + v8249).sqrt();
                    let v8257 = (v8235 - (v8 * (v8244 + v8252))) * v8256;
                    let v18118 = (((v18109 + ((v18110 + v18110) * (v9360 / (v10427 * v8252)))) * v8) * v10382) * v8256;
                    v8671 = v8257;
                    v10162 = v18118;
                } else {
                    v8671 = v8237;
                    v10162 = v10148;
                }
                v8668 = v7957;
                v8669 = v7955;
                v8670 = v8671;
                v8672 = v8673;
                v10129 = v17890;
                v10130 = v17887;
                v10131 = v10162;
                v10132 = v10161;
            } else {
                v8668 = v0;
                v8669 = v0;
                v8670 = v0;
                v8672 = v0;
                v10129 = v17762;
                v10130 = v17763;
                v10131 = v10372;
                v10132 = v10373;
            }
            let v8979: f64;
            let v8984: f64;
            let v10163: Lanes<6>;
            let v10164: Lanes<4>;
            if v66 != 0.0 {
                let v8980: f64;
                let v10165: Lanes<6>;
                if v5709 != 0.0 {
                    let v8261 = v8258 * v8259;
                    let v8262 = v8261 * v8260;
                    let v8266 = v8259 * v8260;
                    let v8269 = (((v5775 * v4838) * v8258) + (v8266 * v8260)) + v358;
                    let v8270 = (v8262 * v8260) / v8269;
                    let v18134 = ((((v9773 * v8261) * v8260) + (v9773 * v8262)) - (((((v9771 * v4838) + (v9444 * v5775)) * v8258) + (((v9773 * v8259) * v8260) + (v9773 * v8266))) * v8270)) / v8269;
                    v8980 = v8270;
                    v10165 = v18134;
                } else {
                    let v8271 = v8258 + v358;
                    v8980 = v8271;
                    v10165 = v11054;
                }
                let v8273 = v8272 * v1123;
                let v18135 = v9413 * v8272;
                v8979 = v8980;
                v8984 = v8273;
                v10163 = v10165;
                v10164 = v18135;
            } else {
                v8979 = v0;
                v8984 = v0;
                v10163 = v11054;
                v10164 = v10617;
            }
            let v8276 = if v4320 == 0.0 { 1.0 } else { 0.0 };
            let v8277 = if (if v8274 != v0 { 1.0 } else { 0.0 }) != 0.0 && v8276 != 0.0 { 1.0 } else { 0.0 };
            if v8277 != 0.0 {
                let v8278 = v4343 / v202;
                let v8290 = if (((((((v8279 * v8280) / v202) / v8283) / v164) - v8278) - v8278).abs()) > v8289 { 1.0 } else { 0.0 };
                if v8290 != 0.0 {
                } else {
                }
            } else {
            }
            let v8291 = if v4835 != v0 { 1.0 } else { 0.0 };
            let v8292 = if v8291 != 0.0 && v8276 != 0.0 { 1.0 } else { 0.0 };
            let v8396: f64;
            let v8724: f64;
            let v10166: Lanes<6>;
            let v10167: Lanes<6>;
            if v8292 != 0.0 {
                let v8303 = (v8293 - v4335) / v8260;
                let v8306 = (v8304 * v8303) / v4383;
                let v18143 = ((v9776 * v8303) + ((((v10030 - v9433) - (v9773 * v8303)) / v8260) * v8304)) / v4383;
                let v8311 = if (if v8307 <= v4543 { 1.0 } else { 0.0 }) != 0.0 && (if v4543 <= v8309 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8319: f64;
                let v10168: Lanes<6>;
                if v8311 != 0.0 {
                    v8319 = v4;
                    v10168 = v11054;
                } else {
                    let v8316 = if (if v8312 <= v4543 { 1.0 } else { 0.0 }) != 0.0 && (if v4543 <= v8314 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v8320: f64;
                    let v10169: Lanes<6>;
                    if v8316 != 0.0 {
                        v8320 = v8306;
                        v10169 = v18143;
                    } else {
                        let v8317 = v4543 - v4;
                        let v8318 = v8306.powf(v8317);
                        let v18147 = v18143 * (v8317 * (v8306.powf((v8317 - v9360))));
                        v8320 = v8318;
                        v10169 = v18147;
                    }
                    v8319 = v8320;
                    v10168 = v10169;
                }
                let v18150 = (v18143 * v8319) + (v10168 * v8306);
                let v8322 = v4 + (v8306 * v8319);
                let v8325 = (v8323 / v4543) - v4;
                let v8326 = v8322.powf(v8325);
                let v8327 = v8322 * v8326;
                let v8328 = v8304 * v8327;
                let v18160 = (v9776 * v8327) + (((v18150 * v8326) + ((v18150 * (v8325 * (v8322.powf((v8325 - v9360))))) * v8322)) * v8304);
                let v8330 = (v5775 + v8328) / v73;
                let v18162 = (v9771 + v18160) / v73;
                let v8331 = v4302 * v4302;
                let v18163 = v9429 * v4302;
                let v18164 = v18163 + v18163;
                let v8332 = v162 * v1123;
                let v8333 = v8332 * v4838;
                let v18166 = (v9413 * v162) * v4838;
                let v8334 = v8333 * v5775;
                let v8335 = v91 * v4302;
                let v18173 = v9429 * v91;
                let v8338 = (v4 + v8335) + (v641 * v8331);
                let v8339 = v8338 * v8328;
                let v8344 = (v91 + (v85 * v4302)) + (v91 * v8331);
                let v8345 = v8344 * v8328;
                let v8349 = (v641 + v8335) + v8331;
                let v8350 = v8349 * v5775;
                let v8352 = ((v8339 * v8328) + (v8345 * v5775)) + (v8350 * v5775);
                let v8355 = v8354 * v8260;
                let v8356 = v4 + v4302;
                let v8357 = v8355 * v8356;
                let v8358 = v8357 * v8330;
                let v8359 = v8358 * v8330;
                let v8360 = (v8334 * v8352) / v8359;
                let v18215 = (((((((Lanes([v18166[0], v18166[1], 0.0, v18166[2], v18166[3], 0.0])) + (v9444 * v8332)) * v5775) + (v9771 * v8333)) * v8352) + ((((((((v18173 + (v18164 * v641)) * v8328) + (v18160 * v8338)) * v8328) + (v18160 * v8339)) + ((((((v9429 * v85) + (v18164 * v91)) * v8328) + (v18160 * v8344)) * v5775) + (v9771 * v8345))) + (((((v18173 + v18164) * v5775) + (v9771 * v8349)) * v5775) + (v9771 * v8350))) * v8334)) - ((((((((v9773 * v8354) * v8356) + (v9429 * v8355)) * v8330) + (v18162 * v8357)) * v8330) + (v18162 * v8358)) * v8360)) / v8359;
                v8396 = v8360;
                v8724 = v8328;
                v10166 = v18215;
                v10167 = v18160;
            } else {
                v8396 = v0;
                v8724 = v0;
                v10166 = v11054;
                v10167 = v11054;
            }
            let v8368 = if (if (if (if v4833 != v0 { 1.0 } else { 0.0 }) != 0.0 && v8291 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v8363 == v4 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v8276 != 0.0 { 1.0 } else { 0.0 };
            let v8716: f64;
            let v8729: f64;
            let v8738: f64;
            let v8742: f64;
            let v10170: Lanes<6>;
            let v10171: Lanes<6>;
            let v10172: Lanes<6>;
            let v10173: Lanes<6>;
            if v8368 != 0.0 {
                let v8371 = v8369.sqrt();
                let v18218 = v9777 * (v9360 / (v10427 * v8371));
                let v8372 = v4838 + v8371;
                let v18219 = v9444 + v18218;
                let v18220 = v9778 * v8373;
                let v18222 = v9777 * v8369;
                let v8378 = v8377 * v8373;
                let v8383 = v816 * v8371;
                let v8384 = v8383 * v4838;
                let v8385 = v8373 + v8369;
                let v8387 = ((v8378 * v8369) + (v85 * ((v8373 * v8373) + (v8369 * v8369)))) + (v8384 * v8385);
                let v18239 = ((((v9778 * v8377) * v8369) + (v9777 * v8378)) + (((v18220 + v18220) + (v18222 + v18222)) * v85)) + (((((v18218 * v816) * v4838) + (v9444 * v8383)) * v8385) + ((v9778 + v9777) * v8384));
                let v8388 = v8372 * v8372;
                let v18240 = v18219 * v8372;
                let v8389 = v8388 * v8388;
                let v18242 = (v18240 + v18240) * v8388;
                let v8390 = v8389 * v8372;
                let v8391 = v8387 / v8390;
                let v18249 = (v18239 - ((((v18242 + v18242) * v8372) + (v18219 * v8389)) * v8391)) / v8390;
                let v8392 = v162 / v8260;
                let v8393 = v8392 * v5775;
                let v8394 = v8393 * v1123;
                let v18257 = v9413 * v8393;
                let v18259 = ((((((v9773 * v8392) * v10382) / v8260) * v5775) + (v9771 * v8392)) * v1123) + (Lanes([v18257[0], v18257[1], 0.0, v18257[2], v18257[3], 0.0]));
                let v8395 = v8394 * v4838;
                let v8397 = v8396 / v8395;
                let v8398 = v85 * v4838;
                let v8401 = (v8373 + (v8398 * v8371)) + v8369;
                let v8405 = v8402 * v8403;
                let v8407 = v641 * v8372;
                let v8408 = v8397 * v8372;
                let v8409 = v8408 * v4838;
                let v8411 = (v8409 * v8387).sqrt();
                let v8412 = v8407 * v8411;
                let v8413 = (v8405 * v8401) / v8412;
                let v18294 = ((((v9779 * v8402) * v8401) + (((v9778 + (((v9444 * v85) * v8371) + (v18218 * v8398))) + v9777) * v8405)) - ((((v18219 * v641) * v8411) + ((((((((((v10166 - (((v18259 * v4838) + (v9444 * v8394)) * v8397)) / v8395) * v8372) + (v18219 * v8397)) * v4838) + (v9444 * v8408)) * v8387) + (v18239 * v8409)) * (v9360 / (v10427 * v8411))) * v8407)) * v8413)) / v8412;
                v8716 = v8394;
                v8729 = v8371;
                v8738 = v8391;
                v8742 = v8413;
                v10170 = v18259;
                v10171 = v18218;
                v10172 = v18249;
                v10173 = v18294;
            } else {
                v8716 = v6;
                v8729 = v0;
                v8738 = v0;
                v8742 = v0;
                v10170 = v11054;
                v10171 = v11054;
                v10172 = v11054;
                v10173 = v11054;
            }
            let v8415 = v5615 + v8414;
            let v18295 = v9823 + v9879;
            let v8610: f64;
            let v8611: f64;
            let v8613: f64;
            let v10174: Lanes<6>;
            let v10175: Lanes<6>;
            let v10176: Lanes<4>;
            if v5 != 0.0 {
                let v8422 = v8416 + v8419;
                let v8426: f64;
                if v364 != 0.0 {
                    let v8425 = v8422 - (v8423 * v137);
                    v8426 = v8425;
                } else {
                    v8426 = v8422;
                }
                let v8427 = -v8426;
                let v8428 = v825 - v873;
                let v18308 = v10554 - (Lanes([v9407[0], v9407[1], 0.0, v9407[2]]));
                let v8435 = v8430 * ((v4 + (v8431 / v117)).ln());
                let v8436 = v8435 * v140;
                let v8439 = v8436 * (v141 + v8437);
                let v8442 = v8436 * (v141 + v8440);
                let v18312 = (v9404 - (Lanes([v9402[0], v9402[1], 0.0]))) * v8439;
                let v18313 = v9404 * v8442;
                let v8447 = (v8435 * v565) * v140;
                let v8452 = v8449 + (v8439 * (v825 - v818));
                let v18316 = v10035 + (Lanes([v18312[0], v18312[1], 0.0, v18312[2], 0.0, 0.0]));
                let v8456 = v8453 + (v8442 * v825);
                let v18318 = v10036 + (Lanes([v18313[0], v18313[1], 0.0, v18313[2], 0.0, 0.0]));
                let v8457 = (v8427 * v8428) + (v8447 * v8428);
                let v18319 = (v18308 * v8427) + (v18308 * v8447);
                v8610 = v8452;
                v8611 = v8456;
                v8613 = v8457;
                v10174 = v18316;
                v10175 = v18318;
                v10176 = v18319;
            } else {
                let v8614: f64;
                let v10177: Lanes<4>;
                if v364 != 0.0 {
                    let v8460 = -((-v8423) * v137);
                    let v8462 = v8460 * (v825 - v873);
                    let v18298 = (v10554 - (Lanes([v9407[0], v9407[1], 0.0, v9407[2]]))) * v8460;
                    v8614 = v8462;
                    v10177 = v18298;
                } else {
                    v8614 = v0;
                    v10177 = v10617;
                }
                let v8469 = ((v8463 * v141) * v140) * ((v4 + (v8431 / v117)).ln());
                let v18301 = (v9404 - (Lanes([v9402[0], v9402[1], 0.0]))) * v8469;
                let v18302 = v9404 * v8469;
                let v8473 = v8449 + (v8469 * (v825 - v818));
                let v18304 = v10035 + (Lanes([v18301[0], v18301[1], 0.0, v18301[2], 0.0, 0.0]));
                let v8474 = v8453 + (v8469 * v825);
                let v18306 = v10036 + (Lanes([v18302[0], v18302[1], 0.0, v18302[2], 0.0, 0.0]));
                v8610 = v8473;
                v8611 = v8474;
                v8613 = v8614;
                v10174 = v18304;
                v10175 = v18306;
                v10176 = v10177;
            }
            let v8608: f64;
            let v8632: f64;
            let v8644: f64;
            let v8988: f64;
            let v8994: f64;
            let v9002: f64;
            let v9026: f64;
            let v9033: f64;
            let v10178: Lanes<6>;
            let v10179: Lanes<6>;
            let v10180: Lanes<6>;
            let v10181: Lanes<6>;
            let v10182: Lanes<6>;
            let v10183: Lanes<6>;
            let v10184: Lanes<6>;
            if v66 != 0.0 {
                let v8989: f64;
                let v8995: f64;
                let v9003: f64;
                let v9027: f64;
                let v9034: f64;
                let v10185: Lanes<6>;
                let v10186: Lanes<6>;
                let v10187: Lanes<6>;
                let v10188: Lanes<6>;
                if v5 != 0.0 {
                    v8989 = v8;
                    v8995 = v8280;
                    v9003 = v8475;
                    v9027 = v0;
                    v9034 = v0;
                    v10185 = v9774;
                    v10186 = v9780;
                    v10187 = v11054;
                    v10188 = v11054;
                } else {
                    let v8488 = v8483 + v8484;
                    let v18330 = v9782 + v9783;
                    let v8494 = (v8280 - v8483) + v8490;
                    let v18332 = (v9774 - v9782) + v9784;
                    v8989 = v0;
                    v8995 = v0;
                    v9003 = v8479;
                    v9027 = v8488;
                    v9034 = v8494;
                    v10185 = v11054;
                    v10186 = v9781;
                    v10187 = v18330;
                    v10188 = v18332;
                }
                v8608 = v0;
                v8632 = v0;
                v8644 = v0;
                v8988 = v8989;
                v8994 = v8995;
                v9002 = v9003;
                v9026 = v9027;
                v9033 = v9034;
                v10178 = v11054;
                v10179 = v11054;
                v10180 = v11054;
                v10181 = v10185;
                v10182 = v10186;
                v10183 = v10187;
                v10184 = v10188;
            } else {
                let v8609: f64;
                let v8633: f64;
                let v8645: f64;
                let v10189: Lanes<6>;
                let v10190: Lanes<6>;
                let v10191: Lanes<6>;
                if v5 != 0.0 {
                    let v8496 = (-v8475) - v8280;
                    let v18328 = (v9780 * v10382) - v9774;
                    let v8497 = v8280 - v8483;
                    let v18329 = v9774 - v9782;
                    v8609 = v8496;
                    v8633 = v8483;
                    v8645 = v8497;
                    v10189 = v18328;
                    v10190 = v9782;
                    v10191 = v18329;
                } else {
                    let v8501 = (((-v8479) - v8280) - v8490) - v8484;
                    let v18323 = (((v9781 * v10382) - v9774) - v9784) - v9783;
                    let v8502 = v8483 + v8484;
                    let v18324 = v9782 + v9783;
                    let v8504 = (v8280 - v8483) + v8490;
                    let v18326 = (v9774 - v9782) + v9784;
                    v8609 = v8501;
                    v8633 = v8502;
                    v8645 = v8504;
                    v10189 = v18323;
                    v10190 = v18324;
                    v10191 = v18326;
                }
                v8608 = v8609;
                v8632 = v8633;
                v8644 = v8645;
                v8988 = v0;
                v8994 = v0;
                v9002 = v0;
                v9026 = v0;
                v9033 = v0;
                v10178 = v10189;
                v10179 = v10190;
                v10180 = v10191;
                v10181 = v11054;
                v10182 = v11054;
                v10183 = v11054;
                v10184 = v11054;
            }
            let v8505 = if v6870 == v0 { 1.0 } else { 0.0 };
            let v8530: f64;
            let v10192: Lanes<6>;
            if v8505 != 0.0 {
                v8530 = v0;
                v10192 = v11054;
            } else {
                let v8510 = (v8506 * v131) + v4335;
                let v18334 = (v10031 * v131) + v9433;
                let v8511 = if v8510 > v8293 { 1.0 } else { 0.0 };
                let v8515: f64;
                let v10193: Lanes<6>;
                if v8511 != 0.0 {
                    v8515 = v8293;
                    v10193 = v10030;
                } else {
                    v8515 = v8510;
                    v10193 = v18334;
                }
                let v8512 = v818 + v4335;
                let v18336 = (Lanes([v9402[0], v9402[1], 0.0, 0.0, 0.0, 0.0])) + v9433;
                let v8514 = v4 - v4351;
                let v8524 = (v118 * v164) * (((v8518 / v486).sqrt()) * v8521);
                let v8528 = (((v8512 - ((v4351 * v8512) + (v8514 * v8515))) / v6870) - v8506) * v8524;
                let v18343 = (((v18336 - ((v18336 * v4351) + (v10193 * v8514))) / v6870) - v10031) * v8524;
                v8530 = v8528;
                v10192 = v18343;
            }
            let v8529 = if v334 != v0 { 1.0 } else { 0.0 };
            let v8616: f64;
            let v10194: Lanes<6>;
            if v8529 != 0.0 {
                let v18344 = v9407 * v338;
                let v8532 = v8530 + (v338 * v873);
                let v18346 = v10192 + (Lanes([v18344[0], v18344[1], 0.0, 0.0, v18344[2], 0.0]));
                v8616 = v8532;
                v10194 = v18346;
            } else {
                v8616 = v8530;
                v10194 = v10192;
            }
            let v8533 = if v561 == v4 { 1.0 } else { 0.0 };
            let v8703: f64;
            let v9008: f64;
            let v9016: f64;
            let v9047: f64;
            let v9053: f64;
            let v10195: Lanes<6>;
            let v10196: Lanes<6>;
            let v10197: Lanes<6>;
            let v10198: Lanes<6>;
            let v10199: Lanes<6>;
            if v8533 != 0.0 {
                let v8704: f64;
                let v9009: f64;
                let v9017: f64;
                let v9048: f64;
                let v9054: f64;
                let v10200: Lanes<6>;
                let v10201: Lanes<6>;
                let v10202: Lanes<6>;
                let v10203: Lanes<6>;
                let v10204: Lanes<6>;
                if v5 != 0.0 {
                    let v18366 = (v9906 * v10382) - v9907;
                    let v8573 = (((-v8534) - v8542) - v8550) - v8562;
                    let v18369 = ((Lanes([v18366[0], v18366[1], v18366[2], v18366[3], v18366[4], 0.0])) - v9908) - v9909;
                    let v8607 = v8591 + v8598;
                    let v18373 = (Lanes([v9912[0], v9912[1], v9912[2], v9912[3], v9912[4], 0.0])) + v9913;
                    let v8631 = v8608 + ((((((v8610 + v8611) + v8613) - v8616) - v8618) - v8624) + v8573);
                    let v18383 = v10178 + ((((((v10174 + v10175) + (Lanes([v10176[0], v10176[1], 0.0, v10176[2], v10176[3], 0.0]))) - v10194) - (Lanes([v10037[0], v10037[1], v10037[2], v10037[3], v10037[4], 0.0]))) - (Lanes([v10038[0], v10038[1], v10038[2], v10038[3], v10038[4], 0.0]))) + v18369);
                    let v8643 = v8632 + ((((-v8610) + v8616) + v8636) + (v8574 + v8581));
                    let v18389 = v10179 + ((((v10174 * v10382) + v10194) + (Lanes([v10039[0], v10039[1], v10039[2], v10039[3], v10039[4], 0.0]))) + ((Lanes([v9910[0], v9910[1], v9910[2], v9910[3], v9910[4], 0.0])) + v9911));
                    let v8654 = v8644 + (((-v8611) + v8647) + v8607);
                    let v18394 = v10180 + (((v10175 * v10382) + (Lanes([v10040[0], v10040[1], v10040[2], v10040[3], v10040[4], 0.0]))) + v18373);
                    v8704 = v8631;
                    v9009 = v8607;
                    v9017 = v8573;
                    v9048 = v8643;
                    v9054 = v8654;
                    v10200 = v18383;
                    v10201 = v18373;
                    v10202 = v18369;
                    v10203 = v18389;
                    v10204 = v18394;
                } else {
                    let v8660 = v8608 + (((((v8610 + v8611) + v8613) - v8616) - v8618) - v8624);
                    let v18355 = v10178 + (((((v10174 + v10175) + (Lanes([v10176[0], v10176[1], 0.0, v10176[2], v10176[3], 0.0]))) - v10194) - (Lanes([v10037[0], v10037[1], v10037[2], v10037[3], v10037[4], 0.0]))) - (Lanes([v10038[0], v10038[1], v10038[2], v10038[3], v10038[4], 0.0])));
                    let v8664 = v8632 + (((-v8610) + v8616) + v8636);
                    let v18360 = v10179 + (((v10174 * v10382) + v10194) + (Lanes([v10039[0], v10039[1], v10039[2], v10039[3], v10039[4], 0.0])));
                    let v8667 = v8644 + ((-v8611) + v8647);
                    let v18364 = v10180 + ((v10175 * v10382) + (Lanes([v10040[0], v10040[1], v10040[2], v10040[3], v10040[4], 0.0])));
                    v8704 = v8660;
                    v9009 = v0;
                    v9017 = v0;
                    v9048 = v8664;
                    v9054 = v8667;
                    v10200 = v18355;
                    v10201 = v11054;
                    v10202 = v11054;
                    v10203 = v18360;
                    v10204 = v18364;
                }
                v8703 = v8704;
                v9008 = v9009;
                v9016 = v9017;
                v9047 = v9048;
                v9053 = v9054;
                v10195 = v10200;
                v10196 = v10201;
                v10197 = v10202;
                v10198 = v10203;
                v10199 = v10204;
            } else {
                v8703 = v8608;
                v9008 = v0;
                v9016 = v0;
                v9047 = v8632;
                v9053 = v8644;
                v10195 = v10178;
                v10196 = v11054;
                v10197 = v11054;
                v10198 = v10179;
                v10199 = v10180;
            }
            let v9074: f64;
            let v9075: f64;
            let v9076: f64;
            let v9077: f64;
            let v10205: Lanes<3>;
            let v10206: Lanes<2>;
            let v10207: Lanes<3>;
            let v10208: Lanes<2>;
            if v5 != 0.0 {
                v9074 = v8669;
                v9075 = v8670;
                v9076 = v8668;
                v9077 = v8672;
                v10205 = v10130;
                v10206 = v10131;
                v10207 = v10129;
                v10208 = v10132;
            } else {
                v9074 = v0;
                v9075 = v0;
                v9076 = v0;
                v9077 = v0;
                v10205 = v17763;
                v10206 = v10372;
                v10207 = v17762;
                v10208 = v10373;
            }
            let v8674 = if v1881 != v4 { 1.0 } else { 0.0 };
            let v9042: f64;
            let v10209: Lanes<6>;
            if v8674 != 0.0 {
                v9042 = v0;
                v10209 = v11054;
            } else {
                v9042 = v5635;
                v10209 = v9865;
            }
            let v8677 = -v8675;
            let v18395 = v9886 * v10382;
            let v8678 = if v7820 == v4 { 1.0 } else { 0.0 };
            let v9072: f64;
            let v10210: Lanes<6>;
            if v8678 != 0.0 {
                let v8686 = (v8679 * v8680) - v8684;
                let v18401 = (v9887 * v8679) - (Lanes([v9888[0], v9888[1], 0.0, v9888[2], 0.0, 0.0]));
                v9072 = v8686;
                v10210 = v18401;
            } else {
                let v8687 = v4 - v8679;
                let v8691 = (v8687 * v8680) - v8689;
                let v18398 = (v9887 * v8687) - (Lanes([v9889[0], v9889[1], 0.0, v9889[2], 0.0, 0.0]));
                v9072 = v8691;
                v10210 = v18398;
            }
            let v9073: f64;
            let v10211: Lanes<6>;
            if v8678 != 0.0 {
                let v8692 = v4 - v8679;
                let v8694 = (v8692 * v8680) - v8689;
                let v18407 = (v9887 * v8692) - (Lanes([v9889[0], v9889[1], 0.0, v9889[2], 0.0, 0.0]));
                v9073 = v8694;
                v10211 = v18407;
            } else {
                let v8696 = (v8679 * v8680) - v8684;
                let v18404 = (v9887 * v8679) - (Lanes([v9888[0], v9888[1], 0.0, v9888[2], 0.0, 0.0]));
                v9073 = v8696;
                v10211 = v18404;
            }
            let v8701: f64;
            let v10212: Lanes<5>;
            if v8678 != 0.0 {
                v8701 = v8697;
                v10212 = v9898;
            } else {
                v8701 = v8699;
                v10212 = v9902;
            }
            let v8702: f64;
            let v10213: Lanes<5>;
            if v8678 != 0.0 {
                v8702 = v8699;
                v10213 = v9902;
            } else {
                v8702 = v8697;
                v10213 = v9898;
            }
            let v8705 = v361 * (v10195[0]);
            let v8706 = v361 * (v10195[1]);
            let v8707 = if v7820 > v0 { 1.0 } else { 0.0 };
            let v8708: f64;
            if v8707 != 0.0 {
                v8708 = v8706;
            } else {
                v8708 = v8705;
            }
            let v9116: f64;
            let v9118: f64;
            let v10214: Lanes<6>;
            let v10215: Lanes<6>;
            if v8368 != 0.0 {
                let v8711 = ((v18 * v1123) * v164) * v134;
                let v18412 = ((v10407 * v8712) * v8708) * v8708;
                let v8717 = (((v8712 * v660) * v8708) * v8708) / v8716;
                let v18416 = ((Lanes([0.0, 0.0, v18412[0], 0.0, 0.0, 0.0])) - (v10170 * v8717)) / v8716;
                let v8722 = if (if v8403 > v8718 { 1.0 } else { 0.0 }) != 0.0 && (if v818 > v8720 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8740: f64;
                let v10216: Lanes<6>;
                if v8722 != 0.0 {
                    let v8723 = v8304 / v5775;
                    let v18422 = (v9776 - (v9771 * v8723)) / v5775;
                    let v8725 = v8304 / v8724;
                    let v8727 = (v8725 - v8723) / v818;
                    let v18427 = v9402 * v8727;
                    let v8728 = v4269 * v8727;
                    let v8732 = (v8373 + (v4838 * v8729)) + v8369;
                    let v8734 = v4838 + v8729;
                    let v8735 = (v8728 * v8732) / v8734;
                    let v8736 = v8723 + v8735;
                    let v18444 = v18422 + ((((((((((v9776 - (v10167 * v8725)) / v8724) - v18422) - (Lanes([v18427[0], v18427[1], 0.0, 0.0, 0.0, 0.0]))) / v818) * v4269) * v8732) + (((v9778 + ((v9444 * v8729) + (v10171 * v4838))) + v9777) * v8728)) - ((v9444 + v10171) * v8735)) / v8734);
                    v8740 = v8736;
                    v10216 = v18444;
                } else {
                    let v8737 = v8304 / v8724;
                    let v18419 = (v9776 - (v10167 * v8737)) / v8724;
                    v8740 = v8737;
                    v10216 = v18419;
                }
                let v8739 = v8717 * v8738;
                let v8741 = v8739 * v8740;
                let v18450 = (((v18416 * v8738) + (v10172 * v8717)) * v8740) + (v10216 * v8739);
                let v8744 = if (-v8708) > v8711 { 1.0 } else { 0.0 };
                let v8746 = if v8744 != 0.0 && (if v8741 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8747: f64;
                let v10217: Lanes<6>;
                if v8746 != 0.0 {
                    v8747 = v8741;
                    v10217 = v18450;
                } else {
                    v8747 = v0;
                    v10217 = v11054;
                }
                let v8748: f64;
                let v10218: Lanes<6>;
                if v8744 != 0.0 {
                    v8748 = v8742;
                    v10218 = v10173;
                } else {
                    v8748 = v0;
                    v10218 = v11054;
                }
                v9116 = v8748;
                v9118 = v8747;
                v10214 = v10218;
                v10215 = v10217;
            } else {
                v9116 = v0;
                v9118 = v0;
                v10214 = v11054;
                v10215 = v11054;
            }
            let v8750 = if v8749 == v4 { 1.0 } else { 0.0 };
            let v9041: f64;
            let v10219: Lanes<5>;
            if v8750 != 0.0 {
                let v8780: f64;
                let v8782: f64;
                let v8791: f64;
                let v8814: f64;
                let v8815: f64;
                let v8863: f64;
                let v8869: f64;
                let v10220: Lanes<4>;
                if v8751 != 0.0 {
                    let v8753 = v8752 / v18;
                    let v8758 = if v8757 > v0 { 1.0 } else { 0.0 };
                    let v8761: f64;
                    if v8758 != 0.0 {
                        let v8760 = v8757 * v8759;
                        v8761 = v8760;
                    } else {
                        v8761 = v0;
                    }
                    let v8764 = v361 * (v598 - v608);
                    let v18460 = ((Lanes([0.0, v9362[0]])) - (Lanes([v9366[0], 0.0]))) * v361;
                    let v18461 = Lanes([0.0, v18460[0], 0.0, v18460[1]]);
                    v8780 = v8754;
                    v8782 = v8755;
                    v8791 = v8756;
                    v8814 = v8764;
                    v8815 = v8762;
                    v8863 = v8753;
                    v8869 = v8761;
                    v10220 = v18461;
                } else {
                    let v8768 = if v8757 > v0 { 1.0 } else { 0.0 };
                    let v8771: f64;
                    if v8768 != 0.0 {
                        let v8770 = v8757 * v8769;
                        v8771 = v8770;
                    } else {
                        v8771 = v0;
                    }
                    let v8774 = v361 * (v607 - v597);
                    let v18455 = ((Lanes([v9365[0], 0.0])) - (Lanes([0.0, v9361[0]]))) * v361;
                    let v18456 = Lanes([v18455[0], 0.0, v18455[1], 0.0]);
                    v8780 = v8765;
                    v8782 = v8766;
                    v8791 = v8767;
                    v8814 = v8774;
                    v8815 = v8772;
                    v8863 = v34;
                    v8869 = v8771;
                    v10220 = v18456;
                }
                let v8779 = ((v8775 * v8775) + (v129 * v129)).sqrt();
                let v8785 = v694.powf(v8784);
                let v8786 = (v8780 / v552) / v8785;
                let v8789 = v708 - (v8787 * v709);
                let v8790 = (v8782 / v63) / v8789;
                let v18474 = v9389 * v8792;
                let v8794 = v8791 + (v8792 * v648);
                let v8799 = v4 + (v8795 / (v138.powf(v8796)));
                let v8804 = v4 + (v8800 / (v138.powf(v8801)));
                let v8809 = v4 + (v8805 / (v165.powf(v8806)));
                let v8810 = v8786 * v8799;
                let v18475 = ((((v10408 * (v8784 * (v694.powf((v8784 - v9360))))) * v8786) * v10382) / v8785) * v8799;
                let v18477 = (((((v10420 - (v10421 * v8787)) * v8790) * v10382) / v8789) * v8809) * v8804;
                let v8813 = ((v8790 * v8809) * v8804) + v358;
                let v8816 = v8814 / v8815;
                let v8817 = v8810 * v8816;
                let v18479 = v18475 * v8816;
                let v18480 = (v10220 / v8815) * v8810;
                let v18483 = (Lanes([0.0, 0.0, 0.0, 0.0, v18479[0]])) + (Lanes([v18480[0], v18480[1], v18480[2], v18480[3], 0.0]));
                let v8818 = if v8814 >= v0 { 1.0 } else { 0.0 };
                let v8832: f64;
                let v10221: Lanes<5>;
                if v8818 != 0.0 {
                    let v8819 = v8817 / v8813;
                    let v18489 = v18477 * v8819;
                    let v18492 = (v18483 - (Lanes([0.0, 0.0, 0.0, 0.0, v18489[0]]))) / v8813;
                    v8832 = v8819;
                    v10221 = v18492;
                } else {
                    let v8821 = (-v8817) / v8813;
                    let v18485 = v18477 * v8821;
                    let v18488 = ((v18483 * v10382) - (Lanes([0.0, 0.0, 0.0, 0.0, v18485[0]]))) / v8813;
                    v8832 = v8821;
                    v10221 = v18488;
                }
                let v8826 = if (if v8822 <= v8794 { 1.0 } else { 0.0 }) != 0.0 && (if v8794 <= v8824 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8835: f64;
                let v10222: Lanes<5>;
                if v8826 != 0.0 {
                    v8835 = v4;
                    v10222 = v18451;
                } else {
                    let v8831 = if (if v8827 <= v8794 { 1.0 } else { 0.0 }) != 0.0 && (if v8794 <= v8829 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v8836: f64;
                    let v10223: Lanes<5>;
                    if v8831 != 0.0 {
                        v8836 = v8832;
                        v10223 = v10221;
                    } else {
                        let v8833 = v8794 - v4;
                        let v8834 = v8832.powf(v8833);
                        let v18499 = v18474 * (v8834 * (v8832.ln()));
                        let v18501 = (v10221 * (v8833 * (v8832.powf((v8833 - v9360))))) + (Lanes([0.0, 0.0, 0.0, 0.0, v18499[0]]));
                        v8836 = v8834;
                        v10223 = v18501;
                    }
                    v8835 = v8836;
                    v10222 = v10223;
                }
                let v18504 = (v10221 * v8835) + (v10222 * v8832);
                let v8838 = v4 + (v8832 * v8835);
                let v8843 = if (if v8839 <= v8794 { 1.0 } else { 0.0 }) != 0.0 && (if v8794 <= v8841 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8857: f64;
                let v10224: Lanes<5>;
                if v8843 != 0.0 {
                    let v8844 = v4 / v8838;
                    let v18528 = ((v18504 * v8844) * v10382) / v8838;
                    v8857 = v8844;
                    v10224 = v18528;
                } else {
                    let v8849 = if (if v8845 <= v8794 { 1.0 } else { 0.0 }) != 0.0 && (if v8794 <= v8847 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v8858: f64;
                    let v10225: Lanes<5>;
                    if v8849 != 0.0 {
                        let v8850 = v8838.sqrt();
                        let v8851 = v4 / v8850;
                        let v18525 = (((v18504 * (v9360 / (v10427 * v8850))) * v8851) * v10382) / v8850;
                        v8858 = v8851;
                        v10225 = v18525;
                    } else {
                        let v8853 = v8852 / v8794;
                        let v8854 = v8853 - v4;
                        let v8855 = v8838.powf(v8854);
                        let v18514 = (((v18474 * v8853) * v10382) / v8794) * (v8855 * (v8838.ln()));
                        let v8856 = v8838 * v8855;
                        let v18519 = (v18504 * v8855) + (((v18504 * (v8854 * (v8838.powf((v8854 - v9360))))) + (Lanes([0.0, 0.0, 0.0, 0.0, v18514[0]]))) * v8838);
                        v8858 = v8856;
                        v10225 = v18519;
                    }
                    v8857 = v8858;
                    v10224 = v10225;
                }
                let v18529 = v18475 * v8857;
                let v8861 = (v202 / v8815) * v8779;
                let v8864 = (v8861 * (v8810 * v8857)) * v8863;
                let v18534 = (((Lanes([0.0, 0.0, 0.0, 0.0, v18529[0]])) + (v10224 * v8810)) * v8861) * v8863;
                let v8865 = if v8864 <= v0 { 1.0 } else { 0.0 };
                let v8866: f64;
                let v10226: Lanes<5>;
                if v8865 != 0.0 {
                    v8866 = v358;
                    v10226 = v18451;
                } else {
                    v8866 = v8864;
                    v10226 = v18534;
                }
                let v8867 = v4 / v8866;
                let v18538 = (((v10226 * v8867) * v10382) / v8866) / v162;
                let v8870 = (v8867 / v162) + v8869;
                let v8872 = if (if v8870 > v24 { 1.0 } else { 0.0 }) != 0.0 && v8291 != 0.0 { 1.0 } else { 0.0 };
                if v8872 != 0.0 {
                } else {
                }
                let v8873 = if v8870 < v24 { 1.0 } else { 0.0 };
                let v8874: f64;
                let v10227: Lanes<5>;
                if v8873 != 0.0 {
                    v8874 = v24;
                    v10227 = v18451;
                } else {
                    v8874 = v8870;
                    v10227 = v18538;
                }
                v9041 = v8874;
                v10219 = v10227;
            } else {
                v9041 = v0;
                v10219 = v18451;
            }
            let v8876 = if v8875 == v4 { 1.0 } else { 0.0 };
            let v9040: f64;
            let v10228: Lanes<5>;
            if v8876 != 0.0 {
                let v8893: f64;
                let v8895: f64;
                let v8902: f64;
                let v8918: f64;
                let v8919: f64;
                let v8967: f64;
                let v8973: f64;
                let v10229: Lanes<4>;
                if v8877 != 0.0 {
                    let v8878 = v8752 / v18;
                    let v8879 = if v8757 > v0 { 1.0 } else { 0.0 };
                    let v8881: f64;
                    if v8879 != 0.0 {
                        let v8880 = v8757 * v8759;
                        v8881 = v8880;
                    } else {
                        v8881 = v0;
                    }
                    let v8883 = v361 * (v598 - v608);
                    let v18547 = ((Lanes([0.0, v9362[0]])) - (Lanes([v9366[0], 0.0]))) * v361;
                    let v18548 = Lanes([0.0, v18547[0], 0.0, v18547[1]]);
                    v8893 = v8754;
                    v8895 = v8755;
                    v8902 = v8756;
                    v8918 = v8883;
                    v8919 = v8762;
                    v8967 = v8878;
                    v8973 = v8881;
                    v10229 = v18548;
                } else {
                    let v8884 = if v8757 > v0 { 1.0 } else { 0.0 };
                    let v8886: f64;
                    if v8884 != 0.0 {
                        let v8885 = v8757 * v8769;
                        v8886 = v8885;
                    } else {
                        v8886 = v0;
                    }
                    let v8888 = v361 * (v607 - v597);
                    let v18542 = ((Lanes([v9365[0], 0.0])) - (Lanes([0.0, v9361[0]]))) * v361;
                    let v18543 = Lanes([v18542[0], 0.0, v18542[1], 0.0]);
                    v8893 = v8765;
                    v8895 = v8766;
                    v8902 = v8767;
                    v8918 = v8888;
                    v8919 = v8772;
                    v8967 = v34;
                    v8973 = v8886;
                    v10229 = v18543;
                }
                let v8892 = ((v8775 * v8775) + (v129 * v129)).sqrt();
                let v8897 = v694.powf(v8784);
                let v8898 = (v8893 / v552) / v8897;
                let v8900 = v708 - (v8787 * v709);
                let v8901 = (v8895 / v63) / v8900;
                let v18561 = v9389 * v8792;
                let v8904 = v8902 + (v8792 * v648);
                let v8907 = v4 + (v8795 / (v138.powf(v8796)));
                let v8910 = v4 + (v8800 / (v138.powf(v8801)));
                let v8913 = v4 + (v8805 / (v165.powf(v8806)));
                let v8914 = v8898 * v8907;
                let v18562 = ((((v10408 * (v8784 * (v694.powf((v8784 - v9360))))) * v8898) * v10382) / v8897) * v8907;
                let v18564 = (((((v10420 - (v10421 * v8787)) * v8901) * v10382) / v8900) * v8913) * v8910;
                let v8917 = ((v8901 * v8913) * v8910) + v358;
                let v8920 = v8918 / v8919;
                let v8921 = v8914 * v8920;
                let v18566 = v18562 * v8920;
                let v18567 = (v10229 / v8919) * v8914;
                let v18570 = (Lanes([0.0, 0.0, 0.0, 0.0, v18566[0]])) + (Lanes([v18567[0], v18567[1], v18567[2], v18567[3], 0.0]));
                let v8922 = if v8918 >= v0 { 1.0 } else { 0.0 };
                let v8936: f64;
                let v10230: Lanes<5>;
                if v8922 != 0.0 {
                    let v8923 = v8921 / v8917;
                    let v18576 = v18564 * v8923;
                    let v18579 = (v18570 - (Lanes([0.0, 0.0, 0.0, 0.0, v18576[0]]))) / v8917;
                    v8936 = v8923;
                    v10230 = v18579;
                } else {
                    let v8925 = (-v8921) / v8917;
                    let v18572 = v18564 * v8925;
                    let v18575 = ((v18570 * v10382) - (Lanes([0.0, 0.0, 0.0, 0.0, v18572[0]]))) / v8917;
                    v8936 = v8925;
                    v10230 = v18575;
                }
                let v8930 = if (if v8926 <= v8904 { 1.0 } else { 0.0 }) != 0.0 && (if v8904 <= v8928 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8939: f64;
                let v10231: Lanes<5>;
                if v8930 != 0.0 {
                    v8939 = v4;
                    v10231 = v18451;
                } else {
                    let v8935 = if (if v8931 <= v8904 { 1.0 } else { 0.0 }) != 0.0 && (if v8904 <= v8933 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v8940: f64;
                    let v10232: Lanes<5>;
                    if v8935 != 0.0 {
                        v8940 = v8936;
                        v10232 = v10230;
                    } else {
                        let v8937 = v8904 - v4;
                        let v8938 = v8936.powf(v8937);
                        let v18586 = v18561 * (v8938 * (v8936.ln()));
                        let v18588 = (v10230 * (v8937 * (v8936.powf((v8937 - v9360))))) + (Lanes([0.0, 0.0, 0.0, 0.0, v18586[0]]));
                        v8940 = v8938;
                        v10232 = v18588;
                    }
                    v8939 = v8940;
                    v10231 = v10232;
                }
                let v18591 = (v10230 * v8939) + (v10231 * v8936);
                let v8942 = v4 + (v8936 * v8939);
                let v8947 = if (if v8943 <= v8904 { 1.0 } else { 0.0 }) != 0.0 && (if v8904 <= v8945 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8961: f64;
                let v10233: Lanes<5>;
                if v8947 != 0.0 {
                    let v8948 = v4 / v8942;
                    let v18615 = ((v18591 * v8948) * v10382) / v8942;
                    v8961 = v8948;
                    v10233 = v18615;
                } else {
                    let v8953 = if (if v8949 <= v8904 { 1.0 } else { 0.0 }) != 0.0 && (if v8904 <= v8951 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v8962: f64;
                    let v10234: Lanes<5>;
                    if v8953 != 0.0 {
                        let v8954 = v8942.sqrt();
                        let v8955 = v4 / v8954;
                        let v18612 = (((v18591 * (v9360 / (v10427 * v8954))) * v8955) * v10382) / v8954;
                        v8962 = v8955;
                        v10234 = v18612;
                    } else {
                        let v8957 = v8956 / v8904;
                        let v8958 = v8957 - v4;
                        let v8959 = v8942.powf(v8958);
                        let v18601 = (((v18561 * v8957) * v10382) / v8904) * (v8959 * (v8942.ln()));
                        let v8960 = v8942 * v8959;
                        let v18606 = (v18591 * v8959) + (((v18591 * (v8958 * (v8942.powf((v8958 - v9360))))) + (Lanes([0.0, 0.0, 0.0, 0.0, v18601[0]]))) * v8942);
                        v8962 = v8960;
                        v10234 = v18606;
                    }
                    v8961 = v8962;
                    v10233 = v10234;
                }
                let v18616 = v18562 * v8961;
                let v8965 = (v202 / v8919) * v8892;
                let v8968 = (v8965 * (v8914 * v8961)) * v8967;
                let v18621 = (((Lanes([0.0, 0.0, 0.0, 0.0, v18616[0]])) + (v10233 * v8914)) * v8965) * v8967;
                let v8969 = if v8968 <= v0 { 1.0 } else { 0.0 };
                let v8970: f64;
                let v10235: Lanes<5>;
                if v8969 != 0.0 {
                    v8970 = v358;
                    v10235 = v18451;
                } else {
                    v8970 = v8968;
                    v10235 = v18621;
                }
                let v8971 = v4 / v8970;
                let v18625 = (((v10235 * v8971) * v10382) / v8970) / v162;
                let v8974 = (v8971 / v162) + v8973;
                let v8976 = if (if v8974 > v24 { 1.0 } else { 0.0 }) != 0.0 && v8291 != 0.0 { 1.0 } else { 0.0 };
                if v8976 != 0.0 {
                } else {
                }
                let v8977 = if v8974 < v24 { 1.0 } else { 0.0 };
                let v8978: f64;
                let v10236: Lanes<5>;
                if v8977 != 0.0 {
                    v8978 = v24;
                    v10236 = v18451;
                } else {
                    v8978 = v8974;
                    v10236 = v18625;
                }
                v9040 = v8978;
                v10228 = v10236;
            } else {
                v9040 = v0;
                v10228 = v18451;
            }
            let v9043: f64;
            let v9049: f64;
            let v9055: f64;
            let v9061: f64;
            let v9190: f64;
            let v9192: f64;
            let v9226: f64;
            let v9228: f64;
            let v10237: Lanes<10>;
            let v10238: Lanes<8>;
            let v10239: Lanes<8>;
            let v10240: Lanes<1>;
            let v10241: Lanes<7>;
            let v10242: Lanes<7>;
            let v10243: Lanes<7>;
            let v10244: Lanes<7>;
            if v5 != 0.0 {
                let v9044: f64;
                let v9050: f64;
                let v9056: f64;
                let v9062: f64;
                let v9191: f64;
                let v9193: f64;
                let v10245: Lanes<8>;
                let v10246: Lanes<7>;
                let v10247: Lanes<7>;
                let v10248: Lanes<1>;
                let v10249: Lanes<7>;
                let v10250: Lanes<7>;
                if v66 != 0.0 {
                    let v8982 = if v8979 < v8981 { 1.0 } else { 0.0 };
                    let v8997: f64;
                    let v10251: Lanes<6>;
                    if v8982 != 0.0 {
                        v8997 = v8983;
                        v10251 = v11054;
                    } else {
                        v8997 = v8979;
                        v10251 = v10163;
                    }
                    let v8986 = if v8984 < v8985 { 1.0 } else { 0.0 };
                    let v9005: f64;
                    let v10252: Lanes<4>;
                    if v8986 != 0.0 {
                        v9005 = v8987;
                        v10252 = v10617;
                    } else {
                        v9005 = v8984;
                        v10252 = v10164;
                    }
                    let v8991: f64;
                    if v8678 != 0.0 {
                        v8991 = v8988;
                    } else {
                        let v8990 = v4 - v8988;
                        v8991 = v8990;
                    }
                    let v8998 = (v8992 - v8994) / v8997;
                    let v18659 = v10251 * v8998;
                    let v18662 = (((Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, v9380[0]])) - (Lanes([v10181[0], v10181[1], v10181[2], v10181[3], v10181[4], v10181[5], 0.0]))) - (Lanes([v18659[0], v18659[1], v18659[2], v18659[3], v18659[4], v18659[5], 0.0]))) / v8997;
                    let v9006 = (v8999 - v9002) / v9005;
                    let v18666 = v10252 * v9006;
                    let v18669 = (((Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v9381[0], 0.0])) - (Lanes([v10182[0], v10182[1], v10182[2], v10182[3], v10182[4], 0.0, v10182[5]]))) - (Lanes([v18666[0], v18666[1], 0.0, v18666[2], v18666[3], 0.0, 0.0]))) / v9005;
                    let v18670 = v9380 * v8991;
                    let v9010 = (v8992 * v8991) + v9008;
                    let v18672 = Lanes([v10196[0], v10196[1], v10196[2], v10196[3], v10196[4], v10196[5], 0.0]);
                    let v18673 = (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, v18670[0]])) + v18672;
                    let v9011 = v4 - v8991;
                    let v18674 = v9380 * v9011;
                    let v9013 = (v8992 * v9011) + v9008;
                    let v18676 = (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, v18674[0]])) + v18672;
                    let v18677 = v9380 * v10382;
                    let v18680 = (Lanes([0.0, v18677[0]])) - (Lanes([v9381[0], 0.0]));
                    let v9018 = ((-v8992) - v8999) + v9016;
                    let v18683 = (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v18680[0], 0.0, v18680[1]])) + (Lanes([v10197[0], v10197[1], v10197[2], v10197[3], v10197[4], 0.0, v10197[5], 0.0]));
                    v9044 = v9018;
                    v9050 = v9010;
                    v9056 = v9013;
                    v9062 = v8999;
                    v9191 = v8998;
                    v9193 = v9006;
                    v10245 = v18683;
                    v10246 = v18673;
                    v10247 = v18676;
                    v10248 = v9381;
                    v10249 = v18662;
                    v10250 = v18669;
                } else {
                    v9044 = v0;
                    v9050 = v0;
                    v9056 = v0;
                    v9062 = v0;
                    v9191 = v0;
                    v9193 = v0;
                    v10245 = v18655;
                    v10246 = v18653;
                    v10247 = v18653;
                    v10248 = v10366;
                    v10249 = v18653;
                    v10250 = v18654;
                }
                let v18684 = Lanes([v10245[0], v10245[1], v10245[2], v10245[3], v10245[4], v10245[5], 0.0, 0.0, v10245[6], v10245[7]]);
                let v18685 = Lanes([v10246[0], v10246[1], v10246[2], v10246[3], v10246[4], 0.0, v10246[5], v10246[6]]);
                let v18686 = Lanes([v10247[0], v10247[1], v10247[2], v10247[3], v10247[4], 0.0, v10247[5], v10247[6]]);
                v9043 = v9044;
                v9049 = v9050;
                v9055 = v9056;
                v9061 = v9062;
                v9190 = v9191;
                v9192 = v9193;
                v9226 = v0;
                v9228 = v0;
                v10237 = v18684;
                v10238 = v18685;
                v10239 = v18686;
                v10240 = v10248;
                v10241 = v10249;
                v10242 = v10250;
                v10243 = v18627;
                v10244 = v18628;
            } else {
                let v9045: f64;
                let v9051: f64;
                let v9057: f64;
                let v9063: f64;
                let v9227: f64;
                let v9229: f64;
                let v10253: Lanes<3>;
                let v10254: Lanes<1>;
                let v10255: Lanes<1>;
                let v10256: Lanes<1>;
                let v10257: Lanes<7>;
                let v10258: Lanes<7>;
                if v66 != 0.0 {
                    let v9020 = if v8979 < v9019 { 1.0 } else { 0.0 };
                    let v9029: f64;
                    let v10259: Lanes<6>;
                    if v9020 != 0.0 {
                        v9029 = v9021;
                        v10259 = v11054;
                    } else {
                        v9029 = v8979;
                        v10259 = v10163;
                    }
                    let v9023 = if v8984 < v9022 { 1.0 } else { 0.0 };
                    if v9023 != 0.0 {
                    } else {
                    }
                    let v9030 = (v9024 - v9026) / v9029;
                    let v18632 = v10259 * v9030;
                    let v18635 = (((Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v9382[0], 0.0])) - (Lanes([v10183[0], v10183[1], v10183[2], v10183[3], v10183[4], 0.0, v10183[5]]))) - (Lanes([v18632[0], v18632[1], v18632[2], v18632[3], v18632[4], 0.0, v18632[5]]))) / v9029;
                    let v9036 = (v9031 - v9033) / v9029;
                    let v18639 = v10259 * v9036;
                    let v18642 = (((Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v9383[0], 0.0])) - (Lanes([v10184[0], v10184[1], v10184[2], v10184[3], v10184[4], 0.0, v10184[5]]))) - (Lanes([v18639[0], v18639[1], v18639[2], v18639[3], v18639[4], 0.0, v18639[5]]))) / v9029;
                    let v18643 = v9382 * v10382;
                    let v18646 = (Lanes([v18643[0], 0.0])) - (Lanes([0.0, v9383[0]]));
                    let v9039 = ((-v9024) - v9031) - v8999;
                    let v18649 = (Lanes([0.0, v18646[0], v18646[1]])) - (Lanes([v9381[0], 0.0, 0.0]));
                    v9045 = v9039;
                    v9051 = v9024;
                    v9057 = v9031;
                    v9063 = v8999;
                    v9227 = v9030;
                    v9229 = v9036;
                    v10253 = v18649;
                    v10254 = v9382;
                    v10255 = v9383;
                    v10256 = v9381;
                    v10257 = v18635;
                    v10258 = v18642;
                } else {
                    v9045 = v0;
                    v9051 = v0;
                    v9057 = v0;
                    v9063 = v0;
                    v9227 = v0;
                    v9229 = v0;
                    v10253 = v18626;
                    v10254 = v10367;
                    v10255 = v10368;
                    v10256 = v10366;
                    v10257 = v18627;
                    v10258 = v18628;
                }
                let v18650 = Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v10253[0], v10253[1], v10253[2], 0.0, 0.0]);
                let v18651 = Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v10254[0], 0.0, 0.0]);
                let v18652 = Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v10255[0], 0.0, 0.0]);
                v9043 = v9045;
                v9049 = v9051;
                v9055 = v9057;
                v9061 = v9063;
                v9190 = v0;
                v9192 = v0;
                v9226 = v9227;
                v9228 = v9229;
                v10237 = v18650;
                v10238 = v18651;
                v10239 = v18652;
                v10240 = v10256;
                v10241 = v18653;
                v10242 = v18654;
                v10243 = v10257;
                v10244 = v10258;
            }
            let v9080: f64;
            let v9083: f64;
            let v9084: f64;
            let v9086: f64;
            let v9087: f64;
            let v9088: f64;
            let v10260: Lanes<6>;
            let v10261: Lanes<6>;
            let v10262: Lanes<6>;
            let v10263: Lanes<10>;
            let v10264: Lanes<9>;
            let v10265: Lanes<7>;
            if v8678 != 0.0 {
                let v9046 = v8703 + v9043;
                let v18700 = (Lanes([v10195[0], v10195[1], v10195[2], v10195[3], v10195[4], 0.0, 0.0, 0.0, v10195[5], 0.0])) + v10237;
                let v9052 = v9047 + v9049;
                let v18702 = (Lanes([v10198[0], v10198[1], v10198[2], v10198[3], v10198[4], 0.0, v10198[5], 0.0])) + v10238;
                let v18705 = ((v10195 + v10198) + v10199) * v10382;
                let v9064 = (-((v8703 + v9047) + v9053)) + v9061;
                let v18708 = (Lanes([v18705[0], v18705[1], v18705[2], v18705[3], v18705[4], 0.0, v18705[5]])) + (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v10240[0], 0.0]));
                let v18709 = Lanes([v18702[0], v18702[1], v18702[2], v18702[3], v18702[4], v18702[5], 0.0, v18702[6], v18702[7]]);
                v9080 = v8415;
                v9083 = v9042;
                v9084 = v0;
                v9086 = v9046;
                v9087 = v9052;
                v9088 = v9064;
                v10260 = v18295;
                v10261 = v10209;
                v10262 = v11054;
                v10263 = v18700;
                v10264 = v18709;
                v10265 = v18708;
            } else {
                let v9065 = -v8415;
                let v18687 = v18295 * v10382;
                let v9066 = v8703 + v9043;
                let v18689 = (Lanes([v10195[0], v10195[1], v10195[2], v10195[3], v10195[4], 0.0, 0.0, 0.0, v10195[5], 0.0])) + v10237;
                let v9067 = v9053 + v9055;
                let v18691 = (Lanes([v10199[0], v10199[1], v10199[2], v10199[3], v10199[4], 0.0, v10199[5], 0.0])) + v10239;
                let v18694 = ((v10195 + v10198) + v10199) * v10382;
                let v9071 = (-((v8703 + v9047) + v9053)) + v9061;
                let v18697 = (Lanes([v18694[0], v18694[1], v18694[2], v18694[3], v18694[4], 0.0, v18694[5]])) + (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v10240[0], 0.0]));
                let v18698 = Lanes([v18691[0], v18691[1], v18691[2], v18691[3], v18691[4], 0.0, v18691[5], v18691[6], v18691[7]]);
                v9080 = v9065;
                v9083 = v0;
                v9084 = v9042;
                v9086 = v9066;
                v9087 = v9067;
                v9088 = v9071;
                v10260 = v18687;
                v10261 = v11054;
                v10262 = v10209;
                v10263 = v18689;
                v10264 = v18698;
                v10265 = v18697;
            }
            let v9089: f64;
            let v9090: f64;
            let v9091: f64;
            let v9092: f64;
            let v10266: Lanes<3>;
            let v10267: Lanes<3>;
            let v10268: Lanes<2>;
            let v10269: Lanes<2>;
            if v5 != 0.0 {
                v9089 = v9074;
                v9090 = v9076;
                v9091 = v9075;
                v9092 = v9077;
                v10266 = v10205;
                v10267 = v10207;
                v10268 = v10206;
                v10269 = v10208;
            } else {
                v9089 = v8669;
                v9090 = v8668;
                v9091 = v8670;
                v9092 = v8672;
                v10266 = v10130;
                v10267 = v10129;
                v10268 = v10131;
                v10269 = v10132;
            }
            let v9079 = if (if v626 == v4 { 1.0 } else { 0.0 }) != 0.0 && v628 != 0.0 { 1.0 } else { 0.0 };
            let v9153: f64;
            let v9154: f64;
            let v9158: f64;
            let v10270: Lanes<6>;
            if v9079 != 0.0 {
                let v9081 = v9080 * v818;
                let v18711 = v9402 * v9080;
                let v18713 = (v10260 * v818) + (Lanes([v18711[0], v18711[1], 0.0, 0.0, 0.0, 0.0]));
                let v9082 = v4 / v382;
                v9153 = v9081;
                v9154 = v9082;
                v9158 = v383;
                v10270 = v18713;
            } else {
                v9153 = v0;
                v9154 = v0;
                v9158 = v0;
                v10270 = v11054;
            }
            let v9085 = if v7820 != v4 { 1.0 } else { 0.0 };
            if v9085 != 0.0 {
            } else {
            }
            if v5 != 0.0 {
            } else {
            }
            let v9093 = if v65 >= v86 { 1.0 } else { 0.0 };
            if v9093 != 0.0 {
                if v5 != 0.0 {
                } else {
                }
            } else {
            }
            let v9095 = v9094 * v647;
            let v18714 = v9389 * v9094;
            let v9096 = v361 * v9080;
            let v18715 = v10260 * v361;
            let v9097 = if v5788 == v4 { 1.0 } else { 0.0 };
            let v9248: f64;
            let v9249: f64;
            let v9250: f64;
            let v10271: Lanes<6>;
            let v10272: Lanes<6>;
            let v10273: Lanes<4>;
            if v9097 != 0.0 {
                let v9098 = v361 * v9073;
                let v18716 = v10211 * v361;
                let v9099 = v361 * v9072;
                let v18717 = v10210 * v361;
                let v9100 = v361 * v8677;
                let v18718 = v18395 * v361;
                v9248 = v9098;
                v9249 = v9099;
                v9250 = v9100;
                v10271 = v18716;
                v10272 = v18717;
                v10273 = v18718;
            } else {
                v9248 = v0;
                v9249 = v0;
                v9250 = v0;
                v10271 = v11054;
                v10272 = v11054;
                v10273 = v10617;
            }
            let v9251: f64;
            let v9252: f64;
            let v10274: Lanes<5>;
            if v8749 != 0.0 {
                let v18721 = (Lanes([0.0, v9362[0]])) - (Lanes([v9366[0], 0.0]));
                let v9102 = (v598 - v608) / v9041;
                let v18725 = ((Lanes([0.0, v18721[0], 0.0, v18721[1], 0.0])) - (v10219 * v9102)) / v9041;
                v9251 = v9102;
                v9252 = v0;
                v10274 = v18725;
            } else {
                v9251 = v0;
                v9252 = v9103;
                v10274 = v18451;
            }
            let v9253: f64;
            let v9254: f64;
            let v10275: Lanes<5>;
            if v8875 != 0.0 {
                let v18728 = (Lanes([v9365[0], 0.0])) - (Lanes([0.0, v9361[0]]));
                let v9105 = (v607 - v597) / v9040;
                let v18732 = ((Lanes([v18728[0], 0.0, v18728[1], 0.0, 0.0])) - (v10228 * v9105)) / v9040;
                v9253 = v9105;
                v9254 = v0;
                v10275 = v18732;
            } else {
                v9253 = v0;
                v9254 = v9106;
                v10275 = v18451;
            }
            let v9108 = v361 * (ddt(73821, v9086));
            let v18735 = (v10263 * v18733) * v361;
            let v9110 = v361 * (ddt(73825, v9087));
            let v18737 = (v10264 * v18733) * v361;
            let v9112 = v361 * (ddt(73829, v9088));
            let v18739 = (v10265 * v18733) * v361;
            let v9115 = v9095 * v8396;
            let v18740 = v18714 * v8396;
            let v18743 = (Lanes([0.0, 0.0, v18740[0], 0.0, 0.0, 0.0])) + (v10166 * v9095);
            let v9120 = if (if v9115 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v9118 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v9123: f64;
            let v10276: Lanes<6>;
            if v9120 != 0.0 {
                let v9121 = v9118 / v9115;
                let v9122 = v9121.sqrt();
                let v18749 = ((v10215 - (v18743 * v9121)) / v9115) * (v9360 / (v10427 * v9122));
                v9123 = v9122;
                v10276 = v18749;
            } else {
                v9123 = v0;
                v10276 = v11054;
            }
            let v9127 = v9116 * v9124;
            let v18750 = v10214 * v9124;
            let v18751 = v9373 * v9116;
            let v18754 = (Lanes([v18750[0], v18750[1], v18750[2], v18750[3], v18750[4], 0.0, v18750[5]])) + (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v18751[0], 0.0]));
            let v9131: f64;
            let v10277: Lanes<6>;
            if v8707 != 0.0 {
                let v9128 = v4 - v9113;
                let v9129 = v9123 * v9128;
                let v18761 = (v10276 * v9128) + ((v9785 * v10382) * v9123);
                v9131 = v9129;
                v10277 = v18761;
            } else {
                let v9130 = v9123 * v9113;
                let v18757 = (v10276 * v9113) + (v9785 * v9123);
                v9131 = v9130;
                v10277 = v18757;
            }
            let v9135: f64;
            let v10278: Lanes<6>;
            if v8707 != 0.0 {
                let v9132 = v9123 * v9113;
                let v18768 = (v10276 * v9113) + (v9785 * v9123);
                v9135 = v9132;
                v10278 = v18768;
            } else {
                let v9133 = v4 - v9113;
                let v9134 = v9123 * v9133;
                let v18765 = (v10276 * v9133) + ((v9785 * v10382) * v9123);
                v9135 = v9134;
                v10278 = v18765;
            }
            let v9136 = v9124 * v9131;
            let v18769 = v9373 * v9131;
            let v18770 = v10277 * v9124;
            let v18773 = (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v18769[0], 0.0])) + (Lanes([v18770[0], v18770[1], v18770[2], v18770[3], v18770[4], 0.0, v18770[5]]));
            let v9137 = ddt(73902, v9136);
            let v18774 = v18773 * v18733;
            let v9138 = v9124 * v9135;
            let v18775 = v9373 * v9135;
            let v18776 = v10278 * v9124;
            let v18779 = (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v18775[0], 0.0])) + (Lanes([v18776[0], v18776[1], v18776[2], v18776[3], v18776[4], 0.0, v18776[5]]));
            let v9139 = ddt(73906, v9138);
            let v18780 = v18779 * v18733;
            let v9255: f64;
            if v8749 != 0.0 {
                v9255 = v9140;
            } else {
                v9255 = v0;
            }
            let v9256: f64;
            if v8875 != 0.0 {
                v9256 = v9141;
            } else {
                v9256 = v0;
            }
            let v9257: f64;
            let v9258: f64;
            let v9259: f64;
            if v9097 != 0.0 {
                v9257 = v9142;
                v9258 = v9143;
                v9259 = v9144;
            } else {
                v9257 = v0;
                v9258 = v0;
                v9259 = v0;
            }
            let v9260: f64;
            let v9261: f64;
            let v10279: Lanes<2>;
            if v535 != 0.0 {
                let v9149 = v9145 * (v9147 - v601);
                let v18785 = ((Lanes([v9374[0], 0.0])) - (Lanes([0.0, v9363[0]]))) * v9145;
                v9260 = v9149;
                v9261 = v0;
                v10279 = v18785;
            } else {
                v9260 = v0;
                v9261 = v9150;
                v10279 = v18781;
            }
            let v9152 = if v627 != 0.0 && (if v29 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v9262: f64;
            let v9263: f64;
            let v9264: f64;
            let v9265: f64;
            let v9266: f64;
            let v9345: f64;
            let v10280: Lanes<1>;
            let v10281: Lanes<6>;
            let v10282: Lanes<1>;
            let v10283: Lanes<1>;
            let v10284: Lanes<1>;
            let v10285: Lanes<1>;
            if v9152 != 0.0 {
                let v9155 = v630 * v9154;
                let v18787 = v9371 * v9154;
                let v9156 = -v9153;
                let v18788 = v10270 * v10382;
                let v9157 = v630 * v6;
                let v18789 = v9371 * v6;
                let v9159 = v9158 * v630;
                let v18790 = v9371 * v9158;
                let v9160 = ddt(73967, v9159);
                let v18791 = v18790 * v18733;
                v9262 = v9155;
                v9263 = v9156;
                v9264 = v9157;
                v9265 = v9160;
                v9266 = v0;
                v9345 = v9159;
                v10280 = v18787;
                v10281 = v18788;
                v10282 = v18789;
                v10283 = v18791;
                v10284 = v10381;
                v10285 = v18790;
            } else {
                let v9161 = v630 * v552;
                let v18786 = v9371 * v552;
                v9262 = v0;
                v9263 = v0;
                v9264 = v0;
                v9265 = v0;
                v9266 = v9161;
                v9345 = v0;
                v10280 = v10381;
                v10281 = v11054;
                v10282 = v10381;
                v10283 = v10381;
                v10284 = v18786;
                v10285 = v10381;
            }
            let v9267: f64;
            let v9268: f64;
            let v9269: f64;
            let v9270: f64;
            let v9271: f64;
            let v9273: f64;
            let v9275: f64;
            let v9277: f64;
            let v9279: f64;
            let v9281: f64;
            let v9283: f64;
            let v9285: f64;
            let v9287: f64;
            let v9289: f64;
            let v9291: f64;
            let v9293: f64;
            let v9295: f64;
            let v9297: f64;
            let v9299: f64;
            let v9301: f64;
            let v9303: f64;
            let v9305: f64;
            let v9307: f64;
            let v9308: f64;
            let v9309: f64;
            let v9310: f64;
            let v9312: f64;
            let v9314: f64;
            let v9316: f64;
            let v9318: f64;
            let v9320: f64;
            let v9322: f64;
            let v9324: f64;
            let v9326: f64;
            let v9328: f64;
            let v9330: f64;
            let v9332: f64;
            let v9334: f64;
            let v9336: f64;
            let v9338: f64;
            let v9340: f64;
            let v9347: f64;
            let v9349: f64;
            let v9351: f64;
            let v9353: f64;
            let v9355: f64;
            let v9357: f64;
            let v9359: f64;
            let v10286: Lanes<6>;
            let v10287: Lanes<6>;
            let v10288: Lanes<3>;
            let v10289: Lanes<3>;
            let v10290: Lanes<2>;
            let v10291: Lanes<2>;
            let v10292: Lanes<2>;
            let v10293: Lanes<7>;
            let v10294: Lanes<7>;
            let v10295: Lanes<1>;
            let v10296: Lanes<1>;
            let v10297: Lanes<1>;
            let v10298: Lanes<1>;
            let v10299: Lanes<6>;
            let v10300: Lanes<1>;
            let v10301: Lanes<1>;
            let v10302: Lanes<6>;
            let v10303: Lanes<6>;
            let v10304: Lanes<6>;
            let v10305: Lanes<1>;
            let v10306: Lanes<1>;
            let v10307: Lanes<7>;
            let v10308: Lanes<7>;
            let v10309: Lanes<7>;
            let v10310: Lanes<1>;
            let v10311: Lanes<1>;
            let v10312: Lanes<1>;
            let v10313: Lanes<1>;
            let v10314: Lanes<1>;
            let v10315: Lanes<1>;
            let v10316: Lanes<1>;
            let v10317: Lanes<1>;
            let v10318: Lanes<1>;
            let v10319: Lanes<1>;
            let v10320: Lanes<1>;
            let v10321: Lanes<1>;
            let v10322: Lanes<1>;
            if v5 != 0.0 {
                let v9163 = v361 * (v8701 + v9083);
                let v18815 = ((Lanes([v10212[0], v10212[1], v10212[2], v10212[3], v10212[4], 0.0])) + v10261) * v361;
                let v9165 = v361 * (v8702 + v9084);
                let v18818 = ((Lanes([v10213[0], v10213[1], v10213[2], v10213[3], v10213[4], 0.0])) + v10262) * v361;
                let v18819 = v10269 * v18733;
                let v9168 = v361 * (v9090 + (ddt(73987, v9092)));
                let v18822 = (v10267 + (Lanes([v18819[0], 0.0, v18819[1]]))) * v361;
                let v18823 = v10268 * v18733;
                let v9171 = v361 * (v9089 + (ddt(73993, v9091)));
                let v18826 = (v10266 + (Lanes([v18823[0], 0.0, v18823[1]]))) * v361;
                let v9272: f64;
                let v9274: f64;
                let v10323: Lanes<2>;
                if v541 != 0.0 {
                    let v9176 = (v9172 - v604) / v9174;
                    let v18830 = ((Lanes([v9375[0], 0.0])) - (Lanes([0.0, v9364[0]]))) / v9174;
                    v9272 = v9176;
                    v9274 = v0;
                    v10323 = v18830;
                } else {
                    v9272 = v0;
                    v9274 = v9177;
                    v10323 = v18810;
                }
                let v9276: f64;
                let v9278: f64;
                let v9280: f64;
                let v9282: f64;
                let v10324: Lanes<2>;
                let v10325: Lanes<2>;
                if v548 != 0.0 {
                    let v9182 = v9178 * (v9180 - v604);
                    let v18834 = ((Lanes([v9376[0], 0.0])) - (Lanes([0.0, v9364[0]]))) * v9178;
                    let v9187 = v9183 * (v9185 - v604);
                    let v18838 = ((Lanes([v9377[0], 0.0])) - (Lanes([0.0, v9364[0]]))) * v9183;
                    v9276 = v9182;
                    v9278 = v9187;
                    v9280 = v0;
                    v9282 = v0;
                    v10324 = v18834;
                    v10325 = v18838;
                } else {
                    v9276 = v0;
                    v9278 = v0;
                    v9280 = v9188;
                    v9282 = v9189;
                    v10324 = v18811;
                    v10325 = v18812;
                }
                let v9284: f64;
                let v9286: f64;
                let v9288: f64;
                let v9290: f64;
                let v9292: f64;
                let v9294: f64;
                let v9296: f64;
                let v9298: f64;
                let v9346: f64;
                let v9348: f64;
                let v10326: Lanes<7>;
                let v10327: Lanes<7>;
                let v10328: Lanes<1>;
                let v10329: Lanes<1>;
                let v10330: Lanes<1>;
                let v10331: Lanes<1>;
                let v10332: Lanes<1>;
                let v10333: Lanes<1>;
                if v66 != 0.0 {
                    let v9194 = v613 * v6;
                    let v18839 = v9367 * v6;
                    let v9195 = v616 * v6;
                    let v18840 = v9368 * v6;
                    let v9197 = v9196 * v613;
                    let v18841 = v9367 * v9196;
                    let v9198 = ddt(74024, v9197);
                    let v18842 = v18841 * v18733;
                    let v9200 = v9199 * v616;
                    let v18843 = v9368 * v9199;
                    let v9201 = ddt(74030, v9200);
                    let v18844 = v18843 * v18733;
                    v9284 = v9190;
                    v9286 = v9192;
                    v9288 = v9194;
                    v9290 = v9195;
                    v9292 = v9198;
                    v9294 = v9201;
                    v9296 = v0;
                    v9298 = v0;
                    v9346 = v9197;
                    v9348 = v9200;
                    v10326 = v10241;
                    v10327 = v10242;
                    v10328 = v18839;
                    v10329 = v18840;
                    v10330 = v18842;
                    v10331 = v18844;
                    v10332 = v18841;
                    v10333 = v18843;
                } else {
                    v9284 = v0;
                    v9286 = v0;
                    v9288 = v0;
                    v9290 = v0;
                    v9292 = v0;
                    v9294 = v0;
                    v9296 = v9202;
                    v9298 = v9203;
                    v9346 = v0;
                    v9348 = v0;
                    v10326 = v18653;
                    v10327 = v18654;
                    v10328 = v10374;
                    v10329 = v10366;
                    v10330 = v10374;
                    v10331 = v10366;
                    v10332 = v10374;
                    v10333 = v10366;
                }
                let v9204 = if v2242 != 0.0 || v5620 != 0.0 { 1.0 } else { 0.0 };
                let v9300: f64;
                let v9302: f64;
                let v9304: f64;
                let v9306: f64;
                let v9350: f64;
                let v10334: Lanes<6>;
                let v10335: Lanes<1>;
                let v10336: Lanes<1>;
                let v10337: Lanes<1>;
                if v9204 != 0.0 {
                    let v9211 = v2249 * v6;
                    let v18845 = v9372 * v6;
                    let v9213 = v9212 * v2249;
                    let v18846 = v9372 * v9212;
                    let v9214 = ddt(74051, v9213);
                    let v18847 = v18846 * v18733;
                    v9300 = v9205;
                    v9302 = v9211;
                    v9304 = v9214;
                    v9306 = v0;
                    v9350 = v9213;
                    v10334 = v9876;
                    v10335 = v18845;
                    v10336 = v18847;
                    v10337 = v18846;
                } else {
                    v9300 = v0;
                    v9302 = v0;
                    v9304 = v0;
                    v9306 = v9215;
                    v9350 = v0;
                    v10334 = v11054;
                    v10335 = v11028;
                    v10336 = v11028;
                    v10337 = v11028;
                }
                v9267 = v9163;
                v9268 = v9165;
                v9269 = v9168;
                v9270 = v9171;
                v9271 = v9272;
                v9273 = v9274;
                v9275 = v9276;
                v9277 = v9278;
                v9279 = v9280;
                v9281 = v9282;
                v9283 = v9284;
                v9285 = v9286;
                v9287 = v9288;
                v9289 = v9290;
                v9291 = v9292;
                v9293 = v9294;
                v9295 = v9296;
                v9297 = v9298;
                v9299 = v9300;
                v9301 = v9302;
                v9303 = v9304;
                v9305 = v9306;
                v9307 = v0;
                v9308 = v0;
                v9309 = v0;
                v9310 = v0;
                v9312 = v0;
                v9314 = v0;
                v9316 = v0;
                v9318 = v0;
                v9320 = v0;
                v9322 = v0;
                v9324 = v0;
                v9326 = v0;
                v9328 = v0;
                v9330 = v0;
                v9332 = v0;
                v9334 = v0;
                v9336 = v0;
                v9338 = v0;
                v9340 = v0;
                v9347 = v9346;
                v9349 = v9348;
                v9351 = v9350;
                v9353 = v0;
                v9355 = v0;
                v9357 = v0;
                v9359 = v0;
                v10286 = v18815;
                v10287 = v18818;
                v10288 = v18822;
                v10289 = v18826;
                v10290 = v10323;
                v10291 = v10324;
                v10292 = v10325;
                v10293 = v10326;
                v10294 = v10327;
                v10295 = v10328;
                v10296 = v10329;
                v10297 = v10330;
                v10298 = v10331;
                v10299 = v10334;
                v10300 = v10335;
                v10301 = v10336;
                v10302 = v11054;
                v10303 = v11054;
                v10304 = v11054;
                v10305 = v11028;
                v10306 = v11028;
                v10307 = v18627;
                v10308 = v18628;
                v10309 = v18654;
                v10310 = v10367;
                v10311 = v10368;
                v10312 = v10366;
                v10313 = v10367;
                v10314 = v10368;
                v10315 = v10366;
                v10316 = v10332;
                v10317 = v10333;
                v10318 = v10337;
                v10319 = v11028;
                v10320 = v10367;
                v10321 = v10368;
                v10322 = v10366;
            } else {
                let v9217 = v361 * (v8701 + v9083);
                let v18794 = ((Lanes([v10212[0], v10212[1], v10212[2], v10212[3], v10212[4], 0.0])) + v10261) * v361;
                let v9219 = v361 * (v8702 + v9084);
                let v18797 = ((Lanes([v10213[0], v10213[1], v10213[2], v10213[3], v10213[4], 0.0])) + v10262) * v361;
                let v9311: f64;
                let v9313: f64;
                let v9315: f64;
                let v9317: f64;
                let v9352: f64;
                let v10338: Lanes<6>;
                let v10339: Lanes<1>;
                let v10340: Lanes<1>;
                let v10341: Lanes<1>;
                if v2242 != 0.0 {
                    let v9221 = v2249 * v6;
                    let v18798 = v9372 * v6;
                    let v9223 = v9222 * v2249;
                    let v18799 = v9372 * v9222;
                    let v9224 = ddt(74074, v9223);
                    let v18800 = v18799 * v18733;
                    v9311 = v9205;
                    v9313 = v9221;
                    v9315 = v9224;
                    v9317 = v0;
                    v9352 = v9223;
                    v10338 = v9876;
                    v10339 = v18798;
                    v10340 = v18800;
                    v10341 = v18799;
                } else {
                    v9311 = v0;
                    v9313 = v0;
                    v9315 = v0;
                    v9317 = v9225;
                    v9352 = v0;
                    v10338 = v11054;
                    v10339 = v11028;
                    v10340 = v11028;
                    v10341 = v11028;
                }
                let v9319: f64;
                let v9321: f64;
                let v9323: f64;
                let v9325: f64;
                let v9327: f64;
                let v9329: f64;
                let v9331: f64;
                let v9333: f64;
                let v9335: f64;
                let v9337: f64;
                let v9339: f64;
                let v9341: f64;
                let v9354: f64;
                let v9356: f64;
                let v9358: f64;
                let v10342: Lanes<7>;
                let v10343: Lanes<7>;
                let v10344: Lanes<7>;
                let v10345: Lanes<1>;
                let v10346: Lanes<1>;
                let v10347: Lanes<1>;
                let v10348: Lanes<1>;
                let v10349: Lanes<1>;
                let v10350: Lanes<1>;
                let v10351: Lanes<1>;
                let v10352: Lanes<1>;
                let v10353: Lanes<1>;
                if v66 != 0.0 {
                    let v9230 = v619 * v6;
                    let v18801 = v9369 * v6;
                    let v9231 = v622 * v6;
                    let v18802 = v9370 * v6;
                    let v9232 = v616 * v6;
                    let v18803 = v9368 * v6;
                    let v9234 = v9233 * v619;
                    let v18804 = v9369 * v9233;
                    let v9235 = ddt(74094, v9234);
                    let v18805 = v18804 * v18733;
                    let v9237 = v9236 * v622;
                    let v18806 = v9370 * v9236;
                    let v9238 = ddt(74100, v9237);
                    let v18807 = v18806 * v18733;
                    let v9240 = v9239 * v616;
                    let v18808 = v9368 * v9239;
                    let v9241 = ddt(74106, v9240);
                    let v18809 = v18808 * v18733;
                    v9319 = v9226;
                    v9321 = v9228;
                    v9323 = v9192;
                    v9325 = v9230;
                    v9327 = v9231;
                    v9329 = v9232;
                    v9331 = v9235;
                    v9333 = v9238;
                    v9335 = v9241;
                    v9337 = v0;
                    v9339 = v0;
                    v9341 = v0;
                    v9354 = v9234;
                    v9356 = v9237;
                    v9358 = v9240;
                    v10342 = v10243;
                    v10343 = v10244;
                    v10344 = v10242;
                    v10345 = v18801;
                    v10346 = v18802;
                    v10347 = v18803;
                    v10348 = v18805;
                    v10349 = v18807;
                    v10350 = v18809;
                    v10351 = v18804;
                    v10352 = v18806;
                    v10353 = v18808;
                } else {
                    v9319 = v0;
                    v9321 = v0;
                    v9323 = v0;
                    v9325 = v0;
                    v9327 = v0;
                    v9329 = v0;
                    v9331 = v0;
                    v9333 = v0;
                    v9335 = v0;
                    v9337 = v9242;
                    v9339 = v9243;
                    v9341 = v9244;
                    v9354 = v0;
                    v9356 = v0;
                    v9358 = v0;
                    v10342 = v18627;
                    v10343 = v18628;
                    v10344 = v18654;
                    v10345 = v10367;
                    v10346 = v10368;
                    v10347 = v10366;
                    v10348 = v10367;
                    v10349 = v10368;
                    v10350 = v10366;
                    v10351 = v10367;
                    v10352 = v10368;
                    v10353 = v10366;
                }
                v9267 = v0;
                v9268 = v0;
                v9269 = v0;
                v9270 = v0;
                v9271 = v0;
                v9273 = v0;
                v9275 = v0;
                v9277 = v0;
                v9279 = v0;
                v9281 = v0;
                v9283 = v0;
                v9285 = v0;
                v9287 = v0;
                v9289 = v0;
                v9291 = v0;
                v9293 = v0;
                v9295 = v0;
                v9297 = v0;
                v9299 = v0;
                v9301 = v0;
                v9303 = v0;
                v9305 = v0;
                v9307 = v9217;
                v9308 = v9219;
                v9309 = v9220;
                v9310 = v9311;
                v9312 = v9313;
                v9314 = v9315;
                v9316 = v9317;
                v9318 = v9319;
                v9320 = v9321;
                v9322 = v9323;
                v9324 = v9325;
                v9326 = v9327;
                v9328 = v9329;
                v9330 = v9331;
                v9332 = v9333;
                v9334 = v9335;
                v9336 = v9337;
                v9338 = v9339;
                v9340 = v9341;
                v9347 = v0;
                v9349 = v0;
                v9351 = v0;
                v9353 = v9352;
                v9355 = v9354;
                v9357 = v9356;
                v9359 = v9358;
                v10286 = v11054;
                v10287 = v11054;
                v10288 = v17762;
                v10289 = v17763;
                v10290 = v18810;
                v10291 = v18811;
                v10292 = v18812;
                v10293 = v18653;
                v10294 = v18654;
                v10295 = v10374;
                v10296 = v10366;
                v10297 = v10374;
                v10298 = v10366;
                v10299 = v11054;
                v10300 = v11028;
                v10301 = v11028;
                v10302 = v18794;
                v10303 = v18797;
                v10304 = v10338;
                v10305 = v10339;
                v10306 = v10340;
                v10307 = v10342;
                v10308 = v10343;
                v10309 = v10344;
                v10310 = v10345;
                v10311 = v10346;
                v10312 = v10347;
                v10313 = v10348;
                v10314 = v10349;
                v10315 = v10350;
                v10316 = v10374;
                v10317 = v10366;
                v10318 = v11028;
                v10319 = v10341;
                v10320 = v10351;
                v10321 = v10352;
                v10322 = v10353;
            }
            let v9342: f64;
            let v9343: f64;
            let v9344: f64;
            if v148 != 0.0 {
                v9342 = v9245;
                v9343 = v0;
                v9344 = v0;
            } else {
                v9342 = v0;
                v9343 = v9246;
                v9344 = v9247;
            }
            let v19264 = v18715[0];
            let v19265 = v18715[1];
            let v19266 = v18715[2];
            let v19267 = v18715[3];
            let v19268 = v18715[4];
            let v19269 = v18715[5];
            let v19270 = v10271[0];
            let v19271 = v10271[1];
            let v19272 = v10271[2];
            let v19273 = v10271[3];
            let v19274 = v10271[4];
            let v19275 = v10271[5];
            let v19276 = v10272[0];
            let v19277 = v10272[1];
            let v19278 = v10272[2];
            let v19279 = v10272[3];
            let v19280 = v10272[4];
            let v19281 = v10272[5];
            let v19282 = v10273[0];
            let v19283 = v10273[1];
            let v19284 = v10273[2];
            let v19285 = v10273[3];
            let v19286 = v10274[0];
            let v19287 = v10274[1];
            let v19288 = v10274[2];
            let v19289 = v10274[3];
            let v19290 = v10274[4];
            let v19291 = v10275[0];
            let v19292 = v10275[1];
            let v19293 = v10275[2];
            let v19294 = v10275[3];
            let v19295 = v10275[4];
            let v19296 = v18735[0];
            let v19297 = v18735[1];
            let v19298 = v18735[2];
            let v19299 = v18735[3];
            let v19300 = v18735[4];
            let v19301 = v18735[5];
            let v19302 = v18735[6];
            let v19303 = v18735[7];
            let v19304 = v18735[8];
            let v19305 = v18735[9];
            let v19306 = v18737[0];
            let v19307 = v18737[1];
            let v19308 = v18737[2];
            let v19309 = v18737[3];
            let v19310 = v18737[4];
            let v19311 = v18737[5];
            let v19312 = v18737[6];
            let v19313 = v18737[7];
            let v19314 = v18737[8];
            let v19315 = v18739[0];
            let v19316 = v18739[1];
            let v19317 = v18739[2];
            let v19318 = v18739[3];
            let v19319 = v18739[4];
            let v19320 = v18739[5];
            let v19321 = v18739[6];
            let v19322 = v9373[0];
            let v19323 = v18754[0];
            let v19324 = v18754[1];
            let v19325 = v18754[2];
            let v19326 = v18754[3];
            let v19327 = v18754[4];
            let v19328 = v18754[5];
            let v19329 = v18754[6];
            let v19330 = v18774[0];
            let v19331 = v18774[1];
            let v19332 = v18774[2];
            let v19333 = v18774[3];
            let v19334 = v18774[4];
            let v19335 = v18774[5];
            let v19336 = v18774[6];
            let v19337 = v18780[0];
            let v19338 = v18780[1];
            let v19339 = v18780[2];
            let v19340 = v18780[3];
            let v19341 = v18780[4];
            let v19342 = v18780[5];
            let v19343 = v18780[6];
            let v19344 = v10279[0];
            let v19345 = v10279[1];
            let v19346 = v10280[0];
            let v19347 = v10281[0];
            let v19348 = v10281[1];
            let v19349 = v10281[2];
            let v19350 = v10281[3];
            let v19351 = v10281[4];
            let v19352 = v10281[5];
            let v19353 = v10282[0];
            let v19354 = v10283[0];
            let v19355 = v10284[0];
            let v19356 = v10286[0];
            let v19357 = v10286[1];
            let v19358 = v10286[2];
            let v19359 = v10286[3];
            let v19360 = v10286[4];
            let v19361 = v10286[5];
            let v19362 = v10287[0];
            let v19363 = v10287[1];
            let v19364 = v10287[2];
            let v19365 = v10287[3];
            let v19366 = v10287[4];
            let v19367 = v10287[5];
            let v19368 = v10288[0];
            let v19369 = v10288[1];
            let v19370 = v10288[2];
            let v19371 = v10289[0];
            let v19372 = v10289[1];
            let v19373 = v10289[2];
            let v19374 = v10290[0];
            let v19375 = v10290[1];
            let v19376 = v10291[0];
            let v19377 = v10291[1];
            let v19378 = v10292[0];
            let v19379 = v10292[1];
            let v19380 = v10293[0];
            let v19381 = v10293[1];
            let v19382 = v10293[2];
            let v19383 = v10293[3];
            let v19384 = v10293[4];
            let v19385 = v10293[5];
            let v19386 = v10293[6];
            let v19387 = v10294[0];
            let v19388 = v10294[1];
            let v19389 = v10294[2];
            let v19390 = v10294[3];
            let v19391 = v10294[4];
            let v19392 = v10294[5];
            let v19393 = v10294[6];
            let v19394 = v10295[0];
            let v19395 = v10296[0];
            let v19396 = v10297[0];
            let v19397 = v10298[0];
            let v19398 = v10299[0];
            let v19399 = v10299[1];
            let v19400 = v10299[2];
            let v19401 = v10299[3];
            let v19402 = v10299[4];
            let v19403 = v10299[5];
            let v19404 = v10300[0];
            let v19405 = v10301[0];
            let v19406 = v10302[0];
            let v19407 = v10302[1];
            let v19408 = v10302[2];
            let v19409 = v10302[3];
            let v19410 = v10302[4];
            let v19411 = v10302[5];
            let v19412 = v10303[0];
            let v19413 = v10303[1];
            let v19414 = v10303[2];
            let v19415 = v10303[3];
            let v19416 = v10303[4];
            let v19417 = v10303[5];
            let v19418 = v10304[0];
            let v19419 = v10304[1];
            let v19420 = v10304[2];
            let v19421 = v10304[3];
            let v19422 = v10304[4];
            let v19423 = v10304[5];
            let v19424 = v10305[0];
            let v19425 = v10306[0];
            let v19426 = v10307[0];
            let v19427 = v10307[1];
            let v19428 = v10307[2];
            let v19429 = v10307[3];
            let v19430 = v10307[4];
            let v19431 = v10307[5];
            let v19432 = v10307[6];
            let v19433 = v10308[0];
            let v19434 = v10308[1];
            let v19435 = v10308[2];
            let v19436 = v10308[3];
            let v19437 = v10308[4];
            let v19438 = v10308[5];
            let v19439 = v10308[6];
            let v19440 = v10309[0];
            let v19441 = v10309[1];
            let v19442 = v10309[2];
            let v19443 = v10309[3];
            let v19444 = v10309[4];
            let v19445 = v10309[5];
            let v19446 = v10309[6];
            let v19447 = v10310[0];
            let v19448 = v10311[0];
            let v19449 = v10312[0];
            let v19450 = v10313[0];
            let v19451 = v10314[0];
            let v19452 = v10315[0];
            let v19453 = v18773[0];
            let v19454 = v18773[1];
            let v19455 = v18773[2];
            let v19456 = v18773[3];
            let v19457 = v18773[4];
            let v19458 = v18773[5];
            let v19459 = v18773[6];
            let v19460 = v18779[0];
            let v19461 = v18779[1];
            let v19462 = v18779[2];
            let v19463 = v18779[3];
            let v19464 = v18779[4];
            let v19465 = v18779[5];
            let v19466 = v18779[6];
            let v19467 = v10285[0];
            let v19468 = v10316[0];
            let v19469 = v10317[0];
            let v19470 = v10318[0];
            let v19471 = v10319[0];
            let v19472 = v10320[0];
            let v19473 = v10321[0];
            let v19474 = v10322[0];
        stamper.stamp_potential_branch_local(Some(5), None, 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            v1,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(4), None, 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            v2,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(7),
            multiplicity * (v9096),
            [6, 7, 10, 11, 12, 17],
            [v19264, v19265, v19266, v19267, v19268, v19269],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(7),
            multiplicity * (v9248),
            [6, 7, 10, 11, 12, 17],
            [v19270, v19271, v19272, v19273, v19274, v19275],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(6),
            multiplicity * (v9249),
            [6, 7, 10, 11, 12, 17],
            [v19276, v19277, v19278, v19279, v19280, v19281],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(11),
            Some(12),
            multiplicity * (v9250),
            [6, 7, 11, 12],
            [v19282, v19283, v19284, v19285],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(2),
            multiplicity * (v9251),
            [0, 2, 6, 7, 10],
            [v19286, v19287, v19288, v19289, v19290],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(7), Some(2), 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            v9252,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(0),
            Some(6),
            multiplicity * (v9253),
            [0, 2, 6, 7, 10],
            [v19291, v19292, v19293, v19294, v19295],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(0), Some(6), 3, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            3,
            v9254,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(11),
            Some(7),
            multiplicity * (v9108),
            [6, 7, 10, 11, 12, 13, 15, 16, 17, 18],
            [v19296, v19297, v19298, v19299, v19300, v19301, v19302, v19303, v19304, v19305],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(7),
            multiplicity * (v9110),
            [6, 7, 10, 11, 12, 15, 16, 17, 18],
            [v19306, v19307, v19308, v19309, v19310, v19311, v19312, v19313, v19314],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(12),
            Some(7),
            multiplicity * (v9112),
            [6, 7, 10, 11, 12, 13, 17],
            [v19315, v19316, v19317, v19318, v19319, v19320, v19321],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(7),
            multiplicity * (v9114),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(14),
            None,
            multiplicity * (v9124),
            [14],
            [v19322],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(14),
            None,
            multiplicity * (v9125),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(7),
            multiplicity * (v9126),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(6),
            Some(7),
            multiplicity * (v9127),
            [6, 7, 10, 11, 12, 14, 17],
            [v19323, v19324, v19325, v19326, v19327, v19328, v19329],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(11),
            Some(7),
            multiplicity * (v9137),
            [6, 7, 10, 11, 12, 14, 17],
            [v19330, v19331, v19332, v19333, v19334, v19335, v19336],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(11),
            Some(6),
            multiplicity * (v9139),
            [6, 7, 10, 11, 12, 14, 17],
            [v19337, v19338, v19339, v19340, v19341, v19342, v19343],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(2),
            multiplicity * (v9255),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(6),
            multiplicity * (v9256),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(11),
            Some(6),
            multiplicity * (v9257),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(11),
            Some(7),
            multiplicity * (v9258),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(11),
            Some(12),
            multiplicity * (v9259),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(11),
            multiplicity * (v9260),
            [1, 11],
            [v19344, v19345],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(1), Some(11), 4, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            4,
            v9261,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(10),
            None,
            multiplicity * (v9262),
            [10],
            [v19346],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            None,
            multiplicity * (v9263),
            [6, 7, 10, 11, 12, 17],
            [v19347, v19348, v19349, v19350, v19351, v19352],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(10),
            None,
            multiplicity * (v9264),
            [10],
            [v19353],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(10),
            None,
            multiplicity * (v9265),
            [10],
            [v19354],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(10),
            None,
            multiplicity * (v9266),
            [10],
            [v19355],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(12),
            multiplicity * (v9267),
            [6, 7, 10, 11, 12, 17],
            [v19356, v19357, v19358, v19359, v19360, v19361],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(12),
            multiplicity * (v9268),
            [6, 7, 10, 11, 12, 17],
            [v19362, v19363, v19364, v19365, v19366, v19367],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(9),
            Some(7),
            multiplicity * (v9269),
            [7, 10, 12],
            [v19368, v19369, v19370],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(8),
            Some(6),
            multiplicity * (v9270),
            [6, 10, 12],
            [v19371, v19372, v19373],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(4),
            Some(12),
            multiplicity * (v9271),
            [4, 12],
            [v19374, v19375],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(4), Some(12), 5, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            5,
            v9273,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(9),
            Some(12),
            multiplicity * (v9275),
            [9, 12],
            [v19376, v19377],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(8),
            Some(12),
            multiplicity * (v9277),
            [8, 12],
            [v19378, v19379],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(9), Some(12), 6, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            6,
            v9279,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(8), Some(12), 7, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            7,
            v9281,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(18),
            None,
            multiplicity * (v9283),
            [6, 7, 10, 11, 12, 17, 18],
            [v19380, v19381, v19382, v19383, v19384, v19385, v19386],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(13),
            None,
            multiplicity * (v9285),
            [6, 7, 10, 11, 12, 13, 17],
            [v19387, v19388, v19389, v19390, v19391, v19392, v19393],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(18),
            None,
            multiplicity * (v9287),
            [18],
            [v19394],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(13),
            None,
            multiplicity * (v9289),
            [13],
            [v19395],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(18),
            None,
            multiplicity * (v9291),
            [18],
            [v19396],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(13),
            None,
            multiplicity * (v9293),
            [13],
            [v19397],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(18), None, 8, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            8,
            v9295,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(13), None, 9, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            9,
            v9297,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(17),
            None,
            multiplicity * (v9299),
            [6, 7, 10, 11, 12, 17],
            [v19398, v19399, v19400, v19401, v19402, v19403],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(17),
            None,
            multiplicity * (v9301),
            [17],
            [v19404],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(17),
            None,
            multiplicity * (v9303),
            [17],
            [v19405],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(17), None, 10, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            10,
            v9305,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(7),
            multiplicity * (v9307),
            [6, 7, 10, 11, 12, 17],
            [v19406, v19407, v19408, v19409, v19410, v19411],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(6),
            multiplicity * (v9308),
            [6, 7, 10, 11, 12, 17],
            [v19412, v19413, v19414, v19415, v19416, v19417],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(3), Some(12), 11, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            11,
            v9309,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(17),
            None,
            multiplicity * (v9310),
            [6, 7, 10, 11, 12, 17],
            [v19418, v19419, v19420, v19421, v19422, v19423],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(17),
            None,
            multiplicity * (v9312),
            [17],
            [v19424],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(17),
            None,
            multiplicity * (v9314),
            [17],
            [v19425],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(17), None, 12, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            12,
            v9316,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(15),
            None,
            multiplicity * (v9318),
            [6, 7, 10, 11, 12, 15, 17],
            [v19426, v19427, v19428, v19429, v19430, v19431, v19432],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(16),
            None,
            multiplicity * (v9320),
            [6, 7, 10, 11, 12, 16, 17],
            [v19433, v19434, v19435, v19436, v19437, v19438, v19439],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(13),
            None,
            multiplicity * (v9322),
            [6, 7, 10, 11, 12, 13, 17],
            [v19440, v19441, v19442, v19443, v19444, v19445, v19446],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(15),
            None,
            multiplicity * (v9324),
            [15],
            [v19447],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(16),
            None,
            multiplicity * (v9326),
            [16],
            [v19448],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(13),
            None,
            multiplicity * (v9328),
            [13],
            [v19449],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(15),
            None,
            multiplicity * (v9330),
            [15],
            [v19450],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(16),
            None,
            multiplicity * (v9332),
            [16],
            [v19451],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(13),
            None,
            multiplicity * (v9334),
            [13],
            [v19452],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(15), None, 13, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            13,
            v9336,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(16), None, 14, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            14,
            v9338,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(13), None, 15, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            15,
            v9340,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(18), None, 16, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            16,
            v9342,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(15), None, 17, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            17,
            v9343,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(16), None, 18, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            18,
            v9344,
            [],
            [],
            [],
            [],
        );
        self.canonical_reactive[0] = v1;
        self.canonical_reactive[1] = v2;
        self.canonical_reactive[2] = v9096;
        self.canonical_reactive[3] = v9248;
        self.canonical_reactive[4] = v9249;
        self.canonical_reactive[5] = v9250;
        self.canonical_reactive[6] = v9251;
        self.canonical_reactive[7] = v9252;
        self.canonical_reactive[8] = v9253;
        self.canonical_reactive[9] = v9254;
        self.canonical_reactive[10] = v9108;
        self.canonical_reactive[11] = v9110;
        self.canonical_reactive[12] = v9112;
        self.canonical_reactive[13] = v9114;
        self.canonical_reactive[14] = v9124;
        self.canonical_reactive[15] = v9125;
        self.canonical_reactive[16] = v9126;
        self.canonical_reactive[17] = v9127;
        self.canonical_reactive[18] = v9136;
        self.canonical_reactive[19] = v19453;
        self.canonical_reactive[20] = v19454;
        self.canonical_reactive[21] = v19455;
        self.canonical_reactive[22] = v19456;
        self.canonical_reactive[23] = v19457;
        self.canonical_reactive[24] = v19458;
        self.canonical_reactive[25] = v19459;
        self.canonical_reactive[26] = v9138;
        self.canonical_reactive[27] = v19460;
        self.canonical_reactive[28] = v19461;
        self.canonical_reactive[29] = v19462;
        self.canonical_reactive[30] = v19463;
        self.canonical_reactive[31] = v19464;
        self.canonical_reactive[32] = v19465;
        self.canonical_reactive[33] = v19466;
        self.canonical_reactive[34] = v9255;
        self.canonical_reactive[35] = v9256;
        self.canonical_reactive[36] = v9257;
        self.canonical_reactive[37] = v9258;
        self.canonical_reactive[38] = v9259;
        self.canonical_reactive[39] = v9260;
        self.canonical_reactive[40] = v9261;
        self.canonical_reactive[41] = v9262;
        self.canonical_reactive[42] = v9263;
        self.canonical_reactive[43] = v9264;
        self.canonical_reactive[44] = v9345;
        self.canonical_reactive[45] = v19467;
        self.canonical_reactive[46] = v9266;
        self.canonical_reactive[47] = v9267;
        self.canonical_reactive[48] = v9268;
        self.canonical_reactive[49] = v9269;
        self.canonical_reactive[50] = v9270;
        self.canonical_reactive[51] = v9271;
        self.canonical_reactive[52] = v9273;
        self.canonical_reactive[53] = v9275;
        self.canonical_reactive[54] = v9277;
        self.canonical_reactive[55] = v9279;
        self.canonical_reactive[56] = v9281;
        self.canonical_reactive[57] = v9283;
        self.canonical_reactive[58] = v9285;
        self.canonical_reactive[59] = v9287;
        self.canonical_reactive[60] = v9289;
        self.canonical_reactive[61] = v9347;
        self.canonical_reactive[62] = v19468;
        self.canonical_reactive[63] = v9349;
        self.canonical_reactive[64] = v19469;
        self.canonical_reactive[65] = v9295;
        self.canonical_reactive[66] = v9297;
        self.canonical_reactive[67] = v9299;
        self.canonical_reactive[68] = v9301;
        self.canonical_reactive[69] = v9351;
        self.canonical_reactive[70] = v19470;
        self.canonical_reactive[71] = v9305;
        self.canonical_reactive[72] = v9307;
        self.canonical_reactive[73] = v9308;
        self.canonical_reactive[74] = v9309;
        self.canonical_reactive[75] = v9310;
        self.canonical_reactive[76] = v9312;
        self.canonical_reactive[77] = v9353;
        self.canonical_reactive[78] = v19471;
        self.canonical_reactive[79] = v9316;
        self.canonical_reactive[80] = v9318;
        self.canonical_reactive[81] = v9320;
        self.canonical_reactive[82] = v9322;
        self.canonical_reactive[83] = v9324;
        self.canonical_reactive[84] = v9326;
        self.canonical_reactive[85] = v9328;
        self.canonical_reactive[86] = v9355;
        self.canonical_reactive[87] = v19472;
        self.canonical_reactive[88] = v9357;
        self.canonical_reactive[89] = v19473;
        self.canonical_reactive[90] = v9359;
        self.canonical_reactive[91] = v19474;
        self.canonical_reactive[92] = v9336;
        self.canonical_reactive[93] = v9338;
        self.canonical_reactive[94] = v9340;
        self.canonical_reactive[95] = v9342;
        self.canonical_reactive[96] = v9343;
        self.canonical_reactive[97] = v9344;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(11),
            Some(7),
            &[6, 7, 10, 11, 12, 14, 17],
            &[cached[19], cached[20], cached[21], cached[22], cached[23], cached[24], cached[25]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(11),
            Some(6),
            &[6, 7, 10, 11, 12, 14, 17],
            &[cached[27], cached[28], cached[29], cached[30], cached[31], cached[32], cached[33]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(10),
            None,
            &[10],
            &[cached[45]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(18),
            None,
            &[18],
            &[cached[62]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(13),
            None,
            &[13],
            &[cached[64]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(17),
            None,
            &[17],
            &[cached[70]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(17),
            None,
            &[17],
            &[cached[78]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(15),
            None,
            &[15],
            &[cached[87]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(16),
            None,
            &[16],
            &[cached[89]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(13),
            None,
            &[13],
            &[cached[91]],
            &[],
            &[],
            multiplicity,
        );
    }

}
